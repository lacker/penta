//! Darksteel cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::ManaColor;
use crate::card::{
    AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef, CardArt, CardNameDef,
    CardRules, CardSet, CardType, CostDef, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, ObjectSetFilterDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

// DST 21 — Echoing Truth
pub(in crate::card::sets) static ECHOING_TRUTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4aefedd7-1bf5-4148-9084-d7d8f1138140"),
    "Echoing Truth",
    CardArt::new("4aefedd7-1bf5-4148-9084-d7d8f1138140", "Greg Staples"),
    CardSet::Darksteel,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent and all other permanents with the same name as that permanent to their owners' hands.",
        &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
            &ObjectPredicateDef::HasType(CardType::Land),
        ))],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::objects(ObjectSetDef::Union(&[
                ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                ObjectSetDef::Matching {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Battlefield],
                    )),
                    object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameEquals(
                        CardNameDef::NameOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    )),
                },
            ])),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// DST 59 — Echoing Ruin
pub(in crate::card::sets) static ECHOING_RUIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01f842c8-cd6c-4f4d-9aa8-417a92b37867"),
    "Echoing Ruin",
    CardArt::new("01f842c8-cd6c-4f4d-9aa8-417a92b37867", "Greg Staples"),
    CardSet::Darksteel,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact and all other artifacts with the same name as that artifact.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::objects(ObjectSetDef::Union(&[
                ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                ObjectSetDef::Matching {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                    )),
                    object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameEquals(
                        CardNameDef::NameOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    )),
                },
            ])),
            then: None,
        },
    )),
);

// DST 74 — Echoing Courage
pub(in crate::card::sets) static ECHOING_COURAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f37bf33-f340-44eb-a092-1cb9385c8981"),
    "Echoing Courage",
    CardArt::new("6f37bf33-f340-44eb-a092-1cb9385c8981", "Greg Staples"),
    CardSet::Darksteel,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature and all other creatures with the same name as that creature get +2/+2 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Union(&[
                ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                ObjectSetDef::Matching {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                    )),
                    object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameEquals(
                        CardNameDef::NameOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    )),
                },
            ])),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DST 112 — Darksteel Ingot
pub(in crate::card::sets) static DARKSTEEL_INGOT: CardRecord = CardRecord::new_with_legacy_id(
    263,
    "Darksteel Ingot",
    CardArt::new("b02b9634-77e9-48ae-a6bf-859598d12c52", "Martina Pilcerova"),
    CardSet::Darksteel,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// DST 127 — Leonin Bola
// Audit: unsupported — Needs unattaching the Equipment as an activation cost. EffectDef::Unattach exists but only as an effect, and CostDef has no unattach variant; Special is a marker the runtime rejects rather than a general escape hatch. Paying only the tap would make the ability repeatable, which is the opposite of what the card does.
pub(in crate::card::sets) static LEONIN_BOLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7eab112-20a6-414f-84c9-678580485420"),
    "Leonin Bola",
    crate::card::CardArt::new(
        "a7eab112-20a6-414f-84c9-678580485420",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Darksteel,
    crate::card::CardRules::unsupported(),
);

// DST 138 — Serum Powder
pub(in crate::card::sets) static SERUM_POWDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8330afd6-f43a-4955-a704-8f2b963cd0c6"),
    "Serum Powder",
    CardArt::new("8330afd6-f43a-4955-a704-8f2b963cd0c6", "Matt Thompson"),
    CardSet::Darksteel,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::mulligan_action(
            "Any time you could mulligan and this card is in your hand, you may exile all the cards from your hand, then draw that many cards. (You can do this in addition to taking mulligans.)",
            abilities::bind_objects_then(
                crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    ),
                )),
                &EffectDef::Sequence(&[
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            ParentBinding,
                        )),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::BoundObjectCount(ParentBinding),
                    },
                ]),
            ),
        ),
    ]),
);

// DST 140 — Skullclamp
pub(in crate::card::sets) static SKULLCLAMP: CardRecord = CardRecord::new_with_legacy_id(
    2120,
    "Skullclamp",
    CardArt::new("55318397-de3c-47ea-a088-72a24df5c8fa", "Luca Zontini"),
    CardSet::Darksteel,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            // The minus is the whole card: a one-toughness creature it is
            // attached to dies to state-based actions rather than to anything
            // the Clamp does on purpose, and the trigger below collects.
            AbilityDef::static_ability(
                "Equipped creature gets +1/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever equipped creature dies, draw two cards.",
                // The creature is already in the graveyard and the Clamp
                // already unattached by the time this is collected, so the
                // attachment it names is last-known information.
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// DST 157 — Vulshok Morningstar
pub(in crate::card::sets) static VULSHOK_MORNINGSTAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("acf00de0-af24-4ef9-8ac2-135e6b53a8fd"),
    "Vulshok Morningstar",
    CardArt::new("acf00de0-af24-4ef9-8ac2-135e6b53a8fd", "David Martin"),
    CardSet::Darksteel,
    // Four mana across two turns for +2/+2, and the toughness is what
    // separates it from Bonesplitter in a deck that has to block.
    CardRules::new_artifact(mana_cost!("{2}"))
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
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ECHOING_TRUTH,
    &ECHOING_RUIN,
    &ECHOING_COURAGE,
    &DARKSTEEL_INGOT,
    &LEONIN_BOLA,
    &SERUM_POWDER,
    &SKULLCLAMP,
    &VULSHOK_MORNINGSTAR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
