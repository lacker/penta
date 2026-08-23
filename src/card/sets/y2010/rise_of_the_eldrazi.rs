//! Rise of the Eldrazi cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

static FLAME_SLASH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// ROE 13 — Ulamog's Crusher
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ULAMOG_S_CRUSHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76bacedb-9fa8-4a21-b0eb-e7ead64360b4"),
    "Ulamog's Crusher",
    crate::card::CardArt::new("76bacedb-9fa8-4a21-b0eb-e7ead64360b4", "Todd Lockwood"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 130 — Vendetta
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VENDETTA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67ced38e-0f33-4bda-8e18-09f6ac03a3d7"),
    "Vendetta",
    crate::card::CardArt::new("039fc76d-3b7e-4329-a997-07c25509e421", "Karl Kopinski"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

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

// ROE 161 — Raid Bombardment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAID_BOMBARDMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c2d1a48-efde-4134-95f0-b23f6cf85259"),
    "Raid Bombardment",
    crate::card::CardArt::new("9c2d1a48-efde-4134-95f0-b23f6cf85259", "Matt Cavotta"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 201 — Nest Invader
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NEST_INVADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24517d9c-6cde-41e8-9e82-ee73f069379a"),
    "Nest Invader",
    crate::card::CardArt::new("24517d9c-6cde-41e8-9e82-ee73f069379a", "Trevor Claxton"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ULAMOG_S_CRUSHER,
    &VENDETTA,
    &FLAME_SLASH,
    &RAID_BOMBARDMENT,
    &NEST_INVADER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
