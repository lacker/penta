//! Bloomburrow cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AlternativeCastKindDef, CardArt, CardRules, CardSet, CardType, ComparisonDef, CounterKind,
    EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRelation,
    TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static AN_INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

static AN_INSTANT_OR_SORCERY_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ]),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: AN_INSTANT_OR_SORCERY,
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: Some(PlayerRelation::You),
        },
    )];

static MAKE_AN_OTTER: EffectDef =
    EffectDef::create_creature_token(&["Otter"], &[ManaColor::Blue, ManaColor::Red], 1, 1)
        .with_abilities(&[abilities::prowess()])
        .with_art(CardArt::new(
            "e6b2c465-c446-4dee-9101-763105dcf813",
            "Julia Griffin",
        ));

/// A Class is level 1 with no counters on it, so climbing to two takes one
/// counter and to three takes two. Each level is bought separately and only
/// at sorcery speed (CR 717.2b), and only from below it.
static BELOW_LEVEL_TWO: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Level,
    comparison: ComparisonDef::Less,
    amount: 1,
};

static BELOW_LEVEL_THREE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Level,
    comparison: ComparisonDef::Less,
    amount: 2,
};

/// The level-3 clause only functions once the Class is there. Written as an
/// intervening-if, which is checked when it would trigger and again as it
/// resolves -- so a Class knocked back down between the two does not make
/// the Otter.
static AT_LEVEL_THREE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Level,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static STORMCHASERS_TALENT_ABILITIES: [AbilityDef; 5] = [
    AbilityDef::triggered(
        "When this Class enters, create a 1/1 blue and red Otter creature token with prowess.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        MAKE_AN_OTTER,
    ),
    AbilityDef::activated(
        "{3}{U}: Level 2",
        &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
        EffectDef::GainClassLevel { level: 2 },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
    .with_activation_condition(&BELOW_LEVEL_TWO),
    AbilityDef::triggered_with_targets(
        "When this Class becomes level 2, return target instant or sorcery card from your \
         graveyard to your hand.",
        TriggerEventDef::BecomesLevel(2),
        &AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD,
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
    AbilityDef::activated(
        "{5}{U}: Level 3",
        &[AbilityCostDef::Mana(mana_cost!("{5}{U}"))],
        EffectDef::GainClassLevel { level: 3 },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
    .with_activation_condition(&BELOW_LEVEL_THREE),
    AbilityDef::triggered_if(
        "Whenever you cast an instant or sorcery spell, create a 1/1 blue and red Otter creature \
         token with prowess.",
        TriggerEventDef::SpellCast(AN_INSTANT_OR_SORCERY_YOU_CAST),
        &AT_LEVEL_THREE,
        MAKE_AN_OTTER,
    ),
];

// BLB 54 — Kitsa, Otterball Elite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KITSA_OTTERBALL_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8ff751a-ec64-41d5-b22c-2a483ad9a9b2"),
    "Kitsa, Otterball Elite",
    crate::card::CardArt::new("c8ff751a-ec64-41d5-b22c-2a483ad9a9b2", "Zoltan Boros"),
    crate::card::CardSet::Bloomburrow,
    crate::card::CardRules::unsupported(),
);

// BLB 75 — Stormchaser's Talent
pub(in crate::card::sets) static STORMCHASERS_TALENT: CardRecord = CardRecord::new_with_legacy_id(
    2232,
    "Stormchaser's Talent",
    CardArt::new("a36e682d-b43d-4e08-bf5b-70d7e924dbe5", "Christina Kraus"),
    CardSet::Bloomburrow,
    // One mana for a body, and a mana sink that buys back a spell and then
    // turns every cantrip afterwards into another creature.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Class"])
        .with_abilities(&STORMCHASERS_TALENT_ABILITIES),
);

/// "If you do, when this creature enters": the arrival asks what the cast
/// paid, which the permanent recorded as it arrived.
static TRAINER_HAD_OFFSPRING: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Offspring);

/// A 1/1 copy of himself, which arrives with his own look at four attached
/// to it -- the whole reason the extra four mana is worth paying.
static TRAINER_OFFSPRING_TOKEN: EffectDef = EffectDef::CreateTokenCopyOf {
    object: EffectRecipientDef::Source,
    base_power_toughness: Some((1, 1)),
};

/// A noncreature, nonland card among the four, which is what the deck
/// playing him is digging for.
static A_NONCREATURE_NONLAND_CARD: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
]);

static TRAINER_DIGS: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: Some(A_NONCREATURE_NONLAND_CARD),
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: true,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    rest_random_order: true,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
};

static TRAINER_ARRIVES: TriggerEventDef = TriggerEventDef::zone_changed(
    ObjectPredicateDef::Source,
    None,
    Some(ZoneKind::Battlefield),
);

// BLB 78 — Thundertrap Trainer
pub(in crate::card::sets) static THUNDERTRAP_TRAINER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cf3af94-b7c8-415c-a5a1-d89967fd0bba"),
    "Thundertrap Trainer",
    CardArt::new("9cf3af94-b7c8-415c-a5a1-d89967fd0bba", "Matt Stewart"),
    CardSet::Bloomburrow,
    // Two mana to dig four cards deep for the spell you want, or six for two
    // bodies and two looks.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Otter", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{5}{U}"),
            AlternativeCastKindDef::Offspring,
            Some(
                "Offspring {4} (You may pay an additional {4} as you cast this spell. If you do, \
                 when this creature enters, create a 1/1 token copy of it.)",
            ),
            EffectDef::None,
        ),
        AbilityDef::triggered_if(
            "When this creature enters, create a 1/1 token copy of it.",
            TRAINER_ARRIVES,
            &TRAINER_HAD_OFFSPRING,
            TRAINER_OFFSPRING_TOKEN,
        ),
        AbilityDef::triggered(
            "When this creature enters, look at the top four cards of your library. You may \
             reveal a noncreature, nonland card from among them and put it into your hand. Put \
             the rest on the bottom of your library in a random order.",
            TRAINER_ARRIVES,
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &TRAINER_DIGS,
            },
        ),
    ]),
);

// BLB 208 — Cindering Cutthroat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CINDERING_CUTTHROAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2ea10dd-21ea-4622-be27-79d03a802b85"),
    "Cindering Cutthroat",
    crate::card::CardArt::new("b2ea10dd-21ea-4622-be27-79d03a802b85", "Wayne Reynolds"),
    crate::card::CardSet::Bloomburrow,
    crate::card::CardRules::unsupported(),
);

// BLB 235 — Tempest Angler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPEST_ANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("850daae4-f0b7-4604-95e7-ad044ec165c3"),
    "Tempest Angler",
    crate::card::CardArt::new("850daae4-f0b7-4604-95e7-ad044ec165c3", "Raluca Marinescu"),
    crate::card::CardSet::Bloomburrow,
    crate::card::CardRules::unsupported(),
);

// BLB 254 — Hidden Grotto
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_GROTTO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ba8f2e7-8357-4862-97dc-1942d066023a"),
    "Hidden Grotto",
    crate::card::CardArt::new("4ba8f2e7-8357-4862-97dc-1942d066023a", "Fiona Hsieh"),
    crate::card::CardSet::Bloomburrow,
    crate::card::CardRules::unsupported(),
);

// BLB 307 — Thundertrap Trainer (alternate printing)

// BLB 322 — Keen-Eyed Curator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEEN_EYED_CURATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("004a67ce-60ef-4cc2-9f4d-f30e3029d80a"),
    "Keen-Eyed Curator",
    crate::card::CardArt::new("004a67ce-60ef-4cc2-9f4d-f30e3029d80a", "Mariah Tekulve"),
    crate::card::CardSet::Bloomburrow,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KITSA_OTTERBALL_ELITE,
    &STORMCHASERS_TALENT,
    &THUNDERTRAP_TRAINER,
    &CINDERING_CUTTHROAT,
    &TEMPEST_ANGLER,
    &HIDDEN_GROTTO,
    &KEEN_EYED_CURATOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&THUNDERTRAP_TRAINER, 1), // BLB 307
];
