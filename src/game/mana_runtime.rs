use super::{
    AbilityCostDef, AbilityDef, AbilityOrigin, AbilityProcedureDef, Action, ActivatedAbilityDef,
    AddManaEffectDef, AppliedStackEffect, CardBehavior, CardType, CharacteristicContext,
    ConditionDef, DeclarativeAbilityDef, EffectDef, Game, GameObjectId, Mana,
    ManaAbilityActivation, ManaActivationChoices, ManaColor, ManaCost, ManaPaymentPurpose,
    ManaPool, ManaRestrictionDef, ManaSelectionDef, ManaSource, ManaSpendEffectDef,
    ObjectCountConditionDef, Permanent, PlayerId, RetiredObject, StackObject, TriggerContext,
    TriggerEventObject, ZoneKind, pay_cost_with_generic_strategy,
};
use crate::AbilityProgramDef;
use crate::card::{AbilityCostList, ManaSplit};

mod eligibility;
mod pricing;

impl Game {
    pub(super) fn mana_ability_activations(
        &self,
        permanent: &Permanent,
    ) -> Vec<ManaAbilityActivation> {
        let mut activations = Vec::new();
        // "Activated abilities can't be activated" covers mana abilities too,
        // and they are enumerated here rather than with the rest, so the
        // prohibition has to be read in both places.
        if self.activated_abilities_are_prohibited(permanent) {
            return activations;
        }
        self.for_each_effective_ability(permanent, |effective| {
            let ability = effective.ability;
            if !ability.is_executable() {
                return;
            }
            let DeclarativeAbilityDef::ActivatedMana(definition) = ability.definition else {
                return;
            };
            if !self.mana_ability_is_usable(permanent, definition) {
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
                definition,
                &ability,
            ));
        });
        activations
    }

    fn shared_add_mana_effect(
        definition: ActivatedAbilityDef,
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

    /// Which "one mana of any type a land could produce" ability this is, if
    /// either. Fellwar Stone reads the opponent's lands and Reflecting Pool
    /// reads its controller's own; both compute their colours from the board
    /// rather than declaring them.
    fn borrowed_mana_ability_behavior(
        definition: ActivatedAbilityDef,
        ability: &AbilityDef,
    ) -> Option<CardBehavior> {
        if definition.procedure != AbilityProcedureDef::Legacy
            || !matches!(
                ability.effect.definition,
                AbilityProgramDef::Effects(EffectDef::Special(_))
            )
        {
            return None;
        }
        match ability.custom_behavior() {
            Some(behavior @ (CardBehavior::FellwarStone | CardBehavior::ReflectingPool)) => {
                Some(behavior)
            }
            _ => None,
        }
    }

    /// The concrete activations one mana ability offers, which is one per
    /// colour it can produce.
    /// Which permanents a mana ability's "Sacrifice a <thing>" cost could
    /// take, one per activation. An ability with no such cost yields a single
    /// `None`, so the enumeration below runs once for it rather than not at
    /// all.
    fn mana_ability_sacrifice_candidates(
        &self,
        permanent: &Permanent,
        definition: ActivatedAbilityDef,
    ) -> Vec<Option<GameObjectId>> {
        let Some((object, controller)) = definition.costs.iter().find_map(|cost| match cost {
            AbilityCostDef::SacrificePermanent { object, controller } => {
                Some((*object, *controller))
            }
            _ => None,
        }) else {
            return vec![None];
        };
        self.battlefield
            .iter()
            .filter(|candidate| {
                self.player_relation_matches(
                    candidate.controller,
                    controller,
                    permanent.controller,
                    TriggerContext::empty(),
                ) && self.trigger_object_matches(
                    object,
                    &self.trigger_event_object(candidate),
                    permanent.card.id,
                    false,
                )
            })
            .map(|candidate| Some(candidate.card.id))
            .collect()
    }

    pub(super) fn mana_activations_for(
        &self,
        permanent: &Permanent,
        origin: AbilityOrigin,
        definition: ActivatedAbilityDef,
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
            let sacrifices = self.mana_ability_sacrifice_candidates(permanent, definition);
            let mut add_activation =
                |color, costs, amount, counters_removed, cost_object, combination| {
                    activations.push(ManaAbilityActivation {
                        source: permanent.card.id,
                        ability: origin,
                        color,
                        costs,
                        effect: AddManaEffectDef { amount, ..effect },
                        counters_removed,
                        cost_object,
                        combination,
                    });
                };
            for (costs, amount, counters_removed) in sizes {
                for cost_object in &sacrifices {
                    match effect.mana {
                        ManaSelectionDef::One(color) => {
                            add_activation(
                                color,
                                costs,
                                amount,
                                counters_removed,
                                *cost_object,
                                None,
                            );
                        }
                        ManaSelectionDef::Choice(colors) => {
                            for color in colors {
                                add_activation(
                                    *color,
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
                        ManaSelectionDef::Combination(colors) => {
                            for combination in Self::mana_combinations(colors, amount) {
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
        } else if let Some(behavior) = Self::borrowed_mana_ability_behavior(definition, ability) {
            activations.extend(self.borrowed_mana_activations(
                permanent,
                behavior,
                origin,
                definition.costs,
            ));
        }

        activations
    }

    /// One activation per colour Fellwar Stone can currently produce. The
    /// set is read off the battlefield, so it has to be recomputed rather
    /// than frozen on the ability.
    pub(super) fn borrowed_mana_activations(
        &self,
        permanent: &Permanent,
        behavior: CardBehavior,
        ability: AbilityOrigin,
        costs: crate::card::AbilityCostList,
    ) -> Vec<ManaAbilityActivation> {
        let mut visiting = Vec::new();
        self.borrowed_mana_colors(permanent, behavior, &mut visiting)
            .into_iter()
            .map(|color| ManaAbilityActivation {
                source: permanent.card.id,
                ability,
                color,
                costs,
                effect: AddManaEffectDef::one(color),
                counters_removed: None,
                cost_object: None,
                combination: None,
            })
            .collect()
    }

    /// Every mana one of these abilities could presently make. Fellwar Stone
    /// says "any color", so colourless is not one of the answers; Reflecting
    /// Pool says "any type", so it is.
    pub(super) fn borrowed_mana_colors(
        &self,
        permanent: &Permanent,
        behavior: CardBehavior,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        if visiting.contains(&permanent.card.id) {
            return Vec::new();
        }
        let own_lands = behavior == CardBehavior::ReflectingPool;
        let lender = if own_lands {
            permanent.controller
        } else {
            permanent.controller.opponent()
        };
        visiting.push(permanent.card.id);
        let mut colors = self
            .battlefield
            .iter()
            .filter(|candidate| {
                candidate.controller == lender
                    && self
                        .permanent_types(candidate)
                        .is_some_and(|types| types.contains(CardType::Land))
            })
            .flat_map(|candidate| self.colors_permanent_could_produce(candidate, visiting))
            .filter(|color| own_lands || *color != ManaColor::Colorless)
            .collect::<Vec<_>>();
        visiting.pop();
        colors.sort_unstable();
        colors.dedup();
        colors
    }

    pub(super) fn colors_permanent_could_produce(
        &self,
        permanent: &Permanent,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        if visiting.contains(&permanent.card.id) {
            return Vec::new();
        }
        let mut colors = Vec::new();
        self.for_each_effective_ability(permanent, |effective| {
            if !effective.ability.is_executable() {
                return;
            }
            let DeclarativeAbilityDef::ActivatedMana(definition) = effective.ability.definition
            else {
                return;
            };
            // "Could produce" is asked to decide whether a cost can be paid,
            // so an ability whose printed condition is false produces
            // nothing: planning around it would build a payment that cannot
            // actually be made.
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
            if let Some(effect) = Self::shared_add_mana_effect(definition, &effective.ability) {
                match effect.mana {
                    ManaSelectionDef::One(kind) => colors.push(kind),
                    ManaSelectionDef::Choice(kinds) | ManaSelectionDef::Combination(kinds) => {
                        colors.extend_from_slice(kinds);
                    }
                }
            } else if let Some(behavior) =
                Self::borrowed_mana_ability_behavior(definition, &effective.ability)
            {
                colors.extend(self.borrowed_mana_colors(permanent, behavior, visiting));
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
        choices: ManaActivationChoices,
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
            })
    }

    /// Every way to divide `amount` mana across `colors`, in a stable order.
    /// A division that produces none of a type simply leaves it out, so "add
    /// two in any combination of {U} and/or {R}" enumerates three ways rather
    /// than naming both types every time.
    fn mana_combinations(colors: &'static [ManaColor], amount: u16) -> Vec<ManaSplit> {
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
            return pool;
        }
        pool.add_color(activation.color, activation.effect.amount);
        if let Some(also) = activation.effect.also {
            pool.add_color(also, 1);
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

    pub(super) fn payment_object(
        &self,
        purpose: &ManaPaymentPurpose,
    ) -> Option<(TriggerEventObject, bool)> {
        match purpose {
            ManaPaymentPurpose::Spell {
                object,
                definition,
                controller,
                form,
            } => self
                .printed_trigger_event_object(
                    *object,
                    *definition,
                    *controller,
                    &CharacteristicContext::Stack { form: form.clone() },
                )
                .map(|object| (object, true)),
            ManaPaymentPurpose::Ability { source, .. } => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *source)
                .map(|permanent| (self.trigger_event_object(permanent), false))
                .or_else(|| match self.retired_objects.get(source) {
                    Some(RetiredObject::Permanent { permanent, .. }) => {
                        Some((self.trigger_event_object(permanent), false))
                    }
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
                })
                .or_else(|| {
                    let (zone, card) = self.card_in_nonbattlefield_zone(*source)?;
                    let context = match zone {
                        ZoneKind::Library => CharacteristicContext::Library,
                        ZoneKind::Hand => CharacteristicContext::Hand,
                        ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                        ZoneKind::Exile => CharacteristicContext::Exile,
                        ZoneKind::Command => CharacteristicContext::Command,
                        ZoneKind::Battlefield | ZoneKind::Stack => return None,
                    };
                    self.printed_trigger_event_object(
                        card.id,
                        card.definition,
                        card.owner,
                        &context,
                    )
                    .map(|object| (object, false))
                }),
            ManaPaymentPurpose::Other => None,
        }
    }

    pub(super) fn chosen_creature_type_for_mana_source(
        &self,
        source: GameObjectId,
    ) -> Option<&str> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| permanent.chosen_creature_type.as_deref())
            .or_else(|| match self.retired_objects.get(&source) {
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    permanent.chosen_creature_type.as_deref()
                }
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    pub(super) fn mana_can_pay_for(&self, mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.restrictions
            .iter()
            .all(|restriction| match restriction {
                ManaRestrictionDef::CastSpell(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, true)
                    }),
                ManaRestrictionDef::CastCreatureSpellOfChosenType => {
                    let Some(source) = mana.source else {
                        return false;
                    };
                    let Some(chosen) = self.chosen_creature_type_for_mana_source(source.object)
                    else {
                        return false;
                    };
                    self.payment_object(purpose)
                        .is_some_and(|(object, is_spell)| {
                            is_spell
                                && object.types.contains(CardType::Creature)
                                && object.subtypes.contains(&chosen)
                        })
                }
                ManaRestrictionDef::ActivateAbility(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        !is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, false)
                    }),
                ManaRestrictionDef::Special(_) => false,
            })
    }

    pub(super) fn mana_has_spend_effect_for(mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.spend_effects.iter().any(|effect| {
            matches!(
                (purpose, effect),
                (
                    ManaPaymentPurpose::Spell { .. },
                    ManaSpendEffectDef::ApplyToPaidSpell(_)
                ) | (
                    ManaPaymentPurpose::Ability { .. },
                    ManaSpendEffectDef::ApplyToPaidAbility(_)
                )
            )
        })
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

    /// Turns life into the {C} this payment is short, one point at a time,
    /// so the ordinary payment below finds a pool that can cover the cost.
    pub(super) fn channel_for_shortfall(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) {
        let available = self.channel_mana_available(player);
        if available == 0 {
            return;
        }
        let pool = self.eligible_mana_pool(player, purpose);
        let needed = Self::generic_shortfall(pool, cost, x).min(available);
        for _ in 0..needed {
            self.players[player.index()].life -= 1;
            self.add_unrestricted_mana(player, ManaColor::Colorless, 1);
        }
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
        self.channel_for_shortfall(player, cost, x, purpose);
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
        let hybrid_preference = |color: ManaColor| !has_eligible_spend_effect(color);
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

    /// Whether the spell being paid for reads how many colours paid for it.
    /// Only converge asks, and only converge changes how the generic portion
    /// is spread.
    fn payment_counts_colors_spent(&self, purpose: &ManaPaymentPurpose) -> bool {
        let ManaPaymentPurpose::Spell { definition, .. } = purpose else {
            return false;
        };
        self.catalog
            .get(*definition)
            .is_some_and(|card| card.rules.counts_colors_of_mana_spent())
    }

    pub(super) fn pay_player_cost(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
    ) -> Vec<Mana> {
        self.pay_player_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }

    pub(super) fn apply_spent_mana_to_spell(object: &mut StackObject, spent: &[Mana]) {
        for mana in spent {
            for spend_effect in mana.spend_effects {
                if let ManaSpendEffectDef::ApplyToPaidSpell(effect) = *spend_effect {
                    object.applied_effects.push(AppliedStackEffect {
                        source: mana.source,
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
        self.assigned_mana_activations_for(player, cost, x, purpose)
            .is_some()
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
                },
            ));
        }
    }
}
