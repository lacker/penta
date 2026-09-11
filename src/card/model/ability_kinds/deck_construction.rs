// Clauses that are rules of deck construction rather than of play.
//
// Split out of `ability_kinds.rs` for the source-size budget, and kept
// distinct from everything next door because these clauses never do anything
// during a game: they are read while a deck is being assembled and are silent
// once it is shuffled. Included textually, so the imports here are that
// module's.

/// A permission a card grants the deck it is built into.
///
/// A card may print more than one over time, so this is an enumeration rather
/// than a flag; each variant names one printed sentence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeckConstructionDef {
    /// "<This card> can be your commander." A card that is not a legendary
    /// creature may still be the commander of a Commander deck (CR 903.3b).
    /// The permission is spent entirely at deck construction: the card does
    /// nothing extra once the game begins.
    MayBeCommander,
    /// "Choose a Background." This card may take a Background as a second
    /// commander (CR 702.124a), which is the only thing the sentence does:
    /// what the chosen Background then gives the deck is printed on the
    /// Background rather than here.
    ChooseABackground,
    /// "Partner." This card may lead a Commander deck beside another card
    /// that also prints Partner (CR 702.124a). The pairing is symmetric;
    /// neither physical card is the primary commander.
    Partner,
    /// "Companion — <condition>." The card may be designated a companion
    /// before the game if the starting deck meets the condition, and is then
    /// outside the game rather than in the deck (CR 702.139a). Taking it
    /// from there is a special action the card also prints.
    Companion(CompanionConditionDef),
}

/// What a companion asks of the deck it sits beside.
///
/// A dedicated vocabulary rather than a reused object predicate, because
/// these are read over a list of card definitions before a game exists: no
/// zone, no controller, nothing to match against. Each variant names one
/// printed sentence, and a fourth companion earns a fourth variant.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompanionConditionDef {
    /// "Each permanent card in your starting deck has mana value N or less."
    PermanentManaValueAtMost(u16),
    /// "Each permanent card in your starting deck has an activated ability."
    EveryPermanentHasAnActivatedAbility,
    /// "Each nonland card in your starting deck has a different name."
    NonlandNamesAreDistinct,
}
