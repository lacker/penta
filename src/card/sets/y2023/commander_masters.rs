//! Commander Masters cards cataloged for legend-rule coverage.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CMM",
    slug: "commander-masters",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CMM 707 — Sliver Gravemother
// Audit: unsupported — Needs encore, including a graveyard-granted variable-cost ability and its attacking token copies with delayed sacrifice.
pub(in crate::card::sets) static SLIVER_GRAVEMOTHER: CardRecord = CardRecord::new(
    "Sliver Gravemother",
    "9f5d253e-9eb2-423c-90ee-68f27ec6bf88",
    "Chris Rahn",
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SLIVER_GRAVEMOTHER];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
