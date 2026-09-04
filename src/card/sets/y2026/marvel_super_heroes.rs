//! Marvel Super Heroes cards cataloged for opening-hand rules coverage.

use super::{CardRecord, PrintingRecord};
use crate::card::{CardRules, CardSet};

// MSH 148 — Quicksilver, Brash Blur
// Audit: unsupported — Needs the Power-up once-per-object limit and entered-this-turn cost reduction.
pub(in crate::card::sets) static QUICKSILVER_BRASH_BLUR: CardRecord = CardRecord::new(
    CardSet::MarvelSuperHeroes,
    "Quicksilver, Brash Blur",
    "2d5819ca-165d-4f4c-9500-3ac206994880",
    "Michael MacRae",
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&QUICKSILVER_BRASH_BLUR];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
