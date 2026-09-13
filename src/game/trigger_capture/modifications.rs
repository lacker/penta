// Continuous trigger modifiers are evaluated before an occurrence is captured.
// Listeners retain the applicable rules for look-back events; destination
// triggers consult the battlefield after the move instead (CR 603.10).

impl Game {
    fn trigger_modifications_for_listener(
        &self,
        listener: &BattlefieldTriggerListener,
    ) -> Vec<(GameObjectId, crate::card::TriggerModificationDef)> {
        let controller = listener.capture.controller;
        let permanent = listener
            .installed
            .is_none()
            .then(|| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == listener.capture.source.object)
                    .map(|permanent| self.trigger_event_object(permanent))
            })
            .flatten();
        let mut modifications = Vec::new();
        self.visit_player_static_rules_with_source(controller, |source, rule| {
            let crate::card::AppliedRuleDef::ModifyTriggers(modification) = rule else {
                return;
            };
            if modification.permanent.is_none_or(|predicate| {
                permanent.as_ref().is_some_and(|permanent| {
                    self.trigger_object_matches_for_controller(
                        predicate,
                        permanent,
                        source,
                        false,
                        Some(controller),
                    )
                })
            }) {
                modifications.push((source, *modification));
            }
        });
        modifications
    }

    fn modified_trigger_occurrences(
        &self,
        listener: &BattlefieldTriggerListener,
        event: &CommittedTriggerEvent,
    ) -> usize {
        if !matches!(
            event,
            CommittedTriggerEvent::ZoneChanged { .. } | CommittedTriggerEvent::ObjectsDied { .. }
        ) {
            return 1;
        }
        let look_back = Self::zone_change_event_observation(listener.event, event)
            == Some(ZoneChangeObservationDef::Before)
            || matches!(event, CommittedTriggerEvent::ObjectsDied { .. });
        // A from-anywhere ability of the new graveyard card observes its
        // arrival there, rather than the old permanent dying (603.6c).
        if !look_back
            && matches!(event, CommittedTriggerEvent::ZoneChanged {
            after: Some(after), from: ZoneKind::Battlefield, to: ZoneKind::Graveyard, ..
        } if after.id == listener.capture.source.object)
        {
            return 1;
        }
        let current;
        let modifications = if look_back {
            &listener.modifications
        } else {
            current = self.trigger_modifications_for_listener(listener);
            &current
        };
        // A source that moved away cannot supply a battlefield ability checked
        // after the move. Its new graveyard object can supply its own listener.
        if !look_back
            && listener.installed.is_none()
            && matches!(event, CommittedTriggerEvent::ZoneChanged { .. })
            && matches!(
                self.retired_objects.get(&listener.capture.source.object),
                Some(RetiredObject::Permanent { .. })
            )
        {
            return 0;
        }
        let causes = if let CommittedTriggerEvent::ObjectsDied { objects } = event {
            objects
                .iter()
                .filter(|object| {
                    self.trigger_event_matches_for_controller(
                        listener.event,
                        &CommittedTriggerEvent::ObjectsDied {
                            objects: vec![(*object).clone()],
                        },
                        listener.capture.source.object,
                        Some(listener.capture.controller),
                    )
                })
                .map(|object| CommittedTriggerEvent::ZoneChanged {
                    before: Some(object.clone()),
                    after: None,
                    from: ZoneKind::Battlefield,
                    to: ZoneKind::Graveyard,
                    damage_sources: Vec::new(),
                })
                .collect::<Vec<_>>()
        } else {
            vec![event.clone()]
        };
        let matches = |source, modification: &crate::card::TriggerModificationDef, cause| {
            self.trigger_event_matches_for_controller(
                modification.cause,
                cause,
                source,
                Some(listener.capture.controller),
            )
        };
        let causes = causes
            .iter()
            .filter(|cause| {
                !modifications.iter().any(|(source, modification)| {
                    modification.kind == crate::card::TriggerModificationKindDef::Suppress
                        && matches(*source, modification, cause)
                })
            })
            .collect::<Vec<_>>();
        if causes.is_empty() {
            return 0;
        }
        1 + modifications
            .iter()
            .filter(|(source, modification)| {
                modification.kind == crate::card::TriggerModificationKindDef::Additional
                    && causes
                        .iter()
                        .any(|cause| matches(*source, modification, cause))
            })
            .count()
    }
}
