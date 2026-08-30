//! Outlaws of Thunder Junction cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, DiscardSelectionDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectQueryDef, PlayerRelation, ResolvedEffectDurationDef, ScaledValueDef,
    TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::{TargetIndex, mana_cost};

// OTJ 45 — Duelist of the Mind
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
            // The count defines her power outright, which is why it also
            // answers in a hand or a graveyard; the printed toughness is
            // left alone.
            effect: AppliedEffectDef::define_power(ValueDef::CardsDrawnThisTurn(
                PlayerRelation::You,
            )),
        },
    ),
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

// OTJ 61 — Phantom Interference
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_INTERFERENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00bf4dd1-5468-4594-9c7b-0737610f19d4"),
    "Phantom Interference",
    crate::card::CardArt::new("00bf4dd1-5468-4594-9c7b-0737610f19d4", "Ruxing Gao"),
    crate::card::CardSet::OutlawsOfThunderJunction,
    crate::card::CardRules::unsupported(),
);

// OTJ 82 — Caustic Bronco
/// "You lose life equal to that card's mana value if this creature isn't
/// saddled. Otherwise, each opponent loses that much life." Two clauses
/// reading one number, which is the mana value of the card the reveal just
/// put in hand -- gone from anywhere the resolution could look at it by the
/// time either half asks.
static BRONCO_IS_SADDLED: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Saddled,
};

static BRONCO_IS_NOT_SADDLED: TriggerConditionDef = TriggerConditionDef::Not(&BRONCO_IS_SADDLED);

static BRONCO_DRAIN: EffectDef = EffectDef::LoseLife {
    recipient: EffectRecipientDef::Opponent,
    amount: ValueDef::MatchedManaValue,
};

static BRONCO_KICK: EffectDef = EffectDef::LoseLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::MatchedManaValue,
};

static BRONCO_PAYMENT: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &BRONCO_IS_NOT_SADDLED,
        then: &BRONCO_KICK,
    },
    EffectDef::IfCondition {
        condition: &BRONCO_IS_SADDLED,
        then: &BRONCO_DRAIN,
    },
];

static BRONCO_PAYMENT_SEQUENCE: EffectDef = EffectDef::Sequence(&BRONCO_PAYMENT);

/// The reveal itself: one card off the top, shown to everybody, into your
/// hand, and then the clause above reads what it cost.
static BRONCO_REVEAL: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: true,
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
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: Some(&BRONCO_PAYMENT_SEQUENCE),
};

pub(in crate::card::sets) static CAUSTIC_BRONCO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9a268ba-c442-4fe4-90b4-2810c8474f4e"),
    "Caustic Bronco",
    CardArt::new("e9a268ba-c442-4fe4-90b4-2810c8474f4e", "Brent Hollowell"),
    CardSet::OutlawsOfThunderJunction,
    // Two mana for a 2/2 that draws you an extra card every attack. Whether
    // that card costs you or them is what the saddle buys.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Snake", "Horse", "Mount"], 2, 2)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever this creature attacks, reveal the top card of your library and put it \
                 into your hand. You lose life equal to that card's mana value if this creature \
                 isn't saddled. Otherwise, each opponent loses that much life.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::LookAtTopAndSelect {
                    player: EffectRecipientDef::Controller,
                    looker: EffectRecipientDef::Controller,
                    selection: &BRONCO_REVEAL,
                },
            ),
            abilities::saddle(
                3,
                "Saddle 3 (Tap any number of other creatures you control with total power 3 or \
                 more: This Mount becomes saddled until end of turn. Saddle only as a sorcery.)",
            ),
        ]),
);

// OTJ 157 — Bristly Bill, Spine Sower
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

// OTJ 188 — Voracious Varmint
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VORACIOUS_VARMINT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99b74fa3-c1d7-4780-977d-f2d6663a529a"),
    "Voracious Varmint",
    crate::card::CardArt::new(
        "99b74fa3-c1d7-4780-977d-f2d6663a529a",
        "Adrián Rodríguez Pérez",
    ),
    crate::card::CardSet::OutlawsOfThunderJunction,
    crate::card::CardRules::unsupported(),
);

// OTJ 224 — Pillage the Bog
/// "Twice the number of lands you control", which is what makes the card a
/// land-count payoff rather than a fixed dig: six lands look at twelve.
static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static TWICE_YOUR_LANDS: ScaledValueDef =
    ScaledValueDef::new(ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL), 2);

/// One card of however many were seen, and the rest go to the bottom in a
/// random order: the looker has seen them, so the order they return in is
/// the game's to decide rather than the order they came out.
static PILLAGE_THE_BOG_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Scaled(&TWICE_YOUR_LANDS),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: false,
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

pub(in crate::card::sets) static PILLAGE_THE_BOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa3b415f-7901-4ab4-84fe-60b90d40ac90"),
    "Pillage the Bog",
    CardArt::new("fa3b415f-7901-4ab4-84fe-60b90d40ac90", "Forrest Imel"),
    CardSet::OutlawsOfThunderJunction,
    // Two mana to find the one card the deck is built around, and plot is
    // what makes the two mana free: pay three on a turn with nothing to do,
    // and dig for nothing on the turn it matters.
    CardRules::new_sorcery(mana_cost!("{B}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top X cards of your library, where X is twice the number of lands you \
             control. Put one of them into your hand and the rest on the bottom of your library \
             in a random order.",
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &PILLAGE_THE_BOG_LOOK,
            },
        ),
        abilities::plot(mana_cost!("{1}{B}{G}")),
    ]),
);

// OTJ 243 — Lavaspur Boots
/// Ward reads as one clause on the Boots, so the granted ability carries the
/// whole of the printed reminder rather than a paraphrase of it.
static LAVASPUR_WARD: AbilityDef = abilities::ward(
    1,
    "Ward {1} (Whenever this creature becomes the target of a spell or ability an opponent \
     controls, counter it unless that player pays {1}.)",
);

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

// OTJ 251 — Abraded Bluffs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABRADED_BLUFFS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19e96521-b4ce-4a36-a887-200e05ccc804"),
    "Abraded Bluffs",
    crate::card::CardArt::new("19e96521-b4ce-4a36-a887-200e05ccc804", "Piotr Dura"),
    crate::card::CardSet::OutlawsOfThunderJunction,
    crate::card::CardRules::unsupported(),
);

// OTJ 253 — Bristling Backwoods
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRISTLING_BACKWOODS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d61dfeb7-7f6b-4601-8396-2cbb98165489"),
    "Bristling Backwoods",
    crate::card::CardArt::new("d61dfeb7-7f6b-4601-8396-2cbb98165489", "Viko Menezes"),
    crate::card::CardSet::OutlawsOfThunderJunction,
    crate::card::CardRules::unsupported(),
);

// OTJ 254 — Conduit Pylons
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONDUIT_PYLONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ffa48cc-b991-4d47-b7ec-cf678915c758"),
    "Conduit Pylons",
    crate::card::CardArt::new("5ffa48cc-b991-4d47-b7ec-cf678915c758", "Raymond Bonilla"),
    crate::card::CardSet::OutlawsOfThunderJunction,
    crate::card::CardRules::unsupported(),
);

// OTJ 256 — Eroded Canyon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERODED_CANYON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c9d080f-28d7-41d6-a4e0-5b3e3a5ed770"),
    "Eroded Canyon",
    crate::card::CardArt::new("5c9d080f-28d7-41d6-a4e0-5b3e3a5ed770", "Piotr Dura"),
    crate::card::CardSet::OutlawsOfThunderJunction,
    crate::card::CardRules::unsupported(),
);

// OTJ 335 — Slickshot Show-Off
/// A noncreature spell you cast, which is prowess with a bigger number and
/// no toughness: what the Bird wants is one turn with several spells in it.
static A_NONCREATURE_SPELL_YOU_CAST_SLICKSHOT: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static SLICKSHOT_ABILITIES: [AbilityDef; 4] = [
    abilities::flying(),
    abilities::haste(),
    AbilityDef::triggered(
        "Whenever you cast a noncreature spell, this creature gets +2/+0 until end of turn.",
        TriggerEventDef::spell_cast(A_NONCREATURE_SPELL_YOU_CAST_SLICKSHOT),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
    abilities::plot(mana_cost!("{1}{R}")),
];

pub(in crate::card::sets) static SLICKSHOT_SHOW_OFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("304523e7-f332-4c1d-9590-ff9a70daff26"),
    "Slickshot Show-Off",
    CardArt::new("304523e7-f332-4c1d-9590-ff9a70daff26", "Augusto Quirino"),
    CardSet::OutlawsOfThunderJunction,
    // Two mana for a hasty flier that grows with every spell after it, and
    // a plot cost that pays the two a turn early so the whole of a later
    // turn's mana can go into the spells it grows on.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Bird", "Wizard"], 1, 2)
        .with_abilities(&SLICKSHOT_ABILITIES),
);

// OTJ 359 — Pillage the Bog (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DUELIST_OF_THE_MIND,
    &PHANTOM_INTERFERENCE,
    &CAUSTIC_BRONCO,
    &BRISTLY_BILL_SPINE_SOWER,
    &VORACIOUS_VARMINT,
    &PILLAGE_THE_BOG,
    &LAVASPUR_BOOTS,
    &ABRADED_BLUFFS,
    &BRISTLING_BACKWOODS,
    &CONDUIT_PYLONS,
    &ERODED_CANYON,
    &SLICKSHOT_SHOW_OFF,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&PILLAGE_THE_BOG, 1), // OTJ 359
];
