//! Two removal spells that can only aim at a creature which has already
//! dealt damage this turn.
//!
//! The predicate is a targeting restriction, so an unbloodied creature is not
//! merely a bad choice -- it is never offered. The record is the mirror of
//! the one Giant Shark reads, and lasts the turn rather than the combat.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

/// Puts the removal spell in hand with mana for it, and answers whether the
/// named creature is offered as a target.
fn offers(game: &Game, spell: GameObjectId, victim: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).contains(&cast_action(
        spell,
        vec![Target::Permanent(victim)],
        Vec::new(),
        0,
    ))
}

fn stage(spell_definition: crate::ids::CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let victim = creature(10_000, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);

    let spell = card(20_000, spell_definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    (game, spell_id, victim_id)
}

/// The Arrow waits for the creature to do something first.
#[test]
fn the_arrow_only_aims_at_a_creature_that_has_dealt_damage() {
    let (mut game, arrow, victim) = stage(cards::AVENGING_ARROW);
    assert!(
        !offers(&game, arrow, victim),
        "a creature that has done nothing is no target",
    );

    game.damage_target_from(Some(victim), Some(Target::Player(PlayerId::One)), 4);
    assert!(offers(&game, arrow, victim), "now it has connected");

    game.apply(
        PlayerId::One,
        cast_action(arrow, vec![Target::Permanent(victim)], Vec::new(), 0),
    )
    .expect("the cast is legal");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == victim),
        "and the Arrow destroys it",
    );
}

/// Damage to a creature counts too, not only damage to a player.
#[test]
fn damage_to_anything_makes_a_creature_a_target() {
    let (mut game, arrow, victim) = stage(cards::AVENGING_ARROW);
    let bystander = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    game.damage_target_from(Some(victim), Some(Target::Permanent(bystander_id)), 1);
    assert!(
        offers(&game, arrow, victim),
        "the recipient does not matter",
    );
}

/// The record lasts the turn and no longer.
#[test]
fn the_record_clears_when_the_turn_does() {
    let (mut game, arrow, victim) = stage(cards::AVENGING_ARROW);
    game.damage_target_from(Some(victim), Some(Target::Player(PlayerId::One)), 4);
    assert!(offers(&game, arrow, victim));

    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.commit_next_turn(PlayerId::One, Vec::new());
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    assert!(
        !offers(&game, arrow, victim),
        "a new turn, and the creature is clean again",
    );
}

/// The Swing shrinks rather than destroys, so a big enough creature survives.
#[test]
fn the_swing_takes_five_off_both_halves() {
    let (mut game, swing, victim) = stage(cards::EXECUTIONERS_SWING);
    game.damage_target_from(Some(victim), Some(Target::Player(PlayerId::One)), 4);

    game.apply(
        PlayerId::One,
        cast_action(swing, vec![Target::Permanent(victim)], Vec::new(), 0),
    )
    .expect("the cast is legal");
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == victim),
        "a 4/4 with -5/-5 has no toughness left",
    );
}
