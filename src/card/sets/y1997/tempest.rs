//! Tempest cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, EffectDef, EffectDurationDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    PlayerRelation, ReplacementEffectDef, ReplacementEventDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, cards,
};
use crate::{TargetIndex, mana_cost};

static QUICKENING_LICID_END: AbilityDef = AbilityDef::special_action(
    "You may pay {W} to end this effect.",
    &[ZoneKind::Battlefield],
    &[AbilityCostDef::Mana(mana_cost!("{W}"))],
    EffectDef::EndAuraEffect,
);

// TMP 36 — Quickening Licid
pub(in crate::card::sets) static QUICKENING_LICID: CardRecord = CardRecord::new(
    cards::QUICKENING_LICID,
    "Quickening Licid",
    CardArt::new(
        "e6e91f3d-5a23-4df1-a879-d18a3af92a28",
        "Andrew Robinson",
    ),
    CardSet::Tempest,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Licid"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}, {T}: This creature loses this ability and becomes an Aura enchantment with enchant creature. Attach it to target creature. You may pay {W} to end this effect.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::BecomeAuraAndAttach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                end: &QUICKENING_LICID_END,
            },
        ),
        AbilityDef::static_ability(
            "Enchanted creature has first strike.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// TMP 51 — Warmth
pub(in crate::card::sets) static WARMTH: CardRecord = CardRecord::new(
    cards::WARMTH,
    "Warmth",
    CardArt::new("d7dbeea8-06b0-4482-bdae-aa82b9db8856", "Drew Tucker"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a red spell, you gain 2 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::Color(ManaColor::Red),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// TMP 151 — Reanimate
pub(in crate::card::sets) static REANIMATE: CardRecord = CardRecord::new(
    cards::REANIMATE,
    "Reanimate",
    CardArt::new("fc00f897-988b-4602-969a-c510804ec12a", "Robert Bliss"),
    CardSet::Tempest,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature card from a graveyard onto the battlefield under your control. You lose life equal to that card's mana value.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: None,
        })],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                controller: Some(PlayerRelation::You),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// TMP 183 — Jackal Pup
pub(in crate::card::sets) static JACKAL_PUP: CardRecord = CardRecord::new(
    cards::JACKAL_PUP,
    "Jackal Pup",
    CardArt::new("3707ab74-9aec-4d30-86e0-ffa5f72d5b4f", "Susan Van Camp"),
    CardSet::Tempest,
    CardRules::new_creature(mana_cost!("{R}"), &["Jackal"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, it deals that much damage to you.",
            TriggerEventDef::DamageDealt {
                source: ObjectPredicateDef::Any,
                recipient: EffectRecipientDef::Source,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// TMP 190 — Mogg Fanatic
pub(in crate::card::sets) static MOGG_FANATIC: CardRecord = CardRecord::new(
    cards::MOGG_FANATIC,
    "Mogg Fanatic",
    CardArt::new("ca2ecfd4-c874-4468-8601-87aa110d5a00", "Brom"),
    CardSet::Tempest,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: It deals 1 damage to any target.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 250 — Root Maze
pub(in crate::card::sets) static ROOT_MAZE: CardRecord = CardRecord::new(
    cards::ROOT_MAZE,
    "Root Maze",
    CardArt::new("99a12b74-f191-4362-81ab-77590ae5e68f", "Rebecca Guay"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::replacement_for(
        "Artifacts and lands enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            controller: PlayerRelation::Any,
        },
        EffectDef::Replacement(ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::Tapped,
        )),
    )),
);

// TMP 294 — Lotus Petal
pub(in crate::card::sets) static LOTUS_PETAL: CardRecord = CardRecord::new(
    cards::LOTUS_PETAL,
    "Lotus Petal",
    CardArt::new("6c877da3-68fa-41d0-8a24-8c79fcd8ecc1", "April Lee"),
    CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_mana(
        "{T}, Sacrifice this artifact: Add one mana of any color.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// TMP 315 — Ancient Tomb
pub(in crate::card::sets) static ANCIENT_TOMB: CardRecord = CardRecord::new(
    cards::ANCIENT_TOMB,
    "Ancient Tomb",
    CardArt::new("30e401e3-282b-4524-87e1-c6cd50cd6d00", "Colin MacNeil"),
    CardSet::Tempest,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add {C}{C}. This land deals 2 damage to you.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless)
                .with_amount(2)
                .with_damage_to_controller(2),
        ),
    )),
);

// TMP 330 — Wasteland
pub(in crate::card::sets) static WASTELAND: CardRecord = CardRecord::new(
    cards::WASTELAND,
    "Wasteland",
    CardArt::new("99ff731b-8399-40c8-b539-ba6ba5783771", "Una Fricker"),
    CardSet::Tempest,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Destroy target nonbasic land.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &QUICKENING_LICID,
    &WARMTH,
    &REANIMATE,
    &JACKAL_PUP,
    &MOGG_FANATIC,
    &ROOT_MAZE,
    &LOTUS_PETAL,
    &ANCIENT_TOMB,
    &WASTELAND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
