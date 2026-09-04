//! Mirage cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::CardSupertype;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1995::homelands as catalog_hml;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef,
    EffectDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

/// The original slow fetchlands enter tapped and trade themselves for either
/// of two land types without charging life.
const fn slow_fetch_land_ability(
    text: &'static str,
    land_types: &'static [crate::card::BasicLandType],
) -> AbilityDef {
    AbilityDef::activated(
        text,
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(land_types),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )
}

// MIR 1 — Afterlife
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AFTERLIFE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Afterlife",
    "4644694d-52e6-4d00-8cad-748899eeea84",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MIR 2 — Alarum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALARUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Alarum",
    "155f2aa6-6c47-4a06-b0ef-2d9205cd133e",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// MIR 3 — Auspicious Ancestor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUSPICIOUS_ANCESTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Auspicious Ancestor",
    "7f6f9ec3-6033-4cd4-a52e-31a559559a93",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// MIR 4 — Benevolent Unicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENEVOLENT_UNICORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Benevolent Unicorn",
    "2a243bd7-af98-4e44-af6e-3b0b71d4837b",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// MIR 5 — Blinding Light
pub(in crate::card::sets) static BLINDING_LIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Blinding Light",
    "99b2192a-78a5-4579-94ce-cccf773a809d",
    "Hannibal King",
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Tap all nonwhite creatures.",
        EffectDef::Tap {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
        },
    )),
);

// MIR 6 — Celestial Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_DAWN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Celestial Dawn",
    "89fb4c4a-ccdd-4f3c-80cf-356ab7836e16",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// MIR 7 — Civic Guildmage
pub(in crate::card::sets) static CIVIC_GUILDMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Civic Guildmage",
    "a9319039-db2f-47bf-9ef0-8d3a381d54fb",
    "Andrew Robinson",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}, {T}: Target creature gets +0/+1 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{U}, {T}: Put target creature you control on top of its owner's library.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// MIR 8 — Dazzling Beauty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAZZLING_BEAUTY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Dazzling Beauty",
    "ad0ece98-5506-4a18-b900-8d1a6cd87385",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// MIR 9 — Disempower
pub(in crate::card::sets) static DISEMPOWER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Disempower",
    "3a86a2ae-aaf3-4d4d-ae06-ec3d4a539550",
    "John Matson",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Put target artifact or enchantment on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// MIR 10 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "71e27cbc-d986-437a-9195-6f5e065a49a2",
    "Charles Gillespie",
);

// MIR 11 — Divine Offering (reprint)
const DIVINE_OFFERING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DIVINE_OFFERING,
    "4b84697e-d72d-4169-bf16-753144da281f",
    "Terese Nielsen",
);

// MIR 12 — Divine Retribution
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVINE_RETRIBUTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Divine Retribution",
    "75629aa3-426e-4e25-a7ab-71e03436e061",
    "Charles Gillespie",
    crate::card::CardRules::unsupported(),
);

// MIR 13 — Ekundu Griffin
pub(in crate::card::sets) static EKUNDU_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ekundu Griffin",
    "3a32778b-e6ff-45ca-8b22-dc97a406faa4",
    "David A. Cherry",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// MIR 14 — Enlightened Tutor
pub(in crate::card::sets) static ENLIGHTENED_TUTOR: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Enlightened Tutor",
    "cbac1d27-15e2-4e2f-82ab-625a16e096cb",
    "Dan Frazier",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Search your library for an artifact or enchantment card, reveal it, then shuffle and put that card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// MIR 15 — Ethereal Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ETHEREAL_CHAMPION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ethereal Champion",
    "88f2d15e-490b-4754-8197-ac91653698f7",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MIR 16 — Favorable Destiny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAVORABLE_DESTINY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Favorable Destiny",
    "e5657403-7c86-4eb6-84b0-75eedb04a5a2",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// MIR 17 — Femeref Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEMEREF_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Femeref Healer",
    "90dedd5e-e2ee-46ec-8541-27f1548b2a2a",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// MIR 18 — Femeref Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEMEREF_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Femeref Knight",
    "915a2e07-b449-4d94-93e3-e756e891c542",
    "Tony Roberts",
    crate::card::CardRules::unsupported(),
);

// MIR 19 — Femeref Scouts
pub(in crate::card::sets) static FEMEREF_SCOUTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Femeref Scouts",
    "60192ded-689b-4cc5-9293-bff52924089b",
    "Zak Plucinski",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Scout"], 1, 4),
);

// MIR 20 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "0d1c9ca9-3611-46df-8108-45ba383b4ee6",
    "Hannibal King",
);

// MIR 21 — Illumination
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUMINATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Illumination",
    "eb28f6e5-c9ef-416e-b315-967d857e7600",
    "David O'Connor",
    crate::card::CardRules::unsupported(),
);

// MIR 22 — Iron Tusk Elephant
pub(in crate::card::sets) static IRON_TUSK_ELEPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Iron Tusk Elephant",
    "d7c8e952-f040-4e5b-88f3-f80ad4b3f2f1",
    "Tony Roberts",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Elephant"], 3, 3)
        .with_ability(abilities::trample()),
);

// MIR 23 — Ivory Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IVORY_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ivory Charm",
    "0706acf6-587e-4f29-944a-fdf25aeacb6d",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// MIR 24 — Jabari's Influence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JABARI_S_INFLUENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Jabari's Influence",
    "c9f764d7-4c18-40ef-8373-ef1e2a88007e",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// MIR 25 — Mangara's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANGARA_S_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mangara's Blessing",
    "b0125f79-ff68-4fb9-b309-8d277259f323",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// MIR 26 — Mangara's Equity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANGARA_S_EQUITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mangara's Equity",
    "6d796153-3f7f-4d94-8ad4-5a4b2c8e09bb",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MIR 27 — Melesse Spirit
pub(in crate::card::sets) static MELESSE_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Melesse Spirit",
    "8292b6e7-92ac-4bbb-906c-1fe47e260a24",
    "Gerry Grace",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel", "Spirit"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Black),
    ]),
);

// MIR 28 — Mtenda Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MTENDA_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mtenda Griffin",
    "162dd988-0beb-48e4-9eaa-a08ddb835648",
    "Janine Johnston",
    crate::card::CardRules::unsupported(),
);

// MIR 29 — Mtenda Herder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MTENDA_HERDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mtenda Herder",
    "51f30a3d-1421-4706-b17f-39a9ec7a0d8b",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// MIR 30 — Noble Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_ELEPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Noble Elephant",
    "65f399cb-dddb-422a-8d36-938b82b59e10",
    "Tony Roberts",
    crate::card::CardRules::unsupported(),
);

// MIR 31 — Null Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NULL_CHAMBER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Null Chamber",
    "814f8976-6612-438f-a04a-8edb63edb1e7",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// MIR 32 — Pacifism
pub(in crate::card::sets) static PACIFISM: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Pacifism",
    "c891df1b-bae6-4d6d-85ee-42901c149f98",
    "Robert Bliss",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack or block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Two prohibitions rather than one: nothing in the vocabulary bars combat
                    // wholesale, and nothing needs to -- attacking and blocking are separate
                    // declarations, so barring each is barring both.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            ),
        ]),
);

// MIR 33 — Pearl Dragon
pub(in crate::card::sets) static PEARL_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Pearl Dragon",
    "41ac3411-3286-4698-b5d9-d4bb2db30770",
    "Ian Miller",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{W}: This creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MIR 34 — Prismatic Circle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_CIRCLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Prismatic Circle",
    "8d4e3de4-f173-4bcc-a5fb-089eee7108d3",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MIR 35 — Rashida Scalebane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RASHIDA_SCALEBANE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Rashida Scalebane",
    "ebb48053-da60-477a-b1eb-a9ab9ea682af",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MIR 36 — Ritual of Steel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITUAL_OF_STEEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ritual of Steel",
    "21205189-0d00-44d5-9772-820e607dba25",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// MIR 37 — Sacred Mesa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_MESA: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sacred Mesa",
    "6622f325-6bc3-49dc-ba8e-154e70772dd5",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// MIR 38 — Shadowbane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOWBANE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Shadowbane",
    "0fbe8fa5-5ca8-43ce-863e-61802126e485",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// MIR 39 — Sidar Jabari
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIDAR_JABARI: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sidar Jabari",
    "743f0d06-cd14-49f3-ad3c-0d419a0c30bc",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// MIR 40 — Soul Echo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_ECHO: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Soul Echo",
    "cb6f9427-ef5c-49fd-81e1-ddf130d69da0",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MIR 41 — Spectral Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTRAL_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Spectral Guardian",
    "2a0bea61-8fd2-4802-90b4-651bebbe9638",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// MIR 42 — Sunweb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNWEB: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sunweb",
    "4b4dc838-3022-42f1-bc92-8e8358c27ea4",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MIR 43 — Teremko Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEREMKO_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Teremko Griffin",
    "96267012-24da-43ae-97af-69ca3d7704f8",
    "Martin McKenna",
    crate::card::CardRules::unsupported(),
);

// MIR 44 — Unyaro Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNYARO_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Unyaro Griffin",
    "d6db3573-2cc5-47ca-b65f-080aab7fdddc",
    "Al Davidson",
    crate::card::CardRules::unsupported(),
);

// MIR 45 — Vigilant Martyr
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIGILANT_MARTYR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Vigilant Martyr",
    "a02e3c10-cc21-4e06-a987-b03aee61bd50",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MIR 46 — Wall of Resistance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_RESISTANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Wall of Resistance",
    "fd25787d-b90c-4b25-8259-1ac41d4dcd15",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// MIR 47 — Ward of Lights
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARD_OF_LIGHTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ward of Lights",
    "6d389daf-c7d8-4bd1-b4ea-5082f5d280c0",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// MIR 48 — Yare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YARE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Yare",
    "9a14343d-c2cd-4a67-ae54-6b6a4677ca85",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MIR 49 — Zhalfirin Commander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZHALFIRIN_COMMANDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Zhalfirin Commander",
    "cb0e63cb-6da5-4f65-a976-e79d201e9fc7",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// MIR 50 — Zhalfirin Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZHALFIRIN_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Zhalfirin Knight",
    "eb65d104-bd50-481e-a70e-62aeb2f2c12b",
    "John Bolton",
    crate::card::CardRules::unsupported(),
);

// MIR 51 — Zuberi, Golden Feather
pub(in crate::card::sets) static ZUBERI_GOLDEN_FEATHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Zuberi, Golden Feather",
    "c1b24a80-b5e1-484f-9e21-886cb6b5db48",
    "Alan Rabinowitz",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Griffin"], 3, 3)
        .with_supertype(crate::card::CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Other Griffin creatures get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Griffin"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// MIR 52 — Ancestral Memories
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_MEMORIES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ancestral Memories",
    "b1d1298b-9f56-4540-8d6b-7eecfe38cf62",
    "William Donohoe",
    crate::card::CardRules::unsupported(),
);

// MIR 53 — Azimaet Drake
pub(in crate::card::sets) static AZIMAET_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Azimaet Drake",
    "854ba4e0-f6f3-4b6c-b6cb-ab2b93d64601",
    "Gerry Grace",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 1, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}: This creature gets +1/+0 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ]),
);

// MIR 54 — Bay Falcon
pub(in crate::card::sets) static BAY_FALCON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Bay Falcon",
    "df45268a-b757-4ad2-bab0-869058ee9186",
    "Una Fricker",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// MIR 55 — Bazaar of Wonders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAZAAR_OF_WONDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Bazaar of Wonders",
    "78e7a165-e135-4b85-943d-8352b6e65870",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// MIR 56 — Boomerang (reprint)
const BOOMERANG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BOOMERANG,
    "f5c98413-740d-47fd-bed6-2d3d05a72b31",
    "Richard Kane Ferguson",
);

// MIR 57 — Cerulean Wyvern
pub(in crate::card::sets) static CERULEAN_WYVERN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Cerulean Wyvern",
    "5c599f95-c0dd-4dd8-a8d7-9481df0cf649",
    "Gerry Grace",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Green),
    ]),
);

// MIR 58 — Cloak of Invisibility
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOAK_OF_INVISIBILITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Cloak of Invisibility",
    "3f6e21e6-fb3e-49c1-b5a0-499faf66d279",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// MIR 59 — Coral Fighters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORAL_FIGHTERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Coral Fighters",
    "08f9dfa0-bdb3-4419-ae4b-cc394552af74",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// MIR 60 — Daring Apprentice
pub(in crate::card::sets) static DARING_APPRENTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Daring Apprentice",
    "f87b3c06-5479-4060-8d3a-d161fd5830c1",
    "Kaja Foglio",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Counter target spell.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// MIR 61 — Dissipate
pub(in crate::card::sets) static DISSIPATE: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Dissipate",
    "36d9271d-6dbf-4640-9222-721a7a3ccc08",
    "Richard Kane Ferguson",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target spell. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// MIR 62 — Dream Cache
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_CACHE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Dream Cache",
    "b599422e-5f78-4d4f-bc67-684caf69458f",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// MIR 63 — Dream Fighter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_FIGHTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Dream Fighter",
    "aec06bc9-553c-4e01-8b43-a4eeaa511b4d",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// MIR 64 — Energy Vortex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_VORTEX: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Energy Vortex",
    "87e060d7-42ed-49ab-bc5c-2f3210cbd0d1",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// MIR 65 — Ether Well
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ETHER_WELL: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ether Well",
    "0a2cf195-01bf-4076-a0c6-ca5403d84f7d",
    "Charles Gillespie",
    crate::card::CardRules::unsupported(),
);

// MIR 66 — Flash
pub(in crate::card::sets) static FLASH: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Flash",
    "63af3c26-5b1f-46f6-9aa2-036c615bf5ea",
    "David Ho",
    // Two mana that puts anything onto the battlefield for two less, and a
    // real card in a deck that would rather the creature died anyway.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "You may put a creature card from your hand onto the battlefield. If you do, sacrifice \
         it unless you pay its mana cost reduced by {2}.",
        // "You may": a minimum of none, so a hand with nothing worth cheating in
        // leaves the spell doing nothing at all.
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(ParentBinding),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Hand],
                PlayerRelation::You,
            )),
            exclude: None,
            minimum: 0,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &const {
                EffectDef::PutOntoBattlefieldThen {
                    object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                    binding: ParentBinding,
                    counters: None,
                    // "Its mana cost reduced by {2}", which is a discount on the generic half
                    // and nothing else: the coloured pips are still paid in their colours.
                    then: &const {
                        EffectDef::PayOr(PayOrDef {
                            payment: EffectPaymentDef {
                                payer: PlayerSetDef::Related(PlayerRelation::You),
                                cost: EffectPaymentCostDef::ObjectManaCostReducedBy {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    )),
                                    generic: 2,
                                },
                            },
                            if_paid: None,
                            otherwise: Some(
                                &const {
                                    EffectDef::Sacrifice {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            ParentBinding,
                                        )),
                                    }
                                },
                            ),
                            visibility: ChoiceVisibilityDef::Public,
                            condition: None,
                        })
                    },
                }
            },
        }),
    )),
);

// MIR 67 — Floodgate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODGATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Floodgate",
    "fb2f594d-e608-444b-b81f-836de2452868",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MIR 68 — Hakim, Loreweaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAKIM_LOREWEAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Hakim, Loreweaver",
    "8192bca7-03e5-4ea1-ae77-8bc811c19417",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MIR 69 — Harmattan Efreet
pub(in crate::card::sets) static HARMATTAN_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Harmattan Efreet",
    "b6673d49-c3f6-41f6-84c6-1957fff71509",
    "Drew Tucker",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Efreet"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{1}{U}{U}: Target creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MIR 70 — Jolt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Jolt",
    "3e0c085a-e17d-4003-bb58-f97555365fcf",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MIR 71 — Kukemssa Pirates
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KUKEMSSA_PIRATES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Kukemssa Pirates",
    "d10b5ce7-16c9-48f3-a1da-8a91092d053a",
    "Jock",
    crate::card::CardRules::unsupported(),
);

// MIR 72 — Kukemssa Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KUKEMSSA_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Kukemssa Serpent",
    "9099eacf-1898-493c-ae87-d5ff4a3646a2",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 73 — Meddle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEDDLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Meddle",
    "0245553c-b483-47df-940b-5d7deb108642",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// MIR 74 — Memory Lapse (reprint)
const MEMORY_LAPSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::MEMORY_LAPSE,
    "63453ed9-5cf1-4cad-b173-a067f22a4405",
    "Rebecca Guay",
);

// MIR 75 — Merfolk Raiders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERFOLK_RAIDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Merfolk Raiders",
    "d0f34166-8946-4bcc-84af-3540c42ac7f7",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// MIR 76 — Merfolk Seer
pub(in crate::card::sets) static MERFOLK_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Merfolk Seer",
    "63139890-e860-4791-9bb6-b79bb361ef8b",
    "Steve Luke",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 2, 2).with_ability(
        abilities::dies_trigger(
            "When this creature dies, you may pay {1}{U}. If you do, draw a card.",
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}{U}"),
                ),
                &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            )),
        ),
    ),
);

// MIR 77 — Mind Bend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_BEND: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mind Bend",
    "952eb6ae-a530-4f4f-92f0-a6602beaa7b2",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// MIR 78 — Mind Harness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_HARNESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mind Harness",
    "5bf17780-801d-4ab8-91f4-a803ede51395",
    "John Malloy",
    crate::card::CardRules::unsupported(),
);

// MIR 79 — Mist Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIST_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mist Dragon",
    "3b9b4be4-74c8-4fe5-a5e4-de57c11e8ec1",
    "Al Davidson",
    crate::card::CardRules::unsupported(),
);

// MIR 80 — Mystical Tutor
pub(in crate::card::sets) static MYSTICAL_TUTOR: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Mystical Tutor",
    "5d98101f-e32a-4a4a-a649-faa920d111ee",
    "David O'Connor",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Search your library for an instant or sorcery card, reveal it, then shuffle and put that card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// MIR 81 — Political Trickery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLITICAL_TRICKERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Political Trickery",
    "3e56e720-5e8b-4685-969c-e073de78b9a1",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// MIR 82 — Polymorph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLYMORPH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Polymorph",
    "fbae8702-a152-4c53-8a76-691a221f2475",
    "Robert Bliss",
    crate::card::CardRules::unsupported(),
);

// MIR 83 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "49717583-e0bb-47d6-92d0-8959af13391f",
    "Charles Gillespie",
);

// MIR 84 — Prismatic Lace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_LACE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Prismatic Lace",
    "d1129585-6d59-4217-9404-747a100f1e8c",
    "David O'Connor",
    crate::card::CardRules::unsupported(),
);

// MIR 85 — Psychic Transfer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_TRANSFER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Psychic Transfer",
    "5507c474-5c4b-4292-b7bc-3ab4b48ea290",
    "Dom!",
    crate::card::CardRules::unsupported(),
);

// MIR 86 — Ray of Command (reprint)
const RAY_OF_COMMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::RAY_OF_COMMAND,
    "4361c600-7a97-46bf-8cc1-f3713d3c28ab",
    "Andrew Robinson",
);

// MIR 87 — Reality Ripple
pub(in crate::card::sets) static REALITY_RIPPLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Reality Ripple",
    "c5b94eed-2a45-4a6b-a06b-a2021b174bc5",
    "Alan Rabinowitz",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target artifact, creature, or land phases out.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::PhaseOut {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// MIR 87† — Reality Ripple (alternate printing)
const REALITY_RIPPLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REALITY_RIPPLE,
    1,
    "d2bea2ea-04d3-4835-843a-ebbed0ef5831",
    "D. Alexander Gregory",
);

// MIR 88 — Sandbar Crocodile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDBAR_CROCODILE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sandbar Crocodile",
    "450b79d9-5ab8-4699-8052-a278c316a5c3",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// MIR 89 — Sapphire Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPPHIRE_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sapphire Charm",
    "2944e5df-eef9-42be-a591-5ac15e306ad8",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// MIR 90 — Sea Scryer
pub(in crate::card::sets) static SEA_SCRYER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sea Scryer",
    "124308ac-1bf6-4a79-8aaa-f3be8eeb3e78",
    "Martin McKenna",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Wizard"], 1, 1).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add {U}.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ]),
);

// MIR 91 — Shaper Guildmage
pub(in crate::card::sets) static SHAPER_GUILDMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Shaper Guildmage",
    "4a9304e1-f403-404d-9fe9-169da75e0d62",
    "D. Alexander Gregory",
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{W}, {T}: Target creature gains first strike until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{B}, {T}: Target creature gets +1/+0 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MIR 92 — Shimmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Shimmer",
    "5c892daa-57fe-4712-b64a-6099d531bb26",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// MIR 93 — Soar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Soar",
    "bd3666dd-a90e-43c2-bd78-ea9c1af08a0e",
    "Tony Roberts",
    crate::card::CardRules::unsupported(),
);

// MIR 94 — Suq'Ata Firewalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUQ_ATA_FIREWALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Suq'Ata Firewalker",
    "b8a7c22e-fe96-4960-96d4-ee85abec3281",
    "David O'Connor",
    crate::card::CardRules::unsupported(),
);

// MIR 95 — Taniwha
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANIWHA: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Taniwha",
    "72d12315-4220-470d-9628-b9a3ea904ca7",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 96 — Teferi's Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_CURSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Teferi's Curse",
    "fb813ef5-8441-4024-a585-1ea24145e1bd",
    "Robert Bliss",
    crate::card::CardRules::unsupported(),
);

// MIR 97 — Teferi's Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Teferi's Drake",
    "c24823df-5651-4578-a0c8-f9f52f66abe4",
    "Kari Johnson",
    crate::card::CardRules::unsupported(),
);

// MIR 98 — Teferi's Imp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_IMP: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Teferi's Imp",
    "048f2368-0e2f-4197-b977-353d38b38ccc",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// MIR 99 — Thirst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THIRST: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Thirst",
    "12630ac5-a6c4-4852-abd9-5a0bc71bbf83",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// MIR 100 — Tidal Wave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_WAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Tidal Wave",
    "49a9689b-bf1e-404e-ba08-0b04de4288fb",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// MIR 101 — Vaporous Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAPOROUS_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Vaporous Djinn",
    "e7ea65e2-68d8-429f-9be7-e6e5e12a2a4d",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// MIR 102 — Wave Elemental
pub(in crate::card::sets) static WAVE_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Wave Elemental",
    "111ee762-72cb-43e3-88a1-6f1f0b9ee66e",
    "Zak Plucinski",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Elemental"], 2, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{U}, {T}, Sacrifice this creature: Tap up to three target creatures without flying.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            crate::card::KeywordAbility::Flying,
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                3,
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// MIR 103 — Abyssal Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABYSSAL_HUNTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Abyssal Hunter",
    "08798c53-46f3-4a51-9284-491730605b2b",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// MIR 104 — Ashen Powder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHEN_POWDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ashen Powder",
    "686aebd0-0d34-47e3-bbbd-ad08d2a3a864",
    "Geofrey Darrow",
    crate::card::CardRules::unsupported(),
);

// MIR 105 — Barbed-Back Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_BACK_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Barbed-Back Wurm",
    "1b96810d-72d3-4dee-a29f-cdf85ea5ce6f",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// MIR 106 — Binding Agony
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BINDING_AGONY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Binding Agony",
    "b620a9f4-358d-4436-85b8-b0c16602ff57",
    "Robert Bliss",
    crate::card::CardRules::unsupported(),
);

// MIR 107 — Blighted Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIGHTED_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Blighted Shaman",
    "b5f2b33c-8d4d-406e-98de-b92d92a3012a",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 108 — Bone Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_HARVEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Bone Harvest",
    "1cfde1ca-52f9-477d-a36e-6e4f7ca2e4d8",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// MIR 109 — Breathstealer
pub(in crate::card::sets) static BREATHSTEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Breathstealer",
    "0cd9da69-5e57-4719-a712-630c9464fada",
    "Cliff Nielsen",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Nightstalker"], 2, 2).with_ability(
        AbilityDef::activated(
            "{B}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MIR 110 — Cadaverous Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CADAVEROUS_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Cadaverous Knight",
    "06e954b2-c8ce-4b6c-a118-4a14ffa72063",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// MIR 111 — Carrion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Carrion",
    "9e6c7750-bc3b-4790-bc9d-88e2cf16881e",
    "Geofrey Darrow",
    crate::card::CardRules::unsupported(),
);

// MIR 112 — Catacomb Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATACOMB_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Catacomb Dragon",
    "46daebe4-199e-4580-8a52-1aebc8492d8c",
    "David O'Connor",
    crate::card::CardRules::unsupported(),
);

// MIR 113 — Choking Sands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHOKING_SANDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Choking Sands",
    "e41c15fb-01a1-446e-9e88-71e8e95d9bce",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// MIR 114 — Crypt Cobra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPT_COBRA: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Crypt Cobra",
    "4f7bcd36-13e2-4ac7-a449-246cecb3fc0f",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MIR 115 — Dark Banishing (reprint)
const DARK_BANISHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::DARK_BANISHING,
    "5f983dcb-b077-465f-a70b-6bd0e425556c",
    "Dermot Power",
);

// MIR 116 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "1b5fc7ce-0618-4145-b8aa-bc871e4afa99",
    "John Coulthart",
);

// MIR 117 — Dirtwater Wraith
pub(in crate::card::sets) static DIRTWATER_WRAITH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Dirtwater Wraith",
    "fcc3fa22-a4c4-423d-969b-98c65b23e782",
    "Steve Luke",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Wraith"], 1, 3).with_abilities(&[
        abilities::landwalk(crate::card::BasicLandType::Swamp),
        AbilityDef::activated(
            "{B}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MIR 118 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_LIFE,
    "e8f030a5-62c6-439d-93b6-43f2cf927a7f",
    "Richard Kane Ferguson",
);

// MIR 119 — Dread Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Dread Specter",
    "00c48e08-9a77-4ba2-8041-90998f7e3812",
    "Kathryn Rathke",
    crate::card::CardRules::unsupported(),
);

// MIR 120 — Ebony Charm (alternate printing)
const EBONY_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EBONY_CHARM,
    1,
    "937c8016-43c8-418b-9057-b5d6715f71f7",
    "Gerry Grace",
);

// MIR 120† — Ebony Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EBONY_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ebony Charm",
    "72bf17a8-d30d-4ac4-b052-7226de1b9679",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// MIR 121 — Enfeeblement
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENFEEBLEMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Enfeeblement",
    "bf808509-c6c2-4dcb-b35b-e61291faf5d9",
    "John Bolton",
    crate::card::CardRules::unsupported(),
);

// MIR 122 — Feral Shadow
pub(in crate::card::sets) static FERAL_SHADOW: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Feral Shadow",
    "672e2d34-1aae-47c7-8b7a-e3354bbbd662",
    "Cliff Nielsen",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Nightstalker"], 2, 1)
        .with_ability(abilities::flying()),
);

// MIR 123 — Fetid Horror
pub(in crate::card::sets) static FETID_HORROR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Fetid Horror",
    "4be39d50-1e36-4dac-a923-81fc9f229b8d",
    "Gary Leach",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Shade", "Horror"], 1, 2).with_ability(
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MIR 124 — Forbidden Crypt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDEN_CRYPT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Forbidden Crypt",
    "c6168af2-e49b-4c57-91c0-2cac9290a560",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// MIR 125 — Forsaken Wastes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORSAKEN_WASTES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Forsaken Wastes",
    "c9dbfc7c-164d-47b8-8f05-987864fca89b",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MIR 126 — Grave Servitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVE_SERVITUDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Grave Servitude",
    "fbe072da-0524-492b-af3d-7c2600e915ab",
    "Adrian Smith",
    crate::card::CardRules::unsupported(),
);

// MIR 127 — Gravebane Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEBANE_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Gravebane Zombie",
    "6e16f8bc-3200-4a0a-b298-c7e7b4e8376c",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// MIR 128 — Harbinger of Night
pub(in crate::card::sets) static HARBINGER_OF_NIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Harbinger of Night",
    "33124133-ed2c-4b86-a135-ac76f4fe4da5",
    "Tom Kyffin",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Spirit"], 2, 3).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a -1/-1 counter on each creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                kind: crate::card::CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// MIR 129 — Infernal Contract
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_CONTRACT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Infernal Contract",
    "e62c43bd-59fe-46e3-83f8-c4b37cbc4931",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// MIR 130 — Kaervek's Hex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAERVEK_S_HEX: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Kaervek's Hex",
    "097910fb-7c48-4535-8ffc-b521d08294b0",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 131 — Mire Shade
pub(in crate::card::sets) static MIRE_SHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mire Shade",
    "120822b8-02ae-411f-bda5-a774c21db66c",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Shade"], 1, 1).with_ability(
        AbilityDef::activated(
            "{B}, Sacrifice a Swamp: Put a +1/+1 counter on this creature. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[
                        crate::card::BasicLandType::Swamp,
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: crate::card::CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(crate::card::ActivationTimingDef::SorcerySpeed),
    ),
);

// MIR 132 — Nocturnal Raid
pub(in crate::card::sets) static NOCTURNAL_RAID: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Nocturnal Raid",
    "0015fee8-068a-421e-9143-bcb575371f9a",
    "John Matson",
    CardRules::new_instant(mana_cost!("{2}{B}{B}")).with_ability(AbilityDef::spell(
        "Black creatures get +2/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(0),
            ),
            duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// MIR 133 — Painful Memories
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAINFUL_MEMORIES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Painful Memories",
    "79223c17-ecb1-47f1-8e24-eea464cc9b1e",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// MIR 134 — Phyrexian Tribute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_TRIBUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Phyrexian Tribute",
    "bfc55d48-6d6f-429d-8281-e66a9996d574",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MIR 135 — Purraj of Urborg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURRAJ_OF_URBORG: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Purraj of Urborg",
    "4704bf02-c6be-4983-9bc7-a1d464d21b31",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MIR 136 — Ravenous Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVENOUS_VAMPIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ravenous Vampire",
    "5ee67033-7f0f-49b4-8472-3bc4fbb8ffe1",
    "John Bolton",
    crate::card::CardRules::unsupported(),
);

// MIR 137 — Reign of Terror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REIGN_OF_TERROR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Reign of Terror",
    "7bd83049-aec1-4911-bc70-39adba04b174",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// MIR 138 — Restless Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTLESS_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Restless Dead",
    "a237cff4-af6f-4745-bda1-e3ed2267fa89",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 139 — Sewer Rats
pub(in crate::card::sets) static SEWER_RATS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sewer Rats",
    "42fe08c3-5024-486c-ba03-19d371ceccb0",
    "Martin McKenna",
    CardRules::new_creature(mana_cost!("{B}"), &["Rat"], 1, 1).with_ability(
        AbilityDef::activated(
            "{B}, Pay 1 life: This creature gets +1/+0 until end of turn. Activate no more than three times each turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::PayLife(1),
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .activations_each_turn(3),
    ),
);

// MIR 140 — Shadow Guildmage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOW_GUILDMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Shadow Guildmage",
    "ba3fc11e-db36-430c-920b-31195913c16a",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// MIR 141 — Shallow Grave
pub(in crate::card::sets) static SHALLOW_GRAVE: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Shallow Grave",
    "d5c782cc-c951-4c6f-a93f-774ae6c1c214",
    "John Coulthart",
    // One turn with the creature, at instant speed, for two mana. The deck
    // that wants it is the one whose creature only has to attack once.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "Return the top creature card of your graveyard to the battlefield. That creature gains haste until end of turn. Exile it at the beginning of the next end step.",
        EffectDef::WithZoneMoveResult {
            effect: &const {
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::TopOfGraveyardMatching {
                        player: PlayerRefDef::EffectController,
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                    }),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                }
            },
            binding: ParentBinding,
            then: &const {
                EffectDef::Apply {
                    recipient: EffectRecipientDef::binding_zone_change_successors(
                        ParentBinding,
                    ),
                    effect: AppliedEffectDef::Composite(&const {
                        [
                            AppliedEffectDef::add_ability(&const { abilities::haste() }),
                            // The creature exiles itself rather than being named by a delayed trigger:
                            // it is the object that arrived, and it carries the clause with it.
                            AppliedEffectDef::add_ability(&const {
                                AbilityDef::triggered(
                                    "At the beginning of the next end step, exile this creature.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::End,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::MoveToZone {
                                        object: EffectRecipientDef::Source,
                                        zone: ZoneKind::Exile,
                                        placement: ZonePlacement::Top,
                                    },
                                )
                            }),
                        ]
                    }),
                    duration: crate::card::ResolvedEffectDurationDef::Permanent,
                }
            },
        },
    )),
);

// MIR 142 — Shauku, Endbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHAUKU_ENDBRINGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Shauku, Endbringer",
    "06d94b21-7568-4e5c-a8ec-ff5bb48a4f36",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MIR 143 — Skulking Ghost
pub(in crate::card::sets) static SKULKING_GHOST: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Skulking Ghost",
    "f8ca7e96-0545-4f36-85c0-944d5c0b760a",
    "Robert Bliss",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature becomes the target of a spell or ability, sacrifice it.",
            TriggerEventDef::BecomesTargetOfSpellOrAbility(ObjectPredicateDef::Any),
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// MIR 144 — Soul Rend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_REND: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Soul Rend",
    "7fa084e1-05c2-4691-b9fe-3e3c717e5c9d",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MIR 145 — Soulshriek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULSHRIEK: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Soulshriek",
    "5c3bf717-2b88-4704-a7e9-f62dbb3d3d3d",
    "John Bolton",
    crate::card::CardRules::unsupported(),
);

// MIR 146 — Spirit of the Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_OF_THE_NIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Spirit of the Night",
    "845c4b06-090f-4217-acb2-8900b7dab37c",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// MIR 147 — Stupor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STUPOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Stupor",
    "0fed2498-20ce-48ad-a56e-2c7e297c0c66",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// MIR 148 — Tainted Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAINTED_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Tainted Specter",
    "74feb223-784a-4540-8a8e-6007d10a9505",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// MIR 149 — Tombstone Stairwell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOMBSTONE_STAIRWELL: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Tombstone Stairwell",
    "f8fe2f99-7ec2-490c-8ec3-aa2fb4680826",
    "Dom!",
    crate::card::CardRules::unsupported(),
);

// MIR 150 — Urborg Panther
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_PANTHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Urborg Panther",
    "edc9ff0f-adec-4d39-b281-98c5862f506b",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// MIR 151 — Wall of Corpses
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_CORPSES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Wall of Corpses",
    "dde6d3d1-75db-445f-9f17-632ee0292211",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// MIR 152 — Withering Boon
pub(in crate::card::sets) static WITHERING_BOON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Withering Boon",
    "6e6499cb-6073-4c94-8c82-47f489094df5",
    "Robert Bliss",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "As an additional cost to cast this spell, pay 3 life.\nCounter target creature spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        )
        .with_spell_additional_cost(&crate::card::SpellAdditionalCostDef::pay_life(
            crate::card::CostQuantityDef::Fixed(3),
        )),
    ),
);

// MIR 153 — Zombie Mob
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_MOB: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Zombie Mob",
    "ab85551f-c9cc-409c-9fb5-a45de695e521",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MIR 154 — Agility
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGILITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Agility",
    "5694cb42-7489-40c5-b21a-aeb36636015f",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// MIR 155 — Aleatory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALEATORY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Aleatory",
    "7380af0a-8a8f-44fd-9456-14a68a2830d3",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MIR 156 — Armorer Guildmage
pub(in crate::card::sets) static ARMORER_GUILDMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Armorer Guildmage",
    "e999fdc3-9269-44d7-9015-e16f5e5b73eb",
    "Martin McKenna",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{B}, {T}: Target creature gets +1/+0 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{G}, {T}: Target creature gets +0/+1 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MIR 157 — Barreling Attack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRELING_ATTACK: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Barreling Attack",
    "a3a2955e-e714-419e-9e3d-7ae3d7fae041",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 158 — Blind Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIND_FURY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Blind Fury",
    "0ff91a17-0a5f-456f-a431-8505ba29e679",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// MIR 159 — Blistering Barrier
pub(in crate::card::sets) static BLISTERING_BARRIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Blistering Barrier",
    "56d000d8-24e1-4cf3-bed9-e68a89c8f569",
    "David Ho",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wall"], 5, 2)
        .with_ability(abilities::defender()),
);

// MIR 160 — Builder's Bane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUILDER_S_BANE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Builder's Bane",
    "fb398027-5a29-4c81-aab5-b1a2b82fd655",
    "Charles Gillespie",
    crate::card::CardRules::unsupported(),
);

// MIR 161 — Burning Palm Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNING_PALM_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Burning Palm Efreet",
    "a194488a-1d6f-4bc3-8f30-82e2c3c91389",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// MIR 162 — Burning Shield Askari
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNING_SHIELD_ASKARI: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Burning Shield Askari",
    "486547cd-d2e7-4c46-9f7b-81c4267d65cc",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MIR 163 — Chaos Charm
pub(in crate::card::sets) static CHAOS_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Chaos Charm",
    "ebf8ccb8-57fb-491c-b07d-93348b33a765",
    "Steve Luke",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target Wall.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype("Wall"),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Chaos Charm deals 1 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::spell_with_targets(
                "Target creature gains haste until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// MIR 164 — Chaosphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOSPHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Chaosphere",
    "bd41cb92-578b-4fc8-b1e6-56604088fcd5",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// MIR 165 — Cinder Cloud
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_CLOUD: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Cinder Cloud",
    "f044c470-50ce-4a6c-b8ab-665357c3c11e",
    "Jock",
    crate::card::CardRules::unsupported(),
);

// MIR 166 — Consuming Ferocity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSUMING_FEROCITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Consuming Ferocity",
    "05835e35-f4f8-4513-b5a1-9e31b21168f0",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// MIR 167 — Crimson Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIMSON_HELLKITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Crimson Hellkite",
    "694f2314-67fe-4a31-8015-9762edf15187",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// MIR 168 — Crimson Roc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIMSON_ROC: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Crimson Roc",
    "2e63836c-37af-49c1-84a1-8650ed072805",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 169 — Dwarven Miner
pub(in crate::card::sets) static DWARVEN_MINER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Dwarven Miner",
    "b7142196-c379-4932-9402-97642ad2ebdb",
    "Jock",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}, {T}: Destroy target nonbasic land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                        crate::card::CardSupertype::Basic,
                    )),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// MIR 170 — Dwarven Nomad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_NOMAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Dwarven Nomad",
    "30b09e65-5e69-48f8-be9b-a1e9706f18bf",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// MIR 171 — Ekundu Cyclops
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EKUNDU_CYCLOPS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ekundu Cyclops",
    "9047d292-8f5c-4a6b-b74e-c8dbf3e0ab24",
    "Robert Bliss",
    crate::card::CardRules::unsupported(),
);

// MIR 172 — Emberwilde Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERWILDE_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Emberwilde Djinn",
    "c9e8c9f5-61dd-4dcc-bd40-8f366374ea18",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// MIR 173 — Final Fortune
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FINAL_FORTUNE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Final Fortune",
    "83589c25-58a2-4172-8f60-6033a61f34c6",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// MIR 174 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "8d39dc35-87c9-4984-8a11-443da414e629",
    "Mike Kerr",
);

// MIR 175 — Flame Elemental
pub(in crate::card::sets) static FLAME_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Flame Elemental",
    "498e813d-b0b3-4040-b87a-4fa2be681ec5",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Elemental"], 3, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}, Sacrifice this creature: It deals damage equal to its power to target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::SourcePower,
            },
        ),
    ),
);

// MIR 176 — Flare (reprint)
const FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FLARE,
    "1bd7755f-7ca5-4948-8baf-976823906891",
    "Greg Simanson",
);

// MIR 177 — Goblin Elite Infantry
pub(in crate::card::sets) static GOBLIN_ELITE_INFANTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Goblin Elite Infantry",
    "12b9c6b8-9f87-41f1-9e81-b5995e3ed6b7",
    "Robert Bliss",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked, it gets -1/-1 until end of turn.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MIR 178 — Goblin Scouts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SCOUTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Goblin Scouts",
    "a4b87068-aaa6-41de-9b9a-76ca4210a485",
    "Geofrey Darrow",
    crate::card::CardRules::unsupported(),
);

// MIR 179 — Goblin Soothsayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SOOTHSAYER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Goblin Soothsayer",
    "686b847c-242c-4b91-9fa9-69c9c9f187a7",
    "Robert Bliss",
    crate::card::CardRules::unsupported(),
);

// MIR 180 — Goblin Tinkerer
pub(in crate::card::sets) static GOBLIN_TINKERER: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Goblin Tinkerer",
    "e6529852-8b3e-4a70-a4a1-029e012231c6",
    "Hannibal King",
    // The artifact hits back on the way out, which is why a 1/2 body
    // survives a Cursed Scroll and not much larger.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Artificer"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}: Destroy target artifact. That artifact deals damage equal to its mana value to this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            // The damage is read after the destruction, from the target slot's own
            // last-known information: the artifact is already in a graveyard by then,
            // which is the only time the reading is interesting.
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Source,
                    amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ),
);

// MIR 181 — Hammer of Bogardan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAMMER_OF_BOGARDAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Hammer of Bogardan",
    "f7285f52-5df0-4f90-9cf7-a57295d90fd4",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MIR 182 — Hivis of the Scale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIVIS_OF_THE_SCALE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Hivis of the Scale",
    "ee84e61e-d99a-489b-a3b1-cb45fc81bce6",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// MIR 183 — Illicit Auction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLICIT_AUCTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Illicit Auction",
    "9c3f633f-538c-4581-b3cc-9285ed6bc4fe",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// MIR 184 — Incinerate (reprint)
const INCINERATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::INCINERATE,
    "409b2be8-5bb6-45e0-ab87-ca73b4e3a396",
    "Brian Snõddy",
);

// MIR 185 — Kaervek's Torch
pub(in crate::card::sets) static KAERVEK_S_TORCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Kaervek's Torch",
    "0a1624ab-e50e-48a3-acf7-457069914616",
    "John Coulthart",
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[
        abilities::targeting_source_spell_cost_increase(
            "As long as Kaervek's Torch is on the stack, spells that target it cost {2} more to cast.",
            ObjectPredicateDef::Any,
            PlayerRelation::Any,
            mana_cost!("{2}"),
        )
        .with_source_zones(&[ZoneKind::Stack]),
        AbilityDef::spell_with_targets(
            "Kaervek's Torch deals X damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

// MIR 186 — Lightning Reflexes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_REFLEXES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Lightning Reflexes",
    "271febe5-98ea-403d-87be-7865cf9f426d",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// MIR 187 — Pyric Salamander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYRIC_SALAMANDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Pyric Salamander",
    "7f2dc1a7-4b70-4643-90a8-fdc7877c01ca",
    "Tony Roberts",
    crate::card::CardRules::unsupported(),
);

// MIR 188 — Raging Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Raging Spirit",
    "a196c21b-e9f5-4ae8-8a1e-668685ef4cf0",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MIR 189 — Reckless Embermage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_EMBERMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Reckless Embermage",
    "9e41febe-6fad-451e-afe8-20d3ca3c88a4",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// MIR 190 — Reign of Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REIGN_OF_CHAOS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Reign of Chaos",
    "9285b14a-fc8e-457a-b803-202e05be41e5",
    "Kathryn Rathke",
    crate::card::CardRules::unsupported(),
);

// MIR 191 — Searing Spear Askari
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARING_SPEAR_ASKARI: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Searing Spear Askari",
    "5cf66916-7f6b-412f-acd6-f96ad4539a46",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// MIR 192 — Sirocco
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIROCCO: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sirocco",
    "ce37492e-4f07-4171-97d4-84f28fb4e2be",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MIR 193 — Spitting Earth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITTING_EARTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Spitting Earth",
    "e6b774d2-1272-4cd2-b5f8-dfe3ca6e41ee",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// MIR 194 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "e64bc56f-9e0b-478e-9af4-5333ec2da1c3",
    "Tony Roberts",
);

// MIR 195 — Subterranean Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBTERRANEAN_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Subterranean Spirit",
    "132e8aac-9698-45fa-8d64-b460fd5deffc",
    "John Bolton",
    crate::card::CardRules::unsupported(),
);

// MIR 196 — Talruum Minotaur
pub(in crate::card::sets) static TALRUUM_MINOTAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Talruum Minotaur",
    "51657034-2c30-40a2-a215-a00277f01642",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Minotaur", "Berserker"], 3, 3)
        .with_ability(abilities::haste()),
);

// MIR 197 — Telim'Tor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TELIM_TOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Telim'Tor",
    "33cc89c8-a3ea-469e-b499-f48acec4f538",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MIR 198 — Telim'Tor's Edict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TELIM_TOR_S_EDICT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Telim'Tor's Edict",
    "088a60f0-2224-4e00-a06a-1d376c8d82a4",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MIR 199 — Torrent of Lava
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORRENT_OF_LAVA: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Torrent of Lava",
    "19528a24-4968-4742-a2d1-06f94e60f290",
    "Kathryn Rathke",
    crate::card::CardRules::unsupported(),
);

// MIR 200 — Viashino Warrior
pub(in crate::card::sets) static VIASHINO_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Viashino Warrior",
    "4947cbf9-69dc-44e3-a22e-a6129f331f3c",
    "Roger Raupp",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Lizard", "Warrior"], 4, 2),
);

// MIR 201 — Volcanic Dragon
pub(in crate::card::sets) static VOLCANIC_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Volcanic Dragon",
    "bda123a7-d121-483a-8ff0-a541ccdbc7ca",
    "Janine Johnston",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dragon"], 4, 4)
        .with_abilities(&[abilities::flying(), abilities::haste()]),
);

// MIR 202 — Volcanic Geyser
pub(in crate::card::sets) static VOLCANIC_GEYSER: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Volcanic Geyser",
    "911787db-9023-46f8-9501-3ad26b6ca51d",
    "David O'Connor",
    CardRules::new_instant(mana_cost!("{X}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Volcanic Geyser deals X damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )),
);

// MIR 203 — Wildfire Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILDFIRE_EMISSARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Wildfire Emissary",
    "6d99204c-b42d-48bc-9a93-fae5660665c7",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// MIR 204 — Zirilan of the Claw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZIRILAN_OF_THE_CLAW: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Zirilan of the Claw",
    "a105b7f4-c93b-4b54-acf8-7212907d9cd6",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// MIR 205 — Afiya Grove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AFIYA_GROVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Afiya Grove",
    "c495e1f3-0845-4556-83e6-de8b9d518d1d",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// MIR 206 — Armor of Thorns
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMOR_OF_THORNS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Armor of Thorns",
    "dacdb9e7-d79e-4d24-8678-a37ab6e3a413",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MIR 207 — Barbed Foliage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_FOLIAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Barbed Foliage",
    "1220fdd1-e4b0-4175-9b8a-178f6a84b8e6",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// MIR 208 — Brushwagg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRUSHWAGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Brushwagg",
    "6c20edc3-5ad0-42c1-a5ec-3e680fb03297",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 209 — Canopy Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANOPY_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Canopy Dragon",
    "20c5e4d6-b716-4994-b226-b1eb799bec25",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MIR 210 — Crash of Rhinos
pub(in crate::card::sets) static CRASH_OF_RHINOS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Crash of Rhinos",
    "d74e0337-d9ac-4bf2-b2b5-aadc97433030",
    "Steve White",
    CardRules::new_creature(mana_cost!("{6}{G}{G}"), &["Rhino"], 8, 4)
        .with_ability(abilities::trample()),
);

// MIR 211 — Cycle of Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CYCLE_OF_LIFE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Cycle of Life",
    "49b53861-97aa-4fff-9526-7d496d1717a4",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// MIR 212 — Decomposition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECOMPOSITION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Decomposition",
    "b0ea46a1-ea65-4802-812c-4f0e0f3088d2",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// MIR 213 — Early Harvest
pub(in crate::card::sets) static EARLY_HARVEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Early Harvest",
    "4f4e18b5-baef-4fe9-807e-097b972c879f",
    "Janine Johnston",
    CardRules::new_instant(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target player untaps all basic lands they control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Untap {
            object: EffectRecipientDef::objects_controlled_by_target(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(crate::card::CardSupertype::Basic),
                ]),
                TargetIndex::PRIMARY,
            ),
        },
    )),
);

// MIR 214 — Fallow Earth
pub(in crate::card::sets) static FALLOW_EARTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Fallow Earth",
    "1482c390-69cf-484e-a06c-8d63f770c7de",
    "Janine Johnston",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Put target land on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// MIR 215 — Femeref Archers
pub(in crate::card::sets) static FEMEREF_ARCHERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Femeref Archers",
    "e859180f-084c-4023-848a-96f22b8f9698",
    "William Donohoe",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Archer"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 4 damage to target attacking creature with flying.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// MIR 216 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "6d822598-2f0f-45fa-9643-0368e2c0e18b",
    "Harold McNeill",
);

// MIR 217 — Foratog
pub(in crate::card::sets) static FORATOG: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Foratog",
    "b2e1195d-443f-4826-a748-bc6e7e24153a",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Atog"], 1, 2).with_ability(
        AbilityDef::activated(
            "{G}, Sacrifice a Forest: This creature gets +2/+2 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[
                        crate::card::BasicLandType::Forest,
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MIR 218 — Giant Mantis
pub(in crate::card::sets) static GIANT_MANTIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Giant Mantis",
    "2b56c895-37d3-4475-a542-dc6d21c46f06",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Insect"], 2, 4)
        .with_ability(abilities::reach()),
);

// MIR 219 — Gibbering Hyenas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIBBERING_HYENAS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Gibbering Hyenas",
    "a252a1f5-bba5-4525-8141-57caea9624e9",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// MIR 220 — Granger Guildmage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRANGER_GUILDMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Granger Guildmage",
    "3f495b27-3eed-4962-b69a-b86f9fc6a9a7",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// MIR 221 — Hall of Gemstone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALL_OF_GEMSTONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Hall of Gemstone",
    "18e4551f-9f6c-4421-ad66-a270df6d3463",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// MIR 222 — Jolrael's Centaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLRAEL_S_CENTAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Jolrael's Centaur",
    "31588ea3-31d2-4118-9b9f-4ce820c16a15",
    "Junior Tomlin",
    crate::card::CardRules::unsupported(),
);

// MIR 223 — Jungle Patrol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNGLE_PATROL: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Jungle Patrol",
    "91c343b8-bbfd-4bc5-b80a-4bc4565cdd40",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// MIR 224 — Jungle Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNGLE_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Jungle Wurm",
    "f17f81b9-1fa1-4062-a9b3-048179274c05",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// MIR 225 — Karoo Meerkat
pub(in crate::card::sets) static KAROO_MEERKAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Karoo Meerkat",
    "16396594-4f59-4600-8e39-d99544062265",
    "Janine Johnston",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Mongoose"], 2, 1)
        .with_ability(abilities::protection_from_color(ManaColor::Blue)),
);

// MIR 226 — Locust Swarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOCUST_SWARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Locust Swarm",
    "bd269842-4c31-4398-9451-be0d941397ac",
    "William Donohoe",
    crate::card::CardRules::unsupported(),
);

// MIR 227 — Lure of Prey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LURE_OF_PREY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Lure of Prey",
    "7165f1d8-7b31-4a81-aab4-c6cd4ff2e67d",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// MIR 228 — Maro
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARO: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Maro",
    "bcfec2de-cff3-4015-9a43-a58be525a2da",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// MIR 229 — Mindbender Spores
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINDBENDER_SPORES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mindbender Spores",
    "f41066eb-be5e-4a6d-9156-64f5e56c7ab3",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 230 — Mtenda Lion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MTENDA_LION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mtenda Lion",
    "cb05cf5b-2a0d-432a-b8e7-10335c2a18e8",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// MIR 231 — Natural Balance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURAL_BALANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Natural Balance",
    "b0f7c47c-416e-4f73-89ec-024a29dfb5e9",
    "John Malloy",
    crate::card::CardRules::unsupported(),
);

// MIR 232 — Nettletooth Djinn
pub(in crate::card::sets) static NETTLETOOTH_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Nettletooth Djinn",
    "7b5aa60d-91aa-4eee-9d13-55a357c8eced",
    "Janine Johnston",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Djinn"], 4, 4).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, this creature deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// MIR 233 — Preferred Selection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PREFERRED_SELECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Preferred Selection",
    "59ce9668-78ef-44a5-ba9c-49fa740b8cb5",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// MIR 234 — Quirion Elves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_ELVES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Quirion Elves",
    "be9a64fb-1e8d-4ed8-b4c5-3d44db9c1d3b",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MIR 235 — Rampant Growth
pub(in crate::card::sets) static RAMPANT_GROWTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Rampant Growth",
    "a9dd8043-4099-42bb-9d54-4efc8b38fe18",
    "Pat Lewis",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// MIR 236 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "cc3d7a4f-8ccc-44cf-bacc-5055c7e1edbc",
    "Charles Gillespie",
);

// MIR 237 — Roots of Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTS_OF_LIFE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Roots of Life",
    "26724a51-87dd-4159-b012-71598e4cf5eb",
    "Tony Roberts",
    crate::card::CardRules::unsupported(),
);

// MIR 238 — Sabertooth Cobra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SABERTOOTH_COBRA: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sabertooth Cobra",
    "48ead72b-f3f5-4065-a33c-0992cf1fdb34",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// MIR 239 — Sandstorm (reprint)
const SANDSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::SANDSTORM,
    "cdce7d3c-cc17-4010-85ec-402a6f256360",
    "Mike Kimble",
);

// MIR 240 — Seedling Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDLING_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Seedling Charm",
    "e42746e1-f422-4453-a860-3993d5796479",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// MIR 241 — Seeds of Innocence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDS_OF_INNOCENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Seeds of Innocence",
    "f9c868f5-0f90-4f7e-bafb-c45d2372fe06",
    "Junior Tomlin",
    crate::card::CardRules::unsupported(),
);

// MIR 242 — Serene Heart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERENE_HEART: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Serene Heart",
    "aff19d9d-8069-4f8d-a81b-e2fcd94c13b3",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// MIR 243 — Stalking Tiger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALKING_TIGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Stalking Tiger",
    "f12cc4e9-010a-4ff7-a026-dcb6113a36fb",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MIR 244 — Superior Numbers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPERIOR_NUMBERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Superior Numbers",
    "43d90914-ddfc-49c2-8e58-fdc3693040f2",
    "Geofrey Darrow",
    crate::card::CardRules::unsupported(),
);

// MIR 245 — Tranquil Domain
pub(in crate::card::sets) static TRANQUIL_DOMAIN: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Tranquil Domain",
    "801f34a6-9f22-43c2-b1e5-194395cc7da1",
    "D. Alexander Gregory",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Destroy all non-Aura enchantments.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Aura")),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )),
);

// MIR 246 — Tropical Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROPICAL_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Tropical Storm",
    "cd5f473c-e11e-4047-91f9-81b80f0a3562",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// MIR 247 — Uktabi Faerie
pub(in crate::card::sets) static UKTABI_FAERIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Uktabi Faerie",
    "c3cdee9f-7ea9-4397-a230-ce610be5d3af",
    "Junior Tomlin",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Faerie"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{3}{G}, Sacrifice this creature: Destroy target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}{G}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// MIR 248 — Uktabi Wildcats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UKTABI_WILDCATS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Uktabi Wildcats",
    "d0f287ff-1b1b-454a-9360-fac34c8e1f24",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MIR 249 — Unseen Walker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSEEN_WALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Unseen Walker",
    "0861a73d-8810-42f7-b20a-a6dd53586220",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MIR 250 — Unyaro Bee Sting
pub(in crate::card::sets) static UNYARO_BEE_STING: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Unyaro Bee Sting",
    "71bdd944-e86c-4e5e-b75c-9bbf4fb27ccd",
    "Pat Lewis",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Unyaro Bee Sting deals 2 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// MIR 251 — Village Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILLAGE_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Village Elder",
    "253a1d97-ec45-41c9-ba81-bbb6ab584b2b",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MIR 252 — Waiting in the Weeds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAITING_IN_THE_WEEDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Waiting in the Weeds",
    "5f91ec4e-6818-46f4-94da-f3f8c4489fb2",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// MIR 253 — Wall of Roots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_ROOTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Wall of Roots",
    "aeb151d2-c313-44d2-972e-33487f070c23",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MIR 254 — Wild Elephant
pub(in crate::card::sets) static WILD_ELEPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Wild Elephant",
    "7809131c-747c-4c33-a3ca-13e573a92b66",
    "Junior Tomlin",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elephant"], 3, 3)
        .with_ability(abilities::trample()),
);

// MIR 255 — Worldly Tutor
pub(in crate::card::sets) static WORLDLY_TUTOR: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Worldly Tutor",
    "f00115bc-b551-4bf5-a121-bebb37201575",
    "David O'Connor",
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Search your library for a creature card, reveal it, then shuffle and put the card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// MIR 256 — Asmira, Holy Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASMIRA_HOLY_AVENGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Asmira, Holy Avenger",
    "a7d64600-84fc-42a5-a6a6-b26f98fac0a4",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MIR 257 — Benthic Djinn
pub(in crate::card::sets) static BENTHIC_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Benthic Djinn",
    "95b03586-8d04-4114-a7ce-b8c8ab5ff857",
    "Adam Rex",
    CardRules::new_creature(mana_cost!("{2}{U}{B}"), &["Djinn"], 5, 3).with_abilities(&[
        abilities::landwalk(crate::card::BasicLandType::Island),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you lose 2 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// MIR 258 — Cadaverous Bloom
pub(in crate::card::sets) static CADAVEROUS_BLOOM: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Cadaverous Bloom",
    "c9bef70b-61c7-4df5-b4df-09cd6ab2015c",
    "Alan Rabinowitz",
    CardRules::new_enchantment(mana_cost!("{3}{B}{G}")).with_ability(AbilityDef::activated_mana(
        "Exile a card from your hand: Add {B}{B} or {G}{G}.",
        &[AbilityCostDef::ExileCardFromHand(ObjectPredicateDef::Any)],
        EffectDef::AddMana(
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Green]).with_amount(2),
        ),
    )),
);

// MIR 259 — Circle of Despair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_DESPAIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Circle of Despair",
    "ef9d2c05-0d2f-4d02-aef3-e1078d78e5ff",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MIR 260 — Delirium
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELIRIUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Delirium",
    "c52981ea-1173-4f4b-a929-705a11c6e381",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MIR 261 — Discordant Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCORDANT_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Discordant Spirit",
    "be67b950-dfe3-4159-aa53-63df25d2a926",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// MIR 262 — Emberwilde Caliph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERWILDE_CALIPH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Emberwilde Caliph",
    "9598a9b4-15bb-4645-92ed-8eedef75dc24",
    "Jennifer Law",
    crate::card::CardRules::unsupported(),
);

// MIR 263 — Energy Bolt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_BOLT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Energy Bolt",
    "711f4cff-0256-44b2-a2fe-1cae6e9edb2b",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// MIR 264 — Frenetic Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRENETIC_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Frenetic Efreet",
    "50d4468b-f7de-44fe-898a-4125d26d242f",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// MIR 265 — Grim Feast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIM_FEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Grim Feast",
    "a69dc4ac-7354-465e-b859-d8556f3b1498",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// MIR 266 — Harbor Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARBOR_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Harbor Guardian",
    "78a2359f-6586-42a6-a855-c0049b448cb9",
    "Stuart Beel",
    crate::card::CardRules::unsupported(),
);

// MIR 267 — Haunting Apparition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTING_APPARITION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Haunting Apparition",
    "8ce9c58f-6470-4e0f-8f6b-457fbaac7451",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// MIR 268 — Hazerider Drake
pub(in crate::card::sets) static HAZERIDER_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Hazerider Drake",
    "68d2ae5c-58b9-4690-8039-d50cca9ef6cf",
    "Zina Saunders",
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Drake"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// MIR 269 — Jungle Troll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNGLE_TROLL: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Jungle Troll",
    "59fb9591-399a-4196-a52d-f2954d287a10",
    "John Bolton",
    crate::card::CardRules::unsupported(),
);

// MIR 270 — Kaervek's Purge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAERVEK_S_PURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Kaervek's Purge",
    "7a42ef95-92ec-40fe-ab30-a476f012a525",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// MIR 271 — Leering Gargoyle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEERING_GARGOYLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Leering Gargoyle",
    "a05d1cac-7012-4d22-82d3-0f82b168fe68",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// MIR 272 — Malignant Growth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALIGNANT_GROWTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Malignant Growth",
    "760b6703-ac92-45f6-8c32-60f760eba866",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MIR 273 — Phyrexian Purge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_PURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Phyrexian Purge",
    "312bbc1b-4c2a-44c1-8e62-c0f94fd2ba8e",
    "Robert Bliss",
    crate::card::CardRules::unsupported(),
);

// MIR 274 — Prismatic Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_BOON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Prismatic Boon",
    "fb774a9b-d29f-4f41-9fb8-a0189205e16f",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// MIR 275 — Purgatory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURGATORY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Purgatory",
    "9ed5e58b-46f3-437c-89ac-24235a65bd1f",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// MIR 276 — Radiant Essence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_ESSENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Radiant Essence",
    "d9b1853c-d5b9-4361-8a2f-36946a62847b",
    "Jennifer Law",
    crate::card::CardRules::unsupported(),
);

// MIR 277 — Reflect Damage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REFLECT_DAMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Reflect Damage",
    "3a2bf39b-9665-426b-b618-eb731d24a1ee",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MIR 278 — Reparations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPARATIONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Reparations",
    "0a9edf28-79c0-42a5-af0f-6df9c3a1f546",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// MIR 279 — Rock Basilisk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCK_BASILISK: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Rock Basilisk",
    "79949237-dcce-4ac1-bdc6-7c6d8b5f5fde",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// MIR 280 — Savage Twister (alternate printing)
const SAVAGE_TWISTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAVAGE_TWISTER,
    1,
    "eb73313b-d39a-46ab-abfc-76f94a75dfca",
    "Bob Eggleton",
);

// MIR 280† — Savage Twister
pub(in crate::card::sets) static SAVAGE_TWISTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Savage Twister",
    "025dfd44-f611-42e3-9e22-f630599f0591",
    "Bob Eggleton",
    CardRules::new_sorcery(mana_cost!("{X}{R}{G}")).with_ability(AbilityDef::spell(
        "Savage Twister deals X damage to each creature.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::ChosenX,
        },
    )),
);

// MIR 281 — Sawback Manticore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAWBACK_MANTICORE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sawback Manticore",
    "c24e74e9-ce88-48af-a113-b4fe76f963d4",
    "Martin McKenna",
    crate::card::CardRules::unsupported(),
);

// MIR 282 — Sealed Fate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEALED_FATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sealed Fate",
    "36cf9da8-f078-4f6e-8077-bb24a7ed487f",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// MIR 283 — Shauku's Minion
pub(in crate::card::sets) static SHAUKU_S_MINION: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Shauku's Minion",
    "6e9743dc-031b-4932-80a6-1c8ec1dea069",
    "Greg Simanson",
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Human", "Minion"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{B}{R}, {T}: This creature deals 2 damage to target white creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// MIR 284 — Spatial Binding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPATIAL_BINDING: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Spatial Binding",
    "176d625f-1410-4ad6-a279-9a184fac6507",
    "Dom!",
    crate::card::CardRules::unsupported(),
);

// MIR 285 — Unfulfilled Desires
pub(in crate::card::sets) static UNFULFILLED_DESIRES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Unfulfilled Desires",
    "3389d6c7-2a8a-48d6-a09d-aa195d830576",
    "D. Alexander Gregory",
    CardRules::new_enchantment(mana_cost!("{1}{U}{B}")).with_ability(AbilityDef::activated(
        "{1}, Pay 1 life: Draw a card, then discard a card.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::PayLife(1),
        ],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: crate::card::DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// MIR 286 — Vitalizing Cascade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VITALIZING_CASCADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Vitalizing Cascade",
    "5fe8a5b8-1a87-46f5-920f-fbbb05bfd563",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// MIR 287 — Warping Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARPING_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Warping Wurm",
    "a2c41d0f-f1db-4797-b245-7de12ffa3a0d",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// MIR 288 — Wellspring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELLSPRING: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Wellspring",
    "c69ba095-dd78-4999-87a9-63f7165846e4",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// MIR 289 — Windreaper Falcon
pub(in crate::card::sets) static WINDREAPER_FALCON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Windreaper Falcon",
    "65285030-0714-43bf-bb91-a79dcba1ccc5",
    "Tony Roberts",
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Blue),
    ]),
);

// MIR 290 — Zebra Unicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZEBRA_UNICORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Zebra Unicorn",
    "a663ee9d-78f1-4c89-af9e-c788e165fa91",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// MIR 291 — Acidic Dagger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACIDIC_DAGGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Acidic Dagger",
    "6dfdea79-2a7c-489d-8466-e6090c2a0919",
    "Stuart Beel",
    crate::card::CardRules::unsupported(),
);

// MIR 292 — Amber Prison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMBER_PRISON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Amber Prison",
    "046475e5-36d1-4b5f-af31-6df715c7a368",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MIR 293 — Amulet of Unmaking
pub(in crate::card::sets) static AMULET_OF_UNMAKING: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Amulet of Unmaking",
    "fdbd94c8-611c-4b20-99ca-dd2d7661d644",
    "Kaja Foglio",
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(
        AbilityDef::activated_with_targets(
            "{5}, {T}, Exile this artifact: Exile target artifact, creature, or land. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{5}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::ExileSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        )
        .with_activation_timing(crate::card::ActivationTimingDef::SorcerySpeed),
    ),
);

// MIR 294 — Basalt Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BASALT_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Basalt Golem",
    "5ff538f9-10b4-4327-aea6-a86759daf488",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// MIR 295 — Bone Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_MASK: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Bone Mask",
    "1f46aea3-ea00-46c4-b207-522ceeeae68b",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// MIR 296 — Charcoal Diamond
pub(in crate::card::sets) static CHARCOAL_DIAMOND: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Charcoal Diamond",
    "2a81b3f1-babc-4bd7-8b87-754c8389ae85",
    "Drew Tucker",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        abilities::tap_for(ManaColor::Black),
    ]),
);

// MIR 297 — Chariot of the Sun
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARIOT_OF_THE_SUN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Chariot of the Sun",
    "2e3bc470-cfcc-4835-b46f-c08d698ee1ab",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// MIR 298 — Crystal Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Crystal Golem",
    "4b1d3280-f3e1-42ea-93e1-dbab7336fb73",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// MIR 299 — Cursed Totem
pub(in crate::card::sets) static CURSED_TOTEM: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Cursed Totem",
    "cc99ee76-45b6-4f1d-b0b0-7da8775ca90c",
    "D. Alexander Gregory",
    // Symmetrical and unconditional: it shuts off every creature on the
    // table, including the ones that make mana.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Activated abilities of creatures can't be activated.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
        },
    )),
);

// MIR 300 — Elixir of Vitality
pub(in crate::card::sets) static ELIXIR_OF_VITALITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Elixir of Vitality",
    "60450239-6055-4561-9f8c-565b4e4d9cb1",
    "Douglas Shuler",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: You gain 4 life.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
        AbilityDef::activated(
            "{8}, {T}, Sacrifice this artifact: You gain 8 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{8}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(8),
            },
        ),
    ]),
);

// MIR 301 — Ersatz Gnomes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERSATZ_GNOMES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ersatz Gnomes",
    "5a2747ab-00c8-4f59-b9a6-54ff4e99f6c8",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MIR 302 — Fire Diamond
pub(in crate::card::sets) static FIRE_DIAMOND: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Fire Diamond",
    "bcca5bbe-df01-45ea-a6ac-4e3d1cf237c8",
    "Richard Thomas",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        abilities::tap_for(ManaColor::Red),
    ]),
);

// MIR 303 — Grinning Totem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRINNING_TOTEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Grinning Totem",
    "f3558ddf-2bc6-4870-bd24-2467d870ffe5",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// MIR 304 — Horrible Hordes
pub(in crate::card::sets) static HORRIBLE_HORDES: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Horrible Hordes",
    "cf04f2cd-d9f8-4e0c-adaf-6d38bc14dd7a",
    "Ian Miller",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Spirit"], 2, 2).with_ability(
        abilities::rampage(
            1,
            "Rampage 1 (Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocking it beyond the first.)",
        ),
    ),
);

// MIR 305 — Igneous Golem
pub(in crate::card::sets) static IGNEOUS_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Igneous Golem",
    "f44c5e24-98f9-4a4d-9ecc-c862363eb66d",
    "Adam Rex",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Golem"], 3, 4).with_ability(
        AbilityDef::activated(
            "{2}: This creature gains trample until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MIR 306 — Lead Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEAD_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Lead Golem",
    "dc9afb9e-eab3-4969-9a44-2c01bf730e68",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// MIR 307 — Lion's Eye Diamond
pub(in crate::card::sets) static LION_S_EYE_DIAMOND: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Lion's Eye Diamond",
    "63bacc32-d6ba-420c-9b49-299c08e5fb39",
    "Margaret Organ-Kean",
    // Three mana for nothing, once, at the price of everything in hand.
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(
        AbilityDef::activated_mana(
            "Discard your hand, Sacrifice this artifact: Add three mana of any one color. \
             Activate only as an instant.",
            // The hand is the cost and the clause is the drawback: "activate only as an
            // instant" is what stops it from paying for the spell you are holding,
            // because that spell is still in the hand it discards. What the deck
            // playing it wants is the hand already emptied -- a graveyard the discard
            // filled, or a spell already on the stack.
            &[AbilityCostDef::DiscardHand, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
        )
        .only_as_instant(),
    ),
);

// MIR 308 — Mana Prism
pub(in crate::card::sets) static MANA_PRISM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mana Prism",
    "81137c17-18b8-484e-8334-28f143c08177",
    "Margaret Organ-Kean",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// MIR 309 — Mangara's Tome
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANGARA_S_TOME: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mangara's Tome",
    "557ebacf-4a8f-4548-a142-533b7adfdac3",
    "John Bolton",
    crate::card::CardRules::unsupported(),
);

// MIR 310 — Marble Diamond
pub(in crate::card::sets) static MARBLE_DIAMOND: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Marble Diamond",
    "4731eb16-3088-4d4d-80a4-7e39d00c1a87",
    "Jeff Miracola",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        abilities::tap_for(ManaColor::White),
    ]),
);

// MIR 311 — Misers' Cage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISERS_CAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Misers' Cage",
    "f4c00691-26b1-4e7a-bdf5-4b15f0eb45ce",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// MIR 312 — Moss Diamond
pub(in crate::card::sets) static MOSS_DIAMOND: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Moss Diamond",
    "87cea56a-b64f-452a-8e4b-40363aabb452",
    "Donato Giancola",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        abilities::tap_for(ManaColor::Green),
    ]),
);

// MIR 313 — Patagia Golem
pub(in crate::card::sets) static PATAGIA_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Patagia Golem",
    "89920b7a-fd56-4fa8-96c9-fb66c2af6fbf",
    "Scott Kirschner",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 2, 3).with_ability(
        AbilityDef::activated(
            "{3}: This creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MIR 314 — Paupers' Cage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAUPERS_CAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Paupers' Cage",
    "3411c350-a940-4172-b558-f5dc44b4fb33",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// MIR 315 — Phyrexian Dreadnought
pub(in crate::card::sets) static PHYREXIAN_DREADNOUGHT: CardRecord = CardRecord::new(
    CardSet::Mirage,
    "Phyrexian Dreadnought",
    "7b8197b9-0cd1-4fa1-9668-d1b5f1759151",
    "Pete Venters",
    // A 12/12 for one mana whose drawback nobody intends to pay: the deck
    // answers its own trigger and keeps the body.
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Phyrexian", "Dreadnought"], 12, 12)
        .with_abilities(&[
            abilities::trample(),
            // Twelve power, paid in creatures. A board that cannot reach it is never
            // asked, which is the ordinary case: the deck plays this to be answered by
            // its own Stifle, not to be paid for.
            abilities::enters_trigger("When this creature enters, sacrifice it unless you sacrifice any number of creatures with total power 12 or greater.", EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef {
                    payer: PlayerSetDef::One(PlayerRefDef::EffectController),
                    cost: EffectPaymentCostDef::SacrificeCreaturesWithTotalPower(12),
                },
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            ))),
        ]),
);

// MIR 316 — Phyrexian Vault
pub(in crate::card::sets) static PHYREXIAN_VAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Phyrexian Vault",
    "0966a4e7-bf0f-4880-a4ba-8641fb7e4519",
    "Hannibal King",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated(
        "{2}, {T}, Sacrifice a creature: Draw a card.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// MIR 317 — Razor Pendulum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAZOR_PENDULUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Razor Pendulum",
    "72562860-6097-4c63-9858-ae130805f4d6",
    "Zak Plucinski",
    crate::card::CardRules::unsupported(),
);

// MIR 318 — Sand Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAND_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sand Golem",
    "84e4a955-ce9a-4386-b6ac-c00fd25de882",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MIR 319 — Sky Diamond
pub(in crate::card::sets) static SKY_DIAMOND: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Sky Diamond",
    "8959aef9-77bb-45a3-939e-324bd59aec89",
    "D. Alexander Gregory",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        abilities::tap_for(ManaColor::Blue),
    ]),
);

// MIR 320 — Teeka's Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEEKA_S_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Teeka's Dragon",
    "57e8971d-baeb-4e4f-8c4d-0e8109e4505e",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// MIR 321 — Telim'Tor's Darts
pub(in crate::card::sets) static TELIM_TOR_S_DARTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Telim'Tor's Darts",
    "59733b76-3b44-4543-8bb2-a160232cee27",
    "Kev Walker",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: This artifact deals 1 damage to target player or planeswalker.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    )),
);

// MIR 322 — Unerring Sling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNERRING_SLING: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Unerring Sling",
    "ecf946b8-8574-406a-83fd-966ed912921f",
    "Zak Plucinski",
    crate::card::CardRules::unsupported(),
);

// MIR 323 — Ventifact Bottle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENTIFACT_BOTTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Ventifact Bottle",
    "520f4a24-fb1a-4964-887c-2f08a752fae2",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// MIR 324 — Bad River
pub(in crate::card::sets) static BAD_RIVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Bad River",
    "7a78abdb-d1ac-49cb-a74b-9de21c06364a",
    "Terese Nielsen",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        slow_fetch_land_ability(
            "{T}, Sacrifice this land: Search your library for an Island or Swamp card, put it onto the battlefield, then shuffle.",
            &[
                crate::card::BasicLandType::Island,
                crate::card::BasicLandType::Swamp,
            ],
        ),
    ]),
);

// MIR 325 — Crystal Vein
pub(in crate::card::sets) static CRYSTAL_VEIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Crystal Vein",
    "1afb6e29-058e-44c5-a3fa-56c462f070a0",
    "Pat Lewis",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {C}{C}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
    ]),
);

// MIR 326 — Flood Plain
pub(in crate::card::sets) static FLOOD_PLAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Flood Plain",
    "7b7610f3-f182-404e-80b9-ccd94e174db0",
    "Pat Lewis",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        slow_fetch_land_ability(
            "{T}, Sacrifice this land: Search your library for a Plains or Island card, put it onto the battlefield, then shuffle.",
            &[
                crate::card::BasicLandType::Plains,
                crate::card::BasicLandType::Island,
            ],
        ),
    ]),
);

// MIR 327 — Grasslands
pub(in crate::card::sets) static GRASSLANDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Grasslands",
    "65f5efac-ef98-4be2-abcc-1aa38bf66b06",
    "John Avon",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        slow_fetch_land_ability(
            "{T}, Sacrifice this land: Search your library for a Forest or Plains card, put it onto the battlefield, then shuffle.",
            &[
                crate::card::BasicLandType::Forest,
                crate::card::BasicLandType::Plains,
            ],
        ),
    ]),
);

// MIR 328 — Mountain Valley
pub(in crate::card::sets) static MOUNTAIN_VALLEY: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Mountain Valley",
    "ded4e5c2-4f03-47c1-9843-b98c239ccfea",
    "Kari Johnson",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        slow_fetch_land_ability(
            "{T}, Sacrifice this land: Search your library for a Mountain or Forest card, put it onto the battlefield, then shuffle.",
            &[
                crate::card::BasicLandType::Mountain,
                crate::card::BasicLandType::Forest,
            ],
        ),
    ]),
);

// MIR 329 — Rocky Tar Pit
pub(in crate::card::sets) static ROCKY_TAR_PIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Rocky Tar Pit",
    "1e21c347-7aaf-42ae-abf3-f1283c5b54e6",
    "Jeff Miracola",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        slow_fetch_land_ability(
            "{T}, Sacrifice this land: Search your library for a Swamp or Mountain card, put it onto the battlefield, then shuffle.",
            &[
                crate::card::BasicLandType::Swamp,
                crate::card::BasicLandType::Mountain,
            ],
        ),
    ]),
);

// MIR 330 — Teferi's Isle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_ISLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirage,
    "Teferi's Isle",
    "b6ed7ca8-fd91-46e3-9149-a3de23c7078e",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// MIR 331 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "888f32ad-3bdd-4c46-b3f0-522ac9763591",
    "Tom Wänerstrand",
);

// MIR 332 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "b078ef66-a83b-4e12-bcba-838bc3809322",
    "Tom Wänerstrand",
);

// MIR 333 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "ac15978d-41e6-4eaa-9456-0de3b912d437",
    "Tom Wänerstrand",
);

// MIR 334 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "81bbbf38-5d1a-4013-aff9-6167709897f0",
    "Tom Wänerstrand",
);

// MIR 335 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "df84bf1b-8698-4c3e-baa9-926f0efc6a12",
    "Douglas Shuler",
);

// MIR 336 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "776912e7-fa16-4d71-b830-0a5d35f38297",
    "Douglas Shuler",
);

// MIR 337 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "1e571f07-167f-4a7d-8f98-0c98d7d15e81",
    "Douglas Shuler",
);

// MIR 338 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "a39fc1e0-caf0-4cfa-bbf2-fea7ca32c00d",
    "Douglas Shuler",
);

// MIR 339 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "f2d28477-8be1-4caf-8185-64f47905ccb1",
    "Bob Eggleton",
);

// MIR 340 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "2c4be2d6-d168-4e67-8d75-a04f637b56dd",
    "Bob Eggleton",
);

// MIR 341 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "65181db5-3fc5-497b-ba61-385efdf740ed",
    "Bob Eggleton",
);

// MIR 342 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "5083de34-d127-45df-9252-ff09b5cf8b47",
    "Bob Eggleton",
);

// MIR 343 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "97004cdc-0baf-415d-8e87-7f36667c4628",
    "John Avon",
);

// MIR 344 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "207a468f-79c3-486e-ac5a-4516361b7b4d",
    "John Avon",
);

// MIR 345 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "681b9c8a-6140-4086-bc72-37f0058a8e9f",
    "John Avon",
);

// MIR 346 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "1cef9230-34fa-496f-8835-5dfaac627f70",
    "John Avon",
);

// MIR 347 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "a0ae7a81-9a61-4112-af22-369c525b2eb1",
    "Tony Roberts",
);

// MIR 348 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "bb6b3f80-20bf-4693-b106-cfb31a3d7f83",
    "Tony Roberts",
);

// MIR 349 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "d8a67bf6-818b-4afe-af15-3d955dc5e884",
    "Tony Roberts",
);

// MIR 350 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "95dfef30-acca-4b15-a05e-d33289055218",
    "Tony Roberts",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AFTERLIFE,
    &ALARUM,
    &AUSPICIOUS_ANCESTOR,
    &BENEVOLENT_UNICORN,
    &BLINDING_LIGHT,
    &CELESTIAL_DAWN,
    &CIVIC_GUILDMAGE,
    &DAZZLING_BEAUTY,
    &DISEMPOWER,
    &DIVINE_RETRIBUTION,
    &EKUNDU_GRIFFIN,
    &ENLIGHTENED_TUTOR,
    &ETHEREAL_CHAMPION,
    &FAVORABLE_DESTINY,
    &FEMEREF_HEALER,
    &FEMEREF_KNIGHT,
    &FEMEREF_SCOUTS,
    &ILLUMINATION,
    &IRON_TUSK_ELEPHANT,
    &IVORY_CHARM,
    &JABARI_S_INFLUENCE,
    &MANGARA_S_BLESSING,
    &MANGARA_S_EQUITY,
    &MELESSE_SPIRIT,
    &MTENDA_GRIFFIN,
    &MTENDA_HERDER,
    &NOBLE_ELEPHANT,
    &NULL_CHAMBER,
    &PACIFISM,
    &PEARL_DRAGON,
    &PRISMATIC_CIRCLE,
    &RASHIDA_SCALEBANE,
    &RITUAL_OF_STEEL,
    &SACRED_MESA,
    &SHADOWBANE,
    &SIDAR_JABARI,
    &SOUL_ECHO,
    &SPECTRAL_GUARDIAN,
    &SUNWEB,
    &TEREMKO_GRIFFIN,
    &UNYARO_GRIFFIN,
    &VIGILANT_MARTYR,
    &WALL_OF_RESISTANCE,
    &WARD_OF_LIGHTS,
    &YARE,
    &ZHALFIRIN_COMMANDER,
    &ZHALFIRIN_KNIGHT,
    &ZUBERI_GOLDEN_FEATHER,
    &ANCESTRAL_MEMORIES,
    &AZIMAET_DRAKE,
    &BAY_FALCON,
    &BAZAAR_OF_WONDERS,
    &CERULEAN_WYVERN,
    &CLOAK_OF_INVISIBILITY,
    &CORAL_FIGHTERS,
    &DARING_APPRENTICE,
    &DISSIPATE,
    &DREAM_CACHE,
    &DREAM_FIGHTER,
    &ENERGY_VORTEX,
    &ETHER_WELL,
    &FLASH,
    &FLOODGATE,
    &HAKIM_LOREWEAVER,
    &HARMATTAN_EFREET,
    &JOLT,
    &KUKEMSSA_PIRATES,
    &KUKEMSSA_SERPENT,
    &MEDDLE,
    &MERFOLK_RAIDERS,
    &MERFOLK_SEER,
    &MIND_BEND,
    &MIND_HARNESS,
    &MIST_DRAGON,
    &MYSTICAL_TUTOR,
    &POLITICAL_TRICKERY,
    &POLYMORPH,
    &PRISMATIC_LACE,
    &PSYCHIC_TRANSFER,
    &REALITY_RIPPLE,
    &SANDBAR_CROCODILE,
    &SAPPHIRE_CHARM,
    &SEA_SCRYER,
    &SHAPER_GUILDMAGE,
    &SHIMMER,
    &SOAR,
    &SUQ_ATA_FIREWALKER,
    &TANIWHA,
    &TEFERI_S_CURSE,
    &TEFERI_S_DRAKE,
    &TEFERI_S_IMP,
    &THIRST,
    &TIDAL_WAVE,
    &VAPOROUS_DJINN,
    &WAVE_ELEMENTAL,
    &ABYSSAL_HUNTER,
    &ASHEN_POWDER,
    &BARBED_BACK_WURM,
    &BINDING_AGONY,
    &BLIGHTED_SHAMAN,
    &BONE_HARVEST,
    &BREATHSTEALER,
    &CADAVEROUS_KNIGHT,
    &CARRION,
    &CATACOMB_DRAGON,
    &CHOKING_SANDS,
    &CRYPT_COBRA,
    &DIRTWATER_WRAITH,
    &DREAD_SPECTER,
    &EBONY_CHARM,
    &ENFEEBLEMENT,
    &FERAL_SHADOW,
    &FETID_HORROR,
    &FORBIDDEN_CRYPT,
    &FORSAKEN_WASTES,
    &GRAVE_SERVITUDE,
    &GRAVEBANE_ZOMBIE,
    &HARBINGER_OF_NIGHT,
    &INFERNAL_CONTRACT,
    &KAERVEK_S_HEX,
    &MIRE_SHADE,
    &NOCTURNAL_RAID,
    &PAINFUL_MEMORIES,
    &PHYREXIAN_TRIBUTE,
    &PURRAJ_OF_URBORG,
    &RAVENOUS_VAMPIRE,
    &REIGN_OF_TERROR,
    &RESTLESS_DEAD,
    &SEWER_RATS,
    &SHADOW_GUILDMAGE,
    &SHALLOW_GRAVE,
    &SHAUKU_ENDBRINGER,
    &SKULKING_GHOST,
    &SOUL_REND,
    &SOULSHRIEK,
    &SPIRIT_OF_THE_NIGHT,
    &STUPOR,
    &TAINTED_SPECTER,
    &TOMBSTONE_STAIRWELL,
    &URBORG_PANTHER,
    &WALL_OF_CORPSES,
    &WITHERING_BOON,
    &ZOMBIE_MOB,
    &AGILITY,
    &ALEATORY,
    &ARMORER_GUILDMAGE,
    &BARRELING_ATTACK,
    &BLIND_FURY,
    &BLISTERING_BARRIER,
    &BUILDER_S_BANE,
    &BURNING_PALM_EFREET,
    &BURNING_SHIELD_ASKARI,
    &CHAOS_CHARM,
    &CHAOSPHERE,
    &CINDER_CLOUD,
    &CONSUMING_FEROCITY,
    &CRIMSON_HELLKITE,
    &CRIMSON_ROC,
    &DWARVEN_MINER,
    &DWARVEN_NOMAD,
    &EKUNDU_CYCLOPS,
    &EMBERWILDE_DJINN,
    &FINAL_FORTUNE,
    &FLAME_ELEMENTAL,
    &GOBLIN_ELITE_INFANTRY,
    &GOBLIN_SCOUTS,
    &GOBLIN_SOOTHSAYER,
    &GOBLIN_TINKERER,
    &HAMMER_OF_BOGARDAN,
    &HIVIS_OF_THE_SCALE,
    &ILLICIT_AUCTION,
    &KAERVEK_S_TORCH,
    &LIGHTNING_REFLEXES,
    &PYRIC_SALAMANDER,
    &RAGING_SPIRIT,
    &RECKLESS_EMBERMAGE,
    &REIGN_OF_CHAOS,
    &SEARING_SPEAR_ASKARI,
    &SIROCCO,
    &SPITTING_EARTH,
    &SUBTERRANEAN_SPIRIT,
    &TALRUUM_MINOTAUR,
    &TELIM_TOR,
    &TELIM_TOR_S_EDICT,
    &TORRENT_OF_LAVA,
    &VIASHINO_WARRIOR,
    &VOLCANIC_DRAGON,
    &VOLCANIC_GEYSER,
    &WILDFIRE_EMISSARY,
    &ZIRILAN_OF_THE_CLAW,
    &AFIYA_GROVE,
    &ARMOR_OF_THORNS,
    &BARBED_FOLIAGE,
    &BRUSHWAGG,
    &CANOPY_DRAGON,
    &CRASH_OF_RHINOS,
    &CYCLE_OF_LIFE,
    &DECOMPOSITION,
    &EARLY_HARVEST,
    &FALLOW_EARTH,
    &FEMEREF_ARCHERS,
    &FORATOG,
    &GIANT_MANTIS,
    &GIBBERING_HYENAS,
    &GRANGER_GUILDMAGE,
    &HALL_OF_GEMSTONE,
    &JOLRAEL_S_CENTAUR,
    &JUNGLE_PATROL,
    &JUNGLE_WURM,
    &KAROO_MEERKAT,
    &LOCUST_SWARM,
    &LURE_OF_PREY,
    &MARO,
    &MINDBENDER_SPORES,
    &MTENDA_LION,
    &NATURAL_BALANCE,
    &NETTLETOOTH_DJINN,
    &PREFERRED_SELECTION,
    &QUIRION_ELVES,
    &RAMPANT_GROWTH,
    &ROOTS_OF_LIFE,
    &SABERTOOTH_COBRA,
    &SEEDLING_CHARM,
    &SEEDS_OF_INNOCENCE,
    &SERENE_HEART,
    &STALKING_TIGER,
    &SUPERIOR_NUMBERS,
    &TRANQUIL_DOMAIN,
    &TROPICAL_STORM,
    &UKTABI_FAERIE,
    &UKTABI_WILDCATS,
    &UNSEEN_WALKER,
    &UNYARO_BEE_STING,
    &VILLAGE_ELDER,
    &WAITING_IN_THE_WEEDS,
    &WALL_OF_ROOTS,
    &WILD_ELEPHANT,
    &WORLDLY_TUTOR,
    &ASMIRA_HOLY_AVENGER,
    &BENTHIC_DJINN,
    &CADAVEROUS_BLOOM,
    &CIRCLE_OF_DESPAIR,
    &DELIRIUM,
    &DISCORDANT_SPIRIT,
    &EMBERWILDE_CALIPH,
    &ENERGY_BOLT,
    &FRENETIC_EFREET,
    &GRIM_FEAST,
    &HARBOR_GUARDIAN,
    &HAUNTING_APPARITION,
    &HAZERIDER_DRAKE,
    &JUNGLE_TROLL,
    &KAERVEK_S_PURGE,
    &LEERING_GARGOYLE,
    &MALIGNANT_GROWTH,
    &PHYREXIAN_PURGE,
    &PRISMATIC_BOON,
    &PURGATORY,
    &RADIANT_ESSENCE,
    &REFLECT_DAMAGE,
    &REPARATIONS,
    &ROCK_BASILISK,
    &SAVAGE_TWISTER,
    &SAWBACK_MANTICORE,
    &SEALED_FATE,
    &SHAUKU_S_MINION,
    &SPATIAL_BINDING,
    &UNFULFILLED_DESIRES,
    &VITALIZING_CASCADE,
    &WARPING_WURM,
    &WELLSPRING,
    &WINDREAPER_FALCON,
    &ZEBRA_UNICORN,
    &ACIDIC_DAGGER,
    &AMBER_PRISON,
    &AMULET_OF_UNMAKING,
    &BASALT_GOLEM,
    &BONE_MASK,
    &CHARCOAL_DIAMOND,
    &CHARIOT_OF_THE_SUN,
    &CRYSTAL_GOLEM,
    &CURSED_TOTEM,
    &ELIXIR_OF_VITALITY,
    &ERSATZ_GNOMES,
    &FIRE_DIAMOND,
    &GRINNING_TOTEM,
    &HORRIBLE_HORDES,
    &IGNEOUS_GOLEM,
    &LEAD_GOLEM,
    &LION_S_EYE_DIAMOND,
    &MANA_PRISM,
    &MANGARA_S_TOME,
    &MARBLE_DIAMOND,
    &MISERS_CAGE,
    &MOSS_DIAMOND,
    &PATAGIA_GOLEM,
    &PAUPERS_CAGE,
    &PHYREXIAN_DREADNOUGHT,
    &PHYREXIAN_VAULT,
    &RAZOR_PENDULUM,
    &SAND_GOLEM,
    &SKY_DIAMOND,
    &TEEKA_S_DRAGON,
    &TELIM_TOR_S_DARTS,
    &UNERRING_SLING,
    &VENTIFACT_BOTTLE,
    &BAD_RIVER,
    &CRYSTAL_VEIN,
    &FLOOD_PLAIN,
    &GRASSLANDS,
    &MOUNTAIN_VALLEY,
    &ROCKY_TAR_PIT,
    &TEFERI_S_ISLE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    DISENCHANT_REPRINT,
    DIVINE_OFFERING_REPRINT,
    HEALING_SALVE_REPRINT,
    BOOMERANG_REPRINT,
    MEMORY_LAPSE_REPRINT,
    POWER_SINK_REPRINT,
    RAY_OF_COMMAND_REPRINT,
    REALITY_RIPPLE_ALTERNATE_1,
    DARK_BANISHING_REPRINT,
    DARK_RITUAL_REPRINT,
    DRAIN_LIFE_REPRINT,
    EBONY_CHARM_ALTERNATE_1,
    FIREBREATHING_REPRINT,
    FLARE_REPRINT,
    INCINERATE_REPRINT,
    STONE_RAIN_REPRINT,
    FOG_REPRINT,
    REGENERATION_REPRINT,
    SANDSTORM_REPRINT,
    SAVAGE_TWISTER_ALTERNATE_1,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
];
