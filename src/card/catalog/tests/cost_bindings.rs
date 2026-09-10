use super::*;

const EVOKE: [AbilityDef; 2] =
    crate::card::abilities::evoke(&[CostDef::Mana(crate::mana_cost!("{1}"))]);
const EVOKE_COST: AbilityDef = EVOKE[0];

fn cost_binding_card(id: u64, name: &str, abilities: &'static [AbilityDef]) -> CardDefinition {
    CardDefinition::new(
        CardDefinitionId::new(id),
        name,
        CardSet::Alpha,
        crate::CardRules::new_creature(crate::mana_cost!("{3}"), &["Elemental"], 2, 2)
            .with_abilities(abilities),
    )
}

#[test]
fn alternative_cost_bindings_require_a_declaration_on_the_same_card_part() {
    static REFERENCES: [AbilityDef; 1] = [EVOKE[1]];
    let declares = cost_binding_card(1, "Declares Evoke", &[EVOKE_COST]);
    let references = cost_binding_card(2, "Missing Evoke", &REFERENCES);
    assert!(matches!(
        CardCatalog::new([declares, references]),
        Err(CatalogError::InvalidAlternativeCostBinding { definition, binding, .. })
            if definition == CardDefinitionId::new(2) && binding == crate::Binding!("evoke")
    ));
}

#[test]
fn alternative_cost_bindings_reject_duplicate_declarations() {
    let card = cost_binding_card(1, "Duplicate Evoke", &[EVOKE_COST, EVOKE_COST]);
    assert!(matches!(
        error(card),
        CatalogError::InvalidAlternativeCostBinding { ability, binding, .. }
            if ability == AbilityId(1) && binding == crate::Binding!("evoke")
    ));
}

#[test]
fn alternative_cost_bindings_require_a_durable_name() {
    static ABILITIES: [AbilityDef; 1] =
        [EVOKE_COST.with_alternative_cost_binding(crate::ParentBinding)];
    let card = cost_binding_card(1, "Unnamed Cost", &ABILITIES);
    assert!(matches!(
        error(card),
        CatalogError::InvalidAlternativeCostBinding { binding, .. }
            if binding == crate::ParentBinding
    ));
}

#[test]
fn alternative_cost_bindings_can_be_reused_by_different_cards() {
    static ABILITIES: [AbilityDef; 2] = [EVOKE[1], EVOKE_COST];
    CardCatalog::new([
        cost_binding_card(1, "First Evoke", &ABILITIES),
        cost_binding_card(2, "Second Evoke", &ABILITIES),
    ])
    .expect("each card declares its own evoke binding, even after the trigger");
}
