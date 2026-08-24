//! Overlord of the Balemurk: a five-mana 5/5 that most decks would rather
//! cast for two and wait five turns for the body.

use super::*;

/// Player One holding an Overlord with enough mana for either price.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let overlord = game
        .build_zone(PlayerId::One, &[cards::OVERLORD_OF_THE_BALEMURK])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = overlord.id;
    game.players[0].hand.push(overlord);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
    game.priority = PlayerId::One;
    (game, id)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect();
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
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Casts the Overlord, taking the cheaper price when `impending` is set.
fn cast(game: &mut Game, overlord: GameObjectId, impending: bool) {
    let mut casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == overlord))
        .collect::<Vec<_>>();
    assert_eq!(casts.len(), 2, "both prices are on offer");
    // The impending clause is the card's own alternative, so it is the cast
    // that names one; the printed cost names none.
    let wanted = casts
        .iter()
        .position(|action| match action {
            Action::CastSpell { choices, .. } => {
                choices.costs().alternative().is_some() == impending
            }
            _ => false,
        })
        .expect("one cast each way");
    let chosen = casts.remove(wanted);
    game.apply(PlayerId::One, chosen).expect("it is castable");
    settle(game);
}

fn on_battlefield(game: &Game) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::OVERLORD_OF_THE_BALEMURK)
}

fn is_a_creature(game: &Game) -> bool {
    on_battlefield(game).is_some_and(|permanent| {
        game.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Creature))
    })
}

fn time_counters(game: &Game) -> u16 {
    on_battlefield(game).map_or(0, |permanent| {
        permanent.counters(CounterKind::named("time"))
    })
}

/// Runs Player One's end step.
fn end_step(game: &mut Game) {
    game.active_player = PlayerId::One;
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(game);
}

/// The printed price gets a 5/5 with no counters on it.
#[test]
fn hard_cast_it_is_a_creature_at_once() {
    let (mut game, overlord) = staged();
    cast(&mut game, overlord, false);

    assert!(is_a_creature(&game), "five mana buys the body");
    assert_eq!(time_counters(&game), 0);
}

/// The impending price gets five counters and no body.
#[test]
fn impending_enters_with_counters_and_no_body() {
    let (mut game, overlord) = staged();
    cast(&mut game, overlord, true);

    assert_eq!(time_counters(&game), 5);
    assert!(
        !is_a_creature(&game),
        "the enchantment is here; the creature is not",
    );
}

/// Either way the enters trigger fires: it watches the permanent, not the
/// creature.
#[test]
fn the_enters_trigger_fires_even_without_a_body() {
    let (mut game, overlord) = staged();
    let bears = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].graveyard.push(bears);
    let library_before = game.players[0].library.len();

    cast(&mut game, overlord, true);

    assert_eq!(
        game.players[0].library.len(),
        library_before - 4,
        "four cards milled",
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "and the Bears came back",
    );
}

/// A counter comes off each end step, and the body arrives with the last one.
#[test]
fn the_counters_tick_down_to_a_creature() {
    let (mut game, overlord) = staged();
    cast(&mut game, overlord, true);

    for remaining in (1..=4).rev() {
        end_step(&mut game);
        assert_eq!(time_counters(&game), remaining);
        assert!(
            !is_a_creature(&game),
            "{remaining} counters is still no body"
        );
    }

    end_step(&mut game);
    assert_eq!(time_counters(&game), 0);
    assert!(
        is_a_creature(&game),
        "the last one off is the body arriving"
    );
}

/// It cannot buy itself back: the Overlord is an Avatar, which is what the
/// exclusion is for.
#[test]
fn it_cannot_return_another_avatar() {
    let (mut game, overlord) = staged();
    let other = game
        .build_zone(PlayerId::One, &[cards::OVERLORD_OF_THE_BALEMURK])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].graveyard.push(other);

    cast(&mut game, overlord, true);

    assert!(
        !game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::OVERLORD_OF_THE_BALEMURK),
        "an Avatar in the graveyard is not on offer",
    );
}
