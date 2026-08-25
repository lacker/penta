use super::*;

/// `apply_enumerated` accepts exactly what `apply` accepts, and leaves the
/// game in the same state -- it only skips re-deriving legality for an
/// action the caller already enumerated.
#[test]
fn apply_enumerated_matches_apply() {
    let mut through_apply = fixtures::ready_game();
    let mut through_enumerated = through_apply.clone();

    let legal = through_apply.legal_actions(PlayerId::One);
    let action = legal
        .iter()
        .find(|action| !matches!(action, Action::Concede))
        .expect("the ready board offers something besides conceding")
        .clone();

    through_apply
        .apply(PlayerId::One, action.clone())
        .expect("apply accepts a legal action");
    through_enumerated
        .apply_enumerated(PlayerId::One, &legal, action)
        .expect("apply_enumerated accepts the same action");

    assert_eq!(
        through_apply.legal_actions(PlayerId::One),
        through_enumerated.legal_actions(PlayerId::One),
        "the two paths must reach the same position",
    );
    assert_eq!(through_apply.step, through_enumerated.step);
    assert_eq!(through_apply.priority, through_enumerated.priority);
}

/// An action missing from the supplied list is refused, so a stale list
/// cannot smuggle an illegal action past validation.
#[test]
fn apply_enumerated_rejects_an_action_outside_the_list() {
    let mut game = fixtures::ready_game();
    let legal = game.legal_actions(PlayerId::One);
    let action = legal
        .iter()
        .find(|action| !matches!(action, Action::Concede))
        .expect("the ready board offers something besides conceding")
        .clone();
    let without_it: Vec<Action> = legal
        .iter()
        .filter(|candidate| **candidate != action)
        .cloned()
        .collect();

    assert!(matches!(
        game.apply_enumerated(PlayerId::One, &without_it, action),
        Err(ActionError::NotLegal { .. })
    ));
}
