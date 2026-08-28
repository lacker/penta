//! RNA card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, EffectDef, InstalledTriggerDef, PlayerRelation,
    TriggerEventDef, TurnStepDef, ValueDef, abilities,
};
use crate::mana_cost;

static SPHINX_OPENING_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your first upkeep, scry 3.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    abilities::scry(ValueDef::Constant(3)),
);

// RNA 55 — Sphinx of Foresight
pub(in crate::card::sets) static SPHINX_OF_FORESIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf2386fd-edc0-4731-8f4e-7a7c45548bf3"),
    "Sphinx of Foresight",
    CardArt::new("cf2386fd-edc0-4731-8f4e-7a7c45548bf3", "Titus Lunter"),
    CardSet::RavnicaAllegiance,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Sphinx"], 4, 4).with_abilities(&[
        AbilityDef::opening_hand_reveal(
            "You may reveal this card from your opening hand. If you do, scry 3 at the beginning of your first upkeep.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&SPHINX_OPENING_TRIGGER)),
        ),
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, scry 1.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            abilities::scry(ValueDef::Constant(1)),
        ),
    ]),
);

// RNA 115 — Skewer the Critics
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKEWER_THE_CRITICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97295660-6bea-46ae-9a3b-0fc6abba407f"),
    "Skewer the Critics",
    crate::card::CardArt::new("97295660-6bea-46ae-9a3b-0fc6abba407f", "Heonhwa"),
    crate::card::CardSet::RavnicaAllegiance,
    crate::card::CardRules::unsupported(),
);

// RNA 172 — Fireblade Artist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIREBLADE_ARTIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21e1161f-bd2c-45a7-a86b-3b2e5210f148"),
    "Fireblade Artist",
    crate::card::CardArt::new("21e1161f-bd2c-45a7-a86b-3b2e5210f148", "Steve Argyle"),
    crate::card::CardSet::RavnicaAllegiance,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SPHINX_OF_FORESIGHT, &SKEWER_THE_CRITICS, &FIREBLADE_ARTIST];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
