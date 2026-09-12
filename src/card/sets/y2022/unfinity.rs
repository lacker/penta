//! Unfinity card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "UNF",
    slug: "unfinity",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());







// UNF 88 — Saw in Half
// Audit: unsupported — CopyExceptionsDef stores fixed power/toughness only. It cannot install copiable base values computed by halving the destroyed creature's last-known power and toughness; a later Apply would not be a copy exception.
pub(in crate::card::sets) static SAW_IN_HALF_88: CardRecord = CardRecord::new(
    "Saw in Half",
    "05e6a7bc-a35a-4e68-99a0-be264553b5de",
    "Sebastian Giacobino",
    crate::card::CardRules::unsupported(),
);

// UNF 393 — _____ Goblin
// Audit: unsupported — The engine has no sticker sheet selection, name-sticker application, or unique-vowel value over the selected sticker.
pub(in crate::card::sets) static GOBLIN_393: CardRecord = CardRecord::new(
    "_____ Goblin",
    "40f80bd8-7ce2-449d-b06c-d3e353b54daa",
    "Chuck Lukacs",
    crate::card::CardRules::unsupported(),
);

// UNF 472 — Clown Car
// Audit: unsupported — The engine has no dice-roll collection or odd/even result count for X d6 rolls.
pub(in crate::card::sets) static CLOWN_CAR_472: CardRecord = CardRecord::new(
    "Clown Car",
    "44fd9541-3b1c-4af1-ad23-92c1b1bd01e2",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &SAW_IN_HALF_88,
    &GOBLIN_393,
    &CLOWN_CAR_472,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
