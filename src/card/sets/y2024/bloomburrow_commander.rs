//! Bloomburrow Commander cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::ComparisonDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ReplacementEffectDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("BLC", "bloomburrow-commander");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// BLC 9 — Jacked Rabbit
pub(in crate::card::sets) static JACKED_RABBIT: CardRecord = CardRecord::new(
    "Jacked Rabbit",
    "2c695df6-6bf2-4e6b-8500-e3116137ca27",
    "Scott Murphy",
// The counters are the body and the body is the token count, so every
    // mana past the second is another Rabbit on every attack.
    CardRules::new_creature(mana_cost!("{X}{1}{W}"), &["Rabbit", "Warrior"], 1, 2)
        .with_abilities(&[
            AbilityDef::as_enters(
                "Ravenous (This creature enters with X +1/+1 counters on it.)",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCastXCounters {
                        kind: CounterKind::PlusOnePlusOne,
                    },
                ),
            ),
            AbilityDef::triggered_if(
                "If X is 5 or more, draw a card when this creature enters.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &// Ravenous reads the X the spell was cast for, which the permanent recorded
                    // as it arrived: the entering object is a new one, so the X the spell chose
                    // is not on it any more.
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::SourceCastX,
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(5),
                    }),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever this creature attacks, create a number of 1/1 white Rabbit creature tokens \
                 equal to this creature's power.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::create_creature_token(&["Rabbit"], &[ManaColor::White], 1, 1)
                    .with_art(CardArt::new(
                        "81de52ef-7515-4958-abea-fb8ebdcef93c",
                        "Gina Matarazzo",
                    ))
                    .with_count(ValueDef::SourcePower),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&JACKED_RABBIT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
