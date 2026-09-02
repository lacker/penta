//! What an effect asks about before it does anything: where a card may be
//! chosen from, and the reusable conditions a clause is gated on.

use super::{
    AppliedEffectDef, ComparisonDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectSetDef, ObjectSetFilterDef, ZoneKind,
};

/// One place an effect may choose an owned card from.
///
/// Outside the game is deliberately not a [`ZoneKind`]: Magic's zones include
/// exile, while a tournament sideboard remains outside the game until an
/// effect brings one of its cards in.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardChoiceSourceDef {
    Zone(ZoneKind),
    OutsideGame,
}

/// A reusable condition evaluated in an effect's source and event context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConditionDef {
    /// At least one object matches this zone, controller, and object query.
    Exists(ObjectQueryDef),
    /// Every listed condition holds. An empty list is vacuously true, which
    /// is what makes it safe to build one from a card's own list of named
    /// permanents without a special case for the shortest one.
    All(&'static [ConditionDef]),
    /// How many objects the query matches, against a printed number. The
    /// counting form of [`Self::Exists`], for the clauses that name a bound
    /// rather than asking whether anything is there at all.
    ///
    /// Held behind a reference so that adding it does not widen every
    /// condition, and with it the mana-ability activation this sits inside.
    ObjectCount(&'static ObjectCountConditionDef),
    /// The condition's controller has begun no more than this many turns.
    /// "Unless it's your first, second, or third turn of the game" counts
    /// the turns you have taken rather than the turn number: on the draw,
    /// your third turn is the game's sixth.
    ControllerTurnsTakenAtMost(u8),
}

/// The parts of a counting condition. Split out of [`ConditionDef`] so the
/// enum stays the width of its smallest useful variant.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectCountConditionDef {
    pub query: ObjectQueryDef,
    pub comparison: ComparisonDef,
    pub amount: u8,
}

/// A predicate over an already-resolved group of objects.
///
/// This does not name where the group comes from. Producers such as
/// `MillUntil` supply the growing group they are evaluating, which lets the
/// same shape express both "until a land" and "until four lands" without
/// baking either stopping rule into the effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectSetPredicateDef {
    pub filter: Option<ObjectSetFilterDef>,
    pub comparison: ComparisonDef,
    pub amount: u8,
}

/// How many members of an already-resolved object set must exist. Unlike an
/// [`ObjectCountConditionDef`], this composes with provenance-based sets such
/// as cards linked in exile rather than querying zones.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectSetCountConditionDef {
    pub objects: &'static ObjectSetDef,
    pub predicate: ObjectSetPredicateDef,
}

impl ObjectSetPredicateDef {
    /// Whether the group contains at least one object matching `object`.
    #[must_use]
    pub const fn contains(object: &'static ObjectPredicateDef) -> Self {
        Self {
            filter: Some(ObjectSetFilterDef::Predicate(object)),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 1,
        }
    }
}

/// The unconditional operation performed by one static effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StaticApplyDef {
    pub recipient: EffectRecipientDef,
    pub effect: AppliedEffectDef,
}

/// A static operation gated by a separately composed object-set condition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConditionalStaticEffectDef {
    pub condition: ObjectSetCountConditionDef,
    pub then: StaticApplyDef,
}
