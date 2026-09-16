//! A search that spans more than one zone and exiles what it leaves behind.
//!
//! Doomsday is the shape: your library and graveyard are searched as one,
//! five cards go on top of the library in the order you picked them, and
//! everything else in both zones is exiled.

use super::{
    DecisionContinuation, DecisionPreference, DecisionVisibility, DecisionZone, Game, GameObjectId,
    PlayerId, ZoneKind, remove_card,
};

impl Game {
    /// Offers every card in the named zones as one search.
    pub(super) fn queue_search_zones_and_exile_rest(
        &mut self,
        player: PlayerId,
        zones: &[ZoneKind],
        count: usize,
    ) {
        if zones.contains(&ZoneKind::Library) {
            self.capture_battlefield_triggers(&super::CommittedTriggerEvent::LibrarySearched {
                player,
                owner: player,
            });
        }
        let mut options = Vec::new();
        for zone in zones {
            let decision_zone = match zone {
                ZoneKind::Library => DecisionZone::Library,
                ZoneKind::Graveyard => DecisionZone::Graveyard,
                ZoneKind::Hand => DecisionZone::Hand,
                ZoneKind::Exile => DecisionZone::Exile,
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => continue,
            };
            let cards = self.cards_in_searched_zone(player, *zone).to_vec();
            let offered = self.card_decision_options(&cards, decision_zone);
            options.extend(offered);
        }
        // Ids are positional within one call, so a second zone's options
        // would repeat the first's numbering without this.
        for (index, option) in options.iter_mut().enumerate() {
            option.id = u32::try_from(index).unwrap_or(u32::MAX);
        }
        // What "the rest" means is fixed here, before anybody answers. The
        // spell doing the searching reaches the graveyard while the decision
        // is still open, and it was never part of the search.
        let searched = options
            .iter()
            .filter_map(|option| option.card.map(|(card, _)| card))
            .collect::<Vec<_>>();
        // Fewer cards than the card asks for is not a failed search: you
        // take what there is, and the rest of the instruction still happens.
        let taken = count.min(options.len());
        if options.is_empty() {
            return;
        }
        self.queue_decision(
            player,
            "Choose cards to keep, in the order they will be drawn",
            DecisionVisibility::Private,
            DecisionPreference::HigherCardValue,
            taken..=taken,
            false,
            options,
            DecisionContinuation::SearchZonesAndExileRest {
                player,
                zones: zones.to_vec(),
                searched,
            },
        );
    }

    fn cards_in_searched_zone(&self, player: PlayerId, zone: ZoneKind) -> &[super::CardInstance] {
        match zone {
            ZoneKind::Library => &self.players[player.index()].library,
            ZoneKind::Graveyard => &self.players[player.index()].graveyard,
            ZoneKind::Hand => &self.players[player.index()].hand,
            ZoneKind::Exile => &self.players[player.index()].exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => &[],
        }
    }

    /// Puts the chosen cards on top in the order they were chosen -- the
    /// first one picked is the one drawn first -- and exiles everything the
    /// search left in the zones it looked through.
    pub(super) fn finish_search_zones_and_exile_rest(
        &mut self,
        player: PlayerId,
        zones: &[ZoneKind],
        searched: &[GameObjectId],
        chosen: &[GameObjectId],
    ) {
        let mut kept = Vec::new();
        for id in chosen {
            for zone in zones {
                let cards = match zone {
                    ZoneKind::Library => &mut self.players[player.index()].library,
                    ZoneKind::Graveyard => &mut self.players[player.index()].graveyard,
                    ZoneKind::Hand => &mut self.players[player.index()].hand,
                    ZoneKind::Exile => &mut self.players[player.index()].exile,
                    ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => continue,
                };
                if let Some(card) = remove_card(cards, *id) {
                    kept.push(card);
                    break;
                }
            }
        }
        // Everything the search looked at and did not take is exiled, which
        // is what makes the five cards the whole deck.
        for id in searched {
            if chosen.contains(id) {
                continue;
            }
            for zone in zones {
                let cards = match zone {
                    ZoneKind::Library => &mut self.players[player.index()].library,
                    ZoneKind::Graveyard => &mut self.players[player.index()].graveyard,
                    ZoneKind::Hand => &mut self.players[player.index()].hand,
                    ZoneKind::Battlefield
                    | ZoneKind::Stack
                    | ZoneKind::Command
                    | ZoneKind::Exile => continue,
                };
                if let Some(card) = remove_card(cards, *id) {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].exile.push(card);
                    break;
                }
            }
        }
        // The library is stored bottom-first, so the card chosen first has
        // to end up last: it is the one drawn next.
        for card in kept.into_iter().rev() {
            self.players[player.index()].library.push(card);
        }
    }
}
