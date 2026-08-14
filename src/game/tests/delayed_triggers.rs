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
                .take(decision.minimum.max(1))
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

#[test]
fn delayed_trigger_partition_preserves_order_and_waiting_capacity() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    const LOSE_TWO: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    };
    const LOSE_THREE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    };
    const LOSE_FOUR: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(4),
    };
    let delayed = |id: u32, step: TurnStepDef, effect: EffectDef| DelayedTrigger {
        object: Box::new(spell(id, cards::LIGHTNING_BOLT, PlayerId::One, 0)),
        context: TriggerContext::empty(),
        step,
        player: PlayerRelation::Any,
        effect: ScopedEffect::primary(effect),
    };

    let mut game = ready_game();
    game.delayed_triggers = Vec::with_capacity(8);
    game.delayed_triggers.extend([
        delayed(10_000, TurnStepDef::End, LOSE_ONE),
        delayed(10_001, TurnStepDef::Draw, LOSE_THREE),
        delayed(10_002, TurnStepDef::End, LOSE_TWO),
        delayed(10_003, TurnStepDef::Draw, LOSE_FOUR),
    ]);
    let waiting_capacity = game.delayed_triggers.capacity();
    let event_start = game.events.len();

    game.fire_delayed_triggers(TurnStepDef::End);

    let lost = game.events[event_start..]
        .iter()
        .filter_map(|event| match event {
            GameEvent::LifeLost {
                player: PlayerId::One,
                amount,
            } => Some(*amount),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(lost, vec![1, 2], "due effects keep their queued order");
    assert_eq!(
        game.delayed_triggers
            .iter()
            .map(|delayed| delayed.object.id.0)
            .collect::<Vec<_>>(),
        vec![10_001, 10_003],
        "waiting effects keep their queued order"
    );
    assert_eq!(
        game.delayed_triggers.capacity(),
        waiting_capacity,
        "partitioning reuses the waiting queue allocation"
    );
}

#[test]
fn delayed_effect_preserves_its_trigger_context() {
    static TAP_TRIGGERING_OBJECT: EffectDef = EffectDef::Tap {
        object: EffectRecipientDef::TriggeringObject,
    };
    static LOSE_TRIGGER_AMOUNT: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::EventPlayer,
        amount: ValueDef::TriggerEventAmount,
    };
    static DELAYED_EFFECTS: [EffectDef; 2] = [TAP_TRIGGERING_OBJECT, LOSE_TRIGGER_AMOUNT];
    static DELAYED: EffectDef = EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::EventPlayer,
        effect: &EffectDef::Sequence(&DELAYED_EFFECTS),
    };

    let mut game = ready_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let triggering = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two);
    let triggering_id = triggering.card.id;
    game.battlefield.push(triggering);
    let source = spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    let context = TriggerContext {
        object: Some(triggering_id),
        chosen_objects: [None; 8],
        object_controller: Some(PlayerId::Two),
        event_player: Some(PlayerId::Two),
        amount: Some(3),
        source_attachment: None,
        source_linked: None,
    };
    let life_before = game.players[PlayerId::Two.index()].life;

    game.resolve_effect_def(ScopedEffect::primary(DELAYED), &source, context);

    assert_eq!(game.delayed_triggers.len(), 1);
    assert!(!game.battlefield[0].tapped);
    assert_eq!(game.players[PlayerId::Two.index()].life, life_before);

    game.fire_delayed_triggers(TurnStepDef::End);

    assert!(game.delayed_triggers.is_empty());
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[PlayerId::Two.index()].life, life_before - 3);
}

#[test]
fn delayed_effect_enqueued_during_firing_waits_for_the_next_matching_step() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    const ENQUEUE_LOSS: EffectDef = EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
        effect: &LOSE_ONE,
    };
    let mut game = ready_game();
    game.delayed_triggers = Vec::with_capacity(4);
    game.delayed_triggers.push(DelayedTrigger {
        object: Box::new(spell(10_000, cards::LIGHTNING_BOLT, PlayerId::One, 0)),
        context: TriggerContext::empty(),
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
        effect: ScopedEffect::primary(ENQUEUE_LOSS),
    });
    let waiting_capacity = game.delayed_triggers.capacity();
    let life_before = game.players[0].life;

    game.fire_delayed_triggers(TurnStepDef::End);

    assert_eq!(game.players[0].life, life_before);
    assert_eq!(game.delayed_triggers.len(), 1);
    assert_eq!(game.delayed_triggers[0].effect.effect, LOSE_ONE);
    assert_eq!(game.delayed_triggers.capacity(), waiting_capacity);

    game.fire_delayed_triggers(TurnStepDef::End);

    assert_eq!(game.players[0].life, life_before - 1);
    assert!(game.delayed_triggers.is_empty());
    assert_eq!(game.delayed_triggers.capacity(), waiting_capacity);
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
    let definition_id = CardDefinitionId(10_068);
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
    let definition_id = CardDefinitionId(10_069);
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
        .map(|(_, definition)| definition)
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
