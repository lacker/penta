use crate::card::ZoneRelativePositionDef;
use crate::card::catalog::{EffectSubjectKind, GrantedAbilityValidationError};
use crate::card::{
    AbilityOperationDef, AbilityProcedureDef, AbilityProgramDef, AbilityTargetDef,
    AbilityTargetPredicate, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef,
    ArrivalAttachmentDef, AttackDefenderScopeDef, AttackRestrictionDef,
    BattlefieldEntryChoiceDestinationDef, BlockRestrictionDef, BlockRestrictionMatchDef,
    CharacteristicOperationDef, ConditionDef, CostModificationDef, DamageEventMatcherDef,
    DamagePreventionCapacityDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeclarativeAbilityDef, EffectDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef,
    EffectRecipientSetDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, ReplacementChoiceDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, ScalarChoiceListDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneChangeEventMatcherDef, ZoneKind,
};
use crate::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};

include!("targeting/references.rs");
// What a trigger event itself may name, split from the references above for
// the source-size budget: the questions an event asks about its own object,
// player, and damage matcher are a boundary of their own.
include!("targeting/trigger_references.rs");
include!("targeting/effect_references.rs");

#[derive(Clone, Copy)]
enum RecipientExpectation {
    Any,
    Object,
    Player,
}

include!("targeting/shapes.rs");
include!("targeting/effect_shapes.rs");
