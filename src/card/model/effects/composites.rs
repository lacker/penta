//! The composite shapes an effect carries: a bounded choice among objects,
//! a partition into piles, a triggered ability installed by a resolution,
//! what a token clause puts on the tokens it makes, and how a discard picks
//! and counts. Each is a small vocabulary of its own that several effects
//! reach for, rather than a variant of any one of them.

use super::super::{
    AbilityDef, ChoiceVisibilityDef, CounterKind, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerSetDef,
    ResolvedEffectDurationDef, ValueDef,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex};

/// The context slot populated by an object choice.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectChoiceBindingDef {
    Object(ObjectBindingIndex),
    Objects(ObjectSetBindingIndex),
    /// Preserve the chooser's submitted order for a continuation that acts on
    /// the chosen objects one at a time.
    OrderedObjects(ObjectSetBindingIndex),
}

/// Choose a bounded number of non-targeted objects, save them in the resolving
/// context, then continue the effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseDef {
    pub binding: ObjectChoiceBindingDef,
    /// Where the candidates that were *not* chosen are saved, when the
    /// printed clause goes on to say what happens to them. "Put that card
    /// into your hand and the rest into your graveyard" names both halves of
    /// one partition, so both have to be nameable.
    pub unchosen: Option<ObjectSetBindingIndex>,
    pub chooser: PlayerRefDef,
    pub candidates: ObjectSetDef,
    pub exclude: Option<ObjectRefDef>,
    pub minimum: usize,
    pub maximum: usize,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}

/// What a named colour is used for once it has been chosen.
///
/// An operation rather than a general effect: the colour has to reach a
/// characteristic leaf, and only the leaves that take one are meaningful
/// here.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorChoiceOperationDef {
    /// Gain protection from the chosen colour.
    ProtectionFromChosenColor,
    /// Become the chosen colour, replacing whatever colours it had.
    BecomesChosenColor,
}

/// The objects divided by a pile-splitting procedure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PartitionItemsDef {
    Objects(ObjectSetDef),
    TopOfLibrary {
        player: PlayerRefDef,
        count: ValueDef,
    },
}

/// Divide objects into two piles, choose one pile, bind both results, and then
/// continue the effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SplitIntoPilesDef {
    pub items: PartitionItemsDef,
    pub divider: PlayerSetDef,
    pub chooser: PlayerSetDef,
    pub chosen: ObjectSetBindingIndex,
    pub unchosen: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// How long an effect-created triggered ability listens from outside every
/// zone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InstalledTriggerLifetimeDef {
    Once,
    UntilNextTurn(PlayerRefDef),
}

/// A triggered ability installed by a resolving effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InstalledTriggerDef {
    pub ability: &'static AbilityDef,
    pub lifetime: InstalledTriggerLifetimeDef,
}

/// A resolving effect that remains outside every zone and offers one
/// activated ability for a fixed duration.
///
/// When present, the affected recipient is frozen into `binding` as the
/// ongoing effect is created, so the nested ability can read it without
/// targeting it again. An unbound effect instead carries a self-contained
/// ability such as Channel's mana ability. The ongoing effect is a game object
/// for ability-source identity, but it is not a permanent and cannot pay costs
/// that require permanent state. Penta treats it as command-zone-resident for
/// source-zone checks. The rules effect does not technically occupy a zone,
/// but that approximation is gameplay-indistinguishable while the object
/// remains untargetable and separate from emblems.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OngoingEffectDef {
    pub affected: Option<EffectRecipientDef>,
    pub binding: Option<ObjectBindingIndex>,
    pub ability: &'static AbilityDef,
    pub duration: ResolvedEffectDurationDef,
}

impl OngoingEffectDef {
    #[must_use]
    pub const fn new(
        affected: EffectRecipientDef,
        binding: ObjectBindingIndex,
        ability: &'static AbilityDef,
        duration: ResolvedEffectDurationDef,
    ) -> Self {
        Self {
            affected: Some(affected),
            binding: Some(binding),
            ability,
            duration,
        }
    }

    /// Creates an ongoing effect whose ability does not refer back to an
    /// affected object. Channel is the representative shape.
    #[must_use]
    pub const fn unbound(
        ability: &'static AbilityDef,
        duration: ResolvedEffectDurationDef,
    ) -> Self {
        Self {
            affected: None,
            binding: None,
            ability,
            duration,
        }
    }
}

impl InstalledTriggerDef {
    #[must_use]
    pub const fn once(ability: &'static AbilityDef) -> Self {
        Self {
            ability,
            lifetime: InstalledTriggerLifetimeDef::Once,
        }
    }

    #[must_use]
    pub const fn until_next_turn(ability: &'static AbilityDef, player: PlayerRefDef) -> Self {
        Self {
            ability,
            lifetime: InstalledTriggerLifetimeDef::UntilNextTurn(player),
        }
    }
}

/// Counters a token is created with.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TokenCountersDef {
    pub kind: CounterKind,
    pub amount: ValueDef,
}

/// What happens next to the tokens a clause just created.
///
/// A sequence hands each component its own copy of the resolution context,
/// so a binding made in one component is gone by the next. A clause that has
/// to name exactly the tokens it made therefore nests its continuation the
/// way every other binding clause does.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreatedTokensDef {
    pub binding: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// What follows a discard, and what it counts among the cards that went.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiscardFollowUpDef {
    /// Which discarded cards the follow-up counts, read back with
    /// [`ValueDef::MatchedCount`].
    pub counted: ObjectPredicateDef,
    /// Where the counted cards are saved, so the follow-up can name them
    /// rather than only count them. "You may cast the discarded card" needs
    /// the card itself, and by the time the follow-up runs it is one card in
    /// a graveyard among however many were already there.
    pub bound: Option<ObjectSetBindingIndex>,
    pub effect: &'static EffectDef,
}

/// How cards are selected for a discard effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiscardSelectionDef {
    /// Each affected player chooses cards from their own hand.
    RecipientChooses,
    /// The engine selects cards using the recorded random seed.
    Random,
    /// The same, but only from the cards in hand that match. "Discards a
    /// creature card at random" leaves everything else where it is, and
    /// discards nothing when the hand holds none.
    RandomMatching(&'static ObjectPredicateDef),
}

/// How long a permission to play an exiled card lasts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExilePlayDurationDef {
    /// "This turn", which is the turn the card was exiled on and no longer.
    ThisTurn,
    /// "Until your next end step", which reaches into the holder's own turn
    /// when the card was exiled on somebody else's.
    UntilYourNextEndStep,
}
