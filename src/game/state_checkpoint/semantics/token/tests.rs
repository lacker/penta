use super::*;
use crate::card::{AbilityDef, CardDefinition, CardRules, CardSet};
use crate::game::state_checkpoint::model::AbilityLocator;
use crate::{CardCatalog, CardDefinitionId, CardPartId};

#[test]
fn token_owned_abilities_can_locate_tokens_they_create() {
    static CHILD_TOKEN: TokenCharacteristics =
        TokenCharacteristics::creature(&["Child"], &[], 1, 1).with_name("Child Token");
    static CREATE_CHILD: EffectDef = EffectDef::create_token(CHILD_TOKEN);
    static PARENT_TOKEN_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Create a Child token.",
        &[],
        CREATE_CHILD,
    )];
    static PARENT_TOKEN: TokenCharacteristics =
        TokenCharacteristics::creature(&["Parent"], &[], 2, 2)
            .with_name("Parent Token")
            .with_abilities(&PARENT_TOKEN_ABILITIES);
    static CREATE_PARENT: EffectDef = EffectDef::create_token(PARENT_TOKEN);
    static CREATOR_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Create a Parent token.",
        &[],
        CREATE_PARENT,
    )];

    let mut definition = CardDefinition::new(
        CardDefinitionId::new(1),
        "Nested Token Creator",
        CardSet::Alpha,
        crate::card::CardRules::unsupported(),
    );
    let rules = CardRules::unsupported().with_abilities(&CREATOR_ABILITIES);
    definition.rules = rules;
    definition
        .parts
        .iter_mut()
        .find(|part| part.id == CardPartId::PRIMARY)
        .expect("the synthetic definition has a primary part")
        .rules = rules;
    let catalog = CardCatalog::new([definition]).expect("the nested token catalog validates");

    let locator = token_characteristics_locator(&catalog, CHILD_TOKEN)
        .expect("the token-owned creator has a durable locator");
    assert!(matches!(locator.creator(), AbilityLocator::Token { .. }));
    assert_eq!(
        catalog_token_characteristics(&catalog, &locator),
        Some(CHILD_TOKEN),
    );
}
