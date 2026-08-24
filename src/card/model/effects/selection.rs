//! Looking at the top of a library and settling where the cards go.

use super::super::FaceDownCharacteristics;
use super::{CounterKind, EffectDef, ObjectPredicateDef, ValueDef, ZoneKind, ZonePlacement};

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
    /// Restrict which selected cards [`ValueDef::MatchedCount`] and
    /// [`ValueDef::MatchedManaValue`] describe in `then`. `None` counts every
    /// selected card. This is independent from `object`: a card may let the
    /// player reveal any inspected card, then care only when what they chose
    /// has a particular quality.
    pub counted: Option<ObjectPredicateDef>,
    pub selected_zone: ZoneKind,
    pub selected_placement: ZonePlacement,
    /// Whether the selected cards lie face down where they land, which only
    /// exile supports: their controller may look at them and nobody else
    /// sees what they are.
    pub selected_hidden: bool,
    /// Whether the selected cards are linked to the ability's own source.
    /// Hideaway needs it: the land that hid the card is the only thing that
    /// can name it afterwards, and nothing in exile says which card that is.
    pub selected_linked_to_source: bool,
    /// Copiable values for selected cards put onto the battlefield face down.
    /// `None` places them normally. This belongs to the arrival: a card put
    /// down face down was never face up there.
    pub selected_face_down: Option<FaceDownCharacteristics>,
    pub rest_zone: ZoneKind,
    pub rest_placement: ZonePlacement,
    /// "Put the rest on the bottom of your library in a random order." The
    /// looker has seen those cards, so the order they go back in is the
    /// difference between a look and a stack: without this they would return
    /// in the order they were drawn out.
    pub rest_random_order: bool,
    /// Counters the cards that were not selected arrive carrying. "Exile the
    /// other with a silver counter on it" belongs to the move for the same
    /// reason [`super::EffectDef::MoveToZone`]'s counters do: what lands is
    /// a new object, and a following clause would have nothing to name.
    pub rest_counters: Option<(CounterKind, u16)>,
    /// The selected cards are placed in the order they were chosen rather
    /// than the order they were drawn out of the library. This is what "put
    /// them back in any order" asks for: with every inspected card selected,
    /// the sequence of the choice is the arrangement. Ordinary digs leave it
    /// off, so which of two cards went to the graveyard first stays an
    /// implementation detail rather than a decision a bot has to make.
    pub selected_order_follows_choice: bool,
    pub then: Option<&'static EffectDef>,
}
