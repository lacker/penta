//! The composite shapes an effect carries: a bounded choice among objects,
//! a partition into piles, a triggered ability installed by a resolution,
//! what a token clause puts on the tokens it makes, and how a discard picks
//! and counts. Each is a small vocabulary of its own that several effects
//! reach for, rather than a variant of any one of them.

use super::super::{
    AbilityDef, ChoiceVisibilityDef, ColorSet, CounterKind, CreatureTypeSetDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerSetDef,
    ResolvedEffectDurationDef, ValueDef, ZoneKind,
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

/// Each affected player chooses one permanent they control for every
/// predicate in `one_of_each`. Choices are locked in APNAP order before the
/// chosen and unchosen unions are bound and the nested effect continues.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SimultaneousChooseDef {
    pub player: EffectRecipientDef,
    /// The complete universe divided into `chosen` and `unchosen`.
    pub candidates: ObjectPredicateDef,
    /// One distinct matching permanent is chosen for each predicate when one
    /// exists. A permanent already chosen for an earlier predicate is not
    /// offered again.
    pub one_of_each: &'static [ObjectPredicateDef],
    pub chosen: ObjectSetBindingIndex,
    pub unchosen: ObjectSetBindingIndex,
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
    /// The same, with colourless among the choices. "Protection from
    /// colorless or from the color of your choice" is one choice of six
    /// rather than a colour choice with a rider: colourless is a quality a
    /// source has by having no colour, and the player picks it or a colour.
    ProtectionFromChosenColorOrColorless,
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

/// Revealing from the top of a library until a matching card turns up.
///
/// What was passed over goes to the graveyard; the match itself goes where
/// `matched_zone` says, which is the graveyard for a plain dig and the hand
/// for Hermit Druid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MillUntilDef {
    pub player: EffectRecipientDef,
    pub object: ObjectPredicateDef,
    pub matched_zone: ZoneKind,
    /// Saves the identities of cards this effect put into a graveyard for a
    /// same-resolution follow-up. They are bound under their new zone
    /// identities rather than reconstructed from the graveyard. When the
    /// matching card has another destination, use [`ValueDef::MatchedCount`]
    /// to count every revealed card; the binding contains only the cards
    /// that were milled.
    pub binding: Option<ObjectSetBindingIndex>,
    /// Runs immediately after the named reveal-and-move procedures.
    /// [`ValueDef::MatchedCount`] describes every card revealed, including a
    /// match sent somewhere other than a graveyard.
    pub then: Option<&'static EffectDef>,
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

/// What a permission to play an exiled card asks for where it is used.
///
/// A closed vocabulary rather than a general condition: the permission
/// outlives the resolution that granted it, so what it asks has to be
/// something a checkpoint can write down and read back.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExilePlayConditionDef {
    /// "During any turn you attacked with a Rogue." Read off the creatures
    /// that attacked this turn, so a Rogue that attacked and then died is
    /// not among them.
    AttackedWithSubtypeThisTurn(&'static str),
}

/// How long a permission to play an exiled card lasts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExilePlayDurationDef {
    /// "This turn", which is the turn the card was exiled on and no longer.
    ThisTurn,
    /// "Until your next end step", which reaches into the holder's own turn
    /// when the card was exiled on somebody else's.
    UntilYourNextEndStep,
    /// For as long as the card is in exile. What bounds it is not a turn but
    /// whatever the clause granting it says: Robber of the Rich hands one
    /// out that only works on the turns you attacked with a Rogue.
    WhileExiled,
}

/// What a token copy is created "except" for.
///
/// Offspring names one of these and embalm and eternalize name four, but
/// they are the same kind of thing: copy exceptions, which CR 707.9a makes
/// copiable values in their own right. A later copy of the token copies
/// them along with everything else.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TokenCopyExceptionsDef {
    /// "Except it's a 1/1", or a 4/4.
    pub base_power_toughness: Option<(i16, i16)>,
    /// "Except it's black": the colours it has instead of the ones it
    /// copied, rather than in addition to them.
    pub colors: Option<ColorSet>,
    /// "Except it's a Zombie <its own types>": creature types on top of the
    /// ones it copied.
    pub added_creature_types: CreatureTypeSetDef,
    /// "With no mana cost", which is what makes an eternalized card's mana
    /// value zero.
    pub no_mana_cost: bool,
}

impl TokenCopyExceptionsDef {
    /// A plain copy, with nothing said after "except".
    pub const NONE: Self = Self {
        base_power_toughness: None,
        colors: None,
        added_creature_types: CreatureTypeSetDef::named(&[]),
        no_mana_cost: false,
    };

    #[must_use]
    pub const fn power_toughness(power: i16, toughness: i16) -> Self {
        Self {
            base_power_toughness: Some((power, toughness)),
            ..Self::NONE
        }
    }

    /// The list embalm and eternalize print: a body, a colour, a type on
    /// top of the ones it had, and no mana cost.
    #[must_use]
    pub const fn undead(
        power: i16,
        toughness: i16,
        colors: ColorSet,
        added_creature_types: &'static [&'static str],
    ) -> Self {
        Self {
            base_power_toughness: Some((power, toughness)),
            colors: Some(colors),
            added_creature_types: CreatureTypeSetDef::named(added_creature_types),
            no_mana_cost: true,
        }
    }
}
