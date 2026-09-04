//! TLA card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AddManaEffectDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CostDef, EffectDef, EffectRecipientDef, ManaColor, ReplacementEffectDef, ValueDef,
};
use crate::mana_cost;

// TLA 144 — The Last Agni Kai
pub(in crate::card::sets) static THE_LAST_AGNI_KAI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61eaebc6-7575-48ed-b212-ff8b0c7ae694"),
    "The Last Agni Kai",
    CardArt::new("61eaebc6-7575-48ed-b212-ff8b0c7ae694", "Pablo Rivera"),
    CardSet::AvatarTheLastAirbender,
    // Audit: unsupported — Needs an effect-scoped mana-retention duration for only the excess mana it creates.
    CardRules::unsupported(),
);

/// The TLA cycle of tapped duals that cash themselves in: three lands that
/// differ only in which two colours they make, so the clauses are written
/// once here. `colors` is a promoted literal at each call site, since a
/// slice assembled inside this function could not be given a `'static`
/// lifetime.
const fn cashable_dual_land(mana_text: &'static str, colors: &'static [ManaColor]) -> CardRules {
    CardRules::new_land(&[])
        .with_ability(AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ))
        // Added one at a time and in printed order: an array holding the
        // parameterized mana ability could not be given a 'static lifetime.
        .with_ability(AbilityDef::activated_mana(
            mana_text,
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        .with_ability(AbilityDef::activated(
            "{4}, {T}, Sacrifice this land: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{4}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ))
}

// TLA 267 — Boiling Rock Prison
pub(in crate::card::sets) static BOILING_ROCK_PRISON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c2e2220-54d1-4180-93a0-964e3b0ba8b8"),
    "Boiling Rock Prison",
    CardArt::new("1c2e2220-54d1-4180-93a0-964e3b0ba8b8", "Matteo Bassini"),
    CardSet::AvatarTheLastAirbender,
    // Entering tapped is the price of the two colours; cashing it in later
    // is what keeps it from being a dead draw once the mana is there.
    cashable_dual_land("{T}: Add {B} or {R}.", &[ManaColor::Black, ManaColor::Red]),
);

// TLA 271 — Kyoshi Village
pub(in crate::card::sets) static KYOSHI_VILLAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d5f3008-2af8-4e81-8847-1c91f524e747"),
    "Kyoshi Village",
    CardArt::new("8d5f3008-2af8-4e81-8847-1c91f524e747", "Luc Courtois"),
    CardSet::AvatarTheLastAirbender,
    cashable_dual_land(
        "{T}: Add {G} or {W}.",
        &[ManaColor::Green, ManaColor::White],
    ),
);

// TLA 279 — Serpent's Pass
pub(in crate::card::sets) static SERPENT_S_PASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad87bff5-9b8c-44e4-a6d3-8cc71be9640a"),
    "Serpent's Pass",
    CardArt::new("ad87bff5-9b8c-44e4-a6d3-8cc71be9640a", "Matteo Bassini"),
    CardSet::AvatarTheLastAirbender,
    cashable_dual_land("{T}: Add {U} or {B}.", &[ManaColor::Blue, ManaColor::Black]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THE_LAST_AGNI_KAI,
    &BOILING_ROCK_PRISON,
    &KYOSHI_VILLAGE,
    &SERPENT_S_PASS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
