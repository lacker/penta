//! Tempest cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardChoiceSourceDef;
use crate::card::CardNameDef;
use crate::card::CardNameSetDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ColorChoiceOperationDef;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::DiscardSelectionDef;
use crate::card::DividedTotal;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaTypeSetDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SacrificedAmountDef;
use crate::card::ScaledValueDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TargetChooserDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::ids::ParentBinding;
use crate::mana_cost;

const fn all_slivers_get(effect: AppliedEffectDef) -> EffectDef {
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
        effect,
    }
}

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
/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TMP",
    slug: "tempest",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// TMP 1 — Advance Scout
pub(in crate::card::sets) static ADVANCE_SCOUT: CardRecord = CardRecord::new(
    "Advance Scout",
    "81ce7e1e-ffe5-4ced-8967-9a6917245240",
    "Heather Hudson",
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
pub(in crate::card::sets) static ANGELIC_PROTECTOR: CardRecord = CardRecord::new(
    "Angelic Protector",
    "44faefbe-d5e7-48f3-ba88-833da0b19707",
    "DiTerlizzi",
CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature becomes the target of a spell or ability, this creature gets +0/+3 until end of turn.",
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::Any),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 3 — Anoint
pub(in crate::card::sets) static ANOINT: CardRecord = CardRecord::new(
    "Anoint",
    "ca65ee0f-fdd7-4a5e-a4a3-5dd9c62096ab",
    "Eric David Anderson",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Prevent the next 3 damage that would be dealt to target creature this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 4 — Armor Sliver
pub(in crate::card::sets) static ARMOR_SLIVER: CardRecord = CardRecord::new(
    "Armor Sliver",
    "c275aba7-cac6-48e8-b12c-6bd77a5c38fe",
    "Scott Kirschner",
    // A mana sink every Sliver shares, which turns a board of 1/1s into a
    // board nothing profitably blocks.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have \"{2}: This creature gets +0/+1 until end of turn.\"",
            all_slivers_get(AppliedEffectDef::add_ability(
                &const {
                    abilities::apply_to_self_until_end_of_turn(
                        "{2}: This creature gets +0/+1 until end of turn.",
                        &[CostDef::Mana(mana_cost!("{2}"))],
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(1),
                        ),
                    )
                },
            )),
        ),
    ),
);

// TMP 5 — Armored Pegasus (reprint)
const ARMORED_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ARMORED_PEGASUS,
    "012049f8-0936-49ed-948d-0d34af28550f",
    "Una Fricker",
);

// TMP 6 — Auratog
pub(in crate::card::sets) static AURATOG: CardRecord = CardRecord::new(
    "Auratog",
    "86dca066-d5e3-442a-95a0-e695c1d5850c",
    "Jeff Miracola",
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
pub(in crate::card::sets) static AVENGING_ANGEL: CardRecord = CardRecord::new(
    "Avenging Angel",
    "28333138-60bc-459b-a0cd-1b7fd19c89cd",
    "Matthew D. Wilson",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, you may put it on top of its owner's library.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::TriggeringZoneChangeResult,
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// TMP 8 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "da6dda88-fc2a-4404-8a82-40c5d77860da",
    "Harold McNeill",
);

// TMP 9 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "0430fb60-78f6-4577-9ec5-a93d6662ef76",
    "Harold McNeill",
);

// TMP 10 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "88c76e49-ddd2-4967-a81a-86405260b4bc",
    "Harold McNeill",
);

// TMP 11 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "cc38872d-f389-43aa-b6f7-97b2c90e5a1f",
    "Harold McNeill",
);

// TMP 12 — Circle of Protection: Shadow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_SHADOW: CardRecord = CardRecord::new(
    "Circle of Protection: Shadow",
    "49f29a3b-7136-496c-bc29-8808bfff0f82",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// TMP 13 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "6eff4351-5501-4061-a409-49f518ba9628",
    "Harold McNeill",
);

// TMP 14 — Clergy en-Vec
pub(in crate::card::sets) static CLERGY_EN_VEC: CardRecord = CardRecord::new(
    "Clergy en-Vec",
    "fcb0e068-16d0-4e1c-acad-0a6d34148c5a",
    "Heather Hudson",
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
    "Cloudchaser Eagle",
    "3a70a6da-dea3-49c0-8c49-6a2229c3ac91",
    "Una Fricker",
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
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "8a65e678-dcb9-4c6c-9a30-8332030dead6",
    "Allen Williams",
);

// TMP 17 — Elite Javelineer
pub(in crate::card::sets) static ELITE_JAVELINEER: CardRecord = CardRecord::new(
    "Elite Javelineer",
    "ea1c730f-76da-4eae-b3fc-b428b860ea93",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature blocks, it deals 1 damage to target attacking creature.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::Any,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 18 — Field of Souls
pub(in crate::card::sets) static FIELD_OF_SOULS: CardRecord = CardRecord::new(
    "Field of Souls",
    "9816a3ef-e2a8-4d97-afbf-d190a62265bf",
    "Richard Kane Ferguson",
CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(
        abilities::dies_trigger_matching(
            "Whenever a nontoken creature is put into your graveyard from the battlefield, create a 1/1 white Spirit creature token with flying.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ObjectPredicateDef::OwnedBy(PlayerRelation::You),
            ]),
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()]),
        ),
    ),
);

// TMP 19 — Flickering Ward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLICKERING_WARD: CardRecord = CardRecord::new(
    "Flickering Ward",
    "d4d2b011-bb0d-463c-bf2a-04b6650771a3",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// TMP 20 — Gallantry
pub(in crate::card::sets) static GALLANTRY: CardRecord = CardRecord::new(
    "Gallantry",
    "1ccdca3b-7d53-4d19-bd15-9a1b148c4aaf",
    "Douglas Shuler",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target blocking creature gets +4/+4 until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Blocking,
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// TMP 21 — Gerrard's Battle Cry
pub(in crate::card::sets) static GERRARD_S_BATTLE_CRY: CardRecord = CardRecord::new(
    "Gerrard's Battle Cry",
    "504950d5-2df7-4518-b987-fe3a57ad1c58",
    "Val Mayerik",
    CardRules::new_enchantment(mana_cost!("{W}")).with_ability(AbilityDef::activated(
        "{2}{W}: Creatures you control get +1/+1 until end of turn.",
        &[CostDef::Mana(mana_cost!("{2}{W}"))],
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// TMP 22 — Hanna's Custody
pub(in crate::card::sets) static HANNA_S_CUSTODY: CardRecord = CardRecord::new(
    "Hanna's Custody",
    "7ea44536-ef4e-4dcf-9c1a-c1122dd00cbb",
    "DiTerlizzi",
    // Symmetrical, which in a format where one deck plays the artifacts and
    // the other plays the answers is not symmetrical at all.
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(AbilityDef::static_ability(
        "All artifacts have shroud.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&const { abilities::shroud() }),
        },
    )),
);

// TMP 23 — Hero's Resolve
pub(in crate::card::sets) static HERO_S_RESOLVE: CardRecord = CardRecord::new(
    "Hero's Resolve",
    "b4cdcc7c-0d01-4aa2-8934-079dfc00eef2",
    "Pete Venters",
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
pub(in crate::card::sets) static HUMILITY: CardRecord = CardRecord::new(
    "Humility",
    "a2fb7128-806b-4148-80fe-eb967f248021",
    "Phil Foglio",
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
    "Invulnerability",
    "d66d1f00-e857-4bc3-a36d-a33669d281e9",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// TMP 26 — Knight of Dawn
pub(in crate::card::sets) static KNIGHT_OF_DAWN: CardRecord = CardRecord::new(
    "Knight of Dawn",
    "bf0e5034-a134-4eb6-af8e-b2419b92b3a6",
    "Ron Spencer",
CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::activated(
                "{W}{W}: This creature gains protection from the color of your choice until end of turn.",
                &[CostDef::Mana(mana_cost!("{W}{W}"))],
                EffectDef::ChooseColor {
                    object: EffectRecipientDef::Source,
                    operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// TMP 27 — Light of Day
pub(in crate::card::sets) static LIGHT_OF_DAY: CardRecord = CardRecord::new(
    "Light of Day",
    "70fa9ebe-bdf5-4359-aa3e-6cfa1a1d96cf",
    "Drew Tucker",
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::static_ability(
        "Black creatures can't attack or block.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            ]),
        },
    )),
);

// TMP 28 — Marble Titan
pub(in crate::card::sets) static MARBLE_TITAN: CardRecord = CardRecord::new(
    "Marble Titan",
    "9ca62c97-0bbd-4f74-afd5-99b48c063aa0",
    "Brom",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Giant"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "Creatures with power 3 or greater don't untap during their controllers' untap steps.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(3),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
    ),
);

// TMP 29 — Master Decoy
pub(in crate::card::sets) static MASTER_DECOY: CardRecord = CardRecord::new(
    "Master Decoy",
    "f3e11097-1ace-4ae8-a9e8-d00b9f709e54",
    "Phil Foglio",
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
pub(in crate::card::sets) static MOUNTED_ARCHERS: CardRecord = CardRecord::new(
    "Mounted Archers",
    "4f3abcf2-fe52-4096-8fdb-6917d75a04e3",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier", "Archer"], 2, 3)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::activated(
                "{W}: This creature can block an additional creature this turn.",
                &[CostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// TMP 31 — Oracle en-Vec
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORACLE_EN_VEC: CardRecord = CardRecord::new(
    "Oracle en-Vec",
    "cc538730-c46c-4e5f-bc1f-0efb7765086d",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// TMP 32 — Orim's Prayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_PRAYER: CardRecord = CardRecord::new(
    "Orim's Prayer",
    "2dc45565-4b56-49ba-b115-be8e0de7d937",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// TMP 33 — Orim, Samite Healer
pub(in crate::card::sets) static ORIM_SAMITE_HEALER: CardRecord = CardRecord::new(
    "Orim, Samite Healer",
    "7086d077-f083-4870-8b0b-2d34aca49df1",
    "Kaja Foglio",
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
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "6492bf53-ad49-4cd5-83df-0005a5b77811",
    "Adam Rex",
);

// TMP 35 — Pegasus Refuge
pub(in crate::card::sets) static PEGASUS_REFUGE: CardRecord = CardRecord::new(
    "Pegasus Refuge",
    "a2bce334-0ae6-4a7d-85db-99ee205ce546",
    "Kev Walker",
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::activated(
        "{2}, Discard a card: Create a 1/1 white Pegasus creature token with flying.",
        &[
            CostDef::Mana(mana_cost!("{2}")),
            CostDef::discard(ObjectPredicateDef::Any),
        ],
        EffectDef::create_creature_token(&["Pegasus"], &[ManaColor::White], 1, 1)
            .with_abilities(&[abilities::flying()]),
    )),
);

// TMP 36 — Quickening Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKENING_LICID: CardRecord = CardRecord::new(
    "Quickening Licid",
    "e6e91f3d-5a23-4df1-a879-d18a3af92a28",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// TMP 37 — Repentance
pub(in crate::card::sets) static REPENTANCE: CardRecord = CardRecord::new(
    "Repentance",
    "3e28ac76-c671-4be1-bcfc-17f2d7bbe08f",
    "Ron Spencer",
    // The bigger the creature, the more surely it kills itself, which is
    // white's answer to something it cannot block.
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature deals damage to itself equal to its power.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The creature is the source as well as the recipient, so lifelink or
        // a damage trigger on it sees its own damage.
        EffectDef::damage_from(
            ObjectRefDef::Target(TargetIndex::PRIMARY),
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::TargetPower(TargetIndex::PRIMARY),
        ),
    )),
);

// TMP 38 — Sacred Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_GUIDE: CardRecord = CardRecord::new(
    "Sacred Guide",
    "7f10c37d-d25a-47d6-83b0-dbe0a9cfc938",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// TMP 39 — Safeguard
pub(in crate::card::sets) static SAFEGUARD: CardRecord = CardRecord::new(
    "Safeguard",
    "2c8e174c-7abb-4a93-aa1d-8c2a2e815ba6",
    "Thomas M. Baxa",
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{W}: Prevent all combat damage that would be dealt by target creature this turn.",
            &[CostDef::Mana(mana_cost!("{2}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 40 — Serene Offering
pub(in crate::card::sets) static SERENE_OFFERING: CardRecord = CardRecord::new(
    "Serene Offering",
    "6c0b3795-7f30-4c61-b5d8-f238055d6be1",
    "Paolo Parente",
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
    "Soltari Crusader",
    "6cd07471-b216-465c-9946-1eac689db32e",
    "Randy Gallegos",
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
    "Soltari Emissary",
    "a18751d3-052b-4ae5-ba07-16f00a1af40e",
    "Adam Rex",
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
    "Soltari Foot Soldier",
    "bdf295dc-72df-4097-b767-d89ab807bf2e",
    "Janet Aulisio",
    // One mana for one unblockable damage a turn, which is what shadow does
    // in a format where nobody else has it.
    CardRules::new_creature(mana_cost!("{W}"), &["Soltari", "Soldier"], 1, 1)
        .with_ability(abilities::shadow()),
);

// TMP 44 — Soltari Lancer
pub(in crate::card::sets) static SOLTARI_LANCER: CardRecord = CardRecord::new(
    "Soltari Lancer",
    "ab4b6c91-dd07-4d39-bd36-6fbf28e7698e",
    "Matthew D. Wilson",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Soltari", "Knight"], 2, 2).with_abilities(&[
        abilities::shadow(),
        AbilityDef::static_ability(
            "This creature has first strike as long as it's attacking.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::Attacking,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            },
        ),
    ]),
);

// TMP 45 — Soltari Monk
pub(in crate::card::sets) static SOLTARI_MONK: CardRecord = CardRecord::new(
    "Soltari Monk",
    "54e0d969-3e4d-4ff9-8bda-3a6ac8df01b2",
    "Janet Aulisio",
    // Shadow makes it unblockable and protection makes it unkillable by the
    // one colour that could have raced it.
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Soltari", "Monk"], 2, 1).with_abilities(&[
        abilities::shadow(),
        abilities::protection_from_color(ManaColor::Black),
    ]),
);

// TMP 46 — Soltari Priest
pub(in crate::card::sets) static SOLTARI_PRIEST: CardRecord = CardRecord::new(
    "Soltari Priest",
    "35a71390-3fa8-43eb-ad86-67de2a7aeab8",
    "Janet Aulisio",
    // The same two mana aimed at the burn deck instead, which in this format
    // was the other half of the field.
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Soltari", "Cleric"], 2, 1).with_abilities(&[
        abilities::shadow(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// TMP 47 — Soltari Trooper
pub(in crate::card::sets) static SOLTARI_TROOPER: CardRecord = CardRecord::new(
    "Soltari Trooper",
    "32f74aa3-4003-4f53-b774-22b111935391",
    "Kev Walker",
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
pub(in crate::card::sets) static SPIRIT_MIRROR: CardRecord = CardRecord::new(
    "Spirit Mirror",
    "8a7089c9-70ba-4009-86a5-d4e322c00fba",
    "D. Alexander Gregory",
CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if there are no Reflection tokens on the battlefield, create a 2/2 white Reflection creature token.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Token,
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Reflection")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::create_creature_token(&["Reflection"], &[ManaColor::White], 2, 2),
        ),
        AbilityDef::activated_with_targets(
            "{0}: Destroy target Reflection.",
            &[],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Reflection")),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// TMP 49 — Staunch Defenders
pub(in crate::card::sets) static STAUNCH_DEFENDERS: CardRecord = CardRecord::new(
    "Staunch Defenders",
    "88ed7210-17a4-4750-a003-617ba75bff3e",
    "Mark Poole",
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
    "Talon Sliver",
    "f186c4b1-b7ec-46eb-a961-257411b401b0",
    "Mike Raabe",
    // The white member of the cycle, which turns every Sliver on the board
    // into a creature nothing profitably blocks.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have first strike.",
            all_slivers_get(AppliedEffectDef::add_ability(
                &const { abilities::first_strike() },
            )),
        ),
    ),
);

// TMP 51 — Warmth
pub(in crate::card::sets) static WARMTH: CardRecord = CardRecord::new(
    "Warmth",
    "d7dbeea8-06b0-4482-bdae-aa82b9db8856",
    "Drew Tucker",
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
    "Winds of Rath",
    "a6d731b2-0113-4fd5-8b78-1aa1064bb4f5",
    "Drew Tucker",
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
    "Worthy Cause",
    "1f610d8b-1782-43a4-bfb3-40887bdedba0",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// TMP 54 — Benthic Behemoth
pub(in crate::card::sets) static BENTHIC_BEHEMOTH: CardRecord = CardRecord::new(
    "Benthic Behemoth",
    "cc9fb7b6-d20c-4c08-9dae-4ccc9138b662",
    "Jim Nelson",
    // Eight mana for seven unblockable damage against any blue deck, and a
    // 7/6 that gets chumped against everybody else.
    CardRules::new_creature(mana_cost!("{5}{U}{U}{U}"), &["Serpent"], 7, 6)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// TMP 55 — Capsize
pub(in crate::card::sets) static CAPSIZE: CardRecord = CardRecord::new(
    "Capsize",
    "e538b359-d893-422d-9d60-5f3e8ee0fa9e",
    "Tom Wänerstrand",
    // Six mana a turn to bounce anything forever: the buyback is the card,
    // and the unbought half is only what you cast when tapped out.
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Return target permanent to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// TMP 56 — Chill
pub(in crate::card::sets) static CHILL: CardRecord = CardRecord::new(
    "Chill",
    "5a7bd777-6f11-441e-887f-9cee1ef96035",
    "Greg Simanson",
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
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "dacdd380-71cf-4832-bd02-3697501325f3",
    "Stephen Daniele",
);

// TMP 58 — Dismiss
pub(in crate::card::sets) static DISMISS: CardRecord = CardRecord::new(
    "Dismiss",
    "1e55d6be-7682-4786-9872-e847afd710b0",
    "Donato Giancola",
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::counter_target(TargetIndex::PRIMARY),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// TMP 59 — Dream Cache (reprint)
const DREAM_CACHE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::DREAM_CACHE,
    "875f81cf-5f27-451e-9248-746fad1e43d7",
    "Phil Foglio",
);

// TMP 60 — Duplicity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUPLICITY: CardRecord = CardRecord::new(
    "Duplicity",
    "d529cb33-292d-40e3-8cfe-db5eeb0d711e",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// TMP 61 — Ertai's Meddling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_S_MEDDLING: CardRecord = CardRecord::new(
    "Ertai's Meddling",
    "35c7e7fa-1493-4ef8-9cdb-b02b07a1ad85",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// TMP 62 — Escaped Shapeshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESCAPED_SHAPESHIFTER: CardRecord = CardRecord::new(
    "Escaped Shapeshifter",
    "e0171d4f-c871-4a7f-821a-82b7f401e9ca",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// TMP 63 — Fighting Drake
pub(in crate::card::sets) static FIGHTING_DRAKE: CardRecord = CardRecord::new(
    "Fighting Drake",
    "be436b65-9193-45ca-93e0-c5e9718f7e72",
    "DiTerlizzi",
    // A flier that blocks fliers and lives, which is what blue wanted from
    // four mana before it wanted card advantage.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Drake"], 2, 4)
        .with_abilities(&[abilities::flying()]),
);

// TMP 64 — Fylamarid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYLAMARID: CardRecord = CardRecord::new(
    "Fylamarid",
    "8dd4f686-79e3-4067-81f9-7fae0c25dc8f",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// TMP 65 — Gaseous Form (reprint)
const GASEOUS_FORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GASEOUS_FORM,
    "ce8402d6-b509-4771-ba80-128db343880d",
    "Roger Raupp",
);

// TMP 66 — Giant Crab
pub(in crate::card::sets) static GIANT_CRAB: CardRecord = CardRecord::new(
    "Giant Crab",
    "11c65a35-e219-4b60-ab95-ce7eff67d646",
    "Tom Kyffin",
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

// TMP 67 — Horned Turtle (reprint)
const HORNED_TURTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::HORNED_TURTLE,
    "b2348ce1-6305-42a7-8061-64275f6dc5c6",
    "DiTerlizzi",
);

// TMP 68 — Insight
pub(in crate::card::sets) static INSIGHT: CardRecord = CardRecord::new(
    "Insight",
    "1dfd9cb9-51f6-4d09-b5c0-5b0ed9d16542",
    "Ron Chironna",
    // Drawing a card off every spell they cast is the sort of hoser that wins a
    // long game outright, if the matchup ever shows up.
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a green spell, you draw a card.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::Color(ManaColor::Green),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// TMP 69 — Interdict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTERDICT: CardRecord = CardRecord::new(
    "Interdict",
    "3442c919-73b9-4d29-a014-87293f456325",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// TMP 70 — Intuition
pub(in crate::card::sets) static INTUITION: CardRecord = CardRecord::new(
    "Intuition",
    "c99f6785-e5a1-4fdc-9fb5-e1a372e7e848",
    "April Lee",
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
                                EffectDef::move_to_zone(
                                    EffectRecipientDef::object(ObjectRefDef::Binding(
                                        Binding!("intuition_chosen"),
                                    )),
                                    ZoneKind::Hand,
                                    ZonePlacement::Top,
                                ),
                                EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        Binding!("intuition_unchosen"),
                                    )),
                                    ZoneKind::Graveyard,
                                    ZonePlacement::Top,
                                ),
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
    "Legacy's Allure",
    "649a89c5-71bd-4fee-ae35-78081e4e0353",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// TMP 72 — Legerdemain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEGERDEMAIN: CardRecord = CardRecord::new(
    "Legerdemain",
    "65506830-fa2c-4e3b-9f64-5a569dd28249",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// TMP 73 — Mana Severance
pub(in crate::card::sets) static MANA_SEVERANCE: CardRecord = CardRecord::new(
    "Mana Severance",
    "854dc5e6-63f7-4c8b-83e5-a364f41c9a15",
    "Terese Nielsen",
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Search your library for any number of land cards, exile them, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Land),
            minimum: 0,
            maximum: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Library],
                PlayerRelation::You,
            )),
            reveal: false,
            destination: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// TMP 74 — Manta Riders
pub(in crate::card::sets) static MANTA_RIDERS: CardRecord = CardRecord::new(
    "Manta Riders",
    "cdff306c-1c7e-49ae-b10f-99e1927bbef1",
    "Kaja Foglio",
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
    "Mawcor",
    "9f50971e-2a18-4db7-8b5b-83dd5e85766e",
    "John Matson",
    // The same ping on a body that survives what it is shooting at.
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Beast"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// TMP 76 — Meditate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEDITATE: CardRecord = CardRecord::new(
    "Meditate",
    "edb79a97-c1fc-4aa3-bb13-3d24a6dabeea",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// TMP 77 — Mnemonic Sliver
pub(in crate::card::sets) static MNEMONIC_SLIVER: CardRecord = CardRecord::new(
    "Mnemonic Sliver",
    "2b167347-2f8f-4338-a651-c7543d812597",
    "Randy Gallegos",
    // It turns a board that has stopped attacking back into cards, one
    // Sliver at a time.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Slivers have \"{2}, Sacrifice this permanent: Draw a card.\"",
            all_slivers_get(AppliedEffectDef::add_ability(
                &const {
                    AbilityDef::activated(
                        "{2}, Sacrifice this permanent: Draw a card.",
                        &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    )
                },
            )),
        ),
    ),
);

// TMP 78 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "abc58c34-c3de-47f8-a42f-3a974dcb9c47",
    "Jeff Miracola",
);

// TMP 79 — Precognition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRECOGNITION: CardRecord = CardRecord::new(
    "Precognition",
    "76a0e317-5a76-4eac-a903-b0e3f0a45873",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// TMP 80 — Propaganda
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROPAGANDA: CardRecord = CardRecord::new(
    "Propaganda",
    "f67dde4d-3df1-480d-a8b8-ab22c768bb12",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// TMP 81 — Rootwater Diver
pub(in crate::card::sets) static ROOTWATER_DIVER: CardRecord = CardRecord::new(
    "Rootwater Diver",
    "a6315323-cf82-46c0-b164-e6ea1bf809f4",
    "Ron Spencer",
// The blue equivalent, and in an artifact block the card it buys back is
    // usually the one that was worth destroying.
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Return target artifact card from your graveyard to your hand.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &const {
                [AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                })]
            },
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ),
);

// TMP 82 — Rootwater Hunter
pub(in crate::card::sets) static ROOTWATER_HUNTER: CardRecord = CardRecord::new(
    "Rootwater Hunter",
    "cdf7ea34-2cde-4ec5-9b12-99b0002da986",
    "Brom",
    // A pinger in blue, which is unusual enough that the three-mana 1/1
    // body is beside the point.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 83 — Rootwater Matriarch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_MATRIARCH: CardRecord = CardRecord::new(
    "Rootwater Matriarch",
    "ec46812d-0721-4e93-b1a7-1d38f477fab6",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// TMP 84 — Rootwater Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_SHAMAN: CardRecord = CardRecord::new(
    "Rootwater Shaman",
    "caa1b84b-efda-4324-9106-0d1d00385cdc",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// TMP 85 — Sea Monster
pub(in crate::card::sets) static SEA_MONSTER: CardRecord = CardRecord::new(
    "Sea Monster",
    "8d3837ac-54af-44f7-b576-ad5badbee9f2",
    "Daniel Gelon",
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
pub(in crate::card::sets) static SHADOW_RIFT: CardRecord = CardRecord::new(
    "Shadow Rift",
    "57c11175-9feb-4801-9b46-d577d5ecef40",
    "Adam Rex",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains shadow until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::shadow()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// TMP 87 — Shimmering Wings
pub(in crate::card::sets) static SHIMMERING_WINGS: CardRecord = CardRecord::new(
    "Shimmering Wings",
    "a6a8dc46-04c7-479a-90c1-b55e6c67e0e3",
    "Steve Luke",
    // One mana for evasion that can be picked up and replayed, which makes
    // it a repeatable trick rather than a card spent.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                },
            ),
            AbilityDef::activated(
                "{U}: Return this Aura to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{U}"))],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TMP 88 — Skyshroud Condor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_CONDOR: CardRecord = CardRecord::new(
    "Skyshroud Condor",
    "a7d05ef5-c046-4929-b59d-988f0313a645",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// TMP 89 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "5fe58a24-f6a6-4858-82a5-0ca1d524efe1",
    "Steve Luke",
);

// TMP 90 — Steal Enchantment
pub(in crate::card::sets) static STEAL_ENCHANTMENT: CardRecord = CardRecord::new(
    "Steal Enchantment",
    "734be7fa-0998-4771-9b97-4989b3fc1471",
    "Hannibal King",
    // Two mana for whatever enchantment the opponent spent the most on,
    // which is the cheapest theft in the game when it has a target.
    CardRules::new_enchantment(mana_cost!("{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_enchantment(),
            AbilityDef::static_ability(
                "You control enchanted enchantment.",
                EffectDef::gain_control(
                    EffectRecipientDef::AttachedPermanent,
                    PlayerRefDef::EffectController,
                    ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                ),
            ),
        ]),
);

// TMP 91 — Stinging Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STINGING_LICID: CardRecord = CardRecord::new(
    "Stinging Licid",
    "807227d7-2eb2-4d47-bb3c-9d1ec9befeb7",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// TMP 92 — Thalakos Dreamsower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THALAKOS_DREAMSOWER: CardRecord = CardRecord::new(
    "Thalakos Dreamsower",
    "d725cdc0-3a85-4722-bb13-40c336f511b6",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// TMP 93 — Thalakos Mistfolk
pub(in crate::card::sets) static THALAKOS_MISTFOLK: CardRecord = CardRecord::new(
    "Thalakos Mistfolk",
    "9e7b5b00-9d14-4090-b8c3-28b70375571e",
    "Richard Kane Ferguson",
    // Unblockable, and it can dodge removal by going back to the top of the
    // library -- at the price of the next draw.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Thalakos", "Illusion"], 2, 1).with_abilities(
        &[
            abilities::shadow(),
            AbilityDef::activated(
                "{U}: Put this creature on top of its owner's library.",
                &[CostDef::Mana(mana_cost!("{U}"))],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
            ),
        ],
    ),
);

// TMP 94 — Thalakos Seer
pub(in crate::card::sets) static THALAKOS_SEER: CardRecord = CardRecord::new(
    "Thalakos Seer",
    "136a7d63-94ae-4d92-86ab-12bf9d78a803",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Thalakos", "Wizard"], 1, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TMP 95 — Thalakos Sentry
pub(in crate::card::sets) static THALAKOS_SENTRY: CardRecord = CardRecord::new(
    "Thalakos Sentry",
    "739a13d6-5f73-4166-b923-9db8ee3f2cf7",
    "Andrew Robinson",
    // Blue's shadow body: it connects every turn and blocks the other
    // shadow creatures.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Thalakos", "Soldier"], 1, 2)
        .with_ability(abilities::shadow()),
);

// TMP 96 — Time Ebb (reprint)
const TIME_EBB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::TIME_EBB,
    "69e63f0c-a09f-493d-a8a9-ddcc0a0ca383",
    "Thomas M. Baxa",
);

// TMP 97 — Time Warp
pub(in crate::card::sets) static TIME_WARP: CardRecord = CardRecord::new(
    "Time Warp",
    "3447aeaf-3b26-442a-99d4-0a7ee76c8e76",
    "Pete Venters",
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
pub(in crate::card::sets) static TRADEWIND_RIDER: CardRecord = CardRecord::new(
    "Tradewind Rider",
    "09412374-3645-4644-952e-2beaefb3104b",
    "John Matson",
CardRules::new_creature(mana_cost!("{3}{U}"), &["Spirit"], 1, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}, Tap two untapped creatures you control: Return target permanent to its owner's hand.",
            &[
                CostDef::TapSource,
                CostDef::TapPermanents {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                    count: 2,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// TMP 99 — Twitch
pub(in crate::card::sets) static TWITCH: CardRecord = CardRecord::new(
    "Twitch",
    "cba021eb-3d8b-41bf-aec4-af211e0860ad",
    "DiTerlizzi",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "You may tap or untap target artifact, creature, or land.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Tap the target permanent",
                            effect: EffectDef::Tap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        },
                        EffectChoiceDef {
                            label: "Untap the target permanent",
                            effect: EffectDef::Untap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        },
                    ],
                },
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// TMP 100 — Unstable Shapeshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSTABLE_SHAPESHIFTER: CardRecord = CardRecord::new(
    "Unstable Shapeshifter",
    "84e8cbd4-f49d-420d-a027-3be64ca58989",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// TMP 101 — Volrath's Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_S_CURSE: CardRecord = CardRecord::new(
    "Volrath's Curse",
    "bce63d86-8748-428a-aa9c-d3c0526537a2",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// TMP 102 — Whim of Volrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIM_OF_VOLRATH: CardRecord = CardRecord::new(
    "Whim of Volrath",
    "e259da60-c8bc-4a77-98ed-e529dc067732",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// TMP 103 — Whispers of the Muse
pub(in crate::card::sets) static WHISPERS_OF_THE_MUSE: CardRecord = CardRecord::new(
    "Whispers of the Muse",
    "75c5cfd1-3f7c-4250-a84d-8db83c6d7eb7",
    "Quinton Hoover",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{5}"))]),
        AbilityDef::spell(
            "Draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TMP 104 — Wind Dancer
pub(in crate::card::sets) static WIND_DANCER: CardRecord = CardRecord::new(
    "Wind Dancer",
    "ea7f7a94-700a-4f3b-846c-a36505b80875",
    "Susan Van Camp",
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
const WIND_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::WIND_DRAKE,
    "91e0c9e2-a45d-44d1-b73e-73c0a22d0752",
    "Greg Simanson",
);

// TMP 106 — Winged Sliver
pub(in crate::card::sets) static WINGED_SLIVER: CardRecord = CardRecord::new(
    "Winged Sliver",
    "03aa58b4-dbc2-414e-aa7a-f09360d59b3c",
    "Anthony S. Waters",
    // Evasion for the whole tribe, which is what turned a board of 1/1s
    // into a clock.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have flying.",
            all_slivers_get(AppliedEffectDef::add_ability(
                &const { abilities::flying() },
            )),
        ),
    ),
);

// TMP 107 — Abandon Hope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABANDON_HOPE: CardRecord = CardRecord::new(
    "Abandon Hope",
    "942cf220-472c-48f6-8f60-993939ea5ab8",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// TMP 108 — Bellowing Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELLOWING_FIEND: CardRecord = CardRecord::new(
    "Bellowing Fiend",
    "26915b4b-ada1-45f3-b908-04a774011b66",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// TMP 109 — Blood Pet
pub(in crate::card::sets) static BLOOD_PET: CardRecord = CardRecord::new(
    "Blood Pet",
    "5a89ba1b-e68b-4d70-a25e-27be9bf48a3b",
    "Brom",
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
    "Bounty Hunter",
    "98319fd3-0aad-4fc3-bb83-3c027d0ed652",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// TMP 111 — Carrionette
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRIONETTE: CardRecord = CardRecord::new(
    "Carrionette",
    "884e19fb-67a4-42d8-b163-720a99cb8506",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// TMP 112 — Clot Sliver
pub(in crate::card::sets) static CLOT_SLIVER: CardRecord = CardRecord::new(
    "Clot Sliver",
    "fdead1f4-a6e4-4370-80ae-811881a90d01",
    "Jeff Laubenstein",
    // Every Sliver on the board becomes unkillable in combat for two mana,
    // which is the clause that made the tribe a real deck.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Slivers have \"{2}: Regenerate this permanent.\"",
            all_slivers_get(AppliedEffectDef::add_ability(
                &const {
                    abilities::regenerate_self(
                        "{2}: Regenerate this permanent.",
                        &[CostDef::Mana(mana_cost!("{2}"))],
                    )
                },
            )),
        ),
    ),
);

// TMP 113 — Coercion (reprint)
const COERCION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::COERCION,
    "2df7b947-bdb2-4204-8eb0-92fe66411613",
    "Pete Venters",
);

// TMP 114 — Coffin Queen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COFFIN_QUEEN: CardRecord = CardRecord::new(
    "Coffin Queen",
    "edf8af70-c26d-4b78-aad6-bd51b5afc590",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// TMP 115 — Commander Greven il-Vec
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMANDER_GREVEN_IL_VEC: CardRecord = CardRecord::new(
    "Commander Greven il-Vec",
    "ab0ce69f-a259-4801-9ac3-f6754040434c",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// TMP 116 — Corpse Dance
pub(in crate::card::sets) static CORPSE_DANCE: CardRecord = CardRecord::new(
    "Corpse Dance",
    "76ae81ea-13e3-4ab8-b956-4c7b139a5e9c",
    "Brian Snõddy",
// Shallow Grave that comes back, which is why five mana a turn is a
    // price worth paying: whatever is on top of the graveyard attacks every
    // turn from here, and the card is never spent.
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        abilities::buyback(
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
        AbilityDef::spell(
            "Return the top creature card of your graveyard to the battlefield. That creature gains haste until end of turn. Exile it at the beginning of the next end step.",
            EffectDef::WithZoneMoveResult {
                effect: &const {
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::TopOfGraveyardMatching {
                            player: PlayerRefDef::EffectController,
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                        }),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    )
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
                                        EffectDef::move_to_zone(
                                            EffectRecipientDef::Source,
                                            ZoneKind::Exile,
                                            ZonePlacement::Top,
                                        ),
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
const DARK_BANISHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::DARK_BANISHING,
    "922d6c8b-70ae-4db4-bf26-1904e4906211",
    "John Matson",
);

// TMP 118 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "bf4708e8-2149-4990-987c-2ea55fc6c508",
    "Ken Meyer, Jr.",
);

// TMP 119 — Darkling Stalker
pub(in crate::card::sets) static DARKLING_STALKER: CardRecord = CardRecord::new(
    "Darkling Stalker",
    "4eb883b7-da6a-45c3-9dde-61334a0ddcae",
    "Susan Van Camp",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Shade", "Spirit"], 1, 1).with_abilities(&[
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
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
    ]),
);

// TMP 120 — Dauthi Embrace
pub(in crate::card::sets) static DAUTHI_EMBRACE: CardRecord = CardRecord::new(
    "Dauthi Embrace",
    "7e84bb94-d654-4d69-89d9-0a398a940125",
    "Andrew Robinson",
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "{B}{B}: Target creature gains shadow until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}{B}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::shadow()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TMP 121 — Dauthi Ghoul
pub(in crate::card::sets) static DAUTHI_GHOUL: CardRecord = CardRecord::new(
    "Dauthi Ghoul",
    "0a70778e-8171-4d97-b86c-d4d92b7e7f06",
    "Tom Kyffin",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Dauthi", "Zombie"], 1, 1).with_abilities(&[
        abilities::shadow(),
        abilities::dies_trigger_matching(
            "Whenever a creature with shadow dies, put a +1/+1 counter on this creature.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(KeywordAbility::Shadow),
            ]),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TMP 122 — Dauthi Horror
pub(in crate::card::sets) static DAUTHI_HORROR: CardRecord = CardRecord::new(
    "Dauthi Horror",
    "c5a8bb3a-3a84-442f-8e31-8af2f04408ab",
    "Jeff Laubenstein",
    // Shadow already stops white creatures blocking it, so the second
    // clause only matters against a white creature that also has shadow.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Dauthi", "Horror"], 2, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::static_ability(
            "This creature can't be blocked by white creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Color(ManaColor::White),
                )),
            },
        ),
    ]),
);

// TMP 123 — Dauthi Marauder
pub(in crate::card::sets) static DAUTHI_MARAUDER: CardRecord = CardRecord::new(
    "Dauthi Marauder",
    "ee847d84-ec8d-4ec3-8436-68d6f144e22f",
    "Andrew Robinson",
    // Three unblockable damage a turn for three mana, and no defence at all
    // -- the fastest clock in the cycle.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Dauthi", "Minion"], 3, 1)
        .with_ability(abilities::shadow()),
);

// TMP 124 — Dauthi Mercenary
pub(in crate::card::sets) static DAUTHI_MERCENARY: CardRecord = CardRecord::new(
    "Dauthi Mercenary",
    "c340e779-c648-48fd-a159-174b46f2d1b3",
    "Matthew D. Wilson",
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
    "Dauthi Mindripper",
    "6fb990ee-f027-4c74-a67e-98ada6aa21e4",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// TMP 126 — Dauthi Slayer
pub(in crate::card::sets) static DAUTHI_SLAYER: CardRecord = CardRecord::new(
    "Dauthi Slayer",
    "652ccd79-aefd-4b45-b747-75190da0cfc6",
    "Dermot Power",
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
    "Death Pits of Rath",
    "72122e8f-97ab-495e-aade-5d736c432873",
    "Joel Biske",
    crate::card::CardRules::unsupported(),
);

// TMP 128 — Diabolic Edict
pub(in crate::card::sets) static DIABOLIC_EDICT: CardRecord = CardRecord::new(
    "Diabolic Edict",
    "a2ecf2ee-1e2d-4ab2-8b2c-717c794b09b2",
    "Ron Spencer",
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
pub(in crate::card::sets) static DISTURBED_BURIAL: CardRecord = CardRecord::new(
    "Disturbed Burial",
    "06254b6c-eb22-4ec9-9420-74e9ee15e072",
    "Heather Hudson",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// TMP 130 — Dread of Night
pub(in crate::card::sets) static DREAD_OF_NIGHT: CardRecord = CardRecord::new(
    "Dread of Night",
    "d08586d4-8163-454c-b8d8-c5034c4aee6c",
    "Richard Thomas",
    CardRules::new_enchantment(mana_cost!("{B}")).with_ability(AbilityDef::static_ability(
        "White creatures get -1/-1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
        },
    )),
);

// TMP 131 — Dregs of Sorrow
pub(in crate::card::sets) static DREGS_OF_SORROW: CardRecord = CardRecord::new(
    "Dregs of Sorrow",
    "80e4203a-ff11-4075-a03a-11448779b413",
    "Thomas Gianni",
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
pub(in crate::card::sets) static ENDLESS_SCREAM: CardRecord = CardRecord::new(
    "Endless Scream",
    "e9474231-34c5-4563-8a61-fd1bc2693f86",
    "Joel Biske",
    CardRules::new_enchantment(mana_cost!("{X}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::as_enters(
                "This Aura enters with X scream counters on it.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCastXCounters {
                        kind: CounterKind::named("scream"),
                    },
                ),
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+0 for each scream counter on this Aura.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountersOnSource(CounterKind::named("scream")),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// TMP 133 — Enfeeblement (reprint)
const ENFEEBLEMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ENFEEBLEMENT,
    "71705205-0165-4774-8209-90ce800b9450",
    "D. Alexander Gregory",
);

// TMP 134 — Evincar's Justice
pub(in crate::card::sets) static EVINCAR_S_JUSTICE: CardRecord = CardRecord::new(
    "Evincar's Justice",
    "5d53f46f-b069-4b34-af4b-98143328c078",
    "Hannibal King",
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell(
            "Evincar's Justice deals 2 damage to each creature and each player.",
            EffectDef::damage_simultaneously(&[
                DamageAssignmentDef {
                    source: Some(ObjectRefDef::Source),
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    amount: ValueDef::Constant(2),
                },
                DamageAssignmentDef {
                    source: Some(ObjectRefDef::Source),
                    recipient: EffectRecipientDef::players(PlayerSetDef::All),
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// TMP 135 — Extinction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTINCTION: CardRecord = CardRecord::new(
    "Extinction",
    "a233a244-7f84-4525-b0ce-e10db0a95385",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// TMP 136 — Fevered Convulsions
pub(in crate::card::sets) static FEVERED_CONVULSIONS: CardRecord = CardRecord::new(
    "Fevered Convulsions",
    "3a790769-e76e-49e9-9d6d-05ce8e858243",
    "Jeff Miracola",
    CardRules::new_enchantment(mana_cost!("{B}{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{B}{B}: Put a -1/-1 counter on target creature.",
            &[CostDef::Mana(mana_cost!("{2}{B}{B}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 137 — Gravedigger (reprint)
const GRAVEDIGGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::GRAVEDIGGER,
    "872f3328-c65d-49eb-a1bb-ca40e9c05627",
    "Dermot Power",
);

// TMP 138 — Imps' Taunt
pub(in crate::card::sets) static IMPS_TAUNT: CardRecord = CardRecord::new(
    "Imps' Taunt",
    "79d6ed64-9f5c-4233-85a9-028b8e5949c3",
    "Colin MacNeil",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Target creature attacks this turn if able.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(
                    &const {
                        abilities::attacks_each_combat_if_able()
                            .override_text("This creature attacks this turn if able.")
                    },
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 139 — Kezzerdrix
pub(in crate::card::sets) static KEZZERDRIX: CardRecord = CardRecord::new(
    "Kezzerdrix",
    "23b95d3a-bb19-474d-9939-8817038fe9fc",
    "Matthew D. Wilson",
CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Rabbit", "Beast"], 4, 4)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, if your opponents control no creatures, this creature deals 4 damage to you.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                    comparison: ComparisonDef::Equal,
                    amount: 0,
                },
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(4)),
            ),
        ]),
);

// TMP 140 — Knight of Dusk
pub(in crate::card::sets) static KNIGHT_OF_DUSK: CardRecord = CardRecord::new(
    "Knight of Dusk",
    "8aba09c4-9259-4743-9e4a-a63505f1efe6",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Knight"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{B}{B}: Destroy target creature blocking this creature.",
            &[CostDef::Mana(mana_cost!("{B}{B}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::BlockingSource,
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// TMP 141 — Leeching Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEECHING_LICID: CardRecord = CardRecord::new(
    "Leeching Licid",
    "27bffefb-23c0-4d03-b716-b1a7eff39a05",
    "Joel Biske",
    crate::card::CardRules::unsupported(),
);

// TMP 142 — Living Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_DEATH: CardRecord = CardRecord::new(
    "Living Death",
    "6c820476-fbda-4073-baf6-51e71f45ed58",
    "Charles Gillespie",
    crate::card::CardRules::unsupported(),
);

// TMP 143 — Maddening Imp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MADDENING_IMP: CardRecord = CardRecord::new(
    "Maddening Imp",
    "dda08eb5-c75c-4c21-bfd1-1f04a3575241",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// TMP 144 — Marsh Lurker
pub(in crate::card::sets) static MARSH_LURKER: CardRecord = CardRecord::new(
    "Marsh Lurker",
    "90c4b759-f53d-4977-8d97-a93762622e75",
    "Tom Kyffin",
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
pub(in crate::card::sets) static MINDWHIP_SLIVER: CardRecord = CardRecord::new(
    "Mindwhip Sliver",
    "fa966fbb-140d-4057-a4fc-998ebe07c307",
    "Jeff Miracola",
CardRules::new_creature(mana_cost!("{2}{B}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Slivers have \"{2}, Sacrifice this permanent: Target player discards a card at random. Activate only as a sorcery.\"",
            all_slivers_get(AppliedEffectDef::add_ability(&const {
                AbilityDef::activated_with_targets(
                    "{2}, Sacrifice this permanent: Target player discards a card at random. Activate only as a sorcery.",
                    &[
                        CostDef::Mana(mana_cost!("{2}")),
                        CostDef::SacrificeSource,
                    ],
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Player(PlayerRelation::Any),
                        )]
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::Random,
                        then: None,
                    },
                )
                .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            })),
        ),
    ),
);

// TMP 146 — Minion of the Wastes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_THE_WASTES: CardRecord = CardRecord::new(
    "Minion of the Wastes",
    "d9f120fc-c681-47b6-827e-1cc7ead47a0f",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// TMP 147 — Perish
pub(in crate::card::sets) static PERISH: CardRecord = CardRecord::new(
    "Perish",
    "e47ace1d-73de-44aa-a3fe-2e2a21ebec79",
    "Rebecca Guay",
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
    "Pit Imp",
    "24c7acfe-b5b2-426f-a5a1-1ff8ef7ebf72",
    "Phil Foglio",
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

// TMP 149 — Rain of Tears (reprint)
const RAIN_OF_TEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::RAIN_OF_TEARS,
    "cad93919-273f-4a26-8ebd-13503dd6b220",
    "Charles Gillespie",
);

// TMP 150 — Rats of Rath
pub(in crate::card::sets) static RATS_OF_RATH: CardRecord = CardRecord::new(
    "Rats of Rath",
    "7cb8d3a2-ed96-4490-9432-401da19ad3c5",
    "John Matson",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{B}: Destroy target artifact, creature, or land you control.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// TMP 151 — Reanimate
pub(in crate::card::sets) static REANIMATE: CardRecord = CardRecord::new(
    "Reanimate",
    "ae1ef31c-8ca5-444c-8f39-e1d1827318f5",
    "Robert Bliss",
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
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
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
    "Reckless Spite",
    "9141daea-1f4f-4227-b7d7-20753e3cb4d4",
    "Pete Venters",
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
pub(in crate::card::sets) static SADISTIC_GLEE: CardRecord = CardRecord::new(
    "Sadistic Glee",
    "d9e1959c-b87b-4e17-a0d2-0489ea79220b",
    "Pete Venters",
    // Every dead creature on either side makes it permanently bigger, which
    // is a one-mana threat in a format of trades.
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::dies_trigger_matching(
                "Whenever a creature dies, put a +1/+1 counter on enchanted creature.",
                ObjectPredicateDef::HasType(CardType::Creature),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// TMP 154 — Sarcomancy
pub(in crate::card::sets) static SARCOMANCY: CardRecord = CardRecord::new(
    "Sarcomancy",
    "eb5730f5-a44c-4f75-a26f-90815cfcd31e",
    "Daren Bader",
CardRules::new_enchantment(mana_cost!("{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a 2/2 black Zombie creature token.",
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2),
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if there are no Zombies on the battlefield, this enchantment deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
        ),
    ]),
);

// TMP 155 — Screeching Harpy
pub(in crate::card::sets) static SCREECHING_HARPY: CardRecord = CardRecord::new(
    "Screeching Harpy",
    "10c02902-4e3a-445e-9dd9-116806ddc966",
    "Una Fricker",
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
pub(in crate::card::sets) static SERVANT_OF_VOLRATH: CardRecord = CardRecord::new(
    "Servant of Volrath",
    "691afabb-f266-45fd-b5a3-577be4f10f86",
    "Brian Snõddy",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Minion"], 3, 3).with_ability(
        AbilityDef::triggered(
            "When this creature leaves the battlefield, sacrifice a creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ),
);

// TMP 157 — Skyshroud Vampire
pub(in crate::card::sets) static SKYSHROUD_VAMPIRE: CardRecord = CardRecord::new(
    "Skyshroud Vampire",
    "eed2c97b-f003-436c-9faa-5518aba42fc1",
    "Gary Leach",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Vampire"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "Discard a creature card: This creature gets +2/+2 until end of turn.",
            &[CostDef::discard(ObjectPredicateDef::HasType(
                CardType::Creature,
            ))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 158 — Souldrinker
pub(in crate::card::sets) static SOULDRINKER: CardRecord = CardRecord::new(
    "Souldrinker",
    "07d2d0ff-e44e-427a-9d68-3ed2d51b1b86",
    "Dermot Power",
    // Life into permanent size, which a deck already ahead on life can keep
    // paying until the creature is unanswerable.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 2).with_ability(
        AbilityDef::activated(
            "Pay 3 life: Put a +1/+1 counter on this creature.",
            &[CostDef::PayLife(3)],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 159 — Spinal Graft
pub(in crate::card::sets) static SPINAL_GRAFT: CardRecord = CardRecord::new(
    "Spinal Graft",
    "3a073b0c-2309-4070-a18e-0937ec8d4d1c",
    "Ron Spencer",
CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature becomes the target of a spell or ability, destroy that creature. It can't be regenerated.",
                TriggerEventDef::becomes_targeted(ObjectPredicateDef::AttachedToSource),
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &EffectDef::Destroy {
                        object: EffectRecipientDef::TriggeringObject,
                        then: None,
                    },
                },
            ),
        ]),
);

// TMP 160 — Aftershock
pub(in crate::card::sets) static AFTERSHOCK: CardRecord = CardRecord::new(
    "Aftershock",
    "c91a26b2-03f8-43f0-a3a4-ff6c5a3690c4",
    "Hannibal King",
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
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(3)),
        ]),
    )),
);

// TMP 161 — Ancient Runes
pub(in crate::card::sets) static ANCIENT_RUNES: CardRecord = CardRecord::new(
    "Ancient Runes",
    "e315c1c2-436e-48ff-9214-938697178393",
    "Susan Van Camp",
CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::triggered(
        "At the beginning of each player's upkeep, this enchantment deals damage to that player equal to the number of artifacts they control.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::damage(
            EffectRecipientDef::EventPlayer,
            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::EventPlayer,
            )),
        ),
    )),
);

// TMP 162 — Apocalypse
pub(in crate::card::sets) static APOCALYPSE: CardRecord = CardRecord::new(
    "Apocalypse",
    "7ff23780-d183-4cca-ad0c-448ef325bf36",
    "Allen Williams",
    // Everything goes, including your hand. It is a reset button rather
    // than a play, which is why it costs five.
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}{R}")).with_ability(AbilityDef::spell(
        "Exile all permanents. You discard your hand.",
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
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
    "Barbed Sliver",
    "19bddea7-daa7-4bdb-9b91-f7fcbc0d7a57",
    "Scott Kirschner",
    // The aggressive half: spare mana becomes damage on every Sliver at
    // once.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have \"{2}: This creature gets +1/+0 until end of turn.\"",
            all_slivers_get(AppliedEffectDef::add_ability(
                &const {
                    abilities::apply_to_self_until_end_of_turn(
                        "{2}: This creature gets +1/+0 until end of turn.",
                        &[CostDef::Mana(mana_cost!("{2}"))],
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                    )
                },
            )),
        ),
    ),
);

// TMP 164 — Blood Frenzy
pub(in crate::card::sets) static BLOOD_FRENZY: CardRecord = CardRecord::new(
    "Blood Frenzy",
    "06f39d83-e1ea-45c9-8181-2a2b6e5148da",
    "Paolo Parente",
CardRules::new_instant(mana_cost!("{1}{R}"))
        .cast_only_before_combat_damage()
        .with_abilities(&[
            AbilityDef::enforced_when_cast(
                "Cast this spell only before the combat damage step.",
                "The play option refuses the cast from the combat damage step onward.",
            ),
            AbilityDef::spell_with_targets(
                "Target attacking or blocking creature gets +4/+0 until end of turn. Destroy that creature at the beginning of the next end step.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::AttackingOrBlocking,
                    ]),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, destroy that creature.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            then: None,
                        },
                    ))),
                ]),
            ),
        ]),
);

// TMP 165 — Boil
pub(in crate::card::sets) static BOIL: CardRecord = CardRecord::new(
    "Boil",
    "2fa1c529-44e5-41b6-9704-ae2319f31f13",
    "Jason Alexander Behnke",
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
pub(in crate::card::sets) static CANYON_DRAKE: CardRecord = CardRecord::new(
    "Canyon Drake",
    "22f84143-5912-43ca-a274-f26ed0dbadd0",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Drake"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}, Discard a card at random: This creature gets +2/+0 until end of turn.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::DiscardCardsAtRandom(1),
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 167 — Canyon Wildcat
pub(in crate::card::sets) static CANYON_WILDCAT: CardRecord = CardRecord::new(
    "Canyon Wildcat",
    "0169e52b-7909-4a8f-8ca2-62f030f9a85a",
    "Gary Leach",
    // Two damage a turn that the red mirror cannot block.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Cat"], 2, 1)
        .with_ability(abilities::landwalk(BasicLandType::Mountain)),
);

// TMP 168 — Chaotic Goo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOTIC_GOO: CardRecord = CardRecord::new(
    "Chaotic Goo",
    "0e9881a4-3078-4fe0-be09-54ddad1d18a0",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// TMP 169 — Crown of Flames
pub(in crate::card::sets) static CROWN_OF_FLAMES: CardRecord = CardRecord::new(
    "Crown of Flames",
    "f2c82741-2869-41f9-82f4-6ed88756e2fd",
    "William O'Connor",
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::activated(
                "{R}: Enchanted creature gets +1/+0 until end of turn.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{R}: Return this Aura to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TMP 170 — Deadshot
pub(in crate::card::sets) static DEADSHOT: CardRecord = CardRecord::new(
    "Deadshot",
    "55d212c5-1642-456f-829f-57f68a2116b6",
    "Heather Hudson",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target creature. It deals damage equal to its power to another target creature.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            ))
            .another(),
        ],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ]),
    )),
);

// TMP 171 — Enraging Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENRAGING_LICID: CardRecord = CardRecord::new(
    "Enraging Licid",
    "fb7bff44-36e1-4855-aa2d-5c7bd6bf6f10",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// TMP 172 — Firefly
pub(in crate::card::sets) static FIREFLY: CardRecord = CardRecord::new(
    "Firefly",
    "a312f0cf-225a-4f3d-b9a7-c47dd03b25c3",
    "Stephen Daniele",
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
    "Fireslinger",
    "de253d94-9968-47da-bb7a-9c8ebf50f4e0",
    "Jeff Reitz",
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
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
                // Damage rather than life loss, so it can be prevented and
                // the Fireslinger's controller is a legal damage recipient.
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
            ]),
        ),
    ),
);

// TMP 174 — Flowstone Giant
pub(in crate::card::sets) static FLOWSTONE_GIANT: CardRecord = CardRecord::new(
    "Flowstone Giant",
    "46e8240a-d882-4f60-8960-1856284e04a0",
    "Joel Biske",
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
pub(in crate::card::sets) static FLOWSTONE_SALAMANDER: CardRecord = CardRecord::new(
    "Flowstone Salamander",
    "bf5b6749-42a6-498b-8908-b28d1749dea6",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Salamander"], 3, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{R}: This creature deals 1 damage to target creature blocking it.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::BlockingSource,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 176 — Flowstone Wyvern
pub(in crate::card::sets) static FLOWSTONE_WYVERN: CardRecord = CardRecord::new(
    "Flowstone Wyvern",
    "ee7949c7-ab80-46a1-9cf7-d8e8c004df6e",
    "Stephen Daniele",
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
    "Furnace of Rath",
    "606abea6-109a-4dd5-99cf-0d5ce492d7f0",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// TMP 178 — Giant Strength (reprint)
const GIANT_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GIANT_STRENGTH,
    "bfb43289-35dc-4983-9c49-22b1b3a7d85a",
    "Pete Venters",
);

// TMP 179 — Goblin Bombardment
pub(in crate::card::sets) static GOBLIN_BOMBARDMENT: CardRecord = CardRecord::new(
    "Goblin Bombardment",
    "179e954f-1d90-4ef4-b800-25845cc338e2",
    "Brian Snõddy",
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
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 180 — Hand to Hand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAND_TO_HAND: CardRecord = CardRecord::new(
    "Hand to Hand",
    "af4cb86a-db01-4d9a-99e9-bb50ce23507f",
    "Carl Frank",
    crate::card::CardRules::unsupported(),
);

// TMP 181 — Havoc
pub(in crate::card::sets) static HAVOC: CardRecord = CardRecord::new(
    "Havoc",
    "d7b032a1-6e43-4e22-9efa-43cfbf211e1c",
    "Donato Giancola",
    // Two life a spell is a real clock against a deck that has to cast four
    // things to stabilise, and nothing at all against anyone else.
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a white spell, they lose 2 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::Color(ManaColor::White),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(2),
        },
    )),
);

// TMP 182 — Heart Sliver
pub(in crate::card::sets) static HEART_SLIVER: CardRecord = CardRecord::new(
    "Heart Sliver",
    "27a83ab6-0d15-49e4-90e3-b3a2a095c632",
    "Ron Spencer",
    // Haste for the tribe, so every Sliver drawn later attacks the turn it
    // arrives.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have haste.",
            all_slivers_get(AppliedEffectDef::add_ability(&const { abilities::haste() })),
        ),
    ),
);

// TMP 183 — Jackal Pup
pub(in crate::card::sets) static JACKAL_PUP: CardRecord = CardRecord::new(
    "Jackal Pup",
    "3707ab74-9aec-4d30-86e0-ffa5f72d5b4f",
    "Susan Van Camp",
    CardRules::new_creature(mana_cost!("{R}"), &["Jackal"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, it deals that much damage to you.",
            TriggerEventDef::damage_to_source(),
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::TriggerEventAmount),
        ),
    ),
);

// TMP 184 — Kindle
pub(in crate::card::sets) static KINDLE: CardRecord = CardRecord::new(
    "Kindle",
    "930745eb-b038-4b55-97f3-bf8d99b54d32",
    "Donato Giancola",
CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Kindle deals X damage to any target, where X is 2 plus the number of cards named Kindle in all graveyards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(2),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::NameEquals(CardNameDef::Literal("Kindle")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::Any,
                    )),
                )),
            ),
        ),
    ),
);

// TMP 185 — Lightning Blast
pub(in crate::card::sets) static LIGHTNING_BLAST: CardRecord = CardRecord::new(
    "Lightning Blast",
    "63fec3f9-d399-48e6-84b6-c8410c24c382",
    "Richard Thomas",
    // Four damage at instant speed for four mana: the unexciting rate that
    // every red set reprints in some form.
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Lightning Blast deals 4 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(4),
        ),
    )]),
);

// TMP 186 — Lightning Elemental
pub(in crate::card::sets) static LIGHTNING_ELEMENTAL: CardRecord = CardRecord::new(
    "Lightning Elemental",
    "11f6d2a4-cc97-43f3-a8b2-f96262c27371",
    "D. Alexander Gregory",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 4, 1)
        .with_abilities(&[abilities::haste()]),
);

// TMP 187 — Lowland Giant
pub(in crate::card::sets) static LOWLAND_GIANT: CardRecord = CardRecord::new(
    "Lowland Giant",
    "7398dec7-5e60-43c0-81a0-ab49beb37077",
    "Paolo Parente",
    // A vanilla 4/3 for four, which is the rate red paid for raw size.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 4, 3),
);

// TMP 188 — Magmasaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMASAUR: CardRecord = CardRecord::new(
    "Magmasaur",
    "48115601-fa00-4d33-8205-faac02997bb4",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// TMP 189 — Mogg Conscripts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_CONSCRIPTS: CardRecord = CardRecord::new(
    "Mogg Conscripts",
    "73719325-b091-4464-b0b0-77dfbb19562a",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// TMP 190 — Mogg Fanatic
pub(in crate::card::sets) static MOGG_FANATIC: CardRecord = CardRecord::new(
    "Mogg Fanatic",
    "ca2ecfd4-c874-4468-8601-87aa110d5a00",
    "Brom",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: It deals 1 damage to any target.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 191 — Mogg Raider
pub(in crate::card::sets) static MOGG_RAIDER: CardRecord = CardRecord::new(
    "Mogg Raider",
    "94e9cc0a-c210-4525-8c7f-9c6306cc21b0",
    "Brian Snõddy",
    // A free sacrifice outlet stapled to a combat trick, and the reason
    // every Goblin deck could turn a chump block into a kill.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a Goblin: Target creature gets +1/+1 until end of turn.",
            // "A Goblin", so it can eat itself, which is what makes it a
            // free sacrifice outlet as well as a combat trick.
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
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
    "Mogg Squad",
    "4b267071-42a6-4e25-9c92-5bca32f8d9af",
    "Joel Biske",
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
    "No Quarter",
    "7c317ee7-ed92-4e3e-92bb-502099caccf8",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// TMP 194 — Opportunist
pub(in crate::card::sets) static OPPORTUNIST: CardRecord = CardRecord::new(
    "Opportunist",
    "ae0026c0-8f61-4485-8909-6a44c2ca9169",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target creature that was dealt damage this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::WasDealtDamageThisTurn,
                ]),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 195 — Pallimud
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PALLIMUD: CardRecord = CardRecord::new(
    "Pallimud",
    "61adc314-cfb2-4fdd-925c-cc1dc4692992",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// TMP 196 — Rathi Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_DRAGON: CardRecord = CardRecord::new(
    "Rathi Dragon",
    "7df61bff-a459-4ddb-a084-f47859a43795",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// TMP 197 — Renegade Warlord
pub(in crate::card::sets) static RENEGADE_WARLORD: CardRecord = CardRecord::new(
    "Renegade Warlord",
    "a69ea676-60ba-4807-bc9b-976bf5666485",
    "Ron Spencer",
CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Soldier"], 3, 3)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::triggered(
                "Whenever this creature attacks, each other attacking creature gets +1/+0 until end of turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// TMP 198 — Rolling Thunder
pub(in crate::card::sets) static ROLLING_THUNDER: CardRecord = CardRecord::new(
    "Rolling Thunder",
    "0bb07402-d526-4938-89a3-9174d5b5a4de",
    "Richard Thomas",
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
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::DividedAmongTargets,
        ),
    )),
);

// TMP 199 — Sandstone Warrior
pub(in crate::card::sets) static SANDSTONE_WARRIOR: CardRecord = CardRecord::new(
    "Sandstone Warrior",
    "eaa61413-3c6a-4895-b8e7-2723e273a952",
    "Stephen Daniele",
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
pub(in crate::card::sets) static SCORCHED_EARTH: CardRecord = CardRecord::new(
    "Scorched Earth",
    "e6a97817-d1fd-4ba4-9ced-c2702b081523",
    "Nicola Leonard",
CardRules::new_sorcery(mana_cost!("{X}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard X land cards.\nDestroy X target lands.",
            &[AbilityTargetDef::exactly_chosen_x(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            CostDef::discard(ObjectPredicateDef::HasType(CardType::Land)).with_quantity(CostQuantityDef::ChosenX),
            EffectDef::Destroy {
                object: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// TMP 201 — Searing Touch
pub(in crate::card::sets) static SEARING_TOUCH: CardRecord = CardRecord::new(
    "Searing Touch",
    "e9091667-d5a8-4978-9023-032ff65f9642",
    "D. Alexander Gregory",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{4}"))]),
        AbilityDef::spell_with_targets(
            "Searing Touch deals 1 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// TMP 202 — Shadowstorm
pub(in crate::card::sets) static SHADOWSTORM: CardRecord = CardRecord::new(
    "Shadowstorm",
    "367c4ad6-973d-47ba-9431-312f9f2996f6",
    "Adam Rex",
    // A sideboard card in spell form: worthless against anything that is
    // not the one deck it answers.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell(
        "Shadowstorm deals 2 damage to each creature with shadow.",
        EffectDef::damage(
            EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Shadow),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            ValueDef::Constant(2),
        ),
    )),
);

// TMP 203 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "5fa05258-2ce3-4604-938c-e8c3fb8cf142",
    "Jason Alexander Behnke",
);

// TMP 204 — Shocker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOCKER: CardRecord = CardRecord::new(
    "Shocker",
    "90954848-af7e-47b3-82e7-9fedde6ad606",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// TMP 205 — Starke of Rath
pub(in crate::card::sets) static STARKE_OF_RATH: CardRecord = CardRecord::new(
    "Starke of Rath",
    "de398b32-10a3-45aa-9886-76806e1602c6",
    "Dan Frazier",
CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Rogue"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Destroy target artifact or creature. That permanent's controller gains control of Starke.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::gain_control(
                    EffectRecipientDef::Source,
                    PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    crate::card::ControlDurationDef::Indefinitely,
                ),
            ]),
        )),
);

// TMP 206 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "f92548d9-cba6-495f-bef1-73e4519fe336",
    "Christopher Rush",
);

// TMP 207 — Stun
pub(in crate::card::sets) static STUN: CardRecord = CardRecord::new(
    "Stun",
    "c09c0da6-37a7-42ba-b264-18898ee372f0",
    "Terese Nielsen",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature can't block this turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// TMP 208 — Sudden Impact
pub(in crate::card::sets) static SUDDEN_IMPACT: CardRecord = CardRecord::new(
    "Sudden Impact",
    "1d7fd516-4b0d-407f-b0c2-d656ad160b8d",
    "Alan Pollack",
CardRules::new_instant(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Sudden Impact deals damage to target player equal to the number of cards in that player's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                )),
            ),
        ),
    ),
);

// TMP 209 — Tahngarth's Rage
pub(in crate::card::sets) static TAHNGARTH_S_RAGE: CardRecord = CardRecord::new(
    "Tahngarth's Rage",
    "36658368-88c8-4c55-8147-f6e581f6af36",
    "Hannibal King",
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+0 as long as it's attacking.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::Attacking,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(0),
                        ),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Otherwise, enchanted creature gets -2/-1.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-2),
                            ValueDef::Constant(-1),
                        ),
                    },
                },
            ),
        ]),
);

// TMP 210 — Tooth and Claw
pub(in crate::card::sets) static TOOTH_AND_CLAW: CardRecord = CardRecord::new(
    "Tooth and Claw",
    "71696093-0867-4889-8fb4-56fa143f9b27",
    "Val Mayerik",
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(AbilityDef::activated(
        "Sacrifice two creatures: Create a 3/1 red Beast creature token named Carnivore.",
        &[CostDef::SacrificePermanents {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            controller: PlayerRelation::You,
            count: 2,
        }],
        EffectDef::create_creature_token(&["Beast"], &[ManaColor::Red], 3, 1)
            .with_name("Carnivore"),
    )),
);

// TMP 211 — Wall of Diffusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_DIFFUSION: CardRecord = CardRecord::new(
    "Wall of Diffusion",
    "2e6a469e-3d67-4edf-a735-03dcd626f858",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// TMP 212 — Wild Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_WURM: CardRecord = CardRecord::new(
    "Wild Wurm",
    "e5a1b213-cba0-4eca-b058-93f4fde717c8",
    "Randy Elliott",
    crate::card::CardRules::unsupported(),
);

// TMP 213 — Aluren
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALUREN: CardRecord = CardRecord::new(
    "Aluren",
    "268403bc-733d-446e-a7c1-abc957c42bc2",
    "April Lee",
    crate::card::CardRules::unsupported(),
);

// TMP 214 — Apes of Rath
pub(in crate::card::sets) static APES_OF_RATH: CardRecord = CardRecord::new(
    "Apes of Rath",
    "25eff287-6b53-4e6d-9da2-d80d05bb8c51",
    "Jeff Laubenstein",
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
pub(in crate::card::sets) static BAYOU_DRAGONFLY: CardRecord = CardRecord::new(
    "Bayou Dragonfly",
    "93cfcca5-070b-4946-b17b-0c94b1e47fcd",
    "DiTerlizzi",
    // Two kinds of evasion on a one-power body, which only matters against
    // the deck that has both Swamps and no fliers.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::landwalk(BasicLandType::Swamp),
    ]),
);

// TMP 216 — Broken Fall
pub(in crate::card::sets) static BROKEN_FALL: CardRecord = CardRecord::new(
    "Broken Fall",
    "d43f5349-a3f8-492d-a835-24a9f948741c",
    "Zina Saunders",
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_ability(
        AbilityDef::activated_with_targets(
            "Return this enchantment to its owner's hand: Regenerate target creature.",
            &[CostDef::ReturnSourceToHand],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// TMP 217 — Canopy Spider
pub(in crate::card::sets) static CANOPY_SPIDER: CardRecord = CardRecord::new(
    "Canopy Spider",
    "afc114b0-2e95-4143-a4b6-6537813946e7",
    "Christopher Rush",
    // The cheapest way green answers a two-mana flier, and the reason every
    // limited deck wanted one.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider"], 1, 3)
        .with_ability(abilities::reach()),
);

// TMP 218 — Charging Rhino (reprint)
const CHARGING_RHINO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::CHARGING_RHINO,
    "651f89e5-9ce2-4713-aca9-6581005f6ca2",
    "Daren Bader",
);

// TMP 219 — Choke
pub(in crate::card::sets) static CHOKE: CardRecord = CardRecord::new(
    "Choke",
    "e2f85205-3c4f-4411-b09c-d1271be56dde",
    "Terese Nielsen",
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_ability(AbilityDef::static_ability(
        "Islands don't untap during their controllers' untap steps.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
        },
    )),
);

// TMP 220 — Crazed Armodon
pub(in crate::card::sets) static CRAZED_ARMODON: CardRecord = CardRecord::new(
    "Crazed Armodon",
    "b83e4b36-57c1-493d-ab79-52075990b2d5",
    "Gary Leach",
CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elephant"], 3, 3).with_ability(
        AbilityDef::activated(
            "{G}: This creature gets +3/+0 and gains trample until end of turn. Destroy this creature at the beginning of the next end step. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next end step, destroy this creature.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Source,
                        then: None,
                    },
                ))),
            ]),
        )
        .once_each_turn(),
    ),
);

// TMP 221 — Dirtcowl Wurm
pub(in crate::card::sets) static DIRTCOWL_WURM: CardRecord = CardRecord::new(
    "Dirtcowl Wurm",
    "a9e2df7d-5d72-4a32-a453-6d8611f0d63c",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Wurm"], 3, 4).with_ability(
        AbilityDef::triggered(
            "Whenever an opponent plays a land, put a +1/+1 counter on this creature.",
            TriggerEventDef::LandPlayed {
                land: ObjectPredicateDef::Any,
                player: PlayerRelation::Opponent,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 222 — Earthcraft
pub(in crate::card::sets) static EARTHCRAFT: CardRecord = CardRecord::new(
    "Earthcraft",
    "9dda7531-82a1-4f49-8858-601ddbc6e2bc",
    "Randy Gallegos",
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
    "Eladamri's Vineyard",
    "d8531643-5657-44b6-89d1-9cdf67ed09c4",
    "Ron Chironna",
    crate::card::CardRules::unsupported(),
);

// TMP 224 — Eladamri, Lord of Leaves
pub(in crate::card::sets) static ELADAMRI_LORD_OF_LEAVES: CardRecord = CardRecord::new(
    "Eladamri, Lord of Leaves",
    "0b1689f3-9dfa-4525-90b3-7af15f7eb720",
    "Ron Chironna",
    // Shroud on the whole tribe is what makes an Elf deck immune to the
    // removal that would otherwise answer its one important creature.
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Elf", "Warrior"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Elf creatures have forestwalk.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(&const { abilities::forestwalk() }),
                },
            ),
            AbilityDef::static_ability(
                "Other Elves have shroud.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(&const { abilities::shroud() }),
                },
            ),
        ]),
);

// TMP 225 — Elven Warhounds
pub(in crate::card::sets) static ELVEN_WARHOUNDS: CardRecord = CardRecord::new(
    "Elven Warhounds",
    "29138c1e-11cb-488f-8e04-f5488e08a81e",
    "Kev Walker",
    // Not removal but close enough: the blocker goes on top of its library, so
    // the defender spends their next draw buying it back.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Dog"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a creature, put that creature on top of \
             its owner's library.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::move_to_zone(
                EffectRecipientDef::TriggeringObject,
                ZoneKind::Library,
                ZonePlacement::Top,
            ),
        ),
    ),
);

// TMP 226 — Elvish Fury
pub(in crate::card::sets) static ELVISH_FURY: CardRecord = CardRecord::new(
    "Elvish Fury",
    "f99c10b5-b93b-40c3-936c-d1b81b49c5a4",
    "Quinton Hoover",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{4}"))]),
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.",
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
    ]),
);

// TMP 227 — Flailing Drake
pub(in crate::card::sets) static FLAILING_DRAKE: CardRecord = CardRecord::new(
    "Flailing Drake",
    "43d246ac-1ca5-4c55-856c-4a83a4d638ab",
    "Heather Hudson",
CardRules::new_creature(mana_cost!("{3}{G}"), &["Drake"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a creature, that creature gets +1/+1 until end of turn.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TMP 228 — Frog Tongue
pub(in crate::card::sets) static FROG_TONGUE: CardRecord = CardRecord::new(
    "Frog Tongue",
    "3941e799-a254-423e-90bb-091dbe56ca6a",
    "Phil Foglio",
    // It replaces itself, so the reach is free -- which is the only way a
    // one-mana Aura is worth a card.
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has reach.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&const { abilities::reach() }),
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// TMP 229 — Fugitive Druid
pub(in crate::card::sets) static FUGITIVE_DRUID: CardRecord = CardRecord::new(
    "Fugitive Druid",
    "afe165cd-8ef7-408e-ae56-3c6a0cc4e409",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Human", "Druid"], 3, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes the target of an Aura spell, you draw a card.",
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")),
            ])),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 230 — Harrow
pub(in crate::card::sets) static HARROW: CardRecord = CardRecord::new(
    "Harrow",
    "3c207142-4880-4935-9827-b91bc7d9d643",
    "Eric David Anderson",
    // Two lands for one at instant speed: the land count stays level while the
    // colours it can produce do not.
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a land.\nSearch your library \
             for up to two basic land cards, put them onto the battlefield, then shuffle.",
            &[],
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Land),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(
                    &const {
                        [
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]
                    },
                ),
                minimum: 0,
                maximum: ValueDef::Constant(2),
                reveal: false,
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

// TMP 231 — Heartwood Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTWOOD_DRYAD: CardRecord = CardRecord::new(
    "Heartwood Dryad",
    "e2b9a001-2a1e-4fc4-9b84-c776f741a858",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// TMP 232 — Heartwood Giant
pub(in crate::card::sets) static HEARTWOOD_GIANT: CardRecord = CardRecord::new(
    "Heartwood Giant",
    "4baacffe-76d1-4cfb-a047-d6d126bb8de0",
    "Randy Elliott",
CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Giant"], 4, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice a Forest: This creature deals 2 damage to target player or planeswalker.",
            &[
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ),
);

// TMP 233 — Heartwood Treefolk
pub(in crate::card::sets) static HEARTWOOD_TREEFOLK: CardRecord = CardRecord::new(
    "Heartwood Treefolk",
    "de263f02-8e3e-4785-9c06-9adc168994f3",
    "Daren Bader",
    // A 3/4 that green cannot block, which in a format of mirror matches is
    // most of what it does.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Treefolk"], 3, 4)
        .with_ability(abilities::landwalk(BasicLandType::Forest)),
);

// TMP 234 — Horned Sliver
pub(in crate::card::sets) static HORNED_SLIVER: CardRecord = CardRecord::new(
    "Horned Sliver",
    "d0175cec-e64c-45c6-9208-76127e76a7cf",
    "Allen Williams",
    // Trample, which matters exactly when the opponent has Slivers of their
    // own to chump with.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have trample.",
            all_slivers_get(AppliedEffectDef::add_ability(
                &const { abilities::trample() },
            )),
        ),
    ),
);

// TMP 235 — Krakilin
pub(in crate::card::sets) static KRAKILIN: CardRecord = CardRecord::new(
    "Krakilin",
    "a90442e8-9d22-4767-9e08-bd314169ea70",
    "Richard Kane Ferguson",
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
pub(in crate::card::sets) static MIRRI_S_GUILE: CardRecord = CardRecord::new(
    "Mirri's Guile",
    "73d51a3c-95c0-4810-b847-4b8afd12fd64",
    "Brom",
CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, you may look at the top three cards of your library, then put them back in any order.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::EffectController,
                ValueDef::Constant(3),
            ),
        },
    )),
);

// TMP 237 — Mongrel Pack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONGREL_PACK: CardRecord = CardRecord::new(
    "Mongrel Pack",
    "56b84315-5287-4401-a4c8-34c192423270",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// TMP 238 — Muscle Sliver
pub(in crate::card::sets) static MUSCLE_SLIVER: CardRecord = CardRecord::new(
    "Muscle Sliver",
    "602a1e1f-4195-48c0-8290-562e7e0db6d8",
    "Richard Kane Ferguson",
    // The card that made Slivers a deck: every one you play makes every one
    // you already played bigger.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures get +1/+1.",
            all_slivers_get(AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            )),
        ),
    ),
);

// TMP 239 — Natural Spring (reprint)
const NATURAL_SPRING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::NATURAL_SPRING,
    "1ff5d12a-8634-468b-86ca-4ba0f7c013ca",
    "Susan Van Camp",
);

// TMP 240 — Nature's Revolt
pub(in crate::card::sets) static NATURE_S_REVOLT: CardRecord = CardRecord::new(
    "Nature's Revolt",
    "a70386c5-053a-46c4-b26d-c6f92f536bed",
    "Donato Giancola",
    // It animates both mana bases at once, so it belongs in the deck that
    // can kill creatures and does not need its own lands.
    CardRules::new_enchantment(mana_cost!("{3}{G}{G}")).with_ability(AbilityDef::static_ability(
        "All lands are 2/2 creatures that are still lands.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
            ]),
        },
    )),
);

// TMP 241 — Needle Storm (reprint)
const NEEDLE_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::NEEDLE_STORM,
    "be80dd2d-f595-4d80-84ae-66d3d18e7399",
    "Val Mayerik",
);

// TMP 242 — Nurturing Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NURTURING_LICID: CardRecord = CardRecord::new(
    "Nurturing Licid",
    "0bf53069-44ac-49c5-83bf-9c3c1274e407",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// TMP 243 — Overrun
pub(in crate::card::sets) static OVERRUN: CardRecord = CardRecord::new(
    "Overrun",
    "0ad7a961-d3a1-471a-8472-8407d1057de0",
    "Jeff Miracola",
    // Five mana that turns a stalled board into lethal, which is the reason
    // green decks were allowed to have one.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}{G}")).with_ability(AbilityDef::spell(
        "Creatures you control get +3/+3 and gain trample until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                AppliedEffectDef::add_ability(&const { abilities::trample() }),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// TMP 244 — Pincher Beetles
pub(in crate::card::sets) static PINCHER_BEETLES: CardRecord = CardRecord::new(
    "Pincher Beetles",
    "dba68902-1e05-414a-8c3d-1f97da61d09d",
    "Stephen Daniele",
    // A 3/1 nothing can target, which in a removal-heavy format is worth
    // more than the fragile body suggests.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect"], 3, 1)
        .with_ability(abilities::shroud()),
);

// TMP 245 — Rampant Growth (reprint)
const RAMPANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::RAMPANT_GROWTH,
    "3ffbf716-9c3a-45aa-8fdb-632128cc97e2",
    "Tom Kyffin",
);

// TMP 246 — Reality Anchor
pub(in crate::card::sets) static REALITY_ANCHOR: CardRecord = CardRecord::new(
    "Reality Anchor",
    "21204f62-c253-4d88-a4cd-7c0f6f0513e0",
    "Randy Gallegos",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature loses shadow until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Shadow,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// TMP 247 — Reap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REAP: CardRecord = CardRecord::new(
    "Reap",
    "229f8a3d-d1a5-46d7-9b1b-e165397e6579",
    "Ron Chironna",
    crate::card::CardRules::unsupported(),
);

// TMP 248 — Recycle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECYCLE: CardRecord = CardRecord::new(
    "Recycle",
    "ae984b86-ac6d-45e2-9c8d-0b7ac50021a1",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// TMP 249 — Respite
pub(in crate::card::sets) static RESPITE: CardRecord = CardRecord::new(
    "Respite",
    "228a8d29-cc14-49c7-ae24-5847344583ed",
    "Rebecca Guay",
CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn. You gain 1 life for each attacking creature.",
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                )),
            },
        ]),
    )),
);

// TMP 250 — Root Maze
pub(in crate::card::sets) static ROOT_MAZE: CardRecord = CardRecord::new(
    "Root Maze",
    "99a12b74-f191-4362-81ab-77590ae5e68f",
    "Rebecca Guay",
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
    "Rootbreaker Wurm",
    "9a686ed6-fc13-4882-b56c-667f556d9804",
    "Richard Kane Ferguson",
    // Seven mana for a 6/6 trampler, the plain top end of green's curve.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Wurm"], 6, 6)
        .with_ability(abilities::trample()),
);

// TMP 252 — Rootwalla
pub(in crate::card::sets) static ROOTWALLA: CardRecord = CardRecord::new(
    "Rootwalla",
    "03ce4d5d-63cb-47b6-94ce-2063977db9b4",
    "Roger Raupp",
    // The quota is per turn and per permanent, so a second Rootwalla still
    // has its own.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Lizard"], 2, 2).with_ability(
        abilities::apply_to_self_until_end_of_turn(
            "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
            AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
        )
        .once_each_turn(),
    ),
);

// TMP 253 — Scragnoth
pub(in crate::card::sets) static SCRAGNOTH: CardRecord = CardRecord::new(
    "Scragnoth",
    "d80f7fa7-e7c4-4fc4-99bf-8a8502965fc8",
    "Jeff Laubenstein",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 3, 4).with_abilities(&[
        abilities::cannot_be_countered(),
        abilities::protection_from_color(ManaColor::Blue),
    ]),
);

// TMP 254 — Seeker of Skybreak
pub(in crate::card::sets) static SEEKER_OF_SKYBREAK: CardRecord = CardRecord::new(
    "Seeker of Skybreak",
    "b4bb98ba-d599-4597-911c-2b472fa8817c",
    "Daren Bader",
    // Untapping a mana creature is mana, and untapping a blocker is a second
    // block -- the same two mana either way.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Untap target creature.",
            &[CostDef::TapSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// TMP 255 — Skyshroud Elf
// Audit: unsupported — The shared mana-ability runtime does not support a mana cost on the red-or-white ability.
pub(in crate::card::sets) static SKYSHROUD_ELF: CardRecord = CardRecord::new(
    "Skyshroud Elf",
    "26877a52-dec3-433d-b7a5-767f6cdf2365",
    "Jeff Miracola",
    CardRules::unsupported(),
);

// TMP 256 — Skyshroud Ranger
pub(in crate::card::sets) static SKYSHROUD_RANGER: CardRecord = CardRecord::new(
    "Skyshroud Ranger",
    "efe01296-2b8b-4cdf-a041-a08bebea9c29",
    "Steve Luke",
CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Ranger"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}: You may put a land card from your hand onto the battlefield. Activate only as a sorcery.",
            &[CostDef::TapSource],
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: 1,
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// TMP 257 — Skyshroud Troll
pub(in crate::card::sets) static SKYSHROUD_TROLL: CardRecord = CardRecord::new(
    "Skyshroud Troll",
    "925c488d-79db-47d1-b7be-851f31732026",
    "Matthew D. Wilson",
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
    "Spike Drone",
    "5d45a3d3-a114-496e-b575-504179a297cc",
    "Charles Gillespie",
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
pub(in crate::card::sets) static STORM_FRONT: CardRecord = CardRecord::new(
    "Storm Front",
    "994bb02d-6fef-454b-b1b1-d3d1af8dcd1a",
    "William O'Connor",
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::activated_with_targets(
        "{G}{G}: Tap target creature with flying.",
        &[CostDef::Mana(mana_cost!("{G}{G}"))],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            ]),
        )],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// TMP 260 — Trained Armodon
pub(in crate::card::sets) static TRAINED_ARMODON: CardRecord = CardRecord::new(
    "Trained Armodon",
    "2380ab8f-58d2-4e1c-a115-cd2615b5a871",
    "Gary Leach",
    // The vanilla 3/3 for three green kept reprinting, and the yardstick
    // every other three-drop was measured against.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elephant"], 3, 3),
);

// TMP 261 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "160b8060-e755-4a96-9193-b76550d4a6ba",
    "Margaret Organ-Kean",
);

// TMP 262 — Trumpeting Armodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRUMPETING_ARMODON: CardRecord = CardRecord::new(
    "Trumpeting Armodon",
    "38f94fd1-6f85-41ad-9674-f05cc893324f",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// TMP 263 — Verdant Force
pub(in crate::card::sets) static VERDANT_FORCE: CardRecord = CardRecord::new(
    "Verdant Force",
    "29bd094c-fcc1-4abf-ba3e-03a5b9b6d1c2",
    "DiTerlizzi",
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
    "Verdigris",
    "0c79664d-3461-44e7-afe6-33ec54e312ad",
    "Zina Saunders",
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

// TMP 265 — Winter's Grasp (reprint)
const WINTER_S_GRASP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::WINTER_S_GRASP,
    "7af28a5d-45dc-4e31-9009-5c0bd25a9032",
    "Tom Wänerstrand",
);

// TMP 266 — Dracoplasm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRACOPLASM: CardRecord = CardRecord::new(
    "Dracoplasm",
    "3560556c-f1d3-4d69-afe7-c2a5fa2a5c3d",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// TMP 267 — Lobotomy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOBOTOMY: CardRecord = CardRecord::new(
    "Lobotomy",
    "ee7ba92d-d327-4b1c-be40-708c5abb27df",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// TMP 268 — Ranger en-Vec
pub(in crate::card::sets) static RANGER_EN_VEC: CardRecord = CardRecord::new(
    "Ranger en-Vec",
    "4a89e82c-7206-4d74-95c6-ad3627e5a9ce",
    "Randy Elliott",
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
pub(in crate::card::sets) static SEGMENTED_WURM: CardRecord = CardRecord::new(
    "Segmented Wurm",
    "18be7cfa-bf75-407d-b79f-2fec4b1aacf5",
    "Jeff Miracola",
CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Wurm"], 5, 5).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes the target of a spell or ability, put a -1/-1 counter on it.",
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::Any),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 270 — Selenia, Dark Angel
pub(in crate::card::sets) static SELENIA_DARK_ANGEL: CardRecord = CardRecord::new(
    "Selenia, Dark Angel",
    "9c1624f7-8275-46d3-ab7e-7b162e27593f",
    "Matthew D. Wilson",
    CardRules::new_creature(mana_cost!("{3}{W}{B}"), &["Phyrexian", "Angel"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated(
                "Pay 2 life: Return Selenia to its owner's hand.",
                &[CostDef::PayLife(2)],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TMP 271 — Sky Spirit
pub(in crate::card::sets) static SKY_SPIRIT: CardRecord = CardRecord::new(
    "Sky Spirit",
    "eb8efbec-e8bf-4e34-bf13-b43916d2e9ff",
    "Rebecca Guay",
    // Flying and first strike on the same 2/2: it beats every other flier
    // its size in the air and takes nothing back.
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Spirit"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// TMP 272 — Soltari Guerrillas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLTARI_GUERRILLAS: CardRecord = CardRecord::new(
    "Soltari Guerrillas",
    "1b683571-6ac5-4d65-99a6-981755ed4764",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// TMP 273 — Spontaneous Combustion
pub(in crate::card::sets) static SPONTANEOUS_COMBUSTION: CardRecord = CardRecord::new(
    "Spontaneous Combustion",
    "34e6c04f-9d1a-497b-bc96-a0e48a1c1904",
    "Doug Chaffee",
    // A sweeper that costs a creature, so it is one-sided only for the deck
    // whose creatures were already expendable.
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nThis spell deals \
             3 damage to each creature.",
            &[],
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::damage(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::Constant(3),
            ),
        ),
    ),
);

// TMP 274 — Vhati il-Dal
pub(in crate::card::sets) static VHATI_IL_DAL: CardRecord = CardRecord::new(
    "Vhati il-Dal",
    "be535a4a-c00d-4c58-a663-a3419a54da51",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Human", "Warrior"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Until end of turn, target creature has base power 1 or base toughness 1.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::Controller,
                choices: &[
                    EffectChoiceDef {
                        label: "Base power 1",
                        effect: EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::set_base_power(ValueDef::Constant(1)),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    },
                    EffectChoiceDef {
                        label: "Base toughness 1",
                        effect: EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::set_base_toughness(ValueDef::Constant(1)),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    },
                ],
            },
        )),
);

// TMP 275 — Wood Sage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOOD_SAGE: CardRecord = CardRecord::new(
    "Wood Sage",
    "4b073ac1-be1b-49c0-98b4-bf8165e2f872",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// TMP 276 — Altar of Dementia
// Audit: unsupported — Needs the power of a permanent sacrificed as an activation cost. ValueDef reads a cost sacrifice's mana value but not its power, and SacrificeOfChoice's Power follow-up is a resolving effect rather than a cost.
pub(in crate::card::sets) static ALTAR_OF_DEMENTIA: CardRecord = CardRecord::new(
    "Altar of Dementia",
    "4f2da99f-3c53-4980-97d6-2158c765aac0",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// TMP 277 — Booby Trap
pub(in crate::card::sets) static BOOBY_TRAP: CardRecord = CardRecord::new(
    "Booby Trap",
    "bfedc78e-47dc-43e3-aed7-2d5c8e97fdac",
    "Doug Chaffee",
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
                EffectDef::sacrifice(EffectRecipientDef::Source),
                EffectDef::damage(EffectRecipientDef::EventPlayer, ValueDef::Constant(10)),
            ]),
        ),
    ]),
);

// TMP 278 — Bottle Gnomes
pub(in crate::card::sets) static BOTTLE_GNOMES: CardRecord = CardRecord::new(
    "Bottle Gnomes",
    "645297d1-ee77-4879-83eb-8114fbabb9a4",
    "Kaja Foglio",
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
    "Coiled Tinviper",
    "426a28bd-033d-41af-b577-ece73cbd7b3a",
    "John Matson",
    // A colourless first striker, which is what an artifact body was for:
    // any deck could play it.
    CardRules::new_creature(mana_cost!("{3}"), &["Snake"], 2, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// TMP 280 — Cold Storage
pub(in crate::card::sets) static COLD_STORAGE: CardRecord = CardRecord::new(
    "Cold Storage",
    "b26e28d4-50e1-41db-984d-c55781295012",
    "Greg Simanson",
CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}: Exile target creature you control.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
        AbilityDef::activated(
            "Sacrifice this artifact: Return each creature card exiled with this artifact to the battlefield under your control.",
            &[CostDef::SacrificeSource],
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zone: ZoneKind::Battlefield,
                grant: None,
                counters: None,
                transformed: false,
                controller: Some(PlayerRelation::You),
            },
        ),
    ]),
);

// TMP 281 — Cursed Scroll
pub(in crate::card::sets) static CURSED_SCROLL: CardRecord = CardRecord::new(
    "Cursed Scroll",
    "31415b9b-fb30-4132-a9a3-795b4573a901",
    "D. Alexander Gregory",
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
                then: &EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            },
        ]),
    )),
);

// TMP 282 — Echo Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECHO_CHAMBER: CardRecord = CardRecord::new(
    "Echo Chamber",
    "06425615-6c10-4766-8128-a1a09a35649d",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// TMP 283 — Emerald Medallion
pub(in crate::card::sets) static EMERALD_MEDALLION: CardRecord = CardRecord::new(
    "Emerald Medallion",
    "67e87f30-b27a-48e2-a133-192309dd5902",
    "Sue Ellen Brown",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Green spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Green),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 284 — Emmessi Tome
pub(in crate::card::sets) static EMMESSI_TOME: CardRecord = CardRecord::new(
    "Emmessi Tome",
    "a870e48a-41ae-4d9f-b181-074deb067d40",
    "Tom Wänerstrand",
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
    "Energizer",
    "914f204c-7f3d-41f2-a771-0b6227d539eb",
    "Val Mayerik",
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
    "Essence Bottle",
    "7e3c7760-04d8-424d-a097-df0ca8297837",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// TMP 287 — Excavator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCAVATOR: CardRecord = CardRecord::new(
    "Excavator",
    "6dc3d5b4-b04f-4b34-afd2-72fb3de0a33b",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// TMP 288 — Flowstone Sculpture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_SCULPTURE: CardRecord = CardRecord::new(
    "Flowstone Sculpture",
    "7f89f599-b063-4568-b7b9-08b96d04bde1",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// TMP 289 — Fool's Tome
pub(in crate::card::sets) static FOOL_S_TOME: CardRecord = CardRecord::new(
    "Fool's Tome",
    "83be257c-8945-46be-8b58-fb2881084026",
    "Julie Baroh",
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(
        AbilityDef::activated(
            "{2}, {T}: Draw a card. Activate only if you have no cards in hand.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::Any,
                &[ZoneKind::Hand],
                PlayerRelation::You,
            ),
            comparison: ComparisonDef::Equal,
            amount: 0,
        }),
    ),
);

// TMP 290 — Grindstone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRINDSTONE: CardRecord = CardRecord::new(
    "Grindstone",
    "f4459187-de64-456f-bb66-56dea40d5c3e",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// TMP 291 — Helm of Possession
pub(in crate::card::sets) static HELM_OF_POSSESSION: CardRecord = CardRecord::new(
    "Helm of Possession",
    "79e16191-8dca-491b-b892-17696023d581",
    "Janet Aulisio",
CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may choose not to untap this artifact during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}, Sacrifice a creature: Gain control of target creature for as long as you control this artifact and this artifact remains tapped.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::gain_control(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                PlayerRefDef::EffectController,
                crate::card::ControlDurationDef::WhileSourceRemains { while_tapped: true },
            ),
        ),
    ]),
);

// TMP 292 — Jet Medallion
pub(in crate::card::sets) static JET_MEDALLION: CardRecord = CardRecord::new(
    "Jet Medallion",
    "c0db458c-2ced-454c-8061-fff8bd363b33",
    "Sue Ellen Brown",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Black spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Black),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 293 — Jinxed Idol
pub(in crate::card::sets) static JINXED_IDOL: CardRecord = CardRecord::new(
    "Jinxed Idol",
    "0c728e38-5656-4feb-8610-0cf45fb38094",
    "John Matson",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, this artifact deals 2 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(2)),
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice a creature: Target opponent gains control of this artifact.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::gain_control(
                EffectRecipientDef::Source,
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                crate::card::ControlDurationDef::Indefinitely,
            ),
        ),
    ]),
);

// TMP 294 — Lotus Petal
pub(in crate::card::sets) static LOTUS_PETAL: CardRecord = CardRecord::new(
    "Lotus Petal",
    "6c877da3-68fa-41d0-8a24-8c79fcd8ecc1",
    "April Lee",
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_mana(
        "{T}, Sacrifice this artifact: Add one mana of any color.",
        &[CostDef::TapSource, CostDef::SacrificeSource],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// TMP 295 — Magnetic Web
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNETIC_WEB: CardRecord = CardRecord::new(
    "Magnetic Web",
    "9f3c7309-4efb-49ce-a9cc-a8f7b04c1a15",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// TMP 296 — Manakin
pub(in crate::card::sets) static MANAKIN: CardRecord = CardRecord::new(
    "Manakin",
    "3d33ce7f-f318-4161-843a-f5bb6d6e3d29",
    "Scott Kirschner",
    // Two mana for a body that pays one back every turn, which is what a
    // colourless accelerant has to offer to be worth a card.
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 1, 1)
        .with_ability(abilities::tap_for(ManaColor::Colorless)),
);

// TMP 297 — Metallic Sliver
pub(in crate::card::sets) static METALLIC_SLIVER: CardRecord = CardRecord::new(
    "Metallic Sliver",
    "30143f4f-9846-448d-8797-8fe0bc0cc5df",
    "Allen Williams",
    // A 1/1 with nothing on it, printed so that every Sliver lord had one
    // more body to talk to.
    CardRules::new_creature(mana_cost!("{1}"), &["Sliver"], 1, 1),
);

// TMP 298 — Mogg Cannon
pub(in crate::card::sets) static MOGG_CANNON: CardRecord = CardRecord::new(
    "Mogg Cannon",
    "5ee64a77-5308-45c7-b865-400820968c74",
    "Mike Raabe",
CardRules::new_artifact(mana_cost!("{2}")).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature you control gets +1/+0 and gains flying until end of turn. Destroy that creature at the beginning of the next end step.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next end step, destroy that creature.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ))),
            ]),
        ),
    ),
);

// TMP 299 — Patchwork Gnomes
pub(in crate::card::sets) static PATCHWORK_GNOMES: CardRecord = CardRecord::new(
    "Patchwork Gnomes",
    "bdaa9ac4-b742-4a24-a316-97538adfd361",
    "Mike Raabe",
    // A colorless regenerator, so the discard is the only colour
    // requirement and any deck can hold the ground with it.
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Gnome"], 2, 1).with_ability(
        abilities::regenerate_self(
            "Discard a card: Regenerate this creature.",
            &[CostDef::discard(ObjectPredicateDef::Any)],
        ),
    ),
);

// TMP 300 — Pearl Medallion
pub(in crate::card::sets) static PEARL_MEDALLION: CardRecord = CardRecord::new(
    "Pearl Medallion",
    "44588d53-7cce-406a-8e61-cd9866691966",
    "Sue Ellen Brown",
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
    "Phyrexian Grimoire",
    "2fcf5b8a-563b-48d7-a874-e9c226192320",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// TMP 302 — Phyrexian Hulk
pub(in crate::card::sets) static PHYREXIAN_HULK: CardRecord = CardRecord::new(
    "Phyrexian Hulk",
    "307f5d5e-3a4b-430e-8769-fb553216befa",
    "Matthew D. Wilson",
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Phyrexian", "Golem"], 5, 4),
);

// TMP 303 — Phyrexian Splicer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_SPLICER: CardRecord = CardRecord::new(
    "Phyrexian Splicer",
    "b1e9061a-b9e6-49ec-bc7e-c18557da9fd5",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// TMP 304 — Puppet Strings
pub(in crate::card::sets) static PUPPET_STRINGS: CardRecord = CardRecord::new(
    "Puppet Strings",
    "92b55586-9ea3-403e-96ec-2604f91c79cc",
    "Scott Kirschner",
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
    "Ruby Medallion",
    "24cdb28b-85f3-41ae-b1f5-fac766b2dcd2",
    "Sue Ellen Brown",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Red spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Red),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 306 — Sapphire Medallion
pub(in crate::card::sets) static SAPPHIRE_MEDALLION: CardRecord = CardRecord::new(
    "Sapphire Medallion",
    "3ab1e253-47cb-4089-87d5-0f998025d98c",
    "Sue Ellen Brown",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_reduction(
        "Blue spells you cast cost {1} less to cast.",
        ObjectPredicateDef::Color(ManaColor::Blue),
        PlayerRelation::You,
        ValueDef::Constant(1),
    )),
);

// TMP 307 — Scalding Tongs
pub(in crate::card::sets) static SCALDING_TONGS: CardRecord = CardRecord::new(
    "Scalding Tongs",
    "34136f2c-3edd-4ca3-b1ef-1fdcaa4518a0",
    "Randy Gallegos",
CardRules::new_artifact(mana_cost!("{2}")).with_ability(
        AbilityDef::triggered_if_with_targets(
            "At the beginning of your upkeep, if you have three or fewer cards in hand, this artifact deals 1 damage to target opponent or planeswalker.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::LessOrEqual,
                amount: 3,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 308 — Scroll Rack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCROLL_RACK: CardRecord = CardRecord::new(
    "Scroll Rack",
    "5f7346d8-1aef-4618-88e6-74bd8865e0f3",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// TMP 309 — Squee's Toy
pub(in crate::card::sets) static SQUEE_S_TOY: CardRecord = CardRecord::new(
    "Squee's Toy",
    "2b524ae7-cb24-41af-b41b-3cb3ee8cf3b0",
    "Heather Hudson",
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
    "Static Orb",
    "3574d758-9143-4a2b-9ebd-ed8dab238251",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// TMP 311 — Telethopter
pub(in crate::card::sets) static TELETHOPTER: CardRecord = CardRecord::new(
    "Telethopter",
    "77d26c29-cd98-446b-b4e1-687561ed6d3f",
    "Thomas M. Baxa",
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
pub(in crate::card::sets) static THUMBSCREWS: CardRecord = CardRecord::new(
    "Thumbscrews",
    "71025a6b-3aed-4943-a907-9584d135f6c0",
    "Charles Gillespie",
CardRules::new_artifact(mana_cost!("{2}")).with_ability(
        AbilityDef::triggered_if_with_targets(
            "At the beginning of your upkeep, if you have five or more cards in hand, this artifact deals 1 damage to target opponent or planeswalker.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 5,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// TMP 313 — Torture Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORTURE_CHAMBER: CardRecord = CardRecord::new(
    "Torture Chamber",
    "5648158d-38d4-4167-8af5-ee5d7d6fd7cb",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// TMP 314 — Watchdog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATCHDOG: CardRecord = CardRecord::new(
    "Watchdog",
    "8c2ffc07-9993-40de-b36e-33c7afd4cfc2",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// TMP 315 — Ancient Tomb
pub(in crate::card::sets) static ANCIENT_TOMB: CardRecord = CardRecord::new(
    "Ancient Tomb",
    "30e401e3-282b-4524-87e1-c6cd50cd6d00",
    "Colin MacNeil",
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
    "Caldera Lake",
    "7f01fe22-e8ff-4106-8ac5-693ef920b2c9",
    "Allen Williams",
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
    "Cinder Marsh",
    "9fee067d-c31f-4b09-99f5-84d1102f96b0",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// TMP 318 — Ghost Town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOST_TOWN: CardRecord = CardRecord::new(
    "Ghost Town",
    "4218cdda-3a62-43fb-aaf7-7ac836392796",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// TMP 319 — Maze of Shadows
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAZE_OF_SHADOWS: CardRecord = CardRecord::new(
    "Maze of Shadows",
    "ba69c3d3-6fb5-478d-93ba-341dd3ace97d",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// TMP 320 — Mogg Hollows
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static MOGG_HOLLOWS: CardRecord = CardRecord::new(
    "Mogg Hollows",
    "474e3fb0-b0f9-4c6d-8c57-e5079a3a3c66",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// TMP 321 — Pine Barrens
pub(in crate::card::sets) static PINE_BARRENS: CardRecord = CardRecord::new(
    "Pine Barrens",
    "d5ac39e8-bd0e-4fa3-bc1e-a93944d013f3",
    "Rebecca Guay",
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
pub(in crate::card::sets) static REFLECTING_POOL: CardRecord = CardRecord::new(
    "Reflecting Pool",
    "7b633ac2-bf98-42bd-8c26-8737a7d8edc7",
    "Adam Rex",
    // Worth nothing on its own and everything beside four other lands, which
    // is why a five-color deck plays it and nobody else does.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any type that a land you control could produce.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice_from(
            ManaTypeSetDef::could_be_produced_by(&ObjectSetDef::Query(ObjectQueryDef::matching(
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
    "Rootwater Depths",
    "4d4bcbef-66bf-4625-82d5-a01c39d3d78e",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// TMP 324 — Salt Flats
pub(in crate::card::sets) static SALT_FLATS: CardRecord = CardRecord::new(
    "Salt Flats",
    "224cb63f-9af0-4b00-ba0b-0b604abf20c8",
    "Scott Kirschner",
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
    "Scabland",
    "0374f269-b07e-43af-911a-5454b35f14e6",
    "Andrew Robinson",
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
    "Skyshroud Forest",
    "aa01f43b-d0b3-4cd5-9694-aed30a79462c",
    "Roger Raupp",
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
pub(in crate::card::sets) static STALKING_STONES: CardRecord = CardRecord::new(
    "Stalking Stones",
    "b4d3d349-5c23-43a9-b25e-0e1a35b84673",
    "Stephen Daniele",
    // Six mana to turn a colourless land into a threat, which is a card only
    // a deck with nothing else to spend on ever activates.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{6}: This land becomes a 3/3 Elemental artifact creature that's still a land.",
            &[CostDef::Mana(mana_cost!("{6}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
                    ),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ]),
);

// TMP 328 — Thalakos Lowlands
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static THALAKOS_LOWLANDS: CardRecord = CardRecord::new(
    "Thalakos Lowlands",
    "aa3dcb12-e224-40c8-aecf-941fceb1d323",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// TMP 329 — Vec Townships
// Audit: unsupported — Needs an untap-skipping rider on a mana ability. A mana ability's effect must be a bare AddMana, not a Sequence, and AddManaEffectDef carries only its own fixed riders (self-damage, sacrifice-when-out-of-counters); there is no "skip your next untap step".
pub(in crate::card::sets) static VEC_TOWNSHIPS: CardRecord = CardRecord::new(
    "Vec Townships",
    "15377e49-e929-413f-9501-8f3e4afa0050",
    "Eric David Anderson",
    crate::card::CardRules::unsupported(),
);

// TMP 330 — Wasteland
pub(in crate::card::sets) static WASTELAND: CardRecord = CardRecord::new(
    "Wasteland",
    "99ff731b-8399-40c8-b539-ba6ba5783771",
    "Una Fricker",
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
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "62e339a9-3f9b-401e-a459-dc3affc9a114",
    "Terese Nielsen",
);

// TMP 332 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "da37f78a-a0a6-4ca3-921c-4bc2c17ccda6",
    "Terese Nielsen",
);

// TMP 333 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "3bd35bef-1702-4b3d-b149-8761c5cc5ed9",
    "Terese Nielsen",
);

// TMP 334 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "c7f7baf6-de14-40f8-871f-dda889672608",
    "Terese Nielsen",
);

// TMP 335 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "22578bc8-10c6-4598-82bc-70e970a8f518",
    "Randy Gallegos",
);

// TMP 336 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "3f891c3f-82e2-4f9d-ae4b-443fce0bdd71",
    "Randy Gallegos",
);

// TMP 337 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "b38e48ff-17c8-470c-ba8b-966da1777e77",
    "Randy Gallegos",
);

// TMP 338 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "5e6e59d7-5038-46b4-9f60-3d9d0cbf0a4e",
    "Randy Gallegos",
);

// TMP 339 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "5912d2aa-fe91-4cea-9c7a-6dca745f8560",
    "Brom",
);

// TMP 340 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "000366c8-7a43-49d7-a103-ac5bd7efd9aa",
    "Brom",
);

// TMP 341 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "c96dbfea-3b79-4e5b-a356-3c04d1e78e83",
    "Brom",
);

// TMP 342 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "9330376c-0242-4e28-b535-26c84b43c3e6",
    "Brom",
);

// TMP 343 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "9d6dc341-89dc-4e99-baa2-ad0ae59f0e94",
    "Mark Poole",
);

// TMP 344 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "676fb0ed-5a19-4fde-b2c4-f46c7e0915f8",
    "Mark Poole",
);

// TMP 345 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "9515ced4-b679-48f0-bf62-8b7baef5e1c2",
    "Mark Poole",
);

// TMP 346 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "23e043bf-a6d7-4778-8460-13bdf38b7d39",
    "Mark Poole",
);

// TMP 347 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "36b3ffbd-3f3e-4fca-80c2-94f9fdc198a5",
    "Douglas Shuler",
);

// TMP 348 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "72bac71a-c326-4640-bdf8-43682f59060b",
    "Douglas Shuler",
);

// TMP 349 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "6b667225-a808-43e9-955e-6c4e7ecd53f2",
    "Douglas Shuler",
);

// TMP 350 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "32411c3a-da2b-4316-9848-971e90951303",
    "Douglas Shuler",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADVANCE_SCOUT,
    &ANGELIC_PROTECTOR,
    &ANOINT,
    &ARMOR_SLIVER,
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
    &DAUTHI_HORROR,
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
    &LIGHTNING_ELEMENTAL,
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
    &NATURE_S_REVOLT,
    &NURTURING_LICID,
    &OVERRUN,
    &PINCHER_BEETLES,
    &REALITY_ANCHOR,
    &REAP,
    &RECYCLE,
    &RESPITE,
    &ROOT_MAZE,
    &ROOTBREAKER_WURM,
    &ROOTWALLA,
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
    &PHYREXIAN_HULK,
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
    ARMORED_PEGASUS_REPRINT,
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    DISENCHANT_REPRINT,
    PACIFISM_REPRINT,
    COUNTERSPELL_REPRINT,
    DREAM_CACHE_REPRINT,
    GASEOUS_FORM_REPRINT,
    HORNED_TURTLE_REPRINT,
    POWER_SINK_REPRINT,
    SPELL_BLAST_REPRINT,
    TIME_EBB_REPRINT,
    WIND_DRAKE_REPRINT,
    COERCION_REPRINT,
    DARK_BANISHING_REPRINT,
    DARK_RITUAL_REPRINT,
    ENFEEBLEMENT_REPRINT,
    GRAVEDIGGER_REPRINT,
    RAIN_OF_TEARS_REPRINT,
    GIANT_STRENGTH_REPRINT,
    SHATTER_REPRINT,
    STONE_RAIN_REPRINT,
    CHARGING_RHINO_REPRINT,
    NATURAL_SPRING_REPRINT,
    NEEDLE_STORM_REPRINT,
    RAMPANT_GROWTH_REPRINT,
    TRANQUILITY_REPRINT,
    WINTER_S_GRASP_REPRINT,
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
