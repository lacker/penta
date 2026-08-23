//! March of the Machine Commander card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// MOC 30 — Death-Greeter's Champion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_GREETER_S_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7cb2b582-1c45-4bb2-8aef-59a71a5a9e94"),
    "Death-Greeter's Champion",
    crate::card::CardArt::new("7cb2b582-1c45-4bb2-8aef-59a71a5a9e94", "Jason Rainville"),
    crate::card::CardSet::MarchOfTheMachineCommander,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DEATH_GREETER_S_CHAMPION];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
