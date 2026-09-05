//! Three lands that each pay more once the other two are down.
//!
//! Each asks for the other two-part land subtypes, so a permanent with every
//! nonbasic land type can stand in for both missing pieces. The amount is
//! resolved as the activation is offered, which keeps the payment planner and
//! the mana pool agreeing about how much a tap is worth.

use super::*;

const TRON: [CardDefinitionId; 3] = [
    cards::URZA_S_MINE,
    cards::URZA_S_POWER_PLANT,
    cards::URZA_S_TOWER,
];

/// A board holding exactly the listed lands, all untapped.
fn lands(pieces: &[CardDefinitionId]) -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    let mut ids = Vec::new();
    for (index, definition) in pieces.iter().enumerate() {
        let land = creature(
            10_000 + u32::try_from(index).expect("a short list"),
            *definition,
            PlayerId::One,
        );
        ids.push(land.card.id);
        game.battlefield.push(land);
    }
    game.priority = PlayerId::One;
    (game, ids)
}

/// Taps `land` for mana and reports how much colorless arrived.
fn tap_for_colorless(game: &mut Game, land: GameObjectId) -> u16 {
    let before = game.players[PlayerId::One.index()].mana_pool.colorless;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == land),
        )
        .expect("an untapped land offers its mana ability");
    game.apply(PlayerId::One, action).expect("tapping is free");
    drain_pending(game);
    game.players[PlayerId::One.index()].mana_pool.colorless - before
}

/// The control: alone, each piece is an ordinary colorless land.
#[test]
fn one_piece_alone_taps_for_one() {
    for piece in TRON {
        let (mut game, ids) = lands(&[piece]);
        assert_eq!(tap_for_colorless(&mut game, ids[0]), 1);
    }
}

/// Two thirds is still one apiece: each land asks for *both* other subtypes.
#[test]
fn two_pieces_still_tap_for_one_each() {
    let (mut game, ids) = lands(&[cards::URZA_S_MINE, cards::URZA_S_TOWER]);
    assert_eq!(tap_for_colorless(&mut game, ids[0]), 1, "the Mine");
    assert_eq!(tap_for_colorless(&mut game, ids[1]), 1, "the Tower");
}

/// Assembled: two, two, and three, so seven mana from three lands.
#[test]
fn the_assembled_set_taps_for_seven() {
    let (mut game, ids) = lands(&TRON);
    assert_eq!(tap_for_colorless(&mut game, ids[0]), 2, "the Mine");
    assert_eq!(tap_for_colorless(&mut game, ids[1]), 2, "the Power Plant");
    assert_eq!(tap_for_colorless(&mut game, ids[2]), 3, "the Tower");
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 7);
}

/// The amount is read as each land is tapped rather than fixed when the set
/// came together.
#[test]
fn losing_a_piece_takes_the_bonus_away_again() {
    let (mut game, ids) = lands(&TRON);
    assert_eq!(tap_for_colorless(&mut game, ids[2]), 3, "assembled");

    game.battlefield
        .retain(|permanent| permanent.card.id != ids[0]);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == ids[1])
        .expect("the Power Plant is still there")
        .tapped = false;

    assert_eq!(
        tap_for_colorless(&mut game, ids[1]),
        1,
        "with the Mine gone the Power Plant is a plain land again",
    );
}

/// A second copy of one piece does not stand in for a missing one.
#[test]
fn a_duplicate_piece_does_not_complete_the_set() {
    let (mut game, ids) = lands(&[
        cards::URZA_S_MINE,
        cards::URZA_S_MINE,
        cards::URZA_S_POWER_PLANT,
    ]);
    assert_eq!(
        tap_for_colorless(&mut game, ids[2]),
        1,
        "no Tower, no bonus"
    );
}

/// The pieces have to be yours: the predicate is controller-scoped.
#[test]
fn pieces_across_the_table_do_not_count() {
    let (mut game, ids) = lands(&[cards::URZA_S_TOWER]);
    for (index, definition) in [cards::URZA_S_MINE, cards::URZA_S_POWER_PLANT]
        .into_iter()
        .enumerate()
    {
        game.battlefield.push(creature(
            20_000 + u32::try_from(index).expect("a short list"),
            definition,
            PlayerId::Two,
        ));
    }
    assert_eq!(tap_for_colorless(&mut game, ids[0]), 1);
}

#[test]
fn each_piece_has_its_two_printed_land_subtypes() {
    for (piece, subtype) in [
        (cards::URZA_S_MINE, "Mine"),
        (cards::URZA_S_POWER_PLANT, "Power-Plant"),
        (cards::URZA_S_TOWER, "Tower"),
    ] {
        let (game, ids) = lands(&[piece]);
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == ids[0])
            .expect("the land is on the battlefield");
        let subtypes = game.effective_subtypes(permanent);
        assert!(subtypes.contains(&"Urza's"));
        assert!(subtypes.contains(&subtype));
    }
}

#[test]
fn planar_nexus_supplies_both_missing_urza_land_types() {
    for (piece, expected) in [
        (cards::URZA_S_MINE, 2),
        (cards::URZA_S_POWER_PLANT, 2),
        (cards::URZA_S_TOWER, 3),
    ] {
        let (mut game, ids) = lands(&[piece, cards::PLANAR_NEXUS]);
        assert_eq!(tap_for_colorless(&mut game, ids[0]), expected);
    }
}

#[test]
fn planar_nexus_has_every_urza_land_subtype() {
    let (game, ids) = lands(&[cards::PLANAR_NEXUS]);
    let nexus = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ids[0])
        .expect("Planar Nexus is on the battlefield");
    let subtypes = game.effective_subtypes(nexus);
    for subtype in ["Urza's", "Mine", "Power-Plant", "Tower"] {
        assert!(subtypes.contains(&subtype), "missing {subtype}");
    }
}
