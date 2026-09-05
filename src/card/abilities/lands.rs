// The land cycles: the printed clauses several lands in a cycle share.
//
// Grouped by subject rather than by shape -- what belongs here is anything
// whose whole reason to exist is that a run of lands prints it word for
// word. Included textually into `abilities.rs`, so the imports here are the
// parent module's.

/// The two mana abilities shared by the allied- and enemy-color painlands.
#[must_use]
pub const fn pain_land(
    colored_text: &'static str,
    colors: &'static [ManaColor],
) -> [AbilityDef; 2] {
    [
        tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            colored_text,
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors).with_damage_to_controller(1)),
        ),
    ]
}

/// The one ability every fetchland prints: tap, a life, and itself, for a
/// land out of the library. What differs between them is only which card
/// they may find, so that is the parameter.
///
/// The two cycles name a pair of basic land types, which a nonbasic dual
/// answers as well; Prismatic Vista names a basic land card, which one does
/// not. Both are ordinary object predicates, so neither needs its own
/// clause.
#[must_use]
pub const fn fetch_land_ability(text: &'static str, object: ObjectPredicateDef) -> AbilityDef {
    AbilityDef::activated(
        text,
        &FETCH_LAND_COST,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object,
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )
}

static FETCH_LAND_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::TapSource,
    AbilityCostDef::PayLife(1),
    AbilityCostDef::SacrificeSource,
];

/// The two abilities shared by the horizon-land cycle: two colours for a
/// life apiece, and, once the game has gone long enough that the land is
/// dead weight, a way to cash itself in for a card. What it never does is
/// make colourless mana for free, which is the whole of the tradeoff.
#[must_use]
pub const fn horizon_land(
    colored_text: &'static str,
    colors: &'static [ManaColor],
) -> [AbilityDef; 2] {
    [
        AbilityDef::activated_mana(
            colored_text,
            &HORIZON_MANA_COST,
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ),
        // The same on every land in the cycle, down to the printed text.
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this land: Draw a card.",
            &HORIZON_CASH_IN_COST,
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]
}

static HORIZON_MANA_COST: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)];

static HORIZON_CASH_IN_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(crate::mana_cost!("{1}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificeSource,
];

/// The shared replacement clause printed on shock lands.
#[must_use]
pub const fn shock_land_enters() -> AbilityDef {
    AbilityDef::as_enters(
        "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
        ReplacementEffectDef::PayOr {
            payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 2),
            if_paid: &[],
            if_declined: &ENTER_TAPPED,
        },
    )
}

/// A shared checkland-style entry clause backed by the general object-query
/// condition vocabulary.
#[must_use]
pub const fn check_land_enters(
    text: &'static str,
    land_types: &'static [BasicLandType],
) -> AbilityDef {
    enters_tapped_unless_you_control(text, ObjectPredicateDef::HasAnyBasicLandType(land_types))
}

/// An as-enters clause whose untapped branch depends on any controlled
/// battlefield object matching `object`.
#[must_use]
pub const fn enters_tapped_unless_you_control(
    text: &'static str,
    object: ObjectPredicateDef,
) -> AbilityDef {
    AbilityDef::as_enters(
        text,
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::Exists(ObjectQueryDef::matching(
                object,
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
            if_true: &[],
            if_false: &ENTER_TAPPED,
        },
    )
}

static TWO_OR_FEWER_OTHER_LANDS: ObjectCountConditionDef = ObjectCountConditionDef {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::LessOrEqual,
    amount: 2,
};

/// "This land enters tapped unless you control two or fewer other lands."
///
/// The bound is on the lands already there, so the land entering is excluded
/// from its own count -- which is what makes the clause read the board as it
/// was rather than as it is about to be.
#[must_use]
pub const fn fast_land_enters() -> AbilityDef {
    AbilityDef::as_enters(
        "This land enters tapped unless you control two or fewer other lands.",
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::ObjectCount(&TWO_OR_FEWER_OTHER_LANDS),
            if_true: &[],
            if_false: &ENTER_TAPPED,
        },
    )
}

static A_LAND_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::controlled_by(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerSetDef::Related(PlayerRelation::You),
);

static RETURN_CHOSEN_LAND_TO_HAND: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
    zone: ZoneKind::Hand,
    placement: ZonePlacement::Top,
};

/// The karoo bounce: "When this land enters, return a land you control to
/// its owner's hand."
///
/// Shared by the whole ten-land cycle, which differs only in the two colours
/// its mana ability adds -- so the colours stay on each card and this, which
/// names none of them, does not. The land is chosen as the trigger resolves
/// rather than targeted, so nothing about the choice can be responded to,
/// and the karoo returning itself is a legal answer that an otherwise empty
/// board is forced into.
#[must_use]
pub const fn karoo_bounce() -> AbilityDef {
    enters_trigger(
        "When this land enters, return a land you control to its owner's hand.",
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(ParentBinding),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(A_LAND_YOU_CONTROL),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &RETURN_CHOSEN_LAND_TO_HAND,
        }),
    )
}

/// The Campus cycle's shared sink: "{4}, {T}: Scry 1."
///
/// Ten lands print it word for word, differing only in the two colours their
/// mana ability adds -- so the colours stay on each card and this, which
/// names none of them, does not.
#[must_use]
pub const fn campus_scry() -> AbilityDef {
    AbilityDef::activated(
        "{4}, {T}: Scry 1.",
        &const {
            [
                AbilityCostDef::Mana(crate::mana_cost!("{4}")),
                AbilityCostDef::TapSource,
            ]
        },
        scry(ValueDef::Constant(1)),
    )
}

/// The Desert cycle's shared arrival: "When this land enters, it deals 1
/// damage to target opponent."
///
/// Ten lands print it word for word, differing only in the two colours their
/// mana ability adds -- so the colours stay on each card and this, which
/// names none of them, does not.
#[must_use]
pub const fn desert_entry_ping() -> AbilityDef {
    enters_trigger_with_targets(
        "When this land enters, it deals 1 damage to target opponent.",
        &const {
            [AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))]
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    )
}

/// The Landscape cycle's shared search: "{T}, Sacrifice this land: Search
/// your library for a basic <three types> card, put it onto the battlefield
/// tapped, then shuffle."
///
/// Ten lands print it with only the three basic types changing, so what it
/// may find is the parameter -- the same shape as `fetch_land_ability`
/// above, which differs in charging a life and not entering tapped. The text
/// comes from the caller because it names those types.
#[must_use]
pub const fn landscape_fetch(text: &'static str, object: ObjectPredicateDef) -> AbilityDef {
    AbilityDef::activated(
        text,
        &LANDSCAPE_FETCH_COST,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object,
            // A qualified library search may legally fail to find.
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    )
}

static LANDSCAPE_FETCH_COST: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource];
