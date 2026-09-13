//! Catalog-owned resolving plans, including negative compilation results.
use super::{PreparedEffect, PreparedEngine};
use crate::{AbilityOrigin, EffectDef, EffectRecipientDef, TargetIndex};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PreparedDamageRecipient {
    Controller,
    Opponent,
    EachPlayer,
    EventPlayer,
    ControllerOfTriggeringObject,
    LegalTargets(TargetIndex),
}

impl PreparedDamageRecipient {
    pub(super) fn compile(recipient: EffectRecipientDef) -> Option<Self> {
        Some(match recipient {
            EffectRecipientDef::Controller => Self::Controller,
            EffectRecipientDef::Opponent => Self::Opponent,
            EffectRecipientDef::EachPlayer => Self::EachPlayer,
            EffectRecipientDef::EventPlayer => Self::EventPlayer,
            EffectRecipientDef::ControllerOfTriggeringObject => Self::ControllerOfTriggeringObject,
            other => Self::LegalTargets(other.legal_target()?),
        })
    }
}

impl PreparedEngine {
    pub(crate) fn resolving_effect(
        &self,
        origin: AbilityOrigin,
        effect: EffectDef,
    ) -> Option<PreparedEffect> {
        // Preparation is reusable even when execution is disabled. The stack
        // retains its reference effect and chooses an execution path later.
        let cached = if let AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } = origin
        {
            self.catalog
                .static_program(definition, part)
                .and_then(|program| {
                    let (_, printed) = program.base_abilities.get(usize::from(ability.0))?;
                    // Origins are provenance. Text changes and substituted keyword
                    // abilities may have the same origin but a different program.
                    (printed.declarative_effect().unwrap_or(EffectDef::None) == effect)
                        .then(|| program.base_resolvers[usize::from(ability.0)])
                })
        } else {
            None
        };
        #[cfg(feature = "engine-profiling")]
        crate::engine_profiling::record(
            "resolver_plan",
            crate::engine_profiling::effect_kind(effect),
            if cached.is_some() {
                "catalog"
            } else {
                "runtime"
            },
            match cached {
                Some(Some(_)) => "supported",
                Some(None) => "unsupported_shape",
                None => "uncached",
            },
        );
        cached.unwrap_or_else(|| super::compile_effect(effect))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{DamageDef, DamageFollowUpDef};
    use crate::{ObjectRefDef, ValueDef};

    #[test]
    fn prepared_damage_rejects_complete_unsupported_shapes() {
        for effect in [
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::ChosenX),
            EffectDef::damage(EffectRecipientDef::Source, ValueDef::Constant(2)),
            EffectDef::damage_from(
                ObjectRefDef::Source,
                EffectRecipientDef::Controller,
                ValueDef::Constant(2),
            ),
            EffectDef::damage_simultaneously(&[]),
            EffectDef::DealDamage(
                DamageDef::new(EffectRecipientDef::Controller, ValueDef::Constant(2))
                    .with_follow_up(DamageFollowUpDef::IfDealtToIntended(&EffectDef::None)),
            ),
        ] {
            assert_eq!(super::super::compile_effect(effect), None);
        }
    }
}
