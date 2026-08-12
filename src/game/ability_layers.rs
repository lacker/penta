use crate::card::AbilityPredicateDef;

use super::{
    AbilityDef, AbilityId, AbilityLayerOperation, AbilityLayerOperationKind, AbilityOrigin,
    AppliedEffectDef, BasicLandType, CardBehavior, CardType, ControlFlow, DeclarativeAbilityDef,
    EffectiveAbility, Game, KeywordAbility, Permanent, StaticAppliedEffect, abilities,
};

impl Game {
    /// Whether an object has a nonmana activated ability without recursively
    /// walking static effects. Static-effect recipient matching calls this
    /// helper, so folding static grants back into the query would recurse.
    /// Printed/copied abilities and already-resolved grants/removals are all
    /// represented here, which covers the current rules vocabulary.
    pub(super) fn has_nonmana_activated_ability(&self, permanent: &Permanent) -> bool {
        let mut abilities = self.collect_base_effective_abilities(permanent, None);
        for operation in Self::resolved_ability_layer_operations(permanent) {
            Self::apply_ability_layer_operation(&mut abilities, operation);
        }
        abilities.into_iter().any(|effective| {
            effective.ability.is_executable()
                && matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Activated(_)
                )
        })
    }

    /// Abilities the object currently has after the modeled layer-4 subtype
    /// setters and ordered layer-6 add/remove operations.
    pub(super) fn visit_effective_abilities(
        &self,
        permanent: &Permanent,
        mut visitor: impl FnMut(EffectiveAbility) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        for effective in self.collect_effective_abilities(permanent, None) {
            if visitor(effective).is_break() {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    fn collect_effective_abilities(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Vec<EffectiveAbility> {
        let mut abilities = self.collect_base_effective_abilities(permanent, prospective);
        for operation in self.collect_ability_layer_operations(permanent, prospective) {
            Self::apply_ability_layer_operation(&mut abilities, operation);
        }
        abilities
    }

    fn collect_base_effective_abilities(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Vec<EffectiveAbility> {
        let characteristics = prospective.unwrap_or(permanent);
        let rules_text_removed = prospective.map_or_else(
            || self.rules_text_abilities_removed(permanent),
            |prospective| {
                self.rules_text_abilities_removed_with_prospective(permanent, prospective)
            },
        );
        let animation_removes_abilities = permanent
            .animation
            .is_some_and(|animation| animation.loses_abilities);
        let mut abilities = Vec::new();
        if !rules_text_removed
            && !animation_removes_abilities
            && let Some(rules) = self.effective_rules(characteristics)
        {
            let (definition, part) = Self::effective_rules_source(characteristics);
            for attached in rules.indexed_abilities() {
                abilities.push(EffectiveAbility {
                    origin: AbilityOrigin::Printed {
                        definition,
                        part,
                        ability: attached.id,
                    },
                    ability: attached.definition,
                });
            }
            if let Some(copy) = &characteristics.copy_effect {
                for added in &copy.added_abilities {
                    abilities.push(EffectiveAbility {
                        origin: added.origin,
                        ability: added.definition,
                    });
                }
            }
        }

        let subtypes = prospective.map_or_else(
            || self.effective_subtypes(permanent),
            |prospective| self.effective_subtypes_with_prospective(permanent, prospective),
        );
        if self
            .permanent_types(characteristics)
            .is_some_and(|types| types.contains(CardType::Land))
        {
            let mut present = [false; BasicLandType::ALL.len()];
            for subtype in subtypes.iter() {
                let Some(land_type) = BasicLandType::from_subtype(subtype) else {
                    continue;
                };
                if !present[land_type.index()] {
                    present[land_type.index()] = true;
                    abilities.push(EffectiveAbility {
                        origin: AbilityOrigin::IntrinsicBasicLand(land_type),
                        ability: abilities::tap_for(land_type.mana_color()),
                    });
                }
            }
        }

        // A few legacy setup paths still write keyword markers directly. Seed
        // them before ordered operations so generalized removal affects them
        // just like an ordinary granted ability.
        let (definition, part) = Self::effective_rules_source(characteristics);
        for keyword in permanent.temporary_keywords.iter().copied().chain(
            permanent
                .keywords_until_upkeep_of
                .iter()
                .map(|(_, keyword)| *keyword),
        ) {
            abilities.push(EffectiveAbility {
                origin: AbilityOrigin::Printed {
                    definition,
                    part,
                    ability: AbilityId::PRIMARY,
                },
                ability: AbilityDef::keyword("Granted keyword ability", keyword),
            });
        }
        abilities
    }

    /// Builds the ordered layer-6 slice from resolved effects and static
    /// abilities that survive the modeled layer-4 setters. It deliberately
    /// does not feed layer-6 removals back into the set of static sources;
    /// dependencies where one static ability removes its own or another
    /// source's static ability still require the future fixed-point evaluator.
    fn collect_ability_layer_operations(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Vec<AbilityLayerOperation> {
        let mut operations = Self::resolved_ability_layer_operations(permanent);
        let mut visit_static = |applied: StaticAppliedEffect| {
            let order = u16::try_from(operations.len()).unwrap_or(u16::MAX);
            if let Some(operation) = Self::static_ability_layer_operation(applied, order) {
                operations.push(operation);
            }
            ControlFlow::Continue(())
        };
        let result = if let Some(prospective) = prospective {
            self.visit_static_applied_effects_with_prospective(
                permanent,
                prospective,
                &mut visit_static,
            )
        } else {
            self.visit_static_applied_effects(permanent, &mut visit_static)
        };
        debug_assert!(result.is_continue());

        operations.sort_by_key(|operation| (operation.timestamp, operation.order));
        operations
    }

    fn resolved_ability_layer_operations(permanent: &Permanent) -> Vec<AbilityLayerOperation> {
        let mut operations = Vec::new();
        for granted in &permanent.temporary_granted_abilities {
            operations.push(AbilityLayerOperation {
                timestamp: granted.timestamp,
                order: granted.order,
                kind: AbilityLayerOperationKind::Add {
                    origin: AbilityOrigin::Granted {
                        source: granted.source,
                        source_definition: granted.source_definition,
                        source_part: granted.source_part,
                        source_ability: granted.source_ability,
                        grant: granted.grant,
                    },
                    ability: granted.ability,
                },
            });
        }
        for removal in &permanent.temporary_removed_abilities {
            operations.push(AbilityLayerOperation {
                timestamp: removal.timestamp,
                order: removal.order,
                kind: AbilityLayerOperationKind::Remove(removal.predicate),
            });
        }
        operations.sort_by_key(|operation| (operation.timestamp, operation.order));
        operations
    }

    fn apply_ability_layer_operation(
        abilities: &mut Vec<EffectiveAbility>,
        operation: AbilityLayerOperation,
    ) {
        match operation.kind {
            AbilityLayerOperationKind::Add { origin, ability } => {
                abilities.push(EffectiveAbility {
                    origin,
                    ability: *ability,
                });
            }
            AbilityLayerOperationKind::Remove(predicate) => {
                abilities.retain(|ability| !Self::ability_predicate_matches(predicate, ability));
            }
        }
    }

    fn static_ability_layer_operation(
        applied: StaticAppliedEffect,
        order: u16,
    ) -> Option<AbilityLayerOperation> {
        let kind = match applied.effect {
            AppliedEffectDef::GrantAbility(ability) => AbilityLayerOperationKind::Add {
                origin: AbilityOrigin::Granted {
                    source: applied.source,
                    source_definition: applied.source_definition,
                    source_part: applied.source_part,
                    source_ability: applied.source_ability,
                    grant: applied
                        .grant
                        .expect("a granted ability has a structural grant identity"),
                },
                ability,
            },
            AppliedEffectDef::RemoveAbilities(predicate) => {
                AbilityLayerOperationKind::Remove(predicate)
            }
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::DoesNotUntapDuringUntapStep
            | AppliedEffectDef::CannotBeEnchanted
            | AppliedEffectDef::CannotBecomeEnchanted
            | AppliedEffectDef::CannotChangeController
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::PreventDamageFrom(_)
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::SetLandTypes(_)
            | AppliedEffectDef::Animate(_)
            | AppliedEffectDef::Composite(_)
            | AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::Special(_) => return None,
        };
        Some(AbilityLayerOperation {
            timestamp: applied.timestamp,
            order,
            kind,
        })
    }

    fn ability_predicate_matches(
        predicate: AbilityPredicateDef,
        ability: &EffectiveAbility,
    ) -> bool {
        match predicate {
            AbilityPredicateDef::Any => true,
            AbilityPredicateDef::Keyword(expected) => matches!(
                ability.ability.definition,
                DeclarativeAbilityDef::Keyword(actual) if actual == expected
            ),
        }
    }

    pub(super) fn visit_effective_replacement_abilities_with_prospective(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
        mut visitor: impl FnMut(EffectiveAbility) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        for effective in self.collect_effective_abilities(permanent, prospective) {
            if matches!(
                effective.ability.definition,
                DeclarativeAbilityDef::Replacement(_)
            ) && visitor(effective).is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn for_each_effective_ability(
        &self,
        permanent: &Permanent,
        mut visitor: impl FnMut(EffectiveAbility),
    ) {
        let result = self.visit_effective_abilities(permanent, |effective| {
            visitor(effective);
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
    }

    pub(super) fn find_effective_ability(
        &self,
        permanent: &Permanent,
        mut predicate: impl FnMut(EffectiveAbility) -> bool,
    ) -> Option<EffectiveAbility> {
        let mut found = None;
        let _ = self.visit_effective_abilities(permanent, |effective| {
            if predicate(effective) {
                found = Some(effective);
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        found
    }

    pub(super) fn effective_abilities(&self, permanent: &Permanent) -> Vec<EffectiveAbility> {
        let mut abilities = Vec::new();
        self.for_each_effective_ability(permanent, |effective| abilities.push(effective));
        abilities
    }

    /// The keywords an object presents without consulting ability-layer
    /// operations from static sources. Resolved additions and removals are
    /// safe to apply here because they cannot recurse through recipient
    /// matching; granted static abilities remain outside this boundary until
    /// fixed-point evaluation exists.
    pub(super) fn keyword_mask_ignoring_static_effects(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> u32 {
        let mut abilities = self.collect_base_effective_abilities(permanent, prospective);
        for operation in Self::resolved_ability_layer_operations(permanent) {
            Self::apply_ability_layer_operation(&mut abilities, operation);
        }
        let mut mask = 0;
        let mut set = |keyword: KeywordAbility| {
            if let Some(index) = keyword.simple_index() {
                mask |= 1 << index;
            }
        };
        for effective in abilities {
            if effective.ability.is_executable()
                && let DeclarativeAbilityDef::Keyword(keyword) = effective.ability.definition
            {
                set(keyword);
            }
        }
        mask
    }

    pub(super) fn effective_behavior(&self, permanent: &Permanent) -> Option<CardBehavior> {
        let mut abilities = self.collect_base_effective_abilities(permanent, None);
        for operation in Self::resolved_ability_layer_operations(permanent) {
            Self::apply_ability_layer_operation(&mut abilities, operation);
        }
        abilities
            .into_iter()
            .find_map(|effective| effective.ability.custom_behavior())
    }
}
