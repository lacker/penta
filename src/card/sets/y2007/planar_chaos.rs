//! Planar Chaos cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// PLC 25 — Mana Tithe
pub(in crate::card::sets) static MANA_TITHE: CardRecord = CardRecord::new_with_legacy_id(
    2114,
    "Mana Tithe",
    CardArt::new("7d48d622-f397-4f31-b1a5-0c23f60aa71c", "Martina Pilcerova"),
    CardSet::PlanarChaos,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {1}.",
        &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
        abilities::counter_target_unless_paid(ValueDef::Constant(1)),
    )),
);

// PLC 165 — Urborg, Tomb of Yawgmoth
pub(in crate::card::sets) static URBORG_TOMB_OF_YAWGMOTH: CardRecord =
    CardRecord::new_with_legacy_id(
        261,
        "Urborg, Tomb of Yawgmoth",
        CardArt::new("19e1224f-82cb-4f41-8739-f880cba61bbb", "John Avon"),
        CardSet::PlanarChaos,
        CardRules::new_land(&[])
            .with_supertype(CardSupertype::Legendary)
            .with_ability(AbilityDef::static_ability(
                "Each land is a Swamp in addition to its other land types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Swamp]),
                },
            )),
    );

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MANA_TITHE, &URBORG_TOMB_OF_YAWGMOTH];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
