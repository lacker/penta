use std::cell::Cell;

use crate::ids::GrantId;

use super::continuous_effects::StaticEffectKind;
use super::{
    AbilityDef, AbilityId, AbilityLayerOperation, AbilityLayerOperationKind, AbilityOperationDef,
    AbilityOrigin, AppliedEffectDef, BasicLandType, CardType, CharacteristicOperationDef,
    ControlFlow, DeclarativeAbilityDef, EffectiveAbility, Game, KeywordAbility, Permanent,
    ResolvedAbilityOperation, ResolvedContinuousEffectKind, StaticAppliedEffect, abilities,
};

thread_local! {
    /// Set while a layer-6 gathering pass is running on this thread.
    ///
    /// This is scratch state rather than game state: it is false at every
    /// point an action can observe a game, nothing that changes a game reads
    /// it, and keeping it per-thread rather than per-`Game` means two threads
    /// sharing one game still each get the same answer they would get alone.
    static STATIC_ABILITY_LAYER_PASS: Cell<bool> = const { Cell::new(false) };
}

/// Owns the layer-6 gathering pass for as long as it lives, and releases it
/// afterwards even if that pass unwinds.
struct StaticAbilityLayerGuard;

impl StaticAbilityLayerGuard {
    /// Claims the pass, or returns `None` when one is already running and the
    /// caller must therefore answer from the layer below.
    fn enter() -> Option<Self> {
        STATIC_ABILITY_LAYER_PASS.with(|pass| if pass.replace(true) { None } else { Some(Self) })
    }
}

impl Drop for StaticAbilityLayerGuard {
    fn drop(&mut self) {
        STATIC_ABILITY_LAYER_PASS.with(|pass| pass.set(false));
    }
}

impl Game {
    /// Whether one printed or copied ability remains after rules-text removal
    /// and already-resolved layer-6 operations. Static layer-6 dependencies
    /// still use the documented one-level model, but a resolved "loses all
    /// abilities" effect must immediately stop this ability from supplying a
    /// live static rule.
    pub(super) fn ability_survives_resolved_operations(
        &self,
        permanent: &Permanent,
        origin: AbilityOrigin,
    ) -> bool {
        let operations = self.resolved_ability_layer_operations(permanent);
        if operations.is_empty() {
            return true;
        }
        let mut abilities = self.collect_base_effective_abilities(permanent, None);
        for operation in operations {
            Self::apply_ability_layer_operation(&mut abilities, &operation);
        }
        abilities
            .into_iter()
            .any(|effective| effective.origin == origin)
    }

    /// Whether an object has a nonmana activated ability, reading printed and
    /// copied abilities plus already-resolved grants and removals. The one
    /// predicate that asks — Rising Waters' recipient query — is itself a
    /// static effect, so this deliberately stops where
    /// [`Self::collect_ability_layer_operations`] would stop for it anyway.
    /// Nothing else needs the question, so no full-walk variant exists yet.
    pub(super) fn has_nonmana_activated_ability(&self, permanent: &Permanent) -> bool {
        let mut abilities = self.collect_base_effective_abilities(permanent, None);
        for operation in self.resolved_ability_layer_operations(permanent) {
            Self::apply_ability_layer_operation(&mut abilities, &operation);
        }
        abilities.into_iter().any(|effective| {
            matches!(
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
            Self::apply_ability_layer_operation(&mut abilities, &operation);
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
        let mut abilities = Vec::new();
        if !rules_text_removed && let Some(rules) = self.effective_rules(characteristics) {
            let source = Self::effective_rules_source(characteristics);
            for attached in rules.indexed_abilities() {
                abilities.push(EffectiveAbility {
                    origin: Self::authored_ability_origin(source, attached.id),
                    ability: attached.definition,
                });
            }
            if let Some(copy) = characteristics.active_copy_values() {
                for added in &copy.added_abilities {
                    abilities.push(EffectiveAbility {
                        origin: added.origin,
                        ability: added.definition,
                    });
                }
            }
        }

        // A keyword counter is not a grant with a duration: the permanent
        // has the keyword exactly while the counter is sitting on it
        // (CR 122.1b), so it is read off the counters the way a basic land's
        // mana ability is read off its subtypes.
        for (kind, _) in characteristics.counters.iter() {
            if let Some(ability) = abilities::keyword_counter_ability(kind) {
                abilities.push(EffectiveAbility {
                    origin: AbilityOrigin::IntrinsicCounter(kind),
                    ability,
                });
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
        let source = Self::effective_rules_source(characteristics);
        for keyword in permanent.temporary_keywords.iter().copied().chain(
            permanent
                .keywords_until_upkeep_of
                .iter()
                .map(|(_, keyword)| *keyword),
        ) {
            abilities.push(EffectiveAbility {
                origin: Self::authored_ability_origin(source, AbilityId::PRIMARY),
                ability: AbilityDef::keyword("Granted keyword ability", keyword),
            });
        }
        if permanent.suspend_haste {
            abilities.push(EffectiveAbility {
                origin: Self::authored_ability_origin(source, AbilityId::PRIMARY),
                ability: AbilityDef::keyword("Suspend haste", KeywordAbility::Haste),
            });
        }
        abilities
    }

    /// Builds the ordered layer-6 slice from resolved effects and static
    /// abilities that survive the modeled layer-4 setters. It deliberately
    /// does not feed layer-6 removals back into the set of static sources;
    /// dependencies where one static ability removes its own or another
    /// source's static ability still require the future fixed-point evaluator.
    ///
    /// Gathering the static half is not re-entrant. A static source is matched
    /// against the characteristics of the permanent it might apply to, and one
    /// of those characteristics is the ability set this function produces, so
    /// the walk would otherwise call itself forever. Instead the first caller
    /// claims the pass and every query raised underneath it sees the printed,
    /// copied, and already-resolved abilities alone. That is the one-level
    /// stratification the future fixed-point evaluator replaces: it answers a
    /// static ability's own "which permanents do I apply to?" question from the
    /// layer below, which is right whenever no two static ability grants depend
    /// on each other, and is what CR 613.8 dependency ordering generalizes.
    fn collect_ability_layer_operations(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Vec<AbilityLayerOperation> {
        let mut operations = self.resolved_ability_layer_operations(permanent);
        let Some(_pass) = StaticAbilityLayerGuard::enter() else {
            return operations;
        };
        let mut visit_static = |applied: StaticAppliedEffect| {
            self.push_static_ability_layer_operations(&applied, &mut operations);
            ControlFlow::Continue(())
        };
        let result = if let Some(prospective) = prospective {
            self.visit_static_applied_effects_with_prospective(
                permanent,
                prospective,
                StaticEffectKind::Abilities,
                &mut visit_static,
            )
        } else {
            self.visit_static_applied_effects(
                permanent,
                StaticEffectKind::Abilities,
                &mut visit_static,
            )
        };
        debug_assert!(result.is_continue());

        operations.sort_by_key(|operation| (operation.timestamp, operation.order));
        operations
    }

    fn resolved_ability_layer_operations(
        &self,
        permanent: &Permanent,
    ) -> Vec<AbilityLayerOperation> {
        let mut operations = Vec::new();
        for effect in &permanent.resolved_continuous_effects {
            if !self.resolved_continuous_effect_is_active(effect) {
                continue;
            }
            let ResolvedContinuousEffectKind::Abilities(operation) = effect.kind else {
                continue;
            };
            let kind = match operation {
                ResolvedAbilityOperation::Add { ability, grant } => {
                    AbilityLayerOperationKind::Add {
                        origin: Self::granted_ability_origin(
                            effect.source.object,
                            effect.source.ability,
                            Self::effective_rules_source(permanent),
                            grant,
                        ),
                        ability,
                    }
                }
                ResolvedAbilityOperation::Remove(predicate) => {
                    AbilityLayerOperationKind::Remove(predicate)
                }
            };
            operations.push(AbilityLayerOperation {
                timestamp: effect.timestamp,
                order: effect.component_order,
                kind,
            });
        }
        operations.sort_by_key(|operation| (operation.timestamp, operation.order));
        operations
    }

    fn apply_ability_layer_operation(
        abilities: &mut Vec<EffectiveAbility>,
        operation: &AbilityLayerOperation,
    ) {
        match operation.kind {
            AbilityLayerOperationKind::Add { origin, ability } => {
                abilities.push(EffectiveAbility { origin, ability });
            }
            AbilityLayerOperationKind::Remove(predicate) => {
                abilities.retain(|ability| !predicate.matches(&ability.ability));
            }
        }
    }

    /// The layer-6 operations one static applied effect contributes. Every
    /// written-down operation contributes exactly one; a grant that reads the
    /// exile pile contributes one per ability it finds there, which is why
    /// this fills a list rather than returning a single operation.
    fn push_static_ability_layer_operations(
        &self,
        applied: &StaticAppliedEffect,
        operations: &mut Vec<AbilityLayerOperation>,
    ) {
        if matches!(
            applied.effect,
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(_),
            ))
        ) {
            self.push_linked_exile_ability_grants(applied, operations);
            return;
        }
        operations.extend(Self::static_ability_layer_operation(applied));
    }

    /// One Add operation for each activated ability of each matching card
    /// exiled with the granting object, in exile order. The grant identity is
    /// that position, which is stable because the pile only ever grows while
    /// the granting object is on the battlefield: an ability keeps the same
    /// identity from the moment it appears until the whole pile goes home.
    fn push_linked_exile_ability_grants(
        &self,
        applied: &StaticAppliedEffect,
        operations: &mut Vec<AbilityLayerOperation>,
    ) {
        let AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(predicate),
        )) = applied.effect
        else {
            return;
        };
        let mut position = 0_usize;
        for exiled in self.linked_exile_ids(applied.source) {
            let Some((zone, card)) = self.card_in_nonbattlefield_zone(exiled) else {
                continue;
            };
            if !self.card_object_matches(predicate, card, zone, applied.source) {
                continue;
            }
            let Some(rules) = self
                .catalog
                .get(card.definition)
                .map(|definition| &definition.rules)
            else {
                continue;
            };
            for attached in rules.indexed_abilities() {
                if !matches!(
                    attached.definition.definition,
                    DeclarativeAbilityDef::Activated(_) | DeclarativeAbilityDef::ActivatedMana(_)
                ) {
                    continue;
                }
                let Some(grant) = GrantId::from_index(position) else {
                    return;
                };
                position += 1;
                operations.push(AbilityLayerOperation {
                    timestamp: applied.timestamp,
                    order: applied.component_order,
                    kind: AbilityLayerOperationKind::Add {
                        origin: Self::granted_ability_origin(
                            applied.source,
                            applied.source_origin,
                            applied.source_presentation,
                            grant,
                        ),
                        ability: attached.definition,
                    },
                });
            }
        }
    }

    fn static_ability_layer_operation(
        applied: &StaticAppliedEffect,
    ) -> Option<AbilityLayerOperation> {
        let kind = match applied.effect {
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )) => AbilityLayerOperationKind::Add {
                origin: Self::granted_ability_origin(
                    applied.source,
                    applied.source_origin,
                    applied.source_presentation,
                    applied
                        .grant
                        .expect("a granted ability has a structural grant identity"),
                ),
                ability: *ability,
            },
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Remove(predicate),
            )) => AbilityLayerOperationKind::Remove(predicate),
            AppliedEffectDef::Characteristic(_)
            | AppliedEffectDef::Rule(_)
            | AppliedEffectDef::Composite(_) => return None,
        };
        Some(AbilityLayerOperation {
            timestamp: applied.timestamp,
            order: applied.component_order,
            kind,
        })
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

    /// The keywords an object presents, as a bitmask over
    /// [`KeywordAbility::simple_index`].
    ///
    /// This is the same ability set every other reader sees, so a predicate
    /// asking "target creature with islandwalk" and the blocking rules asking
    /// whether islandwalk applies agree. Asked from inside the layer-6 walk it
    /// degrades to printed, copied, and already-resolved keywords, which is
    /// what keeps `collect_ability_layer_operations` terminating.
    pub(super) fn keyword_mask(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> u64 {
        let abilities = self.collect_effective_abilities(permanent, prospective);
        let mut mask = 0;
        let mut set = |keyword: KeywordAbility| {
            if let Some(index) = keyword.simple_index() {
                mask |= 1 << index;
            }
        };
        for effective in abilities {
            if let DeclarativeAbilityDef::Keyword(keyword) = effective.ability.definition {
                set(keyword);
            }
        }
        mask
    }
}
