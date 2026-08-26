// The keyword actions a card prints as one word and the engine runs as a
// whole clause: what surveil and populate mean, spelled out once each.
// Included textually into `abilities.rs`, so the imports here are that
// module's.

/// Surveil N (CR 701.42): look at that many cards from the top of your
/// library and put any of them into your graveyard, the rest back on top in
/// any order. Nothing is revealed and nothing has to go, so the minimum is
/// zero; `then` is whatever the card goes on to do, which has to be carried
/// here because the look is answered by a decision.
#[must_use]
pub const fn surveil(count: u8, then: Option<&'static EffectDef>) -> TopCardSelectionDef {
    TopCardSelectionDef {
        count: ValueDef::Constant(count as i32),
        object: None,
        minimum: 0,
        maximum: count,
        select_all_matching: false,
        select_one_of_each_type: false,
        reveal_inspected: false,
        reveal_selected: false,
        counted: None,
        selected_zone: ZoneKind::Graveyard,
        selected_placement: ZonePlacement::Top,
        rest_zone: ZoneKind::Library,
        rest_placement: ZonePlacement::Top,
        rest_random_order: false,
        rest_counters: None,
        selected_order_follows_choice: false,
        then,
        selected_hidden: false,
        selected_linked_to_source: false,
        selected_face_down: None,
    }
}

/// Populate's copy step, made once its choice has landed.
static POPULATE_COPY: EffectDef = EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
    object: &EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    exceptions: CopyExceptionsDef::NONE,
});

/// Populate: choose a creature token you control, then create a copy of it.
/// The choice is not a target -- nothing about it is checked again -- and a
/// player with no creature tokens simply does nothing.
#[must_use]
pub const fn populate() -> EffectDef {
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
            ObjectPredicateDef::All(&POPULATE_CANDIDATE),
            &[ZoneKind::Battlefield],
            PlayerSetDef::One(PlayerRefDef::EffectController),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &POPULATE_COPY,
    })
}

static POPULATE_CANDIDATE: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Token,
];
