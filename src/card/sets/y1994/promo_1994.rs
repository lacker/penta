//! The three unique 1994 promotional cards are legal in Old School 93/94, but
//! each currently needs an unsupported declarative capability. Their exact gaps
//! are recorded inline at their synthetic Eternal Central collector positions.

use crate::TargetIndex;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardType, ComparisonDef, EffectDef, EffectRecipientDef,
    InstalledTriggerDef, ObjectPredicateDef, ObjectRefDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, abilities,
};
use crate::mana_cost;

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// P94 1 — Arena
pub(in crate::card::sets) static ARENA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f989fda-2e54-427c-9154-4820c48abb02"),
    "Arena",
    CardArt::new("2f989fda-2e54-427c-9154-4820c48abb02", "Rob Alexander"),
    CardSet::Promo1994,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_with_targets(
        "{3}, {T}: Tap target creature you control and target creature of an opponent's choice they control. Those creatures fight each other.",
        &[
            AbilityCostDef::Mana(mana_cost!("{3}")),
            AbilityCostDef::TapSource,
        ],
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            })
            .chosen_by_opponent(),
        ],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex(1)),
            },
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ]),
    )),
);

// P94 2 — Sewers of Estark
// Audit: unsupported — Needs a duration-scoped replacement/prevention effect for “Choose target creature. If it's attacking, it can't be blocked this turn. If it's blocking, prevent all combat damage that would be dealt this combat by it and each creature it's blocking”.
pub(in crate::card::sets) static SEWERS_OF_ESTARK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0da11d4-3603-4f59-8f61-7204bf04e165"),
    "Sewers of Estark",
    CardArt::new("b0da11d4-3603-4f59-8f61-7204bf04e165", "Melissa A. Benson"),
    CardSet::Promo1994,
    CardRules::unsupported(),
);

// P94 3 — Nalathni Dragon
pub(in crate::card::sets) static NALATHNI_DRAGON: CardRecord = CardRecord::new_with_legacy_id(
    1781,
    "Nalathni Dragon",
    CardArt::new("7f9c6be5-ec44-4c66-aad6-cf9eca765b6b", "Michael Whelan"),
    CardSet::Promo1994,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::banding(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
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
                        EffectDef::Sacrifice {
                            object: EffectRecipientDef::Source,
                        },
                    ))),
                },
            ]),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&ARENA, &SEWERS_OF_ESTARK, &NALATHNI_DRAGON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
