use super::*;
use crate::card::catalog::MismatchedAlternativeCost;

#[test]
fn every_structure_family_rejects_undefined_or_repeated_parts() {
    let invalid_structures = [
        CardStructure::Single {
            main: CardPartId(9),
        },
        CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(9)],
            fused: None,
        },
        CardStructure::Flip {
            normal: CardPartId::PRIMARY,
            flipped: CardPartId(9),
        },
        CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(9),
            kind: DoubleFacedKind::Transforming,
        },
        CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(9),
            kind: AlternateSpellKind::Adventure,
        },
        CardStructure::MeldPart {
            front: CardPartId(9),
            recipe: MeldRecipeId(1),
        },
    ];
    for structure in invalid_structures {
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        card.structure = structure;
        assert!(matches!(
            error(card),
            CatalogError::UndefinedStructurePart {
                definition,
                part: CardPartId(9),
            } if definition == CardDefinitionId::new(1)
        ));
    }

    let mut repeated = definition(1, "Test Card", CardSet::Alpha);
    repeated.structure = CardStructure::Flip {
        normal: CardPartId::PRIMARY,
        flipped: CardPartId::PRIMARY,
    };
    assert_eq!(
        error(repeated),
        CatalogError::DuplicateStructurePart {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
        }
    );
}

#[test]
fn spell_forms_must_reference_defined_structural_parts() {
    let mut undefined = definition(1, "Test Card", CardSet::Alpha);
    undefined.play_options[0].form = SpellForm::Part(CardPartId(9));
    assert_eq!(
        error(undefined),
        CatalogError::UndefinedSpellFormPart {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            part: CardPartId(9),
        }
    );

    let mut empty = split_definition(Some(PlayOptionId(2)));
    empty.play_options.push(PlayOptionDef::cast(
        PlayOptionId(2),
        "Left // Right",
        SpellForm::Combined(Vec::new()),
        ManaCost::default(),
        CardEffectStatus::MetadataOnly,
    ));
    assert_eq!(
        error(empty),
        CatalogError::EmptySpellForm {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId(2),
        }
    );
}

#[test]
fn fused_option_must_exist_and_match_all_split_parts_in_printed_order() {
    assert_eq!(
        error(split_definition(Some(PlayOptionId(2)))),
        CatalogError::MissingFusedPlayOption {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId(2),
        }
    );

    let mut reversed = split_definition(Some(PlayOptionId(2)));
    reversed.play_options.push(PlayOptionDef::cast(
        PlayOptionId(2),
        "Right // Left",
        SpellForm::Combined(vec![CardPartId(1), CardPartId::PRIMARY]),
        ManaCost::default(),
        CardEffectStatus::MetadataOnly,
    ));
    assert!(matches!(
        error(reversed),
        CatalogError::InvalidFusedPlayOption {
            expected,
            actual: SpellForm::Combined(actual),
            ..
        } if expected == vec![CardPartId::PRIMARY, CardPartId(1)]
            && actual == vec![CardPartId(1), CardPartId::PRIMARY]
    ));

    let mut undeclared = split_definition(None);
    undeclared.play_options.push(PlayOptionDef::cast(
        PlayOptionId(2),
        "Left // Right",
        SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
        ManaCost::default(),
        CardEffectStatus::MetadataOnly,
    ));
    assert_eq!(
        error(undeclared),
        CatalogError::UnexpectedCombinedSpellForm {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId(2),
        }
    );
}

#[test]
fn mode_and_alternative_cost_ids_are_local_to_options() {
    let modes = ModeSetDef::choose_one(vec![mode(3, Vec::new()), mode(3, Vec::new())]);
    let mut duplicate_mode = definition(1, "Test Card", CardSet::Alpha);
    duplicate_mode.play_options[0].modes = Some(modes);
    assert_eq!(
        error(duplicate_mode),
        CatalogError::DuplicateModeId {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: ModeId(3),
        }
    );

    let mut nonpositional_mode = definition(1, "Test Card", CardSet::Alpha);
    nonpositional_mode.play_options[0].modes =
        Some(ModeSetDef::choose_one(vec![mode(3, Vec::new())]));
    assert_eq!(
        error(nonpositional_mode),
        CatalogError::NonPositionalModeId {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            expected: ModeId(0),
            actual: ModeId(3),
        }
    );

    let mut duplicate_alternative = definition(1, "Test Card", CardSet::Alpha);
    duplicate_alternative.play_options[0].alternative_costs = vec![
        AlternativeCostDef {
            id: AlternativeCostId(4),
            label: "first".into(),
            mana_cost: ManaCost::default(),
        },
        AlternativeCostDef {
            id: AlternativeCostId(4),
            label: "second".into(),
            mana_cost: ManaCost::default(),
        },
    ];
    assert_eq!(
        error(duplicate_alternative),
        CatalogError::DuplicateAlternativeCostId {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            cost: AlternativeCostId(4),
        }
    );

    let mut alternatives_on_distinct_options = split_definition(None);
    for option in &mut alternatives_on_distinct_options.play_options {
        option.alternative_costs.push(AlternativeCostDef {
            id: AlternativeCostId(4),
            label: "Generic alternative".into(),
            mana_cost: ManaCost::default(),
        });
    }
    CardCatalog::new([alternatives_on_distinct_options])
        .expect("alternative-cost identities are local to a play option");

    let mut duplicate_additional = definition(1, "Test Card", CardSet::Alpha);
    duplicate_additional.play_options[0].additional_costs = vec![
        AdditionalCostDef {
            id: AdditionalCostId(5),
            label: "first".into(),
            mana_cost: None,
            repeatable: false,
        },
        AdditionalCostDef {
            id: AdditionalCostId(5),
            label: "second".into(),
            mana_cost: None,
            repeatable: false,
        },
    ];
    assert_eq!(
        error(duplicate_additional),
        CatalogError::DuplicateAdditionalCostId {
            definition: CardDefinitionId::new(1),
            cost: AdditionalCostId(5),
        }
    );
}

#[test]
fn alternative_cast_ability_requires_its_derived_cost_projection() {
    let flashback_cost = ManaCost {
        generic: 2,
        blue: 1,
        ..ManaCost::default()
    };
    let missing_abilities = Box::leak(
        vec![AbilityDef::alternative_cast(
            flashback_cost,
            AlternativeCastKindDef::Flashback,
            None,
            EffectDef::None,
        )]
        .into_boxed_slice(),
    );
    let mut missing = definition(1, "Test Card", CardSet::Alpha);
    let rules =
        crate::CardRules::new_instant(ManaCost::default()).with_abilities(missing_abilities);
    set_primary_rules(&mut missing, &rules);
    assert_eq!(
        error(missing),
        CatalogError::MissingAlternativeCostForAbility {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            cost: AlternativeCostId(AbilityId::PRIMARY.0),
        }
    );

    let projected_abilities = Box::leak(
        vec![
            AbilityDef::spell("Draw a card.", EffectDef::None),
            AbilityDef::alternative_cast(
                flashback_cost,
                AlternativeCastKindDef::Flashback,
                None,
                EffectDef::None,
            ),
        ]
        .into_boxed_slice(),
    );
    let mut projected = definition(1, "Test Card", CardSet::Alpha);
    projected.play_options[0]
        .alternative_costs
        .push(AlternativeCostDef {
            id: AlternativeCostId(1),
            label: "Flashback".into(),
            mana_cost: flashback_cost,
        });
    let rules =
        crate::CardRules::new_instant(ManaCost::default()).with_abilities(projected_abilities);
    set_primary_rules(&mut projected, &rules);
    CardCatalog::new([projected.clone()])
        .expect("the ability's positional ID derives its matching cost projection");

    let mut mismatched_label = projected.clone();
    mismatched_label.play_options[0].alternative_costs[0].label = "Overload".into();
    assert_eq!(
        error(mismatched_label),
        CatalogError::MismatchedAlternativeCostForAbility(Box::new(MismatchedAlternativeCost {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId(1),
            option: PlayOptionId::DEFAULT,
            cost: AlternativeCostId(1),
            expected_label: "Flashback".into(),
            actual_label: "Overload".into(),
            expected_mana_cost: flashback_cost,
            actual_mana_cost: flashback_cost,
        }))
    );

    let mut mismatched_mana = projected;
    mismatched_mana.play_options[0].alternative_costs[0].mana_cost = ManaCost::default();
    assert_eq!(
        error(mismatched_mana),
        CatalogError::MismatchedAlternativeCostForAbility(Box::new(MismatchedAlternativeCost {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId(1),
            option: PlayOptionId::DEFAULT,
            cost: AlternativeCostId(1),
            expected_label: "Flashback".into(),
            actual_label: "Flashback".into(),
            expected_mana_cost: flashback_cost,
            actual_mana_cost: ManaCost::default(),
        }))
    );
}

#[test]
fn incomplete_alternative_cast_ability_remains_non_executable_catalog_metadata() {
    let alternative = AlternativeCostId(1);
    let abilities = Box::leak(
        vec![
            AbilityDef::spell("Draw a card.", EffectDef::None),
            AbilityDef::alternative_cast(
                ManaCost::default(),
                AlternativeCastKindDef::Overload,
                Some("Draw a card for each opponent."),
                EffectDef::None,
            )
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Test-only incomplete overload.",
            )),
        ]
        .into_boxed_slice(),
    );
    let mut definition = definition(1, "Test Card", CardSet::Alpha);
    definition.play_options[0]
        .alternative_costs
        .push(AlternativeCostDef {
            id: alternative,
            label: "Overload".into(),
            mana_cost: ManaCost::default(),
        });
    let rules = crate::CardRules::new_instant(ManaCost::default()).with_abilities(abilities);
    set_primary_rules(&mut definition, &rules);

    let catalog = CardCatalog::new([definition]).expect("incomplete clauses stay cataloged");
    let stored = catalog.get(CardDefinitionId::new(1)).unwrap();
    assert_eq!(
        stored.implementation_status(),
        crate::ImplementationStatus::Partial,
    );
    assert!(
        !stored.parts[0]
            .rules
            .ability(AbilityId(1))
            .unwrap()
            .is_executable(),
    );
}

#[test]
fn mode_and_target_cardinality_bounds_are_sane() {
    let mut invalid_modes = definition(1, "Test Card", CardSet::Alpha);
    invalid_modes.play_options[0].modes = Some(ModeSetDef {
        minimum: 2,
        maximum: 1,
        may_repeat: false,
        modes: vec![mode(0, Vec::new()), mode(1, Vec::new())],
        conditional_maximum: None,
    });
    assert_eq!(
        error(invalid_modes),
        CatalogError::InvalidModeBounds {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            minimum: 2,
            maximum: 1,
        }
    );

    let mut too_many_modes = definition(1, "Test Card", CardSet::Alpha);
    too_many_modes.play_options[0].modes = Some(ModeSetDef {
        minimum: 1,
        maximum: 2,
        may_repeat: false,
        modes: vec![mode(0, Vec::new())],
        conditional_maximum: None,
    });
    assert_eq!(
        error(too_many_modes),
        CatalogError::TooManyModesWithoutRepetition {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            maximum: 2,
            available: 1,
        }
    );

    let mut invalid_targets = definition(1, "Test Card", CardSet::Alpha);
    invalid_targets.play_options[0].targets = vec![target(0, 2, 1)];
    assert_eq!(
        error(invalid_targets),
        CatalogError::InvalidTargetBounds {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: None,
            slot: TargetSlotId(0),
            minimum: 2,
            maximum: 1,
        }
    );
}

#[test]
fn semantic_spell_modes_require_matching_presentation_mode_ids() {
    let valid = semantic_modal_definition(
        vec![semantic_mode(Vec::new())],
        Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
    );
    CardCatalog::new([valid]).unwrap();

    let missing_presentation = semantic_modal_definition(vec![semantic_mode(Vec::new())], None);
    assert_eq!(
        error(missing_presentation),
        CatalogError::MissingPresentationSpellMode {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: ModeId(0),
        }
    );

    let missing_semantic = semantic_modal_definition(
        vec![semantic_mode(Vec::new())],
        Some(ModeSetDef::choose_one(vec![
            mode(0, Vec::new()),
            mode(1, Vec::new()),
        ])),
    );
    assert_eq!(
        error(missing_semantic),
        CatalogError::MissingSemanticSpellMode {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: ModeId(1),
        }
    );
}

#[test]
fn semantic_modal_spell_selection_must_be_possible() {
    let definition = semantic_spell_definition(&AbilityDef::modal_spell("Choose one —", &[]), None);

    assert_eq!(
        error(definition),
        CatalogError::InvalidModalSpellSelection {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId(0),
            minimum: 1,
            maximum: 1,
            may_repeat: false,
            available: 0,
        }
    );
}

#[test]
fn executable_nonmodal_spells_reject_presentation_modes() {
    let definition = semantic_spell_definition(
        &AbilityDef::spell("Do the thing.", EffectDef::None),
        Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
    );

    assert_eq!(
        error(definition),
        CatalogError::UnexpectedPresentationSpellModes {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
        }
    );
}

#[test]
fn nonmodal_spell_target_presentations_are_derived_positionally() {
    let targets = Box::leak(vec![semantic_target(1, 1)].into_boxed_slice());
    let ability = AbilityDef::spell_with_targets("Target something.", targets, EffectDef::None);
    let missing = semantic_spell_definition(&ability, None);

    assert_eq!(
        error(missing),
        CatalogError::MissingPresentationSpellTarget {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            target: TargetSlotId(0),
        }
    );

    let mut valid = semantic_spell_definition(&ability, None);
    valid.play_options[0].targets = vec![target(0, 1, 1)];
    CardCatalog::new([valid.clone()]).expect("the positional projection matches");

    let mut mismatched = valid;
    mismatched.play_options[0].targets[0].predicate = TargetPredicate::Player;
    assert!(matches!(
        error(mismatched),
        CatalogError::MismatchedSpellTargetPresentation {
            definition,
            option: PlayOptionId::DEFAULT,
            position: 0,
            ..
        } if definition == CardDefinitionId::new(1)
    ));
}

#[test]
fn unpresentable_nonmodal_targets_use_only_the_semantic_runtime_definition() {
    let targets = Box::leak(
        vec![AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[],
                controller: None,
                owner: None,
            },
        )]
        .into_boxed_slice(),
    );
    let ability = AbilityDef::spell_with_targets("Target a card.", targets, EffectDef::None);
    let semantic_only = semantic_spell_definition(&ability, None);
    CardCatalog::new([semantic_only.clone()])
        .expect("an empty presentation leaves semantic runtime targeting authoritative");

    let mut approximated = semantic_only;
    approximated.play_options[0].targets = vec![target(0, 1, 1)];
    assert_eq!(
        error(approximated),
        CatalogError::UnpresentableSpellTarget {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            target: TargetSlotId(0),
        }
    );
}

#[test]
fn unpresentable_modal_targets_use_only_the_semantic_runtime_definition() {
    let semantic = semantic_mode(vec![AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(crate::CardType::Creature),
            zones: &[crate::ZoneKind::Graveyard],
            controller: None,
            owner: Some(PlayerRelation::You),
        },
    )]);
    let presentation = ModeSetDef::choose_one(vec![mode(0, Vec::new())]);
    let semantic_only = semantic_modal_definition(vec![semantic], Some(presentation));
    CardCatalog::new([semantic_only.clone()])
        .expect("an empty modal projection leaves semantic runtime targeting authoritative");

    let mut approximated = semantic_only;
    approximated.play_options[0]
        .modes
        .as_mut()
        .expect("the test supplied modal presentation")
        .modes[0]
        .targets = vec![target(0, 1, 1)];
    assert_eq!(
        error(approximated),
        CatalogError::UnpresentableSpellModeTarget {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: ModeId(0),
            target: TargetSlotId(0),
        }
    );
}

#[test]
fn combined_play_options_reject_modal_constituent_parts() {
    static MODES: [AbilityDef; 1] = [AbilityDef::spell("Test mode.", EffectDef::None)];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::modal_spell("Choose one.", &MODES)];
    let modal_rules = crate::CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
    let mut definition = split_definition(Some(PlayOptionId(2)));
    definition.rules = modal_rules;
    definition.parts[0].rules = modal_rules;
    let option = PlayOptionDef::cast(
        PlayOptionId(2),
        "Left // Right",
        SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
        ManaCost::default(),
        CardEffectStatus::MetadataOnly,
    );

    assert_eq!(
        validate_semantic_spell_presentation(&definition, &option),
        Err(CatalogError::CombinedModalSpellUnsupported {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId(2),
            part: CardPartId::PRIMARY,
        })
    );
}

#[test]
fn semantic_modal_spells_keep_targets_on_their_branches() {
    let mut definition = semantic_modal_definition(
        vec![semantic_mode(Vec::new())],
        Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
    );
    definition.play_options[0].targets = vec![target(0, 1, 1)];

    assert_eq!(
        error(definition),
        CatalogError::UnexpectedModalSpellTargets {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            count: 1,
        }
    );
}

#[test]
fn semantic_spell_mode_selection_rules_cannot_drift_from_presentation() {
    let mismatched = semantic_modal_definition(
        vec![semantic_mode(Vec::new()), semantic_mode(Vec::new())],
        Some(ModeSetDef {
            minimum: 1,
            maximum: 2,
            may_repeat: true,
            modes: vec![mode(0, Vec::new()), mode(1, Vec::new())],
            conditional_maximum: None,
        }),
    );

    assert_eq!(
        error(mismatched),
        CatalogError::MismatchedSpellModeSelection {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            presentation_minimum: 1,
            presentation_maximum: 2,
            presentation_may_repeat: true,
            semantic_minimum: 1,
            semantic_maximum: 1,
            semantic_may_repeat: false,
        }
    );
}

#[test]
fn semantic_spell_mode_targets_require_matching_positions_and_cardinalities() {
    let valid = semantic_modal_definition(
        vec![semantic_mode(vec![semantic_target(1, 1)])],
        Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 1, 1)])])),
    );
    CardCatalog::new([valid]).unwrap();

    let missing_presentation = semantic_modal_definition(
        vec![semantic_mode(vec![semantic_target(1, 1)])],
        Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
    );
    assert_eq!(
        error(missing_presentation),
        CatalogError::MissingPresentationSpellModeTarget {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: ModeId(0),
            target: TargetSlotId(0),
        }
    );

    let missing_semantic = semantic_modal_definition(
        vec![semantic_mode(Vec::new())],
        Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 1, 1)])])),
    );
    assert_eq!(
        error(missing_semantic),
        CatalogError::MissingSemanticSpellModeTarget {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: ModeId(0),
            target: TargetSlotId(0),
        }
    );

    let mismatched_cardinality = semantic_modal_definition(
        vec![semantic_mode(vec![semantic_target(1, 1)])],
        Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 0, 1)])])),
    );
    assert_eq!(
        error(mismatched_cardinality),
        CatalogError::MismatchedSpellModeTargetCardinality {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            mode: ModeId(0),
            target: TargetSlotId(0),
            presentation_minimum: 0,
            presentation_maximum: 1,
            semantic_minimum: 1,
            semantic_maximum: 1,
        }
    );
}

#[test]
fn semantic_spell_mode_presentation_matches_branch_order_and_predicates() {
    let reordered = semantic_modal_definition(
        vec![semantic_mode(vec![
            semantic_target(1, 1),
            semantic_target(1, 1),
        ])],
        Some(ModeSetDef::choose_one(vec![mode(
            0,
            vec![target(1, 1, 1), target(0, 1, 1)],
        )])),
    );
    assert!(matches!(
        error(reordered),
        CatalogError::NonPositionalTargetSlot {
            mode: Some(ModeId(0)),
            expected: TargetSlotId(0),
            actual: TargetSlotId(1),
            ..
        }
    ));

    let mut wrong_predicate = semantic_modal_definition(
        vec![semantic_mode(vec![semantic_target(1, 1)])],
        Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 1, 1)])])),
    );
    wrong_predicate.play_options[0]
        .modes
        .as_mut()
        .unwrap()
        .modes[0]
        .targets[0]
        .predicate = TargetPredicate::Player;
    assert!(matches!(
        error(wrong_predicate),
        CatalogError::MismatchedSpellModeTargetPresentation {
            mode: ModeId(0),
            position: 0,
            ..
        }
    ));

    let mut wrong_label = semantic_modal_definition(
        vec![semantic_mode(Vec::new())],
        Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
    );
    wrong_label.play_options[0].modes.as_mut().unwrap().modes[0].label = "different mode".into();
    assert!(matches!(
        error(wrong_label),
        CatalogError::MismatchedSpellModeLabel {
            mode: ModeId(0),
            ..
        }
    ));

    let mut wrong_cost = semantic_modal_definition(
        vec![semantic_mode(Vec::new())],
        Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
    );
    wrong_cost.play_options[0].modes.as_mut().unwrap().modes[0].additional_mana_cost =
        Some(ManaCost::new(1, 0));
    assert!(matches!(
        error(wrong_cost),
        CatalogError::MismatchedSpellModeAdditionalManaCost {
            mode: ModeId(0),
            presentation,
            semantic,
            ..
        } if presentation.is_some() && semantic.is_none()
    ));
}

#[test]
fn metadata_only_presentation_modes_do_not_require_semantic_modes() {
    let mut card = definition(1, "Metadata-Only Modal Spell", CardSet::Alpha);
    card.play_options[0].modes = Some(ModeSetDef::choose_one(vec![
        mode(0, vec![target(0, 1, 1)]),
        mode(1, Vec::new()),
    ]));

    CardCatalog::new([card]).unwrap();
}

#[test]
fn composed_target_count_fits_the_runtime_slot_space() {
    let targets = || (0_u8..200).map(|id| target(id, 1, 1)).collect::<Vec<_>>();
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    card.play_options[0].modes = Some(ModeSetDef {
        minimum: 2,
        maximum: 2,
        may_repeat: false,
        modes: vec![mode(0, targets()), mode(1, targets())],
        conditional_maximum: None,
    });

    assert_eq!(
        error(card),
        CatalogError::TooManyInstantiatedTargets {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
            count: 400,
        }
    );
}

#[test]
fn modal_target_slots_are_local_to_each_selected_occurrence() {
    let mutually_exclusive = ModeSetDef::choose_one(vec![
        mode(0, vec![target(0, 1, 1)]),
        mode(1, vec![target(0, 1, 1)]),
    ]);
    let mut valid = definition(1, "Test Card", CardSet::Alpha);
    valid.play_options[0].modes = Some(mutually_exclusive);
    CardCatalog::new([valid]).unwrap();

    let mut coexisting = definition(1, "Test Card", CardSet::Alpha);
    coexisting.play_options[0].modes = Some(ModeSetDef {
        minimum: 2,
        maximum: 2,
        may_repeat: false,
        modes: vec![
            mode(0, vec![target(0, 1, 1)]),
            mode(1, vec![target(0, 1, 1)]),
        ],
        conditional_maximum: None,
    });
    CardCatalog::new([coexisting]).unwrap();

    let mut repeatable = definition(1, "Test Card", CardSet::Alpha);
    repeatable.play_options[0].modes = Some(ModeSetDef {
        minimum: 2,
        maximum: 2,
        may_repeat: true,
        modes: vec![mode(0, vec![target(0, 1, 1)])],
        conditional_maximum: None,
    });
    CardCatalog::new([repeatable]).unwrap();
}
