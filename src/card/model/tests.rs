use super::{
    AbilityCostDef, AbilityCostList, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AlternativeCastKindDef, AlternativeCostDef, CardBehavior, CardComposition,
    CardDefinition, CardEffectStatus, CardPart, CardPrinting, CardPrintingId, CardRules, CardSet,
    CardType, CardTypeSet, CreatureStats, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    ImplementationStatus, ManaColor, ManaCost, ManaCostParseErrorKind, ManaRestrictionDef,
    ManaSelectionDef, ObjectPredicateDef, PlayOptionDef, PlayerRelation, PrintedManaCost,
    SpellForm, TargetPredicate, TriggerEventDef, ZoneKind,
};
use crate::{
    AbilityId, AlternativeCostId, CardDefinitionId, CardPartId, ModeId, PlayOptionId, TargetIndex,
};

static DEFERRED_CLAUSE: [AbilityDef; 1] = [AbilityDef::not_implemented(
    "A deferred card-specific ability.",
    "The card-specific ability is not executed.",
)];

#[test]
fn ability_cost_list_equality_and_hash_ignore_storage_representation() {
    use std::collections::{HashSet, hash_map::DefaultHasher};
    use std::hash::{Hash, Hasher};

    static COSTS: [AbilityCostDef; 2] = [
        AbilityCostDef::Mana(ManaCost::new(2, 0)),
        AbilityCostDef::DiscardSource,
    ];
    let borrowed = AbilityCostList::borrowed(&COSTS);
    let inline = AbilityCostList::two(COSTS[0], COSTS[1]);

    let hash = |costs: AbilityCostList| {
        let mut hasher = DefaultHasher::new();
        costs.hash(&mut hasher);
        hasher.finish()
    };

    assert_eq!(borrowed, inline);
    assert_eq!(hash(borrowed), hash(inline));
    assert!(HashSet::from([borrowed]).contains(&inline));
}

#[test]
fn modal_spell_semantics_derive_their_presentation_modes() {
    const RULES: CardRules = CardRules::new_instant(crate::mana_cost!("{0}")).with_ability(
        AbilityDef::choose_one_spell(
            "Choose one.",
            &[
                AbilityDef::counter_target(
                    "Counter target blue spell",
                    &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(
                        ManaColor::Blue,
                    )),
                ),
                AbilityDef::destroy_target(
                    "Destroy target blue permanent",
                    &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                        ManaColor::Blue,
                    )),
                    true,
                ),
                AbilityDef::spell_with_targets(
                    "Return target creature card from your graveyard.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::None,
                ),
            ],
        ),
    );
    let rules = RULES;
    let composition = CardComposition::single("Test Modal Spell", rules);
    let modes = composition.play_options[0]
        .modes
        .as_ref()
        .expect("semantic modes synthesize the presentation choices");

    assert_eq!(modes.minimum, 1);
    assert_eq!(modes.maximum, 1);
    assert!(!modes.may_repeat);
    assert_eq!(modes.modes[0].id, ModeId(0));
    assert_eq!(modes.modes[0].label, "Counter target blue spell");
    assert_eq!(modes.modes[0].targets[0].predicate, TargetPredicate::Spell);
    assert_eq!(modes.modes[1].label, "Destroy target blue permanent");
    assert_eq!(
        modes.modes[1].targets[0].predicate,
        TargetPredicate::Permanent
    );
    assert_eq!(
        modes.modes[2].label,
        "Return target creature card from your graveyard."
    );
    assert!(
        modes.modes[2].targets.is_empty(),
        "semantic-only mode targets keep an empty legacy projection"
    );
    assert_eq!(
        match rules.ability_clauses()[0].definition {
            DeclarativeAbilityDef::Spell(spell) => spell.mode(ModeId(0)),
            _ => None,
        }
        .expect("first positional mode")
        .effect
        .definition,
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
        }
    );
    assert_eq!(
        match rules.ability_clauses()[0].definition {
            DeclarativeAbilityDef::Spell(spell) => spell.mode(ModeId(1)),
            _ => None,
        }
        .expect("second positional mode")
        .effect
        .definition,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        }
    );
    assert_eq!(rules.rules_text(), "Choose one.");
}

#[test]
fn semantic_target_labels_are_derived_from_predicates() {
    let opponent =
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent));
    assert_eq!(opponent.label(), "target opponent");

    let creature_you_control = AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    });
    assert_eq!(creature_you_control.label(), "target creature you control");

    let constrained_creature = AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Special("creature with toughness less than the source's power"),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    });
    assert_eq!(
        constrained_creature.label(),
        "target creature you control with toughness less than the source's power"
    );

    let non_demon = AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Demon")),
    ]));
    assert_eq!(non_demon.label(), "target non-Demon creature");

    let not_red_land = AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
        &ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Color(ManaColor::Red),
        ]),
    ));
    assert_eq!(
        not_red_land.label(),
        "target permanent",
        "a conservative label must not turn 'not a red land' into 'nonland'"
    );

    let graveyard = AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    });
    assert_eq!(graveyard.label(), "target creature card in your graveyard");
    assert!(
        graveyard.presentation(crate::TargetSlotId(0)).is_none(),
        "semantic-only targets still have decision labels without a legacy projection",
    );

    let blue_spell =
        AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Blue));
    let presentation = blue_spell
        .presentation(crate::TargetSlotId(0))
        .expect("a stack target has a presentation projection");
    assert_eq!(blue_spell.label(), "target blue spell");
    assert_eq!(presentation.label, blue_spell.label());
}

#[test]
fn printing_ids_distinguish_variants_within_one_set() {
    let definition = CardDefinitionId(7);
    let primary = CardPrintingId::new(definition, CardSet::Alpha);
    let alternate = CardPrintingId::with_variant(definition, CardSet::Alpha, 1);

    assert_eq!(primary.variant, 0);
    assert_ne!(primary, alternate);
    assert_eq!(
        CardPrinting::with_variant(definition, CardSet::Alpha, 1).id,
        alternate
    );
}

#[test]
fn definitions_start_with_their_primary_printing() {
    let id = CardDefinitionId(7);
    let definition = CardDefinition::new(
        id,
        "Test Card",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
    );

    assert_eq!(
        definition.printings,
        vec![CardPrinting::new(id, CardSet::Alpha)]
    );
}

#[test]
fn planeswalkers_are_permanents() {
    let types = CardTypeSet::single(CardType::Planeswalker);
    assert!(types.is_permanent());
    assert!(!types.is_creature());
}

#[test]
fn artifact_creatures_have_both_card_types() {
    let rules = CardRules::new_artifact_creature(mana_cost!("{3}"), &["Golem"], 3, 3);
    assert!(rules.has_type(CardType::Artifact));
    assert!(rules.has_type(CardType::Creature));
    assert!(!rules.has_type(CardType::Enchantment));
    assert_eq!(rules.kind_name(), "ArtifactCreature");
    assert_eq!(rules.type_line(), "Artifact Creature — Golem");
}

#[test]
fn composable_types_cover_magic_card_type_combinations() {
    let enchantment_creature = CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 2, 2)
        .with_type(CardType::Enchantment);
    assert_eq!(
        enchantment_creature.type_line(),
        "Enchantment Creature — Dryad"
    );

    let artifact_land = CardRules::new_land(&[]).with_type(CardType::Artifact);
    assert_eq!(artifact_land.type_line(), "Artifact Land");

    let land_creature = CardRules::new_land(&[])
        .with_type(CardType::Creature)
        .with_subtypes(&["Forest", "Dryad"])
        .with_creature_stats(CreatureStats {
            power: 1,
            toughness: 1,
        });
    assert_eq!(land_creature.type_line(), "Land Creature — Forest Dryad");
    assert_eq!(
        CardComposition::single("Land creature", land_creature).play_options[0].action,
        super::PlayActionKind::PlayLand
    );
}

#[test]
fn white_red_hybrid_costs_have_both_printed_colors() {
    let rules = CardRules::new_creature(mana_cost!("{R/W}{R/W}{R/W}"), &[], 1, 1);
    assert_eq!(rules.colors(), [true, false, false, true, false]);
}

#[test]
fn symbolic_mana_costs_parse_at_compile_time_and_runtime() {
    const COMPILED: ManaCost = mana_cost!("{2}{G}{G}");
    assert_eq!(COMPILED, "{2}{G}{G}".parse().unwrap());
    assert_eq!(COMPILED.generic, 2);
    assert_eq!(COMPILED.green, 2);
    assert_eq!(mana_cost!("{X}{X}{U}").x_multiplier, 2);
    assert_eq!(mana_cost!("{0}"), ManaCost::default());
    assert_eq!(mana_cost!("{0}").to_string(), "{0}");
    assert_eq!(
        mana_cost!("{12}{X}{X}{W}{U}{B}{R}{G}{R/W}").to_string(),
        "{12}{X}{X}{W}{U}{B}{R}{G}{R/W}",
    );
}

#[test]
fn alternative_cast_clauses_render_and_project_their_owned_costs() {
    static ABILITIES: [AbilityDef; 3] = [
        AbilityDef::spell("Draw a card.", EffectDef::None),
        AbilityDef::alternative_cast(
            mana_cost!("{2}{U}"),
            AlternativeCastKindDef::Flashback,
            None,
            EffectDef::None,
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{3}{R}"),
            AlternativeCastKindDef::Overload,
            Some("Draw a card for each opponent."),
            EffectDef::None,
        ),
    ];
    let rules = CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&ABILITIES);

    assert_eq!(ABILITIES[1].text, "Flashback");
    assert_eq!(
        ABILITIES[1].rules_text(),
        "Flashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
    );
    assert_eq!(
        ABILITIES[2].rules_text(),
        "Overload {3}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
    );
    assert_eq!(
        rules.rules_text(),
        concat!(
            "Draw a card.\n",
            "Flashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)\n",
            "Overload {3}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
        ),
    );

    let composition = CardComposition::single("Test spell", rules);
    assert_eq!(
        composition.play_options[0].alternative_costs,
        vec![
            AlternativeCostDef {
                id: AlternativeCostId(1),
                label: "Flashback".into(),
                mana_cost: mana_cost!("{2}{U}"),
            },
            AlternativeCostDef {
                id: AlternativeCostId(2),
                label: "Overload".into(),
                mana_cost: mana_cost!("{3}{R}"),
            },
        ],
    );

    let mut generic = PlayOptionDef::cast(
        PlayOptionId(4),
        "Generic alternative",
        SpellForm::Part(CardPartId::PRIMARY),
        mana_cost!("{1}{U}"),
        CardEffectStatus::Implemented,
    );
    generic.alternative_costs.push(AlternativeCostDef {
        id: AlternativeCostId(9),
        label: "Generic".into(),
        mana_cost: mana_cost!("{U}"),
    });
    let projected = generic.with_alternative_cast_costs(&rules);
    assert_eq!(projected.alternative_costs[0].label, "Generic");
    assert_eq!(projected.alternative_costs.len(), 3);
}

#[test]
fn symbolic_mana_costs_reject_invalid_or_unsupported_notation() {
    for (symbols, expected) in [
        ("", ManaCostParseErrorKind::Empty),
        ("2GG", ManaCostParseErrorKind::ExpectedOpeningBrace),
        ("{2", ManaCostParseErrorKind::UnterminatedSymbol),
        ("{}", ManaCostParseErrorKind::EmptySymbol),
        ("{C}", ManaCostParseErrorKind::InvalidSymbol),
        ("{2}{3}", ManaCostParseErrorKind::DuplicateGenericSymbol),
        ("{65536}", ManaCostParseErrorKind::Overflow),
    ] {
        assert_eq!(ManaCost::parse_symbols(symbols).unwrap_err().kind, expected);
    }
}

#[test]
fn clause_implementation_drives_the_ordinary_play_option_gate() {
    let implemented = CardRules::new_instant(ManaCost::default());
    assert_eq!(
        ImplementationStatus::default(),
        ImplementationStatus::Complete
    );
    assert_eq!(
        CardComposition::single("Implemented", implemented).play_options[0].effect_status,
        CardEffectStatus::Implemented
    );

    let uncategorized =
        CardRules::new_instant(ManaCost::default()).with_ability(AbilityDef::not_implemented(
            "Text with no assigned implementation.",
            "The card-specific ability is not executed.",
        ));
    assert_eq!(
        uncategorized.implementation_status(),
        ImplementationStatus::MetadataOnly
    );
    let custom = CardRules::new_instant(ManaCost::default()).with_ability(AbilityDef::custom_full(
        "A card-local effect.",
        CardBehavior::Fireball,
        "Implemented by the named card-local special behavior.",
    ));
    assert_eq!(
        custom.implementation_status(),
        ImplementationStatus::Complete
    );
    assert_eq!(custom.special_behavior(), Some(CardBehavior::Fireball));

    let metadata_only =
        CardRules::new_instant(ManaCost::default()).with_ability(AbilityDef::not_implemented(
            "A deferred spell effect.",
            "The card-specific ability is not executed.",
        ));
    assert_eq!(
        metadata_only.implementation_status(),
        ImplementationStatus::MetadataOnly
    );
    assert_eq!(
        CardComposition::single("Deferred", metadata_only).play_options[0].effect_status,
        CardEffectStatus::MetadataOnly
    );
    let metadata_definition = CardDefinition::new(
        CardDefinitionId(8),
        "Unsupported",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
    );
    assert_eq!(
        metadata_definition.implementation_status(),
        ImplementationStatus::MetadataOnly
    );

    let partial =
        CardRules::new_enchantment(ManaCost::default()).with_ability(AbilityDef::custom_partial(
            "A custom clause with one deferred rider.",
            CardBehavior::Fireball,
            "One rider is deferred.",
        ));
    assert_eq!(
        partial.ability_clauses()[0].coverage.explanation,
        Some("One rider is deferred.")
    );
    assert_eq!(
        partial.implementation_status(),
        ImplementationStatus::Partial
    );
    assert_eq!(
        CardComposition::single("Partial", partial).play_options[0].effect_status,
        CardEffectStatus::Implemented
    );
}

#[test]
fn vanilla_creature_body_is_complete() {
    let rules = CardRules::new_creature(ManaCost::default(), &[], 2, 2);

    assert_eq!(
        rules.implementation_status(),
        ImplementationStatus::Complete
    );
}

#[test]
fn creature_body_with_an_unimplemented_clause_is_partial() {
    let rules =
        CardRules::new_creature(ManaCost::default(), &[], 2, 2).with_abilities(&DEFERRED_CLAUSE);

    assert_eq!(rules.implementation_status(), ImplementationStatus::Partial);
    assert_eq!(
        CardComposition::single("Partial creature", rules).play_options[0].effect_status,
        CardEffectStatus::Implemented
    );
}

#[test]
fn noncreature_with_only_an_unimplemented_clause_is_metadata_only() {
    let rules = CardRules::new_enchantment(ManaCost::default()).with_abilities(&DEFERRED_CLAUSE);

    assert_eq!(
        rules.implementation_status(),
        ImplementationStatus::MetadataOnly
    );
    assert_eq!(
        CardComposition::single("Deferred enchantment", rules).play_options[0].effect_status,
        CardEffectStatus::MetadataOnly
    );
}

#[test]
fn no_mana_cost_is_distinct_from_a_printed_zero_cost() {
    let rules = CardRules::new_sorcery(ManaCost::default());
    let zero = CardPart::new(CardPartId::PRIMARY, "Zero", rules);
    let no_cost_rules = CardRules::new_sorcery_without_mana_cost();
    let none = CardPart::new(CardPartId::PRIMARY, "None", no_cost_rules);

    assert_eq!(
        zero.printed_mana_cost(),
        PrintedManaCost::Cost(ManaCost::default())
    );
    assert_eq!(none.printed_mana_cost(), PrintedManaCost::None);
    assert_eq!(zero.printed_mana_cost().mana_value(), 0);
    assert_eq!(none.printed_mana_cost().mana_value(), 0);

    let composition = CardComposition::single("No-cost spell", no_cost_rules);
    assert_eq!(composition.parts[0].mana_cost(), None);
    assert_eq!(composition.play_options[0].mana_cost, None);
}

#[test]
fn typed_rules_expose_coherent_kind_specific_characteristics() {
    let creature = CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear"], 2, 2);
    assert_eq!(creature.types(), CardTypeSet::single(CardType::Creature));
    assert_eq!(creature.subtypes(), &["Bear"]);
    assert_eq!(
        creature.creature_stats(),
        Some(CreatureStats {
            power: 2,
            toughness: 2,
        })
    );
    assert_eq!(creature.starting_loyalty(), None);
    assert_eq!(creature.coherence_error(), None);

    let land = CardRules::new_land(&["Forest"]);
    assert_eq!(land.types(), CardTypeSet::single(CardType::Land));
    assert_eq!(land.printed_mana_cost(), PrintedManaCost::None);
    assert_eq!(land.creature_stats(), None);
    assert_eq!(land.coherence_error(), None);
}

#[test]
fn coherence_validation_covers_kind_specific_invariants() {
    let mut creature_without_stats = CardRules::new_creature(ManaCost::default(), &["Bear"], 2, 2);
    creature_without_stats.creature_stats = None;

    let mut instant_with_stats = CardRules::new_instant(ManaCost::default());
    instant_with_stats.creature_stats = Some(CreatureStats {
        power: 1,
        toughness: 1,
    });

    let mut instant_with_loyalty = CardRules::new_instant(ManaCost::default());
    instant_with_loyalty.starting_loyalty = Some(3);

    let mut planeswalker_without_loyalty =
        CardRules::new_planeswalker(ManaCost::default(), &["Test"], 3);
    planeswalker_without_loyalty.starting_loyalty = None;

    let permanent_instant =
        CardRules::new_instant(ManaCost::default()).with_type(CardType::Artifact);

    for (rules, expected) in [
        (
            creature_without_stats,
            "a creature must have power and toughness",
        ),
        (
            instant_with_stats,
            "a noncreature cannot have creature power and toughness",
        ),
        (
            instant_with_loyalty,
            "a nonplaneswalker cannot have starting loyalty",
        ),
        (
            planeswalker_without_loyalty,
            "a castable planeswalker face must have starting loyalty",
        ),
        (
            permanent_instant,
            "an instant or sorcery cannot also be a permanent card type",
        ),
    ] {
        assert_eq!(rules.coherence_error(), Some(expected));
    }
}

#[test]
#[should_panic(expected = "with_creature_stats() is only valid for creature rules")]
fn noncreatures_cannot_declare_creature_stats() {
    let _ = CardRules::new_land(&[]).with_creature_stats(CreatureStats {
        power: 1,
        toughness: 1,
    });
}

#[test]
fn ability_category_is_explicit_and_not_inferred_from_effect() {
    const COSTS: &[AbilityCostDef] = &[AbilityCostDef::TapSource];
    const ADD_MANA: EffectDef = EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green));
    const MANA_ABILITY: AbilityDef = AbilityDef::activated_mana("Add green.", COSTS, ADD_MANA);
    const ORDINARY_TRIGGER: AbilityDef = AbilityDef::triggered(
        "Add green when this dies.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Source,
            from: Some(super::ZoneKind::Battlefield),
            to: Some(super::ZoneKind::Graveyard),
        },
        ADD_MANA,
    );
    const TURN_FACE_UP: AbilityDef = AbilityDef::special_action(
        "Turn this face up.",
        &[super::ZoneKind::Battlefield],
        &[AbilityCostDef::Mana(ManaCost::new(3, 0))],
        EffectDef::Special("turn face up"),
    );
    static ABILITIES: [AbilityDef; 3] = [MANA_ABILITY, ORDINARY_TRIGGER, TURN_FACE_UP];

    assert!(!MANA_ABILITY.uses_stack());
    assert!(ORDINARY_TRIGGER.uses_stack());
    assert!(!TURN_FACE_UP.uses_stack());

    let rules = CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&ABILITIES);
    let attached = rules.indexed_abilities().collect::<Vec<_>>();
    assert_eq!(attached[0].id, AbilityId::PRIMARY);
    assert_eq!(attached[1].id, AbilityId(1));
    assert_eq!(attached[2].id, AbilityId(2));
}

#[test]
#[should_panic(expected = "only activated and triggered abilities have a selectable procedure")]
fn legacy_procedure_rejects_ability_categories_without_a_procedure() {
    let _ = AbilityDef::spell("Draw a card.", EffectDef::None).with_legacy_procedure();
}

#[test]
fn mana_effects_keep_restrictions_attached_to_each_counted_unit() {
    const RESTRICTIONS: &[ManaRestrictionDef] = &[ManaRestrictionDef::CastSpell(
        ObjectPredicateDef::HasType(super::CardType::Artifact),
    )];
    let workshop_mana = AddManaEffectDef::one(ManaColor::Colorless)
        .with_amount(3)
        .with_restrictions(RESTRICTIONS);

    assert_eq!(workshop_mana.amount, 3);
    assert_eq!(workshop_mana.restrictions, RESTRICTIONS);
}

#[test]
fn any_color_mana_effect_chooses_from_the_five_colors() {
    let mana = AddManaEffectDef::any_color();

    assert_eq!(mana.mana, ManaSelectionDef::Choice(&ManaColor::COLORS));
    assert_eq!(mana.amount, 1);
    assert!(mana.restrictions.is_empty());
    assert!(mana.spend_effects.is_empty());
}
