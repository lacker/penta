mod display;

use std::fmt;

use crate::card::{
    CardEffectStatus, CardPrintingId, ManaCost, PlayActionKind, SpellForm, TargetSlotDef,
};
use crate::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, ChoiceIndex,
    GrantId, ModeId, PlayOptionId, TargetIndex, TargetSlotId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GrantedAbilityValidationError {
    TooManyGrantSites {
        count: usize,
    },
    EmptyText,
    MissingImplementationExplanation,
    LegacyProcedureRequiresCustomExecution,
    HasNoSourceZone,
    ManaAbilityHasTargets,
    TooManyTargets {
        count: usize,
    },
    InvalidTargetBounds {
        target: TargetIndex,
        minimum: u8,
        maximum: u8,
    },
    TargetReferenceOutOfBounds {
        target: TargetIndex,
        target_count: usize,
    },
    ChoiceReferenceOutOfScope {
        choice: ChoiceIndex,
    },
    ChoiceBindingAlreadyInScope {
        choice: ChoiceIndex,
    },
    /// Runtime static-effect discovery currently starts from attached printed
    /// or copied clauses. Reject an executable static ability granted by
    /// another ability until continuous effects have guarded fixed-point
    /// evaluation rather than silently claiming support.
    ExecutableStaticAbility,
}

impl fmt::Display for GrantedAbilityValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyGrantSites { count } => write!(
                formatter,
                "defines {count} grant sites, but grant IDs support at most 256"
            ),
            Self::EmptyText => formatter.write_str("has empty rules text"),
            Self::MissingImplementationExplanation => formatter.write_str(
                "has a non-declarative implementation without an explanation",
            ),
            Self::LegacyProcedureRequiresCustomExecution => formatter.write_str(
                "uses the legacy rules procedure without a custom effect executor",
            ),
            Self::HasNoSourceZone => formatter.write_str("has no source zone"),
            Self::ManaAbilityHasTargets => formatter.write_str("is a mana ability that declares targets"),
            Self::TooManyTargets { count } => write!(
                formatter,
                "defines {count} targets, but positional target indices support at most 256"
            ),
            Self::InvalidTargetBounds {
                target,
                minimum,
                maximum,
            } => write!(
                formatter,
                "defines target {target:?} requiring at least {minimum} targets but allowing at most {maximum}",
            ),
            Self::TargetReferenceOutOfBounds {
                target,
                target_count,
            } => write!(
                formatter,
                "references target {target:?}, but the clause defines only {target_count} target slots"
            ),
            Self::ChoiceReferenceOutOfScope { choice } => {
                write!(formatter, "references choice {choice:?} outside its binding scope")
            }
            Self::ChoiceBindingAlreadyInScope { choice } => write!(
                formatter,
                "binds choice {choice:?}, but that choice is already bound in this scope"
            ),
            Self::ExecutableStaticAbility => formatter.write_str(
                "is an executable static ability, but granted static abilities are not evaluated yet",
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogError {
    DuplicateId(CardDefinitionId),
    DuplicateName(String),
    DuplicatePrintingId(CardPrintingId),
    MismatchedPrintingDefinition {
        definition: CardDefinitionId,
        printing: CardPrintingId,
    },
    OrphanPrinting(CardPrintingId),
    EmptyAbilityText {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    MissingImplementationExplanation {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    LegacyProcedureRequiresCustomExecution {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    DuplicatePartId {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    IncoherentCardRules {
        definition: CardDefinitionId,
        part: CardPartId,
        explanation: &'static str,
    },
    MismatchedPrimaryRules {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    TooManyAbilities {
        definition: CardDefinitionId,
        part: CardPartId,
        count: usize,
    },
    MultipleSpellAbilities {
        definition: CardDefinitionId,
        part: CardPartId,
        count: usize,
    },
    InvalidModalSpellParent {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    TooManySpellModes {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        count: usize,
    },
    InvalidModalSpellSelection {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
        available: usize,
    },
    NonSpellMode {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
    },
    NestedModalSpellMode {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
    },
    CustomSpellModeImplementation {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
    },
    InvalidSpellMode {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
        problem: GrantedAbilityValidationError,
    },
    TooManyAbilityGrantSites {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        count: usize,
    },
    InvalidGrantedAbility {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        grant_path: Vec<GrantId>,
        problem: GrantedAbilityValidationError,
    },
    AbilityHasNoSourceZone {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    ManaAbilityHasTargets {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    TooManyAbilityTargets {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        count: usize,
    },
    InvalidAbilityTargetBounds {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        target: TargetIndex,
        minimum: u8,
        maximum: u8,
    },
    AbilityTargetReferenceOutOfBounds {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        target: TargetIndex,
        target_count: usize,
    },
    AbilityChoiceReferenceOutOfScope {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        choice: ChoiceIndex,
    },
    AbilityChoiceBindingAlreadyInScope {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        choice: ChoiceIndex,
    },
    DuplicateStructurePart {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    InvalidSplitPartCount {
        definition: CardDefinitionId,
        actual: usize,
    },
    UndefinedStructurePart {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    PartOutsideStructure {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    DuplicatePlayOptionId {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    EmptySpellForm {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    DuplicateSpellFormPart {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    UndefinedSpellFormPart {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    SpellFormPartOutsideStructure {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    MissingFusedPlayOption {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    InvalidFusedPlayOption {
        definition: CardDefinitionId,
        option: PlayOptionId,
        expected: Vec<CardPartId>,
        actual: SpellForm,
        actual_action: PlayActionKind,
    },
    UnexpectedCombinedSpellForm {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    CombinedModalSpellUnsupported {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    DuplicateModeId {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
    },
    NonPositionalModeId {
        definition: CardDefinitionId,
        option: PlayOptionId,
        expected: ModeId,
        actual: ModeId,
    },
    EmptyModeSet {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    TooManyModes {
        definition: CardDefinitionId,
        option: PlayOptionId,
        count: usize,
    },
    InvalidModeBounds {
        definition: CardDefinitionId,
        option: PlayOptionId,
        minimum: u8,
        maximum: u8,
    },
    ZeroModeMaximum {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    TooManyModesWithoutRepetition {
        definition: CardDefinitionId,
        option: PlayOptionId,
        maximum: u8,
        available: usize,
    },
    UnexpectedPresentationSpellModes {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    MissingPresentationSpellTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
    },
    MissingSemanticSpellTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
    },
    MismatchedSpellTargetCardinality {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
        presentation_minimum: u8,
        presentation_maximum: u8,
        semantic_minimum: u8,
        semantic_maximum: u8,
    },
    UnpresentableSpellTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
    },
    MismatchedSpellTargetPresentation {
        definition: CardDefinitionId,
        option: PlayOptionId,
        position: usize,
        presentation: TargetSlotDef,
        semantic: TargetSlotDef,
    },
    UnexpectedModalSpellTargets {
        definition: CardDefinitionId,
        option: PlayOptionId,
        count: usize,
    },
    MissingPresentationSpellMode {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
    },
    MissingSemanticSpellMode {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
    },
    MissingPresentationSpellModeTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
    },
    MissingSemanticSpellModeTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
    },
    MismatchedSpellModeTargetCardinality {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
        presentation_minimum: u8,
        presentation_maximum: u8,
        semantic_minimum: u8,
        semantic_maximum: u8,
    },
    UnpresentableSpellModeTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
    },
    MismatchedSpellModeTargetPresentation {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        position: usize,
        presentation: TargetSlotDef,
        semantic: TargetSlotDef,
    },
    MismatchedSpellModeSelection {
        definition: CardDefinitionId,
        option: PlayOptionId,
        presentation_minimum: u8,
        presentation_maximum: u8,
        presentation_may_repeat: bool,
        semantic_minimum: u8,
        semantic_maximum: u8,
        semantic_may_repeat: bool,
    },
    MismatchedSpellModeImplementation {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        presentation: CardEffectStatus,
        semantic: CardEffectStatus,
    },
    MismatchedSpellModeLabel {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        presentation: String,
        semantic: &'static str,
    },
    DuplicateAlternativeCostId {
        definition: CardDefinitionId,
        option: PlayOptionId,
        cost: AlternativeCostId,
    },
    MissingAlternativeCostForAbility {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        cost: AlternativeCostId,
    },
    MismatchedAlternativeCostForAbility {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        option: PlayOptionId,
        cost: AlternativeCostId,
        expected_label: String,
        actual_label: String,
        expected_mana_cost: ManaCost,
        actual_mana_cost: ManaCost,
    },
    DuplicateAdditionalCostId {
        definition: CardDefinitionId,
        cost: AdditionalCostId,
    },
    InvalidTargetBounds {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: Option<ModeId>,
        slot: TargetSlotId,
        minimum: u8,
        maximum: u8,
    },
    TooManyTargetSlots {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: Option<ModeId>,
        count: usize,
    },
    NonPositionalTargetSlot {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: Option<ModeId>,
        expected: TargetSlotId,
        actual: TargetSlotId,
    },
    TooManyInstantiatedTargets {
        definition: CardDefinitionId,
        option: PlayOptionId,
        count: usize,
    },
}
