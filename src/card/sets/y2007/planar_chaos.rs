//! Planar Chaos cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AggregateOperationDef, AppliedEffectDef,
    BasicLandType, CardArt, CardNameDef, CardRules, CardSet, CardSupertype, CardType,
    ChoiceVisibilityDef, ChooseDef, CounterKind, EffectDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    ObjectSetFilterDef, ObjectValueAggregateDef, ObjectValueDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// PLC 21 — Voidstone Gargoyle
pub(in crate::card::sets) static VOIDSTONE_GARGOYLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("583741a1-0faf-4fb3-8536-b9b1cc8a3b6f"),
    "Voidstone Gargoyle",
    CardArt::new("583741a1-0faf-4fb3-8536-b9b1cc8a3b6f", "Terese Nielsen"),
    CardSet::PlanarChaos,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Gargoyle"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::choose_nonland_card_name("As this creature enters, choose a nonland card name."),
        abilities::cannot_cast_spells_with_chosen_name(
            "Spells with the chosen name can't be cast.",
        ),
        abilities::cannot_activate_abilities_with_chosen_name(
            "Activated abilities of sources with the chosen name can't be activated.",
        ),
    ]),
);

// PLC 25 — Mana Tithe
pub(in crate::card::sets) static MANA_TITHE: CardRecord = CardRecord::new_with_legacy_id(
    2114,
    "Mana Tithe",
    CardArt::new("7d48d622-f397-4f31-b1a5-0c23f60aa71c", "Martina Pilcerova"),
    CardSet::PlanarChaos,
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

// PLC 31 — Sunlance
pub(in crate::card::sets) static SUNLANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46144ca5-aa81-4314-a1e5-1716f8565d70"),
    "Sunlance",
    CardArt::new("46144ca5-aa81-4314-a1e5-1716f8565d70", "Volkan Baǵa"),
    CardSet::PlanarChaos,
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
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )),
);

// PLC 71 — Extirpate
pub(in crate::card::sets) static EXTIRPATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2608b5fc-3b58-4b02-aef1-35d885b16b01"),
    "Extirpate",
    CardArt::new("2608b5fc-3b58-4b02-aef1-35d885b16b01", "Jon Foster"),
    CardSet::PlanarChaos,
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
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(Binding!(
                            "extirpate_graveyard"
                        )),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Graveyard],
                                PlayerSetDef::One(PlayerRefDef::OwnerOf(ObjectRefDef::Binding(
                                    Binding!("extirpate_target"),
                                ))),
                            )),
                            object: ObjectSetFilterDef::Predicate(
                                &ObjectPredicateDef::NameEquals(CardNameDef::Object(
                                    ObjectRefDef::Binding(Binding!("extirpate_target")),
                                )),
                            ),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: usize::MAX,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                                "extirpate_graveyard"
                            ))),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                    }),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(Binding!("extirpate_hand")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerSetDef::One(PlayerRefDef::OwnerOf(ObjectRefDef::Binding(
                                    Binding!("extirpate_target"),
                                ))),
                            )),
                            object: ObjectSetFilterDef::Predicate(
                                &ObjectPredicateDef::NameEquals(CardNameDef::Object(
                                    ObjectRefDef::Binding(Binding!("extirpate_target")),
                                )),
                            ),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: usize::MAX,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                                "extirpate_hand"
                            ))),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                    }),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(Binding!("extirpate_library")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Library],
                                PlayerSetDef::One(PlayerRefDef::OwnerOf(ObjectRefDef::Binding(
                                    Binding!("extirpate_target"),
                                ))),
                            )),
                            object: ObjectSetFilterDef::Predicate(
                                &ObjectPredicateDef::NameEquals(CardNameDef::Object(
                                    ObjectRefDef::Binding(Binding!("extirpate_target")),
                                )),
                            ),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: usize::MAX,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                                "extirpate_library"
                            ))),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                    }),
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
    PrintingAnchor::scryfall("53c1910b-9475-4551-b9a0-4b24511a6f98"),
    "Fungal Behemoth",
    CardArt::new("53c1910b-9475-4551-b9a0-4b24511a6f98", "Mark Tedin"),
    CardSet::PlanarChaos,
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
pub(in crate::card::sets) static URBORG_TOMB_OF_YAWGMOTH: CardRecord =
    CardRecord::new_with_legacy_id(
        261,
        "Urborg, Tomb of Yawgmoth",
        CardArt::new("19e1224f-82cb-4f41-8739-f880cba61bbb", "John Avon"),
        CardSet::PlanarChaos,
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
    &SUNLANCE,
    &EXTIRPATE,
    &FUNGAL_BEHEMOTH,
    &URBORG_TOMB_OF_YAWGMOTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
