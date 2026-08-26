//! The Big Score cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType, CardTypeSet,
    CopyExceptionsDef, EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef,
    PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// BIG 9 — Harvester of Misery
/// "Other creatures": everyone's, and not the Spirit itself, which is what
/// lets a 5/4 sweep a board of two-toughness creatures and survive it.
static EVERY_OTHER_CREATURE: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    ),
    effect: AppliedEffectDef::modify_power_toughness(
        ValueDef::Constant(-2),
        ValueDef::Constant(-2),
    ),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// The same shrink, aimed at one creature. The card is discarded to pay for
/// it, so this is what the Spirit does on the turns five mana is too much.
static HARVESTER_SHRINKS_ONE: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    effect: AppliedEffectDef::modify_power_toughness(
        ValueDef::Constant(-2),
        ValueDef::Constant(-2),
    ),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static HARVESTER_OF_MISERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3012af9-621d-4fae-b00d-079a89ae35fe"),
    "Harvester of Misery",
    CardArt::new("a3012af9-621d-4fae-b00d-079a89ae35fe", "Jorge Jacinto"),
    CardSet::TheBigScore,
    // Five mana for a board sweep on a hard-to-block body, or two mana from
    // the hand for one creature when the board does not need sweeping.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Spirit"], 5, 4).with_abilities(&[
        abilities::menace(),
        abilities::enters_trigger(
            "When this creature enters, other creatures get -2/-2 until end of turn.",
            EVERY_OTHER_CREATURE,
        ),
        AbilityDef::activated_with_targets(
            "{1}{B}, Discard this card: Target creature gets -2/-2 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{B}")),
                AbilityCostDef::DiscardSource,
            ],
            &A_CREATURE,
            HARVESTER_SHRINKS_ONE,
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// BIG 12 — Legion Extruder
/// Another one: the Extruder is an artifact itself and may not eat itself,
/// which is what stops a two-mana artifact from being a Golem on its own.
static ANOTHER_ARTIFACT: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

static EXTRUDER_GOLEM_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{2}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificePermanent {
        object: ANOTHER_ARTIFACT,
        controller: PlayerRelation::You,
    },
];

static LEGION_EXTRUDER_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger_with_targets(
        "When this artifact enters, it deals 2 damage to any target.",
        &ANY_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    ),
    AbilityDef::activated(
        "{2}, {T}, Sacrifice another artifact: Create a 3/3 colorless Golem artifact creature \
         token.",
        &EXTRUDER_GOLEM_COST,
        EffectDef::create_artifact_creature_token(&["Golem"], &[], 3, 3).with_art(CardArt::new(
            "406e2960-f560-48bb-b4a6-4bd35889a8f8",
            "Brian Valeza",
        )),
    ),
];

pub(in crate::card::sets) static LEGION_EXTRUDER: CardRecord = CardRecord::new_with_legacy_id(
    2288,
    "Legion Extruder",
    CardArt::new("5a077de0-1893-40d0-a499-ee2e6e2258f1", "Anton Solovianchyk"),
    CardSet::TheBigScore,
    // Two mana that answers a creature on the way in and then turns every
    // spent artifact -- a cracked Lotus Petal, an emptied Bauble -- into a
    // 3/3, which is what the cube's artifact decks have lying around.
    CardRules::new_artifact(mana_cost!("{1}{R}")).with_abilities(&LEGION_EXTRUDER_ABILITIES),
);

// BIG 21 — Loot, the Pathfinder
static LOOT_ABILITIES: [AbilityDef; 6] = [
    abilities::double_strike(),
    abilities::vigilance(),
    abilities::haste(),
    AbilityDef::activated_mana(
        "Exhaust — {G}, {T}: Add three mana of any one color. (Activate each exhaust ability \
         only once.)",
        &LOOT_GREEN_COST,
        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
    )
    .exhausting(),
    AbilityDef::activated(
        "Exhaust — {U}, {T}: Draw three cards.",
        &LOOT_BLUE_COST,
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    )
    .exhausting(),
    AbilityDef::activated_with_targets(
        "Exhaust — {R}, {T}: This creature deals 3 damage to any target.",
        &LOOT_RED_COST,
        &ANY_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )
    .exhausting(),
];

static LOOT_GREEN_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{G}")),
    AbilityCostDef::TapSource,
];

static LOOT_BLUE_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{U}")),
    AbilityCostDef::TapSource,
];

static LOOT_RED_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{R}")),
    AbilityCostDef::TapSource,
];

pub(in crate::card::sets) static LOOT_THE_PATHFINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb169fa2-c92e-45f7-89a2-0ca0e3910a1c"),
    "Loot, the Pathfinder",
    CardArt::new("fb169fa2-c92e-45f7-89a2-0ca0e3910a1c", "Rudy Siswanto"),
    CardSet::TheBigScore,
    // Five mana for a hasty double striker that also unloads three cards,
    // three mana, or three damage -- once each, and never twice, because
    // every one of them taps it.
    CardRules::new_creature(mana_cost!("{2}{G}{U}{R}"), &["Beast", "Noble"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&LOOT_ABILITIES),
);

// BIG 41 — Generous Plunderer
static AN_OPPONENT_PLUNDERER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

/// Yours untapped and theirs tapped, which is the whole of the bargain: the
/// Treasure you keep is usable this turn and the one you hand over is not.
static PLUNDERER_TREASURES: EffectDef = EffectDef::Sequence(&PLUNDERER_TREASURE_PAIR);

static PLUNDERER_TREASURE_PAIR: [EffectDef; 2] = [
    EffectDef::create_token(crate::card::tokens::treasure()),
    EffectDef::create_token(crate::card::tokens::treasure())
        .with_controller(PlayerRefDef::Target(TargetIndex::PRIMARY))
        .entering_tapped(),
];

/// Artifacts they control as the trigger resolves, which is what makes the
/// Treasure handed over on the upkeep into damage on the attack.
static THEIR_ARTIFACTS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

static PLUNDERER_ABILITIES: [AbilityDef; 3] = [
    abilities::menace(),
    // The opponent is named as the upkeep trigger goes on the stack rather
    // than when the Treasure is actually made, which is the one place this
    // differs from the printed reflexive trigger -- and with two players
    // there is only ever the one opponent to name.
    AbilityDef::triggered_with_targets(
        "At the beginning of your upkeep, you may create a Treasure token. When you do, target \
         opponent creates a tapped Treasure token.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        &AN_OPPONENT_PLUNDERER,
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &PLUNDERER_TREASURES,
        },
    ),
    // "Defending player" is the opponent in a two-player game, whether the
    // attack is aimed at them or at something they control.
    AbilityDef::triggered(
        "Whenever this creature attacks, it deals damage to defending player equal to the number \
         of artifacts they control.",
        TriggerEventDef::attack_declared(ObjectPredicateDef::Source, 1, None),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::CountMatchingObjects(&THEIR_ARTIFACTS),
        },
    ),
];

pub(in crate::card::sets) static GENEROUS_PLUNDERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("351eea06-f5be-4044-b3b3-cc6bf805abb1"),
    "Generous Plunderer",
    CardArt::new(
        "351eea06-f5be-4044-b3b3-cc6bf805abb1",
        "Josiah \"Jo\" Cameron",
    ),
    CardSet::TheBigScore,
    // Two mana for a 2/2 that hands the other player a Treasure every
    // upkeep and then bills them for it on the attack.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Rogue"], 2, 2)
        .with_abilities(&PLUNDERER_ABILITIES),
);

// BIG 85 — Vaultborn Tyrant
/// "This creature or another creature you control with power 4 or greater":
/// one predicate covers both halves, because the Tyrant is a 6/6 and so
/// matches the size clause itself.
static A_BIG_CREATURE_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ObjectPredicateDef::PowerAtLeast(4),
]);

static TYRANT_PAYS_ITS_CONTROLLER: [EffectDef; 2] = [
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

/// "If it's not a token", read off the creature that died rather than off
/// the card in the graveyard: without it every copy would make another copy
/// and the Tyrant would never stay dead.
static TYRANT_IS_NOT_A_TOKEN: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
};

/// The copy is of the creature as it last existed on the battlefield
/// (CR 608.2h), which is why a Tyrant that grew before it died comes back
/// the size it was.
static TYRANT_COPIES_ITSELF: EffectDef =
    EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
        object: &EffectRecipientDef::Source,
        exceptions: CopyExceptionsDef::NONE
            .with_added_types(CardTypeSet::single(CardType::Artifact)),
    });

static VAULTBORN_TYRANT_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::triggered(
        "Whenever this creature or another creature you control with power 4 or greater enters, \
         you gain 3 life and draw a card.",
        TriggerEventDef::zone_changed(
            A_BIG_CREATURE_YOU_CONTROL,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Sequence(&TYRANT_PAYS_ITS_CONTROLLER),
    ),
    AbilityDef::triggered_if(
        "When this creature dies, if it's not a token, create a token that's a copy of it, \
         except it's an artifact in addition to its other types.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        &TYRANT_IS_NOT_A_TOKEN,
        TYRANT_COPIES_ITSELF,
    ),
];

pub(in crate::card::sets) static VAULTBORN_TYRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07ca436a-e992-40a9-978a-501a82e443ed"),
    "Vaultborn Tyrant",
    crate::card::CardArt::new("07ca436a-e992-40a9-978a-501a82e443ed", "Loïc Canavaggia"),
    crate::card::CardSet::TheBigScore,
    // Seven mana that draws a card the moment it lands, and killing it hands
    // the same body back once.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 6, 6)
        .with_abilities(&VAULTBORN_TYRANT_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HARVESTER_OF_MISERY,
    &LEGION_EXTRUDER,
    &LOOT_THE_PATHFINDER,
    &GENEROUS_PLUNDERER,
    &VAULTBORN_TYRANT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
