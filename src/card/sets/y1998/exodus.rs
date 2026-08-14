//! Exodus cards cataloged as attachment edge cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardType, EffectDef, EffectDurationDef, EffectRecipientDef, ObjectPredicateDef, ZoneKind,
    cards,
};
use crate::{TargetIndex, mana_cost};

static DOMINATING_LICID_END: AbilityDef = AbilityDef::special_action(
    "You may pay {U} to end this effect.",
    &[ZoneKind::Battlefield],
    &[AbilityCostDef::Mana(mana_cost!("{U}"))],
    EffectDef::EndAuraEffect,
);

// EXO 30 — Dominating Licid
pub(in crate::card::sets) static DOMINATING_LICID: CardRecord = CardRecord::new(
    cards::DOMINATING_LICID,
    "Dominating Licid",
    CardArt::new(
        "e3e03323-43e8-4ddc-a874-211a97fd7648",
        "Heather Hudson",
    ),
    CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Licid"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{U}{U}, {T}: This creature loses this ability and becomes an Aura enchantment with enchant creature. Attach it to target creature. You may pay {U} to end this effect.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::BecomeAuraAndAttach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                end: &DOMINATING_LICID_END,
            },
        ),
        AbilityDef::static_ability(
            "You control enchanted creature.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect: AppliedEffectDef::ControlBySourceController,
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DOMINATING_LICID];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
