use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde_json::Value;

use super::{
    AbilityEffectExpiration, AbilitySourceRef, ApplicableReplacement, AppliedStackEffect,
    BasicLandTypeChange, BattlefieldEntryReplacementEffect, CardInstance, CharacteristicSource,
    CombatDamageStage, ContinuousEffectTimestamp, CopiableAbility, CopiableCharacteristics,
    CounterKind, EntryCompletion, Game, GameEvent, GameObjectId, GameStack, Mana, ManaSource,
    ObjectBacking, PendingBattlefieldEntry, PendingEvent, PendingReplacementEffect, Permanent,
    PlayerId, PlayerState, Pregame, ReplaceableEvent, ReplacementEffectContext, ReplayRng,
    RetiredObject, ScopedEffect, StackAbilityPayload, StackObject, StackObjectKind, Step,
    TemporaryAbilityGrant, TemporaryGrantedAbility, TemporaryRemovedAbilities, TriggerContext,
    ZoneMoveCause,
};
use crate::card::{
    AppliedEffectDef, BasicLandType, CardType, CardTypeSet, DeclarativeAbilityDef, EffectDef,
    EffectRecipientDef, ReplacementEffectDef, ReplacementEventDef, SpellForm, ZoneKind,
};
use crate::casting::{CastChoices, CastSignature, CostConfiguration, TargetSelection};
use crate::{
    AbilityId, AbilityOrigin, AdditionalCostId, AlternativeCostId, AttackDefender, CardCatalog,
    CardDefinitionId, CardPartId, Format, GameObjectId as PublicGameObjectId, GrantId, ModeId,
    PlayOptionId, Target, TargetSlotId,
};

mod decision;
mod emblem;
mod event;
mod model;
mod model_animation;
mod model_procedure;
mod model_trigger;
mod permanent;
mod procedure;
mod semantics;
mod stack;
mod trigger;
mod wire;
mod wire_decision;

use decision::{
    decision_referenced_object_ids, decision_snapshot, parse_pending_decision,
    parse_pending_trigger, pending_trigger_snapshot,
};
use emblem::{emblem_snapshot, parse_emblems};
#[cfg(test)]
use event::entry_replacement_locator;
use event::{
    applicable_replacement_snapshot, entry_replacement_effect, parse_applicable_replacement,
    parse_replacement_context_snapshot, pending_event_referenced_object_ids,
    pending_event_snapshot,
};
use model::{
    AbilityActivationSnapshot, AbilityEffectExpirationSnapshot, AbilityOriginSnapshot,
    AbilitySourceSnapshot, ApplicableReplacementSnapshot, AttackDefenderSnapshot,
    BasicLandTypeSnapshot, CombatDamageAssignmentSnapshot, CombatDamageStageSnapshot,
    CopiableAbilitySnapshot, CopiableCharacteristicsSnapshot, CopiedFromSnapshot,
    DetachedCardSnapshot, DetachedPermanentSnapshot, EntryCompletionSnapshot,
    EntryReplacementLocator, GameSnapshot, ManaColorSnapshot, ManaSnapshot, ManaSourceSnapshot,
    PendingBattlefieldEntrySnapshot, PendingEventSnapshot, PendingReplacementEffectSnapshot,
    PermanentSnapshot, PregameSnapshot, ReplacementEffectContextSnapshot, RetiredObjectSnapshot,
    StackSnapshot, TemporaryAbilityGrantSnapshot, TemporaryGrantedAbilitySnapshot,
    TemporaryRemovedAbilitySnapshot, ZoneKindSnapshot,
};
use model_animation::UpkeepKeywordSnapshot;
use permanent::{detached_permanent_snapshot, permanent_snapshot};
use procedure::{
    draw_replacement_referenced_object_ids, draw_replacement_snapshot, parse_draw_replacement,
    parse_pending_procedure, pending_procedure_referenced_object_ids, pending_procedure_snapshot,
};
use semantics::{
    ability_locator, animation_snapshot, applied_effect_locator, catalog_ability,
    catalog_animation, catalog_applied_effect, catalog_mana_payload, keyword_snapshot,
    mana_payload_locator, parse_keyword,
};
use stack::{
    applied_stack_effect_snapshots, detached_stack_snapshot, parse_detached_stack, parse_stack,
    parse_target as parse_snapshot_target, referenced_object_ids, stack_ability_snapshot,
    stack_object_requires_retired, target_snapshot,
};
use trigger::{
    delayed_trigger_snapshot, floating_trigger_snapshot, parse_delayed_trigger,
    parse_floating_trigger,
};
#[allow(clippy::wildcard_imports)]
use wire::*;
use wire_decision::rebind_visible_decision_cards;

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
            .flat_map(referenced_object_ids)
            .chain(
                self.battlefield
                    .iter()
                    .flat_map(|permanent| permanent.damage_sources.iter().copied()),
            )
            .chain(self.delayed_triggers.iter().flat_map(|trigger| {
                referenced_object_ids(&trigger.object)
                    .chain(trigger.context.object)
                    .chain(trigger.context.chosen_objects.iter().flatten().copied())
                    .collect::<Vec<_>>()
            }))
            .chain(self.floating_triggers.iter().flat_map(|trigger| {
                [trigger.capture.source.object]
                    .into_iter()
                    .chain(trigger.capture.context.object)
                    .chain(
                        trigger
                            .capture
                            .context
                            .chosen_objects
                            .iter()
                            .flatten()
                            .copied(),
                    )
            }))
            .chain(self.pending_triggers.iter().flat_map(|trigger| {
                [trigger.source.object]
                    .into_iter()
                    .chain(trigger.context.object)
                    .chain(trigger.context.chosen_objects.iter().flatten().copied())
            }))
            .chain(
                self.pending_events
                    .iter()
                    .flat_map(pending_event_referenced_object_ids),
            )
            .chain(
                self.pending_decisions
                    .iter()
                    .flat_map(|pending| decision_referenced_object_ids(&pending.continuation)),
            )
            .chain(
                self.draw_replacements
                    .iter()
                    .flatten()
                    .flat_map(draw_replacement_referenced_object_ids),
            )
            .chain(
                self.pending_procedures
                    .iter()
                    .flat_map(pending_procedure_referenced_object_ids),
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
                    permanent: detached_permanent_snapshot(&self.catalog, permanent),
                    power: *power,
                    toughness: *toughness,
                    mana_value: *mana_value,
                    keywords: keywords.iter().copied().map(keyword_snapshot).collect(),
                }),
                RetiredObject::Card(card) => Some(RetiredObjectSnapshot::Card {
                    card: DetachedCardSnapshot {
                        object_id: card.id.0,
                        definition: card.definition.0,
                        owner: card.owner.index(),
                    },
                }),
                RetiredObject::Stack(object) => Some(RetiredObjectSnapshot::Stack {
                    object: detached_stack_snapshot(self, object)?,
                }),
            })
            .collect::<Vec<_>>();
        let has_unlocated_retired_object = retired_objects.len() != retired_ids.len();
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
        let delayed_triggers = self
            .delayed_triggers
            .iter()
            .filter_map(|trigger| delayed_trigger_snapshot(self, trigger))
            .collect::<Vec<_>>();
        let has_unlocated_delayed_trigger = delayed_triggers.len() != self.delayed_triggers.len();
        let floating_triggers = self
            .floating_triggers
            .iter()
            .filter_map(|trigger| floating_trigger_snapshot(self, trigger))
            .collect::<Vec<_>>();
        let has_unlocated_floating_trigger =
            floating_triggers.len() != self.floating_triggers.len();
        let pending_triggers = self
            .pending_triggers
            .iter()
            .filter_map(|trigger| pending_trigger_snapshot(self, trigger))
            .collect::<Vec<_>>();
        let has_unlocated_pending_trigger = pending_triggers.len() != self.pending_triggers.len();
        let draw_replacements = [PlayerId::One, PlayerId::Two].map(|player| {
            self.draw_replacements[player.index()]
                .iter()
                .filter_map(|replacement| draw_replacement_snapshot(self, replacement))
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
            .filter_map(|procedure| pending_procedure_snapshot(self, procedure))
            .collect::<Vec<_>>();
        let has_unlocated_pending_procedure =
            pending_procedures.len() != self.pending_procedures.len();
        GameSnapshot {
            version: crate::protocol::CHECKPOINT_VERSION,
            simulation_fingerprint: crate::protocol::SIMULATION_FINGERPRINT.to_owned(),
            turns_started: self.turns_started,
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
            linked_exiles: self
                .linked_exiles
                .iter()
                .map(|(source, card)| [source.0, card.0])
                .collect(),
            sorcery_flash_grants: self.sorcery_flash_grants,
            additional_combat_phases: self.additional_combat_phases,
            noncreature_casts_locked: self.noncreature_casts_locked,
            spells_cast_this_turn: self.spells_cast_this_turn,
            spells_cast_last_turn: self.spells_cast_last_turn,
            cards_drawn_this_turn: self.cards_drawn_this_turn,
            drawn_this_turn: visible_drawn_this_turn,
            defer_empty_library_loss: self.defer_empty_library_loss,
            draw_replacements,
            miracle_window: self
                .miracle_window
                .filter(|id| {
                    self.players[viewer.index()]
                        .hand
                        .iter()
                        .any(|card| card.id == *id)
                })
                .map(|id| id.0),
            pending_combat_attackers: self
                .pending_combat_attackers
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
            battlefield: self
                .battlefield
                .iter()
                .map(|permanent| permanent_snapshot(&self.catalog, permanent))
                .collect(),
            emblems: self.emblems.iter().map(emblem_snapshot).collect(),
            stack: self
                .stack
                .iter()
                .map(|object| {
                    let ability_payload = (object.kind != StackObjectKind::Spell)
                        .then(|| stack_ability_snapshot(self, object))
                        .flatten();
                    let has_unlocated_ability_payload = object.kind != StackObjectKind::Spell
                        && object.ability.is_some()
                        && ability_payload.is_none();
                    let (applied_effects, has_unlocated_applied_effect) =
                        applied_stack_effect_snapshots(self, object);
                    StackSnapshot {
                        object_id: object.id.0,
                        owner: object.card.owner.index(),
                        ability_payload,
                        requires_retired_object: stack_object_requires_retired(self, object),
                        has_runtime_overrides: has_unlocated_ability_payload
                            || has_unlocated_applied_effect,
                        applied_effects,
                        text_changes: object
                            .text_changes
                            .iter()
                            .map(|change| model::BasicLandTypeChangeSnapshot {
                                from: basic_land_type_snapshot(change.from),
                                to: basic_land_type_snapshot(change.to),
                            })
                            .collect(),
                        colors: object.colors.map(crate::card::ColorSet::to_flags),
                        cast_via_flashback: object.cast_via_flashback,
                        is_copy: object.is_copy,
                    }
                })
                .collect(),
            retired_objects,
            pending_events,
            temporary_ability_grants,
            delayed_triggers,
            floating_triggers,
            pending_triggers,
            pending_procedures,
            decision_state,
            has_deferred_state: has_unlocated_temporary_ability_grant
                || has_unlocated_delayed_trigger
                || has_unlocated_floating_trigger
                || has_unsupported_decision
                || has_unsupported_event
                || has_unlocated_pending_trigger
                || has_unlocated_retired_object
                || has_unlocated_mana
                || has_unlocated_draw_replacement
                || has_unlocated_pending_procedure,
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
        let mut checkpoint_hands = if viewer == PlayerId::One {
            [own_hand, opponent_hand]
        } else {
            [opponent_hand, own_hand]
        };
        let mut libraries = [library_one, library_two];
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
        let mut game = Self {
            format,
            seed: rollout_seed,
            rng: ReplayRng::new(rollout_seed),
            catalog,
            physical_cards: Vec::new(),
            players,
            battlefield: Vec::new(),
            stack: GameStack::default(),
            retired_objects: BTreeMap::new(),
            temporary_ability_grants,
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
            linked_exiles: checkpoint
                .linked_exiles
                .iter()
                .map(|pair| (GameObjectId(pair[0]), GameObjectId(pair[1])))
                .collect(),
            sorcery_flash_grants: checkpoint.sorcery_flash_grants,
            additional_combat_phases: checkpoint.additional_combat_phases,
            noncreature_casts_locked: checkpoint.noncreature_casts_locked,
            emblems: Vec::new(),
            spells_cast_this_turn: checkpoint.spells_cast_this_turn,
            spells_cast_last_turn: checkpoint.spells_cast_last_turn,
            cards_drawn_this_turn: checkpoint.cards_drawn_this_turn,
            drawn_this_turn: parse_drawn_this_turn(&checkpoint, hidden, viewer, &checkpoint_hands)?,
            defer_empty_library_loss: checkpoint.defer_empty_library_loss,
            draw_replacements: std::array::from_fn(|_| VecDeque::new()),
            miracle_window: parse_miracle_window(&checkpoint, hidden, viewer, &checkpoint_hands)?,
            delayed_triggers: Vec::new(),
            floating_triggers: Vec::new(),
            blockers_declared: checkpoint.blockers_declared,
            untap_pending: checkpoint.untap_pending,
            pregame: parse_pregame(checkpoint.pregame)?,
            mulligans: checkpoint.mulligans,
            cleanup_pending: checkpoint.cleanup_pending,
            pending_decisions: Vec::new(),
            next_decision_id: checkpoint.next_decision_id,
            pending_events: VecDeque::new(),
            pending_procedures: VecDeque::new(),
            pending_triggers: Vec::new(),
            next_trigger_id: checkpoint.next_trigger_id,
            last_seen_hands: [None, None],
            pending_combat_attackers: ids(&checkpoint.pending_combat_attackers),
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
            result: None,
            events: vec![GameEvent::GameStarted { seed: rollout_seed }],
        };
        game.battlefield = parse_battlefield(observation, &checkpoint.battlefield, &game.catalog)?;
        game.emblems = parse_emblems(observation, &checkpoint.emblems, &game)?;
        game.retired_objects = parse_retired_objects(&checkpoint.retired_objects, &game)?;
        game.stack = parse_stack(observation, &checkpoint.stack, &game)?;
        game.pending_events = parse_pending_events(&checkpoint.pending_events, &game.catalog)?;
        game.delayed_triggers = checkpoint
            .delayed_triggers
            .iter()
            .map(|trigger| parse_delayed_trigger(trigger, &game))
            .collect::<Result<Vec<_>, _>>()?;
        game.floating_triggers = checkpoint
            .floating_triggers
            .iter()
            .map(|trigger| parse_floating_trigger(trigger, &game))
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
        Ok(game)
    }
}

const fn zone_kind_snapshot(zone: ZoneKind) -> ZoneKindSnapshot {
    match zone {
        ZoneKind::Library => ZoneKindSnapshot::Library,
        ZoneKind::Hand => ZoneKindSnapshot::Hand,
        ZoneKind::Battlefield => ZoneKindSnapshot::Battlefield,
        ZoneKind::Graveyard => ZoneKindSnapshot::Graveyard,
        ZoneKind::Stack => ZoneKindSnapshot::Stack,
        ZoneKind::Exile => ZoneKindSnapshot::Exile,
        ZoneKind::Command => ZoneKindSnapshot::Command,
    }
}

const fn completion_snapshot(completion: EntryCompletion) -> EntryCompletionSnapshot {
    match completion {
        EntryCompletion::LandPlayed { player } => EntryCompletionSnapshot::LandPlayed {
            seat: player.index(),
        },
        EntryCompletion::SpellResolved { card, definition } => {
            EntryCompletionSnapshot::SpellResolved {
                card: card.0,
                definition: definition.0,
            }
        }
        EntryCompletion::Setup => EntryCompletionSnapshot::Setup,
        EntryCompletion::None => EntryCompletionSnapshot::None,
    }
}

fn mana_snapshot(catalog: &CardCatalog, mana: Mana) -> ManaSnapshot {
    ManaSnapshot {
        color: mana_color_snapshot(mana.color),
        source: mana.source.map(|source| ManaSourceSnapshot {
            object: source.object.0,
            ability: ability_origin_snapshot(source.ability),
        }),
        payload: mana_payload_locator(catalog, mana),
    }
}

const fn mana_color_snapshot(color: crate::ManaColor) -> ManaColorSnapshot {
    match color {
        crate::ManaColor::White => ManaColorSnapshot::White,
        crate::ManaColor::Blue => ManaColorSnapshot::Blue,
        crate::ManaColor::Black => ManaColorSnapshot::Black,
        crate::ManaColor::Red => ManaColorSnapshot::Red,
        crate::ManaColor::Green => ManaColorSnapshot::Green,
        crate::ManaColor::Colorless => ManaColorSnapshot::Colorless,
    }
}

const fn parse_mana_color(color: ManaColorSnapshot) -> crate::ManaColor {
    match color {
        ManaColorSnapshot::White => crate::ManaColor::White,
        ManaColorSnapshot::Blue => crate::ManaColor::Blue,
        ManaColorSnapshot::Black => crate::ManaColor::Black,
        ManaColorSnapshot::Red => crate::ManaColor::Red,
        ManaColorSnapshot::Green => crate::ManaColor::Green,
        ManaColorSnapshot::Colorless => crate::ManaColor::Colorless,
    }
}

const fn expiration_snapshot(
    expiration: AbilityEffectExpiration,
) -> AbilityEffectExpirationSnapshot {
    match expiration {
        AbilityEffectExpiration::EndOfTurn => AbilityEffectExpirationSnapshot::EndOfTurn,
        AbilityEffectExpiration::UpkeepOf(player) => AbilityEffectExpirationSnapshot::UpkeepOf {
            seat: player.index(),
        },
        AbilityEffectExpiration::TurnOf { player, turn } => {
            AbilityEffectExpirationSnapshot::TurnOf {
                seat: player.index(),
                turn,
            }
        }
        AbilityEffectExpiration::Never => AbilityEffectExpirationSnapshot::Never,
    }
}

fn parse_expiration(
    expiration: AbilityEffectExpirationSnapshot,
) -> Result<AbilityEffectExpiration, String> {
    match expiration {
        AbilityEffectExpirationSnapshot::EndOfTurn => Ok(AbilityEffectExpiration::EndOfTurn),
        AbilityEffectExpirationSnapshot::UpkeepOf { seat } => {
            Ok(AbilityEffectExpiration::UpkeepOf(player_from_index(seat)?))
        }
        AbilityEffectExpirationSnapshot::TurnOf { seat, turn } => {
            Ok(AbilityEffectExpiration::TurnOf {
                player: player_from_index(seat)?,
                turn,
            })
        }
        AbilityEffectExpirationSnapshot::Never => Ok(AbilityEffectExpiration::Never),
    }
}

const fn ability_origin_snapshot(origin: AbilityOrigin) -> AbilityOriginSnapshot {
    match origin {
        AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } => AbilityOriginSnapshot::Printed {
            definition: definition.0,
            part_id: part.0,
            ability_id: ability.0,
        },
        AbilityOrigin::IntrinsicBasicLand(land_type) => AbilityOriginSnapshot::IntrinsicBasicLand {
            land_type: basic_land_type_snapshot(land_type),
        },
        AbilityOrigin::Granted {
            source,
            source_definition,
            source_part,
            source_ability,
            grant,
        } => AbilityOriginSnapshot::Granted {
            source: source.0,
            source_definition: source_definition.0,
            source_part_id: source_part.0,
            source_ability_id: source_ability.0,
            grant_id: grant.0,
        },
    }
}

const fn ability_origin_from_snapshot(origin: AbilityOriginSnapshot) -> AbilityOrigin {
    match origin {
        AbilityOriginSnapshot::Printed {
            definition,
            part_id,
            ability_id,
        } => AbilityOrigin::Printed {
            definition: CardDefinitionId(definition),
            part: CardPartId(part_id),
            ability: AbilityId(ability_id),
        },
        AbilityOriginSnapshot::IntrinsicBasicLand { land_type } => {
            AbilityOrigin::IntrinsicBasicLand(parse_basic_land_type(land_type))
        }
        AbilityOriginSnapshot::Granted {
            source,
            source_definition,
            source_part_id,
            source_ability_id,
            grant_id,
        } => AbilityOrigin::Granted {
            source: GameObjectId(source),
            source_definition: CardDefinitionId(source_definition),
            source_part: CardPartId(source_part_id),
            source_ability: AbilityId(source_ability_id),
            grant: GrantId(grant_id),
        },
    }
}

const fn basic_land_type_snapshot(value: BasicLandType) -> BasicLandTypeSnapshot {
    match value {
        BasicLandType::Plains => BasicLandTypeSnapshot::Plains,
        BasicLandType::Island => BasicLandTypeSnapshot::Island,
        BasicLandType::Swamp => BasicLandTypeSnapshot::Swamp,
        BasicLandType::Mountain => BasicLandTypeSnapshot::Mountain,
        BasicLandType::Forest => BasicLandTypeSnapshot::Forest,
    }
}

const fn parse_basic_land_type(value: BasicLandTypeSnapshot) -> BasicLandType {
    match value {
        BasicLandTypeSnapshot::Plains => BasicLandType::Plains,
        BasicLandTypeSnapshot::Island => BasicLandType::Island,
        BasicLandTypeSnapshot::Swamp => BasicLandType::Swamp,
        BasicLandTypeSnapshot::Mountain => BasicLandType::Mountain,
        BasicLandTypeSnapshot::Forest => BasicLandType::Forest,
    }
}

#[cfg(test)]
mod tests;
