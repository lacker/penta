//! Revised Edition has no unique catalog records.
//!
//! Cards legal through this printing reuse their earliest built-in definition.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha;

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL), // 3ED 21
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT), // 3ED 48
];
