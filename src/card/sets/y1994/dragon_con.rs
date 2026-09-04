//! Dragon Con promo card records.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardRules, CardSet, ComparisonDef, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, PlayerRelation, ResolvedEffectDurationDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, abilities,
};
use crate::mana_cost;

// PDRC 1 — Nalathni Dragon
pub(in crate::card::sets) static NALATHNI_DRAGON: CardRecord = CardRecord::new(
    CardSet::DragonCon,
    "Nalathni Dragon",
    "7f9c6be5-ec44-4c66-aad6-cf9eca765b6b",
    "Michael Whelan",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::banding(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
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
                    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                        &AbilityDef::triggered(
                            "At the beginning of the next end step, sacrifice this creature.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::End,
                                player: PlayerRelation::Any,
                            },
                            EffectDef::Sacrifice {
                                object: EffectRecipientDef::Source,
                            },
                        ),
                    )),
                },
            ]),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&NALATHNI_DRAGON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
