//! Foretell (CR 702.143a).
//!
//! Two halves that meet in exile. The first is a special action: during your
//! turn, pay {2} and exile a card from your hand face down. The second is
//! the alternative cast the card prints, taken from exile on a later turn.
//!
//! Only the first half lives here. The second is an ordinary alternative
//! cast, found where every other one is -- the difference is only that the
//! card it is printed on is sitting in exile rather than in a hand.

use crate::card::ManaCost;
use crate::ids::GameObjectId;

use super::{
    Action, AlternativeCastKindDef, DeclarativeAbilityDef, Game, ManaPaymentPurpose, PlayerId,
};

/// What exiling a card face down costs. The same {2} for every card that
/// prints the keyword, which is why no card writes it down.
const FORETELL_COST: ManaCost = crate::mana_cost!("{2}");

impl Game {
    /// Whether this card prints a foretell cost, which is what makes the
    /// special action available for it at all.
    fn card_has_foretell(&self, definition: crate::ids::CardDefinitionId) -> bool {
        self.catalog.get(definition).is_some_and(|card| {
            card.parts.iter().any(|part| {
                part.rules.ability_clauses().iter().any(|ability| {
                    matches!(
                        ability.definition,
                        DeclarativeAbilityDef::AlternativeCast(alternative)
                            if alternative.kind == AlternativeCastKindDef::Foretell
                    )
                })
            })
        })
    }

    /// "Any time you have priority during your turn" (CR 702.143a): the
    /// reminder text says only "during your turn", and the rule means that
    /// literally. Not a sorcery window -- a foretell can be taken in
    /// response to a spell, or in the middle of your own combat.
    pub(super) fn add_foretell_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        if player != self.active_player {
            return;
        }
        if !self.can_pay_cost_for(player, FORETELL_COST, 0, &ManaPaymentPurpose::Other) {
            return;
        }
        actions.extend(
            self.players[player.index()]
                .hand
                .iter()
                .filter(|card| self.card_has_foretell(card.definition))
                .map(|card| Action::Foretell { card: card.id }),
        );
    }

    pub(super) fn foretell(&mut self, player: PlayerId, card: GameObjectId) {
        if !self.players[player.index()]
            .hand
            .iter()
            .any(|candidate| candidate.id == card)
        {
            return;
        }
        self.activate_mana_for_cost(player, FORETELL_COST, 0);
        let _spent = self.pay_player_cost(player, FORETELL_COST, 0);
        let Some(index) = self.players[player.index()]
            .hand
            .iter()
            .position(|candidate| candidate.id == card)
        else {
            return;
        };
        let moved = self.players[player.index()].hand.remove(index);
        let owner = moved.owner;
        // A zone change mints a new object, and the permission has to name
        // the card that ended up in exile rather than the one that left the
        // hand.
        let (moved, _zone_change) = self.zone_change_card(moved);
        let exiled = moved.id;
        self.players[owner.index()].exile.push(moved);
        self.permit_foretold_cast(exiled, player);
    }
}
