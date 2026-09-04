//! Planeshift cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, CounterKind, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ParentBinding;
use crate::mana_cost;

// PLS 1 — Aura Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_BLAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Aura Blast",
    "090f5ad6-e10e-49b3-8643-51a4e792517c",
    "Ron Walotsky",
    crate::card::CardRules::unsupported(),
);

// PLS 2 — Aurora Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURORA_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Aurora Griffin",
    "bfd6c695-1944-4bb0-a701-0daf47cdbcb4",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// PLS 3 — Disciple of Kangee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCIPLE_OF_KANGEE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Disciple of Kangee",
    "e268fe16-070b-4b78-9793-59755edb2fd5",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// PLS 4 — Dominaria's Judgment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOMINARIA_S_JUDGMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Dominaria's Judgment",
    "9703d090-b415-48e2-8158-dd8fc57ecc50",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// PLS 5 — Guard Dogs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUARD_DOGS: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Guard Dogs",
    "ba32eee7-10ba-4f0b-8a87-c3ecfa22ae41",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// PLS 6 — Heroic Defiance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEROIC_DEFIANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Heroic Defiance",
    "0dc1aa36-5d3b-4d25-9d54-937cdabf72a4",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// PLS 7 — Hobble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOBBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Hobble",
    "54c76a22-f9e3-408b-a5bd-403add57e31a",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// PLS 8 — Honorable Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONORABLE_SCOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Honorable Scout",
    "bd311758-0352-4b7d-a24f-7f3f2b5d7b0f",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// PLS 9 — Lashknife Barrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LASHKNIFE_BARRIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Lashknife Barrier",
    "2485c10d-de02-4be9-8119-afb2296e3317",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PLS 10 — March of Souls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARCH_OF_SOULS: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "March of Souls",
    "f07dd0f1-b80b-4af0-ae76-907ec55ec7d5",
    "Marc Fishman",
    crate::card::CardRules::unsupported(),
);

// PLS 11 — Orim's Chant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_CHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Orim's Chant",
    "055afa78-b969-498f-a3ad-c792426e5ee6",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PLS 12 — Planeswalker's Mirth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_MIRTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Planeswalker's Mirth",
    "0205d094-c846-4aa0-ade8-2a52c57b11da",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// PLS 13 — Pollen Remedy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLLEN_REMEDY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Pollen Remedy",
    "9797c813-0cda-44ad-ae41-330e9bde9cb9",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// PLS 14 — Samite Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Samite Elder",
    "b3c5dccc-2a48-4dcc-a796-fa6fdc11a14e",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// PLS 15 — Samite Pilgrim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_PILGRIM: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Samite Pilgrim",
    "c12529e4-f4b1-45be-8252-28783badbec5",
    "D. J. Cleland-Hura",
    crate::card::CardRules::unsupported(),
);

// PLS 16 — Sunscape Battlemage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sunscape Battlemage",
    "a85e590f-0a4a-4ad0-b8ef-d3a18edadc05",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// PLS 17 — Sunscape Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sunscape Familiar",
    "9621f341-bf85-4b77-bf19-2fb013b4c955",
    "Brian Despain",
    crate::card::CardRules::unsupported(),
);

// PLS 18 — Surprise Deployment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURPRISE_DEPLOYMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Surprise Deployment",
    "9a26148b-b981-4af5-995b-52b1426737e3",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// PLS 19 — Voice of All
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOICE_OF_ALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Voice of All",
    "75f37536-db3d-4726-9e45-b9108247d0e6",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// PLS 20 — Allied Strategies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALLIED_STRATEGIES: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Allied Strategies",
    "51d4f211-10e8-486d-b982-287ab0c060c9",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PLS 21 — Arctic Merfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCTIC_MERFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Arctic Merfolk",
    "86369fe5-d86d-4f4c-8f3d-dedc174f2032",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// PLS 22 — Confound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONFOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Confound",
    "4f3b7d39-ce98-48e2-b2bf-0d55b4d3102b",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// PLS 23 — Dralnu's Pet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRALNU_S_PET: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Dralnu's Pet",
    "cd5f4daf-7b54-4425-a93a-19532dfb83ca",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// PLS 24 — Ertai's Trickery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_S_TRICKERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Ertai's Trickery",
    "544e3575-9fb6-41f7-a4e6-f8460dfae344",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PLS 25 — Escape Routes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESCAPE_ROUTES: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Escape Routes",
    "dbc9062e-ddd9-41ac-a88a-33f5a7b22103",
    "Marc Fishman",
    crate::card::CardRules::unsupported(),
);

// PLS 26 — Gainsay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAINSAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Gainsay",
    "a70a2092-5048-49c0-9351-a3f882c2f56e",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// PLS 27 — Hunting Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Hunting Drake",
    "5b0293a9-48fe-4018-bd25-3e02c227a3dd",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// PLS 28 — Planar Overlay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_OVERLAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Planar Overlay",
    "1315fef0-234e-44f5-a7a3-bf3db78943c3",
    "Ron Walotsky",
    crate::card::CardRules::unsupported(),
);

// PLS 29 — Planeswalker's Mischief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_MISCHIEF: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Planeswalker's Mischief",
    "79aa232c-3f16-4c68-99dc-09a7aeef477b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// PLS 30 — Rushing River
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHING_RIVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Rushing River",
    "52ddf7bf-de9c-4657-8d5b-79869d36fa63",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// PLS 31 — Sea Snidd
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_SNIDD: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sea Snidd",
    "ca11015e-200b-488c-8bf5-662dcc03cd2d",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// PLS 32 — Shifting Sky
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIFTING_SKY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Shifting Sky",
    "1071726d-48f0-46d6-802b-dd9589489580",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// PLS 33 — Sisay's Ingenuity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SISAY_S_INGENUITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sisay's Ingenuity",
    "bbe20cc1-621a-4813-9bbb-ace006e173ff",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PLS 34 — Sleeping Potion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLEEPING_POTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sleeping Potion",
    "6f79f4b2-71cd-4f78-a161-d75b162c745e",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// PLS 35 — Stormscape Battlemage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Stormscape Battlemage",
    "7d46a39d-c6f4-4281-b31f-f0a0c9fba887",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// PLS 36 — Stormscape Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Stormscape Familiar",
    "4c831c42-77a0-4f4f-9628-ad630541cf66",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// PLS 37 — Sunken Hope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNKEN_HOPE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sunken Hope",
    "5f12ac0c-cfe6-4f08-b6df-20be4ce83e8c",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// PLS 38 — Waterspout Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATERSPOUT_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Waterspout Elemental",
    "425156e6-8eee-4bff-8f2f-86edd9a4f73b",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// PLS 39 — Bog Down
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_DOWN: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Bog Down",
    "8752a605-38f8-4d75-b122-063a788dff6e",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// PLS 40 — Dark Suspicions
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_SUSPICIONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Dark Suspicions",
    "d518e2fd-7767-43d7-92e3-62a4a465154c",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// PLS 41 — Death Bomb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_BOMB: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Death Bomb",
    "f8a84715-c5dc-4a19-af6a-796c6ee912c2",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// PLS 42 — Diabolic Intent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIABOLIC_INTENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Diabolic Intent",
    "76d1b5c5-cc47-465f-8549-4fd1ca4280df",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// PLS 43 — Exotic Disease
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXOTIC_DISEASE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Exotic Disease",
    "4e9624e5-79a2-41de-997b-12d871d4be66",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PLS 44 — Lord of the Undead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_OF_THE_UNDEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Lord of the Undead",
    "0a7f50f4-37a0-476e-8655-edba228aafd6",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// PLS 45 — Maggot Carrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGGOT_CARRIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Maggot Carrier",
    "ab2c3dc4-bb49-4ec3-a6c8-4256d1939326",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// PLS 46 — Morgue Toad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORGUE_TOAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Morgue Toad",
    "77d8ae73-70d1-4082-8581-5f74c1aaa63b",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// PLS 47 — Nightscape Battlemage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Nightscape Battlemage",
    "d5389643-4cc0-4a17-bc2d-7f9b76d30f9f",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// PLS 48 — Nightscape Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Nightscape Familiar",
    "24fa6853-09b0-4c9f-a138-9dd005780255",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// PLS 49 — Noxious Vapors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOXIOUS_VAPORS: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Noxious Vapors",
    "e3cf9326-6e1c-4a05-abea-16d6b6cb2a6d",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// PLS 50 — Phyrexian Bloodstock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_BLOODSTOCK: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Phyrexian Bloodstock",
    "785e1a67-af94-48e8-bb37-4999d1fb4c66",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// PLS 51 — Phyrexian Scuta
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_SCUTA: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Phyrexian Scuta",
    "eb57e656-c94e-4cc2-ae8d-9300f51f941f",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// PLS 52 — Planeswalker's Scorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_SCORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Planeswalker's Scorn",
    "8ed08376-836f-4313-83d0-481895ead9da",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// PLS 53 — Shriek of Dread
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEK_OF_DREAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Shriek of Dread",
    "54a7fb3b-8e81-4763-b2a1-7c2108a00afe",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// PLS 54 — Sinister Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINISTER_STRENGTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sinister Strength",
    "afe487b8-c1ae-483d-bcd5-62c62b66a22e",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// PLS 55 — Slay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Slay",
    "eccda747-2680-4793-8a13-35e49b4de12f",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// PLS 56 — Volcano Imp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLCANO_IMP: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Volcano Imp",
    "a8281cc6-2132-4f76-841e-d1ade9cafb84",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// PLS 57 — Warped Devotion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARPED_DEVOTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Warped Devotion",
    "3bce620f-799a-4ad8-9edb-6fb3d9ea1cc6",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// PLS 58 — Caldera Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALDERA_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Caldera Kavu",
    "fcad32aa-2ce1-402d-a9d8-ad5c81fe4c5b",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// PLS 59 — Deadapult
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADAPULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Deadapult",
    "bdc93b3d-bde4-422f-9edc-e337719be7b4",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// PLS 60 — Flametongue Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMETONGUE_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Flametongue Kavu",
    "e5056bca-bd90-4b50-8630-105558f8ef92",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// PLS 61 — Goblin Game
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GAME: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Goblin Game",
    "cbe6e7e5-ffea-4c6c-8a42-28e695029f24",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// PLS 62 — Implode
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPLODE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Implode",
    "a76ee318-8126-4ebf-884d-8369ae8726ac",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// PLS 63 — Insolence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSOLENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Insolence",
    "d8009a37-f966-4a71-9a2a-469127758dc6",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// PLS 64 — Kavu Recluse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_RECLUSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Kavu Recluse",
    "6f04ac02-3eff-4a66-8320-ee7b4357522f",
    "Aaron Boyd",
    crate::card::CardRules::unsupported(),
);

// PLS 65 — Keldon Mantle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_MANTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Keldon Mantle",
    "35bb73df-f488-468c-a9ad-72f52c8da3dc",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// PLS 66 — Magma Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMA_BURST: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Magma Burst",
    "d9752bc3-0bdf-4657-8750-73c8cbc8e83f",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// PLS 67 — Mire Kavu
pub(in crate::card::sets) static MIRE_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Mire Kavu",
    "ccdd0086-eb27-48b3-91cb-a113aa1de102",
    "Wayne England",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_JAILER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Mogg Jailer",
    "52513235-0e6c-40ea-8ead-a050e6da676e",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// PLS 69 — Mogg Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_SENTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Mogg Sentry",
    "8536ec54-cebd-4d44-8e52-42344a3e6daa",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// PLS 70 — Planeswalker's Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_FURY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Planeswalker's Fury",
    "6fa09e3a-bc7e-4292-aa5d-ce97c1b1f79f",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// PLS 71 — Singe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Singe",
    "32323277-db9a-48a7-b9a4-8e6914386e26",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// PLS 72 — Slingshot Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLINGSHOT_GOBLIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Slingshot Goblin",
    "81825aef-bef7-46b7-bf52-29e32c1836b0",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// PLS 73 — Strafe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRAFE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Strafe",
    "ec8b77cf-9c1e-4c8f-b452-295cc1570d0e",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// PLS 74 — Tahngarth, Talruum Hero (alternate printing)
const TAHNGARTH_TALRUUM_HERO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TAHNGARTH_TALRUUM_HERO,
    1,
    "c1778f37-af01-4f8c-ab9d-a4c60abf7e78",
    "Dave Dorman",
);

// PLS 74★ — Tahngarth, Talruum Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAHNGARTH_TALRUUM_HERO: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Tahngarth, Talruum Hero",
    "6cdab0f9-7208-4555-b509-e61773ebc1f9",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PLS 75 — Thunderscape Battlemage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Thunderscape Battlemage",
    "d707243e-7f11-44bc-b8b8-af635ab1dc87",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// PLS 76 — Thunderscape Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Thunderscape Familiar",
    "26c9c0aa-9412-4320-aaee-e05369b8bc7b",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// PLS 77 — Alpha Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALPHA_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Alpha Kavu",
    "545ed916-59fc-4c60-9260-8c2dc88e67a1",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// PLS 78 — Amphibious Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMPHIBIOUS_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Amphibious Kavu",
    "37d94fb2-958c-487e-9f64-52d2771c6ea4",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// PLS 79 — Falling Timber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALLING_TIMBER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Falling Timber",
    "6e54c84d-ccc9-4c52-b02c-e0392e8fe447",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// PLS 80 — Gaea's Herald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_HERALD: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Gaea's Herald",
    "aa52bc97-109a-4de5-b287-bce21dad6a9c",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// PLS 81 — Gaea's Might
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_MIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Gaea's Might",
    "67e5adce-7735-4fa5-aa14-8dce012e9fcc",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// PLS 82 — Magnigoth Treefolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNIGOTH_TREEFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Magnigoth Treefolk",
    "90c2869b-43cf-4d5e-8a54-9ae200f5bff9",
    "Peter Bollinger",
    crate::card::CardRules::unsupported(),
);

// PLS 83 — Mirrorwood Treefolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRORWOOD_TREEFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Mirrorwood Treefolk",
    "ba9a1c94-2b7f-4df7-8517-a122616d9ae4",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// PLS 84 — Multani's Harmony
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_S_HARMONY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Multani's Harmony",
    "c76352ea-e3d2-4221-8ebe-e953301c35ab",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// PLS 85 — Nemata, Grove Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEMATA_GROVE_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Nemata, Grove Guardian",
    "8c6a0ca4-5006-4c9b-91cd-e01d77e4fdc2",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// PLS 86 — Planeswalker's Favor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANESWALKER_S_FAVOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Planeswalker's Favor",
    "b3387540-93bf-451e-8e7a-fc78caab42b0",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// PLS 87 — Primal Growth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMAL_GROWTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Primal Growth",
    "1d4a3c83-faaa-4dd9-9349-abcaf09cc7a8",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// PLS 88 — Pygmy Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Pygmy Kavu",
    "b31c69ec-feb5-430a-a3e9-3a6f3fb8ee1c",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// PLS 89 — Quirion Dryad
pub(in crate::card::sets) static QUIRION_DRYAD: CardRecord = CardRecord::new(
    CardSet::Planeshift,
    "Quirion Dryad",
    "f6841ae6-b15f-488e-9cae-2cc5ec668278",
    "Don Hazeltine",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a spell that's white, blue, black, or red, put a +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_EXPLORER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Quirion Explorer",
    "141a031d-f899-497b-adf7-4af142078085",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// PLS 91 — Root Greevil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOT_GREEVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Root Greevil",
    "306e3429-b3b4-4186-935b-18cfc308d22c",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// PLS 92 — Skyshroud Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Skyshroud Blessing",
    "c0c10b16-97b1-4a36-b2b4-f0c28ead3eb4",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// PLS 93 — Stone Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Stone Kavu",
    "36a1cdca-d48c-4936-ad6a-4610aeb991ce",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// PLS 94 — Thornscape Battlemage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_BATTLEMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Thornscape Battlemage",
    "13f24f89-3996-4740-a6c9-d26b8869554b",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// PLS 95 — Thornscape Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Thornscape Familiar",
    "76c6e426-6165-4f8e-8766-de768ae13452",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// PLS 96 — Ancient Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Ancient Spider",
    "75ca99de-57e7-47c4-b40a-6e41e3b18069",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// PLS 97 — Cavern Harpy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVERN_HARPY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Cavern Harpy",
    "adfb0804-50d6-4bca-8733-72e01030a543",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// PLS 98 — Cloud Cover
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_COVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Cloud Cover",
    "943b3886-5556-474f-8dc1-18219e25abc3",
    "Marc Fishman",
    crate::card::CardRules::unsupported(),
);

// PLS 99 — Crosis's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_S_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Crosis's Charm",
    "b59a9e75-9988-4040-a718-b1655fc20d11",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// PLS 100 — Darigaaz's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_S_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Darigaaz's Charm",
    "cf4c9d6a-86eb-45be-9405-473eb263b94c",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// PLS 101 — Daring Leap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARING_LEAP: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Daring Leap",
    "37ec6c4b-2de0-4759-a25d-007706cb18cc",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PLS 102 — Destructive Flow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESTRUCTIVE_FLOW: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Destructive Flow",
    "7db86e34-c3ec-4a29-8779-81350a985644",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// PLS 103 — Doomsday Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOOMSDAY_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Doomsday Specter",
    "85206cc1-5484-40c6-b11d-b8d6fad4fc5c",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// PLS 104 — Dralnu's Crusade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRALNU_S_CRUSADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Dralnu's Crusade",
    "6a35d227-4489-4a0b-8f81-eb8e5949e1fc",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// PLS 105 — Dromar's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_S_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Dromar's Charm",
    "c7a1894c-af4e-4530-960f-2225916be8cb",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// PLS 106 — Eladamri's Call
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELADAMRI_S_CALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Eladamri's Call",
    "dcb79f39-5ef3-4ad6-9a43-04beb27d8480",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PLS 107 — Ertai, the Corrupted
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_THE_CORRUPTED: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Ertai, the Corrupted",
    "66b950d9-8fef-4deb-b51b-26edb90abc56",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// PLS 107★ — Ertai, the Corrupted (alternate printing)
const ERTAI_THE_CORRUPTED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ERTAI_THE_CORRUPTED,
    1,
    "fbbfeb32-1654-4bf6-9a38-891f1a03e02b",
    "Kev Walker",
);

// PLS 108 — Fleetfoot Panther
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEETFOOT_PANTHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Fleetfoot Panther",
    "b70220d8-f81b-44a4-b92e-d66de8c1b4ce",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// PLS 109 — Gerrard's Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_S_COMMAND: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Gerrard's Command",
    "d0fda263-b6a7-43e3-998a-72a9d84c4572",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// PLS 110 — Horned Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORNED_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Horned Kavu",
    "ecd79fbf-626d-4549-917b-435f16b973d9",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// PLS 111 — Hull Breach
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULL_BREACH: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Hull Breach",
    "6907fa19-29ed-4319-8835-68f424c92831",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// PLS 112 — Keldon Twilight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_TWILIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Keldon Twilight",
    "e071665e-bb72-42e0-a42d-0d0ff02abd2b",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// PLS 113 — Lava Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Lava Zombie",
    "fd87185b-1242-4fb3-abee-44bc267ee5fb",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// PLS 114 — Malicious Advice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALICIOUS_ADVICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Malicious Advice",
    "7b1547c2-ae9f-4871-a675-4026bf20e7e1",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// PLS 115 — Marsh Crocodile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARSH_CROCODILE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Marsh Crocodile",
    "813279d1-d7bd-4d49-bd9d-fc9a6595dd39",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PLS 116 — Meddling Mage
pub(in crate::card::sets) static MEDDLING_MAGE: CardRecord = CardRecord::new(
    CardSet::Planeshift,
    "Meddling Mage",
    "176f84c6-aa5e-449c-bd2b-cc91a898f0c7",
    "Christopher Moeller",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURAL_EMERGENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Natural Emergence",
    "c3eb4857-7c66-42e4-913c-97a0306366d5",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// PLS 118 — Phyrexian Tyranny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_TYRANNY: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Phyrexian Tyranny",
    "e8440ca8-73ca-462b-a735-f6fb3d0de603",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PLS 119 — Questing Phelddagrif
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUESTING_PHELDDAGRIF: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Questing Phelddagrif",
    "cea4cfef-6736-42a5-9f3e-10de8d0cd8d3",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// PLS 120 — Radiant Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Radiant Kavu",
    "153077a8-38c0-44aa-9b84-cdd9ade50ad6",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// PLS 121 — Razing Snidd
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAZING_SNIDD: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Razing Snidd",
    "d2090b80-2ce2-4c9a-87fe-d221f3c677b4",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// PLS 122 — Rith's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_S_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Rith's Charm",
    "dd30f389-bac8-4b82-a8a7-6948d43a9f60",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// PLS 123 — Sawtooth Loon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAWTOOTH_LOON: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sawtooth Loon",
    "31b0a87f-e946-4ef1-b30d-fe32c19a0f52",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// PLS 124 — Shivan Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Shivan Wurm",
    "4bc72997-78b0-47aa-a029-bf55f77c3e73",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// PLS 125 — Silver Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Silver Drake",
    "ac35ee86-96b2-47aa-a1ba-2988737f11ee",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// PLS 126 — Sparkcaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPARKCASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Sparkcaster",
    "daf442b3-fa39-4f6a-90a0-22dcd9df649c",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// PLS 127 — Steel Leaf Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEEL_LEAF_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Steel Leaf Paladin",
    "28e8697f-fdf3-4a1a-a84d-dd29b17336c2",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PLS 128 — Terminate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERMINATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Terminate",
    "190ca502-672d-4cc0-b6e0-b9de517058d0",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// PLS 129 — Treva's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREVA_S_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Treva's Charm",
    "72acb67d-01cb-4fde-8b0b-199e8d1e396a",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// PLS 130 — Urza's Guilt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_GUILT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Urza's Guilt",
    "d429233e-1cf9-4f87-b191-894a73e7a876",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PLS 131 — Draco
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRACO: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Draco",
    "212e3edb-62f1-4680-884f-70323547f8ad",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// PLS 132 — Mana Cylix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_CYLIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Mana Cylix",
    "c6f95767-afda-4d74-bbd4-1b702eeae54b",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// PLS 133 — Skyship Weatherlight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHIP_WEATHERLIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Skyship Weatherlight",
    "63f5498b-bb12-48ec-811b-b52e45ffddaf",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// PLS 133★ — Skyship Weatherlight (alternate printing)
const SKYSHIP_WEATHERLIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYSHIP_WEATHERLIGHT,
    1,
    "99791ef7-ff51-4982-b0ef-55560f9577ff",
    "Kev Walker",
);

// PLS 134 — Star Compass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAR_COMPASS: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Star Compass",
    "b1d0beb4-c3dd-4bb1-b49b-a48b2d4ad38d",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// PLS 135 — Stratadon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRATADON: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Stratadon",
    "324bc757-9942-4862-b691-5af42e07f682",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// PLS 136 — Crosis's Catacombs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_S_CATACOMBS: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Crosis's Catacombs",
    "7caad74f-c0d0-4eca-94be-b89a2c9a3980",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// PLS 137 — Darigaaz's Caldera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_S_CALDERA: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Darigaaz's Caldera",
    "752f6f0c-af30-4937-b4a7-48f493e007a0",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// PLS 138 — Dromar's Cavern
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_S_CAVERN: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Dromar's Cavern",
    "85f10cee-6a63-438e-a9df-6b902dd025b8",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// PLS 139 — Forsaken City
pub(in crate::card::sets) static FORSAKEN_CITY: CardRecord = CardRecord::new(
    CardSet::Planeshift,
    "Forsaken City",
    "676703fe-bd80-413c-8704-1da5d3248b7e",
    "Dana Knutson",
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
                effect: &const {
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Object(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        // A card from your own hand, whichever you can spare. The exile is the
                        // upkeep cost of a land that would otherwise stay tapped forever.
                        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &const { [ZoneKind::Hand] },
                            PlayerSetDef::Related(PlayerRelation::You),
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &const {
                            EffectDef::Sequence(&const {
                                [
                                    EffectDef::MoveToZone {
                                        object: EffectRecipientDef::object(ObjectRefDef::Binding(
                                            ParentBinding,
                                        )),
                                        zone: ZoneKind::Exile,
                                        placement: ZonePlacement::Top,
                                    },
                                    EffectDef::Untap {
                                        object: EffectRecipientDef::Source,
                                    },
                                ]
                            })
                        },
                    })
                },
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_CRATER: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Meteor Crater",
    "043a2299-1cfc-4732-a10a-58c773b9992c",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// PLS 141 — Rith's Grove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_S_GROVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Rith's Grove",
    "740fa25d-9c1f-44eb-9eb4-0dd514cb315a",
    "Scott Bailey",
    crate::card::CardRules::unsupported(),
);

// PLS 142 — Terminal Moraine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERMINAL_MORAINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Planeshift,
    "Terminal Moraine",
    "353a8ea8-3f1f-4f77-95bc-b09b96996285",
    "Scott Bailey",
    crate::card::CardRules::unsupported(),
);

// PLS 143 — Treva's Ruins
pub(in crate::card::sets) static TREVAS_RUINS: CardRecord = CardRecord::new(
    CardSet::Planeshift,
    "Treva's Ruins",
    "8bae2458-b54f-426a-ad40-13529a73c423",
    "Jerry Tiritilli",
    // Three colours for the price of a land drop you already made: the Lair
    // costs tempo rather than cards.
    CardRules::new_land(&["Lair"]).with_abilities(&[
        abilities::enters_trigger("When this land enters, sacrifice it unless you return a non-Lair land you control to its owner's hand.", EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    cost: EffectPaymentCostDef::MovePermanentMatching {
                        // The Lair itself is excluded by its own subtype, so a second one cannot pay
                        // for the first.
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Lair")),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        zone: ZoneKind::Hand,
                    },
                },
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            ))),
        AbilityDef::activated_mana(
            "{T}: Add {G}, {W}, or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::White, ManaColor::Blue])),
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
    TAHNGARTH_TALRUUM_HERO_ALTERNATE_1,
    ERTAI_THE_CORRUPTED_ALTERNATE_1,
    SKYSHIP_WEATHERLIGHT_ALTERNATE_1,
];
