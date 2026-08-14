//! Future Sight cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType,
    CreatureStats, EffectDef, EffectDurationDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    TriggerEventDef, ValueDef, abilities, cards,
};
use crate::mana_cost;

// FUT 167 — Darksteel Garrison
pub(in crate::card::sets) static DARKSTEEL_GARRISON: CardRecord = CardRecord::new(
    cards::DARKSTEEL_GARRISON,
    "Darksteel Garrison",
    CardArt::new(
        "e77eaaa0-40f9-40e4-b0ba-5a8addd764d3",
        "David Martin",
    ),
    CardSet::FutureSight,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Fortification"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Fortified land has indestructible.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::indestructible()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever fortified land becomes tapped, target creature gets +1/+1 until end of turn.",
                TriggerEventDef::BecomesTapped(ObjectPredicateDef::AttachedToSource),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(crate::TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::fortify(
                mana_cost!("{3}"),
                "Fortify {3} ({3}: Attach to target land you control. Fortify only as a sorcery. This card enters unattached and stays on the battlefield if the land leaves.)",
            ),
        ]),
);

// FUT 174 — Dryad Arbor
pub(in crate::card::sets) static DRYAD_ARBOR: CardRecord = CardRecord::new(
    cards::DRYAD_ARBOR,
    "Dryad Arbor",
    CardArt::new("8cee476d-42e1-4997-87af-73e18f542167", "Eric Fortune"),
    CardSet::FutureSight,
    CardRules::new_land(&[])
        .with_type(CardType::Creature)
        .with_subtypes(&["Forest", "Dryad"])
        .with_creature_stats(CreatureStats {
            power: 1,
            toughness: 1,
        })
        .printed_colors(&[ManaColor::Green]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DARKSTEEL_GARRISON, &DRYAD_ARBOR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
