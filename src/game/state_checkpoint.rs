use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde_json::Value;

use super::{
    AbilitySourceRef, ApplicableReplacement, AppliedStackEffect, BasicLandTypeChange, CardInstance,
    CharacteristicSource, CombatDamageStage, ContinuousEffectExpiration, ContinuousEffectTimestamp,
    CopiableAbility, CopiableCharacteristics, CounterKind, DamageSourceGroupDef,
    DoubleFacedCopiableCharacteristics, EffectResolutionContext, EntryCompletion,
    EnumeratedActions, ExilePlayCost, ExilePlayPermission, Game, GameEvent, GameObjectId,
    GameStack, InstalledTrigger, InstalledTriggerLifetime, Mana, ManaColor, ManaSource,
    NonbattlefieldAbilityGrant, ObjectBacking, ObjectInstance, ObjectKind, PendingBattlefieldEntry,
    PendingEvent, PendingReplacementEffect, Permanent, PlayerId, PlayerState, Pregame,
    RelationalSourceFilter, ReplaceableEvent, ReplacementEffectContext, ReplayRng,
    ResolvedAbilityOperation, ResolvedAttackRestriction, ResolvedContinuousEffect,
    ResolvedContinuousEffectKind, ResolvedDamagePrevention, ResolvedDamagePreventionCapacity,
    ResolvedDamagePreventionCoverage, ResolvedDamageRecipientMatcher, ResolvedDamageRedirect,
    ResolvedDamageSourceMatcher, ResolvedOngoingEffect, ResolvedPlayPermission,
    ResolvedPlayRestriction, ResolvedPlayerProtection, ResolvedPowerToughnessOperation,
    RetiredObject, ScopedEffect, StackAbilityPayload, StackAbilityResolver, StackObject,
    StackObjectKind, Step, TriggerCapture, TriggerContext, TurnPhaseResume, ZoneMoveCause,
    cast_source_zone_from_label,
};
use crate::card::ManaCost;
use crate::card::{
    AbilityOperationDef, AppliedEffectDef, BasicLandType, CardType, CardTypeSet,
    CharacteristicOperationDef, DeclarativeAbilityDef, DoubleFacedKind, PowerToughnessOperationDef,
    ReplacementEffectDef, ReplacementEventDef, SetOperationDef, SpellForm, TurnPhaseDef, ZoneKind,
};
use crate::casting::{CastChoices, CastSignature, CostConfiguration, TargetSelection};
use crate::{
    AbilityId, AbilityOrigin, AdditionalCostId, AlternativeCostId, AttackDefender, CardCatalog,
    CardDefinitionId, CardPartId, Format, GameObjectId as PublicGameObjectId, GrantId, ModeId,
    ObjectCharacteristics, PlayOptionId, Target, TargetSlotId,
};

mod counter;
mod decision;
mod emblem;
mod event;
mod exile_play;
mod model;
mod model_keyword;
mod model_ongoing;
mod model_prevention;
mod model_procedure;
mod model_trigger;
mod ongoing_effect;
mod permanent;
mod play_restriction;
mod prevention;
mod procedure;
mod semantics;
mod stack;
mod trigger;
mod wire;
mod wire_decision;
include!("state_checkpoint/compatibility.rs");

use counter::{player_counters, restore_visible_card_counters};
use decision::{
    decision_referenced_object_ids, decision_snapshot, mana_cost_from_snapshot, mana_cost_snapshot,
    parse_pending_decision, parse_pending_trigger, pending_trigger_snapshot,
};
use emblem::{emblem_snapshot, parse_emblems};
use event::{
    applicable_replacement_snapshot, catalog_entry_replacement_effect,
    parse_applicable_replacement, parse_replacement_context_snapshot,
    pending_event_referenced_object_ids, pending_event_snapshot,
};
use model::{
    AbilityActivationSnapshot, AbilityOriginSnapshot, AbilitySourceSnapshot,
    ApplicableReplacementSnapshot, AttackDefenderSnapshot, BasicLandTypeSnapshot,
    CombatDamageAssignmentSnapshot, CombatDamageStageSnapshot, ContinuousEffectExpirationSnapshot,
    CopiableAbilitySnapshot, CopiableCharacteristicsSnapshot, CopiedFromSnapshot,
    CounterKindSnapshot, CounterSnapshot, DetachedCardSnapshot, DetachedPermanentSnapshot,
    DoubleFacedCopiableCharacteristicsSnapshot, EntryCompletionSnapshot, GameSnapshot,
    ManaColorSnapshot, ManaSnapshot, ManaSourceSnapshot, NonbattlefieldAbilityGrantSnapshot,
    ObjectKindSnapshot, PendingBattlefieldEntrySnapshot, PendingEventSnapshot,
    PendingReplacementEffectSnapshot, PermanentSnapshot, PregameSnapshot,
    ReplacementEffectContextSnapshot, ReplacementEffectLocator, ResolvedContinuousEffectSnapshot,
    ResolvedContinuousOperationSnapshot, RetiredObjectSnapshot, SetOperationSnapshot,
    SuccessorSnapshot, TurnPhaseResumeSnapshot, TurnPhaseSnapshot, ZoneKindSnapshot,
};
use model_keyword::UpkeepKeywordSnapshot;
use ongoing_effect::{ongoing_effect_snapshot, parse_ongoing_effect};
use permanent::{detached_permanent_snapshot, permanent_snapshot};
use procedure::{
    draw_replacement_referenced_object_ids, draw_replacement_snapshot, parse_draw_replacement,
    parse_pending_procedure, pending_procedure_referenced_object_ids, pending_procedure_snapshot,
};
use semantics::{
    ability_locator, ability_locator_for_origin, ability_target_defs, catalog_ability,
    catalog_applied_effect, catalog_mana_payload, catalog_replacement_effect,
    catalog_token_characteristics, face_down_characteristics_from_snapshot,
    face_down_characteristics_snapshot, keyword_snapshot, mana_payload_locator,
    object_characteristics_from_snapshot, object_characteristics_snapshot, parse_keyword,
    replacement_effect_locator_matches_source, resolved_applied_effect_locator,
    resolved_replacement_effect_locator, token_characteristics_locator,
};
use stack::{
    current_stack_snapshot, detached_stack_snapshot_allowing, parse_detached_stack, parse_stack,
    parse_target as parse_snapshot_target, referenced_object_ids,
    resolution_context_referenced_object_ids, stack_object_has_unrebindable_hidden_reference,
    stack_source_origins, target_selections_referenced_object_ids, target_snapshot,
    trigger_capture_has_unrebindable_hidden_reference,
};
use trigger::{installed_trigger_snapshot, parse_installed_trigger};
#[allow(clippy::wildcard_imports)]
use wire::*;
use wire_decision::{rebind_stack_source_cards, rebind_visible_decision_cards};

impl Game {
    /// Hidden-safe rules bookkeeping needed to use an observation as a
    /// current-state checkpoint. Presentation fields stay in the ordinary
    /// observation; this object carries the state which cannot be inferred
    /// reliably from them.
    #[allow(clippy::too_many_lines)]
    fn snapshot(&self, viewer: PlayerId) -> GameSnapshot {
        let decision_state = (self.pending_decisions.len() == 1)
            .then(|| decision_snapshot(self, viewer, &self.pending_decisions[0]))
            .flatten();
        let has_unsupported_decision =
            !self.pending_decisions.is_empty() && decision_state.is_none();
        let visible_decision_rebindings = visible_decision_rebinding_ids(decision_state.as_ref());
        let visible_drawn_this_turn = [PlayerId::One, PlayerId::Two].map(|player| {
            if player == viewer {
                self.drawn_this_turn[player.index()]
                    .iter()
                    .map(|id| id.0)
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        });
        let mana = [PlayerId::One, PlayerId::Two].map(|player| {
            self.players[player.index()]
                .mana
                .iter()
                .copied()
                .map(|mana| mana_snapshot(&self.catalog, mana))
                .collect::<Vec<_>>()
        });
        let has_unlocated_mana = self.players.iter().any(|player| {
            player.mana.iter().any(|mana| {
                (!mana.restrictions.is_empty() || !mana.spend_effects.is_empty())
                    && mana_payload_locator(&self.catalog, *mana).is_none()
            })
        });
        let retired_ids = self
            .stack
            .iter()
            .filter(|object| !stack_object_has_unrebindable_hidden_reference(self, viewer, object))
            .flat_map(referenced_object_ids)
            .chain(
                self.battlefield
                    .iter()
                    .flat_map(|permanent| permanent.damage_sources.iter().copied()),
            )
            .chain(
                self.installed_triggers
                    .iter()
                    .filter(|trigger| {
                        !trigger_capture_has_unrebindable_hidden_reference(
                            self,
                            viewer,
                            &trigger.capture.targets,
                            &trigger.capture.context,
                        )
                    })
                    .flat_map(|trigger| {
                        [trigger.capture.source.object]
                            .into_iter()
                            .chain(target_selections_referenced_object_ids(
                                &trigger.capture.targets,
                            ))
                            .chain(resolution_context_referenced_object_ids(
                                &trigger.capture.context,
                            ))
                    }),
            )
            .chain(
                self.ongoing_effects
                    .iter()
                    .filter(|ongoing| {
                        !trigger_capture_has_unrebindable_hidden_reference(
                            self,
                            viewer,
                            &[],
                            &ongoing.context,
                        )
                    })
                    .flat_map(|ongoing| resolution_context_referenced_object_ids(&ongoing.context)),
            )
            .chain(
                self.pending_triggers
                    .iter()
                    .filter(|trigger| {
                        !trigger_capture_has_unrebindable_hidden_reference(
                            self,
                            viewer,
                            &trigger.targets,
                            &trigger.context,
                        )
                    })
                    .flat_map(|trigger| {
                        [trigger.source.object]
                            .into_iter()
                            .chain(target_selections_referenced_object_ids(&trigger.targets))
                            .chain(resolution_context_referenced_object_ids(&trigger.context))
                    }),
            )
            .chain(
                self.pending_events
                    .iter()
                    .flat_map(pending_event_referenced_object_ids),
            )
            .chain(
                self.pending_decisions
                    .iter()
                    .filter(|pending| {
                        decision_state.is_some()
                            && (pending.observation.visibility == crate::DecisionVisibility::Public
                                || pending.observation.player == viewer)
                    })
                    .flat_map(|pending| decision_referenced_object_ids(&pending.continuation)),
            )
            .chain(
                self.draw_replacements
                    .iter()
                    .flatten()
                    .filter(|replacement| {
                        draw_replacement_snapshot(self, viewer, replacement).is_some()
                    })
                    .flat_map(draw_replacement_referenced_object_ids),
            )
            .chain(
                self.pending_procedures
                    .iter()
                    .filter(|procedure| {
                        pending_procedure_snapshot(
                            self,
                            viewer,
                            procedure,
                            &visible_decision_rebindings,
                        )
                        .is_some()
                    })
                    .flat_map(pending_procedure_referenced_object_ids),
            )
            .chain(
                self.damage_preventions
                    .iter()
                    .copied()
                    .filter(|prevention| {
                        !prevention::damage_prevention_has_unrebindable_hidden_reference(
                            self,
                            viewer,
                            *prevention,
                        )
                    })
                    .flat_map(prevention::damage_prevention_referenced_object_ids),
            )
            .chain(
                self.resolved_play_restrictions
                    .iter()
                    .map(|restriction| restriction.source.object),
            )
            .chain(
                self.resolved_attack_restrictions
                    .iter()
                    .map(|restriction| restriction.source.object),
            )
            .chain(
                self.resolved_play_permissions
                    .iter()
                    .map(|permission| permission.source.object),
            )
            .chain(
                self.resolved_player_protections
                    .iter()
                    .map(|protection| protection.source.object),
            )
            .chain(self.spell_cast_history_this_turn.iter().copied())
            .chain(
                self.damage_redirects
                    .iter()
                    .copied()
                    .flat_map(prevention::damage_redirect_referenced_object_ids),
            )
            .filter(|id| self.retired_objects.contains_key(id))
            .collect::<BTreeSet<_>>();
        let retired_objects = retired_ids
            .iter()
            .copied()
            .filter_map(|id| match self.retired_objects.get(&id)? {
                RetiredObject::Permanent {
                    permanent,
                    power,
                    toughness,
                    mana_value,
                    keywords,
                } => Some(RetiredObjectSnapshot::Permanent {
                    permanent: Box::new(detached_permanent_snapshot(&self.catalog, permanent)),
                    power: *power,
                    toughness: *toughness,
                    mana_value: *mana_value,
                    keywords: keywords.iter().copied().map(keyword_snapshot).collect(),
                }),
                RetiredObject::Card(card) => Some(RetiredObjectSnapshot::Card {
                    card: DetachedCardSnapshot {
                        object_id: card.id.0,
                        definition: card.definition,
                        owner: card.owner.index(),
                    },
                }),
                RetiredObject::Stack(object) => Some(RetiredObjectSnapshot::Stack {
                    object: Box::new(detached_stack_snapshot_allowing(
                        self,
                        viewer,
                        object,
                        &visible_decision_rebindings,
                    )?),
                }),
            })
            .collect::<Vec<_>>();
        let has_unlocated_retired_object = retired_objects.len() != retired_ids.len();
        // Only for objects something might still name: the map itself grows
        // for the length of the game and most of it can never be asked for.
        let successors = retired_ids
            .iter()
            .filter_map(|id| {
                self.successors.get(id).map(|became| SuccessorSnapshot {
                    retired: id.0,
                    became: became.0,
                })
            })
            .collect::<Vec<_>>();
        let pending_events = self
            .pending_events
            .iter()
            .filter_map(|pending| pending_event_snapshot(&self.catalog, pending))
            .collect::<Vec<_>>();
        let has_unsupported_event = pending_events.len() != self.pending_events.len();
        let nonbattlefield_ability_grants = self
            .nonbattlefield_ability_grants
            .iter()
            .filter_map(|grant| {
                Some(NonbattlefieldAbilityGrantSnapshot {
                    object: grant.object.0,
                    ability: ability_locator(&self.catalog, |ability| *ability == grant.ability)?,
                    expiration: expiration_snapshot(grant.expiration),
                    source: grant.source.map(ability_origin_snapshot),
                })
            })
            .collect::<Vec<_>>();
        let has_unlocated_nonbattlefield_ability_grant =
            nonbattlefield_ability_grants.len() != self.nonbattlefield_ability_grants.len();
        let ongoing_effects = self
            .ongoing_effects
            .iter()
            .filter_map(|ongoing| ongoing_effect_snapshot(self, viewer, ongoing))
            .collect::<Vec<_>>();
        let has_unlocated_ongoing_effect = ongoing_effects.len() != self.ongoing_effects.len();
        let installed_triggers = self
            .installed_triggers
            .iter()
            .filter_map(|trigger| installed_trigger_snapshot(self, viewer, trigger))
            .collect::<Vec<_>>();
        let has_unlocated_installed_trigger =
            installed_triggers.len() != self.installed_triggers.len();
        let pending_triggers = self
            .pending_triggers
            .iter()
            .filter_map(|trigger| pending_trigger_snapshot(self, viewer, trigger))
            .collect::<Vec<_>>();
        let has_unlocated_pending_trigger = pending_triggers.len() != self.pending_triggers.len();
        let draw_replacements = [PlayerId::One, PlayerId::Two].map(|player| {
            self.draw_replacements[player.index()]
                .iter()
                .filter_map(|replacement| draw_replacement_snapshot(self, viewer, replacement))
                .collect::<Vec<_>>()
        });
        let has_unlocated_draw_replacement =
            [PlayerId::One, PlayerId::Two].into_iter().any(|player| {
                draw_replacements[player.index()].len()
                    != self.draw_replacements[player.index()].len()
            });
        let pending_procedures = self
            .pending_procedures
            .iter()
            .filter_map(|procedure| {
                pending_procedure_snapshot(self, viewer, procedure, &visible_decision_rebindings)
            })
            .collect::<Vec<_>>();
        let has_unlocated_pending_procedure =
            pending_procedures.len() != self.pending_procedures.len();
        let damage_preventions = self
            .damage_preventions
            .iter()
            .copied()
            .filter_map(|prevention| {
                prevention::damage_prevention_snapshot(self, viewer, prevention)
            })
            .collect::<Vec<_>>();
        let has_unlocated_damage_prevention =
            damage_preventions.len() != self.damage_preventions.len();
        let damage_redirects = self
            .damage_redirects
            .iter()
            .copied()
            .map(prevention::damage_redirect_snapshot)
            .collect();
        let resolved_play_restrictions = self
            .resolved_play_restrictions
            .iter()
            .copied()
            .filter_map(|restriction| {
                play_restriction::resolved_play_restriction_snapshot(&self.catalog, restriction)
            })
            .collect::<Vec<_>>();
        let has_unlocated_resolved_player_rule =
            resolved_play_restrictions.len() != self.resolved_play_restrictions.len();
        let resolved_attack_restrictions = self
            .resolved_attack_restrictions
            .iter()
            .filter_map(|restriction| {
                play_restriction::resolved_attack_restriction_snapshot(&self.catalog, restriction)
            })
            .collect::<Vec<_>>();
        let has_unlocated_resolved_player_rule = has_unlocated_resolved_player_rule
            || resolved_attack_restrictions.len() != self.resolved_attack_restrictions.len();
        let resolved_play_permissions = self
            .resolved_play_permissions
            .iter()
            .filter_map(|permission| {
                play_restriction::resolved_play_permission_snapshot(&self.catalog, permission)
            })
            .collect::<Vec<_>>();
        let resolved_player_protections = self
            .resolved_player_protections
            .iter()
            .filter_map(|protection| {
                play_restriction::resolved_player_protection_snapshot(&self.catalog, protection)
            })
            .collect::<Vec<_>>();
        let has_unlocated_resolved_player_rule = has_unlocated_resolved_player_rule
            || resolved_player_protections.len() != self.resolved_player_protections.len();
        let has_unlocated_resolved_player_rule = has_unlocated_resolved_player_rule
            || resolved_play_permissions.len() != self.resolved_play_permissions.len();
        // Phased-out permanents follow the battlefield in the observation,
        // so they follow it here too: the two lists are zipped by position.
        let battlefield = self
            .battlefield
            .iter()
            .chain(self.phased_out.iter())
            .map(|permanent| permanent_snapshot(&self.catalog, permanent))
            .collect::<Vec<_>>();
        let has_unlocated_battlefield_characteristics = battlefield
            .iter()
            .any(|permanent| permanent.has_dynamic_characteristics);
        let has_unlocated_pending_characteristics = pending_events
            .iter()
            .any(|pending| pending.entry.permanent.state.has_dynamic_characteristics);
        let has_unlocated_retired_characteristics = retired_objects.iter().any(|retired| {
            matches!(
                retired,
                RetiredObjectSnapshot::Permanent { permanent, .. }
                    if permanent.state.has_dynamic_characteristics
            )
        });
        let stack = self
            .stack
            .iter()
            .map(|object| current_stack_snapshot(self, viewer, object))
            .collect::<Vec<_>>();
        let has_unlocated_stack_state = stack.iter().any(|object| object.has_runtime_overrides);
        let emblems = self
            .emblems
            .iter()
            .filter_map(|emblem| emblem_snapshot(&self.catalog, emblem))
            .collect::<Vec<_>>();
        let has_unlocated_emblem = emblems.len() != self.emblems.len();
        GameSnapshot {
            version: crate::protocol::CHECKPOINT_VERSION,
            simulation_fingerprint: crate::protocol::SIMULATION_FINGERPRINT.to_owned(),
            turns_started: self.turns_started,
            damage_taken_this_turn: self.damage_taken_this_turn,
            attacked_subtypes_this_turn: [
                self.attacked_subtypes_this_turn[0]
                    .iter()
                    .map(|subtype| (*subtype).to_owned())
                    .collect(),
                self.attacked_subtypes_this_turn[1]
                    .iter()
                    .map(|subtype| (*subtype).to_owned())
                    .collect(),
            ],
            damage_taken_by_group_this_turn: self
                .damage_taken_by_group_this_turn
                .iter()
                .map(|groups| groups.to_vec())
                .collect(),
            next_decision_id: self.next_decision_id,
            next_trigger_id: self.next_trigger_id,
            next_continuous_effect_timestamp: self.next_continuous_effect_timestamp,
            consecutive_passes: self.consecutive_passes,
            attackers_declared: self.attackers_declared,
            blockers_declared: self.blockers_declared,
            untap_pending: self.untap_pending,
            cleanup_pending: self.cleanup_pending,
            mulligans: self.mulligans,
            lands_played_this_turn: [
                self.players[0].lands_played_this_turn,
                self.players[1].lands_played_this_turn,
            ],
            companions: [
                companion_definitions(&self.players[0].companions),
                companion_definitions(&self.players[1].companions),
            ],
            tried_to_draw_from_empty_library: [
                self.players[0].tried_to_draw_from_empty_library,
                self.players[1].tried_to_draw_from_empty_library,
            ],
            mana,
            creature_died_this_turn: self.creature_died_this_turn,
            creatures_died_this_turn: self.creatures_died_this_turn,
            linked_exiles: self
                .linked_exiles
                .iter()
                .map(|(source, card)| [source.0, card.0])
                .collect(),
            graveyard_permission_uses: self
                .graveyard_permission_uses
                .iter()
                .map(|(source, uses)| [source.0, u32::from(*uses)])
                .collect(),
            damage_cannot_be_prevented_this_turn: self.damage_cannot_be_prevented_this_turn,
            exile_play_permissions: self
                .exile_play_permissions
                .iter()
                .map(exile_play::permission_snapshot)
                .collect(),
            monarch: self.monarch.map(PlayerId::index),
            sorcery_flash_grants: self.sorcery_flash_grants,
            cannot_gain_life: self.cannot_gain_life,
            turn_phase_queue: self
                .turn_phase_queue
                .iter()
                .copied()
                .map(turn_phase_snapshot)
                .collect(),
            turn_phase_resume: self.turn_phase_resume.map(turn_phase_resume_snapshot),
            resolved_play_restrictions,
            resolved_attack_restrictions,
            resolved_play_permissions,
            resolved_player_protections,
            spells_cast_this_turn: self.spells_cast_this_turn,
            spells_cast_this_game: self.total_spells_cast,
            spells_cast_last_turn: self.spells_cast_last_turn,
            spell_cast_history_this_turn: object_ids_snapshot(&self.spell_cast_history_this_turn),
            cards_drawn_this_turn: self.cards_drawn_this_turn,
            citys_blessing: self.citys_blessing,
            permanent_left_battlefield_this_turn: self.permanent_left_battlefield_this_turn,
            card_left_graveyard_this_turn: self.card_left_graveyard_this_turn,
            life_gained_this_turn: self.life_gained_this_turn,
            lost_life_this_turn: self.lost_life_this_turn,
            draw_step_draw_taken: self.draw_step_draw_taken,
            drawn_this_turn: visible_drawn_this_turn,
            channel_active: [false; 2],
            defer_empty_library_loss: self.defer_empty_library_loss,
            draw_replacements,
            pending_combat_attackers: self
                .pending_combat_assignments
                .iter()
                .map(|id| id.0)
                .collect(),
            combat_blocked_attackers: self
                .combat_blocked_attackers
                .iter()
                .map(|id| id.0)
                .collect(),
            extra_turns: self
                .extra_turns
                .iter()
                .map(|player| player.index())
                .collect(),
            next_regular_player: self.next_regular_player.index(),
            damage_preventions,
            damage_redirects,
            pregame: self.pregame.map(|pregame| match pregame {
                Pregame::Mulligan(player) => PregameSnapshot::Mulligan {
                    seat: player.index(),
                },
                Pregame::Bottom(player) => PregameSnapshot::Bottom {
                    seat: player.index(),
                },
                Pregame::OpeningHand(player) => PregameSnapshot::OpeningHand {
                    seat: player.index(),
                },
            }),
            combat_damage_stage: match &self.combat_damage_stage {
                CombatDamageStage::NotStarted => CombatDamageStageSnapshot::NotStarted,
                CombatDamageStage::Single => CombatDamageStageSnapshot::Single,
                CombatDamageStage::FirstStrike {
                    strike_wave_combatants,
                } => CombatDamageStageSnapshot::FirstStrike {
                    combatants: strike_wave_combatants.iter().map(|id| id.0).collect(),
                },
                CombatDamageStage::RegularAfterFirstStrike {
                    strike_wave_combatants,
                } => CombatDamageStageSnapshot::RegularAfterFirstStrike {
                    combatants: strike_wave_combatants.iter().map(|id| id.0).collect(),
                },
            },
            battlefield,
            emblems,
            stack,
            retired_objects,
            successors,
            pending_events,
            nonbattlefield_ability_grants,
            ongoing_effects,
            next_installed_trigger_id: self.next_installed_trigger_id,
            installed_triggers,
            pending_triggers,
            pending_procedures,
            decision_state,
            has_deferred_state: has_unlocated_nonbattlefield_ability_grant
                || has_unlocated_ongoing_effect
                || has_unlocated_installed_trigger
                || has_unsupported_decision
                || has_unsupported_event
                || has_unlocated_pending_trigger
                || has_unlocated_retired_object
                || has_unlocated_battlefield_characteristics
                || has_unlocated_pending_characteristics
                || has_unlocated_retired_characteristics
                || has_unlocated_mana
                || has_unlocated_draw_replacement
                || has_unlocated_pending_procedure
                || has_unlocated_damage_prevention
                || has_unlocated_resolved_player_rule
                || has_unlocated_stack_state
                || has_unlocated_emblem,
            // Makes accidental reuse with another seat fail closed in the
            // importer without revealing anything about that other seat.
            viewer: viewer.index(),
        }
    }
}

/// The definitions a seat may still take as a companion, as the wire names
/// them. A definition id rather than an object id because the cards outside
/// the game are re-minted on restore and would not keep their identities.
/// The subtypes a seat attacked with this turn, matched back to the static
/// names the engine uses. A name no printing carries is dropped rather than
/// leaked into the game as a fresh static string.
fn restore_attacked_subtypes(recorded: &[String]) -> Vec<&'static str> {
    recorded
        .iter()
        .filter_map(|subtype| {
            crate::card::CREATURE_TYPES
                .iter()
                .find(|known| *known == subtype)
                .copied()
        })
        .collect()
}

fn companion_definitions(companions: &[CardDefinitionId]) -> Vec<u64> {
    companions
        .iter()
        .map(|definition| definition.get())
        .collect()
}

include!("state_checkpoint/restore.rs");

include!("state_checkpoint/support.rs");

#[cfg(test)]
mod tests;
