//! Planechase 2012 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{CardRules, CardSet, EffectDef, EffectRecipientDef, ValueDef, abilities};
use crate::mana_cost;

// PC2 40 — Beetleback Chief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEETLEBACK_CHIEF: CardRecord = CardRecord::new(
    crate::card::CardSet::Planechase2012,
    "Beetleback Chief",
    "1e3ccf3d-583c-46b4-b51e-ae1b0628d506",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// PC2 82 — Baleful Strix
pub(in crate::card::sets) static BALEFUL_STRIX: CardRecord = CardRecord::new(
    CardSet::Planechase2012,
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
