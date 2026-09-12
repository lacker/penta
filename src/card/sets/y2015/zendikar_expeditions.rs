//! Zendikar Expeditions card records required by the cEDH corpus.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ZoneKind;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "EXP",
    slug: "zendikar-expeditions",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// EXP 3 — Smoldering Marsh
pub(in crate::card::sets) static SMOLDERING_MARSH_3: CardRecord = CardRecord::new(
    "Smoldering Marsh",
    "2c9814b3-d4fe-4823-b310-ab284dc9b9be",
    "Titus Lunter",
    CardRules::new_land(&["Swamp", "Mountain"]).with_abilities(&[AbilityDef::as_enters(
        "This land enters tapped unless you control two or more basic lands.",
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 2,
            }),
            if_true: &[],
            if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::Tapped,
            )],
        },
    )]),
);

// EXP 4 — Cinder Glade
pub(in crate::card::sets) static CINDER_GLADE_4: CardRecord = CardRecord::new(
    "Cinder Glade",
    "1b5d2e9c-dd93-49ae-9a03-77c13c5ec298",
    "Titus Lunter",
    CardRules::new_land(&["Mountain", "Forest"]).with_abilities(&[AbilityDef::as_enters(
        "This land enters tapped unless you control two or more basic lands.",
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 2,
            }),
            if_true: &[],
            if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::Tapped,
            )],
        },
    )]),
);

// EXP 5 — Canopy Vista
pub(in crate::card::sets) static CANOPY_VISTA_5: CardRecord = CardRecord::new(
    "Canopy Vista",
    "3125bf61-f1b8-4b97-96bc-2d2a3a32adbd",
    "Titus Lunter",
    CardRules::new_land(&["Forest", "Plains"]).with_abilities(&[AbilityDef::as_enters(
        "This land enters tapped unless you control two or more basic lands.",
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 2,
            }),
            if_true: &[],
            if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::Tapped,
            )],
        },
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SMOLDERING_MARSH_3, &CINDER_GLADE_4, &CANOPY_VISTA_5];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
