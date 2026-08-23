//! GPT card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// GPT 56 — Plagued Rusalka
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUED_RUSALKA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd84bbb3-8b99-4e6d-b514-b094ec93eaa0"),
    "Plagued Rusalka",
    crate::card::CardArt::new(
        "cd84bbb3-8b99-4e6d-b514-b094ec93eaa0",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 74 — Scorched Rusalka
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHED_RUSALKA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f955164-ddb8-484c-a063-967621abce87"),
    "Scorched Rusalka",
    crate::card::CardArt::new("9f955164-ddb8-484c-a063-967621abce87", "Luca Zontini"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 125 — Pillory of the Sleepless
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PILLORY_OF_THE_SLEEPLESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36964bbd-f068-4a69-8d6b-7e4e97938b98"),
    "Pillory of the Sleepless",
    crate::card::CardArt::new("36964bbd-f068-4a69-8d6b-7e4e97938b98", "Mark Romanoski"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 158 — Gruul Turf
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRUUL_TURF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("550b70e0-ebd5-49de-b62c-5224b8bf8e98"),
    "Gruul Turf",
    crate::card::CardArt::new("550b70e0-ebd5-49de-b62c-5224b8bf8e98", "John Avon"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PLAGUED_RUSALKA,
    &SCORCHED_RUSALKA,
    &PILLORY_OF_THE_SLEEPLESS,
    &GRUUL_TURF,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
