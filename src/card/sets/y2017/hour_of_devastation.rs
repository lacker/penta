//! HOU card records required by supported formats.

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
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("HOU", "hour-of-devastation");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// HOU 48 — Striped Riverwinder
pub(in crate::card::sets) static STRIPED_RIVERWINDER: CardRecord = CardRecord::new(
    "Striped Riverwinder",
    "bbeef9ef-487c-400b-bcee-1c0e8ec94b6a",
    "Craig J Spearing",
    // Seven mana is never the plan, which is the point: a one-mana cantrip
    // that is still a real threat in the games that go long.
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Serpent"], 5, 5).with_abilities(&[
        abilities::hexproof(),
        abilities::cycling!(
            "Cycling {U} ({U}, Discard this card: Draw a card.)",
            &[crate::CostDef::Mana(mana_cost!("{U}"))],
        ),
    ]),
);

// HOU 83 — Abrade
pub(in crate::card::sets) static ABRADE: CardRecord = CardRecord::new(
    "Abrade",
    "84319dfb-eaf7-4b98-8c4f-30f5e779591b",
    "Jonas De Ro",
    // Two mana that is never dead: the half a red deck wants is whichever
    // one the board is holding.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        // One of two, chosen as it is cast: each half carries its own slot, so a
        // board with neither a creature nor an artifact leaves nothing to cast it
        // at.
        &[
            AbilityDef::spell_with_targets(
                "Abrade deals 3 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )),
);

// HOU 92 — Firebrand Archer
pub(in crate::card::sets) static FIREBRAND_ARCHER: CardRecord = CardRecord::new(
    "Firebrand Archer",
    "6ddc6b73-298b-4afa-990a-63706e77dd9f",
    "John Stanko",
    // The trigger fires on the cast rather than on the resolution, so a
    // countered spell has already paid for its point of damage.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Archer"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, this creature deals 1 damage to each opponent.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::damage(
                EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// HOU 138 — Bloodwater Entity
pub(in crate::card::sets) static BLOODWATER_ENTITY: CardRecord = CardRecord::new(
    "Bloodwater Entity",
    "474d0a04-b640-4d1d-b538-2d946c1ff913",
    "Viktor Titov",
    // The rebuy costs a draw step rather than a card, which is the price a
    // prowess deck pays to cast its best spell twice.
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Elemental"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::prowess(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may put target instant or sorcery card from your \
             graveyard on top of your library.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &STRIPED_RIVERWINDER,
    &ABRADE,
    &FIREBRAND_ARCHER,
    &BLOODWATER_ENTITY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
