use crate::card::{ManaCost, ReplacementAbilityDef, ReplacementEffectDef, SpendModeDef, ZoneKind};
use crate::ids::{CardDefinitionId, GameObjectId, PlayerId};

use super::{
    AbilitySourceRef, BalancePhase, BalanceTask, EffectResolutionContext, FrozenActivatedAbility,
    Game, Mana, ManaAbilityActivation, ManaPaymentPurpose, ObjectCharacteristics, ObjectInstance,
    Permanent, PlannedManaActivation, SacrificeFollowup, ScopedEffect, StackObject, Target,
    TargetSelection,
};

/// One replacement effect that currently applies to a prospective event.
///
/// The source and ability origin form the per-event identity required by
/// rule 614.5: after this instance changes the event, it cannot apply to that
/// same event again even if re-evaluation still finds it applicable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ReplacementEffectContext {
    pub(super) source: AbilitySourceRef,
    pub(super) controller: PlayerId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ApplicableReplacement {
    pub(super) context: ReplacementEffectContext,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) text: &'static str,
    pub(super) optional: bool,
    pub(super) effect: ReplacementEffectDef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PendingReplacementEffect {
    pub(super) context: ReplacementEffectContext,
    pub(super) effect: ReplacementEffectDef,
}

/// One replacement that currently applies to one member of a simultaneous
/// battlefield-exit batch. The move index keeps a suspended CR 616 choice
/// attached to the exact prospective event it was offered for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ApplicableZoneMoveReplacement {
    pub(super) move_index: usize,
    pub(super) context: ReplacementEffectContext,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) text: &'static str,
    pub(super) effect: ReplacementEffectDef,
}

/// Event-local state accumulated while replacement effects change one
/// battlefield-to-graveyard proposal. No object leaves the battlefield until
/// every member of the simultaneous batch has reached a final destination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingBattlefieldExitMove {
    pub(super) object: GameObjectId,
    pub(super) controller: PlayerId,
    pub(super) destination: ZoneKind,
    pub(super) replaced_with_nothing: bool,
    pub(super) applied: Vec<AbilitySourceRef>,
}

/// An effective replacement ability frozen before any member of a
/// simultaneous exit batch leaves. This preserves the continuous effects used
/// to determine the single event under CR 400.6, including copied, granted,
/// and removed abilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FrozenZoneMoveReplacement {
    pub(super) source: AbilitySourceRef,
    pub(super) controller: PlayerId,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) text: &'static str,
    pub(super) replacement: ReplacementAbilityDef,
    pub(super) effect: ReplacementEffectDef,
}

#[derive(Clone, Debug)]
pub(super) struct PendingBattlefieldExitBatch {
    pub(super) moves: Vec<PendingBattlefieldExitMove>,
    pub(super) replacements: Vec<FrozenZoneMoveReplacement>,
    /// Work belonging to the same atomic rules procedure. A replacement-order
    /// choice can suspend the prospective move, so callers install their next
    /// operation here instead of running it before the move commits.
    pub(super) completion: Option<Box<BattlefieldExitCompletion>>,
}

/// The small set of engine procedures that can still owe work after moving a
/// permanent toward a graveyard. Keeping the completion on the prospective
/// batch makes suspension local: no half-paid cast or activation is exposed as
/// a separate global state, and a synchronous batch pays no allocation beyond
/// the optional box.
#[derive(Clone, Debug)]
pub(super) enum BattlefieldExitCompletion {
    Completions(Vec<BattlefieldExitCompletion>),
    ResolveEffects {
        object: Box<StackObject>,
        context: EffectResolutionContext,
        effects: Vec<ScopedEffect>,
    },
    FinishStackResolution {
        object: Box<StackObject>,
        resolved: bool,
    },
    SacrificeFollowup {
        followup: SacrificeFollowup,
        sacrificed: Option<GameObjectId>,
    },
    Balance {
        controller: PlayerId,
        phase: BalancePhase,
        remaining: Vec<BalanceTask>,
    },
    CompleteSpellCast {
        object: Box<StackObject>,
        targets: Vec<Target>,
        remaining_sacrifices: Vec<(GameObjectId, SpendModeDef)>,
    },
    CompleteActivatedAbility {
        source: GameObjectId,
        source_card: ObjectInstance,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
        remaining_sacrifices: Vec<GameObjectId>,
    },
    CompleteManaAbility {
        player: PlayerId,
        activation: ManaAbilityActivation,
        produced_mana: Vec<Mana>,
    },
    ContinueSpellManaPayment {
        object: Box<StackObject>,
        targets: Vec<Target>,
        object_payments: Vec<(GameObjectId, SpendModeDef)>,
        cost: ManaCost,
        x: u16,
        purpose: ManaPaymentPurpose,
        plan: Vec<PlannedManaActivation>,
        next_activation: usize,
    },
}

impl Game {
    /// Resolves an ordered effect program, suspending its tail on a
    /// battlefield-exit replacement choice when one is required.
    pub(super) fn resolve_effect_defs(
        &mut self,
        effects: Vec<ScopedEffect>,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        let mut effects = effects.into_iter();
        while let Some(effect) = effects.next() {
            let pending_before = self.pending_decisions.len();
            self.resolve_effect_def(effect, object, context.clone());
            let remaining = effects.as_slice();
            if !remaining.is_empty()
                && self.defer_after_battlefield_exit(
                    pending_before,
                    BattlefieldExitCompletion::ResolveEffects {
                        object: Box::new(object.clone()),
                        context: context.clone(),
                        effects: remaining.to_vec(),
                    },
                )
            {
                return;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum EntryCompletion {
    LandPlayed {
        player: PlayerId,
    },
    SpellResolved {
        card: GameObjectId,
        definition: CardDefinitionId,
    },
    AttachSource {
        source: GameObjectId,
    },
    /// The mirror of the above: what arrives attaches to a permanent that is
    /// already there. "Return this card from your graveyard to the
    /// battlefield, then attach it to that creature" moves the Equipment, so
    /// the host is the one thing that keeps its identity through the entry.
    AttachToHost {
        host: GameObjectId,
    },
    /// "Create a token that's tapped and attacking." The attack is declared
    /// as the token enters rather than afterwards, because what enters is a
    /// new object and the declaration step is long over.
    Attacking {
        defender: crate::AttackDefender,
    },
    /// The development setup surface minted this object's battlefield
    /// identity directly, so committing it must not reincarnate it again.
    Setup,
    None,
}

/// Mutable state for an object that would enter the battlefield. The object
/// deliberately remains outside every public zone until replacement effects
/// finish and `commit_battlefield_entry` gives it its destination object ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingBattlefieldEntry {
    pub(super) permanent: Permanent,
    pub(super) from: ZoneKind,
    pub(super) completion: EntryCompletion,
    /// Where this goes instead of the battlefield. Mox Diamond's unpaid entry
    /// is the case: the permanent never arrives, so nothing about it enters,
    /// and the card is put where the replacement sent it.
    pub(super) redirected_to: Option<ZoneKind>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ReplaceableEvent {
    BattlefieldEntry(PendingBattlefieldEntry),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingEvent {
    pub(super) event: ReplaceableEvent,
    pub(super) applied: Vec<AbilitySourceRef>,
    /// A LIFO program of event-local modifications. Replacement clauses can
    /// suspend this program for a choice and resume it without committing.
    pub(super) effects: Vec<PendingReplacementEffect>,
}
