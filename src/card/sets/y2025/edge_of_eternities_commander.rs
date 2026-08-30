//! Edge of Eternities Commander cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardType, CounterKind, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    PlayerRelation, ReplacementEffectDef, TriggerEventDef, ValueDef, tokens,
};
use crate::mana_cost;

// EOC 13 — Baloth Prime
pub(in crate::card::sets) static BALOTH_PRIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c723fc9-d5c9-4126-a9a6-f80c247a4b6b"),
    "Baloth Prime",
    CardArt::new("2c723fc9-d5c9-4126-a9a6-f80c247a4b6b", "Joshua Raphael"),
    CardSet::EdgeOfEternitiesCommander,
    // A 10/10 for four that owes six untaps. Every land you feed him buys
    // one of them back and leaves a 4/4 behind, so the six counters are a
    // schedule rather than a wall.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 10, 10)
        .with_abilities(&[
            AbilityDef::as_enters(
                "This creature enters tapped with six stun counters on it. (If a permanent with a stun \
                 counter would become untapped, remove one from it instead.)",
                // "Enters tapped with six stun counters on it" is one clause about the way
                // he arrives, so it is one replacement with two parts rather than two
                // abilities: a 10/10 for four that owes six untaps.
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
                    ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::Stun,
                        amount: 6,
                    }),
                ]),
            ),
            AbilityDef::triggered(
                "Whenever you sacrifice a land, create a tapped 4/4 green Beast creature token and untap \
                 this creature.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    player: PlayerRelation::You,
                },
                // The untap is what pays the counters off: while any are left the clause
                // removes one instead of untapping him, so the lands are what wake him up.
                EffectDef::Sequence(&[
                    EffectDef::CreateToken {
                        token: tokens::creature(&["Beast"], &[ManaColor::Green], 4, 4),
                        copy: None,
                        controller: None,
                        count: ValueDef::Constant(1),
                        tapped: true,
                        attacking: false,
                        counters: None,
                        created: None,
                    },
                    EffectDef::Untap {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            ),
            AbilityDef::activated(
                "{4}, Sacrifice a land: You gain 2 life.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{4}")),
                    AbilityCostDef::SacrificePermanent {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        controller: PlayerRelation::You,
                    },
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BALOTH_PRIME];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
