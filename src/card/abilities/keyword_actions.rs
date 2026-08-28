// The keyword actions a card prints as one word and the engine runs as a
// whole clause: what surveil and populate mean, spelled out once each.
// Included textually into `abilities.rs`, so the imports here are that
// module's.

const SCRY_INSPECTED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(0);
const SCRY_BOTTOM: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);
const SCRY_TOP: ObjectSetBindingIndex = ObjectSetBindingIndex::new(2);
const SCRY_ORDERED_BOTTOM: ObjectSetBindingIndex = ObjectSetBindingIndex::new(3);
const SCRY_ORDERED_TOP: ObjectSetBindingIndex = ObjectSetBindingIndex::new(4);

static SCRY_PUT_TOP: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(SCRY_ORDERED_TOP),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Library,
    placement: ZonePlacement::Top,
    moved: None,
    then: &EffectDef::None,
});
static SCRY_PUT_BOTTOM: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(SCRY_ORDERED_BOTTOM),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Library,
    placement: ZonePlacement::Bottom,
    moved: None,
    then: &SCRY_PUT_TOP,
});
static SCRY_ORDER_BOTTOM: EffectDef = EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
    actor: PlayerRefDef::EffectController,
    input: ObjectSetDef::Binding(SCRY_BOTTOM),
    ordered: SCRY_ORDERED_BOTTOM,
    placement: ZonePlacement::Bottom,
    visibility: ChoiceVisibilityDef::Private,
    then: &SCRY_PUT_BOTTOM,
});
static SCRY_ORDER_TOP: EffectDef = EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
    actor: PlayerRefDef::EffectController,
    input: ObjectSetDef::Binding(SCRY_TOP),
    ordered: SCRY_ORDERED_TOP,
    placement: ZonePlacement::Top,
    visibility: ChoiceVisibilityDef::Private,
    then: &SCRY_PUT_BOTTOM,
});
static SCRY_CHOOSE_TOP: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::OrderedObjects(SCRY_ORDERED_TOP),
    unchosen: Some(SCRY_BOTTOM),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(SCRY_INSPECTED),
    exclude: None,
    minimum: 0,
    maximum: usize::MAX,
    visibility: ChoiceVisibilityDef::Private,
    then: &SCRY_ORDER_BOTTOM,
});
static SCRY_ONE_CHOOSE_BOTTOM: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::OrderedObjects(SCRY_ORDERED_BOTTOM),
    unchosen: Some(SCRY_ORDERED_TOP),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(SCRY_INSPECTED),
    exclude: None,
    minimum: 0,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Private,
    then: &SCRY_PUT_BOTTOM,
});
static FATESEAL_CHOOSE_BOTTOM: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::OrderedObjects(SCRY_ORDERED_BOTTOM),
    unchosen: Some(SCRY_TOP),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(SCRY_INSPECTED),
    exclude: None,
    minimum: 0,
    maximum: usize::MAX,
    visibility: ChoiceVisibilityDef::Private,
    then: &SCRY_ORDER_TOP,
});

/// Scry is a rules-defined procedure, not a card-authored collection program.
#[must_use]
pub const fn scry(count: ValueDef) -> EffectDef {
    let choose = match count {
        // The existing one-card interaction names the card to bottom; larger
        // scries name and order the cards staying on top in one decision.
        ValueDef::Constant(1) => &SCRY_ONE_CHOOSE_BOTTOM,
        _ => &SCRY_CHOOSE_TOP,
    };
    bind_top_cards_then(
        PlayerRefDef::EffectController,
        count,
        SCRY_INSPECTED,
        choose,
    )
}

/// Fateseal is scry performed by the effect controller on another player's
/// library, so it shares the complete two-order procedure.
#[must_use]
pub const fn fateseal(player: PlayerRefDef, count: ValueDef) -> EffectDef {
    bind_top_cards_then(player, count, SCRY_INSPECTED, &FATESEAL_CHOOSE_BOTTOM)
}

const SURVEIL_INSPECTED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(0);
const SURVEIL_GRAVEYARD: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);
const SURVEIL_TOP: ObjectSetBindingIndex = ObjectSetBindingIndex::new(2);
const SURVEIL_ORDERED_TOP: ObjectSetBindingIndex = ObjectSetBindingIndex::new(3);

static SURVEIL_PUT_TOP: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(SURVEIL_ORDERED_TOP),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Library,
    placement: ZonePlacement::Top,
    moved: None,
    then: &EffectDef::None,
});
static SURVEIL_ORDER_TOP: EffectDef = EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
    actor: PlayerRefDef::EffectController,
    input: ObjectSetDef::Binding(SURVEIL_TOP),
    ordered: SURVEIL_ORDERED_TOP,
    placement: ZonePlacement::Top,
    visibility: ChoiceVisibilityDef::Private,
    then: &SURVEIL_PUT_TOP,
});
static SURVEIL_PUT_GRAVEYARD: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(SURVEIL_GRAVEYARD),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Graveyard,
    placement: ZonePlacement::Top,
    moved: None,
    then: &SURVEIL_ORDER_TOP,
});
static SURVEIL_CHOOSE_GRAVEYARD: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(SURVEIL_GRAVEYARD),
    unchosen: Some(SURVEIL_TOP),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(SURVEIL_INSPECTED),
    exclude: None,
    minimum: 0,
    maximum: usize::MAX,
    visibility: ChoiceVisibilityDef::Private,
    then: &SURVEIL_PUT_GRAVEYARD,
});

/// Surveil is the corresponding rules-defined graveyard-or-top procedure.
#[must_use]
pub const fn surveil(count: ValueDef) -> EffectDef {
    bind_top_cards_then(
        PlayerRefDef::EffectController,
        count,
        SURVEIL_INSPECTED,
        &SURVEIL_CHOOSE_GRAVEYARD,
    )
}

const HIDEAWAY_INSPECTED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(0);
const HIDEAWAY_HIDDEN: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);
const HIDEAWAY_REST: ObjectSetBindingIndex = ObjectSetBindingIndex::new(2);
const HIDEAWAY_RANDOMIZED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(3);

static HIDEAWAY_PUT_REST: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(HIDEAWAY_RANDOMIZED),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Library,
    placement: ZonePlacement::Bottom,
    moved: None,
    then: &EffectDef::None,
});
static HIDEAWAY_RANDOMIZE_REST: EffectDef =
    EffectDef::RandomizeObjectOrder(crate::card::RandomizeObjectOrderDef {
        input: ObjectSetDef::Binding(HIDEAWAY_REST),
        randomized: HIDEAWAY_RANDOMIZED,
        then: &HIDEAWAY_PUT_REST,
    });
static HIDEAWAY_PERMIT_LOOK: EffectDef = EffectDef::PermitLookAtExiled {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(HIDEAWAY_HIDDEN)),
    player: PlayerRefDef::EffectController,
    then: &HIDEAWAY_RANDOMIZE_REST,
};
static HIDEAWAY_EXILE: EffectDef = EffectDef::ExileLinkedToSource {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(HIDEAWAY_HIDDEN)),
    face_down: true,
    until_source_leaves: false,
    then: Some(&HIDEAWAY_PERMIT_LOOK),
};
static HIDEAWAY_CHOOSE: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(HIDEAWAY_HIDDEN),
    unchosen: Some(HIDEAWAY_REST),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(HIDEAWAY_INSPECTED),
    exclude: None,
    minimum: 1,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Private,
    then: &HIDEAWAY_EXILE,
});

/// Hideaway is a linked, rules-defined procedure: inspect the requested
/// number, exile one face down linked to the source, and randomize the rest
/// onto the bottom. The land's tapped entry is a separate ability.
#[must_use]
pub const fn hideaway(count: ValueDef) -> EffectDef {
    bind_top_cards_then(
        PlayerRefDef::EffectController,
        count,
        HIDEAWAY_INSPECTED,
        &HIDEAWAY_CHOOSE,
    )
}

const MANIFEST_DREAD_INSPECTED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(0);
const MANIFEST_DREAD_PERMANENT: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);
const MANIFEST_DREAD_GRAVEYARD: ObjectSetBindingIndex = ObjectSetBindingIndex::new(2);

static MANIFEST_DREAD_PUT_GRAVEYARD: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(MANIFEST_DREAD_GRAVEYARD),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Graveyard,
    placement: ZonePlacement::Top,
    moved: None,
    then: &EffectDef::None,
});
static MANIFEST_DREAD_PUT_FACE_DOWN: EffectDef =
    EffectDef::PutObjectsOntoBattlefieldFaceDown(PutObjectsOntoBattlefieldFaceDownDef {
        input: ObjectSetDef::Binding(MANIFEST_DREAD_PERMANENT),
        controller: PlayerRefDef::EffectController,
        characteristics: crate::card::face_down::manifest(),
        turn_up_for_mana_cost: true,
        moved: None,
        then: &MANIFEST_DREAD_PUT_GRAVEYARD,
    });
static MANIFEST_DREAD_CHOOSE: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(MANIFEST_DREAD_PERMANENT),
    unchosen: Some(MANIFEST_DREAD_GRAVEYARD),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(MANIFEST_DREAD_INSPECTED),
    exclude: None,
    minimum: 1,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Private,
    then: &MANIFEST_DREAD_PUT_FACE_DOWN,
});

/// Manifest dread is a fixed keyword workflow: inspect two, divide them, put
/// one group down with manifest characteristics, and move the other normally.
#[must_use]
pub const fn manifest_dread() -> EffectDef {
    bind_top_cards_then(
        PlayerRefDef::EffectController,
        ValueDef::Constant(2),
        MANIFEST_DREAD_INSPECTED,
        &MANIFEST_DREAD_CHOOSE,
    )
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
