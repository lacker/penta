//! Portal Three Kingdoms card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::{AbilityDef, CardRules, CardSet, EffectDef};
use crate::mana_cost;

// PTK 78 — Imperial Seal
pub(in crate::card::sets) static IMPERIAL_SEAL: CardRecord = CardRecord::new(
    CardSet::PortalThreeKingdoms,
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
