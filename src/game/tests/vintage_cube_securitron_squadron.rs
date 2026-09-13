//! Securitron Squadron: a squad you pay for as many times as you like, and
//! a counter on every token that joins it.

use super::*;

/// The Squadron in hand with `mana` colourless beside a white.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let card = game
        .build_zone(PlayerId::One, &[cards::SECURITRON_SQUADRON])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, id)
}

fn settle(game: &mut Game) {
    for _ in 0..200 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Every distinct way of casting the Squadron.
fn casts(game: &Game, card: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card: cast, .. } if *cast == card))
        .collect()
}

fn squadrons(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Robot"))
        })
        .collect()
}

/// Two mana casts it and nothing else happens: squad paid zero times makes
/// no copies.
#[test]
fn paying_squad_no_times_makes_no_copies() {
    let (mut game, card) = staged(1);

    let cast = casts(&game, card)
        .into_iter()
        .next()
        .expect("{1}{W} casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);

    assert_eq!(squadrons(&game).len(), 1, "just the one that was cast");
}

/// Squad {3} paid once buys one token copy. Two counters land on it: one
/// from the Squadron that made it, and one from the copy's own trigger,
/// which sees itself arrive.
#[test]
fn paying_squad_once_makes_one_copy_with_a_counter() {
    let (mut game, card) = staged(4);
    let offers = casts(&game, card);
    assert_eq!(
        offers.len(),
        2,
        "five mana pays for the spell, or for the spell and one squad",
    );

    // The squad payment is the more expensive of the two offers.
    let squadded = offers
        .into_iter()
        .max_by_key(|action| match action {
            Action::CastSpell { choices, .. } => choices.costs().additional().len(),
            _ => 0,
        })
        .expect("one of them pays squad");
    game.apply(PlayerId::One, squadded).expect("it is cast");
    settle(&mut game);

    let made = squadrons(&game);
    assert_eq!(made.len(), 2, "the Squadron and one copy of it");
    let token = made
        .iter()
        .find(|permanent| !matches!(permanent.card.definition, ObjectKind::Card(_)))
        .expect("one of them is a token");
    assert_eq!(
        token.counters(CounterKind::PlusOnePlusOne),
        2,
        "the original's trigger and the token's own",
    );
    assert_eq!(
        (game.power(token), game.toughness(token)),
        (Some(4), Some(4)),
        "a 2/2 copy with two counters on it",
    );
}

/// Paid twice, it makes two -- and they see each other arrive, so each ends
/// with three rather than the two and three sequential arrivals would give.
#[test]
fn paying_squad_twice_makes_two_copies() {
    let (mut game, card) = staged(7);

    let squadded = casts(&game, card)
        .into_iter()
        .max_by_key(|action| match action {
            Action::CastSpell { choices, .. } => choices.costs().additional().len(),
            _ => 0,
        })
        .expect("eight mana pays for two squads");
    game.apply(PlayerId::One, squadded).expect("it is cast");
    settle(&mut game);

    let made = squadrons(&game);
    assert_eq!(made.len(), 3, "the Squadron and two copies");
    for token in made
        .iter()
        .filter(|permanent| !matches!(permanent.card.definition, ObjectKind::Card(_)))
    {
        assert_eq!(
            token.counters(CounterKind::PlusOnePlusOne),
            3,
            "both tokens and the original each gave it one",
        );
    }
}

/// The worked example the card's own ruling gives: squad paid three times
/// makes three tokens that enter at the same time, so each is seen arriving
/// by all three of them and by the Squadron that made them -- four counters
/// apiece.
#[test]
fn three_squads_leave_four_counters_on_each_token() {
    let (mut game, card) = staged(10);

    let squadded = casts(&game, card)
        .into_iter()
        .max_by_key(|action| match action {
            Action::CastSpell { choices, .. } => choices.costs().additional().len(),
            _ => 0,
        })
        .expect("eleven mana pays for three squads");
    game.apply(PlayerId::One, squadded).expect("it is cast");
    settle(&mut game);

    let made = squadrons(&game);
    assert_eq!(made.len(), 4, "the Squadron and three copies");
    let tokens = made
        .iter()
        .filter(|permanent| !matches!(permanent.card.definition, ObjectKind::Card(_)))
        .collect::<Vec<_>>();
    assert_eq!(tokens.len(), 3);
    for token in tokens {
        assert_eq!(
            token.counters(CounterKind::PlusOnePlusOne),
            4,
            "one from each of the three tokens, and one from the original",
        );
    }
    let original = made
        .iter()
        .find(|permanent| matches!(permanent.card.definition, ObjectKind::Card(_)))
        .expect("the one that was cast");
    assert_eq!(
        original.counters(CounterKind::PlusOnePlusOne),
        0,
        "the counters go on the tokens, not on what made them",
    );
}

/// "The tokens created by the squad ability aren't 'cast', so any abilities
/// that trigger when a spell is cast won't trigger for the copies." A
/// Forensic Gadgeteer watching artifact spells pays one Clue for the
/// Squadron itself and nothing at all for the two artifact creatures that
/// arrive behind it.
#[test]
fn the_squad_copies_are_not_cast_and_pay_no_cast_trigger() {
    let (mut game, card) = staged(7);
    game.put_onto_battlefield(PlayerId::One, cards::FORENSIC_GADGETEER)
        .expect("cataloged");
    drain_pending(&mut game);
    settle(&mut game);
    game.priority = PlayerId::One;

    let squadded = casts(&game, card)
        .into_iter()
        .max_by_key(|action| match action {
            Action::CastSpell { choices, .. } => choices.costs().additional().len(),
            _ => 0,
        })
        .expect("eight mana pays for two squads");
    game.apply(PlayerId::One, squadded).expect("it is cast");
    settle(&mut game);

    assert_eq!(
        squadrons(&game).len(),
        3,
        "the Squadron and the two copies arrived",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(permanent, tokens::clue()))
            .count(),
        1,
        "one Clue for the one artifact spell that was cast",
    );
}
