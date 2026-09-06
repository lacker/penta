fn triggered_mana_program_is_immediate(effect: EffectDef) -> bool {
    match effect {
        EffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(triggered_mana_program_is_immediate)
        }
        EffectDef::AddMana(mana) => {
            mana.amount > 0
                && mana.variable_amount.is_none()
                && matches!(
                    mana.recipient,
                    crate::card::PlayerRefDef::EffectController
                        | crate::card::PlayerRefDef::ControllerOf(
                            crate::card::ObjectRefDef::TriggeringObject
                        )
                )
                && match mana.mana {
                    crate::card::ManaSelectionDef::One(_) => true,
                    crate::card::ManaSelectionDef::Choice(types)
                    | crate::card::ManaSelectionDef::Combination(types) => match types.source {
                        crate::card::ManaTypeSourceDef::Fixed(colors) => !colors.is_empty(),
                        crate::card::ManaTypeSourceDef::ProducedBy(
                            crate::card::ObjectRefDef::TriggeringObject,
                        ) => true,
                        crate::card::ManaTypeSourceDef::ProducedBy(_)
                        | crate::card::ManaTypeSourceDef::CouldBeProducedBy(_) => false,
                    },
                    crate::card::ManaSelectionDef::ColorsOfLinkedExiles
                    | crate::card::ManaSelectionDef::ChoiceOfBundles(_) => false,
                }
        }
        _ => false,
    }
}
