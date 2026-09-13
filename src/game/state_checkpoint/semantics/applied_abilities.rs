use super::{
    AbilityDef, AbilityOperationDef, AppliedEffectDef, AppliedRuleDef, CharacteristicOperationDef,
    EffectDef, child_effects,
};

pub(super) fn collect_applied_abilities(
    effect: AppliedEffectDef,
    abilities: &mut Vec<&'static AbilityDef>,
) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                collect_applied_abilities(*effect, abilities);
            }
        }
        AppliedEffectDef::Rule(AppliedRuleDef::MayPlot { ability, .. })
        | AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => abilities.push(ability),
        AppliedEffectDef::Rule(AppliedRuleDef::MayPlay(crate::card::PlayPermissionDef {
            benefit: Some(benefit),
            ..
        })) => {
            if let Some(ability) = benefit.on_play {
                abilities.push(ability);
            }
        }
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {}
    }
}

pub(super) fn has_dynamic_grant(ability: &AbilityDef) -> bool {
    fn applied(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::AddActivatedAbilitiesOf { .. },
            )) => true,
            AppliedEffectDef::Composite(effects) => effects.iter().copied().any(applied),
            _ => false,
        }
    }
    fn contains(effect: EffectDef) -> bool {
        matches!(effect, EffectDef::StaticApply { effect, .. } if applied(effect))
            || child_effects(effect).into_iter().any(contains)
    }
    ability.declarative_effect().is_some_and(contains)
}
