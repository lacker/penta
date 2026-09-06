//! Two static abilities whose size is read off the board rather than printed.
//! Domain counts basic land types, so the same creature is a different size
//! on every board; "no untapped lands" is a negated count, which is the shape
//! that turns itself permanently on if the count cannot be taken. Both are
//! checked at more than one board state, since either would look right at one.

use super::*;

/// One creature under player one, with `lands` on the battlefield.
fn board(definition: CardDefinitionId, lands: &[(CardDefinitionId, bool)]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut subject = creature(46_000, definition, PlayerId::One);
    subject.entered_controller_turn = 0;
    let subject_id = subject.card.id;
    game.battlefield.push(subject);
    for (index, (land, tapped)) in lands.iter().enumerate() {
        let mut permanent = creature(
            46_100 + u32::try_from(index).expect("a small fixture"),
            *land,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        permanent.tapped = *tapped;
        game.battlefield.push(permanent);
    }
    (game, subject_id)
}

fn stats(game: &Game, id: GameObjectId) -> (i16, i16) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is on the battlefield");
    (
        game.power(permanent).expect("power"),
        game.toughness(permanent).expect("toughness"),
    )
}

#[test]
fn domain_counts_types_not_lands() {
    let (one_type, giant) = board(
        cards::WAYFARING_GIANT,
        &[(cards::MOUNTAIN, false), (cards::MOUNTAIN, false)],
    );
    assert_eq!(
        stats(&one_type, giant),
        (2, 4),
        "two Mountains are one basic land type, so the 1/3 grows once"
    );

    let (three_types, giant) = board(
        cards::WAYFARING_GIANT,
        &[
            (cards::MOUNTAIN, false),
            (cards::ISLAND, false),
            (cards::PLAINS, false),
        ],
    );
    assert_eq!(stats(&three_types, giant), (4, 6));
}

#[test]
fn domain_can_go_entirely_into_power() {
    let (game, scout) = board(
        cards::KAVU_SCOUT,
        &[(cards::MOUNTAIN, false), (cards::ISLAND, false)],
    );
    assert_eq!(
        stats(&game, scout),
        (2, 2),
        "a 0/2 whose toughness the clause never touches"
    );
}

#[test]
fn an_untapped_land_switches_the_bonus_off() {
    let (tapped_out, cat) = board(cards::SCORIA_CAT, &[(cards::MOUNTAIN, true)]);
    assert_eq!(stats(&tapped_out, cat), (6, 6), "everything is spent");

    let (holding_up, cat) = board(
        cards::SCORIA_CAT,
        &[(cards::MOUNTAIN, true), (cards::MOUNTAIN, false)],
    );
    assert_eq!(
        stats(&holding_up, cat),
        (3, 3),
        "one untapped land is enough to turn the negated count off"
    );
}

#[test]
fn an_empty_board_still_counts_as_no_untapped_lands() {
    let (game, grappler) = board(cards::SPUR_GRAPPLER, &[]);
    assert_eq!(stats(&game, grappler), (4, 2), "zero is zero");
}
