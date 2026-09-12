//! Commander 2011 card records required by supported formats.

use crate::card::BindObjectsDef;
use crate::card::ControlDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::RevealObjectsDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::CardType;
use crate::CounterKind;
use crate::TargetConditionDef;
use crate::TargetIndex;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::TriggerConditionDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CMD",
    slug: "commander-2011",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CMD 46 — Flusterstorm
pub(in crate::card::sets) static FLUSTERSTORM_46: CardRecord = CardRecord::new(
    "Flusterstorm",
    "1e2e09bf-e7c8-4f13-bcee-f9c8cbc57993",
    "Erica Yang",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target instant or sorcery spell unless its controller pays {1}.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(&[CostDef::Mana(mana_cost!("{1}"))]),
        ),
        abilities::storm(),
    ]),
);

// CMD 114 — Chaos Warp
pub(in crate::card::sets) static CHAOS_WARP_114: CardRecord = CardRecord::new(
    "Chaos Warp",
    "042431bc-0b21-4920-802f-6dd02e4c8721",
    "Trevor Claxton",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
AbilityDef::spell_with_targets("The owner of target permanent shuffles it into their library, then reveals the top card of their library. If it's a permanent card, they put it onto the battlefield.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)], EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Library, ZonePlacement::Top), EffectDef::ShuffleLibrary { player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Target(TargetIndex::PRIMARY))) }, EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::OwnerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)), count: ValueDef::Constant(1) }, binding: Binding!("warp_revealed"), then: &EffectDef::Sequence(&[EffectDef::RevealObjects(RevealObjectsDef { input: ObjectSetDef::Binding(Binding!("warp_revealed")), then: &EffectDef::None }), EffectDef::ForEachInBinding { objects: Binding!("warp_revealed"), binding: Binding!("warp_card"), effect: &EffectDef::IfCondition { condition: &TriggerConditionDef::BoundObjectMatches { binding: Binding!("warp_card"), object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Enchantment), ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::HasType(CardType::Planeswalker)]) }, then: &EffectDef::move_to_zone(EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("warp_card"))), ZoneKind::Battlefield, ZonePlacement::Top) } }]) })]))
]),
);

// CMD 170 — Scavenging Ooze
/// One when the exiled card was a creature, nothing otherwise.
static EXILED_A_CREATURE: TargetConditionDef = TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    then: ValueDef::Constant(1),
    otherwise: ValueDef::Constant(0),
};

pub(in crate::card::sets) static SCAVENGING_OOZE: CardRecord = CardRecord::new(
    "Scavenging Ooze",
    "371ceb58-f498-4616-a7f0-eb118fe2e4ff",
    "Austin Hsu",
CardRules::new_creature(
        mana_cost!("{1}{G}"),
        &["Ooze"],
        2,
        2,
    )
    .with_ability(
        AbilityDef::activated_with_targets("{G}: Exile target card from a graveyard. If it was a creature card, put a +1/+1 counter on this creature and you gain 1 life.", &[CostDef::Mana(mana_cost!("{G}"))], &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )], // The counter and the life come first so the card is still in the
            // graveyard to be asked what it was. Exiling it first would leave
            // nothing to look at, and nothing here can observe the order.
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::IfTargetMatches(&EXILED_A_CREATURE),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::IfTargetMatches(&EXILED_A_CREATURE),
                },
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
),
            ])),
    ),
);

// CMD 244 — Champion's Helm
pub(in crate::card::sets) static CHAMPIONS_HELM: CardRecord = CardRecord::new(
    "Champion's Helm",
    "dcad6846-0b35-4193-b647-16e597357f9b",
    "Alan Pollack",
CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is legendary, it has hexproof. (It can't be the target of spells or abilities your opponents control.)",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    },
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// CMD 269 — Command Tower
// Audit: unsupported — Commander designation is recorded, but mana selectors cannot derive the commander pair’s color identity (including rules-text mana symbols and reverse faces).
pub(in crate::card::sets) static COMMAND_TOWER_269: CardRecord = CardRecord::new(
    "Command Tower",
    "46982091-cc78-4171-8b3d-d07592684728",
    "Ryan Yee",
    crate::card::CardRules::unsupported(),
);

// CMD 277 — Homeward Path
pub(in crate::card::sets) static HOMEWARD_PATH_277: CardRecord = CardRecord::new(
    "Homeward Path",
    "b5fb67ed-f4ea-47d6-876a-2ad6a3fc9a18",
    "Tomasz Jedruszek",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{T}: Each player gains control of all creatures they own.",
            &[CostDef::TapSource],
            EffectDef::BindObjects(crate::card::BindObjectsDef {
                source: crate::card::ObjectCollectionSourceDef::ObjectSet(
                    crate::card::ObjectSetDef::Query(crate::card::ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                ),
                binding: crate::Binding!("owned_creatures"),
                then: &EffectDef::ForEachInBinding {
                    objects: crate::Binding!("owned_creatures"),
                    binding: crate::Binding!("owned_creature"),
                    effect: &EffectDef::gain_control(
                        EffectRecipientDef::object(crate::card::ObjectRefDef::Binding(
                            crate::Binding!("owned_creature"),
                        )),
                        PlayerRefDef::OwnerOf(crate::card::ObjectRefDef::Binding(crate::Binding!(
                            "owned_creature"
                        ))),
                        ControlDurationDef::Indefinitely,
                    ),
                },
            }),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FLUSTERSTORM_46,
    &CHAOS_WARP_114,
    &SCAVENGING_OOZE,
    &CHAMPIONS_HELM,
    &COMMAND_TOWER_269,
    &HOMEWARD_PATH_277,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
