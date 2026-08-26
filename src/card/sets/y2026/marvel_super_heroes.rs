//! Marvel Super Heroes cards cataloged for opening-hand rules coverage.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{AbilityDef, CardArt, CardRules, CardSet, CardSupertype, abilities};
use crate::mana_cost;

// MSH 148 — Quicksilver, Brash Blur
// Audit: partial — The opening-hand action and haste are declarative; power-up needs its once-per-object limit and entered-this-turn cost reduction.
pub(in crate::card::sets) static QUICKSILVER_BRASH_BLUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d5819ca-165d-4f4c-9500-3ac206994880"),
    "Quicksilver, Brash Blur",
    CardArt::new("2d5819ca-165d-4f4c-9500-3ac206994880", "Michael MacRae"),
    CardSet::MarvelSuperHeroes,
    CardRules::new_creature(mana_cost!("{R}"), &["Mutant", "Hero"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::begin_game_on_battlefield("If Quicksilver, Brash Blur is in your opening hand, you may begin the game with him on the battlefield."),
            abilities::haste(),
            AbilityDef::not_implemented("Power-up — {4}{R}: Put a +1/+1 counter and a double strike counter on Quicksilver. (Activate each power-up ability only once. Reduce the cost by his mana cost if he entered this turn.)", "Needs a once-per-object activation limit plus a cost reduction derived from the source's mana cost when it entered this turn."),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&QUICKSILVER_BRASH_BLUR];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
