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
        CardSet::ModernHorizons2,
    ] {
        assert!(registered_sets.contains(&testbed_set));
        assert!(!Format::OldSchool9394.allows_set(testbed_set));
        assert!(!Format::IsdRtrStandard.allows_set(testbed_set));
    }
    assert_eq!(registered_sets.len(), 40);
    assert_eq!(
        registered_sets
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .len(),
        40
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
    assert_eq!(records.len(), 605);

    let mut ids = records.iter().map(|record| record.id).collect::<Vec<_>>();
    ids.sort_unstable();
    assert_eq!(
        ids.iter().map(|id| id.0).collect::<Vec<_>>(),
        (1..=605).collect::<Vec<_>>()
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
    let printing_count = (1..=605)
        .filter(|id| {
            *id != cards::BEAST_TOKEN_3_3_GREEN.0
                && *id != cards::KNIGHT_TOKEN_2_2_WHITE.0
                && *id != cards::SOLDIER_TOKEN_1_1_RED_WHITE.0
                && *id != cards::DEMON_TOKEN_5_5_BLACK.0
                && *id != cards::ELEMENTAL_TOKEN_GREEN_WHITE.0
                && *id != cards::SPIRIT_TOKEN_1_1_WHITE.0
                && *id != cards::WOLF_TOKEN_2_2_GREEN.0
                && *id != cards::WOLF_TOKEN_1_1_BLACK.0
                && *id != cards::DOMRI_RADE_EMBLEM.0
                && *id != cards::TETRAVITE_TOKEN.0
                && *id != cards::ASSASSIN_TOKEN_1_1_BLACK.0
                && *id != cards::BIRD_TOKEN_4_4_RED.0
                && *id != cards::CITIZEN_TOKEN_1_1_WHITE.0
                && *id != cards::THRULL_TOKEN_0_1_BLACK.0
                && *id != cards::WASP_TOKEN_1_1_COLORLESS.0
                && *id != cards::MINOR_DEMON_TOKEN_1_1_BLACK_RED.0
        })
        .map(|id| catalog.printings_for(CardDefinitionId(id)).len())
        .sum::<usize>();

    assert_eq!(printing_count, 977);
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
fn every_non_declarative_clause_explains_its_implementation() {
    let records = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 605);

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
    assert_eq!(records.len(), 117);

    let expected_ids = (129..=244).chain([251]).collect::<Vec<_>>();
    assert_eq!(
        records.iter().map(|record| record.id.0).collect::<Vec<_>>(),
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
