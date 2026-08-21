use crate::action::{ManaColor, Target};
use crate::card::{
    AbilityDef, BattlefieldEntryScalarChoiceDef, CardType, CardTypeSet, ColorChoiceOperationDef,
    ColorSet, EffectDef, ManaCost, ObjectChoiceBindingDef, ObjectPredicateDef,
    ReplacementEffectDef, TopCardSelectionDef, TurnKindDef, ZoneKind, ZonePlacement,
};
use crate::casting::TargetSelection;
use crate::ids::{CardDefinitionId, GameObjectId, ObjectSetBindingIndex, PlayerId};

use super::{
    AbilitySourceRef, ApplicableReplacement, ApplicableZoneMoveReplacement, CardInstance,
    DecisionObservation, DecisionOption, DecisionZone, DrawReplacement, EffectResolutionContext,
    Mana, ObjectCharacteristics, PendingActivation, PendingBattlefieldExitBatch, PendingTrigger,
    PileChosen, PileSplit, PilesSeparated, ReplacementEffectContext, ResolvedEffectDurationDef,
    SacrificeQuota, SacrificedAmountDef, ScopedEffect, StackObject, TriggerPlacementBatch,
};

/// Fork repaints its copy, so the copy is red and nothing else.
pub(super) const FORK_COPY_COLOR: ColorSet = ColorSet::from_colors(&[ManaColor::Red]);

/// What runs once a demanded sacrifice has been chosen and made. The
/// sacrificed permanent's power travels as the trigger amount, so an effect
/// measured by what was sacrificed can read it.
#[derive(Clone, Debug)]
pub(super) struct SacrificeFollowup {
    pub(super) object: Box<StackObject>,
    pub(super) context: EffectResolutionContext,
    pub(super) effect: ScopedEffect,
    /// Which characteristic of the sacrificed permanent this reads. Carried
    /// here because the permanent is gone by the time the follow-up runs.
    pub(super) amount: SacrificedAmountDef,
}

/// The branch an optional sacrifice takes when it is declined or has nothing
/// to take. Carried beside the follow-up because both are frozen when the
/// offer is made, and exactly one of them runs.
#[derive(Clone, Debug)]
pub(super) struct SacrificeDeclined {
    pub(super) object: Box<StackObject>,
    pub(super) context: EffectResolutionContext,
    pub(super) effect: ScopedEffect,
}

/// A payment whose dynamic values and payer have been frozen before a
/// resolving effect suspends behind a decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ResolvedEffectPayment {
    Mana(ManaCost),
    Life(u16),
    /// Energy, spent in full or not at all.
    Energy(u16),
    Mill(u16),
    Discard(u16),
    /// One card matching the predicate, named as part of the payment
    /// decision rather than after it.
    DiscardMatching(ObjectPredicateDef),
    /// Generic mana in an amount the payer chooses, named the same way.
    ChosenGenericMana,
    /// One matching permanent, returned to its owner's hand.
    ReturnPermanentMatching(ObjectPredicateDef),
    /// One matching permanent, sacrificed.
    SacrificePermanentMatching(ObjectPredicateDef),
    /// Creatures sacrificed one at a time until their power reaches this
    /// total.
    SacrificeCreaturesWithTotalPower(u16),
}

/// What runs once a discard finishes, and what it counts among the cards
/// that went. Held beside the pending discard rather than inside its
/// continuation: one discard effect produces one follow-up, and a decision
/// per player in between.
#[derive(Clone, Debug)]
pub(super) struct DiscardFollowUp {
    pub(super) counted: ObjectPredicateDef,
    pub(super) effect: ScopedEffect,
    pub(super) object: Box<StackObject>,
    pub(super) context: EffectResolutionContext,
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
    pub(super) presentation: ObjectCharacteristics,
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
    pub(super) cards: Vec<(GameObjectId, ObjectCharacteristics)>,
    pub(super) count: usize,
    pub(super) action: BalanceAction,
    pub(super) cause: ZoneMoveCause,
}

/// Where a spell taken off the stack ends up. A counter sends it to the
/// graveyard, or to exile for the replacement-style counters; `Hand` is for
/// the clauses that return a spell without countering it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CounteredSpellZone {
    Graveyard,
    Exile,
    Hand,
    /// Onto one end of its owner's library. Subtlety puts a spell there
    /// rather than countering it, which is why this enum covers more than
    /// what its name says.
    Library(ZonePlacement),
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
        enters_tapped: bool,
        /// Where the cards found are saved for the follow-up below.
        binding: Option<ObjectSetBindingIndex>,
        /// What runs once the search is answered, with the found cards
        /// bound. Boxed because most searches have none and the variant
        /// would otherwise carry a stack object for all of them.
        follow_up: Option<Box<SearchFollowUp>>,
    },
    /// Cascade's offer: the card the dig turned up, and the whole pile it
    /// walked through. However the offer ends -- cast or declined -- the
    /// pile goes to the bottom of the library in a random order, which is
    /// why the ids travel with it rather than being recomputed.
    CascadeCast {
        player: PlayerId,
        card: GameObjectId,
        exiled: Vec<GameObjectId>,
    },
    /// An activation cost paid by sacrificing a printed number of
    /// permanents, asked one at a time. The activation itself waits: its
    /// costs are not finished, so nothing is on the stack yet.
    ActivationCostSacrifice {
        player: PlayerId,
        /// What is still owed and what may pay it.
        quota: SacrificeQuota,
        /// Everything the activation already chose, boxed for the same
        /// reason every other suspended procedure boxes it.
        pending: Box<PendingActivation>,
        /// What has been named so far, sacrificed together once the last one
        /// is chosen.
        chosen: Vec<GameObjectId>,
    },
    /// A cost paid by sacrificing creatures until their power reaches a
    /// total, asked one creature at a time.
    SacrificeToTotalPower {
        player: PlayerId,
        /// How much power is still owed. Zero or less means the payer may
        /// stop, and the offer includes a way to.
        remaining: i32,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        if_paid: Option<ScopedEffect>,
    },
    /// The pile an entering permanent takes with it, chosen while the entry
    /// waits.
    BattlefieldEntryExile {
        player: PlayerId,
        entering: GameObjectId,
        candidates: Vec<(GameObjectId, CardDefinitionId)>,
    },
    /// The pair of basic land types a substitution was answered with, and
    /// the resolution that asked.
    BasicLandTypeSubstitution {
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    ChooseCards {
        controller: PlayerId,
        destination: ZoneKind,
        placement: ZonePlacement,
        reveal: bool,
        /// The resolution this choice belongs to, carried only when the
        /// choice puts a permanent onto the battlefield carrying something.
        /// What it arrives with is read back off the effect itself, so the
        /// continuation stores the resolution rather than a second copy of
        /// the printed clause.
        arrival: Option<Box<SearchFollowUp>>,
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
    /// A player naming a colour, with everything the answer will be applied
    /// to already settled. The resolving object travels with it for the same
    /// reason a sacrifice follow-up carries one: the effect it produces has
    /// to be attributed to the same source it would have been without the
    /// question in the middle.
    ChooseColor {
        object: Box<StackObject>,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
        targets: Vec<Target>,
        operation: ColorChoiceOperationDef,
        duration: ResolvedEffectDurationDef,
    },
    /// One mana of a colour the controller is choosing, out of a run of
    /// them. "Add one mana of any color for each charge counter removed"
    /// names each mana separately, so the run is answered one colour at a
    /// time and re-queues itself until it is spent.
    ChosenColorMana {
        controller: PlayerId,
        /// The mana each answer produces, with only its colour still open.
        /// Carried whole so that a restriction or a spend rider the clause
        /// attaches survives the question in the middle.
        prototype: Mana,
        /// How many are still unanswered, counting this one.
        remaining: u16,
        /// Which colours the printed clause allows. "Any color" is all five.
        choosable: ColorSet,
    },
    /// One step of "keep one of each of these types, then sacrifice the
    /// rest". Each type is asked separately, and what has been kept so far
    /// travels with the question; the sacrifice happens once the last type
    /// has been answered.
    KeepOnePerType {
        /// Who is choosing, and whose permanents are at stake.
        player: PlayerId,
        /// Who the sacrifice is attributed to.
        controller: PlayerId,
        /// The types still to be asked about, this one first.
        remaining: Vec<CardType>,
        kept: Vec<GameObjectId>,
    },
    ChainLightning {
        player: PlayerId,
        spell: StackObject,
        targets: Vec<Target>,
    },
    Fork {
        /// Fork repaints what it copies; a card copying itself does not. The
        /// colours travel with the decision so one continuation serves both.
        colors: Option<ColorSet>,
        /// Copies still to offer after this one, for storm.
        remaining: u16,
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
    /// An effect the controller was offered and may decline.
    OptionalEffect {
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// "You may cast that card." The card is already in exile with a
    /// permission attached; casting it is a `CastSpell` action taken while
    /// this decision stands, which discards the decision without ever
    /// answering it. Answering the decision is the decline, and takes the
    /// permission back.
    MayCastExiled {
        player: PlayerId,
        card: GameObjectId,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        /// The whole authored clause rather than only its else branch, so
        /// that a rebuilt decision can check it is still the clause it says
        /// it is instead of trusting a detached fragment.
        definition: ScopedEffect,
    },
    /// "Put that card back on top of your library or into your graveyard."
    /// The counter is already on the creature by the time this is asked, so
    /// nothing but the card's destination is left.
    ExploredCardPlacement {
        player: PlayerId,
        revealed: GameObjectId,
    },
    /// "Choose any number of permanents and/or players, then give each
    /// another counter of each kind already there." Nothing is bound: what
    /// each chosen thing gets is read off it when the answer comes back.
    Proliferate {
        candidates: Vec<Target>,
    },
    /// "You may cast target instant or sorcery card from your graveyard
    /// without paying its mana cost." The card has not moved; what it holds
    /// is a lent ability, and answering the decision takes it back.
    MayCastGranted {
        player: PlayerId,
        card: GameObjectId,
        ability: AbilityDef,
    },
    /// "Its owner puts it on their choice of the top or bottom of their
    /// library." The owner answers, not whoever is resolving, so the spell
    /// waits on the stack until they have.
    SpellLibraryEnd {
        owner: PlayerId,
        spell: GameObjectId,
    },
    /// A generic bounded non-targeting object choice. `candidates` is kept
    /// typed because a spell and a permanent are different objects even
    /// though both are addressed by `GameObjectId`.
    ChooseForEffect {
        definition: ScopedEffect,
        binding: ObjectChoiceBindingDef,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        candidates: Vec<Target>,
        effect: ScopedEffect,
    },
    /// A mana payment offered during effect resolution, with either branch
    /// able to continue the same effect program.
    PayOr {
        player: PlayerId,
        payment: ResolvedEffectPayment,
        definition: ScopedEffect,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        if_paid: Option<ScopedEffect>,
        otherwise: Option<ScopedEffect>,
    },
    /// A card name chosen while an effect resolves, with the rest of that
    /// effect waiting on the answer.
    CardNameChoice {
        choices: Vec<String>,
        /// Whose cards the name is matched against, and where.
        searched: PlayerId,
        zone: ZoneKind,
        binding: ObjectSetBindingIndex,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// The divider has selected the first pile. The chooser still has to
    /// choose between the two typed groups before the nested effect runs.
    SplitForEffect {
        definition: ScopedEffect,
        chooser: PlayerId,
        items: Vec<Target>,
        object: Box<StackObject>,
        context: EffectResolutionContext,
    },
    /// The divider's two piles, waiting for the chooser to name one.
    ChoosePileForEffect {
        definition: ScopedEffect,
        first: Vec<Target>,
        second: Vec<Target>,
        chosen: ObjectSetBindingIndex,
        unchosen: ObjectSetBindingIndex,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// The card just drawn, offered to its controller to reveal.
    MiracleReveal {
        card: GameObjectId,
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
        declined: Option<SacrificeDeclined>,
        optional: bool,
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
    /// A Doomsday-style search in progress: the cards chosen go on top of
    /// the library in the order they were chosen, and everything left in the
    /// searched zones is exiled.
    SearchZonesAndExileRest {
        player: PlayerId,
        zones: Vec<ZoneKind>,
        /// Every card the search looked at, fixed before anybody answers:
        /// the spell doing the searching reaches the graveyard while the
        /// decision is still open, and it was never part of the search.
        searched: Vec<GameObjectId>,
    },
    /// A "will of the council" vote in progress. The candidates are frozen
    /// before the first vote is cast, so every voter sees the same ballot;
    /// a permanent that has left by the time the votes are counted simply
    /// is not there to exile.
    Vote {
        candidates: Vec<GameObjectId>,
        /// Who has yet to vote, in the order they vote: the resolving
        /// controller first.
        remaining: Vec<PlayerId>,
        /// One entry per vote cast, in the order they were cast.
        votes: Vec<GameObjectId>,
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
        selection: &'static TopCardSelectionDef,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// The affected object's controller chooses which currently applicable
    /// replacement effect to apply next.
    BattlefieldEntryReplacement {
        candidates: Vec<ApplicableReplacement>,
    },
    /// A replacement its controller may decline as the permanent enters. The
    /// exact authored operation is retained so accepting resumes the same
    /// program that was offered; checkpoint import authenticates it against
    /// the source ability before rebuilding this continuation.
    BattlefieldEntryOptional {
        context: ReplacementEffectContext,
        effect: ReplacementEffectDef,
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
        player: PlayerId,
        payment: ResolvedEffectPayment,
        definition: ReplacementEffectDef,
    },
    BattlefieldEntryScalarChoice {
        context: ReplacementEffectContext,
        choice: BattlefieldEntryScalarChoiceDef,
        choices: Vec<String>,
    },
    /// The permanents an entering copy effect could imitate, plus the option
    /// of entering as itself.
    BattlefieldEntryCopy {
        choices: Vec<GameObjectId>,
        added_types: CardTypeSet,
        retain_printed_subtypes: bool,
        added_abilities: Vec<super::CopiableAbility>,
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
    /// How a triggered ability's fixed total is split among the targets it
    /// just chose. A cast spell settles this while the action is enumerated;
    /// a trigger has already put its targets on the stack, so the split is a
    /// second question with the same answer space.
    TriggerDivision {
        trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
        targets: Vec<Target>,
        divisions: Vec<Vec<u16>>,
    },
}

/// The resolution an answered decision belongs to: the object that was
/// resolving, the context it had built, and the effect to run or to read the
/// rest of the printed clause out of.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SearchFollowUp {
    pub(super) object: StackObject,
    pub(super) context: EffectResolutionContext,
    pub(super) effect: ScopedEffect,
}
