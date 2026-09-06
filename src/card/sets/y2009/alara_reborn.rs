//! ARB card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardArt, CardRules,
    CardSet, CardType, CostDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    PlayerRelation, ResolvedEffectDurationDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// ARB 29 — Soul Manipulation
pub(in crate::card::sets) static SOUL_MANIPULATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcd3cb05-c6f9-435a-a0e7-1f85da4a36eb"),
    "Soul Manipulation",
    CardArt::new("bcd3cb05-c6f9-435a-a0e7-1f85da4a36eb", "Carl Critchlow"),
    CardSet::AlaraReborn,
    // Three mana for a counterspell is a poor rate and three for a regrowth
    // is worse; taking both at once is the whole card.
    CardRules::new_instant(mana_cost!("{1}{U}{B}")).with_ability(
        AbilityDef::modal_spell(
            "Choose one or both —",
            &[
                AbilityDef::spell_with_targets(
                    "Counter target creature spell.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Spell,
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::counter_target(TargetIndex::PRIMARY),
                ),
                AbilityDef::spell_with_targets(
                    "Return target creature card from your graveyard to your hand.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                ),
            ],
        )
        // Each mode carries its own target, so taking both counters one
        // spell and buys back a different creature.
        .with_mode_selection(1, 2, false),
    ),
);

// ARB 95 — Putrid Leech
pub(in crate::card::sets) static PUTRID_LEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aaa47568-5668-4a9f-ad1c-9a13010ffc2b"),
    "Putrid Leech",
    CardArt::new("aaa47568-5668-4a9f-ad1c-9a13010ffc2b", "Dave Allsop"),
    CardSet::AlaraReborn,
    // A two-mana 4/4 that costs two life a turn to be one, and the life is
    // paid before blockers rather than after.
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Zombie", "Leech"], 2, 2).with_ability(
        AbilityDef::activated(
            "Pay 2 life: This creature gets +2/+2 until end of turn. Activate only once each turn.",
            &[CostDef::PayLife(2)],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

// ARB 133 — Thopter Foundry
pub(in crate::card::sets) static THOPTER_FOUNDRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42b8d797-b01d-49cf-9818-d84bba17029d"),
    "Thopter Foundry",
    CardArt::new("42b8d797-b01d-49cf-9818-d84bba17029d", "Ralph Horsley"),
    CardSet::AlaraReborn,
    // Two mana for a machine that turns every spent artifact into a flier
    // and a life, which is why it is played beside the artifacts that come
    // back on their own.
    CardRules::new_artifact(mana_cost!("{W/B}{U}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice a nontoken artifact: Create a 1/1 blue Thopter artifact creature token \
         with flying. You gain 1 life.",
        &[
            CostDef::Mana(mana_cost!("{1}")),
            CostDef::SacrificePermanent {
                // "A nontoken artifact": the Thopters it makes are artifacts too, so
                // without that word the Foundry would eat its own output forever.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::Sequence(&[
            EffectDef::create_artifact_creature_token(&["Thopter"], &[ManaColor::Blue], 1, 1)
                .with_abilities(&[abilities::flying()]),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SOUL_MANIPULATION, &PUTRID_LEECH, &THOPTER_FOUNDRY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
