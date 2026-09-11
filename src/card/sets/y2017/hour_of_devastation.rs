//! HOU card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "HOU",
    slug: "hour-of-devastation",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// HOU 22 — Solemnity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLEMNITY_22: CardRecord = CardRecord::new(
    "Solemnity",
    "0a71fb62-acbd-49f5-842f-0fc9fa48afea",
    "Greg Opalinski",
    crate::card::CardRules::unsupported(),
);

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

// HOU 73 — Razaketh, the Foulblooded
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAZAKETH_THE_FOULBLOODED_73: CardRecord = CardRecord::new(
    "Razaketh, the Foulblooded",
    "e14adff9-33cc-467e-b782-068854c5e7b7",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// HOU 77 — Torment of Hailfire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORMENT_OF_HAILFIRE_77: CardRecord = CardRecord::new(
    "Torment of Hailfire",
    "f69d77d1-5980-436c-bf48-790939b069aa",
    "Grzegorz Rutkowski",
    crate::card::CardRules::unsupported(),
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

// HOU 88 — Crash Through
pub(in crate::card::sets) static CRASH_THROUGH: CardRecord = CardRecord::new(
    "Crash Through",
    "4bdaba76-b98d-4699-9a5f-e59285b09552",
    "Izzy",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell(
        "Creatures you control gain trample until end of turn. (Each \
         of those creatures can deal excess combat damage to the \
         player or planeswalker it's attacking.)\nDraw a card.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
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

// HOU 104 — Neheb, the Eternal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEHEB_THE_ETERNAL_104: CardRecord = CardRecord::new(
    "Neheb, the Eternal",
    "54231832-d492-4812-b658-4ab9a30fefe2",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
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
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Library,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// HOU 165 — Mirage Mirror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRAGE_MIRROR_165: CardRecord = CardRecord::new(
    "Mirage Mirror",
    "29148d7e-b398-4e19-a29e-d9a660ad5016",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// HOU 170 — Desert of the Fervent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESERT_OF_THE_FERVENT_170: CardRecord = CardRecord::new(
    "Desert of the Fervent",
    "f547d664-25ce-4a24-b3ae-7bf3cbdf4703",
    "Titus Lunter",
    crate::card::CardRules::unsupported(),
);

// HOU 180 — Ipnu Rivulet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IPNU_RIVULET_180: CardRecord = CardRecord::new(
    "Ipnu Rivulet",
    "203011ef-3737-4fd1-bd23-0e531b5a7c32",
    "James Paick",
    crate::card::CardRules::unsupported(),
);

// HOU 181 — Ramunap Ruins
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMUNAP_RUINS_181: CardRecord = CardRecord::new(
    "Ramunap Ruins",
    "af11d41a-0d29-45e9-9d27-a41282b9e292",
    "Florian de Gesincourt",
    crate::card::CardRules::unsupported(),
);

// HOU 182 — Scavenger Grounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCAVENGER_GROUNDS_182: CardRecord = CardRecord::new(
    "Scavenger Grounds",
    "6cd91eeb-7abf-4538-91dc-47c736dfc237",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// HOU 184 — Survivors' Encampment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURVIVORS_ENCAMPMENT_184: CardRecord = CardRecord::new(
    "Survivors' Encampment",
    "c7b0404e-0f42-456b-91ce-f960195c4951",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SOLEMNITY_22,
    &STRIPED_RIVERWINDER,
    &RAZAKETH_THE_FOULBLOODED_73,
    &TORMENT_OF_HAILFIRE_77,
    &ABRADE,
    &CRASH_THROUGH,
    &FIREBRAND_ARCHER,
    &NEHEB_THE_ETERNAL_104,
    &BLOODWATER_ENTITY,
    &MIRAGE_MIRROR_165,
    &DESERT_OF_THE_FERVENT_170,
    &IPNU_RIVULET_180,
    &RAMUNAP_RUINS_181,
    &SCAVENGER_GROUNDS_182,
    &SURVIVORS_ENCAMPMENT_184,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
