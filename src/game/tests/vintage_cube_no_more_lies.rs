//! No More Lies: a Mana Leak that eats what it catches.

use super::*;

/// Player Two holding a spell with the mana for it, Player One holding the
/// answer, on Player Two's turn.
fn staged(theirs: CardDefinitionId, their_mana: u16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.players[1].graveyard.clear();
    let lies = game
        .build_zone(PlayerId::One, &[cards::NO_MORE_LIES])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let lies_id = lies.id;
    game.players[0].hand.push(lies);
    let spell = game
        .build_zone(PlayerId::Two, &[theirs])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let spell_id = spell.id;
    game.players[1].hand.push(spell);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::Two, color, their_mana);
    }
    (game, lies_id, spell_id)
}

/// Answers every question, taking the option labelled `wanted` where it is
/// offered and the first one otherwise.
fn settle(game: &mut Game, wanted: &str) {
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options: Vec<_> = decision
                .options
                .iter()
                .find(|option| option.label == wanted)
                .map(|option| option.id)
                .into_iter()
                .collect();
            let options = if options.len() < decision.minimum.max(1) {
                decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum.max(1))
                    .collect()
            } else {
                options
            };
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

/// Player Two casts their spell; Player One answers it, and `wanted` decides
/// whether the payment is made.
fn cast_and_answer(game: &mut Game, lies: GameObjectId, spell: GameObjectId, wanted: &str) {
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("they have the mana");
    game.apply(PlayerId::Two, cast)
        .expect("their spell is cast");
    for _ in 0..4 {
        if game.priority == PlayerId::One {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    let answer = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == lies))
        .expect("two mana answers it");
    game.apply(PlayerId::One, answer).expect("it is cast");
    settle(game, wanted);
}

/// Unpaid, the spell is countered -- and exiled rather than binned.
#[test]
fn the_countered_spell_is_exiled() {
    let (mut game, lies, spell) = staged(cards::SERRA_ANGEL, 5);

    cast_and_answer(&mut game, lies, spell, "Decline");

    assert!(game.battlefield.is_empty(), "the creature never resolved");
    assert!(
        game.players[1].graveyard.is_empty(),
        "and nothing of theirs reached a graveyard",
    );
    assert_eq!(
        game.players[1]
            .exile
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL],
    );
}

/// The answer itself is an ordinary spell and goes to its own graveyard.
#[test]
fn no_more_lies_goes_to_its_own_graveyard() {
    let (mut game, lies, spell) = staged(cards::SERRA_ANGEL, 5);

    cast_and_answer(&mut game, lies, spell, "Decline");

    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::NO_MORE_LIES],
    );
}

/// Three mana buys the spell back, and the three are spent.
#[test]
fn paying_three_keeps_the_spell() {
    let (mut game, lies, spell) = staged(cards::SERRA_ANGEL, 5);
    let before = game.players[1].mana_pool.total();

    cast_and_answer(&mut game, lies, spell, "Pay the cost");

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![ObjectKind::Card(cards::SERRA_ANGEL)],
        "the Angel resolved",
    );
    assert!(game.players[1].exile.is_empty(), "and nothing was exiled");
    assert_eq!(
        game.players[1].mana_pool.total(),
        before - 5 - 3,
        "the Angel's five and the ransom's three",
    );
}

/// With only the Angel's own mana available there is nothing to pay with, so
/// no payment is offered at all.
#[test]
fn without_three_to_spare_there_is_no_offer() {
    let (mut game, lies, spell) = staged(cards::LIGHTNING_BOLT, 0);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("one red casts the Bolt and leaves nothing");
    game.apply(PlayerId::Two, cast)
        .expect("their spell is cast");
    for _ in 0..4 {
        if game.priority == PlayerId::One {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    let answer = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == lies))
        .expect("two mana answers it");
    game.apply(PlayerId::One, answer).expect("it is cast");
    let priority = game.priority;
    let _ = game.apply(priority, Action::PassPriority);
    let priority = game.priority;
    let _ = game.apply(priority, Action::PassPriority);

    assert!(
        game.pending_decisions.first().is_none_or(|pending| pending
            .observation
            .options
            .iter()
            .all(|option| option.label != "Pay the cost")),
        "an empty pool has nothing to offer",
    );

    settle(&mut game, "Pay the cost");
    assert_eq!(
        game.players[1]
            .exile
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "so it is countered and exiled",
    );
}
