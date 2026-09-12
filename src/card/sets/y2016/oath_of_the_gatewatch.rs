//! Oath of the Gatewatch card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostObjectIndex;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SumValueDef;
use crate::card::TriggerConditionDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "OGW",
    slug: "oath-of-the-gatewatch",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// OGW 26 — Make a Stand
pub(in crate::card::sets) static MAKE_A_STAND: CardRecord = CardRecord::new(
    "Make a Stand",
    "30cace63-91ca-493c-b67f-740fbbf06370",
    "Magali Villeneuve",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell(
        "Creatures you control get +1/+0 and gain indestructible until \
         end of turn. (Damage and effects that say \"destroy\" don't \
         destroy them.)",
        EffectDef::Apply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::indestructible()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// OGW 63 — Sphinx of the Final Word
pub(in crate::card::sets) static SPHINX_OF_THE_FINAL_WORD: CardRecord = CardRecord::new(
    "Sphinx of the Final Word",
    "866f92a3-2738-4e0f-adda-3ff9227dc17a",
    "Lius Lasahido",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Sphinx"], 5, 5).with_abilities(&[
        AbilityDef::static_ability(
            "This spell can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        )
        .with_source_zones(&[ZoneKind::Stack]),
        abilities::flying(),
        abilities::hexproof(),
        AbilityDef::static_ability(
            "Instant and sorcery spells you control can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        &[ZoneKind::Stack],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        ),
    ]),
);

// OGW 91 — Untamed Hunger
pub(in crate::card::sets) static UNTAMED_HUNGER: CardRecord = CardRecord::new(
    "Untamed Hunger",
    "595deb86-b6cf-4e4f-a6cf-f5ff128b720d",
    "Willian Murai",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+1 and has menace. (It can't be \
                 blocked except by two or more creatures.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::menace()),
                    ]),
                },
            ),
        ]),
);

// OGW 141 — Pulse of Murasa
pub(in crate::card::sets) static PULSE_OF_MURASA: CardRecord = CardRecord::new(
    "Pulse of Murasa",
    "c0c8057f-b45b-4f67-90cd-c808b5e9cbfa",
    "Matt Stewart",
// Either graveyard, so it also answers an opponent's reanimation target
    // by handing the card back to them rather than leaving it where it is.
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature or land card from a graveyard to its owner's hand. You gain 6 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(6),
            },
        ]),
    )),
);

// OGW 145 — Tajuru Pathwarden
pub(in crate::card::sets) static TAJURU_PATHWARDEN: CardRecord = CardRecord::new(
    "Tajuru Pathwarden",
    "b073e75b-b432-41a5-a71e-d169fecf774f",
    "Victor Adame Minguez",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Elf", "Warrior", "Ally"], 5, 4)
        .with_abilities(&[abilities::vigilance(), abilities::trample()]),
);

// OGW 151 — Ayli, Eternal Pilgrim
pub(in crate::card::sets) static AYLI_ETERNAL_PILGRIM: CardRecord = CardRecord::new(
    "Ayli, Eternal Pilgrim",
    "e7b5893d-6df6-4cae-ae70-d02d443d1740",
    "Cynthia Sheppard",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Kor", "Cleric"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::activated(
                "{1}, Sacrifice another creature: You gain life equal to the \
                 sacrificed creature's toughness.",
                &[
                    CostDef::Mana(mana_cost!("{1}")),
                    CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ])),
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(
                            AdditionalCostObjectIndex::PRIMARY,
                        )),
                        select: ObjectValueDef::Toughness,
                        operation: AggregateOperationDef::Sum,
                    }),
                },
            ),
            AbilityDef::activated_with_targets(
                "{1}{W}{B}, Sacrifice another creature: Exile target nonland \
                 permanent. Activate only if you have at least 10 life more \
                 than your starting life total.",
                &[
                    CostDef::Mana(mana_cost!("{1}{W}{B}")),
                    CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ])),
                ],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            )
            .with_activation_condition(&TriggerConditionDef::ValueComparison(
                &ValueComparisonDef {
                    left: ValueDef::LifeTotal(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Sum(&SumValueDef {
                        left: ValueDef::StartingLifeTotal,
                        right: ValueDef::Constant(10),
                    }),
                },
            )),
        ]),
);

// OGW 183 — Wastes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WASTES: CardRecord = CardRecord::new(
    "Wastes",
    "7019912c-bd9b-4b96-9388-400794909aa1",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MAKE_A_STAND,
    &SPHINX_OF_THE_FINAL_WORD,
    &UNTAMED_HUNGER,
    &PULSE_OF_MURASA,
    &TAJURU_PATHWARDEN,
    &AYLI_ETERNAL_PILGRIM,
    &WASTES,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
