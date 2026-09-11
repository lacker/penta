//! Secret Lair Drop card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SLD",
    slug: "secret-lair-drop",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());











// SLD 1240 — Jurin, Leading the Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JURIN_LEADING_THE_CHARGE_1240: CardRecord = CardRecord::new(
    "Jurin, Leading the Charge",
    "fab07547-a6b8-487a-9688-8f93aaa71f5b",
    "Yang Luo",
    crate::card::CardRules::unsupported(),
);

// SLD 1731 — Iron Man, Titan of Innovation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_MAN_TITAN_OF_INNOVATION_1731: CardRecord = CardRecord::new(
    "Iron Man, Titan of Innovation",
    "7542b1d1-e34d-46dc-af24-1b718034c0e4",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// SLD 1753 — Deadpool, Trading Card
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADPOOL_TRADING_CARD_1753: CardRecord = CardRecord::new(
    "Deadpool, Trading Card",
    "3a14d6c5-cfd1-4860-834a-5a0dc9df0320",
    "Justine Cruz",
    crate::card::CardRules::unsupported(),
);

// SLD 2082 — Knuckles the Echidna
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNUCKLES_THE_ECHIDNA_2082: CardRecord = CardRecord::new(
    "Knuckles the Echidna",
    "54b65cca-7844-4d3e-9d7f-ed1f49f94425",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// SLD 2226 — Jin Sakai, Ghost of Tsushima
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JIN_SAKAI_GHOST_OF_TSUSHIMA_2226: CardRecord = CardRecord::new(
    "Jin Sakai, Ghost of Tsushima",
    "9f52160b-dcc0-4ab8-b8b2-b576b1688d57",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &JURIN_LEADING_THE_CHARGE_1240,
    &IRON_MAN_TITAN_OF_INNOVATION_1731,
    &DEADPOOL_TRADING_CARD_1753,
    &KNUCKLES_THE_ECHIDNA_2082,
    &JIN_SAKAI_GHOST_OF_TSUSHIMA_2226,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
