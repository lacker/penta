use crate::action::{AbilityOrigin, Target};
use crate::card::{
    AbilityDef, BattlefieldEntryScalarChoiceDef, CardTypeSet, ColorChoiceOperationDef, ColorSet,
    CounterKind, EffectDef, ManaCost, ModalSpellDef, ObjectChoiceBindingDef, ObjectPredicateDef,
    ReplacementEffectDef, TurnKindDef, ZoneKind, ZonePlacement,
};
use crate::casting::TargetSelection;
use crate::ids::{CardDefinitionId, GameObjectId, ObjectSetBindingIndex, PlayerId};

use super::{
    AbilitySourceRef, ApplicableReplacement, ApplicableZoneMoveReplacement, CardInstance,
    CastOffer, CastOfferCost, CastSourceZone, DecisionObservation, DecisionZone, DrawReplacement,
    EffectResolutionContext, Mana, ObjectCharacteristics, PendingActivation,
    PendingActivationTargeting, PendingBattlefieldExitBatch, PendingTrigger,
    ReplacementEffectContext, ResolvedEffectDurationDef, SacrificeQuota, SacrificedAmountDef,
    ScopedEffect, StackObject, TapQuota, TriggerPlacementBatch,
};

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
    /// Energy in an amount the payer chooses.
    ChosenEnergy,
    /// One matching permanent, moved to the named zone.
    MovePermanentMatching {
        object: ObjectPredicateDef,
        zone: ZoneKind,
    },
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
    pub(super) bound: Option<ObjectSetBindingIndex>,
    /// The authored discard whose nested follow-up this is. Checkpoints
    /// relocate the parent and recover the counted predicate and binding
    /// from it rather than serializing catalog definitions.
    pub(super) definition: ScopedEffect,
    pub(super) effect: ScopedEffect,
    pub(super) object: Box<StackObject>,
    pub(super) context: EffectResolutionContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Pregame {
    Mulligan(PlayerId),
    Bottom(PlayerId),
    OpeningHand(PlayerId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PregameAbilityAction {
    pub(super) source: GameObjectId,
    pub(super) ability: crate::AbilityOrigin,
    pub(super) cost_objects: Vec<GameObjectId>,
}

impl PregameAbilityAction {
    pub(super) fn action(&self) -> crate::Action {
        crate::Action::ActivateAbility {
            source: self.source,
            ability: self.ability,
            targets: Vec::new(),
            cost_objects: self.cost_objects.clone(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        }
    }
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
    /// The chooser may take any remaining opening-hand action, in any order,
    /// or answer this zero-option decision to finish their window.
    PregameActions {
        player: PlayerId,
        actions: Vec<PregameAbilityAction>,
    },
    ScryBottom {
        player: PlayerId,
        revealed: Vec<CardInstance>,
    },
    /// A creature that arrived attacking has to be told what it is
    /// attacking: the defending player, or one of their planeswalkers
    /// (CR 506.3d). One decision per arriving attacker, with the ones still
    /// waiting carried along.
    ArrivingAttackerDefender {
        player: PlayerId,
        defending: PlayerId,
        attackers: Vec<GameObjectId>,
    },
    ScryTop {
        player: PlayerId,
        top: Vec<CardInstance>,
        bottom: Vec<CardInstance>,
    },
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
        follow_up: Option<Box<DiscardFollowUp>>,
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
        /// A searched Aura enters attached to this player.
        attached_player: Option<PlayerId>,
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
    /// Crew's and saddle's cost, paid one creature at a time while the
    /// activation waits.
    ActivationCostTap {
        player: PlayerId,
        /// How much power is still owed. Zero or less means the payer may
        /// stop, and the offer includes a way to.
        remaining: i32,
        pending: Box<PendingActivation>,
        chosen: Vec<GameObjectId>,
    },
    /// An exact-count tap cost, paid one matching permanent at a time while
    /// the activation waits.
    ActivationCostTapPermanents {
        player: PlayerId,
        quota: TapQuota,
        pending: Box<PendingActivation>,
        chosen: Vec<GameObjectId>,
    },
    /// A target another player chooses while an activated ability is being
    /// declared. Costs remain unpaid and nothing is on the stack yet.
    ActivationTargeting {
        pending: Box<PendingActivationTargeting>,
        candidates: Vec<Target>,
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
        /// The resolution this choice belongs to when a composed follow-up
        /// needs the identities created by moving the chosen cards.
        arrival: Option<Box<SearchFollowUp>>,
    },
    /// The affected player chooses which of several applicable next-draw
    /// replacements consumes this draw. Unchosen replacements remain live.
    DrawReplacement {
        player: PlayerId,
        applied: Vec<AbilitySourceRef>,
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
    /// Choose one counter kind currently on an object, then continue the
    /// nested effect with that kind bound in its resolution context.
    ChooseCounter {
        object: Box<StackObject>,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
        target: Target,
        kinds: Vec<CounterKind>,
    },
    /// Choose one labelled effect branch during resolution.
    ChooseEffect {
        object: Box<StackObject>,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
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
    /// One step of a declarative multi-player choice. All chosen permanents
    /// travel together until every player has answered, then the definition
    /// binds both halves and resumes its ordinary nested effect.
    SimultaneousChoose {
        definition: ScopedEffect,
        task: usize,
        players: Vec<PlayerId>,
        chosen: Vec<GameObjectId>,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        candidates: Vec<GameObjectId>,
    },
    ChainLightning {
        player: PlayerId,
        spell: StackObject,
        targets: Vec<Target>,
    },
    CopyStackObject {
        /// A copy-process color exception, when the originating instruction
        /// changes the copied object's colors.
        colors: Option<ColorSet>,
        /// Copies still to offer after this one, for storm.
        remaining: u16,
        player: PlayerId,
        spell: StackObject,
        target_lists: Vec<Vec<TargetSelection>>,
    },
    /// A resolving effect has enumerated legal replacement targets for an
    /// existing stack object and is waiting for the chooser to select one.
    ChangeStackTargets {
        object: GameObjectId,
        target_lists: Vec<Vec<TargetSelection>>,
    },
    RecallDiscard {
        player: PlayerId,
    },
    RecallReturn {
        player: PlayerId,
    },
    /// Endure N (CR 702.183a): the counters or the Spirit, chosen as the
    /// ability resolves. Neither branch needs the resolving object, so
    /// nothing about it is carried here.
    Endure {
        player: PlayerId,
        permanent: GameObjectId,
        amount: u16,
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
    /// Suspend's last-counter instruction. Casting the card answers this
    /// standing decision; there is no decline while a legal cast exists.
    CastSuspended {
        player: PlayerId,
        card: GameObjectId,
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
        grant: usize,
        /// Where the lent cast comes from. Derived from where the card
        /// actually is rather than stored on the wire, because it is the
        /// same fact: a graveyard for the clauses that buy a spell back,
        /// exile for rebound's own card waiting there.
        source_zone: CastSourceZone,
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
    /// Every member of one frozen collection is waiting to be submitted in the
    /// order a later effect should consume it.
    ChooseObjectOrderForEffect {
        definition: ScopedEffect,
        candidates: Vec<Target>,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// A pure private look has shown its collection and waits only for the viewer
    /// to acknowledge it before resolution continues.
    LookAtObjectsForEffect {
        definition: ScopedEffect,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// The divider is choosing the first half of a generic two-pile
    /// partition.  Both piles are bound before its nested effect resumes.
    PartitionGroupForEffect {
        definition: ScopedEffect,
        items: Vec<Target>,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// Two previously formed groups wait for one player to choose between
    /// them.
    ChooseGroupForEffect {
        definition: ScopedEffect,
        first: Vec<Target>,
        second: Vec<Target>,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    },
    /// One actor is making an optional distinct pick for each predicate in a
    /// generic choose-one-of-each stage.
    ChooseOneOfEachForEffect {
        definition: ScopedEffect,
        next: usize,
        candidates: Vec<Target>,
        remaining: Vec<Target>,
        chosen: Vec<Target>,
        object: Box<StackObject>,
        context: EffectResolutionContext,
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
    /// The first card a player drew this turn, waiting for one optional
    /// private draw-specific action. An empty answer takes no action.
    DrawActionWindow {
        card: GameObjectId,
    },
    /// A linked trigger has resolved and offers one exact alternative way to
    /// cast its source card. Casting answers the decision; choosing its sole
    /// option declines.
    MayCastAlternative {
        player: PlayerId,
        card: GameObjectId,
        ability: AbilityOrigin,
    },
    /// A sacrifice an effect demanded, chosen by the sacrificing player.
    SacrificeOfChoice {
        followup: Option<SacrificeFollowup>,
        declined: Option<SacrificeDeclined>,
        optional: bool,
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
    /// Replacement effects for a simultaneous exit batch are final. One
    /// library owner is now arranging the cards that the event puts into the
    /// same position, with later APNAP groups still waiting behind it.
    BattlefieldExitOrder {
        batch: PendingBattlefieldExitBatch,
        remaining: Vec<Vec<GameObjectId>>,
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
        base_power_toughness: Option<(i16, i16)>,
        colors: Option<crate::card::ColorSet>,
        added_creature_types: Vec<&'static str>,
        no_mana_cost: bool,
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
    /// A modal trigger waiting for the mode it goes on the stack with.
    TriggerMode {
        trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
        modes: ModalSpellDef,
    },
    TriggerDivision {
        trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
        targets: Vec<Target>,
        divisions: Vec<Vec<u16>>,
    },
}

impl DecisionContinuation {
    pub(super) fn pregame_actions(&self, player: PlayerId) -> Option<&[PregameAbilityAction]> {
        match self {
            Self::PregameActions {
                player: chooser,
                actions,
            } if *chooser == player => Some(actions),
            _ => None,
        }
    }

    pub(super) fn cast_offer(&self) -> Option<CastOffer> {
        match self {
            Self::MayCastExiled { player, card, .. }
            | Self::CastSuspended { player, card }
            | Self::CascadeCast { player, card, .. } => Some(CastOffer {
                player: *player,
                card: *card,
                source_zone: CastSourceZone::Exile,
                cost: CastOfferCost::Any,
            }),
            Self::MayCastGranted {
                player,
                card,
                grant,
                source_zone,
                ..
            } => Some(CastOffer {
                player: *player,
                card: *card,
                source_zone: *source_zone,
                cost: CastOfferCost::GrantedAlternative(*grant),
            }),
            Self::MayCastAlternative {
                player,
                card,
                ability,
            } => Some(CastOffer {
                player: *player,
                card: *card,
                source_zone: CastSourceZone::Hand,
                cost: CastOfferCost::PrintedAlternative(*ability),
            }),
            _ => None,
        }
    }

    pub(super) fn cast_offer_is_mandatory(&self) -> bool {
        matches!(self, Self::CastSuspended { .. })
    }
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
