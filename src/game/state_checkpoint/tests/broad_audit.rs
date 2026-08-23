//! The closure criterion, checked by playing the formats out.
//!
//! Sampled decision boundaries are handed to `from_observation_checkpoint`
//! twice: once with the host's true hidden zones and once with a hypothesis
//! that deliberately disagrees with them. Both must produce a live game with
//! the same public observation and the same indexed legal actions, because
//! that pair is what a hosted search bot is promised.
//!
//! Every boundary of every game used to be reconstructed. The games grow with
//! the deck registry, so that count grew with them -- three times the floor
//! below, and climbing -- while the extra boundaries were the most redundant
//! ones available, neighbours of positions already checked. See
//! [`reconstructs_here`] for which half is paid for now.
//!
//! The audit also counts what it walked past at those boundaries. A
//! reconstruction suite that
//! stops reaching flashback, or pending replacement events, or restricted
//! mana still passes -- it just stops proving anything -- so the categories
//! below are required rather than merely reported.

use super::super::*;
use super::{determinized, true_hidden_hypothesis};
use crate::policy::Policy;
use std::collections::BTreeMap;

/// Categories the sampled games must actually reach. Each one is rules state
/// that a checkpoint has to carry rather than recompute, and each was chosen
/// because a census of a much wider sweep showed this sample reaches it
/// comfortably rather than by luck.
const REQUIRED_CATEGORIES: &[&str] = &[
    "combat-damage-stage",
    "counters",
    "damage-sources",
    "installed-triggers",
    "flashback",
    "granted-permanent-abilities",
    "modified-permanents",
    "pending-events",
    "restricted-mana",
    "retired-objects-referenced",
    "sourced-mana",
    "stack-abilities",
    "temporary-ability-grants",
];

enum Seat {
    Random(Box<crate::RandomPolicy>),
    Handcrafted(Box<crate::HandcraftedPolicy>),
}

impl Seat {
    fn choose(&mut self, observation: &crate::PlayerObservation) -> Option<crate::Action> {
        match self {
            Self::Random(policy) => policy.choose_action(observation),
            Self::Handcrafted(policy) => policy.choose_action(observation),
        }
    }
}

#[test]
#[ignore = "slow decision-boundary reconstruction audit"]
fn sampled_game_decisions_reconstruct_from_their_observations() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut audited = 0_usize;
    let mut census: BTreeMap<&'static str, usize> = BTreeMap::new();

    let formats = [
        crate::Format::OldSchool9394,
        crate::Format::IsdM14Standard,
        crate::Format::Premodern,
    ];
    for (format_index, format) in formats.into_iter().enumerate() {
        let decks = crate::protocol::deck_names_for_format(format);
        for (index, name) in decks.iter().enumerate() {
            // Every built-in deck plays both seats across the pass, so a card
            // pool that only one archetype can reach still gets audited.
            let opposing = decks[(index * 7 + 3) % decks.len()];
            for pass in 0..2_u64 {
                let seed = 30_000
                    + u64::try_from(index).expect("deck index fits") * 211
                    + pass * 7
                    + u64::try_from(format_index).expect("format index fits") * 4_099;
                audit_one_game(
                    &catalog,
                    format,
                    [name, opposing],
                    seed,
                    pass == 1,
                    &mut audited,
                    &mut census,
                );
            }
        }
    }

    assert!(
        audited >= 10_000,
        "the audit sampled only {audited} decision boundaries"
    );
    let missing = REQUIRED_CATEGORIES
        .iter()
        .filter(|category| !census.contains_key(**category))
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "the sampled games never reached {missing:?}, so the audit stopped \
         covering that state; census was {census:#?}"
    );
}

fn audit_one_game(
    catalog: &CardCatalog,
    format: crate::Format,
    decks: [&str; 2],
    seed: u64,
    scripted_opponent: bool,
    audited: &mut usize,
    census: &mut BTreeMap<&'static str, usize>,
) {
    let [first, second] = decks.map(|name| {
        crate::protocol::deck_by_name_for_format(format, name)
            .unwrap_or_else(|| panic!("{name} is a built-in {format:?} deck"))
    });
    let mut game =
        Game::new_with_format(format, catalog.clone(), [first, second], seed).expect("game starts");
    let mut seats = [
        Seat::Random(Box::new(crate::RandomPolicy::new(seed ^ 0x1111))),
        if scripted_opponent {
            Seat::Handcrafted(Box::new(crate::HandcraftedPolicy::new(catalog.clone())))
        } else {
            Seat::Random(Box::new(crate::RandomPolicy::new(seed ^ 0x2222)))
        },
    ];

    for action_number in 0..3_000 {
        let Some(viewer) = game.decision_player() else {
            return;
        };
        let observation = game.observe(viewer);
        if reconstructs_here(seed, action_number) {
            let actions = crate::protocol::protocol_actions(&observation);
            let wire = crate::protocol::observation_json_for_format(
                catalog,
                format,
                &observation,
                game.in_pregame(),
                &actions,
            );
            for category in categories(&game, &wire) {
                *census.entry(category).or_default() += 1;
            }
            let truth = true_hidden_hypothesis(&game, viewer);
            for (kind, hidden) in [
                ("the host's own hidden zones", truth.clone()),
                ("a determinized hypothesis", determinized(&truth, viewer)),
            ] {
                let rebuilt = Game::from_observation_checkpoint(
                    catalog.clone(),
                    format,
                    &wire,
                    &hidden,
                    seed ^ 0x5555,
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "{}: {error}",
                        context(&game, format, seed, action_number, kind)
                    )
                });
                let rebuilt_observation = rebuilt.observe(viewer);
                let rebuilt_actions = crate::protocol::protocol_actions(&rebuilt_observation);
                assert_eq!(
                    rebuilt_actions,
                    actions,
                    "{}: rebuilt different actions",
                    context(&game, format, seed, action_number, kind),
                );
                assert_eq!(
                    crate::protocol::observation_json_for_format(
                        catalog,
                        format,
                        &rebuilt_observation,
                        rebuilt.in_pregame(),
                        &rebuilt_actions,
                    ),
                    wire,
                    "{}: rebuilt different public state",
                    context(&game, format, seed, action_number, kind),
                );
            }
            *audited += 1;
        }

        let Some(action) = seats[viewer.index()].choose(&observation) else {
            return;
        };
        game.apply_observed_action(&observation, action)
            .expect("a sampled policy action is legal");
    }
}

/// Whether this boundary pays for the two reconstructions and the census.
///
/// The games themselves are still played out in full, every deck and both
/// formats: what is sampled is the expensive work at each boundary, not which
/// positions the sweep reaches. Consecutive boundaries in one game are nearly
/// the same position, so thinning along that axis costs the least coverage
/// per second saved -- unlike dropping decks or formats, which is where
/// genuinely different rules states come from.
///
/// Every required category still appears, the rarest of them seventeen times.
///
/// The choice is hashed rather than taken on `action_number` parity because
/// priority alternates between the seats: a periodic stride would audit one
/// player's view far more often than the other's. Deterministic in the game's
/// own seed, so a failure reproduces exactly.
fn reconstructs_here(seed: u64, action_number: usize) -> bool {
    let mut mixed = seed
        ^ u64::try_from(action_number)
            .expect("action number fits")
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    mixed ^= mixed >> 31;
    mixed.is_multiple_of(2)
}

fn categories(game: &Game, wire: &Value) -> Vec<&'static str> {
    let mut found = Vec::new();
    let mut note = |flag: bool, name: &'static str| {
        if flag {
            found.push(name);
        }
    };
    note(!game.installed_triggers.is_empty(), "installed-triggers");
    note(!game.pending_events.is_empty(), "pending-events");
    note(!game.emblems.is_empty(), "emblems");
    note(
        !game.temporary_ability_grants.is_empty(),
        "temporary-ability-grants",
    );
    note(
        !wire["checkpoint"]["retiredObjects"]
            .as_array()
            .is_none_or(Vec::is_empty),
        "retired-objects-referenced",
    );
    let mana = || game.players.iter().flat_map(|player| &player.mana);
    note(
        mana().any(|mana| !mana.restrictions.is_empty() || !mana.spend_effects.is_empty()),
        "restricted-mana",
    );
    note(mana().any(|mana| mana.source.is_some()), "sourced-mana");
    note(
        game.stack.iter().any(|object| object.is_copy),
        "stack-copies",
    );
    note(
        game.stack.iter().any(|object| object.cast_via_flashback),
        "flashback",
    );
    note(
        game.stack
            .iter()
            .any(|object| object.kind != StackObjectKind::Spell),
        "stack-abilities",
    );
    let battlefield = || game.battlefield.iter();
    note(
        battlefield().any(|permanent| permanent.copy_effect.is_some()),
        "copied-permanents",
    );
    note(
        battlefield().any(|permanent| {
            permanent.resolved_continuous_effects.iter().any(|effect| {
                matches!(
                    effect.kind,
                    ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add { .. })
                )
            })
        }),
        "granted-permanent-abilities",
    );
    note(
        battlefield().any(|permanent| {
            !permanent.temporary_keywords.is_empty()
                || !permanent.resolved_continuous_effects.is_empty()
                || !permanent.text_changes.is_empty()
        }),
        "modified-permanents",
    );
    note(
        battlefield().any(|permanent| !permanent.damage_sources.is_empty()),
        "damage-sources",
    );
    note(
        battlefield().any(|permanent| permanent.counters.iter().any(|count| *count != 0)),
        "counters",
    );
    note(
        !matches!(game.combat_damage_stage, CombatDamageStage::NotStarted),
        "combat-damage-stage",
    );
    if let Some(pending) = game.pending_decisions.first() {
        note(
            matches!(
                pending.continuation,
                crate::game::DecisionContinuation::TriggerOrder { .. }
                    | crate::game::DecisionContinuation::TriggerPlacement { .. }
                    | crate::game::DecisionContinuation::TriggerDivision { .. }
            ),
            "pending-triggers",
        );
    }
    found
}

fn context(
    game: &Game,
    format: crate::Format,
    seed: u64,
    action_number: usize,
    kind: &str,
) -> String {
    format!(
        "{format:?} seed {seed} action {action_number} with {kind} (turn {} {:?} decision={:?} \
         stack={} installed={} events={} retired={})",
        game.turn,
        game.step,
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        game.stack.len(),
        game.installed_triggers.len(),
        game.pending_events.len(),
        game.retired_objects.len(),
    )
}
