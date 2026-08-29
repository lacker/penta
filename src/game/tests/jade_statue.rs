//! An artifact that is a creature for exactly one combat.
//!
//! Two things make the card what it is: the window is the whole combat phase
//! rather than one step, so the Statue can animate before attackers are chosen
//! or after blockers are, and the animation expires with the combat rather
//! than at cleanup, so it is an inert artifact again in the postcombat main
//! phase.

use super::*;

fn statued() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    let statue = creature(10_000, cards::JADE_STATUE, PlayerId::One);
    let statue_id = statue.card.id;
    game.battlefield.push(statue);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    game.priority = PlayerId::One;
    (game, statue_id)
}

fn offers(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn animate(game: &mut Game, statue: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == statue),
        )
        .expect("the window is open");
    game.apply(PlayerId::One, action)
        .expect("two colorless is enough");
    drain_pending(game);
}

fn on_battlefield(game: &Game, id: GameObjectId) -> Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .clone()
}

fn is_creature(game: &Game, id: GameObjectId) -> bool {
    let permanent = on_battlefield(game, id);
    game.permanent_types(&permanent)
        .expect("the Statue has card types")
        .contains(CardType::Creature)
}

/// The window is the phase, not a step inside it.
#[test]
fn the_window_is_every_combat_step_and_nothing_outside_combat() {
    let (mut game, statue) = statued();
    for step in [
        Step::Upkeep,
        Step::PrecombatMain,
        Step::PostcombatMain,
        Step::End,
    ] {
        game.step = step;
        assert!(!offers(&game, statue), "{step:?} is not combat");
    }
    for step in [
        Step::BeginningOfCombat,
        Step::DeclareAttackers,
        Step::DeclareBlockers,
        Step::CombatDamage,
        Step::EndOfCombat,
    ] {
        game.step = step;
        game.attackers_declared = true;
        game.blockers_declared = true;
        assert!(offers(&game, statue), "{step:?} is combat");
    }
}

/// It opens on the other player's turn too, which is what lets the Statue
/// block.
#[test]
fn the_window_opens_on_the_opponents_turn() {
    let (mut game, statue) = statued();
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    // Past the declaration, where priority is actually offered. In practice
    // the Statue animates a step earlier so it is a creature to declare.
    game.blockers_declared = true;
    assert!(offers(&game, statue));
}

#[test]
fn it_becomes_a_three_six_golem_artifact_creature() {
    let (mut game, statue) = statued();
    game.step = Step::BeginningOfCombat;
    assert!(!is_creature(&game, statue), "an inert artifact to start");

    animate(&mut game, statue);

    let permanent = on_battlefield(&game, statue);
    let types = game
        .permanent_types(&permanent)
        .expect("the Statue has card types");
    assert!(types.contains(CardType::Creature));
    assert!(
        types.contains(CardType::Artifact),
        "it keeps its artifact type rather than trading it away",
    );
    assert!(game.effective_subtypes(&permanent).contains(&"Golem"));
    assert_eq!(
        (game.power(&permanent), game.toughness(&permanent)),
        (Some(3), Some(6)),
    );
}

/// Until end of *combat*, which is shorter than until end of turn: the
/// postcombat main phase finds an artifact again.
#[test]
fn the_animation_ends_with_the_combat_not_the_turn() {
    let (mut game, statue) = statued();
    game.step = Step::BeginningOfCombat;
    animate(&mut game, statue);
    assert!(is_creature(&game, statue));

    while game.step != Step::PostcombatMain {
        game.advance_step();
        drain_pending(&mut game);
    }
    assert!(
        !is_creature(&game, statue),
        "an artifact again in the postcombat main phase",
    );
}
