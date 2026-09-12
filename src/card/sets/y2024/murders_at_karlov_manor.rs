//! Murders at Karlov Manor card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

static SURVEIL_LAND_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_tapped(CardType::Land),
    abilities::enters_trigger(
        "When this land enters, surveil 1. (Look at the top card of \
         your library. You may put it into your graveyard.)",
        abilities::surveil(ValueDef::Constant(1)),
    ),
];

/// The surveil-land cycle: two basic types, tapped on the way in, and one
/// look at the top of your library to pay for it. The mana abilities come
/// from the types rather than from a printed clause.
const fn surveil_land(types: &'static [&'static str]) -> CardRules {
    CardRules::new_land(types).with_abilities(&SURVEIL_LAND_ABILITIES)
}

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::stronghold as catalog_sth;
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2016::shadows_over_innistrad as catalog_soi;
use crate::card::sets::y2018::guilds_of_ravnica as catalog_grn;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MKM",
    slug: "murders-at-karlov-manor",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const CLUE_TOKEN: TokenCharacteristics = tokens::clue().with_art(CardArt::new(
    "ef607895-d6d2-44ab-a6b4-84af55fce593",
    "Daneen Wilkerson",
));

// MKM 1 — Case of the Shattered Pact
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_SHATTERED_PACT: CardRecord = CardRecord::new(
    "Case of the Shattered Pact",
    "2a70f0ae-d49b-4cc8-9f76-895039c3dc39",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// MKM 2 — Absolving Lammasu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABSOLVING_LAMMASU: CardRecord = CardRecord::new(
    "Absolving Lammasu",
    "bd6e71a1-713e-4eca-bd65-9f0638c16794",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// MKM 3 — Assemble the Players
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASSEMBLE_THE_PLAYERS: CardRecord = CardRecord::new(
    "Assemble the Players",
    "f5bcb21a-8559-4791-8cd0-482e7b8dcfd2",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// MKM 4 — Aurelia's Vindicator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURELIA_S_VINDICATOR: CardRecord = CardRecord::new(
    "Aurelia's Vindicator",
    "5901dff4-e09b-4747-9297-797a1a057cd5",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// MKM 5 — Auspicious Arrival
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUSPICIOUS_ARRIVAL: CardRecord = CardRecord::new(
    "Auspicious Arrival",
    "5180c85c-6add-4066-83c4-27fb1fd4de16",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// MKM 6 — Call a Surprise Witness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_A_SURPRISE_WITNESS: CardRecord = CardRecord::new(
    "Call a Surprise Witness",
    "f5148def-cf1a-460e-8dfd-856103940892",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// MKM 7 — Case File Auditor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_FILE_AUDITOR: CardRecord = CardRecord::new(
    "Case File Auditor",
    "70a52038-9d1c-4be1-8dbe-6f0ee916ba94",
    "Ryan Valle",
    crate::card::CardRules::unsupported(),
);

// MKM 7† — Case File Auditor (alternate printing)
const CASE_FILE_AUDITOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CASE_FILE_AUDITOR,
    1,
    "1e2278d7-215e-49f9-ae2b-9df099c67b6d",
    "Ryan Valle",
);

// MKM 8 — Case of the Gateway Express
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_GATEWAY_EXPRESS: CardRecord = CardRecord::new(
    "Case of the Gateway Express",
    "0862bf07-8a76-4e80-bba2-20d22f8eee30",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// MKM 9 — Case of the Pilfered Proof
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_PILFERED_PROOF: CardRecord = CardRecord::new(
    "Case of the Pilfered Proof",
    "32927bf2-63c1-4402-99dc-3a0f2f8e0f9c",
    "Joshua Cairos",
    crate::card::CardRules::unsupported(),
);

// MKM 10 — Case of the Uneaten Feast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_UNEATEN_FEAST: CardRecord = CardRecord::new(
    "Case of the Uneaten Feast",
    "ac63941b-3f78-4bd3-8b05-ca12aaaa006c",
    "Titus Lunter",
    crate::card::CardRules::unsupported(),
);

// MKM 11 — Defenestrated Phantom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENESTRATED_PHANTOM: CardRecord = CardRecord::new(
    "Defenestrated Phantom",
    "8b284304-c3bf-4413-9fd3-b44eb4eb642a",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// MKM 12 — Delney, Streetwise Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELNEY_STREETWISE_LOOKOUT: CardRecord = CardRecord::new(
    "Delney, Streetwise Lookout",
    "be219928-3d0e-4d00-b124-152ce8a8c13b",
    "Darren Tan",
    crate::card::CardRules::unsupported(),
);

// MKM 13 — Doorkeeper Thrull
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOORKEEPER_THRULL: CardRecord = CardRecord::new(
    "Doorkeeper Thrull",
    "80a1cd28-d2a5-4d1a-aa03-a6a5958ae432",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// MKM 14 — Due Diligence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUE_DILIGENCE: CardRecord = CardRecord::new(
    "Due Diligence",
    "076d9f76-a247-4727-9e0f-a0289c51059e",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 15 — Essence of Antiquity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_OF_ANTIQUITY: CardRecord = CardRecord::new(
    "Essence of Antiquity",
    "aee2945d-bf6d-4328-a482-df24c2973b56",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// MKM 16 — Forum Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORUM_FAMILIAR: CardRecord = CardRecord::new(
    "Forum Familiar",
    "b06a243d-acc8-42cd-926c-98a4cc96ab21",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// MKM 17 — Griffnaut Tracker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIFFNAUT_TRACKER: CardRecord = CardRecord::new(
    "Griffnaut Tracker",
    "95f5d048-226f-49a4-a2ce-a6fa99aa9e8a",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// MKM 18 — Haazda Vigilante
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAAZDA_VIGILANTE: CardRecord = CardRecord::new(
    "Haazda Vigilante",
    "5277fe72-dc33-4d19-a439-63b5344c6034",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// MKM 19 — Inside Source
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIDE_SOURCE: CardRecord = CardRecord::new(
    "Inside Source",
    "1548d181-3f83-457a-b2d3-eb88cfb2afda",
    "John Stanko",
    crate::card::CardRules::unsupported(),
);

// MKM 20 — Karlov Watchdog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARLOV_WATCHDOG: CardRecord = CardRecord::new(
    "Karlov Watchdog",
    "79cfb366-ae2a-4b3d-9a80-383a32db1509",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// MKM 21 — Krovod Haunch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVOD_HAUNCH: CardRecord = CardRecord::new(
    "Krovod Haunch",
    "f663cd86-39e6-467b-85b7-dd27536251a6",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// MKM 22 — Make Your Move
pub(in crate::card::sets) static MAKE_YOUR_MOVE: CardRecord = CardRecord::new(
    "Make Your Move",
    "73475d29-2673-4614-86d3-404232426aa8",
    "Xabi Gaztelua",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact, enchantment, or creature with power \
         4 or greater.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(4),
                ]),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )]),
);

// MKM 23 — Makeshift Binding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAKESHIFT_BINDING: CardRecord = CardRecord::new(
    "Makeshift Binding",
    "e45d2e0c-d70d-40e5-8c3d-db6803393516",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// MKM 24 — Marketwatch Phantom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARKETWATCH_PHANTOM: CardRecord = CardRecord::new(
    "Marketwatch Phantom",
    "daf6a392-3fc4-45d1-b85d-3b1381b3ab7d",
    "Bram Sels",
    crate::card::CardRules::unsupported(),
);

// MKM 25 — Museum Nightwatch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUSEUM_NIGHTWATCH: CardRecord = CardRecord::new(
    "Museum Nightwatch",
    "37860682-4973-4a0f-a43a-3056037bd2dc",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// MKM 26 — Neighborhood Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEIGHBORHOOD_GUARDIAN: CardRecord = CardRecord::new(
    "Neighborhood Guardian",
    "0438d482-b74c-4d5e-a2bc-7063c1ae73fa",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// MKM 27 — No Witnesses
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NO_WITNESSES: CardRecord = CardRecord::new(
    "No Witnesses",
    "f98db67a-c39c-45a8-ae21-85133be46ed5",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// MKM 28 — Not on My Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOT_ON_MY_WATCH: CardRecord = CardRecord::new(
    "Not on My Watch",
    "700294fc-7c16-4f7d-bce0-7452d8e9c401",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// MKM 29 — Novice Inspector
pub(in crate::card::sets) static NOVICE_INSPECTOR: CardRecord = CardRecord::new(
    "Novice Inspector",
    "0ad38866-fc5f-4f62-89c1-afc0f50765aa",
    "Fajareka Setiawan",
// One mana for a blocker and half a card, which is the floor a white
    // one-drop has to clear to be playable at all.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Detective"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, investigate. (Create a Clue token. It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                CLUE_TOKEN,
            ))),
        ),
    ),
);

// MKM 30 — On the Job
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ON_THE_JOB: CardRecord = CardRecord::new(
    "On the Job",
    "48b92629-4196-4943-91fd-8c8d5f3fcaef",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// MKM 31 — Perimeter Enforcer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERIMETER_ENFORCER: CardRecord = CardRecord::new(
    "Perimeter Enforcer",
    "1f88d077-5082-4a67-91e4-97aafb9a5e91",
    "Josh Hass",
    crate::card::CardRules::unsupported(),
);

// MKM 32 — Sanctuary Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANCTUARY_WALL: CardRecord = CardRecord::new(
    "Sanctuary Wall",
    "4a009ba2-c7b9-4cf6-bb90-9d6fd589e932",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// MKM 33 — Seasoned Consultant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASONED_CONSULTANT: CardRecord = CardRecord::new(
    "Seasoned Consultant",
    "c0ebfe5d-8819-495d-bf72-b9c28c6fd23e",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// MKM 34 — Tenth District Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TENTH_DISTRICT_HERO: CardRecord = CardRecord::new(
    "Tenth District Hero",
    "7c65a79e-f28a-4f30-95a4-1ea55fd84564",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// MKM 35 — Unyielding Gatekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNYIELDING_GATEKEEPER: CardRecord = CardRecord::new(
    "Unyielding Gatekeeper",
    "f3a0d597-d2df-4aaf-8084-c8eeda64ce60",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 36 — Wojek Investigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOJEK_INVESTIGATOR: CardRecord = CardRecord::new(
    "Wojek Investigator",
    "296574c6-3933-4ab3-b591-72514b244da9",
    "Ben Hill",
    crate::card::CardRules::unsupported(),
);

// MKM 36† — Wojek Investigator (alternate printing)
const WOJEK_INVESTIGATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WOJEK_INVESTIGATOR,
    1,
    "684bb0b1-c374-4af7-b018-fa7efaf14b25",
    "Ben Hill",
);

// MKM 37 — Wrench
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRENCH: CardRecord = CardRecord::new(
    "Wrench",
    "c36cc39b-8f3b-4297-b118-86fa624308b4",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// MKM 38 — Agency Outfitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENCY_OUTFITTER: CardRecord = CardRecord::new(
    "Agency Outfitter",
    "8112f133-535e-4264-8357-9cbf97957710",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// MKM 39 — Behind the Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEHIND_THE_MASK: CardRecord = CardRecord::new(
    "Behind the Mask",
    "e522b043-fbd8-48a4-9f20-39e2a66a35ec",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// MKM 40 — Benthic Criminologists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENTHIC_CRIMINOLOGISTS: CardRecord = CardRecord::new(
    "Benthic Criminologists",
    "283b6b5a-acdf-4255-a294-0964d9c62686",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// MKM 41 — Bubble Smuggler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUBBLE_SMUGGLER: CardRecord = CardRecord::new(
    "Bubble Smuggler",
    "6b863ee0-d9f3-4b1e-993d-5212731d9353",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// MKM 42 — Burden of Proof
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURDEN_OF_PROOF: CardRecord = CardRecord::new(
    "Burden of Proof",
    "4ea29c34-4b55-4170-9120-0a8dda61f2eb",
    "Deruchenko Alexander",
    crate::card::CardRules::unsupported(),
);

// MKM 43 — Candlestick
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANDLESTICK: CardRecord = CardRecord::new(
    "Candlestick",
    "5aeae6fb-3834-4891-924e-3d1fb3e19e09",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// MKM 44 — Case of the Filched Falcon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_FILCHED_FALCON: CardRecord = CardRecord::new(
    "Case of the Filched Falcon",
    "266be5bd-71ba-4511-8b71-d0b03885a28d",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// MKM 45 — Case of the Ransacked Lab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_RANSACKED_LAB: CardRecord = CardRecord::new(
    "Case of the Ransacked Lab",
    "16a9a596-61de-4fcf-aae0-41836c3deca5",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 46 — Cold Case Cracker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLD_CASE_CRACKER: CardRecord = CardRecord::new(
    "Cold Case Cracker",
    "f082111b-9b1c-4a25-8c5d-d6ef77533a9b",
    "Wayne Wu",
    crate::card::CardRules::unsupported(),
);

// MKM 47 — Conspiracy Unraveler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSPIRACY_UNRAVELER: CardRecord = CardRecord::new(
    "Conspiracy Unraveler",
    "88e791fc-bf9f-49b6-b5f2-a24d4b3e360e",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// MKM 48 — Coveted Falcon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COVETED_FALCON: CardRecord = CardRecord::new(
    "Coveted Falcon",
    "bc936987-d58b-4e7c-870f-379bcae77727",
    "Madeline Boni",
    crate::card::CardRules::unsupported(),
);

// MKM 49 — Crimestopper Sprite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIMESTOPPER_SPRITE: CardRecord = CardRecord::new(
    "Crimestopper Sprite",
    "dc4ac597-38f0-48b5-ac2d-dfb0b169f834",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// MKM 50 — Cryptic Coat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPTIC_COAT: CardRecord = CardRecord::new(
    "Cryptic Coat",
    "0c3d7e2c-a104-4757-9984-fb31088f92c4",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// MKM 51 — Curious Inquiry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURIOUS_INQUIRY: CardRecord = CardRecord::new(
    "Curious Inquiry",
    "5a3603c6-92df-45c7-b402-1f0a552ea398",
    "Ekaterina Burmak",
    crate::card::CardRules::unsupported(),
);

// MKM 52 — Deduce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEDUCE: CardRecord = CardRecord::new(
    "Deduce",
    "7cbb17af-e17e-438a-ad72-0c942e6706b6",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// MKM 53 — Dramatic Accusation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAMATIC_ACCUSATION: CardRecord = CardRecord::new(
    "Dramatic Accusation",
    "2ca93438-a132-45ca-9fa8-364aeb519594",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// MKM 54 — Eliminate the Impossible
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELIMINATE_THE_IMPOSSIBLE: CardRecord = CardRecord::new(
    "Eliminate the Impossible",
    "486f1cc2-c162-448e-91a9-577d7d796584",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// MKM 55 — Exit Specialist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXIT_SPECIALIST: CardRecord = CardRecord::new(
    "Exit Specialist",
    "268f142d-9fb1-4673-b804-add1f08dacb9",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// MKM 56 — Fae Flight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAE_FLIGHT: CardRecord = CardRecord::new(
    "Fae Flight",
    "d9caa4eb-ed8c-4d05-8029-2a42163938a7",
    "Durion",
    crate::card::CardRules::unsupported(),
);

// MKM 57 — Forensic Gadgeteer
pub(in crate::card::sets) static FORENSIC_GADGETEER: CardRecord = CardRecord::new(
    "Forensic Gadgeteer",
    "97d08a15-e61c-4421-a541-c68a4f87cb74",
    "Volkan Baǵa",
// Every artifact you cast is a card later, and every artifact you
    // already have is cheaper to use -- including the Clues it just made.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Artificer", "Detective"], 2, 3)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast an artifact spell, investigate. (Create a Clue token. It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
                // An artifact spell you cast, which is the whole of the trigger: what it
                // does is not part of the condition.
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    CLUE_TOKEN,
                ))),
            ),
            AbilityDef::static_ability(
                "Activated abilities of artifacts you control cost {1} less to activate. This effect can't reduce the mana in that cost to less than one mana.",
                EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                    permanent: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    amount: ValueDef::Constant(1),
                    minimum: 1,
                }),
            ),
        ]),
);

// MKM 58 — Forensic Researcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORENSIC_RESEARCHER: CardRecord = CardRecord::new(
    "Forensic Researcher",
    "1384df5d-d705-49cb-a982-1588cbf303d8",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// MKM 59 — Furtive Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FURTIVE_COURIER: CardRecord = CardRecord::new(
    "Furtive Courier",
    "6f359fc2-b9e4-4a01-9d04-442bb160b01e",
    "Mark Behm",
    crate::card::CardRules::unsupported(),
);

// MKM 60 — Hotshot Investigators
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOTSHOT_INVESTIGATORS: CardRecord = CardRecord::new(
    "Hotshot Investigators",
    "8dc3b23b-04d6-4ac0-b698-596ea90b7781",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// MKM 61 — Intrude on the Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTRUDE_ON_THE_MIND: CardRecord = CardRecord::new(
    "Intrude on the Mind",
    "fbe62f47-df17-4646-88ca-89a8ec4deee9",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// MKM 62 — Jaded Analyst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JADED_ANALYST: CardRecord = CardRecord::new(
    "Jaded Analyst",
    "2807dcfb-d99c-483b-835f-2606eae4bd30",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 63 — Living Conundrum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_CONUNDRUM: CardRecord = CardRecord::new(
    "Living Conundrum",
    "97fb11c7-7b7f-4bdb-a022-53e28ebadecc",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

// MKM 64 — Lost in the Maze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOST_IN_THE_MAZE: CardRecord = CardRecord::new(
    "Lost in the Maze",
    "6308dc62-d945-4761-aa4c-ef8e9271e901",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// MKM 65 — Mistway Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTWAY_SPY: CardRecord = CardRecord::new(
    "Mistway Spy",
    "e8578839-046f-4afd-a0e7-4737ded9e6eb",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// MKM 66 — Out Cold
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUT_COLD: CardRecord = CardRecord::new(
    "Out Cold",
    "aabfada0-3c1b-4237-b06c-573071ccd68d",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// MKM 67 — Proft's Eidetic Memory (alternate printing)
const PROFT_S_EIDETIC_MEMORY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PROFT_S_EIDETIC_MEMORY,
    1,
    "af5b29b3-974c-4200-8df8-b072c11e1600",
    "Julie Dillon",
);

// MKM 68 — Projektor Inspector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROJEKTOR_INSPECTOR: CardRecord = CardRecord::new(
    "Projektor Inspector",
    "ad378843-e2b0-48d6-90dc-b584e857473d",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// MKM 69 — Reasonable Doubt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REASONABLE_DOUBT: CardRecord = CardRecord::new(
    "Reasonable Doubt",
    "270570a3-8637-4e4e-92d9-e985474cd5d2",
    "Betty Jiang",
    crate::card::CardRules::unsupported(),
);

// MKM 70 — Reenact the Crime
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REENACT_THE_CRIME: CardRecord = CardRecord::new(
    "Reenact the Crime",
    "d942e4ce-f582-4264-89aa-9b4a743e6b29",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// MKM 71 — Steamcore Scholar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAMCORE_SCHOLAR: CardRecord = CardRecord::new(
    "Steamcore Scholar",
    "6bbf7394-9b17-45f8-a25b-d865e8452b2c",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// MKM 72 — Sudden Setback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUDDEN_SETBACK: CardRecord = CardRecord::new(
    "Sudden Setback",
    "0b9e5fd6-a5ea-4ae5-83f5-89ed6a658dd3",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// MKM 72† — Sudden Setback (alternate printing)
const SUDDEN_SETBACK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUDDEN_SETBACK,
    1,
    "4cbc6af2-f6b1-429d-b3ac-06a2c392d2b8",
    "Olivier Bernard",
);

// MKM 73 — Surveillance Monitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURVEILLANCE_MONITOR: CardRecord = CardRecord::new(
    "Surveillance Monitor",
    "703b874d-6739-4063-9891-e9c040dd9618",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// MKM 73† — Surveillance Monitor (alternate printing)
const SURVEILLANCE_MONITOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SURVEILLANCE_MONITOR,
    1,
    "19529568-c759-4ae0-b2e4-ab5a11d421a4",
    "Scott Murphy",
);

// MKM 74 — Unauthorized Exit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNAUTHORIZED_EXIT: CardRecord = CardRecord::new(
    "Unauthorized Exit",
    "8a458474-d721-46d7-b487-6a47dc063cfd",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// MKM 75 — Agency Coroner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENCY_CORONER: CardRecord = CardRecord::new(
    "Agency Coroner",
    "d63f2c23-e877-42e3-9362-5d003a173c6d",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// MKM 76 — Alley Assailant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALLEY_ASSAILANT: CardRecord = CardRecord::new(
    "Alley Assailant",
    "edf238c9-61de-4f3a-b82f-05af46e5e81b",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// MKM 77 — Barbed Servitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_SERVITOR: CardRecord = CardRecord::new(
    "Barbed Servitor",
    "1c34e4ae-9bf3-4098-88f1-267e7d6cfa35",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// MKM 78 — Basilica Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BASILICA_STALKER: CardRecord = CardRecord::new(
    "Basilica Stalker",
    "fdafea8f-283d-4f19-a74a-669bfbdfed98",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// MKM 79 — Case of the Gorgon's Kiss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_GORGON_S_KISS: CardRecord = CardRecord::new(
    "Case of the Gorgon's Kiss",
    "45e4c07a-3205-4193-8163-b0e63e6242a4",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// MKM 79† — Case of the Gorgon's Kiss (alternate printing)
const CASE_OF_THE_GORGON_S_KISS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CASE_OF_THE_GORGON_S_KISS,
    1,
    "0d7ead0e-477b-4b3d-b6bf-b2598ddc6f04",
    "Peter Polach",
);

// MKM 80 — Case of the Stashed Skeleton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_STASHED_SKELETON: CardRecord = CardRecord::new(
    "Case of the Stashed Skeleton",
    "4b120cbe-f0af-46c5-863f-03ecadf0435c",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// MKM 81 — Cerebral Confiscation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEREBRAL_CONFISCATION: CardRecord = CardRecord::new(
    "Cerebral Confiscation",
    "3c3c3d15-b775-44a2-91ea-4abcc3cf2dba",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// MKM 82 — Clandestine Meddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLANDESTINE_MEDDLER: CardRecord = CardRecord::new(
    "Clandestine Meddler",
    "2e069de0-3218-456c-b191-93e755634783",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// MKM 82† — Clandestine Meddler (alternate printing)
const CLANDESTINE_MEDDLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLANDESTINE_MEDDLER,
    1,
    "e1b13406-3b4e-4c61-b19a-c1faf31ed676",
    "Jodie Muir",
);

// MKM 83 — Deadly Cover-Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADLY_COVER_UP: CardRecord = CardRecord::new(
    "Deadly Cover-Up",
    "3876aa0f-b199-43f5-8a91-c2d620b8ef84",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// MKM 84 — Extract a Confession
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTRACT_A_CONFESSION: CardRecord = CardRecord::new(
    "Extract a Confession",
    "256c8b6e-4031-458b-8eb9-bbfe58405a0c",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// MKM 85 — Festerleech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FESTERLEECH: CardRecord = CardRecord::new(
    "Festerleech",
    "83a6fa37-8351-403b-ae05-b67e9bf74bbb",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// MKM 86 — Homicide Investigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOMICIDE_INVESTIGATOR: CardRecord = CardRecord::new(
    "Homicide Investigator",
    "21d6accd-167a-4b21-a488-44d54cdfa608",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// MKM 87 — Hunted Bonebrute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTED_BONEBRUTE: CardRecord = CardRecord::new(
    "Hunted Bonebrute",
    "a4ca0e10-8b7c-4ce2-888b-752fc909757a",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

// MKM 88 — Illicit Masquerade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLICIT_MASQUERADE: CardRecord = CardRecord::new(
    "Illicit Masquerade",
    "2a7a3ec4-afaa-45e1-8cde-f15bf4bd7379",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// MKM 89 — It Doesn't Add Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IT_DOESN_T_ADD_UP: CardRecord = CardRecord::new(
    "It Doesn't Add Up",
    "fa02dbc2-ad01-47fd-b39e-f0a695029f26",
    "Anastasia Ovchinnikova",
    crate::card::CardRules::unsupported(),
);

// MKM 90 — Lead Pipe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEAD_PIPE: CardRecord = CardRecord::new(
    "Lead Pipe",
    "87f69249-c6e4-40c1-9870-b9c45ce24c39",
    "Igor Krstic",
    crate::card::CardRules::unsupported(),
);

// MKM 91 — Leering Onlooker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEERING_ONLOOKER: CardRecord = CardRecord::new(
    "Leering Onlooker",
    "dc687588-9c57-411d-b666-b9699949d48f",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// MKM 92 — Long Goodbye
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONG_GOODBYE: CardRecord = CardRecord::new(
    "Long Goodbye",
    "c3896705-bbd2-4ffb-a590-ee78e0eabdc5",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// MKM 93 — Macabre Reconstruction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MACABRE_RECONSTRUCTION: CardRecord = CardRecord::new(
    "Macabre Reconstruction",
    "abb6184c-e3d0-4275-b25b-95e4a64b26f3",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// MKM 94 — Massacre Girl, Known Killer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASSACRE_GIRL_KNOWN_KILLER: CardRecord = CardRecord::new(
    "Massacre Girl, Known Killer",
    "cb1c8800-9d33-485c-b776-042003b9ea92",
    "Billy Christian",
    crate::card::CardRules::unsupported(),
);

// MKM 95 — Murder (reprint)
const MURDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::MURDER,
    "1ea6438b-0e6c-4d65-8bcd-34a988717c81",
    "Isis",
);

// MKM 96 — Nightdrinker Moroii
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTDRINKER_MOROII: CardRecord = CardRecord::new(
    "Nightdrinker Moroii",
    "ce043cba-aea4-4156-b1d0-545eda06c400",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// MKM 97 — Outrageous Robbery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTRAGEOUS_ROBBERY: CardRecord = CardRecord::new(
    "Outrageous Robbery",
    "b87813fa-ad12-4062-bb9e-436d8418fba5",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// MKM 98 — Persuasive Interrogators
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERSUASIVE_INTERROGATORS: CardRecord = CardRecord::new(
    "Persuasive Interrogators",
    "f0713025-581f-451b-97a3-97d891285dcc",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// MKM 99 — Polygraph Orb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLYGRAPH_ORB: CardRecord = CardRecord::new(
    "Polygraph Orb",
    "a6cc4c6f-4a84-4d42-89fa-7405f7ad6ba0",
    "Jokubas Uogintas",
    crate::card::CardRules::unsupported(),
);

// MKM 100 — Presumed Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRESUMED_DEAD: CardRecord = CardRecord::new(
    "Presumed Dead",
    "4dd64e5c-ea0b-4ea0-aba3-88e7e96ac7ba",
    "Matt Forsyth",
    crate::card::CardRules::unsupported(),
);

// MKM 101 — Repeat Offender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPEAT_OFFENDER: CardRecord = CardRecord::new(
    "Repeat Offender",
    "0c2ca1e7-e0de-4d29-a81b-62185ccd295f",
    "Joshua Cairos",
    crate::card::CardRules::unsupported(),
);

// MKM 102 — Rot Farm Mortipede
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROT_FARM_MORTIPEDE: CardRecord = CardRecord::new(
    "Rot Farm Mortipede",
    "023b0142-663a-47e7-a9f1-0b565a172b60",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// MKM 103 — Slice from the Shadows
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLICE_FROM_THE_SHADOWS: CardRecord = CardRecord::new(
    "Slice from the Shadows",
    "317800b3-6b2d-4de6-8e44-7e54dd623055",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// MKM 104 — Slimy Dualleech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIMY_DUALLEECH: CardRecord = CardRecord::new(
    "Slimy Dualleech",
    "4bc803ad-f8a2-4198-a8a6-8d987b3d00fb",
    "Igor Krstic",
    crate::card::CardRules::unsupported(),
);

// MKM 105 — Snarling Gorehound
pub(in crate::card::sets) static SNARLING_GOREHOUND: CardRecord = CardRecord::new(
    "Snarling Gorehound",
    "93ab3e11-8584-406f-b9ae-9e1df4396cbc",
    "John Tedrick",
    // A one-drop that keeps paying in a deck full of other one-drops, which
    // is exactly the deck that wants a menace body this cheap.
    CardRules::new_creature(mana_cost!("{B}"), &["Dog"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever another creature you control with power 2 or less enters, surveil 1. (Look \
             at the top card of your library. You may put it into your graveyard.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    // "Power 2 or less" has to be a strict comparison because
                    // power only reads upward here.
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// MKM 106 — Soul Enervation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_ENERVATION: CardRecord = CardRecord::new(
    "Soul Enervation",
    "6f22ac67-06ce-47cc-a515-d216d30b9cae",
    "Domenico Cava",
    crate::card::CardRules::unsupported(),
);

// MKM 107 — Toxin Analysis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOXIN_ANALYSIS: CardRecord = CardRecord::new(
    "Toxin Analysis",
    "0eda1aff-c1f4-4171-a800-605396cc8168",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// MKM 108 — Undercity Eliminator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERCITY_ELIMINATOR: CardRecord = CardRecord::new(
    "Undercity Eliminator",
    "a67a4c5e-215b-4f03-87f7-c1af4f9f0a63",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// MKM 109 — Unscrupulous Agent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSCRUPULOUS_AGENT: CardRecord = CardRecord::new(
    "Unscrupulous Agent",
    "37692f9a-3825-43aa-aacb-1bb92cb5bd07",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// MKM 110 — Vein Ripper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEIN_RIPPER: CardRecord = CardRecord::new(
    "Vein Ripper",
    "078933b3-6d82-45f2-94e8-addf54cf1704",
    "Bastien L. Deharme",
    crate::card::CardRules::unsupported(),
);

// MKM 111 — Anzrag's Rampage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANZRAG_S_RAMPAGE: CardRecord = CardRecord::new(
    "Anzrag's Rampage",
    "9dc52b53-3e4f-4d7d-851f-86c6e0ac67b2",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// MKM 112 — Bolrac-Clan Basher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOLRAC_CLAN_BASHER: CardRecord = CardRecord::new(
    "Bolrac-Clan Basher",
    "b87683f7-8a61-4e4a-8b8b-3bf812454096",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// MKM 113 — Case of the Burning Masks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_BURNING_MASKS: CardRecord = CardRecord::new(
    "Case of the Burning Masks",
    "29ee07df-215f-45a6-9a5a-708143d73e45",
    "Bastien L. Deharme",
    crate::card::CardRules::unsupported(),
);

// MKM 114 — Case of the Crimson Pulse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_CRIMSON_PULSE: CardRecord = CardRecord::new(
    "Case of the Crimson Pulse",
    "bb18b1de-bc08-4522-b891-6117a8271534",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// MKM 115 — Caught Red-Handed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAUGHT_RED_HANDED: CardRecord = CardRecord::new(
    "Caught Red-Handed",
    "95bc5f89-2f01-40c4-9883-4c90ab89fcbb",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MKM 116 — The Chase Is On
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_CHASE_IS_ON: CardRecord = CardRecord::new(
    "The Chase Is On",
    "1d54d596-f7aa-4b05-ab13-19b246698c04",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// MKM 117 — Concealed Weapon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONCEALED_WEAPON: CardRecord = CardRecord::new(
    "Concealed Weapon",
    "38e31fa6-a445-47c6-a73f-135087f6d760",
    "Nicholas Elias",
    crate::card::CardRules::unsupported(),
);

// MKM 118 — Connecting the Dots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONNECTING_THE_DOTS: CardRecord = CardRecord::new(
    "Connecting the Dots",
    "8e02731a-8698-4b41-99c3-f0a19fc31430",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// MKM 119 — Convenient Target
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONVENIENT_TARGET: CardRecord = CardRecord::new(
    "Convenient Target",
    "2d2cf2ae-9152-41c4-9dc4-a19da5812869",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// MKM 120 — Cornered Crook
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORNERED_CROOK: CardRecord = CardRecord::new(
    "Cornered Crook",
    "a3aff1ea-1d25-49c3-a2d9-f435124a5969",
    "Gabor Szikszai",
    crate::card::CardRules::unsupported(),
);

// MKM 121 — Crime Novelist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIME_NOVELIST: CardRecord = CardRecord::new(
    "Crime Novelist",
    "14a5cd7c-b0b1-4ffa-a806-bb0e73baffad",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// MKM 122 — Demand Answers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMAND_ANSWERS: CardRecord = CardRecord::new(
    "Demand Answers",
    "eca092fc-7c67-4a73-989e-5297bbaaea76",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// MKM 123 — Expedited Inheritance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPEDITED_INHERITANCE: CardRecord = CardRecord::new(
    "Expedited Inheritance",
    "b65209da-cf48-4d37-b045-7d181070fd05",
    "Micah Epstein",
    crate::card::CardRules::unsupported(),
);

// MKM 124 — Expose the Culprit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPOSE_THE_CULPRIT: CardRecord = CardRecord::new(
    "Expose the Culprit",
    "31aadd3d-5ce1-44ba-ac6d-b192a9ea491b",
    "Ryan Valle",
    crate::card::CardRules::unsupported(),
);

// MKM 125 — Felonious Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FELONIOUS_RAGE: CardRecord = CardRecord::new(
    "Felonious Rage",
    "4538d6a8-a24a-40e3-b894-45a30882c92a",
    "Justine Cruz",
    crate::card::CardRules::unsupported(),
);

// MKM 126 — Frantic Scapegoat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRANTIC_SCAPEGOAT: CardRecord = CardRecord::new(
    "Frantic Scapegoat",
    "eb81e343-7242-44b1-9ce6-1dddd104f764",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// MKM 127 — Fugitive Codebreaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUGITIVE_CODEBREAKER: CardRecord = CardRecord::new(
    "Fugitive Codebreaker",
    "b682bf8a-06dc-4828-bc46-9e1427bf981f",
    "Joseph Weston",
    crate::card::CardRules::unsupported(),
);

// MKM 128 — Galvanize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALVANIZE: CardRecord = CardRecord::new(
    "Galvanize",
    "64ed3bfa-3294-45dd-825e-3afc2580f0d4",
    "Matt Forsyth",
    crate::card::CardRules::unsupported(),
);

// MKM 129 — Gearbane Orangutan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEARBANE_ORANGUTAN: CardRecord = CardRecord::new(
    "Gearbane Orangutan",
    "6900a344-a155-4ee1-a3ac-d6c28e024270",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// MKM 130 — Goblin Maskmaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MASKMAKER: CardRecord = CardRecord::new(
    "Goblin Maskmaker",
    "6154a991-c602-4fca-91a3-3830060da60e",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// MKM 131 — Harried Dronesmith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARRIED_DRONESMITH: CardRecord = CardRecord::new(
    "Harried Dronesmith",
    "36cb25ee-3c84-40a8-ba45-7e44893deecf",
    "Lindsey Look",
    crate::card::CardRules::unsupported(),
);

// MKM 132 — Incinerator of the Guilty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INCINERATOR_OF_THE_GUILTY: CardRecord = CardRecord::new(
    "Incinerator of the Guilty",
    "0c6aca64-a554-45c1-9f23-4f7878abeda5",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// MKM 133 — Innocent Bystander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INNOCENT_BYSTANDER: CardRecord = CardRecord::new(
    "Innocent Bystander",
    "085f4595-4ae5-428e-a934-e918774df6fd",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// MKM 134 — Knife
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIFE: CardRecord = CardRecord::new(
    "Knife",
    "b6883788-e1ee-4ddd-add2-24d6bc367717",
    "Tony Foti",
    crate::card::CardRules::unsupported(),
);

// MKM 135 — Krenko, Baron of Tin Street
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRENKO_BARON_OF_TIN_STREET: CardRecord = CardRecord::new(
    "Krenko, Baron of Tin Street",
    "5524b712-c67d-4d2e-9344-9e85a6ce3227",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// MKM 136 — Krenko's Buzzcrusher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRENKO_S_BUZZCRUSHER: CardRecord = CardRecord::new(
    "Krenko's Buzzcrusher",
    "0edcda2a-071b-40c5-9fb3-8a4ff87ca00e",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// MKM 137 — Lamplight Phoenix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAMPLIGHT_PHOENIX: CardRecord = CardRecord::new(
    "Lamplight Phoenix",
    "2faa0e56-527c-4be5-b8c9-b10ccde275f5",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// MKM 138 — Offender at Large
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OFFENDER_AT_LARGE: CardRecord = CardRecord::new(
    "Offender at Large",
    "f096ff4a-85f4-46f1-9478-e8921f21309d",
    "Mike Bierek",
    crate::card::CardRules::unsupported(),
);

// MKM 139 — Person of Interest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERSON_OF_INTEREST: CardRecord = CardRecord::new(
    "Person of Interest",
    "7d56ebff-67c8-4bc7-a533-ddde4ce0c2af",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// MKM 140 — Pyrotechnic Performer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYROTECHNIC_PERFORMER: CardRecord = CardRecord::new(
    "Pyrotechnic Performer",
    "0fa5671b-2651-4944-a50a-c768ec70229e",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// MKM 141 — Reckless Detective
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_DETECTIVE: CardRecord = CardRecord::new(
    "Reckless Detective",
    "18da1a1d-e6ba-47e5-a545-0bacd427b782",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// MKM 142 — Red Herring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RED_HERRING: CardRecord = CardRecord::new(
    "Red Herring",
    "6c137a44-9ab6-4e59-8324-34d9dca8f5a6",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// MKM 143 — Rubblebelt Braggart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUBBLEBELT_BRAGGART: CardRecord = CardRecord::new(
    "Rubblebelt Braggart",
    "f90f8691-210a-4bf0-9fc2-fb2efcf057fb",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// MKM 144 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::SHOCK,
    "298747bb-eb40-4b58-bb22-4ac2bc1d795c",
    "Eric Wilkerson",
);

// MKM 145 — Suspicious Detonation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSPICIOUS_DETONATION: CardRecord = CardRecord::new(
    "Suspicious Detonation",
    "6e280482-ed7e-4011-899e-096ff7bd4c41",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// MKM 146 — Torch the Witness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORCH_THE_WITNESS: CardRecord = CardRecord::new(
    "Torch the Witness",
    "22bbf709-d8e9-4e3b-8ec8-206f1b2162b3",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// MKM 146† — Torch the Witness (alternate printing)
const TORCH_THE_WITNESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TORCH_THE_WITNESS,
    1,
    "7bcc6106-71ad-40e4-8e1f-ddbfd655612b",
    "Andrew Mar",
);

// MKM 147 — Vengeful Tracker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEFUL_TRACKER: CardRecord = CardRecord::new(
    "Vengeful Tracker",
    "a247c9a0-0c65-47bc-92fd-bebe95cd35a3",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// MKM 148 — Aftermath Analyst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AFTERMATH_ANALYST: CardRecord = CardRecord::new(
    "Aftermath Analyst",
    "1c1aa6f8-2d34-4f4b-9184-0eab2e4745f7",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// MKM 149 — Airtight Alibi
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AIRTIGHT_ALIBI: CardRecord = CardRecord::new(
    "Airtight Alibi",
    "bffbbe21-0a1d-48b9-903e-81c109aa11de",
    "Jeremy Wilson",
    crate::card::CardRules::unsupported(),
);

// MKM 150 — Analyze the Pollen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANALYZE_THE_POLLEN: CardRecord = CardRecord::new(
    "Analyze the Pollen",
    "5563967f-09fd-4ccf-8892-4dd0c2544c98",
    "Anna Christenson",
    crate::card::CardRules::unsupported(),
);

// MKM 151 — Archdruid's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHDRUID_S_CHARM: CardRecord = CardRecord::new(
    "Archdruid's Charm",
    "5caae5ae-845f-42c2-b1ae-956df2739433",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// MKM 152 — Audience with Trostani
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUDIENCE_WITH_TROSTANI: CardRecord = CardRecord::new(
    "Audience with Trostani",
    "a8e23d15-33af-4fd8-964b-8ca4efdebc37",
    "Ben Hill",
    crate::card::CardRules::unsupported(),
);

// MKM 153 — Axebane Ferox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AXEBANE_FEROX: CardRecord = CardRecord::new(
    "Axebane Ferox",
    "610a0de4-a4f7-446b-8477-00c917cb4789",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

// MKM 154 — Bite Down on Crime
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BITE_DOWN_ON_CRIME: CardRecord = CardRecord::new(
    "Bite Down on Crime",
    "29bbfe93-8225-444c-835b-33ffa006ef66",
    "Mike Bierek",
    crate::card::CardRules::unsupported(),
);

// MKM 155 — Case of the Locked Hothouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_LOCKED_HOTHOUSE: CardRecord = CardRecord::new(
    "Case of the Locked Hothouse",
    "0929a1bd-e35c-4ca5-8c8c-dd304cf4b830",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// MKM 156 — Case of the Trampled Garden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASE_OF_THE_TRAMPLED_GARDEN: CardRecord = CardRecord::new(
    "Case of the Trampled Garden",
    "9e80f5c7-ae29-473c-ac64-04bcbc629385",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

// MKM 157 — Chalk Outline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHALK_OUTLINE: CardRecord = CardRecord::new(
    "Chalk Outline",
    "b3ff56c1-4153-4e15-9ac6-06d93fa2ae50",
    "Julia Griffin",
    crate::card::CardRules::unsupported(),
);

// MKM 158 — Culvert Ambusher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CULVERT_AMBUSHER: CardRecord = CardRecord::new(
    "Culvert Ambusher",
    "2ccdc58b-1e7e-402c-88f9-c789ff1dae31",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// MKM 159 — Fanatical Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FANATICAL_STRENGTH: CardRecord = CardRecord::new(
    "Fanatical Strength",
    "4941fa74-c7b9-4468-8080-de8057d3d27b",
    "Durion",
    crate::card::CardRules::unsupported(),
);

// MKM 160 — Flourishing Bloom-Kin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOURISHING_BLOOM_KIN: CardRecord = CardRecord::new(
    "Flourishing Bloom-Kin",
    "5ddcb31e-9301-44f1-b138-0573fbf56a47",
    "Ben Hill",
    crate::card::CardRules::unsupported(),
);

// MKM 161 — Get a Leg Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GET_A_LEG_UP: CardRecord = CardRecord::new(
    "Get a Leg Up",
    "34e61e0f-d0f3-492a-92f1-36f72a91583a",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// MKM 162 — Glint Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLINT_WEAVER: CardRecord = CardRecord::new(
    "Glint Weaver",
    "c8eff4d0-67ad-4900-b33d-605659b59161",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// MKM 163 — Greenbelt Radical
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREENBELT_RADICAL: CardRecord = CardRecord::new(
    "Greenbelt Radical",
    "88e62346-cc62-4938-970c-b56beeb79fa6",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// MKM 164 — Hard-Hitting Question
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARD_HITTING_QUESTION: CardRecord = CardRecord::new(
    "Hard-Hitting Question",
    "8ad807c2-14a7-4464-bf57-c323fb3c0bd0",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// MKM 165 — Hedge Whisperer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEDGE_WHISPERER: CardRecord = CardRecord::new(
    "Hedge Whisperer",
    "4627adcd-ace7-4777-a7e6-fc80ac6b9dfe",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// MKM 166 — Hide in Plain Sight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDE_IN_PLAIN_SIGHT: CardRecord = CardRecord::new(
    "Hide in Plain Sight",
    "d87c1ed8-c644-4ad5-9a21-c7bd9a7e8d20",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// MKM 167 — A Killer Among Us
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static A_KILLER_AMONG_US: CardRecord = CardRecord::new(
    "A Killer Among Us",
    "2c1392c5-91a5-4e6e-803d-ed032e4d594b",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// MKM 168 — Loxodon Eavesdropper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOXODON_EAVESDROPPER: CardRecord = CardRecord::new(
    "Loxodon Eavesdropper",
    "bbbf8c3a-6c74-42fd-bb8d-61e3f0a77848",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// MKM 169 — Nervous Gardener
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NERVOUS_GARDENER: CardRecord = CardRecord::new(
    "Nervous Gardener",
    "93b747c7-b342-47f8-a190-16c393b20607",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 170 — Pick Your Poison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PICK_YOUR_POISON: CardRecord = CardRecord::new(
    "Pick Your Poison",
    "f58cfb23-4d99-4133-bf4b-d7e7c7d17cea",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// MKM 171 — Pompous Gadabout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POMPOUS_GADABOUT: CardRecord = CardRecord::new(
    "Pompous Gadabout",
    "6d803b93-c1df-4a02-9dbb-d347c841d4d7",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// MKM 172 — The Pride of Hull Clade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_PRIDE_OF_HULL_CLADE: CardRecord = CardRecord::new(
    "The Pride of Hull Clade",
    "edb40ab9-e552-4eb5-9c35-09094136dd4f",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// MKM 173 — Rope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROPE: CardRecord = CardRecord::new(
    "Rope",
    "6881946c-5036-4d9f-926f-932c9a592aff",
    "Matt Forsyth",
    crate::card::CardRules::unsupported(),
);

// MKM 174 — Rubblebelt Maverick
pub(in crate::card::sets) static RUBBLEBELT_MAVERICK: CardRecord = CardRecord::new(
    "Rubblebelt Maverick",
    "81c7ff67-b9e1-4d2e-b1ae-da9b946da00b",
    "Carissa Susilo",
    // A one-drop that fills the graveyard on the way in and cashes itself
    // out of it later, so trading it away costs the deck almost nothing.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Detective"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two cards of your library, \
             then put any number of them into your graveyard and the rest on top of your library \
             in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
        AbilityDef::activated_with_targets(
            "{G}, Exile this card from your graveyard: Put a +1/+1 counter on target creature. \
             Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::ExileSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        // Activated from the graveyard rather than the battlefield, and at
        // sorcery speed.
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// MKM 175 — Sample Collector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMPLE_COLLECTOR: CardRecord = CardRecord::new(
    "Sample Collector",
    "76f7480c-82cc-4ddd-b619-c1a609c29a13",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 176 — Sharp-Eyed Rookie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHARP_EYED_ROOKIE: CardRecord = CardRecord::new(
    "Sharp-Eyed Rookie",
    "3d5d4788-a970-4e09-89a1-740eca9331d9",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// MKM 177 — Slime Against Humanity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIME_AGAINST_HUMANITY: CardRecord = CardRecord::new(
    "Slime Against Humanity",
    "1eb21318-d32e-4724-8908-c0d7613de2f4",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// MKM 178 — They Went This Way
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THEY_WENT_THIS_WAY: CardRecord = CardRecord::new(
    "They Went This Way",
    "f4a31d4a-34bc-46b4-b20f-a5460191b35d",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// MKM 179 — Topiary Panther
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOPIARY_PANTHER: CardRecord = CardRecord::new(
    "Topiary Panther",
    "03d0365e-6dee-4236-a997-6761e3cde90d",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// MKM 180 — Tunnel Tipster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUNNEL_TIPSTER: CardRecord = CardRecord::new(
    "Tunnel Tipster",
    "3e29b890-35b9-4e2a-9b4c-9417ca7db31d",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// MKM 181 — Undergrowth Recon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERGROWTH_RECON: CardRecord = CardRecord::new(
    "Undergrowth Recon",
    "4b8a22b8-368f-41e4-8d49-432c6c2ed11e",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// MKM 182 — Vengeful Creeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEFUL_CREEPER: CardRecord = CardRecord::new(
    "Vengeful Creeper",
    "7a914416-effd-4eda-b609-2773c53a08ec",
    "Maria Poliakova",
    crate::card::CardRules::unsupported(),
);

// MKM 183 — Vitu-Ghazi Inspector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VITU_GHAZI_INSPECTOR: CardRecord = CardRecord::new(
    "Vitu-Ghazi Inspector",
    "664d15d7-2724-4a9b-b5a7-8042d4b7da7b",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 184 — Agrus Kos, Spirit of Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGRUS_KOS_SPIRIT_OF_JUSTICE: CardRecord = CardRecord::new(
    "Agrus Kos, Spirit of Justice",
    "58aeac7c-1275-49d4-9915-7604ae4bfdff",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// MKM 185 — Alquist Proft, Master Sleuth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALQUIST_PROFT_MASTER_SLEUTH: CardRecord = CardRecord::new(
    "Alquist Proft, Master Sleuth",
    "41129b44-4fa7-473b-b2b7-48c6a58be03c",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// MKM 186 — Anzrag, the Quake-Mole
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANZRAG_THE_QUAKE_MOLE: CardRecord = CardRecord::new(
    "Anzrag, the Quake-Mole",
    "70e9d8b8-4b32-4414-b32f-1f47523239c5",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// MKM 187 — Assassin's Trophy (reprint)
const ASSASSIN_S_TROPHY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::ASSASSIN_S_TROPHY,
    "ed6c7d29-71b4-4134-b591-5598f479d592",
    "Dmitry Burmak",
);

// MKM 188 — Aurelia, the Law Above
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURELIA_THE_LAW_ABOVE: CardRecord = CardRecord::new(
    "Aurelia, the Law Above",
    "8f80c6e7-e9f9-4ca6-87f7-a52c96079e4a",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// MKM 189 — Blood Spatter Analysis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_SPATTER_ANALYSIS: CardRecord = CardRecord::new(
    "Blood Spatter Analysis",
    "2b80feb8-5cc8-4e91-ac22-a733305a67de",
    "Jokubas Uogintas",
    crate::card::CardRules::unsupported(),
);

// MKM 190 — Break Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAK_OUT: CardRecord = CardRecord::new(
    "Break Out",
    "8c628476-0987-47d4-8d2a-cfc3977b2357",
    "Daniel Correia",
    crate::card::CardRules::unsupported(),
);

// MKM 190† — Break Out (alternate printing)
const BREAK_OUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BREAK_OUT,
    1,
    "60b50a8f-2788-41df-81ca-61194960b730",
    "Daniel Correia",
);

// MKM 191 — Buried in the Garden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURIED_IN_THE_GARDEN: CardRecord = CardRecord::new(
    "Buried in the Garden",
    "7e144609-e1f6-4bdc-8d14-b735ef4140d3",
    "Tom Babbey",
    crate::card::CardRules::unsupported(),
);

// MKM 192 — Coerced to Kill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COERCED_TO_KILL: CardRecord = CardRecord::new(
    "Coerced to Kill",
    "2dc9f352-5076-4b5f-9815-cf47abb63d5b",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// MKM 193 — Crowd-Control Warden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWD_CONTROL_WARDEN: CardRecord = CardRecord::new(
    "Crowd-Control Warden",
    "cdf0578f-4966-4ecd-81e1-83ae13126f13",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// MKM 194 — Curious Cadaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURIOUS_CADAVER: CardRecord = CardRecord::new(
    "Curious Cadaver",
    "2893aef8-835d-4935-b532-d8670585e489",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// MKM 195 — Deadly Complication
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADLY_COMPLICATION: CardRecord = CardRecord::new(
    "Deadly Complication",
    "7c68981c-037c-42e7-9b7f-6f07edab5f2e",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// MKM 196 — Detective's Satchel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DETECTIVE_S_SATCHEL: CardRecord = CardRecord::new(
    "Detective's Satchel",
    "2c05bf2d-7d4f-4717-b1ea-ec4284854f4f",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// MKM 197 — Dog Walker
// Audit: unsupported — Needs a turned-face-up trigger. Disguise itself has a cast kind and face-down characteristics, but no event fires when a permanent is turned face up, which is when this card does everything it does.
pub(in crate::card::sets) static DOG_WALKER: CardRecord = CardRecord::new(
    "Dog Walker",
    "a6e0adb7-a030-4dcc-9284-cd91c7598a22",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// MKM 198 — Doppelgang
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOPPELGANG: CardRecord = CardRecord::new(
    "Doppelgang",
    "a2daec58-78ed-4da5-b3b0-b04f12b0acbd",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// MKM 199 — Drag the Canal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAG_THE_CANAL: CardRecord = CardRecord::new(
    "Drag the Canal",
    "508d7096-2be3-4d4b-a55c-d4dbd3c9019c",
    "Josh Hass",
    crate::card::CardRules::unsupported(),
);

// MKM 200 — Etrata, Deadly Fugitive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ETRATA_DEADLY_FUGITIVE: CardRecord = CardRecord::new(
    "Etrata, Deadly Fugitive",
    "4410db5a-62af-43ac-979d-88a7c975f7bd",
    "Livia Prima",
    crate::card::CardRules::unsupported(),
);

// MKM 201 — Evidence Examiner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVIDENCE_EXAMINER: CardRecord = CardRecord::new(
    "Evidence Examiner",
    "f53a6ee7-86e1-4d2d-994c-214e0ec08dad",
    "Paolo Puggioni",
    crate::card::CardRules::unsupported(),
);

// MKM 202 — Ezrim, Agency Chief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EZRIM_AGENCY_CHIEF: CardRecord = CardRecord::new(
    "Ezrim, Agency Chief",
    "9554d5f2-7a33-4734-8cf3-dfae2ccc3596",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// MKM 203 — Faerie Snoop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_SNOOP: CardRecord = CardRecord::new(
    "Faerie Snoop",
    "20267dab-8898-4b44-8ef4-8a239967662c",
    "Dallas Williams",
    crate::card::CardRules::unsupported(),
);

// MKM 204 — Gadget Technician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GADGET_TECHNICIAN: CardRecord = CardRecord::new(
    "Gadget Technician",
    "3b489a54-ee43-4962-be7b-16e0e28800e0",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// MKM 205 — Gleaming Geardrake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLEAMING_GEARDRAKE: CardRecord = CardRecord::new(
    "Gleaming Geardrake",
    "cabb5875-42ff-4e3a-a32e-aab392fccff8",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// MKM 206 — Granite Witness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRANITE_WITNESS: CardRecord = CardRecord::new(
    "Granite Witness",
    "daee9d98-f8c6-4980-8f23-c6c636b69430",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// MKM 207 — Ill-Timed Explosion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILL_TIMED_EXPLOSION: CardRecord = CardRecord::new(
    "Ill-Timed Explosion",
    "0b5cdb01-eaa4-4a0a-b42a-332bcf4d6fff",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// MKM 208 — Insidious Roots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIDIOUS_ROOTS: CardRecord = CardRecord::new(
    "Insidious Roots",
    "0bb91a22-2040-4a37-85f8-5f22de8c5907",
    "Jeremy Wilson",
    crate::card::CardRules::unsupported(),
);

// MKM 209 — Izoni, Center of the Web
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IZONI_CENTER_OF_THE_WEB: CardRecord = CardRecord::new(
    "Izoni, Center of the Web",
    "70ea66cd-587a-4ca9-9ca8-d7d2046bfbed",
    "Justine Cruz",
    crate::card::CardRules::unsupported(),
);

// MKM 210 — Judith, Carnage Connoisseur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUDITH_CARNAGE_CONNOISSEUR: CardRecord = CardRecord::new(
    "Judith, Carnage Connoisseur",
    "3eaa19ce-cace-499e-8b23-ef9e56b23700",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// MKM 211 — Kaya, Spirits' Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAYA_SPIRITS_JUSTICE: CardRecord = CardRecord::new(
    "Kaya, Spirits' Justice",
    "a2827593-4951-4ba7-b73e-c27de56f2606",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// MKM 212 — Kellan, Inquisitive Prodigy // Tail the Suspect
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELLAN_INQUISITIVE_PRODIGY: CardRecord = CardRecord::new(
    "Kellan, Inquisitive Prodigy // Tail the Suspect",
    "c49690c7-c282-4eb4-8da3-5e0c46a80fc4",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// MKM 213 — Kraul Whipcracker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAUL_WHIPCRACKER: CardRecord = CardRecord::new(
    "Kraul Whipcracker",
    "0c08ed44-2c13-4029-af2e-68585a76bb03",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// MKM 213† — Kraul Whipcracker (alternate printing)
const KRAUL_WHIPCRACKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRAUL_WHIPCRACKER,
    1,
    "1ceb4988-0a79-4757-9fcc-381ee8643d0f",
    "Filip Burburan",
);

// MKM 214 — Kylox, Visionary Inventor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYLOX_VISIONARY_INVENTOR: CardRecord = CardRecord::new(
    "Kylox, Visionary Inventor",
    "00faa272-91ad-407b-9175-8fa1d02585b8",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// MKM 215 — Kylox's Voltstrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYLOX_S_VOLTSTRIDER: CardRecord = CardRecord::new(
    "Kylox's Voltstrider",
    "86a8a1af-b1cf-47fc-ab42-7efa07a1c95b",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// MKM 216 — Lazav, Wearer of Faces
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAZAV_WEARER_OF_FACES: CardRecord = CardRecord::new(
    "Lazav, Wearer of Faces",
    "cc264d1d-689d-41ac-b624-3fc7bb890e58",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// MKM 217 — Leyline of the Guildpact
pub(in crate::card::sets) static LEYLINE_OF_THE_GUILDPACT: CardRecord = CardRecord::new(
    "Leyline of the Guildpact",
    "bf6e59be-f959-4f4a-8c2d-b7c441e88135",
    "Daarken",
    CardRules::new_enchantment(mana_cost!("{G/W}{G/U}{B/G}{R/G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "Each nonland permanent you control is all colors.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_colors(ColorSet::from_colors(&[
                    ManaColor::White,
                    ManaColor::Blue,
                    ManaColor::Black,
                    ManaColor::Red,
                    ManaColor::Green,
                ])),
            },
        ),
        AbilityDef::static_ability(
            "Lands you control are every basic land type in addition to their other types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_basic_land_types(&[
                    BasicLandType::Plains,
                    BasicLandType::Island,
                    BasicLandType::Swamp,
                    BasicLandType::Mountain,
                    BasicLandType::Forest,
                ]),
            },
        ),
    ]),
);

// MKM 218 — Lightning Helix (reprint)
const LIGHTNING_HELIX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::LIGHTNING_HELIX,
    "4101e3fe-b0e7-4f0f-b9ac-9b61a4d628b3",
    "Eli Minaya",
);

// MKM 219 — Meddling Youths
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEDDLING_YOUTHS: CardRecord = CardRecord::new(
    "Meddling Youths",
    "af12417c-b082-4379-a850-c72e2652c6fb",
    "Matt Forsyth",
    crate::card::CardRules::unsupported(),
);

// MKM 220 — Niv-Mizzet, Guildpact
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIV_MIZZET_GUILDPACT: CardRecord = CardRecord::new(
    "Niv-Mizzet, Guildpact",
    "32a8fda6-8614-45cd-879c-0cb7fa29647e",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// MKM 221 — No More Lies
pub(in crate::card::sets) static NO_MORE_LIES: CardRecord = CardRecord::new(
    "No More Lies",
    "1e0c695d-62f9-4805-9e2f-7032e8464136",
    "Liiga Smilshkalne",
    // Mana Leak that eats what it catches: the exile is what makes it worth
    // a second color, since nothing gets the spell back afterwards.
    CardRules::new_instant(mana_cost!("{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {3}. If that spell is countered this \
         way, exile it instead of putting it into its owner's graveyard.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_to_exile_unless_paid(&[CostDef::GenericMana(
            ValueDef::Constant(3),
        )]),
    )),
);

// MKM 222 — Officious Interrogation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OFFICIOUS_INTERROGATION: CardRecord = CardRecord::new(
    "Officious Interrogation",
    "a433ca4c-82d0-4e49-bc8e-98e18dd174e9",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MKM 223 — Private Eye
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIVATE_EYE: CardRecord = CardRecord::new(
    "Private Eye",
    "b6a50807-058e-45dc-847c-8ffd13b1bd48",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// MKM 224 — Rakdos, Patron of Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKDOS_PATRON_OF_CHAOS: CardRecord = CardRecord::new(
    "Rakdos, Patron of Chaos",
    "cc6fd2d5-8eb2-4265-a1bf-d4ae635285af",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// MKM 225 — Rakish Scoundrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKISH_SCOUNDREL: CardRecord = CardRecord::new(
    "Rakish Scoundrel",
    "6aaa8c6b-7ef7-45db-99c9-4a6e7f177b94",
    "Ina Wong",
    crate::card::CardRules::unsupported(),
);

// MKM 226 — Relive the Past
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELIVE_THE_PAST: CardRecord = CardRecord::new(
    "Relive the Past",
    "20948cd2-e40c-4648-832f-ab0f1cc21610",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// MKM 227 — Repulsive Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPULSIVE_MUTATION: CardRecord = CardRecord::new(
    "Repulsive Mutation",
    "71701c28-f113-4d38-8fd3-a19cd9749661",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// MKM 228 — Riftburst Hellion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIFTBURST_HELLION: CardRecord = CardRecord::new(
    "Riftburst Hellion",
    "9fae9044-a859-434d-8dc6-4f9d455ca5e1",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// MKM 229 — Rune-Brand Juggler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_BRAND_JUGGLER: CardRecord = CardRecord::new(
    "Rune-Brand Juggler",
    "5288cf17-9d79-4d35-85f1-bf4d0a73494b",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// MKM 230 — Sanguine Savior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANGUINE_SAVIOR: CardRecord = CardRecord::new(
    "Sanguine Savior",
    "9cba5503-ba99-43d8-8062-66d905e0d86b",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// MKM 231 — Shady Informant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADY_INFORMANT: CardRecord = CardRecord::new(
    "Shady Informant",
    "0de36e63-8190-415f-b65b-bae1e595845d",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// MKM 232 — Soul Search
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_SEARCH: CardRecord = CardRecord::new(
    "Soul Search",
    "0f852937-381d-4445-99d3-2ecb8af6bb6a",
    "A. M. Sartor",
    crate::card::CardRules::unsupported(),
);

// MKM 233 — Sumala Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUMALA_SENTRY: CardRecord = CardRecord::new(
    "Sumala Sentry",
    "3b9d4691-59d1-4e97-9b5d-8017788fbcb3",
    "Nicholas Elias",
    crate::card::CardRules::unsupported(),
);

// MKM 234 — Teysa, Opulent Oligarch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEYSA_OPULENT_OLIGARCH: CardRecord = CardRecord::new(
    "Teysa, Opulent Oligarch",
    "9b5a13dd-c2fd-432e-bc49-cc62d94d62a0",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// MKM 235 — Tin Street Gossip
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIN_STREET_GOSSIP: CardRecord = CardRecord::new(
    "Tin Street Gossip",
    "4094b13f-28d4-48b6-8cce-3c44656745b7",
    "Tony Foti",
    crate::card::CardRules::unsupported(),
);

// MKM 236 — Tolsimir, Midnight's Light
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOLSIMIR_MIDNIGHT_S_LIGHT: CardRecord = CardRecord::new(
    "Tolsimir, Midnight's Light",
    "08d22402-c41d-43d7-be1f-42be1e300726",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// MKM 237 — Treacherous Greed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREACHEROUS_GREED: CardRecord = CardRecord::new(
    "Treacherous Greed",
    "e4b9260b-0993-42c5-9bcf-87ab394d51db",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// MKM 238 — Trostani, Three Whispers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROSTANI_THREE_WHISPERS: CardRecord = CardRecord::new(
    "Trostani, Three Whispers",
    "329f3aca-6db7-41d3-95b2-c479d14b7fa3",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// MKM 239 — Undercover Crocodelf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERCOVER_CROCODELF: CardRecord = CardRecord::new(
    "Undercover Crocodelf",
    "5bc669c8-6f39-4d52-82d3-a4005d41c8a5",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// MKM 239† — Undercover Crocodelf (alternate printing)
const UNDERCOVER_CROCODELF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNDERCOVER_CROCODELF,
    1,
    "1163c6f2-dca1-4d6f-8f0d-7c3c55c6f75e",
    "Nicholas Gregory",
);

// MKM 240 — Urgent Necropsy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URGENT_NECROPSY: CardRecord = CardRecord::new(
    "Urgent Necropsy",
    "d2ac346a-fc46-4023-aa60-4d55170697dc",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// MKM 241 — Vannifar, Evolved Enigma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VANNIFAR_EVOLVED_ENIGMA: CardRecord = CardRecord::new(
    "Vannifar, Evolved Enigma",
    "f6d381eb-6cb6-4505-aebe-995c1ddc8527",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// MKM 242 — Warleader's Call
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARLEADER_S_CALL: CardRecord = CardRecord::new(
    "Warleader's Call",
    "b3e8f8bd-1c8b-4a7c-96c4-57a247ce9ccc",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// MKM 243 — Wispdrinker Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WISPDRINKER_VAMPIRE: CardRecord = CardRecord::new(
    "Wispdrinker Vampire",
    "a3490c05-d12c-4483-acbc-1b4ae68877f0",
    "Zara Alfonso",
    crate::card::CardRules::unsupported(),
);

// MKM 244 — Worldsoul's Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDSOUL_S_RAGE: CardRecord = CardRecord::new(
    "Worldsoul's Rage",
    "fc3340bd-1d8c-4c21-a59d-e092fcbe02e3",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// MKM 245 — Yarus, Roar of the Old Gods
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YARUS_ROAR_OF_THE_OLD_GODS: CardRecord = CardRecord::new(
    "Yarus, Roar of the Old Gods",
    "326845a7-7502-4dc3-8f3e-867d6c84e931",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// MKM 246 — Cease // Desist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEASE: CardRecord = CardRecord::new(
    "Cease // Desist",
    "cb59130a-a134-4383-b983-e4b526d11fb4",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// MKM 247 — Flotsam // Jetsam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOTSAM: CardRecord = CardRecord::new(
    "Flotsam // Jetsam",
    "c1500cbf-5619-465e-a97b-75e676ce789b",
    "Anastasia Ovchinnikova",
    crate::card::CardRules::unsupported(),
);

// MKM 248 — Fuss // Bother
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUSS: CardRecord = CardRecord::new(
    "Fuss // Bother",
    "269a031e-0b89-40e1-b11b-ae870d72161c",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// MKM 249 — Hustle // Bustle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUSTLE: CardRecord = CardRecord::new(
    "Hustle // Bustle",
    "5f664827-e22e-43af-82f1-861b3c7607f1",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// MKM 250 — Push // Pull
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUSH: CardRecord = CardRecord::new(
    "Push // Pull",
    "85835473-b9b6-4f4a-bb93-fef93d5ec57b",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// MKM 251 — Cryptex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPTEX: CardRecord = CardRecord::new(
    "Cryptex",
    "f92a2563-6cfb-4d12-9513-b44d1a7a20ab",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

// MKM 252 — Gravestone Strider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVESTONE_STRIDER: CardRecord = CardRecord::new(
    "Gravestone Strider",
    "1f952d8d-c089-432c-822a-8ef1e605ae38",
    "Tom Babbey",
    crate::card::CardRules::unsupported(),
);

// MKM 253 — Lumbering Laundry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMBERING_LAUNDRY: CardRecord = CardRecord::new(
    "Lumbering Laundry",
    "080ad039-1669-4735-9864-76f4c61fc59e",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// MKM 254 — Magnetic Snuffler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNETIC_SNUFFLER: CardRecord = CardRecord::new(
    "Magnetic Snuffler",
    "70476534-2fc7-4872-a009-3380dd5ce2ab",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// MKM 255 — Magnifying Glass (reprint)
const MAGNIFYING_GLASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_soi::MAGNIFYING_GLASS,
    "c606a71f-8fb0-486b-955f-727ebf436946",
    "Paolo Puggioni",
);

// MKM 256 — Sanitation Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANITATION_AUTOMATON: CardRecord = CardRecord::new(
    "Sanitation Automaton",
    "52608ba4-c47d-44e4-b624-dee2a3a42ae2",
    "Mike Burns",
    crate::card::CardRules::unsupported(),
);

// MKM 257 — Thinking Cap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THINKING_CAP: CardRecord = CardRecord::new(
    "Thinking Cap",
    "6d2565e1-dd7b-462b-8270-a17913277793",
    "Tony Foti",
    crate::card::CardRules::unsupported(),
);

// MKM 258 — Branch of Vitu-Ghazi
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRANCH_OF_VITU_GHAZI: CardRecord = CardRecord::new(
    "Branch of Vitu-Ghazi",
    "73a8169f-b858-47a5-9c76-2e7c50ad4ecd",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// MKM 259 — Commercial District
pub(in crate::card::sets) static COMMERCIAL_DISTRICT: CardRecord = CardRecord::new(
    "Commercial District",
    "bf220c06-3cce-4bdd-aa58-83940c223e9c",
    "Julian Kok Joon Wen",
    // The red-green half, which wants the graveyard less than the others and
    // plays it anyway because a tapped dual is what the mana costs.
    surveil_land(&["Mountain", "Forest"]),
);

// MKM 260 — Elegant Parlor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEGANT_PARLOR: CardRecord = CardRecord::new(
    "Elegant Parlor",
    "72c6d541-e2cb-4d6e-acac-90a8f53b7006",
    "Kamila Szutenberg",
    crate::card::CardRules::unsupported(),
);

// MKM 261 — Escape Tunnel
pub(in crate::card::sets) static ESCAPE_TUNNEL: CardRecord = CardRecord::new(
    "Escape Tunnel",
    "93ddde4f-d35e-4128-8f43-d0eadbd715de",
    "Carlos Palma Cruchaga",
// A land that taps for nothing: both halves spend the land itself, so
    // playing it is a decision about which one the deck wants later.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated(
            "{T}, Sacrifice this land: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                // A qualified library search may legally fail to find.
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Target creature with power 2 or less can't be blocked this turn.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    // The predicate reads power only upwards, so the cap is
                    // the complement of three or more.
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MKM 262 — Hedge Maze
pub(in crate::card::sets) static HEDGE_MAZE: CardRecord = CardRecord::new(
    "Hedge Maze",
    "5260f8ae-805b-4eae-badf-62de0f768867",
    "Andrew Mar",
    // The green-blue half of the cycle, and the one whose deck is usually
    // happiest to see the surveil: the graveyard is where half its cards
    // want to be anyway.
    surveil_land(&["Forest", "Island"]),
);

// MKM 263 — Lush Portico
pub(in crate::card::sets) static LUSH_PORTICO: CardRecord = CardRecord::new(
    "Lush Portico",
    "c17816e8-28b1-4295-a637-efb0e5c18873",
    "Kamila Szutenberg",
    // The green-white half of the cycle, which the decks that want it are
    // playing for the fixing rather than for the graveyard.
    surveil_land(&["Forest", "Plains"]),
);

// MKM 264 — Meticulous Archive
pub(in crate::card::sets) static METICULOUS_ARCHIVE: CardRecord = CardRecord::new(
    "Meticulous Archive",
    "652236c2-84ef-45e4-b5fc-ed6170bc3d6c",
    "Sam Burley",
    // The white-blue half, which wants the graveyard least of the cycle and
    // is played for the dual land the tempo decks cannot otherwise have.
    surveil_land(&["Plains", "Island"]),
);

// MKM 265 — Public Thoroughfare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUBLIC_THOROUGHFARE: CardRecord = CardRecord::new(
    "Public Thoroughfare",
    "1f8b915f-3e82-4b05-b963-01ebff7a8f7b",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// MKM 266 — Raucous Theater (alternate printing)
const RAUCOUS_THEATER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAUCOUS_THEATER,
    1,
    "b598c93e-dae1-4d71-a9e4-917abf76d2d0",
    "Sergey Glushakov",
);

// MKM 267 — Scene of the Crime
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCENE_OF_THE_CRIME: CardRecord = CardRecord::new(
    "Scene of the Crime",
    "de039992-631b-4feb-a522-acdb0a6d1f26",
    "Jokubas Uogintas",
    crate::card::CardRules::unsupported(),
);

// MKM 268 — Shadowy Backstreet (alternate printing)
const SHADOWY_BACKSTREET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHADOWY_BACKSTREET,
    1,
    "69c1b656-1d67-499c-bf0f-417682a86c7d",
    "Andreas Zafiratos",
);

// MKM 269 — Thundering Falls
pub(in crate::card::sets) static THUNDERING_FALLS: CardRecord = CardRecord::new(
    "Thundering Falls",
    "17260fff-b239-4af4-9306-3236ae3fa5a5",
    "Grady Frederick",
    // A dual that costs you the turn it lands and pays a little of it back by
    // filling the graveyard the decks that want it are built around.
    surveil_land(&["Island", "Mountain"]),
);

// MKM 270 — Undercity Sewers
pub(in crate::card::sets) static UNDERCITY_SEWERS: CardRecord = CardRecord::new(
    "Undercity Sewers",
    "2b5801fb-2026-4f25-98bc-ebb2f99684b9",
    "Yeong-Hao Han",
    // The blue-black half, and the one the cycle was designed for: the deck
    // playing it is already trying to fill a graveyard, so the look costs it
    // nothing it was not going to spend.
    surveil_land(&["Island", "Swamp"]),
);

// MKM 271 — Underground Mortuary (alternate printing)
const UNDERGROUND_MORTUARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNDERGROUND_MORTUARY,
    1,
    "f6ca59cd-8779-4a84-a54b-e863b79c61f0",
    "Sergey Glushakov",
);

// MKM 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "598f857d-ee17-4478-bb39-cc3ab77ab8d8",
    "Mia Boas",
);

// MKM 273 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "fe5f01e9-f85f-41e8-9527-88f76e7bfc02",
    "Mia Boas",
);

// MKM 274 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "53d0a415-26b8-4ba2-9503-b4ee2b93617c",
    "Mia Boas",
);

// MKM 275 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "5383b66e-e559-47d2-9967-9c2d5f898653",
    "Mia Boas",
);

// MKM 276 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "8126b842-ea58-4988-8b3f-0394cf766b91",
    "Mia Boas",
);

// MKM 277 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "81a1a8a6-916b-4eef-8079-6775afcb63cf",
    "Muhammad Firdaus",
);

// MKM 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "ec05cb6c-6e7f-4d40-ba38-a9fc06158094",
    "Carlos Palma Cruchaga",
);

// MKM 279 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "6908b583-4a52-4819-82a3-9db9c860aeb6",
    "Jorge Jacinto",
);

// MKM 280 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "df18762d-9980-4344-9d4f-e9d2cd8e0456",
    "Titus Lunter",
);

// MKM 281 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "70d0ef58-c0b3-4bcd-b470-0cf41219a4e6",
    "Carlos Palma Cruchaga",
);

// MKM 282 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "b1cbccfc-3585-4895-94d1-c3a67205e5fe",
    "Carlos Palma Cruchaga",
);

// MKM 283 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "de754d20-5371-456b-9064-8f0687d2eab7",
    "Jorge Jacinto",
);

// MKM 284 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "cd8a0cd6-8d3b-44bd-b609-c12fb851d17b",
    "Svetlin Velinov",
);

// MKM 285 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "cc485069-e081-4e83-bbad-d5faf7a5bd03",
    "Jorge Jacinto",
);

// MKM 286 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "6cb8f264-cc26-476e-a3b7-53638dce7395",
    "Carlos Palma Cruchaga",
);

// MKM 287 — Assemble the Players (alternate printing)
const ASSEMBLE_THE_PLAYERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASSEMBLE_THE_PLAYERS,
    1,
    "c88c5959-c033-46d0-94ea-df22988dc923",
    "Monztre",
);

// MKM 288 — Auspicious Arrival (alternate printing)
const AUSPICIOUS_ARRIVAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AUSPICIOUS_ARRIVAL,
    1,
    "49598c0a-3b3c-42a2-9358-ff072de9d9f2",
    "Robin Olausson",
);

// MKM 289 — Call a Surprise Witness (alternate printing)
const CALL_A_SURPRISE_WITNESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CALL_A_SURPRISE_WITNESS,
    1,
    "e9e844d2-80de-4473-acd3-c39c1b11246f",
    "Greg Opalinski",
);

// MKM 290 — Makeshift Binding (alternate printing)
const MAKESHIFT_BINDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAKESHIFT_BINDING,
    1,
    "f512dafb-3cba-4415-9f26-9345f1935b1e",
    "Lorenzo Mastroianni",
);

// MKM 291 — Not on My Watch (alternate printing)
const NOT_ON_MY_WATCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NOT_ON_MY_WATCH,
    1,
    "eb67d2a1-4b0b-4d4a-8ae9-eaf0f831cdb5",
    "Greg Opalinski",
);

// MKM 292 — On the Job (alternate printing)
const ON_THE_JOB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ON_THE_JOB,
    1,
    "70a5571e-02cb-4aa4-926d-d04f5ab04f29",
    "Alexander Mokhov",
);

// MKM 293 — Deduce (alternate printing)
const DEDUCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEDUCE,
    1,
    "5ad92dbd-828e-425f-8cab-819a04ea9020",
    "Ina Wong",
);

// MKM 294 — Dramatic Accusation (alternate printing)
const DRAMATIC_ACCUSATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAMATIC_ACCUSATION,
    1,
    "28bff30e-c25c-4d89-8fbe-1a9effe63ba7",
    "Ryan Alexander Lee",
);

// MKM 295 — Fae Flight (alternate printing)
const FAE_FLIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAE_FLIGHT,
    1,
    "51724cc6-edf7-490d-8039-b11b91ae1eb4",
    "Pablo Mendoza",
);

// MKM 296 — Intrude on the Mind (alternate printing)
const INTRUDE_ON_THE_MIND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INTRUDE_ON_THE_MIND,
    1,
    "bf6c5c84-2721-4aaa-9185-69351a4ab105",
    "Francis Tneh",
);

// MKM 297 — Reenact the Crime (alternate printing)
const REENACT_THE_CRIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REENACT_THE_CRIME,
    1,
    "580e1986-d22b-4deb-914a-2d38d6da4145",
    "Monztre",
);

// MKM 298 — Unauthorized Exit (alternate printing)
const UNAUTHORIZED_EXIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNAUTHORIZED_EXIT,
    1,
    "7695437c-1b7e-4163-aade-ca79b9c70e15",
    "Stephen Stark",
);

// MKM 299 — It Doesn't Add Up (alternate printing)
const IT_DOESN_T_ADD_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IT_DOESN_T_ADD_UP,
    1,
    "aef279e1-cbce-4766-9d56-02fa6c79d69d",
    "Eliz Roxs",
);

// MKM 300 — Murder (alternate printing)
const MURDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m13::MURDER,
    1,
    "4c98fcd3-8273-459f-857d-d0a34a1d285a",
    "Richard Wright",
);

// MKM 301 — Slice from the Shadows (alternate printing)
const SLICE_FROM_THE_SHADOWS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLICE_FROM_THE_SHADOWS,
    1,
    "d4e49c72-0fa2-4271-aabd-38a3e2b4779c",
    "Thea Dumitriu",
);

// MKM 302 — Soul Enervation (alternate printing)
const SOUL_ENERVATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOUL_ENERVATION,
    1,
    "7ef61777-be03-40c5-be9c-d573cf025813",
    "Joshua Cairos",
);

// MKM 303 — Anzrag's Rampage (alternate printing)
const ANZRAG_S_RAMPAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANZRAG_S_RAMPAGE,
    1,
    "14ba0604-3d9d-4f68-9086-6c7bf85a1354",
    "Richard Wright",
);

// MKM 304 — The Chase Is On (alternate printing)
const THE_CHASE_IS_ON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_CHASE_IS_ON,
    1,
    "87d5d312-8664-4690-9823-8be4d1c06cd0",
    "Wangjie Li",
);

// MKM 305 — Convenient Target (alternate printing)
const CONVENIENT_TARGET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONVENIENT_TARGET,
    1,
    "e11c0f62-80a0-4e45-9ed2-295814475a3f",
    "Sergio Cosmai",
);

// MKM 306 — Demand Answers (alternate printing)
const DEMAND_ANSWERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEMAND_ANSWERS,
    1,
    "2a35d1cb-5a61-49b7-a124-e8713655e4c3",
    "Robin Olausson",
);

// MKM 307 — Expose the Culprit (alternate printing)
const EXPOSE_THE_CULPRIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXPOSE_THE_CULPRIT,
    1,
    "dbaae9b6-510f-4039-8cca-fe7d7c57f1c5",
    "Lorenzo Mastroianni",
);

// MKM 308 — Analyze the Pollen (alternate printing)
const ANALYZE_THE_POLLEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANALYZE_THE_POLLEN,
    1,
    "46e8faf9-4230-4bb5-ad2e-b520ab338c35",
    "Monztre",
);

// MKM 309 — Audience with Trostani (alternate printing)
const AUDIENCE_WITH_TROSTANI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AUDIENCE_WITH_TROSTANI,
    1,
    "8b4b7b29-b1c1-4413-80d9-3c8585ed8eb4",
    "Hendry Iwanaga",
);

// MKM 310 — Fanatical Strength (alternate printing)
const FANATICAL_STRENGTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FANATICAL_STRENGTH,
    1,
    "28ee8b5b-ab55-4a26-9491-a0dc20cf54c8",
    "Joshua Cairos",
);

// MKM 310† — Fanatical Strength (alternate printing)
const FANATICAL_STRENGTH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FANATICAL_STRENGTH,
    2,
    "e6cccc8f-2a87-400a-83b1-74b4bde55dc6",
    "Joshua Cairos",
);

// MKM 311 — Coerced to Kill (alternate printing)
const COERCED_TO_KILL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COERCED_TO_KILL,
    1,
    "5c9c1153-93d6-4250-bc42-2d9fd3042e55",
    "Hendry Iwanaga",
);

// MKM 312 — Deadly Complication (alternate printing)
const DEADLY_COMPLICATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEADLY_COMPLICATION,
    1,
    "3f1aaebc-105a-4867-9727-24b8d7899e28",
    "Monztre",
);

// MKM 313 — Insidious Roots (alternate printing)
const INSIDIOUS_ROOTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSIDIOUS_ROOTS,
    1,
    "61c9e0ae-38c0-4618-b664-16a9ad1661c1",
    "Monztre",
);

// MKM 314 — Officious Interrogation (alternate printing)
const OFFICIOUS_INTERROGATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OFFICIOUS_INTERROGATION,
    1,
    "dd7a78b4-9fe3-4f99-8678-2cf5d344bd0c",
    "Eliz Roxs",
);

// MKM 315 — Warleader's Call (alternate printing)
const WARLEADER_S_CALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARLEADER_S_CALL,
    1,
    "c26cd008-0a09-4a3d-a5de-2babf79ec6c6",
    "Francis Tneh",
);

// MKM 316 — Worldsoul's Rage (alternate printing)
const WORLDSOUL_S_RAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WORLDSOUL_S_RAGE,
    1,
    "d62c3c3c-1196-45b0-832a-77ba26df3b70",
    "Axel Sauerwald",
);

// MKM 317 — Aurelia, the Law Above (alternate printing)
const AURELIA_THE_LAW_ABOVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AURELIA_THE_LAW_ABOVE,
    1,
    "c3f000f6-6a24-4daa-8552-bfc2634e6d83",
    "Barbara Rosiak",
);

// MKM 317z — Aurelia, the Law Above (alternate printing)
const AURELIA_THE_LAW_ABOVE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AURELIA_THE_LAW_ABOVE,
    2,
    "f39c4234-3b5a-4114-871a-ae8dd5659e86",
    "Barbara Rosiak",
);

// MKM 318 — Lazav, Wearer of Faces (alternate printing)
const LAZAV_WEARER_OF_FACES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAZAV_WEARER_OF_FACES,
    1,
    "a7f43e9f-8a0e-4534-ba4d-14cde33df864",
    "Julie Dillon",
);

// MKM 318z — Lazav, Wearer of Faces (alternate printing)
const LAZAV_WEARER_OF_FACES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LAZAV_WEARER_OF_FACES,
    2,
    "32a701da-47d6-4c85-a428-adeb4a20678d",
    "Julie Dillon",
);

// MKM 319 — Niv-Mizzet, Guildpact (alternate printing)
const NIV_MIZZET_GUILDPACT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NIV_MIZZET_GUILDPACT,
    1,
    "15fd713f-a0d0-4456-9678-d0a48e4bc334",
    "Alix Branwyn",
);

// MKM 319z — Niv-Mizzet, Guildpact (alternate printing)
const NIV_MIZZET_GUILDPACT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NIV_MIZZET_GUILDPACT,
    2,
    "3fdad75c-82e3-4feb-9bc9-29b6251439cb",
    "Alix Branwyn",
);

// MKM 320 — Rakdos, Patron of Chaos (alternate printing)
const RAKDOS_PATRON_OF_CHAOS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_PATRON_OF_CHAOS,
    1,
    "dea612ae-6e7e-4a42-844f-2bc97153b6b1",
    "Alex Dos Diaz",
);

// MKM 320z — Rakdos, Patron of Chaos (alternate printing)
const RAKDOS_PATRON_OF_CHAOS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_PATRON_OF_CHAOS,
    2,
    "6d5c2b49-ae53-48b7-877a-7f9aa7b6fc4a",
    "Alex Dos Diaz",
);

// MKM 321 — Teysa, Opulent Oligarch (alternate printing)
const TEYSA_OPULENT_OLIGARCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TEYSA_OPULENT_OLIGARCH,
    1,
    "536f271e-5daa-4ffe-9dd4-0ccc48d6fb66",
    "Barbara Rosiak",
);

// MKM 321z — Teysa, Opulent Oligarch (alternate printing)
const TEYSA_OPULENT_OLIGARCH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TEYSA_OPULENT_OLIGARCH,
    2,
    "4193a7bd-bba3-44f2-acf3-685847b8be27",
    "Barbara Rosiak",
);

// MKM 322 — Trostani, Three Whispers (alternate printing)
const TROSTANI_THREE_WHISPERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TROSTANI_THREE_WHISPERS,
    1,
    "a158690c-0a9a-4057-a6ee-f651bfb6664a",
    "Julie Dillon",
);

// MKM 322z — Trostani, Three Whispers (alternate printing)
const TROSTANI_THREE_WHISPERS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TROSTANI_THREE_WHISPERS,
    2,
    "8c420297-04c7-4659-b7cc-141a5c0ad1e1",
    "Julie Dillon",
);

// MKM 323 — Vannifar, Evolved Enigma (alternate printing)
const VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VANNIFAR_EVOLVED_ENIGMA,
    1,
    "9ea7381c-c745-458c-bf55-457242781a09",
    "Olena Richards",
);

// MKM 323z — Vannifar, Evolved Enigma (alternate printing)
const VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VANNIFAR_EVOLVED_ENIGMA,
    2,
    "62fa5a00-34d3-4ca4-afb9-62ab1c7afb14",
    "Olena Richards",
);

// MKM 324 — Commercial District (alternate printing)
const COMMERCIAL_DISTRICT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COMMERCIAL_DISTRICT,
    1,
    "3a9889ab-3527-414e-9c30-539f0608bd2b",
    "Sergey Glushakov",
);

// MKM 325 — Elegant Parlor (alternate printing)
const ELEGANT_PARLOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELEGANT_PARLOR,
    1,
    "1f0cef88-50bd-4baa-8d02-26ec4967a7be",
    "Sergey Glushakov",
);

// MKM 326 — Hedge Maze (alternate printing)
const HEDGE_MAZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEDGE_MAZE,
    1,
    "ee215175-d48b-4630-a1b1-36e195db05dd",
    "Sergey Glushakov",
);

// MKM 327 — Lush Portico (alternate printing)
const LUSH_PORTICO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUSH_PORTICO,
    1,
    "c3bd2920-9fbf-4853-ace3-66cd7015de32",
    "Sergey Glushakov",
);

// MKM 328 — Meticulous Archive (alternate printing)
const METICULOUS_ARCHIVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &METICULOUS_ARCHIVE,
    1,
    "79d4ba4b-03f3-4e64-b4e7-8c025fc70178",
    "Sergey Glushakov",
);

// MKM 329 — Raucous Theater
pub(in crate::card::sets) static RAUCOUS_THEATER: CardRecord = CardRecord::new(
    "Raucous Theater",
    "2faf0337-c7a3-45a0-bb14-c431526da2cd",
    "Sergey Glushakov",
    // The black-red half, which wants the graveyard for what it can cast out
    // of it rather than for a count: the look is a discard the deck was glad
    // to make.
    surveil_land(&["Swamp", "Mountain"]),
);

// MKM 330 — Shadowy Backstreet
pub(in crate::card::sets) static SHADOWY_BACKSTREET: CardRecord = CardRecord::new(
    "Shadowy Backstreet",
    "27eae4ce-e0b3-482b-9136-6fc17333877e",
    "Sergey Glushakov",
    // The white-black half. Its deck is the one least pleased to be given a
    // card it has to bin, which is why the look is worth reading twice.
    surveil_land(&["Plains", "Swamp"]),
);

// MKM 331 — Thundering Falls (alternate printing)
const THUNDERING_FALLS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDERING_FALLS,
    1,
    "da1792e4-2170-42ae-a335-fde6f5ef8932",
    "Sergey Glushakov",
);

// MKM 332 — Undercity Sewers (alternate printing)
const UNDERCITY_SEWERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNDERCITY_SEWERS,
    1,
    "3239a1e5-a002-4389-b9a7-049b1d60e2ac",
    "Sergey Glushakov",
);

// MKM 333 — Underground Mortuary
pub(in crate::card::sets) static UNDERGROUND_MORTUARY: CardRecord = CardRecord::new(
    "Underground Mortuary",
    "0d8938e4-bfa5-47e1-8c71-9c6583346300",
    "Sergey Glushakov",
    // The black-green half, whose deck is usually pleased to bin whatever
    // the look turns up: half of what it wants is already in the graveyard.
    surveil_land(&["Swamp", "Forest"]),
);

// MKM 334 — Kellan, Inquisitive Prodigy // Tail the Suspect (alternate printing)
const KELLAN_INQUISITIVE_PRODIGY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_INQUISITIVE_PRODIGY,
    1,
    "f60fcb1e-6136-4330-ae9b-57742fbb114f",
    "David Robert Hovey",
);

// MKM 335 — Kaya, Spirits' Justice (alternate printing)
const KAYA_SPIRITS_JUSTICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAYA_SPIRITS_JUSTICE,
    1,
    "7c9acee3-12b7-41c6-8a5c-7985d052bc7a",
    "Fay Dalton",
);

// MKM 336 — Aurelia's Vindicator (alternate printing)
const AURELIA_S_VINDICATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AURELIA_S_VINDICATOR,
    1,
    "975b5145-b30e-4ae3-b318-3b245bf12cc3",
    "Roberta Ingranata",
);

// MKM 337 — Delney, Streetwise Lookout (alternate printing)
const DELNEY_STREETWISE_LOOKOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DELNEY_STREETWISE_LOOKOUT,
    1,
    "b6297104-b73c-439c-ac0c-b7fac01d49dc",
    "Jack Hughes",
);

// MKM 338 — Doorkeeper Thrull (alternate printing)
const DOORKEEPER_THRULL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOORKEEPER_THRULL,
    1,
    "61cd24cf-ee30-4a05-a473-f3ec589c35dd",
    "Ivan Shavrin",
);

// MKM 339 — Neighborhood Guardian (alternate printing)
const NEIGHBORHOOD_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NEIGHBORHOOD_GUARDIAN,
    1,
    "0d12e748-e8d6-42d2-b501-c39cfcd6551d",
    "Ionomycin",
);

// MKM 340 — Wojek Investigator (alternate printing)
const WOJEK_INVESTIGATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WOJEK_INVESTIGATOR,
    2,
    "481b5d9a-872d-4a2f-a52a-d4a746ee04ab",
    "Lisa Heidhoff",
);

// MKM 341 — Conspiracy Unraveler (alternate printing)
const CONSPIRACY_UNRAVELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONSPIRACY_UNRAVELER,
    1,
    "0a33f43e-e57e-4d2c-b6f3-c785ab96214e",
    "Vance Kelly",
);

// MKM 342 — Forensic Gadgeteer (alternate printing)
const FORENSIC_GADGETEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FORENSIC_GADGETEER,
    1,
    "aab507e5-7bf2-4fa0-9ac0-a6aaec90de83",
    "Vance Kelly",
);

// MKM 343 — Homicide Investigator (alternate printing)
const HOMICIDE_INVESTIGATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOMICIDE_INVESTIGATOR,
    1,
    "b9106abf-f589-48a2-90f8-d606a7367377",
    "David Robert Hovey",
);

// MKM 344 — Massacre Girl, Known Killer (alternate printing)
const MASSACRE_GIRL_KNOWN_KILLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MASSACRE_GIRL_KNOWN_KILLER,
    1,
    "1ab741b3-6230-46c3-9c43-8a0d735e9d49",
    "Jack Hughes",
);

// MKM 345 — Persuasive Interrogators (alternate printing)
const PERSUASIVE_INTERROGATORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PERSUASIVE_INTERROGATORS,
    1,
    "6c134956-ba18-415e-b101-c1254415ea04",
    "Tyler Walpole",
);

// MKM 346 — Vein Ripper (alternate printing)
const VEIN_RIPPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VEIN_RIPPER,
    1,
    "65121bff-89f0-436d-89a5-b8191dc91891",
    "Marko Manev",
);

// MKM 347 — Frantic Scapegoat (alternate printing)
const FRANTIC_SCAPEGOAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRANTIC_SCAPEGOAT,
    1,
    "3ae07b33-dcee-41aa-9886-38336d2aff7a",
    "Peter Diamond",
);

// MKM 348 — Fugitive Codebreaker (alternate printing)
const FUGITIVE_CODEBREAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FUGITIVE_CODEBREAKER,
    1,
    "3cbb3378-291c-4de6-be98-dd09daa3b828",
    "Cosmin Podar",
);

// MKM 349 — Incinerator of the Guilty (alternate printing)
const INCINERATOR_OF_THE_GUILTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INCINERATOR_OF_THE_GUILTY,
    1,
    "874659c8-43e2-4298-844a-510f2b21830c",
    "Lisa Heidhoff",
);

// MKM 350 — Krenko, Baron of Tin Street (alternate printing)
const KRENKO_BARON_OF_TIN_STREET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRENKO_BARON_OF_TIN_STREET,
    1,
    "7fe8f3c6-934a-4819-8057-b8c80f2a8696",
    "Marko Manev",
);

// MKM 351 — Culvert Ambusher (alternate printing)
const CULVERT_AMBUSHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CULVERT_AMBUSHER,
    1,
    "82849769-ee1b-4f70-935c-9cb97772ade1",
    "Alex Stone",
);

// MKM 352 — The Pride of Hull Clade (alternate printing)
const THE_PRIDE_OF_HULL_CLADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_PRIDE_OF_HULL_CLADE,
    1,
    "6f8bdb03-02d0-4b97-b058-959f1fff9118",
    "Ivan Shavrin",
);

// MKM 353 — Sharp-Eyed Rookie (alternate printing)
const SHARP_EYED_ROOKIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHARP_EYED_ROOKIE,
    1,
    "be2b1b18-179d-48e1-ab8a-9374eb1ae429",
    "Marko Manev",
);

// MKM 354 — Agrus Kos, Spirit of Justice (alternate printing)
const AGRUS_KOS_SPIRIT_OF_JUSTICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGRUS_KOS_SPIRIT_OF_JUSTICE,
    1,
    "7fd2d61c-194a-482b-9c59-6800d6cc3a5b",
    "Cosmin Podar",
);

// MKM 355 — Alquist Proft, Master Sleuth (alternate printing)
const ALQUIST_PROFT_MASTER_SLEUTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALQUIST_PROFT_MASTER_SLEUTH,
    1,
    "e528424e-a3b1-4902-b42d-1ba9ac412e73",
    "Peter Diamond",
);

// MKM 356 — Anzrag, the Quake-Mole (alternate printing)
const ANZRAG_THE_QUAKE_MOLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANZRAG_THE_QUAKE_MOLE,
    1,
    "ba703ed8-3c5e-479f-b7f1-b8070d4f83b7",
    "Ivan Shavrin",
);

// MKM 357 — Aurelia, the Law Above (alternate printing)
const AURELIA_THE_LAW_ABOVE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &AURELIA_THE_LAW_ABOVE,
    3,
    "1c5598d3-b66b-42a2-aa43-d2785c1bfe36",
    "Peter Diamond",
);

// MKM 358 — Curious Cadaver (alternate printing)
const CURIOUS_CADAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CURIOUS_CADAVER,
    1,
    "edf5e45d-0f1a-4c85-a944-770fc525dc01",
    "Marko Manev",
);

// MKM 359 — Etrata, Deadly Fugitive (alternate printing)
const ETRATA_DEADLY_FUGITIVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ETRATA_DEADLY_FUGITIVE,
    1,
    "b01cf59e-b98b-4a8d-8347-07e2c1527330",
    "Ionomycin",
);

// MKM 360 — Ezrim, Agency Chief (alternate printing)
const EZRIM_AGENCY_CHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EZRIM_AGENCY_CHIEF,
    1,
    "4f05677b-ceec-4774-b22f-ff95cc6c5943",
    "Vance Kelly",
);

// MKM 361 — Gleaming Geardrake (alternate printing)
const GLEAMING_GEARDRAKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEAMING_GEARDRAKE,
    1,
    "387e53fd-58b2-45ac-a604-531b75442732",
    "Alex Dos Diaz",
);

// MKM 362 — Izoni, Center of the Web (alternate printing)
const IZONI_CENTER_OF_THE_WEB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IZONI_CENTER_OF_THE_WEB,
    1,
    "34c5f282-9297-4763-a04d-4147e5349953",
    "Marko Manev",
);

// MKM 363 — Judith, Carnage Connoisseur (alternate printing)
const JUDITH_CARNAGE_CONNOISSEUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JUDITH_CARNAGE_CONNOISSEUR,
    1,
    "fc220d51-b545-4414-9f2e-d3bfe53dcbd8",
    "Alex Dos Diaz",
);

// MKM 364 — Kraul Whipcracker (alternate printing)
const KRAUL_WHIPCRACKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KRAUL_WHIPCRACKER,
    2,
    "0a70bb7a-ba29-4fa5-b5f7-e2c71ca7df6f",
    "David Robert Hovey",
);

// MKM 365 — Kylox, Visionary Inventor (alternate printing)
const KYLOX_VISIONARY_INVENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KYLOX_VISIONARY_INVENTOR,
    1,
    "2260036c-cc6f-4016-8d8c-4953ce5b22d3",
    "Tyler Walpole",
);

// MKM 366 — Lazav, Wearer of Faces (alternate printing)
const LAZAV_WEARER_OF_FACES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LAZAV_WEARER_OF_FACES,
    3,
    "afd1791b-f12f-4e76-8276-cb1905bd2e33",
    "Tyler Walpole",
);

// MKM 367 — Meddling Youths (alternate printing)
const MEDDLING_YOUTHS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEDDLING_YOUTHS,
    1,
    "d7d1e216-a150-4f57-bd1c-8c8c86023c6d",
    "Jack Hughes",
);

// MKM 368 — Niv-Mizzet, Guildpact (alternate printing)
const NIV_MIZZET_GUILDPACT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &NIV_MIZZET_GUILDPACT,
    3,
    "02f2fcaf-97d6-441a-bdde-fb105ffad1c5",
    "Alex Stone",
);

// MKM 369 — Rakdos, Patron of Chaos (alternate printing)
const RAKDOS_PATRON_OF_CHAOS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_PATRON_OF_CHAOS,
    3,
    "9ea5f435-144e-4466-be0f-781664cfddb7",
    "Ivan Shavrin",
);

// MKM 370 — Teysa, Opulent Oligarch (alternate printing)
const TEYSA_OPULENT_OLIGARCH_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TEYSA_OPULENT_OLIGARCH,
    3,
    "ba0870d4-b5b8-4fd7-a21b-417f832e0d7e",
    "Jack Hughes",
);

// MKM 371 — Tolsimir, Midnight's Light (alternate printing)
const TOLSIMIR_MIDNIGHT_S_LIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOLSIMIR_MIDNIGHT_S_LIGHT,
    1,
    "8acaae58-93b5-4544-84cc-98e134edb738",
    "David Robert Hovey",
);

// MKM 372 — Trostani, Three Whispers (alternate printing)
const TROSTANI_THREE_WHISPERS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TROSTANI_THREE_WHISPERS,
    3,
    "839ff656-1886-4b31-8d72-dde696258a97",
    "Ionomycin",
);

// MKM 373 — Vannifar, Evolved Enigma (alternate printing)
const VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VANNIFAR_EVOLVED_ENIGMA,
    3,
    "4c0df81e-1700-41f0-bede-f3eb594351b6",
    "Jack Hughes",
);

// MKM 374 — Wispdrinker Vampire (alternate printing)
const WISPDRINKER_VAMPIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WISPDRINKER_VAMPIRE,
    1,
    "74463d05-3928-4d55-84d4-21d65623a3e6",
    "Tyler Walpole",
);

// MKM 375 — Yarus, Roar of the Old Gods (alternate printing)
const YARUS_ROAR_OF_THE_OLD_GODS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YARUS_ROAR_OF_THE_OLD_GODS,
    1,
    "2230bda5-42a9-40fa-bda4-1eabab5675f4",
    "Peter Diamond",
);

// MKM 376 — Magnetic Snuffler (alternate printing)
const MAGNETIC_SNUFFLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAGNETIC_SNUFFLER,
    1,
    "c5f9a827-b41d-4628-95e1-6546a8140c61",
    "Cosmin Podar",
);

// MKM 377 — Aurelia's Vindicator (alternate printing)
const AURELIA_S_VINDICATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AURELIA_S_VINDICATOR,
    2,
    "1fd20526-024a-4a08-8f86-f5d7c2d7e8e7",
    "Roberta Ingranata",
);

// MKM 378 — Delney, Streetwise Lookout (alternate printing)
const DELNEY_STREETWISE_LOOKOUT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DELNEY_STREETWISE_LOOKOUT,
    2,
    "0063654b-6e4e-4958-a63c-24cf933e4a40",
    "Jack Hughes",
);

// MKM 379 — Conspiracy Unraveler (alternate printing)
const CONSPIRACY_UNRAVELER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CONSPIRACY_UNRAVELER,
    2,
    "a8002174-8cff-4566-ba4d-d6caec14d69c",
    "Vance Kelly",
);

// MKM 380 — Massacre Girl, Known Killer (alternate printing)
const MASSACRE_GIRL_KNOWN_KILLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MASSACRE_GIRL_KNOWN_KILLER,
    2,
    "ae24a4c2-6a19-4e26-902a-cdf73d9f3608",
    "Jack Hughes",
);

// MKM 381 — Incinerator of the Guilty (alternate printing)
const INCINERATOR_OF_THE_GUILTY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &INCINERATOR_OF_THE_GUILTY,
    2,
    "321fe5e9-50c4-4b96-883a-2d9b10f2172f",
    "Lisa Heidhoff",
);

// MKM 382 — The Pride of Hull Clade (alternate printing)
const THE_PRIDE_OF_HULL_CLADE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_PRIDE_OF_HULL_CLADE,
    2,
    "99027565-f009-48d6-a7d0-fec1afff0611",
    "Ivan Shavrin",
);

// MKM 383 — Agrus Kos, Spirit of Justice (alternate printing)
const AGRUS_KOS_SPIRIT_OF_JUSTICE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AGRUS_KOS_SPIRIT_OF_JUSTICE,
    2,
    "55fc7b85-a321-4259-9fe6-a107d6d79819",
    "Cosmin Podar",
);

// MKM 384 — Alquist Proft, Master Sleuth (alternate printing)
const ALQUIST_PROFT_MASTER_SLEUTH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ALQUIST_PROFT_MASTER_SLEUTH,
    2,
    "33c4f8d2-1e9c-41e1-9991-8125cd7151a5",
    "Peter Diamond",
);

// MKM 385 — Anzrag, the Quake-Mole (alternate printing)
const ANZRAG_THE_QUAKE_MOLE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ANZRAG_THE_QUAKE_MOLE,
    2,
    "52c7933b-a65a-4032-a652-1a29b26de2c8",
    "Ivan Shavrin",
);

// MKM 386 — Etrata, Deadly Fugitive (alternate printing)
const ETRATA_DEADLY_FUGITIVE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ETRATA_DEADLY_FUGITIVE,
    2,
    "00b70f7a-8e77-4a99-8d13-7465a190630e",
    "Ionomycin",
);

// MKM 387 — Rakdos, Patron of Chaos (alternate printing)
const RAKDOS_PATRON_OF_CHAOS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_PATRON_OF_CHAOS,
    4,
    "fac8ed51-897f-4587-a234-dd3e051fab3f",
    "Ivan Shavrin",
);

// MKM 388 — Trostani, Three Whispers (alternate printing)
const TROSTANI_THREE_WHISPERS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &TROSTANI_THREE_WHISPERS,
    4,
    "0efe1969-e30c-4f07-90b0-3d9699a09f1b",
    "Ionomycin",
);

// MKM 389 — Vannifar, Evolved Enigma (alternate printing)
const VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &VANNIFAR_EVOLVED_ENIGMA,
    4,
    "3e1b9d5f-fb88-49ea-ab56-ea6e78ea2077",
    "Jack Hughes",
);

// MKM 390 — No Witnesses (alternate printing)
const NO_WITNESSES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NO_WITNESSES,
    1,
    "4d327d7d-1c01-4af1-8a6f-ce6cded4119d",
    "Michele Giorgi",
);

// MKM 391 — Tenth District Hero (alternate printing)
const TENTH_DISTRICT_HERO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TENTH_DISTRICT_HERO,
    1,
    "a2adffd5-ddc1-41ef-9442-2fbc17153f3d",
    "Kai Carpenter",
);

// MKM 392 — Unyielding Gatekeeper (alternate printing)
const UNYIELDING_GATEKEEPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNYIELDING_GATEKEEPER,
    1,
    "6349bcef-4ec5-4160-96ef-302d825f3888",
    "Borja Pindado",
);

// MKM 393 — Coveted Falcon (alternate printing)
const COVETED_FALCON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COVETED_FALCON,
    1,
    "03e5b027-7cd5-4343-b029-f4d197e1f1da",
    "Madeline Boni",
);

// MKM 394 — Cryptic Coat (alternate printing)
const CRYPTIC_COAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYPTIC_COAT,
    1,
    "c3faa376-7391-4084-8b57-15d71ab07cbd",
    "Julia Metzger",
);

// MKM 395 — Lost in the Maze (alternate printing)
const LOST_IN_THE_MAZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOST_IN_THE_MAZE,
    1,
    "d2a5a56a-3017-4f08-802d-e6f9d246c754",
    "Julian Kok Joon Wen",
);

// MKM 396 — Proft's Eidetic Memory
pub(in crate::card::sets) static PROFT_S_EIDETIC_MEMORY: CardRecord = CardRecord::new(
    "Proft's Eidetic Memory",
    "a3472756-0305-4567-b425-f7dbf9b3cc7f",
    "Julie Dillon",
// Two mana that replaces itself and then turns every spare cantrip into
    // permanent power, as long as there is a creature to put it on.
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this enchantment enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "You have no maximum hand size.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        crate::card::PlayerRuleDef::NoMaximumHandSize,
                    )),
                },
            ),
            AbilityDef::triggered_if_with_targets(
                "At the beginning of combat on your turn, if you've drawn more than one card this turn, \
                 put X +1/+1 counters on target creature you control, where X is the number of cards \
                 you've drawn this turn minus one.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                &// The card it draws on the way in is the first of the turn, so anything at
                    // all afterwards -- a cantrip, a fetchland cracked on their turn is not it,
                    // but a second draw on yours -- turns the trigger on.
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(1),
                    }),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    // "Minus one", which is why the card it draws itself is free rather than
                    // the first counter: the draw that turns the ability on is the one it does
                    // not pay for.
                    amount: ValueDef::Sum(&SumValueDef::new(
                        ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                        ValueDef::Constant(-1),
                    )),
                },
            ),
        ]),
);

// MKM 397 — Steamcore Scholar (alternate printing)
const STEAMCORE_SCHOLAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STEAMCORE_SCHOLAR,
    1,
    "83f0ac41-3f03-4480-aea8-41ec0a024671",
    "David Astruga",
);

// MKM 398 — Barbed Servitor (alternate printing)
const BARBED_SERVITOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARBED_SERVITOR,
    1,
    "7330586a-9614-4373-b455-ee5fb09ed262",
    "Simon Dominic",
);

// MKM 399 — Deadly Cover-Up (alternate printing)
const DEADLY_COVER_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEADLY_COVER_UP,
    1,
    "01f667e6-c27d-4055-8ddb-ea6cbcde6e4e",
    "Sam Guay",
);

// MKM 400 — Hunted Bonebrute (alternate printing)
const HUNTED_BONEBRUTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUNTED_BONEBRUTE,
    1,
    "1edff2c6-a259-4442-9e21-07572d0d1665",
    "Maxime Minard",
);

// MKM 401 — Illicit Masquerade (alternate printing)
const ILLICIT_MASQUERADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ILLICIT_MASQUERADE,
    1,
    "b436f52c-b780-4d16-a62f-e1b339356a39",
    "Valera Lutfullina",
);

// MKM 402 — Outrageous Robbery (alternate printing)
const OUTRAGEOUS_ROBBERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OUTRAGEOUS_ROBBERY,
    1,
    "89d8304d-59e3-4478-911d-54d77abfe69c",
    "Kai Carpenter",
);

// MKM 403 — Connecting the Dots (alternate printing)
const CONNECTING_THE_DOTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONNECTING_THE_DOTS,
    1,
    "597b8c60-d06b-425b-9fca-f2e3dfb45623",
    "Aaron J. Riley",
);

// MKM 404 — Expedited Inheritance (alternate printing)
const EXPEDITED_INHERITANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXPEDITED_INHERITANCE,
    1,
    "538c112a-203e-4b3e-ae1d-6eb7824fe68a",
    "Micah Epstein",
);

// MKM 405 — Krenko's Buzzcrusher (alternate printing)
const KRENKO_S_BUZZCRUSHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRENKO_S_BUZZCRUSHER,
    1,
    "7ebec98c-a935-48c3-ba2f-80c70079929d",
    "Joshua Raphael",
);

// MKM 406 — Lamplight Phoenix (alternate printing)
const LAMPLIGHT_PHOENIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAMPLIGHT_PHOENIX,
    1,
    "e9549ad7-5eb3-4672-a0f5-cad96db011c3",
    "Ryan Pancoast",
);

// MKM 407 — Pyrotechnic Performer (alternate printing)
const PYROTECHNIC_PERFORMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PYROTECHNIC_PERFORMER,
    1,
    "348027a1-1fb4-4473-b77e-b961e00a22b7",
    "Peter Polach",
);

// MKM 408 — Archdruid's Charm (alternate printing)
const ARCHDRUID_S_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHDRUID_S_CHARM,
    1,
    "00c2c17b-889b-416b-af58-c4039efec2bb",
    "Liiga Smilshkalne",
);

// MKM 409 — Axebane Ferox (alternate printing)
const AXEBANE_FEROX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AXEBANE_FEROX,
    1,
    "87a057ac-c9f9-4cf6-896f-e18976bdfb38",
    "Maxime Minard",
);

// MKM 410 — Hide in Plain Sight (alternate printing)
const HIDE_IN_PLAIN_SIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIDE_IN_PLAIN_SIGHT,
    1,
    "1d2c69f3-3c83-41e9-b389-21235a2dc5cd",
    "Vincent Christiaens",
);

// MKM 411 — Undergrowth Recon (alternate printing)
const UNDERGROWTH_RECON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNDERGROWTH_RECON,
    1,
    "b0c0e63c-ca4e-4991-89f1-dc4c22189372",
    "Ryan Pancoast",
);

// MKM 412 — Assassin's Trophy (alternate printing)
const ASSASSIN_S_TROPHY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_grn::ASSASSIN_S_TROPHY,
    1,
    "d6644d62-3d2a-4db1-a387-736fcad9a851",
    "Dmitry Burmak",
);

// MKM 413 — Blood Spatter Analysis (alternate printing)
const BLOOD_SPATTER_ANALYSIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOOD_SPATTER_ANALYSIS,
    1,
    "f82a68f5-62ef-40c9-8a40-e7b79619f719",
    "Jokubas Uogintas",
);

// MKM 414 — Doppelgang (alternate printing)
const DOPPELGANG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOPPELGANG,
    1,
    "7db9df42-042d-48a0-ac28-228b72a0b19a",
    "Chris Rallis",
);

// MKM 415 — Drag the Canal (alternate printing)
const DRAG_THE_CANAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAG_THE_CANAL,
    1,
    "066fed99-5aa5-42db-9472-240cfc2f4f42",
    "Josh Hass",
);

// MKM 416 — Ill-Timed Explosion (alternate printing)
const ILL_TIMED_EXPLOSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ILL_TIMED_EXPLOSION,
    1,
    "6cddd86f-2eaa-41ad-8f97-bd18bfa2f766",
    "Aaron J. Riley",
);

// MKM 417 — Kylox's Voltstrider (alternate printing)
const KYLOX_S_VOLTSTRIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KYLOX_S_VOLTSTRIDER,
    1,
    "7b524c71-c2ac-4a08-8ab7-27b1c232a359",
    "Volkan Baǵa",
);

// MKM 418 — Leyline of the Guildpact (alternate printing)
const LEYLINE_OF_THE_GUILDPACT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_OF_THE_GUILDPACT,
    1,
    "e549370b-f32d-48f0-877c-05e6fd1daf7b",
    "Daarken",
);

// MKM 419 — Relive the Past (alternate printing)
const RELIVE_THE_PAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RELIVE_THE_PAST,
    1,
    "9ffadebd-a7ef-4a75-918a-284d509b11c5",
    "Randy Vargas",
);

// MKM 420 — Treacherous Greed (alternate printing)
const TREACHEROUS_GREED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TREACHEROUS_GREED,
    1,
    "7c0ee59f-439a-47ab-b543-e0e013419f12",
    "Eli Minaya",
);

// MKM 421 — Urgent Necropsy (alternate printing)
const URGENT_NECROPSY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &URGENT_NECROPSY,
    1,
    "fe89b207-04a4-4cdc-95a2-07c7761b885e",
    "Uriah Voth",
);

// MKM 422 — Cryptex (alternate printing)
const CRYPTEX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYPTEX,
    1,
    "17938b40-a4a3-4150-a94c-ecddbed3ccfc",
    "Yeong-Hao Han",
);

// MKM 423 — Long Goodbye (alternate printing)
const LONG_GOODBYE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LONG_GOODBYE,
    1,
    "b12a8889-1323-4268-b34c-146c4136fd53",
    "Jarel Threat",
);

// MKM 424 — Gleaming Geardrake (alternate printing)
const GLEAMING_GEARDRAKE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GLEAMING_GEARDRAKE,
    2,
    "00b6cd91-8178-4c5c-9cb1-f3d970fe3b03",
    "Filipe Pagliuso",
);

// MKM 425 — Kraul Whipcracker (alternate printing)
const KRAUL_WHIPCRACKER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KRAUL_WHIPCRACKER,
    3,
    "fefd02c8-7b28-4128-b2fc-d961e565a147",
    "Filip Burburan",
);

// MKM 426 — Lightning Helix (alternate printing)
const LIGHTNING_HELIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::LIGHTNING_HELIX,
    1,
    "7e478972-27a6-4a89-a8d7-cb1abdd9f0d9",
    "Eli Minaya",
);

// MKM 427 — No More Lies (alternate printing)
const NO_MORE_LIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NO_MORE_LIES,
    1,
    "7c882d46-c26c-47f1-96ce-b0676f345b89",
    "Liiga Smilshkalne",
);

// MKM 428 — Axebane Ferox (alternate printing)
const AXEBANE_FEROX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AXEBANE_FEROX,
    2,
    "3ddd5e33-5a58-4321-8742-236d6cc6f4f4",
    "Adam Volker",
);

// MKM 429 — Wojek Investigator (alternate printing)
const WOJEK_INVESTIGATOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WOJEK_INVESTIGATOR,
    3,
    "58eccc85-bdfe-48f4-a79e-76d9ef122815",
    "Greg Staples",
);

// MKM 430 — Melek, Reforged Researcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELEK_REFORGED_RESEARCHER: CardRecord = CardRecord::new(
    "Melek, Reforged Researcher",
    "01c5ede0-a098-4f21-8b7e-795a83e75aae",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// MKM 431 — Tomik, Wielder of Law
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOMIK_WIELDER_OF_LAW: CardRecord = CardRecord::new(
    "Tomik, Wielder of Law",
    "2c5a7550-fe1a-4797-9583-70ab56cfac0d",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// MKM 432 — Voja, Jaws of the Conclave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOJA_JAWS_OF_THE_CONCLAVE: CardRecord = CardRecord::new(
    "Voja, Jaws of the Conclave",
    "bfa1bd2f-25bd-4fbd-877b-cef00ab7f92f",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// MKM 433 — Vein Ripper (alternate printing)
const VEIN_RIPPER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VEIN_RIPPER,
    2,
    "e7be0f36-9acd-457b-bdfa-77be787be5c3",
    "Marko Manev",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CASE_OF_THE_SHATTERED_PACT,
    &ABSOLVING_LAMMASU,
    &ASSEMBLE_THE_PLAYERS,
    &AURELIA_S_VINDICATOR,
    &AUSPICIOUS_ARRIVAL,
    &CALL_A_SURPRISE_WITNESS,
    &CASE_FILE_AUDITOR,
    &CASE_OF_THE_GATEWAY_EXPRESS,
    &CASE_OF_THE_PILFERED_PROOF,
    &CASE_OF_THE_UNEATEN_FEAST,
    &DEFENESTRATED_PHANTOM,
    &DELNEY_STREETWISE_LOOKOUT,
    &DOORKEEPER_THRULL,
    &DUE_DILIGENCE,
    &ESSENCE_OF_ANTIQUITY,
    &FORUM_FAMILIAR,
    &GRIFFNAUT_TRACKER,
    &HAAZDA_VIGILANTE,
    &INSIDE_SOURCE,
    &KARLOV_WATCHDOG,
    &KROVOD_HAUNCH,
    &MAKE_YOUR_MOVE,
    &MAKESHIFT_BINDING,
    &MARKETWATCH_PHANTOM,
    &MUSEUM_NIGHTWATCH,
    &NEIGHBORHOOD_GUARDIAN,
    &NO_WITNESSES,
    &NOT_ON_MY_WATCH,
    &NOVICE_INSPECTOR,
    &ON_THE_JOB,
    &PERIMETER_ENFORCER,
    &SANCTUARY_WALL,
    &SEASONED_CONSULTANT,
    &TENTH_DISTRICT_HERO,
    &UNYIELDING_GATEKEEPER,
    &WOJEK_INVESTIGATOR,
    &WRENCH,
    &AGENCY_OUTFITTER,
    &BEHIND_THE_MASK,
    &BENTHIC_CRIMINOLOGISTS,
    &BUBBLE_SMUGGLER,
    &BURDEN_OF_PROOF,
    &CANDLESTICK,
    &CASE_OF_THE_FILCHED_FALCON,
    &CASE_OF_THE_RANSACKED_LAB,
    &COLD_CASE_CRACKER,
    &CONSPIRACY_UNRAVELER,
    &COVETED_FALCON,
    &CRIMESTOPPER_SPRITE,
    &CRYPTIC_COAT,
    &CURIOUS_INQUIRY,
    &DEDUCE,
    &DRAMATIC_ACCUSATION,
    &ELIMINATE_THE_IMPOSSIBLE,
    &EXIT_SPECIALIST,
    &FAE_FLIGHT,
    &FORENSIC_GADGETEER,
    &FORENSIC_RESEARCHER,
    &FURTIVE_COURIER,
    &HOTSHOT_INVESTIGATORS,
    &INTRUDE_ON_THE_MIND,
    &JADED_ANALYST,
    &LIVING_CONUNDRUM,
    &LOST_IN_THE_MAZE,
    &MISTWAY_SPY,
    &OUT_COLD,
    &PROJEKTOR_INSPECTOR,
    &REASONABLE_DOUBT,
    &REENACT_THE_CRIME,
    &STEAMCORE_SCHOLAR,
    &SUDDEN_SETBACK,
    &SURVEILLANCE_MONITOR,
    &UNAUTHORIZED_EXIT,
    &AGENCY_CORONER,
    &ALLEY_ASSAILANT,
    &BARBED_SERVITOR,
    &BASILICA_STALKER,
    &CASE_OF_THE_GORGON_S_KISS,
    &CASE_OF_THE_STASHED_SKELETON,
    &CEREBRAL_CONFISCATION,
    &CLANDESTINE_MEDDLER,
    &DEADLY_COVER_UP,
    &EXTRACT_A_CONFESSION,
    &FESTERLEECH,
    &HOMICIDE_INVESTIGATOR,
    &HUNTED_BONEBRUTE,
    &ILLICIT_MASQUERADE,
    &IT_DOESN_T_ADD_UP,
    &LEAD_PIPE,
    &LEERING_ONLOOKER,
    &LONG_GOODBYE,
    &MACABRE_RECONSTRUCTION,
    &MASSACRE_GIRL_KNOWN_KILLER,
    &NIGHTDRINKER_MOROII,
    &OUTRAGEOUS_ROBBERY,
    &PERSUASIVE_INTERROGATORS,
    &POLYGRAPH_ORB,
    &PRESUMED_DEAD,
    &REPEAT_OFFENDER,
    &ROT_FARM_MORTIPEDE,
    &SLICE_FROM_THE_SHADOWS,
    &SLIMY_DUALLEECH,
    &SNARLING_GOREHOUND,
    &SOUL_ENERVATION,
    &TOXIN_ANALYSIS,
    &UNDERCITY_ELIMINATOR,
    &UNSCRUPULOUS_AGENT,
    &VEIN_RIPPER,
    &ANZRAG_S_RAMPAGE,
    &BOLRAC_CLAN_BASHER,
    &CASE_OF_THE_BURNING_MASKS,
    &CASE_OF_THE_CRIMSON_PULSE,
    &CAUGHT_RED_HANDED,
    &THE_CHASE_IS_ON,
    &CONCEALED_WEAPON,
    &CONNECTING_THE_DOTS,
    &CONVENIENT_TARGET,
    &CORNERED_CROOK,
    &CRIME_NOVELIST,
    &DEMAND_ANSWERS,
    &EXPEDITED_INHERITANCE,
    &EXPOSE_THE_CULPRIT,
    &FELONIOUS_RAGE,
    &FRANTIC_SCAPEGOAT,
    &FUGITIVE_CODEBREAKER,
    &GALVANIZE,
    &GEARBANE_ORANGUTAN,
    &GOBLIN_MASKMAKER,
    &HARRIED_DRONESMITH,
    &INCINERATOR_OF_THE_GUILTY,
    &INNOCENT_BYSTANDER,
    &KNIFE,
    &KRENKO_BARON_OF_TIN_STREET,
    &KRENKO_S_BUZZCRUSHER,
    &LAMPLIGHT_PHOENIX,
    &OFFENDER_AT_LARGE,
    &PERSON_OF_INTEREST,
    &PYROTECHNIC_PERFORMER,
    &RECKLESS_DETECTIVE,
    &RED_HERRING,
    &RUBBLEBELT_BRAGGART,
    &SUSPICIOUS_DETONATION,
    &TORCH_THE_WITNESS,
    &VENGEFUL_TRACKER,
    &AFTERMATH_ANALYST,
    &AIRTIGHT_ALIBI,
    &ANALYZE_THE_POLLEN,
    &ARCHDRUID_S_CHARM,
    &AUDIENCE_WITH_TROSTANI,
    &AXEBANE_FEROX,
    &BITE_DOWN_ON_CRIME,
    &CASE_OF_THE_LOCKED_HOTHOUSE,
    &CASE_OF_THE_TRAMPLED_GARDEN,
    &CHALK_OUTLINE,
    &CULVERT_AMBUSHER,
    &FANATICAL_STRENGTH,
    &FLOURISHING_BLOOM_KIN,
    &GET_A_LEG_UP,
    &GLINT_WEAVER,
    &GREENBELT_RADICAL,
    &HARD_HITTING_QUESTION,
    &HEDGE_WHISPERER,
    &HIDE_IN_PLAIN_SIGHT,
    &A_KILLER_AMONG_US,
    &LOXODON_EAVESDROPPER,
    &NERVOUS_GARDENER,
    &PICK_YOUR_POISON,
    &POMPOUS_GADABOUT,
    &THE_PRIDE_OF_HULL_CLADE,
    &ROPE,
    &RUBBLEBELT_MAVERICK,
    &SAMPLE_COLLECTOR,
    &SHARP_EYED_ROOKIE,
    &SLIME_AGAINST_HUMANITY,
    &THEY_WENT_THIS_WAY,
    &TOPIARY_PANTHER,
    &TUNNEL_TIPSTER,
    &UNDERGROWTH_RECON,
    &VENGEFUL_CREEPER,
    &VITU_GHAZI_INSPECTOR,
    &AGRUS_KOS_SPIRIT_OF_JUSTICE,
    &ALQUIST_PROFT_MASTER_SLEUTH,
    &ANZRAG_THE_QUAKE_MOLE,
    &AURELIA_THE_LAW_ABOVE,
    &BLOOD_SPATTER_ANALYSIS,
    &BREAK_OUT,
    &BURIED_IN_THE_GARDEN,
    &COERCED_TO_KILL,
    &CROWD_CONTROL_WARDEN,
    &CURIOUS_CADAVER,
    &DEADLY_COMPLICATION,
    &DETECTIVE_S_SATCHEL,
    &DOG_WALKER,
    &DOPPELGANG,
    &DRAG_THE_CANAL,
    &ETRATA_DEADLY_FUGITIVE,
    &EVIDENCE_EXAMINER,
    &EZRIM_AGENCY_CHIEF,
    &FAERIE_SNOOP,
    &GADGET_TECHNICIAN,
    &GLEAMING_GEARDRAKE,
    &GRANITE_WITNESS,
    &ILL_TIMED_EXPLOSION,
    &INSIDIOUS_ROOTS,
    &IZONI_CENTER_OF_THE_WEB,
    &JUDITH_CARNAGE_CONNOISSEUR,
    &KAYA_SPIRITS_JUSTICE,
    &KELLAN_INQUISITIVE_PRODIGY,
    &KRAUL_WHIPCRACKER,
    &KYLOX_VISIONARY_INVENTOR,
    &KYLOX_S_VOLTSTRIDER,
    &LAZAV_WEARER_OF_FACES,
    &LEYLINE_OF_THE_GUILDPACT,
    &MEDDLING_YOUTHS,
    &NIV_MIZZET_GUILDPACT,
    &NO_MORE_LIES,
    &OFFICIOUS_INTERROGATION,
    &PRIVATE_EYE,
    &RAKDOS_PATRON_OF_CHAOS,
    &RAKISH_SCOUNDREL,
    &RELIVE_THE_PAST,
    &REPULSIVE_MUTATION,
    &RIFTBURST_HELLION,
    &RUNE_BRAND_JUGGLER,
    &SANGUINE_SAVIOR,
    &SHADY_INFORMANT,
    &SOUL_SEARCH,
    &SUMALA_SENTRY,
    &TEYSA_OPULENT_OLIGARCH,
    &TIN_STREET_GOSSIP,
    &TOLSIMIR_MIDNIGHT_S_LIGHT,
    &TREACHEROUS_GREED,
    &TROSTANI_THREE_WHISPERS,
    &UNDERCOVER_CROCODELF,
    &URGENT_NECROPSY,
    &VANNIFAR_EVOLVED_ENIGMA,
    &WARLEADER_S_CALL,
    &WISPDRINKER_VAMPIRE,
    &WORLDSOUL_S_RAGE,
    &YARUS_ROAR_OF_THE_OLD_GODS,
    &CEASE,
    &FLOTSAM,
    &FUSS,
    &HUSTLE,
    &PUSH,
    &CRYPTEX,
    &GRAVESTONE_STRIDER,
    &LUMBERING_LAUNDRY,
    &MAGNETIC_SNUFFLER,
    &SANITATION_AUTOMATON,
    &THINKING_CAP,
    &BRANCH_OF_VITU_GHAZI,
    &COMMERCIAL_DISTRICT,
    &ELEGANT_PARLOR,
    &ESCAPE_TUNNEL,
    &HEDGE_MAZE,
    &LUSH_PORTICO,
    &METICULOUS_ARCHIVE,
    &PUBLIC_THOROUGHFARE,
    &SCENE_OF_THE_CRIME,
    &THUNDERING_FALLS,
    &UNDERCITY_SEWERS,
    &RAUCOUS_THEATER,
    &SHADOWY_BACKSTREET,
    &UNDERGROUND_MORTUARY,
    &PROFT_S_EIDETIC_MEMORY,
    &MELEK_REFORGED_RESEARCHER,
    &TOMIK_WIELDER_OF_LAW,
    &VOJA_JAWS_OF_THE_CONCLAVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    CASE_FILE_AUDITOR_ALTERNATE_1,
    WOJEK_INVESTIGATOR_ALTERNATE_1,
    PROFT_S_EIDETIC_MEMORY_ALTERNATE_1,
    SUDDEN_SETBACK_ALTERNATE_1,
    SURVEILLANCE_MONITOR_ALTERNATE_1,
    CASE_OF_THE_GORGON_S_KISS_ALTERNATE_1,
    CLANDESTINE_MEDDLER_ALTERNATE_1,
    MURDER_REPRINT,
    SHOCK_REPRINT,
    TORCH_THE_WITNESS_ALTERNATE_1,
    ASSASSIN_S_TROPHY_REPRINT,
    BREAK_OUT_ALTERNATE_1,
    KRAUL_WHIPCRACKER_ALTERNATE_1,
    LIGHTNING_HELIX_REPRINT,
    UNDERCOVER_CROCODELF_ALTERNATE_1,
    MAGNIFYING_GLASS_REPRINT,
    RAUCOUS_THEATER_ALTERNATE_1,
    SHADOWY_BACKSTREET_ALTERNATE_1,
    UNDERGROUND_MORTUARY_ALTERNATE_1,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    ASSEMBLE_THE_PLAYERS_ALTERNATE_1,
    AUSPICIOUS_ARRIVAL_ALTERNATE_1,
    CALL_A_SURPRISE_WITNESS_ALTERNATE_1,
    MAKESHIFT_BINDING_ALTERNATE_1,
    NOT_ON_MY_WATCH_ALTERNATE_1,
    ON_THE_JOB_ALTERNATE_1,
    DEDUCE_ALTERNATE_1,
    DRAMATIC_ACCUSATION_ALTERNATE_1,
    FAE_FLIGHT_ALTERNATE_1,
    INTRUDE_ON_THE_MIND_ALTERNATE_1,
    REENACT_THE_CRIME_ALTERNATE_1,
    UNAUTHORIZED_EXIT_ALTERNATE_1,
    IT_DOESN_T_ADD_UP_ALTERNATE_1,
    MURDER_ALTERNATE_1,
    SLICE_FROM_THE_SHADOWS_ALTERNATE_1,
    SOUL_ENERVATION_ALTERNATE_1,
    ANZRAG_S_RAMPAGE_ALTERNATE_1,
    THE_CHASE_IS_ON_ALTERNATE_1,
    CONVENIENT_TARGET_ALTERNATE_1,
    DEMAND_ANSWERS_ALTERNATE_1,
    EXPOSE_THE_CULPRIT_ALTERNATE_1,
    ANALYZE_THE_POLLEN_ALTERNATE_1,
    AUDIENCE_WITH_TROSTANI_ALTERNATE_1,
    FANATICAL_STRENGTH_ALTERNATE_1,
    FANATICAL_STRENGTH_ALTERNATE_2,
    COERCED_TO_KILL_ALTERNATE_1,
    DEADLY_COMPLICATION_ALTERNATE_1,
    INSIDIOUS_ROOTS_ALTERNATE_1,
    OFFICIOUS_INTERROGATION_ALTERNATE_1,
    WARLEADER_S_CALL_ALTERNATE_1,
    WORLDSOUL_S_RAGE_ALTERNATE_1,
    AURELIA_THE_LAW_ABOVE_ALTERNATE_1,
    AURELIA_THE_LAW_ABOVE_ALTERNATE_2,
    LAZAV_WEARER_OF_FACES_ALTERNATE_1,
    LAZAV_WEARER_OF_FACES_ALTERNATE_2,
    NIV_MIZZET_GUILDPACT_ALTERNATE_1,
    NIV_MIZZET_GUILDPACT_ALTERNATE_2,
    RAKDOS_PATRON_OF_CHAOS_ALTERNATE_1,
    RAKDOS_PATRON_OF_CHAOS_ALTERNATE_2,
    TEYSA_OPULENT_OLIGARCH_ALTERNATE_1,
    TEYSA_OPULENT_OLIGARCH_ALTERNATE_2,
    TROSTANI_THREE_WHISPERS_ALTERNATE_1,
    TROSTANI_THREE_WHISPERS_ALTERNATE_2,
    VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_1,
    VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_2,
    COMMERCIAL_DISTRICT_ALTERNATE_1,
    ELEGANT_PARLOR_ALTERNATE_1,
    HEDGE_MAZE_ALTERNATE_1,
    LUSH_PORTICO_ALTERNATE_1,
    METICULOUS_ARCHIVE_ALTERNATE_1,
    THUNDERING_FALLS_ALTERNATE_1,
    UNDERCITY_SEWERS_ALTERNATE_1,
    KELLAN_INQUISITIVE_PRODIGY_ALTERNATE_1,
    KAYA_SPIRITS_JUSTICE_ALTERNATE_1,
    AURELIA_S_VINDICATOR_ALTERNATE_1,
    DELNEY_STREETWISE_LOOKOUT_ALTERNATE_1,
    DOORKEEPER_THRULL_ALTERNATE_1,
    NEIGHBORHOOD_GUARDIAN_ALTERNATE_1,
    WOJEK_INVESTIGATOR_ALTERNATE_2,
    CONSPIRACY_UNRAVELER_ALTERNATE_1,
    FORENSIC_GADGETEER_ALTERNATE_1,
    HOMICIDE_INVESTIGATOR_ALTERNATE_1,
    MASSACRE_GIRL_KNOWN_KILLER_ALTERNATE_1,
    PERSUASIVE_INTERROGATORS_ALTERNATE_1,
    VEIN_RIPPER_ALTERNATE_1,
    FRANTIC_SCAPEGOAT_ALTERNATE_1,
    FUGITIVE_CODEBREAKER_ALTERNATE_1,
    INCINERATOR_OF_THE_GUILTY_ALTERNATE_1,
    KRENKO_BARON_OF_TIN_STREET_ALTERNATE_1,
    CULVERT_AMBUSHER_ALTERNATE_1,
    THE_PRIDE_OF_HULL_CLADE_ALTERNATE_1,
    SHARP_EYED_ROOKIE_ALTERNATE_1,
    AGRUS_KOS_SPIRIT_OF_JUSTICE_ALTERNATE_1,
    ALQUIST_PROFT_MASTER_SLEUTH_ALTERNATE_1,
    ANZRAG_THE_QUAKE_MOLE_ALTERNATE_1,
    AURELIA_THE_LAW_ABOVE_ALTERNATE_3,
    CURIOUS_CADAVER_ALTERNATE_1,
    ETRATA_DEADLY_FUGITIVE_ALTERNATE_1,
    EZRIM_AGENCY_CHIEF_ALTERNATE_1,
    GLEAMING_GEARDRAKE_ALTERNATE_1,
    IZONI_CENTER_OF_THE_WEB_ALTERNATE_1,
    JUDITH_CARNAGE_CONNOISSEUR_ALTERNATE_1,
    KRAUL_WHIPCRACKER_ALTERNATE_2,
    KYLOX_VISIONARY_INVENTOR_ALTERNATE_1,
    LAZAV_WEARER_OF_FACES_ALTERNATE_3,
    MEDDLING_YOUTHS_ALTERNATE_1,
    NIV_MIZZET_GUILDPACT_ALTERNATE_3,
    RAKDOS_PATRON_OF_CHAOS_ALTERNATE_3,
    TEYSA_OPULENT_OLIGARCH_ALTERNATE_3,
    TOLSIMIR_MIDNIGHT_S_LIGHT_ALTERNATE_1,
    TROSTANI_THREE_WHISPERS_ALTERNATE_3,
    VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_3,
    WISPDRINKER_VAMPIRE_ALTERNATE_1,
    YARUS_ROAR_OF_THE_OLD_GODS_ALTERNATE_1,
    MAGNETIC_SNUFFLER_ALTERNATE_1,
    AURELIA_S_VINDICATOR_ALTERNATE_2,
    DELNEY_STREETWISE_LOOKOUT_ALTERNATE_2,
    CONSPIRACY_UNRAVELER_ALTERNATE_2,
    MASSACRE_GIRL_KNOWN_KILLER_ALTERNATE_2,
    INCINERATOR_OF_THE_GUILTY_ALTERNATE_2,
    THE_PRIDE_OF_HULL_CLADE_ALTERNATE_2,
    AGRUS_KOS_SPIRIT_OF_JUSTICE_ALTERNATE_2,
    ALQUIST_PROFT_MASTER_SLEUTH_ALTERNATE_2,
    ANZRAG_THE_QUAKE_MOLE_ALTERNATE_2,
    ETRATA_DEADLY_FUGITIVE_ALTERNATE_2,
    RAKDOS_PATRON_OF_CHAOS_ALTERNATE_4,
    TROSTANI_THREE_WHISPERS_ALTERNATE_4,
    VANNIFAR_EVOLVED_ENIGMA_ALTERNATE_4,
    NO_WITNESSES_ALTERNATE_1,
    TENTH_DISTRICT_HERO_ALTERNATE_1,
    UNYIELDING_GATEKEEPER_ALTERNATE_1,
    COVETED_FALCON_ALTERNATE_1,
    CRYPTIC_COAT_ALTERNATE_1,
    LOST_IN_THE_MAZE_ALTERNATE_1,
    STEAMCORE_SCHOLAR_ALTERNATE_1,
    BARBED_SERVITOR_ALTERNATE_1,
    DEADLY_COVER_UP_ALTERNATE_1,
    HUNTED_BONEBRUTE_ALTERNATE_1,
    ILLICIT_MASQUERADE_ALTERNATE_1,
    OUTRAGEOUS_ROBBERY_ALTERNATE_1,
    CONNECTING_THE_DOTS_ALTERNATE_1,
    EXPEDITED_INHERITANCE_ALTERNATE_1,
    KRENKO_S_BUZZCRUSHER_ALTERNATE_1,
    LAMPLIGHT_PHOENIX_ALTERNATE_1,
    PYROTECHNIC_PERFORMER_ALTERNATE_1,
    ARCHDRUID_S_CHARM_ALTERNATE_1,
    AXEBANE_FEROX_ALTERNATE_1,
    HIDE_IN_PLAIN_SIGHT_ALTERNATE_1,
    UNDERGROWTH_RECON_ALTERNATE_1,
    ASSASSIN_S_TROPHY_ALTERNATE_1,
    BLOOD_SPATTER_ANALYSIS_ALTERNATE_1,
    DOPPELGANG_ALTERNATE_1,
    DRAG_THE_CANAL_ALTERNATE_1,
    ILL_TIMED_EXPLOSION_ALTERNATE_1,
    KYLOX_S_VOLTSTRIDER_ALTERNATE_1,
    LEYLINE_OF_THE_GUILDPACT_ALTERNATE_1,
    RELIVE_THE_PAST_ALTERNATE_1,
    TREACHEROUS_GREED_ALTERNATE_1,
    URGENT_NECROPSY_ALTERNATE_1,
    CRYPTEX_ALTERNATE_1,
    LONG_GOODBYE_ALTERNATE_1,
    GLEAMING_GEARDRAKE_ALTERNATE_2,
    KRAUL_WHIPCRACKER_ALTERNATE_3,
    LIGHTNING_HELIX_ALTERNATE_1,
    NO_MORE_LIES_ALTERNATE_1,
    AXEBANE_FEROX_ALTERNATE_2,
    WOJEK_INVESTIGATOR_ALTERNATE_3,
    VEIN_RIPPER_ALTERNATE_2,
];
