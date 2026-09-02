//! The monarch (CR 720).
//!
//! There is at most one, and holding the crown does three things: it draws
//! its holder a card at the beginning of their end step, it is taken by
//! whoever deals combat damage to the current monarch, and cards can ask
//! about it. Nobody is the monarch until something says so.
//!
//! Audit: unsupported — CR 720.4 and 720.5 are two inherent *triggered*
//! abilities with no source, controlled by whoever was the monarch when they
//! triggered. Both are carried out here as rules the moment they apply, and
//! a trigger needs a permanent or an emblem to hang on. Nothing in a
//! two-player game changes hands between the trigger and its resolution on
//! its own, so the outcomes match; what is missing is the window in between,
//! where the draw could be answered -- countered by a Stifle, or kept by the
//! player who was the monarch when a later spell took the crown off them.

use super::{CommittedTriggerEvent, Game, GameObjectId, PlayerId};

impl Game {
    /// Who holds the crown, if anyone does.
    #[must_use]
    pub const fn monarch(&self) -> Option<PlayerId> {
        self.monarch
    }

    /// Hands the crown to `player`. A player who already has it keeps it and
    /// nothing is raised: the crown did not change hands, so nothing that
    /// watches for it changing hands should fire.
    pub(super) fn set_monarch(&mut self, player: PlayerId) {
        if self.monarch == Some(player) {
            return;
        }
        self.monarch = Some(player);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::BecameMonarch { player });
    }

    /// "Whenever a creature deals combat damage to the monarch, that
    /// creature's controller becomes the monarch" (CR 720.5). Read where the
    /// damage lands rather than put on the stack; see the module note.
    pub(super) fn combat_damage_may_steal_the_crown(
        &mut self,
        source: Option<GameObjectId>,
        damaged: PlayerId,
        amount: u16,
    ) {
        if amount == 0 || self.monarch != Some(damaged) {
            return;
        }
        let Some(controller) = source.and_then(|source| {
            self.battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .map(|permanent| permanent.controller)
        }) else {
            return;
        };
        self.set_monarch(controller);
    }

    /// "At the beginning of the monarch's end step, that player draws a
    /// card" (CR 720.4). Carried out as the step begins rather than put on
    /// the stack; see the module note.
    pub(super) fn monarch_draws_at_end_step(&mut self) {
        if self.monarch == Some(self.active_player) {
            self.draw_instruction(self.active_player, 1);
        }
    }
}
