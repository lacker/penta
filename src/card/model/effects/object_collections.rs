//! Resumable operations over frozen collections of cards and objects.
//!
//! A producer such as [`BindObjectsDef`] freezes one collection in the
//! resolving context. The stages below transform that collection, bind their
//! results, and resume an ordinary nested effect. Information, choices, and
//! zone changes remain separate operations over the same identities.

use super::super::FaceDownCharacteristics;
use super::{
    ChoiceVisibilityDef, EffectDef, ObjectPredicateDef, ObjectSetDef, PlayerRefDef, ValueDef,
    ZonePlacement,
};
use crate::ids::ObjectSetBindingIndex;

/// How a collection is materialized before any information, choice, or zone
/// action occurs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectCollectionSourceDef {
    /// An already addressable set, usually a binding from an earlier stage.
    ObjectSet(ObjectSetDef),
    /// The first `count` cards, in top-first order.
    TopCards {
        player: PlayerRefDef,
        count: ValueDef,
    },
    /// The top of a library through and including its first matching card.
    /// If no card matches, this is the whole library. Looking, revealing,
    /// and moving are deliberately left to later stages.
    TopCardsThroughFirstMatching {
        player: PlayerRefDef,
        object: ObjectPredicateDef,
    },
}

/// How the cards in a collection become known while a choice is made.
///
/// This is intentionally smaller than general decision visibility: a card
/// either says one player looks at the collection or says to reveal it to
/// everyone. More elaborate handoffs remain explicit workflow stages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CollectionInspectionDef {
    Look,
    Reveal,
}

/// The common single-decision procedure "look at/reveal these cards, choose
/// some matching cards from among them, then do one thing with those cards
/// and another with the rest."
///
/// The follow-up names the two groups. It keeps movement and ordering in the
/// ordinary collection effects while hiding the implementation-only source
/// binding, eligibility split, and recombination from card declarations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseCardsFromCollectionDef {
    pub source: ObjectCollectionSourceDef,
    pub actor: PlayerRefDef,
    pub inspection: CollectionInspectionDef,
    pub object: ObjectPredicateDef,
    pub minimum: usize,
    pub maximum: usize,
    pub chosen: ObjectSetBindingIndex,
    pub remainder: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// The mandatory counterpart to [`ChooseCardsFromCollectionDef`]: reveal a
/// collection, split every card by one predicate, and continue with the two
/// resulting groups. No player decision is created.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RevealAndClassifyCardsDef {
    pub source: ObjectCollectionSourceDef,
    pub object: ObjectPredicateDef,
    pub matching: ObjectSetBindingIndex,
    pub remainder: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// Materialize a collection source, freeze its exact identities under a name,
/// and continue. Information and zone changes are separate later stages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BindObjectsDef {
    pub source: ObjectCollectionSourceDef,
    pub binding: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// Divide one frozen collection according to a predicate without asking a
/// player.  Mandatory "put all matching cards ... and the rest ..." clauses
/// are classifications, not bounded choices.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ClassifyObjectsDef {
    pub input: ObjectSetDef,
    pub object: ObjectPredicateDef,
    pub matching: ObjectSetBindingIndex,
    pub remainder: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// Concatenate named collections in authored order and continue. This closes the
/// common partition shape where a choice is made only from the eligible half
/// but "the rest" includes both declined eligible cards and ineligible ones.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CombineObjectsDef {
    pub inputs: &'static [ObjectSetDef],
    pub combined: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// Continue down one of two authored branches according to whether a frozen
/// collection has any members. This makes empty-result handling explicit without
/// changing the behavior of the choice or movement stages on either branch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IfNoObjectsDef {
    pub input: ObjectSetDef,
    pub if_empty: &'static EffectDef,
    pub otherwise: &'static EffectDef,
}

/// Let one player arrange every member of a frozen collection. The placement is
/// only the player-facing interpretation of the answer (top-first,
/// bottom-first, or upper-first); movement remains a separate effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseObjectOrderDef {
    pub actor: PlayerRefDef,
    pub input: ObjectSetDef,
    pub ordered: ObjectSetBindingIndex,
    pub placement: ZonePlacement,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}

/// Randomize a collection's order with the replay-stable game RNG and continue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RandomizeObjectOrderDef {
    pub input: ObjectSetDef,
    pub randomized: ObjectSetBindingIndex,
    pub then: &'static EffectDef,
}

/// Reveal every card in a collection, then continue. Revelation is information;
/// moving or otherwise acting on the cards remains a separate stage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RevealObjectsDef {
    pub input: ObjectSetDef,
    pub then: &'static EffectDef,
}

/// Move a frozen collection as one ordered instruction and bind the objects the
/// cards become. Unlike the single-object movement primitive, this preserves
/// the group's relative order at a library position and exposes zone-change
/// successors to later stages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveObjectsDef {
    pub input: ObjectSetDef,
    pub from: Option<super::ZoneKind>,
    pub zone: super::ZoneKind,
    pub placement: ZonePlacement,
    pub moved: Option<ObjectSetBindingIndex>,
    pub then: &'static EffectDef,
}

/// Put every card in a frozen collection onto the battlefield face down under one
/// player's control. This is a distinct arrival action because the permanent
/// never has its face-up characteristics on the battlefield.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PutObjectsOntoBattlefieldFaceDownDef {
    pub input: ObjectSetDef,
    pub controller: PlayerRefDef,
    pub characteristics: FaceDownCharacteristics,
    pub turn_up_for_mana_cost: bool,
    pub moved: Option<ObjectSetBindingIndex>,
    pub then: &'static EffectDef,
}

/// Show a collection to one player and wait for acknowledgement before
/// continuing. This is the decision-bearing form of a pure "look at" effect;
/// a top-card source is materialized by the runtime, while a bound source
/// reuses identities frozen by an earlier stage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LookAtObjectsDef {
    pub actor: PlayerRefDef,
    pub source: ObjectCollectionSourceDef,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}

/// Let one player freely divide a frozen group into two piles.  Choosing one
/// of those piles, if the card asks for that, is a separate stage and may name
/// a different actor.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PartitionGroupDef {
    pub actor: PlayerRefDef,
    pub input: ObjectSetDef,
    pub first: ObjectSetBindingIndex,
    pub second: ObjectSetBindingIndex,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}

/// Choose between two previously bound groups and bind the chosen and
/// unchosen results under semantic names.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseGroupDef {
    pub actor: PlayerRefDef,
    pub first: ObjectSetDef,
    pub second: ObjectSetDef,
    pub chosen: ObjectSetBindingIndex,
    pub unchosen: ObjectSetBindingIndex,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}

/// One optional distinct pick for each predicate, consuming a card after it
/// satisfies one predicate so it cannot satisfy another.  Atraxa is the
/// representative shape.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseOneOfEachDef {
    pub actor: PlayerRefDef,
    pub input: ObjectSetDef,
    pub predicates: &'static [ObjectPredicateDef],
    pub chosen: ObjectSetBindingIndex,
    pub remainder: ObjectSetBindingIndex,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}
