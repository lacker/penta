//! Mirrodin Besieged cards cataloged as attachment edge cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef, ValueDef, abilities, cards,
};
use crate::{TargetIndex, mana_cost};

// MBS 115 — Mortarpod
pub(in crate::card::sets) static MORTARPOD: CardRecord = CardRecord::new(
    cards::MORTARPOD,
    "Mortarpod",
    CardArt::new("fbd23da5-421f-41d0-bb60-59560da7dece", "Eric Deschamps"),
    CardSet::MirrodinBesieged,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(cards::GERM_TOKEN_0_0_BLACK),
            AbilityDef::static_ability(
                "Equipped creature gets +0/+1 and has \"Sacrifice this creature: This creature deals 1 damage to any target.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "Sacrifice this creature: This creature deals 1 damage to any target.",
                            &[AbilityCostDef::SacrificeSource],
                            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                            EffectDef::DealDamage {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(1),
                            },
                        )),
                    ]),
                },
            ),
            abilities::equip(mana_cost!("{2}"), "Equip {2}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MORTARPOD];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
