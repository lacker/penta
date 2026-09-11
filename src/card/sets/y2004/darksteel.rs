//! Darksteel cards cataloged as cross-format rules-engine test cases.

use super::CardRecord;
use super::PrintingRecord;
use crate::AbilityTargetPredicate;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardNameDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DST",
    slug: "darksteel",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DST 21 — Echoing Truth
pub(in crate::card::sets) static ECHOING_TRUTH: CardRecord = CardRecord::new(
    "Echoing Truth",
    "4aefedd7-1bf5-4148-9084-d7d8f1138140",
    "Greg Staples",
CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent and all other permanents with the same name as that permanent to their owners' hands.",
        &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
            &ObjectPredicateDef::HasType(CardType::Land),
        ))],
        EffectDef::move_to_zone(
            EffectRecipientDef::objects(ObjectSetDef::Union(&[
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
            ZoneKind::Hand,
            ZonePlacement::Top,
        ),
    )),
);

// DST 43 — Essence Drain
pub(in crate::card::sets) static ESSENCE_DRAIN: CardRecord = CardRecord::new(
    "Essence Drain",
    "9950052e-f674-4f09-802e-3f5f52f5e717",
    "Tony Szczudlo",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Essence Drain deals 3 damage to any target and you gain 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// DST 59 — Echoing Ruin
pub(in crate::card::sets) static ECHOING_RUIN: CardRecord = CardRecord::new(
    "Echoing Ruin",
    "01f842c8-cd6c-4f4d-9aa8-417a92b37867",
    "Greg Staples",
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
    "Echoing Courage",
    "6f37bf33-f340-44eb-a092-1cb9385c8981",
    "Greg Staples",
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

// DST 92 — Angel's Feather
pub(in crate::card::sets) static ANGEL_S_FEATHER: CardRecord = CardRecord::new(
    "Angel's Feather",
    "4a11d101-2e82-42d5-b4a1-8f0c520441ab",
    "Alan Pollack",
    // A sideboard card that only reads well against one colour, which is
    // exactly what the cycle was printed for.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a white spell, you may gain 1 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::White)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// DST 109 — Darksteel Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARKSTEEL_COLOSSUS: CardRecord = CardRecord::new(
    "Darksteel Colossus",
    "cbc27b24-f085-48b0-8757-cd11fbf25b91",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// DST 110 — Darksteel Forge
pub(in crate::card::sets) static DARKSTEEL_FORGE: CardRecord = CardRecord::new(
    "Darksteel Forge",
    "99078ecc-f50a-43e0-93c1-63240cd97bf7",
    "Martina Pilcerova",
    CardRules::new_artifact(mana_cost!("{9}")).with_ability(AbilityDef::static_ability(
        "Artifacts you control have indestructible.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
        },
    )),
);

// DST 112 — Darksteel Ingot
pub(in crate::card::sets) static DARKSTEEL_INGOT: CardRecord = CardRecord::new(
    "Darksteel Ingot",
    "b02b9634-77e9-48ae-a6bf-859598d12c52",
    "Martina Pilcerova",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// DST 116 — Demon's Horn
pub(in crate::card::sets) static DEMON_S_HORN: CardRecord = CardRecord::new(
    "Demon's Horn",
    "41d40eb4-643a-4e22-a15f-eda45a48cfd6",
    "Alan Pollack",
    // The black member of the same cycle, and the one most likely to be
    // triggering several times a turn.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a black spell, you may gain 1 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Black)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// DST 117 — Dragon's Claw
pub(in crate::card::sets) static DRAGON_S_CLAW: CardRecord = CardRecord::new(
    "Dragon's Claw",
    "7a46bbcc-b287-47bb-b252-5dd3217f61a9",
    "Alan Pollack",
    // Against burn this is the card that turns a race into a grind, one life at
    // a time.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a red spell, you may gain 1 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Red)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// DST 126 — Kraken's Eye
pub(in crate::card::sets) static KRAKEN_S_EYE: CardRecord = CardRecord::new(
    "Kraken's Eye",
    "cc767637-627a-4ea2-873b-d8a80ccc925b",
    "Alan Pollack",
    // The least useful of the five in practice, because the deck it is aimed at
    // wins without dealing damage at all.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a blue spell, you may gain 1 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Blue)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// DST 127 — Leonin Bola
// Audit: unsupported — Needs unattaching the Equipment as an activation cost. EffectDef::Unattach exists but only as an effect, and CostDef has no unattach variant; Special is a marker the runtime rejects rather than a general escape hatch. Paying only the tap would make the ability repeatable, which is the opposite of what the card does.
pub(in crate::card::sets) static LEONIN_BOLA: CardRecord = CardRecord::new(
    "Leonin Bola",
    "a7eab112-20a6-414f-84c9-678580485420",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// DST 138 — Serum Powder
pub(in crate::card::sets) static SERUM_POWDER: CardRecord = CardRecord::new(
    "Serum Powder",
    "8330afd6-f43a-4955-a704-8f2b963cd0c6",
    "Matt Thompson",
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
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(
                            ParentBinding,
                        )),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
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
pub(in crate::card::sets) static SKULLCLAMP: CardRecord = CardRecord::new(
    "Skullclamp",
    "55318397-de3c-47ea-a088-72a24df5c8fa",
    "Luca Zontini",
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
    "Vulshok Morningstar",
    "acf00de0-af24-4ef9-8ac2-135e6b53a8fd",
    "David Martin",
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

// DST 162 — Wurm's Tooth
pub(in crate::card::sets) static WURM_S_TOOTH: CardRecord = CardRecord::new(
    "Wurm's Tooth",
    "482cdbe0-b865-4e09-bd30-61ab93739b53",
    "Alan Pollack",
    // Two mana that quietly undoes a whole turn of green beats, provided the
    // green deck keeps casting things.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a green spell, you may gain 1 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Green)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ECHOING_TRUTH,
    &ESSENCE_DRAIN,
    &ECHOING_RUIN,
    &ECHOING_COURAGE,
    &ANGEL_S_FEATHER,
    &DARKSTEEL_COLOSSUS,
    &DARKSTEEL_FORGE,
    &DARKSTEEL_INGOT,
    &DEMON_S_HORN,
    &DRAGON_S_CLAW,
    &KRAKEN_S_EYE,
    &LEONIN_BOLA,
    &SERUM_POWDER,
    &SKULLCLAMP,
    &VULSHOK_MORNINGSTAR,
    &WURM_S_TOOTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
