use super::*;

#[test]
fn part_and_play_option_ids_are_unique_within_a_definition() {
    let mut duplicate_part = definition(1, "Test Card", CardSet::Alpha);
    duplicate_part.parts.push(duplicate_part.parts[0].clone());
    assert_eq!(
        error(duplicate_part),
        CatalogError::DuplicatePartId {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
        }
    );

    let mut duplicate_option = definition(1, "Test Card", CardSet::Alpha);
    duplicate_option
        .play_options
        .push(duplicate_option.play_options[0].clone());
    assert_eq!(
        error(duplicate_option),
        CatalogError::DuplicatePlayOptionId {
            definition: CardDefinitionId::new(1),
            option: PlayOptionId::DEFAULT,
        }
    );
}

#[test]
fn incoherent_rules_cannot_enter_the_catalog() {
    let invalid_rules = crate::CardRules::new_land(&[])
        .with_printed_mana_cost_for_test(PrintedManaCost::Cost(ManaCost::default()));

    let mut invalid_compatibility_view = definition(1, "Test Card", CardSet::Alpha);
    invalid_compatibility_view.rules = invalid_rules;
    assert_eq!(
        error(invalid_compatibility_view),
        CatalogError::IncoherentCardRules {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            explanation: "a land cannot have a printed mana cost",
        }
    );

    let mut invalid_part = definition(1, "Test Card", CardSet::Alpha);
    invalid_part.parts[0].rules = invalid_rules;
    assert_eq!(
        error(invalid_part),
        CatalogError::IncoherentCardRules {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            explanation: "a land cannot have a printed mana cost",
        }
    );
}

#[test]
fn creator_owned_token_rules_receive_catalog_composition_validation() {
    static INCOHERENT_TOKEN: TokenCharacteristics = TokenCharacteristics::new(
        crate::CardTypeSet::single(CardType::Land),
        &[],
        &[],
        Some(crate::CreatureStats {
            power: 1,
            toughness: 1,
        }),
    )
    .with_name("Broken Land");
    static CREATE_TOKEN: [AbilityDef; 1] = [AbilityDef::activated(
        "Create a token.",
        &[],
        EffectDef::CreateToken {
            token: INCOHERENT_TOKEN,
            copy: None,
            controller: None,
            count: ValueDef::Constant(1),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    )];

    let mut creator = definition(1, "Token Creator", CardSet::Alpha);
    let rules = creator.rules.with_abilities(&CREATE_TOKEN);
    set_primary_rules(&mut creator, &rules);
    assert_eq!(
        error(creator),
        CatalogError::IncoherentCardRules {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            explanation: "a noncreature cannot have creature power and toughness",
        }
    );
}

#[test]
fn creator_owned_token_abilities_receive_catalog_validation() {
    static INVALID_TOKEN_ABILITIES: [AbilityDef; 1] =
        [AbilityDef::activated("", &[], EffectDef::None)];
    static INVALID_TOKEN: TokenCharacteristics =
        TokenCharacteristics::creature(&["Germ"], &[], 0, 0)
            .with_name("Broken Germ")
            .with_abilities(&INVALID_TOKEN_ABILITIES);
    static CREATE_ATTACHED_TOKEN: [AbilityDef; 1] = [AbilityDef::activated(
        "Create and attach a token.",
        &[],
        EffectDef::CreateAttachedToken {
            token: INVALID_TOKEN,
            host: None,
        },
    )];

    let mut creator = definition(1, "Living Weapon", CardSet::Alpha);
    let rules = creator.rules.with_abilities(&CREATE_ATTACHED_TOKEN);
    set_primary_rules(&mut creator, &rules);
    assert_eq!(
        error(creator),
        CatalogError::EmptyAbilityText {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        }
    );
}

#[test]
fn compatibility_rules_must_match_the_primary_part() {
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    card.rules = crate::CardRules::new_artifact(ManaCost::default());

    assert_eq!(
        error(card),
        CatalogError::MismatchedPrimaryRules {
            definition: CardDefinitionId::new(1),
            part: CardPartId::PRIMARY,
        }
    );
}
