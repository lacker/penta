use crate::action::{ManaColor, Target};
use crate::card::{
    CardTypeSet, ColorSet, EffectDef, ManaCost, PaymentDef, ReplacementEffectDef, TurnKindDef,
    ZoneKind, ZonePlacement,
};
use crate::casting::TargetSelection;
use crate::ids::{CardDefinitionId, ChoiceIndex, GameObjectId, PlayerId};

use super::{
    AbilitySourceRef, ApplicableReplacement, ApplicableZoneMoveReplacement, CardInstance,
    DecisionObservation, DecisionOption, DecisionZone, DrawReplacement,
    PendingBattlefieldExitBatch, PendingTrigger, PileChosen, PileSplit, PilesSeparated,
    ReplacementEffectContext, ScopedEffect, StackObject, TriggerContext, TriggerPlacementBatch,
};

/// Fork repaints its copy, so the copy is red and nothing else.
pub(super) const FORK_COPY_COLOR: ColorSet = ColorSet::from_colors(&[ManaColor::Red]);

/// What runs once a demanded sacrifice has been chosen and made. The
/// sacrificed permanent's power travels as the trigger amount, so an effect
/// measured by what was sacrificed can read it.
#[derive(Clone, Debug)]
pub(super) struct SacrificeFollowup {
    pub(super) object: Box<StackObject>,
    pub(super) context: TriggerContext,
    pub(super) effect: ScopedEffect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Pregame {
    Mulligan(PlayerId),
    Bottom(PlayerId),
}

#[derive(Clone, Debug)]
pub(super) struct PendingDecision {
    pub(super) observation: DecisionObservation,
    pub(super) continuation: DecisionContinuation,
}

/// One optional replacement that can consume a prospective turn before it
/// begins. The effective ability identity is frozen with its public
/// presentation so copied, granted, and ability-removed sources participate
/// through the same scheduler procedure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ApplicableBeginTurnReplacement {
    pub(super) source: AbilitySourceRef,
    pub(super) controller: PlayerId,
    pub(super) definition: CardDefinitionId,
    pub(super) text: &'static str,
    pub(super) optional: bool,
    pub(super) effect: ReplacementEffectDef,
}

/// An action appended to a skipped prospective turn. CR 614.10b carries it
/// forward until a turn actually begins, when it happens before the turn's
/// ordinary turn-based actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredBeginTurnEffect {
    pub(super) replacement: ApplicableBeginTurnReplacement,
    pub(super) effect: EffectDef,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BalanceAction {
    Sacrifice,
    Discard,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum BalancePhase {
    Lands,
    Hands,
    Creatures,
}

impl BalancePhase {
    pub(super) const fn next(self) -> Option<Self> {
        match self {
            Self::Lands => Some(Self::Hands),
            Self::Hands => Some(Self::Creatures),
            Self::Creatures => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ZoneMoveCause {
    Rules,
    Effect { controller: PlayerId },
}

#[derive(Clone, Debug)]
pub(super) struct BalanceTask {
    pub(super) player: PlayerId,
    pub(super) prompt: String,
    pub(super) zone: DecisionZone,
    pub(super) cards: Vec<CardInstance>,
    pub(super) count: usize,
    pub(super) action: BalanceAction,
    pub(super) cause: ZoneMoveCause,
}

/// Where a countered spell ends up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CounteredSpellZone {
    Graveyard,
    Exile,
}

#[derive(Clone, Debug)]
pub(super) enum DecisionContinuation {
    /// A prospective turn is suspended before any part of it is committed.
    /// When every replacement is optional, option zero begins it. Every other
    /// option applies one replacement; replacing the event asks the scheduler
    /// for the next proposal, while modifying it resumes this same proposal.
    BeginTurn {
        player: PlayerId,
        kind: TurnKindDef,
        applied: Vec<AbilitySourceRef>,
        replacements: Vec<ApplicableBeginTurnReplacement>,
        deferred: Vec<DeferredBeginTurnEffect>,
    },
    /// One of several players choosing cards for an effect before any chosen
    /// card changes zones.
    DiscardForEffect {
        player: PlayerId,
        amount: usize,
        remaining: Vec<PlayerId>,
        chosen: Vec<(PlayerId, Vec<GameObjectId>)>,
        cause: ZoneMoveCause,
    },
    SearchZone {
        controller: PlayerId,
        source: ZoneKind,
        destination: ZoneKind,
        placement: ZonePlacement,
        reveal: bool,
        /// A search shuffles whether or not it found anything. Looking at the
        /// top card does not: the rest of the library was never disturbed.
        shuffle: bool,
    },
    ChooseCards {
        controller: PlayerId,
        destination: ZoneKind,
        placement: ZonePlacement,
        reveal: bool,
    },
    /// The affected player chooses which of several applicable next-draw
    /// replacements consumes this draw. Unchosen replacements remain live.
    DrawReplacement {
        player: PlayerId,
        replacements: Vec<DrawReplacement>,
    },
    BasicLandTypeTextChange {
        target: Target,
    },
    OptionalManaPayment {
        player: PlayerId,
        cost: ManaCost,
        object: Box<StackObject>,
        context: TriggerContext,
        effect: ScopedEffect,
    },
    /// The same offer read the other way round: declining is what makes the
    /// effect happen.
    ManaPaymentOrElse {
        player: PlayerId,
        cost: ManaCost,
        object: Box<StackObject>,
        context: TriggerContext,
        effect: ScopedEffect,
    },
    ChainLightning {
        player: PlayerId,
        spell: StackObject,
        targets: Vec<Target>,
    },
    Fork {
        player: PlayerId,
        spell: StackObject,
        target_lists: Vec<Vec<TargetSelection>>,
    },
    RecallDiscard {
        player: PlayerId,
    },
    RecallReturn {
        player: PlayerId,
    },
    Duress {
        victim: PlayerId,
        cause: ZoneMoveCause,
    },
    /// An effect the controller was offered and may decline.
    OptionalEffect {
        object: Box<StackObject>,
        context: TriggerContext,
        effect: ScopedEffect,
    },
    /// Resume a declarative effect after its controller chooses a permanent
    /// during resolution. The choice is not a target.
    ChoosePermanentForEffect {
        choice: ChoiceIndex,
        object: Box<StackObject>,
        context: TriggerContext,
        effect: ScopedEffect,
    },
    /// The card just drawn, offered to its controller to reveal.
    MiracleReveal {
        card: GameObjectId,
    },
    /// One player separating another's permanents into two piles.
    PileSplit {
        owner: PlayerId,
    },
    /// An opponent separating revealed cards into two piles. The cards have
    /// already left the library, so the continuation must place all of them.
    RevealedPileSplit {
        player: PlayerId,
        revealed: Vec<CardInstance>,
        rest: ZoneKind,
        placement: ZonePlacement,
    },
    /// The revealed piles, offered to whoever gets to keep one.
    RevealedPileChoice {
        player: PlayerId,
        first: Vec<CardInstance>,
        second: Vec<CardInstance>,
        rest: ZoneKind,
        placement: ZonePlacement,
    },
    /// The split piles, offered to whoever must give one up.
    PileChoice {
        first: Vec<GameObjectId>,
        second: Vec<GameObjectId>,
    },
    /// A card-owned resolver has separated object-backed options into two
    /// piles. The shared runtime owns choice mechanics; the card owns what a
    /// chosen pile means.
    SeparateIntoPiles {
        resolving_controller: PlayerId,
        subject: PlayerId,
        items: Vec<DecisionOption>,
        on_complete: PilesSeparated,
    },
    ChoosePile {
        piles: PileSplit,
        on_complete: PileChosen,
    },
    /// A sacrifice an effect demanded, chosen by the sacrificing player.
    SacrificeOfChoice {
        followup: Option<SacrificeFollowup>,
        optional: bool,
    },
    /// A destruction an effect demanded, chosen by the player who controls
    /// the candidates.
    DestroyOfChoice {
        can_regenerate: bool,
    },
    /// The spell's controller deciding whether to keep it alive.
    CounterUnlessPaid {
        spell: GameObjectId,
        player: PlayerId,
        cost: ManaCost,
        zone: CounteredSpellZone,
    },
    /// Holds the revealed cards while the caster decides which to keep; they
    /// have already left the library, so the continuation must place them all.
    GrislySalvage {
        player: PlayerId,
        revealed: Vec<CardInstance>,
    },
    Balance {
        controller: PlayerId,
        phase: BalancePhase,
        task: BalanceTask,
        remaining: Vec<BalanceTask>,
    },
    SylvanOffer {
        player: PlayerId,
    },
    SylvanSelect {
        player: PlayerId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    },
    SylvanMode {
        player: PlayerId,
        card: GameObjectId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    },
    /// How many +1/+1 counters Tetravus is trading for Tetravites. Every
    /// option stands for one counter, so the count selected is the answer.
    TetravusDetach {
        source: GameObjectId,
    },
    /// Which of Tetravus's own Tetravites it is exiling to take the counters
    /// back. The options are the tokens themselves.
    TetravusAssemble {
        source: GameObjectId,
    },
    /// Sin Collector and Lifebane Zombie, holding the hand they exile from.
    ExileFromHand {
        victim: PlayerId,
    },
    /// Augur of Bolas holding the three cards it looked at; they have already
    /// left the library, so the continuation must place all of them.
    AugurOfBolas {
        player: PlayerId,
        revealed: Vec<CardInstance>,
    },
    /// A generic private top-of-library selection. The cards have already
    /// left the library, so both groups and any deferred follow-up live here.
    TopCardSelection {
        player: PlayerId,
        revealed: Vec<CardInstance>,
        selected_zone: ZoneKind,
        selected_placement: ZonePlacement,
        rest_zone: ZoneKind,
        rest_placement: ZonePlacement,
        followup: Option<(Box<StackObject>, TriggerContext, ScopedEffect)>,
    },
    /// The affected object's controller chooses which currently applicable
    /// replacement effect to apply next.
    BattlefieldEntryReplacement {
        candidates: Vec<ApplicableReplacement>,
    },
    /// A simultaneous battlefield-exit batch suspended while the affected
    /// object's controller orders two or more applicable replacement effects.
    BattlefieldExitReplacement {
        batch: PendingBattlefieldExitBatch,
        candidates: Vec<ApplicableZoneMoveReplacement>,
    },
    /// A replacement effect suspended while its controller chooses whether to
    /// pay. The prospective event itself remains at the front of the queue.
    BattlefieldEntryPayment {
        context: ReplacementEffectContext,
        payment: PaymentDef,
        if_paid: &'static [ReplacementEffectDef],
        if_declined: &'static [ReplacementEffectDef],
    },
    BattlefieldEntryCardName {
        choices: Vec<String>,
    },
    /// The permanents an entering copy effect could imitate, plus the option
    /// of entering as itself.
    BattlefieldEntryCopy {
        choices: Vec<GameObjectId>,
        added_types: CardTypeSet,
    },
    BattlefieldEntryCreatureType {
        choices: Vec<String>,
    },
    TriggerOrder {
        batch: TriggerPlacementBatch,
        remaining: Vec<TriggerPlacementBatch>,
    },
    TriggerPlacement {
        trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
        candidates: Vec<Target>,
    },
}
