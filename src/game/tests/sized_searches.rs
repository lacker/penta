//! Searches whose size and arrival state come from the card.
//!
//! A library search took a fixed maximum, so "up to X" could not be said at
//! all; the ceiling is now a value read as the spell resolves. The tapped
//! arrival was available all along, which two of these three audit lines had
//! not caught up with.

use super::*;

/// A board with `lands` Forests under player one and `library` Mountains in
/// their library, plus the spell in hand and mana to cast anything here.
fn board(spell: CardDefinitionId, lands: usize, library: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::PrecombatMain;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    for index in 0..lands {
        game.battlefield.push(creature(
            10_000 + u32::try_from(index).expect("small"),
            cards::FOREST,
            PlayerId::One,
        ));
    }
    for index in 0..library {
        game.players[PlayerId::One.index()].library.push(card(
            20_000 + u32::try_from(index).expect("small"),
            cards::MOUNTAIN,
            PlayerId::One,
        ));
    }
    let card = card(30_000, spell, PlayerId::One);
    let card_id = card.id;
    game.players[PlayerId::One.index()].hand.push(card);
    game.players[PlayerId::One.index()].mana_pool.black = 2;
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 12;
    (game, card_id)
}

/// Answers every pending decision by taking as much as it allows, which
/// `drain_pending` deliberately does not do -- it takes the minimum.
fn take_everything(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.maximum)
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Casts the spell, taking every option the search offers.
fn cast(game: &mut Game, spell: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the spell is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    take_everything(game);
}

/// How many cards the pending search will let the player take.
fn offered_maximum(game: &Game) -> usize {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.maximum)
        .expect("a search is waiting")
}

/// Four lands out means up to four basics, not the fixed one a constant
/// maximum could have said.
#[test]
fn boundless_realms_is_sized_by_the_lands_you_control() {
    let (mut game, spell) = board(cards::BOUNDLESS_REALMS, 4, 8);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the spell is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert_eq!(offered_maximum(&game), 4, "one per land you control");
}

/// Two lands, two basics -- and both arrive tapped.
#[test]
fn boundless_realms_puts_them_onto_the_battlefield_tapped() {
    let (mut game, spell) = board(cards::BOUNDLESS_REALMS, 2, 8);
    cast(&mut game, spell);

    let fetched = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .collect::<Vec<_>>();
    assert_eq!(fetched.len(), 2, "one per land you controlled");
    assert!(
        fetched.iter().all(|permanent| permanent.tapped),
        "and every one of them tapped",
    );
}

/// X is the number chosen for the spell, not anything on the board.
#[test]
fn diabolic_revelation_is_sized_by_the_chosen_x() {
    let (mut game, spell) = board(cards::DIABOLIC_REVELATION, 1, 8);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == spell && choices.x() == 3)
        })
        .expect("three can be chosen for X");
    game.apply(PlayerId::One, action).expect("it is cast");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert_eq!(offered_maximum(&game), 3, "up to the X that was paid");
}

/// The land it fetches arrives tapped, which was the whole reason its audit
/// line stood.
#[test]
fn frenzied_tilling_fetches_a_tapped_basic() {
    let (mut game, spell) = board(cards::FRENZIED_TILLING, 1, 4);
    let island = creature(11_000, cards::ISLAND, PlayerId::Two);
    let island_id = island.card.id;
    game.battlefield.push(island);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell
                    && choices.targets().iter().any(|selection| {
                        selection.targets().contains(&Target::Permanent(island_id))
                    })
            }
            _ => false,
        })
        .expect("the opponent's land can be named");
    game.apply(PlayerId::One, action).expect("it is cast");
    take_everything(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ISLAND),
        "the targeted land was destroyed",
    );
    let fetched = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .expect("a basic was found");
    assert!(fetched.tapped, "and it arrived tapped");
}
