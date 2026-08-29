use super::*;
use crate::card::AppliedRuleDef;
use crate::mana_cost;

#[test]
fn standard_records_have_complete_unique_scryfall_metadata() {
    let records = standard_records();
    let mut scryfall_ids = HashSet::new();

    for record in records {
        let scryfall_id = record.art.scryfall_id;
        assert!(
            is_uuid(scryfall_id),
            "{} has an invalid Scryfall ID: {scryfall_id}",
            record.name
        );
        assert!(
            scryfall_ids.insert(scryfall_id),
            "{} repeats Scryfall ID {scryfall_id}",
            record.name
        );
        assert!(
            !record.art.artist.trim().is_empty(),
            "{} is missing its artist",
            record.name
        );
    }
}

#[test]
fn structured_records_expose_parts_and_play_options_without_losing_primary_rules() {
    let garruk = y2011::innistrad::GARRUK_RELENTLESS.definition();
    assert_eq!(garruk.name, "Garruk Relentless // Garruk, the Veil-Cursed");
    assert_eq!(garruk.rules, garruk.primary_part().unwrap().rules);
    assert_eq!(garruk.parts.len(), 2);
    assert_eq!(garruk.parts[1].name, "Garruk, the Veil-Cursed");
    assert_eq!(garruk.parts[1].rules.mana_cost(), None);
    assert_eq!(
        garruk.parts[1].rules.colors(),
        [false, false, true, false, true]
    );
    assert!(matches!(
        garruk.structure,
        CardStructure::DoubleFaced {
            front: CardPartId(0),
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        }
    ));

    let huntmaster = y2012::dark_ascension::HUNTMASTER_OF_THE_FELLS.definition();
    assert_eq!(huntmaster.rules, huntmaster.primary_part().unwrap().rules);
    assert_eq!(huntmaster.parts.len(), 2);
    assert_eq!(huntmaster.parts[1].name, "Ravager of the Fells");
    assert_eq!(huntmaster.parts[1].rules.mana_cost(), None);
    assert_eq!(huntmaster.parts[1].rules.creature_stats().unwrap().power, 4);
    assert!(
        huntmaster.parts[1]
            .rules
            .has_executable_keyword(KeywordAbility::Trample)
    );

    let witch = y2024::modern_horizons_3::WITCH_ENCHANTER.definition();
    assert_eq!(witch.name, "Witch Enchanter // Witch-Blessed Meadow");
    assert!(matches!(
        witch.structure,
        CardStructure::DoubleFaced {
            kind: DoubleFacedKind::Modal,
            ..
        }
    ));
    assert_eq!(witch.play_options.len(), 2);
    assert_eq!(witch.play_options[0].action, PlayActionKind::CastSpell);
    assert_eq!(witch.play_options[1].action, PlayActionKind::PlayLand);

    let turn_burn = y2013::dragons_maze::TURN_BURN.definition();
    assert_eq!(turn_burn.name, "Turn // Burn");
    assert_eq!(turn_burn.rules, turn_burn.parts[0].rules);
    assert_eq!(turn_burn.parts.len(), 2);
    assert_eq!(turn_burn.parts[0].name, "Turn");
    assert_eq!(turn_burn.parts[1].name, "Burn");
    assert_eq!(
        turn_burn.parts[1].rules.colors(),
        [false, false, false, true, false]
    );
    assert!(matches!(
        turn_burn.structure,
        CardStructure::Split {
            ref parts,
            fused: Some(PlayOptionId(2)),
        } if parts == &[CardPartId(0), CardPartId(1)]
    ));
    assert_eq!(turn_burn.play_options.len(), 3);
    assert!(matches!(
        turn_burn.play_options[2].form,
        SpellForm::Combined(ref parts) if parts == &[CardPartId(0), CardPartId(1)]
    ));
    assert_eq!(
        turn_burn.play_options[2].restriction,
        PlayRestriction::FromHandOnly
    );
    assert_eq!(turn_burn.play_options[0].targets[0].id, TargetSlotId(0));
    assert_eq!(turn_burn.play_options[1].targets[0].id, TargetSlotId(0));
    assert_eq!(turn_burn.play_options[2].targets.len(), 2);
    assert_eq!(turn_burn.play_options[2].targets[0].id, TargetSlotId(0));
    assert_eq!(turn_burn.play_options[2].targets[1].id, TargetSlotId(1));

    let charm = y2012::return_to_ravnica::IZZET_CHARM.definition();
    assert_eq!(charm.parts.len(), 1);
    assert_eq!(charm.play_options.len(), 1);
    let modes = charm.play_options[0].modes.as_ref().unwrap();
    assert_eq!(
        (modes.minimum, modes.maximum, modes.may_repeat),
        (1, 1, false)
    );
    assert_eq!(modes.modes.len(), 3);
    assert_eq!(modes.modes[0].id, ModeId(0));
    assert_eq!(
        modes.modes[0].targets[0].predicate,
        TargetPredicate::NoncreatureSpell
    );
    assert_eq!(
        modes.modes[1].targets[0].predicate,
        TargetPredicate::CreaturePermanent
    );
    assert!(modes.modes[2].targets.is_empty());
    assert_eq!(charm.play_options[0].action, PlayActionKind::CastSpell);
}

#[test]
fn ordinary_records_synthesize_one_primary_part_and_play_option() {
    let bolt = y1993::alpha::LIGHTNING_BOLT.definition();
    assert_eq!(bolt.parts.len(), 1);
    assert_eq!(bolt.primary_part_id(), CardPartId::PRIMARY);
    assert_eq!(bolt.primary_part().unwrap().rules, bolt.rules);
    assert!(matches!(
        bolt.structure,
        CardStructure::Single {
            main: CardPartId::PRIMARY,
        }
    ));
    assert_eq!(bolt.play_options.len(), 1);
    assert_eq!(bolt.play_options[0].id, PlayOptionId::DEFAULT);
    assert_eq!(
        bolt.play_options[0].form,
        SpellForm::Part(CardPartId::PRIMARY)
    );

    let mountain = y1993::alpha::MOUNTAIN.definition();
    assert_eq!(mountain.parts[0].rules.mana_cost(), None);
    assert_eq!(mountain.play_options[0].action, PlayActionKind::PlayLand);
    assert_eq!(mountain.play_options[0].mana_cost, None);
}

#[test]
fn buyback_records_project_only_optional_additional_costs() {
    let projected_buyback = |record: &CardRecord| {
        let definition = record.definition();
        assert_eq!(
            definition.play_options.len(),
            1,
            "{} has one ordinary spell form",
            record.name,
        );
        let option = &definition.play_options[0];
        assert!(
            option.alternative_costs.is_empty(),
            "{} must not project Buyback as an alternative cost",
            record.name,
        );
        assert_eq!(
            option.additional_costs.len(),
            1,
            "{} projects exactly one Buyback choice",
            record.name,
        );
        option.additional_costs[0].clone()
    };

    let sprout_swarm = projected_buyback(&y2007::future_sight::SPROUT_SWARM);
    assert_eq!(sprout_swarm.label, "Buyback");
    assert_eq!(sprout_swarm.mana_cost, Some(mana_cost!("{3}")));

    let corpse_dance = projected_buyback(&y1997::tempest::CORPSE_DANCE);
    assert_eq!(corpse_dance.label, "Buyback");
    assert_eq!(corpse_dance.mana_cost, Some(mana_cost!("{2}")));

    let constant_mists = projected_buyback(&y1998::stronghold::CONSTANT_MISTS);
    assert_eq!(constant_mists.label, "Buyback");
    assert_eq!(constant_mists.mana_cost, None);
}

#[test]
fn cavern_records_both_mana_abilities_and_the_colored_mana_riders() {
    let abilities = y2012::avacyn_restored::CAVERN_OF_SOULS
        .rules
        .ability_clauses();
    assert_eq!(abilities.len(), 3);
    assert!(matches!(
        abilities[1].definition,
        DeclarativeAbilityDef::ActivatedMana(_)
    ));
    assert!(matches!(
        abilities[1].declarative_effect(),
        Some(EffectDef::AddMana(mana))
            if mana.mana == ManaSelectionDef::One(ManaColor::Colorless)
                && mana.amount == 1
                && mana.restrictions.is_empty()
                && mana.spend_effects.is_empty()
    ));
    assert!(matches!(
        abilities[2].definition,
        DeclarativeAbilityDef::ActivatedMana(_)
    ));
    assert!(matches!(
        abilities[2].declarative_effect(),
        Some(EffectDef::AddMana(mana))
            if mana.mana == ManaSelectionDef::Choice(&ManaColor::COLORS)
                && mana.amount == 1
                && mana.restrictions
                    == [ManaRestrictionDef::CastCreatureSpellOfChosenType]
                && mana.spend_effects
                    == [ManaSpendEffectDef::ApplyToPaidSpell(
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
                    )]
    ));
}

#[test]
fn every_builtin_land_without_mana_is_named_explicitly() {
    let lands = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .filter(|record| record.rules.has_type(crate::card::CardType::Land))
        .collect::<Vec<_>>();
    let lands_without_mana = lands
        .iter()
        .filter(|record| {
            let has_intrinsic_source = BasicLandType::ALL
                .into_iter()
                .any(|land_type| record.rules.has_subtype(land_type.subtype()));
            let has_printed_source = record.rules.ability_clauses().iter().any(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
            });
            let has_static_land_type_source =
                record.rules.ability_clauses().iter().any(|ability| {
                    ability.is_executable()
                        && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                        && matches!(
                            ability.declarative_effect(),
                            Some(EffectDef::StaticApply {
                                effect: AppliedEffectDef::Characteristic(
                                    CharacteristicOperationDef::BasicLandTypes(
                                        SetOperationDef::Add(types)
                                    )
                                ),
                                ..
                            }) if !types.is_empty()
                        )
                });
            // "This land is the chosen type" is a mana source too: which
            // one is not knowable here, but there is always exactly one.
            let has_chosen_land_type_source =
                record.rules.ability_clauses().iter().any(|ability| {
                    ability.is_executable()
                        && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                        && matches!(
                            ability.declarative_effect(),
                            Some(EffectDef::StaticApply {
                                effect: AppliedEffectDef::Characteristic(
                                    CharacteristicOperationDef::ChosenBasicLandType
                                ),
                                ..
                            })
                        )
                });
            !has_intrinsic_source
                && !has_printed_source
                && !has_static_land_type_source
                && !has_chosen_land_type_source
        })
        .map(|record| record.name)
        .collect::<Vec<_>>();
    assert_eq!(
        lands_without_mana,
        [
            "Bazaar of Baghdad",
            // Sacrifices creatures for life and taps for nothing.
            "Diamond Valley",
            "Island of Wak-Wak",
            "Oasis",
            // The five Legends band lands do nothing but hand out "bands with
            // other legendary creatures", one color each.
            "Adventurers' Guildhouse",
            "Cathedral of Serra",
            "Mountain Stronghold",
            "Seafarer's Quay",
            "The Tabernacle at Pendrell Vale",
            "Unholy Citadel",
            "Maze of Ith",
            "Safe Haven",
            // Spends its tap making two creatures fight rather than making
            // mana; its opponent chooses one of those targets.
            "Arena",
            // Spends its own tap fetching a basic rather than making mana.
            "Thawing Glaciers",
            "Bloodstained Mire",
            "Flooded Strand",
            "Polluted Delta",
            "Windswept Heath",
            "Wooded Foothills",
            // The enemy-coloured fetchland cycle, printed six years after
            // the allied one above.
            "Arid Mesa",
            "Marsh Flats",
            "Misty Rainforest",
            "Scalding Tarn",
            "Verdant Catacombs",
            "Evolving Wilds",
            // The same clause as the two cycles above, over every basic at
            // once rather than a pair of land types.
            "Prismatic Vista",
            // The same fetch, with the tapped land untapping again once you
            // are no longer the one behind on lands.
            "Fabled Passage",
            // It prints no mana ability at all: chapter I hands it one, and
            // by chapter III it has sacrificed itself again.
            "Urza's Saga",
            // Its opening-hand action is executable, but the printed mana
            // ability remains partial until a mana result can branch on the
            // luck counter it places.
            "Gemstone Caverns",
            // Counts ten counters down and trades itself for a 20/20; the
            // mana it costs to do that goes in rather than coming out.
            "Dark Depths",
        ]
    );
}

#[test]
fn basic_land_subtypes_do_not_repeat_intrinsic_mana_as_printed_clauses() {
    let lands = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .filter(|record| record.rules.has_type(crate::card::CardType::Land))
        .filter(|record| {
            BasicLandType::ALL
                .into_iter()
                .any(|land_type| record.rules.has_subtype(land_type.subtype()))
        })
        .collect::<Vec<_>>();
    for land in lands {
        assert_eq!(
            land.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be complete once basic-land mana is derived intrinsically",
            land.name,
        );
        assert!(
            !land.rules.ability_clauses().iter().any(|ability| matches!(
                ability.definition,
                DeclarativeAbilityDef::ActivatedMana(_)
            )),
            "{} should rely on its basic land subtypes for mana",
            land.name,
        );
    }
}

#[test]
fn every_nonland_mana_permanent_has_an_activated_mana_clause() {
    let records = [
        &y1993::alpha::BLACK_LOTUS,
        &y1993::alpha::MOX_EMERALD,
        &y1993::alpha::MOX_JET,
        &y1993::alpha::MOX_PEARL,
        &y1993::alpha::MOX_RUBY,
        &y1993::alpha::MOX_SAPPHIRE,
        &y1993::alpha::SOL_RING,
        &y1993::alpha::MANA_VAULT,
        &y1993::alpha::BASALT_MONOLITH,
        &y1993::alpha::BIRDS_OF_PARADISE,
        &y1993::alpha::LLANOWAR_ELVES,
        &y1994::legends::PRINCESS_LUCREZIA,
        &y1994::legends::RIVEN_TURNBULL,
        &y1994::legends::SUNASTIAN_FALCONER,
        &y1994::the_dark::SISTERS_OF_THE_FLAME,
        &y1994::fallen_empires::BASAL_THRULL,
        &y1994::the_dark::FELLWAR_STONE,
        &y2004::darksteel::DARKSTEEL_INGOT,
        &y2011::innistrad::AVACYNS_PILGRIM,
        &y2013::magic_2014::ELVISH_MYSTIC,
    ];
    for record in records {
        assert!(
            record.rules.ability_clauses().iter().any(|ability| {
                matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
            }),
            "{} is missing its activated mana clause",
            record.name
        );
    }
}

#[test]
fn migrated_activated_cards_preserve_their_derived_implementation_status() {
    let complete = [
        &y1993::alpha::CHAOS_ORB,
        &y1993::alpha::GLASSES_OF_URZA,
        &y1993::alpha::ICY_MANIPULATOR,
        &y1993::alpha::STONE_GIANT,
        &y1994::antiquities::MISHRA_S_FACTORY,
        &y1994::antiquities::ORCISH_MECHANICS,
        &y1994::antiquities::STRIP_MINE,
        &y1994::antiquities::TRISKELION,
        &y1994::fallen_empires::ICATIAN_JAVELINEERS,
        &y1994::legends::PENDELHAVEN,
        &y1994::legends::RELIC_BARRIER,
        &y1994::the_dark::MAZE_OF_ITH,
    ];

    for record in complete {
        assert_eq!(
            record.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should remain completely implemented",
            record.name
        );
    }
}

#[test]
fn early_core_sets_reuse_definitions_without_duplicating_identity() {
    let all_definition_ids = SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().map(|record| record.id()))
        .collect::<HashSet<_>>();
    let basics = [
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
        cards::MOUNTAIN,
        cards::FOREST,
    ];

    let early_sets = [
        (CardSet::Alpha, 2_u16),
        (CardSet::Beta, 3_u16),
        (CardSet::Unlimited, 3_u16),
        (CardSet::CollectorsEdition, 3_u16),
        (CardSet::InternationalCollectorsEdition, 3_u16),
    ];

    let mut printing_ids = HashSet::new();
    for (set, expected_basic_variants) in early_sets {
        let printings = printings_for_set(set);

        for printing in &printings {
            assert!(all_definition_ids.contains(&printing.id.definition));
            assert_eq!(printing.id.set, set);
            assert!(printing_ids.insert(printing.id));
        }
        for basic in basics {
            let variants = printings
                .iter()
                .filter(|printing| printing.id.definition == basic)
                .map(|printing| printing.id.variant)
                .collect::<HashSet<_>>();
            assert_eq!(variants.len(), usize::from(expected_basic_variants));
            assert_eq!(variants, (0..expected_basic_variants).collect());
        }
    }

    assert_eq!(y1993::beta::VOLCANIC_ISLAND.id(), cards::VOLCANIC_ISLAND);
    assert_eq!(y1993::beta::VOLCANIC_ISLAND.debut_set, CardSet::Beta);
}
