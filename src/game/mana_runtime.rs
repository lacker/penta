use super::{
    AbilityCostDef, AbilityDef, AbilityOrigin, AbilityProcedureDef, Action, ActivatedAbilityDef,
    AddManaEffectDef, AppliedStackEffect, CardType, CharacteristicContext, CommittedTriggerEvent,
    ConditionDef, DeclarativeAbilityDef, EffectDef, Game, GameObjectId, Mana,
    ManaAbilityActivation, ManaActivationChoices, ManaColor, ManaCost, ManaPaymentPurpose,
    ManaPool, ManaRestrictionDef, ManaSelectionDef, ManaSource, ManaSpendEffectDef, ManaTypeDef,
    ManaTypeFilterDef, ManaTypeSetDef, ManaTypeSourceDef, ObjectCountConditionDef, ObjectRefDef,
    ObjectSetDef, Permanent, PlayerId, RetiredObject, StackObject, TriggerContext,
    TriggerEventObject, ZoneKind, pay_cost_with_generic_strategy,
};
use crate::ManaPaymentChoice;
use crate::card::{AbilityCostList, ManaSplit};

mod color_spending;
mod eligibility;
mod pricing;
include!("mana_runtime/nonpermanent.rs");

impl Game {
    pub(super) fn mana_type_for_source(
        &self,
        mana: ManaTypeDef,
        source: GameObjectId,
    ) -> Option<ManaColor> {
        match mana {
            ManaTypeDef::Fixed(color) => Some(color),
            ManaTypeDef::ChosenColor => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .and_then(|permanent| permanent.chosen_color),
        }
    }

    pub(super) fn mana_ability_activations_uncached(
        &self,
        permanent: &Permanent,
    ) -> Vec<ManaAbilityActivation> {
        let mut activations = Vec::new();
        // Name-based activation prohibitions cover mana abilities too, and
        // mana abilities are enumerated separately from the rest.
        if self.mana_ability_activation_of_object_is_prohibited(
            permanent.controller,
            &self.trigger_event_object(permanent),
        ) {
            return activations;
        }
        self.for_each_effective_ability(permanent, |effective| {
            let ability = effective.ability;
            let DeclarativeAbilityDef::ActivatedMana(definition) = ability.definition else {
                return;
            };
            if self.activated_ability_is_prohibited(permanent, &ability) {
                return;
            }
            if !self.mana_ability_is_usable(permanent, &definition) {
                return;
            }
            // A mana ability is enumerated here rather than with the rest, so
            // the two restrictions a printed "activate only during your turn
            // and only once each turn" imposes have to be read here too.
            if !self.activation_timing_allows(permanent.controller, definition.timing) {
                return;
            }
            if definition.activation_limit.is_some_and(|limit| {
                permanent
                    .activations_this_turn
                    .iter()
                    .any(|(origin, count)| *origin == effective.origin && *count >= limit)
            }) {
                return;
            }
            // And so does "activate only if you control a Swamp or a
            // Forest". A false condition means there is no legal activation
            // at all, which for a mana ability means the land does not
            // produce that colour and nothing can be planned around it.
            if definition.condition.is_some_and(|condition| {
                !self.trigger_condition_holds(
                    condition,
                    permanent.card.id,
                    permanent.controller,
                    TriggerContext::empty(),
                    Some(effective.origin),
                    None,
                )
            }) {
                return;
            }
            activations.extend(self.mana_activations_for(
                permanent,
                effective.origin,
                &definition,
                &ability,
            ));
        });
        self.with_triggered_mana_choices(permanent, activations)
    }

    fn shared_add_mana_effect(
        definition: &ActivatedAbilityDef,
        ability: &AbilityDef,
    ) -> Option<AddManaEffectDef> {
        if definition.procedure != AbilityProcedureDef::Shared
            || ability.declarative_effect().is_none()
        {
            return None;
        }
        match ability.declarative_effect() {
            Some(EffectDef::AddMana(effect)) => Some(effect),
            // "Add {G} for each +1/+1 counter on this creature" is a fixed
            // amount at the moment it is offered; the caller resolves the
            // value against the permanent before the activation is built.
            Some(EffectDef::AddManaEqualTo { color, amount }) => {
                Some(AddManaEffectDef::one(color).with_variable_amount(amount))
            }
            _ => None,
        }
    }

    /// Whether a condition standing outside any resolving effect holds, read
    /// from the battlefield as it is now. `All` is the conjunction a card
    /// spells out when it names more than one permanent.
    pub(in crate::game) fn static_condition_holds(
        &self,
        condition: ConditionDef,
        controller: PlayerId,
        source: GameObjectId,
    ) -> bool {
        match condition {
            ConditionDef::Exists(query) => self.any_object_matches_query_with_prospective(
                query,
                controller,
                source,
                TriggerContext::empty(),
                None,
            ),
            ConditionDef::All(conditions) => conditions
                .iter()
                .all(|condition| self.static_condition_holds(*condition, controller, source)),
            ConditionDef::ObjectCount(counting) => {
                let ObjectCountConditionDef {
                    query,
                    comparison,
                    amount,
                } = *counting;
                let mut count = 0_usize;
                let _ = self.visit_objects_matching_query_with_prospective(
                    query,
                    controller,
                    source,
                    TriggerContext::empty(),
                    None,
                    None,
                    |_| {
                        count += 1;
                        std::ops::ControlFlow::Continue(())
                    },
                );
                crate::game::effect_support::compare(&count, comparison, &usize::from(amount))
            }
            ConditionDef::ControllerTurnsTakenAtMost(turns) => {
                self.turns_started[controller.index()] <= u32::from(turns)
            }
        }
    }

    /// How much mana this effect actually produces, with any "add ... instead"
    /// clause resolved against the board. Read once as the activation is
    /// built, so every later reader -- payment planning, the pool, the
    /// spend -- sees one consistent number.
    pub(in crate::game) fn mana_amount_for(
        &self,
        effect: AddManaEffectDef,
        controller: PlayerId,
        source: GameObjectId,
    ) -> u16 {
        effect
            .amount_override
            .filter(|override_| {
                self.static_condition_holds(override_.condition, controller, source)
            })
            .map_or(effect.amount, |override_| override_.amount)
    }

    /// The concrete activations one mana ability offers, which is one per
    /// colour it can produce.
    /// Which permanents a mana ability's "Sacrifice a <thing>" cost could
    /// take, one per activation. An ability with no such cost yields a single
    /// `None`, so the enumeration below runs once for it rather than not at
    /// all.
    fn mana_ability_cost_candidates(
        &self,
        permanent: &Permanent,
        definition: &ActivatedAbilityDef,
    ) -> Vec<Option<GameObjectId>> {
        let Some(cost) = definition.costs.iter().find(|cost| {
            matches!(
                cost,
                AbilityCostDef::SacrificePermanent { .. } | AbilityCostDef::ExileCardFromHand(_)
            )
        }) else {
            return vec![None];
        };
        match cost {
            AbilityCostDef::SacrificePermanent { object, controller } => self
                .battlefield
                .iter()
                .filter(|candidate| {
                    self.player_relation_matches(
                        candidate.controller,
                        *controller,
                        permanent.controller,
                        TriggerContext::empty(),
                    ) && self.trigger_object_matches(
                        *object,
                        &self.trigger_event_object(candidate),
                        permanent.card.id,
                        false,
                    )
                })
                .map(|candidate| Some(candidate.card.id))
                .collect(),
            AbilityCostDef::ExileCardFromHand(object) => self.players[permanent.controller.index()]
                .hand
                .iter()
                .filter(|card| {
                    self.card_object_matches(*object, card, ZoneKind::Hand, permanent.card.id)
                })
                .map(|card| Some(card.id))
                .collect(),
            _ => unreachable!("candidate cost was filtered above"),
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn mana_activations_for(
        &self,
        permanent: &Permanent,
        origin: AbilityOrigin,
        definition: &ActivatedAbilityDef,
        ability: &AbilityDef,
    ) -> Vec<ManaAbilityActivation> {
        let mut activations = Vec::new();
        if let Some(mut effect) = Self::shared_add_mana_effect(definition, ability) {
            // Resolved here rather than at payment time so that the amount
            // the planner counts on is the amount the pool receives.
            if let Some(value) = effect.variable_amount {
                effect.amount = self.mana_ability_value(value, permanent);
            }
            effect.amount = self.mana_amount_for(effect, permanent.controller, permanent.card.id);
            // "Remove any number of storage counters" is a choice of size, so
            // it becomes one activation per size rather than one activation
            // carrying an unanswered question. Each carries a sized removal,
            // and the amount it produces is the same number.
            let sizes = match Self::variable_counter_removal(definition) {
                Some(kind) => (1..=permanent.counters(kind))
                    .map(|removed| {
                        let costs = AbilityCostList::two(
                            AbilityCostDef::TapSource,
                            AbilityCostDef::RemoveCountersFromSource {
                                kind,
                                amount: removed,
                            },
                        );
                        (costs, removed, Some(removed))
                    })
                    .collect::<Vec<_>>(),
                None => vec![(definition.costs, effect.amount, None)],
            };
            // "Sacrifice a Goblin" is a choice of which one, and a mana
            // ability has no window in which to ask: like the counter sizes
            // above, each candidate becomes its own activation.
            let sacrifices = self.mana_ability_cost_candidates(permanent, definition);
            let mut add_activation =
                |color, costs, amount, counters_removed, cost_object, combination| {
                    activations.push(ManaAbilityActivation {
                        source: permanent.card.id,
                        ability: origin,
                        color,
                        costs,
                        only_as_instant: definition.only_as_instant,
                        effect: AddManaEffectDef { amount, ..effect },
                        counters_removed,
                        cost_object,
                        combination,
                        triggered_mana: None,
                    });
                };
            for (costs, amount, counters_removed) in sizes {
                for cost_object in &sacrifices {
                    match effect.mana {
                        ManaSelectionDef::One(kind) => {
                            let Some(color) = self.mana_type_for_source(kind, permanent.card.id)
                            else {
                                continue;
                            };
                            add_activation(
                                color,
                                costs,
                                amount,
                                counters_removed,
                                *cost_object,
                                None,
                            );
                        }
                        ManaSelectionDef::Choice(types) => {
                            let mut visiting = Vec::new();
                            for color in self.mana_types_for_set(permanent, types, &mut visiting) {
                                add_activation(
                                    color,
                                    costs,
                                    amount,
                                    counters_removed,
                                    *cost_object,
                                    None,
                                );
                            }
                        }
                        // Imprint: which colours this makes was settled when
                        // the card was exiled, so the list comes off the
                        // board rather than off the printed clause. A
                        // permanent that imprinted nothing offers nothing.
                        ManaSelectionDef::ColorsOfLinkedExiles => {
                            for color in self.linked_exile_colors(permanent.card.id) {
                                add_activation(
                                    color,
                                    costs,
                                    amount,
                                    counters_removed,
                                    *cost_object,
                                    None,
                                );
                            }
                        }
                        // "In any combination of" divides one amount across
                        // several types, so each division is its own
                        // activation. `color` names the first type the
                        // division actually produces, so that the planner
                        // still reads a colour off every activation.
                        ManaSelectionDef::Combination(types) => {
                            let mut visiting = Vec::new();
                            let colors = self.mana_types_for_set(permanent, types, &mut visiting);
                            for combination in Self::mana_combinations(&colors, amount) {
                                let Some((color, _)) = combination.iter().next() else {
                                    continue;
                                };
                                add_activation(
                                    color,
                                    costs,
                                    amount,
                                    counters_removed,
                                    *cost_object,
                                    Some(combination),
                                );
                            }
                        }
                    }
                }
            }
        }

        activations
    }

    /// Evaluate a declarative mana domain for one activated mana ability.
    /// Event-produced domains belong to immediate triggers and therefore
    /// intentionally have no answer here.
    fn mana_types_for_set(
        &self,
        permanent: &Permanent,
        types: ManaTypeSetDef,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        let computed = !matches!(types.source, ManaTypeSourceDef::Fixed(_));
        let mut colors = match types.source {
            ManaTypeSourceDef::Fixed(colors) => colors.to_vec(),
            ManaTypeSourceDef::ProducedBy(_) => Vec::new(),
            ManaTypeSourceDef::CouldBeProducedBy(objects) => self
                .mana_type_source_permanents(permanent, objects)
                .into_iter()
                .flat_map(|candidate| self.mana_types_permanent_could_produce(candidate, visiting))
                .collect(),
        };
        if types.filter == ManaTypeFilterDef::Colors {
            colors.retain(|color| *color != ManaColor::Colorless);
        }
        if computed {
            colors.sort_unstable();
            colors.dedup();
        }
        colors
    }

    fn mana_type_source_permanents<'a>(
        &'a self,
        source: &Permanent,
        objects: ObjectSetDef,
    ) -> Vec<&'a Permanent> {
        match objects {
            ObjectSetDef::One(crate::card::ObjectRefDef::Source) => self
                .battlefield
                .iter()
                .filter(|candidate| candidate.card.id == source.card.id)
                .collect(),
            ObjectSetDef::Query(query) => self
                .objects_matching_query(
                    query,
                    source.controller,
                    source.card.id,
                    TriggerContext::empty(),
                )
                .into_iter()
                .filter_map(|target| {
                    let crate::Target::Permanent(id) = target else {
                        return None;
                    };
                    self.battlefield
                        .iter()
                        .find(|candidate| candidate.card.id == id)
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    pub(super) fn mana_types_permanent_could_produce(
        &self,
        permanent: &Permanent,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        if visiting.contains(&permanent.card.id) {
            return Vec::new();
        }
        let mut colors = Vec::new();
        self.for_each_effective_ability(permanent, |effective| {
            let DeclarativeAbilityDef::ActivatedMana(definition) = effective.ability.definition
            else {
                return;
            };
            // CR 106.7 asks what the ability would produce if it resolved
            // now; whether its activation costs can presently be paid is
            // irrelevant. Spending restrictions and riders are likewise
            // properties of the resulting mana, not of its type.
            if let Some(effect) = Self::shared_add_mana_effect(&definition, &effective.ability) {
                match effect.mana {
                    ManaSelectionDef::One(kind) => {
                        colors.extend(self.mana_type_for_source(kind, permanent.card.id));
                    }
                    ManaSelectionDef::Choice(types) | ManaSelectionDef::Combination(types) => {
                        visiting.push(permanent.card.id);
                        colors.extend(self.mana_types_for_set(permanent, types, visiting));
                        visiting.pop();
                    }
                    ManaSelectionDef::ColorsOfLinkedExiles => {
                        colors.extend(self.linked_exile_colors(permanent.card.id));
                    }
                }
            }
        });
        colors.sort_unstable();
        colors.dedup();
        colors
    }

    pub(super) fn mana_ability_activation(
        &self,
        permanent: &Permanent,
        ability: AbilityOrigin,
        color: ManaColor,
        choices: &ManaActivationChoices,
    ) -> Option<ManaAbilityActivation> {
        self.mana_ability_activations(permanent)
            .into_iter()
            .find(|activation| {
                activation.ability == ability
                    && activation.color == color
                    // Source, ability, and colour name one storage land's
                    // ability three times over; the size is what tells them
                    // apart, and for a sacrifice cost the permanent does.
                    && activation.counters_removed == choices.counters_removed
                    && activation.cost_object == choices.cost_object
                    && activation.combination == choices.combination
                    && activation.triggered_mana == choices.triggered_mana
            })
    }

    /// Every way to divide `amount` mana across `colors`, in a stable order.
    /// A division that produces none of a type simply leaves it out, so "add
    /// two in any combination of {U} and/or {R}" enumerates three ways rather
    /// than naming both types every time.
    fn mana_combinations(colors: &[ManaColor], amount: u16) -> Vec<ManaSplit> {
        let mut divisions = vec![ManaSplit::empty()];
        for (index, color) in colors.iter().enumerate() {
            let last = index + 1 == colors.len();
            let mut next = Vec::new();
            for division in divisions {
                let remaining = amount - division.total();
                // The final type takes whatever is left: choosing for it too
                // would enumerate divisions that do not add up.
                let range = if last {
                    remaining..=remaining
                } else {
                    0..=remaining
                };
                for taken in range {
                    let mut division = division;
                    division.add(*color, taken);
                    next.push(division);
                }
            }
            divisions = next;
        }
        divisions.sort_unstable();
        divisions.dedup();
        divisions
    }

    pub(super) fn mana_production(activation: &ManaAbilityActivation) -> ManaPool {
        let mut pool = ManaPool::default();
        if let Some(combination) = activation.combination {
            for (color, amount) in combination.iter() {
                pool.add_color(color, amount);
            }
            if let Some(also) = activation.effect.also {
                pool.add_color(also, 1);
            }
            if let Some(triggered) = &activation.triggered_mana {
                for split in triggered {
                    for (color, amount) in split.iter() {
                        pool.add_color(color, amount);
                    }
                }
            }
            return pool;
        }
        pool.add_color(activation.color, activation.effect.amount);
        if let Some(also) = activation.effect.also {
            pool.add_color(also, 1);
        }
        if let Some(triggered) = &activation.triggered_mana {
            for split in triggered {
                for (color, amount) in split.iter() {
                    pool.add_color(color, amount);
                }
            }
        }
        pool
    }

    pub(super) fn mana_for_activation(activation: &ManaAbilityActivation) -> Vec<Mana> {
        let mana = Mana::from_ability(
            activation.color,
            ManaSource {
                object: activation.source,
                ability: activation.ability,
            },
            activation.effect.restrictions,
            activation.effect.spend_effects,
        );
        let mut produced = match activation.combination {
            Some(combination) => combination
                .iter()
                .flat_map(|(color, amount)| {
                    let mana = Mana::from_ability(
                        color,
                        ManaSource {
                            object: activation.source,
                            ability: activation.ability,
                        },
                        activation.effect.restrictions,
                        activation.effect.spend_effects,
                    );
                    vec![mana; usize::from(amount)]
                })
                .collect(),
            None => vec![mana; usize::from(activation.effect.amount)],
        };
        // The second colour carries the same source and riders: it is the
        // same activation, not a second one.
        if let Some(also) = activation.effect.also {
            produced.push(Mana::from_ability(
                also,
                ManaSource {
                    object: activation.source,
                    ability: activation.ability,
                },
                activation.effect.restrictions,
                activation.effect.spend_effects,
            ));
        }
        produced
    }

    pub(super) fn add_mana(&mut self, player: PlayerId, mana: impl IntoIterator<Item = Mana>) {
        for mana in mana {
            self.players[player.index()]
                .mana_pool
                .add_color(mana.color, 1);
            self.players[player.index()].mana.push(mana);
        }
    }

    pub(super) fn add_unrestricted_mana(
        &mut self,
        player: PlayerId,
        color: ManaColor,
        amount: u16,
    ) {
        self.add_mana(
            player,
            std::iter::repeat_n(Mana::unrestricted(color), usize::from(amount)),
        );
    }

    pub(super) fn eligible_mana_pool(
        &self,
        player: PlayerId,
        purpose: &ManaPaymentPurpose,
    ) -> ManaPool {
        let aggregate = self.players[player.index()].mana_pool;
        let mut eligible = ManaPool::default();
        let mut tracked = ManaPool::default();
        for mana in &self.players[player.index()].mana {
            if tracked.amount(mana.color) >= aggregate.amount(mana.color) {
                continue;
            }
            tracked.add_color(mana.color, 1);
            if self.mana_can_pay_for(*mana, purpose) {
                eligible.add_color(mana.color, 1);
            }
        }
        // Compatibility callers and tests may still write aggregate pools
        // directly. Any units without per-mana records are unrestricted.
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            eligible.add_color(
                color,
                aggregate
                    .amount(color)
                    .saturating_sub(tracked.amount(color)),
            );
        }
        eligible
    }

    pub(super) fn pay_player_cost_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<Mana> {
        let (cost, x) = self.restrict_x(cost, x, purpose);
        self.reconcile_mana(player);
        self.activate_repeatable_life_mana_for_shortfall(player, cost, x, purpose);
        let before = self.eligible_mana_pool(player, purpose);
        let mut after = before;
        let has_eligible_spend_effect = |color| {
            self.players[player.index()].mana.iter().any(|mana| {
                mana.color == color
                    && self.mana_can_pay_for(*mana, purpose)
                    && Self::mana_has_spend_effect_for(*mana, purpose)
            })
        };
        // A hybrid symbol prefers whichever of its colours carries a rider
        // this payment can use.
        let hybrid_preference = |color: ManaColor| u16::from(!has_eligible_spend_effect(color));
        let mut generic_order = [
            ManaColor::Colorless,
            ManaColor::Green,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::White,
            ManaColor::Blue,
        ];
        generic_order.sort_by_key(|color| !has_eligible_spend_effect(*color));
        let spread_generic_colors = self.payment_counts_colors_spent(purpose);
        if spread_generic_colors {
            // Converge counts colours, so the generic portion reaches first
            // for a colour the coloured symbols have not already spent, and
            // reaches for colourless last of all: it is a mana type rather
            // than a colour and adds nothing to the count.
            generic_order.sort_by_key(|color| {
                (
                    *color == ManaColor::Colorless,
                    super::mana_planning::mana_cost_amount(cost, *color) > 0,
                    !has_eligible_spend_effect(*color),
                )
            });
        }
        pay_cost_with_generic_strategy(
            &mut after,
            cost,
            x,
            &hybrid_preference,
            &generic_order,
            spread_generic_colors,
        );
        let mut spent = Vec::new();
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            let count = before.amount(color).saturating_sub(after.amount(color));
            for _ in 0..count {
                let index = self.players[player.index()]
                    .mana
                    .iter()
                    .enumerate()
                    .filter(|(_, mana)| {
                        mana.color == color && self.mana_can_pay_for(**mana, purpose)
                    })
                    .max_by_key(|(_, mana)| {
                        (
                            Self::mana_has_spend_effect_for(**mana, purpose),
                            !mana.restrictions.is_empty(),
                        )
                    })
                    .map(|(index, _)| index);
                if let Some(index) = index {
                    spent.push(self.players[player.index()].mana.remove(index));
                }
                self.players[player.index()]
                    .mana_pool
                    .remove_color(color, 1);
            }
        }
        spent
    }

    pub(super) fn pay_player_cost(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
    ) -> Vec<Mana> {
        self.pay_player_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }

    pub(super) fn apply_spent_mana_to_spell(&self, object: &mut StackObject, spent: &[Mana]) {
        for mana in spent {
            for spend_effect in mana.spend_effects {
                let effect = match *spend_effect {
                    ManaSpendEffectDef::ApplyToPaidSpell(effect) => Some(effect),
                    // "If that mana is spent on a creature spell": the rider
                    // asks what it paid for, and a spell that does not match
                    // simply carries nothing.
                    ManaSpendEffectDef::ApplyToPaidSpellMatching {
                        object: predicate,
                        effect,
                    } => self
                        .stack_trigger_event_object(object)
                        .is_some_and(|event| {
                            let source = mana.source.map_or(object.id, |source| source.object);
                            self.trigger_object_matches(predicate, &event, source, true)
                        })
                        .then_some(effect),
                    ManaSpendEffectDef::ApplyToPaidAbility(_) | ManaSpendEffectDef::Special(_) => {
                        None
                    }
                };
                if let Some(effect) = effect {
                    object.applied_effects.push(AppliedStackEffect {
                        source: mana.source,
                        granting: None,
                        effect,
                    });
                }
            }
        }
    }

    /// Tests and compatibility callers can still construct aggregate pools
    /// directly. Trim any now-impossible annotations before authoritative
    /// payment so that those writes cannot leave stale spend riders behind.
    pub(super) fn reconcile_mana(&mut self, player: PlayerId) {
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            let allowed = usize::from(self.players[player.index()].mana_pool.amount(color));
            let mut retained = 0;
            self.players[player.index()].mana.retain(|mana| {
                if mana.color != color {
                    true
                } else if retained < allowed {
                    retained += 1;
                    true
                } else {
                    false
                }
            });
        }
    }

    pub(super) fn can_pay_cost(&self, player: PlayerId, cost: ManaCost, x: u16) -> bool {
        self.can_pay_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }

    pub(super) fn can_pay_cost_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> bool {
        self.plan_mana_activations_for(player, cost, x, None, purpose)
            .is_some()
    }

    /// Every way this player may announce paying the flexible symbols in an
    /// activation cost that they can then actually pay: the mana left after
    /// the announcement has to be raisable, and the life the announcement
    /// itself spends has to be there to spend (CR 118.4).
    ///
    /// `None` is the announcement that names nothing, which is the only one
    /// a cost without a flexible symbol has.
    pub(super) fn affordable_activation_payments(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<Option<ManaPaymentChoice>> {
        Self::mana_payment_choices(cost)
            .into_iter()
            .filter_map(|choice| {
                // Activation costs, which no cast-time colour permission reaches.
                let (locked, life) = Self::locked_mana_payment(cost, &choice, false)?;
                let affordable = self.can_pay_life(player, life)
                    && self.can_pay_cost_for(player, locked, x, purpose);
                affordable.then(|| (!choice.alternatives().is_empty()).then_some(choice))
            })
            .collect()
    }

    pub(super) fn can_pay_cost_for_reserving_with_life(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
        reserved: &[GameObjectId],
        life_available: u16,
    ) -> bool {
        self.assigned_mana_activations_for_reserving_with_life(
            player,
            cost,
            x,
            purpose,
            reserved,
            life_available,
        )
        .is_some()
    }

    pub(super) fn add_mana_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
        {
            actions.extend(self.mana_ability_activations(permanent).into_iter().map(
                |activation| Action::ActivateManaAbility {
                    source: activation.source,
                    ability: activation.ability,
                    color: activation.color,
                    counters_removed: activation.counters_removed,
                    cost_object: activation.cost_object,
                    combination: activation.combination,
                    triggered_mana: activation.triggered_mana,
                },
            ));
        }
        actions.extend(
            self.hand_mana_ability_activations(player)
                .into_iter()
                .chain(self.ongoing_mana_ability_activations(player))
                .map(|activation| Action::ActivateManaAbility {
                    source: activation.source,
                    ability: activation.ability,
                    color: activation.color,
                    counters_removed: activation.counters_removed,
                    cost_object: activation.cost_object,
                    combination: activation.combination,
                    triggered_mana: activation.triggered_mana,
                }),
        );
    }
}

include!("mana_runtime/spend_questions.rs");
include!("mana_runtime/triggered.rs");
