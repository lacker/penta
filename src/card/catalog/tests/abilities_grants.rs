use super::*;
use crate::card::{
    BasicLandType, CardTypeSet, EffectPaymentDef, PayOrDef, ScaledValueDef, abilities,
};

#[test]
fn catalog_rejects_effect_operations_in_the_wrong_execution_context() {
    static STATIC_PUMP: EffectDef = EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(1),
        ),
    };
    static RESOLVING_STATIC: [EffectDef; 1] = [STATIC_PUMP];
    static ATTACK_QUERY: ObjectQueryDef =
        ObjectQueryDef::new(ObjectPredicateDef::Any, &[ZoneKind::Battlefield]);

    let cases = [
        (
            AbilityDef::static_ability(
                "At static-effect time, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            "static",
            "DrawCards",
        ),
        (
            AbilityDef::static_ability(
                "At static-effect time, store a resolved pump.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            "static",
            "Apply",
        ),
        (
            AbilityDef::spell(
                "Resolve a live static effect.",
                EffectDef::Sequence(&RESOLVING_STATIC),
            ),
            "resolving",
            "StaticApply",
        ),
        (
            AbilityDef::activated(
                "Use a declaration-only attack restriction.",
                &[],
                EffectDef::CannotAttackUnless(&ATTACK_QUERY),
            ),
            "resolving",
            "CannotAttackUnless",
        ),
    ];

    for (ability, context, operation) in cases {
        assert_eq!(
            error(definition_with_ability(ability)),
            CatalogError::UnsupportedAbilityEffectProgramContext {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                context,
                operation,
            },
        );
    }
}

#[test]
fn catalog_accepts_each_supported_static_program_lane() {
    static GRAVEYARD_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Graveyard],
        PlayerRelation::You,
    );
    static FORESTS: ObjectQueryDef = ObjectQueryDef::new(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
        &[ZoneKind::Battlefield],
    );
    static LAND_CREATURE: [AppliedEffectDef; 2] = [
        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
    ];
    static AURAS_ON_SOURCE: ObjectQueryDef = ObjectQueryDef::new(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Subtype("Aura"),
            ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
    );
    static SCALED_AURA_BONUS: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
        ValueDef::CountMatchingObjects(&AURAS_ON_SOURCE),
        2,
    ));
    let abilities = [
        AbilityDef::static_ability(
            "This creature gets +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::static_ability(
            "Players can't cast noncreature spells.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::NoncreatureSpell,
                    ),
                )),
            },
        ),
        AbilityDef::static_ability(
            "This spell can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        )
        .with_source_zones(&[ZoneKind::Stack]),
        AbilityDef::static_ability(
            "This spell costs {1} less for each creature card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&GRAVEYARD_CREATURES)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::static_ability(
            "Forests are 1/1 creatures that are still lands.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(FORESTS)),
                effect: AppliedEffectDef::Composite(&LAND_CREATURE),
            },
        ),
        AbilityDef::static_ability(
            "This creature gets +2/+2 for each Aura attached to it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    SCALED_AURA_BONUS,
                    SCALED_AURA_BONUS,
                ),
            },
        ),
        AbilityDef::enforced_when_cast(
            "This spell has an externally enforced casting restriction.",
            "The casting action generator enforces this clause.",
        ),
    ];

    for ability in abilities {
        CardCatalog::new([definition_with_ability(ability)])
            .expect("the live runtime consumes this static program lane");
    }
}

#[test]
fn static_apply_rejects_shapes_its_live_reader_would_ignore() {
    let cases = [
        (
            EffectRecipientDef::EachPlayer,
            AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
            "StaticApply with an unsupported player-facing effect",
        ),
        (
            EffectRecipientDef::Source,
            AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::CastSpell,
                ObjectPredicateDef::Any,
            ))),
            "StaticApply with an unsupported object-facing effect",
        ),
        (
            EffectRecipientDef::Source,
            AppliedEffectDef::Composite(&[]),
            "StaticApply with an unsupported object-facing effect",
        ),
        (
            EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::new(
                ObjectPredicateDef::Subtype("Forest"),
                &[ZoneKind::Battlefield],
            ))),
            AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
            "StaticApply with an unsupported object-facing effect",
        ),
        (
            EffectRecipientDef::Source,
            AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef {
                recipient: DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::EffectController,
                ),
                ..DamageEventMatcherDef::ANY
            })),
            "StaticApply with an unsupported object-facing effect",
        ),
        (
            EffectRecipientDef::Source,
            AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef {
                source: DamageSourceMatcherDef::Object(ObjectRefDef::ResolvingObject),
                ..DamageEventMatcherDef::ANY
            })),
            "StaticApply with an unsupported object-facing effect",
        ),
    ];

    for (recipient, effect, operation) in cases {
        let ability = AbilityDef::static_ability(
            "Apply a live static effect.",
            EffectDef::StaticApply { recipient, effect },
        );
        assert_eq!(
            error(definition_with_ability(ability)),
            CatalogError::UnsupportedAbilityEffectProgramContext {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                context: "static",
                operation,
            },
        );
    }
}

#[test]
fn resolving_apply_rejects_shapes_that_cannot_be_stored() {
    for effect in [
        AppliedEffectDef::Composite(&[]),
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef::ANY)),
    ] {
        assert_eq!(
            validate_ability_targets(
                &[],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect,
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
        );
    }
    assert_eq!(
        validate_ability_targets(
            &[],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn nonbattlefield_ability_grants_are_executable_flashback_until_cleanup() {
    static FLYING: AbilityDef = abilities::flying();
    static FLASHBACK: AbilityDef = abilities::flashback_for_card_mana_cost();
    static MIRACLE: AbilityDef = abilities::miracle(ManaCost::new(0, 0));
    static INCOMPLETE_FLASHBACK: AbilityDef = abilities::flashback_for_card_mana_cost()
        .with_coverage(AbilityCoverageDef::metadata_only(
            "This fixture verifies that non-executable grants are rejected.",
        ));
    static GRAVEYARD_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: None,
        },
    )];
    static UNSUPPORTED_ZONE_TARGETS: [AbilityTargetDef; 2] = [
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Hand],
            controller: None,
            owner: None,
        }),
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    ];
    static GRAVEYARD_CARDS: ObjectQueryDef =
        ObjectQueryDef::new(ObjectPredicateDef::Any, &[ZoneKind::Graveyard]);

    let targeted_grant = |ability, duration| EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(ability),
        duration,
    };

    validate_ability_targets(
        &GRAVEYARD_TARGET,
        targeted_grant(&FLASHBACK, ResolvedEffectDurationDef::UntilEndOfTurn),
    )
    .expect("the hidden-zone runtime reads executable flashback grants until cleanup");
    validate_ability_targets(
        &[],
        EffectDef::Apply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(GRAVEYARD_CARDS)),
            effect: AppliedEffectDef::add_ability(&FLASHBACK),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )
    .expect("mass flashback grants use the same supported hidden-zone reader");

    for ability in [&FLYING, &MIRACLE, &INCOMPLETE_FLASHBACK] {
        assert_eq!(
            validate_ability_targets(
                &GRAVEYARD_TARGET,
                targeted_grant(ability, ResolvedEffectDurationDef::UntilEndOfTurn),
            ),
            Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
            "a hidden-zone grant must be an executable Flashback ability",
        );
    }
    for target in UNSUPPORTED_ZONE_TARGETS {
        assert_eq!(
            validate_ability_targets(
                &[target],
                targeted_grant(&FLASHBACK, ResolvedEffectDurationDef::UntilEndOfTurn),
            ),
            Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
            "the temporary Flashback reader only consumes graveyard-card grants",
        );
    }
    assert_eq!(
        validate_ability_targets(
            &GRAVEYARD_TARGET,
            targeted_grant(&FLASHBACK, ResolvedEffectDurationDef::Permanent),
        ),
        Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
        "the runtime only stores nonbattlefield card grants until cleanup",
    );

    for duration in [
        ResolvedEffectDurationDef::UntilEndOfTurn,
        ResolvedEffectDurationDef::Permanent,
    ] {
        let spell = AbilityDef::spell(
            "This spell grants itself an ability.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&FLYING),
                duration,
            },
        );
        assert_eq!(
            error(definition_with_ability(spell)),
            CatalogError::UnsupportedAbilityEffectProgramContext {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                context: "resolving",
                operation: "Apply grants an ability to a nonbattlefield source",
            },
        );
    }
}

#[test]
fn triggering_object_grants_use_the_declared_event_zone() {
    static HASTE: AbilityDef = abilities::haste();

    let grant = |event| {
        AbilityDef::triggered(
            "The triggering object gains haste until end of turn.",
            event,
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::add_ability(&HASTE),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
    };

    CardCatalog::new([definition_with_ability(grant(
        TriggerEventDef::zone_changed(ObjectPredicateDef::Any, None, Some(ZoneKind::Battlefield)),
    ))])
    .expect("an ETB trigger provably names a battlefield object");

    assert_eq!(
        error(definition_with_ability(grant(
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Any,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            )
        ))),
        CatalogError::UnsupportedResolvingAppliedEffect {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
        "a departure trigger names a nonbattlefield card and cannot grant haste",
    );
}

#[test]
fn payment_target_sets_must_resolve_to_one_player() {
    static PLAYER_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
        2,
    )];
    static NONE: EffectDef = EffectDef::None;

    assert_eq!(
        validate_ability_targets(
            &PLAYER_TARGETS,
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::LegalTargets(TargetIndex::PRIMARY),
                    ManaCost::new(1, 0),
                ),
                &NONE,
            )),
        ),
        Err(
            GrantedAbilityValidationError::TargetReferenceRequiresSingular {
                target: TargetIndex::PRIMARY,
                maximum: 2,
            },
        ),
    );
}

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
            definition: CardDefinitionId::new(1),
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
            definition: CardDefinitionId::new(1),
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
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&GRANTED),
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
            definition: CardDefinitionId::new(1),
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
    static GRANT: EffectDef = EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&GRANTED),
    };
    static DELAYED_GRANT: EffectDef =
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
            "At the beginning of your next end step, grant an ability.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            GRANT,
        )));
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
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            count: 257,
        }
    );
}

#[test]
fn replacement_program_grants_count_toward_the_structural_address_space() {
    static GRANTED: AbilityDef = AbilityDef::not_implemented(
        "A granted ability.",
        "The test only needs a reusable definition.",
    );
    static GRANT: EffectDef = EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&GRANTED),
    };
    let replacement_effects =
        Box::leak(vec![ReplacementEffectDef::Perform(&GRANT); 257].into_boxed_slice());
    let abilities = Box::leak(
        vec![
            AbilityDef::replacement(
                "This replacement performs many ability grants.",
                ReplacementEffectDef::Sequence(replacement_effects),
            )
            .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::Unsupported))
            .with_coverage(AbilityCoverageDef::explained_complete(
                "This structural-capacity test does not execute the replacement program.",
            )),
        ]
        .into_boxed_slice(),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(abilities);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::TooManyAbilityGrantSites {
            definition: CardDefinitionId::new(1),
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
            definition: CardDefinitionId::new(1),
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
            effect: AppliedEffectDef::add_ability(&INVALID),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    );

    assert_eq!(
        error(definition_granting(&CHILD)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId::new(1),
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
        effect: AppliedEffectDef::add_ability(&INVALID),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    };
    static CHILD: AbilityDef = AbilityDef::activated(
        "Sacrifice a permanent, then grant an ability.",
        &[],
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::Any,
            then: Some(&THEN),
            amount: crate::card::SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    );

    assert_eq!(
        error(definition_granting(&CHILD)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY, GrantId::PRIMARY],
            problem: GrantedAbilityValidationError::EmptyText,
        }
    );
}

#[test]
fn granted_ability_validation_follows_replacement_programs() {
    static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
    static GRANT: EffectDef = EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&INVALID),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    };
    static PROGRAM: [ReplacementEffectDef; 2] = [
        ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ReplacementEffectDef::Perform(&GRANT),
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::replacement(
        "Replace an event, then grant an ability.",
        ReplacementEffectDef::Sequence(&PROGRAM),
    )
    .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::Unsupported))
    .with_coverage(AbilityCoverageDef::explained_complete(
        "This structural grant-validation test does not execute the replacement program.",
    ))];
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(&ABILITIES);
    set_primary_rules(&mut card, &rules);

    assert_eq!(
        error(card),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            grant_path: vec![GrantId::PRIMARY],
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
                effect: AppliedEffectDef::add_ability(&VALID),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::spell(
            "The second mode grants an invalid ability.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&INVALID),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ];
    static GRANTED_MODAL: AbilityDef = AbilityDef::choose_one_spell("Choose one.", &MODES);

    assert_eq!(
        error(definition_granting(&GRANTED_MODAL)),
        CatalogError::InvalidGrantedAbility {
            definition: CardDefinitionId::new(1),
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
                    effect: AppliedEffectDef::add_ability(&TERMINAL),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
            definition: CardDefinitionId::new(1),
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
                definition: CardDefinitionId::new(1),
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
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            target: TargetIndex(1),
            target_count: 1,
        }
    );
}

// Choice scopes, replacements, triggers, and positional limits share the
// catalog fixtures above but form a separate validation surface.
include!("abilities_grants/program_scopes.rs");

include!("abilities_grants/effect_continuations.rs");
