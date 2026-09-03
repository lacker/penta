//! SPM card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{CardArt, CardRules, CardSet};

// SPM 93 — Spider-Verse
// Audit: unsupported — Needs a once-each-turn trigger for spells cast outside hand whose optional stack copy can grant haste specifically when it copies a permanent spell.
pub(in crate::card::sets) static SPIDER_VERSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8779eb2-1210-430d-8d42-3077053441ee"),
    "Spider-Verse",
    CardArt::new("f8779eb2-1210-430d-8d42-3077053441ee", "Alexander Gering"),
    CardSet::MarvelsSpiderMan,
    CardRules::unsupported(),
);

// SPM 141 — Rhino's Rampage
// Audit: unsupported — Needs a reflexive excess-damage trigger that chooses its artifact target after the fight.
pub(in crate::card::sets) static RHINOS_RAMPAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f668817c-1cab-44c5-b6a8-95113e480d5e"),
    "Rhino's Rampage",
    CardArt::new("f668817c-1cab-44c5-b6a8-95113e480d5e", "Nino Is"),
    CardSet::MarvelsSpiderMan,
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SPIDER_VERSE, &RHINOS_RAMPAGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
