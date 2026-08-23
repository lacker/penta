//! New Phyrexia cards used to exercise Phyrexian mana.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

static GUT_SHOT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// NPH 1 — Karn Liberated
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KARN_LIBERATED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9287151-95df-4f5a-b32a-4b0aea825452"),
    "Karn Liberated",
    crate::card::CardArt::new("f9287151-95df-4f5a-b32a-4b0aea825452", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 2 — Apostle's Blessing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static APOSTLE_S_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f7c3571-925d-486e-80dd-bac47aa48283"),
    "Apostle's Blessing",
    crate::card::CardArt::new("9f7c3571-925d-486e-80dd-bac47aa48283", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 3 — Auriok Survivors
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURIOK_SURVIVORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("deffb601-6a53-4d88-a6af-686ce97eb4f0"),
    "Auriok Survivors",
    crate::card::CardArt::new("deffb601-6a53-4d88-a6af-686ce97eb4f0", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 4 — Blade Splicer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLADE_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8e56a28-713b-4a13-a601-1128cf117539"),
    "Blade Splicer",
    crate::card::CardArt::new("b8e56a28-713b-4a13-a601-1128cf117539", "Greg Staples"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 5 — Cathedral Membrane
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CATHEDRAL_MEMBRANE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07592731-68be-4218-bb2c-c2523c5a27f1"),
    "Cathedral Membrane",
    crate::card::CardArt::new("07592731-68be-4218-bb2c-c2523c5a27f1", "Richard Whitters"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 6 — Chancellor of the Annex
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHANCELLOR_OF_THE_ANNEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be1b482a-badb-4b9a-ab63-2e7944826aa0"),
    "Chancellor of the Annex",
    crate::card::CardArt::new("be1b482a-badb-4b9a-ab63-2e7944826aa0", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 7 — Dispatch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISPATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("496634f9-1271-4be7-bad5-364bb87a6962"),
    "Dispatch",
    crate::card::CardArt::new("496634f9-1271-4be7-bad5-364bb87a6962", "Erica Yang"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 8 — Due Respect
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DUE_RESPECT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7159850-964b-4f12-957f-614eb0570544"),
    "Due Respect",
    crate::card::CardArt::new("a7159850-964b-4f12-957f-614eb0570544", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 9 — Elesh Norn, Grand Cenobite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELESH_NORN_GRAND_CENOBITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b66390d6-1649-4bfa-92d3-77664650d552"),
    "Elesh Norn, Grand Cenobite",
    crate::card::CardArt::new("b66390d6-1649-4bfa-92d3-77664650d552", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 10 — Exclusion Ritual
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXCLUSION_RITUAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e3b826a-7349-45ae-89bf-675fea7ce8e3"),
    "Exclusion Ritual",
    crate::card::CardArt::new("9e3b826a-7349-45ae-89bf-675fea7ce8e3", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 11 — Forced Worship
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORCED_WORSHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e050701d-4609-470d-85ff-4b7638893c6a"),
    "Forced Worship",
    crate::card::CardArt::new("e050701d-4609-470d-85ff-4b7638893c6a", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 12 — Inquisitor Exarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INQUISITOR_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49e241a0-a027-494b-8187-6ecb006d1d33"),
    "Inquisitor Exarch",
    crate::card::CardArt::new("49e241a0-a027-494b-8187-6ecb006d1d33", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 13 — Lost Leonin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOST_LEONIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8209fa5d-2c0e-4827-813b-fff123533f16"),
    "Lost Leonin",
    crate::card::CardArt::new("8209fa5d-2c0e-4827-813b-fff123533f16", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 14 — Loxodon Convert
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOXODON_CONVERT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00c050c3-4f50-4bb6-8477-6737887ca10d"),
    "Loxodon Convert",
    crate::card::CardArt::new("00c050c3-4f50-4bb6-8477-6737887ca10d", "Adrian Smith"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 15 — Marrow Shards
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MARROW_SHARDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53ca60ee-e54b-4a28-b6a6-7bf3503c35b4"),
    "Marrow Shards",
    crate::card::CardArt::new("53ca60ee-e54b-4a28-b6a6-7bf3503c35b4", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 16 — Master Splicer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("859d2b91-63af-4700-8ca5-b1756aa6639b"),
    "Master Splicer",
    crate::card::CardArt::new("859d2b91-63af-4700-8ca5-b1756aa6639b", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 17 — Norn's Annex
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NORN_S_ANNEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a64073f2-99f5-4dc7-9403-e7cb94ce0e60"),
    "Norn's Annex",
    crate::card::CardArt::new("a64073f2-99f5-4dc7-9403-e7cb94ce0e60", "James Paick"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 18 — Phyrexian Unlife
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_UNLIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4a1e16a-39f0-47ab-aba8-73e82ba9ab18"),
    "Phyrexian Unlife",
    crate::card::CardArt::new("b4a1e16a-39f0-47ab-aba8-73e82ba9ab18", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 19 — Porcelain Legionnaire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PORCELAIN_LEGIONNAIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2616aa0e-8413-4e63-877c-bffd5263f552"),
    "Porcelain Legionnaire",
    crate::card::CardArt::new("2616aa0e-8413-4e63-877c-bffd5263f552", "Eric Deschamps"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 20 — Puresteel Paladin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PURESTEEL_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca100248-fcd6-41ed-8d75-bcb473845edd"),
    "Puresteel Paladin",
    crate::card::CardArt::new("ca100248-fcd6-41ed-8d75-bcb473845edd", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 21 — Remember the Fallen
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REMEMBER_THE_FALLEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d9b8325-2a28-4312-b778-40087f8ea778"),
    "Remember the Fallen",
    crate::card::CardArt::new("6d9b8325-2a28-4312-b778-40087f8ea778", "Eric Deschamps"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 22 — Sensor Splicer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SENSOR_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79076264-d71c-4b30-aac9-702a4d229933"),
    "Sensor Splicer",
    crate::card::CardArt::new("79076264-d71c-4b30-aac9-702a4d229933", "Izzy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 23 — Shattered Angel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTERED_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("012f94e9-91cd-48da-873f-2da2b03a4965"),
    "Shattered Angel",
    crate::card::CardArt::new("012f94e9-91cd-48da-873f-2da2b03a4965", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 24 — Shriek Raptor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEK_RAPTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73071a3b-9329-418e-9285-fa4765463d1f"),
    "Shriek Raptor",
    crate::card::CardArt::new("73071a3b-9329-418e-9285-fa4765463d1f", "Efrem Palacios"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 25 — Suture Priest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUTURE_PRIEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31432e98-86cd-42ea-ad37-eb4383dc6a81"),
    "Suture Priest",
    crate::card::CardArt::new("31432e98-86cd-42ea-ad37-eb4383dc6a81", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 26 — War Report
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_REPORT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d837262-cd5d-4fc9-96dd-39ed04166883"),
    "War Report",
    crate::card::CardArt::new("6d837262-cd5d-4fc9-96dd-39ed04166883", "Mike Bierek"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 27 — Argent Mutation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARGENT_MUTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("507fa5fd-2aa5-4721-a059-2c8c3056a4ca"),
    "Argent Mutation",
    crate::card::CardArt::new(
        "507fa5fd-2aa5-4721-a059-2c8c3056a4ca",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 28 — Arm with Aether
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARM_WITH_AETHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0878b20-315d-49fa-a4d7-232ba1ed6b0d"),
    "Arm with Aether",
    crate::card::CardArt::new("a0878b20-315d-49fa-a4d7-232ba1ed6b0d", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 29 — Blighted Agent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLIGHTED_AGENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cddaebde-a060-4510-8c97-68432d931987"),
    "Blighted Agent",
    crate::card::CardArt::new("cddaebde-a060-4510-8c97-68432d931987", "Anthony Francisco"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 30 — Chained Throatseeker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHAINED_THROATSEEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a7bb447-c2b0-429e-bf82-02d6a966fe73"),
    "Chained Throatseeker",
    crate::card::CardArt::new("3a7bb447-c2b0-429e-bf82-02d6a966fe73", "Stephan Martiniere"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 31 — Chancellor of the Spires
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHANCELLOR_OF_THE_SPIRES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1e06e16-96fa-4611-b4a9-512eeeeddd3c"),
    "Chancellor of the Spires",
    crate::card::CardArt::new("b1e06e16-96fa-4611-b4a9-512eeeeddd3c", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 32 — Corrupted Resolve
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CORRUPTED_RESOLVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28432161-023b-4a98-b92a-55dc6d936cd1"),
    "Corrupted Resolve",
    crate::card::CardArt::new("28432161-023b-4a98-b92a-55dc6d936cd1", "Greg Staples"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 33 — Deceiver Exarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DECEIVER_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f123ad6-fe84-4fed-9c0f-6b41921e9c26"),
    "Deceiver Exarch",
    crate::card::CardArt::new("1f123ad6-fe84-4fed-9c0f-6b41921e9c26", "Izzy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 34 — Defensive Stance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENSIVE_STANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0670653-d4fe-4fac-b769-d19ca4698c97"),
    "Defensive Stance",
    crate::card::CardArt::new("d0670653-d4fe-4fac-b769-d19ca4698c97", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 35 — Gitaxian Probe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GITAXIAN_PROBE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("995486ce-58bb-4753-a812-0ca73ef1a235"),
    "Gitaxian Probe",
    crate::card::CardArt::new("995486ce-58bb-4753-a812-0ca73ef1a235", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 36 — Impaler Shrike
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IMPALER_SHRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91e1f8b5-4792-457d-b3de-1d4874ddf72e"),
    "Impaler Shrike",
    crate::card::CardArt::new("91e1f8b5-4792-457d-b3de-1d4874ddf72e", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 37 — Jin-Gitaxias, Core Augur
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JIN_GITAXIAS_CORE_AUGUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd46fc9f-5b92-44d7-8940-2f39b0962b8f"),
    "Jin-Gitaxias, Core Augur",
    crate::card::CardArt::new("bd46fc9f-5b92-44d7-8940-2f39b0962b8f", "Eric Deschamps"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 38 — Mental Misstep
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MENTAL_MISSTEP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61e9c6df-1c84-4eab-9076-a4feb6347c10"),
    "Mental Misstep",
    crate::card::CardArt::new("61e9c6df-1c84-4eab-9076-a4feb6347c10", "Erica Yang"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 39 — Mindculling
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MINDCULLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6faf4372-6fb5-48aa-9b94-b0e77c867116"),
    "Mindculling",
    crate::card::CardArt::new("6faf4372-6fb5-48aa-9b94-b0e77c867116", "Cos Koniotis"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 40 — Numbing Dose
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NUMBING_DOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f28a0f4-43e1-46df-8b6a-d588c5cceb88"),
    "Numbing Dose",
    crate::card::CardArt::new("8f28a0f4-43e1-46df-8b6a-d588c5cceb88", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 41 — Phyrexian Ingester
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_INGESTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("376e9829-23eb-4b43-9ec7-246cb3156e95"),
    "Phyrexian Ingester",
    crate::card::CardArt::new("376e9829-23eb-4b43-9ec7-246cb3156e95", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 42 — Phyrexian Metamorph
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_METAMORPH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8903546d-4f9a-4e90-8dd8-5ab068d40907"),
    "Phyrexian Metamorph",
    crate::card::CardArt::new(
        "d2e27911-87cb-49a0-a34f-6afe4bddd592",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 43 — Psychic Barrier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_BARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1cba7d67-5c6c-4738-8907-7cce503e3180"),
    "Psychic Barrier",
    crate::card::CardArt::new("1cba7d67-5c6c-4738-8907-7cce503e3180", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 44 — Psychic Surgery
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_SURGERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51ea9a6d-d6ca-48cb-adac-958ad0e7440c"),
    "Psychic Surgery",
    crate::card::CardArt::new("51ea9a6d-d6ca-48cb-adac-958ad0e7440c", "Anthony Francisco"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 45 — Spined Thopter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPINED_THOPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd27f71a-cd22-4b5e-9536-3e160111875a"),
    "Spined Thopter",
    crate::card::CardArt::new("bd27f71a-cd22-4b5e-9536-3e160111875a", "Pete Venters"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 46 — Spire Monitor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRE_MONITOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("189f83aa-264b-4d09-b45f-099597a789d4"),
    "Spire Monitor",
    crate::card::CardArt::new("189f83aa-264b-4d09-b45f-099597a789d4", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 47 — Tezzeret's Gambit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEZZERET_S_GAMBIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fff5a09e-9276-44b8-b374-4b84aebd47cc"),
    "Tezzeret's Gambit",
    crate::card::CardArt::new("fff5a09e-9276-44b8-b374-4b84aebd47cc", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 48 — Vapor Snag
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAPOR_SNAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70305148-23bd-41dd-9de5-13cf5ae591ae"),
    "Vapor Snag",
    crate::card::CardArt::new("70305148-23bd-41dd-9de5-13cf5ae591ae", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 49 — Viral Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRAL_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d89b312b-cc90-4f08-ae2e-043a79e51156"),
    "Viral Drake",
    crate::card::CardArt::new("d89b312b-cc90-4f08-ae2e-043a79e51156", "Lars Grant-West"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 50 — Wing Splicer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WING_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2dbfb1b-092c-44a3-932d-a8b27be0a72b"),
    "Wing Splicer",
    crate::card::CardArt::new("e2dbfb1b-092c-44a3-932d-a8b27be0a72b", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 51 — Xenograft
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static XENOGRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f52f08e1-b234-42e4-8f1f-485a4f6edb3b"),
    "Xenograft",
    crate::card::CardArt::new("f52f08e1-b234-42e4-8f1f-485a4f6edb3b", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 52 — Blind Zealot
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLIND_ZEALOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bd04df1-5131-455d-b497-fcce4f9af552"),
    "Blind Zealot",
    crate::card::CardArt::new(
        "9bd04df1-5131-455d-b497-fcce4f9af552",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 53 — Caress of Phyrexia
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARESS_OF_PHYREXIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ef987ad-a3dc-4ef5-90ec-9a8cfa95965b"),
    "Caress of Phyrexia",
    crate::card::CardArt::new("5ef987ad-a3dc-4ef5-90ec-9a8cfa95965b", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 54 — Chancellor of the Dross
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHANCELLOR_OF_THE_DROSS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eec6d85e-6263-44b4-a91f-d51585c561c2"),
    "Chancellor of the Dross",
    crate::card::CardArt::new("eec6d85e-6263-44b4-a91f-d51585c561c2", "Stephan Martiniere"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 55 — Dementia Bat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEMENTIA_BAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72ae22c3-2dea-463e-894a-188657849909"),
    "Dementia Bat",
    crate::card::CardArt::new("72ae22c3-2dea-463e-894a-188657849909", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 56 — Despise
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DESPISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee7bfcd3-9f2b-41f5-93b4-8c1ee6ba4d88"),
    "Despise",
    crate::card::CardArt::new("ee7bfcd3-9f2b-41f5-93b4-8c1ee6ba4d88", "Terese Nielsen"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 57 — Dismember
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISMEMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("064dfdeb-485f-473e-9fa0-8fdb7638cdc6"),
    "Dismember",
    crate::card::CardArt::new("064dfdeb-485f-473e-9fa0-8fdb7638cdc6", "Terese Nielsen"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 58 — Enslave
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENSLAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c6283e1-e4f1-4ff6-be01-b66ab623e0ac"),
    "Enslave",
    crate::card::CardArt::new("17c2f5f0-1f37-4f51-9c10-c02e2ef7d4ee", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 59 — Entomber Exarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENTOMBER_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f58020e-6d4d-474d-8d4b-cfb7d5a5e9a8"),
    "Entomber Exarch",
    crate::card::CardArt::new("7f58020e-6d4d-474d-8d4b-cfb7d5a5e9a8", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 60 — Evil Presence (reprint)

// NPH 61 — Geth's Verdict
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GETH_S_VERDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a20b5a2-8613-49ed-b5cc-7cae9d0e0850"),
    "Geth's Verdict",
    crate::card::CardArt::new("7a20b5a2-8613-49ed-b5cc-7cae9d0e0850", "Whit Brachna"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 62 — Glistening Oil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLISTENING_OIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("483e99fd-7e48-400d-9817-451089089e0c"),
    "Glistening Oil",
    crate::card::CardArt::new("483e99fd-7e48-400d-9817-451089089e0c", "Steven Belledin"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 63 — Grim Affliction
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRIM_AFFLICTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d5c8ba8-d9f4-440c-8e0b-93699df6343e"),
    "Grim Affliction",
    crate::card::CardArt::new("9d5c8ba8-d9f4-440c-8e0b-93699df6343e", "Erica Yang"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 64 — Ichor Explosion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICHOR_EXPLOSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b207e2f-4604-43c5-bb35-a877e35ddd81"),
    "Ichor Explosion",
    crate::card::CardArt::new("0b207e2f-4604-43c5-bb35-a877e35ddd81", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 65 — Life's Finale
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIFE_S_FINALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ffd3fbd2-87c7-4f08-baaa-91d61c1114da"),
    "Life's Finale",
    crate::card::CardArt::new("ffd3fbd2-87c7-4f08-baaa-91d61c1114da", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 66 — Mortis Dogs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MORTIS_DOGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cae1f40-0e43-41d8-bc5c-aa9873f7d7d5"),
    "Mortis Dogs",
    crate::card::CardArt::new("3cae1f40-0e43-41d8-bc5c-aa9873f7d7d5", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 67 — Parasitic Implant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PARASITIC_IMPLANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e34f1bf3-9f3a-47f0-9761-8b2356328a39"),
    "Parasitic Implant",
    crate::card::CardArt::new("e34f1bf3-9f3a-47f0-9761-8b2356328a39", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 68 — Phyrexian Obliterator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_OBLITERATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44c4476d-58f9-420d-9545-f5d580c589de"),
    "Phyrexian Obliterator",
    crate::card::CardArt::new("44c4476d-58f9-420d-9545-f5d580c589de", "Todd Lockwood"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 69 — Pith Driller
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PITH_DRILLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28e960c6-6da0-4679-87eb-55bac890e0c6"),
    "Pith Driller",
    crate::card::CardArt::new("28e960c6-6da0-4679-87eb-55bac890e0c6", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 70 — Postmortem Lunge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static POSTMORTEM_LUNGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5f8b46e-1ad3-4c6e-aa63-376f2d222d46"),
    "Postmortem Lunge",
    crate::card::CardArt::new("d5f8b46e-1ad3-4c6e-aa63-376f2d222d46", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 71 — Praetor's Grasp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRAETOR_S_GRASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9588be49-d9b5-4491-a5a0-10bcadc9f8b3"),
    "Praetor's Grasp",
    crate::card::CardArt::new("9588be49-d9b5-4491-a5a0-10bcadc9f8b3", "Steve Argyle"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 72 — Reaper of Sheoldred
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REAPER_OF_SHEOLDRED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a300a645-aec6-4cda-8c11-1e8a6af056ff"),
    "Reaper of Sheoldred",
    crate::card::CardArt::new("a300a645-aec6-4cda-8c11-1e8a6af056ff", "Stephan Martiniere"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 73 — Sheoldred, Whispering One
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHEOLDRED_WHISPERING_ONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72ddbbac-9914-44ff-b4d4-60989031744e"),
    "Sheoldred, Whispering One",
    crate::card::CardArt::new(
        "3bb8347b-8663-40b8-bdfb-411236d2efc8",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 74 — Surgical Extraction
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SURGICAL_EXTRACTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("114834d8-4da5-48b9-9ac7-5e3e4b7ddf2d"),
    "Surgical Extraction",
    crate::card::CardArt::new("dca7e072-edb5-4f7e-bdec-a3a393053c80", "Steven Belledin"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 75 — Toxic Nim
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOXIC_NIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5823990c-8d40-4352-8d34-74332934adb2"),
    "Toxic Nim",
    crate::card::CardArt::new("5823990c-8d40-4352-8d34-74332934adb2", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 76 — Vault Skirge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAULT_SKIRGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f254239c-c07a-4c41-98f7-8f4de539c73e"),
    "Vault Skirge",
    crate::card::CardArt::new("f254239c-c07a-4c41-98f7-8f4de539c73e", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 77 — Whispering Specter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHISPERING_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcb1b486-e336-4e88-b635-b6ff18cb4841"),
    "Whispering Specter",
    crate::card::CardArt::new("bcb1b486-e336-4e88-b635-b6ff18cb4841", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 78 — Act of Aggression
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ACT_OF_AGGRESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61a9f30b-d154-49a4-ad6b-f05601992de3"),
    "Act of Aggression",
    crate::card::CardArt::new("61a9f30b-d154-49a4-ad6b-f05601992de3", "Whit Brachna"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 79 — Artillerize
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARTILLERIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("034522ae-f531-44d9-b186-ada046ce0abc"),
    "Artillerize",
    crate::card::CardArt::new("034522ae-f531-44d9-b186-ada046ce0abc", "Johann Bodin"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 80 — Bludgeon Brawl
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLUDGEON_BRAWL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a30fa96d-64d1-423e-a62e-d43453ea838d"),
    "Bludgeon Brawl",
    crate::card::CardArt::new("a30fa96d-64d1-423e-a62e-d43453ea838d", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 81 — Chancellor of the Forge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHANCELLOR_OF_THE_FORGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd3520a7-a55f-4c00-b4f1-c1c154adfc8f"),
    "Chancellor of the Forge",
    crate::card::CardArt::new("dd3520a7-a55f-4c00-b4f1-c1c154adfc8f", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 82 — Fallen Ferromancer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FALLEN_FERROMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b200986-f553-4156-8f5e-37678db09687"),
    "Fallen Ferromancer",
    crate::card::CardArt::new("7b200986-f553-4156-8f5e-37678db09687", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 83 — Flameborn Viron
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMEBORN_VIRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9601ea62-a609-4bc5-a2f0-f7615b4dd5fa"),
    "Flameborn Viron",
    crate::card::CardArt::new("9601ea62-a609-4bc5-a2f0-f7615b4dd5fa", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 84 — Furnace Scamp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FURNACE_SCAMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97538294-058c-47d4-b7a8-4db3753a6628"),
    "Furnace Scamp",
    crate::card::CardArt::new("97538294-058c-47d4-b7a8-4db3753a6628", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 85 — Geosurge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GEOSURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("118b7aa3-bb05-4691-978e-51486435bf05"),
    "Geosurge",
    crate::card::CardArt::new("118b7aa3-bb05-4691-978e-51486435bf05", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 86 — Gut Shot
pub(in crate::card::sets) static GUT_SHOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a54a2a30-b96a-49c7-9151-1f4b0d4a4413"),
    "Gut Shot",
    CardArt::new("a54a2a30-b96a-49c7-9151-1f4b0d4a4413", "Greg Staples"),
    CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{R/P}")).with_ability(AbilityDef::spell_with_targets(
        "Gut Shot deals 1 damage to any target.",
        &GUT_SHOT_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    )),
);

// NPH 87 — Invader Parasite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INVADER_PARASITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89a8c53f-2cb0-41ea-8391-c32667f17c30"),
    "Invader Parasite",
    crate::card::CardArt::new("89a8c53f-2cb0-41ea-8391-c32667f17c30", "Volkan Baǵa"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 88 — Moltensteel Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTENSTEEL_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13b78018-bfbe-43fa-809f-9b52a155e11c"),
    "Moltensteel Dragon",
    crate::card::CardArt::new("13b78018-bfbe-43fa-809f-9b52a155e11c", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 89 — Ogre Menial
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_MENIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6271c5c1-5f39-4908-b838-0f34c74e912e"),
    "Ogre Menial",
    crate::card::CardArt::new("6271c5c1-5f39-4908-b838-0f34c74e912e", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 90 — Priest of Urabrask
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRIEST_OF_URABRASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0a9f49c-f15c-4b2d-b6a5-8efc3c430d87"),
    "Priest of Urabrask",
    crate::card::CardArt::new("d0a9f49c-f15c-4b2d-b6a5-8efc3c430d87", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 91 — Rage Extractor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAGE_EXTRACTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8cebc2c-a46b-4459-b62b-7fce1a744b11"),
    "Rage Extractor",
    crate::card::CardArt::new("d8cebc2c-a46b-4459-b62b-7fce1a744b11", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 92 — Razor Swine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAZOR_SWINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fb022a3-5f9e-491e-8340-087e33f927d6"),
    "Razor Swine",
    crate::card::CardArt::new("2fb022a3-5f9e-491e-8340-087e33f927d6", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 93 — Ruthless Invasion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUTHLESS_INVASION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc2bbff9-af57-4858-9351-d148b8c4bc3a"),
    "Ruthless Invasion",
    crate::card::CardArt::new("bc2bbff9-af57-4858-9351-d148b8c4bc3a", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 94 — Scrapyard Salvo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPYARD_SALVO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a4874eb-635b-47f0-bbee-6bd8b26e2f10"),
    "Scrapyard Salvo",
    crate::card::CardArt::new("3a4874eb-635b-47f0-bbee-6bd8b26e2f10", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 95 — Slag Fiend
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLAG_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0d1ee33-e247-4ada-bb01-518611cd7d00"),
    "Slag Fiend",
    crate::card::CardArt::new("c0d1ee33-e247-4ada-bb01-518611cd7d00", "Mike Bierek"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 96 — Slash Panther
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLASH_PANTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f510946-34de-4c12-8998-f61887d1a0e1"),
    "Slash Panther",
    crate::card::CardArt::new("2f510946-34de-4c12-8998-f61887d1a0e1", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 97 — Tormentor Exarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TORMENTOR_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4886eb6a-0f6a-4ea7-8e85-4a27d1a6f03b"),
    "Tormentor Exarch",
    crate::card::CardArt::new("4886eb6a-0f6a-4ea7-8e85-4a27d1a6f03b", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 98 — Urabrask the Hidden
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URABRASK_THE_HIDDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b06fcab2-891e-4fa3-8583-068ba56c2e27"),
    "Urabrask the Hidden",
    crate::card::CardArt::new("b06fcab2-891e-4fa3-8583-068ba56c2e27", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 99 — Victorious Destruction
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VICTORIOUS_DESTRUCTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b81cb30-e9f8-41f3-a10b-26e0ba2503aa"),
    "Victorious Destruction",
    crate::card::CardArt::new("7b81cb30-e9f8-41f3-a10b-26e0ba2503aa", "Jung Park"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 100 — Volt Charge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOLT_CHARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa88011c-a19d-4faa-8da6-86b9980cd571"),
    "Volt Charge",
    crate::card::CardArt::new(
        "aa88011c-a19d-4faa-8da6-86b9980cd571",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 101 — Vulshok Refugee
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VULSHOK_REFUGEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b1615ec-21b3-4575-8b02-fd2bccb930ba"),
    "Vulshok Refugee",
    crate::card::CardArt::new("0b1615ec-21b3-4575-8b02-fd2bccb930ba", "Wayne Reynolds"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 102 — Whipflare
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPFLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a7e6c10-d066-4967-932f-5b6c8d74568b"),
    "Whipflare",
    crate::card::CardArt::new("5a7e6c10-d066-4967-932f-5b6c8d74568b", "Johann Bodin"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 103 — Beast Within
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BEAST_WITHIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce5b6d19-22e3-4f57-8f4d-a17e982286c7"),
    "Beast Within",
    crate::card::CardArt::new("ce5b6d19-22e3-4f57-8f4d-a17e982286c7", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 104 — Birthing Pod
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BIRTHING_POD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b768efa2-e56b-4a7e-ace8-d673f10e0714"),
    "Birthing Pod",
    crate::card::CardArt::new("b768efa2-e56b-4a7e-ace8-d673f10e0714", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 105 — Brutalizer Exarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRUTALIZER_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ddfa4ed-70fb-4e25-875d-df0f973f7294"),
    "Brutalizer Exarch",
    crate::card::CardArt::new("9ddfa4ed-70fb-4e25-875d-df0f973f7294", "Mark Zug"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 106 — Chancellor of the Tangle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHANCELLOR_OF_THE_TANGLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d129aa8-b637-451e-8123-5221e08cc2cc"),
    "Chancellor of the Tangle",
    crate::card::CardArt::new("6d129aa8-b637-451e-8123-5221e08cc2cc", "Steve Prescott"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 107 — Corrosive Gale
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CORROSIVE_GALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04a13825-ab9b-4ffd-9b59-6198181891b9"),
    "Corrosive Gale",
    crate::card::CardArt::new("04a13825-ab9b-4ffd-9b59-6198181891b9", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 108 — Death-Hood Cobra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_HOOD_COBRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5279ac25-8175-44ad-ab7b-dfa17e359a10"),
    "Death-Hood Cobra",
    crate::card::CardArt::new("5279ac25-8175-44ad-ab7b-dfa17e359a10", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 109 — Fresh Meat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FRESH_MEAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("000ce65b-5347-4a88-81af-be9053e4d3f3"),
    "Fresh Meat",
    crate::card::CardArt::new("000ce65b-5347-4a88-81af-be9053e4d3f3", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 110 — Glissa's Scorn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLISSA_S_SCORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f11187c1-de35-4e85-87c3-656f978b2d7e"),
    "Glissa's Scorn",
    crate::card::CardArt::new("f11187c1-de35-4e85-87c3-656f978b2d7e", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 111 — Glistener Elf
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLISTENER_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b94f4c6-b518-43b3-be52-e889d1f3ea38"),
    "Glistener Elf",
    crate::card::CardArt::new("8b94f4c6-b518-43b3-be52-e889d1f3ea38", "Steve Argyle"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 112 — Greenhilt Trainee
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GREENHILT_TRAINEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("370f8ef5-c809-43cc-903a-077fad33cd30"),
    "Greenhilt Trainee",
    crate::card::CardArt::new("370f8ef5-c809-43cc-903a-077fad33cd30", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 113 — Leeching Bite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEECHING_BITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c3bdbeb-c376-42bd-af2a-251cd7ac704c"),
    "Leeching Bite",
    crate::card::CardArt::new("1c3bdbeb-c376-42bd-af2a-251cd7ac704c", "Cos Koniotis"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 114 — Maul Splicer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAUL_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d2c6a6d-5b59-47d7-b290-df3640d9555f"),
    "Maul Splicer",
    crate::card::CardArt::new("2d2c6a6d-5b59-47d7-b290-df3640d9555f", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 115 — Melira, Sylvok Outcast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MELIRA_SYLVOK_OUTCAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e83851a1-e4e8-49ec-af5c-4efe86fa51ad"),
    "Melira, Sylvok Outcast",
    crate::card::CardArt::new("e83851a1-e4e8-49ec-af5c-4efe86fa51ad", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 116 — Mutagenic Growth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MUTAGENIC_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af2d23da-70a1-49ba-91bf-c110cc4bbedc"),
    "Mutagenic Growth",
    crate::card::CardArt::new("af2d23da-70a1-49ba-91bf-c110cc4bbedc", "Dave Kendall"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 117 — Mycosynth Fiend
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYCOSYNTH_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdcd1b8e-9f1f-48a3-b7a1-43a32cc03bb1"),
    "Mycosynth Fiend",
    crate::card::CardArt::new("bdcd1b8e-9f1f-48a3-b7a1-43a32cc03bb1", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 118 — Noxious Revival
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NOXIOUS_REVIVAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1bdd1243-1d14-496a-9b7a-0c5b34461361"),
    "Noxious Revival",
    crate::card::CardArt::new("1bdd1243-1d14-496a-9b7a-0c5b34461361", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 119 — Phyrexian Swarmlord
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_SWARMLORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a91dea7-9792-4714-82b0-ba2c06cef304"),
    "Phyrexian Swarmlord",
    crate::card::CardArt::new("8a91dea7-9792-4714-82b0-ba2c06cef304", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 120 — Rotted Hystrix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROTTED_HYSTRIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bcae97d-468a-4e16-bfed-d2946f64784c"),
    "Rotted Hystrix",
    crate::card::CardArt::new("7bcae97d-468a-4e16-bfed-d2946f64784c", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 121 — Spinebiter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPINEBITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfc79ac6-ffc6-4506-9dea-e20176f960ea"),
    "Spinebiter",
    crate::card::CardArt::new("cfc79ac6-ffc6-4506-9dea-e20176f960ea", "Jaime Jones"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 122 — Thundering Tanadon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERING_TANADON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2fab443-0f4b-45ea-8a6d-435b93803409"),
    "Thundering Tanadon",
    crate::card::CardArt::new("e2fab443-0f4b-45ea-8a6d-435b93803409", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 123 — Triumph of the Hordes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRIUMPH_OF_THE_HORDES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c16b90ff-d256-4ac6-b687-3430b8c80dd7"),
    "Triumph of the Hordes",
    crate::card::CardArt::new("c16b90ff-d256-4ac6-b687-3430b8c80dd7", "Izzy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 124 — Viridian Betrayers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRIDIAN_BETRAYERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc6ea52f-4b24-45ff-99e1-4d0e1bd42875"),
    "Viridian Betrayers",
    crate::card::CardArt::new("cc6ea52f-4b24-45ff-99e1-4d0e1bd42875", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 125 — Viridian Harvest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRIDIAN_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("666eb9a5-b105-45c1-be3e-7ac5cc650338"),
    "Viridian Harvest",
    crate::card::CardArt::new("666eb9a5-b105-45c1-be3e-7ac5cc650338", "Johann Bodin"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 126 — Vital Splicer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VITAL_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("273b982d-bca2-4418-8618-c711d28fc901"),
    "Vital Splicer",
    crate::card::CardArt::new("273b982d-bca2-4418-8618-c711d28fc901", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 127 — Vorinclex, Voice of Hunger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VORINCLEX_VOICE_OF_HUNGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0806adab-6a08-411b-b249-e1c58ade354b"),
    "Vorinclex, Voice of Hunger",
    crate::card::CardArt::new("0806adab-6a08-411b-b249-e1c58ade354b", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 128 — Jor Kadeen, the Prevailer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JOR_KADEEN_THE_PREVAILER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bfd8d7de-a2e1-4f83-85f9-7057eebf0c37"),
    "Jor Kadeen, the Prevailer",
    crate::card::CardArt::new("bfd8d7de-a2e1-4f83-85f9-7057eebf0c37", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 129 — Alloy Myr
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALLOY_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abd3350b-89fb-40b4-a942-28e0c8c274aa"),
    "Alloy Myr",
    crate::card::CardArt::new("abd3350b-89fb-40b4-a942-28e0c8c274aa", "Matt Cavotta"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 130 — Batterskull
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BATTERSKULL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd114ec3-d286-4c70-a122-3043bc53cc88"),
    "Batterskull",
    crate::card::CardArt::new("cd114ec3-d286-4c70-a122-3043bc53cc88", "Mark Zug"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 131 — Blinding Souleater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLINDING_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2220e9dd-f1d8-4a69-9df9-1322e4a5cdc7"),
    "Blinding Souleater",
    crate::card::CardArt::new("2220e9dd-f1d8-4a69-9df9-1322e4a5cdc7", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 132 — Caged Sun
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAGED_SUN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("506597cc-48f9-4098-a229-2b3b3c0de944"),
    "Caged Sun",
    crate::card::CardArt::new("506597cc-48f9-4098-a229-2b3b3c0de944", "Scott Chou"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 133 — Conversion Chamber
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONVERSION_CHAMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14d5a8f3-05b6-4bb7-bbe1-e753e22cbb50"),
    "Conversion Chamber",
    crate::card::CardArt::new("14d5a8f3-05b6-4bb7-bbe1-e753e22cbb50", "Anthony Francisco"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 134 — Darksteel Relic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARKSTEEL_RELIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fd8c918-62d9-41be-a3e1-32ddac71b7e7"),
    "Darksteel Relic",
    crate::card::CardArt::new("0fd8c918-62d9-41be-a3e1-32ddac71b7e7", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 135 — Etched Monstrosity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ETCHED_MONSTROSITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff9c4451-dd17-4859-a31d-62ed2430c63c"),
    "Etched Monstrosity",
    crate::card::CardArt::new("ff9c4451-dd17-4859-a31d-62ed2430c63c", "Steven Belledin"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 136 — Gremlin Mine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GREMLIN_MINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccde7ebb-90de-4174-a1c5-75fc9384deaa"),
    "Gremlin Mine",
    crate::card::CardArt::new("ccde7ebb-90de-4174-a1c5-75fc9384deaa", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 137 — Hex Parasite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEX_PARASITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43502078-5349-4e29-8e7d-277654a9a71e"),
    "Hex Parasite",
    crate::card::CardArt::new("43502078-5349-4e29-8e7d-277654a9a71e", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 138 — Hovermyr
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOVERMYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95e4e445-8333-4cb4-b4fb-80957fae0b97"),
    "Hovermyr",
    crate::card::CardArt::new("95e4e445-8333-4cb4-b4fb-80957fae0b97", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 139 — Immolating Souleater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IMMOLATING_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abbaf916-067d-4834-a55c-b400fe0d8c1f"),
    "Immolating Souleater",
    crate::card::CardArt::new("abbaf916-067d-4834-a55c-b400fe0d8c1f", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 140 — Insatiable Souleater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INSATIABLE_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("171d5213-5bb4-4f5b-9ddd-e2a7ac092ec6"),
    "Insatiable Souleater",
    crate::card::CardArt::new("171d5213-5bb4-4f5b-9ddd-e2a7ac092ec6", "Dave Kendall"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 141 — Isolation Cell
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ISOLATION_CELL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5e72c64-cb0e-4a04-97d0-3537bb0420cd"),
    "Isolation Cell",
    crate::card::CardArt::new("c5e72c64-cb0e-4a04-97d0-3537bb0420cd", "Adrian Smith"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 142 — Kiln Walker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KILN_WALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91678632-ebe6-41b6-9250-cd3ffd63663b"),
    "Kiln Walker",
    crate::card::CardArt::new("91678632-ebe6-41b6-9250-cd3ffd63663b", "Volkan Baǵa"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 143 — Lashwrithe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LASHWRITHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c418159-b5d1-48e9-9a31-707f49d6733b"),
    "Lashwrithe",
    crate::card::CardArt::new("8c418159-b5d1-48e9-9a31-707f49d6733b", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 144 — Mindcrank
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MINDCRANK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d13a5ae0-d76a-4430-98c1-47a19e615e2c"),
    "Mindcrank",
    crate::card::CardArt::new("d13a5ae0-d76a-4430-98c1-47a19e615e2c", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 145 — Mycosynth Wellspring
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYCOSYNTH_WELLSPRING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("097f7ab8-01fa-4699-943a-32075aecebc2"),
    "Mycosynth Wellspring",
    crate::card::CardArt::new("097f7ab8-01fa-4699-943a-32075aecebc2", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 146 — Myr Superion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_SUPERION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("290c6036-02a3-43fa-b0d4-af3818794c3c"),
    "Myr Superion",
    crate::card::CardArt::new(
        "290c6036-02a3-43fa-b0d4-af3818794c3c",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 147 — Necropouncer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NECROPOUNCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ed51dbc-bbec-4c78-a71e-26322a8d2439"),
    "Necropouncer",
    crate::card::CardArt::new("4ed51dbc-bbec-4c78-a71e-26322a8d2439", "Cos Koniotis"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 148 — Omen Machine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OMEN_MACHINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ff4e35f-2a82-4d3c-86c5-ae05a5abc4d7"),
    "Omen Machine",
    crate::card::CardArt::new("0ff4e35f-2a82-4d3c-86c5-ae05a5abc4d7", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 149 — Pestilent Souleater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PESTILENT_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a069cc07-55eb-4ddb-a548-cbf463d078d3"),
    "Pestilent Souleater",
    crate::card::CardArt::new("a069cc07-55eb-4ddb-a548-cbf463d078d3", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 150 — Phyrexian Hulk (reprint)

// NPH 151 — Pristine Talisman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRISTINE_TALISMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e30e622d-1e82-4954-8f7d-ee256d5606bf"),
    "Pristine Talisman",
    crate::card::CardArt::new("b31d96cf-7276-46c4-ad17-d6a5c85f1315", "Matt Cavotta"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 152 — Shrine of Boundless Growth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRINE_OF_BOUNDLESS_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2ec7b95-667f-43ed-b310-b657befd55a2"),
    "Shrine of Boundless Growth",
    crate::card::CardArt::new("b2ec7b95-667f-43ed-b310-b657befd55a2", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 153 — Shrine of Burning Rage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRINE_OF_BURNING_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1a8afef-fa50-4aeb-94de-a4d90b1e5631"),
    "Shrine of Burning Rage",
    crate::card::CardArt::new("d1a8afef-fa50-4aeb-94de-a4d90b1e5631", "Dave Kendall"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 154 — Shrine of Limitless Power
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRINE_OF_LIMITLESS_POWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61873223-f378-4478-9cf3-f1326eb76834"),
    "Shrine of Limitless Power",
    crate::card::CardArt::new("61873223-f378-4478-9cf3-f1326eb76834", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 155 — Shrine of Loyal Legions
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRINE_OF_LOYAL_LEGIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d13563c7-abe0-4760-9b4c-841de47dbc46"),
    "Shrine of Loyal Legions",
    crate::card::CardArt::new("d13563c7-abe0-4760-9b4c-841de47dbc46", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 156 — Shrine of Piercing Vision
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRINE_OF_PIERCING_VISION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b150924-f83c-410e-aaab-ff2d06c9d356"),
    "Shrine of Piercing Vision",
    crate::card::CardArt::new(
        "9b150924-f83c-410e-aaab-ff2d06c9d356",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 157 — Sickleslicer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SICKLESLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d44746d5-3d34-4480-b4cd-c66de72f0622"),
    "Sickleslicer",
    crate::card::CardArt::new("d44746d5-3d34-4480-b4cd-c66de72f0622", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 158 — Soul Conduit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_CONDUIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa7e4989-cba7-4e0c-bb9d-140af6c006c3"),
    "Soul Conduit",
    crate::card::CardArt::new("aa7e4989-cba7-4e0c-bb9d-140af6c006c3", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 159 — Spellskite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLSKITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a84bada-ed6a-4e97-8a0c-05b7cb32d66f"),
    "Spellskite",
    crate::card::CardArt::new("1a84bada-ed6a-4e97-8a0c-05b7cb32d66f", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 160 — Surge Node
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SURGE_NODE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12414fc0-bb24-4244-baf4-adad0125376e"),
    "Surge Node",
    crate::card::CardArt::new("12414fc0-bb24-4244-baf4-adad0125376e", "Lars Grant-West"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 161 — Sword of War and Peace
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SWORD_OF_WAR_AND_PEACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fab5bc6c-8943-4078-866a-5d02f9be0eef"),
    "Sword of War and Peace",
    crate::card::CardArt::new("fab5bc6c-8943-4078-866a-5d02f9be0eef", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 162 — Torpor Orb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TORPOR_ORB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("953610f6-ea96-4e71-969f-50ecac09c091"),
    "Torpor Orb",
    crate::card::CardArt::new("953610f6-ea96-4e71-969f-50ecac09c091", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 163 — Trespassing Souleater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRESPASSING_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5263269-9f64-43de-9e82-408644dbc628"),
    "Trespassing Souleater",
    crate::card::CardArt::new("b5263269-9f64-43de-9e82-408644dbc628", "Scott Chou"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 164 — Unwinding Clock
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNWINDING_CLOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("495d520b-7560-4ecb-ae62-143eeec5682f"),
    "Unwinding Clock",
    crate::card::CardArt::new("495d520b-7560-4ecb-ae62-143eeec5682f", "Mike Bierek"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 165 — Phyrexia's Core
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIA_S_CORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db36c8e7-0c13-4f0d-9947-68cb0e9ea239"),
    "Phyrexia's Core",
    crate::card::CardArt::new("db36c8e7-0c13-4f0d-9947-68cb0e9ea239", "Franz Vohwinkel"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 166 — Plains (reprint)

// NPH 167 — Plains (alternate printing)

// NPH 168 — Island (reprint)

// NPH 169 — Island (alternate printing)

// NPH 170 — Swamp (reprint)

// NPH 171 — Swamp (alternate printing)

// NPH 172 — Mountain (reprint)

// NPH 173 — Mountain (alternate printing)

// NPH 174 — Forest (reprint)

// NPH 175 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KARN_LIBERATED,
    &APOSTLE_S_BLESSING,
    &AURIOK_SURVIVORS,
    &BLADE_SPLICER,
    &CATHEDRAL_MEMBRANE,
    &CHANCELLOR_OF_THE_ANNEX,
    &DISPATCH,
    &DUE_RESPECT,
    &ELESH_NORN_GRAND_CENOBITE,
    &EXCLUSION_RITUAL,
    &FORCED_WORSHIP,
    &INQUISITOR_EXARCH,
    &LOST_LEONIN,
    &LOXODON_CONVERT,
    &MARROW_SHARDS,
    &MASTER_SPLICER,
    &NORN_S_ANNEX,
    &PHYREXIAN_UNLIFE,
    &PORCELAIN_LEGIONNAIRE,
    &PURESTEEL_PALADIN,
    &REMEMBER_THE_FALLEN,
    &SENSOR_SPLICER,
    &SHATTERED_ANGEL,
    &SHRIEK_RAPTOR,
    &SUTURE_PRIEST,
    &WAR_REPORT,
    &ARGENT_MUTATION,
    &ARM_WITH_AETHER,
    &BLIGHTED_AGENT,
    &CHAINED_THROATSEEKER,
    &CHANCELLOR_OF_THE_SPIRES,
    &CORRUPTED_RESOLVE,
    &DECEIVER_EXARCH,
    &DEFENSIVE_STANCE,
    &GITAXIAN_PROBE,
    &IMPALER_SHRIKE,
    &JIN_GITAXIAS_CORE_AUGUR,
    &MENTAL_MISSTEP,
    &MINDCULLING,
    &NUMBING_DOSE,
    &PHYREXIAN_INGESTER,
    &PHYREXIAN_METAMORPH,
    &PSYCHIC_BARRIER,
    &PSYCHIC_SURGERY,
    &SPINED_THOPTER,
    &SPIRE_MONITOR,
    &TEZZERET_S_GAMBIT,
    &VAPOR_SNAG,
    &VIRAL_DRAKE,
    &WING_SPLICER,
    &XENOGRAFT,
    &BLIND_ZEALOT,
    &CARESS_OF_PHYREXIA,
    &CHANCELLOR_OF_THE_DROSS,
    &DEMENTIA_BAT,
    &DESPISE,
    &DISMEMBER,
    &ENSLAVE,
    &ENTOMBER_EXARCH,
    &GETH_S_VERDICT,
    &GLISTENING_OIL,
    &GRIM_AFFLICTION,
    &ICHOR_EXPLOSION,
    &LIFE_S_FINALE,
    &MORTIS_DOGS,
    &PARASITIC_IMPLANT,
    &PHYREXIAN_OBLITERATOR,
    &PITH_DRILLER,
    &POSTMORTEM_LUNGE,
    &PRAETOR_S_GRASP,
    &REAPER_OF_SHEOLDRED,
    &SHEOLDRED_WHISPERING_ONE,
    &SURGICAL_EXTRACTION,
    &TOXIC_NIM,
    &VAULT_SKIRGE,
    &WHISPERING_SPECTER,
    &ACT_OF_AGGRESSION,
    &ARTILLERIZE,
    &BLUDGEON_BRAWL,
    &CHANCELLOR_OF_THE_FORGE,
    &FALLEN_FERROMANCER,
    &FLAMEBORN_VIRON,
    &FURNACE_SCAMP,
    &GEOSURGE,
    &GUT_SHOT,
    &INVADER_PARASITE,
    &MOLTENSTEEL_DRAGON,
    &OGRE_MENIAL,
    &PRIEST_OF_URABRASK,
    &RAGE_EXTRACTOR,
    &RAZOR_SWINE,
    &RUTHLESS_INVASION,
    &SCRAPYARD_SALVO,
    &SLAG_FIEND,
    &SLASH_PANTHER,
    &TORMENTOR_EXARCH,
    &URABRASK_THE_HIDDEN,
    &VICTORIOUS_DESTRUCTION,
    &VOLT_CHARGE,
    &VULSHOK_REFUGEE,
    &WHIPFLARE,
    &BEAST_WITHIN,
    &BIRTHING_POD,
    &BRUTALIZER_EXARCH,
    &CHANCELLOR_OF_THE_TANGLE,
    &CORROSIVE_GALE,
    &DEATH_HOOD_COBRA,
    &FRESH_MEAT,
    &GLISSA_S_SCORN,
    &GLISTENER_ELF,
    &GREENHILT_TRAINEE,
    &LEECHING_BITE,
    &MAUL_SPLICER,
    &MELIRA_SYLVOK_OUTCAST,
    &MUTAGENIC_GROWTH,
    &MYCOSYNTH_FIEND,
    &NOXIOUS_REVIVAL,
    &PHYREXIAN_SWARMLORD,
    &ROTTED_HYSTRIX,
    &SPINEBITER,
    &THUNDERING_TANADON,
    &TRIUMPH_OF_THE_HORDES,
    &VIRIDIAN_BETRAYERS,
    &VIRIDIAN_HARVEST,
    &VITAL_SPLICER,
    &VORINCLEX_VOICE_OF_HUNGER,
    &JOR_KADEEN_THE_PREVAILER,
    &ALLOY_MYR,
    &BATTERSKULL,
    &BLINDING_SOULEATER,
    &CAGED_SUN,
    &CONVERSION_CHAMBER,
    &DARKSTEEL_RELIC,
    &ETCHED_MONSTROSITY,
    &GREMLIN_MINE,
    &HEX_PARASITE,
    &HOVERMYR,
    &IMMOLATING_SOULEATER,
    &INSATIABLE_SOULEATER,
    &ISOLATION_CELL,
    &KILN_WALKER,
    &LASHWRITHE,
    &MINDCRANK,
    &MYCOSYNTH_WELLSPRING,
    &MYR_SUPERION,
    &NECROPOUNCER,
    &OMEN_MACHINE,
    &PESTILENT_SOULEATER,
    &PRISTINE_TALISMAN,
    &SHRINE_OF_BOUNDLESS_GROWTH,
    &SHRINE_OF_BURNING_RAGE,
    &SHRINE_OF_LIMITLESS_POWER,
    &SHRINE_OF_LOYAL_LEGIONS,
    &SHRINE_OF_PIERCING_VISION,
    &SICKLESLICER,
    &SOUL_CONDUIT,
    &SPELLSKITE,
    &SURGE_NODE,
    &SWORD_OF_WAR_AND_PEACE,
    &TORPOR_ORB,
    &TRESPASSING_SOULEATER,
    &UNWINDING_CLOCK,
    &PHYREXIA_S_CORE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::EVIL_PRESENCE), // NPH 60
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::PHYREXIAN_HULK), // NPH 150
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::PLAINS),        // NPH 166
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),                       // NPH 167
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::ISLAND),        // NPH 168
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),                       // NPH 169
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SWAMP),         // NPH 170
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),                        // NPH 171
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::MOUNTAIN),      // NPH 172
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),                     // NPH 173
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FOREST),        // NPH 174
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),                       // NPH 175
];
