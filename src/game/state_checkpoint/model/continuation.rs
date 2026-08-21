//! The pending question a checkpoint was taken in the middle of.
//!
//! A decision is the one place a resolving effect stops with work still to
//! do, so every continuation has to say enough for the rest of that work to
//! be found again in the catalog rather than carried as executable state.

use serde::{Deserialize, Serialize};

use super::{
    AbilityLocator, AbilityOriginSnapshot, AbilitySourceSnapshot,
    ApplicableBeginTurnReplacementSnapshot, ApplicableReplacementSnapshot, BalancePhaseSnapshot,
    BalanceTaskSnapshot, DecisionOptionSnapshot, DeferredBeginTurnEffectSnapshot,
    DetachedCardSnapshot, DetachedStackSnapshot, DiscardChoiceSnapshot, DrawReplacementSnapshot,
    EffectContinuationSnapshot, EffectResolutionContextSnapshot, ManaSnapshot,
    PendingTriggerSnapshot, PileSplitSnapshot, ReplacementEffectContextSnapshot,
    ReplacementEffectLocator, ResolvedEffectPaymentSnapshot, ScopedEffectSnapshot,
    TargetSelectionSnapshot, TargetSnapshot, TriggerPlacementBatchSnapshot, TurnKindSnapshot,
    ZoneKindSnapshot, ZoneMoveCauseSnapshot, ZonePlacementSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum DecisionContinuationSnapshot {
    BeginTurn {
        player: usize,
        turn_kind: TurnKindSnapshot,
        applied: Vec<AbilitySourceSnapshot>,
        replacements: Vec<ApplicableBeginTurnReplacementSnapshot>,
        deferred: Vec<DeferredBeginTurnEffectSnapshot>,
    },
    SearchZone {
        controller: usize,
        source: ZoneKindSnapshot,
        destination: ZoneKindSnapshot,
        placement: ZonePlacementSnapshot,
        reveal: bool,
        shuffle: bool,
        /// Additive: a checkpoint written before fetch lands existed carries
        /// no flag and reconstructs as an untapped arrival.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        enters_tapped: bool,
        /// Where the results are saved for the follow-up. Absent from a
        /// payload written before any search had one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        binding: Option<usize>,
        /// What the search runs once it is answered, relocated in the
        /// catalog rather than carried as executable state.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        follow_up: Option<EffectContinuationSnapshot>,
    },
    ChooseCards {
        controller: usize,
        destination: ZoneKindSnapshot,
        placement: ZonePlacementSnapshot,
        reveal: bool,
        /// The resolution a battlefield arrival belongs to, relocated in the
        /// catalog rather than carried as executable state. Absent from a
        /// payload written before any choice put a permanent onto the
        /// battlefield carrying something.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arrival: Option<EffectContinuationSnapshot>,
    },
    DrawReplacement {
        player: usize,
        replacements: Vec<DrawReplacementSnapshot>,
    },
    BasicLandTypeTextChange {
        target: TargetSnapshot,
    },
    DiscardForEffect {
        player: usize,
        amount: usize,
        remaining: Vec<usize>,
        chosen: Vec<DiscardChoiceSnapshot>,
        cause: ZoneMoveCauseSnapshot,
    },
    GrislySalvage {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
    },
    SacrificeToTotalPower {
        player: usize,
        /// How much power is still owed. Zero or less means the payer may
        /// stop, and the offer includes a way to.
        remaining: i32,
        /// Boxed for the same reason the live continuation boxes it: a stack
        /// object beside a handful of numbers would otherwise set the size
        /// of every variant here.
        object: Box<DetachedStackSnapshot>,
        context: EffectResolutionContextSnapshot,
        /// The half a completed payment runs. Most printed forms have none:
        /// "sacrifice it unless you sacrifice ..." buys only the absence of
        /// the other branch, which is settled before this is asked.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        if_paid: Option<Box<EffectContinuationSnapshot>>,
    },
    CardNameChoice {
        /// The names on offer. A card name is stable catalog data, so the
        /// list is written down rather than recomputed: which names were
        /// offered is part of the pending question.
        choices: Vec<String>,
        searched: usize,
        zone: ZoneKindSnapshot,
        binding: usize,
        continuation: EffectContinuationSnapshot,
    },
    AugurOfBolas {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
    },
    TopCardSelection {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
        continuation: EffectContinuationSnapshot,
    },
    ChainLightning {
        player: usize,
        spell: DetachedStackSnapshot,
        targets: Vec<TargetSnapshot>,
    },
    Fork {
        player: usize,
        spell: DetachedStackSnapshot,
        target_lists: Vec<Vec<TargetSelectionSnapshot>>,
        /// Absent for a card copying itself, which keeps its own colours.
        #[serde(default)]
        repainted: bool,
        /// Copies still to offer after this one, for storm.
        #[serde(default)]
        remaining: u16,
    },
    OptionalEffect {
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: EffectResolutionContextSnapshot,
        effect: ScopedEffectSnapshot,
    },
    MayCastExiled {
        player: usize,
        card: u32,
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: EffectResolutionContextSnapshot,
        definition: ScopedEffectSnapshot,
    },
    ChooseForEffect {
        continuation: EffectContinuationSnapshot,
    },
    PayOr {
        player: usize,
        payment: ResolvedEffectPaymentSnapshot,
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: EffectResolutionContextSnapshot,
        definition: ScopedEffectSnapshot,
    },
    SplitForEffect {
        continuation: EffectContinuationSnapshot,
    },
    ChoosePileForEffect {
        first: Vec<TargetSnapshot>,
        second: Vec<TargetSnapshot>,
        continuation: EffectContinuationSnapshot,
    },
    BattlefieldEntryPayment {
        context: ReplacementEffectContextSnapshot,
        player: usize,
        payment: ResolvedEffectPaymentSnapshot,
        effect: ReplacementEffectLocator,
    },
    BattlefieldEntryReplacement {
        candidates: Vec<ApplicableReplacementSnapshot>,
    },
    BattlefieldEntryOptional {
        context: ReplacementEffectContextSnapshot,
        effect: ReplacementEffectLocator,
    },
    BattlefieldEntryScalarChoice {
        context: ReplacementEffectContextSnapshot,
        effect: ReplacementEffectLocator,
        choices: Vec<String>,
    },
    BattlefieldEntryCopy {
        choices: Vec<u32>,
        added_types: [bool; crate::card::CardType::COUNT],
        /// Additive: a checkpoint written before a copy could keep anything
        /// of its own restores with a copy that keeps nothing, which is what
        /// every copy did then.
        #[serde(default)]
        retain_printed_subtypes: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        added_abilities: Vec<super::CopiableAbilitySnapshot>,
    },
    TriggerOrder {
        batch: TriggerPlacementBatchSnapshot,
        remaining: Vec<TriggerPlacementBatchSnapshot>,
    },
    TriggerPlacement {
        trigger: PendingTriggerSnapshot,
        pending: Vec<PendingTriggerSnapshot>,
        remaining: Vec<TriggerPlacementBatchSnapshot>,
        candidates: Vec<TargetSnapshot>,
    },
    TriggerDivision {
        trigger: PendingTriggerSnapshot,
        pending: Vec<PendingTriggerSnapshot>,
        remaining: Vec<TriggerPlacementBatchSnapshot>,
        targets: Vec<TargetSnapshot>,
        divisions: Vec<Vec<u16>>,
    },
    DrawActionWindow {
        card: u32,
    },
    MayCastAlternative {
        player: usize,
        card: u32,
        ability: AbilityOriginSnapshot,
    },
    SpellLibraryEnd {
        owner: usize,
        spell: u32,
    },
    CascadeCast {
        player: usize,
        card: u32,
        exiled: Vec<u32>,
    },
    MayCastGranted {
        player: usize,
        card: u32,
        ability: AbilityLocator,
        grant: usize,
    },
    Proliferate {
        candidates: Vec<TargetSnapshot>,
    },
    ExploredCardPlacement {
        player: usize,
        revealed: u32,
    },
    SeparateIntoPiles {
        resolving_controller: usize,
        subject: usize,
        items: Vec<DecisionOptionSnapshot>,
        on_complete: String,
    },
    ChoosePile {
        piles: PileSplitSnapshot,
        on_complete: String,
    },
    SacrificeOfChoice {
        followup: Option<Box<EffectContinuationSnapshot>>,
        /// The branch a declined offer takes. Appended after the follow-up,
        /// so a checkpoint written before this existed still reads. Boxed
        /// alongside it to keep this variant off the enum's size.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        declined: Option<Box<EffectContinuationSnapshot>>,
        optional: bool,
    },
    /// A colour choice waiting to be answered. Only the recipients are
    /// stored: what to do with the answer and how long it lasts are read
    /// back off the effect the continuation already locates.
    ChooseColor {
        continuation: Box<EffectContinuationSnapshot>,
        targets: Vec<TargetSnapshot>,
    },
    KeepOnePerType {
        player: usize,
        controller: usize,
        /// Indices into `CardType::ALL`, in the order the printed clause
        /// names them. The same positional encoding a copied entry's added
        /// types already uses.
        remaining: Vec<usize>,
        kept: Vec<u32>,
    },
    ChosenColorMana {
        controller: usize,
        /// The mana each answer produces, with its colour standing in for
        /// the one still to be chosen.
        prototype: ManaSnapshot,
        remaining: u16,
        /// Which colours may be chosen, in the usual WUBRG flag order.
        choosable: [bool; 5],
    },
    RecallDiscard {
        player: usize,
    },
    RecallReturn {
        player: usize,
    },
    Balance {
        controller: usize,
        phase: BalancePhaseSnapshot,
        task: BalanceTaskSnapshot,
        remaining: Vec<BalanceTaskSnapshot>,
    },
    SearchZonesAndExileRest {
        player: usize,
        zones: Vec<ZoneKindSnapshot>,
        searched: Vec<u32>,
    },
    Vote {
        candidates: Vec<u32>,
        remaining: Vec<usize>,
        votes: Vec<u32>,
    },
    SylvanOffer {
        player: usize,
    },
    SylvanSelect {
        player: usize,
        candidates: Vec<u32>,
        choices_left: usize,
    },
    SylvanMode {
        player: usize,
        card: u32,
        candidates: Vec<u32>,
        choices_left: usize,
    },
    TetravusDetach {
        source: u32,
    },
    TetravusAssemble {
        source: u32,
    },
}
