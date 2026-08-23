//! TSP card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// TSP 29 — Momentary Blink
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENTARY_BLINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("032e072a-0630-472b-9106-5df554dff785"),
    "Momentary Blink",
    crate::card::CardArt::new("032e072a-0630-472b-9106-5df554dff785", "Anthony S. Waters"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 66 — Looter il-Kor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOOTER_IL_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("368ee06f-9021-4b65-9f53-9c326bf3a27f"),
    "Looter il-Kor",
    crate::card::CardArt::new("368ee06f-9021-4b65-9f53-9c326bf3a27f", "Mike Dringenberg"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 104 — Dread Return
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_RETURN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7e304fc-0ace-459e-8d2f-376f1899639c"),
    "Dread Return",
    crate::card::CardArt::new("d7e304fc-0ace-459e-8d2f-376f1899639c", "Kev Walker"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 180 — Sulfurous Blast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SULFUROUS_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67511e0e-be09-4f4e-9949-b9ecbdc7f536"),
    "Sulfurous Blast",
    crate::card::CardArt::new("67511e0e-be09-4f4e-9949-b9ecbdc7f536", "Jeff Miracola"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MOMENTARY_BLINK,
    &LOOTER_IL_KOR,
    &DREAD_RETURN,
    &SULFUROUS_BLAST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
