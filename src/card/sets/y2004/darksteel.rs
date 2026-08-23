//! Darksteel cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind,
    abilities,
};
use crate::mana_cost;

// DST 112 — Darksteel Ingot
pub(in crate::card::sets) static DARKSTEEL_INGOT: CardRecord = CardRecord::new_with_legacy_id(
    263,
    "Darksteel Ingot",
    CardArt::new("b02b9634-77e9-48ae-a6bf-859598d12c52", "Martina Pilcerova"),
    CardSet::Darksteel,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// DST 127 — Leonin Bola
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEONIN_BOLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7eab112-20a6-414f-84c9-678580485420"),
    "Leonin Bola",
    crate::card::CardArt::new(
        "a7eab112-20a6-414f-84c9-678580485420",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Darksteel,
    crate::card::CardRules::unsupported(),
);

// DST 140 — Skullclamp
pub(in crate::card::sets) static SKULLCLAMP: CardRecord = CardRecord::new_with_legacy_id(
    2120,
    "Skullclamp",
    CardArt::new("55318397-de3c-47ea-a088-72a24df5c8fa", "Luca Zontini"),
    CardSet::Darksteel,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            // The minus is the whole card: a one-toughness creature it is
            // attached to dies to state-based actions rather than to anything
            // the Clamp does on purpose, and the trigger below collects.
            AbilityDef::static_ability(
                "Equipped creature gets +1/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever equipped creature dies, draw two cards.",
                // The creature is already in the graveyard and the Clamp
                // already unattached by the time this is collected, so the
                // attachment it names is last-known information.
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// DST 157 — Vulshok Morningstar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VULSHOK_MORNINGSTAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("acf00de0-af24-4ef9-8ac2-135e6b53a8fd"),
    "Vulshok Morningstar",
    crate::card::CardArt::new("acf00de0-af24-4ef9-8ac2-135e6b53a8fd", "David Martin"),
    crate::card::CardSet::Darksteel,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DARKSTEEL_INGOT,
    &LEONIN_BOLA,
    &SKULLCLAMP,
    &VULSHOK_MORNINGSTAR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
