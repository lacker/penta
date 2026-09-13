//! Plot (CR 702.170a).
//!
//! The hand action and effects both mark an ordinary exile object as plotted.
//! That designation, independent of its printed abilities, supplies CR 702.170d's
//! cast permission. Suspend instead derives its status from exile, time counters,
//! and the suspend ability; rebound retains an object-linked delayed trigger.

use crate::ids::GameObjectId;

use super::{Action, AlternativeCastKindDef, DeclarativeAbilityDef, Game, PlayerId};

impl Game {
    /// The plot cost this card prints, which is what makes the special
    /// action available for it at all.
    pub(in crate::game) fn card_plot_cost(
        &self,
        definition: crate::ids::CardDefinitionId,
    ) -> Option<&'static [crate::CostDef]> {
        self.catalog.get(definition).and_then(|card| {
            card.parts.iter().find_map(|part| {
                part.rules.ability_clauses().iter().find_map(|ability| {
                    let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition
                    else {
                        return None;
                    };
                    (alternative.kind == AlternativeCastKindDef::Plot).then_some(alternative.costs)
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
                    self.can_pay_special_action(
                        player,
                        card.id,
                        super::special_action_payments::PaidSpecialAction::Plot,
                    )
                })
                .map(|card| Action::Plot { card: card.id }),
        );
    }

    pub(super) fn plot(&mut self, player: PlayerId, card: GameObjectId) {
        self.begin_special_action_payment(
            player,
            card,
            super::special_action_payments::PaidSpecialAction::Plot,
        );
    }

    pub(in crate::game) fn finish_plot(&mut self, player: PlayerId, card: GameObjectId) {
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
        self.players[owner.index()].exile.push(moved.clone());
        self.capture_cards_exiled(std::slice::from_ref(&moved), crate::card::ZoneKind::Hand);
        self.make_plotted(exiled);
    }
}

impl Game {
    /// Becoming plotted is not performing the plot special action (CR 702.170e).
    pub(super) fn make_plotted(&mut self, card: GameObjectId) {
        if self
            .players
            .iter()
            .any(|state| state.exile.iter().any(|exiled| exiled.id == card))
        {
            self.plotted_cards.insert(
                card,
                (
                    self.active_player,
                    self.turns_started[self.active_player.index()],
                ),
            );
        }
    }

    pub(super) fn plotted_cast_permission(
        &self,
        card: GameObjectId,
        player: PlayerId,
    ) -> Option<super::ExilePlayPermission> {
        let &(active, turn) = self.plotted_cards.get(&card)?;
        if !self.sorcery_speed_window(player)
            || (self.active_player == active && self.turns_started[active.index()] == turn)
            || !self.players[player.index()]
                .exile
                .iter()
                .any(|exiled| exiled.id == card && exiled.owner == player)
        {
            return None;
        }
        Some(super::ExilePlayPermission {
            card,
            player,
            cost: super::ExilePlayCost::Free,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: crate::card::ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            lands_may_be_played: false,
            hidden_from_owner: false,
            spend_any_color: false,
            condition: None,
            hidden_only: false,
            until_holder_end_step: None,
            zone: crate::card::ZoneKind::Exile,
            group: None,
            grants_haste: false,
        })
    }
}
