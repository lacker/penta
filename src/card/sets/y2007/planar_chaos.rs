//! Planar Chaos cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet, CardSupertype,
    CardType, EffectDef, EffectDurationDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    ZoneKind, cards,
};

// PLC 165 — Urborg, Tomb of Yawgmoth
pub(in crate::card::sets) static URBORG_TOMB_OF_YAWGMOTH: CardRecord = CardRecord::new(
    cards::URBORG_TOMB_OF_YAWGMOTH,
    "Urborg, Tomb of Yawgmoth",
    CardArt::new("19e1224f-82cb-4f41-8739-f880cba61bbb", "John Avon"),
    CardSet::PlanarChaos,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Each land is a Swamp in addition to its other land types.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::AddLandTypes(&[BasicLandType::Swamp]),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&URBORG_TOMB_OF_YAWGMOTH];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
