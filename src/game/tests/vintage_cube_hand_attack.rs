//! Creatures that reach into an opponent's hand.
//!
//! Both of these take a card and hold it rather than destroying it, so what
//! each test has to establish twice over is which cards were eligible and
//! that the held card comes back.

use super::*;

/// The Freebooter takes the answer rather than the threat: a creature card is
/// never eligible, and neither is a land. And "until this creature leaves the
/// battlefield" is one printed ability, so the return rides on the same
/// resolution rather than on a second clause.
#[test]
fn the_freebooter_takes_a_noncreature_nonland_card_and_gives_it_back() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].hand.clear();
    for (instance, definition) in [
        (71_000, cards::LIGHTNING_BOLT),
        (71_001, cards::ANCESTRAL_RECALL),
        (71_002, cards::SERRA_ANGEL),
        (71_003, cards::FOREST),
    ] {
        game.players[PlayerId::Two.index()]
            .hand
            .push(card(instance, definition, PlayerId::Two));
    }

    let freebooter = game
        .put_onto_battlefield(PlayerId::One, cards::KITESAIL_FREEBOOTER)
        .expect("cataloged");

    let mut offered = Vec::new();
    for _ in 0..8 {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            let cards = decision
                .options
                .iter()
                .filter_map(|option| {
                    option
                        .card
                        .and_then(|(_, characteristics)| characteristics.card_definition())
                })
                .collect::<Vec<_>>();
            if !cards.is_empty() {
                offered = cards;
            }
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[0].id],
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        assert!(
            game.apply(player, Action::PassPriority).is_ok(),
            "the enters trigger is waiting",
        );
    }
    offered.sort_unstable();
    let mut expected = vec![cards::LIGHTNING_BOLT, cards::ANCESTRAL_RECALL];
    expected.sort_unstable();
    assert_eq!(
        offered, expected,
        "the Angel is a creature and the Forest is a land",
    );

    assert_eq!(game.players[PlayerId::Two.index()].exile.len(), 1);
    let held = game.players[PlayerId::Two.index()].exile[0].definition;

    game.move_permanents_to_graveyard(&[freebooter]);
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::Two.index()]
            .hand
            .iter()
            .any(|card| card.definition == held),
        "the card comes back when the body goes",
    );
    assert!(game.players[PlayerId::Two.index()].exile.is_empty());
}

/// The Squadron grows every creature token that arrives, whoever made it and
/// whatever made it -- but only tokens, and only yours.
#[test]
fn the_squadron_puts_a_counter_on_every_creature_token_you_control() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::SECURITRON_SQUADRON)
        .expect("cataloged");
    drain_pending(&mut game);

    let size_of = |game: &Game, token| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| is_token_with(permanent, token))
            .expect("the token arrived");
        (game.power(permanent), game.toughness(permanent))
    };

    game.create_token(
        PlayerId::One,
        crate::card::TokenCharacteristics::creature(&["Beast"], &[ManaColor::Green], 3, 3),
    );
    drain_pending(&mut game);
    assert_eq!(
        size_of(
            &game,
            crate::card::TokenCharacteristics::creature(&["Beast"], &[ManaColor::Green], 3, 3)
        ),
        (Some(4), Some(4)),
        "a 3/3 token arrives and is grown",
    );

    // A Food token is a token but not a creature.
    game.create_token(PlayerId::One, tokens::food());
    drain_pending(&mut game);
    let food = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::food()))
        .expect("the Food arrived");
    assert_eq!(food.counters(CounterKind::PlusOnePlusOne), 0);

    // An opponent's token is not one you control.
    game.create_token(
        PlayerId::Two,
        token_with_vigilance(crate::card::TokenCharacteristics::creature(
            &["Knight"],
            &[ManaColor::White],
            2,
            2,
        )),
    );
    drain_pending(&mut game);
    assert_eq!(
        size_of(
            &game,
            token_with_vigilance(crate::card::TokenCharacteristics::creature(
                &["Knight"],
                &[ManaColor::White],
                2,
                2
            ))
        ),
        (Some(2), Some(2)),
        "and the other player's token is untouched",
    );
}

/// "If Kitesail Freebooter leaves the battlefield before its enters-the-
/// battlefield ability resolves, the opponent will reveal their hand, but no
/// card will be exiled." The Bat's clause is the Freebooter's clause, and it
/// reads the same way when the body is gone.
#[test]
fn a_freebooter_answered_in_response_exiles_nothing() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].hand.clear();
    game.players[PlayerId::Two.index()].hand.push(card(
        71_200,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));

    let freebooter = game
        .put_onto_battlefield(PlayerId::One, cards::KITESAIL_FREEBOOTER)
        .expect("cataloged");
    for _ in 0..8 {
        if !game.stack.is_empty() {
            break;
        }
        if let Some(decision) = game.observe(PlayerId::One).decision {
            let chosen = decision
                .options
                .iter()
                .take(decision.minimum.max(1).min(decision.maximum))
                .map(|option| option.id)
                .collect::<Vec<_>>();
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: chosen,
                },
            )
            .expect("the trigger names its opponent");
            continue;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the trigger goes on the stack");
    }
    game.move_permanents_to_graveyard(&[freebooter]);
    game.check_state_based_actions();
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::Two.index()].exile.is_empty(),
        "nothing is held by a Freebooter that is not there",
    );
    assert!(
        game.players[PlayerId::Two.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the Bolt stayed in their hand",
    );
}

/// "You choose a noncreature, nonland card from it." Not "you may": with two
/// eligible cards in their hand -- one is taken without asking, there being
/// nothing to decide -- the choice belongs to the Freebooter's controller,
/// and taking nothing is not one of the answers.
#[test]
fn the_freebooter_must_take_one_and_its_controller_picks() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].hand.clear();
    for (instance, definition) in [
        (71_300, cards::LIGHTNING_BOLT),
        (71_301, cards::COUNTERSPELL),
    ] {
        game.players[PlayerId::Two.index()]
            .hand
            .push(card(instance, definition, PlayerId::Two));
    }

    game.put_onto_battlefield(PlayerId::One, cards::KITESAIL_FREEBOOTER)
        .expect("cataloged");
    let mut choice = None;
    for _ in 0..8 {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            if decision.options.iter().any(|option| {
                option
                    .card
                    .is_some_and(|(_, card)| card.card_definition() == Some(cards::LIGHTNING_BOLT))
            }) {
                choice = Some(decision);
                break;
            }
            let taken = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect();
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: taken,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let choice = choice.expect("the Bolt is what the trigger offers");
    assert_eq!(
        choice.player,
        PlayerId::One,
        "the Freebooter's controller does the choosing, not the player losing the card",
    );
    assert_eq!(choice.minimum, 1, "and one is the fewest that may be taken");
    assert!(
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: choice.id,
                options: Vec::new(),
            },
        )
        .is_err(),
        "so declining is not an answer the way the Bat's is",
    );
}

/// A hand with nothing eligible in it leaves the trigger nothing to take:
/// the Freebooter still arrives and the hand is untouched.
#[test]
fn a_hand_of_creatures_and_lands_gives_the_freebooter_nothing() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].hand.clear();
    for (instance, definition) in [(71_400, cards::GRIZZLY_BEARS), (71_401, cards::FOREST)] {
        game.players[PlayerId::Two.index()]
            .hand
            .push(card(instance, definition, PlayerId::Two));
    }

    game.put_onto_battlefield(PlayerId::One, cards::KITESAIL_FREEBOOTER)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].hand.len(),
        2,
        "a creature and a land are not what it takes",
    );
    assert!(game.players[PlayerId::Two.index()].exile.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::KITESAIL_FREEBOOTER),
        "and the body is on the battlefield either way",
    );
}
