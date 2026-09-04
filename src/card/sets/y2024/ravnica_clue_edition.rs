//! Ravnica: Clue Edition cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    AppliedRuleDef, CardRules, CardSet, CardSupertype, CardType, ComparisonDef, CounterKind,
    DiscardSelectionDef, EffectDef, EffectRecipientDef, ExilePlayDurationDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, abilities, tokens,
};
use crate::{TargetIndex, mana_cost};

// CLU 4 — Headliner Scarlett
pub(in crate::card::sets) static HEADLINER_SCARLETT: CardRecord = CardRecord::new(
    CardSet::RavnicaClueEdition,
    "Headliner Scarlett",
    "be77b98a-dd79-477c-8ab2-7ebf5637a89e",
    "Heonhwa",
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
    CardSet::RavnicaClueEdition,
    "Carnage Interpreter",
    "f6fb576e-a4a4-496b-b553-3f81cc651210",
    "Justine Cruz",
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
pub(in crate::card::sets) static UNRULY_KRASIS: CardRecord = CardRecord::new(
    CardSet::RavnicaClueEdition,
    "Unruly Krasis",
    "a3b1b58d-b7f1-404f-aec6-b19cef4bebbd",
    "Billy Christian",
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

// CLU 94 — Repeal (reprint)
const REPEAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::guildpact::REPEAL,
    "265b80cd-2e9c-4e4b-a065-eafb29b3e07a",
    "Dan Murayama Scott",
);

// CLU 186 — Dimir Guildmage (reprint)
const DIMIR_GUILDMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::DIMIR_GUILDMAGE,
    "0b963389-6231-4095-a1f4-33457ce51ff2",
    "Adam Rex",
);

// CLU 229 — Azorius Chancery (reprint)
const AZORIUS_CHANCERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::dissension::AZORIUS_CHANCERY,
    "a9d629f3-24b0-400c-b054-b66250696708",
    "John Avon",
);

// CLU 241 — Orzhov Basilica (reprint)
const ORZHOV_BASILICA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::guildpact::ORZHOV_BASILICA,
    "7c14375a-98c1-4e57-bf0d-1bea89a6bbd9",
    "John Avon",
);

// CLU 246 — Selesnya Sanctuary (reprint)
const SELESNYA_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::SELESNYA_SANCTUARY,
    "fdc53c6a-8e28-4314-9bcf-b31b6c6f56d7",
    "John Avon",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&HEADLINER_SCARLETT, &CARNAGE_INTERPRETER, &UNRULY_KRASIS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    REPEAL_REPRINT,
    DIMIR_GUILDMAGE_REPRINT,
    AZORIUS_CHANCERY_REPRINT,
    ORZHOV_BASILICA_REPRINT,
    SELESNYA_SANCTUARY_REPRINT,
];
