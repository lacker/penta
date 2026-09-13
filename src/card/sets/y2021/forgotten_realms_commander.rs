//! Forgotten Realms Commander card records required by the cEDH corpus.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::{
    AbilityDef, AddManaEffectDef, CardRules, CostDef, CounterKind, EffectDef, EffectRecipientDef,
    RollDieDef, ValueDef,
};
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "AFC",
    slug: "forgotten-realms-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// AFC 59 — Component Pouch
pub(in crate::card::sets) static COMPONENT_POUCH: CardRecord = CardRecord::new(
    "Component Pouch",
    "8aa4eacb-48d8-49a1-90dd-042f3f1f75bb",
    "Chris Seaman",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}, Remove a component counter from this artifact: Add two mana of different colors.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("component"),
                    amount: 1,
                },
            ],
            EffectDef::AddMana(AddManaEffectDef::two_different_colors()),
        ),
        AbilityDef::activated(
            "{T}: Roll a d20.\n\
             1—9 | Put a component counter on this artifact.\n\
             10—20 | Put two component counters on this artifact.",
            &[CostDef::TapSource],
            EffectDef::RollDie(RollDieDef::new(
                20,
                &[
                    (
                        9,
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::named("component"),
                            amount: ValueDef::Constant(1),
                        },
                    ),
                    (
                        20,
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::named("component"),
                            amount: ValueDef::Constant(2),
                        },
                    ),
                ],
            )),
        ),
    ]),
);

// AFC 324 — Prosper, Tome-Bound
// Audit: unsupported — The end-step trigger needs an exile-to-play permission limited to the next turn and a trigger for cards played from exile.
pub(in crate::card::sets) static PROSPER_TOME_BOUND_324: CardRecord = CardRecord::new(
    "Prosper, Tome-Bound",
    "0333ccf1-239a-4de2-bb0a-9b4ac1649adf",
    "Yongjae Choi",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&COMPONENT_POUCH, &PROSPER_TOME_BOUND_324];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
