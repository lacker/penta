use super::*;

#[test]
fn format_sets_and_card_records_have_catalog_modules() {
    let format_sets = Format::OldSchool9394
        .rules()
        .allowed_sets
        .iter()
        .chain(Format::IsdRtrStandard.rules().allowed_sets)
        .copied()
        .collect::<Vec<_>>();
    // Tokens are registered like a set so a client can resolve one by
    // definition, but they are deliberately in no format's card pool, so
    // they are not part of this correspondence.
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
    assert!(all_registered_sets.contains(&CardSet::Token));

    let registered_sets = all_registered_sets
        .iter()
        .copied()
        .filter(|set| *set != CardSet::Token)
        .collect::<Vec<_>>();
    for format in [Format::OldSchool9394, Format::IsdRtrStandard] {
        assert!(
            !format.rules().allowed_sets.contains(&CardSet::Token),
            "no format may allow the token set"
        );
    }

    assert!(
        format_sets.iter().all(|set| registered_sets.contains(set)),
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
    let records = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    let mut ids = records.iter().map(|record| record.id).collect::<Vec<_>>();
    ids.sort_unstable();
    let expected = (1..=records.len())
        .map(|raw| {
            CardDefinitionId(
                u16::try_from(raw).expect("the built-in catalog must fit its definition ID type"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        ids, expected,
        "definition IDs must remain dense until deterministic IDs replace them",
    );
    // Names identify the cards a decklist can name. Tokens are not among
    // them, and Magic prints several that share a name.
    let deck_legal = records
        .iter()
        .filter(|record| record.debut_set != CardSet::Token)
        .collect::<Vec<_>>();
    assert_eq!(
        deck_legal
            .iter()
            .map(|record| record.name)
            .collect::<HashSet<_>>()
            .len(),
        deck_legal.len()
    );
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
            .get(record.id)
            .unwrap_or_else(|| panic!("{} is missing from the catalog", record.name));
        assert_eq!(definition.name, record.name);
        assert!(
            catalog
                .get_printing(CardPrintingId::new(record.id, record.debut_set))
                .is_some(),
            "{} is missing its debut printing",
            record.name,
        );
        if record.debut_set == CardSet::Token {
            let printings = catalog.printings_for(record.id);
            assert_eq!(printings.len(), 1, "{} should be synthetic", record.name);
            assert_eq!(printings[0].id.set, CardSet::Token);
        }
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
}

#[test]
fn tutors_and_fetch_lands_use_declarative_zone_searches() {
    let enlightened = y1996::mirage::ENLIGHTENED_TUTOR.rules.ability_clauses()[0];
    assert_eq!(
        enlightened.effect.definition,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            minimum: 0,
            maximum: 1,
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
        }
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
            ability.effect.definition,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasAnyBasicLandType(basic_land_types),
                minimum: 0,
                maximum: 1,
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
            },
            "{} has the wrong search parameters",
            fetch.name
        );
    }
}

#[test]
fn standard_search_cards_preserve_may_reveal_and_cardinality_semantics() {
    let liliana = y2012::magic_2013::LILIANAS_SHADE.rules.ability_clauses()[0];
    let EffectDef::May { player, effect } = liliana.effect.definition else {
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
            maximum: 1,
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
        }
    );

    let seek = y2012::return_to_ravnica::SEEK_THE_HORIZON
        .rules
        .ability_clauses()[0];
    assert_eq!(
        seek.effect.definition,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: 3,
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
        }
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
    let EffectDef::ReplaceNextDrawThisTurn { player, effect } = ability.effect.definition else {
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
fn every_non_declarative_clause_explains_its_implementation() {
    let records = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    for record in records {
        let definition = record.definition();
        for part in &definition.parts {
            for ability in part.rules.ability_clauses() {
                if ability.effect.execution != EffectExecutionDef::Declarative
                    || ability.coverage.status != ImplementationStatus::Complete
                {
                    assert!(
                        ability
                            .coverage
                            .explanation
                            .is_some_and(|explanation| !explanation.trim().is_empty()),
                        "{} has a non-declarative clause without an explanation: {}",
                        record.name,
                        ability.text
                    );
                }
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
            .map(|record| record.id)
            .collect::<HashSet<_>>()
            .len(),
        records.len(),
        "Standard records must have unique definitions",
    );

    let mut names = HashSet::new();
    for record in records {
        assert!(names.insert(record.name));
        assert!(!record.rules.has_supertype(CardSupertype::Basic));
        assert!(Format::IsdRtrStandard.allows_set(record.debut_set));
        if let Some(behavior) = record.rules.special_behavior() {
            assert_eq!(behavior.rules(), &record.rules);
        }
    }

    assert!(!names.contains("Celestial Purge"));
    assert!(names.contains("Celestial Flare"));
    assert!(names.contains("Thespian's Stage"));
}
