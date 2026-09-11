//! Doctor Who cards cataloged for legend-rule coverage.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WHO",
    slug: "doctor-who",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// WHO 146 — The Master, Multiplied
// Audit: unsupported — Needs a player rule that prevents triggered abilities from causing sacrifice or exile of creature tokens.
pub(in crate::card::sets) static THE_MASTER_MULTIPLIED: CardRecord = CardRecord::new(
    "The Master, Multiplied",
    "7f734ca0-91bc-4496-9bd7-2d09415e850f",
    "Lie Setiawan",
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&THE_MASTER_MULTIPLIED];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
