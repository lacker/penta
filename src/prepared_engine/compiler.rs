use super::{PreparedCatalog, PreparedEffect, PreparedStaticProgram};
use crate::card::EffectExecutorDef;
use crate::{
    AbilityDef, AppliedEffectDef, CardCatalog, CharacteristicOperationDef, DeclarativeAbilityDef,
    EffectDef, ValueDef,
};

pub(crate) fn compile_catalog(catalog: &CardCatalog) -> PreparedCatalog {
    let mut prepared = PreparedCatalog::default();
    for definition in catalog.definitions() {
        for part in &definition.parts {
            prepared.insert_static_program(
                definition.id,
                part.id,
                compile_static_program(part.rules.ability_clauses()),
            );
        }
    }
    prepared
}

pub(crate) fn compile_effect(executor: EffectExecutorDef) -> Option<PreparedEffect> {
    match executor {
        EffectExecutorDef::DrawCards(ValueDef::Constant(count)) => u16::try_from(count)
            .ok()
            .map(|count| PreparedEffect::DrawCards { count }),
        EffectExecutorDef::DrawCards(_) => None,
    }
}

fn compile_static_program(abilities: &[AbilityDef]) -> PreparedStaticProgram {
    PreparedStaticProgram {
        supplies_land_type_effect: abilities.iter().copied().any(|ability| {
            ability.is_executable()
                && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                && ability
                    .declarative_effect()
                    .is_some_and(effect_contains_land_type_operation)
        }),
    }
}

fn effect_contains_land_type_operation(effect: EffectDef) -> bool {
    match effect {
        EffectDef::Sequence(effects) => effects
            .iter()
            .copied()
            .any(effect_contains_land_type_operation),
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            effect_contains_land_type_operation(*conditional.then)
                || conditional
                    .otherwise
                    .is_some_and(|otherwise| effect_contains_land_type_operation(*otherwise))
        }
        EffectDef::ConditionalStatic(conditional) => {
            applied_effect_contains_land_type_operation(conditional.then.effect)
        }
        EffectDef::StaticApply { effect, .. } => {
            applied_effect_contains_land_type_operation(effect)
        }
        _ => false,
    }
}

fn applied_effect_contains_land_type_operation(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .copied()
            .any(applied_effect_contains_land_type_operation),
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::BasicLandTypes(_)
            | CharacteristicOperationDef::ChosenBasicLandType,
        ) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::abilities;

    #[test]
    fn dynamic_draw_collapses_the_whole_prepared_root() {
        assert_eq!(
            compile_effect(EffectExecutorDef::DrawCards(ValueDef::ChosenX)),
            None
        );
    }

    #[test]
    fn constant_draw_prepares_to_an_intrinsic() {
        let prepared = abilities::prepared_draw_cards(ValueDef::Constant(3));
        assert_eq!(
            compile_effect(prepared.executor),
            Some(PreparedEffect::DrawCards { count: 3 })
        );
        assert_eq!(
            prepared.effect,
            EffectDef::DrawCards {
                recipient: crate::EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            }
        );
    }
}
