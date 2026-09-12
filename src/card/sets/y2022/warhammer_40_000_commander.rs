//! Warhammer 40,000 Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "40K",
    slug: "warhammer-40-000-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// 40K 8 — Marneus Calgar
// Audit: unsupported — The battlefield event stream is per object and cannot raise one entry trigger per simultaneous batch of tokens.
pub(in crate::card::sets) static MARNEUS_CALGAR_8: CardRecord = CardRecord::new(
    "Marneus Calgar",
    "e7517e8e-b424-4731-ba9d-6132bdefa6bf",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// 40K 51★ — Psychomancer
pub(in crate::card::sets) static PSYCHOMANCER_51_: CardRecord = CardRecord::new(
    "Psychomancer",
    "32c7cab2-4fc9-4d53-ba67-902f72799d20",
    "Alex Konstad",
    CardRules::new_artifact_creature(mana_cost!("{1}{B}"), &["Necron", "Wizard"], 1, 1).with_abilities(&[
abilities::flying(),
AbilityDef::triggered_with_targets("Harbinger of Despair — Whenever this creature or another nontoken artifact you control is put into a graveyard from the battlefield or is put into exile from the battlefield, target opponent loses 1 life and you gain 1 life.", TriggerEventDef::AnyOf(&[TriggerEventDef::zone_changed(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Source, ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::Token), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)), TriggerEventDef::zone_changed(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Source, ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::Token), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])]), Some(ZoneKind::Battlefield), Some(ZoneKind::Exile))]), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::Sequence(&[EffectDef::LoseLife { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), amount: ValueDef::Constant(1) }, EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) }]))
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MARNEUS_CALGAR_8,
    &PSYCHOMANCER_51_,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
