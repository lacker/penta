//! New Phyrexia cards used to exercise Phyrexian mana.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

static GUT_SHOT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// NPH 86 — Gut Shot
pub(in crate::card::sets) static GUT_SHOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a54a2a30-b96a-49c7-9151-1f4b0d4a4413"),
    "Gut Shot",
    CardArt::new("a54a2a30-b96a-49c7-9151-1f4b0d4a4413", "Greg Staples"),
    CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{R/P}")).with_ability(AbilityDef::spell_with_targets(
        "Gut Shot deals 1 damage to any target.",
        &GUT_SHOT_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&GUT_SHOT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
