//! Journey into Nyx cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::CardRules;
use crate::card::CostDef;
use crate::card::EffectDef;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "JOU",
    slug: "journey-into-nyx",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// JOU 163 — Mana Confluence
pub(in crate::card::sets) static MANA_CONFLUENCE: CardRecord = CardRecord::new(
    "Mana Confluence",
    "504a69eb-3c2d-4bb1-b117-252b15acf0c2",
    "Richard Wright",
    // City of Brass charges its life when it becomes tapped, by anyone and
    // for any reason. This charges it as a cost of its own ability, so a land
    // tapped by someone else costs nothing and an activation with no life to
    // spare is simply not offered.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add one mana of any color.",
        &[CostDef::TapSource, CostDef::PayLife(1)],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MANA_CONFLUENCE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
