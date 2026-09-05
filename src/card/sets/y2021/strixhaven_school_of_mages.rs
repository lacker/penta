//! Strixhaven: School of Mages cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef,
    CounterKind, EffectDef, EffectRecipientDef, ManaColor, MoveObjectsDef, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, tokens,
};
use crate::ids::{Binding, ParentBinding};
use crate::{TargetIndex, mana_cost};

// STX 17 — Elite Spellbinder
pub(in crate::card::sets) static ELITE_SPELLBINDER: CardRecord = CardRecord::new_with_legacy_id(
    2274,
    "Elite Spellbinder",
    CardArt::new("9d3a7998-ccac-45ad-a4e9-3a2cb057f63b", "Ryan Pancoast"),
    CardSet::StrixhavenSchoolOfMages,
    // A three-mana 3/1 flier that also buys a turn: the card comes back, but
    // a turn later and two mana worse, which is often the whole game.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 3, 1)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, look at target opponent's hand. You may exile a nonland card \
                 from it. For as long as that card remains exiled, its owner may play it. A spell cast \
                 this way costs {2} more to cast.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                // "You may exile" -- a minimum of none, so a hand of nothing worth taking
                // is looked at and left alone.
                EffectDef::Sequence(&[
                    EffectDef::LookAtHand {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Object(ParentBinding),
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
                        // Not linked to the Spellbinder: killing it does not give the card back,
                        // and the tax outlives it. What the owner keeps is the card itself, one
                        // turn later and two mana worse.
                        then: &EffectDef::ExileGrantingOwnerPlay {
                            object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                            surcharge: mana_cost!("{2}"),
                        },
                    }),
                ]),
            ),
        ]),
);

// STX 43 — Frost Trickster
pub(in crate::card::sets) static FROST_TRICKSTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd79c9cc-0a8c-4d88-96e2-cb177134a18d"),
    "Frost Trickster",
    CardArt::new("fd79c9cc-0a8c-4d88-96e2-cb177134a18d", "Uriah Voth"),
    CardSet::StrixhavenSchoolOfMages,
    // A Frost Lynx with wings: the tap buys the turn the flier needs to
    // start attacking through an empty board.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Bird", "Wizard"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, tap target creature an opponent controls. That creature doesn't untap during its controller's next untap step.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                // One step rather than a duration: a creature already tapped
                // when this resolves still misses its next untap.
                EffectDef::SkipNextUntapSteps {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    count: 1,
                },
            ]),
        ),
    ]),
);

// STX 64 — Baleful Mastery
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
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            // Printed order: the draw is named before the exile, and it happens first.
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    // The discount is the whole cost of the card: two mana instead of four,
                    // and the opponent gets the card back. Which cast was used is read off the
                    // spell itself, so the rider is part of one resolution rather than a
                    // second clause.
                    condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::AlternativeCost),
                    then: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(1),
                    },
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            ]),
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
pub(in crate::card::sets) static UNWILLING_INGREDIENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30448144-639a-43c7-a408-bd6ed543c231"),
    "Unwilling Ingredient",
    CardArt::new("30448144-639a-43c7-a408-bd6ed543c231", "David Auden Nash"),
    CardSet::StrixhavenSchoolOfMages,
    // Menace makes the one-drop trade awkwardly, and once it has traded the
    // graveyard half is a card at instant speed.
    CardRules::new_creature(mana_cost!("{B}"), &["Frog"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::activated(
            "{2}{B}, Exile this card from your graveyard: You draw a card and you lose 1 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                AbilityCostDef::ExileSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )
        // No sorcery restriction printed, so unlike Mother Bear this one is
        // available with the mana held up.
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// STX 186 — Expressive Iteration
/// Three cards, three places, in the printed order: the hand card is chosen
/// first, then the one that goes underneath, and the last is exiled with
/// nothing left to decide. Exiling it is the payoff rather than the cost --
/// it is playable for the rest of the turn, which is why the spell is two
/// cards for two mana as long as the mana holds out.
const ITERATION_HAND: Binding = Binding!("iteration_hand");
const ITERATION_AFTER_HAND: Binding = Binding!("iteration_after_hand");
const ITERATION_BOTTOM: Binding = Binding!("iteration_bottom");
const ITERATION_EXILE: Binding = Binding!("iteration_exile");
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
        // Each instruction moves its own card as soon as that card is
        // named. Chaining all three moves behind the last question would
        // strand them when a short library leaves that question nothing to
        // ask: with one card left you still "put one of them into your
        // hand" and simply never reach the other two.
        abilities::bind_top_cards_then(
            PlayerRefDef::EffectController,
            ValueDef::Constant(3),
            &const {
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ITERATION_HAND),
                    unchosen: Some(ITERATION_AFTER_HAND),
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Binding(ParentBinding),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &const {
                        EffectDef::Sequence(&[
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(ITERATION_HAND),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Hand,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                            EffectDef::Choose(ChooseDef {
                                binding: ObjectChoiceBindingDef::Objects(ITERATION_BOTTOM),
                                unchosen: Some(ITERATION_EXILE),
                                chooser: PlayerRefDef::EffectController,
                                candidates: ObjectSetDef::Binding(ITERATION_AFTER_HAND),
                                exclude: None,
                                minimum: 1,
                                maximum: 1,
                                visibility: ChoiceVisibilityDef::Private,
                                then: &const {
                                    EffectDef::Sequence(&[
                                        EffectDef::MoveObjects(MoveObjectsDef {
                                            input: ObjectSetDef::Binding(ITERATION_BOTTOM),
                                            from: Some(ZoneKind::Library),
                                            zone: ZoneKind::Library,
                                            placement: ZonePlacement::Bottom,
                                            moved: None,
                                            then: &EffectDef::None,
                                        }),
                                        EffectDef::ExileGrantingControllerPlayThisTurn {
                                            object: EffectRecipientDef::objects(
                                                ObjectSetDef::Binding(ITERATION_EXILE),
                                            ),
                                        },
                                    ])
                                },
                            }),
                        ])
                    },
                })
            },
        ),
    )),
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
    TriggerEventDef::spell_cast(YOUR_INSTANT_OR_SORCERY),
    TriggerEventDef::spell_copied(YOUR_INSTANT_OR_SORCERY),
]);

// STX 219 — Quandrix Pledgemage
pub(in crate::card::sets) static QUANDRIX_PLEDGEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07633b7f-4150-458b-89c3-d05dc0e3c4bd"),
    "Quandrix Pledgemage",
    CardArt::new("07633b7f-4150-458b-89c3-d05dc0e3c4bd", "Caroline Gariba"),
    CardSet::StrixhavenSchoolOfMages,
    // Counters rather than a temporary pump, so a turn of cheap spells
    // leaves a threat behind instead of a one-turn swing.
    CardRules::new_creature(mana_cost!("{1}{G/U}{G/U}"), &["Merfolk", "Druid"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Magecraft — Whenever you cast or copy an instant or sorcery spell, put a +1/+1 \
             counter on this creature.",
            MAGECRAFT,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

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
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// STX 271 — Quandrix Campus
pub(in crate::card::sets) static QUANDRIX_CAMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f788da28-481b-41fa-a70c-b53db6b0f068"),
    "Quandrix Campus",
    CardArt::new("f788da28-481b-41fa-a70c-b53db6b0f068", "Piotr Dura"),
    CardSet::StrixhavenSchoolOfMages,
    // The green-blue Campus; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
        abilities::campus_scry(),
    ]),
);

// STX 275 — Witherbloom Campus
pub(in crate::card::sets) static WITHERBLOOM_CAMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7346fb2e-754e-47de-b33d-eb089b357ee4"),
    "Witherbloom Campus",
    CardArt::new("7346fb2e-754e-47de-b33d-eb089b357ee4", "Alayna Danner"),
    CardSet::StrixhavenSchoolOfMages,
    // The black-green Campus; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
        abilities::campus_scry(),
    ]),
);

// STX 306 — Sedgemoor Witch
pub(in crate::card::sets) static SEDGEMOOR_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("075bfaa8-3d54-4934-aaf6-72be43a87324"),
    "Sedgemoor Witch",
    crate::card::CardArt::new("075bfaa8-3d54-4934-aaf6-72be43a87324", "Igor Kieryluk"),
    crate::card::CardSet::StrixhavenSchoolOfMages,
    // Three mana for a body that is hard to block and harder to answer, and
    // that turns every cantrip into another creature.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Warlock"], 3, 2).with_abilities(&[
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
    ]),
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
