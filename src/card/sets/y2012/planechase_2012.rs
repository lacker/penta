//! Planechase 2012 cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

// PC2 40 — Beetleback Chief
pub(in crate::card::sets) static BEETLEBACK_CHIEF: CardRecord = CardRecord::new(
    "Beetleback Chief",
    "1e3ccf3d-583c-46b4-b51e-ae1b0628d506",
    "Wayne England",
    // Four power across three bodies for four mana: the Chief is a sacrifice
    // outlet's worth of goblins rather than one creature.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Goblin", "Warrior"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, create two 1/1 red Goblin creature tokens.",
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_amount(2),
        ),
    ),
);

// PC2 82 — Baleful Strix
pub(in crate::card::sets) static BALEFUL_STRIX: CardRecord = CardRecord::new(
    "Baleful Strix",
    "62090c97-7e3e-4854-bc44-c4a900133ec5",
    "Nils Hamm",
    // Two mana that replaces itself and then eats whatever attacks into it,
    // however large. Nothing about the body matters except that it blocks.
    CardRules::new_artifact_creature(mana_cost!("{U}{B}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::deathtouch(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BEETLEBACK_CHIEF, &BALEFUL_STRIX];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
