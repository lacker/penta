// Keywords that let nonmana resources pay a spell's mana cost.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

/// Convoke (CR 702.51): creatures become one-unit payment sources while the
/// total cost of this spell is being paid. The payment planner executes the
/// tap; this clause only marks which spells use that shared procedure.
#[must_use]
pub const fn convoke() -> AbilityDef {
    keyword(
        "Convoke (Your creatures can help cast this spell. Each creature you tap while casting \
         this spell pays for {1} or one mana of that creature's color.)",
        KeywordAbility::Convoke,
    )
}

/// Delve (CR 702.66): graveyard cards become generic-only payment sources.
#[must_use]
pub const fn delve() -> AbilityDef {
    keyword(
        "Delve (Each card you exile from your graveyard while casting this spell pays for {1}.)",
        KeywordAbility::Delve,
    )
}

/// Improvise (CR 702.126): untapped artifacts become generic-only payment
/// sources after mana abilities have been activated.
#[must_use]
pub const fn improvise() -> AbilityDef {
    keyword(
        "Improvise (Your artifacts can help cast this spell. Each artifact you tap after you're \
         done activating mana abilities pays for {1}.)",
        KeywordAbility::Improvise,
    )
}
