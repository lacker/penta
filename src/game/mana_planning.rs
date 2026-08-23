use std::{collections::HashMap, ops::ControlFlow};

use crate::card::{CostModificationDef, FlexibleManaSymbol};

use super::{
    AbilityCostDef, AbilityOrigin, AbilityProcedureDef, Action, ActivatedAbilityDef,
    AppliedEffectDef, CardBehavior, CardDefinitionId, CardInstance, CardType,
    CharacteristicContext, CharacteristicOperationDef, CostConfiguration, DeclarativeAbilityDef,
    EffectDef, EffectRecipientDef, FlexibleManaSource, Game, GameObjectId, HybridPair,
    KeywordAbility, ManaAbilityActivation, ManaActivationChoices, ManaColor, ManaContributionKind,
    ManaCost, ManaPaymentPurpose, ManaPlanOptions, ManaPool, ManaSourceOutput, ManaSourceOutputs,
    PaymentCapacity, Permanent, PlannedManaActivation, PlannedPaymentKind, PlayActionKind,
    PlayOptionDef, PlayerId, SetOperationDef, TriggerContext, ValueDef, ZoneKind,
    extra_target_cost,
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
        let reserved = match action {
            Action::CastSpell { sacrifices, .. } => sacrifices.as_slice(),
            _ => &[],
        };
        let life_available = match action {
            Action::CastSpell { .. } => self.mana_ability_life_budget(player, &purpose),
            _ => u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX),
        };
        unique_payment_source_ids(
            self.plan_mana_activations(ManaPlanningRequest {
                player,
                cost,
                x,
                options,
                purpose: &purpose,
                reserved,
                life_available,
            })
            .unwrap_or_default(),
        )
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
                let increased = add_mana_cost(
                    add_generic(
                        cost,
                        extra_target_cost(definition, choices.iter_targets().count()),
                    ),
                    self.spell_cost_increase(player, *card),
                );
                let (locked, phyrexian_life) =
                    Self::locked_mana_payment(increased, choices.mana_payment())?;
                let cast_life = self.configured_cast_life_payment(
                    definition,
                    option,
                    *card,
                    choices.costs(),
                    choices.x(),
                    offer,
                );
                let library_life = self
                    .players
                    .iter()
                    .flat_map(|state| &state.library)
                    .find(|candidate| candidate.id == *card)
                    .and_then(|held| self.library_top_life_cost(held, player, option))
                    .unwrap_or(0);
                let total_life = cast_life
                    .saturating_add(library_life)
                    .saturating_add(phyrexian_life);
                Some((
                    reduce_generic(
                        locked,
                        self.spell_cost_reduction(definition.id, player, *card),
                    ),
                    choices.x(),
                    ManaPlanOptions::default(),
                    ManaPaymentPurpose::Spell {
                        object: *card,
                        definition: definition.id,
                        controller: player,
                        form: option.form.clone(),
                        reserved_life_payment: total_life,
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
        definition: &ActivatedAbilityDef,
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
            return Self::activated_ability_mana_cost(&definition).map(|cost| {
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
                | DeclarativeAbilityDef::OptionalAdditionalCost(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
        {
            let (options, purpose) = Self::battlefield_ability_mana_context(
                &definition,
                source,
                cost_objects,
                animates_source,
            );
            return Self::activated_ability_mana_cost(&definition)
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

    /// How much {C} a repeatable pay-life ongoing mana ability could still
    /// produce. Paying the last point is legal; state-based actions handle
    /// the resulting zero life after the casting procedure finishes.
    pub(super) fn repeatable_life_mana_available(&self, player: PlayerId) -> u16 {
        if self
            .repeatable_colorless_life_mana_activation(player)
            .is_none()
        {
            return 0;
        }
        u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(0)
    }

    /// Repeatable life-mana capacity after reserving life already committed
    /// by the spell being paid for.
    pub(super) fn repeatable_life_mana_available_for(
        &self,
        player: PlayerId,
        purpose: &ManaPaymentPurpose,
    ) -> u16 {
        if self
            .repeatable_colorless_life_mana_activation(player)
            .is_none()
        {
            return 0;
        }
        self.mana_ability_life_budget(player, purpose)
    }

    pub(super) fn mana_ability_life_budget(
        &self,
        player: PlayerId,
        purpose: &ManaPaymentPurpose,
    ) -> u16 {
        let reserved = match purpose {
            ManaPaymentPurpose::Spell {
                reserved_life_payment,
                ..
            } => *reserved_life_payment,
            ManaPaymentPurpose::Ability { .. } | ManaPaymentPurpose::Other => 0,
        };
        let reserved = i16::try_from(reserved).unwrap_or(i16::MAX);
        u16::try_from(self.players[player.index()].life.saturating_sub(reserved)).unwrap_or(0)
    }

    /// The generic mana this payment would be short if it drew only on the
    /// pool. Coloured and hybrid symbols come off first, exactly as the
    /// payment does, because the generated {C} cannot pay a coloured symbol
    /// and must not be counted against one.
    pub(super) fn generic_shortfall(pool: ManaPool, cost: ManaCost, x: u16) -> u16 {
        let mut spare = pool;
        for color in colored_mana() {
            spare.remove_color(color, mana_cost_amount(cost, color));
        }
        if cost.hybrid_total() > 0 {
            let hybrid = maximum_hybrid_payment(spare, cost, &|_| false);
            debug_assert_eq!(hybrid.total, hybrid_required_total(cost));
            for (pair, allocation) in HybridPair::ALL.into_iter().zip(hybrid.allocations) {
                let (first, second) = pair.colors();
                spare.remove_color(first, allocation[0]);
                spare.remove_color(second, allocation[1]);
            }
        }
        let colorless_paid = spare.colorless.min(cost.colorless);
        spare.remove_color(ManaColor::Colorless, colorless_paid);
        cost.colorless
            .saturating_sub(colorless_paid)
            .saturating_add(
                cost.generic
                    .saturating_add(x.saturating_mul(cost.x_multiplier))
                    .saturating_sub(spare.total()),
            )
    }

    /// Whether the player's floating pool alone covers this cost.
    ///
    /// A mana ability that costs mana is paid from the pool and nowhere
    /// else. Planning further activations to cover it would ask the planner
    /// about the very ability being planned, so the mana has to be there
    /// already: tap the land, then filter what it made.
    pub(super) fn pool_covers_cost(&self, player: PlayerId, cost: ManaCost) -> bool {
        let mut spare = self.eligible_mana_pool(player, &ManaPaymentPurpose::Other);
        spare.add_color(
            ManaColor::Colorless,
            self.repeatable_life_mana_available(player),
        );
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
                    kind: PlannedPaymentKind::Mana {
                        ability: activation.ability,
                        color: activation.color,
                        counters_removed: activation.counters_removed,
                        cost_object: activation.cost_object,
                        combination: activation.combination,
                        contribution: None,
                    },
                    production: Self::mana_production(activation),
                    colored_contribution: ManaPool::default(),
                    generic_payment: 0,
                    life_payment: activation
                        .costs
                        .iter()
                        .filter_map(|cost| match cost {
                            AbilityCostDef::PayLife(amount) => Some(*amount),
                            _ => None,
                        })
                        .fold(0, u16::saturating_add),
                    benefits_payment,
                }
            })
            .collect()
    }

    pub(super) fn plan_mana_activations_with_options_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        options: ManaPlanOptions,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let life_available =
            u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX);
        self.plan_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x,
            options,
            purpose,
            reserved: &[],
            life_available,
        })
    }

    pub(super) fn plan_mana_activations_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let life_available = self.mana_ability_life_budget(player, purpose);
        self.plan_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x,
            options: ManaPlanOptions {
                avoid,
                tap_cost_payer: None,
            },
            purpose,
            reserved: &[],
            life_available,
        })
    }

    pub(super) fn plan_mana_activations_for_reserving(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
        reserved: &[GameObjectId],
    ) -> Option<Vec<PlannedManaActivation>> {
        let life_available =
            u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX);
        self.plan_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x,
            options: ManaPlanOptions {
                avoid,
                tap_cost_payer: None,
            },
            purpose,
            reserved,
            life_available,
        })
    }

    fn plan_mana_activations(
        &self,
        request: ManaPlanningRequest<'_>,
    ) -> Option<Vec<PlannedManaActivation>> {
        order_mana_activations_before_consumption(
            self.assigned_mana_activations(request)?,
            request.cost,
        )
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
        let contributions = self.payment_contributions(purpose);
        let maximum = self.players[player.index()]
            .mana_pool
            .total()
            .saturating_add(
                self.battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == player)
                    .map(|permanent| self.maximum_payment_from_permanent(permanent, contributions))
                    .sum(),
            )
            .saturating_add(if contributions.delve {
                u16::try_from(self.players[player.index()].graveyard.len()).unwrap_or(u16::MAX)
            } else {
                0
            })
            .saturating_add(self.repeatable_life_mana_available(player));
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

    /// Whether a combat declaration cost remains payable. `tap_cost_payer`
    /// names an attacker that will tap before mana abilities may be activated;
    /// blockers and vigilant attackers pass `None` because they remain able
    /// to activate a mana ability while declaration costs are paid.
    pub(super) fn can_pay_declaration_cost(
        &self,
        player: PlayerId,
        cost: ManaCost,
        tap_cost_payer: Option<GameObjectId>,
    ) -> bool {
        let life_available =
            u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX);
        self.plan_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x: 0,
            options: ManaPlanOptions {
                avoid: None,
                tap_cost_payer,
            },
            purpose: &ManaPaymentPurpose::Other,
            reserved: &[],
            life_available,
        })
        .is_some()
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
    ) -> (ManaCost, u16) {
        self.activate_mana_for_cost_with_options_for(
            player,
            cost,
            x,
            ManaPlanOptions {
                avoid,
                tap_cost_payer: None,
            },
            purpose,
        )
    }

    pub(super) fn activate_mana_for_cost_with_options_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        options: ManaPlanOptions,
        purpose: &ManaPaymentPurpose,
    ) -> (ManaCost, u16) {
        self.activate_mana_for_cost_with_options_reserving_for(
            player,
            cost,
            x,
            options,
            purpose,
            &[],
        )
    }

    fn activate_mana_for_cost_with_options_reserving_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        options: ManaPlanOptions,
        purpose: &ManaPaymentPurpose,
        reserved: &[GameObjectId],
    ) -> (ManaCost, u16) {
        let life_available =
            u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX);
        let Some(plan) = self.plan_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x,
            options,
            purpose,
            reserved,
            life_available,
        }) else {
            panic!(
                "{}",
                self.unplannable_payment(player, cost, x, options.avoid, purpose)
            );
        };
        let residual = self.residual_cost_after_contributions(cost, x, purpose, &plan, false);
        // CR 601.2g comes before paying the spell's costs in 601.2h: activate
        // actual mana abilities first, then spend direct contributors.
        for payment in &plan {
            let PlannedPaymentKind::Mana {
                ability,
                color,
                counters_removed,
                cost_object,
                combination,
                ..
            } = payment.kind
            else {
                continue;
            };
            self.activate_mana_source(
                player,
                payment.source,
                ability,
                color,
                ManaActivationChoices {
                    counters_removed,
                    cost_object,
                    combination,
                },
            );
        }
        for payment in &plan {
            if payment
                .kind
                .contribution()
                .is_some_and(ManaContributionKind::taps_source)
            {
                self.tap_permanent(payment.source)
                    .expect("a planned contribution source remains on the battlefield");
            }
        }
        let exiled = plan
            .iter()
            .filter(|payment| {
                payment
                    .kind
                    .contribution()
                    .is_some_and(ManaContributionKind::exiles_source)
            })
            .map(|payment| payment.source)
            .collect::<Vec<_>>();
        self.exile_graveyard_cards(player, &exiled);
        residual
    }

    /// Removes selected Convoke, Delve, and Improvise contributions from a
    /// spell's total cost. The result is the mana-only remainder; the cast's
    /// chosen X itself remains frozen on the stack even though its payable
    /// generic portion has been folded into this remainder.
    pub(super) fn residual_cost_after_contributions(
        &self,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
        plan: &[PlannedManaActivation],
        planned_production_is_in_pool: bool,
    ) -> (ManaCost, u16) {
        if !self.payment_contributions(purpose).any() {
            return (cost, x);
        }
        let (mut residual, restricted_x) = self.restrict_x(cost, x, purpose);
        let mut actual = self.eligible_mana_pool(
            match purpose {
                ManaPaymentPurpose::Spell { controller, .. } => *controller,
                _ => unreachable!("only spell payments use direct contributions"),
            },
            purpose,
        );
        let mut convoke = ManaPool::default();
        let mut generic_only = 0_u16;
        for payment in plan {
            if !planned_production_is_in_pool {
                actual.add(payment.production);
            }
            if payment.kind.uses_contribution() {
                convoke.add(payment.colored_contribution);
                generic_only = generic_only.saturating_add(payment.generic_payment);
            }
        }
        for color in colored_mana() {
            let required = mana_cost_amount(residual, color);
            let paid = convoke.amount(color).min(required);
            convoke.remove_color(color, paid);
            actual.remove_color(color, required.saturating_sub(paid));
            match color {
                ManaColor::White => residual.white -= paid,
                ManaColor::Blue => residual.blue -= paid,
                ManaColor::Black => residual.black -= paid,
                ManaColor::Red => residual.red -= paid,
                ManaColor::Green => residual.green -= paid,
                ManaColor::Colorless => unreachable!("colored_mana excludes colorless"),
            }
        }
        if residual.hybrid_total() > 0 {
            let mut combined = actual;
            combined.add(convoke);
            let hybrid = maximum_hybrid_payment(combined, residual, &|_| false);
            debug_assert_eq!(hybrid.total, hybrid_required_total(residual));
            for (pair, allocation) in HybridPair::ALL.into_iter().zip(hybrid.allocations) {
                let (first, second) = pair.colors();
                let mut convoke_paid = 0_u16;
                for (color, assigned) in [(first, allocation[0]), (second, allocation[1])] {
                    let paid = convoke.amount(color).min(assigned);
                    convoke.remove_color(color, paid);
                    actual.remove_color(color, assigned.saturating_sub(paid));
                    convoke_paid = convoke_paid.saturating_add(paid);
                }
                residual.hybrid[pair.index()] =
                    residual.hybrid[pair.index()].saturating_sub(convoke_paid);
            }
        }
        let generic_due = residual
            .generic
            .saturating_add(restricted_x.saturating_mul(residual.x_multiplier));
        residual.generic = generic_due.saturating_sub(convoke.total().saturating_add(generic_only));
        residual.variable_x = false;
        residual.x_multiplier = 0;
        (residual, 0)
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
             \n  pool {:?}, eligible for this purpose {:?}, repeatable life mana {}",
            self.can_pay_cost_for(player, cost, x, purpose),
            self.players[player.index()].mana_pool,
            self.eligible_mana_pool(player, purpose),
            self.repeatable_life_mana_available(player),
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
    payment_remainder(pool, cost, x, &|_| 0, &ManaColor::ALL, false).is_some()
}

/// The selected mana cost before optional additional costs are added. An
/// effect that replaces "its mana cost" replaces this value and leaves those
/// additions intact (CR 118.9d).
pub(super) fn configured_base_mana_cost(
    option: &PlayOptionDef,
    configuration: &CostConfiguration,
) -> Option<ManaCost> {
    if let Some(selected) = configuration.alternative() {
        option
            .alternative_costs
            .iter()
            .find(|candidate| candidate.id == selected)
            .map(|candidate| candidate.mana_cost)
    } else {
        option.mana_cost
    }
}

include!("mana_planning/payment.rs");
include!("mana_planning/convoke.rs");
include!("mana_planning/source_assignment.rs");
include!("mana_planning/payment_order.rs");
include!("mana_planning/cost_reduction.rs");
include!("mana_planning/activation_characteristics.rs");
