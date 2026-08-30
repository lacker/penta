//! Zendikar cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet, CardType,
    ComparisonDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectRefDef,
    PlayerRelation, TriggerConditionDef, TriggerEventDef, ValueComparisonDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// The five allied fetchlands of Onslaught got an enemy-coloured cycle here,
/// with the same text. One helper states it once; only the two land types and
/// the order they are named in differ.
const fn fetch_land(text: &'static str, land_types: &'static [BasicLandType]) -> CardRules {
    CardRules::new_land(&[]).with_ability(abilities::fetch_land_ability(
        text,
        ObjectPredicateDef::HasAnyBasicLandType(land_types),
    ))
}

// ZEN 14 — Journey to Nowhere
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JOURNEY_TO_NOWHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("09cfe585-8a55-4b27-89e0-dfb6946fe1f3"),
    "Journey to Nowhere",
    crate::card::CardArt::new("09cfe585-8a55-4b27-89e0-dfb6946fe1f3", "Warren Mahy"),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 23 — Kor Skyfisher
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KOR_SKYFISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb2e9465-f5ba-4c7b-9f03-d40dc8394acd"),
    "Kor Skyfisher",
    crate::card::CardArt::new("bb2e9465-f5ba-4c7b-9f03-d40dc8394acd", "Dan Murayama Scott"),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 48 — Into the Roil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INTO_THE_ROIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5dba9972-dd8b-407b-9374-a8f0ed1a96db"),
    "Into the Roil",
    crate::card::CardArt::new("5dba9972-dd8b-407b-9374-a8f0ed1a96db", "Kieran Yanner"),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 67 — Spell Pierce
pub(in crate::card::sets) static SPELL_PIERCE: CardRecord = CardRecord::new_with_legacy_id(
    2115,
    "Spell Pierce",
    CardArt::new("cb3d3901-e4a6-45ab-a7b5-c65d91e1875e", "Vance Kovacs"),
    CardSet::Zendikar,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target noncreature spell unless its controller pays {2}.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
        )],
        abilities::counter_target_unless_paid(ValueDef::Constant(2)),
    )),
);

// ZEN 83 — Bloodghast
pub(in crate::card::sets) static BLOODGHAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63870c81-63bf-4a9a-aeb5-74c6eaded9f1"),
    "Bloodghast",
    CardArt::new("63870c81-63bf-4a9a-aeb5-74c6eaded9f1", "Daarken"),
    CardSet::Zendikar,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Vampire", "Spirit"], 2, 1)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This creature can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                },
            ),
            AbilityDef::static_ability(
                "This creature has haste as long as an opponent has 10 or less life.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::LifeTotal(PlayerRelation::Opponent),
                            comparison: ComparisonDef::LessOrEqual,
                            right: ValueDef::Constant(10),
                        }),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    },
                },
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, you may return this card from your graveyard to the battlefield.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// ZEN 87 — Disfigure
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISFIGURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3842ad2-a449-4963-8c96-276554125757"),
    "Disfigure",
    crate::card::CardArt::new("b3842ad2-a449-4963-8c96-276554125757", "Justin Sweet"),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 114 — Vampire Hexmage
pub(in crate::card::sets) static VAMPIRE_HEXMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93d2c4d1-6205-404a-b03d-995b90a3a33a"),
    "Vampire Hexmage",
    CardArt::new("93d2c4d1-6205-404a-b03d-995b90a3a33a", "Eric Deschamps"),
    CardSet::Zendikar,
    // A two-mana first striker that is never a dead card: it answers a
    // planeswalker outright, and everything else it might name is a bonus.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Vampire", "Shaman"], 2, 1).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Remove all counters from target permanent.",
            &[AbilityCostDef::SacrificeSource],
            // Any permanent, which is the point: what it takes off a planeswalker is
            // the loyalty, and a planeswalker with no loyalty is put into a graveyard
            // by the ordinary state-based action.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::RemoveAllCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: None,
            },
        ),
    ]),
);

// ZEN 115 — Vampire Lacerator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_LACERATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("114eca6c-76de-4b87-8174-78e2d17ad0e3"),
    "Vampire Lacerator",
    crate::card::CardArt::new("114eca6c-76de-4b87-8174-78e2d17ad0e3", "Steve Argyle"),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 119 — Burst Lightning
static BURST_LIGHTNING_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

pub(in crate::card::sets) static BURST_LIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2dc16614-5cf8-444d-a5ae-cac25018af68"),
    "Burst Lightning",
    CardArt::new("2dc16614-5cf8-444d-a5ae-cac25018af68", "Vance Kovacs"),
    CardSet::Zendikar,
    // One mana to answer what a one-drop deck leads with, and five to point
    // the same card at anything later. Kicking it is one whole cast rather
    // than a rider on the small one, so the size is settled as it goes on
    // the stack.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Kicker {4} (You may pay an additional {4} as you cast this spell.)\nBurst Lightning \
             deals 2 damage to any target. If this spell was kicked, it deals 4 damage instead.",
            &BURST_LIGHTNING_TARGET,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::kicker(
            mana_cost!("{4}{R}"),
            "Burst Lightning deals 4 damage to any target.",
            &BURST_LIGHTNING_TARGET,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// ZEN 125 — Goblin Bushwhacker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BUSHWHACKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4085a5bf-a71b-4c73-9b39-0dcc328fe11b"),
    "Goblin Bushwhacker",
    crate::card::CardArt::new("4085a5bf-a71b-4c73-9b39-0dcc328fe11b", "Mark Tedin"),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 168 — Lotus Cobra
pub(in crate::card::sets) static LOTUS_COBRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19adde22-e5eb-4815-beb6-c520b3274cc9"),
    "Lotus Cobra",
    CardArt::new("19adde22-e5eb-4815-beb6-c520b3274cc9", "Chippy"),
    CardSet::Zendikar,
    // Two mana that turns every land after it into a Lotus Petal, which is
    // what makes a fetchland a ritual.
    // Not a mana ability: it triggers off a land entering rather than off mana
    // being made (CR 605.1b), so it uses the stack, and the colour is named as
    // it resolves rather than when it triggers.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, add one mana of any color.",
            // A land you control, not any land: their fetchland does nothing for her.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ),
);

// ZEN 193 — Vines of Vastwood
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VINES_OF_VASTWOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8bd8b10-de86-4bb6-b49f-6ccb5297c81c"),
    "Vines of Vastwood",
    crate::card::CardArt::new(
        "e8bd8b10-de86-4bb6-b49f-6ccb5297c81c",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 197 — Blazing Torch
pub(in crate::card::sets) static BLAZING_TORCH: CardRecord = CardRecord::new_with_legacy_id(
    2314,
    "Blazing Torch",
    CardArt::new("1e9d1ff2-9ce3-4737-af1d-9fc82e4dffe6", "Vance Kovacs"),
    CardSet::Zendikar,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature can't be blocked by Vampires or Zombies.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype("Vampire"),
                            ObjectPredicateDef::Subtype("Zombie"),
                        ]),
                    )),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature has \"{T}, Sacrifice Blazing Torch: Blazing Torch deals 2 damage to any target.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}, Sacrifice Blazing Torch: Blazing Torch deals 2 damage to any target.",
                        &[
                            AbilityCostDef::TapSource,
                            AbilityCostDef::SacrificeObject(ObjectRefDef::AbilityGrantSource),
                        ],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )],
                        EffectDef::DealDamageFrom {
                            source: ObjectRefDef::AbilityGrantSource,
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(2),
                        },
                    )),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// ZEN 201 — Expedition Map
pub(in crate::card::sets) static EXPEDITION_MAP: CardRecord = CardRecord::new_with_legacy_id(
    2245,
    "Expedition Map",
    CardArt::new("c55bee97-593f-441f-b96c-a998d5212a55", "Franz Vohwinkel"),
    CardSet::Zendikar,
    // Three mana over two turns for any land in the deck, which is a
    // terrible rate and exactly what a deck built around one land wants.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{2}, {T}, Sacrifice this artifact: Search your library for a land card, reveal it, put \
         it into your hand, then shuffle.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Land),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// ZEN 211 — Arid Mesa
pub(in crate::card::sets) static ARID_MESA: CardRecord = CardRecord::new_with_legacy_id(
    2091,
    "Arid Mesa",
    CardArt::new("16c8d2fa-54a7-46e8-980c-905258497c90", "Raymond Swanland"),
    CardSet::Zendikar,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Mountain or Plains card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Mountain, BasicLandType::Plains],
    ),
);

// ZEN 219 — Marsh Flats
pub(in crate::card::sets) static MARSH_FLATS: CardRecord = CardRecord::new_with_legacy_id(
    2092,
    "Marsh Flats",
    CardArt::new("45026d57-0324-4312-8b86-2e7d4f581ee9", "Izzy"),
    CardSet::Zendikar,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Plains or Swamp card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Plains, BasicLandType::Swamp],
    ),
);

// ZEN 220 — Misty Rainforest
pub(in crate::card::sets) static MISTY_RAINFOREST: CardRecord = CardRecord::new_with_legacy_id(
    2093,
    "Misty Rainforest",
    CardArt::new("24a5cc2c-0fbf-4a5f-b175-6e0ffd0d0787", "Shelly Wan"),
    CardSet::Zendikar,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Forest or Island card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Forest, BasicLandType::Island],
    ),
);

// ZEN 223 — Scalding Tarn
pub(in crate::card::sets) static SCALDING_TARN: CardRecord = CardRecord::new_with_legacy_id(
    2094,
    "Scalding Tarn",
    CardArt::new("327cf118-cc92-4073-85d0-94d2a0a6989a", "Philip Straub"),
    CardSet::Zendikar,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for an Island or Mountain card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Island, BasicLandType::Mountain],
    ),
);

// ZEN 226 — Teetering Peaks
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEETERING_PEAKS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e56aca36-bb51-45e3-9ef9-9f9f2aa1e088"),
    "Teetering Peaks",
    crate::card::CardArt::new("e56aca36-bb51-45e3-9ef9-9f9f2aa1e088", "Fred Fields"),
    crate::card::CardSet::Zendikar,
    crate::card::CardRules::unsupported(),
);

// ZEN 229 — Verdant Catacombs
pub(in crate::card::sets) static VERDANT_CATACOMBS: CardRecord = CardRecord::new_with_legacy_id(
    2095,
    "Verdant Catacombs",
    CardArt::new("7abd2723-2851-4f1a-b2d0-dfcb526472c3", "Vance Kovacs"),
    CardSet::Zendikar,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Swamp or Forest card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Swamp, BasicLandType::Forest],
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &JOURNEY_TO_NOWHERE,
    &KOR_SKYFISHER,
    &INTO_THE_ROIL,
    &SPELL_PIERCE,
    &BLOODGHAST,
    &DISFIGURE,
    &VAMPIRE_HEXMAGE,
    &VAMPIRE_LACERATOR,
    &BURST_LIGHTNING,
    &GOBLIN_BUSHWHACKER,
    &LOTUS_COBRA,
    &VINES_OF_VASTWOOD,
    &BLAZING_TORCH,
    &EXPEDITION_MAP,
    &ARID_MESA,
    &MARSH_FLATS,
    &MISTY_RAINFOREST,
    &SCALDING_TARN,
    &TEETERING_PEAKS,
    &VERDANT_CATACOMBS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
