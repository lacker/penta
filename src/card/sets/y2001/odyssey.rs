//! Odyssey cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1997::tempest as catalog_tmp;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::sets::y1998::exodus as catalog_exo;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2012::avacyn_restored as catalog_avr;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::sets::y2016::eternal_masters as catalog_ema;
use crate::card::sets::y2019::modern_horizons as catalog_mh1;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef,
    BasicLandType, CardArt, CardNameDef, CardNameSetDef, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, CostDef, CostQuantityDef, DamageEventMatcherDef, DamageKindDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, DiscardSelectionDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, KeywordAbility, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, ObjectSetFilterDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, ScaledValueDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

/// Threshold: seven or more cards in your own graveyard. The count is of
/// cards you own, not of every graveyard on the table.
static YOUR_GRAVEYARD: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Any,
    &[ZoneKind::Graveyard],
    crate::card::PlayerSetDef::Related(PlayerRelation::You),
);

static THRESHOLD: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: YOUR_GRAVEYARD,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 7,
};

// ODY 1 — Aegis of Honor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AEGIS_OF_HONOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9561ffdb-f4dd-4b62-a2c2-933498e3a061"),
    "Aegis of Honor",
    crate::card::CardArt::new("9561ffdb-f4dd-4b62-a2c2-933498e3a061", "Ron Spears"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 2 — Ancestral Tribute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_TRIBUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f28b2af-7891-49eb-a07f-5552b5fe3d15"),
    "Ancestral Tribute",
    crate::card::CardArt::new(
        "3f28b2af-7891-49eb-a07f-5552b5fe3d15",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 3 — Angelic Wall (reprint)

// ODY 4 — Animal Boneyard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANIMAL_BONEYARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3379317-be18-4252-84ad-b7ebb6e557ff"),
    "Animal Boneyard",
    crate::card::CardArt::new(
        "e3379317-be18-4252-84ad-b7ebb6e557ff",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 5 — Auramancer (reprint)

// ODY 6 — Aven Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_ARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d174892-c192-4667-94fb-9f8dbcc6c5eb"),
    "Aven Archer",
    crate::card::CardArt::new("4d174892-c192-4667-94fb-9f8dbcc6c5eb", "Mark Zug"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 7 — Aven Cloudchaser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_CLOUDCHASER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08afe190-368e-4259-9959-00beaccee7ba"),
    "Aven Cloudchaser",
    crate::card::CardArt::new("08afe190-368e-4259-9959-00beaccee7ba", "Justin Sweet"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 8 — Aven Flock
pub(in crate::card::sets) static AVEN_FLOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29866d0b-f9be-4284-9d21-03598ef6ae4f"),
    "Aven Flock",
    CardArt::new(
        "29866d0b-f9be-4284-9d21-03598ef6ae4f",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::Odyssey,
    // Toughness rather than power, so it wins the air by surviving rather
    // than by killing.
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird", "Soldier"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
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

// ODY 9 — Aven Shrine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_SHRINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9890be7f-35af-43b7-b255-25be4ff20dc0"),
    "Aven Shrine",
    crate::card::CardArt::new("9890be7f-35af-43b7-b255-25be4ff20dc0", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 10 — Balancing Act
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALANCING_ACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7d6cb368-43f7-4d06-9cd5-492df5766567"),
    "Balancing Act",
    crate::card::CardArt::new("7d6cb368-43f7-4d06-9cd5-492df5766567", "Scott M. Fischer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 11 — Beloved Chaplain
pub(in crate::card::sets) static BELOVED_CHAPLAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("decec62b-7ac4-4097-9215-5db18db2dec6"),
    "Beloved Chaplain",
    CardArt::new("decec62b-7ac4-4097-9215-5db18db2dec6", "Darrell Riche"),
    CardSet::Odyssey,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::keyword(
            "Protection from creatures",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Creature)),
        ),
    ),
);

// ODY 12 — Blessed Orator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_ORATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5654575e-0849-4e7f-98f2-0074ac8e0faa"),
    "Blessed Orator",
    crate::card::CardArt::new("5654575e-0849-4e7f-98f2-0074ac8e0faa", "Terese Nielsen"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 13 — Cantivore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANTIVORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5243fc3-176b-44a3-9f1a-ab069a08757a"),
    "Cantivore",
    crate::card::CardArt::new("b5243fc3-176b-44a3-9f1a-ab069a08757a", "Daren Bader"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 14 — Cease-Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEASE_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1646c998-aee5-497f-bd75-24ced1dabef9"),
    "Cease-Fire",
    crate::card::CardArt::new("1646c998-aee5-497f-bd75-24ced1dabef9", "Darrell Riche"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 15 — Confessor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONFESSOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("984c0bdb-e9e2-435e-aa68-2dbad52c3dbd"),
    "Confessor",
    crate::card::CardArt::new("984c0bdb-e9e2-435e-aa68-2dbad52c3dbd", "John Avon"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 16 — Dedicated Martyr
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEDICATED_MARTYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1f95b64-858b-4344-84a2-3afa5c7b9ee9"),
    "Dedicated Martyr",
    crate::card::CardArt::new("d1f95b64-858b-4344-84a2-3afa5c7b9ee9", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 17 — Delaying Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELAYING_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14e8195a-d0e4-4b11-b413-765ed1cae834"),
    "Delaying Shield",
    crate::card::CardArt::new("14e8195a-d0e4-4b11-b413-765ed1cae834", "Luca Zontini"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 18 — Devoted Caretaker
pub(in crate::card::sets) static DEVOTED_CARETAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d400b63-de3e-4805-a51d-c4c0dfbb7033"),
    "Devoted Caretaker",
    CardArt::new("8d400b63-de3e-4805-a51d-c4c0dfbb7033", "Clyde Caldwell"),
    CardSet::Odyssey,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{W}, {T}: Target permanent you control gains protection from instant spells and from sorcery spells until end of turn.",
            &[
                CostDef::Mana(mana_cost!("{W}")),
                CostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&AbilityDef::keyword(
                    "Protection from instant spells and from sorcery spells",
                    KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    ])),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ODY 19 — Divine Sacrament
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVINE_SACRAMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f69ecef5-4f1a-4463-8908-7c0ef955b02d"),
    "Divine Sacrament",
    crate::card::CardArt::new("f69ecef5-4f1a-4463-8908-7c0ef955b02d", "Ray Lago"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 20 — Dogged Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOGGED_HUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b50d4337-9b71-429b-b6bc-2d70299a6e76"),
    "Dogged Hunter",
    crate::card::CardArt::new("b50d4337-9b71-429b-b6bc-2d70299a6e76", "rk post"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 21 — Earnest Fellowship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARNEST_FELLOWSHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66392169-5c6f-46bf-b0df-5670e40aecd9"),
    "Earnest Fellowship",
    crate::card::CardArt::new("66392169-5c6f-46bf-b0df-5670e40aecd9", "Heather Hudson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 22 — Embolden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBOLDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36848fb0-4070-40cf-b24a-2e8f47c5ebc3"),
    "Embolden",
    crate::card::CardArt::new("36848fb0-4070-40cf-b24a-2e8f47c5ebc3", "Rebecca Guay"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 23 — Gallantry (reprint)

// ODY 24 — Graceful Antelope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRACEFUL_ANTELOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("948f0d9d-4e05-48b4-8652-1f8f41d35563"),
    "Graceful Antelope",
    crate::card::CardArt::new("948f0d9d-4e05-48b4-8652-1f8f41d35563", "Heather Hudson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 25 — Hallowed Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALLOWED_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("254d5bf0-f985-4919-a142-d4578ae9a38e"),
    "Hallowed Healer",
    crate::card::CardArt::new("254d5bf0-f985-4919-a142-d4578ae9a38e", "Ben Thompson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 26 — Karmic Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARMIC_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2ffb8e7-7ae3-4846-b3da-ca6b4598eb7c"),
    "Karmic Justice",
    crate::card::CardArt::new("c2ffb8e7-7ae3-4846-b3da-ca6b4598eb7c", "Ray Lago"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 27 — Kirtar's Desire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KIRTAR_S_DESIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f1e36fe-ecbe-46aa-9ee8-2ba4daba7d31"),
    "Kirtar's Desire",
    crate::card::CardArt::new("7f1e36fe-ecbe-46aa-9ee8-2ba4daba7d31", "Pete Venters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 28 — Kirtar's Wrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KIRTAR_S_WRATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5a0c4e6-d50e-42e8-b062-8f6ef5950ab7"),
    "Kirtar's Wrath",
    crate::card::CardArt::new("b5a0c4e6-d50e-42e8-b062-8f6ef5950ab7", "Kev Walker"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 29 — Lieutenant Kirtar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIEUTENANT_KIRTAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e518750f-492d-4575-b4ee-c80c1f8e0a58"),
    "Lieutenant Kirtar",
    crate::card::CardArt::new("e518750f-492d-4575-b4ee-c80c1f8e0a58", "Paolo Parente"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 30 — Life Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIFE_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f8eee50-efd2-45fd-b815-051167ef4541"),
    "Life Burst",
    crate::card::CardArt::new("7f8eee50-efd2-45fd-b815-051167ef4541", "John Avon"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 31 — Luminous Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMINOUS_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5becc51-4df8-4f67-a029-42a3da598a26"),
    "Luminous Guardian",
    crate::card::CardArt::new("a5becc51-4df8-4f67-a029-42a3da598a26", "Terese Nielsen"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 32 — Master Apothecary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_APOTHECARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9ff6624-ed0e-43d0-9519-112979b165f5"),
    "Master Apothecary",
    crate::card::CardArt::new("f9ff6624-ed0e-43d0-9519-112979b165f5", "Terese Nielsen"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 33 — Mystic Crusader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_CRUSADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27ce1baf-8a89-4884-b0f7-19311bfc8c4c"),
    "Mystic Crusader",
    crate::card::CardArt::new("27ce1baf-8a89-4884-b0f7-19311bfc8c4c", "Kev Walker"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 34 — Mystic Penitent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_PENITENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb37f08b-019e-4e6b-8b15-b2971a3b5ebb"),
    "Mystic Penitent",
    crate::card::CardArt::new("fb37f08b-019e-4e6b-8b15-b2971a3b5ebb", "Larry Elmore"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 35 — Mystic Visionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48ed1b3d-0f7e-46ff-ada7-ebbd416ecf5b"),
    "Mystic Visionary",
    crate::card::CardArt::new("48ed1b3d-0f7e-46ff-ada7-ebbd416ecf5b", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 36 — Mystic Zealot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_ZEALOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ba21062-5d13-47b9-bea1-ded8530a9aeb"),
    "Mystic Zealot",
    crate::card::CardArt::new("1ba21062-5d13-47b9-bea1-ded8530a9aeb", "Paolo Parente"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 37 — Nomad Decoy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOMAD_DECOY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ffef7c6-e05b-4bef-84aa-8df10be110af"),
    "Nomad Decoy",
    crate::card::CardArt::new("9ffef7c6-e05b-4bef-84aa-8df10be110af", "Pete Venters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 38 — Patrol Hound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATROL_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e216a539-152b-4a83-98ef-1996182a5714"),
    "Patrol Hound",
    crate::card::CardArt::new("e216a539-152b-4a83-98ef-1996182a5714", "Daren Bader"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 39 — Pianna, Nomad Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIANNA_NOMAD_CAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0812edfa-3a53-48e8-ba35-6922a1f3aa90"),
    "Pianna, Nomad Captain",
    crate::card::CardArt::new(
        "0812edfa-3a53-48e8-ba35-6922a1f3aa90",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 40 — Pilgrim of Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILGRIM_OF_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6103a3a7-78ce-4a66-bd82-434e0bd9dea4"),
    "Pilgrim of Justice",
    crate::card::CardArt::new("6103a3a7-78ce-4a66-bd82-434e0bd9dea4", "Dany Orizio"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 41 — Pilgrim of Virtue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILGRIM_OF_VIRTUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be58f0dd-3856-4960-83de-06eed78ed703"),
    "Pilgrim of Virtue",
    crate::card::CardArt::new(
        "be58f0dd-3856-4960-83de-06eed78ed703",
        "Massimiliano Frezzato",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 42 — Ray of Distortion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAY_OF_DISTORTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9940f593-021f-44cd-9725-0779a2808b6c"),
    "Ray of Distortion",
    crate::card::CardArt::new("9940f593-021f-44cd-9725-0779a2808b6c", "Carl Critchlow"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 43 — Resilient Wanderer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESILIENT_WANDERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e5a61de-ee4e-4097-b342-b5f2c976e16e"),
    "Resilient Wanderer",
    crate::card::CardArt::new("5e5a61de-ee4e-4097-b342-b5f2c976e16e", "Clyde Caldwell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 44 — Sacred Rites
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_RITES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d7f1dd0-ee1b-484a-949d-dde4c3340a09"),
    "Sacred Rites",
    crate::card::CardArt::new("3d7f1dd0-ee1b-484a-949d-dde4c3340a09", "Bradley Williams"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 45 — Second Thoughts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SECOND_THOUGHTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b09586de-4853-4e51-a4ac-70eabb37eef4"),
    "Second Thoughts",
    crate::card::CardArt::new("b09586de-4853-4e51-a4ac-70eabb37eef4", "Ray Lago"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 46 — Shelter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHELTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c726ab3-1692-4e14-ab21-664a1679bc53"),
    "Shelter",
    crate::card::CardArt::new(
        "9c726ab3-1692-4e14-ab21-664a1679bc53",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 47 — Soulcatcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULCATCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14d24d2f-699b-46d8-9353-45e6a67f99d2"),
    "Soulcatcher",
    crate::card::CardArt::new("14d24d2f-699b-46d8-9353-45e6a67f99d2", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 48 — Sphere of Duty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPHERE_OF_DUTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4bdcd0de-6466-488c-a882-bad80c86504d"),
    "Sphere of Duty",
    crate::card::CardArt::new(
        "4bdcd0de-6466-488c-a882-bad80c86504d",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 49 — Sphere of Grace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPHERE_OF_GRACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5b49d82-6d11-4294-a367-0b5b45c05e44"),
    "Sphere of Grace",
    crate::card::CardArt::new(
        "d5b49d82-6d11-4294-a367-0b5b45c05e44",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 50 — Sphere of Law
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPHERE_OF_LAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3451ddf-efec-474b-884c-64288e5552d2"),
    "Sphere of Law",
    crate::card::CardArt::new(
        "f3451ddf-efec-474b-884c-64288e5552d2",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 51 — Sphere of Reason
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPHERE_OF_REASON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76ef2016-b3c8-4615-a056-3000bcccb68e"),
    "Sphere of Reason",
    crate::card::CardArt::new(
        "76ef2016-b3c8-4615-a056-3000bcccb68e",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 52 — Sphere of Truth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPHERE_OF_TRUTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7c3de9f5-a05c-4341-a2c2-a1a475d400fd"),
    "Sphere of Truth",
    crate::card::CardArt::new(
        "7c3de9f5-a05c-4341-a2c2-a1a475d400fd",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 53 — Spiritualize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRITUALIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6780ded-86df-419d-b63b-6b67d2960b28"),
    "Spiritualize",
    crate::card::CardArt::new(
        "f6780ded-86df-419d-b63b-6b67d2960b28",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 54 — Tattoo Ward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TATTOO_WARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89920480-9b39-47dd-988a-c040067da7fb"),
    "Tattoo Ward",
    crate::card::CardArt::new("89920480-9b39-47dd-988a-c040067da7fb", "Ben Thompson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 55 — Testament of Faith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TESTAMENT_OF_FAITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f399b371-fed0-42b8-8f95-5d76fdbe193d"),
    "Testament of Faith",
    crate::card::CardArt::new("f399b371-fed0-42b8-8f95-5d76fdbe193d", "Roger Raupp"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 56 — Tireless Tribe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIRELESS_TRIBE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d23e47a-21d5-4d7e-8aa0-3b3064da5967"),
    "Tireless Tribe",
    crate::card::CardArt::new("1d23e47a-21d5-4d7e-8aa0-3b3064da5967", "Carl Critchlow"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 57 — Wayward Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAYWARD_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fb726e8-162d-4143-9778-32476c0e1ab1"),
    "Wayward Angel",
    crate::card::CardArt::new("1fb726e8-162d-4143-9778-32476c0e1ab1", "Mark Tedin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 58 — Aboshan, Cephalid Emperor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABOSHAN_CEPHALID_EMPEROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82db4a41-03e8-4f0c-946c-a98fc5c9f7c8"),
    "Aboshan, Cephalid Emperor",
    crate::card::CardArt::new(
        "82db4a41-03e8-4f0c-946c-a98fc5c9f7c8",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 59 — Aboshan's Desire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABOSHAN_S_DESIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("137b701e-6140-463e-84b5-58d8a728df40"),
    "Aboshan's Desire",
    crate::card::CardArt::new("137b701e-6140-463e-84b5-58d8a728df40", "Ciruelo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 60 — Aether Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d09ea4a-0d3a-40b8-be0f-a31693099c4f"),
    "Aether Burst",
    crate::card::CardArt::new("3d09ea4a-0d3a-40b8-be0f-a31693099c4f", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 61 — Amugaba
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMUGABA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d73d1e7-79be-4b28-a480-b65b4f34f755"),
    "Amugaba",
    crate::card::CardArt::new("8d73d1e7-79be-4b28-a480-b65b4f34f755", "Heather Hudson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 62 — Aura Graft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_GRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7553877-7df2-42f7-98a3-1a07f66a4905"),
    "Aura Graft",
    crate::card::CardArt::new("c7553877-7df2-42f7-98a3-1a07f66a4905", "Ray Lago"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 63 — Aven Fisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_FISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b27130d-2296-4076-9829-15ab63081896"),
    "Aven Fisher",
    crate::card::CardArt::new(
        "5b27130d-2296-4076-9829-15ab63081896",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 64 — Aven Smokeweaver
pub(in crate::card::sets) static AVEN_SMOKEWEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57f5d024-f137-4ea8-be02-7de46dee95fd"),
    "Aven Smokeweaver",
    CardArt::new("57f5d024-f137-4ea8-be02-7de46dee95fd", "Kev Walker"),
    CardSet::Odyssey,
    // A 2/3 flier that red cannot answer at all: it blocks their fliers and
    // survives their removal.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird", "Soldier"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// ODY 65 — Aven Windreader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_WINDREADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9fd2f2cb-a0f1-4844-a692-bcaf1899d41c"),
    "Aven Windreader",
    crate::card::CardArt::new("9fd2f2cb-a0f1-4844-a692-bcaf1899d41c", "Greg Hildebrandt"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 66 — Balshan Beguiler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALSHAN_BEGUILER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d977da2-4024-4c7b-b557-e89564f8d465"),
    "Balshan Beguiler",
    crate::card::CardArt::new("5d977da2-4024-4c7b-b557-e89564f8d465", "Ray Lago"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 67 — Balshan Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALSHAN_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("529c5440-e31f-40be-9e66-699d17049fb4"),
    "Balshan Griffin",
    crate::card::CardArt::new("529c5440-e31f-40be-9e66-699d17049fb4", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 68 — Bamboozle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAMBOOZLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44012bb8-17b7-4b50-a796-662ef09bfc29"),
    "Bamboozle",
    crate::card::CardArt::new("44012bb8-17b7-4b50-a796-662ef09bfc29", "Alan Pollack"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 69 — Battle of Wits (reprint)

// ODY 70 — Careful Study
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAREFUL_STUDY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dea15b53-2940-40e7-8d48-8ec11341da83"),
    "Careful Study",
    crate::card::CardArt::new("dea15b53-2940-40e7-8d48-8ec11341da83", "Scott M. Fischer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 71 — Cephalid Broker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_BROKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1d2cfe5-b905-4d8a-89e4-474619e2796c"),
    "Cephalid Broker",
    crate::card::CardArt::new("b1d2cfe5-b905-4d8a-89e4-474619e2796c", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 72 — Cephalid Looter (alternate printing)

// ODY 72† — Cephalid Looter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_LOOTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ae87f9d-d6eb-4178-a99d-76a9dffacf28"),
    "Cephalid Looter",
    crate::card::CardArt::new("1ae87f9d-d6eb-4178-a99d-76a9dffacf28", "Keith Garletts"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 73 — Cephalid Retainer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_RETAINER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("78480527-cfd6-4065-b702-82de4694f9bb"),
    "Cephalid Retainer",
    crate::card::CardArt::new("78480527-cfd6-4065-b702-82de4694f9bb", "Tony Szczudlo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 74 — Cephalid Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6efdf190-970d-4751-b214-cd962f7f2ca8"),
    "Cephalid Scout",
    crate::card::CardArt::new("6efdf190-970d-4751-b214-cd962f7f2ca8", "Alan Pollack"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 75 — Cephalid Shrine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_SHRINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("137e0ac1-3bf2-4583-b44f-c9fe8d7e8882"),
    "Cephalid Shrine",
    crate::card::CardArt::new("137e0ac1-3bf2-4583-b44f-c9fe8d7e8882", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 76 — Chamber of Manipulation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMBER_OF_MANIPULATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50496069-0f55-4dfe-8b1e-7fc4f649f2f1"),
    "Chamber of Manipulation",
    crate::card::CardArt::new("50496069-0f55-4dfe-8b1e-7fc4f649f2f1", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 77 — Cognivore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COGNIVORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7de76ff6-d065-4db1-b28d-4a4ccb1cc0fa"),
    "Cognivore",
    crate::card::CardArt::new("7de76ff6-d065-4db1-b28d-4a4ccb1cc0fa", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 78 — Concentrate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONCENTRATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45f2878e-1c1c-4dd3-9687-7bb216d557c7"),
    "Concentrate",
    crate::card::CardArt::new(
        "45f2878e-1c1c-4dd3-9687-7bb216d557c7",
        "Glen Angus & Arnie Swekel",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 79 — Cultural Exchange
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CULTURAL_EXCHANGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4f38d07-829c-4311-a61b-2785cf2266ef"),
    "Cultural Exchange",
    crate::card::CardArt::new("d4f38d07-829c-4311-a61b-2785cf2266ef", "Daren Bader"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 80 — Deluge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELUGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22e64f32-0436-4f07-b2b5-a622e3f59d65"),
    "Deluge",
    crate::card::CardArt::new("22e64f32-0436-4f07-b2b5-a622e3f59d65", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 81 — Dematerialize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMATERIALIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04217c17-7c29-4b02-b9b6-bfa1df50d4bc"),
    "Dematerialize",
    crate::card::CardArt::new("04217c17-7c29-4b02-b9b6-bfa1df50d4bc", "Mike Ploog"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 82 — Divert
pub(in crate::card::sets) static DIVERT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8eb6e9f8-a508-451e-8a72-5f9834ba5352"),
    "Divert",
    crate::card::CardArt::new(
        "8eb6e9f8-a508-451e-8a72-5f9834ba5352",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Change the target of target spell with a single target unless that spell's controller pays {2}.",
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
        EffectDef::PayOr(PayOrDef::unless(
            EffectPaymentDef::mana(
                PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                mana_cost!("{2}"),
            ),
            &EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                chooser: PlayerRefDef::EffectController,
                change: crate::card::StackTargetChangeDef::ChooseNew {
                    optional: false,
                    restriction: None,
                },
            }),
        )),
    )),
);

// ODY 83 — Dreamwinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAMWINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ae3a000-478c-4185-a8c0-82f29942497b"),
    "Dreamwinder",
    crate::card::CardArt::new("2ae3a000-478c-4185-a8c0-82f29942497b", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 84 — Escape Artist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESCAPE_ARTIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f5d0e3f-b8f1-472a-857b-5464174d243b"),
    "Escape Artist",
    crate::card::CardArt::new("7f5d0e3f-b8f1-472a-857b-5464174d243b", "Scott M. Fischer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 85 — Extract
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTRACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97f99a3d-d811-4666-aac8-5957068157dc"),
    "Extract",
    crate::card::CardArt::new("97f99a3d-d811-4666-aac8-5957068157dc", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 86 — Fervent Denial
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FERVENT_DENIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed13fdb4-f28a-43c9-a69f-bab227806c39"),
    "Fervent Denial",
    crate::card::CardArt::new("ed13fdb4-f28a-43c9-a69f-bab227806c39", "Scott M. Fischer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 87 — Immobilizing Ink
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMMOBILIZING_INK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee6e2d96-dd9e-45f6-b412-78562f2ada40"),
    "Immobilizing Ink",
    crate::card::CardArt::new("ee6e2d96-dd9e-45f6-b412-78562f2ada40", "Darrell Riche"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 88 — Laquatus's Creativity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAQUATUS_S_CREATIVITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dcd48c28-bfa3-4eca-8e1a-f72d785b2ab9"),
    "Laquatus's Creativity",
    crate::card::CardArt::new("dcd48c28-bfa3-4eca-8e1a-f72d785b2ab9", "Don Hazeltine"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 89 — Patron Wizard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATRON_WIZARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10b863b8-7780-4bbe-a7f8-46bfdcb34a2b"),
    "Patron Wizard",
    crate::card::CardArt::new("10b863b8-7780-4bbe-a7f8-46bfdcb34a2b", "Donato Giancola"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 90 — Pedantic Learning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEDANTIC_LEARNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67445332-0b56-4fec-8dcd-bb9a87194db5"),
    "Pedantic Learning",
    crate::card::CardArt::new("67445332-0b56-4fec-8dcd-bb9a87194db5", "Heather Hudson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 91 — Peek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f50843cc-20ac-4746-816e-f2630aa31594"),
    "Peek",
    crate::card::CardArt::new("f50843cc-20ac-4746-816e-f2630aa31594", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 92 — Persuasion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERSUASION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7db5d930-23b4-4726-8f5c-3c690b36f4a4"),
    "Persuasion",
    crate::card::CardArt::new("7db5d930-23b4-4726-8f5c-3c690b36f4a4", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 93 — Phantom Whelp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_WHELP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9216e73b-56ef-48b9-a6a1-b370fb0d97c1"),
    "Phantom Whelp",
    crate::card::CardArt::new("9216e73b-56ef-48b9-a6a1-b370fb0d97c1", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 94 — Predict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PREDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04dc842c-3250-44ed-906c-38f14bf1f0e2"),
    "Predict",
    crate::card::CardArt::new("04dc842c-3250-44ed-906c-38f14bf1f0e2", "Rebecca Guay"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 95 — Psionic Gift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSIONIC_GIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("862fc18f-90d2-430d-aff2-47c1483dee9d"),
    "Psionic Gift",
    crate::card::CardArt::new("862fc18f-90d2-430d-aff2-47c1483dee9d", "Dany Orizio"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 96 — Pulsating Illusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULSATING_ILLUSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3297f28-a15d-43dd-a96e-09701d5f9aed"),
    "Pulsating Illusion",
    crate::card::CardArt::new("d3297f28-a15d-43dd-a96e-09701d5f9aed", "Arnie Swekel"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 97 — Puppeteer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUPPETEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("819d5bd2-02bf-4a22-a656-764425711c29"),
    "Puppeteer",
    crate::card::CardArt::new("819d5bd2-02bf-4a22-a656-764425711c29", "Scott M. Fischer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 98 — Repel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("68ea14a6-539a-47eb-9147-a310be7b63fe"),
    "Repel",
    crate::card::CardArt::new("68ea14a6-539a-47eb-9147-a310be7b63fe", "Terese Nielsen"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 99 — Rites of Refusal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITES_OF_REFUSAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa88f595-1b6f-4af0-bc50-bd07c8be431f"),
    "Rites of Refusal",
    crate::card::CardArt::new("fa88f595-1b6f-4af0-bc50-bd07c8be431f", "Bradley Williams"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 100 — Scrivener (reprint)

// ODY 101 — Shifty Doppelganger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIFTY_DOPPELGANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("538ffe1e-c312-4797-99dd-9bf5f896bdc3"),
    "Shifty Doppelganger",
    crate::card::CardArt::new("538ffe1e-c312-4797-99dd-9bf5f896bdc3", "Greg Staples"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 102 — Standstill
pub(in crate::card::sets) static STANDSTILL: CardRecord = CardRecord::new_with_legacy_id(
    2043,
    "Standstill",
    CardArt::new("3ede3f6f-e642-4fe4-aa37-0f01cdf4d149", "Heather Hudson"),
    CardSet::Odyssey,
    // A deck built to do nothing profits from the stalemate; whoever blinks
    // first hands over three cards.
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_ability(AbilityDef::triggered(
        "When a player casts a spell, sacrifice this enchantment. If you do, each of that player's opponents draws three cards.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Any),
        // Everyone who is not the caster draws, so casting into it is what makes it
        // resolve against you. In a two-player game that is the opponent alone.
        EffectDef::Sequence(&const {
            [
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::Opponent,
                    )),
                    amount: ValueDef::Constant(3),
                },
            ]
        }),
    )),
);

// ODY 103 — Syncopate (reprint)

// ODY 104 — Think Tank
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THINK_TANK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c1c9d34-ea4c-4e89-a7ed-06c4469c1aca"),
    "Think Tank",
    crate::card::CardArt::new("8c1c9d34-ea4c-4e89-a7ed-06c4469c1aca", "Jim Nelson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 105 — Thought Devourer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHT_DEVOURER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba7a96ee-e2d1-4d76-a09e-d6868ddd9282"),
    "Thought Devourer",
    crate::card::CardArt::new("ba7a96ee-e2d1-4d76-a09e-d6868ddd9282", "Jim Nelson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 106 — Thought Eater
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHT_EATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e05f63c-f93d-44b9-98e9-c5e3e3aad6b9"),
    "Thought Eater",
    crate::card::CardArt::new("4e05f63c-f93d-44b9-98e9-c5e3e3aad6b9", "Luca Zontini"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 107 — Thought Nibbler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHT_NIBBLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7284a7fd-cda8-43ac-b119-ad47b33c2ec4"),
    "Thought Nibbler",
    crate::card::CardArt::new("7284a7fd-cda8-43ac-b119-ad47b33c2ec4", "Arnie Swekel"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 108 — Time Stretch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_STRETCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4c3d626-880c-422a-ac09-a79a52847477"),
    "Time Stretch",
    crate::card::CardArt::new("d4c3d626-880c-422a-ac09-a79a52847477", "Paolo Parente"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 109 — Touch of Invisibility
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCH_OF_INVISIBILITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5a0c915-92a9-4eb0-a02a-c1571b00761b"),
    "Touch of Invisibility",
    crate::card::CardArt::new("a5a0c915-92a9-4eb0-a02a-c1571b00761b", "Eric Peterson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 110 — Traumatize (reprint)

// ODY 111 — Treetop Sentinel
pub(in crate::card::sets) static TREETOP_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("606b4e02-f81a-490c-8d2e-cfea7917d6b7"),
    "Treetop Sentinel",
    CardArt::new("606b4e02-f81a-490c-8d2e-cfea7917d6b7", "Carl Critchlow"),
    CardSet::Odyssey,
    // A 2/3 flier green cannot touch: it blocks their ground creatures'
    // pump spells and survives their removal.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird", "Soldier"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Green),
    ]),
);

// ODY 112 — Unifying Theory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNIFYING_THEORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8aa4bf82-65e7-4b0c-9f96-dd84a67dcfb2"),
    "Unifying Theory",
    crate::card::CardArt::new("8aa4bf82-65e7-4b0c-9f96-dd84a67dcfb2", "Ron Spears"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 113 — Upheaval
pub(in crate::card::sets) static UPHEAVAL: CardRecord = CardRecord::new_with_legacy_id(
    290,
    "Upheaval",
    CardArt::new("9e201229-34a6-48c8-a07c-d8aefcf5f8a7", "Kev Walker"),
    CardSet::Odyssey,
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_ability(AbilityDef::spell(
        "Return all permanents to their owners' hands.",
        EffectDef::MoveToZone {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Any,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// ODY 114 — Words of Wisdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WISDOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0199cfb-5e44-40f0-b9cf-71473155eb94"),
    "Words of Wisdom",
    crate::card::CardArt::new("c0199cfb-5e44-40f0-b9cf-71473155eb94", "Eric Peterson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 115 — Afflict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AFFLICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0012621b-c0e1-48d6-99b9-ecca4763d748"),
    "Afflict",
    crate::card::CardArt::new("0012621b-c0e1-48d6-99b9-ecca4763d748", "Roger Raupp"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 116 — Bloodcurdler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODCURDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd1452ff-cec6-4df3-8bdc-bfb7869553a1"),
    "Bloodcurdler",
    crate::card::CardArt::new("dd1452ff-cec6-4df3-8bdc-bfb7869553a1", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 117 — Braids, Cabal Minion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAIDS_CABAL_MINION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4dcdcad5-e4fb-480e-984f-1ac5cdc986b9"),
    "Braids, Cabal Minion",
    crate::card::CardArt::new("4dcdcad5-e4fb-480e-984f-1ac5cdc986b9", "Eric Peterson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 118 — Buried Alive (reprint)

// ODY 119 — Cabal Inquisitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_INQUISITOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0530e09-5206-460e-abd2-e928eca294a5"),
    "Cabal Inquisitor",
    crate::card::CardArt::new(
        "c0530e09-5206-460e-abd2-e928eca294a5",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 120 — Cabal Patriarch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_PATRIARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d2f0da3-a13e-4a45-98dc-6227cf952a5e"),
    "Cabal Patriarch",
    crate::card::CardArt::new("0d2f0da3-a13e-4a45-98dc-6227cf952a5e", "Mark Zug"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 121 — Cabal Shrine
pub(in crate::card::sets) static CABAL_SHRINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd376a52-5dfd-49f3-a520-537cd4527439"),
    "Cabal Shrine",
    crate::card::CardArt::new("dd376a52-5dfd-49f3-a520-537cd4527439", "Ben Thompson"),
    crate::card::CardSet::Odyssey,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a spell, that player discards X cards, where X is the number of cards in all graveyards with the same name as that spell.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Any),
        EffectDef::Discard {
            recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                ObjectRefDef::TriggeringObject,
            )),
            amount: ValueDef::CountObjects(&ObjectSetDef::Matching {
                objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                )),
                object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameEquals(
                    CardNameDef::NameOf(ObjectRefDef::TriggeringObject),
                )),
            }),
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    )),
);

// ODY 122 — Caustic Tar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAUSTIC_TAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3700a61f-76fe-42e6-be93-0ba319b7b543"),
    "Caustic Tar",
    crate::card::CardArt::new("3700a61f-76fe-42e6-be93-0ba319b7b543", "John Avon"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 123 — Childhood Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHILDHOOD_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10c180ea-2d1e-458e-b5d4-e87975eba968"),
    "Childhood Horror",
    crate::card::CardArt::new("10c180ea-2d1e-458e-b5d4-e87975eba968", "Larry Elmore"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 124 — Coffin Purge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COFFIN_PURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("541e06cb-5616-40b4-9d36-89471a795ac8"),
    "Coffin Purge",
    crate::card::CardArt::new("541e06cb-5616-40b4-9d36-89471a795ac8", "Pete Venters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 125 — Crypt Creeper (reprint)

// ODY 126 — Cursed Monstrosity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURSED_MONSTROSITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8de97585-3d24-4e8d-8bf9-edd75ee88443"),
    "Cursed Monstrosity",
    crate::card::CardArt::new("8de97585-3d24-4e8d-8bf9-edd75ee88443", "Jeff Remmer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 127 — Decaying Soil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECAYING_SOIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2bea0ec-52e0-4f45-a445-8805dc6ed596"),
    "Decaying Soil",
    crate::card::CardArt::new("f2bea0ec-52e0-4f45-a445-8805dc6ed596", "Don Hazeltine"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 128 — Decompose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECOMPOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("916fb8a8-f402-4ef6-8a6b-8fc591e04367"),
    "Decompose",
    crate::card::CardArt::new("916fb8a8-f402-4ef6-8a6b-8fc591e04367", "Tony Szczudlo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 129 — Diabolic Tutor (reprint)

// ODY 130 — Dirty Wererat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIRTY_WERERAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97d15534-310f-4f9d-be0d-59b0ed8a5f53"),
    "Dirty Wererat",
    crate::card::CardArt::new("97d15534-310f-4f9d-be0d-59b0ed8a5f53", "Daren Bader"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 131 — Dusk Imp
pub(in crate::card::sets) static DUSK_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f67943c0-304a-4d04-8e26-f45b3ab27a45"),
    "Dusk Imp",
    CardArt::new(
        "f67943c0-304a-4d04-8e26-f45b3ab27a45",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Odyssey,
    // A 2/1 flier for three: it races and it does not block, which is the
    // deal black takes when it buys evasion.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Imp"], 2, 1)
        .with_abilities(&[abilities::flying()]),
);

// ODY 132 — Entomb
pub(in crate::card::sets) static ENTOMB: CardRecord = CardRecord::new_with_legacy_id(
    2113,
    "Entomb",
    CardArt::new("f60a2091-fb97-4f04-911b-fce9b6351044", "Ron Spears"),
    CardSet::Odyssey,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, put that card into your graveyard, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// ODY 133 — Execute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXECUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("333123bc-fb66-4b5a-bf55-045d2906c8c3"),
    "Execute",
    crate::card::CardArt::new("333123bc-fb66-4b5a-bf55-045d2906c8c3", "Gary Ruddell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 134 — Face of Fear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FACE_OF_FEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17542219-1165-4483-9cef-7abecaebb6a2"),
    "Face of Fear",
    crate::card::CardArt::new("17542219-1165-4483-9cef-7abecaebb6a2", "Thomas M. Baxa"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 135 — Famished Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAMISHED_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e413b65a-700d-44eb-a880-5a0118d2ebac"),
    "Famished Ghoul",
    crate::card::CardArt::new("e413b65a-700d-44eb-a880-5a0118d2ebac", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 136 — Filthy Cur
pub(in crate::card::sets) static FILTHY_CUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("634b2b46-b213-4eb0-81d8-0e9dd161b85f"),
    "Filthy Cur",
    CardArt::new("634b2b46-b213-4eb0-81d8-0e9dd161b85f", "Adam Rex"),
    CardSet::Odyssey,
    // Damage to it is damage to you, so blocking with it is never free --
    // and burning it is a way to burn you.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Dog"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, you lose that much life.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Any,
                recipient: DamageRecipientMatcherDef::MatchingObject(ObjectPredicateDef::Source),
            }),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// ODY 137 — Fledgling Imp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEDGLING_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d11770ee-dcf0-4dd4-ab43-b98f1133cec7"),
    "Fledgling Imp",
    crate::card::CardArt::new("d11770ee-dcf0-4dd4-ab43-b98f1133cec7", "John Matson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 138 — Frightcrawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRIGHTCRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d79129e7-6cc4-4060-8b76-3afbd2e0d456"),
    "Frightcrawler",
    crate::card::CardArt::new("d79129e7-6cc4-4060-8b76-3afbd2e0d456", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 139 — Ghastly Demise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHASTLY_DEMISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9d2bfa3-0499-43ea-a76d-b12fddbc104e"),
    "Ghastly Demise",
    crate::card::CardArt::new("d9d2bfa3-0499-43ea-a76d-b12fddbc104e", "Gary Ruddell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 140 — Gravedigger (reprint)

// ODY 141 — Gravestorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVESTORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e273b71-85ca-49d2-ba96-70cc0ae8d718"),
    "Gravestorm",
    crate::card::CardArt::new(
        "5e273b71-85ca-49d2-ba96-70cc0ae8d718",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 142 — Haunting Echoes
pub(in crate::card::sets) static HAUNTING_ECHOES: CardRecord = CardRecord::new_with_legacy_id(
    2069,
    "Haunting Echoes",
    CardArt::new("3f051d37-e5ad-4975-839e-2da5538685f2", "Arnie Swekel"),
    CardSet::Odyssey,
    // Against a deck that wins with four copies of one card, taking the one
    // in the graveyard takes the other three as well.
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Exile all cards from target player's graveyard other than basic land cards. For each card exiled this way, search that player's library for all cards with the same name as that card and exile them. Then that player shuffles.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        abilities::bind_objects_then(
            crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ])),
                    &[ZoneKind::Graveyard],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                ),
            )),
            // Exile the yard, then hunt the library for every copy of what was taken.
            // The library search reads the bound set after the graveyard has emptied,
            // which is why the set is bound rather than queried twice.
            &const {
                EffectDef::Sequence(&const {
                    [
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                ParentBinding,
                            )),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::NameIn(&CardNameSetDef::NamesOf(
                                        &ObjectSetDef::Binding(ParentBinding),
                                    )),
                                    &[ZoneKind::Library],
                                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                                ),
                            )),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                        EffectDef::ShuffleLibrary {
                            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    ]
                })
            },
        ),
    )),
);

// ODY 143 — Hint of Insanity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HINT_OF_INSANITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6abaca0-7ce1-4024-adf6-ef6cc0fbcb75"),
    "Hint of Insanity",
    crate::card::CardArt::new("d6abaca0-7ce1-4024-adf6-ef6cc0fbcb75", "Luca Zontini"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 144 — Infected Vermin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFECTED_VERMIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00615487-3526-4b4c-bb06-8f2af1f101d0"),
    "Infected Vermin",
    crate::card::CardArt::new("00615487-3526-4b4c-bb06-8f2af1f101d0", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 145 — Innocent Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INNOCENT_BLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d26af8f6-df64-4027-880c-f2fae2d8103f"),
    "Innocent Blood",
    crate::card::CardArt::new("d26af8f6-df64-4027-880c-f2fae2d8103f", "Carl Critchlow"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 146 — Last Rites
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_RITES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b140a36-6686-4781-a432-cb2ef58afa81"),
    "Last Rites",
    crate::card::CardArt::new("0b140a36-6686-4781-a432-cb2ef58afa81", "Bradley Williams"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 147 — Malevolent Awakening
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALEVOLENT_AWAKENING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35d94052-aae3-4ced-9309-fc4a0d1f159d"),
    "Malevolent Awakening",
    crate::card::CardArt::new(
        "35d94052-aae3-4ced-9309-fc4a0d1f159d",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 148 — Mind Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c07889a1-1582-4f14-8925-fa21a1a5fd65"),
    "Mind Burst",
    crate::card::CardArt::new("c07889a1-1582-4f14-8925-fa21a1a5fd65", "Marc Fishman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 149 — Mindslicer
pub(in crate::card::sets) static MINDSLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("895b1c9b-b1a1-457b-92cd-3469a38b69a3"),
    "Mindslicer",
    CardArt::new("895b1c9b-b1a1-457b-92cd-3469a38b69a3", "Kev Walker"),
    CardSet::Odyssey,
    // A 4/3 nobody can afford to kill, which means it attacks freely and
    // the game ends on whoever blinks.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Horror"], 4, 3).with_ability(
        abilities::dies_trigger(
            "When this creature dies, each player discards their hand.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(i32::MAX),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ),
);

// ODY 150 — Morbid Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORBID_HUNGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47a4e7d0-ff09-4e19-8456-f4845f56dc8b"),
    "Morbid Hunger",
    crate::card::CardArt::new("47a4e7d0-ff09-4e19-8456-f4845f56dc8b", "Eric Peterson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 151 — Morgue Theft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORGUE_THEFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("937465ca-4cf7-4412-86eb-264efb0fdddd"),
    "Morgue Theft",
    crate::card::CardArt::new("937465ca-4cf7-4412-86eb-264efb0fdddd", "Justin Sweet"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 152 — Mortivore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORTIVORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fdc0d499-4a22-4f33-83c7-4e8cdbc3ebe6"),
    "Mortivore",
    crate::card::CardArt::new("fdc0d499-4a22-4f33-83c7-4e8cdbc3ebe6", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 153 — Nefarious Lich
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEFARIOUS_LICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d90f5a4e-8c3a-4803-bb59-d3d7c23761db"),
    "Nefarious Lich",
    crate::card::CardArt::new("d90f5a4e-8c3a-4803-bb59-d3d7c23761db", "Jerry Tiritilli"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 154 — Overeager Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVEREAGER_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b886292-5937-44ee-bc2a-f316791c91ae"),
    "Overeager Apprentice",
    crate::card::CardArt::new("8b886292-5937-44ee-bc2a-f316791c91ae", "Ray Lago"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 155 — Painbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAINBRINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f993815-af2f-4fa0-8848-979fc8c72d9a"),
    "Painbringer",
    crate::card::CardArt::new("3f993815-af2f-4fa0-8848-979fc8c72d9a", "Pete Venters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 156 — Patriarch's Desire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATRIARCH_S_DESIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("309bc2e7-bece-493d-b7ab-0e08a9f40909"),
    "Patriarch's Desire",
    crate::card::CardArt::new("309bc2e7-bece-493d-b7ab-0e08a9f40909", "Eric Peterson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 157 — Repentant Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPENTANT_VAMPIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c1dbad4-1105-4426-8d4d-a30f6fa95ce8"),
    "Repentant Vampire",
    crate::card::CardArt::new("2c1dbad4-1105-4426-8d4d-a30f6fa95ce8", "Mark Tedin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 158 — Rotting Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROTTING_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a2db4b2-6209-4a49-b868-e5d229ffcbc1"),
    "Rotting Giant",
    crate::card::CardArt::new("0a2db4b2-6209-4a49-b868-e5d229ffcbc1", "Kev Walker"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 159 — Sadistic Hypnotist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SADISTIC_HYPNOTIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98a6fba0-e5fc-4fa8-9895-8e4b8272bebe"),
    "Sadistic Hypnotist",
    crate::card::CardArt::new("98a6fba0-e5fc-4fa8-9895-8e4b8272bebe", "Paolo Parente"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 160 — Screams of the Damned
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCREAMS_OF_THE_DAMNED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66cc53cf-13eb-4028-944d-a670ad30dbca"),
    "Screams of the Damned",
    crate::card::CardArt::new("66cc53cf-13eb-4028-944d-a670ad30dbca", "Jerry Tiritilli"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 161 — Skeletal Scrying
pub(in crate::card::sets) static SKELETAL_SCRYING: CardRecord = CardRecord::new_with_legacy_id(
    2066,
    "Skeletal Scrying",
    CardArt::new("ee49bae4-6b1a-4c3f-8b2e-1d5a7c9e3f2b", "Bob Petillo"),
    CardSet::Odyssey,
    // Cards for life, paid for with the graveyard: a control deck that has
    // already spent its removal has the fuel and can afford the life.
    CardRules::new_instant(mana_cost!("{X}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile X cards from your graveyard.\nYou draw X cards and you lose X life.",
            &[],
            // X cards from your own graveyard, exiled as the spell is cast. The count is
            // the X it is cast for, so a big Scrying costs the graveyard that fed it.
            CostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                CostQuantityDef::ChosenX,
            ),
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::ChosenX,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::ChosenX,
                },
            ]),
        ),
    ),
);

// ODY 162 — Skull Fracture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKULL_FRACTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93e3c013-319f-4a77-a55e-11323468b8ea"),
    "Skull Fracture",
    crate::card::CardArt::new("93e3c013-319f-4a77-a55e-11323468b8ea", "Paolo Parente"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 163 — Stalking Bloodsucker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALKING_BLOODSUCKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16705695-1ba8-4169-974f-d8c683ab2652"),
    "Stalking Bloodsucker",
    crate::card::CardArt::new("16705695-1ba8-4169-974f-d8c683ab2652", "Greg Staples"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 164 — Tainted Pact
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAINTED_PACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c513f51b-a0db-4c08-8acc-1e91060b93b7"),
    "Tainted Pact",
    crate::card::CardArt::new("c513f51b-a0db-4c08-8acc-1e91060b93b7", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 165 — Tombfire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOMBFIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb08a4a7-941b-4b54-a89b-a4145a6fc14e"),
    "Tombfire",
    crate::card::CardArt::new("bb08a4a7-941b-4b54-a89b-a4145a6fc14e", "Arnie Swekel"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 166 — Traveling Plague
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAVELING_PLAGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4b6413d-68d3-4097-9bb3-873df4900e4c"),
    "Traveling Plague",
    crate::card::CardArt::new("d4b6413d-68d3-4097-9bb3-873df4900e4c", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 167 — Whispering Shade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHISPERING_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fbfdc2a-7bf3-4461-bef7-fa499d29d1b8"),
    "Whispering Shade",
    crate::card::CardArt::new("5fbfdc2a-7bf3-4461-bef7-fa499d29d1b8", "Daren Bader"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 168 — Zombie Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4649311-1300-42fb-ac10-90490756c7aa"),
    "Zombie Assassin",
    crate::card::CardArt::new("c4649311-1300-42fb-ac10-90490756c7aa", "Pete Venters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 169 — Zombie Cannibal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_CANNIBAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fe73ea4-caa2-41de-a065-272a4d850362"),
    "Zombie Cannibal",
    crate::card::CardArt::new("5fe73ea4-caa2-41de-a065-272a4d850362", "Adam Rex"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 170 — Zombie Infestation (reprint)

// ODY 171 — Zombify (alternate printing)

// ODY 171† — Zombify
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("284d5203-92db-4b1f-af73-fd372c8244cf"),
    "Zombify",
    crate::card::CardArt::new("284d5203-92db-4b1f-af73-fd372c8244cf", "Mark Romanoski"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 172 — Acceptable Losses
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACCEPTABLE_LOSSES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9082bfb8-0ab0-4378-976d-a2c9d3c35a5e"),
    "Acceptable Losses",
    crate::card::CardArt::new("9082bfb8-0ab0-4378-976d-a2c9d3c35a5e", "Mike Ploog"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 173 — Anarchist (reprint)

// ODY 174 — Ashen Firebeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHEN_FIREBEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebaef0bd-8288-49ba-a889-d897a4aae64c"),
    "Ashen Firebeast",
    crate::card::CardArt::new("ebaef0bd-8288-49ba-a889-d897a4aae64c", "Mark Tedin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 175 — Barbarian Lunatic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBARIAN_LUNATIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c899f9b-ebce-4424-9cd9-861a50a5f7d2"),
    "Barbarian Lunatic",
    crate::card::CardArt::new("6c899f9b-ebce-4424-9cd9-861a50a5f7d2", "Ron Spears"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 176 — Bash to Bits
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BASH_TO_BITS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("694b24de-d7f8-49c6-8ab3-d5fab13b6a8f"),
    "Bash to Bits",
    crate::card::CardArt::new("694b24de-d7f8-49c6-8ab3-d5fab13b6a8f", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 177 — Battle Strain
// Audit: unsupported — Needs an event for any creature becoming a blocker. TriggerEventDef::Blocks and BecomesBlockedBy both require the source to be the creature in combat, and BlocksOrBecomesBlockedBy fires once per orientation with the other side as the triggering object, so neither can name an arbitrary blocker's controller.
pub(in crate::card::sets) static BATTLE_STRAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee1be893-1287-40a3-81d6-271df3154b45"),
    "Battle Strain",
    crate::card::CardArt::new("ee1be893-1287-40a3-81d6-271df3154b45", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 178 — Blazing Salvo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLAZING_SALVO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7d192ef-a174-4df5-b67f-22918c32cf71"),
    "Blazing Salvo",
    crate::card::CardArt::new("f7d192ef-a174-4df5-b67f-22918c32cf71", "rk post"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 179 — Bomb Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOMB_SQUAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e9535a5-29ea-4085-a36b-4905d85e97ac"),
    "Bomb Squad",
    crate::card::CardArt::new(
        "8e9535a5-29ea-4085-a36b-4905d85e97ac",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 180 — Burning Sands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNING_SANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a5d5eef-6e3c-4907-a277-a13de2916e2b"),
    "Burning Sands",
    crate::card::CardArt::new("9a5d5eef-6e3c-4907-a277-a13de2916e2b", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 181 — Chainflinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAINFLINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("670a5bba-a10f-41f6-88cd-cef1dfe4bfa9"),
    "Chainflinger",
    crate::card::CardArt::new("670a5bba-a10f-41f6-88cd-cef1dfe4bfa9", "Scott M. Fischer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 182 — Chance Encounter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHANCE_ENCOUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57817159-de10-4a68-83e1-971fa9cfee2c"),
    "Chance Encounter",
    crate::card::CardArt::new("57817159-de10-4a68-83e1-971fa9cfee2c", "Mark Brill"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 183 — Demolish (reprint)

// ODY 184 — Demoralize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMORALIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e2f16f9-4980-45e9-99bf-0e00e3008b1d"),
    "Demoralize",
    crate::card::CardArt::new(
        "2e2f16f9-4980-45e9-99bf-0e00e3008b1d",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 185 — Dwarven Grunt
pub(in crate::card::sets) static DWARVEN_GRUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f00d726-4289-48ff-8c14-6a5080a00fda"),
    "Dwarven Grunt",
    CardArt::new("6f00d726-4289-48ff-8c14-6a5080a00fda", "Mike Ploog"),
    CardSet::Odyssey,
    // A one-drop that gets through against red decks, which is the only
    // matchup where one power matters.
    CardRules::new_creature(mana_cost!("{R}"), &["Dwarf"], 1, 1)
        .with_ability(abilities::landwalk(BasicLandType::Mountain)),
);

// ODY 186 — Dwarven Recruiter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_RECRUITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a15d274-85b4-4f3c-b502-f6dfe7db4d37"),
    "Dwarven Recruiter",
    crate::card::CardArt::new("6a15d274-85b4-4f3c-b502-f6dfe7db4d37", "Ciruelo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 187 — Dwarven Shrine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_SHRINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85197997-5e1a-46ce-8f0d-6da5ce297baf"),
    "Dwarven Shrine",
    crate::card::CardArt::new("85197997-5e1a-46ce-8f0d-6da5ce297baf", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 188 — Dwarven Strike Force
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_STRIKE_FORCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2bc3d85-5ba1-4f2e-a676-be989bdb04f7"),
    "Dwarven Strike Force",
    crate::card::CardArt::new("a2bc3d85-5ba1-4f2e-a676-be989bdb04f7", "Mike Ploog"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 189 — Earth Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTH_RIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2e10742-77c9-4f91-81b2-37b2ac910f09"),
    "Earth Rift",
    crate::card::CardArt::new("e2e10742-77c9-4f91-81b2-37b2ac910f09", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 190 — Ember Beast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBER_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25080720-612f-40c0-8894-cda8e3e8afb8"),
    "Ember Beast",
    crate::card::CardArt::new("25080720-612f-40c0-8894-cda8e3e8afb8", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 191 — Engulfing Flames
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENGULFING_FLAMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("164a92ac-0788-4238-a2ed-1ebd3dd0dd88"),
    "Engulfing Flames",
    crate::card::CardArt::new("164a92ac-0788-4238-a2ed-1ebd3dd0dd88", "Marc Fishman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 192 — Epicenter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EPICENTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a9232ae-9f32-4bbc-a020-554b3f9cbbd3"),
    "Epicenter",
    crate::card::CardArt::new("1a9232ae-9f32-4bbc-a020-554b3f9cbbd3", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 193 — Firebolt
pub(in crate::card::sets) static FIREBOLT: CardRecord = CardRecord::new_with_legacy_id(
    2152,
    "Firebolt",
    CardArt::new("d5e45005-dd81-4d80-b043-02f719aca929", "Ron Spencer"),
    CardSet::Odyssey,
    // Two cards for six mana across two turns, which is why it is played in
    // decks that would never pay five for two damage on its own.
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Firebolt deals 2 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::flashback(mana_cost!("{4}{R}")),
    ]),
);

// ODY 194 — Flame Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAME_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64bbd438-7df2-4d7b-88ad-4531ebaf3931"),
    "Flame Burst",
    crate::card::CardArt::new("64bbd438-7df2-4d7b-88ad-4531ebaf3931", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 195 — Frenetic Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRENETIC_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0dc939cc-826a-4208-ba5b-a11e1cd47aa2"),
    "Frenetic Ogre",
    crate::card::CardArt::new("0dc939cc-826a-4208-ba5b-a11e1cd47aa2", "Ron Spears"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 196 — Halberdier
pub(in crate::card::sets) static HALBERDIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b69dfc05-51ba-4798-ac00-1e9b8bbbf280"),
    "Halberdier",
    CardArt::new("b69dfc05-51ba-4798-ac00-1e9b8bbbf280", "Ben Thompson"),
    CardSet::Odyssey,
    // First strike on a 3/1 kills anything its size before it swings back,
    // and dies to any burn spell in the meantime.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Barbarian"], 3, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// ODY 197 — Impulsive Maneuvers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPULSIVE_MANEUVERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9b404c0-9ce1-4491-82fd-6dba7e6895cd"),
    "Impulsive Maneuvers",
    crate::card::CardArt::new("d9b404c0-9ce1-4491-82fd-6dba7e6895cd", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 198 — Kamahl, Pit Fighter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMAHL_PIT_FIGHTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee88c776-85f1-4beb-a814-f706f5c4f341"),
    "Kamahl, Pit Fighter",
    crate::card::CardArt::new("ee88c776-85f1-4beb-a814-f706f5c4f341", "Kev Walker"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 199 — Kamahl's Desire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMAHL_S_DESIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c99e1ded-7440-40a6-846b-7450a9b7d2a8"),
    "Kamahl's Desire",
    crate::card::CardArt::new("c99e1ded-7440-40a6-846b-7450a9b7d2a8", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 200 — Lava Blister
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_BLISTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd0e9e53-2710-4c2a-a8e4-48f25375ebc7"),
    "Lava Blister",
    crate::card::CardArt::new("cd0e9e53-2710-4c2a-a8e4-48f25375ebc7", "Rob Alexander"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 201 — Liquid Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIQUID_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b836d4c-38f3-4661-9dc7-0c9baaa595a5"),
    "Liquid Fire",
    crate::card::CardArt::new("8b836d4c-38f3-4661-9dc7-0c9baaa595a5", "Greg Staples"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 202 — Mad Dog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAD_DOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7be2da24-a0cd-4e90-bc29-4da649fa56bf"),
    "Mad Dog",
    crate::card::CardArt::new("7be2da24-a0cd-4e90-bc29-4da649fa56bf", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 203 — Magma Vein
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMA_VEIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69ed2b52-2862-423c-8ce3-6c8232d9d92c"),
    "Magma Vein",
    crate::card::CardArt::new("69ed2b52-2862-423c-8ce3-6c8232d9d92c", "Glen Angus"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 204 — Magnivore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNIVORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("200fa50d-5a54-47eb-a123-29c4fb55ebee"),
    "Magnivore",
    crate::card::CardArt::new("200fa50d-5a54-47eb-a123-29c4fb55ebee", "Carl Critchlow"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 205 — Mine Layer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINE_LAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("497efda1-5013-45d2-b717-b0296218c42b"),
    "Mine Layer",
    crate::card::CardArt::new("497efda1-5013-45d2-b717-b0296218c42b", "Mark Brill"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 206 — Minotaur Explorer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINOTAUR_EXPLORER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55725e38-d60a-41a2-93b0-2eefe6d2cc59"),
    "Minotaur Explorer",
    crate::card::CardArt::new("55725e38-d60a-41a2-93b0-2eefe6d2cc59", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 207 — Molten Influence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_INFLUENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c2b326b-d177-4a03-a0a3-fe2c2d4af272"),
    "Molten Influence",
    crate::card::CardArt::new("4c2b326b-d177-4a03-a0a3-fe2c2d4af272", "Franz Vohwinkel"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 208 — Mudhole
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUDHOLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("993d31bc-7355-4dfc-ac4e-ababaa0dc529"),
    "Mudhole",
    crate::card::CardArt::new("993d31bc-7355-4dfc-ac4e-ababaa0dc529", "Gary Ruddell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 209 — Need for Speed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEED_FOR_SPEED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8407b02f-66b3-4cfa-a3c4-105f314fd037"),
    "Need for Speed",
    crate::card::CardArt::new(
        "8407b02f-66b3-4cfa-a3c4-105f314fd037",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 210 — Obstinate Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBSTINATE_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88468a76-1f64-4189-bbb8-7c333181d57c"),
    "Obstinate Familiar",
    crate::card::CardArt::new("88468a76-1f64-4189-bbb8-7c333181d57c", "Terese Nielsen"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 211 — Pardic Firecat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARDIC_FIRECAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16858395-c742-4657-88c2-5af20c92718d"),
    "Pardic Firecat",
    crate::card::CardArt::new("16858395-c742-4657-88c2-5af20c92718d", "Glen Angus"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 212 — Pardic Miner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARDIC_MINER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a734abe-7bfd-4153-be2d-1c289fb60996"),
    "Pardic Miner",
    crate::card::CardArt::new("4a734abe-7bfd-4153-be2d-1c289fb60996", "Tony Szczudlo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 213 — Pardic Swordsmith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARDIC_SWORDSMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44ac622c-db04-41bf-817e-4698843e6346"),
    "Pardic Swordsmith",
    crate::card::CardArt::new("44ac622c-db04-41bf-817e-4698843e6346", "Bob Petillo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 214 — Price of Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRICE_OF_GLORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("785ccfe1-04f1-4d2e-9e30-2ae3efe7213a"),
    "Price of Glory",
    crate::card::CardArt::new("785ccfe1-04f1-4d2e-9e30-2ae3efe7213a", "Darrell Riche"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 215 — Reckless Charge (reprint)

// ODY 216 — Recoup
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECOUP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("431145f8-64e9-4e7d-998a-21fe55f49e01"),
    "Recoup",
    crate::card::CardArt::new("431145f8-64e9-4e7d-998a-21fe55f49e01", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 217 — Rites of Initiation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITES_OF_INITIATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f2fc246-2e95-456f-aa4e-97768c4f4bb4"),
    "Rites of Initiation",
    crate::card::CardArt::new("6f2fc246-2e95-456f-aa4e-97768c4f4bb4", "Bradley Williams"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 218 — Savage Firecat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_FIRECAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52e14dbe-4d33-405c-ab84-2fc6b1e000b8"),
    "Savage Firecat",
    crate::card::CardArt::new("52e14dbe-4d33-405c-ab84-2fc6b1e000b8", "Dave Dorman"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 219 — Scorching Missile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHING_MISSILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0672960b-4cb5-4ed6-ba3c-6b97290e0330"),
    "Scorching Missile",
    crate::card::CardArt::new("0672960b-4cb5-4ed6-ba3c-6b97290e0330", "Don Hazeltine"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 220 — Seize the Day
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEIZE_THE_DAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98d03ec3-2279-4266-a3bd-4cebdcb04d70"),
    "Seize the Day",
    crate::card::CardArt::new("98d03ec3-2279-4266-a3bd-4cebdcb04d70", "Greg Staples"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 221 — Shower of Coals
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOWER_OF_COALS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1e623a8-145e-4649-b2cc-d3bb8a93f0f3"),
    "Shower of Coals",
    crate::card::CardArt::new("b1e623a8-145e-4649-b2cc-d3bb8a93f0f3", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 222 — Spark Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPARK_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cff5902e-2ca1-43bb-b636-c860b3d0b3f2"),
    "Spark Mage",
    crate::card::CardArt::new("cff5902e-2ca1-43bb-b636-c860b3d0b3f2", "Paolo Parente"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 223 — Steam Vines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAM_VINES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3936053-b2ed-49b8-ab69-fe47a972cfe4"),
    "Steam Vines",
    crate::card::CardArt::new("d3936053-b2ed-49b8-ab69-fe47a972cfe4", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 224 — Thermal Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THERMAL_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("623cc141-0f75-4e67-b852-6c144d98e619"),
    "Thermal Blast",
    crate::card::CardArt::new("623cc141-0f75-4e67-b852-6c144d98e619", "Franz Vohwinkel"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 225 — Tremble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREMBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e1dc36f-3fdd-42cf-9d3a-695f4bf60c68"),
    "Tremble",
    crate::card::CardArt::new("7e1dc36f-3fdd-42cf-9d3a-695f4bf60c68", "Ciruelo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 226 — Volcanic Spray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLCANIC_SPRAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97daab4b-d934-4a3f-a043-f7c9c1dd32bf"),
    "Volcanic Spray",
    crate::card::CardArt::new("97daab4b-d934-4a3f-a043-f7c9c1dd32bf", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 227 — Volley of Boulders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLLEY_OF_BOULDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f663979-a778-4fb2-97c4-67ab82926f13"),
    "Volley of Boulders",
    crate::card::CardArt::new("4f663979-a778-4fb2-97c4-67ab82926f13", "Tony Szczudlo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 228 — Whipkeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d86db25-3ec1-42f3-8b02-2729d50dd201"),
    "Whipkeeper",
    crate::card::CardArt::new("3d86db25-3ec1-42f3-8b02-2729d50dd201", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 229 — Bearscape
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEARSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3284b61a-bd95-4846-ad3f-903cd1158867"),
    "Bearscape",
    crate::card::CardArt::new("3284b61a-bd95-4846-ad3f-903cd1158867", "Heather Hudson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 230 — Beast Attack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEAST_ATTACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a5ebe27-2083-400e-8943-b506be470e3f"),
    "Beast Attack",
    crate::card::CardArt::new("6a5ebe27-2083-400e-8943-b506be470e3f", "Ciruelo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 231 — Call of the Herd
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_OF_THE_HERD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("429a88cc-53db-4c5e-a061-f0f49a38c675"),
    "Call of the Herd",
    crate::card::CardArt::new("429a88cc-53db-4c5e-a061-f0f49a38c675", "Carl Critchlow"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 232 — Cartographer (reprint)

// ODY 233 — Chatter of the Squirrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHATTER_OF_THE_SQUIRREL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84273844-7fab-4ff7-afb3-c82153132daf"),
    "Chatter of the Squirrel",
    crate::card::CardArt::new("84273844-7fab-4ff7-afb3-c82153132daf", "Jim Nelson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 234 — Chlorophant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHLOROPHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4fd0ea3-4134-4f7d-a0c8-01c49bacfcfc"),
    "Chlorophant",
    crate::card::CardArt::new("c4fd0ea3-4134-4f7d-a0c8-01c49bacfcfc", "John Avon"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 235 — Crashing Centaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRASHING_CENTAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1f3a32a-bfd2-4c31-a349-3e62a84c20e1"),
    "Crashing Centaur",
    crate::card::CardArt::new("e1f3a32a-bfd2-4c31-a349-3e62a84c20e1", "Eric Peterson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 236 — Deep Reconnaissance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEP_RECONNAISSANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("148ce683-3911-4b77-9e07-3985766a3777"),
    "Deep Reconnaissance",
    crate::card::CardArt::new("148ce683-3911-4b77-9e07-3985766a3777", "Jeff Remmer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 237 — Diligent Farmhand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DILIGENT_FARMHAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bb40e09-6855-46d5-9bc9-bc6b2b0d7653"),
    "Diligent Farmhand",
    crate::card::CardArt::new("7bb40e09-6855-46d5-9bc9-bc6b2b0d7653", "Gary Ruddell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 238 — Druid Lyrist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRUID_LYRIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9923532-bc4f-44de-b963-d6914321c49a"),
    "Druid Lyrist",
    crate::card::CardArt::new("e9923532-bc4f-44de-b963-d6914321c49a", "Mark Zug"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 239 — Druid's Call
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRUID_S_CALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e22318a5-80c8-4fa3-ad05-8d3035caf336"),
    "Druid's Call",
    crate::card::CardArt::new(
        "e22318a5-80c8-4fa3-ad05-8d3035caf336",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 240 — Elephant Ambush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEPHANT_AMBUSH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4abdf1c-a0a9-4e1d-b448-58742830f767"),
    "Elephant Ambush",
    crate::card::CardArt::new("b4abdf1c-a0a9-4e1d-b448-58742830f767", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 241 — Gorilla Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("435d9562-8f2b-43fe-ba21-8f5896378280"),
    "Gorilla Titan",
    crate::card::CardArt::new("435d9562-8f2b-43fe-ba21-8f5896378280", "Heather Hudson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 242 — Ground Seal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROUND_SEAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("977383a8-9a31-4aad-ace2-6ed4d0dd1cbe"),
    "Ground Seal",
    crate::card::CardArt::new(
        "977383a8-9a31-4aad-ace2-6ed4d0dd1cbe",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 243 — Holistic Wisdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLISTIC_WISDOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2134a564-b089-4049-bd9c-a3f10e072263"),
    "Holistic Wisdom",
    crate::card::CardArt::new("2134a564-b089-4049-bd9c-a3f10e072263", "Rebecca Guay"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 244 — Howling Gale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOWLING_GALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9917cf32-0236-4463-9b1d-e8193754ff97"),
    "Howling Gale",
    crate::card::CardArt::new("9917cf32-0236-4463-9b1d-e8193754ff97", "Greg Staples"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 245 — Ivy Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IVY_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc441d3a-e917-4dd6-b5f9-f99075ec398f"),
    "Ivy Elemental",
    crate::card::CardArt::new("fc441d3a-e917-4dd6-b5f9-f99075ec398f", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 246 — Krosan Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_ARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e188562f-8219-4fb0-ac2c-5618cfb00bca"),
    "Krosan Archer",
    crate::card::CardArt::new("e188562f-8219-4fb0-ac2c-5618cfb00bca", "Ron Spears"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 247 — Krosan Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_AVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0afd6911-32b5-410a-afb0-fd3d2996fe59"),
    "Krosan Avenger",
    crate::card::CardArt::new(
        "0afd6911-32b5-410a-afb0-fd3d2996fe59",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 248 — Krosan Beast
pub(in crate::card::sets) static KROSAN_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af822507-fd4c-454b-ab07-106c81c535bf"),
    "Krosan Beast",
    CardArt::new("af822507-fd4c-454b-ab07-106c81c535bf", "Kev Walker"),
    CardSet::Odyssey,
    // A 1/1 for four that becomes an 8/8. Nothing in between: it is dead
    // weight until the graveyard fills and unanswerable afterwards.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Squirrel", "Beast"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "Threshold — This creature gets +7/+7 as long as there are seven or more \
             cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &THRESHOLD,
                then: &const {
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(7),
                            ValueDef::Constant(7),
                        ),
                    }
                },
            },
        ),
    ),
);

// ODY 249 — Leaf Dancer
pub(in crate::card::sets) static LEAF_DANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aebab65c-7d5f-4086-8eb1-7dc445e801e9"),
    "Leaf Dancer",
    CardArt::new(
        "aebab65c-7d5f-4086-8eb1-7dc445e801e9",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::Odyssey,
    // A 2/2 for three that the green mirror cannot block.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Centaur"], 2, 2)
        .with_ability(abilities::landwalk(BasicLandType::Forest)),
);

// ODY 250 — Metamorphic Wurm
pub(in crate::card::sets) static METAMORPHIC_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f47e020a-ed73-465a-b7eb-a4eeccd096cc"),
    "Metamorphic Wurm",
    CardArt::new("f47e020a-ed73-465a-b7eb-a4eeccd096cc", "Thomas M. Baxa"),
    CardSet::Odyssey,
    // A 3/3 that becomes a 7/7, which is the middle of the threshold curve
    // and the one that is playable before it turns on.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elephant", "Wurm"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "Threshold — This creature gets +4/+4 as long as there are seven or more \
             cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &THRESHOLD,
                then: &const {
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(4),
                        ),
                    }
                },
            },
        ),
    ),
);

// ODY 251 — Moment's Peace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENT_S_PEACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40ebe935-ccf9-435e-8fe8-53bcbf3526e7"),
    "Moment's Peace",
    crate::card::CardArt::new("40ebe935-ccf9-435e-8fe8-53bcbf3526e7", "Rebecca Guay"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 252 — Muscle Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUSCLE_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("217dada5-7ffc-488b-8062-34c034906ea9"),
    "Muscle Burst",
    crate::card::CardArt::new("217dada5-7ffc-488b-8062-34c034906ea9", "Gary Ruddell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 253 — Nantuko Disciple
pub(in crate::card::sets) static NANTUKO_DISCIPLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f5db5e0-bd95-4c82-a12d-7288dbbbe3ba"),
    "Nantuko Disciple",
    CardArt::new("3f5db5e0-bd95-4c82-a12d-7288dbbbe3ba", "Justin Sweet"),
    CardSet::Odyssey,
    // Tapping to pump, so it is a combat trick that cannot also block, and
    // the mana is trivial next to that.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Insect", "Druid"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}: Target creature gets +2/+2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ODY 254 — Nantuko Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c0a4e6e-cc4e-43d5-aece-f009e117366a"),
    "Nantuko Elder",
    crate::card::CardArt::new("5c0a4e6e-cc4e-43d5-aece-f009e117366a", "Daren Bader"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 255 — Nantuko Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_MENTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48cb5283-d384-490e-a0a5-2d10c1acc8cc"),
    "Nantuko Mentor",
    crate::card::CardArt::new("48cb5283-d384-490e-a0a5-2d10c1acc8cc", "John Matson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 256 — Nantuko Shrine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_SHRINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b66bab3-ee6f-4f74-bbc1-8099c9c4904b"),
    "Nantuko Shrine",
    crate::card::CardArt::new("4b66bab3-ee6f-4f74-bbc1-8099c9c4904b", "Rebecca Guay"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 257 — New Frontiers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEW_FRONTIERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd1f6cfa-9576-4bb8-83db-33c2b147206d"),
    "New Frontiers",
    crate::card::CardArt::new("fd1f6cfa-9576-4bb8-83db-33c2b147206d", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 258 — Nimble Mongoose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIMBLE_MONGOOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99e5ecf5-a662-4df0-a6ba-9177c62b6503"),
    "Nimble Mongoose",
    crate::card::CardArt::new("99e5ecf5-a662-4df0-a6ba-9177c62b6503", "Terese Nielsen"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 259 — Nut Collector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NUT_COLLECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4eec210-5df0-4fb8-8eb1-e616d9995acc"),
    "Nut Collector",
    crate::card::CardArt::new(
        "f4eec210-5df0-4fb8-8eb1-e616d9995acc",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 260 — Overrun (reprint)

// ODY 261 — Piper's Melody
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIPER_S_MELODY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("642ea1f9-49f0-4e0d-8122-c3a305c149e9"),
    "Piper's Melody",
    crate::card::CardArt::new(
        "642ea1f9-49f0-4e0d-8122-c3a305c149e9",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 262 — Primal Frenzy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMAL_FRENZY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5e2d822-09d0-42e5-ad91-67c66e947b3d"),
    "Primal Frenzy",
    crate::card::CardArt::new(
        "f5e2d822-09d0-42e5-ad91-67c66e947b3d",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 263 — Rabid Elephant
pub(in crate::card::sets) static RABID_ELEPHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fe394a38-ee44-4342-adcf-8c65d14f4978"),
    "Rabid Elephant",
    CardArt::new("fe394a38-ee44-4342-adcf-8c65d14f4978", "Dave Dorman"),
    CardSet::Odyssey,
    // A 3/4 that becomes a 7/8 against a double block, which is the whole
    // reason to attack with it into an open board.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Elephant"], 3, 4).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets +2/+2 until end of turn \
             for each creature blocking it.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // Counted as the trigger resolves, so a blocker that
                // has already left is not counted and one added by a
                // later effect is.
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(
                        &const {
                            ScaledValueDef {
                                value: ValueDef::CountMatchingObjects(
                                    &const {
                                        ObjectQueryDef::matching(
                                            ObjectPredicateDef::BlockingSource,
                                            &[ZoneKind::Battlefield],
                                            PlayerRelation::Any,
                                        )
                                    },
                                ),
                                factor: 2,
                            }
                        },
                    ),
                    ValueDef::Scaled(
                        &const {
                            ScaledValueDef {
                                value: ValueDef::CountMatchingObjects(
                                    &const {
                                        ObjectQueryDef::matching(
                                            ObjectPredicateDef::BlockingSource,
                                            &[ZoneKind::Battlefield],
                                            PlayerRelation::Any,
                                        )
                                    },
                                ),
                                factor: 2,
                            }
                        },
                    ),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ODY 264 — Refresh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REFRESH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0084ba4e-98eb-4eb4-b23e-c5ab4d7d95cb"),
    "Refresh",
    crate::card::CardArt::new("0084ba4e-98eb-4eb4-b23e-c5ab4d7d95cb", "Keith Garletts"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 265 — Rites of Spring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITES_OF_SPRING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1bddb0ef-fdda-491b-a0cf-48cbdd761918"),
    "Rites of Spring",
    crate::card::CardArt::new("1bddb0ef-fdda-491b-a0cf-48cbdd761918", "Bradley Williams"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 266 — Roar of the Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROAR_OF_THE_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ddf54317-4c08-4a66-9fd2-9983384e2374"),
    "Roar of the Wurm",
    crate::card::CardArt::new("ddf54317-4c08-4a66-9fd2-9983384e2374", "Kev Walker"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 267 — Seton, Krosan Protector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SETON_KROSAN_PROTECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("641a3bae-9146-4e70-862a-86a56f7c3816"),
    "Seton, Krosan Protector",
    crate::card::CardArt::new("641a3bae-9146-4e70-862a-86a56f7c3816", "Greg Staples"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 268 — Seton's Desire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SETON_S_DESIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70d05b6a-5c7e-4e08-88d3-98f2b5b054bc"),
    "Seton's Desire",
    crate::card::CardArt::new("70d05b6a-5c7e-4e08-88d3-98f2b5b054bc", "Ciruelo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 269 — Simplify
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIMPLIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a43fd3e-1e08-400c-b22b-e22da82bcdee"),
    "Simplify",
    crate::card::CardArt::new(
        "7a43fd3e-1e08-400c-b22b-e22da82bcdee",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 270 — Skyshooter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHOOTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f56f195e-a80c-4447-8d3f-cc1259035d1b"),
    "Skyshooter",
    crate::card::CardArt::new("f56f195e-a80c-4447-8d3f-cc1259035d1b", "Jerry Tiritilli"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 271 — Spellbane Centaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLBANE_CENTAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72194a6e-481d-4b07-922b-53f919bfa316"),
    "Spellbane Centaur",
    crate::card::CardArt::new("72194a6e-481d-4b07-922b-53f919bfa316", "Rick Farrell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 272 — Springing Tiger
pub(in crate::card::sets) static SPRINGING_TIGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04462629-fb03-40e8-84e6-4a66e4d5392b"),
    "Springing Tiger",
    CardArt::new("04462629-fb03-40e8-84e6-4a66e4d5392b", "Arnie Swekel"),
    CardSet::Odyssey,
    // A fair 3/3 that quietly becomes a 5/5, which is what a common was
    // allowed to do.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Cat"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "Threshold — This creature gets +2/+2 as long as there are seven or more \
             cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &THRESHOLD,
                then: &const {
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                    }
                },
            },
        ),
    ),
);

// ODY 273 — Squirrel Mob
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUIRREL_MOB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("181254ce-259a-4b31-8937-728564f2baf3"),
    "Squirrel Mob",
    crate::card::CardArt::new("181254ce-259a-4b31-8937-728564f2baf3", "Carl Critchlow"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 274 — Squirrel Nest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUIRREL_NEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22eccb27-1723-4c5a-96b8-85e6e5739c30"),
    "Squirrel Nest",
    crate::card::CardArt::new("22eccb27-1723-4c5a-96b8-85e6e5739c30", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 275 — Still Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STILL_LIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12be33d7-f25e-4dbf-9706-74403103b127"),
    "Still Life",
    crate::card::CardArt::new("12be33d7-f25e-4dbf-9706-74403103b127", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 276 — Stone-Tongue Basilisk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_TONGUE_BASILISK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ec4eeb5-2f82-460c-8140-20587442d40b"),
    "Stone-Tongue Basilisk",
    crate::card::CardArt::new("938d5157-154c-4300-82d4-0e23d934d436", "Wayne England"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 277 — Sylvan Might
pub(in crate::card::sets) static SYLVAN_MIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("576e3ccd-40a3-4ea9-8e76-5e70b2ef9123"),
    "Sylvan Might",
    CardArt::new("576e3ccd-40a3-4ea9-8e76-5e70b2ef9123", "Arnie Swekel"),
    CardSet::Odyssey,
    // The trample is what makes the flashback worth four mana: a second
    // combat trick out of the graveyard that the blocker cannot absorb.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 and gains trample until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{2}{G}{G}")),
    ]),
);

// ODY 278 — Terravore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRAVORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c39c412b-2f21-483a-b744-5d55bc007c0d"),
    "Terravore",
    crate::card::CardArt::new("c39c412b-2f21-483a-b744-5d55bc007c0d", "Jim Nelson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 279 — Twigwalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWIGWALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90b8ba27-c3bb-48cf-b831-518f5d255c2e"),
    "Twigwalker",
    crate::card::CardArt::new("90b8ba27-c3bb-48cf-b831-518f5d255c2e", "Carl Critchlow"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 280 — Verdant Succession
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDANT_SUCCESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd3c78de-0c2a-441e-8912-ff920fc563ef"),
    "Verdant Succession",
    crate::card::CardArt::new(
        "bd3c78de-0c2a-441e-8912-ff920fc563ef",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 281 — Vivify
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIVIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8b5e7ed-a0ab-48f6-8f69-56a8bd115007"),
    "Vivify",
    crate::card::CardArt::new("f8b5e7ed-a0ab-48f6-8f69-56a8bd115007", "Greg Staples"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 282 — Werebear (reprint)

// ODY 283 — Wild Mongrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_MONGREL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bb8dd5c-a79a-4afc-80b2-64645bb17a34"),
    "Wild Mongrel",
    crate::card::CardArt::new("5bb8dd5c-a79a-4afc-80b2-64645bb17a34", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 284 — Woodland Druid
pub(in crate::card::sets) static WOODLAND_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34e501e6-38da-44ad-abe2-53ea7f0eb4ae"),
    "Woodland Druid",
    CardArt::new("34e501e6-38da-44ad-abe2-53ea7f0eb4ae", "Rick Farrell"),
    CardSet::Odyssey,
    // A vanilla 1/2 for one, which is a blocker rather than a threat.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Druid"], 1, 2),
);

// ODY 285 — Zoologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOOLOGIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be645675-d263-42c4-9e71-f98cf4c2aab5"),
    "Zoologist",
    crate::card::CardArt::new(
        "be645675-d263-42c4-9e71-f98cf4c2aab5",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 286 — Atogatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATOGATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a3e6eb5-6d0f-4f82-86f9-bbce8d27afbb"),
    "Atogatog",
    crate::card::CardArt::new("4a3e6eb5-6d0f-4f82-86f9-bbce8d27afbb", "Ron Spears"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 287 — Decimate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECIMATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("912c398a-e49a-4399-ac41-7b1d4328a59d"),
    "Decimate",
    crate::card::CardArt::new(
        "912c398a-e49a-4399-ac41-7b1d4328a59d",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 288 — Iridescent Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRIDESCENT_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3bce4f8-3a3f-4ec2-9b37-10bb12713afc"),
    "Iridescent Angel",
    crate::card::CardArt::new("f3bce4f8-3a3f-4ec2-9b37-10bb12713afc", "Matt Cavotta"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 289 — Lithatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LITHATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c69742a8-cc6d-457b-8d99-81d05ab1bf0b"),
    "Lithatog",
    crate::card::CardArt::new("c69742a8-cc6d-457b-8d99-81d05ab1bf0b", "Franz Vohwinkel"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 290 — Mystic Enforcer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_ENFORCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb68983d-fd8c-4844-8ceb-afd3cae7e4df"),
    "Mystic Enforcer",
    crate::card::CardArt::new("eb68983d-fd8c-4844-8ceb-afd3cae7e4df", "Gary Ruddell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 291 — Phantatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6967608d-141d-426f-a129-6f1a6a58273c"),
    "Phantatog",
    crate::card::CardArt::new("6967608d-141d-426f-a129-6f1a6a58273c", "Glen Angus"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 292 — Psychatog
/// Both halves pump the same amount, so they share the applied effect. The
/// Atog eats its own graveyard as readily as its hand, which is why it grows
/// so fast in a deck that has been drawing and discarding all game.
static ATOG_PUMP: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static PSYCHATOG: CardRecord = CardRecord::new_with_legacy_id(
    2040,
    "Psychatog",
    CardArt::new(
        "6757bf0e-489f-4be2-9e41-463b59f00dd1",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Odyssey,
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Atog"], 1, 2).with_abilities(&[
        AbilityDef::activated(
            "Discard a card: This creature gets +1/+1 until end of turn.",
            &[CostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
            ATOG_PUMP,
        ),
        AbilityDef::activated(
            "Exile two cards from your graveyard: This creature gets +1/+1 until end of turn.",
            &[CostDef::MoveToZone(crate::card::MoveToZoneCostDef::new(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                ZoneKind::Exile,
                2,
            ))],
            ATOG_PUMP,
        ),
    ]),
);

// ODY 293 — Sarcatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SARCATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("760a329c-e815-49d3-8df1-d052ce19b0c6"),
    "Sarcatog",
    crate::card::CardArt::new("760a329c-e815-49d3-8df1-d052ce19b0c6", "Mark Brill"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 294 — Shadowmage Infiltrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOWMAGE_INFILTRATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("932ce702-565f-4d8c-b9fc-2d7c939ef7d7"),
    "Shadowmage Infiltrator",
    crate::card::CardArt::new("932ce702-565f-4d8c-b9fc-2d7c939ef7d7", "Rick Farrell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 295 — Thaumatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THAUMATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa559342-9587-4fe3-9059-3b3a08d6637a"),
    "Thaumatog",
    crate::card::CardArt::new(
        "aa559342-9587-4fe3-9059-3b3a08d6637a",
        "Monte Michael Moore",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 296 — Vampiric Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRIC_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f21d595-c248-4aae-9fd7-4e5787ab8781"),
    "Vampiric Dragon",
    crate::card::CardArt::new("4f21d595-c248-4aae-9fd7-4e5787ab8781", "Gary Ruddell"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 297 — Catalyst Stone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATALYST_STONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aaca85d4-b4c2-4adc-a0f9-26339bacfd92"),
    "Catalyst Stone",
    crate::card::CardArt::new("aaca85d4-b4c2-4adc-a0f9-26339bacfd92", "Tony Szczudlo"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 298 — Charmed Pendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARMED_PENDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("701942dd-7777-436f-8076-194584be8285"),
    "Charmed Pendant",
    crate::card::CardArt::new("701942dd-7777-436f-8076-194584be8285", "Anthony S. Waters"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 299 — Darkwater Egg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARKWATER_EGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb988572-1e1d-4b1d-8dc7-78bda966554e"),
    "Darkwater Egg",
    crate::card::CardArt::new("fb988572-1e1d-4b1d-8dc7-78bda966554e", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 300 — Junk Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNK_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9410da0-5693-456f-afa8-cb92ac176847"),
    "Junk Golem",
    crate::card::CardArt::new("c9410da0-5693-456f-afa8-cb92ac176847", "Brian Snõddy"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 301 — Limestone Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIMESTONE_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b522518c-1846-4e95-b77d-2aadac78684d"),
    "Limestone Golem",
    crate::card::CardArt::new("b522518c-1846-4e95-b77d-2aadac78684d", "Mark Tedin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 302 — Millikin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MILLIKIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0550133b-22cf-4ecd-b89a-8c2f0beeaa22"),
    "Millikin",
    crate::card::CardArt::new(
        "0550133b-22cf-4ecd-b89a-8c2f0beeaa22",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 303 — Mirari
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRARI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cbaad4f-fb66-4e56-b383-32f55552c8b2"),
    "Mirari",
    crate::card::CardArt::new("3cbaad4f-fb66-4e56-b383-32f55552c8b2", "Donato Giancola"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 304 — Mossfire Egg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOSSFIRE_EGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fd66433-b0e9-453d-9a27-ea0efb158dac"),
    "Mossfire Egg",
    crate::card::CardArt::new("1fd66433-b0e9-453d-9a27-ea0efb158dac", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 305 — Otarian Juggernaut
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OTARIAN_JUGGERNAUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("752420dc-d69a-4c9d-b563-f8928a4a0920"),
    "Otarian Juggernaut",
    crate::card::CardArt::new("752420dc-d69a-4c9d-b563-f8928a4a0920", "Brian Snõddy"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 306 — Patchwork Gnomes (reprint)

// ODY 307 — Sandstone Deadfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDSTONE_DEADFALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("241ebc17-b8ae-4ca4-8413-f53501d86244"),
    "Sandstone Deadfall",
    crate::card::CardArt::new("241ebc17-b8ae-4ca4-8413-f53501d86244", "Jim Nelson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 308 — Shadowblood Egg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOWBLOOD_EGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("044bbf15-e71d-4f47-bac3-bba8021118bd"),
    "Shadowblood Egg",
    crate::card::CardArt::new("044bbf15-e71d-4f47-bac3-bba8021118bd", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 309 — Skycloud Egg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYCLOUD_EGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e588cc41-4b86-4e93-94e8-dd765b27ab63"),
    "Skycloud Egg",
    crate::card::CardArt::new("e588cc41-4b86-4e93-94e8-dd765b27ab63", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 310 — Steamclaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAMCLAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f84a9bf-ce64-4301-8f2c-20f1b4acd3ef"),
    "Steamclaw",
    crate::card::CardArt::new("4f84a9bf-ce64-4301-8f2c-20f1b4acd3ef", "Jim Nelson"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 311 — Sungrass Egg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNGRASS_EGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70e888f9-c583-478a-9ef0-2e89db8a8dbb"),
    "Sungrass Egg",
    crate::card::CardArt::new("70e888f9-c583-478a-9ef0-2e89db8a8dbb", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 312 — Abandoned Outpost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABANDONED_OUTPOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4945031e-1158-474c-9e50-1ec817acc767"),
    "Abandoned Outpost",
    crate::card::CardArt::new(
        "4945031e-1158-474c-9e50-1ec817acc767",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 313 — Barbarian Ring
pub(in crate::card::sets) static BARBARIAN_RING: CardRecord = CardRecord::new_with_legacy_id(
    2033,
    "Barbarian Ring",
    CardArt::new("1809361e-ae1a-4c47-8464-e6496e94d962", "John Avon"),
    CardSet::Odyssey,
    // The land costs a life every time it makes mana, and pays that back once
    // the graveyard is deep enough to turn it into a burn spell.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}. This land deals 1 damage to you.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_damage_to_controller(1)),
        ),
        AbilityDef::activated_with_targets(
            "Threshold — {R}, {T}, Sacrifice this land: It deals 2 damage to any target. Activate only if there are seven or more cards in your graveyard.",
            &[
                CostDef::Mana(mana_cost!("{R}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_condition(&THRESHOLD),
    ]),
);

// ODY 314 — Bog Wreckage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_WRECKAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("189b4925-b34b-43bd-869e-1b1db99450e6"),
    "Bog Wreckage",
    crate::card::CardArt::new("189b4925-b34b-43bd-869e-1b1db99450e6", "Brian Snõddy"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 315 — Cabal Pit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_PIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("848d686a-e2f7-488d-947f-a555099b74b1"),
    "Cabal Pit",
    crate::card::CardArt::new("848d686a-e2f7-488d-947f-a555099b74b1", "John Avon"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 316 — Centaur Garden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_GARDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca041d5e-c65f-4e7e-ae4f-9c748a069aa3"),
    "Centaur Garden",
    crate::card::CardArt::new("ca041d5e-c65f-4e7e-ae4f-9c748a069aa3", "John Avon"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 317 — Cephalid Coliseum
pub(in crate::card::sets) static CEPHALID_COLISEUM: CardRecord = CardRecord::new_with_legacy_id(
    2041,
    "Cephalid Coliseum",
    CardArt::new("d5d74112-7244-4c3f-a5eb-b6be671aefe8", "John Avon"),
    CardSet::Odyssey,
    // The blue Barbarian Ring: a life every time it makes mana, and once the
    // graveyard is deep enough it cashes itself in for three fresh cards.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {U}. This land deals 1 damage to you.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue).with_damage_to_controller(1)),
        ),
        AbilityDef::activated_with_targets(
            "Threshold — {U}, {T}, Sacrifice this land: Target player draws three then discards three cards. Activate only if there are seven or more cards in your graveyard.",
            &[
                CostDef::Mana(mana_cost!("{U}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        )
        .with_activation_condition(&THRESHOLD),
    ]),
);

// ODY 318 — Crystal Quarry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_QUARRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1012124a-5401-4bd1-a163-4633d934938f"),
    "Crystal Quarry",
    crate::card::CardArt::new("1012124a-5401-4bd1-a163-4633d934938f", "Alan Pollack"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 319 — Darkwater Catacombs
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static DARKWATER_CATACOMBS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2583b941-6156-4c4b-a068-0d0ac75a3dd3"),
    "Darkwater Catacombs",
    crate::card::CardArt::new(
        "2583b941-6156-4c4b-a068-0d0ac75a3dd3",
        "Monte Michael Moore",
    ),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 320 — Deserted Temple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESERTED_TEMPLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1fa74b2-f9bd-4617-8b65-878781f3a2fd"),
    "Deserted Temple",
    crate::card::CardArt::new("d1fa74b2-f9bd-4617-8b65-878781f3a2fd", "Rob Alexander"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 321 — Mossfire Valley
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static MOSSFIRE_VALLEY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b6c08ce-d01d-4ae6-81d3-149679e27e6a"),
    "Mossfire Valley",
    crate::card::CardArt::new("6b6c08ce-d01d-4ae6-81d3-149679e27e6a", "John Avon"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 322 — Nomad Stadium
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOMAD_STADIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64300b71-050f-47a3-83be-f24480bdc01d"),
    "Nomad Stadium",
    crate::card::CardArt::new("64300b71-050f-47a3-83be-f24480bdc01d", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 323 — Petrified Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PETRIFIED_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eaeaf9f2-d196-4607-a704-06f2315d8cc5"),
    "Petrified Field",
    crate::card::CardArt::new("eaeaf9f2-d196-4607-a704-06f2315d8cc5", "Glen Angus"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 324 — Ravaged Highlands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVAGED_HIGHLANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb60ffc9-4919-40f2-bdd6-07eee6abf37c"),
    "Ravaged Highlands",
    crate::card::CardArt::new("bb60ffc9-4919-40f2-bdd6-07eee6abf37c", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 325 — Seafloor Debris (alternate printing)

// ODY 325† — Seafloor Debris
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEAFLOOR_DEBRIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fb5977c-5009-4d97-82c8-c150a0d41bc3"),
    "Seafloor Debris",
    crate::card::CardArt::new("6fb5977c-5009-4d97-82c8-c150a0d41bc3", "Larry Elmore"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 326 — Shadowblood Ridge
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static SHADOWBLOOD_RIDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69a5f84a-9e9b-42b6-a973-864409d6e564"),
    "Shadowblood Ridge",
    crate::card::CardArt::new("69a5f84a-9e9b-42b6-a973-864409d6e564", "Glen Angus"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 327 — Skycloud Expanse
pub(in crate::card::sets) static SKYCLOUD_EXPANSE: CardRecord = CardRecord::new_with_legacy_id(
    2087,
    "Skycloud Expanse",
    CardArt::new("35c527b6-4004-41f7-b70c-1ac1a49dce1f", "Rob Alexander"),
    CardSet::Odyssey,
    // Two mana for two, which is only worth a land slot to a deck that
    // needs both colours on the same turn and is happy to spend a land drop
    // on fixing rather than on the count.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{1}, {T}: Add {W}{U}.",
        &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one_of_each(
            ManaColor::White,
            ManaColor::Blue,
        )),
    )),
);

// ODY 328 — Sungrass Prairie
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static SUNGRASS_PRAIRIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bfa27ee-553e-4c6e-a79c-9757bd74c057"),
    "Sungrass Prairie",
    crate::card::CardArt::new("7bfa27ee-553e-4c6e-a79c-9757bd74c057", "Ron Spencer"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 329 — Tarnished Citadel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TARNISHED_CITADEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30375d24-ccfe-47a2-babd-1bda0a6298fe"),
    "Tarnished Citadel",
    crate::card::CardArt::new("30375d24-ccfe-47a2-babd-1bda0a6298fe", "David Martin"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 330 — Timberland Ruins
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMBERLAND_RUINS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd2e8770-c72b-439c-8f79-3aa24646cdd5"),
    "Timberland Ruins",
    crate::card::CardArt::new("dd2e8770-c72b-439c-8f79-3aa24646cdd5", "Alan Pollack"),
    crate::card::CardSet::Odyssey,
    crate::card::CardRules::unsupported(),
);

// ODY 331 — Plains (reprint)

// ODY 332 — Plains (alternate printing)

// ODY 333 — Plains (alternate printing)

// ODY 334 — Plains (alternate printing)

// ODY 335 — Island (reprint)

// ODY 336 — Island (alternate printing)

// ODY 337 — Island (alternate printing)

// ODY 338 — Island (alternate printing)

// ODY 339 — Swamp (reprint)

// ODY 340 — Swamp (alternate printing)

// ODY 341 — Swamp (alternate printing)

// ODY 342 — Swamp (alternate printing)

// ODY 343 — Mountain (reprint)

// ODY 344 — Mountain (alternate printing)

// ODY 345 — Mountain (alternate printing)

// ODY 346 — Mountain (alternate printing)

// ODY 347 — Forest (reprint)

// ODY 348 — Forest (alternate printing)

// ODY 349 — Forest (alternate printing)

// ODY 350 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AEGIS_OF_HONOR,
    &ANCESTRAL_TRIBUTE,
    &ANIMAL_BONEYARD,
    &AVEN_ARCHER,
    &AVEN_CLOUDCHASER,
    &AVEN_FLOCK,
    &AVEN_SHRINE,
    &BALANCING_ACT,
    &BELOVED_CHAPLAIN,
    &BLESSED_ORATOR,
    &CANTIVORE,
    &CEASE_FIRE,
    &CONFESSOR,
    &DEDICATED_MARTYR,
    &DELAYING_SHIELD,
    &DEVOTED_CARETAKER,
    &DIVINE_SACRAMENT,
    &DOGGED_HUNTER,
    &EARNEST_FELLOWSHIP,
    &EMBOLDEN,
    &GRACEFUL_ANTELOPE,
    &HALLOWED_HEALER,
    &KARMIC_JUSTICE,
    &KIRTAR_S_DESIRE,
    &KIRTAR_S_WRATH,
    &LIEUTENANT_KIRTAR,
    &LIFE_BURST,
    &LUMINOUS_GUARDIAN,
    &MASTER_APOTHECARY,
    &MYSTIC_CRUSADER,
    &MYSTIC_PENITENT,
    &MYSTIC_VISIONARY,
    &MYSTIC_ZEALOT,
    &NOMAD_DECOY,
    &PATROL_HOUND,
    &PIANNA_NOMAD_CAPTAIN,
    &PILGRIM_OF_JUSTICE,
    &PILGRIM_OF_VIRTUE,
    &RAY_OF_DISTORTION,
    &RESILIENT_WANDERER,
    &SACRED_RITES,
    &SECOND_THOUGHTS,
    &SHELTER,
    &SOULCATCHER,
    &SPHERE_OF_DUTY,
    &SPHERE_OF_GRACE,
    &SPHERE_OF_LAW,
    &SPHERE_OF_REASON,
    &SPHERE_OF_TRUTH,
    &SPIRITUALIZE,
    &TATTOO_WARD,
    &TESTAMENT_OF_FAITH,
    &TIRELESS_TRIBE,
    &WAYWARD_ANGEL,
    &ABOSHAN_CEPHALID_EMPEROR,
    &ABOSHAN_S_DESIRE,
    &AETHER_BURST,
    &AMUGABA,
    &AURA_GRAFT,
    &AVEN_FISHER,
    &AVEN_SMOKEWEAVER,
    &AVEN_WINDREADER,
    &BALSHAN_BEGUILER,
    &BALSHAN_GRIFFIN,
    &BAMBOOZLE,
    &CAREFUL_STUDY,
    &CEPHALID_BROKER,
    &CEPHALID_LOOTER,
    &CEPHALID_RETAINER,
    &CEPHALID_SCOUT,
    &CEPHALID_SHRINE,
    &CHAMBER_OF_MANIPULATION,
    &COGNIVORE,
    &CONCENTRATE,
    &CULTURAL_EXCHANGE,
    &DELUGE,
    &DEMATERIALIZE,
    &DIVERT,
    &DREAMWINDER,
    &ESCAPE_ARTIST,
    &EXTRACT,
    &FERVENT_DENIAL,
    &IMMOBILIZING_INK,
    &LAQUATUS_S_CREATIVITY,
    &PATRON_WIZARD,
    &PEDANTIC_LEARNING,
    &PEEK,
    &PERSUASION,
    &PHANTOM_WHELP,
    &PREDICT,
    &PSIONIC_GIFT,
    &PULSATING_ILLUSION,
    &PUPPETEER,
    &REPEL,
    &RITES_OF_REFUSAL,
    &SHIFTY_DOPPELGANGER,
    &STANDSTILL,
    &THINK_TANK,
    &THOUGHT_DEVOURER,
    &THOUGHT_EATER,
    &THOUGHT_NIBBLER,
    &TIME_STRETCH,
    &TOUCH_OF_INVISIBILITY,
    &TREETOP_SENTINEL,
    &UNIFYING_THEORY,
    &UPHEAVAL,
    &WORDS_OF_WISDOM,
    &AFFLICT,
    &BLOODCURDLER,
    &BRAIDS_CABAL_MINION,
    &CABAL_INQUISITOR,
    &CABAL_PATRIARCH,
    &CABAL_SHRINE,
    &CAUSTIC_TAR,
    &CHILDHOOD_HORROR,
    &COFFIN_PURGE,
    &CURSED_MONSTROSITY,
    &DECAYING_SOIL,
    &DECOMPOSE,
    &DIRTY_WERERAT,
    &DUSK_IMP,
    &ENTOMB,
    &EXECUTE,
    &FACE_OF_FEAR,
    &FAMISHED_GHOUL,
    &FILTHY_CUR,
    &FLEDGLING_IMP,
    &FRIGHTCRAWLER,
    &GHASTLY_DEMISE,
    &GRAVESTORM,
    &HAUNTING_ECHOES,
    &HINT_OF_INSANITY,
    &INFECTED_VERMIN,
    &INNOCENT_BLOOD,
    &LAST_RITES,
    &MALEVOLENT_AWAKENING,
    &MIND_BURST,
    &MINDSLICER,
    &MORBID_HUNGER,
    &MORGUE_THEFT,
    &MORTIVORE,
    &NEFARIOUS_LICH,
    &OVEREAGER_APPRENTICE,
    &PAINBRINGER,
    &PATRIARCH_S_DESIRE,
    &REPENTANT_VAMPIRE,
    &ROTTING_GIANT,
    &SADISTIC_HYPNOTIST,
    &SCREAMS_OF_THE_DAMNED,
    &SKELETAL_SCRYING,
    &SKULL_FRACTURE,
    &STALKING_BLOODSUCKER,
    &TAINTED_PACT,
    &TOMBFIRE,
    &TRAVELING_PLAGUE,
    &WHISPERING_SHADE,
    &ZOMBIE_ASSASSIN,
    &ZOMBIE_CANNIBAL,
    &ZOMBIFY,
    &ACCEPTABLE_LOSSES,
    &ASHEN_FIREBEAST,
    &BARBARIAN_LUNATIC,
    &BASH_TO_BITS,
    &BATTLE_STRAIN,
    &BLAZING_SALVO,
    &BOMB_SQUAD,
    &BURNING_SANDS,
    &CHAINFLINGER,
    &CHANCE_ENCOUNTER,
    &DEMORALIZE,
    &DWARVEN_GRUNT,
    &DWARVEN_RECRUITER,
    &DWARVEN_SHRINE,
    &DWARVEN_STRIKE_FORCE,
    &EARTH_RIFT,
    &EMBER_BEAST,
    &ENGULFING_FLAMES,
    &EPICENTER,
    &FIREBOLT,
    &FLAME_BURST,
    &FRENETIC_OGRE,
    &HALBERDIER,
    &IMPULSIVE_MANEUVERS,
    &KAMAHL_PIT_FIGHTER,
    &KAMAHL_S_DESIRE,
    &LAVA_BLISTER,
    &LIQUID_FIRE,
    &MAD_DOG,
    &MAGMA_VEIN,
    &MAGNIVORE,
    &MINE_LAYER,
    &MINOTAUR_EXPLORER,
    &MOLTEN_INFLUENCE,
    &MUDHOLE,
    &NEED_FOR_SPEED,
    &OBSTINATE_FAMILIAR,
    &PARDIC_FIRECAT,
    &PARDIC_MINER,
    &PARDIC_SWORDSMITH,
    &PRICE_OF_GLORY,
    &RECOUP,
    &RITES_OF_INITIATION,
    &SAVAGE_FIRECAT,
    &SCORCHING_MISSILE,
    &SEIZE_THE_DAY,
    &SHOWER_OF_COALS,
    &SPARK_MAGE,
    &STEAM_VINES,
    &THERMAL_BLAST,
    &TREMBLE,
    &VOLCANIC_SPRAY,
    &VOLLEY_OF_BOULDERS,
    &WHIPKEEPER,
    &BEARSCAPE,
    &BEAST_ATTACK,
    &CALL_OF_THE_HERD,
    &CHATTER_OF_THE_SQUIRREL,
    &CHLOROPHANT,
    &CRASHING_CENTAUR,
    &DEEP_RECONNAISSANCE,
    &DILIGENT_FARMHAND,
    &DRUID_LYRIST,
    &DRUID_S_CALL,
    &ELEPHANT_AMBUSH,
    &GORILLA_TITAN,
    &GROUND_SEAL,
    &HOLISTIC_WISDOM,
    &HOWLING_GALE,
    &IVY_ELEMENTAL,
    &KROSAN_ARCHER,
    &KROSAN_AVENGER,
    &KROSAN_BEAST,
    &LEAF_DANCER,
    &METAMORPHIC_WURM,
    &MOMENT_S_PEACE,
    &MUSCLE_BURST,
    &NANTUKO_DISCIPLE,
    &NANTUKO_ELDER,
    &NANTUKO_MENTOR,
    &NANTUKO_SHRINE,
    &NEW_FRONTIERS,
    &NIMBLE_MONGOOSE,
    &NUT_COLLECTOR,
    &PIPER_S_MELODY,
    &PRIMAL_FRENZY,
    &RABID_ELEPHANT,
    &REFRESH,
    &RITES_OF_SPRING,
    &ROAR_OF_THE_WURM,
    &SETON_KROSAN_PROTECTOR,
    &SETON_S_DESIRE,
    &SIMPLIFY,
    &SKYSHOOTER,
    &SPELLBANE_CENTAUR,
    &SPRINGING_TIGER,
    &SQUIRREL_MOB,
    &SQUIRREL_NEST,
    &STILL_LIFE,
    &STONE_TONGUE_BASILISK,
    &SYLVAN_MIGHT,
    &TERRAVORE,
    &TWIGWALKER,
    &VERDANT_SUCCESSION,
    &VIVIFY,
    &WILD_MONGREL,
    &WOODLAND_DRUID,
    &ZOOLOGIST,
    &ATOGATOG,
    &DECIMATE,
    &IRIDESCENT_ANGEL,
    &LITHATOG,
    &MYSTIC_ENFORCER,
    &PHANTATOG,
    &PSYCHATOG,
    &SARCATOG,
    &SHADOWMAGE_INFILTRATOR,
    &THAUMATOG,
    &VAMPIRIC_DRAGON,
    &CATALYST_STONE,
    &CHARMED_PENDANT,
    &DARKWATER_EGG,
    &JUNK_GOLEM,
    &LIMESTONE_GOLEM,
    &MILLIKIN,
    &MIRARI,
    &MOSSFIRE_EGG,
    &OTARIAN_JUGGERNAUT,
    &SANDSTONE_DEADFALL,
    &SHADOWBLOOD_EGG,
    &SKYCLOUD_EGG,
    &STEAMCLAW,
    &SUNGRASS_EGG,
    &ABANDONED_OUTPOST,
    &BARBARIAN_RING,
    &BOG_WRECKAGE,
    &CABAL_PIT,
    &CENTAUR_GARDEN,
    &CEPHALID_COLISEUM,
    &CRYSTAL_QUARRY,
    &DARKWATER_CATACOMBS,
    &DESERTED_TEMPLE,
    &MOSSFIRE_VALLEY,
    &NOMAD_STADIUM,
    &PETRIFIED_FIELD,
    &RAVAGED_HIGHLANDS,
    &SEAFLOOR_DEBRIS,
    &SHADOWBLOOD_RIDGE,
    &SKYCLOUD_EXPANSE,
    &SUNGRASS_PRAIRIE,
    &TARNISHED_CITADEL,
    &TIMBERLAND_RUINS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_avr::ANGELIC_WALL), // ODY 3
    PrintingRecord::reprint(&catalog_m14::AURAMANCER),   // ODY 5
    PrintingRecord::reprint(&catalog_tmp::GALLANTRY),    // ODY 23
    PrintingRecord::reprint(&catalog_m13::BATTLE_OF_WITS), // ODY 69
    PrintingRecord::alternate(&CEPHALID_LOOTER, 1),      // ODY 72
    PrintingRecord::reprint(&catalog_exo::SCRIVENER),    // ODY 100
    PrintingRecord::reprint(&crate::card::sets::y2012::return_to_ravnica::SYNCOPATE), // ODY 103
    PrintingRecord::reprint(&catalog_m14::TRAUMATIZE),   // ODY 110
    PrintingRecord::reprint(&catalog_wth::BURIED_ALIVE), // ODY 118
    PrintingRecord::reprint(&catalog_avr::CRYPT_CREEPER), // ODY 125
    PrintingRecord::reprint(&catalog_m14::DIABOLIC_TUTOR), // ODY 129
    PrintingRecord::reprint(&catalog_m12::GRAVEDIGGER),  // ODY 140
    PrintingRecord::reprint(&catalog_m12::ZOMBIE_INFESTATION), // ODY 170
    PrintingRecord::alternate(&ZOMBIFY, 1),              // ODY 171
    PrintingRecord::reprint(&catalog_exo::ANARCHIST),    // ODY 173
    PrintingRecord::reprint(&catalog_avr::DEMOLISH),     // ODY 183
    PrintingRecord::reprint(&catalog_mh1::RECKLESS_CHARGE), // ODY 215
    PrintingRecord::reprint(&catalog_exo::CARTOGRAPHER), // ODY 232
    PrintingRecord::reprint(&catalog_m12::OVERRUN),      // ODY 260
    PrintingRecord::reprint(&catalog_ema::WEREBEAR),     // ODY 282
    PrintingRecord::reprint(&catalog_tmp::PATCHWORK_GNOMES), // ODY 306
    PrintingRecord::alternate(&SEAFLOOR_DEBRIS, 1),      // ODY 325
    PrintingRecord::reprint(&catalog_lea::PLAINS),       // ODY 331
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),  // ODY 332
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),  // ODY 333
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3),  // ODY 334
    PrintingRecord::reprint(&catalog_lea::ISLAND),       // ODY 335
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),  // ODY 336
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),  // ODY 337
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3),  // ODY 338
    PrintingRecord::reprint(&catalog_lea::SWAMP),        // ODY 339
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),   // ODY 340
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),   // ODY 341
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3),   // ODY 342
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),     // ODY 343
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1), // ODY 344
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2), // ODY 345
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3), // ODY 346
    PrintingRecord::reprint(&catalog_lea::FOREST),       // ODY 347
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),  // ODY 348
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),  // ODY 349
    PrintingRecord::alternate(&catalog_lea::FOREST, 3),  // ODY 350
];
