use super::*;

#[test]
fn ability_ids_follow_clause_order_within_each_card_part() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::spell("first", EffectDef::None),
        AbilityDef::not_implemented("second", "Only positional identity matters here."),
    ];
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(&ABILITIES);
    set_primary_rules(&mut card, &rules);

    let attached = card.parts[0].rules.indexed_abilities().collect::<Vec<_>>();
    assert_eq!(attached[0].id, AbilityId(0));
    assert_eq!(attached[1].id, AbilityId(1));
    CardCatalog::new(vec![card]).expect("ordered clauses receive distinct positional IDs");
}

#[test]
fn one_card_part_cannot_define_multiple_spell_abilities() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::spell("first", EffectDef::None),
        AbilityDef::spell("second", EffectDef::None),
    ];
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(&ABILITIES);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::MultipleSpellAbilities {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            count: 2,
        }
    );
}

#[test]
fn positional_ability_ids_reject_more_than_their_address_space() {
    let abilities = Box::leak(
        vec![AbilityDef::spell("A spell ability.", EffectDef::None); 257].into_boxed_slice(),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(abilities);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::TooManyAbilities {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            count: 257,
        }
    );
}

#[test]
fn grant_ids_reject_more_than_their_structural_address_space() {
    static GRANTED: AbilityDef = AbilityDef::not_implemented(
        "A granted ability.",
        "The test only needs a reusable definition.",
    );
    let effects = Box::leak(
        vec![
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&GRANTED),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            };
            257
        ]
        .into_boxed_slice(),
    );
    let abilities = Box::leak(
        vec![AbilityDef::static_ability(
            "This object receives many abilities.",
            EffectDef::Sequence(effects),
        )]
        .into_boxed_slice(),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(abilities);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::TooManyAbilityGrantSites {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            count: 257,
        }
    );
}

#[test]
fn delayed_grants_count_toward_the_structural_address_space() {
    static GRANTED: AbilityDef = AbilityDef::not_implemented(
        "A granted ability.",
        "The test only needs a reusable definition.",
    );
    static GRANT: EffectDef = EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::GrantAbility(&GRANTED),
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    };
    static DELAYED_GRANT: EffectDef = EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::You,
        effect: &GRANT,
    };
    let effects = Box::leak(vec![DELAYED_GRANT; 257].into_boxed_slice());
    let abilities = Box::leak(
        vec![AbilityDef::static_ability(
            "This object schedules many granted abilities.",
            EffectDef::Sequence(effects),
        )]
        .into_boxed_slice(),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(abilities);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::TooManyAbilityGrantSites {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            count: 257,
        }
    );
}

#[test]
fn executable_granted_static_abilities_are_rejected_until_fixed_point_evaluation_exists() {
    static GRANTED: AbilityDef =
        AbilityDef::static_ability("This object gets +1/+1.", EffectDef::None);

    assert_eq!(
        error(definition_granting(&GRANTED)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::ExecutableStaticAbility,
        }
    );
}

#[test]
fn granted_ability_validation_reports_nested_structural_paths() {
    static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
    static CHILD: AbilityDef = AbilityDef::activated(
        "This ability grants another ability.",
        &[],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&INVALID),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    );

    assert_eq!(
        error(definition_granting(&CHILD)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY, GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::EmptyText,
        }
    );
}

#[test]
fn granted_ability_validation_follows_sacrifice_continuations() {
    static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
    static THEN: EffectDef = EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::GrantAbility(&INVALID),
        duration: EffectDurationDef::UntilEndOfTurn,
    };
    static CHILD: AbilityDef = AbilityDef::activated(
        "Sacrifice a permanent, then grant an ability.",
        &[],
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::Any,
            then: Some(&THEN),
            optional: false,
        },
    );

    assert_eq!(
        error(definition_granting(&CHILD)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY, GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::EmptyText,
        }
    );
}

#[test]
fn granted_modal_branches_validate_nested_grants_in_printed_order() {
    static VALID: AbilityDef = AbilityDef::not_implemented(
        "A valid granted ability.",
        "Only nested validation matters in this fixture.",
    );
    static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
    static MODES: [AbilityDef; 2] = [
        AbilityDef::spell(
            "The first mode grants a valid ability.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&VALID),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::spell(
            "The second mode grants an invalid ability.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&INVALID),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ];
    static GRANTED_MODAL: AbilityDef = AbilityDef::choose_one_spell("Choose one.", &MODES);

    assert_eq!(
        error(definition_granting(&GRANTED_MODAL)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY, GrantId(1)],
            problem: GrantedAbilityValidationError::EmptyText,
        }
    );
}

#[test]
fn granted_modal_capacity_counts_grants_across_all_modes() {
    static TERMINAL: AbilityDef = AbilityDef::not_implemented(
        "A terminal granted ability.",
        "The terminal ability is intentionally not executable.",
    );
    let grants = |count| {
        Box::leak(
            vec![
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&TERMINAL),
                    duration: EffectDurationDef::UntilEndOfTurn,
                };
                count
            ]
            .into_boxed_slice(),
        )
    };
    let modes = Box::leak(
        vec![
            AbilityDef::spell("First mode.", EffectDef::Sequence(grants(128))),
            AbilityDef::spell("Second mode.", EffectDef::Sequence(grants(129))),
        ]
        .into_boxed_slice(),
    );
    let granted_modal = Box::leak(Box::new(AbilityDef::choose_one_spell("Choose one.", modes)));

    assert_eq!(
        error(definition_granting(granted_modal)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::TooManyGrantSites { count: 257 },
        }
    );
}

#[test]
fn granted_ability_validation_checks_zones_mana_targets_and_target_slots() {
    static MANA_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )];
    static NO_ZONES: AbilityDef =
        AbilityDef::activated("An activated ability.", &[], EffectDef::None).with_source_zones(&[]);
    static TARGETED_MANA: AbilityDef = AbilityDef::defined(
        "A targeted mana ability.",
        DeclarativeAbilityDef::ActivatedMana(
            ActivatedAbilityDef::new(&[AbilityCostDef::TapSource]).with_targets(&MANA_TARGETS),
        ),
        EffectDef::None,
    );
    static OUT_OF_RANGE_TARGET: AbilityDef = AbilityDef::activated_with_targets(
        "An activated ability.",
        &[],
        &MANA_TARGETS,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex(1)),
            amount: crate::ValueDef::Constant(1),
        },
    );

    let cases = [
        (&NO_ZONES, GrantedAbilityValidationError::HasNoSourceZone),
        (
            &TARGETED_MANA,
            GrantedAbilityValidationError::ManaAbilityHasTargets,
        ),
        (
            &OUT_OF_RANGE_TARGET,
            GrantedAbilityValidationError::TargetReferenceOutOfBounds {
                target: TargetIndex(1),
                target_count: 1,
            },
        ),
    ];
    for (granted, problem) in cases {
        assert_eq!(
            error(definition_granting(granted)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY],
                problem,
            }
        );
    }
}

#[test]
fn target_references_are_validated_through_nested_values() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )];
    static CONDITION: TargetConditionDef = TargetConditionDef {
        slot: TargetIndex(1),
        object: crate::ObjectPredicateDef::Any,
        then: ValueDef::Constant(1),
        otherwise: ValueDef::Constant(0),
    };
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::spell_with_targets(
        "Use a nested value from the chosen target.",
        &TARGETS,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::IfTargetMatches(&CONDITION),
        },
    )];
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(&ABILITIES);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::AbilityTargetReferenceOutOfBounds {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            target: TargetIndex(1),
            target_count: 1,
        }
    );
}

#[test]
fn non_targeting_choice_references_are_lexically_scoped() {
    let choice = ChoiceIndex::PRIMARY;
    let chosen = EffectRecipientDef::ChosenPermanent(choice);
    let destroy_chosen = Box::leak(Box::new(EffectDef::Destroy {
        object: chosen,
        can_regenerate: true,
    }));

    assert_eq!(
        super::validate_ability_targets(&[], *destroy_chosen,),
        Err(GrantedAbilityValidationError::ChoiceReferenceOutOfScope { choice }),
    );

    let rebound = Box::leak(Box::new(EffectDef::ChoosePermanent {
        choice,
        chooser: EffectRecipientDef::Controller,
        object: ObjectPredicateDef::Any,
        controller: PlayerRelation::Any,
        then: destroy_chosen,
    }));
    let nested_rebinding = EffectDef::ChoosePermanent {
        choice,
        chooser: EffectRecipientDef::Controller,
        object: ObjectPredicateDef::Any,
        controller: PlayerRelation::Any,
        then: rebound,
    };
    assert_eq!(
        super::validate_ability_targets(&[], nested_rebinding),
        Err(GrantedAbilityValidationError::ChoiceBindingAlreadyInScope { choice }),
    );

    super::validate_ability_targets(
        &[],
        EffectDef::ChoosePermanent {
            choice,
            chooser: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::Any,
            controller: PlayerRelation::Any,
            then: destroy_chosen,
        },
    )
    .expect("the binding is visible only inside its continuation");
}

#[test]
fn merged_effect_vocabulary_preserves_local_target_bounds() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )];
    let out_of_range = TargetIndex(1);
    let recipient = EffectRecipientDef::ControllerOfTarget(out_of_range);
    let effects = [
        EffectDef::Tap {
            object: EffectRecipientDef::ObjectsControlledByTarget {
                object: ObjectPredicateDef::Any,
                slot: out_of_range,
            },
        },
        EffectDef::SplitPermanentsAndSacrificeAPile { player: recipient },
        EffectDef::Mill {
            player: recipient,
            amount: ValueDef::DividedAmongTargets,
        },
        EffectDef::CannotCastNoncreatureSpellsThisTurn { player: recipient },
        EffectDef::ChooseCardName { object: recipient },
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

    super::validate_ability_targets(
        &TARGETS,
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
            EffectDef::AdditionalCombatPhase,
        ]),
    )
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
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            count: 257,
        }
    );
}

#[test]
fn nested_grant_capacity_is_validated_per_granted_definition() {
    static TERMINAL: AbilityDef = AbilityDef::not_implemented(
        "A terminal granted ability.",
        "The terminal ability is intentionally not executable.",
    );
    let effects = Box::leak(
        vec![
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&TERMINAL),
                duration: EffectDurationDef::UntilEndOfTurn,
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
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::TooManyGrantSites { count: 257 },
        }
    );
}

#[test]
fn granted_non_declarative_implementations_require_an_explanation() {
    static GRANTED: AbilityDef =
        AbilityDef::activated("An incompletely implemented ability.", &[], EffectDef::None)
            .with_coverage(AbilityCoverageDef::metadata_only(""));

    assert_eq!(
        error(definition_granting(&GRANTED)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::MissingImplementationExplanation,
        }
    );
}

#[test]
fn executable_legacy_procedures_require_custom_effect_execution() {
    static LEGACY: AbilityDef = AbilityDef::activated(
        "An ability routed through the legacy procedure.",
        &[],
        EffectDef::None,
    )
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The test supplies the required legacy-procedure explanation.",
    ))
    .with_legacy_procedure();

    let mut top_level = definition(1, "Test Card", CardSet::Alpha);
    let rules = top_level.rules.with_ability(LEGACY);
    set_primary_rules(&mut top_level, &rules);
    assert_eq!(
        error(top_level),
        CatalogError::LegacyProcedureRequiresCustomExecution {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        }
    );

    assert_eq!(
        error(definition_granting(&LEGACY)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::LegacyProcedureRequiresCustomExecution,
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
            definition: CardDefinitionId(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        }
    );
}
