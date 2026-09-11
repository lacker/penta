//! Global Series Jiang Yanggu & Mu Yanling card records.

use super::{CardRecord, PrintingRecord};

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "GS1",
    slug: "global-series-jiang-yanggu-and-mu-yanling",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// GS1 12 — Ancestor Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTOR_DRAGON: CardRecord = CardRecord::new(
    "Ancestor Dragon",
    "9ba9d1f0-a864-490c-b258-6bf9de251c4b",
    "Shinchuen Chen",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&ANCESTOR_DRAGON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
