//! Minsc & Boo, Timeless Heroes: the hamster is the ammunition, and what
//! was thrown is read after it has already left.

use super::*;

/// Minsc on the battlefield with three loyalty and a library to draw from.
/// The enters trigger has already been answered, so Boo is beside him.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let minsc = game
        .put_onto_battlefield(PlayerId::One, cards::MINSC_BOO_TIMELESS_HEROES)
        .expect("cataloged");
    settle(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[1].life = 20;
    (game, minsc)
}

/// Answers every open decision, preferring `wanted` where it is offered and
/// never declining an offer.
fn settle_wanting(game: &mut Game, wanted: &[GameObjectId]) {
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .find(|option| {
                    option
                        .card
                        .as_ref()
                        .is_some_and(|(object, _)| wanted.contains(object))
                })
                // Otherwise the opponent, which is where the reflexive
                // trigger is aimed: an "any target" offer lists this player
                // first, and damage to anything else would not show up in
                // the life total the tests read.
                .or_else(|| {
                    decision
                        .options
                        .iter()
                        .find(|option| option.label == "your opponent")
                })
                .or_else(|| {
                    decision
                        .options
                        .iter()
                        .find(|option| option.label != "Decline")
                })
                .map(|option| vec![option.id])
                .unwrap_or_default();
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
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    drain_pending(game);
    game.check_state_based_actions();
}

fn settle(game: &mut Game) {
    settle_wanting(game, &[]);
}

fn boo(game: &Game) -> Option<GameObjectId> {
    game.battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Hamster"))
        })
        .map(|permanent| permanent.card.id)
}

/// Activates the minus and sacrifices `fodder`, aiming the reflexive trigger
/// at the opponent.
fn throw(game: &mut Game, minsc: GameObjectId, fodder: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    ability: AbilityOrigin::Printed { ability, .. },
                    ..
                } if *source == minsc && *ability == AbilityId(2)
            )
        })
        .expect("the minus is offered");
    game.apply(PlayerId::One, action).expect("it is activated");
    settle_wanting(game, &[fodder]);
}

/// Boo enters beside him, which is what the compulsory sacrifice will spend.
#[test]
fn boo_arrives_with_minsc() {
    let (game, _) = staged();

    assert!(boo(&game).is_some(), "Boo is created as Minsc enters");
}

/// The reflexive half reads the power of what was sacrificed, not of what is
/// still on the battlefield, and a Hamster also pays the draw.
#[test]
fn throwing_boo_deals_his_power_and_draws_that_many() {
    let (mut game, minsc) = staged();
    let hamster = boo(&game).expect("Boo is there");
    let hand_before = game.players[0].hand.len();

    throw(&mut game, minsc, hamster);

    assert_eq!(game.players[1].life, 19, "Boo's one power was thrown");
    assert_eq!(
        game.players[0].hand.len(),
        hand_before + 1,
        "and a Hamster draws that many",
    );
}

/// The same clause with a creature that is not a Hamster: the damage still
/// reads its power, and the conditional draw stays silent.
#[test]
fn throwing_a_bear_deals_two_and_draws_nothing() {
    let (mut game, minsc) = staged();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    settle(&mut game);
    let hand_before = game.players[0].hand.len();

    throw(&mut game, minsc, bear);

    assert_eq!(game.players[1].life, 18, "the bear's two power was thrown");
    assert_eq!(
        game.players[0].hand.len(),
        hand_before,
        "and nothing is drawn for a creature that is not a Hamster",
    );
}
