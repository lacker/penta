use super::*;
use crate::card::{
    CardBehavior, CardDefinition, CardSet, EffectRecipientDef, ObjectPredicateDef, ObjectRefDef,
    SacrificedAmountDef, ValueDef,
};
use crate::{CardDefinitionId, CardPartId, ObjectBindingIndex, ObjectSetBindingIndex};

static NESTED_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Test"], &[], 1, 1).with_name("Nested Locator Test");
static CREATE_TOKEN: EffectDef = EffectDef::create_token(NESTED_TOKEN);
static MILL_THEN: EffectDef = EffectDef::Mill {
    player: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(1),
    binding: None,
    then: Some(&CREATE_TOKEN),
};
static EXILE_OTHERWISE: EffectDef = EffectDef::ExileTopAndMayCast {
    player: EffectRecipientDef::Controller,
    otherwise: Some(&CREATE_TOKEN),
};
static SACRIFICE_OTHERWISE: EffectDef = EffectDef::SacrificeOfChoice {
    count: ValueDef::Constant(1),
    player: EffectRecipientDef::Controller,
    object: ObjectPredicateDef::Any,
    then: Some(&EffectDef::None),
    amount: SacrificedAmountDef::Power,
    otherwise: Some(&CREATE_TOKEN),
    optional: true,
};
static RETURN_THEN: EffectDef = EffectDef::PutOntoBattlefieldThen {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    binding: ObjectSetBindingIndex::PRIMARY,
    counters: None,
    then: &CREATE_TOKEN,
};

#[test]
fn virtual_object_effect_paths_cover_every_continuation_branch() {
    let cases = [
        (MILL_THEN, vec![0]),
        (EXILE_OTHERWISE, vec![0]),
        (SACRIFICE_OTHERWISE, vec![1]),
        (RETURN_THEN, vec![0]),
    ];
    let creator = AbilityLocator::Card {
        definition: CardDefinitionId::new(1),
        part_id: 0,
        ability_id: 0,
        nested: Vec::new(),
    };

    for (root, expected_path) in cases {
        let mut found = AuthoredVirtualObjects {
            tokens: Vec::new(),
            emblems: Vec::new(),
        };
        collect_effects(root, &mut Vec::new(), &creator, &mut found);
        let [(token, TokenCharacteristicsLocator::EffectPath { effect_path, .. })] =
            found.tokens.as_slice()
        else {
            panic!("the nested token has exactly one effect-path locator");
        };
        assert_eq!(*token, NESTED_TOKEN);
        assert_eq!(effect_path, &expected_path);

        let ability = AbilityDef::activated("Create a deeply nested token.", &[], root);
        assert_eq!(effect_at_path(&ability, effect_path), Some(CREATE_TOKEN));
    }
}

#[test]
fn token_and_emblem_owned_creators_form_one_semantic_chain() {
    static CHILD_TOKEN: TokenCharacteristics =
        TokenCharacteristics::creature(&["Child"], &[], 1, 1).with_name("Child Token");
    static EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Create a Child token.",
        &[],
        EffectDef::create_token(CHILD_TOKEN),
    )];
    static EMBLEM: EmblemCharacteristics =
        EmblemCharacteristics::new("Nested Creator emblem", &EMBLEM_ABILITIES);
    static PARENT_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Create an emblem.",
        &[],
        EffectDef::CreateEmblem { emblem: EMBLEM },
    )];
    static PARENT_TOKEN: TokenCharacteristics =
        TokenCharacteristics::creature(&["Parent"], &[], 2, 2)
            .with_name("Parent Token")
            .with_abilities(&PARENT_ABILITIES);
    static PRINTED_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Create a Parent token.",
        &[],
        EffectDef::create_token(PARENT_TOKEN),
    )];

    let mut definition = CardDefinition::new(
        CardDefinitionId::new(1),
        "Virtual Creator",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
    );
    let rules = (*CardBehavior::Unsupported.rules()).with_abilities(&PRINTED_ABILITIES);
    definition.rules = rules;
    definition
        .parts
        .iter_mut()
        .find(|part| part.id == CardPartId::PRIMARY)
        .expect("the synthetic definition has a primary part")
        .rules = rules;
    let catalog = CardCatalog::new([definition]).expect("the virtual creator catalog validates");

    let emblem_locator = super::super::emblem::emblem_characteristics_locator(&catalog, EMBLEM)
        .expect("the token-owned emblem is discoverable");
    assert!(matches!(
        emblem_locator.creator(),
        AbilityLocator::Token { .. }
    ));
    assert_eq!(
        super::super::emblem::catalog_emblem_characteristics(&catalog, &emblem_locator)
            .expect("the emblem locator reconstructs"),
        EMBLEM,
    );

    let child_locator = super::super::token::token_characteristics_locator(&catalog, CHILD_TOKEN)
        .expect("the emblem-owned token is discoverable");
    assert!(matches!(
        child_locator.creator(),
        AbilityLocator::Emblem { .. }
    ));
    assert_eq!(
        super::super::token::catalog_token_characteristics(&catalog, &child_locator),
        Some(CHILD_TOKEN),
    );
}
