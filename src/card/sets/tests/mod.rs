use std::collections::HashSet;

use super::{CardRecord, SET_MODULES, y1993, y1994, y1996, y2002, y2004, y2011, y2012, y2013};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityOperationDef, AbilityPredicateDef, AbilityProcedureDef,
    AbilityProgramDef, AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, BasicLandType,
    CardChoiceSourceDef, CardPrinting, CardPrintingId, CardStructure, CardSupertype, CardType,
    CharacteristicOperationDef, ComparisonDef, ConditionDef, DamagePreventionCapacityDef,
    DamagePreventionFollowUpDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeclarativeAbilityDef, DoubleFacedKind, EffectDef, EffectExecutionDef, EffectPaymentDef,
    EffectRecipientDef, EffectRecipientSetDef, ImplementationStatus, KeywordAbility, ManaColor,
    ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PayOrDef, PlayActionKind, PlayRestriction, PlayerRefDef,
    PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, SetOperationDef, SpellForm, TargetPredicate,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZoneMoveCauseDef,
    ZonePlacement, cards,
};
use crate::{
    CardDefinitionId, CardPartId, CardSet, Format, ManaCost, ModeId, PlayOptionId, TargetSlotId,
};

fn standard_records() -> Vec<&'static CardRecord> {
    let mut records = SET_MODULES
        .iter()
        .filter(|module| Format::IsdDgmStandard.allows_set(module.set))
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    records.sort_unstable_by_key(|record| record.id);
    records
}

fn is_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        })
}

fn printings_for_set(set: CardSet) -> Vec<CardPrinting> {
    let module = SET_MODULES.iter().find(|module| module.set == set).unwrap();
    module
        .cards
        .iter()
        .map(|record| CardPrinting::new(record.id, set))
        .chain(
            module
                .additional_printings
                .iter()
                .map(|record| record.printing(set)),
        )
        .collect()
}

mod catalog_report;
mod isd_dgm_coverage;
mod metadata_composition_mana;
mod old_school_coverage;
mod registry_integrity;
mod runtime_boundary;
mod runtime_support;
mod source_organization;
