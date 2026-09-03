//! SOK card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, CopyAbilityDef, CopyExceptionsDef, EffectDef, EffectRecipientDef,
    InstalledTriggerDef, ObjectPredicateDef, PlayerRelation, ReplacementEffectDef,
    ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
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

static SAKASHIMA_RETURN: AbilityDef = AbilityDef::activated(
    "{2}{U}{U}: Return Sakashima the Impostor to its owner's hand at the beginning of the next end step.",
    &[AbilityCostDef::Mana(mana_cost!("{2}{U}{U}"))],
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, return Sakashima the Impostor to its owner's hand.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Source,
            zone: crate::card::ZoneKind::Hand,
            placement: crate::card::ZonePlacement::Top,
        },
    ))),
);

// SOK 53 — Sakashima the Impostor
pub(in crate::card::sets) static SAKASHIMA_THE_IMPOSTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61dc2f54-3637-4caa-9741-36ff14dc5527"),
    "Sakashima the Impostor",
    CardArt::new("61dc2f54-3637-4caa-9741-36ff14dc5527", "rk post"),
    CardSet::SaviorsOfKamigawa,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Rogue"], 3, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::replacement(
            "You may have Sakashima the Impostor enter as a copy of any creature on the battlefield, except its name is Sakashima the Impostor, it's legendary in addition to its other types, and it has \"{2}{U}{U}: Return Sakashima the Impostor to its owner's hand at the beginning of the next end step.\"",
            ReplacementEffectDef::CopyEntering {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                exceptions: CopyExceptionsDef::NONE
                    .with_name("Sakashima the Impostor")
                    .with_added_supertypes(&[CardSupertype::Legendary])
                    .with_abilities(&[CopyAbilityDef::Ability(&SAKASHIMA_RETURN)]),
            },
        )),
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARABA_MOTHRIDER,
    &SAKASHIMA_THE_IMPOSTOR,
    &DEATH_DENIED,
    &IIZUKA_THE_RUTHLESS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
