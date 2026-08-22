//! War of the Spark cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules,
    CardSet, CardSupertype, CardType, CardTypeSet, ComparisonDef, CounterKind, CreatureTypeSetDef,
    EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    PlayActionMatcherDef, PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, TopCardSelectionDef, TopOfLibraryCostDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

/// Your own library, empty. Written as a count rather than a dedicated
/// question so the same shape answers "no cards in it" and any other bound.
static YOUR_LIBRARY_IS_EMPTY: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Any,
        &[ZoneKind::Library],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::LessOrEqual,
    amount: 0,
};

/// In a two-player game winning is the opponent losing, which is the shape
/// the engine has. The recorded reason is therefore an effect rather than a
/// dedicated win, which nothing in the supported pool reads.
static YOU_WIN: EffectDef = EffectDef::LoseTheGame {
    player: EffectRecipientDef::players(crate::card::PlayerSetDef::Related(
        PlayerRelation::Opponent,
    )),
};

static JACE_MILLS_AND_DRAWS: [EffectDef; 2] = [
    EffectDef::Mill {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
        binding: None,
        then: None,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

static JACE_DRAWS_SEVEN: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(7),
    },
    EffectDef::IfCondition {
        condition: &YOUR_LIBRARY_IS_EMPTY,
        then: &YOU_WIN,
    },
];

static JACE_ABILITIES: [AbilityDef; 3] = [
    // The static is the card: without it the seven-card draw and the mill
    // are just a slow Jace, and with it an empty library is a win rather
    // than the usual loss.
    AbilityDef::static_ability(
        "If you would draw a card while your library has no cards in it, you win the game instead.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(crate::card::PlayerSetDef::Related(
                PlayerRelation::You,
            )),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::WinsInsteadOfDrawingFromEmptyLibrary),
        },
    ),
    AbilityDef::activated_with_targets(
        "+1: Target player mills two cards. Draw a card.",
        &[AbilityCostDef::Loyalty(1)],
        &JACE_MILL_TARGET,
        EffectDef::Sequence(&JACE_MILLS_AND_DRAWS),
    ),
    AbilityDef::activated(
        "−8: Draw seven cards. Then if your library has no cards in it, you win the game.",
        &[AbilityCostDef::Loyalty(-8)],
        EffectDef::Sequence(&JACE_DRAWS_SEVEN),
    ),
];

static JACE_MILL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// WAR 54 — Jace, Wielder of Mysteries
pub(in crate::card::sets) static JACE_WIELDER_OF_MYSTERIES: CardRecord =
    CardRecord::new_with_legacy_id(
        2160,
        "Jace, Wielder of Mysteries",
        CardArt::new("6adb7d73-4482-4930-8497-cffd169b57e2", "Anna Steinbauer"),
        CardSet::WarOfTheSpark,
        CardRules::new_planeswalker(mana_cost!("{1}{U}{U}{U}"), &["Jace"], 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&JACE_ABILITIES),
    );

/// "You tap a Forest for mana" is the tap transition carrying its purpose,
/// so an ordinary tap does not fire it and a mana tap does.
static NISSA_FOREST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static NISSA_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
    1,
)];

static NISSA_VIGILANCE: AbilityDef = abilities::vigilance();

static NISSA_HASTE: AbilityDef = abilities::haste();

/// "Still a land" is why the types are added rather than set: the animated
/// permanent keeps tapping for mana, and Nissa's own static doubles it.
static NISSA_ANIMATION: [AppliedEffectDef; 5] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(0)),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
    AppliedEffectDef::add_ability(&NISSA_VIGILANCE),
    AppliedEffectDef::add_ability(&NISSA_HASTE),
];

/// The counters go on first, while the land is still a noncreature: the
/// animation then sets a base of 0/0 and the three counters make it a 3/3.
static NISSA_AWAKENS_A_LAND: [EffectDef; 3] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(3),
    },
    EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::Composite(&NISSA_ANIMATION),
        duration: ResolvedEffectDurationDef::Permanent,
    },
];

/// "Any number" is every one there is, so the bound is how many the library
/// actually holds rather than a printed number.
static FORESTS_IN_YOUR_LIBRARY: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    &[ZoneKind::Library],
    PlayerRelation::You,
);

static NISSA_LANDS_ARE_INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

static NISSA_WHO_SHAKES_THE_WORLD_EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Lands you control have indestructible.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Land),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        effect: AppliedEffectDef::add_ability(&NISSA_LANDS_ARE_INDESTRUCTIBLE),
    },
)];

static NISSA_ULTIMATE: [EffectDef; 2] = [
    EffectDef::create_emblem(
        "Nissa, Who Shakes the World emblem",
        &NISSA_WHO_SHAKES_THE_WORLD_EMBLEM_ABILITIES,
    ),
    EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
        minimum: 0,
        maximum: ValueDef::CountMatchingObjects(&FORESTS_IN_YOUR_LIBRARY),
        reveal: false,
        destination: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: true,
        binding: None,
        then: None,
    },
];

static NISSA_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered_mana(
        "Whenever you tap a Forest for mana, add an additional {G}.",
        TriggerEventDef::tapped_for_mana(NISSA_FOREST),
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
    ),
    AbilityDef::activated_with_targets(
        "+1: Put three +1/+1 counters on up to one target noncreature land you control. Untap it. It becomes a 0/0 Elemental creature with vigilance and haste that's still a land.",
        &[AbilityCostDef::Loyalty(1)],
        &NISSA_TARGET,
        EffectDef::Sequence(&NISSA_AWAKENS_A_LAND),
    ),
    AbilityDef::activated(
        "−8: You get an emblem with \"Lands you control have indestructible.\" Search your library for any number of Forest put them onto the battlefield tapped, then shuffle.",
        &[AbilityCostDef::Loyalty(-8)],
        EffectDef::Sequence(&NISSA_ULTIMATE),
    ),
];

/// Anything at all, which is what "lands and spells" comes to once the top
/// of the library is the only place being named.
static CITADEL_PERMISSION: PlayRestrictionDef =
    PlayRestrictionDef::new(PlayActionMatcherDef::Any, ObjectPredicateDef::Any);

static CITADEL_SACRIFICE: [AbilityCostDef; 2] = [
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificePermanents {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        controller: PlayerRelation::You,
        count: 10,
    },
];

static BOLASS_CITADEL_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "You may look at the top card of your library any time.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
        },
    ),
    AbilityDef::static_ability(
        "You may play lands and cast spells from the top of your library. If you cast a spell \
         this way, pay life equal to its mana value rather than pay its mana cost.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                restriction: CITADEL_PERMISSION,
                cost: TopOfLibraryCostDef::LifeEqualToManaValue,
            }),
        },
    ),
    AbilityDef::activated(
        "{T}, Sacrifice ten nonland permanents: Each opponent loses 10 life.",
        &CITADEL_SACRIFICE,
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::Constant(10),
        },
    ),
];

// WAR 79 — Bolas's Citadel
pub(in crate::card::sets) static BOLASS_CITADEL: CardRecord = CardRecord::new_with_legacy_id(
    2253,
    "Bolas's Citadel",
    CardArt::new("d2124603-d20e-40eb-97f0-a66323397ac2", "Jonas De Ro"),
    CardSet::WarOfTheSpark,
    // Six mana to turn a library into a hand and a life total into mana.
    // The ten-permanent ability is the finish, not the plan.
    CardRules::new_artifact(mana_cost!("{3}{B}{B}{B}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&BOLASS_CITADEL_ABILITIES),
);

/// "Mana value less than or equal to this creature's power" is read live off
/// the Arcanist, so a counter or a pump changes what it can reach.
static ARCANIST_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            ObjectPredicateDef::ManaValueAtMostValue(ValueDef::SourcePower),
        ]),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

/// What the card is lent while the offer stands. The kind says both halves
/// of the printed clause at once: the cast costs nothing, and the card is
/// exiled rather than buried afterwards.
static ARCANIST_FREE_CAST: AbilityDef = AbilityDef::alternative_cast(
    mana_cost!("{0}"),
    AlternativeCastKindDef::WithoutPayingManaCost,
    Some("Cast without paying its mana cost, then exile it."),
    EffectDef::None,
);

static DREADHORDE_ARCANIST_ABILITIES: [AbilityDef; 2] = [
    abilities::trample(),
    AbilityDef::triggered_with_targets(
        "Whenever this creature attacks, you may cast target instant or sorcery card with mana \
         value less than or equal to this creature's power from your graveyard without paying \
         its mana cost. If that spell would be put into your graveyard, exile it instead.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        &ARCANIST_TARGET,
        EffectDef::MayCastTargetWithoutPaying {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ability: &ARCANIST_FREE_CAST,
        },
    ),
];

// WAR 125 — Dreadhorde Arcanist
pub(in crate::card::sets) static DREADHORDE_ARCANIST: CardRecord = CardRecord::new_with_legacy_id(
    2279,
    "Dreadhorde Arcanist",
    CardArt::new("fd97b3cf-924e-4f77-bb82-0bf19592389f", "G-host Lee"),
    CardSet::WarOfTheSpark,
    // A 1/3 that only buys back one-mana spells until something makes it
    // bigger, which in the cube is most of what the deck is doing anyway.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Zombie", "Wizard"], 1, 3)
        .with_abilities(&DREADHORDE_ARCANIST_ABILITIES),
);

// WAR 169 — Nissa, Who Shakes the World
pub(in crate::card::sets) static NISSA_WHO_SHAKES_THE_WORLD: CardRecord =
    CardRecord::new_with_legacy_id(
        2172,
        "Nissa, Who Shakes the World",
        CardArt::new("41e108a5-4e2f-42cf-9ea1-87bf3c0a2b7f", "Chris Rallis"),
        CardSet::WarOfTheSpark,
        // Doubling every Forest is the card: five mana becomes eight the turn
        // after, and the +1 turns the spare land into a 3/3 that attacks at once.
        CardRules::new_planeswalker(mana_cost!("{3}{G}{G}"), &["Nissa"], 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&NISSA_ABILITIES),
    )
    .with_identity_anchor(PrintingAnchor::scryfall(
        "f857bbe4-5619-4733-a0c7-69700f2ef4f3",
    ));

/// Two prohibitions in one printed sentence, which is why they are a
/// sequence rather than one effect: other cards state only one of them.
static TAMIYO_PROTECTIONS: [EffectDef; 2] = [
    EffectDef::CannotBeForcedToDiscard,
    EffectDef::CannotBeForcedToSacrifice,
];

/// The name is chosen before the four cards are seen, so the reveal cannot
/// be used to pick a name that is already there.
static TAMIYO_SORT_THE_FOUR: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: Some(ObjectPredicateDef::HasChosenName),
    minimum: 0,
    maximum: 4,
    select_all_matching: true,
    reveal_selected: true,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Graveyard,
    rest_placement: ZonePlacement::Top,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: None,
};

static TAMIYO_REVEAL: EffectDef = EffectDef::LookAtTopAndSelect {
    player: EffectRecipientDef::Controller,
    looker: EffectRecipientDef::Controller,
    selection: &TAMIYO_SORT_THE_FOUR,
};

/// The binding the name-choice makes is unused here: what matches is decided
/// among the four revealed cards rather than across a whole zone, so the
/// selection reads the name itself.
static TAMIYO_NAME_THEN_REVEAL: EffectDef = EffectDef::ChooseCardName {
    chooser: PlayerRefDef::EffectController,
    nonland_only: true,
    matched_in: PlayerRefDef::EffectController,
    zone: ZoneKind::Library,
    binding: ObjectSetBindingIndex::PRIMARY,
    then: &TAMIYO_REVEAL,
};

static TAMIYO_RETURN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

static TAMIYO_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "Spells and abilities your opponents control can't cause you to discard cards or sacrifice permanents.",
        EffectDef::Sequence(&TAMIYO_PROTECTIONS),
    ),
    AbilityDef::activated(
        "+1: Choose a nonland card name, then reveal the top four cards of your library. Put all cards with the chosen name from among them into your hand and the rest into your graveyard.",
        &[AbilityCostDef::Loyalty(1)],
        TAMIYO_NAME_THEN_REVEAL,
    ),
    AbilityDef::activated_with_targets(
        "\u{2212}3: Return target card from your graveyard to your hand.",
        &[AbilityCostDef::Loyalty(-3)],
        &TAMIYO_RETURN_TARGET,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
    ),
];

// WAR 220 — Tamiyo, Collector of Tales
pub(in crate::card::sets) static TAMIYO_COLLECTOR_OF_TALES: CardRecord =
    CardRecord::new_with_legacy_id(
        2186,
        "Tamiyo, Collector of Tales",
        CardArt::new("786d89de-da0c-47af-80ae-2734dc0514fc", "Chase Stone"),
        CardSet::WarOfTheSpark,
        // The static is what the card is played for: it turns off every
        // discard-based and sacrifice-based answer an opponent has, and the
        // loyalty abilities are what it does while doing that.
        CardRules::new_planeswalker(mana_cost!("{2}{G}{U}"), &["Tamiyo"], 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&TAMIYO_ABILITIES),
    )
    .with_identity_anchor(PrintingAnchor::scryfall(
        "76776b24-a2e1-4590-88e7-8a421baf2fc4",
    ));

static A_NONCREATURE_SPELL_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::NoncreatureSpell,
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// "Another target artifact or creature you control": the second slot is a
/// separate target, so the two cannot be the same permanent.
static SAHEELI_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ])),
    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ]),
        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ]))
    .another(),
];

static SAHEELI_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered(
        "Whenever you cast a noncreature spell, create a 1/1 colorless Servo artifact creature \
         token.",
        TriggerEventDef::SpellCast(A_NONCREATURE_SPELL_YOU_CAST),
        EffectDef::create_artifact_creature_token(&["Servo"], &[], 1, 1).with_art(CardArt::new(
            "761507d5-d36a-4123-a074-95d7f6ffb4c5",
            "Victor Adame Minguez",
        )),
    ),
    AbilityDef::activated_with_targets(
        "−2: Target artifact you control becomes a copy of another target artifact or creature \
         you control until end of turn, except it's an artifact in addition to its other types.",
        &[AbilityCostDef::Loyalty(-2)],
        &SAHEELI_TARGETS,
        EffectDef::BecomeCopyOf {
            object: EffectRecipientDef::Target(TargetIndex(1)),
            copier: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            retain_source_ability: false,
            added_types: CardTypeSet::single(CardType::Artifact),
            duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
        },
    ),
];

// WAR 234 — Saheeli, Sublime Artificer
pub(in crate::card::sets) static SAHEELI_SUBLIME_ARTIFICER: CardRecord =
    CardRecord::new_with_legacy_id(
        2247,
        "Saheeli, Sublime Artificer",
        CardArt::new("5a10b543-d5d4-42a8-9ee8-dada59a2ad7e", "Wesley Burt"),
        CardSet::WarOfTheSpark,
        // A planeswalker that never has to be activated: three mana, five
        // loyalty, and a body for every spell the deck was casting anyway.
        CardRules::new_planeswalker(mana_cost!("{1}{U/R}{U/R}"), &["Saheeli"], 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&SAHEELI_ABILITIES),
    );

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &JACE_WIELDER_OF_MYSTERIES,
    &BOLASS_CITADEL,
    &DREADHORDE_ARCANIST,
    &NISSA_WHO_SHAKES_THE_WORLD,
    &TAMIYO_COLLECTOR_OF_TALES,
    &SAHEELI_SUBLIME_ARTIFICER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
