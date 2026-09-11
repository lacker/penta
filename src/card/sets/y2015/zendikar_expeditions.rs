//! Zendikar Expeditions card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "EXP",
    slug: "zendikar-expeditions",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());







// EXP 3 — Smoldering Marsh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMOLDERING_MARSH_3: CardRecord = CardRecord::new(
    "Smoldering Marsh",
    "2c9814b3-d4fe-4823-b310-ab284dc9b9be",
    "Titus Lunter",
    crate::card::CardRules::unsupported(),
);

// EXP 4 — Cinder Glade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_GLADE_4: CardRecord = CardRecord::new(
    "Cinder Glade",
    "1b5d2e9c-dd93-49ae-9a03-77c13c5ec298",
    "Titus Lunter",
    crate::card::CardRules::unsupported(),
);

// EXP 5 — Canopy Vista
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANOPY_VISTA_5: CardRecord = CardRecord::new(
    "Canopy Vista",
    "3125bf61-f1b8-4b97-96bc-2d2a3a32adbd",
    "Titus Lunter",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &SMOLDERING_MARSH_3,
    &CINDER_GLADE_4,
    &CANOPY_VISTA_5,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
