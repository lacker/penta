//! Milling until a land turns up.
//!
//! How deep it goes is whatever the library says rather than a count known
//! before it starts, and the card it stops on goes to the graveyard with
//! everything above it. A library with no land in it empties, which is the
//! whole reason these two are a combo piece rather than a mill spell.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].library.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    game
}

/// Stack the opponent's library so that `definitions` are drawn top-first.
fn stack_library(game: &mut Game, definitions: &[CardDefinitionId]) {
    for (index, definition) in definitions.iter().rev().enumerate() {
        game.players[PlayerId::Two.index()].library.push(card(
            20_000 + u32::try_from(index).expect("small"),
            *definition,
            PlayerId::Two,
        ));
    }
}

fn graveyard(game: &Game) -> Vec<CardDefinitionId> {
    game.players[PlayerId::Two.index()]
        .graveyard
        .iter()
        .map(|card| card.definition)
        .collect()
}

/// Answers the target decision by naming player two, then resolves.
fn aim_at_opponent(game: &mut Game) {
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // "your opponent" rather than "you": the whole point is that it
            // is aimed at somebody else's library.
            let option = decision
                .options
                .iter()
                .find(|option| option.label == "your opponent")
                .or_else(|| decision.options.first())
                .map(|option| option.id)
                .expect("the decision offers something");
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option],
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

/// Two spells and then a Forest: all three go, and the rest of the library
/// stays put.
#[test]
fn the_spy_stops_on_the_first_land_and_buries_it_too() {
    let mut game = ready();
    stack_library(
        &mut game,
        &[
            cards::LIGHTNING_BOLT,
            cards::LIGHTNING_BOLT,
            cards::FOREST,
            cards::LIGHTNING_BOLT,
            cards::ISLAND,
        ],
    );
    game.put_onto_battlefield(PlayerId::One, cards::BALUSTRADE_SPY)
        .expect("cataloged");
    aim_at_opponent(&mut game);

    assert_eq!(
        graveyard(&game),
        vec![cards::LIGHTNING_BOLT, cards::LIGHTNING_BOLT, cards::FOREST],
        "everything down to and including the land",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].library.len(),
        2,
        "and the rest of the library is undisturbed",
    );
}

/// A landless library empties rather than stopping partway.
#[test]
fn a_landless_library_empties() {
    let mut game = ready();
    stack_library(
        &mut game,
        &[
            cards::LIGHTNING_BOLT,
            cards::GRIZZLY_BEARS,
            cards::SERRA_ANGEL,
        ],
    );
    game.put_onto_battlefield(PlayerId::One, cards::BALUSTRADE_SPY)
        .expect("cataloged");
    aim_at_opponent(&mut game);

    assert!(
        game.players[PlayerId::Two.index()].library.is_empty(),
        "nothing matched, so nothing stopped it",
    );
    assert_eq!(graveyard(&game).len(), 3);
}

/// The Informer does the same thing, and eats a creature to do it.
#[test]
fn the_informer_pays_a_creature_for_the_same_effect() {
    let mut game = ready();
    stack_library(&mut game, &[cards::LIGHTNING_BOLT, cards::MOUNTAIN]);
    let informer = creature(10_000, cards::UNDERCITY_INFORMER, PlayerId::One);
    let informer_id = informer.card.id;
    game.battlefield.push(informer);
    let bear = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                targets,
                cost_objects,
                ..
            } => {
                *source == informer_id
                    // It may legally eat itself, so the bear has to be named.
                    && cost_objects.as_slice() == [bear_id]
                    && targets.iter().any(|selection| {
                        selection.targets().contains(&Target::Player(PlayerId::Two))
                    })
            }
            _ => false,
        })
        .expect("the bear can pay and the opponent can be named");
    game.apply(PlayerId::One, action).expect("it is activated");
    aim_at_opponent(&mut game);

    assert_eq!(
        graveyard(&game),
        vec![cards::LIGHTNING_BOLT, cards::MOUNTAIN]
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS),
        "a creature paid for it",
    );
}
