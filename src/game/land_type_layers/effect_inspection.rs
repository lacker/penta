impl Game {
    const fn resolved_land_type_operation(
        operation: SetOperationDef<&'static [BasicLandType]>,
    ) -> LandTypeOperation {
        match operation {
            SetOperationDef::Add(types) => LandTypeOperation::Add(types),
            SetOperationDef::Remove(types) => LandTypeOperation::Remove(types),
            SetOperationDef::Set(types) => LandTypeOperation::SetTo(types),
        }
    }

    fn effect_contains_land_type_operation(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(Self::effect_contains_land_type_operation),
            effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
                let conditional = effect
                    .conditional()
                    .expect("conditional variants expose their shared shape");
                Self::effect_contains_land_type_operation(*conditional.then)
                    || conditional.otherwise.is_some_and(|otherwise| {
                        Self::effect_contains_land_type_operation(*otherwise)
                    })
            }
            EffectDef::ConditionalStatic(conditional) => {
                Self::applied_effect_contains_land_type_operation(conditional.then.effect)
            }
            EffectDef::StaticApply { effect, .. } => {
                Self::applied_effect_contains_land_type_operation(effect)
            }
            _ => false,
        }
    }

    fn applied_effect_contains_land_type_operation(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_contains_land_type_operation),
            AppliedEffectDef::Characteristic(
                CharacteristicOperationDef::BasicLandTypes(_)
                | CharacteristicOperationDef::SetChosenBasicLandType
                | CharacteristicOperationDef::AddChosenBasicLandType,
            ) => true,
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
        }
    }
}
