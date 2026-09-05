//! Ravnica: Clue Edition cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, CounterKind, DiscardSelectionDef, EffectDef, EffectRecipientDef,
    ExilePlayDurationDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::{TargetIndex, mana_cost};

// CLU 4 — Headliner Scarlett
pub(in crate::card::sets) static HEADLINER_SCARLETT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be77b98a-dd79-477c-8ab2-7ebf5637a89e"),
    "Headliner Scarlett",
    CardArt::new("be77b98a-dd79-477c-8ab2-7ebf5637a89e", "Heonhwa"),
    CardSet::RavnicaClueEdition,
    // Four mana that attacks the turn it lands into a board that cannot
    // block, and then draws an extra card every turn it survives.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Warlock"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            abilities::enters_trigger_with_targets(
                "When Headliner Scarlett enters, creatures target player controls can't block this turn.",
                // "Creatures target player controls." Read as the trigger resolves, so a
                // creature that arrives afterwards blocks perfectly well -- which is what
                // makes this a tempo card rather than an evasion one.
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::controlled_by(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                        ),
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            // Face down, so the card is hers to see and nobody else's to plan
            // around, and at its own cost: what the upkeep buys is a card a turn,
            // not a free one.
            AbilityDef::triggered(
                "At the beginning of your upkeep, exile the top card of your library face down. You may \
                 look at and play that card this turn.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::ExileTopOfLibraryToPlay {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    free: false,
                    face_down: true,
                    duration: ExilePlayDurationDef::ThisTurn,
                    spend_any_color: false,
                    play_condition: None,
                    cast_only: false,
                },
            ),
        ]),
);

// CLU 26 — Carnage Interpreter
pub(in crate::card::sets) static CARNAGE_INTERPRETER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6fb576e-a4a4-496b-b553-3f81cc651210"),
    "Carnage Interpreter",
    CardArt::new("f6fb576e-a4a4-496b-b553-3f81cc651210", "Justine Cruz"),
    CardSet::RavnicaClueEdition,
    // Three mana for a 5/5 with menace and four cards' worth of Clues, paid
    // for with whatever was left in hand -- which is nothing, on the turn
    // the deck wants to cast it.
    CardRules::new_creature(mana_cost!("{1}{B/R}{B/R}"), &["Devil", "Detective"], 3, 3)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, discard your hand, then investigate four times. (To \
                 investigate, create a Clue token. It's an artifact with \"{2}, Sacrifice this token: \
                 Draw a card.\")",
                // Four Clues in one instruction. Investigating four times is four events
                // where the card is played, and nothing in the catalog watches for one, so
                // the four tokens arrive together.
                EffectDef::Sequence(&[
                    // "Discard your hand": as many cards as there are, so the count is the
                    // hand rather than a number the card names.
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(i32::MAX),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    EffectDef::create_token(tokens::clue()).with_count(ValueDef::Constant(4)),
                ]),
            ),
            AbilityDef::static_ability(
                "As long as you have one or fewer cards in hand, this creature gets +2/+2 and has menace.",
                EffectDef::IfCondition {
                    // "One or fewer cards in hand", read live off the hand rather than off what
                    // the discard left: a card drawn afterwards turns the bonus off again.
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Hand],
                            PlayerSetDef::One(PlayerRefDef::EffectController),
                        ),
                        comparison: ComparisonDef::LessOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                            AppliedEffectDef::add_ability(&abilities::menace()),
                        ]),
                    },
                },
            ),
        ]),
);

// CLU 50 — Unruly Krasis
pub(in crate::card::sets) static UNRULY_KRASIS: CardRecord = CardRecord::new_with_legacy_id(
    2144,
    "Unruly Krasis",
    CardArt::new("a3b1b58d-b7f1-404f-aec6-b19cef4bebbd", "Billy Christian"),
    CardSet::RavnicaClueEdition,
    CardRules::new_creature(
        mana_cost!("{1}{G}{U}"),
        &["Shark", "Octopus", "Lizard"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may have the base power and toughness of another target creature you control become X/X until end of turn, where X is this creature's power.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                // X is read once, as this resolves. It sets a base rather than adding to
                // one, so it overwrites an earlier setting effect while leaving counters and
                // ordinary pumps to apply on top.
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::SourcePower,
                        ValueDef::SourcePower,
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
        AbilityDef::activated(
            "{3}{G}{U}: Adapt 3. (If this creature has no +1/+1 counters on it, put three +1/+1 counters on it.)",
            &[AbilityCostDef::Mana(mana_cost!("{3}{G}{U}"))],
            // Adapt is a conditional, not a cost: the ability always activates and
            // always resolves, and finding a counter already there is what makes it do
            // nothing. So a creature that lost its counters can adapt again.
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::LessOrEqual,
                    amount: 0,
                },
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(3),
                },
            },
        ),
    ]),
);

// CLU 94 — Repeal
pub(in crate::card::sets) static REPEAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e7dd929-4bba-46a6-86c9-b8ed853eb721"),
    "Repeal",
    CardArt::new("265b80cd-2e9c-4e4b-a065-eafb29b3e07a", "Dan Murayama Scott"),
    CardSet::RavnicaClueEdition,
    // X is paid to match what it answers rather than to make it bigger, so
    // the cantrip is what keeps a one-mana mode from being a wasted card.
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent with mana value X to its owner's hand. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            // The draw is unconditional: it still happens when the target
            // has left before this resolves.
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// CLU 186 — Dimir Guildmage
pub(in crate::card::sets) static DIMIR_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9ab53af-749e-4559-85fa-f8d4181cf7da"),
    "Dimir Guildmage",
    CardArt::new("0b963389-6231-4095-a1f4-33457ce51ff2", "Adam Rex"),
    CardSet::RavnicaClueEdition,
    // Castable off either colour but only useful with both: the hybrid cost
    // is what gets it into the deck, and the two halves are why it stays.
    CardRules::new_creature(mana_cost!("{U/B}{U/B}"), &["Human", "Wizard"], 2, 2).with_abilities(
        &[
            AbilityDef::activated_with_targets(
                "{3}{U}: Target player draws a card. Activate only as a sorcery.",
                &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
            AbilityDef::activated_with_targets(
                "{3}{B}: Target player discards a card. Activate only as a sorcery.",
                &[AbilityCostDef::Mana(mana_cost!("{3}{B}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ],
    ),
);

// CLU 229 — Azorius Chancery
pub(in crate::card::sets) static AZORIUS_CHANCERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e58365d2-e4db-444b-b1a9-795668ad3038"),
    "Azorius Chancery",
    CardArt::new("a9d629f3-24b0-400c-b054-b66250696708", "John Avon"),
    CardSet::RavnicaClueEdition,
    // The blue-white karoo. Only the two colours below are its own; the rest
    // of the cycle prints the same two clauses word for word.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {W}{U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::White,
                ManaColor::Blue,
            )),
        ),
    ]),
);

// CLU 241 — Orzhov Basilica
pub(in crate::card::sets) static ORZHOV_BASILICA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9154d2a-3fc5-4fd6-9885-a810cb6b542a"),
    "Orzhov Basilica",
    CardArt::new("7c14375a-98c1-4e57-bf0d-1bea89a6bbd9", "John Avon"),
    CardSet::RavnicaClueEdition,
    // The white-black karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {W}{B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::White,
                ManaColor::Black,
            )),
        ),
    ]),
);

// CLU 246 — Selesnya Sanctuary
pub(in crate::card::sets) static SELESNYA_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5e51787-f9c9-4926-9df1-a384a3092676"),
    "Selesnya Sanctuary",
    CardArt::new("fdc53c6a-8e28-4314-9bcf-b31b6c6f56d7", "John Avon"),
    CardSet::RavnicaClueEdition,
    // The green-white karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {G}{W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Green,
                ManaColor::White,
            )),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HEADLINER_SCARLETT,
    &CARNAGE_INTERPRETER,
    &UNRULY_KRASIS,
    &REPEAL,
    &DIMIR_GUILDMAGE,
    &AZORIUS_CHANCERY,
    &ORZHOV_BASILICA,
    &SELESNYA_SANCTUARY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
