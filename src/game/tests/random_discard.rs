//! Discarding at random as an activation cost.
//!
//! Unlike the discard the payer chooses, nobody decides which cards go, so
//! paying it needs no decision at all -- the cards leave as the cost is paid.
//! The picks come off the seeded generator, so the same seed spends the same
//! cards.

use super::*;

/// A Coral Helm and a hand of `hand_size` cards, ready to activate.
fn helm_with_hand(seed: u64, hand_size: u32) -> (Game, GameObjectId) {
    let mut game = ready_game_with_seed(seed);
    game.turns_started[PlayerId::One.index()] = 5;
    let helm = creature(10_000, cards::CORAL_HELM, PlayerId::One);
    let helm_id = helm.card.id;
    game.battlefield.push(helm);
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    for index in 0..hand_size {
        game.players[PlayerId::One.index()].hand.push(card(
            20_000 + index,
            cards::MOUNTAIN,
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    (game, helm_id)
}

fn activation(game: &Game, source: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source))
}

#[test]
fn paying_it_takes_a_card_out_of_hand_with_no_choice_asked() {
    let (mut game, helm) = helm_with_hand(7, 3);

    let action = activation(&game, helm).expect("three cards is enough to pay");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");

    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        2,
        "one card gone"
    );
    assert!(
        game.pending_decisions.is_empty(),
        "and nobody was asked which"
    );
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 1);
}

/// An empty hand cannot pay, so the ability is not offered at all.
#[test]
fn an_empty_hand_cannot_pay_it() {
    let (game, helm) = helm_with_hand(7, 0);

    assert!(activation(&game, helm).is_none());
}

/// The pick is seeded: different seeds reach different cards, and the same
/// seed reaches the same one. A cost that always took the first card would
/// pass a test that only counted the hand.
#[test]
fn the_card_taken_follows_the_seed() {
    // Which hand card went, by the id it had in hand: changing zones gives
    // the card a fresh object, so the graveyard cannot be asked directly.
    let discarded = |seed: u64| {
        let (mut game, helm) = helm_with_hand(seed, 5);
        let before: std::collections::BTreeSet<_> = game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.id)
            .collect();
        let action = activation(&game, helm).expect("five cards is enough");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        let after: std::collections::BTreeSet<_> = game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.id)
            .collect();
        *before
            .difference(&after)
            .next()
            .expect("exactly one card left hand")
    };

    assert_eq!(
        discarded(11),
        discarded(11),
        "the same seed spends the same"
    );
    let picks: std::collections::BTreeSet<_> = (0..24).map(discarded).collect();
    assert!(
        picks.len() > 1,
        "across seeds it does not always reach for the same card: {picks:?}"
    );
}

#[test]
fn the_cylix_regenerates_and_pays_the_same_way() {
    let mut game = ready_game_with_seed(3);
    game.turns_started[PlayerId::One.index()] = 5;
    let cylix = creature(10_000, cards::DRACONIAN_CYLIX, PlayerId::One);
    let cylix_id = cylix.card.id;
    game.battlefield.push(cylix);
    let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    game.players[PlayerId::One.index()]
        .hand
        .push(card(20_000, cards::MOUNTAIN, PlayerId::One));
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = activation(&game, cylix_id).expect("a creature to regenerate");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert!(game.players[PlayerId::One.index()].hand.is_empty());
    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("still there");
    assert!(troll.regeneration_shields > 0, "a shield is waiting on it");
}

/// Rag Man takes a creature card and nothing else, which is what the filtered
/// selection is for: a hand of lands loses nothing at all.
#[test]
fn the_rag_man_reaches_past_the_lands_for_a_creature() {
    let discard_from = |seed: u64, creatures: u32, lands: u32| {
        let mut game = ready_game_with_seed(seed);
        game.turns_started[PlayerId::One.index()] = 5;
        game.active_player = PlayerId::One;
        let rag_man = creature(10_000, cards::RAG_MAN, PlayerId::One);
        let rag_man_id = rag_man.card.id;
        game.battlefield.push(rag_man);
        game.players[PlayerId::One.index()].mana_pool.black = 3;
        game.players[PlayerId::Two.index()].hand.clear();
        for index in 0..creatures {
            game.players[PlayerId::Two.index()].hand.push(card(
                20_000 + index,
                cards::SAVANNAH_LIONS,
                PlayerId::Two,
            ));
        }
        for index in 0..lands {
            game.players[PlayerId::Two.index()].hand.push(card(
                21_000 + index,
                cards::MOUNTAIN,
                PlayerId::Two,
            ));
        }

        let action = activation(&game, rag_man_id).expect("an opponent to point at");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        drain_pending(&mut game);
        game.players[PlayerId::Two.index()]
            .hand
            .iter()
            .filter(|card| card.definition == cards::SAVANNAH_LIONS)
            .count()
    };

    assert_eq!(discard_from(5, 2, 3), 1, "one of the two creatures went");
    assert_eq!(
        discard_from(5, 0, 3),
        0,
        "a hand with no creature card in it loses nothing"
    );
}
