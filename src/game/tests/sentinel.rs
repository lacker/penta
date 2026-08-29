//! A toughness-only base set, read off whatever the Sentinel is in combat
//! with.
//!
//! Only toughness moves: the printed power of 1 survives, which is what makes
//! the card a wall rather than a threat. The ability is free and lasts
//! indefinitely, so each use replaces the last rather than stacking.

use super::*;

/// The Sentinel blocking `attacker_power`, with the attacker under player two.
fn blocking(attacker_power: i16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::Two;

    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let mut sentinel = creature(10_001, cards::SENTINEL, PlayerId::One);
    sentinel.blocking = vec![attacker_id];
    let sentinel_id = sentinel.card.id;
    game.battlefield.push(sentinel);

    // Grizzly Bears is 2/2 printed; anything else is set here so one helper
    // covers every size the test wants.
    if attacker_power != 2 {
        attach_constant_resolved_characteristics(
            &mut game,
            attacker_id,
            &[AppliedEffectDef::set_base_power_toughness(
                ValueDef::Constant(i32::from(attacker_power)),
                ValueDef::Constant(2),
            )],
            ContinuousEffectExpiration::Never,
        );
    }

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::One;
    (game, sentinel_id, attacker_id)
}

fn activate(game: &mut Game, sentinel: GameObjectId) {
    // Resolving the last activation passed priority on; the ability is still
    // open, so take it back rather than walking the step around.
    game.priority = PlayerId::One;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == sentinel))
        .expect("a creature it is in combat with is a legal target");
    game.apply(PlayerId::One, action).expect("the cost is zero");
    drain_pending(game);
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

#[test]
fn it_sets_toughness_to_one_more_than_the_blocked_creatures_power() {
    let (mut game, sentinel, _attacker) = blocking(2);
    assert_eq!(stats(&game, sentinel), (Some(1), Some(1)));

    activate(&mut game, sentinel);

    assert_eq!(
        stats(&game, sentinel),
        (Some(1), Some(3)),
        "one plus two, with power untouched",
    );
}

/// Only toughness is set, so a big attacker does not make the Sentinel big.
#[test]
fn power_stays_at_its_printed_one() {
    let (mut game, sentinel, _attacker) = blocking(7);
    activate(&mut game, sentinel);

    assert_eq!(stats(&game, sentinel), (Some(1), Some(8)));
}

/// The effect sets rather than modifies, so a second use replaces the first
/// instead of adding to it.
#[test]
fn a_second_use_replaces_the_first() {
    let (mut game, sentinel, attacker) = blocking(7);
    activate(&mut game, sentinel);
    assert_eq!(stats(&game, sentinel), (Some(1), Some(8)));

    attach_constant_resolved_characteristics(
        &mut game,
        attacker,
        &[AppliedEffectDef::set_base_power_toughness(
            ValueDef::Constant(2),
            ValueDef::Constant(2),
        )],
        ContinuousEffectExpiration::Never,
    );
    activate(&mut game, sentinel);

    assert_eq!(
        stats(&game, sentinel),
        (Some(1), Some(3)),
        "the newer set wins rather than stacking to 11",
    );
}

/// The control: with nothing in combat with it, the ability has no legal
/// target and is not offered.
#[test]
fn out_of_combat_it_is_not_offered() {
    let (mut game, sentinel, attacker) = blocking(2);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == sentinel)
        .expect("still there")
        .blocking
        .clear();
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
        .expect("still there")
        .attacking = false;

    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source, .. } if *source == sentinel)
    ),);
}
