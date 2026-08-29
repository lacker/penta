//! The M13 Rings.
//!
//! Each is an Equipment whose upkeep bonus is conditioned on the equipped
//! creature's colour, so the same Ring grows one creature and not another.
//! The condition is an intervening-if read as the trigger would go on the
//! stack, which is what makes moving the Ring change the answer.

use super::*;

/// A Ring already attached to a creature of the given definition.
fn ring_on(ring: CardDefinitionId, creature_definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let host = creature(10_000, creature_definition, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let mut equipment = creature(10_001, ring, PlayerId::One);
    equipment.attached_to = Some(host_id);
    game.battlefield.push(equipment);
    game.check_state_based_actions();
    (game, host_id)
}

fn counters(game: &Game, permanent: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

fn take_upkeep(game: &mut Game) {
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(game);
}

/// Ring of Thune is white, and Savannah Lions is a white creature.
#[test]
fn a_matching_colour_earns_the_counter() {
    let (mut game, host) = ring_on(cards::RING_OF_THUNE, cards::SAVANNAH_LIONS);
    assert_eq!(counters(&game, host), 0);

    take_upkeep(&mut game);
    assert_eq!(counters(&game, host), 1);

    take_upkeep(&mut game);
    assert_eq!(counters(&game, host), 2, "every upkeep, not just the first");
}

/// The same Ring on a red creature does nothing at all.
#[test]
fn a_different_colour_earns_nothing() {
    let (mut game, host) = ring_on(cards::RING_OF_THUNE, cards::SEDGE_TROLL);

    take_upkeep(&mut game);

    assert_eq!(counters(&game, host), 0);
}

/// Ring of Valkas is the red one, so the same red creature grows under it.
#[test]
fn each_ring_names_its_own_colour() {
    let (mut game, host) = ring_on(cards::RING_OF_VALKAS, cards::SEDGE_TROLL);

    take_upkeep(&mut game);

    assert_eq!(counters(&game, host), 1);
}

/// Ring of Valkas also grants haste, which the equipped creature keeps for
/// as long as the Ring is on it.
#[test]
fn the_ring_grants_its_keyword_while_attached() {
    let (mut game, host) = ring_on(cards::RING_OF_VALKAS, cards::SEDGE_TROLL);

    let has_haste = |game: &Game| {
        let host = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == host)
            .expect("still there");
        game.permanent_has_executable_keyword(host, KeywordAbility::Haste)
    };
    assert!(has_haste(&game));

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.definition == cards::RING_OF_VALKAS)
        .expect("the Ring is there")
        .attached_to = None;
    assert!(!has_haste(&game), "and loses it when the Ring comes off");
}
