use serde_json::Value;

use crate::card::{
    AlternativeCastKindDef, CardType, CardTypeSet, EffectDef, ReplacementChoiceDef,
    ReplacementEventDef, TurnKindDef, ZonePlacement,
};
use crate::{
    CardCatalog, CardDefinitionId, CardPartId, GameObjectId, ManaCost, ObjectCharacteristics,
    PlayerId,
};

use super::super::decision_offers::effect_choice_visibility;
use super::super::{
    AbilitySourceRef, ApplicableBeginTurnReplacement, CastOffer, CastOfferCost, CastSourceZone,
    DecisionContinuation, DecisionKind, DecisionObservation, DecisionOption,
    DecisionOrderSemantics, DecisionPreference, DecisionVisibility, DecisionZone,
    DeferredBeginTurnEffect, GameEvent, PendingDecision, PendingTrigger, SacrificeFollowup,
    ScopedEffect, Target, TriggerPlacementBatch,
};
use super::model::{
    AbilityLocator, AbilitySourceSnapshot, ApplicableBeginTurnReplacementSnapshot,
    CounterKindSnapshot, DecisionCardOriginSnapshot, DecisionCardSnapshot,
    DecisionContinuationSnapshot, DecisionOptionSnapshot, DecisionPreferenceSnapshot,
    DecisionStateSnapshot, DecisionZoneSnapshot, DeferredBeginTurnEffectSnapshot,
    DetachedCardSnapshot, DiscardChoiceSnapshot, EffectContinuationSnapshot,
    PendingTriggerSnapshot, PregameAbilityActionSnapshot, ReplacementEffectContextSnapshot,
    ReplacementEffectLocator, TargetSnapshot, TriggerPlacementBatchSnapshot, TurnKindSnapshot,
    ZoneMoveCauseSnapshot, ZonePlacementSnapshot,
};
mod option;
use option::parse_option;

use super::procedure::{
    draw_replacement_snapshot_allowing, parse_draw_replacement, parse_pending_procedure,
    pending_procedure_snapshot,
};
use super::semantics::{
    ability_locator, ability_locator_for_origin, ability_target_defs, catalog_ability,
    catalog_replacement_effect, catalog_scoped_effect, replacement_effect_locator_matches_source,
    replacement_effects, resolved_replacement_effect_locator, scoped_effect_snapshot_in_catalog,
};
use super::stack::{binding_snapshot, parse_binding_snapshot};
use super::stack::{
    detached_stack_snapshot_allowing, effect_resolution_context_snapshot,
    object_reference_requires_hidden_rebinding, parse_detached_stack,
    parse_effect_resolution_context, parse_target, parse_target_selection, referenced_object_ids,
    resolution_context_referenced_object_ids, stack_ability_snapshot_allowing,
    target_selection_snapshot, target_selections_referenced_object_ids, target_snapshot,
    trigger_capture_has_unrebindable_hidden_reference,
    trigger_capture_has_unrebindable_hidden_reference_except,
};
use super::{
    DeclarativeAbilityDef, Game, ReplacementEffectContext, ReplacementEffectDef, ZoneMoveCause,
    ability_origin_from_snapshot, ability_origin_snapshot, applicable_replacement_snapshot, array,
    bool_field, card, copiable_ability_snapshot, expiration_snapshot, field,
    object_characteristics_from_snapshot, object_characteristics_snapshot,
    parse_applicable_replacement, parse_copiable_ability, parse_expiration, parse_text_change_kind,
    parse_zone_kind, seat_value, str_field, text_change_kind_snapshot, u32_field, usize_field,
    zone_kind_snapshot,
};

include!("decision/observation.rs");

include!("decision/card_origins.rs");

include!("decision/snapshot_continuation.rs");

include!("decision/parse_observation.rs");

include!("decision/continuation.rs");
include!("decision/special_action_continuation.rs");
include!("decision/battlefield_entry_continuation.rs");

include!("decision/validation.rs");

mod begin_turn;
mod support;

#[allow(clippy::wildcard_imports)]
use begin_turn::*;
pub(super) use support::decision_referenced_object_ids;
#[allow(clippy::wildcard_imports)]
use support::*;
pub(super) use support::{
    mana_cost_from_snapshot, mana_cost_snapshot, parse_pending_trigger, pending_trigger_snapshot,
};

include!("decision/explicit_payment.rs");
