//! Tempest cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::sets::y2022::commander_legends_baldurs_gate as catalog_clb;
use crate::card::{
    AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef,
    BlockRestrictionDef, CardArt, CardNameDef, CardNameSetDef, CardRules, CardSet, CardSupertype,
    CardType, ChoiceVisibilityDef, ChooseDef, CostDef, CostModificationDef, CounterKind,
    DamageEventMatcherDef, DamagePreventionDef, DiscardSelectionDef, DividedTotal,
    DrawEventMatcherDef, EffectChoiceDef, EffectDef, EffectRecipientDef, KeywordAbility, ManaColor,
    ManaTypeSetDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetCountConditionDef, ObjectSetDef, ObjectSetPredicateDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ReplacementChoiceDef, ReplacementEffectDef, ReplacementEventDef,
    ResolvedEffectDurationDef, SacrificedAmountDef, ScaledValueDef, TargetChooserDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

/// Every creature but the one asking, negated: what "gets -1/-1 for each
/// other creature on the battlefield" subtracts. Scaled by -1 rather than
/// negated, because the static power-and-toughness layer reads a scale and
/// not a negation.
static OTHER_CREATURES: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    )),
    -1,
));
// TMP 1 — Advance Scout
pub(in crate::card::sets) static ADVANCE_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81ce7e1e-ffe5-4ced-8967-9a6917245240"),
    "Advance Scout",
    CardArt::new("81ce7e1e-ffe5-4ced-8967-9a6917245240", "Heather Hudson"),
    CardSet::Tempest,
    // First strike it can hand to something bigger, which is where the
    // keyword is actually worth mana.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier", "Scout"], 1, 1)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::activated_with_targets(
                "{W}: Target creature gains first strike until end of turn.",
                &[CostDef::Mana(mana_cost!("{W}"))],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&const { abilities::first_strike() }),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// TMP 2 — Angelic Protector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_PROTECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44faefbe-d5e7-48f3-ba88-833da0b19707"),
    "Angelic Protector",
    crate::card::CardArt::new("44faefbe-d5e7-48f3-ba88-833da0b19707", "DiTerlizzi"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 3 — Anoint
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANOINT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca65ee0f-fdd7-4a5e-a4a3-5dd9c62096ab"),
    "Anoint",
    crate::card::CardArt::new(
        "ca65ee0f-fdd7-4a5e-a4a3-5dd9c62096ab",
        "Eric David Anderson",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 4 — Armor Sliver
pub(in crate::card::sets) static ARMOR_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c275aba7-cac6-48e8-b12c-6bd77a5c38fe"),
    "Armor Sliver",
    CardArt::new("c275aba7-cac6-48e8-b12c-6bd77a5c38fe", "Scott Kirschner"),
    CardSet::Tempest,
    // A mana sink every Sliver shares, which turns a board of 1/1s into a
    // board nothing profitably blocks.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have \"{2}: This creature gets +0/+1 until end of turn.\"",
            EffectDef::StaticApply {
                // "All Sliver creatures", so the opponent's get it too.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                // "This creature" inside the granted ability is
                // whichever Sliver has it, which is that ability's own
                // source rather than this one.
                effect: AppliedEffectDef::add_ability(
                    &const {
                        AbilityDef::activated(
                            "{2}: This creature gets +0/+1 until end of turn.",
                            &[CostDef::Mana(mana_cost!("{2}"))],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(0),
                                    ValueDef::Constant(1),
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        )
                    },
                ),
            },
        ),
    ),
);

// TMP 5 — Armored Pegasus
pub(in crate::card::sets) static ARMORED_PEGASUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f021a79-a182-4914-9ff4-d6fcba7c1d22"),
    "Armored Pegasus",
    CardArt::new("012049f8-0936-49ed-948d-0d34af28550f", "Una Fricker"),
    CardSet::Tempest,
    // Two mana for a flier that survives the burn spell aimed at it, which
    // is the whole difference between this and a 1/1.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Pegasus"], 1, 2)
        .with_abilities(&[abilities::flying()]),
);

// TMP 6 — Auratog
pub(in crate::card::sets) static AURATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86dca066-d5e3-442a-95a0-e695c1d5850c"),
    "Auratog",
    CardArt::new("86dca066-d5e3-442a-95a0-e695c1d5850c", "Jeff Miracola"),
    CardSet::Tempest,
    // Enchantments for size, which only reads well in the deck that was
    // stacking Auras on it anyway.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Atog"], 1, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice an enchantment: This creature gets +2/+2 until end of turn.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Enchantment),
                controller: PlayerRelation::You,
            }],
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

// TMP 7 — Avenging Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGING_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28333138-60bc-459b-a0cd-1b7fd19c89cd"),
    "Avenging Angel",
    crate::card::CardArt::new("28333138-60bc-459b-a0cd-1b7fd19c89cd", "Matthew D. Wilson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 8 — Circle of Protection: Black (reprint)

// TMP 9 — Circle of Protection: Blue (reprint)

// TMP 10 — Circle of Protection: Green (reprint)

// TMP 11 — Circle of Protection: Red (reprint)

// TMP 12 — Circle of Protection: Shadow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_SHADOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49f29a3b-7136-496c-bc29-8808bfff0f82"),
    "Circle of Protection: Shadow",
    crate::card::CardArt::new("49f29a3b-7136-496c-bc29-8808bfff0f82", "Harold McNeill"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 13 — Circle of Protection: White (reprint)

// TMP 14 — Clergy en-Vec
pub(in crate::card::sets) static CLERGY_EN_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fcb0e068-16d0-4e1c-acad-0a6d34148c5a"),
    "Clergy en-Vec",
    CardArt::new("fcb0e068-16d0-4e1c-acad-0a6d34148c5a", "Heather Hudson"),
    CardSet::Tempest,
    // The same Samite body Mirage printed, reprinted for a block where the
    // one point still swung combat.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 15 — Cloudchaser Eagle
pub(in crate::card::sets) static CLOUDCHASER_EAGLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a70a6da-dea3-49c0-8c49-6a2229c3ac91"),
    "Cloudchaser Eagle",
    CardArt::new("3a70a6da-dea3-49c0-8c49-6a2229c3ac91", "Una Fricker"),
    CardSet::Tempest,
    // A body and an answer in one card, which is why white commons like
    // this never rotate out of a limited deck.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target enchantment.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// TMP 16 — Disenchant (reprint)

// TMP 17 — Elite Javelineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELITE_JAVELINEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea1c730f-76da-4eae-b3fc-b428b860ea93"),
    "Elite Javelineer",
    crate::card::CardArt::new("ea1c730f-76da-4eae-b3fc-b428b860ea93", "Mark Poole"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 18 — Field of Souls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIELD_OF_SOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9816a3ef-e2a8-4d97-afbf-d190a62265bf"),
    "Field of Souls",
    crate::card::CardArt::new(
        "9816a3ef-e2a8-4d97-afbf-d190a62265bf",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 19 — Flickering Ward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLICKERING_WARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4d2b011-bb0d-463c-bf2a-04b6650771a3"),
    "Flickering Ward",
    crate::card::CardArt::new("d4d2b011-bb0d-463c-bf2a-04b6650771a3", "Greg Simanson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 20 — Gallantry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALLANTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ccdca3b-7d53-4d19-bd15-9a1b148c4aaf"),
    "Gallantry",
    crate::card::CardArt::new("1ccdca3b-7d53-4d19-bd15-9a1b148c4aaf", "Douglas Shuler"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 21 — Gerrard's Battle Cry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_S_BATTLE_CRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("504950d5-2df7-4518-b987-fe3a57ad1c58"),
    "Gerrard's Battle Cry",
    crate::card::CardArt::new("504950d5-2df7-4518-b987-fe3a57ad1c58", "Val Mayerik"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 22 — Hanna's Custody
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HANNA_S_CUSTODY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ea44536-ef4e-4dcf-9c1a-c1122dd00cbb"),
    "Hanna's Custody",
    crate::card::CardArt::new("7ea44536-ef4e-4dcf-9c1a-c1122dd00cbb", "DiTerlizzi"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 23 — Hero's Resolve
pub(in crate::card::sets) static HERO_S_RESOLVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4cdcc7c-0d01-4aa2-8934-079dfc00eef2"),
    "Hero's Resolve",
    CardArt::new("b4cdcc7c-0d01-4aa2-8934-079dfc00eef2", "Pete Venters"),
    CardSet::Tempest,
    // Five toughness for two mana, which wins a block and does nothing
    // else -- and the card is gone either way.
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+5.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(5),
                    ),
                },
            ),
        ]),
);

// TMP 24 — Humility
pub(in crate::card::sets) static HUMILITY: CardRecord = CardRecord::new_with_legacy_id(
    2055,
    "Humility",
    CardArt::new("a2fb7128-806b-4148-80fe-eb967f248021", "Phil Foglio"),
    CardSet::Tempest,
    // Symmetric and total: the control deck playing it has no creatures to
    // lose, which is the whole argument for the card.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "All creatures lose all abilities and have base power and toughness 1/1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            // Everything at once, in one static effect: the abilities go in layer 6 and
            // the stats are set in layer 7b, and a creature that arrives later is caught
            // by the same continuous effect rather than needing its own.
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            ]),
        },
    )),
);

// TMP 25 — Invulnerability
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVULNERABILITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d66d1f00-e857-4bc3-a36d-a33669d281e9"),
    "Invulnerability",
    crate::card::CardArt::new("d66d1f00-e857-4bc3-a36d-a33669d281e9", "Brian Snõddy"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 26 — Knight of Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_DAWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf0e5034-a134-4eb6-af8e-b2419b92b3a6"),
    "Knight of Dawn",
    crate::card::CardArt::new("bf0e5034-a134-4eb6-af8e-b2419b92b3a6", "Ron Spencer"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 27 — Light of Day
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHT_OF_DAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70fa9ebe-bdf5-4359-aa3e-6cfa1a1d96cf"),
    "Light of Day",
    crate::card::CardArt::new("70fa9ebe-bdf5-4359-aa3e-6cfa1a1d96cf", "Drew Tucker"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 28 — Marble Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARBLE_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ca62c97-0bbd-4f74-afd5-99b48c063aa0"),
    "Marble Titan",
    crate::card::CardArt::new("9ca62c97-0bbd-4f74-afd5-99b48c063aa0", "Brom"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 29 — Master Decoy
pub(in crate::card::sets) static MASTER_DECOY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3e11097-1ace-4ae8-a9e8-d00b9f709e54"),
    "Master Decoy",
    CardArt::new("f3e11097-1ace-4ae8-a9e8-d00b9f709e54", "Phil Foglio"),
    CardSet::Tempest,
    // Two mana to turn off their best creature every turn, which is more
    // than the 1/2 body suggests.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{W}, {T}: Tap target creature.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// TMP 30 — Mounted Archers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUNTED_ARCHERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f3abcf2-fe52-4096-8fdb-6917d75a04e3"),
    "Mounted Archers",
    crate::card::CardArt::new("4f3abcf2-fe52-4096-8fdb-6917d75a04e3", "Kev Walker"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 31 — Oracle en-Vec
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORACLE_EN_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc538730-c46c-4e5f-bc1f-0efb7765086d"),
    "Oracle en-Vec",
    crate::card::CardArt::new("cc538730-c46c-4e5f-bc1f-0efb7765086d", "Dan Frazier"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 32 — Orim's Prayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_PRAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2dc45565-4b56-49ba-b115-be8e0de7d937"),
    "Orim's Prayer",
    crate::card::CardArt::new("2dc45565-4b56-49ba-b115-be8e0de7d937", "Donato Giancola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 33 — Orim, Samite Healer
pub(in crate::card::sets) static ORIM_SAMITE_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7086d077-f083-4870-8b0b-2d34aca49df1"),
    "Orim, Samite Healer",
    CardArt::new("7086d077-f083-4870-8b0b-2d34aca49df1", "Kaja Foglio"),
    CardSet::Tempest,
    // Three points a turn from a 1/3 body: it blanks a burn spell or wins
    // any fight its own creatures pick.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Cleric"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Prevent the next 3 damage that would be dealt to any target this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// TMP 34 — Pacifism (reprint)

// TMP 35 — Pegasus Refuge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEGASUS_REFUGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2bce334-0ae6-4a7d-85db-99ee205ce546"),
    "Pegasus Refuge",
    crate::card::CardArt::new("a2bce334-0ae6-4a7d-85db-99ee205ce546", "Kev Walker"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 36 — Quickening Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKENING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6e91f3d-5a23-4df1-a879-d18a3af92a28"),
    "Quickening Licid",
    crate::card::CardArt::new("e6e91f3d-5a23-4df1-a879-d18a3af92a28", "Andrew Robinson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 37 — Repentance
pub(in crate::card::sets) static REPENTANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e28ac76-c671-4be1-bcfc-17f2d7bbe08f"),
    "Repentance",
    CardArt::new("3e28ac76-c671-4be1-bcfc-17f2d7bbe08f", "Ron Spencer"),
    CardSet::Tempest,
    // The bigger the creature, the more surely it kills itself, which is
    // white's answer to something it cannot block.
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature deals damage to itself equal to its power.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The creature is the source as well as the recipient, so lifelink or
        // a damage trigger on it sees its own damage.
        EffectDef::DealDamageFrom {
            source: ObjectRefDef::Target(TargetIndex::PRIMARY),
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
        },
    )),
);

// TMP 38 — Sacred Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_GUIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f10c37d-d25a-47d6-83b0-dbe0a9cfc938"),
    "Sacred Guide",
    crate::card::CardArt::new("7f10c37d-d25a-47d6-83b0-dbe0a9cfc938", "Zina Saunders"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 39 — Safeguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAFEGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c8e174c-7abb-4a93-aa1d-8c2a2e815ba6"),
    "Safeguard",
    crate::card::CardArt::new("2c8e174c-7abb-4a93-aa1d-8c2a2e815ba6", "Thomas M. Baxa"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 40 — Serene Offering
pub(in crate::card::sets) static SERENE_OFFERING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c0b3795-7f30-4c61-b5d8-f238055d6be1"),
    "Serene Offering",
    CardArt::new("6c0b3795-7f30-4c61-b5d8-f238055d6be1", "Paolo Parente"),
    CardSet::Tempest,
    // Two mana to answer an enchantment, and the life is a rebate scaled to
    // how expensive the thing you answered was.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target enchantment. You gain life equal to its mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // Read after the destruction, so the value comes from
            // last-known information about a permanent already gone.
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// TMP 41 — Soltari Crusader
pub(in crate::card::sets) static SOLTARI_CRUSADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6cd07471-b216-465c-9946-1eac689db32e"),
    "Soltari Crusader",
    CardArt::new("6cd07471-b216-465c-9946-1eac689db32e", "Randy Gallegos"),
    CardSet::Tempest,
    // Shadow means the pump is never wasted: nothing blocks it, so every
    // point is damage.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Soltari", "Knight"], 2, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::activated(
            "{1}{W}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
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

// TMP 42 — Soltari Emissary
pub(in crate::card::sets) static SOLTARI_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a18751d3-052b-4ae5-ba07-16f00a1af40e"),
    "Soltari Emissary",
    CardArt::new("a18751d3-052b-4ae5-ba07-16f00a1af40e", "Adam Rex"),
    CardSet::Tempest,
    // One mana turns a 2/1 into an unblockable one, so the body is really a
    // two-mana Shade with no ceiling on damage.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Soltari", "Soldier"], 2, 1).with_ability(
        AbilityDef::activated(
            "{W}: This creature gains shadow until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::shadow() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 43 — Soltari Foot Soldier
pub(in crate::card::sets) static SOLTARI_FOOT_SOLDIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdf295dc-72df-4097-b767-d89ab807bf2e"),
    "Soltari Foot Soldier",
    CardArt::new("bdf295dc-72df-4097-b767-d89ab807bf2e", "Janet Aulisio"),
    CardSet::Tempest,
    // One mana for one unblockable damage a turn, which is what shadow does
    // in a format where nobody else has it.
    CardRules::new_creature(mana_cost!("{W}"), &["Soltari", "Soldier"], 1, 1)
        .with_ability(abilities::shadow()),
);

// TMP 44 — Soltari Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLTARI_LANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab4b6c91-dd07-4d39-bd36-6fbf28e7698e"),
    "Soltari Lancer",
    crate::card::CardArt::new("ab4b6c91-dd07-4d39-bd36-6fbf28e7698e", "Matthew D. Wilson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 45 — Soltari Monk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLTARI_MONK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54e0d969-3e4d-4ff9-8bda-3a6ac8df01b2"),
    "Soltari Monk",
    crate::card::CardArt::new("54e0d969-3e4d-4ff9-8bda-3a6ac8df01b2", "Janet Aulisio"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 46 — Soltari Priest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLTARI_PRIEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35a71390-3fa8-43eb-ad86-67de2a7aeab8"),
    "Soltari Priest",
    crate::card::CardArt::new("35a71390-3fa8-43eb-ad86-67de2a7aeab8", "Janet Aulisio"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 47 — Soltari Trooper
pub(in crate::card::sets) static SOLTARI_TROOPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32f74aa3-4003-4f53-b774-22b111935391"),
    "Soltari Trooper",
    CardArt::new("32f74aa3-4003-4f53-b774-22b111935391", "Kev Walker"),
    CardSet::Tempest,
    // Shadow makes it unblockable in practice, so the attack bonus is the
    // whole of what two mana buys: two guaranteed damage a turn.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Soltari", "Soldier"], 1, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +1/+1 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 48 — Spirit Mirror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_MIRROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a7089c9-70ba-4009-86a5-d4e322c00fba"),
    "Spirit Mirror",
    crate::card::CardArt::new(
        "8a7089c9-70ba-4009-86a5-d4e322c00fba",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 49 — Staunch Defenders
pub(in crate::card::sets) static STAUNCH_DEFENDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88ed7210-17a4-4750-a003-617ba75bff3e"),
    "Staunch Defenders",
    CardArt::new("88ed7210-17a4-4750-a003-617ba75bff3e", "Mark Poole"),
    CardSet::Tempest,
    // Five mana for a body and four life, which is what white paid to stop
    // losing races it had already stabilised.
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Soldier"], 3, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you gain 4 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// TMP 50 — Talon Sliver
pub(in crate::card::sets) static TALON_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f186c4b1-b7ec-46eb-a961-257411b401b0"),
    "Talon Sliver",
    CardArt::new("f186c4b1-b7ec-46eb-a961-257411b401b0", "Mike Raabe"),
    CardSet::Tempest,
    // The white member of the cycle, which turns every Sliver on the board
    // into a creature nothing profitably blocks.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have first strike.",
            EffectDef::StaticApply {
                // "All", not "you control": it hands the keyword to the
                // opponent's as readily, which is the drawback these were
                // priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::first_strike() }),
            },
        ),
    ),
);

// TMP 51 — Warmth
pub(in crate::card::sets) static WARMTH: CardRecord = CardRecord::new_with_legacy_id(
    286,
    "Warmth",
    CardArt::new("d7dbeea8-06b0-4482-bdae-aa82b9db8856", "Drew Tucker"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a red spell, you gain 2 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::Color(ManaColor::Red),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// TMP 52 — Winds of Rath
pub(in crate::card::sets) static WINDS_OF_RATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6d731b2-0113-4fd5-8b78-1aa1064bb4f5"),
    "Winds of Rath",
    CardArt::new("a6d731b2-0113-4fd5-8b78-1aa1064bb4f5", "Drew Tucker"),
    CardSet::Tempest,
    // A wrath that spares whatever is wearing an Aura, which in the block
    // that printed it meant your own board survived.
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all creatures that aren't enchanted. They can't be regenerated.",
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &const {
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Enchanted),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                }
            },
        },
    )),
);

// TMP 53 — Worthy Cause
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORTHY_CAUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f610d8b-1782-43a4-bfb3-40887bdedba0"),
    "Worthy Cause",
    crate::card::CardArt::new("1f610d8b-1782-43a4-bfb3-40887bdedba0", "John Matson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 54 — Benthic Behemoth
pub(in crate::card::sets) static BENTHIC_BEHEMOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc9fb7b6-d20c-4c08-9dae-4ccc9138b662"),
    "Benthic Behemoth",
    CardArt::new("cc9fb7b6-d20c-4c08-9dae-4ccc9138b662", "Jim Nelson"),
    CardSet::Tempest,
    // Eight mana for seven unblockable damage against any blue deck, and a
    // 7/6 that gets chumped against everybody else.
    CardRules::new_creature(mana_cost!("{5}{U}{U}{U}"), &["Serpent"], 7, 6)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// TMP 55 — Capsize
pub(in crate::card::sets) static CAPSIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e538b359-d893-422d-9d60-5f3e8ee0fa9e"),
    "Capsize",
    CardArt::new("e538b359-d893-422d-9d60-5f3e8ee0fa9e", "Tom Wänerstrand"),
    CardSet::Tempest,
    // Six mana a turn to bounce anything forever: the buyback is the card,
    // and the unbought half is only what you cast when tapped out.
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::spell_with_targets(
            "Return target permanent to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// TMP 56 — Chill
pub(in crate::card::sets) static CHILL: CardRecord = CardRecord::new_with_legacy_id(
    2038,
    "Chill",
    CardArt::new("5a7bd777-6f11-441e-887f-9cee1ef96035", "Greg Simanson"),
    CardSet::Tempest,
    // Two extra mana on every burn spell, which is most of what a red deck
    // has to say.
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_ability(AbilityDef::static_ability(
        "Red spells cost {2} more to cast.",
        EffectDef::ModifyCost(CostModificationDef::increase_spell(
            ObjectPredicateDef::Color(ManaColor::Red),
            PlayerRelation::Any,
            mana_cost!("{2}"),
        )),
    )),
);

// TMP 57 — Counterspell (reprint)

// TMP 58 — Dismiss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISMISS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e55d6be-7682-4786-9872-e847afd710b0"),
    "Dismiss",
    crate::card::CardArt::new("1e55d6be-7682-4786-9872-e847afd710b0", "Donato Giancola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 59 — Dream Cache (reprint)

// TMP 60 — Duplicity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUPLICITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d529cb33-292d-40e3-8cfe-db5eeb0d711e"),
    "Duplicity",
    crate::card::CardArt::new("d529cb33-292d-40e3-8cfe-db5eeb0d711e", "Dan Frazier"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 61 — Ertai's Meddling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_S_MEDDLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35c7e7fa-1493-4ef8-9cdb-b02b07a1ad85"),
    "Ertai's Meddling",
    crate::card::CardArt::new("35c7e7fa-1493-4ef8-9cdb-b02b07a1ad85", "Steve Luke"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 62 — Escaped Shapeshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESCAPED_SHAPESHIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e0171d4f-c871-4a7f-821a-82b7f401e9ca"),
    "Escaped Shapeshifter",
    crate::card::CardArt::new("e0171d4f-c871-4a7f-821a-82b7f401e9ca", "Douglas Shuler"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 63 — Fighting Drake
pub(in crate::card::sets) static FIGHTING_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be436b65-9193-45ca-93e0-c5e9718f7e72"),
    "Fighting Drake",
    CardArt::new("be436b65-9193-45ca-93e0-c5e9718f7e72", "DiTerlizzi"),
    CardSet::Tempest,
    // A flier that blocks fliers and lives, which is what blue wanted from
    // four mana before it wanted card advantage.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Drake"], 2, 4)
        .with_abilities(&[abilities::flying()]),
);

// TMP 64 — Fylamarid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYLAMARID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dd4f686-79e3-4067-81f9-7fae0c25dc8f"),
    "Fylamarid",
    crate::card::CardArt::new("8dd4f686-79e3-4067-81f9-7fae0c25dc8f", "Una Fricker"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 65 — Gaseous Form (reprint)

// TMP 66 — Giant Crab
pub(in crate::card::sets) static GIANT_CRAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11c65a35-e219-4b60-ab95-ce7eff67d646"),
    "Giant Crab",
    CardArt::new("11c65a35-e219-4b60-ab95-ce7eff67d646", "Tom Kyffin"),
    CardSet::Tempest,
    // One mana makes a 3/3 untargetable, which is what blue paid instead of
    // counterspells to protect a body.
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Crab"], 3, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains shroud until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::shroud() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 67 — Horned Turtle
pub(in crate::card::sets) static HORNED_TURTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7d25497-36b4-48b9-ba01-f24f6222d6be"),
    "Horned Turtle",
    CardArt::new("b2348ce1-6305-42a7-8061-64275f6dc5c6", "DiTerlizzi"),
    CardSet::Tempest,
    // A vanilla wall that is not a Wall: it can attack, and every so often
    // that matters.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Turtle"], 1, 4),
);

// TMP 68 — Insight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1dfd9cb9-51f6-4d09-b5c0-5b0ed9d16542"),
    "Insight",
    crate::card::CardArt::new("1dfd9cb9-51f6-4d09-b5c0-5b0ed9d16542", "Ron Chironna"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 69 — Interdict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTERDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3442c919-73b9-4d29-a014-87293f456325"),
    "Interdict",
    crate::card::CardArt::new("3442c919-73b9-4d29-a014-87293f456325", "Jeff Laubenstein"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 70 — Intuition
pub(in crate::card::sets) static INTUITION: CardRecord = CardRecord::new_with_legacy_id(
    2084,
    "Intuition",
    CardArt::new("19eae4ac-10a4-4860-bcc2-0c9816f8bcdd", "April Lee"),
    CardSet::Tempest,
    // Naming three copies of one card makes the opponent's choice no choice
    // at all; naming three different ones is how a graveyard deck fills its
    // graveyard and keeps the piece it needs.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Search your library for three cards and reveal them. Target opponent chooses one. Put that card into your hand and the rest into your graveyard. Then shuffle.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 3,
            maximum: ValueDef::Constant(3),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: Some(ParentBinding),
            // The opponent picks which of the three is worth giving up, out of the
            // cards the search found rather than out of the library it found them in.
            then: Some(&const {
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(Binding!("intuition_chosen")),
                    unchosen: Some(Binding!("intuition_unchosen")),
                    chooser: PlayerRefDef::Target(TargetIndex::PRIMARY),
                    candidates: ObjectSetDef::Binding(ParentBinding),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    // The one the opponent hands over, and the two they keep back. Both halves
                    // are one partition of the three that were found, which is why the choice
                    // names the rest as well as the pick.
                    then: &const {
                        EffectDef::Sequence(&const {
                            [
                                EffectDef::MoveToZone {
                                    object: EffectRecipientDef::object(ObjectRefDef::Binding(
                                        Binding!("intuition_chosen"),
                                    )),
                                    zone: ZoneKind::Hand,
                                    placement: ZonePlacement::Top,
                                },
                                EffectDef::MoveToZone {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        Binding!("intuition_unchosen"),
                                    )),
                                    zone: ZoneKind::Graveyard,
                                    placement: ZonePlacement::Top,
                                },
                            ]
                        })
                    },
                })
            }),
        },
    )),
);

// TMP 71 — Legacy's Allure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEGACY_S_ALLURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("649a89c5-71bd-4fee-ae35-78081e4e0353"),
    "Legacy's Allure",
    crate::card::CardArt::new("649a89c5-71bd-4fee-ae35-78081e4e0353", "Daren Bader"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 72 — Legerdemain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEGERDEMAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65506830-fa2c-4e3b-9f64-5a569dd28249"),
    "Legerdemain",
    crate::card::CardArt::new("65506830-fa2c-4e3b-9f64-5a569dd28249", "Daren Bader"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 73 — Mana Severance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_SEVERANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("854dc5e6-63f7-4c8b-83e5-a364f41c9a15"),
    "Mana Severance",
    crate::card::CardArt::new("854dc5e6-63f7-4c8b-83e5-a364f41c9a15", "Terese Nielsen"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 74 — Manta Riders
pub(in crate::card::sets) static MANTA_RIDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdff306c-1c7e-49ae-b10f-99e1927bbef1"),
    "Manta Riders",
    CardArt::new("cdff306c-1c7e-49ae-b10f-99e1927bbef1", "Kaja Foglio"),
    CardSet::Tempest,
    // One mana for a creature that is unblockable whenever you have another
    // blue mana, which is most turns.
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk"], 1, 1).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 75 — Mawcor
pub(in crate::card::sets) static MAWCOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f50971e-2a18-4db7-8b5b-83dd5e85766e"),
    "Mawcor",
    CardArt::new("9f50971e-2a18-4db7-8b5b-83dd5e85766e", "John Matson"),
    CardSet::Tempest,
    // The same ping on a body that survives what it is shooting at.
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Beast"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[CostDef::TapSource],
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

// TMP 76 — Meditate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEDITATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edb79a97-c1fc-4aa3-bb13-3d24a6dabeea"),
    "Meditate",
    crate::card::CardArt::new("edb79a97-c1fc-4aa3-bb13-3d24a6dabeea", "Susan Van Camp"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 77 — Mnemonic Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MNEMONIC_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b167347-2f8f-4338-a651-c7543d812597"),
    "Mnemonic Sliver",
    crate::card::CardArt::new("2b167347-2f8f-4338-a651-c7543d812597", "Randy Gallegos"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 78 — Power Sink (reprint)

// TMP 79 — Precognition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRECOGNITION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76a0e317-5a76-4eac-a903-b0e3f0a45873"),
    "Precognition",
    crate::card::CardArt::new("76a0e317-5a76-4eac-a903-b0e3f0a45873", "Jeff Miracola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 80 — Propaganda
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROPAGANDA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f67dde4d-3df1-480d-a8b8-ab22c768bb12"),
    "Propaganda",
    crate::card::CardArt::new("f67dde4d-3df1-480d-a8b8-ab22c768bb12", "Jeff Miracola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 81 — Rootwater Diver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_DIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6315323-cf82-46c0-b164-e6ea1bf809f4"),
    "Rootwater Diver",
    crate::card::CardArt::new("a6315323-cf82-46c0-b164-e6ea1bf809f4", "Ron Spencer"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 82 — Rootwater Hunter
pub(in crate::card::sets) static ROOTWATER_HUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdf7ea34-2cde-4ec5-9b12-99b0002da986"),
    "Rootwater Hunter",
    CardArt::new("cdf7ea34-2cde-4ec5-9b12-99b0002da986", "Brom"),
    CardSet::Tempest,
    // A pinger in blue, which is unusual enough that the three-mana 1/1
    // body is beside the point.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[CostDef::TapSource],
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

// TMP 83 — Rootwater Matriarch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_MATRIARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec46812d-0721-4e93-b1a7-1d38f477fab6"),
    "Rootwater Matriarch",
    crate::card::CardArt::new("ec46812d-0721-4e93-b1a7-1d38f477fab6", "Randy Gallegos"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 84 — Rootwater Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("caa1b84b-efda-4324-9106-0d1d00385cdc"),
    "Rootwater Shaman",
    crate::card::CardArt::new("caa1b84b-efda-4324-9106-0d1d00385cdc", "Paolo Parente"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 85 — Sea Monster
pub(in crate::card::sets) static SEA_MONSTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d3837ac-54af-44f7-b576-ad5badbee9f2"),
    "Sea Monster",
    CardArt::new("8d3837ac-54af-44f7-b576-ad5badbee9f2", "Daniel Gelon"),
    CardSet::Tempest,
    // Six power for six mana, and a blocker in every matchup where it
    // cannot attack -- which is most of them.
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Serpent"], 6, 6).with_ability(
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            // Read at declaration, and off the defending player rather
            // than the controller: it is their Islands that let it swim.
            EffectDef::CannotAttackUnless(
                &const {
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )
                },
            ),
        ),
    ),
);

// TMP 86 — Shadow Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOW_RIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57c11175-9feb-4801-9b46-d577d5ecef40"),
    "Shadow Rift",
    crate::card::CardArt::new("57c11175-9feb-4801-9b46-d577d5ecef40", "Adam Rex"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 87 — Shimmering Wings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERING_WINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6a8dc46-04c7-479a-90c1-b55e6c67e0e3"),
    "Shimmering Wings",
    crate::card::CardArt::new("a6a8dc46-04c7-479a-90c1-b55e6c67e0e3", "Steve Luke"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 88 — Skyshroud Condor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_CONDOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7d05ef5-c046-4929-b59d-988f0313a645"),
    "Skyshroud Condor",
    crate::card::CardArt::new("a7d05ef5-c046-4929-b59d-988f0313a645", "Doug Chaffee"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 89 — Spell Blast (reprint)

// TMP 90 — Steal Enchantment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAL_ENCHANTMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("734be7fa-0998-4771-9b97-4989b3fc1471"),
    "Steal Enchantment",
    crate::card::CardArt::new("734be7fa-0998-4771-9b97-4989b3fc1471", "Hannibal King"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 91 — Stinging Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STINGING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("807227d7-2eb2-4d47-bb3c-9d1ec9befeb7"),
    "Stinging Licid",
    crate::card::CardArt::new("807227d7-2eb2-4d47-bb3c-9d1ec9befeb7", "Paolo Parente"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 92 — Thalakos Dreamsower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THALAKOS_DREAMSOWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d725cdc0-3a85-4722-bb13-40c336f511b6"),
    "Thalakos Dreamsower",
    crate::card::CardArt::new("d725cdc0-3a85-4722-bb13-40c336f511b6", "Susan Van Camp"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 93 — Thalakos Mistfolk
pub(in crate::card::sets) static THALAKOS_MISTFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e7b5b00-9d14-4090-b8c3-28b70375571e"),
    "Thalakos Mistfolk",
    CardArt::new(
        "9e7b5b00-9d14-4090-b8c3-28b70375571e",
        "Richard Kane Ferguson",
    ),
    CardSet::Tempest,
    // Unblockable, and it can dodge removal by going back to the top of the
    // library -- at the price of the next draw.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Thalakos", "Illusion"], 2, 1).with_abilities(
        &[
            abilities::shadow(),
            AbilityDef::activated(
                "{U}: Put this creature on top of its owner's library.",
                &[CostDef::Mana(mana_cost!("{U}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                },
            ),
        ],
    ),
);

// TMP 94 — Thalakos Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THALAKOS_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("136a7d63-94ae-4d92-86ab-12bf9d78a803"),
    "Thalakos Seer",
    crate::card::CardArt::new("136a7d63-94ae-4d92-86ab-12bf9d78a803", "Ron Spencer"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 95 — Thalakos Sentry
pub(in crate::card::sets) static THALAKOS_SENTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("739a13d6-5f73-4166-b923-9db8ee3f2cf7"),
    "Thalakos Sentry",
    CardArt::new("739a13d6-5f73-4166-b923-9db8ee3f2cf7", "Andrew Robinson"),
    CardSet::Tempest,
    // Blue's shadow body: it connects every turn and blocks the other
    // shadow creatures.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Thalakos", "Soldier"], 1, 2)
        .with_ability(abilities::shadow()),
);

// TMP 96 — Time Ebb (reprint)

// TMP 97 — Time Warp
pub(in crate::card::sets) static TIME_WARP: CardRecord = CardRecord::new_with_legacy_id(
    2109,
    "Time Warp",
    CardArt::new("3447aeaf-3b26-442a-99d4-0a7ee76c8e76", "Pete Venters"),
    CardSet::Tempest,
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player takes an extra turn after this one.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::TakeExtraTurn {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// TMP 98 — Tradewind Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADEWIND_RIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("09412374-3645-4644-952e-2beaefb3104b"),
    "Tradewind Rider",
    crate::card::CardArt::new("09412374-3645-4644-952e-2beaefb3104b", "John Matson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 99 — Twitch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cba021eb-3d8b-41bf-aec4-af211e0860ad"),
    "Twitch",
    crate::card::CardArt::new("cba021eb-3d8b-41bf-aec4-af211e0860ad", "DiTerlizzi"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 100 — Unstable Shapeshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSTABLE_SHAPESHIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84e8cbd4-f49d-420d-a027-3be64ca58989"),
    "Unstable Shapeshifter",
    crate::card::CardArt::new("84e8cbd4-f49d-420d-a027-3be64ca58989", "Terese Nielsen"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 101 — Volrath's Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_S_CURSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bce63d86-8748-428a-aa9c-d3c0526537a2"),
    "Volrath's Curse",
    crate::card::CardArt::new("bce63d86-8748-428a-aa9c-d3c0526537a2", "Daren Bader"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 102 — Whim of Volrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIM_OF_VOLRATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e259da60-c8bc-4a77-98ed-e529dc067732"),
    "Whim of Volrath",
    crate::card::CardArt::new("e259da60-c8bc-4a77-98ed-e529dc067732", "Anthony S. Waters"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 103 — Whispers of the Muse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHISPERS_OF_THE_MUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75c5cfd1-3f7c-4250-a84d-8db83c6d7eb7"),
    "Whispers of the Muse",
    crate::card::CardArt::new("75c5cfd1-3f7c-4250-a84d-8db83c6d7eb7", "Quinton Hoover"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 104 — Wind Dancer
pub(in crate::card::sets) static WIND_DANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea7f7a94-700a-4f3b-846c-a36505b80875"),
    "Wind Dancer",
    CardArt::new("ea7f7a94-700a-4f3b-846c-a36505b80875", "Susan Van Camp"),
    CardSet::Tempest,
    // Evasion by the turn, which is what a one-power flier is for in a
    // deck that has something worth pushing through.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}: Target creature gains flying until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 105 — Wind Drake (reprint)

// TMP 106 — Winged Sliver
pub(in crate::card::sets) static WINGED_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("03aa58b4-dbc2-414e-aa7a-f09360d59b3c"),
    "Winged Sliver",
    CardArt::new("03aa58b4-dbc2-414e-aa7a-f09360d59b3c", "Anthony S. Waters"),
    CardSet::Tempest,
    // Evasion for the whole tribe, which is what turned a board of 1/1s
    // into a clock.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have flying.",
            EffectDef::StaticApply {
                // "All", not "you control": it hands the keyword to the
                // opponent's as readily, which is the drawback these were
                // priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
            },
        ),
    ),
);

// TMP 107 — Abandon Hope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABANDON_HOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("942cf220-472c-48f6-8f60-993939ea5ab8"),
    "Abandon Hope",
    crate::card::CardArt::new("942cf220-472c-48f6-8f60-993939ea5ab8", "Alan Pollack"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 108 — Bellowing Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELLOWING_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26915b4b-ada1-45f3-b908-04a774011b66"),
    "Bellowing Fiend",
    crate::card::CardArt::new("26915b4b-ada1-45f3-b908-04a774011b66", "Jim Nelson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 109 — Blood Pet
pub(in crate::card::sets) static BLOOD_PET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a89ba1b-e68b-4d70-a25e-27be9bf48a3b"),
    "Blood Pet",
    CardArt::new("5a89ba1b-e68b-4d70-a25e-27be9bf48a3b", "Brom"),
    CardSet::Tempest,
    // A one-drop that is really a ritual with a body attached, which an
    // aggressive black deck counted as both.
    CardRules::new_creature(mana_cost!("{B}"), &["Thrull"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: Add {B}.",
            &[CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ),
);

// TMP 110 — Bounty Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNTY_HUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98319fd3-0aad-4fc3-bb83-3c027d0ed652"),
    "Bounty Hunter",
    crate::card::CardArt::new("98319fd3-0aad-4fc3-bb83-3c027d0ed652", "Brian Snõddy"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 111 — Carrionette
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRIONETTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("884e19fb-67a4-42d8-b163-720a99cb8506"),
    "Carrionette",
    crate::card::CardArt::new("884e19fb-67a4-42d8-b163-720a99cb8506", "Pete Venters"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 112 — Clot Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOT_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fdead1f4-a6e4-4370-80ae-811881a90d01"),
    "Clot Sliver",
    crate::card::CardArt::new("fdead1f4-a6e4-4370-80ae-811881a90d01", "Jeff Laubenstein"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 113 — Coercion (reprint)

// TMP 114 — Coffin Queen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COFFIN_QUEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edf8af70-c26d-4b78-aad6-bd51b5afc590"),
    "Coffin Queen",
    crate::card::CardArt::new("edf8af70-c26d-4b78-aad6-bd51b5afc590", "Kaja Foglio"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 115 — Commander Greven il-Vec
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMANDER_GREVEN_IL_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab0ce69f-a259-4801-9ac3-f6754040434c"),
    "Commander Greven il-Vec",
    crate::card::CardArt::new("ab0ce69f-a259-4801-9ac3-f6754040434c", "Kev Walker"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 116 — Corpse Dance
pub(in crate::card::sets) static CORPSE_DANCE: CardRecord = CardRecord::new_with_legacy_id(
    2187,
    "Corpse Dance",
    CardArt::new("76ae81ea-13e3-4ab8-b956-4c7b139a5e9c", "Brian Snõddy"),
    CardSet::Tempest,
    // Shallow Grave that comes back, which is why five mana a turn is a
    // price worth paying: whatever is on top of the graveyard attacks every
    // turn from here, and the card is never spent.
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        abilities::buyback(mana_cost!("{2}")),
        AbilityDef::spell(
            "Return the top creature card of your graveyard to the battlefield. That creature gains haste until end of turn. Exile it at the beginning of the next end step.",
            EffectDef::WithZoneMoveResult {
                effect: &const {
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::objects(ObjectSetDef::TopOfGraveyardMatching {
                            player: PlayerRefDef::EffectController,
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                        }),
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    }
                },
                binding: ParentBinding,
                then: &const {
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::binding_zone_change_successors(
                            ParentBinding,
                        ),
                        effect: AppliedEffectDef::Composite(&const {
                            [
                                AppliedEffectDef::add_ability(&const { abilities::haste() }),
                                // The creature exiles itself rather than being named by a delayed trigger:
                                // it is the object that arrived, and it carries the clause with it.
                                AppliedEffectDef::add_ability(&const {
                                    AbilityDef::triggered(
                                        "At the beginning of the next end step, exile this creature.",
                                        TriggerEventDef::StepBegins {
                                            step: TurnStepDef::End,
                                            player: PlayerRelation::Any,
                                        },
                                        EffectDef::MoveToZone {
                                            object: EffectRecipientDef::Source,
                                            zone: ZoneKind::Exile,
                                            placement: ZonePlacement::Top,
                                        },
                                    )
                                }),
                            ]
                        }),
                        duration: crate::card::ResolvedEffectDurationDef::Permanent,
                    }
                },
            },
        ),
    ]),
);

// TMP 117 — Dark Banishing (reprint)

// TMP 118 — Dark Ritual (reprint)

// TMP 119 — Darkling Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARKLING_STALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4eb883b7-da6a-45c3-9dde-61334a0ddcae"),
    "Darkling Stalker",
    crate::card::CardArt::new("4eb883b7-da6a-45c3-9dde-61334a0ddcae", "Susan Van Camp"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 120 — Dauthi Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_EMBRACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e84bb94-d654-4d69-89d9-0a398a940125"),
    "Dauthi Embrace",
    crate::card::CardArt::new("7e84bb94-d654-4d69-89d9-0a398a940125", "Andrew Robinson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 121 — Dauthi Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a70778e-8171-4d97-b86c-d4d92b7e7f06"),
    "Dauthi Ghoul",
    crate::card::CardArt::new("0a70778e-8171-4d97-b86c-d4d92b7e7f06", "Tom Kyffin"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 122 — Dauthi Horror (reprint)

// TMP 123 — Dauthi Marauder
pub(in crate::card::sets) static DAUTHI_MARAUDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee847d84-ec8d-4ec3-8436-68d6f144e22f"),
    "Dauthi Marauder",
    CardArt::new("ee847d84-ec8d-4ec3-8436-68d6f144e22f", "Andrew Robinson"),
    CardSet::Tempest,
    // Three unblockable damage a turn for three mana, and no defence at all
    // -- the fastest clock in the cycle.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Dauthi", "Minion"], 3, 1)
        .with_ability(abilities::shadow()),
);

// TMP 124 — Dauthi Mercenary
pub(in crate::card::sets) static DAUTHI_MERCENARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c340e779-c648-48fd-a159-174b46f2d1b3"),
    "Dauthi Mercenary",
    CardArt::new("c340e779-c648-48fd-a159-174b46f2d1b3", "Matthew D. Wilson"),
    CardSet::Tempest,
    // The black member of the same pair, which needed the other colour to
    // turn its mana into damage.
    CardRules::new_creature(
        mana_cost!("{2}{B}"),
        &["Dauthi", "Knight", "Mercenary"],
        2,
        1,
    )
    .with_abilities(&[
        abilities::shadow(),
        AbilityDef::activated(
            "{1}{B}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
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

// TMP 125 — Dauthi Mindripper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_MINDRIPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fb990ee-f027-4c74-a67e-98ada6aa21e4"),
    "Dauthi Mindripper",
    crate::card::CardArt::new("6fb990ee-f027-4c74-a67e-98ada6aa21e4", "Allen Williams"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 126 — Dauthi Slayer
pub(in crate::card::sets) static DAUTHI_SLAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("652ccd79-aefd-4b45-b747-75190da0cfc6"),
    "Dauthi Slayer",
    CardArt::new("652ccd79-aefd-4b45-b747-75190da0cfc6", "Dermot Power"),
    CardSet::Tempest,
    // Shadow means the forced attack costs nothing most games: almost
    // nothing can block it, so the drawback only bites against another
    // shadow creature.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Dauthi", "Soldier"], 2, 2).with_abilities(&[
        abilities::shadow(),
        abilities::attacks_each_combat_if_able(),
    ]),
);

// TMP 127 — Death Pits of Rath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_PITS_OF_RATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72122e8f-97ab-495e-aade-5d736c432873"),
    "Death Pits of Rath",
    crate::card::CardArt::new("72122e8f-97ab-495e-aade-5d736c432873", "Joel Biske"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 128 — Diabolic Edict
pub(in crate::card::sets) static DIABOLIC_EDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2ecf2ee-1e2d-4ab2-8b2c-717c794b09b2"),
    "Diabolic Edict",
    CardArt::new("a2ecf2ee-1e2d-4ab2-8b2c-717c794b09b2", "Ron Spencer"),
    CardSet::Tempest,
    // Two mana at instant speed that answers a creature nothing else can
    // touch, at the price of letting them pick which.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player sacrifices a creature of their choice.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        // They choose, so it goes around hexproof and shroud and takes their
        // worst creature rather than their best.
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            object: ObjectPredicateDef::HasType(CardType::Creature),
            count: ValueDef::Constant(1),
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )),
);

// TMP 129 — Disturbed Burial
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISTURBED_BURIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06254b6c-eb22-4ec9-9420-74e9ee15e072"),
    "Disturbed Burial",
    crate::card::CardArt::new("06254b6c-eb22-4ec9-9420-74e9ee15e072", "Heather Hudson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 130 — Dread of Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_OF_NIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d08586d4-8163-454c-b8d8-c5034c4aee6c"),
    "Dread of Night",
    crate::card::CardArt::new("d08586d4-8163-454c-b8d8-c5034c4aee6c", "Richard Thomas"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 131 — Dregs of Sorrow
pub(in crate::card::sets) static DREGS_OF_SORROW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80e4203a-ff11-4075-a03a-11448779b413"),
    "Dregs of Sorrow",
    CardArt::new("80e4203a-ff11-4075-a03a-11448779b413", "Thomas Gianni"),
    CardSet::Tempest,
    // Removal and cards in one card at a rate nobody would pay before turn
    // seven, which is exactly when it is castable.
    CardRules::new_sorcery(mana_cost!("{X}{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy X target nonblack creatures. Draw X cards.",
        &[AbilityTargetDef::exactly_chosen_x(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // The same X the targets were chosen for, so a countered or
            // vanished target does not reduce the draw.
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ChosenX,
            },
        ]),
    )),
);

// TMP 132 — Endless Scream
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDLESS_SCREAM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9474231-34c5-4563-8a61-fd1bc2693f86"),
    "Endless Scream",
    crate::card::CardArt::new("e9474231-34c5-4563-8a61-fd1bc2693f86", "Joel Biske"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 133 — Enfeeblement (reprint)

// TMP 134 — Evincar's Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVINCAR_S_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d53f46f-b069-4b34-af4b-98143328c078"),
    "Evincar's Justice",
    crate::card::CardArt::new("5d53f46f-b069-4b34-af4b-98143328c078", "Hannibal King"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 135 — Extinction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTINCTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a233a244-7f84-4525-b0ce-e10db0a95385"),
    "Extinction",
    crate::card::CardArt::new("a233a244-7f84-4525-b0ce-e10db0a95385", "Una Fricker"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 136 — Fevered Convulsions
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEVERED_CONVULSIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a790769-e76e-49e9-9d6d-05ce8e858243"),
    "Fevered Convulsions",
    crate::card::CardArt::new("3a790769-e76e-49e9-9d6d-05ce8e858243", "Jeff Miracola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 137 — Gravedigger (reprint)

// TMP 138 — Imps' Taunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPS_TAUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79d6ed64-9f5c-4233-85a9-028b8e5949c3"),
    "Imps' Taunt",
    crate::card::CardArt::new("79d6ed64-9f5c-4233-85a9-028b8e5949c3", "Colin MacNeil"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 139 — Kezzerdrix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEZZERDRIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23b95d3a-bb19-474d-9939-8817038fe9fc"),
    "Kezzerdrix",
    crate::card::CardArt::new("23b95d3a-bb19-474d-9939-8817038fe9fc", "Matthew D. Wilson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 140 — Knight of Dusk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_DUSK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8aba09c4-9259-4743-9e4a-a63505f1efe6"),
    "Knight of Dusk",
    crate::card::CardArt::new("8aba09c4-9259-4743-9e4a-a63505f1efe6", "Ron Spencer"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 141 — Leeching Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEECHING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27bffefb-23c0-4d03-b716-b1a7eff39a05"),
    "Leeching Licid",
    crate::card::CardArt::new("27bffefb-23c0-4d03-b716-b1a7eff39a05", "Joel Biske"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 142 — Living Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_DEATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c820476-fbda-4073-baf6-51e71f45ed58"),
    "Living Death",
    crate::card::CardArt::new("6c820476-fbda-4073-baf6-51e71f45ed58", "Charles Gillespie"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 143 — Maddening Imp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MADDENING_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dda08eb5-c75c-4c21-bfd1-1f04a3575241"),
    "Maddening Imp",
    crate::card::CardArt::new("dda08eb5-c75c-4c21-bfd1-1f04a3575241", "Zina Saunders"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 144 — Marsh Lurker
pub(in crate::card::sets) static MARSH_LURKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90c4b759-f53d-4977-8d97-a93762622e75"),
    "Marsh Lurker",
    CardArt::new("90c4b759-f53d-4977-8d97-a93762622e75", "Tom Kyffin"),
    CardSet::Tempest,
    // Lands for evasion, so it gets through once or twice and then the
    // deck has paid for it.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Beast"], 3, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice a Swamp: This creature gains fear until end of turn.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: abilities::FEAR_RESTRICTION,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 145 — Mindwhip Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINDWHIP_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa966fbb-140d-4057-a4fc-998ebe07c307"),
    "Mindwhip Sliver",
    crate::card::CardArt::new("fa966fbb-140d-4057-a4fc-998ebe07c307", "Jeff Miracola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 146 — Minion of the Wastes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_THE_WASTES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9f120fc-c681-47b6-827e-1cc7ead47a0f"),
    "Minion of the Wastes",
    crate::card::CardArt::new("d9f120fc-c681-47b6-827e-1cc7ead47a0f", "Scott Kirschner"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 147 — Perish
pub(in crate::card::sets) static PERISH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e47ace1d-73de-44aa-a3fe-2e2a21ebec79"),
    "Perish",
    CardArt::new("e47ace1d-73de-44aa-a3fe-2e2a21ebec79", "Rebecca Guay"),
    CardSet::Tempest,
    // Three mana that answers a whole green board, printed for a format
    // where that was most boards.
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell(
        "Destroy all green creatures. They can't be regenerated.",
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &const {
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Color(ManaColor::Green),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                }
            },
        },
    )),
);

// TMP 148 — Pit Imp
pub(in crate::card::sets) static PIT_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24c7acfe-b5b2-426f-a5a1-1ff8ef7ebf72"),
    "Pit Imp",
    CardArt::new("24c7acfe-b5b2-426f-a5a1-1ff8ef7ebf72", "Phil Foglio"),
    CardSet::Tempest,
    // A 0/1 flier that becomes a 2/1 for two more mana, capped so it cannot
    // simply eat the whole hand.
    CardRules::new_creature(mana_cost!("{B}"), &["Imp"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{B}: This creature gets +1/+0 until end of turn. Activate no more than twice each turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .activations_each_turn(2),
    ]),
);

// TMP 149 — Rain of Tears
pub(in crate::card::sets) static RAIN_OF_TEARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("803ba4ef-24ed-4f45-aed8-f9442322e31e"),
    "Rain of Tears",
    CardArt::new("cad93919-273f-4a26-8ebd-13503dd6b220", "Charles Gillespie"),
    CardSet::Tempest,
    // Three mana to set them back one, which is a losing trade unless the
    // deck is built to keep doing it.
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// TMP 150 — Rats of Rath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATS_OF_RATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7cb8d3a2-ed96-4490-9432-401da19ad3c5"),
    "Rats of Rath",
    crate::card::CardArt::new("7cb8d3a2-ed96-4490-9432-401da19ad3c5", "John Matson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 151 — Reanimate
pub(in crate::card::sets) static REANIMATE: CardRecord = CardRecord::new_with_legacy_id(
    305,
    "Reanimate",
    CardArt::new("fc00f897-988b-4602-969a-c510804ec12a", "Robert Bliss"),
    CardSet::Tempest,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature card from a graveyard onto the battlefield under your control. You lose life equal to that card's mana value.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: None,
        })],
        EffectDef::Sequence(&[
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                arrival: crate::card::BattlefieldArrivalDef {
                    controller: Some(PlayerRelation::You),
                    ..crate::card::BattlefieldArrivalDef::DEFAULT
                },
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// TMP 152 — Reckless Spite
pub(in crate::card::sets) static RECKLESS_SPITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9141daea-1f4f-4227-b7d7-20753e3cb4d4"),
    "Reckless Spite",
    CardArt::new("9141daea-1f4f-4227-b7d7-20753e3cb4d4", "Pete Venters"),
    CardSet::Tempest,
    // Two creatures for three mana at instant speed, and five life is what
    // that rate costs.
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy two target nonblack creatures. You lose 5 life.",
        // Exactly two, so it needs two legal targets to be cast at all.
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            ValueDef::Constant(2),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ]),
    )),
);

// TMP 153 — Sadistic Glee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SADISTIC_GLEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9e1959c-b87b-4e17-a0d2-0489ea79220b"),
    "Sadistic Glee",
    crate::card::CardArt::new("d9e1959c-b87b-4e17-a0d2-0489ea79220b", "Pete Venters"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 154 — Sarcomancy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SARCOMANCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb5730f5-a44c-4f75-a26f-90815cfcd31e"),
    "Sarcomancy",
    crate::card::CardArt::new("eb5730f5-a44c-4f75-a26f-90815cfcd31e", "Daren Bader"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 155 — Screeching Harpy
pub(in crate::card::sets) static SCREECHING_HARPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10c02902-4e3a-445e-9dd9-116806ddc966"),
    "Screeching Harpy",
    CardArt::new("10c02902-4e3a-445e-9dd9-116806ddc966", "Una Fricker"),
    CardSet::Tempest,
    // Four mana for a 2/2 flier is a bad rate; the regeneration is what it
    // is really selling, and it needs the mana free to matter.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Harpy", "Beast"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "{1}{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
        ),
    ]),
);

// TMP 156 — Servant of Volrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERVANT_OF_VOLRATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("691afabb-f266-45fd-b5a3-577be4f10f86"),
    "Servant of Volrath",
    crate::card::CardArt::new("691afabb-f266-45fd-b5a3-577be4f10f86", "Brian Snõddy"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 157 — Skyshroud Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_VAMPIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eed2c97b-f003-436c-9faa-5518aba42fc1"),
    "Skyshroud Vampire",
    crate::card::CardArt::new("eed2c97b-f003-436c-9faa-5518aba42fc1", "Gary Leach"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 158 — Souldrinker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULDRINKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07d2d0ff-e44e-427a-9d68-3ed2d51b1b86"),
    "Souldrinker",
    crate::card::CardArt::new("07d2d0ff-e44e-427a-9d68-3ed2d51b1b86", "Dermot Power"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 159 — Spinal Graft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINAL_GRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a073b0c-2309-4070-a18e-0937ec8d4d1c"),
    "Spinal Graft",
    crate::card::CardArt::new("3a073b0c-2309-4070-a18e-0937ec8d4d1c", "Ron Spencer"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 160 — Aftershock
pub(in crate::card::sets) static AFTERSHOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c91a26b2-03f8-43f0-a3a4-ff6c5a3690c4"),
    "Aftershock",
    CardArt::new("c91a26b2-03f8-43f0-a3a4-ff6c5a3690c4", "Hannibal King"),
    CardSet::Tempest,
    // Four mana to answer anything, and three damage to yourself for the
    // privilege of not having to pick a category.
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact, creature, or land. Aftershock deals 3 damage to you.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // The damage happens whether or not the destruction did,
            // so it is a second clause rather than a follow-up.
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// TMP 161 — Ancient Runes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_RUNES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e315c1c2-436e-48ff-9214-938697178393"),
    "Ancient Runes",
    crate::card::CardArt::new("e315c1c2-436e-48ff-9214-938697178393", "Susan Van Camp"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 162 — Apocalypse
pub(in crate::card::sets) static APOCALYPSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ff23780-d183-4cca-ad0c-448ef325bf36"),
    "Apocalypse",
    CardArt::new("7ff23780-d183-4cca-ad0c-448ef325bf36", "Allen Williams"),
    CardSet::Tempest,
    // Everything goes, including your hand. It is a reset button rather
    // than a play, which is why it costs five.
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}{R}")).with_ability(AbilityDef::spell(
        "Exile all permanents. You discard your hand.",
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
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                // "Discard your hand": as many cards as there are, which is
                // how the catalog says it elsewhere.
                amount: ValueDef::Constant(i32::MAX),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// TMP 163 — Barbed Sliver
pub(in crate::card::sets) static BARBED_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19bddea7-daa7-4bdb-9b91-f7fcbc0d7a57"),
    "Barbed Sliver",
    CardArt::new("19bddea7-daa7-4bdb-9b91-f7fcbc0d7a57", "Scott Kirschner"),
    CardSet::Tempest,
    // The aggressive half: spare mana becomes damage on every Sliver at
    // once.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have \"{2}: This creature gets +1/+0 until end of turn.\"",
            EffectDef::StaticApply {
                // "All Sliver creatures", so the opponent's get it too.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                // "This creature" inside the granted ability is
                // whichever Sliver has it, which is that ability's own
                // source rather than this one.
                effect: AppliedEffectDef::add_ability(
                    &const {
                        AbilityDef::activated(
                            "{2}: This creature gets +1/+0 until end of turn.",
                            &[CostDef::Mana(mana_cost!("{2}"))],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(0),
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        )
                    },
                ),
            },
        ),
    ),
);

// TMP 164 — Blood Frenzy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_FRENZY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06f39d83-e1ea-45c9-8181-2a2b6e5148da"),
    "Blood Frenzy",
    crate::card::CardArt::new("06f39d83-e1ea-45c9-8181-2a2b6e5148da", "Paolo Parente"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 165 — Boil
pub(in crate::card::sets) static BOIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fa1c529-44e5-41b6-9704-ae2319f31f13"),
    "Boil",
    CardArt::new(
        "2fa1c529-44e5-41b6-9704-ae2319f31f13",
        "Jason Alexander Behnke",
    ),
    CardSet::Tempest,
    // Four mana at instant speed that answers a blue deck's whole mana
    // base, which is what a sideboard card was allowed to do.
    CardRules::new_instant(mana_cost!("{3}{R}")).with_ability(AbilityDef::spell(
        "Destroy all Islands.",
        // A subtype check, so any land that is an Island goes -- including a
        // dual land that is an Island among other types.
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            then: None,
        },
    )),
);

// TMP 166 — Canyon Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANYON_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22f84143-5912-43ca-a274-f26ed0dbadd0"),
    "Canyon Drake",
    crate::card::CardArt::new("22f84143-5912-43ca-a274-f26ed0dbadd0", "Quinton Hoover"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 167 — Canyon Wildcat
pub(in crate::card::sets) static CANYON_WILDCAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0169e52b-7909-4a8f-8ca2-62f030f9a85a"),
    "Canyon Wildcat",
    CardArt::new("0169e52b-7909-4a8f-8ca2-62f030f9a85a", "Gary Leach"),
    CardSet::Tempest,
    // Two damage a turn that the red mirror cannot block.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Cat"], 2, 1)
        .with_ability(abilities::landwalk(BasicLandType::Mountain)),
);

// TMP 168 — Chaotic Goo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOTIC_GOO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e9881a4-3078-4fe0-be09-54ddad1d18a0"),
    "Chaotic Goo",
    crate::card::CardArt::new("0e9881a4-3078-4fe0-be09-54ddad1d18a0", "Allen Williams"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 169 — Crown of Flames
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_FLAMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2c82741-2869-41f9-82f4-6ed88756e2fd"),
    "Crown of Flames",
    crate::card::CardArt::new("f2c82741-2869-41f9-82f4-6ed88756e2fd", "William O'Connor"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 170 — Deadshot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADSHOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55d212c5-1642-456f-829f-57f68a2116b6"),
    "Deadshot",
    crate::card::CardArt::new("55d212c5-1642-456f-829f-57f68a2116b6", "Heather Hudson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 171 — Enraging Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENRAGING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb7bff44-36e1-4855-aa2d-5c7bd6bf6f10"),
    "Enraging Licid",
    crate::card::CardArt::new("fb7bff44-36e1-4855-aa2d-5c7bd6bf6f10", "Doug Chaffee"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 172 — Firefly
pub(in crate::card::sets) static FIREFLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a312f0cf-225a-4f3d-b9a7-c47dd03b25c3"),
    "Firefly",
    CardArt::new("a312f0cf-225a-4f3d-b9a7-c47dd03b25c3", "Stephen Daniele"),
    CardSet::Tempest,
    // Uncapped, which is the difference: with mana open it is as big as the
    // turn allows.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
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

// TMP 173 — Fireslinger
pub(in crate::card::sets) static FIRESLINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de253d94-9968-47da-bb7a-9c8ebf50f4e0"),
    "Fireslinger",
    CardArt::new("de253d94-9968-47da-bb7a-9c8ebf50f4e0", "Jeff Reitz"),
    CardSet::Tempest,
    // A Tim that shoots itself as well: the point back is what keeps it from
    // simply winning long games on its own.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target and 1 damage to you.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                // Damage rather than life loss, so it can be prevented and
                // the Fireslinger's controller is a legal damage recipient.
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// TMP 174 — Flowstone Giant
pub(in crate::card::sets) static FLOWSTONE_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46e8240a-d882-4f60-8960-1856284e04a0"),
    "Flowstone Giant",
    CardArt::new("46e8240a-d882-4f60-8960-1856284e04a0", "Joel Biske"),
    CardSet::Tempest,
    // Trades toughness for power one point at a time, so it can always
    // kill exactly what it needs to and then die to anything.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 3, 3).with_ability(
        AbilityDef::activated(
            "{R}: This creature gets +2/-2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 175 — Flowstone Salamander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_SALAMANDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf5b6749-42a6-498b-8908-b28d1749dea6"),
    "Flowstone Salamander",
    crate::card::CardArt::new("bf5b6749-42a6-498b-8908-b28d1749dea6", "Daniel Gelon"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 176 — Flowstone Wyvern
pub(in crate::card::sets) static FLOWSTONE_WYVERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee7949c7-ab80-46a1-9cf7-d8e8c004df6e"),
    "Flowstone Wyvern",
    CardArt::new("ee7949c7-ab80-46a1-9cf7-d8e8c004df6e", "Stephen Daniele"),
    CardSet::Tempest,
    // Toughness for power on a flier, which turns a 3/3 into a lethal 7/1
    // the turn it matters.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +2/-2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 177 — Furnace of Rath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FURNACE_OF_RATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("606abea6-109a-4dd5-99cf-0d5ce492d7f0"),
    "Furnace of Rath",
    crate::card::CardArt::new("606abea6-109a-4dd5-99cf-0d5ce492d7f0", "John Matson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 178 — Giant Strength (reprint)

// TMP 179 — Goblin Bombardment
pub(in crate::card::sets) static GOBLIN_BOMBARDMENT: CardRecord = CardRecord::new_with_legacy_id(
    2110,
    "Goblin Bombardment",
    CardArt::new("179e954f-1d90-4ef4-b800-25845cc338e2", "Brian Snoddy"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a creature: This enchantment deals 1 damage to any target.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
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

// TMP 180 — Hand to Hand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAND_TO_HAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af4cb86a-db01-4d9a-99e9-bb50ce23507f"),
    "Hand to Hand",
    crate::card::CardArt::new("af4cb86a-db01-4d9a-99e9-bb50ce23507f", "Carl Frank"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 181 — Havoc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAVOC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7b032a1-6e43-4e22-9efa-43cfbf211e1c"),
    "Havoc",
    crate::card::CardArt::new("d7b032a1-6e43-4e22-9efa-43cfbf211e1c", "Donato Giancola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 182 — Heart Sliver
pub(in crate::card::sets) static HEART_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27a83ab6-0d15-49e4-90e3-b3a2a095c632"),
    "Heart Sliver",
    CardArt::new("27a83ab6-0d15-49e4-90e3-b3a2a095c632", "Ron Spencer"),
    CardSet::Tempest,
    // Haste for the tribe, so every Sliver drawn later attacks the turn it
    // arrives.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have haste.",
            EffectDef::StaticApply {
                // "All", not "you control": it hands the keyword to the
                // opponent's as readily, which is the drawback these were
                // priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::haste() }),
            },
        ),
    ),
);

// TMP 183 — Jackal Pup
pub(in crate::card::sets) static JACKAL_PUP: CardRecord = CardRecord::new_with_legacy_id(
    267,
    "Jackal Pup",
    CardArt::new("3707ab74-9aec-4d30-86e0-ffa5f72d5b4f", "Susan Van Camp"),
    CardSet::Tempest,
    CardRules::new_creature(mana_cost!("{R}"), &["Jackal"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, it deals that much damage to you.",
            TriggerEventDef::damage_to_source(),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// TMP 184 — Kindle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINDLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("930745eb-b038-4b55-97f3-bf8d99b54d32"),
    "Kindle",
    crate::card::CardArt::new("930745eb-b038-4b55-97f3-bf8d99b54d32", "Donato Giancola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 185 — Lightning Blast
pub(in crate::card::sets) static LIGHTNING_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63fec3f9-d399-48e6-84b6-c8410c24c382"),
    "Lightning Blast",
    CardArt::new("63fec3f9-d399-48e6-84b6-c8410c24c382", "Richard Thomas"),
    CardSet::Tempest,
    // Four damage at instant speed for four mana: the unexciting rate that
    // every red set reprints in some form.
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Lightning Blast deals 4 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )]),
);

// TMP 186 — Lightning Elemental (reprint)

// TMP 187 — Lowland Giant
pub(in crate::card::sets) static LOWLAND_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7398dec7-5e60-43c0-81a0-ab49beb37077"),
    "Lowland Giant",
    CardArt::new("7398dec7-5e60-43c0-81a0-ab49beb37077", "Paolo Parente"),
    CardSet::Tempest,
    // A vanilla 4/3 for four, which is the rate red paid for raw size.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 4, 3),
);

// TMP 188 — Magmasaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMASAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48115601-fa00-4d33-8205-faac02997bb4"),
    "Magmasaur",
    crate::card::CardArt::new("48115601-fa00-4d33-8205-faac02997bb4", "Daniel Gelon"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 189 — Mogg Conscripts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_CONSCRIPTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73719325-b091-4464-b0b0-77dfbb19562a"),
    "Mogg Conscripts",
    crate::card::CardArt::new("73719325-b091-4464-b0b0-77dfbb19562a", "Pete Venters"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 190 — Mogg Fanatic
pub(in crate::card::sets) static MOGG_FANATIC: CardRecord = CardRecord::new_with_legacy_id(
    268,
    "Mogg Fanatic",
    CardArt::new("ca2ecfd4-c874-4468-8601-87aa110d5a00", "Brom"),
    CardSet::Tempest,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: It deals 1 damage to any target.",
            &[CostDef::SacrificeSource],
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

// TMP 191 — Mogg Raider
pub(in crate::card::sets) static MOGG_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94e9cc0a-c210-4525-8c7f-9c6306cc21b0"),
    "Mogg Raider",
    CardArt::new("94e9cc0a-c210-4525-8c7f-9c6306cc21b0", "Brian Snõddy"),
    CardSet::Tempest,
    // A free sacrifice outlet stapled to a combat trick, and the reason
    // every Goblin deck could turn a chump block into a kill.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a Goblin: Target creature gets +1/+1 until end of turn.",
            // "A Goblin", so it can eat itself, which is what makes it a
            // free sacrifice outlet as well as a combat trick.
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Goblin"),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 192 — Mogg Squad
pub(in crate::card::sets) static MOGG_SQUAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b267071-42a6-4e25-9c92-5bca32f8d9af"),
    "Mogg Squad",
    CardArt::new("4b267071-42a6-4e25-9c92-5bca32f8d9af", "Joel Biske"),
    CardSet::Tempest,
    // Three power that shrinks as the board fills, so it is a bear on turn
    // two and a blank on turn six.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature gets -1/-1 for each other creature on the battlefield.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(OTHER_CREATURES, OTHER_CREATURES),
            },
        ),
    ),
);

// TMP 193 — No Quarter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NO_QUARTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7c317ee7-ed92-4e3e-92bb-502099caccf8"),
    "No Quarter",
    crate::card::CardArt::new("7c317ee7-ed92-4e3e-92bb-502099caccf8", "Doug Chaffee"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 194 — Opportunist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPPORTUNIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae0026c0-8f61-4485-8909-6a44c2ca9169"),
    "Opportunist",
    crate::card::CardArt::new("ae0026c0-8f61-4485-8909-6a44c2ca9169", "Dan Frazier"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 195 — Pallimud
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PALLIMUD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61adc314-cfb2-4fdd-925c-cc1dc4692992"),
    "Pallimud",
    crate::card::CardArt::new("61adc314-cfb2-4fdd-925c-cc1dc4692992", "Quinton Hoover"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 196 — Rathi Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7df61bff-a459-4ddb-a084-f47859a43795"),
    "Rathi Dragon",
    crate::card::CardArt::new("7df61bff-a459-4ddb-a084-f47859a43795", "Christopher Rush"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 197 — Renegade Warlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENEGADE_WARLORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a69ea676-60ba-4807-bc9b-976bf5666485"),
    "Renegade Warlord",
    crate::card::CardArt::new("a69ea676-60ba-4807-bc9b-976bf5666485", "Ron Spencer"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 198 — Rolling Thunder
pub(in crate::card::sets) static ROLLING_THUNDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bb07402-d526-4938-89a3-9174d5b5a4de"),
    "Rolling Thunder",
    CardArt::new("0bb07402-d526-4938-89a3-9174d5b5a4de", "Richard Thomas"),
    CardSet::Tempest,
    // The division is the card: X spread across a board is a sweeper, and
    // all of it at one face is the last few points of a race.
    CardRules::new_sorcery(mana_cost!("{X}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Rolling Thunder deals X damage divided as you choose among any number of targets.",
        &[AbilityTargetDef {
            predicate: AbilityTargetPredicate::AnyTarget,
            minimum: 1,
            // Each target has to be dealt at least one, so X caps how many
            // there can be as well as how much they share.
            maximum: AbilityTargetDef::UNLIMITED,
            exact_count: None,
            divided_total: Some(DividedTotal::ChosenX),
            another: false,
            excludes_source: false,
            chooser: TargetChooserDef::Controller,
        }],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::DividedAmongTargets,
        },
    )),
);

// TMP 199 — Sandstone Warrior
pub(in crate::card::sets) static SANDSTONE_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eaa61413-3c6a-4895-b8e7-2723e273a952"),
    "Sandstone Warrior",
    CardArt::new("eaa61413-3c6a-4895-b8e7-2723e273a952", "Stephen Daniele"),
    CardSet::Tempest,
    // First strike makes each point of power worth more than it looks,
    // because the blocker never gets to answer.
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}"),
        &["Human", "Soldier", "Warrior"],
        1,
        3,
    )
    .with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
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

// TMP 200 — Scorched Earth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHED_EARTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6a97817-d1fd-4ba4-9ced-c2702b081523"),
    "Scorched Earth",
    crate::card::CardArt::new("e6a97817-d1fd-4ba4-9ced-c2702b081523", "Nicola Leonard"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 201 — Searing Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARING_TOUCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9091667-d5a8-4978-9023-032ff65f9642"),
    "Searing Touch",
    crate::card::CardArt::new(
        "e9091667-d5a8-4978-9023-032ff65f9642",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 202 — Shadowstorm
pub(in crate::card::sets) static SHADOWSTORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("367c4ad6-973d-47ba-9431-312f9f2996f6"),
    "Shadowstorm",
    CardArt::new("367c4ad6-973d-47ba-9431-312f9f2996f6", "Adam Rex"),
    CardSet::Tempest,
    // A sideboard card in spell form: worthless against anything that is
    // not the one deck it answers.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell(
        "Shadowstorm deals 2 damage to each creature with shadow.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Shadow),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(2),
        },
    )),
);

// TMP 203 — Shatter (reprint)

// TMP 204 — Shocker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOCKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90954848-af7e-47b3-82e7-9fedde6ad606"),
    "Shocker",
    crate::card::CardArt::new("90954848-af7e-47b3-82e7-9fedde6ad606", "Thomas M. Baxa"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 205 — Starke of Rath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARKE_OF_RATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de398b32-10a3-45aa-9886-76806e1602c6"),
    "Starke of Rath",
    crate::card::CardArt::new("de398b32-10a3-45aa-9886-76806e1602c6", "Dan Frazier"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 206 — Stone Rain (reprint)

// TMP 207 — Stun
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STUN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c09c0da6-37a7-42ba-b264-18898ee372f0"),
    "Stun",
    crate::card::CardArt::new("c09c0da6-37a7-42ba-b264-18898ee372f0", "Terese Nielsen"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 208 — Sudden Impact
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUDDEN_IMPACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d7fd516-4b0d-407f-b0c2-d656ad160b8d"),
    "Sudden Impact",
    crate::card::CardArt::new("1d7fd516-4b0d-407f-b0c2-d656ad160b8d", "Alan Pollack"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 209 — Tahngarth's Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAHNGARTH_S_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36658368-88c8-4c55-8147-f6e581f6af36"),
    "Tahngarth's Rage",
    crate::card::CardArt::new("36658368-88c8-4c55-8147-f6e581f6af36", "Hannibal King"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 210 — Tooth and Claw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOOTH_AND_CLAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71696093-0867-4889-8fb4-56fa143f9b27"),
    "Tooth and Claw",
    crate::card::CardArt::new("71696093-0867-4889-8fb4-56fa143f9b27", "Val Mayerik"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 211 — Wall of Diffusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_DIFFUSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e6a469e-3d67-4edf-a735-03dcd626f858"),
    "Wall of Diffusion",
    crate::card::CardArt::new("2e6a469e-3d67-4edf-a735-03dcd626f858", "DiTerlizzi"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 212 — Wild Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e5a1b213-cba0-4eca-b058-93f4fde717c8"),
    "Wild Wurm",
    crate::card::CardArt::new("e5a1b213-cba0-4eca-b058-93f4fde717c8", "Randy Elliott"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 213 — Aluren
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALUREN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("268403bc-733d-446e-a7c1-abc957c42bc2"),
    "Aluren",
    crate::card::CardArt::new("268403bc-733d-446e-a7c1-abc957c42bc2", "April Lee"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 214 — Apes of Rath
pub(in crate::card::sets) static APES_OF_RATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25eff287-6b53-4e6d-9da2-d80d05bb8c51"),
    "Apes of Rath",
    CardArt::new("25eff287-6b53-4e6d-9da2-d80d05bb8c51", "Jeff Laubenstein"),
    CardSet::Tempest,
    // Five power for four mana, paid for by only attacking every second
    // turn -- and by never blocking on the turns in between.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Ape"], 5, 4).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, it doesn't untap during its controller's \
             next untap step.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // The untap step comes before upkeep, so an effect that
                // runs to the next upkeep is still live while that untap
                // step happens and gone immediately after it.
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                duration: ResolvedEffectDurationDef::UntilYourNextUpkeep,
            },
        ),
    ),
);

// TMP 215 — Bayou Dragonfly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAYOU_DRAGONFLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93cfcca5-070b-4946-b17b-0c94b1e47fcd"),
    "Bayou Dragonfly",
    crate::card::CardArt::new("93cfcca5-070b-4946-b17b-0c94b1e47fcd", "DiTerlizzi"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 216 — Broken Fall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROKEN_FALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d43f5349-a3f8-492d-a835-24a9f948741c"),
    "Broken Fall",
    crate::card::CardArt::new("d43f5349-a3f8-492d-a835-24a9f948741c", "Zina Saunders"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 217 — Canopy Spider
pub(in crate::card::sets) static CANOPY_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afc114b0-2e95-4143-a4b6-6537813946e7"),
    "Canopy Spider",
    CardArt::new("afc114b0-2e95-4143-a4b6-6537813946e7", "Christopher Rush"),
    CardSet::Tempest,
    // The cheapest way green answers a two-mana flier, and the reason every
    // limited deck wanted one.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider"], 1, 3)
        .with_ability(abilities::reach()),
);

// TMP 218 — Charging Rhino
pub(in crate::card::sets) static CHARGING_RHINO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49e47248-051c-4ee6-aad2-352ebd1f38ca"),
    "Charging Rhino",
    CardArt::new("651f89e5-9ce2-4713-aca9-6581005f6ca2", "Daren Bader"),
    CardSet::Tempest,
    // The same clause on a body big enough that no single blocker is a
    // fair fight.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Rhino"], 4, 4).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked by more than one creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::MaximumBlockers(1),
                )),
            },
        ),
    ),
);

// TMP 219 — Choke
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHOKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2f85205-3c4f-4411-b09c-d1271be56dde"),
    "Choke",
    crate::card::CardArt::new("e2f85205-3c4f-4411-b09c-d1271be56dde", "Terese Nielsen"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 220 — Crazed Armodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRAZED_ARMODON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b83e4b36-57c1-493d-ab79-52075990b2d5"),
    "Crazed Armodon",
    crate::card::CardArt::new("b83e4b36-57c1-493d-ab79-52075990b2d5", "Gary Leach"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 221 — Dirtcowl Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIRTCOWL_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59f9b7f9-1f6c-416f-ba73-62924bca1856"),
    "Dirtcowl Wurm",
    crate::card::CardArt::new("a9e2df7d-5d72-4a32-a453-6d8611f0d63c", "Dan Frazier"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 222 — Earthcraft
pub(in crate::card::sets) static EARTHCRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9dda7531-82a1-4f49-8858-601ddbc6e2bc"),
    "Earthcraft",
    CardArt::new("9dda7531-82a1-4f49-8858-601ddbc6e2bc", "Randy Gallegos"),
    CardSet::Tempest,
    // Untapping a land with a creature is the whole card: every creature on
    // the board becomes a land that has already been played.
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(
        AbilityDef::activated_with_targets(
            "Tap an untapped creature you control: Untap target basic land.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 1,
            }],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                )]
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// TMP 223 — Eladamri's Vineyard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELADAMRI_S_VINEYARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8531643-5657-44b6-89d1-9cdf67ed09c4"),
    "Eladamri's Vineyard",
    crate::card::CardArt::new("d8531643-5657-44b6-89d1-9cdf67ed09c4", "Ron Chironna"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 224 — Eladamri, Lord of Leaves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELADAMRI_LORD_OF_LEAVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b1689f3-9dfa-4525-90b3-7af15f7eb720"),
    "Eladamri, Lord of Leaves",
    crate::card::CardArt::new("0b1689f3-9dfa-4525-90b3-7af15f7eb720", "Ron Chironna"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 225 — Elven Warhounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVEN_WARHOUNDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29138c1e-11cb-488f-8e04-f5488e08a81e"),
    "Elven Warhounds",
    crate::card::CardArt::new("29138c1e-11cb-488f-8e04-f5488e08a81e", "Kev Walker"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 226 — Elvish Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_FURY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f99c10b5-b93b-40c3-936c-d1b81b49c5a4"),
    "Elvish Fury",
    crate::card::CardArt::new("f99c10b5-b93b-40c3-936c-d1b81b49c5a4", "Quinton Hoover"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 227 — Flailing Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAILING_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43d246ac-1ca5-4c55-856c-4a83a4d638ab"),
    "Flailing Drake",
    crate::card::CardArt::new("43d246ac-1ca5-4c55-856c-4a83a4d638ab", "Heather Hudson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 228 — Frog Tongue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROG_TONGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3941e799-a254-423e-90bb-091dbe56ca6a"),
    "Frog Tongue",
    crate::card::CardArt::new("3941e799-a254-423e-90bb-091dbe56ca6a", "Phil Foglio"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 229 — Fugitive Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUGITIVE_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afe165cd-8ef7-408e-ae56-3c6a0cc4e409"),
    "Fugitive Druid",
    crate::card::CardArt::new("afe165cd-8ef7-408e-ae56-3c6a0cc4e409", "Quinton Hoover"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 230 — Harrow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARROW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c207142-4880-4935-9827-b91bc7d9d643"),
    "Harrow",
    crate::card::CardArt::new(
        "3c207142-4880-4935-9827-b91bc7d9d643",
        "Eric David Anderson",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 231 — Heartwood Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTWOOD_DRYAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2b9a001-2a1e-4fc4-9b84-c776f741a858"),
    "Heartwood Dryad",
    crate::card::CardArt::new("e2b9a001-2a1e-4fc4-9b84-c776f741a858", "Rebecca Guay"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 232 — Heartwood Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTWOOD_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4baacffe-76d1-4cfb-a047-d6d126bb8de0"),
    "Heartwood Giant",
    crate::card::CardArt::new("4baacffe-76d1-4cfb-a047-d6d126bb8de0", "Randy Elliott"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 233 — Heartwood Treefolk
pub(in crate::card::sets) static HEARTWOOD_TREEFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de263f02-8e3e-4785-9c06-9adc168994f3"),
    "Heartwood Treefolk",
    CardArt::new("de263f02-8e3e-4785-9c06-9adc168994f3", "Daren Bader"),
    CardSet::Tempest,
    // A 3/4 that green cannot block, which in a format of mirror matches is
    // most of what it does.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Treefolk"], 3, 4)
        .with_ability(abilities::landwalk(BasicLandType::Forest)),
);

// TMP 234 — Horned Sliver
pub(in crate::card::sets) static HORNED_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0175cec-e64c-45c6-9208-76127e76a7cf"),
    "Horned Sliver",
    CardArt::new("d0175cec-e64c-45c6-9208-76127e76a7cf", "Allen Williams"),
    CardSet::Tempest,
    // Trample, which matters exactly when the opponent has Slivers of their
    // own to chump with.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have trample.",
            EffectDef::StaticApply {
                // "All", not "you control": it hands the keyword to the
                // opponent's as readily, which is the drawback these were
                // priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::trample() }),
            },
        ),
    ),
);

// TMP 235 — Krakilin
pub(in crate::card::sets) static KRAKILIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a90442e8-9d22-4767-9e08-bd314169ea70"),
    "Krakilin",
    CardArt::new(
        "a90442e8-9d22-4767-9e08-bd314169ea70",
        "Richard Kane Ferguson",
    ),
    CardSet::Tempest,
    // The same deal as any X creature with a regeneration shield stapled on,
    // which is what makes the mana spent on it hard to answer.
    CardRules::new_creature(mana_cost!("{X}{G}{G}"), &["Elemental"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ]),
);

// TMP 236 — Mirri's Guile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRI_S_GUILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73d51a3c-95c0-4810-b847-4b8afd12fd64"),
    "Mirri's Guile",
    crate::card::CardArt::new("73d51a3c-95c0-4810-b847-4b8afd12fd64", "Brom"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 237 — Mongrel Pack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONGREL_PACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56b84315-5287-4401-a4c8-34c192423270"),
    "Mongrel Pack",
    crate::card::CardArt::new("56b84315-5287-4401-a4c8-34c192423270", "Jeff Miracola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 238 — Muscle Sliver
pub(in crate::card::sets) static MUSCLE_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("602a1e1f-4195-48c0-8290-562e7e0db6d8"),
    "Muscle Sliver",
    CardArt::new(
        "602a1e1f-4195-48c0-8290-562e7e0db6d8",
        "Richard Kane Ferguson",
    ),
    CardSet::Tempest,
    // The card that made Slivers a deck: every one you play makes every one
    // you already played bigger.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures get +1/+1.",
            EffectDef::StaticApply {
                // "All Sliver creatures", not "Sliver creatures you
                // control": the older Slivers pump the opponent's as
                // well, which is the drawback the cycle was priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// TMP 239 — Natural Spring
pub(in crate::card::sets) static NATURAL_SPRING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ddfc1cc-5c13-443c-a0ae-0bcc931923e7"),
    "Natural Spring",
    CardArt::new("1ff5d12a-8634-468b-86ca-4ba0f7c013ca", "Susan Van Camp"),
    CardSet::Tempest,
    // Eight life for five mana at sorcery speed. It does nothing to the
    // board, which is why it was a limited card.
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains 8 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(8),
        },
    )),
);

// TMP 240 — Nature's Revolt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_REVOLT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a70386c5-053a-46c4-b26d-c6f92f536bed"),
    "Nature's Revolt",
    crate::card::CardArt::new("a70386c5-053a-46c4-b26d-c6f92f536bed", "Donato Giancola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 241 — Needle Storm
pub(in crate::card::sets) static NEEDLE_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29a44e44-94b1-4bd2-8e00-6bd2ec07ee4c"),
    "Needle Storm",
    CardArt::new("be80dd2d-f595-4d80-84ae-66d3d18e7399", "Val Mayerik"),
    CardSet::Tempest,
    // Green's answer to fliers, indiscriminate enough to catch its own
    // controller's.
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Needle Storm deals 4 damage to each creature with flying.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(4),
        },
    )]),
);

// TMP 242 — Nurturing Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NURTURING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bf53069-44ac-49c5-83bf-9c3c1274e407"),
    "Nurturing Licid",
    crate::card::CardArt::new("0bf53069-44ac-49c5-83bf-9c3c1274e407", "Mark Poole"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 243 — Overrun (reprint)

// TMP 244 — Pincher Beetles
pub(in crate::card::sets) static PINCHER_BEETLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dba68902-1e05-414a-8c3d-1f97da61d09d"),
    "Pincher Beetles",
    CardArt::new("dba68902-1e05-414a-8c3d-1f97da61d09d", "Stephen Daniele"),
    CardSet::Tempest,
    // A 3/1 nothing can target, which in a removal-heavy format is worth
    // more than the fragile body suggests.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect"], 3, 1)
        .with_ability(abilities::shroud()),
);

// TMP 245 — Rampant Growth (reprint)

// TMP 246 — Reality Anchor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REALITY_ANCHOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21204f62-c253-4d88-a4cd-7c0f6f0513e0"),
    "Reality Anchor",
    crate::card::CardArt::new("21204f62-c253-4d88-a4cd-7c0f6f0513e0", "Randy Gallegos"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 247 — Reap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("229f8a3d-d1a5-46d7-9b1b-e165397e6579"),
    "Reap",
    crate::card::CardArt::new("229f8a3d-d1a5-46d7-9b1b-e165397e6579", "Ron Chironna"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 248 — Recycle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECYCLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae984b86-ac6d-45e2-9c8d-0b7ac50021a1"),
    "Recycle",
    crate::card::CardArt::new("ae984b86-ac6d-45e2-9c8d-0b7ac50021a1", "Phil Foglio"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 249 — Respite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESPITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("228a8d29-cc14-49c7-ae24-5847344583ed"),
    "Respite",
    crate::card::CardArt::new("228a8d29-cc14-49c7-ae24-5847344583ed", "Rebecca Guay"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 250 — Root Maze
pub(in crate::card::sets) static ROOT_MAZE: CardRecord = CardRecord::new_with_legacy_id(
    287,
    "Root Maze",
    CardArt::new("99a12b74-f191-4362-81ab-77590ae5e68f", "Rebecca Guay"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::replacement_for(
        "Artifacts and lands enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            controller: PlayerRelation::Any,
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )),
);

// TMP 251 — Rootbreaker Wurm
pub(in crate::card::sets) static ROOTBREAKER_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a686ed6-fc13-4882-b56c-667f556d9804"),
    "Rootbreaker Wurm",
    CardArt::new(
        "9a686ed6-fc13-4882-b56c-667f556d9804",
        "Richard Kane Ferguson",
    ),
    CardSet::Tempest,
    // Seven mana for a 6/6 trampler, the plain top end of green's curve.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Wurm"], 6, 6)
        .with_ability(abilities::trample()),
);

// TMP 252 — Rootwalla (reprint)

// TMP 253 — Scragnoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAGNOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d80f7fa7-e7c4-4fc4-99bf-8a8502965fc8"),
    "Scragnoth",
    crate::card::CardArt::new("d80f7fa7-e7c4-4fc4-99bf-8a8502965fc8", "Jeff Laubenstein"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 254 — Seeker of Skybreak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEKER_OF_SKYBREAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4bb98ba-d599-4597-911c-2b472fa8817c"),
    "Seeker of Skybreak",
    crate::card::CardArt::new("b4bb98ba-d599-4597-911c-2b472fa8817c", "Daren Bader"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 255 — Skyshroud Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26877a52-dec3-433d-b7a5-767f6cdf2365"),
    "Skyshroud Elf",
    crate::card::CardArt::new("26877a52-dec3-433d-b7a5-767f6cdf2365", "Jeff Miracola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 256 — Skyshroud Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_RANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efe01296-2b8b-4cdf-a041-a08bebea9c29"),
    "Skyshroud Ranger",
    crate::card::CardArt::new("efe01296-2b8b-4cdf-a041-a08bebea9c29", "Steve Luke"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 257 — Skyshroud Troll
pub(in crate::card::sets) static SKYSHROUD_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("925c488d-79db-47d1-b7be-851f31732026"),
    "Skyshroud Troll",
    CardArt::new("925c488d-79db-47d1-b7be-851f31732026", "Matthew D. Wilson"),
    CardSet::Tempest,
    // The Troll clause at green's usual rate: nothing kills it but
    // exile, an edict, or a swing wide enough not to care.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Troll", "Giant"], 3, 3).with_ability(
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ),
);

// TMP 258 — Spike Drone
pub(in crate::card::sets) static SPIKE_DRONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d45a3d3-a114-496e-b575-504179a297cc"),
    "Spike Drone",
    CardArt::new("5d45a3d3-a114-496e-b575-504179a297cc", "Charles Gillespie"),
    CardSet::Tempest,
    // One mana for a counter that can move once, which is a 1/1 that turns
    // into a permanent upgrade on something worth keeping.
    CardRules::new_creature(mana_cost!("{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}, Remove a +1/+1 counter from this creature: Put a +1/+1 counter on target creature.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            &const {
                [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))]
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TMP 259 — Storm Front
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_FRONT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("994bb02d-6fef-454b-b1b1-d3d1af8dcd1a"),
    "Storm Front",
    crate::card::CardArt::new("994bb02d-6fef-454b-b1b1-d3d1af8dcd1a", "William O'Connor"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 260 — Trained Armodon
pub(in crate::card::sets) static TRAINED_ARMODON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2380ab8f-58d2-4e1c-a115-cd2615b5a871"),
    "Trained Armodon",
    CardArt::new("2380ab8f-58d2-4e1c-a115-cd2615b5a871", "Gary Leach"),
    CardSet::Tempest,
    // The vanilla 3/3 for three green kept reprinting, and the yardstick
    // every other three-drop was measured against.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elephant"], 3, 3),
);

// TMP 261 — Tranquility (reprint)

// TMP 262 — Trumpeting Armodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRUMPETING_ARMODON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38f94fd1-6f85-41ad-9674-f05cc893324f"),
    "Trumpeting Armodon",
    crate::card::CardArt::new("38f94fd1-6f85-41ad-9674-f05cc893324f", "Gary Leach"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 263 — Verdant Force
pub(in crate::card::sets) static VERDANT_FORCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29bd094c-fcc1-4abf-ba3e-03a5b9b6d1c2"),
    "Verdant Force",
    CardArt::new("29bd094c-fcc1-4abf-ba3e-03a5b9b6d1c2", "DiTerlizzi"),
    CardSet::Tempest,
    // Two bodies a turn round rather than one, because it triggers on the
    // opponent's upkeep as well -- which is what eight mana buys.
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Elemental"], 7, 7).with_ability(
        AbilityDef::triggered(
            "At the beginning of each upkeep, create a 1/1 green Saproling creature token.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
    ),
);

// TMP 264 — Verdigris
pub(in crate::card::sets) static VERDIGRIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c79664d-3461-44e7-afe6-33ec54e312ad"),
    "Verdigris",
    CardArt::new("0c79664d-3461-44e7-afe6-33ec54e312ad", "Zina Saunders"),
    CardSet::Tempest,
    // Three mana at instant speed for an artifact, which is what green paid
    // before it got cheaper answers.
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// TMP 265 — Winter's Grasp
pub(in crate::card::sets) static WINTER_S_GRASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2215de4-da49-4270-aec7-5e16a938bae4"),
    "Winter's Grasp",
    CardArt::new("7af28a5d-45dc-4e31-9009-5c0bd25a9032", "Tom Wänerstrand"),
    CardSet::Tempest,
    // The green printing of the same effect, from the block where every
    // colour got one.
    CardRules::new_sorcery(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// TMP 266 — Dracoplasm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRACOPLASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3560556c-f1d3-4d69-afe7-c2a5fa2a5c3d"),
    "Dracoplasm",
    crate::card::CardArt::new("3560556c-f1d3-4d69-afe7-c2a5fa2a5c3d", "Andrew Robinson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 267 — Lobotomy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOBOTOMY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee7ba92d-d327-4b1c-be40-708c5abb27df"),
    "Lobotomy",
    crate::card::CardArt::new("ee7ba92d-d327-4b1c-be40-708c5abb27df", "Thomas M. Baxa"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 268 — Ranger en-Vec
pub(in crate::card::sets) static RANGER_EN_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a89e82c-7206-4d74-95c6-ad3627e5a9ce"),
    "Ranger en-Vec",
    CardArt::new("4a89e82c-7206-4d74-95c6-ad3627e5a9ce", "Randy Elliott"),
    CardSet::Tempest,
    // First strike and regeneration together mean it wins every combat and
    // survives it, twice over.
    CardRules::new_creature(
        mana_cost!("{1}{G}{W}"),
        &["Human", "Soldier", "Archer", "Ranger"],
        2,
        2,
    )
    .with_abilities(&[
        abilities::first_strike(),
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{G}"))],
        ),
    ]),
);

// TMP 269 — Segmented Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEGMENTED_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("18be7cfa-bf75-407d-b79f-2fec4b1aacf5"),
    "Segmented Wurm",
    crate::card::CardArt::new("18be7cfa-bf75-407d-b79f-2fec4b1aacf5", "Jeff Miracola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 270 — Selenia, Dark Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SELENIA_DARK_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c1624f7-8275-46d3-ab7e-7b162e27593f"),
    "Selenia, Dark Angel",
    crate::card::CardArt::new("9c1624f7-8275-46d3-ab7e-7b162e27593f", "Matthew D. Wilson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 271 — Sky Spirit
pub(in crate::card::sets) static SKY_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb8efbec-e8bf-4e34-bf13-b43916d2e9ff"),
    "Sky Spirit",
    CardArt::new("eb8efbec-e8bf-4e34-bf13-b43916d2e9ff", "Rebecca Guay"),
    CardSet::Tempest,
    // Flying and first strike on the same 2/2: it beats every other flier
    // its size in the air and takes nothing back.
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Spirit"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// TMP 272 — Soltari Guerrillas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLTARI_GUERRILLAS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b683571-6ac5-4d65-99a6-981755ed4764"),
    "Soltari Guerrillas",
    crate::card::CardArt::new("1b683571-6ac5-4d65-99a6-981755ed4764", "Val Mayerik"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 273 — Spontaneous Combustion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPONTANEOUS_COMBUSTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34e6c04f-9d1a-497b-bc96-a0e48a1c1904"),
    "Spontaneous Combustion",
    crate::card::CardArt::new("34e6c04f-9d1a-497b-bc96-a0e48a1c1904", "Doug Chaffee"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 274 — Vhati il-Dal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VHATI_IL_DAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be535a4a-c00d-4c58-a663-a3419a54da51"),
    "Vhati il-Dal",
    crate::card::CardArt::new("be535a4a-c00d-4c58-a663-a3419a54da51", "Ron Spencer"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 275 — Wood Sage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOOD_SAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b073ac1-be1b-49c0-98b4-bf8165e2f872"),
    "Wood Sage",
    crate::card::CardArt::new("4b073ac1-be1b-49c0-98b4-bf8165e2f872", "Paolo Parente"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 276 — Altar of Dementia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALTAR_OF_DEMENTIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f2da99f-3c53-4980-97d6-2158c765aac0"),
    "Altar of Dementia",
    crate::card::CardArt::new("4f2da99f-3c53-4980-97d6-2158c765aac0", "Brom"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 277 — Booby Trap
pub(in crate::card::sets) static BOOBY_TRAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bfedc78e-47dc-43e3-aed7-2d5c8e97fdac"),
    "Booby Trap",
    crate::card::CardArt::new("bfedc78e-47dc-43e3-aed7-2d5c8e97fdac", "Doug Chaffee"),
    crate::card::CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this artifact enters, choose an opponent and a card name other than a basic land card name.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Player(PlayerRelation::Opponent)),
                ReplacementEffectDef::BindOutput {
                    binding: Binding!("booby_trap_name"),
                    effect: &abilities::choose_card_name_as_enters(
                        CardNameSetDef::CardNamesOtherThanBasicLands,
                    ),
                },
            ]),
        ),
        AbilityDef::static_ability(
            "The chosen player reveals each card they draw.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                    PlayerRelation::ChosenPlayer,
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::RevealsDrawnCards),
            },
        ),
        AbilityDef::triggered_if(
            "When the chosen player draws a card with the chosen name, sacrifice this artifact. If you do, it deals 10 damage to that player.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::matching(
                PlayerRelation::ChosenPlayer,
                ObjectPredicateDef::NameEquals(CardNameDef::Binding(Binding!(
                    "booby_trap_name"
                ))),
            )),
            &TriggerConditionDef::SourceOnBattlefield,
            EffectDef::Sequence(&[
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(10),
                },
            ]),
        ),
    ]),
);

// TMP 278 — Bottle Gnomes
pub(in crate::card::sets) static BOTTLE_GNOMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("645297d1-ee77-4879-83eb-8114fbabb9a4"),
    "Bottle Gnomes",
    CardArt::new("645297d1-ee77-4879-83eb-8114fbabb9a4", "Kaja Foglio"),
    CardSet::Tempest,
    // Three life whenever it is convenient, on a body that also blocks
    // once for free.
    CardRules::new_creature(mana_cost!("{3}"), &["Gnome"], 1, 3).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: You gain 3 life.",
            &[CostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// TMP 279 — Coiled Tinviper
pub(in crate::card::sets) static COILED_TINVIPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("426a28bd-033d-41af-b577-ece73cbd7b3a"),
    "Coiled Tinviper",
    CardArt::new("426a28bd-033d-41af-b577-ece73cbd7b3a", "John Matson"),
    CardSet::Tempest,
    // A colourless first striker, which is what an artifact body was for:
    // any deck could play it.
    CardRules::new_creature(mana_cost!("{3}"), &["Snake"], 2, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// TMP 280 — Cold Storage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLD_STORAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b26e28d4-50e1-41db-984d-c55781295012"),
    "Cold Storage",
    crate::card::CardArt::new("b26e28d4-50e1-41db-984d-c55781295012", "Greg Simanson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 281 — Cursed Scroll
pub(in crate::card::sets) static CURSED_SCROLL: CardRecord = CardRecord::new_with_legacy_id(
    2037,
    "Cursed Scroll",
    CardArt::new(
        "31415b9b-fb30-4132-a9a3-795b4573a901",
        "D. Alexander Gregory",
    ),
    CardSet::Tempest,
    // A one-card hand makes the random reveal deterministic, which is why the
    // card belongs in a deck that has spent almost everything.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{3}, {T}: Choose a card name, then reveal a card at random from your hand. If that card has the chosen name, this artifact deals 2 damage to any target.",
        &[
            CostDef::Mana(mana_cost!("{3}")),
            CostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::BindOutput {
                binding: Binding!("cursed_scroll_name"),
                effect: &EffectDef::ChooseCardName {
                    chooser: PlayerRefDef::EffectController,
                    names: CardNameSetDef::AllCardNames,
                },
            },
            EffectDef::BindOutput {
                binding: Binding!("revealed_card"),
                effect: &EffectDef::RevealAtRandomFromHand {
                    player: EffectRecipientDef::Controller,
                },
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Binding(Binding!("revealed_card")),
                    predicate: ObjectSetPredicateDef::contains(
                        &ObjectPredicateDef::NameEquals(CardNameDef::Binding(Binding!(
                            "cursed_scroll_name"
                        ))),
                    ),
                }),
                then: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
            },
        ]),
    )),
);

// TMP 282 — Echo Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECHO_CHAMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06425615-6c10-4766-8128-a1a09a35649d"),
    "Echo Chamber",
    crate::card::CardArt::new("06425615-6c10-4766-8128-a1a09a35649d", "Donato Giancola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 283 — Emerald Medallion
pub(in crate::card::sets) static EMERALD_MEDALLION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67e87f30-b27a-48e2-a133-192309dd5902"),
    "Emerald Medallion",
    crate::card::CardArt::new("67e87f30-b27a-48e2-a133-192309dd5902", "Sue Ellen Brown"),
    crate::card::CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Green spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Green),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 284 — Emmessi Tome
pub(in crate::card::sets) static EMMESSI_TOME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a870e48a-41ae-4d9f-b181-074deb067d40"),
    "Emmessi Tome",
    CardArt::new("a870e48a-41ae-4d9f-b181-074deb067d40", "Tom Wänerstrand"),
    CardSet::Tempest,
    // Nine mana across two turns for one extra card, which only a deck
    // with nothing else to do with its mana can afford.
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated(
        "{5}, {T}: Draw two cards, then discard a card.",
        &[CostDef::Mana(mana_cost!("{5}")), CostDef::TapSource],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// TMP 285 — Energizer
pub(in crate::card::sets) static ENERGIZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("914f204c-7f3d-41f2-a771-0b6227d539eb"),
    "Energizer",
    CardArt::new("914f204c-7f3d-41f2-a771-0b6227d539eb", "Val Mayerik"),
    CardSet::Tempest,
    // Four mana for a 2/2 that grows a point a turn, which is a rate only a
    // game with nothing else to do can pay.
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Juggernaut"], 2, 2).with_ability(
        AbilityDef::activated(
            "{2}, {T}: Put a +1/+1 counter on this creature.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 286 — Essence Bottle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_BOTTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e3c7760-04d8-424d-a097-df0ca8297837"),
    "Essence Bottle",
    crate::card::CardArt::new("7e3c7760-04d8-424d-a097-df0ca8297837", "Donato Giancola"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 287 — Excavator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCAVATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6dc3d5b4-b04f-4b34-afd2-72fb3de0a33b"),
    "Excavator",
    crate::card::CardArt::new("6dc3d5b4-b04f-4b34-afd2-72fb3de0a33b", "Tom Kyffin"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 288 — Flowstone Sculpture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_SCULPTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f89f599-b063-4568-b7b9-08b96d04bde1"),
    "Flowstone Sculpture",
    crate::card::CardArt::new("7f89f599-b063-4568-b7b9-08b96d04bde1", "Hannibal King"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 289 — Fool's Tome
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOL_S_TOME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("83be257c-8945-46be-8b58-fb2881084026"),
    "Fool's Tome",
    crate::card::CardArt::new("83be257c-8945-46be-8b58-fb2881084026", "Julie Baroh"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 290 — Grindstone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRINDSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4459187-de64-456f-bb66-56dea40d5c3e"),
    "Grindstone",
    crate::card::CardArt::new("f4459187-de64-456f-bb66-56dea40d5c3e", "Greg Simanson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 291 — Helm of Possession
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELM_OF_POSSESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79e16191-8dca-491b-b892-17696023d581"),
    "Helm of Possession",
    crate::card::CardArt::new("79e16191-8dca-491b-b892-17696023d581", "Janet Aulisio"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 292 — Jet Medallion
pub(in crate::card::sets) static JET_MEDALLION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0db458c-2ced-454c-8061-fff8bd363b33"),
    "Jet Medallion",
    crate::card::CardArt::new("c0db458c-2ced-454c-8061-fff8bd363b33", "Sue Ellen Brown"),
    crate::card::CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Black spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Black),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 293 — Jinxed Idol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JINXED_IDOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c728e38-5656-4feb-8610-0cf45fb38094"),
    "Jinxed Idol",
    crate::card::CardArt::new("0c728e38-5656-4feb-8610-0cf45fb38094", "John Matson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 294 — Lotus Petal
pub(in crate::card::sets) static LOTUS_PETAL: CardRecord = CardRecord::new_with_legacy_id(
    271,
    "Lotus Petal",
    CardArt::new("6c877da3-68fa-41d0-8a24-8c79fcd8ecc1", "April Lee"),
    CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_mana(
        "{T}, Sacrifice this artifact: Add one mana of any color.",
        &[CostDef::TapSource, CostDef::SacrificeSource],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// TMP 295 — Magnetic Web
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNETIC_WEB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f3c7309-4efb-49ce-a9cc-a8f7b04c1a15"),
    "Magnetic Web",
    crate::card::CardArt::new("9f3c7309-4efb-49ce-a9cc-a8f7b04c1a15", "Adam Rex"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 296 — Manakin
pub(in crate::card::sets) static MANAKIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d33ce7f-f318-4161-843a-f5bb6d6e3d29"),
    "Manakin",
    CardArt::new("3d33ce7f-f318-4161-843a-f5bb6d6e3d29", "Scott Kirschner"),
    CardSet::Tempest,
    // Two mana for a body that pays one back every turn, which is what a
    // colourless accelerant has to offer to be worth a card.
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 1, 1)
        .with_ability(abilities::tap_for(ManaColor::Colorless)),
);

// TMP 297 — Metallic Sliver
pub(in crate::card::sets) static METALLIC_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30143f4f-9846-448d-8797-8fe0bc0cc5df"),
    "Metallic Sliver",
    CardArt::new("30143f4f-9846-448d-8797-8fe0bc0cc5df", "Allen Williams"),
    CardSet::Tempest,
    // A 1/1 with nothing on it, printed so that every Sliver lord had one
    // more body to talk to.
    CardRules::new_creature(mana_cost!("{1}"), &["Sliver"], 1, 1),
);

// TMP 298 — Mogg Cannon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_CANNON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ee64a77-5308-45c7-b865-400820968c74"),
    "Mogg Cannon",
    crate::card::CardArt::new("5ee64a77-5308-45c7-b865-400820968c74", "Mike Raabe"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 299 — Patchwork Gnomes
pub(in crate::card::sets) static PATCHWORK_GNOMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdaa9ac4-b742-4a24-a316-97538adfd361"),
    "Patchwork Gnomes",
    CardArt::new("bdaa9ac4-b742-4a24-a316-97538adfd361", "Mike Raabe"),
    CardSet::Tempest,
    // A colorless regenerator, so the discard is the only colour
    // requirement and any deck can hold the ground with it.
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Gnome"], 2, 1).with_ability(
        abilities::regenerate_self(
            "Discard a card: Regenerate this creature.",
            &[CostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
        ),
    ),
);

// TMP 300 — Pearl Medallion
pub(in crate::card::sets) static PEARL_MEDALLION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44588d53-7cce-406a-8e61-cd9866691966"),
    "Pearl Medallion",
    crate::card::CardArt::new("44588d53-7cce-406a-8e61-cd9866691966", "Sue Ellen Brown"),
    crate::card::CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "White spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::White),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 301 — Phyrexian Grimoire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_GRIMOIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fcf5b8a-563b-48d7-a874-e9c226192320"),
    "Phyrexian Grimoire",
    crate::card::CardArt::new("2fcf5b8a-563b-48d7-a874-e9c226192320", "Doug Chaffee"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 302 — Phyrexian Hulk (reprint)

// TMP 303 — Phyrexian Splicer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1e9061a-b9e6-49ec-bc7e-c18557da9fd5"),
    "Phyrexian Splicer",
    crate::card::CardArt::new("b1e9061a-b9e6-49ec-bc7e-c18557da9fd5", "Brom"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 304 — Puppet Strings
pub(in crate::card::sets) static PUPPET_STRINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92b55586-9ea3-403e-96ec-2604f91c79cc"),
    "Puppet Strings",
    CardArt::new("92b55586-9ea3-403e-96ec-2604f91c79cc", "Scott Kirschner"),
    CardSet::Tempest,
    // Both halves matter: it stops an attacker on their turn and untaps a
    // blocker on yours.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: You may tap or untap target creature.",
        &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::ChooseEffect {
                player: EffectRecipientDef::Controller,
                choices: &[
                    EffectChoiceDef {
                        label: "Tap the target creature",
                        effect: EffectDef::Tap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    },
                    EffectChoiceDef {
                        label: "Untap the target creature",
                        effect: EffectDef::Untap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    },
                ],
            },
        },
    )),
);

// TMP 305 — Ruby Medallion
pub(in crate::card::sets) static RUBY_MEDALLION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24cdb28b-85f3-41ae-b1f5-fac766b2dcd2"),
    "Ruby Medallion",
    crate::card::CardArt::new("24cdb28b-85f3-41ae-b1f5-fac766b2dcd2", "Sue Ellen Brown"),
    crate::card::CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Red spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Red),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 306 — Sapphire Medallion
pub(in crate::card::sets) static SAPPHIRE_MEDALLION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ab1e253-47cb-4089-87d5-0f998025d98c"),
    "Sapphire Medallion",
    crate::card::CardArt::new("3ab1e253-47cb-4089-87d5-0f998025d98c", "Sue Ellen Brown"),
    crate::card::CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Blue spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Blue),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 307 — Scalding Tongs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALDING_TONGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34136f2c-3edd-4ca3-b1ef-1fdcaa4518a0"),
    "Scalding Tongs",
    crate::card::CardArt::new("34136f2c-3edd-4ca3-b1ef-1fdcaa4518a0", "Randy Gallegos"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 308 — Scroll Rack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCROLL_RACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f7346d8-1aef-4618-88e6-74bd8865e0f3"),
    "Scroll Rack",
    crate::card::CardArt::new("5f7346d8-1aef-4618-88e6-74bd8865e0f3", "Heather Hudson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 309 — Squee's Toy
pub(in crate::card::sets) static SQUEE_S_TOY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b524ae7-cb24-41af-b41b-3cb3ee8cf3b0"),
    "Squee's Toy",
    CardArt::new("2b524ae7-cb24-41af-b41b-3cb3ee8cf3b0", "Heather Hudson"),
    CardSet::Tempest,
    // One mana for a shield that resets every turn, which is what a deck
    // full of one-toughness blockers actually wants.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{T}: Prevent the next 1 damage that would be dealt to target creature this turn.",
        &[CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::amount(
                DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// TMP 310 — Static Orb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STATIC_ORB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3574d758-9143-4a2b-9ebd-ed8dab238251"),
    "Static Orb",
    crate::card::CardArt::new("3574d758-9143-4a2b-9ebd-ed8dab238251", "Dermot Power"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 311 — Telethopter
pub(in crate::card::sets) static TELETHOPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77d26c29-cd98-446b-b4e1-687561ed6d3f"),
    "Telethopter",
    CardArt::new("77d26c29-cd98-446b-b4e1-687561ed6d3f", "Thomas M. Baxa"),
    CardSet::Tempest,
    // Any spare creature buys it evasion, which turns a board stall into
    // three damage a turn.
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Thopter"], 3, 1).with_ability(
        AbilityDef::activated(
            "Tap an untapped creature you control: This creature gains flying until end of turn.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 1,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 312 — Thumbscrews
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUMBSCREWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71025a6b-3aed-4943-a907-9584d135f6c0"),
    "Thumbscrews",
    crate::card::CardArt::new("71025a6b-3aed-4943-a907-9584d135f6c0", "Charles Gillespie"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 313 — Torture Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORTURE_CHAMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5648158d-38d4-4167-8af5-ee5d7d6fd7cb"),
    "Torture Chamber",
    crate::card::CardArt::new("5648158d-38d4-4167-8af5-ee5d7d6fd7cb", "Thomas Gianni"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 314 — Watchdog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATCHDOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c2ffc07-9993-40de-b36e-33c7afd4cfc2"),
    "Watchdog",
    crate::card::CardArt::new(
        "8c2ffc07-9993-40de-b36e-33c7afd4cfc2",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 315 — Ancient Tomb
pub(in crate::card::sets) static ANCIENT_TOMB: CardRecord = CardRecord::new_with_legacy_id(
    300,
    "Ancient Tomb",
    CardArt::new("30e401e3-282b-4524-87e1-c6cd50cd6d00", "Colin MacNeil"),
    CardSet::Tempest,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add {C}{C}. This land deals 2 damage to you.",
        &[CostDef::TapSource],
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless)
                .with_amount(2)
                .with_damage_to_controller(2),
        ),
    )),
);

// TMP 316 — Caldera Lake
pub(in crate::card::sets) static CALDERA_LAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f01fe22-e8ff-4106-8ac5-693ef920b2c9"),
    "Caldera Lake",
    CardArt::new("7f01fe22-e8ff-4106-8ac5-693ef920b2c9", "Allen Williams"),
    CardSet::Tempest,
    // A painland that also costs a turn, which is what the rate looked
    // like before the Apocalypse cycle fixed it.
    CardRules::new_land(&[]).with_abilities(
        &const {
            let [colorless, colored] = abilities::pain_land(
                "{T}: Add {U} or {R}. This land deals 1 damage to you.",
                &[ManaColor::Blue, ManaColor::Red],
            );
            [abilities::enters_tapped(CardType::Land), colorless, colored]
        },
    ),
);

// TMP 317 — Cinder Marsh
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static CINDER_MARSH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9fee067d-c31f-4b09-99f5-84d1102f96b0"),
    "Cinder Marsh",
    crate::card::CardArt::new("9fee067d-c31f-4b09-99f5-84d1102f96b0", "John Matson"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 318 — Ghost Town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOST_TOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4218cdda-3a62-43fb-aaf7-7ac836392796"),
    "Ghost Town",
    crate::card::CardArt::new("4218cdda-3a62-43fb-aaf7-7ac836392796", "Tom Wänerstrand"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 319 — Maze of Shadows
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAZE_OF_SHADOWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba69c3d3-6fb5-478d-93ba-341dd3ace97d"),
    "Maze of Shadows",
    crate::card::CardArt::new(
        "ba69c3d3-6fb5-478d-93ba-341dd3ace97d",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 320 — Mogg Hollows
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static MOGG_HOLLOWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("474e3fb0-b0f9-4c6d-8c57-e5079a3a3c66"),
    "Mogg Hollows",
    crate::card::CardArt::new("474e3fb0-b0f9-4c6d-8c57-e5079a3a3c66", "Jeff Laubenstein"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 321 — Pine Barrens
pub(in crate::card::sets) static PINE_BARRENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5ac39e8-bd0e-4fa3-bc1e-a93944d013f3"),
    "Pine Barrens",
    CardArt::new("d5ac39e8-bd0e-4fa3-bc1e-a93944d013f3", "Rebecca Guay"),
    CardSet::Tempest,
    // The black-green member of the slow cycle.
    CardRules::new_land(&[]).with_abilities(
        &const {
            let [colorless, colored] = abilities::pain_land(
                "{T}: Add {B} or {G}. This land deals 1 damage to you.",
                &[ManaColor::Black, ManaColor::Green],
            );
            [abilities::enters_tapped(CardType::Land), colorless, colored]
        },
    ),
);

// TMP 322 — Reflecting Pool
pub(in crate::card::sets) static REFLECTING_POOL: CardRecord = CardRecord::new_with_legacy_id(
    2073,
    "Reflecting Pool",
    CardArt::new("4fc67298-6610-47d7-971b-baf5728d5349", "Adam Rex"),
    CardSet::Tempest,
    // Worth nothing on its own and everything beside four other lands, which
    // is why a five-color deck plays it and nobody else does.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any type that a land you control could produce.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice_from(
            ManaTypeSetDef::could_be_produced_by(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
        )),
    )),
);

// TMP 323 — Rootwater Depths
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static ROOTWATER_DEPTHS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d4bcbef-66bf-4625-82d5-a01c39d3d78e"),
    "Rootwater Depths",
    crate::card::CardArt::new("4d4bcbef-66bf-4625-82d5-a01c39d3d78e", "Roger Raupp"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 324 — Salt Flats
pub(in crate::card::sets) static SALT_FLATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("224cb63f-9af0-4b00-ba0b-0b604abf20c8"),
    "Salt Flats",
    CardArt::new("224cb63f-9af0-4b00-ba0b-0b604abf20c8", "Scott Kirschner"),
    CardSet::Tempest,
    // The white-black member.
    CardRules::new_land(&[]).with_abilities(
        &const {
            let [colorless, colored] = abilities::pain_land(
                "{T}: Add {W} or {B}. This land deals 1 damage to you.",
                &[ManaColor::White, ManaColor::Black],
            );
            [abilities::enters_tapped(CardType::Land), colorless, colored]
        },
    ),
);

// TMP 325 — Scabland
pub(in crate::card::sets) static SCABLAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0374f269-b07e-43af-911a-5454b35f14e6"),
    "Scabland",
    CardArt::new("0374f269-b07e-43af-911a-5454b35f14e6", "Andrew Robinson"),
    CardSet::Tempest,
    // The red-white member.
    CardRules::new_land(&[]).with_abilities(
        &const {
            let [colorless, colored] = abilities::pain_land(
                "{T}: Add {R} or {W}. This land deals 1 damage to you.",
                &[ManaColor::Red, ManaColor::White],
            );
            [abilities::enters_tapped(CardType::Land), colorless, colored]
        },
    ),
);

// TMP 326 — Skyshroud Forest
pub(in crate::card::sets) static SKYSHROUD_FOREST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa01f43b-d0b3-4cd5-9694-aed30a79462c"),
    "Skyshroud Forest",
    CardArt::new("aa01f43b-d0b3-4cd5-9694-aed30a79462c", "Roger Raupp"),
    CardSet::Tempest,
    // The green-blue member.
    CardRules::new_land(&[]).with_abilities(
        &const {
            let [colorless, colored] = abilities::pain_land(
                "{T}: Add {G} or {U}. This land deals 1 damage to you.",
                &[ManaColor::Green, ManaColor::Blue],
            );
            [abilities::enters_tapped(CardType::Land), colorless, colored]
        },
    ),
);

// TMP 327 — Stalking Stones
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALKING_STONES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4d3d349-5c23-43a9-b25e-0e1a35b84673"),
    "Stalking Stones",
    crate::card::CardArt::new("b4d3d349-5c23-43a9-b25e-0e1a35b84673", "Stephen Daniele"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 328 — Thalakos Lowlands
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static THALAKOS_LOWLANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa3dcb12-e224-40c8-aecf-941fceb1d323"),
    "Thalakos Lowlands",
    crate::card::CardArt::new("aa3dcb12-e224-40c8-aecf-941fceb1d323", "Jeff A. Menges"),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 329 — Vec Townships
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static VEC_TOWNSHIPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15377e49-e929-413f-9501-8f3e4afa0050"),
    "Vec Townships",
    crate::card::CardArt::new(
        "15377e49-e929-413f-9501-8f3e4afa0050",
        "Eric David Anderson",
    ),
    crate::card::CardSet::Tempest,
    crate::card::CardRules::unsupported(),
);

// TMP 330 — Wasteland
pub(in crate::card::sets) static WASTELAND: CardRecord = CardRecord::new_with_legacy_id(
    279,
    "Wasteland",
    CardArt::new("99ff731b-8399-40c8-b539-ba6ba5783771", "Una Fricker"),
    CardSet::Tempest,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Destroy target nonbasic land.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
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

// TMP 331 — Plains (reprint)

// TMP 332 — Plains (alternate printing)

// TMP 333 — Plains (alternate printing)

// TMP 334 — Plains (alternate printing)

// TMP 335 — Island (reprint)

// TMP 336 — Island (alternate printing)

// TMP 337 — Island (alternate printing)

// TMP 338 — Island (alternate printing)

// TMP 339 — Swamp (reprint)

// TMP 340 — Swamp (alternate printing)

// TMP 341 — Swamp (alternate printing)

// TMP 342 — Swamp (alternate printing)

// TMP 343 — Mountain (reprint)

// TMP 344 — Mountain (alternate printing)

// TMP 345 — Mountain (alternate printing)

// TMP 346 — Mountain (alternate printing)

// TMP 347 — Forest (reprint)

// TMP 348 — Forest (alternate printing)

// TMP 349 — Forest (alternate printing)

// TMP 350 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADVANCE_SCOUT,
    &ANGELIC_PROTECTOR,
    &ANOINT,
    &ARMOR_SLIVER,
    &ARMORED_PEGASUS,
    &AURATOG,
    &AVENGING_ANGEL,
    &CIRCLE_OF_PROTECTION_SHADOW,
    &CLERGY_EN_VEC,
    &CLOUDCHASER_EAGLE,
    &ELITE_JAVELINEER,
    &FIELD_OF_SOULS,
    &FLICKERING_WARD,
    &GALLANTRY,
    &GERRARD_S_BATTLE_CRY,
    &HANNA_S_CUSTODY,
    &HERO_S_RESOLVE,
    &HUMILITY,
    &INVULNERABILITY,
    &KNIGHT_OF_DAWN,
    &LIGHT_OF_DAY,
    &MARBLE_TITAN,
    &MASTER_DECOY,
    &MOUNTED_ARCHERS,
    &ORACLE_EN_VEC,
    &ORIM_S_PRAYER,
    &ORIM_SAMITE_HEALER,
    &PEGASUS_REFUGE,
    &QUICKENING_LICID,
    &REPENTANCE,
    &SACRED_GUIDE,
    &SAFEGUARD,
    &SERENE_OFFERING,
    &SOLTARI_CRUSADER,
    &SOLTARI_EMISSARY,
    &SOLTARI_FOOT_SOLDIER,
    &SOLTARI_LANCER,
    &SOLTARI_MONK,
    &SOLTARI_PRIEST,
    &SOLTARI_TROOPER,
    &SPIRIT_MIRROR,
    &STAUNCH_DEFENDERS,
    &TALON_SLIVER,
    &WARMTH,
    &WINDS_OF_RATH,
    &WORTHY_CAUSE,
    &BENTHIC_BEHEMOTH,
    &CAPSIZE,
    &CHILL,
    &DISMISS,
    &DUPLICITY,
    &ERTAI_S_MEDDLING,
    &ESCAPED_SHAPESHIFTER,
    &FIGHTING_DRAKE,
    &FYLAMARID,
    &GIANT_CRAB,
    &HORNED_TURTLE,
    &INSIGHT,
    &INTERDICT,
    &INTUITION,
    &LEGACY_S_ALLURE,
    &LEGERDEMAIN,
    &MANA_SEVERANCE,
    &MANTA_RIDERS,
    &MAWCOR,
    &MEDITATE,
    &MNEMONIC_SLIVER,
    &PRECOGNITION,
    &PROPAGANDA,
    &ROOTWATER_DIVER,
    &ROOTWATER_HUNTER,
    &ROOTWATER_MATRIARCH,
    &ROOTWATER_SHAMAN,
    &SEA_MONSTER,
    &SHADOW_RIFT,
    &SHIMMERING_WINGS,
    &SKYSHROUD_CONDOR,
    &STEAL_ENCHANTMENT,
    &STINGING_LICID,
    &THALAKOS_DREAMSOWER,
    &THALAKOS_MISTFOLK,
    &THALAKOS_SEER,
    &THALAKOS_SENTRY,
    &TIME_WARP,
    &TRADEWIND_RIDER,
    &TWITCH,
    &UNSTABLE_SHAPESHIFTER,
    &VOLRATH_S_CURSE,
    &WHIM_OF_VOLRATH,
    &WHISPERS_OF_THE_MUSE,
    &WIND_DANCER,
    &WINGED_SLIVER,
    &ABANDON_HOPE,
    &BELLOWING_FIEND,
    &BLOOD_PET,
    &BOUNTY_HUNTER,
    &CARRIONETTE,
    &CLOT_SLIVER,
    &COFFIN_QUEEN,
    &COMMANDER_GREVEN_IL_VEC,
    &CORPSE_DANCE,
    &DARKLING_STALKER,
    &DAUTHI_EMBRACE,
    &DAUTHI_GHOUL,
    &DAUTHI_MARAUDER,
    &DAUTHI_MERCENARY,
    &DAUTHI_MINDRIPPER,
    &DAUTHI_SLAYER,
    &DEATH_PITS_OF_RATH,
    &DIABOLIC_EDICT,
    &DISTURBED_BURIAL,
    &DREAD_OF_NIGHT,
    &DREGS_OF_SORROW,
    &ENDLESS_SCREAM,
    &EVINCAR_S_JUSTICE,
    &EXTINCTION,
    &FEVERED_CONVULSIONS,
    &IMPS_TAUNT,
    &KEZZERDRIX,
    &KNIGHT_OF_DUSK,
    &LEECHING_LICID,
    &LIVING_DEATH,
    &MADDENING_IMP,
    &MARSH_LURKER,
    &MINDWHIP_SLIVER,
    &MINION_OF_THE_WASTES,
    &PERISH,
    &PIT_IMP,
    &RAIN_OF_TEARS,
    &RATS_OF_RATH,
    &REANIMATE,
    &RECKLESS_SPITE,
    &SADISTIC_GLEE,
    &SARCOMANCY,
    &SCREECHING_HARPY,
    &SERVANT_OF_VOLRATH,
    &SKYSHROUD_VAMPIRE,
    &SOULDRINKER,
    &SPINAL_GRAFT,
    &AFTERSHOCK,
    &ANCIENT_RUNES,
    &APOCALYPSE,
    &BARBED_SLIVER,
    &BLOOD_FRENZY,
    &BOIL,
    &CANYON_DRAKE,
    &CANYON_WILDCAT,
    &CHAOTIC_GOO,
    &CROWN_OF_FLAMES,
    &DEADSHOT,
    &ENRAGING_LICID,
    &FIREFLY,
    &FIRESLINGER,
    &FLOWSTONE_GIANT,
    &FLOWSTONE_SALAMANDER,
    &FLOWSTONE_WYVERN,
    &FURNACE_OF_RATH,
    &GOBLIN_BOMBARDMENT,
    &HAND_TO_HAND,
    &HAVOC,
    &HEART_SLIVER,
    &JACKAL_PUP,
    &KINDLE,
    &LIGHTNING_BLAST,
    &LOWLAND_GIANT,
    &MAGMASAUR,
    &MOGG_CONSCRIPTS,
    &MOGG_FANATIC,
    &MOGG_RAIDER,
    &MOGG_SQUAD,
    &NO_QUARTER,
    &OPPORTUNIST,
    &PALLIMUD,
    &RATHI_DRAGON,
    &RENEGADE_WARLORD,
    &ROLLING_THUNDER,
    &SANDSTONE_WARRIOR,
    &SCORCHED_EARTH,
    &SEARING_TOUCH,
    &SHADOWSTORM,
    &SHOCKER,
    &STARKE_OF_RATH,
    &STUN,
    &SUDDEN_IMPACT,
    &TAHNGARTH_S_RAGE,
    &TOOTH_AND_CLAW,
    &WALL_OF_DIFFUSION,
    &WILD_WURM,
    &ALUREN,
    &APES_OF_RATH,
    &BAYOU_DRAGONFLY,
    &BROKEN_FALL,
    &CANOPY_SPIDER,
    &CHARGING_RHINO,
    &CHOKE,
    &CRAZED_ARMODON,
    &DIRTCOWL_WURM,
    &EARTHCRAFT,
    &ELADAMRI_S_VINEYARD,
    &ELADAMRI_LORD_OF_LEAVES,
    &ELVEN_WARHOUNDS,
    &ELVISH_FURY,
    &FLAILING_DRAKE,
    &FROG_TONGUE,
    &FUGITIVE_DRUID,
    &HARROW,
    &HEARTWOOD_DRYAD,
    &HEARTWOOD_GIANT,
    &HEARTWOOD_TREEFOLK,
    &HORNED_SLIVER,
    &KRAKILIN,
    &MIRRI_S_GUILE,
    &MONGREL_PACK,
    &MUSCLE_SLIVER,
    &NATURAL_SPRING,
    &NATURE_S_REVOLT,
    &NEEDLE_STORM,
    &NURTURING_LICID,
    &PINCHER_BEETLES,
    &REALITY_ANCHOR,
    &REAP,
    &RECYCLE,
    &RESPITE,
    &ROOT_MAZE,
    &ROOTBREAKER_WURM,
    &SCRAGNOTH,
    &SEEKER_OF_SKYBREAK,
    &SKYSHROUD_ELF,
    &SKYSHROUD_RANGER,
    &SKYSHROUD_TROLL,
    &SPIKE_DRONE,
    &STORM_FRONT,
    &TRAINED_ARMODON,
    &TRUMPETING_ARMODON,
    &VERDANT_FORCE,
    &VERDIGRIS,
    &WINTER_S_GRASP,
    &DRACOPLASM,
    &LOBOTOMY,
    &RANGER_EN_VEC,
    &SEGMENTED_WURM,
    &SELENIA_DARK_ANGEL,
    &SKY_SPIRIT,
    &SOLTARI_GUERRILLAS,
    &SPONTANEOUS_COMBUSTION,
    &VHATI_IL_DAL,
    &WOOD_SAGE,
    &ALTAR_OF_DEMENTIA,
    &BOOBY_TRAP,
    &BOTTLE_GNOMES,
    &COILED_TINVIPER,
    &COLD_STORAGE,
    &CURSED_SCROLL,
    &ECHO_CHAMBER,
    &EMERALD_MEDALLION,
    &EMMESSI_TOME,
    &ENERGIZER,
    &ESSENCE_BOTTLE,
    &EXCAVATOR,
    &FLOWSTONE_SCULPTURE,
    &FOOL_S_TOME,
    &GRINDSTONE,
    &HELM_OF_POSSESSION,
    &JET_MEDALLION,
    &JINXED_IDOL,
    &LOTUS_PETAL,
    &MAGNETIC_WEB,
    &MANAKIN,
    &METALLIC_SLIVER,
    &MOGG_CANNON,
    &PATCHWORK_GNOMES,
    &PEARL_MEDALLION,
    &PHYREXIAN_GRIMOIRE,
    &PHYREXIAN_SPLICER,
    &PUPPET_STRINGS,
    &RUBY_MEDALLION,
    &SAPPHIRE_MEDALLION,
    &SCALDING_TONGS,
    &SCROLL_RACK,
    &SQUEE_S_TOY,
    &STATIC_ORB,
    &TELETHOPTER,
    &THUMBSCREWS,
    &TORTURE_CHAMBER,
    &WATCHDOG,
    &ANCIENT_TOMB,
    &CALDERA_LAKE,
    &CINDER_MARSH,
    &GHOST_TOWN,
    &MAZE_OF_SHADOWS,
    &MOGG_HOLLOWS,
    &PINE_BARRENS,
    &REFLECTING_POOL,
    &ROOTWATER_DEPTHS,
    &SALT_FLATS,
    &SCABLAND,
    &SKYSHROUD_FOREST,
    &STALKING_STONES,
    &THALAKOS_LOWLANDS,
    &VEC_TOWNSHIPS,
    &WASTELAND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // TMP 8
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE),  // TMP 9
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // TMP 10
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED),   // TMP 11
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // TMP 13
    PrintingRecord::reprint(&catalog_lea::DISENCHANT),                 // TMP 16
    PrintingRecord::reprint(&catalog_m13::PACIFISM),                   // TMP 34
    PrintingRecord::reprint(&catalog_lea::COUNTERSPELL),               // TMP 57
    PrintingRecord::reprint(&catalog_mir::DREAM_CACHE),                // TMP 59
    PrintingRecord::reprint(&catalog_leg::GASEOUS_FORM),               // TMP 65
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),                 // TMP 78
    PrintingRecord::reprint(&catalog_lea::SPELL_BLAST),                // TMP 89
    PrintingRecord::reprint(&catalog_m14::TIME_EBB),                   // TMP 96
    PrintingRecord::reprint(&catalog_m13::WIND_DRAKE),                 // TMP 105
    PrintingRecord::reprint(&catalog_vis::COERCION),                   // TMP 113
    PrintingRecord::reprint(&catalog_ice::DARK_BANISHING),             // TMP 117
    PrintingRecord::reprint(&catalog_lea::DARK_RITUAL),                // TMP 118
    PrintingRecord::reprint(&catalog_clb::DAUTHI_HORROR),              // TMP 122
    PrintingRecord::reprint(&catalog_mir::ENFEEBLEMENT),               // TMP 133
    PrintingRecord::reprint(&catalog_m12::GRAVEDIGGER),                // TMP 137
    PrintingRecord::reprint(&catalog_leg::GIANT_STRENGTH),             // TMP 178
    PrintingRecord::reprint(&catalog_m12::LIGHTNING_ELEMENTAL),        // TMP 186
    PrintingRecord::reprint(&catalog_lea::SHATTER),                    // TMP 203
    PrintingRecord::reprint(&catalog_lea::STONE_RAIN),                 // TMP 206
    PrintingRecord::reprint(&catalog_m12::OVERRUN),                    // TMP 243
    PrintingRecord::reprint(&catalog_m12::RAMPANT_GROWTH),             // TMP 245
    PrintingRecord::reprint(&catalog_m14::ROOTWALLA),                  // TMP 252
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),                // TMP 261
    PrintingRecord::reprint(&catalog_m13::PHYREXIAN_HULK),             // TMP 302
    PrintingRecord::reprint(&catalog_lea::PLAINS),                     // TMP 331
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),                // TMP 332
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),                // TMP 333
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3),                // TMP 334
    PrintingRecord::reprint(&catalog_lea::ISLAND),                     // TMP 335
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),                // TMP 336
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),                // TMP 337
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3),                // TMP 338
    PrintingRecord::reprint(&catalog_lea::SWAMP),                      // TMP 339
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),                 // TMP 340
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),                 // TMP 341
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3),                 // TMP 342
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),                   // TMP 343
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),              // TMP 344
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2),              // TMP 345
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3),              // TMP 346
    PrintingRecord::reprint(&catalog_lea::FOREST),                     // TMP 347
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),                // TMP 348
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),                // TMP 349
    PrintingRecord::alternate(&catalog_lea::FOREST, 3),                // TMP 350
];
