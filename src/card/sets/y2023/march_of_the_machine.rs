//! March of the Machine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, CounterKind, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, TokenCountersDef, ValueDef, ZoneKind,
    ZonePlacement, cards, tokens,
};
use crate::ids::ObjectSetBindingIndex;
use crate::mana_cost;

/// Everyone's, which is what "all creatures" means.
static EVERY_CREATURE: ObjectQueryDef = ObjectQueryDef::new(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
);

/// The creatures are bound before they move, because "X, where X is the
/// number of creatures exiled this way" asks about a set the board no longer
/// holds by the time the token is made.
static SUNFALL_STEPS: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
    },
    // Incubate X. One token however large X is, and X of zero still makes
    // one: the keyword creates the token unconditionally.
    EffectDef::create_token(tokens::incubator())
        .with_art(CardArt::new(
            "2c5ed737-657b-43bf-b222-941da7579a4a",
            "Johann Bodin",
        ))
        .with_counters(TokenCountersDef {
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::BoundObjectCount(ObjectSetBindingIndex::PRIMARY),
        }),
];

static SUNFALL_EXILES_THEN_INCUBATES: EffectDef = EffectDef::BindMatching {
    objects: ObjectSetDef::Query(EVERY_CREATURE),
    binding: ObjectSetBindingIndex::PRIMARY,
    then: &EffectDef::Sequence(&SUNFALL_STEPS),
};

// MOM 40 — Sunfall
pub(in crate::card::sets) static SUNFALL: CardRecord = CardRecord::new(
    cards::SUNFALL,
    "Sunfall",
    CardArt::new(
        "32e29c7d-ed4b-4eff-b3c2-d99e5b63ef8d",
        "Kasia 'Kafis' Zielińska",
    ),
    CardSet::MarchOfTheMachine,
    // A wrath that exiles rather than destroys, and hands the caster the
    // biggest thing on the empty board it just made.
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::spell(
        "Exile all creatures. Incubate X, where X is the number of creatures exiled this way. \
         (Create an Incubator token with X +1/+1 counters on it and \"{2}: Transform this \
         token.\" It transforms into a 0/0 Phyrexian artifact creature.)",
        SUNFALL_EXILES_THEN_INCUBATES,
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SUNFALL];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
