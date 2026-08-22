//! Eternal Masters cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, LikelihoodDef, ManaColor, PlayerRelation, TriggerEventDef, TurnStepDef,
    ValueDef,
};
use crate::mana_cost;

/// Losing the flip is the whole cost of the card, and it is paid to the
/// artifact itself: three damage from a source its controller chose to keep
/// around.
static MANA_CRYPT_TOLL: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(3),
};

static MANA_CRYPT_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered(
        "At the beginning of your upkeep, flip a coin. If you lose the flip, this artifact deals 3 damage to you.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::Randomized {
            likelihood: LikelihoodDef::new(0.5),
            on_success: &EffectDef::None,
            on_failure: &MANA_CRYPT_TOLL,
        },
    ),
    AbilityDef::activated_mana(
        "{T}: Add {C}{C}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
    ),
];

// EMA 225 — Mana Crypt
pub(in crate::card::sets) static MANA_CRYPT: CardRecord = CardRecord::new_with_legacy_id(
    2142,
    "Mana Crypt",
    CardArt::new("0cb33b46-4d1b-4f97-bfdc-d815aee111da", "Matt Stewart"),
    CardSet::EternalMasters,
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&MANA_CRYPT_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MANA_CRYPT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
