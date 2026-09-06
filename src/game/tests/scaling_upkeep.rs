//! Cumulative upkeep paid in coloured mana, and a filter that names only
//! tokens. The upkeep is the interesting half: the payment scales with the
//! age counters, so paying it once tells you nothing about whether the
//! second one costs twice as much.

use super::*;

/// Illusionary Forces under player one at the start of its controller's
/// upkeep, with `islands` untapped Islands to pay with.
fn upkeep(islands: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut forces = creature(83_000, cards::ILLUSIONARY_FORCES, PlayerId::One);
    forces.entered_controller_turn = 0;
    let forces_id = forces.card.id;
    game.battlefield.push(forces);
    for index in 0..islands {
        let mut island = creature(
            83_100 + u32::try_from(index).expect("a small fixture"),
            cards::ISLAND,
            PlayerId::One,
        );
        island.entered_controller_turn = 0;
        game.battlefield.push(island);
    }
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::Upkeep;
    (game, forces_id)
}

/// Runs one upkeep, paying if the option is offered.
fn take_upkeep(game: &mut Game, pay: bool) {
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    game.resolve_stack_top();
    if game.observe(PlayerId::One).decision.is_some() {
        choose_decision_by_label(
            game,
            PlayerId::One,
            if pay { "Pay the cost" } else { "Decline" },
        );
    }
    drain_pending(game);
}

fn survives(game: &Game, forces: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == forces)
}

#[test]
fn the_first_upkeep_costs_one_blue() {
    let (mut game, forces) = upkeep(1);
    take_upkeep(&mut game, true);
    assert!(survives(&game, forces), "one Island covers the first age");
}

#[test]
fn the_second_upkeep_costs_two() {
    let (mut game, forces) = upkeep(1);
    take_upkeep(&mut game, true);
    for permanent in &mut game.battlefield {
        permanent.tapped = false;
    }
    take_upkeep(&mut game, true);
    assert!(
        !survives(&game, forces),
        "one Island is not enough for two age counters"
    );

    let (mut game, forces) = upkeep(2);
    take_upkeep(&mut game, true);
    for permanent in &mut game.battlefield {
        permanent.tapped = false;
    }
    take_upkeep(&mut game, true);
    assert!(survives(&game, forces), "two Islands are");
}

#[test]
fn declining_the_upkeep_sacrifices_it() {
    let (mut game, forces) = upkeep(2);
    take_upkeep(&mut game, false);
    assert!(!survives(&game, forces), "declining eats the creature");
}

/// Dogged Hunter under player one with a Grizzly Bears and a Squirrel token
/// under player two.
fn hunting() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut hunter = creature(83_200, cards::DOGGED_HUNTER, PlayerId::One);
    hunter.entered_controller_turn = 0;
    game.battlefield.push(hunter);
    let mut bear = creature(83_201, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let token = token_permanent(
        83_202,
        tokens::creature(&["Squirrel"], &[ManaColor::Green], 1, 1),
        PlayerId::Two,
    );
    let token_id = token.card.id;
    game.battlefield.push(token);
    (game, bear_id, token_id)
}

#[test]
fn the_hunter_names_only_the_token() {
    let (game, bear, token) = hunting();
    let offered: Vec<Target> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { targets, .. } => Some(targets),
            _ => None,
        })
        .flatten()
        .flat_map(|selection| selection.targets().to_vec())
        .collect();

    assert!(
        offered.contains(&Target::Permanent(token)),
        "the Squirrel token may be destroyed"
    );
    assert!(
        !offered.contains(&Target::Permanent(bear)),
        "and a real Grizzly Bears may not"
    );
}
