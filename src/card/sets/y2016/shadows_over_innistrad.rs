//! SOI card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::card::tokens;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SOI",
    slug: "shadows-over-innistrad",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const CLUE_TOKEN: TokenCharacteristics = tokens::clue().with_art(CardArt::new(
    "f2c859e1-181e-44d1-afbd-bbd6e52cf42a",
    "John Avon",
));

// SOI 44 — Thraben Inspector
pub(in crate::card::sets) static THRABEN_INSPECTOR: CardRecord = CardRecord::new(
    "Thraben Inspector",
    "d140c3b7-ca78-483d-baeb-307b624fea8b",
    "Matt Stewart",
    // One mana for a body and a card: the Clue is the reason the 1/2 is
    // worth playing, and the body is the reason the card is cheap.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, investigate. (Create a Clue token. It's an artifact with \
             \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))),
        ),
    ),
);

// SOI 105 — Crow of Dark Tidings
pub(in crate::card::sets) static CROW_OF_DARK_TIDINGS: CardRecord = CardRecord::new(
    "Crow of Dark Tidings",
    "14e4d1b5-72de-4062-9fdd-e9bfd655ee79",
    "Tianhua X",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Bird"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters or dies, mill two cards. (Put the \
             top two cards of your library into your graveyard.)",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// SOI 223 — Rabid Bite
pub(in crate::card::sets) static RABID_BITE: CardRecord = CardRecord::new(
    "Rabid Bite",
    "2f573622-877b-4d21-adfc-40a32b7c2e6d",
    "Karl Kopinski",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control deals damage equal to its power \
         to target creature you don't control.",
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
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::damage_from(
            ObjectRefDef::Target(TargetIndex::PRIMARY),
            EffectRecipientDef::Target(TargetIndex(1)),
            ValueDef::TargetPower(TargetIndex::PRIMARY),
        ),
    )]),
);

// SOI 233 — Tireless Tracker
pub(in crate::card::sets) static TIRELESS_TRACKER: CardRecord = CardRecord::new(
    "Tireless Tracker",
    "ee8e9928-d9b2-4570-adb8-44b34115decd",
    "Eric Deschamps",
    // Three mana for a 3/2 that turns every land after it into a card, and
    // grows every time one of those cards is cashed in.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Scout"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, investigate. (Create a Clue token. \
             It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            // A land you control arriving, which is what landfall is: the Tracker's
            // own arrival is not one, and neither is a land somebody else plays.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))),
        ),
        AbilityDef::triggered(
            "Whenever you sacrifice a Clue, put a +1/+1 counter on this creature.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Clue")),
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// SOI 258 — Magnifying Glass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNIFYING_GLASS: CardRecord = CardRecord::new(
    "Magnifying Glass",
    "f7a708d5-f757-4fcf-a167-5b5920c6adeb",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THRABEN_INSPECTOR,
    &CROW_OF_DARK_TIDINGS,
    &RABID_BITE,
    &TIRELESS_TRACKER,
    &MAGNIFYING_GLASS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
