//! RNA card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    InstalledTriggerDef, ObjectPredicateDef, PlayerRelation, SpellAdditionalCostDef,
    TriggerEventDef, TurnStepDef, ValueDef, abilities,
};
use crate::mana_cost;

static SPHINX_OPENING_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your first upkeep, scry 3.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    abilities::scry(ValueDef::Constant(3)),
);

// RNA 55 — Sphinx of Foresight
pub(in crate::card::sets) static SPHINX_OF_FORESIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf2386fd-edc0-4731-8f4e-7a7c45548bf3"),
    "Sphinx of Foresight",
    CardArt::new("cf2386fd-edc0-4731-8f4e-7a7c45548bf3", "Titus Lunter"),
    CardSet::RavnicaAllegiance,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Sphinx"], 4, 4).with_abilities(&[
        AbilityDef::opening_hand_reveal(
            "You may reveal this card from your opening hand. If you do, scry 3 at the beginning of your first upkeep.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&SPHINX_OPENING_TRIGGER)),
        ),
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, scry 1.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            abilities::scry(ValueDef::Constant(1)),
        ),
    ]),
);

// RNA 115 — Skewer the Critics
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKEWER_THE_CRITICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97295660-6bea-46ae-9a3b-0fc6abba407f"),
    "Skewer the Critics",
    crate::card::CardArt::new("97295660-6bea-46ae-9a3b-0fc6abba407f", "Heonhwa"),
    crate::card::CardSet::RavnicaAllegiance,
    crate::card::CardRules::unsupported(),
);

// RNA 171 — Final Payment
pub(in crate::card::sets) static FINAL_PAYMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49a21a8f-9c7b-4ae8-8635-f2ee2151c8de"),
    "Final Payment",
    CardArt::new(
        "49a21a8f-9c7b-4ae8-8635-f2ee2151c8de",
        "Victor Adame Minguez",
    ),
    CardSet::RavnicaAllegiance,
    CardRules::new_instant(mana_cost!("{W}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay 5 life or sacrifice a creature or \
             enchantment.\nDestroy target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            SpellAdditionalCostDef::choice(&[
                SpellAdditionalCostDef::pay_life(5),
                SpellAdditionalCostDef::sacrifice(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    1,
                ),
            ]),
            EffectDef::destroy_target(crate::TargetIndex::PRIMARY, true),
        ),
    ),
);

// RNA 172 — Fireblade Artist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIREBLADE_ARTIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21e1161f-bd2c-45a7-a86b-3b2e5210f148"),
    "Fireblade Artist",
    crate::card::CardArt::new("21e1161f-bd2c-45a7-a86b-3b2e5210f148", "Steve Argyle"),
    crate::card::CardSet::RavnicaAllegiance,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SPHINX_OF_FORESIGHT,
    &SKEWER_THE_CRITICS,
    &FINAL_PAYMENT,
    &FIREBLADE_ARTIST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
