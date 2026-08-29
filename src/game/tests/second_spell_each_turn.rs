//! "Your second spell each turn."
//!
//! The count is read after the spell that caused the trigger is already
//! counted, so the second one compares equal to two. Exactly the second: a
//! third spell in the same turn does nothing, and the count resets when the
//! turn does.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.spells_cast_this_turn = [0; 2];
    game
}

/// The Specialist out, with `count` Bolts in hand and mana for all of them.
fn board(count: usize) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready();
    let specialist = creature(10_000, cards::INCURSION_SPECIALIST, PlayerId::One);
    let specialist_id = specialist.card.id;
    game.battlefield.push(specialist);
    let mut bolts = Vec::new();
    for index in 0..count {
        let bolt = card(
            20_000 + u32::try_from(index).expect("small"),
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        );
        bolts.push(bolt.id);
        game.players[PlayerId::One.index()].hand.push(bolt);
    }
    game.players[PlayerId::One.index()].mana_pool.red =
        u16::try_from(count).expect("a handful of Bolts");
    (game, specialist_id, bolts)
}

fn cast(game: &mut Game, bolt: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt))
        .expect("the Bolt is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
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

/// The first spell does nothing; the second pumps it and makes it evasive.
#[test]
fn the_second_spell_pumps_it_and_the_first_does_not() {
    let (mut game, specialist, bolts) = board(2);
    assert_eq!(stats(&game, specialist), (Some(1), Some(3)));

    cast(&mut game, bolts[0]);
    assert_eq!(
        stats(&game, specialist),
        (Some(1), Some(3)),
        "the first spell is not the second",
    );

    cast(&mut game, bolts[1]);
    assert_eq!(
        stats(&game, specialist),
        (Some(3), Some(3)),
        "+2/+0 on the second",
    );
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == specialist)
        .expect("still there");
    assert!(
        game.has_applied_rule(permanent, AppliedRuleDef::CANNOT_BE_BLOCKED),
        "and it cannot be blocked this turn",
    );
}

/// Exactly the second: a third spell adds nothing more.
#[test]
fn a_third_spell_adds_nothing() {
    let (mut game, specialist, bolts) = board(3);
    for bolt in &bolts {
        cast(&mut game, *bolt);
    }

    assert_eq!(
        stats(&game, specialist),
        (Some(3), Some(3)),
        "one pump, not two",
    );
}
