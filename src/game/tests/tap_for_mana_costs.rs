//! Immediate mana payments share chosen tap costs and reserve each untapped state.
use super::*;

fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[0] = 3;
    let drum = creature(10_000, cards::SPRINGLEAF_DRUM, PlayerId::One);
    let mut helper = creature(10_001, cards::LLANOWAR_ELVES, PlayerId::One);
    helper.entered_controller_turn = 3;
    let ids = (drum.card.id, helper.card.id);
    game.battlefield.extend([drum, helper]);
    (game, ids.0, ids.1)
}

fn mana_actions(game: &Game, source: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One).into_iter().filter(|action|
        matches!(action, Action::ActivateManaAbility { source: candidate, .. } if *candidate == source)
    ).collect()
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let spell = card(10_100, definition, PlayerId::One);
    let id = spell.id;
    game.players[0].hand.push(spell);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .expect("the joint mana payment is legal");
    game.apply(PlayerId::One, action).unwrap();
}

#[test]
fn springleaf_drum_chooses_each_color_and_can_tap_a_summoning_sick_creature() {
    let (game, drum, helper) = staged();
    assert!(mana_actions(&game, helper).is_empty());
    let actions = mana_actions(&game, drum);
    assert_eq!(actions.len(), 5);
    for action in actions {
        let Action::ActivateManaAbility {
            color, cost_object, ..
        } = &action
        else {
            unreachable!()
        };
        assert_ne!(*color, ManaColor::Colorless);
        assert_eq!(*cost_object, Some(helper));
        let mut played = game.clone();
        played.apply(PlayerId::One, action.clone()).unwrap();
        assert_eq!(played.players[0].mana_pool.amount(*color), 1);
        assert_eq!(played.players[0].mana_pool.total(), 1);
        assert!(played.battlefield.iter().all(|permanent| permanent.tapped));
        assert!(played.stack.is_empty());
        assert!(played.pending_decisions.is_empty());
        assert!(mana_actions(&played, drum).is_empty());
    }
}

#[test]
fn tap_cost_filters_control_type_and_tapped_state_and_rejects_stale_choices() {
    let (mut game, drum, helper) = staged();
    let action = mana_actions(&game, drum).remove(0);
    game.battlefield[1].tapped = true;
    game.battlefield
        .push(creature(10_002, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(10_003, cards::SOL_RING, PlayerId::One));
    assert!(mana_actions(&game, drum).is_empty());
    assert!(game.apply(PlayerId::One, action).is_err());
    assert!(!game.battlefield[0].tapped);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    game.battlefield[1].tapped = false;
    let choices = mana_actions(&game, drum);
    assert_eq!(choices.len(), 5);
    assert!(choices.iter().all(|action|
        matches!(action, Action::ActivateManaAbility { cost_object: Some(id), .. } if *id == helper)
    ));
    game.battlefield[0].tapped = true;
    assert!(mana_actions(&game, drum).is_empty());
}

#[test]
fn automatic_payment_uses_drum_without_using_the_creatures_own_mana_ability() {
    let (mut game, _, _) = staged();
    cast(&mut game, cards::SAVANNAH_LIONS);
    assert_eq!(game.stack.len(), 1);
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn two_drums_cannot_tap_one_creature_twice_but_can_choose_distinct_creatures() {
    let (mut game, _, _) = staged();
    game.battlefield
        .push(creature(10_002, cards::SPRINGLEAF_DRUM, PlayerId::One));
    assert!(!game.can_pay_cost(PlayerId::One, mana_cost!("{U}{R}"), 0));
    let mut helper = creature(10_003, cards::GRIZZLY_BEARS, PlayerId::One);
    helper.entered_controller_turn = 3;
    game.battlefield.push(helper);
    cast(&mut game, cards::GOBLIN_ELECTROMANCER);
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
}

#[test]
fn a_creature_cannot_tap_for_its_own_mana_and_for_drum_in_the_same_payment() {
    let (mut game, _, _) = staged();
    game.battlefield[1].entered_controller_turn = 0;
    assert!(!game.can_pay_cost(PlayerId::One, mana_cost!("{1}{G}"), 0));
    // A different untapped creature makes the two mana independently payable.
    game.battlefield
        .push(creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One));
    cast(&mut game, cards::GRIZZLY_BEARS);
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
}

#[test]
fn drum_and_convoke_cannot_share_the_same_tap() {
    let (mut game, _, _) = staged();
    let spell = card(10_100, cards::SPROUT_SWARM, PlayerId::One);
    let id = spell.id;
    game.players[0].hand.push(spell);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
    );
    game.battlefield
        .push(creature(10_002, cards::ORNITHOPTER, PlayerId::One));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.tapped)
            .count(),
        2
    );
}

#[test]
fn a_creature_can_pay_drums_tap_before_sacrificing_itself_for_mana() {
    let (mut game, _, _) = staged();
    game.battlefield[1] = creature(10_001, cards::MORGUE_TOAD, PlayerId::One);
    // The colored spell makes payment ordering prefer the Toad unless the
    // planner retains the dependency on its still-existing body.
    cast(&mut game, cards::DIVINATION);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].tapped);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MORGUE_TOAD)
    );
}

#[test]
fn drum_can_tap_the_creature_that_another_mana_ability_will_sacrifice() {
    let (mut game, _, _) = staged();
    game.battlefield[1] = creature(10_001, cards::GOBLIN_MATRON, PlayerId::One);
    game.battlefield
        .push(creature(10_002, cards::SKIRK_PROSPECTOR, PlayerId::One));
    cast(&mut game, cards::DIVINATION);
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.players[0].graveyard.len(), 2);
}

#[test]
fn chosen_tap_and_immediate_mana_survive_checkpoint_reconstruction() {
    let (mut game, drum, _) = staged();
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
            .unwrap();
    let actions = mana_actions(&game, drum);
    assert_eq!(actions, mana_actions(&restored, drum));
    let action = actions[0].clone();
    game.apply(PlayerId::One, action.clone()).unwrap();
    restored.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, restored.players[0].mana_pool);
    assert_eq!(game.observe(PlayerId::One), restored.observe(PlayerId::One));
}

#[test]
fn an_animated_drum_cannot_pay_both_tap_costs_with_itself() {
    for prepared in [false, true] {
        let (mut game, drum, helper) = staged();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield
            .retain(|permanent| permanent.card.id != helper);
        let mut animation = creature(10_002, cards::ANIMATE_ARTIFACT, PlayerId::One);
        animation.attached_to = Some(drum);
        game.battlefield.push(animation);
        assert!(
            game.permanent_types(&game.battlefield[0])
                .unwrap()
                .contains(CardType::Creature)
        );
        assert!(mana_actions(&game, drum).is_empty());
        game.battlefield
            .push(creature(10_003, cards::ORNITHOPTER, PlayerId::One));
        assert_eq!(mana_actions(&game, drum).len(), 5);
        game.battlefield[0].entered_controller_turn = 3;
        assert!(
            mana_actions(&game, drum).is_empty(),
            "the animated Drum's own tap symbol requires readiness"
        );
    }
}

#[test]
fn tapping_a_land_creature_for_drum_triggers_ordinary_taps_but_not_mana_taps() {
    let (mut game, drum, helper) = staged();
    game.battlefield[1] = creature(helper.0, cards::DRYAD_ARBOR, PlayerId::One);
    for (id, definition) in [(10_002, cards::OVERGROWTH), (10_003, cards::PSYCHIC_VENOM)] {
        let mut aura = creature(id, definition, PlayerId::One);
        aura.attached_to = Some(helper);
        game.battlefield.push(aura);
    }
    game.apply(PlayerId::One, mana_actions(&game, drum).remove(0))
        .unwrap();
    assert_eq!(
        game.players[0].mana_pool.total(),
        1,
        "Overgrowth does not add mana for the chosen payer"
    );
    assert_eq!(game.stack.len(), 1, "Psychic Venom sees the ordinary tap");
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 18);
}

#[test]
fn drum_preserves_a_source_reserved_for_another_abilitys_tap_symbol() {
    let (game, _, helper) = staged();
    assert!(!game.can_pay_cost_for(
        PlayerId::One,
        mana_cost!("{G}"),
        0,
        &ManaPaymentPurpose::Ability {
            source: helper,
            taps_source: true,
            leaves_source: false
        }
    ));
    assert!(game.can_pay_cost_for(
        PlayerId::One,
        mana_cost!("{G}"),
        0,
        &ManaPaymentPurpose::Ability {
            source: helper,
            taps_source: false,
            leaves_source: true
        }
    ));
}
