//! Narcomoeba: only the trip from library to graveyard wakes it, and the
//! return remains optional after its trigger resolves.

use super::*;

fn staged(zone: ZoneKind) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    game.players[0].graveyard.clear();
    let narcomoeba = game
        .build_zone(PlayerId::One, &[cards::NARCOMOEBA])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = narcomoeba.id;
    match zone {
        ZoneKind::Library => game.players[0].library.push(narcomoeba),
        ZoneKind::Hand => game.players[0].hand.push(narcomoeba),
        _ => panic!("test stages only a library or hand"),
    }
    (game, id)
}

fn settle(game: &mut Game, accept: bool) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let label = if accept { "Do it" } else { "Decline" };
            let option = decision
                .options
                .iter()
                .find(|option| option.label == label)
                .expect("both optional answers are offered");
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option.id],
                },
            )
            .expect("the decision accepts its offered answer");
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

fn move_to_graveyard(game: &mut Game, id: GameObjectId, from: ZoneKind) {
    game.move_card_from_nonbattlefield_zone(
        id,
        from,
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
    )
    .expect("the card moves to the graveyard");
}

fn narcomoeba_on_battlefield(game: &Game) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == cards::NARCOMOEBA)
}

#[test]
fn milling_narcomoeba_may_put_it_onto_the_battlefield() {
    let (mut game, narcomoeba) = staged(ZoneKind::Library);

    move_to_graveyard(&mut game, narcomoeba, ZoneKind::Library);
    settle(&mut game, true);

    assert!(narcomoeba_on_battlefield(&game));
    assert!(game.players[0].graveyard.is_empty());
}

#[test]
fn declining_leaves_narcomoeba_in_the_graveyard() {
    let (mut game, narcomoeba) = staged(ZoneKind::Library);

    move_to_graveyard(&mut game, narcomoeba, ZoneKind::Library);
    settle(&mut game, false);

    assert!(!narcomoeba_on_battlefield(&game));
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn discarding_narcomoeba_does_not_trigger_it() {
    let (mut game, narcomoeba) = staged(ZoneKind::Hand);

    move_to_graveyard(&mut game, narcomoeba, ZoneKind::Hand);
    settle(&mut game, true);

    assert!(!narcomoeba_on_battlefield(&game));
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert!(game.pending_decisions.is_empty());
}
