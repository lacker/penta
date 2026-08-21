impl Game {
    /// Whether the spell currently being paid for has an executable convoke
    /// clause on one of the parts in its selected form.
    fn payment_uses_convoke(&self, purpose: &ManaPaymentPurpose) -> bool {
        let ManaPaymentPurpose::Spell {
            definition, form, ..
        } = purpose
        else {
            return false;
        };
        let Some(definition) = self.catalog.get(*definition) else {
            return false;
        };
        let parts: &[crate::CardPartId] = match form {
            crate::card::SpellForm::Part(part) => core::slice::from_ref(part),
            crate::card::SpellForm::Combined(parts) => parts,
        };
        parts.iter().any(|part| {
            definition
                .part(*part)
                .is_some_and(|part| part.rules.has_executable_keyword(KeywordAbility::Convoke))
        })
    }

    /// The distinct ways one untapped creature can contribute to convoke.
    /// A colored creature has one choice per color; a colorless creature has
    /// a generic-only contribution that can never satisfy `{C}`.
    fn convoke_outputs(&self, permanent: &Permanent) -> ManaSourceOutputs {
        if permanent.tapped
            || !self
                .permanent_types(permanent)
                .is_some_and(|types| types.contains(CardType::Creature))
        {
            return Vec::new();
        }
        let mut outputs = Vec::new();
        for (color, present) in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
        ]
        .into_iter()
        .zip(self.permanent_colors(permanent))
        {
            if present {
                let mut production = ManaPool::default();
                production.add_color(color, 1);
                outputs.push(ManaSourceOutput {
                    kind: PlannedPaymentKind::Convoke,
                    production: ManaPool::default(),
                    convoke_production: production,
                    generic_payment: 0,
                    life_payment: 0,
                    benefits_payment: false,
                });
            }
        }
        if outputs.is_empty() {
            outputs.push(ManaSourceOutput {
                kind: PlannedPaymentKind::Convoke,
                production: ManaPool::default(),
                convoke_production: ManaPool::default(),
                generic_payment: 1,
                life_payment: 0,
                benefits_payment: false,
            });
        }
        outputs
    }

    /// A mana ability whose source remains untapped on the battlefield can be
    /// activated in 601.2g and then have that same source tapped for convoke
    /// in 601.2h. A tap or source-leaving cost makes the two uses mutually
    /// exclusive, which is the Llanowar Elves case.
    fn mana_activation_can_also_convoke(activation: &ManaAbilityActivation) -> bool {
        activation.cost_object != Some(activation.source)
            && !activation.costs.iter().any(|cost| {
                matches!(
                    cost,
                    AbilityCostDef::TapSource
                        | AbilityCostDef::SacrificeSource
                        | AbilityCostDef::ExileSource
                        | AbilityCostDef::ReturnSourceToHand
                        | AbilityCostDef::SacrificePermanents { .. }
                )
            })
    }

    /// An unlimited ability whose concrete cost consumes a different object
    /// may be activated once for each such object during the same payment.
    /// Keeping this deliberately narrow avoids inventing extra counter, tap,
    /// or once-per-turn capacity that the planner does not reserve.
    fn mana_activation_can_repeat_in_payment(
        &self,
        permanent: &Permanent,
        activation: &ManaAbilityActivation,
    ) -> bool {
        let distinct_sacrifice = activation.cost_object.is_some_and(|object| {
            object != activation.source
                && activation
                    .costs
                    .iter()
                    .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
                    .count()
                    == 1
                && activation.costs.iter().all(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::SacrificePermanent { .. } | AbilityCostDef::PayLife(_)
                    )
                })
        });
        distinct_sacrifice
            && self
                .find_effective_ability(permanent, |effective| {
                    effective.origin == activation.ability
                })
                .is_some_and(|effective| {
                    matches!(
                        effective.ability.definition,
                        DeclarativeAbilityDef::ActivatedMana(definition)
                            if definition.activation_limit.is_none()
                                && definition.condition.is_none()
                    )
                })
    }

    fn append_repeatable_convoke_mana_sources(
        &self,
        sources: &mut Vec<FlexibleManaSource>,
        permanent: &Permanent,
        activations: &[ManaAbilityActivation],
        outputs: &[ManaSourceOutput],
        order: usize,
    ) {
        if activations.iter().any(|activation| {
            activation.effect.sacrifice_source_when_out_of.is_some()
                || activation.costs.iter().any(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::SacrificeSource
                            | AbilityCostDef::ExileSource
                            | AbilityCostDef::ReturnSourceToHand
                    )
                })
        }) {
            return;
        }
        for (activation, output) in activations.iter().zip(outputs) {
            if self.mana_activation_can_repeat_in_payment(permanent, activation) {
                sources.push(FlexibleManaSource {
                    source: permanent.card.id,
                    outputs: vec![*output],
                    order,
                });
            }
        }
    }

    fn maximum_payment_from_permanent(&self, permanent: &Permanent, uses_convoke: bool) -> u16 {
        let activations = self.mana_ability_activations(permanent);
        let mut repeatable = Vec::<(GameObjectId, u16)>::new();
        let mut single = 0_u16;
        let mut single_with_convoke = 0_u16;
        let convoke = u16::from(uses_convoke && !self.convoke_outputs(permanent).is_empty());
        for activation in &activations {
            let amount = Self::mana_production(activation).total();
            if uses_convoke && self.mana_activation_can_repeat_in_payment(permanent, activation) {
                let object = activation.cost_object.expect("repeatable cost names an object");
                match repeatable.iter_mut().find(|(candidate, _)| *candidate == object) {
                    Some((_, maximum)) => *maximum = (*maximum).max(amount),
                    None => repeatable.push((object, amount)),
                }
            } else {
                single = single.max(amount);
                if Self::mana_activation_can_also_convoke(activation) {
                    single_with_convoke = single_with_convoke.max(amount.saturating_add(convoke));
                }
            }
        }
        repeatable
            .iter()
            .map(|(_, amount)| *amount)
            .fold(0_u16, u16::saturating_add)
            .saturating_add(single.max(convoke).max(single_with_convoke))
    }

    fn mana_and_convoke_outputs(
        activations: &[ManaAbilityActivation],
        mana_outputs: &[ManaSourceOutput],
        convoke_outputs: &[ManaSourceOutput],
    ) -> ManaSourceOutputs {
        let mut combined = Vec::new();
        for (activation, mana) in activations.iter().zip(mana_outputs) {
            if !Self::mana_activation_can_also_convoke(activation) {
                continue;
            }
            for convoke in convoke_outputs {
                let PlannedPaymentKind::Mana {
                    ability,
                    color,
                    counters_removed,
                    cost_object,
                    combination,
                    ..
                } = mana.kind
                else {
                    unreachable!("planned_outputs returns mana outputs")
                };
                combined.push(ManaSourceOutput {
                    kind: PlannedPaymentKind::Mana {
                        ability,
                        color,
                        counters_removed,
                        cost_object,
                        combination,
                        convokes: true,
                    },
                    production: mana.production,
                    convoke_production: convoke.convoke_production,
                    generic_payment: convoke.generic_payment,
                    life_payment: mana.life_payment,
                    benefits_payment: mana.benefits_payment,
                });
            }
        }
        combined
    }
}
