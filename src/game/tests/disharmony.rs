//! Pulling an attacker out of combat and keeping it for the turn.
//!
//! Three effects in sequence and a casting window narrower than any other
//! card's: combat has begun and the blockers are not committed. The window is
//! the interesting half, because a spell that could be cast a step later
//! would be answering a board that no longer exists.

use super::*;

/// Player two attacking player one, with Disharmony in player one's hand.
fn attacked() -> (Game, GameObjectId, CardInstanceId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = true;
    let mut attacker = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    attacker.tapped = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let spell = card(20_000, cards::DISHARMONY, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    game.priority = PlayerId::One;
    (game, attacker_id, spell_id)
}

fn castable(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

#[test]
fn it_untaps_the_attacker_pulls_it_from_combat_and_takes_it() {
    let (mut game, attacker_id, spell_id) = attacked();

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the attacker is a legal target");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    let stolen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .expect("still there");
    assert!(!stolen.tapped, "untapped");
    assert!(!stolen.attacking, "and out of combat");
    assert_eq!(stolen.controller, PlayerId::One, "and under new management");
}

/// The window opens with combat.
#[test]
fn it_cannot_be_cast_in_the_main_phase() {
    let (mut game, _attacker, spell_id) = attacked();
    game.step = Step::PrecombatMain;

    assert!(!castable(&game, spell_id));
}

/// And shuts before the blockers are in, which is the whole point of the
/// restriction: it is a decision made before the defender has committed.
/// Nobody holds priority inside the declaration itself, so the last window is
/// the declare-attackers step.
#[test]
fn it_cannot_be_cast_once_blockers_are_declared() {
    let (mut game, _attacker, spell_id) = attacked();
    assert!(
        castable(&game, spell_id),
        "the attack is declared and the blocks are not"
    );

    game.step = Step::DeclareBlockers;
    game.blockers_declared = true;
    assert!(!castable(&game, spell_id));
}
