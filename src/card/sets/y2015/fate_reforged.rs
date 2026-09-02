//! FRF card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType, ComparisonDef,
    ConditionalStaticEffectDef, EffectDef, EffectRecipientDef, KeywordAbility, ObjectPredicateDef,
    ObjectSetCountConditionDef, ObjectSetDef, ObjectSetFilterDef, StaticApplyDef, abilities,
};
use crate::mana_cost;

// FRF 72 — Gurmag Angler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GURMAG_ANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c60a8cf1-a8c7-4f45-bbd3-188fab2652f9"),
    "Gurmag Angler",
    crate::card::CardArt::new("c60a8cf1-a8c7-4f45-bbd3-188fab2652f9", "YW Tang"),
    crate::card::CardSet::FateReforged,
    crate::card::CardRules::unsupported(),
);

// FRF 84 — Soulflayer
const fn soulflayer_ability(keyword: KeywordAbility, ability: &'static AbilityDef) -> EffectDef {
    EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
        condition: ObjectSetCountConditionDef {
            objects: &ObjectSetDef::Matching {
                objects: &ObjectSetDef::LinkedExiles,
                object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            },
            filter: Some(ObjectSetFilterDef::HasKeyword(keyword)),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 1,
        },
        then: StaticApplyDef {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(ability),
        },
    })
}

pub(in crate::card::sets) static SOULFLAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5084c8ff-1296-4d8e-bd06-93b1a3401661"),
    "Soulflayer",
    CardArt::new("5084c8ff-1296-4d8e-bd06-93b1a3401661", "Seb McKinnon"),
    CardSet::FateReforged,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Demon"], 4, 4).with_abilities(&[
        abilities::delve(),
        AbilityDef::static_ability(
            "If a creature card with flying was exiled with delve to cast this creature, this creature has flying. The same is true for first strike, double strike, deathtouch, haste, hexproof, indestructible, lifelink, reach, trample, and vigilance.",
            EffectDef::Sequence(&[
                soulflayer_ability(KeywordAbility::Flying, &abilities::flying()),
                soulflayer_ability(KeywordAbility::FirstStrike, &abilities::first_strike()),
                soulflayer_ability(KeywordAbility::DoubleStrike, &abilities::double_strike()),
                soulflayer_ability(KeywordAbility::Deathtouch, &abilities::deathtouch()),
                soulflayer_ability(KeywordAbility::Haste, &abilities::haste()),
                soulflayer_ability(KeywordAbility::Hexproof, &abilities::hexproof()),
                soulflayer_ability(KeywordAbility::Indestructible, &abilities::indestructible()),
                soulflayer_ability(KeywordAbility::Lifelink, &abilities::lifelink()),
                soulflayer_ability(KeywordAbility::Reach, &abilities::reach()),
                soulflayer_ability(KeywordAbility::Trample, &abilities::trample()),
                soulflayer_ability(KeywordAbility::Vigilance, &abilities::vigilance()),
            ]),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&GURMAG_ANGLER, &SOULFLAYER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
