//! Modern Horizons 2 cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet, CardSupertype,
    CardType, EffectDef, EffectDurationDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    ZoneKind, cards,
};

// MH2 261 — Yavimaya, Cradle of Growth
pub(in crate::card::sets) static YAVIMAYA_CRADLE_OF_GROWTH: CardRecord = CardRecord::new(
    cards::YAVIMAYA_CRADLE_OF_GROWTH,
    "Yavimaya, Cradle of Growth",
    CardArt::new("4e4b6e22-93b2-4896-bba5-0ceaa5d8ea3c", "Sarah Finnigan"),
    CardSet::ModernHorizons2,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Each land is a Forest in addition to its other land types.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::AddLandTypes(&[BasicLandType::Forest]),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&YAVIMAYA_CRADLE_OF_GROWTH];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
