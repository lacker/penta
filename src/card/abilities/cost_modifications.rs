// Reusable constructors for filtered spell-cost static abilities.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's. These helpers only author the declarative clauses; the mana planner
// interprets every variant together while determining a spell's total cost.

/// A battlefield static that adds a mana cost to filtered spells.
#[must_use]
pub const fn spell_cost_increase(
    text: &'static str,
    spell: ObjectPredicateDef,
    caster: PlayerRelation,
    amount: ManaCost,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
            spell,
            caster,
            condition: SpellCostConditionDef::Always,
            adjustment: CostAdjustmentDef::Add(CostAmountDef::Mana(amount)),
        })),
    )
}

/// A battlefield static that removes generic mana from filtered spells.
#[must_use]
pub const fn spell_cost_reduction(
    text: &'static str,
    spell: ObjectPredicateDef,
    caster: PlayerRelation,
    amount: ValueDef,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
            spell,
            caster,
            condition: SpellCostConditionDef::Always,
            adjustment: CostAdjustmentDef::Subtract(CostAmountDef::Generic(amount)),
        })),
    )
}

/// A battlefield static that removes named colored symbols from filtered spells.
#[must_use]
pub const fn spell_colored_cost_reduction(
    text: &'static str,
    spell: ObjectPredicateDef,
    caster: PlayerRelation,
    amount: ManaCost,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
            spell,
            caster,
            condition: SpellCostConditionDef::Always,
            adjustment: CostAdjustmentDef::Subtract(CostAmountDef::Mana(amount)),
        })),
    )
}

/// A static that taxes filtered spells which target its own source.
#[must_use]
pub const fn targeting_source_spell_cost_increase(
    text: &'static str,
    spell: ObjectPredicateDef,
    caster: PlayerRelation,
    amount: ManaCost,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
            spell,
            caster,
            condition: SpellCostConditionDef::TargetsSource,
            adjustment: CostAdjustmentDef::Add(CostAmountDef::Mana(amount)),
        })),
    )
}

/// A filtered spell-cost adjustment whose direction and amount are explicit.
#[must_use]
pub const fn spell_cost_adjustment(
    text: &'static str,
    spell: ObjectPredicateDef,
    caster: PlayerRelation,
    condition: SpellCostConditionDef,
    adjustment: CostAdjustmentDef,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
            spell,
            caster,
            condition,
            adjustment,
        })),
    )
}

/// A static ability modifying this spell's own total cost on the stack
/// (CR 113.6d), independent of the zone it is cast from.
#[must_use]
pub const fn this_spell_cost_adjustment(
    text: &'static str,
    adjustment: CostAdjustmentDef,
) -> AbilityDef {
    spell_cost_adjustment(
        text,
        ObjectPredicateDef::Source,
        PlayerRelation::You,
        SpellCostConditionDef::Always,
        adjustment,
    )
    .with_source_zones(&[ZoneKind::Stack])
}

/// A live generic reduction of this spell's own total cost.
#[must_use]
pub const fn this_spell_cost_reduction(text: &'static str, amount: ValueDef) -> AbilityDef {
    this_spell_cost_adjustment(
        text,
        CostAdjustmentDef::Subtract(CostAmountDef::Generic(amount)),
    )
}
