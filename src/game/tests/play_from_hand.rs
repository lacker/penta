//! Putting a card from your hand onto the battlefield.
//!
//! Not casting it: no spell, no stack, no cost beyond the ability's own. The
//! choice reads a hidden zone, so it is offered as a decision, and "you may"
//! is a minimum of zero rather than a separate question.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

/// Activates `source`'s ability at `index` and returns the offer it makes.
fn offer(game: &mut Game, source: GameObjectId, index: usize) -> DecisionObservation {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .nth(index)
        .expect("the ability is activatable");
    game.apply(PlayerId::One, action).expect("it is activated");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the choice was offered")
}

fn answer(game: &mut Game, decision: &DecisionObservation, options: Vec<u32>) {
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("the decision accepts what it offered");
    drain_pending(game);
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

/// The Wizard offers only the Goblin, and putting it there is not casting it.
#[test]
fn the_wizard_offers_only_goblins_from_your_hand() {
    let mut game = ready();
    let wizard = creature(10_000, cards::GOBLIN_WIZARD, PlayerId::One);
    let wizard_id = wizard.card.id;
    game.battlefield.push(wizard);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()].hand.push(card(
        20_000,
        cards::GOBLINS_OF_THE_FLARG,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].hand.push(card(
        20_001,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));

    let decision = offer(&mut game, wizard_id, 0);
    assert_eq!(
        decision.options.len(),
        1,
        "the bear is not a Goblin, so it is not on offer",
    );
    let chosen = decision.options[0].id;
    answer(&mut game, &decision, vec![chosen]);

    assert!(on_battlefield(&game, cards::GOBLINS_OF_THE_FLARG));
    assert!(game.stack.is_empty(), "it was put there, not cast");
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        1,
        "and only the bear is left in hand",
    );
}

/// "You may" means the offer can be answered with nothing.
#[test]
fn the_wizard_may_be_declined() {
    let mut game = ready();
    let wizard = creature(10_000, cards::GOBLIN_WIZARD, PlayerId::One);
    let wizard_id = wizard.card.id;
    game.battlefield.push(wizard);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()].hand.push(card(
        20_000,
        cards::GOBLINS_OF_THE_FLARG,
        PlayerId::One,
    ));

    let decision = offer(&mut game, wizard_id, 0);
    assert_eq!(decision.minimum, 0, "nothing is a legal answer");
    answer(&mut game, &decision, Vec::new());

    assert!(!on_battlefield(&game, cards::GOBLINS_OF_THE_FLARG));
    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 1);
}

/// Gaea's Touch names a basic Forest, so a nonbasic that taps for green is
/// not on offer -- and the land it puts down is a free extra one.
#[test]
fn gaeas_touch_offers_only_a_basic_forest() {
    let mut game = ready();
    let touch = creature(10_000, cards::GAEAS_TOUCH, PlayerId::One);
    let touch_id = touch.card.id;
    game.battlefield.push(touch);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()]
        .hand
        .push(card(20_000, cards::FOREST, PlayerId::One));
    game.players[PlayerId::One.index()]
        .hand
        .push(card(20_001, cards::ISLAND, PlayerId::One));

    let decision = offer(&mut game, touch_id, 0);
    assert_eq!(
        decision.options.len(),
        1,
        "an Island is basic but not a Forest",
    );
    let chosen = decision.options[0].id;
    answer(&mut game, &decision, vec![chosen]);

    assert!(on_battlefield(&game, cards::FOREST));
    assert!(
        game.players[PlayerId::One.index()].lands_played_this_turn == 0,
        "putting a land there is not playing one",
    );
}
