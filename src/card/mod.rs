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

mod catalog;
mod characteristics;
mod creature_types;
mod model;
mod record;
pub(crate) mod sets;

pub use catalog::{CardCatalog, CatalogError, EffectSubjectKind, GrantedAbilityValidationError};
pub(crate) use characteristics::applicable_part_ids_ref;
pub use characteristics::{CharacteristicContext, CharacteristicError, applicable_part_ids};
pub use creature_types::{CREATURE_TYPES, creature_type_name};
pub(crate) use model::child_effects;
pub use model::{
    AbilityCostDef, AbilityCostList, AbilityCostReductionDef, AbilityDef, AbilityEffectDef,
    AbilityKindDef, AbilityOperationDef, AbilityPredicateDef, AbilityProcedureDef,
    AbilityProgramDef, AbilityTargetDef, AbilityTargetPredicate, ActivatedAbilityDef,
    ActivationTimingDef, AddManaEffectDef, AdditionalCostDef, AdditionalCostValueDef,
    AdditionalTriggerDef, AggregateOperationDef, AlternateSpellKind, AlternativeCastAbilityDef,
    AlternativeCastKindDef, AlternativeCastManaCostDef, AlternativeCostDef, AppliedEffectDef,
    AppliedRuleDef, ArrivalAttachmentDef, AttachedAbilityDef, AttackDeclarationRangeDef,
    AttackDefenderKindDef, AttackDefenderScopeDef, AttackEventMatcherDef, AttackRestrictionDef,
    BandingQuality, BasicLandType, BattlefieldArrivalDef, BattlefieldEntryChoiceDestinationDef,
    BattlefieldEntryModificationDef, BattlefieldEntryScalarChoiceDef, BindObjectsDef,
    BlockRestrictionDef, BlockRestrictionMatchDef, BlockRestrictionSubjectDef, CardAbilityList,
    CardArt, CardChoiceSourceDef, CardComposition, CardDefinition, CardEffectStatus, CardPart,
    CardPrinting, CardPrintingId, CardRules, CardSet, CardStructure, CardSupertype,
    CardSupertypeSet, CardType, CardTypeSet, CastTimingPermissionDef, ChangeStackTargetsDef,
    CharacteristicOperationDef, ChoiceVisibilityDef, ChooseCardsFromCollectionDef, ChooseDef,
    ChooseExactDef, ChooseForEachPlayerDef, ChooseGroupDef, ChooseObjectOrderDef,
    ChooseOneOfEachDef, ClassifyObjectsDef, CollectionInspectionDef, ColorChoiceOperationDef,
    ColorSet, CombineObjectsDef, CompanionConditionDef, ComparisonDef, ConditionDef,
    ConditionValueDef, ConditionalStaticEffectDef, ConditionalValueDef, ControlDurationDef,
    CopyAbilityDef, CopyExceptionsDef, CopyStackObjectDef, CostAdjustmentDef, CostAmountDef,
    CostDef, CostModificationDef, CostQuantityDef, CountConditionDef, CounterFamily, CounterKind,
    CounterKindDef, CounterName, CounterOperationDef, CreatedTokensDef, CreatureStats,
    CreatureTypeSetDef, CumulativeUpkeepCostDef, DamageEventMatcherDef, DamageKindDef,
    DamageLimitDef, DamagePreventionCapacityDef, DamagePreventionDef, DamagePreventionFollowUpDef,
    DamageRecipientMatcherDef, DamageSourceGroupDef, DamageSourceMatcherDef, DeckConstructionDef,
    DeclarativeAbilityDef, DestroyFollowUpDef, DiscardFollowUpDef, DiscardSelectionDef,
    DividedTotal, DoubleFacedKind, DrawEventMatcherDef, EffectChoiceDef, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, EffectRecipientSetDef,
    EmblemCharacteristics, ExilePlayConditionDef, ExilePlayDurationDef, ExiledCastPermissionDef,
    FaceDownCharacteristics, FlexibleManaSymbol, FreePlayDef, FreePlayDurationDef,
    GraveyardPlayPermissionDef, GraveyardTypeConditionDef, HalvedValueDef, HybridPair,
    IfNoObjectsDef, ImplementationStatus, InstalledTriggerDef, InstalledTriggerLifetimeDef,
    IntrinsicCounter, KeywordAbility, KeywordCounter, LAND_SUBTYPES, LifeConditionDef,
    LikelihoodDef, LookAtObjectsDef, ManaColor, ManaCost, ManaCostParseError,
    ManaCostParseErrorKind, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, ManaSplit,
    ManaTypeDef, ManaTypeFilterDef, ManaTypeSetDef, ManaTypeSourceDef, MeldComponentDef,
    MeldRecipeDef, MeldResultDef, MillLoopDef, MillUntilDef, ModalModeListDef, ModalSpellDef,
    ModeDef, ModeSetDef, MoveObjectsDef, MoveToZoneCostDef, ObjectChoiceBindingDef,
    ObjectCollectionSourceDef, ObjectCountConditionDef, ObjectCounterValueDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetCountConditionDef, ObjectSetDef, ObjectSetFilterDef,
    ObjectSetPredicateDef, ObjectSetValueAtLeastDef, ObjectSetValueDef, ObjectValueAggregateDef,
    ObjectValueDef, OngoingEffectDef, OptionalAdditionalCostAbilityDef,
    OptionalAdditionalCostKindDef, PartitionGroupDef, PayOrDef, PerPlayerSelectionDef,
    PileExileDef, PlayActionKind, PlayActionMatcherDef, PlayOptionDef, PlayRestriction,
    PlayRestrictionDef, PlayerAttachmentQueryDef, PlayerObjectCountAggregateDef, PlayerRefDef,
    PlayerRelation, PlayerRuleDef, PlayerSetDef, PowerToughnessCounter, PowerToughnessOperationDef,
    PregameAbilityDef, PregameConditionDef, PregameTimingDef, PrintedManaCost,
    PutObjectsOntoBattlefieldFaceDownDef, QuantifierDef, QuotientValueDef, RandomizeObjectOrderDef,
    ReplacementAbilityDef, ReplacementChoiceDef, ReplacementConditionDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, ResolvedEffectDurationSetDef,
    RevealAndClassifyCardsDef, RevealObjectsDef, RoundingDef, SacrificedAmountDef,
    ScalarChoiceListDef, ScaledValueDef, SetOperationDef, SourceMatchValueDef, SpecialActionDef,
    SpellAbilityDef, SpellAdditionalCostDef, SpellCastQueryDef, SpellCostConditionDef,
    SpellCostModificationDef, SpellForm, SpellResolutionDestinationDef, StackObjectEventDef,
    StackObjectEventMatcherDef, StackTargetAggregationDef, StackTargetChangeDef,
    StackTargetFilterDef, StaticAbilityDef, StaticApplyDef, SumValueDef, SuspendAbilityDef,
    SuspendTimeDef, TapEventMatcherDef, TapPurposeDef, TargetChooserDef, TargetConditionDef,
    TargetPredicate, TargetSlotDef, TokenCharacteristics, TokenCopyDef, TokenCountersDef,
    TokenPart, TokenStatsDef, TokenStructure, TopOfLibraryCostDef, TriggerConditionDef,
    TriggerEventDef, TriggeredAbilityDef, TurnKindDef, TurnPhaseDef, TurnStepDef,
    ValueComparisonDef, ValueDef, ZoneChangeEventMatcherDef, ZoneChangeObservationDef, ZoneKind,
    ZoneMoveCauseDef, ZonePickDef, ZonePickModeDef, ZonePlacement, ZoneRelativePositionDef,
};
pub use model::{DamageAssignmentDef, FightExcessDef};

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
