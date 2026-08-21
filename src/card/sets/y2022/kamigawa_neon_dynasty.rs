//! Kamigawa: Neon Dynasty attachment edge cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TokenCharacteristics,
    TriggerConditionDef, ValueDef, ZoneKind, ZonePlacement, abilities,
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
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
    },
];

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
        EffectDef::ReduceMatchingSpellCostBy {
            spell: ObjectPredicateDef::Any,
            caster: PlayerRelation::You,
            amount: ValueDef::Constant(2),
        },
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&LION_SASH, &RABBIT_BATTERY, &TAMIYO_COMPLEATED_SAGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
