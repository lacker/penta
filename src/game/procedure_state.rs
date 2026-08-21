use crate::ids::PlayerId;

use super::{CardBehavior, EffectResolutionContext, ScopedEffect, StackObject};

/// A duration-limited replacement for one player's next draw.
///
/// The resolving object is frozen because the source has normally left the
/// battlefield by the time the draw happens. Ring of Ma'rûf exiles itself as
/// a cost before it creates this replacement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DrawReplacement {
    pub(super) object: Box<StackObject>,
    pub(super) context: EffectResolutionContext,
    pub(super) effect: ScopedEffect,
}

/// Rules procedures that paused behind a decision and must finish before
/// state-based actions, trigger placement, or priority.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub(super) enum PendingProcedure {
    DrawCards {
        player: PlayerId,
        remaining: u16,
    },
    ResolveEffects {
        effects: Vec<ScopedEffect>,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        custom_followup: Option<CardBehavior>,
    },
    SylvanAfterDraw {
        player: PlayerId,
    },
    SimultaneousDraws {
        remaining: [u16; 2],
        next: PlayerId,
        was_deferred: bool,
    },
    /// Finish a search only after any prospective battlefield entry and its
    /// replacement choices have completed.
    ShuffleLibrary {
        player: PlayerId,
    },
    /// A resolving spell or ability suspended behind one of the procedures
    /// above. Its source remains a resolving stack object until every part of
    /// the effect has completed, then takes its normal resolution destination.
    FinishStackResolution {
        object: Box<StackObject>,
        resolved: bool,
    },
    FinishStepAdvance,
}
