use super::{
    AbilityTargetDef, CardPartId, DecisionContinuation, DecisionKind, DecisionObservation,
    DecisionOption, DecisionOrderSemantics, DecisionPreference, DecisionVisibility, DecisionZone,
    EffectDef, Game, GameEvent, GameObjectId, ObjectCharacteristics, PendingDecision,
    PendingTrigger, PlayerId, StackAbilityPayload, StackObject, StackObjectKind, Target,
    TargetSelection, TargetSlotId, TriggerPlacementBatch, ZoneKind,
};
use crate::card::TargetChooserDef;

impl Game {
    pub(super) fn begin_trigger_placement(&mut self) {
        if self.pending_triggers.is_empty() {
            return;
        }
        let triggers = std::mem::take(&mut self.pending_triggers);
        let mut batches = Vec::new();
        for controller in [self.active_player, self.active_player.opponent()] {
            let controlled = triggers
                .iter()
                .filter(|trigger| trigger.controller == controller)
                .cloned()
                .collect::<Vec<_>>();
            if !controlled.is_empty() {
                batches.push(TriggerPlacementBatch {
                    controller,
                    triggers: controlled,
                });
            }
        }
        self.continue_trigger_placement(batches);
    }

    pub(super) fn continue_trigger_placement(&mut self, mut batches: Vec<TriggerPlacementBatch>) {
        let Some(batch) = (!batches.is_empty()).then(|| batches.remove(0)) else {
            // APNAP determines only how simultaneous triggers are placed. The
            // player who was about to receive priority before placement keeps
            // it afterward (for example, the nonactive player who tapped City
            // of Brass after the active player passed).
            self.consecutive_passes = 0;
            return;
        };
        if batch.triggers.len() == 1 {
            self.place_trigger_sequence(batch.triggers, batches);
        } else {
            self.queue_trigger_order_decision(batch, batches);
        }
    }

    pub(super) fn place_trigger_sequence(
        &mut self,
        mut triggers: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
    ) {
        while !triggers.is_empty() {
            let trigger = triggers.remove(0);
            // The mode comes first: what it names decides both the effect
            // and the targets there are left to choose (CR 603.3c).
            if trigger.modes.is_some() {
                self.queue_trigger_mode_decision(trigger, triggers, remaining);
                return;
            }
            if trigger.targets.len() < trigger.target_defs.len() {
                self.queue_trigger_target_decision(trigger, triggers, remaining);
                return;
            }
            self.put_trigger_on_stack(trigger);
        }
        self.continue_trigger_placement(remaining);
    }

    pub(super) fn queue_trigger_target_decision(
        &mut self,
        mut trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
    ) {
        let target = trigger.target_defs[trigger.targets.len()];
        let candidates = self.trigger_target_candidates(&trigger, target);
        if candidates.len() < usize::from(target.minimum) {
            // A triggered ability with no legal choice for a required target
            // is removed from the stack as the placement procedure completes.
            self.place_trigger_sequence(pending, remaining);
            return;
        }
        if candidates.is_empty() && target.minimum == 0 {
            let slot = TargetSlotId::from_index(trigger.targets.len())
                .expect("validated trigger targets fit the runtime slot space");
            trigger.targets.push(TargetSelection::new(slot, Vec::new()));
            let mut continued = vec![trigger];
            continued.extend(pending);
            self.place_trigger_sequence(continued, remaining);
            return;
        }

        let chooser = Self::target_chooser(&trigger, target);
        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                // Labelled from the seat that is being asked: "you" has to
                // mean the player reading it.
                label: self.target_label(chooser, *candidate),
                card: self.target_card(*candidate),
                members: Vec::new(),
                ability_text: None,
                zone: match candidate {
                    Target::Player(_) => DecisionZone::None,
                    Target::Card(id) => self.card_in_nonbattlefield_zone(*id).map_or(
                        DecisionZone::None,
                        |(zone, _)| match zone {
                            ZoneKind::Library => DecisionZone::Library,
                            ZoneKind::Hand => DecisionZone::Hand,
                            ZoneKind::Graveyard => DecisionZone::Graveyard,
                            ZoneKind::Exile => DecisionZone::Exile,
                            ZoneKind::Command => DecisionZone::Command,
                            ZoneKind::Battlefield | ZoneKind::Stack => DecisionZone::None,
                        },
                    ),
                    Target::Permanent(_) => DecisionZone::Battlefield,
                    Target::Spell(_) => DecisionZone::Stack,
                },
            })
            .collect::<Vec<_>>();
        let source_name = self.presentation_name(trigger.presentation).map_or_else(
            || "Triggered ability".to_owned(),
            std::borrow::Cow::into_owned,
        );
        let target_effect = match trigger.effect {
            EffectDef::May { effect, .. } => *effect,
            effect => effect,
        };
        let preference = if matches!(
            target_effect,
            EffectDef::ExileLinkedToSource { object, .. }
                if object
                    .legal_target()
                    .is_some_and(|slot| slot.index() == trigger.targets.len())
        ) {
            DecisionPreference::LinkedExileTargets
        } else {
            DecisionPreference::Neutral
        };
        let id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        self.pending_decisions.insert(
            0,
            PendingDecision {
                observation: DecisionObservation {
                    id,
                    // The player who chooses is asked, which is not always
                    // the player whose ability it is.
                    player: chooser,
                    kind: DecisionKind::TriggerPlacement,
                    order_semantics: None,
                    source: Some(trigger.source.object),
                    prompt: format!("{source_name}: choose {}", target.label()),
                    visibility: DecisionVisibility::Public,
                    preference,
                    minimum: usize::from(target.minimum),
                    maximum: usize::from(target.maximum).min(options.len()),
                    cancellable: false,
                    options,
                },
                continuation: DecisionContinuation::TriggerPlacement {
                    trigger,
                    pending,
                    remaining,
                    candidates,
                },
            },
        );
    }

    fn trigger_target_candidates(
        &self,
        trigger: &PendingTrigger,
        target: AbilityTargetDef,
    ) -> Vec<Target> {
        let chooser = Self::target_chooser(trigger, target);
        let candidates = self.ability_targets_matching_with_selections_for_chooser(
            target.predicate,
            &trigger.targets,
            chooser,
            trigger.controller,
            trigger.source.object,
            trigger.context.trigger,
        );
        Self::without_excluded_source(&target, trigger.source.object, candidates)
    }

    /// Who picks this slot's targets, and therefore who every relation in it
    /// is measured against. The ability's controller unless the clause hands
    /// the choice somewhere else; a clause that names the event player and
    /// has no event falls back to the controller rather than to nobody.
    fn target_chooser(trigger: &PendingTrigger, target: AbilityTargetDef) -> PlayerId {
        match target.chooser {
            TargetChooserDef::Controller => trigger.controller,
            TargetChooserDef::EventPlayer => trigger
                .context
                .trigger
                .event_player
                .unwrap_or(trigger.controller),
            TargetChooserDef::Opponent => trigger.controller.opponent(),
        }
    }

    /// Asks which mode a modal trigger was put onto the stack with. Only
    /// modes the runtime can execute are offered, and a modal trigger with
    /// none of them simply resolves as the nothing its own program is.
    pub(super) fn queue_trigger_mode_decision(
        &mut self,
        mut trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
    ) {
        let modal = trigger
            .modes
            .take()
            .expect("asked only for a modal trigger");
        let offered = modal
            .modes
            .iter()
            .enumerate()
            .filter(|(_, mode)| mode.is_executable())
            .collect::<Vec<_>>();
        // "Choose up to one" makes declining an answer in its own right, so
        // a lone executable mode is still a question worth asking.
        let required = usize::from(modal.minimum.min(1));
        let Some((only, _)) = offered
            .first()
            .filter(|_| offered.len() == 1 && required == 1)
        else {
            if offered.is_empty() {
                let mut continued = vec![trigger];
                continued.extend(pending);
                self.place_trigger_sequence(continued, remaining);
                return;
            }
            let options = offered
                .iter()
                .map(|(index, mode)| DecisionOption {
                    id: u32::try_from(*index).unwrap_or(u32::MAX),
                    label: mode.text.to_owned(),
                    card: None,
                    members: Vec::new(),
                    ability_text: Some(mode.text.to_owned()),
                    zone: DecisionZone::None,
                })
                .collect::<Vec<_>>();
            let source_name = self.presentation_name(trigger.presentation).map_or_else(
                || "Triggered ability".to_owned(),
                std::borrow::Cow::into_owned,
            );
            let id = self.next_decision_id;
            self.next_decision_id = self.next_decision_id.saturating_add(1);
            self.pending_decisions.insert(
                0,
                PendingDecision {
                    observation: DecisionObservation {
                        id,
                        player: trigger.controller,
                        kind: DecisionKind::TriggerPlacement,
                        order_semantics: None,
                        source: Some(trigger.source.object),
                        prompt: if required == 1 {
                            format!("{source_name}: choose one")
                        } else {
                            format!("{source_name}: choose up to one")
                        },
                        visibility: DecisionVisibility::Public,
                        preference: DecisionPreference::Neutral,
                        minimum: required,
                        maximum: 1,
                        cancellable: false,
                        options,
                    },
                    continuation: DecisionContinuation::TriggerMode {
                        trigger,
                        pending,
                        remaining,
                        modes: modal,
                    },
                },
            );
            return;
        };
        // One executable mode is no choice at all.
        Self::apply_trigger_mode(&mut trigger, modal, *only);
        let mut continued = vec![trigger];
        continued.extend(pending);
        self.place_trigger_sequence(continued, remaining);
    }

    /// Takes the chosen mode's own text, effect, and targets onto the
    /// trigger. From here it is an ordinary trigger carrying one program,
    /// which is also what lets a checkpoint locate the mode again.
    pub(super) fn apply_trigger_mode(
        trigger: &mut PendingTrigger,
        modal: crate::card::ModalSpellDef,
        index: usize,
    ) {
        let Some(mode) = modal.modes.get(index) else {
            return;
        };
        trigger.modes = None;
        trigger.text = mode.text;
        trigger.effect = mode.declarative_effect().unwrap_or(EffectDef::None);
        trigger.resolver = Self::ability_resolver(trigger.source.ability, mode);
        if let crate::card::DeclarativeAbilityDef::Spell(spell) = mode.definition {
            trigger.target_defs.extend_from_slice(spell.targets());
        }
    }

    /// Asks how a fixed total is split among targets already chosen. Every
    /// target takes at least one, so with one target there is nothing to ask.
    pub(super) fn queue_trigger_division_decision(
        &mut self,
        trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
        targets: Vec<Target>,
        divisions: Vec<Vec<u16>>,
    ) {
        let source_name = self.presentation_name(trigger.presentation).map_or_else(
            || "Triggered ability".to_owned(),
            std::borrow::Cow::into_owned,
        );
        let labels = targets
            .iter()
            .map(|target| self.target_label(trigger.controller, *target))
            .collect::<Vec<_>>();
        let options = divisions
            .iter()
            .enumerate()
            .map(|(index, amounts)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: amounts
                    .iter()
                    .zip(&labels)
                    .map(|(amount, label)| format!("{amount} to {label}"))
                    .collect::<Vec<_>>()
                    .join(", "),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect::<Vec<_>>();
        let id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        self.pending_decisions.insert(
            0,
            PendingDecision {
                observation: DecisionObservation {
                    id,
                    player: trigger.controller,
                    kind: DecisionKind::TriggerPlacement,
                    order_semantics: None,
                    source: Some(trigger.source.object),
                    prompt: format!("{source_name}: divide the total"),
                    visibility: DecisionVisibility::Public,
                    preference: DecisionPreference::Neutral,
                    minimum: 1,
                    maximum: 1,
                    cancellable: false,
                    options,
                },
                continuation: DecisionContinuation::TriggerDivision {
                    trigger,
                    pending,
                    remaining,
                    targets,
                    divisions,
                },
            },
        );
    }

    pub(super) fn queue_trigger_order_decision(
        &mut self,
        batch: TriggerPlacementBatch,
        remaining: Vec<TriggerPlacementBatch>,
    ) {
        let options = batch
            .triggers
            .iter()
            .map(|trigger| {
                let name = self
                    .presentation_name(trigger.presentation)
                    .unwrap_or_else(|| "Triggered ability".into());
                DecisionOption {
                    id: trigger.id,
                    label: format!("{name} triggered ability"),
                    card: Some((trigger.source.object, trigger.presentation)),
                    members: Vec::new(),
                    ability_text: Some(trigger.text.into()),
                    zone: DecisionZone::Battlefield,
                }
            })
            .collect::<Vec<_>>();
        let count = options.len();
        let id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        // Trigger placement precedes any older legacy prompt that was queued
        // while the enclosing event was being processed.
        self.pending_decisions.insert(
            0,
            PendingDecision {
                observation: DecisionObservation {
                    id,
                    player: batch.controller,
                    kind: DecisionKind::TriggerOrder,
                    order_semantics: Some(DecisionOrderSemantics::Resolution),
                    source: None,
                    prompt: "Choose triggered ability resolution order".into(),
                    visibility: DecisionVisibility::Public,
                    preference: DecisionPreference::Neutral,
                    minimum: count,
                    maximum: count,
                    cancellable: false,
                    options,
                },
                continuation: DecisionContinuation::TriggerOrder { batch, remaining },
            },
        );
    }

    pub(super) fn complete_trigger_order(
        &mut self,
        batch: &TriggerPlacementBatch,
        remaining: Vec<TriggerPlacementBatch>,
        resolution_order: &[u32],
    ) {
        // The last object pushed is the first to resolve, so consume the
        // player-facing resolution order in reverse.
        let push_order = resolution_order
            .iter()
            .rev()
            .map(|trigger_id| {
                batch
                    .triggers
                    .iter()
                    .find(|trigger| trigger.id == *trigger_id)
                    .expect("validated trigger order contains each pending trigger")
                    .clone()
            })
            .collect();
        self.place_trigger_sequence(push_order, remaining);
    }

    pub(super) fn target_card(
        &self,
        target: Target,
    ) -> Option<(GameObjectId, ObjectCharacteristics)> {
        match target {
            Target::Player(_) => None,
            Target::Card(id) => self.card_in_nonbattlefield_zone(id).map(|(_, card)| {
                (
                    id,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                )
            }),
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| (id, Self::effective_rules_source(permanent))),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|object| object.id == id)
                .map(|object| (id, object.presentation())),
        }
    }

    pub(super) fn put_trigger_on_stack(&mut self, trigger: PendingTrigger) {
        let card = self.unbacked_ability_object(trigger.presentation, trigger.owner);
        let object = card.id;
        self.stack.push(StackObject {
            id: object,
            kind: StackObjectKind::TriggeredAbility,
            card,
            source: Some(trigger.source.object),
            ability: Some(StackAbilityPayload {
                origin: trigger.source.ability,
                definition: None,
                presentation: trigger.presentation,
                text: Some(trigger.text),
                target_defs: trigger.target_defs,
                targets: trigger.targets,
                context: trigger.context,
                resolver: trigger.resolver,
                condition: trigger.condition,
                mode_effects: Vec::new(),
                resolution_destination: None,
                x: trigger.x,
                sacrificed_mana_value: 0,
            }),
            controller: trigger.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback: false,
            cast_via_suspend: false,
            cast_at_instant_speed: false,
            cast_from_zone: None,
            face_down: None,
            colors_of_mana_spent: crate::card::ColorSet::empty(),
            phyrexian_symbols_paid_with_life: 0,
            is_copy: false,
        });
        self.events.push(GameEvent::TriggeredAbilityPutOnStack {
            player: trigger.controller,
            trigger: trigger.id,
            object,
            source: trigger.source.object,
            presentation: trigger.presentation,
        });
        self.capture_ability_targeting_triggers(object);
    }
}
