//! Aetherdrift Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DRC",
    slug: "aetherdrift-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());







// DRC 1 — Hashaton, Scarab's Fist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HASHATON_SCARAB_S_FIST_1: CardRecord = CardRecord::new(
    "Hashaton, Scarab's Fist",
    "02645651-cd55-4bd0-8a4d-fa257270a0e0",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// DRC 19 — Stridehangar Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRIDEHANGAR_AUTOMATON_19: CardRecord = CardRecord::new(
    "Stridehangar Automaton",
    "4c398d11-a2fc-43e2-96e9-7f3383e1a43c",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// DRC 36 — Accursed Duneyard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACCURSED_DUNEYARD_36: CardRecord = CardRecord::new(
    "Accursed Duneyard",
    "7f288449-e9a8-444e-9639-fbcb742d2409",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HASHATON_SCARAB_S_FIST_1,
    &STRIDEHANGAR_AUTOMATON_19,
    &ACCURSED_DUNEYARD_36,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
