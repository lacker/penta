//! The Onslaught chain cycle and Chain Lightning: the affected permanent's
//! controller decides whether the spell keeps moving, and controls its copy.

use super::*;

fn choose_first_non_decline(game: &mut Game, player: PlayerId) {
    let decision = game
        .observe(player)
        .decision
        .expect("the chain choice is pending");
    let option = decision
        .options
        .iter()
        .find(|option| option.label != "Decline")
        .expect("the chain can be continued")
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the chain choice is legal");
}

fn choose_copy_target(game: &mut Game, player: PlayerId, target: Target) {
    let decision = game
        .observe(player)
        .decision
        .expect("the copied chain may choose a new target");
    let option = match &game
        .pending_decisions
        .first()
        .expect("the retarget decision is pending")
        .continuation
    {
        DecisionContinuation::CopyStackObject { target_lists, .. } => target_lists
            .iter()
            .position(|targets| flatten_target_selections(targets) == [target])
            .and_then(|index| u32::try_from(index).ok())
            .expect("the requested target is offered"),
        continuation => panic!("unexpected copy continuation: {continuation:?}"),
    };
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the copied chain target is legal");
}

#[test]
fn chain_of_silence_sacrifices_then_copies_and_prevents_each_targets_damage() {
    let mut game = ready_game();
    let first = creature(170_000, cards::SERRA_ANGEL, PlayerId::Two);
    let second = creature(170_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let land = creature(170_002, cards::MOUNTAIN, PlayerId::Two);
    let first_id = first.card.id;
    let second_id = second.card.id;
    let land_id = land.card.id;
    game.battlefield.extend([first, second, land]);

    let chain = card(170_003, cards::CHAIN_OF_SILENCE, PlayerId::One);
    let chain_id = chain.id;
    game.players[0].hand.push(chain);
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(chain_id, vec![Target::Permanent(first_id)], Vec::new(), 0),
    )
    .expect("Chain of Silence is castable");
    pass_priority_pair(&mut game);

    choose_first_non_decline(&mut game, PlayerId::Two);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != land_id),
        "the target's controller sacrificed their land",
    );
    choose_decision_by_label(&mut game, PlayerId::Two, "Do it");
    choose_copy_target(&mut game, PlayerId::Two, Target::Permanent(second_id));
    pass_priority_pair(&mut game);

    assert_eq!(
        game.damage_target_from(Some(first_id), Some(Target::Player(PlayerId::One)), 4,),
        0,
    );
    assert_eq!(
        game.damage_target_from(Some(second_id), Some(Target::Player(PlayerId::Two)), 2,),
        0,
    );
}

#[test]
fn chain_of_acid_uses_last_known_controller_then_destroys_the_copys_target() {
    let mut game = ready_game();
    let first = creature(170_010, cards::BLACK_VISE, PlayerId::Two);
    let second = creature(170_011, cards::BLACK_VISE, PlayerId::One);
    let first_id = first.card.id;
    let second_id = second.card.id;
    game.battlefield.extend([first, second]);

    let chain = card(170_012, cards::CHAIN_OF_ACID, PlayerId::One);
    let chain_id = chain.id;
    game.players[0].hand.push(chain);
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;
    game.apply(
        PlayerId::One,
        cast_action(chain_id, vec![Target::Permanent(first_id)], Vec::new(), 0),
    )
    .expect("Chain of Acid is castable");
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first_id),
        "the original target was destroyed before its controller chose",
    );
    choose_decision_by_label(&mut game, PlayerId::Two, "Do it");
    choose_copy_target(&mut game, PlayerId::Two, Target::Permanent(second_id));
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first_id && permanent.card.id != second_id),
        "the copy destroyed its separately chosen target",
    );
}
