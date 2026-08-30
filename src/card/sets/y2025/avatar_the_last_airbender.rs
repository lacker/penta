//! TLA card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules,
    CardSet, CardType, EffectDef, FightExcessDef, ManaColor, ObjectPredicateDef, ObjectRefDef,
    PlayerRelation, ValueDef, ZoneKind,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// TLA 144 — The Last Agni Kai
pub(in crate::card::sets) static THE_LAST_AGNI_KAI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61eaebc6-7575-48ed-b212-ff8b0c7ae694"),
    "The Last Agni Kai",
    CardArt::new("61eaebc6-7575-48ed-b212-ff8b0c7ae694", "Pablo Rivera"),
    CardSet::AvatarTheLastAirbender,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature you control fights target creature an opponent controls. If excess damage was dealt to the creature an opponent controls this way, add that much {R}. Until end of turn, you don't lose this mana as steps and phases end.",
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
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: Some(FightExcessDef {
                    recipient: ObjectRefDef::Target(TargetIndex(1)),
                    then: &EffectDef::AddManaEqualTo {
                        color: ManaColor::Red,
                        amount: ValueDef::MatchedCount,
                    },
                }),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The fight and excess red mana are implemented; retaining only that mana across steps and phases until end of turn is not yet modeled.",
        )),
    ),
);

// TLA 267 — Boiling Rock Prison
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOILING_ROCK_PRISON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c2e2220-54d1-4180-93a0-964e3b0ba8b8"),
    "Boiling Rock Prison",
    crate::card::CardArt::new("1c2e2220-54d1-4180-93a0-964e3b0ba8b8", "Matteo Bassini"),
    crate::card::CardSet::AvatarTheLastAirbender,
    crate::card::CardRules::unsupported(),
);

// TLA 271 — Kyoshi Village
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KYOSHI_VILLAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d5f3008-2af8-4e81-8847-1c91f524e747"),
    "Kyoshi Village",
    crate::card::CardArt::new("8d5f3008-2af8-4e81-8847-1c91f524e747", "Luc Courtois"),
    crate::card::CardSet::AvatarTheLastAirbender,
    crate::card::CardRules::unsupported(),
);

// TLA 279 — Serpent's Pass
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENT_S_PASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad87bff5-9b8c-44e4-a6d3-8cc71be9640a"),
    "Serpent's Pass",
    crate::card::CardArt::new("ad87bff5-9b8c-44e4-a6d3-8cc71be9640a", "Matteo Bassini"),
    crate::card::CardSet::AvatarTheLastAirbender,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THE_LAST_AGNI_KAI,
    &BOILING_ROCK_PRISON,
    &KYOSHI_VILLAGE,
    &SERPENT_S_PASS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
