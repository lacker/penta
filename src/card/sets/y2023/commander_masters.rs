//! Commander Masters cards cataloged for legend-rule coverage.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{CardArt, CardRules, CardSet};

// CMM 707 — Sliver Gravemother
// Audit: unsupported — Needs encore, including a graveyard-granted variable-cost ability and its attacking token copies with delayed sacrifice.
pub(in crate::card::sets) static SLIVER_GRAVEMOTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f5d253e-9eb2-423c-90ee-68f27ec6bf88"),
    "Sliver Gravemother",
    CardArt::new("9f5d253e-9eb2-423c-90ee-68f27ec6bf88", "Chris Rahn"),
    CardSet::CommanderMasters,
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SLIVER_GRAVEMOTHER];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
