//! Sagas (CR 714): the counter that reads them, and the rules actions that
//! place it.
//!
//! Nothing about a Saga's progress is printed on the card. A lore counter
//! goes on as it enters and another after its controller's draw step, its
//! chapter abilities trigger on the counter that reaches their number, and
//! once the last chapter has been read and left the stack the Saga is
//! sacrificed. Only the chapters themselves are authored.

use super::{CounterKind, Game, GameObjectId, Permanent, PlayerId};

impl Game {
    fn is_saga(&self, permanent: &Permanent) -> bool {
        self.effective_subtypes(permanent)
            .contains(crate::card::Subtype::Saga)
    }

    /// CR 714.2a: a Saga enters with a lore counter on it. Placed through
    /// the ordinary counter path rather than as part of the arrival, so the
    /// first chapter sees a counter being put on and triggers.
    pub(super) fn place_entry_lore_counter(&mut self, permanent: GameObjectId) {
        if self
            .battlefield
            .iter()
            .find(|candidate| candidate.card.id == permanent)
            .is_some_and(|candidate| self.is_saga(candidate))
        {
            self.add_counters_to_permanent(permanent, CounterKind::Lore, 1);
        }
    }

    /// CR 714.2b: after its controller's draw step, every Saga they control
    /// gets another lore counter -- which is what reads the next chapter.
    pub(super) fn place_draw_step_lore_counters(&mut self, player: PlayerId) {
        let sagas: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player && self.is_saga(permanent))
            .map(|permanent| permanent.card.id)
            .collect();
        for saga in sagas {
            self.add_counters_to_permanent(saga, CounterKind::Lore, 1);
        }
    }

    /// CR 714.4: a Saga whose last chapter has been read and whose chapter
    /// abilities have all left the stack is sacrificed. A Saga that changed
    /// what it is -- Fable of the Mirror-Breaker exiles itself and comes
    /// back as a creature -- is no longer a Saga and is never asked.
    pub(super) fn sacrifice_completed_sagas(&mut self) {
        let finished: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| self.is_saga(permanent))
            .filter(|permanent| {
                permanent.counters(CounterKind::Lore)
                    >= u16::from(self.saga_final_chapter(permanent))
            })
            .map(|permanent| permanent.card.id)
            .filter(|saga| !self.saga_has_a_chapter_waiting(*saga))
            .collect();
        if !finished.is_empty() {
            self.move_permanents_to_graveyard(&finished);
        }
    }

    /// The highest chapter number this Saga currently has. Read off the
    /// abilities it actually presents rather than the ones it prints,
    /// because an effect can take chapters away: Blood Moon setting Urza's
    /// Saga's land type strips its printed abilities, and the Saga is
    /// sacrificed on the spot with nothing left to read.
    ///
    /// A Saga with no chapter abilities has a final chapter of zero, which
    /// any number of lore counters already meets.
    fn saga_final_chapter(&self, permanent: &Permanent) -> u8 {
        self.effective_abilities(permanent)
            .iter()
            .filter_map(|effective| effective.ability.saga_chapter())
            .max()
            .unwrap_or(0)
    }

    /// Whether one of this Saga's chapter abilities is still waiting to
    /// resolve, on the stack or on its way there.
    fn saga_has_a_chapter_waiting(&self, saga: GameObjectId) -> bool {
        self.stack.iter().any(|object| object.source == Some(saga))
            || self
                .pending_triggers
                .iter()
                .any(|trigger| trigger.source.object == saga)
    }
}
