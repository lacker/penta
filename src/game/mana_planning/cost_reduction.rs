// What a spell's cost is reduced by, and what the reductions may read.
//
// Two shapes share this file because they answer the same question from
// opposite sides: a spell discounting itself on the stack, and a permanent on the
// battlefield discounting other spells. Included textually into
// `mana_planning.rs`, so the imports here are the parent module's.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct SpellCostReduction {
    generic: u16,
    symbols: ManaCost,
}

impl SpellCostReduction {
    pub(super) fn with_generic(mut self, amount: u16) -> Self {
        self.generic = self.generic.saturating_add(amount);
        self
    }

    pub(super) const fn generic(self) -> u16 {
        self.generic
    }
}

impl Game {
    /// Alternative mana costs that battlefield statics offer for this spell.
    /// Equivalent costs are one choice even when several permanents offer
    /// them; choosing which Rooftop Storm supplied `{0}` has no rules meaning.
    pub(super) fn battlefield_spell_alternative_costs(
        &self,
        view: super::SpellView<'_>,
    ) -> Vec<&'static [crate::CostDef]> {
        let player = view.controller;
        let Some(zone) = view.source_zone else {
            return Vec::new();
        };
        let Some(object) = self.spell_view_characteristics(view) else {
            return Vec::new();
        };
        let mut alternatives = Vec::new();
        for permanent in &self.battlefield {
            for effective in self.collect_effective_abilities(permanent, None) {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Static(static_definition) = ability.definition else {
                    continue;
                };
                if !static_definition
                    .source_zones
                    .contains(&ZoneKind::Battlefield)
                {
                    continue;
                }
                let Some(EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
                    spell,
                    caster,
                    zones,
                    costs,
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
                    || !self.trigger_object_matches(spell, &object, permanent.card.id, true)
                    || alternatives.contains(&costs)
                {
                    continue;
                }
                alternatives.push(costs);
            }
        }
        alternatives
    }

    /// Intrinsic reductions and battlefield modifiers read the same selected
    /// spell that cast-trigger capture will observe after payment.
    pub(super) fn spell_cost_reduction(
        &self,
        view: super::SpellView<'_>,
        targets: &[TargetSelection],
    ) -> SpellCostReduction {
        let mut reduction = self.battlefield_spell_cost_reduction(view, targets, true);
        let intrinsic = self.intrinsic_spell_cost_reduction(view, targets);
        reduction.generic = reduction.generic.saturating_add(intrinsic.generic);
        reduction.symbols = add_mana_cost(reduction.symbols, intrinsic.symbols);
        reduction
    }

    /// Enumeration must not exclude an X that makes a discount applicable.
    /// Ignore the spell predicate for this bound; each candidate is priced
    /// again with its actual characteristics before becoming a legal action.
    pub(super) fn spell_cost_reduction_ceiling(
        &self,
        view: super::SpellView<'_>,
    ) -> SpellCostReduction {
        let mut reduction = self.battlefield_spell_cost_reduction(view, &[], false);
        let intrinsic = self.intrinsic_spell_cost_reduction(view, &[]);
        reduction.generic = reduction.generic.saturating_add(intrinsic.generic);
        reduction.symbols = add_mana_cost(reduction.symbols, intrinsic.symbols);
        reduction
    }

    fn intrinsic_spell_cost_reduction(
        &self,
        view: super::SpellView<'_>,
        targets: &[TargetSelection],
    ) -> SpellCostReduction {
        let mut reduction = SpellCostReduction::default();
        self.visit_casting_spell_cost_adjustments(view, |adjustment| {
            if let CostAdjustmentDef::Subtract(amount) = adjustment {
                reduction = self.add_spell_cost_reduction(
                    reduction,
                    amount,
                    view.controller,
                    view.object,
                    targets,
                    view,
                );
            }
        });
        reduction
    }

    /// An object's own cost modifiers function on the stack (CR 113.6d).
    /// Read effective abilities from the same view used by cast triggers.
    fn visit_casting_spell_cost_adjustments(
        &self,
        view: super::SpellView<'_>,
        mut visit: impl FnMut(CostAdjustmentDef),
    ) {
        self.for_each_spell_view_ability(view, |effective| {
            let ability = effective.ability;
            let DeclarativeAbilityDef::Static(static_definition) = ability.definition else {
                return;
            };
            if !static_definition.source_zones.contains(&ZoneKind::Stack) {
                return;
            }
            let Some(EffectDef::ModifyCost(CostModificationDef::Spell(modification))) =
                ability.declarative_effect()
            else {
                return;
            };
            if modification.spell == crate::card::ObjectPredicateDef::Source
                && modification.condition == SpellCostConditionDef::Always
                && self.player_relation_matches(
                    view.controller,
                    modification.caster,
                    view.controller,
                    TriggerContext::empty(),
                )
            {
                visit(modification.adjustment);
            }
        });
    }

    fn battlefield_spell_cost_reduction(
        &self,
        view: super::SpellView<'_>,
        targets: &[TargetSelection],
        match_spell: bool,
    ) -> SpellCostReduction {
        let player = view.controller;
        let Some(object) = self.spell_view_characteristics(view) else {
            return SpellCostReduction::default();
        };
        let mut reduction = SpellCostReduction::default();
        for permanent in &self.battlefield {
            for effective in self.collect_effective_abilities(permanent, None) {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Static(static_definition) = ability.definition else {
                    continue;
                };
                if !static_definition
                    .source_zones
                    .contains(&ZoneKind::Battlefield)
                {
                    continue;
                }
                let Some(EffectDef::ModifyCost(modification)) = ability.declarative_effect() else {
                    continue;
                };
                let CostModificationDef::Spell(modification) = modification else {
                    continue;
                };
                let CostAdjustmentDef::Subtract(amount) = modification.adjustment else {
                    continue;
                };
                if !self.player_relation_matches(
                    player,
                    modification.caster,
                    permanent.controller,
                    TriggerContext::empty(),
                ) {
                    continue;
                }
                if (match_spell
                    && !self.trigger_object_matches(
                        modification.spell,
                        &object,
                        permanent.card.id,
                        true,
                    ))
                    || !spell_cost_condition_matches(
                        modification.condition,
                        view.source_zone.unwrap_or(ZoneKind::Stack),
                        view.owner,
                        player,
                        Target::Permanent(permanent.card.id),
                        targets,
                    )
                {
                    continue;
                }
                reduction = self.add_spell_cost_reduction(
                    reduction,
                    amount,
                    player,
                    permanent.card.id,
                    targets,
                    view,
                );
            }
        }
        reduction
    }

    /// What permanents on the battlefield add to this spell's cost. Read the
    /// same way as the discount beside it. Increases and reductions stay in
    /// separate calculation phases because the rules apply all increases
    /// before any reduction, even though both share one declarative shape.
    pub(super) fn spell_cost_increase(
        &self,
        view: super::SpellView<'_>,
        targets: &[TargetSelection],
    ) -> ManaCost {
        let player = view.controller;
        let Some(object) = self.spell_view_characteristics(view) else {
            return ManaCost::default();
        };
        let mut increase = self.spell_cost_increase_floor(view);
        self.visit_casting_spell_cost_adjustments(view, |adjustment| {
            if let CostAdjustmentDef::Add(amount) = adjustment {
                increase = self.add_spell_cost_amount(
                    increase,
                    amount,
                    player,
                    view.object,
                    targets,
                    view,
                );
            }
        });
        for permanent in &self.battlefield {
            for effective in self.collect_effective_abilities(permanent, None) {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Static(static_definition) = ability.definition else {
                    continue;
                };
                if !static_definition
                    .source_zones
                    .contains(&ZoneKind::Battlefield)
                {
                    continue;
                }
                let Some(EffectDef::ModifyCost(modification)) = ability.declarative_effect() else {
                    continue;
                };
                let CostModificationDef::Spell(modification) = modification else {
                    continue;
                };
                let CostAdjustmentDef::Add(amount) = modification.adjustment else {
                    continue;
                };
                if !self.player_relation_matches(
                    player,
                    modification.caster,
                    permanent.controller,
                    TriggerContext::empty(),
                ) {
                    continue;
                }
                if !self.trigger_object_matches(
                    modification.spell,
                    &object,
                    permanent.card.id,
                    true,
                ) || !spell_cost_condition_matches(
                    modification.condition,
                    view.source_zone.unwrap_or(ZoneKind::Stack),
                    view.owner,
                    player,
                    Target::Permanent(permanent.card.id),
                    targets,
                ) {
                    continue;
                }
                increase = self.add_spell_cost_amount(
                    increase,
                    amount,
                    player,
                    permanent.card.id,
                    targets,
                    view,
                );
            }
        }

        add_mana_cost(
            increase,
            self.stack_targeting_source_spell_increase(view, targets),
        )
    }

    /// Origin-based surcharges do not depend on chosen X. Other taxes may
    /// stop applying as X changes, so enumeration omits them from its floor.
    pub(super) fn spell_cost_increase_floor(&self, view: super::SpellView<'_>) -> ManaCost {
        if view.source_zone == Some(ZoneKind::Exile) {
            self.exile_play_surcharge(view.object, view.controller)
        } else if view.source_zone == Some(ZoneKind::Command) {
            ManaCost {
                generic: self.commander_tax(view.object),
                ..ManaCost::default()
            }
        } else {
            ManaCost::default()
        }
    }

    /// A static printed on a spell normally does nothing away from the
    /// battlefield. The targeting-source form is the deliberate exception:
    /// Kaervek's Torch is itself the stack object a counterspell names.
    fn stack_targeting_source_spell_increase(
        &self,
        view: super::SpellView<'_>,
        targets: &[TargetSelection],
    ) -> ManaCost {
        let player = view.controller;
        let Some(object) = self.spell_view_characteristics(view) else {
            return ManaCost::default();
        };
        let mut increase = ManaCost::default();
        for stack in self.stack.iter() {
            let Some(signature) = &stack.signature else {
                continue;
            };
            let Some(definition_id) = stack.presentation().card_definition() else {
                continue;
            };
            let Some(definition) = self.catalog.get(definition_id) else {
                continue;
            };
            let context = CharacteristicContext::Stack {
                form: signature.form().clone(),
            };
            let Ok(parts) = crate::card::applicable_part_ids_ref(definition, &context) else {
                continue;
            };
            for part in parts.iter().copied() {
                let Some(rules) = definition.part(part).map(|part| &part.rules) else {
                    continue;
                };
                for ability in rules.ability_clauses() {
                    let DeclarativeAbilityDef::Static(static_definition) = ability.definition
                    else {
                        continue;
                    };
                    if !static_definition.source_zones.contains(&ZoneKind::Stack) {
                        continue;
                    }
                    let Some(EffectDef::ModifyCost(CostModificationDef::Spell(modification))) =
                        ability.declarative_effect()
                    else {
                        continue;
                    };
                    let CostAdjustmentDef::Add(amount) = modification.adjustment else {
                        continue;
                    };
                    if self.player_relation_matches(
                        player,
                        modification.caster,
                        stack.controller,
                        TriggerContext::empty(),
                    ) && self.trigger_object_matches(modification.spell, &object, stack.id, true)
                        && spell_cost_condition_matches(
                            modification.condition,
                            view.source_zone.unwrap_or(ZoneKind::Stack),
                            view.owner,
                            player,
                            Target::Spell(stack.id),
                            targets,
                        )
                    {
                        increase = self.add_spell_cost_amount(
                            increase, amount, player, stack.id, targets, view,
                        );
                    }
                }
            }
        }
        increase
    }

    fn add_spell_cost_amount(
        &self,
        cost: ManaCost,
        amount: CostAmountDef,
        player: PlayerId,
        modifier_source: GameObjectId,
        targets: &[TargetSelection],
        casting: super::SpellView<'_>,
    ) -> ManaCost {
        match amount {
            CostAmountDef::Mana(amount) => add_mana_cost(cost, amount),
            CostAmountDef::Generic(value) => add_generic(
                cost,
                self.spell_cost_value(value, player, modifier_source, targets, casting),
            ),
        }
    }

    fn add_spell_cost_reduction(
        &self,
        mut reduction: SpellCostReduction,
        amount: CostAmountDef,
        player: PlayerId,
        modifier_source: GameObjectId,
        targets: &[TargetSelection],
        casting: super::SpellView<'_>,
    ) -> SpellCostReduction {
        match amount {
            CostAmountDef::Mana(mut amount) => {
                reduction.generic = reduction.generic.saturating_add(amount.generic);
                amount.generic = 0;
                reduction.symbols = add_mana_cost(reduction.symbols, amount);
            }
            CostAmountDef::Generic(value) => {
                reduction.generic = reduction.generic.saturating_add(self.spell_cost_value(
                    value,
                    player,
                    modifier_source,
                    targets,
                    casting,
                ));
            }
        }
        reduction
    }

    fn spell_cost_value(
        &self,
        value: ValueDef,
        player: PlayerId,
        modifier_source: GameObjectId,
        targets: &[TargetSelection],
        casting: super::SpellView<'_>,
    ) -> u16 {
        // CR 601.2a precedes total-cost determination: the announced card
        // is no longer in the hand, graveyard, exile, or library being counted.
        let count = |query| {
            self.objects_matching_query(query, player, modifier_source, TriggerContext::empty())
                .into_iter()
                .filter(|target| *target != Target::Card(casting.object))
                .count()
        };
        match value {
            ValueDef::ColorIntersectionCount(sets) => u16::from(
                Self::color_intersection(sets, |set| match set {
                    crate::card::ColorSetDef::OfObject(ObjectRefDef::ResolvingObject) => self
                        .spell_view_characteristics(casting)
                        .map_or(crate::card::ColorSet::empty(), |view| {
                            crate::card::ColorSet::from_flags(view.colors)
                        }),
                    other => self.color_set_value(other, |reference| {
                        self.static_object_reference(reference, modifier_source)
                    }),
                })
                .count(),
            ),
            ValueDef::AggregateObjectValues(aggregate) => {
                let crate::card::ObjectSetDef::Query(query) = aggregate.objects else {
                    return 0;
                };
                let objects = self
                    .objects_matching_query(query, player, modifier_source, TriggerContext::empty())
                    .into_iter()
                    .filter(|target| *target != Target::Card(casting.object))
                    .collect();
                u16::try_from(
                    self.aggregate_object_values(objects, aggregate.select, aggregate.operation)
                        .max(0),
                )
                .unwrap_or(u16::MAX)
            }
            ValueDef::CountMatchingObjects(query) => {
                u16::try_from(count(*query)).unwrap_or(u16::MAX)
            }
            ValueDef::IfMatchingObjectCount(condition) => {
                let branch = if crate::game::effect_support::compare(
                    &count(condition.query),
                    condition.comparison,
                    &usize::from(condition.amount),
                ) {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.spell_cost_value(branch, player, modifier_source, targets, casting)
            }
            ValueDef::IfCreatureDiedThisTurn(branches) => self.spell_cost_value(
                if self.creature_died_this_turn {
                    branches.then
                } else {
                    branches.otherwise
                },
                player,
                modifier_source,
                targets,
                casting,
            ),
            ValueDef::Sum(sum) => self
                .spell_cost_value(sum.left, player, modifier_source, targets, casting)
                .saturating_add(self.spell_cost_value(
                    sum.right,
                    player,
                    modifier_source,
                    targets,
                    casting,
                )),
            ValueDef::IfCondition(branches) => {
                let selected = if self.trigger_condition_holds(
                    branches.condition,
                    modifier_source,
                    player,
                    TriggerContext::empty(),
                    None,
                    None,
                ) {
                    branches.then
                } else {
                    branches.otherwise
                };
                self.spell_cost_value(selected, player, modifier_source, targets, casting)
            }
            ValueDef::DistinctTargets => distinct_target_count(targets),
            ValueDef::CountSpellsCastThisTurn(query) => u16::try_from(
                self.spells_cast_matching_this_turn(
                    *query,
                    player,
                    modifier_source,
                    TriggerContext::empty(),
                )
                .max(0),
            )
            .unwrap_or(u16::MAX),
            _ => self.cost_reduction_value(value, player, modifier_source),
        }
    }
}

include!("ability_costs.rs");

fn spell_cost_condition_matches(
    condition: SpellCostConditionDef,
    origin: ZoneKind,
    owner: PlayerId,
    caster: PlayerId,
    modifier_source: Target,
    targets: &[TargetSelection],
) -> bool {
    match condition {
        SpellCostConditionDef::Always => true,
        SpellCostConditionDef::CastFrom {
            zones,
            owner: relation,
        } => {
            zones.contains(&origin)
                && match relation {
                    PlayerRelation::Any => true,
                    PlayerRelation::You => owner == caster,
                    PlayerRelation::Opponent => owner != caster,
                    _ => false,
                }
        }
        SpellCostConditionDef::AnyOf(conditions) => conditions.iter().any(|condition| {
            spell_cost_condition_matches(
                *condition,
                origin,
                owner,
                caster,
                modifier_source,
                targets,
            )
        }),
        SpellCostConditionDef::TargetsSource => targets
            .iter()
            .flat_map(TargetSelection::targets)
            .any(|target| *target == modifier_source),
    }
}

fn distinct_target_count(targets: &[TargetSelection]) -> u16 {
    let mut targets = targets
        .iter()
        .flat_map(TargetSelection::targets)
        .copied()
        .collect::<Vec<_>>();
    targets.sort_unstable();
    targets.dedup();
    u16::try_from(targets.len()).unwrap_or(u16::MAX)
}
