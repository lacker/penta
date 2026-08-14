//! Ice Age cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules,
    CardSet, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ValueDef, abilities,
    cards,
};
use crate::{TargetIndex, mana_cost};

// ICE 72 — Hydroblast
pub(in crate::card::sets) static HYDROBLAST: CardRecord = CardRecord::new(
    cards::HYDROBLAST,
    "Hydroblast",
    CardArt::new("f62716f0-fde2-49ef-b8a4-c1b03f451194", "Kaja Foglio"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target spell if it's red.\n• Destroy target permanent if it's red.",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's red",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Red)),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's red",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Red,
                )),
                true,
            ),
        ],
    )),
);

// ICE 194 — Incinerate
pub(in crate::card::sets) static INCINERATE: CardRecord = CardRecord::new(
    cards::INCINERATE,
    "Incinerate",
    CardArt::new("9c3f00af-010d-4485-b8b7-47400d99c496", "Mark Poole"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Incinerate deals 3 damage to any target. A creature dealt damage this way can't be regenerated this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The prohibition exists, but applying it only when this spell actually dealt damage to a creature needs damage-result linkage.",
        )),
    ),
);

// ICE 213 — Pyroblast
pub(in crate::card::sets) static PYROBLAST: CardRecord = CardRecord::new(
    cards::PYROBLAST,
    "Pyroblast",
    CardArt::new("c342cac5-08ae-4428-9c2c-f6c5904e54d2", "Kaja Foglio"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target spell if it's blue.\n• Destroy target permanent if it's blue.",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's blue",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Blue)),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's blue",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Blue,
                )),
                true,
            ),
        ],
    )),
);

// ICE 351 — Adarkar Wastes
pub(in crate::card::sets) static ADARKAR_WASTES: CardRecord = CardRecord::new(
    cards::ADARKAR_WASTES,
    "Adarkar Wastes",
    CardArt::new("09dd9023-f7ee-4e99-8821-7059deb83730", "Mike Raabe"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {W} or {U}. This land deals 1 damage to you.",
        &[ManaColor::White, ManaColor::Blue],
    )),
);

// ICE 356 — Karplusan Forest
pub(in crate::card::sets) static KARPLUSAN_FOREST: CardRecord = CardRecord::new(
    cards::KARPLUSAN_FOREST,
    "Karplusan Forest",
    CardArt::new("ba6f1263-d598-49fb-b5f8-09f11822ebd0", "Nicola Leonard"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {R} or {G}. This land deals 1 damage to you.",
        &[ManaColor::Red, ManaColor::Green],
    )),
);

// ICE 362 — Underground River
pub(in crate::card::sets) static UNDERGROUND_RIVER: CardRecord = CardRecord::new(
    cards::UNDERGROUND_RIVER,
    "Underground River",
    CardArt::new("92369d7e-5e5a-46f9-bb31-c57d62410283", "NéNé Thomas"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {U} or {B}. This land deals 1 damage to you.",
        &[ManaColor::Blue, ManaColor::Black],
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HYDROBLAST,
    &INCINERATE,
    &PYROBLAST,
    &ADARKAR_WASTES,
    &KARPLUSAN_FOREST,
    &UNDERGROUND_RIVER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
