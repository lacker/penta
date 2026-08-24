//! Ravnica: Clue Edition cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef,
    CounterKind, EffectDef, EffectRecipientDef, ExilePlayDurationDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// CLU 4 — Headliner Scarlett
/// "Creatures target player controls." Read as the trigger resolves, so a
/// creature that arrives afterwards blocks perfectly well -- which is what
/// makes this a tempo card rather than an evasion one.
static CREATURES_THAT_PLAYER_CONTROLS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

static SCARLETT_ABILITIES: [AbilityDef; 3] = [
    abilities::haste(),
    AbilityDef::triggered_with_targets(
        "When Headliner Scarlett enters, creatures target player controls can't block this turn.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &CREATURES_THAT_PLAYER_CONTROLS,
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
        },
    ),
];

pub(in crate::card::sets) static HEADLINER_SCARLETT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be77b98a-dd79-477c-8ab2-7ebf5637a89e"),
    "Headliner Scarlett",
    CardArt::new("be77b98a-dd79-477c-8ab2-7ebf5637a89e", "Heonhwa"),
    CardSet::RavnicaClueEdition,
    // Four mana that attacks the turn it lands into a board that cannot
    // block, and then draws an extra card every turn it survives.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Warlock"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&SCARLETT_ABILITIES),
);

// CLU 26 — Carnage Interpreter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARNAGE_INTERPRETER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6fb576e-a4a4-496b-b553-3f81cc651210"),
    "Carnage Interpreter",
    crate::card::CardArt::new("f6fb576e-a4a4-496b-b553-3f81cc651210", "Justine Cruz"),
    crate::card::CardSet::RavnicaClueEdition,
    crate::card::CardRules::unsupported(),
);

// CLU 50 — Unruly Krasis
static ANOTHER_CREATURE_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// X is read once, as this resolves. It sets a base rather than adding to
/// one, so it overwrites an earlier setting effect while leaving counters and
/// ordinary pumps to apply on top.
static KRASIS_LENDS_ITS_BODY: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    effect: AppliedEffectDef::set_base_power_toughness(
        ValueDef::SourcePower,
        ValueDef::SourcePower,
    ),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

/// Adapt is a conditional, not a cost: the ability always activates and
/// always resolves, and finding a counter already there is what makes it do
/// nothing. So a creature that lost its counters can adapt again.
static KRASIS_ADAPTS: EffectDef = EffectDef::IfCondition {
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
};

static UNRULY_KRASIS_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::triggered_with_targets(
        "Whenever this creature attacks, you may have the base power and toughness of another target creature you control become X/X until end of turn, where X is this creature's power.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        &ANOTHER_CREATURE_YOU_CONTROL,
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &KRASIS_LENDS_ITS_BODY,
        },
    ),
    AbilityDef::activated(
        "{3}{G}{U}: Adapt 3. (If this creature has no +1/+1 counters on it, put three +1/+1 counters on it.)",
        &[AbilityCostDef::Mana(mana_cost!("{3}{G}{U}"))],
        KRASIS_ADAPTS,
    ),
];

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
    .with_abilities(&UNRULY_KRASIS_ABILITIES),
);

// CLU 94 — Repeal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REPEAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e7dd929-4bba-46a6-86c9-b8ed853eb721"),
    "Repeal",
    crate::card::CardArt::new("265b80cd-2e9c-4e4b-a065-eafb29b3e07a", "Dan Murayama Scott"),
    crate::card::CardSet::RavnicaClueEdition,
    crate::card::CardRules::unsupported(),
);

// CLU 186 — Dimir Guildmage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DIMIR_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9ab53af-749e-4559-85fa-f8d4181cf7da"),
    "Dimir Guildmage",
    crate::card::CardArt::new("0b963389-6231-4095-a1f4-33457ce51ff2", "Adam Rex"),
    crate::card::CardSet::RavnicaClueEdition,
    crate::card::CardRules::unsupported(),
);

// CLU 229 — Azorius Chancery
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AZORIUS_CHANCERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e58365d2-e4db-444b-b1a9-795668ad3038"),
    "Azorius Chancery",
    crate::card::CardArt::new("a9d629f3-24b0-400c-b054-b66250696708", "John Avon"),
    crate::card::CardSet::RavnicaClueEdition,
    crate::card::CardRules::unsupported(),
);

// CLU 241 — Orzhov Basilica
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORZHOV_BASILICA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9154d2a-3fc5-4fd6-9885-a810cb6b542a"),
    "Orzhov Basilica",
    crate::card::CardArt::new("7c14375a-98c1-4e57-bf0d-1bea89a6bbd9", "John Avon"),
    crate::card::CardSet::RavnicaClueEdition,
    crate::card::CardRules::unsupported(),
);

// CLU 246 — Selesnya Sanctuary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SELESNYA_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5e51787-f9c9-4926-9df1-a384a3092676"),
    "Selesnya Sanctuary",
    crate::card::CardArt::new("fdc53c6a-8e28-4314-9bcf-b31b6c6f56d7", "John Avon"),
    crate::card::CardSet::RavnicaClueEdition,
    crate::card::CardRules::unsupported(),
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
