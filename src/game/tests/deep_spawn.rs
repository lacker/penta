//! An upkeep "unless" whose cost is cards off the library rather than mana.
//!
//! Milling is never impossible, so the choice is real even at one card left:
//! a short library mills what it has instead of failing to pay. The card's
//! other half hides the creature for a turn, and the untap prohibition has to
//! outlive the turn it was created on to do anything at all.

use super::*;

/// Deep Spawn on the battlefield with `library` cards under player one.
fn spawned(library: u32) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let spawn = creature(10_000, cards::DEEP_SPAWN, PlayerId::One);
    let spawn_id = spawn.card.id;
    game.battlefield.push(spawn);
    game.players[PlayerId::One.index()].library.clear();
    for index in 0..library {
        game.players[PlayerId::One.index()].library.push(card(
            30_000 + index,
            cards::SEDGE_TROLL,
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.priority = PlayerId::One;
    (game, spawn_id)
}

fn on_battlefield(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

/// Answers each waiting decision by taking the option at `index`, clamped to
/// what is on offer.
fn drain_choosing(game: &mut Game, index: usize) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let pick = index.min(decision.options.len().saturating_sub(1));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[pick].id],
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Runs player one's upkeep, taking the last decision option each time.
fn upkeep_paying(game: &mut Game) {
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(game, usize::MAX);
}

#[test]
fn paying_the_upkeep_mills_two_and_keeps_the_creature() {
    let (mut game, spawn) = spawned(10);
    upkeep_paying(&mut game);

    assert!(on_battlefield(&game, spawn), "paid, so it stays");
    assert_eq!(game.players[PlayerId::One.index()].library.len(), 8);
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 2);
}

/// The control: declining takes the other branch.
#[test]
fn declining_sacrifices_the_creature() {
    let (mut game, spawn) = spawned(10);
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(&mut game, 0);

    assert!(!on_battlefield(&game, spawn), "unpaid, so it goes");
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        10,
        "and nothing was milled",
    );
}

/// A library shorter than the payment is not a failure to pay: it mills what
/// it has and the creature survives.
#[test]
fn a_short_library_still_pays() {
    let (mut game, spawn) = spawned(1);
    upkeep_paying(&mut game);

    assert!(on_battlefield(&game, spawn));
    assert_eq!(game.players[PlayerId::One.index()].library.len(), 0);
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 1);
}

#[test]
fn hiding_taps_it_and_holds_it_down_through_the_next_untap_step() {
    let (mut game, spawn) = spawned(10);
    game.step = Step::PrecombatMain;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == spawn))
        .expect("one blue is enough");
    game.apply(PlayerId::One, action).expect("legal");
    drain_pending(&mut game);

    let tapped = |game: &Game| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == spawn)
            .expect("still there")
            .tapped
    };
    assert!(tapped(&game), "the activation taps it");

    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_choosing(&mut game, usize::MAX);
    assert!(
        tapped(&game),
        "and it stays down through the untap step that follows",
    );
}
