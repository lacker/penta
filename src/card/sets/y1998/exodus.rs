//! EXO card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y2011::innistrad as catalog_isd;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::{
    AbilityCoverageDef, AbilityDef, CardArt, CardRules, CardSet, CardType, ComparisonDef,
    EffectDef, EffectRecipientDef, MillUntilDef, ObjectPredicateDef, ObjectQueryDef,
    PlayerRelation, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueComparisonDef,
    ValueDef, ZoneKind,
};
use crate::mana_cost;

// EXO 1 — Allay
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALLAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f20a1c6d-ec6a-4bd6-b3b2-b997f71d41fc"),
    "Allay",
    crate::card::CardArt::new("f20a1c6d-ec6a-4bd6-b3b2-b997f71d41fc", "Randy Gallegos"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 2 — Angelic Blessing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31dda640-2a00-437e-855f-173c487e7395"),
    "Angelic Blessing",
    crate::card::CardArt::new("ed3c8bae-953f-4bb4-a78d-02e4e354e53c", "Mark Zug"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 3 — Cataclysm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CATACLYSM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("024ae668-a1ae-4020-89c8-acbd8bd0a691"),
    "Cataclysm",
    crate::card::CardArt::new("024ae668-a1ae-4020-89c8-acbd8bd0a691", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 4 — Charging Paladin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29db1bbf-a6cf-460c-bec8-dbd682157af4"),
    "Charging Paladin",
    crate::card::CardArt::new("851f3f72-2923-4432-898a-02679a8b320f", "Ciruelo"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 5 — Convalescence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONVALESCENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fd49a61-42ba-400a-8ca9-9f6058bf85ca"),
    "Convalescence",
    crate::card::CardArt::new(
        "0fd49a61-42ba-400a-8ca9-9f6058bf85ca",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 6 — Exalted Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXALTED_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7537bab3-4bac-4b83-9ad3-dfcb4ff19d6d"),
    "Exalted Dragon",
    crate::card::CardArt::new("7537bab3-4bac-4b83-9ad3-dfcb4ff19d6d", "Matthew D. Wilson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 7 — High Ground
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_GROUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c5239dc-f51b-48c0-91a2-ed6551aaff32"),
    "High Ground",
    crate::card::CardArt::new("1c5239dc-f51b-48c0-91a2-ed6551aaff32", "rk post"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 8 — Keeper of the Light
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_THE_LIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06eda847-c599-4163-b48b-aa76b153ed86"),
    "Keeper of the Light",
    crate::card::CardArt::new(
        "06eda847-c599-4163-b48b-aa76b153ed86",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 9 — Kor Chant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KOR_CHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dc61cc3-0312-44f4-9c23-4fc37c3fbbd5"),
    "Kor Chant",
    crate::card::CardArt::new("8dc61cc3-0312-44f4-9c23-4fc37c3fbbd5", "John Matson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 10 — Limited Resources
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIMITED_RESOURCES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20ae3609-a3cc-486c-94f6-b8f647adfb47"),
    "Limited Resources",
    crate::card::CardArt::new("20ae3609-a3cc-486c-94f6-b8f647adfb47", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 11 — Oath of Lieges
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OATH_OF_LIEGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("470a2092-eeda-4557-8cee-ac401b61a225"),
    "Oath of Lieges",
    crate::card::CardArt::new("470a2092-eeda-4557-8cee-ac401b61a225", "Mark Zug"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 12 — Paladin en-Vec
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PALADIN_EN_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf1ea89d-4b9d-455f-a7f4-a26026e0c272"),
    "Paladin en-Vec",
    crate::card::CardArt::new("bf1ea89d-4b9d-455f-a7f4-a26026e0c272", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 13 — Peace of Mind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PEACE_OF_MIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c383f12f-da06-4ef0-bf8e-6a8a9cfcc74c"),
    "Peace of Mind",
    crate::card::CardArt::new("c383f12f-da06-4ef0-bf8e-6a8a9cfcc74c", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 14 — Pegasus Stampede
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PEGASUS_STAMPEDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b941576-8254-4d69-85ae-c748c7921ce5"),
    "Pegasus Stampede",
    crate::card::CardArt::new("3b941576-8254-4d69-85ae-c748c7921ce5", "Mark Zug"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 15 — Penance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PENANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f3db848-8394-43bd-a236-264641033a6d"),
    "Penance",
    crate::card::CardArt::new("1f3db848-8394-43bd-a236-264641033a6d", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 16 — Reaping the Rewards
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REAPING_THE_REWARDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("379b0495-8795-4b21-9d0a-dc4e10098de2"),
    "Reaping the Rewards",
    crate::card::CardArt::new("379b0495-8795-4b21-9d0a-dc4e10098de2", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 17 — Reconnaissance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECONNAISSANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a16012d8-703c-4385-8769-13e3caba3fc6"),
    "Reconnaissance",
    crate::card::CardArt::new("a16012d8-703c-4385-8769-13e3caba3fc6", "Val Mayerik"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 18 — Shackles
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHACKLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5315668-b8ef-49ab-a8f5-144adc7bcd84"),
    "Shackles",
    crate::card::CardArt::new("c5315668-b8ef-49ab-a8f5-144adc7bcd84", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 19 — Shield Mate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_MATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b49261bb-66b5-4226-9001-02d045fbcbce"),
    "Shield Mate",
    crate::card::CardArt::new("b49261bb-66b5-4226-9001-02d045fbcbce", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 20 — Soltari Visionary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOLTARI_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a3ae384-7b60-4264-9dc1-1613917168ca"),
    "Soltari Visionary",
    crate::card::CardArt::new("1a3ae384-7b60-4264-9dc1-1613917168ca", "Adam Rex"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 21 — Soul Warden
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_WARDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5ee24ee-4d28-4634-bd43-90eff15c16dd"),
    "Soul Warden",
    crate::card::CardArt::new("d5ee24ee-4d28-4634-bd43-90eff15c16dd", "Randy Gallegos"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 22 — Standing Troops
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STANDING_TROOPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("135e258a-71d8-45dd-9307-91111aa34bde"),
    "Standing Troops",
    crate::card::CardArt::new("135e258a-71d8-45dd-9307-91111aa34bde", "Daren Bader"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 23 — Treasure Hunter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREASURE_HUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06452630-621b-498e-8f25-ecfe544d4213"),
    "Treasure Hunter",
    crate::card::CardArt::new("06452630-621b-498e-8f25-ecfe544d4213", "Adam Rex"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 24 — Wall of Nets
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_NETS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1da8e79-365d-4a36-87c5-648085828f9f"),
    "Wall of Nets",
    crate::card::CardArt::new("c1da8e79-365d-4a36-87c5-648085828f9f", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 25 — Welkin Hawk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WELKIN_HAWK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8166253c-c6ac-4b5e-9746-09ce3774c66b"),
    "Welkin Hawk",
    crate::card::CardArt::new("8166253c-c6ac-4b5e-9746-09ce3774c66b", "Rob Alexander"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 26 — Zealots en-Dal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZEALOTS_EN_DAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a9fb486-1d6a-478e-af6e-fd8539dc646d"),
    "Zealots en-Dal",
    crate::card::CardArt::new("6a9fb486-1d6a-478e-af6e-fd8539dc646d", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 27 — Aether Tide
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9aab7526-5825-4f31-92ff-be25ab5af2f5"),
    "Aether Tide",
    crate::card::CardArt::new("9aab7526-5825-4f31-92ff-be25ab5af2f5", "Andrew Robinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 28 — Cunning
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CUNNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52f36bb8-5a97-4596-8ca3-707665770c76"),
    "Cunning",
    crate::card::CardArt::new("52f36bb8-5a97-4596-8ca3-707665770c76", "Kev Walker"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 29 — Curiosity (reprint)

// EXO 30 — Dominating Licid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOMINATING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3e03323-43e8-4ddc-a874-211a97fd7648"),
    "Dominating Licid",
    crate::card::CardArt::new("e3e03323-43e8-4ddc-a874-211a97fd7648", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 31 — Ephemeron
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EPHEMERON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2cdcd3b-6df5-481a-a244-1fc2545d1356"),
    "Ephemeron",
    crate::card::CardArt::new("f2cdcd3b-6df5-481a-a244-1fc2545d1356", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 32 — Equilibrium
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EQUILIBRIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("460b2ec6-0180-4214-acca-c9eed778ef50"),
    "Equilibrium",
    crate::card::CardArt::new("460b2ec6-0180-4214-acca-c9eed778ef50", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 33 — Ertai, Wizard Adept
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_WIZARD_ADEPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91971e19-61ce-45ac-b700-9ffca5091a27"),
    "Ertai, Wizard Adept",
    crate::card::CardArt::new("91971e19-61ce-45ac-b700-9ffca5091a27", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 34 — Fade Away
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FADE_AWAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6f9103e-dcc2-4f7a-a8ca-eaa831f5f83b"),
    "Fade Away",
    crate::card::CardArt::new("a6f9103e-dcc2-4f7a-a8ca-eaa831f5f83b", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 35 — Forbid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORBID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29df5ef7-d679-4543-bdb7-3984155c87e0"),
    "Forbid",
    crate::card::CardArt::new("29df5ef7-d679-4543-bdb7-3984155c87e0", "Scott Kirschner"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 36 — Keeper of the Mind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_THE_MIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bc232d4-ab4f-4d88-a9ec-72403d05ec04"),
    "Keeper of the Mind",
    crate::card::CardArt::new("7bc232d4-ab4f-4d88-a9ec-72403d05ec04", "Matthew D. Wilson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 37 — Killer Whale
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KILLER_WHALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d932f6d3-4918-4a41-836c-4eaa6cfac049"),
    "Killer Whale",
    crate::card::CardArt::new("d932f6d3-4918-4a41-836c-4eaa6cfac049", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 38 — Mana Breach
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_BREACH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a97f019-5ad9-4520-ba79-2c9b259748d9"),
    "Mana Breach",
    crate::card::CardArt::new("3a97f019-5ad9-4520-ba79-2c9b259748d9", "Rebecca Guay"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 39 — Merfolk Looter (reprint)

// EXO 40 — Mind Over Matter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_OVER_MATTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6e091dd6-149f-46ea-bae0-224e79e3aacb"),
    "Mind Over Matter",
    crate::card::CardArt::new("6e091dd6-149f-46ea-bae0-224e79e3aacb", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 41 — Mirozel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIROZEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16417e94-e33f-4ed4-bb3e-52f29f7d441b"),
    "Mirozel",
    crate::card::CardArt::new("16417e94-e33f-4ed4-bb3e-52f29f7d441b", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 42 — Oath of Scholars
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OATH_OF_SCHOLARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d61376ad-21c8-4d34-b37d-ed60877f5d4a"),
    "Oath of Scholars",
    crate::card::CardArt::new("d61376ad-21c8-4d34-b37d-ed60877f5d4a", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 43 — Robe of Mirrors
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROBE_OF_MIRRORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("371720a2-ec3f-43a5-9551-c018e164e79f"),
    "Robe of Mirrors",
    crate::card::CardArt::new("371720a2-ec3f-43a5-9551-c018e164e79f", "John Matson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 44 — Rootwater Mystic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_MYSTIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94af81f2-383c-4129-b8dc-60633c3f4ea1"),
    "Rootwater Mystic",
    crate::card::CardArt::new("94af81f2-383c-4129-b8dc-60633c3f4ea1", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 45 — School of Piranha
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCHOOL_OF_PIRANHA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71217af5-3538-4e42-9343-3949b5306671"),
    "School of Piranha",
    crate::card::CardArt::new("71217af5-3538-4e42-9343-3949b5306671", "Daren Bader"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 46 — Scrivener
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCRIVENER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b236bba-160a-4637-a83e-8456834ce59f"),
    "Scrivener",
    crate::card::CardArt::new("8b236bba-160a-4637-a83e-8456834ce59f", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 47 — Thalakos Drifters
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THALAKOS_DRIFTERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("468e13d2-6bd7-403c-8e2e-e00917b39597"),
    "Thalakos Drifters",
    crate::card::CardArt::new("468e13d2-6bd7-403c-8e2e-e00917b39597", "Andrew Robinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 48 — Thalakos Scout
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THALAKOS_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1703fe9d-ca70-4e8a-9d6a-6173a17d0f04"),
    "Thalakos Scout",
    crate::card::CardArt::new("1703fe9d-ca70-4e8a-9d6a-6173a17d0f04", "John Matson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 49 — Theft of Dreams
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THEFT_OF_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29019e28-4ef8-4732-9972-0a47305fe303"),
    "Theft of Dreams",
    crate::card::CardArt::new(
        "099da8aa-16b1-4395-8467-1636feb14a8a",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 50 — Treasure Trove
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREASURE_TROVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f23ce909-e744-47ca-943d-62d97e97b1ea"),
    "Treasure Trove",
    crate::card::CardArt::new("f23ce909-e744-47ca-943d-62d97e97b1ea", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 51 — Wayward Soul
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WAYWARD_SOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28f96d5d-1d16-40bb-aaa7-8a7dd465d37b"),
    "Wayward Soul",
    crate::card::CardArt::new(
        "28f96d5d-1d16-40bb-aaa7-8a7dd465d37b",
        "M. W. Kaluta & DiTerlizzi",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 52 — Whiptongue Frog
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPTONGUE_FROG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fc17186-e786-46a3-9812-4a6e367e78b9"),
    "Whiptongue Frog",
    crate::card::CardArt::new("6fc17186-e786-46a3-9812-4a6e367e78b9", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 53 — Carnophage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARNOPHAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d17c057f-cb1b-4895-831a-fb35c75d3845"),
    "Carnophage",
    crate::card::CardArt::new("d17c057f-cb1b-4895-831a-fb35c75d3845", "Pete Venters"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 54 — Cat Burglar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAT_BURGLAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("947109f9-7035-4a2a-bbc2-a2958f8c5d01"),
    "Cat Burglar",
    crate::card::CardArt::new("947109f9-7035-4a2a-bbc2-a2958f8c5d01", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 55 — Culling the Weak
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CULLING_THE_WEAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50c33f18-0a5c-4e46-ab0d-6e450915594f"),
    "Culling the Weak",
    crate::card::CardArt::new("50c33f18-0a5c-4e46-ab0d-6e450915594f", "Scott M. Fischer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 56 — Cursed Flesh
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CURSED_FLESH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7433b9bf-ee6e-41fe-b826-0d20584198b1"),
    "Cursed Flesh",
    crate::card::CardArt::new("7433b9bf-ee6e-41fe-b826-0d20584198b1", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 57 — Dauthi Cutthroat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_CUTTHROAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("127b8994-fff8-4500-8ab4-244eeb3ed110"),
    "Dauthi Cutthroat",
    crate::card::CardArt::new("127b8994-fff8-4500-8ab4-244eeb3ed110", "Dermot Power"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 58 — Dauthi Jackal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_JACKAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("419871bc-f036-4244-8b6c-3857ebe993f3"),
    "Dauthi Jackal",
    crate::card::CardArt::new("419871bc-f036-4244-8b6c-3857ebe993f3", "Adam Rex"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 59 — Dauthi Warlord
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_WARLORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af3ca689-482a-457d-9744-0bd79981f361"),
    "Dauthi Warlord",
    crate::card::CardArt::new("af3ca689-482a-457d-9744-0bd79981f361", "Kev Walker"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 60 — Death's Duet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_S_DUET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4756b6fd-2bb2-4be1-9b02-851a26ff4303"),
    "Death's Duet",
    crate::card::CardArt::new("4756b6fd-2bb2-4be1-9b02-851a26ff4303", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 61 — Entropic Specter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENTROPIC_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdb04d81-b0ab-4bc7-935d-c31005887240"),
    "Entropic Specter",
    crate::card::CardArt::new("bdb04d81-b0ab-4bc7-935d-c31005887240", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 62 — Fugue
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FUGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1629cd63-95aa-40b6-aa57-7fb88f569e59"),
    "Fugue",
    crate::card::CardArt::new("1629cd63-95aa-40b6-aa57-7fb88f569e59", "Randy Gallegos"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 63 — Grollub
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GROLLUB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47f6301a-d581-4aaf-9993-3013323074aa"),
    "Grollub",
    crate::card::CardArt::new("47f6301a-d581-4aaf-9993-3013323074aa", "Chippy"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 64 — Hatred
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HATRED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2383a8d9-96fd-4f9a-bcf9-eb81fdb15ead"),
    "Hatred",
    crate::card::CardArt::new("2383a8d9-96fd-4f9a-bcf9-eb81fdb15ead", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 65 — Keeper of the Dead
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b641171-35bc-4945-ada9-3ea28ea9fabf"),
    "Keeper of the Dead",
    crate::card::CardArt::new("6b641171-35bc-4945-ada9-3ea28ea9fabf", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 66 — Mind Maggots
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_MAGGOTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3c92a7f-a250-4497-aa7a-0394e94ef13d"),
    "Mind Maggots",
    crate::card::CardArt::new("c3c92a7f-a250-4497-aa7a-0394e94ef13d", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 67 — Nausea
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NAUSEA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a10531d8-fc99-4a2b-94b0-97a25521d725"),
    "Nausea",
    crate::card::CardArt::new("a10531d8-fc99-4a2b-94b0-97a25521d725", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 68 — Necrologia
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NECROLOGIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c2ee9d9-20be-46f0-8752-1df50942f59c"),
    "Necrologia",
    crate::card::CardArt::new("8c2ee9d9-20be-46f0-8752-1df50942f59c", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 69 — Oath of Ghouls
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OATH_OF_GHOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1102f35a-ae62-479d-b61c-31a82978aedd"),
    "Oath of Ghouls",
    crate::card::CardArt::new("1102f35a-ae62-479d-b61c-31a82978aedd", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 70 — Pit Spawn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PIT_SPAWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("669ad60b-4053-4f07-9072-52e6ff65b4e3"),
    "Pit Spawn",
    crate::card::CardArt::new("669ad60b-4053-4f07-9072-52e6ff65b4e3", "Thomas M. Baxa"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 71 — Plaguebearer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUEBEARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8493df6-9954-4e33-867c-ca4bcf3953b2"),
    "Plaguebearer",
    crate::card::CardArt::new("a8493df6-9954-4e33-867c-ca4bcf3953b2", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 72 — Recurring Nightmare
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECURRING_NIGHTMARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8173030-1c33-417c-b8e9-79231b6a85a7"),
    "Recurring Nightmare",
    crate::card::CardArt::new("c8173030-1c33-417c-b8e9-79231b6a85a7", "Jeff Laubenstein"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 73 — Scare Tactics
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCARE_TACTICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a9d4e11-ce2e-445a-9536-756a6687d6d7"),
    "Scare Tactics",
    crate::card::CardArt::new("6a9d4e11-ce2e-445a-9536-756a6687d6d7", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 74 — Slaughter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLAUGHTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ff06c7d-5e78-4bcf-864b-34487f6555b2"),
    "Slaughter",
    crate::card::CardArt::new("8ff06c7d-5e78-4bcf-864b-34487f6555b2", "Pete Venters"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 75 — Spike Cannibal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKE_CANNIBAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64003772-c62f-4728-a00c-48c78991c6ae"),
    "Spike Cannibal",
    crate::card::CardArt::new("64003772-c62f-4728-a00c-48c78991c6ae", "Joel Biske"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 76 — Thrull Surgeon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRULL_SURGEON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6e89bf1-42c9-4829-a565-78cac632810b"),
    "Thrull Surgeon",
    crate::card::CardArt::new("d6e89bf1-42c9-4829-a565-78cac632810b", "rk post"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 77 — Vampire Hounds
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_HOUNDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("746bc301-9f08-4d9b-819e-690f6fce6bc8"),
    "Vampire Hounds",
    crate::card::CardArt::new("746bc301-9f08-4d9b-819e-690f6fce6bc8", "Kev Walker"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 78 — Volrath's Dungeon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_S_DUNGEON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4ab28e1-74e1-4c4e-920f-a658c6a44d75"),
    "Volrath's Dungeon",
    crate::card::CardArt::new("a4ab28e1-74e1-4c4e-920f-a658c6a44d75", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 79 — Anarchist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANARCHIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a298df66-2075-40a7-bced-457656b6b788"),
    "Anarchist",
    crate::card::CardArt::new("a298df66-2075-40a7-bced-457656b6b788", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 80 — Cinder Crawler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9406050-d76b-4569-a463-e21acaf84166"),
    "Cinder Crawler",
    crate::card::CardArt::new("a9406050-d76b-4569-a463-e21acaf84166", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 81 — Dizzying Gaze
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DIZZYING_GAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71a482cf-a1cd-47b5-a76a-08e03965c679"),
    "Dizzying Gaze",
    crate::card::CardArt::new("71a482cf-a1cd-47b5-a76a-08e03965c679", "Thomas M. Baxa"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 82 — Fighting Chance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIGHTING_CHANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca75f6e9-5eee-4904-88c0-71ec730a0f23"),
    "Fighting Chance",
    crate::card::CardArt::new("ca75f6e9-5eee-4904-88c0-71ec730a0f23", "Mike Raabe"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 83 — Flowstone Flood
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_FLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8bcda003-6fac-4879-87e6-ec0c115630ba"),
    "Flowstone Flood",
    crate::card::CardArt::new("8bcda003-6fac-4879-87e6-ec0c115630ba", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 84 — Furnace Brood
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FURNACE_BROOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a79c6d9-96f1-434a-89b8-d773aa77ac5e"),
    "Furnace Brood",
    crate::card::CardArt::new("0a79c6d9-96f1-434a-89b8-d773aa77ac5e", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 85 — Keeper of the Flame
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_THE_FLAME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bf246ca-9dfc-400f-8883-acc80ac016e1"),
    "Keeper of the Flame",
    crate::card::CardArt::new("9bf246ca-9dfc-400f-8883-acc80ac016e1", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 86 — Mage il-Vec
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAGE_IL_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04e3e38b-2191-4b92-ae5d-bb9397d24a27"),
    "Mage il-Vec",
    crate::card::CardArt::new("04e3e38b-2191-4b92-ae5d-bb9397d24a27", "John Matson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 87 — Maniacal Rage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANIACAL_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3aa840f-6a70-4674-acb7-ded0ea4397d8"),
    "Maniacal Rage",
    crate::card::CardArt::new("f3aa840f-6a70-4674-acb7-ded0ea4397d8", "Pete Venters"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 88 — Mogg Assassin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1124725d-e643-43a1-873e-255636c7f334"),
    "Mogg Assassin",
    crate::card::CardArt::new("1124725d-e643-43a1-873e-255636c7f334", "Dermot Power"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 89 — Monstrous Hound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MONSTROUS_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ae4162d-e080-4db1-912f-d53674c76170"),
    "Monstrous Hound",
    crate::card::CardArt::new("d5066b1b-3910-4434-83d6-030851f20bcf", "Dermot Power"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 90 — Oath of Mages
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OATH_OF_MAGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed8708d2-2c73-4da5-b6ff-41c083b59caa"),
    "Oath of Mages",
    crate::card::CardArt::new("ed8708d2-2c73-4da5-b6ff-41c083b59caa", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 91 — Ogre Shaman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb3224ac-9b60-48cf-9734-86768fd370ac"),
    "Ogre Shaman",
    crate::card::CardArt::new("cb3224ac-9b60-48cf-9734-86768fd370ac", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 92 — Onslaught
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ONSLAUGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0afaf142-dbca-45bf-aea2-01c53bda635a"),
    "Onslaught",
    crate::card::CardArt::new("0afaf142-dbca-45bf-aea2-01c53bda635a", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 93 — Pandemonium
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PANDEMONIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f618231-28bb-4cdd-b887-a8aa186814d5"),
    "Pandemonium",
    crate::card::CardArt::new("5f618231-28bb-4cdd-b887-a8aa186814d5", "Pete Venters"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 94 — Paroxysm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PAROXYSM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53320321-4f02-40ee-8171-2375b1d4ed66"),
    "Paroxysm",
    crate::card::CardArt::new("53320321-4f02-40ee-8171-2375b1d4ed66", "Scott Kirschner"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 95 — Price of Progress
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRICE_OF_PROGRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e5283db-3e22-4862-9d95-56d03d09c2ae"),
    "Price of Progress",
    crate::card::CardArt::new(
        "8e5283db-3e22-4862-9d95-56d03d09c2ae",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 96 — Raging Goblin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c0fa444-5534-4476-8bfa-78b2364f2dd3"),
    "Raging Goblin",
    crate::card::CardArt::new("1f0a166c-f7c0-45b4-aa90-053ce545cfb2", "Brian Snõddy"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 97 — Ravenous Baboons
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAVENOUS_BABOONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d00b68b-8b6a-48c9-8911-2a3270897091"),
    "Ravenous Baboons",
    crate::card::CardArt::new("6d00b68b-8b6a-48c9-8911-2a3270897091", "Daren Bader"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 98 — Reckless Ogre
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90d27b79-a22d-48d9-86b2-7ad02cab8697"),
    "Reckless Ogre",
    crate::card::CardArt::new("90d27b79-a22d-48d9-86b2-7ad02cab8697", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 99 — Sabertooth Wyvern
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SABERTOOTH_WYVERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84c1d384-d341-4bab-bf71-5dbcf76d51e8"),
    "Sabertooth Wyvern",
    crate::card::CardArt::new("84c1d384-d341-4bab-bf71-5dbcf76d51e8", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 100 — Scalding Salamander
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCALDING_SALAMANDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a0e9433-88d7-4bfc-99a0-ff47807fd594"),
    "Scalding Salamander",
    crate::card::CardArt::new("5a0e9433-88d7-4bfc-99a0-ff47807fd594", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 101 — Seismic Assault
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEISMIC_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc494af5-4da4-43f5-a193-426ef84d80a7"),
    "Seismic Assault",
    crate::card::CardArt::new("cc494af5-4da4-43f5-a193-426ef84d80a7", "Dermot Power"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 102 — Shattering Pulse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTERING_PULSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89d3b846-6071-4d65-86ba-da08c4bd0aa1"),
    "Shattering Pulse",
    crate::card::CardArt::new("89d3b846-6071-4d65-86ba-da08c4bd0aa1", "Donato Giancola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 103 — Sonic Burst
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SONIC_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05530d5a-dcb6-403e-9e35-224c7b5cf615"),
    "Sonic Burst",
    crate::card::CardArt::new("05530d5a-dcb6-403e-9e35-224c7b5cf615", "Brian Snõddy"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 104 — Spellshock
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLSHOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52db2a78-e1c5-4732-a4ee-04b4c540edbe"),
    "Spellshock",
    crate::card::CardArt::new("52db2a78-e1c5-4732-a4ee-04b4c540edbe", "Thomas M. Baxa"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 105 — Avenging Druid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGING_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fca9fd31-639a-4fbc-84bd-c3078df29c0a"),
    "Avenging Druid",
    crate::card::CardArt::new("fca9fd31-639a-4fbc-84bd-c3078df29c0a", "Daren Bader"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 106 — Bequeathal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BEQUEATHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20aae577-9683-4d9b-bfd5-52702b38d3a7"),
    "Bequeathal",
    crate::card::CardArt::new(
        "20aae577-9683-4d9b-bfd5-52702b38d3a7",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 107 — Cartographer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARTOGRAPHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f2c2cc9-37ce-435e-9df2-083d5e3c8c5c"),
    "Cartographer",
    crate::card::CardArt::new("7f2c2cc9-37ce-435e-9df2-083d5e3c8c5c", "Jeff Laubenstein"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 108 — Crashing Boars
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRASHING_BOARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2965bd5-4f16-443a-9133-adb92cf0e12b"),
    "Crashing Boars",
    crate::card::CardArt::new("a2965bd5-4f16-443a-9133-adb92cf0e12b", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 109 — Elven Palisade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELVEN_PALISADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b990ffe5-fd2a-4646-bac3-8e52cdc328aa"),
    "Elven Palisade",
    crate::card::CardArt::new("b990ffe5-fd2a-4646-bac3-8e52cdc328aa", "Mark Zug"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 110 — Elvish Berserker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_BERSERKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfa69a8e-1b75-4d93-918d-d772cec69e99"),
    "Elvish Berserker",
    crate::card::CardArt::new("dfa69a8e-1b75-4d93-918d-d772cec69e99", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 111 — Jackalope Herd
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JACKALOPE_HERD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb80105c-d2c0-4f8c-9302-5e6152a60f54"),
    "Jackalope Herd",
    crate::card::CardArt::new("cb80105c-d2c0-4f8c-9302-5e6152a60f54", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 112 — Keeper of the Beasts
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_THE_BEASTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cccccf78-8b00-406b-a2b7-0e6ba76703d0"),
    "Keeper of the Beasts",
    crate::card::CardArt::new("cccccf78-8b00-406b-a2b7-0e6ba76703d0", "rk post"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 113 — Manabond
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANABOND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("212ca7e7-5ba3-4da7-a2f0-16c721004bac"),
    "Manabond",
    crate::card::CardArt::new("212ca7e7-5ba3-4da7-a2f0-16c721004bac", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 114 — Mirri, Cat Warrior
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRI_CAT_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d1682dd-5a99-4bee-a2c2-c8735047e1a9"),
    "Mirri, Cat Warrior",
    crate::card::CardArt::new("6d1682dd-5a99-4bee-a2c2-c8735047e1a9", "Daren Bader"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 115 — Oath of Druids
static MILL_UNTIL_1: MillUntilDef = MillUntilDef {
    player: EffectRecipientDef::EventPlayer,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    matched_zone: ZoneKind::Battlefield,
    binding: None,
    then: None,
};

static CREATURES_THE_UPKEEP_PLAYER_CONTROLS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::EventPlayer,
);

static CREATURES_THEIR_OPPONENT_CONTROLS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::NotEventPlayer,
);

static THE_UPKEEP_PLAYER_IS_BEHIND: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::CountMatchingObjects(&CREATURES_THE_UPKEEP_PLAYER_CONTROLS),
    comparison: ComparisonDef::Less,
    right: ValueDef::CountMatchingObjects(&CREATURES_THEIR_OPPONENT_CONTROLS),
};

static OATH_CONDITION: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&THE_UPKEEP_PLAYER_IS_BEHIND);

static OATH_DIGS_FOR_A_CREATURE: EffectDef = EffectDef::MillUntil(&MILL_UNTIL_1);

pub(in crate::card::sets) static OATH_OF_DRUIDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf14de50-d123-400c-862e-2c95fd2aa23f"),
    "Oath of Druids",
    CardArt::new("cf14de50-d123-400c-862e-2c95fd2aa23f", "Daren Bader"),
    CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of each player's upkeep, that player chooses target player who \
             controls more creatures than they do and is their opponent. The first player may \
             reveal cards from the top of their library until they reveal a creature card. If the \
             first player does, that player puts that card onto the battlefield and all other \
             cards revealed this way into their graveyard.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            &OATH_CONDITION,
            EffectDef::May {
                player: EffectRecipientDef::EventPlayer,
                effect: &OATH_DIGS_FOR_A_CREATURE,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The ability does not target. In a two-player game the printed target has exactly one \
             candidate and its legality is the condition checked here, so what happens is the \
             same -- but nothing that answers targeting sees this ability.",
        )),
    ),
);

// EXO 116 — Plated Rootwalla
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLATED_ROOTWALLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4bf4da70-c656-4e40-bb0f-68e9dda024c9"),
    "Plated Rootwalla",
    crate::card::CardArt::new("4bf4da70-c656-4e40-bb0f-68e9dda024c9", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 117 — Predatory Hunger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PREDATORY_HUNGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db6d9d28-3a05-4dfa-a322-36b4cc2697d4"),
    "Predatory Hunger",
    crate::card::CardArt::new("db6d9d28-3a05-4dfa-a322-36b4cc2697d4", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 118 — Pygmy Troll
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7be9714d-125f-4700-879d-b920fe9f1b68"),
    "Pygmy Troll",
    crate::card::CardArt::new("7be9714d-125f-4700-879d-b920fe9f1b68", "Daniel Gelon"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 119 — Rabid Wolverines
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RABID_WOLVERINES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99121a2b-c735-47be-b01e-cdf59809e7f3"),
    "Rabid Wolverines",
    crate::card::CardArt::new("99121a2b-c735-47be-b01e-cdf59809e7f3", "Daren Bader"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 120 — Reclaim (reprint)

// EXO 121 — Resuscitate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RESUSCITATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5707560-fcc6-4aca-adce-d41de45f37e8"),
    "Resuscitate",
    crate::card::CardArt::new("f5707560-fcc6-4aca-adce-d41de45f37e8", "Rebecca Guay"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 122 — Rootwater Alligator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_ALLIGATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a840bba-4725-45fd-885f-1b3d615dfa97"),
    "Rootwater Alligator",
    crate::card::CardArt::new("3a840bba-4725-45fd-885f-1b3d615dfa97", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 123 — Skyshroud Elite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6a496a4-1b4c-4c5d-99e5-ec40601c759d"),
    "Skyshroud Elite",
    crate::card::CardArt::new("f6a496a4-1b4c-4c5d-99e5-ec40601c759d", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 124 — Skyshroud War Beast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_WAR_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19d809c1-e674-40b8-816d-c45d77c66722"),
    "Skyshroud War Beast",
    crate::card::CardArt::new("19d809c1-e674-40b8-816d-c45d77c66722", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 125 — Song of Serenity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SONG_OF_SERENITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ba85b2f-37da-4595-9880-8e9f1ddbac09"),
    "Song of Serenity",
    crate::card::CardArt::new("2ba85b2f-37da-4595-9880-8e9f1ddbac09", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 126 — Spike Hatcher
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKE_HATCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f79fb79-37a0-483f-ba19-853cbfffc73d"),
    "Spike Hatcher",
    crate::card::CardArt::new("1f79fb79-37a0-483f-ba19-853cbfffc73d", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 127 — Spike Rogue
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKE_ROGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0d9b671-344b-460d-8f65-d65129db91c3"),
    "Spike Rogue",
    crate::card::CardArt::new("f0d9b671-344b-460d-8f65-d65129db91c3", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 128 — Spike Weaver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKE_WEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c561a2a-91c6-4d4b-9f96-bffd43a00478"),
    "Spike Weaver",
    crate::card::CardArt::new("9c561a2a-91c6-4d4b-9f96-bffd43a00478", "Mike Raabe"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 129 — Survival of the Fittest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SURVIVAL_OF_THE_FITTEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c060c178-3c0e-493f-b6f0-ead5b1d6f191"),
    "Survival of the Fittest",
    crate::card::CardArt::new("c060c178-3c0e-493f-b6f0-ead5b1d6f191", "Pete Venters"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 130 — Wood Elves
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WOOD_ELVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7f1fb90-5c85-46a5-802d-248cc0250921"),
    "Wood Elves",
    crate::card::CardArt::new("4716bb55-0821-4809-9bc0-04e299b09549", "Rebecca Guay"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 131 — Coat of Arms
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COAT_OF_ARMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e32c939-1d64-4082-bafe-59dfa9c054f6"),
    "Coat of Arms",
    crate::card::CardArt::new("9e32c939-1d64-4082-bafe-59dfa9c054f6", "Scott M. Fischer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 132 — Erratic Portal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERRATIC_PORTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e652007-02f0-424f-b52c-c1540d1939bd"),
    "Erratic Portal",
    crate::card::CardArt::new("2e652007-02f0-424f-b52c-c1540d1939bd", "John Matson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 133 — Medicine Bag
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MEDICINE_BAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("399c06d5-af2a-47a1-9239-ff14224a026b"),
    "Medicine Bag",
    crate::card::CardArt::new("399c06d5-af2a-47a1-9239-ff14224a026b", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 134 — Memory Crystal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MEMORY_CRYSTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c786ea5b-52ad-4d1b-855e-ce6d0b9af67e"),
    "Memory Crystal",
    crate::card::CardArt::new("c786ea5b-52ad-4d1b-855e-ce6d0b9af67e", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 135 — Mindless Automaton
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MINDLESS_AUTOMATON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ddfc5ab-b11b-4ad7-ab46-8ee60d938a5b"),
    "Mindless Automaton",
    crate::card::CardArt::new("6ddfc5ab-b11b-4ad7-ab46-8ee60d938a5b", "Brian Snõddy"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 136 — Null Brooch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NULL_BROOCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5d5a0c6-916c-428a-ae66-8adc8844e56e"),
    "Null Brooch",
    crate::card::CardArt::new("d5d5a0c6-916c-428a-ae66-8adc8844e56e", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 137 — Skyshaper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("234ed934-6ea7-41f6-bd13-3df8662a3a1d"),
    "Skyshaper",
    crate::card::CardArt::new("234ed934-6ea7-41f6-bd13-3df8662a3a1d", "Donato Giancola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 138 — Spellbook
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLBOOK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33fb104c-f8ca-4da2-8f1f-8fe6f291407e"),
    "Spellbook",
    crate::card::CardArt::new("33fb104c-f8ca-4da2-8f1f-8fe6f291407e", "Ciruelo"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 139 — Sphere of Resistance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPHERE_OF_RESISTANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17f4d2a5-bb85-4662-b2dd-a363ec7eab9b"),
    "Sphere of Resistance",
    crate::card::CardArt::new("17f4d2a5-bb85-4662-b2dd-a363ec7eab9b", "Doug Chaffee"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 140 — Thopter Squadron
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THOPTER_SQUADRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3ac2d30-7c9a-40b3-812e-e77e49229f48"),
    "Thopter Squadron",
    crate::card::CardArt::new("d3ac2d30-7c9a-40b3-812e-e77e49229f48", "Doug Chaffee"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 141 — Transmogrifying Licid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRANSMOGRIFYING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a115563-81da-42f6-95c4-22ae7bb51a0f"),
    "Transmogrifying Licid",
    crate::card::CardArt::new("1a115563-81da-42f6-95c4-22ae7bb51a0f", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 142 — Workhorse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WORKHORSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2571ff7-0287-4ba2-8365-5ff08de641a2"),
    "Workhorse",
    crate::card::CardArt::new("c2571ff7-0287-4ba2-8365-5ff08de641a2", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 143 — City of Traitors
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CITY_OF_TRAITORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7a8b6b8-b95f-4014-b17a-a6d44d965995"),
    "City of Traitors",
    crate::card::CardArt::new("a7a8b6b8-b95f-4014-b17a-a6d44d965995", "Kev Walker"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALLAY,
    &ANGELIC_BLESSING,
    &CATACLYSM,
    &CHARGING_PALADIN,
    &CONVALESCENCE,
    &EXALTED_DRAGON,
    &HIGH_GROUND,
    &KEEPER_OF_THE_LIGHT,
    &KOR_CHANT,
    &LIMITED_RESOURCES,
    &OATH_OF_LIEGES,
    &PALADIN_EN_VEC,
    &PEACE_OF_MIND,
    &PEGASUS_STAMPEDE,
    &PENANCE,
    &REAPING_THE_REWARDS,
    &RECONNAISSANCE,
    &SHACKLES,
    &SHIELD_MATE,
    &SOLTARI_VISIONARY,
    &SOUL_WARDEN,
    &STANDING_TROOPS,
    &TREASURE_HUNTER,
    &WALL_OF_NETS,
    &WELKIN_HAWK,
    &ZEALOTS_EN_DAL,
    &AETHER_TIDE,
    &CUNNING,
    &DOMINATING_LICID,
    &EPHEMERON,
    &EQUILIBRIUM,
    &ERTAI_WIZARD_ADEPT,
    &FADE_AWAY,
    &FORBID,
    &KEEPER_OF_THE_MIND,
    &KILLER_WHALE,
    &MANA_BREACH,
    &MIND_OVER_MATTER,
    &MIROZEL,
    &OATH_OF_SCHOLARS,
    &ROBE_OF_MIRRORS,
    &ROOTWATER_MYSTIC,
    &SCHOOL_OF_PIRANHA,
    &SCRIVENER,
    &THALAKOS_DRIFTERS,
    &THALAKOS_SCOUT,
    &THEFT_OF_DREAMS,
    &TREASURE_TROVE,
    &WAYWARD_SOUL,
    &WHIPTONGUE_FROG,
    &CARNOPHAGE,
    &CAT_BURGLAR,
    &CULLING_THE_WEAK,
    &CURSED_FLESH,
    &DAUTHI_CUTTHROAT,
    &DAUTHI_JACKAL,
    &DAUTHI_WARLORD,
    &DEATH_S_DUET,
    &ENTROPIC_SPECTER,
    &FUGUE,
    &GROLLUB,
    &HATRED,
    &KEEPER_OF_THE_DEAD,
    &MIND_MAGGOTS,
    &NAUSEA,
    &NECROLOGIA,
    &OATH_OF_GHOULS,
    &PIT_SPAWN,
    &PLAGUEBEARER,
    &RECURRING_NIGHTMARE,
    &SCARE_TACTICS,
    &SLAUGHTER,
    &SPIKE_CANNIBAL,
    &THRULL_SURGEON,
    &VAMPIRE_HOUNDS,
    &VOLRATH_S_DUNGEON,
    &ANARCHIST,
    &CINDER_CRAWLER,
    &DIZZYING_GAZE,
    &FIGHTING_CHANCE,
    &FLOWSTONE_FLOOD,
    &FURNACE_BROOD,
    &KEEPER_OF_THE_FLAME,
    &MAGE_IL_VEC,
    &MANIACAL_RAGE,
    &MOGG_ASSASSIN,
    &MONSTROUS_HOUND,
    &OATH_OF_MAGES,
    &OGRE_SHAMAN,
    &ONSLAUGHT,
    &PANDEMONIUM,
    &PAROXYSM,
    &PRICE_OF_PROGRESS,
    &RAGING_GOBLIN,
    &RAVENOUS_BABOONS,
    &RECKLESS_OGRE,
    &SABERTOOTH_WYVERN,
    &SCALDING_SALAMANDER,
    &SEISMIC_ASSAULT,
    &SHATTERING_PULSE,
    &SONIC_BURST,
    &SPELLSHOCK,
    &AVENGING_DRUID,
    &BEQUEATHAL,
    &CARTOGRAPHER,
    &CRASHING_BOARS,
    &ELVEN_PALISADE,
    &ELVISH_BERSERKER,
    &JACKALOPE_HERD,
    &KEEPER_OF_THE_BEASTS,
    &MANABOND,
    &MIRRI_CAT_WARRIOR,
    &OATH_OF_DRUIDS,
    &PLATED_ROOTWALLA,
    &PREDATORY_HUNGER,
    &PYGMY_TROLL,
    &RABID_WOLVERINES,
    &RESUSCITATE,
    &ROOTWATER_ALLIGATOR,
    &SKYSHROUD_ELITE,
    &SKYSHROUD_WAR_BEAST,
    &SONG_OF_SERENITY,
    &SPIKE_HATCHER,
    &SPIKE_ROGUE,
    &SPIKE_WEAVER,
    &SURVIVAL_OF_THE_FITTEST,
    &WOOD_ELVES,
    &COAT_OF_ARMS,
    &ERRATIC_PORTAL,
    &MEDICINE_BAG,
    &MEMORY_CRYSTAL,
    &MINDLESS_AUTOMATON,
    &NULL_BROOCH,
    &SKYSHAPER,
    &SPELLBOOK,
    &SPHERE_OF_RESISTANCE,
    &THOPTER_SQUADRON,
    &TRANSMOGRIFYING_LICID,
    &WORKHORSE,
    &CITY_OF_TRAITORS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_isd::CURIOSITY), // EXO 29
    PrintingRecord::reprint(&catalog_m12::MERFOLK_LOOTER), // EXO 39
    PrintingRecord::reprint(&catalog_m12::RECLAIM),   // EXO 120
];
