//! Plot (CR 702.170a).
//!
//! Two halves that meet in exile, the same shape foretell has and the mirror
//! of its economics: the plot cost is paid up front to a special action, and
//! what it buys is a free cast on a later turn. The card lies face up in
//! exile, so both players can see what is coming.
//!
//! Only the first half lives here. The second is an ordinary free permission
//! to cast from exile, which is why nothing in the casting path knows the
//! word "plot" at all. The permission does not carry a sorcery-speed
//! restriction of its own: every card that prints the keyword so far is a
//! sorcery, and its type already says so.

use crate::ids::GameObjectId;

use super::{
    Action, AlternativeCastKindDef, DeclarativeAbilityDef, Game, ManaCost, ManaPaymentPurpose,
    PlayerId,
};

impl Game {
    /// The plot cost this card prints, which is what makes the special
    /// action available for it at all.
    fn card_plot_cost(&self, definition: crate::ids::CardDefinitionId) -> Option<ManaCost> {
        self.catalog.get(definition).and_then(|card| {
            card.parts.iter().find_map(|part| {
                part.rules.ability_clauses().iter().find_map(|ability| {
                    let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition
                    else {
                        return None;
                    };
                    (alternative.kind == AlternativeCastKindDef::Plot)
                        .then(|| alternative.mana_cost.resolve(None))
                        .flatten()
                })
            })
        })
    }

    /// "Plot only as a sorcery": your own main phase with the stack empty,
    /// whatever the plotted card's own type would allow.
    pub(super) fn add_plot_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        if player != self.active_player || !self.step.is_main() || !self.stack.is_empty() {
            return;
        }
        actions.extend(
            self.players[player.index()]
                .hand
                .iter()
                .filter(|card| {
                    self.card_plot_cost(card.definition).is_some_and(|cost| {
                        self.can_pay_cost_for(player, cost, 0, &ManaPaymentPurpose::Other)
                    })
                })
                .map(|card| Action::Plot { card: card.id }),
        );
    }

    pub(super) fn plot(&mut self, player: PlayerId, card: GameObjectId) {
        let Some(cost) = self.players[player.index()]
            .hand
            .iter()
            .find(|candidate| candidate.id == card)
            .and_then(|candidate| self.card_plot_cost(candidate.definition))
        else {
            return;
        };
        self.activate_mana_for_cost(player, cost, 0);
        let _spent = self.pay_player_cost(player, cost, 0);
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
        self.permit_plotted_cast(exiled, player);
    }
}
