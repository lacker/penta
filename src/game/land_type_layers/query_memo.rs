// Scratch answers for one board-reading query.
//
// The layer-4 land-type slice is re-derived from scratch on every question,
// and the questions are asked per permanent. `land_type_effect_sources` walks
// the whole battlefield asking each permanent for its effective rules, and
// `rules_text_abilities_removed_from_sources` walks the sources asking
// `raw_land_type_set_applies` about each, which walks the sources again. That
// is a battlefield scan plus a quadratic for one permanent, repeated for
// every permanent, and on a board with a few land-type sources it is where
// most of a turn's CPU goes.
//
// A query holds `&self`, so the board cannot move underneath it and an answer
// stays good until it returns. The memo is installed by the two long `&self`
// reads, `legal_actions` and `observe`, and dropped when they return. Nothing
// installs one around a mutation, so an answer never outlives the board it
// describes; where no memo is installed every question is asked as before.
//
// Thread-local rather than a field, because `Game` stays `Send + Sync` for the
// Python binding and a `RefCell` field would cost that.

#[derive(Default)]
struct LandTypeQueryMemo {
    /// The game these answers describe. The installing borrow keeps it alive
    /// for the memo's whole life, so a different address is a different board
    /// rather than a reused one.
    game: usize,
    /// Whether a permanent supplies a land-type effect at all.
    supplies: std::collections::HashMap<crate::GameObjectId, bool>,
    /// Whether some source sets this permanent's land types, and so silences
    /// its rules text under CR 305.7. Keyed by the sources considered as well,
    /// because a prospective arrival changes that list.
    removed: std::collections::HashMap<(crate::GameObjectId, usize), bool>,
    /// The source lists seen so far, identified by position. The board is
    /// fixed for the memo's life, so the list varies only with whether a
    /// prospective arrival was folded in: there are one or two of them, and
    /// comparing them outright is both cheaper than hashing each call and
    /// exact, where a fingerprint could collide into a wrong answer.
    source_lists: Vec<Vec<(crate::GameObjectId, ContinuousEffectTimestamp)>>,
}

thread_local! {
    static LAND_TYPE_QUERY_MEMO: std::cell::RefCell<Option<LandTypeQueryMemo>> =
        const { std::cell::RefCell::new(None) };
}

/// Drops the memo when the query that installed it returns, panic included.
pub(in crate::game) struct LandTypeQueryMemoGuard {
    installed: bool,
}

impl Drop for LandTypeQueryMemoGuard {
    fn drop(&mut self) {
        if self.installed {
            LAND_TYPE_QUERY_MEMO.with(|memo| *memo.borrow_mut() = None);
        }
    }
}

impl Game {
    /// Lets one `&self` read reuse land-type answers across the permanents it
    /// asks about. Held by the caller; answers are discarded when it drops.
    pub(in crate::game) fn hold_land_type_query_memo(&self) -> LandTypeQueryMemoGuard {
        let game = std::ptr::from_ref(self) as usize;
        let installed = LAND_TYPE_QUERY_MEMO.with(|memo| {
            let mut memo = memo.borrow_mut();
            if memo.is_none() {
                *memo = Some(LandTypeQueryMemo {
                    game,
                    ..LandTypeQueryMemo::default()
                });
                true
            } else {
                false
            }
        });
        LandTypeQueryMemoGuard { installed }
    }

    /// Whether a memo for this board is installed. Checked before a key is
    /// built, because building one walks the source list and every question
    /// asked outside `legal_actions` and `observe` would pay for it unused.
    fn land_type_memo_installed(&self) -> bool {
        let game = std::ptr::from_ref(self) as usize;
        LAND_TYPE_QUERY_MEMO
            .with(|memo| memo.borrow().as_ref().is_some_and(|memo| memo.game == game))
    }

    /// Reads one remembered answer, if a memo for this board is installed.
    fn remembered<K: std::hash::Hash + Eq, V: Copy>(
        &self,
        key: &K,
        pick: impl Fn(&LandTypeQueryMemo) -> &std::collections::HashMap<K, V>,
    ) -> Option<V> {
        let game = std::ptr::from_ref(self) as usize;
        LAND_TYPE_QUERY_MEMO.with(|memo| {
            memo.borrow()
                .as_ref()
                .filter(|memo| memo.game == game)
                .and_then(|memo| pick(memo).get(key).copied())
        })
    }

    /// Remembers one answer, if a memo for this board is installed.
    fn remember<K: std::hash::Hash + Eq, V: Copy>(
        &self,
        key: K,
        value: V,
        pick: impl Fn(&mut LandTypeQueryMemo) -> &mut std::collections::HashMap<K, V>,
    ) {
        let game = std::ptr::from_ref(self) as usize;
        LAND_TYPE_QUERY_MEMO.with(|memo| {
            if let Some(memo) = memo.borrow_mut().as_mut()
                && memo.game == game
            {
                pick(memo).insert(key, value);
            }
        });
    }

    pub(in crate::game) fn supplies_land_type_effect(&self, source: &Permanent) -> bool {
        if !self.land_type_memo_installed() {
            return self.supplies_land_type_effect_uncached(source);
        }
        let key = source.card.id;
        if let Some(supplies) = self.remembered(&key, |memo| &memo.supplies) {
            return supplies;
        }
        let supplies = self.supplies_land_type_effect_uncached(source);
        self.remember(key, supplies, |memo| &mut memo.supplies);
        supplies
    }

    pub(super) fn rules_text_abilities_removed_from_sources(
        &self,
        affected: &Permanent,
        sources: &[(&Permanent, ContinuousEffectTimestamp)],
    ) -> bool {
        let Some(list) = self.source_list_index(sources) else {
            return self.rules_text_abilities_removed_from_sources_uncached(affected, sources);
        };
        let key = (affected.card.id, list);
        if let Some(removed) = self.remembered(&key, |memo| &memo.removed) {
            return removed;
        }
        let removed = self.rules_text_abilities_removed_from_sources_uncached(affected, sources);
        self.remember(key, removed, |memo| &mut memo.removed);
        removed
    }

    /// Which of the memo's source lists this one is, recording it if new.
    fn source_list_index(
        &self,
        sources: &[(&Permanent, ContinuousEffectTimestamp)],
    ) -> Option<usize> {
        let game = std::ptr::from_ref(self) as usize;
        LAND_TYPE_QUERY_MEMO.with(|memo| {
            let mut memo = memo.borrow_mut();
            let memo = memo.as_mut().filter(|memo| memo.game == game)?;
            let matches = |list: &Vec<(crate::GameObjectId, ContinuousEffectTimestamp)>| {
                list.len() == sources.len()
                    && list
                        .iter()
                        .zip(sources)
                        .all(|((id, at), (source, timestamp))| {
                            *id == source.card.id && at == timestamp
                        })
            };
            if let Some(index) = memo.source_lists.iter().position(matches) {
                return Some(index);
            }
            memo.source_lists.push(
                sources
                    .iter()
                    .map(|(source, timestamp)| (source.card.id, *timestamp))
                    .collect(),
            );
            Some(memo.source_lists.len() - 1)
        })
    }
}
