//! Eternal Masters cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, LikelihoodDef, ManaColor, PlayerRelation, TriggerEventDef, TurnStepDef,
    ValueDef,
};
use crate::mana_cost;

// EMA 6 — Coalition Honor Guard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COALITION_HONOR_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5b7be3e-b4af-46d4-bcc6-b44c651f2012"),
    "Coalition Honor Guard",
    crate::card::CardArt::new("2c7c2b5c-634a-4d83-81bc-c6128e3ac339", "Eric Peterson"),
    crate::card::CardSet::EternalMasters,
    crate::card::CardRules::unsupported(),
);

// EMA 45 — Deep Analysis
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEEP_ANALYSIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01e3c2e9-d8df-4a7a-be86-7be8c6254fa2"),
    "Deep Analysis",
    crate::card::CardArt::new("821cc8b6-eb2e-4441-8d88-c54cb44ab024", "Jesper Ejsing"),
    crate::card::CardSet::EternalMasters,
    crate::card::CardRules::unsupported(),
);

// EMA 119 — Beetleback Chief
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BEETLEBACK_CHIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e3ccf3d-583c-46b4-b51e-ae1b0628d506"),
    "Beetleback Chief",
    crate::card::CardArt::new("779d4745-ff14-4c79-b2c8-8e273faf7375", "Wayne England"),
    crate::card::CardSet::EternalMasters,
    crate::card::CardRules::unsupported(),
);

// EMA 139 — Mogg War Marshal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_WAR_MARSHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b9e0bdb-b615-447a-b80d-d7244c25c56e"),
    "Mogg War Marshal",
    crate::card::CardArt::new("deed0a5a-6662-460c-bd78-e3d95e8bc83e", "Jesper Ejsing"),
    crate::card::CardSet::EternalMasters,
    crate::card::CardRules::unsupported(),
);

// EMA 191 — Werebear
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WEREBEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("964cf7e3-932d-432f-8ad4-9bd651aada96"),
    "Werebear",
    crate::card::CardArt::new("224ea635-b95b-4803-8716-edd4cb655923", "Filip Burburan"),
    crate::card::CardSet::EternalMasters,
    crate::card::CardRules::unsupported(),
);

// EMA 225 — Mana Crypt
pub(in crate::card::sets) static MANA_CRYPT: CardRecord = CardRecord::new_with_legacy_id(
    2142,
    "Mana Crypt",
    CardArt::new("0cb33b46-4d1b-4f97-bfdc-d815aee111da", "Matt Stewart"),
    CardSet::EternalMasters,
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, flip a coin. If you lose the flip, this artifact deals 3 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Randomized {
                likelihood: LikelihoodDef::new(0.5),
                on_success: &EffectDef::None,
                // Losing the flip is the whole cost of the card, and it is paid to the
                // artifact itself: three damage from a source its controller chose to keep
                // around.
                on_failure: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COALITION_HONOR_GUARD,
    &DEEP_ANALYSIS,
    &BEETLEBACK_CHIEF,
    &MOGG_WAR_MARSHAL,
    &WEREBEAR,
    &MANA_CRYPT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
