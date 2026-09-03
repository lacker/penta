use super::{
    CardInstance, Game, GameEvent, GameObjectId, ObjectPredicateDef, PlayerId, Target, ZoneKind,
    ZoneMoveCause, ZonePlacement,
};

impl Game {
    /// Lifts the top `count` cards off a library, fewer if it is short, in
    /// top-first order. A library keeps its top at the end, which is the end
    /// a draw takes from, so taking from the front would have handed back the
    /// bottom of the deck instead.
    ///
    /// Revealing them is informational only; nothing yet keys off having seen
    /// a card, so the mechanical effect is where they end up.
    pub(super) fn take_top_of_library(
        &mut self,
        player: PlayerId,
        count: usize,
    ) -> Vec<CardInstance> {
        let library = &mut self.players[player.index()].library;
        let taken = count.min(library.len());
        let remaining = library.len() - taken;
        let mut cards = library.split_off(remaining);
        cards.reverse();
        cards
    }

    /// Where the pile the controller did not take ends up. The library end
    /// that counts as the top is the back of the vector, so bottoming inserts
    /// at the front.
    pub(super) fn place_revealed_remainder(
        &mut self,
        player: PlayerId,
        cards: Vec<CardInstance>,
        zone: ZoneKind,
        placement: ZonePlacement,
    ) {
        match zone {
            ZoneKind::Hand => {
                for card in cards {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
            }
            ZoneKind::Library => {
                match placement {
                    // These arrive top-first, and the back of the vector is
                    // the top, so they go back on in reverse -- otherwise a
                    // group put back on top comes back inverted, which is
                    // wrong for anything that says "in the same order" and
                    // for a look that moved nothing at all.
                    ZonePlacement::Top => {
                        for card in cards.into_iter().rev() {
                            self.players[player.index()].library.push(card);
                        }
                    }
                    // A depth counts from the top, so a group put in at one
                    // goes in top-first for the same reason: each card is
                    // inserted below the one before it.
                    placement => {
                        for card in cards.into_iter().rev() {
                            let library = &mut self.players[player.index()].library;
                            let index = placement.library_index(library.len());
                            library.insert(index, card);
                        }
                    }
                }
            }
            ZoneKind::Exile => {
                for card in cards {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].exile.push(card);
                }
            }
            ZoneKind::Graveyard => self.bury_cards(player, cards),
            // Under the player who looked, which is what "you may put it
            // onto the battlefield" means -- the card is still theirs and
            // was never cast.
            ZoneKind::Battlefield => {
                for card in cards {
                    let _ = self.put_card_onto_battlefield_from(
                        card,
                        ZoneKind::Library,
                        super::BattlefieldArrival::under(player),
                        None,
                    );
                }
            }
            ZoneKind::Stack | ZoneKind::Command => {
                debug_assert!(false, "unsupported revealed-card destination: {zone:?}");
                self.bury_cards(player, cards);
            }
        }
    }

    /// Take from the top until one matches, publicly reveal every card, and
    /// move the passed cards plus the match to their printed destinations.
    ///
    /// The returned targets are the new identities of cards put into a
    /// graveyard, in reveal order. The count includes the matching card even
    /// when it goes somewhere else, and includes every card in a library that
    /// contains no match.
    pub(super) fn mill_until_matching(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        matched_zone: ZoneKind,
        source: GameObjectId,
    ) -> (Vec<Target>, u16) {
        let mut revealed = Vec::new();
        let mut matched_card = None;
        while let Some(card) = self.players[player.index()].library.pop() {
            if self.card_object_matches(predicate, &card, ZoneKind::Library, source) {
                matched_card = Some(card);
                break;
            }
            revealed.push(card);
        }
        let revealed_count = revealed
            .len()
            .saturating_add(usize::from(matched_card.is_some()));
        self.events
            .extend(revealed.iter().chain(matched_card.iter()).map(|card| {
                GameEvent::CardRevealed {
                    player,
                    card: card.id,
                    definition: card.definition,
                }
            }));
        // The match keeps its own destination; a library with nothing
        // matching found no match and buries everything it passed.
        match matched_card {
            Some(card) if matched_zone == ZoneKind::Graveyard => revealed.push(card),
            Some(card) => {
                self.place_revealed_remainder(player, vec![card], matched_zone, ZonePlacement::Top);
            }
            None => {}
        }
        let buried = self.bury_cards_with_ids(player, revealed);
        (buried, u16::try_from(revealed_count).unwrap_or(u16::MAX))
    }

    pub(super) fn bury_cards(&mut self, player: PlayerId, cards: Vec<CardInstance>) {
        let _ = self.bury_cards_with_ids(player, cards);
    }

    fn bury_cards_with_ids(&mut self, player: PlayerId, cards: Vec<CardInstance>) -> Vec<Target> {
        let mut buried = Vec::with_capacity(cards.len());
        for card in cards {
            if let Some(card) =
                self.put_card_into_graveyard_replacing(player, card, ZoneKind::Library)
            {
                buried.push(Target::Card(card.id));
            }
        }
        buried
    }

    /// Discards at random from the cards in hand that match, leaving the
    /// rest alone. A hand with nothing matching discards nothing, which is
    /// what "discards a creature card at random" does to a hand of lands.
    pub(super) fn discard_random_matching(
        &mut self,
        player: PlayerId,
        count: u16,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        cause: ZoneMoveCause,
    ) {
        let mut matching: Vec<_> = self.players[player.index()]
            .hand
            .iter()
            .filter(|card| self.card_object_matches(predicate, card, ZoneKind::Hand, source))
            .map(|card| card.id)
            .collect();
        let mut discarded = Vec::new();
        for _ in 0..usize::from(count).min(matching.len()) {
            let index = self.rng.index_below(matching.len());
            discarded.push(matching.swap_remove(index));
        }
        self.discard_cards_with_cause(player, &discarded, cause);
    }

    pub(super) fn discard_random(&mut self, player: PlayerId, count: u16, cause: ZoneMoveCause) {
        self.rng.shuffle(&mut self.players[player.index()].hand);
        let hand_count = u16::try_from(self.players[player.index()].hand.len()).unwrap_or(u16::MAX);
        let discard_count = count.min(hand_count);
        let discarded = self.players[player.index()]
            .hand
            .iter()
            .rev()
            .take(usize::from(discard_count))
            .map(|card| card.id)
            .collect::<Vec<_>>();
        self.discard_cards_with_cause(player, &discarded, cause);
    }
}
