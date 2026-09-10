// Optional additional-cost constructors, including repeatable payments.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

/// Buyback with an additional cost. It is an optional additional cost, so it
/// composes with flashback and every other casting permission.
#[must_use]
pub const fn buyback(costs: &'static [CostDef]) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Buyback.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Buyback,
            label: OptionalAdditionalCostKindDef::Buyback.label(),
            resolution_destination: SpellResolutionDestinationDef::Hand,
            costs,
        },
    )
}

/// Kicker: an optional additional cost payable once. Unlike the legacy
/// alternative-cast helper, this composes with every legal way of casting the
/// spell and preserves the selected cost as part of the cast signature.
#[must_use]
pub const fn kicker(costs: &'static [CostDef]) -> AbilityDef {
    kicker_with_label(OptionalAdditionalCostKindDef::Kicker.label(), costs)
}

/// Kicker with a distinct action label, for cards that print two independent
/// kicker costs and need both choices to remain legible.
#[must_use]
pub const fn kicker_with_label(label: &'static str, costs: &'static [CostDef]) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Kicker.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Kicker,
            label,
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
            costs,
        },
    )
}

/// Replicate (CR 702.55): an optional additional cost payable any number of
/// times. The copies are not this ability's business -- the card prints a
/// cast trigger beside it that counts the payments -- so all this says is
/// what one payment costs and that it may be made again.
#[must_use]
pub const fn replicate(costs: &'static [CostDef]) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Replicate.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Replicate,
            label: OptionalAdditionalCostKindDef::Replicate.label(),
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
            costs,
        },
    )
}

/// Squad (CR 702.152): the same repeatable cost as replicate, bought by a
/// creature spell rather than by a spell that copies itself. What the
/// payments buy is printed beside it -- an enters trigger that makes that
/// many token copies -- so all this says is what one payment costs.
#[must_use]
pub const fn squad(costs: &'static [CostDef]) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Squad.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Squad,
            label: OptionalAdditionalCostKindDef::Squad.label(),
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
            costs,
        },
    )
}

/// Multikicker: an additional cost the caster may pay any number of times,
/// with nothing else attached. What it buys is printed separately, as a
/// clause that reads how many times it was paid.
#[must_use]
pub const fn multikicker(costs: &'static [CostDef]) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Multikicker.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Multikicker,
            label: OptionalAdditionalCostKindDef::Multikicker.label(),
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
            costs,
        },
    )
}

/// A named repeatable additional cost without a keyword wrapper. The caller's
/// full text remains the printed clause while `label` distinguishes several
/// costs offered by the same spell.
#[must_use]
pub const fn repeatable_additional_cost(
    text: &'static str,
    label: &'static str,
    costs: &'static [CostDef],
) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        text,
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Repeatable,
            label,
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
            costs,
        },
    )
}
