use super::*;
use crate::CardEffectStatus;
use crate::ControlDurationDef;
use crate::card::child_effects;

#[test]
fn format_sets_and_card_records_have_catalog_modules() {
    let set_formats = Format::ALL
        .iter()
        .filter_map(|format| format.set_definition())
        .collect::<Vec<_>>();
    let format_sets = set_formats
        .iter()
        .flat_map(|definition| definition.allowed_sets)
        .copied()
        .collect::<Vec<_>>();
    let all_registered_sets = SET_MODULES
        .iter()
        .map(|module| module.set)
        .collect::<Vec<_>>();
    assert_eq!(
        all_registered_sets
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .len(),
        all_registered_sets.len(),
        "each set must have exactly one catalog module",
    );
    assert!(!all_registered_sets.contains(&CardSet::Token));
    for format in Format::ALL {
        let Some(definition) = format.set_definition() else {
            continue;
        };
        assert!(
            !definition.allowed_sets.contains(&CardSet::Token),
            "no format may allow the token set"
        );
    }

    assert!(
        format_sets
            .iter()
            .all(|set| all_registered_sets.contains(set)),
        "every format-supported set must be cataloged",
    );
    for module in SET_MODULES {
        for record in module.cards {
            assert_eq!(
                record.debut_set, module.set,
                "{} is registered in the wrong set",
                record.name
            );
        }
    }
}

#[test]
fn built_in_records_have_unique_identity() {
    const RETIRED_VIRTUAL_OBJECT_IDS: &[u16] = &[
        245, 246, 247, 249, 254, 255, 256, 257, 258, 259, 260, 538, 539, 540, 602, 603, 676, 677,
        678, 679, 840, 841, 842, 963, 964, 1051, 1052, 1053, 1143, 1236, 1237, 1238, 1239, 1350,
        1351, 1481, 1561, 1701, 1705, 1708, 1791, 1893, 2075, 2121, 2147, 2173, 2198, 2205, 2210,
        2214, 2216, 2218, 2224, 2231, 2246, 2249, 2257, 2262, 2281, 2287, 2293, 2295, 2297,
    ];

    let records = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    let record_ids = records
        .iter()
        .map(|record| record.id())
        .collect::<HashSet<_>>();
    assert_eq!(
        record_ids.len(),
        records.len(),
        "definition IDs must remain globally unique",
    );
    assert_eq!(
        crate::card::cards::ALL_CARD_DEFINITION_IDS
            .iter()
            .copied()
            .collect::<HashSet<_>>(),
        record_ids,
        "generated compatibility IDs must match the runtime records",
    );
    for retired in RETIRED_VIRTUAL_OBJECT_IDS {
        assert!(
            records
                .iter()
                .all(|record| record.id() != CardDefinitionId::new(u64::from(*retired))),
            "retired virtual-object definition ID {retired} must remain a tombstone",
        );
    }
    assert_eq!(
        records
            .iter()
            .map(|record| record.name)
            .collect::<HashSet<_>>()
            .len(),
        records.len(),
        "every catalog definition name must remain globally unique",
    );
    assert_eq!(
        records
            .iter()
            .map(|record| record.identity_anchor())
            .collect::<HashSet<_>>()
            .len(),
        records.len(),
        "every catalog definition must have a unique anchor printing",
    );
    for record in records {
        assert!(
            super::is_uuid(record.identity_anchor()),
            "{} has an invalid anchor printing UUID: {}",
            record.name,
            record.identity_anchor(),
        );
    }
}

#[test]
fn modal_spells_derive_their_rules_text_from_modes() {
    for record in SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
    {
        for ability in record.rules.ability_clauses() {
            if !matches!(ability.definition, DeclarativeAbilityDef::Spell(_)) {
                continue;
            }
            if ability.modal().is_none() {
                continue;
            }
            assert!(
                !ability.text.contains("\n•"),
                "{} stores its printed modes both on the modal wrapper and its branches",
                record.name,
            );
        }
    }
}

#[test]
fn virtual_and_face_down_characteristics_are_not_card_catalog_definitions() {
    let synthetic_names = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .filter(|record| record.debut_set == CardSet::Token)
        .map(|record| record.name)
        .collect::<HashSet<_>>();

    assert!(synthetic_names.is_empty());
}

#[test]
fn built_in_catalog_indexes_definitions_and_printings_separately() {
    let catalog = crate::card::catalog().unwrap();
    let records = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    assert_eq!(
        catalog.definitions().len(),
        records.len(),
        "every registered record must become one catalog definition",
    );

    for record in records {
        let definition = catalog
            .get(record.id())
            .unwrap_or_else(|| panic!("{} is missing from the catalog", record.name));
        assert_eq!(definition.name, record.name);
        assert!(
            catalog
                .get_printing(CardPrintingId::new(record.id(), record.debut_set))
                .is_some(),
            "{} is missing its debut printing",
            record.name,
        );
    }

    for module in SET_MODULES {
        for record in module.additional_printings {
            assert!(
                catalog
                    .get_printing(record.printing(module.set).id)
                    .is_some(),
                "{} is missing an additional {:?} printing",
                record.card.name,
                module.set,
            );
        }
    }
    for variant in 0..3 {
        assert!(
            catalog
                .get_printing(CardPrintingId::with_variant(
                    cards::PLAINS,
                    CardSet::Beta,
                    variant,
                ))
                .is_some()
        );
    }
    assert_eq!(catalog.find_by_name("Plains"), Some(cards::PLAINS));
    assert_eq!(
        catalog.find_by_name("Delver of Secrets // Insectile Aberration"),
        Some(cards::DELVER_OF_SECRETS),
    );
    assert_eq!(
        catalog.find_by_name("Delver of Secrets"),
        Some(cards::DELVER_OF_SECRETS),
    );
    assert!(catalog.is_allowed_in(cards::WITCH_ENCHANTER, Format::VintageCube));
}

#[test]
fn tutors_and_fetch_lands_use_declarative_zone_searches() {
    let enlightened = y1996::mirage::ENLIGHTENED_TUTOR.rules.ability_clauses()[0];
    assert_eq!(
        enlightened.declarative_effect(),
        Some(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        })
    );

    let fetches: [(&CardRecord, &[BasicLandType]); 5] = [
        (
            &y2002::onslaught::BLOODSTAINED_MIRE,
            &[BasicLandType::Swamp, BasicLandType::Mountain],
        ),
        (
            &y2002::onslaught::FLOODED_STRAND,
            &[BasicLandType::Plains, BasicLandType::Island],
        ),
        (
            &y2002::onslaught::POLLUTED_DELTA,
            &[BasicLandType::Island, BasicLandType::Swamp],
        ),
        (
            &y2002::onslaught::WINDSWEPT_HEATH,
            &[BasicLandType::Forest, BasicLandType::Plains],
        ),
        (
            &y2002::onslaught::WOODED_FOOTHILLS,
            &[BasicLandType::Mountain, BasicLandType::Forest],
        ),
    ];
    for (fetch, basic_land_types) in fetches {
        let ability = fetch.rules.ability_clauses()[0];
        let DeclarativeAbilityDef::Activated(activated) = ability.definition else {
            panic!("{} should have an activated ability", fetch.name);
        };
        assert_eq!(
            activated.costs.as_slice(),
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::PayLife(1),
                AbilityCostDef::SacrificeSource,
            ],
            "{} has the wrong activation cost",
            fetch.name
        );
        assert_eq!(
            ability.declarative_effect(),
            Some(EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasAnyBasicLandType(basic_land_types),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            }),
            "{} has the wrong search parameters",
            fetch.name
        );
    }
}

#[test]
fn standard_search_cards_preserve_may_reveal_and_cardinality_semantics() {
    let liliana = y2012::magic_2013::LILIANAS_SHADE.rules.ability_clauses()[0];
    let Some(EffectDef::May { player, effect }) = liliana.declarative_effect() else {
        panic!("Liliana's Shade should make the entire search optional");
    };
    assert_eq!(player, EffectRecipientDef::Controller);
    assert_eq!(
        *effect,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        }
    );

    let seek = y2012::return_to_ravnica::SEEK_THE_HORIZON
        .rules
        .ability_clauses()[0];
    assert_eq!(
        seek.declarative_effect(),
        Some(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(3),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        })
    );

    let farseek = y2012::magic_2013::FARSEEK.rules.ability_clauses()[0];
    assert_eq!(
        farseek.declarative_effect(),
        Some(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Plains,
                BasicLandType::Island,
                BasicLandType::Swamp,
                BasicLandType::Mountain,
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        })
    );

    let rangers_path = y2012::magic_2013::RANGERS_PATH.rules.ability_clauses()[0];
    assert_eq!(
        rangers_path.declarative_effect(),
        Some(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            minimum: 0,
            maximum: ValueDef::Constant(2),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        })
    );
}

#[test]
fn ring_uses_declarative_format_and_draw_replacement_constructs() {
    let ability = y1993::arabian_nights::RING_OF_MARUF.rules.ability_clauses()[0];
    let DeclarativeAbilityDef::Activated(activated) = ability.definition else {
        panic!("Ring of Ma'rûf should have an activated ability");
    };
    assert_eq!(
        activated.costs.as_slice(),
        &[
            AbilityCostDef::Mana(crate::mana_cost!("{5}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::ExileSource,
        ]
    );
    let Some(EffectDef::ReplaceNextDrawThisTurn { player, effect }) = ability.declarative_effect()
    else {
        panic!("Ring should install a shared next-draw replacement");
    };
    assert_eq!(player, EffectRecipientDef::Controller);
    let EffectDef::IfFormat {
        format,
        then,
        otherwise,
    } = *effect
    else {
        panic!("Ring should branch through the shared format construct");
    };
    assert_eq!(format, Format::OldSchool9394);
    let EffectDef::ChooseCards {
        sources: old_school_sources,
        destination: old_school_destination,
        ..
    } = *then
    else {
        panic!("Old School Ring should choose an owned card");
    };
    assert_eq!(
        old_school_sources,
        &[
            CardChoiceSourceDef::Zone(ZoneKind::Exile),
            CardChoiceSourceDef::OutsideGame,
        ]
    );
    assert_eq!(old_school_destination, ZoneKind::Hand);
    let EffectDef::ChooseCards {
        sources: oracle_sources,
        destination: oracle_destination,
        ..
    } = *otherwise
    else {
        panic!("Oracle Ring should choose an owned card");
    };
    assert_eq!(oracle_sources, &[CardChoiceSourceDef::OutsideGame]);
    assert_eq!(oracle_destination, ZoneKind::Hand);
}

#[test]
fn unsupported_cards_expose_no_executable_clauses() {
    let records = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    for record in records {
        let definition = record.definition();
        if definition.implementation_status() == ImplementationStatus::Unsupported {
            assert!(
                definition
                    .play_options
                    .iter()
                    .all(|option| option.effect_status == CardEffectStatus::Unsupported),
                "{} is unsupported but exposes an implemented play option",
                record.name
            );
            for part in &definition.parts {
                assert!(
                    part.rules.implementation_status() == ImplementationStatus::Unsupported,
                    "{} mixes declarative and unsupported parts",
                    record.name
                );
                assert!(
                    part.rules.ability_clauses().is_empty(),
                    "{} is unsupported but exposes executable clauses",
                    record.name
                );
            }
        }
    }
}

#[test]
fn attached_control_changes_are_static_abilities() {
    fn changes_attached_control(effect: EffectDef) -> bool {
        matches!(
            effect,
            EffectDef::GainControl {
                object: EffectRecipientDef::AttachedPermanent,
                duration: ControlDurationDef::WhileSourceRemains {
                    while_tapped: false,
                },
                ..
            }
        ) || child_effects(effect)
            .into_iter()
            .any(changes_attached_control)
    }

    fn audit_ability(card: &str, ability: &AbilityDef) {
        if ability
            .declarative_effect()
            .is_some_and(changes_attached_control)
        {
            assert!(
                matches!(ability.definition, DeclarativeAbilityDef::Static(_)),
                "{card} models attached-permanent control as a nonstatic ability: {}",
                ability.text
            );
        }
        if let Some(modal) = ability.modal() {
            for mode in modal.modes {
                audit_ability(card, mode);
            }
        }
    }

    for record in SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
    {
        let definition = record.definition();
        for part in &definition.parts {
            for ability in part.rules.ability_clauses() {
                audit_ability(record.name, ability);
            }
        }
    }
}

#[test]
fn standard_records_are_unique_and_format_legal() {
    let records = standard_records();
    assert_eq!(
        records
            .iter()
            .map(|record| record.id())
            .collect::<HashSet<_>>()
            .len(),
        records.len(),
        "Standard records must have unique definitions",
    );

    let mut names = HashSet::new();
    for record in records {
        assert!(names.insert(record.name));
        assert!(!record.rules.has_supertype(CardSupertype::Basic));
        assert!(Format::IsdM14Standard.allows_set(record.debut_set));
    }

    assert!(!names.contains("Celestial Purge"));
    assert!(names.contains("Celestial Flare"));
    assert!(names.contains("Thespian's Stage"));
}
