//! Darksteel cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::CardType;
use crate::TargetIndex;
use crate::card::ManaColor;
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, AppliedEffectDef, CardRules, CardSet, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRelation,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ParentBinding;
use crate::mana_cost;

// DST 43 — Essence Drain
pub(in crate::card::sets) static ESSENCE_DRAIN: CardRecord = CardRecord::new(
    CardSet::Darksteel,
    "Essence Drain",
    "9950052e-f674-4f09-802e-3f5f52f5e717",
    "Tony Szczudlo",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Essence Drain deals 3 damage to any target and you gain 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// DST 92 — Angel's Feather
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGEL_S_FEATHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Darksteel,
    "Angel's Feather",
    "4a11d101-2e82-42d5-b4a1-8f0c520441ab",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// DST 110 — Darksteel Forge
pub(in crate::card::sets) static DARKSTEEL_FORGE: CardRecord = CardRecord::new(
    CardSet::Darksteel,
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
    CardSet::Darksteel,
    "Darksteel Ingot",
    "b02b9634-77e9-48ae-a6bf-859598d12c52",
    "Martina Pilcerova",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// DST 116 — Demon's Horn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMON_S_HORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Darksteel,
    "Demon's Horn",
    "41d40eb4-643a-4e22-a15f-eda45a48cfd6",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// DST 117 — Dragon's Claw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_S_CLAW: CardRecord = CardRecord::new(
    crate::card::CardSet::Darksteel,
    "Dragon's Claw",
    "7a46bbcc-b287-47bb-b252-5dd3217f61a9",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// DST 126 — Kraken's Eye
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAKEN_S_EYE: CardRecord = CardRecord::new(
    crate::card::CardSet::Darksteel,
    "Kraken's Eye",
    "cc767637-627a-4ea2-873b-d8a80ccc925b",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// DST 127 — Leonin Bola
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONIN_BOLA: CardRecord = CardRecord::new(
    crate::card::CardSet::Darksteel,
    "Leonin Bola",
    "a7eab112-20a6-414f-84c9-678580485420",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// DST 138 — Serum Powder
pub(in crate::card::sets) static SERUM_POWDER: CardRecord = CardRecord::new(
    CardSet::Darksteel,
    "Serum Powder",
    "8330afd6-f43a-4955-a704-8f2b963cd0c6",
    "Matt Thompson",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
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
pub(in crate::card::sets) static SKULLCLAMP: CardRecord = CardRecord::new(
    CardSet::Darksteel,
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
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// DST 157 — Vulshok Morningstar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VULSHOK_MORNINGSTAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Darksteel,
    "Vulshok Morningstar",
    "acf00de0-af24-4ef9-8ac2-135e6b53a8fd",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// DST 162 — Wurm's Tooth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WURM_S_TOOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Darksteel,
    "Wurm's Tooth",
    "482cdbe0-b865-4e09-bd30-61ab93739b53",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ESSENCE_DRAIN,
    &ANGEL_S_FEATHER,
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
