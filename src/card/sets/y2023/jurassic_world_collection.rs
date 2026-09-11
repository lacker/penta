//! Jurassic World Collection card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "REX",
    slug: "jurassic-world-collection",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());









// REX 7 — Welcome to . . . // Jurassic Park
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELCOME_TO_JURASSIC_PARK_7: CardRecord = CardRecord::new(
    "Welcome to . . . // Jurassic Park",
    "6d84e2d4-38bf-4d46-99a6-37c2dda66b16",
    "Villarrte",
    crate::card::CardRules::unsupported(),
);

// REX 30 — Hunting Velociraptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_VELOCIRAPTOR_30: CardRecord = CardRecord::new(
    "Hunting Velociraptor",
    "419ef4a1-20d2-4770-99a5-7673518d0b86",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// REX 32 — Savage Order
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_ORDER_32: CardRecord = CardRecord::new(
    "Savage Order",
    "e01e2ded-f3ed-41c2-b200-6573d2c15611",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// REX 42 — Permission Denied
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERMISSION_DENIED_42: CardRecord = CardRecord::new(
    "Permission Denied",
    "678ad86d-6b2b-4e66-a535-9fc696cfa4ab",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &WELCOME_TO_JURASSIC_PARK_7,
    &HUNTING_VELOCIRAPTOR_30,
    &SAVAGE_ORDER_32,
    &PERMISSION_DENIED_42,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
