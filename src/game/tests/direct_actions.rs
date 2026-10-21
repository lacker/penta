use super::*;
use crate::action::{ExplicitFundingStep, ExplicitPayment};

#[test]
fn direct_actions_play_permission_metadata_tracks_visibility_and_lifetime() {
    let mut game = ready_game();
    let bolt = GameObjectId(10_000);
    game.players[0]
        .exile
        .push(card(bolt.0, cards::LIGHTNING_BOLT, PlayerId::One));
    game.permit_look_while_exiled(bolt, PlayerId::One);
    assert!(game.action_play_permission(PlayerId::One, bolt).is_none());
    game.exile_play_permissions.clear();
    game.permit_energy_cast(bolt, PlayerId::One);
    let permission = game.action_play_permission(PlayerId::One, bolt).unwrap();
    assert_eq!(permission["cost"]["kind"], "energyEqualToManaValue");
    assert!(permission["cost"]["manaCost"].is_null());
    assert!(game.action_play_permission(PlayerId::Two, bolt).is_none());

    game.exile_play_permissions.clear();
    let card = game.players[0].exile.pop().unwrap();
    game.players[0].graveyard.push(card);
    game.permit_graveyard_cast_this_turn(bolt, PlayerId::One);
    let permission = game.action_play_permission(PlayerId::One, bolt).unwrap();
    assert_eq!(permission["sourceZone"], "graveyard");
    assert_eq!(permission["cost"]["kind"], "printed");
    game.active_player = PlayerId::Two;
    assert!(game.action_play_permission(PlayerId::One, bolt).is_none());
}

#[test]
fn direct_actions_describe_granted_costs_without_discovery() {
    let mut game = ready_game();
    let bolt = GameObjectId(10_000);
    game.players[0]
        .graveyard
        .push(card(bolt.0, cards::LIGHTNING_BOLT, PlayerId::One));
    game.players[0]
        .hand
        .push(card(10_001, cards::SNAPCASTER_MAGE, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    game.apply(
        PlayerId::One,
        cast_action(GameObjectId(10_001), vec![], vec![], 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let decision = game.observe_state(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let costs = game.action_cast_costs(PlayerId::One, bolt);
    assert_eq!(costs.len(), 1);
    assert_eq!(costs[0]["alternativeCostId"], 255);
    assert_eq!(costs[0]["manaCost"], "{R}");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    crate::protocol::ActionProposal::parse(&serde_json::json!({"action":{
    "type":"CastSpell","card":bolt.0,"choices":{
        "alternativeCostId":costs[0]["alternativeCostId"],
        "targetSelections":[{"slotId":0,"targets":[{"type":"player","seat":"p2"}]}]
    }}}))
    .unwrap()
    .apply(&mut game, PlayerId::One)
    .unwrap();

    let mut game = ready_game();
    game.battlefield
        .push(creature(10_002, cards::OMNISCIENCE, PlayerId::One));
    game.players[0]
        .hand
        .push(card(bolt.0, cards::LIGHTNING_BOLT, PlayerId::One));
    let costs = game.action_cast_costs(PlayerId::One, bolt);
    assert_eq!(costs[0]["kind"], "battlefield");
    assert_eq!(costs[0]["alternativeCostId"], 254);
    assert_eq!(costs[0]["manaCost"], "{0}");
}

#[test]
fn direct_actions_combination_funding_does_not_require_redundant_color() {
    let mut game = ready_game();
    game.players[0].library = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS, cards::FOREST])
        .unwrap();
    let watch = game
        .put_onto_battlefield(PlayerId::One, cards::INTERDIMENSIONAL_WEB_WATCH)
        .unwrap();
    drain_pending(&mut game);
    let exiled = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == cards::GRIZZLY_BEARS)
        .unwrap()
        .id;
    let ability = game
        .action_abilities(watch)
        .into_iter()
        .find(|(_, ability)| matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_)))
        .unwrap()
        .0;
    let proposal = serde_json::json!({"action":{"type":"CastSpell","card":exiled.0,"choices":{}},
        "payment":{"funding":[{"action":{"type":"ActivateManaAbility","source":watch.0,
            "ability":crate::protocol::ability_origin_json(ability),"combination":{"green":1,"blue":1}}}]}});
    for color in [None, Some("green"), Some("blue")] {
        let mut proposed = proposal.clone();
        if let Some(color) = color {
            proposed["payment"]["funding"][0]["action"]["color"] = serde_json::json!(color);
        }
        let mut trial = game.clone();
        crate::protocol::ActionProposal::parse(&proposed)
            .unwrap()
            .apply(&mut trial, PlayerId::One)
            .unwrap();
        assert_eq!(trial.stack.len(), 1);
        assert_eq!(trial.players[0].mana_pool.total(), 0);
    }
}

fn sol_ring_proposal() -> Action {
    Action::CastSpell {
        card: GameObjectId(10_000),
        choices: CastChoices::default(),
        sacrifices: Vec::new(),
    }
}

#[test]
fn direct_actions_omitted_cast_choices_do_not_choose_targets() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.players[0]
        .hand
        .push(card(10_001, cards::LIGHTNING_BOLT, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    let proposal = |card| {
        crate::protocol::ActionProposal::parse(&serde_json::json!({
            "action":{"type":"CastSpell","card":card}
        }))
        .unwrap()
    };
    proposal(10_000).apply(&mut game, PlayerId::One).unwrap();
    let before = game.observe_state(PlayerId::One);
    assert!(proposal(10_001).apply(&mut game, PlayerId::One).is_err());
    assert_eq!(game.observe_state(PlayerId::One), before);
}

#[test]
fn direct_actions_cost_objects_pay_evidence_without_sacrificing_permanents() {
    let mut game = ready_game();
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::HEARTH_ELEMENTAL, cards::FOREST])
        .unwrap();
    let evidence = game.players[0]
        .graveyard
        .iter()
        .map(|card| card.id.0)
        .collect::<Vec<_>>();
    game.players[0]
        .hand
        .push(card(10_000, cards::DEADLY_COVER_UP, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
    let mut proposal = serde_json::json!({"action":{"type":"CastSpell","card":10_000,
        "choices":{"additionalCostIds":[0]},"costObjects":evidence}});
    crate::protocol::ActionProposal::parse(&proposal)
        .unwrap()
        .apply(&mut game, PlayerId::One)
        .unwrap();
    assert_eq!(game.players[0].exile.len(), 2);
    assert!(game.players[0].graveyard.is_empty());
    proposal["action"]["sacrifices"] = serde_json::json!([]);
    assert!(crate::protocol::ActionProposal::parse(&proposal).is_err());
}

#[test]
fn direct_actions_explicit_funding_is_atomic_and_uses_selected_sources() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::ISLAND, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::FOREST, PlayerId::One));
    let mut mana = Vec::new();
    game.add_mana_actions(PlayerId::One, &mut mana);
    let funding = mana.into_iter().find(|action| matches!(action, Action::ActivateManaAbility { source, .. } if source.0 == 10_001)).unwrap();
    let mut payment = ExplicitPayment {
        funding: vec![ExplicitFundingStep {
            action: funding,
            mana_units: None,
            answers: vec![],
        }],
        mana_units: Some(vec![0, 0]),
        ..ExplicitPayment::default()
    };
    let before = game.observe_state(PlayerId::One);
    let events = game.events.clone();
    assert!(
        game.apply_with_payment(PlayerId::One, sol_ring_proposal(), &payment)
            .is_err()
    );
    assert_eq!(game.observe_state(PlayerId::One), before);
    assert_eq!(game.events, events);
    assert_eq!(
        game.explain_proposal_rejection(PlayerId::One, &sol_ring_proposal(), Some(&payment))["code"],
        "invalidManaAllocation"
    );
    assert!(
        game.apply_with_payment(
            PlayerId::Two,
            sol_ring_proposal(),
            &ExplicitPayment::default()
        )
        .is_err()
    );
    payment.mana_units = None;
    game.apply_with_payment(PlayerId::One, sol_ring_proposal(), &payment)
        .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id.0 == 10_001)
            .unwrap()
            .tapped
    );
    assert!(
        !game
            .battlefield
            .iter()
            .find(|p| p.card.id.0 == 10_002)
            .unwrap()
            .tapped
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn direct_actions_reject_opponents_funding_and_preserve_priority() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::ISLAND, PlayerId::Two));
    let mut mana = Vec::new();
    game.add_mana_actions(PlayerId::Two, &mut mana);
    let funding = mana.remove(0);
    assert!(!game.is_legal_action(PlayerId::One, &funding));
    let before = game.observe_state(PlayerId::One);
    let payment = ExplicitPayment {
        funding: vec![ExplicitFundingStep {
            action: funding,
            mana_units: None,
            answers: vec![],
        }],
        ..ExplicitPayment::default()
    };
    assert!(
        game.apply_with_payment(PlayerId::One, sol_ring_proposal(), &payment)
            .is_err()
    );
    assert_eq!(game.observe_state(PlayerId::One), before);
    assert!(game.is_legal_action(PlayerId::One, &Action::PassPriority));
    assert!(!game.is_legal_action(PlayerId::Two, &Action::PassPriority));
    assert!(game.observe_state(PlayerId::One).legal_actions.is_empty());
    assert_eq!(game.forced_action_without_search(), None);
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    assert!(game.has_action_priority(PlayerId::Two));
}

#[test]
fn direct_actions_validate_casts_without_accepting_unannounced_allocations() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::LIGHTNING_BOLT, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let proposal = |selection| Action::CastSpell {
        card: GameObjectId(10_000),
        choices: CastChoices::default().with_targets(vec![selection]),
        sacrifices: vec![],
    };
    let target = Target::Player(PlayerId::Two);
    assert!(!game.is_legal_action(
        PlayerId::One,
        &proposal(TargetSelection::new(TargetSlotId(0), vec![target, target]))
    ));
    assert!(!game.is_legal_action(
        PlayerId::One,
        &proposal(TargetSelection::divided(
            TargetSlotId(0),
            vec![target],
            vec![3]
        ))
    ));
    let action = proposal(TargetSelection::single(TargetSlotId(0), target));
    assert!(game.is_legal_action(PlayerId::One, &action));
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.stack.len(), 1);
}

fn assert_enumerated_actions_validate(game: &Game, player: PlayerId) -> Vec<Action> {
    let actions = game.legal_actions(player);
    for action in &actions {
        if matches!(action, Action::ChooseDecision { .. }) {
            continue; // A bounded decision marker is a schema, not a chosen answer.
        }
        assert!(
            game.is_legal_action(player, action),
            "enumerated action rejected: {action:?}"
        );
    }
    let casts = actions
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { .. }))
        .collect::<Vec<_>>();
    assert!(!casts.is_empty(), "fixture must exercise casts");
    casts
}

#[test]
fn direct_actions_preserve_enumerated_jeskai_modal_and_divided_casts() {
    for definition in [
        cards::JESKAI_REVELATION,
        cards::FIERY_CONFLUENCE,
        cards::FIRE_COVENANT,
    ] {
        let mut game = ready_game();
        game.players[0]
            .hand
            .push(card(10_000, definition, PlayerId::One));
        game.players[0].life = 3; // Bound Covenant's life-valued X for this differential fixture.
        game.battlefield
            .push(creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One));
        game.battlefield
            .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));
        game.battlefield
            .push(creature(10_003, cards::SOL_RING, PlayerId::Two));
        for color in ManaColor::ALL {
            game.add_unrestricted_mana(PlayerId::One, color, 4);
        }
        for action in assert_enumerated_actions_validate(&game, PlayerId::One) {
            let mut submitted = game.clone();
            submitted.apply(PlayerId::One, action.clone()).unwrap();
            let mut explicit = game.clone();
            explicit
                .apply_with_payment(PlayerId::One, action, &ExplicitPayment::default())
                .unwrap();
            assert_eq!(
                explicit.observe_state(PlayerId::One).stack,
                submitted.observe_state(PlayerId::One).stack
            );
            assert_eq!(explicit.players[0].life, submitted.players[0].life);
        }
    }
}

#[test]
fn direct_actions_preserve_flashback_and_airbend_permission_costs() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(10_000, cards::LINGERING_SOULS, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    for action in assert_enumerated_actions_validate(&game, PlayerId::One) {
        let mut cast = game.clone();
        cast.apply_with_payment(PlayerId::One, action.clone(), &ExplicitPayment::default())
            .unwrap();
        assert!(cast.players[0].graveyard.is_empty());
        let mut wrong_timing = game.clone();
        wrong_timing.step = Step::Upkeep;
        assert!(!wrong_timing.is_legal_action(PlayerId::One, &action));
        let before = wrong_timing.observe_state(PlayerId::One);
        assert!(
            wrong_timing
                .apply_with_payment(PlayerId::One, action, &ExplicitPayment::default())
                .is_err()
        );
        assert_eq!(wrong_timing.observe_state(PlayerId::One), before);
    }

    let mut game = ready_game();
    game.turns_started = [3, 3];
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::AIRBENDER_ASCENSION)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[1].exile.len(), 1);
    let exiled = game.players[1].exile[0].id;
    let permission = game.action_play_permission(PlayerId::Two, exiled).unwrap();
    assert_eq!(permission["sourceZone"], "exile");
    assert_eq!(permission["cost"]["kind"], "alternativeMana");
    assert_eq!(permission["cost"]["manaCost"], "{2}");
    assert!(game.action_play_permission(PlayerId::One, exiled).is_none());
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 2);
    for action in assert_enumerated_actions_validate(&game, PlayerId::Two) {
        assert!(!game.is_legal_action(PlayerId::One, &action));
        let mut cast = game.clone();
        cast.apply_with_payment(PlayerId::Two, action, &ExplicitPayment::default())
            .unwrap();
        assert!(cast.players[1].exile.is_empty());
        assert!(cast.action_play_permission(PlayerId::Two, exiled).is_none());
        assert_eq!(cast.players[1].mana_pool.total(), 0);
    }
}

#[test]
fn direct_actions_offered_casts_preserve_other_offers_and_reject_extra_answers() {
    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_000, cards::LIGHTNING_BOLT, PlayerId::One));
    game.players[0]
        .library
        .push(card(10_001, cards::LIGHTNING_BOLT, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    let source = spell(10_003, cards::CHANDRA_TORCH_OF_DEFIANCE, PlayerId::One, 0);
    for _ in 0..2 {
        game.exile_top_and_offer_cast(
            PlayerId::One,
            &source,
            TriggerContext::empty().into(),
            ScopedEffect::primary(EffectDef::None),
        );
    }
    assert_eq!(game.pending_decisions.len(), 2);
    let offered = game.pending_decisions[1]
        .continuation
        .cast_offer()
        .unwrap()
        .card;
    let action = assert_enumerated_actions_validate(&game, PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == offered))
        .unwrap();
    let before = game.observe_state(PlayerId::One);
    let pending_ids = game
        .pending_decisions
        .iter()
        .map(|pending| pending.observation.id)
        .collect::<Vec<_>>();
    let invalid = ExplicitPayment {
        announcements: vec![vec![0]],
        ..ExplicitPayment::default()
    };
    assert!(
        game.apply_with_payment(PlayerId::One, action.clone(), &invalid)
            .is_err()
    );
    assert_eq!(game.observe_state(PlayerId::One), before);
    assert_eq!(
        game.pending_decisions
            .iter()
            .map(|pending| pending.observation.id)
            .collect::<Vec<_>>(),
        pending_ids
    );
    game.apply_with_payment(PlayerId::One, action, &ExplicitPayment::default())
        .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.pending_decisions.len(), 1);
    assert_eq!(game.pending_decisions[0].observation.id, pending_ids[0]);
    assert_ne!(
        game.pending_decisions[0]
            .continuation
            .cast_offer()
            .unwrap()
            .card,
        offered
    );
}

#[test]
fn direct_actions_reject_restricted_pool_and_stale_priority_atomically() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana = vec![Mana {
        color: ManaColor::Blue,
        source: None,
        restrictions: &[ManaRestrictionDef::CastYourCommander],
        spend_effects: &[],
    }];
    let before = game.observe_state(PlayerId::One);
    let checkpoint = game.checkpoint_json(PlayerId::One);
    assert!(
        game.apply_with_payment(
            PlayerId::One,
            sol_ring_proposal(),
            &ExplicitPayment::default()
        )
        .is_err()
    );
    assert_eq!(game.observe_state(PlayerId::One), before);
    assert_eq!(game.checkpoint_json(PlayerId::One), checkpoint);
    game.players[0].mana.clear();
    assert!(game.is_legal_action(PlayerId::One, &sol_ring_proposal()));
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let before = game.observe_state(PlayerId::One);
    assert!(
        game.apply_with_payment(
            PlayerId::One,
            sol_ring_proposal(),
            &ExplicitPayment::default()
        )
        .is_err()
    );
    assert_eq!(game.observe_state(PlayerId::One), before);
}

#[test]
fn direct_actions_cannot_answer_an_opponents_announcement_decision() {
    static ABILITY: AbilityDef = AbilityDef::activated_with_targets(
        "{1}: An opponent chooses a target player. That player gains 1 life.",
        &[CostDef::Mana(mana_cost!("{1}"))],
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any))
                .chosen_by_opponent(),
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    );
    let (mut game, id) = super::cost_lists::game_with_cost_rules(
        &CardRules::new_artifact(mana_cost!("{1}")).with_ability(ABILITY),
    );
    game.prepared_engine = PreparedEngine::compile(&game.catalog);
    let held = game.players[0].hand.remove(0);
    game.battlefield
        .push(creature(id.0, held.definition, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { .. }))
        .unwrap();
    let (preview, frame) = game.prepare_payment(PlayerId::One, &action, &[]).unwrap();
    assert!(frame.is_none());
    let pending = &preview.pending_decisions[0];
    assert_eq!(pending.observation.player, PlayerId::Two);
    let payment = ExplicitPayment {
        announcements: vec![vec![pending.observation.options[0].id]],
        ..ExplicitPayment::default()
    };
    let before = game.observe_state(PlayerId::One);
    assert!(
        game.apply_with_payment(PlayerId::One, action.clone(), &payment)
            .is_err()
    );
    assert_eq!(game.observe_state(PlayerId::One), before);
    // The normal staged path still opens the actual opponent's decision.
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
}

#[test]
fn direct_actions_split_second_blocks_priority_casts_but_preserves_resolution_offers() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::LIGHTNING_BOLT, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    let action = Action::CastSpell {
        card: GameObjectId(10_000),
        choices: CastChoices::default().with_targets(vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )]),
        sacrifices: vec![],
    };
    assert!(game.is_legal_action(PlayerId::One, &action));
    game.stack.push(spell(
        10_001,
        cards::LEGOLASS_QUICK_REFLEXES,
        PlayerId::Two,
        0,
    ));
    assert!(game.split_second_is_active());
    assert!(game.has_action_priority(PlayerId::One));
    assert!(!game.legal_actions(PlayerId::One).contains(&action));
    assert!(!game.is_legal_action(PlayerId::One, &action));
    let checkpoint = game.checkpoint_json(PlayerId::One);
    let before = game.observe_state(PlayerId::One);
    assert!(game.apply(PlayerId::One, action.clone()).is_err());
    assert!(
        game.apply_with_payment(PlayerId::One, action, &ExplicitPayment::default())
            .is_err()
    );
    assert_eq!(game.observe_state(PlayerId::One), before);
    assert_eq!(game.checkpoint_json(PlayerId::One), checkpoint);
    assert!(game.is_legal_action(PlayerId::One, &Action::PassPriority));

    // Resolving cast instructions use their existing offer authority rather
    // than ordinary priority casting, even with split second on the stack.
    game.players[0]
        .library
        .push(card(10_002, cards::LIGHTNING_BOLT, PlayerId::One));
    let source = spell(10_003, cards::CHANDRA_TORCH_OF_DEFIANCE, PlayerId::One, 0);
    game.exile_top_and_offer_cast(
        PlayerId::One,
        &source,
        TriggerContext::empty().into(),
        ScopedEffect::primary(EffectDef::None),
    );
    let offered = game.pending_decisions[0]
        .continuation
        .cast_offer()
        .unwrap()
        .card;
    let action = assert_enumerated_actions_validate(&game, PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == offered))
        .unwrap();
    let mut implicit = game.clone();
    implicit.apply(PlayerId::One, action.clone()).unwrap();
    game.apply_with_payment(PlayerId::One, action, &ExplicitPayment::default())
        .unwrap();
    assert_eq!(game.stack.len(), 2);
    assert!(game.pending_decisions.is_empty());
}
