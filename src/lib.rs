//! Deterministic engine primitives for supported two-player Magic formats.

/// Declares a durable effect binding with a human-readable label.
#[macro_export]
macro_rules! Binding {
    ($label:literal) => {
        $crate::Binding::from_label($label)
    };
}

pub mod action;
pub mod card;
pub mod casting;
pub mod deck;
pub mod decks;
pub mod formats;
/// Backwards-compatible module path for callers compiled against `penta::format`.
#[deprecated(note = "use penta::formats")]
pub mod format {
    pub use crate::formats::*;

    /// Backwards-compatible path for the former Vintage Cube pool module.
    #[deprecated(note = "use penta::formats::cubes::vintage")]
    pub mod vintage_cube {
        pub use crate::formats::cubes::vintage::*;
    }
}
pub mod game;
pub mod ids;
pub mod poc;
pub mod policy;
mod prepared_engine;
pub mod protocol;
mod rng;
pub mod rules;

pub use action::{
    AbilityOrigin, Action, ActionError, AttackDefender, CombatDamageAssignment, ManaColor, Target,
};
pub use card::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityEffectDef,
    AbilityOperationDef, AbilityPredicateDef, AbilityProcedureDef, AbilityProgramDef,
    AbilityTargetDef, AbilityTargetPredicate, ActivatedAbilityDef, AddManaEffectDef,
    AdditionalCostDef, AlternateSpellKind, AlternativeCastAbilityDef, AlternativeCastKindDef,
    AlternativeCastManaCostDef, AlternativeCostDef, AppliedEffectDef, AppliedRuleDef,
    AttachedAbilityDef, AttackDeclarationRangeDef, AttackDefenderKindDef, AttackEventMatcherDef,
    BasicLandType, BattlefieldEntryChoiceDestinationDef, BattlefieldEntryModificationDef,
    BattlefieldEntryScalarChoiceDef, CREATURE_TYPES, CardAbilityList, CardArt, CardCatalog,
    CardChoiceSourceDef, CardComposition, CardDefinition, CardEffectStatus, CardPart, CardPrinting,
    CardPrintingId, CardRules, CardSet, CardStructure, CardSupertype, CardType, CardTypeSet,
    CastTimingPermissionDef, CatalogError, CharacteristicContext, CharacteristicError,
    CharacteristicOperationDef, ColorSet, CompanionConditionDef, ComparisonDef, ConditionDef,
    ConditionalValueDef, ControlDurationDef, CostDef, CountConditionDef, CounterFamily,
    CounterKind, CounterName, CreatureStats, CreatureTypeSetDef, DamageCoverageDef,
    DamageEventMatcherDef, DamageKindDef, DamagePreventionCapacityDef, DamagePreventionDef,
    DamagePreventionFollowUpDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeckConstructionDef, DeclarativeAbilityDef, DiscardSelectionDef, DividedTotal, DoubleFacedKind,
    EffectDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, EffectRecipientSetDef,
    EffectSubjectKind, EmblemCharacteristics, FaceDownCharacteristics, FlexibleManaSymbol,
    FreePlayDef, FreePlayDurationDef, GrantedAbilityValidationError, HybridPair,
    ImplementationStatus, InstalledTriggerDef, InstalledTriggerLifetimeDef, IntrinsicCounter,
    KeywordAbility, KeywordCounter, LikelihoodDef, ManaCost, ManaCostParseError,
    ManaCostParseErrorKind, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, ManaSplit,
    ManaTypeDef, ManaTypeFilterDef, ManaTypeSetDef, ManaTypeSourceDef, MeldComponentDef,
    MeldRecipeDef, MeldResultDef, ModalModeListDef, ModalSpellDef, ModeDef, ModeSetDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    OptionalAdditionalCostAbilityDef, OptionalAdditionalCostKindDef, PlayActionKind,
    PlayActionMatcherDef, PlayOptionDef, PlayRestriction, PlayRestrictionDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, PowerToughnessCounter, PowerToughnessOperationDef,
    PrintedManaCost, ReplacementAbilityDef, ReplacementChoiceDef, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef,
    ResolvedEffectDurationSetDef, ScalarChoiceListDef, SetOperationDef, SourceMatchValueDef,
    SpecialActionDef, SpellAbilityDef, SpellForm, SpellResolutionDestinationDef, StaticAbilityDef,
    TapEventMatcherDef, TapPurposeDef, TargetChooserDef, TargetConditionDef, TargetPredicate,
    TargetSlotDef, TokenCharacteristics, TokenPart, TokenStructure, TriggerConditionDef,
    TriggerEventDef, TriggeredAbilityDef, TurnKindDef, TurnPhaseDef, TurnStepDef, ValueDef,
    ZoneChangeEventMatcherDef, ZoneChangeObservationDef, ZoneKind, ZoneMoveCauseDef, ZonePlacement,
    applicable_part_ids, face_down, tokens,
};
pub use casting::{
    CastChoices, CastSignature, CostConfiguration, FlexibleManaPayment, ManaPaymentChoice,
    TargetReplacementError, TargetSelection,
};
pub use deck::{Deck, DeckError, ValidatedDeck};
pub use formats::{
    CubeFormatDefinition, Format, FormatCategory, FormatDefinition, FormatRules,
    SetFormatDefinition,
};
pub use game::{
    BattlefieldExit, CardCounterObservation, CounterObservation, DecisionObservation,
    DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone, EmblemObservation, Game,
    GameError, GameEvent, GameResult, Mana, ManaPool, ManaSource, ObjectCharacteristics,
    PermanentObservation, PhysicalFaceObservation, PhysicalFaceSide, PlayerObservation,
    StackObjectKind, Step, WinReason, ZoneCard, ZoneChangeOutcome, ZoneError,
};
pub use ids::{
    AbilityId, AdditionalCostId, AdditionalCostIndex, AdditionalCostObjectIndex, AlternativeCostId,
    Binding, CardDefinitionId, CardInstanceId, CardPartId, GameObjectId, GrantId, MeldRecipeId,
    ModeId, ParentBinding, PhysicalCardId, PlayOptionId, PlayerId, StackObjectId, TargetIndex,
    TargetSlotId,
};
pub use policy::{HandcraftedPolicy, PlayError, Policy, RandomPolicy, play_game};
