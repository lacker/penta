//! The Brothers' War Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BRC",
    slug: "the-brothers-war-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// BRC 16 — Machine God's Effigy
// Audit: unsupported — The copy model can add Artifact and an ability, but CopyExceptionsDef can
// only add card types. This copy must remove Creature after copying its target; adding Artifact
// alone leaves an artifact creature and changes the printed "It's not a creature" behavior.
pub(in crate::card::sets) static MACHINE_GOD_S_EFFIGY_16: CardRecord = CardRecord::new(
    "Machine God's Effigy",
    "637f69c2-ba24-42d1-9345-8ebdb04b6904",
    "Martin de Diego Sádaba",
    crate::card::CardRules::unsupported(),
);

// BRC 51 — Urza's Workshop
pub(in crate::card::sets) static URZA_S_WORKSHOP_51: CardRecord = CardRecord::new(
    "Urza's Workshop",
    "1e01c1e2-f657-4a24-b8b0-561911c4e754",
    "Alexander Forssberg",
    CardRules::new_land(&["Urza's"]).with_abilities(&[
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated_mana_if("Metalcraft — {T}: Add {C} for each Urza's land you control. Activate only if you control three or more artifacts.", &[CostDef::TapSource], &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 3 }, EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_variable_amount(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Urza's"))]), &[ZoneKind::Battlefield], PlayerRelation::You)))))
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &MACHINE_GOD_S_EFFIGY_16,
    &URZA_S_WORKSHOP_51,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
