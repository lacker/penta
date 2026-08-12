use crate::action::{ManaColor, Target};
use crate::card::{
    CardTypeSet, ColorSet, ManaCost, PaymentDef, ReplacementEffectDef, ZoneKind, ZonePlacement,
};
use crate::casting::TargetSelection;
use crate::ids::{GameObjectId, PlayerId};

use super::{
    ApplicableReplacement, CardInstance, DecisionObservation, DecisionOption, DecisionZone,
    PendingTrigger, PileChosen, PileSplit, PilesSeparated, ReplacementEffectContext, ScopedEffect,
    StackObject, TriggerContext, TriggerPlacementBatch,
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
    /// One of several players choosing cards for an effect before any chosen
    /// card changes zones.
    DiscardForEffect {
        player: PlayerId,
        amount: usize,
        remaining: Vec<PlayerId>,
        chosen: Vec<(PlayerId, Vec<GameObjectId>)>,
        cause: ZoneMoveCause,
    },
    Tutor,
    LibrarySearch {
        destination: ZoneKind,
        /// A search shuffles whether or not it found anything. Looking at the
        /// top card does not: the rest of the library was never disturbed.
        shuffle: bool,
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
    ManaVault {
        player: PlayerId,
        permanent: GameObjectId,
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
    TimeVault {
        permanent: GameObjectId,
        remaining: Vec<GameObjectId>,
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
    /// The affected object's controller chooses which currently applicable
    /// replacement effect to apply next.
    BattlefieldEntryReplacement {
        candidates: Vec<ApplicableReplacement>,
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
