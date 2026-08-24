//! Betrayers of Kamigawa cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ResolvedEffectDurationDef, TriggerEventDef, ValueDef, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// BOK 76 — Okiba-Gang Shinobi
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OKIBA_GANG_SHINOBI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cd9297e-301e-4e70-af9b-3218eacacf8d"),
    "Okiba-Gang Shinobi",
    crate::card::CardArt::new("5cd9297e-301e-4e70-af9b-3218eacacf8d", "Mark Zug"),
    crate::card::CardSet::BetrayersOfKamigawa,
    crate::card::CardRules::unsupported(),
);

// BOK 163 — Umezawa's Jitte
/// The three modes of the Jitte's counter-spending ability. Each is a clause
/// of its own with its own targets, chosen as the ability is activated.
static JITTE_MODES: &[AbilityDef] = &[
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
];

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
                JITTE_MODES,
                1,
                1,
                false,
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&OKIBA_GANG_SHINOBI, &UMEZAWAS_JITTE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
