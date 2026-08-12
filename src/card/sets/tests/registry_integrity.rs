use super::*;

#[test]
fn every_cataloged_set_has_one_matching_module() {
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
    let registered_sets = SET_MODULES
        .iter()
        .map(|module| module.set)
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
    for testbed_set in [
        CardSet::IceAge,
        CardSet::Mirage,
        CardSet::Visions,
        CardSet::Tempest,
        CardSet::Stronghold,
        CardSet::PortalSecondAge,
        CardSet::UrzasSaga,
        CardSet::MercadianMasques,
        CardSet::Nemesis,
        CardSet::Invasion,
        CardSet::Planeshift,
        CardSet::Apocalypse,
        CardSet::Odyssey,
        CardSet::Judgment,
        CardSet::Onslaught,
        CardSet::Darksteel,
        CardSet::PlanarChaos,
        CardSet::FutureSight,
        CardSet::Theros,
        CardSet::KhansOfTarkir,
        CardSet::ModernHorizons2,
    ] {
        assert!(registered_sets.contains(&testbed_set));
        assert!(!Format::OldSchool9394.allows_set(testbed_set));
        assert!(!Format::IsdRtrStandard.allows_set(testbed_set));
    }
    assert_eq!(registered_sets.len(), 41);
    assert_eq!(
        registered_sets
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .len(),
        41
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
fn built_in_records_keep_stable_dense_ids_and_unique_identity() {
    let records = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 1_368);

    let mut ids = records.iter().map(|record| record.id).collect::<Vec<_>>();
    ids.sort_unstable();
    assert_eq!(ids, (1..=1_368).map(CardDefinitionId).collect::<Vec<_>>());
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
    let (token_records, printed_records): (Vec<_>, Vec<_>) = records
        .into_iter()
        .partition(|record| record.debut_set == CardSet::Token);

    assert_eq!(token_records.len(), 35);
    assert_eq!(printed_records.len(), 1_333);
    assert_eq!(
        SET_MODULES
            .iter()
            .map(|module| module.additional_printings.len())
            .sum::<usize>(),
        558
    );

    for record in token_records {
        let printings = catalog.printings_for(record.id);
        assert_eq!(printings.len(), 1, "{} should be synthetic", record.name);
        assert_eq!(printings[0].id.set, CardSet::Token);
    }

    let printing_count = printed_records
        .iter()
        .map(|record| catalog.printings_for(record.id).len())
        .sum::<usize>();

    assert_eq!(printing_count, 1_891);
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
    assert_eq!(records.len(), 1_368);

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
fn standard_records_cover_the_top_eight_pool_with_stable_unique_ids() {
    let records = standard_records();
    assert_eq!(records.len(), 855);

    let token_ids = SET_MODULES
        .iter()
        .find(|module| module.set == CardSet::Token)
        .unwrap()
        .cards
        .iter()
        .map(|record| record.id)
        .collect::<HashSet<_>>();
    let expected_ids = (129..=244)
        .chain([251])
        .chain(607..=1_361)
        .chain(1_366..=1_367)
        .map(CardDefinitionId)
        .filter(|id| !token_ids.contains(id))
        .collect::<Vec<_>>();
    assert_eq!(
        records.iter().map(|record| record.id).collect::<Vec<_>>(),
        expected_ids,
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
