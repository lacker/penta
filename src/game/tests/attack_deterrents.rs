//! "Can't attack if ..." — the mirror of "can't attack unless ...".
//!
//! The negation is over the existential rather than the object: Orgg is
//! stopped when *anything* matches, where Merfolk of the Pearl Trident's
//! cousin is stopped when *nothing* does. A negated object predicate cannot
//! express that, which is why the two clauses are separate.
//!
//! It is also read as attackers are declared, so tapping the deterrent frees
//! the Orgg without either creature being touched.

use super::*;

/// An Orgg for player one, with `deterrent` under player two.
fn orgg_facing(deterrent: Option<CardDefinitionId>) -> (Game, GameObjectId, Option<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let orgg = creature(10_000, cards::ORGG, PlayerId::One);
    let orgg_id = orgg.card.id;
    game.battlefield.push(orgg);
    let deterrent_id = deterrent.map(|definition| {
        let permanent = creature(10_001, definition, PlayerId::Two);
        let id = permanent.card.id;
        game.battlefield.push(permanent);
        id
    });
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::DeclareAttackers;
    (game, orgg_id, deterrent_id)
}

fn can_attack(game: &Game, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::DeclareAttacker { attacker: id, .. } if *id == attacker),
    )
}

#[test]
fn nothing_in_the_way_lets_it_attack() {
    let (game, orgg, _) = orgg_facing(None);
    assert!(can_attack(&game, orgg));
}

/// Serra Angel is a 4/4, so an untapped one stops it.
#[test]
fn an_untapped_big_creature_stops_it() {
    let (game, orgg, _) = orgg_facing(Some(cards::SERRA_ANGEL));
    assert!(!can_attack(&game, orgg));
}

/// Read as attackers are declared: tapping the deterrent frees the Orgg.
#[test]
fn tapping_the_deterrent_frees_it() {
    let (mut game, orgg, deterrent) = orgg_facing(Some(cards::SERRA_ANGEL));
    let deterrent = deterrent.expect("there is one");
    assert!(!can_attack(&game, orgg));

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == deterrent)
        .expect("still there")
        .tapped = true;

    assert!(can_attack(&game, orgg), "a tapped creature deters nothing");
}

/// A small creature is not one with power 3 or greater.
#[test]
fn a_small_creature_does_not_stop_it() {
    let (game, orgg, _) = orgg_facing(Some(cards::SEDGE_TROLL));
    assert!(can_attack(&game, orgg), "a 2/2 is under the bar");
}

/// The deterrent has to be the defending player's, not its controller's own.
#[test]
fn its_own_big_creature_does_not_stop_it() {
    let (mut game, orgg, _) = orgg_facing(None);
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::One));

    assert!(can_attack(&game, orgg));
}
