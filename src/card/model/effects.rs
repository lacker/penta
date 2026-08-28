mod children;
mod composites;
mod conditions;
mod emblem_creation;
mod likelihood;
mod object_collections;
mod replacements;
mod token_creation;
mod triggers;
mod turn_structure;
mod values;

pub(crate) use children::child_effects;
pub use composites::*;
pub use conditions::*;
pub use likelihood::*;
pub use object_collections::*;
pub use replacements::*;
pub use triggers::*;
pub use turn_structure::*;
pub use values::*;

use super::payments::{EffectPaymentDef, PayOrDef};
use crate::Format;
use crate::ids::{
    AdditionalCostObjectIndex, ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex,
};

use super::{
    AbilityDef, AddManaEffectDef, AlternativeCastKindDef, BasicLandType, CardTypeSet, ColorSet,
    ComparisonDef, CounterKind, DeclarativeAbilityDef, KeywordAbility, ManaColor, ManaCost,
    ManaTypeDef, ObjectPredicateDef, PlayActionKind, PlayerRelation, TokenCharacteristics,
    TriggerConditionDef, ZoneKind, ZonePlacement,
};

// Effect subjects, lifetimes, and event matchers form the shared vocabulary
// consumed by both resolving and continuously applied effects below.
include!("effects/zone_change_references.rs");
include!("effects/recipients_and_matchers.rs");
include!("effects/zone_change_event_matchers.rs");
include!("effects/applied.rs");
include!("effects/vocabulary.rs");
include!("effects/damage.rs");
include!("effects/definition.rs");
include!("effects/shorthands.rs");
