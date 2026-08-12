//! Onslaught cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    BasicLandType, CardArt, CardRules, CardSet, CardType, EffectDef, EffectDurationDef,
    EffectRecipientDef, ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    cards,
};
use crate::{PlayerRelation, TargetIndex, TurnStepDef, mana_cost};

// ONS 206 — Goblin Pyromancer
pub(in crate::card::sets) static GOBLIN_PYROMANCER: CardRecord = CardRecord::new(
    cards::GOBLIN_PYROMANCER,
    "Goblin Pyromancer",
    CardArt::new(
        "bb4815b7-fc20-44a4-ad1c-66d92993557f",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Onslaught,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, Goblin creatures get +3/+0 until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Goblin"),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(3),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of the end step, destroy all Goblins.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::Subtype("Goblin"),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                can_regenerate: true,
            },
        ),
    ]),
);

// ONS 207 — Goblin Sharpshooter
pub(in crate::card::sets) static GOBLIN_SHARPSHOOTER: CardRecord = CardRecord::new(
    cards::GOBLIN_SHARPSHOOTER,
    "Goblin Sharpshooter",
    CardArt::new("7e689df7-b85d-4346-bee8-5e978b5cbbbc", "Greg Staples"),
    CardSet::Onslaught,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::DoesNotUntapDuringUntapStep,
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature dies, untap this creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ONS 275 — Naturalize
pub(in crate::card::sets) static NATURALIZE: CardRecord = CardRecord::new(
    cards::NATURALIZE,
    "Naturalize",
    CardArt::new("c0acc41f-b55b-47cb-8803-d39d72788799", "Ron Spears"),
    CardSet::Onslaught,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or enchantment.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ])),
        true,
    )),
);

const fn fetch_land(text: &'static str, land_types: &'static [BasicLandType]) -> CardRules {
    CardRules::new_land(&[]).with_ability(AbilityDef::activated(
        text,
        &[
            AbilityCostDef::TapSource,
            AbilityCostDef::PayLife(1),
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(land_types),
            minimum: 0,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
        },
    ))
}

// ONS 313 — Bloodstained Mire
pub(in crate::card::sets) static BLOODSTAINED_MIRE: CardRecord = CardRecord::new(
    cards::BLOODSTAINED_MIRE,
    "Bloodstained Mire",
    CardArt::new("68c72226-6f52-4322-8b14-18737293dfa0", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Swamp or Mountain card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Swamp, BasicLandType::Mountain],
    ),
);

// ONS 316 — Flooded Strand
pub(in crate::card::sets) static FLOODED_STRAND: CardRecord = CardRecord::new(
    cards::FLOODED_STRAND,
    "Flooded Strand",
    CardArt::new("b4e3d844-d3b4-41d8-921d-c1cb3af343f8", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Plains or Island card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Plains, BasicLandType::Island],
    ),
);

// ONS 321 — Polluted Delta
pub(in crate::card::sets) static POLLUTED_DELTA: CardRecord = CardRecord::new(
    cards::POLLUTED_DELTA,
    "Polluted Delta",
    CardArt::new("0f7585c8-9e21-4eef-afc1-2852de23db2f", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for an Island or Swamp card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Island, BasicLandType::Swamp],
    ),
);

// ONS 328 — Windswept Heath
pub(in crate::card::sets) static WINDSWEPT_HEATH: CardRecord = CardRecord::new(
    cards::WINDSWEPT_HEATH,
    "Windswept Heath",
    CardArt::new("7a7c5941-9c8a-4a40-9efb-a84f05c58e53", "Anthony S. Waters"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Forest or Plains card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Forest, BasicLandType::Plains],
    ),
);

// ONS 330 — Wooded Foothills
pub(in crate::card::sets) static WOODED_FOOTHILLS: CardRecord = CardRecord::new(
    cards::WOODED_FOOTHILLS,
    "Wooded Foothills",
    CardArt::new("cdad38f7-9dfa-4f1b-9fac-41ab2b253f53", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Mountain or Forest card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Mountain, BasicLandType::Forest],
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GOBLIN_PYROMANCER,
    &GOBLIN_SHARPSHOOTER,
    &NATURALIZE,
    &BLOODSTAINED_MIRE,
    &FLOODED_STRAND,
    &POLLUTED_DELTA,
    &WINDSWEPT_HEATH,
    &WOODED_FOOTHILLS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
