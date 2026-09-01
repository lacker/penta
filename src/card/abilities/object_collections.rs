// Collection-facing ability helpers. Included into `abilities.rs`, so this
// file shares that module's imports and public namespace.

/// Privately look at the top `count` cards of the named player's library.
/// The runtime materializes the exact cards for the acknowledgement decision;
/// callers that need to act on those identities should use [`BindObjectsDef`]
/// and an explicit collection pipeline instead.
#[must_use]
pub const fn look_at_top_cards(player: PlayerRefDef, count: ValueDef) -> EffectDef {
    EffectDef::LookAtObjects(LookAtObjectsDef {
        actor: PlayerRefDef::EffectController,
        source: ObjectCollectionSourceDef::TopCards { player, count },
        visibility: ChoiceVisibilityDef::Private,
        then: &EffectDef::None,
    })
}

const TOP_CARD_CHOSEN: Binding = Binding!("top_card_chosen");
const TOP_CARD_REMAINDER: Binding = Binding!("top_card_remainder");

static PUT_ORDERED_REMAINDER_ON_BOTTOM: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(ParentBinding),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Library,
    placement: ZonePlacement::Bottom,
    moved: None,
    then: &EffectDef::None,
});
static RANDOMIZE_REMAINDER_FOR_BOTTOM: EffectDef =
    EffectDef::RandomizeObjectOrder(crate::card::RandomizeObjectOrderDef {
        input: ObjectSetDef::Binding(TOP_CARD_REMAINDER),
        randomized: ParentBinding,
        then: &PUT_ORDERED_REMAINDER_ON_BOTTOM,
    });
static PRIVATELY_ORDER_REMAINDER_FOR_BOTTOM: EffectDef =
    EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
        actor: PlayerRefDef::EffectController,
        input: ObjectSetDef::Binding(TOP_CARD_REMAINDER),
        ordered: ParentBinding,
        placement: ZonePlacement::Bottom,
        visibility: ChoiceVisibilityDef::Private,
        then: &PUT_ORDERED_REMAINDER_ON_BOTTOM,
    });
static PUBLICLY_ORDER_REMAINDER_FOR_BOTTOM: EffectDef =
    EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
        actor: PlayerRefDef::EffectController,
        input: ObjectSetDef::Binding(TOP_CARD_REMAINDER),
        ordered: ParentBinding,
        placement: ZonePlacement::Bottom,
        visibility: ChoiceVisibilityDef::Public,
        then: &PUT_ORDERED_REMAINDER_ON_BOTTOM,
    });
static PUT_CHOSEN_IN_HAND_THEN_PRIVATE_BOTTOM: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveObjects(MoveObjectsDef {
        input: ObjectSetDef::Binding(TOP_CARD_CHOSEN),
        from: Some(ZoneKind::Library),
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        moved: None,
        then: &EffectDef::None,
    }),
    PRIVATELY_ORDER_REMAINDER_FOR_BOTTOM,
]);
static PUT_CHOSEN_IN_HAND_THEN_PUBLIC_BOTTOM: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveObjects(MoveObjectsDef {
        input: ObjectSetDef::Binding(TOP_CARD_CHOSEN),
        from: Some(ZoneKind::Library),
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        moved: None,
        then: &EffectDef::None,
    }),
    PUBLICLY_ORDER_REMAINDER_FOR_BOTTOM,
]);
static PUT_CHOSEN_IN_HAND_THEN_RANDOM_BOTTOM: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveObjects(MoveObjectsDef {
        input: ObjectSetDef::Binding(TOP_CARD_CHOSEN),
        from: Some(ZoneKind::Library),
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        moved: None,
        then: &EffectDef::None,
    }),
    RANDOMIZE_REMAINDER_FOR_BOTTOM,
]);
static REVEAL_CHOSEN_THEN_PUT_IN_HAND_AND_PRIVATE_BOTTOM: EffectDef =
    EffectDef::Sequence(&[
        EffectDef::RevealObjects(RevealObjectsDef {
            input: ObjectSetDef::Binding(TOP_CARD_CHOSEN),
            then: &EffectDef::None,
        }),
        PUT_CHOSEN_IN_HAND_THEN_PRIVATE_BOTTOM,
    ]);
static REVEAL_CHOSEN_THEN_PUT_IN_HAND_AND_RANDOM_BOTTOM: EffectDef =
    EffectDef::Sequence(&[
        EffectDef::RevealObjects(RevealObjectsDef {
            input: ObjectSetDef::Binding(TOP_CARD_CHOSEN),
            then: &EffectDef::None,
        }),
        PUT_CHOSEN_IN_HAND_THEN_RANDOM_BOTTOM,
    ]);
static PUT_REMAINDER_IN_GRAVEYARD: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(TOP_CARD_REMAINDER),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Graveyard,
    placement: ZonePlacement::Top,
    moved: None,
    then: &EffectDef::None,
});
static PUT_CHOSEN_IN_HAND_THEN_REST_IN_GRAVEYARD: EffectDef =
    EffectDef::Sequence(&[
        EffectDef::MoveObjects(MoveObjectsDef {
            input: ObjectSetDef::Binding(TOP_CARD_CHOSEN),
            from: Some(ZoneKind::Library),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            moved: None,
            then: &EffectDef::None,
        }),
        PUT_REMAINDER_IN_GRAVEYARD,
    ]);
static PUT_REORDERED_CARDS_ON_TOP: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(ParentBinding),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Library,
    placement: ZonePlacement::Top,
    moved: None,
    then: &EffectDef::None,
});
static REORDER_TOP_CARDS: EffectDef = EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
    actor: PlayerRefDef::EffectController,
    input: ObjectSetDef::Binding(ParentBinding),
    ordered: ParentBinding,
    placement: ZonePlacement::Top,
    visibility: ChoiceVisibilityDef::Private,
    then: &PUT_REORDERED_CARDS_ON_TOP,
});

const fn choose_from_top_cards(
    count: ValueDef,
    inspection: CollectionInspectionDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
    then: &'static EffectDef,
) -> EffectDef {
    EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
        source: ObjectCollectionSourceDef::TopCards {
            player: PlayerRefDef::EffectController,
            count,
        },
        actor: PlayerRefDef::EffectController,
        inspection,
        object,
        minimum,
        maximum,
        chosen: TOP_CARD_CHOSEN,
        remainder: TOP_CARD_REMAINDER,
        then,
    })
}

/// Look at the top cards, choose a bounded number matching `object` for the
/// hand, and put everything else on the bottom in an order you choose.
#[must_use]
pub const fn look_at_top_cards_choose_to_hand_rest_bottom(
    count: ValueDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
) -> EffectDef {
    choose_from_top_cards(
        count,
        CollectionInspectionDef::Look,
        object,
        minimum,
        maximum,
        &PUT_CHOSEN_IN_HAND_THEN_PRIVATE_BOTTOM,
    )
}

/// The Augur/Lead-the-Stampede form of the same dig: the chosen cards are
/// revealed before they enter the hand.
#[must_use]
pub const fn look_at_top_cards_reveal_choice_to_hand_rest_bottom(
    count: ValueDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
) -> EffectDef {
    choose_from_top_cards(
        count,
        CollectionInspectionDef::Look,
        object,
        minimum,
        maximum,
        &REVEAL_CHOSEN_THEN_PUT_IN_HAND_AND_PRIVATE_BOTTOM,
    )
}

/// Look at the top cards, reveal a bounded matching choice into the hand,
/// and randomize everything else onto the bottom.
#[must_use]
pub const fn look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
    count: ValueDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
) -> EffectDef {
    choose_from_top_cards(
        count,
        CollectionInspectionDef::Look,
        object,
        minimum,
        maximum,
        &REVEAL_CHOSEN_THEN_PUT_IN_HAND_AND_RANDOM_BOTTOM,
    )
}

/// Look at the top cards, put a bounded matching choice into the hand, and
/// randomize everything else onto the bottom without revealing the choice.
#[must_use]
pub const fn look_at_top_cards_choose_to_hand_rest_random_bottom(
    count: ValueDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
) -> EffectDef {
    choose_from_top_cards(
        count,
        CollectionInspectionDef::Look,
        object,
        minimum,
        maximum,
        &PUT_CHOSEN_IN_HAND_THEN_RANDOM_BOTTOM,
    )
}

/// Reveal the top cards, put a bounded matching choice into the hand, and
/// randomize everything else onto the bottom.
#[must_use]
pub const fn reveal_top_cards_choose_to_hand_rest_random_bottom(
    count: ValueDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
) -> EffectDef {
    choose_from_top_cards(
        count,
        CollectionInspectionDef::Reveal,
        object,
        minimum,
        maximum,
        &PUT_CHOSEN_IN_HAND_THEN_RANDOM_BOTTOM,
    )
}

/// Look at the top cards, choose matching cards for the hand, and put every
/// other inspected card into the graveyard.
#[must_use]
pub const fn look_at_top_cards_choose_to_hand_rest_graveyard(
    count: ValueDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
) -> EffectDef {
    choose_from_top_cards(
        count,
        CollectionInspectionDef::Look,
        object,
        minimum,
        maximum,
        &PUT_CHOSEN_IN_HAND_THEN_REST_IN_GRAVEYARD,
    )
}

/// Reveal the top cards, choose matching cards for the hand, and put every
/// other inspected card into the graveyard.
#[must_use]
pub const fn reveal_top_cards_choose_to_hand_rest_graveyard(
    count: ValueDef,
    object: ObjectPredicateDef,
    minimum: usize,
    maximum: usize,
) -> EffectDef {
    choose_from_top_cards(
        count,
        CollectionInspectionDef::Reveal,
        object,
        minimum,
        maximum,
        &PUT_CHOSEN_IN_HAND_THEN_REST_IN_GRAVEYARD,
    )
}

/// Reveal the top cards and mandatorily put every matching card into the
/// hand; everything else goes to the graveyard.
#[must_use]
pub const fn reveal_top_cards_put_matching_in_hand_rest_graveyard(
    count: ValueDef,
    object: ObjectPredicateDef,
) -> EffectDef {
    EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
        source: ObjectCollectionSourceDef::TopCards {
            player: PlayerRefDef::EffectController,
            count,
        },
        object,
        matching: TOP_CARD_CHOSEN,
        remainder: TOP_CARD_REMAINDER,
        then: &PUT_CHOSEN_IN_HAND_THEN_REST_IN_GRAVEYARD,
    })
}

/// Reveal the top cards and mandatorily put every matching card into the
/// hand; everything else goes to the bottom in an order you choose.
#[must_use]
pub const fn reveal_top_cards_put_matching_in_hand_rest_bottom(
    count: ValueDef,
    object: ObjectPredicateDef,
) -> EffectDef {
    EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
        source: ObjectCollectionSourceDef::TopCards {
            player: PlayerRefDef::EffectController,
            count,
        },
        object,
        matching: TOP_CARD_CHOSEN,
        remainder: TOP_CARD_REMAINDER,
        then: &PUT_CHOSEN_IN_HAND_THEN_PUBLIC_BOTTOM,
    })
}

/// Look at the top cards of a library and put those same cards back on top
/// in an order chosen by the effect's controller.
#[must_use]
pub const fn look_at_top_cards_and_reorder(player: PlayerRefDef, count: ValueDef) -> EffectDef {
    bind_top_cards_then(player, count, &REORDER_TOP_CARDS)
}

/// Freeze the top `count` cards of a library for a multi-stage workflow.
/// This is the stateful counterpart to [`look_at_top_cards`]: use it when a
/// later stage must choose, reveal, move, or order the exact same cards.
#[must_use]
pub const fn bind_top_cards_then(
    player: PlayerRefDef,
    count: ValueDef,
    then: &'static EffectDef,
) -> EffectDef {
    bind_objects_then(ObjectCollectionSourceDef::TopCards { player, count }, then)
}

/// Freeze the top of a library through and including the first card matching
/// `object`. With no match, the binding contains the whole library. The
/// continuation decides whether those cards are looked at, revealed, moved,
/// or classified; this producer performs none of those actions itself.
#[must_use]
pub const fn bind_top_cards_through_first_matching_then(
    player: PlayerRefDef,
    object: ObjectPredicateDef,
    then: &'static EffectDef,
) -> EffectDef {
    bind_objects_then(
        ObjectCollectionSourceDef::TopCardsThroughFirstMatching { player, object },
        then,
    )
}

/// Freeze any collection source under a binding and continue.
#[must_use]
pub const fn bind_objects_then(
    source: ObjectCollectionSourceDef,
    then: &'static EffectDef,
) -> EffectDef {
    EffectDef::BindObjects(BindObjectsDef {
        source,
        binding: ParentBinding,
        then,
    })
}
