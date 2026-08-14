use serde::{Deserialize, Serialize};

use super::model::{
    AbilityLocator, AbilitySourceSnapshot, ApplicableReplacementSnapshot, DetachedCardSnapshot,
    DetachedStackSnapshot, ManaCostSnapshot, PendingTriggerSnapshot,
    ReplacementEffectContextSnapshot, ReplacementEffectLocator, ScopedEffectSnapshot,
    TargetSelectionSnapshot, TargetSnapshot, TriggerContextSnapshot, TriggerPlacementBatchSnapshot,
    ZoneKindSnapshot, ZoneMoveCauseSnapshot,
};
use super::model_procedure::DrawReplacementSnapshot;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionStateSnapshot {
    pub(super) preference: DecisionPreferenceSnapshot,
    pub(super) continuation: DecisionContinuationSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum DecisionPreferenceSnapshot {
    Name(String),
    PreferOption {
        #[serde(rename = "preferOption")]
        prefer_option: u32,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum TurnKindSnapshot {
    Any,
    Regular,
    Extra,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ApplicableBeginTurnReplacementSnapshot {
    pub(super) source: AbilitySourceSnapshot,
    pub(super) controller: usize,
    pub(super) definition: u16,
    pub(super) effect: ReplacementEffectLocator,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeferredBeginTurnEffectSnapshot {
    pub(super) replacement: ApplicableBeginTurnReplacementSnapshot,
    pub(super) effect: ScopedEffectSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum DecisionContinuationSnapshot {
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
    },
    ChooseCards {
        controller: usize,
        destination: ZoneKindSnapshot,
        placement: ZonePlacementSnapshot,
        reveal: bool,
    },
    DrawReplacement {
        player: usize,
        replacements: Vec<DrawReplacementSnapshot>,
    },
    BasicLandTypeTextChange {
        target: TargetSnapshot,
    },
    ExileFromHand {
        victim: usize,
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
    AugurOfBolas {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
    },
    TopCardSelection {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
        selected_zone: ZoneKindSnapshot,
        selected_placement: ZonePlacementSnapshot,
        rest_zone: ZoneKindSnapshot,
        rest_placement: ZonePlacementSnapshot,
        followup: Option<EffectContinuationSnapshot>,
    },
    OptionalManaPayment {
        player: usize,
        cost: ManaCostSnapshot,
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: TriggerContextSnapshot,
        effect: ScopedEffectSnapshot,
    },
    ManaPaymentOrElse {
        player: usize,
        cost: ManaCostSnapshot,
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: TriggerContextSnapshot,
        effect: ScopedEffectSnapshot,
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
    },
    OptionalEffect {
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: TriggerContextSnapshot,
        effect: ScopedEffectSnapshot,
    },
    ChoosePermanentForEffect {
        choice: u8,
        continuation: EffectContinuationSnapshot,
    },
    BattlefieldEntryPayment {
        context: ReplacementEffectContextSnapshot,
        effect: ReplacementEffectLocator,
    },
    BattlefieldEntryReplacement {
        candidates: Vec<ApplicableReplacementSnapshot>,
    },
    BattlefieldEntryCardName {
        choices: Vec<String>,
    },
    BattlefieldEntryOptional {
        context: ReplacementEffectContextSnapshot,
    },
    BattlefieldEntryCreatureType {
        choices: Vec<String>,
    },
    BattlefieldEntryCopy {
        choices: Vec<u32>,
        added_types: [bool; crate::card::CardType::COUNT],
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
    MiracleReveal {
        card: u32,
    },
    PileSplit {
        owner: usize,
    },
    RevealedPileSplit {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
        rest: ZoneKindSnapshot,
        placement: ZonePlacementSnapshot,
    },
    RevealedPileChoice {
        player: usize,
        first: Vec<DetachedCardSnapshot>,
        second: Vec<DetachedCardSnapshot>,
        rest: ZoneKindSnapshot,
        placement: ZonePlacementSnapshot,
    },
    PileChoice {
        first: Vec<u32>,
        second: Vec<u32>,
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
        followup: Option<EffectContinuationSnapshot>,
        optional: bool,
    },
    DestroyOfChoice {
        can_regenerate: bool,
    },
    CounterUnlessPaid {
        spell: u32,
        player: usize,
        cost: ManaCostSnapshot,
        zone: CounteredSpellZoneSnapshot,
    },
    RecallDiscard {
        player: usize,
    },
    RecallReturn {
        player: usize,
    },
    Duress {
        victim: usize,
        cause: ZoneMoveCauseSnapshot,
    },
    Balance {
        controller: usize,
        phase: BalancePhaseSnapshot,
        task: BalanceTaskSnapshot,
        remaining: Vec<BalanceTaskSnapshot>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PileSplitSnapshot {
    pub(super) resolving_controller: usize,
    pub(super) subject: usize,
    pub(super) first: Vec<DecisionOptionSnapshot>,
    pub(super) second: Vec<DecisionOptionSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionOptionSnapshot {
    pub(super) id: u32,
    pub(super) label: String,
    pub(super) card: Option<DecisionCardSnapshot>,
    pub(super) members: Vec<DecisionCardSnapshot>,
    pub(super) ability_text: Option<String>,
    pub(super) zone: DecisionZoneSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionCardSnapshot {
    pub(super) object_id: u32,
    pub(super) definition: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum BalancePhaseSnapshot {
    Lands,
    Hands,
    Creatures,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BalanceTaskSnapshot {
    pub(super) player: usize,
    pub(super) prompt: String,
    pub(super) zone: DecisionZoneSnapshot,
    pub(super) cards: Option<Vec<DetachedCardSnapshot>>,
    pub(super) count: usize,
    pub(super) action: BalanceActionSnapshot,
    pub(super) cause: ZoneMoveCauseSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum BalanceActionSnapshot {
    Sacrifice,
    Discard,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum DecisionZoneSnapshot {
    Hand,
    Graveyard,
    Battlefield,
    Stack,
    Library,
    Exile,
    OutsideGame,
    Command,
    DrawnThisStep,
    None,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum ZonePlacementSnapshot {
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum CounteredSpellZoneSnapshot {
    Graveyard,
    Exile,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct EffectContinuationSnapshot {
    pub(super) object: DetachedStackSnapshot,
    pub(super) ability: AbilityLocator,
    pub(super) context: TriggerContextSnapshot,
    pub(super) effect: ScopedEffectSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DiscardChoiceSnapshot {
    pub(super) player: usize,
    pub(super) cards: Option<Vec<u32>>,
    pub(super) count: usize,
}
