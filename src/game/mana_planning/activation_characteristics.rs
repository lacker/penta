// Mana-activation characteristics used by the payment planner.
//
// Included textually into `mana_planning.rs`, so the imports here are the
// parent module's.

impl Game {
    /// Whether this mana activation leaves the chosen permanent available to
    /// pay a later tap cost. A different source can be incompatible too when
    /// its mana ability would sacrifice the chosen permanent.
    pub(super) fn mana_activation_preserves_tap_cost_payer(
        permanent: &Permanent,
        activation: &ManaAbilityActivation,
        payer: GameObjectId,
    ) -> bool {
        if activation.cost_object == Some(payer) {
            return false;
        }
        if activation.source != payer {
            return true;
        }
        if activation.costs.iter().any(|cost| {
            matches!(
                cost,
                AbilityCostDef::TapSource
                    | AbilityCostDef::SacrificeSource
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::ReturnSourceToHand
            )
        }) {
            return false;
        }
        activation
            .effect
            .sacrifice_source_when_out_of
            .is_none_or(|kind| {
                let removed = activation.costs.iter().fold(0_u16, |removed, cost| {
                    if let AbilityCostDef::RemoveCountersFromSource {
                        kind: removed_kind,
                        amount,
                    } = cost
                        && *removed_kind == kind
                    {
                        return removed.saturating_add(*amount);
                    }
                    removed
                });
                permanent.counters(kind) > removed
            })
    }

    /// Whether an ability turns its own source into a creature.
    pub(super) fn effect_animates_source(effect: Option<EffectDef>) -> bool {
        match effect {
            Some(EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect,
                ..
            }) => Self::applied_effect_adds_creature_type(effect),
            Some(EffectDef::Sequence(effects)) => effects
                .iter()
                .any(|effect| Self::effect_animates_source(Some(*effect))),
            Some(EffectDef::Randomized {
                on_success,
                on_failure,
                ..
            }) => {
                Self::effect_animates_source(Some(*on_success))
                    || Self::effect_animates_source(Some(*on_failure))
            }
            Some(EffectDef::Choose(choice)) => Self::effect_animates_source(Some(*choice.then)),
            Some(EffectDef::PayOr(payment)) => payment
                .if_paid
                .iter()
                .chain(payment.otherwise.iter())
                .any(|effect| Self::effect_animates_source(Some(**effect))),
            Some(EffectDef::SplitIntoPiles(partition)) => {
                Self::effect_animates_source(Some(*partition.then))
            }
            _ => false,
        }
    }

    fn applied_effect_adds_creature_type(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_adds_creature_type),
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
                SetOperationDef::Add(types) | SetOperationDef::Set(types),
            )) => types.contains(CardType::Creature),
            _ => false,
        }
    }

    pub(super) fn activated_ability_mana_cost(definition: ActivatedAbilityDef) -> Option<ManaCost> {
        let mut cost = ManaCost::default();
        let mut has_mana_cost = false;
        for ability_cost in definition.costs.as_slice() {
            if let AbilityCostDef::Mana(mana) = ability_cost {
                cost = add_mana_cost(cost, *mana);
                has_mana_cost = true;
            }
        }
        has_mana_cost.then_some(cost)
    }
}
