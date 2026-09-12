//! Wilds of Eldraine Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WOC",
    slug: "wilds-of-eldraine-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// WOC 40 — Throne of Eldraine
// Audit: unsupported — The activation payment planner cannot require every mana unit paying a generic cost to have the source's chosen color. A production restriction would constrain the wrong payment.
pub(in crate::card::sets) static THRONE_OF_ELDRAINE_40: CardRecord = CardRecord::new(
    "Throne of Eldraine",
    "6bba4159-d507-4bb2-8c71-be7cd42d9277",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// WOC 47 — Misleading Signpost
// Audit: unsupported — Attack targets are fixed by attack declaration. No resolving operation can reselect an existing attacker's defending player or attacked permanent.
pub(in crate::card::sets) static MISLEADING_SIGNPOST_47: CardRecord = CardRecord::new(
    "Misleading Signpost",
    "958b247d-83d3-4dd6-9a1a-654ba3adf078",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &THRONE_OF_ELDRAINE_40,
    &MISLEADING_SIGNPOST_47,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
