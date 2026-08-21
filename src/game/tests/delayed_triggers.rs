use super::*;

/// Answers every waiting decision by taking what is offered and otherwise
/// passing, until the stack and the trigger queue are empty.
pub(super) fn drain_pending(game: &mut Game) {
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
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                // At least one where anything may be taken, but never more
                // than the decision allows: a pure look permits nothing at
                // all, and offers its one option only to show what was seen.
                .take(decision.minimum.max(1).min(decision.maximum))
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Rule 603.4 checks an intervening-if twice. Shadowborn Demon is the pair of
/// checks in one card: a full graveyard means it never triggers, and a
/// graveyard filled after it triggers means the ability resolves for nothing.
#[test]
fn an_intervening_if_is_checked_when_it_triggers_and_again_when_it_resolves() {
    let graveyard = |game: &mut Game, creatures: usize| {
        game.players[0].graveyard = (0..creatures)
            .map(|index| {
                card(
                    11_000 + u32::try_from(index).expect("small index"),
                    cards::SAVANNAH_LIONS,
                    PlayerId::One,
                )
            })
            .collect();
    };
    let upkeep_with = |creatures: usize| {
        let mut game = ready_game();
        game.battlefield.clear();
        game.battlefield
            .push(creature(10_000, cards::SHADOWBORN_DEMON, PlayerId::One));
        game.battlefield
            .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
        graveyard(&mut game, creatures);
        game.turn = 2;
        game.step = Step::Upkeep;
        game.handle_upkeep_triggers();
        game
    };

    // Five is fewer than six, so the Demon is hungry.
    let mut hungry = upkeep_with(5);
    assert!(
        !hungry.pending_triggers.is_empty() || !hungry.stack.is_empty(),
        "the condition held, so the ability triggered"
    );

    // Six is not fewer than six, so it never triggers at all.
    let fed = upkeep_with(6);
    assert!(
        fed.pending_triggers.is_empty() && fed.stack.is_empty(),
        "the condition failed, so nothing triggered"
    );

    // Filling the graveyard after the trigger makes it resolve for nothing.
    let mut interrupted = upkeep_with(5);
    graveyard(&mut interrupted, 6);
    drain_pending(&mut interrupted);
    assert_eq!(
        interrupted.battlefield.len(),
        2,
        "the second check failed, so nothing was sacrificed"
    );

    // Left alone, the Demon eats. Which creature it takes is its controller's
    // choice, and the Demon itself is a legal one.
    drain_pending(&mut hungry);
    assert_eq!(
        hungry.battlefield.len(),
        1,
        "both checks held, so a creature was sacrificed"
    );
}

pub(super) fn installing_object(
    id: u32,
    controller: PlayerId,
    target_defs: Vec<AbilityTargetDef>,
    targets: Vec<TargetSelection>,
    x: u16,
) -> StackObject {
    let mut object = spell(id, cards::LIGHTNING_BOLT, controller, x);
    object.kind = StackObjectKind::ActivatedAbility;
    object.source = Some(GameObjectId(id.saturating_add(10_000)));
    object.signature = None;
    object.ability = Some(StackAbilityPayload {
        origin: primary_ability(cards::LIGHTNING_BOLT),
        definition: None,
        presentation: ObjectCharacteristics::card(cards::LIGHTNING_BOLT, CardPartId::PRIMARY),
        text: Some("Install a triggered ability."),
        target_defs,
        targets,
        context: EffectResolutionContext::empty(),
        resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::None)),
        condition: None,
        mode_effects: Vec::new(),
        resolution_destination: None,
        x,
    });
    object
}

#[test]
fn one_shot_installed_triggers_use_apnap_and_the_stack_and_are_consumed_on_match() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    static END_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the end step, you lose 1 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        LOSE_ONE,
    );
    const INSTALL: EffectDef =
        EffectDef::InstallTrigger(crate::InstalledTriggerDef::once(&END_TRIGGER));

    let mut game = ready_game();
    let first = installing_object(10_000, PlayerId::One, Vec::new(), Vec::new(), 0);
    let second = installing_object(10_001, PlayerId::Two, Vec::new(), Vec::new(), 0);
    game.resolve_effect_def(
        ScopedEffect::primary(INSTALL),
        &first,
        TriggerContext::empty(),
    );
    game.resolve_effect_def(
        ScopedEffect::primary(INSTALL),
        &second,
        TriggerContext::empty(),
    );

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    assert!(game.installed_triggers.is_empty(), "matching consumes Once");
    assert_eq!(game.pending_triggers.len(), 2);
    assert_eq!(game.players[0].life, 20, "nothing resolves at the boundary");
    game.finish_rules_procedure();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        game.stack
            .iter()
            .map(|object| object.controller)
            .collect::<Vec<_>>(),
        vec![PlayerId::One, PlayerId::Two],
        "the active player's trigger is below the nonactive player's trigger",
    );
    let countered = game
        .stack
        .iter()
        .map(|object| object.id)
        .collect::<Vec<_>>();
    for object in countered {
        game.counter_spell(object);
    }
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    assert!(
        game.stack.is_empty(),
        "countering does not restore the listener"
    );
    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[1].life, 20);
}

#[test]
fn an_installed_trigger_cannot_observe_the_event_whose_listener_snapshot_predates_it() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    static END_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the end step, you lose 1 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        LOSE_ONE,
    );
    const INSTALL: EffectDef =
        EffectDef::InstallTrigger(crate::InstalledTriggerDef::once(&END_TRIGGER));

    let mut game = ready_game();
    let listeners = game.battlefield_trigger_listeners();
    let object = installing_object(10_000, PlayerId::One, Vec::new(), Vec::new(), 0);
    game.resolve_effect_def(
        ScopedEffect::primary(INSTALL),
        &object,
        TriggerContext::empty(),
    );
    let event = CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    };
    game.capture_battlefield_triggers_from_snapshot(&listeners, &event);
    assert!(game.pending_triggers.is_empty());
    assert_eq!(game.installed_triggers.len(), 1);

    game.capture_battlefield_triggers(&event);
    assert_eq!(game.pending_triggers.len(), 1);
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn installed_trigger_retains_lexical_bindings_targets_and_target_scope() {
    static TARGETS: [AbilityTargetDef; 2] = [
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
    ];
    const TAP_BOUND: EffectDef = EffectDef::Tap {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    };
    const LOSE_TARGET_TWO: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
    };
    const LOSE_EVENT_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::EventPlayer,
        amount: ValueDef::Constant(1),
    };
    static EFFECTS: [EffectDef; 3] = [TAP_BOUND, LOSE_TARGET_TWO, LOSE_EVENT_ONE];
    static END_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the end step, use the installing effect's context.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::Sequence(&EFFECTS),
    );
    const INSTALL: EffectDef =
        EffectDef::InstallTrigger(crate::InstalledTriggerDef::once(&END_TRIGGER));

    let mut game = ready_game();
    let bound = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let bound_id = bound.card.id;
    game.battlefield.push(bound);
    let object = installing_object(
        10_001,
        PlayerId::One,
        TARGETS.to_vec(),
        vec![
            TargetSelection::single(TargetSlotId(0), Target::Player(PlayerId::One)),
            TargetSelection::single(TargetSlotId(1), Target::Player(PlayerId::Two)),
        ],
        7,
    );
    let mut context = EffectResolutionContext::new(TriggerContext {
        object: None,
        object_controller: None,
        event_player: Some(PlayerId::One),
        amount: Some(99),
    });
    context.bind_single_object(
        ObjectBindingIndex::PRIMARY,
        Some(Target::Permanent(bound_id)),
    );
    game.resolve_effect_def(
        ScopedEffect {
            effect: INSTALL,
            target_base: 1,
        },
        &object,
        context,
    );

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::Two,
    });
    game.finish_rules_procedure();
    let payload = game.stack[0].ability.as_ref().expect("trigger payload");
    assert!(
        payload.target_defs.is_empty(),
        "installer selections are lexical references, not the delayed ability's targets",
    );
    assert_eq!(payload.targets.len(), 2);
    assert_eq!(payload.x, 7);
    drain_pending(&mut game);

    assert!(
        game.battlefield[0].tapped,
        "the object binding was retained"
    );
    assert_eq!(
        game.players[0].life, 20,
        "the old event player was replaced"
    );
    assert_eq!(
        game.players[1].life, 17,
        "target base 1 and the matching event player were both retained",
    );
}

#[test]
fn until_next_turn_listener_survives_an_extra_turn_and_expires_for_jaces_controller() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::EventPlayer,
        amount: ValueDef::Constant(1),
    };
    static UPKEEP_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of each opponent's upkeep, that player loses 1 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Opponent,
        },
        LOSE_ONE,
    );
    const INSTALL: EffectDef =
        EffectDef::InstallTrigger(crate::InstalledTriggerDef::until_next_turn(
            &UPKEEP_TRIGGER,
            PlayerRefDef::EffectController,
        ));

    let mut game = ready_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.next_regular_player = PlayerId::One;
    let object = installing_object(10_000, PlayerId::One, Vec::new(), Vec::new(), 0);
    game.resolve_effect_def(
        ScopedEffect::primary(INSTALL),
        &object,
        TriggerContext::empty(),
    );
    game.extra_turns.push(PlayerId::Two);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.installed_triggers.len(), 1);
    drain_pending(&mut game);
    assert_eq!(game.players[1].life, 19, "the extra-turn upkeep matched");

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert!(game.installed_triggers.is_empty());
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].life, 20,
        "Jace's next turn begins after expiry"
    );
}

#[test]
fn stacked_quickens_are_all_spent_by_the_same_next_sorcery() {
    let mut game = ready_game();
    let quickens = [
        card(10_000, cards::QUICKEN, PlayerId::One),
        card(10_001, cards::QUICKEN, PlayerId::One),
    ];
    let sorceries = [
        card(10_002, cards::MIND_TWIST, PlayerId::One),
        card(10_003, cards::MIND_TWIST, PlayerId::One),
    ];
    game.players[0].hand.extend(quickens.iter().cloned());
    game.players[0].hand.extend(sorceries.iter().cloned());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.black = 4;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;

    for quicken in &quickens {
        game.apply(
            PlayerId::One,
            cast_action(quicken.id, Vec::new(), Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);
        game.priority = PlayerId::One;
    }
    assert_eq!(game.sorcery_flash_grants[0], 2);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == sorceries[0].id))
        .expect("both Quicken grants cover the same next sorcery");
    game.apply(PlayerId::One, cast).unwrap();
    game.priority = PlayerId::One;

    assert_eq!(game.sorcery_flash_grants[0], 0);
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == sorceries[1].id)
        ),
        "the second sorcery needs a new timing permission"
    );
}

#[test]
fn quicken_consumes_its_grant_for_the_selected_sorcery_part() {
    let definition_id = CardDefinitionId::new(10_068);
    let instant = CardRules::new_instant(ManaCost::default());
    let sorcery = CardRules::new_sorcery(ManaCost::default());
    let (mut game, _, _) = game_with_test_fused_split(definition_id, &instant, &sorcery);
    let split = card(10_000, definition_id, PlayerId::One);
    let next_sorcery = card(10_001, cards::MIND_TWIST, PlayerId::One);
    game.players[0]
        .hand
        .extend([split.clone(), next_sorcery.clone()]);
    game.players[0].mana_pool.black = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.sorcery_flash_grants[0] = 1;

    let cast_second_part = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == split.id && choices.play_option() == PlayOptionId(1)
            )
        })
        .expect("Quicken makes the selected sorcery part castable now");
    game.apply(PlayerId::One, cast_second_part).unwrap();
    game.priority = PlayerId::One;

    assert_eq!(game.sorcery_flash_grants[0], 0);
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == next_sorcery.id)
        ),
        "consumption follows the selected part rather than the primary instant characteristics"
    );
}

#[test]
fn quicken_preserves_its_grant_for_the_selected_instant_part() {
    let definition_id = CardDefinitionId::new(10_069);
    let sorcery = CardRules::new_sorcery(ManaCost::default());
    let instant = CardRules::new_instant(ManaCost::default());
    let (mut game, _, _) = game_with_test_fused_split(definition_id, &sorcery, &instant);
    let split = card(10_000, definition_id, PlayerId::One);
    let next_sorcery = card(10_001, cards::MIND_TWIST, PlayerId::One);
    game.players[0]
        .hand
        .extend([split.clone(), next_sorcery.clone()]);
    game.players[0].mana_pool.black = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.sorcery_flash_grants[0] = 1;

    let cast_second_part = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == split.id && choices.play_option() == PlayOptionId(1)
            )
        })
        .expect("the selected instant part is castable without using Quicken");
    game.apply(PlayerId::One, cast_second_part).unwrap();
    game.priority = PlayerId::One;

    assert_eq!(game.sorcery_flash_grants[0], 1);
    assert!(
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == next_sorcery.id)
        ),
        "the grant remains available for the next sorcery"
    );
}

#[test]
fn mutavault_becomes_a_creature_of_every_type_until_cleanup() {
    let mut game = ready_game();
    game.battlefield.clear();
    let vault = game
        .put_onto_battlefield(PlayerId::One, cards::MUTAVAULT)
        .expect("cataloged");
    // Something to pay the activation with that is not the Mutavault itself.
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");

    let land = game.battlefield[0].clone();
    assert!(
        !game
            .permanent_types(&land)
            .expect("a battlefield permanent has types")
            .contains(CardType::Creature),
        "a Mutavault is only a land until it is animated"
    );

    let activate = game
        .observe(PlayerId::One)
        .legal_actions
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == vault))
        .expect("the animation ability is offered");
    game.apply(PlayerId::One, activate).unwrap();
    drain_pending(&mut game);

    let animated = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vault)
        .expect("the Mutavault is still on the battlefield")
        .clone();
    let types = game.permanent_types(&animated).expect("types");
    assert!(types.contains(CardType::Creature), "it became a creature");
    assert!(types.contains(CardType::Land), "and it is still a land");
    assert_eq!(
        game.base_stats(&animated),
        Some(crate::CreatureStats {
            power: 2,
            toughness: 2
        })
    );
    let subtypes = game.effective_subtypes(&animated);
    for creature_type in ["Goblin", "Angel", "Assembly-Worker"] {
        assert!(
            subtypes.contains(&creature_type),
            "all creature types includes {creature_type}"
        );
    }

    game.cleanup();
    let after = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vault)
        .expect("still there");
    assert!(
        !game
            .permanent_types(after)
            .expect("types")
            .contains(CardType::Creature),
        "the animation lasts only until end of turn"
    );
}

#[test]
fn ghost_quarter_destroys_a_land_and_lets_its_owner_replace_it() {
    let mut game = ready_game();
    game.battlefield.clear();
    let quarter = game
        .put_onto_battlefield(PlayerId::One, cards::GHOST_QUARTER)
        .expect("cataloged");
    let victim = game
        .put_onto_battlefield(PlayerId::Two, cards::TROPICAL_ISLAND)
        .expect("cataloged");
    // A basic to find and a nonbasic that the search may not take.
    game.players[1].library = vec![
        card(10_050, cards::SAVANNAH_LIONS, PlayerId::Two),
        card(10_051, cards::FOREST, PlayerId::Two),
    ];

    let activate = game
        .observe(PlayerId::One)
        .legal_actions
        .into_iter()
        .find(|action| match action {
            // The Quarter is a legal target for itself at announcement, so
            // pick the one aimed at the opponent's land.
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == quarter
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(victim)))
            }
            _ => false,
        })
        .expect("the sacrifice ability is offered");
    game.apply(PlayerId::One, activate).unwrap();
    while !game.stack.is_empty() && game.pending_decisions.is_empty() {
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == victim),
        "the targeted land was destroyed"
    );

    // The search belongs to the land's controller, not the Quarter's.
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the destroyed land's controller searches");
    assert_eq!(decision.player, PlayerId::Two);
    let accept = decision
        .options
        .iter()
        .find(|option| option.label == "Do it")
        .expect("the land's controller may accept the search")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![accept],
        },
    )
    .unwrap();

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("accepting offers the qualified library search");
    let offered = decision
        .options
        .iter()
        .filter_map(|option| option.card)
        .filter_map(|(_, characteristics)| characteristics.card_definition())
        .collect::<Vec<_>>();
    assert_eq!(
        offered,
        vec![cards::FOREST],
        "only a basic land card is a legal find"
    );

    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::FOREST
                && permanent.controller == PlayerId::Two),
        "the basic land arrived under its owner's control"
    );
    assert!(game.players[1].library.len() == 1, "and left the library");
}

#[test]
fn ghost_quarters_controller_may_decline_without_searching_or_shuffling() {
    let mut game = ready_game();
    game.battlefield.clear();
    let quarter = game
        .put_onto_battlefield(PlayerId::One, cards::GHOST_QUARTER)
        .expect("cataloged");
    let victim = game
        .put_onto_battlefield(PlayerId::Two, cards::TROPICAL_ISLAND)
        .expect("cataloged");
    game.players[1].library = (10_060..10_068)
        .map(|id| card(id, cards::FOREST, PlayerId::Two))
        .collect();
    let before = game.players[1]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();

    let activate = game
        .observe(PlayerId::One)
        .legal_actions
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == quarter
                        && targets.iter().any(|selection| {
                            selection.targets().contains(&Target::Permanent(victim))
                        })
            )
        })
        .expect("the sacrifice ability is offered");
    game.apply(PlayerId::One, activate).unwrap();
    while !game.stack.is_empty() && game.pending_decisions.is_empty() {
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }

    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let decline = decision
        .options
        .iter()
        .find(|option| option.label == "Decline")
        .expect("the printed may can be declined")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decline],
        },
    )
    .unwrap();

    assert!(game.pending_decisions.is_empty());
    assert_eq!(
        game.players[1]
            .library
            .iter()
            .map(|card| card.id)
            .collect::<Vec<_>>(),
        before,
        "declining skips the entire search-and-shuffle procedure"
    );
}

#[test]
fn a_creature_that_attacks_each_combat_holds_the_declaration_open() {
    let mut game = ready_game();
    game.battlefield.clear();
    let ruric = game
        .put_onto_battlefield(PlayerId::One, cards::RURIC_THAR_THE_UNBOWED)
        .expect("cataloged");
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    let actions = game.legal_actions(PlayerId::One);
    assert!(
        !actions.contains(&Action::FinishDeclaringAttackers),
        "the declaration cannot be finished while Ruric Thar could still attack"
    );
    assert!(
        actions.contains(&Action::DeclareAttacker {
            attacker: lions,
            defender: AttackDefender::Player(PlayerId::Two)
        }),
        "another creature may still be declared first"
    );

    // Declaring the free attacker does not satisfy the requirement.
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: lions,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers),
        "only Ruric Thar attacking releases the declaration"
    );

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: ruric,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers),
        "once it is attacking the requirement is met"
    );
}

#[test]
fn an_attack_requirement_yields_when_the_creature_cannot_attack() {
    let mut game = ready_game();
    game.battlefield.clear();
    let ruric = game
        .put_onto_battlefield(PlayerId::One, cards::RURIC_THAR_THE_UNBOWED)
        .expect("cataloged");
    // Summoning sick, so it is not able and the requirement does not apply.
    game.turns_started = [1, 1];
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers),
        "a creature that cannot attack is not required to"
    );

    // The same creature, able but tapped by something else.
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.tap_permanent(ruric);
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers),
        "a tapped creature cannot attack, so it is not held against its controller"
    );
}
