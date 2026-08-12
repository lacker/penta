use std::collections::HashSet;

use super::{CardRecord, SET_MODULES, y1993, y1994, y2004, y2011, y2012, y2013};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityPredicateDef, AbilityProcedureDef, AddManaEffectDef,
    AlternativeCastKindDef, AppliedEffectDef, BasicLandType, CardPrinting, CardPrintingId,
    CardStructure, CardSupertype, CardType, ComparisonDef, ConditionDef, DeclarativeAbilityDef,
    DoubleFacedKind, EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef,
    ImplementationStatus, KeywordAbility, LibraryPlacement, ManaColor, ManaRestrictionDef,
    ManaSelectionDef, ManaSpendEffectDef, ObjectPredicateDef, ObjectQueryDef, PlayActionKind,
    PlayRestriction, PlayerRelation, ReplacementEffectDef, ReplacementEventDef, SpellForm,
    TargetPredicate, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZoneMoveCauseDef, cards,
};
use crate::{
    CardDefinitionId, CardPartId, CardSet, Format, ManaCost, ModeId, PlayOptionId, TargetSlotId,
};

fn standard_records() -> Vec<&'static CardRecord> {
    let mut records = SET_MODULES
        .iter()
        .filter(|module| Format::IsdRtrStandard.allows_set(module.set))
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

mod metadata_composition_mana;
mod registry_integrity;
mod runtime_boundary;
mod runtime_support;
mod source_organization;
