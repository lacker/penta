//! SOK card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    ResolvedEffectDurationDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// SOK 2 — Araba Mothrider
pub(in crate::card::sets) static ARABA_MOTHRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6e4a170-1075-47e4-abe6-996b161573c1"),
    "Araba Mothrider",
    CardArt::new("f6e4a170-1075-47e4-abe6-996b161573c1", "Anthony S. Waters"),
    CardSet::SaviorsOfKamigawa,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Samurai"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::bushido(ValueDef::Constant(1)),
    ]),
);

// SOK 63 — Death Denied
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_DENIED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f66ddc5-f5e6-44de-8189-87b6521d1fea"),
    "Death Denied",
    crate::card::CardArt::new("8f66ddc5-f5e6-44de-8189-87b6521d1fea", "Greg Hildebrandt"),
    crate::card::CardSet::SaviorsOfKamigawa,
    crate::card::CardRules::unsupported(),
);

// SOK 104 — Iizuka the Ruthless
pub(in crate::card::sets) static IIZUKA_THE_RUTHLESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ce461f7-385d-4379-83de-49571247c30d"),
    "Iizuka the Ruthless",
    CardArt::new("9ce461f7-385d-4379-83de-49571247c30d", "Darrell Riche"),
    CardSet::SaviorsOfKamigawa,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Human", "Samurai"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::bushido(ValueDef::Constant(2)),
            AbilityDef::activated(
                "{2}{R}, Sacrifice a Samurai: Samurai creatures you control gain double strike until end of turn.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}{R}")),
                    AbilityCostDef::SacrificePermanent {
                        object: ObjectPredicateDef::Subtype("Samurai"),
                        controller: PlayerRelation::You,
                    },
                ],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Samurai"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&ARABA_MOTHRIDER, &DEATH_DENIED, &IIZUKA_THE_RUTHLESS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
