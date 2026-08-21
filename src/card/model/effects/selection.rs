//! Looking at the top of a library and settling where the cards go.

use super::super::FaceDownCharacteristics;
use super::{EffectDef, ObjectPredicateDef, ValueDef, ZoneKind, ZonePlacement};

// Four independent printed knobs rather than a state machine: whether the
// selection asks at all, whether it reveals, whether what it selects goes
// down face down, and whether the order it was named in is the order it is
// placed in. No two of them imply each other.
#[allow(clippy::struct_excessive_bools)]
/// A private look at the top of a library followed by one bounded card
/// selection. Selected and unselected cards can go to different zones; an
/// optional follow-up resumes only after the choice is complete. This covers
/// both selection spells such as Impulse and scry-then-draw sequencing.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TopCardSelectionDef {
    pub count: ValueDef,
    /// Restrict the cards that may be selected while still looking at and
    /// placing every card in the inspected group.
    pub object: Option<ObjectPredicateDef>,
    pub minimum: u8,
    pub maximum: u8,
    /// Take every inspected card the predicate matches, asking nothing.
    ///
    /// "Put all Goblin cards revealed this way into your hand" is mandatory
    /// and has no printed bound: a minimum and maximum could only approximate
    /// it, and any maximum small enough to be safe would let a player decline
    /// cards the card does not let them decline. The bounds above are ignored
    /// when this is set.
    pub select_all_matching: bool,
    /// Reveal selected cards before moving them, for effects that instruct
    /// the player to reveal what they took.
    pub reveal_selected: bool,
    pub selected_zone: ZoneKind,
    pub selected_placement: ZonePlacement,
    /// Copiable values for selected cards put onto the battlefield face down.
    /// `None` places them normally. This belongs to the arrival: a card put
    /// down face down was never face up there.
    pub selected_face_down: Option<FaceDownCharacteristics>,
    pub rest_zone: ZoneKind,
    pub rest_placement: ZonePlacement,
    /// The selected cards are placed in the order they were chosen rather
    /// than the order they were drawn out of the library. This is what "put
    /// them back in any order" asks for: with every inspected card selected,
    /// the sequence of the choice is the arrangement. Ordinary digs leave it
    /// off, so which of two cards went to the graveyard first stays an
    /// implementation detail rather than a decision a bot has to make.
    pub selected_order_follows_choice: bool,
    pub then: Option<&'static EffectDef>,
}
