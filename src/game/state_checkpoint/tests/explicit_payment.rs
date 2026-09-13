use super::rare_states::assert_reconstructs;
use crate::game::tests::{card, ready_game};
use crate::{Action, ManaColor, PlayerId};

fn choose(game: &mut crate::Game, option: u32) {
    let decision = game.pending_decisions[0].observation.id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision,
            options: vec![option],
        },
    )
    .unwrap();
}

#[test]
fn explicit_payment_choices_reconstruct_before_and_after_selecting_a_unit() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, crate::card::cards::SOL_RING, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let index = game
        .manual_payment_actions(PlayerId::One)
        .iter()
        .position(|action| {
            matches!(action,
        Action::CastSpell { card, .. } if card.0 == 10_000)
        })
        .unwrap();
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    assert_reconstructs(&game, "explicit operation selection");
    choose(
        &mut game,
        u32::try_from(index).expect("option index fits u32"),
    );
    assert_reconstructs(&game, "explicit funding selection");
    choose(&mut game, 0);
    assert_reconstructs(&game, "explicit mana selection");
    choose(&mut game, 1);
    assert_reconstructs(&game, "explicit selected mana waiting for commit");
    choose(&mut game, 0);
    assert_eq!(game.players[0].mana_pool.green, 1);
}

#[test]
fn explicit_payment_reconstructs_the_enclosing_resolving_cost() {
    let mut game = crate::game::tests::cost_lists::mixed_echo_pending_game();
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    assert_reconstructs(&game, "explicit resolving payment operations");
    let option = game.pending_decisions[0]
        .observation
        .options
        .iter()
        .find(|o| o.label.starts_with("Choose mana:"))
        .unwrap()
        .id;
    choose(&mut game, option);
    assert_reconstructs(&game, "explicit resolving payment allocation");
    choose(&mut game, 1);
    assert_reconstructs(&game, "explicit resolving payment before commit");
    choose(&mut game, 0);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn explicit_payment_reconstructs_funding_steps_and_replacement_answers() {
    use crate::card::cards;
    use crate::game::tests::creature;
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SKIRK_PROSPECTOR, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::MOGG_FANATIC, PlayerId::One));
    for id in [10_003, 10_004] {
        game.battlefield
            .push(creature(id, cards::REST_IN_PEACE, PlayerId::Two));
    }
    let index = game
        .manual_payment_actions(PlayerId::One)
        .iter()
        .position(|a| matches!(a, Action::CastSpell { .. }))
        .unwrap();
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    choose(
        &mut game,
        u32::try_from(index).expect("option index fits u32"),
    );
    let crate::game::DecisionContinuation::Payment(
        crate::game::payment::state::PaymentDecision::Funding(draft),
    ) = &game.pending_decisions[0].continuation
    else {
        unreachable!()
    };
    let index = game.funding_candidates(draft).unwrap().iter().position(|a| matches!(a, Action::ActivateManaAbility { cost_object: Some(chosen), .. } if chosen.0 == 10_002)).unwrap();
    choose(
        &mut game,
        u32::try_from(index).expect("option index fits u32") + 1,
    );
    assert_reconstructs(&game, "a proposed mana cost awaits a replacement answer");
    let option = game.pending_decisions[0].observation.options[0].id;
    choose(&mut game, option);
    assert_reconstructs(
        &game,
        "a proposed mana ability records its replacement answer",
    );
    choose(&mut game, 0);
    choose(&mut game, 1);
    assert_reconstructs(&game, "proposed produced mana awaits final payment");
    choose(&mut game, 0);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn explicit_payment_reconstructs_a_manual_x_announcement() {
    let mut game = ready_game();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.players[0]
        .hand
        .push(card(10_000, crate::card::cards::EARTHQUAKE, PlayerId::One));
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    choose(&mut game, u32::MAX - 1); // X = 10, regardless of the current mana ceiling.
    assert_reconstructs(&game, "manual X choices before funding");
    let index = game
        .manual_payment_actions(PlayerId::One)
        .iter()
        .position(|a| matches!(a, Action::CastSpell { choices, .. } if choices.x() == 10))
        .unwrap();
    choose(
        &mut game,
        u32::try_from(index).expect("option index fits u32"),
    );
    assert_reconstructs(&game, "an unfunded X spell proposal");
}

#[test]
fn explicit_payment_reconstructs_direct_contributions_and_can_cancel_them() {
    use crate::card::cards;
    use crate::game::tests::creature;
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SPROUT_SWARM, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One));
    let index = game.manual_payment_actions(PlayerId::One).iter().position(|a| matches!(a, Action::CastSpell { choices, .. } if choices.costs().additional().is_empty())).unwrap();
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    choose(
        &mut game,
        u32::try_from(index).expect("option index fits u32"),
    );
    for label in [
        "Convoke Grizzly Bears to pay {G}",
        "Convoke Savannah Lions to pay {1}",
    ] {
        let option = game.pending_decisions[0]
            .observation
            .options
            .iter()
            .find(|o| o.label == label)
            .unwrap()
            .id;
        choose(&mut game, option);
        assert_reconstructs(&game, "a chosen direct cost contribution");
    }
    choose(&mut game, 0);
    assert_reconstructs(&game, "a fully convoked spell awaiting confirmation");
    let decision = game.pending_decisions[0].observation.id;
    game.apply(PlayerId::One, Action::CancelDecision { decision })
        .unwrap();
    assert!(game.battlefield.iter().all(|p| !p.tapped));
    assert_eq!(game.players[0].hand.len(), 1);
}
