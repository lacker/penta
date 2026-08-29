//! Three cards whose audit lines called evolve, scavenge and regeneration
//! unavailable.
//!
//! All three are built. What is worth pinning is what each card does with
//! them: Experiment One spends the counters evolve banks, and Golgari Decoy's
//! block clause is a requirement rather than a permission.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

fn offers(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

/// The counters evolve banks are the same ones the regeneration spends.
#[test]
fn experiment_one_spends_the_counters_evolve_banked() {
    let mut game = ready();
    let one = creature(10_000, cards::EXPERIMENT_ONE, PlayerId::One);
    let one_id = one.card.id;
    game.battlefield.push(one);

    assert!(
        !offers(&game, one_id),
        "with no counters there is nothing to pay with",
    );

    // Two arrivals, each bigger than the growing Ooze.
    for (index, definition) in [cards::GRIZZLY_BEARS, cards::AIR_ELEMENTAL]
        .into_iter()
        .enumerate()
    {
        game.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent: creature(
                10_100 + u32::try_from(index).expect("a short list"),
                definition,
                PlayerId::One,
            ),
            from: ZoneKind::Hand,
            completion: EntryCompletion::None,
            redirected_to: None,
        });
        drain_pending(&mut game);
    }
    assert_eq!(counters(&game, one_id), 2, "evolved twice");

    game.priority = PlayerId::One;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == one_id),
        )
        .expect("two counters buys one regeneration");
    game.apply(PlayerId::One, action)
        .expect("removing the counters is the whole cost");
    drain_pending(&mut game);

    assert_eq!(counters(&game, one_id), 0, "both counters spent");
}

/// A requirement, not a permission: a creature that could block the Decoy has
/// no other legal block to make.
#[test]
fn the_decoy_forces_every_able_blocker_onto_itself() {
    let mut game = ready();
    game.active_player = PlayerId::One;

    let mut decoy = creature(10_000, cards::GOLGARI_DECOY, PlayerId::One);
    decoy.attacking = true;
    let decoy_id = decoy.card.id;
    game.battlefield.push(decoy);

    let mut other = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    other.attacking = true;
    let other_id = other.card.id;
    game.battlefield.push(other);

    let blocker = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    game.priority = PlayerId::Two;

    let blocks = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, attacker } if blocker == blocker_id => Some(attacker),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        blocks,
        vec![decoy_id],
        "the Decoy is the only block on offer, not one of two",
    );
    assert!(!blocks.contains(&other_id));
}
