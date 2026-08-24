//! Worldwake cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardChoiceSourceDef, CardRules, CardSet,
    CardSupertype, CardType, CardTypeSet, ColorSet, CreatureTypeSetDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, TopCardSelectionDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// WWK 20 — Stoneforge Mystic
static AN_EQUIPMENT_IN_HAND: [CardChoiceSourceDef; 1] = [CardChoiceSourceDef::Zone(ZoneKind::Hand)];

/// The second half of the card, and the reason the first half is worth
/// finding: a minimum of zero is the printed "you may", and with no
/// Equipment in hand the choice is never offered at all.
static MYSTIC_PUT_EQUIPMENT_DOWN: EffectDef = EffectDef::ChooseCards {
    player: EffectRecipientDef::Controller,
    sources: &AN_EQUIPMENT_IN_HAND,
    object: ObjectPredicateDef::Subtype("Equipment"),
    minimum: 0,
    maximum: 1,
    reveal: false,
    destination: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    // It arrives as itself: nothing about the Equipment changes on the way
    // down, and it is not attached to anything.
    arrival_effect: None,
};

pub(in crate::card::sets) static STONEFORGE_MYSTIC: CardRecord = CardRecord::new_with_legacy_id(
    2191,
    "Stoneforge Mystic",
    CardArt::new("19557351-b65f-4b04-b971-66abdc07000a", "Mike Bierek"),
    CardSet::Worldwake,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Kor", "Artificer"], 1, 2)
        .with_abilities(&[
            abilities::enters_trigger("When this creature enters, you may search your library for an Equipment card, reveal it, put it into your hand, then shuffle.", EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::Subtype("Equipment"),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        binding: None,
                        then: None,
                    },
                }),
            AbilityDef::activated(
                "{1}{W}, {T}: You may put an Equipment card from your hand onto the battlefield.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                    AbilityCostDef::TapSource,
                ],
                MYSTIC_PUT_EQUIPMENT_DOWN,
            ),
        ]),
);

// WWK 31 — Jace, the Mind Sculptor
static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// "You may put that card on the bottom." One card looked at, none or one
/// selected, and what is selected goes to the bottom while what is not goes
/// back where it came from.
static FATESEAL_ONE: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: None,
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    counted: None,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Bottom,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

/// Their whole library, named by owner rather than by relation: the ultimate
/// points at a player and empties that one.
static THE_TARGET_PLAYERS_LIBRARY: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Any,
    &[ZoneKind::Library],
    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
);

static THE_TARGET_PLAYERS_HAND: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Any,
    &[ZoneKind::Hand],
    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
);

static JACE_ULTIMATE: [EffectDef; 3] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::objects(crate::card::ObjectSetDef::Query(
            THE_TARGET_PLAYERS_LIBRARY,
        )),
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
    },
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::objects(crate::card::ObjectSetDef::Query(
            THE_TARGET_PLAYERS_HAND,
        )),
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
    },
    // The shuffle is what leaves them a library at all, so it is the whole
    // difference between this and drawing from nothing next upkeep.
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
];

static JACE_THE_MIND_SCULPTOR_ABILITIES: [AbilityDef; 4] = [
    AbilityDef::activated_with_targets(
        "+2: Look at the top card of target player's library. You may put that card on the \
         bottom of that player's library.",
        &[AbilityCostDef::Loyalty(2)],
        &A_PLAYER,
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            looker: EffectRecipientDef::Controller,
            selection: &FATESEAL_ONE,
        },
    ),
    AbilityDef::activated(
        "0: Draw three then put two cards from your hand on top of your library in any \
         order.",
        &[AbilityCostDef::Loyalty(0)],
        abilities::brainstorm(),
    ),
    AbilityDef::activated_with_targets(
        "−1: Return target creature to its owner's hand.",
        &[AbilityCostDef::Loyalty(-1)],
        &A_CREATURE,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
            arrival_effect: None,
            attachment: None,
        },
    ),
    AbilityDef::activated_with_targets(
        "−12: Exile all cards from target player's library, then that player shuffles their hand \
         into their library.",
        &[AbilityCostDef::Loyalty(-12)],
        &A_PLAYER,
        EffectDef::Sequence(&JACE_ULTIMATE),
    ),
];

pub(in crate::card::sets) static JACE_THE_MIND_SCULPTOR: CardRecord =
    CardRecord::new_with_legacy_id(
        2291,
        "Jace, the Mind Sculptor",
        CardArt::new("0e606072-a3aa-4300-ba90-ec92a721fa76", "Jason Chan"),
        CardSet::Worldwake,
        // Four abilities and three of them matter: the bounce buys the turn, the
        // zero rebuilds the hand, and the fateseal is what a Jace that is not
        // under pressure does forever.
        CardRules::new_planeswalker(mana_cost!("{2}{U}{U}"), &["Jace"], 3)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&JACE_THE_MIND_SCULPTOR_ABILITIES),
    );

// WWK 123 — Everflowing Chalice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EVERFLOWING_CHALICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fdcc0c3-4029-4fc3-a486-5d7f45c910bd"),
    "Everflowing Chalice",
    crate::card::CardArt::new("1fdcc0c3-4029-4fc3-a486-5d7f45c910bd", "Steve Argyle"),
    crate::card::CardSet::Worldwake,
    crate::card::CardRules::unsupported(),
);

// WWK 133 — Celestial Colonnade
static COLONNADE_FLYING: AbilityDef = abilities::flying();

static COLONNADE_VIGILANCE: AbilityDef = abilities::vigilance();

/// "It's still a land" is the type being added rather than set: everything
/// else about the animation replaces, and the land stays a land.
static COLONNADE_ANIMATION: [AppliedEffectDef; 6] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White, ManaColor::Blue])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
    AppliedEffectDef::add_ability(&COLONNADE_FLYING),
    AppliedEffectDef::add_ability(&COLONNADE_VIGILANCE),
];

static COLONNADE_COLORS: [ManaColor; 2] = [ManaColor::White, ManaColor::Blue];

pub(in crate::card::sets) static CELESTIAL_COLONNADE: CardRecord = CardRecord::new_with_legacy_id(
    2239,
    "Celestial Colonnade",
    CardArt::new("f6929259-2903-4f6f-9b06-42048fd55c6a", "Eric Deschamps"),
    CardSet::Worldwake,
    // A land that costs you a turn and then wins the game on its own, which
    // is the trade every control deck in the format is happy to make.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&COLONNADE_COLORS)),
        ),
        AbilityDef::activated(
            "{3}{W}{U}: Until end of turn, this land becomes a 4/4 white and blue Elemental \
             creature with flying and vigilance. It's still a land.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{W}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&COLONNADE_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WWK 134 — Creeping Tar Pit
static TAR_PIT_ANIMATION: [AppliedEffectDef; 5] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue, ManaColor::Black])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(2)),
    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
        ObjectPredicateDef::Any,
    )),
];

static TAR_PIT_COLORS: [ManaColor; 2] = [ManaColor::Blue, ManaColor::Black];

pub(in crate::card::sets) static CREEPING_TAR_PIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f427f0b-034c-4821-8758-e395c0042d8a"),
    "Creeping Tar Pit",
    crate::card::CardArt::new("0f427f0b-034c-4821-8758-e395c0042d8a", "Jason Felix"),
    crate::card::CardSet::Worldwake,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&TAR_PIT_COLORS)),
        ),
        AbilityDef::activated(
            "{1}{U}{B}: Until end of turn, this land becomes a 3/2 blue and black Elemental creature. It's still a land. It can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&TAR_PIT_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WWK 140 — Quicksand
static QUICKSAND_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Attacking,
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
            crate::card::KeywordAbility::Flying,
        )),
    ]),
)];

pub(in crate::card::sets) static QUICKSAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11370658-8d80-4d2f-afa5-ec6df6dee369"),
    "Quicksand",
    crate::card::CardArt::new("4e396df7-9931-43f6-b009-27cf93c4a3e5", "Matt Stewart"),
    crate::card::CardSet::Worldwake,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Target attacking creature without flying gets -1/-2 until end of turn.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &QUICKSAND_TARGET,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &STONEFORGE_MYSTIC,
    &JACE_THE_MIND_SCULPTOR,
    &EVERFLOWING_CHALICE,
    &CELESTIAL_COLONNADE,
    &CREEPING_TAR_PIT,
    &QUICKSAND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
