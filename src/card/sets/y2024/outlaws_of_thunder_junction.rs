//! Outlaws of Thunder Junction cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CounterKind, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    abilities,
};
use crate::{TargetIndex, mana_cost};

static BILL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static BILL_DOUBLE_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{3}{G}{G}"))];

static BILL_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered_with_targets(
        "Landfall — Whenever a land you control enters, put a +1/+1 counter on target creature.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        &BILL_TARGET,
        EffectDef::AddCounters {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
    // Each creature doubles its own, so a board of one-counter creatures
    // gains one apiece and a single large one gains everything it has.
    AbilityDef::activated(
        "{3}{G}{G}: Double the number of +1/+1 counters on each creature you control.",
        &BILL_DOUBLE_COST,
        EffectDef::DoubleCounters {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            kind: CounterKind::PlusOnePlusOne,
        },
    ),
];

/// "Draw a card. If you do, discard a card." A draw from an empty library
/// does not happen, so the discard is conditional on the draw rather than
/// sequenced after it.
static DUELIST_DRAWS_THEN_DISCARDS: EffectDef = EffectDef::Sequence(&[
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
]);

static DUELIST_ABILITIES: [AbilityDef; 4] = [
    abilities::flying(),
    abilities::vigilance(),
    AbilityDef::static_ability(
        "Duelist of the Mind's power is equal to the number of cards you've drawn this turn.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            // The printed power is zero, so the counted part is the whole of
            // it; the toughness the card prints is left alone.
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                ValueDef::Constant(0),
            ),
        },
    )
    .with_coverage(AbilityCoverageDef::partial(
        "A characteristic-defining ability sets power in every zone. This is a battlefield-only \
         continuous effect, so the value is right wherever the card is played and absent for \
         anything reading it in another zone.",
    )),
    AbilityDef::triggered(
        "Whenever you commit a crime, you may draw a card. If you do, discard a card. This ability triggers only once each turn.",
        TriggerEventDef::CommittedCrime(PlayerRelation::You),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &DUELIST_DRAWS_THEN_DISCARDS,
        },
    )
    .triggering_at_most(1),
];

// OTJ 45 — Duelist of the Mind
pub(in crate::card::sets) static DUELIST_OF_THE_MIND: CardRecord = CardRecord::new_with_legacy_id(
    2200,
    "Duelist of the Mind",
    CardArt::new("2b58e47b-c165-4a58-aa2a-033a35645adc", "Darren Tan"),
    CardSet::OutlawsOfThunderJunction,
    // A 0/3 flier that grows with every draw and feeds itself once a turn,
    // provided you point something at your opponent.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Advisor"], 0, 3)
        .with_abilities(&DUELIST_ABILITIES),
);

// OTJ 157 — Bristly Bill, Spine Sower
pub(in crate::card::sets) static BRISTLY_BILL_SPINE_SOWER: CardRecord =
    CardRecord::new_with_legacy_id(
        2177,
        "Bristly Bill, Spine Sower",
        CardArt::new("52eef0d6-24b7-40b7-8403-e8e863d0cd55", "Daniel Zrom"),
        CardSet::OutlawsOfThunderJunction,
        // The counters accumulate for free off lands, and then the activation
        // turns a slow board into a lethal one in a single turn.
        CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Druid"], 2, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&BILL_ABILITIES),
    );

/// Ward reads as one clause on the Boots, so the granted ability carries the
/// whole of the printed reminder rather than a paraphrase of it.
static LAVASPUR_WARD: AbilityDef = abilities::ward(
    1,
    "Ward {1} (Whenever this creature becomes the target of a spell or ability an opponent \
     controls, counter it unless that player pays {1}.)",
);

// OTJ 243 — Lavaspur Boots
pub(in crate::card::sets) static LAVASPUR_BOOTS: CardRecord = CardRecord::new_with_legacy_id(
    2252,
    "Lavaspur Boots",
    CardArt::new("e50709de-e6ef-4dbc-af1e-290fed279f34", "Mila Pesic"),
    CardSet::OutlawsOfThunderJunction,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and has haste and ward {1}.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        AppliedEffectDef::add_ability(&LAVASPUR_WARD),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DUELIST_OF_THE_MIND,
    &BRISTLY_BILL_SPINE_SOWER,
    &LAVASPUR_BOOTS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
