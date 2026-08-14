use super::{
    CardDefinitionId, CharacteristicSource, DecisionContinuation, DecisionKind,
    DecisionObservation, DecisionOption, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone, EffectDef, EffectRecipientDef, Game, GameEvent, GameObjectId,
    PendingDecision, PendingTrigger, StackAbilityPayload, StackObject, StackObjectKind, Target,
    TargetSelection, TargetSlotId, TriggerPlacementBatch, ZoneKind,
};

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
        let candidates = self.ability_targets_matching(
            target.predicate,
            trigger.controller,
            trigger.source.object,
            trigger.context,
        );
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

        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self.target_label(trigger.controller, *candidate),
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
        let source_name = self
            .catalog
            .get(trigger.definition)
            .map_or("Triggered ability", |card| card.name.as_str());
        let target_effect = match trigger.effect {
            EffectDef::May { effect, .. } => *effect,
            effect => effect,
        };
        let preference = if matches!(
            target_effect,
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(slot),
            } if slot.index() == trigger.targets.len()
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
                    player: trigger.controller,
                    kind: DecisionKind::TriggerPlacement,
                    order_semantics: None,
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
                    .catalog
                    .get(trigger.definition)
                    .map_or("Triggered ability", |card| card.name.as_str());
                DecisionOption {
                    id: trigger.id,
                    label: format!("{name} triggered ability"),
                    card: Some((trigger.source.object, trigger.definition)),
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

    pub(super) fn target_card(&self, target: Target) -> Option<(GameObjectId, CardDefinitionId)> {
        match target {
            Target::Player(_) => None,
            Target::Card(id) => self
                .card_in_nonbattlefield_zone(id)
                .map(|(_, card)| (id, card.definition)),
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| (id, permanent.card.definition)),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|object| object.id == id)
                .map(|object| (id, object.card.definition)),
        }
    }

    pub(super) fn put_trigger_on_stack(&mut self, trigger: PendingTrigger) {
        let card = self.unbacked_object(
            trigger.definition,
            trigger.owner,
            CharacteristicSource::Ability(trigger.definition),
        );
        let object = card.id;
        self.stack.push(StackObject {
            id: object,
            kind: StackObjectKind::TriggeredAbility,
            card,
            source: Some(trigger.source.object),
            ability: Some(StackAbilityPayload {
                origin: trigger.source.ability,
                definition: None,
                presentation_definition: trigger.definition,
                text: Some(trigger.text),
                target_defs: trigger.target_defs.to_vec(),
                targets: trigger.targets,
                context: trigger.context,
                resolver: trigger.resolver,
                condition: trigger.condition,
                mode_effects: Vec::new(),
                x: 0,
            }),
            controller: trigger.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback: false,
            schedule_on_entry: None,
            is_copy: false,
        });
        self.events.push(GameEvent::TriggeredAbilityPutOnStack {
            player: trigger.controller,
            trigger: trigger.id,
            object,
            source: trigger.source.object,
            definition: trigger.definition,
        });
    }
}
