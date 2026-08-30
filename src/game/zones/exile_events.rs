#[cfg(test)]
use super::GameObjectId;
use super::{CardInstance, CharacteristicContext, CommittedTriggerEvent, Game, ZoneKind};

impl Game {
    /// "Whenever one or more cards are put into exile from ...": one event
    /// for the whole move, published once however many cards it took.
    ///
    /// Raised only where the cards came out of a hidden or public zone that
    /// a clause can name -- a permanent exiled from the battlefield is a
    /// zone change of its own and is published there.
    pub(in crate::game) fn capture_cards_exiled(&mut self, cards: &[CardInstance], from: ZoneKind) {
        let Some(owner) = cards.first().map(|card| card.owner) else {
            return;
        };
        let objects = cards
            .iter()
            .filter_map(|card| {
                self.printed_trigger_event_object(
                    card.id,
                    card.definition,
                    card.owner,
                    &CharacteristicContext::Exile,
                )
            })
            .collect::<Vec<_>>();
        if objects.is_empty() {
            return;
        }
        let source_context = match from {
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return,
        };
        for card in cards {
            let Some(previous) = self
                .successors
                .iter()
                .find_map(|(previous, successor)| (*successor == card.id).then_some(*previous))
            else {
                continue;
            };
            let Some(before) = self.printed_trigger_event_object(
                previous,
                card.definition,
                card.owner,
                &source_context,
            ) else {
                continue;
            };
            let Some(after) = self.printed_trigger_event_object(
                card.id,
                card.definition,
                card.owner,
                &CharacteristicContext::Exile,
            ) else {
                continue;
            };
            self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
                before: Some(before),
                after: Some(after),
                from,
                to: ZoneKind::Exile,
                damage_sources: Vec::new(),
            });
        }
        self.capture_battlefield_triggers(&CommittedTriggerEvent::CardsExiled {
            cards: objects,
            from,
            owner,
        });
    }

    /// Raises the exile event for cards already sitting in exile, for the
    /// tests that need one without an effect to make it.
    #[cfg(test)]
    pub(in crate::game) fn capture_exile_for_test(
        &mut self,
        cards: &[GameObjectId],
        from: ZoneKind,
    ) {
        let moved = cards
            .iter()
            .filter_map(|id| {
                self.card_in_nonbattlefield_zone(*id)
                    .map(|(_, card)| card.clone())
            })
            .collect::<Vec<_>>();
        self.capture_cards_exiled(&moved, from);
    }
}
