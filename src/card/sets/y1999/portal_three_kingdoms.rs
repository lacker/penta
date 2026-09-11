//! Portal Three Kingdoms card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::EffectDef;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "PTK",
    slug: "portal-three-kingdoms",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// PTK 78 — Imperial Seal
pub(in crate::card::sets) static IMPERIAL_SEAL: CardRecord = CardRecord::new(
    "Imperial Seal",
    "822e30db-40c5-4099-868b-185ad9b7c7dc",
    "Li Tie",
    // Vampiric Tutor's clause at sorcery speed, which is the whole of the
    // difference: the card you want is on top of your library, and you wait
    // a turn to draw it.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, then shuffle and put that card on top. You lose 2 life.",
        EffectDef::Sequence(&catalog_vis::VAMPIRIC_TUTOR_EFFECT),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&IMPERIAL_SEAL];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
