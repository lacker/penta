impl Game {
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
                ObjectCharacteristics::Card { .. } | ObjectCharacteristics::Emblem { .. } => {
                    fallback
                }
            },
            AbilityOrigin::Emblem { .. }
            | AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_)
            | AbilityOrigin::Granted { .. }
            | AbilityOrigin::TokenGranted { .. }
            | AbilityOrigin::EmblemGranted { .. } => fallback,
        }
    }
}
