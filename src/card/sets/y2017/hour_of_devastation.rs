//! HOU card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

// HOU 48 — Striped Riverwinder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRIPED_RIVERWINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbeef9ef-487c-400b-bcee-1c0e8ec94b6a"),
    "Striped Riverwinder",
    crate::card::CardArt::new("bbeef9ef-487c-400b-bcee-1c0e8ec94b6a", "Craig J Spearing"),
    crate::card::CardSet::HourOfDevastation,
    crate::card::CardRules::unsupported(),
);

// HOU 83 — Abrade
pub(in crate::card::sets) static ABRADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84319dfb-eaf7-4b98-8c4f-30f5e779591b"),
    "Abrade",
    CardArt::new("84319dfb-eaf7-4b98-8c4f-30f5e779591b", "Jonas De Ro"),
    CardSet::HourOfDevastation,
    // Two mana that is never dead: the half a red deck wants is whichever
    // one the board is holding.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        // One of two, chosen as it is cast: each half carries its own slot, so a
        // board with neither a creature nor an artifact leaves nothing to cast it
        // at.
        &[
            AbilityDef::spell_with_targets(
                "Abrade deals 3 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
        ],
    )),
);

// HOU 92 — Firebrand Archer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIREBRAND_ARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ddc6b73-298b-4afa-990a-63706e77dd9f"),
    "Firebrand Archer",
    crate::card::CardArt::new("6ddc6b73-298b-4afa-990a-63706e77dd9f", "John Stanko"),
    crate::card::CardSet::HourOfDevastation,
    crate::card::CardRules::unsupported(),
);

// HOU 138 — Bloodwater Entity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODWATER_ENTITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("474d0a04-b640-4d1d-b538-2d946c1ff913"),
    "Bloodwater Entity",
    crate::card::CardArt::new("474d0a04-b640-4d1d-b538-2d946c1ff913", "Viktor Titov"),
    crate::card::CardSet::HourOfDevastation,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &STRIPED_RIVERWINDER,
    &ABRADE,
    &FIREBRAND_ARCHER,
    &BLOODWATER_ENTITY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
