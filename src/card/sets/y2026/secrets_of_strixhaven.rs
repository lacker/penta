//! SOS card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityDef, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    PlayerRelation, SpellAdditionalCostDef, ValueDef, ZoneKind,
};
use crate::mana_cost;

// SOS 12 — Elite Interceptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELITE_INTERCEPTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Elite Interceptor",
    "2970683e-e69c-42cb-a067-34abd56fb42b",
    "Lindsey Look",
    crate::card::CardRules::unsupported(),
);

// SOS 42 — Deluge Virtuoso
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELUGE_VIRTUOSO: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Deluge Virtuoso",
    "2e3b16ed-8727-48fd-8b1f-c0cbd329385e",
    "Justine Cruz",
    crate::card::CardRules::unsupported(),
);

// SOS 68 — Spellbook Seeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLBOOK_SEEKER: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Spellbook Seeker",
    "cc44eaa4-59a4-419e-b1d1-d92f354ff588",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// SOS 241 — Vicious Rivalry
pub(in crate::card::sets) static VICIOUS_RIVALRY: CardRecord = CardRecord::new(
    CardSet::SecretsOfStrixhaven,
    "Vicious Rivalry",
    "6fa9cd18-3181-4373-ab65-49bf9de9487f",
    "Chris Rallis",
    CardRules::new_sorcery(mana_cost!("{2}{B}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay X life.\nDestroy all artifacts and \
             creatures with mana value X or less.",
            &[],
            SpellAdditionalCostDef::pay_life(CostQuantityDef::ChosenX),
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// SOS 242 — Visionary's Dance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISIONARY_S_DANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Visionary's Dance",
    "846a0e79-a530-429e-8f7f-4b87f1b0156e",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// SOS 255 — Fields of Strife
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIELDS_OF_STRIFE: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Fields of Strife",
    "3dc7a4c3-c356-4fba-bea0-e8788da3eb57",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// SOS 256 — Forum of Amity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORUM_OF_AMITY: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Forum of Amity",
    "1de6c6cc-0c55-4997-8623-d7f796bd9ab8",
    "Richard Wright",
    crate::card::CardRules::unsupported(),
);

// SOS 258 — Paradox Gardens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARADOX_GARDENS: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Paradox Gardens",
    "dbc3447e-1329-4ea1-b4ca-b321b0ffec8f",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// SOS 262 — Spectacle Summit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTACLE_SUMMIT: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Spectacle Summit",
    "a0a66f7b-eab4-45da-8895-c2c2c7eb05f8",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// SOS 266 — Titan's Grave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TITAN_S_GRAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::SecretsOfStrixhaven,
    "Titan's Grave",
    "a9ab41c8-3ee2-4676-9b8b-20c34d9f5f21",
    "Lorenzo Lanfranconi",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ELITE_INTERCEPTOR,
    &DELUGE_VIRTUOSO,
    &SPELLBOOK_SEEKER,
    &VICIOUS_RIVALRY,
    &VISIONARY_S_DANCE,
    &FIELDS_OF_STRIFE,
    &FORUM_OF_AMITY,
    &PARADOX_GARDENS,
    &SPECTACLE_SUMMIT,
    &TITAN_S_GRAVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
