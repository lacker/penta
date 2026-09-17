impl Game {
    /// What this permanent's activated abilities actually cost in mana, with
    /// every increase and discount on the battlefield folded in.
    ///
    /// Increases go on first and discounts second, the way they do for a
    /// spell (CR 601.2f): a discount that ran first could take a cost to its
    /// floor and leave an increase to push it back up, which is not what
    /// either printed clause means.
    pub(super) fn ability_mana_cost(
        &self,
        permanent: &Permanent,
        cost: ManaCost,
        mana_ability: bool,
        definition: Option<&ActivatedAbilityDef>,
        targets: Option<&[TargetSelection]>,
    ) -> ManaCost {
        let mut total = cost;
        let mut discounts = Vec::new();
        for other in &self.battlefield {
            for effective in self.collect_effective_abilities(other, None) {
                let ability = effective.ability;
                match ability.declarative_effect() {
                    Some(EffectDef::ModifyCost(CostModificationDef::AbilityIncrease {
                        permanent: matcher,
                        amount,
                    })) if self.ability_cost_effect_applies(matcher, permanent, other) => {
                        total = add_mana_cost(total, amount);
                    }
                    Some(EffectDef::ModifyCost(CostModificationDef::SourceAbilityIncrease {
                        source: matcher,
                        amount,
                    })) if self.ability_cost_effect_applies(matcher, permanent, other) => {
                        total = add_mana_cost(total, amount);
                    }
                    Some(EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                        abilities,
                        permanent: matcher,
                        target,
                        amount,
                        minimum,
                    })) if Self::activation_kind_matches(abilities, mana_ability, definition)
                        && self.ability_cost_effect_applies(matcher, permanent, other)
                        && self.activation_target_condition_matches(
                            target,
                            targets,
                            other.card.id,
                        ) =>
                    {
                        let amount = self.cost_reduction_value_for(
                            amount,
                            other.controller,
                            other.card.id,
                            targets,
                        );
                        discounts.push((amount, minimum));
                    }
                    _ => {}
                }
            }
        }
        for (amount, minimum) in discounts {
            total = Self::reduce_ability_cost(total, amount, minimum);
        }
        total
    }

    pub(super) fn nonbattlefield_ability_mana_cost(
        &self,
        object: &crate::game::TriggerEventObject,
        cost: ManaCost,
        mana_ability: bool,
        definition: Option<&ActivatedAbilityDef>,
        targets: Option<&[TargetSelection]>,
    ) -> ManaCost {
        let mut total = cost;
        let mut discounts = Vec::new();
        for permanent in &self.battlefield {
            for effective in self.collect_effective_abilities(permanent, None) {
                let ability = effective.ability;
                let Some(effect) = ability.declarative_effect() else {
                    continue;
                };
                match effect {
                    EffectDef::ModifyCost(CostModificationDef::SourceAbilityIncrease {
                        source,
                        amount,
                    }) if self.trigger_object_matches(source, object, permanent.card.id, false) => {
                        total = add_mana_cost(total, amount);
                    }
                    // "Abilities you activate" reaches a card in a hand or a
                    // graveyard as readily as a permanent: what the clause
                    // names is the activation, and the card object answers
                    // the same predicate a permanent would.
                    EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                        abilities,
                        permanent: matcher,
                        target,
                        amount,
                        minimum,
                    }) if Self::activation_kind_matches(abilities, mana_ability, definition)
                        && self.trigger_object_matches(
                            matcher,
                            object,
                            permanent.card.id,
                            false,
                        )
                        && self.activation_target_condition_matches(
                            target,
                            targets,
                            permanent.card.id,
                        ) =>
                    {
                        let amount = self.cost_reduction_value_for(
                            amount,
                            permanent.controller,
                            permanent.card.id,
                            targets,
                        );
                        discounts.push((amount, minimum));
                    }
                    _ => {}
                }
            }
        }
        for (amount, minimum) in discounts {
            total = Self::reduce_ability_cost(total, amount, minimum);
        }
        total
    }

    pub(super) fn ability_mana_cost_for_source(
        &self,
        source: crate::ids::GameObjectId,
        cost: ManaCost,
        mana_ability: bool,
        definition: Option<&ActivatedAbilityDef>,
        targets: Option<&[TargetSelection]>,
    ) -> ManaCost {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return self.ability_mana_cost(permanent, cost, mana_ability, definition, targets);
        }
        let Some((zone, card)) = self.card_in_nonbattlefield_zone(source) else {
            return cost;
        };
        let context = match zone {
            crate::card::ZoneKind::Hand => crate::CharacteristicContext::Hand,
            crate::card::ZoneKind::Graveyard => crate::CharacteristicContext::Graveyard,
            crate::card::ZoneKind::Exile => crate::CharacteristicContext::Exile,
            crate::card::ZoneKind::Library => crate::CharacteristicContext::Library,
            crate::card::ZoneKind::Battlefield
            | crate::card::ZoneKind::Command
            | crate::card::ZoneKind::Stack => return cost,
        };
        self.printed_trigger_event_object(card.id, card.definition, card.owner, &context)
            .map_or(cost, |object| {
                self.nonbattlefield_ability_mana_cost(
                    &object,
                    cost,
                    mana_ability,
                    definition,
                    targets,
                )
            })
    }

    /// Price the entire declared mana payment before any activation costs
    /// change the board or player state.
    pub(super) fn priced_ability_mana_cost(
        &self,
        source: GameObjectId,
        definition: &ActivatedAbilityDef,
        targets: &[TargetSelection],
    ) -> ManaCost {
        let cost = crate::card::costs::mana_cost(definition.costs, None)
            .expect("offered nonbattlefield activations have fixed mana expressions");
        self.activation_mana_cost(definition, source, cost, targets)
    }

    /// What activating this ability costs in mana: the increases and
    /// discounts the battlefield supplies first, then the discount the
    /// ability prints about itself.
    ///
    /// The order matters for the same reason it does between increases and
    /// discounts (CR 601.2f): a printed floor is a floor on the finished
    /// cost, not on some intermediate one.
    pub(super) fn activation_mana_cost(
        &self,
        definition: &ActivatedAbilityDef,
        source: GameObjectId,
        cost: ManaCost,
        targets: &[TargetSelection],
    ) -> ManaCost {
        self.activation_mana_cost_with_targets(definition, source, cost, Some(targets))
    }

    /// An affordability lower bound before target selection. Exact pricing
    /// follows for each target choice; a color-count discount is at most five.
    pub(super) fn minimum_activation_mana_cost(
        &self,
        definition: &ActivatedAbilityDef,
        source: GameObjectId,
        cost: ManaCost,
    ) -> ManaCost {
        self.activation_mana_cost_with_targets(definition, source, cost, None)
    }

    fn activation_mana_cost_with_targets(
        &self,
        definition: &ActivatedAbilityDef,
        source: GameObjectId,
        cost: ManaCost,
        targets: Option<&[TargetSelection]>,
    ) -> ManaCost {
        self.activation_mana_cost_for_kind(definition, source, cost, false, targets)
    }

    pub(super) fn priced_mana_ability_cost(
        &self,
        source: GameObjectId,
        definition: &ActivatedAbilityDef,
    ) -> ManaCost {
        let cost = crate::card::costs::mana_cost(definition.costs, None)
            .expect("mana abilities have fixed costs");
        self.activation_mana_cost_for_kind(definition, source, cost, true, Some(&[]))
    }

    fn activation_target_condition_matches(
        &self,
        predicate: Option<crate::card::ObjectPredicateDef>,
        targets: Option<&[TargetSelection]>,
        source: GameObjectId,
    ) -> bool {
        let (Some(predicate), Some(targets)) = (predicate, targets) else {
            return true;
        };
        targets
            .iter()
            .flat_map(TargetSelection::targets)
            .any(|target| match target {
                Target::Permanent(id) => self
                    .battlefield
                    .iter()
                    .find(|p| p.card.id == *id)
                    .is_some_and(|p| {
                        self.trigger_object_matches(
                            predicate,
                            &self.targeting_event_object(p),
                            source,
                            false,
                        )
                    }),
                Target::Spell(id) => self
                    .stack
                    .iter()
                    .find(|s| s.id == *id)
                    .and_then(|s| self.stack_object_event_object(s))
                    .is_some_and(|o| self.trigger_object_matches(predicate, &o, source, true)),
                Target::Card(id) => {
                    self.card_in_nonbattlefield_zone(*id)
                        .is_some_and(|(zone, card)| {
                            self.card_object_matches(predicate, card, zone, source)
                        })
                }
                Target::Player(_) => false,
            })
    }

    pub(in crate::game) fn activation_kind_matches(
        kind: crate::card::AbilityKindDef,
        mana_ability: bool,
        definition: Option<&ActivatedAbilityDef>,
    ) -> bool {
        use crate::card::AbilityKindDef;
        definition.is_some_and(|definition| definition.keyword_kind == Some(kind))
            || matches!(kind, AbilityKindDef::Activated)
            || (mana_ability && kind == AbilityKindDef::ActivatedMana)
            || (!mana_ability && kind == AbilityKindDef::NonManaActivated)
    }

    fn activation_mana_cost_for_kind(
        &self,
        definition: &ActivatedAbilityDef,
        source: GameObjectId,
        cost: ManaCost,
        mana_ability: bool,
        targets: Option<&[TargetSelection]>,
    ) -> ManaCost {
        let cost = self.ability_mana_cost_for_source(
            source,
            cost,
            mana_ability,
            Some(definition),
            targets,
        );
        let Some(reduction) = definition.cost_reduction else {
            return cost;
        };
        let Some(player) = self.ability_cost_payer(source) else {
            return cost;
        };
        let amount = self.cost_reduction_value_for(reduction.amount, player, source, targets);
        Self::reduce_ability_cost(cost, amount, reduction.minimum)
    }

    /// Whose board a printed activation discount reads. A permanent's
    /// controller activates its abilities; a card anywhere else is activated
    /// by the player holding it, which for every printed channel cost is its
    /// owner.
    fn ability_cost_payer(&self, source: GameObjectId) -> Option<PlayerId> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return Some(permanent.controller);
        }
        self.card_in_nonbattlefield_zone(source)
            .map(|(_, card)| card.owner)
    }

    fn ability_cost_effect_applies(
        &self,
        matcher: crate::card::ObjectPredicateDef,
        permanent: &Permanent,
        source: &Permanent,
    ) -> bool {
        self.trigger_object_matches(
            matcher,
            &self.trigger_event_object(permanent),
            source.card.id,
            false,
        )
    }

    /// A discount touches generic mana only, and stops at the printed floor:
    /// "this effect can't reduce the mana in that cost to less than one
    /// mana" leaves an ability that already costs that little alone.
    fn reduce_ability_cost(cost: ManaCost, amount: u16, minimum: u16) -> ManaCost {
        // Only the generic portion can go, and only down to the floor. A
        // cost whose coloured symbols already meet the floor keeps all of
        // its generic anyway.
        let floor = minimum.max(cost.mana_value().saturating_sub(cost.generic));
        let room = cost.mana_value().saturating_sub(floor);
        reduce_generic(cost, amount.min(room))
    }

    /// The values a cost reduction can read. There is no resolving object
    /// while a cost is being worked out, but static zone queries can still
    /// use the card being cast as their source.
    /// A mana ability's amount, read off the permanent offering it. Only
    /// board-readable values belong here: the number has to be known before
    /// the ability is activated, not while it resolves.
    pub(super) fn mana_ability_value(&self, value: ValueDef, permanent: &Permanent) -> u16 {
        match value {
            ValueDef::CountersOnSource(kind) => permanent.counters(kind),
            // "Where X is this creature's power" is read as the ability is
            // offered, so a Vivi that has grown produces the larger amount
            // and a negative power produces nothing at all.
            ValueDef::SourcePower => self
                .power(permanent)
                .map_or(0, |power| u16::try_from(power.max(0)).unwrap_or(u16::MAX)),
            other => self.cost_reduction_value(other, permanent.controller, permanent.card.id),
        }
        // `cost_reduction_value` already answers constants and battlefield
        // counts; anything it does not know reads as zero, which is why the
        // boundary rule admits only the forms listed there.
    }

    pub(super) fn cost_reduction_value(
        &self,
        value: ValueDef,
        player: PlayerId,
        source: GameObjectId,
    ) -> u16 {
        self.cost_reduction_value_for(value, player, source, Some(&[]))
    }

    fn cost_reduction_value_for(
        &self,
        value: ValueDef,
        player: PlayerId,
        source: GameObjectId,
        targets: Option<&[TargetSelection]>,
    ) -> u16 {
        match value {
            ValueDef::ColorCount(ObjectRefDef::Target(index)) => targets.map_or(5, |targets| {
                targets
                    .iter()
                    .find(|selection| selection.slot().index() == index.index())
                    .and_then(|selection| selection.targets().first())
                    .and_then(|target| Self::target_object_id(*target))
                    .map_or(0, |object| self.object_color_count(object))
            }),
            ValueDef::ColorIntersectionCount(sets) => u16::from(
                Self::color_intersection(sets, |set| {
                    self.color_set_value(set, |reference| {
                        self.static_object_reference(reference, source)
                    })
                })
                .count(),
            ),
            ValueDef::ColorCount(reference) => self
                .static_object_reference(reference, source)
                .map_or(0, |object| self.object_color_count(object)),
            ValueDef::ManaInPool {
                player: relation,
                color,
            } => self.mana_in_pool_value(relation, color, player, None),
            ValueDef::Constant(amount) => u16::try_from(amount.max(0)).unwrap_or(u16::MAX),
            ValueDef::CountersOnSource(kind) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .map_or(0, |permanent| permanent.counters(kind)),
            ValueDef::CountMatchingObjects(query) => u16::try_from(
                self.objects_matching_query(*query, player, source, TriggerContext::empty())
                    .len(),
            )
            .unwrap_or(u16::MAX),
            ValueDef::IfMatchingObjectCount(condition) => {
                let count = self
                    .objects_matching_query(
                        condition.query,
                        player,
                        source,
                        TriggerContext::empty(),
                    )
                    .len();
                let chosen = if crate::game::effect_support::compare(
                    &count,
                    condition.comparison,
                    &usize::from(condition.amount),
                ) {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.cost_reduction_value_for(chosen, player, source, targets)
            }
            // Morbid, read while the spell is being paid for. The turn-scoped
            // flag is already maintained for resolution-time clauses, so
            // pricing asks the same question at a different moment.
            ValueDef::IfCreatureDiedThisTurn(branches) => {
                let chosen = if self.creature_died_this_turn {
                    branches.then
                } else {
                    branches.otherwise
                };
                self.cost_reduction_value_for(chosen, player, source, targets)
            }
            ValueDef::Sum(sum) => self
                .cost_reduction_value_for(sum.left, player, source, targets)
                .saturating_add(self.cost_reduction_value_for(sum.right, player, source, targets)),
            // Domain: how many basic land types are among the lands you
            // control, which is a count of types rather than of permanents
            // and so cannot be said as a query.
            ValueDef::BasicLandTypesControlled(_)
            | ValueDef::PermanentsSacrificedThisTurn(_)
            | ValueDef::CardsDiscardedThisTurn(_) => {
                u16::try_from(self.player_readable_value(value, player).max(0)).unwrap_or(u16::MAX)
            }
            _ => 0,
        }
    }
}
