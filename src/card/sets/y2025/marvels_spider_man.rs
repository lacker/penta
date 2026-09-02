//! SPM card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{CardArt, CardRules, CardSet};

// SPM 141 — Rhino's Rampage
// Audit: unsupported — Needs a reflexive excess-damage trigger that chooses its artifact target after the fight.
pub(in crate::card::sets) static RHINOS_RAMPAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f668817c-1cab-44c5-b6a8-95113e480d5e"),
    "Rhino's Rampage",
    CardArt::new("f668817c-1cab-44c5-b6a8-95113e480d5e", "Nino Is"),
    CardSet::MarvelsSpiderMan,
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&RHINOS_RAMPAGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
