#[derive(Clone, Copy, Default)]
struct ManaContributionKinds {
    convoke: bool,
    delve: bool,
    improvise: bool,
}

impl ManaContributionKinds {
    const fn any(self) -> bool {
        self.convoke || self.delve || self.improvise
    }
}

impl Game {
    /// Which direct mana-cost contribution keywords are executable on the
    /// selected spell form.
    fn payment_contributions(&self, purpose: &ManaPaymentPurpose) -> ManaContributionKinds {
        let ManaPaymentPurpose::Spell {
            definition, form, ..
        } = purpose
        else {
            return ManaContributionKinds::default();
        };
        let Some(definition) = self.catalog.get(*definition) else {
            return ManaContributionKinds::default();
        };
        let parts: &[crate::CardPartId] = match form {
            crate::card::SpellForm::Part(part) => core::slice::from_ref(part),
            crate::card::SpellForm::Combined(parts) => parts,
        };
        let has = |keyword| {
            parts.iter().any(|part| {
                definition
                    .part(*part)
                    .is_some_and(|part| part.rules.has_executable_keyword(keyword))
            })
        };
        ManaContributionKinds {
            convoke: has(KeywordAbility::Convoke),
            delve: has(KeywordAbility::Delve),
            improvise: has(KeywordAbility::Improvise),
        }
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
                    kind: PlannedPaymentKind::Contribution(ManaContributionKind::Convoke),
                    production: ManaPool::default(),
                    colored_contribution: production,
                    generic_payment: 0,
                    life_payment: 0,
                    benefits_payment: false,
                });
            }
        }
        if outputs.is_empty() {
            outputs.push(ManaSourceOutput {
                kind: PlannedPaymentKind::Contribution(ManaContributionKind::Convoke),
                production: ManaPool::default(),
                colored_contribution: ManaPool::default(),
                generic_payment: 1,
                life_payment: 0,
                benefits_payment: false,
            });
        }
        outputs
    }

    /// A permanent can supply at most one tap-based contribution. Convoke
    /// may pay with a creature's color; improvise is always generic-only.
    fn permanent_contribution_outputs(
        &self,
        permanent: &Permanent,
        kinds: ManaContributionKinds,
    ) -> ManaSourceOutputs {
        let mut outputs = Vec::new();
        if kinds.convoke {
            outputs.extend(self.convoke_outputs(permanent));
        }
        if kinds.improvise
            && !permanent.tapped
            && self
                .permanent_types(permanent)
                .is_some_and(|types| types.contains(CardType::Artifact))
        {
            outputs.push(ManaSourceOutput {
                kind: PlannedPaymentKind::Contribution(ManaContributionKind::Improvise),
                production: ManaPool::default(),
                colored_contribution: ManaPool::default(),
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
    fn mana_activation_can_also_contribute(activation: &ManaAbilityActivation) -> bool {
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
                    .filter(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::SacrificePermanent { .. }
                                | AbilityCostDef::ExileCardFromHand(_)
                        )
                    })
                    .count()
                    == 1
                && activation.costs.iter().all(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::SacrificePermanent { .. }
                            | AbilityCostDef::ExileCardFromHand(_)
                            | AbilityCostDef::PayLife(_)
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

    fn append_repeatable_costed_mana_sources(
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

    fn maximum_payment_from_permanent(
        &self,
        permanent: &Permanent,
        contributions: ManaContributionKinds,
    ) -> u16 {
        let activations = self.mana_ability_activations(permanent);
        let mut repeatable = Vec::<(GameObjectId, u16)>::new();
        let mut single = 0_u16;
        let mut single_with_contribution = 0_u16;
        let contribution = self
            .permanent_contribution_outputs(permanent, contributions)
            .iter()
            .map(|output| output.payment_total())
            .max()
            .unwrap_or(0);
        for activation in &activations {
            let amount = Self::mana_production(activation).total();
            if self.mana_activation_can_repeat_in_payment(permanent, activation) {
                let object = activation.cost_object.expect("repeatable cost names an object");
                match repeatable.iter_mut().find(|(candidate, _)| *candidate == object) {
                    Some((_, maximum)) => *maximum = (*maximum).max(amount),
                    None => repeatable.push((object, amount)),
                }
            } else {
                single = single.max(amount);
                if Self::mana_activation_can_also_contribute(activation) {
                    single_with_contribution =
                        single_with_contribution.max(amount.saturating_add(contribution));
                }
            }
        }
        repeatable
            .iter()
            .map(|(_, amount)| *amount)
            .fold(0_u16, u16::saturating_add)
            .saturating_add(single.max(contribution).max(single_with_contribution))
    }

    fn mana_and_contribution_outputs(
        activations: &[ManaAbilityActivation],
        mana_outputs: &[ManaSourceOutput],
        contribution_outputs: &[ManaSourceOutput],
    ) -> ManaSourceOutputs {
        let mut combined = Vec::new();
        for (activation, mana) in activations.iter().zip(mana_outputs) {
            if !Self::mana_activation_can_also_contribute(activation) {
                continue;
            }
            for contribution in contribution_outputs {
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
                        contribution: contribution.kind.contribution(),
                    },
                    production: mana.production,
                    colored_contribution: contribution.colored_contribution,
                    generic_payment: contribution.generic_payment,
                    life_payment: mana.life_payment,
                    benefits_payment: mana.benefits_payment,
                });
            }
        }
        combined
    }
}
