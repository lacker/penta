//! Torment cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules,
    CardSet, EffectDef, EffectRecipientDef, ObjectPredicateDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// TOR 100 — Grim Lavamancer
pub(in crate::card::sets) static GRIM_LAVAMANCER: CardRecord = CardRecord::new_with_legacy_id(
    2036,
    "Grim Lavamancer",
    CardArt::new("5dd72697-24be-42c7-a6d9-a837bdbd4662", "Jim Nelson"),
    CardSet::Torment,
    // The graveyard is the limit: two cards a turn is the rate, and a deck
    // that empties its hand quickly is the one that can pay it.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}, Exile two cards from your graveyard: This creature deals 2 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::ExileCardsFromGraveyard {
                    object: ObjectPredicateDef::Any,
                    count: 2,
                },
            ],
            &ANY_TARGET,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&GRIM_LAVAMANCER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
