// `enumerate_legal_actions` keeps the list it hands out so the `apply` that
// follows validates against it rather than enumerating a second time. These
// tests hold it to the one promise that matters: the fast path must accept
// and refuse exactly what the slow one does.

use super::*;

/// Picks an action the way a search would: deterministically, and never the
/// concession that would end the game on the first ply.
fn pick(actions: &[Action], step: u64) -> Action {
    let choices: Vec<&Action> = actions
        .iter()
        .filter(|action| !matches!(action, Action::Concede))
        .collect();
    let choices = if choices.is_empty() {
        actions.iter().collect()
    } else {
        choices
    };
    let index = usize::try_from(step % choices.len() as u64).unwrap_or(0);
    choices[index].clone()
}

/// A bounded decision is advertised as one placeholder action plus its
/// selection schema. Materialize the deterministic minimum-size selection a
/// search would submit before asking either validation path to apply it.
fn materialize_bounded_selection(game: &Game, action: Action) -> Action {
    let Action::ChooseDecision { decision, .. } = action else {
        return action;
    };
    let observation = &game
        .pending_decisions
        .first()
        .expect("the decision placeholder names a pending decision")
        .observation;
    assert_eq!(observation.id, decision);
    Action::ChooseDecision {
        decision,
        options: observation
            .options
            .iter()
            .take(observation.minimum)
            .map(|option| option.id)
            .collect(),
    }
}

/// The whole point, over a whole game: enumerating through the engine and
/// applying reaches exactly the position that enumerating separately and
/// applying does, ply for ply.
///
/// This is also what exercises the memo's debug assertion, which re-derives
/// the list on every kept-enumeration hit and compares. A mutating path that
/// forgot to drop a stale list fails here rather than in a caller's search.
#[test]
fn enumerating_through_the_engine_reaches_the_same_position() {
    let mut kept = fixtures::ready_game();
    let mut derived = kept.clone();

    for step in 0..600 {
        let Some(player) = kept.decision_player() else {
            break;
        };
        assert_eq!(
            derived.decision_player(),
            Some(player),
            "the two games diverged on whose decision it is at ply {step}",
        );

        let through_kept = pick(kept.enumerate_legal_actions(player), step);
        let through_kept = materialize_bounded_selection(&kept, through_kept);
        let through_derived = pick(&derived.legal_actions(player), step);
        let through_derived = materialize_bounded_selection(&derived, through_derived);
        assert_eq!(
            through_kept, through_derived,
            "the kept enumeration differed from a fresh one at ply {step}",
        );

        kept.apply(player, through_kept)
            .expect("the kept enumeration offered a legal action");
        derived
            .apply(player, through_derived)
            .expect("a fresh enumeration offered a legal action");

        assert_eq!(
            kept.legal_actions(player),
            derived.legal_actions(player),
            "the two games reached different positions at ply {step}",
        );
    }

    assert_eq!(kept.result, derived.result);
    assert_eq!(kept.step, derived.step);
    assert_eq!(kept.priority, derived.priority);
}

/// A mutation between enumerating and applying drops the kept list, so an
/// action that was legal a moment ago is still refused once it is not.
///
/// This is the property that makes the list safe to keep at all, and the
/// reason it lives in the engine rather than being handed to a caller: only
/// the engine sees every way a game can change.
#[test]
fn a_mutation_between_enumerating_and_applying_is_not_trusted() {
    let mut game = fixtures::ready_game();
    game.set_hand(PlayerId::One, &[cards::MOUNTAIN])
        .expect("the fixture accepts a one-card hand");

    let played_land = game
        .enumerate_legal_actions(PlayerId::One)
        .iter()
        .find(|action| matches!(action, Action::PlayLand { .. }))
        .expect("a land in hand on a main phase is playable")
        .clone();

    // The land is gone, so playing it is no longer legal -- but the list
    // enumerated a moment ago still names it.
    game.set_hand(PlayerId::One, &[])
        .expect("the fixture accepts an empty hand");

    assert!(
        matches!(
            game.apply(PlayerId::One, played_land),
            Err(ActionError::NotLegal { .. })
        ),
        "a list from before the mutation must not vouch for an action after it",
    );
}

/// A clone starts with nothing enumerated, so it validates the long way
/// rather than trusting a list its parent was holding.
#[test]
fn an_enumeration_does_not_survive_a_clone() {
    let mut parent = fixtures::ready_game();
    parent.set_hand(PlayerId::One, &[cards::MOUNTAIN]).unwrap();
    let played_land = parent
        .enumerate_legal_actions(PlayerId::One)
        .iter()
        .find(|action| matches!(action, Action::PlayLand { .. }))
        .expect("a land in hand on a main phase is playable")
        .clone();

    let mut clone = parent.clone();
    assert!(
        clone.enumerated.0.is_none(),
        "a cloned game must not carry its parent's enumeration",
    );

    clone
        .apply(PlayerId::One, played_land)
        .expect("the clone validates the action for itself and accepts it");
}

/// A decision is checked against the pending selection schema either way: the
/// enumerated list carries a placeholder rather than every combination, so
/// trusting it would accept option sets that were never offered.
#[test]
fn a_decision_is_still_validated_against_its_schema() {
    let mut game = fixtures::ready_game();
    let player = PlayerId::One;
    let _ = game.enumerate_legal_actions(player);

    // No decision is pending, so any decision action is refused -- the kept
    // list must not be what answers that question.
    let invented = Action::ChooseDecision {
        decision: 0,
        options: Vec::new(),
    };
    assert!(matches!(
        game.apply(player, invented),
        Err(ActionError::NotLegal { .. })
    ));
}
