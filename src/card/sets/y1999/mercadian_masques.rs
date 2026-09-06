//! Mercadian Masques cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1996::alliances as catalog_all;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::tempest as catalog_tmp;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::gatecrash as catalog_gtc;
use crate::card::{
    AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardNameDef, CardNameSetDef, CardRules, CardSet,
    CardSupertype, CardType, ComparisonDef, CostDef, CounterKind, EffectDef, EffectRecipientDef,
    KeywordAbility, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayActionMatcherDef, PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{AdditionalCostObjectIndex, TargetIndex, mana_cost};

// MMQ 1 — Afterlife (reprint)

// MMQ 2 — Alabaster Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cf393a3-831e-4d3a-8404-ee83f60970aa"),
    "Alabaster Wall",
    crate::card::CardArt::new("9cf393a3-831e-4d3a-8404-ee83f60970aa", "Randy Gallegos"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 3 — Armistice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMISTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1eb4402a-f263-4f82-b4c0-cf0aa58dc946"),
    "Armistice",
    crate::card::CardArt::new("1eb4402a-f263-4f82-b4c0-cf0aa58dc946", "Dan Frazier"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 4 — Arrest
pub(in crate::card::sets) static ARREST: CardRecord = CardRecord::new_with_legacy_id(
    1952,
    "Arrest",
    CardArt::new("3b083fd8-6422-4cd3-a27d-41b6d88598c2", "Dan Frazier"),
    CardSet::MercadianMasques,
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
                        AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Any),
                    ]),
                },
            ),
        ]),
);

// MMQ 5 — Ballista Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALLISTA_SQUAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30d51d84-23d2-41ff-ab68-a633beddba06"),
    "Ballista Squad",
    crate::card::CardArt::new("30d51d84-23d2-41ff-ab68-a633beddba06", "Matthew D. Wilson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 6 — Charm Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARM_PEDDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("082e6ee3-cc1f-46c7-9d82-56751478b3cf"),
    "Charm Peddler",
    crate::card::CardArt::new("082e6ee3-cc1f-46c7-9d82-56751478b3cf", "John Matson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 7 — Charmed Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARMED_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66d36960-3c78-4032-9325-8002b2a48503"),
    "Charmed Griffin",
    crate::card::CardArt::new("66d36960-3c78-4032-9325-8002b2a48503", "Ray Lago"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 8 — Cho-Arrim Alchemist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_ARRIM_ALCHEMIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42c9d49d-61eb-4f33-b06b-03bdd990efd0"),
    "Cho-Arrim Alchemist",
    crate::card::CardArt::new("42c9d49d-61eb-4f33-b06b-03bdd990efd0", "Scott M. Fischer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 9 — Cho-Arrim Bruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_ARRIM_BRUISER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26e98f06-ad8d-4a93-8ae6-3da42b63b5b5"),
    "Cho-Arrim Bruiser",
    crate::card::CardArt::new("26e98f06-ad8d-4a93-8ae6-3da42b63b5b5", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 10 — Cho-Arrim Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_ARRIM_LEGATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1427a3a1-24e1-4697-b5eb-1c0a24f89e75"),
    "Cho-Arrim Legate",
    crate::card::CardArt::new("1427a3a1-24e1-4697-b5eb-1c0a24f89e75", "rk post"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 11 — Cho-Manno, Revolutionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_MANNO_REVOLUTIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3dc51393-de63-4ce3-ab02-c695e4448018"),
    "Cho-Manno, Revolutionary",
    crate::card::CardArt::new(
        "3dc51393-de63-4ce3-ab02-c695e4448018",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 12 — Cho-Manno's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHO_MANNO_S_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c9f33c6-5294-4584-854d-c8c0f847aba8"),
    "Cho-Manno's Blessing",
    crate::card::CardArt::new("5c9f33c6-5294-4584-854d-c8c0f847aba8", "John Matson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 13 — Common Cause
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMON_CAUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eae4c25f-2005-4ac0-a5f0-2fc250520995"),
    "Common Cause",
    crate::card::CardArt::new("eae4c25f-2005-4ac0-a5f0-2fc250520995", "John Matson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 14 — Cornered Market
pub(in crate::card::sets) static CORNERED_MARKET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d4f3c1d-d25e-4263-ab2b-19534c852678"),
    "Cornered Market",
    crate::card::CardArt::new(
        "0d4f3c1d-d25e-4263-ab2b-19534c852678",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::MercadianMasques,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(AbilityDef::static_ability(
        "Players can't cast spells with the same name as a nontoken permanent. Players can't play nonbasic lands with the same name as a nontoken permanent.",
        EffectDef::Sequence(&[
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::NameIn(&CardNameSetDef::NamesOf(
                            &ObjectSetDef::Query(ObjectQueryDef::new(
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                                &[ZoneKind::Battlefield],
                            )),
                        )),
                    ),
                )),
            },
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::PlayLand,
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                            ObjectPredicateDef::NameIn(&CardNameSetDef::NamesOf(
                                &ObjectSetDef::Query(ObjectQueryDef::new(
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                                    &[ZoneKind::Battlefield],
                                )),
                            )),
                        ]),
                    ),
                )),
            },
        ]),
    )),
);

// MMQ 15 — Crackdown
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRACKDOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7009fd8-1d80-41bb-a1b0-fea9c909c63d"),
    "Crackdown",
    crate::card::CardArt::new("a7009fd8-1d80-41bb-a1b0-fea9c909c63d", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 16 — Crossbow Infantry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSSBOW_INFANTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("744c2177-3140-48a1-95a4-2f0a27ca5b2f"),
    "Crossbow Infantry",
    crate::card::CardArt::new(
        "744c2177-3140-48a1-95a4-2f0a27ca5b2f",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 17 — Devout Witness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOUT_WITNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48ca7aeb-09db-4409-9ba2-c5c5500ad72f"),
    "Devout Witness",
    crate::card::CardArt::new("48ca7aeb-09db-4409-9ba2-c5c5500ad72f", "Don Hazeltine"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 18 — Disenchant (reprint)

// MMQ 19 — Fountain Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUNTAIN_WATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("690daa19-1842-4605-9bda-bf67e4ede3c4"),
    "Fountain Watch",
    crate::card::CardArt::new("690daa19-1842-4605-9bda-bf67e4ede3c4", "Jeff Miracola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 20 — Fresh Volunteers
pub(in crate::card::sets) static FRESH_VOLUNTEERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e070ea4a-c417-405f-b788-78fb7ca2eaa5"),
    "Fresh Volunteers",
    CardArt::new("e070ea4a-c417-405f-b788-78fb7ca2eaa5", "Jeff Miracola"),
    CardSet::MercadianMasques,
    // A vanilla 2/2 for two whose Rebel type is the whole point: it is what
    // the searchers in this block are looking for.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Rebel"], 2, 2),
);

// MMQ 21 — Honor the Fallen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONOR_THE_FALLEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70147617-10e0-413d-be0a-a888b9cb6b97"),
    "Honor the Fallen",
    crate::card::CardArt::new("70147617-10e0-413d-be0a-a888b9cb6b97", "Terese Nielsen"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 22 — Ignoble Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IGNOBLE_SOLDIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("676a2506-17f8-4b8e-be0c-eacc0fe972f6"),
    "Ignoble Soldier",
    crate::card::CardArt::new("676a2506-17f8-4b8e-be0c-eacc0fe972f6", "Mark Romanoski"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 23 — Inviolability
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVIOLABILITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ece8504-389a-43e3-b178-7067722c4b75"),
    "Inviolability",
    crate::card::CardArt::new("9ece8504-389a-43e3-b178-7067722c4b75", "DiTerlizzi"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 24 — Ivory Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IVORY_MASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35ea3762-a419-412c-b2bd-0a40902d8d51"),
    "Ivory Mask",
    crate::card::CardArt::new("35ea3762-a419-412c-b2bd-0a40902d8d51", "Glen Angus"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 25 — Jhovall Queen
pub(in crate::card::sets) static JHOVALL_QUEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8eb55cc-ddde-4f15-9262-b9aee28059d3"),
    "Jhovall Queen",
    CardArt::new("b8eb55cc-ddde-4f15-9262-b9aee28059d3", "Michael Sutfin"),
    CardSet::MercadianMasques,
    // Six mana for a wall that attacks. Vigilance on a 4/7 means it is doing
    // both jobs every turn rather than choosing.
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Cat", "Rebel"], 4, 7)
        .with_abilities(&[abilities::vigilance()]),
);

// MMQ 26 — Jhovall Rider
pub(in crate::card::sets) static JHOVALL_RIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e1f7c51-0011-4ea5-b123-3c26293f5dab"),
    "Jhovall Rider",
    CardArt::new("7e1f7c51-0011-4ea5-b123-3c26293f5dab", "Scott M. Fischer"),
    CardSet::MercadianMasques,
    // Five mana for a 3/3 trampler, which is what a Rebel chain paid for a
    // body it could fetch rather than draw.
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Rebel"], 3, 3)
        .with_abilities(&[abilities::trample()]),
);

// MMQ 27 — Last Breath
pub(in crate::card::sets) static LAST_BREATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b540da2-f8c6-48d6-af6d-db78958f0a17"),
    "Last Breath",
    CardArt::new("3b540da2-f8c6-48d6-af6d-db78958f0a17", "DiTerlizzi"),
    CardSet::MercadianMasques,
    // Two mana to exile a small creature, and four life is what white pays
    // for an answer that leaves nothing behind.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature with power 2 or less. Its controller gains 4 life.",
        // "Power 2 or less" is written as less-than-3: the predicate
        // vocabulary offers strict comparisons.
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            // Their life, not yours: the four is what buys an exile at two
            // mana, and it is paid whether or not the exile happened.
            EffectDef::GainLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// MMQ 28 — Moment of Silence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENT_OF_SILENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f50b5f5-8dac-4785-b1af-a0bd64ce7a92"),
    "Moment of Silence",
    crate::card::CardArt::new(
        "3f50b5f5-8dac-4785-b1af-a0bd64ce7a92",
        "Christopher Moeller",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 29 — Moonlit Wake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONLIT_WAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1eba9595-6789-4d7a-9e46-8d1f75993b21"),
    "Moonlit Wake",
    crate::card::CardArt::new(
        "1eba9595-6789-4d7a-9e46-8d1f75993b21",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 30 — Muzzle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUZZLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b3048ec-bcbf-4a69-b56f-83bbe82b68e5"),
    "Muzzle",
    crate::card::CardArt::new("8b3048ec-bcbf-4a69-b56f-83bbe82b68e5", "Matt Cavotta"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 31 — Nightwind Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTWIND_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0968401d-522f-4def-92a1-d504471ac54e"),
    "Nightwind Glider",
    crate::card::CardArt::new("0968401d-522f-4def-92a1-d504471ac54e", "Randy Gallegos"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 32 — Noble Purpose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_PURPOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad5ff149-8516-456e-af8a-3dea78715acb"),
    "Noble Purpose",
    crate::card::CardArt::new("ad5ff149-8516-456e-af8a-3dea78715acb", "Kev Walker"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 33 — Orim's Cure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_CURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("754ae359-363b-456a-bbca-52fbfbaa86b8"),
    "Orim's Cure",
    crate::card::CardArt::new("754ae359-363b-456a-bbca-52fbfbaa86b8", "Don Hazeltine"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 34 — Pious Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIOUS_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc20c1f0-9883-484c-88d8-1cab08d0b210"),
    "Pious Warrior",
    crate::card::CardArt::new("bc20c1f0-9883-484c-88d8-1cab08d0b210", "Jeff Miracola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 35 — Ramosian Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_CAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e9d2e2a-c608-4787-bbd9-e1871f681b58"),
    "Ramosian Captain",
    crate::card::CardArt::new("0e9d2e2a-c608-4787-bbd9-e1871f681b58", "Matthew D. Wilson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 36 — Ramosian Commander
pub(in crate::card::sets) static RAMOSIAN_COMMANDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("867f5d82-71c2-455f-ab16-5a32bba46986"),
    "Ramosian Commander",
    CardArt::new("867f5d82-71c2-455f-ab16-5a32bba46986", "Scott Hampton"),
    CardSet::MercadianMasques,
    // The top of the chain a deck actually assembles: by the time this is
    // searching, the mana is there and the library is the deck.
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Rebel"], 2, 4).with_ability(
        AbilityDef::activated(
            "{6}, {T}: Search your library for a Rebel permanent card with mana value 5 \
             or less, put it onto the battlefield, then shuffle.",
            &[CostDef::Mana(mana_cost!("{6}")), CostDef::TapSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                // "Rebel permanent card", so a Rebel instant would not
                // qualify even if one existed; the chain fetches bodies
                // and Equipment alike.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Rebel"),
                    ObjectPredicateDef::ManaValueAtMost(5),
                ]),
                // Failing to find is allowed, so the minimum is zero:
                // the mana and the tap are spent either way.
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// MMQ 37 — Ramosian Lieutenant
pub(in crate::card::sets) static RAMOSIAN_LIEUTENANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("debe840a-ebc9-43c4-9bf7-7eb292b65bf9"),
    "Ramosian Lieutenant",
    CardArt::new("debe840a-ebc9-43c4-9bf7-7eb292b65bf9", "Alan Pollack"),
    CardSet::MercadianMasques,
    // The middle link, fetched by the Sergeant and fetching the Commander.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Rebel"], 1, 2).with_ability(
        AbilityDef::activated(
            "{4}, {T}: Search your library for a Rebel permanent card with mana value 3 \
             or less, put it onto the battlefield, then shuffle.",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                // "Rebel permanent card", so a Rebel instant would not
                // qualify even if one existed; the chain fetches bodies
                // and Equipment alike.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Rebel"),
                    ObjectPredicateDef::ManaValueAtMost(3),
                ]),
                // Failing to find is allowed, so the minimum is zero:
                // the mana and the tap are spent either way.
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// MMQ 38 — Ramosian Rally
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_RALLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fc0ff04-43e7-4a0d-b7e2-8bab72cc6cc0"),
    "Ramosian Rally",
    crate::card::CardArt::new(
        "7fc0ff04-43e7-4a0d-b7e2-8bab72cc6cc0",
        "Christopher Moeller",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 39 — Ramosian Sergeant
pub(in crate::card::sets) static RAMOSIAN_SERGEANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef2b036d-5721-4a6e-bf43-69148b90da10"),
    "Ramosian Sergeant",
    CardArt::new("ef2b036d-5721-4a6e-bf43-69148b90da10", "Don Hazeltine"),
    CardSet::MercadianMasques,
    // The bottom of the Rebel chain and the card that starts it: one mana
    // for a body that turns spare mana into the next link.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Rebel"], 1, 1).with_ability(
        AbilityDef::activated(
            "{3}, {T}: Search your library for a Rebel permanent card with mana value 2 \
             or less, put it onto the battlefield, then shuffle.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                // "Rebel permanent card", so a Rebel instant would not
                // qualify even if one existed; the chain fetches bodies
                // and Equipment alike.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Rebel"),
                    ObjectPredicateDef::ManaValueAtMost(2),
                ]),
                // Failing to find is allowed, so the minimum is zero:
                // the mana and the tap are spent either way.
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// MMQ 40 — Ramosian Sky Marshal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMOSIAN_SKY_MARSHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16638976-8a78-4233-8ebc-42ea9bb49e0a"),
    "Ramosian Sky Marshal",
    crate::card::CardArt::new("16638976-8a78-4233-8ebc-42ea9bb49e0a", "Matt Cavotta"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 41 — Rappelling Scouts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPPELLING_SCOUTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("113b8366-e6d0-423e-af4b-52c1e08ed446"),
    "Rappelling Scouts",
    crate::card::CardArt::new("113b8366-e6d0-423e-af4b-52c1e08ed446", "Nelson DeCastro"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 42 — Renounce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENOUNCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8bb2bfb9-cc4a-4d33-99f9-17db4d9fc718"),
    "Renounce",
    crate::card::CardArt::new("8bb2bfb9-cc4a-4d33-99f9-17db4d9fc718", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 43 — Revered Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERED_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0793175-e56b-4ff8-9e22-3a96a698068c"),
    "Revered Elder",
    crate::card::CardArt::new("b0793175-e56b-4ff8-9e22-3a96a698068c", "Donato Giancola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 44 — Reverent Mantra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERENT_MANTRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48364e19-a3ea-4980-925f-7918e57315f1"),
    "Reverent Mantra",
    crate::card::CardArt::new("48364e19-a3ea-4980-925f-7918e57315f1", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 45 — Righteous Aura (reprint)

// MMQ 46 — Righteous Indignation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_INDIGNATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1fb6335-cfd8-438c-b936-09b850d61b28"),
    "Righteous Indignation",
    crate::card::CardArt::new("c1fb6335-cfd8-438c-b936-09b850d61b28", "Val Mayerik"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 47 — Security Detail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SECURITY_DETAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b89d34b-1a67-4f5c-a731-54b56c5233ff"),
    "Security Detail",
    crate::card::CardArt::new("5b89d34b-1a67-4f5c-a731-54b56c5233ff", "Val Mayerik"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 48 — Soothing Balm
pub(in crate::card::sets) static SOOTHING_BALM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("96b8f4be-9f4d-4373-8141-a03518ecd38a"),
    "Soothing Balm",
    CardArt::new("96b8f4be-9f4d-4373-8141-a03518ecd38a", "Scott M. Fischer"),
    CardSet::MercadianMasques,
    // Five life at instant speed for two mana, which is a burn spell's
    // worth of life and no board presence at all.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains 5 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(5),
        },
    )),
);

// MMQ 49 — Spiritual Focus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRITUAL_FOCUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8521ae08-eb46-45ff-8fc4-62d8b07cfac2"),
    "Spiritual Focus",
    crate::card::CardArt::new("8521ae08-eb46-45ff-8fc4-62d8b07cfac2", "Andrew Goldhawk"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 50 — Steadfast Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEADFAST_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6381774b-fb91-46cc-9bf6-6eeb4d67a165"),
    "Steadfast Guard",
    crate::card::CardArt::new("6381774b-fb91-46cc-9bf6-6eeb4d67a165", "Adam Rex"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 51 — Story Circle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORY_CIRCLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("675378bb-7dc1-4bc8-b026-27e6e8e72e18"),
    "Story Circle",
    crate::card::CardArt::new("675378bb-7dc1-4bc8-b026-27e6e8e72e18", "Bradley Williams"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 52 — Task Force
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TASK_FORCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17a58c5b-28c2-4261-992c-2ecadb721880"),
    "Task Force",
    crate::card::CardArt::new("17a58c5b-28c2-4261-992c-2ecadb721880", "Gary Ruddell"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 53 — Thermal Glider
pub(in crate::card::sets) static THERMAL_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd909c26-930d-4af0-b19a-c899847338b4"),
    "Thermal Glider",
    CardArt::new("fd909c26-930d-4af0-b19a-c899847338b4", "Mark Zug"),
    CardSet::MercadianMasques,
    // The Rebel chain's answer to red: a fetchable body that red removal
    // cannot touch.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Rebel"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// MMQ 54 — Tonic Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TONIC_PEDDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("334bbd9d-3549-4352-9635-d772aab28503"),
    "Tonic Peddler",
    crate::card::CardArt::new("334bbd9d-3549-4352-9635-d772aab28503", "Adam Rex"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 55 — Trap Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAP_RUNNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eba97681-1d1f-4ab6-a21b-fbbbe63a1c74"),
    "Trap Runner",
    crate::card::CardArt::new("eba97681-1d1f-4ab6-a21b-fbbbe63a1c74", "Ron Spencer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 56 — Wave of Reckoning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAVE_OF_RECKONING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b101b5e-d478-4686-b3cf-bdc545f089e5"),
    "Wave of Reckoning",
    crate::card::CardArt::new("0b101b5e-d478-4686-b3cf-bdc545f089e5", "Bradley Williams"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 57 — Wishmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WISHMONGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a0d8834-109e-4235-a145-75edc43da0ec"),
    "Wishmonger",
    crate::card::CardArt::new("5a0d8834-109e-4235-a145-75edc43da0ec", "Heather Hudson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 58 — Aerial Caravan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AERIAL_CARAVAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("adac91af-5165-4779-99f7-e75c83fa5d5d"),
    "Aerial Caravan",
    crate::card::CardArt::new("adac91af-5165-4779-99f7-e75c83fa5d5d", "DiTerlizzi"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 59 — Balloon Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALLOON_PEDDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c34963e6-850e-4ce4-b04f-5e623ce5b73f"),
    "Balloon Peddler",
    crate::card::CardArt::new("c34963e6-850e-4ce4-b04f-5e623ce5b73f", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 60 — Blockade Runner
pub(in crate::card::sets) static BLOCKADE_RUNNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59e483df-b58a-401e-85bc-0afda4bf7cac"),
    "Blockade Runner",
    CardArt::new("59e483df-b58a-401e-85bc-0afda4bf7cac", "Carl Critchlow"),
    CardSet::MercadianMasques,
    // One mana makes it unblockable, so the 2/2 body connects every turn
    // the mana is available.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Merfolk"], 2, 2).with_ability(
        AbilityDef::activated(
            "{U}: This creature can't be blocked this turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MMQ 61 — Brainstorm (reprint)

// MMQ 62 — Bribery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIBERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfc0ea8a-62f6-49e8-8eec-9748870bc596"),
    "Bribery",
    crate::card::CardArt::new("dfc0ea8a-62f6-49e8-8eec-9748870bc596", "Andrew Robinson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 63 — Buoyancy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUOYANCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b208dad2-a412-45fd-b19a-d370426ef5b8"),
    "Buoyancy",
    crate::card::CardArt::new("b208dad2-a412-45fd-b19a-d370426ef5b8", "Jeff Miracola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 64 — Chambered Nautilus
pub(in crate::card::sets) static CHAMBERED_NAUTILUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("860c613d-d031-4c2a-922b-39f4eec04e18"),
    "Chambered Nautilus",
    CardArt::new("860c613d-d031-4c2a-922b-39f4eec04e18", "John Matson"),
    CardSet::MercadianMasques,
    // A 2/2 that draws when they stop it, so blocking it is never free.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Nautilus", "Beast"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you may draw a card.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &const {
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    }
                },
            },
        ),
    ),
);

// MMQ 65 — Chameleon Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMELEON_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05972ea2-b0bc-40fd-bce4-07eebdb150d5"),
    "Chameleon Spirit",
    crate::card::CardArt::new("05972ea2-b0bc-40fd-bce4-07eebdb150d5", "Bradley Williams"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 66 — Charisma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARISMA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63565b03-28e9-4534-b085-d5803e2623bb"),
    "Charisma",
    crate::card::CardArt::new("63565b03-28e9-4534-b085-d5803e2623bb", "Terese Nielsen"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 67 — Cloud Sprite
pub(in crate::card::sets) static CLOUD_SPRITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d14352c-ac8c-45b5-b930-63822408ba3d"),
    "Cloud Sprite",
    CardArt::new("3d14352c-ac8c-45b5-b930-63822408ba3d", "Mark Zug"),
    CardSet::MercadianMasques,
    // One mana for a flier that answers other one-mana fliers and nothing
    // else at all.
    CardRules::new_creature(mana_cost!("{U}"), &["Faerie"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            // A restriction on the blocker rather than the attacker, so
            // it stops this creature from blocking on the ground without
            // saying anything about who may block it.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// MMQ 68 — Coastal Piracy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COASTAL_PIRACY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("179d1f76-6f4c-4a77-815a-aae7a933c9ad"),
    "Coastal Piracy",
    crate::card::CardArt::new("179d1f76-6f4c-4a77-815a-aae7a933c9ad", "Matthew D. Wilson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 69 — Counterspell (reprint)

// MMQ 70 — Cowardice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COWARDICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2e46d3d-7c7f-487f-8cc6-078b17c113a0"),
    "Cowardice",
    crate::card::CardArt::new("d2e46d3d-7c7f-487f-8cc6-078b17c113a0", "Scott M. Fischer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 71 — Customs Depot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUSTOMS_DEPOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("067d8c46-c334-4b00-af06-2e28b6086c58"),
    "Customs Depot",
    crate::card::CardArt::new("067d8c46-c334-4b00-af06-2e28b6086c58", "Scott M. Fischer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 72 — Darting Merfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARTING_MERFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("438e15f7-59bb-4047-af1f-ef92cc1866b8"),
    "Darting Merfolk",
    crate::card::CardArt::new("438e15f7-59bb-4047-af1f-ef92cc1866b8", "Sam Wood"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 73 — Dehydration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEHYDRATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c9e4043-e7a6-4c68-aa03-ef2f88e46451"),
    "Dehydration",
    crate::card::CardArt::new("2c9e4043-e7a6-4c68-aa03-ef2f88e46451", "Val Mayerik"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 74 — Diplomatic Escort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIPLOMATIC_ESCORT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9356bdbb-d647-4f51-a7a3-18ecea898a7f"),
    "Diplomatic Escort",
    crate::card::CardArt::new("9356bdbb-d647-4f51-a7a3-18ecea898a7f", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 75 — Diplomatic Immunity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIPLOMATIC_IMMUNITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb1e610e-a4a2-460b-8e4c-13674badbce3"),
    "Diplomatic Immunity",
    crate::card::CardArt::new("fb1e610e-a4a2-460b-8e4c-13674badbce3", "Terese Nielsen"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 76 — Drake Hatchling
pub(in crate::card::sets) static DRAKE_HATCHLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64ee32f9-6120-4f15-a692-89a4cd8167c6"),
    "Drake Hatchling",
    CardArt::new("64ee32f9-6120-4f15-a692-89a4cd8167c6", "Bradley Williams"),
    CardSet::MercadianMasques,
    // A 1/3 flier that attacks as a 2/3 once a turn, which is most of what
    // blue asked of a three-drop.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 1, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}: This creature gets +1/+0 until end of turn. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .activations_each_turn(1),
    ]),
);

// MMQ 77 — Embargo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBARGO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3fca3c65-f20e-4978-bfbb-ee7f9e1d829f"),
    "Embargo",
    crate::card::CardArt::new("3fca3c65-f20e-4978-bfbb-ee7f9e1d829f", "Nelson DeCastro"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 78 — Energy Flux (reprint)

// MMQ 79 — Extravagant Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTRAVAGANT_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99243564-9dbd-420c-922d-c17854c99d2a"),
    "Extravagant Spirit",
    crate::card::CardArt::new(
        "99243564-9dbd-420c-922d-c17854c99d2a",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 80 — False Demise (reprint)

// MMQ 81 — Glowing Anemone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOWING_ANEMONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("708593e6-787b-4f76-a86c-1d52857493ea"),
    "Glowing Anemone",
    crate::card::CardArt::new("708593e6-787b-4f76-a86c-1d52857493ea", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 82 — Gush
pub(in crate::card::sets) static GUSH: CardRecord = CardRecord::new_with_legacy_id(
    2045,
    "Gush",
    CardArt::new("e755bbef-bf34-49c0-ae72-d70e3599de52", "Kev Walker"),
    CardSet::MercadianMasques,
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
        .with_alternative_additional_cost(&CostDef::return_to_hand(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
            CostQuantityDef::Fixed(2),
        )),
    ]),
);

// MMQ 83 — High Seas
pub(in crate::card::sets) static HIGH_SEAS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f12eb6a6-14cc-4ad6-9684-ff33a39ba09f"),
    "High Seas",
    crate::card::CardArt::new(
        "f12eb6a6-14cc-4ad6-9684-ff33a39ba09f",
        "Massimiliano Frezzato",
    ),
    crate::card::CardSet::MercadianMasques,
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
pub(in crate::card::sets) static HOODWINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d505fbb-ec85-475b-a0e1-6670627ec017"),
    "Hoodwink",
    CardArt::new("8d505fbb-ec85-475b-a0e1-6670627ec017", "Arnie Swekel"),
    CardSet::MercadianMasques,
    // Two mana at instant speed to undo a land drop or bounce a rock, which
    // is tempo rather than an answer.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target artifact, enchantment, or land to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// MMQ 85 — Indentured Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INDENTURED_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dae62ce7-852b-42c6-9cbe-4807d8bf5740"),
    "Indentured Djinn",
    crate::card::CardArt::new("dae62ce7-852b-42c6-9cbe-4807d8bf5740", "Val Mayerik"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 86 — Karn's Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARN_S_TOUCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07845861-f974-43b7-829c-79a4a41ac3e3"),
    "Karn's Touch",
    crate::card::CardArt::new("07845861-f974-43b7-829c-79a4a41ac3e3", "Alan Pollack"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 87 — Misdirection
pub(in crate::card::sets) static MISDIRECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("581ad59c-29e9-4498-a6fd-33bf21e8e7c4"),
    "Misdirection",
    crate::card::CardArt::new("581ad59c-29e9-4498-a6fd-33bf21e8e7c4", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may exile a blue card from your hand rather than pay this spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&CostDef::exile(
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
    PrintingAnchor::scryfall("8e23f5a1-bf3e-41e0-875f-fc2f8508e69f"),
    "Misstep",
    crate::card::CardArt::new("8e23f5a1-bf3e-41e0-875f-fc2f8508e69f", "Kev Walker"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 89 — Overtaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERTAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b916a20-84f2-4e91-8dfa-039658735f5e"),
    "Overtaker",
    crate::card::CardArt::new("145903cb-9eaa-4f3c-a376-88dcd474ffda", "Clyde Caldwell"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 90 — Port Inspector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PORT_INSPECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef25d969-68d9-4580-bb29-f72bd5646a3d"),
    "Port Inspector",
    crate::card::CardArt::new("ef25d969-68d9-4580-bb29-f72bd5646a3d", "Dan Frazier"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 91 — Rishadan Airship
pub(in crate::card::sets) static RISHADAN_AIRSHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d8e596b-f5ef-405a-8910-c5d0b5c8c0fc"),
    "Rishadan Airship",
    CardArt::new("5d8e596b-f5ef-405a-8910-c5d0b5c8c0fc", "Kev Walker"),
    CardSet::MercadianMasques,
    // Three evasive damage for three mana. It dies to everything and
    // blocks almost nothing, which is what pays for the rate.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Pirate"], 3, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            // A restriction on the blocker rather than the attacker, so
            // it stops this creature from blocking on the ground without
            // saying anything about who may block it.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// MMQ 92 — Rishadan Brigand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_BRIGAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6efb653-97d8-4bc7-af8f-0b09fda655ff"),
    "Rishadan Brigand",
    crate::card::CardArt::new("a6efb653-97d8-4bc7-af8f-0b09fda655ff", "Scott Hampton"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 93 — Rishadan Cutpurse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_CUTPURSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("947fc270-11e3-46cd-9086-e880a5845c79"),
    "Rishadan Cutpurse",
    crate::card::CardArt::new(
        "947fc270-11e3-46cd-9086-e880a5845c79",
        "Christopher Moeller",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 94 — Rishadan Footpad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_FOOTPAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("493ee964-1a44-46a1-8606-90e215805483"),
    "Rishadan Footpad",
    crate::card::CardArt::new("493ee964-1a44-46a1-8606-90e215805483", "Adam Rex"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 95 — Sailmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAILMONGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("142479d8-8956-44a2-8c54-9dd6dc1774c0"),
    "Sailmonger",
    crate::card::CardArt::new("142479d8-8956-44a2-8c54-9dd6dc1774c0", "Michael Sutfin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 96 — Sand Squid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAND_SQUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4efd7ce9-b920-409d-a4d2-a07fff280712"),
    "Sand Squid",
    crate::card::CardArt::new("4efd7ce9-b920-409d-a4d2-a07fff280712", "Kev Walker"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 97 — Saprazzan Bailiff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_BAILIFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9f50964-1b57-426a-ac46-c90c045c7e40"),
    "Saprazzan Bailiff",
    crate::card::CardArt::new("a9f50964-1b57-426a-ac46-c90c045c7e40", "Ron Spencer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 98 — Saprazzan Breaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_BREAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2de7bf0f-5ad5-467b-ad80-28517951bbe1"),
    "Saprazzan Breaker",
    crate::card::CardArt::new("2de7bf0f-5ad5-467b-ad80-28517951bbe1", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 99 — Saprazzan Heir
pub(in crate::card::sets) static SAPRAZZAN_HEIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e3d913d-2dcf-4747-8169-0c44ec895864"),
    "Saprazzan Heir",
    CardArt::new("0e3d913d-2dcf-4747-8169-0c44ec895864", "Terese Nielsen"),
    CardSet::MercadianMasques,
    // Three cards for letting a 1/1 die, which is a price most boards can
    // afford to refuse.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you may draw three cards.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &const {
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    }
                },
            },
        ),
    ),
);

// MMQ 100 — Saprazzan Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_LEGATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db9adf84-ee7e-472b-bd96-9abf853afa83"),
    "Saprazzan Legate",
    crate::card::CardArt::new("db9adf84-ee7e-472b-bd96-9abf853afa83", "Andrew Goldhawk"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 101 — Saprazzan Outrigger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_OUTRIGGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b28048f1-4cf5-4389-9e69-9b5e1bc95396"),
    "Saprazzan Outrigger",
    crate::card::CardArt::new("b28048f1-4cf5-4389-9e69-9b5e1bc95396", "Doug Chaffee"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 102 — Saprazzan Raider
pub(in crate::card::sets) static SAPRAZZAN_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62493f34-cea8-4d9f-8781-005947b69c9d"),
    "Saprazzan Raider",
    CardArt::new("62493f34-cea8-4d9f-8781-005947b69c9d", "Jeff Miracola"),
    CardSet::MercadianMasques,
    // Unblockable in effect, though it goes home instead of through: the
    // attack is a threat the defender cannot profitably answer.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk"], 1, 2).with_ability(
        AbilityDef::triggered(
            "When this creature becomes blocked, return it to its owner's hand.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// MMQ 103 — Shoving Match
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOVING_MATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa9f4787-9b29-4f57-b105-1f9eb4bb8861"),
    "Shoving Match",
    crate::card::CardArt::new("aa9f4787-9b29-4f57-b105-1f9eb4bb8861", "Dave Dorman"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 104 — Soothsaying
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOOTHSAYING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("def384ff-1b6f-4c4f-8151-3c72c29b63ce"),
    "Soothsaying",
    crate::card::CardArt::new("def384ff-1b6f-4c4f-8151-3c72c29b63ce", "Pat Lewis"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 105 — Squeeze
pub(in crate::card::sets) static SQUEEZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbe63220-992b-459c-81ca-d4e2de273ce1"),
    "Squeeze",
    crate::card::CardArt::new("bbe63220-992b-459c-81ca-d4e2de273ce1", "DiTerlizzi"),
    crate::card::CardSet::MercadianMasques,
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
    PrintingAnchor::scryfall("76dcd19e-8daf-4d53-946b-c07d5eca3cc9"),
    "Statecraft",
    crate::card::CardArt::new("76dcd19e-8daf-4d53-946b-c07d5eca3cc9", "Mike Ploog"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 107 — Stinging Barrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STINGING_BARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca7f7cd5-4e91-474a-9f60-a66f3f462b1c"),
    "Stinging Barrier",
    crate::card::CardArt::new("ca7f7cd5-4e91-474a-9f60-a66f3f462b1c", "Pat Lewis"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 108 — Thwart
pub(in crate::card::sets) static THWART: CardRecord = CardRecord::new_with_legacy_id(
    2046,
    "Thwart",
    CardArt::new("c12a0717-e9ea-4be3-a29f-179671ed4489", "Christopher Moeller"),
    CardSet::MercadianMasques,
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
        .with_alternative_additional_cost(&CostDef::return_to_hand(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
            CostQuantityDef::Fixed(3),
        )),
    ]),
);

// MMQ 109 — Tidal Bore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_BORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f68fd547-59fb-41e6-be55-1ec17fe2840b"),
    "Tidal Bore",
    crate::card::CardArt::new("f68fd547-59fb-41e6-be55-1ec17fe2840b", "Frank Kelly Freas"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 110 — Tidal Kraken
pub(in crate::card::sets) static TIDAL_KRAKEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("356a9dcd-1a4b-4371-8f1d-aa7cb65e97e8"),
    "Tidal Kraken",
    CardArt::new(
        "356a9dcd-1a4b-4371-8f1d-aa7cb65e97e8",
        "Christopher Moeller",
    ),
    CardSet::MercadianMasques,
    // Eight mana for six unblockable damage a turn, which is three turns of
    // attacking and no way to stop it.
    CardRules::new_creature(mana_cost!("{5}{U}{U}{U}"), &["Kraken"], 6, 6)
        .with_ability(abilities::cannot_be_blocked()),
);

// MMQ 111 — Timid Drake (reprint)

// MMQ 112 — Trade Routes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADE_ROUTES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eeaba189-b215-4d1c-9135-a86ce5ec955d"),
    "Trade Routes",
    crate::card::CardArt::new("eeaba189-b215-4d1c-9135-a86ce5ec955d", "Matt Cavotta"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 113 — War Tax
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_TAX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7c15159-8466-43e5-9dc5-a8cc94619931"),
    "War Tax",
    crate::card::CardArt::new(
        "e7c15159-8466-43e5-9dc5-a8cc94619931",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 114 — Waterfront Bouncer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATERFRONT_BOUNCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dbdce9e-94fa-4ed5-9b97-d2026cffe7cb"),
    "Waterfront Bouncer",
    crate::card::CardArt::new("8dbdce9e-94fa-4ed5-9b97-d2026cffe7cb", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 115 — Alley Grifters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALLEY_GRIFTERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfb648e3-f5ad-4b33-afa3-d4cda0d369a1"),
    "Alley Grifters",
    crate::card::CardArt::new("cfb648e3-f5ad-4b33-afa3-d4cda0d369a1", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 116 — Black Market
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_MARKET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05976e5c-b46c-431e-9dbd-1dc5fad3536c"),
    "Black Market",
    crate::card::CardArt::new("05976e5c-b46c-431e-9dbd-1dc5fad3536c", "Jeff Easley"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 117 — Bog Smugglers
pub(in crate::card::sets) static BOG_SMUGGLERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2103a44-87e5-40cd-a0de-cd19456a8366"),
    "Bog Smugglers",
    CardArt::new("c2103a44-87e5-40cd-a0de-cd19456a8366", "Mike Ploog"),
    CardSet::MercadianMasques,
    // Unblockable against the mirror, which is what swampwalk on a black
    // creature actually means.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Mercenary"], 2, 2)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// MMQ 118 — Bog Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a926f9e-ee63-4b6e-8e5b-0650b74344a5"),
    "Bog Witch",
    crate::card::CardArt::new("6a926f9e-ee63-4b6e-8e5b-0650b74344a5", "Gao Yan"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 119 — Cackling Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CACKLING_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cec755ee-b4c0-47fd-9e61-9a3161766de6"),
    "Cackling Witch",
    crate::card::CardArt::new("cec755ee-b4c0-47fd-9e61-9a3161766de6", "Brian Despain"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 120 — Cateran Brute
pub(in crate::card::sets) static CATERAN_BRUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73b6ce76-0ed0-4994-ae2c-d8e51ae09920"),
    "Cateran Brute",
    CardArt::new(
        "73b6ce76-0ed0-4994-ae2c-d8e51ae09920",
        "Edward P. Beard, Jr.",
    ),
    CardSet::MercadianMasques,
    // The middle link, and the one that fetches the Persuader to start the
    // chain running the other way.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Horror", "Mercenary"], 2, 2).with_ability(
        AbilityDef::activated(
            "{2}, {T}: Search your library for a Mercenary permanent card with mana value \
             2 or less, put it onto the battlefield, then shuffle.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Mercenary"),
                    ObjectPredicateDef::ManaValueAtMost(2),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// MMQ 121 — Cateran Enforcer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_ENFORCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e9b6da8-39da-4fce-89cf-ea972f981331"),
    "Cateran Enforcer",
    crate::card::CardArt::new("9e9b6da8-39da-4fce-89cf-ea972f981331", "Mike Ploog"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 122 — Cateran Kidnappers
pub(in crate::card::sets) static CATERAN_KIDNAPPERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3768bdc1-4055-423a-a1cc-69b4c620e3e6"),
    "Cateran Kidnappers",
    CardArt::new("3768bdc1-4055-423a-a1cc-69b4c620e3e6", "Carl Critchlow"),
    CardSet::MercadianMasques,
    // Four power for four mana with a tutor attached, which is what the
    // chain pays for being slower than the Rebels.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Human", "Mercenary"], 4, 2).with_ability(
        AbilityDef::activated(
            "{3}, {T}: Search your library for a Mercenary permanent card with mana value \
             3 or less, put it onto the battlefield, then shuffle.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Mercenary"),
                    ObjectPredicateDef::ManaValueAtMost(3),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// MMQ 123 — Cateran Overlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_OVERLORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8a1ffcb-40a7-423f-b28a-b5b4c1c9ffd0"),
    "Cateran Overlord",
    crate::card::CardArt::new("e8a1ffcb-40a7-423f-b28a-b5b4c1c9ffd0", "Michael Sutfin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 124 — Cateran Persuader
pub(in crate::card::sets) static CATERAN_PERSUADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a98bdbf1-32a6-4d9b-8e57-5d3aca6b05bc"),
    "Cateran Persuader",
    CardArt::new("a98bdbf1-32a6-4d9b-8e57-5d3aca6b05bc", "Carl Critchlow"),
    CardSet::MercadianMasques,
    // The bottom of the Mercenary chain. Unlike the Rebels it searches
    // downward, so each link fetches something smaller than itself.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Human", "Mercenary"], 2, 1).with_ability(
        AbilityDef::activated(
            "{1}, {T}: Search your library for a Mercenary permanent card with mana value \
             1 or less, put it onto the battlefield, then shuffle.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Mercenary"),
                    ObjectPredicateDef::ManaValueAtMost(1),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// MMQ 125 — Cateran Slaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_SLAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d293c51-714c-45b8-bfa4-fe35e8f3fbc1"),
    "Cateran Slaver",
    crate::card::CardArt::new("2d293c51-714c-45b8-bfa4-fe35e8f3fbc1", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 126 — Cateran Summons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATERAN_SUMMONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af3de1f9-9038-4352-b4bf-2e9c5c27495a"),
    "Cateran Summons",
    crate::card::CardArt::new("af3de1f9-9038-4352-b4bf-2e9c5c27495a", "Alan Pollack"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 127 — Conspiracy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSPIRACY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("411c9f22-2df0-4a63-b2be-fa02612a6ef8"),
    "Conspiracy",
    crate::card::CardArt::new("411c9f22-2df0-4a63-b2be-fa02612a6ef8", "Jeff Easley"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 128 — Corrupt Official
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORRUPT_OFFICIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cb652fc-5a21-4e02-a776-a38fb41ad18c"),
    "Corrupt Official",
    crate::card::CardArt::new(
        "5cb652fc-5a21-4e02-a776-a38fb41ad18c",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 129 — Dark Ritual (reprint)

// MMQ 130 — Deathgazer
pub(in crate::card::sets) static DEATHGAZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0fff328-704e-462d-9613-82d05371f544"),
    "Deathgazer",
    CardArt::new("d0fff328-704e-462d-9613-82d05371f544", "Donato Giancola"),
    CardSet::MercadianMasques,
    // The same deal Dread Specter offers, reprinted for a block where the
    // mirror match was the one that mattered.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Lizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a nonblack creature, \
             destroy that creature at end of combat.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            },
            abilities::destroy_triggering_object_at_end_of_combat(),
        ),
    ),
);

// MMQ 131 — Deepwood Ghoul
pub(in crate::card::sets) static DEEPWOOD_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29cd6685-37ca-47c7-8f64-1fb86e9610ca"),
    "Deepwood Ghoul",
    CardArt::new("29cd6685-37ca-47c7-8f64-1fb86e9610ca", "Alan Pollack"),
    CardSet::MercadianMasques,
    // Life is the only cost, so it blocks forever against a deck with no
    // reach and not at all against one with any.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 1).with_ability(
        abilities::regenerate_self(
            "Pay 2 life: Regenerate this creature.",
            &[CostDef::PayLife(2)],
        ),
    ),
);

// MMQ 132 — Deepwood Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_LEGATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54f01925-7fd0-472d-91a4-3309e615f22f"),
    "Deepwood Legate",
    crate::card::CardArt::new("54f01925-7fd0-472d-91a4-3309e615f22f", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 133 — Delraich
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELRAICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da64094f-df6e-4c43-b4ae-03aab6b92816"),
    "Delraich",
    crate::card::CardArt::new("da64094f-df6e-4c43-b4ae-03aab6b92816", "Todd Lockwood"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 134 — Enslaved Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENSLAVED_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dffca723-360d-48de-a0a8-32288627f3df"),
    "Enslaved Horror",
    crate::card::CardArt::new("dffca723-360d-48de-a0a8-32288627f3df", "Mike Ploog"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 135 — Extortion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTORTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a66742db-4750-49ce-ad05-b825af7222c4"),
    "Extortion",
    crate::card::CardArt::new("a66742db-4750-49ce-ad05-b825af7222c4", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 136 — Forced March
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORCED_MARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36eae0e1-7100-449d-a259-7abfcd429117"),
    "Forced March",
    crate::card::CardArt::new(
        "36eae0e1-7100-449d-a259-7abfcd429117",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 137 — Ghoul's Feast
pub(in crate::card::sets) static GHOUL_S_FEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a0054c1-6510-41dd-8695-9bf50296b615"),
    "Ghoul's Feast",
    CardArt::new("6a0054c1-6510-41dd-8695-9bf50296b615", "Alan Pollack"),
    CardSet::MercadianMasques,
    // Two mana for a pump that grows all game, which is what a graveyard
    // deck plays instead of a bigger creature.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +X/+0 until end of turn, where X is the number of creature cards \
         in your graveyard.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            // Counted as the spell resolves, so a creature that died in
            // response is already there to be counted.
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::CountMatchingObjects(
                    &const {
                        ObjectQueryDef::owned_by(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Graveyard],
                            PlayerSetDef::Related(PlayerRelation::You),
                        )
                    },
                ),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// MMQ 138 — Haunted Crossroads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTED_CROSSROADS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c065cae-1ed5-445e-ace3-e81cf4c773de"),
    "Haunted Crossroads",
    crate::card::CardArt::new("3c065cae-1ed5-445e-ace3-e81cf4c773de", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 139 — Highway Robber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGHWAY_ROBBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc826c88-fe3c-4004-8283-27910c550fae"),
    "Highway Robber",
    crate::card::CardArt::new("fc826c88-fe3c-4004-8283-27910c550fae", "Kev Walker"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 140 — Instigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSTIGATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e3f57af-17d4-4a4c-ae46-fe37f97466fa"),
    "Instigator",
    crate::card::CardArt::new("2e3f57af-17d4-4a4c-ae46-fe37f97466fa", "Fred Fields"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 141 — Insubordination
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSUBORDINATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2544c9d-adc2-4d67-8850-9af38e73ea1e"),
    "Insubordination",
    crate::card::CardArt::new("d2544c9d-adc2-4d67-8850-9af38e73ea1e", "Andrew Goldhawk"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 142 — Intimidation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTIMIDATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b9e1724-91cf-422e-909b-ddb69a6f9f76"),
    "Intimidation",
    crate::card::CardArt::new("1b9e1724-91cf-422e-909b-ddb69a6f9f76", "Terese Nielsen"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 143 — Larceny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LARCENY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a863da2-0639-4eed-8da9-2e9a38c04a23"),
    "Larceny",
    crate::card::CardArt::new("3a863da2-0639-4eed-8da9-2e9a38c04a23", "Dave Dorman"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 144 — Liability
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIABILITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b07c66d-5f37-4098-b7e6-03e6c684806b"),
    "Liability",
    crate::card::CardArt::new(
        "0b07c66d-5f37-4098-b7e6-03e6c684806b",
        "Christopher Moeller",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 145 — Maggot Therapy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGGOT_THERAPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ab963aa-2304-4ee6-a8c7-c485c5133b40"),
    "Maggot Therapy",
    crate::card::CardArt::new("6ab963aa-2304-4ee6-a8c7-c485c5133b40", "Jeff Easley"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 146 — Midnight Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIDNIGHT_RITUAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e98c4f7-b0f4-48c6-b502-3dc5802d827f"),
    "Midnight Ritual",
    crate::card::CardArt::new("0e98c4f7-b0f4-48c6-b502-3dc5802d827f", "Jeff Easley"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 147 — Misshapen Fiend
pub(in crate::card::sets) static MISSHAPEN_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a43cf59e-7583-4651-968a-2a7201c69b6b"),
    "Misshapen Fiend",
    CardArt::new("a43cf59e-7583-4651-968a-2a7201c69b6b", "Adam Rex"),
    CardSet::MercadianMasques,
    // A 1/1 flier that is also a Mercenary, so the chain that fetches it
    // cares about the type more than the body.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Horror", "Mercenary"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// MMQ 148 — Molting Harpy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTING_HARPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ddfe33fb-71d5-4552-bcd3-f07e4e3847e1"),
    "Molting Harpy",
    crate::card::CardArt::new("ddfe33fb-71d5-4552-bcd3-f07e4e3847e1", "Jeff Laubenstein"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 149 — Nether Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NETHER_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("220217c5-408c-40df-8133-da16b13d4f21"),
    "Nether Spirit",
    crate::card::CardArt::new("220217c5-408c-40df-8133-da16b13d4f21", "Alan Pollack"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 150 — Notorious Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOTORIOUS_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("239e48d8-e2ba-4e25-88ef-301420c796b4"),
    "Notorious Assassin",
    crate::card::CardArt::new("239e48d8-e2ba-4e25-88ef-301420c796b4", "Heather Hudson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 151 — Pretender's Claim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRETENDER_S_CLAIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2cc29dab-9211-46bc-a98c-a5dbd5b0980a"),
    "Pretender's Claim",
    crate::card::CardArt::new(
        "2cc29dab-9211-46bc-a98c-a5dbd5b0980a",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 152 — Primeval Shambler
pub(in crate::card::sets) static PRIMEVAL_SHAMBLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d6ed1fb-2f7d-4a21-bbf3-660cad631975"),
    "Primeval Shambler",
    CardArt::new("5d6ed1fb-2f7d-4a21-bbf3-660cad631975", "Chippy"),
    CardSet::MercadianMasques,
    // A mana sink that turns a stalled board into a clock, which is what a
    // five-mana 3/3 has to do to be worth the slot.
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Horror", "Mercenary"], 3, 3).with_ability(
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MMQ 153 — Putrefaction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUTREFACTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65104b23-58a6-41c0-b887-90a3fb959289"),
    "Putrefaction",
    crate::card::CardArt::new("65104b23-58a6-41c0-b887-90a3fb959289", "DiTerlizzi"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 154 — Quagmire Lamprey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUAGMIRE_LAMPREY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c91c44e-6bfc-4595-9cdb-17d73f912c09"),
    "Quagmire Lamprey",
    crate::card::CardArt::new("3c91c44e-6bfc-4595-9cdb-17d73f912c09", "Glen Angus"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 155 — Rain of Tears (reprint)

// MMQ 156 — Rampart Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMPART_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b60f86f-c78a-4dfb-bb18-e9bcf21b26c4"),
    "Rampart Crawler",
    crate::card::CardArt::new("8b60f86f-c78a-4dfb-bb18-e9bcf21b26c4", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 157 — Rouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad01a8e2-5dc5-49a3-ad1c-7d5bf006b774"),
    "Rouse",
    crate::card::CardArt::new("ad01a8e2-5dc5-49a3-ad1c-7d5bf006b774", "Dave Dorman"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 158 — Scandalmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCANDALMONGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10c97baa-9bc0-4894-867c-ad1f56c469fd"),
    "Scandalmonger",
    crate::card::CardArt::new("10c97baa-9bc0-4894-867c-ad1f56c469fd", "Matt Cavotta"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 159 — Sever Soul
pub(in crate::card::sets) static SEVER_SOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2d84fec-18f1-4231-a293-0dc1ff868a40"),
    "Sever Soul",
    CardArt::new("c2d84fec-18f1-4231-a293-0dc1ff868a40", "Jeff Easley"),
    CardSet::MercadianMasques,
    // Five mana for removal and a life swing, which is the rate black paid
    // when it also wanted to survive the race.
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target nonblack creature. It can't be regenerated. You gain life equal to its \
         toughness.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // The life is its own sentence, so it is gained whether or not
            // the creature actually died, from last-known toughness.
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetToughness(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// MMQ 160 — Silent Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILENT_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba34034a-1150-41c8-a340-543f529ae07f"),
    "Silent Assassin",
    crate::card::CardArt::new("ba34034a-1150-41c8-a340-543f529ae07f", "rk post"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 161 — Skulking Fugitive
pub(in crate::card::sets) static SKULKING_FUGITIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("175ed19c-9635-45ee-bb2c-32b96270a246"),
    "Skulking Fugitive",
    CardArt::new("175ed19c-9635-45ee-bb2c-32b96270a246", "Scott M. Fischer"),
    CardSet::MercadianMasques,
    // The Masques printing of the same deal, fetchable by the Mercenary
    // chain, which is what makes it worth the drawback.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Horror", "Mercenary"], 3, 4).with_ability(
        AbilityDef::triggered(
            "When this creature becomes the target of a spell or ability, sacrifice it.",
            // Any spell or ability, including its controller's own: a
            // pump spell kills it just as surely as removal does.
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::Any),
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// MMQ 162 — Snuff Out
pub(in crate::card::sets) static SNUFF_OUT: CardRecord = CardRecord::new_with_legacy_id(
    2158,
    "Snuff Out",
    CardArt::new("18a3cca1-e50e-49b6-9e1a-f86640e3b177", "Mike Ploog"),
    CardSet::MercadianMasques,
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
        AbilityDef::spell_with_targets(
            "Destroy target nonblack creature. It can't be regenerated.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]),
            )],
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// MMQ 163 — Soul Channeling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_CHANNELING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55cd09ef-1655-4a62-b6c5-6eda33d2607a"),
    "Soul Channeling",
    crate::card::CardArt::new("55cd09ef-1655-4a62-b6c5-6eda33d2607a", "DiTerlizzi"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 164 — Specter's Wail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTER_S_WAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1637b62-e364-4250-aad5-841c6a47a11e"),
    "Specter's Wail",
    crate::card::CardArt::new("d1637b62-e364-4250-aad5-841c6a47a11e", "Randy Gallegos"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 165 — Strongarm Thug
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGARM_THUG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20aa9108-470c-484d-908a-c31cf6935765"),
    "Strongarm Thug",
    crate::card::CardArt::new("20aa9108-470c-484d-908a-c31cf6935765", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 166 — Thrashing Wumpus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRASHING_WUMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86bc07c6-2ba7-41f8-90ab-f9bbac86dd08"),
    "Thrashing Wumpus",
    crate::card::CardArt::new("86bc07c6-2ba7-41f8-90ab-f9bbac86dd08", "Jeff Miracola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 167 — Undertaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERTAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f615f531-e8af-4f7b-a4ea-fb962149093f"),
    "Undertaker",
    crate::card::CardArt::new("f615f531-e8af-4f7b-a4ea-fb962149093f", "Jeff Easley"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 168 — Unmask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNMASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2db7a0e6-eea5-4fa6-ac14-401411b106cc"),
    "Unmask",
    crate::card::CardArt::new("2db7a0e6-eea5-4fa6-ac14-401411b106cc", "rk post"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 169 — Unnatural Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNNATURAL_HUNGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3985f240-4289-4d48-978c-bb2ce2b54c36"),
    "Unnatural Hunger",
    crate::card::CardArt::new("3985f240-4289-4d48-978c-bb2ce2b54c36", "Jeff Miracola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 170 — Vendetta (reprint)

// MMQ 171 — Wall of Distortion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_DISTORTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2b2d07a-9ea1-430d-b432-ae507f4fe73b"),
    "Wall of Distortion",
    crate::card::CardArt::new("d2b2d07a-9ea1-430d-b432-ae507f4fe73b", "Mark Tedin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 172 — Arms Dealer (reprint)

// MMQ 173 — Battle Rampart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_RAMPART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f27f6658-0f00-4934-8d12-cd0dda3958c9"),
    "Battle Rampart",
    crate::card::CardArt::new("f27f6658-0f00-4934-8d12-cd0dda3958c9", "Ron Spencer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 174 — Battle Squadron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_SQUADRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37d55504-ee04-4a5a-a952-9ec5dc2db413"),
    "Battle Squadron",
    crate::card::CardArt::new("37d55504-ee04-4a5a-a952-9ec5dc2db413", "Mark Tedin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 175 — Blaster Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLASTER_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("801b0fd1-bbb2-47c0-a4c3-4129a67473b9"),
    "Blaster Mage",
    crate::card::CardArt::new("801b0fd1-bbb2-47c0-a4c3-4129a67473b9", "George Pratt"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 176 — Blood Hound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("baa1e796-809c-49af-a84e-ec088f7f48f8"),
    "Blood Hound",
    crate::card::CardArt::new("baa1e796-809c-49af-a84e-ec088f7f48f8", "Bradley Williams"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 177 — Blood Oath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_OATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1556a12-ff45-4a12-988a-63615b3799a9"),
    "Blood Oath",
    crate::card::CardArt::new("f1556a12-ff45-4a12-988a-63615b3799a9", "Mike Ploog"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 178 — Brawl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAWL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f4e783c-0717-4127-bd7b-885ca617ca29"),
    "Brawl",
    crate::card::CardArt::new(
        "3f4e783c-0717-4127-bd7b-885ca617ca29",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 179 — Cave Sense
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVE_SENSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d718421-c742-489c-a243-3adb19f6716a"),
    "Cave Sense",
    crate::card::CardArt::new("2d718421-c742-489c-a243-3adb19f6716a", "Mark Romanoski"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 180 — Cave-In
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVE_IN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("440d9d26-f304-467d-af79-914cc65f082e"),
    "Cave-In",
    crate::card::CardArt::new("440d9d26-f304-467d-af79-914cc65f082e", "Mark Tedin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 181 — Cavern Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVERN_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd0a8af9-2e86-4639-a6c9-209f115e95f8"),
    "Cavern Crawler",
    crate::card::CardArt::new("bd0a8af9-2e86-4639-a6c9-209f115e95f8", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 182 — Ceremonial Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEREMONIAL_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a6c69d1-5295-419f-bb8f-af826bf92cb3"),
    "Ceremonial Guard",
    crate::card::CardArt::new("4a6c69d1-5295-419f-bb8f-af826bf92cb3", "Daren Bader"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 183 — Cinder Elemental (reprint)

// MMQ 184 — Close Quarters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOSE_QUARTERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b9131c7-4e46-4c01-80b3-a6b055439346"),
    "Close Quarters",
    crate::card::CardArt::new("1b9131c7-4e46-4c01-80b3-a6b055439346", "Ron Spencer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 185 — Crag Saurian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRAG_SAURIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f0907a5-938e-4ef4-aa85-e7c1ae4317a6"),
    "Crag Saurian",
    crate::card::CardArt::new("1f0907a5-938e-4ef4-aa85-e7c1ae4317a6", "Matthew D. Wilson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 186 — Crash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a26bde3-8392-4476-b347-f223d52554a6"),
    "Crash",
    crate::card::CardArt::new("7a26bde3-8392-4476-b347-f223d52554a6", "Doug Chaffee"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 187 — Flailing Manticore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAILING_MANTICORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6eee8c2e-bda7-4bf9-80fe-87d96024ca8b"),
    "Flailing Manticore",
    crate::card::CardArt::new("6eee8c2e-bda7-4bf9-80fe-87d96024ca8b", "Roger Raupp"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 188 — Flailing Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAILING_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e400e520-b2b8-4c13-a4ea-f8810c927bf7"),
    "Flailing Ogre",
    crate::card::CardArt::new("e400e520-b2b8-4c13-a4ea-f8810c927bf7", "Daniel R. Horne"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 189 — Flailing Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAILING_SOLDIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb44b0f6-0608-40d6-9eaa-48e5a834701f"),
    "Flailing Soldier",
    crate::card::CardArt::new("fb44b0f6-0608-40d6-9eaa-48e5a834701f", "Dany Orizio"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 190 — Flaming Sword
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMING_SWORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17ecd9ff-8c30-4e17-8cff-dd40d653c4af"),
    "Flaming Sword",
    crate::card::CardArt::new("17ecd9ff-8c30-4e17-8cff-dd40d653c4af", "Randy Gallegos"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 191 — Furious Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FURIOUS_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27a07fae-0f34-45e7-b22d-97eea9031022"),
    "Furious Assault",
    crate::card::CardArt::new("27a07fae-0f34-45e7-b22d-97eea9031022", "Greg Staples"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 192 — Gerrard's Irregulars
pub(in crate::card::sets) static GERRARD_S_IRREGULARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a88f507-3d78-4f7f-a91f-8489ad9250f2"),
    "Gerrard's Irregulars",
    CardArt::new("8a88f507-3d78-4f7f-a91f-8489ad9250f2", "Eric Peterson"),
    CardSet::MercadianMasques,
    // Five mana for four damage the turn it lands, and trample so that a
    // chump block does not take all of it.
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Soldier"], 4, 2)
        .with_abilities(&[abilities::trample(), abilities::haste()]),
);

// MMQ 193 — Hammer Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAMMER_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b959d7ad-a78e-439f-9225-4dbb89f490d7"),
    "Hammer Mage",
    crate::card::CardArt::new("b959d7ad-a78e-439f-9225-4dbb89f490d7", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 194 — Hired Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIRED_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dc33920a-f05c-46fa-b94b-278af0022b78"),
    "Hired Giant",
    crate::card::CardArt::new("dc33920a-f05c-46fa-b94b-278af0022b78", "Ben Thompson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 195 — Kris Mage
pub(in crate::card::sets) static KRIS_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4389fbcd-182a-4cac-b14f-aa971948cf8e"),
    "Kris Mage",
    CardArt::new("4389fbcd-182a-4cac-b14f-aa971948cf8e", "Matthew D. Wilson"),
    CardSet::MercadianMasques,
    // A one-drop that turns spare cards into damage, which is the whole
    // Spellshaper idea in its cheapest form.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Spellshaper"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}, Discard a card: This creature deals 1 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{R}")),
                CostDef::TapSource,
                CostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// MMQ 196 — Kyren Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bc55e01-342e-4856-937e-14561b8d165b"),
    "Kyren Glider",
    crate::card::CardArt::new("0bc55e01-342e-4856-937e-14561b8d165b", "Daren Bader"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 197 — Kyren Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_LEGATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f0e9806-be8c-4b88-a4be-0111d1be81d9"),
    "Kyren Legate",
    crate::card::CardArt::new("6f0e9806-be8c-4b88-a4be-0111d1be81d9", "Dave Dorman"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 198 — Kyren Negotiations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_NEGOTIATIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c263a17-bbc2-433e-93f8-72e57b818322"),
    "Kyren Negotiations",
    crate::card::CardArt::new("0c263a17-bbc2-433e-93f8-72e57b818322", "Scott Hampton"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 199 — Kyren Sniper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_SNIPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4df99e19-0b1e-48ec-a146-38cf147eab61"),
    "Kyren Sniper",
    crate::card::CardArt::new("4df99e19-0b1e-48ec-a146-38cf147eab61", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 200 — Lava Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_RUNNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("09d0fbe6-6ce1-4b95-afb7-a7386b5033cf"),
    "Lava Runner",
    crate::card::CardArt::new("09d0fbe6-6ce1-4b95-afb7-a7386b5033cf", "Donato Giancola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 201 — Lightning Hounds
pub(in crate::card::sets) static LIGHTNING_HOUNDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38c82a1d-5db1-4090-b446-cc5bc6dc811d"),
    "Lightning Hounds",
    CardArt::new("38c82a1d-5db1-4090-b446-cc5bc6dc811d", "Andrew Robinson"),
    CardSet::MercadianMasques,
    // A 3/2 first striker for four, which beats every three-drop it runs
    // into and dies to the burn spell aimed at it.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dog"], 3, 2)
        .with_abilities(&[abilities::first_strike()]),
);

// MMQ 202 — Lithophage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LITHOPHAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98ee0f17-de64-4abb-afad-4005275f1a3c"),
    "Lithophage",
    crate::card::CardArt::new("98ee0f17-de64-4abb-afad-4005275f1a3c", "Mike Ploog"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 203 — Lunge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUNGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9e43349-429c-43f7-b808-c4bf37370a9f"),
    "Lunge",
    crate::card::CardArt::new("e9e43349-429c-43f7-b808-c4bf37370a9f", "Dan Frazier"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 204 — Magistrate's Veto
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGISTRATE_S_VETO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f83d39e-bf49-4968-829e-c0e9abf2fb86"),
    "Magistrate's Veto",
    crate::card::CardArt::new("2f83d39e-bf49-4968-829e-c0e9abf2fb86", "Brian Snõddy"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 205 — Mercadia's Downfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIA_S_DOWNFALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14507fe6-80a9-4ed4-bf3e-4656f3d377c0"),
    "Mercadia's Downfall",
    crate::card::CardArt::new("14507fe6-80a9-4ed4-bf3e-4656f3d377c0", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 206 — Ogre Taskmaster (reprint)

// MMQ 207 — Pulverize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULVERIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afbbc44d-60fb-45fc-a588-14aab0340134"),
    "Pulverize",
    crate::card::CardArt::new("afbbc44d-60fb-45fc-a588-14aab0340134", "Scott M. Fischer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 208 — Puppet's Verdict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUPPET_S_VERDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("052b743a-456d-49c3-881e-4f30c7645fa5"),
    "Puppet's Verdict",
    crate::card::CardArt::new(
        "052b743a-456d-49c3-881e-4f30c7645fa5",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 209 — Robber Fly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROBBER_FLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7d5cf073-2ba0-463e-bcd4-979ad18e28fc"),
    "Robber Fly",
    crate::card::CardArt::new("7d5cf073-2ba0-463e-bcd4-979ad18e28fc", "John Matson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 210 — Rock Badger
pub(in crate::card::sets) static ROCK_BADGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dff05df8-76f5-48c6-ac96-7b4e6a7050f6"),
    "Rock Badger",
    CardArt::new("dff05df8-76f5-48c6-ac96-7b4e6a7050f6", "Heather Hudson"),
    CardSet::MercadianMasques,
    // Five mana for a 3/3 that is unblockable in the mirror and ordinary
    // against everybody else.
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Badger", "Beast"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Mountain)),
);

// MMQ 211 — Seismic Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEISMIC_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9524432a-3186-4c7b-a780-28bdbe36053f"),
    "Seismic Mage",
    crate::card::CardArt::new("9524432a-3186-4c7b-a780-28bdbe36053f", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 212 — Shock Troops
pub(in crate::card::sets) static SHOCK_TROOPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7a918ca-3e60-46de-9f29-56bdc6430a77"),
    "Shock Troops",
    CardArt::new("e7a918ca-3e60-46de-9f29-56bdc6430a77", "Jeff Miracola"),
    CardSet::MercadianMasques,
    // Mogg Fanatic for four mana and twice the damage, and the sacrifice
    // is the cost, so the body is gone before the damage resolves.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: It deals 2 damage to any target.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// MMQ 213 — Sizzle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIZZLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1ca1eee-d97d-48c6-84f1-7d1f972c3ca9"),
    "Sizzle",
    crate::card::CardArt::new("f1ca1eee-d97d-48c6-84f1-7d1f972c3ca9", "Brian Snõddy"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 214 — Squee, Goblin Nabob
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUEE_GOBLIN_NABOB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ba8325a-1203-4125-9111-94d9e2b1f14b"),
    "Squee, Goblin Nabob",
    crate::card::CardArt::new("4ba8325a-1203-4125-9111-94d9e2b1f14b", "David Monette"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 215 — Stone Rain (reprint)

// MMQ 216 — Tectonic Break
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TECTONIC_BREAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9de0ee5d-10f6-4152-8416-1f2b749b439d"),
    "Tectonic Break",
    crate::card::CardArt::new("9de0ee5d-10f6-4152-8416-1f2b749b439d", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 217 — Territorial Dispute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRITORIAL_DISPUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4fa8a13a-f09f-4b10-8fab-3ea4fdc643d1"),
    "Territorial Dispute",
    crate::card::CardArt::new("4fa8a13a-f09f-4b10-8fab-3ea4fdc643d1", "Mike Ploog"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 218 — Thieves' Auction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THIEVES_AUCTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5708c87-108d-4ba1-a1e9-e83cb9b16b6c"),
    "Thieves' Auction",
    crate::card::CardArt::new("b5708c87-108d-4ba1-a1e9-e83cb9b16b6c", "Kevin Murphy"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 219 — Thunderclap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERCLAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3f8c5ee-2179-4c05-adc9-0b66d02b59ad"),
    "Thunderclap",
    crate::card::CardArt::new("b3f8c5ee-2179-4c05-adc9-0b66d02b59ad", "Tom Wänerstrand"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 220 — Tremor (reprint)

// MMQ 221 — Two-Headed Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWO_HEADED_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40fed2c7-c922-41c3-b86b-a8ed41a1308d"),
    "Two-Headed Dragon",
    crate::card::CardArt::new("40fed2c7-c922-41c3-b86b-a8ed41a1308d", "Sam Wood"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 222 — Uphill Battle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UPHILL_BATTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73fa3455-3ba0-41ad-aefd-40f183aed2a6"),
    "Uphill Battle",
    crate::card::CardArt::new("73fa3455-3ba0-41ad-aefd-40f183aed2a6", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 223 — Volcanic Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLCANIC_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c69dd00-46ce-42b2-a2ed-43b4cf04a975"),
    "Volcanic Wind",
    crate::card::CardArt::new("3c69dd00-46ce-42b2-a2ed-43b4cf04a975", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 224 — War Cadence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_CADENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e030d2eb-70c5-4ff7-8f03-ad5495cf9c69"),
    "War Cadence",
    crate::card::CardArt::new("e030d2eb-70c5-4ff7-8f03-ad5495cf9c69", "John Matson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 225 — Warmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARMONGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c4077d6-d98c-4fdd-ba57-0781aa21f68b"),
    "Warmonger",
    crate::card::CardArt::new("5577ac30-ee84-4d3c-b407-82578779dc90", "Heather Hudson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 226 — Warpath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARPATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e031c819-1237-4911-8a1d-87d6095a5faa"),
    "Warpath",
    crate::card::CardArt::new("e031c819-1237-4911-8a1d-87d6095a5faa", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 227 — Wild Jhovall
pub(in crate::card::sets) static WILD_JHOVALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64bcc06a-de86-4387-882d-ead33e9c9e01"),
    "Wild Jhovall",
    CardArt::new("64bcc06a-de86-4387-882d-ead33e9c9e01", "Daren Bader"),
    CardSet::MercadianMasques,
    // A vanilla 3/3 for four, printed so the Cat deck had a Cat to play on
    // turn four.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Cat"], 3, 3),
);

// MMQ 228 — Word of Blasting (reprint)

// MMQ 229 — Ancestral Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_MASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1203f98a-fb6e-4f16-88e3-553eba177450"),
    "Ancestral Mask",
    crate::card::CardArt::new(
        "1203f98a-fb6e-4f16-88e3-553eba177450",
        "Massimiliano Frezzato",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 230 — Bifurcate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIFURCATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dedb483e-b2c6-46c6-b02b-a49599d33521"),
    "Bifurcate",
    crate::card::CardArt::new("dedb483e-b2c6-46c6-b02b-a49599d33521", "John Matson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 231 — Boa Constrictor
pub(in crate::card::sets) static BOA_CONSTRICTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7369cbf-6986-4a39-b07c-a283b40aee40"),
    "Boa Constrictor",
    CardArt::new("f7369cbf-6986-4a39-b07c-a283b40aee40", "Carl Critchlow"),
    CardSet::MercadianMasques,
    // Tapping is the cost, so the pump happens on defence or not at all: it
    // blocks as a 6/6 and attacks as a 3/3.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Snake"], 3, 3).with_ability(
        AbilityDef::activated(
            "{T}: This creature gets +3/+3 until end of turn.",
            &[CostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MMQ 232 — Briar Patch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIAR_PATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5913fc6-eeb0-411b-9264-1e75bea8489b"),
    "Briar Patch",
    crate::card::CardArt::new("b5913fc6-eeb0-411b-9264-1e75bea8489b", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 233 — Caller of the Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLER_OF_THE_HUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0e8e1cf-0a47-4ce4-889a-091229d0e466"),
    "Caller of the Hunt",
    crate::card::CardArt::new("c0e8e1cf-0a47-4ce4-889a-091229d0e466", "Clyde Caldwell"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 234 — Caustic Wasps
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAUSTIC_WASPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59a46e20-2910-4287-a5e0-bccac8cbabcd"),
    "Caustic Wasps",
    crate::card::CardArt::new("59a46e20-2910-4287-a5e0-bccac8cbabcd", "Glen Angus"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 235 — Clear the Land
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLEAR_THE_LAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b87f3579-f314-4207-a02c-14e9cb269b47"),
    "Clear the Land",
    crate::card::CardArt::new("b87f3579-f314-4207-a02c-14e9cb269b47", "Bradley Williams"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 236 — Collective Unconscious
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLECTIVE_UNCONSCIOUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8fa7d6a8-9190-403f-bbdd-ab71d9c89e4d"),
    "Collective Unconscious",
    crate::card::CardArt::new("8fa7d6a8-9190-403f-bbdd-ab71d9c89e4d", "Andrew Goldhawk"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 237 — Dawnstrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWNSTRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d193a35-8950-4a77-ace3-c4d4085727f4"),
    "Dawnstrider",
    crate::card::CardArt::new("2d193a35-8950-4a77-ace3-c4d4085727f4", "rk post"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 238 — Deadly Insect (reprint)

// MMQ 239 — Deepwood Drummer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_DRUMMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("acbed0f5-2ac0-48d8-b5ab-b4cd7176fde2"),
    "Deepwood Drummer",
    crate::card::CardArt::new("acbed0f5-2ac0-48d8-b5ab-b4cd7176fde2", "Ron Spears"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 240 — Deepwood Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWOOD_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2a1ed74-027e-4c8e-ac7e-e58c5fccff14"),
    "Deepwood Elder",
    crate::card::CardArt::new(
        "d2a1ed74-027e-4c8e-ac7e-e58c5fccff14",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 241 — Deepwood Tantiv
pub(in crate::card::sets) static DEEPWOOD_TANTIV: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bfa2028e-4e73-4ff2-a9e2-9ac347d67893"),
    "Deepwood Tantiv",
    CardArt::new("bfa2028e-4e73-4ff2-a9e2-9ac347d67893", "Joel Biske"),
    CardSet::MercadianMasques,
    // A body that punishes blocking rather than attacking, which is what a
    // 2/4 is for.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 2, 4).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you gain 2 life.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// MMQ 242 — Deepwood Wolverine
pub(in crate::card::sets) static DEEPWOOD_WOLVERINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db9a9a76-741a-4ba3-bd4b-0eb87d678253"),
    "Deepwood Wolverine",
    CardArt::new("db9a9a76-741a-4ba3-bd4b-0eb87d678253", "Ray Lago"),
    CardSet::MercadianMasques,
    // A one-mana 1/1 that punishes a block, so it either gets through or
    // trades up.
    CardRules::new_creature(mana_cost!("{G}"), &["Wolverine"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets +2/+0 until end of turn.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MMQ 243 — Desert Twister (reprint)

// MMQ 244 — Erithizon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERITHIZON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec4ea4e2-2102-4b99-bea5-6fc4203f2b26"),
    "Erithizon",
    crate::card::CardArt::new("ec4ea4e2-2102-4b99-bea5-6fc4203f2b26", "Scott M. Fischer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 245 — Ferocity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEROCITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4afda489-8397-4ad4-89dc-e8bad92db133"),
    "Ferocity",
    crate::card::CardArt::new("4afda489-8397-4ad4-89dc-e8bad92db133", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 246 — Food Chain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOD_CHAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("18a1bb9e-006c-495e-8f99-d451183d2669"),
    "Food Chain",
    crate::card::CardArt::new("18a1bb9e-006c-495e-8f99-d451183d2669", "Val Mayerik"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 247 — Foster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOSTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be54e1d7-1388-4184-ad0d-dde4b0a3d02d"),
    "Foster",
    crate::card::CardArt::new("be54e1d7-1388-4184-ad0d-dde4b0a3d02d", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 248 — Game Preserve
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAME_PRESERVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1bb62356-72bb-4dc1-a2f9-45a3aca62e41"),
    "Game Preserve",
    crate::card::CardArt::new("1bb62356-72bb-4dc1-a2f9-45a3aca62e41", "Luca Zontini"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 249 — Giant Caterpillar (reprint)

// MMQ 250 — Groundskeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROUNDSKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31d9fe16-562a-4a86-84ed-15cd90b8afc0"),
    "Groundskeeper",
    crate::card::CardArt::new("31d9fe16-562a-4a86-84ed-15cd90b8afc0", "Alan Rabinowitz"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 251 — Horned Troll
pub(in crate::card::sets) static HORNED_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f2a6d10-054e-4d6f-aeb7-4204f02490c7"),
    "Horned Troll",
    CardArt::new("7f2a6d10-054e-4d6f-aeb7-4204f02490c7", "Heather Hudson"),
    CardSet::MercadianMasques,
    // A 2/2 for three that survives combat as long as the mana is open,
    // which is what green paid for instead of size.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Troll"], 2, 2).with_ability(
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{G}"))],
        ),
    ),
);

// MMQ 252 — Howling Wolf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOWLING_WOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7416c68a-5a6a-4d51-8dc7-5c62da81ec77"),
    "Howling Wolf",
    crate::card::CardArt::new("7416c68a-5a6a-4d51-8dc7-5c62da81ec77", "Heather Hudson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 253 — Hunted Wumpus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTED_WUMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b21c8b2d-ef0f-4839-acfc-20fd248c62cf"),
    "Hunted Wumpus",
    crate::card::CardArt::new("b21c8b2d-ef0f-4839-acfc-20fd248c62cf", "Brian Snõddy"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 254 — Invigorate
pub(in crate::card::sets) static INVIGORATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("406b343c-90b5-4a4d-91c3-2fddcc9a0e05"),
    "Invigorate",
    CardArt::new("406b343c-90b5-4a4d-91c3-2fddcc9a0e05", "Dan Frazier"),
    CardSet::MercadianMasques,
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
    PrintingAnchor::scryfall("d6862005-32d1-473e-a28b-5dfc4b7782cd"),
    "Land Grant",
    crate::card::CardArt::new(
        "d6862005-32d1-473e-a28b-5dfc4b7782cd",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 256 — Ley Line
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEY_LINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8990efd-708a-4019-bce0-2d6409ecc004"),
    "Ley Line",
    crate::card::CardArt::new("f8990efd-708a-4019-bce0-2d6409ecc004", "Terese Nielsen"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 257 — Lumbering Satyr
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMBERING_SATYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d897088-0667-4864-91c3-5f0ac7f9b220"),
    "Lumbering Satyr",
    crate::card::CardArt::new("5d897088-0667-4864-91c3-5f0ac7f9b220", "Alan Pollack"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 258 — Lure (reprint)

// MMQ 259 — Megatherium
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEGATHERIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c58a1e43-a173-45d6-ac55-363664bf6e1b"),
    "Megatherium",
    crate::card::CardArt::new("c58a1e43-a173-45d6-ac55-363664bf6e1b", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 260 — Natural Affinity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURAL_AFFINITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69c6f647-f71b-4f61-9b16-774884ed52e2"),
    "Natural Affinity",
    crate::card::CardArt::new("69c6f647-f71b-4f61-9b16-774884ed52e2", "Pete Venters"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 261 — Pangosaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANGOSAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0335d282-cd1a-4be3-8eb2-82aaee91401a"),
    "Pangosaur",
    crate::card::CardArt::new("0335d282-cd1a-4be3-8eb2-82aaee91401a", "Mark Tedin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 262 — Revive (reprint)

// MMQ 263 — Rushwood Dryad
pub(in crate::card::sets) static RUSHWOOD_DRYAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55367a94-b343-4a04-bfa9-47722e32cc45"),
    "Rushwood Dryad",
    CardArt::new("55367a94-b343-4a04-bfa9-47722e32cc45", "Todd Lockwood"),
    CardSet::MercadianMasques,
    // Two mana for two damage a turn against any green deck, and a plain
    // 2/1 against everybody else.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 2, 1)
        .with_ability(abilities::landwalk(BasicLandType::Forest)),
);

// MMQ 264 — Rushwood Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHWOOD_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52128694-d9f5-4acb-b684-bb02a4e766b8"),
    "Rushwood Elemental",
    crate::card::CardArt::new("52128694-d9f5-4acb-b684-bb02a4e766b8", "Hannibal King"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 265 — Rushwood Herbalist
pub(in crate::card::sets) static RUSHWOOD_HERBALIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9afde98f-a429-4eff-9d06-8582267ac74b"),
    "Rushwood Herbalist",
    CardArt::new("9afde98f-a429-4eff-9d06-8582267ac74b", "Terese Nielsen"),
    CardSet::MercadianMasques,
    // A Spellshaper body on the Medicine Bag effect, which trades the
    // artifact's durability for a creature that can also attack.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Spellshaper"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}, Discard a card: Regenerate target creature.",
            &[
                CostDef::Mana(mana_cost!("{G}")),
                CostDef::TapSource,
                CostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// MMQ 266 — Rushwood Legate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSHWOOD_LEGATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("827b9c99-87d7-493c-9dc3-0c6aa4a61b49"),
    "Rushwood Legate",
    crate::card::CardArt::new("827b9c99-87d7-493c-9dc3-0c6aa4a61b49", "Mark Romanoski"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 267 — Saber Ants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SABER_ANTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9269f52-1002-475d-a0f3-d652630591ca"),
    "Saber Ants",
    crate::card::CardArt::new("e9269f52-1002-475d-a0f3-d652630591ca", "Greg Staples"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 268 — Sacred Prey
pub(in crate::card::sets) static SACRED_PREY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e965d32c-3151-48e8-b256-0b7fa8a8a211"),
    "Sacred Prey",
    CardArt::new("e965d32c-3151-48e8-b256-0b7fa8a8a211", "Rebecca Guay"),
    CardSet::MercadianMasques,
    // The one-mana version of the same idea, and about as small as it
    // could be.
    CardRules::new_creature(mana_cost!("{G}"), &["Horse"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you gain 1 life.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// MMQ 269 — Silverglade Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVERGLADE_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f222fe90-ac92-4ba9-b060-9b64075bf139"),
    "Silverglade Elemental",
    crate::card::CardArt::new("f222fe90-ac92-4ba9-b060-9b64075bf139", "Chippy"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 270 — Silverglade Pathfinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVERGLADE_PATHFINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bc99b33-ce06-4a44-8b23-300b41b2b2fe"),
    "Silverglade Pathfinder",
    crate::card::CardArt::new("9bc99b33-ce06-4a44-8b23-300b41b2b2fe", "rk post"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 271 — Snake Pit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAKE_PIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("059a70a5-d4fb-445e-af98-e81821df2c59"),
    "Snake Pit",
    crate::card::CardArt::new("059a70a5-d4fb-445e-af98-e81821df2c59", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 272 — Snorting Gahr
pub(in crate::card::sets) static SNORTING_GAHR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e568503e-a886-4c8b-9d46-8520c2cdda48"),
    "Snorting Gahr",
    CardArt::new("e568503e-a886-4c8b-9d46-8520c2cdda48", "Andrew Goldhawk"),
    CardSet::MercadianMasques,
    // Four mana for a 3/3 that blocks are afraid of, which is most of
    // what green commons did that year.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Rhino", "Beast"], 3, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets +2/+2 until end of turn.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MMQ 273 — Spidersilk Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDERSILK_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9eb7694f-af4c-4152-b868-528257d05154"),
    "Spidersilk Armor",
    crate::card::CardArt::new("9eb7694f-af4c-4152-b868-528257d05154", "Scott Hampton"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 274 — Spontaneous Generation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPONTANEOUS_GENERATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce5765cb-00cd-4920-9fe8-68791048ec4a"),
    "Spontaneous Generation",
    crate::card::CardArt::new("ce5765cb-00cd-4920-9fe8-68791048ec4a", "Alan Pollack"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 275 — Squall
pub(in crate::card::sets) static SQUALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63c1b2f6-e47f-4f18-a94a-1d08eb009ef3"),
    "Squall",
    CardArt::new("e5409b54-66ed-4add-bf43-cfeb074b1c50", "Val Mayerik"),
    CardSet::MercadianMasques,
    // Half a Needle Storm at the same cost, which is what a common gets.
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Squall deals 2 damage to each creature with flying.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(2),
        },
    )]),
);

// MMQ 276 — Squallmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUALLMONGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c845e1b8-6a39-456c-aa67-d180ae63e200"),
    "Squallmonger",
    crate::card::CardArt::new("c845e1b8-6a39-456c-aa67-d180ae63e200", "Heather Hudson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 277 — Stamina
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMINA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed8abca3-6e31-49cd-b9bf-86ad68e1cc83"),
    "Stamina",
    crate::card::CardArt::new("ed8abca3-6e31-49cd-b9bf-86ad68e1cc83", "Paolo Parente"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 278 — Sustenance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSTENANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a61db44-80dc-4058-9c9d-65cd18e63fd4"),
    "Sustenance",
    crate::card::CardArt::new("5a61db44-80dc-4058-9c9d-65cd18e63fd4", "Qiao Dafu"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 279 — Tiger Claws
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIGER_CLAWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0146a689-4817-4849-a90d-4cc64566960d"),
    "Tiger Claws",
    crate::card::CardArt::new("0146a689-4817-4849-a90d-4cc64566960d", "Adam Rex"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 280 — Tranquility (reprint)

// MMQ 281 — Venomous Breath (reprint)

// MMQ 282 — Venomous Dragonfly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMOUS_DRAGONFLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("479fc902-ce94-4a6b-af87-4645387a46c6"),
    "Venomous Dragonfly",
    crate::card::CardArt::new("479fc902-ce94-4a6b-af87-4645387a46c6", "Tom Wänerstrand"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 283 — Vernal Equinox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERNAL_EQUINOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bed69d2-f5fb-4173-b939-5abdb48b82b4"),
    "Vernal Equinox",
    crate::card::CardArt::new("3bed69d2-f5fb-4173-b939-5abdb48b82b4", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 284 — Vine Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINE_DRYAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc9c9158-faed-42ae-9f6b-71dee49ff79f"),
    "Vine Dryad",
    crate::card::CardArt::new("fc9c9158-faed-42ae-9f6b-71dee49ff79f", "Jeff Laubenstein"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 285 — Vine Trellis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINE_TRELLIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e660241f-0976-4206-8149-7dac8466a2a3"),
    "Vine Trellis",
    crate::card::CardArt::new("e660241f-0976-4206-8149-7dac8466a2a3", "DiTerlizzi"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 286 — Assembly Hall
pub(in crate::card::sets) static ASSEMBLY_HALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1676ccbb-91d2-4f26-b3a5-ccb1a21bdebf"),
    "Assembly Hall",
    crate::card::CardArt::new("1676ccbb-91d2-4f26-b3a5-ccb1a21bdebf", "Val Mayerik"),
    crate::card::CardSet::MercadianMasques,
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated(
        "{4}, {T}, Reveal a creature card from your hand: Search your library for a card with the same name as the revealed card, reveal it, put it into your hand, then shuffle.",
        &[
            CostDef::Mana(mana_cost!("{4}")),
            CostDef::TapSource,
            CostDef::RevealCardFromHand(ObjectPredicateDef::HasType(CardType::Creature)),
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::NameEquals(CardNameDef::NameOf(
                ObjectRefDef::AdditionalCostObject(AdditionalCostObjectIndex::PRIMARY),
            )),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: crate::card::ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// MMQ 287 — Barbed Wire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_WIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be9e2e59-1527-4c61-9cc9-dcaf1181bd43"),
    "Barbed Wire",
    crate::card::CardArt::new("be9e2e59-1527-4c61-9cc9-dcaf1181bd43", "Ron Spencer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 288 — Bargaining Table
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARGAINING_TABLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85da9395-42e9-4408-832d-74ea4b01256b"),
    "Bargaining Table",
    crate::card::CardArt::new("85da9395-42e9-4408-832d-74ea4b01256b", "Scott M. Fischer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 289 — Credit Voucher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CREDIT_VOUCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ab65242-17ad-4c22-9c70-aac8076d1b4c"),
    "Credit Voucher",
    crate::card::CardArt::new(
        "1ab65242-17ad-4c22-9c70-aac8076d1b4c",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 290 — Crenellated Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRENELLATED_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d85ad08d-1120-411a-8bbe-ac93a56476bd"),
    "Crenellated Wall",
    crate::card::CardArt::new("d85ad08d-1120-411a-8bbe-ac93a56476bd", "Arnie Swekel"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 291 — Crooked Scales
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROOKED_SCALES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd7084ba-cca6-4fb9-b21b-b79e7d74c5c0"),
    "Crooked Scales",
    crate::card::CardArt::new("fd7084ba-cca6-4fb9-b21b-b79e7d74c5c0", "Ron Spears"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 292 — Crumbling Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUMBLING_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8fa6d6c-c1cd-46f3-8430-94f67be55bf7"),
    "Crumbling Sanctuary",
    crate::card::CardArt::new("d8fa6d6c-c1cd-46f3-8430-94f67be55bf7", "Randy Gallegos"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 293 — Distorting Lens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISTORTING_LENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab196d8d-5d1c-4f0e-a924-37774db02821"),
    "Distorting Lens",
    crate::card::CardArt::new("ab196d8d-5d1c-4f0e-a924-37774db02821", "Glen Angus"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 294 — Eye of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EYE_OF_RAMOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("78d22400-39f6-444d-b508-783a7df7e945"),
    "Eye of Ramos",
    crate::card::CardArt::new("78d22400-39f6-444d-b508-783a7df7e945", "David Martin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 295 — General's Regalia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENERAL_S_REGALIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb99d982-8ab1-4d6a-ba24-58ac23a9b9e7"),
    "General's Regalia",
    crate::card::CardArt::new("fb99d982-8ab1-4d6a-ba24-58ac23a9b9e7", "David Monette"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 296 — Heart of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_OF_RAMOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0046226-7563-4345-aa4b-a2c732c2780a"),
    "Heart of Ramos",
    crate::card::CardArt::new("a0046226-7563-4345-aa4b-a2c732c2780a", "David Martin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 297 — Henge Guardian
pub(in crate::card::sets) static HENGE_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("028e5e18-b639-4461-87e4-5306371440b5"),
    "Henge Guardian",
    CardArt::new("028e5e18-b639-4461-87e4-5306371440b5", "Chippy"),
    CardSet::MercadianMasques,
    // Colourless trample on demand, so any deck can turn a 3/4 into a real
    // attacker for two more mana.
    CardRules::new_creature(mana_cost!("{5}"), &["Dragon", "Wurm"], 3, 4).with_ability(
        AbilityDef::activated(
            "{2}: This creature gains trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::trample() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MMQ 298 — Horn of Plenty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORN_OF_PLENTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5e04462-1d10-47df-b456-211dd0a87891"),
    "Horn of Plenty",
    crate::card::CardArt::new("d5e04462-1d10-47df-b456-211dd0a87891", "Brian Despain"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 299 — Horn of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORN_OF_RAMOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b17f541-8e9d-43b0-b688-e3f2e7fa55c8"),
    "Horn of Ramos",
    crate::card::CardArt::new("6b17f541-8e9d-43b0-b688-e3f2e7fa55c8", "David Martin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 300 — Iron Lance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_LANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41f7d212-faf2-4a6f-a338-d9e5014b56d5"),
    "Iron Lance",
    crate::card::CardArt::new("41f7d212-faf2-4a6f-a338-d9e5014b56d5", "Scott M. Fischer"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 301 — Jeweled Torque
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JEWELED_TORQUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eab076bc-e4c3-42a1-b701-9bc49bcc3cdd"),
    "Jeweled Torque",
    crate::card::CardArt::new("eab076bc-e4c3-42a1-b701-9bc49bcc3cdd", "Mark Zug"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 302 — Kyren Archive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_ARCHIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e65a06a-e7af-422a-9481-446731009935"),
    "Kyren Archive",
    crate::card::CardArt::new("5e65a06a-e7af-422a-9481-446731009935", "Roger Raupp"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 303 — Kyren Toy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYREN_TOY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a8318bb-bc3c-45e9-bd57-60ae72b6f8b0"),
    "Kyren Toy",
    crate::card::CardArt::new("7a8318bb-bc3c-45e9-bd57-60ae72b6f8b0", "Arnie Swekel"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 304 — Magistrate's Scepter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGISTRATE_S_SCEPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4785ed7-c948-4ad2-b24d-2f45806d9fcc"),
    "Magistrate's Scepter",
    crate::card::CardArt::new("d4785ed7-c948-4ad2-b24d-2f45806d9fcc", "Adam Rex"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 305 — Mercadian Atlas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIAN_ATLAS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00ad3531-399c-4897-b0ee-ad2a26445a17"),
    "Mercadian Atlas",
    crate::card::CardArt::new("00ad3531-399c-4897-b0ee-ad2a26445a17", "Dan Frazier"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 306 — Mercadian Lift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIAN_LIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("395a1a8a-785f-442b-8e95-8b4ca44af2a3"),
    "Mercadian Lift",
    crate::card::CardArt::new("395a1a8a-785f-442b-8e95-8b4ca44af2a3", "Gary Ruddell"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 307 — Monkey Cage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONKEY_CAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07f6be53-7a20-4e6b-a6ce-11cba06af8cb"),
    "Monkey Cage",
    crate::card::CardArt::new("07f6be53-7a20-4e6b-a6ce-11cba06af8cb", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 308 — Panacea
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANACEA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89414770-2a19-4baf-9b18-76104b7b0b9a"),
    "Panacea",
    crate::card::CardArt::new("89414770-2a19-4baf-9b18-76104b7b0b9a", "Donato Giancola"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 309 — Power Matrix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POWER_MATRIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a578599c-7d90-4881-b59a-9cf64b90d917"),
    "Power Matrix",
    crate::card::CardArt::new("a578599c-7d90-4881-b59a-9cf64b90d917", "Alan Pollack"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 310 — Puffer Extract
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUFFER_EXTRACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("83093cdf-0b12-419c-a748-21acf166e195"),
    "Puffer Extract",
    crate::card::CardArt::new("83093cdf-0b12-419c-a748-21acf166e195", "Heather Hudson"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 311 — Rishadan Pawnshop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISHADAN_PAWNSHOP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c5fc9fc-a0f9-4f56-8368-2d7e1fec5ba0"),
    "Rishadan Pawnshop",
    crate::card::CardArt::new("2c5fc9fc-a0f9-4f56-8368-2d7e1fec5ba0", "Joel Biske"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 312 — Skull of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKULL_OF_RAMOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f071957c-9bea-4d00-9ffd-30f98d57b8d2"),
    "Skull of Ramos",
    crate::card::CardArt::new("f071957c-9bea-4d00-9ffd-30f98d57b8d2", "David Martin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 313 — Tooth of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOOTH_OF_RAMOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a3b999d-8e63-4647-a921-15e169022096"),
    "Tooth of Ramos",
    crate::card::CardArt::new("9a3b999d-8e63-4647-a921-15e169022096", "David Martin"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 314 — Toymaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOYMAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76f3992a-553c-4032-b144-55aad2f909f1"),
    "Toymaker",
    crate::card::CardArt::new("76f3992a-553c-4032-b144-55aad2f909f1", "Frank Kelly Freas"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 315 — Worry Beads
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORRY_BEADS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("400edfe9-9efa-43f9-b713-13ad4eae2fa4"),
    "Worry Beads",
    crate::card::CardArt::new("400edfe9-9efa-43f9-b713-13ad4eae2fa4", "rk post"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 316 — Dust Bowl
pub(in crate::card::sets) static DUST_BOWL: CardRecord = CardRecord::new_with_legacy_id(
    280,
    "Dust Bowl",
    CardArt::new("75b03c30-c2b8-4207-b675-26c59c40a7e5", "Ben Thompson"),
    CardSet::MercadianMasques,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{3}, {T}, Sacrifice a land: Destroy target nonbasic land.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
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
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// MMQ 317 — Fountain of Cho
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUNTAIN_OF_CHO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41f352c3-4b63-4174-b2b4-6c19fb8c06ff"),
    "Fountain of Cho",
    crate::card::CardArt::new("41f352c3-4b63-4174-b2b4-6c19fb8c06ff", "Scott Hampton"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 318 — Henge of Ramos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HENGE_OF_RAMOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0582b42f-5ae5-4be2-ba2d-ed62b3cb20c5"),
    "Henge of Ramos",
    crate::card::CardArt::new(
        "0582b42f-5ae5-4be2-ba2d-ed62b3cb20c5",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 319 — Hickory Woodlot
pub(in crate::card::sets) static HICKORY_WOODLOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af7aafb7-6870-4d09-a191-70786766c459"),
    "Hickory Woodlot",
    CardArt::new("af7aafb7-6870-4d09-a191-70786766c459", "Sean McConnell"),
    CardSet::MercadianMasques,
    // Four green mana out of one land, spread over two turns and paid for
    // with the turn it enters tapped. What it is really buying is a fast
    // start, and it is gone by the time the game is long.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped with two depletion counters on it.",
            // One clause about the way it arrives, so one replacement with
            // two parts rather than two abilities.
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("depletion"),
                        amount: 2,
                    },
                ),
            ]),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a depletion counter from this land: Add {G}{G}. If there are no \
             depletion counters on this land, sacrifice it.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("depletion"),
                    amount: 1,
                },
            ],
            // The sacrifice is checked after the counter is removed, so the
            // second activation is the last one: the land pays out twice and
            // then goes away.
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Green)
                    .with_amount(2)
                    .sacrificing_source_when_out_of(CounterKind::named("depletion")),
            ),
        ),
    ]),
);

// MMQ 320 — High Market
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_MARKET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4c58683-65a6-4df9-8952-458e397b1374"),
    "High Market",
    crate::card::CardArt::new("f4c58683-65a6-4df9-8952-458e397b1374", "Carl Critchlow"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 321 — Mercadian Bazaar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCADIAN_BAZAAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f787cb6-78cb-4baa-a9cf-cee8b7d8d6b1"),
    "Mercadian Bazaar",
    crate::card::CardArt::new("6f787cb6-78cb-4baa-a9cf-cee8b7d8d6b1", "Terese Nielsen"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 322 — Peat Bog
pub(in crate::card::sets) static PEAT_BOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcc9d1e0-c8f4-4bac-90d4-8167f7a1515a"),
    "Peat Bog",
    CardArt::new("bcc9d1e0-c8f4-4bac-90d4-8167f7a1515a", "Val Mayerik"),
    CardSet::MercadianMasques,
    // Four black mana out of one land across two turns, then nothing. The
    // black half of the Masques depletion cycle.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped with two depletion counters on it.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("depletion"),
                        amount: 2,
                    },
                ),
            ]),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a depletion counter from this land: Add {B}{B}. If there are no depletion counters \
             on this land, sacrifice it.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("depletion"),
                    amount: 1,
                },
            ],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Black)
                    .with_amount(2)
                    .sacrificing_source_when_out_of(CounterKind::named("depletion")),
            ),
        ),
    ]),
);

// MMQ 323 — Remote Farm
pub(in crate::card::sets) static REMOTE_FARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("115cab84-60d7-4bf2-9beb-b4ed7b5ceaf4"),
    "Remote Farm",
    CardArt::new("115cab84-60d7-4bf2-9beb-b4ed7b5ceaf4", "Rob Alexander"),
    CardSet::MercadianMasques,
    // The white member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped with two depletion counters on it.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("depletion"),
                        amount: 2,
                    },
                ),
            ]),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a depletion counter from this land: Add {W}{W}. If there are no depletion counters \
             on this land, sacrifice it.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("depletion"),
                    amount: 1,
                },
            ],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::White)
                    .with_amount(2)
                    .sacrificing_source_when_out_of(CounterKind::named("depletion")),
            ),
        ),
    ]),
);

// MMQ 324 — Rishadan Port
pub(in crate::card::sets) static RISHADAN_PORT: CardRecord = CardRecord::new_with_legacy_id(
    281,
    "Rishadan Port",
    CardArt::new("477a1f53-5cdf-4b45-b584-2e36b31a3fdb", "Jerry Tiritilli"),
    CardSet::MercadianMasques,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{1}, {T}: Tap target land.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
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
    PrintingAnchor::scryfall("c315c72c-3e2f-4aff-b7d7-2f709ccec332"),
    "Rushwood Grove",
    crate::card::CardArt::new("c315c72c-3e2f-4aff-b7d7-2f709ccec332", "George Pratt"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 326 — Sandstone Needle
pub(in crate::card::sets) static SANDSTONE_NEEDLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82bc7c6b-2e3d-42d1-b2bb-b37b6f34d33b"),
    "Sandstone Needle",
    CardArt::new("82bc7c6b-2e3d-42d1-b2bb-b37b6f34d33b", "Alan Rabinowitz"),
    CardSet::MercadianMasques,
    // The red member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped with two depletion counters on it.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("depletion"),
                        amount: 2,
                    },
                ),
            ]),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a depletion counter from this land: Add {R}{R}. If there are no depletion counters \
             on this land, sacrifice it.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("depletion"),
                    amount: 1,
                },
            ],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Red)
                    .with_amount(2)
                    .sacrificing_source_when_out_of(CounterKind::named("depletion")),
            ),
        ),
    ]),
);

// MMQ 327 — Saprazzan Cove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPRAZZAN_COVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52a69122-19c0-47ec-8bea-478511ba88e6"),
    "Saprazzan Cove",
    crate::card::CardArt::new("52a69122-19c0-47ec-8bea-478511ba88e6", "Rebecca Guay"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 328 — Saprazzan Skerry
pub(in crate::card::sets) static SAPRAZZAN_SKERRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("006871fd-2641-42cb-a2ac-a33d05fc5a35"),
    "Saprazzan Skerry",
    CardArt::new("006871fd-2641-42cb-a2ac-a33d05fc5a35", "Pat Lewis"),
    CardSet::MercadianMasques,
    // The blue member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped with two depletion counters on it.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("depletion"),
                        amount: 2,
                    },
                ),
            ]),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a depletion counter from this land: Add {U}{U}. If there are no depletion counters \
             on this land, sacrifice it.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("depletion"),
                    amount: 1,
                },
            ],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Blue)
                    .with_amount(2)
                    .sacrificing_source_when_out_of(CounterKind::named("depletion")),
            ),
        ),
    ]),
);

// MMQ 329 — Subterranean Hangar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBTERRANEAN_HANGAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edc199d1-970b-489f-b713-8285151f16ae"),
    "Subterranean Hangar",
    crate::card::CardArt::new("edc199d1-970b-489f-b713-8285151f16ae", "Matt Cavotta"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 330 — Tower of the Magistrate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOWER_OF_THE_MAGISTRATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee0481db-15ae-46b4-89a3-01c95a9626c7"),
    "Tower of the Magistrate",
    crate::card::CardArt::new("ee0481db-15ae-46b4-89a3-01c95a9626c7", "Thomas Gianni"),
    crate::card::CardSet::MercadianMasques,
    crate::card::CardRules::unsupported(),
);

// MMQ 331 — Plains (reprint)

// MMQ 332 — Plains (alternate printing)

// MMQ 333 — Plains (alternate printing)

// MMQ 334 — Plains (alternate printing)

// MMQ 335 — Island (reprint)

// MMQ 336 — Island (alternate printing)

// MMQ 337 — Island (alternate printing)

// MMQ 338 — Island (alternate printing)

// MMQ 339 — Swamp (reprint)

// MMQ 340 — Swamp (alternate printing)

// MMQ 341 — Swamp (alternate printing)

// MMQ 342 — Swamp (alternate printing)

// MMQ 343 — Mountain (reprint)

// MMQ 344 — Mountain (alternate printing)

// MMQ 345 — Mountain (alternate printing)

// MMQ 346 — Mountain (alternate printing)

// MMQ 347 — Forest (reprint)

// MMQ 348 — Forest (alternate printing)

// MMQ 349 — Forest (alternate printing)

// MMQ 350 — Forest (alternate printing)

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
    &WALL_OF_DISTORTION,
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
    &SQUALL,
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
    PrintingRecord::reprint(&catalog_mir::AFTERLIFE), // MMQ 1
    PrintingRecord::reprint(&catalog_lea::DISENCHANT), // MMQ 18
    PrintingRecord::reprint(&catalog_vis::RIGHTEOUS_AURA), // MMQ 45
    PrintingRecord::reprint(&catalog_ice::BRAINSTORM), // MMQ 61
    PrintingRecord::reprint(&catalog_lea::COUNTERSPELL), // MMQ 69
    PrintingRecord::reprint(&catalog_atq::ENERGY_FLUX), // MMQ 78
    PrintingRecord::reprint(&catalog_all::FALSE_DEMISE), // MMQ 80
    PrintingRecord::reprint(&catalog_wth::TIMID_DRAKE), // MMQ 111
    PrintingRecord::reprint(&catalog_lea::DARK_RITUAL), // MMQ 129
    PrintingRecord::reprint(&catalog_tmp::RAIN_OF_TEARS), // MMQ 155
    PrintingRecord::reprint(&catalog_roe::VENDETTA),  // MMQ 170
    PrintingRecord::reprint(&catalog_m13::ARMS_DEALER), // MMQ 172
    PrintingRecord::reprint(&catalog_gtc::CINDER_ELEMENTAL), // MMQ 183
    PrintingRecord::reprint(&catalog_p02::OGRE_TASKMASTER), // MMQ 206
    PrintingRecord::reprint(&catalog_lea::STONE_RAIN), // MMQ 215
    PrintingRecord::reprint(&catalog_vis::TREMOR),    // MMQ 220
    PrintingRecord::reprint(&catalog_ice::WORD_OF_BLASTING), // MMQ 228
    PrintingRecord::reprint(&catalog_all::DEADLY_INSECT), // MMQ 238
    PrintingRecord::reprint(&catalog_arn::DESERT_TWISTER), // MMQ 243
    PrintingRecord::reprint(&catalog_vis::GIANT_CATERPILLAR), // MMQ 249
    PrintingRecord::reprint(&catalog_lea::LURE),      // MMQ 258
    PrintingRecord::reprint(&catalog_m13::REVIVE),    // MMQ 262
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY), // MMQ 280
    PrintingRecord::reprint(&catalog_ice::VENOMOUS_BREATH), // MMQ 281
    PrintingRecord::reprint(&catalog_lea::PLAINS),    // MMQ 331
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1), // MMQ 332
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2), // MMQ 333
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3), // MMQ 334
    PrintingRecord::reprint(&catalog_lea::ISLAND),    // MMQ 335
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1), // MMQ 336
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2), // MMQ 337
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3), // MMQ 338
    PrintingRecord::reprint(&catalog_lea::SWAMP),     // MMQ 339
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1), // MMQ 340
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2), // MMQ 341
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3), // MMQ 342
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),  // MMQ 343
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1), // MMQ 344
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2), // MMQ 345
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3), // MMQ 346
    PrintingRecord::reprint(&catalog_lea::FOREST),    // MMQ 347
    PrintingRecord::alternate(&catalog_lea::FOREST, 1), // MMQ 348
    PrintingRecord::alternate(&catalog_lea::FOREST, 2), // MMQ 349
    PrintingRecord::alternate(&catalog_lea::FOREST, 3), // MMQ 350
];
