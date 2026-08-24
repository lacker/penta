//! Planeshift cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, CounterKind, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectBindingIndex;
use crate::mana_cost;

// PLS 1 — Aura Blast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("090f5ad6-e10e-49b3-8643-51a4e792517c"),
    "Aura Blast",
    crate::card::CardArt::new("090f5ad6-e10e-49b3-8643-51a4e792517c", "Ron Walotsky"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 2 — Aurora Griffin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURORA_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bfd6c695-1944-4bb0-a701-0daf47cdbcb4"),
    "Aurora Griffin",
    crate::card::CardArt::new("bfd6c695-1944-4bb0-a701-0daf47cdbcb4", "Ciruelo"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 3 — Disciple of Kangee
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISCIPLE_OF_KANGEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e268fe16-070b-4b78-9793-59755edb2fd5"),
    "Disciple of Kangee",
    crate::card::CardArt::new("e268fe16-070b-4b78-9793-59755edb2fd5", "Wayne England"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 4 — Dominaria's Judgment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOMINARIA_S_JUDGMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9703d090-b415-48e2-8158-dd8fc57ecc50"),
    "Dominaria's Judgment",
    crate::card::CardArt::new("9703d090-b415-48e2-8158-dd8fc57ecc50", "John Avon"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 5 — Guard Dogs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GUARD_DOGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba32eee7-10ba-4f0b-8a87-c3ecfa22ae41"),
    "Guard Dogs",
    crate::card::CardArt::new("ba32eee7-10ba-4f0b-8a87-c3ecfa22ae41", "Mike Raabe"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 6 — Heroic Defiance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEROIC_DEFIANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0dc1aa36-5d3b-4d25-9d54-937cdabf72a4"),
    "Heroic Defiance",
    crate::card::CardArt::new("0dc1aa36-5d3b-4d25-9d54-937cdabf72a4", "Terese Nielsen"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 7 — Hobble
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOBBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54c76a22-f9e3-408b-a5bd-403add57e31a"),
    "Hobble",
    crate::card::CardArt::new("54c76a22-f9e3-408b-a5bd-403add57e31a", "Alan Pollack"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 8 — Honorable Scout
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HONORABLE_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd311758-0352-4b7d-a24f-7f3f2b5d7b0f"),
    "Honorable Scout",
    crate::card::CardArt::new("bd311758-0352-4b7d-a24f-7f3f2b5d7b0f", "Mike Ploog"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 9 — Lashknife Barrier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LASHKNIFE_BARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2485c10d-de02-4be9-8119-afb2296e3317"),
    "Lashknife Barrier",
    crate::card::CardArt::new("2485c10d-de02-4be9-8119-afb2296e3317", "Paolo Parente"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 10 — March of Souls
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MARCH_OF_SOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f07dd0f1-b80b-4af0-ae76-907ec55ec7d5"),
    "March of Souls",
    crate::card::CardArt::new("f07dd0f1-b80b-4af0-ae76-907ec55ec7d5", "Marc Fishman"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 11 — Orim's Chant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_CHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("055afa78-b969-498f-a3ad-c792426e5ee6"),
    "Orim's Chant",
    crate::card::CardArt::new("055afa78-b969-498f-a3ad-c792426e5ee6", "Kev Walker"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 12 — Planeswalker's Mirth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_MIRTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0205d094-c846-4aa0-ade8-2a52c57b11da"),
    "Planeswalker's Mirth",
    crate::card::CardArt::new("0205d094-c846-4aa0-ade8-2a52c57b11da", "John Matson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 13 — Pollen Remedy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static POLLEN_REMEDY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9797c813-0cda-44ad-ae41-330e9bde9cb9"),
    "Pollen Remedy",
    crate::card::CardArt::new("9797c813-0cda-44ad-ae41-330e9bde9cb9", "Ben Thompson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 14 — Samite Elder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3c5dccc-2a48-4dcc-a796-fa6fdc11a14e"),
    "Samite Elder",
    crate::card::CardArt::new("b3c5dccc-2a48-4dcc-a796-fa6fdc11a14e", "Terese Nielsen"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 15 — Samite Pilgrim
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_PILGRIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c12529e4-f4b1-45be-8252-28783badbec5"),
    "Samite Pilgrim",
    crate::card::CardArt::new("c12529e4-f4b1-45be-8252-28783badbec5", "D. J. Cleland-Hura"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 16 — Sunscape Battlemage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a85e590f-0a4a-4ad0-b8ef-d3a18edadc05"),
    "Sunscape Battlemage",
    crate::card::CardArt::new("a85e590f-0a4a-4ad0-b8ef-d3a18edadc05", "Tony Szczudlo"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 17 — Sunscape Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9621f341-bf85-4b77-bf19-2fb013b4c955"),
    "Sunscape Familiar",
    crate::card::CardArt::new("9621f341-bf85-4b77-bf19-2fb013b4c955", "Brian Despain"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 18 — Surprise Deployment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SURPRISE_DEPLOYMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a26148b-b981-4af5-995b-52b1426737e3"),
    "Surprise Deployment",
    crate::card::CardArt::new("9a26148b-b981-4af5-995b-52b1426737e3", "Bradley Williams"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 19 — Voice of All
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOICE_OF_ALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75f37536-db3d-4726-9e45-b9108247d0e6"),
    "Voice of All",
    crate::card::CardArt::new("75f37536-db3d-4726-9e45-b9108247d0e6", "rk post"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 20 — Allied Strategies
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALLIED_STRATEGIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51d4f211-10e8-486d-b982-287ab0c060c9"),
    "Allied Strategies",
    crate::card::CardArt::new("51d4f211-10e8-486d-b982-287ab0c060c9", "Paolo Parente"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 21 — Arctic Merfolk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCTIC_MERFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86369fe5-d86d-4f4c-8f3d-dedc174f2032"),
    "Arctic Merfolk",
    crate::card::CardArt::new("86369fe5-d86d-4f4c-8f3d-dedc174f2032", "Ron Spears"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 22 — Confound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONFOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f3b7d39-ce98-48e2-b2bf-0d55b4d3102b"),
    "Confound",
    crate::card::CardArt::new("4f3b7d39-ce98-48e2-b2bf-0d55b4d3102b", "Doug Chaffee"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 23 — Dralnu's Pet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRALNU_S_PET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd5f4daf-7b54-4425-a93a-19532dfb83ca"),
    "Dralnu's Pet",
    crate::card::CardArt::new("cd5f4daf-7b54-4425-a93a-19532dfb83ca", "Glen Angus"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 24 — Ertai's Trickery
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_S_TRICKERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("544e3575-9fb6-41f7-a4e6-f8460dfae344"),
    "Ertai's Trickery",
    crate::card::CardArt::new("544e3575-9fb6-41f7-a4e6-f8460dfae344", "Kev Walker"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 25 — Escape Routes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ESCAPE_ROUTES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dbc9062e-ddd9-41ac-a88a-33f5a7b22103"),
    "Escape Routes",
    crate::card::CardArt::new("dbc9062e-ddd9-41ac-a88a-33f5a7b22103", "Marc Fishman"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 26 — Gainsay
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GAINSAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a70a2092-5048-49c0-9351-a3f882c2f56e"),
    "Gainsay",
    crate::card::CardArt::new("a70a2092-5048-49c0-9351-a3f882c2f56e", "Roger Raupp"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 27 — Hunting Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b0293a9-48fe-4018-bd25-3e02c227a3dd"),
    "Hunting Drake",
    crate::card::CardArt::new("5b0293a9-48fe-4018-bd25-3e02c227a3dd", "Wayne England"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 28 — Planar Overlay
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_OVERLAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1315fef0-234e-44f5-a7a3-bf3db78943c3"),
    "Planar Overlay",
    crate::card::CardArt::new("1315fef0-234e-44f5-a7a3-bf3db78943c3", "Ron Walotsky"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 29 — Planeswalker's Mischief
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_MISCHIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79aa232c-3f16-4c68-99dc-09a7aeef477b"),
    "Planeswalker's Mischief",
    crate::card::CardArt::new("79aa232c-3f16-4c68-99dc-09a7aeef477b", "Pete Venters"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 30 — Rushing River
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHING_RIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52ddf7bf-de9c-4657-8d5b-79869d36fa63"),
    "Rushing River",
    crate::card::CardArt::new("52ddf7bf-de9c-4657-8d5b-79869d36fa63", "Don Hazeltine"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 31 — Sea Snidd
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_SNIDD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca11015e-200b-488c-8bf5-662dcc03cd2d"),
    "Sea Snidd",
    crate::card::CardArt::new("ca11015e-200b-488c-8bf5-662dcc03cd2d", "Chippy"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 32 — Shifting Sky
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIFTING_SKY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1071726d-48f0-46d6-802b-dd9589489580"),
    "Shifting Sky",
    crate::card::CardArt::new("1071726d-48f0-46d6-802b-dd9589489580", "Jerry Tiritilli"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 33 — Sisay's Ingenuity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SISAY_S_INGENUITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbe20cc1-621a-4813-9bbb-ace006e173ff"),
    "Sisay's Ingenuity",
    crate::card::CardArt::new("bbe20cc1-621a-4813-9bbb-ace006e173ff", "Paolo Parente"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 34 — Sleeping Potion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLEEPING_POTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f79f4b2-71cd-4f78-a161-d75b162c745e"),
    "Sleeping Potion",
    crate::card::CardArt::new("6f79f4b2-71cd-4f78-a161-d75b162c745e", "Daren Bader"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 35 — Stormscape Battlemage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7d46a39d-c6f4-4281-b31f-f0a0c9fba887"),
    "Stormscape Battlemage",
    crate::card::CardArt::new(
        "7d46a39d-c6f4-4281-b31f-f0a0c9fba887",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 36 — Stormscape Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c831c42-77a0-4f4f-9628-ad630541cf66"),
    "Stormscape Familiar",
    crate::card::CardArt::new("4c831c42-77a0-4f4f-9628-ad630541cf66", "Heather Hudson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 37 — Sunken Hope
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNKEN_HOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f12ac0c-cfe6-4f08-b6df-20be4ce83e8c"),
    "Sunken Hope",
    crate::card::CardArt::new("5f12ac0c-cfe6-4f08-b6df-20be4ce83e8c", "Greg Staples"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 38 — Waterspout Elemental
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WATERSPOUT_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("425156e6-8eee-4bff-8f2f-86edd9a4f73b"),
    "Waterspout Elemental",
    crate::card::CardArt::new("425156e6-8eee-4bff-8f2f-86edd9a4f73b", "Mark Romanoski"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 39 — Bog Down
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_DOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8752a605-38f8-4d75-b122-063a788dff6e"),
    "Bog Down",
    crate::card::CardArt::new("8752a605-38f8-4d75-b122-063a788dff6e", "Andrew Goldhawk"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 40 — Dark Suspicions
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_SUSPICIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d518e2fd-7767-43d7-92e3-62a4a465154c"),
    "Dark Suspicions",
    crate::card::CardArt::new("d518e2fd-7767-43d7-92e3-62a4a465154c", "Matt Cavotta"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 41 — Death Bomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_BOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8a84715-c5dc-4a19-af6a-796c6ee912c2"),
    "Death Bomb",
    crate::card::CardArt::new("f8a84715-c5dc-4a19-af6a-796c6ee912c2", "Dan Frazier"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 42 — Diabolic Intent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DIABOLIC_INTENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76d1b5c5-cc47-465f-8549-4fd1ca4280df"),
    "Diabolic Intent",
    crate::card::CardArt::new("76d1b5c5-cc47-465f-8549-4fd1ca4280df", "Dave Dorman"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 43 — Exotic Disease
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXOTIC_DISEASE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e9624e5-79a2-41de-997b-12d871d4be66"),
    "Exotic Disease",
    crate::card::CardArt::new("4e9624e5-79a2-41de-997b-12d871d4be66", "Kev Walker"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 44 — Lord of the Undead
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_OF_THE_UNDEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a7f50f4-37a0-476e-8655-edba228aafd6"),
    "Lord of the Undead",
    crate::card::CardArt::new("0a7f50f4-37a0-476e-8655-edba228aafd6", "Brom"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 45 — Maggot Carrier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAGGOT_CARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab2c3dc4-bb49-4ec3-a6c8-4256d1939326"),
    "Maggot Carrier",
    crate::card::CardArt::new("ab2c3dc4-bb49-4ec3-a6c8-4256d1939326", "Ron Spencer"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 46 — Morgue Toad
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MORGUE_TOAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77d8ae73-70d1-4082-8581-5f74c1aaa63b"),
    "Morgue Toad",
    crate::card::CardArt::new("77d8ae73-70d1-4082-8581-5f74c1aaa63b", "Franz Vohwinkel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 47 — Nightscape Battlemage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5389643-4cc0-4a17-bc2d-7f9b76d30f9f"),
    "Nightscape Battlemage",
    crate::card::CardArt::new("d5389643-4cc0-4a17-bc2d-7f9b76d30f9f", "Andrew Goldhawk"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 48 — Nightscape Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24fa6853-09b0-4c9f-a138-9dd005780255"),
    "Nightscape Familiar",
    crate::card::CardArt::new("24fa6853-09b0-4c9f-a138-9dd005780255", "Jeff Easley"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 49 — Noxious Vapors
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NOXIOUS_VAPORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3cf9326-6e1c-4a05-abea-16d6b6cb2a6d"),
    "Noxious Vapors",
    crate::card::CardArt::new("e3cf9326-6e1c-4a05-abea-16d6b6cb2a6d", "Ben Thompson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 50 — Phyrexian Bloodstock
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_BLOODSTOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("785e1a67-af94-48e8-bb37-4999d1fb4c66"),
    "Phyrexian Bloodstock",
    crate::card::CardArt::new("785e1a67-af94-48e8-bb37-4999d1fb4c66", "Mark Tedin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 51 — Phyrexian Scuta
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_SCUTA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb57e656-c94e-4cc2-ae8d-9300f51f941f"),
    "Phyrexian Scuta",
    crate::card::CardArt::new("eb57e656-c94e-4cc2-ae8d-9300f51f941f", "Scott M. Fischer"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 52 — Planeswalker's Scorn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_SCORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ed08376-836f-4313-83d0-481895ead9da"),
    "Planeswalker's Scorn",
    crate::card::CardArt::new("8ed08376-836f-4313-83d0-481895ead9da", "Glen Angus"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 53 — Shriek of Dread
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEK_OF_DREAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54a7fb3b-8e81-4763-b2a1-7c2108a00afe"),
    "Shriek of Dread",
    crate::card::CardArt::new("54a7fb3b-8e81-4763-b2a1-7c2108a00afe", "Nelson DeCastro"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 54 — Sinister Strength
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SINISTER_STRENGTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afe487b8-c1ae-483d-bcd5-62c62b66a22e"),
    "Sinister Strength",
    crate::card::CardArt::new("afe487b8-c1ae-483d-bcd5-62c62b66a22e", "Terese Nielsen"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 55 — Slay
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eccda747-2680-4793-8a13-35e49b4de12f"),
    "Slay",
    crate::card::CardArt::new("eccda747-2680-4793-8a13-35e49b4de12f", "Ben Thompson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 56 — Volcano Imp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOLCANO_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8281cc6-2132-4f76-841e-d1ade9cafb84"),
    "Volcano Imp",
    crate::card::CardArt::new("a8281cc6-2132-4f76-841e-d1ade9cafb84", "Thomas M. Baxa"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 57 — Warped Devotion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WARPED_DEVOTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bce620f-799a-4ad8-9edb-6fb3d9ea1cc6"),
    "Warped Devotion",
    crate::card::CardArt::new("3bce620f-799a-4ad8-9edb-6fb3d9ea1cc6", "Dany Orizio"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 58 — Caldera Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CALDERA_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fcad32aa-2ce1-402d-a9d8-ad5c81fe4c5b"),
    "Caldera Kavu",
    crate::card::CardArt::new("fcad32aa-2ce1-402d-a9d8-ad5c81fe4c5b", "Arnie Swekel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 59 — Deadapult
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEADAPULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdc93b3d-bde4-422f-9edc-e337719be7b4"),
    "Deadapult",
    crate::card::CardArt::new("bdc93b3d-bde4-422f-9edc-e337719be7b4", "Mark Brill"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 60 — Flametongue Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMETONGUE_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e5056bca-bd90-4b50-8630-105558f8ef92"),
    "Flametongue Kavu",
    crate::card::CardArt::new("e5056bca-bd90-4b50-8630-105558f8ef92", "Pete Venters"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 61 — Goblin Game
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GAME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cbe6e7e5-ffea-4c6c-8a42-28e695029f24"),
    "Goblin Game",
    crate::card::CardArt::new("cbe6e7e5-ffea-4c6c-8a42-28e695029f24", "DiTerlizzi"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 62 — Implode
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IMPLODE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a76ee318-8126-4ebf-884d-8369ae8726ac"),
    "Implode",
    crate::card::CardArt::new("a76ee318-8126-4ebf-884d-8369ae8726ac", "Arnie Swekel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 63 — Insolence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INSOLENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8009a37-f966-4a71-9a2a-469127758dc6"),
    "Insolence",
    crate::card::CardArt::new("d8009a37-f966-4a71-9a2a-469127758dc6", "Carl Critchlow"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 64 — Kavu Recluse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_RECLUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f04ac02-3eff-4a66-8320-ee7b4357522f"),
    "Kavu Recluse",
    crate::card::CardArt::new("6f04ac02-3eff-4a66-8320-ee7b4357522f", "Aaron Boyd"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 65 — Keldon Mantle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_MANTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35bb73df-f488-468c-a9ad-72f52c8da3dc"),
    "Keldon Mantle",
    crate::card::CardArt::new("35bb73df-f488-468c-a9ad-72f52c8da3dc", "Rebecca Guay"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 66 — Magma Burst
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMA_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9752bc3-0bdf-4657-8750-73c8cbc8e83f"),
    "Magma Burst",
    crate::card::CardArt::new("d9752bc3-0bdf-4657-8750-73c8cbc8e83f", "Bradley Williams"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 67 — Mire Kavu
pub(in crate::card::sets) static MIRE_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccdd0086-eb27-48b3-91cb-a113aa1de102"),
    "Mire Kavu",
    crate::card::CardArt::new("ccdd0086-eb27-48b3-91cb-a113aa1de102", "Wayne England"),
    crate::card::CardSet::Planeshift,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Kavu"], 3, 2).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Swamp.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::controls_basic_land_type(
                    PlayerRelation::You,
                    BasicLandType::Swamp,
                ),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
    ),
);

// PLS 68 — Mogg Jailer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_JAILER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52513235-0e6c-40ea-8ead-a050e6da676e"),
    "Mogg Jailer",
    crate::card::CardArt::new("52513235-0e6c-40ea-8ead-a050e6da676e", "Mark Romanoski"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 69 — Mogg Sentry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_SENTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8536ec54-cebd-4d44-8e52-42344a3e6daa"),
    "Mogg Sentry",
    crate::card::CardArt::new(
        "8536ec54-cebd-4d44-8e52-42344a3e6daa",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 70 — Planeswalker's Fury
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_FURY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fa09e3a-bc7e-4292-aa5d-ce97c1b1f79f"),
    "Planeswalker's Fury",
    crate::card::CardArt::new(
        "6fa09e3a-bc7e-4292-aa5d-ce97c1b1f79f",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 71 — Singe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SINGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32323277-db9a-48a7-b9a4-8e6914386e26"),
    "Singe",
    crate::card::CardArt::new("32323277-db9a-48a7-b9a4-8e6914386e26", "John Avon"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 72 — Slingshot Goblin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLINGSHOT_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81825aef-bef7-46b7-bf52-29e32c1836b0"),
    "Slingshot Goblin",
    crate::card::CardArt::new("81825aef-bef7-46b7-bf52-29e32c1836b0", "Jeff Easley"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 73 — Strafe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRAFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec8b77cf-9c1e-4c8f-b452-295cc1570d0e"),
    "Strafe",
    crate::card::CardArt::new("ec8b77cf-9c1e-4c8f-b452-295cc1570d0e", "Jim Nelson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 74 — Tahngarth, Talruum Hero (alternate printing)

// PLS 74★ — Tahngarth, Talruum Hero
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TAHNGARTH_TALRUUM_HERO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6cdab0f9-7208-4555-b509-e61773ebc1f9"),
    "Tahngarth, Talruum Hero",
    crate::card::CardArt::new("6cdab0f9-7208-4555-b509-e61773ebc1f9", "Kev Walker"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 75 — Thunderscape Battlemage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d707243e-7f11-44bc-b8b8-af635ab1dc87"),
    "Thunderscape Battlemage",
    crate::card::CardArt::new("d707243e-7f11-44bc-b8b8-af635ab1dc87", "Mike Ploog"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 76 — Thunderscape Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26c9c0aa-9412-4320-aaee-e05369b8bc7b"),
    "Thunderscape Familiar",
    crate::card::CardArt::new("26c9c0aa-9412-4320-aaee-e05369b8bc7b", "Daren Bader"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 77 — Alpha Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALPHA_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("545ed916-59fc-4c60-9260-8c2dc88e67a1"),
    "Alpha Kavu",
    crate::card::CardArt::new("545ed916-59fc-4c60-9260-8c2dc88e67a1", "Matt Cavotta"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 78 — Amphibious Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AMPHIBIOUS_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37d94fb2-958c-487e-9f64-52d2771c6ea4"),
    "Amphibious Kavu",
    crate::card::CardArt::new("37d94fb2-958c-487e-9f64-52d2771c6ea4", "Wayne England"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 79 — Falling Timber
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FALLING_TIMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6e54c84d-ccc9-4c52-b02c-e0392e8fe447"),
    "Falling Timber",
    crate::card::CardArt::new("6e54c84d-ccc9-4c52-b02c-e0392e8fe447", "Eric Peterson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 80 — Gaea's Herald
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_HERALD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa52bc97-109a-4de5-b287-bce21dad6a9c"),
    "Gaea's Herald",
    crate::card::CardArt::new("aa52bc97-109a-4de5-b287-bce21dad6a9c", "Dan Frazier"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 81 — Gaea's Might
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_MIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67e5adce-7735-4fa5-aa14-8dce012e9fcc"),
    "Gaea's Might",
    crate::card::CardArt::new("67e5adce-7735-4fa5-aa14-8dce012e9fcc", "Ron Spencer"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 82 — Magnigoth Treefolk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNIGOTH_TREEFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90c2869b-43cf-4d5e-8a54-9ae200f5bff9"),
    "Magnigoth Treefolk",
    crate::card::CardArt::new("90c2869b-43cf-4d5e-8a54-9ae200f5bff9", "Peter Bollinger"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 83 — Mirrorwood Treefolk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRORWOOD_TREEFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba9a1c94-2b7f-4df7-8517-a122616d9ae4"),
    "Mirrorwood Treefolk",
    crate::card::CardArt::new("ba9a1c94-2b7f-4df7-8517-a122616d9ae4", "Arnie Swekel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 84 — Multani's Harmony
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_S_HARMONY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c76352ea-e3d2-4221-8ebe-e953301c35ab"),
    "Multani's Harmony",
    crate::card::CardArt::new("c76352ea-e3d2-4221-8ebe-e953301c35ab", "Darrell Riche"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 85 — Nemata, Grove Guardian
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NEMATA_GROVE_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c6a0ca4-5006-4c9b-91cd-e01d77e4fdc2"),
    "Nemata, Grove Guardian",
    crate::card::CardArt::new("8c6a0ca4-5006-4c9b-91cd-e01d77e4fdc2", "John Avon"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 86 — Planeswalker's Favor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_FAVOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3387540-93bf-451e-8e7a-fc78caab42b0"),
    "Planeswalker's Favor",
    crate::card::CardArt::new("b3387540-93bf-451e-8e7a-fc78caab42b0", "Rebecca Guay"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 87 — Primal Growth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMAL_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d4a3c83-faaa-4dd9-9349-abcaf09cc7a8"),
    "Primal Growth",
    crate::card::CardArt::new("1d4a3c83-faaa-4dd9-9349-abcaf09cc7a8", "rk post"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 88 — Pygmy Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b31c69ec-feb5-430a-a3e9-3a6f3fb8ee1c"),
    "Pygmy Kavu",
    crate::card::CardArt::new("b31c69ec-feb5-430a-a3e9-3a6f3fb8ee1c", "Greg Staples"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 89 — Quirion Dryad
pub(in crate::card::sets) static QUIRION_DRYAD: CardRecord = CardRecord::new_with_legacy_id(
    291,
    "Quirion Dryad",
    CardArt::new("f6841ae6-b15f-488e-9cae-2cc5ec668278", "Don Hazeltine"),
    CardSet::Planeshift,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a spell that's white, blue, black, or red, put a +1/+1 counter on this creature.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ObjectPredicateDef::Color(ManaColor::Red),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// PLS 90 — Quirion Explorer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_EXPLORER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("141a031d-f899-497b-adf7-4af142078085"),
    "Quirion Explorer",
    crate::card::CardArt::new("141a031d-f899-497b-adf7-4af142078085", "Ron Spears"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 91 — Root Greevil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROOT_GREEVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("306e3429-b3b4-4186-935b-18cfc308d22c"),
    "Root Greevil",
    crate::card::CardArt::new("306e3429-b3b4-4186-935b-18cfc308d22c", "Andrew Robinson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 92 — Skyshroud Blessing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0c10b16-97b1-4a36-b2b4-f0c28ead3eb4"),
    "Skyshroud Blessing",
    crate::card::CardArt::new("c0c10b16-97b1-4a36-b2b4-f0c28ead3eb4", "Jerry Tiritilli"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 93 — Stone Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36a1cdca-d48c-4936-ad6a-4610aeb991ce"),
    "Stone Kavu",
    crate::card::CardArt::new("36a1cdca-d48c-4936-ad6a-4610aeb991ce", "Adam Rex"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 94 — Thornscape Battlemage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13f24f89-3996-4740-a6c9-d26b8869554b"),
    "Thornscape Battlemage",
    crate::card::CardArt::new("13f24f89-3996-4740-a6c9-d26b8869554b", "Matt Cavotta"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 95 — Thornscape Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76c6e426-6165-4f8e-8766-de768ae13452"),
    "Thornscape Familiar",
    crate::card::CardArt::new("76c6e426-6165-4f8e-8766-de768ae13452", "Heather Hudson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 96 — Ancient Spider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75ca99de-57e7-47c4-b40a-6e41e3b18069"),
    "Ancient Spider",
    crate::card::CardArt::new("75ca99de-57e7-47c4-b40a-6e41e3b18069", "Greg Staples"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 97 — Cavern Harpy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAVERN_HARPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("adfb0804-50d6-4bca-8733-72e01030a543"),
    "Cavern Harpy",
    crate::card::CardArt::new("adfb0804-50d6-4bca-8733-72e01030a543", "Daren Bader"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 98 — Cloud Cover
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_COVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("943b3886-5556-474f-8dc1-18219e25abc3"),
    "Cloud Cover",
    crate::card::CardArt::new("943b3886-5556-474f-8dc1-18219e25abc3", "Marc Fishman"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 99 — Crosis's Charm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_S_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b59a9e75-9988-4040-a718-b1655fc20d11"),
    "Crosis's Charm",
    crate::card::CardArt::new("b59a9e75-9988-4040-a718-b1655fc20d11", "David Martin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 100 — Darigaaz's Charm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_S_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf4c9d6a-86eb-45be-9405-473eb263b94c"),
    "Darigaaz's Charm",
    crate::card::CardArt::new("cf4c9d6a-86eb-45be-9405-473eb263b94c", "David Martin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 101 — Daring Leap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARING_LEAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37ec6c4b-2de0-4759-a25d-007706cb18cc"),
    "Daring Leap",
    crate::card::CardArt::new("37ec6c4b-2de0-4759-a25d-007706cb18cc", "Paolo Parente"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 102 — Destructive Flow
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DESTRUCTIVE_FLOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7db86e34-c3ec-4a29-8779-81350a985644"),
    "Destructive Flow",
    crate::card::CardArt::new("7db86e34-c3ec-4a29-8779-81350a985644", "Don Hazeltine"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 103 — Doomsday Specter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOOMSDAY_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85206cc1-5484-40c6-b11d-b8d6fad4fc5c"),
    "Doomsday Specter",
    crate::card::CardArt::new("85206cc1-5484-40c6-b11d-b8d6fad4fc5c", "Donato Giancola"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 104 — Dralnu's Crusade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRALNU_S_CRUSADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a35d227-4489-4a0b-8f81-eb8e5949e1fc"),
    "Dralnu's Crusade",
    crate::card::CardArt::new("6a35d227-4489-4a0b-8f81-eb8e5949e1fc", "Arnie Swekel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 105 — Dromar's Charm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_S_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7a1894c-af4e-4530-960f-2225916be8cb"),
    "Dromar's Charm",
    crate::card::CardArt::new("c7a1894c-af4e-4530-960f-2225916be8cb", "David Martin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 106 — Eladamri's Call
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELADAMRI_S_CALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dcb79f39-5ef3-4ad6-9a43-04beb27d8480"),
    "Eladamri's Call",
    crate::card::CardArt::new("dcb79f39-5ef3-4ad6-9a43-04beb27d8480", "Kev Walker"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 107 — Ertai, the Corrupted
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_THE_CORRUPTED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66b950d9-8fef-4deb-b51b-26edb90abc56"),
    "Ertai, the Corrupted",
    crate::card::CardArt::new("66b950d9-8fef-4deb-b51b-26edb90abc56", "Mark Tedin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 107★ — Ertai, the Corrupted (alternate printing)

// PLS 108 — Fleetfoot Panther
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLEETFOOT_PANTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b70220d8-f81b-44a4-b92e-d66de8c1b4ce"),
    "Fleetfoot Panther",
    crate::card::CardArt::new("b70220d8-f81b-44a4-b92e-d66de8c1b4ce", "Mark Brill"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 109 — Gerrard's Command
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_S_COMMAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0fda263-b6a7-43e3-998a-72a9d84c4572"),
    "Gerrard's Command",
    crate::card::CardArt::new("d0fda263-b6a7-43e3-998a-72a9d84c4572", "Roger Raupp"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 110 — Horned Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HORNED_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ecd79fbf-626d-4549-917b-435f16b973d9"),
    "Horned Kavu",
    crate::card::CardArt::new("ecd79fbf-626d-4549-917b-435f16b973d9", "Michael Sutfin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 111 — Hull Breach
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HULL_BREACH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6907fa19-29ed-4319-8835-68f424c92831"),
    "Hull Breach",
    crate::card::CardArt::new("6907fa19-29ed-4319-8835-68f424c92831", "Brian Snõddy"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 112 — Keldon Twilight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_TWILIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e071665e-bb72-42e0-a42d-0d0ff02abd2b"),
    "Keldon Twilight",
    crate::card::CardArt::new("e071665e-bb72-42e0-a42d-0d0ff02abd2b", "Franz Vohwinkel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 113 — Lava Zombie
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_ZOMBIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd87185b-1242-4fb3-abee-44bc267ee5fb"),
    "Lava Zombie",
    crate::card::CardArt::new("fd87185b-1242-4fb3-abee-44bc267ee5fb", "Tom Wänerstrand"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 114 — Malicious Advice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MALICIOUS_ADVICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b1547c2-ae9f-4871-a675-4026bf20e7e1"),
    "Malicious Advice",
    crate::card::CardArt::new("7b1547c2-ae9f-4871-a675-4026bf20e7e1", "Glen Angus"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 115 — Marsh Crocodile
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MARSH_CROCODILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("813279d1-d7bd-4d49-bd9d-fc9a6595dd39"),
    "Marsh Crocodile",
    crate::card::CardArt::new("813279d1-d7bd-4d49-bd9d-fc9a6595dd39", "Kev Walker"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 116 — Meddling Mage
pub(in crate::card::sets) static MEDDLING_MAGE: CardRecord = CardRecord::new_with_legacy_id(
    2050,
    "Meddling Mage",
    CardArt::new(
        "176f84c6-aa5e-449c-bd2b-cc91a898f0c7",
        "Christopher Moeller",
    ),
    CardSet::Planeshift,
    // Both players, which is why the mirror is miserable: the Mage does not
    // care who was going to cast the card it named.
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        abilities::choose_card_name_as_enters(
            "As this creature enters, choose a nonland card name.",
            crate::card::BattlefieldEntryScalarChoiceDef::NONLAND_CARD_NAME,
        ),
        abilities::cannot_cast_spells_with_chosen_name(
            "Spells with the chosen name can't be cast.",
        ),
    ]),
);

// PLS 117 — Natural Emergence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NATURAL_EMERGENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3eb4857-7c66-42e4-913c-97a0306366d5"),
    "Natural Emergence",
    crate::card::CardArt::new("c3eb4857-7c66-42e4-913c-97a0306366d5", "Heather Hudson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 118 — Phyrexian Tyranny
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_TYRANNY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8440ca8-73ca-462b-a735-f6fb3d0de603"),
    "Phyrexian Tyranny",
    crate::card::CardArt::new("e8440ca8-73ca-462b-a735-f6fb3d0de603", "Kev Walker"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 119 — Questing Phelddagrif
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUESTING_PHELDDAGRIF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4953790-5723-4f28-ba2d-8a0b328ee5ab"),
    "Questing Phelddagrif",
    crate::card::CardArt::new("cea4cfef-6736-42a5-9f3e-10de8d0cd8d3", "Matt Cavotta"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 120 — Radiant Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("153077a8-38c0-44aa-9b84-cdd9ade50ad6"),
    "Radiant Kavu",
    crate::card::CardArt::new("153077a8-38c0-44aa-9b84-cdd9ade50ad6", "Ron Spencer"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 121 — Razing Snidd
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAZING_SNIDD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2090b80-2ce2-4c9a-87fe-d221f3c677b4"),
    "Razing Snidd",
    crate::card::CardArt::new("d2090b80-2ce2-4c9a-87fe-d221f3c677b4", "Alan Pollack"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 122 — Rith's Charm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_S_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd30f389-bac8-4b82-a8a7-6948d43a9f60"),
    "Rith's Charm",
    crate::card::CardArt::new("dd30f389-bac8-4b82-a8a7-6948d43a9f60", "David Martin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 123 — Sawtooth Loon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAWTOOTH_LOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31b0a87f-e946-4ef1-b30d-fe32c19a0f52"),
    "Sawtooth Loon",
    crate::card::CardArt::new("31b0a87f-e946-4ef1-b30d-fe32c19a0f52", "Heather Hudson"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 124 — Shivan Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4bc72997-78b0-47aa-a029-bf55f77c3e73"),
    "Shivan Wurm",
    crate::card::CardArt::new("4bc72997-78b0-47aa-a029-bf55f77c3e73", "Scott M. Fischer"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 125 — Silver Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("726aa407-dadd-4575-aee2-b7888e67a722"),
    "Silver Drake",
    crate::card::CardArt::new("ac35ee86-96b2-47aa-a1ba-2988737f11ee", "Alan Pollack"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 126 — Sparkcaster
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPARKCASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("daf442b3-fa39-4f6a-90a0-22dcd9df649c"),
    "Sparkcaster",
    crate::card::CardArt::new("daf442b3-fa39-4f6a-90a0-22dcd9df649c", "Adam Rex"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 127 — Steel Leaf Paladin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STEEL_LEAF_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28e8697f-fdf3-4a1a-a84d-dd29b17336c2"),
    "Steel Leaf Paladin",
    crate::card::CardArt::new("28e8697f-fdf3-4a1a-a84d-dd29b17336c2", "Paolo Parente"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 128 — Terminate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TERMINATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("190ca502-672d-4cc0-b6e0-b9de517058d0"),
    "Terminate",
    crate::card::CardArt::new("190ca502-672d-4cc0-b6e0-b9de517058d0", "DiTerlizzi"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 129 — Treva's Charm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREVA_S_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72acb67d-01cb-4fde-8b0b-199e8d1e396a"),
    "Treva's Charm",
    crate::card::CardArt::new("72acb67d-01cb-4fde-8b0b-199e8d1e396a", "David Martin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 130 — Urza's Guilt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_GUILT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d429233e-1cf9-4f87-b191-894a73e7a876"),
    "Urza's Guilt",
    crate::card::CardArt::new("d429233e-1cf9-4f87-b191-894a73e7a876", "Paolo Parente"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 131 — Draco
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRACO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("212e3edb-62f1-4680-884f-70323547f8ad"),
    "Draco",
    crate::card::CardArt::new("212e3edb-62f1-4680-884f-70323547f8ad", "Sam Wood"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 132 — Mana Cylix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_CYLIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c6f95767-afda-4d74-bbd4-1b702eeae54b"),
    "Mana Cylix",
    crate::card::CardArt::new("c6f95767-afda-4d74-bbd4-1b702eeae54b", "Donato Giancola"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 133 — Skyship Weatherlight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHIP_WEATHERLIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63f5498b-bb12-48ec-811b-b52e45ffddaf"),
    "Skyship Weatherlight",
    crate::card::CardArt::new("63f5498b-bb12-48ec-811b-b52e45ffddaf", "Mark Tedin"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 133★ — Skyship Weatherlight (alternate printing)

// PLS 134 — Star Compass
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STAR_COMPASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1d0beb4-c3dd-4bb1-b49b-a48b2d4ad38d"),
    "Star Compass",
    crate::card::CardArt::new("b1d0beb4-c3dd-4bb1-b49b-a48b2d4ad38d", "Donato Giancola"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 135 — Stratadon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRATADON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("324bc757-9942-4862-b691-5af42e07f682"),
    "Stratadon",
    crate::card::CardArt::new("324bc757-9942-4862-b691-5af42e07f682", "Brian Snõddy"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 136 — Crosis's Catacombs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_S_CATACOMBS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7caad74f-c0d0-4eca-94be-b89a2c9a3980"),
    "Crosis's Catacombs",
    crate::card::CardArt::new(
        "7caad74f-c0d0-4eca-94be-b89a2c9a3980",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 137 — Darigaaz's Caldera
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_S_CALDERA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("752f6f0c-af30-4937-b4a7-48f493e007a0"),
    "Darigaaz's Caldera",
    crate::card::CardArt::new("752f6f0c-af30-4937-b4a7-48f493e007a0", "Franz Vohwinkel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 138 — Dromar's Cavern
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_S_CAVERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85f10cee-6a63-438e-a9df-6b902dd025b8"),
    "Dromar's Cavern",
    crate::card::CardArt::new("85f10cee-6a63-438e-a9df-6b902dd025b8", "Franz Vohwinkel"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 139 — Forsaken City
/// A card from your own hand, whichever you can spare. The exile is the
/// upkeep cost of a land that would otherwise stay tapped forever.
static A_CARD_IN_YOUR_HAND: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Any,
    &[ZoneKind::Hand],
    PlayerSetDef::Related(PlayerRelation::You),
);

static CITY_EXILE_AND_UNTAP: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Query(A_CARD_IN_YOUR_HAND),
    exclude: None,
    minimum: 1,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &EffectDef::Sequence(&[
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
            zone: ZoneKind::Exile,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
        EffectDef::Untap {
            object: EffectRecipientDef::Source,
        },
    ]),
});

pub(in crate::card::sets) static FORSAKEN_CITY: CardRecord = CardRecord::new_with_legacy_id(
    2059,
    "Forsaken City",
    CardArt::new("676703fe-0e1a-4b40-9a2b-8b2e2c6b4a05", "Dana Knutson"),
    CardSet::Planeshift,
    // Perfect mana for a deck with cards to spare, and a dead land for one
    // without: the Stasis deck is holding a hand it is not casting anyway.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::static_ability(
            "This land doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may exile a card from your hand. If you do, untap this land.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &CITY_EXILE_AND_UNTAP,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// PLS 140 — Meteor Crater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_CRATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("043a2299-1cfc-4732-a10a-58c773b9992c"),
    "Meteor Crater",
    crate::card::CardArt::new("043a2299-1cfc-4732-a10a-58c773b9992c", "John Avon"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 141 — Rith's Grove
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_S_GROVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("740fa25d-9c1f-44eb-9eb4-0dd514cb315a"),
    "Rith's Grove",
    crate::card::CardArt::new("740fa25d-9c1f-44eb-9eb4-0dd514cb315a", "Scott Bailey"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 142 — Terminal Moraine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TERMINAL_MORAINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("353a8ea8-3f1f-4f77-95bc-b09b96996285"),
    "Terminal Moraine",
    crate::card::CardArt::new("353a8ea8-3f1f-4f77-95bc-b09b96996285", "Scott Bailey"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 143 — Treva's Ruins
static TREVA_COLORS: [ManaColor; 3] = [ManaColor::Green, ManaColor::White, ManaColor::Blue];

/// The Lair itself is excluded by its own subtype, so a second one cannot pay
/// for the first.
static NON_LAIR_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Lair")),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

pub(in crate::card::sets) static TREVAS_RUINS: CardRecord = CardRecord::new_with_legacy_id(
    2060,
    "Treva's Ruins",
    CardArt::new("8bae2458-7cfa-4e0e-9d55-2b2ef8d1c6a1", "Jerry Tiritilli"),
    CardSet::Planeshift,
    // Three colours for the price of a land drop you already made: the Lair
    // costs tempo rather than cards.
    CardRules::new_land(&["Lair"]).with_abilities(&[
        abilities::enters_trigger("When this land enters, sacrifice it unless you return a non-Lair land you control to its owner's hand.", EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    cost: EffectPaymentCostDef::ReturnPermanentMatching(NON_LAIR_LAND_YOU_CONTROL),
                },
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            ))),
        AbilityDef::activated_mana(
            "{T}: Add {G}, {W}, or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&TREVA_COLORS)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AURA_BLAST,
    &AURORA_GRIFFIN,
    &DISCIPLE_OF_KANGEE,
    &DOMINARIA_S_JUDGMENT,
    &GUARD_DOGS,
    &HEROIC_DEFIANCE,
    &HOBBLE,
    &HONORABLE_SCOUT,
    &LASHKNIFE_BARRIER,
    &MARCH_OF_SOULS,
    &ORIM_S_CHANT,
    &PLANESWALKER_S_MIRTH,
    &POLLEN_REMEDY,
    &SAMITE_ELDER,
    &SAMITE_PILGRIM,
    &SUNSCAPE_BATTLEMAGE,
    &SUNSCAPE_FAMILIAR,
    &SURPRISE_DEPLOYMENT,
    &VOICE_OF_ALL,
    &ALLIED_STRATEGIES,
    &ARCTIC_MERFOLK,
    &CONFOUND,
    &DRALNU_S_PET,
    &ERTAI_S_TRICKERY,
    &ESCAPE_ROUTES,
    &GAINSAY,
    &HUNTING_DRAKE,
    &PLANAR_OVERLAY,
    &PLANESWALKER_S_MISCHIEF,
    &RUSHING_RIVER,
    &SEA_SNIDD,
    &SHIFTING_SKY,
    &SISAY_S_INGENUITY,
    &SLEEPING_POTION,
    &STORMSCAPE_BATTLEMAGE,
    &STORMSCAPE_FAMILIAR,
    &SUNKEN_HOPE,
    &WATERSPOUT_ELEMENTAL,
    &BOG_DOWN,
    &DARK_SUSPICIONS,
    &DEATH_BOMB,
    &DIABOLIC_INTENT,
    &EXOTIC_DISEASE,
    &LORD_OF_THE_UNDEAD,
    &MAGGOT_CARRIER,
    &MORGUE_TOAD,
    &NIGHTSCAPE_BATTLEMAGE,
    &NIGHTSCAPE_FAMILIAR,
    &NOXIOUS_VAPORS,
    &PHYREXIAN_BLOODSTOCK,
    &PHYREXIAN_SCUTA,
    &PLANESWALKER_S_SCORN,
    &SHRIEK_OF_DREAD,
    &SINISTER_STRENGTH,
    &SLAY,
    &VOLCANO_IMP,
    &WARPED_DEVOTION,
    &CALDERA_KAVU,
    &DEADAPULT,
    &FLAMETONGUE_KAVU,
    &GOBLIN_GAME,
    &IMPLODE,
    &INSOLENCE,
    &KAVU_RECLUSE,
    &KELDON_MANTLE,
    &MAGMA_BURST,
    &MIRE_KAVU,
    &MOGG_JAILER,
    &MOGG_SENTRY,
    &PLANESWALKER_S_FURY,
    &SINGE,
    &SLINGSHOT_GOBLIN,
    &STRAFE,
    &TAHNGARTH_TALRUUM_HERO,
    &THUNDERSCAPE_BATTLEMAGE,
    &THUNDERSCAPE_FAMILIAR,
    &ALPHA_KAVU,
    &AMPHIBIOUS_KAVU,
    &FALLING_TIMBER,
    &GAEA_S_HERALD,
    &GAEA_S_MIGHT,
    &MAGNIGOTH_TREEFOLK,
    &MIRRORWOOD_TREEFOLK,
    &MULTANI_S_HARMONY,
    &NEMATA_GROVE_GUARDIAN,
    &PLANESWALKER_S_FAVOR,
    &PRIMAL_GROWTH,
    &PYGMY_KAVU,
    &QUIRION_DRYAD,
    &QUIRION_EXPLORER,
    &ROOT_GREEVIL,
    &SKYSHROUD_BLESSING,
    &STONE_KAVU,
    &THORNSCAPE_BATTLEMAGE,
    &THORNSCAPE_FAMILIAR,
    &ANCIENT_SPIDER,
    &CAVERN_HARPY,
    &CLOUD_COVER,
    &CROSIS_S_CHARM,
    &DARIGAAZ_S_CHARM,
    &DARING_LEAP,
    &DESTRUCTIVE_FLOW,
    &DOOMSDAY_SPECTER,
    &DRALNU_S_CRUSADE,
    &DROMAR_S_CHARM,
    &ELADAMRI_S_CALL,
    &ERTAI_THE_CORRUPTED,
    &FLEETFOOT_PANTHER,
    &GERRARD_S_COMMAND,
    &HORNED_KAVU,
    &HULL_BREACH,
    &KELDON_TWILIGHT,
    &LAVA_ZOMBIE,
    &MALICIOUS_ADVICE,
    &MARSH_CROCODILE,
    &MEDDLING_MAGE,
    &NATURAL_EMERGENCE,
    &PHYREXIAN_TYRANNY,
    &QUESTING_PHELDDAGRIF,
    &RADIANT_KAVU,
    &RAZING_SNIDD,
    &RITH_S_CHARM,
    &SAWTOOTH_LOON,
    &SHIVAN_WURM,
    &SILVER_DRAKE,
    &SPARKCASTER,
    &STEEL_LEAF_PALADIN,
    &TERMINATE,
    &TREVA_S_CHARM,
    &URZA_S_GUILT,
    &DRACO,
    &MANA_CYLIX,
    &SKYSHIP_WEATHERLIGHT,
    &STAR_COMPASS,
    &STRATADON,
    &CROSIS_S_CATACOMBS,
    &DARIGAAZ_S_CALDERA,
    &DROMAR_S_CAVERN,
    &FORSAKEN_CITY,
    &METEOR_CRATER,
    &RITH_S_GROVE,
    &TERMINAL_MORAINE,
    &TREVAS_RUINS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&TAHNGARTH_TALRUUM_HERO, 1), // PLS 74
    PrintingRecord::alternate(&ERTAI_THE_CORRUPTED, 1),    // PLS 107★
    PrintingRecord::alternate(&SKYSHIP_WEATHERLIGHT, 1),   // PLS 133★
];
