//! Starter 1999 card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "S99",
    slug: "starter-1999",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// S99 15 — Eager Cadet
pub(in crate::card::sets) static EAGER_CADET: CardRecord = CardRecord::new(
    "Eager Cadet",
    "d1e1ce2f-d8af-4fd0-975e-9d910d12b883",
    "Scott M. Fischer",
    // A vanilla 1/1 for one, printed for the starter decks that needed a
    // creature everybody could read.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1),
);

// S99 59 — Vizzerdrix
pub(in crate::card::sets) static VIZZERDRIX: CardRecord = CardRecord::new(
    "Vizzerdrix",
    "25711022-7270-4335-a48b-9f2b8275ceeb",
    "Eric Peterson",
    // Seven mana for a 6/6 with nothing on it. A starter-deck rare, and a
    // reminder of what blue paid for raw size before it stopped paying.
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Rabbit", "Beast"], 6, 6),
);

// S99 71 — Dakmor Lancer
pub(in crate::card::sets) static DAKMOR_LANCER: CardRecord = CardRecord::new(
    "Dakmor Lancer",
    "9d012ddf-abe1-4de9-89cb-78d82afb9e7b",
    "Chippy",
    // Six mana for removal and a body, which is what removal on a creature
    // cost before the Kavu showed it could cost four.
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Human", "Knight"], 3, 3).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target nonblack creature.",
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                    ]),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// S99 99 — Goblin Chariot
pub(in crate::card::sets) static GOBLIN_CHARIOT: CardRecord = CardRecord::new(
    "Goblin Chariot",
    "9ca11a7e-17f8-419f-9ba8-1bcaa3860f8b",
    "Pete Venters",
    // A vanilla 2/2 that gets to attack immediately.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 2, 2)
        .with_ability(abilities::haste()),
);

// S99 120 — Trained Orgg
pub(in crate::card::sets) static TRAINED_ORGG: CardRecord = CardRecord::new(
    "Trained Orgg",
    "425540b0-c826-4814-b0df-032264b1c237",
    "Eric Peterson",
    // Seven mana for a 6/6, red's half of the same starter-deck bargain.
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Orgg"], 6, 6),
);

// S99 139 — Pride of Lions
pub(in crate::card::sets) static PRIDE_OF_LIONS: CardRecord = CardRecord::new(
    "Pride of Lions",
    "f5006984-8e3d-4f13-b12e-1fbecd134bb3",
    "Carl Critchlow",
    // The mid-sized version of the same idea, printed into a core set
    // where four unpreventable damage a turn was a clock.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Cat"], 4, 4).with_abilities(&[
        AbilityDef::static_ability(
            "You may have this creature assign its combat damage as though it weren't blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(
                    AppliedRuleDef::MayAssignCombatDamageAsThoughUnblocked,
                ),
            },
        ),
    ]),
);

// S99 143 — Squall
pub(in crate::card::sets) static SQUALL: CardRecord = CardRecord::new(
    "Squall",
    "63c1b2f6-e47f-4f18-a94a-1d08eb009ef3",
    "Carl Critchlow",
    // Half a Needle Storm at the same cost, which is what a common gets.
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Squall deals 2 damage to each creature with flying.",
        EffectDef::damage(
            EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            ValueDef::Constant(2),
        ),
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EAGER_CADET,
    &VIZZERDRIX,
    &DAKMOR_LANCER,
    &GOBLIN_CHARIOT,
    &TRAINED_ORGG,
    &PRIDE_OF_LIONS,
    &SQUALL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
