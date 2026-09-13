/// Layer-6 grants currently feed only the layer-7 static-effect reader.
/// Other lanes still require their own live discovery/dependency support.
fn granted_static_power_toughness_supported(effect: EffectDef) -> bool {
    match effect {
        EffectDef::StaticApply { effect, .. } => granted_static_stat_components(effect),
        EffectDef::ConditionalStatic(conditional) => {
            granted_static_stat_components(conditional.then.effect)
        }
        EffectDef::IfCondition { then, .. } => granted_static_power_toughness_supported(*then),
        EffectDef::IfElseCondition {
            then, otherwise, ..
        } => {
            granted_static_power_toughness_supported(*then)
                && granted_static_power_toughness_supported(*otherwise)
        }
        EffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(granted_static_power_toughness_supported)
        }
        _ => false,
    }
}

fn granted_static_stat_components(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(_)) => true,
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty() && effects.iter().copied().all(granted_static_stat_components)
        }
        _ => false,
    }
}
