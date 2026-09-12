//! HOU card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
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
// Audit: unsupported — There is no continuous prohibition on placing player or permanent counters in the counter mutation pipeline.
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
pub(in crate::card::sets) static RAZAKETH_THE_FOULBLOODED_73: CardRecord = CardRecord::new(
    "Razaketh, the Foulblooded",
    "e14adff9-33cc-467e-b782-068854c5e7b7",
    "Chris Rallis",
    CardRules::new_creature(mana_cost!("{5}{B}{B}{B}"), &["Demon"], 8, 8).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::flying(),
abilities::trample(),
AbilityDef::activated("Pay 2 life, Sacrifice another creature: Search your library for a card, put that card into your hand, then shuffle.", &[CostDef::PayLife(2), CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]))], EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::Any, minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// HOU 77 — Torment of Hailfire
// Audit: unsupported — There is no value-counted effect repetition that offers a fresh sacrifice-or-discard choice on each iteration; repeated payment costs cannot reproduce the optional life-loss branch.
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
// Audit: unsupported — Turn state records life gain and opponents who lost life, but not the total amount of life opponents lost this turn for its mana-producing trigger.
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
pub(in crate::card::sets) static MIRAGE_MIRROR_165: CardRecord = CardRecord::new(
    "Mirage Mirror",
    "29148d7e-b398-4e19-a29e-d9a660ad5016",
    "Craig J Spearing",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
AbilityDef::activated_with_targets("{2}: This artifact becomes a copy of target artifact, creature, enchantment, or land until end of turn.", &[CostDef::Mana(mana_cost!("{2}"))], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact),ObjectPredicateDef::HasType(CardType::Creature),ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::HasType(CardType::Enchantment)]))], EffectDef::BecomeCopyOf { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), copier: None, exceptions: CopyExceptionsDef::NONE, duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn) })
]),
);

// HOU 170 — Desert of the Fervent
pub(in crate::card::sets) static DESERT_OF_THE_FERVENT_170: CardRecord = CardRecord::new(
    "Desert of the Fervent",
    "f547d664-25ce-4a24-b3ae-7bf3cbdf4703",
    "Titus Lunter",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Red),
        abilities::cycling!("Cycling {1}{R}", &[CostDef::Mana(mana_cost!("{1}{R}"))]),
    ]),
);

// HOU 180 — Ipnu Rivulet
pub(in crate::card::sets) static IPNU_RIVULET_180: CardRecord = CardRecord::new(
    "Ipnu Rivulet",
    "203011ef-3737-4fd1-bd23-0e531b5a7c32",
    "James Paick",
    CardRules::new_land(&["Desert"]).with_abilities(&[
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated_mana("{T}, Pay 1 life: Add {U}.", &[CostDef::TapSource, CostDef::PayLife(1)], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue))),
AbilityDef::activated_with_targets("{1}{U}, {T}, Sacrifice a Desert: Target player mills four cards. (They put the top four cards of their library into their graveyard.)", &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::TapSource, CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any))], EffectDef::Mill { player: EffectRecipientDef::Target(TargetIndex::PRIMARY), amount: ValueDef::Constant(4) })
]),
);

// HOU 181 — Ramunap Ruins
pub(in crate::card::sets) static RAMUNAP_RUINS_181: CardRecord = CardRecord::new(
    "Ramunap Ruins",
    "af11d41a-0d29-45e9-9d27-a41282b9e292",
    "Florian de Gesincourt",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}, Pay 1 life: Add {R}.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated(
            "{2}{R}{R}, {T}, Sacrifice a Desert: This land deals 2 damage to each opponent.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}{R}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                    "Desert",
                ))),
            ],
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
        ),
    ]),
);

// HOU 182 — Scavenger Grounds
pub(in crate::card::sets) static SCAVENGER_GROUNDS_182: CardRecord = CardRecord::new(
    "Scavenger Grounds",
    "6cd91eeb-7abf-4538-91dc-47c736dfc237",
    "Steven Belledin",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice a Desert: Exile all graveyards.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                    "Desert",
                ))),
            ],
            EffectDef::move_to_zone(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::Any,
                ),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// HOU 184 — Survivors' Encampment
// Audit: unsupported — Mana-ability eligibility cannot reserve and pay the additional TapPermanents cost for another creature.
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
