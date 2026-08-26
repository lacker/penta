//! Kamigawa: Neon Dynasty attachment edge cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityTargetDef,
    AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef,
    CostAdjustmentDef, CostAmountDef, CostModificationDef, CounterKind, CreatedTokensDef,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ResolvedEffectDurationDef, SpellCostConditionDef, TokenCharacteristics,
    TokenCopyExceptionsDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, tokens,
};
use crate::ids::{ObjectSetBindingIndex, TargetIndex};
use crate::mana_cost;

// NEO 17 — Imperial Oath
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IMPERIAL_OATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d6750dd-2303-493b-885d-1bfb5787b16c"),
    "Imperial Oath",
    crate::card::CardArt::new("3d6750dd-2303-493b-885d-1bfb5787b16c", "Nicholas Elias"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 26 — Lion Sash
/// A card in anybody's graveyard, which is what "from a graveyard" means:
/// yours as readily as theirs.
static A_CARD_IN_A_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    },
)];

/// A permanent card is one of the types that can stay on the battlefield.
/// Asked of the target while it is still in the graveyard, which is what
/// "if it was" means once it has been exiled.
static A_PERMANENT_CARD: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Enchantment),
        ObjectPredicateDef::HasType(CardType::Land),
        ObjectPredicateDef::HasType(CardType::Planeswalker),
    ]),
};

static SASH_GROWS: EffectDef = EffectDef::AddCounters {
    object: EffectRecipientDef::Source,
    kind: CounterKind::PlusOnePlusOne,
    amount: ValueDef::Constant(1),
};

/// The counter is decided before the card moves: a card in exile is no
/// longer where the target slot is looking.
static LION_SASH_EXILE: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &A_PERMANENT_CARD,
        then: &SASH_GROWS,
    },
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        from: None,
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
        tapped: false,
    },
];

pub(in crate::card::sets) static LION_SASH: CardRecord = CardRecord::new_with_legacy_id(
    2243,
    "Lion Sash",
    CardArt::new("3e1766e9-2fa7-4446-a255-7beea1467ece", "Yongjae Choi"),
    CardSet::KamigawaNeonDynasty,
    // Graveyard hate that grows into a threat, and reconfigure means the
    // two halves are the same card rather than a choice made on turn two.
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Equipment", "Cat"], 1, 1)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{W}: Exile target card from a graveyard. If it was a permanent card, put a \
                 +1/+1 counter on this permanent.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                &A_CARD_IN_A_GRAVEYARD,
                EffectDef::Sequence(&LION_SASH_EXILE),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 for each +1/+1 counter on this Equipment.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
                        ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
                    ),
                },
            ),
            abilities::reconfigure(
                mana_cost!("{2}"),
                "Reconfigure {2} ({2}: Attach to target creature you control; or unattach from a \
                 creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

// NEO 40 — Touch the Spirit Realm
/// "Until this enchantment leaves the battlefield" is one printed clause, so
/// the return rides on a delayed trigger rather than appearing as a second
/// ability the card does not print.
static TOUCH_RETURNS_IT: AbilityDef = AbilityDef::triggered(
    "When this enchantment leaves the battlefield, return the exiled card to the battlefield \
     under its owner's control.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        Some(ZoneKind::Battlefield),
        None,
    ),
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: false,
    },
);

static AN_ARTIFACT_OR_CREATURE: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
]);

static UP_TO_ONE_ARTIFACT_OR_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: AN_ARTIFACT_OR_CREATURE,
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

static ONE_ARTIFACT_OR_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    AN_ARTIFACT_OR_CREATURE,
)];

static TOUCH_EXILES_IT: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        face_down: false,
        then: None,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&TOUCH_RETURNS_IT)),
];

static TOUCH_CHANNEL_COST: AbilityCostList = AbilityCostList::two(
    AbilityCostDef::Mana(mana_cost!("{1}{W}")),
    AbilityCostDef::DiscardSource,
);

pub(in crate::card::sets) static TOUCH_THE_SPIRIT_REALM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e16ab44e-4257-4c0c-b705-8ac1e9c1d835"),
    "Touch the Spirit Realm",
    CardArt::new("e16ab44e-4257-4c0c-b705-8ac1e9c1d835", "Marta Nael"),
    CardSet::KamigawaNeonDynasty,
    // Three mana to answer something for as long as the enchantment lives,
    // or two from hand to blink one of yours -- which is why it is never
    // quite dead.
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, exile up to one target artifact or creature until this \
             enchantment leaves the battlefield.",
            &UP_TO_ONE_ARTIFACT_OR_CREATURE,
            EffectDef::Sequence(&TOUCH_EXILES_IT),
        ),
        AbilityDef::activated_with_cost_list_and_targets(
            "Channel — {1}{W}, Discard this card: Exile target artifact or creature. Return it to \
             the battlefield under its owner's control at the beginning of the next end step.",
            TOUCH_CHANNEL_COST,
            &ONE_ARTIFACT_OR_CREATURE,
            abilities::exile_until_next_end_step(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// NEO 42 — The Wandering Emperor
/// "As long as The Wandering Emperor entered this turn": the permission is
/// hers for the turn she lands and no longer, which is what makes flashing
/// her in at the end of a turn a plan rather than a waste.
static EMPEROR_ENTERED_THIS_TURN: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::EnteredThisTurn,
};

static EMPEROR_INSTANT_SPEED_LOYALTY: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayActivateLoyaltyAnyTime),
};

/// "Up to one target creature", which is what keeps the plus activatable on
/// an empty board.
static UP_TO_ONE_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

/// A tapped creature: the minus answers an attacker that has already
/// committed, which is the half of removal flash was made for.
static A_TAPPED_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Tapped,
    ]),
)];

static EMPEROR_COUNTER_AND_FIRST_STRIKE: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&FIRST_STRIKE),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

static FIRST_STRIKE: AbilityDef = abilities::first_strike();

static EMPEROR_EXILE_AND_GAIN: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        from: None,
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
        tapped: false,
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

static WANDERING_EMPEROR_ABILITIES: [AbilityDef; 5] = [
    abilities::flash(),
    AbilityDef::static_ability(
        "As long as The Wandering Emperor entered this turn, you may activate her loyalty \
         abilities any time you could cast an instant.",
        EffectDef::IfCondition {
            condition: &EMPEROR_ENTERED_THIS_TURN,
            then: &EMPEROR_INSTANT_SPEED_LOYALTY,
        },
    ),
    AbilityDef::activated_with_targets(
        "+1: Put a +1/+1 counter on up to one target creature. It gains first strike until end of \
         turn.",
        &[AbilityCostDef::Loyalty(1)],
        &UP_TO_ONE_CREATURE,
        EffectDef::Sequence(&EMPEROR_COUNTER_AND_FIRST_STRIKE),
    ),
    AbilityDef::activated(
        "−1: Create a 2/2 white Samurai creature token with vigilance.",
        &[AbilityCostDef::Loyalty(-1)],
        EffectDef::create_creature_token(&["Samurai"], &[ManaColor::White], 2, 2)
            .with_abilities(&[abilities::vigilance()])
            .with_art(CardArt::new(
                "f68e5337-6e44-4f8f-a102-2f97b433beea",
                "Gaboleps",
            )),
    ),
    AbilityDef::activated_with_targets(
        "−2: Exile target tapped creature. You gain 2 life.",
        &[AbilityCostDef::Loyalty(-2)],
        &A_TAPPED_CREATURE,
        EffectDef::Sequence(&EMPEROR_EXILE_AND_GAIN),
    ),
];

pub(in crate::card::sets) static THE_WANDERING_EMPEROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fab2d8a9-ab4c-4225-a570-22636293c17d"),
    "The Wandering Emperor",
    CardArt::new("fab2d8a9-ab4c-4225-a570-22636293c17d", "Tommy Arnold"),
    CardSet::KamigawaNeonDynasty,
    // A planeswalker you cast on their turn: she answers an attacker, makes
    // a blocker, or wins a fight, and she does it before they can respond by
    // killing her.
    CardRules::new_planeswalker(mana_cost!("{2}{W}{W}"), &["The Wandering Emperor"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&WANDERING_EMPEROR_ABILITIES),
);

// NEO 63 — Mirrorshell Crab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRORSHELL_CRAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0394c8df-2e8a-4477-93b7-569934d7b936"),
    "Mirrorshell Crab",
    crate::card::CardArt::new("0394c8df-2e8a-4477-93b7-569934d7b936", "Cristi Balanescu"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 67 — Moon-Circuit Hacker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOON_CIRCUIT_HACKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75c43923-7280-4ccb-810b-e8c38dd8a26f"),
    "Moon-Circuit Hacker",
    crate::card::CardArt::new("c6e466d1-943d-41e6-a47d-c9d951ca4262", "Tia Masic"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 91 — Clawing Torment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLAWING_TORMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("621fce96-5933-4e2b-98ec-2589940e24cb"),
    "Clawing Torment",
    crate::card::CardArt::new("621fce96-5933-4e2b-98ec-2589940e24cb", "Rovina Cai"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 117 — Okiba Reckoner Raid // Nezumi Road Captain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OKIBA_RECKONER_RAID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f0582b4-d951-4450-b158-4a34109e48cd"),
    "Okiba Reckoner Raid",
    crate::card::CardArt::new(
        "4f0582b4-d951-4450-b158-4a34109e48cd",
        "Victor Adame Minguez",
    ),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 148 — Ironhoof Boar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IRONHOOF_BOAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73abe574-6fb8-4809-9c18-0cf989f986f5"),
    "Ironhoof Boar",
    crate::card::CardArt::new(
        "73abe574-6fb8-4809-9c18-0cf989f986f5",
        "Antonio José Manzanedo",
    ),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 157 — Rabbit Battery
pub(in crate::card::sets) static RABBIT_BATTERY: CardRecord = CardRecord::new_with_legacy_id(
    1706,
    "Rabbit Battery",
    CardArt::new("5d33a5b7-797b-4079-8d62-edd124c0fb5a", "Justyna Dura"),
    CardSet::KamigawaNeonDynasty,
    CardRules::new_artifact_creature(mana_cost!("{R}"), &["Equipment", "Rabbit"], 1, 1)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            abilities::reconfigure(
                mana_cost!("{R}"),
                "Reconfigure {R} ({R}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

// NEO 189 — Greater Tanuki
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GREATER_TANUKI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4fbaee3-a10f-4b2d-b07e-d041a96a7e27"),
    "Greater Tanuki",
    crate::card::CardArt::new("b4fbaee3-a10f-4b2d-b07e-d041a96a7e27", "Ilse Gort"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 211 — Tamiyo's Safekeeping
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TAMIYO_S_SAFEKEEPING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd4b7ee2-de65-4288-872d-486065a4f226"),
    "Tamiyo's Safekeeping",
    crate::card::CardArt::new("fd4b7ee2-de65-4288-872d-486065a4f226", "Aurore Folny"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 222 — Hinata, Dawn-Crowned
static HINATA_ABILITIES: [AbilityDef; 4] = [
    abilities::flying(),
    abilities::trample(),
    abilities::spell_cost_adjustment(
        "Spells you cast cost {1} less to cast for each target.",
        ObjectPredicateDef::Any,
        PlayerRelation::You,
        SpellCostConditionDef::Always,
        CostAdjustmentDef::Subtract(CostAmountDef::Generic(ValueDef::DistinctTargets)),
    ),
    abilities::spell_cost_adjustment(
        "Spells your opponents cast cost {1} more to cast for each target.",
        ObjectPredicateDef::Any,
        PlayerRelation::Opponent,
        SpellCostConditionDef::Always,
        CostAdjustmentDef::Add(CostAmountDef::Generic(ValueDef::DistinctTargets)),
    ),
];

pub(in crate::card::sets) static HINATA_DAWN_CROWNED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f25aff90-56fd-4f70-bb3b-cabf2900c391"),
    "Hinata, Dawn-Crowned",
    CardArt::new("f25aff90-56fd-4f70-bb3b-cabf2900c391", "Alexander Mokhov"),
    CardSet::KamigawaNeonDynasty,
    CardRules::new_creature(mana_cost!("{1}{U}{R}{W}"), &["Kirin", "Spirit"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&HINATA_ABILITIES),
);

// NEO 238 — Tamiyo, Compleated Sage
// Audit: partial — Compleated and +1 are executable; −7 creates a Notebook whose cost reduction does not yet reduce announced X, and −X needs variable loyalty costs plus arbitrary graveyard-card copy tokens using last-known information.
static TAMIYO_PLUS_ONE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

static TAMIYO_PLUS_ONE_EFFECTS: [EffectDef; 2] = [
    EffectDef::Tap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::SkipNextUntapSteps {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        count: 1,
    },
];

static TAMIYOS_NOTEBOOK_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "Spells you cast cost {2} less to cast.",
        EffectDef::ModifyCost(CostModificationDef::reduce_spell(
            ObjectPredicateDef::Any,
            PlayerRelation::You,
            ValueDef::Constant(2),
        )),
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The generic reduction applies to printed generic mana but does not yet reduce a spell's announced X payment.",
    )),
    AbilityDef::activated(
        "{T}: Draw a card.",
        &[AbilityCostDef::TapSource],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
];

static TAMIYOS_NOTEBOOK: TokenCharacteristics = TokenCharacteristics::artifact(&["Book"], &[])
    .with_name("Tamiyo's Notebook")
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&TAMIYOS_NOTEBOOK_ABILITIES);

static TAMIYO_ABILITIES: [AbilityDef; 4] = [
    abilities::compleated(
        "Compleated ({G/U/P} can be paid with {G}, {U}, or 2 life. If life was paid, this planeswalker enters with two fewer loyalty counters.)",
    ),
    AbilityDef::activated_with_targets(
        "+1: Tap up to one target artifact or creature. It doesn't untap during its controller's next untap step.",
        &[AbilityCostDef::Loyalty(1)],
        &TAMIYO_PLUS_ONE_TARGET,
        EffectDef::Sequence(&TAMIYO_PLUS_ONE_EFFECTS),
    ),
    AbilityDef::not_implemented(
        "−X: Exile target nonland permanent card with mana value X from your graveyard. Create a token that's a copy of that card.",
        "The engine supports only fixed loyalty costs and cannot create a token from the last-known characteristics of an arbitrary targeted graveyard card.",
    ),
    AbilityDef::activated(
        "−7: Create Tamiyo's Notebook, a legendary colorless Book artifact token with \"Spells you cast cost {2} less to cast\" and \"{T}: Draw a card.\"",
        &[AbilityCostDef::Loyalty(-7)],
        EffectDef::create_token(TAMIYOS_NOTEBOOK),
    ),
];

pub(in crate::card::sets) static TAMIYO_COMPLEATED_SAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("222a736e-d819-452d-aeda-eb848c4b2302"),
    "Tamiyo, Compleated Sage",
    CardArt::new("222a736e-d819-452d-aeda-eb848c4b2302", "Chris Rahn"),
    CardSet::KamigawaNeonDynasty,
    CardRules::new_planeswalker(mana_cost!("{2}{G}{G/U/P}{U}"), &["Tamiyo"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&TAMIYO_ABILITIES),
);

// NEO 248 — Iron Apprentice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13d6d9fc-509b-42db-8ac1-85066eb6e9c4"),
    "Iron Apprentice",
    crate::card::CardArt::new("13d6d9fc-509b-42db-8ac1-85066eb6e9c4", "Kekai Kotaki"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 271 — Otawara, Soaring City
/// Everything a bounce spell would want and nothing else: a land answers a
/// creature, but not another land.
static AN_ARTIFACT_CREATURE_ENCHANTMENT_OR_PLANESWALKER: ObjectPredicateDef =
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Enchantment),
        ObjectPredicateDef::HasType(CardType::Planeswalker),
    ]);

static ONE_NONLAND_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    AN_ARTIFACT_CREATURE_ENCHANTMENT_OR_PLANESWALKER,
)];

/// The discount, which is what makes the land a spell: a legendary board
/// takes the channel cost down toward the {U} that cannot be reduced away.
static LEGENDARY_CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
        ObjectPredicateDef::HasType(CardType::Creature),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static OTAWARA_CHANNEL_COST: AbilityCostList = AbilityCostList::two(
    AbilityCostDef::Mana(mana_cost!("{3}{U}")),
    AbilityCostDef::DiscardSource,
);

pub(in crate::card::sets) static OTAWARA_SOARING_CITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("486d7edc-d983-41f0-8b78-c99aecd72996"),
    "Otawara, Soaring City",
    CardArt::new("486d7edc-d983-41f0-8b78-c99aecd72996", "Alayna Danner"),
    CardSet::KamigawaNeonDynasty,
    // A land that costs nothing to play and is never a dead draw, which is
    // the whole of why the cycle is in the cube.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {U}.",
                &OTAWARA_MANA_COST,
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
            ),
            AbilityDef::activated_with_cost_list_and_targets(
                "Channel — {3}{U}, Discard this card: Return target artifact, creature, \
                 enchantment, or planeswalker to its owner\'s hand. This ability costs {1} less \
                 to activate for each legendary creature you control.",
                OTAWARA_CHANNEL_COST,
                &ONE_NONLAND_PERMANENT,
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    from: None,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                    tapped: false,
                },
            )
            .with_source_zones(&[ZoneKind::Hand])
            .with_activation_cost_reduction(
                ValueDef::CountMatchingObjects(&LEGENDARY_CREATURES_YOU_CONTROL),
                0,
            ),
        ]),
);

static OTAWARA_MANA_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

// NEO 357 — Fable of the Mirror-Breaker // Reflection of Kiki-Jiki
/// The Goblin's own clause, printed on the token rather than on the Saga.
static GOBLIN_MAKES_TREASURE: [AbilityDef; 1] = [AbilityDef::triggered(
    "Whenever this token attacks, create a Treasure token.",
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
    EffectDef::create_token(tokens::treasure()),
)];

/// "Discard up to two cards. If you do, draw that many." The size is the
/// player's to choose, so the discard is a choice with a floor of none and
/// what is drawn is however many that turned out to be.
static FABLE_REFILLS: [EffectDef; 2] = [
    EffectDef::DiscardCards {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::BoundObjectCount(ObjectSetBindingIndex::PRIMARY),
    },
];

static FABLE_LOOTS: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
        ObjectPredicateDef::Any,
        &[ZoneKind::Hand],
        PlayerSetDef::One(PlayerRefDef::EffectController),
    )),
    exclude: None,
    minimum: 0,
    maximum: 2,
    visibility: ChoiceVisibilityDef::Private,
    then: &EffectDef::Sequence(&FABLE_REFILLS),
});

static FABLE_CHAPTERS: [AbilityDef; 3] = [
    abilities::saga_chapter(
        1,
        "I — Create a 2/2 red Goblin Shaman creature token with \"Whenever this token attacks, \
         create a Treasure token.\"",
        EffectDef::create_creature_token(&["Goblin", "Shaman"], &[ManaColor::Red], 2, 2)
            .with_abilities(&GOBLIN_MAKES_TREASURE),
    ),
    abilities::saga_chapter(
        2,
        "II — You may discard up to two cards. If you do, draw that many cards.",
        FABLE_LOOTS,
    ),
    abilities::saga_chapter(
        3,
        "III — Exile this Saga, then return it to the battlefield transformed under your control.",
        abilities::exile_and_return_transformed(EffectRecipientDef::Source),
    ),
];

const fn fable_front_rules() -> CardRules {
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&FABLE_CHAPTERS)
}

/// "Another target nonlegendary creature you control": the Reflection may
/// not copy itself, and a legendary copy would be put into a graveyard by
/// the legend rule the moment it arrived.
static ANOTHER_NONLEGENDARY_CREATURE_YOU_CONTROL: [AbilityTargetDef; 1] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    })
    .excluding_source(),
];

static KIKI_GRANTS_HASTE: AbilityDef = abilities::haste();

static KIKI_SACRIFICES_IT: AbilityDef = AbilityDef::triggered(
    "Sacrifice it at the beginning of the next end step.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::Sacrifice {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(KIKI_COPY)),
    },
);

const KIKI_COPY: ObjectSetBindingIndex = ObjectSetBindingIndex::PRIMARY;

static KIKI_COPIES: EffectDef = EffectDef::CreateTokenCopyOf {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    exceptions: TokenCopyExceptionsDef::with_ability(&KIKI_GRANTS_HASTE),
    created: Some(CreatedTokensDef {
        binding: KIKI_COPY,
        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&KIKI_SACRIFICES_IT)),
    }),
};

static KIKI_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
];

static KIKI_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_with_targets(
    "{1}, {T}: Create a token that's a copy of another target nonlegendary creature you control, \
     except it has haste. Sacrifice it at the beginning of the next end step.",
    &KIKI_COST,
    &ANOTHER_NONLEGENDARY_CREATURE_YOU_CONTROL,
    KIKI_COPIES,
)];

const fn fable_back_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Goblin", "Shaman"], 2, 2)
        .with_type(CardType::Enchantment)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&KIKI_ABILITIES)
}

static FABLE_FACES: [(&str, CardRules); 2] = [
    ("Fable of the Mirror-Breaker", fable_front_rules()),
    ("Reflection of Kiki-Jiki", fable_back_rules()),
];

pub(in crate::card::sets) static FABLE_OF_THE_MIRROR_BREAKER: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("0b696cd1-0d72-4df5-bacc-dc77e62f9a13"),
    "Fable of the Mirror-Breaker // Reflection of Kiki-Jiki",
    CardArt::new("0b696cd1-0d72-4df5-bacc-dc77e62f9a13", "akio"),
    CardSet::KamigawaNeonDynasty,
    // Three mana that pays for itself twice over: a body, a loot, and then
    // the half nobody reads the Saga for.
    &FABLE_FACES,
);

// NEO 412 — Boseiju, Who Endures
/// "Nonbasic" is the whole reason the land half is in the target list: every
/// land worth answering is one, and a basic is never worth the card.
static AN_ARTIFACT_ENCHANTMENT_OR_NONBASIC_LAND: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Enchantment),
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Land),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
    ]),
]);

static ONE_OF_THEIRS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: AN_ARTIFACT_ENCHANTMENT_OR_NONBASIC_LAND,
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

/// "A land card with a basic land type", which is what makes the
/// compensation a fixing land rather than a card: a dual with a basic type
/// counts and a Wasteland does not.
static A_LAND_WITH_A_BASIC_TYPE: ObjectPredicateDef =
    ObjectPredicateDef::HasAnyBasicLandType(&BasicLandType::ALL);

/// Their search, not yours: the player whose permanent was destroyed is the
/// one who may go looking, and the land arrives untapped.
static THEY_MAY_REPLACE_IT: EffectDef = EffectDef::May {
    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
        TargetIndex::PRIMARY,
    ))),
    effect: &EffectDef::SearchZone {
        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
            TargetIndex::PRIMARY,
        ))),
        source: ZoneKind::Library,
        object: A_LAND_WITH_A_BASIC_TYPE,
        minimum: 0,
        maximum: ValueDef::Constant(1),
        reveal: false,
        destination: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: false,
        attachment: None,
        binding: None,
        then: None,
    },
};

static BOSEIJU_CHANNEL: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
        then: None,
    },
    THEY_MAY_REPLACE_IT,
];

static BOSEIJU_CHANNEL_COST: AbilityCostList = AbilityCostList::two(
    AbilityCostDef::Mana(mana_cost!("{1}{G}")),
    AbilityCostDef::DiscardSource,
);

static BOSEIJU_MANA_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

pub(in crate::card::sets) static BOSEIJU_WHO_ENDURES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0055ea30-20fb-4324-a632-8fed87628f05"),
    "Boseiju, Who Endures",
    CardArt::new("0055ea30-20fb-4324-a632-8fed87628f05", "Esuthio"),
    CardSet::KamigawaNeonDynasty,
    // A Forest that answers the one artifact the deck could not otherwise
    // beat, and costs nothing to play when it does not have to.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {G}.",
                &BOSEIJU_MANA_COST,
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ),
            AbilityDef::activated_with_cost_list_and_targets(
                "Channel — {1}{G}, Discard this card: Destroy target artifact, enchantment, or \
                 nonbasic land an opponent controls. That player may search their library for a \
                 land card with a basic land type, put it onto the battlefield, then shuffle. \
                 This ability costs {1} less to activate for each legendary creature you control.",
                BOSEIJU_CHANNEL_COST,
                &ONE_OF_THEIRS,
                EffectDef::Sequence(&BOSEIJU_CHANNEL),
            )
            .with_source_zones(&[ZoneKind::Hand])
            .with_activation_cost_reduction(
                ValueDef::CountMatchingObjects(&LEGENDARY_CREATURES_YOU_CONTROL),
                0,
            ),
        ]),
);

// NEO 418 — The Wandering Emperor (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &IMPERIAL_OATH,
    &LION_SASH,
    &TOUCH_THE_SPIRIT_REALM,
    &THE_WANDERING_EMPEROR,
    &MIRRORSHELL_CRAB,
    &MOON_CIRCUIT_HACKER,
    &CLAWING_TORMENT,
    &OKIBA_RECKONER_RAID,
    &IRONHOOF_BOAR,
    &RABBIT_BATTERY,
    &GREATER_TANUKI,
    &TAMIYO_S_SAFEKEEPING,
    &HINATA_DAWN_CROWNED,
    &TAMIYO_COMPLEATED_SAGE,
    &IRON_APPRENTICE,
    &OTAWARA_SOARING_CITY,
    &FABLE_OF_THE_MIRROR_BREAKER,
    &BOSEIJU_WHO_ENDURES,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&THE_WANDERING_EMPEROR, 1), // NEO 418
];
