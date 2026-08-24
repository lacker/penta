// What a spell's cost is reduced by, and what the reductions may read.
//
// Two shapes share this file because they answer the same question from
// opposite sides: a card in hand discounting itself, and a permanent on the
// battlefield discounting other spells. Included textually into
// `mana_planning.rs`, so the imports here are the parent module's.

impl Game {
    /// Alternative mana costs that battlefield statics offer for this spell.
    /// Equivalent costs are one choice even when several permanents offer
    /// them; choosing which Rooftop Storm supplied `{0}` has no rules meaning.
    pub(super) fn battlefield_spell_alternative_costs(
        &self,
        player: PlayerId,
        source: GameObjectId,
    ) -> Vec<ManaCost> {
        let Some((zone, card)) = self.card_in_nonbattlefield_zone(source) else {
            return Vec::new();
        };
        let mut costs = Vec::new();
        for permanent in &self.battlefield {
            let Some(rules) = self.effective_rules(permanent) else {
                continue;
            };
            for ability in rules.ability_clauses() {
                if !ability.is_executable() {
                    continue;
                }
                let Some(EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
                    spell,
                    caster,
                    zones,
                    cost,
                })) = ability.declarative_effect()
                else {
                    continue;
                };
                if !self.player_relation_matches(
                    player,
                    caster,
                    permanent.controller,
                    TriggerContext::empty(),
                ) || !zones.contains(&zone)
                    || !self.card_object_matches(spell, card, zone, permanent.card.id)
                    || costs.contains(&cost)
                {
                    continue;
                }
                costs.push(cost);
            }
        }
        costs
    }

    /// How much generic mana this card's own static clauses take off its
    /// cost. Read from the hand, which is where casting reads it.
    pub(super) fn spell_cost_reduction(
        &self,
        definition: CardDefinitionId,
        player: PlayerId,
        source: GameObjectId,
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
            .map(|value| self.cost_reduction_value(value, player, source))
            .fold(0, u16::saturating_add)
            .saturating_add(self.battlefield_spell_cost_reduction(player, source))
    }

    /// What permanents on the battlefield take off this spell's cost.
    ///
    /// Read from the card being cast rather than from its definition, so a
    /// continuous effect that changed its types is what the predicate sees.
    fn battlefield_spell_cost_reduction(&self, player: PlayerId, source: GameObjectId) -> u16 {
        let Some((zone, card)) = self.card_in_nonbattlefield_zone(source) else {
            return 0;
        };
        let mut reduction = 0_u16;
        for permanent in &self.battlefield {
            let Some(rules) = self.effective_rules(permanent) else {
                continue;
            };
            for ability in rules.ability_clauses() {
                if !ability.is_executable() {
                    continue;
                }
                let Some(EffectDef::ModifyCost(CostModificationDef::SpellReduction {
                    spell,
                    caster,
                    amount,
                })) = ability.declarative_effect()
                else {
                    continue;
                };
                if !self.player_relation_matches(
                    player,
                    caster,
                    permanent.controller,
                    TriggerContext::empty(),
                ) {
                    continue;
                }
                if !self.card_object_matches(spell, card, zone, permanent.card.id) {
                    continue;
                }
                reduction =
                    reduction.saturating_add(self.cost_reduction_value(amount, player, source));
            }
        }
        reduction
    }

    /// What permanents on the battlefield add to this spell's cost. Read the
    /// same way as the discount beside it, and kept separate from it because
    /// the two are not opposite numbers: a discount is generic-only, while an
    /// increase can name a colour.
    pub(super) fn spell_cost_increase(&self, player: PlayerId, source: GameObjectId) -> ManaCost {
        let Some((zone, card)) = self.card_in_nonbattlefield_zone(source) else {
            return ManaCost::default();
        };
        // A permission to play a card out of exile can carry its own tax,
        // which lives on the card rather than on any permanent: Elite
        // Spellbinder's is still owed long after the Spellbinder is dead.
        let mut increase = if zone == ZoneKind::Exile {
            self.exile_play_surcharge(source, player)
        } else {
            ManaCost::default()
        };
        for permanent in &self.battlefield {
            let Some(rules) = self.effective_rules(permanent) else {
                continue;
            };
            for ability in rules.ability_clauses() {
                if !ability.is_executable() {
                    continue;
                }
                let Some(EffectDef::ModifyCost(CostModificationDef::SpellIncrease {
                    spell,
                    caster,
                    amount,
                })) = ability.declarative_effect()
                else {
                    continue;
                };
                if !self.player_relation_matches(
                    player,
                    caster,
                    permanent.controller,
                    TriggerContext::empty(),
                ) {
                    continue;
                }
                if !self.card_object_matches(spell, card, zone, permanent.card.id) {
                    continue;
                }
                increase = add_mana_cost(increase, amount);
            }
        }
        increase
    }

    /// What this permanent's activated abilities actually cost in mana, with
    /// every increase and discount on the battlefield folded in.
    ///
    /// Increases go on first and discounts second, the way they do for a
    /// spell (CR 601.2f): a discount that ran first could take a cost to its
    /// floor and leave an increase to push it back up, which is not what
    /// either printed clause means.
    pub(super) fn ability_mana_cost(&self, permanent: &Permanent, cost: ManaCost) -> ManaCost {
        let mut total = cost;
        let mut discounts = Vec::new();
        for other in &self.battlefield {
            let Some(rules) = self.effective_rules(other) else {
                continue;
            };
            for ability in rules.ability_clauses() {
                if !ability.is_executable() {
                    continue;
                }
                match ability.declarative_effect() {
                    Some(EffectDef::ModifyCost(CostModificationDef::AbilityIncrease {
                        permanent: matcher,
                        amount,
                    })) if self.ability_cost_effect_applies(matcher, permanent, other) => {
                        total = add_mana_cost(total, amount);
                    }
                    Some(EffectDef::ModifyCost(
                        CostModificationDef::SourceAbilityIncrease {
                            source: matcher,
                            amount,
                        },
                    )) if self.ability_cost_effect_applies(matcher, permanent, other) => {
                        total = add_mana_cost(total, amount);
                    }
                    Some(EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                        permanent: matcher,
                        amount,
                        minimum,
                    })) if self.ability_cost_effect_applies(matcher, permanent, other) => {
                        let amount =
                            self.cost_reduction_value(amount, other.controller, other.card.id);
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
    ) -> ManaCost {
        let mut total = cost;
        for permanent in &self.battlefield {
            let Some(rules) = self.effective_rules(permanent) else {
                continue;
            };
            for ability in rules.ability_clauses() {
                let Some(EffectDef::ModifyCost(
                    CostModificationDef::SourceAbilityIncrease { source, amount },
                )) = ability
                    .is_executable()
                    .then(|| ability.declarative_effect())
                    .flatten()
                else {
                    continue;
                };
                if self.trigger_object_matches(source, object, permanent.card.id, false) {
                    total = add_mana_cost(total, amount);
                }
            }
        }
        total
    }

    pub(super) fn ability_mana_cost_for_source(
        &self,
        source: crate::ids::GameObjectId,
        cost: ManaCost,
    ) -> ManaCost {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return self.ability_mana_cost(permanent, cost);
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
        self.printed_trigger_event_object(
            card.id,
            card.definition,
            card.owner,
            &context,
        )
        .map_or(cost, |object| {
            self.nonbattlefield_ability_mana_cost(&object, cost)
        })
    }

    pub(super) fn priced_ability_mana_cost(
        &self,
        source: GameObjectId,
        costs: &[AbilityCostDef],
    ) -> Option<ManaCost> {
        costs.iter().find_map(|cost| match cost {
            AbilityCostDef::Mana(cost) => Some(self.ability_mana_cost_for_source(source, *cost)),
            _ => None,
        })
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
        match value {
            ValueDef::Constant(amount) => u16::try_from(amount.max(0)).unwrap_or(u16::MAX),
            ValueDef::CountMatchingObjects(query) => u16::try_from(
                self.objects_matching_query(*query, player, source, TriggerContext::empty())
                    .len(),
            )
            .unwrap_or(u16::MAX),
            // Domain: how many basic land types are among the lands you
            // control, which is a count of types rather than of permanents
            // and so cannot be said as a query.
            ValueDef::BasicLandTypesControlled(_) => {
                u16::try_from(self.player_readable_value(value, player).max(0)).unwrap_or(u16::MAX)
            }
            _ => 0,
        }
    }
}
