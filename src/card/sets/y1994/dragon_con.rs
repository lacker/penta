//! Dragon Con promo card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "PDRC",
    slug: "dragon-con",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// PDRC 1 — Nalathni Dragon
pub(in crate::card::sets) static NALATHNI_DRAGON: CardRecord = CardRecord::new(
    "Nalathni Dragon",
    "7f9c6be5-ec44-4c66-aad6-cf9eca765b6b",
    "Michael Whelan",
CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::banding(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            // The pump is the whole ability: the fourth activation in a turn installs the
            // delayed sacrifice, the way Dragon Whelp's does.
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceActivationsThisTurn {
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 4,
                    },
                    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, sacrifice this creature.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::sacrifice(EffectRecipientDef::Source),
                    ))),
                },
            ]),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&NALATHNI_DRAGON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
