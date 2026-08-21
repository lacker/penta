//! Face-down permanents: casting one, and turning it face up.
//!
//! A face-down permanent's body lives in `characteristics`; this module owns
//! the one action that changes the state. Turning a permanent face up is a
//! special action (CR 702.37b): it uses no stack, nobody may respond to it,
//! and it is available even though the permanent has no abilities at all
//! while it is face down.

use super::{Action, Game, GameObjectId, ManaPaymentPurpose, PlayerId};

impl Game {
    /// The morph cost printed on the physical card under a permanent, if it
    /// has one. Read off `card.definition` rather than the presented rules,
    /// which while face down are the body's and carry nothing.
    pub(super) fn printed_morph_cost(
        &self,
        permanent: &super::Permanent,
    ) -> Option<crate::card::ManaCost> {
        self.catalog
            .get(permanent.card.definition.card_definition()?)?
            .part(permanent.presented)?
            .rules
            .morph_cost()
    }

    /// What turning this permanent face up costs, or `None` when nothing
    /// can. A morph pays what its card prints as a morph cost; a manifested
    /// permanent pays the card's own mana cost, and only if the card under
    /// it is a creature card (CR 701.34c).
    pub(super) fn face_up_cost(
        &self,
        permanent: &super::Permanent,
    ) -> Option<crate::card::ManaCost> {
        if let Some(cost) = self.printed_morph_cost(permanent) {
            return Some(cost);
        }
        if !permanent.manifested {
            return None;
        }
        let part = self
            .catalog
            .get(permanent.card.definition.card_definition()?)?
            .part(permanent.presented)?;
        part.rules
            .has_type(crate::card::CardType::Creature)
            .then(|| part.rules.mana_cost())
            .flatten()
    }

    pub(super) fn add_face_up_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.face_down && permanent.controller == player)
        {
            let Some(cost) = self.face_up_cost(permanent) else {
                continue;
            };
            if self.can_pay_cost_for(player, cost, 0, &ManaPaymentPurpose::Other) {
                actions.push(Action::TurnFaceUp {
                    permanent: permanent.card.id,
                });
            }
        }
    }

    pub(super) fn turn_face_up(&mut self, player: PlayerId, permanent: GameObjectId) {
        let Some(cost) = self
            .battlefield
            .iter()
            .find(|candidate| candidate.card.id == permanent)
            .and_then(|candidate| self.face_up_cost(candidate))
        else {
            return;
        };
        self.activate_mana_for_cost(player, cost, 0);
        let _spent = self.pay_player_cost(player, cost, 0);
        // Turning face up is not a zone change and creates no new object, so
        // the permanent keeps its identity, its counters, and its damage. It
        // simply stops presenting the body.
        if let Some(target) = self
            .battlefield
            .iter_mut()
            .find(|candidate| candidate.card.id == permanent)
        {
            target.face_down = false;
        }
        // Nothing in the staged tranche triggers on a permanent being
        // turned face up, so there is no event to raise yet; the state
        // change alone is what the morph cards here need.
        let _ = permanent;
    }
}
