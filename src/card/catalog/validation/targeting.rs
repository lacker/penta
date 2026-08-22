use crate::card::catalog::{EffectSubjectKind, GrantedAbilityValidationError};
use crate::card::{
    AbilityOperationDef, AbilityProcedureDef, AbilityProgramDef, AbilityTargetDef,
    AbilityTargetPredicate, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef,
    AttackDefenderScopeDef, BattlefieldEntryChoiceDestinationDef, CharacteristicOperationDef,
    ConditionDef, DamageEventMatcherDef, DamagePreventionCapacityDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DeclarativeAbilityDef, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, EffectRecipientDef, EffectRecipientSetDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    PowerToughnessOperationDef, ReplacementChoiceDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, ScalarChoiceListDef, TriggerConditionDef, TriggerEventDef, ValueDef,
    ZoneKind,
};
use crate::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};

include!("targeting/references.rs");
include!("targeting/effect_references.rs");

#[derive(Clone, Copy)]
enum RecipientExpectation {
    Any,
    Object,
    Player,
}

include!("targeting/shapes.rs");
include!("targeting/effect_shapes.rs");
