//! Lorwyn Eclipsed Commander card records required by the cEDH corpus.

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
    code: "ECC",
    slug: "lorwyn-eclipsed-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ECC 44 — Sodden Verdure
pub(in crate::card::sets) static SODDEN_VERDURE_44: CardRecord = CardRecord::new(
    "Sodden Verdure",
    "9030440a-a049-4152-afcf-b19648b20ce6",
    "Raymond Bonilla",
    CardRules::new_land(&["Forest", "Island"]).with_abilities(&[AbilityDef::as_enters(
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SODDEN_VERDURE_44];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
