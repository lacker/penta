//! Titania, Protector of Argoth: a land back on the way in, and five power
//! every time one dies afterwards.

use super::*;

/// Player One holding a Titania with five mana up, `graveyard` behind her
/// and `lands` already on the battlefield.
fn staged(graveyard: &[CardDefinitionId], lands: &[CardDefinitionId]) -> (Game, GameObjectId) {
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
    for definition in lands {
        game.put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::TITANIA_PROTECTOR_OF_ARGOTH])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let titania = card.id;
    game.players[0].hand.push(card);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    (game, titania)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
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

/// Casts her, answering the entry trigger's target with `wanted` when one is
/// offered.
fn cast(game: &mut Game, titania: GameObjectId, wanted: Option<CardDefinitionId>) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == titania))
        .expect("five mana casts her");
    game.apply(PlayerId::One, action).expect("she casts");
    settle(game);
    let Some(wanted) = wanted else {
        return;
    };
    let seat = deciding(game).expect("the trigger asks which land comes back");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, found)| found.card_definition() == Some(wanted))
        })
        .unwrap_or_else(|| panic!("{wanted:?} is a legal target"))
        .id;
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(game);
}

fn elementals(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Elemental"], &[ManaColor::Green], 5, 3),
            )
        })
        .count()
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
}

/// Entering brings a land back from the graveyard.
#[test]
fn she_returns_a_land_from_your_graveyard() {
    let (mut game, titania) = staged(&[cards::MOUNTAIN], &[]);

    cast(&mut game, titania, Some(cards::MOUNTAIN));

    assert!(
        on_battlefield(&game, cards::MOUNTAIN).is_some(),
        "the Mountain came back to the battlefield",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and left the graveyard",
    );
}

/// "A land card": a spell in the graveyard is not one.
#[test]
fn a_nonland_card_in_the_graveyard_is_not_a_target() {
    let (mut game, titania) = staged(&[cards::LIGHTNING_BOLT], &[]);

    cast(&mut game, titania, None);

    assert!(
        deciding(&game).is_none(),
        "there is no land card to point at",
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "and the Bolt stayed where it was",
    );
}

/// A land dying afterwards makes a 5/3.
#[test]
fn a_land_dying_makes_a_five_three() {
    let (mut game, titania) = staged(&[], &[cards::MOUNTAIN]);
    cast(&mut game, titania, None);
    let mountain = on_battlefield(&game, cards::MOUNTAIN)
        .expect("it is here")
        .card
        .id;
    assert_eq!(elementals(&game), 0, "nothing yet");

    game.move_permanents_to_graveyard(&[mountain]);
    settle(&mut game);

    assert_eq!(elementals(&game), 1, "one Elemental");
    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Elemental"], &[ManaColor::Green], 5, 3),
            )
        })
        .expect("it is here");
    assert_eq!(
        (game.power(token), game.toughness(token)),
        (Some(5), Some(3))
    );
    assert_eq!(token.controller, PlayerId::One, "under her controller");
}

/// Each land is its own trigger: two dying makes two.
#[test]
fn two_lands_dying_makes_two() {
    let (mut game, titania) = staged(&[], &[cards::MOUNTAIN, cards::ISLAND]);
    cast(&mut game, titania, None);
    let lands = game
        .battlefield
        .iter()
        .filter(|permanent| {
            permanent.card.definition == cards::MOUNTAIN
                || permanent.card.definition == cards::ISLAND
        })
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();

    game.move_permanents_to_graveyard(&lands);
    // Two triggers at once, so their controller is asked what order they go
    // on the stack in before either resolves.
    for _ in 0..8 {
        settle(&mut game);
        let Some(seat) = deciding(&game) else { break };
        let decision = game.observe(seat).decision.expect("just checked");
        let options = decision
            .options
            .iter()
            .take(decision.minimum)
            .map(|option| option.id)
            .collect();
        game.apply(
            seat,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .expect("the order it offered is a legal answer");
    }

    assert_eq!(elementals(&game), 2, "one for each land");
}

/// "A land you control": theirs is not yours.
#[test]
fn their_land_dying_makes_nothing() {
    let (mut game, titania) = staged(&[], &[]);
    cast(&mut game, titania, None);
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .expect("cataloged");
    drain_pending(&mut game);

    game.move_permanents_to_graveyard(&[theirs]);
    settle(&mut game);

    assert_eq!(elementals(&game), 0, "the clause names your own lands");
}

/// A creature dying is not a land dying.
#[test]
fn a_creature_dying_makes_nothing() {
    let (mut game, titania) = staged(&[], &[]);
    cast(&mut game, titania, None);
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    game.move_permanents_to_graveyard(&[bears]);
    settle(&mut game);

    assert_eq!(elementals(&game), 0, "a Grizzly Bears is not a land");
}

/// She is a 5/3 herself.
#[test]
fn she_is_a_five_three() {
    let (mut game, titania) = staged(&[], &[]);
    cast(&mut game, titania, None);

    let body = on_battlefield(&game, cards::TITANIA_PROTECTOR_OF_ARGOTH).expect("she resolved");
    assert_eq!((game.power(body), game.toughness(body)), (Some(5), Some(3)));
}
