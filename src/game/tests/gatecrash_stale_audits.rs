//! Gatecrash cards whose audit lines had gone stale.
//!
//! These checks concentrate on the compositions where the newly reusable
//! primitives cross a rules boundary: linked zone moves, controller snapshots,
//! sequential sacrifices, resolving-spell movement, and temporary triggers.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn offers(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

fn cast_at(game: &mut Game, spell: CardInstanceId, target: Target) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell
                    && choices.targets().iter().flat_map(TargetSelection::targets)
                        .any(|actual| *actual == target))
        })
        .expect("the targeted cast is offered");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(game);
}

/// Realmwright adds the chosen type rather than applying the replacing form
/// used by effects that say a land "is" one type.
#[test]
fn realmwright_preserves_a_lands_existing_basic_land_types() {
    let mut game = ready();
    let mut realmwright = creature(10_000, cards::REALMWRIGHT, PlayerId::One);
    realmwright.chosen_basic_land_type = Some(BasicLandType::Island);
    let forest = creature(10_001, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.extend([realmwright, forest]);

    let forest = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == forest_id)
        .expect("the Forest remains on the battlefield");
    let types = game.effective_land_types(forest);
    assert!(types[BasicLandType::Forest.index()]);
    assert!(types[BasicLandType::Island.index()]);
}

/// The token's effect values are evaluated while it is created. A larger
/// creature entering later does not turn the Ooze into a live */* token.
#[test]
fn miming_slime_fixes_its_token_stats_on_creation() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One));
    let slime = card(20_000, cards::MIMING_SLIME, PlayerId::One);
    let slime_id = slime.id;
    game.players[PlayerId::One.index()].hand.push(slime);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == slime_id))
        .expect("Miming Slime is castable");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    drain_pending(&mut game);

    let ooze = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Ooze"], &[ManaColor::Green], 2, 2),
            )
        })
        .expect("the 2/2 Ooze was created")
        .card
        .id;
    game.battlefield
        .push(creature(10_001, cards::SERRA_ANGEL, PlayerId::One));

    assert_eq!(stats(&game, ooze), (Some(2), Some(2)));
}

/// A generic choice feeding `Sacrifice` must honor the same prohibition as
/// the sacrifice itself: the protected player is not offered an unusable
/// choice, and Devour Flesh consequently gains them no life.
#[test]
fn devour_flesh_respects_forced_sacrifice_prohibitions() {
    let mut game = ready();
    let mut tamiyo = creature(10_000, cards::TAMIYO_COLLECTOR_OF_TALES, PlayerId::Two);
    tamiyo.add_counters(CounterKind::Loyalty, 5);
    game.battlefield.extend([
        tamiyo,
        creature(10_001, cards::WALL_OF_STONE, PlayerId::Two),
    ]);
    let spell = card(20_000, cards::DEVOUR_FLESH, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    cast_at(&mut game, spell_id, Target::Player(PlayerId::Two));

    assert_eq!(game.players[PlayerId::Two.index()].life, 20);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.card.definition == cards::WALL_OF_STONE && permanent.controller == PlayerId::Two
    }));
}

/// The pump runs once and then the ability is gone for the turn, however much
/// green is left.
#[test]
fn the_oculus_pumps_once_a_turn() {
    let mut game = ready();
    let oculus = creature(10_000, cards::FRILLED_OCULUS, PlayerId::One);
    let oculus_id = oculus.card.id;
    game.battlefield.push(oculus);
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    assert_eq!(stats(&game, oculus_id), (Some(1), Some(3)));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == oculus_id))
        .expect("one green and one generic is enough");
    game.apply(PlayerId::One, action)
        .expect("the cost is payable");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    assert_eq!(stats(&game, oculus_id), (Some(3), Some(5)));
    assert!(
        !offers(&game, oculus_id),
        "the ration closed it for the rest of the turn",
    );
}

/// And it opens again next turn.
#[test]
fn the_ration_returns_with_the_turn() {
    let mut game = ready();
    let oculus = creature(10_000, cards::FRILLED_OCULUS, PlayerId::One);
    let oculus_id = oculus.card.id;
    game.battlefield.push(oculus);
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == oculus_id))
        .expect("on offer");
    game.apply(PlayerId::One, action).expect("payable");
    drain_pending(&mut game);

    // Walked rather than jumped, so the cleanup that ends an
    // until-end-of-turn effect actually happens.
    for _ in 0..12 {
        if game.step == Step::Cleanup {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    assert!(offers(&game, oculus_id), "a new turn, a new activation");
    assert_eq!(
        stats(&game, oculus_id),
        (Some(1), Some(3)),
        "and last turn's pump has worn off",
    );
}

/// Gridlock in hand with `blue` available, over `permanents` nonland
/// permanents and one land.
fn gridlocked(
    permanents: u32,
    blue: u16,
) -> (Game, CardInstanceId, Vec<GameObjectId>, GameObjectId) {
    let mut game = ready();
    let mut ids = Vec::new();
    for index in 0..permanents {
        let permanent = creature(10_000 + index, cards::GRIZZLY_BEARS, PlayerId::Two);
        ids.push(permanent.card.id);
        game.battlefield.push(permanent);
    }
    let land = creature(10_500, cards::ISLAND, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.push(land);

    let spell = card(20_000, cards::GRIDLOCK, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = blue;
    (game, spell_id, ids, land_id)
}

fn offered_shapes(game: &Game, spell: CardInstanceId) -> Vec<(u16, usize)> {
    let mut shapes = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => Some((
                choices.x(),
                choices
                    .targets()
                    .iter()
                    .map(|slot| slot.targets().len())
                    .sum(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    shapes.sort_unstable();
    shapes.dedup();
    shapes
}

#[test]
fn gridlock_takes_exactly_as_many_targets_as_the_x_paid() {
    let (game, spell, _permanents, _land) = gridlocked(3, 4);
    let shapes = offered_shapes(&game, spell);
    assert!(!shapes.is_empty(), "the spell is castable");
    for (x, count) in shapes {
        assert_eq!(
            usize::from(u8::try_from(x).expect("small X")),
            count,
            "X={x} took {count}",
        );
    }
}

/// The land is not a legal target, so three nonland permanents is the
/// ceiling however much blue is spare.
#[test]
fn the_land_is_not_among_the_targets() {
    let (game, spell, _permanents, _land) = gridlocked(3, 8);
    let largest = offered_shapes(&game, spell)
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .expect("something is on offer");
    assert_eq!(largest, 3, "the land does not raise the ceiling");
}

#[test]
fn gridlock_taps_the_permanents_chosen() {
    let (mut game, spell, permanents, land) = gridlocked(3, 3);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell
                    && choices.x() == 2
                    && choices.targets().iter().any(|slot| slot.targets()
                        == [Target::Permanent(permanents[0]), Target::Permanent(permanents[1])]))
        })
        .expect("two of the three is a legal choice");
    game.apply(PlayerId::One, action)
        .expect("three blue covers {X=2}{U}");
    drain_pending(&mut game);

    let tapped = |id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .tapped
    };
    assert!(tapped(permanents[0]) && tapped(permanents[1]));
    assert!(!tapped(permanents[2]), "the untargeted one stayed up");
    assert!(!tapped(land), "and so did the land");
}

/// The token belongs to the destroyed creature's controller, not to the
/// Hybridization's caster.
#[test]
fn rapid_hybridization_gives_the_frog_lizard_to_the_victim() {
    let mut game = ready();
    let victim = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    let spell = card(20_000, cards::RAPID_HYBRIDIZATION, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;

    cast_at(&mut game, spell_id, Target::Permanent(victim_id));

    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Frog", "Lizard"], &[ManaColor::Green], 3, 3),
            )
        })
        .expect("the Frog Lizard was created");
    assert_eq!(token.controller, PlayerId::Two);
}

/// Removing a counter does not make having one a targeting condition. The
/// ability may legally point at a counterless nonland permanent and simply do
/// nothing if no counter is available when it resolves.
#[test]
fn thrull_parasite_can_target_a_counterless_nonland_permanent() {
    let mut game = ready();
    let parasite = creature(10_000, cards::THRULL_PARASITE, PlayerId::One);
    let parasite_id = parasite.card.id;
    let bear = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let creature_id = bear.card.id;
    let land = creature(10_002, cards::ISLAND, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.extend([parasite, bear, land]);

    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == parasite_id => targets
                .iter()
                .flat_map(TargetSelection::targets)
                .next()
                .copied(),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(targets.contains(&Target::Permanent(creature_id)));
    assert!(!targets.contains(&Target::Permanent(land_id)));
}

/// Only creature cards among this spell's four milled cards count; older
/// cards in that graveyard do not.
#[test]
fn coerced_confession_draws_for_its_own_milled_creatures() {
    let mut game = ready();
    game.players[PlayerId::One.index()].library = (0..4)
        .map(|index| card(30_000 + index, cards::ISLAND, PlayerId::One))
        .collect();
    game.players[PlayerId::Two.index()].graveyard.push(card(
        31_000,
        cards::GRIZZLY_BEARS,
        PlayerId::Two,
    ));
    game.players[PlayerId::Two.index()].library = vec![
        card(31_001, cards::ISLAND, PlayerId::Two),
        card(31_002, cards::GRIZZLY_BEARS, PlayerId::Two),
        card(31_003, cards::ISLAND, PlayerId::Two),
        card(31_004, cards::GRIZZLY_BEARS, PlayerId::Two),
    ];
    let spell = card(20_000, cards::COERCED_CONFESSION, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    cast_at(&mut game, spell_id, Target::Player(PlayerId::Two));

    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 2);
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 5);
}

/// Chosen X filters the whole graveyard before the group moves, so every
/// matching creature returns together and the rest stay put.
#[test]
fn immortal_servitude_returns_each_creature_with_the_chosen_mana_value() {
    let mut game = ready();
    game.players[PlayerId::One.index()].graveyard = vec![
        card(30_000, cards::GRIZZLY_BEARS, PlayerId::One),
        card(30_001, cards::SAVANNAH_LIONS, PlayerId::One),
        card(30_002, cards::GRIZZLY_BEARS, PlayerId::One),
    ];
    let spell = card(20_000, cards::IMMORTAL_SERVITUDE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 3;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell_id && choices.x() == 2)
        })
        .expect("X=2 is castable");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
            .count(),
        2,
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS),
        "the one-mana creature stayed behind",
    );
}

/// The first sacrifice can change what remains eligible for the second. The
/// continuations must therefore run in sequence rather than choose both from
/// one stale battlefield snapshot.
#[test]
fn structural_collapse_sacrifices_the_artifact_before_the_land() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::PROPHETIC_PRISM, PlayerId::Two));
    game.battlefield
        .push(creature(10_001, cards::ISLAND, PlayerId::Two));
    let spell = card(20_000, cards::STRUCTURAL_COLLAPSE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;

    cast_at(&mut game, spell_id, Target::Player(PlayerId::Two));

    assert_eq!(game.players[PlayerId::Two.index()].life, 18);
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 2);
}

/// The seven cards belong to the damaged player, while the selected creature
/// enters under the trigger controller's control. The other six stay exiled.
#[test]
fn lord_of_the_void_uses_the_damaged_players_library() {
    let mut game = ready();
    let lord = creature(10_000, cards::LORD_OF_THE_VOID, PlayerId::One);
    let lord_id = lord.card.id;
    game.battlefield.push(lord);
    game.players[PlayerId::Two.index()].library = vec![
        card(30_000, cards::ISLAND, PlayerId::Two),
        card(30_001, cards::GRIZZLY_BEARS, PlayerId::Two),
        card(30_002, cards::ISLAND, PlayerId::Two),
        card(30_003, cards::SAVANNAH_LIONS, PlayerId::Two),
        card(30_004, cards::ISLAND, PlayerId::Two),
        card(30_005, cards::ISLAND, PlayerId::Two),
        card(30_006, cards::ISLAND, PlayerId::Two),
    ];

    game.deal_combat_damage_to_player(lord_id, PlayerId::Two, 7);
    drain_pending(&mut game);

    let stolen = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.owner == PlayerId::Two)
        .collect::<Vec<_>>();
    assert_eq!(stolen.len(), 1);
    assert_eq!(stolen[0].controller, PlayerId::One);
    assert!(
        game.power(stolen[0]).is_some(),
        "the selected card is a creature"
    );
    assert!(game.players[PlayerId::Two.index()].library.is_empty());
    assert_eq!(game.players[PlayerId::Two.index()].exile.len(), 6);
}

/// The installed trigger keeps both the activating player's relation and the
/// amount of the later life-gain event.
#[test]
fn vizkopa_guildmage_uses_the_later_life_gain_amount() {
    let mut game = ready();
    let guildmage = creature(10_000, cards::VIZKOPA_GUILDMAGE, PlayerId::One);
    let guildmage_id = guildmage.card.id;
    game.battlefield.push(guildmage);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == guildmage_id
                    && matches!(ability, AbilityOrigin::Printed { ability, .. }
                        if *ability == AbilityId(1)))
        })
        .expect("the second Guildmage ability is offered");
    game.apply(PlayerId::One, action)
        .expect("the activation is legal");
    drain_pending(&mut game);

    game.gain_life(PlayerId::One, 4);
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, 24);
    assert_eq!(game.players[PlayerId::Two.index()].life, 16);
}

/// Both Guildmage abilities compose through ordinary zone-change triggers:
/// each of the two milled cards is a separate opponent-graveyard event.
#[test]
fn duskmantle_guildmage_counts_each_card_milled_by_its_second_ability() {
    let mut game = ready();
    let guildmage = creature(10_000, cards::DUSKMANTLE_GUILDMAGE, PlayerId::One);
    let guildmage_id = guildmage.card.id;
    game.battlefield.push(guildmage);
    game.players[PlayerId::Two.index()].library = vec![
        card(30_000, cards::ISLAND, PlayerId::Two),
        card(30_001, cards::ISLAND, PlayerId::Two),
    ];
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.black = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let first = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == guildmage_id
                    && matches!(ability, AbilityOrigin::Printed { ability, .. }
                        if *ability == AbilityId::PRIMARY))
        })
        .expect("the turn-long trigger ability is offered");
    game.apply(PlayerId::One, first)
        .expect("the first activation is legal");
    drain_pending(&mut game);

    let second = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, targets, .. }
                if *source == guildmage_id
                    && matches!(ability, AbilityOrigin::Printed { ability, .. }
                        if *ability == AbilityId(1))
                    && targets.iter().flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Player(PlayerId::Two)))
        })
        .expect("the mill ability can target the opponent");
    game.apply(PlayerId::One, second)
        .expect("the second activation is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 2);
    assert_eq!(game.players[PlayerId::Two.index()].life, 18);
}

/// The same temporary trigger must observe the reveal-until-land mill path,
/// not only the fixed-count `Mill` effect printed on the Guildmage itself.
#[test]
fn duskmantle_guildmage_observes_consuming_aberrations_mill() {
    let mut game = ready();
    let guildmage = creature(10_000, cards::DUSKMANTLE_GUILDMAGE, PlayerId::One);
    let guildmage_id = guildmage.card.id;
    game.battlefield.push(guildmage);
    game.battlefield
        .push(creature(10_001, cards::CONSUMING_ABERRATION, PlayerId::One));
    game.players[PlayerId::Two.index()]
        .graveyard
        .push(card(29_999, cards::ISLAND, PlayerId::Two));
    game.players[PlayerId::Two.index()].library = vec![
        card(30_000, cards::ISLAND, PlayerId::Two),
        card(30_001, cards::GRIZZLY_BEARS, PlayerId::Two),
    ];
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let install = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == guildmage_id
                    && matches!(ability, AbilityOrigin::Printed { ability, .. }
                        if *ability == AbilityId::PRIMARY))
        })
        .expect("the turn-long trigger ability is offered");
    game.apply(PlayerId::One, install)
        .expect("the first activation is legal");
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let spell = card(20_000, cards::THINK_TWICE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the spell is castable");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 3);
    assert_eq!(game.players[PlayerId::Two.index()].life, 18);
}

/// The granted cast belongs to the Primordial's controller even though the
/// chosen card is in an opponent's graveyard, and its later graveyard move is
/// replaced with exile.
#[test]
fn diluvian_primordial_casts_an_opponents_spell_free_and_exiles_it() {
    let mut game = ready();
    game.players[PlayerId::One.index()].library = vec![card(30_000, cards::ISLAND, PlayerId::One)];
    let borrowed = card(30_001, cards::THINK_TWICE, PlayerId::Two);
    let borrowed_id = borrowed.id;
    game.players[PlayerId::Two.index()].graveyard.push(borrowed);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == borrowed_id)),
        "an opponent's graveyard stays inaccessible without the exact offer",
    );
    game.put_onto_battlefield(PlayerId::One, cards::DILUVIAN_PRIMORDIAL)
        .expect("Diluvian Primordial is cataloged");
    game.finish_rules_procedure();

    for _ in 0..24 {
        if matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::MayCastGranted { .. })
        ) {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1).min(decision.maximum))
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the target choice is legal");
            continue;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority passes toward the cast offer");
    }
    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::MayCastGranted { .. })
    ));
    let free_cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == borrowed_id))
        .expect("the opponent's spell can be cast during the trigger resolution");
    game.apply(PlayerId::One, free_cast)
        .expect("the granted cast is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 1);
    assert!(
        game.players[PlayerId::Two.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::THINK_TWICE)
    );
}
