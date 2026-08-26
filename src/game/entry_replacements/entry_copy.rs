//! Choosing copiable values as a permanent enters.

use super::super::{
    AbilityOrigin, CopiableAbility, Game, ObjectPredicateDef, PendingEvent, ReplaceableEvent,
};
use crate::card::CopyExceptionsDef;

impl Game {
    /// Offers the copy choice an entering permanent may make, or lets it enter
    /// as itself when there is nothing to copy.
    pub(super) fn offer_entry_copy(
        &mut self,
        pending: PendingEvent,
        object: ObjectPredicateDef,
        exceptions: CopyExceptionsDef,
        origin: AbilityOrigin,
    ) -> Option<PendingEvent> {
        let player = Self::pending_event_controller(&pending);
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        let entering = entry.permanent.card.id;
        let added_abilities = exceptions
            .added_abilities
            .iter()
            .filter_map(|ability| match ability {
                crate::card::CopyAbilityDef::This => None,
                crate::card::CopyAbilityDef::Ability(ability) => Some(CopiableAbility {
                    origin,
                    definition: **ability,
                }),
            })
            .collect();
        let choices = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.id != entering)
            .filter(|permanent| {
                self.trigger_object_matches(
                    object,
                    &self.trigger_event_object(permanent),
                    entering,
                    false,
                )
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        if choices.is_empty() {
            return Some(pending);
        }
        self.pending_events.push_front(pending);
        self.queue_entry_copy_choice(player, choices, exceptions, added_abilities);
        None
    }
}
