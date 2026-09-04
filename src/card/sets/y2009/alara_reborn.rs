//! ARB card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, PlayerRelation, ValueDef, abilities,
};
use crate::mana_cost;

// ARB 29 — Soul Manipulation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_MANIPULATION: CardRecord = CardRecord::new(
    crate::card::CardSet::AlaraReborn,
    "Soul Manipulation",
    "bcd3cb05-c6f9-435a-a0e7-1f85da4a36eb",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ARB 95 — Putrid Leech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUTRID_LEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::AlaraReborn,
    "Putrid Leech",
    "aaa47568-5668-4a9f-ad1c-9a13010ffc2b",
    "Dave Allsop",
    crate::card::CardRules::unsupported(),
);

// ARB 133 — Thopter Foundry
pub(in crate::card::sets) static THOPTER_FOUNDRY: CardRecord = CardRecord::new(
    CardSet::AlaraReborn,
    "Thopter Foundry",
    "42b8d797-b01d-49cf-9818-d84bba17029d",
    "Ralph Horsley",
    // Two mana for a machine that turns every spent artifact into a flier
    // and a life, which is why it is played beside the artifacts that come
    // back on their own.
    CardRules::new_artifact(mana_cost!("{W/B}{U}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice a nontoken artifact: Create a 1/1 blue Thopter artifact creature token \
         with flying. You gain 1 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificePermanent {
                // "A nontoken artifact": the Thopters it makes are artifacts too, so
                // without that word the Foundry would eat its own output forever.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::Sequence(&[
            EffectDef::create_artifact_creature_token(&["Thopter"], &[ManaColor::Blue], 1, 1)
                .with_abilities(&[abilities::flying()]),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SOUL_MANIPULATION, &PUTRID_LEECH, &THOPTER_FOUNDRY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
