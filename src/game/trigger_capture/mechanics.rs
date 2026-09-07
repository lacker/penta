impl Game {
    /// Wrapping a trigger in a condition or an OR does not lose the named
    /// action's aggregation boundary.
    fn mechanic_batch_occurrence(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
        controller: PlayerId,
    ) -> Option<(crate::ids::MechanicId, PlayerId)> {
        match definition {
            TriggerEventDef::MechanicPerformed { mechanic, one_or_more: true, .. } => {
                let CommittedTriggerEvent::MechanicPerformed { player, .. } = event else {
                    return None;
                };
                self.trigger_event_matches_for_controller(definition, event, source, Some(controller))
                    .then_some((mechanic, *player))
            }
            TriggerEventDef::While { event: definition, .. } => {
                self.mechanic_batch_occurrence(*definition, event, source, controller)
            }
            TriggerEventDef::AnyOf(definitions) => definitions.iter().find_map(|definition| {
                self.mechanic_batch_occurrence(*definition, event, source, controller)
            }),
            _ => None,
        }
    }
}
