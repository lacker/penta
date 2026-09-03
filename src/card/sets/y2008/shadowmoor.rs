//! Shadowmoor cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, CopyStackObjectDef,
    CostQuantityDef, EffectDef, EffectRecipientDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    OptionalAdditionalCostAbilityDef, OptionalAdditionalCostKindDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    SpellResolutionDestinationDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::{AdditionalCostIndex, TargetIndex, mana_cost};

/// Conspire is one optional creature-tapping cast cost plus the cast trigger
/// that copies the spell when that cost was paid. Each card supplies the
/// creature predicate that shares one of its colors.
const fn conspire(
    spell: &'static AbilityDef,
    additional_cost: SpellAdditionalCostDef,
) -> [AbilityDef; 3] {
    [
        *spell,
        AbilityDef::optional_additional_cost(
            "Conspire (As you cast this spell, you may tap two untapped creatures you control that share a color with it.)",
            OptionalAdditionalCostAbilityDef {
                kind: OptionalAdditionalCostKindDef::Conspire,
                label: "Conspire",
                mana_cost: None,
                additional_cost: Some(additional_cost),
                resolution_destination: SpellResolutionDestinationDef::Graveyard,
            },
        ),
        AbilityDef::triggered_if(
            "When you conspire, copy this spell. You may choose new targets for the copy.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
            &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
            EffectDef::CopyStackObject(&CopyStackObjectDef {
                object: EffectRecipientDef::TriggeringObject,
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: true,
                colors: None,
            }),
        ),
    ]
}

// SHM 33 — Counterbore
pub(in crate::card::sets) static COUNTERBORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4228b80-d87d-4ebe-ae92-04e4a7d0dc43"),
    "Counterbore",
    CardArt::new("f4228b80-d87d-4ebe-ae92-04e4a7d0dc43", "Wayne England"),
    CardSet::Shadowmoor,
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target spell. Search its controller's graveyard, hand, and library for all cards with the same name as that spell and exile them. Then that player shuffles.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(Binding!("counterbore_target")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::Sequence(&[
                    EffectDef::Counter {
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(Binding!(
                            "counterbore_target"
                        ))),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(Binding!(
                            "counterbore_graveyard"
                        )),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::SharingNameWithIn {
                            reference: ObjectRefDef::Binding(Binding!("counterbore_target")),
                            objects: &ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Graveyard],
                                PlayerSetDef::One(PlayerRefDef::ControllerOf(
                                    ObjectRefDef::Binding(Binding!("counterbore_target")),
                                )),
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: usize::MAX,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                                "counterbore_graveyard"
                            ))),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                    }),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(Binding!("counterbore_hand")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::SharingNameWithIn {
                            reference: ObjectRefDef::Binding(Binding!("counterbore_target")),
                            objects: &ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerSetDef::One(PlayerRefDef::ControllerOf(
                                    ObjectRefDef::Binding(Binding!("counterbore_target")),
                                )),
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: usize::MAX,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                                "counterbore_hand"
                            ))),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                    }),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(Binding!(
                            "counterbore_library"
                        )),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::SharingNameWithIn {
                            reference: ObjectRefDef::Binding(Binding!("counterbore_target")),
                            objects: &ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Library],
                                PlayerSetDef::One(PlayerRefDef::ControllerOf(
                                    ObjectRefDef::Binding(Binding!("counterbore_target")),
                                )),
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: usize::MAX,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                                "counterbore_library"
                            ))),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                    }),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Binding(Binding!("counterbore_target")),
                        )),
                    },
                ]),
            }),
        ),
    ),
);

// SHM 57 — Beseech the Queen
pub(in crate::card::sets) static BESEECH_THE_QUEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64ee0a93-0f6d-42be-bdca-1de5422d8d54"),
    "Beseech the Queen",
    CardArt::new("64ee0a93-0f6d-42be-bdca-1de5422d8d54", "Jason Chan"),
    CardSet::Shadowmoor,
    CardRules::new_sorcery(mana_cost!("{2/B}{2/B}{2/B}")).with_ability(AbilityDef::spell(
        "Search your library for a card with mana value less than or equal to the number of lands you control, reveal it, put it into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::ManaValueAtMostValue(ValueDef::CountMatchingObjects(
                // The lands the caster controls when Beseech the Queen resolves.
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
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

// SHM 86 — Burn Trail
pub(in crate::card::sets) static BURN_TRAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f01f9a0-f1d0-4241-a270-df4ed673d1fd"),
    "Burn Trail",
    CardArt::new("7f01f9a0-f1d0-4241-a270-df4ed673d1fd", "Nils Hamm"),
    CardSet::Shadowmoor,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&conspire(
        &AbilityDef::spell_with_targets(
            "Burn Trail deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        SpellAdditionalCostDef::Tap {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]),
            quantity: CostQuantityDef::Fixed(2),
        },
    )),
);

// SHM 135 — Woodfall Primus
pub(in crate::card::sets) static WOODFALL_PRIMUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43aa7e35-55ee-4e02-a8aa-ea2b267055d1"),
    "Woodfall Primus",
    CardArt::new("43aa7e35-55ee-4e02-a8aa-ea2b267055d1", "Adam Rex"),
    CardSet::Shadowmoor,
    // Eight mana for two Naturalizes and a trampling body that has to be
    // answered twice.
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Treefolk", "Shaman"], 6, 6)
        .with_abilities(&[
            abilities::trample(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, destroy target noncreature permanent.",
                // A noncreature permanent: lands and artifacts above all, which is what
                // eight mana of Treefolk is being paid to answer twice.
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            abilities::persist(),
        ]),
);

// SHM 211 — Manamorphose
pub(in crate::card::sets) static MANAMORPHOSE: CardRecord = CardRecord::new_with_legacy_id(
    2238,
    "Manamorphose",
    CardArt::new("50283122-b8c4-4fb3-8eba-6252b72222f4", "Jeff Miracola"),
    CardSet::Shadowmoor,
    // It costs nothing and does nothing, which is the point: the deck that
    // wants it wants a spell that replaces itself and moves the storm count.
    CardRules::new_instant(mana_cost!("{1}{R/G}")).with_ability(AbilityDef::spell(
        "Add two mana in any combination of colors.\nDraw a card.",
        // "In any combination of colors" is one question per mana rather than one
        // for the pair, which is what lets it fix two colours at once.
        EffectDef::Sequence(&[
            EffectDef::AddMana(AddManaEffectDef::combination(
                &[
                    ManaColor::White,
                    ManaColor::Blue,
                    ManaColor::Black,
                    ManaColor::Red,
                    ManaColor::Green,
                ],
                2,
            )),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// SHM 224 — Barkshell Blessing
pub(in crate::card::sets) static BARKSHELL_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd273ef2-4aed-4c7e-8c97-fe8b1af9ce69"),
    "Barkshell Blessing",
    CardArt::new("cd273ef2-4aed-4c7e-8c97-fe8b1af9ce69", "Steven Belledin"),
    CardSet::Shadowmoor,
    CardRules::new_instant(mana_cost!("{G/W}")).with_abilities(&conspire(
        &AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        SpellAdditionalCostDef::Tap {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
            quantity: CostQuantityDef::Fixed(2),
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COUNTERBORE,
    &BESEECH_THE_QUEEN,
    &BURN_TRAIL,
    &WOODFALL_PRIMUS,
    &MANAMORPHOSE,
    &BARKSHELL_BLESSING,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
