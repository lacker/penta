//! Phyrexia: All Will Be One Commander cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, EffectDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::mana_cost;

// ONC 6 — Glimmer Lens
pub(in crate::card::sets) static GLIMMER_LENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9262000-e6f3-4da1-ad1c-038f65d3bef6"),
    "Glimmer Lens",
    CardArt::new(
        "c9262000-e6f3-4da1-ad1c-038f65d3bef6",
        "Sidharth Chaturvedi",
    ),
    CardSet::PhyrexiaAllWillBeOneCommander,
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::for_mirrodin(),
            // "And at least one other creature" is the whole declaration
            // being two or more: the Rebel it brought is one attacker, so
            // the card asks for a second body before it draws.
            AbilityDef::triggered(
                "Whenever equipped creature and at least one other creature attack, draw a card.",
                TriggerEventDef::attacks_in_declaration(
                    ObjectPredicateDef::AttachedToSource,
                    2,
                    None,
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
                "Equip {1}{W}",
            ),
        ]),
);

// ONC 39 — Otharri, Suns\' Glory
/// The counter goes on the player, not on him: it stays through his death
/// and counts for whatever he comes back to.
static OTHARRI_EXPERIENCE: ValueDef = ValueDef::PlayerCounters {
    player: PlayerRelation::You,
    kind: CounterKind::Experience,
};

/// "Then" is the order that matters: the counter is his first, so the
/// attack he arrives on already makes one Rebel.
static OTHARRI_ATTACK: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Controller,
        kind: CounterKind::Experience,
        amount: ValueDef::Constant(1),
    },
    EffectDef::create_creature_token(&["Rebel"], &[ManaColor::Red], 2, 2)
        .entering_tapped()
        .entering_attacking()
        .with_count(OTHARRI_EXPERIENCE),
];

static OTHARRI_RETURN_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{2}{R}{W}")),
    AbilityCostDef::TapPermanent {
        object: ObjectPredicateDef::Subtype("Rebel"),
        controller: PlayerRelation::You,
    },
];

static OTHARRI_ABILITIES: [AbilityDef; 5] = [
    abilities::flying(),
    abilities::lifelink(),
    abilities::haste(),
    AbilityDef::triggered(
        "Whenever this creature attacks, you get an experience counter. Then create a 2/2 red \
         Rebel creature token that\'s tapped and attacking for each experience counter you have.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::Sequence(&OTHARRI_ATTACK),
    ),
    AbilityDef::activated(
        "{2}{R}{W}, Tap an untapped Rebel you control: Return this card from your graveyard to \
         the battlefield tapped.",
        &OTHARRI_RETURN_COST,
        EffectDef::WithBattlefieldArrival {
            effect: &EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
            arrival: crate::card::BattlefieldArrivalDef {
                modifications: &[BattlefieldEntryModificationDef::Tapped],
                ..crate::card::BattlefieldArrivalDef::DEFAULT
            },
        },
    )
    .with_source_zones(&[ZoneKind::Graveyard]),
];

pub(in crate::card::sets) static OTHARRI_SUNS_GLORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("348e0927-1d8f-4723-879d-f7e95ac60c27"),
    "Otharri, Suns\' Glory",
    CardArt::new("348e0927-1d8f-4723-879d-f7e95ac60c27", "Marta Nael"),
    CardSet::PhyrexiaAllWillBeOneCommander,
    // Five mana for a hasty lifelinking flier that pays out more every time
    // it connects, and buys itself back out of the graveyard with what it
    // left behind.
    CardRules::new_creature(mana_cost!("{3}{R}{W}"), &["Phoenix"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&OTHARRI_ABILITIES),
);

// ONC 48 — Staff of the Storyteller
static STAFF_SPIRIT: [AbilityDef; 1] = [abilities::flying()];

/// The Staff pays for itself the moment it lands: the Spirit it makes is a
/// creature token you created, so its own trigger sees it.
static STAFF_MAKES_A_SPIRIT: EffectDef =
    EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
        .with_abilities(&STAFF_SPIRIT);

/// One instruction, one counter, however many tokens it made -- which is
/// what "one or more" says and what makes a wide token maker no better here
/// than a narrow one.
static STAFF_COUNTS_THE_STORY: TriggerEventDef = TriggerEventDef::TokensCreated {
    player: PlayerRelation::You,
    token: ObjectPredicateDef::HasType(CardType::Creature),
};

static STAFF_DRAW_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{W}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::RemoveCountersFromSource {
        kind: CounterKind::Story,
        amount: 1,
    },
];

static STAFF_ABILITIES: [AbilityDef; 3] = [
    abilities::enters_trigger(
        "When this artifact enters, create a 1/1 white Spirit creature token with flying.",
        STAFF_MAKES_A_SPIRIT,
    ),
    AbilityDef::triggered(
        "Whenever you create one or more creature tokens, put a story counter on this artifact.",
        STAFF_COUNTS_THE_STORY,
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::Story,
            amount: ValueDef::Constant(1),
        },
    ),
    AbilityDef::activated(
        "{W}, {T}, Remove a story counter from this artifact: Draw a card.",
        &STAFF_DRAW_COST,
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
];

pub(in crate::card::sets) static STAFF_OF_THE_STORYTELLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17be11f2-f2db-40c4-8fc1-2ed7173f9a1a"),
    "Staff of the Storyteller",
    CardArt::new("17be11f2-f2db-40c4-8fc1-2ed7173f9a1a", "Dan Murayama Scott"),
    CardSet::PhyrexiaAllWillBeOneCommander,
    // Two mana for a flier, and a card for every turn the deck keeps making
    // tokens afterwards.
    CardRules::new_artifact(mana_cost!("{1}{W}")).with_abilities(&STAFF_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GLIMMER_LENS,
    &OTHARRI_SUNS_GLORY,
    &STAFF_OF_THE_STORYTELLER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
