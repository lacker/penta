//! Opt-in semantic execution counters, compiled only with `engine-profiling`.
//!
//! A capture owns the current thread, including all games run on that thread.
//! Counts describe dispatch/evaluation attempts, not successful completion or
//! CPU time. They are diagnostic data and never enter game state or checkpoints.

mod names;
#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;

pub(crate) use names::{action_kind, effect_kind, predicate_kind, prepared_predicate_kind};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Key {
    category: &'static str,
    operation: &'static str,
    path: &'static str,
    reason: &'static str,
}

thread_local! {
    static ACTIVE: RefCell<Option<BTreeMap<Key, u64>>> = const { RefCell::new(None) };
}

/// One bounded-label semantic counter. Labels never contain game/card data.
#[derive(Debug, serde::Serialize)]
pub struct Counter {
    pub category: &'static str,
    pub operation: &'static str,
    pub path: &'static str,
    pub reason: &'static str,
    pub count: u64,
}

/// Sorted semantic counters for one current-thread capture.
#[derive(Debug, serde::Serialize)]
pub struct Report {
    pub schema_version: u32,
    pub scope: &'static str,
    pub counters: Vec<Counter>,
}

/// Owns the current thread's capture. Cannot move across threads. Dropping an
/// unfinished capture discards its counters, including during panic unwinding.
#[derive(Debug)]
pub struct Capture {
    finished: bool,
    thread: PhantomData<Rc<()>>,
}

impl Capture {
    /// Start an empty capture on this thread. Other threads are independent.
    ///
    /// # Errors
    /// Returns an error when a capture already owns this thread. Its counters
    /// remain intact; nested callers cannot silently reset an outer capture.
    pub fn start() -> Result<Self, &'static str> {
        ACTIVE.with(|active| {
            let mut active = active.borrow_mut();
            if active.is_some() {
                return Err("an engine profiling capture is already active on this thread");
            }
            *active = Some(BTreeMap::new());
            Ok(Self {
                finished: false,
                thread: PhantomData,
            })
        })
    }

    /// Stop recording and return counters in deterministic label order.
    #[must_use]
    pub fn finish(mut self) -> Report {
        let counters = ACTIVE.with(|active| active.borrow_mut().take().unwrap_or_default());
        self.finished = true;
        Report {
            schema_version: 1,
            scope: "current_thread",
            counters: counters
                .into_iter()
                .map(|(key, count)| Counter {
                    category: key.category,
                    operation: key.operation,
                    path: key.path,
                    reason: key.reason,
                    count,
                })
                .collect(),
        }
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        if !self.finished {
            ACTIVE.with(|active| *active.borrow_mut() = None);
        }
    }
}

pub(crate) fn record(
    category: &'static str,
    operation: &'static str,
    path: &'static str,
    reason: &'static str,
) {
    ACTIVE.with(|active| {
        if let Some(counters) = active.borrow_mut().as_mut() {
            let count = counters
                .entry(Key {
                    category,
                    operation,
                    path,
                    reason,
                })
                .or_default();
            *count = count.saturating_add(1);
        }
    });
}

pub(crate) fn predicate_evaluated(predicate: crate::ObjectPredicateDef, path: &'static str) {
    record(
        "predicate_evaluation",
        predicate_kind(predicate),
        path,
        "entered",
    );
}
