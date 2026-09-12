//! Jumpstart card records.

use super::{CardRecord, PrintingRecord};
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "JMP",
    slug: "jumpstart",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// JMP 4 — Release the Dogs
pub(in crate::card::sets) static RELEASE_THE_DOGS: CardRecord = CardRecord::new(
    "Release the Dogs",
    "7df3cd89-02c9-4a1c-9a8a-d17a0b1030c9",
    "Jason Kang",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell(
        "Create four 1/1 white Dog creature tokens.",
        EffectDef::create_creature_token(&["Dog"], &[ManaColor::White], 1, 1)
            .with_count(ValueDef::Constant(4)),
    )]),
);

// JMP 11 — Corsair Captain
pub(in crate::card::sets) static CORSAIR_CAPTAIN: CardRecord = CardRecord::new(
    "Corsair Captain",
    "a9b016d4-ddf6-47d6-b934-a0b979b60680",
    "Victor Adame Minguez",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Pirate"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            EffectDef::create_token(tokens::treasure()).with_count(ValueDef::Constant(1)),
        ),
        AbilityDef::static_ability(
            "Other Pirates you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&RELEASE_THE_DOGS, &CORSAIR_CAPTAIN];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
