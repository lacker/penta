//! Forgotten Realms Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "AFC",
    slug: "forgotten-realms-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());



// AFC 324 — Prosper, Tome-Bound
// Audit: unsupported — The end-step trigger needs an exile-to-play permission limited to the next turn and a trigger for cards played from exile.
pub(in crate::card::sets) static PROSPER_TOME_BOUND_324: CardRecord = CardRecord::new(
    "Prosper, Tome-Bound",
    "0333ccf1-239a-4de2-bb0a-9b4ac1649adf",
    "Yongjae Choi",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PROSPER_TOME_BOUND_324,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
