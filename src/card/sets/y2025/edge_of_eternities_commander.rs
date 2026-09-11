//! Edge of Eternities Commander cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "EOC",
    slug: "edge-of-eternities-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// EOC 13 — Baloth Prime
pub(in crate::card::sets) static BALOTH_PRIME: CardRecord = CardRecord::new(
    "Baloth Prime",
    "2c723fc9-d5c9-4126-a9a6-f80c247a4b6b",
    "Joshua Raphael",
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
                    CostDef::Mana(mana_cost!("{4}")),
                    CostDef::SacrificePermanent {
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
