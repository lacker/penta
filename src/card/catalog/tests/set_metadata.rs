use std::collections::HashSet;

use super::*;

#[test]
fn set_identity_uses_code_for_equality_and_hashing() {
    let original = CardSet::new(&crate::card::CardSetMetadata {
        code: "TST",
        slug: "test-set",
    });
    let renamed = CardSet::new(&crate::card::CardSetMetadata {
        code: "TST",
        slug: "renamed-test-set",
    });
    let other = CardSet::new(&crate::card::CardSetMetadata {
        code: "OTH",
        slug: "test-set",
    });
    assert_eq!(original, renamed);
    assert_ne!(original, other);
    assert_eq!(HashSet::from([original, renamed, other]).len(), 2);
}

#[test]
fn catalog_rejects_conflicting_set_metadata_across_definitions_and_printings() {
    let original = CardSet::new(&crate::card::CardSetMetadata {
        code: "TST",
        slug: "test-set",
    });
    let conflicts = [
        (
            CardSet::new(&crate::card::CardSetMetadata {
                code: "TST",
                slug: "renamed-test-set",
            }),
            CatalogError::ConflictingSetSlug {
                code: "TST",
                first: "test-set",
                second: "renamed-test-set",
            },
        ),
        (
            CardSet::new(&crate::card::CardSetMetadata {
                code: "OTH",
                slug: "test-set",
            }),
            CatalogError::DuplicateSetSlug {
                slug: "test-set",
                first: "TST",
                second: "OTH",
            },
        ),
    ];
    for (conflict, expected) in conflicts {
        let error = CardCatalog::new([
            definition(1, "First", original),
            definition(2, "Second", conflict),
        ])
        .unwrap_err();
        assert_eq!(error, expected);

        let printing = CardPrinting::with_variant(CardDefinitionId::new(1), conflict, 1);
        let mut card = definition(1, "First", original);
        card.printings.push(printing);
        assert_eq!(CardCatalog::new([card]).unwrap_err(), expected);
        assert_eq!(
            CardCatalog::with_additional_printings([definition(1, "First", original)], [printing],)
                .unwrap_err(),
            expected,
        );
    }
}

#[test]
fn catalog_accepts_repeated_set_metadata_and_preserves_wire_slugs() {
    let set = CardSet::new(&crate::card::CardSetMetadata {
        code: "TST",
        slug: "test_set",
    });
    let printing = CardPrinting::with_variant(CardDefinitionId::new(1), set, 1);
    let catalog = CardCatalog::with_additional_printings(
        [definition(1, "First", set), definition(2, "Second", set)],
        [printing],
    )
    .unwrap();
    assert_eq!(catalog.get_printing(printing.id), Some(&printing));
    let json = crate::protocol::catalog_json(&catalog);
    assert_eq!(json["cards"][0]["debutSet"], "test_set");
    assert_eq!(json["cards"][0]["printings"][1]["set"], "test_set");
}
