//! Foundations Jumpstart cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardSupertype, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    abilities, cards, tokens,
};
use crate::mana_cost;

/// One printed ability with two ways in, which is what "when it enters and
/// whenever it deals combat damage" says. Splitting it would make her two
/// triggered abilities where the card has one.
static IVORA_MAKES_BLOOD: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
];

static IVORA_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::triggered(
        "When Ivora enters and whenever it deals combat damage to a player, create a Blood token.",
        TriggerEventDef::AnyOf(&IVORA_MAKES_BLOOD),
        EffectDef::create_token(tokens::blood()).with_art(CardArt::new(
            "6b563165-b97f-42c6-82a8-65d8ee69e381",
            "Stephen Andrade",
        )),
    ),
    // Any discard, including one paid as a cost -- which is how her own Blood
    // token feeds her.
    AbilityDef::triggered(
        "Whenever you discard a card, put a +1/+1 counter on Ivora.",
        TriggerEventDef::Discarded(PlayerRelation::You),
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
];

// J25 50 — Ivora, Insatiable Heir
pub(in crate::card::sets) static IVORA_INSATIABLE_HEIR: CardRecord = CardRecord::new(
    cards::IVORA_INSATIABLE_HEIR,
    "Ivora, Insatiable Heir",
    CardArt::new("2ba70366-b6ae-423a-a8d8-29d2b8afd939", "Canata Katana"),
    CardSet::FoundationsJumpstart,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire", "Warrior"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&IVORA_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&IVORA_INSATIABLE_HEIR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
