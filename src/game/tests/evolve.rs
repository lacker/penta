//! Evolve.
//!
//! The comparison is against the source's power and toughness *at the moment
//! the creature enters*, so a creature that has already grown stops evolving
//! from arrivals it has outgrown. That feedback is the whole keyword, and it
//! is what these drive.

use super::*;

fn raptor_board() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let raptor = creature(10_000, cards::CLOUDFIN_RAPTOR, PlayerId::One);
    let raptor_id = raptor.card.id;
    game.battlefield.push(raptor);
    (game, raptor_id)
}

fn arrive(game: &mut Game, id: u32, definition: CardDefinitionId, controller: PlayerId) {
    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(id, definition, controller),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pending(game);
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("on the battlefield")
        .counters(CounterKind::PlusOnePlusOne)
}

#[test]
fn a_bigger_creature_evolves_it() {
    let (mut game, raptor_id) = raptor_board();
    // A 0/1 meeting a 2/1: greater power.
    arrive(&mut game, 10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    assert_eq!(counters(&game, raptor_id), 1);
}

/// Once it has grown, the same arrival no longer beats it -- which is the
/// comparison being against current power rather than printed.
#[test]
fn it_stops_evolving_from_what_it_has_outgrown() {
    let (mut game, raptor_id) = raptor_board();
    arrive(&mut game, 10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    assert_eq!(counters(&game, raptor_id), 1, "a 1/2 now");

    arrive(&mut game, 10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    assert_eq!(
        counters(&game, raptor_id),
        2,
        "a 2/1 still has greater power than 1/2"
    );

    arrive(&mut game, 10_003, cards::SAVANNAH_LIONS, PlayerId::One);
    assert_eq!(
        counters(&game, raptor_id),
        2,
        "a 2/1 no longer beats a 2/3 on either half"
    );
}

#[test]
fn an_opposing_creature_does_not_evolve_it() {
    let (mut game, raptor_id) = raptor_board();
    arrive(&mut game, 10_001, cards::SEDGE_TROLL, PlayerId::Two);
    assert_eq!(counters(&game, raptor_id), 0);
}

/// A creature does not evolve itself as it arrives, however big it is.
#[test]
fn it_does_not_evolve_from_its_own_arrival() {
    let mut game = ready_game();
    arrive(&mut game, 10_000, cards::ADAPTIVE_SNAPJAW, PlayerId::One);
    let snapjaw = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ADAPTIVE_SNAPJAW)
        .expect("it entered");
    assert_eq!(snapjaw.counters(CounterKind::PlusOnePlusOne), 0);
}
