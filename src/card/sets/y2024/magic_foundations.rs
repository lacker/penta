//! FDN card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardRules, CardSet, CardSupertype,
    CharacteristicOperationDef, CreatureTypeSetDef, EffectDef, EffectRecipientDef,
    ExilePlayDurationDef, ObjectPredicateDef, PowerToughnessOperationDef,
    ResolvedEffectDurationDef, SetOperationDef, TriggerConditionDef, TriggerEventDef, ValueDef,
    abilities,
};
use crate::mana_cost;

// FDN 18 — Inspiring Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRING_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::MagicFoundations,
    "Inspiring Paladin",
    "0763be06-25b2-4d6b-ab33-a1af85aeb443",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// FDN 114 — Treetop Snarespinner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREETOP_SNARESPINNER: CardRecord = CardRecord::new(
    crate::card::CardSet::MagicFoundations,
    "Treetop Snarespinner",
    "88e68fa3-159d-49a6-8ac6-afc9bd6f1718",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// FDN 129 — Leyline Axe
pub(in crate::card::sets) static LEYLINE_AXE: CardRecord = CardRecord::new(
    CardSet::MagicFoundations,
    "Leyline Axe",
    "b9c03336-a321-4c06-94d1-809f328fabd8",
    "Edgar Sánchez Hidalgo",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has double strike and trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                        AppliedEffectDef::add_ability(&abilities::double_strike()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// FDN 195 — Fanatical Firebrand (reprint)
const FANATICAL_FIREBRAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2018::rivals_of_ixalan::FANATICAL_FIREBRAND,
    "d1296316-7781-4e98-95e6-7020648be6a5",
    "Wayne Reynolds",
);

// FDN 200 — Goblin Surprise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SURPRISE: CardRecord = CardRecord::new(
    crate::card::CardSet::MagicFoundations,
    "Goblin Surprise",
    "527dd5d4-5f72-40bb-8a9d-1f5ac3f81e2e",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// FDN 330 — Kellan, Planar Trailblazer
pub(in crate::card::sets) static KELLAN_PLANAR_TRAILBLAZER: CardRecord = CardRecord::new(
    CardSet::MagicFoundations,
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
                &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
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
                &[AbilityCostDef::Mana(mana_cost!("{2}{R}"))],
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
