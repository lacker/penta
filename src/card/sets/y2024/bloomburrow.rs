//! Bloomburrow cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AlternativeCastKindDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    ComparisonDef, CopyExceptionsDef, CopyStackObjectDef, CounterKind, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRefDef, PlayerRelation,
    TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, ValueComparisonDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// BLB 54 — Kitsa, Otterball Elite
static KITSA_LOOTS: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RecipientChooses,
        then: None,
    },
];

/// Yours rather than anybody's: Kitsa copies what you are casting, not what
/// is being cast at you.
static YOUR_INSTANT_OR_SORCERY_SPELL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Instant),
            ObjectPredicateDef::HasType(CardType::Sorcery),
        ]),
        zones: &[ZoneKind::Stack],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// Read live where the activation is offered, so the prowess trigger from
/// the spell being copied is what turns the ability on: a 1/3 that has cast
/// two noncreature spells this turn is a 3/5.
static KITSA_IS_BIG_ENOUGH: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
        left: ValueDef::SourcePower,
        comparison: ComparisonDef::GreaterOrEqual,
        right: ValueDef::Constant(3),
    });

static KITSA_LOOT_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

static KITSA_COPY_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{2}")),
    AbilityCostDef::TapSource,
];

static KITSA_COPIES_A_SPELL: CopyStackObjectDef = CopyStackObjectDef {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    controller: PlayerRefDef::EffectController,
    count: ValueDef::Constant(1),
    retarget: true,
    colors: None,
};

static KITSA_ABILITIES: [AbilityDef; 4] = [
    abilities::vigilance(),
    abilities::prowess(),
    AbilityDef::activated(
        "{T}: Draw a card, then discard a card.",
        &KITSA_LOOT_COST,
        EffectDef::Sequence(&KITSA_LOOTS),
    ),
    AbilityDef::activated_with_targets(
        "{2}, {T}: Copy target instant or sorcery spell you control. You may choose new targets \
         for the copy. Activate only if Kitsa's power is 3 or greater.",
        &KITSA_COPY_COST,
        &YOUR_INSTANT_OR_SORCERY_SPELL,
        EffectDef::CopyStackObject(&KITSA_COPIES_A_SPELL),
    )
    .with_activation_condition(&KITSA_IS_BIG_ENOUGH),
];

pub(in crate::card::sets) static KITSA_OTTERBALL_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8ff751a-ec64-41d5-b22c-2a483ad9a9b2"),
    "Kitsa, Otterball Elite",
    CardArt::new("c8ff751a-ec64-41d5-b22c-2a483ad9a9b2", "Zoltan Boros"),
    CardSet::Bloomburrow,
    // Two mana for a body that loots every turn it has nothing better to
    // do, and copies the spell that made it big enough on the turns it
    // does. Vigilance is why the tap is not a real cost.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Otter", "Wizard"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&KITSA_ABILITIES),
);

// BLB 75 — Stormchaser's Talent
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
    kind: CounterKind::named("level"),
    comparison: ComparisonDef::Less,
    amount: 1,
};

static BELOW_LEVEL_THREE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::named("level"),
    comparison: ComparisonDef::Less,
    amount: 2,
};

/// The level-3 clause only functions once the Class is there. Written as an
/// intervening-if, which is checked when it would trigger and again as it
/// resolves -- so a Class knocked back down between the two does not make
/// the Otter.
static AT_LEVEL_THREE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::named("level"),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static STORMCHASERS_TALENT_ABILITIES: [AbilityDef; 5] = [
    abilities::enters_trigger(
        "When this Class enters, create a 1/1 blue and red Otter creature token with prowess.",
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
            tapped: false,
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

// BLB 78 — Thundertrap Trainer
/// "If you do, when this creature enters": the arrival asks what the cast
/// paid, which the permanent recorded as it arrived.
static TRAINER_HAD_OFFSPRING: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Offspring);

/// A 1/1 copy of himself, which arrives with his own look at four attached
/// to it -- the whole reason the extra four mana is worth paying.
static TRAINER_OFFSPRING_TOKEN: EffectDef =
    EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
        object: &EffectRecipientDef::Source,
        exceptions: CopyExceptionsDef::power_toughness(1, 1),
    });

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
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: true,
    counted: None,
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
static A_CARD_IN_A_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    },
)];

static CURATOR_EXILE_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{1}"))];

/// Four card types among the cards he took, counted over the pile rather
/// than over any zone: he keeps them, so a card that leaves exile stops
/// counting and the rest still do.
static CURATOR_HAS_FOUR_TYPES: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
        left: ValueDef::CardTypesAmongLinkedExiles,
        comparison: ComparisonDef::GreaterOrEqual,
        right: ValueDef::Constant(4),
    });

static CURATOR_TRAMPLE: AbilityDef = abilities::trample();

static CURATOR_GRANTS: [EffectDef; 2] = [
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(4),
            ValueDef::Constant(4),
        ),
    },
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&CURATOR_TRAMPLE),
    },
];

static CURATOR_BONUS: EffectDef = EffectDef::Sequence(&CURATOR_GRANTS);

static CURATOR_ABILITIES: [AbilityDef; 2] = [
    // "As long as", so the 7/7 comes and goes with the pile rather than
    // being settled once.
    AbilityDef::static_ability(
        "As long as there are four or more card types among cards exiled with this creature, it \
         gets +4/+4 and has trample.",
        EffectDef::IfCondition {
            condition: &CURATOR_HAS_FOUR_TYPES,
            then: &CURATOR_BONUS,
        },
    ),
    // Either graveyard: what he is played for is emptying theirs, and the
    // card types he needs come from wherever they are.
    AbilityDef::activated_with_targets(
        "{1}: Exile target card from a graveyard.",
        &CURATOR_EXILE_COST,
        &A_CARD_IN_A_GRAVEYARD,
        EffectDef::ExileLinkedToSource {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            face_down: false,
            then: None,
        },
    ),
];

pub(in crate::card::sets) static KEEN_EYED_CURATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("004a67ce-60ef-4cc2-9f4d-f30e3029d80a"),
    "Keen-Eyed Curator",
    CardArt::new("004a67ce-60ef-4cc2-9f4d-f30e3029d80a", "Mariah Tekulve"),
    CardSet::Bloomburrow,
    // Two mana for a 3/3 that answers a graveyard a card at a time, and
    // turns into a 7/7 trampler for having done it four kinds of times.
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Raccoon", "Scout"], 3, 3)
        .with_abilities(&CURATOR_ABILITIES),
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
