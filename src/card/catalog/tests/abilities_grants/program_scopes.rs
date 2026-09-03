#[test]
#[allow(clippy::items_after_statements, clippy::too_many_lines)]
fn non_targeting_choice_references_are_lexically_scoped() {
    let binding = Binding!("object");
    let chosen = EffectRecipientDef::object(ObjectRefDef::Binding(binding));
    let destroy_chosen: &'static EffectDef = Box::leak(Box::new(EffectDef::Destroy {
        object: chosen,
        can_regenerate: true,
        then: None,
    }));

    assert_eq!(
        super::validate_ability_targets(&[], *destroy_chosen,),
        Err(GrantedAbilityValidationError::ObjectBindingReferenceOutOfScope { binding }),
    );

    let rebound: &'static EffectDef = Box::leak(Box::new(EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(binding),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::new(
            ObjectPredicateDef::Any,
            &[ZoneKind::Battlefield],
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: destroy_chosen,
    })));
    let nested_rebinding = EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(binding),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::new(
            ObjectPredicateDef::Any,
            &[ZoneKind::Battlefield],
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: rebound,
    });
    assert_eq!(
        super::validate_ability_targets(&[], nested_rebinding),
        Err(GrantedAbilityValidationError::BindingAlreadyDeclared { binding }),
    );

    super::validate_ability_targets(
        &[],
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(binding),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::new(
                ObjectPredicateDef::Any,
                &[ZoneKind::Battlefield],
            )),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: destroy_chosen,
        }),
    )
    .expect("the binding is visible only inside its continuation");

    static CHOSEN_CONTROLLER_QUERY: ObjectQueryDef = ObjectQueryDef::controlled_by(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Battlefield],
        PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Binding(
            Binding!("object"),
        ))),
    );
    static COUNT_CHOSEN_CONTROLLERS_CREATURES: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::CountMatchingObjects(&CHOSEN_CONTROLLER_QUERY),
    };

    assert_eq!(
        super::validate_ability_targets(&[], COUNT_CHOSEN_CONTROLLERS_CREATURES),
        Err(GrantedAbilityValidationError::ObjectBindingReferenceOutOfScope { binding }),
        "queries embedded in values participate in lexical binding validation",
    );
    super::validate_ability_targets(
        &[],
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(binding),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::new(
                ObjectPredicateDef::Any,
                &[ZoneKind::Battlefield],
            )),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &COUNT_CHOSEN_CONTROLLERS_CREATURES,
        }),
    )
    .expect("a value query can consume a choice inside its continuation");

    let set_binding = Binding!("objects");
    let sacrifice_chosen: &'static EffectDef = Box::leak(Box::new(EffectDef::Sacrifice {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(set_binding)),
    }));
    assert_eq!(
        super::validate_ability_targets(&[], *sacrifice_chosen),
        Err(
            GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope {
                binding: set_binding,
            }
        ),
    );

    let choose_set = |then: &'static EffectDef| {
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(set_binding),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::new(
                ObjectPredicateDef::Any,
                &[ZoneKind::Battlefield],
            )),
            exclude: None,
            minimum: 0,
            maximum: 2,
            visibility: ChoiceVisibilityDef::Public,
            then,
        })
    };
    let rebound_set: &'static EffectDef = Box::leak(Box::new(choose_set(sacrifice_chosen)));
    assert_eq!(
        super::validate_ability_targets(&[], choose_set(rebound_set)),
        Err(
            GrantedAbilityValidationError::BindingAlreadyDeclared {
                binding: set_binding,
            }
        ),
    );
    super::validate_ability_targets(&[], choose_set(sacrifice_chosen))
        .expect("the object-set binding is visible only inside its continuation");
}

#[test]
fn generic_object_choices_validate_their_cardinality() {
    let cases = [
        (
            ObjectChoiceBindingDef::Objects(Binding!("objects")),
            2,
            1,
        ),
        (
            ObjectChoiceBindingDef::Object(Binding!("object")),
            0,
            2,
        ),
    ];

    for (binding, minimum, maximum) in cases {
        let effect = EffectDef::Choose(ChooseDef {
            binding,
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::new(
                ObjectPredicateDef::Any,
                &[ZoneKind::Battlefield],
            )),
            exclude: None,
            minimum,
            maximum,
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::None,
        });
        assert_eq!(
            super::validate_ability_targets(&[], effect),
            Err(GrantedAbilityValidationError::InvalidObjectChoiceBounds {
                binding,
                minimum,
                maximum,
            }),
        );
    }
}

#[test]
fn target_references_are_validated_through_replacement_programs() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )];
    static TARGET_EFFECT: EffectDef = EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex(1)),
    };
    static PROGRAM: [ReplacementEffectDef; 1] = [ReplacementEffectDef::Perform(&TARGET_EFFECT)];

    assert_eq!(
        super::validate_replacement_ability_targets(
            &TARGETS,
            ReplacementEffectDef::Sequence(&PROGRAM),
        ),
        Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target: TargetIndex(1),
            target_count: 1,
        })
    );
}

#[test]
fn ability_and_program_kinds_must_agree() {
    let mut replacement = AbilityDef::replacement(
        "This permanent enters tapped.",
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    );
    replacement.effect = AbilityEffectDef::declarative(EffectDef::None);
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_ability(replacement);
    set_primary_rules(&mut card, &rules);
    assert_eq!(
        error(card),
        CatalogError::ReplacementAbilityRequiresReplacementProgram {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    );

    let mut spell = AbilityDef::spell("Do nothing.", EffectDef::None);
    spell.effect =
        AbilityEffectDef::replacement_program(ReplacementEffectDef::ReplaceEventWithNothing);
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_ability(spell);
    set_primary_rules(&mut card, &rules);
    assert_eq!(
        error(card),
        CatalogError::ReplacementProgramRequiresReplacementAbility {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    );
}

#[test]
fn replacement_events_reject_programs_their_runtime_would_ignore() {
    static NO_EFFECT: EffectDef = EffectDef::None;
    static NESTED_INVALID: [ReplacementEffectDef; 1] =
        [ReplacementEffectDef::MultiplyEventAmount(2)];
    static INVALID_SEQUENCE: [ReplacementEffectDef; 1] =
        [ReplacementEffectDef::Sequence(&NESTED_INVALID)];

    let cases = [
        (
            ReplacementEventDef::SourceEntersBattlefield,
            ReplacementEffectDef::ReplaceEventWithNothing,
            "ReplaceEventWithNothing",
        ),
        (
            ReplacementEventDef::SourceEntersBattlefield,
            ReplacementEffectDef::Sequence(&INVALID_SEQUENCE),
            "MultiplyEventAmount",
        ),
        (
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
            "ModifyBattlefieldEntry",
        ),
        (
            ReplacementEventDef::WouldBeginTurn {
                player: PlayerRelation::You,
                kind: TurnKindDef::Any,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
            "MoveToZone",
        ),
        (
            ReplacementEventDef::WouldMove {
                from: Some(ZoneKind::Hand),
                to: ZoneKind::Graveyard,
                cause: ZoneMoveCauseDef::Any,
            },
            ReplacementEffectDef::Perform(&NO_EFFECT),
            "Perform",
        ),
        (
            ReplacementEventDef::AnyObjectWouldMove {
                object: ObjectPredicateDef::Any,
                to: ZoneKind::Graveyard,
            },
            ReplacementEffectDef::MultiplyEventAmount(2),
            "MultiplyEventAmount",
        ),
    ];

    for (event, effect, operation) in cases {
        let ability = AbilityDef::defined_replacement(
            "Replace an event.",
            ReplacementAbilityDef::new().with_event(event),
            effect,
        );
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_ability(ability);
        set_primary_rules(&mut card, &rules);
        assert_eq!(
            error(card),
            CatalogError::UnsupportedReplacementProgram {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                event,
                operation,
            },
        );
    }
}

#[test]
fn replacement_event_validation_accepts_each_supported_program_family() {
    static UNTAP_SOURCE: EffectDef = EffectDef::Untap {
        object: EffectRecipientDef::Source,
    };
    static BEGIN_TURN: [ReplacementEffectDef; 2] = [
        ReplacementEffectDef::ReplaceEventWithNothing,
        ReplacementEffectDef::Perform(&UNTAP_SOURCE),
    ];
    static TAKE_EXTRA_TURN: EffectDef = EffectDef::TakeExtraTurn {
        player: EffectRecipientDef::Controller,
    };
    static BATTLEFIELD_EXIT: [ReplacementEffectDef; 2] = [
        ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ReplacementEffectDef::Perform(&TAKE_EXTRA_TURN),
    ];

    let cases = [
        (
            ReplacementEventDef::SourceEntersBattlefield,
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        (
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            ReplacementEffectDef::MultiplyEventAmount(2),
        ),
        (
            ReplacementEventDef::WouldBeginTurn {
                player: PlayerRelation::You,
                kind: TurnKindDef::Any,
            },
            ReplacementEffectDef::Sequence(&BEGIN_TURN),
        ),
        (
            ReplacementEventDef::WouldMove {
                from: Some(ZoneKind::Hand),
                to: ZoneKind::Graveyard,
                cause: ZoneMoveCauseDef::Any,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Battlefield),
        ),
        (
            ReplacementEventDef::WouldMove {
                from: Some(ZoneKind::Battlefield),
                to: ZoneKind::Graveyard,
                cause: ZoneMoveCauseDef::Any,
            },
            ReplacementEffectDef::Sequence(&BATTLEFIELD_EXIT),
        ),
        (
            ReplacementEventDef::AnyObjectWouldMove {
                object: ObjectPredicateDef::Any,
                to: ZoneKind::Graveyard,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ];

    for (event, effect) in cases {
        let ability = AbilityDef::defined_replacement(
            "Replace an event.",
            ReplacementAbilityDef::new().with_event(event),
            effect,
        );
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_ability(ability);
        set_primary_rules(&mut card, &rules);
        CardCatalog::new([card]).expect("the event's shared runtime supports this program");
    }
}

#[test]
#[allow(clippy::items_after_statements, clippy::too_many_lines)]
fn installed_triggers_retain_installer_targets_and_reject_fresh_target_scopes() {
    static INSTALLER_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        },
    )];
    static FRESH_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        },
    )];
    static LEXICAL_EFFECT: EffectDef = EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
        then: None,
    };
    static LEXICAL_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the next end step, destroy that permanent.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        LEXICAL_EFFECT,
    );
    static FRESH_TARGET_TRIGGER: AbilityDef = AbilityDef::triggered_with_targets(
        "At the beginning of the next end step, destroy target permanent.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        &FRESH_TARGETS,
        LEXICAL_EFFECT,
    );
    static LEGACY_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the next end step, destroy that permanent.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        LEXICAL_EFFECT,
    )
    .with_legacy_procedure();

    super::validate_ability_targets(
        &INSTALLER_TARGETS,
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&LEXICAL_TRIGGER)),
    )
    .expect("a targetless installed trigger may retain its installer's target slots");

    for ability in [&FRESH_TARGET_TRIGGER, &LEGACY_TRIGGER] {
        assert_eq!(
            super::validate_ability_targets(
                &INSTALLER_TARGETS,
                EffectDef::InstallTrigger(InstalledTriggerDef::once(ability)),
            ),
            Err(GrantedAbilityValidationError::UnsupportedInstalledTriggerAbility),
        );
    }

    static CONDITIONLESS_STATE_TRIGGER: AbilityDef = AbilityDef::triggered(
        "Whenever an unspecified state exists, trigger.",
        TriggerEventDef::StateCondition,
        EffectDef::None,
    );
    assert_eq!(
        super::validate_ability_targets(
            &[],
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&CONDITIONLESS_STATE_TRIGGER,)),
        ),
        Err(GrantedAbilityValidationError::UnsupportedTriggerEvent {
            event: TriggerEventDef::StateCondition,
        }),
    );
    static STATE_CONDITION: TriggerConditionDef = TriggerConditionDef::SourceOnBattlefield;
    static CONDITIONAL_STATE_TRIGGER: AbilityDef = AbilityDef::triggered_if(
        "Whenever this remains on the battlefield, trigger.",
        TriggerEventDef::StateCondition,
        &STATE_CONDITION,
        EffectDef::None,
    );
    assert_eq!(
        super::validate_ability_targets(
            &[],
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&CONDITIONAL_STATE_TRIGGER,)),
        ),
        Err(GrantedAbilityValidationError::UnsupportedTriggerEvent {
            event: TriggerEventDef::StateCondition,
        }),
        "installed state triggers stay rejected until Once consumption joins state capture",
    );
    static WRONG_ZONE_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the next end step, trigger.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::None,
    )
    .with_source_zones(&[ZoneKind::Graveyard]);
    assert_eq!(
        super::validate_ability_targets(
            &[],
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&WRONG_ZONE_TRIGGER)),
        ),
        Err(GrantedAbilityValidationError::UnsupportedInstalledTriggerAbility),
    );

    static INVALID_SPELL: AbilityDef = AbilityDef::spell_with_targets(
        "Install an unsupported delayed trigger.",
        &INSTALLER_TARGETS,
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&FRESH_TARGET_TRIGGER)),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_ability(INVALID_SPELL);
    set_primary_rules(&mut card, &rules);
    assert_eq!(
        error(card),
        CatalogError::UnsupportedInstalledTriggerAbility {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    );

    static INVALID_STATE_INSTALL_SPELL: AbilityDef = AbilityDef::spell(
        "Install an unsupported state trigger.",
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&CONDITIONLESS_STATE_TRIGGER)),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_ability(INVALID_STATE_INSTALL_SPELL);
    set_primary_rules(&mut card, &rules);
    assert_eq!(
        error(card),
        CatalogError::UnsupportedTriggerEvent {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            event: TriggerEventDef::StateCondition,
        },
    );
}

#[allow(clippy::large_types_passed_by_value)]
fn definition_with_ability(ability: AbilityDef) -> CardDefinition {
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_ability(ability);
    set_primary_rules(&mut card, &rules);
    card
}

#[test]
fn shared_trigger_catalog_rejects_undiscoverable_or_incomplete_listeners() {
    static CONDITION: TriggerConditionDef = TriggerConditionDef::SourceOnBattlefield;
    let upkeep = TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    };
    // A graveyard listener is discoverable; a hand one is not, and neither
    // is a listener that claims two zones at once, because the capture pass
    // reads a card from exactly one of them.
    let outside_battlefield = AbilityDef::triggered("At upkeep, trigger.", upkeep, EffectDef::None)
        .with_source_zones(&[ZoneKind::Hand]);
    let mixed_zones = AbilityDef::triggered("At upkeep, trigger.", upkeep, EffectDef::None)
        .with_source_zones(&[ZoneKind::Battlefield, ZoneKind::Graveyard]);
    let state_without_condition = AbilityDef::triggered(
        "Trigger whenever a state exists.",
        TriggerEventDef::StateCondition,
        EffectDef::None,
    );
    let conditional_mana = AbilityDef::defined(
        "Whenever this is tapped for mana, if it remains on the battlefield, add {B}.",
        DeclarativeAbilityDef::TriggeredMana(
            match AbilityDef::triggered_mana(
                "placeholder",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Source),
                EffectDef::None,
            )
            .definition
            {
                DeclarativeAbilityDef::TriggeredMana(definition) => {
                    definition.with_condition(&CONDITION)
                }
                _ => unreachable!(),
            },
        ),
        EffectDef::AddMana(crate::card::AddManaEffectDef::one(
            crate::card::ManaColor::Black,
        )),
    );
    let ordinary_tap_mana = AbilityDef::triggered_mana(
        "Whenever this becomes tapped, add {B}.",
        TriggerEventDef::tapped(ObjectPredicateDef::Source),
        EffectDef::AddMana(crate::card::AddManaEffectDef::one(
            crate::card::ManaColor::Black,
        )),
    );
    let damage_mana = AbilityDef::triggered_mana(
        "Whenever this deals damage, add {B}.",
        TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
        EffectDef::AddMana(crate::card::AddManaEffectDef::one(
            crate::card::ManaColor::Black,
        )),
    );

    for (ability, event) in [
        (outside_battlefield, upkeep),
        (mixed_zones, upkeep),
        (state_without_condition, TriggerEventDef::StateCondition),
        (
            conditional_mana,
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Source),
        ),
        (
            ordinary_tap_mana,
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
        ),
        (
            damage_mana,
            TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
        ),
    ] {
        assert_eq!(
            error(definition_with_ability(ability)),
            CatalogError::UnsupportedTriggerEvent {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                event,
            },
        );
    }
}

#[test]
fn triggered_mana_catalog_requires_a_supported_nonempty_add_mana_program() {
    static MIXED_PROGRAM: [EffectDef; 2] = [
        EffectDef::AddMana(crate::card::AddManaEffectDef::one(
            crate::card::ManaColor::Black,
        )),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ];
    let event = TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Source);
    for effect in [
        EffectDef::None,
        EffectDef::Sequence(&[]),
        EffectDef::Sequence(&MIXED_PROGRAM),
        EffectDef::AddMana(crate::card::AddManaEffectDef::choice(&[])),
    ] {
        let ability =
            AbilityDef::triggered_mana("Whenever tapped for mana, add mana.", event, effect);
        assert_eq!(
            error(definition_with_ability(ability)),
            CatalogError::UnsupportedTriggeredManaProgram {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            },
        );
    }

    for effect in [
        EffectDef::AddMana(crate::card::AddManaEffectDef::choice(&[
            crate::card::ManaColor::Black,
            crate::card::ManaColor::Green,
        ])),
        EffectDef::AddMana(crate::card::AddManaEffectDef::choice_from(
            crate::card::ManaTypeSetDef::produced_by(
                crate::card::ObjectRefDef::TriggeringObject,
            ),
        )),
    ] {
        let ability =
            AbilityDef::triggered_mana("Whenever tapped for mana, add mana.", event, effect);
        CardCatalog::new([definition_with_ability(ability)])
            .expect("supported triggered mana choices enter the catalog");
    }
}

#[test]
fn trigger_catalog_rejects_static_only_affected_object_anchors() {
    for event in [
        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
            source: DamageSourceMatcherDef::AffectedObject,
            ..DamageEventMatcherDef::ANY
        }),
        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
            recipient: DamageRecipientMatcherDef::AffectedObject,
            ..DamageEventMatcherDef::ANY
        }),
        TriggerEventDef::spell_cast(ObjectPredicateDef::HasNonManaActivatedAbility),
        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
            source: DamageSourceMatcherDef::Matching(
                ObjectPredicateDef::HasNonManaActivatedAbility,
            ),
            ..DamageEventMatcherDef::ANY
        }),
    ] {
        let ability =
            AbilityDef::triggered("Whenever damage is dealt, trigger.", event, EffectDef::None);
        assert_eq!(
            error(definition_with_ability(ability)),
            CatalogError::UnsupportedTriggerEvent {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                event,
            },
        );
    }
}

#[test]
#[allow(clippy::items_after_statements)]
fn merged_effect_vocabulary_preserves_local_target_bounds() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )];
    let out_of_range = TargetIndex(1);
    let recipient = EffectRecipientDef::ControllerOfTarget(out_of_range);
    let effects = [
        EffectDef::Tap {
            object: EffectRecipientDef::objects_controlled_by_target(
                ObjectPredicateDef::Any,
                out_of_range,
            ),
        },
        EffectDef::PartitionGroup(crate::card::PartitionGroupDef {
            actor: PlayerRefDef::ControllerOf(ObjectRefDef::Target(out_of_range)),
            input: ObjectSetDef::Query(ObjectQueryDef::new(
                ObjectPredicateDef::Any,
                &[ZoneKind::Battlefield],
            )),
            first: Binding!("objects"),
            second: Binding!("objects_2"),
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::None,
        }),
        EffectDef::Mill {
            player: recipient,
            amount: ValueDef::DividedAmongTargets,
        },
        EffectDef::Apply {
            recipient,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::CastSpell,
                ObjectPredicateDef::NoncreatureSpell,
            ))),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ];

    for effect in effects {
        assert_eq!(
            super::validate_ability_targets(&TARGETS, effect),
            Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
                target: out_of_range,
                target_count: 1,
            })
        );
    }

    static VALID_SEQUENCE: [EffectDef; 2] = [
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::DividedAmongTargets,
        },
        EffectDef::ScheduleTurnPhases(&[crate::card::TurnPhaseDef::Combat]),
    ];
    super::validate_ability_targets(&TARGETS, EffectDef::Sequence(&VALID_SEQUENCE))
        .expect("implicit divided values and target-free combat effects add no slot reference");
}

#[test]
fn authored_target_count_fits_the_positional_index_space() {
    let targets = Box::leak(
        vec![
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any),);
            257
        ]
        .into_boxed_slice(),
    );
    let abilities = Box::leak(
        vec![AbilityDef::activated_with_targets(
            "An ability with too many targets.",
            &[],
            targets,
            EffectDef::None,
        )]
        .into_boxed_slice(),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(abilities);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::TooManyAbilityTargets {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            count: 257,
        }
    );
}

#[test]
fn nested_grant_capacity_is_validated_per_granted_definition() {
    static TERMINAL: AbilityDef = AbilityDef::static_ability("A terminal granted ability.", EffectDef::None);
    let effects = Box::leak(
        vec![
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&TERMINAL),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            };
            257
        ]
        .into_boxed_slice(),
    );
    let child = Box::leak(Box::new(AbilityDef::activated(
        "This ability contains too many nested grant sites.",
        &[],
        EffectDef::Sequence(effects),
    )));

    assert_eq!(
        error(definition_granting(child)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::TooManyGrantSites { count: 257 },
        }
    );
}

#[test]
fn executable_legacy_procedures_are_rejected() {
    static LEGACY: AbilityDef = AbilityDef::activated(
        "An ability routed through the legacy procedure.",
        &[],
        EffectDef::None,
    )
    .with_legacy_procedure();

    let mut top_level = definition(1, "Test Card", CardSet::Alpha);
    let rules = top_level.rules.with_ability(LEGACY);
    set_primary_rules(&mut top_level, &rules);
    assert_eq!(
        error(top_level),
        CatalogError::UnsupportedLegacyProcedure {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        }
    );

    assert_eq!(
        error(definition_granting(&LEGACY)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::UnsupportedLegacyProcedure,
        }
    );
}

#[test]
fn explicitly_tagged_mana_abilities_cannot_declare_targets() {
    static COSTS: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::defined(
        "Target player adds mana.",
        DeclarativeAbilityDef::ActivatedMana(
            ActivatedAbilityDef::new(&COSTS).with_targets(&TARGETS),
        ),
        EffectDef::None,
    )];
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(&ABILITIES);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::ManaAbilityHasTargets {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        }
    );
}
