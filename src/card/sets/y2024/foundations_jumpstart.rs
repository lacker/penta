//! Foundations Jumpstart cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardSupertype, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    abilities, tokens,
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

// J25 19 — Scholar of Combustion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCHOLAR_OF_COMBUSTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23660e44-8546-438d-a2c4-e1cef6e50855"),
    "Scholar of Combustion",
    crate::card::CardArt::new("23660e44-8546-438d-a2c4-e1cef6e50855", "Nereida"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 28 — Shardless Outlander
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHARDLESS_OUTLANDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fccb51a4-cb78-4437-b9ab-cc77736af561"),
    "Shardless Outlander",
    crate::card::CardArt::new("fccb51a4-cb78-4437-b9ab-cc77736af561", "Leon Tukker"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 50 — Ivora, Insatiable Heir
pub(in crate::card::sets) static IVORA_INSATIABLE_HEIR: CardRecord = CardRecord::new_with_legacy_id(
    2148,
    "Ivora, Insatiable Heir",
    CardArt::new("2ba70366-b6ae-423a-a8d8-29d2b8afd939", "Canata Katana"),
    CardSet::FoundationsJumpstart,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire", "Warrior"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&IVORA_ABILITIES),
);

// J25 212 — Inspiring Overseer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRING_OVERSEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35d9da1d-8678-4252-b0f8-9960795642f0"),
    "Inspiring Overseer",
    crate::card::CardArt::new("be1c0c41-cd92-49b2-be07-0c44219bcb6a", "Irina Nordsol"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 343 — Pestermite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PESTERMITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f252ae53-443c-4a27-b8f0-639a9a2b8598"),
    "Pestermite",
    crate::card::CardArt::new(
        "4c8b4f64-244c-4944-b23f-c383039d9767",
        "Christopher Moeller",
    ),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 641 — Bushwhack
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BUSHWHACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("712a0640-d9c8-46fc-b38b-bf20a40fa902"),
    "Bushwhack",
    crate::card::CardArt::new("f6b92766-1ab8-462d-bd45-ccd6f55cbe14", "Artur Nakhodkin"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 684 — Llanowar Visionary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("880c9523-717e-4903-a09e-d6c47614383d"),
    "Llanowar Visionary",
    crate::card::CardArt::new("c2635b0c-c990-4cce-9ac4-97602a757cf0", "Cristi Balanescu"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 753 — Guardian Idol
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GUARDIAN_IDOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6a62a73-b7db-47ec-9b68-65dd7c1a06a5"),
    "Guardian Idol",
    crate::card::CardArt::new("1537f377-64c3-4c3b-a276-28d8234c029b", "Igor Kieryluk"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SCHOLAR_OF_COMBUSTION,
    &SHARDLESS_OUTLANDER,
    &IVORA_INSATIABLE_HEIR,
    &INSPIRING_OVERSEER,
    &PESTERMITE,
    &BUSHWHACK,
    &LLANOWAR_VISIONARY,
    &GUARDIAN_IDOL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
