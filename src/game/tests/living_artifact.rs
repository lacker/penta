//! Damage banked as counters, spent one a turn for life.
//!
//! The bank fills by the amount of the damage rather than one per event, and
//! it only empties when there is something in it -- "you may remove a
//! counter" with none banked is not a choice, so the trigger does not fire at
//! all.

use super::*;
use crate::ImplementationStatus;

/// Living Artifact on a Mox player one controls, with `banked` counters.
fn enchanted(banked: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let host = creature(10_000, cards::MOX_JET, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let mut aura = creature(10_001, cards::LIVING_ARTIFACT, PlayerId::One);
    aura.attached_to = Some(host_id);
    aura.set_counters(CounterKind::named("vitality"), banked);
    let aura_id = aura.card.id;
    game.battlefield.push(aura);
    (game, aura_id)
}

fn vitality_on(game: &Game, aura: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == aura)
        .expect("still there")
        .counters(CounterKind::named("vitality"))
}

/// Answers every waiting decision by taking the last option, which for a
/// "you may" is the branch that accepts.
fn drain_accepting(game: &mut Game) {
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
            let take = decision.minimum.max(1).min(decision.maximum);
            let options = decision
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
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
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

#[test]
fn damage_banks_counters_by_the_amount_dealt() {
    let (mut game, aura) = enchanted(0);
    let source = creature(10_100, cards::SEDGE_TROLL, PlayerId::Two);
    let source_id = source.card.id;
    game.battlefield.push(source);
    game.damage_target_from(Some(source_id), Some(Target::Player(PlayerId::One)), 3);
    drain_pending(&mut game);

    assert_eq!(
        vitality_on(&game, aura),
        3,
        "three damage banked three, not one",
    );
}

/// The control: damage to the other player is not damage to you.
#[test]
fn damage_to_the_opponent_banks_nothing() {
    let (mut game, aura) = enchanted(0);
    let source = creature(10_100, cards::SEDGE_TROLL, PlayerId::Two);
    let source_id = source.card.id;
    game.battlefield.push(source);
    game.damage_target_from(Some(source_id), Some(Target::Player(PlayerId::Two)), 3);
    drain_pending(&mut game);

    assert_eq!(vitality_on(&game, aura), 0);
}

#[test]
fn a_banked_counter_buys_a_point_of_life() {
    let (mut game, aura) = enchanted(2);
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_accepting(&mut game);

    assert_eq!(vitality_on(&game, aura), 1, "one counter spent");
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) + 1,
    );
}

/// With nothing banked the trigger has nothing to offer, so no life arrives
/// however the decision would have been answered.
#[test]
fn an_empty_bank_buys_nothing() {
    let (mut game, aura) = enchanted(0);
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_accepting(&mut game);

    assert_eq!(vitality_on(&game, aura), 0);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE),
    );
}

#[test]
fn living_artifact_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    let card = catalog
        .get(cards::LIVING_ARTIFACT)
        .expect("the card is cataloged");
    assert_eq!(
        card.rules.implementation_status(),
        ImplementationStatus::Complete,
    );
}
