//! Mercadian Masques cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::ZonePlacement;
use crate::card::CostQuantityDef;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1996::alliances as catalog_all;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardRules, CardSet, CardSupertype, CardType,
    ComparisonDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    TriggerConditionDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// MMQ 1 — Afterlife (reprint)
const AFTERLIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::AFTERLIFE,
    "8fa2ecf9-b53c-4f1d-9028-ca3820d043cb",
    "Brian Snõddy",
);

// MMQ 2 — Alabaster Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Alabaster Wall",
    "9cf393a3-831e-4d3a-8404-ee83f60970aa",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MMQ 3 — Armistice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMISTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Armistice",
    "1eb4402a-f263-4f82-b4c0-cf0aa58dc946",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MMQ 4 — Arrest
pub(in crate::card::sets) static ARREST: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Arrest",
    "3b083fd8-6422-4cd3-a27d-41b6d88598c2",
    "Dan Frazier",
    // The creature keeps its triggered and static abilities: only the
    // activations are shut off.
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack or block, and its activated abilities can't be \
                 activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Three prohibitions, applied together for the same duration, so the Aura
                    // leaving gives all three back at once.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
                    ]),
                },
            ),
        ]),
);

// MMQ 5 — Ballista Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALLISTA_SQUAD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ballista Squad",
    "30d51d84-23d2-41ff-ab68-a633beddba06",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// MMQ 6 — Charm Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARM_PEDDLER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Charm Peddler",
    "082e6ee3-cc1f-46c7-9d82-56751478b3cf",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MMQ 7 — Charmed Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARMED_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Charmed Griffin",
    "66d36960-3c78-4032-9325-8002b2a48503",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// MMQ 8 — Cho-Arrim Alchemist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_ARRIM_ALCHEMIST: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cho-Arrim Alchemist",
    "42c9d49d-61eb-4f33-b06b-03bdd990efd0",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 9 — Cho-Arrim Bruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_ARRIM_BRUISER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cho-Arrim Bruiser",
    "26e98f06-ad8d-4a93-8ae6-3da42b63b5b5",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MMQ 10 — Cho-Arrim Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_ARRIM_LEGATE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cho-Arrim Legate",
    "1427a3a1-24e1-4697-b5eb-1c0a24f89e75",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// MMQ 11 — Cho-Manno, Revolutionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_MANNO_REVOLUTIONARY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cho-Manno, Revolutionary",
    "3dc51393-de63-4ce3-ab02-c695e4448018",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 12 — Cho-Manno's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_MANNO_S_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cho-Manno's Blessing",
    "5c9f33c6-5294-4584-854d-c8c0f847aba8",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MMQ 13 — Common Cause
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMON_CAUSE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Common Cause",
    "eae4c25f-2005-4ac0-a5f0-2fc250520995",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MMQ 14 — Cornered Market
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORNERED_MARKET: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cornered Market",
    "0d4f3c1d-d25e-4263-ab2b-19534c852678",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// MMQ 15 — Crackdown
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRACKDOWN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Crackdown",
    "a7009fd8-1d80-41bb-a1b0-fea9c909c63d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 16 — Crossbow Infantry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSSBOW_INFANTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Crossbow Infantry",
    "744c2177-3140-48a1-95a4-2f0a27ca5b2f",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 17 — Devout Witness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOUT_WITNESS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Devout Witness",
    "48ca7aeb-09db-4409-9ba2-c5c5500ad72f",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// MMQ 18 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "366407d8-3ed9-4809-b9bb-388ebb9ea815",
    "Adam Rex",
);

// MMQ 19 — Fountain Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUNTAIN_WATCH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Fountain Watch",
    "690daa19-1842-4605-9bda-bf67e4ede3c4",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 20 — Fresh Volunteers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRESH_VOLUNTEERS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Fresh Volunteers",
    "e070ea4a-c417-405f-b788-78fb7ca2eaa5",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 21 — Honor the Fallen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONOR_THE_FALLEN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Honor the Fallen",
    "70147617-10e0-413d-be0a-a888b9cb6b97",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 22 — Ignoble Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IGNOBLE_SOLDIER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ignoble Soldier",
    "676a2506-17f8-4b8e-be0c-eacc0fe972f6",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// MMQ 23 — Inviolability
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVIOLABILITY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Inviolability",
    "9ece8504-389a-43e3-b178-7067722c4b75",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// MMQ 24 — Ivory Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IVORY_MASK: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ivory Mask",
    "35ea3762-a419-412c-b2bd-0a40902d8d51",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// MMQ 25 — Jhovall Queen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JHOVALL_QUEEN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Jhovall Queen",
    "b8eb55cc-ddde-4f15-9262-b9aee28059d3",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// MMQ 26 — Jhovall Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JHOVALL_RIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Jhovall Rider",
    "7e1f7c51-0011-4ea5-b123-3c26293f5dab",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 27 — Last Breath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_BREATH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Last Breath",
    "3b540da2-f8c6-48d6-af6d-db78958f0a17",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// MMQ 28 — Moment of Silence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENT_OF_SILENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Moment of Silence",
    "3f50b5f5-8dac-4785-b1af-a0bd64ce7a92",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// MMQ 29 — Moonlit Wake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONLIT_WAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Moonlit Wake",
    "1eba9595-6789-4d7a-9e46-8d1f75993b21",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 30 — Muzzle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUZZLE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Muzzle",
    "8b3048ec-bcbf-4a69-b56f-83bbe82b68e5",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// MMQ 31 — Nightwind Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTWIND_GLIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Nightwind Glider",
    "0968401d-522f-4def-92a1-d504471ac54e",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MMQ 32 — Noble Purpose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_PURPOSE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Noble Purpose",
    "ad5ff149-8516-456e-af8a-3dea78715acb",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MMQ 33 — Orim's Cure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_CURE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Orim's Cure",
    "754ae359-363b-456a-bbca-52fbfbaa86b8",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// MMQ 34 — Pious Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIOUS_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Pious Warrior",
    "bc20c1f0-9883-484c-88d8-1cab08d0b210",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 35 — Ramosian Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_CAPTAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ramosian Captain",
    "0e9d2e2a-c608-4787-bbd9-e1871f681b58",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// MMQ 36 — Ramosian Commander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_COMMANDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ramosian Commander",
    "867f5d82-71c2-455f-ab16-5a32bba46986",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// MMQ 37 — Ramosian Lieutenant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_LIEUTENANT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ramosian Lieutenant",
    "debe840a-ebc9-43c4-9bf7-7eb292b65bf9",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 38 — Ramosian Rally
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_RALLY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ramosian Rally",
    "7fc0ff04-43e7-4a0d-b7e2-8bab72cc6cc0",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// MMQ 39 — Ramosian Sergeant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_SERGEANT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ramosian Sergeant",
    "ef2b036d-5721-4a6e-bf43-69148b90da10",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// MMQ 40 — Ramosian Sky Marshal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_SKY_MARSHAL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ramosian Sky Marshal",
    "16638976-8a78-4233-8ebc-42ea9bb49e0a",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// MMQ 41 — Rappelling Scouts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPPELLING_SCOUTS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rappelling Scouts",
    "113b8366-e6d0-423e-af4b-52c1e08ed446",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// MMQ 42 — Renounce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENOUNCE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Renounce",
    "8bb2bfb9-cc4a-4d33-99f9-17db4d9fc718",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 43 — Revered Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERED_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Revered Elder",
    "b0793175-e56b-4ff8-9e22-3a96a698068c",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MMQ 44 — Reverent Mantra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERENT_MANTRA: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Reverent Mantra",
    "48364e19-a3ea-4980-925f-7918e57315f1",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 45 — Righteous Aura (reprint)
const RIGHTEOUS_AURA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::RIGHTEOUS_AURA,
    "66d3bcb4-6cbd-4144-a95d-f61e68c10296",
    "Pete Venters",
);

// MMQ 46 — Righteous Indignation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_INDIGNATION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Righteous Indignation",
    "c1fb6335-cfd8-438c-b936-09b850d61b28",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// MMQ 47 — Security Detail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SECURITY_DETAIL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Security Detail",
    "5b89d34b-1a67-4f5c-a731-54b56c5233ff",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// MMQ 48 — Soothing Balm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOOTHING_BALM: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Soothing Balm",
    "96b8f4be-9f4d-4373-8141-a03518ecd38a",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 49 — Spiritual Focus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRITUAL_FOCUS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Spiritual Focus",
    "8521ae08-eb46-45ff-8fc4-62d8b07cfac2",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// MMQ 50 — Steadfast Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEADFAST_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Steadfast Guard",
    "6381774b-fb91-46cc-9bf6-6eeb4d67a165",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// MMQ 51 — Story Circle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORY_CIRCLE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Story Circle",
    "675378bb-7dc1-4bc8-b026-27e6e8e72e18",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// MMQ 52 — Task Force
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TASK_FORCE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Task Force",
    "17a58c5b-28c2-4261-992c-2ecadb721880",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// MMQ 53 — Thermal Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THERMAL_GLIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Thermal Glider",
    "fd909c26-930d-4af0-b19a-c899847338b4",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// MMQ 54 — Tonic Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TONIC_PEDDLER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Tonic Peddler",
    "334bbd9d-3549-4352-9635-d772aab28503",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// MMQ 55 — Trap Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAP_RUNNER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Trap Runner",
    "eba97681-1d1f-4ab6-a21b-fbbbe63a1c74",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MMQ 56 — Wave of Reckoning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAVE_OF_RECKONING: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Wave of Reckoning",
    "0b101b5e-d478-4686-b3cf-bdc545f089e5",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// MMQ 57 — Wishmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WISHMONGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Wishmonger",
    "5a0d8834-109e-4235-a145-75edc43da0ec",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 58 — Aerial Caravan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AERIAL_CARAVAN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Aerial Caravan",
    "adac91af-5165-4779-99f7-e75c83fa5d5d",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// MMQ 59 — Balloon Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALLOON_PEDDLER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Balloon Peddler",
    "c34963e6-850e-4ce4-b04f-5e623ce5b73f",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MMQ 60 — Blockade Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOCKADE_RUNNER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Blockade Runner",
    "59e483df-b58a-401e-85bc-0afda4bf7cac",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 61 — Brainstorm (reprint)
const BRAINSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BRAINSTORM,
    "9ff71d13-c4b7-4125-ab10-db4abbb7a074",
    "DiTerlizzi",
);

// MMQ 62 — Bribery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIBERY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Bribery",
    "dfc0ea8a-62f6-49e8-8eec-9748870bc596",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// MMQ 63 — Buoyancy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUOYANCY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Buoyancy",
    "b208dad2-a412-45fd-b19a-d370426ef5b8",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 64 — Chambered Nautilus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMBERED_NAUTILUS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Chambered Nautilus",
    "860c613d-d031-4c2a-922b-39f4eec04e18",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MMQ 65 — Chameleon Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMELEON_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Chameleon Spirit",
    "05972ea2-b0bc-40fd-bce4-07eebdb150d5",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// MMQ 66 — Charisma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARISMA: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Charisma",
    "63565b03-28e9-4534-b085-d5803e2623bb",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 67 — Cloud Sprite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_SPRITE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cloud Sprite",
    "3d14352c-ac8c-45b5-b930-63822408ba3d",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// MMQ 68 — Coastal Piracy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COASTAL_PIRACY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Coastal Piracy",
    "179d1f76-6f4c-4a77-815a-aae7a933c9ad",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// MMQ 69 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "7bd03c80-7812-4704-9e07-9cf73b49c01f",
    "Gao Yan",
);

// MMQ 70 — Cowardice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COWARDICE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cowardice",
    "d2e46d3d-7c7f-487f-8cc6-078b17c113a0",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 71 — Customs Depot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUSTOMS_DEPOT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Customs Depot",
    "067d8c46-c334-4b00-af06-2e28b6086c58",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 72 — Darting Merfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARTING_MERFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Darting Merfolk",
    "438e15f7-59bb-4047-af1f-ef92cc1866b8",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// MMQ 73 — Dehydration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEHYDRATION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Dehydration",
    "2c9e4043-e7a6-4c68-aa03-ef2f88e46451",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// MMQ 74 — Diplomatic Escort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIPLOMATIC_ESCORT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Diplomatic Escort",
    "9356bdbb-d647-4f51-a7a3-18ecea898a7f",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 75 — Diplomatic Immunity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIPLOMATIC_IMMUNITY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Diplomatic Immunity",
    "fb1e610e-a4a2-460b-8e4c-13674badbce3",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 76 — Drake Hatchling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAKE_HATCHLING: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Drake Hatchling",
    "64ee32f9-6120-4f15-a692-89a4cd8167c6",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// MMQ 77 — Embargo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBARGO: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Embargo",
    "3fca3c65-f20e-4978-bfbb-ee7f9e1d829f",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// MMQ 78 — Energy Flux (reprint)
const ENERGY_FLUX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ENERGY_FLUX,
    "a77b22a0-d5cc-4dbb-aec3-763b8efaee7e",
    "Qiao Dafu",
);

// MMQ 79 — Extravagant Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTRAVAGANT_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Extravagant Spirit",
    "99243564-9dbd-420c-922d-c17854c99d2a",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// MMQ 80 — False Demise (reprint)
const FALSE_DEMISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::FALSE_DEMISE,
    "48872422-895f-45f0-ba2a-7cd307285c7d",
    "Pat Lewis",
);

// MMQ 81 — Glowing Anemone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOWING_ANEMONE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Glowing Anemone",
    "708593e6-787b-4f76-a86c-1d52857493ea",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 82 — Gush
pub(in crate::card::sets) static GUSH: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Gush",
    "e755bbef-bf34-49c0-ae72-d70e3599de52",
    "Kev Walker",
    CardRules::new_instant(mana_cost!("{4}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw two cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may return two Islands you control to their owner's hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SpellAdditionalCostDef::return_to_hand(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
            CostQuantityDef::Fixed(2),
        )),
    ]),
);

// MMQ 83 — High Seas
pub(in crate::card::sets) static HIGH_SEAS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "High Seas",
    "f12eb6a6-14cc-4ad6-9684-ff33a39ba09f",
    "Massimiliano Frezzato",
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_ability(abilities::spell_cost_increase(
        "Red creature spells and green creature spells cost {1} more to cast.",
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Color(ManaColor::Red),
                ObjectPredicateDef::Color(ManaColor::Green),
            ]),
        ]),
        PlayerRelation::Any,
        mana_cost!("{1}"),
    )),
);

// MMQ 84 — Hoodwink
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOODWINK: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Hoodwink",
    "8d505fbb-ec85-475b-a0e1-6670627ec017",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// MMQ 85 — Indentured Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INDENTURED_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Indentured Djinn",
    "dae62ce7-852b-42c6-9cbe-4807d8bf5740",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// MMQ 86 — Karn's Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARN_S_TOUCH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Karn's Touch",
    "07845861-f974-43b7-829c-79a4a41ac3e3",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 87 — Misdirection
pub(in crate::card::sets) static MISDIRECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Misdirection",
    "581ad59c-29e9-4498-a6fd-33bf21e8e7c4",
    "Paolo Parente",
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may exile a blue card from your hand rather than pay this spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
            ObjectPredicateDef::Color(ManaColor::Blue),
            ZoneKind::Hand,
            CostQuantityDef::Fixed(1),
        )),
        AbilityDef::spell_with_targets(
            "Change the target of target spell with a single target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::DeclaredTargetCount {
                            minimum: 1,
                            maximum: 1,
                        },
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                chooser: PlayerRefDef::EffectController,
                change: crate::card::StackTargetChangeDef::ChooseNew {
                    optional: false,
                    restriction: None,
                },
            }),
        ),
    ]),
);

// MMQ 88 — Misstep
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISSTEP: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Misstep",
    "8e23f5a1-bf3e-41e0-875f-fc2f8508e69f",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MMQ 89 — Overtaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERTAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Overtaker",
    "145903cb-9eaa-4f3c-a376-88dcd474ffda",
    "Clyde Caldwell",
    crate::card::CardRules::unsupported(),
);

// MMQ 90 — Port Inspector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PORT_INSPECTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Port Inspector",
    "ef25d969-68d9-4580-bb29-f72bd5646a3d",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MMQ 91 — Rishadan Airship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_AIRSHIP: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rishadan Airship",
    "5d8e596b-f5ef-405a-8910-c5d0b5c8c0fc",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MMQ 92 — Rishadan Brigand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_BRIGAND: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rishadan Brigand",
    "a6efb653-97d8-4bc7-af8f-0b09fda655ff",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// MMQ 93 — Rishadan Cutpurse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_CUTPURSE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rishadan Cutpurse",
    "947fc270-11e3-46cd-9086-e880a5845c79",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// MMQ 94 — Rishadan Footpad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_FOOTPAD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rishadan Footpad",
    "493ee964-1a44-46a1-8606-90e215805483",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// MMQ 95 — Sailmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAILMONGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Sailmonger",
    "142479d8-8956-44a2-8c54-9dd6dc1774c0",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// MMQ 96 — Sand Squid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAND_SQUID: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Sand Squid",
    "4efd7ce9-b920-409d-a4d2-a07fff280712",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MMQ 97 — Saprazzan Bailiff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_BAILIFF: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Bailiff",
    "a9f50964-1b57-426a-ac46-c90c045c7e40",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MMQ 98 — Saprazzan Breaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_BREAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Breaker",
    "2de7bf0f-5ad5-467b-ad80-28517951bbe1",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 99 — Saprazzan Heir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_HEIR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Heir",
    "0e3d913d-2dcf-4747-8169-0c44ec895864",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 100 — Saprazzan Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_LEGATE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Legate",
    "db9adf84-ee7e-472b-bd96-9abf853afa83",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// MMQ 101 — Saprazzan Outrigger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_OUTRIGGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Outrigger",
    "b28048f1-4cf5-4389-9e69-9b5e1bc95396",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// MMQ 102 — Saprazzan Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_RAIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Raider",
    "62493f34-cea8-4d9f-8781-005947b69c9d",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 103 — Shoving Match
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOVING_MATCH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Shoving Match",
    "aa9f4787-9b29-4f57-b105-1f9eb4bb8861",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// MMQ 104 — Soothsaying
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOOTHSAYING: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Soothsaying",
    "def384ff-1b6f-4c4f-8151-3c72c29b63ce",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// MMQ 105 — Squeeze
pub(in crate::card::sets) static SQUEEZE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Squeeze",
    "bbe63220-992b-459c-81ca-d4e2de273ce1",
    "DiTerlizzi",
    CardRules::new_enchantment(mana_cost!("{3}{U}")).with_ability(abilities::spell_cost_increase(
        "Sorcery spells cost {3} more to cast.",
        ObjectPredicateDef::HasType(CardType::Sorcery),
        PlayerRelation::Any,
        mana_cost!("{3}"),
    )),
);

// MMQ 106 — Statecraft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STATECRAFT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Statecraft",
    "76dcd19e-8daf-4d53-946b-c07d5eca3cc9",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// MMQ 107 — Stinging Barrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STINGING_BARRIER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Stinging Barrier",
    "ca7f7cd5-4e91-474a-9f60-a66f3f462b1c",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// MMQ 108 — Thwart
pub(in crate::card::sets) static THWART: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Thwart",
    "c12a0717-e9ea-4be3-a29f-179671ed4489",
    "Christopher Moeller",
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_abilities(&[
        AbilityDef::counter_target("Counter target spell.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )][0]),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may return three Islands you control to their owner's hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SpellAdditionalCostDef::return_to_hand(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
            CostQuantityDef::Fixed(3),
        )),
    ]),
);

// MMQ 109 — Tidal Bore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_BORE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Tidal Bore",
    "f68fd547-59fb-41e6-be55-1ec17fe2840b",
    "Frank Kelly Freas",
    crate::card::CardRules::unsupported(),
);

// MMQ 110 — Tidal Kraken
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_KRAKEN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Tidal Kraken",
    "356a9dcd-1a4b-4371-8f1d-aa7cb65e97e8",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// MMQ 111 — Timid Drake (reprint)
const TIMID_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::TIMID_DRAKE,
    "9212f685-d7af-4279-b17e-7201d8f63813",
    "Edward P. Beard, Jr.",
);

// MMQ 112 — Trade Routes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADE_ROUTES: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Trade Routes",
    "eeaba189-b215-4d1c-9135-a86ce5ec955d",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// MMQ 113 — War Tax
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_TAX: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "War Tax",
    "e7c15159-8466-43e5-9dc5-a8cc94619931",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 114 — Waterfront Bouncer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATERFRONT_BOUNCER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Waterfront Bouncer",
    "8dbdce9e-94fa-4ed5-9b97-d2026cffe7cb",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MMQ 115 — Alley Grifters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALLEY_GRIFTERS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Alley Grifters",
    "cfb648e3-f5ad-4b33-afa3-d4cda0d369a1",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MMQ 116 — Black Market
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_MARKET: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Black Market",
    "05976e5c-b46c-431e-9dbd-1dc5fad3536c",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// MMQ 117 — Bog Smugglers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_SMUGGLERS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Bog Smugglers",
    "c2103a44-87e5-40cd-a0de-cd19456a8366",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// MMQ 118 — Bog Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_WITCH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Bog Witch",
    "6a926f9e-ee63-4b6e-8e5b-0650b74344a5",
    "Gao Yan",
    crate::card::CardRules::unsupported(),
);

// MMQ 119 — Cackling Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CACKLING_WITCH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cackling Witch",
    "cec755ee-b4c0-47fd-9e61-9a3161766de6",
    "Brian Despain",
    crate::card::CardRules::unsupported(),
);

// MMQ 120 — Cateran Brute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_BRUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cateran Brute",
    "73b6ce76-0ed0-4994-ae2c-d8e51ae09920",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// MMQ 121 — Cateran Enforcer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_ENFORCER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cateran Enforcer",
    "9e9b6da8-39da-4fce-89cf-ea972f981331",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// MMQ 122 — Cateran Kidnappers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_KIDNAPPERS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cateran Kidnappers",
    "3768bdc1-4055-423a-a1cc-69b4c620e3e6",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 123 — Cateran Overlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_OVERLORD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cateran Overlord",
    "e8a1ffcb-40a7-423f-b28a-b5b4c1c9ffd0",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// MMQ 124 — Cateran Persuader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_PERSUADER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cateran Persuader",
    "a98bdbf1-32a6-4d9b-8e57-5d3aca6b05bc",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 125 — Cateran Slaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_SLAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cateran Slaver",
    "2d293c51-714c-45b8-bfa4-fe35e8f3fbc1",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 126 — Cateran Summons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_SUMMONS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cateran Summons",
    "af3de1f9-9038-4352-b4bf-2e9c5c27495a",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 127 — Conspiracy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSPIRACY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Conspiracy",
    "411c9f22-2df0-4a63-b2be-fa02612a6ef8",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// MMQ 128 — Corrupt Official
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORRUPT_OFFICIAL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Corrupt Official",
    "5cb652fc-5a21-4e02-a776-a38fb41ad18c",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 129 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "a6aacc3e-fe37-4a08-83e6-7ee8c0c0af74",
    "Rebecca Guay",
);

// MMQ 130 — Deathgazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATHGAZER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Deathgazer",
    "d0fff328-704e-462d-9613-82d05371f544",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MMQ 131 — Deepwood Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_GHOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Deepwood Ghoul",
    "29cd6685-37ca-47c7-8f64-1fb86e9610ca",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 132 — Deepwood Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_LEGATE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Deepwood Legate",
    "54f01925-7fd0-472d-91a4-3309e615f22f",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 133 — Delraich
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELRAICH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Delraich",
    "da64094f-df6e-4c43-b4ae-03aab6b92816",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// MMQ 134 — Enslaved Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENSLAVED_HORROR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Enslaved Horror",
    "dffca723-360d-48de-a0a8-32288627f3df",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// MMQ 135 — Extortion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTORTION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Extortion",
    "a66742db-4750-49ce-ad05-b825af7222c4",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 136 — Forced March
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORCED_MARCH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Forced March",
    "36eae0e1-7100-449d-a259-7abfcd429117",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 137 — Ghoul's Feast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOUL_S_FEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ghoul's Feast",
    "6a0054c1-6510-41dd-8695-9bf50296b615",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 138 — Haunted Crossroads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTED_CROSSROADS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Haunted Crossroads",
    "3c065cae-1ed5-445e-ace3-e81cf4c773de",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 139 — Highway Robber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGHWAY_ROBBER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Highway Robber",
    "fc826c88-fe3c-4004-8283-27910c550fae",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MMQ 140 — Instigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSTIGATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Instigator",
    "2e3f57af-17d4-4a4c-ae46-fe37f97466fa",
    "Fred Fields",
    crate::card::CardRules::unsupported(),
);

// MMQ 141 — Insubordination
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSUBORDINATION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Insubordination",
    "d2544c9d-adc2-4d67-8850-9af38e73ea1e",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// MMQ 142 — Intimidation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTIMIDATION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Intimidation",
    "1b9e1724-91cf-422e-909b-ddb69a6f9f76",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 143 — Larceny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LARCENY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Larceny",
    "3a863da2-0639-4eed-8da9-2e9a38c04a23",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// MMQ 144 — Liability
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIABILITY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Liability",
    "0b07c66d-5f37-4098-b7e6-03e6c684806b",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// MMQ 145 — Maggot Therapy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGGOT_THERAPY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Maggot Therapy",
    "6ab963aa-2304-4ee6-a8c7-c485c5133b40",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// MMQ 146 — Midnight Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIDNIGHT_RITUAL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Midnight Ritual",
    "0e98c4f7-b0f4-48c6-b502-3dc5802d827f",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// MMQ 147 — Misshapen Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISSHAPEN_FIEND: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Misshapen Fiend",
    "a43cf59e-7583-4651-968a-2a7201c69b6b",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// MMQ 148 — Molting Harpy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTING_HARPY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Molting Harpy",
    "ddfe33fb-71d5-4552-bcd3-f07e4e3847e1",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// MMQ 149 — Nether Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NETHER_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Nether Spirit",
    "220217c5-408c-40df-8133-da16b13d4f21",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 150 — Notorious Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOTORIOUS_ASSASSIN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Notorious Assassin",
    "239e48d8-e2ba-4e25-88ef-301420c796b4",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 151 — Pretender's Claim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRETENDER_S_CLAIM: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Pretender's Claim",
    "2cc29dab-9211-46bc-a98c-a5dbd5b0980a",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 152 — Primeval Shambler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMEVAL_SHAMBLER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Primeval Shambler",
    "5d6ed1fb-2f7d-4a21-bbf3-660cad631975",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// MMQ 153 — Putrefaction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUTREFACTION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Putrefaction",
    "65104b23-58a6-41c0-b887-90a3fb959289",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// MMQ 154 — Quagmire Lamprey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUAGMIRE_LAMPREY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Quagmire Lamprey",
    "3c91c44e-6bfc-4595-9cdb-17d73f912c09",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// MMQ 155 — Rain of Tears (reprint)
const RAIN_OF_TEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::RAIN_OF_TEARS,
    "85bffa7f-919c-4c9b-9fdc-dde8204c61c2",
    "Edward P. Beard, Jr.",
);

// MMQ 156 — Rampart Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMPART_CRAWLER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rampart Crawler",
    "8b60f86f-c78a-4dfb-bb18-e9bcf21b26c4",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 157 — Rouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROUSE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rouse",
    "ad01a8e2-5dc5-49a3-ad1c-7d5bf006b774",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// MMQ 158 — Scandalmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCANDALMONGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Scandalmonger",
    "10c97baa-9bc0-4894-867c-ad1f56c469fd",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// MMQ 159 — Sever Soul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEVER_SOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Sever Soul",
    "c2d84fec-18f1-4231-a293-0dc1ff868a40",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// MMQ 160 — Silent Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILENT_ASSASSIN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Silent Assassin",
    "ba34034a-1150-41c8-a340-543f529ae07f",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// MMQ 161 — Skulking Fugitive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKULKING_FUGITIVE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Skulking Fugitive",
    "175ed19c-9635-45ee-bb2c-32b96270a246",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 162 — Snuff Out
pub(in crate::card::sets) static SNUFF_OUT: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Snuff Out",
    "18a3cca1-e50e-49b6-9e1a-f86640e3b177",
    "Mike Ploog",
    // Four life and no mana is why it is played: the answer costs nothing on
    // the turn it is needed, which is somebody else's.
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("If you control a Swamp, you may pay 4 life rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_life(4)
        // A Swamp on the battlefield, which is what the free cast is gated on.
        .with_alternative_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 1,
        }),
        AbilityDef::destroy_target(
            "Destroy target nonblack creature. It can't be regenerated.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]),
            )][0],
            false,
        ),
    ]),
);

// MMQ 163 — Soul Channeling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_CHANNELING: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Soul Channeling",
    "55cd09ef-1655-4a62-b6c5-6eda33d2607a",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// MMQ 164 — Specter's Wail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTER_S_WAIL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Specter's Wail",
    "d1637b62-e364-4250-aad5-841c6a47a11e",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MMQ 165 — Strongarm Thug
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGARM_THUG: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Strongarm Thug",
    "20aa9108-470c-484d-908a-c31cf6935765",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 166 — Thrashing Wumpus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRASHING_WUMPUS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Thrashing Wumpus",
    "86bc07c6-2ba7-41f8-90ab-f9bbac86dd08",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 167 — Undertaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERTAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Undertaker",
    "f615f531-e8af-4f7b-a4ea-fb962149093f",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// MMQ 168 — Unmask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNMASK: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Unmask",
    "2db7a0e6-eea5-4fa6-ac14-401411b106cc",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// MMQ 169 — Unnatural Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNNATURAL_HUNGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Unnatural Hunger",
    "3985f240-4289-4d48-978c-bb2ce2b54c36",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 170 — Vendetta
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENDETTA: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Vendetta",
    "67ced38e-0f33-4bda-8e18-09f6ac03a3d7",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MMQ 171 — Wall of Distortion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_DISTORTION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Wall of Distortion",
    "d2b2d07a-9ea1-430d-b432-ae507f4fe73b",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// MMQ 172 — Arms Dealer
pub(in crate::card::sets) static ARMS_DEALER: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Arms Dealer",
    "dafea45d-be00-428d-a127-70e6a14efe3f",
    "Luca Zontini",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{R}, Sacrifice a Goblin: This creature deals 4 damage to target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype("Goblin"),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// MMQ 173 — Battle Rampart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_RAMPART: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Battle Rampart",
    "f27f6658-0f00-4934-8d12-cd0dda3958c9",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MMQ 174 — Battle Squadron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_SQUADRON: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Battle Squadron",
    "37d55504-ee04-4a5a-a952-9ec5dc2db413",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// MMQ 175 — Blaster Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLASTER_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Blaster Mage",
    "801b0fd1-bbb2-47c0-a4c3-4129a67473b9",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// MMQ 176 — Blood Hound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_HOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Blood Hound",
    "baa1e796-809c-49af-a84e-ec088f7f48f8",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// MMQ 177 — Blood Oath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_OATH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Blood Oath",
    "f1556a12-ff45-4a12-988a-63615b3799a9",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// MMQ 178 — Brawl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAWL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Brawl",
    "3f4e783c-0717-4127-bd7b-885ca617ca29",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// MMQ 179 — Cave Sense
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVE_SENSE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cave Sense",
    "2d718421-c742-489c-a243-3adb19f6716a",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// MMQ 180 — Cave-In
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVE_IN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cave-In",
    "440d9d26-f304-467d-af79-914cc65f082e",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// MMQ 181 — Cavern Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVERN_CRAWLER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Cavern Crawler",
    "bd0a8af9-2e86-4639-a6c9-209f115e95f8",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 182 — Ceremonial Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEREMONIAL_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ceremonial Guard",
    "4a6c69d1-5295-419f-bb8f-af826bf92cb3",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// MMQ 183 — Cinder Elemental
pub(in crate::card::sets) static CINDER_ELEMENTAL: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Cinder Elemental",
    "80b39056-2ee8-4cfd-acbd-ba99f74e788d",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{X}{R}, {T}, Sacrifice this creature: It deals X damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{X}{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
    ),
);

// MMQ 184 — Close Quarters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOSE_QUARTERS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Close Quarters",
    "1b9131c7-4e46-4c01-80b3-a6b055439346",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MMQ 185 — Crag Saurian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRAG_SAURIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Crag Saurian",
    "1f0907a5-938e-4ef4-aa85-e7c1ae4317a6",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// MMQ 186 — Crash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRASH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Crash",
    "7a26bde3-8392-4476-b347-f223d52554a6",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// MMQ 187 — Flailing Manticore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAILING_MANTICORE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Flailing Manticore",
    "6eee8c2e-bda7-4bf9-80fe-87d96024ca8b",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// MMQ 188 — Flailing Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAILING_OGRE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Flailing Ogre",
    "e400e520-b2b8-4c13-a4ea-f8810c927bf7",
    "Daniel R. Horne",
    crate::card::CardRules::unsupported(),
);

// MMQ 189 — Flailing Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAILING_SOLDIER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Flailing Soldier",
    "fb44b0f6-0608-40d6-9eaa-48e5a834701f",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// MMQ 190 — Flaming Sword
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMING_SWORD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Flaming Sword",
    "17ecd9ff-8c30-4e17-8cff-dd40d653c4af",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MMQ 191 — Furious Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FURIOUS_ASSAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Furious Assault",
    "27a07fae-0f34-45e7-b22d-97eea9031022",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// MMQ 192 — Gerrard's Irregulars
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_S_IRREGULARS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Gerrard's Irregulars",
    "8a88f507-3d78-4f7f-a91f-8489ad9250f2",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// MMQ 193 — Hammer Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAMMER_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Hammer Mage",
    "b959d7ad-a78e-439f-9225-4dbb89f490d7",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 194 — Hired Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIRED_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Hired Giant",
    "dc33920a-f05c-46fa-b94b-278af0022b78",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// MMQ 195 — Kris Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRIS_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Kris Mage",
    "4389fbcd-182a-4cac-b14f-aa971948cf8e",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// MMQ 196 — Kyren Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_GLIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Kyren Glider",
    "0bc55e01-342e-4856-937e-14561b8d165b",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// MMQ 197 — Kyren Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_LEGATE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Kyren Legate",
    "6f0e9806-be8c-4b88-a4be-0111d1be81d9",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// MMQ 198 — Kyren Negotiations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_NEGOTIATIONS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Kyren Negotiations",
    "0c263a17-bbc2-433e-93f8-72e57b818322",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// MMQ 199 — Kyren Sniper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_SNIPER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Kyren Sniper",
    "4df99e19-0b1e-48ec-a146-38cf147eab61",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 200 — Lava Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_RUNNER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Lava Runner",
    "09d0fbe6-6ce1-4b95-afb7-a7386b5033cf",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MMQ 201 — Lightning Hounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_HOUNDS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Lightning Hounds",
    "38c82a1d-5db1-4090-b446-cc5bc6dc811d",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// MMQ 202 — Lithophage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LITHOPHAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Lithophage",
    "98ee0f17-de64-4abb-afad-4005275f1a3c",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// MMQ 203 — Lunge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUNGE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Lunge",
    "e9e43349-429c-43f7-b808-c4bf37370a9f",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MMQ 204 — Magistrate's Veto
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGISTRATE_S_VETO: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Magistrate's Veto",
    "2f83d39e-bf49-4968-829e-c0e9abf2fb86",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// MMQ 205 — Mercadia's Downfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIA_S_DOWNFALL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Mercadia's Downfall",
    "14507fe6-80a9-4ed4-bf3e-4656f3d377c0",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 206 — Ogre Taskmaster (reprint)
const OGRE_TASKMASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::OGRE_TASKMASTER,
    "186d6c28-6468-4bde-9738-eb51594fa7c1",
    "Dany Orizio",
);

// MMQ 207 — Pulverize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULVERIZE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Pulverize",
    "afbbc44d-60fb-45fc-a588-14aab0340134",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 208 — Puppet's Verdict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUPPET_S_VERDICT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Puppet's Verdict",
    "052b743a-456d-49c3-881e-4f30c7645fa5",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// MMQ 209 — Robber Fly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROBBER_FLY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Robber Fly",
    "7d5cf073-2ba0-463e-bcd4-979ad18e28fc",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MMQ 210 — Rock Badger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCK_BADGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rock Badger",
    "dff05df8-76f5-48c6-ac96-7b4e6a7050f6",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 211 — Seismic Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEISMIC_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Seismic Mage",
    "9524432a-3186-4c7b-a780-28bdbe36053f",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 212 — Shock Troops
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOCK_TROOPS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Shock Troops",
    "e7a918ca-3e60-46de-9f29-56bdc6430a77",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MMQ 213 — Sizzle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIZZLE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Sizzle",
    "f1ca1eee-d97d-48c6-84f1-7d1f972c3ca9",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// MMQ 214 — Squee, Goblin Nabob
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUEE_GOBLIN_NABOB: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Squee, Goblin Nabob",
    "4ba8325a-1203-4125-9111-94d9e2b1f14b",
    "David Monette",
    crate::card::CardRules::unsupported(),
);

// MMQ 215 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "29cd7ded-9249-42c8-bb17-4c6b8cd2a9cc",
    "Ben Thompson",
);

// MMQ 216 — Tectonic Break
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TECTONIC_BREAK: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Tectonic Break",
    "9de0ee5d-10f6-4152-8416-1f2b749b439d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 217 — Territorial Dispute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRITORIAL_DISPUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Territorial Dispute",
    "4fa8a13a-f09f-4b10-8fab-3ea4fdc643d1",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// MMQ 218 — Thieves' Auction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THIEVES_AUCTION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Thieves' Auction",
    "b5708c87-108d-4ba1-a1e9-e83cb9b16b6c",
    "Kevin Murphy",
    crate::card::CardRules::unsupported(),
);

// MMQ 219 — Thunderclap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERCLAP: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Thunderclap",
    "b3f8c5ee-2179-4c05-adc9-0b66d02b59ad",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// MMQ 220 — Tremor (reprint)
const TREMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::TREMOR,
    "8531efb1-d77d-451a-8621-424fc278ccf9",
    "Mark Romanoski",
);

// MMQ 221 — Two-Headed Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWO_HEADED_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Two-Headed Dragon",
    "40fed2c7-c922-41c3-b86b-a8ed41a1308d",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// MMQ 222 — Uphill Battle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UPHILL_BATTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Uphill Battle",
    "73fa3455-3ba0-41ad-aefd-40f183aed2a6",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 223 — Volcanic Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLCANIC_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Volcanic Wind",
    "3c69dd00-46ce-42b2-a2ed-43b4cf04a975",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 224 — War Cadence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_CADENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "War Cadence",
    "e030d2eb-70c5-4ff7-8f03-ad5495cf9c69",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MMQ 225 — Warmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARMONGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Warmonger",
    "5577ac30-ee84-4d3c-b407-82578779dc90",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 226 — Warpath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARPATH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Warpath",
    "e031c819-1237-4911-8a1d-87d6095a5faa",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MMQ 227 — Wild Jhovall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_JHOVALL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Wild Jhovall",
    "64bcc06a-de86-4387-882d-ead33e9c9e01",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// MMQ 228 — Word of Blasting (reprint)
const WORD_OF_BLASTING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::WORD_OF_BLASTING,
    "c5362ead-9162-4160-bfa9-432f7d0e222d",
    "Eric Peterson",
);

// MMQ 229 — Ancestral Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_MASK: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ancestral Mask",
    "1203f98a-fb6e-4f16-88e3-553eba177450",
    "Massimiliano Frezzato",
    crate::card::CardRules::unsupported(),
);

// MMQ 230 — Bifurcate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIFURCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Bifurcate",
    "dedb483e-b2c6-46c6-b02b-a49599d33521",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MMQ 231 — Boa Constrictor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOA_CONSTRICTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Boa Constrictor",
    "f7369cbf-6986-4a39-b07c-a283b40aee40",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 232 — Briar Patch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIAR_PATCH: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Briar Patch",
    "b5913fc6-eeb0-411b-9264-1e75bea8489b",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 233 — Caller of the Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLER_OF_THE_HUNT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Caller of the Hunt",
    "c0e8e1cf-0a47-4ce4-889a-091229d0e466",
    "Clyde Caldwell",
    crate::card::CardRules::unsupported(),
);

// MMQ 234 — Caustic Wasps
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAUSTIC_WASPS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Caustic Wasps",
    "59a46e20-2910-4287-a5e0-bccac8cbabcd",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// MMQ 235 — Clear the Land
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLEAR_THE_LAND: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Clear the Land",
    "b87f3579-f314-4207-a02c-14e9cb269b47",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// MMQ 236 — Collective Unconscious
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLECTIVE_UNCONSCIOUS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Collective Unconscious",
    "8fa7d6a8-9190-403f-bbdd-ab71d9c89e4d",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// MMQ 237 — Dawnstrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWNSTRIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Dawnstrider",
    "2d193a35-8950-4a77-ace3-c4d4085727f4",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// MMQ 238 — Deadly Insect (reprint)
const DEADLY_INSECT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::DEADLY_INSECT,
    "46be78e6-13bb-4500-87db-5ed5cae0145e",
    "Randy Gallegos",
);

// MMQ 239 — Deepwood Drummer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_DRUMMER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Deepwood Drummer",
    "acbed0f5-2ac0-48d8-b5ab-b4cd7176fde2",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// MMQ 240 — Deepwood Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Deepwood Elder",
    "d2a1ed74-027e-4c8e-ac7e-e58c5fccff14",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// MMQ 241 — Deepwood Tantiv
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_TANTIV: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Deepwood Tantiv",
    "bfa2028e-4e73-4ff2-a9e2-9ac347d67893",
    "Joel Biske",
    crate::card::CardRules::unsupported(),
);

// MMQ 242 — Deepwood Wolverine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_WOLVERINE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Deepwood Wolverine",
    "db9a9a76-741a-4ba3-bd4b-0eb87d678253",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// MMQ 243 — Desert Twister (reprint)
const DESERT_TWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DESERT_TWISTER,
    "7d2437f2-1966-4e83-9f7e-6aaf76e21d11",
    "Kevin Murphy",
);

// MMQ 244 — Erithizon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERITHIZON: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Erithizon",
    "ec4ea4e2-2102-4b99-bea5-6fc4203f2b26",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 245 — Ferocity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEROCITY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ferocity",
    "4afda489-8397-4ad4-89dc-e8bad92db133",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 246 — Food Chain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOD_CHAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Food Chain",
    "18a1bb9e-006c-495e-8f99-d451183d2669",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// MMQ 247 — Foster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOSTER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Foster",
    "be54e1d7-1388-4184-ad0d-dde4b0a3d02d",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 248 — Game Preserve
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAME_PRESERVE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Game Preserve",
    "1bb62356-72bb-4dc1-a2f9-45a3aca62e41",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// MMQ 249 — Giant Caterpillar (reprint)
const GIANT_CATERPILLAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::GIANT_CATERPILLAR,
    "bdc5eb8a-4531-4408-90fe-9b352d71a052",
    "Arnie Swekel",
);

// MMQ 250 — Groundskeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROUNDSKEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Groundskeeper",
    "31d9fe16-562a-4a86-84ed-15cd90b8afc0",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MMQ 251 — Horned Troll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORNED_TROLL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Horned Troll",
    "7f2a6d10-054e-4d6f-aeb7-4204f02490c7",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 252 — Howling Wolf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOWLING_WOLF: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Howling Wolf",
    "7416c68a-5a6a-4d51-8dc7-5c62da81ec77",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 253 — Hunted Wumpus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTED_WUMPUS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Hunted Wumpus",
    "b21c8b2d-ef0f-4839-acfc-20fd248c62cf",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// MMQ 254 — Invigorate
pub(in crate::card::sets) static INVIGORATE: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Invigorate",
    "406b343c-90b5-4a4d-91c3-2fddcc9a0e05",
    "Dan Frazier",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If you control a Forest, rather than pay this spell's mana cost, you may have an \
                 opponent gain 3 life.",
            ),
            EffectDef::None,
        )
        .with_alternative_opponent_life_gain(3)
        .with_alternative_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 1,
        }),
        AbilityDef::spell_with_targets(
            "Target creature gets +4/+4 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MMQ 255 — Land Grant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAND_GRANT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Land Grant",
    "d6862005-32d1-473e-a28b-5dfc4b7782cd",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// MMQ 256 — Ley Line
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEY_LINE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Ley Line",
    "f8990efd-708a-4019-bce0-2d6409ecc004",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 257 — Lumbering Satyr
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMBERING_SATYR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Lumbering Satyr",
    "5d897088-0667-4864-91c3-5f0ac7f9b220",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 258 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "89e0015e-9b16-4787-8b4f-02d8bddb1b80",
    "DiTerlizzi",
);

// MMQ 259 — Megatherium
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEGATHERIUM: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Megatherium",
    "c58a1e43-a173-45d6-ac55-363664bf6e1b",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MMQ 260 — Natural Affinity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURAL_AFFINITY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Natural Affinity",
    "69c6f647-f71b-4f61-9b16-774884ed52e2",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MMQ 261 — Pangosaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANGOSAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Pangosaur",
    "0335d282-cd1a-4be3-8eb2-82aaee91401a",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// MMQ 262 — Revive
pub(in crate::card::sets) static REVIVE: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Revive",
    "7b19b453-8393-424d-9578-3ab568c92882",
    "Matthew D. Wilson",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return target green card from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Color(ManaColor::Green),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// MMQ 263 — Rushwood Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHWOOD_DRYAD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rushwood Dryad",
    "55367a94-b343-4a04-bfa9-47722e32cc45",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// MMQ 264 — Rushwood Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHWOOD_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rushwood Elemental",
    "52128694-d9f5-4acb-b684-bb02a4e766b8",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// MMQ 265 — Rushwood Herbalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHWOOD_HERBALIST: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rushwood Herbalist",
    "9afde98f-a429-4eff-9d06-8582267ac74b",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 266 — Rushwood Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHWOOD_LEGATE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rushwood Legate",
    "827b9c99-87d7-493c-9dc3-0c6aa4a61b49",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// MMQ 267 — Saber Ants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SABER_ANTS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saber Ants",
    "e9269f52-1002-475d-a0f3-d652630591ca",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// MMQ 268 — Sacred Prey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_PREY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Sacred Prey",
    "e965d32c-3151-48e8-b256-0b7fa8a8a211",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 269 — Silverglade Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVERGLADE_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Silverglade Elemental",
    "f222fe90-ac92-4ba9-b060-9b64075bf139",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// MMQ 270 — Silverglade Pathfinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVERGLADE_PATHFINDER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Silverglade Pathfinder",
    "9bc99b33-ce06-4a44-8b23-300b41b2b2fe",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// MMQ 271 — Snake Pit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAKE_PIT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Snake Pit",
    "059a70a5-d4fb-445e-af98-e81821df2c59",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 272 — Snorting Gahr
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNORTING_GAHR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Snorting Gahr",
    "e568503e-a886-4c8b-9d46-8520c2cdda48",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// MMQ 273 — Spidersilk Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDERSILK_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Spidersilk Armor",
    "9eb7694f-af4c-4152-b868-528257d05154",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// MMQ 274 — Spontaneous Generation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPONTANEOUS_GENERATION: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Spontaneous Generation",
    "ce5765cb-00cd-4920-9fe8-68791048ec4a",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 275 — Squall (reprint)
const SQUALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::SQUALL,
    "e5409b54-66ed-4add-bf43-cfeb074b1c50",
    "Val Mayerik",
);

// MMQ 276 — Squallmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUALLMONGER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Squallmonger",
    "c845e1b8-6a39-456c-aa67-d180ae63e200",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 277 — Stamina
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMINA: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Stamina",
    "ed8abca3-6e31-49cd-b9bf-86ad68e1cc83",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MMQ 278 — Sustenance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSTENANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Sustenance",
    "5a61db44-80dc-4058-9c9d-65cd18e63fd4",
    "Qiao Dafu",
    crate::card::CardRules::unsupported(),
);

// MMQ 279 — Tiger Claws
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIGER_CLAWS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Tiger Claws",
    "0146a689-4817-4849-a90d-4cc64566960d",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// MMQ 280 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "843e801e-1ceb-4e3f-82e6-3c092051ba8c",
    "Heather Hudson",
);

// MMQ 281 — Venomous Breath (reprint)
const VENOMOUS_BREATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::VENOMOUS_BREATH,
    "4797556c-df74-4e13-b8fb-8b0b58c92b4c",
    "DiTerlizzi",
);

// MMQ 282 — Venomous Dragonfly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMOUS_DRAGONFLY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Venomous Dragonfly",
    "479fc902-ce94-4a6b-af87-4645387a46c6",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// MMQ 283 — Vernal Equinox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERNAL_EQUINOX: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Vernal Equinox",
    "3bed69d2-f5fb-4173-b939-5abdb48b82b4",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 284 — Vine Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINE_DRYAD: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Vine Dryad",
    "fc9c9158-faed-42ae-9f6b-71dee49ff79f",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// MMQ 285 — Vine Trellis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINE_TRELLIS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Vine Trellis",
    "e660241f-0976-4206-8149-7dac8466a2a3",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// MMQ 286 — Assembly Hall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASSEMBLY_HALL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Assembly Hall",
    "1676ccbb-91d2-4f26-b3a5-ccb1a21bdebf",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// MMQ 287 — Barbed Wire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_WIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Barbed Wire",
    "be9e2e59-1527-4c61-9cc9-dcaf1181bd43",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MMQ 288 — Bargaining Table
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARGAINING_TABLE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Bargaining Table",
    "85da9395-42e9-4408-832d-74ea4b01256b",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 289 — Credit Voucher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CREDIT_VOUCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Credit Voucher",
    "1ab65242-17ad-4c22-9c70-aac8076d1b4c",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// MMQ 290 — Crenellated Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRENELLATED_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Crenellated Wall",
    "d85ad08d-1120-411a-8bbe-ac93a56476bd",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// MMQ 291 — Crooked Scales
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROOKED_SCALES: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Crooked Scales",
    "fd7084ba-cca6-4fb9-b21b-b79e7d74c5c0",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// MMQ 292 — Crumbling Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUMBLING_SANCTUARY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Crumbling Sanctuary",
    "d8fa6d6c-c1cd-46f3-8430-94f67be55bf7",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MMQ 293 — Distorting Lens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISTORTING_LENS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Distorting Lens",
    "ab196d8d-5d1c-4f0e-a924-37774db02821",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// MMQ 294 — Eye of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EYE_OF_RAMOS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Eye of Ramos",
    "78d22400-39f6-444d-b508-783a7df7e945",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// MMQ 295 — General's Regalia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENERAL_S_REGALIA: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "General's Regalia",
    "fb99d982-8ab1-4d6a-ba24-58ac23a9b9e7",
    "David Monette",
    crate::card::CardRules::unsupported(),
);

// MMQ 296 — Heart of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_OF_RAMOS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Heart of Ramos",
    "a0046226-7563-4345-aa4b-a2c732c2780a",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// MMQ 297 — Henge Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HENGE_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Henge Guardian",
    "028e5e18-b639-4461-87e4-5306371440b5",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// MMQ 298 — Horn of Plenty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORN_OF_PLENTY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Horn of Plenty",
    "d5e04462-1d10-47df-b456-211dd0a87891",
    "Brian Despain",
    crate::card::CardRules::unsupported(),
);

// MMQ 299 — Horn of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORN_OF_RAMOS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Horn of Ramos",
    "6b17f541-8e9d-43b0-b688-e3f2e7fa55c8",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// MMQ 300 — Iron Lance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_LANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Iron Lance",
    "41f7d212-faf2-4a6f-a338-d9e5014b56d5",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MMQ 301 — Jeweled Torque
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JEWELED_TORQUE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Jeweled Torque",
    "eab076bc-e4c3-42a1-b701-9bc49bcc3cdd",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// MMQ 302 — Kyren Archive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_ARCHIVE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Kyren Archive",
    "5e65a06a-e7af-422a-9481-446731009935",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// MMQ 303 — Kyren Toy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_TOY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Kyren Toy",
    "7a8318bb-bc3c-45e9-bd57-60ae72b6f8b0",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// MMQ 304 — Magistrate's Scepter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGISTRATE_S_SCEPTER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Magistrate's Scepter",
    "d4785ed7-c948-4ad2-b24d-2f45806d9fcc",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// MMQ 305 — Mercadian Atlas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIAN_ATLAS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Mercadian Atlas",
    "00ad3531-399c-4897-b0ee-ad2a26445a17",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MMQ 306 — Mercadian Lift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIAN_LIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Mercadian Lift",
    "395a1a8a-785f-442b-8e95-8b4ca44af2a3",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// MMQ 307 — Monkey Cage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONKEY_CAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Monkey Cage",
    "07f6be53-7a20-4e6b-a6ce-11cba06af8cb",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 308 — Panacea
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANACEA: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Panacea",
    "89414770-2a19-4baf-9b18-76104b7b0b9a",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MMQ 309 — Power Matrix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POWER_MATRIX: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Power Matrix",
    "a578599c-7d90-4881-b59a-9cf64b90d917",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// MMQ 310 — Puffer Extract
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUFFER_EXTRACT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Puffer Extract",
    "83093cdf-0b12-419c-a748-21acf166e195",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// MMQ 311 — Rishadan Pawnshop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_PAWNSHOP: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rishadan Pawnshop",
    "2c5fc9fc-a0f9-4f56-8368-2d7e1fec5ba0",
    "Joel Biske",
    crate::card::CardRules::unsupported(),
);

// MMQ 312 — Skull of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKULL_OF_RAMOS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Skull of Ramos",
    "f071957c-9bea-4d00-9ffd-30f98d57b8d2",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// MMQ 313 — Tooth of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOOTH_OF_RAMOS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Tooth of Ramos",
    "9a3b999d-8e63-4647-a921-15e169022096",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// MMQ 314 — Toymaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOYMAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Toymaker",
    "76f3992a-553c-4032-b144-55aad2f909f1",
    "Frank Kelly Freas",
    crate::card::CardRules::unsupported(),
);

// MMQ 315 — Worry Beads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORRY_BEADS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Worry Beads",
    "400edfe9-9efa-43f9-b713-13ad4eae2fa4",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// MMQ 316 — Dust Bowl
pub(in crate::card::sets) static DUST_BOWL: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Dust Bowl",
    "75b03c30-c2b8-4207-b675-26c59c40a7e5",
    "Ben Thompson",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{3}, {T}, Sacrifice a land: Destroy target nonbasic land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// MMQ 317 — Fountain of Cho
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUNTAIN_OF_CHO: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Fountain of Cho",
    "41f352c3-4b63-4174-b2b4-6c19fb8c06ff",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// MMQ 318 — Henge of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HENGE_OF_RAMOS: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Henge of Ramos",
    "0582b42f-5ae5-4be2-ba2d-ed62b3cb20c5",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// MMQ 319 — Hickory Woodlot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HICKORY_WOODLOT: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Hickory Woodlot",
    "af7aafb7-6870-4d09-a191-70786766c459",
    "Sean McConnell",
    crate::card::CardRules::unsupported(),
);

// MMQ 320 — High Market
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_MARKET: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "High Market",
    "f4c58683-65a6-4df9-8952-458e397b1374",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// MMQ 321 — Mercadian Bazaar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIAN_BAZAAR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Mercadian Bazaar",
    "6f787cb6-78cb-4baa-a9cf-cee8b7d8d6b1",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MMQ 322 — Peat Bog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEAT_BOG: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Peat Bog",
    "bcc9d1e0-c8f4-4bac-90d4-8167f7a1515a",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// MMQ 323 — Remote Farm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMOTE_FARM: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Remote Farm",
    "115cab84-60d7-4bf2-9beb-b4ed7b5ceaf4",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// MMQ 324 — Rishadan Port
pub(in crate::card::sets) static RISHADAN_PORT: CardRecord = CardRecord::new(
    CardSet::MercadianMasques,
    "Rishadan Port",
    "477a1f53-5cdf-4b45-b584-2e36b31a3fdb",
    "Jerry Tiritilli",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{1}, {T}: Tap target land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// MMQ 325 — Rushwood Grove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHWOOD_GROVE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Rushwood Grove",
    "c315c72c-3e2f-4aff-b7d7-2f709ccec332",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// MMQ 326 — Sandstone Needle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDSTONE_NEEDLE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Sandstone Needle",
    "82bc7c6b-2e3d-42d1-b2bb-b37b6f34d33b",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MMQ 327 — Saprazzan Cove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_COVE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Cove",
    "52a69122-19c0-47ec-8bea-478511ba88e6",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MMQ 328 — Saprazzan Skerry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_SKERRY: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Saprazzan Skerry",
    "006871fd-2641-42cb-a2ac-a33d05fc5a35",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// MMQ 329 — Subterranean Hangar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBTERRANEAN_HANGAR: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Subterranean Hangar",
    "edc199d1-970b-489f-b713-8285151f16ae",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// MMQ 330 — Tower of the Magistrate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOWER_OF_THE_MAGISTRATE: CardRecord = CardRecord::new(
    crate::card::CardSet::MercadianMasques,
    "Tower of the Magistrate",
    "ee0481db-15ae-46b4-89a3-01c95a9626c7",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// MMQ 331 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "2edf5042-d185-424e-922d-c0bd4ce3e8b0",
    "Terry Springer",
);

// MMQ 332 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "44214f36-8bb3-4a32-8046-3ecdfff8407b",
    "Scott Bailey",
);

// MMQ 333 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "e3e536cc-e724-43d4-9fe3-dfb4952613cb",
    "Dana Knutson",
);

// MMQ 334 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "3a8fd867-8be1-4ee7-bb67-32c3f22db59e",
    "Edward P. Beard, Jr.",
);

// MMQ 335 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "5bae77e8-1230-4a6e-8c75-c99d2741a509",
    "Terry Springer",
);

// MMQ 336 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "9a38509a-2b74-42a0-af91-ed453e463b95",
    "Scott Bailey",
);

// MMQ 337 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "b2d83856-2201-4c30-bfcf-9cab62545201",
    "Scott Bailey",
);

// MMQ 338 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "e0fedd66-e547-492c-ad0d-9c7b527bdd17",
    "Tony Szczudlo",
);

// MMQ 339 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "72020810-bfa3-42d5-ad0d-6d02a6fe1b31",
    "Jeff Easley",
);

// MMQ 340 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "c2436ceb-05c0-40e6-b370-a6f02f4adbe4",
    "Rob Alexander",
);

// MMQ 341 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "1017347b-6b1a-4a2f-9147-98acad779616",
    "Rob Alexander",
);

// MMQ 342 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "4a0243d2-5fde-489f-8113-4ece0511cb5c",
    "Terry Springer",
);

// MMQ 343 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "19b5fff1-7a60-4e50-893a-8177cd62bf82",
    "Terry Springer",
);

// MMQ 344 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "4dbd12ed-e512-43d8-919d-478b18674deb",
    "Scott Bailey",
);

// MMQ 345 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "1921ce16-8ed8-41d7-a2b4-9e62f44ac8d6",
    "Dana Knutson",
);

// MMQ 346 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "423f4311-9feb-4c63-8b4c-32ddd38382e0",
    "Rob Alexander",
);

// MMQ 347 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "695de19e-801f-4f08-b44c-b0726e4aced0",
    "Donato Giancola",
);

// MMQ 348 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "a38e4ee7-6965-4e12-95d4-c9de1dbb014c",
    "Rob Alexander",
);

// MMQ 349 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "c1973049-2d42-4091-9703-189ba374254d",
    "Rob Alexander",
);

// MMQ 350 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "98c4806b-a31a-4026-9876-eab4d0d1694b",
    "Terry Springer",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALABASTER_WALL,
    &ARMISTICE,
    &ARREST,
    &BALLISTA_SQUAD,
    &CHARM_PEDDLER,
    &CHARMED_GRIFFIN,
    &CHO_ARRIM_ALCHEMIST,
    &CHO_ARRIM_BRUISER,
    &CHO_ARRIM_LEGATE,
    &CHO_MANNO_REVOLUTIONARY,
    &CHO_MANNO_S_BLESSING,
    &COMMON_CAUSE,
    &CORNERED_MARKET,
    &CRACKDOWN,
    &CROSSBOW_INFANTRY,
    &DEVOUT_WITNESS,
    &FOUNTAIN_WATCH,
    &FRESH_VOLUNTEERS,
    &HONOR_THE_FALLEN,
    &IGNOBLE_SOLDIER,
    &INVIOLABILITY,
    &IVORY_MASK,
    &JHOVALL_QUEEN,
    &JHOVALL_RIDER,
    &LAST_BREATH,
    &MOMENT_OF_SILENCE,
    &MOONLIT_WAKE,
    &MUZZLE,
    &NIGHTWIND_GLIDER,
    &NOBLE_PURPOSE,
    &ORIM_S_CURE,
    &PIOUS_WARRIOR,
    &RAMOSIAN_CAPTAIN,
    &RAMOSIAN_COMMANDER,
    &RAMOSIAN_LIEUTENANT,
    &RAMOSIAN_RALLY,
    &RAMOSIAN_SERGEANT,
    &RAMOSIAN_SKY_MARSHAL,
    &RAPPELLING_SCOUTS,
    &RENOUNCE,
    &REVERED_ELDER,
    &REVERENT_MANTRA,
    &RIGHTEOUS_INDIGNATION,
    &SECURITY_DETAIL,
    &SOOTHING_BALM,
    &SPIRITUAL_FOCUS,
    &STEADFAST_GUARD,
    &STORY_CIRCLE,
    &TASK_FORCE,
    &THERMAL_GLIDER,
    &TONIC_PEDDLER,
    &TRAP_RUNNER,
    &WAVE_OF_RECKONING,
    &WISHMONGER,
    &AERIAL_CARAVAN,
    &BALLOON_PEDDLER,
    &BLOCKADE_RUNNER,
    &BRIBERY,
    &BUOYANCY,
    &CHAMBERED_NAUTILUS,
    &CHAMELEON_SPIRIT,
    &CHARISMA,
    &CLOUD_SPRITE,
    &COASTAL_PIRACY,
    &COWARDICE,
    &CUSTOMS_DEPOT,
    &DARTING_MERFOLK,
    &DEHYDRATION,
    &DIPLOMATIC_ESCORT,
    &DIPLOMATIC_IMMUNITY,
    &DRAKE_HATCHLING,
    &EMBARGO,
    &EXTRAVAGANT_SPIRIT,
    &GLOWING_ANEMONE,
    &GUSH,
    &HIGH_SEAS,
    &HOODWINK,
    &INDENTURED_DJINN,
    &KARN_S_TOUCH,
    &MISDIRECTION,
    &MISSTEP,
    &OVERTAKER,
    &PORT_INSPECTOR,
    &RISHADAN_AIRSHIP,
    &RISHADAN_BRIGAND,
    &RISHADAN_CUTPURSE,
    &RISHADAN_FOOTPAD,
    &SAILMONGER,
    &SAND_SQUID,
    &SAPRAZZAN_BAILIFF,
    &SAPRAZZAN_BREAKER,
    &SAPRAZZAN_HEIR,
    &SAPRAZZAN_LEGATE,
    &SAPRAZZAN_OUTRIGGER,
    &SAPRAZZAN_RAIDER,
    &SHOVING_MATCH,
    &SOOTHSAYING,
    &SQUEEZE,
    &STATECRAFT,
    &STINGING_BARRIER,
    &THWART,
    &TIDAL_BORE,
    &TIDAL_KRAKEN,
    &TRADE_ROUTES,
    &WAR_TAX,
    &WATERFRONT_BOUNCER,
    &ALLEY_GRIFTERS,
    &BLACK_MARKET,
    &BOG_SMUGGLERS,
    &BOG_WITCH,
    &CACKLING_WITCH,
    &CATERAN_BRUTE,
    &CATERAN_ENFORCER,
    &CATERAN_KIDNAPPERS,
    &CATERAN_OVERLORD,
    &CATERAN_PERSUADER,
    &CATERAN_SLAVER,
    &CATERAN_SUMMONS,
    &CONSPIRACY,
    &CORRUPT_OFFICIAL,
    &DEATHGAZER,
    &DEEPWOOD_GHOUL,
    &DEEPWOOD_LEGATE,
    &DELRAICH,
    &ENSLAVED_HORROR,
    &EXTORTION,
    &FORCED_MARCH,
    &GHOUL_S_FEAST,
    &HAUNTED_CROSSROADS,
    &HIGHWAY_ROBBER,
    &INSTIGATOR,
    &INSUBORDINATION,
    &INTIMIDATION,
    &LARCENY,
    &LIABILITY,
    &MAGGOT_THERAPY,
    &MIDNIGHT_RITUAL,
    &MISSHAPEN_FIEND,
    &MOLTING_HARPY,
    &NETHER_SPIRIT,
    &NOTORIOUS_ASSASSIN,
    &PRETENDER_S_CLAIM,
    &PRIMEVAL_SHAMBLER,
    &PUTREFACTION,
    &QUAGMIRE_LAMPREY,
    &RAMPART_CRAWLER,
    &ROUSE,
    &SCANDALMONGER,
    &SEVER_SOUL,
    &SILENT_ASSASSIN,
    &SKULKING_FUGITIVE,
    &SNUFF_OUT,
    &SOUL_CHANNELING,
    &SPECTER_S_WAIL,
    &STRONGARM_THUG,
    &THRASHING_WUMPUS,
    &UNDERTAKER,
    &UNMASK,
    &UNNATURAL_HUNGER,
    &VENDETTA,
    &WALL_OF_DISTORTION,
    &ARMS_DEALER,
    &BATTLE_RAMPART,
    &BATTLE_SQUADRON,
    &BLASTER_MAGE,
    &BLOOD_HOUND,
    &BLOOD_OATH,
    &BRAWL,
    &CAVE_SENSE,
    &CAVE_IN,
    &CAVERN_CRAWLER,
    &CEREMONIAL_GUARD,
    &CINDER_ELEMENTAL,
    &CLOSE_QUARTERS,
    &CRAG_SAURIAN,
    &CRASH,
    &FLAILING_MANTICORE,
    &FLAILING_OGRE,
    &FLAILING_SOLDIER,
    &FLAMING_SWORD,
    &FURIOUS_ASSAULT,
    &GERRARD_S_IRREGULARS,
    &HAMMER_MAGE,
    &HIRED_GIANT,
    &KRIS_MAGE,
    &KYREN_GLIDER,
    &KYREN_LEGATE,
    &KYREN_NEGOTIATIONS,
    &KYREN_SNIPER,
    &LAVA_RUNNER,
    &LIGHTNING_HOUNDS,
    &LITHOPHAGE,
    &LUNGE,
    &MAGISTRATE_S_VETO,
    &MERCADIA_S_DOWNFALL,
    &PULVERIZE,
    &PUPPET_S_VERDICT,
    &ROBBER_FLY,
    &ROCK_BADGER,
    &SEISMIC_MAGE,
    &SHOCK_TROOPS,
    &SIZZLE,
    &SQUEE_GOBLIN_NABOB,
    &TECTONIC_BREAK,
    &TERRITORIAL_DISPUTE,
    &THIEVES_AUCTION,
    &THUNDERCLAP,
    &TWO_HEADED_DRAGON,
    &UPHILL_BATTLE,
    &VOLCANIC_WIND,
    &WAR_CADENCE,
    &WARMONGER,
    &WARPATH,
    &WILD_JHOVALL,
    &ANCESTRAL_MASK,
    &BIFURCATE,
    &BOA_CONSTRICTOR,
    &BRIAR_PATCH,
    &CALLER_OF_THE_HUNT,
    &CAUSTIC_WASPS,
    &CLEAR_THE_LAND,
    &COLLECTIVE_UNCONSCIOUS,
    &DAWNSTRIDER,
    &DEEPWOOD_DRUMMER,
    &DEEPWOOD_ELDER,
    &DEEPWOOD_TANTIV,
    &DEEPWOOD_WOLVERINE,
    &ERITHIZON,
    &FEROCITY,
    &FOOD_CHAIN,
    &FOSTER,
    &GAME_PRESERVE,
    &GROUNDSKEEPER,
    &HORNED_TROLL,
    &HOWLING_WOLF,
    &HUNTED_WUMPUS,
    &INVIGORATE,
    &LAND_GRANT,
    &LEY_LINE,
    &LUMBERING_SATYR,
    &MEGATHERIUM,
    &NATURAL_AFFINITY,
    &PANGOSAUR,
    &REVIVE,
    &RUSHWOOD_DRYAD,
    &RUSHWOOD_ELEMENTAL,
    &RUSHWOOD_HERBALIST,
    &RUSHWOOD_LEGATE,
    &SABER_ANTS,
    &SACRED_PREY,
    &SILVERGLADE_ELEMENTAL,
    &SILVERGLADE_PATHFINDER,
    &SNAKE_PIT,
    &SNORTING_GAHR,
    &SPIDERSILK_ARMOR,
    &SPONTANEOUS_GENERATION,
    &SQUALLMONGER,
    &STAMINA,
    &SUSTENANCE,
    &TIGER_CLAWS,
    &VENOMOUS_DRAGONFLY,
    &VERNAL_EQUINOX,
    &VINE_DRYAD,
    &VINE_TRELLIS,
    &ASSEMBLY_HALL,
    &BARBED_WIRE,
    &BARGAINING_TABLE,
    &CREDIT_VOUCHER,
    &CRENELLATED_WALL,
    &CROOKED_SCALES,
    &CRUMBLING_SANCTUARY,
    &DISTORTING_LENS,
    &EYE_OF_RAMOS,
    &GENERAL_S_REGALIA,
    &HEART_OF_RAMOS,
    &HENGE_GUARDIAN,
    &HORN_OF_PLENTY,
    &HORN_OF_RAMOS,
    &IRON_LANCE,
    &JEWELED_TORQUE,
    &KYREN_ARCHIVE,
    &KYREN_TOY,
    &MAGISTRATE_S_SCEPTER,
    &MERCADIAN_ATLAS,
    &MERCADIAN_LIFT,
    &MONKEY_CAGE,
    &PANACEA,
    &POWER_MATRIX,
    &PUFFER_EXTRACT,
    &RISHADAN_PAWNSHOP,
    &SKULL_OF_RAMOS,
    &TOOTH_OF_RAMOS,
    &TOYMAKER,
    &WORRY_BEADS,
    &DUST_BOWL,
    &FOUNTAIN_OF_CHO,
    &HENGE_OF_RAMOS,
    &HICKORY_WOODLOT,
    &HIGH_MARKET,
    &MERCADIAN_BAZAAR,
    &PEAT_BOG,
    &REMOTE_FARM,
    &RISHADAN_PORT,
    &RUSHWOOD_GROVE,
    &SANDSTONE_NEEDLE,
    &SAPRAZZAN_COVE,
    &SAPRAZZAN_SKERRY,
    &SUBTERRANEAN_HANGAR,
    &TOWER_OF_THE_MAGISTRATE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    AFTERLIFE_REPRINT,
    DISENCHANT_REPRINT,
    RIGHTEOUS_AURA_REPRINT,
    BRAINSTORM_REPRINT,
    COUNTERSPELL_REPRINT,
    ENERGY_FLUX_REPRINT,
    FALSE_DEMISE_REPRINT,
    TIMID_DRAKE_REPRINT,
    DARK_RITUAL_REPRINT,
    RAIN_OF_TEARS_REPRINT,
    OGRE_TASKMASTER_REPRINT,
    STONE_RAIN_REPRINT,
    TREMOR_REPRINT,
    WORD_OF_BLASTING_REPRINT,
    DEADLY_INSECT_REPRINT,
    DESERT_TWISTER_REPRINT,
    GIANT_CATERPILLAR_REPRINT,
    LURE_REPRINT,
    SQUALL_REPRINT,
    TRANQUILITY_REPRINT,
    VENOMOUS_BREATH_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
];
