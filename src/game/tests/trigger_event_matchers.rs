//! Canonical committed trigger-event matching.

use super::*;

fn add_definition(game: &mut Game, definition: CardDefinition) {
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the trigger fixture is valid");
}

fn trigger_creature_definition(
    id: CardDefinitionId,
    name: &str,
    abilities: &'static [AbilityDef],
) -> CardDefinition {
    let mut definition = CardDefinition::new(
        id,
        name,
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules =
        CardRules::new_creature(ManaCost::default(), &[], 2, 2).with_abilities(abilities);
    synchronize_single_part_definition(&mut definition);
    definition
}

#[test]
fn overlapping_damage_matchers_scan_one_committed_event_once_each() {
    static ABILITIES: [AbilityDef; 3] = [
        AbilityDef::triggered(
            "Whenever this deals damage to a player.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::players(
                    PlayerSetDef::All,
                )),
            }),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever this deals combat damage to a player.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever this is dealt damage.",
            TriggerEventDef::damage_to_source(),
            EffectDef::None,
        ),
    ];

    let definition = CardDefinitionId::new(10_300);
    let mut game = ready_game();
    add_definition(
        &mut game,
        trigger_creature_definition(definition, "Canonical damage watcher", &ABILITIES),
    );
    let watcher = creature(10_300, definition, PlayerId::One);
    let watcher_id = watcher.card.id;
    game.battlefield.push(watcher);

    game.damage_target_from_kind(
        Some(watcher_id),
        Some(Target::Player(PlayerId::Two)),
        2,
        true,
    );
    assert_eq!(game.pending_triggers.len(), 2);
    assert!(game.pending_triggers.iter().all(|trigger| {
        trigger.context.trigger.object == Some(watcher_id)
            && trigger.context.trigger.event_player == Some(PlayerId::Two)
            && trigger.context.trigger.amount == Some(2)
    }));
    assert_eq!(
        game.pending_triggers
            .iter()
            .filter(|trigger| trigger.text == "Whenever this deals damage to a player.")
            .count(),
        1,
    );
    assert_eq!(
        game.pending_triggers
            .iter()
            .filter(|trigger| trigger.text == "Whenever this deals combat damage to a player.")
            .count(),
        1,
    );

    game.pending_triggers.clear();
    game.damage_target_from(Some(watcher_id), Some(Target::Player(PlayerId::Two)), 1);
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "noncombat matches only the broad clause"
    );

    game.pending_triggers.clear();
    let dealer = creature(10_301, cards::SAVANNAH_LIONS, PlayerId::Two);
    let dealer_id = dealer.card.id;
    game.battlefield.push(dealer);
    game.damage_target_from(Some(dealer_id), Some(Target::Permanent(watcher_id)), 1);
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "incoming damage is a separate axis"
    );
    assert_eq!(
        game.pending_triggers[0].text,
        "Whenever this is dealt damage."
    );
}

#[test]
fn simultaneous_damage_coalesces_recipient_but_not_source_occurrences() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::triggered(
            "Whenever this is dealt damage.",
            TriggerEventDef::damage_to_source(),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever a creature deals damage to this.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Source),
                ..DamageEventMatcherDef::ANY
            }),
            EffectDef::None,
        ),
    ];

    let definition = CardDefinitionId::new(10_314);
    let mut game = ready_game();
    add_definition(
        &mut game,
        trigger_creature_definition(definition, "Simultaneous damage watcher", &ABILITIES),
    );
    let watcher = creature(10_314, definition, PlayerId::One);
    let watcher_id = watcher.card.id;
    let first = creature(10_315, cards::SAVANNAH_LIONS, PlayerId::Two);
    let first_id = first.card.id;
    let second = creature(10_316, cards::SAVANNAH_LIONS, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.extend([watcher, first, second]);

    game.deal_damage_simultaneously(vec![
        DamageAssignment {
            source: Some(first_id),
            target: Some(Target::Permanent(watcher_id)),
            amount: 1,
            combat: true,
        },
        DamageAssignment {
            source: Some(second_id),
            target: Some(Target::Permanent(watcher_id)),
            amount: 1,
            combat: true,
        },
    ]);

    let recipient_triggers = game
        .pending_triggers
        .iter()
        .filter(|trigger| trigger.text == "Whenever this is dealt damage.")
        .collect::<Vec<_>>();
    assert_eq!(recipient_triggers.len(), 1);
    assert_eq!(recipient_triggers[0].context.trigger.amount, Some(2));
    assert_eq!(
        game.pending_triggers
            .iter()
            .filter(|trigger| trigger.text == "Whenever a creature deals damage to this.")
            .count(),
        2,
        "the source-qualified clause sees one occurrence per damage source",
    );
}

#[test]
fn related_damage_recipients_are_relative_to_the_ability_controller() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "Whenever the watched creature deals damage to an opponent.",
        TriggerEventDef::damage_to_player(
            ObjectPredicateDef::AttachedToSource,
            PlayerRelation::Opponent,
        ),
        EffectDef::None,
    )];

    let definition = CardDefinitionId::new(10_302);
    let mut aura = CardDefinition::new(
        definition,
        "Controller-relative damage watcher",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    aura.rules = CardRules::new_enchantment(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut aura);

    let mut game = ready_game();
    add_definition(&mut game, aura);
    let dealer = creature(10_302, cards::SAVANNAH_LIONS, PlayerId::Two);
    let dealer_id = dealer.card.id;
    game.battlefield.push(dealer);
    let mut watcher = creature(10_303, definition, PlayerId::One);
    watcher.attached_to = Some(dealer_id);
    game.battlefield.push(watcher);

    game.damage_target_from(Some(dealer_id), Some(Target::Player(PlayerId::Two)), 1);
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "player two is the watcher controller's opponent even though it controls the dealer",
    );

    game.pending_triggers.clear();
    game.damage_target_from(Some(dealer_id), Some(Target::Player(PlayerId::One)), 1);
    assert!(
        game.pending_triggers.is_empty(),
        "the dealer's opponent relation must not replace the ability controller's relation",
    );
}

#[test]
fn damage_matching_uses_frozen_source_and_recipient_characteristics() {
    let mut game = ready_game();
    let watcher = creature(10_304, cards::SAVANNAH_LIONS, PlayerId::One);
    let watcher_id = watcher.card.id;
    let dealer = creature(10_305, cards::SAVANNAH_LIONS, PlayerId::One);
    let dealer_id = dealer.card.id;
    let recipient = creature(10_306, cards::SAVANNAH_LIONS, PlayerId::One);
    let recipient_id = recipient.card.id;
    game.battlefield.extend([watcher, dealer, recipient]);

    let dealer_snapshot = game.trigger_event_object(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == dealer_id)
            .expect("dealer exists"),
    );
    let recipient_snapshot = game.trigger_event_object(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == recipient_id)
            .expect("recipient exists"),
    );
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == dealer_id)
        .expect("dealer exists")
        .controller = PlayerId::Two;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == recipient_id)
        .expect("recipient exists")
        .controller = PlayerId::Two;

    let event = CommittedTriggerEvent::DamageDealt {
        source: Some(dealer_snapshot),
        source_is_spell: false,
        recipient: Target::Permanent(recipient_id),
        recipient_object: Some(recipient_snapshot),
        amount: 1,
        combat: false,
    };
    assert!(game.trigger_event_matches_for_controller(
        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Any,
            source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::ControlledBy(
                PlayerRelation::You,
            )),
            recipient: DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                PlayerRefDef::EffectController,
            ),
        }),
        &event,
        watcher_id,
        game.controller_of_object(watcher_id),
    ));
}

#[test]
fn damage_source_matching_uses_frozen_spell_status() {
    let mut game = ready_game();
    let watcher = creature(10_312, cards::SAVANNAH_LIONS, PlayerId::One);
    let watcher_id = watcher.card.id;
    let artifact = creature(10_313, cards::MANA_VAULT, PlayerId::One);
    let artifact_id = artifact.card.id;
    game.battlefield.extend([watcher, artifact]);
    let source = game.trigger_event_object(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == artifact_id)
            .expect("artifact exists"),
    );
    let matcher = |predicate| {
        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
            source: DamageSourceMatcherDef::Matching(predicate),
            ..DamageEventMatcherDef::ANY
        })
    };
    let event = CommittedTriggerEvent::DamageDealt {
        source: Some(source.clone()),
        source_is_spell: true,
        recipient: Target::Player(PlayerId::Two),
        recipient_object: None,
        amount: 3,
        combat: false,
    };
    assert!(game.trigger_event_matches_for_controller(
        matcher(ObjectPredicateDef::Spell),
        &event,
        watcher_id,
        game.controller_of_object(watcher_id),
    ));
    assert!(game.trigger_event_matches_for_controller(
        matcher(ObjectPredicateDef::NoncreatureSpell),
        &event,
        watcher_id,
        game.controller_of_object(watcher_id),
    ));
    assert!(!game.trigger_event_matches_for_controller(
        matcher(ObjectPredicateDef::Spell),
        &CommittedTriggerEvent::DamageDealt {
            source: Some(source),
            source_is_spell: false,
            recipient: Target::Player(PlayerId::Two),
            recipient_object: None,
            amount: 3,
            combat: false,
        },
        watcher_id,
        game.controller_of_object(watcher_id),
    ));
}

#[test]
fn simultaneous_deaths_use_deduped_damage_history_and_source_lki() {
    let mut game = ready_game();
    let sengir = creature(10_307, cards::SENGIR_VAMPIRE, PlayerId::One);
    let sengir_id = sengir.card.id;
    let victim = creature(10_308, cards::SAVANNAH_LIONS, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.extend([sengir, victim]);

    game.damage_target_from(Some(sengir_id), Some(Target::Permanent(victim_id)), 1);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == victim_id)
        .expect("victim remains before state-based actions")
        .damage_sources
        .push(sengir_id);
    game.move_permanents_to_graveyard(&[sengir_id, victim_id]);

    let triggers = game
        .pending_triggers
        .iter()
        .filter(|trigger| trigger.source.object == sengir_id)
        .collect::<Vec<_>>();
    assert_eq!(
        triggers.len(),
        1,
        "duplicate history cannot duplicate a death trigger"
    );
    assert_eq!(triggers[0].context.trigger.object, Some(victim_id));
    assert!(
        game.retired_objects.contains_key(&sengir_id),
        "the departed source is available through last-known information",
    );
}

#[test]
fn cleanup_preserves_damage_history_until_the_turn_actually_ends() {
    let mut game = ready_game();
    let sengir = creature(10_309, cards::SENGIR_VAMPIRE, PlayerId::One);
    let sengir_id = sengir.card.id;
    let victim = creature(10_310, cards::SAVANNAH_LIONS, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.extend([sengir, victim]);

    game.damage_target_from(Some(sengir_id), Some(Target::Permanent(victim_id)), 1);
    game.finish_cleanup();
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == victim_id)
            .expect("victim remains after cleanup")
            .damage_sources,
        vec![sengir_id],
        "a phase inserted after cleanup is still part of the same turn",
    );

    game.start_next_turn();
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == victim_id)
            .expect("victim remains at the next turn")
            .damage_sources
            .is_empty(),
        "the committed next turn, not cleanup alone, ends damage history",
    );
}

#[test]
fn attack_tap_and_transform_matchers_read_committed_event_facts() {
    let mut game = ready_game();
    let permanent = creature(10_311, cards::SAVANNAH_LIONS, PlayerId::One);
    let permanent_id = permanent.card.id;
    game.battlefield.push(permanent);
    let object = game.trigger_event_object(&game.battlefield[0]);

    let attack = CommittedTriggerEvent::Attacks {
        object: object.clone(),
        declaration_size: 3,
        attack_number: 1,
        defending_player: PlayerId::Two,
        attacked_a_planeswalker: false,
    };
    game.battlefield[0].attacks_this_turn = 2;
    assert!(game.trigger_event_matches_for_controller(
        TriggerEventDef::attacks_first_time_this_turn(ObjectPredicateDef::Source),
        &attack,
        permanent_id,
        game.controller_of_object(permanent_id),
    ));
    assert!(game.trigger_event_matches_for_controller(
        TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 3, None),
        &attack,
        permanent_id,
        game.controller_of_object(permanent_id),
    ));

    let ordinary_tap = CommittedTriggerEvent::Tapped {
        object: object.clone(),
        for_mana: false,
    };
    assert!(game.trigger_event_matches_for_controller(
        TriggerEventDef::tapped(ObjectPredicateDef::Source),
        &ordinary_tap,
        permanent_id,
        game.controller_of_object(permanent_id),
    ));
    assert!(!game.trigger_event_matches_for_controller(
        TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Source),
        &ordinary_tap,
        permanent_id,
        game.controller_of_object(permanent_id),
    ));
    let mana_tap = CommittedTriggerEvent::Tapped {
        object: object.clone(),
        for_mana: true,
    };
    assert!(game.trigger_event_matches_for_controller(
        TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Source),
        &mana_tap,
        permanent_id,
        game.controller_of_object(permanent_id),
    ));

    let transformed = CommittedTriggerEvent::Transformed { object };
    assert!(game.trigger_event_matches_for_controller(
        TriggerEventDef::transforms(ObjectPredicateDef::HasType(CardType::Creature)),
        &transformed,
        GameObjectId(99_999),
        None,
    ));
    assert!(!game.trigger_event_matches_for_controller(
        TriggerEventDef::transforms(ObjectPredicateDef::HasType(CardType::Land)),
        &transformed,
        GameObjectId(99_999),
        None,
    ));
}

#[test]
fn live_tap_event_snapshots_the_post_transition_object() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "Whenever this tapped permanent becomes tapped.",
        TriggerEventDef::tapped(ObjectPredicateDef::Tapped),
        EffectDef::None,
    )];
    let definition = CardDefinitionId::new(10_314);
    let mut game = ready_game();
    add_definition(
        &mut game,
        trigger_creature_definition(definition, "Post-transition tap watcher", &ABILITIES),
    );
    let permanent = creature(10_314, definition, PlayerId::One);
    let permanent_id = permanent.card.id;
    game.battlefield.push(permanent);

    game.tap_permanent(permanent_id);

    assert!(game.battlefield[0].tapped);
    assert_eq!(game.pending_triggers.len(), 1);
    assert_eq!(
        game.pending_triggers[0].context.trigger.object,
        Some(permanent_id)
    );
}

#[test]
fn attacker_tap_events_wait_for_the_whole_declaration() {
    static CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
        query: ObjectQueryDef::matching(
            ObjectPredicateDef::Attacking,
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 2,
    };
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_if(
        "Whenever a creature you control becomes tapped, if two creatures you control are attacking, trigger.",
        TriggerEventDef::tapped(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
        &CONDITION,
        EffectDef::None,
    )];
    let definition = CardDefinitionId::new(10_315);
    let mut game = ready_game();
    add_definition(
        &mut game,
        trigger_creature_definition(definition, "Declaration tap watcher", &ABILITIES),
    );
    let watcher = creature(10_315, definition, PlayerId::One);
    let first = creature(10_316, cards::SAVANNAH_LIONS, PlayerId::One);
    let first_id = first.card.id;
    let second = creature(10_317, cards::SAVANNAH_LIONS, PlayerId::One);
    let second_id = second.card.id;
    game.battlefield.extend([watcher, first, second]);

    game.declare_attacker(first_id, AttackDefender::Player(PlayerId::Two));
    game.declare_attacker(second_id, AttackDefender::Player(PlayerId::Two));
    assert!(
        game.pending_triggers.is_empty(),
        "tap events remain deferred during the declaration"
    );

    game.finish_declaring_attackers();

    assert_eq!(
        game.pending_triggers.len(),
        2,
        "both tap conditions read the complete two-attacker declaration"
    );
}

#[test]
fn partial_attack_and_block_declarations_do_not_run_state_triggers() {
    static ATTACKING_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
        query: ObjectQueryDef::matching(
            ObjectPredicateDef::Attacking,
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    };
    static BLOCKING_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
        query: ObjectQueryDef::matching(
            ObjectPredicateDef::Blocking,
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    };
    static ATTACK_ABILITY: [AbilityDef; 1] = [AbilityDef::triggered_if(
        "Whenever you control an attacking creature, trigger.",
        TriggerEventDef::StateCondition,
        &ATTACKING_CONDITION,
        EffectDef::None,
    )];
    static BLOCK_ABILITY: [AbilityDef; 1] = [AbilityDef::triggered_if(
        "Whenever you control a blocking creature, trigger.",
        TriggerEventDef::StateCondition,
        &BLOCKING_CONDITION,
        EffectDef::None,
    )];

    let mut attacks = ready_game();
    let attack_definition = CardDefinitionId::new(10_318);
    add_definition(
        &mut attacks,
        trigger_creature_definition(attack_definition, "Attack state watcher", &ATTACK_ABILITY),
    );
    let watcher = creature(10_318, attack_definition, PlayerId::One);
    let watcher_id = watcher.card.id;
    let attacker = creature(10_319, cards::SAVANNAH_LIONS, PlayerId::One);
    let attacker_id = attacker.card.id;
    attacks.battlefield.extend([watcher, attacker]);
    attacks.step = Step::DeclareAttackers;
    attacks.attackers_declared = false;
    attacks.turns_started[PlayerId::One.index()] = 1;
    attacks
        .apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker: attacker_id,
                defender: AttackDefender::Player(PlayerId::Two),
            },
        )
        .expect("the attacker is legal");
    assert!(!attacks.events.iter().any(
        |event| matches!(event, GameEvent::AbilityTriggered { source, .. } if *source == watcher_id)
    ));
    attacks
        .apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the completed attack declaration is legal");
    assert!(attacks.events.iter().any(
        |event| matches!(event, GameEvent::AbilityTriggered { source, .. } if *source == watcher_id)
    ));

    let mut blocks = ready_game();
    let block_definition = CardDefinitionId::new(10_320);
    add_definition(
        &mut blocks,
        trigger_creature_definition(block_definition, "Block state watcher", &BLOCK_ABILITY),
    );
    let watcher = creature(10_320, block_definition, PlayerId::Two);
    let watcher_id = watcher.card.id;
    let mut attacker = creature(10_321, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let blocker = creature(10_322, cards::SAVANNAH_LIONS, PlayerId::Two);
    let blocker_id = blocker.card.id;
    blocks.battlefield.extend([watcher, attacker, blocker]);
    blocks.step = Step::DeclareBlockers;
    blocks.blockers_declared = false;
    blocks
        .apply(
            PlayerId::Two,
            Action::DeclareBlocker {
                blocker: blocker_id,
                attacker: attacker_id,
            },
        )
        .expect("the blocker is legal");
    assert!(!blocks.events.iter().any(
        |event| matches!(event, GameEvent::AbilityTriggered { source, .. } if *source == watcher_id)
    ));
    blocks
        .apply(PlayerId::Two, Action::FinishDeclaringBlockers)
        .expect("the completed block declaration is legal");
    assert!(blocks.events.iter().any(
        |event| matches!(event, GameEvent::AbilityTriggered { source, .. } if *source == watcher_id)
    ));
}

#[test]
fn attack_batch_freezes_all_conditions_and_consumes_once_only_once() {
    static CONDITION: TriggerConditionDef = TriggerConditionDef::SourceUntapped;
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_if(
        "Whenever a creature attacks, if this is untapped, trigger.",
        TriggerEventDef::attacks(ObjectPredicateDef::Any),
        &CONDITION,
        EffectDef::None,
    )];
    let definition = CardDefinitionId::new(10_323);
    let mut game = ready_game();
    add_definition(
        &mut game,
        trigger_creature_definition(definition, "Atomic attack watcher", &ABILITIES),
    );
    let watcher = creature(10_323, definition, PlayerId::One);
    let watcher_id = watcher.card.id;
    let first = creature(10_324, cards::SAVANNAH_LIONS, PlayerId::One);
    let second = creature(10_325, cards::SAVANNAH_LIONS, PlayerId::One);
    game.battlefield.extend([watcher, first, second]);

    let ordinary = game
        .battlefield_trigger_listeners()
        .into_iter()
        .next()
        .expect("the ordinary listener is discoverable");
    game.installed_triggers.push(InstalledTrigger {
        id: 77,
        event: ordinary.event,
        capture: ordinary.capture.clone(),
        lifetime: InstalledTriggerLifetime::Once,
    });
    let mut listeners = game.battlefield_trigger_listeners();
    let mut mana = ordinary;
    mana.uses_stack = false;
    mana.capture.condition = None;
    mana.capture.text = "Synthetic immediate mutation";
    listeners.insert(0, mana);
    let events = game.battlefield[1..]
        .iter()
        .map(|permanent| CommittedTriggerEvent::Attacks {
            object: game.trigger_event_object(permanent),
            declaration_size: 2,
            attack_number: 1,
            defending_player: PlayerId::Two,
            attacked_a_planeswalker: false,
        })
        .collect::<Vec<_>>();
    let mut immediate_resolutions = 0;

    game.capture_battlefield_trigger_batch_with_mana_resolver(&listeners, &events, |game, _| {
        immediate_resolutions += 1;
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == watcher_id)
            .expect("watcher exists")
            .tapped = true;
    });

    assert_eq!(immediate_resolutions, 2);
    assert_eq!(
        game.pending_triggers.len(),
        3,
        "two ordinary matches and the first Once match used declaration-time truth"
    );
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn simultaneous_exit_batch_freezes_all_conditions_before_immediate_resolution() {
    static CONDITION: TriggerConditionDef = TriggerConditionDef::SourceUntapped;
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_if(
        "Whenever a creature dies, if this is untapped, trigger.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Creature),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        &CONDITION,
        EffectDef::None,
    )];
    let definition = CardDefinitionId::new(10_326);
    let mut game = ready_game();
    add_definition(
        &mut game,
        trigger_creature_definition(definition, "Atomic exit watcher", &ABILITIES),
    );
    let watcher = creature(10_326, definition, PlayerId::One);
    let watcher_id = watcher.card.id;
    let first = creature(10_327, cards::SAVANNAH_LIONS, PlayerId::One);
    let second = creature(10_328, cards::SAVANNAH_LIONS, PlayerId::One);
    game.battlefield.extend([watcher, first, second]);
    let ordinary = game
        .battlefield_trigger_listeners()
        .into_iter()
        .next()
        .expect("the ordinary listener is discoverable");
    let mut mana = ordinary.clone();
    mana.uses_stack = false;
    mana.capture.condition = None;
    let listeners = [mana, ordinary];
    let events = game.battlefield[1..]
        .iter()
        .map(|permanent| CommittedTriggerEvent::ZoneChanged {
            before: Some(game.trigger_event_object(permanent)),
            after: None,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Graveyard,
            damage_sources: Vec::new(),
        })
        .collect::<Vec<_>>();

    game.capture_battlefield_trigger_batch_with_mana_resolver(&listeners, &events, |game, _| {
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == watcher_id)
            .expect("watcher exists")
            .tapped = true;
    });

    assert_eq!(
        game.pending_triggers.len(),
        2,
        "both simultaneous exits use the pre-resolution intervening-if truth"
    );
}

#[test]
fn simultaneous_exit_predicates_use_source_and_object_lki() {
    let mut game = ready_game();
    let mut source = creature(10_329, cards::SAVANNAH_LIONS, PlayerId::One);
    let source_id = source.card.id;
    let object = creature(10_330, cards::SAVANNAH_LIONS, PlayerId::One);
    let object_id = object.card.id;
    source.attached_to = Some(object_id);
    game.battlefield.extend([source, object]);
    let object_snapshot = game.trigger_event_object(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object_id)
            .expect("object exists before the batch"),
    );

    game.move_permanents_to_graveyard(&[source_id, object_id]);

    let event = CommittedTriggerEvent::ZoneChanged {
        before: Some(object_snapshot),
        after: None,
        from: ZoneKind::Battlefield,
        to: ZoneKind::Graveyard,
        damage_sources: Vec::new(),
    };
    for predicate in [
        ObjectPredicateDef::AttachedToSource,
        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::SourcePower),
        ObjectPredicateDef::DebutSet(CardSet::Alpha),
        ObjectPredicateDef::HasName(ObjectRefDef::Source),
    ] {
        assert!(
            game.trigger_event_matches_for_controller(
                TriggerEventDef::zone_changed(
                    predicate,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &event,
                source_id,
                game.controller_of_object(source_id),
            ),
            "{predicate:?} reads the simultaneous-exit snapshot/LKI",
        );
    }
}

/// One printed ability that names several events fires on each of them and on
/// nothing else. Splitting such a card into one ability per event would make
/// it two triggered abilities where the card prints one, so the alternatives
/// are held inside a single event definition instead.
#[test]
fn an_any_of_event_fires_on_each_alternative_and_no_others() {
    static ALTERNATIVES: [TriggerEventDef; 2] = [
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "Whenever this creature enters or attacks, trigger.",
        TriggerEventDef::AnyOf(&ALTERNATIVES),
        EffectDef::None,
    )];
    let definition = CardDefinitionId::new(10_400);
    let mut game = ready_game();
    add_definition(
        &mut game,
        trigger_creature_definition(definition, "Atomic two-way watcher", &ABILITIES),
    );
    let watcher = creature(10_400, definition, PlayerId::One);
    let watcher_id = watcher.card.id;
    game.battlefield.push(watcher);

    let listener = game
        .battlefield_trigger_listeners()
        .into_iter()
        .find(|listener| listener.capture.source.object == watcher_id)
        .expect("the ability is one listener, not two");
    let snapshot = game.trigger_event_object(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == watcher_id)
            .expect("it is there"),
    );
    let matches = |event: &CommittedTriggerEvent| {
        game.trigger_event_matches_for_controller(
            listener.event,
            event,
            watcher_id,
            Some(PlayerId::One),
        )
    };

    assert!(
        matches(&CommittedTriggerEvent::ZoneChanged {
            before: None,
            after: Some(snapshot.clone()),
            from: ZoneKind::Hand,
            to: ZoneKind::Battlefield,
            damage_sources: Vec::new(),
        }),
        "the entry half matches",
    );
    assert!(
        matches(&CommittedTriggerEvent::Attacks {
            object: snapshot.clone(),
            declaration_size: 1,
            attack_number: 1,
            defending_player: PlayerId::Two,
            attacked_a_planeswalker: false,
        }),
        "and so does the attack half",
    );
    assert!(
        !matches(&CommittedTriggerEvent::ZoneChanged {
            before: Some(snapshot),
            after: None,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Graveyard,
            damage_sources: Vec::new(),
        }),
        "a third event is still no match",
    );
}
