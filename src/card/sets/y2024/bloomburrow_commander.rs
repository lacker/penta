//! Bloomburrow Commander cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::ComparisonDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ReplacementEffectDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BLC",
    slug: "bloomburrow-commander",
});

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
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Rabbit"], &[ManaColor::White], 1, 1).with_art(
                            CardArt::new("81de52ef-7515-4958-abea-fb8ebdcef93c", "Gina Matarazzo"),
                        ),
                    ))
                    .with_count(ValueDef::SourcePower),
                ),
            ),
        ]),
);

// BLC 14 — Fortune Teller's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORTUNE_TELLER_S_TALENT_14: CardRecord = CardRecord::new(
    "Fortune Teller's Talent",
    "a1d43877-20ab-4e84-a597-4b5e03a6bf90",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// BLC 17 — Hazel's Brewmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZEL_S_BREWMASTER_17: CardRecord = CardRecord::new(
    "Hazel's Brewmaster",
    "52af8b70-a9c8-40d7-99da-fa51dc293688",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// BLC 35 — Trailtracker Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAILTRACKER_SCOUT_35: CardRecord = CardRecord::new(
    "Trailtracker Scout",
    "36ee967a-3cac-4fff-b616-ec2557c676f2",
    "Henry Peters",
    crate::card::CardRules::unsupported(),
);

// BLC 50 — Pollywog Prodigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLLYWOG_PRODIGY_50: CardRecord = CardRecord::new(
    "Pollywog Prodigy",
    "292158eb-cef0-4807-a38f-c5686064b95a",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

// BLC 56 — Agate Instigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGATE_INSTIGATOR_56: CardRecord = CardRecord::new(
    "Agate Instigator",
    "163c093e-9b6f-497d-a167-cfee1dbc5106",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &JACKED_RABBIT,
    &FORTUNE_TELLER_S_TALENT_14,
    &HAZEL_S_BREWMASTER_17,
    &TRAILTRACKER_SCOUT_35,
    &POLLYWOG_PRODIGY_50,
    &AGATE_INSTIGATOR_56,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
