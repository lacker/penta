//! Avacyn Restored cards resting on machinery that existed.
//!
//! The focused cases here pin both older audit promotions and cards unlocked
//! by newer shared target and combat-requirement primitives.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Answers each waiting decision by taking the option at `index`, clamped to
/// what is on offer.
fn drain_choosing(game: &mut Game, index: usize) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let pick = index.min(decision.options.len().saturating_sub(1));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[pick].id],
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

#[test]
fn the_explorer_has_swampwalk() {
    let mut game = ready();
    let explorer = creature(10_000, cards::FARBOG_EXPLORER, PlayerId::One);
    let explorer_id = explorer.card.id;
    game.battlefield.push(explorer);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == explorer_id)
        .expect("still there");
    assert!(game.permanent_has_executable_keyword(
        permanent,
        KeywordAbility::Landwalk(BasicLandType::Swamp),
    ));
}

#[test]
fn outwit_sees_only_spells_with_a_player_target() {
    let mut game = ready();
    let outwit = card(10_000, cards::OUTWIT, PlayerId::One);
    let outwit_id = outwit.id;
    game.players[0].hand.push(outwit);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

    let creature = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let creature_id = creature.card.id;
    game.battlefield.push(creature);
    let player_spell = spell_with_targets(
        10_002,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Player(PlayerId::One)],
        0,
    );
    let player_spell_id = player_spell.id;
    let creature_spell = spell_with_targets(
        10_003,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Permanent(creature_id)],
        0,
    );
    let creature_spell_id = creature_spell.id;
    game.stack.push(player_spell);
    game.stack.push(creature_spell);

    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == outwit_id => {
                choices.iter_targets().find_map(|target| match target {
                    Target::Spell(spell) => Some(*spell),
                    _ => None,
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(targets, vec![player_spell_id]);
    assert!(!targets.contains(&creature_spell_id));
}

#[test]
fn revenge_pumps_and_pulls_every_able_blocker() {
    let mut game = ready();
    let target = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let other = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.push(other);
    let revenge = card(10_002, cards::REVENGE_OF_THE_HUNTED, PlayerId::One);
    let revenge_id = revenge.id;
    game.players[0].hand.push(revenge);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == revenge_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(target_id))
            }
            _ => false,
        })
        .expect("Revenge can target the creature");
    game.apply(PlayerId::One, cast).expect("the spell casts");
    drain_pending(&mut game);

    assert_eq!(game.current_or_last_known_power(target_id), Some(8));
    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target_id)
        .expect("the target remains");
    assert!(game.permanent_has_executable_keyword(target, KeywordAbility::Trample));

    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::Two;
    for attacker in [target_id, other_id] {
        let permanent = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker)
            .expect("the attacker remains");
        permanent.attacking = true;
        permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    }
    let blocker = creature(10_003, cards::GRIZZLY_BEARS, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    let seats = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, attacker } if blocker == blocker_id => Some(attacker),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(seats, vec![target_id]);
}

/// Two per *other* creature, so the Redeemer's own arrival is not counted.
#[test]
fn the_redeemer_gains_two_for_each_other_creature() {
    let mut game = ready();
    for index in 0..2 {
        game.battlefield.push(creature(
            10_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.battlefield
        .push(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two));
    let before = game.players[PlayerId::One.index()].life;

    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(10_200, cards::GOLDNIGHT_REDEEMER, PlayerId::One),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before + 4,
        "two of mine at two apiece; theirs and itself excluded",
    );
}

/// The Fettergeist's tax counts other creatures, so alone the payment is
/// zero. The choice is still offered -- "unless you pay {0}" is a real
/// decision -- but taking it costs nothing.
#[test]
fn a_lone_fettergeist_pays_nothing() {
    let mut game = ready();
    let geist = creature(10_000, cards::FETTERGEIST, PlayerId::One);
    let geist_id = geist.card.id;
    game.battlefield.push(geist);

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(&mut game, usize::MAX);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == geist_id),
        "an empty pool still covered an empty cost",
    );
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 0);
}

/// With company it costs one per other creature, and declining sacrifices it.
#[test]
fn a_crowded_fettergeist_costs_one_each() {
    let mut game = ready();
    let geist = creature(10_000, cards::FETTERGEIST, PlayerId::One);
    let geist_id = geist.card.id;
    game.battlefield.push(geist);
    for index in 0..2 {
        game.battlefield.push(creature(
            10_100 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(&mut game, usize::MAX);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == geist_id),
        "two mana covered two other creatures",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        0,
        "and both were spent",
    );
}

/// The control: declining lets it go.
#[test]
fn declining_the_fettergeist_tax_sacrifices_it() {
    let mut game = ready();
    let geist = creature(10_000, cards::FETTERGEIST, PlayerId::One);
    let geist_id = geist.card.id;
    game.battlefield.push(geist);
    game.battlefield
        .push(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One));
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(&mut game, 0);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == geist_id),
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        5,
        "and nothing was spent",
    );
}
