//! Planar Chaos cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::ControlDurationDef;
use crate::ObjectRefDef;
use crate::PlayerRefDef;
use crate::TurnStepDef;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AggregateOperationDef, AppliedEffectDef,
    BasicLandType, CardRules, CardSet, CardSupertype, CardType, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, ObjectValueAggregateDef,
    ObjectValueDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// PLC 25 — Mana Tithe
pub(in crate::card::sets) static MANA_TITHE: CardRecord = CardRecord::new(
    CardSet::PlanarChaos,
    "Mana Tithe",
    "7d48d622-f397-4f31-b1a5-0c23f60aa71c",
    "Martina Pilcerova",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {1}.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(ValueDef::Constant(1)),
    )),
);

// PLC 26 — Mesa Enchantress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MESA_ENCHANTRESS: CardRecord = CardRecord::new(
    crate::card::CardSet::PlanarChaos,
    "Mesa Enchantress",
    "4037d6de-f30b-483c-83a8-9a4e2978f7fc",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// PLC 31 — Sunlance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNLANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::PlanarChaos,
    "Sunlance",
    "46144ca5-aa81-4314-a1e5-1716f8565d70",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// PLC 70 — Enslave
pub(in crate::card::sets) static ENSLAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::PlanarChaos,
    "Enslave",
    "6c6283e1-e4f1-4ff6-be01-b66ab623e0ac",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_enchantment(mana_cost!("{4}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "You control enchanted creature.",
                EffectDef::GainControl {
                    object: EffectRecipientDef::AttachedPermanent,
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, enchanted creature deals 1 damage to its owner.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::DealDamageFrom {
                    source: ObjectRefDef::AttachedToSource,
                    recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// PLC 128 — Fungal Behemoth
pub(in crate::card::sets) static FUNGAL_BEHEMOTH: CardRecord = CardRecord::new(
    CardSet::PlanarChaos,
    "Fungal Behemoth",
    "53c1910b-9475-4551-b9a0-4b24511a6f98",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Fungus"], 0, 0).with_abilities(&[
        AbilityDef::static_ability(
            "Fungal Behemoth's power and toughness are each equal to the number of +1/+1 counters on creatures you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        select: ObjectValueDef::Counters(CounterKind::PlusOnePlusOne),
                        operation: AggregateOperationDef::Sum,
                    }),
                    ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        select: ObjectValueDef::Counters(CounterKind::PlusOnePlusOne),
                        operation: AggregateOperationDef::Sum,
                    }),
                ),
            },
        ),
        abilities::suspend_x(
            "Suspend X—{X}{G}{G}. X can't be 0.",
            &mana_cost!("{X}{G}{G}"),
            1,
        ),
        AbilityDef::triggered_with_targets(
            "Whenever a time counter is removed from this card while it's exiled, you may put a +1/+1 counter on target creature.",
            TriggerEventDef::CountersRemoved {
                object: ObjectPredicateDef::Source,
                kind: CounterKind::named("time"),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            },
        )
        .with_source_zones(&[ZoneKind::Exile]),
    ]),
);

// PLC 165 — Urborg, Tomb of Yawgmoth
pub(in crate::card::sets) static URBORG_TOMB_OF_YAWGMOTH: CardRecord = CardRecord::new(
    CardSet::PlanarChaos,
    "Urborg, Tomb of Yawgmoth",
    "19e1224f-82cb-4f41-8739-f880cba61bbb",
    "John Avon",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Each land is a Swamp in addition to its other land types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Swamp]),
            },
        )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MANA_TITHE,
    &MESA_ENCHANTRESS,
    &SUNLANCE,
    &ENSLAVE,
    &FUNGAL_BEHEMOTH,
    &URBORG_TOMB_OF_YAWGMOTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
