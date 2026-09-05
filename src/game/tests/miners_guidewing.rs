//! Miner's Guidewing: the first card in the catalog to explore, so this
//! covers both branches of the procedure -- a land goes to hand and the
//! creature stays the same size, anything else grows it and asks where the
//! revealed card goes.

use super::*;

/// Player One with the Guidewing about to die and `top` on their library.
fn staged(top: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library = vec![card(10_200, top, PlayerId::One)];
    let bird = creature(10_000, cards::MINER_S_GUIDEWING, PlayerId::One);
    let bird_id = bird.card.id;
    game.battlefield.push(bird);
    let mut explorer = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    explorer.entered_controller_turn = 0;
    let explorer_id = explorer.card.id;
    game.battlefield.push(explorer);
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, bird_id, explorer_id)
}

/// Kills the Bird outright and resolves the explore, keeping the revealed
/// card on top whenever that choice is offered.
fn kill_the_bird(game: &mut Game, bird: GameObjectId) {
    game.damage_target_from(None, Some(Target::Permanent(bird)), 5);
    game.check_state_based_actions();
    settle(game);
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game.pending_decisions.first().cloned() {
            let option = decision
                .observation
                .options
                .first()
                .expect("an offered decision has at least one option")
                .id;
            if game
                .apply(
                    decision.observation.player,
                    Action::ChooseDecision {
                        decision: decision.observation.id,
                        options: vec![option],
                    },
                )
                .is_err()
            {
                break;
            }
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
    game.check_state_based_actions();
}

fn explorer(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the explorer is still on the battlefield")
}

#[test]
fn revealing_a_land_puts_it_in_hand_and_leaves_the_creature_alone() {
    let (mut game, bird_id, explorer_id) = staged(cards::PLAINS);
    kill_the_bird(&mut game, bird_id);

    assert_eq!(game.players[0].hand.len(), 1, "the land is put into hand");
    assert!(game.players[0].library.is_empty());
    assert_eq!(
        explorer(&game, explorer_id).counters(CounterKind::PlusOnePlusOne),
        0,
        "a land does not grow the explorer"
    );
}

#[test]
fn revealing_a_nonland_grows_the_creature_instead() {
    let (mut game, bird_id, explorer_id) = staged(cards::SAVANNAH_LIONS);
    kill_the_bird(&mut game, bird_id);

    assert!(game.players[0].hand.is_empty(), "a nonland is not drawn");
    assert_eq!(
        explorer(&game, explorer_id).counters(CounterKind::PlusOnePlusOne),
        1,
        "a nonland puts a +1/+1 counter on the explorer"
    );
}
