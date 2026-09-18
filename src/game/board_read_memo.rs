// Scratch answers a long `&self` board read may reuse.
//
// Two sweeps dominate `legal_actions` on a board with a dozen lands and
// several activatable abilities, and both are re-derived per question:
//
//   * `battlefield_trigger_listeners` walks every permanent and emblem asking
//     for its effective abilities, which rebuilds the layer stack for each.
//     `with_triggered_mana_choices` asks it for every mana source that taps,
//     to learn whether tapping wakes a mana trigger.
//   * `mana_ability_activations` rebuilds one permanent's mana abilities.
//     `flexible_mana_sources` asks it for every permanent its controller has,
//     and payment planning runs once per candidate action.
//
// Nesting those inside the payment search made the work grow with the square
// of the board: a Premodern gauntlet pairing that took two seconds at eight
// hundred actions took four minutes at sixteen hundred.
//
// The invariant is the one `land_type_layers/query_memo.rs` documents: a query
// holds `&self`, so the board cannot move underneath it and an answer stays
// good until it returns. Installed by long immutable reads, including payment
// announcement queries, and dropped when they return. Where no memo
// is installed both sweeps run as before, so every mutation path is untouched.
// The game's address identifies the board, and the installing borrow keeps it
// alive for the memo's whole life, so a different address is a different game
// rather than a reused allocation.
//
// Thread-local rather than a field, because `Game` stays `Send + Sync` for the
// Python binding and a `RefCell` field would cost that.

mod abilities;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

use super::{BattlefieldTriggerListener, Game, GameObjectId, ManaAbilityActivation, Permanent};

#[derive(Default)]
struct BoardReadMemo {
    game: usize,
    live_permanents: HashSet<usize>,
    inline_rules: HashMap<usize, Arc<super::CardRules>>,
    abilities: abilities::AbilityReadMemo,
    listeners: Option<Rc<Vec<BattlefieldTriggerListener>>>,
    mana_activations: std::collections::HashMap<GameObjectId, Rc<Vec<ManaAbilityActivation>>>,
}

thread_local! {
    static BOARD_READ_MEMO: std::cell::RefCell<Option<BoardReadMemo>> =
        const { std::cell::RefCell::new(None) };
}

/// Restores the enclosing query's memo when a nested game read returns, panic
/// included. The borrow prevents mutation of the game whose answers are cached.
pub(in crate::game) struct BoardReadMemoGuard<'a> {
    installed: bool,
    previous: Option<BoardReadMemo>,
    borrow: std::marker::PhantomData<&'a Game>,
}

impl Drop for BoardReadMemoGuard<'_> {
    fn drop(&mut self) {
        if self.installed {
            BOARD_READ_MEMO.with(|memo| *memo.borrow_mut() = self.previous.take());
        }
    }
}

impl Game {
    /// Lets one `&self` read reuse the board sweeps below. Held by the caller;
    /// answers are discarded when it drops.
    pub(in crate::game) fn hold_board_read_memo(&self) -> BoardReadMemoGuard<'_> {
        let game = std::ptr::from_ref(self) as usize;
        let (installed, previous) = BOARD_READ_MEMO.with(|memo| {
            let mut memo = memo.borrow_mut();
            if memo.as_ref().is_none_or(|memo| memo.game != game) {
                let previous = memo.replace(BoardReadMemo {
                    game,
                    live_permanents: self
                        .battlefield
                        .iter()
                        .chain(&self.emblems)
                        .map(|permanent| std::ptr::from_ref(permanent) as usize)
                        .collect(),
                    ..BoardReadMemo::default()
                });
                (true, previous)
            } else {
                (false, None)
            }
        });
        BoardReadMemoGuard {
            installed,
            previous,
            borrow: std::marker::PhantomData,
        }
    }

    /// Only actual members of this borrowed board have stable addresses for
    /// the whole read. A prospective or last-known view may share an object
    /// ID, and a temporary's address can be reused before the read ends.
    fn live_read_key(&self, permanent: &Permanent) -> Option<usize> {
        let key = std::ptr::from_ref(permanent) as usize;
        self.board_memo(|memo| memo.live_permanents.contains(&key).then_some(key))
    }

    pub(super) fn remembered_inline_rules(
        &self,
        permanent: &Permanent,
    ) -> Option<Arc<super::CardRules>> {
        let key = std::ptr::from_ref(permanent) as usize;
        self.board_memo(|memo| memo.inline_rules.get(&key).cloned())
    }

    pub(super) fn remember_inline_rules(
        &self,
        permanent: &Permanent,
        rules: &Arc<super::CardRules>,
    ) {
        if let Some(key) = self.live_read_key(permanent) {
            self.remember_board(|memo| {
                memo.inline_rules.insert(key, Arc::clone(rules));
            });
        }
    }

    /// Reads from the memo installed for this board, if there is one.
    fn board_memo<T>(&self, read: impl Fn(&BoardReadMemo) -> Option<T>) -> Option<T> {
        let game = std::ptr::from_ref(self) as usize;
        BOARD_READ_MEMO.with(|memo| {
            memo.borrow()
                .as_ref()
                .filter(|memo| memo.game == game)
                .and_then(read)
        })
    }

    /// Writes to the memo installed for this board, if there is one.
    fn remember_board(&self, write: impl FnOnce(&mut BoardReadMemo)) {
        let game = std::ptr::from_ref(self) as usize;
        BOARD_READ_MEMO.with(|memo| {
            if let Some(memo) = memo.borrow_mut().as_mut()
                && memo.game == game
            {
                write(memo);
            }
        });
    }

    /// The listeners this board has, sweeping only when no memo for it is
    /// installed or the installed one has not been asked yet.
    pub(super) fn battlefield_trigger_listeners(&self) -> Vec<BattlefieldTriggerListener> {
        let _board = self.hold_board_read_memo();
        if let Some(listeners) = self.board_memo(|memo| memo.listeners.clone()) {
            return (*listeners).clone();
        }
        let listeners = Rc::new(self.battlefield_trigger_listeners_uncached());
        self.remember_board(|memo| memo.listeners = Some(Rc::clone(&listeners)));
        (*listeners).clone()
    }

    /// One permanent's mana abilities, under the same memo.
    pub(super) fn mana_ability_activations(
        &self,
        permanent: &Permanent,
    ) -> Vec<ManaAbilityActivation> {
        let key = permanent.card.id;
        if let Some(activations) =
            self.board_memo(|memo| memo.mana_activations.get(&key).map(Rc::clone))
        {
            return (*activations).clone();
        }
        let activations = Rc::new(self.mana_ability_activations_uncached(permanent));
        self.remember_board(|memo| {
            memo.mana_activations.insert(key, Rc::clone(&activations));
        });
        (*activations).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_read_memo_restores_outer_game_after_nested_reads_and_unwind() {
        let game = Game::new(
            crate::poc::catalog().unwrap(),
            [crate::poc::sligh(), crate::poc::the_deck()],
            1,
        )
        .unwrap();
        let other = game.clone();
        let outer = game.hold_board_read_memo();
        let listeners = Rc::new(Vec::new());
        game.remember_board(|memo| memo.listeners = Some(Rc::clone(&listeners)));
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _inner = other.hold_board_read_memo();
            assert_eq!(other.board_memo(|_| Some(true)), Some(true));
            assert_eq!(game.board_memo(|_| Some(true)), None);
            {
                let _same = other.hold_board_read_memo();
                other.remember_board(|memo| memo.listeners = Some(Rc::new(Vec::new())));
            }
            assert!(other.board_memo(|memo| memo.listeners.clone()).is_some());
            panic!("exercise read-scope unwinding");
        }));
        assert!(failed.is_err());
        assert!(Rc::ptr_eq(
            &listeners,
            &game.board_memo(|memo| memo.listeners.clone()).unwrap()
        ));
        assert_eq!(other.board_memo(|_| Some(true)), None);
        drop(outer);
        assert_eq!(game.board_memo(|_| Some(true)), None);
    }
}
