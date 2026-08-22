//! Rise of the Eldrazi cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

static FLAME_SLASH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// ROE 145 — Flame Slash
pub(in crate::card::sets) static FLAME_SLASH: CardRecord = CardRecord::new_with_legacy_id(
    2184,
    "Flame Slash",
    CardArt::new("006d2bf1-20f7-4b09-8d98-8233d91682bd", "Raymond Swanland"),
    CardSet::RiseOfTheEldrazi,
    // One mana for four damage is the best rate in the format; the sorcery
    // speed is the whole price, and it cannot go upstairs.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Flame Slash deals 4 damage to target creature.",
        &FLAME_SLASH_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FLAME_SLASH];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
