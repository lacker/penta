// Current and last-known object characteristics. Included from lifecycle.rs.

impl Game {
    pub(super) fn current_or_last_known_power(&self, object: GameObjectId) -> Option<i16> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| self.power(permanent))
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|stack| stack.id == object)
                    .and_then(|stack| self.stack_trigger_event_object(stack)?.power)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { power, .. }) => *power,
                // A card that was never a permanent still has a power to
                // read, and nothing in a graveyard or exile modifies it, so
                // the printed value is the whole answer. Scavenge asks this
                // of a card it has already exiled to pay its own cost.
                Some(RetiredObject::Card(card)) => card.stats.map(|stats| stats.power),
                Some(RetiredObject::Stack(stack)) => self
                    .stack_trigger_event_object(stack)
                    .and_then(|view| view.power),
                None => self
                    .stack
                    .iter()
                    .find(|candidate| candidate.id == object)
                    .and_then(|stack| self.stack_trigger_event_object(stack))
                    .and_then(|view| view.power)
                    .or_else(|| {
                        self.card_in_nonbattlefield_zone(object)
                            .and_then(|(zone, card)| self.printed_card_power(card, Some(zone)))
                    }),
            })
    }

    /// A card's power outside the battlefield: what its corner prints,
    /// unless it prints a characteristic-defining ability instead, which
    /// functions in every zone (CR 604.3).
    fn printed_card_power(&self, card: &CardInstance, zone: Option<ZoneKind>) -> Option<i16> {
        self.printed_card_stats(card, zone).map(|stats| stats.power)
    }

    /// The mirror of [`Self::printed_card_power`]. A Lhurgoyf in a graveyard
    /// has the toughness its own text gives it there.
    fn printed_card_toughness(&self, card: &CardInstance, zone: Option<ZoneKind>) -> Option<i16> {
        self.printed_card_stats(card, zone)
            .map(|stats| stats.toughness)
    }

    /// Outside the battlefield nobody controls a card, so its owner is who
    /// "you" means to any amount its own text reads (CR 108.4).
    fn printed_card_stats(
        &self,
        card: &CardInstance,
        zone: Option<ZoneKind>,
    ) -> Option<crate::CreatureStats> {
        let definition = self.catalog.get(card.definition)?;
        // What the card says it is where it is comes first: a planeswalker
        // card that is a 1/1 Insect in a graveyard has a body there and
        // nothing in its corner to read it from.
        let printed = zone
            .and_then(|zone| Self::card_zone_stats(definition, zone))
            .or_else(|| definition.rules.creature_stats())?;
        Some(
            self.card_defined_stats(definition, card.id, card.owner)
                .over(printed),
        )
    }

    /// The object an Aura was attached to immediately before it left the
    /// battlefield. Activated abilities are independent of their source once
    /// on the stack, so sacrificing the Aura as a cost or removing it in
    /// response must not erase what "enchanted permanent" means.
    pub(super) fn current_or_last_known_attached_host(
        &self,
        object: GameObjectId,
    ) -> Option<GameObjectId> {
        self.attached_host(object)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => permanent.attached_to,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// The player an Aura enchanted immediately before it left the
    /// battlefield. A triggered ability remains independent of its source,
    /// so removing the Curse in response must not erase who it enchanted.
    pub(super) fn current_or_last_known_enchanted_player(
        &self,
        object: GameObjectId,
    ) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| permanent.attached_player)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => permanent.attached_player,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// What a permanent was blocking, using last-known information once it
    /// has left the battlefield. A creature that died in combat still knows
    /// what it had blocked, which is what a death trigger reading "creatures
    /// blocked by it" has to see.
    pub(super) fn current_or_last_known_blocking(
        &self,
        object: GameObjectId,
    ) -> Option<GameObjectId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| permanent.blocking.first().copied())
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    permanent.blocking.first().copied()
                }
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// How many counters of one kind an object has, using last-known
    /// information once it has left the battlefield. An ability whose cost
    /// sacrificed its own source still reads the counters it had.
    pub(super) fn current_or_last_known_counters(
        &self,
        object: GameObjectId,
        kind: CounterKind,
    ) -> u16 {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        {
            return permanent.counters(kind);
        }
        // A card outside the battlefield can carry counters too: suspend's
        // time counters sit on a card in exile, and so does the silver
        // counter that says which exiled cards Karn may take back.
        if let Some((_, card)) = self.card_in_nonbattlefield_zone(object) {
            return card.counters(kind);
        }
        match self.retired_objects.get(&object) {
            Some(RetiredObject::Permanent { permanent, .. }) => permanent.counters(kind),
            Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => 0,
        }
    }

    /// Whether an object is tapped, using its last existence on the
    /// battlefield after it has left. Intervening-if conditions re-read this
    /// information as their abilities resolve.
    pub(super) fn current_or_last_known_tapped(&self, object: GameObjectId) -> bool {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map_or_else(
                || match self.retired_objects.get(&object) {
                    Some(RetiredObject::Permanent { permanent, .. }) => permanent.tapped,
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => false,
                },
                |permanent| permanent.tapped,
            )
    }

    pub(super) fn current_or_last_known_toughness(&self, object: GameObjectId) -> Option<i16> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| self.toughness(permanent))
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|stack| stack.id == object)
                    .and_then(|stack| self.stack_trigger_event_object(stack)?.toughness)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { toughness, .. }) => *toughness,
                Some(RetiredObject::Card(card)) => card.stats.map(|stats| stats.toughness),
                Some(RetiredObject::Stack(stack)) => {
                    self.stack_trigger_event_object(stack)?.toughness
                }
                None => self
                    .card_in_nonbattlefield_zone(object)
                    .and_then(|(zone, card)| self.printed_card_toughness(card, Some(zone))),
            })
    }

    pub(super) fn current_or_last_known_controller(
        &self,
        object: GameObjectId,
    ) -> Option<PlayerId> {
        self.battlefield
            .iter()
            // Phasing preserves this incarnation and its last controller.
            .chain(self.phased_out.iter())
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.controller)
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|candidate| candidate.id == object)
                    .map(|candidate| candidate.controller)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.controller),
                Some(RetiredObject::Stack(stack)) => Some(stack.controller),
                Some(RetiredObject::Card(_)) | None => None,
            })
    }

    pub(super) fn current_or_last_known_owner(&self, object: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.card.owner)
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|candidate| candidate.id == object)
                    .map(|candidate| candidate.card.owner)
            })
            .or_else(|| {
                self.card_in_nonbattlefield_zone(object)
                    .map(|(_, card)| card.owner)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Card(card)) => Some(card.owner),
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.card.owner),
                Some(RetiredObject::Stack(stack)) => Some(stack.card.owner),
                None => None,
            })
    }
}
