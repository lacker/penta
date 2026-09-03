use crate::card::{ObjectPredicateDef, ValueDef};

/// Which sources a relational prevention answers. The variants name rules
/// rather than cards, but the list is deliberately closed: a prevention has
/// to survive a checkpoint, and a whole predicate has no serialised form.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RelationalSourceFilter {
    CreaturesWithFlying,
    AttackingCreaturesWithoutFlying,
    Artifacts,
    UnblockedCreatures,
}

use super::{
    AbilitySourceRef, ContinuousEffectExpiration, ContinuousEffectTimestamp, GameObjectId,
    PlayerId, Target,
};

/// The source side of one resolved damage-prevention rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ResolvedDamageSourceMatcher {
    Any,
    Exact(GameObjectId),
    Except(GameObjectId),
    Matching {
        predicate: ObjectPredicateDef,
        relative_to: GameObjectId,
    },
    Group(RelationalSourceFilter),
}

/// The recipient side of one resolved damage-prevention rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ResolvedDamageRecipientMatcher {
    Any,
    Exact(Target),
    /// The named player plus creatures that player controls when damage would
    /// be dealt. The creature set is intentionally dynamic.
    PlayerAndCreaturesControlledBy(PlayerId),
}

/// How many matching damage events or points a resolved rule can still
/// prevent. Amount capacities can span several events; event capacities are
/// consumed by a match even when their deferred amount rounds to zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ResolvedDamagePreventionCapacity {
    Amount(u16),
    Events(u16),
    Unlimited,
}

/// One installed damage-prevention rule. Static rules are evaluated live and
/// are not stored here; every entry in this vector was created by resolving a
/// spell or ability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedDamagePrevention {
    pub(super) source: ResolvedDamageSourceMatcher,
    pub(super) recipient: ResolvedDamageRecipientMatcher,
    pub(super) combat_only: bool,
    pub(super) capacity: ResolvedDamagePreventionCapacity,
    /// Finalized against the current damage amount only when this rule is
    /// applied, after earlier modifications have changed the event.
    pub(super) amount: ValueDef,
    /// A frozen player who gains the amount actually prevented, when the
    /// authored rule has that rider.
    pub(super) gain_life: Option<PlayerId>,
    pub(super) source_ability: AbilitySourceRef,
    pub(super) timestamp: ContinuousEffectTimestamp,
    pub(super) expiration: ContinuousEffectExpiration,
}

/// A resolved one-turn replacement that moves damage from one named source
/// and player to one named permanent. Redirection is applied before every
/// prevention rule and therefore is stored separately from prevention.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedDamageRedirect {
    pub(super) player: PlayerId,
    pub(super) source: GameObjectId,
    pub(super) destination: GameObjectId,
    pub(super) expiration: ContinuousEffectExpiration,
}
