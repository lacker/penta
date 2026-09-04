//! Betrayers of Kamigawa cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ResolvedEffectDurationDef, TriggerEventDef, ValueDef, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// BOK 44 — Ninja of the Deep Hours
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NINJA_OF_THE_DEEP_HOURS: CardRecord = CardRecord::new(
    crate::card::CardSet::BetrayersOfKamigawa,
    "Ninja of the Deep Hours",
    "367a67c7-54db-4336-b55a-3fa27625172a",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// BOK 76 — Okiba-Gang Shinobi
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OKIBA_GANG_SHINOBI: CardRecord = CardRecord::new(
    crate::card::CardSet::BetrayersOfKamigawa,
    "Okiba-Gang Shinobi",
    "5cd9297e-301e-4e70-af9b-3218eacacf8d",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// BOK 163 — Umezawa's Jitte
pub(in crate::card::sets) static UMEZAWAS_JITTE: CardRecord = CardRecord::new(
    CardSet::BetrayersOfKamigawa,
    "Umezawa's Jitte",
    "3b6e5956-f795-451b-bb24-56462d1ced27",
    "Christopher Moeller",
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
    &NINJA_OF_THE_DEEP_HOURS,
    &OKIBA_GANG_SHINOBI,
    &UMEZAWAS_JITTE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
