//! Searching a land onto the battlefield tapped.
//!
//! A search could put a permanent onto the battlefield, but always untapped.
//! Both cards here fetch a basic from the deck's own library, so the assertion
//! is about what arrived rather than about a planted card.
//! The flag is set on the prospective permanent before entry replacements
//! run, the same way an as-enters clause would, so nothing observes the land
//! arriving untapped first.

use super::*;

#[test]
fn evolving_wilds_fetches_a_tapped_basic() {
    let mut game = ready_game();
    let wilds = creature(10_000, cards::EVOLVING_WILDS, PlayerId::One);
    let wilds_id = wilds.card.id;
    game.battlefield.push(wilds);
    game.players[PlayerId::One.index()]
        .library
        .push(card(20_000, cards::MOUNTAIN, PlayerId::One));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == wilds_id)
        })
        .expect("the land can be sacrificed to fetch");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let fetched = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .expect("a Mountain arrived");
    assert!(fetched.tapped, "and it arrived tapped");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == wilds_id),
        "the Wilds sacrificed itself to do it"
    );
}

/// The Elk reaches the same code by a different route -- a creature's
/// activated ability rather than a land's -- and its search is tapped too.
#[test]
fn the_elk_fetches_a_tapped_basic_as_well() {
    let mut game = ready_game();
    let elk = creature(10_000, cards::DAWNTREADER_ELK, PlayerId::One);
    let elk_id = elk.card.id;
    game.battlefield.push(elk);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    let basics_before = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .count();

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == elk_id),
        )
        .expect("the Elk can be sacrificed to fetch");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let fetched = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .collect::<Vec<_>>();
    assert_eq!(
        fetched.len(),
        basics_before + 1,
        "one basic land arrived from the library"
    );
    assert!(
        fetched.iter().all(|permanent| permanent.tapped),
        "and it arrived tapped"
    );
}
