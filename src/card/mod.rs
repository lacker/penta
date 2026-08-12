//! Card definitions, rules metadata, and the built-in multi-format card corpus.
//!
//! The corpus is grouped by release year and set in [`sets`]. Canonical card
//! records own rules and implementation status, while reprint and alternate-art
//! records provide distinct physical-printing identities without duplicating
//! gameplay definitions.

use std::sync::LazyLock;

pub mod abilities;
pub mod cards;

mod behavior;
mod catalog;
mod characteristics;
mod creature_types;
mod model;
mod record;
pub(crate) mod sets;

pub(crate) use record::{AbilityPolicyHint, CardAbilityBinding};

pub use behavior::CardBehavior;
pub use catalog::{CardCatalog, CatalogError, GrantedAbilityValidationError};
pub use characteristics::{CharacteristicContext, CharacteristicError, applicable_part_ids};
pub use creature_types::CREATURE_TYPES;
pub use model::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityEffectDef,
    AbilityPredicateDef, AbilityProcedureDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivatedAbilityDef, AddManaEffectDef, AdditionalCostDef, AlternateSpellKind,
    AlternativeCastAbilityDef, AlternativeCastKindDef, AlternativeCastManaCostDef,
    AlternativeCostDef, AnimationDef, AppliedEffectDef, AttachedAbilityDef, BasicLandType,
    BattlefieldEntryModificationDef, CardAbilityList, CardArt, CardChoiceSourceDef,
    CardComposition, CardDefinition, CardEffectStatus, CardPart, CardPrinting, CardPrintingId,
    CardRules, CardSet, CardStructure, CardSupertype, CardType, CardTypeSet, ColorSet,
    ComparisonDef, ConditionDef, ConditionalValueDef, CostDef, CountConditionDef, CounterKind,
    CreatureStats, DeclarativeAbilityDef, DiscardSelectionDef, DividedTotal, DoubleFacedKind,
    EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef, HybridPair,
    ImplementationStatus, KeywordAbility, LikelihoodDef, ManaColor, ManaCost, ManaCostParseError,
    ManaCostParseErrorKind, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef,
    MeldComponentDef, MeldRecipeDef, MeldResultDef, ModalSpellDef, ModeDef, ModeSetDef,
    ObjectPredicateDef, ObjectQueryDef, PaymentDef, PlayActionKind, PlayOptionDef, PlayRestriction,
    PlayerRelation, PrintedManaCost, QuantifierDef, ReplacementAbilityDef, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, SpecialActionDef, SpellAbilityDef, SpellForm,
    StaticAbilityDef, TargetConditionDef, TargetPredicate, TargetSlotDef, TopCardSelectionDef,
    TriggerConditionDef, TriggerEventDef, TriggeredAbilityDef, TurnKindDef, TurnStepDef, ValueDef,
    ZoneKind, ZoneMoveCauseDef, ZonePlacement,
};

/// The built-in catalog, validated once per process. Construction walks every
/// definition and printing, and callers used to pay for it on every game — a
/// training loop opens thousands.
static BUILT_IN: LazyLock<Result<CardCatalog, CatalogError>> = LazyLock::new(|| {
    CardCatalog::with_additional_printings(sets::definitions(), sets::additional_printings())
});

/// Builds the complete card catalog required by the built-in decks.
///
/// The returned catalog shares its definitions with every other caller, so
/// this is cheap enough to call per game.
///
/// # Errors
///
/// Returns [`CatalogError`] if a built-in definition, name, or printing is
/// accidentally duplicated or references an unknown card.
pub fn catalog() -> Result<CardCatalog, CatalogError> {
    BUILT_IN.clone()
}

pub(crate) fn ability_binding(
    origin: crate::AbilityOrigin,
    ability: &AbilityDef,
) -> Option<&'static CardAbilityBinding> {
    sets::ability_binding(origin, ability)
}
