//! Doctor Who cards cataloged for legend-rule coverage.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{CardArt, CardRules, CardSet};

// WHO 146 — The Master, Multiplied
// Audit: unsupported — Needs a player rule that prevents triggered abilities from causing sacrifice or exile of creature tokens.
pub(in crate::card::sets) static THE_MASTER_MULTIPLIED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f734ca0-91bc-4496-9bd7-2d09415e850f"),
    "The Master, Multiplied",
    CardArt::new("7f734ca0-91bc-4496-9bd7-2d09415e850f", "Lie Setiawan"),
    CardSet::DoctorWho,
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&THE_MASTER_MULTIPLIED];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
