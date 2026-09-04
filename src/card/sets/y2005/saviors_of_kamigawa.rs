//! SOK card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardArt, CardRules,
    CardSet, CardSupertype, CardType, CopyAbilityDef, CopyExceptionsDef, CostDef, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef, PlayerRelation,
    ReplacementEffectDef, ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

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

// SOK 53 — Sakashima the Impostor
static SAKASHIMA_RETURN: AbilityDef = AbilityDef::activated(
    "{2}{U}{U}: Return Sakashima the Impostor to its owner's hand at the beginning of the next end step.",
    &[CostDef::Mana(mana_cost!("{2}{U}{U}"))],
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

pub(in crate::card::sets) static SAKASHIMA_THE_IMPOSTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61dc2f54-3637-4caa-9741-36ff14dc5527"),
    "Sakashima the Impostor",
    CardArt::new("61dc2f54-3637-4caa-9741-36ff14dc5527", "rk post"),
    CardSet::SaviorsOfKamigawa,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Rogue"], 3, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::replacement(
            "You may have Sakashima the Impostor enter as a copy of any creature on the battlefield, except its name is Sakashima the Impostor, it's legendary in addition to its other types, and it has \"{2}{U}{U}: Return Sakashima the Impostor to its owner's hand at the beginning of the next end step.\"",
            ReplacementEffectDef::CopyEntering {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                exceptions: CopyExceptionsDef::NONE
                    .with_name("Sakashima the Impostor")
                    .with_added_supertypes(&[CardSupertype::Legendary])
                    .with_abilities(&[CopyAbilityDef::Ability(&SAKASHIMA_RETURN)]),
            },
            ),
            SAKASHIMA_RETURN,
        ]),
);

// SOK 63 — Death Denied
pub(in crate::card::sets) static DEATH_DENIED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f66ddc5-f5e6-44de-8189-87b6521d1fea"),
    "Death Denied",
    CardArt::new("8f66ddc5-f5e6-44de-8189-87b6521d1fea", "Greg Hildebrandt"),
    CardSet::SaviorsOfKamigawa,
    // Two black on top of X is a real tax, which is why this is a late-game
    // rebuild rather than a way to buy back one creature.
    CardRules::new_instant(mana_cost!("{X}{B}{B}"))
        .with_subtypes(&["Arcane"])
        .with_ability(AbilityDef::spell_with_targets(
            "Return X target creature cards from your graveyard to your hand.",
            // Exactly X, not up to X: a graveyard with fewer creatures than
            // the X paid for cannot legally cast it.
            &[AbilityTargetDef::exactly_value(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                ValueDef::ChosenX,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        )),
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
                    CostDef::Mana(mana_cost!("{2}{R}")),
                    CostDef::SacrificePermanent {
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
