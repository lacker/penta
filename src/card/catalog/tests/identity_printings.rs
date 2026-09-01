use super::*;

#[test]
fn definition_ids_are_nonzero_and_javascript_safe() {
    assert_eq!(CardDefinitionId::try_new(0), None);
    assert_eq!(CardDefinitionId::try_new(CardDefinitionId::MAX + 1), None);
    assert_eq!(
        CardDefinitionId::try_new(CardDefinitionId::MAX),
        Some(CardDefinitionId::new(CardDefinitionId::MAX)),
    );
}

#[test]
fn primary_and_additional_printings_are_indexed_by_canonical_definition() {
    let id = CardDefinitionId::new(1);
    let primary = CardPrintingId::new(id, CardSet::Alpha);
    let beta = CardPrinting::new(id, CardSet::Beta);
    let alternate_beta = CardPrinting::with_variant(id, CardSet::Beta, 1);
    let catalog = CardCatalog::with_additional_printings(
        [definition(1, "Test Card", CardSet::Alpha)],
        [beta, alternate_beta],
    )
    .unwrap();

    assert_eq!(catalog.find_by_name(" test card "), Some(id));
    assert_eq!(catalog.get_printing(primary).unwrap().id, primary);
    assert_eq!(catalog.get_printing(beta.id), Some(&beta));
    assert_eq!(
        catalog.get_printing(alternate_beta.id),
        Some(&alternate_beta)
    );
    assert_eq!(catalog.printings_for(id).len(), 3);
    assert!(catalog.has_printing_in(id, CardSet::Alpha));
    assert!(catalog.has_printing_in(id, CardSet::Beta));
    assert!(!catalog.has_printing_in(id, CardSet::Unlimited));
}

#[test]
fn sparse_javascript_safe_definition_ids_do_not_expand_the_dense_index() {
    let sparse = CardDefinitionId::new(1_u64 << 40);
    let dense = CardDefinitionId::new(2);
    let catalog = CardCatalog::new([
        definition(sparse.get(), "Sparse Card", CardSet::Alpha),
        definition(dense.get(), "Dense Card", CardSet::Alpha),
    ])
    .unwrap();

    assert_eq!(
        catalog.get(sparse).map(|card| card.name.as_str()),
        Some("Sparse Card")
    );
    assert_eq!(catalog.find_by_name("sparse card"), Some(sparse));
    assert_eq!(
        catalog.get(dense).map(|card| card.name.as_str()),
        Some("Dense Card")
    );
    assert_eq!(
        catalog
            .definitions()
            .into_iter()
            .map(|definition| definition.id)
            .collect::<Vec<_>>(),
        vec![dense, sparse],
    );
}

#[test]
fn duplicate_printing_ids_are_rejected() {
    let id = CardDefinitionId::new(1);
    let duplicate = CardPrinting::new(id, CardSet::Alpha);
    assert_eq!(
        CardCatalog::with_additional_printings(
            [definition(1, "Test Card", CardSet::Alpha)],
            [duplicate],
        )
        .unwrap_err(),
        CatalogError::DuplicatePrintingId(duplicate.id)
    );
}

#[test]
fn an_allowed_reprint_makes_the_canonical_identity_format_legal() {
    let id = CardDefinitionId::new(1);
    let catalog = CardCatalog::with_additional_printings(
        [definition(1, "Test Card", CardSet::Alpha)],
        [CardPrinting::new(id, CardSet::Magic2014)],
    )
    .unwrap();

    assert_eq!(catalog.get(id).unwrap().debut_set, CardSet::Alpha);
    assert!(catalog.is_allowed_in(id, Format::OldSchool9394));
    assert!(catalog.is_allowed_in(id, Format::IsdM14Standard));
}

#[test]
fn additional_printings_must_reference_a_cataloged_definition() {
    let orphan = CardPrinting::new(CardDefinitionId::new(2), CardSet::Beta);
    assert_eq!(
        CardCatalog::with_additional_printings(
            [definition(1, "Test Card", CardSet::Alpha)],
            [orphan],
        )
        .unwrap_err(),
        CatalogError::OrphanPrinting(orphan.id)
    );
}

#[test]
fn definition_supplied_printings_must_belong_to_that_definition() {
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let mismatched = CardPrinting::new(CardDefinitionId::new(2), CardSet::Beta);
    card.printings.push(mismatched);

    assert_eq!(
        CardCatalog::new([card]).unwrap_err(),
        CatalogError::MismatchedPrintingDefinition {
            definition: CardDefinitionId::new(1),
            printing: mismatched.id,
        }
    );
}

#[test]
fn unknown_definitions_have_no_printings() {
    let catalog = CardCatalog::new([definition(1, "Test Card", CardSet::Alpha)]).unwrap();
    assert!(catalog.printings_for(CardDefinitionId::new(2)).is_empty());
    assert!(
        catalog
            .get_printing(CardPrintingId::new(
                CardDefinitionId::new(2),
                CardSet::Alpha
            ))
            .is_none()
    );
}

#[test]
fn synthetic_definitions_do_not_bypass_global_name_uniqueness() {
    assert_eq!(
        CardCatalog::new([
            definition(1, "Shared name", CardSet::Token),
            definition(2, " shared NAME ", CardSet::Token),
        ])
        .unwrap_err(),
        CatalogError::DuplicateName(" shared NAME ".into()),
    );
}
