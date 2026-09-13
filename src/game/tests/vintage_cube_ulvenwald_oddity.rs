//! Ulvenwald Oddity: a hasty trampling body and a mana sink that turns it
//! into the reason every other creature is bigger.

use super::*;

/// The Oddity on the battlefield with a bear beside it, and mana to spare.
fn staged(mana: u16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let oddity = game
        .put_onto_battlefield(PlayerId::One, cards::ULVENWALD_ODDITY)
        .expect("cataloged");
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, oddity, bears)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

fn transform(game: &mut Game, oddity: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(
        |action| matches!(action, Action::ActivateAbility { source, .. } if *source == oddity),
    )
}

/// The front face is a 4/4 with both keywords.
#[test]
fn the_front_face_tramples_and_hastes() {
    let (game, oddity, _bears) = staged(0);
    let front = permanent(&game, oddity);

    assert_eq!(game.power(front), Some(4));
    assert_eq!(game.toughness(front), Some(4));
    assert!(game.permanent_has_executable_keyword(front, KeywordAbility::Trample));
    assert!(game.permanent_has_executable_keyword(front, KeywordAbility::Haste));
}

/// Seven mana turns it over into an 8/8.
#[test]
fn seven_mana_transforms_it() {
    let (mut game, oddity, _bears) = staged(5);
    let action = transform(&mut game, oddity).expect("seven mana pays for it");

    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    let back = permanent(&game, oddity);
    assert_eq!(game.power(back), Some(8));
    assert_eq!(game.toughness(back), Some(8));
    assert_eq!(
        game.effective_permanent_name(back).as_deref(),
        Some("Ulvenwald Behemoth"),
    );
}

/// Six mana is not seven.
#[test]
fn six_mana_is_not_enough() {
    let (mut game, oddity, _bears) = staged(4);

    assert!(transform(&mut game, oddity).is_none());
}

/// The back face makes every other creature bigger and hasty.
#[test]
fn the_back_face_grows_the_rest_of_the_board() {
    let (mut game, oddity, bears) = staged(5);
    let before = permanent(&game, bears);
    assert_eq!(game.power(before), Some(2));
    assert!(!game.permanent_has_executable_keyword(before, KeywordAbility::Trample));

    let action = transform(&mut game, oddity).expect("seven mana pays for it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    let grown = permanent(&game, bears);
    assert_eq!(game.power(grown), Some(3));
    assert_eq!(game.toughness(grown), Some(3));
    assert!(game.permanent_has_executable_keyword(grown, KeywordAbility::Trample));
    assert!(game.permanent_has_executable_keyword(grown, KeywordAbility::Haste));
}

/// "Other creatures": the Behemoth is not among them, so it stays 8/8.
#[test]
fn the_behemoth_does_not_grow_itself() {
    let (mut game, oddity, _bears) = staged(5);
    let action = transform(&mut game, oddity).expect("seven mana pays for it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(game.power(permanent(&game, oddity)), Some(8));
}

/// "You control": their creatures are left alone.
#[test]
fn it_does_not_grow_their_board() {
    let (mut game, oddity, _bears) = staged(5);
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    let action = transform(&mut game, oddity).expect("seven mana pays for it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(game.power(permanent(&game, theirs)), Some(2));
}

/// The card is a Beast on the front and a Beast Horror on the back.
#[test]
fn transforming_changes_the_subtypes() {
    let (mut game, oddity, _bears) = staged(5);
    assert!(
        !game
            .effective_subtypes(permanent(&game, oddity))
            .contains(crate::card::Subtype::named("Horror"))
    );

    let action = transform(&mut game, oddity).expect("seven mana pays for it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    let subtypes = game.effective_subtypes(permanent(&game, oddity));
    assert!(subtypes.contains(crate::card::Subtype::named("Beast")));
    assert!(subtypes.contains(crate::card::Subtype::named("Horror")));
}

/// The anthem is a static effect rather than something the transform did
/// once: a creature that arrives afterwards is bigger and hasty the moment
/// it lands, and shrinks again if the Behemoth leaves.
#[test]
fn a_creature_that_arrives_later_is_grown_too() {
    let (mut game, oddity, _bears) = staged(5);
    let action = transform(&mut game, oddity).expect("seven mana pays for it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    let latecomer = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);

    let lions = permanent(&game, latecomer);
    assert_eq!(game.power(lions), Some(3), "a 2/1 arrives as a 3/2");
    assert_eq!(game.toughness(lions), Some(2));
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::Haste));
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::Trample));

    game.move_permanents_to_graveyard(&[oddity]);
    game.check_state_based_actions();

    let lions = permanent(&game, latecomer);
    assert_eq!(
        game.power(lions),
        Some(2),
        "and is a 2/1 again once it is gone"
    );
    assert!(!game.permanent_has_executable_keyword(lions, KeywordAbility::Trample));
}
