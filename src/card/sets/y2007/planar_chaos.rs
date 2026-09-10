//! Planar Chaos cards cataloged as cross-format rules-engine test cases.

use super::CardRecord;
use super::PrintingRecord;
use crate::ControlDurationDef;
use crate::TargetIndex;
use crate::TurnStepDef;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::BasicLandType;
use crate::card::CardNameDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

// PLC 21 — Voidstone Gargoyle
pub(in crate::card::sets) static VOIDSTONE_GARGOYLE: CardRecord = CardRecord::new(
    "Voidstone Gargoyle",
    "583741a1-0faf-4fb3-8536-b9b1cc8a3b6f",
    "Terese Nielsen",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Gargoyle"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::as_enters(
            "As this creature enters, choose a nonland card name.",
            crate::card::ReplacementEffectDef::BindOutput {
                binding: Binding!("voidstone_gargoyle_name"),
                effect: &abilities::choose_card_name_as_enters(
                    crate::card::CardNameSetDef::NonlandCardNames,
                ),
            },
        ),
        abilities::cannot_cast_spells_with_name(
            "Spells with the chosen name can't be cast.",
            CardNameDef::Binding(Binding!("voidstone_gargoyle_name")),
        ),
        abilities::cannot_activate_abilities_with_name(
            "Activated abilities of sources with the chosen name can't be activated.",
            CardNameDef::Binding(Binding!("voidstone_gargoyle_name")),
        ),
    ]),
);

// PLC 25 — Mana Tithe
pub(in crate::card::sets) static MANA_TITHE: CardRecord = CardRecord::new(
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
        abilities::counter_target_unless_paid(&[crate::CostDef::GenericMana(ValueDef::Constant(
            1,
        ))]),
    )),
);

// PLC 26 — Mesa Enchantress
pub(in crate::card::sets) static MESA_ENCHANTRESS: CardRecord = CardRecord::new(
    "Mesa Enchantress",
    "4037d6de-f30b-483c-83a8-9a4e2978f7fc",
    "Randy Gallegos",
    // The same engine in white and without the shroud, which in practice means
    // it draws one card before it dies.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Druid"], 0, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an enchantment spell, you may draw a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// PLC 31 — Sunlance
pub(in crate::card::sets) static SUNLANCE: CardRecord = CardRecord::new(
    "Sunlance",
    "46144ca5-aa81-4314-a1e5-1716f8565d70",
    "Volkan Baǵa",
    // One white mana for three damage, priced by the one thing white removal
    // is never allowed to answer: the mirror.
    CardRules::new_sorcery(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Sunlance deals 3 damage to target nonwhite creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                // "Nonwhite" is the absence of white, not the presence of
                // another colour: a colourless creature is a legal target.
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
            ]),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(3),
        ),
    )),
);

// PLC 70 — Enslave
pub(in crate::card::sets) static ENSLAVE: CardRecord = CardRecord::new(
    "Enslave",
    "6c6283e1-e4f1-4ff6-be01-b66ab623e0ac",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_enchantment(mana_cost!("{4}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "You control enchanted creature.",
                EffectDef::gain_control(
                    EffectRecipientDef::AttachedPermanent,
                    PlayerRefDef::EffectController,
                    ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                ),
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, enchanted creature deals 1 damage to its owner.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::damage_from(
                    ObjectRefDef::AttachedToSource,
                    EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    ValueDef::Constant(1),
                ),
            ),
        ]),
);

// PLC 71 — Extirpate
pub(in crate::card::sets) static EXTIRPATE: CardRecord = CardRecord::new(
    "Extirpate",
    "2608b5fc-3b58-4b02-aef1-35d885b16b01",
    "Jon Foster",
CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Choose target card in a graveyard other than a basic land card. Search its owner's graveyard, hand, and library for all cards with the same name as that card and exile them. Then that player shuffles.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                    CardSupertype::Basic,
                )),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(Binding!("extirpate_target")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::Sequence(&[
                    abilities::search_and_exile(
                        ZoneKind::Graveyard,
                        Binding!("extirpate_target"),
                    ),
                    abilities::search_and_exile(
                        ZoneKind::Hand,
                        Binding!("extirpate_target"),
                    ),
                    abilities::search_and_exile(
                        ZoneKind::Library,
                        Binding!("extirpate_target"),
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                            ObjectRefDef::Binding(Binding!("extirpate_target")),
                        )),
                    },
                ]),
            }),
        ),
    ]),
);

// PLC 128 — Fungal Behemoth
pub(in crate::card::sets) static FUNGAL_BEHEMOTH: CardRecord = CardRecord::new(
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
        abilities::suspend(
            "Suspend X—{X}{G}{G}. X can't be 0.",
            &crate::card::SuspendAbilityDef::chosen_x(&[crate::CostDef::Mana(mana_cost!("{X}{G}{G}"))], 1),
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
    &VOIDSTONE_GARGOYLE,
    &MANA_TITHE,
    &MESA_ENCHANTRESS,
    &SUNLANCE,
    &ENSLAVE,
    &EXTIRPATE,
    &FUNGAL_BEHEMOTH,
    &URBORG_TOMB_OF_YAWGMOTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
