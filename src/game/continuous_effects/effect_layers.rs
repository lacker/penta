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
                | CharacteristicOperationDef::ChosenBasicLandTypeSubstitution
                | CharacteristicOperationDef::BasicLandTypes(_)
                | CharacteristicOperationDef::CardTypes(_)
                | CharacteristicOperationDef::Supertypes(_)
                | CharacteristicOperationDef::CreatureTypes(_)
                | CharacteristicOperationDef::AddChosenCreatureType
                | CharacteristicOperationDef::SetChosenCreatureType
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

impl Game {
    /// The first layer in which this static effect applies. Once an effect
    /// begins, its later components continue even if its ability is removed
    /// in layer 6 (CR 613.6).
    pub(in crate::game) fn static_effect_start_layer(effect: EffectDef) -> u8 {
        fn applied_layer(effect: AppliedEffectDef) -> u8 {
            match effect {
                AppliedEffectDef::Composite(effects) => effects
                    .iter()
                    .copied()
                    .map(applied_layer)
                    .min()
                    .unwrap_or(8),
                AppliedEffectDef::Characteristic(operation) => match operation {
                    CharacteristicOperationDef::Color(_)
                    | CharacteristicOperationDef::Colors(_) => 5,
                    CharacteristicOperationDef::Abilities(_) => 6,
                    CharacteristicOperationDef::PowerToughness(_) => 7,
                    _ => 4,
                },
                AppliedEffectDef::Rule(_) => 8,
            }
        }
        match effect {
            EffectDef::StaticApply { effect, .. } => applied_layer(effect),
            EffectDef::ConditionalStatic(conditional) => applied_layer(conditional.then.effect),
            _ => crate::card::child_effects(effect)
                .into_iter()
                .map(Self::static_effect_start_layer)
                .min()
                .unwrap_or(8),
        }
    }
}

impl Game {
    pub(in crate::game) fn static_effect_can_remove_static_abilities(effect: EffectDef) -> bool {
        fn removes(effect: AppliedEffectDef) -> bool {
            match effect {
                AppliedEffectDef::Composite(effects) => effects.iter().copied().any(removes),
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                    crate::card::AbilityOperationDef::Remove(
                        crate::card::AbilityPredicateDef::Any | crate::card::AbilityPredicateDef::Label(_),
                    ),
                )) => true,
                _ => false,
            }
        }
        match effect {
            EffectDef::StaticApply { effect, .. } => removes(effect),
            EffectDef::ConditionalStatic(conditional) => removes(conditional.then.effect),
            _ => crate::card::child_effects(effect).into_iter().any(Self::static_effect_can_remove_static_abilities),
        }
    }
}

impl StaticEffectKind {
    /// Reject unrelated static roots before asking live layer questions.
    /// This inspects structure only: conditions and recipient selection still
    /// run in the ordinary traversal, with stable component and grant IDs.
    fn may_be_supplied_by(self, effect: EffectDef) -> bool {
        fn includes(kind: StaticEffectKind, effect: AppliedEffectDef) -> bool {
            match effect {
                AppliedEffectDef::Composite(effects) => effects
                    .iter()
                    .copied()
                    .any(|effect| includes(kind, effect)),
                effect => kind.includes(effect),
            }
        }
        match effect {
            EffectDef::StaticApply { effect, .. } => includes(self, effect),
            EffectDef::ConditionalStatic(conditional) => includes(self, conditional.then.effect),
            _ => crate::card::child_effects(effect)
                .into_iter()
                .any(|effect| self.may_be_supplied_by(effect)),
        }
    }
}
