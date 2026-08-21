use std::ops::ControlFlow;

use super::{
    AbilityCostDef, AbilityOrigin, AbilityProcedureDef, Action, ActivatedAbilityDef,
    AppliedEffectDef, CardBehavior, CardDefinitionId, CardInstance, CardType,
    CharacteristicContext, CharacteristicOperationDef, CostConfiguration, DeclarativeAbilityDef,
    EffectDef, EffectRecipientDef, FlexibleManaSource, Game, GameObjectId, HybridPair,
    ManaAbilityActivation, ManaActivationChoices, ManaColor, ManaCost, ManaPaymentPurpose,
    ManaPlanOptions, ManaPool, ManaSourceOutput, ManaSourceOutputs, Permanent,
    PlannedManaActivation, PlayActionKind, PlayOptionDef, PlayerId, SetOperationDef,
    TriggerContext, ValueDef, ZoneKind, extra_target_cost,
};

impl Game {
    /// Returns the mana sources the engine's default payment policy would tap
    /// for an action. This is a read-only preview for clients; applying the
    /// action still performs the authoritative payment and validation.
    #[must_use]
    pub fn mana_sources_for_action(&self, player: PlayerId, action: &Action) -> Vec<GameObjectId> {
        let Some((cost, x, options, purpose)) = self.mana_requirement(player, action) else {
            return Vec::new();
        };
        self.plan_mana_sources(player, cost, x, options, &purpose)
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn mana_requirement(
        &self,
        player: PlayerId,
        action: &Action,
    ) -> Option<(ManaCost, u16, ManaPlanOptions, ManaPaymentPurpose)> {
        match action {
            Action::CastSpell { card, choices, .. } => {
                let definition = self
                    .players
                    .iter()
                    .flat_map(|player| player.hand.iter().chain(&player.graveyard))
                    .find(|candidate| candidate.id == *card)
                    .and_then(|candidate| self.catalog.get(candidate.definition))?;
                let option = definition.play_option(choices.play_option())?;
                let offer = self
                    .pending_decisions
                    .first()
                    .and_then(|pending| pending.continuation.cast_offer())
                    .filter(|offer| offer.player == player && offer.card == *card)
                    .map(|offer| offer.cost);
                let cost = self.configured_cast_mana_cost(*card, option, choices.costs(), offer)?;
                Some((
                    reduce_generic(
                        add_mana_cost(
                            add_generic(
                                cost,
                                extra_target_cost(definition, choices.iter_targets().count()),
                            ),
                            self.spell_cost_increase(player, *card),
                        ),
                        self.spell_cost_reduction(definition.id, player, *card),
                    ),
                    choices.x(),
                    ManaPlanOptions::default(),
                    ManaPaymentPurpose::Spell {
                        object: *card,
                        definition: definition.id,
                        controller: player,
                        form: option.form.clone(),
                    },
                ))
            }
            Action::ActivateAbility {
                source,
                ability,
                cost_objects,
                x,
                ..
            } => self.ability_mana_requirement(player, *source, *ability, cost_objects, *x),
            _ => None,
        }
    }

    fn battlefield_ability_mana_context(
        definition: ActivatedAbilityDef,
        source: GameObjectId,
        cost_objects: &[GameObjectId],
        animates_source: bool,
    ) -> (ManaPlanOptions, ManaPaymentPurpose) {
        let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
        let leaves_source = definition.costs.iter().any(|cost| {
            matches!(
                cost,
                AbilityCostDef::SacrificeSource
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::ReturnSourceToHand
            )
        });
        let tap_cost_payer = if definition
            .costs
            .iter()
            .any(|cost| matches!(cost, AbilityCostDef::TapPermanent { .. }))
        {
            cost_objects.first().copied()
        } else {
            None
        };
        (
            ManaPlanOptions {
                // Tapping the source to pay would hand back a tapped
                // creature, so auto-payment leaves it alone even though the
                // tap itself is legal.
                avoid: (taps_source || animates_source).then_some(source),
                tap_cost_payer,
            },
            ManaPaymentPurpose::Ability {
                source,
                taps_source,
                leaves_source,
            },
        )
    }

    /// The mana half of an activation cost, and how the payment should treat
    /// the ability's own source.
    pub(super) fn ability_mana_requirement(
        &self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        cost_objects: &[GameObjectId],
        x: u16,
    ) -> Option<(ManaCost, u16, ManaPlanOptions, ManaPaymentPurpose)> {
        if let Some(card) = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == source)
            && let Some(definition) = self
                .find_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
                    effective.origin == ability
                        && effective.ability.is_executable()
                        && matches!(
                            effective.ability.definition,
                            DeclarativeAbilityDef::Activated(definition)
                                if definition.procedure == AbilityProcedureDef::Shared
                        )
                })
                .and_then(|effective| match effective.ability.definition {
                    DeclarativeAbilityDef::Activated(definition)
                        if definition.source_zones.contains(&ZoneKind::Hand) =>
                    {
                        Some(definition)
                    }
                    _ => None,
                })
        {
            return Self::activated_ability_mana_cost(definition).map(|cost| {
                (
                    cost,
                    x,
                    ManaPlanOptions::default(),
                    ManaPaymentPurpose::Ability {
                        source,
                        taps_source: false,
                        leaves_source: false,
                    },
                )
            });
        }

        let permanent = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)?;
        if let Some((definition, animates_source)) = self
            .find_effective_ability(permanent, |effective| effective.origin == ability)
            .and_then(|effective| match effective.ability.definition {
                DeclarativeAbilityDef::Activated(definition) => Some((
                    definition,
                    Self::effect_animates_source(effective.ability.declarative_effect()),
                )),
                DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::TriggeredMana(_)
                | DeclarativeAbilityDef::Triggered(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
        {
            let (options, purpose) = Self::battlefield_ability_mana_context(
                definition,
                source,
                cost_objects,
                animates_source,
            );
            return Self::activated_ability_mana_cost(definition)
                .map(|cost| (cost, x, options, purpose));
        }

        let behavior = self.effective_behavior(permanent)?;
        let cost = match behavior {
            CardBehavior::SedgeTroll => ManaCost::colored(0, 0, 0, 1, 0, 0),
            _ => return None,
        };
        Some((
            cost,
            0,
            ManaPlanOptions::default(),
            ManaPaymentPurpose::Ability {
                source,
                taps_source: false,
                leaves_source: false,
            },
        ))
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

    pub(super) fn plan_mana_sources(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        options: ManaPlanOptions,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<GameObjectId> {
        self.plan_mana_activations_with_options_for(player, cost, x, options, purpose)
            .unwrap_or_default()
            .into_iter()
            .map(|activation| activation.source)
            .collect()
    }

    /// How much {C} Channel could still produce for this player. The last
    /// point of life is not spendable, which is the same limit the priority
    /// action already enforces.
    pub(super) fn channel_mana_available(&self, player: PlayerId) -> u16 {
        if !self.channel_active[player.index()] {
            return 0;
        }
        u16::try_from(self.players[player.index()].life.saturating_sub(1)).unwrap_or(0)
    }

    /// The generic mana this payment would be short if it drew only on the
    /// pool. Coloured and hybrid symbols come off first, exactly as the
    /// payment does, because Channel's {C} cannot pay a coloured symbol and
    /// must not be counted against one.
    pub(super) fn generic_shortfall(pool: ManaPool, cost: ManaCost, x: u16) -> u16 {
        let mut spare = pool;
        for color in colored_mana() {
            spare.remove_color(color, mana_cost_amount(cost, color));
        }
        for pair in HybridPair::ALL {
            let mut remaining = cost.hybrid[pair.index()];
            let (first, second) = pair.colors();
            for color in [first, second] {
                let spent = spare.amount(color).min(remaining);
                spare.remove_color(color, spent);
                remaining -= spent;
            }
        }
        cost.generic
            .saturating_add(x.saturating_mul(cost.x_multiplier))
            .saturating_sub(spare.total())
    }

    /// Whether the player's floating pool alone covers this cost.
    ///
    /// A mana ability that costs mana is paid from the pool and nowhere
    /// else. Planning further activations to cover it would ask the planner
    /// about the very ability being planned, so the mana has to be there
    /// already: tap the land, then filter what it made.
    pub(super) fn pool_covers_cost(&self, player: PlayerId, cost: ManaCost) -> bool {
        let mut spare = self.eligible_mana_pool(player, &ManaPaymentPurpose::Other);
        spare.add_color(ManaColor::Colorless, self.channel_mana_available(player));
        for color in colored_mana() {
            let required = mana_cost_amount(cost, color);
            if spare.amount(color) < required {
                return false;
            }
            spare.remove_color(color, required);
        }
        spare.total() >= cost.generic
    }

    /// One planning record per enumerated activation.
    fn planned_outputs(
        activations: &[ManaAbilityActivation],
        purpose: &ManaPaymentPurpose,
    ) -> ManaSourceOutputs {
        activations
            .iter()
            .map(|activation| {
                let benefits_payment = Self::mana_for_activation(activation)
                    .first()
                    .is_some_and(|mana| Self::mana_has_spend_effect_for(*mana, purpose));
                ManaSourceOutput {
                    ability: activation.ability,
                    color: activation.color,
                    production: Self::mana_production(activation),
                    benefits_payment,
                    counters_removed: activation.counters_removed,
                    cost_object: activation.cost_object,
                    combination: activation.combination,
                }
            })
            .collect()
    }

    pub(super) fn assigned_mana_activations_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        self.assigned_mana_activations_with_options(
            player,
            cost,
            x,
            ManaPlanOptions::default(),
            purpose,
        )
    }

    /// Whether this mana activation leaves the chosen permanent available to
    /// pay a later tap cost. A different source can be incompatible too when
    /// its mana ability would sacrifice the chosen permanent.
    fn mana_activation_preserves_tap_cost_payer(
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

    fn assigned_mana_activations_with_options(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        options: ManaPlanOptions,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let (cost, x) = self.restrict_x(cost, x, purpose);
        let mut pool = self.eligible_mana_pool(player, purpose);
        // Channel lets a player make {C} any time they could activate a mana
        // ability, which includes the middle of paying for this spell.
        pool.add_color(ManaColor::Colorless, self.channel_mana_available(player));
        let mut assigned = Vec::new();
        let mut flexible = Vec::new();
        // An ability that taps its source as a cost cannot also tap it for
        // mana, so that source is not a candidate at all.
        let barred = match purpose {
            ManaPaymentPurpose::Ability {
                source,
                taps_source: true,
                ..
            } => Some(*source),
            _ => None,
        };
        for (order, permanent) in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| Some(permanent.card.id) != barred)
            .enumerate()
        {
            let mut activations = self
                .mana_ability_activations(permanent)
                .into_iter()
                .filter(|activation| {
                    let preserves_tap_cost_payer = options.tap_cost_payer.is_none_or(|payer| {
                        Self::mana_activation_preserves_tap_cost_payer(permanent, activation, payer)
                    });
                    let preserves_required_source = !matches!(
                        purpose,
                        ManaPaymentPurpose::Ability {
                            source,
                            leaves_source: true,
                            ..
                        } if *source == activation.source
                    ) || !activation.costs.iter().any(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::SacrificeSource
                                | AbilityCostDef::ExileSource
                                | AbilityCostDef::ReturnSourceToHand
                        )
                    });
                    // An activation that itself costs mana is left to the
                    // player. The plan adds each source's production to a
                    // running pool, and one that also spends from that pool
                    // would be counted as free.
                    let costs_mana = activation
                        .costs
                        .iter()
                        .any(|cost| matches!(cost, AbilityCostDef::Mana(_)));
                    Self::mana_for_activation(activation)
                        .first()
                        .is_some_and(|mana| self.mana_can_pay_for(*mana, purpose))
                        && preserves_tap_cost_payer
                        && preserves_required_source
                        && !costs_mana
                })
                .collect::<Vec<_>>();
            // When several outputs are legal, prefer one whose spend rider
            // benefits this payment. Players can still manually choose a
            // different mana ability before casting.
            activations.sort_by_key(|activation| {
                let benefits_payment = Self::mana_for_activation(activation)
                    .first()
                    .is_some_and(|mana| Self::mana_has_spend_effect_for(*mana, purpose));
                let production = Self::mana_production(activation);
                let pays_colored_symbol = colored_mana().into_iter().any(|color| {
                    production.amount(color) > 0
                        && (mana_cost_amount(cost, color) > 0 || hybrid_pays_with(cost, color))
                });
                (!benefits_payment, !pays_colored_symbol)
            });
            let outputs = Self::planned_outputs(&activations, purpose);
            match outputs.as_slice() {
                [] => {}
                [output] => {
                    pool.add(output.production);
                    assigned.push(PlannedManaActivation {
                        source: permanent.card.id,
                        ability: output.ability,
                        color: output.color,
                        production: output.production,
                        benefits_payment: output.benefits_payment,
                        flexibility: 1,
                        order,
                        counters_removed: output.counters_removed,
                        cost_object: output.cost_object,
                        combination: output.combination,
                    });
                }
                _ => flexible.push(FlexibleManaSource {
                    source: permanent.card.id,
                    outputs,
                    order,
                }),
            }
        }

        let mut flexible_assignment = Vec::new();
        if !assign_flexible_mana_outputs(&flexible, 0, pool, cost, x, &mut flexible_assignment) {
            return None;
        }
        assigned.extend(flexible_assignment);
        Some(assigned)
    }

    pub(super) fn plan_mana_activations_with_options_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        options: ManaPlanOptions,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let mut available =
            self.assigned_mana_activations_with_options(player, cost, x, options, purpose)?;
        let (cost, x) = self.restrict_x(cost, x, purpose);
        let mut pool = self.eligible_mana_pool(player, purpose);
        // The payment tops the pool up from Channel before it spends, so the
        // plan has to count that mana too or it will tap sources for it.
        pool.add_color(ManaColor::Colorless, self.channel_mana_available(player));
        let mut selected = Vec::new();

        for color in colored_mana() {
            let required = mana_cost_amount(cost, color);
            while pool.amount(color) < required {
                let index = available
                    .iter()
                    .enumerate()
                    // Read from what the activation makes rather than from
                    // the colour that labels it: an ability making two
                    // unlike mana pays for either.
                    .filter(|(_, activation)| activation.production.amount(color) > 0)
                    .min_by_key(|(_, activation)| {
                        (
                            Some(activation.source) == options.avoid,
                            !activation.benefits_payment,
                            activation.flexibility,
                            activation.production.total(),
                            activation.order,
                        )
                    })
                    .map(|(index, _)| index)?;
                let activation = available.remove(index);
                pool.add(activation.production);
                selected.push(activation);
            }
        }

        // Each pair is satisfied in turn. No printed cost mixes pairs that
        // share a colour, so taking them one at a time cannot strand a symbol
        // another pair had already claimed.
        for pair in HybridPair::ALL {
            let needed = cost.hybrid[pair.index()];
            while available_hybrid(pool, cost, pair) < needed {
                let index = available
                    .iter()
                    .enumerate()
                    .filter(|(_, activation)| {
                        colored_mana().into_iter().any(|color| {
                            pair.contains(color) && activation.production.amount(color) > 0
                        })
                    })
                    .min_by_key(|(_, activation)| {
                        (
                            Some(activation.source) == options.avoid,
                            !activation.benefits_payment,
                            activation.flexibility,
                            activation.production.total(),
                            activation.order,
                        )
                    })
                    .map(|(index, _)| index)?;
                let activation = available.remove(index);
                pool.add(activation.production);
                selected.push(activation);
            }
        }

        let required_total = colored_cost_total(cost)
            .saturating_add(cost.generic)
            .saturating_add(x.saturating_mul(cost.x_multiplier));
        while pool.total() < required_total {
            let index = available
                .iter()
                .enumerate()
                .min_by_key(|(_, activation)| {
                    (
                        Some(activation.source) == options.avoid,
                        !activation.benefits_payment,
                        activation.production.amount(ManaColor::Colorless) == 0,
                        activation.production.total(),
                        activation.order,
                    )
                })
                .map(|(index, _)| index)?;
            let activation = available.remove(index);
            pool.add(activation.production);
            selected.push(activation);
        }

        debug_assert!(can_pay(pool, cost, x));
        Some(selected)
    }

    /// "Players can't cast spells or play lands with ..." Read while play
    /// options are offered, so a prohibited card is simply not a legal action.
    /// The prohibition is a property of the card, not of who holds it, so it
    /// applies to both players.
    pub(super) fn play_is_prohibited(
        &self,
        card: &CardInstance,
        controller: PlayerId,
        option: &PlayOptionDef,
    ) -> bool {
        let context = match option.action {
            PlayActionKind::CastSpell => CharacteristicContext::Stack {
                form: option.form.clone(),
            },
            // Land plays never become stack objects. The current authored
            // land restriction is set-origin based, for which the ordinary
            // hand characteristics are exact.
            PlayActionKind::PlayLand => CharacteristicContext::Hand,
        };
        let Some(object) =
            self.printed_trigger_event_object(card.id, card.definition, controller, &context)
        else {
            return false;
        };
        self.visit_play_restrictions(controller, |applied| {
            if applied.restriction.action.matches(option.action)
                && self.trigger_object_matches(
                    applied.restriction.object,
                    &object,
                    applied.source,
                    option.action == PlayActionKind::CastSpell,
                )
            {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    pub(super) fn maximum_x_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        purpose: &ManaPaymentPurpose,
    ) -> u16 {
        let maximum = self.players[player.index()]
            .mana_pool
            .total()
            .saturating_add(
                self.battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == player)
                    .filter_map(|permanent| {
                        self.mana_ability_activations(permanent)
                            .into_iter()
                            .map(|activation| Self::mana_production(&activation))
                            .max_by_key(|production| production.total())
                    })
                    .map(ManaPool::total)
                    .sum(),
            )
            .saturating_add(self.channel_mana_available(player));
        // The upper bound is only a search ceiling; can_pay_cost_for is
        // what rules each X in or out, including the barred source.
        (0..=maximum)
            .rev()
            .find(|x| self.can_pay_cost_for(player, cost, *x, purpose))
            .unwrap_or(0)
    }

    pub(super) fn activate_mana_for_cost(&mut self, player: PlayerId, cost: ManaCost, x: u16) {
        self.activate_mana_for_cost_avoiding(player, cost, x, None);
    }

    pub(super) fn activate_mana_for_cost_avoiding(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
    ) {
        self.activate_mana_for_cost_avoiding_for(
            player,
            cost,
            x,
            avoid,
            &ManaPaymentPurpose::Other,
        );
    }

    pub(super) fn activate_mana_for_cost_avoiding_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) {
        self.activate_mana_for_cost_with_options_for(
            player,
            cost,
            x,
            ManaPlanOptions {
                avoid,
                tap_cost_payer: None,
            },
            purpose,
        );
    }

    pub(super) fn activate_mana_for_cost_with_options_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        options: ManaPlanOptions,
        purpose: &ManaPaymentPurpose,
    ) {
        let Some(plan) =
            self.plan_mana_activations_with_options_for(player, cost, x, options, purpose)
        else {
            panic!(
                "{}",
                self.unplannable_payment(player, cost, x, options.avoid, purpose)
            );
        };
        for activation in plan {
            self.activate_mana_source(
                player,
                activation.source,
                activation.ability,
                activation.color,
                ManaActivationChoices {
                    counters_removed: activation.counters_removed,
                    cost_object: activation.cost_object,
                    combination: activation.combination,
                },
            );
        }
    }

    /// Describes a payment that passed its affordability gate and then found
    /// no plan. The invariant still fails hard, because a half-paid cost is
    /// not a state the engine can continue from -- but the report carries
    /// what the planner saw, so an occurrence that is hard to reproduce is
    /// still worth something to whoever reads the crash.
    #[cold]
    pub(super) fn unplannable_payment(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) -> String {
        use std::fmt::Write as _;

        let mut report = String::from("a legal payment has a complete mana activation plan");
        let _ = write!(
            report,
            "\n  cost {cost} with x {x} for {player:?}, purpose {purpose:?}, avoiding {avoid:?}\
             \n  affordable per the gate: {}\
             \n  pool {:?}, eligible for this purpose {:?}, channel {}",
            self.can_pay_cost_for(player, cost, x, purpose),
            self.players[player.index()].mana_pool,
            self.eligible_mana_pool(player, purpose),
            self.channel_mana_available(player),
        );
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
        {
            let activations = self.mana_ability_activations(permanent);
            if activations.is_empty() {
                continue;
            }
            let name = self
                .presentation_name(Self::effective_rules_source(permanent))
                .unwrap_or_else(|| "?".into());
            let colors: Vec<ManaColor> = activations
                .iter()
                .map(|activation| activation.color)
                .collect();
            let _ = write!(
                report,
                "\n  source {:?} {name}{} produces {colors:?}",
                permanent.card.id,
                if permanent.tapped { " (tapped)" } else { "" },
            );
        }
        report
    }
}

pub(super) fn can_pay(pool: ManaPool, cost: ManaCost, x: u16) -> bool {
    pool.white >= cost.white
        && pool.blue >= cost.blue
        && pool.black >= cost.black
        && pool.red >= cost.red
        && pool.green >= cost.green
        && pool.colorless >= cost.colorless
        && HybridPair::ALL
            .into_iter()
            .all(|pair| available_hybrid(pool, cost, pair) >= cost.hybrid[pair.index()])
        && pool.total()
            >= colored_cost_total(cost)
                .saturating_add(cost.generic)
                .saturating_add(x.saturating_mul(cost.x_multiplier))
}

pub(super) fn assign_flexible_mana_outputs(
    sources: &[FlexibleManaSource],
    index: usize,
    pool: ManaPool,
    cost: ManaCost,
    x: u16,
    assignment: &mut Vec<PlannedManaActivation>,
) -> bool {
    // A colour assignment cannot make up a total-mana shortfall. Prune before
    // branching over every output of every remaining source; otherwise a
    // large group of flexible sources can make an impossible high-X probe
    // exponential even though their combined production is already too low.
    let maximum_total = sources[index..]
        .iter()
        .filter_map(|source| {
            source
                .outputs
                .iter()
                .map(|output| output.production.total())
                .max()
        })
        .fold(pool.total(), u16::saturating_add);
    let required_total = colored_cost_total(cost)
        .saturating_add(cost.generic)
        .saturating_add(x.saturating_mul(cost.x_multiplier));
    if maximum_total < required_total {
        return false;
    }

    let Some(source) = sources.get(index) else {
        return can_pay(pool, cost, x);
    };
    for output in &source.outputs {
        let mut next = pool;
        next.add(output.production);
        assignment.push(PlannedManaActivation {
            source: source.source,
            ability: output.ability,
            color: output.color,
            production: output.production,
            benefits_payment: output.benefits_payment,
            flexibility: source.outputs.len(),
            order: source.order,
            counters_removed: output.counters_removed,
            cost_object: output.cost_object,
            combination: output.combination,
        });
        if assign_flexible_mana_outputs(sources, index + 1, next, cost, x, assignment) {
            return true;
        }
        assignment.pop();
    }
    false
}

pub(super) fn configured_mana_cost(
    option: &PlayOptionDef,
    configuration: &CostConfiguration,
) -> Option<ManaCost> {
    let mut cost = if let Some(selected) = configuration.alternative() {
        option
            .alternative_costs
            .iter()
            .find(|candidate| candidate.id == selected)
            .map(|candidate| candidate.mana_cost)?
    } else {
        option.mana_cost?
    };
    for selected in configuration.additional() {
        let additional = option
            .additional_costs
            .iter()
            .find(|candidate| candidate.id == *selected)?;
        if let Some(mana) = additional.mana_cost {
            cost = add_mana_cost(cost, mana);
        }
    }
    Some(cost)
}

include!("mana_planning/payment.rs");
include!("mana_planning/cost_reduction.rs");
