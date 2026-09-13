// Abilities observing another ability triggering form the second APNAP pass.
// Freeze them at the event, before either source can leave and before targets
// or modes are chosen. A reserved stack identity links the two occurrences.

impl Game {
    fn capture_trigger_and_observers(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        event: &CommittedTriggerEvent,
        capture: &TriggerCapture,
    ) {
        let index = self.pending_triggers.len();
        self.capture_trigger_prechecked(capture);
        self.pending_triggers[index].observes_trigger =
            matches!(event, CommittedTriggerEvent::AbilityTriggered { .. });
        let mut observed = CommittedTriggerEvent::AbilityTriggered {
            object: GameObjectId(0),
            controller: capture.controller,
            cause: Box::new(event.clone()),
        };
        if !listeners.iter().any(|listener| {
            self.trigger_event_matches_for_controller(
                listener.event,
                &observed,
                listener.capture.source.object,
                Some(listener.capture.controller),
            )
        }) {
            return;
        }
        let reserved = self.allocate_object_id();
        self.pending_triggers[index].stack_object = Some(reserved);
        if let CommittedTriggerEvent::AbilityTriggered { object, .. } = &mut observed {
            *object = reserved;
        }
        self.capture_battlefield_triggers_from_snapshot(listeners, &observed);
    }
}
