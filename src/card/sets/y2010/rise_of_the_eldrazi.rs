//! Rise of the Eldrazi cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, KeywordAbility, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerSetDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// ROE 4 — Emrakul, the Aeons Torn
static EMRAKUL_GRAVEYARD_ZONES: [ZoneKind; 1] = [ZoneKind::Graveyard];

static EMRAKUL_SHUFFLES_GRAVEYARD: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Any,
            &[ZoneKind::Graveyard],
            PlayerSetDef::One(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
        ))),
        zone: ZoneKind::Library,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
    },
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
    },
];

static EMRAKUL_ABILITIES: [AbilityDef; 6] = [
    abilities::cannot_be_countered(),
    AbilityDef::triggered(
        "When you cast this spell, take an extra turn after this one.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
        EffectDef::TakeExtraTurn {
            player: EffectRecipientDef::Controller,
        },
    ),
    abilities::flying(),
    AbilityDef::keyword(
        "Protection from spells that are one or more colors",
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
            ObjectPredicateDef::Spell,
            ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
        ])),
    ),
    abilities::annihilator(6),
    AbilityDef::triggered(
        "When Emrakul is put into a graveyard from anywhere, its owner shuffles their graveyard into their library.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Graveyard)),
        EffectDef::Sequence(&EMRAKUL_SHUFFLES_GRAVEYARD),
    )
    .with_source_zones(&EMRAKUL_GRAVEYARD_ZONES),
];

pub(in crate::card::sets) static EMRAKUL_THE_AEONS_TORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67600383-bbb8-411c-b8e6-2296650bc747"),
    "Emrakul, the Aeons Torn",
    CardArt::new("67600383-bbb8-411c-b8e6-2296650bc747", "Mark Tedin"),
    CardSet::RiseOfTheEldrazi,
    CardRules::new_creature(mana_cost!("{15}"), &["Eldrazi"], 15, 15)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&EMRAKUL_ABILITIES),
);

// ROE 13 — Ulamog's Crusher
pub(in crate::card::sets) static ULAMOG_S_CRUSHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76bacedb-9fa8-4a21-b0eb-e7ead64360b4"),
    "Ulamog's Crusher",
    crate::card::CardArt::new("76bacedb-9fa8-4a21-b0eb-e7ead64360b4", "Todd Lockwood"),
    crate::card::CardSet::RiseOfTheEldrazi,
    CardRules::new_creature(mana_cost!("{8}"), &["Eldrazi"], 8, 8).with_abilities(&[
        abilities::annihilator(2),
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    ]),
);

// ROE 40 — Oust
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OUST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07313dd3-d0dc-40ca-98a3-fa4d39e5bcae"),
    "Oust",
    crate::card::CardArt::new("07313dd3-d0dc-40ca-98a3-fa4d39e5bcae", "Mike Bierek"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 61 — Domestication
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOMESTICATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1f15831-8dfd-4232-875c-efa6744c9a12"),
    "Domestication",
    crate::card::CardArt::new("e1f15831-8dfd-4232-875c-efa6744c9a12", "Jesper Ejsing"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 115 — Inquisition of Kozilek
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INQUISITION_OF_KOZILEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a3ff5c3-0fdb-4d54-b4e5-ce7bad9953f0"),
    "Inquisition of Kozilek",
    crate::card::CardArt::new("6a3ff5c3-0fdb-4d54-b4e5-ce7bad9953f0", "Tomasz Jedruszek"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 130 — Vendetta
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VENDETTA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67ced38e-0f33-4bda-8e18-09f6ac03a3d7"),
    "Vendetta",
    crate::card::CardArt::new("039fc76d-3b7e-4329-a997-07c25509e421", "Karl Kopinski"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 145 — Flame Slash
static FLAME_SLASH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

pub(in crate::card::sets) static FLAME_SLASH: CardRecord = CardRecord::new_with_legacy_id(
    2184,
    "Flame Slash",
    CardArt::new("006d2bf1-20f7-4b09-8d98-8233d91682bd", "Raymond Swanland"),
    CardSet::RiseOfTheEldrazi,
    // One mana for four damage is the best rate in the format; the sorcery
    // speed is the whole price, and it cannot go upstairs.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Flame Slash deals 4 damage to target creature.",
        &FLAME_SLASH_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

// ROE 161 — Raid Bombardment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAID_BOMBARDMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c2d1a48-efde-4134-95f0-b23f6cf85259"),
    "Raid Bombardment",
    crate::card::CardArt::new("9c2d1a48-efde-4134-95f0-b23f6cf85259", "Matt Cavotta"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 201 — Nest Invader
static ELDRAZI_SPAWN_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
    "Sacrifice this creature: Add {C}.",
    &[AbilityCostDef::SacrificeSource],
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
)];

pub(in crate::card::sets) static NEST_INVADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24517d9c-6cde-41e8-9e82-ee73f069379a"),
    "Nest Invader",
    CardArt::new("24517d9c-6cde-41e8-9e82-ee73f069379a", "Trevor Claxton"),
    CardSet::RiseOfTheEldrazi,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Eldrazi", "Drone"], 2, 2).with_ability(
        abilities::enters_trigger("When this creature enters, create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\"", EffectDef::create_creature_token(&["Eldrazi", "Spawn"], &[], 0, 1)
                .with_abilities(&ELDRAZI_SPAWN_ABILITIES)
                .with_art(CardArt::new(
                    "d0da4f8d-cce9-4d08-8d11-792e0b2af7d0",
                    "Véronique Meignaud",
                ))),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EMRAKUL_THE_AEONS_TORN,
    &ULAMOG_S_CRUSHER,
    &OUST,
    &DOMESTICATION,
    &INQUISITION_OF_KOZILEK,
    &VENDETTA,
    &FLAME_SLASH,
    &RAID_BOMBARDMENT,
    &NEST_INVADER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
