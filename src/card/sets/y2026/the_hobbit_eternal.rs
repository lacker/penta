//! The Hobbit Eternal card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "HOC",
    slug: "the-hobbit-eternal",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// HOC 1 — Fíli and Kíli, Joyous
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FILI_AND_KILI_JOYOUS_1: CardRecord = CardRecord::new(
    "Fíli and Kíli, Joyous",
    "e1d12200-ae0b-4155-9853-3ffaf490c84c",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// HOC 8 — Dragon-Cursed Halls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_CURSED_HALLS_8: CardRecord = CardRecord::new(
    "Dragon-Cursed Halls",
    "506b9df7-8236-4c6e-aebc-6b7e6fcd7e88",
    "Marta Nael",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &FILI_AND_KILI_JOYOUS_1,
    &DRAGON_CURSED_HALLS_8,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
