//! Invasion cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::CardSupertype;
use crate::ResolvedEffectDurationDef;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::tempest as catalog_tmp;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::sets::y1998::exodus as catalog_exo;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AdditionalCostValueDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardRules, CardSet,
    CardType, ChoiceVisibilityDef, ChooseGroupDef, EffectDef, EffectRecipientDef, ManaColor,
    MoveObjectsDef, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PartitionGroupDef,
    PlayerRefDef, PlayerRelation, RevealObjectsDef, TriggerConditionDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::{Binding, ParentBinding, TargetIndex, mana_cost};

// INV 1 — Alabaster Leech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_LEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Alabaster Leech",
    "c86b45d9-aba6-4c09-8605-037754ba7fd4",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// INV 2 — Angel of Mercy (reprint)
const ANGEL_OF_MERCY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::ANGEL_OF_MERCY,
    "5b6de688-685f-4389-be35-a472ada988e1",
    "Mark Tedin",
);

// INV 3 — Ardent Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENT_SOLDIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Ardent Soldier",
    "39dce974-846f-4365-b0a5-851e38668e7d",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// INV 4 — Atalya, Samite Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATALYA_SAMITE_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Atalya, Samite Master",
    "90500e7a-f76d-453a-bda0-d56d3f7c7534",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 5 — Benalish Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_EMISSARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Benalish Emissary",
    "6b82d56e-80d7-4be9-ac22-de3257efc458",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 6 — Benalish Heralds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_HERALDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Benalish Heralds",
    "13c6e51d-54eb-4e5b-9ec9-54521b16b8d1",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// INV 7 — Benalish Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_LANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Benalish Lancer",
    "3a38d40a-e745-4fee-b179-f8c27e9b2fbd",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// INV 8 — Benalish Trapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_TRAPPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Benalish Trapper",
    "e312653d-c3e1-4c79-90d2-0963419b618c",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// INV 9 — Blinding Light (reprint)
const BLINDING_LIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::BLINDING_LIGHT,
    "882c1e15-b508-4885-9626-4c8d2598a006",
    "Marc Fishman",
);

// INV 10 — Capashen Unicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPASHEN_UNICORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Capashen Unicorn",
    "ec3e5741-88d7-4837-9b43-ba8304d9ee74",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// INV 11 — Crimson Acolyte
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIMSON_ACOLYTE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Crimson Acolyte",
    "c1718028-3009-4bdd-9f6f-59c17edd1344",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// INV 12 — Crusading Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUSADING_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Crusading Knight",
    "a4ab4640-1871-41dd-bd21-64741e21ba37",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// INV 13 — Death or Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_OR_GLORY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Death or Glory",
    "81f967c9-b38d-489d-96cc-44a6b1804e10",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// INV 14 — Dismantling Blow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISMANTLING_BLOW: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Dismantling Blow",
    "39514d54-cb6c-4b3b-a3be-46db991be4d4",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// INV 15 — Divine Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVINE_PRESENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Divine Presence",
    "28cb898d-d6ce-410a-83bf-37962cca2735",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// INV 16 — Fight or Flight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIGHT_OR_FLIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Fight or Flight",
    "46bde162-3737-4b93-a27a-63b909a4183d",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 17 — Glimmering Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLIMMERING_ANGEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Glimmering Angel",
    "f14f55e4-eded-4a86-87f4-b8fa6f30bc0f",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// INV 18 — Global Ruin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOBAL_RUIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Global Ruin",
    "336474b4-2cf5-44c0-b72c-f75f1a7ed928",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 19 — Harsh Judgment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARSH_JUDGMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Harsh Judgment",
    "34c78dee-ab45-4638-b89a-10686145b19a",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// INV 20 — Holy Day (reprint)
const HOLY_DAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::HOLY_DAY,
    "aa91fd4e-4e1f-4cfa-b10f-456bd875238f",
    "Pete Venters",
);

// INV 21 — Liberate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIBERATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Liberate",
    "96794470-31ea-478f-b11c-dc8342a508e2",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 22 — Obsidian Acolyte
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBSIDIAN_ACOLYTE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Obsidian Acolyte",
    "868efcee-bb13-4b6f-b81b-99408685e4c4",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 23 — Orim's Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_TOUCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Orim's Touch",
    "559f551e-7891-4c6d-8798-a25c0255fa3b",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// INV 24 — Pledge of Loyalty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLEDGE_OF_LOYALTY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Pledge of Loyalty",
    "d6f98c26-5b30-400c-8af1-8c6c43065f63",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// INV 25 — Prison Barricade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISON_BARRICADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Prison Barricade",
    "449c4800-8718-4593-a61e-03ad7f348c6d",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// INV 26 — Protective Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROTECTIVE_SPHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Protective Sphere",
    "ef5ef13e-1cf0-42a9-95d0-30ade254d6a8",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 27 — Pure Reflection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURE_REFLECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Pure Reflection",
    "bbff85a6-a51b-424e-a86b-da52c9b3a9da",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 28 — Rampant Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMPANT_ELEPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rampant Elephant",
    "752642d2-3dad-4f58-b154-beb5982141dc",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 29 — Razorfoot Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAZORFOOT_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Razorfoot Griffin",
    "819e2046-9b78-4fd0-92f8-798bfac51195",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// INV 30 — Restrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTRAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Restrain",
    "f6b5c765-619c-4db9-b509-91892fb65e8f",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 31 — Reviving Dose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVIVING_DOSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Reviving Dose",
    "8d44dd88-ad20-4d89-8831-d2dfa6873428",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 32 — Rewards of Diversity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REWARDS_OF_DIVERSITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rewards of Diversity",
    "04116b38-8fb1-47c6-b68d-060d0fc4a60d",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// INV 33 — Reya Dawnbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REYA_DAWNBRINGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Reya Dawnbringer",
    "e1e0e72b-e65e-4578-b610-9f529daa32d7",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 34 — Rout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROUT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rout",
    "94bc55ed-b89b-4e22-b3f1-4ce0f8d180d7",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 35 — Ruham Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUHAM_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Ruham Djinn",
    "a46c7718-1ecc-418c-b213-13be9de5cb7f",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// INV 36 — Samite Ministration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_MINISTRATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Samite Ministration",
    "b1de62ed-79e6-4daf-a2ab-dc0726e1f7df",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// INV 37 — Shackles (reprint)
const SHACKLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::SHACKLES,
    "35b3da05-9a3e-4827-96b8-5de244128db3",
    "Greg Staples",
);

// INV 38 — Spirit of Resistance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_OF_RESISTANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Spirit of Resistance",
    "5fb66439-df73-4a01-a8d4-6f2334297fdf",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// INV 39 — Spirit Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_WEAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Spirit Weaver",
    "90b0ef47-cb22-4146-a17e-e49a6031a7e6",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 40 — Strength of Unity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRENGTH_OF_UNITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Strength of Unity",
    "1a9d4ff8-af35-413f-9aa2-f4c6e34fade2",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// INV 41 — Sunscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sunscape Apprentice",
    "a9d6bd19-77c9-4a1a-a2d5-6f9737693fea",
    "Stephanie Law",
    crate::card::CardRules::unsupported(),
);

// INV 42 — Sunscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sunscape Master",
    "ebb7203d-529d-45d2-8e03-cd342c153f38",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// INV 43 — Teferi's Care
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_CARE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Teferi's Care",
    "031b1cc1-4468-4bc5-85c0-c22dce131225",
    "Scott Bailey",
    crate::card::CardRules::unsupported(),
);

// INV 44 — Wayfaring Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAYFARING_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Wayfaring Giant",
    "57e45de5-0e8b-41d3-979b-ec5a29cac682",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 45 — Winnow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINNOW: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Winnow",
    "d61748dd-4010-47da-8717-ca0147877057",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// INV 46 — Barrin's Unmaking
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_S_UNMAKING: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Barrin's Unmaking",
    "4d4cecb0-12b5-4678-b5e7-8cec8fc86cef",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// INV 47 — Blind Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIND_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Blind Seer",
    "5c54ec26-c7f1-4258-9cc9-1709987f293c",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 48 — Breaking Wave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAKING_WAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Breaking Wave",
    "1b39cd77-97aa-4099-8405-366f82079758",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// INV 49 — Collective Restraint
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLECTIVE_RESTRAINT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Collective Restraint",
    "d71daa57-ac02-4dd9-8c90-d38bdd45fb51",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// INV 50 — Crystal Spray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_SPRAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Crystal Spray",
    "8798a4f1-34bb-449d-a8cc-faf8bda8e0ab",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// INV 51 — Disrupt (reprint)
const DISRUPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::DISRUPT,
    "c000a02f-6b7e-4925-a938-59e645e980d7",
    "Paolo Parente",
);

// INV 52 — Distorting Wake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISTORTING_WAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Distorting Wake",
    "cf48eec9-96be-4f53-9d9a-c6f02d44c995",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 53 — Dream Thrush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_THRUSH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Dream Thrush",
    "258217df-ae88-4d93-895a-3fd242baacd1",
    "D. J. Cleland-Hura",
    crate::card::CardRules::unsupported(),
);

// INV 54 — Empress Galina
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMPRESS_GALINA: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Empress Galina",
    "6851dbc7-f072-41e7-a899-897445d99425",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 55 — Essence Leak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_LEAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Essence Leak",
    "9099b2e6-9ed8-4a9c-97ca-77cc47678228",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 56 — Exclude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCLUDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Exclude",
    "aeb359c8-209c-455f-84b2-970e5678a9fa",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// INV 57 — Fact or Fiction
const FACT_FIRST: Binding = Binding!("fact_first");
const FACT_SECOND: Binding = Binding!("fact_second");
const FACT_CHOSEN: Binding = Binding!("fact_chosen");
const FACT_UNCHOSEN: Binding = Binding!("fact_unchosen");

pub(in crate::card::sets) static FACT_OR_FICTION: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Fact or Fiction",
    "7fd4d018-dcf3-4439-8445-02d66e44f7d3",
    "Terese Nielsen",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard.",
        abilities::bind_top_cards_then(
            PlayerRefDef::EffectController,
            ValueDef::Constant(5),
            &const { EffectDef::Sequence(&[
                EffectDef::RevealObjects(RevealObjectsDef {
                    input: ObjectSetDef::Binding(ParentBinding),
                    then: &EffectDef::None,
                }),
                EffectDef::PartitionGroup(PartitionGroupDef {
                    actor: PlayerRefDef::Opponent,
                    input: ObjectSetDef::Binding(ParentBinding),
                    first: FACT_FIRST,
                    second: FACT_SECOND,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &const { EffectDef::ChooseGroup(ChooseGroupDef {
                        actor: PlayerRefDef::EffectController,
                        first: ObjectSetDef::Binding(FACT_FIRST),
                        second: ObjectSetDef::Binding(FACT_SECOND),
                        chosen: FACT_CHOSEN,
                        unchosen: FACT_UNCHOSEN,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &const { EffectDef::Sequence(&[
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(FACT_CHOSEN),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Hand,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(FACT_UNCHOSEN),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Graveyard,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                        ]) },
                    }) },
                }),
            ]) },
        ),
    )),
);

// INV 58 — Faerie Squadron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_SQUADRON: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Faerie Squadron",
    "4c707c81-dbbd-43be-a79a-7bc92a584839",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// INV 59 — Mana Maze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_MAZE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Mana Maze",
    "0d62cc17-8fa3-495c-a098-ffbbec89fa53",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 60 — Manipulate Fate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANIPULATE_FATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Manipulate Fate",
    "5bb52acb-dedb-4ed6-a6da-8c036f2b2958",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// INV 61 — Metathran Aerostat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_AEROSTAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Metathran Aerostat",
    "59f34850-fb6f-4ac5-8309-4d53d770e28c",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 62 — Metathran Transport
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_TRANSPORT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Metathran Transport",
    "4fa9048d-1599-44a5-b4b2-45382c5b238d",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 63 — Metathran Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Metathran Zombie",
    "6676a0f7-8213-4547-b2ac-b904cd418073",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 64 — Opt
pub(in crate::card::sets) static OPT: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Opt",
    "958262ec-8e52-40cf-a9fd-a60e42643e15",
    "John Howe",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Scry 1.\nDraw a card.",
        EffectDef::Sequence(&[
            abilities::scry(ValueDef::Constant(1)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// INV 65 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "ea56a1bb-f52c-4c6b-a089-1f78600f3db0",
    "Dana Knutson",
);

// INV 66 — Probe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROBE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Probe",
    "a2a58d18-3d52-4178-86b2-7590d4164e76",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 67 — Prohibit
pub(in crate::card::sets) static PROHIBIT: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Prohibit",
    "0daa5458-2a97-40d0-b18d-2381a7a68ee1",
    "Adam Rex",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        abilities::kicker(mana_cost!("{2}")),
        AbilityDef::spell_with_targets(
            "Counter target spell if its mana value is 2 or less. If this spell was kicked, counter that spell if its mana value is 4 or less instead.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMostValue(
                        ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                            crate::AdditionalCostIndex::PRIMARY,
                            ValueDef::Constant(4),
                            ValueDef::Constant(2),
                        )),
                    ),
                },
                then: &EffectDef::Counter {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                },
            },
        ),
    ]),
);

// INV 68 — Psychic Battle
// Audit: unsupported — The final target change is supported, but this still needs an event for choosing spell-or-ability targets, simultaneous top-card reveals by every player, and repeat-until-untied highest-mana-value selection.
pub(in crate::card::sets) static PSYCHIC_BATTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Psychic Battle",
    "8758ca24-e613-43bf-be58-4cf557f82d0c",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// INV 69 — Rainbow Crow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAINBOW_CROW: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rainbow Crow",
    "7e622ad2-473f-489e-b4cf-bbdcc44d0cde",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// INV 70 — Repulse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPULSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Repulse",
    "9a04e9be-48be-440e-9825-cfffd4c2b1a4",
    "Aaron Boyd",
    crate::card::CardRules::unsupported(),
);

// INV 71 — Sapphire Leech
pub(in crate::card::sets) static SAPPHIRE_LEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sapphire Leech",
    "e6763ffd-9d89-4f26-871a-be24fbdef38d",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Leech"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::spell_cost_increase(
            "Blue spells you cast cost {U} more to cast.",
            ObjectPredicateDef::Color(ManaColor::Blue),
            PlayerRelation::You,
            mana_cost!("{U}"),
        ),
    ]),
);

// INV 72 — Shimmering Wings (reprint)
const SHIMMERING_WINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::SHIMMERING_WINGS,
    "9615a6c2-1732-4a04-9be1-cc0a8d39de3f",
    "Carl Critchlow",
);

// INV 73 — Shoreline Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHORELINE_RAIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Shoreline Raider",
    "d895b3b8-2acc-4c9f-8341-f651c1255b7c",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// INV 74 — Sky Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKY_WEAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sky Weaver",
    "04974146-42a8-4f10-b443-67bfeaa54d5d",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 75 — Stormscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Stormscape Apprentice",
    "1eb42f39-9187-44e4-aa34-14ab31977199",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 76 — Stormscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Stormscape Master",
    "9b704165-4587-48f1-8830-c5a07ec666cc",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// INV 77 — Sway of Illusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWAY_OF_ILLUSION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sway of Illusion",
    "ff65e386-9aec-4deb-a4ec-d9a97bd87645",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 78 — Teferi's Response
pub(in crate::card::sets) static TEFERIS_RESPONSE: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Teferi's Response",
    "f3bb2df8-c559-4a34-83b0-d48fbc694cc8",
    "Scott Bailey",
    // The answer to Wasteland and Dust Bowl: the land lives, the thing that
    // came for it dies, and two cards make the exchange worth a card.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell or ability an opponent controls that targets a land you control. If a permanent's ability is countered this way, destroy that permanent.\nDraw two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                // A land you control, read off what the spell or ability already targets.
                object: ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                zones: &[ZoneKind::Stack],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )],
        // The destroy follows the counter rather than preceding it: the countered
        // ability is retired with its source recorded, so the permanent is still
        // findable afterwards, and a spell -- which has no such source -- leaves
        // nothing to destroy.
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::object(ObjectRefDef::SourceOfTargetedStackObject(
                    TargetIndex::PRIMARY,
                )),
                can_regenerate: true,
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// INV 79 — Temporal Distortion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPORAL_DISTORTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Temporal Distortion",
    "74bd0d14-8d26-403f-9405-d0dcdecd1a49",
    "Stephanie Law",
    crate::card::CardRules::unsupported(),
);

// INV 80 — Tidal Visionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_VISIONARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tidal Visionary",
    "a72a3051-7f46-4b6b-b4fb-0f170d9687ab",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 81 — Tolarian Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_EMISSARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tolarian Emissary",
    "1cbc55e5-b84c-4449-a288-ec26cdd3997c",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 82 — Tower Drake
pub(in crate::card::sets) static TOWER_DRAKE: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Tower Drake",
    "aef97b38-f7a5-4db7-9550-24aa1a1ebbda",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// INV 83 — Traveler's Cloak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAVELER_S_CLOAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Traveler's Cloak",
    "977f0f82-0542-40c9-9a48-73077941dbd1",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 84 — Vodalian Hypnotist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_HYPNOTIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Vodalian Hypnotist",
    "721fd877-0a28-4002-8b47-058bac4ac44d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 85 — Vodalian Merchant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_MERCHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Vodalian Merchant",
    "c1c0effa-a4b8-4166-a66a-90cf01c6ea0d",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 86 — Vodalian Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Vodalian Serpent",
    "92adcf6c-ab14-414c-a5cb-56feae048c84",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 87 — Wash Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WASH_OUT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Wash Out",
    "7719d043-5827-4479-825b-23d9e979ead7",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 88 — Well-Laid Plans
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_LAID_PLANS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Well-Laid Plans",
    "1c55eb8f-925a-42c1-9e48-d7f99cab3b01",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 89 — Worldly Counsel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDLY_COUNSEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Worldly Counsel",
    "8fc66fbf-f411-4607-aece-7c35d9a07c80",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// INV 90 — Zanam Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZANAM_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Zanam Djinn",
    "57a3c1d5-0ca8-443b-ae7a-66e0363e377b",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 91 — Addle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADDLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Addle",
    "e8afb9d0-affa-4599-bf29-729cfe64703b",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// INV 92 — Agonizing Demise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGONIZING_DEMISE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Agonizing Demise",
    "539ac5e1-4bad-4f70-abac-e70c406bebec",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// INV 93 — Andradite Leech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANDRADITE_LEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Andradite Leech",
    "6da0d4f3-9216-406c-8f3e-b9bb0a11dc75",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// INV 94 — Annihilate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANNIHILATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Annihilate",
    "4a3bf039-ecf6-477e-997c-e32c55323c01",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 95 — Bog Initiate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_INITIATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Bog Initiate",
    "8962dc3b-24ca-4c3c-ba1d-933c29cf7b73",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// INV 96 — Cremate
pub(in crate::card::sets) static CREMATE: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Cremate",
    "1095cdfe-8060-4a73-bacf-9f983152b486",
    "Andrew Goldhawk",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target card from a graveyard. Draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// INV 97 — Crypt Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPT_ANGEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Crypt Angel",
    "522ddc6f-ec13-4a70-8f4c-b3c846b102fd",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// INV 98 — Cursed Flesh (reprint)
const CURSED_FLESH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::CURSED_FLESH,
    "fb151ae8-9281-434d-ba8d-9ce34f0875eb",
    "Chippy",
);

// INV 99 — Defiling Tears
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFILING_TEARS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Defiling Tears",
    "db7cba29-9472-4874-bd54-37edf70645b2",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// INV 100 — Desperate Research
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_RESEARCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Desperate Research",
    "6a42ac7e-4a27-488c-a2e7-338b18103b02",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 101 — Devouring Strossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOURING_STROSSUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Devouring Strossus",
    "064f013f-e74f-419d-8d17-7748bd91885e",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 102 — Do or Die
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DO_OR_DIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Do or Die",
    "05f63cd9-e82b-4cf8-b8ce-f0aa0157692b",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 103 — Dredge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREDGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Dredge",
    "68bfa3d5-0f0b-4684-9567-f1478da01df7",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// INV 104 — Duskwalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKWALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Duskwalker",
    "39a4a026-f44e-40e1-9942-a3d8448aca70",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 105 — Exotic Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXOTIC_CURSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Exotic Curse",
    "8ee35d99-9a8a-421b-bf43-74446909d87d",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// INV 106 — Firescreamer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRESCREAMER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Firescreamer",
    "155a2213-bf6e-4a54-924b-e450b7d06f26",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 107 — Goham Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOHAM_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Goham Djinn",
    "d67796c7-4d93-4c50-8839-bb69e075bc42",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 108 — Hate Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HATE_WEAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Hate Weaver",
    "8328e131-b44d-4dd0-9ce4-454c6afe6fa6",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// INV 109 — Hypnotic Cloud
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYPNOTIC_CLOUD: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Hypnotic Cloud",
    "a7502ea2-7555-449e-baee-6ecef5573a3b",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 110 — Marauding Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARAUDING_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Marauding Knight",
    "cea2a7de-c67e-4541-be8c-e5ef7b64d94a",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// INV 111 — Mourning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOURNING: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Mourning",
    "4649d881-709f-4ed0-91de-744d232a82f5",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// INV 112 — Nightscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Nightscape Apprentice",
    "7498ca4c-614a-4776-8886-0a6ed58520f6",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// INV 113 — Nightscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Nightscape Master",
    "d86174b8-dd9e-4ece-bc23-4f9ac50bccd3",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// INV 114 — Phyrexian Battleflies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_BATTLEFLIES: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Phyrexian Battleflies",
    "da27c489-c541-4b0d-a844-71aa65e55ceb",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// INV 115 — Phyrexian Delver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DELVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Phyrexian Delver",
    "e66d87a5-7b67-4ec5-b5e2-518d67123118",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// INV 116 — Phyrexian Infiltrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_INFILTRATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Phyrexian Infiltrator",
    "224b8254-553d-4d88-8163-1f15e1244bd2",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// INV 117 — Phyrexian Reaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_REAPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Phyrexian Reaper",
    "ccdd498b-1081-43fe-8193-518337a5a3ea",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// INV 118 — Phyrexian Slayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_SLAYER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Phyrexian Slayer",
    "5fa8c604-343f-4c94-ac25-439ab1845c19",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// INV 119 — Plague Spitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_SPITTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Plague Spitter",
    "8845e6bd-40ee-45ca-a099-53f19ff20a8a",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// INV 120 — Ravenous Rats (reprint)
const RAVENOUS_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::portal_second_age::RAVENOUS_RATS,
    "89e29069-add5-4099-b800-9f1e4402cc1a",
    "Tom Wänerstrand",
);

// INV 121 — Reckless Spite (reprint)
const RECKLESS_SPITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::RECKLESS_SPITE,
    "2412497b-cae5-444d-9beb-7761d15cd5c5",
    "Chippy",
);

// INV 122 — Recover
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECOVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Recover",
    "771e695b-24e1-4c65-81e0-1624bda646e7",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// INV 123 — Scavenged Weaponry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCAVENGED_WEAPONRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Scavenged Weaponry",
    "4e8072a9-2699-4c6c-9556-67d91bd67a4b",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 124 — Soul Burn (reprint)
const SOUL_BURN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SOUL_BURN,
    "70515cd2-97d5-4491-a758-bc7188fdc6dc",
    "Andrew Goldhawk",
);

// INV 124s — Soul Burn (alternate printing)
const SOUL_BURN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::SOUL_BURN,
    1,
    "6eb3278a-1a23-4e0a-b541-0c37b2bc3f3c",
    "Andrew Goldhawk",
);

// INV 124★ — Soul Burn (alternate printing)
const SOUL_BURN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::SOUL_BURN,
    2,
    "301c4e8e-0468-4e16-9be5-7feb7999226f",
    "Andrew Goldhawk",
);

// INV 125 — Spreading Plague
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPREADING_PLAGUE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Spreading Plague",
    "ac86055d-ce08-4b05-a92c-45e007ca0ba4",
    "Scott Bailey",
    crate::card::CardRules::unsupported(),
);

// INV 126 — Tainted Well
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAINTED_WELL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tainted Well",
    "2eec00a1-7e12-42d2-8f46-de8ab7323c2c",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// INV 127 — Trench Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRENCH_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Trench Wurm",
    "1b076f85-d1bf-491a-af9d-f35b8e1bd163",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// INV 128 — Tsabo's Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_S_ASSASSIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tsabo's Assassin",
    "0047302d-4e3d-4327-9bb2-ecd5b00b00e3",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 129 — Tsabo's Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_S_DECREE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tsabo's Decree",
    "0c1a0ebd-1add-49e6-b5e6-5b26abb1de88",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// INV 130 — Twilight's Call
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWILIGHT_S_CALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Twilight's Call",
    "3c97c8a5-33b3-4f7f-a224-bb4df7b4bcc0",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// INV 131 — Urborg Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_EMISSARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urborg Emissary",
    "e6912c71-1836-4e87-9a65-d577d903d03c",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 132 — Urborg Phantom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_PHANTOM: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urborg Phantom",
    "397355b9-5b67-4973-972e-3505c500d116",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// INV 133 — Urborg Shambler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_SHAMBLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urborg Shambler",
    "eaedd5c8-03c6-4bbb-bf83-632551830bd4",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 134 — Urborg Skeleton (alternate printing)
const URBORG_SKELETON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &URBORG_SKELETON,
    1,
    "6e522a62-fbca-4362-9006-d4356c525704",
    "Alan Pollack",
);

// INV 134s — Urborg Skeleton (alternate printing)
const URBORG_SKELETON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &URBORG_SKELETON,
    2,
    "6819c2f5-29a2-46d2-af36-c22b64338807",
    "Tom Wänerstrand",
);

// INV 134★ — Urborg Skeleton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_SKELETON: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urborg Skeleton",
    "467e9486-1604-4fa2-ab1f-be0d7a036798",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// INV 135 — Yawgmoth's Agenda
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAWGMOTH_S_AGENDA: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Yawgmoth's Agenda",
    "50f7ea7f-4f17-4f78-b68e-693e265ca829",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 136 — Ancient Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Ancient Kavu",
    "c8ccb5d0-735b-443f-addd-8b70f5f2c60d",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 137 — Bend or Break
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEND_OR_BREAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Bend or Break",
    "b76b6660-d4b2-44de-a1a7-8d00811f90f6",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 138 — Breath of Darigaaz
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREATH_OF_DARIGAAZ: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Breath of Darigaaz",
    "480bb7e3-df03-454d-ada0-592ef8a4a6f0",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 139 — Callous Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLOUS_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Callous Giant",
    "330028c4-8e91-4fe3-a87d-1660dfd2507e",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// INV 140 — Chaotic Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOTIC_STRIKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Chaotic Strike",
    "061df8e4-6947-4bbb-9fe7-52ca4fd95d65",
    "Massimiliano Frezzato",
    crate::card::CardRules::unsupported(),
);

// INV 141 — Collapsing Borders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLAPSING_BORDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Collapsing Borders",
    "cc019633-788e-4095-9610-6c0a432f7656",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 142 — Crown of Flames (reprint)
const CROWN_OF_FLAMES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::CROWN_OF_FLAMES,
    "5a46239c-3de7-48ca-8f5c-b51f307fd0e5",
    "Christopher Moeller",
);

// INV 143 — Firebrand Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIREBRAND_RANGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Firebrand Ranger",
    "ee05211e-cf08-4dea-9740-ed06f8682153",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// INV 144 — Ghitu Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_FIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Ghitu Fire",
    "78827acd-a526-411b-bd22-ab9b538c75dd",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 145 — Goblin Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SPY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Goblin Spy",
    "2a89a099-8805-4b26-babd-5d9f48ee406a",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 146 — Halam Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALAM_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Halam Djinn",
    "369ade1f-e909-47ae-bb01-19588269ad8f",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 147 — Hooded Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOODED_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Hooded Kavu",
    "5464b80a-22fe-42c7-a839-31667712fb2d",
    "John Howe",
    crate::card::CardRules::unsupported(),
);

// INV 148 — Kavu Aggressor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_AGGRESSOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Aggressor",
    "a2832ad3-ce7f-44d2-beb2-c95d982905a6",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 149 — Kavu Monarch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_MONARCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Monarch",
    "ea63dfd5-d8d7-45b8-8219-1cc2b3de5666",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// INV 150 — Kavu Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_RUNNER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Runner",
    "2bc1b462-4e3c-47cc-87c5-f6e29dd70c01",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// INV 151 — Kavu Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_SCOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Scout",
    "cbc2670d-a3f4-47c2-b424-01fd379ff186",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// INV 152 — Lightning Dart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_DART: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Lightning Dart",
    "54d05157-d154-4203-bf3e-add110cb1cee",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 153 — Loafing Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOAFING_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Loafing Giant",
    "fab5f738-04d0-44c9-88ec-28469b668040",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 154 — Mages' Contest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGES_CONTEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Mages' Contest",
    "c516861c-68d9-4d02-a343-689dba0526c6",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// INV 155 — Maniacal Rage (reprint)
const MANIACAL_RAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::MANIACAL_RAGE,
    "3d17886c-fffd-4f0d-b4da-4b5fba18b811",
    "Matt Cavotta",
);

// INV 156 — Obliterate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBLITERATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Obliterate",
    "cdabde40-2143-4677-b7b4-ea8fbf9b1f25",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 157 — Overload
pub(in crate::card::sets) static OVERLOAD: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Overload",
    "c91fca91-7296-422e-b251-d571b710ff71",
    "Gary Ruddell",
    // One mana answers a Lotus Petal or a Cursed Scroll; three answers most
    // of what a Premodern deck actually plays.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        abilities::kicker(mana_cost!("{2}")),
        AbilityDef::spell_with_targets(
            "Destroy target artifact if its mana value is 2 or less. If this spell was kicked, destroy that artifact if its mana value is 5 or less instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMostValue(
                        ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                            crate::AdditionalCostIndex::PRIMARY,
                            ValueDef::Constant(5),
                            ValueDef::Constant(2),
                        )),
                    ),
                },
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            },
        ),
    ]),
);

// INV 158 — Pouncing Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POUNCING_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Pouncing Kavu",
    "7e6e2e49-7bde-43c1-8caf-43d237dfc052",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 159 — Rage Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGE_WEAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rage Weaver",
    "a654295d-b63c-4025-bf36-899023a8ba1d",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// INV 160 — Rogue Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROGUE_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rogue Kavu",
    "61e1a445-129d-4bb9-a8b0-3f55e3e0bc58",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// INV 161 — Ruby Leech
pub(in crate::card::sets) static RUBY_LEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Ruby Leech",
    "be621b12-4f4e-43a6-b65e-da4223e742b5",
    "Jacques Bredy",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Leech"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::spell_cost_increase(
            "Red spells you cast cost {R} more to cast.",
            ObjectPredicateDef::Color(ManaColor::Red),
            PlayerRelation::You,
            mana_cost!("{R}"),
        ),
    ]),
);

// INV 162 — Savage Offensive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_OFFENSIVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Savage Offensive",
    "356744f3-e444-4f4e-bf00-80bb6b2ef76f",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 163 — Scarred Puma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARRED_PUMA: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Scarred Puma",
    "067ff95e-c4dc-41bb-9677-67f51a09b05a",
    "Aaron Boyd",
    crate::card::CardRules::unsupported(),
);

// INV 164 — Scorching Lava
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHING_LAVA: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Scorching Lava",
    "2a85437f-052e-494c-a9ee-265c4624a409",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// INV 165 — Searing Rays
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARING_RAYS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Searing Rays",
    "4f66ff2d-f2d2-4a6b-bf26-b510de60c0b6",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// INV 166 — Shivan Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_EMISSARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Shivan Emissary",
    "945c596e-492e-4cf5-857c-4ddbbdd78485",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// INV 167 — Shivan Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_HARVEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Shivan Harvest",
    "47dbd765-d7ea-4181-bd22-5c749ad081af",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// INV 168 — Skittish Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKITTISH_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Skittish Kavu",
    "be806378-50a7-4416-9d99-1ea2c1f2b7cb",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 169 — Skizzik
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIZZIK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Skizzik",
    "dc7732bc-e168-44d9-923a-db7e985bd6db",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 170 — Slimy Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIMY_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Slimy Kavu",
    "8e82044d-88cd-4ee4-8ec9-e71a0a85ed46",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 171 — Stand or Fall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAND_OR_FALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Stand or Fall",
    "60c34970-a106-490c-ac37-6156eb7f34ce",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 172 — Stun (reprint)
const STUN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::STUN,
    "d22f3ae8-a40b-4dab-abf4-3ab7b05191f7",
    "Mike Ploog",
);

// INV 173 — Tectonic Instability
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TECTONIC_INSTABILITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tectonic Instability",
    "0476cc6b-ecc6-44d6-9f44-a90d4ee85daa",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// INV 174 — Thunderscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Thunderscape Apprentice",
    "75a0b075-5414-48d3-a2b1-47dc20213e96",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 175 — Thunderscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Thunderscape Master",
    "22abdc2f-bdc8-46c4-8ce2-f06befedbc32",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 176 — Tribal Flames
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_FLAMES: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tribal Flames",
    "9b32531e-c759-4603-abd0-1724e8df70db",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// INV 177 — Turf Wound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURF_WOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Turf Wound",
    "91392e9f-f96a-4ac5-b1f1-c73540cf249e",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// INV 178 — Urza's Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_RAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urza's Rage",
    "61a25a35-3ae4-471e-adcd-d8baf2f77b68",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 179 — Viashino Grappler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_GRAPPLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Viashino Grappler",
    "4a94aeb4-349c-4394-848d-c1c9133856e2",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// INV 180 — Zap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZAP: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Zap",
    "7502ce01-b762-40fe-a064-c7b20b08a722",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// INV 181 — Aggressive Urge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRESSIVE_URGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Aggressive Urge",
    "37e3154d-9b1c-4f93-9bc3-a39e68d59d23",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 182 — Bind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Bind",
    "cfa51783-9ef8-4e51-ba0d-ce8439d83bdf",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// INV 183 — Blurred Mongoose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLURRED_MONGOOSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Blurred Mongoose",
    "4b073e3f-6a6f-495a-ab16-39d906b660f1",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 184 — Canopy Surge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANOPY_SURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Canopy Surge",
    "2e19d68e-7554-4627-a316-beb1f75fa494",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 185 — Elfhame Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELFHAME_SANCTUARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Elfhame Sanctuary",
    "6ab9a90c-5fd8-4f8c-b692-f98a2974810c",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// INV 186 — Elvish Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_CHAMPION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Elvish Champion",
    "c19bb473-03b0-4e6d-a7da-0ec1e7707a68",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 187 — Explosive Growth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPLOSIVE_GROWTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Explosive Growth",
    "eabc1e77-404c-436b-bde1-be1b21d00584",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 188 — Fertile Ground (reprint)
const FERTILE_GROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::FERTILE_GROUND,
    "789e3582-b541-4916-ac7e-015214d7a27a",
    "Carl Critchlow",
);

// INV 189 — Harrow (reprint)
const HARROW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::HARROW,
    "ed0f633e-7238-4d02-ad8b-06dd20453030",
    "Rob Alexander",
);

// INV 190 — Jade Leech
pub(in crate::card::sets) static JADE_LEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Jade Leech",
    "3392171d-ed25-46a1-91cc-a4f24537617d",
    "John Howe",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Leech"], 5, 5).with_ability(
        abilities::spell_cost_increase(
            "Green spells you cast cost {G} more to cast.",
            ObjectPredicateDef::Color(ManaColor::Green),
            PlayerRelation::You,
            mana_cost!("{G}"),
        ),
    ),
);

// INV 191 — Kavu Chameleon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_CHAMELEON: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Chameleon",
    "f726437b-a41a-4ee9-b0ee-e09327508615",
    "John Howe",
    crate::card::CardRules::unsupported(),
);

// INV 192 — Kavu Climber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_CLIMBER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Climber",
    "2063f31e-d972-411e-a265-1d409153b49c",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// INV 193 — Kavu Lair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_LAIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Lair",
    "f4581b53-23a0-4ca6-a77c-97d79e7a6570",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// INV 194 — Kavu Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_TITAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kavu Titan",
    "2c5fb86d-1d9a-4da2-bb5b-4266faa20197",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// INV 195 — Llanowar Cavalry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_CAVALRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Llanowar Cavalry",
    "21d92191-a743-4916-bbe4-5e207e964d9b",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 196 — Llanowar Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_ELITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Llanowar Elite",
    "3e207863-de68-47e1-8c63-413b5fa48943",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 197 — Llanowar Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_VANGUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Llanowar Vanguard",
    "72e6ed79-bdfd-49f9-bfa4-be4196880487",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 198 — Might Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIGHT_WEAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Might Weaver",
    "032a4ec7-82ce-4ea0-b0dd-ebc40823a014",
    "Larry Elmore",
    crate::card::CardRules::unsupported(),
);

// INV 199 — Molimo, Maro-Sorcerer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLIMO_MARO_SORCERER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Molimo, Maro-Sorcerer",
    "750d3475-ae72-42c1-ae4d-638f8e7c6d1a",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// INV 200 — Nomadic Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOMADIC_ELF: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Nomadic Elf",
    "3b69e57a-5b19-450c-9cf5-c189e8505781",
    "D. J. Cleland-Hura",
    crate::card::CardRules::unsupported(),
);

// INV 201 — Pincer Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINCER_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Pincer Spider",
    "23271658-19ae-420d-beeb-4bed4fdbb891",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// INV 202 — Pulse of Llanowar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULSE_OF_LLANOWAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Pulse of Llanowar",
    "db09afe5-5f01-4f77-a239-12d7a6e59024",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 203 — Quirion Elves (reprint)
const QUIRION_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::QUIRION_ELVES,
    "c660a748-82a9-4d6a-8023-56aeafe1bdce",
    "Douglas Shuler",
);

// INV 204 — Quirion Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_SENTINEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Quirion Sentinel",
    "2fc639ea-a925-4f1e-879f-b8fcb12bf257",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 205 — Quirion Trailblazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_TRAILBLAZER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Quirion Trailblazer",
    "c2b258c1-5fb4-4072-bb32-ad364df1874a",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 206 — Restock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTOCK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Restock",
    "11a013ff-7c99-445a-b9e0-0fc45036f068",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// INV 207 — Rooting Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTING_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rooting Kavu",
    "12c25a4c-d93a-402b-999f-0b9919123cc5",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 208 — Saproling Infestation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_INFESTATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Saproling Infestation",
    "8642e530-914c-4149-944a-c4966ee27299",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 209 — Saproling Symbiosis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_SYMBIOSIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Saproling Symbiosis",
    "2bb63748-5c84-43a0-8f17-a2a17f658337",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// INV 210 — Scouting Trek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCOUTING_TREK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Scouting Trek",
    "1b882e68-5c03-4ec6-9982-8c3b09847969",
    "Stephanie Law",
    crate::card::CardRules::unsupported(),
);

// INV 211 — Serpentine Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENTINE_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Serpentine Kavu",
    "699f1fe8-02c6-4d95-9231-3f8aefe603da",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 212 — Sulam Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULAM_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sulam Djinn",
    "7aeab16f-e104-47e7-81c7-b6e0123120d7",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 213 — Tangle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tangle",
    "6b37e39c-8aa4-4938-a492-7dac5de98dfb",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// INV 214 — Thicket Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THICKET_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Thicket Elemental",
    "f80a56ed-3ebb-4e20-bf6a-e27127f762e8",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 215 — Thornscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Thornscape Apprentice",
    "505da522-73a8-4232-ae1a-d3365f3e598f",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 216 — Thornscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Thornscape Master",
    "7e8f164d-3782-4eaa-a4db-ab7082d45ee7",
    "Larry Elmore",
    crate::card::CardRules::unsupported(),
);

// INV 217 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "97019ba5-ce2a-460c-8a4e-2b22053ced65",
    "Rob Alexander",
);

// INV 218 — Treefolk Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREEFOLK_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Treefolk Healer",
    "73c6f5c0-686d-4b3a-add7-487f9fff5faa",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 219 — Utopia Tree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UTOPIA_TREE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Utopia Tree",
    "720452e9-3245-4b0e-94b6-843cbcb641a5",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// INV 220 — Verdeloth the Ancient
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDELOTH_THE_ANCIENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Verdeloth the Ancient",
    "72d5fab1-fa20-4006-b19d-179d36238c9b",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// INV 221 — Verduran Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDURAN_EMISSARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Verduran Emissary",
    "55f3361b-e2e7-4297-85c2-94323f90cc90",
    "Alton Lawson",
    crate::card::CardRules::unsupported(),
);

// INV 222 — Vigorous Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIGOROUS_CHARGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Vigorous Charge",
    "af6f57ad-d370-4c81-8da0-c15d87725ab1",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 223 — Wallop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALLOP: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Wallop",
    "45ce5126-e7b1-41ab-9e56-1e12927c4d27",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// INV 224 — Wandering Stream
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERING_STREAM: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Wandering Stream",
    "6da5cb6c-253b-44f0-98f9-d75f42c6e14b",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// INV 225 — Whip Silk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIP_SILK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Whip Silk",
    "10566804-fd15-4ef0-ad7d-cc979f4cc8c5",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 226 — Absorb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABSORB: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Absorb",
    "5d6a0f3e-457f-41f5-be26-5fb249874f1a",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// INV 227 — Aether Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_RIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Aether Rift",
    "692c186a-997c-4f7e-a339-bf84884e1019",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 228 — Angelic Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_SHIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Angelic Shield",
    "5aaa3e4e-4e08-4df2-9e0c-66e15a10fec4",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 229 — Armadillo Cloak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMADILLO_CLOAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Armadillo Cloak",
    "9d816f98-6cb6-432c-b0a4-a0eed21658ac",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// INV 230 — Armored Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMORED_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Armored Guardian",
    "6de5e1bd-1d31-4f9f-b18d-d6f49bc7ef10",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 231 — Artifact Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARTIFACT_MUTATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Artifact Mutation",
    "d5eef49c-a80f-4622-ba77-999f9151c841",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 232 — Aura Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_MUTATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Aura Mutation",
    "38421179-615e-4aba-91a4-503bfee05403",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 233 — Aura Shards
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_SHARDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Aura Shards",
    "df4039ef-af72-4267-ade9-fdb7c921279e",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 234 — Backlash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BACKLASH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Backlash",
    "dadf030d-5451-43fc-bf0c-c1629fdf88ec",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// INV 235 — Barrin's Spite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_S_SPITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Barrin's Spite",
    "6d8ec4dc-c74a-4d49-856e-95703675fe9b",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// INV 236 — Blazing Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLAZING_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Blazing Specter",
    "3bd397be-0e61-4f41-b0cf-f0c9d2440da7",
    "Marc Fishman",
    crate::card::CardRules::unsupported(),
);

// INV 237 — Captain Sisay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_SISAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Captain Sisay",
    "d24d441c-f37f-44fe-8a93-f5c89df807e4",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// INV 238 — Cauldron Dance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAULDRON_DANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Cauldron Dance",
    "8dadcae0-f2b2-487c-bb93-0a2c073044c0",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// INV 239 — Charging Troll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_TROLL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Charging Troll",
    "58956099-6b97-4c7b-ab23-9f9b4d50ef95",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 240 — Cinder Shade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_SHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Cinder Shade",
    "b8dd933a-19ed-4d30-a94a-bfb2f66f8f13",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// INV 241 — Coalition Victory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COALITION_VICTORY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Coalition Victory",
    "dd8ad3aa-3225-45ae-8343-5991f5b52269",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 242 — Crosis, the Purger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_THE_PURGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Crosis, the Purger",
    "e5f336d8-12a4-482d-8ffd-c205858c72ba",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 243 — Darigaaz, the Igniter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_THE_IGNITER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Darigaaz, the Igniter",
    "54dcf5e3-4303-41a3-b54c-24a9d462ce07",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// INV 244 — Dromar, the Banisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_THE_BANISHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Dromar, the Banisher",
    "cfcc3c72-fff5-454c-814c-eb952fd23ba9",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 245 — Dueling Grounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUELING_GROUNDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Dueling Grounds",
    "52760183-bee0-4ce0-96c0-074b88f78980",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 246 — Fires of Yavimaya
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRES_OF_YAVIMAYA: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Fires of Yavimaya",
    "967f1658-8777-46fc-a648-07fb19e46745",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// INV 247 — Frenzied Tilling
pub(in crate::card::sets) static FRENZIED_TILLING: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Frenzied Tilling",
    "15875876-3341-40fb-866f-5587c3638538",
    "Mike Raabe",
    CardRules::new_sorcery(mana_cost!("{3}{R}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            // Tapped, so the land it fetches does not pay for anything this turn --
            // which is the whole reason a five-mana Stone Rain is playable.
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ]),
    )),
);

// INV 248 — Galina's Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALINA_S_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Galina's Knight",
    "11b492d6-5e28-4f4b-942c-080d03cb0e92",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 249 — Hanna, Ship's Navigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HANNA_SHIP_S_NAVIGATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Hanna, Ship's Navigator",
    "83a4e48d-6452-4245-bdad-63fe3263550e",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 250 — Heroes' Reunion
pub(in crate::card::sets) static HEROES_REUNION: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Heroes' Reunion",
    "135d6043-5ec1-4ad4-8296-41fe23f11cb9",
    "Terese Nielsen",
    CardRules::new_instant(mana_cost!("{G}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains 7 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(7),
        },
    )),
);

// INV 251 — Horned Cheetah
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORNED_CHEETAH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Horned Cheetah",
    "a28ad983-ce91-40b6-a1ce-fe36ec7fbce8",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// INV 252 — Hunting Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Hunting Kavu",
    "8943304a-89c9-48b0-97b4-3e1aa690ca4d",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 253 — Kangee, Aerie Keeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KANGEE_AERIE_KEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Kangee, Aerie Keeper",
    "3afd7e8e-4fcc-4003-9791-7baf10ef1880",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// INV 254 — Llanowar Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Llanowar Knight",
    "e6c75d89-e432-49aa-a407-555b223b7eff",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 255 — Lobotomy (reprint)
const LOBOTOMY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::LOBOTOMY,
    "ff307dbb-4ab6-457b-be56-47106864bf61",
    "D. Alexander Gregory",
);

// INV 256 — Meteor Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Meteor Storm",
    "36489b24-f8a8-46b6-b879-0a5ce400a6dc",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// INV 257 — Noble Panther
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_PANTHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Noble Panther",
    "3f327818-8222-4295-8cef-118757b34d17",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 258 — Ordered Migration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDERED_MIGRATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Ordered Migration",
    "04d83a07-6054-45f1-bdf9-07f2006238d2",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 259 — Overabundance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERABUNDANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Overabundance",
    "4183e73d-609a-4292-b173-e39eb51949f3",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// INV 260 — Plague Spores
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_SPORES: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Plague Spores",
    "0d106d56-a688-49cc-8d5d-0279a5a7c0a7",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 261 — Pyre Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYRE_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Pyre Zombie",
    "6c030108-2995-4fb0-9b80-efdfdd0f11e0",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// INV 262 — Raging Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Raging Kavu",
    "27573679-e9e5-4bfc-b5d5-85d4648b01b6",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 263 — Reckless Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_ASSAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Reckless Assault",
    "ff0f568e-4d3a-40a5-b72a-63040ec5402d",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// INV 264 — Recoil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECOIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Recoil",
    "b6a77be3-e3b0-40f5-a470-414bac49da60",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 265 — Reviving Vapors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVIVING_VAPORS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Reviving Vapors",
    "47a23c32-e122-400b-b252-e636ea2e684b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 266 — Riptide Crab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_CRAB: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Riptide Crab",
    "7e42ae1d-62b4-4b19-aafc-f12bdd6fb8cc",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 267 — Rith, the Awakener
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_THE_AWAKENER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rith, the Awakener",
    "c30be387-280d-49bd-a3d1-c1636ee931ce",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// INV 268 — Sabertooth Nishoba
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SABERTOOTH_NISHOBA: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sabertooth Nishoba",
    "8338c296-cf3f-41d7-b380-3fb4237cb41c",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// INV 269 — Samite Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_ARCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Samite Archer",
    "07a262d7-6d0c-43d0-89b6-9f46a1a9eb69",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 270 — Seer's Vision
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEER_S_VISION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Seer's Vision",
    "0c94618a-808c-4b3c-8f34-45e64d0414d3",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 271 — Shivan Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Shivan Zombie",
    "f4c99269-f730-4d33-bbce-9e855e9ad0fc",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// INV 272 — Simoon (reprint)
const SIMOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::SIMOON,
    "84b1930d-2e4b-472f-98a9-008fd632f3be",
    "Tony Szczudlo",
);

// INV 273 — Sleeper's Robe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLEEPER_S_ROBE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sleeper's Robe",
    "3411f0fd-8b85-4d0d-a202-701a24ffac9f",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 274 — Slinking Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLINKING_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Slinking Serpent",
    "070a7004-5a28-4ccb-8640-ad6b07b51ece",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// INV 275 — Smoldering Tar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMOLDERING_TAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Smoldering Tar",
    "fcdc55c0-c8ac-49d5-969b-9bf0ee8e696c",
    "David Day",
    crate::card::CardRules::unsupported(),
);

// INV 276 — Spinal Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINAL_EMBRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Spinal Embrace",
    "692ad1eb-62a3-4560-bf8e-35f7db73c7a3",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// INV 277 — Stalking Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALKING_ASSASSIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Stalking Assassin",
    "ff8cc71f-3070-497f-908f-35aa13a8a857",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// INV 278 — Sterling Grove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STERLING_GROVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sterling Grove",
    "40b26aa3-8169-4978-9554-bd2fc8e18e3b",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// INV 279 — Teferi's Moat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_MOAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Teferi's Moat",
    "9ed5845c-ef6d-4a7b-b725-b09d3e9bbc17",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// INV 280 — Treva, the Renewer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREVA_THE_RENEWER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Treva, the Renewer",
    "4ee67039-6cee-4a2d-b973-570f5060f550",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// INV 281 — Tsabo Tavoc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_TAVOC: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tsabo Tavoc",
    "ccbe2539-7a7c-468b-a270-7ca1bdcccb1e",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// INV 282 — Undermine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERMINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Undermine",
    "2334bc71-5f85-47ff-b393-601a1e746a4e",
    "Massimiliano Frezzato",
    crate::card::CardRules::unsupported(),
);

// INV 283 — Urborg Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urborg Drake",
    "97d1327e-bf87-423f-8a04-8124e45b9ae0",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// INV 284 — Vicious Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VICIOUS_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Vicious Kavu",
    "31e9e629-7c25-4d45-aa35-9ba5f95b43cb",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 285 — Vile Consumption
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILE_CONSUMPTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Vile Consumption",
    "7f7e5716-77f3-45d2-a40a-f5bf500f6ad7",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 286 — Vodalian Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Vodalian Zombie",
    "f30a5a06-32ce-4d71-b71f-e3e1d8d4511a",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 287 — Void
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOID: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Void",
    "62dc1df7-b9db-4f5f-a340-08287cd3d9e5",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 288 — Voracious Cobra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VORACIOUS_COBRA: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Voracious Cobra",
    "9d8c5669-11a9-4d95-8431-7065037f1fb6",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// INV 289 — Wings of Hope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINGS_OF_HOPE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Wings of Hope",
    "be0d2402-f1ef-4a71-ac01-c7099c4ce54c",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// INV 290 — Yavimaya Barbarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_BARBARIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Yavimaya Barbarian",
    "8e17377d-4dad-4144-b0ce-c849636096a2",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// INV 291 — Yavimaya Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_KAVU: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Yavimaya Kavu",
    "1872f104-7cf1-41e3-b1b4-ca75c678e08b",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 292 — Stand // Deliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAND_DELIVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Stand // Deliver",
    "be8b338f-6f05-43c6-beeb-c5052cc0d6a9",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 293 — Spite // Malice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITE_MALICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Spite // Malice",
    "054f1845-196f-41c1-9682-042171cccd49",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 294 — Pain // Suffering
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAIN_SUFFERING: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Pain // Suffering",
    "81be27d6-e16f-4158-b2b6-66a0f3315327",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 295 — Assault // Battery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASSAULT_BATTERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Assault // Battery",
    "0ec6a889-c941-4898-a2f6-4d3863faf535",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// INV 296 — Wax // Wane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAX_WANE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Wax // Wane",
    "19859061-f5ec-4b7f-86a1-196f98648e0a",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// INV 297 — Alloy Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALLOY_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Alloy Golem",
    "1fb6d6a1-9d71-405b-9c93-1a7f06c67abd",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 298 — Bloodstone Cameo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODSTONE_CAMEO: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Bloodstone Cameo",
    "f9db32fa-64b2-4ef6-88f2-28e758d420bb",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// INV 299 — Chromatic Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROMATIC_SPHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Chromatic Sphere",
    "920cd17f-9274-443e-906f-c9904f0658d5",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// INV 300 — Crosis's Attendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_S_ATTENDANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Crosis's Attendant",
    "45edc18c-2046-4d0e-92fe-a6cf4aaf1c6f",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 301 — Darigaaz's Attendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_S_ATTENDANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Darigaaz's Attendant",
    "6f22b575-443a-4c06-8e75-d4140cbd3660",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// INV 302 — Drake-Skull Cameo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAKE_SKULL_CAMEO: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Drake-Skull Cameo",
    "4a3ce135-9c2f-45bd-b2db-c0e00c50c964",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// INV 303 — Dromar's Attendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_S_ATTENDANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Dromar's Attendant",
    "24936fa9-41a3-4da5-91cf-c28fa45f47c9",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// INV 304 — Juntu Stakes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNTU_STAKES: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Juntu Stakes",
    "3ab7cf53-f62d-47e1-af70-ab12be0d22e2",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// INV 305 — Lotus Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOTUS_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Lotus Guardian",
    "ddfc6396-5377-4ab3-9c10-8abcdeae2aa1",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// INV 306 — Phyrexian Altar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_ALTAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Phyrexian Altar",
    "25158cd5-749b-408c-9ab1-0f83e38730f7",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// INV 307 — Phyrexian Lens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_LENS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Phyrexian Lens",
    "6ec9a91d-7af0-44a8-839f-fb9960be0ddd",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 308 — Planar Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_PORTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Planar Portal",
    "24315eaa-ef55-4fd6-9145-e75b3de6f492",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// INV 309 — Power Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POWER_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Power Armor",
    "ed1981dd-c0f3-4e9d-a1f1-8bea823326ef",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// INV 310 — Rith's Attendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_S_ATTENDANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Rith's Attendant",
    "a26e8130-7fe9-4ef4-98af-928814f5b130",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 311 — Seashell Cameo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASHELL_CAMEO: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Seashell Cameo",
    "9efdbcad-e2e4-4f54-ade5-920b1853109e",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// INV 312 — Sparring Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPARRING_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sparring Golem",
    "d829d9de-83fa-4feb-8efc-0075315163c6",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 313 — Tek
pub(in crate::card::sets) static TEK: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tek",
    "c1f38104-a699-4bb9-930a-699f7bbc338a",
    "Chippy",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Dragon"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature gets +0/+2 as long as you control a Plains, has flying as long as you \
             control an Island, gets +2/+0 as long as you control a Swamp, has first strike as \
             long as you control a Mountain, and has trample as long as you control a Forest.",
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::controls_basic_land_type(
                        PlayerRelation::You,
                        BasicLandType::Plains,
                    ),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(2),
                        ),
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::controls_basic_land_type(
                        PlayerRelation::You,
                        BasicLandType::Island,
                    ),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::controls_basic_land_type(
                        PlayerRelation::You,
                        BasicLandType::Swamp,
                    ),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::controls_basic_land_type(
                        PlayerRelation::You,
                        BasicLandType::Mountain,
                    ),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::controls_basic_land_type(
                        PlayerRelation::You,
                        BasicLandType::Forest,
                    ),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    },
                },
            ]),
        ),
    ),
);

// INV 314 — Tigereye Cameo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIGEREYE_CAMEO: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tigereye Cameo",
    "25976da8-338d-4f46-b8ea-78a0aa3daa35",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// INV 315 — Treva's Attendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREVA_S_ATTENDANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Treva's Attendant",
    "9857af81-fb95-4dc4-b048-9ce4e96d1eca",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 316 — Troll-Horn Cameo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROLL_HORN_CAMEO: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Troll-Horn Cameo",
    "42b1ca6c-6ca0-4b02-885a-58cee3fa2aa8",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// INV 317 — Tsabo's Web
pub(in crate::card::sets) static TSABOS_WEB: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Tsabo's Web",
    "0dee69f8-cceb-41b9-a0ee-6b2ac9f4bad9",
    "Carl Critchlow",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_trigger("When this artifact enters, draw a card.", EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            }),
        AbilityDef::static_ability(
            "Each land with an activated ability that isn't a mana ability doesn't untap during its controller's untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasNonManaActivatedAbility,
                    ]), &[ZoneKind::Battlefield], PlayerRelation::Any),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
    ]),
);

// INV 318 — Urza's Filter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_FILTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urza's Filter",
    "680c75b1-e766-40be-84d7-2332047bb3de",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 319 — Ancient Spring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_SPRING: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Ancient Spring",
    "004eefa4-947b-45fc-b45c-5263bfd763bc",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// INV 320 — Archaeological Dig
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHAEOLOGICAL_DIG: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Archaeological Dig",
    "35f55af0-5a46-4900-b3d0-ca796b710e07",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// INV 321 — Coastal Tower
pub(in crate::card::sets) static COASTAL_TOWER: CardRecord = CardRecord::new(
    CardSet::Invasion,
    "Coastal Tower",
    "d115dbff-e35b-495f-a1e3-19651895927e",
    "Don Hazeltine",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// INV 322 — Elfhame Palace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELFHAME_PALACE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Elfhame Palace",
    "65986555-a5d7-497e-876f-b8d967d6aa5b",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// INV 323 — Geothermal Crevice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEOTHERMAL_CREVICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Geothermal Crevice",
    "e744b593-13fe-4967-b492-ac02f5815e57",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// INV 324 — Irrigation Ditch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRRIGATION_DITCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Irrigation Ditch",
    "977f1b44-166c-4faf-8a7b-d431707e90ce",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// INV 325 — Keldon Necropolis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_NECROPOLIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Keldon Necropolis",
    "4f0cccf6-b79b-4fff-89aa-801341598532",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// INV 326 — Salt Marsh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SALT_MARSH: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Salt Marsh",
    "ed64934b-0e64-4b2f-97aa-c3fb7e6ce0b0",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// INV 327 — Shivan Oasis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_OASIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Shivan Oasis",
    "9841f7e8-162c-44a3-96f3-af944fce15d1",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// INV 328 — Sulfur Vent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULFUR_VENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Sulfur Vent",
    "22c66ed6-55fb-4c65-aac4-26d9cc3053b8",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// INV 329 — Tinder Farm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TINDER_FARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Tinder Farm",
    "989b5901-aeb0-4a48-8c53-3b0ec0e0deba",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// INV 330 — Urborg Volcano
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_VOLCANO: CardRecord = CardRecord::new(
    crate::card::CardSet::Invasion,
    "Urborg Volcano",
    "c76f346c-ae34-4f5f-8e3b-6c77b0c4d530",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// INV 331 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "5ba9ef2e-d3ec-41f7-802e-e1414f14dd10",
    "John Avon",
);

// INV 332 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "bc73d7ff-bbef-4df9-ae7f-aa2ac8ac7025",
    "Ben Thompson",
);

// INV 333 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "87a66868-2efa-4985-b4fc-405d7fa8d410",
    "D. J. Cleland-Hura",
);

// INV 334 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "b5b4963b-c706-439f-9800-ff5d70003dcf",
    "Scott Bailey",
);

// INV 335 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "8a3fc29c-f5cb-49b9-aabf-a5fef97e7a7e",
    "Tony Szczudlo",
);

// INV 336 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "2fc04e1e-6a14-41cc-9fff-6dcd92cc6a3b",
    "John Avon",
);

// INV 337 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "f849f726-c6a2-400d-9b90-fe050f8ef5eb",
    "Terese Nielsen",
);

// INV 338 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "d07d1982-56ff-47ef-87aa-62978f1fcf30",
    "Darrell Riche",
);

// INV 339 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "2a7ce037-e04d-404a-afde-9122518e6a31",
    "Ron Spencer",
);

// INV 340 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "7cdb8b9d-2573-4162-9255-50a281dfb775",
    "Rob Alexander",
);

// INV 341 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "99b2bba7-8889-460c-a18f-fcd1e350ef4e",
    "Rob Alexander",
);

// INV 342 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "31a756b0-f430-4286-afe1-97c641e4f3b4",
    "Ron Spencer",
);

// INV 343 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "ba6694bb-f3b7-48ff-9d93-cbed84fac210",
    "Matt Cavotta",
);

// INV 344 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "977527da-2953-493f-8e8c-ffc64ddeaf10",
    "Jeff Miracola",
);

// INV 345 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "68df89dc-3909-4051-adc1-a86589d0e99d",
    "Glen Angus",
);

// INV 346 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "7e8ae541-98e2-4a84-90a6-b17502f4442d",
    "Scott Bailey",
);

// INV 347 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "f82d0a1c-5812-4254-a000-e4ff9aece3d9",
    "John Avon",
);

// INV 348 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "24788990-42ff-4b2b-8d01-fa2d0ec66a03",
    "Alan Pollack",
);

// INV 349 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "0b741c86-a563-4180-a857-7850de6ee366",
    "Alan Pollack",
);

// INV 350 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "cfacc498-f089-4ae4-8ce8-697cc671f445",
    "Glen Angus",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALABASTER_LEECH,
    &ARDENT_SOLDIER,
    &ATALYA_SAMITE_MASTER,
    &BENALISH_EMISSARY,
    &BENALISH_HERALDS,
    &BENALISH_LANCER,
    &BENALISH_TRAPPER,
    &CAPASHEN_UNICORN,
    &CRIMSON_ACOLYTE,
    &CRUSADING_KNIGHT,
    &DEATH_OR_GLORY,
    &DISMANTLING_BLOW,
    &DIVINE_PRESENCE,
    &FIGHT_OR_FLIGHT,
    &GLIMMERING_ANGEL,
    &GLOBAL_RUIN,
    &HARSH_JUDGMENT,
    &LIBERATE,
    &OBSIDIAN_ACOLYTE,
    &ORIM_S_TOUCH,
    &PLEDGE_OF_LOYALTY,
    &PRISON_BARRICADE,
    &PROTECTIVE_SPHERE,
    &PURE_REFLECTION,
    &RAMPANT_ELEPHANT,
    &RAZORFOOT_GRIFFIN,
    &RESTRAIN,
    &REVIVING_DOSE,
    &REWARDS_OF_DIVERSITY,
    &REYA_DAWNBRINGER,
    &ROUT,
    &RUHAM_DJINN,
    &SAMITE_MINISTRATION,
    &SPIRIT_OF_RESISTANCE,
    &SPIRIT_WEAVER,
    &STRENGTH_OF_UNITY,
    &SUNSCAPE_APPRENTICE,
    &SUNSCAPE_MASTER,
    &TEFERI_S_CARE,
    &WAYFARING_GIANT,
    &WINNOW,
    &BARRIN_S_UNMAKING,
    &BLIND_SEER,
    &BREAKING_WAVE,
    &COLLECTIVE_RESTRAINT,
    &CRYSTAL_SPRAY,
    &DISTORTING_WAKE,
    &DREAM_THRUSH,
    &EMPRESS_GALINA,
    &ESSENCE_LEAK,
    &EXCLUDE,
    &FACT_OR_FICTION,
    &FAERIE_SQUADRON,
    &MANA_MAZE,
    &MANIPULATE_FATE,
    &METATHRAN_AEROSTAT,
    &METATHRAN_TRANSPORT,
    &METATHRAN_ZOMBIE,
    &OPT,
    &PROBE,
    &PROHIBIT,
    &PSYCHIC_BATTLE,
    &RAINBOW_CROW,
    &REPULSE,
    &SAPPHIRE_LEECH,
    &SHORELINE_RAIDER,
    &SKY_WEAVER,
    &STORMSCAPE_APPRENTICE,
    &STORMSCAPE_MASTER,
    &SWAY_OF_ILLUSION,
    &TEFERIS_RESPONSE,
    &TEMPORAL_DISTORTION,
    &TIDAL_VISIONARY,
    &TOLARIAN_EMISSARY,
    &TOWER_DRAKE,
    &TRAVELER_S_CLOAK,
    &VODALIAN_HYPNOTIST,
    &VODALIAN_MERCHANT,
    &VODALIAN_SERPENT,
    &WASH_OUT,
    &WELL_LAID_PLANS,
    &WORLDLY_COUNSEL,
    &ZANAM_DJINN,
    &ADDLE,
    &AGONIZING_DEMISE,
    &ANDRADITE_LEECH,
    &ANNIHILATE,
    &BOG_INITIATE,
    &CREMATE,
    &CRYPT_ANGEL,
    &DEFILING_TEARS,
    &DESPERATE_RESEARCH,
    &DEVOURING_STROSSUS,
    &DO_OR_DIE,
    &DREDGE,
    &DUSKWALKER,
    &EXOTIC_CURSE,
    &FIRESCREAMER,
    &GOHAM_DJINN,
    &HATE_WEAVER,
    &HYPNOTIC_CLOUD,
    &MARAUDING_KNIGHT,
    &MOURNING,
    &NIGHTSCAPE_APPRENTICE,
    &NIGHTSCAPE_MASTER,
    &PHYREXIAN_BATTLEFLIES,
    &PHYREXIAN_DELVER,
    &PHYREXIAN_INFILTRATOR,
    &PHYREXIAN_REAPER,
    &PHYREXIAN_SLAYER,
    &PLAGUE_SPITTER,
    &RECOVER,
    &SCAVENGED_WEAPONRY,
    &SPREADING_PLAGUE,
    &TAINTED_WELL,
    &TRENCH_WURM,
    &TSABO_S_ASSASSIN,
    &TSABO_S_DECREE,
    &TWILIGHT_S_CALL,
    &URBORG_EMISSARY,
    &URBORG_PHANTOM,
    &URBORG_SHAMBLER,
    &URBORG_SKELETON,
    &YAWGMOTH_S_AGENDA,
    &ANCIENT_KAVU,
    &BEND_OR_BREAK,
    &BREATH_OF_DARIGAAZ,
    &CALLOUS_GIANT,
    &CHAOTIC_STRIKE,
    &COLLAPSING_BORDERS,
    &FIREBRAND_RANGER,
    &GHITU_FIRE,
    &GOBLIN_SPY,
    &HALAM_DJINN,
    &HOODED_KAVU,
    &KAVU_AGGRESSOR,
    &KAVU_MONARCH,
    &KAVU_RUNNER,
    &KAVU_SCOUT,
    &LIGHTNING_DART,
    &LOAFING_GIANT,
    &MAGES_CONTEST,
    &OBLITERATE,
    &OVERLOAD,
    &POUNCING_KAVU,
    &RAGE_WEAVER,
    &ROGUE_KAVU,
    &RUBY_LEECH,
    &SAVAGE_OFFENSIVE,
    &SCARRED_PUMA,
    &SCORCHING_LAVA,
    &SEARING_RAYS,
    &SHIVAN_EMISSARY,
    &SHIVAN_HARVEST,
    &SKITTISH_KAVU,
    &SKIZZIK,
    &SLIMY_KAVU,
    &STAND_OR_FALL,
    &TECTONIC_INSTABILITY,
    &THUNDERSCAPE_APPRENTICE,
    &THUNDERSCAPE_MASTER,
    &TRIBAL_FLAMES,
    &TURF_WOUND,
    &URZA_S_RAGE,
    &VIASHINO_GRAPPLER,
    &ZAP,
    &AGGRESSIVE_URGE,
    &BIND,
    &BLURRED_MONGOOSE,
    &CANOPY_SURGE,
    &ELFHAME_SANCTUARY,
    &ELVISH_CHAMPION,
    &EXPLOSIVE_GROWTH,
    &JADE_LEECH,
    &KAVU_CHAMELEON,
    &KAVU_CLIMBER,
    &KAVU_LAIR,
    &KAVU_TITAN,
    &LLANOWAR_CAVALRY,
    &LLANOWAR_ELITE,
    &LLANOWAR_VANGUARD,
    &MIGHT_WEAVER,
    &MOLIMO_MARO_SORCERER,
    &NOMADIC_ELF,
    &PINCER_SPIDER,
    &PULSE_OF_LLANOWAR,
    &QUIRION_SENTINEL,
    &QUIRION_TRAILBLAZER,
    &RESTOCK,
    &ROOTING_KAVU,
    &SAPROLING_INFESTATION,
    &SAPROLING_SYMBIOSIS,
    &SCOUTING_TREK,
    &SERPENTINE_KAVU,
    &SULAM_DJINN,
    &TANGLE,
    &THICKET_ELEMENTAL,
    &THORNSCAPE_APPRENTICE,
    &THORNSCAPE_MASTER,
    &TREEFOLK_HEALER,
    &UTOPIA_TREE,
    &VERDELOTH_THE_ANCIENT,
    &VERDURAN_EMISSARY,
    &VIGOROUS_CHARGE,
    &WALLOP,
    &WANDERING_STREAM,
    &WHIP_SILK,
    &ABSORB,
    &AETHER_RIFT,
    &ANGELIC_SHIELD,
    &ARMADILLO_CLOAK,
    &ARMORED_GUARDIAN,
    &ARTIFACT_MUTATION,
    &AURA_MUTATION,
    &AURA_SHARDS,
    &BACKLASH,
    &BARRIN_S_SPITE,
    &BLAZING_SPECTER,
    &CAPTAIN_SISAY,
    &CAULDRON_DANCE,
    &CHARGING_TROLL,
    &CINDER_SHADE,
    &COALITION_VICTORY,
    &CROSIS_THE_PURGER,
    &DARIGAAZ_THE_IGNITER,
    &DROMAR_THE_BANISHER,
    &DUELING_GROUNDS,
    &FIRES_OF_YAVIMAYA,
    &FRENZIED_TILLING,
    &GALINA_S_KNIGHT,
    &HANNA_SHIP_S_NAVIGATOR,
    &HEROES_REUNION,
    &HORNED_CHEETAH,
    &HUNTING_KAVU,
    &KANGEE_AERIE_KEEPER,
    &LLANOWAR_KNIGHT,
    &METEOR_STORM,
    &NOBLE_PANTHER,
    &ORDERED_MIGRATION,
    &OVERABUNDANCE,
    &PLAGUE_SPORES,
    &PYRE_ZOMBIE,
    &RAGING_KAVU,
    &RECKLESS_ASSAULT,
    &RECOIL,
    &REVIVING_VAPORS,
    &RIPTIDE_CRAB,
    &RITH_THE_AWAKENER,
    &SABERTOOTH_NISHOBA,
    &SAMITE_ARCHER,
    &SEER_S_VISION,
    &SHIVAN_ZOMBIE,
    &SLEEPER_S_ROBE,
    &SLINKING_SERPENT,
    &SMOLDERING_TAR,
    &SPINAL_EMBRACE,
    &STALKING_ASSASSIN,
    &STERLING_GROVE,
    &TEFERI_S_MOAT,
    &TREVA_THE_RENEWER,
    &TSABO_TAVOC,
    &UNDERMINE,
    &URBORG_DRAKE,
    &VICIOUS_KAVU,
    &VILE_CONSUMPTION,
    &VODALIAN_ZOMBIE,
    &VOID,
    &VORACIOUS_COBRA,
    &WINGS_OF_HOPE,
    &YAVIMAYA_BARBARIAN,
    &YAVIMAYA_KAVU,
    &STAND_DELIVER,
    &SPITE_MALICE,
    &PAIN_SUFFERING,
    &ASSAULT_BATTERY,
    &WAX_WANE,
    &ALLOY_GOLEM,
    &BLOODSTONE_CAMEO,
    &CHROMATIC_SPHERE,
    &CROSIS_S_ATTENDANT,
    &DARIGAAZ_S_ATTENDANT,
    &DRAKE_SKULL_CAMEO,
    &DROMAR_S_ATTENDANT,
    &JUNTU_STAKES,
    &LOTUS_GUARDIAN,
    &PHYREXIAN_ALTAR,
    &PHYREXIAN_LENS,
    &PLANAR_PORTAL,
    &POWER_ARMOR,
    &RITH_S_ATTENDANT,
    &SEASHELL_CAMEO,
    &SPARRING_GOLEM,
    &TEK,
    &TIGEREYE_CAMEO,
    &TREVA_S_ATTENDANT,
    &TROLL_HORN_CAMEO,
    &TSABOS_WEB,
    &URZA_S_FILTER,
    &ANCIENT_SPRING,
    &ARCHAEOLOGICAL_DIG,
    &COASTAL_TOWER,
    &ELFHAME_PALACE,
    &GEOTHERMAL_CREVICE,
    &IRRIGATION_DITCH,
    &KELDON_NECROPOLIS,
    &SALT_MARSH,
    &SHIVAN_OASIS,
    &SULFUR_VENT,
    &TINDER_FARM,
    &URBORG_VOLCANO,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANGEL_OF_MERCY_REPRINT,
    BLINDING_LIGHT_REPRINT,
    HOLY_DAY_REPRINT,
    SHACKLES_REPRINT,
    DISRUPT_REPRINT,
    PHANTASMAL_TERRAIN_REPRINT,
    SHIMMERING_WINGS_REPRINT,
    CURSED_FLESH_REPRINT,
    RAVENOUS_RATS_REPRINT,
    RECKLESS_SPITE_REPRINT,
    SOUL_BURN_REPRINT,
    SOUL_BURN_ALTERNATE_1,
    SOUL_BURN_ALTERNATE_2,
    URBORG_SKELETON_ALTERNATE_1,
    URBORG_SKELETON_ALTERNATE_2,
    CROWN_OF_FLAMES_REPRINT,
    MANIACAL_RAGE_REPRINT,
    STUN_REPRINT,
    FERTILE_GROUND_REPRINT,
    HARROW_REPRINT,
    QUIRION_ELVES_REPRINT,
    TRANQUILITY_REPRINT,
    LOBOTOMY_REPRINT,
    SIMOON_REPRINT,
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
