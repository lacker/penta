//! Three turns tied down, then something better comes out.
//!
//! The counters are a clock the Aura keeps on itself: while any remain the
//! creature under it stays tapped, and the upkeep that finds none left opens
//! it. The two halves are complementary conditions rather than a branch, so
//! the turn that sheds the last counter is not also the turn it hatches.

use super::*;

/// A Cocoon on a Sedge Troll player one controls, resolved through the cast
/// so the enters trigger actually runs.
fn cocooned() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let spell = card(20_000, cards::COCOON, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("a creature of its controller's to wrap");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COCOON)
        .expect("the Aura resolved")
        .card
        .id;
    (game, host_id, aura_id)
}

fn pupa_on(game: &Game, aura: GameObjectId) -> Option<u16> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == aura)
        .map(|permanent| permanent.counters(CounterKind::named("pupa")))
}

/// Runs one of player one's upkeeps.
fn take_an_upkeep(game: &mut Game) {
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(game);
}

fn host_of(game: &Game, host: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == host)
        .expect("still there")
}

#[test]
fn it_taps_the_creature_and_arrives_with_three_counters() {
    let (game, host, aura) = cocooned();

    assert!(host_of(&game, host).tapped, "wrapped up");
    assert_eq!(pupa_on(&game, aura), Some(3));
}

/// While counters remain the creature stays down, which is the whole cost of
/// the card.
#[test]
fn it_holds_the_creature_down_while_counters_remain() {
    let (mut game, host, aura) = cocooned();
    take_an_upkeep(&mut game);

    assert_eq!(pupa_on(&game, aura), Some(2), "one counter shed");
    assert!(
        host_of(&game, host).tapped,
        "and the untap step left it alone",
    );
}

/// The upkeep that finds no counters left opens it: the Aura goes, and the
/// creature is bigger and flying.
#[test]
fn the_fourth_upkeep_hatches_it() {
    let (mut game, host, aura) = cocooned();
    for _ in 0..3 {
        take_an_upkeep(&mut game);
    }

    assert_eq!(
        pupa_on(&game, aura),
        Some(0),
        "shedding the last counter does not also hatch it",
    );
    assert_eq!(
        host_of(&game, host).counters(CounterKind::PlusOnePlusOne),
        0,
        "the reward waits for the upkeep that finds no counter",
    );

    take_an_upkeep(&mut game);

    assert_eq!(pupa_on(&game, aura), None, "the Aura is gone");
    let host = host_of(&game, host);
    assert_eq!(
        host.counters(CounterKind::PlusOnePlusOne),
        1,
        "and left something behind",
    );
    assert!(game.permanent_has_executable_keyword(host, KeywordAbility::Flying));
    assert!(!host.tapped, "nothing is holding it down any more");
}
