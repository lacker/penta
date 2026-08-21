impl Game {
    const fn face_down_granted_origin(
        source: GameObjectId,
        source_ability: AbilityId,
        grant: GrantId,
    ) -> AbilityOrigin {
        AbilityOrigin::FaceDownGranted {
            source,
            source_ability,
            grant,
        }
    }

    pub(super) const fn granted_ability_origin(
        source: GameObjectId,
        origin: AbilityOrigin,
        fallback: ObjectCharacteristics,
        grant: GrantId,
    ) -> AbilityOrigin {
        match origin {
            AbilityOrigin::Printed {
                definition,
                part,
                ability,
            } => AbilityOrigin::Granted {
                source,
                source_definition: definition,
                source_part: part,
                source_ability: ability,
                grant,
            },
            AbilityOrigin::Token { part, ability } => AbilityOrigin::TokenGranted {
                source,
                source_part: part,
                source_ability: ability,
                grant,
            },
            AbilityOrigin::Emblem { ability } => AbilityOrigin::EmblemGranted {
                source,
                source_ability: ability,
                grant,
            },
            AbilityOrigin::FaceDown { ability } => {
                Self::face_down_granted_origin(source, ability, grant)
            }
            AbilityOrigin::Granted {
                source_definition,
                source_part,
                source_ability,
                ..
            } => AbilityOrigin::Granted {
                source,
                source_definition,
                source_part,
                source_ability,
                grant,
            },
            AbilityOrigin::TokenGranted {
                source_part,
                source_ability,
                ..
            } => AbilityOrigin::TokenGranted {
                source,
                source_part,
                source_ability,
                grant,
            },
            AbilityOrigin::EmblemGranted { source_ability, .. } => AbilityOrigin::EmblemGranted {
                source,
                source_ability,
                grant,
            },
            AbilityOrigin::FaceDownGranted { source_ability, .. } => {
                Self::face_down_granted_origin(source, source_ability, grant)
            }
            AbilityOrigin::IntrinsicBasicLand(_) | AbilityOrigin::IntrinsicCounter(_) => {
                match fallback {
                    ObjectCharacteristics::Card { definition, part } => AbilityOrigin::Granted {
                        source,
                        source_definition: definition,
                        source_part: part,
                        source_ability: AbilityId::PRIMARY,
                        grant,
                    },
                    ObjectCharacteristics::Token { part, .. } => AbilityOrigin::TokenGranted {
                        source,
                        source_part: part,
                        source_ability: AbilityId::PRIMARY,
                        grant,
                    },
                    ObjectCharacteristics::Emblem { .. } => AbilityOrigin::EmblemGranted {
                        source,
                        source_ability: AbilityId::PRIMARY,
                        grant,
                    },
                    ObjectCharacteristics::FaceDown { .. } => {
                        Self::face_down_granted_origin(source, AbilityId::PRIMARY, grant)
                    }
                }
            }
        }
    }

    /// Finishes an atomic rules procedure before a player can receive
    /// priority. Mana abilities invoked while casting resolve inside the
    /// procedure, while ordinary triggers collected by them wait here.
    pub(super) fn finish_rules_procedure(&mut self) {
        // A decision can be one step in a still-resolving spell or turn-based
        // procedure. Neither state-based actions nor trigger placement happen
        // in the middle of that procedure: for example, a creature dealt
        // lethal damage by Chain Lightning can still activate a mana ability
        // when its controller is asked whether to pay for the copy. Drain the
        // continuation chain before reaching either priority-boundary check.
        loop {
            if self.pending_decisions.is_empty() && !self.pending_events.is_empty() {
                self.continue_pending_events();
            }
            if !self.pending_decisions.is_empty() || !self.pending_events.is_empty() {
                return;
            }
            if self.pending_procedures.is_empty() {
                break;
            }
            self.continue_pending_procedures();
        }

        self.check_state_based_actions();
        if self.result.is_none()
            && self.pending_decisions.is_empty()
            && self.pending_events.is_empty()
            && self.pending_procedures.is_empty()
        {
            self.begin_trigger_placement();
        }
    }

    pub(super) fn capture_trigger(&mut self, capture: &TriggerCapture) {
        // Rule 603.4: an intervening-if condition is checked as the ability
        // would trigger. Failing it means the ability never triggers at all,
        // so nothing reaches the stack and nothing is reported.
        if !self.trigger_capture_condition_holds(capture) {
            return;
        }
        self.capture_trigger_prechecked(capture);
    }

    fn trigger_capture_condition_holds(&self, capture: &TriggerCapture) -> bool {
        capture.condition.is_none_or(|condition| {
            self.trigger_condition_holds(
                condition,
                capture.source.object,
                capture.controller,
                capture.context.trigger,
                Some(capture.source.ability),
                None,
            )
        })
    }

    fn capture_trigger_prechecked(&mut self, capture: &TriggerCapture) {
        let id = self.next_trigger_id;
        self.next_trigger_id = self.next_trigger_id.saturating_add(1);
        self.pending_triggers.push(PendingTrigger {
            id,
            source: capture.source,
            presentation: capture.presentation,
            owner: capture.owner,
            controller: capture.controller,
            text: capture.text,
            target_defs: capture.target_defs.clone(),
            targets: capture.targets.clone(),
            effect: capture.effect,
            resolver: capture.resolver,
            context: capture.context.clone(),
            condition: capture.condition,
            x: capture.x,
        });
        self.events.push(GameEvent::AbilityTriggered {
            player: capture.controller,
            trigger: id,
            source: capture.source.object,
            presentation: capture.presentation,
        });
    }

    pub(super) const fn authored_ability_origin(
        source: ObjectCharacteristics,
        ability: AbilityId,
    ) -> AbilityOrigin {
        match source {
            ObjectCharacteristics::Card { definition, part } => AbilityOrigin::Printed {
                definition,
                part,
                ability,
            },
            ObjectCharacteristics::Token { part, .. } => AbilityOrigin::Token { part, ability },
            ObjectCharacteristics::Emblem { .. } => AbilityOrigin::Emblem { ability },
            ObjectCharacteristics::FaceDown { .. } => AbilityOrigin::FaceDown { ability },
        }
    }

    pub(super) const fn ability_presentation(
        origin: AbilityOrigin,
        fallback: ObjectCharacteristics,
    ) -> ObjectCharacteristics {
        match origin {
            AbilityOrigin::Printed {
                definition, part, ..
            } => ObjectCharacteristics::card(definition, part),
            AbilityOrigin::Token { part, .. } => match fallback {
                ObjectCharacteristics::Token { token, .. } => {
                    ObjectCharacteristics::token(token, part)
                }
                ObjectCharacteristics::Card { .. }
                | ObjectCharacteristics::Emblem { .. }
                | ObjectCharacteristics::FaceDown { .. } => {
                    fallback
                }
            },
            AbilityOrigin::Emblem { .. }
            | AbilityOrigin::FaceDown { .. }
            | AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_)
            | AbilityOrigin::Granted { .. }
            | AbilityOrigin::TokenGranted { .. }
            | AbilityOrigin::EmblemGranted { .. }
            | AbilityOrigin::FaceDownGranted { .. } => fallback,
        }
    }
}
