//! Invasion cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::Binding;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AdditionalCostValueDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseGroupDef;
use crate::card::ColorSet;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaTypeDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PartitionGroupDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementConditionDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::SacrificedAmountDef;
use crate::card::ScaledValueDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
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
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("INV", "invasion");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// INV 1 — Alabaster Leech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_LEECH: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static ARDENT_SOLDIER: CardRecord = CardRecord::new(
    "Ardent Soldier",
    "39dce974-846f-4365-b0a5-851e38668e7d",
    "Paolo Parente",
    // Two mana for a blocker or five for a slightly better one, which is what
    // kicker sells: one card that is never dead.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 1, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{W}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2} (You may pay an additional {2} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::vigilance(),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with a +1/+1 counter on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
    ]),
);

// INV 4 — Atalya, Samite Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATALYA_SAMITE_MASTER: CardRecord = CardRecord::new(
    "Atalya, Samite Master",
    "90500e7a-f76d-453a-bda0-d56d3f7c7534",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 5 — Benalish Emissary
pub(in crate::card::sets) static BENALISH_EMISSARY: CardRecord = CardRecord::new(
    "Benalish Emissary",
    "6b82d56e-80d7-4be9-ac22-de3257efc458",
    "Randy Gallegos",
    // A 1/4 wall early or a Stone Rain with a body later, which is a lot of
    // mileage for a common.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Wizard"], 1, 4).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{W}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{G} (You may pay an additional {1}{G} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, destroy target land.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &const { TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked) },
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// INV 6 — Benalish Heralds
pub(in crate::card::sets) static BENALISH_HERALDS: CardRecord = CardRecord::new(
    "Benalish Heralds",
    "13c6e51d-54eb-4e5b-9ec9-54521b16b8d1",
    "Don Hazeltine",
    // A card a turn for four mana is a bad rate, and the second colour is
    // what stops it being a good one.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Spellshaper"], 2, 4).with_ability(
        AbilityDef::activated(
            "{3}{U}, {T}: Draw a card.",
            &[CostDef::Mana(mana_cost!("{3}{U}")), CostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// INV 7 — Benalish Lancer
pub(in crate::card::sets) static BENALISH_LANCER: CardRecord = CardRecord::new(
    "Benalish Lancer",
    "3a38d40a-e745-4fee-b179-f8c27e9b2fbd",
    "Paolo Parente",
    // A 2/2 on turn three or a 4/4 first striker on turn six, and white decks
    // that flooded out finally had somewhere to put the mana.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{4}{W}{W}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2}{W} (You may pay an additional {2}{W} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with 2 +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::static_ability(
            "If this creature was kicked, it has first strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&const { abilities::first_strike() }),
                },
            },
        ),
    ]),
);

// INV 8 — Benalish Trapper
pub(in crate::card::sets) static BENALISH_TRAPPER: CardRecord = CardRecord::new(
    "Benalish Trapper",
    "e312653d-c3e1-4c79-90d2-0963419b618c",
    "Ken Meyer, Jr.",
    // Master Decoy again, in a block where the tapper was white's answer to
    // everything bigger than it.
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

// INV 9 — Blinding Light (reprint)
const BLINDING_LIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::BLINDING_LIGHT,
    "882c1e15-b508-4885-9626-4c8d2598a006",
    "Marc Fishman",
);

// INV 10 — Capashen Unicorn
pub(in crate::card::sets) static CAPASHEN_UNICORN: CardRecord = CardRecord::new(
    "Capashen Unicorn",
    "ec3e5741-88d7-4837-9b43-ba8304d9ee74",
    "Jerry Tiritilli",
    // Disenchant stapled to a body, payable in instalments -- white gets to hold
    // the answer on the battlefield instead of in hand.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Unicorn"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{W}, {T}, Sacrifice this creature: Destroy target artifact or enchantment.",
            &[
                CostDef::Mana(mana_cost!("{1}{W}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// INV 11 — Crimson Acolyte
pub(in crate::card::sets) static CRIMSON_ACOLYTE: CardRecord = CardRecord::new(
    "Crimson Acolyte",
    "c1718028-3009-4bdd-9f6f-59c17edd1344",
    "Dany Orizio",
    // The red version, which turns off a burn spell rather than a removal
    // spell.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::Red),
        AbilityDef::activated_with_targets(
            "{W}: Target creature gains protection from red until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(
                    &const { abilities::protection_from_color(ManaColor::Red) },
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// INV 12 — Crusading Knight
pub(in crate::card::sets) static CRUSADING_KNIGHT: CardRecord = CardRecord::new(
    "Crusading Knight",
    "a4ab4640-1871-41dd-bd21-64741e21ba37",
    "Edward P. Beard, Jr.",
    // Protection makes it unblockable against the deck whose lands make it
    // big, which is the whole design.
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Black),
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each Swamp your opponents control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                ),
            },
        ),
    ]),
);

// INV 13 — Death or Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_OR_GLORY: CardRecord = CardRecord::new(
    "Death or Glory",
    "81f967c9-b38d-489d-96cc-44a6b1804e10",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// INV 14 — Dismantling Blow
pub(in crate::card::sets) static DISMANTLING_BLOW: CardRecord = CardRecord::new(
    "Dismantling Blow",
    "39514d54-cb6c-4b3b-a3be-46db991be4d4",
    "Mark Tedin",
    // Disenchant that replaces itself, which is what makes a maindeck slot for
    // artifact removal defensible at all.
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{4}{W}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2}{U} (You may pay an additional {2}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell_with_targets(
            "Destroy target artifact or enchantment. If this spell was kicked, draw two cards.",
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                )]
            },
            EffectDef::Sequence(
                &const {
                    [
                        EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            then: None,
                        },
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::SourceCastWith(
                                AlternativeCastKindDef::Kicked,
                            ),
                            then: &EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(2),
                            },
                        },
                    ]
                },
            ),
        ),
    ]),
);

// INV 15 — Divine Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVINE_PRESENCE: CardRecord = CardRecord::new(
    "Divine Presence",
    "28cb898d-d6ce-410a-83bf-37962cca2735",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// INV 16 — Fight or Flight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIGHT_OR_FLIGHT: CardRecord = CardRecord::new(
    "Fight or Flight",
    "46bde162-3737-4b93-a27a-63b909a4183d",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 17 — Glimmering Angel
pub(in crate::card::sets) static GLIMMERING_ANGEL: CardRecord = CardRecord::new(
    "Glimmering Angel",
    "f14f55e4-eded-4a86-87f4-b8fa6f30bc0f",
    "Ciruelo",
    // A white flier that blue mana protects, which is the whole point of
    // the gold-adjacent cycle it belongs to.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}: This creature gains shroud until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::shroud() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// INV 18 — Global Ruin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOBAL_RUIN: CardRecord = CardRecord::new(
    "Global Ruin",
    "336474b4-2cf5-44c0-b72c-f75f1a7ed928",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 19 — Harsh Judgment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARSH_JUDGMENT: CardRecord = CardRecord::new(
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
    "Liberate",
    "96794470-31ea-478f-b11c-dc8342a508e2",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 22 — Obsidian Acolyte
pub(in crate::card::sets) static OBSIDIAN_ACOLYTE: CardRecord = CardRecord::new(
    "Obsidian Acolyte",
    "868efcee-bb13-4b6f-b81b-99408685e4c4",
    "Matthew D. Wilson",
    // Protection it can hand out, so a one-drop answers a black removal
    // spell aimed at anything.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::Black),
        AbilityDef::activated_with_targets(
            "{W}: Target creature gains protection from black until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(
                    &const { abilities::protection_from_color(ManaColor::Black) },
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// INV 23 — Orim's Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_TOUCH: CardRecord = CardRecord::new(
    "Orim's Touch",
    "559f551e-7891-4c6d-8798-a25c0255fa3b",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// INV 24 — Pledge of Loyalty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLEDGE_OF_LOYALTY: CardRecord = CardRecord::new(
    "Pledge of Loyalty",
    "d6f98c26-5b30-400c-8af1-8c6c43065f63",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// INV 25 — Prison Barricade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISON_BARRICADE: CardRecord = CardRecord::new(
    "Prison Barricade",
    "449c4800-8718-4593-a61e-03ad7f348c6d",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// INV 26 — Protective Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROTECTIVE_SPHERE: CardRecord = CardRecord::new(
    "Protective Sphere",
    "ef5ef13e-1cf0-42a9-95d0-30ade254d6a8",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 27 — Pure Reflection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURE_REFLECTION: CardRecord = CardRecord::new(
    "Pure Reflection",
    "bbff85a6-a51b-424e-a86b-da52c9b3a9da",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 28 — Rampant Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMPANT_ELEPHANT: CardRecord = CardRecord::new(
    "Rampant Elephant",
    "752642d2-3dad-4f58-b154-beb5982141dc",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// INV 29 — Razorfoot Griffin
pub(in crate::card::sets) static RAZORFOOT_GRIFFIN: CardRecord = CardRecord::new(
    "Razorfoot Griffin",
    "819e2046-9b78-4fd0-92f8-798bfac51195",
    "Ben Thompson",
    // A flier that wins every fight in the air it is not outsized in, which
    // is most of them at four mana.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// INV 30 — Restrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTRAIN: CardRecord = CardRecord::new(
    "Restrain",
    "f6b5c765-619c-4db9-b509-91892fb65e8f",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 31 — Reviving Dose
pub(in crate::card::sets) static REVIVING_DOSE: CardRecord = CardRecord::new(
    "Reviving Dose",
    "8d44dd88-ad20-4d89-8831-d2dfa6873428",
    "D. Alexander Gregory",
    // Three life for three mana is a bad rate; three life and a card is the
    // rate a deck racing on life actually pays.
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "You gain 3 life.\nDraw a card.",
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]
            },
        ),
    )),
);

// INV 32 — Rewards of Diversity
pub(in crate::card::sets) static REWARDS_OF_DIVERSITY: CardRecord = CardRecord::new(
    "Rewards of Diversity",
    "04116b38-8fb1-47c6-b68d-060d0fc4a60d",
    "Darrell Riche",
    // Four life is a lot, and in a block where every good card was gold the
    // trigger was never going to be short of work.
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a multicolored spell, you gain 4 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::ColorCount(2),
                ObjectPredicateDef::ColorCount(3),
                ObjectPredicateDef::ColorCount(4),
                ObjectPredicateDef::ColorCount(5),
            ]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(4),
        },
    )),
);

// INV 33 — Reya Dawnbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REYA_DAWNBRINGER: CardRecord = CardRecord::new(
    "Reya Dawnbringer",
    "e1e0e72b-e65e-4578-b610-9f529daa32d7",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 34 — Rout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROUT: CardRecord = CardRecord::new(
    "Rout",
    "94bc55ed-b89b-4e22-b3f1-4ce0f8d180d7",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 35 — Ruham Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUHAM_DJINN: CardRecord = CardRecord::new(
    "Ruham Djinn",
    "a46c7718-1ecc-418c-b213-13be9de5cb7f",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// INV 36 — Samite Ministration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_MINISTRATION: CardRecord = CardRecord::new(
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
    "Spirit of Resistance",
    "5fb66439-df73-4a01-a8d4-6f2334297fdf",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// INV 39 — Spirit Weaver
pub(in crate::card::sets) static SPIRIT_WEAVER: CardRecord = CardRecord::new(
    "Spirit Weaver",
    "90b0ef47-cb22-4146-a17e-e49a6031a7e6",
    "Matthew D. Wilson",
    // Toughness for green and blue, which turns every trade into a block
    // that survives.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}: Target green or blue creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::Green),
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 40 — Strength of Unity
pub(in crate::card::sets) static STRENGTH_OF_UNITY: CardRecord = CardRecord::new(
    "Strength of Unity",
    "1a9d4ff8-af35-413f-9aa2-f4c6e34fade2",
    "Andrew Goldhawk",
// Domain on an Aura, so its size is the deck's mana base rather than
    // anything on the board.
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Domain — Enchanted creature gets +1/+1 for each basic land type among lands you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                        ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                    ),
                },
            ),
        ]),
);

// INV 41 — Sunscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    "Sunscape Apprentice",
    "a9d6bd19-77c9-4a1a-a2d5-6f9737693fea",
    "Stephanie Law",
    crate::card::CardRules::unsupported(),
);

// INV 42 — Sunscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_MASTER: CardRecord = CardRecord::new(
    "Sunscape Master",
    "ebb7203d-529d-45d2-8e03-cd342c153f38",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// INV 43 — Teferi's Care
pub(in crate::card::sets) static TEFERI_S_CARE: CardRecord = CardRecord::new(
    "Teferi's Care",
    "031b1cc1-4468-4bc5-85c0-c22dce131225",
    "Scott Bailey",
    // Two answers to enchantments in one card, one of which spends the card
    // itself -- which is the point of a two-mana enchantment.
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{W}, Sacrifice an enchantment: Destroy target enchantment.",
            &[
                CostDef::Mana(mana_cost!("{W}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    controller: PlayerRelation::You,
                },
            ],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}{U}{U}: Counter target enchantment spell.",
            &[CostDef::Mana(mana_cost!("{3}{U}{U}"))],
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )]
            },
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// INV 44 — Wayfaring Giant
pub(in crate::card::sets) static WAYFARING_GIANT: CardRecord = CardRecord::new(
    "Wayfaring Giant",
    "57e45de5-0e8b-41d3-979b-ec5a29cac682",
    "Christopher Moeller",
    // Six mana for a 1/3 that is only worth casting in the deck that can
    // already cast anything.
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Giant"], 1, 3).with_ability(
        AbilityDef::static_ability(
            "Domain — This creature gets +1/+1 for each basic land type among lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                ),
            },
        ),
    ),
);

// INV 45 — Winnow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINNOW: CardRecord = CardRecord::new(
    "Winnow",
    "d61748dd-4010-47da-8717-ca0147877057",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// INV 46 — Barrin's Unmaking
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_S_UNMAKING: CardRecord = CardRecord::new(
    "Barrin's Unmaking",
    "4d4cecb0-12b5-4678-b5e7-8cec8fc86cef",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// INV 47 — Blind Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIND_SEER: CardRecord = CardRecord::new(
    "Blind Seer",
    "5c54ec26-c7f1-4258-9cc9-1709987f293c",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 48 — Breaking Wave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAKING_WAVE: CardRecord = CardRecord::new(
    "Breaking Wave",
    "1b39cd77-97aa-4099-8405-366f82079758",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// INV 49 — Collective Restraint
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLECTIVE_RESTRAINT: CardRecord = CardRecord::new(
    "Collective Restraint",
    "d71daa57-ac02-4dd9-8c90-d38bdd45fb51",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// INV 50 — Crystal Spray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_SPRAY: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static DISTORTING_WAKE: CardRecord = CardRecord::new(
    "Distorting Wake",
    "cf48eec9-96be-4f53-9d9a-c6f02d44c995",
    "Arnie Swekel",
    // A bounce sweeper sized to the mana, which at four or five targets is
    // a whole turn and then the game.
    CardRules::new_sorcery(mana_cost!("{X}{U}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return X target nonland permanents to their owners' hands.",
            &[AbilityTargetDef::exactly_chosen_x(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// INV 53 — Dream Thrush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_THRUSH: CardRecord = CardRecord::new(
    "Dream Thrush",
    "258217df-ae88-4d93-895a-3fd242baacd1",
    "D. J. Cleland-Hura",
    crate::card::CardRules::unsupported(),
);

// INV 54 — Empress Galina
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMPRESS_GALINA: CardRecord = CardRecord::new(
    "Empress Galina",
    "6851dbc7-f072-41e7-a899-897445d99425",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 55 — Essence Leak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_LEAK: CardRecord = CardRecord::new(
    "Essence Leak",
    "9099b2e6-9ed8-4a9c-97ca-77cc47678228",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 56 — Exclude
pub(in crate::card::sets) static EXCLUDE: CardRecord = CardRecord::new(
    "Exclude",
    "aeb359c8-209c-455f-84b2-970e5678a9fa",
    "Mark Romanoski",
    // A counter that replaces itself, which is what made the narrow half of
    // the card affordable: a dead Exclude still cycles.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target creature spell.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::counter_target(TargetIndex::PRIMARY),
            // The draw is not conditional on the counter resolving, so an
            // Exclude whose target left the stack still replaces itself.
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// INV 57 — Fact or Fiction
const FACT_FIRST: Binding = Binding!("fact_first");
const FACT_SECOND: Binding = Binding!("fact_second");
const FACT_CHOSEN: Binding = Binding!("fact_chosen");
const FACT_UNCHOSEN: Binding = Binding!("fact_unchosen");

pub(in crate::card::sets) static FACT_OR_FICTION: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static FAERIE_SQUADRON: CardRecord = CardRecord::new(
    "Faerie Squadron",
    "4c707c81-dbbd-43be-a79a-7bc92a584839",
    "rk post",
    // One mana on turn one or a 3/3 flier later: the same card is the play in
    // both halves of the game, which is the whole point of kicker.
    CardRules::new_creature(mana_cost!("{U}"), &["Faerie"], 1, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{U}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {3}{U} (You may pay an additional {3}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with 2 +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::static_ability(
            "If this creature was kicked, it has flying.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                },
            },
        ),
    ]),
);

// INV 59 — Mana Maze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_MAZE: CardRecord = CardRecord::new(
    "Mana Maze",
    "0d62cc17-8fa3-495c-a098-ffbbec89fa53",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 60 — Manipulate Fate
pub(in crate::card::sets) static MANIPULATE_FATE: CardRecord = CardRecord::new(
    "Manipulate Fate",
    "5bb52acb-dedb-4ed6-a6da-8c036f2b2958",
    "John Matson",
    // Exiling your own cards is the point: it is a cantrip in a deck that
    // wanted three specific cards out of the library.
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Search your library for three cards, exile them, then shuffle.\nDraw a card.",
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::Any,
                        minimum: 0,
                        maximum: ValueDef::Constant(3),
                        reveal: false,
                        destination: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]
            },
        ),
    )),
);

// INV 61 — Metathran Aerostat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_AEROSTAT: CardRecord = CardRecord::new(
    "Metathran Aerostat",
    "59f34850-fb6f-4ac5-8309-4d53d770e28c",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 62 — Metathran Transport
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_TRANSPORT: CardRecord = CardRecord::new(
    "Metathran Transport",
    "4fa9048d-1599-44a5-b4b2-45382c5b238d",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 63 — Metathran Zombie
pub(in crate::card::sets) static METATHRAN_ZOMBIE: CardRecord = CardRecord::new(
    "Metathran Zombie",
    "6676a0f7-8213-4547-b2ac-b904cd418073",
    "Arnie Swekel",
    // A blue creature that regenerates for black: the card is a gold card
    // in everything but its mana cost, which is the point of the block.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Metathran", "Zombie"], 1, 1).with_ability(
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
    ),
);

// INV 64 — Opt
pub(in crate::card::sets) static OPT: CardRecord = CardRecord::new(
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
    "Probe",
    "a2a58d18-3d52-4178-86b2-7590d4164e76",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 67 — Prohibit
pub(in crate::card::sets) static PROHIBIT: CardRecord = CardRecord::new(
    "Prohibit",
    "0daa5458-2a97-40d0-b18d-2381a7a68ee1",
    "Adam Rex",
CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        abilities::kicker(
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
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
    "Psychic Battle",
    "8758ca24-e613-43bf-be58-4cf557f82d0c",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// INV 69 — Rainbow Crow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAINBOW_CROW: CardRecord = CardRecord::new(
    "Rainbow Crow",
    "7e622ad2-473f-489e-b4cf-bbdcc44d0cde",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// INV 70 — Repulse
pub(in crate::card::sets) static REPULSE: CardRecord = CardRecord::new(
    "Repulse",
    "9a04e9be-48be-440e-9825-cfffd4c2b1a4",
    "Aaron Boyd",
    // Bouncing at instant speed rarely answers anything permanently, so the
    // cantrip is what pays for the card and makes three mana acceptable.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            // The draw is not conditional on the bounce: a target that has
            // already left still leaves this a cantrip.
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// INV 71 — Sapphire Leech
pub(in crate::card::sets) static SAPPHIRE_LEECH: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static SHORELINE_RAIDER: CardRecord = CardRecord::new(
    "Shoreline Raider",
    "d895b3b8-2acc-4c9f-8341-f651c1255b7c",
    "Nelson DeCastro",
    // Protection from a creature type that only exists in this block, which
    // is as narrow as the keyword gets.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk"], 2, 2).with_ability(
        AbilityDef::keyword(
            "Protection from Kavu",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Kavu")),
        ),
    ),
);

// INV 74 — Sky Weaver
pub(in crate::card::sets) static SKY_WEAVER: CardRecord = CardRecord::new(
    "Sky Weaver",
    "04974146-42a8-4f10-b443-67bfeaa54d5d",
    "Christopher Moeller",
    // Evasion for white and black, which is the version that actually ends
    // games.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Metathran", "Wizard"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}: Target white or black creature gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::White),
                        ObjectPredicateDef::Color(ManaColor::Black),
                    ]),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 75 — Stormscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    "Stormscape Apprentice",
    "1eb42f39-9187-44e4-aa34-14ab31977199",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 76 — Stormscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_MASTER: CardRecord = CardRecord::new(
    "Stormscape Master",
    "9b704165-4587-48f1-8830-c5a07ec666cc",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// INV 77 — Sway of Illusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWAY_OF_ILLUSION: CardRecord = CardRecord::new(
    "Sway of Illusion",
    "ff65e386-9aec-4deb-a4ec-d9a97bd87645",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 78 — Teferi's Response
pub(in crate::card::sets) static TEFERIS_RESPONSE: CardRecord = CardRecord::new(
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
    "Temporal Distortion",
    "74bd0d14-8d26-403f-9405-d0dcdecd1a49",
    "Stephanie Law",
    crate::card::CardRules::unsupported(),
);

// INV 80 — Tidal Visionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_VISIONARY: CardRecord = CardRecord::new(
    "Tidal Visionary",
    "a72a3051-7f46-4b6b-b4fb-0f170d9687ab",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 81 — Tolarian Emissary
pub(in crate::card::sets) static TOLARIAN_EMISSARY: CardRecord = CardRecord::new(
    "Tolarian Emissary",
    "1cbc55e5-b84c-4449-a288-ec26cdd3997c",
    "Ron Spencer",
    // The blue half of the cycle answers an enchantment, which is the thing blue
    // otherwise cannot touch at all.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{U}{W}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{W} (You may pay an additional {1}{W} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::flying(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, destroy target enchantment.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &const { TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked) },
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// INV 82 — Tower Drake
pub(in crate::card::sets) static TOWER_DRAKE: CardRecord = CardRecord::new(
    "Tower Drake",
    "aef97b38-f7a5-4db7-9550-24aa1a1ebbda",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 1).with_abilities(&[
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

// INV 83 — Traveler's Cloak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAVELER_S_CLOAK: CardRecord = CardRecord::new(
    "Traveler's Cloak",
    "977f0f82-0542-40c9-9a48-73077941dbd1",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 84 — Vodalian Hypnotist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_HYPNOTIST: CardRecord = CardRecord::new(
    "Vodalian Hypnotist",
    "721fd877-0a28-4002-8b47-058bac4ac44d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 85 — Vodalian Merchant
pub(in crate::card::sets) static VODALIAN_MERCHANT: CardRecord = CardRecord::new(
    "Vodalian Merchant",
    "c1c0effa-a4b8-4166-a66a-90cf01c6ea0d",
    "Scott M. Fischer",
    // A body and a look at one more card, which is what blue's commons did
    // before they drew outright.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card, then discard a card.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ),
);

// INV 86 — Vodalian Serpent
pub(in crate::card::sets) static VODALIAN_SERPENT: CardRecord = CardRecord::new(
    "Vodalian Serpent",
    "92adcf6c-ab14-414c-a5cb-56feae048c84",
    "Christopher Moeller",
    // A big body that only attacks into Islands, so the two extra mana buy a
    // 6/6 that still needs the opponent's cooperation.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Serpent"], 2, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{5}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2} (You may pay an additional {2} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
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
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with four +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 4,
                },
            ),
        ),
    ]),
);

// INV 87 — Wash Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WASH_OUT: CardRecord = CardRecord::new(
    "Wash Out",
    "7719d043-5827-4479-825b-23d9e979ead7",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 88 — Well-Laid Plans
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_LAID_PLANS: CardRecord = CardRecord::new(
    "Well-Laid Plans",
    "1c55eb8f-925a-42c1-9e48-d7f99cab3b01",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 89 — Worldly Counsel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDLY_COUNSEL: CardRecord = CardRecord::new(
    "Worldly Counsel",
    "8fc66fbf-f411-4607-aece-7c35d9a07c80",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// INV 90 — Zanam Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZANAM_DJINN: CardRecord = CardRecord::new(
    "Zanam Djinn",
    "57a3c1d5-0ca8-443b-ae7a-66e0363e377b",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 91 — Addle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADDLE: CardRecord = CardRecord::new(
    "Addle",
    "e8afb9d0-affa-4599-bf29-729cfe64703b",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// INV 92 — Agonizing Demise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGONIZING_DEMISE: CardRecord = CardRecord::new(
    "Agonizing Demise",
    "539ac5e1-4bad-4f70-abac-e70c406bebec",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// INV 93 — Andradite Leech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANDRADITE_LEECH: CardRecord = CardRecord::new(
    "Andradite Leech",
    "6da0d4f3-9216-406c-8f3e-b9bb0a11dc75",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// INV 94 — Annihilate
pub(in crate::card::sets) static ANNIHILATE: CardRecord = CardRecord::new(
    "Annihilate",
    "4a3bf039-ecf6-477e-997c-e32c55323c01",
    "Kev Walker",
    // Five mana for removal is a lot; five mana for removal and a card is
    // what a control deck plays instead of two spells.
    CardRules::new_instant(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target nonblack creature. It can't be regenerated.\nDraw a card.",
        &const {
            [AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]),
            )]
        },
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::WithRule {
                        rule: AppliedRuleDef::CannotRegenerate,
                        effect: &EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            then: None,
                        },
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]
            },
        ),
    )),
);

// INV 95 — Bog Initiate
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static BOG_INITIATE: CardRecord = CardRecord::new(
    "Bog Initiate",
    "8962dc3b-24ca-4c3c-ba1d-933c29cf7b73",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// INV 96 — Cremate
pub(in crate::card::sets) static CREMATE: CardRecord = CardRecord::new(
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
    "Defiling Tears",
    "db7cba29-9472-4874-bd54-37edf70645b2",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// INV 100 — Desperate Research
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_RESEARCH: CardRecord = CardRecord::new(
    "Desperate Research",
    "6a42ac7e-4a27-488c-a2e7-338b18103b02",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 101 — Devouring Strossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOURING_STROSSUS: CardRecord = CardRecord::new(
    "Devouring Strossus",
    "064f013f-e74f-419d-8d17-7748bd91885e",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 102 — Do or Die
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DO_OR_DIE: CardRecord = CardRecord::new(
    "Do or Die",
    "05f63cd9-e82b-4cf8-b8ce-f0aa0157692b",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 103 — Dredge
pub(in crate::card::sets) static DREDGE: CardRecord = CardRecord::new(
    "Dredge",
    "68bfa3d5-0f0b-4684-9567-f1478da01df7",
    "Donato Giancola",
    // One mana to turn a permanent into a card, which is only a gain when
    // the permanent was going to die anyway.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Sacrifice a creature or land.\nDraw a card.",
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::SacrificeOfChoice {
                        player: EffectRecipientDef::Controller,
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        count: ValueDef::Constant(1),
                        then: None,
                        amount: SacrificedAmountDef::Power,
                        otherwise: None,
                        optional: false,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]
            },
        ),
    )),
);

// INV 104 — Duskwalker
pub(in crate::card::sets) static DUSKWALKER: CardRecord = CardRecord::new(
    "Duskwalker",
    "39a4a026-f44e-40e1-9942-a3d8448aca70",
    "David Martin",
    // A one-drop that becomes a 3/3 nearly nothing blocks, which is more than
    // black usually gets out of its worst creature.
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Minion"], 1, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{B}{B}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {3}{B} (You may pay an additional {3}{B} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with 2 +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::static_ability(
            "If this creature was kicked, it has fear.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: abilities::FEAR_RESTRICTION,
                },
            },
        ),
    ]),
);

// INV 105 — Exotic Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXOTIC_CURSE: CardRecord = CardRecord::new(
    "Exotic Curse",
    "8ee35d99-9a8a-421b-bf43-74446909d87d",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// INV 106 — Firescreamer
pub(in crate::card::sets) static FIRESCREAMER: CardRecord = CardRecord::new(
    "Firescreamer",
    "155a2213-bf6e-4a54-924b-e450b7d06f26",
    "Alan Pollack",
    // A black creature that pumps with red mana, which is the block's whole
    // idea: the card is gold in play without being gold in the deck.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Kavu"], 2, 2).with_ability(
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
    ),
);

// INV 107 — Goham Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOHAM_DJINN: CardRecord = CardRecord::new(
    "Goham Djinn",
    "d67796c7-4d93-4c50-8839-bb69e075bc42",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 108 — Hate Weaver
pub(in crate::card::sets) static HATE_WEAVER: CardRecord = CardRecord::new(
    "Hate Weaver",
    "8328e131-b44d-4dd0-9ce4-454c6afe6fa6",
    "Roger Raupp",
    // The same shape aimed at blue and red, where a point of power is worth
    // more than it looks on an evasive body.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Wizard"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}: Target blue or red creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::Blue),
                        ObjectPredicateDef::Color(ManaColor::Red),
                    ]),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 109 — Hypnotic Cloud
pub(in crate::card::sets) static HYPNOTIC_CLOUD: CardRecord = CardRecord::new(
    "Hypnotic Cloud",
    "a7502ea2-7555-449e-baee-6ecef5573a3b",
    "Randy Gallegos",
    // A two-mana Coercion that turns into a Mind Twist for three once the game
    // has gone long enough for the hand to be worth taking.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{5}{B}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {4} (You may pay an additional {4} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell_with_targets(
            "Target player discards a card. If this spell was kicked, that player discards three \
             cards instead.",
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )]
            },
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                otherwise: &EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            },
        ),
    ]),
);

// INV 110 — Marauding Knight
pub(in crate::card::sets) static MARAUDING_KNIGHT: CardRecord = CardRecord::new(
    "Marauding Knight",
    "cea2a7de-c67e-4541-be8c-e5ef7b64d94a",
    "Daren Bader",
    // The black member of the pair, aimed at white the same way.
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}"),
        &["Phyrexian", "Zombie", "Knight"],
        2,
        2,
    )
    .with_abilities(&[
        abilities::protection_from_color(ManaColor::White),
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each Plains your opponents control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                ),
            },
        ),
    ]),
);

// INV 111 — Mourning
pub(in crate::card::sets) static MOURNING: CardRecord = CardRecord::new(
    "Mourning",
    "4649d881-709f-4ed0-91de-744d232a82f5",
    "Terese Nielsen",
    // Two power off an attacker, taken back and reused every turn there is a
    // black mana spare.
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::activated(
                "{B}: Return this Aura to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{B}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// INV 112 — Nightscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    "Nightscape Apprentice",
    "7498ca4c-614a-4776-8886-0a6ed58520f6",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// INV 113 — Nightscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_MASTER: CardRecord = CardRecord::new(
    "Nightscape Master",
    "d86174b8-dd9e-4ece-bc23-4f9ac50bccd3",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// INV 114 — Phyrexian Battleflies
pub(in crate::card::sets) static PHYREXIAN_BATTLEFLIES: CardRecord = CardRecord::new(
    "Phyrexian Battleflies",
    "da27c489-c541-4b0d-a844-71aa65e55ceb",
    "Dan Frazier",
// Pit Imp again, reprinted as a Phyrexian for a block that counted
    // them.
    CardRules::new_creature(mana_cost!("{B}"), &["Phyrexian", "Insect"], 0, 1).with_abilities(&[
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

// INV 115 — Phyrexian Delver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DELVER: CardRecord = CardRecord::new(
    "Phyrexian Delver",
    "e66d87a5-7b67-4ec5-b5e2-518d67123118",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// INV 116 — Phyrexian Infiltrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_INFILTRATOR: CardRecord = CardRecord::new(
    "Phyrexian Infiltrator",
    "224b8254-553d-4d88-8163-1f15e1244bd2",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// INV 117 — Phyrexian Reaper
pub(in crate::card::sets) static PHYREXIAN_REAPER: CardRecord = CardRecord::new(
    "Phyrexian Reaper",
    "ccdd498b-1081-43fe-8193-518337a5a3ea",
    "Sam Wood",
    // Green decks block with everything, so the Reaper is a 3/3 that the whole
    // board has to walk around rather than trade with.
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Phyrexian", "Zombie"], 3, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a green creature, destroy that \
             creature. It can't be regenerated.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::Color(ManaColor::Green),
            },
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &const {
                    EffectDef::Destroy {
                        object: EffectRecipientDef::TriggeringObject,
                        then: None,
                    }
                },
            },
        ),
    ),
);

// INV 118 — Phyrexian Slayer
pub(in crate::card::sets) static PHYREXIAN_SLAYER: CardRecord = CardRecord::new(
    "Phyrexian Slayer",
    "5fa8c604-343f-4c94-ac25-439ab1845c19",
    "Sam Wood",
    // The same deal against white, and this one flies, so the blocker it eats is
    // usually the only one that could have reached it.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Phyrexian", "Minion"], 2, 2).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature becomes blocked by a white creature, destroy that \
             creature. It can't be regenerated.",
                TriggerEventDef::BecomesBlockedBy {
                    blocker: ObjectPredicateDef::Color(ManaColor::White),
                },
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &const {
                        EffectDef::Destroy {
                            object: EffectRecipientDef::TriggeringObject,
                            then: None,
                        }
                    },
                },
            ),
        ],
    ),
);

// INV 119 — Plague Spitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_SPITTER: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static RECOVER: CardRecord = CardRecord::new(
    "Recover",
    "771e695b-24e1-4c65-81e0-1624bda646e7",
    "Nelson DeCastro",
    // Two cards out of one spell, which is the rate that makes a slow
    // three-mana sorcery playable at all.
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to your hand.\nDraw a card.",
        &const {
            [AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )]
        },
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]
            },
        ),
    )),
);

// INV 123 — Scavenged Weaponry
pub(in crate::card::sets) static SCAVENGED_WEAPONRY: CardRecord = CardRecord::new(
    "Scavenged Weaponry",
    "4e8072a9-2699-4c6c-9556-67d91bd67a4b",
    "Alan Pollack",
    // The card it draws is most of the cost back, which is what makes a +1/+1
    // Aura playable at all.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
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
    "Spreading Plague",
    "ac86055d-ce08-4b05-a92c-45e007ca0ba4",
    "Scott Bailey",
    crate::card::CardRules::unsupported(),
);

// INV 126 — Tainted Well
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAINTED_WELL: CardRecord = CardRecord::new(
    "Tainted Well",
    "2eec00a1-7e12-42d2-8f46-de8ab7323c2c",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// INV 127 — Trench Wurm
pub(in crate::card::sets) static TRENCH_WURM: CardRecord = CardRecord::new(
    "Trench Wurm",
    "1b076f85-d1bf-491a-af9d-f35b8e1bd163",
    "Wayne England",
    // A 3/3 body attached to repeatable land destruction, at a rate slow
    // enough that it only wins a game already going long.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Wurm"], 3, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}, {T}: Destroy target nonbasic land.",
            &[CostDef::Mana(mana_cost!("{2}{R}")), CostDef::TapSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Basic,
                        )),
                    ]),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// INV 128 — Tsabo's Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_S_ASSASSIN: CardRecord = CardRecord::new(
    "Tsabo's Assassin",
    "0047302d-4e3d-4327-9bb2-ecd5b00b00e3",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 129 — Tsabo's Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_S_DECREE: CardRecord = CardRecord::new(
    "Tsabo's Decree",
    "0c1a0ebd-1add-49e6-b5e6-5b26abb1de88",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// INV 130 — Twilight's Call
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWILIGHT_S_CALL: CardRecord = CardRecord::new(
    "Twilight's Call",
    "3c97c8a5-33b3-4f7f-a224-bb4df7b4bcc0",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// INV 131 — Urborg Emissary
pub(in crate::card::sets) static URBORG_EMISSARY: CardRecord = CardRecord::new(
    "Urborg Emissary",
    "e6912c71-1836-4e87-9a65-d577d903d03c",
    "Eric Peterson",
// A 3/1 that bounces anything at all, which in a format of expensive gold
    // cards is often better than killing it.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Wizard"], 3, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{B}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{U} (You may pay an additional {1}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, return target permanent to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &const { TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked) },
            &const { [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)] },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// INV 132 — Urborg Phantom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_PHANTOM: CardRecord = CardRecord::new(
    "Urborg Phantom",
    "397355b9-5b67-4973-972e-3505c500d116",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// INV 133 — Urborg Shambler
pub(in crate::card::sets) static URBORG_SHAMBLER: CardRecord = CardRecord::new(
    "Urborg Shambler",
    "eaedd5c8-03c6-4bbb-bf83-632551830bd4",
    "Pete Venters",
    // It shrinks the opponent's black creatures and its own, so it belongs
    // in a deck that is not black at all -- except for this card.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Horror"], 4, 3).with_ability(
        AbilityDef::static_ability(
            "Other black creatures get -1/-1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Black),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
            },
        ),
    ),
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
pub(in crate::card::sets) static URBORG_SKELETON: CardRecord = CardRecord::new(
    "Urborg Skeleton",
    "467e9486-1604-4fa2-ab1f-be0d7a036798",
    "Tom Wänerstrand",
    // A regenerating wall for one mana, or a regenerating body for four --
    // the same card at both ends of the curve.
    CardRules::new_creature(mana_cost!("{B}"), &["Skeleton"], 0, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{B}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {3} (You may pay an additional {3} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with a +1/+1 counter on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
    ]),
);

// INV 135 — Yawgmoth's Agenda
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAWGMOTH_S_AGENDA: CardRecord = CardRecord::new(
    "Yawgmoth's Agenda",
    "50f7ea7f-4f17-4f78-b68e-693e265ca829",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 136 — Ancient Kavu
pub(in crate::card::sets) static ANCIENT_KAVU: CardRecord = CardRecord::new(
    "Ancient Kavu",
    "c8ccb5d0-735b-443f-addd-8b70f5f2c60d",
    "Glen Angus",
    // A colourless mode in a block full of protection-from-colour: two mana
    // turns off every one of them at once.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Kavu"], 3, 3).with_ability(
        AbilityDef::activated(
            "{2}: This creature becomes colorless until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_colors(ColorSet::empty()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 137 — Bend or Break
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEND_OR_BREAK: CardRecord = CardRecord::new(
    "Bend or Break",
    "b76b6660-d4b2-44de-a1a7-8d00811f90f6",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 138 — Breath of Darigaaz
pub(in crate::card::sets) static BREATH_OF_DARIGAAZ: CardRecord = CardRecord::new(
    "Breath of Darigaaz",
    "480bb7e3-df03-454d-ada0-592ef8a4a6f0",
    "Greg Hildebrandt & Tim Hildebrandt",
    // Two mana to sweep the ground early, five to sweep it late, and the four
    // damage to the face is the reason the deck was red in the first place.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{R}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2} (You may pay an additional {2} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "This spell deals 1 damage to each creature without flying and each player. If this \
             spell was kicked, it deals 4 damage to each creature without flying and each player \
             instead.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::Sequence(
                    &const {
                        [
                            EffectDef::damage(
                                EffectRecipientDef::matching_objects(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                                            KeywordAbility::Flying,
                                        )),
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                ),
                                ValueDef::Constant(4),
                            ),
                            EffectDef::damage(
                                EffectRecipientDef::EachPlayer,
                                ValueDef::Constant(4),
                            ),
                        ]
                    },
                ),
                otherwise: &EffectDef::Sequence(
                    &const {
                        [
                            EffectDef::damage(
                                EffectRecipientDef::matching_objects(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                                            KeywordAbility::Flying,
                                        )),
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                ),
                                ValueDef::Constant(1),
                            ),
                            EffectDef::damage(
                                EffectRecipientDef::EachPlayer,
                                ValueDef::Constant(1),
                            ),
                        ]
                    },
                ),
            },
        ),
    ]),
);

// INV 139 — Callous Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLOUS_GIANT: CardRecord = CardRecord::new(
    "Callous Giant",
    "330028c4-8e91-4fe3-a87d-1660dfd2507e",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// INV 140 — Chaotic Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOTIC_STRIKE: CardRecord = CardRecord::new(
    "Chaotic Strike",
    "061df8e4-6947-4bbb-9fe7-52ca4fd95d65",
    "Massimiliano Frezzato",
    crate::card::CardRules::unsupported(),
);

// INV 141 — Collapsing Borders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLAPSING_BORDERS: CardRecord = CardRecord::new(
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
    "Firebrand Ranger",
    "ee05211e-cf08-4dea-9740-ed06f8682153",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// INV 144 — Ghitu Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_FIRE: CardRecord = CardRecord::new(
    "Ghitu Fire",
    "78827acd-a526-411b-bd22-ab9b538c75dd",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// INV 145 — Goblin Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SPY: CardRecord = CardRecord::new(
    "Goblin Spy",
    "2a89a099-8805-4b26-babd-5d9f48ee406a",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 146 — Halam Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALAM_DJINN: CardRecord = CardRecord::new(
    "Halam Djinn",
    "369ade1f-e909-47ae-bb01-19588269ad8f",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 147 — Hooded Kavu
pub(in crate::card::sets) static HOODED_KAVU: CardRecord = CardRecord::new(
    "Hooded Kavu",
    "5464b80a-22fe-42c7-a839-31667712fb2d",
    "John Howe",
    // A red creature whose evasion costs black, which is the whole point of
    // the cycle it belongs to.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Kavu"], 2, 2).with_ability(
        AbilityDef::activated(
            "{B}: This creature gains fear until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: abilities::FEAR_RESTRICTION,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 148 — Kavu Aggressor
pub(in crate::card::sets) static KAVU_AGGRESSOR: CardRecord = CardRecord::new(
    "Kavu Aggressor",
    "a2832ad3-ce7f-44d2-beb2-c95d982905a6",
    "Christopher Moeller",
    // Three power for three that never blocks, and a fourth point for anybody
    // with seven mana and nothing better to do.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Kavu"], 3, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{6}{R}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {4} (You may pay an additional {4} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with a +1/+1 counter on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
    ]),
);

// INV 149 — Kavu Monarch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_MONARCH: CardRecord = CardRecord::new(
    "Kavu Monarch",
    "ea63dfd5-d8d7-45b8-8219-1cc2b3de5666",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// INV 150 — Kavu Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_RUNNER: CardRecord = CardRecord::new(
    "Kavu Runner",
    "2bc1b462-4e3c-47cc-87c5-f6e29dd70c01",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// INV 151 — Kavu Scout
pub(in crate::card::sets) static KAVU_SCOUT: CardRecord = CardRecord::new(
    "Kavu Scout",
    "cbc2670d-a3f4-47c2-b424-01fd379ff186",
    "DiTerlizzi",
    // All the domain goes into power, so it hits like a five-drop and
    // blocks like a 0/2 whatever the board looks like.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Kavu", "Scout"], 0, 2).with_ability(
        AbilityDef::static_ability(
            "Domain — This creature gets +1/+0 for each basic land type among lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ),
);

// INV 152 — Lightning Dart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_DART: CardRecord = CardRecord::new(
    "Lightning Dart",
    "54d05157-d154-4203-bf3e-add110cb1cee",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 153 — Loafing Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOAFING_GIANT: CardRecord = CardRecord::new(
    "Loafing Giant",
    "fab5f738-04d0-44c9-88ec-28469b668040",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 154 — Mages' Contest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGES_CONTEST: CardRecord = CardRecord::new(
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
    "Obliterate",
    "cdabde40-2143-4677-b7b4-ea8fbf9b1f25",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 157 — Overload
pub(in crate::card::sets) static OVERLOAD: CardRecord = CardRecord::new(
    "Overload",
    "c91fca91-7296-422e-b251-d571b710ff71",
    "Gary Ruddell",
// One mana answers a Lotus Petal or a Cursed Scroll; three answers most
    // of what a Premodern deck actually plays.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        abilities::kicker(
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
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
                    then: None,
                },
            },
        ),
    ]),
);

// INV 158 — Pouncing Kavu
pub(in crate::card::sets) static POUNCING_KAVU: CardRecord = CardRecord::new(
    "Pouncing Kavu",
    "7e6e2e49-7bde-43c1-8caf-43d237dfc052",
    "Adam Rex",
    // First strike on a 3/3 that attacks the turn it lands is a real threat; the
    // 1/1 it started as is a real turn-two play.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kavu"], 1, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{R}{R}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2}{R} (You may pay an additional {2}{R} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::first_strike(),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with 2 +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::static_ability(
            "If this creature was kicked, it has haste.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&const { abilities::haste() }),
                },
            },
        ),
    ]),
);

// INV 159 — Rage Weaver
pub(in crate::card::sets) static RAGE_WEAVER: CardRecord = CardRecord::new(
    "Rage Weaver",
    "a654295d-b63c-4025-bf36-899023a8ba1d",
    "John Matson",
    // Haste handed to the two colours that had the bodies worth hasting,
    // which is what the cycle was built around.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}: Target black or green creature gains haste until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::Black),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::haste() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 160 — Rogue Kavu
pub(in crate::card::sets) static ROGUE_KAVU: CardRecord = CardRecord::new(
    "Rogue Kavu",
    "61e1a445-129d-4bb9-a8b0-3f55e3e0bc58",
    "Darrell Riche",
    // A two-mana 3/1 as long as it goes in alone, which is exactly the
    // turn a red deck has nothing else to add.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kavu"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks alone, it gets +2/+0 until end of turn.",
            TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 1, Some(1)),
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

// INV 161 — Ruby Leech
pub(in crate::card::sets) static RUBY_LEECH: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static SAVAGE_OFFENSIVE: CardRecord = CardRecord::new(
    "Savage Offensive",
    "356744f3-e444-4f4e-bf00-80bb6b2ef76f",
    "Greg Hildebrandt & Tim Hildebrandt",
    // First strike across the board wins a race the attacker was already ahead
    // in; the green mana turns it into the swing that ends the game.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{1}{R}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {G} (You may pay an additional {G} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "Creatures you control gain first strike until end of turn. If this spell was \
             kicked, they get +1/+1 until end of turn.",
            EffectDef::Sequence(
                &const {
                    [
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            effect: AppliedEffectDef::add_ability(
                                &const { abilities::first_strike() },
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::SourceCastWith(
                                AlternativeCastKindDef::Kicked,
                            ),
                            then: &EffectDef::Apply {
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
                        },
                    ]
                },
            ),
        ),
    ]),
);

// INV 163 — Scarred Puma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARRED_PUMA: CardRecord = CardRecord::new(
    "Scarred Puma",
    "067ff95e-c4dc-41bb-9677-67f51a09b05a",
    "Aaron Boyd",
    crate::card::CardRules::unsupported(),
);

// INV 164 — Scorching Lava
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHING_LAVA: CardRecord = CardRecord::new(
    "Scorching Lava",
    "2a85437f-052e-494c-a9ee-265c4624a409",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// INV 165 — Searing Rays
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARING_RAYS: CardRecord = CardRecord::new(
    "Searing Rays",
    "4f66ff2d-f2d2-4a6b-bf26-b510de60c0b6",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// INV 166 — Shivan Emissary
pub(in crate::card::sets) static SHIVAN_EMISSARY: CardRecord = CardRecord::new(
    "Shivan Emissary",
    "945c596e-492e-4cf5-857c-4ddbbdd78485",
    "Paolo Parente",
// Red's only clean answer to a big creature in this block came stapled to a
    // 1/1, and it cost two colours to get it.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{R}{B}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{B} (You may pay an additional {1}{B} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, destroy target nonblack creature. It can't be \
             regenerated.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &const { TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked) },
            &const { [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                    ]))] },
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ),
    ]),
);

// INV 167 — Shivan Harvest
pub(in crate::card::sets) static SHIVAN_HARVEST: CardRecord = CardRecord::new(
    "Shivan Harvest",
    "47dbd765-d7ea-4181-bd22-5c749ad081af",
    "Daren Bader",
    // Repeatable land destruction paid for in creatures, which only a deck
    // making tokens for free can keep pointing at a mana base.
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{R}, Sacrifice a creature: Destroy target nonbasic land.",
            &[
                CostDef::Mana(mana_cost!("{1}{R}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Basic,
                        )),
                    ]),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// INV 168 — Skittish Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKITTISH_KAVU: CardRecord = CardRecord::new(
    "Skittish Kavu",
    "be806378-50a7-4416-9d99-1ea2c1f2b7cb",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 169 — Skizzik
pub(in crate::card::sets) static SKIZZIK: CardRecord = CardRecord::new(
    "Skizzik",
    "dc7732bc-e168-44d9-923a-db7e985bd6db",
    "Ron Spencer",
    // Five power with haste for four mana, rented for the turn unless the fifth
    // mana was there -- a burn spell that sometimes stays.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 5, 3).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{R}{R}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {R} (You may pay an additional {R} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::trample(),
        abilities::haste(),
        AbilityDef::triggered_if(
            "At the beginning of the end step, if this creature wasn't kicked, sacrifice it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &const {
                TriggerConditionDef::Not(&TriggerConditionDef::SourceCastWith(
                    AlternativeCastKindDef::Kicked,
                ))
            },
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
    ]),
);

// INV 170 — Slimy Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIMY_KAVU: CardRecord = CardRecord::new(
    "Slimy Kavu",
    "8e82044d-88cd-4ee4-8ec9-e71a0a85ed46",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 171 — Stand or Fall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAND_OR_FALL: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static TECTONIC_INSTABILITY: CardRecord = CardRecord::new(
    "Tectonic Instability",
    "0476cc6b-ecc6-44d6-9f44-a90d4ee85daa",
    "Rob Alexander",
    // Every land drop costs its player the rest of their mana, which turns a
    // tapped-out turn into two of them.
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::triggered(
        "Whenever a land enters, tap all lands its controller controls.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Land),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Tap {
            object: EffectRecipientDef::objects(ObjectSetDef::PermanentsControlledBy(
                PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject),
            )),
        },
    )),
);

// INV 174 — Thunderscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    "Thunderscape Apprentice",
    "75a0b075-5414-48d3-a2b1-47dc20213e96",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// INV 175 — Thunderscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_MASTER: CardRecord = CardRecord::new(
    "Thunderscape Master",
    "22abdc2f-bdc8-46c4-8ce2-f06befedbc32",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 176 — Tribal Flames
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_FLAMES: CardRecord = CardRecord::new(
    "Tribal Flames",
    "9b32531e-c759-4603-abd0-1724e8df70db",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// INV 177 — Turf Wound
pub(in crate::card::sets) static TURF_WOUND: CardRecord = CardRecord::new(
    "Turf Wound",
    "91392e9f-f96a-4ac5-b1f1-c73540cf249e",
    "Thomas Gianni",
    // Taking a land drop for three mana is a losing trade; taking it at
    // instant speed for free is a tempo play.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target player can't play lands this turn.\nDraw a card.",
        &const {
            [AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )]
        },
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                            PlayRestrictionDef::new(
                                PlayActionMatcherDef::PlayLand,
                                ObjectPredicateDef::Any,
                            ),
                        )),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]
            },
        ),
    )),
);

// INV 178 — Urza's Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_RAGE: CardRecord = CardRecord::new(
    "Urza's Rage",
    "61a25a35-3ae4-471e-adcd-d8baf2f77b68",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// INV 179 — Viashino Grappler
pub(in crate::card::sets) static VIASHINO_GRAPPLER: CardRecord = CardRecord::new(
    "Viashino Grappler",
    "4a94aeb4-349c-4394-848d-c1c9133856e2",
    "Mark Romanoski",
    // A red creature that tramples for green mana, which is what the block
    // charged for a gold effect on a mono-coloured card.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Lizard"], 3, 1).with_ability(
        AbilityDef::activated(
            "{G}: This creature gains trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::trample() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 180 — Zap
pub(in crate::card::sets) static ZAP: CardRecord = CardRecord::new(
    "Zap",
    "7502ce01-b762-40fe-a064-c7b20b08a722",
    "John Matson",
    // One damage is barely a spell; the card it replaces is the reason
    // to run it.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Zap deals 1 damage to any target.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// INV 181 — Aggressive Urge
pub(in crate::card::sets) static AGGRESSIVE_URGE: CardRecord = CardRecord::new(
    "Aggressive Urge",
    "37e3154d-9b1c-4f93-9bc3-a39e68d59d23",
    "Christopher Moeller",
    // A trick that costs nothing in cards, so it can be held up every turn
    // without ever being a blank.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+1 until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
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

// INV 182 — Bind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIND: CardRecord = CardRecord::new(
    "Bind",
    "cfa51783-9ef8-4e51-ba0d-ce8439d83bdf",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// INV 183 — Blurred Mongoose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLURRED_MONGOOSE: CardRecord = CardRecord::new(
    "Blurred Mongoose",
    "4b073e3f-6a6f-495a-ab16-39d906b660f1",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 184 — Canopy Surge
pub(in crate::card::sets) static CANOPY_SURGE: CardRecord = CardRecord::new(
    "Canopy Surge",
    "2e19d68e-7554-4627-a316-beb1f75fa494",
    "Matt Cavotta",
    // Green's answer to fliers, which it otherwise has none of, and it charges
    // the caster the same life it charges everyone else.
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2} (You may pay an additional {2} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "This spell deals 1 damage to each creature with flying and each player. If this \
             spell was kicked, it deals 4 damage to each creature with flying and each player \
             instead.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::Sequence(
                    &const {
                        [
                            EffectDef::damage(
                                EffectRecipientDef::matching_objects(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                ),
                                ValueDef::Constant(4),
                            ),
                            EffectDef::damage(
                                EffectRecipientDef::EachPlayer,
                                ValueDef::Constant(4),
                            ),
                        ]
                    },
                ),
                otherwise: &EffectDef::Sequence(
                    &const {
                        [
                            EffectDef::damage(
                                EffectRecipientDef::matching_objects(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                ),
                                ValueDef::Constant(1),
                            ),
                            EffectDef::damage(
                                EffectRecipientDef::EachPlayer,
                                ValueDef::Constant(1),
                            ),
                        ]
                    },
                ),
            },
        ),
    ]),
);

// INV 185 — Elfhame Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELFHAME_SANCTUARY: CardRecord = CardRecord::new(
    "Elfhame Sanctuary",
    "6ab9a90c-5fd8-4f8c-b692-f98a2974810c",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// INV 186 — Elvish Champion
pub(in crate::card::sets) static ELVISH_CHAMPION: CardRecord = CardRecord::new(
    "Elvish Champion",
    "c19bb473-03b0-4e6d-a7da-0ec1e7707a68",
    "D. Alexander Gregory",
    // The forestwalk is the real clause: against the other green deck the
    // whole team simply cannot be blocked.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elf"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Other Elf creatures get +1/+1 and have forestwalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Elf"),
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
                    AppliedEffectDef::add_ability(&const { abilities::forestwalk() }),
                ]),
            },
        ),
    ),
);

// INV 187 — Explosive Growth
pub(in crate::card::sets) static EXPLOSIVE_GROWTH: CardRecord = CardRecord::new(
    "Explosive Growth",
    "eabc1e77-404c-436b-bde1-be1b21d00584",
    "Arnie Swekel",
    // One mana for a combat trick, six for a game-ending one, and the deck never
    // has to decide which half it wanted when it drew the card.
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{5}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {5} (You may pay an additional {5} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn. If this spell was kicked, that \
             creature gets +5/+5 until end of turn instead.",
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(5),
                        ValueDef::Constant(5),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                otherwise: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
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
    "Kavu Chameleon",
    "f726437b-a41a-4ee9-b0ee-e09327508615",
    "John Howe",
    crate::card::CardRules::unsupported(),
);

// INV 192 — Kavu Climber
pub(in crate::card::sets) static KAVU_CLIMBER: CardRecord = CardRecord::new(
    "Kavu Climber",
    "2063f31e-d972-411e-a265-1d409153b49c",
    "Rob Alexander",
    // Five mana for a 3/3 and a card, which is what green paid to stop
    // running out of gas.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Kavu"], 3, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// INV 193 — Kavu Lair
pub(in crate::card::sets) static KAVU_LAIR: CardRecord = CardRecord::new(
    "Kavu Lair",
    "f4581b53-23a0-4ca6-a77c-97d79e7a6570",
    "Chippy",
    // It draws for whoever played the big creature, so in a deck of small
    // ones it is a gift to the opponent.
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_ability(AbilityDef::triggered(
        "Whenever a creature with power 4 or greater enters, its controller draws a card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerAtLeast(4),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(1),
        },
    )),
);

// INV 194 — Kavu Titan
pub(in crate::card::sets) static KAVU_TITAN: CardRecord = CardRecord::new(
    "Kavu Titan",
    "2c5fb86d-1d9a-4da2-bb5b-4266faa20197",
    "Todd Lockwood",
    // The card that defined the cycle: a two-mana 2/2 or a five-mana 5/5
    // trampler, and never a dead draw at either end.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Kavu"], 2, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{G}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2}{G} (You may pay an additional {2}{G} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with 3 +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::static_ability(
            "If this creature was kicked, it has trample.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&const { abilities::trample() }),
                },
            },
        ),
    ]),
);

// INV 195 — Llanowar Cavalry
pub(in crate::card::sets) static LLANOWAR_CAVALRY: CardRecord = CardRecord::new(
    "Llanowar Cavalry",
    "21d92191-a743-4916-bbe4-5e207e964d9b",
    "Eric Peterson",
    // A 1/4 that attacks and still blocks, for one white mana.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Soldier"], 1, 4).with_ability(
        AbilityDef::activated(
            "{W}: This creature gains vigilance until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::vigilance() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 196 — Llanowar Elite
pub(in crate::card::sets) static LLANOWAR_ELITE: CardRecord = CardRecord::new(
    "Llanowar Elite",
    "3e207863-de68-47e1-8c63-413b5fa48943",
    "Kev Walker",
    // A one-drop that is still a live draw on turn nine, which is the only
    // reason a 1/1 trampler is worth a slot.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Warrior"], 1, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{8}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {8} (You may pay an additional {8} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::trample(),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with five +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 5,
                },
            ),
        ),
    ]),
);

// INV 197 — Llanowar Vanguard
pub(in crate::card::sets) static LLANOWAR_VANGUARD: CardRecord = CardRecord::new(
    "Llanowar Vanguard",
    "72e6ed79-bdfd-49f9-bfa4-be4196880487",
    "Greg Hildebrandt & Tim Hildebrandt",
    // Tapping for four toughness means it can block one thing enormously
    // well, and only if it has not already attacked.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Dryad"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}: This creature gets +0/+4 until end of turn.",
            &[CostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 198 — Might Weaver
pub(in crate::card::sets) static MIGHT_WEAVER: CardRecord = CardRecord::new(
    "Might Weaver",
    "032a4ec7-82ce-4ea0-b0dd-ebc40823a014",
    "Larry Elmore",
    // Trample for red and white, so a wide board stops being answered by
    // one chump blocker.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}: Target red or white creature gains trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::Red),
                        ObjectPredicateDef::Color(ManaColor::White),
                    ]),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::trample() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 199 — Molimo, Maro-Sorcerer
pub(in crate::card::sets) static MOLIMO_MARO_SORCERER: CardRecord = CardRecord::new(
    "Molimo, Maro-Sorcerer",
    "750d3475-ae72-42c1-ae4d-638f8e7c6d1a",
    "Mark Zug",
    // Seven mana for a trampler as big as the mana that cast it, which is
    // green's idea of a reward.
    CardRules::new_creature(mana_cost!("{4}{G}{G}{G}"), &["Elemental", "Sorcerer"], 0, 0)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::static_ability(
                "Molimo's power and toughness are each equal to the number of lands you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::define_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    ),
                },
            ),
        ]),
);

// INV 200 — Nomadic Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOMADIC_ELF: CardRecord = CardRecord::new(
    "Nomadic Elf",
    "3b69e57a-5b19-450c-9cf5-c189e8505781",
    "D. J. Cleland-Hura",
    crate::card::CardRules::unsupported(),
);

// INV 201 — Pincer Spider
pub(in crate::card::sets) static PINCER_SPIDER: CardRecord = CardRecord::new(
    "Pincer Spider",
    "23271658-19ae-420d-beeb-4bed4fdbb891",
    "Dan Frazier",
    // A reach blocker early or a slightly larger one late, which is exactly
    // what a green deck wants from its filler.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Spider"], 2, 3).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{5}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {3} (You may pay an additional {3} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::reach(),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with a +1/+1 counter on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
    ]),
);

// INV 202 — Pulse of Llanowar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULSE_OF_LLANOWAR: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static QUIRION_SENTINEL: CardRecord = CardRecord::new(
    "Quirion Sentinel",
    "2fc639ea-a925-4f1e-879f-b8fcb12bf257",
    "Heather Hudson",
    // The mana arrives once and has to be spent that turn, which makes it
    // a rebate on the next spell rather than a mana creature.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 2, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, add one mana of any color.",
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ),
);

// INV 205 — Quirion Trailblazer
pub(in crate::card::sets) static QUIRION_TRAILBLAZER: CardRecord = CardRecord::new(
    "Quirion Trailblazer",
    "c2b258c1-5fb4-4072-bb32-ad364df1874a",
    "Rebecca Guay",
// Four mana for a land and a small body, which is the price a
    // five-colour deck pays for the land being any colour it wants.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Scout"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you may search your library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &const {
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
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
                    }
                },
            },
        ),
    ),
);

// INV 206 — Restock
pub(in crate::card::sets) static RESTOCK: CardRecord = CardRecord::new(
    "Restock",
    "11a013ff-7c99-445a-b9e0-0fc45036f068",
    "Daren Bader",
    // Two cards back for five mana, and it exiles itself so the loop never
    // closes -- which is the only reason it is legal.
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return two target cards from your graveyard to your hand. Exile Restock.",
        &const {
            [
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                }),
            ]
        },
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex(1)),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                ]
            },
        ),
    )),
);

// INV 207 — Rooting Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTING_KAVU: CardRecord = CardRecord::new(
    "Rooting Kavu",
    "12c25a4c-d93a-402b-999f-0b9919123cc5",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 208 — Saproling Infestation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_INFESTATION: CardRecord = CardRecord::new(
    "Saproling Infestation",
    "8642e530-914c-4149-944a-c4966ee27299",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 209 — Saproling Symbiosis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_SYMBIOSIS: CardRecord = CardRecord::new(
    "Saproling Symbiosis",
    "2bb63748-5c84-43a0-8f17-a2a17f658337",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// INV 210 — Scouting Trek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCOUTING_TREK: CardRecord = CardRecord::new(
    "Scouting Trek",
    "1b882e68-5c03-4ec6-9982-8c3b09847969",
    "Stephanie Law",
    crate::card::CardRules::unsupported(),
);

// INV 211 — Serpentine Kavu
pub(in crate::card::sets) static SERPENTINE_KAVU: CardRecord = CardRecord::new(
    "Serpentine Kavu",
    "699f1fe8-02c6-4d95-9231-3f8aefe603da",
    "Heather Hudson",
    // Five mana for a 4/4, or six for a 4/4 that attacks immediately.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Kavu"], 4, 4).with_ability(
        AbilityDef::activated(
            "{R}: This creature gains haste until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::haste() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 212 — Sulam Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULAM_DJINN: CardRecord = CardRecord::new(
    "Sulam Djinn",
    "7aeab16f-e104-47e7-81c7-b6e0123120d7",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// INV 213 — Tangle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE: CardRecord = CardRecord::new(
    "Tangle",
    "6b37e39c-8aa4-4938-a492-7dac5de98dfb",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// INV 214 — Thicket Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THICKET_ELEMENTAL: CardRecord = CardRecord::new(
    "Thicket Elemental",
    "f80a56ed-3ebb-4e20-bf6a-e27127f762e8",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// INV 215 — Thornscape Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    "Thornscape Apprentice",
    "505da522-73a8-4232-ae1a-d3365f3e598f",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// INV 216 — Thornscape Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_MASTER: CardRecord = CardRecord::new(
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
    "Treefolk Healer",
    "73c6f5c0-686d-4b3a-add7-487f9fff5faa",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// INV 219 — Utopia Tree
pub(in crate::card::sets) static UTOPIA_TREE: CardRecord = CardRecord::new(
    "Utopia Tree",
    "720452e9-3245-4b0e-94b6-843cbcb641a5",
    "Gary Ruddell",
    // Two mana for any colour a turn, which is what a five-colour deck paid
    // before lands could do it.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant"], 0, 2).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ),
);

// INV 220 — Verdeloth the Ancient
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDELOTH_THE_ANCIENT: CardRecord = CardRecord::new(
    "Verdeloth the Ancient",
    "72d5fab1-fa20-4006-b19d-179d36238c9b",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// INV 221 — Verduran Emissary
pub(in crate::card::sets) static VERDURAN_EMISSARY: CardRecord = CardRecord::new(
    "Verduran Emissary",
    "55f3361b-e2e7-4297-85c2-94323f90cc90",
    "Alton Lawson",
    // A 2/3 body with a Shatter attached, which is the rate green pays for
    // answers it could otherwise only sideboard into.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Wizard"], 2, 3).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{G}{R}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{R} (You may pay an additional {1}{R} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, destroy target artifact. It can't be \
             regenerated.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &const { TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked) },
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )]
            },
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ),
    ]),
);

// INV 222 — Vigorous Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIGOROUS_CHARGE: CardRecord = CardRecord::new(
    "Vigorous Charge",
    "af6f57ad-d370-4c81-8da0-c15d87725ab1",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 223 — Wallop
pub(in crate::card::sets) static WALLOP: CardRecord = CardRecord::new(
    "Wallop",
    "45ce5126-e7b1-41ab-9e56-1e12927c4d27",
    "Mike Ploog",
    // Narrower and cheaper: it answers exactly the fliers the two enemy
    // colours were putting on the board.
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target blue or black creature with flying.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// INV 224 — Wandering Stream
pub(in crate::card::sets) static WANDERING_STREAM: CardRecord = CardRecord::new(
    "Wandering Stream",
    "6da5cb6c-253b-44f0-98f9-d75f42c6e14b",
    "Quinton Hoover",
    // Ten life in a five-colour deck and two in anything else, which is the
    // whole domain cycle in one card.
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Domain — You gain 2 life for each basic land type among lands you control.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Scaled(
                &const {
                    ScaledValueDef::new(ValueDef::BasicLandTypesControlled(PlayerRelation::You), 2)
                },
            ),
        },
    )),
);

// INV 225 — Whip Silk
pub(in crate::card::sets) static WHIP_SILK: CardRecord = CardRecord::new(
    "Whip Silk",
    "10566804-fd15-4ef0-ad7d-cc979f4cc8c5",
    "Dave Dorman",
    // Reach matters only against fliers, so being able to pick it back up is
    // what keeps the card from being dead.
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
            AbilityDef::activated(
                "{G}: Return this Aura to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{G}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// INV 226 — Absorb
pub(in crate::card::sets) static ABSORB: CardRecord = CardRecord::new(
    "Absorb",
    "5d6a0f3e-457f-41f5-be26-5fb249874f1a",
    "Andrew Goldhawk",
    // Three mana of two colours for a hard counter and three life, which is
    // what a gold card was allowed to be.
    CardRules::new_instant(mana_cost!("{W}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. You gain 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// INV 227 — Aether Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_RIFT: CardRecord = CardRecord::new(
    "Aether Rift",
    "692c186a-997c-4f7e-a339-bf84884e1019",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 228 — Angelic Shield
pub(in crate::card::sets) static ANGELIC_SHIELD: CardRecord = CardRecord::new(
    "Angelic Shield",
    "5aaa3e4e-4e08-4df2-9e0c-66e15a10fec4",
    "Adam Rex",
    // A point of toughness on every body wins the combats a two-mana
    // enchantment has no business winning, and it can still be cashed in.
    CardRules::new_enchantment(mana_cost!("{W}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice this enchantment: Return target creature to its owner's hand.",
            &[CostDef::SacrificeSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// INV 229 — Armadillo Cloak
pub(in crate::card::sets) static ARMADILLO_CLOAK: CardRecord = CardRecord::new(
    "Armadillo Cloak",
    "9d816f98-6cb6-432c-b0a4-a0eed21658ac",
    "Paolo Parente",
    // Trample plus lifegain on any damage, so the creature wearing it wins
    // races even when the board is stalled.
    CardRules::new_enchantment(mana_cost!("{1}{G}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "Whenever enchanted creature deals damage, you gain that much life.",
                // Any damage, not just combat: a creature that pings or
                // fights while wearing this gains the life too.
                TriggerEventDef::damage_dealt_by(ObjectPredicateDef::AttachedToSource),
                EffectDef::GainLife {
                    // The Aura's controller, which need not be the creature's:
                    // this can be put on something across the table.
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
        ]),
);

// INV 230 — Armored Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMORED_GUARDIAN: CardRecord = CardRecord::new(
    "Armored Guardian",
    "6de5e1bd-1d31-4f9f-b18d-d6f49bc7ef10",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 231 — Artifact Mutation
pub(in crate::card::sets) static ARTIFACT_MUTATION: CardRecord = CardRecord::new(
    "Artifact Mutation",
    "d5eef49c-a80f-4622-ba77-999f9151c841",
    "Greg Staples",
    // Aura Mutation's red-green half: the same trade, aimed at artifacts.
    CardRules::new_instant(mana_cost!("{R}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact. It can't be regenerated. Create X 1/1 green Saproling creature \
         tokens, where X is that artifact's mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
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
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1)
                .with_count(ValueDef::TargetManaValue(TargetIndex::PRIMARY)),
        ]),
    )),
);

// INV 232 — Aura Mutation
pub(in crate::card::sets) static AURA_MUTATION: CardRecord = CardRecord::new(
    "Aura Mutation",
    "38421179-615e-4aba-91a4-503bfee05403",
    "Pete Venters",
    // Two mana to answer an enchantment and get paid what it cost in bodies,
    // which is a rate no single-colour card was offered.
    CardRules::new_instant(mana_cost!("{G}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target enchantment. Create X 1/1 green Saproling creature tokens, where \
         X is that enchantment's mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // X is read after the destruction, from last-known
            // information about the enchantment that just left.
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1)
                .with_count(ValueDef::TargetManaValue(TargetIndex::PRIMARY)),
        ]),
    )),
);

// INV 233 — Aura Shards
pub(in crate::card::sets) static AURA_SHARDS: CardRecord = CardRecord::new(
    "Aura Shards",
    "df4039ef-af72-4267-ade9-fdb7c921279e",
    "Ron Spencer",
// Every creature becomes a Disenchant, which against a deck built on
    // permanents is an answer to every one of them.
    CardRules::new_enchantment(mana_cost!("{1}{G}{W}")).with_ability(AbilityDef::triggered_with_targets(
    "Whenever a creature you control enters, you may destroy target artifact or enchantment.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ]),
        None,
        Some(ZoneKind::Battlefield),
    ),
    &const {
        [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ]))]
    },
    EffectDef::May {
        player: EffectRecipientDef::Controller,
        effect: &const {
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            }
        },
    },
)),
);

// INV 234 — Backlash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BACKLASH: CardRecord = CardRecord::new(
    "Backlash",
    "dadf030d-5451-43fc-bf0c-c1629fdf88ec",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// INV 235 — Barrin's Spite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_S_SPITE: CardRecord = CardRecord::new(
    "Barrin's Spite",
    "6d8ec4dc-c74a-4d49-856e-95703675fe9b",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// INV 236 — Blazing Specter
pub(in crate::card::sets) static BLAZING_SPECTER: CardRecord = CardRecord::new(
    "Blazing Specter",
    "3bd397be-0e61-4f41-b0cf-f0c9d2440da7",
    "Marc Fishman",
    // Evasion, haste, and a card off the top of the hand, which is three
    // things a two-colour aggressive deck wants at once.
    CardRules::new_creature(mana_cost!("{2}{B}{R}"), &["Specter"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player discards a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// INV 237 — Captain Sisay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_SISAY: CardRecord = CardRecord::new(
    "Captain Sisay",
    "d24d441c-f37f-44fe-8a93-f5c89df807e4",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// INV 238 — Cauldron Dance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAULDRON_DANCE: CardRecord = CardRecord::new(
    "Cauldron Dance",
    "8dadcae0-f2b2-487c-bb93-0a2c073044c0",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// INV 239 — Charging Troll
pub(in crate::card::sets) static CHARGING_TROLL: CardRecord = CardRecord::new(
    "Charging Troll",
    "58956099-6b97-4c7b-ab23-9f9b4d50ef95",
    "Dave Dorman",
    // Vigilance and regeneration on one body, so it attacks and still holds
    // the ground behind it.
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Troll"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{G}"))],
        ),
    ]),
);

// INV 240 — Cinder Shade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_SHADE: CardRecord = CardRecord::new(
    "Cinder Shade",
    "b8dd933a-19ed-4d30-a94a-bfb2f66f8f13",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// INV 241 — Coalition Victory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COALITION_VICTORY: CardRecord = CardRecord::new(
    "Coalition Victory",
    "dd8ad3aa-3225-45ae-8343-5991f5b52269",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// INV 242 — Crosis, the Purger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_THE_PURGER: CardRecord = CardRecord::new(
    "Crosis, the Purger",
    "e5f336d8-12a4-482d-8ffd-c205858c72ba",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 243 — Darigaaz, the Igniter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_THE_IGNITER: CardRecord = CardRecord::new(
    "Darigaaz, the Igniter",
    "54dcf5e3-4303-41a3-b54c-24a9d462ce07",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// INV 244 — Dromar, the Banisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_THE_BANISHER: CardRecord = CardRecord::new(
    "Dromar, the Banisher",
    "cfcc3c72-fff5-454c-814c-eb952fd23ba9",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 245 — Dueling Grounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUELING_GROUNDS: CardRecord = CardRecord::new(
    "Dueling Grounds",
    "52760183-bee0-4ce0-96c0-074b88f78980",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 246 — Fires of Yavimaya
pub(in crate::card::sets) static FIRES_OF_YAVIMAYA: CardRecord = CardRecord::new(
    "Fires of Yavimaya",
    "967f1658-8777-46fc-a648-07fb19e46745",
    "Val Mayerik",
    // Haste for the whole board is what made a deck of four-drops beat a
    // deck of answers, and the sacrifice wins the last combat.
    CardRules::new_enchantment(mana_cost!("{1}{R}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures you control have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::haste() }),
            },
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice this enchantment: Target creature gets +2/+2 until end of turn.",
            &[CostDef::SacrificeSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
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

// INV 247 — Frenzied Tilling
pub(in crate::card::sets) static FRENZIED_TILLING: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static GALINA_S_KNIGHT: CardRecord = CardRecord::new(
    "Galina's Knight",
    "11b492d6-5e28-4f4b-942c-080d03cb0e92",
    "David Martin",
    // A gold two-drop that walks past a whole colour, which is what the
    // cycle was for.
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Merfolk", "Knight"], 2, 2)
        .with_ability(abilities::protection_from_color(ManaColor::Red)),
);

// INV 249 — Hanna, Ship's Navigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HANNA_SHIP_S_NAVIGATOR: CardRecord = CardRecord::new(
    "Hanna, Ship's Navigator",
    "83a4e48d-6452-4245-bdad-63fe3263550e",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 250 — Heroes' Reunion
pub(in crate::card::sets) static HEROES_REUNION: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static HORNED_CHEETAH: CardRecord = CardRecord::new(
    "Horned Cheetah",
    "a28ad983-ce91-40b6-a1ce-fe36ec7fbce8",
    "John Matson",
    // The Invasion printing of the same gold creature.
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Cat"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals damage, you gain that much life.",
            // Any damage, not only combat damage, and the amount is
            // read off the event rather than from the creature's power.
            TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// INV 252 — Hunting Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_KAVU: CardRecord = CardRecord::new(
    "Hunting Kavu",
    "8943304a-89c9-48b0-97b4-3e1aa690ca4d",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 253 — Kangee, Aerie Keeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KANGEE_AERIE_KEEPER: CardRecord = CardRecord::new(
    "Kangee, Aerie Keeper",
    "3afd7e8e-4fcc-4003-9791-7baf10ef1880",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// INV 254 — Llanowar Knight
pub(in crate::card::sets) static LLANOWAR_KNIGHT: CardRecord = CardRecord::new(
    "Llanowar Knight",
    "e6c75d89-e432-49aa-a407-555b223b7eff",
    "Heather Hudson",
    // The two-mana version of the same hoser.
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Elf", "Knight"], 2, 2)
        .with_ability(abilities::protection_from_color(ManaColor::Black)),
);

// INV 255 — Lobotomy (reprint)
const LOBOTOMY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::LOBOTOMY,
    "ff307dbb-4ab6-457b-be56-47106864bf61",
    "D. Alexander Gregory",
);

// INV 256 — Meteor Storm
pub(in crate::card::sets) static METEOR_STORM: CardRecord = CardRecord::new(
    "Meteor Storm",
    "36489b24-f8a8-46b6-b879-0a5ce400a6dc",
    "John Avon",
// Four damage for two cards and four mana, which is a rate that only
    // makes sense when the hand is already dead weight.
    CardRules::new_enchantment(mana_cost!("{R}{G}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}{G}, Discard two cards at random: This enchantment deals 4 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}{G}")),
                CostDef::DiscardCardsAtRandom(2),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ),
);

// INV 257 — Noble Panther
pub(in crate::card::sets) static NOBLE_PANTHER: CardRecord = CardRecord::new(
    "Noble Panther",
    "3f327818-8222-4295-8cef-118757b34d17",
    "Matt Cavotta",
    // Colourless activation, so the first strike is available whatever else
    // the turn was spent on.
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Cat"], 3, 3).with_ability(
        AbilityDef::activated(
            "{1}: This creature gains first strike until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::first_strike() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 258 — Ordered Migration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDERED_MIGRATION: CardRecord = CardRecord::new(
    "Ordered Migration",
    "04d83a07-6054-45f1-bdf9-07f2006238d2",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 259 — Overabundance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERABUNDANCE: CardRecord = CardRecord::new(
    "Overabundance",
    "4183e73d-609a-4292-b173-e39eb51949f3",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// INV 260 — Plague Spores
pub(in crate::card::sets) static PLAGUE_SPORES: CardRecord = CardRecord::new(
    "Plague Spores",
    "0d106d56-a688-49cc-8d5d-0279a5a7c0a7",
    "Randy Gallegos",
    // Six mana for two cards' worth of removal, which is what a two-colour
    // deck pays to answer both halves of a board at once.
    CardRules::new_sorcery(mana_cost!("{4}{B}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target nonblack creature and target land. They can't be regenerated.",
        &const {
            [
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ])),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Land,
                )),
            ]
        },
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &const {
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::Union(
                        &const {
                            [
                                ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                                ObjectSetDef::LegalTargets(TargetIndex(1)),
                            ]
                        },
                    )),
                    then: None,
                }
            },
        },
    )),
);

// INV 261 — Pyre Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYRE_ZOMBIE: CardRecord = CardRecord::new(
    "Pyre Zombie",
    "6c030108-2995-4fb0-9b80-efdfdd0f11e0",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// INV 262 — Raging Kavu
pub(in crate::card::sets) static RAGING_KAVU: CardRecord = CardRecord::new(
    "Raging Kavu",
    "27573679-e9e5-4bfc-b5d5-85d4648b01b6",
    "Arnie Swekel",
    // Flash and haste together make it a combat trick that stays on the
    // board, which is what three power for three mana is really selling.
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &["Kavu"], 3, 1)
        .with_abilities(&[abilities::flash(), abilities::haste()]),
);

// INV 263 — Reckless Assault
pub(in crate::card::sets) static RECKLESS_ASSAULT: CardRecord = CardRecord::new(
    "Reckless Assault",
    "ff0f568e-4d3a-40a5-b72a-63040ec5402d",
    "Jeff Easley",
    // Life is the resource being burned, so the enchantment is only as
    // big as the controller's remaining margin.
    CardRules::new_enchantment(mana_cost!("{2}{B}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "{1}, Pay 2 life: This enchantment deals 1 damage to any target.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::PayLife(2)],
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

// INV 264 — Recoil
pub(in crate::card::sets) static RECOIL: CardRecord = CardRecord::new(
    "Recoil",
    "b6a77be3-e3b0-40f5-a470-414bac49da60",
    "Alan Pollack",
    // A bounce that costs the opponent a card either way, which is what
    // makes it removal rather than a delay.
    CardRules::new_instant(mana_cost!("{1}{U}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return target permanent to its owner's hand. Then that player discards a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// INV 265 — Reviving Vapors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVIVING_VAPORS: CardRecord = CardRecord::new(
    "Reviving Vapors",
    "47a23c32-e122-400b-b252-e636ea2e684b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// INV 266 — Riptide Crab
pub(in crate::card::sets) static RIPTIDE_CRAB: CardRecord = CardRecord::new(
    "Riptide Crab",
    "7e42ae1d-62b4-4b19-aafc-f12bdd6fb8cc",
    "David Martin",
    // A blocker that replaces itself, which is the whole plan of a deck
    // that wants the game to go long.
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Crab"], 1, 3).with_abilities(&[
        abilities::vigilance(),
        abilities::dies_trigger(
            "When this creature dies, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// INV 267 — Rith, the Awakener
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_THE_AWAKENER: CardRecord = CardRecord::new(
    "Rith, the Awakener",
    "c30be387-280d-49bd-a3d1-c1636ee931ce",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// INV 268 — Sabertooth Nishoba
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SABERTOOTH_NISHOBA: CardRecord = CardRecord::new(
    "Sabertooth Nishoba",
    "8338c296-cf3f-41d7-b380-3fb4237cb41c",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// INV 269 — Samite Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_ARCHER: CardRecord = CardRecord::new(
    "Samite Archer",
    "07a262d7-6d0c-43d0-89b6-9f46a1a9eb69",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// INV 270 — Seer's Vision
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEER_S_VISION: CardRecord = CardRecord::new(
    "Seer's Vision",
    "0c94618a-808c-4b3c-8f34-45e64d0414d3",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// INV 271 — Shivan Zombie
pub(in crate::card::sets) static SHIVAN_ZOMBIE: CardRecord = CardRecord::new(
    "Shivan Zombie",
    "f4c99269-f730-4d33-bbce-9e855e9ad0fc",
    "Tony Szczudlo",
    // The black-red member of the same cycle.
    CardRules::new_creature(
        mana_cost!("{B}{R}"),
        &["Phyrexian", "Barbarian", "Zombie"],
        2,
        2,
    )
    .with_ability(abilities::protection_from_color(ManaColor::White)),
);

// INV 272 — Simoon (reprint)
const SIMOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::SIMOON,
    "84b1930d-2e4b-472f-98a9-008fd632f3be",
    "Tony Szczudlo",
);

// INV 273 — Sleeper's Robe
pub(in crate::card::sets) static SLEEPER_S_ROBE: CardRecord = CardRecord::new(
    "Sleeper's Robe",
    "3411f0fd-8b85-4d0d-a202-701a24ffac9f",
    "Alan Pollack",
    // Evasion and a card each time it connects, which together are most of
    // what a two-mana Aura can hope to be.
    CardRules::new_enchantment(mana_cost!("{U}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has fear.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: abilities::FEAR_RESTRICTION,
                },
            ),
            AbilityDef::triggered(
                "Whenever enchanted creature deals combat damage to an opponent, you may \
                 draw a card.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::AttachedToSource),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
        ]),
);

// INV 274 — Slinking Serpent
pub(in crate::card::sets) static SLINKING_SERPENT: CardRecord = CardRecord::new(
    "Slinking Serpent",
    "070a7004-5a28-4ccb-8640-ad6b07b51ece",
    "Wayne England",
    // A blue-black creature with forestwalk: gold in cost and hosing a
    // third colour, which is Invasion's whole idea.
    CardRules::new_creature(mana_cost!("{2}{U}{B}"), &["Serpent"], 2, 3)
        .with_ability(abilities::landwalk(BasicLandType::Forest)),
);

// INV 275 — Smoldering Tar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMOLDERING_TAR: CardRecord = CardRecord::new(
    "Smoldering Tar",
    "fcdc55c0-c8ac-49d5-969b-9bf0ee8e696c",
    "David Day",
    crate::card::CardRules::unsupported(),
);

// INV 276 — Spinal Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINAL_EMBRACE: CardRecord = CardRecord::new(
    "Spinal Embrace",
    "692ad1eb-62a3-4560-bf8e-35f7db73c7a3",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// INV 277 — Stalking Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALKING_ASSASSIN: CardRecord = CardRecord::new(
    "Stalking Assassin",
    "ff8cc71f-3070-497f-908f-35aa13a8a857",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// INV 278 — Sterling Grove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STERLING_GROVE: CardRecord = CardRecord::new(
    "Sterling Grove",
    "40b26aa3-8169-4978-9554-bd2fc8e18e3b",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// INV 279 — Teferi's Moat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_MOAT: CardRecord = CardRecord::new(
    "Teferi's Moat",
    "9ed5845c-ef6d-4a7b-b725-b09d3e9bbc17",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// INV 280 — Treva, the Renewer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREVA_THE_RENEWER: CardRecord = CardRecord::new(
    "Treva, the Renewer",
    "4ee67039-6cee-4a2d-b973-570f5060f550",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// INV 281 — Tsabo Tavoc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_TAVOC: CardRecord = CardRecord::new(
    "Tsabo Tavoc",
    "ccbe2539-7a7c-468b-a270-7ca1bdcccb1e",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// INV 282 — Undermine
pub(in crate::card::sets) static UNDERMINE: CardRecord = CardRecord::new(
    "Undermine",
    "2334bc71-5f85-47ff-b393-601a1e746a4e",
    "Massimiliano Frezzato",
    // Three life on top of a counter, which is what makes it a clock rather
    // than just an answer.
    CardRules::new_instant(mana_cost!("{U}{U}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. Its controller loses 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// INV 283 — Urborg Drake
pub(in crate::card::sets) static URBORG_DRAKE: CardRecord = CardRecord::new(
    "Urborg Drake",
    "97d1327e-bf87-423f-8a04-8124e45b9ae0",
    "Sam Wood",
    // A 2/3 flier for three that never gets to stay home, so the deck
    // playing it has to be the one that wanted to attack anyway.
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Drake"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::attacks_each_combat_if_able(),
    ]),
);

// INV 284 — Vicious Kavu
pub(in crate::card::sets) static VICIOUS_KAVU: CardRecord = CardRecord::new(
    "Vicious Kavu",
    "31e9e629-7c25-4d45-aa35-9ba5f95b43cb",
    "Kev Walker",
    // A 2/2 that attacks as a 4/2, so blocking it profitably takes a
    // creature they were not going to trade.
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Kavu"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +2/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
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

// INV 285 — Vile Consumption
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILE_CONSUMPTION: CardRecord = CardRecord::new(
    "Vile Consumption",
    "7f7e5716-77f3-45d2-a40a-f5bf500f6ad7",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// INV 286 — Vodalian Zombie
pub(in crate::card::sets) static VODALIAN_ZOMBIE: CardRecord = CardRecord::new(
    "Vodalian Zombie",
    "f30a5a06-32ce-4d71-b71f-e3e1d8d4511a",
    "Greg Hildebrandt & Tim Hildebrandt",
    // The blue-black member.
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Merfolk", "Zombie"], 2, 2)
        .with_ability(abilities::protection_from_color(ManaColor::Green)),
);

// INV 287 — Void
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOID: CardRecord = CardRecord::new(
    "Void",
    "62dc1df7-b9db-4f5f-a340-08287cd3d9e5",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// INV 288 — Voracious Cobra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VORACIOUS_COBRA: CardRecord = CardRecord::new(
    "Voracious Cobra",
    "9d8c5669-11a9-4d95-8431-7065037f1fb6",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// INV 289 — Wings of Hope
pub(in crate::card::sets) static WINGS_OF_HOPE: CardRecord = CardRecord::new(
    "Wings of Hope",
    "be0d2402-f1ef-4a71-ac01-c7099c4ce54c",
    "Wayne England",
    // Evasion and a body that survives the block it dodges, which is more
    // than most two-mana Auras manage.
    CardRules::new_enchantment(mana_cost!("{W}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+3 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::add_ability(&const { abilities::flying() }),
                    ]),
                },
            ),
        ]),
);

// INV 290 — Yavimaya Barbarian
pub(in crate::card::sets) static YAVIMAYA_BARBARIAN: CardRecord = CardRecord::new(
    "Yavimaya Barbarian",
    "8e17377d-4dad-4144-b0ce-c849636096a2",
    "Don Hazeltine",
    // The red-green member.
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Elf", "Barbarian"], 2, 2)
        .with_ability(abilities::protection_from_color(ManaColor::Blue)),
);

// INV 291 — Yavimaya Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_KAVU: CardRecord = CardRecord::new(
    "Yavimaya Kavu",
    "1872f104-7cf1-41e3-b1b4-ca75c678e08b",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// INV 292 — Stand // Deliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAND_DELIVER: CardRecord = CardRecord::new(
    "Stand // Deliver",
    "be8b338f-6f05-43c6-beeb-c5052cc0d6a9",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 293 — Spite // Malice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITE_MALICE: CardRecord = CardRecord::new(
    "Spite // Malice",
    "054f1845-196f-41c1-9682-042171cccd49",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 294 — Pain // Suffering
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAIN_SUFFERING: CardRecord = CardRecord::new(
    "Pain // Suffering",
    "81be27d6-e16f-4158-b2b6-66a0f3315327",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// INV 295 — Assault // Battery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASSAULT_BATTERY: CardRecord = CardRecord::new(
    "Assault // Battery",
    "0ec6a889-c941-4898-a2f6-4d3863faf535",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// INV 296 — Wax // Wane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAX_WANE: CardRecord = CardRecord::new(
    "Wax // Wane",
    "19859061-f5ec-4b7f-86a1-196f98648e0a",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// INV 297 — Alloy Golem
pub(in crate::card::sets) static ALLOY_GOLEM: CardRecord = CardRecord::new(
    "Alloy Golem",
    "1fb6d6a1-9d71-405b-9c93-1a7f06c67abd",
    "Greg Staples",
    // Six mana for a 4/4 whose only trick is being whatever colour the rest
    // of the deck needed it to be.
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Golem"], 4, 4).with_abilities(&[
        AbilityDef::as_enters(
            "As this creature enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::static_ability(
            "This creature is the chosen color.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_color(ManaTypeDef::ChosenColor),
            },
        ),
    ]),
);

// INV 298 — Bloodstone Cameo
pub(in crate::card::sets) static BLOODSTONE_CAMEO: CardRecord = CardRecord::new(
    "Bloodstone Cameo",
    "f9db32fa-64b2-4ef6-88f2-28e758d420bb",
    "Tony Szczudlo",
    // Three mana for a rock that fixes two colours, which is the rate
    // Invasion charged for the gold deck's mana.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add {B} or {R}.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Black,
            ManaColor::Red,
        ])),
    )),
);

// INV 299 — Chromatic Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROMATIC_SPHERE: CardRecord = CardRecord::new(
    "Chromatic Sphere",
    "920cd17f-9274-443e-906f-c9904f0658d5",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// INV 300 — Crosis's Attendant
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static CROSIS_S_ATTENDANT: CardRecord = CardRecord::new(
    "Crosis's Attendant",
    "45edc18c-2046-4d0e-92fe-a6cf4aaf1c6f",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// INV 301 — Darigaaz's Attendant
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static DARIGAAZ_S_ATTENDANT: CardRecord = CardRecord::new(
    "Darigaaz's Attendant",
    "6f22b575-443a-4c06-8e75-d4140cbd3660",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// INV 302 — Drake-Skull Cameo
pub(in crate::card::sets) static DRAKE_SKULL_CAMEO: CardRecord = CardRecord::new(
    "Drake-Skull Cameo",
    "4a3ce135-9c2f-45bd-b2db-c0e00c50c964",
    "Dan Frazier",
    // The blue-black member of the same cycle.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add {U} or {B}.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Blue,
            ManaColor::Black,
        ])),
    )),
);

// INV 303 — Dromar's Attendant
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static DROMAR_S_ATTENDANT: CardRecord = CardRecord::new(
    "Dromar's Attendant",
    "24936fa9-41a3-4da5-91cf-c28fa45f47c9",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// INV 304 — Juntu Stakes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNTU_STAKES: CardRecord = CardRecord::new(
    "Juntu Stakes",
    "3ab7cf53-f62d-47e1-af70-ab12be0d22e2",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// INV 305 — Lotus Guardian
pub(in crate::card::sets) static LOTUS_GUARDIAN: CardRecord = CardRecord::new(
    "Lotus Guardian",
    "ddfc6396-5377-4ab3-9c10-8abcdeae2aa1",
    "Dana Knutson",
    // Seven mana for a flier that fixes, which only a deck already casting
    // seven-drops could want.
    CardRules::new_creature(mana_cost!("{7}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// INV 306 — Phyrexian Altar
pub(in crate::card::sets) static PHYREXIAN_ALTAR: CardRecord = CardRecord::new(
    "Phyrexian Altar",
    "25158cd5-749b-408c-9ab1-0f83e38730f7",
    "Ron Spears",
    // Any creature becomes any colour of mana, with no tap in the cost, so
    // a board full of tokens is a board full of mana.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "Sacrifice a creature: Add one mana of any color.",
        &[CostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            controller: PlayerRelation::You,
        }],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// INV 307 — Phyrexian Lens
pub(in crate::card::sets) static PHYREXIAN_LENS: CardRecord = CardRecord::new(
    "Phyrexian Lens",
    "6ec9a91d-7af0-44a8-839f-fb9960be0ddd",
    "Matt Cavotta",
    // Life is the filter, so a deck with no other fixing pays for its
    // colours a point at a time.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add one mana of any color.",
        &[CostDef::TapSource, CostDef::PayLife(1)],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// INV 308 — Planar Portal
pub(in crate::card::sets) static PLANAR_PORTAL: CardRecord = CardRecord::new(
    "Planar Portal",
    "24315eaa-ef55-4fd6-9145-e75b3de6f492",
    "Mark Tedin",
    // Twelve mana to turn the whole library into one card, which is a rate
    // that only matters in a game nobody is winning quickly.
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::activated(
        "{6}, {T}: Search your library for a card, put that card into your hand, then shuffle.",
        &[CostDef::Mana(mana_cost!("{6}")), CostDef::TapSource],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 0,
            maximum: ValueDef::Constant(1),
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

// INV 309 — Power Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POWER_ARMOR: CardRecord = CardRecord::new(
    "Power Armor",
    "ed1981dd-c0f3-4e9d-a1f1-8bea823326ef",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// INV 310 — Rith's Attendant
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static RITH_S_ATTENDANT: CardRecord = CardRecord::new(
    "Rith's Attendant",
    "a26e8130-7fe9-4ef4-98af-928814f5b130",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// INV 311 — Seashell Cameo
pub(in crate::card::sets) static SEASHELL_CAMEO: CardRecord = CardRecord::new(
    "Seashell Cameo",
    "9efdbcad-e2e4-4f54-ade5-920b1853109e",
    "Tony Szczudlo",
    // The white-blue member of the same cycle.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add {W} or {U}.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::White,
            ManaColor::Blue,
        ])),
    )),
);

// INV 312 — Sparring Golem
pub(in crate::card::sets) static SPARRING_GOLEM: CardRecord = CardRecord::new(
    "Sparring Golem",
    "d829d9de-83fa-4feb-8efc-0075315163c6",
    "Adam Rex",
    // Colourless, so any deck can have the same awkward attacker nobody
    // wants to gang up on.
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Golem"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets +1/+1 until end of turn \
             for each creature blocking it.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // Counted as the trigger resolves, so a blocker that
                // has already left is not counted and one added by a
                // later effect is.
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(
                        &const {
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::BlockingSource,
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            )
                        },
                    ),
                    ValueDef::CountMatchingObjects(
                        &const {
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::BlockingSource,
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            )
                        },
                    ),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// INV 313 — Tek
pub(in crate::card::sets) static TEK: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static TIGEREYE_CAMEO: CardRecord = CardRecord::new(
    "Tigereye Cameo",
    "25976da8-338d-4f46-b8ea-78a0aa3daa35",
    "Donato Giancola",
    // The green-white member of the same cycle.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add {G} or {W}.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Green,
            ManaColor::White,
        ])),
    )),
);

// INV 315 — Treva's Attendant
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static TREVA_S_ATTENDANT: CardRecord = CardRecord::new(
    "Treva's Attendant",
    "9857af81-fb95-4dc4-b048-9ce4e96d1eca",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// INV 316 — Troll-Horn Cameo
pub(in crate::card::sets) static TROLL_HORN_CAMEO: CardRecord = CardRecord::new(
    "Troll-Horn Cameo",
    "42b1ca6c-6ca0-4b02-885a-58cee3fa2aa8",
    "Donato Giancola",
    // The red-green member of the same cycle.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add {R} or {G}.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Red,
            ManaColor::Green,
        ])),
    )),
);

// INV 317 — Tsabo's Web
pub(in crate::card::sets) static TSABOS_WEB: CardRecord = CardRecord::new(
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
    "Urza's Filter",
    "680c75b1-e766-40be-84d7-2332047bb3de",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// INV 319 — Ancient Spring
pub(in crate::card::sets) static ANCIENT_SPRING: CardRecord = CardRecord::new(
    "Ancient Spring",
    "004eefa4-947b-45fc-b45c-5263bfd763bc",
    "Don Hazeltine",
    // A tapped land that is really a one-shot dual, which is what a
    // five-colour deck paid a turn for.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Blue),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {W}{B}.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::White,
                ManaColor::Black,
            )),
        ),
    ]),
);

// INV 320 — Archaeological Dig
pub(in crate::card::sets) static ARCHAEOLOGICAL_DIG: CardRecord = CardRecord::new(
    "Archaeological Dig",
    "35f55af0-5a46-4900-b3d0-ca796b710e07",
    "Don Hazeltine",
    // It costs no turn to play and only colourless while it lives, so the
    // splash is paid for entirely at the end.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add one mana of any color.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// INV 321 — Coastal Tower
pub(in crate::card::sets) static COASTAL_TOWER: CardRecord = CardRecord::new(
    "Coastal Tower",
    "d115dbff-e35b-495f-a1e3-19651895927e",
    "Don Hazeltine",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// INV 322 — Elfhame Palace
pub(in crate::card::sets) static ELFHAME_PALACE: CardRecord = CardRecord::new(
    "Elfhame Palace",
    "65986555-a5d7-497e-876f-b8d967d6aa5b",
    "Jerry Tiritilli",
    // The green-white tap-land: a turn of tempo for two colours, which is
    // what Invasion's gold deck paid every game.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

// INV 323 — Geothermal Crevice
pub(in crate::card::sets) static GEOTHERMAL_CREVICE: CardRecord = CardRecord::new(
    "Geothermal Crevice",
    "e744b593-13fe-4967-b492-ac02f5815e57",
    "John Avon",
    // The same deal in the other wedge: a turn now for two colours later.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Red),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {B}{G}.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Black,
                ManaColor::Green,
            )),
        ),
    ]),
);

// INV 324 — Irrigation Ditch
pub(in crate::card::sets) static IRRIGATION_DITCH: CardRecord = CardRecord::new(
    "Irrigation Ditch",
    "977f1b44-166c-4faf-8a7b-d431707e90ce",
    "Rob Alexander",
    // One colour every turn or two colours once, which is the choice the
    // whole cycle asks.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::White),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {G}{U}.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Green,
                ManaColor::Blue,
            )),
        ),
    ]),
);

// INV 325 — Keldon Necropolis
pub(in crate::card::sets) static KELDON_NECROPOLIS: CardRecord = CardRecord::new(
    "Keldon Necropolis",
    "4f0cccf6-b79b-4fff-89aa-801341598532",
    "Franz Vohwinkel",
    // A land that turns spare creatures into damage, which is a win condition
    // no removal spell can answer.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{4}{R}, {T}, Sacrifice a creature: Keldon Necropolis deals 2 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{4}{R}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )]
            },
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// INV 326 — Salt Marsh
pub(in crate::card::sets) static SALT_MARSH: CardRecord = CardRecord::new(
    "Salt Marsh",
    "ed64934b-0e64-4b2f-97aa-c3fb7e6ce0b0",
    "Jerry Tiritilli",
    // The blue-black member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// INV 327 — Shivan Oasis
pub(in crate::card::sets) static SHIVAN_OASIS: CardRecord = CardRecord::new(
    "Shivan Oasis",
    "9841f7e8-162c-44a3-96f3-af944fce15d1",
    "Rob Alexander",
    // The red-green member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// INV 328 — Sulfur Vent
pub(in crate::card::sets) static SULFUR_VENT: CardRecord = CardRecord::new(
    "Sulfur Vent",
    "22c66ed6-55fb-4c65-aac4-26d9cc3053b8",
    "Edward P. Beard, Jr.",
    // Black up front, and the two colours it does not make when it goes.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Black),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {U}{R}.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Blue,
                ManaColor::Red,
            )),
        ),
    ]),
);

// INV 329 — Tinder Farm
pub(in crate::card::sets) static TINDER_FARM: CardRecord = CardRecord::new(
    "Tinder Farm",
    "989b5901-aeb0-4a48-8c53-3b0ec0e0deba",
    "Rob Alexander",
    // Green now, or the two colours a green deck splashes for.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Green),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {R}{W}.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Red,
                ManaColor::White,
            )),
        ),
    ]),
);

// INV 330 — Urborg Volcano
pub(in crate::card::sets) static URBORG_VOLCANO: CardRecord = CardRecord::new(
    "Urborg Volcano",
    "c76f346c-ae34-4f5f-8e3b-6c77b0c4d530",
    "Tony Szczudlo",
    // The black-red member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
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
