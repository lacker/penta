use crate::card::ZoneRelativePositionDef;
use crate::card::catalog::{EffectSubjectKind, GrantedAbilityValidationError};
use crate::card::{
    AbilityOperationDef, AbilityProcedureDef, AbilityProgramDef, AbilityTargetDef,
    AbilityTargetPredicate, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef,
    ArrivalAttachmentDef, AttackDefenderScopeDef, AttackRestrictionDef,
    BattlefieldEntryChoiceDestinationDef, BlockRestrictionDef, BlockRestrictionMatchDef,
    CardNameDef, CardNameSetDef, CharacteristicOperationDef, ConditionDef, CostAdjustmentDef,
    CostAmountDef, CostDef, CostModificationDef, DamageEventMatcherDef,
    DamagePreventionCapacityDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeclarativeAbilityDef, EffectDef, EffectPaymentDef, EffectRecipientDef, EffectRecipientSetDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PerPlayerSelectionDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, ReplacementChoiceDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, ScalarChoiceListDef,
    SpellCostModificationDef, StackObjectEventDef, StackTargetFilterDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneChangeEventMatcherDef, ZoneKind,
};
use crate::{Binding, TargetIndex};

include!("targeting/references.rs");
include!("targeting/resolving_applied_effect.rs");
// What a trigger event itself may name, split from the references above for
// the source-size budget: the questions an event asks about its own object,
// player, and damage matcher are a boundary of their own.
include!("targeting/trigger_references.rs");
include!("targeting/effect_predicate_references.rs");
include!("targeting/effect_references.rs");
include!("targeting/bindable_outputs.rs");
include!("targeting/replacement_effect_references.rs");
include!("targeting/trigger_zones.rs");

#[derive(Clone, Copy)]
enum RecipientExpectation {
    Any,
    Object,
    Player,
}

fn duration_is_valid_for_applied_effect(
    duration: ResolvedEffectDurationDef,
    effect: AppliedEffectDef,
) -> bool {
    !duration.contains(ResolvedEffectDurationDef::UntilNextMatchingCast)
        || matches!(
            effect,
            AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(_))
        )
}

include!("targeting/shapes.rs");
include!("targeting/restriction_shapes.rs");
include!("targeting/target_predicate_zones.rs");
include!("targeting/zone_move_shapes.rs");
include!("targeting/created_token_continuations.rs");
include!("targeting/replacement_effect_shapes.rs");
include!("targeting/effect_shapes.rs");
