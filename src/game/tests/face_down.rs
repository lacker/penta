//! The body a face-down permanent presents, and who may see the card under it.

use super::*;

/// A face-down permanent is a 2/2 creature with no name whatever the card
/// under it says, and the card under it is still what the game holds.
#[test]
fn a_face_down_permanent_is_a_nameless_two_two() {
    let mut game = ready_game();
    // Serra Angel is a 4/4 flier with two keywords face up.
    let mut angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    angel.face_down = true;
    game.battlefield.push(angel);

    let permanent = &game.battlefield[0];
    let stats = game
        .creature_stats(permanent)
        .expect("a face-down permanent is a creature");
    assert_eq!(
        (stats.power, stats.toughness),
        (2, 2),
        "the body, not the card",
    );
    assert!(
        !game.has_flying(permanent),
        "and none of the card's abilities",
    );
    assert_eq!(
        game.effective_permanent_name(permanent),
        None,
        "a face-down permanent has no name",
    );
    assert_eq!(
        permanent.card.definition,
        cards::SERRA_ANGEL,
        "the physical card is unchanged underneath",
    );
    assert!(
        !permanent.card.definition.is_token(),
        "and it is not a token",
    );
}

/// Its controller may look at it. Nobody else may.
#[test]
fn only_its_controller_sees_what_it_is() {
    let mut game = ready_game();
    let mut angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    angel.face_down = true;
    game.battlefield.push(angel);

    let mine = game.observe(PlayerId::One);
    let theirs = game.observe(PlayerId::Two);
    assert_eq!(
        mine.battlefield[0].characteristics,
        ObjectCharacteristics::card(cards::SERRA_ANGEL, CardPartId::PRIMARY),
        "its controller knows what they played",
    );
    assert_eq!(
        theirs.battlefield[0].characteristics,
        ObjectCharacteristics::card(cards::FACE_DOWN_CREATURE, CardPartId::PRIMARY),
        "and the opponent sees only a body",
    );
    assert!(
        mine.battlefield[0].face_down && theirs.battlefield[0].face_down,
        "both seats see that it is face down",
    );
    assert_eq!(
        (theirs.battlefield[0].power, theirs.battlefield[0].toughness),
        (Some(2), Some(2)),
        "which is a 2/2 from either side",
    );
}

/// The whole morph cycle: cast for three, attack as a nameless 2/2, then pay
/// the morph cost and become what it really is.
#[test]
fn exalted_angel_comes_down_face_down_and_stands_up_later() {
    let mut game = ready_game();
    let angel = card(10_000, cards::EXALTED_ANGEL, PlayerId::One);
    let angel_id = angel.id;
    game.players[PlayerId::One.index()].hand.push(angel);
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    game.priority = PlayerId::One;

    // Three colourless is not six, so the only cast on offer is the face-down
    // one.
    let casts: Vec<Action> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == angel_id))
        .collect();
    assert_eq!(casts.len(), 1, "only the face-down cast is affordable");
    game.apply(PlayerId::One, casts[0].clone()).unwrap();
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    // A card entering the battlefield is a new object, so the permanent has
    // an identity of its own.
    let angel_id = game.battlefield[0].card.id;
    let permanent = &game.battlefield[0];
    let stats = game.creature_stats(permanent).expect("it is a creature");
    assert_eq!((stats.power, stats.toughness), (2, 2), "a 2/2 body");
    assert!(!game.has_flying(permanent), "and none of the Angel's text");
    assert_eq!(
        game.observe(PlayerId::Two).battlefield[0].characteristics,
        ObjectCharacteristics::card(cards::FACE_DOWN_CREATURE, CardPartId::PRIMARY),
        "the opponent cannot see what it is",
    );

    // Not enough mana for the morph cost yet.
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::TurnFaceUp { .. })),
        "an empty pool cannot pay the morph cost",
    );

    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 2;
    pool.colorless = 2;
    let turn_up = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::TurnFaceUp { permanent } if *permanent == angel_id))
        .expect("the morph cost is payable now");
    game.apply(PlayerId::One, turn_up).unwrap();

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel_id)
        .expect("turning face up is not a zone change");
    let stats = game.creature_stats(permanent).expect("still a creature");
    assert_eq!(
        (stats.power, stats.toughness),
        (4, 5),
        "the Angel it always was",
    );
    assert!(
        game.has_flying(permanent),
        "with its printed abilities back"
    );
    assert_eq!(
        game.observe(PlayerId::Two).battlefield[0].characteristics,
        ObjectCharacteristics::card(cards::EXALTED_ANGEL, CardPartId::PRIMARY),
        "and now the opponent sees it too",
    );
}
