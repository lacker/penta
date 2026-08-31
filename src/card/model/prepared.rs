use super::{
    AbilityCoverageDef, AbilityDef, AbilityEffectDef, AbilityProgramDef, DeclarativeAbilityDef,
    EffectDef, EffectExecutionDef, SpellAbilityDef, TriggerEventDef, TriggeredAbilityDef, ValueDef,
};

/// A manually selected lowering that may replace an equivalent reference
/// effect when the prepared engine supports all of its inputs.
///
/// This is deliberately only a declaration-side hint. The executable program
/// remains private to the prepared engine, and the reference effect remains
/// authoritative for validation, traversal, checkpoints, and fallback.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum EffectExecutorDef {
    DrawCards(ValueDef),
}

/// A reference effect paired with an optional whole-effect lowering.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PreparedEffectDef {
    pub(crate) effect: EffectDef,
    pub(crate) executor: EffectExecutorDef,
}

impl PreparedEffectDef {
    #[must_use]
    pub(crate) const fn new(effect: EffectDef, executor: EffectExecutorDef) -> Self {
        Self { effect, executor }
    }

    #[must_use]
    pub const fn reference(self) -> EffectDef {
        self.effect
    }
}

impl AbilityEffectDef {
    #[must_use]
    pub(crate) const fn prepared(definition: PreparedEffectDef) -> Self {
        Self {
            definition: AbilityProgramDef::Effects(definition.effect),
            execution: EffectExecutionDef::Declarative,
            prepared: Some(definition.executor),
        }
    }
}

impl AbilityDef {
    /// A spell whose reference effect has a manually selected prepared
    /// lowering. Unsupported lowerings remain ordinary declarative spells.
    #[must_use]
    pub const fn prepared_spell(text: &'static str, effect: PreparedEffectDef) -> Self {
        Self::defined_prepared(
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::new()),
            effect,
        )
    }

    /// A triggered ability whose reference effect has a manually selected
    /// prepared lowering.
    #[must_use]
    pub const fn prepared_triggered(
        text: &'static str,
        event: TriggerEventDef,
        effect: PreparedEffectDef,
    ) -> Self {
        Self::defined_prepared(
            text,
            DeclarativeAbilityDef::Triggered(TriggeredAbilityDef::new(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn defined_prepared(
        text: &'static str,
        definition: DeclarativeAbilityDef,
        effect: PreparedEffectDef,
    ) -> Self {
        Self {
            text,
            definition,
            effect: AbilityEffectDef::prepared(effect),
            coverage: AbilityCoverageDef::complete(),
        }
    }
}
