//! Strixhaven: School of Mages cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef, CardArt,
    CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, SelectionDestinationDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::ids::ObjectBindingIndex;
use crate::{TargetIndex, mana_cost};

// STX 17 — Elite Spellbinder
/// Not linked to the Spellbinder: killing it does not give the card back,
/// and the tax outlives it. What the owner keeps is the card itself, one
/// turn later and two mana worse.
static SPELLBINDER_EXILE: EffectDef = EffectDef::ExileGrantingOwnerPlay {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    surcharge: mana_cost!("{2}"),
};

/// "You may exile" -- a minimum of none, so a hand of nothing worth taking
/// is looked at and left alone.
static SPELLBINDER_TAKES_A_CARD: [EffectDef; 2] = [
    EffectDef::LookAtHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 0,
        maximum: 1,
        // The card lands in exile face up, so which one was taken stops
        // being private the moment it is taken.
        visibility: ChoiceVisibilityDef::Public,
        then: &SPELLBINDER_EXILE,
    }),
];

static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static ELITE_SPELLBINDER_ABILITIES: [AbilityDef; 2] = [
    abilities::flying(),
    abilities::enters_trigger_with_targets(
        "When this creature enters, look at target opponent's hand. You may exile a nonland card \
         from it. For as long as that card remains exiled, its owner may play it. A spell cast \
         this way costs {2} more to cast.",
        &AN_OPPONENT,
        EffectDef::Sequence(&SPELLBINDER_TAKES_A_CARD),
    ),
];

pub(in crate::card::sets) static ELITE_SPELLBINDER: CardRecord = CardRecord::new_with_legacy_id(
    2274,
    "Elite Spellbinder",
    CardArt::new("9d3a7998-ccac-45ad-a4e9-3a2cb057f63b", "Ryan Pancoast"),
    CardSet::StrixhavenSchoolOfMages,
    // A three-mana 3/1 flier that also buys a turn: the card comes back, but
    // a turn later and two mana worse, which is often the whole game.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 3, 1)
        .with_abilities(&ELITE_SPELLBINDER_ABILITIES),
);

// STX 43 — Frost Trickster
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FROST_TRICKSTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd79c9cc-0a8c-4d88-96e2-cb177134a18d"),
    "Frost Trickster",
    crate::card::CardArt::new("fd79c9cc-0a8c-4d88-96e2-cb177134a18d", "Uriah Voth"),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    crate::card::CardRules::unsupported(),
);

// STX 64 — Baleful Mastery
static MASTERY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

/// The discount is the whole cost of the card: two mana instead of four,
/// and the opponent gets the card back. Which cast was used is read off the
/// spell itself, so the rider is part of one resolution rather than a
/// second clause.
static MASTERY_WAS_DISCOUNTED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::AlternativeCost);

static MASTERY_OPPONENT_DRAWS: EffectDef = EffectDef::DrawCards {
    recipient: EffectRecipientDef::Opponent,
    amount: ValueDef::Constant(1),
};

static MASTERY_EXILE: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
    tapped: false,
};

/// Printed order: the draw is named before the exile, and it happens first.
static MASTERY_RESOLUTION: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &MASTERY_WAS_DISCOUNTED,
        then: &MASTERY_OPPONENT_DRAWS,
    },
    MASTERY_EXILE,
];

pub(in crate::card::sets) static BALEFUL_MASTERY: CardRecord = CardRecord::new_with_legacy_id(
    2201,
    "Baleful Mastery",
    CardArt::new("35f1a6ba-e46f-44fb-93f4-fb883d677b36", "Chris Cold"),
    CardSet::StrixhavenSchoolOfMages,
    // Exile at instant speed answers anything, and the choice of price is
    // the card: four mana clean, or two and a card for them.
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "If the {1}{B} cost was paid, an opponent draws a card.\nExile target creature or planeswalker.",
            &MASTERY_TARGET,
            EffectDef::Sequence(&MASTERY_RESOLUTION),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{1}{B}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may pay {1}{B} rather than pay this spell's mana cost."),
            EffectDef::None,
        ),
    ]),
);

// STX 90 — Unwilling Ingredient
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNWILLING_INGREDIENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30448144-639a-43c7-a408-bd6ed543c231"),
    "Unwilling Ingredient",
    crate::card::CardArt::new("30448144-639a-43c7-a408-bd6ed543c231", "David Auden Nash"),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    crate::card::CardRules::unsupported(),
);

// STX 186 — Expressive Iteration
/// Three cards, three places, in the printed order: the hand card is chosen
/// first, then the one that goes underneath, and the last is exiled with
/// nothing left to decide. Exiling it is the payoff rather than the cost --
/// it is playable for the rest of the turn, which is why the spell is two
/// cards for two mana as long as the mana holds out.
static ITERATION_DESTINATIONS: [SelectionDestinationDef; 3] = [
    SelectionDestinationDef::new(ZoneKind::Hand, ZonePlacement::Top),
    SelectionDestinationDef::new(ZoneKind::Library, ZonePlacement::Bottom),
    SelectionDestinationDef::new(ZoneKind::Exile, ZonePlacement::Top).playable_this_turn(),
];

pub(in crate::card::sets) static EXPRESSIVE_ITERATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31b770cc-09e7-4c0b-b2a4-462ab4f7200d"),
    "Expressive Iteration",
    crate::card::CardArt::new(
        "31b770cc-09e7-4c0b-b2a4-462ab4f7200d",
        "Anastasia Ovchinnikova",
    ),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    // Two mana and a card for two cards, one of which has to be spent this
    // turn: the deck playing it is the one with mana left over.
    CardRules::new_sorcery(mana_cost!("{U}{R}")).with_ability(AbilityDef::spell(
        "Look at the top three cards of your library. Put one of them into your hand, put one of \
         them on the bottom of your library, and exile one of them. You may play the exiled card \
         this turn.",
        EffectDef::LookAtTopAndDistribute {
            player: EffectRecipientDef::Controller,
            count: ValueDef::Constant(3),
            destinations: &ITERATION_DESTINATIONS,
        },
    )),
);

// STX 219 — Quandrix Pledgemage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUANDRIX_PLEDGEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07633b7f-4150-458b-89c3-d05dc0e3c4bd"),
    "Quandrix Pledgemage",
    crate::card::CardArt::new("07633b7f-4150-458b-89c3-d05dc0e3c4bd", "Caroline Gariba"),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    crate::card::CardRules::unsupported(),
);

/// An instant or sorcery spell of yours, which is the whole of what
/// magecraft watches: what the spell does is no part of the condition.
static YOUR_INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ]),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// "Cast or copy" is one printed clause with two ways in, so it is one
/// ability: a copy is not cast, and every other clause that watches casting
/// means casting only.
static MAGECRAFT: TriggerEventDef = TriggerEventDef::AnyOf(&[
    TriggerEventDef::SpellCast(YOUR_INSTANT_OR_SORCERY),
    TriggerEventDef::SpellCopied(YOUR_INSTANT_OR_SORCERY),
]);

static APPRENTICE_DRAIN: [EffectDef; 2] = [
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(1),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

// STX 247 — Witherbloom Apprentice
pub(in crate::card::sets) static WITHERBLOOM_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f80a11b-188b-464c-b00d-c9d1cfb8ddee"),
    "Witherbloom Apprentice",
    CardArt::new("7f80a11b-188b-464c-b00d-c9d1cfb8ddee", "Josh Hass"),
    CardSet::StrixhavenSchoolOfMages,
    // Two mana for a 2/2 that turns a deck full of cheap spells into a
    // clock, two life at a time.
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Human", "Druid"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Magecraft — Whenever you cast or copy an instant or sorcery spell, each opponent \
             loses 1 life and you gain 1 life.",
            MAGECRAFT,
            EffectDef::Sequence(&APPRENTICE_DRAIN),
        ),
    ),
);

// STX 271 — Quandrix Campus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUANDRIX_CAMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f788da28-481b-41fa-a70c-b53db6b0f068"),
    "Quandrix Campus",
    crate::card::CardArt::new("f788da28-481b-41fa-a70c-b53db6b0f068", "Piotr Dura"),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    crate::card::CardRules::unsupported(),
);

// STX 275 — Witherbloom Campus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WITHERBLOOM_CAMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7346fb2e-754e-47de-b33d-eb089b357ee4"),
    "Witherbloom Campus",
    crate::card::CardArt::new("7346fb2e-754e-47de-b33d-eb089b357ee4", "Alayna Danner"),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    crate::card::CardRules::unsupported(),
);

// STX 306 — Sedgemoor Witch
static SEDGEMOOR_WITCH_ABILITIES: [AbilityDef; 3] = [
    abilities::menace(),
    // Ward's cost is whatever the card prints, and hers is life -- which a
    // deck that already pays life for its lands is well placed to charge.
    abilities::ward_life(
        3,
        "Ward—Pay 3 life. (Whenever this creature becomes the target of a spell or ability an \
         opponent controls, counter it unless that player pays 3 life.)",
    ),
    AbilityDef::triggered(
        "Magecraft — Whenever you cast or copy an instant or sorcery spell, create a 1/1 black \
         and green Pest creature token with \"When this token dies, you gain 1 life.\"",
        MAGECRAFT,
        EffectDef::create_token(tokens::pest()).with_art(CardArt::new(
            "d0ddbe3e-4a66-494d-9304-7471232549bf",
            "Ilse Gort",
        )),
    ),
];

pub(in crate::card::sets) static SEDGEMOOR_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("075bfaa8-3d54-4934-aaf6-72be43a87324"),
    "Sedgemoor Witch",
    crate::card::CardArt::new("075bfaa8-3d54-4934-aaf6-72be43a87324", "Igor Kieryluk"),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    // Three mana for a body that is hard to block and harder to answer, and
    // that turns every cantrip into another creature.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Warlock"], 3, 2)
        .with_abilities(&SEDGEMOOR_WITCH_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ELITE_SPELLBINDER,
    &FROST_TRICKSTER,
    &BALEFUL_MASTERY,
    &UNWILLING_INGREDIENT,
    &EXPRESSIVE_ITERATION,
    &QUANDRIX_PLEDGEMAGE,
    &WITHERBLOOM_APPRENTICE,
    &QUANDRIX_CAMPUS,
    &WITHERBLOOM_CAMPUS,
    &SEDGEMOOR_WITCH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
