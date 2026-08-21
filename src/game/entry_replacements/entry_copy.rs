//! Choosing copiable values as a permanent enters.

use super::super::{
    CardTypeSet, CopiableAbility, Game, ObjectCharacteristics, ObjectKind, ObjectPredicateDef,
    PendingEvent, ReplaceableEvent,
};

impl Game {
    /// Offers the copy choice an entering permanent may make, or lets it enter
    /// as itself when there is nothing to copy.
    pub(super) fn offer_entry_copy(
        &mut self,
        pending: PendingEvent,
        object: ObjectPredicateDef,
        added_types: CardTypeSet,
        retain_printed_subtypes: bool,
        retained_abilities: &'static [crate::AbilityId],
    ) -> Option<PendingEvent> {
        let player = Self::pending_event_controller(&pending);
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        let entering = entry.permanent.card.id;
        // Read off the physical card or authored token that is entering,
        // which is where an "except it has ..." clause lives. A copy takes
        // the other permanent's abilities wholesale, so this is how the
        // entering object's own authored abilities come back.
        let part = entry.permanent.presented;
        let authored = match entry.permanent.card.definition {
            ObjectKind::Card(definition) => Some(ObjectCharacteristics::card(definition, part)),
            ObjectKind::Token => entry
                .permanent
                .token_characteristics
                .map(|token| ObjectCharacteristics::token(token, part)),
            ObjectKind::Emblem | ObjectKind::Ability => None,
        };
        let kept = authored
            .and_then(|authored| {
                let rules = match authored {
                    ObjectCharacteristics::Card { definition, part } => {
                        self.catalog.get(definition)?.part(part)?.rules
                    }
                    ObjectCharacteristics::Token { token, part } => token.part(part)?.rules,
                    ObjectCharacteristics::Emblem { .. } => return None,
                };
                Some(
                    retained_abilities
                        .iter()
                        .filter_map(|id| {
                            Some(CopiableAbility {
                                origin: Self::authored_ability_origin(authored, *id),
                                definition: *rules.ability(*id)?,
                            })
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default();
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
        self.queue_entry_copy_choice(player, choices, added_types, retain_printed_subtypes, kept);
        None
    }
}
