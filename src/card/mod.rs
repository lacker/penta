//! Card definitions, rules metadata, and the built-in multi-format card corpus.
//!
//! The corpus is grouped by release year and set in [`sets`]. Canonical card
//! records own rules and implementation status, while reprint and alternate-art
//! records provide distinct physical-printing identities without duplicating
//! gameplay definitions.

use std::sync::LazyLock;

pub mod abilities;
pub mod cards;
pub mod face_down;
pub mod tokens;

mod behavior;
mod catalog;
mod characteristics;
mod creature_types;
mod model;
mod record;
pub(crate) mod sets;

pub(crate) use model::child_effects;
pub(crate) use record::{AbilityPolicyHint, CardAbilityBinding};

pub use behavior::CardBehavior;
pub use catalog::{CardCatalog, CatalogError, EffectSubjectKind, GrantedAbilityValidationError};
pub use characteristics::{CharacteristicContext, CharacteristicError, applicable_part_ids};
pub use creature_types::CREATURE_TYPES;
pub use model::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityEffectDef,
    AbilityOperationDef, AbilityPredicateDef, AbilityProcedureDef, AbilityProgramDef,
    AbilityTargetDef, AbilityTargetPredicate, ActivatedAbilityDef, ActivationTimingDef,
    AddManaEffectDef, AdditionalCostDef, AlternateSpellKind, AlternativeCastAbilityDef,
    AlternativeCastKindDef, AlternativeCastManaCostDef, AlternativeCostDef, AppliedEffectDef,
    AppliedRuleDef, ArrivalAttachmentDef, AttachedAbilityDef, AttackDeclarationRangeDef,
    AttackDefenderScopeDef, AttackEventMatcherDef, AttackRestrictionDef, BandingQuality,
    BasicLandType, BattlefieldEntryChoiceDestinationDef, BattlefieldEntryModificationDef,
    BattlefieldEntryScalarChoiceDef, CardAbilityList, CardArt, CardChoiceSourceDef,
    CardComposition, CardDefinition, CardEffectStatus, CardPart, CardPrinting, CardPrintingId,
    CardRules, CardSet, CardStructure, CardSupertype, CardType, CardTypeSet,
    CharacteristicOperationDef, ChoiceVisibilityDef, ChooseDef, ColorChoiceOperationDef, ColorSet,
    ComparisonDef, ConditionDef, ConditionalValueDef, ControlDurationDef, CostDef,
    CountConditionDef, CounterKind, CreatedTokensDef, CreatureStats, CreatureTypeSetDef,
    DamageCoverageDef, DamageEventMatcherDef, DamageKindDef, DamageLimitDef,
    DamagePreventionCapacityDef, DamagePreventionDef, DamagePreventionFollowUpDef,
    DamageRecipientMatcherDef, DamageSourceGroupDef, DamageSourceMatcherDef, DeclarativeAbilityDef,
    DiscardFollowUpDef, DiscardSelectionDef, DividedTotal, DoubleFacedKind, DrawEventMatcherDef,
    EffectDef, EffectExecutionDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef,
    EffectRecipientSetDef, EmblemCharacteristics, FaceDownCharacteristics, FlexibleManaSymbol,
    GraveyardTypeConditionDef, HalvedValueDef, HybridPair, ImplementationStatus,
    InstalledTriggerDef, InstalledTriggerLifetimeDef, KeywordAbility, LifeConditionDef,
    LikelihoodDef, ManaColor, ManaCost, ManaCostParseError, ManaCostParseErrorKind,
    ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, ManaSplit, MeldComponentDef,
    MeldRecipeDef, MeldResultDef, ModalSpellDef, ModeDef, ModeSetDef, ObjectChoiceBindingDef,
    ObjectCountConditionDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    OptionalAdditionalCostAbilityDef, OptionalAdditionalCostKindDef, PartitionItemsDef, PayOrDef,
    PlayActionKind, PlayActionMatcherDef, PlayOptionDef, PlayRestriction, PlayRestrictionDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, PrintedManaCost,
    ProtectedCreatureType, QuantifierDef, ReplacementAbilityDef, ReplacementChoiceDef,
    ReplacementConditionDef, ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef,
    RoundingDef, SacrificedAmountDef, ScalarChoiceListDef, ScaledValueDef, SetOperationDef,
    SpecialActionDef, SpellAbilityDef, SpellAdditionalCostCountDef, SpellAdditionalCostDef,
    SpellForm, SpellLifeCostDef, SpellResolutionDestinationDef, SpendModeDef, SplitIntoPilesDef,
    StackTargetKindDef, StaticAbilityDef, SumValueDef, TapEventMatcherDef, TapPurposeDef,
    TargetConditionDef, TargetPredicate, TargetSlotDef, TokenCharacteristics, TokenCountersDef,
    TokenPart, TokenStructure, TopCardSelectionDef, TopOfLibraryCostDef, TriggerConditionDef,
    TriggerEventDef, TriggeredAbilityDef, TurnKindDef, TurnPhaseDef, TurnStepDef,
    ValueComparisonDef, ValueDef, ZoneChangeEventMatcherDef, ZoneKind, ZoneMoveCauseDef,
    ZonePlacement,
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
