// Shared Convoke and Buyback constructors.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

/// Convoke (CR 702.51): creatures become one-unit payment sources while the
/// total cost of this spell is being paid. The payment planner executes the
/// tap; this clause only marks which spells use that shared procedure.
#[must_use]
pub const fn convoke() -> AbilityDef {
    keyword(
        "Convoke (Your creatures can help cast this spell. Each creature you tap while casting \
         this spell pays for {1} or one mana of that creature's color.)",
        KeywordAbility::Convoke,
    )
}

/// Buyback with a mana surcharge. It is an optional additional cost, so it
/// composes with flashback and every other casting permission.
#[must_use]
pub const fn buyback(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Buyback.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Buyback,
            mana_cost: Some(mana_cost),
            additional_cost: None,
            resolution_destination: SpellResolutionDestinationDef::Hand,
        },
    )
}

/// Buyback paid with a selected nonmana object, such as sacrificing a land.
#[must_use]
pub const fn buyback_with_additional_cost(
    text: &'static str,
    cost: &'static SpellAdditionalCostDef,
) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        text,
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Buyback,
            mana_cost: None,
            additional_cost: Some(*cost),
            resolution_destination: SpellResolutionDestinationDef::Hand,
        },
    )
}
