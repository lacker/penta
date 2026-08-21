//! Deterministic engine primitives for supported two-player Magic formats.

pub mod action;
pub mod card;
pub mod casting;
pub mod deck;
pub mod decks;
pub mod format;
pub mod game;
pub mod ids;
pub mod poc;
pub mod policy;
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
    AttachedAbilityDef, AttackDeclarationRangeDef, AttackEventMatcherDef, BasicLandType,
    BattlefieldEntryChoiceDestinationDef, BattlefieldEntryModificationDef,
    BattlefieldEntryScalarChoiceDef, CREATURE_TYPES, CardAbilityList, CardArt, CardBehavior,
    CardCatalog, CardChoiceSourceDef, CardComposition, CardDefinition, CardEffectStatus, CardPart,
    CardPrinting, CardPrintingId, CardRules, CardSet, CardStructure, CardSupertype, CardType,
    CardTypeSet, CatalogError, CharacteristicContext, CharacteristicError,
    CharacteristicOperationDef, ColorSet, ComparisonDef, ConditionDef, ConditionalValueDef,
    ControlDurationDef, CostDef, CountConditionDef, CounterKind, CreatureStats, CreatureTypeSetDef,
    DamageCoverageDef, DamageEventMatcherDef, DamageKindDef, DamagePreventionCapacityDef,
    DamagePreventionDef, DamagePreventionFollowUpDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DeclarativeAbilityDef, DiscardSelectionDef, DividedTotal,
    DoubleFacedKind, EffectDef, EffectExecutionDef, EffectPaymentCostDef, EffectPaymentDef,
    EffectRecipientDef, EffectRecipientSetDef, EffectSubjectKind, EmblemCharacteristics,
    GrantedAbilityValidationError, HybridPair, ImplementationStatus, InstalledTriggerDef,
    InstalledTriggerLifetimeDef, KeywordAbility, LikelihoodDef, ManaCost, ManaCostParseError,
    ManaCostParseErrorKind, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, ManaSplit,
    MeldComponentDef, MeldRecipeDef, MeldResultDef, ModalSpellDef, ModeDef, ModeSetDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayActionKind,
    PlayActionMatcherDef, PlayOptionDef, PlayRestriction, PlayRestrictionDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, PrintedManaCost,
    ReplacementAbilityDef, ReplacementChoiceDef, ReplacementConditionDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, ScalarChoiceListDef, SetOperationDef,
    SpecialActionDef, SpellAbilityDef, SpellForm, SpellResolutionDestinationDef, StaticAbilityDef,
    TapEventMatcherDef, TapPurposeDef, TargetConditionDef, TargetPredicate, TargetSlotDef,
    TokenCharacteristics, TokenPart, TokenStructure, TriggerConditionDef, TriggerEventDef,
    TriggeredAbilityDef, TurnKindDef, TurnPhaseDef, TurnStepDef, ValueDef,
    ZoneChangeEventMatcherDef, ZoneKind, ZoneMoveCauseDef, ZonePlacement, applicable_part_ids,
    tokens,
};
pub use casting::{
    CastChoices, CastSignature, CostConfiguration, TargetReplacementError, TargetSelection,
};
pub use deck::{Deck, DeckError, ValidatedDeck};
pub use format::{Format, FormatRules};
pub use game::{
    BattlefieldExit, DecisionObservation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, EmblemObservation, Game, GameError, GameEvent, GameResult, Mana, ManaPool,
    ManaSource, ObjectCharacteristics, PermanentObservation, PhysicalFaceObservation,
    PhysicalFaceSide, PlayerObservation, StackObjectKind, Step, WinReason, ZoneCard,
    ZoneChangeOutcome, ZoneError,
};
pub use ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardInstanceId, CardPartId,
    GameObjectId, GrantId, MeldRecipeId, ModeId, ObjectBindingIndex, ObjectSetBindingIndex,
    PhysicalCardId, PlayOptionId, PlayerId, StackObjectId, TargetIndex, TargetSlotId,
};
pub use policy::{HandcraftedPolicy, PlayError, Policy, RandomPolicy, play_game};
