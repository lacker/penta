//! Catalog records for synthetic presentations that are not independently
//! created game objects.
//!
//! Created tokens and emblems carry characteristics owned by their creating
//! abilities and therefore do not appear here. The face-down presentation is
//! selected independently by the engine, so it retains a catalog definition.

use super::{CardRecord, PrintingRecord};
use crate::card::{CardArt, CardRules, CardSet, cards};

/// Not a token: the body a face-down permanent presents while it is face
/// down. Unlike a created token, this presentation is selected independently
/// of any creating ability, so it retains a catalog definition no format
/// allows. Nothing may ever put a card of it into a deck. A face-down
/// permanent's own definition stays the card underneath, so it is never
/// treated as a token.
pub(in crate::card::sets) static FACE_DOWN_CREATURE: CardRecord = CardRecord::new(
    cards::FACE_DOWN_CREATURE,
    "Face-down creature",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&[], 2, 2),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FACE_DOWN_CREATURE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
