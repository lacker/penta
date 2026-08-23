use std::collections::HashSet;

use super::{Format, FormatCategory, FormatDefinition};
use crate::CardDefinitionId;
use crate::card::{CardBehavior, CardDefinition, CardPrinting, CardSet};

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
        CardDefinitionId::new(1),
        "Ancestral Recall",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
    );
    let outside = CardDefinition::new(
        CardDefinitionId::new(2),
        "Sorrow's Path",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
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
        assert!(!definition.allowed_sets.contains(&CardSet::Token));

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
            CardSet::ScarsOfMirrodin,
            CardSet::MirrodinBesieged,
            CardSet::NewPhyrexia,
            CardSet::Magic2012,
            CardSet::Innistrad,
            CardSet::DarkAscension,
            CardSet::AvacynRestored,
            CardSet::Magic2013,
        ]
    );
    assert_eq!(
        Format::IsdM14Standard
            .set_definition()
            .expect("a set format")
            .allowed_sets,
        Format::IsdDgmStandard
            .set_definition()
            .expect("a set format")
            .allowed_sets
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
        CardDefinitionId::new(1),
        "Old spell",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
    );
    let standard_spell = CardDefinition::new(
        CardDefinitionId::new(2),
        "Standard spell",
        CardSet::Innistrad,
        false,
        CardBehavior::Unsupported,
    );
    let basic = CardDefinition::new(
        CardDefinitionId::new(3),
        "Plains",
        CardSet::Alpha,
        true,
        CardBehavior::Plains,
    );

    assert!(Format::OldSchool9394.allows_card(&old_spell));
    assert!(!Format::OldSchool9394.allows_card(&standard_spell));
    assert!(Format::IsdDgmStandard.allows_card(&standard_spell));
    assert!(!Format::IsdDgmStandard.allows_card(&old_spell));
    assert!(Format::IsdDgmStandard.allows_card(&basic));
    assert!(Format::OldSchool9394.allows_card(&basic));
}

#[test]
fn any_allowed_reprint_makes_the_canonical_card_identity_legal() {
    let id = CardDefinitionId::new(1);
    let mut card = CardDefinition::new(
        id,
        "Reprinted spell",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
    );
    card.printings
        .push(CardPrinting::new(id, CardSet::Magic2014));

    assert!(Format::OldSchool9394.allows_card(&card));
    assert!(Format::IsdDgmStandard.allows_card(&card));
}

#[test]
fn premodern_takes_the_window_from_fourth_edition_through_scourge() {
    let sets = Format::Premodern
        .set_definition()
        .expect("a set format")
        .allowed_sets;
    assert_eq!(sets.len(), 29);
    assert_eq!(sets.first(), Some(&CardSet::FourthEdition));
    assert_eq!(sets.last(), Some(&CardSet::Scourge));
    assert!(!Format::Premodern.allows_set(CardSet::PortalSecondAge));
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
