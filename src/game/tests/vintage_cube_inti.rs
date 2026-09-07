//! Inti, Seneschal of the Sun: two clauses that feed each other, since the
//! discard one asks for is the discard the other is watching for.

use super::*;

/// Inti and a bear attacking, with cards in hand to pitch.
fn staged(hand: usize, library: usize) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for index in 0..hand {
        game.players[0].hand.push(card(
            97_000 + u32::try_from(index).expect("a small hand"),
            cards::MOUNTAIN,
            PlayerId::One,
        ));
    }
    for index in 0..library {
        game.players[0].library.push(card(
            97_100 + u32::try_from(index).expect("a small library"),
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }
    let inti = game
        .put_onto_battlefield(PlayerId::One, cards::INTI_SENESCHAL_OF_THE_SUN)
        .expect("cataloged");
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, inti, bears)
}

/// Attacks with the bear, then answers the "you may discard" offer.
fn attack_and_answer(game: &mut Game, bears: GameObjectId, discard: bool) {
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: bears,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("the bear attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    pass_until_decision(game);
    // The trigger names its target as it goes on the stack; the offer to
    // discard is what its resolution asks.
    let targeting = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks for its target");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: targeting.id,
            options: vec![targeting.options[0].id],
        },
    )
    .expect("naming the attacker is legal");
    pass_until_decision(game);
    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the attack trigger offers the discard");
    if !discard {
        assert!(offer.cancellable);
        game.apply(PlayerId::One, Action::CancelDecision { decision: offer.id })
            .unwrap();
        drain_pending(game);
        return;
    }
    let chosen = vec![offer.options.first().expect("discarding is on offer").id];
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: chosen,
        },
    )
    .expect("answering the offer is legal");
    drain_pending(game);
}

fn counters_on(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map_or(0, |permanent| {
            permanent.counters(CounterKind::PlusOnePlusOne)
        })
}

/// Declining the discard ends the clause.
#[test]
fn declining_the_discard_does_nothing() {
    let (mut game, _inti, bears) = staged(2, 3);

    attack_and_answer(&mut game, bears, false);

    assert_eq!(counters_on(&game, bears), 0);
    assert_eq!(game.players[0].hand.len(), 2, "the hand is intact");
    assert!(game.players[0].exile.is_empty(), "and nothing was exiled");
}

/// Discarding grows the attacker and gives it trample.
#[test]
fn discarding_grows_the_attacker() {
    let (mut game, _inti, bears) = staged(2, 3);

    attack_and_answer(&mut game, bears, true);

    assert_eq!(counters_on(&game, bears), 1);
    let grown = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("it is still attacking");
    assert_eq!(game.power(grown), Some(3));
    assert!(game.permanent_has_executable_keyword(grown, KeywordAbility::Trample));
}

/// The discard he asks for is the discard his other half watches, so one
/// attack both grows a creature and finds a card.
#[test]
fn the_discard_feeds_the_other_half() {
    let (mut game, _inti, bears) = staged(2, 3);
    let library = game.players[0].library.len();

    attack_and_answer(&mut game, bears, true);

    assert_eq!(game.players[0].library.len(), library - 1);
    assert_eq!(game.players[0].exile.len(), 1, "one card, not one per half");
    assert_eq!(game.players[0].graveyard.len(), 1, "the card he pitched");
}

/// A discard of two is one trigger, not two.
#[test]
fn a_discard_of_two_exiles_one_card() {
    let (mut game, _inti, _bears) = staged(3, 4);
    game.step = Step::PrecombatMain;
    let cards = game.players[0]
        .hand
        .iter()
        .take(2)
        .map(|card| card.id)
        .collect::<Vec<_>>();

    game.discard_cards(PlayerId::One, &cards);
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].exile.len(),
        1,
        "\"one or more cards\" is one trigger",
    );
}

/// What he finds is playable, and still costs its mana.
#[test]
fn the_exiled_card_is_playable_for_its_cost() {
    let (mut game, _inti, bears) = staged(2, 3);
    attack_and_answer(&mut game, bears, true);
    let exiled = game.players[0].exile[0].id;
    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if card == exiled)),
        "there is no red mana yet",
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == exiled)),
    );
}

/// "Until your next end step" reaches into your own turn when the discard
/// happened on somebody else's.
#[test]
fn a_discard_on_their_turn_lasts_into_yours() {
    let (mut game, _inti, _bears) = staged(2, 3);
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    let pitched = game.players[0].hand[0].id;

    game.discard_cards(PlayerId::One, &[pitched]);
    drain_pending(&mut game);
    let exiled = game.players[0].exile[0].id;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == exiled)),
        "playable already, on their turn",
    );

    // Your turn arrives; the permission is still there.
    game.active_player = PlayerId::One;
    game.turns_started[PlayerId::One.index()] += 1;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == exiled)),
        "and still playable on yours, which is what the end step names",
    );
}

/// And no further: once that turn is over the permission is gone.
#[test]
fn it_lapses_after_your_next_end_step() {
    let (mut game, _inti, _bears) = staged(2, 3);
    game.step = Step::PrecombatMain;
    let pitched = game.players[0].hand[0].id;
    game.discard_cards(PlayerId::One, &[pitched]);
    drain_pending(&mut game);
    let exiled = game.players[0].exile[0].id;

    game.turns_started[PlayerId::One.index()] += 1;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if card == exiled)),
        "your next end step came and went",
    );
}

/// "You pay all costs and follow all normal timing rules for cards played
/// due to the last ability. For example, if the exiled card is a land card,
/// you may play it only during your main phase while the stack is empty."
#[test]
fn a_land_he_exiles_still_wants_a_land_drop() {
    let (mut game, _inti, bears) = staged(2, 0);
    game.players[0]
        .library
        .push(card(97_200, cards::MOUNTAIN, PlayerId::One));

    attack_and_answer(&mut game, bears, true);
    let exiled = game.players[0].exile[0].id;
    assert_eq!(
        game.players[0].exile[0].definition,
        cards::MOUNTAIN,
        "the land off the top is what he found",
    );

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .all(|action| !matches!(action, Action::PlayLand { card, .. } if card == exiled)),
        "a land is not played in the middle of an attack",
    );

    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    game.players[0].lands_played_this_turn = 1;
    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .all(|action| !matches!(action, Action::PlayLand { card, .. } if card == exiled)),
        "nor once the turn's land drop is spent",
    );

    game.players[0].lands_played_this_turn = 0;
    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if card == exiled)),
        "and with a main phase and a land drop it is playable",
    );
}

/// The deviation this card carries, pinned: the record declares the target
/// as the attack trigger goes on the stack rather than by the printed
/// reflexive trigger, so answering that target counters the whole thing and
/// the card that would have been discarded is still in hand. On the printed
/// card the discard is made first and only the counter is lost.
#[test]
fn an_answered_target_keeps_the_card_in_hand() {
    let (mut game, _inti, bears) = staged(2, 3);
    let held = game.players[0].hand.len();

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: bears,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("the bear attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    pass_until_decision(&mut game);

    let targeting = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks for its target");
    let option = targeting
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == bears))
        .expect("the attacking bear is the thing to name")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: targeting.id,
            options: vec![option],
        },
    )
    .expect("naming it is legal");

    game.destroy_permanent(bears);
    game.check_state_based_actions();
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        held,
        "nothing was discarded, because the offer never came",
    );
    assert!(
        game.players[0].exile.is_empty(),
        "and with no discard there is nothing off the top either",
    );
}
