//! Rivals of Ixalan card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "RIX",
    slug: "rivals-of-ixalan",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// RIX 15 — Moment of Triumph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENT_OF_TRIUMPH: CardRecord = CardRecord::new(
    "Moment of Triumph",
    "08be8c18-eca3-4960-b174-e4a78579ed63",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// RIX 30 — Zetalpa, Primal Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZETALPA_PRIMAL_DAWN: CardRecord = CardRecord::new(
    "Zetalpa, Primal Dawn",
    "3d10560f-199d-4d04-b573-90024f8aecc4",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// RIX 41 — Kitesail Corsair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KITESAIL_CORSAIR: CardRecord = CardRecord::new(
    "Kitesail Corsair",
    "4b8d0e8d-c2d4-4682-8095-827ffd79539b",
    "Greg Opalinski",
    crate::card::CardRules::unsupported(),
);

// RIX 79 — Moment of Craving
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENT_OF_CRAVING: CardRecord = CardRecord::new(
    "Moment of Craving",
    "c0e3ea55-162d-4466-ba4e-b938a0845fb5",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// RIX 93 — Bombard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOMBARD: CardRecord = CardRecord::new(
    "Bombard",
    "0a605abc-78e8-47ba-9022-0fad9006fd05",
    "Alex Konstad",
    crate::card::CardRules::unsupported(),
);

// RIX 94 — Brass's Bounty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRASS_S_BOUNTY: CardRecord = CardRecord::new(
    "Brass's Bounty",
    "13505866-6ce9-4d11-b3f3-fc2f09839b71",
    "Grzegorz Rutkowski",
    crate::card::CardRules::unsupported(),
);

// RIX 100 — Etali, Primal Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ETALI_PRIMAL_STORM: CardRecord = CardRecord::new(
    "Etali, Primal Storm",
    "1d3d8bb4-0430-45bb-930d-5d6db6521945",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// RIX 101 — Fanatical Firebrand
pub(in crate::card::sets) static FANATICAL_FIREBRAND: CardRecord = CardRecord::new(
    "Fanatical Firebrand",
    "5e5565de-028c-4799-a9f6-4dcd685639eb",
    "Wayne Reynolds",
    // Haste is what makes the sacrifice a one-mana Shock the turn it lands;
    // left alive it is a one-power attacker that can cash itself in later.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Pirate"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: It deals 1 damage to any target.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// RIX 130 — Ghalta, Primal Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHALTA_PRIMAL_HUNGER: CardRecord = CardRecord::new(
    "Ghalta, Primal Hunger",
    "0104b5b3-9376-4ad7-9a77-3e564e9c42e6",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// RIX 148 — Thrashing Brontodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRASHING_BRONTODON: CardRecord = CardRecord::new(
    "Thrashing Brontodon",
    "0d9264ff-9f7c-46f3-862a-fee7ad213250",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// RIX 178 — Gleaming Barrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLEAMING_BARRIER: CardRecord = CardRecord::new(
    "Gleaming Barrier",
    "62447a76-4aa7-4823-941e-84bc18eb672a",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

// RIX 203 — Swab Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWAB_GOBLIN: CardRecord = CardRecord::new(
    "Swab Goblin",
    "0403b8e5-29c5-4a4a-b9e7-1e79a7452f14",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MOMENT_OF_TRIUMPH,
    &ZETALPA_PRIMAL_DAWN,
    &KITESAIL_CORSAIR,
    &MOMENT_OF_CRAVING,
    &BOMBARD,
    &BRASS_S_BOUNTY,
    &ETALI_PRIMAL_STORM,
    &FANATICAL_FIREBRAND,
    &GHALTA_PRIMAL_HUNGER,
    &THRASHING_BRONTODON,
    &GLEAMING_BARRIER,
    &SWAB_GOBLIN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
