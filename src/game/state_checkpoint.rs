use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde_json::Value;

use super::{
    AbilitySourceRef, ApplicableReplacement, AppliedStackEffect, BasicLandTypeChange, CardInstance,
    CharacteristicSource, CombatDamageStage, ContinuousEffectExpiration, ContinuousEffectTimestamp,
    CopiableAbility, CopiableCharacteristics, CounterKind, DamageSourceGroupDef,
    DoubleFacedCopiableCharacteristics, EffectResolutionContext, EntryCompletion, ExilePlayCost,
    ExilePlayPermission, Game, GameEvent, GameObjectId, GameStack, InstalledTrigger,
    InstalledTriggerLifetime, Mana, ManaSource, ObjectBacking, ObjectInstance, ObjectKind,
    PendingBattlefieldEntry, PendingEvent, PendingReplacementEffect, Permanent, PlayerId,
    PlayerState, Pregame, RelationalSourceFilter, ReplaceableEvent, ReplacementEffectContext,
    ReplayRng, ResolvedAbilityOperation, ResolvedContinuousEffect, ResolvedContinuousEffectKind,
    ResolvedDamagePrevention, ResolvedDamagePreventionCapacity, ResolvedDamagePreventionCoverage,
    ResolvedDamageRecipientMatcher, ResolvedDamageRedirect, ResolvedDamageSourceMatcher,
    ResolvedOngoingEffect, ResolvedPlayPermission, ResolvedPlayRestriction,
    ResolvedPowerToughnessOperation, RetiredObject, ScopedEffect, StackAbilityPayload,
    StackAbilityResolver, StackObject, StackObjectKind, Step, TemporaryAbilityGrant,
    TriggerCapture, TriggerContext, TurnPhaseResume, ZoneMoveCause, cast_source_zone_from_label,
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

mod decision;
mod emblem;
mod event;
mod model;
mod model_keyword;
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
    DetachedCardSnapshot, DetachedPermanentSnapshot, DoubleFacedCopiableCharacteristicsSnapshot,
    EntryCompletionSnapshot, ExilePlayPermissionSnapshot, GameSnapshot, ManaColorSnapshot,
    ManaSnapshot, ManaSourceSnapshot, ObjectKindSnapshot, PendingBattlefieldEntrySnapshot,
    PendingEventSnapshot, PendingReplacementEffectSnapshot, PermanentSnapshot, PregameSnapshot,
    ReplacementEffectContextSnapshot, ReplacementEffectLocator, ResolvedContinuousEffectSnapshot,
    ResolvedContinuousOperationSnapshot, RetiredObjectSnapshot, SetOperationSnapshot,
    SuccessorSnapshot, TemporaryAbilityGrantSnapshot, TurnPhaseResumeSnapshot, TurnPhaseSnapshot,
    ZoneKindSnapshot,
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
                self.resolved_play_permissions
                    .iter()
                    .map(|permission| permission.source.object),
            )
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
        let temporary_ability_grants = self
            .temporary_ability_grants
            .iter()
            .filter_map(|grant| {
                Some(TemporaryAbilityGrantSnapshot {
                    object: grant.object.0,
                    ability: ability_locator(&self.catalog, |ability| *ability == grant.ability)?,
                })
            })
            .collect::<Vec<_>>();
        let has_unlocated_temporary_ability_grant =
            temporary_ability_grants.len() != self.temporary_ability_grants.len();
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
        let has_unlocated_play_restriction =
            resolved_play_restrictions.len() != self.resolved_play_restrictions.len();
        let resolved_play_permissions = self
            .resolved_play_permissions
            .iter()
            .copied()
            .filter_map(|permission| {
                play_restriction::resolved_play_permission_snapshot(&self.catalog, permission)
            })
            .collect::<Vec<_>>();
        let has_unlocated_play_restriction = has_unlocated_play_restriction
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
            land_played_this_turn: [
                self.players[0].land_played_this_turn,
                self.players[1].land_played_this_turn,
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
            damage_cannot_be_prevented_this_turn: self.damage_cannot_be_prevented_this_turn,
            exile_play_permissions: self
                .exile_play_permissions
                .iter()
                .map(|permission| ExilePlayPermissionSnapshot {
                    card: permission.card.0,
                    player: permission.player.index(),
                    cost: permission.cost.label().to_owned(),
                    until_end_of_turn: permission
                        .until_end_of_turn
                        .map(|(player, turn)| (player.index(), turn)),
                    adventure_return_only: permission.adventure_return_only,
                    surcharge: (permission.surcharge != ManaCost::default())
                        .then(|| mana_cost_snapshot(permission.surcharge)),
                    not_before_turn: permission
                        .not_before_turn
                        .map(|(player, turn)| (player.index(), turn)),
                })
                .collect(),
            monarch: self.monarch.map(PlayerId::index),
            sorcery_flash_grants: self.sorcery_flash_grants,
            turn_phase_queue: self
                .turn_phase_queue
                .iter()
                .copied()
                .map(turn_phase_snapshot)
                .collect(),
            turn_phase_resume: self.turn_phase_resume.map(turn_phase_resume_snapshot),
            resolved_play_restrictions,
            resolved_play_permissions,
            spells_cast_this_turn: self.spells_cast_this_turn,
            spells_cast_last_turn: self.spells_cast_last_turn,
            cards_drawn_this_turn: self.cards_drawn_this_turn,
            citys_blessing: self.citys_blessing,
            permanent_left_battlefield_this_turn: self.permanent_left_battlefield_this_turn,
            card_left_graveyard_this_turn: self.card_left_graveyard_this_turn,
            life_gained_this_turn: self.life_gained_this_turn,
            draw_step_draw_taken: self.draw_step_draw_taken,
            drawn_this_turn: visible_drawn_this_turn,
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
            channel_active: self.channel_active,
            damage_preventions,
            damage_redirects,
            pregame: self.pregame.map(|pregame| match pregame {
                Pregame::Mulligan(player) => PregameSnapshot::Mulligan {
                    seat: player.index(),
                },
                Pregame::Bottom(player) => PregameSnapshot::Bottom {
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
            temporary_ability_grants,
            ongoing_effects,
            next_installed_trigger_id: self.next_installed_trigger_id,
            installed_triggers,
            pending_triggers,
            pending_procedures,
            decision_state,
            has_deferred_state: has_unlocated_temporary_ability_grant
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
                || has_unlocated_play_restriction
                || has_unlocated_stack_state
                || has_unlocated_emblem,
            // Makes accidental reuse with another seat fail closed in the
            // importer without revealing anything about that other seat.
            viewer: viewer.index(),
        }
    }

    /// Projection for the current checkpoint format. The checkpoint has one typed schema
    /// internally; only this boundary turns it into JSON.
    pub(super) fn checkpoint_json(&self, viewer: PlayerId) -> Value {
        serde_json::to_value(self.snapshot(viewer)).expect("GameSnapshot is serializable")
    }

    /// Rebuilds a decision-boundary state from its seat checkpoint and
    /// separately supplied hidden-zone hypothesis.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn from_observation_checkpoint(
        catalog: CardCatalog,
        format: Format,
        observation: &Value,
        hidden: &Value,
        rollout_seed: u64,
    ) -> Result<Self, String> {
        let checkpoint_value = field(observation, "checkpoint")?;
        let version = u32_field(checkpoint_value, "version")
            .map_err(|error| format!("invalid game snapshot: {error}"))?;
        if version != crate::protocol::CHECKPOINT_VERSION {
            return Err(format!(
                "checkpoint version {version} does not match {}",
                crate::protocol::CHECKPOINT_VERSION
            ));
        }
        let fingerprint = str_field(checkpoint_value, "simulationFingerprint")
            .map_err(|error| format!("invalid game snapshot: {error}"))?;
        if fingerprint != crate::protocol::SIMULATION_FINGERPRINT {
            return Err(format!(
                "checkpoint simulation fingerprint {fingerprint:?} does not match {}",
                crate::protocol::SIMULATION_FINGERPRINT
            ));
        }
        let checkpoint: GameSnapshot = serde_json::from_value(checkpoint_value.clone())
            .map_err(|error| format!("invalid game snapshot: {error}"))?;
        if checkpoint.has_deferred_state {
            return Err(
                "checkpoint contains executable rules state without stable catalog semantics"
                    .into(),
            );
        }
        let viewer = seat_value(field(observation, "seat")?)?;
        if checkpoint.viewer != viewer.index() {
            return Err("checkpoint viewer does not match observation seat".into());
        }

        let mut next_object_id = max_public_object_id(observation)
            .unwrap_or(0)
            .saturating_add(1);
        let own_hand = parse_cards(field(observation, "hand")?, viewer, &catalog)?;
        let opponent = viewer.opponent();
        let opponent_hand_defs = hidden_definitions(hidden, "hands", opponent)?;
        if opponent_hand_defs.len() != usize_field(observation, "opponentHandSize")? {
            return Err("hidden opponent hand does not match opponentHandSize".into());
        }
        let opponent_hand =
            mint_cards(&opponent_hand_defs, opponent, &catalog, &mut next_object_id)?;
        let libraries = [PlayerId::One, PlayerId::Two].map(|player| {
            hidden_definitions(hidden, "libraries", player).and_then(|definitions| {
                let expected = array(field(observation, "librarySizes")?)?
                    .get(player.index())
                    .and_then(Value::as_u64)
                    .and_then(|value| usize::try_from(value).ok())
                    .ok_or_else(|| "librarySizes must contain two counts".to_owned())?;
                if definitions.len() != expected {
                    return Err(format!(
                        "hidden {} library has {} cards, expected {expected}",
                        seat_label(player),
                        definitions.len()
                    ));
                }
                mint_cards(&definitions, player, &catalog, &mut next_object_id)
            })
        });
        let [library_one, library_two] = libraries;
        let library_one = library_one?;
        let library_two = library_two?;
        let outside_game = [PlayerId::One, PlayerId::Two].map(|player| {
            hidden_definitions(hidden, "outsideGame", player).and_then(|definitions| {
                mint_cards(&definitions, player, &catalog, &mut next_object_id)
            })
        });
        let [outside_one, outside_two] = outside_game;
        let mut outside_game = [outside_one?, outside_two?];

        let graveyards = parse_two_public_zones(field(observation, "graveyards")?, &catalog)?;
        let exiles = parse_two_public_zones(field(observation, "exiles")?, &catalog)?;
        let life = i16_pair(field(observation, "life")?)?;
        let poison = poison_pair(observation)?;
        let energy = energy_pair(observation)?;
        let mut checkpoint_hands = if viewer == PlayerId::One {
            [own_hand, opponent_hand]
        } else {
            [opponent_hand, own_hand]
        };
        let mut libraries = [library_one, library_two];
        // Before the decision's own rebinding: a stack source names a
        // position in the hypothesis, and the decision pass may reorder the
        // very zone it names.
        rebind_stack_source_cards(
            &stack_source_origins(&checkpoint.stack),
            &mut checkpoint_hands,
            &mut libraries,
            &mut outside_game,
        )?;
        rebind_visible_decision_cards(
            observation,
            checkpoint.decision_state.as_ref(),
            viewer,
            &mut checkpoint_hands,
            &mut libraries,
            &mut outside_game,
        )?;
        let land_played = checkpoint.land_played_this_turn;
        let tried_empty = checkpoint.tried_to_draw_from_empty_library;
        let mana_values = array(field(observation, "manaPools")?)?;
        if mana_values.len() != 2 {
            return Err("manaPools must contain p1 and p2 values".into());
        }
        let mana_pools = [
            parse_mana_pool(&mana_values[0])?,
            parse_mana_pool(&mana_values[1])?,
        ];
        let mana = [
            parse_mana(&checkpoint.mana[0], &catalog)?,
            parse_mana(&checkpoint.mana[1], &catalog)?,
        ];
        for player in [PlayerId::One, PlayerId::Two] {
            if mana_pool_from_units(&mana[player.index()]) != mana_pools[player.index()] {
                return Err(format!(
                    "checkpoint mana units do not match {} aggregate mana pool",
                    seat_label(player),
                ));
            }
        }
        let players = [PlayerId::One, PlayerId::Two].map(|player| PlayerState {
            life: life[player.index()],
            library: libraries[player.index()].clone(),
            tried_to_draw_from_empty_library: tried_empty[player.index()],
            hand: checkpoint_hands[player.index()].clone(),
            graveyard: graveyards[player.index()].clone(),
            exile: exiles[player.index()].clone(),
            outside_game: outside_game[player.index()].clone(),
            mana_pool: mana_pools[player.index()],
            mana: mana[player.index()].clone(),
            land_played_this_turn: land_played[player.index()],
            poison: poison[player.index()],
            energy: energy[player.index()],
        });

        let turns_started = checkpoint.turns_started;
        let temporary_ability_grants = checkpoint
            .temporary_ability_grants
            .iter()
            .map(|grant| {
                Ok(TemporaryAbilityGrant {
                    object: GameObjectId(grant.object),
                    ability: catalog_ability(&catalog, &grant.ability)
                        .ok_or("temporary ability grant locator is absent from this catalog")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let damage_preventions = checkpoint
            .damage_preventions
            .iter()
            .map(|prevention| prevention::parse_damage_prevention(&catalog, prevention))
            .collect::<Result<Vec<_>, _>>()?;
        let damage_redirects = checkpoint
            .damage_redirects
            .iter()
            .copied()
            .map(prevention::parse_damage_redirect)
            .collect::<Result<Vec<_>, _>>()?;
        let resolved_play_restrictions = checkpoint
            .resolved_play_restrictions
            .iter()
            .map(|restriction| {
                play_restriction::parse_resolved_play_restriction(&catalog, restriction)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let resolved_play_permissions = checkpoint
            .resolved_play_permissions
            .iter()
            .map(|permission| {
                play_restriction::parse_resolved_play_permission(&catalog, permission)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut game = Self {
            format,
            arrived: None,
            prospective_x: super::prospective_x::ProspectiveX::default(),
            successors: std::collections::HashMap::new(),
            damage_taken_this_turn: checkpoint.damage_taken_this_turn,
            damage_taken_by_group_this_turn: {
                let mut groups = [[0; DamageSourceGroupDef::COUNT]; 2];
                for (seat, stored) in checkpoint
                    .damage_taken_by_group_this_turn
                    .iter()
                    .enumerate()
                {
                    // A shorter historical vector is tolerated: groups are
                    // only ever appended.
                    if let Some(row) = groups.get_mut(seat) {
                        for (slot, value) in row.iter_mut().zip(stored) {
                            *slot = *value;
                        }
                    }
                }
                groups
            },
            seed: rollout_seed,
            rng: ReplayRng::new(rollout_seed),
            catalog,
            physical_cards: Vec::new(),
            players,
            battlefield: Vec::new(),
            phased_out: Vec::new(),
            stack: GameStack::default(),
            retired_objects: BTreeMap::new(),
            temporary_ability_grants,
            ongoing_effects: Vec::new(),
            next_object_id,
            next_continuous_effect_timestamp: checkpoint.next_continuous_effect_timestamp,
            turn: u32_field(observation, "turn")?,
            turns_started,
            active_player: seat_value(field(observation, "activeSeat")?)?,
            priority: seat_value(field(observation, "prioritySeat")?)?,
            consecutive_passes: checkpoint.consecutive_passes,
            step: parse_step(str_field(observation, "step")?)?,
            attackers_declared: checkpoint.attackers_declared,
            creature_died_this_turn: checkpoint.creature_died_this_turn,
            creatures_died_this_turn: checkpoint.creatures_died_this_turn,
            damage_cannot_be_prevented_this_turn: checkpoint.damage_cannot_be_prevented_this_turn,
            // Never live across a checkpoint: it is read and consumed
            // inside one activation, which cannot be interrupted.
            ninjutsu_returned_defender: None,
            exile_play_permissions: checkpoint
                .exile_play_permissions
                .iter()
                .map(|permission| {
                    Ok(ExilePlayPermission {
                        card: GameObjectId(permission.card),
                        player: player_from_index(permission.player)?,
                        cost: ExilePlayCost::from_label(&permission.cost)
                            .ok_or("unknown exile-play cost")?,
                        until_end_of_turn: match permission.until_end_of_turn {
                            Some((player, turn)) => Some((player_from_index(player)?, turn)),
                            None => None,
                        },
                        adventure_return_only: permission.adventure_return_only,
                        surcharge: permission
                            .surcharge
                            .as_ref()
                            .map_or_else(ManaCost::default, mana_cost_from_snapshot),
                        not_before_turn: match permission.not_before_turn {
                            Some((player, turn)) => Some((player_from_index(player)?, turn)),
                            None => None,
                        },
                    })
                })
                .collect::<Result<Vec<_>, String>>()?,
            monarch: checkpoint.monarch.map(player_from_index).transpose()?,
            linked_exiles: checkpoint
                .linked_exiles
                .iter()
                .map(|pair| (GameObjectId(pair[0]), GameObjectId(pair[1])))
                .collect(),
            sorcery_flash_grants: checkpoint.sorcery_flash_grants,
            turn_phase_queue: checkpoint
                .turn_phase_queue
                .iter()
                .copied()
                .map(parse_turn_phase)
                .collect(),
            turn_phase_resume: checkpoint.turn_phase_resume.map(parse_turn_phase_resume),
            resolved_play_restrictions,
            resolved_play_permissions,
            emblems: Vec::new(),
            spells_cast_this_turn: checkpoint.spells_cast_this_turn,
            spells_cast_last_turn: checkpoint.spells_cast_last_turn,
            cards_drawn_this_turn: checkpoint.cards_drawn_this_turn,
            citys_blessing: checkpoint.citys_blessing,
            permanent_left_battlefield_this_turn: checkpoint.permanent_left_battlefield_this_turn,
            card_left_graveyard_this_turn: checkpoint.card_left_graveyard_this_turn,
            life_gained_this_turn: checkpoint.life_gained_this_turn,
            draw_step_draw_taken: checkpoint.draw_step_draw_taken,
            drawn_this_turn: parse_drawn_this_turn(&checkpoint, hidden, viewer, &checkpoint_hands)?,
            defer_empty_library_loss: checkpoint.defer_empty_library_loss,
            draw_replacements: std::array::from_fn(|_| VecDeque::new()),
            installed_triggers: Vec::new(),
            next_installed_trigger_id: checkpoint.next_installed_trigger_id,
            blockers_declared: checkpoint.blockers_declared,
            untap_pending: checkpoint.untap_pending,
            pregame: parse_pregame(checkpoint.pregame)?,
            mulligans: checkpoint.mulligans,
            cleanup_pending: checkpoint.cleanup_pending,
            pending_decisions: Vec::new(),
            pending_discard_follow_up: None,
            next_decision_id: checkpoint.next_decision_id,
            pending_events: VecDeque::new(),
            pending_procedures: VecDeque::new(),
            pending_triggers: Vec::new(),
            next_trigger_id: checkpoint.next_trigger_id,
            last_seen_hands: [None, None],
            pending_combat_assignments: ids(&checkpoint.pending_combat_attackers),
            combat_damage_stage: parse_combat_stage(&checkpoint.combat_damage_stage),
            combat_blocked_attackers: ids(&checkpoint.combat_blocked_attackers),
            extra_turns: checkpoint
                .extra_turns
                .iter()
                .copied()
                .map(player_from_index)
                .collect::<Result<Vec<_>, _>>()?,
            next_regular_player: player_from_index(checkpoint.next_regular_player)?,
            channel_active: checkpoint.channel_active,
            damage_preventions,
            damage_redirects,
            result: None,
            events: vec![GameEvent::GameStarted { seed: rollout_seed }],
        };
        let (battlefield, phased_out) =
            parse_battlefield(observation, &checkpoint.battlefield, &game.catalog)?;
        game.battlefield = battlefield;
        game.phased_out = phased_out;
        game.emblems = parse_emblems(observation, &checkpoint.emblems, &game)?;
        game.retired_objects = parse_retired_objects(&checkpoint.retired_objects, &game)?;
        game.successors = checkpoint
            .successors
            .iter()
            .map(|entry| (GameObjectId(entry.retired), GameObjectId(entry.became)))
            .collect();

        game.stack = parse_stack(observation, &checkpoint.stack, &game)?;
        game.ongoing_effects = checkpoint
            .ongoing_effects
            .iter()
            .map(|ongoing| parse_ongoing_effect(ongoing, &game))
            .collect::<Result<Vec<_>, _>>()?;
        game.pending_events = parse_pending_events(&checkpoint.pending_events, &game.catalog)?;
        game.installed_triggers = checkpoint
            .installed_triggers
            .iter()
            .map(|trigger| parse_installed_trigger(trigger, &game))
            .collect::<Result<Vec<_>, _>>()?;
        game.pending_triggers = checkpoint
            .pending_triggers
            .iter()
            .map(|trigger| parse_pending_trigger(trigger, &game))
            .collect::<Result<Vec<_>, _>>()?;
        let draw_replacements = [PlayerId::One, PlayerId::Two].map(|player| {
            checkpoint.draw_replacements[player.index()]
                .iter()
                .map(|replacement| parse_draw_replacement(replacement, &game))
                .collect::<Result<VecDeque<_>, _>>()
        });
        let [replacements_one, replacements_two] = draw_replacements;
        game.draw_replacements = [replacements_one?, replacements_two?];
        game.pending_procedures = checkpoint
            .pending_procedures
            .iter()
            .map(|procedure| parse_pending_procedure(procedure, &game))
            .collect::<Result<VecDeque<_>, _>>()?;
        game.pending_decisions = parse_pending_decision(
            observation,
            checkpoint.decision_state.as_ref(),
            hidden,
            &game,
        )?
        .into_iter()
        .collect();
        game.last_seen_hands[viewer.index()] =
            parse_last_seen_hand(observation.get("lastSeenHand"))?;
        if game.pending_decisions.iter().any(|decision| {
            decision.observation.id >= game.next_decision_id && game.next_decision_id != u32::MAX
        }) {
            return Err("checkpoint next decision id does not follow its pending decision".into());
        }
        if game
            .pending_triggers
            .iter()
            .any(|trigger| trigger.id >= game.next_trigger_id && game.next_trigger_id != u32::MAX)
        {
            return Err("checkpoint next trigger id does not follow its pending triggers".into());
        }
        if game.installed_triggers.iter().any(|trigger| {
            trigger.id >= game.next_installed_trigger_id
                && game.next_installed_trigger_id != u32::MAX
        }) {
            return Err(
                "checkpoint next installed trigger id does not follow its installed triggers"
                    .into(),
            );
        }
        Ok(game)
    }
}

include!("state_checkpoint/support.rs");

#[cfg(test)]
mod tests;
