// Activated clauses that apply a temporary effect to their own source.

/// An activated ability that applies an effect to its source until end of turn.
/// Use [`AppliedEffectDef::Composite`] for a clause that changes power/toughness
/// and grants abilities together. Costs and printed text remain card-local;
/// activation limits and timing restrictions can be chained onto the result.
#[must_use]
pub const fn apply_to_self_until_end_of_turn(
    text: &'static str,
    costs: &'static [CostDef],
    effect: AppliedEffectDef,
) -> AbilityDef {
    AbilityDef::activated(text, costs, self_effect_until_end_of_turn(effect))
}

/// The mana-only form of [`apply_to_self_until_end_of_turn`].
#[must_use]
pub const fn apply_to_self_until_end_of_turn_for_mana(
    text: &'static str,
    cost: ManaCost,
    effect: AppliedEffectDef,
) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(CostDef::Mana(cost)),
        &[],
        self_effect_until_end_of_turn(effect),
    )
}

const fn self_effect_until_end_of_turn(effect: AppliedEffectDef) -> EffectDef {
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect,
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    }
}

/// An activated ability that modifies its source's power and toughness until
/// end of turn. The values are deltas, not base characteristics, and may be
/// negative or computed at resolution (for example, [`ValueDef::SourcePower`]).
#[must_use]
pub const fn pump_until_end_of_turn(
    text: &'static str,
    costs: &'static [CostDef],
    power: ValueDef,
    toughness: ValueDef,
) -> AbilityDef {
    apply_to_self_until_end_of_turn(
        text,
        costs,
        AppliedEffectDef::modify_power_toughness(power, toughness),
    )
}

/// The mana-only form of [`pump_until_end_of_turn`].
#[must_use]
pub const fn pump_until_end_of_turn_for_mana(
    text: &'static str,
    cost: ManaCost,
    power: ValueDef,
    toughness: ValueDef,
) -> AbilityDef {
    apply_to_self_until_end_of_turn_for_mana(
        text,
        cost,
        AppliedEffectDef::modify_power_toughness(power, toughness),
    )
}

/// An activated ability that gives its source one ability until end of turn.
/// The caller supplies the complete cost list so nonmana payments retain the
/// same compact declaration shape.
#[must_use]
pub const fn gain_ability_until_end_of_turn(
    text: &'static str,
    costs: &'static [CostDef],
    ability: &'static AbilityDef,
) -> AbilityDef {
    apply_to_self_until_end_of_turn(text, costs, AppliedEffectDef::add_ability(ability))
}

/// The common mana-only form of [`gain_ability_until_end_of_turn`]. Owning
/// the one-element cost list lets card declarations pass a [`ManaCost`]
/// directly without naming the generic activation-cost representation.
#[must_use]
pub const fn gain_ability_until_end_of_turn_for_mana(
    text: &'static str,
    cost: ManaCost,
    ability: &'static AbilityDef,
) -> AbilityDef {
    apply_to_self_until_end_of_turn_for_mana(text, cost, AppliedEffectDef::add_ability(ability))
}
