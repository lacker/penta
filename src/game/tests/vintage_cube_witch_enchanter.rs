//! Witch Enchanter // Witch-Blessed Meadow: one card that is a Disenchant on
//! a body or the white source the hand was short of, decided as it is played.

use super::*;

/// Player One holding the card, with a land drop still available.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let card = card(94_000, cards::WITCH_ENCHANTER, PlayerId::One);
    let held = card.id;
    game.players[0].hand.push(card);
    game.players[0].lands_played_this_turn = 0;
    game.players[0].life = 20;
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, held)
}

/// Plays the land face, paying or declining the three life.
fn play_land(game: &mut Game, held: GameObjectId, pay: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == held))
        .expect("the back face is playable as a land");
    game.apply(PlayerId::One, action).expect("it is played");

    let payment = game
        .observe(PlayerId::One)
        .decision
        .expect("it offers the three life");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: payment.id,
            options: vec![u32::from(pay)],
        },
    )
    .expect("answering the payment is legal");
    drain_pending(game);
}

fn meadow(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::WITCH_ENCHANTER)
        .expect("it is on the battlefield")
}

/// Both faces are offered from hand: one is a spell, the other a land drop.
#[test]
fn it_offers_a_cast_and_a_land_drop() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);

    let actions = game.legal_actions(PlayerId::One);
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == held)),
        "the front is castable",
    );
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if *card == held)),
        "and the back is a land drop for the same card",
    );
}

/// Pay the three life and the land arrives ready to use.
#[test]
fn paying_three_life_leaves_it_untapped() {
    let (mut game, held) = staged();

    play_land(&mut game, held, true);

    assert_eq!(game.players[0].life, 17);
    assert!(!meadow(&game).tapped, "paid, so it is ready now");
    assert_eq!(
        meadow(&game).presented,
        CardPartId(1),
        "and it is the land face that is on the battlefield",
    );
    assert_eq!(
        game.players[0].lands_played_this_turn, 1,
        "it cost the land drop",
    );
}

/// Decline and it enters tapped instead.
#[test]
fn declining_leaves_it_tapped() {
    let (mut game, held) = staged();

    play_land(&mut game, held, false);

    assert_eq!(game.players[0].life, 20, "nothing was paid");
    assert!(meadow(&game).tapped);
}

/// Untapped, it taps for white.
#[test]
fn the_meadow_taps_for_white() {
    let (mut game, held) = staged();
    play_land(&mut game, held, true);
    let land = meadow(&game).card.id;

    let add_white = Action::ActivateManaAbility {
        source: land,
        ability: mana_ability_for(&game, land, ManaColor::White),
        color: ManaColor::White,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&add_white));
    game.apply(PlayerId::One, add_white).expect("it taps");

    assert_eq!(game.players[0].mana_pool.white, 1);
}

/// The front is a Disenchant with a body attached.
#[test]
fn the_front_destroys_an_opponents_artifact() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::MANIFOLD_KEY)
        .expect("cataloged");
    drain_pending(&mut game);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
        .expect("the front is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == theirs),
        "their artifact is gone",
    );
    let body = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::WITCH_ENCHANTER)
        .expect("and the Warlock stayed");
    assert_eq!(game.power(body), Some(2));
    assert_eq!(game.toughness(body), Some(2));
}

/// "An opponent controls": with one artifact on each side, only theirs can
/// be pointed at.
#[test]
fn it_cannot_point_at_your_own_artifact() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);
    let yours = game
        .put_onto_battlefield(PlayerId::One, cards::MANIFOLD_KEY)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::LIGHTNING_GREAVES)
        .expect("cataloged");
    drain_pending(&mut game);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
        .expect("the front is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == yours),
        "your own artifact was never a legal target",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == theirs),
        "and the only one it could point at is gone",
    );
}
