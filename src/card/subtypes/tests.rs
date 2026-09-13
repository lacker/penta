use super::*;

#[test]
fn literal_subtypes_compile_to_distinct_sets_and_preserve_printed_order() {
    const RULES: super::super::CardRules =
        super::super::CardRules::new_land(&["Forest", "Island", "Forest"]);
    const PREDICATE: super::super::SubtypeDef = super::super::SubtypeDef::literal("Dragon");
    assert_eq!(RULES.subtypes(), &["Forest", "Island", "Forest"]);
    assert_eq!(RULES.subtype_set().len(), 2);
    assert!(RULES.subtype_set().contains(Subtype::Island));
    assert!(!RULES.subtype_set().contains(Subtype::Mountain));
    assert_eq!(PREDICATE, super::super::SubtypeDef::Fixed(Subtype::Dragon));
}

#[test]
fn inline_subtypes_preserve_names_without_limiting_set_size() {
    use super::super::TokenCharacteristics;
    const SMALL: TokenCharacteristics =
        TokenCharacteristics::creature(&["Elf", "Druid", "Elf"], &[], 1, 1);
    const LONG: TokenCharacteristics = TokenCharacteristics::creature(CREATURE_TYPES, &[], 1, 1);
    assert_eq!(SMALL.rules().subtypes(), &["Elf", "Druid", "Elf"]);
    assert_eq!(
        SMALL.rules().subtype_set(),
        SubtypeSet::from_names(&["Druid", "Elf"])
    );
    assert_eq!(LONG.rules().subtypes(), CREATURE_TYPES);
    assert_eq!(
        LONG.rules().subtype_set(),
        SubtypeSet::family(SubtypeFamily::Creature)
    );
}

#[test]
fn subtype_names_round_trip_and_aliases_share_one_identity() {
    let mut names = std::collections::BTreeSet::new();
    for subtype in Subtype::ALL {
        assert!(names.insert(subtype.name()));
        assert_eq!(Subtype::from_name(subtype.name()), Some(*subtype));
    }
    assert_eq!(Subtype::from_name("Urza's"), Subtype::from_name("Urza’s"));
    assert_eq!(Subtype::from_name("C'tan"), Subtype::from_name("C’tan"));
    assert_eq!(Subtype::from_name("Dargon"), None);
    assert_eq!(SubtypeSet::from_names(&["Urza's", "Urza’s"]).len(), 1);
}

#[test]
fn subtype_families_share_only_their_rules_defined_card_types() {
    assert!(Subtype::Dragon.allowed_on(CardTypeSet::single(CardType::Kindred)));
    assert!(!Subtype::Dragon.allowed_on(CardTypeSet::single(CardType::Land)));
    assert!(Subtype::Arcane.allowed_on(CardTypeSet::single(CardType::Instant)));
    assert!(Subtype::Arcane.allowed_on(CardTypeSet::single(CardType::Sorcery)));
    assert!(Subtype::Ugin.in_family(SubtypeFamily::Planeswalker));
    assert!(!Subtype::Ugin.in_family(SubtypeFamily::Creature));
    assert!(Subtype::Spacecraft.in_family(SubtypeFamily::Artifact));
    assert!(Subtype::Spacecraft.in_family(SubtypeFamily::Planar));
}

#[test]
fn subtype_family_masks_support_add_remove_replace_and_intersection() {
    const CREATURES: SubtypeSet = SubtypeSet::family(SubtypeFamily::Creature);
    let mut set = SubtypeSet::from_names(&["Forest", "Food"]).union(CREATURES);
    assert_eq!(set.intersection(CREATURES).len(), CREATURE_TYPES.len());
    set.remove(Subtype::Dragon);
    assert!(!set.contains(Subtype::Dragon));
    assert!(set.contains(Subtype::Sliver));
    set = set
        .difference(CREATURES)
        .union(SubtypeSet::from_names(&["Frog"]));
    assert_eq!(set, SubtypeSet::from_names(&["Forest", "Food", "Frog"]));
    set.retain_for_card_types(CardTypeSet::single(CardType::Artifact));
    assert_eq!(set, SubtypeSet::from_names(&["Food"]));
}

#[test]
fn subtype_hash_dispatch_has_no_colliding_canonical_names() {
    let mut hashes = std::collections::BTreeSet::new();
    for subtype in Subtype::ALL {
        assert!(
            hashes.insert(super::name_hash(subtype.name())),
            "{}",
            subtype.name()
        );
    }
}

#[test]
fn subtype_families_match_catalog_type_lines() {
    let catalog = super::super::catalog().unwrap();
    let mut invalid = Vec::new();
    for definition in catalog.definitions() {
        for part in &definition.parts {
            for subtype in part.rules.subtype_set().iter() {
                if !subtype.allowed_on(part.rules.types()) {
                    invalid.push(format!("{}: {}", part.name, subtype.name()));
                }
            }
        }
    }
    assert!(invalid.is_empty(), "{}", invalid.join("\n"));
}
