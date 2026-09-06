//! Landcycling: the same discard as ordinary cycling, but what it buys is a
//! search for one named land type rather than a card off the top. The
//! search is what needs covering -- a wrong predicate fetches the wrong
//! land, and an empty library fetches nothing while still eating the card.

use super::*;

/// Twisted Abomination in hand, with `library` as the whole library.
fn staged(library: &[CardDefinitionId], mana: u16) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    game.players[0].graveyard.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for (index, definition) in library.iter().enumerate() {
        let filler = card(
            85_000 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        game.players[0].library.push(filler);
    }
    let held = card(85_100, cards::TWISTED_ABOMINATION, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    (game, held_id)
}

fn cycling_of(game: &Game, held: CardInstanceId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == held))
}

fn resolve(game: &mut Game) {
    for _ in 0..12 {
        drain_pending(game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
}

fn hand_holds(game: &Game, definition: CardDefinitionId) -> bool {
    game.players[0]
        .hand
        .iter()
        .any(|card| card.definition == ObjectKind::Card(definition))
}

#[test]
fn one_mana_does_not_pay_for_it() {
    let (game, abomination) = staged(&[cards::SWAMP], 1);
    assert!(
        cycling_of(&game, abomination).is_none(),
        "swampcycling costs {{2}}"
    );
}

#[test]
fn it_fetches_a_swamp_and_leaves_the_forest() {
    let (mut game, abomination) = staged(&[cards::FOREST, cards::SWAMP], 2);
    let cycle = cycling_of(&game, abomination).expect("two mana pays for it");
    game.apply(PlayerId::One, cycle)
        .expect("the cost is payable");
    resolve(&mut game);

    assert!(hand_holds(&game, cards::SWAMP), "the Swamp came to hand");
    assert!(
        !hand_holds(&game, cards::FOREST),
        "and the Forest stayed in the library"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::TWISTED_ABOMINATION)),
        "the Abomination was discarded as the cost"
    );
}

#[test]
fn a_library_with_no_swamp_finds_nothing() {
    let (mut game, abomination) = staged(&[cards::FOREST], 2);
    let cycle = cycling_of(&game, abomination).expect("two mana pays for it");
    game.apply(PlayerId::One, cycle)
        .expect("the cost is payable");
    resolve(&mut game);

    assert!(
        game.players[0].hand.is_empty(),
        "failing to find is allowed, and nothing came back"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "but the card was spent either way"
    );
}
