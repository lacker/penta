//! Explore (CR 701.40a).
//!
//! Reveal the top card of that creature's controller's library. Put it into
//! their hand if it is a land card. Otherwise put a +1/+1 counter on the
//! creature, then put the card back on top of the library or into the
//! graveyard, at that player's choice.
//!
//! A procedure of its own rather than a composition: what happens to the
//! revealed card and whether the creature grows both turn on a card type
//! nobody knows until the card is revealed, and the branch that does not
//! take it ends in a choice.

use crate::card::{CardType, CounterKind};
use crate::ids::{CardPartId, GameObjectId};

use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    Game, GameEvent, ObjectCharacteristics, PlayerId,
};

impl Game {
    /// One creature explores. A creature that is not on the battlefield any
    /// more explores nothing, and an empty library reveals nothing.
    pub(super) fn explore(&mut self, creature: GameObjectId) {
        let Some(player) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature)
            .map(|permanent| permanent.controller)
        else {
            return;
        };
        let Some(top) = self.players[player.index()].library.last() else {
            return;
        };
        let (revealed, definition) = (top.id, top.definition);
        self.events.push(GameEvent::CardRevealed {
            player,
            card: revealed,
            definition,
        });
        let is_land = self
            .catalog
            .get(definition)
            .is_some_and(|card| card.rules.has_type(CardType::Land));
        if is_land {
            if let Some(card) = self.players[player.index()].library.pop() {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[player.index()].hand.push(card);
            }
            return;
        }
        // The counter goes on before the card is placed, which is the order
        // the keyword action prints and the order a trigger reading the
        // creature's power would see.
        self.add_explore_counter(creature);
        let name = self
            .catalog
            .get(definition)
            .map_or_else(|| "that card".to_owned(), |card| card.name.clone());
        let presentation = ObjectCharacteristics::card(definition, CardPartId::PRIMARY);
        self.queue_decision(
            player,
            format!("Put {name} back on top of your library or into your graveyard"),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Top of library".into(),
                    card: Some((revealed, presentation)),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Library,
                },
                DecisionOption {
                    id: 1,
                    label: "Graveyard".into(),
                    card: Some((revealed, presentation)),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Library,
                },
            ],
            DecisionContinuation::ExploredCardPlacement { player, revealed },
        );
    }

    fn add_explore_counter(&mut self, creature: GameObjectId) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == creature)
        {
            permanent.add_counters(CounterKind::PlusOnePlusOne, 1);
        }
        self.capture_counters_placed(&[creature], CounterKind::PlusOnePlusOne, 1);
    }

    /// The answer to "top of library or graveyard". Leaving it on top is the
    /// half that moves nothing.
    pub(super) fn place_explored_card(
        &mut self,
        player: PlayerId,
        revealed: GameObjectId,
        bury: bool,
    ) {
        if !bury {
            return;
        }
        let library = &mut self.players[player.index()].library;
        let Some(index) = library.iter().position(|card| card.id == revealed) else {
            return;
        };
        let card = library.remove(index);
        let (card, _zone_change) = self.zone_change_card(card);
        self.players[player.index()].graveyard.push(card);
    }
}
