//! Kamigawa: Neon Dynasty attachment edge cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef,
    ValueDef, abilities, cards,
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
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            abilities::reconfigure(
                mana_cost!("{R}"),
                "Reconfigure {R} ({R}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&RABBIT_BATTERY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
