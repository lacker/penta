//! Oath of the Gatewatch card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "OGW",
    slug: "oath-of-the-gatewatch",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// OGW 26 — Make a Stand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAKE_A_STAND: CardRecord = CardRecord::new(
    "Make a Stand",
    "30cace63-91ca-493c-b67f-740fbbf06370",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// OGW 63 — Sphinx of the Final Word
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPHINX_OF_THE_FINAL_WORD: CardRecord = CardRecord::new(
    "Sphinx of the Final Word",
    "866f92a3-2738-4e0f-adda-3ff9227dc17a",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// OGW 91 — Untamed Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNTAMED_HUNGER: CardRecord = CardRecord::new(
    "Untamed Hunger",
    "595deb86-b6cf-4e4f-a6cf-f5ff128b720d",
    "Willian Murai",
    crate::card::CardRules::unsupported(),
);

// OGW 141 — Pulse of Murasa
pub(in crate::card::sets) static PULSE_OF_MURASA: CardRecord = CardRecord::new(
    "Pulse of Murasa",
    "c0c8057f-b45b-4f67-90cd-c808b5e9cbfa",
    "Matt Stewart",
// Either graveyard, so it also answers an opponent's reanimation target
    // by handing the card back to them rather than leaving it where it is.
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature or land card from a graveyard to its owner's hand. You gain 6 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(6),
            },
        ]),
    )),
);

// OGW 145 — Tajuru Pathwarden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAJURU_PATHWARDEN: CardRecord = CardRecord::new(
    "Tajuru Pathwarden",
    "b073e75b-b432-41a5-a71e-d169fecf774f",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// OGW 151 — Ayli, Eternal Pilgrim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AYLI_ETERNAL_PILGRIM: CardRecord = CardRecord::new(
    "Ayli, Eternal Pilgrim",
    "e7b5893d-6df6-4cae-ae70-d02d443d1740",
    "Cynthia Sheppard",
    crate::card::CardRules::unsupported(),
);

// OGW 183 — Wastes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WASTES: CardRecord = CardRecord::new(
    "Wastes",
    "7019912c-bd9b-4b96-9388-400794909aa1",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MAKE_A_STAND,
    &SPHINX_OF_THE_FINAL_WORD,
    &UNTAMED_HUNGER,
    &PULSE_OF_MURASA,
    &TAJURU_PATHWARDEN,
    &AYLI_ETERNAL_PILGRIM,
    &WASTES,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
