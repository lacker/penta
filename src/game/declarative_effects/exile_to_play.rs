//! Exiling the top of a library with a permission to play what was exiled.
//!
//! Split out of `hand_and_library` for the source-size budget.

use crate::card::{ExilePlayConditionDef, ExilePlayDurationDef, ZoneKind};

use super::super::{Game, PlayerId};

/// What a permission granted over exiled cards allows, gathered so the
/// clause that grants it stays one call.
#[derive(Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct ExilePlayGrant {
    pub(super) free: bool,
    pub(super) face_down: bool,
    pub(super) duration: ExilePlayDurationDef,
    pub(super) mana_spending: Option<crate::card::ManaSpendAsDef>,
    pub(super) play_condition: Option<ExilePlayConditionDef>,
    pub(super) cast_only: bool,
}

impl Game {
    /// "That player exiles the top N cards of their library", with the
    /// permission the clause names granted over each of them.
    pub(super) fn exile_top_of_library_to_play(
        &mut self,
        player: PlayerId,
        count: usize,
        controller: PlayerId,
        grant: ExilePlayGrant,
    ) {
        let mut moved = Vec::new();
        for card in self.take_top_of_library(player, count) {
            let (card, _zone_change) = self.zone_change_card(card);
            let exiled = card.id;
            self.players[player.index()].exile.push(card.clone());
            moved.push(card);
            match (grant.free, grant.face_down, grant.duration) {
                (true, _, _) => self.permit_free_play_this_turn(exiled, controller),
                (false, true, ExilePlayDurationDef::ThisTurn) => {
                    self.permit_face_down_play_this_turn(exiled, controller);
                }
                (false, false, ExilePlayDurationDef::ThisTurn) => {
                    self.permit_cast_this_turn(exiled, controller);
                }
                (false, _, ExilePlayDurationDef::UntilYourNextEndStep) => {
                    self.permit_play_until_your_next_end_step(exiled, controller);
                }
                (false, _, ExilePlayDurationDef::UntilEndOfYourNextTurn) => {
                    self.permit_play_until_end_of_your_next_turn(exiled, controller);
                }
                // Bounded by the exile rather than by a turn: what limits it
                // is whatever the clause asks for each time it is played.
                (false, _, ExilePlayDurationDef::WhileExiled) => {
                    self.permit_conditional_cast_while_exiled(exiled, controller);
                }
            }
            if grant.face_down {
                let permission = self
                    .exile_play_permissions
                    .last_mut()
                    .expect("play grant just installed");
                permission.face_down = true;
                // The grantee is the player allowed to look; this does not expose
                // another owner's cards merely because they own the exile zone.
                permission.hidden_from_owner = false;
            }
            if grant.cast_only {
                self.restrict_exile_permission_to_casting(exiled);
            }
            if grant.mana_spending.is_some() || grant.play_condition.is_some() {
                self.qualify_exile_permission(
                    exiled,
                    grant.mana_spending.is_some(),
                    grant.play_condition,
                );
                if let Some(permission) = self.exile_play_permissions.last_mut() {
                    permission.spend_any_type =
                        grant.mana_spending == Some(crate::card::ManaSpendAsDef::AnyType);
                }
            }
        }
        self.capture_cards_exiled(&moved, ZoneKind::Library);
    }
}
