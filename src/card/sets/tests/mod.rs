use std::collections::HashSet;

use super::{CardRecord, SET_MODULES, y1993, y1996, y2002, y2012};
use crate::card::{
    AbilityDef, AbilityOperationDef, AbilityPredicateDef, AbilityProcedureDef, AbilityProgramDef,
    AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, BasicLandType, CardChoiceSourceDef,
    CardPrintingId, CardSupertype, CardType, CharacteristicOperationDef, ComparisonDef,
    ConditionDef, CostDef, DamagePreventionCapacityDef, DamagePreventionFollowUpDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, DeclarativeAbilityDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, EffectRecipientSetDef, ImplementationStatus,
    KeywordAbility, ManaColor, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, SetOperationDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZoneMoveCauseDef, ZonePlacement, cards,
};
use crate::{CardDefinitionId, CardSet, Format, ManaCost};

fn standard_records() -> Vec<&'static CardRecord> {
    let allowed_sets = Format::IsdM14Standard
        .set_definition()
        .expect("Standard is set based")
        .allowed_sets;
    let mut records = SET_MODULES
        .iter()
        .filter(|module| allowed_sets.contains(&module.set))
        .flat_map(|module| module.cards.iter().copied())
        .collect::<Vec<_>>();
    records.sort_unstable_by_key(|record| record.id());
    records
}

fn is_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        })
}

mod catalog_report;
mod isd_m14_coverage;
mod old_school_coverage;
mod registry_integrity;
mod runtime_boundary;
mod runtime_support;
mod source_organization;
