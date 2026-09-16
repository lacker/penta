//! Phasing out, and phasing back in.
//!
//! A phased-out permanent is treated as though it does not exist (CR 702.25),
//! so it is held apart from the battlefield rather than left on it behind a
//! flag: every walk over the battlefield is then right without knowing that
//! phasing exists. Phasing is not a zone change, so the permanent keeps its
//! identity, its counters, its damage, and whatever is attached to it, and
//! nothing triggers on it leaving or arriving.

use super::{Game, GameObjectId, PlayerId};

impl Game {
    pub(super) fn phase_out_recipients(
        &mut self,
        recipient: super::EffectRecipientDef,
        object: &super::StackObject,
        context: &super::EffectResolutionContext,
        scoped: super::ScopedEffect,
    ) {
        let ids = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .filter_map(|target| match target {
                super::Target::Permanent(id) => Some(id),
                _ => None,
            })
            .collect::<Vec<_>>();
        self.phase_out_group(ids);
    }

    #[cfg(test)]
    pub(super) fn phase_out(&mut self, id: GameObjectId) {
        self.phase_out_group(vec![id]);
    }

    /// Phases a set out simultaneously, including every Aura, Equipment, or
    /// Fortification attached to one of its members. An attached permanent
    /// phases indirectly even when another player controls it.
    fn phase_out_group(&mut self, mut ids: Vec<GameObjectId>) {
        let mut cursor = 0;
        while cursor < ids.len() {
            let host = ids[cursor];
            for attached in self
                .battlefield
                .iter()
                .filter(|permanent| permanent.attached_to == Some(host))
                .map(|permanent| permanent.card.id)
            {
                if !ids.contains(&attached) {
                    ids.push(attached);
                }
            }
            cursor += 1;
        }

        let mut remaining = Vec::with_capacity(self.battlefield.len());
        for permanent in self.battlefield.drain(..) {
            if ids.contains(&permanent.card.id) {
                self.phased_out.push(permanent);
            } else {
                remaining.push(permanent);
            }
        }
        self.battlefield = remaining;
        self.prune_prepared_spell_copies();
    }

    /// Phases in everything of this player's that is waiting, which happens
    /// before they untap rather than as part of untapping.
    pub(super) fn phase_in_for(&mut self, player: PlayerId) {
        let phased_ids = self
            .phased_out
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        let mut returning_ids = self
            .phased_out
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent
                        .attached_to
                        .is_none_or(|host| !phased_ids.contains(&host))
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        let mut cursor = 0;
        while cursor < returning_ids.len() {
            let host = returning_ids[cursor];
            for attached in self
                .phased_out
                .iter()
                .filter(|permanent| permanent.attached_to == Some(host))
                .map(|permanent| permanent.card.id)
            {
                if !returning_ids.contains(&attached) {
                    returning_ids.push(attached);
                }
            }
            cursor += 1;
        }
        let returning = self
            .phased_out
            .iter()
            .enumerate()
            .filter(|(_, permanent)| returning_ids.contains(&permanent.card.id))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        for index in returning.into_iter().rev() {
            let permanent = self.phased_out.remove(index);
            let id = permanent.card.id;
            self.battlefield.push(permanent);
            self.create_prepared_spell(id);
        }
    }
}
