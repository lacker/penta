//! Shadowmoor cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AddManaEffectDef, CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef,
    ManaColor, ValueDef,
};
use crate::mana_cost;

static EVERY_COLOR: [ManaColor; 5] = [
    ManaColor::White,
    ManaColor::Blue,
    ManaColor::Black,
    ManaColor::Red,
    ManaColor::Green,
];

/// "In any combination of colors" is one question per mana rather than one
/// for the pair, which is what lets it fix two colours at once.
static MANAMORPHOSE_EFFECT: [EffectDef; 2] = [
    EffectDef::AddMana(AddManaEffectDef::combination(&EVERY_COLOR, 2)),
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

// SHM 211 — Manamorphose
pub(in crate::card::sets) static MANAMORPHOSE: CardRecord = CardRecord::new_with_legacy_id(
    2238,
    "Manamorphose",
    CardArt::new("50283122-b8c4-4fb3-8eba-6252b72222f4", "Jeff Miracola"),
    CardSet::Shadowmoor,
    // It costs nothing and does nothing, which is the point: the deck that
    // wants it wants a spell that replaces itself and moves the storm count.
    CardRules::new_instant(mana_cost!("{1}{R/G}")).with_ability(AbilityDef::spell(
        "Add two mana in any combination of colors.\nDraw a card.",
        EffectDef::Sequence(&MANAMORPHOSE_EFFECT),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MANAMORPHOSE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
