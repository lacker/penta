//! Future Sight cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{CardArt, CardRules, CardSet, CardType, ManaColor, cards};

pub(in crate::card::sets) static DRYAD_ARBOR: CardRecord = CardRecord::new(
    cards::DRYAD_ARBOR,
    "Dryad Arbor",
    CardArt::new("8cee476d-42e1-4997-87af-73e18f542167", "Eric Fortune"),
    CardSet::FutureSight,
    CardRules::new_creature_without_mana_cost(&["Forest", "Dryad"], 1, 1)
        .with_type(CardType::Land)
        .printed_colors(&[ManaColor::Green]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DRYAD_ARBOR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
