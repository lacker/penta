use crate::card::sets;
use std::collections::HashSet;

use super::{Format, FormatCategory, FormatDefinition};
use crate::CardDefinitionId;
use crate::card::{CardDefinition, CardPrinting, CardRules, CardSet, CardSupertype};

#[test]
fn categories_partition_every_format_in_registry_order() {
    let categorized = FormatCategory::ALL
        .iter()
        .flat_map(|category| category.formats())
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(categorized, Format::ALL);
    for &format in Format::ALL {
        assert!(format.category().formats().contains(&format));
    }
}

#[test]
fn cedh_is_a_commander_format_with_deferred_legality_metadata() {
    assert_eq!(Format::Cedh.category(), FormatCategory::Commander);
    assert_eq!(Format::Cedh.slug(), "cedh");
    assert_eq!(Format::Cedh.rules().starting_life, 40);
    assert!(Format::Cedh.defers_deck_legality());
    assert!(Format::Cedh.allows_card(&CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
        "Any catalog identity",
        sets::alpha::SET,
        crate::card::CardRules::unsupported(),
    )));
    assert!(Format::Cedh.is_banned("Mana Crypt"));
    assert!(!Format::Cedh.is_banned("Lutri, the Spellchaser"));
    assert_eq!(
        Format::Cedh
            .commander_definition()
            .expect("Commander metadata")
            .companion_only_banned_cards,
        &["Lutri, the Spellchaser"]
    );
}

#[test]
fn cubes_are_singleton_pools_rather_than_set_windows() {
    for &format in FormatCategory::Cube.formats() {
        let definition = format.cube_definition().expect("a cube definition");
        assert_eq!(definition.rules.maximum_copies, 1);
        assert_eq!(definition.rules.minimum_main_deck_size, 40);
        assert!(format.set_definition().is_none());
        assert!(!definition.cards.is_empty());
        for pair in definition.cards.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} is sorted and duplicate-free: {} then {}",
                format,
                pair[0],
                pair[1]
            );
        }
    }
}

#[test]
fn pool_membership_decides_legality_regardless_of_printing() {
    let inside = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
        "Ancestral Recall",
        sets::alpha::SET,
        crate::card::CardRules::unsupported(),
    );
    let outside = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"),
        "Sorrow's Path",
        sets::alpha::SET,
        crate::card::CardRules::unsupported(),
    );
    assert!(Format::VintageCube.allows_card(&inside));
    assert!(!Format::VintageCube.allows_card(&outside));
}

#[test]
fn set_windows_are_nonempty_unique_and_exclude_tokens() {
    for &format in Format::ALL {
        let FormatDefinition::Sets(definition) = format.definition() else {
            continue;
        };
        assert!(
            !definition.allowed_sets.is_empty(),
            "{format} needs an allowed set window"
        );
        assert!(!definition.allowed_sets.contains(&CardSet::TOKEN));

        let unique = definition
            .allowed_sets
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        assert_eq!(unique.len(), definition.allowed_sets.len());
    }
}

#[test]
fn standards_have_the_expected_windows_and_categorical_labels() {
    assert_eq!(
        Format::SomM13Standard
            .set_definition()
            .expect("a set format")
            .allowed_sets,
        &[
            sets::scars_of_mirrodin::SET,
            sets::mirrodin_besieged::SET,
            sets::new_phyrexia::SET,
            sets::magic_2012::SET,
            sets::innistrad::SET,
            sets::dark_ascension::SET,
            sets::avacyn_restored::SET,
            sets::magic_2013::SET,
        ]
    );
    assert_eq!(
        Format::IsdM14Standard
            .set_definition()
            .expect("a set format")
            .allowed_sets,
        &[
            sets::innistrad::SET,
            sets::dark_ascension::SET,
            sets::avacyn_restored::SET,
            sets::magic_2013::SET,
            sets::return_to_ravnica::SET,
            sets::gatecrash::SET,
            sets::dragons_maze::SET,
            sets::magic_2014::SET,
        ]
    );
    for &format in FormatCategory::Standard.formats() {
        assert!(format.display_name().starts_with("Standard: "));
    }
    for &format in FormatCategory::Cube.formats() {
        assert!(format.display_name().starts_with("Cube: "));
    }
}

#[test]
fn formats_allow_only_their_sets_but_share_basic_lands() {
    let old_spell = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
        "Old spell",
        sets::alpha::SET,
        crate::card::CardRules::unsupported(),
    );
    let standard_spell = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"),
        "Standard spell",
        sets::innistrad::SET,
        crate::card::CardRules::unsupported(),
    );
    let basic = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000003"),
        "Plains",
        sets::alpha::SET,
        CardRules::new_land(&["Plains"]).with_supertype(CardSupertype::Basic),
    );

    assert!(Format::OldSchool9394.allows_card(&old_spell));
    assert!(!Format::OldSchool9394.allows_card(&standard_spell));
    assert!(Format::IsdM14Standard.allows_card(&standard_spell));
    assert!(!Format::IsdM14Standard.allows_card(&old_spell));
    assert!(Format::IsdM14Standard.allows_card(&basic));
    assert!(Format::OldSchool9394.allows_card(&basic));
}

#[test]
fn any_allowed_reprint_makes_the_canonical_card_identity_legal() {
    let id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001");
    let mut card = CardDefinition::new(
        id,
        "Reprinted spell",
        sets::alpha::SET,
        crate::card::CardRules::unsupported(),
    );
    card.printings
        .push(CardPrinting::new(id, sets::magic_2014::SET));

    assert!(Format::OldSchool9394.allows_card(&card));
    assert!(Format::IsdM14Standard.allows_card(&card));
}

#[test]
fn old_school_promo_legality_is_identity_specific() {
    let arena = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
        "Arena",
        sets::harper_prism_book_promos::SET,
        CardRules::unsupported(),
    );
    let mana_crypt = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"),
        "Mana Crypt",
        sets::harper_prism_book_promos::SET,
        CardRules::unsupported(),
    );
    let nalathni_dragon = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000003"),
        "Nalathni Dragon",
        sets::dragon_con::SET,
        CardRules::unsupported(),
    );

    assert!(Format::OldSchool9394.allows_card(&arena));
    assert!(!Format::OldSchool9394.allows_card(&mana_crypt));
    assert!(Format::OldSchool9394.allows_card(&nalathni_dragon));
}

#[test]
fn premodern_takes_the_window_from_fourth_edition_through_scourge() {
    let sets = Format::Premodern
        .set_definition()
        .expect("a set format")
        .allowed_sets;
    assert_eq!(sets.len(), 29);
    assert_eq!(sets.first(), Some(&sets::fourth_edition::SET));
    assert_eq!(sets.last(), Some(&sets::scourge::SET));
    assert!(!Format::Premodern.allows_set(sets::portal_second_age::SET));
}

#[test]
fn premodern_bans_its_own_list_and_restricts_nothing() {
    for banned in [
        "Brainstorm",
        "Force of Will",
        "Necropotence",
        "  mind twist  ",
    ] {
        assert!(Format::Premodern.is_banned(banned));
    }
    assert!(!Format::Premodern.is_banned("Swords to Plowshares"));
    assert!(
        Format::Premodern
            .set_definition()
            .expect("a set format")
            .restricted_cards
            .is_empty()
    );
}

#[test]
fn only_old_school_has_mana_burn_and_restrictions() {
    assert!(Format::OldSchool9394.rules().mana_burn);
    assert!(!Format::OldSchool9394.rules().mana_empties_at_end_of_step);
    assert!(Format::OldSchool9394.is_restricted(" black lotus "));
    assert!(Format::OldSchool9394.is_banned("CONTRACT FROM BELOW"));

    for &format in Format::ALL {
        if format != Format::OldSchool9394 {
            assert!(!format.rules().mana_burn);
            assert!(format.rules().mana_empties_at_end_of_step);
            assert!(!format.is_restricted("Black Lotus"));
        }
    }
}

#[test]
fn duel_commander_has_separate_gameplay_and_ban_policy() {
    let format = Format::DuelCommander;
    assert_eq!(format.category(), FormatCategory::Commander);
    assert_eq!(format.slug(), "duel-commander");
    assert!(format.defers_deck_legality());
    let rules = format.commander_definition().unwrap();
    assert_eq!(rules.rules.starting_life, 20);
    assert_eq!(rules.commander_damage_limit, None);
    assert!(rules.one_command_zone_commander && rules.commander_swapping);
    assert!(!rules.outside_game_effects);
    assert!(format.is_banned("Sol Ring"));
    assert!(!Format::Cedh.is_banned("Sol Ring"));
    assert!(
        rules
            .commander_only_banned_cards
            .contains(&"Derevi, Empyrial Tactician")
    );
    assert!(!format.is_banned("Derevi, Empyrial Tactician"));
    assert_eq!(
        rules.companion_only_banned_cards,
        &["Lutri, the Spellchaser"]
    );
}
