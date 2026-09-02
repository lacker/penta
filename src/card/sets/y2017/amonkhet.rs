//! Amonkhet cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef,
    ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// AKH 75 — Vizier of Tumbling Sands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIZIER_OF_TUMBLING_SANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce4ff0f5-abee-4f3e-89ae-1b7ee771ec68"),
    "Vizier of Tumbling Sands",
    crate::card::CardArt::new("ce4ff0f5-abee-4f3e-89ae-1b7ee771ec68", "Josu Hernaiz"),
    crate::card::CardSet::Amonkhet,
    crate::card::CardRules::unsupported(),
);

// AKH 81 — Bone Picker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_PICKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdc6a825-43f7-40a4-95f0-335dc538b6cd"),
    "Bone Picker",
    crate::card::CardArt::new("bdc6a825-43f7-40a4-95f0-335dc538b6cd", "Yeong-Hao Han"),
    crate::card::CardSet::Amonkhet,
    crate::card::CardRules::unsupported(),
);

// AKH 134 — Glorybringer
pub(in crate::card::sets) static GLORYBRINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3277ad99-5682-4baa-b106-de15721876a6"),
    "Glorybringer",
    CardArt::new("3277ad99-5682-4baa-b106-de15721876a6", "Sam Burley"),
    CardSet::Amonkhet,
    // Five mana that attacks the turn it lands for four in the air and kills
    // something on the way in. What exerting costs is the next attack, which
    // is the only thing keeping it honest.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "You may exert this creature as it attacks. When you do, it deals 4 damage to target \
             non-Dragon creature an opponent controls.",
            TriggerEventDef::Exerted(ObjectPredicateDef::Source),
            // "Target non-Dragon creature an opponent controls." The exclusion is why
            // the card does not simply answer another Glorybringer, which is the whole
            // reason it is printed that way.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Dragon")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// AKH 241 — Cradle of the Accursed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRADLE_OF_THE_ACCURSED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41713e82-c3d3-4c2f-b075-f684cbd68ce8"),
    "Cradle of the Accursed",
    crate::card::CardArt::new("41713e82-c3d3-4c2f-b075-f684cbd68ce8", "Noah Bradley"),
    crate::card::CardSet::Amonkhet,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &VIZIER_OF_TUMBLING_SANDS,
    &BONE_PICKER,
    &GLORYBRINGER,
    &CRADLE_OF_THE_ACCURSED,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
