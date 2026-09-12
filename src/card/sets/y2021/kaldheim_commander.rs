//! Kaldheim Commander card records.

use super::{CardRecord, PrintingRecord};
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "KHC",
    slug: "kaldheim-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// KHC 1 — Lathril, Blade of the Elves
pub(in crate::card::sets) static LATHRIL_BLADE_OF_THE_ELVES: CardRecord = CardRecord::new(
    "Lathril, Blade of the Elves",
    "547888c3-a9a6-4413-b29a-6bcd8a9279bf",
    "Caroline Gariba",
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Elf", "Noble"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::triggered(
                "Whenever Lathril deals combat damage to a player, create that \
                 many 1/1 green Elf Warrior creature tokens.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::create_creature_token(&["Elf", "Warrior"], &[ManaColor::Green], 1, 1)
                    .with_count(ValueDef::TriggerEventAmount),
            ),
            AbilityDef::activated(
                "{T}, Tap ten untapped Elves you control: Each opponent loses \
                 10 life and you gain 10 life.",
                &[
                    CostDef::TapSource,
                    CostDef::TapPermanents {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                        controller: PlayerRelation::You,
                        count: 10,
                    },
                ],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(10),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(10),
                    },
                ]),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&LATHRIL_BLADE_OF_THE_ELVES];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
