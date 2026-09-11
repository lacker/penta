//! Teenage Mutant Ninja Turtles Eternal card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TMC",
    slug: "teenage-mutant-ninja-turtles-eternal",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());











// TMC 20 — Shredder, Shadow Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHREDDER_SHADOW_MASTER_20: CardRecord = CardRecord::new(
    "Shredder, Shadow Master",
    "ddf4d3c9-bef9-4796-91ef-3d5beebae571",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMC 22 — Casey Jones, Back Alley Brute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASEY_JONES_BACK_ALLEY_BRUTE_22: CardRecord = CardRecord::new(
    "Casey Jones, Back Alley Brute",
    "416d888d-2699-49bd-830b-bf5bda25c91f",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// TMC 97 — Big Mother Mouser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIG_MOTHER_MOUSER_97: CardRecord = CardRecord::new(
    "Big Mother Mouser",
    "8f586a85-f9b7-49c4-bfb0-e348b9088841",
    "Kirokaze",
    crate::card::CardRules::unsupported(),
);

// TMC 107 — April O'Neil, Human Element
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APRIL_O_NEIL_HUMAN_ELEMENT_107: CardRecord = CardRecord::new(
    "April O'Neil, Human Element",
    "cd6d7bba-2de4-49fe-8e07-a7c00c6bfc96",
    "Fahmi Fauzi",
    crate::card::CardRules::unsupported(),
);

// TMC 118 — Raphael, Tag Team Tough
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPHAEL_TAG_TEAM_TOUGH_118: CardRecord = CardRecord::new(
    "Raphael, Tag Team Tough",
    "b3bae5c7-2c4a-42a6-a108-3f23d70204b4",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SHREDDER_SHADOW_MASTER_20,
    &CASEY_JONES_BACK_ALLEY_BRUTE_22,
    &BIG_MOTHER_MOUSER_97,
    &APRIL_O_NEIL_HUMAN_ELEMENT_107,
    &RAPHAEL_TAG_TEAM_TOUGH_118,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
