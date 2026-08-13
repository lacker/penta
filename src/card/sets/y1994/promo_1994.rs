//! The three unique 1994 promotional cards are legal in Old School 93/94, but
//! each currently needs an unsupported declarative capability. Their exact gaps
//! are recorded inline at their synthetic Eternal Central collector positions.

use super::{CardRecord, PrintingRecord};

// P94 1 — Arena
// Audit: blocked — Needs a fight effect that deals simultaneous reciprocal power damage after the linked target choices for “{3}, {T}: Tap target creature you control and target creature of an opponent's choice they control. Those creatures fight each other”.

// P94 2 — Sewers of Estark
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Choose target creature. If it's attacking, it can't be blocked this turn. If it's blocking, prevent all combat damage that would be dealt this combat by it and each creature it's blocking”.

// P94 3 — Nalathni Dragon
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
