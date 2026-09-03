//! Betrayers of Kamigawa cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CounterKind, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRuleDef, PlayerSetDef, ResolvedEffectDurationDef, TriggerEventDef,
    ValueDef, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// BOK 76 — Okiba-Gang Shinobi
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OKIBA_GANG_SHINOBI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cd9297e-301e-4e70-af9b-3218eacacf8d"),
    "Okiba-Gang Shinobi",
    crate::card::CardArt::new("5cd9297e-301e-4e70-af9b-3218eacacf8d", "Mark Zug"),
    crate::card::CardSet::BetrayersOfKamigawa,
    crate::card::CardRules::unsupported(),
);

// BOK 104 — Fumiko the Lowblood
pub(in crate::card::sets) static FUMIKO_THE_LOWBLOOD: CardRecord =
    CardRecord::new(
        PrintingAnchor::scryfall("482678b8-bce6-4847-9f43-1761d61645d8"),
        "Fumiko the Lowblood",
        CardArt::new("482678b8-bce6-4847-9f43-1761d61645d8", "Michael Sutfin"),
        CardSet::BetrayersOfKamigawa,
        CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Samurai"], 3, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::bushido(ValueDef::CountMatchingObjects(
                    &const {
                        crate::card::ObjectQueryDef::new(
                            ObjectPredicateDef::Attacking,
                            &[crate::card::ZoneKind::Battlefield],
                        )
                    },
                ))
                .override_text("Bushido X, where X is the number of attacking creatures."),
                AbilityDef::static_ability(
                    "Creatures your opponents control attack each combat if able.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[crate::card::ZoneKind::Battlefield],
                            crate::card::PlayerRelation::Opponent,
                        ),
                        effect: AppliedEffectDef::add_ability(
                            &abilities::attacks_each_combat_if_able(),
                        ),
                    },
                ),
            ]),
    );
// BOK 154 — Mirror Gallery
pub(in crate::card::sets) static MIRROR_GALLERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00beba34-54cc-4a30-8424-71a1215647a6"),
    "Mirror Gallery",
    CardArt::new("00beba34-54cc-4a30-8424-71a1215647a6", "Scott M. Fischer"),
    CardSet::BetrayersOfKamigawa,
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::static_ability(
        "The \"legend rule\" doesn't apply.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::All),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                PlayerRuleDef::LegendRuleDoesNotApply,
            )),
        },
    )),
);

// BOK 163 — Umezawa's Jitte
pub(in crate::card::sets) static UMEZAWAS_JITTE: CardRecord = CardRecord::new_with_legacy_id(
    2188,
    "Umezawa's Jitte",
    CardArt::new("d4ecc3ef-a9f2-4c4c-9c8d-b4a0e6ba4ac2", "Christopher Moeller"),
    CardSet::BetrayersOfKamigawa,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever equipped creature deals combat damage, put two charge counters on Umezawa's Jitte.",
                // Damage to a blocker counts as readily as damage to the
                // player: the clause names no recipient at all.
                TriggerEventDef::combat_damage_dealt_by(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::modal_activated(
                "Remove a charge counter from Umezawa's Jitte: Choose one —\n• Equipped creature gets +2/+2 until end of turn.\n• Target creature gets -1/-1 until end of turn.\n• You gain 2 life.",
                &[AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("charge"),
                    amount: 1,
                }],
                // The three modes of the Jitte's counter-spending ability. Each is a clause
                // of its own with its own targets, chosen as the ability is activated.
                &[
                    AbilityDef::spell(
                        "Equipped creature gets +2/+2 until end of turn",
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::AttachedPermanent,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "Target creature gets -1/-1 until end of turn",
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(-1),
                                ValueDef::Constant(-1),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::spell(
                        "You gain 2 life",
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ),
                ],
                1,
                1,
                false,
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &OKIBA_GANG_SHINOBI,
    &FUMIKO_THE_LOWBLOOD,
    &MIRROR_GALLERY,
    &UMEZAWAS_JITTE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
