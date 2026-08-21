//! Eternal Witness: a 2/1 nobody plays for the body.

use super::*;

/// Player One with `graveyard` in their graveyard and a Witness in hand,
/// with three mana up.
fn staged(graveyard: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    for definition in graveyard {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].graveyard.push(card);
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::ETERNAL_WITNESS])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let witness = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, witness)
}

/// Passes until somebody is asked something, or the stack is quiet.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.observe(PlayerId::One).decision.is_some()
            || game.observe(PlayerId::Two).decision.is_some()
        {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn cast(game: &mut Game, witness: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == witness))
        .expect("three mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
}

/// Answers the pending decision by picking the option matching `label`, or
/// by card definition when `wanted` is given.
fn answer(game: &mut Game, wanted: Option<CardDefinitionId>, label: Option<&str>) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("something is being asked");
    let option = decision
        .options
        .iter()
        .find(|option| match (wanted, label) {
            (Some(definition), _) => option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(definition)
            }),
            (None, Some(label)) => option.label == label,
            (None, None) => true,
        })
        .unwrap_or_else(|| {
            panic!(
                "the answer is offered: {:?}",
                decision
                    .options
                    .iter()
                    .map(|option| (option.label.clone(), option.card))
                    .collect::<Vec<_>>()
            )
        })
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(game);
}

/// The card comes back, and the Witness stays.
#[test]
fn it_returns_the_card_it_pointed_at() {
    let (mut game, witness) = staged(&[cards::LIGHTNING_BOLT]);

    cast(&mut game, witness);
    answer(&mut game, Some(cards::LIGHTNING_BOLT), None);
    answer(&mut game, None, Some("Do it"));

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the Bolt is back in hand",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and out of the graveyard",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ETERNAL_WITNESS),
        "with the Witness still standing",
    );
}

/// "You may" is a real choice: declining leaves the card where it lies.
#[test]
fn declining_leaves_the_card_in_the_graveyard() {
    let (mut game, witness) = staged(&[cards::LIGHTNING_BOLT]);

    cast(&mut game, witness);
    answer(&mut game, Some(cards::LIGHTNING_BOLT), None);
    answer(&mut game, None, Some("Decline"));

    assert!(
        game.players[0].hand.is_empty(),
        "nothing came back: {:?}",
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "and the Bolt is where it was",
    );
}

/// Any card, not just a spell: a land comes back the same way.
#[test]
fn it_takes_a_land_as_readily_as_a_spell() {
    let (mut game, witness) = staged(&[cards::MOUNTAIN]);

    cast(&mut game, witness);
    answer(&mut game, Some(cards::MOUNTAIN), None);
    answer(&mut game, None, Some("Do it"));

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
        "the land is in hand",
    );
}

/// Your graveyard, not theirs.
#[test]
fn it_cannot_reach_across_the_table() {
    let (mut game, witness) = staged(&[cards::LIGHTNING_BOLT]);
    let theirs = game
        .build_zone(PlayerId::Two, &[cards::SERRA_ANGEL])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[1].graveyard.push(theirs);

    cast(&mut game, witness);

    let offered: Vec<CardDefinitionId> = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks what to take")
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect();
    assert!(
        offered.contains(&cards::LIGHTNING_BOLT),
        "your own Bolt is offered: {offered:?}",
    );
    assert!(
        !offered.contains(&cards::SERRA_ANGEL),
        "their Angel is not: {offered:?}",
    );
}

/// With nothing to point at, the trigger has no legal target and asks
/// nothing at all.
#[test]
fn an_empty_graveyard_asks_nothing() {
    let (mut game, witness) = staged(&[]);

    cast(&mut game, witness);

    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "no target, no question",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ETERNAL_WITNESS),
        "and the Witness arrived all the same",
    );
}
