//! FDN card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::PowerToughnessOperationDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SetOperationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "FDN",
    slug: "magic-foundations",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// FDN 18 — Inspiring Paladin
pub(in crate::card::sets) static INSPIRING_PALADIN: CardRecord = CardRecord::new(
    "Inspiring Paladin",
    "0763be06-25b2-4d6b-ab33-a1af85aeb443",
    "Valera Lutfullina",
    // First strike only while attacking, which is the trade for handing it
    // out to the whole team: it never helps the blocks.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "During your turn, this creature has first strike. (It deals combat damage before \
             creatures without first strike.)",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
        // A second printed ability rather than a rider: it reaches every
        // creature with a counter, and this one only if something has put a
        // counter on it.
        AbilityDef::static_ability(
            "During your turn, creatures you control with +1/+1 counters on them have first \
             strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
    ]),
);

// FDN 114 — Treetop Snarespinner
pub(in crate::card::sets) static TREETOP_SNARESPINNER: CardRecord = CardRecord::new(
    "Treetop Snarespinner",
    "88e68fa3-159d-49a6-8ac6-afc9bd6f1718",
    "Steve Ellis",
    // Reach and deathtouch already answer anything that attacks into it, so
    // the counters are what a stalled board turns spare mana into.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spider"], 1, 4).with_abilities(&[
        abilities::reach(),
        abilities::deathtouch(),
        AbilityDef::activated_with_targets(
            "{2}{G}: Put a +1/+1 counter on target creature you control. Activate only as a \
             sorcery.",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// FDN 129 — Leyline Axe
pub(in crate::card::sets) static LEYLINE_AXE: CardRecord = CardRecord::new(
    "Leyline Axe",
    "b9c03336-a321-4c06-94d1-809f328fabd8",
    "Edgar Sánchez Hidalgo",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::begin_game_on_battlefield(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has double strike and trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::double_strike()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// FDN 195 — Fanatical Firebrand (reprint)
const FANATICAL_FIREBRAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2018::rivals_of_ixalan::FANATICAL_FIREBRAND,
    "d1296316-7781-4e98-95e6-7020648be6a5",
    "Wayne Reynolds",
);

// FDN 200 — Goblin Surprise
pub(in crate::card::sets) static GOBLIN_SURPRISE: CardRecord = CardRecord::new(
    "Goblin Surprise",
    "527dd5d4-5f72-40bb-8a9d-1f5ac3f81e2e",
    "Kevin Sidharta",
    // Held up as a combat trick either way: the tokens are the mode you
    // take when the attack did not happen.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control get +2/+0 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Create two 1/1 red Goblin creature tokens.",
                EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
                    .with_amount(2),
            ),
        ],
    )),
);

// FDN 330 — Kellan, Planar Trailblazer
pub(in crate::card::sets) static KELLAN_PLANAR_TRAILBLAZER: CardRecord = CardRecord::new(
    "Kellan, Planar Trailblazer",
    "0e413f37-b59a-4302-86d3-2abce81edc78",
    "Aaron J. Riley",
// One mana for a 2/1 that grows into what the rest of the turn's mana
    // has nothing better to do with.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Faerie", "Scout"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "{1}{R}: If Kellan is a Scout, it becomes a Human Faerie Detective and gains \"Whenever \
                 Kellan deals combat damage to a player, exile the top card of your library. You may play \
                 that card this turn.\"",
                &[CostDef::Mana(mana_cost!("{1}{R}"))],
                EffectDef::IfCondition {
                    // Each activation asks what Kellan is now, so the two have to be paid in
                    // order and neither does anything twice.
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype("Scout"),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        // "It becomes a Human Faerie Detective": a set rather than an addition, so
                        // the Scout it was is gone and the second activation has something to ask
                        // about.
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                                SetOperationDef::Set(CreatureTypeSetDef::named(&["Human", "Faerie", "Detective"])),
                            )),
                            // The Detective's own clause, granted rather than printed: a card exiled
                            // off the top and playable for the turn, which is what the second
                            // activation is paying to turn on.
                            AppliedEffectDef::add_ability(&AbilityDef::triggered(
                                "Whenever Kellan deals combat damage to a player, exile the top card of your library. You may \
                                 play that card this turn.",
                                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                                EffectDef::ExileTopOfLibraryToPlay {
                                    player: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(1),
                                    free: false,
                                    face_down: false,
                                    duration: ExilePlayDurationDef::ThisTurn,
                                    spend_any_color: false,
                                    play_condition: None,
                                    cast_only: false,
                                },
                            )),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
            AbilityDef::activated(
                "{2}{R}: If Kellan is a Detective, it becomes a 3/2 Human Faerie Rogue and gains double \
                 strike.",
                &[CostDef::Mana(mana_cost!("{2}{R}"))],
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype("Detective"),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                                PowerToughnessOperationDef::SetBase {
                                    power: ValueDef::Constant(3),
                                    toughness: ValueDef::Constant(2),
                                },
                            )),
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                                SetOperationDef::Set(CreatureTypeSetDef::named(&["Human", "Faerie", "Rogue"])),
                            )),
                            AppliedEffectDef::add_ability(&abilities::double_strike()),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
        ]),
);

// FDN 528 — Undying Malice (reprint)
const UNDYING_MALICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2021::innistrad_crimson_vow::UNDYING_MALICE,
    "97b3cf11-e352-4ee1-8c03-13898f576ef9",
    "Igor Kieryluk",
);

// FDN 596 — Shipwreck Dowser (reprint)
const SHIPWRECK_DOWSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2020::core_set_2021::SHIPWRECK_DOWSER,
    "1f20fe3d-792a-4030-a25c-e81b48b2bcb4",
    "Caroline Gariba",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &INSPIRING_PALADIN,
    &TREETOP_SNARESPINNER,
    &LEYLINE_AXE,
    &GOBLIN_SURPRISE,
    &KELLAN_PLANAR_TRAILBLAZER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    FANATICAL_FIREBRAND_REPRINT,
    UNDYING_MALICE_REPRINT,
    SHIPWRECK_DOWSER_REPRINT,
];
