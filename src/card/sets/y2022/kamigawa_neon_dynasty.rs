//! Kamigawa: Neon Dynasty cards cataloged as attachment edge cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, EffectDef, EffectDurationDef,
    EffectRecipientDef, ObjectPredicateDef, TriggerEventDef, ValueDef, abilities, cards,
};
use crate::mana_cost;

// NEO 157 — Rabbit Battery
pub(in crate::card::sets) static RABBIT_BATTERY: CardRecord = CardRecord::new(
    cards::RABBIT_BATTERY,
    "Rabbit Battery",
    CardArt::new("5d33a5b7-797b-4079-8d62-edd124c0fb5a", "Justyna Dura"),
    CardSet::KamigawaNeonDynasty,
    CardRules::new_artifact_creature(mana_cost!("{R}"), &["Equipment", "Rabbit"], 1, 1)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has haste.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(1),
                            toughness: ValueDef::Constant(1),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::haste()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            abilities::reconfigure(
                mana_cost!("{R}"),
                "Reconfigure {R} ({R}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

// NEO 163 — Simian Sling
pub(in crate::card::sets) static SIMIAN_SLING: CardRecord = CardRecord::new(
    cards::SIMIAN_SLING,
    "Simian Sling",
    CardArt::new("37a00f95-d563-4d51-a5f2-af139261921a", "Nicholas Elias"),
    CardSet::KamigawaNeonDynasty,
    CardRules::new_artifact_creature(mana_cost!("{R}"), &["Equipment", "Monkey"], 1, 1)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::triggered(
                "Whenever this creature or equipped creature becomes blocked, it deals 1 damage to defending player.",
                TriggerEventDef::BecomesBlocked(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Source,
                    ObjectPredicateDef::AttachedToSource,
                ])),
                EffectDef::DealDamageFrom {
                    source: EffectRecipientDef::TriggeringObject,
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::reconfigure(
                mana_cost!("{2}"),
                "Reconfigure {2} ({2}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&RABBIT_BATTERY, &SIMIAN_SLING];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
