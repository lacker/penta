use super::{
    AbilityCostDef, AbilityOrigin, AbilityProcedureDef, Action, ActivatedAbilityDef,
    AppliedEffectDef, CardBehavior, CardDefinitionId, CardInstance, CardRules,
    CharacteristicContext, CostConfiguration, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    FlexibleManaSource, Game, GameObjectId, HybridPair, ManaColor, ManaCost, ManaPaymentPurpose,
    ManaPool, PlannedManaActivation, PlayOptionDef, PlayerId, TriggerContext, ValueDef, ZoneKind,
    extra_target_cost,
};

impl Game {
    /// Returns the mana sources the engine's default payment policy would tap
    /// for an action. This is a read-only preview for clients; applying the
    /// action still performs the authoritative payment and validation.
    #[must_use]
    pub fn mana_sources_for_action(&self, player: PlayerId, action: &Action) -> Vec<GameObjectId> {
        let Some((cost, x, avoid, purpose)) = self.mana_requirement(player, action) else {
            return Vec::new();
        };
        self.plan_mana_sources(player, cost, x, avoid, &purpose)
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn mana_requirement(
        &self,
        player: PlayerId,
        action: &Action,
    ) -> Option<(ManaCost, u16, Option<GameObjectId>, ManaPaymentPurpose)> {
        match action {
            Action::CastSpell { card, choices, .. } => {
                let definition = self
                    .players
                    .iter()
                    .flat_map(|player| player.hand.iter().chain(&player.graveyard))
                    .find(|candidate| candidate.id == *card)
                    .and_then(|candidate| self.catalog.get(candidate.definition))?;
                let option = definition.play_option(choices.play_option())?;
                let cost = self.configured_cast_mana_cost(*card, option, choices.costs())?;
                Some((
                    reduce_generic(
                        add_generic(
                            cost,
                            extra_target_cost(definition, choices.iter_targets().count()),
                        ),
                        self.spell_cost_reduction(definition.id, player),
                    ),
                    choices.x(),
                    None,
                    ManaPaymentPurpose::Spell {
                        object: *card,
                        definition: definition.id,
                        controller: player,
                        form: option.form.clone(),
                    },
                ))
            }
            Action::ActivateAbility {
                source, ability, x, ..
            } => self.ability_mana_requirement(player, *source, *ability, *x),
            _ => None,
        }
    }

    /// The mana half of an activation cost, and how the payment should treat
    /// the ability's own source.
    pub(super) fn ability_mana_requirement(
        &self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        x: u16,
    ) -> Option<(ManaCost, u16, Option<GameObjectId>, ManaPaymentPurpose)> {
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
                    None,
                    ManaPaymentPurpose::Ability {
                        source,
                        taps_source: false,
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
            let cost = Self::activated_ability_mana_cost(definition);
            let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
            return cost.map(|cost| {
                (
                    cost,
                    x,
                    // Tapping the source to pay would hand back a tapped
                    // creature, so auto-payment leaves it alone even though
                    // the tap itself is legal.
                    (taps_source || animates_source).then_some(source),
                    ManaPaymentPurpose::Ability {
                        source,
                        taps_source,
                    },
                )
            });
        }

        let behavior = self.effective_behavior(permanent)?;
        let cost = match behavior {
            CardBehavior::ChaosOrb | CardBehavior::NevinyrralsDisk => ManaCost::new(1, 0),
            CardBehavior::SedgeTroll => ManaCost::colored(0, 0, 0, 1, 0, 0),
            _ => return None,
        };
        Some((
            cost,
            0,
            None,
            ManaPaymentPurpose::Ability {
                source,
                taps_source: false,
            },
        ))
    }

    /// Whether an ability turns its own source into a creature.
    pub(super) fn effect_animates_source(effect: Option<EffectDef>) -> bool {
        match effect {
            Some(EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Animate(_),
                ..
            }) => true,
            Some(EffectDef::Sequence(effects)) => effects
                .iter()
                .any(|effect| Self::effect_animates_source(Some(*effect))),
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
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<GameObjectId> {
        self.plan_mana_activations_for(player, cost, x, avoid, purpose)
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

    pub(super) fn assigned_mana_activations_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
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
                    Self::mana_for_activation(*activation)
                        .first()
                        .is_some_and(|mana| self.mana_can_pay_for(*mana, purpose))
                })
                .collect::<Vec<_>>();
            // When several outputs are legal, prefer one whose spend rider
            // benefits this payment. Players can still manually choose a
            // different mana ability before casting.
            activations.sort_by_key(|activation| {
                let benefits_payment = Self::mana_for_activation(*activation)
                    .first()
                    .is_some_and(|mana| Self::mana_has_spend_effect_for(*mana, purpose));
                let pays_colored_symbol = mana_cost_amount(cost, activation.color) > 0
                    || hybrid_pays_with(cost, activation.color);
                (!benefits_payment, !pays_colored_symbol)
            });
            let outputs = activations
                .into_iter()
                .map(|activation| {
                    let benefits_payment = Self::mana_for_activation(activation)
                        .first()
                        .is_some_and(|mana| Self::mana_has_spend_effect_for(*mana, purpose));
                    (
                        activation.ability,
                        activation.color,
                        Self::mana_production(activation),
                        benefits_payment,
                    )
                })
                .collect::<Vec<_>>();
            match outputs.as_slice() {
                [] => {}
                [(ability, color, production, benefits_payment)] => {
                    pool.add(*production);
                    assigned.push(PlannedManaActivation {
                        source: permanent.card.id,
                        ability: *ability,
                        color: *color,
                        production: *production,
                        benefits_payment: *benefits_payment,
                        flexibility: 1,
                        order,
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

    pub(super) fn plan_mana_activations_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let mut available = self.assigned_mana_activations_for(player, cost, x, purpose)?;
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
                    .filter(|(_, activation)| activation.color == color)
                    .min_by_key(|(_, activation)| {
                        (
                            Some(activation.source) == avoid,
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
                    .filter(|(_, activation)| pair.contains(activation.color))
                    .min_by_key(|(_, activation)| {
                        (
                            Some(activation.source) == avoid,
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
                        Some(activation.source) == avoid,
                        !activation.benefits_payment,
                        activation.color != ManaColor::Colorless,
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

    /// How much generic mana this card's own static clauses take off its
    /// cost. Read from the hand, which is where casting reads it.
    /// "Players can't cast spells or play lands with ..." Read while play
    /// options are offered, so a prohibited card is simply not a legal action.
    /// The prohibition is a property of the card, not of who holds it, so it
    /// applies to both players.
    pub(super) fn play_is_prohibited(&self, card: &CardInstance, controller: PlayerId) -> bool {
        let prohibitions = self
            .battlefield
            .iter()
            .filter_map(|permanent| self.effective_rules(permanent))
            .flat_map(CardRules::ability_clauses)
            .filter(|ability| ability.is_executable())
            .filter_map(|ability| match ability.declarative_effect()? {
                EffectDef::PlayersCantPlay(predicate) => Some(predicate),
                _ => None,
            })
            .collect::<Vec<_>>();
        if prohibitions.is_empty() {
            return false;
        }
        let Some(object) = self.printed_trigger_event_object(
            card.id,
            card.definition,
            controller,
            &CharacteristicContext::Hand,
        ) else {
            return false;
        };
        prohibitions
            .into_iter()
            .any(|predicate| self.trigger_object_matches(*predicate, &object, card.id, false))
    }

    pub(super) fn spell_cost_reduction(
        &self,
        definition: CardDefinitionId,
        player: PlayerId,
    ) -> u16 {
        let Some(card) = self.catalog.get(definition) else {
            return 0;
        };
        card.rules
            .ability_clauses()
            .iter()
            .filter(|ability| ability.is_executable())
            .filter_map(|ability| match ability.declarative_effect()? {
                EffectDef::ReduceGenericCostBy(value) => Some(value),
                _ => None,
            })
            .map(|value| self.cost_reduction_value(value, player))
            .fold(0, u16::saturating_add)
    }

    /// The values a cost reduction can read. There is no resolving object
    /// while a cost is being worked out, so only board counts are available.
    pub(super) fn cost_reduction_value(&self, value: ValueDef, player: PlayerId) -> u16 {
        match value {
            ValueDef::Constant(amount) => u16::try_from(amount.max(0)).unwrap_or(u16::MAX),
            ValueDef::CountMatchingObjects(query) if query.zones == [ZoneKind::Battlefield] => {
                u16::try_from(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            self.player_relation_matches(
                                permanent.controller,
                                query.controller,
                                player,
                                TriggerContext::empty(),
                            ) && self.trigger_object_matches(
                                query.object,
                                &self.trigger_event_object(permanent),
                                permanent.card.id,
                                false,
                            )
                        })
                        .count(),
                )
                .unwrap_or(u16::MAX)
            }
            _ => 0,
        }
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
                            .map(Self::mana_production)
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
        let plan = self
            .plan_mana_activations_for(player, cost, x, avoid, purpose)
            .expect("a legal payment has a complete mana activation plan");
        for activation in plan {
            self.activate_mana_source(
                player,
                activation.source,
                activation.ability,
                activation.color,
            );
        }
    }
}

pub(super) fn can_pay(pool: ManaPool, cost: ManaCost, x: u16) -> bool {
    pool.white >= cost.white
        && pool.blue >= cost.blue
        && pool.black >= cost.black
        && pool.red >= cost.red
        && pool.green >= cost.green
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
    let Some(source) = sources.get(index) else {
        return can_pay(pool, cost, x);
    };
    for (ability, color, output, benefits_payment) in &source.outputs {
        let mut next = pool;
        next.add(*output);
        assignment.push(PlannedManaActivation {
            source: source.source,
            ability: *ability,
            color: *color,
            production: *output,
            benefits_payment: *benefits_payment,
            flexibility: source.outputs.len(),
            order: source.order,
        });
        if assign_flexible_mana_outputs(sources, index + 1, next, cost, x, assignment) {
            return true;
        }
        assignment.pop();
    }
    false
}

#[cfg(test)]
pub(super) fn pay_cost(pool: &mut ManaPool, cost: ManaCost, x: u16) {
    pay_cost_with_orders(
        pool,
        cost,
        x,
        // No rider to prefer, so each pair spends in its printed order.
        &|_| false,
        &[
            ManaColor::Colorless,
            ManaColor::Green,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::White,
            ManaColor::Blue,
        ],
    );
}

pub(super) fn pay_cost_with_orders(
    pool: &mut ManaPool,
    cost: ManaCost,
    x: u16,
    hybrid_preference: &impl Fn(ManaColor) -> bool,
    generic_order: &[ManaColor],
) {
    for color in colored_mana() {
        pool.remove_color(color, mana_cost_amount(cost, color));
    }
    for pair in HybridPair::ALL {
        let mut remaining = cost.hybrid[pair.index()];
        if remaining == 0 {
            continue;
        }
        let (first, second) = pair.colors();
        let mut order = [first, second];
        order.sort_by_key(|color| hybrid_preference(*color));
        for color in order {
            let spent = pool.amount(color).min(remaining);
            pool.remove_color(color, spent);
            remaining -= spent;
            if remaining == 0 {
                break;
            }
        }
        debug_assert_eq!(remaining, 0);
    }
    pay_generic_in_order(
        pool,
        cost.generic
            .saturating_add(x.saturating_mul(cost.x_multiplier)),
        generic_order,
    );
}

pub(super) fn add_generic(mut cost: ManaCost, additional: u16) -> ManaCost {
    cost.generic = cost.generic.saturating_add(additional);
    cost
}

/// A cost reduction only ever removes generic mana, and never takes a cost
/// below its colored requirements (CR 601.2f).
pub(super) fn reduce_generic(mut cost: ManaCost, reduction: u16) -> ManaCost {
    cost.generic = cost.generic.saturating_sub(reduction);
    cost
}

pub(super) fn add_mana_cost(mut cost: ManaCost, additional: ManaCost) -> ManaCost {
    cost.generic = cost.generic.saturating_add(additional.generic);
    cost.white = cost.white.saturating_add(additional.white);
    cost.blue = cost.blue.saturating_add(additional.blue);
    cost.black = cost.black.saturating_add(additional.black);
    cost.red = cost.red.saturating_add(additional.red);
    cost.green = cost.green.saturating_add(additional.green);
    for index in 0..HybridPair::COUNT {
        cost.hybrid[index] = cost.hybrid[index].saturating_add(additional.hybrid[index]);
    }
    cost.variable_x |= additional.variable_x;
    cost.x_multiplier = cost.x_multiplier.saturating_add(additional.x_multiplier);
    cost
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

pub(super) fn pay_generic_in_order(pool: &mut ManaPool, amount: u16, order: &[ManaColor]) {
    let mut remaining = amount;
    for color in order {
        let spent = pool.amount(*color).min(remaining);
        pool.remove_color(*color, spent);
        remaining -= spent;
        if remaining == 0 {
            break;
        }
    }
    debug_assert_eq!(remaining, 0);
}

pub(super) fn colored_mana() -> Vec<ManaColor> {
    vec![
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ]
}

/// "Spend only black mana on X." The restriction does not change how much the
/// spell costs, only which mana may pay for it, so folding the X portion out
/// of the generic requirement and into the coloured one says exactly that in
/// the vocabulary every payment path already speaks.
pub(super) fn fold_restricted_x(cost: ManaCost, x: u16, color: ManaColor) -> (ManaCost, u16) {
    let amount = x.saturating_mul(cost.x_multiplier);
    let mut folded = cost;
    match color {
        ManaColor::White => folded.white = folded.white.saturating_add(amount),
        ManaColor::Blue => folded.blue = folded.blue.saturating_add(amount),
        ManaColor::Black => folded.black = folded.black.saturating_add(amount),
        ManaColor::Red => folded.red = folded.red.saturating_add(amount),
        ManaColor::Green => folded.green = folded.green.saturating_add(amount),
        // No printed card restricts X to colourless, and generic already
        // accepts it, so there is nothing to fold.
        ManaColor::Colorless => return (cost, x),
    }
    (folded, 0)
}

pub(super) const fn mana_cost_amount(cost: ManaCost, color: ManaColor) -> u16 {
    match color {
        ManaColor::White => cost.white,
        ManaColor::Blue => cost.blue,
        ManaColor::Black => cost.black,
        ManaColor::Red => cost.red,
        ManaColor::Green => cost.green,
        ManaColor::Colorless => 0,
    }
}

pub(super) const fn colored_cost_total(cost: ManaCost) -> u16 {
    cost.white + cost.blue + cost.black + cost.red + cost.green + cost.hybrid_total()
}

pub(super) const fn mana_cost_value(cost: ManaCost) -> u16 {
    cost.generic.saturating_add(colored_cost_total(cost))
}

/// How much of a hybrid pair's colours is left once the cost's own coloured
/// symbols are covered.
pub(super) fn available_hybrid(pool: ManaPool, cost: ManaCost, pair: HybridPair) -> u16 {
    let (first, second) = pair.colors();
    let spare = |color: ManaColor| {
        pool.amount(color)
            .saturating_sub(mana_cost_amount(cost, color))
    };
    spare(first).saturating_add(spare(second))
}

/// Whether one colour can pay any hybrid symbol this cost carries.
pub(super) fn hybrid_pays_with(cost: ManaCost, color: ManaColor) -> bool {
    HybridPair::ALL
        .into_iter()
        .any(|pair| cost.hybrid[pair.index()] > 0 && pair.contains(color))
}
