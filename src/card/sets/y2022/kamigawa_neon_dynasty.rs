//! Kamigawa: Neon Dynasty attachment edge cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType, CostModificationDef,
    CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    TokenCharacteristics, TriggerConditionDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

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
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
    },
];

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

// NEO 117 — Okiba Reckoner Raid
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
        EffectDef::ModifyCost(CostModificationDef::SpellReduction {
            spell: ObjectPredicateDef::Any,
            caster: PlayerRelation::You,
            amount: ValueDef::Constant(2),
        }),
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

// NEO 238 — Tamiyo, Compleated Sage
// Audit: partial — Compleated and +1 are executable; −7 creates a Notebook whose cost reduction does not yet reduce announced X, and −X needs variable loyalty costs plus arbitrary graveyard-card copy tokens using last-known information.
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &IMPERIAL_OATH,
    &LION_SASH,
    &MIRRORSHELL_CRAB,
    &MOON_CIRCUIT_HACKER,
    &CLAWING_TORMENT,
    &OKIBA_RECKONER_RAID,
    &IRONHOOF_BOAR,
    &RABBIT_BATTERY,
    &GREATER_TANUKI,
    &TAMIYO_S_SAFEKEEPING,
    &TAMIYO_COMPLEATED_SAGE,
    &IRON_APPRENTICE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
