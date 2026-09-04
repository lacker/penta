//! Theros cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, BasicLandType, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ValueDef, ZoneKind, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// THS 16 — Gods Willing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GODS_WILLING: CardRecord = CardRecord::new(
    crate::card::CardSet::Theros,
    "Gods Willing",
    "abafabb3-b2e7-4d78-b4b7-d8f701d3ee8b",
    "Mark Winters",
    crate::card::CardRules::unsupported(),
);

// THS 89 — Gray Merchant of Asphodel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAY_MERCHANT_OF_ASPHODEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Theros,
    "Gray Merchant of Asphodel",
    "b06078ce-f534-4e16-9a70-d51620a33eb2",
    "Robbie Trevino",
    crate::card::CardRules::unsupported(),
);

// THS 127 — Lightning Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_STRIKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Theros,
    "Lightning Strike",
    "bbb03f2e-2b92-4aa1-afae-301ed5d151d3",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// THS 169 — Nylea's Presence
pub(in crate::card::sets) static NYLEAS_PRESENCE: CardRecord = CardRecord::new(
    CardSet::Theros,
    "Nylea's Presence",
    "e68f1fd4-1a2f-405b-a592-6c4af6214eae",
    "Ralph Horsley",
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
pub(in crate::card::sets) static SYLVAN_CARYATID: CardRecord = CardRecord::new(
    CardSet::Theros,
    "Sylvan Caryatid",
    "d40b65c1-b24d-492d-81b9-d8474ebdc08c",
    "Chase Stone",
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GODS_WILLING,
    &GRAY_MERCHANT_OF_ASPHODEL,
    &LIGHTNING_STRIKE,
    &NYLEAS_PRESENCE,
    &SYLVAN_CARYATID,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
