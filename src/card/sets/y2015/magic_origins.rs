//! ORI card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// ORI 62 — Jhessian Thief
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JHESSIAN_THIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33b8553d-d326-4280-bc3a-2fffdd377cd2"),
    "Jhessian Thief",
    crate::card::CardArt::new("33b8553d-d326-4280-bc3a-2fffdd377cd2", "Miles Johnston"),
    crate::card::CardSet::MagicOrigins,
    crate::card::CardRules::unsupported(),
);

// ORI 171 — Conclave Naturalists
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONCLAVE_NATURALISTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3759fc28-9adb-41ed-851c-566a3a424e09"),
    "Conclave Naturalists",
    crate::card::CardArt::new("3759fc28-9adb-41ed-851c-566a3a424e09", "Howard Lyon"),
    crate::card::CardSet::MagicOrigins,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&JHESSIAN_THIEF, &CONCLAVE_NATURALISTS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
