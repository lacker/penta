//! Theros cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, BasicLandType, CardArt,
    CardRules, CardSet, CardType, EffectDef, EffectDurationDef, EffectRecipientDef,
    ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// THS 56 — Nimbus Naiad
pub(in crate::card::sets) static NIMBUS_NAIAD: CardRecord = CardRecord::new(
    cards::NIMBUS_NAIAD,
    "Nimbus Naiad",
    CardArt::new("19e9c9a2-4c5b-4518-a127-e4ffb23437d6", "David Palumbo"),
    CardSet::Theros,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Nymph"], 2, 2)
        .with_type(CardType::Enchantment)
        .with_abilities(&[
            abilities::bestow(mana_cost!("{4}{U}")),
            abilities::flying(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has flying.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(2),
                            toughness: ValueDef::Constant(2),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::flying()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// THS 68 — Thassa's Emissary
pub(in crate::card::sets) static THASSAS_EMISSARY: CardRecord = CardRecord::new(
    cards::THASSAS_EMISSARY,
    "Thassa's Emissary",
    CardArt::new("f52b325b-3ea9-4a99-ae99-9d158560e45b", "Sam Burley"),
    CardSet::Theros,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Crab"], 3, 3)
        .with_type(CardType::Enchantment)
        .with_abilities(&[
            abilities::bestow(mana_cost!("{5}{U}")),
            AbilityDef::triggered(
                "Whenever this creature or enchanted creature deals combat damage to a player, draw a card.",
                TriggerEventDef::CombatDamageDealtToPlayer {
                    source: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::AttachedToSource,
                    ]),
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+3.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(3),
                        toughness: ValueDef::Constant(3),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// THS 169 — Nylea's Presence
pub(in crate::card::sets) static NYLEAS_PRESENCE: CardRecord = CardRecord::new(
    cards::NYLEAS_PRESENCE,
    "Nylea's Presence",
    CardArt::new("e68f1fd4-1a2f-405b-a592-6c4af6214eae", "Ralph Horsley"),
    CardSet::Theros,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When Nylea's Presence enters, draw a card.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: None,
                    to: Some(ZoneKind::Battlefield),
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land is every basic land type in addition to its other types.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::AddLandTypes(&BasicLandType::ALL),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&NIMBUS_NAIAD, &THASSAS_EMISSARY, &NYLEAS_PRESENCE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
