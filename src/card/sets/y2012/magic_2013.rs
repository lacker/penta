//! Magic 2013 card records used by the built-in ISD–M14 Standard deck tranche.

use super::{CardRecord, PrintingRecord, dark_ascension};
use crate::card::CostQuantityDef;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1998::stronghold as catalog_sth;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y2001::apocalypse as catalog_apc;
use crate::card::sets::y2001::odyssey as catalog_ody;
use crate::card::sets::y2004::fifth_dawn as catalog_5dn;
use crate::card::sets::y2006::time_spiral as catalog_tsp;
use crate::card::sets::y2007::future_sight as catalog_fut;
use crate::card::sets::y2010::magic_2011 as catalog_m11;
use crate::card::sets::{
    y1993::alpha, y1994::the_dark, y1999::urzas_legacy, y2001::planeshift, y2002::onslaught,
    y2011::innistrad,
};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, AttackEventMatcherDef, BasicLandType, CardArt, CardRules,
    CardSet, CardSupertype, CardType, CardTypeSet, CastTimingPermissionDef, ChoiceVisibilityDef,
    ChooseDef, ColorSet, ComparisonDef, ConditionalStaticEffectDef, CostModificationDef,
    CounterKind, DamageEventMatcherDef, DamageKindDef, DamageLimitDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DividedTotal, EffectChoiceDef, EffectDef, EffectRecipientDef,
    FreePlayDef, FreePlayDurationDef, KeywordAbility, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetCountConditionDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef, ReplacementEventDef,
    ResolvedEffectDurationDef, SacrificedAmountDef, SpellAdditionalCostDef, StaticApplyDef,
    TargetChooserDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

/// The live number of Swamps controlled by the resolving effect's controller.
/// Liliana reads it twice in either branch of her −3.
static SWAMPS_YOU_CONTROL: ValueDef = ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
));

// M13 1 — Ajani, Caller of the Pride
pub(in crate::card::sets) static AJANI_CALLER_OF_THE_PRIDE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ajani, Caller of the Pride",
    "5e7f410a-7934-48ae-a90b-ffd096aed43d",
    "D. Alexander Gregory",
    CardRules::new_planeswalker(mana_cost!("{1}{W}{W}"), &["Ajani"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Put a +1/+1 counter on up to one target creature.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated_with_targets(
                "−3: Target creature gains flying and double strike until end of turn.",
                &[AbilityCostDef::Loyalty(-3)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::double_strike()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "−8: Create X 2/2 white Cat creature tokens, where X is your life total.",
                &[AbilityCostDef::Loyalty(-8)],
                EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 2, 2)
                    .with_art(CardArt::new(
                        "f97868f6-a9ce-4ce9-bc3f-b535f3202602",
                        "Jesper Ejsing",
                    ))
                    .with_count(ValueDef::LifeTotal(PlayerRelation::You)),
            ),
        ]),
);

// M13 2 — Ajani's Sunstriker
pub(in crate::card::sets) static AJANIS_SUNSTRIKER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ajani's Sunstriker",
    "3570c4d9-cd42-4aca-9421-ac44e057a785",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Cat", "Cleric"], 2, 2)
        .with_abilities(&[abilities::lifelink()]),
);

// M13 3 — Angel's Mercy (reprint)
const ANGELS_MERCY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ANGELS_MERCY,
    "43e6d650-4e96-43a3-8b94-7f044d3b2f82",
    "Andrew Robinson",
);

// M13 4 — Angelic Benediction (reprint)
const ANGELIC_BENEDICTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shards_of_alara::ANGELIC_BENEDICTION,
    "22125507-31e3-424c-9527-d994e4525d75",
    "Michael Komarck",
);

// M13 5 — Attended Knight
pub(in crate::card::sets) static ATTENDED_KNIGHT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Attended Knight",
    "c0f5cb3f-c27d-4b35-930f-00d806393796",
    "Seb McKinnon",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Soldier creature token.",
            EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1).with_art(
                CardArt::new("86272c08-c5f2-413f-87ea-b135aca2d9c5", "Greg Staples"),
            ),
        ),
    ]),
);

// M13 6 — Aven Squire (reprint)
const AVEN_SQUIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::conflux::AVEN_SQUIRE,
    "e60a0c43-9f47-404a-8acf-508173e7062f",
    "David Palumbo",
);

// M13 7 — Battleflight Eagle
pub(in crate::card::sets) static BATTLEFLIGHT_EAGLE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Battleflight Eagle",
    "4182dbd5-8eae-4f4b-86aa-2bfc24481800",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets("When this creature enters, target creature gets +2/+2 and gains flying until end of turn.", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
    ]),
);

// M13 8 — Captain of the Watch (reprint)
const CAPTAIN_OF_THE_WATCH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::CAPTAIN_OF_THE_WATCH,
    "8e3c18f5-89cd-4d33-8d5b-12dacad9f9b3",
    "Greg Staples",
);

// M13 9 — Captain's Call
pub(in crate::card::sets) static CAPTAINS_CALL: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Captain's Call",
    "79258432-ea35-4f2a-9e4a-4abb53f335c6",
    "Greg Staples",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Create three 1/1 white Soldier creature tokens.",
        EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
            .with_art(CardArt::new(
                "86272c08-c5f2-413f-87ea-b135aca2d9c5",
                "Greg Staples",
            ))
            .with_amount(3),
    )),
);

// M13 10 — Crusader of Odric
static CRUSADER_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static CRUSADER_OF_ODRIC: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Crusader of Odric",
    "295096bb-1857-4224-bc7b-307b38cfd338",
    "Michael Komarck",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 0, 0).with_ability(
        AbilityDef::static_ability(
            "Crusader of Odric's power and toughness are each equal to the number of creatures you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CRUSADER_CREATURES), ValueDef::CountMatchingObjects(&CRUSADER_CREATURES)),
            },
        ),
    ),
);

// M13 11 — Divine Favor (reprint)
const DIVINE_FAVOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::DIVINE_FAVOR,
    "b713c1f7-9346-4f4e-8fcd-5ada5b3f95c0",
    "Allen Williams",
);

// M13 12 — Divine Verdict (reprint)
const DIVINE_VERDICT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DIVINE_VERDICT,
    "cc52c269-d44f-449c-af59-4c425aa10bbf",
    "Kev Walker",
);

// M13 13 — Erase (reprint)
const ERASE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_legacy::ERASE,
    "8618b737-faa0-4a0c-a3f2-bee685c00580",
    "Richard Wright",
);

// M13 14 — Faith's Reward
// Audit: unsupported — Needs turn-history provenance for permanent cards put into your graveyard from the battlefield and a simultaneous mass return.
pub(in crate::card::sets) static FAITH_S_REWARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Faith's Reward",
    "799ed076-4724-47bb-94a0-11b42a9826eb",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// M13 15 — Glorious Charge (reprint)
const GLORIOUS_CHARGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::GLORIOUS_CHARGE,
    "f8672cfd-e34b-4587-9e24-015e03c7574d",
    "Izzy",
);

// M13 16 — Griffin Protector
pub(in crate::card::sets) static GRIFFIN_PROTECTOR: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Griffin Protector",
    "ddae4f7a-525c-4306-81b5-b0991840a11e",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature gets +1/+1 until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 17 — Guardian Lions
pub(in crate::card::sets) static GUARDIAN_LIONS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Guardian Lions",
    "3defc506-537e-4659-815d-5dab15fbf199",
    "Johannes Voss",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Cat"], 1, 6)
        .with_abilities(&[abilities::vigilance()]),
);

// M13 18 — Guardians of Akrasa (reprint)
const GUARDIANS_OF_AKRASA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shards_of_alara::GUARDIANS_OF_AKRASA,
    "383c9aa5-30ad-4a2a-8b64-65d4b333c613",
    "Alan Pollack",
);

// M13 19 — Healer of the Pride
pub(in crate::card::sets) static HEALER_OF_THE_PRIDE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Healer of the Pride",
    "35716e37-1bb2-41e2-bb55-e65126b01ce3",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Cat", "Cleric"], 2, 3).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control enters, you gain 2 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// M13 20 — Intrepid Hero (reprint)
const INTREPID_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::INTREPID_HERO,
    "43ec71e9-0024-4f8f-b499-541fb7607fcd",
    "Greg Hildebrandt",
);

// M13 21 — Knight of Glory
pub(in crate::card::sets) static KNIGHT_OF_GLORY: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Knight of Glory",
    "1646cb67-e0ac-4f2d-af21-618ff3613d69",
    "Peter Mohrbacher",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::Black),
        abilities::exalted(),
    ]),
);

// M13 22 — Oblivion Ring (reprint)
const OBLIVION_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::lorwyn::OBLIVION_RING,
    "1e2a73ec-39be-4d23-8c25-17d7c174dcee",
    "Franz Vohwinkel",
);

// M13 23 — Odric, Master Tactician
// Audit: unsupported — AttackDeclared can match the four-creature threshold, but combat has no procedure that lets the attacking player choose every block assignment.
pub(in crate::card::sets) static ODRIC_MASTER_TACTICIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Odric, Master Tactician",
    "bb1552a8-27b4-4a95-9022-6fdd59aca28f",
    "Michael Komarck",
    crate::card::CardRules::unsupported(),
);

// M13 24 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "f442e3b2-9d65-40c4-a3b9-8cb821980d80",
    "Robert Bliss",
);

// M13 25 — Pillarfield Ox (reprint)
const PILLARFIELD_OX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::PILLARFIELD_OX,
    "33e2f3ae-bf92-478b-9c63-acc3f175f02a",
    "Andrew Robinson",
);

// M13 26 — Planar Cleansing (reprint)
const PLANAR_CLEANSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::PLANAR_CLEANSING,
    "b5047b71-2359-4d9a-a168-a8eec43c5f1b",
    "Michael Komarck",
);

// M13 27 — Prized Elephant
static PRIZED_ELEPHANT_FORESTS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static PRIZED_ELEPHANT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Prized Elephant",
    "01597ede-94e7-44a4-93c2-7fd1db11e92a",
    "Ioan Dumitrescu",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Elephant"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Forest.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&PRIZED_ELEPHANT_FORESTS),
                    ValueDef::AnyMatchingObject(&PRIZED_ELEPHANT_FORESTS),
                ),
            },
        ),
        AbilityDef::activated(
            "{G}: This creature gains trample until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 28 — Rain of Blades (reprint)
const RAIN_OF_BLADES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::scourge::RAIN_OF_BLADES,
    "f3bd6ca4-c4ed-41c3-834c-23e0c1741b72",
    "Rob Alexander",
);

// M13 29 — Rhox Faithmender
pub(in crate::card::sets) static RHOX_FAITHMENDER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Rhox Faithmender",
    "85ea185a-7b38-49f3-be73-be8180fb6295",
    "Wesley Burt",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Rhino", "Monk"], 1, 5).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::replacement_for(
            "If you would gain life, you gain twice that much life instead.",
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            ReplacementEffectDef::MultiplyEventAmount(2),
        ),
    ]),
);

// M13 30 — Safe Passage (reprint)
const SAFE_PASSAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SAFE_PASSAGE,
    "9fc65c3f-ad29-4368-bf45-8345a7ec6f31",
    "Christopher Moeller",
);

// M13 31 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SERRA_ANGEL,
    "fe1e5de6-3f95-4e1c-99ae-574074998d5e",
    "Greg Staples",
);

// M13 32 — Serra Avatar (reprint)
const SERRA_AVATAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::SERRA_AVATAR,
    "10387b49-4978-4bb9-9139-2ddab3e184ea",
    "Dermot Power",
);

// M13 33 — Serra Avenger (reprint)
const SERRA_AVENGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tsp::SERRA_AVENGER,
    "aef0e34f-5065-46af-bea3-d748ca25707c",
    "Scott M. Fischer",
);

// M13 34 — Show of Valor
pub(in crate::card::sets) static SHOW_OF_VALOR: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Show of Valor",
    "abe4d19d-1c9f-4b05-bde2-a9290b52c28d",
    "Anthony Palumbo",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+4 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(4),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 35 — Silvercoat Lion (reprint)
const SILVERCOAT_LION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SILVERCOAT_LION,
    "9d33e866-cfd8-44e6-8070-df8df1ce965d",
    "Terese Nielsen",
);

// M13 36 — Sublime Archangel
pub(in crate::card::sets) static SUBLIME_ARCHANGEL: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Sublime Archangel",
    "f5cc38bc-55a4-446a-b054-48fb90216946",
    "Cynthia Sheppard",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 4, 3).with_abilities(&[
        abilities::flying(),
        abilities::exalted(),
        AbilityDef::static_ability(
            "Other creatures you control have exalted.",
            // Each granted copy is its own instance, so attacking alone into a board of
            // three other creatures is four separate triggers.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::exalted()),
            },
        ),
    ]),
);

// M13 37 — Touch of the Eternal
pub(in crate::card::sets) static TOUCH_OF_THE_ETERNAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Touch of the Eternal",
    "55c5f0c2-99e6-42b7-aa16-61d5815d060d",
    "Christopher Moeller",
    CardRules::new_enchantment(mana_cost!("{5}{W}{W}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, count the number of permanents you control. Your life total becomes that number.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::SetLifeTotal {
            recipient: EffectRecipientDef::Controller,
            total: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::Any,
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// M13 38 — War Falcon
pub(in crate::card::sets) static WAR_FALCON: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "War Falcon",
    "7e092a0d-c031-4a76-86c1-7f83878a06e8",
    "Volkan Baǵa",
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can't attack unless you control a Knight or a Soldier.",
            EffectDef::CannotAttackUnless(&ObjectQueryDef::matching(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype("Knight"),
                    ObjectPredicateDef::Subtype("Soldier"),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        ),
    ]),
);

// M13 39 — War Priest of Thune (reprint)
const WAR_PRIEST_OF_THUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::WAR_PRIEST_OF_THUNE,
    "d28eb320-aea7-466e-8718-de8652a2b191",
    "Izzy",
);

// M13 40 — Warclamp Mastiff
pub(in crate::card::sets) static WARCLAMP_MASTIFF: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Warclamp Mastiff",
    "102e48e0-8a5f-499d-ac62-005d3c075ef3",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{W}"), &["Dog"], 1, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// M13 41 — Archaeomancer
pub(in crate::card::sets) static ARCHAEOMANCER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Archaeomancer",
    "73c6d1be-55ad-4ee4-b044-88438e9b78cc",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 1, 2).with_ability(
        abilities::enters_trigger_with_targets("When this creature enters, return target instant or sorcery card from your graveyard to your hand.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })], EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
}),
    ),
);

// M13 42 — Arctic Aven
static PLAINS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static ARCTIC_AVEN: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Arctic Aven",
    "06f6aab1-c400-4d87-b68e-f36552e7417f",
    "Igor Kieryluk",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Bird", "Wizard"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Plains.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&PLAINS_YOU_CONTROL),
                    ValueDef::AnyMatchingObject(&PLAINS_YOU_CONTROL),
                ),
            },
        ),
        AbilityDef::activated(
            "{W}: This creature gains lifelink until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 43 — Augur of Bolas
pub(in crate::card::sets) static AUGUR_OF_BOLAS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Augur of Bolas",
    "2e6ec8a6-ad88-45c9-ab4b-dd7de2418bb7",
    "Slawomir Maniak",
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Merfolk", "Wizard"],
        1,
        3,
    )
    .with_ability(abilities::enters_trigger(
        "When this creature enters, look at the top three cards of your library. You may reveal an instant or sorcery card from among them and put it into your hand. Put the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_reveal_choice_to_hand_rest_bottom(
            ValueDef::Constant(3),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            0,
            1,
        ),
    )),
);

// M13 44 — Battle of Wits (reprint)
const BATTLE_OF_WITS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::BATTLE_OF_WITS,
    "b4be15a4-693f-4e22-a46c-38bb440c073c",
    "Jason Chan",
);

// M13 45 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CLONE,
    "6b2f0e3e-b209-4eda-81e5-b5e474a143d5",
    "Kev Walker",
);

// M13 46 — Courtly Provocateur
pub(in crate::card::sets) static COURTLY_PROVOCATEUR: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Courtly Provocateur",
    "ba912207-a8bf-4ffb-9967-34029cb09f7f",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target creature attacks this turn if able.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able(
                    "This creature attacks this turn if able.",
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Target creature blocks this turn if able.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBlockEachAttackerIfAble),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 47 — Divination (reprint)
const DIVINATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DIVINATION,
    "a3c573ab-9013-4c2b-a039-ce5b20dba264",
    "Howard Lyon",
);

// M13 48 — Downpour
pub(in crate::card::sets) static DOWNPOUR: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Downpour",
    "f220afb1-8638-4b54-b6af-0043b4cc1cef",
    "Eytan Zana",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap up to three target creatures.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            3,
        )],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// M13 49 — Encrust
pub(in crate::card::sets) static ENCRUST: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Encrust",
    "dfd05474-5cec-4c71-85e7-79cf25958525",
    "Jason Felix",
    // It answers an artifact as readily as a creature, which is the whole
    // reason to play it over an ordinary creature Aura.
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
            ),
            AbilityDef::static_ability(
                "Enchanted permanent doesn't untap during its controller's untap step and its \
                 activated abilities can't be activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Both halves for the same duration, so the Aura leaving returns the untap
                    // and the activations together.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
                    ]),
                },
            ),
        ]),
);

// M13 50 — Essence Scatter (reprint)
const ESSENCE_SCATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ESSENCE_SCATTER,
    "fcd965f9-bdaa-4434-a9c8-53fc57e997db",
    "Jon Foster",
);

// M13 51 — Faerie Invaders
pub(in crate::card::sets) static FAERIE_INVADERS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Faerie Invaders",
    "fcbc71b3-544b-4b81-8922-52744892989b",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Faerie", "Rogue"], 3, 3)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// M13 52 — Fog Bank (reprint)
const FOG_BANK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::FOG_BANK,
    "8a5a69dc-c6f3-459b-9dcd-b3363c26ca34",
    "Howard Lyon",
);

// M13 53 — Harbor Serpent (reprint)
const HARBOR_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::HARBOR_SERPENT,
    "af0f7357-08b0-403e-8913-8965662a905e",
    "Daarken",
);

// M13 54 — Hydrosurge
pub(in crate::card::sets) static HYDROSURGE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Hydrosurge",
    "1a22f992-ef16-45be-8bac-bd7418ed068f",
    "Steve Prescott",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -5/-0 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-5),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 55 — Index (reprint)
const INDEX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_apc::INDEX,
    "785fd0b8-7e98-44f5-8012-b9dadb31f9b0",
    "Kev Walker",
);

// M13 56 — Jace, Memory Adept (reprint)
const JACE_MEMORY_ADEPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::JACE_MEMORY_ADEPT,
    "96b2a335-2f01-4ba7-a037-453dbb1045e9",
    "D. Alexander Gregory",
);

// M13 57 — Jace's Phantasm
pub(in crate::card::sets) static JACE_S_PHANTASM: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Jace's Phantasm",
    "16829504-385c-4154-8e6d-f3fbaf273890",
    "Johann Bodin",
    CardRules::new_creature(mana_cost!("{U}"), &["Illusion"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature gets +4/+4 as long as an opponent has ten or more cards in their graveyard.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::Opponent,
                    )),
                    filter: None,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 10,
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                },
            }),
        ),
    ]),
);

// M13 58 — Kraken Hatchling (reprint)
const KRAKEN_HATCHLING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::KRAKEN_HATCHLING,
    "59a50590-9091-4632-bf8c-792e1e0a75a8",
    "Jason Felix",
);

// M13 59 — Master of the Pearl Trident
pub(in crate::card::sets) static MASTER_OF_THE_PEARL_TRIDENT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Master of the Pearl Trident",
    "e7decbd3-c754-451c-8d63-4f31f81412d2",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Other Merfolk creatures you control get +1/+1 and have islandwalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    // "Other Merfolk creatures you control": narrower than Lord of Atlantis,
                    // which reaches every Merfolk on the battlefield including the opponent's.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Merfolk"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::landwalk(BasicLandType::Island)),
                ]),
            },
        ),
    ),
);

// M13 60 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MERFOLK_OF_THE_PEARL_TRIDENT,
    "a360fe4e-c9a6-42fa-a97a-8b5a0c19ef93",
    "Ray Lago",
);

// M13 61 — Mind Sculpt
pub(in crate::card::sets) static MIND_SCULPT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Mind Sculpt",
    "5870d18e-0303-4722-b7f2-a751f8e372be",
    "Michael C. Hayes",
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent mills seven cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(7),
        },
    )),
);

// M13 62 — Negate (reprint)
const NEGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::morningtide::NEGATE,
    "8da17a86-3666-46b8-932e-daafd6a0cd69",
    "Jeremy Jarvis",
);

// M13 63 — Omniscience
pub(in crate::card::sets) static OMNISCIENCE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Omniscience",
    "1088f33e-cb5f-4248-ae8e-280c4e41f291",
    "Jason Chan",
    CardRules::new_enchantment(mana_cost!("{7}{U}{U}{U}")).with_ability(
        AbilityDef::static_ability(
            "You may cast spells from your hand without paying their mana costs.",
            EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
                spell: ObjectPredicateDef::Any,
                caster: PlayerRelation::You,
                zones: &[ZoneKind::Hand],
                cost: mana_cost!("{0}"),
            }),
        ),
    ),
);

// M13 64 — Redirect (reprint)
const REDIRECT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::REDIRECT,
    "0eef8431-f63c-44e0-940c-e1a38c338214",
    "Izzy",
);

// M13 65 — Rewind (reprint)
const REWIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::REWIND,
    "b09e1bb0-ffe8-4e5b-9a9a-f542ab439d3c",
    "Dermot Power",
);

// M13 66 — Scroll Thief (reprint)
const SCROLL_THIEF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::SCROLL_THIEF,
    "dc201a82-fb48-4bb4-b072-e206e6872aa5",
    "Alex Horley-Orlandelli",
);

// M13 67 — Sleep (reprint)
const SLEEP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SLEEP,
    "1e352497-1454-4917-b38c-4cc45424d876",
    "Chris Rahn",
);

// M13 68 — Spelltwine
// Audit: unsupported — Needs a card-copy effect for the two exiled instant or sorcery cards and mandatory free casting of both copies; CopyStackObject only copies spells already on the stack.
pub(in crate::card::sets) static SPELLTWINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Spelltwine",
    "e4d2f5ab-c6be-4661-843c-51b4977a9bea",
    "Noah Bradley",
    crate::card::CardRules::unsupported(),
);

// M13 69 — Sphinx of Uthuun (reprint)
const SPHINX_OF_UTHUUN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::SPHINX_OF_UTHUUN,
    "4462978c-0076-466b-a64b-0f54d09d4f27",
    "Kekai Kotaki",
);

// M13 70 — Stormtide Leviathan (reprint)
const STORMTIDE_LEVIATHAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m11::STORMTIDE_LEVIATHAN,
    "f9797351-eb0c-4774-8dbb-61a2404d66d9",
    "Karl Kopinski",
);

// M13 71 — Switcheroo
pub(in crate::card::sets) static SWITCHEROO: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Switcheroo",
    "7d62aaf3-0fd4-44ba-8eeb-18ac759dfe84",
    "Kev Walker",
    CardRules::new_sorcery(mana_cost!("{4}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Exchange control of two target creatures.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
        ],
        EffectDef::ExchangeControl {
            first: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            second: EffectRecipientDef::Target(TargetIndex(1)),
            otherwise: None,
        },
    )),
);

// M13 72 — Talrand, Sky Summoner
pub(in crate::card::sets) static TALRAND_SKY_SUMMONER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Talrand, Sky Summoner",
    "bc1a6867-921d-4912-afae-c3c445ad81e7",
    "Svetlin Velinov",
    CardRules::new_creature(
        mana_cost!("{2}{U}{U}"),
        &["Merfolk", "Wizard"],
        2,
        2,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_ability(AbilityDef::triggered(
        "Whenever you cast an instant or sorcery spell, create a 2/2 blue Drake creature token with flying.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
        ])),
        EffectDef::create_creature_token(&["Drake"], &[ManaColor::Blue], 2, 2).with_abilities(&[abilities::flying()]).with_art(CardArt::new("93679bb9-ee1c-4eea-bcdd-72785d5788af", "Svetlin Velinov")),
    )),
);

// M13 73 — Talrand's Invocation
pub(in crate::card::sets) static TALRANDS_INVOCATION: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Talrand's Invocation",
    "c2cd809c-557a-42a5-950b-56b5b47b325b",
    "Svetlin Velinov",
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell(
        "Create two 2/2 blue Drake creature tokens with flying.",
        EffectDef::create_creature_token(&["Drake"], &[ManaColor::Blue], 2, 2)
            .with_abilities(&[abilities::flying()])
            .with_art(CardArt::new(
                "93679bb9-ee1c-4eea-bcdd-72785d5788af",
                "Svetlin Velinov",
            ))
            .with_amount(2),
    )),
);

// M13 74 — Tricks of the Trade
pub(in crate::card::sets) static TRICKS_OF_THE_TRADE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Tricks of the Trade",
    "8c796ef9-4061-4f82-9ee9-3bc446804ee9",
    "Steven Belledin",
    CardRules::new_enchantment(mana_cost!("{3}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+0 and can't be blocked.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                            ObjectPredicateDef::Any,
                        )),
                    ]),
                },
            ),
        ]),
);

// M13 75 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::UNSUMMON,
    "84402e82-bae7-470a-8b2f-929dac888018",
    "Izzy",
);

// M13 76 — Vedalken Entrancer (reprint)
const VEDALKEN_ENTRANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::VEDALKEN_ENTRANCER,
    "dc4bbd25-5ddd-4502-b582-b7d89c9f97a5",
    "Dan Murayama Scott",
);

// M13 77 — Void Stalker
// Audit: unsupported — CombineObjects and MoveObjects can move source and target together, but ShuffleLibrary cannot project and deduplicate the moved cards' owners.
pub(in crate::card::sets) static VOID_STALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Void Stalker",
    "7fc30e31-4796-4e98-992c-a56cd51ad3c9",
    "Marco Nelor",
    crate::card::CardRules::unsupported(),
);

// M13 78 — Watercourser
pub(in crate::card::sets) static WATERCOURSER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Watercourser",
    "a27c441a-b31d-4214-8fc5-054003e257dc",
    "Mathias Kollros",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental"], 2, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M13 79 — Welkin Tern (reprint)
const WELKIN_TERN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::WELKIN_TERN,
    "9a3c6dc6-4a16-4a01-822e-353ff84b363c",
    "Austin Hsu",
);

// M13 80 — Wind Drake (reprint)
const WIND_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::WIND_DRAKE,
    "c9dcb8d2-0da9-40fc-b0c0-2c76b3d277bc",
    "Steve Prescott",
);

// M13 81 — Blood Reckoning
pub(in crate::card::sets) static BLOOD_RECKONING: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Blood Reckoning",
    "24577bb2-61b0-4675-84e6-5d675b28fc0e",
    "Wayne Reynolds",
    CardRules::new_enchantment(mana_cost!("{3}{B}")).with_ability(AbilityDef::triggered(
        "Whenever a creature attacks you or a planeswalker you control, that creature's controller loses 1 life.",
        TriggerEventDef::attacks(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(1),
        },
    )),
);

// M13 82 — Bloodhunter Bat
pub(in crate::card::sets) static BLOODHUNTER_BAT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Bloodhunter Bat",
    "99c10705-6e0e-46f6-a64c-0095b2796aaf",
    "Tomasz Jedruszek",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Bat"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target player loses 2 life and you gain 2 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// M13 83 — Bloodthrone Vampire (reprint)
const BLOODTHRONE_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::BLOODTHRONE_VAMPIRE,
    "7c0b87e0-d5e4-44f2-8220-325443ee9f31",
    "Steve Argyle",
);

// M13 84 — Cower in Fear
pub(in crate::card::sets) static COWER_IN_FEAR: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Cower in Fear",
    "bf2d53b8-7847-4b94-9711-eca29facccba",
    "Nils Hamm",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::spell(
        "Creatures your opponents control get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 85 — Crippling Blight
pub(in crate::card::sets) static CRIPPLING_BLIGHT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Crippling Blight",
    "6a96a5d6-6527-4018-923e-7e850fda106a",
    "Lucas Graciano",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -1/-1 and can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-1),
                            ValueDef::Constant(-1),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            ),
        ]),
);

// M13 86 — Dark Favor (reprint)
const DARK_FAVOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::DARK_FAVOR,
    "5aae919b-7da6-42b1-84b4-fbc2971dad1e",
    "Allen Williams",
);

// M13 87 — Diabolic Revelation
pub(in crate::card::sets) static DIABOLIC_REVELATION: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Diabolic Revelation",
    "145d6d7b-1e87-47b7-baf3-d201458ad996",
    "Raymond Swanland",
    // "Up to X" with no qualifier on the cards: whatever X buys, it buys
    // exactly, and a smaller library is its own ceiling.
    CardRules::new_sorcery(mana_cost!("{X}{3}{B}{B}")).with_ability(AbilityDef::spell(
        "Search your library for up to X put those cards into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 0,
            maximum: ValueDef::ChosenX,
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// M13 88 — Disciple of Bolas
pub(in crate::card::sets) static DISCIPLE_OF_BOLAS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Disciple of Bolas",
    "c4dd57f8-27bc-4ad9-a79e-48a68af33b02",
    "Slawomir Maniak",
    CardRules::new_creature(
        mana_cost!("{3}{B}"),
        &["Human", "Wizard"],
        2,
        1,
    )
    .with_ability(abilities::enters_trigger("When this creature enters, sacrifice another creature. You gain X life and draw X where X is that creature's power.", EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                // "Another" creature, so the Disciple cannot eat itself.
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            // X is read off the sacrificed creature, so both halves take the power the
            // sacrifice recorded rather than counting anything on the board.
            then: Some(&EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ])),
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        })),
);

// M13 89 — Disentomb (reprint)
const DISENTOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DISENTOMB,
    "ce7473bb-d092-4d76-b3c3-5036222dbdf7",
    "Alex Horley-Orlandelli",
);

// M13 90 — Duress (reprint)
const DURESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::DURESS,
    "f7201d43-ae2e-4faa-a508-8555079c3bc7",
    "Steven Belledin",
);

// M13 91 — Duskmantle Prowler
pub(in crate::card::sets) static DUSKMANTLE_PROWLER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Duskmantle Prowler",
    "bcb031da-d41a-496a-b78e-0773f6504303",
    "Johannes Voss",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Rogue"], 2, 2)
        .with_abilities(&[abilities::haste(), abilities::exalted()]),
);

// M13 92 — Duty-Bound Dead
pub(in crate::card::sets) static DUTY_BOUND_DEAD: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Duty-Bound Dead",
    "5150aa90-1284-4261-8625-2528139f0015",
    "Johannes Voss",
    // A 0/2 that regenerates: exalted is what gives it something to do on
    // the attack, since alone it is the creature being pumped.
    CardRules::new_creature(mana_cost!("{B}"), &["Skeleton"], 0, 2).with_abilities(&[
        abilities::exalted(),
        abilities::regenerate_self(
            "{3}{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{B}"))],
        ),
    ]),
);

// M13 93 — Essence Drain (reprint)
const ESSENCE_DRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::darksteel::ESSENCE_DRAIN,
    "58df0c6d-3fd2-4d87-81e2-6640e6e75985",
    "Jim Nelson",
);

// M13 94 — Giant Scorpion (reprint)
const GIANT_SCORPION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::GIANT_SCORPION,
    "4097d5dc-46d3-4054-818f-a4ad8d7effe2",
    "Raymond Swanland",
);

// M13 95 — Harbor Bandit
static HARBOR_BANDIT_ISLANDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static HARBOR_BANDIT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Harbor Bandit",
    "8422e109-de8d-46ea-a7f8-d5ccb6340497",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control an Island.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&HARBOR_BANDIT_ISLANDS),
                    ValueDef::AnyMatchingObject(&HARBOR_BANDIT_ISLANDS),
                ),
            },
        ),
        AbilityDef::activated(
            "{1}{U}: This creature can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 96 — Knight of Infamy
pub(in crate::card::sets) static KNIGHT_OF_INFAMY: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Knight of Infamy",
    "9e339853-5b6b-47b7-8d88-e9d3befb803f",
    "Peter Mohrbacher",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::White),
        abilities::exalted(),
    ]),
);

// M13 97 — Liliana of the Dark Realms
/// The negative branch of Liliana's −3 reads the same Swamp count for both
/// power and toughness.
static NEGATIVE_SWAMPS_YOU_CONTROL: ValueDef = ValueDef::Negate(&SWAMPS_YOU_CONTROL);

pub(in crate::card::sets) static LILIANA_OF_THE_DARK_REALMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Liliana of the Dark Realms",
    "2cd2d81e-1388-4f34-9917-2289971cf8da",
    "D. Alexander Gregory",
    CardRules::new_planeswalker(mana_cost!("{2}{B}{B}"), &["Liliana"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Search your library for a Swamp card, reveal it, put it into your hand, then shuffle.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::activated_with_targets(
                "−3: Target creature gets +X/+X or -X/-X until end of turn, where X is the number of Swamps you control.",
                &[AbilityCostDef::Loyalty(-3)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Give +X/+X",
                            effect: EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                effect: AppliedEffectDef::modify_power_toughness(
                                    SWAMPS_YOU_CONTROL,
                                    SWAMPS_YOU_CONTROL,
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        },
                        EffectChoiceDef {
                            label: "Give -X/-X",
                            effect: EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                effect: AppliedEffectDef::modify_power_toughness(
                                    NEGATIVE_SWAMPS_YOU_CONTROL,
                                    NEGATIVE_SWAMPS_YOU_CONTROL,
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        },
                    ],
                },
            ),
            AbilityDef::activated(
                "−6: You get an emblem with \"Swamps you control have '{T}: Add {B}{B}{B}{B}.'\"",
                &[AbilityCostDef::Loyalty(-6)],
                EffectDef::create_emblem(
                    "Liliana of the Dark Realms emblem",
                    &[AbilityDef::static_ability(
                        "Swamps you control have '{T}: Add {B}{B}{B}{B}.'",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                                "{T}: Add {B}{B}{B}{B}.",
                                &[AbilityCostDef::TapSource],
                                EffectDef::AddMana(
                                    AddManaEffectDef::one(ManaColor::Black).with_amount(4),
                                ),
                            )),
                        },
                    )],
                ),
            ),
        ]),
);

// M13 98 — Liliana's Shade
pub(in crate::card::sets) static LILIANAS_SHADE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Liliana's Shade",
    "1cf0c01d-a4a0-43fb-970d-e428e9ac63d7",
    "Eric Deschamps",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Shade"], 1, 1).with_abilities(&[
        abilities::enters_trigger("When this creature enters, you may search your library for a Swamp card, reveal it, put it into your hand, then shuffle.", EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            }),
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 99 — Mark of the Vampire
pub(in crate::card::sets) static MARK_OF_THE_VAMPIRE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Mark of the Vampire",
    "90484815-2529-4a81-9f1b-f0f7382e4b66",
    "Winona Nelson",
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
        ]),
);

// M13 100 — Mind Rot (reprint)
const MIND_ROT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::MIND_ROT,
    "ab454fb8-347f-4d4d-84bb-195c9d51b06b",
    "Steve Luke",
);

// M13 101 — Murder
pub(in crate::card::sets) static MURDER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Murder",
    "c8676f02-cf1e-4d40-a0c5-6e5a97417898",
    "Allen Williams",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
        true,
    )),
);

// M13 102 — Mutilate (reprint)
const MUTILATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::torment::MUTILATE,
    "c48bc86b-df0a-4a9c-8aad-c3ffb742a5ff",
    "Tyler Jacobson",
);

// M13 103 — Nefarox, Overlord of Grixis
pub(in crate::card::sets) static NEFAROX_OVERLORD_OF_GRIXIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Nefarox, Overlord of Grixis",
    "abc382f3-fdb9-4987-acf4-bf1ac4fd2ef7",
    "Aleksi Briclot",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Demon"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::exalted(),
            AbilityDef::triggered(
                "Whenever this creature attacks alone, defending player sacrifices a creature of their choice.",
                TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 1, Some(1)),
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Opponent,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
        ]),
);

// M13 104 — Phylactery Lich (reprint)
const PHYLACTERY_LICH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m11::PHYLACTERY_LICH,
    "4aefd084-548d-4326-901e-832a1b5f5391",
    "Michael Komarck",
);

// M13 105 — Public Execution
// Audit: unsupported — A target-relative creature sweep cannot exclude the destroyed target when destruction is prevented or replaced.
pub(in crate::card::sets) static PUBLIC_EXECUTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Public Execution",
    "48188942-d0ba-4503-bd75-c7a5329bb7c8",
    "Anthony Palumbo",
    crate::card::CardRules::unsupported(),
);

// M13 106 — Ravenous Rats (reprint)
const RAVENOUS_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::portal_second_age::RAVENOUS_RATS,
    "0642111c-f668-4acb-9df5-f0b920352407",
    "Carl Critchlow",
);

// M13 107 — Rise from the Grave (reprint)
const RISE_FROM_THE_GRAVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::RISE_FROM_THE_GRAVE,
    "5d2b187e-c489-4652-a638-390fc9ecef0e",
    "Vance Kovacs",
);

// M13 108 — Servant of Nefarox
pub(in crate::card::sets) static SERVANT_OF_NEFAROX: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Servant of Nefarox",
    "e00a2b22-a473-44ae-919f-29bc8be05543",
    "Igor Kieryluk",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Cleric"], 3, 1)
        .with_ability(abilities::exalted()),
);

// M13 109 — Shimian Specter (reprint)
const SHIMIAN_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fut::SHIMIAN_SPECTER,
    "1ca9e99b-e46c-4f7a-9c51-e2cfe8810450",
    "Anthony S. Waters",
);

// M13 110 — Sign in Blood (reprint)
const SIGN_IN_BLOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SIGN_IN_BLOOD,
    "64f6600b-36c4-43bd-8c01-cfbca402ecd6",
    "Howard Lyon",
);

// M13 111 — Tormented Soul (reprint)
const TORMENTED_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::TORMENTED_SOUL,
    "87810963-9c62-4ff2-b33b-51fcc1b628ac",
    "Karl Kopinski",
);

// M13 112 — Vampire Nighthawk (reprint)
const VAMPIRE_NIGHTHAWK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::VAMPIRE_NIGHTHAWK,
    "9ba96d96-8d9e-47c8-ab39-17479564aadf",
    "Jason Chan",
);

// M13 113 — Vampire Nocturnus (reprint)
const VAMPIRE_NOCTURNUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::VAMPIRE_NOCTURNUS,
    "8daccbbb-6600-4467-810f-277f01a11771",
    "Raymond Swanland",
);

// M13 114 — Veilborn Ghoul
pub(in crate::card::sets) static VEILBORN_GHOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Veilborn Ghoul",
    "d3f49232-2853-427f-8c20-322e09a3ccde",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie"], 4, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        AbilityDef::triggered(
            "Whenever a Swamp you control enters, you may return this card from your graveyard to your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// M13 115 — Vile Rebirth
pub(in crate::card::sets) static VILE_REBIRTH: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Vile Rebirth",
    "965b5a48-d0ff-47ce-b44e-a1611fab1876",
    "Erica Yang",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature card from a graveyard. Create a 2/2 black Zombie creature token.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
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
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(
                CardArt::new("1966d7e6-cd4a-47ff-bc3e-f8e0db8a3439", "Lucas Graciano"),
            ),
        ]),
    )),
);

// M13 116 — Walking Corpse (reprint)
const WALKING_CORPSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &innistrad::WALKING_CORPSE,
    "5ecfc1ab-b7a1-43a8-b1d1-0c1c4358e89f",
    "Igor Kieryluk",
);

// M13 117 — Wit's End (reprint)
const WITS_END_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::dissension::WITS_END,
    "71298c75-533e-4ccd-a1f5-875f63a1e89b",
    "Chris Rahn",
);

// M13 118 — Xathrid Gorgon
pub(in crate::card::sets) static XATHRID_GORGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Xathrid Gorgon",
    "e07524e0-303d-465d-b112-ca605b9b27fc",
    "Chase Stone",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Gorgon"], 3, 6).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::activated_with_targets(
            "{2}{B}, {T}: Put a petrification counter on target creature. It gains defender and becomes a colorless artifact in addition to its other types. Its activated abilities can't be activated.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::named("petrification"),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::defender()),
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Artifact)),
                        AppliedEffectDef::set_colors(ColorSet::empty()),
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        ),
    ]),
);

// M13 119 — Zombie Goliath (reprint)
const ZOMBIE_GOLIATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ZOMBIE_GOLIATH,
    "8638edec-ddcd-4f50-9c2f-2e1668e3d175",
    "E. M. Gist",
);

// M13 120 — Arms Dealer (reprint)
const ARMS_DEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::mercadian_masques::ARMS_DEALER,
    "910d3c33-8cda-487b-8b44-87a9d06d6749",
    "Wayne Reynolds",
);

// M13 121 — Bladetusk Boar (reprint)
const BLADETUSK_BOAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::BLADETUSK_BOAR,
    "d28442f9-06cf-4273-80a3-2b054f5881a4",
    "Paul Bonner",
);

// M13 122 — Canyon Minotaur (reprint)
const CANYON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::conflux::CANYON_MINOTAUR,
    "f8dc0efb-5847-4061-b386-9b4099361a58",
    "Steve Prescott",
);

// M13 123 — Chandra, the Firebrand (reprint)
const CHANDRA_THE_FIREBRAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::CHANDRA_THE_FIREBRAND,
    "beb039db-7367-4af1-8d85-4951f58e2732",
    "D. Alexander Gregory",
);

// M13 124 — Chandra's Fury
pub(in crate::card::sets) static CHANDRAS_FURY: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Chandra's Fury",
    "25335fee-d320-4622-bcf4-292400dee52b",
    "Volkan Baǵa",
    CardRules::new_instant(mana_cost!("{4}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Chandra's Fury deals 4 damage to target player or planeswalker and 1 damage to each creature that player or that planeswalker's controller controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::objects_controlled_by_target(ObjectPredicateDef::HasType(CardType::Creature), TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// M13 125 — Cleaver Riot
pub(in crate::card::sets) static CLEAVER_RIOT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Cleaver Riot",
    "6761eacf-03fc-4ccd-a4a6-eca5357b5c5b",
    "Dave Kendall",
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell(
        "Creatures you control gain double strike until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 126 — Craterize
pub(in crate::card::sets) static CRATERIZE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Craterize",
    "e5459409-5103-4a97-a6fb-3e3ab896eb66",
    "Eytan Zana",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target land.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Land)),
        true,
    )),
);

// M13 127 — Crimson Muckwader
static CRIMSON_MUCKWADER_SWAMPS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static CRIMSON_MUCKWADER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Crimson Muckwader",
    "a0811f91-ed92-4a8e-badd-ae5054e7707d",
    "Steven Belledin",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Lizard"], 2, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Swamp.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&CRIMSON_MUCKWADER_SWAMPS),
                    ValueDef::AnyMatchingObject(&CRIMSON_MUCKWADER_SWAMPS),
                ),
            },
        ),
        abilities::regenerate_self(
            "{2}{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{B}"))],
        ),
    ]),
);

// M13 128 — Dragon Hatchling
pub(in crate::card::sets) static DRAGON_HATCHLING: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Dragon Hatchling",
    "ed599d52-f2d9-4913-ad88-70f8aa4af7b9",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dragon"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 129 — Fervor (reprint)
const FERVOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::weatherlight::FERVOR,
    "a88515c2-4b4f-4d16-9f50-149ef012e961",
    "Wayne England",
);

// M13 130 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FIRE_ELEMENTAL,
    "d39716c6-6c4f-4cd3-9d9c-893f883e6e70",
    "Slawomir Maniak",
);

// M13 131 — Firewing Phoenix
pub(in crate::card::sets) static FIREWING_PHOENIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Firewing Phoenix",
    "b8824674-ced2-448e-9bf0-03c1c43a5315",
    "James Paick",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Phoenix"], 4, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{R}{R}{R}: Return this card from your graveyard to your hand.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}{R}{R}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// M13 132 — Flames of the Firebrand
pub(in crate::card::sets) static FLAMES_OF_THE_FIREBRAND: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Flames of the Firebrand",
    "aca215b1-7b98-49ce-afae-eeb61058125a",
    "Steve Argyle",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Flames of the Firebrand deals 3 damage divided as you choose among one, two, or three targets.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                // One, two, or three targets is not a separate rule: three damage
                // split with every share at least one says the same thing.
                minimum: 1,
                maximum: 3,
                exact_count: None,
                divided_total: Some(DividedTotal::Fixed(3)),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ),
);

// M13 133 — Furnace Whelp (reprint)
const FURNACE_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::fifth_dawn::FURNACE_WHELP,
    "41e73d9c-8c17-4c3c-b535-e21f03e577bc",
    "Matt Cavotta",
);

// M13 134 — Goblin Arsonist (reprint)
const GOBLIN_ARSONIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::GOBLIN_ARSONIST,
    "4d131369-db00-4a11-bd47-4401188b0f35",
    "Wayne Reynolds",
);

// M13 135 — Goblin Battle Jester
pub(in crate::card::sets) static GOBLIN_BATTLE_JESTER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Goblin Battle Jester",
    "c13e56b0-becc-4bc2-9ba3-23b3ca8bfe58",
    "Steve Prescott",
    // The Jester itself is a red spell, but the trigger watches casts rather
    // than arrivals, so casting it does not fire its own ability.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever you cast a red spell, target creature can't block this turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Color(ManaColor::Red),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M13 136 — Hamletback Goliath (reprint)
const HAMLETBACK_GOLIATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::lorwyn::HAMLETBACK_GOLIATH,
    "01ddeef1-f6f9-48c0-a93c-7bb3877c0e59",
    "Paolo Parente & Brian Snõddy",
);

// M13 137 — Kindled Fury (reprint)
const KINDLED_FURY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::morningtide::KINDLED_FURY,
    "35494897-b72b-46c4-8b36-b3b8865559bd",
    "Wayne Reynolds",
);

// M13 138 — Krenko, Mob Boss
pub(in crate::card::sets) static KRENKO_MOB_BOSS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Krenko, Mob Boss",
    "aa078518-0ce2-4c6f-9061-aa7e22ed7493",
    "Karl Kopinski",
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}"),
        &["Goblin", "Warrior"],
        3,
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_ability(AbilityDef::activated(
        "{T}: Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control.",
        &[AbilityCostDef::TapSource],
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_art(CardArt::new("0e67efea-8a80-42ec-8e77-07d387d933d4", "Karl Kopinski")).with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype("Goblin"),
            ]),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ))),
    )),
);

// M13 139 — Krenko's Command
pub(in crate::card::sets) static KRENKOS_COMMAND: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Krenko's Command",
    "84df41e9-e973-4441-b17f-434517134d46",
    "Karl Kopinski",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Create two 1/1 red Goblin creature tokens.",
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
            .with_art(CardArt::new(
                "0e67efea-8a80-42ec-8e77-07d387d933d4",
                "Karl Kopinski",
            ))
            .with_amount(2),
    )),
);

// M13 140 — Magmaquake
pub(in crate::card::sets) static MAGMAQUAKE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Magmaquake",
    "ac85679e-17c7-4525-8eed-979d04feb8f1",
    "Gabor Szikszai",
    CardRules::new_instant(mana_cost!("{X}{R}{R}")).with_ability(AbilityDef::spell(
        "Magmaquake deals X damage to each creature without flying and each planeswalker.",
        // One sweep rather than two, so a permanent that is both a nonflying
        // creature and a planeswalker is dealt X once.
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::ChosenX,
        },
    )),
);

// M13 141 — Mark of Mutiny (reprint)
const MARK_OF_MUTINY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::MARK_OF_MUTINY,
    "0b7c6e09-3a14-4cc4-ba6b-f1f45e7d9f2a",
    "Mike Bierek",
);

// M13 142 — Mindclaw Shaman
pub(in crate::card::sets) static MINDCLAW_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Mindclaw Shaman",
    "0f342fe9-aa73-4222-b908-d4035b5746be",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Lizard", "Shaman"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent reveals their hand. You may cast an instant or sorcery spell from among those cards without paying its mana cost.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))],
            EffectDef::Sequence(&abilities::reveal_hand_and_choose_card(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                &EffectDef::MayPlayWithoutPaying(FreePlayDef {
                    objects: ObjectSetDef::One(ObjectRefDef::Binding(ParentBinding)),
                    duration: FreePlayDurationDef::WhileResolving,
                    mandatory: false,
                    grants_haste: false,
                }),
            )),
        ),
    ),
);

// M13 143 — Mogg Flunkies (reprint)
const MOGG_FLUNKIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::MOGG_FLUNKIES,
    "ed2f1b5d-1b16-4f35-9cce-2f089905fddd",
    "Brom",
);

// M13 144 — Reckless Brute
pub(in crate::card::sets) static RECKLESS_BRUTE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Reckless Brute",
    "5fd32a9e-1d39-4792-9657-69d17e5e0134",
    "Johann Bodin",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ogre", "Warrior"], 3, 1).with_abilities(&[
        abilities::haste(),
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    ]),
);

// M13 145 — Reverberate (reprint)
const REVERBERATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::REVERBERATE,
    "5996feb4-02ac-45e8-a7f2-966cf74391dc",
    "jD",
);

// M13 146 — Rummaging Goblin
pub(in crate::card::sets) static RUMMAGING_GOBLIN: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Rummaging Goblin",
    "cc5b622c-83a4-477e-a99c-2674e2bd6bb9",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}, Discard a card: Draw a card.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 147 — Searing Spear
pub(in crate::card::sets) static SEARING_SPEAR: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Searing Spear",
    "11a94b7c-0216-473c-87a6-71e5a64d7799",
    "Chris Rahn",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Searing Spear deals 3 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )),
);

// M13 148 — Slumbering Dragon
pub(in crate::card::sets) static SLUMBERING_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Slumbering Dragon",
    "277cbd0d-c8da-4a37-965c-6a60771df2f7",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{R}"), &["Dragon"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can't attack or block unless it has five or more +1/+1 counters on it.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::Less,
                    amount: 5,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature attacks you or a planeswalker you control, put a +1/+1 counter on this creature.",
            TriggerEventDef::Attacks(AttackEventMatcherDef::attacking(
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                PlayerRelation::You,
            )),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M13 149 — Smelt
pub(in crate::card::sets) static SMELT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Smelt",
    "723cb7e3-3f48-41fa-aa08-bdc59225e44f",
    "Zoltan Boros",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Artifact)),
        true,
    )),
);

// M13 150 — Thundermaw Hellkite
/// The damage and the tap name the same creatures, so both clauses ask the
/// same question.
const OPPOSING_FLIERS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
]);

pub(in crate::card::sets) static THUNDERMAW_HELLKITE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Thundermaw Hellkite",
    "d0476e0f-61df-46a6-aaf1-8ee79c701160",
    "Svetlin Velinov",
    CardRules::new_creature(
        mana_cost!("{3}{R}{R}"),
        &["Dragon"],
        5,
        5,
    )
    .with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        abilities::enters_trigger("When this creature enters, it deals 1 damage to each creature with flying your opponents control. Tap those creatures.", EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::matching_objects(OPPOSING_FLIERS, &[ZoneKind::Battlefield], PlayerRelation::Opponent),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::matching_objects(OPPOSING_FLIERS, &[ZoneKind::Battlefield], PlayerRelation::Opponent),
                },
            ])),
    ]),
);

// M13 151 — Torch Fiend (reprint)
const TORCH_FIEND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &dark_ascension::TORCH_FIEND,
    "cbd53740-43bb-4ea2-aa01-937a5786ccda",
    "Winona Nelson",
);

// M13 152 — Trumpet Blast (reprint)
const TRUMPET_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_destiny::TRUMPET_BLAST,
    "4ac9f745-236a-4302-acf2-21c14c6e6eab",
    "Carl Critchlow",
);

// M13 153 — Turn to Slag (reprint)
const TURN_TO_SLAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::scars_of_mirrodin::TURN_TO_SLAG,
    "7275ede4-22d6-41db-91e9-3b0295abb8a9",
    "Zoltan Boros & Gabor Szikszai",
);

// M13 154 — Volcanic Geyser (reprint)
const VOLCANIC_GEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::VOLCANIC_GEYSER,
    "df5bab70-3c28-48db-9ed3-64706f64f4fa",
    "Clint Cearley",
);

// M13 155 — Volcanic Strength (reprint)
const VOLCANIC_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::VOLCANIC_STRENGTH,
    "f1963f08-1765-4f3e-92be-479773de47a0",
    "Izzy",
);

// M13 156 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WALL_OF_FIRE,
    "b242e0b6-76c8-4cc6-b914-1dc7842d5a9c",
    "Dan Dos Santos",
);

// M13 157 — Wild Guess
pub(in crate::card::sets) static WILD_GUESS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Wild Guess",
    "a4e513b8-25c2-4645-abcc-a6e9d5f51e09",
    "Lucas Graciano",
    CardRules::new_sorcery(mana_cost!("{R}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a card.\nDraw two cards.",
            &[],
            SpellAdditionalCostDef::discard(ObjectPredicateDef::Any, CostQuantityDef::Fixed(1)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// M13 158 — Worldfire
pub(in crate::card::sets) static WORLDFIRE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Worldfire",
    "2ef3d4b5-0453-4bf0-b018-23b0c3b9ae11",
    "Izzy",
    CardRules::new_sorcery(mana_cost!("{6}{R}{R}{R}")).with_ability(AbilityDef::spell(
        "Exile all permanents. Exile all cards from all hands and graveyards. Each player's life total becomes 1.",
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand, ZoneKind::Graveyard],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::SetLifeTotal {
                recipient: EffectRecipientDef::players(PlayerSetDef::All),
                total: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M13 159 — Acidic Slime (reprint)
const ACIDIC_SLIME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ACIDIC_SLIME,
    "bd7bef5a-e0ab-46d3-a802-620bf2a7546f",
    "Karl Kopinski",
);

// M13 160 — Arbor Elf (reprint)
const ARBOR_ELF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::worldwake::ARBOR_ELF,
    "b7d6b117-0c14-4455-92fc-29555ee75d97",
    "rk post",
);

// M13 161 — Bond Beetle
pub(in crate::card::sets) static BOND_BEETLE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Bond Beetle",
    "f341ed2c-353b-49a3-b200-94ae43cb8e24",
    "John Avon",
    CardRules::new_creature(mana_cost!("{G}"), &["Insect"], 0, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 162 — Boundless Realms
pub(in crate::card::sets) static BOUNDLESS_REALMS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Boundless Realms",
    "e3c3cf16-ba81-4558-b1a6-79942a02f629",
    "Cliff Childs",
    // It doubles the lands you have, so the seven it costs is measured
    // against a board that is already large.
    CardRules::new_sorcery(mana_cost!("{6}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to X basic land where X is the number of lands you control, put them onto the battlefield tapped, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            // The lands you already control, which is what the search is sized by.
            maximum: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
            reveal: true,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// M13 163 — Bountiful Harvest (reprint)
const BOUNTIFUL_HARVEST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::BOUNTIFUL_HARVEST,
    "8d7a4494-2ced-4405-9204-d2617961a1d6",
    "Jason Chan",
);

// M13 164 — Centaur Courser (reprint)
const CENTAUR_COURSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::CENTAUR_COURSER,
    "44a5f7db-ea4e-4af5-9d4a-0335db6ea0e9",
    "Vance Kovacs",
);

// M13 165 — Deadly Recluse (reprint)
const DEADLY_RECLUSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DEADLY_RECLUSE,
    "a32a5f77-7c1f-4da4-9ae6-3947504a8dea",
    "Warren Mahy",
);

// M13 166 — Duskdale Wurm (reprint)
const DUSKDALE_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::eventide::DUSKDALE_WURM,
    "7d1a2d9a-e14c-4c44-8cf1-a2ce09bdae27",
    "Dan Dos Santos",
);

// M13 167 — Elderscale Wurm
pub(in crate::card::sets) static ELDERSCALE_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Elderscale Wurm",
    "20f3f63d-0f04-4945-9895-940c916a2547",
    "Richard Wright",
    CardRules::new_creature(mana_cost!("{4}{G}{G}{G}"), &["Wurm"], 7, 7).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_if(
            "When this creature enters, if your life total is less than 7, your life total becomes 7.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ControllerLifeAtMost(6),
            EffectDef::SetLifeTotal {
                recipient: EffectRecipientDef::Controller,
                total: ValueDef::Constant(7),
            },
        ),
        AbilityDef::static_ability(
            "As long as you have 7 or more life, damage that would reduce your life total to less than 7 reduces it to 7 instead.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(
                    &TriggerConditionDef::ControllerLifeAtMost(6),
                ),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage {
                        matcher: DamageEventMatcherDef {
                            kind: DamageKindDef::Any,
                            source: DamageSourceMatcherDef::Any,
                            recipient: DamageRecipientMatcherDef::Any,
                        },
                        limit: DamageLimitDef::LeaveAtLeastLife(7),
                    }),
                },
            },
        ),
    ]),
);

// M13 168 — Elvish Archdruid (reprint)
const ELVISH_ARCHDRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ELVISH_ARCHDRUID,
    "bf8eba57-8c51-490b-995f-53eeb7ad574f",
    "Karl Kopinski",
);

// M13 169 — Elvish Visionary (reprint)
const ELVISH_VISIONARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shards_of_alara::ELVISH_VISIONARY,
    "65ea2998-ed91-43b8-bd81-b01a6c24a5b0",
    "D. Alexander Gregory",
);

// M13 170 — Farseek (reprint)
const FARSEEK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::FARSEEK,
    "f9b69d33-96dd-4844-aefa-27a885cb2ffc",
    "Martina Pilcerova",
);

// M13 171 — Flinthoof Boar
/// A second Mountain does not make the bonus bigger, so this is asked as a
/// condition rather than counted.
static MOUNTAIN_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Mountain"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static FLINTHOOF_BOAR: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Flinthoof Boar",
    "7e380b99-0173-4083-a4a2-222ad98b904a",
    "Erica Yang",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Boar"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Mountain.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                    ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                ),
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gains haste until end of turn. (It can attack and {T} this turn.)",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 172 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOG,
    "17d591ad-4f3d-4cc6-a888-e30b46ee0771",
    "Jaime Jones",
);

// M13 173 — Fungal Sprouting
pub(in crate::card::sets) static FUNGAL_SPROUTING: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Fungal Sprouting",
    "97413ae3-037e-4786-85a3-e92604acd771",
    "Brad Rigney",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Create X 1/1 green Saproling creature tokens, where X is the greatest power among creatures you control.",
        EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1).with_art(CardArt::new("dd67de8a-3879-4d03-a716-6e907d597b25", "Brad Rigney")).with_count(abilities::greatest_power_you_control()),
    )),
);

// M13 174 — Garruk, Primal Hunter (reprint)
const GARRUK_PRIMAL_HUNTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::GARRUK_PRIMAL_HUNTER,
    "9945307b-d49d-4d21-bba0-2aebba68d57a",
    "D. Alexander Gregory",
);

// M13 175 — Garruk's Packleader (reprint)
const GARRUK_S_PACKLEADER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m11::GARRUK_S_PACKLEADER,
    "5eaa6257-614b-4f39-b7fa-ea5f12f94b64",
    "Nils Hamm",
);

// M13 176 — Ground Seal (reprint)
const GROUND_SEAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ody::GROUND_SEAL,
    "6933959a-485f-41a1-a8d9-3bfa416a0faa",
    "Charles Urbach",
);

// M13 177 — Mwonvuli Beast Tracker
pub(in crate::card::sets) static MWONVULI_BEAST_TRACKER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Mwonvuli Beast Tracker",
    "0034d32c-cc82-48d7-a913-d58cc3d3afeb",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human", "Scout"], 2, 1).with_ability(
        abilities::enters_trigger("When this creature enters, search your library for a creature card with deathtouch, hexproof, reach, or trample, reveal it, then shuffle your library and put that card on top of it.", EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Deathtouch),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Hexproof),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Reach),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Trample),
                    ]),
                ]),
                minimum: 1,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Library,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            }),
    ),
);

// M13 178 — Naturalize (reprint)
const NATURALIZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &onslaught::NATURALIZE,
    "e2db6f65-5160-4c10-8a24-f8d4f106adcd",
    "Scott Chou",
);

// M13 179 — Plummet (reprint)
const PLUMMET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::archenemy::PLUMMET,
    "a96d7d96-5a86-45ef-a30b-b11ece22f060",
    "Pete Venters",
);

// M13 180 — Predatory Rampage
pub(in crate::card::sets) static PREDATORY_RAMPAGE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Predatory Rampage",
    "3e054ea5-3657-4198-9715-6acc0e362da3",
    "Wayne England",
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_ability(AbilityDef::spell(
        "Creatures you control get +3/+3 until end of turn. Each creature your opponents \
         control blocks this turn if able.",
        // The two clauses point at opposite boards, which is the whole card: yours
        // get bigger and theirs are forced in front of them.
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBlockEachAttackerIfAble),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// M13 181 — Prey Upon (reprint)
const PREY_UPON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &innistrad::PREY_UPON,
    "e074cc1d-8f94-4155-974e-574c1dd82e1f",
    "Dave Kendall",
);

// M13 182 — Primal Huntbeast
pub(in crate::card::sets) static PRIMAL_HUNTBEAST: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Primal Huntbeast",
    "eb77f6a8-a9d6-4fdd-996e-70877199ebab",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 3, 3)
        .with_abilities(&[abilities::hexproof()]),
);

// M13 183 — Primordial Hydra (reprint)
const PRIMORDIAL_HYDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::PRIMORDIAL_HYDRA,
    "937deb52-8888-4298-9ae5-0361c6fdbba2",
    "Aleksi Briclot",
);

// M13 184 — Quirion Dryad (reprint)
const QUIRION_DRYAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &planeshift::QUIRION_DRYAD,
    "4dba7f54-e49e-4f10-b5c5-46bb20871871",
    "Todd Lockwood",
);

// M13 185 — Rancor (reprint)
const RANCOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &urzas_legacy::RANCOR,
    "b982558f-5b82-4918-9b54-c7ac1e6f8da5",
    "Kev Walker",
);

// M13 186 — Ranger's Path
pub(in crate::card::sets) static RANGERS_PATH: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ranger's Path",
    "26858a53-1054-407a-b2a2-34a7c4ae0f10",
    "Tomasz Jedruszek",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to two Forest cards, put them onto the battlefield tapped, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            minimum: 0,
            maximum: ValueDef::Constant(2),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// M13 187 — Revive (reprint)
const REVIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::mercadian_masques::REVIVE,
    "3a9aae03-f29b-4da6-a0cb-edd67bb111f5",
    "Matthew D. Wilson",
);

// M13 188 — Roaring Primadox
pub(in crate::card::sets) static ROARING_PRIMADOX: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Roaring Primadox",
    "19529b2f-03f0-469d-92d4-e2a2a933d5dc",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 4, 4).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, return a creature you control to its owner's hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            }),
        ),
    ),
);

// M13 189 — Sentinel Spider
pub(in crate::card::sets) static SENTINEL_SPIDER: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Sentinel Spider",
    "5f55ff4b-f0e1-498b-982b-e6ec01d30d95",
    "Vincent Proce",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Spider"], 4, 4)
        .with_abilities(&[abilities::reach(), abilities::vigilance()]),
);

// M13 190 — Serpent's Gift
pub(in crate::card::sets) static SERPENTS_GIFT: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Serpent's Gift",
    "0e27503e-059e-4c44-a817-678e67254111",
    "Steve Argyle",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains deathtouch until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 191 — Silklash Spider (reprint)
const SILKLASH_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::onslaught::SILKLASH_SPIDER,
    "359d1bb9-dbfd-4094-bda0-9a19817ce4bc",
    "Iain McCaig",
);

// M13 192 — Spiked Baloth
pub(in crate::card::sets) static SPIKED_BALOTH: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Spiked Baloth",
    "522777b1-a89f-4969-a962-0137018ec86c",
    "Daarken",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 4, 2)
        .with_abilities(&[abilities::trample()]),
);

// M13 193 — Thragtusk
pub(in crate::card::sets) static THRAGTUSK: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Thragtusk",
    "28667c8b-d02c-4e57-a050-1549207b65d1",
    "Nils Hamm",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 5, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you gain 5 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, create a 3/3 green Beast creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::create_creature_token(&["Beast"], &[ManaColor::Green], 3, 3).with_art(
                CardArt::new("c94010f1-cd4b-4f65-8a0e-2df6eec058ec", "John Donahue"),
            ),
        ),
    ]),
);

// M13 194 — Timberpack Wolf
static OTHER_TIMBERPACK_WOLVES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasName(ObjectRefDef::Source),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static TIMBERPACK_WOLF: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Timberpack Wolf",
    "d16928c9-0470-46ec-b92d-0d6ff9f23ef7",
    "John Avon",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each other creature you control named Timberpack Wolf.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&OTHER_TIMBERPACK_WOLVES),
                    ValueDef::CountMatchingObjects(&OTHER_TIMBERPACK_WOLVES),
                ),
            },
        ),
    ),
);

// M13 195 — Titanic Growth (reprint)
const TITANIC_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::TITANIC_GROWTH,
    "5f1fb9f8-c070-40c9-89cd-c74eb8dbbf1a",
    "Ryan Pancoast",
);

// M13 196 — Vastwood Gorger (reprint)
const VASTWOOD_GORGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::VASTWOOD_GORGER,
    "70fc4a5f-1c59-4139-a506-72baebb1168f",
    "Kieran Yanner",
);

// M13 197 — Yeva, Nature's Herald
pub(in crate::card::sets) static YEVA_NATURE_S_HERALD: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Yeva, Nature's Herald",
    "80acb6dc-a9bd-4f12-9025-623416bdfc32",
    "Eric Deschamps",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elf", "Shaman"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::static_ability(
                "You may cast green creature spells as though they had flash.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                        CastTimingPermissionDef::new(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Color(ManaColor::Green),
                        ])),
                    )),
                },
            ),
        ]),
);

// M13 198 — Yeva's Forcemage
pub(in crate::card::sets) static YEVAS_FORCEMAGE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Yeva's Forcemage",
    "3f9ebf02-56b3-492e-88fb-2e95f13f5764",
    "Eric Deschamps",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature gets +2/+2 until end of turn.",
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

// M13 199 — Nicol Bolas, Planeswalker (reprint)
const NICOL_BOLAS_PLANESWALKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::conflux::NICOL_BOLAS_PLANESWALKER,
    "0e3b1fea-5c2c-4848-8109-548f56b99d49",
    "D. Alexander Gregory",
);

// M13 200 — Akroma's Memorial (reprint)
const AKROMAS_MEMORIAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::future_sight::AKROMAS_MEMORIAL,
    "d00d63c3-85a5-4c2d-bdba-6213527b5e9a",
    "Dan Murayama Scott",
);

// M13 201 — Chronomaton
pub(in crate::card::sets) static CHRONOMATON: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Chronomaton",
    "aac35e28-dd0e-4dc8-b8e6-4a1e33706214",
    "Vincent Proce",
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Golem"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}, {T}: Put a +1/+1 counter on this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 202 — Clock of Omens (reprint)
const CLOCK_OF_OMENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_5dn::CLOCK_OF_OMENS,
    "5b087992-9c30-4434-acb3-a12ee6f207b3",
    "Ryan Yee",
);

// M13 203 — Door to Nothingness (reprint)
const DOOR_TO_NOTHINGNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::fifth_dawn::DOOR_TO_NOTHINGNESS,
    "57877b1c-e91d-4941-81bd-008dff1272ed",
    "Svetlin Velinov",
);

// M13 204 — Elixir of Immortality (reprint)
const ELIXIR_OF_IMMORTALITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::ELIXIR_OF_IMMORTALITY,
    "813d6a95-719d-474d-942a-b4c5156af7ba",
    "Zoltan Boros & Gabor Szikszai",
);

// M13 205 — Gem of Becoming
pub(in crate::card::sets) static GEM_OF_BECOMING: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Gem of Becoming",
    "0e07bc36-2207-48f4-a151-4ccb0c6d851d",
    "Jack Wang",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated(
        "{3}, {T}, Sacrifice this artifact: Search your library for an Island card, a Swamp card, and a Mountain card. Reveal those cards, put them into your hand, then shuffle.",
        &[
            AbilityCostDef::Mana(mana_cost!("{3}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::Sequence(&[
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: false,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: false,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ]),
    )),
);

// M13 206 — Gilded Lotus (reprint)
const GILDED_LOTUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::GILDED_LOTUS,
    "33704052-aeb1-4798-a64d-778e1879eeb9",
    "Martina Pilcerova",
);

// M13 207 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JAYEMDAE_TOME,
    "0802b908-d4e1-4f58-a085-55782fc08d51",
    "Donato Giancola",
);

// M13 208 — Kitesail (reprint)
const KITESAIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::worldwake::KITESAIL,
    "2f95cf4c-1845-4260-8571-91c03d582da3",
    "Cyril Van Der Haegen",
);

// M13 209 — Phyrexian Hulk (reprint)
const PHYREXIAN_HULK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::tempest::PHYREXIAN_HULK,
    "a761426e-2138-438e-8f3b-024486165260",
    "Steven Belledin",
);

// M13 210 — Primal Clay (reprint)
const PRIMAL_CLAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::PRIMAL_CLAY,
    "774cece8-39ac-48fe-bfbe-494ec76d80ee",
    "Lucas Graciano",
);

// M13 211 — Ring of Evos Isle
pub(in crate::card::sets) static RING_OF_EVOS_ISLE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ring of Evos Isle",
    "a7c740a8-1bbc-4ec8-a72c-01aee9e48f3d",
    "Erica Yang",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::activated(
                "{2}: Equipped creature gains hexproof until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's blue.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Blue),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 212 — Ring of Kalonia
pub(in crate::card::sets) static RING_OF_KALONIA: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ring of Kalonia",
    "2082e04f-f972-424e-a724-7a5975215538",
    "Erica Yang",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's green.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Green),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 213 — Ring of Thune
pub(in crate::card::sets) static RING_OF_THUNE: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ring of Thune",
    "1ee2e94f-5b06-4df0-ba87-4499b1ee4dba",
    "Erica Yang",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's white.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::White),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 214 — Ring of Valkas
pub(in crate::card::sets) static RING_OF_VALKAS: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ring of Valkas",
    "546e9fc1-03ff-4ae5-9488-51bf2e627486",
    "Erica Yang",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's red.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Red),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 215 — Ring of Xathrid
pub(in crate::card::sets) static RING_OF_XATHRID: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Ring of Xathrid",
    "47e2aa59-63dc-4e28-8cdc-2ca868ff8f59",
    "Erica Yang",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::activated(
                "{2}: Regenerate equipped creature.",
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's black.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Black),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 216 — Sands of Delirium
pub(in crate::card::sets) static SANDS_OF_DELIRIUM: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Sands of Delirium",
    "78c9d3bf-c858-42f4-bb61-3292f9a7141b",
    "Charles Urbach",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{X}, {T}: Target player mills X cards.",
        &[
            AbilityCostDef::Mana(mana_cost!("{X}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )),
);

// M13 217 — Staff of Nin
pub(in crate::card::sets) static STAFF_OF_NIN: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Staff of Nin",
    "69b7381a-ec4a-4f1b-b81c-bdf9f9d64f31",
    "Dan Murayama Scott",
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, draw a card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: This artifact deals 1 damage to any target.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M13 218 — Stuffy Doll (reprint)
const STUFFY_DOLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tsp::STUFFY_DOLL,
    "23038e62-9c7b-4e2a-8661-035966b6ed4a",
    "David Rapoza",
);

// M13 219 — Tormod's Crypt (reprint)
const TORMODS_CRYPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &the_dark::TORMODS_CRYPT,
    "efdfb60b-948b-40fb-b18e-08f0300624b3",
    "Lars Grant-West",
);

// M13 220 — Trading Post
pub(in crate::card::sets) static TRADING_POST: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Trading Post",
    "20604b28-d096-40f8-a30c-3bc89e708676",
    "Adam Paquette",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}, Discard a card: You gain 4 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
        AbilityDef::activated(
            "{1}, {T}, Pay 1 life: Create a 0/1 white Goat creature token.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::PayLife(1),
            ],
            EffectDef::create_creature_token(&["Goat"], &[ManaColor::White], 0, 1),
        ),
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice a creature: Return target artifact card from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice an artifact: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M13 221 — Cathedral of War
pub(in crate::card::sets) static CATHEDRAL_OF_WAR: CardRecord = CardRecord::new(
    CardSet::Magic2013,
    "Cathedral of War",
    "dd222c07-0b28-41cb-9237-ad7991ab078f",
    "Kekai Kotaki",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::exalted(),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]),
);

// M13 222 — Dragonskull Summit (reprint)
const DRAGONSKULL_SUMMIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DRAGONSKULL_SUMMIT,
    "5e49c561-570c-43dd-a369-48bc7ad7edac",
    "Jon Foster",
);

// M13 223 — Drowned Catacomb (reprint)
const DROWNED_CATACOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DROWNED_CATACOMB,
    "8b41b86b-58e1-4601-b8ed-0ad31f03a78d",
    "Dave Kendall",
);

// M13 224 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::EVOLVING_WILDS,
    "d9d107e1-8293-4486-9b68-4897b8b7043c",
    "Steven Belledin",
);

// M13 225 — Glacial Fortress (reprint)
const GLACIAL_FORTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::GLACIAL_FORTRESS,
    "bc9d29ee-1a21-4c3e-99c1-f815d40e8f19",
    "Franz Vohwinkel",
);

// M13 226 — Hellion Crucible
pub(in crate::card::sets) static HELLION_CRUCIBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2013,
    "Hellion Crucible",
    "ad8274ef-a46a-4f5f-8ad1-6ce828f24210",
    "Trevor Claxton",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated(
            "{1}{R}, {T}: Put a pressure counter on this land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("pressure"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{1}{R}, {T}, Remove two pressure counters from this land and sacrifice it: Create a 4/4 red Hellion creature token with haste.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("pressure"),
                    amount: 2,
                },
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::create_creature_token(&["Hellion"], &[ManaColor::Red], 4, 4)
                .with_abilities(&[abilities::haste()]),
        ),
    ]),
);

// M13 227 — Reliquary Tower (reprint)
const RELIQUARY_TOWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::conflux::RELIQUARY_TOWER,
    "f92583e4-9749-4c11-9d32-fb81260c5b63",
    "Jesper Ejsing",
);

// M13 228 — Rootbound Crag (reprint)
const ROOTBOUND_CRAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ROOTBOUND_CRAG,
    "76364643-bfcb-4c50-9224-bf9e35648ddf",
    "Matt Stewart",
);

// M13 229 — Sunpetal Grove (reprint)
const SUNPETAL_GROVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SUNPETAL_GROVE,
    "15663129-9deb-4c34-84a0-f94cf1a723f0",
    "Jason Chan",
);

// M13 230 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLAINS,
    "080a001b-7815-469b-bd0c-c92453d80e9a",
    "John Avon",
);

// M13 231 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    1,
    "b4f8fa19-a872-4542-bf24-8bba9f0a64a1",
    "Noah Bradley",
);

// M13 232 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    2,
    "19e094ea-3dee-47a6-997e-842113774973",
    "Nils Hamm",
);

// M13 233 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    3,
    "c80e0478-8c53-4406-acc0-b7662c9c382d",
    "Charles Urbach",
);

// M13 234 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ISLAND,
    "cd86b167-3fc8-4e5a-9e21-b4ce5a7a05cd",
    "Rob Alexander",
);

// M13 235 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    1,
    "dad64d3a-1868-4404-8692-d3c54071140d",
    "Noah Bradley",
);

// M13 236 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    2,
    "92e3d45d-8c6e-430a-82d3-86f66286735d",
    "Cliff Childs",
);

// M13 237 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    3,
    "956fd08b-d403-4565-84b3-e3ca0132ea89",
    "Peter Mohrbacher",
);

// M13 238 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWAMP,
    "83b16e93-ee53-4c1b-9dea-6db7977e9c2f",
    "Mike Bierek",
);

// M13 239 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    1,
    "c7827d75-eb0e-4ebf-876e-7a2f9e373fb3",
    "Mike Bierek",
);

// M13 240 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    2,
    "9364178a-1963-4665-b8c1-a5096ece07e2",
    "Cliff Childs",
);

// M13 241 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    3,
    "7b4ddbe5-1be4-47ee-b07a-74c8bec4d752",
    "Jung Park",
);

// M13 242 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOUNTAIN,
    "d67cf0c1-5658-4f88-9fbd-63c1dbf77ee0",
    "Cliff Childs",
);

// M13 243 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    1,
    "c9a3144f-9b4f-4d1a-b384-6228c99b38b7",
    "Nils Hamm",
);

// M13 244 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    2,
    "17990059-8368-45f8-8325-c82fc181450a",
    "Karl Kopinski",
);

// M13 245 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    3,
    "3cefc72a-83ba-42e4-a036-a9f45363c8cf",
    "Robh Ruppel",
);

// M13 246 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOREST,
    "51a55233-2e1a-4515-8fd1-354605c0c36b",
    "Volkan Baǵa",
);

// M13 247 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    1,
    "2fdf7380-56cb-4d34-ad05-43029341a57a",
    "Steven Belledin",
);

// M13 248 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    2,
    "65aa4dcf-6e3a-4381-b5f6-5056d741ff67",
    "Noah Bradley",
);

// M13 249 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    3,
    "aa35174e-c0c8-4643-bc32-9a7b7c7e7d00",
    "Jim Nelson",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AJANI_CALLER_OF_THE_PRIDE,
    &AJANIS_SUNSTRIKER,
    &ATTENDED_KNIGHT,
    &BATTLEFLIGHT_EAGLE,
    &CAPTAINS_CALL,
    &CRUSADER_OF_ODRIC,
    &FAITH_S_REWARD,
    &GRIFFIN_PROTECTOR,
    &GUARDIAN_LIONS,
    &HEALER_OF_THE_PRIDE,
    &KNIGHT_OF_GLORY,
    &ODRIC_MASTER_TACTICIAN,
    &PRIZED_ELEPHANT,
    &RHOX_FAITHMENDER,
    &SHOW_OF_VALOR,
    &SUBLIME_ARCHANGEL,
    &TOUCH_OF_THE_ETERNAL,
    &WAR_FALCON,
    &WARCLAMP_MASTIFF,
    &ARCHAEOMANCER,
    &ARCTIC_AVEN,
    &AUGUR_OF_BOLAS,
    &COURTLY_PROVOCATEUR,
    &DOWNPOUR,
    &ENCRUST,
    &FAERIE_INVADERS,
    &HYDROSURGE,
    &JACE_S_PHANTASM,
    &MASTER_OF_THE_PEARL_TRIDENT,
    &MIND_SCULPT,
    &OMNISCIENCE,
    &SPELLTWINE,
    &SWITCHEROO,
    &TALRAND_SKY_SUMMONER,
    &TALRANDS_INVOCATION,
    &TRICKS_OF_THE_TRADE,
    &VOID_STALKER,
    &WATERCOURSER,
    &BLOOD_RECKONING,
    &BLOODHUNTER_BAT,
    &COWER_IN_FEAR,
    &CRIPPLING_BLIGHT,
    &DIABOLIC_REVELATION,
    &DISCIPLE_OF_BOLAS,
    &DUSKMANTLE_PROWLER,
    &DUTY_BOUND_DEAD,
    &HARBOR_BANDIT,
    &KNIGHT_OF_INFAMY,
    &LILIANA_OF_THE_DARK_REALMS,
    &LILIANAS_SHADE,
    &MARK_OF_THE_VAMPIRE,
    &MURDER,
    &NEFAROX_OVERLORD_OF_GRIXIS,
    &PUBLIC_EXECUTION,
    &SERVANT_OF_NEFAROX,
    &VEILBORN_GHOUL,
    &VILE_REBIRTH,
    &XATHRID_GORGON,
    &CHANDRAS_FURY,
    &CLEAVER_RIOT,
    &CRATERIZE,
    &CRIMSON_MUCKWADER,
    &DRAGON_HATCHLING,
    &FIREWING_PHOENIX,
    &FLAMES_OF_THE_FIREBRAND,
    &GOBLIN_BATTLE_JESTER,
    &KRENKO_MOB_BOSS,
    &KRENKOS_COMMAND,
    &MAGMAQUAKE,
    &MINDCLAW_SHAMAN,
    &RECKLESS_BRUTE,
    &RUMMAGING_GOBLIN,
    &SEARING_SPEAR,
    &SLUMBERING_DRAGON,
    &SMELT,
    &THUNDERMAW_HELLKITE,
    &WILD_GUESS,
    &WORLDFIRE,
    &BOND_BEETLE,
    &BOUNDLESS_REALMS,
    &ELDERSCALE_WURM,
    &FLINTHOOF_BOAR,
    &FUNGAL_SPROUTING,
    &MWONVULI_BEAST_TRACKER,
    &PREDATORY_RAMPAGE,
    &PRIMAL_HUNTBEAST,
    &RANGERS_PATH,
    &ROARING_PRIMADOX,
    &SENTINEL_SPIDER,
    &SERPENTS_GIFT,
    &SPIKED_BALOTH,
    &THRAGTUSK,
    &TIMBERPACK_WOLF,
    &YEVA_NATURE_S_HERALD,
    &YEVAS_FORCEMAGE,
    &CHRONOMATON,
    &GEM_OF_BECOMING,
    &RING_OF_EVOS_ISLE,
    &RING_OF_KALONIA,
    &RING_OF_THUNE,
    &RING_OF_VALKAS,
    &RING_OF_XATHRID,
    &SANDS_OF_DELIRIUM,
    &STAFF_OF_NIN,
    &TRADING_POST,
    &CATHEDRAL_OF_WAR,
    &HELLION_CRUCIBLE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANGELS_MERCY_REPRINT,
    ANGELIC_BENEDICTION_REPRINT,
    AVEN_SQUIRE_REPRINT,
    CAPTAIN_OF_THE_WATCH_REPRINT,
    DIVINE_FAVOR_REPRINT,
    DIVINE_VERDICT_REPRINT,
    ERASE_REPRINT,
    GLORIOUS_CHARGE_REPRINT,
    GUARDIANS_OF_AKRASA_REPRINT,
    INTREPID_HERO_REPRINT,
    OBLIVION_RING_REPRINT,
    PACIFISM_REPRINT,
    PILLARFIELD_OX_REPRINT,
    PLANAR_CLEANSING_REPRINT,
    RAIN_OF_BLADES_REPRINT,
    SAFE_PASSAGE_REPRINT,
    SERRA_ANGEL_REPRINT,
    SERRA_AVATAR_REPRINT,
    SERRA_AVENGER_REPRINT,
    SILVERCOAT_LION_REPRINT,
    WAR_PRIEST_OF_THUNE_REPRINT,
    BATTLE_OF_WITS_REPRINT,
    CLONE_REPRINT,
    DIVINATION_REPRINT,
    ESSENCE_SCATTER_REPRINT,
    FOG_BANK_REPRINT,
    HARBOR_SERPENT_REPRINT,
    INDEX_REPRINT,
    JACE_MEMORY_ADEPT_REPRINT,
    KRAKEN_HATCHLING_REPRINT,
    MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT,
    NEGATE_REPRINT,
    REDIRECT_REPRINT,
    REWIND_REPRINT,
    SCROLL_THIEF_REPRINT,
    SLEEP_REPRINT,
    SPHINX_OF_UTHUUN_REPRINT,
    STORMTIDE_LEVIATHAN_REPRINT,
    UNSUMMON_REPRINT,
    VEDALKEN_ENTRANCER_REPRINT,
    WELKIN_TERN_REPRINT,
    WIND_DRAKE_REPRINT,
    BLOODTHRONE_VAMPIRE_REPRINT,
    DARK_FAVOR_REPRINT,
    DISENTOMB_REPRINT,
    DURESS_REPRINT,
    ESSENCE_DRAIN_REPRINT,
    GIANT_SCORPION_REPRINT,
    MIND_ROT_REPRINT,
    MUTILATE_REPRINT,
    PHYLACTERY_LICH_REPRINT,
    RAVENOUS_RATS_REPRINT,
    RISE_FROM_THE_GRAVE_REPRINT,
    SHIMIAN_SPECTER_REPRINT,
    SIGN_IN_BLOOD_REPRINT,
    TORMENTED_SOUL_REPRINT,
    VAMPIRE_NIGHTHAWK_REPRINT,
    VAMPIRE_NOCTURNUS_REPRINT,
    WALKING_CORPSE_REPRINT,
    WITS_END_REPRINT,
    ZOMBIE_GOLIATH_REPRINT,
    ARMS_DEALER_REPRINT,
    BLADETUSK_BOAR_REPRINT,
    CANYON_MINOTAUR_REPRINT,
    CHANDRA_THE_FIREBRAND_REPRINT,
    FERVOR_REPRINT,
    FIRE_ELEMENTAL_REPRINT,
    FURNACE_WHELP_REPRINT,
    GOBLIN_ARSONIST_REPRINT,
    HAMLETBACK_GOLIATH_REPRINT,
    KINDLED_FURY_REPRINT,
    MARK_OF_MUTINY_REPRINT,
    MOGG_FLUNKIES_REPRINT,
    REVERBERATE_REPRINT,
    TORCH_FIEND_REPRINT,
    TRUMPET_BLAST_REPRINT,
    TURN_TO_SLAG_REPRINT,
    VOLCANIC_GEYSER_REPRINT,
    VOLCANIC_STRENGTH_REPRINT,
    WALL_OF_FIRE_REPRINT,
    ACIDIC_SLIME_REPRINT,
    ARBOR_ELF_REPRINT,
    BOUNTIFUL_HARVEST_REPRINT,
    CENTAUR_COURSER_REPRINT,
    DEADLY_RECLUSE_REPRINT,
    DUSKDALE_WURM_REPRINT,
    ELVISH_ARCHDRUID_REPRINT,
    ELVISH_VISIONARY_REPRINT,
    FARSEEK_REPRINT,
    FOG_REPRINT,
    GARRUK_PRIMAL_HUNTER_REPRINT,
    GARRUK_S_PACKLEADER_REPRINT,
    GROUND_SEAL_REPRINT,
    NATURALIZE_REPRINT,
    PLUMMET_REPRINT,
    PREY_UPON_REPRINT,
    PRIMORDIAL_HYDRA_REPRINT,
    QUIRION_DRYAD_REPRINT,
    RANCOR_REPRINT,
    REVIVE_REPRINT,
    SILKLASH_SPIDER_REPRINT,
    TITANIC_GROWTH_REPRINT,
    VASTWOOD_GORGER_REPRINT,
    NICOL_BOLAS_PLANESWALKER_REPRINT,
    AKROMAS_MEMORIAL_REPRINT,
    CLOCK_OF_OMENS_REPRINT,
    DOOR_TO_NOTHINGNESS_REPRINT,
    ELIXIR_OF_IMMORTALITY_REPRINT,
    GILDED_LOTUS_REPRINT,
    JAYEMDAE_TOME_REPRINT,
    KITESAIL_REPRINT,
    PHYREXIAN_HULK_REPRINT,
    PRIMAL_CLAY_REPRINT,
    STUFFY_DOLL_REPRINT,
    TORMODS_CRYPT_REPRINT,
    DRAGONSKULL_SUMMIT_REPRINT,
    DROWNED_CATACOMB_REPRINT,
    EVOLVING_WILDS_REPRINT,
    GLACIAL_FORTRESS_REPRINT,
    RELIQUARY_TOWER_REPRINT,
    ROOTBOUND_CRAG_REPRINT,
    SUNPETAL_GROVE_REPRINT,
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
