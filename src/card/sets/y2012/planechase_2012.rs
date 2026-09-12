//! Planechase 2012 cards cataloged for the Vintage Cube pool.

use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ZoneKind;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "PC2",
    slug: "planechase-2012",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// PC2 40 — Beetleback Chief
pub(in crate::card::sets) static BEETLEBACK_CHIEF: CardRecord = CardRecord::new(
    "Beetleback Chief",
    "1e3ccf3d-583c-46b4-b51e-ae1b0628d506",
    "Wayne England",
    // Four power across three bodies for four mana: the Chief is a sacrifice
    // outlet's worth of goblins rather than one creature.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Goblin", "Warrior"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, create two 1/1 red Goblin creature tokens.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Goblin"],
                    &[ManaColor::Red],
                    1,
                    1,
                )))
                .with_amount(2),
            ),
        ),
    ),
);

// PC2 82 — Baleful Strix
pub(in crate::card::sets) static BALEFUL_STRIX: CardRecord = CardRecord::new(
    "Baleful Strix",
    "62090c97-7e3e-4854-bc44-c4a900133ec5",
    "Nils Hamm",
    // Two mana that replaces itself and then eats whatever attacks into it,
    // however large. Nothing about the body matters except that it blocks.
    CardRules::new_artifact_creature(mana_cost!("{U}{B}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::deathtouch(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// PC2 101 — Maelstrom Wanderer
pub(in crate::card::sets) static MAELSTROM_WANDERER_101: CardRecord = CardRecord::new(
    "Maelstrom Wanderer",
    "9129baf5-ffa9-4ffb-bcab-19d6a42dbfcc",
    "Thomas M. Baxa",
    CardRules::new_creature(mana_cost!("{5}{G}{U}{R}"), &["Elemental"], 7, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Creatures you control have haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            abilities::cascade(),
            abilities::cascade(),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BEETLEBACK_CHIEF,
    &BALEFUL_STRIX,
    &MAELSTROM_WANDERER_101,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
