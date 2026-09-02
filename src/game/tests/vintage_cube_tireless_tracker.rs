//! Tireless Tracker: a land drop is a card, and cashing that card in makes
//! the Tracker bigger.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for index in 0..4 {
        game.players[0]
            .library
            .push(card(104_000 + index, cards::GRIZZLY_BEARS, PlayerId::One));
    }
    let tracker = game
        .put_onto_battlefield(PlayerId::One, cards::TIRELESS_TRACKER)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, tracker)
}

fn clues(game: &Game) -> Vec<GameObjectId> {
    game.battlefield
        .iter()
        .filter(|permanent| game.effective_subtypes(permanent).contains(&"Clue"))
        .map(|permanent| permanent.card.id)
        .collect()
}

fn counters(game: &Game, tracker: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == tracker)
        .expect("it is there")
        .counters(CounterKind::PlusOnePlusOne)
}

/// A land you play is a Clue; somebody else's land is not.
#[test]
fn a_land_you_control_investigates() {
    let (mut game, _) = staged();
    assert!(clues(&game).is_empty());

    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(clues(&game).len(), 1);

    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(clues(&game).len(), 1, "theirs is not yours");
}

/// Cashing in the Clue draws a card and grows the Tracker.
#[test]
fn sacrificing_a_clue_grows_it() {
    let (mut game, tracker) = staged();
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    let clue = clues(&game)[0];
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let cash_in = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == clue))
        .expect("two mana and the Clue itself buys a card");
    game.apply(PlayerId::One, cash_in).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(
        counters(&game, tracker),
        1,
        "the sacrifice is what grows it"
    );
    assert_eq!(game.players[0].hand.len(), 1, "and the Clue drew its card");
    assert_eq!(
        (
            game.power(
                game.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == tracker)
                    .expect("it is there")
            ),
            game.toughness(
                game.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == tracker)
                    .expect("it is there")
            ),
        ),
        (Some(4), Some(3)),
    );
}

/// A Clue that dies some other way is not a sacrifice, and grows nothing.
#[test]
fn a_destroyed_clue_grows_nothing() {
    let (mut game, tracker) = staged();
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    let clue = clues(&game)[0];

    game.move_permanents_to_graveyard(&[clue]);
    drain_pending(&mut game);

    assert!(clues(&game).is_empty());
    assert_eq!(counters(&game, tracker), 0, "it was not sacrificed");
}

/// The opponent sacrificing their own Clue is not you sacrificing one.
#[test]
fn their_sacrifice_is_not_yours() {
    let (mut game, tracker) = staged();
    // Any permanent of theirs will do; what matters is who gives it up.
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    game.sacrifice_permanents(&[theirs]);
    drain_pending(&mut game);

    assert_eq!(counters(&game, tracker), 0);
}

/// "Those abilities trigger whenever you sacrifice a Clue for any reason,
/// not just to activate a Clue's activated ability." Kuldotha Rebirth wants
/// an artifact sacrificed, and a Clue is one: the Tracker grows for a Clue it
/// never got to draw with.
#[test]
fn a_clue_sacrificed_to_something_else_grows_it_too() {
    let (mut game, tracker) = staged();
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    let clue = clues(&game)[0];
    assert_eq!(counters(&game, tracker), 0, "nothing sacrificed yet");

    let rebirth = game
        .build_zone(PlayerId::One, &[cards::KULDOTHA_REBIRTH])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let rebirth_id = rebirth.id;
    game.players[0].hand.push(rebirth);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, sacrifices, .. }
                if *card == rebirth_id && sacrifices.contains(&clue))
        })
        .expect("the Clue is an artifact, and Kuldotha Rebirth eats artifacts");
    game.apply(PlayerId::One, cast).expect("it is cast");
    drain_pending(&mut game);

    assert!(
        clues(&game).is_empty(),
        "the Clue was spent on the Crab rather than on a card",
    );
    assert_eq!(
        counters(&game, tracker),
        1,
        "and the Tracker grew for it all the same",
    );
    assert!(
        game.players[0].hand.is_empty(),
        "with no card drawn for the Clue: that was never what it paid for",
    );
}
