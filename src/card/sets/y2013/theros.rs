//! Theros cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ValueDef, ZoneKind, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// THS 16 — Gods Willing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GODS_WILLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abafabb3-b2e7-4d78-b4b7-d8f701d3ee8b"),
    "Gods Willing",
    crate::card::CardArt::new("abafabb3-b2e7-4d78-b4b7-d8f701d3ee8b", "Mark Winters"),
    crate::card::CardSet::Theros,
    crate::card::CardRules::unsupported(),
);

// THS 169 — Nylea's Presence
pub(in crate::card::sets) static NYLEAS_PRESENCE: CardRecord = CardRecord::new_with_legacy_id(
    253,
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
            abilities::enters_trigger(
                "When Nylea's Presence enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land is every basic land type in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_basic_land_types(&BasicLandType::ALL),
                },
            ),
        ]),
);

// THS 180 — Sylvan Caryatid
pub(in crate::card::sets) static SYLVAN_CARYATID: CardRecord = CardRecord::new_with_legacy_id(
    2228,
    "Sylvan Caryatid",
    CardArt::new("d40b65c1-b24d-492d-81b9-d8474ebdc08c", "Chase Stone"),
    CardSet::Theros,
    // Hexproof is what separates it from every other two-mana accelerant: the
    // removal that answers a mana creature cannot be pointed at this one, and
    // a 0/3 wall survives most of what is left.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant"], 0, 3).with_abilities(&[
        abilities::defender(),
        abilities::hexproof(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&GODS_WILLING, &NYLEAS_PRESENCE, &SYLVAN_CARYATID];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
