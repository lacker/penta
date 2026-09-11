//! Aetherdrift card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ManaTypeDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2009::zendikar as catalog_zen;
use crate::card::sets::y2013::theros as catalog_ths;
use crate::card::sets::y2014::khans_of_tarkir as catalog_ktk;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2020::zendikar_rising as catalog_znr;

pub const EXHAUST: crate::card::MechanicId = crate::card::MechanicId::from_name("mtg:exhaust");

/// Exhaust labels an ordinary ability restricted to one activation per object.
///
/// # Panics
///
/// Panics if the clause is not an activated ability or activated mana ability.
#[must_use]
pub const fn exhaust(ability: AbilityDef) -> AbilityDef {
    ability.once_per_object().labeled(EXHAUST)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DFT",
    slug: "aetherdrift",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DFT 1 — Air Response Unit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AIR_RESPONSE_UNIT: CardRecord = CardRecord::new(
    "Air Response Unit",
    "d77c8e29-de24-4664-baf8-959608dd99ca",
    "Brock Grossman",
    crate::card::CardRules::unsupported(),
);

// DFT 2 — Alacrian Armory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALACRIAN_ARMORY: CardRecord = CardRecord::new(
    "Alacrian Armory",
    "d39f7f98-ad5e-4e5e-9f7b-abe0984ffe17",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// DFT 3 — Basri, Tomorrow's Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BASRI_TOMORROW_S_CHAMPION: CardRecord = CardRecord::new(
    "Basri, Tomorrow's Champion",
    "991270fa-a391-4c2e-bd9a-19151386fb67",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// DFT 4 — Brightfield Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIGHTFIELD_GLIDER: CardRecord = CardRecord::new(
    "Brightfield Glider",
    "7eb819eb-ba5c-4449-87b5-3894380558bc",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// DFT 5 — Brightfield Mustang
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIGHTFIELD_MUSTANG: CardRecord = CardRecord::new(
    "Brightfield Mustang",
    "b2c7cacc-f15e-46c1-9c25-b567bb3e8680",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// DFT 6 — Broadcast Rambler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROADCAST_RAMBLER: CardRecord = CardRecord::new(
    "Broadcast Rambler",
    "89ce2385-e33d-47b3-96c8-5a4672d9df7c",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// DFT 7 — Bulwark Ox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BULWARK_OX: CardRecord = CardRecord::new(
    "Bulwark Ox",
    "106944b2-f3ae-4350-be33-61b9f92fc92f",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// DFT 8 — Canyon Vaulter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANYON_VAULTER: CardRecord = CardRecord::new(
    "Canyon Vaulter",
    "cc0b15da-a45c-42f5-aafc-20ad9e38bf24",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// DFT 9 — Cloudspire Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDSPIRE_CAPTAIN: CardRecord = CardRecord::new(
    "Cloudspire Captain",
    "3380d87a-c460-409c-8d47-9b2fc5ddd2ea",
    "Manny Edeko",
    crate::card::CardRules::unsupported(),
);

// DFT 10 — Collision Course
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLISION_COURSE: CardRecord = CardRecord::new(
    "Collision Course",
    "6b60da34-b622-42de-a249-79545bcbf30d",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// DFT 11 — Daring Mechanic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARING_MECHANIC: CardRecord = CardRecord::new(
    "Daring Mechanic",
    "3382552c-2740-409a-83a1-80b60627beb8",
    "Elizabeth Peiró",
    crate::card::CardRules::unsupported(),
);

// DFT 12 — Detention Chariot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DETENTION_CHARIOT: CardRecord = CardRecord::new(
    "Detention Chariot",
    "75d5e64f-7af2-4cb4-abd1-23992e346bee",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// DFT 13 — Gallant Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALLANT_STRIKE: CardRecord = CardRecord::new(
    "Gallant Strike",
    "9bdb58b7-e1ef-496b-b8dd-d1fabf3d2e7a",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// DFT 14 — Gloryheath Lynx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLORYHEATH_LYNX: CardRecord = CardRecord::new(
    "Gloryheath Lynx",
    "ea3ac678-8b74-4865-a896-c42692c02341",
    "Deruchenko Alexander",
    crate::card::CardRules::unsupported(),
);

// DFT 15 — Guardian Sunmare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUARDIAN_SUNMARE: CardRecord = CardRecord::new(
    "Guardian Sunmare",
    "7c274595-94e2-4587-9cef-b38639d6429a",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// DFT 16 — Guidelight Synergist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDELIGHT_SYNERGIST: CardRecord = CardRecord::new(
    "Guidelight Synergist",
    "fdaeea2c-d8aa-416f-95d6-6af888591fdf",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// DFT 17 — Interface Ace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTERFACE_ACE: CardRecord = CardRecord::new(
    "Interface Ace",
    "fcfd487a-a9e6-44e3-80af-bc384316106f",
    "Wonchun Choi",
    crate::card::CardRules::unsupported(),
);

// DFT 18 — Leonin Surveyor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONIN_SURVEYOR: CardRecord = CardRecord::new(
    "Leonin Surveyor",
    "e08e4107-213f-491b-a032-8e3367009ba8",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// DFT 19 — Lightshield Parry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTSHIELD_PARRY: CardRecord = CardRecord::new(
    "Lightshield Parry",
    "dcf6a0e7-1fd4-425f-b634-c93236daea35",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// DFT 20 — Lightwheel Enhancements
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTWHEEL_ENHANCEMENTS: CardRecord = CardRecord::new(
    "Lightwheel Enhancements",
    "9ab169c1-4e25-4a5d-8961-4f06298c3781",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

// DFT 21 — Lotusguard Disciple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOTUSGUARD_DISCIPLE: CardRecord = CardRecord::new(
    "Lotusguard Disciple",
    "80645651-3804-481f-8f8f-ade762a011e1",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// DFT 22 — Nesting Bot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NESTING_BOT: CardRecord = CardRecord::new(
    "Nesting Bot",
    "7829c0ae-f72f-4195-ad43-775d7218565c",
    "Racrufi",
    crate::card::CardRules::unsupported(),
);

// DFT 23 — Perilous Snare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERILOUS_SNARE: CardRecord = CardRecord::new(
    "Perilous Snare",
    "47f7e468-2196-4960-a612-37ab326e2a17",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// DFT 24 — Pride of the Road
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIDE_OF_THE_ROAD: CardRecord = CardRecord::new(
    "Pride of the Road",
    "4172222f-d871-4354-9a02-7af0001d8956",
    "Alfonso Santano",
    crate::card::CardRules::unsupported(),
);

// DFT 25 — Ride's End
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIDE_S_END: CardRecord = CardRecord::new(
    "Ride's End",
    "2f96b33b-c952-45ac-9626-40169b2bd4ef",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// DFT 26 — Roadside Assistance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROADSIDE_ASSISTANCE: CardRecord = CardRecord::new(
    "Roadside Assistance",
    "8f2a9154-7b43-4b8d-9d81-d11cfda5d597",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// DFT 27 — Salvation Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SALVATION_ENGINE: CardRecord = CardRecord::new(
    "Salvation Engine",
    "34ed1bf2-0f3c-4528-b570-e5bdcd7ffda9",
    "Ben Wootten",
    crate::card::CardRules::unsupported(),
);

// DFT 28 — Skyseer's Chariot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSEER_S_CHARIOT: CardRecord = CardRecord::new(
    "Skyseer's Chariot",
    "96ed5b66-8e74-4a90-ad4e-c39d15993994",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// DFT 29 — Spectacular Pileup
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTACULAR_PILEUP: CardRecord = CardRecord::new(
    "Spectacular Pileup",
    "a24a6309-0e69-45f0-a9ff-44d4997e7e4d",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// DFT 30 — Spotcycle Scouter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPOTCYCLE_SCOUTER: CardRecord = CardRecord::new(
    "Spotcycle Scouter",
    "f0489108-75b7-441c-888d-12987c0c1080",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// DFT 31 — Sundial, Dawn Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNDIAL_DAWN_TYRANT: CardRecord = CardRecord::new(
    "Sundial, Dawn Tyrant",
    "b2e5435c-52f3-42d7-bcee-5aa13afd6626",
    "Bruce Brenneise",
    crate::card::CardRules::unsupported(),
);

// DFT 32 — Swiftwing Assailant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWIFTWING_ASSAILANT: CardRecord = CardRecord::new(
    "Swiftwing Assailant",
    "72db9bb9-d930-40e5-b144-01ebfd377996",
    "Pig Hands",
    crate::card::CardRules::unsupported(),
);

// DFT 33 — Tune Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUNE_UP: CardRecord = CardRecord::new(
    "Tune Up",
    "f8bddc5f-8f25-4313-b5bb-e5eae2923878",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// DFT 34 — Unswerving Sloth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSWERVING_SLOTH: CardRecord = CardRecord::new(
    "Unswerving Sloth",
    "12296a74-5d60-4ee3-aa53-2289f84da776",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// DFT 35 — Valor's Flagship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALOR_S_FLAGSHIP: CardRecord = CardRecord::new(
    "Valor's Flagship",
    "8af1dddf-6c95-448b-acc8-df5a99202e9a",
    "Stephan Martiniere",
    crate::card::CardRules::unsupported(),
);

// DFT 36 — Voyager Glidecar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOYAGER_GLIDECAR: CardRecord = CardRecord::new(
    "Voyager Glidecar",
    "13eb445a-dd41-4760-8299-9ba5d6de6aaf",
    "Eduardo Francisco",
    crate::card::CardRules::unsupported(),
);

// DFT 37 — Voyager Quickwelder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOYAGER_QUICKWELDER: CardRecord = CardRecord::new(
    "Voyager Quickwelder",
    "f6dcdc8c-fba1-4ea1-bf93-65072d10f0da",
    "Kenn Yap",
    crate::card::CardRules::unsupported(),
);

// DFT 38 — Aether Syphon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_SYPHON: CardRecord = CardRecord::new(
    "Aether Syphon",
    "d7033739-4cd8-4727-b9b5-099fb597006b",
    "Martin de Diego Sádaba",
    crate::card::CardRules::unsupported(),
);

// DFT 39 — Bounce Off
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNCE_OFF: CardRecord = CardRecord::new(
    "Bounce Off",
    "7b3c8dda-2405-4879-8dd1-e790a833c42d",
    "Deruchenko Alexander",
    crate::card::CardRules::unsupported(),
);

// DFT 40 — Caelorna, Coral Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAELORNA_CORAL_TYRANT: CardRecord = CardRecord::new(
    "Caelorna, Coral Tyrant",
    "e8654e38-4230-4094-b815-778bfb5d06f2",
    "Deruchenko Alexander",
    crate::card::CardRules::unsupported(),
);

// DFT 41 — Diversion Unit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVERSION_UNIT: CardRecord = CardRecord::new(
    "Diversion Unit",
    "e04d4fa6-1fa3-4bfd-a462-47c23ccf9124",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// DFT 42 — Flood the Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOOD_THE_ENGINE: CardRecord = CardRecord::new(
    "Flood the Engine",
    "57402f7c-5d4c-4f1e-8bce-a2328a297111",
    "Eric Wilkerson",
    crate::card::CardRules::unsupported(),
);

// DFT 43 — Gearseeker Serpent (reprint)
const GEARSEEKER_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::GEARSEEKER_SERPENT,
    "3dca0007-42d3-4ee2-8e88-361d80a7103c",
    "J.P. Targete",
);

// DFT 44 — Glitch Ghost Surveyor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLITCH_GHOST_SURVEYOR: CardRecord = CardRecord::new(
    "Glitch Ghost Surveyor",
    "b9bb89b9-50dd-4b36-aa10-aba585e50246",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// DFT 45 — Guidelight Optimizer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDELIGHT_OPTIMIZER: CardRecord = CardRecord::new(
    "Guidelight Optimizer",
    "e9fc07dd-05b1-49ed-a3ee-46c31b8e0a3d",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// DFT 46 — Howler's Heavy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOWLER_S_HEAVY: CardRecord = CardRecord::new(
    "Howler's Heavy",
    "8bbd7758-59c7-4ae1-9af8-c3580f4aa958",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// DFT 47 — Hulldrifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULLDRIFTER: CardRecord = CardRecord::new(
    "Hulldrifter",
    "402666f8-c9cc-4e8f-abaa-6c38be90cdd2",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// DFT 48 — Keen Buccaneer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEN_BUCCANEER: CardRecord = CardRecord::new(
    "Keen Buccaneer",
    "0ed90a63-5aca-470a-8e1e-518d8aeb6d91",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// DFT 49 — Memory Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEMORY_GUARDIAN: CardRecord = CardRecord::new(
    "Memory Guardian",
    "6b199ce2-0ea0-47e5-a36c-36373be53fec",
    "Hardy Fowler",
    crate::card::CardRules::unsupported(),
);

// DFT 50 — Midnight Mangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIDNIGHT_MANGLER: CardRecord = CardRecord::new(
    "Midnight Mangler",
    "237568e3-7331-4bbb-a091-a766723134fc",
    "Villarrte",
    crate::card::CardRules::unsupported(),
);

// DFT 51 — Mindspring Merfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINDSPRING_MERFOLK: CardRecord = CardRecord::new(
    "Mindspring Merfolk",
    "b6250b8b-1943-445f-ada9-30b41eb6d29b",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// DFT 52 — Mu Yanling, Wind Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MU_YANLING_WIND_RIDER: CardRecord = CardRecord::new(
    "Mu Yanling, Wind Rider",
    "76423446-d62f-4cc5-a23a-3175be88bd73",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// DFT 53 — Nimble Thopterist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIMBLE_THOPTERIST: CardRecord = CardRecord::new(
    "Nimble Thopterist",
    "47717312-f6c6-4e86-ba6a-a30698962430",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// DFT 54 — Possession Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POSSESSION_ENGINE: CardRecord = CardRecord::new(
    "Possession Engine",
    "f206b0a1-50d8-4d53-850d-fb15fd328267",
    "Leroy Steinmann",
    crate::card::CardRules::unsupported(),
);

// DFT 55 — Rangers' Refueler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANGERS_REFUELER: CardRecord = CardRecord::new(
    "Rangers' Refueler",
    "67d2d713-8acb-4e3d-bd1d-0416fe9b9ef6",
    "Samuel Perin",
    crate::card::CardRules::unsupported(),
);

// DFT 56 — Repurposing Bay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPURPOSING_BAY: CardRecord = CardRecord::new(
    "Repurposing Bay",
    "0cf1ace1-b7f5-4bd9-a494-ee7cb6c1f854",
    "William Tempest",
    crate::card::CardRules::unsupported(),
);

// DFT 57 — Riverchurn Monument
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVERCHURN_MONUMENT: CardRecord = CardRecord::new(
    "Riverchurn Monument",
    "e66ff696-fd39-49ad-9ee5-c0868167df37",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// DFT 58 — Roadside Blowout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROADSIDE_BLOWOUT: CardRecord = CardRecord::new(
    "Roadside Blowout",
    "d6153a76-56f7-46ee-bba5-b62c0143388a",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// DFT 59 — Sabotage Strategist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SABOTAGE_STRATEGIST: CardRecord = CardRecord::new(
    "Sabotage Strategist",
    "c8bb15e2-e1ad-4645-aab0-df4a1a68563d",
    "Darren Tan",
    crate::card::CardRules::unsupported(),
);

// DFT 60 — Scrounging Skyray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCROUNGING_SKYRAY: CardRecord = CardRecord::new(
    "Scrounging Skyray",
    "d60bece3-6f63-4d9e-bca0-cef2d38f1472",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// DFT 61 — Skystreak Engineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSTREAK_ENGINEER: CardRecord = CardRecord::new(
    "Skystreak Engineer",
    "5bc9c501-098d-4560-9826-329b05689e0f",
    "Elizabeth Peiró",
    crate::card::CardRules::unsupported(),
);

// DFT 62 — Slick Imitator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLICK_IMITATOR: CardRecord = CardRecord::new(
    "Slick Imitator",
    "3e86ef50-4939-4e7c-853d-438f0f3e0411",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// DFT 63 — Spectral Interference
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTRAL_INTERFERENCE: CardRecord = CardRecord::new(
    "Spectral Interference",
    "860cb8af-a5f6-47e7-a34b-7b9f11ddc8c6",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// DFT 64 — Spell Pierce (reprint)
const SPELL_PIERCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::SPELL_PIERCE,
    "8dd4374f-0301-4b2e-bc99-2cd19568cb3b",
    "Maxime Minard",
);

// DFT 65 — Spikeshell Harrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKESHELL_HARRIER: CardRecord = CardRecord::new(
    "Spikeshell Harrier",
    "8f1ece22-ca32-45bb-b5f4-480f9b366cb5",
    "Alfonso Santano",
    crate::card::CardRules::unsupported(),
);

// DFT 66 — Stall Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALL_OUT: CardRecord = CardRecord::new(
    "Stall Out",
    "4ea0e0d3-833f-4353-b648-57b0b657cc1c",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// DFT 67 — Stock Up
pub(in crate::card::sets) static STOCK_UP: CardRecord = CardRecord::new(
    "Stock Up",
    "0a786855-6eb4-42c0-a528-4842db46809d",
    "Izzy",
// Two cards for three mana at sorcery speed is unremarkable; seeing five
    // to find them is what puts it in a deck built around one or two cards.
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Look at the top five cards of your library. Put two of them into your hand and the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_choose_to_hand_rest_bottom(
            ValueDef::Constant(5),
            ObjectPredicateDef::Any,
            2,
            2,
        ),
    )),
);

// DFT 68 — Thopter Fabricator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOPTER_FABRICATOR: CardRecord = CardRecord::new(
    "Thopter Fabricator",
    "8924b785-b140-4212-a1eb-a10340e09fea",
    "Racrufi",
    crate::card::CardRules::unsupported(),
);

// DFT 69 — Trade the Helm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADE_THE_HELM: CardRecord = CardRecord::new(
    "Trade the Helm",
    "eb0b5c09-6c21-4080-81c4-a8376deb729f",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// DFT 70 — Transit Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRANSIT_MAGE: CardRecord = CardRecord::new(
    "Transit Mage",
    "6727169f-c33a-4ca5-889d-a63bcfc5a3f0",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// DFT 71 — Trip Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIP_UP: CardRecord = CardRecord::new(
    "Trip Up",
    "273061f3-7aa7-4cb0-afd6-616252b88948",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// DFT 72 — Unstoppable Plan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSTOPPABLE_PLAN: CardRecord = CardRecord::new(
    "Unstoppable Plan",
    "aaeb5981-7e6a-4ffd-bb02-4757b2e92f08",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// DFT 73 — Vnwxt, Verbose Host
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VNWXT_VERBOSE_HOST: CardRecord = CardRecord::new(
    "Vnwxt, Verbose Host",
    "893254c7-64cc-4cb9-b79f-2c41a8935ea0",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// DFT 74 — Waxen Shapethief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAXEN_SHAPETHIEF: CardRecord = CardRecord::new(
    "Waxen Shapethief",
    "412aaa30-b9cd-4cf8-beb8-1c1229667b31",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// DFT 75 — Ancient Vendetta
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_VENDETTA: CardRecord = CardRecord::new(
    "Ancient Vendetta",
    "230301f2-f288-4b13-9f62-e649ad8357bb",
    "Tianxing Xu",
    crate::card::CardRules::unsupported(),
);

// DFT 76 — Back on Track
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BACK_ON_TRACK: CardRecord = CardRecord::new(
    "Back on Track",
    "884c0032-9c62-4028-a55f-6a3da2545654",
    "Raoul Vitale",
    crate::card::CardRules::unsupported(),
);

// DFT 77 — Bloodghast (reprint)
const BLOODGHAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::BLOODGHAST,
    "fdceefe6-f083-4955-b53a-8e6f8aeb2083",
    "Francisco Badilla",
);

// DFT 78 — Carrion Cruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRION_CRUISER: CardRecord = CardRecord::new(
    "Carrion Cruiser",
    "dc00fdee-3d24-4360-a4d2-ffd3c08a462d",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// DFT 79 — Chitin Gravestalker
pub(in crate::card::sets) static CHITIN_GRAVESTALKER: CardRecord = CardRecord::new(
    "Chitin Gravestalker",
    "903b4141-04a3-44c4-9d3e-aa2a773d9883",
    "Slawomir Maniak",
// Cycling is what makes the discount reachable: the card fills the
    // graveyard it later reads, including with copies of itself.
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Insect", "Warrior"], 5, 4).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each artifact and/or creature card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
            )),
        )
        // Read from hand, where the cost is paid.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// DFT 80 — Cryptcaller Chariot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPTCALLER_CHARIOT: CardRecord = CardRecord::new(
    "Cryptcaller Chariot",
    "a0c8259c-055e-4bff-b945-c0ecb057a8f0",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// DFT 81 — Cursecloth Wrappings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURSECLOTH_WRAPPINGS: CardRecord = CardRecord::new(
    "Cursecloth Wrappings",
    "d5803b32-4a81-46c2-9b10-3198a709611d",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// DFT 82 — Deathless Pilot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATHLESS_PILOT: CardRecord = CardRecord::new(
    "Deathless Pilot",
    "e704fb95-17b7-432a-831c-18abe7d9cc73",
    "Justin Cornell",
    crate::card::CardRules::unsupported(),
);

// DFT 83 — Demonic Junker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMONIC_JUNKER: CardRecord = CardRecord::new(
    "Demonic Junker",
    "4aad569e-4acb-4416-9d4f-64e6991de3ed",
    "Stephan Martiniere",
    crate::card::CardRules::unsupported(),
);

// DFT 84 — Engine Rat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENGINE_RAT: CardRecord = CardRecord::new(
    "Engine Rat",
    "949d6137-98d3-4f46-ab1e-08d7492af307",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// DFT 85 — Gas Guzzler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAS_GUZZLER: CardRecord = CardRecord::new(
    "Gas Guzzler",
    "4db3a28c-e4b4-4b18-8d56-e3842184d105",
    "Yohann Schepacz",
    crate::card::CardRules::unsupported(),
);

// DFT 86 — Gastal Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GASTAL_RAIDER: CardRecord = CardRecord::new(
    "Gastal Raider",
    "6e4877b5-4ce5-466a-810f-6501f2a0f217",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// DFT 87 — Gonti, Night Minister
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GONTI_NIGHT_MINISTER: CardRecord = CardRecord::new(
    "Gonti, Night Minister",
    "d79ca40a-e5c0-4956-8df0-ecbd2a25656f",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// DFT 88 — Grim Bauble
pub(in crate::card::sets) static GRIM_BAUBLE: CardRecord = CardRecord::new(
    "Grim Bauble",
    "9bfdf60a-6f67-4872-8961-d63776b192c3",
    "Wero Gallo",
    // One mana kills an early creature and the artifact stays behind, which
    // is what makes the four-mana surveil a bonus rather than the plan.
    CardRules::new_artifact(mana_cost!("{B}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, target creature an opponent controls gets -2/-2 until \
             end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{2}{B}, {T}, Sacrifice this artifact: Surveil 2. (Look at the top two cards of your \
             library, then put any number of them into your graveyard and the rest on top of \
             your library in any order.)",
            &[
                CostDef::Mana(mana_cost!("{2}{B}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::surveil(ValueDef::Constant(2)),
        ),
    ]),
);

// DFT 89 — Grim Javelineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIM_JAVELINEER: CardRecord = CardRecord::new(
    "Grim Javelineer",
    "87154116-e306-4e15-bd5a-dcdb5ddbcd36",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// DFT 90 — Hellish Sideswipe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELLISH_SIDESWIPE: CardRecord = CardRecord::new(
    "Hellish Sideswipe",
    "7a9db650-47f9-46d7-ac17-8d19fef6d6b0",
    "Diana Franco",
    crate::card::CardRules::unsupported(),
);

// DFT 91 — Hour of Victory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOUR_OF_VICTORY: CardRecord = CardRecord::new(
    "Hour of Victory",
    "9192abc8-05a3-4e72-a634-fc5acbe97b26",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// DFT 92 — Intimidation Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTIMIDATION_TACTICS: CardRecord = CardRecord::new(
    "Intimidation Tactics",
    "9b4e6022-44d2-4dfe-8f7a-51581e298f23",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// DFT 93 — Kalakscion, Hunger Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KALAKSCION_HUNGER_TYRANT: CardRecord = CardRecord::new(
    "Kalakscion, Hunger Tyrant",
    "1214fc6d-ae47-418d-88cc-58633ec2ac7a",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// DFT 94 — The Last Ride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_LAST_RIDE: CardRecord = CardRecord::new(
    "The Last Ride",
    "9cbb7b4e-bd32-44a0-9396-16738c5e4381",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// DFT 95 — Locust Spray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOCUST_SPRAY: CardRecord = CardRecord::new(
    "Locust Spray",
    "54b3a547-6f74-4cb4-ad98-7e1b75f1a120",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// DFT 96 — Maximum Overdrive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAXIMUM_OVERDRIVE: CardRecord = CardRecord::new(
    "Maximum Overdrive",
    "5f6e5bb2-cfe9-48e5-86f9-e21f3d328327",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// DFT 97 — Momentum Breaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENTUM_BREAKER: CardRecord = CardRecord::new(
    "Momentum Breaker",
    "38513b53-384f-45e7-9905-80dd2c3c4918",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// DFT 98 — Mutant Surveyor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUTANT_SURVEYOR: CardRecord = CardRecord::new(
    "Mutant Surveyor",
    "7cec5105-3907-40d2-8e46-95acfaaaa0cc",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// DFT 99 — Pactdoll Terror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PACTDOLL_TERROR: CardRecord = CardRecord::new(
    "Pactdoll Terror",
    "70226354-47b7-4f9d-a5a8-559d17f07720",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// DFT 100 — Quag Feast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUAG_FEAST: CardRecord = CardRecord::new(
    "Quag Feast",
    "dd8b6033-63e6-484e-8efb-a4eb9ca59fbf",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// DFT 101 — Ripclaw Wrangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPCLAW_WRANGLER: CardRecord = CardRecord::new(
    "Ripclaw Wrangler",
    "d4981a4a-6eca-4f84-8715-8e2672507b59",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// DFT 102 — Risen Necroregent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISEN_NECROREGENT: CardRecord = CardRecord::new(
    "Risen Necroregent",
    "5a68482a-401d-48e7-854e-46e3db07ff35",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// DFT 103 — Risky Shortcut
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISKY_SHORTCUT: CardRecord = CardRecord::new(
    "Risky Shortcut",
    "c80aa587-4445-43d7-abc0-654d94ff4cda",
    "Ignatius Budi",
    crate::card::CardRules::unsupported(),
);

// DFT 104 — Shefet Archfiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHEFET_ARCHFIEND: CardRecord = CardRecord::new(
    "Shefet Archfiend",
    "079dc8d2-0de3-415e-8af8-b8dec669368a",
    "GodMachine",
    crate::card::CardRules::unsupported(),
);

// DFT 105 — The Speed Demon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SPEED_DEMON: CardRecord = CardRecord::new(
    "The Speed Demon",
    "62242a80-0444-4a0e-a868-97eabcc77648",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// DFT 106 — Spin Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIN_OUT: CardRecord = CardRecord::new(
    "Spin Out",
    "be722ac5-e8c4-4180-aed0-7c28895afc0d",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// DFT 107 — Streaking Oilgorger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STREAKING_OILGORGER: CardRecord = CardRecord::new(
    "Streaking Oilgorger",
    "6ff120a2-e2bb-42a2-bcb7-a48eb7a6d9b2",
    "Campbell White",
    crate::card::CardRules::unsupported(),
);

// DFT 108 — Syphon Fuel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYPHON_FUEL: CardRecord = CardRecord::new(
    "Syphon Fuel",
    "4af17ae0-1035-4cb2-8974-98b377bfaa48",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// DFT 109 — Wickerfolk Indomitable
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WICKERFOLK_INDOMITABLE: CardRecord = CardRecord::new(
    "Wickerfolk Indomitable",
    "ba78e076-8962-4b3f-b86f-04400b062951",
    "Sergio Cosmai",
    crate::card::CardRules::unsupported(),
);

// DFT 110 — Wreckage Wickerfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRECKAGE_WICKERFOLK: CardRecord = CardRecord::new(
    "Wreckage Wickerfolk",
    "aeff3db7-81ac-4c51-9954-bc1dbcb8c4e3",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// DFT 111 — Wretched Doll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRETCHED_DOLL: CardRecord = CardRecord::new(
    "Wretched Doll",
    "4983177d-fbf4-47fa-997f-9d08294870f2",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// DFT 112 — Adrenaline Jockey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADRENALINE_JOCKEY: CardRecord = CardRecord::new(
    "Adrenaline Jockey",
    "c8655373-320d-440d-b700-d03413f743fd",
    "Alfonso Santano",
    crate::card::CardRules::unsupported(),
);

// DFT 113 — Boommobile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOOMMOBILE: CardRecord = CardRecord::new(
    "Boommobile",
    "930c8289-4043-401a-8a7f-22349b7148b4",
    "Alexandr Leskinen",
    crate::card::CardRules::unsupported(),
);

// DFT 114 — Burner Rocket
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNER_ROCKET: CardRecord = CardRecord::new(
    "Burner Rocket",
    "ecee6509-1103-4ae5-a2e7-0441f5d4a872",
    "José Parodi",
    crate::card::CardRules::unsupported(),
);

// DFT 115 — Burnout Bashtronaut
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNOUT_BASHTRONAUT: CardRecord = CardRecord::new(
    "Burnout Bashtronaut",
    "4db66e7b-cb7a-4d86-a563-d570946aeb0d",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// DFT 116 — Chandra, Spark Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHANDRA_SPARK_HUNTER: CardRecord = CardRecord::new(
    "Chandra, Spark Hunter",
    "11f9b98e-48a1-491e-bdab-6e94e4ec747a",
    "Devin Elle Kurtz",
    crate::card::CardRules::unsupported(),
);

// DFT 117 — Clamorous Ironclad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLAMOROUS_IRONCLAD: CardRecord = CardRecord::new(
    "Clamorous Ironclad",
    "4701da52-b9c8-4ce9-9d57-53e10899f19d",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// DFT 118 — Count on Luck
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COUNT_ON_LUCK: CardRecord = CardRecord::new(
    "Count on Luck",
    "8f31beae-9b8f-4c32-af84-9f3ee767ba1d",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// DFT 119 — Crash and Burn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRASH_AND_BURN: CardRecord = CardRecord::new(
    "Crash and Burn",
    "339a41a1-b36f-4b81-b74c-220d279c0e26",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// DFT 120 — Daretti, Rocketeer Engineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARETTI_ROCKETEER_ENGINEER: CardRecord = CardRecord::new(
    "Daretti, Rocketeer Engineer",
    "be626e2f-1075-4497-bd5b-bd805777afd3",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// DFT 121 — Draconautics Engineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRACONAUTICS_ENGINEER: CardRecord = CardRecord::new(
    "Draconautics Engineer",
    "c44440be-e611-4e70-9919-b08e2354b7ad",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// DFT 122 — Dracosaur Auxiliary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRACOSAUR_AUXILIARY: CardRecord = CardRecord::new(
    "Dracosaur Auxiliary",
    "cf800b8c-d08e-4644-8ed2-11b839153861",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// DFT 123 — Dynamite Diver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DYNAMITE_DIVER: CardRecord = CardRecord::new(
    "Dynamite Diver",
    "5a4f96f6-dd91-4357-ae19-1c35db2c2bcb",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// DFT 124 — Endrider Catalyzer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDRIDER_CATALYZER: CardRecord = CardRecord::new(
    "Endrider Catalyzer",
    "55a5a67b-d969-4ba4-9dcc-32d0c2e5c04a",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// DFT 125 — Endrider Spikespitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDRIDER_SPIKESPITTER: CardRecord = CardRecord::new(
    "Endrider Spikespitter",
    "4e58cb18-f216-4248-8f0d-65b0263c5c28",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// DFT 126 — Fuel the Flames
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUEL_THE_FLAMES: CardRecord = CardRecord::new(
    "Fuel the Flames",
    "624335fb-8a0b-4fa9-aefb-60ac641a7934",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// DFT 127 — Full Throttle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FULL_THROTTLE: CardRecord = CardRecord::new(
    "Full Throttle",
    "d91f7cad-89e8-45cb-a78e-b35b0ee64783",
    "Benjamin Ee",
    crate::card::CardRules::unsupported(),
);

// DFT 128 — Gastal Blockbuster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GASTAL_BLOCKBUSTER: CardRecord = CardRecord::new(
    "Gastal Blockbuster",
    "dca41ec6-8f8f-42ef-abac-cc645c6440b7",
    "Bryan Sola",
    crate::card::CardRules::unsupported(),
);

// DFT 129 — Gastal Thrillroller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GASTAL_THRILLROLLER: CardRecord = CardRecord::new(
    "Gastal Thrillroller",
    "d8b1762b-ff03-4312-afcc-cb6b5f280ade",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// DFT 130 — Gilded Ghoda
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GILDED_GHODA: CardRecord = CardRecord::new(
    "Gilded Ghoda",
    "55685602-a18c-43a4-ac60-13b039672fa4",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// DFT 131 — Goblin Surveyor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SURVEYOR: CardRecord = CardRecord::new(
    "Goblin Surveyor",
    "e1efffe9-00f8-4177-a9e6-4ad62887d32f",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// DFT 132 — Greasewrench Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREASEWRENCH_GOBLIN: CardRecord = CardRecord::new(
    "Greasewrench Goblin",
    "c8f0b123-fdb0-4f3e-ba78-fa155c227e20",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// DFT 133 — Hazoret, Godseeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZORET_GODSEEKER: CardRecord = CardRecord::new(
    "Hazoret, Godseeker",
    "e2f66043-0872-4334-a91f-0e9bbbdddf66",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// DFT 134 — Howlsquad Heavy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOWLSQUAD_HEAVY: CardRecord = CardRecord::new(
    "Howlsquad Heavy",
    "df582f80-7b9a-4f71-95a9-70548ec7d2d7",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// DFT 135 — Kickoff Celebrations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KICKOFF_CELEBRATIONS: CardRecord = CardRecord::new(
    "Kickoff Celebrations",
    "4e5590e1-0ac0-4bdd-815b-136bf24ced03",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// DFT 136 — Lightning Strike (reprint)
const LIGHTNING_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::LIGHTNING_STRIKE,
    "30077b49-b825-4dbb-a0c7-f3992f647df0",
    "Steve Ellis",
);

// DFT 137 — Magmakin Artillerist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMAKIN_ARTILLERIST: CardRecord = CardRecord::new(
    "Magmakin Artillerist",
    "2ac26341-a100-424d-be84-33fbbf1b4078",
    "Madeline Boni",
    crate::card::CardRules::unsupported(),
);

// DFT 138 — Marauding Mako
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARAUDING_MAKO: CardRecord = CardRecord::new(
    "Marauding Mako",
    "9efbfd67-e0f5-43e0-9fff-1eb4a2bed0d8",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// DFT 139 — Outpace Oblivion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTPACE_OBLIVION: CardRecord = CardRecord::new(
    "Outpace Oblivion",
    "c22e415f-636f-4394-9b21-600ab720ac98",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// DFT 140 — Pacesetter Paragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PACESETTER_PARAGON: CardRecord = CardRecord::new(
    "Pacesetter Paragon",
    "7364b4dc-8cce-498d-a62e-eabc612b062a",
    "Wonchun Choi",
    crate::card::CardRules::unsupported(),
);

// DFT 141 — Pedal to the Metal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEDAL_TO_THE_METAL: CardRecord = CardRecord::new(
    "Pedal to the Metal",
    "75d07295-78b7-4f90-82c7-e7a639db9993",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// DFT 142 — Prowcatcher Specialist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROWCATCHER_SPECIALIST: CardRecord = CardRecord::new(
    "Prowcatcher Specialist",
    "affbc8db-4ff7-4a86-954a-910493e5a2ef",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// DFT 143 — Push the Limit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUSH_THE_LIMIT: CardRecord = CardRecord::new(
    "Push the Limit",
    "21de84a2-2654-4e1a-a569-bf385bb43685",
    "Alexander Mokhov",
    crate::card::CardRules::unsupported(),
);

// DFT 144 — Reckless Velocitaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_VELOCITAUR: CardRecord = CardRecord::new(
    "Reckless Velocitaur",
    "8edd18be-3861-4510-ba6d-38ccba60bb5b",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// DFT 145 — Road Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROAD_RAGE: CardRecord = CardRecord::new(
    "Road Rage",
    "fca022de-0c7b-48ea-b3b5-af28987a5070",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// DFT 146 — Skycrash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYCRASH: CardRecord = CardRecord::new(
    "Skycrash",
    "87e82be6-d2b4-4c95-8466-477e8c6832e9",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// DFT 147 — Spire Mechcycle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRE_MECHCYCLE: CardRecord = CardRecord::new(
    "Spire Mechcycle",
    "f483debe-9c54-4323-aec5-226587ece2c9",
    "Adam Volker",
    crate::card::CardRules::unsupported(),
);

// DFT 148 — Thunderhead Gunner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERHEAD_GUNNER: CardRecord = CardRecord::new(
    "Thunderhead Gunner",
    "ef537868-b2cb-4bbd-a935-b7f73f19bd06",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// DFT 149 — Tyrox, Saurid Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TYROX_SAURID_TYRANT: CardRecord = CardRecord::new(
    "Tyrox, Saurid Tyrant",
    "159dc5a8-5cba-4c20-8c07-28a3d86c8411",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// DFT 150 — Afterburner Expert
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AFTERBURNER_EXPERT: CardRecord = CardRecord::new(
    "Afterburner Expert",
    "555e1bfc-6d07-4979-a914-b2bd1fb031f2",
    "April Prime",
    crate::card::CardRules::unsupported(),
);

// DFT 151 — Agonasaur Rex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGONASAUR_REX: CardRecord = CardRecord::new(
    "Agonasaur Rex",
    "5b11ec26-9e07-45e1-bcaa-5d44d1231586",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// DFT 152 — Alacrian Jaguar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALACRIAN_JAGUAR: CardRecord = CardRecord::new(
    "Alacrian Jaguar",
    "2bd40bca-aa54-4e52-8d48-b3709a11e633",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// DFT 153 — Autarch Mammoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUTARCH_MAMMOTH: CardRecord = CardRecord::new(
    "Autarch Mammoth",
    "4d313ea7-2456-48f3-8b12-97d8e8c2a5b3",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// DFT 154 — Beastrider Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEASTRIDER_VANGUARD: CardRecord = CardRecord::new(
    "Beastrider Vanguard",
    "f4b99a61-49f1-46a2-9eb7-b6c88c236215",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// DFT 155 — Bestow Greatness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BESTOW_GREATNESS: CardRecord = CardRecord::new(
    "Bestow Greatness",
    "b6a5adee-3482-4c5b-9260-58efa8fec5ba",
    "Jeff Carpenter",
    crate::card::CardRules::unsupported(),
);

// DFT 156 — Broken Wings (reprint)
const BROKEN_WINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::BROKEN_WINGS,
    "1d7d5b71-7c1b-4fc2-a5ec-7285e17ffe0f",
    "Nils Hamm",
);

// DFT 157 — Defend the Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFEND_THE_RIDER: CardRecord = CardRecord::new(
    "Defend the Rider",
    "59ed23a2-6153-47b2-ab73-062195cafb74",
    "Raph Lomotan",
    crate::card::CardRules::unsupported(),
);

// DFT 158 — District Mascot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISTRICT_MASCOT: CardRecord = CardRecord::new(
    "District Mascot",
    "3171b16f-cd67-416d-be5e-5d9cccb8d0a0",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// DFT 159 — Dredger's Insight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREDGER_S_INSIGHT: CardRecord = CardRecord::new(
    "Dredger's Insight",
    "148400a0-7819-4551-9815-9357eed1db4d",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// DFT 160 — Earthrumbler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHRUMBLER: CardRecord = CardRecord::new(
    "Earthrumbler",
    "ee386cd0-934a-4b33-9db3-0a9033ab577e",
    "J.P. Targete",
    crate::card::CardRules::unsupported(),
);

// DFT 161 — Elvish Refueler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_REFUELER: CardRecord = CardRecord::new(
    "Elvish Refueler",
    "25dfbcb6-9b67-4151-b10f-dde70c5fd16d",
    "Carly Milligan",
    crate::card::CardRules::unsupported(),
);

// DFT 162 — Fang Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FANG_GUARDIAN: CardRecord = CardRecord::new(
    "Fang Guardian",
    "6db483a7-c9f0-449d-be97-77dbd6d3a27a",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// DFT 163 — Fang-Druid Summoner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FANG_DRUID_SUMMONER: CardRecord = CardRecord::new(
    "Fang-Druid Summoner",
    "496442f6-48c7-464e-bcf3-4c14f49fa065",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// DFT 164 — Greenbelt Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREENBELT_GUARDIAN: CardRecord = CardRecord::new(
    "Greenbelt Guardian",
    "2f6b0000-5fd3-483b-bac5-f9b888d8755b",
    "Tianxing Xu",
    crate::card::CardRules::unsupported(),
);

// DFT 165 — Hazard of the Dunes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZARD_OF_THE_DUNES: CardRecord = CardRecord::new(
    "Hazard of the Dunes",
    "c3fbee7a-7e73-43cf-bc39-ccb1c63bf837",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// DFT 166 — Jibbirik Omnivore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JIBBIRIK_OMNIVORE: CardRecord = CardRecord::new(
    "Jibbirik Omnivore",
    "68a0569b-65c8-49ce-91ac-e639b8faf939",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// DFT 167 — Loxodon Surveyor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOXODON_SURVEYOR: CardRecord = CardRecord::new(
    "Loxodon Surveyor",
    "cd1cecb1-6776-4495-be56-b7dde65453f1",
    "J.P. Targete",
    crate::card::CardRules::unsupported(),
);

// DFT 168 — Lumbering Worldwagon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMBERING_WORLDWAGON: CardRecord = CardRecord::new(
    "Lumbering Worldwagon",
    "9f989c59-7b2a-4036-9ea2-cd0c7e85c15b",
    "Raph Lomotan",
    crate::card::CardRules::unsupported(),
);

// DFT 169 — March of the World Ooze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARCH_OF_THE_WORLD_OOZE: CardRecord = CardRecord::new(
    "March of the World Ooze",
    "b1964ec5-0dd1-4b54-917c-cbeab05aba79",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// DFT 170 — Migrating Ketradon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIGRATING_KETRADON: CardRecord = CardRecord::new(
    "Migrating Ketradon",
    "a9ba2219-8184-4141-8708-845cb0957299",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// DFT 171 — Molt Tender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLT_TENDER: CardRecord = CardRecord::new(
    "Molt Tender",
    "f800bf4e-4bfb-45b6-950b-c76952f52bb1",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// DFT 172 — Ooze Patrol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OOZE_PATROL: CardRecord = CardRecord::new(
    "Ooze Patrol",
    "101d22c6-830d-4908-9003-6b206f694eba",
    "Forrest Schehl",
    crate::card::CardRules::unsupported(),
);

// DFT 173 — Oviya, Automech Artisan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVIYA_AUTOMECH_ARTISAN: CardRecord = CardRecord::new(
    "Oviya, Automech Artisan",
    "ee5f504c-33fc-4a91-b69b-8ef555987c79",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// DFT 174 — Plow Through
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLOW_THROUGH: CardRecord = CardRecord::new(
    "Plow Through",
    "a311d4b3-ab2a-43c2-8480-6c5daac41178",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// DFT 175 — Point the Way
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POINT_THE_WAY: CardRecord = CardRecord::new(
    "Point the Way",
    "8fd4c73d-0e9a-4ffe-8062-f2f4d0e601fe",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// DFT 176 — Pothole Mole
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POTHOLE_MOLE: CardRecord = CardRecord::new(
    "Pothole Mole",
    "21c7b59f-fdae-4a11-9784-497a8165d75a",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// DFT 177 — Regal Imperiosaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REGAL_IMPERIOSAUR: CardRecord = CardRecord::new(
    "Regal Imperiosaur",
    "36f569cc-ed09-4c27-b753-18b22ad7f425",
    "Stephanie Cheung",
    crate::card::CardRules::unsupported(),
);

// DFT 178 — Rise from the Wreck
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISE_FROM_THE_WRECK: CardRecord = CardRecord::new(
    "Rise from the Wreck",
    "43e6ac32-a7a4-4c15-81b0-8485d6a0e7ca",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// DFT 179 — Run Over
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUN_OVER: CardRecord = CardRecord::new(
    "Run Over",
    "642f8fdd-6c58-49a3-904c-27f717cae980",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// DFT 180 — Silken Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILKEN_STRENGTH: CardRecord = CardRecord::new(
    "Silken Strength",
    "ce0e1ded-9b00-4d7b-884c-70a429783b1f",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// DFT 181 — Stampeding Scurryfoot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMPEDING_SCURRYFOOT: CardRecord = CardRecord::new(
    "Stampeding Scurryfoot",
    "7fc713d7-4a7e-4f1f-b461-e89bb1457d7d",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// DFT 182 — Terrian, World Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRIAN_WORLD_TYRANT: CardRecord = CardRecord::new(
    "Terrian, World Tyrant",
    "b44255cf-5264-4ead-9de0-20cc0f7cac6f",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// DFT 183 — Thunderous Velocipede
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDEROUS_VELOCIPEDE: CardRecord = CardRecord::new(
    "Thunderous Velocipede",
    "98a79557-8ed6-4d9a-b4e1-cece05664984",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// DFT 184 — Veloheart Bike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VELOHEART_BIKE: CardRecord = CardRecord::new(
    "Veloheart Bike",
    "85edde1a-f05c-4ce8-bedd-88359647aa0d",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// DFT 185 — Venomsac Lagac
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMSAC_LAGAC: CardRecord = CardRecord::new(
    "Venomsac Lagac",
    "9142b1c9-09d7-42e4-8138-6c15d31f2470",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// DFT 186 — Webstrike Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEBSTRIKE_ELITE: CardRecord = CardRecord::new(
    "Webstrike Elite",
    "064cc22d-d424-4bc9-b8f0-88b170fd6c28",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// DFT 187 — Aatchik, Emerald Radian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AATCHIK_EMERALD_RADIAN: CardRecord = CardRecord::new(
    "Aatchik, Emerald Radian",
    "fbdaa29b-85ff-4a06-b27e-fcdbdfd4a3fe",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// DFT 188 — Apocalypse Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APOCALYPSE_RUNNER: CardRecord = CardRecord::new(
    "Apocalypse Runner",
    "971286f5-77ea-4824-91de-f0ac88c46238",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// DFT 189 — Boom Scholar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOOM_SCHOLAR: CardRecord = CardRecord::new(
    "Boom Scholar",
    "f2b84684-10c9-4635-a922-620f04809bb1",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// DFT 190 — Boosted Sloop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOOSTED_SLOOP: CardRecord = CardRecord::new(
    "Boosted Sloop",
    "3563cb48-9dcb-4b92-af3a-4793adf03125",
    "José Parodi",
    crate::card::CardRules::unsupported(),
);

// DFT 191 — Brightglass Gearhulk
pub(in crate::card::sets) static BRIGHTGLASS_GEARHULK: CardRecord = CardRecord::new(
    "Brightglass Gearhulk",
    "3dea5b45-925c-4732-8e9d-fa8232792736",
    "José Parodi",
// A 4/4 first striker with trample that also finds the two one-drops the
    // deck is built around, which is what four coloured pips buy.
    CardRules::new_artifact_creature(mana_cost!("{G}{G}{W}{W}"), &["Construct"], 4, 4)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::trample(),
            // "You may" on top of a search that already allows none: declining and
            // finding nothing look the same from the outside, and the card offers
            // both because a library nobody wants to shuffle is a real answer.
            abilities::enters_trigger(
                "When this creature enters, you may search your library for up to two artifact, creature, \
                 and/or enchantment cards with mana value 1 or less, reveal them, put them into your \
                 hand, then shuffle.",
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // "Up to two" and revealed: a minimum of none, and everything taken is
                    // shown, which is what stops the search being private information.
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        // "Artifact, creature, and/or enchantment cards with mana value 1 or less."
                        // The three types are alternatives and the mana value applies to all of
                        // them, so the bound is outside the choice rather than inside it.
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                            ]),
                            ObjectPredicateDef::ManaValueAtMost(1),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(2),
                        reveal: true,
                        destination: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
            ),
        ]),
);

// DFT 192 — Broadside Barrage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROADSIDE_BARRAGE: CardRecord = CardRecord::new(
    "Broadside Barrage",
    "d086e4e5-98fa-437e-85aa-d8849c94ce94",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// DFT 193 — Broodheart Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROODHEART_ENGINE: CardRecord = CardRecord::new(
    "Broodheart Engine",
    "ed09139b-a66e-4cd1-b45a-23a4648d3401",
    "Rémi Jacquot",
    crate::card::CardRules::unsupported(),
);

// DFT 194 — Captain Howler, Sea Scourge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_HOWLER_SEA_SCOURGE: CardRecord = CardRecord::new(
    "Captain Howler, Sea Scourge",
    "0957c90f-e10d-40f8-a4be-9e9ef623dd43",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// DFT 195 — Caradora, Heart of Alacria
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARADORA_HEART_OF_ALACRIA: CardRecord = CardRecord::new(
    "Caradora, Heart of Alacria",
    "1256d22d-a2a9-41fb-b669-0661ba230bc7",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// DFT 196 — Cloudspire Coordinator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDSPIRE_COORDINATOR: CardRecord = CardRecord::new(
    "Cloudspire Coordinator",
    "eef16cda-9150-4e7d-8490-d9f287b81b62",
    "Eduardo Francisco",
    crate::card::CardRules::unsupported(),
);

// DFT 197 — Cloudspire Skycycle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDSPIRE_SKYCYCLE: CardRecord = CardRecord::new(
    "Cloudspire Skycycle",
    "881f01a4-a923-4d56-bacb-984a389296fa",
    "Hardy Fowler",
    crate::card::CardRules::unsupported(),
);

// DFT 198 — Coalstoke Gearhulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COALSTOKE_GEARHULK: CardRecord = CardRecord::new(
    "Coalstoke Gearhulk",
    "73431628-b9b0-41e6-8e9b-8a090939b0c1",
    "Nino Vecia",
    crate::card::CardRules::unsupported(),
);

// DFT 199 — Debris Beetle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEBRIS_BEETLE: CardRecord = CardRecord::new(
    "Debris Beetle",
    "405e7900-f6df-4033-b70b-b1d2a8b6d7c8",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// DFT 200 — Dune Drifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUNE_DRIFTER: CardRecord = CardRecord::new(
    "Dune Drifter",
    "36185de4-55c2-4e5d-9bcc-ea12b7052728",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// DFT 201 — Embalmed Ascendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBALMED_ASCENDANT: CardRecord = CardRecord::new(
    "Embalmed Ascendant",
    "5cf181ae-daa7-42f9-b667-5e679d80cf34",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// DFT 202 — Explosive Getaway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPLOSIVE_GETAWAY: CardRecord = CardRecord::new(
    "Explosive Getaway",
    "a876edac-b8c8-4994-94f5-548e0fe70fe4",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// DFT 203 — Far Fortune, End Boss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAR_FORTUNE_END_BOSS: CardRecord = CardRecord::new(
    "Far Fortune, End Boss",
    "f523e96d-9df1-4854-accb-9876aef787e5",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// DFT 204 — Fearless Swashbuckler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEARLESS_SWASHBUCKLER: CardRecord = CardRecord::new(
    "Fearless Swashbuckler",
    "0d86d66b-481f-44ec-86d8-6fc91b52ef38",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// DFT 205 — Gastal Thrillseeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GASTAL_THRILLSEEKER: CardRecord = CardRecord::new(
    "Gastal Thrillseeker",
    "a8e5205d-d734-4292-a3c8-70cf5f131289",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// DFT 206 — Guidelight Pathmaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDELIGHT_PATHMAKER: CardRecord = CardRecord::new(
    "Guidelight Pathmaker",
    "b2e00cd7-925e-4bab-b064-c96aa2935f8c",
    "Stephan Martiniere",
    crate::card::CardRules::unsupported(),
);

// DFT 207 — Haunt the Network
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNT_THE_NETWORK: CardRecord = CardRecord::new(
    "Haunt the Network",
    "478d236c-9778-435a-ac21-bd0017a17d5b",
    "Jeff Carpenter",
    crate::card::CardRules::unsupported(),
);

// DFT 208 — Haunted Hellride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTED_HELLRIDE: CardRecord = CardRecord::new(
    "Haunted Hellride",
    "e2ce0d37-d5d7-49dd-885e-9998bb8abede",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// DFT 209 — Ketramose, the New Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KETRAMOSE_THE_NEW_DAWN: CardRecord = CardRecord::new(
    "Ketramose, the New Dawn",
    "cffae8d0-7b4e-42ed-8124-24a86b38f490",
    "Maaz Ali Khan",
    crate::card::CardRules::unsupported(),
);

// DFT 210 — Kolodin, Triumph Caster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOLODIN_TRIUMPH_CASTER: CardRecord = CardRecord::new(
    "Kolodin, Triumph Caster",
    "36bcd476-a236-4434-b191-5c8b6fa7be7b",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// DFT 211 — Lagorin, Soul of Alacria
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAGORIN_SOUL_OF_ALACRIA: CardRecord = CardRecord::new(
    "Lagorin, Soul of Alacria",
    "7b04e3f4-103b-4845-b3f0-5417971c7666",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// DFT 212 — Loot, the Pathfinder
pub(in crate::card::sets) static LOOT_THE_PATHFINDER: CardRecord = CardRecord::new(
    "Loot, the Pathfinder",
    "33c59c04-4c0b-4a60-826e-3a7757d0b2a2",
    "Ernanda Souza",
// Five mana for a hasty double striker that also unloads three cards,
    // three mana, or three damage -- once each, and never twice, because
    // every one of them taps it.
    CardRules::new_creature(mana_cost!("{2}{G}{U}{R}"), &["Beast", "Noble"], 2, 4)
        .with_supertype(crate::card::CardSupertype::Legendary)
        .with_abilities(&[
            abilities::double_strike(),
            abilities::vigilance(),
            abilities::haste(),
            exhaust(AbilityDef::activated_mana(
                "Exhaust — {G}, {T}: Add three mana of any one color. (Activate each exhaust ability \
                 only once.)",
                &[
                    CostDef::Mana(mana_cost!("{G}")),
                    CostDef::TapSource,
                ],
                EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
            )),
            exhaust(AbilityDef::activated(
                "Exhaust — {U}, {T}: Draw three cards.",
                &[
                    CostDef::Mana(mana_cost!("{U}")),
                    CostDef::TapSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            )),
            exhaust(AbilityDef::activated_with_targets(
                "Exhaust — {R}, {T}: This creature deals 3 damage to any target.",
                &[
                    CostDef::Mana(mana_cost!("{R}")),
                    CostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            )),
        ]),
);

// DFT 213 — Mendicant Core, Guidelight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MENDICANT_CORE_GUIDELIGHT: CardRecord = CardRecord::new(
    "Mendicant Core, Guidelight",
    "f434b103-490f-424e-a0a1-efb1b931c8e6",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// DFT 214 — Mimeoplasm, Revered One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIMEOPLASM_REVERED_ONE: CardRecord = CardRecord::new(
    "Mimeoplasm, Revered One",
    "34e4c342-dc22-4e9c-81fc-a691ae9e21c1",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// DFT 215 — Oildeep Gearhulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OILDEEP_GEARHULK: CardRecord = CardRecord::new(
    "Oildeep Gearhulk",
    "a5e2d09e-b9f5-4a0d-96d3-984b5c2c387d",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// DFT 216 — Pyrewood Gearhulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYREWOOD_GEARHULK: CardRecord = CardRecord::new(
    "Pyrewood Gearhulk",
    "3d8493ee-bd06-4583-9908-a0dbcc5be6d7",
    "Martin de Diego Sádaba",
    crate::card::CardRules::unsupported(),
);

// DFT 217 — Rangers' Aetherhive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANGERS_AETHERHIVE: CardRecord = CardRecord::new(
    "Rangers' Aetherhive",
    "1b238d2c-d10f-496d-aa34-5a1536e056b5",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// DFT 218 — Redshift, Rocketeer Chief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDSHIFT_ROCKETEER_CHIEF: CardRecord = CardRecord::new(
    "Redshift, Rocketeer Chief",
    "5fba3820-32ba-47e9-9fa8-f985ff471b3f",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// DFT 219 — Riptide Gearhulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_GEARHULK: CardRecord = CardRecord::new(
    "Riptide Gearhulk",
    "44bfb0f7-18ca-4f6e-ba64-92120010456e",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// DFT 220 — Rocketeer Boostbuggy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCKETEER_BOOSTBUGGY: CardRecord = CardRecord::new(
    "Rocketeer Boostbuggy",
    "4c80c91e-dd3d-4c7b-89e4-bfb253eeaee2",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// DFT 221 — Sab-Sunen, Luxa Embodied
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAB_SUNEN_LUXA_EMBODIED: CardRecord = CardRecord::new(
    "Sab-Sunen, Luxa Embodied",
    "2ef555b1-666d-4386-8983-0e88f9b6cdec",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// DFT 222 — Samut, the Driving Force
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMUT_THE_DRIVING_FORCE: CardRecord = CardRecord::new(
    "Samut, the Driving Force",
    "8efd8222-5c37-46d8-a2ec-1d7aae25320b",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// DFT 223 — Sita Varma, Masked Racer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SITA_VARMA_MASKED_RACER: CardRecord = CardRecord::new(
    "Sita Varma, Masked Racer",
    "eb523c06-6f3e-4e19-b777-19aefc4a4e02",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// DFT 224 — Skyserpent Seeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSERPENT_SEEKER: CardRecord = CardRecord::new(
    "Skyserpent Seeker",
    "8dbd9fc0-c0df-4119-a0d3-2e1790998c21",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// DFT 225 — Thundering Broodwagon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERING_BROODWAGON: CardRecord = CardRecord::new(
    "Thundering Broodwagon",
    "2d1576d4-15a3-4e91-84ef-e71e258185e7",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// DFT 226 — Veteran Beastrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VETERAN_BEASTRIDER: CardRecord = CardRecord::new(
    "Veteran Beastrider",
    "cab38f35-b60c-464c-a0b5-9d5f2dc6de7f",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// DFT 227 — Voyage Home
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOYAGE_HOME: CardRecord = CardRecord::new(
    "Voyage Home",
    "4ba835da-0247-4716-9079-1b605297f6d5",
    "Hardy Fowler",
    crate::card::CardRules::unsupported(),
);

// DFT 228 — Winter, Cursed Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_CURSED_RIDER: CardRecord = CardRecord::new(
    "Winter, Cursed Rider",
    "02d46d0e-3161-45b5-a49e-5cd592c67ddd",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// DFT 229 — Zahur, Glory's Past
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZAHUR_GLORY_S_PAST: CardRecord = CardRecord::new(
    "Zahur, Glory's Past",
    "31944ea5-045d-481d-9aff-3c7ed663813a",
    "Leroy Steinmann",
    crate::card::CardRules::unsupported(),
);

// DFT 230 — Aetherjacket
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHERJACKET: CardRecord = CardRecord::new(
    "Aetherjacket",
    "0a29c8a5-fd98-422c-9518-2db0993495e5",
    "Domenico Cava",
    crate::card::CardRules::unsupported(),
);

// DFT 231 — The Aetherspark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_AETHERSPARK: CardRecord = CardRecord::new(
    "The Aetherspark",
    "05690d52-06c4-40b1-8360-380418a83250",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// DFT 232 — Camera Launcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAMERA_LAUNCHER: CardRecord = CardRecord::new(
    "Camera Launcher",
    "968651db-92fb-46cd-acda-9e097668b7c9",
    "Ben Wootten",
    crate::card::CardRules::unsupported(),
);

// DFT 233 — Guidelight Matrix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDELIGHT_MATRIX: CardRecord = CardRecord::new(
    "Guidelight Matrix",
    "cccf7fb5-c043-4a1f-ad2f-edb280cb5037",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// DFT 234 — Lifecraft Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIFECRAFT_ENGINE: CardRecord = CardRecord::new(
    "Lifecraft Engine",
    "40c92203-17df-4f10-92c6-ebcc79f01357",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// DFT 235 — Marketback Walker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARKETBACK_WALKER: CardRecord = CardRecord::new(
    "Marketback Walker",
    "c2152030-6007-493f-a616-545723a00249",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// DFT 236 — Marshals' Pathcruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARSHALS_PATHCRUISER: CardRecord = CardRecord::new(
    "Marshals' Pathcruiser",
    "8f920f83-6380-4e4f-be68-6bf9df3110d8",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// DFT 237 — Monument to Endurance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONUMENT_TO_ENDURANCE: CardRecord = CardRecord::new(
    "Monument to Endurance",
    "d21433ba-0a14-42bc-ad0b-a4ef823a3295",
    "Victor Sales",
    crate::card::CardRules::unsupported(),
);

// DFT 238 — Pit Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIT_AUTOMATON: CardRecord = CardRecord::new(
    "Pit Automaton",
    "c72527ef-ac05-44c8-8c76-10532ce3da6e",
    "Villarrte",
    crate::card::CardRules::unsupported(),
);

// DFT 239 — Racers' Scoreboard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RACERS_SCOREBOARD: CardRecord = CardRecord::new(
    "Racers' Scoreboard",
    "50bae2ba-a6a0-4a6a-96e9-0e0372e55108",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// DFT 240 — Radiant Lotus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_LOTUS: CardRecord = CardRecord::new(
    "Radiant Lotus",
    "be6dac83-39c2-40dc-a322-76a3ea4e7aee",
    "Bruce Brenneise",
    crate::card::CardRules::unsupported(),
);

// DFT 241 — Rover Blades
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROVER_BLADES: CardRecord = CardRecord::new(
    "Rover Blades",
    "8855a999-83f6-4c0d-9444-3ff292f77d58",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// DFT 242 — Scrap Compactor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAP_COMPACTOR: CardRecord = CardRecord::new(
    "Scrap Compactor",
    "12ccd555-60d4-49a1-b022-1e119344172a",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// DFT 243 — Skybox Ferry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYBOX_FERRY: CardRecord = CardRecord::new(
    "Skybox Ferry",
    "75d9ef34-ae95-4465-a30c-372e231bd733",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// DFT 244 — Starting Column
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARTING_COLUMN: CardRecord = CardRecord::new(
    "Starting Column",
    "0530b343-98c2-440a-b32e-d1566d318c3b",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// DFT 245 — Ticket Tortoise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TICKET_TORTOISE: CardRecord = CardRecord::new(
    "Ticket Tortoise",
    "fa178ed7-8f3a-45f0-817d-5fbc7993b04a",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// DFT 246 — Walking Sarcophagus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_SARCOPHAGUS: CardRecord = CardRecord::new(
    "Walking Sarcophagus",
    "89ccbd73-9414-48a3-bdcf-e838fcffc08f",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// DFT 247 — Wreck Remover
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRECK_REMOVER: CardRecord = CardRecord::new(
    "Wreck Remover",
    "e3151960-cc0c-47b5-b476-295d7a17ae14",
    "Villarrte",
    crate::card::CardRules::unsupported(),
);

// DFT 248 — Amonkhet Raceway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMONKHET_RACEWAY: CardRecord = CardRecord::new(
    "Amonkhet Raceway",
    "4f312807-ea2a-4385-8774-4e23b4a5d4a6",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// DFT 249 — Avishkar Raceway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVISHKAR_RACEWAY: CardRecord = CardRecord::new(
    "Avishkar Raceway",
    "08a6b378-c7fa-4226-a310-4ee7e550b4d6",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// DFT 250 — Bleachbone Verge
pub(in crate::card::sets) static BLEACHBONE_VERGE: CardRecord = CardRecord::new(
    "Bleachbone Verge",
    "52dcdabd-a186-45fe-9fee-6c0f1afeaf16",
    "Mark Tedin",
    // Untapped and free either way: the black is unconditional, and the
    // white is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {W}. Activate only if you control a Plains or a Swamp.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Orzhov colours. Either type answers
                // it, so a Godless Shrine is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Plains,
                        BasicLandType::Swamp,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ]),
);

// DFT 251 — Bloodfell Caves (reprint)
const BLOODFELL_CAVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOODFELL_CAVES,
    "49195d90-de0c-4290-aa1c-9f4d948b5521",
    "Ron Spencer",
);

// DFT 252 — Blossoming Sands (reprint)
const BLOSSOMING_SANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOSSOMING_SANDS,
    "69949863-2510-4fe2-a815-0682beeb08a3",
    "Valera Lutfullina",
);

// DFT 253 — Country Roads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COUNTRY_ROADS: CardRecord = CardRecord::new(
    "Country Roads",
    "897acd91-12ba-4fa8-a26e-c09f009167a8",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// DFT 254 — Dismal Backwater (reprint)
const DISMAL_BACKWATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::DISMAL_BACKWATER,
    "f20aef3f-79f6-4357-8631-1d141f437def",
    "Wayne Wu",
);

// DFT 255 — Foul Roads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUL_ROADS: CardRecord = CardRecord::new(
    "Foul Roads",
    "a37c026a-c89f-41f3-8812-424124dd3760",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// DFT 256 — Jungle Hollow (reprint)
const JUNGLE_HOLLOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::JUNGLE_HOLLOW,
    "7dd24a1a-08db-4fa7-8939-f5541677327e",
    "Eddie Mendoza",
);

// DFT 257 — Muraganda Raceway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MURAGANDA_RACEWAY: CardRecord = CardRecord::new(
    "Muraganda Raceway",
    "5041ae16-29ff-4ad5-8a37-4736e9409294",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// DFT 258 — Night Market
pub(in crate::card::sets) static NIGHT_MARKET: CardRecord = CardRecord::new(
    "Night Market",
    "a8c1dce3-6136-4294-9d2b-5ef8527d733b",
    "David Álvarez",
    // A tapped land that fixes one colour and cycles away once the mana is
    // there, so it is never the draw that loses the game.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::as_enters(
            "As this land enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of the chosen color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
        ),
        abilities::cycling!(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{3}"))],
        ),
    ]),
);

// DFT 259 — Reef Roads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REEF_ROADS: CardRecord = CardRecord::new(
    "Reef Roads",
    "73a8171a-2629-4356-ae86-b4d03aa14bd3",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// DFT 260 — Riverpyre Verge
pub(in crate::card::sets) static RIVERPYRE_VERGE: CardRecord = CardRecord::new(
    "Riverpyre Verge",
    "57a93a71-d77c-417f-85d0-cd420f573331",
    "Titus Lunter",
    // Untapped and free either way: the red is unconditional, and the blue
    // is what the second land in the deck is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {U}. Activate only if you control an Island or a Mountain.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The same verge condition in this cycle's other pair of colours: either
                // type answers it, so a Volcanic Island is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Island,
                        BasicLandType::Mountain,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ]),
);

// DFT 261 — Rocky Roads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCKY_ROADS: CardRecord = CardRecord::new(
    "Rocky Roads",
    "6f0d4e8a-aab5-458e-bc0e-2b6b0054646a",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// DFT 262 — Rugged Highlands (reprint)
const RUGGED_HIGHLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::RUGGED_HIGHLANDS,
    "73b7484f-923c-46c5-95bf-c2c6706c0d48",
    "Florian de Gesincourt",
);

// DFT 263 — Scoured Barrens (reprint)
const SCOURED_BARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SCOURED_BARRENS,
    "ac52be0c-49f4-4956-820e-bdd7d2744d2f",
    "Eddie Mendoza",
);

// DFT 264 — Sunbillow Verge
pub(in crate::card::sets) static SUNBILLOW_VERGE: CardRecord = CardRecord::new(
    "Sunbillow Verge",
    "94ed132f-b818-4dbf-9b4a-e5acb067e0a4",
    "Pete Venters",
    // Untapped and free either way: the white is unconditional, and the red
    // is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {R}. Activate only if you control a Mountain or a Plains.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Boros colours. Either type answers
                // it, so a Plateau is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Mountain,
                        BasicLandType::Plains,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// DFT 265 — Swiftwater Cliffs (reprint)
const SWIFTWATER_CLIFFS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SWIFTWATER_CLIFFS,
    "949bf6bb-1e97-48a3-8547-0a780664c275",
    "Mark Poole",
);

// DFT 266 — Thornwood Falls (reprint)
const THORNWOOD_FALLS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::THORNWOOD_FALLS,
    "4723a303-7f20-4399-8bc6-6a27a61a3532",
    "Eddie Mendoza",
);

// DFT 267 — Tranquil Cove (reprint)
const TRANQUIL_COVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::TRANQUIL_COVE,
    "70688800-7675-4e75-bb3c-9e05b95a685c",
    "Wayne Wu",
);

// DFT 268 — Wastewood Verge
pub(in crate::card::sets) static WASTEWOOD_VERGE: CardRecord = CardRecord::new(
    "Wastewood Verge",
    "5ceacc7d-d407-4f82-af58-9bdf8426924e",
    "Bartek Fedyczak",
    // Untapped and free either way: the green is unconditional, and the
    // black is what the second land in the deck is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {B}. Activate only if you control a Swamp or a Forest.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition: any land you control with either type answers it,
                // so a Bayou is both halves at once and a land whose types were changed
                // counts for what it is now rather than what it was printed as.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Swamp,
                        BasicLandType::Forest,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// DFT 269 — Wild Roads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_ROADS: CardRecord = CardRecord::new(
    "Wild Roads",
    "4d3d48d1-a98e-40af-b04c-c40b9d52e9ee",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// DFT 270 — Willowrush Verge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILLOWRUSH_VERGE: CardRecord = CardRecord::new(
    "Willowrush Verge",
    "758d93d5-3f66-4395-a928-000485396c87",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// DFT 271 — Wind-Scarred Crag (reprint)
const WIND_SCARRED_CRAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::WIND_SCARRED_CRAG,
    "882969d8-7e3e-4f32-858a-00da20c67b83",
    "Svetlin Velinov",
);

// DFT 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "2d2da8b9-797f-46e8-a15d-a86c7c56dc84",
    "Adam Paquette",
);

// DFT 273 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "f53ed206-8ec4-46a6-8603-c819682e1433",
    "Maxime Minard",
);

// DFT 274 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "e969e168-e74c-4608-9fa0-a5660bf7fa02",
    "Jonas De Ro",
);

// DFT 275 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "50748a4c-43a0-4274-9a84-046d76027166",
    "Chris Ostrowski",
);

// DFT 276 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "777e868d-cb88-4b8e-99d1-12ff67f5eae2",
    "Andreas Rocha",
);

// DFT 277 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "464723b3-0723-45f2-a258-32d098c39039",
    "Samuele Bandini",
);

// DFT 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "4b97d2f5-498c-42db-95d9-f9002d4dfcc5",
    "Titus Lunter",
);

// DFT 279 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "a1cf2f81-3041-4349-9b37-f215c4e38c5b",
    "Leon Tukker",
);

// DFT 280 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "5a8e9b9e-4947-4b6a-b21b-6fe009760fc5",
    "Samuele Bandini",
);

// DFT 281 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "f9369d0a-87c0-4c29-b43c-cd45c007f8d6",
    "Mark Poole",
);

// DFT 282 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "7a3c4a9d-48db-474f-aeb6-8b4fa08ce6fa",
    "Leon Tukker",
);

// DFT 283 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "040d064e-b023-4750-afeb-1f58e36bc4ab",
    "Samuele Bandini",
);

// DFT 284 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "73e124b6-3af7-41c0-b829-ff85aa0c3782",
    "Ron Spencer",
);

// DFT 285 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "85288987-6f3f-4b7b-9ecc-9393dd37f115",
    "Leon Tukker",
);

// DFT 286 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "fb76cb6c-2f4d-4ca2-876f-d49ea529e59b",
    "Samuele Bandini",
);

// DFT 287 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "c872e70c-626f-4d2c-b26b-d94f815fceea",
    "Florian de Gesincourt",
);

// DFT 288 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "653e2393-30a7-4e0c-bd20-f05ca0ba3cb6",
    "Leon Tukker",
);

// DFT 289 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "b69adddd-3626-45d6-8100-26cf1f314d00",
    "Samuele Bandini",
);

// DFT 290 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "6472e2a6-287f-4e7c-ad85-1454e40d4a46",
    "Yeong-Hao Han",
);

// DFT 291 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "562b16ae-8ae7-4b2d-9098-bf3ff1429f7b",
    "Leon Tukker",
);

// DFT 292 — Air Response Unit (alternate printing)
const AIR_RESPONSE_UNIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AIR_RESPONSE_UNIT,
    1,
    "89e66d63-56be-46f5-b0df-e1350106ba45",
    "SchmandrewART",
);

// DFT 293 — Broadcast Rambler (alternate printing)
const BROADCAST_RAMBLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BROADCAST_RAMBLER,
    1,
    "b6118bb1-9530-487b-a211-7cc56cef3fbb",
    "Francisco Badilla",
);

// DFT 294 — Detention Chariot (alternate printing)
const DETENTION_CHARIOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DETENTION_CHARIOT,
    1,
    "0e45f00e-2156-4e8c-813c-0e36ab34982e",
    "Arik Roper",
);

// DFT 295 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    1,
    "37c2e68f-3f40-45b1-9afe-cc1c5fb41203",
    "Adam Volker",
);

// DFT 296 — Skyseer's Chariot (alternate printing)
const SKYSEER_S_CHARIOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYSEER_S_CHARIOT,
    1,
    "ec0eb86e-371c-46fd-a261-384b2a603b36",
    "Douglas P. Lobo",
);

// DFT 297 — Spotcycle Scouter (alternate printing)
const SPOTCYCLE_SCOUTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPOTCYCLE_SCOUTER,
    1,
    "c0303505-d6f1-4626-a932-0e577d9572fe",
    "Neo.G",
);

// DFT 298 — Valor's Flagship (alternate printing)
const VALOR_S_FLAGSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALOR_S_FLAGSHIP,
    1,
    "f142d6c1-90f6-49d6-861e-36e9a6fac958",
    "William Tempest",
);

// DFT 299 — Voyager Glidecar (alternate printing)
const VOYAGER_GLIDECAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VOYAGER_GLIDECAR,
    1,
    "e7c7da86-902a-4d40-a230-4d22d4b2df59",
    "Andrew Griffith",
);

// DFT 300 — Hulldrifter (alternate printing)
const HULLDRIFTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HULLDRIFTER,
    1,
    "0279171d-1623-438c-b796-d294c6675604",
    "Juan Marquez",
);

// DFT 301 — Midnight Mangler (alternate printing)
const MIDNIGHT_MANGLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIDNIGHT_MANGLER,
    1,
    "4aeb6fa4-569e-4ede-94c5-a8f3c28f55e4",
    "Boneface",
);

// DFT 302 — Possession Engine (alternate printing)
const POSSESSION_ENGINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POSSESSION_ENGINE,
    1,
    "821045bd-c8e4-40fb-babd-5d13b64a71ec",
    "Douglas P. Lobo",
);

// DFT 303 — Rangers' Refueler (alternate printing)
const RANGERS_REFUELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RANGERS_REFUELER,
    1,
    "530159a6-0672-485a-867c-aa0bac961765",
    "Francisco Badilla",
);

// DFT 304 — Thopter Fabricator (alternate printing)
const THOPTER_FABRICATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THOPTER_FABRICATOR,
    1,
    "1dd26a5a-ca58-4cb6-b599-7c84d601dc07",
    "William Tempest",
);

// DFT 305 — Carrion Cruiser (alternate printing)
const CARRION_CRUISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARRION_CRUISER,
    1,
    "818931d1-258d-4dda-9d0a-2015ec330bab",
    "Ian Jepson",
);

// DFT 306 — Cryptcaller Chariot (alternate printing)
const CRYPTCALLER_CHARIOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYPTCALLER_CHARIOT,
    1,
    "914f4e3c-2729-488b-883f-7657deea39c8",
    "Oliver Barrett",
);

// DFT 307 — Demonic Junker (alternate printing)
const DEMONIC_JUNKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEMONIC_JUNKER,
    1,
    "c322936f-3b56-454c-a89e-fa53faab6a33",
    "Deathburger",
);

// DFT 308 — The Last Ride (alternate printing)
const THE_LAST_RIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_RIDE,
    1,
    "f2d563b1-f03b-4ce3-9de0-05dab257b67c",
    "Deathburger",
);

// DFT 309 — Ripclaw Wrangler (alternate printing)
const RIPCLAW_WRANGLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIPCLAW_WRANGLER,
    1,
    "6ba1b096-7424-4ce9-8dfd-56a86fe612e4",
    "Andrew Griffith",
);

// DFT 310 — Boommobile (alternate printing)
const BOOMMOBILE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOOMMOBILE,
    1,
    "3501b938-7a25-43c7-b1c5-03cd9b690930",
    "Ian Jepson",
);

// DFT 311 — Burner Rocket (alternate printing)
const BURNER_ROCKET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BURNER_ROCKET,
    1,
    "267c93bd-1313-4c7b-bd6f-91e329932dea",
    "Carl Critchlow",
);

// DFT 312 — Clamorous Ironclad (alternate printing)
const CLAMOROUS_IRONCLAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLAMOROUS_IRONCLAD,
    1,
    "79d3c041-976c-41e2-9ce6-4b292e499526",
    "Cosmin Podar",
);

// DFT 313 — Gastal Thrillroller (alternate printing)
const GASTAL_THRILLROLLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_THRILLROLLER,
    1,
    "03a07256-3305-4178-bd4f-d1583d23106d",
    "Dan Mumford",
);

// DFT 314 — Spire Mechcycle (alternate printing)
const SPIRE_MECHCYCLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIRE_MECHCYCLE,
    1,
    "5f348c0a-41f3-4d88-be1d-19c0821f158e",
    "Neo.G",
);

// DFT 315 — Earthrumbler (alternate printing)
const EARTHRUMBLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTHRUMBLER,
    1,
    "0d95a5f3-7f14-4d59-8329-559ed4ff0491",
    "SchmandrewART",
);

// DFT 316 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    1,
    "11c196dd-61a9-4e75-bd43-9531ba3dc961",
    "Francisco Badilla",
);

// DFT 317 — Thunderous Velocipede (alternate printing)
const THUNDEROUS_VELOCIPEDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_VELOCIPEDE,
    1,
    "3e5f6a1c-7b34-4c23-8172-5b869c96eb65",
    "Eduardo Francisco",
);

// DFT 318 — Veloheart Bike (alternate printing)
const VELOHEART_BIKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VELOHEART_BIKE,
    1,
    "cfdffb27-5035-49c2-b214-abe46a17433d",
    "Juan Marquez",
);

// DFT 319 — Apocalypse Runner (alternate printing)
const APOCALYPSE_RUNNER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &APOCALYPSE_RUNNER,
    1,
    "ea55a6ec-634c-4590-9875-db4e5b81795d",
    "Arik Roper",
);

// DFT 320 — Boosted Sloop (alternate printing)
const BOOSTED_SLOOP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOOSTED_SLOOP,
    1,
    "85bb2217-d3e1-4638-9a14-bb72bfedd18c",
    "Gabriel Rubio",
);

// DFT 321 — Cloudspire Skycycle (alternate printing)
const CLOUDSPIRE_SKYCYCLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOUDSPIRE_SKYCYCLE,
    1,
    "73918759-b7b6-4a09-baa1-7f660ecef42e",
    "Yuko Shimizu",
);

// DFT 322 — Debris Beetle (alternate printing)
const DEBRIS_BEETLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEBRIS_BEETLE,
    1,
    "b8fe7293-2d85-42dd-b6a1-21e98487cbfa",
    "Michal Ivan",
);

// DFT 323 — Dune Drifter (alternate printing)
const DUNE_DRIFTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DUNE_DRIFTER,
    1,
    "71586675-e9f5-4bec-984d-8683b2494b35",
    "Arik Roper",
);

// DFT 324 — Guidelight Pathmaker (alternate printing)
const GUIDELIGHT_PATHMAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUIDELIGHT_PATHMAKER,
    1,
    "83566cd0-2253-477e-ae99-4c0c4902ad6e",
    "Dan Mumford",
);

// DFT 325 — Haunted Hellride (alternate printing)
const HAUNTED_HELLRIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HAUNTED_HELLRIDE,
    1,
    "9d68c20a-f377-4613-8771-cdda1745cdff",
    "Deathburger",
);

// DFT 326 — Rangers' Aetherhive (alternate printing)
const RANGERS_AETHERHIVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RANGERS_AETHERHIVE,
    1,
    "3f0d5c5c-d3d6-48ae-9775-fcd2cd9a0c65",
    "Francisco Badilla",
);

// DFT 327 — Rocketeer Boostbuggy (alternate printing)
const ROCKETEER_BOOSTBUGGY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROCKETEER_BOOSTBUGGY,
    1,
    "2db27c67-0462-474c-844b-4f69d8fe6734",
    "Adam Volker",
);

// DFT 328 — Thundering Broodwagon (alternate printing)
const THUNDERING_BROODWAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDERING_BROODWAGON,
    1,
    "b4adfba1-c73a-43b4-8400-e11617958f7f",
    "Villarrte",
);

// DFT 329 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    1,
    "1f0aa565-1ef0-41f5-a86d-0701e357cde9",
    "Eduardo Francisco",
);

// DFT 330 — Marshals' Pathcruiser (alternate printing)
const MARSHALS_PATHCRUISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARSHALS_PATHCRUISER,
    1,
    "5c4e88ae-7f5c-4dea-add0-a53a6a9bd8e4",
    "Michal Ivan",
);

// DFT 331 — Rover Blades (alternate printing)
const ROVER_BLADES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROVER_BLADES,
    1,
    "5a3b07b7-8d51-428e-a399-61da45529c8f",
    "Eduardo Francisco",
);

// DFT 332 — Skybox Ferry (alternate printing)
const SKYBOX_FERRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYBOX_FERRY,
    1,
    "cd9e294e-cdb2-423d-a034-f68f673b5973",
    "SchmandrewART",
);

// DFT 333 — Bulwark Ox (alternate printing)
const BULWARK_OX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BULWARK_OX,
    1,
    "27736f40-a87b-45e4-a0da-accf9433c1fc",
    "Jon Vermilyea",
);

// DFT 334 — Guardian Sunmare (alternate printing)
const GUARDIAN_SUNMARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUARDIAN_SUNMARE,
    1,
    "309072be-b813-42bd-adaf-7d45053029c0",
    "Wojtek Łebski",
);

// DFT 335 — Mindspring Merfolk (alternate printing)
const MINDSPRING_MERFOLK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MINDSPRING_MERFOLK,
    1,
    "bcda94fc-43f0-4fa6-b38b-bb8c4c26d9a1",
    "CatDirty",
);

// DFT 336 — Waxen Shapethief (alternate printing)
const WAXEN_SHAPETHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WAXEN_SHAPETHIEF,
    1,
    "7fc820fb-eec2-4873-a272-68e8379470fc",
    "CatDirty",
);

// DFT 337 — Bloodghast (alternate printing)
const BLOODGHAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::BLOODGHAST,
    1,
    "4560bd1f-eb63-45bc-a8c7-b5de200facc9",
    "Wojtek Łebski",
);

// DFT 338 — Gas Guzzler (alternate printing)
const GAS_GUZZLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GAS_GUZZLER,
    1,
    "3150199e-9e4f-486d-bfb0-fc2a9feb8536",
    "Sam McKenzie",
);

// DFT 339 — The Speed Demon (alternate printing)
const THE_SPEED_DEMON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SPEED_DEMON,
    1,
    "6c4045e7-6d2c-4ef9-922e-3681fa820c4b",
    "CatDirty",
);

// DFT 340 — Burnout Bashtronaut (alternate printing)
const BURNOUT_BASHTRONAUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BURNOUT_BASHTRONAUT,
    1,
    "eb1ffa8e-2794-4f11-8351-42f1a0f26831",
    "Jon Vermilyea",
);

// DFT 341 — Draconautics Engineer (alternate printing)
const DRACONAUTICS_ENGINEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRACONAUTICS_ENGINEER,
    1,
    "4712279a-ae45-488c-870a-0187201c860f",
    "Ryan Roadkill",
);

// DFT 342 — Howlsquad Heavy (alternate printing)
const HOWLSQUAD_HEAVY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOWLSQUAD_HEAVY,
    1,
    "6a758d38-adfe-4894-bf27-d988a228e944",
    "Ryan Roadkill",
);

// DFT 343 — Agonasaur Rex (alternate printing)
const AGONASAUR_REX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGONASAUR_REX,
    1,
    "328d60a4-af43-4a2e-98b5-1098478eeb02",
    "Jon Vermilyea",
);

// DFT 344 — District Mascot (alternate printing)
const DISTRICT_MASCOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DISTRICT_MASCOT,
    1,
    "15f978b9-d464-42f7-ba8b-0c0795d54cb5",
    "Ryan Roadkill",
);

// DFT 345 — Webstrike Elite (alternate printing)
const WEBSTRIKE_ELITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEBSTRIKE_ELITE,
    1,
    "14e8e016-2b68-43d1-b25a-553bad97e431",
    "Sam McKenzie",
);

// DFT 346 — Fearless Swashbuckler (alternate printing)
const FEARLESS_SWASHBUCKLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEARLESS_SWASHBUCKLER,
    1,
    "2e8aff7b-bcf4-46a2-a8f5-dc00394a84da",
    "CatDirty",
);

// DFT 347 — Hazoret, Godseeker (alternate printing)
const HAZORET_GODSEEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HAZORET_GODSEEKER,
    1,
    "e5e6cbde-ce5b-4c25-9a27-715fe7abe4e4",
    "Massiveface",
);

// DFT 348 — Brightglass Gearhulk (alternate printing)
const BRIGHTGLASS_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRIGHTGLASS_GEARHULK,
    1,
    "82000701-d4a8-4f9f-a4b0-236858c84954",
    "Jorge Gutierrez Garcia",
);

// DFT 349 — Coalstoke Gearhulk (alternate printing)
const COALSTOKE_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COALSTOKE_GEARHULK,
    1,
    "b4574869-9598-4295-bd43-ce3633e0d8e6",
    "Ivan Shavrin",
);

// DFT 350 — Ketramose, the New Dawn (alternate printing)
const KETRAMOSE_THE_NEW_DAWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KETRAMOSE_THE_NEW_DAWN,
    1,
    "3c8a81bf-f63c-40ca-a4da-db68df3f3e1a",
    "Florian Bertmer",
);

// DFT 351 — Oildeep Gearhulk (alternate printing)
const OILDEEP_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OILDEEP_GEARHULK,
    1,
    "099ead92-1d35-495c-8ee8-14d73e9f1437",
    "Chun Lo",
);

// DFT 352 — Pyrewood Gearhulk (alternate printing)
const PYREWOOD_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PYREWOOD_GEARHULK,
    1,
    "e0aa0bc1-3c31-4886-bfd2-f1ab6e048559",
    "William Tempest",
);

// DFT 353 — Riptide Gearhulk (alternate printing)
const RIPTIDE_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIPTIDE_GEARHULK,
    1,
    "b6cdbb98-a5b8-4b9e-aaaa-457eb9d4041e",
    "Kudaman",
);

// DFT 354 — Sab-Sunen, Luxa Embodied (alternate printing)
const SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAB_SUNEN_LUXA_EMBODIED,
    1,
    "43655ecc-af27-494f-bee7-a5542cae9b54",
    "Benjamin Ee",
);

// DFT 355 — Basri, Tomorrow's Champion (alternate printing)
const BASRI_TOMORROW_S_CHAMPION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BASRI_TOMORROW_S_CHAMPION,
    1,
    "440766a6-5820-4248-ad91-4d946235e136",
    "Justyna Dura",
);

// DFT 356 — Vnwxt, Verbose Host (alternate printing)
const VNWXT_VERBOSE_HOST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VNWXT_VERBOSE_HOST,
    1,
    "a7d0d849-d400-43fc-a42a-0eeb27131d1b",
    "Chris Seaman",
);

// DFT 357 — Gonti, Night Minister (alternate printing)
const GONTI_NIGHT_MINISTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GONTI_NIGHT_MINISTER,
    1,
    "38a867af-cf5d-4e3d-bba1-361e3cfeca82",
    "Richard Kane Ferguson",
);

// DFT 358 — Daretti, Rocketeer Engineer (alternate printing)
const DARETTI_ROCKETEER_ENGINEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DARETTI_ROCKETEER_ENGINEER,
    1,
    "3993ac84-304a-4e42-aa39-b72be30471ea",
    "Samuel Perin",
);

// DFT 359 — Oviya, Automech Artisan (alternate printing)
const OVIYA_AUTOMECH_ARTISAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVIYA_AUTOMECH_ARTISAN,
    1,
    "c9b47651-5607-40b0-9e73-ad356b10acc9",
    "Ron Spears",
);

// DFT 360 — Aatchik, Emerald Radian (alternate printing)
const AATCHIK_EMERALD_RADIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AATCHIK_EMERALD_RADIAN,
    1,
    "e789df76-d658-47a4-9efb-74da6bd8821c",
    "Erica Williams",
);

// DFT 361 — Captain Howler, Sea Scourge (alternate printing)
const CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_HOWLER_SEA_SCOURGE,
    1,
    "a8f0f937-5519-4d38-9fc3-46460669bb86",
    "Mark Zug",
);

// DFT 362 — Caradora, Heart of Alacria (alternate printing)
const CARADORA_HEART_OF_ALACRIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARADORA_HEART_OF_ALACRIA,
    1,
    "a7bca478-48c0-4c39-95ae-ba8afb207153",
    "Yuko Shimizu",
);

// DFT 363 — Far Fortune, End Boss (alternate printing)
const FAR_FORTUNE_END_BOSS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAR_FORTUNE_END_BOSS,
    1,
    "ae0bd239-25ae-496e-9be2-c2d968dc9346",
    "Justine Jones",
);

// DFT 364 — Kolodin, Triumph Caster (alternate printing)
const KOLODIN_TRIUMPH_CASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KOLODIN_TRIUMPH_CASTER,
    1,
    "33887384-a2bb-4daf-b932-a4259c93b808",
    "John Stanko",
);

// DFT 365 — Mendicant Core, Guidelight (alternate printing)
const MENDICANT_CORE_GUIDELIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MENDICANT_CORE_GUIDELIGHT,
    1,
    "6f11d7a7-72e9-406e-a701-5c3a556a43e0",
    "Dan Mumford",
);

// DFT 366 — Redshift, Rocketeer Chief (alternate printing)
const REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REDSHIFT_ROCKETEER_CHIEF,
    1,
    "c56bc66f-12d1-4eae-8de7-2fae34976d6b",
    "Xavier Ribeiro",
);

// DFT 367 — Samut, the Driving Force (alternate printing)
const SAMUT_THE_DRIVING_FORCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAMUT_THE_DRIVING_FORCE,
    1,
    "227036a1-7570-4657-9c0a-635dd6d889e9",
    "Mark Poole",
);

// DFT 368 — Sita Varma, Masked Racer (alternate printing)
const SITA_VARMA_MASKED_RACER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SITA_VARMA_MASKED_RACER,
    1,
    "119a39ce-80f6-48b3-b159-e5d919d3b617",
    "rk post",
);

// DFT 369 — Winter, Cursed Rider (alternate printing)
const WINTER_CURSED_RIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINTER_CURSED_RIDER,
    1,
    "61aa759b-a940-4071-9373-13e6aef62fe4",
    "Ovidio Cartagena",
);

// DFT 370 — Zahur, Glory's Past (alternate printing)
const ZAHUR_GLORY_S_PAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZAHUR_GLORY_S_PAST,
    1,
    "e1004f64-5052-4b1e-8fd8-eda463fbd9b7",
    "Alex Stone",
);

// DFT 371 — Bleachbone Verge (alternate printing)
const BLEACHBONE_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLEACHBONE_VERGE,
    1,
    "6ba2edc7-dbc0-4948-9f37-a27a91bdf7b9",
    "Daren Bader",
);

// DFT 372 — Riverpyre Verge (alternate printing)
const RIVERPYRE_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIVERPYRE_VERGE,
    1,
    "70efc6f3-2fdc-4629-84b9-f3707b799460",
    "Justin Sweet",
);

// DFT 373 — Sunbillow Verge (alternate printing)
const SUNBILLOW_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNBILLOW_VERGE,
    1,
    "c92a7ca5-a25b-4888-a319-6e176ab4d0ce",
    "Darrell Riche",
);

// DFT 374 — Wastewood Verge (alternate printing)
const WASTEWOOD_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WASTEWOOD_VERGE,
    1,
    "78ab2fc7-7945-4b49-8ad0-223815d0d2c2",
    "Kev Walker",
);

// DFT 375 — Willowrush Verge (alternate printing)
const WILLOWRUSH_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILLOWRUSH_VERGE,
    1,
    "38218d0f-4b70-45eb-a753-cdb44f94271c",
    "Carl Critchlow",
);

// DFT 376 — The Aetherspark (alternate printing)
const THE_AETHERSPARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_AETHERSPARK,
    1,
    "934cc1ca-2ce8-4062-b1b7-1976e76da8b6",
    "Dominik Mayer",
);

// DFT 377 — Perilous Snare (alternate printing)
const PERILOUS_SNARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PERILOUS_SNARE,
    1,
    "d1edff87-8563-4cb5-ab9b-7696e920346a",
    "Chris Seaman",
);

// DFT 378 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    1,
    "b513dfeb-d7c8-4e4d-b419-f67b45608b4b",
    "Zezhou Chen",
);

// DFT 379 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    1,
    "c4797dcb-507f-4219-95ff-994e2c0d251b",
    "Justyna Dura",
);

// DFT 380 — Repurposing Bay (alternate printing)
const REPURPOSING_BAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REPURPOSING_BAY,
    1,
    "231c4f52-5a12-407f-9917-d2ca91ba4706",
    "William Tempest",
);

// DFT 381 — Riverchurn Monument (alternate printing)
const RIVERCHURN_MONUMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIVERCHURN_MONUMENT,
    1,
    "131df43b-2a72-4ca3-8aae-c0dca2eca8fc",
    "Anthony Devine",
);

// DFT 382 — Unstoppable Plan (alternate printing)
const UNSTOPPABLE_PLAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNSTOPPABLE_PLAN,
    1,
    "ce40ff0e-04ae-4786-ac0c-54a66ffb48a6",
    "Borja Pindado",
);

// DFT 383 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    1,
    "c0a57479-6409-49e1-8896-2bd3e88d554d",
    "Dominik Mayer",
);

// DFT 384 — Quag Feast (alternate printing)
const QUAG_FEAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUAG_FEAST,
    1,
    "5d6498f0-5ebd-441a-a851-461b0460e7d5",
    "Loïc Canavaggia",
);

// DFT 385 — Count on Luck (alternate printing)
const COUNT_ON_LUCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COUNT_ON_LUCK,
    1,
    "5d2abab5-c4f0-41ec-ae6f-e42721ca2cfd",
    "Michal Ivan",
);

// DFT 386 — Full Throttle (alternate printing)
const FULL_THROTTLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FULL_THROTTLE,
    1,
    "e7967282-ae00-4fdd-85c6-bcc8bc9b790b",
    "Benjamin Ee",
);

// DFT 387 — Afterburner Expert (alternate printing)
const AFTERBURNER_EXPERT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AFTERBURNER_EXPERT,
    1,
    "aafe531a-f1f9-46d4-aff4-2b0df789be66",
    "April Prime",
);

// DFT 388 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    1,
    "615e3ebb-c41a-497a-b71b-6549366e5b07",
    "Helge C. Balzer",
);

// DFT 389 — Regal Imperiosaur (alternate printing)
const REGAL_IMPERIOSAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REGAL_IMPERIOSAUR,
    1,
    "3a198715-4f82-409d-add0-e0200a88d708",
    "Stephanie Cheung",
);

// DFT 390 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    1,
    "a258dcf4-b481-4ad5-96ef-2a42de8c9c2a",
    "Caio Monteiro",
);

// DFT 391 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    1,
    "2528d058-5ca3-4671-8bf1-c8161872cd36",
    "Ernanda Souza",
);

// DFT 392 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    1,
    "115088f3-9fca-4120-8ca5-c124c3d32975",
    "Ron Spencer",
);

// DFT 393 — Marketback Walker (alternate printing)
const MARKETBACK_WALKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARKETBACK_WALKER,
    1,
    "93211140-5aa7-4700-804d-aa46a8b141d7",
    "Svetlin Velinov",
);

// DFT 394 — Monument to Endurance (alternate printing)
const MONUMENT_TO_ENDURANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MONUMENT_TO_ENDURANCE,
    1,
    "c66cbdac-1d15-40f5-b76f-6f21b917abbc",
    "Victor Sales",
);

// DFT 395 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    1,
    "8ef84601-6ace-4847-abeb-16dcfdba5c89",
    "Bruce Brenneise",
);

// DFT 396 — Muraganda Raceway (alternate printing)
const MURAGANDA_RACEWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MURAGANDA_RACEWAY,
    1,
    "55b87e12-bd95-450e-a528-a44c76dbfae5",
    "Brian Valeza",
);

// DFT 397 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    2,
    "d56cb1b9-38b7-42ad-b176-dc265bc8abe0",
    "Mai Minamiura",
);

// DFT 398 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    2,
    "ec93ed6d-a840-477d-a16c-ffd15a52c941",
    "JIN-E YAMAMOTO",
);

// DFT 399 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    2,
    "7fe19f52-ae23-4a23-800d-f3cf186279f2",
    "D-suzuki",
);

// DFT 400 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    2,
    "c5280941-bcda-4ac7-97d1-db02c7a18ef9",
    "BARON UEDA",
);

// DFT 401 — Chandra, Spark Hunter (alternate printing)
const CHANDRA_SPARK_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_SPARK_HUNTER,
    1,
    "a1864b99-6893-4e4d-94de-641837c05da3",
    "jbstyle.",
);

// DFT 402 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    2,
    "455027f1-26a4-464b-a456-271a6ec88669",
    "Raimaru",
);

// DFT 403 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    2,
    "14a81f86-e80f-47f7-81cd-0c4b9330e646",
    "TSCR",
);

// DFT 404 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    2,
    "4f5b04ab-4c61-47f5-88e4-c5c2f93edc11",
    "Tetsu Kurosawa",
);

// DFT 405 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    2,
    "d5a8caa7-714e-4987-8d44-04fbe5a77c6d",
    "SH11NA",
);

// DFT 406 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    2,
    "8eb85966-66a2-4fb7-b89d-05db7f4e2cb8",
    "Tomoyuki Mizufune",
);

// DFT 407 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    3,
    "0e73d4ed-def7-4456-a750-74fb2bf10f9f",
    "Mai Minamiura",
);

// DFT 408 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    3,
    "22fd4f1b-afbd-41a5-a76a-f9788e0cec9d",
    "JIN-E YAMAMOTO",
);

// DFT 409 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    3,
    "8327b4b6-058e-448b-99ca-7d2de52d3b4c",
    "D-suzuki",
);

// DFT 410 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    3,
    "3c15a04e-b0a7-4560-a838-94213dbb2336",
    "BARON UEDA",
);

// DFT 411 — Chandra, Spark Hunter (alternate printing)
const CHANDRA_SPARK_HUNTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_SPARK_HUNTER,
    2,
    "d6a40fa6-4378-413b-bed8-7eb0d61b70ad",
    "jbstyle.",
);

// DFT 412 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    3,
    "1b9fbbfc-8920-46cd-95cd-2372feb4617a",
    "Raimaru",
);

// DFT 413 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    3,
    "b3dd66b9-f292-4b10-b753-2df112f6f714",
    "TSCR",
);

// DFT 414 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    3,
    "89fba04a-20ab-40eb-9edf-89fdd03bfb7e",
    "Tetsu Kurosawa",
);

// DFT 415 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    3,
    "6b243165-79ad-4193-af89-2ad5e34fc415",
    "SH11NA",
);

// DFT 416 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    3,
    "656ba742-b00f-4fce-8418-987226c25a81",
    "Tomoyuki Mizufune",
);

// DFT 417 — Tune Up (alternate printing)
const TUNE_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TUNE_UP,
    1,
    "6b832719-ac31-470f-9d50-0b514ecc6845",
    "Chris Rallis",
);

// DFT 418 — Gastal Raider (alternate printing)
const GASTAL_RAIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_RAIDER,
    1,
    "4a8b12dd-2ce3-48a6-9ea7-7d5f155164c9",
    "Lorenzo Mastroianni",
);

// DFT 419 — Marauding Mako (alternate printing)
const MARAUDING_MAKO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARAUDING_MAKO,
    1,
    "194ebb23-fecd-4aa5-96b7-447c9768794e",
    "Alix Branwyn",
);

// DFT 420 — Skyserpent Seeker (alternate printing)
const SKYSERPENT_SEEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYSERPENT_SEEKER,
    1,
    "b7de8c7e-7270-4a4f-af03-9f088b04c05b",
    "Johan Grenier",
);

// DFT 421 — Voyage Home (alternate printing)
const VOYAGE_HOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VOYAGE_HOME,
    1,
    "30789c9b-7b4b-43e7-a161-aa6595ea9dee",
    "Hardy Fowler",
);

// DFT 422 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    2,
    "06a85032-0c45-40f7-901a-3fd77ff212a4",
    "Adam Volker",
);

// DFT 423 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    2,
    "23eda600-af1e-4444-a822-830037133c0d",
    "José Parodi",
);

// DFT 424 — Amonkhet Raceway (alternate printing)
const AMONKHET_RACEWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AMONKHET_RACEWAY,
    1,
    "fb37d984-5a9c-45e3-8213-4af93872d512",
    "Titus Lunter",
);

// DFT 425 — Avishkar Raceway (alternate printing)
const AVISHKAR_RACEWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVISHKAR_RACEWAY,
    1,
    "21e9b236-f0e5-4e9d-a9c9-7d625e8412f2",
    "Titus Lunter",
);

// DFT 426 — Muraganda Raceway (alternate printing)
const MURAGANDA_RACEWAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MURAGANDA_RACEWAY,
    2,
    "9db48a2f-5110-431e-8d72-b70bd2e9d887",
    "Titus Lunter",
);

// DFT 427 — Basri, Tomorrow's Champion (alternate printing)
const BASRI_TOMORROW_S_CHAMPION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BASRI_TOMORROW_S_CHAMPION,
    2,
    "932d4e0c-2653-4353-8e70-b7cf0f6808ed",
    "Kai Carpenter",
);

// DFT 428 — Bulwark Ox (alternate printing)
const BULWARK_OX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BULWARK_OX,
    2,
    "146670b1-0b6d-4d9b-b931-137c508309a9",
    "Brent Hollowell",
);

// DFT 429 — Guardian Sunmare (alternate printing)
const GUARDIAN_SUNMARE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GUARDIAN_SUNMARE,
    2,
    "9a6b0d3e-0d3e-44a7-99e3-4ccba6c832e1",
    "Christina Kraus",
);

// DFT 430 — Perilous Snare (alternate printing)
const PERILOUS_SNARE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PERILOUS_SNARE,
    2,
    "87181867-f63d-4642-875f-001c1ed7912f",
    "Chris Seaman",
);

// DFT 431 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    4,
    "7f338af2-f173-45a0-bbea-02517d41bef2",
    "Ben Wootten",
);

// DFT 432 — Skyseer's Chariot (alternate printing)
const SKYSEER_S_CHARIOT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SKYSEER_S_CHARIOT,
    2,
    "3a062cbd-b769-49d3-a9fd-7aeb414f44d6",
    "Carl Critchlow",
);

// DFT 433 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    4,
    "9d21406e-0c00-4501-a93e-ccd8172314e8",
    "Zezhou Chen",
);

// DFT 434 — Valor's Flagship (alternate printing)
const VALOR_S_FLAGSHIP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VALOR_S_FLAGSHIP,
    2,
    "1f73b484-1503-4899-8b6a-076e8b087810",
    "Stephan Martiniere",
);

// DFT 435 — Voyager Glidecar (alternate printing)
const VOYAGER_GLIDECAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VOYAGER_GLIDECAR,
    2,
    "559dd83b-f24a-43e7-9195-9fcbe6c0c64b",
    "Eduardo Francisco",
);

// DFT 436 — Mindspring Merfolk (alternate printing)
const MINDSPRING_MERFOLK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MINDSPRING_MERFOLK,
    2,
    "09e15402-8cc6-4ad2-8088-4df92fa20d8e",
    "Andreia Ugrai",
);

// DFT 437 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    4,
    "40d45912-7b8d-416e-893b-1d103492a5a1",
    "Justyna Dura",
);

// DFT 438 — Possession Engine (alternate printing)
const POSSESSION_ENGINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &POSSESSION_ENGINE,
    2,
    "858dc258-965a-4d1a-aab5-447e0fecbf81",
    "Leroy Steinmann",
);

// DFT 439 — Repurposing Bay (alternate printing)
const REPURPOSING_BAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &REPURPOSING_BAY,
    2,
    "5277eeef-50af-4654-9216-caf0c37c27d7",
    "William Tempest",
);

// DFT 440 — Riverchurn Monument (alternate printing)
const RIVERCHURN_MONUMENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RIVERCHURN_MONUMENT,
    2,
    "06d22866-07ac-4aef-b43e-29a37f3db1c0",
    "Anthony Devine",
);

// DFT 441 — Thopter Fabricator (alternate printing)
const THOPTER_FABRICATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THOPTER_FABRICATOR,
    2,
    "a98ff182-4a84-4d69-83ce-57a7e5b18ebb",
    "Racrufi",
);

// DFT 442 — Unstoppable Plan (alternate printing)
const UNSTOPPABLE_PLAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &UNSTOPPABLE_PLAN,
    2,
    "62a95aeb-b1b0-42cd-ad67-fd13717d3c6c",
    "Borja Pindado",
);

// DFT 443 — Vnwxt, Verbose Host (alternate printing)
const VNWXT_VERBOSE_HOST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VNWXT_VERBOSE_HOST,
    2,
    "7642f9db-3eee-471d-ada6-60e990ec3fab",
    "Izzy",
);

// DFT 444 — Waxen Shapethief (alternate printing)
const WAXEN_SHAPETHIEF_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WAXEN_SHAPETHIEF,
    2,
    "cb17c37c-907d-4416-b8c1-d6a6f4e312f3",
    "Helge C. Balzer",
);

// DFT 445 — Bloodghast (alternate printing)
const BLOODGHAST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::BLOODGHAST,
    2,
    "77534750-fa92-42b4-8f32-be437eca64ad",
    "Francisco Badilla",
);

// DFT 446 — Cryptcaller Chariot (alternate printing)
const CRYPTCALLER_CHARIOT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CRYPTCALLER_CHARIOT,
    2,
    "de65f4cd-3ba4-4e1a-8810-e43cf26b38a4",
    "Aaron Miller",
);

// DFT 447 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    4,
    "2b283c18-abe7-4b73-8532-dd76a85fabd6",
    "Dominik Mayer",
);

// DFT 448 — Demonic Junker (alternate printing)
const DEMONIC_JUNKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEMONIC_JUNKER,
    2,
    "ad0eb3de-796d-4da3-aab3-08c2b5b6943d",
    "Stephan Martiniere",
);

// DFT 449 — Gas Guzzler (alternate printing)
const GAS_GUZZLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GAS_GUZZLER,
    2,
    "e7e8e6d0-d155-49da-ac5b-47fb408202e6",
    "Yohann Schepacz",
);

// DFT 450 — Gonti, Night Minister (alternate printing)
const GONTI_NIGHT_MINISTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GONTI_NIGHT_MINISTER,
    2,
    "21ed5dd3-e67b-4425-bb98-6b07db1e9d1f",
    "Scott M. Fischer",
);

// DFT 451 — The Last Ride (alternate printing)
const THE_LAST_RIDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_RIDE,
    2,
    "a901a616-b8bb-4e10-9886-ee9c6fdc6435",
    "Michele Giorgi",
);

// DFT 452 — Quag Feast (alternate printing)
const QUAG_FEAST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &QUAG_FEAST,
    2,
    "8684d080-070c-4419-9cc8-777500573f55",
    "Loïc Canavaggia",
);

// DFT 453 — The Speed Demon (alternate printing)
const THE_SPEED_DEMON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_SPEED_DEMON,
    2,
    "da8a7c1f-9d05-4a4c-bdf2-cd9ee7ed18a5",
    "Helge C. Balzer",
);

// DFT 454 — Boommobile (alternate printing)
const BOOMMOBILE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BOOMMOBILE,
    2,
    "3b1bff20-0b7a-49f9-b967-773f3043be13",
    "Alexandr Leskinen",
);

// DFT 455 — Burnout Bashtronaut (alternate printing)
const BURNOUT_BASHTRONAUT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BURNOUT_BASHTRONAUT,
    2,
    "60c9b613-1494-4df0-9a38-cc0a69fd3945",
    "Andrea Piparo",
);

// DFT 456 — Chandra, Spark Hunter (alternate printing)
const CHANDRA_SPARK_HUNTER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_SPARK_HUNTER,
    3,
    "625b175a-b63f-46d1-a7e1-95dfc7c03e3f",
    "Devin Elle Kurtz",
);

// DFT 457 — Count on Luck (alternate printing)
const COUNT_ON_LUCK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COUNT_ON_LUCK,
    2,
    "eee38fa7-d5c8-4f76-92f8-4c5cb5bae5e7",
    "Michal Ivan",
);

// DFT 458 — Daretti, Rocketeer Engineer (alternate printing)
const DARETTI_ROCKETEER_ENGINEER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DARETTI_ROCKETEER_ENGINEER,
    2,
    "0a17ef59-177c-4c39-8cbb-efa8ab7ac24a",
    "Borja Pindado",
);

// DFT 459 — Draconautics Engineer (alternate printing)
const DRACONAUTICS_ENGINEER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DRACONAUTICS_ENGINEER,
    2,
    "7758b89f-9629-45f9-ad37-6435dba37997",
    "Artur Nakhodkin",
);

// DFT 460 — Full Throttle (alternate printing)
const FULL_THROTTLE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FULL_THROTTLE,
    2,
    "76bf1b29-5a8a-4ac1-aa17-58fcbda969a2",
    "Benjamin Ee",
);

// DFT 461 — Gastal Thrillroller (alternate printing)
const GASTAL_THRILLROLLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_THRILLROLLER,
    2,
    "5a80bb9d-da0d-4f98-8946-43513db4aef6",
    "Caio Monteiro",
);

// DFT 462 — Hazoret, Godseeker (alternate printing)
const HAZORET_GODSEEKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HAZORET_GODSEEKER,
    2,
    "05bbbcb2-822f-4729-872b-20638e7ad155",
    "Chris Rallis",
);

// DFT 463 — Howlsquad Heavy (alternate printing)
const HOWLSQUAD_HEAVY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HOWLSQUAD_HEAVY,
    2,
    "a28b514a-29a1-4d85-8eaa-279f3d7e09c1",
    "Leonardo Santanna",
);

// DFT 464 — Afterburner Expert (alternate printing)
const AFTERBURNER_EXPERT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AFTERBURNER_EXPERT,
    2,
    "3985ee93-3aeb-4f0d-a6af-686efbcadda4",
    "April Prime",
);

// DFT 465 — Agonasaur Rex (alternate printing)
const AGONASAUR_REX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AGONASAUR_REX,
    2,
    "54fc56fc-8c82-4d74-8119-5d260c691b36",
    "Lucas Graciano",
);

// DFT 466 — District Mascot (alternate printing)
const DISTRICT_MASCOT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DISTRICT_MASCOT,
    2,
    "3e7926c8-1a46-4edb-950b-2c8bac91b476",
    "Liiga Smilshkalne",
);

// DFT 467 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    3,
    "8bb97f9f-e7be-4794-a3a0-a0d73445c8fe",
    "Raph Lomotan",
);

// DFT 468 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    4,
    "f89e403b-9653-493f-a3ac-e01aa6f006e2",
    "Helge C. Balzer",
);

// DFT 469 — Oviya, Automech Artisan (alternate printing)
const OVIYA_AUTOMECH_ARTISAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVIYA_AUTOMECH_ARTISAN,
    2,
    "6d6eae8b-73de-4d79-bbe5-2921db1987b2",
    "Julia Metzger",
);

// DFT 470 — Regal Imperiosaur (alternate printing)
const REGAL_IMPERIOSAUR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &REGAL_IMPERIOSAUR,
    2,
    "d72328b4-19d1-4855-aa0d-331321eb9945",
    "Stephanie Cheung",
);

// DFT 471 — Thunderous Velocipede (alternate printing)
const THUNDEROUS_VELOCIPEDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_VELOCIPEDE,
    2,
    "f63dc8cd-34a7-43ea-a422-3ced3c11f5be",
    "Adrián Rodríguez Pérez",
);

// DFT 472 — Webstrike Elite (alternate printing)
const WEBSTRIKE_ELITE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WEBSTRIKE_ELITE,
    2,
    "10c14636-3ff0-4fbf-9df1-1b6d3d52e29e",
    "Andrew Mar",
);

// DFT 473 — Aatchik, Emerald Radian (alternate printing)
const AATCHIK_EMERALD_RADIAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AATCHIK_EMERALD_RADIAN,
    2,
    "71074080-d83d-4b4a-b385-f918186530b4",
    "Loïc Canavaggia",
);

// DFT 474 — Brightglass Gearhulk (alternate printing)
const BRIGHTGLASS_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BRIGHTGLASS_GEARHULK,
    2,
    "c0e29a88-34b6-4c3d-9956-b7eb8d5ef591",
    "José Parodi",
);

// DFT 475 — Captain Howler, Sea Scourge (alternate printing)
const CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_HOWLER_SEA_SCOURGE,
    2,
    "db48629b-dc1b-4609-aea2-e4833f66d894",
    "Mirko Failoni",
);

// DFT 476 — Caradora, Heart of Alacria (alternate printing)
const CARADORA_HEART_OF_ALACRIA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CARADORA_HEART_OF_ALACRIA,
    2,
    "61bc2ca6-427f-4f07-88b0-cf57a1fe74ab",
    "Mirko Failoni",
);

// DFT 477 — Coalstoke Gearhulk (alternate printing)
const COALSTOKE_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COALSTOKE_GEARHULK,
    2,
    "07cf7eba-b917-4f14-8bf4-420bb919b7e5",
    "Nino Vecia",
);

// DFT 478 — Debris Beetle (alternate printing)
const DEBRIS_BEETLE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEBRIS_BEETLE,
    2,
    "92766c53-703d-41a2-ad13-8c366dcf25ec",
    "Julie Dillon",
);

// DFT 479 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    4,
    "a556367e-7db7-47ad-9f10-37d277d80e77",
    "Caio Monteiro",
);

// DFT 480 — Far Fortune, End Boss (alternate printing)
const FAR_FORTUNE_END_BOSS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FAR_FORTUNE_END_BOSS,
    2,
    "3655e0a5-af3e-4720-b4cf-c93ae9c10cac",
    "Javier Charro",
);

// DFT 481 — Fearless Swashbuckler (alternate printing)
const FEARLESS_SWASHBUCKLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FEARLESS_SWASHBUCKLER,
    2,
    "e3bb615c-e142-4a6e-8d7a-f0a7a3b30568",
    "Konstantin Porubov",
);

// DFT 482 — Ketramose, the New Dawn (alternate printing)
const KETRAMOSE_THE_NEW_DAWN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KETRAMOSE_THE_NEW_DAWN,
    2,
    "9a86678f-e4b4-4c40-9a99-83029fd97730",
    "Maaz Ali Khan",
);

// DFT 483 — Kolodin, Triumph Caster (alternate printing)
const KOLODIN_TRIUMPH_CASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KOLODIN_TRIUMPH_CASTER,
    2,
    "46164400-0557-4c99-83d5-194dc814b46a",
    "Michal Ivan",
);

// DFT 484 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    4,
    "542fe4ca-d3d8-469c-b5cb-4d0824ba514b",
    "Ernanda Souza",
);

// DFT 485 — Mendicant Core, Guidelight (alternate printing)
const MENDICANT_CORE_GUIDELIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MENDICANT_CORE_GUIDELIGHT,
    2,
    "ec057f26-8d14-463b-84ce-cce82b95b08e",
    "Zezhou Chen",
);

// DFT 486 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    4,
    "2976e7e4-c53f-47d5-9890-cd30acad4f68",
    "Ron Spencer",
);

// DFT 487 — Oildeep Gearhulk (alternate printing)
const OILDEEP_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OILDEEP_GEARHULK,
    2,
    "a749306b-26ce-4c71-8586-a1ef9788c286",
    "Artur Nakhodkin",
);

// DFT 488 — Pyrewood Gearhulk (alternate printing)
const PYREWOOD_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PYREWOOD_GEARHULK,
    2,
    "794b7a29-eb42-4c52-a4d9-3976611efdd7",
    "Martin de Diego Sádaba",
);

// DFT 489 — Redshift, Rocketeer Chief (alternate printing)
const REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &REDSHIFT_ROCKETEER_CHIEF,
    2,
    "63ea12cb-c927-44f4-b304-f24919b1104e",
    "Wayne Reynolds",
);

// DFT 490 — Riptide Gearhulk (alternate printing)
const RIPTIDE_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RIPTIDE_GEARHULK,
    2,
    "81d5d0a0-b0ae-4a5c-ace6-77d87fcc8563",
    "Artur Nakhodkin",
);

// DFT 491 — Sab-Sunen, Luxa Embodied (alternate printing)
const SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SAB_SUNEN_LUXA_EMBODIED,
    2,
    "36581aa3-8970-40cc-bd1c-e78787ebe441",
    "Valera Lutfullina",
);

// DFT 492 — Samut, the Driving Force (alternate printing)
const SAMUT_THE_DRIVING_FORCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SAMUT_THE_DRIVING_FORCE,
    2,
    "c1bd0f7d-1686-42ae-9bea-66b96309f322",
    "Chris Rallis",
);

// DFT 493 — Sita Varma, Masked Racer (alternate printing)
const SITA_VARMA_MASKED_RACER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SITA_VARMA_MASKED_RACER,
    2,
    "321d595f-7efe-4004-9bbc-390b5a6eb734",
    "Kai Carpenter",
);

// DFT 494 — Winter, Cursed Rider (alternate printing)
const WINTER_CURSED_RIDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WINTER_CURSED_RIDER,
    2,
    "5f93e48d-4681-4354-aa85-493a48287233",
    "Daren Bader",
);

// DFT 495 — Zahur, Glory's Past (alternate printing)
const ZAHUR_GLORY_S_PAST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZAHUR_GLORY_S_PAST,
    2,
    "f10ee92a-85cc-48cb-b0b5-8e5185781fb2",
    "Leroy Steinmann",
);

// DFT 496 — The Aetherspark (alternate printing)
const THE_AETHERSPARK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_AETHERSPARK,
    2,
    "c0ea3cfc-64f1-4289-ac6c-82d55bab65de",
    "Donato Giancola",
);

// DFT 497 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    3,
    "8c14d4cc-79c3-4934-b7a8-25e1973ffea3",
    "Mirko Failoni",
);

// DFT 498 — Marketback Walker (alternate printing)
const MARKETBACK_WALKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MARKETBACK_WALKER,
    2,
    "20419718-d922-4f3c-aeb2-30ffb2ff7cbb",
    "Svetlin Velinov",
);

// DFT 499 — Monument to Endurance (alternate printing)
const MONUMENT_TO_ENDURANCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MONUMENT_TO_ENDURANCE,
    2,
    "c75f3b27-7c08-4e92-ae9a-6208aa9aec0c",
    "Victor Sales",
);

// DFT 500 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    4,
    "333039ef-b328-4082-85ae-162caed5612e",
    "Bruce Brenneise",
);

// DFT 501 — Bleachbone Verge (alternate printing)
const BLEACHBONE_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BLEACHBONE_VERGE,
    2,
    "712aab40-5452-4d62-8ad3-fec96d03db98",
    "Mark Tedin",
);

// DFT 502 — Muraganda Raceway (alternate printing)
const MURAGANDA_RACEWAY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MURAGANDA_RACEWAY,
    3,
    "b6457d47-8e68-42db-85c9-e0f981c53aa4",
    "Brian Valeza",
);

// DFT 503 — Riverpyre Verge (alternate printing)
const RIVERPYRE_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RIVERPYRE_VERGE,
    2,
    "c21462ee-76d7-4910-9a0c-b470f96b8b51",
    "Titus Lunter",
);

// DFT 504 — Sunbillow Verge (alternate printing)
const SUNBILLOW_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SUNBILLOW_VERGE,
    2,
    "cff03662-3879-4ca8-b73c-34918194203b",
    "Pete Venters",
);

// DFT 505 — Wastewood Verge (alternate printing)
const WASTEWOOD_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WASTEWOOD_VERGE,
    2,
    "ba0c9044-13ae-465e-a9d1-7dc63827540d",
    "Bartek Fedyczak",
);

// DFT 506 — Willowrush Verge (alternate printing)
const WILLOWRUSH_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WILLOWRUSH_VERGE,
    2,
    "8e928cc4-be33-4c99-b0d0-c808c225e3ab",
    "Aaron Miller",
);

// DFT 507 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "4e196d67-1923-459d-98cf-646337914d3b",
    "Adam Paquette",
);

// DFT 508 — Island (alternate printing)
const ISLAND_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    4,
    "9b33841e-7006-41d4-98b0-313ab151c9ec",
    "Maxime Minard",
);

// DFT 509 — Swamp (alternate printing)
const SWAMP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    4,
    "ba042d60-b1e8-44ad-9103-f40588a0b29c",
    "Jonas De Ro",
);

// DFT 510 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    4,
    "c7533cbb-ab73-4a0c-9ea0-baa97c4665b8",
    "Chris Ostrowski",
);

// DFT 511 — Forest (alternate printing)
const FOREST_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    4,
    "fd1b6c5c-4585-47c1-82eb-a324189f3ebd",
    "Andreas Rocha",
);

// DFT 512 — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    5,
    "5dd76e71-df9b-4fd2-b534-d3f724a56a91",
    "Calder Moore",
);

// DFT 513 — Island (alternate printing)
const ISLAND_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    5,
    "d13bb767-1123-48d2-960f-b90b6db3f6ed",
    "Calder Moore",
);

// DFT 514 — Swamp (alternate printing)
const SWAMP_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    5,
    "f4d198b3-2ff5-484a-905f-c272de7c139d",
    "Calder Moore",
);

// DFT 515 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    5,
    "5e4952a8-4e1f-4a74-8a06-c9d633b25dcd",
    "Calder Moore",
);

// DFT 516 — Forest (alternate printing)
const FOREST_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    5,
    "2a7aba60-d300-4c33-9e1e-846d6167dc50",
    "Calder Moore",
);

// DFT 517 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    5,
    "f2c7805b-c952-4847-b321-e0567923c80e",
    "Adam Volker",
);

// DFT 518 — Skyseer's Chariot (alternate printing)
const SKYSEER_S_CHARIOT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SKYSEER_S_CHARIOT,
    3,
    "d18d5057-39f7-42aa-9dd5-ecc52f861366",
    "Douglas P. Lobo",
);

// DFT 519 — Valor's Flagship (alternate printing)
const VALOR_S_FLAGSHIP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VALOR_S_FLAGSHIP,
    3,
    "58d4d35a-1f7e-4c5e-a999-09ad42f8b4e2",
    "William Tempest",
);

// DFT 520 — Voyager Glidecar (alternate printing)
const VOYAGER_GLIDECAR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VOYAGER_GLIDECAR,
    3,
    "b3666b4e-d6a4-41be-a529-4a577e75c86d",
    "Andrew Griffith",
);

// DFT 521 — Possession Engine (alternate printing)
const POSSESSION_ENGINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &POSSESSION_ENGINE,
    3,
    "96f0c1eb-603e-4e3e-b40b-65662c22d72d",
    "Douglas P. Lobo",
);

// DFT 522 — Thopter Fabricator (alternate printing)
const THOPTER_FABRICATOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THOPTER_FABRICATOR,
    3,
    "66f1f362-fa4a-49e6-baa5-6797a9408cad",
    "William Tempest",
);

// DFT 523 — Cryptcaller Chariot (alternate printing)
const CRYPTCALLER_CHARIOT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CRYPTCALLER_CHARIOT,
    3,
    "fa56ae43-4a6b-4af6-9707-679068bd0934",
    "Oliver Barrett",
);

// DFT 524 — Demonic Junker (alternate printing)
const DEMONIC_JUNKER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEMONIC_JUNKER,
    3,
    "a9747c8c-024f-417f-86ad-ee7c6f756aa2",
    "Deathburger",
);

// DFT 525 — The Last Ride (alternate printing)
const THE_LAST_RIDE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_RIDE,
    3,
    "f85dac5a-c6fa-4a86-9fe3-7a17db06f07c",
    "Deathburger",
);

// DFT 526 — Boommobile (alternate printing)
const BOOMMOBILE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BOOMMOBILE,
    3,
    "d62d070e-52b7-4d7f-bd98-13622d859a7b",
    "Ian Jepson",
);

// DFT 527 — Gastal Thrillroller (alternate printing)
const GASTAL_THRILLROLLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_THRILLROLLER,
    3,
    "a02b39bc-8eda-4c5a-8705-5416cdb24f0d",
    "Dan Mumford",
);

// DFT 528 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    4,
    "83725fcd-d5ec-4b99-b3f6-e7f5325c766e",
    "Francisco Badilla",
);

// DFT 529 — Thunderous Velocipede (alternate printing)
const THUNDEROUS_VELOCIPEDE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_VELOCIPEDE,
    3,
    "ac4ff9fc-5c34-4a07-a1f3-35c21a3323b0",
    "Eduardo Francisco",
);

// DFT 530 — Debris Beetle (alternate printing)
const DEBRIS_BEETLE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEBRIS_BEETLE,
    3,
    "4efa1ade-00f9-4736-8397-11d4faf65b7b",
    "Michal Ivan",
);

// DFT 531 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    4,
    "91496c82-44bc-4bb7-9e2a-5f4a3a199f4c",
    "Eduardo Francisco",
);

// DFT 532 — Bulwark Ox (alternate printing)
const BULWARK_OX_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BULWARK_OX,
    3,
    "4d4e8762-0aed-4009-86f0-7935a1d12b52",
    "Jon Vermilyea",
);

// DFT 533 — Guardian Sunmare (alternate printing)
const GUARDIAN_SUNMARE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GUARDIAN_SUNMARE,
    3,
    "72eb0c5b-ef3f-42db-9f40-0463cacd548d",
    "Wojtek Łebski",
);

// DFT 534 — Mindspring Merfolk (alternate printing)
const MINDSPRING_MERFOLK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MINDSPRING_MERFOLK,
    3,
    "f1191c3a-a6eb-4765-bb79-2f0d920899f9",
    "CatDirty",
);

// DFT 535 — Waxen Shapethief (alternate printing)
const WAXEN_SHAPETHIEF_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WAXEN_SHAPETHIEF,
    3,
    "ec7ee49c-1e5d-4b3a-8e0a-59a0d258e804",
    "CatDirty",
);

// DFT 536 — Bloodghast (alternate printing)
const BLOODGHAST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::BLOODGHAST,
    3,
    "3489de46-e90c-483e-ac2b-547497f64e82",
    "Wojtek Łebski",
);

// DFT 537 — Gas Guzzler (alternate printing)
const GAS_GUZZLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GAS_GUZZLER,
    3,
    "48b75e83-2ed5-48c9-881a-da3d77bdef99",
    "Sam McKenzie",
);

// DFT 538 — The Speed Demon (alternate printing)
const THE_SPEED_DEMON_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_SPEED_DEMON,
    3,
    "36bd51bd-e451-4f1d-af19-bb777ec1a85c",
    "CatDirty",
);

// DFT 539 — Burnout Bashtronaut (alternate printing)
const BURNOUT_BASHTRONAUT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BURNOUT_BASHTRONAUT,
    3,
    "b2151639-5621-4ffa-9374-3a1c48a635d9",
    "Jon Vermilyea",
);

// DFT 540 — Draconautics Engineer (alternate printing)
const DRACONAUTICS_ENGINEER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DRACONAUTICS_ENGINEER,
    3,
    "9419e00b-062b-4bb2-9203-f2f86b502371",
    "Ryan Roadkill",
);

// DFT 541 — Howlsquad Heavy (alternate printing)
const HOWLSQUAD_HEAVY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HOWLSQUAD_HEAVY,
    3,
    "bf709cb6-8953-40ce-9fcc-396b6c0ab37a",
    "Ryan Roadkill",
);

// DFT 542 — Agonasaur Rex (alternate printing)
const AGONASAUR_REX_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &AGONASAUR_REX,
    3,
    "8d815a94-cdf0-4a19-ad06-4292efbb7105",
    "Jon Vermilyea",
);

// DFT 543 — District Mascot (alternate printing)
const DISTRICT_MASCOT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DISTRICT_MASCOT,
    3,
    "3d8ad303-505d-4e2a-9380-80ab9f91a0a3",
    "Ryan Roadkill",
);

// DFT 544 — Webstrike Elite (alternate printing)
const WEBSTRIKE_ELITE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WEBSTRIKE_ELITE,
    3,
    "ecf3bcd2-fdc6-4490-858d-3334437a3148",
    "Sam McKenzie",
);

// DFT 545 — Fearless Swashbuckler (alternate printing)
const FEARLESS_SWASHBUCKLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &FEARLESS_SWASHBUCKLER,
    3,
    "47c8f0f0-f2d0-44fa-8163-41a8e35952fd",
    "CatDirty",
);

// DFT 546 — Hazoret, Godseeker (alternate printing)
const HAZORET_GODSEEKER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HAZORET_GODSEEKER,
    3,
    "c5e0ead2-933a-41ad-b1c3-0aa79cc41e93",
    "Massiveface",
);

// DFT 547 — Brightglass Gearhulk (alternate printing)
const BRIGHTGLASS_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BRIGHTGLASS_GEARHULK,
    3,
    "0b787e28-a6bf-489c-affb-4043845c9769",
    "Jorge Gutierrez Garcia",
);

// DFT 548 — Coalstoke Gearhulk (alternate printing)
const COALSTOKE_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &COALSTOKE_GEARHULK,
    3,
    "4363ec0d-a473-4afc-9ae5-4feed762a630",
    "Ivan Shavrin",
);

// DFT 549 — Ketramose, the New Dawn (alternate printing)
const KETRAMOSE_THE_NEW_DAWN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KETRAMOSE_THE_NEW_DAWN,
    3,
    "1688041b-a654-493e-bfac-247fdca68b22",
    "Florian Bertmer",
);

// DFT 550 — Oildeep Gearhulk (alternate printing)
const OILDEEP_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &OILDEEP_GEARHULK,
    3,
    "41dd7467-4e77-4fad-b8e9-b258fa6254f3",
    "Chun Lo",
);

// DFT 551 — Pyrewood Gearhulk (alternate printing)
const PYREWOOD_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &PYREWOOD_GEARHULK,
    3,
    "ec50050d-1da8-4f37-a975-f81d5684a619",
    "William Tempest",
);

// DFT 552 — Riptide Gearhulk (alternate printing)
const RIPTIDE_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RIPTIDE_GEARHULK,
    3,
    "6127cd9e-5785-4ba5-89d0-5eeb0d8aaa0e",
    "Kudaman",
);

// DFT 553 — Sab-Sunen, Luxa Embodied (alternate printing)
const SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SAB_SUNEN_LUXA_EMBODIED,
    3,
    "fb5f9907-e187-4c01-964c-24b3d27948b5",
    "Benjamin Ee",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AIR_RESPONSE_UNIT,
    &ALACRIAN_ARMORY,
    &BASRI_TOMORROW_S_CHAMPION,
    &BRIGHTFIELD_GLIDER,
    &BRIGHTFIELD_MUSTANG,
    &BROADCAST_RAMBLER,
    &BULWARK_OX,
    &CANYON_VAULTER,
    &CLOUDSPIRE_CAPTAIN,
    &COLLISION_COURSE,
    &DARING_MECHANIC,
    &DETENTION_CHARIOT,
    &GALLANT_STRIKE,
    &GLORYHEATH_LYNX,
    &GUARDIAN_SUNMARE,
    &GUIDELIGHT_SYNERGIST,
    &INTERFACE_ACE,
    &LEONIN_SURVEYOR,
    &LIGHTSHIELD_PARRY,
    &LIGHTWHEEL_ENHANCEMENTS,
    &LOTUSGUARD_DISCIPLE,
    &NESTING_BOT,
    &PERILOUS_SNARE,
    &PRIDE_OF_THE_ROAD,
    &RIDE_S_END,
    &ROADSIDE_ASSISTANCE,
    &SALVATION_ENGINE,
    &SKYSEER_S_CHARIOT,
    &SPECTACULAR_PILEUP,
    &SPOTCYCLE_SCOUTER,
    &SUNDIAL_DAWN_TYRANT,
    &SWIFTWING_ASSAILANT,
    &TUNE_UP,
    &UNSWERVING_SLOTH,
    &VALOR_S_FLAGSHIP,
    &VOYAGER_GLIDECAR,
    &VOYAGER_QUICKWELDER,
    &AETHER_SYPHON,
    &BOUNCE_OFF,
    &CAELORNA_CORAL_TYRANT,
    &DIVERSION_UNIT,
    &FLOOD_THE_ENGINE,
    &GLITCH_GHOST_SURVEYOR,
    &GUIDELIGHT_OPTIMIZER,
    &HOWLER_S_HEAVY,
    &HULLDRIFTER,
    &KEEN_BUCCANEER,
    &MEMORY_GUARDIAN,
    &MIDNIGHT_MANGLER,
    &MINDSPRING_MERFOLK,
    &MU_YANLING_WIND_RIDER,
    &NIMBLE_THOPTERIST,
    &POSSESSION_ENGINE,
    &RANGERS_REFUELER,
    &REPURPOSING_BAY,
    &RIVERCHURN_MONUMENT,
    &ROADSIDE_BLOWOUT,
    &SABOTAGE_STRATEGIST,
    &SCROUNGING_SKYRAY,
    &SKYSTREAK_ENGINEER,
    &SLICK_IMITATOR,
    &SPECTRAL_INTERFERENCE,
    &SPIKESHELL_HARRIER,
    &STALL_OUT,
    &STOCK_UP,
    &THOPTER_FABRICATOR,
    &TRADE_THE_HELM,
    &TRANSIT_MAGE,
    &TRIP_UP,
    &UNSTOPPABLE_PLAN,
    &VNWXT_VERBOSE_HOST,
    &WAXEN_SHAPETHIEF,
    &ANCIENT_VENDETTA,
    &BACK_ON_TRACK,
    &CARRION_CRUISER,
    &CHITIN_GRAVESTALKER,
    &CRYPTCALLER_CHARIOT,
    &CURSECLOTH_WRAPPINGS,
    &DEATHLESS_PILOT,
    &DEMONIC_JUNKER,
    &ENGINE_RAT,
    &GAS_GUZZLER,
    &GASTAL_RAIDER,
    &GONTI_NIGHT_MINISTER,
    &GRIM_BAUBLE,
    &GRIM_JAVELINEER,
    &HELLISH_SIDESWIPE,
    &HOUR_OF_VICTORY,
    &INTIMIDATION_TACTICS,
    &KALAKSCION_HUNGER_TYRANT,
    &THE_LAST_RIDE,
    &LOCUST_SPRAY,
    &MAXIMUM_OVERDRIVE,
    &MOMENTUM_BREAKER,
    &MUTANT_SURVEYOR,
    &PACTDOLL_TERROR,
    &QUAG_FEAST,
    &RIPCLAW_WRANGLER,
    &RISEN_NECROREGENT,
    &RISKY_SHORTCUT,
    &SHEFET_ARCHFIEND,
    &THE_SPEED_DEMON,
    &SPIN_OUT,
    &STREAKING_OILGORGER,
    &SYPHON_FUEL,
    &WICKERFOLK_INDOMITABLE,
    &WRECKAGE_WICKERFOLK,
    &WRETCHED_DOLL,
    &ADRENALINE_JOCKEY,
    &BOOMMOBILE,
    &BURNER_ROCKET,
    &BURNOUT_BASHTRONAUT,
    &CHANDRA_SPARK_HUNTER,
    &CLAMOROUS_IRONCLAD,
    &COUNT_ON_LUCK,
    &CRASH_AND_BURN,
    &DARETTI_ROCKETEER_ENGINEER,
    &DRACONAUTICS_ENGINEER,
    &DRACOSAUR_AUXILIARY,
    &DYNAMITE_DIVER,
    &ENDRIDER_CATALYZER,
    &ENDRIDER_SPIKESPITTER,
    &FUEL_THE_FLAMES,
    &FULL_THROTTLE,
    &GASTAL_BLOCKBUSTER,
    &GASTAL_THRILLROLLER,
    &GILDED_GHODA,
    &GOBLIN_SURVEYOR,
    &GREASEWRENCH_GOBLIN,
    &HAZORET_GODSEEKER,
    &HOWLSQUAD_HEAVY,
    &KICKOFF_CELEBRATIONS,
    &MAGMAKIN_ARTILLERIST,
    &MARAUDING_MAKO,
    &OUTPACE_OBLIVION,
    &PACESETTER_PARAGON,
    &PEDAL_TO_THE_METAL,
    &PROWCATCHER_SPECIALIST,
    &PUSH_THE_LIMIT,
    &RECKLESS_VELOCITAUR,
    &ROAD_RAGE,
    &SKYCRASH,
    &SPIRE_MECHCYCLE,
    &THUNDERHEAD_GUNNER,
    &TYROX_SAURID_TYRANT,
    &AFTERBURNER_EXPERT,
    &AGONASAUR_REX,
    &ALACRIAN_JAGUAR,
    &AUTARCH_MAMMOTH,
    &BEASTRIDER_VANGUARD,
    &BESTOW_GREATNESS,
    &DEFEND_THE_RIDER,
    &DISTRICT_MASCOT,
    &DREDGER_S_INSIGHT,
    &EARTHRUMBLER,
    &ELVISH_REFUELER,
    &FANG_GUARDIAN,
    &FANG_DRUID_SUMMONER,
    &GREENBELT_GUARDIAN,
    &HAZARD_OF_THE_DUNES,
    &JIBBIRIK_OMNIVORE,
    &LOXODON_SURVEYOR,
    &LUMBERING_WORLDWAGON,
    &MARCH_OF_THE_WORLD_OOZE,
    &MIGRATING_KETRADON,
    &MOLT_TENDER,
    &OOZE_PATROL,
    &OVIYA_AUTOMECH_ARTISAN,
    &PLOW_THROUGH,
    &POINT_THE_WAY,
    &POTHOLE_MOLE,
    &REGAL_IMPERIOSAUR,
    &RISE_FROM_THE_WRECK,
    &RUN_OVER,
    &SILKEN_STRENGTH,
    &STAMPEDING_SCURRYFOOT,
    &TERRIAN_WORLD_TYRANT,
    &THUNDEROUS_VELOCIPEDE,
    &VELOHEART_BIKE,
    &VENOMSAC_LAGAC,
    &WEBSTRIKE_ELITE,
    &AATCHIK_EMERALD_RADIAN,
    &APOCALYPSE_RUNNER,
    &BOOM_SCHOLAR,
    &BOOSTED_SLOOP,
    &BRIGHTGLASS_GEARHULK,
    &BROADSIDE_BARRAGE,
    &BROODHEART_ENGINE,
    &CAPTAIN_HOWLER_SEA_SCOURGE,
    &CARADORA_HEART_OF_ALACRIA,
    &CLOUDSPIRE_COORDINATOR,
    &CLOUDSPIRE_SKYCYCLE,
    &COALSTOKE_GEARHULK,
    &DEBRIS_BEETLE,
    &DUNE_DRIFTER,
    &EMBALMED_ASCENDANT,
    &EXPLOSIVE_GETAWAY,
    &FAR_FORTUNE_END_BOSS,
    &FEARLESS_SWASHBUCKLER,
    &GASTAL_THRILLSEEKER,
    &GUIDELIGHT_PATHMAKER,
    &HAUNT_THE_NETWORK,
    &HAUNTED_HELLRIDE,
    &KETRAMOSE_THE_NEW_DAWN,
    &KOLODIN_TRIUMPH_CASTER,
    &LAGORIN_SOUL_OF_ALACRIA,
    &LOOT_THE_PATHFINDER,
    &MENDICANT_CORE_GUIDELIGHT,
    &MIMEOPLASM_REVERED_ONE,
    &OILDEEP_GEARHULK,
    &PYREWOOD_GEARHULK,
    &RANGERS_AETHERHIVE,
    &REDSHIFT_ROCKETEER_CHIEF,
    &RIPTIDE_GEARHULK,
    &ROCKETEER_BOOSTBUGGY,
    &SAB_SUNEN_LUXA_EMBODIED,
    &SAMUT_THE_DRIVING_FORCE,
    &SITA_VARMA_MASKED_RACER,
    &SKYSERPENT_SEEKER,
    &THUNDERING_BROODWAGON,
    &VETERAN_BEASTRIDER,
    &VOYAGE_HOME,
    &WINTER_CURSED_RIDER,
    &ZAHUR_GLORY_S_PAST,
    &AETHERJACKET,
    &THE_AETHERSPARK,
    &CAMERA_LAUNCHER,
    &GUIDELIGHT_MATRIX,
    &LIFECRAFT_ENGINE,
    &MARKETBACK_WALKER,
    &MARSHALS_PATHCRUISER,
    &MONUMENT_TO_ENDURANCE,
    &PIT_AUTOMATON,
    &RACERS_SCOREBOARD,
    &RADIANT_LOTUS,
    &ROVER_BLADES,
    &SCRAP_COMPACTOR,
    &SKYBOX_FERRY,
    &STARTING_COLUMN,
    &TICKET_TORTOISE,
    &WALKING_SARCOPHAGUS,
    &WRECK_REMOVER,
    &AMONKHET_RACEWAY,
    &AVISHKAR_RACEWAY,
    &BLEACHBONE_VERGE,
    &COUNTRY_ROADS,
    &FOUL_ROADS,
    &MURAGANDA_RACEWAY,
    &NIGHT_MARKET,
    &REEF_ROADS,
    &RIVERPYRE_VERGE,
    &ROCKY_ROADS,
    &SUNBILLOW_VERGE,
    &WASTEWOOD_VERGE,
    &WILD_ROADS,
    &WILLOWRUSH_VERGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    GEARSEEKER_SERPENT_REPRINT,
    SPELL_PIERCE_REPRINT,
    BLOODGHAST_REPRINT,
    LIGHTNING_STRIKE_REPRINT,
    BROKEN_WINGS_REPRINT,
    BLOODFELL_CAVES_REPRINT,
    BLOSSOMING_SANDS_REPRINT,
    DISMAL_BACKWATER_REPRINT,
    JUNGLE_HOLLOW_REPRINT,
    RUGGED_HIGHLANDS_REPRINT,
    SCOURED_BARRENS_REPRINT,
    SWIFTWATER_CLIFFS_REPRINT,
    THORNWOOD_FALLS_REPRINT,
    TRANQUIL_COVE_REPRINT,
    WIND_SCARRED_CRAG_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
    AIR_RESPONSE_UNIT_ALTERNATE_1,
    BROADCAST_RAMBLER_ALTERNATE_1,
    DETENTION_CHARIOT_ALTERNATE_1,
    SALVATION_ENGINE_ALTERNATE_1,
    SKYSEER_S_CHARIOT_ALTERNATE_1,
    SPOTCYCLE_SCOUTER_ALTERNATE_1,
    VALOR_S_FLAGSHIP_ALTERNATE_1,
    VOYAGER_GLIDECAR_ALTERNATE_1,
    HULLDRIFTER_ALTERNATE_1,
    MIDNIGHT_MANGLER_ALTERNATE_1,
    POSSESSION_ENGINE_ALTERNATE_1,
    RANGERS_REFUELER_ALTERNATE_1,
    THOPTER_FABRICATOR_ALTERNATE_1,
    CARRION_CRUISER_ALTERNATE_1,
    CRYPTCALLER_CHARIOT_ALTERNATE_1,
    DEMONIC_JUNKER_ALTERNATE_1,
    THE_LAST_RIDE_ALTERNATE_1,
    RIPCLAW_WRANGLER_ALTERNATE_1,
    BOOMMOBILE_ALTERNATE_1,
    BURNER_ROCKET_ALTERNATE_1,
    CLAMOROUS_IRONCLAD_ALTERNATE_1,
    GASTAL_THRILLROLLER_ALTERNATE_1,
    SPIRE_MECHCYCLE_ALTERNATE_1,
    EARTHRUMBLER_ALTERNATE_1,
    LUMBERING_WORLDWAGON_ALTERNATE_1,
    THUNDEROUS_VELOCIPEDE_ALTERNATE_1,
    VELOHEART_BIKE_ALTERNATE_1,
    APOCALYPSE_RUNNER_ALTERNATE_1,
    BOOSTED_SLOOP_ALTERNATE_1,
    CLOUDSPIRE_SKYCYCLE_ALTERNATE_1,
    DEBRIS_BEETLE_ALTERNATE_1,
    DUNE_DRIFTER_ALTERNATE_1,
    GUIDELIGHT_PATHMAKER_ALTERNATE_1,
    HAUNTED_HELLRIDE_ALTERNATE_1,
    RANGERS_AETHERHIVE_ALTERNATE_1,
    ROCKETEER_BOOSTBUGGY_ALTERNATE_1,
    THUNDERING_BROODWAGON_ALTERNATE_1,
    LIFECRAFT_ENGINE_ALTERNATE_1,
    MARSHALS_PATHCRUISER_ALTERNATE_1,
    ROVER_BLADES_ALTERNATE_1,
    SKYBOX_FERRY_ALTERNATE_1,
    BULWARK_OX_ALTERNATE_1,
    GUARDIAN_SUNMARE_ALTERNATE_1,
    MINDSPRING_MERFOLK_ALTERNATE_1,
    WAXEN_SHAPETHIEF_ALTERNATE_1,
    BLOODGHAST_ALTERNATE_1,
    GAS_GUZZLER_ALTERNATE_1,
    THE_SPEED_DEMON_ALTERNATE_1,
    BURNOUT_BASHTRONAUT_ALTERNATE_1,
    DRACONAUTICS_ENGINEER_ALTERNATE_1,
    HOWLSQUAD_HEAVY_ALTERNATE_1,
    AGONASAUR_REX_ALTERNATE_1,
    DISTRICT_MASCOT_ALTERNATE_1,
    WEBSTRIKE_ELITE_ALTERNATE_1,
    FEARLESS_SWASHBUCKLER_ALTERNATE_1,
    HAZORET_GODSEEKER_ALTERNATE_1,
    BRIGHTGLASS_GEARHULK_ALTERNATE_1,
    COALSTOKE_GEARHULK_ALTERNATE_1,
    KETRAMOSE_THE_NEW_DAWN_ALTERNATE_1,
    OILDEEP_GEARHULK_ALTERNATE_1,
    PYREWOOD_GEARHULK_ALTERNATE_1,
    RIPTIDE_GEARHULK_ALTERNATE_1,
    SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_1,
    BASRI_TOMORROW_S_CHAMPION_ALTERNATE_1,
    VNWXT_VERBOSE_HOST_ALTERNATE_1,
    GONTI_NIGHT_MINISTER_ALTERNATE_1,
    DARETTI_ROCKETEER_ENGINEER_ALTERNATE_1,
    OVIYA_AUTOMECH_ARTISAN_ALTERNATE_1,
    AATCHIK_EMERALD_RADIAN_ALTERNATE_1,
    CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_1,
    CARADORA_HEART_OF_ALACRIA_ALTERNATE_1,
    FAR_FORTUNE_END_BOSS_ALTERNATE_1,
    KOLODIN_TRIUMPH_CASTER_ALTERNATE_1,
    MENDICANT_CORE_GUIDELIGHT_ALTERNATE_1,
    REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_1,
    SAMUT_THE_DRIVING_FORCE_ALTERNATE_1,
    SITA_VARMA_MASKED_RACER_ALTERNATE_1,
    WINTER_CURSED_RIDER_ALTERNATE_1,
    ZAHUR_GLORY_S_PAST_ALTERNATE_1,
    BLEACHBONE_VERGE_ALTERNATE_1,
    RIVERPYRE_VERGE_ALTERNATE_1,
    SUNBILLOW_VERGE_ALTERNATE_1,
    WASTEWOOD_VERGE_ALTERNATE_1,
    WILLOWRUSH_VERGE_ALTERNATE_1,
    THE_AETHERSPARK_ALTERNATE_1,
    PERILOUS_SNARE_ALTERNATE_1,
    SPECTACULAR_PILEUP_ALTERNATE_1,
    MU_YANLING_WIND_RIDER_ALTERNATE_1,
    REPURPOSING_BAY_ALTERNATE_1,
    RIVERCHURN_MONUMENT_ALTERNATE_1,
    UNSTOPPABLE_PLAN_ALTERNATE_1,
    CURSECLOTH_WRAPPINGS_ALTERNATE_1,
    QUAG_FEAST_ALTERNATE_1,
    COUNT_ON_LUCK_ALTERNATE_1,
    FULL_THROTTLE_ALTERNATE_1,
    AFTERBURNER_EXPERT_ALTERNATE_1,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_1,
    REGAL_IMPERIOSAUR_ALTERNATE_1,
    EXPLOSIVE_GETAWAY_ALTERNATE_1,
    LOOT_THE_PATHFINDER_ALTERNATE_1,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_1,
    MARKETBACK_WALKER_ALTERNATE_1,
    MONUMENT_TO_ENDURANCE_ALTERNATE_1,
    RADIANT_LOTUS_ALTERNATE_1,
    MURAGANDA_RACEWAY_ALTERNATE_1,
    SALVATION_ENGINE_ALTERNATE_2,
    SPECTACULAR_PILEUP_ALTERNATE_2,
    MU_YANLING_WIND_RIDER_ALTERNATE_2,
    CURSECLOTH_WRAPPINGS_ALTERNATE_2,
    CHANDRA_SPARK_HUNTER_ALTERNATE_1,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_2,
    EXPLOSIVE_GETAWAY_ALTERNATE_2,
    LOOT_THE_PATHFINDER_ALTERNATE_2,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_2,
    RADIANT_LOTUS_ALTERNATE_2,
    SALVATION_ENGINE_ALTERNATE_3,
    SPECTACULAR_PILEUP_ALTERNATE_3,
    MU_YANLING_WIND_RIDER_ALTERNATE_3,
    CURSECLOTH_WRAPPINGS_ALTERNATE_3,
    CHANDRA_SPARK_HUNTER_ALTERNATE_2,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_3,
    EXPLOSIVE_GETAWAY_ALTERNATE_3,
    LOOT_THE_PATHFINDER_ALTERNATE_3,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_3,
    RADIANT_LOTUS_ALTERNATE_3,
    TUNE_UP_ALTERNATE_1,
    GASTAL_RAIDER_ALTERNATE_1,
    MARAUDING_MAKO_ALTERNATE_1,
    SKYSERPENT_SEEKER_ALTERNATE_1,
    VOYAGE_HOME_ALTERNATE_1,
    LUMBERING_WORLDWAGON_ALTERNATE_2,
    LIFECRAFT_ENGINE_ALTERNATE_2,
    AMONKHET_RACEWAY_ALTERNATE_1,
    AVISHKAR_RACEWAY_ALTERNATE_1,
    MURAGANDA_RACEWAY_ALTERNATE_2,
    BASRI_TOMORROW_S_CHAMPION_ALTERNATE_2,
    BULWARK_OX_ALTERNATE_2,
    GUARDIAN_SUNMARE_ALTERNATE_2,
    PERILOUS_SNARE_ALTERNATE_2,
    SALVATION_ENGINE_ALTERNATE_4,
    SKYSEER_S_CHARIOT_ALTERNATE_2,
    SPECTACULAR_PILEUP_ALTERNATE_4,
    VALOR_S_FLAGSHIP_ALTERNATE_2,
    VOYAGER_GLIDECAR_ALTERNATE_2,
    MINDSPRING_MERFOLK_ALTERNATE_2,
    MU_YANLING_WIND_RIDER_ALTERNATE_4,
    POSSESSION_ENGINE_ALTERNATE_2,
    REPURPOSING_BAY_ALTERNATE_2,
    RIVERCHURN_MONUMENT_ALTERNATE_2,
    THOPTER_FABRICATOR_ALTERNATE_2,
    UNSTOPPABLE_PLAN_ALTERNATE_2,
    VNWXT_VERBOSE_HOST_ALTERNATE_2,
    WAXEN_SHAPETHIEF_ALTERNATE_2,
    BLOODGHAST_ALTERNATE_2,
    CRYPTCALLER_CHARIOT_ALTERNATE_2,
    CURSECLOTH_WRAPPINGS_ALTERNATE_4,
    DEMONIC_JUNKER_ALTERNATE_2,
    GAS_GUZZLER_ALTERNATE_2,
    GONTI_NIGHT_MINISTER_ALTERNATE_2,
    THE_LAST_RIDE_ALTERNATE_2,
    QUAG_FEAST_ALTERNATE_2,
    THE_SPEED_DEMON_ALTERNATE_2,
    BOOMMOBILE_ALTERNATE_2,
    BURNOUT_BASHTRONAUT_ALTERNATE_2,
    CHANDRA_SPARK_HUNTER_ALTERNATE_3,
    COUNT_ON_LUCK_ALTERNATE_2,
    DARETTI_ROCKETEER_ENGINEER_ALTERNATE_2,
    DRACONAUTICS_ENGINEER_ALTERNATE_2,
    FULL_THROTTLE_ALTERNATE_2,
    GASTAL_THRILLROLLER_ALTERNATE_2,
    HAZORET_GODSEEKER_ALTERNATE_2,
    HOWLSQUAD_HEAVY_ALTERNATE_2,
    AFTERBURNER_EXPERT_ALTERNATE_2,
    AGONASAUR_REX_ALTERNATE_2,
    DISTRICT_MASCOT_ALTERNATE_2,
    LUMBERING_WORLDWAGON_ALTERNATE_3,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_4,
    OVIYA_AUTOMECH_ARTISAN_ALTERNATE_2,
    REGAL_IMPERIOSAUR_ALTERNATE_2,
    THUNDEROUS_VELOCIPEDE_ALTERNATE_2,
    WEBSTRIKE_ELITE_ALTERNATE_2,
    AATCHIK_EMERALD_RADIAN_ALTERNATE_2,
    BRIGHTGLASS_GEARHULK_ALTERNATE_2,
    CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_2,
    CARADORA_HEART_OF_ALACRIA_ALTERNATE_2,
    COALSTOKE_GEARHULK_ALTERNATE_2,
    DEBRIS_BEETLE_ALTERNATE_2,
    EXPLOSIVE_GETAWAY_ALTERNATE_4,
    FAR_FORTUNE_END_BOSS_ALTERNATE_2,
    FEARLESS_SWASHBUCKLER_ALTERNATE_2,
    KETRAMOSE_THE_NEW_DAWN_ALTERNATE_2,
    KOLODIN_TRIUMPH_CASTER_ALTERNATE_2,
    LOOT_THE_PATHFINDER_ALTERNATE_4,
    MENDICANT_CORE_GUIDELIGHT_ALTERNATE_2,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_4,
    OILDEEP_GEARHULK_ALTERNATE_2,
    PYREWOOD_GEARHULK_ALTERNATE_2,
    REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_2,
    RIPTIDE_GEARHULK_ALTERNATE_2,
    SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_2,
    SAMUT_THE_DRIVING_FORCE_ALTERNATE_2,
    SITA_VARMA_MASKED_RACER_ALTERNATE_2,
    WINTER_CURSED_RIDER_ALTERNATE_2,
    ZAHUR_GLORY_S_PAST_ALTERNATE_2,
    THE_AETHERSPARK_ALTERNATE_2,
    LIFECRAFT_ENGINE_ALTERNATE_3,
    MARKETBACK_WALKER_ALTERNATE_2,
    MONUMENT_TO_ENDURANCE_ALTERNATE_2,
    RADIANT_LOTUS_ALTERNATE_4,
    BLEACHBONE_VERGE_ALTERNATE_2,
    MURAGANDA_RACEWAY_ALTERNATE_3,
    RIVERPYRE_VERGE_ALTERNATE_2,
    SUNBILLOW_VERGE_ALTERNATE_2,
    WASTEWOOD_VERGE_ALTERNATE_2,
    WILLOWRUSH_VERGE_ALTERNATE_2,
    PLAINS_ALTERNATE_4,
    ISLAND_ALTERNATE_4,
    SWAMP_ALTERNATE_4,
    MOUNTAIN_ALTERNATE_4,
    FOREST_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    ISLAND_ALTERNATE_5,
    SWAMP_ALTERNATE_5,
    MOUNTAIN_ALTERNATE_5,
    FOREST_ALTERNATE_5,
    SALVATION_ENGINE_ALTERNATE_5,
    SKYSEER_S_CHARIOT_ALTERNATE_3,
    VALOR_S_FLAGSHIP_ALTERNATE_3,
    VOYAGER_GLIDECAR_ALTERNATE_3,
    POSSESSION_ENGINE_ALTERNATE_3,
    THOPTER_FABRICATOR_ALTERNATE_3,
    CRYPTCALLER_CHARIOT_ALTERNATE_3,
    DEMONIC_JUNKER_ALTERNATE_3,
    THE_LAST_RIDE_ALTERNATE_3,
    BOOMMOBILE_ALTERNATE_3,
    GASTAL_THRILLROLLER_ALTERNATE_3,
    LUMBERING_WORLDWAGON_ALTERNATE_4,
    THUNDEROUS_VELOCIPEDE_ALTERNATE_3,
    DEBRIS_BEETLE_ALTERNATE_3,
    LIFECRAFT_ENGINE_ALTERNATE_4,
    BULWARK_OX_ALTERNATE_3,
    GUARDIAN_SUNMARE_ALTERNATE_3,
    MINDSPRING_MERFOLK_ALTERNATE_3,
    WAXEN_SHAPETHIEF_ALTERNATE_3,
    BLOODGHAST_ALTERNATE_3,
    GAS_GUZZLER_ALTERNATE_3,
    THE_SPEED_DEMON_ALTERNATE_3,
    BURNOUT_BASHTRONAUT_ALTERNATE_3,
    DRACONAUTICS_ENGINEER_ALTERNATE_3,
    HOWLSQUAD_HEAVY_ALTERNATE_3,
    AGONASAUR_REX_ALTERNATE_3,
    DISTRICT_MASCOT_ALTERNATE_3,
    WEBSTRIKE_ELITE_ALTERNATE_3,
    FEARLESS_SWASHBUCKLER_ALTERNATE_3,
    HAZORET_GODSEEKER_ALTERNATE_3,
    BRIGHTGLASS_GEARHULK_ALTERNATE_3,
    COALSTOKE_GEARHULK_ALTERNATE_3,
    KETRAMOSE_THE_NEW_DAWN_ALTERNATE_3,
    OILDEEP_GEARHULK_ALTERNATE_3,
    PYREWOOD_GEARHULK_ALTERNATE_3,
    RIPTIDE_GEARHULK_ALTERNATE_3,
    SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_3,
];
