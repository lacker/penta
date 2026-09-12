//! Commander Masters cards cataloged for legend-rule coverage.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CastTimingPermissionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CMM",
    slug: "commander-masters",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CMM 707 — Sliver Gravemother
// Audit: unsupported — Needs encore, including a graveyard-granted variable-cost ability and its attacking token copies with delayed sacrifice.
pub(in crate::card::sets) static SLIVER_GRAVEMOTHER: CardRecord = CardRecord::new(
    "Sliver Gravemother",
    "9f5d253e-9eb2-423c-90ee-68f27ec6bf88",
    "Chris Rahn",
    CardRules::unsupported(),
);

// CMM 750 — Skittering Cicada
pub(in crate::card::sets) static SKITTERING_CICADA_750: CardRecord = CardRecord::new(
    "Skittering Cicada",
    "4a430137-70d9-45fc-acaa-87b29ea0d588",
    "Denis Zhbankov",
    CardRules::new_creature(mana_cost!("{3}"), &["Insect"], 2, 2).with_abilities(&[
abilities::flash(),
AbilityDef::static_ability("You may cast colorless spells as though they had flash.", EffectDef::StaticApply { recipient: EffectRecipientDef::Controller, effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(CastTimingPermissionDef::new(ObjectPredicateDef::ColorCount(0)))) }),
AbilityDef::triggered("Whenever you cast a colorless spell, until end of turn, this creature gains trample and gets +X/+X, where X is that spell's mana value.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::ColorCount(0), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::trample()), AppliedEffectDef::modify_power_toughness(ValueDef::ObjectManaValue(ObjectRefDef::TriggeringObject), ValueDef::ObjectManaValue(ObjectRefDef::TriggeringObject))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SLIVER_GRAVEMOTHER, &SKITTERING_CICADA_750];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
