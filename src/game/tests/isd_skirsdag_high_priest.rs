//! Skirsdag High Priest's morbid gate and exact-count creature tap cost.

use super::*;

fn activation(game: &Game, priest: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, .. } if *source == priest
            )
        })
}

fn choose_permanent(game: &mut Game, permanent: GameObjectId) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the tap-cost choice is pending");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == permanent))
        .expect("the chosen creature is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("choosing a creature for the tap cost is legal");
}

#[test]
fn morbid_and_two_other_untapped_creatures_create_the_demon() {
    let mut game = ready_game();
    game.battlefield.clear();
    let priest = game
        .put_onto_battlefield(PlayerId::One, cards::SKIRSDAG_HIGH_PRIEST)
        .expect("Skirsdag High Priest is cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] += 1;

    let first = creature(10_100, cards::SAVANNAH_LIONS, PlayerId::One);
    let first_id = first.card.id;
    let second = creature(10_101, cards::LLANOWAR_ELVES, PlayerId::One);
    let second_id = second.card.id;
    let opponent = creature(10_102, cards::GRIZZLY_BEARS, PlayerId::Two);
    let opponent_id = opponent.card.id;
    game.battlefield.extend([first, opponent]);

    assert!(
        activation(&game, priest).is_none(),
        "the ability is unavailable before a creature dies this turn",
    );
    game.creature_died_this_turn = true;
    assert!(
        activation(&game, priest).is_none(),
        "the source cannot pay both its own tap symbol and either of the two creature taps",
    );
    game.battlefield.push(second);
    let action = activation(&game, priest)
        .expect("morbid and two other untapped creatures make the ability payable");
    game.apply(PlayerId::One, action)
        .expect("the morbid ability is legal");

    let first_decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the first tap-cost choice is pending");
    let offered = first_decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(id, _)| id))
        .collect::<Vec<_>>();
    assert_eq!(offered, vec![first_id, second_id]);
    assert!(!offered.contains(&priest), "the source already paid {{T}}");
    assert!(!offered.contains(&opponent_id));

    choose_permanent(&mut game, first_id);
    let second_decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the second tap-cost choice is pending");
    assert_eq!(
        second_decision
            .options
            .iter()
            .filter_map(|option| option.card.map(|(id, _)| id))
            .collect::<Vec<_>>(),
        vec![second_id],
        "one permanent cannot pay the same exact-count cost twice",
    );
    choose_permanent(&mut game, second_id);

    for id in [priest, first_id, second_id] {
        assert!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .expect("the cost payer remains on the battlefield")
                .tapped,
        );
    }
    assert_eq!(game.stack.len(), 1, "the paid ability uses the stack");
    pass_priority_pair(&mut game);

    let demon = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Token
                && game.effective_subtypes(permanent).contains(&"Demon")
        })
        .expect("the ability creates a Demon token");
    assert_eq!(
        (game.power(demon), game.toughness(demon)),
        (Some(5), Some(5))
    );
    assert_eq!(
        game.effective_colors(demon, &game.effective_rules(demon).unwrap()),
        [false, false, true, false, false],
    );
    assert!(game.permanent_has_executable_keyword(demon, KeywordAbility::Flying));
}
