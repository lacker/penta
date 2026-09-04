//! Journey into Nyx cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{AbilityCostDef, AbilityDef, AddManaEffectDef, CardRules, CardSet, EffectDef};

// JOU 163 — Mana Confluence
pub(in crate::card::sets) static MANA_CONFLUENCE: CardRecord = CardRecord::new(
    CardSet::JourneyIntoNyx,
    "Mana Confluence",
    "504a69eb-3c2d-4bb1-b117-252b15acf0c2",
    "Richard Wright",
    // City of Brass charges its life when it becomes tapped, by anyone and
    // for any reason. This charges it as a cost of its own ability, so a land
    // tapped by someone else costs nothing and an activation with no life to
    // spare is simply not offered.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add one mana of any color.",
        &[AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MANA_CONFLUENCE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
