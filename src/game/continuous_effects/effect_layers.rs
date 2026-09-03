impl Game {
    /// Whether this one applied effect begins choosing its recipients in
    /// layer 4. Later components keep that selection under CR 613.6.
    fn applied_effect_starts_in_type_layer(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_starts_in_type_layer),
            AppliedEffectDef::Characteristic(
                CharacteristicOperationDef::SetChosenBasicLandType
                | CharacteristicOperationDef::AddChosenBasicLandType
                | CharacteristicOperationDef::BasicLandTypes(_)
                | CharacteristicOperationDef::CardTypes(_)
                | CharacteristicOperationDef::CreatureTypes(_)
                | CharacteristicOperationDef::Subtypes(_),
            ) => true,
            AppliedEffectDef::Characteristic(
                CharacteristicOperationDef::Abilities(_)
                | CharacteristicOperationDef::Color(_)
                | CharacteristicOperationDef::Colors(_)
                | CharacteristicOperationDef::PowerToughness(_),
            )
            | AppliedEffectDef::Rule(_) => false,
        }
    }
}
