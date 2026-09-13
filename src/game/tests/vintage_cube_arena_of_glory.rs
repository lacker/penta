//! Arena of Glory: a red source that buys one creature a surprise, paid for
//! out of next turn's untap step.

use super::*;

/// The Arena on the battlefield since last turn, with `mountains` Mountains
/// beside it and a Dwarven Soldier in hand.
fn staged(mountains: usize) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for _ in 0..mountains {
        game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
            .expect("cataloged");
    }
    let arena = game
        .put_onto_battlefield(PlayerId::One, cards::ARENA_OF_GLORY)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    let soldier = game
        .build_zone(PlayerId::One, &[cards::DWARVEN_SOLDIER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let soldier_id = soldier.id;
    game.players[0].hand.push(soldier);
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    drain_pending(&mut game);
    (game, arena, soldier_id)
}

fn arena_abilities(game: &Game, arena: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == arena),
        )
        .collect()
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// Activates the exert ability, which is the one that costs mana.
fn exert_for_two_red(game: &mut Game, arena: GameObjectId) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let action = arena_abilities(game, arena)
        .into_iter()
        .find(|action| {
            let Action::ActivateManaAbility { ability, .. } = action else {
                return false;
            };
            matches!(ability, crate::AbilityOrigin::Printed { ability, .. } if ability.0 == 2)
        })
        .expect("the exert ability is offered");
    game.apply(PlayerId::One, action).expect("it activates");
}

fn cast_soldier(game: &mut Game, soldier: GameObjectId) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == soldier))
        .expect("two red casts a {1}{R} creature");
    game.apply(PlayerId::One, cast).expect("it is cast");
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn soldier_on_battlefield(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::DWARVEN_SOLDIER)
        .expect("the Soldier resolved")
}

/// A Mountain lets it come down untapped; without one it arrives tapped.
#[test]
fn it_enters_tapped_without_a_mountain() {
    let mut game = ready_game();
    game.battlefield.clear();

    let arena = game
        .put_onto_battlefield(PlayerId::One, cards::ARENA_OF_GLORY)
        .expect("cataloged");
    drain_pending(&mut game);
    assert!(
        permanent(&game, arena).tapped,
        "no Mountain, no untapped land"
    );

    let (game, second, _) = staged(1);
    assert!(
        !permanent(&game, second).tapped,
        "a Mountain lets it come down ready",
    );
}

/// The plain ability adds one red and nothing else.
#[test]
fn the_plain_ability_adds_one_red() {
    let (mut game, arena, _) = staged(1);

    let tap = arena_abilities(&game, arena)
        .into_iter()
        .find(|action| {
            let Action::ActivateManaAbility { ability, .. } = action else {
                return false;
            };
            matches!(ability, crate::AbilityOrigin::Printed { ability, .. } if ability.0 == 1)
        })
        .expect("the plain tap is offered");
    game.apply(PlayerId::One, tap).expect("it activates");

    assert_eq!(game.players[0].mana_pool.red, 1);
    assert!(permanent(&game, arena).tapped);
}

/// The exert ability turns one red into two, and the land pays for it by
/// missing its next untap step.
#[test]
fn exerting_it_costs_an_untap_step() {
    let (mut game, arena, _) = staged(1);

    exert_for_two_red(&mut game, arena);

    assert_eq!(game.players[0].mana_pool.red, 2, "one red in, two out");
    assert!(permanent(&game, arena).tapped);

    game.start_next_turn();
    game.start_next_turn();

    assert!(
        permanent(&game, arena).tapped,
        "and it does not untap on your next turn",
    );
}

/// Mana spent on a creature spell gives that creature haste, and the haste
/// outlives the spell.
#[test]
fn mana_spent_on_a_creature_gives_it_haste() {
    let (mut game, arena, soldier) = staged(1);
    exert_for_two_red(&mut game, arena);

    cast_soldier(&mut game, soldier);

    assert!(
        game.permanent_has_executable_keyword(soldier_on_battlefield(&game), KeywordAbility::Haste),
        "the creature it paid for can attack at once",
    );
}

/// Ordinary red does not: the haste comes from the mana rather than from the
/// creature.
#[test]
fn other_mana_leaves_it_summoning_sick() {
    let (mut game, _arena, soldier) = staged(1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);

    cast_soldier(&mut game, soldier);

    assert!(
        !game
            .permanent_has_executable_keyword(soldier_on_battlefield(&game), KeywordAbility::Haste),
        "nothing about the Soldier gives it haste",
    );
}

/// "If mana generated this way is spent to pay any part of a creature
/// spell's cost... that creature spell will gain haste." One of its two red
/// is part enough.
#[test]
fn one_of_the_two_is_enough_to_carry_the_haste() {
    let (mut game, arena, soldier) = staged(1);
    exert_for_two_red(&mut game, arena);
    // Spend one of the Arena's red elsewhere, so what is left of it in the
    // pool is a single red beside an ordinary one.
    game.players[0].mana_pool.red = 1;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    cast_soldier(&mut game, soldier);

    assert!(
        game.permanent_has_executable_keyword(soldier_on_battlefield(&game), KeywordAbility::Haste),
        "one Arena red among the mana that paid for it is enough",
    );
}

/// "If the mana is spent on two different creature spells, each of those
/// spells will gain haste until end of turn." One activation, two hasty
/// creatures -- one-drops, so that each spell takes exactly one of the two
/// red rather than the first spell taking both.
#[test]
fn the_two_red_can_haste_two_different_creatures() {
    let (mut game, arena, _soldier) = staged(1);
    game.players[0].hand.clear();
    let mut goblins = Vec::new();
    for instance in [85_600, 85_601] {
        let goblin = card(instance, cards::MONSS_GOBLIN_RAIDERS, PlayerId::One);
        goblins.push(goblin.id);
        game.players[0].hand.push(goblin);
    }
    exert_for_two_red(&mut game, arena);

    for goblin in goblins {
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == goblin))
            .expect("one red casts a one-drop");
        game.apply(PlayerId::One, cast).expect("it is cast");
        drain_pending(&mut game);
    }

    let hasty = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::MONSS_GOBLIN_RAIDERS)
        .filter(|permanent| game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste))
        .count();
    assert_eq!(hasty, 2, "the mana carried its rider to both of them");
}

/// "The mana can be spent on anything, not just creature spells." A
/// noncreature spell takes it and simply gains nothing.
#[test]
fn the_mana_pays_for_noncreature_spells_too() {
    let (mut game, arena, _soldier) = staged(1);
    let bolt = card(85_500, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    exert_for_two_red(&mut game, arena);
    let life = game.players[1].life;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("the Arena's red pays for a Bolt like any other red");
    game.apply(PlayerId::One, cast).expect("it is cast");
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, life - 3, "it resolved on their face");
    assert_eq!(
        game.players[0].mana_pool.red, 1,
        "and the other red is still there",
    );
}

#[test]
fn haste_rider_survives_the_land_leaving_and_expires_at_cleanup() {
    let (mut game, arena, soldier) = staged(1);
    exert_for_two_red(&mut game, arena);
    game.destroy_permanent_without_regeneration(arena);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
            .unwrap();
    cast_soldier(&mut rebuilt, soldier);
    assert!(
        rebuilt.permanent_has_executable_keyword(
            soldier_on_battlefield(&rebuilt),
            KeywordAbility::Haste
        )
    );
    rebuilt.cleanup();
    assert!(
        !rebuilt.permanent_has_executable_keyword(
            soldier_on_battlefield(&rebuilt),
            KeywordAbility::Haste
        )
    );
}
