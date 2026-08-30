//! Magic 2012 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CopyAbilityDef, CopyExceptionsDef, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectRefDef, PlayerRelation,
    ReplacementEffectDef, ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// M12 1 — Aegis Angel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AEGIS_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4cdc19da-21af-45e7-ad1f-fcacd84a8d89"),
    "Aegis Angel",
    crate::card::CardArt::new("4cdc19da-21af-45e7-ad1f-fcacd84a8d89", "Aleksi Briclot"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 2 — Alabaster Mage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f82e6a81-6a45-45f9-829d-332859a32257"),
    "Alabaster Mage",
    crate::card::CardArt::new("f82e6a81-6a45-45f9-829d-332859a32257", "Izzy"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 3 — Angelic Destiny
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_DESTINY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0cd7438-fde2-4e26-9c34-52c476a971e9"),
    "Angelic Destiny",
    crate::card::CardArt::new(
        "a0cd7438-fde2-4e26-9c34-52c476a971e9",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 4 — Angel's Mercy (reprint)

// M12 5 — Arbalest Elite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARBALEST_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("094c839e-0aea-4754-af37-edf6292623e1"),
    "Arbalest Elite",
    crate::card::CardArt::new("094c839e-0aea-4754-af37-edf6292623e1", "Chris Rahn"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 6 — Archon of Justice
pub(in crate::card::sets) static ARCHON_OF_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab707e7f-8ab5-43f1-9428-6a17c1b672fa"),
    "Archon of Justice",
    crate::card::CardArt::new("dcaee06f-edc1-4c3a-9ecc-97882c1b911e", "Jason Chan"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Archon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger_with_targets(
            "When this creature dies, exile target permanent.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// M12 7 — Armored Warhorse
pub(in crate::card::sets) static ARMORED_WARHORSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52daf505-d436-4ea6-a157-4268af2ff7a8"),
    "Armored Warhorse",
    crate::card::CardArt::new("52daf505-d436-4ea6-a157-4268af2ff7a8", "rk post"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Horse"], 2, 3),
);

// M12 8 — Assault Griffin (reprint)

// M12 9 — Auramancer (reprint)

// M12 10 — Benalish Veteran
pub(in crate::card::sets) static BENALISH_VETERAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("09a5603a-88c8-4b0c-b091-6d97e873859a"),
    "Benalish Veteran",
    crate::card::CardArt::new("09a5603a-88c8-4b0c-b091-6d97e873859a", "Steven Belledin"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +1/+1 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M12 11 — Celestial Purge
pub(in crate::card::sets) static CELESTIAL_PURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31c404e8-1241-4675-b259-fbbf1dba15c4"),
    "Celestial Purge",
    crate::card::CardArt::new("75f75e85-9454-4008-aa51-a1d5965752d6", "David Palumbo"),
    crate::card::CardSet::Magic2012,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target black or red permanent.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Color(ManaColor::Black),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
        },
    )),
);

// M12 12 — Day of Judgment
pub(in crate::card::sets) static DAY_OF_JUDGMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ba873f7-a7a4-44aa-84a6-44501424dc7a"),
    "Day of Judgment",
    crate::card::CardArt::new("1ed43ed8-9490-4433-843f-9020cd3470a1", "Vincent Proce"),
    crate::card::CardSet::Magic2012,
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all creatures.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )),
);

// M12 13 — Demystify
pub(in crate::card::sets) static DEMYSTIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0df839f-dc4c-44b0-82c7-cb2037172ac5"),
    "Demystify",
    crate::card::CardArt::new("8f1b042f-f059-4e9f-a459-8682688f45cf", "Véronique Meignaud"),
    crate::card::CardSet::Magic2012,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target enchantment.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// M12 14 — Divine Favor (reprint)

// M12 15 — Elite Vanguard
pub(in crate::card::sets) static ELITE_VANGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6bda0b4b-ab5a-4d91-9dd1-7a5a145b67f5"),
    "Elite Vanguard",
    crate::card::CardArt::new("f03487e9-f584-4bbd-8335-4dd001a88b52", "Mark Tedin"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 2, 1),
);

// M12 16 — Gideon Jura
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GIDEON_JURA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e0440668-1b0e-437c-9e42-7166dd14dfe5"),
    "Gideon Jura",
    crate::card::CardArt::new("1c58b63c-e3e5-4575-849c-9a6a00821286", "Aleksi Briclot"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 17 — Gideon's Avenger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GIDEON_S_AVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb0a0d33-8862-433b-a078-82472e5f9af0"),
    "Gideon's Avenger",
    crate::card::CardArt::new("bb0a0d33-8862-433b-a078-82472e5f9af0", "Randy Gallegos"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 18 — Gideon's Lawkeeper
pub(in crate::card::sets) static GIDEON_S_LAWKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c71eb81-a077-4c85-a4ce-4ad664486bee"),
    "Gideon's Lawkeeper",
    crate::card::CardArt::new("1c71eb81-a077-4c85-a4ce-4ad664486bee", "Steve Prescott"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{W}, {T}: Tap target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// M12 19 — Grand Abolisher
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRAND_ABOLISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67e35a40-37dd-436c-b4ac-b17b04508c1f"),
    "Grand Abolisher",
    crate::card::CardArt::new("67e35a40-37dd-436c-b4ac-b17b04508c1f", "Eric Deschamps"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 20 — Griffin Rider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRIFFIN_RIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f1a5517-e442-4fbc-b8c3-fea28e5e44d2"),
    "Griffin Rider",
    crate::card::CardArt::new("3f1a5517-e442-4fbc-b8c3-fea28e5e44d2", "Steve Prescott"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 21 — Griffin Sentinel (reprint)

// M12 22 — Guardians' Pledge
pub(in crate::card::sets) static GUARDIANS_PLEDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7e6105c-8633-46f7-a7ca-2a5c36c6d548"),
    "Guardians' Pledge",
    crate::card::CardArt::new(
        "e7e6105c-8633-46f7-a7ca-2a5c36c6d548",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Magic2012,
    CardRules::new_instant(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::spell(
        "White creatures you control get +2/+2 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M12 23 — Honor of the Pure
pub(in crate::card::sets) static HONOR_OF_THE_PURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e09a2f0a-333a-4114-8b9b-f0011628cb90"),
    "Honor of the Pure",
    crate::card::CardArt::new("650a6831-c352-4ca7-9f8f-43ea99a1cf33", "Greg Staples"),
    crate::card::CardSet::Magic2012,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::static_ability(
        "White creatures you control get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    )),
);

// M12 24 — Lifelink
pub(in crate::card::sets) static LIFELINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0d881c1-24e7-4ce7-8ab1-474cb040ddd7"),
    "Lifelink",
    crate::card::CardArt::new("a8e207d4-9930-4aff-a7c8-b53bd1b5d566", "Terese Nielsen"),
    crate::card::CardSet::Magic2012,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                },
            ),
        ]),
);

// M12 25 — Mesa Enchantress
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MESA_ENCHANTRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4037d6de-f30b-483c-83a8-9a4e2978f7fc"),
    "Mesa Enchantress",
    crate::card::CardArt::new("691dcce5-ac3d-4970-b3ff-3db485f9f5c3", "Randy Gallegos"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 26 — Mighty Leap
pub(in crate::card::sets) static MIGHTY_LEAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf8e0f93-a450-4188-a735-d601a59ab108"),
    "Mighty Leap",
    crate::card::CardArt::new("446e1676-ae7d-46ee-af91-bb54e4d18a78", "rk post"),
    crate::card::CardSet::Magic2012,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains flying until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::flying()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M12 27 — Oblivion Ring (reprint)

// M12 28 — Pacifism (reprint)

// M12 29 — Peregrine Griffin
pub(in crate::card::sets) static PEREGRINE_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0296eaa6-f9fe-4fb8-af9c-04928d99e2e2"),
    "Peregrine Griffin",
    crate::card::CardArt::new("0296eaa6-f9fe-4fb8-af9c-04928d99e2e2", "Steve Prescott"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Griffin"], 2, 4)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// M12 30 — Personal Sanctuary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PERSONAL_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56f10d57-687d-4ee3-8226-bae525d56e9e"),
    "Personal Sanctuary",
    crate::card::CardArt::new("56f10d57-687d-4ee3-8226-bae525d56e9e", "Howard Lyon"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 31 — Pride Guardian
pub(in crate::card::sets) static PRIDE_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8d8d723-743c-45d6-b11b-7213f4872cf1"),
    "Pride Guardian",
    crate::card::CardArt::new("c8d8d723-743c-45d6-b11b-7213f4872cf1", "Chris Rahn"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{W}"), &["Cat", "Monk"], 0, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered(
            "Whenever this creature blocks, you gain 3 life.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::Any,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// M12 32 — Roc Egg
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROC_EGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1dca2c1f-3835-478b-860c-51b2036221b2"),
    "Roc Egg",
    crate::card::CardArt::new("92ae6206-ff0d-4248-b9cb-4ffbf20504fa", "Paul Bonner"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 33 — Serra Angel (reprint)

// M12 34 — Siege Mastodon (reprint)

// M12 35 — Spirit Mantle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_MANTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("930c8444-ccce-411e-bc4f-e5abca749608"),
    "Spirit Mantle",
    crate::card::CardArt::new("930c8444-ccce-411e-bc4f-e5abca749608", "Izzy"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 36 — Stave Off
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STAVE_OFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bb09157-5d7a-4da2-92b6-9354489e607f"),
    "Stave Off",
    crate::card::CardArt::new("3bb09157-5d7a-4da2-92b6-9354489e607f", "Mark Zug"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 37 — Stonehorn Dignitary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STONEHORN_DIGNITARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3797f7f-489d-4735-af56-6359e0fa0a6b"),
    "Stonehorn Dignitary",
    crate::card::CardArt::new("c3797f7f-489d-4735-af56-6359e0fa0a6b", "Dave Kendall"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 38 — Stormfront Pegasus
pub(in crate::card::sets) static STORMFRONT_PEGASUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2429a15-ccbe-463c-9218-968709d9e878"),
    "Stormfront Pegasus",
    crate::card::CardArt::new("bf0ba2d2-09d5-4755-a18f-40cf19d88f25", "rk post"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Pegasus"], 2, 1)
        .with_abilities(&[abilities::flying()]),
);

// M12 39 — Sun Titan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUN_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb07690e-d816-46de-84e7-617149a51b18"),
    "Sun Titan",
    crate::card::CardArt::new("ea3e77ed-9015-4407-b78c-494e46b67b07", "Todd Lockwood"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 40 — Timely Reinforcements
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TIMELY_REINFORCEMENTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ae4669c-e526-4c24-9c25-38cb5c5ef59b"),
    "Timely Reinforcements",
    crate::card::CardArt::new("6ae4669c-e526-4c24-9c25-38cb5c5ef59b", "Tomasz Jedruszek"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 41 — Aether Adept
pub(in crate::card::sets) static AETHER_ADEPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b551dab-1a81-406d-b708-b3b7300eb02e"),
    "Aether Adept",
    crate::card::CardArt::new("fa6f04ca-cab7-4c86-a56c-79d6ae3b73e6", "Eric Deschamps"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target creature to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// M12 42 — Alluring Siren
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALLURING_SIREN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("df4e1cc3-4e47-4eff-9047-c6d1cc84d635"),
    "Alluring Siren",
    crate::card::CardArt::new("a6434841-6cca-4397-b1fa-5ce34dc0b7f3", "Chippy"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 43 — Amphin Cutthroat
pub(in crate::card::sets) static AMPHIN_CUTTHROAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd169064-9c7b-40bd-8be0-a89fcb28ae2f"),
    "Amphin Cutthroat",
    crate::card::CardArt::new("fd169064-9c7b-40bd-8be0-a89fcb28ae2f", "Howard Lyon"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Salamander", "Rogue"], 2, 4),
);

// M12 44 — Aven Fleetwing
pub(in crate::card::sets) static AVEN_FLEETWING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57626fd2-d101-4e23-946f-8309c9676fe5"),
    "Aven Fleetwing",
    crate::card::CardArt::new("57626fd2-d101-4e23-946f-8309c9676fe5", "Wayne Reynolds"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird", "Soldier"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::hexproof()]),
);

// M12 45 — Azure Mage
pub(in crate::card::sets) static AZURE_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a473897f-49eb-4e0f-a5b6-ea75e10be91a"),
    "Azure Mage",
    crate::card::CardArt::new("a473897f-49eb-4e0f-a5b6-ea75e10be91a", "Izzy"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::activated(
            "{3}{U}: Draw a card.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M12 46 — Belltower Sphinx
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BELLTOWER_SPHINX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("452a23a0-62de-4561-b361-9c0de9151129"),
    "Belltower Sphinx",
    crate::card::CardArt::new("d6829959-dae1-4ddf-8a75-33a77e6b4612", "Jim Nelson"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 47 — Cancel (reprint)

// M12 48 — Chasm Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHASM_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e7e246d-92f8-4e6e-89fc-991b888fc1e8"),
    "Chasm Drake",
    crate::card::CardArt::new("5e7e246d-92f8-4e6e-89fc-991b888fc1e8", "Anthony Francisco"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 49 — Coral Merfolk (reprint)

// M12 50 — Divination (reprint)

// M12 51 — Djinn of Wishes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DJINN_OF_WISHES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e3b0949-17e1-4f12-8999-d4638d32dd3e"),
    "Djinn of Wishes",
    crate::card::CardArt::new("74c621dd-9c60-4951-beaf-eb6b597c2f0f", "Kev Walker"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 52 — Flashfreeze
pub(in crate::card::sets) static FLASHFREEZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cefd9955-a195-4855-a00e-3809b96ca92b"),
    "Flashfreeze",
    crate::card::CardArt::new("c425a629-371f-4624-b7a1-b34818ecccad", "Brian Despain"),
    crate::card::CardSet::Magic2012,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target red or green spell.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Color(ManaColor::Red),
                ObjectPredicateDef::Color(ManaColor::Green),
            ]),
        )],
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        },
    )),
);

// M12 53 — Flight (reprint)

// M12 54 — Frost Breath (reprint)

// M12 55 — Frost Titan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FROST_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("065addc8-c235-43cc-a54f-b582826e5df1"),
    "Frost Titan",
    crate::card::CardArt::new("358baa9f-390f-4b99-a274-d28f3bd56824", "Mike Bierek"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 56 — Harbor Serpent (reprint)

// M12 57 — Ice Cage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_CAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d18c4d7-c779-473b-9b41-f22b439bb501"),
    "Ice Cage",
    crate::card::CardArt::new("a5e14b62-c050-4d43-aeee-873f46d1e295", "Mike Bierek"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 58 — Jace, Memory Adept (reprint)

// M12 59 — Jace's Archivist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JACE_S_ARCHIVIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47c6b294-3840-4007-a4e3-67309f6581dd"),
    "Jace's Archivist",
    crate::card::CardArt::new("47c6b294-3840-4007-a4e3-67309f6581dd", "James Ryman"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 60 — Jace's Erasure
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JACE_S_ERASURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3662d1cc-1279-409f-9f0a-9c15c3407103"),
    "Jace's Erasure",
    crate::card::CardArt::new("970f4f34-f834-41a7-aff1-7cef82cefc74", "Jason Chan"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 61 — Levitation
pub(in crate::card::sets) static LEVITATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca18a2e7-6b01-4d10-82b5-0c1cb6ba0d2b"),
    "Levitation",
    crate::card::CardArt::new("63e5124a-67c0-44ed-8085-28bf37816423", "Jim Murray"),
    crate::card::CardSet::Magic2012,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have flying.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::flying()),
        },
    )),
);

// M12 62 — Lord of the Unreal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_OF_THE_UNREAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b09140f6-fa75-4bee-9ca0-3a71cd2b5a7b"),
    "Lord of the Unreal",
    crate::card::CardArt::new("b09140f6-fa75-4bee-9ca0-3a71cd2b5a7b", "Jason Chan"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 63 — Mana Leak (reprint)

// M12 64 — Master Thief
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_THIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77c273d3-ef0f-40c6-baf5-e39279d10509"),
    "Master Thief",
    crate::card::CardArt::new(
        "77c273d3-ef0f-40c6-baf5-e39279d10509",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 65 — Merfolk Looter
pub(in crate::card::sets) static MERFOLK_LOOTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fbb1c41-388f-4ff2-af37-ad64a0f4618e"),
    "Merfolk Looter",
    crate::card::CardArt::new("aad3aaec-7c88-4925-8023-0cf61bf906c2", "Austin Hsu"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}: Draw a card, then discard a card.",
            &[AbilityCostDef::TapSource],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ),
);

// M12 66 — Merfolk Mesmerist
pub(in crate::card::sets) static MERFOLK_MESMERIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("220dede5-472c-4a09-bdf0-73e722d9d4d2"),
    "Merfolk Mesmerist",
    crate::card::CardArt::new(
        "220dede5-472c-4a09-bdf0-73e722d9d4d2",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{U}, {T}: Target player mills two cards.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                binding: None,
                then: None,
            },
        ),
    ),
);

// M12 67 — Mind Control
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_CONTROL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37151305-e489-4df1-9b0a-c5e11c77d2f1"),
    "Mind Control",
    crate::card::CardArt::new("ec7f77af-17d7-4746-bc83-f455b9b6f9ea", "Ryan Pancoast"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 68 — Mind Unbound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_UNBOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd90cf36-9841-4adf-b5cb-0a7bf103eb93"),
    "Mind Unbound",
    crate::card::CardArt::new("fd90cf36-9841-4adf-b5cb-0a7bf103eb93", "Jason Felix"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 69 — Negate (reprint)

// M12 70 — Phantasmal Bear
pub(in crate::card::sets) static PHANTASMAL_BEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06cc574a-f687-4e41-b0a0-62a0eedea7c2"),
    "Phantasmal Bear",
    crate::card::CardArt::new("06cc574a-f687-4e41-b0a0-62a0eedea7c2", "Ryan Yee"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{U}"), &["Bear", "Illusion"], 2, 2).with_ability(
        AbilityDef::triggered(
            "When this creature becomes the target of a spell or ability, sacrifice it.",
            TriggerEventDef::BecomesTargetOfSpellOrAbility(ObjectPredicateDef::Any),
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// M12 71 — Phantasmal Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2cd015c-0569-4e7f-9daf-b39e67fc7096"),
    "Phantasmal Dragon",
    crate::card::CardArt::new("a2cd015c-0569-4e7f-9daf-b39e67fc7096", "Wayne Reynolds"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 72 — Phantasmal Image
static PHANTASMAL_IMAGE_SACRIFICE: AbilityDef = AbilityDef::triggered(
    "When this creature becomes the target of a spell or ability, sacrifice it.",
    TriggerEventDef::BecomesTargetOfSpellOrAbility(ObjectPredicateDef::Any),
    EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
);

pub(in crate::card::sets) static PHANTASMAL_IMAGE: CardRecord = CardRecord::new_with_legacy_id(
    2276,
    "Phantasmal Image",
    CardArt::new("98e7bf8f-dba7-4005-8cee-634c9153931d", "Nils Hamm"),
    CardSet::Magic2012,
    // Two mana for the best creature on the board, which the cube is happy to
    // pay because the drawback only matters to a deck holding removal.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Illusion"], 0, 0)
        .with_abilities(&[
            AbilityDef::replacement(
                "You may have this creature enter as a copy of any creature on the battlefield, except \
                 it's an Illusion in addition to its other types and it has \"When this creature becomes \
                 the target of a spell or ability, sacrifice it.\"",
                ReplacementEffectDef::CopyEntering {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    exceptions: CopyExceptionsDef::NONE
                        .with_added_creature_types(&["Illusion"])
                        .with_abilities(&[CopyAbilityDef::Ability(&PHANTASMAL_IMAGE_SACRIFICE)]),
                },
            ),
            PHANTASMAL_IMAGE_SACRIFICE,
        ]),
);

// M12 73 — Ponder (reprint)

// M12 74 — Redirect (reprint)

// M12 75 — Skywinder Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKYWINDER_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("628213e9-bde9-43fd-a0d9-8c7fb17be879"),
    "Skywinder Drake",
    crate::card::CardArt::new("628213e9-bde9-43fd-a0d9-8c7fb17be879", "Dan Murayama Scott"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 76 — Sphinx of Uthuun (reprint)

// M12 77 — Time Reversal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_REVERSAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1468c851-b20e-4c78-9fcb-45e60b7149db"),
    "Time Reversal",
    crate::card::CardArt::new("2d6500a1-5aea-4b83-b4dc-560fe547590d", "Howard Lyon"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 78 — Turn to Frog
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TURN_TO_FROG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b43d9a1e-0767-4a9b-81b4-4ff2f3dde1d5"),
    "Turn to Frog",
    crate::card::CardArt::new("b43d9a1e-0767-4a9b-81b4-4ff2f3dde1d5", "Warren Mahy"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 79 — Unsummon (reprint)

// M12 80 — Visions of Beyond
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VISIONS_OF_BEYOND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75657d26-b0f8-4892-8684-533c103c921d"),
    "Visions of Beyond",
    crate::card::CardArt::new("75657d26-b0f8-4892-8684-533c103c921d", "Terese Nielsen"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 81 — Blood Seeker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_SEEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1abc9e8-9ecf-4665-9ea5-ee18ab83c148"),
    "Blood Seeker",
    crate::card::CardArt::new("8033de8d-a396-4097-aedd-f9facb800b33", "Greg Staples"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 82 — Bloodlord of Vaasgoth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODLORD_OF_VAASGOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("972fafa6-a001-4597-962b-1aed8e40adad"),
    "Bloodlord of Vaasgoth",
    crate::card::CardArt::new("125c5cff-d4e9-4655-9cc5-3ce21e577569", "Greg Staples"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 83 — Bloodrage Vampire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODRAGE_VAMPIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a078e438-fcf9-4648-95dc-3d4037f9b561"),
    "Bloodrage Vampire",
    crate::card::CardArt::new("a078e438-fcf9-4648-95dc-3d4037f9b561", "Steve Prescott"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 84 — Brink of Disaster
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRINK_OF_DISASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c841c3e-e0d1-49d7-bcec-3c45f73c13c5"),
    "Brink of Disaster",
    crate::card::CardArt::new(
        "dbab78cd-a899-4c5d-86b3-0666adadba87",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 85 — Call to the Grave
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_TO_THE_GRAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a346b4a-ac8a-4f99-9ed7-dd41102e56ce"),
    "Call to the Grave",
    crate::card::CardArt::new("5e1324b6-dba0-4aff-a403-a45d2b405f5b", "Daarken"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 86 — Cemetery Reaper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CEMETERY_REAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("639b48f0-3426-46cf-b857-4611f7de4826"),
    "Cemetery Reaper",
    crate::card::CardArt::new("56494d1e-0d7e-4c29-942c-b376ff07cdf8", "Dave Allsop"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 87 — Child of Night (reprint)

// M12 88 — Consume Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONSUME_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f375a49c-806a-4d8b-9513-6b4afc19497b"),
    "Consume Spirit",
    crate::card::CardArt::new("ef144439-fc8e-4844-8ebb-3e36e05ac9a0", "Justin Sweet"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 89 — Dark Favor (reprint)

// M12 90 — Deathmark
pub(in crate::card::sets) static DEATHMARK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e72e8728-d0a0-4ee5-87c3-092ca94225e0"),
    "Deathmark",
    crate::card::CardArt::new("b101ff4a-8617-4c0a-8503-ed8c857ad000", "Steven Belledin"),
    crate::card::CardSet::Magic2012,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target green or white creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// M12 91 — Devouring Swarm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOURING_SWARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("735c2c79-9b4f-4f86-9dec-0749237fe9ce"),
    "Devouring Swarm",
    crate::card::CardArt::new("735c2c79-9b4f-4f86-9dec-0749237fe9ce", "Wayne England"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 92 — Diabolic Tutor (reprint)

// M12 93 — Disentomb (reprint)

// M12 94 — Distress
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISTRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8130a902-3a03-4473-a64f-84cf3590f4c6"),
    "Distress",
    crate::card::CardArt::new("630d4080-8183-41fb-8091-740719083765", "Michael C. Hayes"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 95 — Doom Blade (reprint)

// M12 96 — Drifting Shade
pub(in crate::card::sets) static DRIFTING_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00dcb25e-764b-47d6-bec4-225aaace77b0"),
    "Drifting Shade",
    crate::card::CardArt::new("00dcb25e-764b-47d6-bec4-225aaace77b0", "Tomasz Jedruszek"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Shade"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M12 97 — Duskhunter Bat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKHUNTER_BAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4560ee1a-1076-4ec5-a177-55ffe12e2165"),
    "Duskhunter Bat",
    crate::card::CardArt::new("4560ee1a-1076-4ec5-a177-55ffe12e2165", "Jesper Ejsing"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 98 — Grave Titan
pub(in crate::card::sets) static GRAVE_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fa6d385-6b8e-45ad-83dc-b477799c05a5"),
    "Grave Titan",
    CardArt::new("5c70da33-ce5d-4b8b-9c1d-9a356a7e196f", "Nils Hamm"),
    CardSet::Magic2012,
    // Ten power over three bodies for six mana, and killing the Titan still
    // leaves four of it behind.
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Giant"], 6, 6)
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, create two 2/2 black Zombie creature tokens.",
                // One printed ability with two ways in, the way every Titan prints it: a
                // Titan that lands and then attacks makes four Zombies, and it makes them
                // as two separate triggers.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_amount(2),
            ),
        ]),
);

// M12 99 — Gravedigger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEDIGGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b979d70e-d514-420f-886c-f60e2bb1861f"),
    "Gravedigger",
    crate::card::CardArt::new("11055d4e-3efe-493c-8c18-9e2642267511", "Dermot Power"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 100 — Hideous Visage
pub(in crate::card::sets) static HIDEOUS_VISAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25925751-b6cb-45a3-915f-d5ec3edcda78"),
    "Hideous Visage",
    crate::card::CardArt::new("25925751-b6cb-45a3-915f-d5ec3edcda78", "Nils Hamm"),
    crate::card::CardSet::Magic2012,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell(
        "Creatures you control gain intimidate until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::intimidate()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M12 101 — Mind Rot (reprint)

// M12 102 — Monomania
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MONOMANIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6af53d7f-7f02-4c35-b6f4-7365d121ba54"),
    "Monomania",
    crate::card::CardArt::new("6af53d7f-7f02-4c35-b6f4-7365d121ba54", "James Ryman"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 103 — Onyx Mage
pub(in crate::card::sets) static ONYX_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eabd38e6-1e59-42d2-bd1a-555c77cf6747"),
    "Onyx Mage",
    crate::card::CardArt::new("eabd38e6-1e59-42d2-bd1a-555c77cf6747", "Izzy"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{B}: Target creature you control gains deathtouch until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M12 104 — Reassembling Skeleton
pub(in crate::card::sets) static REASSEMBLING_SKELETON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("655f983e-3b23-48ee-89d5-d01d469d5a6f"),
    "Reassembling Skeleton",
    crate::card::CardArt::new("75c219bc-a140-4ecd-953a-eef2cc552d58", "Austin Hsu"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton", "Warrior"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}{B}: Return this card from your graveyard to the battlefield tapped.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::object(ObjectRefDef::Source),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                arrival: crate::card::BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::Tapped],
                    ..crate::card::BattlefieldArrivalDef::DEFAULT
                },
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ),
);

// M12 105 — Royal Assassin (reprint)

// M12 106 — Rune-Scarred Demon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_SCARRED_DEMON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("509e0f81-0591-4b28-978e-a2f1c46b7427"),
    "Rune-Scarred Demon",
    crate::card::CardArt::new("509e0f81-0591-4b28-978e-a2f1c46b7427", "Michael Komarck"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 107 — Sengir Vampire (reprint)

// M12 108 — Smallpox
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SMALLPOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("175d5a88-2597-4e85-aed6-7a65c0595fb4"),
    "Smallpox",
    crate::card::CardArt::new("93c8159b-8c1d-480a-b517-dbd67bba1838", "Ryan Pancoast"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 109 — Sorin Markov
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SORIN_MARKOV: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29606aca-f23f-4dfe-b685-2065193109c8"),
    "Sorin Markov",
    crate::card::CardArt::new("e25b3a89-3a99-4e02-bf0c-a3cf450da1a1", "Michael Komarck"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 110 — Sorin's Thirst
pub(in crate::card::sets) static SORIN_S_THIRST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f14a435-811d-4057-93a9-ce74aa852a09"),
    "Sorin's Thirst",
    crate::card::CardArt::new("1f14a435-811d-4057-93a9-ce74aa852a09", "Karl Kopinski"),
    crate::card::CardSet::Magic2012,
    CardRules::new_instant(mana_cost!("{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "This spell deals 2 damage to target creature and you gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// M12 111 — Sorin's Vengeance
pub(in crate::card::sets) static SORIN_S_VENGEANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2cb62846-c5da-4c7c-b0d7-9b677dce68d1"),
    "Sorin's Vengeance",
    crate::card::CardArt::new(
        "2cb62846-c5da-4c7c-b0d7-9b677dce68d1",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::Magic2012,
    CardRules::new_sorcery(mana_cost!("{4}{B}{B}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "This spell deals 10 damage to target player or planeswalker and you gain 10 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(10),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(10),
                },
            ]),
        ),
    ),
);

// M12 112 — Sutured Ghoul (reprint)

// M12 113 — Taste of Blood
pub(in crate::card::sets) static TASTE_OF_BLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29268cef-da18-4c1d-9066-e0d513a61bf9"),
    "Taste of Blood",
    crate::card::CardArt::new("29268cef-da18-4c1d-9066-e0d513a61bf9", "Howard Lyon"),
    crate::card::CardSet::Magic2012,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "This spell deals 1 damage to target player or planeswalker and you gain 1 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M12 114 — Tormented Soul (reprint)

// M12 115 — Vampire Outcasts
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_OUTCASTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1286132d-1697-44da-ab97-387735265c01"),
    "Vampire Outcasts",
    crate::card::CardArt::new("1286132d-1697-44da-ab97-387735265c01", "Clint Cearley"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 116 — Vengeful Pharaoh
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEFUL_PHARAOH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12e0ca97-bc57-4084-86b4-e2e06152cb1c"),
    "Vengeful Pharaoh",
    crate::card::CardArt::new("12e0ca97-bc57-4084-86b4-e2e06152cb1c", "Igor Kieryluk"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 117 — Warpath Ghoul
pub(in crate::card::sets) static WARPATH_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c6cc262-ba0c-4cca-ae9c-24a1824753e4"),
    "Warpath Ghoul",
    crate::card::CardArt::new("94785274-fa79-47cc-9896-0f5f695abb21", "rk post"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 3, 2),
);

// M12 118 — Wring Flesh (reprint)

// M12 119 — Zombie Goliath (reprint)

// M12 120 — Zombie Infestation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_INFESTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccd5f98a-7ab5-44b3-850c-b50963dace66"),
    "Zombie Infestation",
    crate::card::CardArt::new("c84a3e27-841a-4eb5-afcd-ddb87d4280f7", "Thomas M. Baxa"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 121 — Act of Treason (reprint)

// M12 122 — Blood Ogre
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b85ecba6-fc22-48c7-9f00-066cc1fce6b5"),
    "Blood Ogre",
    crate::card::CardArt::new(
        "b85ecba6-fc22-48c7-9f00-066cc1fce6b5",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 123 — Bonebreaker Giant
pub(in crate::card::sets) static BONEBREAKER_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc17e5c1-a6b4-401b-95eb-1c01cd1da570"),
    "Bonebreaker Giant",
    crate::card::CardArt::new("cc17e5c1-a6b4-401b-95eb-1c01cd1da570", "Kev Walker"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Giant"], 4, 4),
);

// M12 124 — Chandra, the Firebrand (reprint)

// M12 125 — Chandra's Outrage (reprint)

// M12 126 — Chandra's Phoenix (reprint)

// M12 127 — Circle of Flame
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_FLAME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("419b1813-9760-47b9-b6f3-e501586cfe4d"),
    "Circle of Flame",
    crate::card::CardArt::new("419b1813-9760-47b9-b6f3-e501586cfe4d", "Jaime Jones"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 128 — Combust
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COMBUST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf23a422-25a7-4c8a-9cff-24563ec20ea7"),
    "Combust",
    crate::card::CardArt::new("f10346e2-46bd-4257-b191-c36c2577c534", "Jaime Jones"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 129 — Crimson Mage
pub(in crate::card::sets) static CRIMSON_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f69ccfc-e2a9-40af-b8ab-85bffe62c0f4"),
    "Crimson Mage",
    crate::card::CardArt::new("0f69ccfc-e2a9-40af-b8ab-85bffe62c0f4", "Izzy"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Shaman"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}: Target creature you control gains haste until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M12 130 — Fiery Hellhound
pub(in crate::card::sets) static FIERY_HELLHOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d6b2c8a-8019-4e4b-8f4e-058ab5284153"),
    "Fiery Hellhound",
    crate::card::CardArt::new("7c96f7a0-99a3-4ba4-b0f0-9ea36c45d5d5", "Ted Galaday"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Elemental", "Dog"], 2, 2).with_ability(
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M12 131 — Fireball (reprint)

// M12 132 — Firebreathing (reprint)

// M12 133 — Flameblast Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMEBLAST_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5544b26b-0bc4-4c1b-9616-613e9bf08557"),
    "Flameblast Dragon",
    crate::card::CardArt::new("c01ab5c8-f9b7-482c-a900-1388b727b89f", "Jaime Jones"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 134 — Fling (reprint)

// M12 135 — Furyborn Hellkite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FURYBORN_HELLKITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5b735e5-da9d-4740-acff-aac9dd24334c"),
    "Furyborn Hellkite",
    crate::card::CardArt::new("b5b735e5-da9d-4740-acff-aac9dd24334c", "Brad Rigney"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 136 — Goblin Arsonist (reprint)

// M12 137 — Goblin Bangchuckers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BANGCHUCKERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b56ddad0-23ea-4139-a200-c76c9c46e8c5"),
    "Goblin Bangchuckers",
    crate::card::CardArt::new("b56ddad0-23ea-4139-a200-c76c9c46e8c5", "Wayne Reynolds"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 138 — Goblin Chieftain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_CHIEFTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5c8a4a4-1611-4188-9c59-8aefb016b5ad"),
    "Goblin Chieftain",
    crate::card::CardArt::new("2540ec6b-9ffa-4ab0-bbd3-ddf1efd2db60", "Sam Wood"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 139 — Goblin Fireslinger
pub(in crate::card::sets) static GOBLIN_FIRESLINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c11db78-f506-4af2-a7be-c7ac2c0ffcf3"),
    "Goblin Fireslinger",
    crate::card::CardArt::new("3c11db78-f506-4af2-a7be-c7ac2c0ffcf3", "Pete Venters"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target player or planeswalker.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M12 140 — Goblin Grenade (reprint)

// M12 141 — Goblin Piker
pub(in crate::card::sets) static GOBLIN_PIKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2786834d-dbda-40ce-82a4-e518cd554312"),
    "Goblin Piker",
    crate::card::CardArt::new("083ec3e7-950c-4e9d-aba5-02ed13d723f0", "DiTerlizzi"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 1),
);

// M12 142 — Goblin Tunneler
pub(in crate::card::sets) static GOBLIN_TUNNELER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b2e4a34-6255-4f89-a62d-941996c573e1"),
    "Goblin Tunneler",
    crate::card::CardArt::new("c466bbb3-9758-47e6-8996-3615f4c31924", "Jesper Ejsing"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature with power 2 or less can't be blocked this turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M12 143 — Goblin War Paint
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_WAR_PAINT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4388e57e-0c87-4d66-a862-58261d76c5ac"),
    "Goblin War Paint",
    crate::card::CardArt::new("fde711c9-fdef-4024-8269-a59ee0748f95", "Austin Hsu"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 144 — Gorehorn Minotaurs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOREHORN_MINOTAURS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1087e015-a3c4-4207-8285-5bda6bb50e52"),
    "Gorehorn Minotaurs",
    crate::card::CardArt::new("1087e015-a3c4-4207-8285-5bda6bb50e52", "Wayne Reynolds"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 145 — Grim Lavamancer (reprint)

// M12 146 — Incinerate (reprint)

// M12 147 — Inferno Titan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNO_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1e4a028-6462-4373-9864-a8adfc78d52b"),
    "Inferno Titan",
    crate::card::CardArt::new("e04c24cb-3c3b-4a35-9694-db512bf394fa", "Kev Walker"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 148 — Lava Axe (reprint)

// M12 149 — Lightning Elemental
pub(in crate::card::sets) static LIGHTNING_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11f6d2a4-cc97-43f3-a8b2-f96262c27371"),
    "Lightning Elemental",
    crate::card::CardArt::new("e106b6af-a13c-42be-9368-9109795de517", "Kev Walker"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 4, 1)
        .with_abilities(&[abilities::haste()]),
);

// M12 150 — Manabarbs (reprint)

// M12 151 — Manic Vandal
pub(in crate::card::sets) static MANIC_VANDAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a503697a-4940-4b8f-98b1-5ea9151866fa"),
    "Manic Vandal",
    crate::card::CardArt::new(
        "985a5866-8c62-46af-a0c0-e69d01d87f4f",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target artifact.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// M12 152 — Reverberate (reprint)

// M12 153 — Scrambleverse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAMBLEVERSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b61fa9d-3f69-4632-be0e-09924ca88501"),
    "Scrambleverse",
    crate::card::CardArt::new("2b61fa9d-3f69-4632-be0e-09924ca88501", "Dan Murayama Scott"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 154 — Shock (reprint)

// M12 155 — Slaughter Cry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLAUGHTER_CRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c93b0eda-693e-4a17-be1d-1df162702146"),
    "Slaughter Cry",
    crate::card::CardArt::new("65ec8b61-e602-41f2-ac1a-64e150b2ce18", "Matt Cavotta"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 156 — Stormblood Berserker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORMBLOOD_BERSERKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc9a50af-ca3e-461a-9dcb-444f56284165"),
    "Stormblood Berserker",
    crate::card::CardArt::new("fc9a50af-ca3e-461a-9dcb-444f56284165", "Min Yum"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 157 — Tectonic Rift
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TECTONIC_RIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9838784-8c6d-4e64-bc34-e21efde99093"),
    "Tectonic Rift",
    crate::card::CardArt::new("e9838784-8c6d-4e64-bc34-e21efde99093", "John Avon"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 158 — Volcanic Dragon
pub(in crate::card::sets) static VOLCANIC_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bda123a7-d121-483a-8ff0-a541ccdbc7ca"),
    "Volcanic Dragon",
    crate::card::CardArt::new("56134669-9575-44bc-9203-edbd75acecbd", "Chris Rahn"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dragon"], 4, 4)
        .with_abilities(&[abilities::flying(), abilities::haste()]),
);

// M12 159 — Wall of Torches
pub(in crate::card::sets) static WALL_OF_TORCHES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76f69b92-7435-4aa8-9d90-89ea078befb1"),
    "Wall of Torches",
    crate::card::CardArt::new("76f69b92-7435-4aa8-9d90-89ea078befb1", "Mike Bierek"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Wall"], 4, 1)
        .with_abilities(&[abilities::defender()]),
);

// M12 160 — Warstorm Surge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WARSTORM_SURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b16443df-52c6-4c9d-a7ff-89a37e593a0a"),
    "Warstorm Surge",
    crate::card::CardArt::new("b16443df-52c6-4c9d-a7ff-89a37e593a0a", "Raymond Swanland"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 161 — Acidic Slime (reprint)

// M12 162 — Arachnus Spinner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARACHNUS_SPINNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23321b80-d7e7-48fd-985d-1e9dc3adcd35"),
    "Arachnus Spinner",
    crate::card::CardArt::new("23321b80-d7e7-48fd-985d-1e9dc3adcd35", "Karl Kopinski"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 163 — Arachnus Web
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARACHNUS_WEB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e4900b0-f934-42d9-92fb-0bb16d2e8bb1"),
    "Arachnus Web",
    crate::card::CardArt::new("9e4900b0-f934-42d9-92fb-0bb16d2e8bb1", "Karl Kopinski"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 164 — Autumn's Veil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AUTUMN_S_VEIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e354ce5-b4c1-4a9c-99d1-7624301b594b"),
    "Autumn's Veil",
    crate::card::CardArt::new("b911fee0-c30b-4d68-a9e2-61c40ece68b0", "Kekai Kotaki"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 165 — Birds of Paradise (reprint)

// M12 166 — Bountiful Harvest (reprint)

// M12 167 — Brindle Boar (reprint)

// M12 168 — Carnage Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARNAGE_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c086eb41-3524-4815-97c9-761ba86a30b2"),
    "Carnage Wurm",
    crate::card::CardArt::new("c086eb41-3524-4815-97c9-761ba86a30b2", "Dave Kendall"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 169 — Cudgel Troll
pub(in crate::card::sets) static CUDGEL_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d779b14c-a100-4382-9e7c-0969efda73ec"),
    "Cudgel Troll",
    crate::card::CardArt::new("e156b8d8-5309-494e-9709-44f98826a69f", "Jesper Ejsing"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Troll"], 4, 3).with_ability(
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
        ),
    ),
);

// M12 170 — Doubling Chant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOUBLING_CHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f71dc232-9b7e-4c0e-ac05-8e48b4936aa9"),
    "Doubling Chant",
    crate::card::CardArt::new("f71dc232-9b7e-4c0e-ac05-8e48b4936aa9", "Wayne England"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 171 — Dungrove Elder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DUNGROVE_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8b4ebbf-1613-42a0-97ff-2f36dc8d984a"),
    "Dungrove Elder",
    crate::card::CardArt::new("b8b4ebbf-1613-42a0-97ff-2f36dc8d984a", "Matt Stewart"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 172 — Elvish Archdruid (reprint)

// M12 173 — Fog (reprint)

// M12 174 — Garruk, Primal Hunter (reprint)

// M12 175 — Garruk's Companion
pub(in crate::card::sets) static GARRUK_S_COMPANION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("863c9a10-d83f-415b-adf2-2d0f870410b2"),
    "Garruk's Companion",
    crate::card::CardArt::new("b8d8806c-43c5-4c6c-9420-6210a17ec2b0", "Efrem Palacios"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Beast"], 3, 2)
        .with_abilities(&[abilities::trample()]),
);

// M12 176 — Garruk's Horde (reprint)

// M12 177 — Giant Spider (reprint)

// M12 178 — Gladecover Scout (reprint)

// M12 179 — Greater Basilisk
pub(in crate::card::sets) static GREATER_BASILISK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("482f169d-8acd-4ee3-a54c-6df6cbeb7eca"),
    "Greater Basilisk",
    crate::card::CardArt::new("994711cb-e85b-4acb-9460-17231e1d66ad", "James Ryman"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Basilisk"], 3, 5)
        .with_abilities(&[abilities::deathtouch()]),
);

// M12 180 — Hunter's Insight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTER_S_INSIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e4044a9f-43bd-4c32-9d53-29a27ad9be80"),
    "Hunter's Insight",
    crate::card::CardArt::new("e4044a9f-43bd-4c32-9d53-29a27ad9be80", "Terese Nielsen"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 181 — Jade Mage
pub(in crate::card::sets) static JADE_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32d6c8d3-04a1-4b35-b7d1-18bed82beaf4"),
    "Jade Mage",
    crate::card::CardArt::new("32d6c8d3-04a1-4b35-b7d1-18bed82beaf4", "Izzy"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Shaman"], 2, 1).with_ability(
        AbilityDef::activated(
            "{2}{G}: Create a 1/1 green Saproling creature token.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}"))],
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
    ),
);

// M12 182 — Llanowar Elves (reprint)

// M12 183 — Lure (reprint)

// M12 184 — Lurking Crocodile
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LURKING_CROCODILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4cd7d075-031e-4766-89e9-03a8a7197019"),
    "Lurking Crocodile",
    crate::card::CardArt::new("4cd7d075-031e-4766-89e9-03a8a7197019", "Donato Giancola"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 185 — Naturalize (reprint)

// M12 186 — Overrun
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OVERRUN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ad7a961-d3a1-471a-8472-8407d1057de0"),
    "Overrun",
    crate::card::CardArt::new("ae0559d4-0015-44e4-8ec4-08bb1c54eec5", "Carl Critchlow"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 187 — Plummet (reprint)

// M12 188 — Primeval Titan (reprint)

// M12 189 — Primordial Hydra (reprint)

// M12 190 — Rampant Growth
pub(in crate::card::sets) static RAMPANT_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9dd8043-4099-42bb-9d54-4efc8b38fe18"),
    "Rampant Growth",
    crate::card::CardArt::new("fe45a787-6d8a-48d7-ad6c-fb20a9b468a4", "Steven Belledin"),
    crate::card::CardSet::Magic2012,
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

// M12 191 — Reclaim
pub(in crate::card::sets) static RECLAIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b47c082-57f6-4f69-87e8-a07cad9ef042"),
    "Reclaim",
    crate::card::CardArt::new("78f67503-2f0f-43bf-9c4f-a254cc6c501a", "Andrew Robinson"),
    crate::card::CardSet::Magic2012,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Put target card from your graveyard on top of your library.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// M12 192 — Rites of Flourishing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RITES_OF_FLOURISHING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("811458c7-dcdc-43ef-8c3e-a90e21ce315e"),
    "Rites of Flourishing",
    crate::card::CardArt::new("0e3d43ce-8297-47f6-a877-d723b9b43fdb", "Brandon Kitkouski"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 193 — Runeclaw Bear
pub(in crate::card::sets) static RUNECLAW_BEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("268bd9d5-4da1-4cbf-83f9-47f7aac1cfc3"),
    "Runeclaw Bear",
    crate::card::CardArt::new("6caf2b93-1971-4702-9aa5-bd223eb37a39", "Jesper Ejsing"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear"], 2, 2),
);

// M12 194 — Sacred Wolf
pub(in crate::card::sets) static SACRED_WOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2bffe20-c469-4ac8-a8a9-361a244f4cfe"),
    "Sacred Wolf",
    crate::card::CardArt::new("ff4661dd-2075-48c3-b19b-fc7f8aaba1b8", "Matt Stewart"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wolf"], 3, 1)
        .with_abilities(&[abilities::hexproof()]),
);

// M12 195 — Skinshifter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKINSHIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d56c82ad-5eb1-4653-8f02-e9bb1f6f3154"),
    "Skinshifter",
    crate::card::CardArt::new("d56c82ad-5eb1-4653-8f02-e9bb1f6f3154", "Matt Stewart"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 196 — Stampeding Rhino
pub(in crate::card::sets) static STAMPEDING_RHINO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5a33394-d26c-4dcd-948c-e7d370059b11"),
    "Stampeding Rhino",
    crate::card::CardArt::new("09d34690-f7cc-4161-9a6f-bfc5393e40b2", "Steven Belledin"),
    crate::card::CardSet::Magic2012,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Rhino"], 4, 4)
        .with_abilities(&[abilities::trample()]),
);

// M12 197 — Stingerfling Spider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STINGERFLING_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b781626-f4ce-4d00-aa7c-0e07f58f688f"),
    "Stingerfling Spider",
    crate::card::CardArt::new("3b781626-f4ce-4d00-aa7c-0e07f58f688f", "Dave Allsop"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 198 — Titanic Growth (reprint)

// M12 199 — Trollhide (reprint)

// M12 200 — Vastwood Gorger (reprint)

// M12 201 — Adaptive Automaton
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ADAPTIVE_AUTOMATON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79e42ead-df6e-4181-ae2b-a2abfc3f1d7c"),
    "Adaptive Automaton",
    crate::card::CardArt::new("79e42ead-df6e-4181-ae2b-a2abfc3f1d7c", "Igor Kieryluk"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 202 — Angel's Feather
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGEL_S_FEATHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a11d101-2e82-42d5-b4a1-8f0c520441ab"),
    "Angel's Feather",
    crate::card::CardArt::new("3992dc7c-61c0-4d5f-9c32-8febfad4ef6d", "Alan Pollack"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 203 — Crown of Empires
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_EMPIRES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4e36991-7b9f-4cc7-8da2-55b8baf19d70"),
    "Crown of Empires",
    crate::card::CardArt::new("d4e36991-7b9f-4cc7-8da2-55b8baf19d70", "John Avon"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 204 — Crumbling Colossus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRUMBLING_COLOSSUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b09afa3b-c172-4cd7-b605-bacbfbd07c24"),
    "Crumbling Colossus",
    crate::card::CardArt::new("b09afa3b-c172-4cd7-b605-bacbfbd07c24", "Michael C. Hayes"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 205 — Demon's Horn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEMON_S_HORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41d40eb4-643a-4e22-a15f-eda45a48cfd6"),
    "Demon's Horn",
    crate::card::CardArt::new("3f56b129-fe2d-4061-b1c9-f1f5a4db564a", "Alan Pollack"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 206 — Dragon's Claw
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_S_CLAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a46bbcc-b287-47bb-b252-5dd3217f61a9"),
    "Dragon's Claw",
    crate::card::CardArt::new("0d732b87-08e5-41b6-8448-62dd6bf20d9c", "Alan Pollack"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 207 — Druidic Satchel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRUIDIC_SATCHEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fddb054f-0617-4afb-8ed1-a067f234f8e7"),
    "Druidic Satchel",
    crate::card::CardArt::new("fddb054f-0617-4afb-8ed1-a067f234f8e7", "Matt Stewart"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 208 — Elixir of Immortality (reprint)

// M12 209 — Greatsword
pub(in crate::card::sets) static GREATSWORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63b4041d-7c95-4cb9-a18b-6568db05942b"),
    "Greatsword",
    crate::card::CardArt::new("63b4041d-7c95-4cb9-a18b-6568db05942b", "Nic Klein"),
    crate::card::CardSet::Magic2012,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +3/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// M12 210 — Kite Shield
pub(in crate::card::sets) static KITE_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a00d1e1-aaa4-4f4d-a887-1e477820d2c6"),
    "Kite Shield",
    crate::card::CardArt::new("1a00d1e1-aaa4-4f4d-a887-1e477820d2c6", "Jim Pavelec"),
    crate::card::CardSet::Magic2012,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +0/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(3),
                    ),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// M12 211 — Kraken's Eye
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KRAKEN_S_EYE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc767637-627a-4ea2-873b-d8a80ccc925b"),
    "Kraken's Eye",
    crate::card::CardArt::new("48052433-c4d3-434e-a609-e8400150a0f6", "Alan Pollack"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 212 — Manalith
pub(in crate::card::sets) static MANALITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17bf5f25-82b4-460c-94da-b84daa8a53d9"),
    "Manalith",
    crate::card::CardArt::new("17bf5f25-82b4-460c-94da-b84daa8a53d9", "Charles Urbach"),
    crate::card::CardSet::Magic2012,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any color.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// M12 213 — Pentavus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PENTAVUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32a11f0a-7547-4fda-a8ed-caf76ce98f10"),
    "Pentavus",
    crate::card::CardArt::new("eb10af81-8ff3-4063-a67a-b760fdba95f8", "Greg Staples"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 214 — Quicksilver Amulet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKSILVER_AMULET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ecfdebbe-6432-426f-ac2a-5a9af3047813"),
    "Quicksilver Amulet",
    crate::card::CardArt::new("04c0357a-e98d-4c49-83ad-d7a8ebe7e2d1", "Brad Rigney"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 215 — Rusted Sentinel
pub(in crate::card::sets) static RUSTED_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cba5fc44-4b9a-418b-a4e0-26d2c3a1eca4"),
    "Rusted Sentinel",
    crate::card::CardArt::new("cba5fc44-4b9a-418b-a4e0-26d2c3a1eca4", "Jason Felix"),
    crate::card::CardSet::Magic2012,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 3, 4)
        .with_ability(abilities::enters_tapped("This creature enters tapped.")),
);

// M12 216 — Scepter of Empires
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCEPTER_OF_EMPIRES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54f1aaef-94cc-45ab-99c9-8ffdcf331a7b"),
    "Scepter of Empires",
    crate::card::CardArt::new("54f1aaef-94cc-45ab-99c9-8ffdcf331a7b", "John Avon"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 217 — Solemn Simulacrum
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOLEMN_SIMULACRUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00f9955f-a522-47bf-b064-92dd21a76b18"),
    "Solemn Simulacrum",
    crate::card::CardArt::new("246d2ce1-6926-4acc-810a-4894dc346b8b", "Dan Murayama Scott"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 218 — Sundial of the Infinite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNDIAL_OF_THE_INFINITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36d3da9c-cb7a-4cea-b6e6-6722bd16c73c"),
    "Sundial of the Infinite",
    crate::card::CardArt::new("36d3da9c-cb7a-4cea-b6e6-6722bd16c73c", "Vincent Proce"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 219 — Swiftfoot Boots
pub(in crate::card::sets) static SWIFTFOOT_BOOTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b82753b-284c-44ba-9d48-d28913f02a5f"),
    "Swiftfoot Boots",
    crate::card::CardArt::new("8b82753b-284c-44ba-9d48-d28913f02a5f", "Svetlin Velinov"),
    crate::card::CardSet::Magic2012,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has hexproof and haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// M12 220 — Thran Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5778c52b-248b-4131-b5c0-12ea1986786e"),
    "Thran Golem",
    crate::card::CardArt::new("f01b98a6-5683-4b1b-a14c-d0b50fc26beb", "Ron Spears"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 221 — Throne of Empires
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRONE_OF_EMPIRES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87352716-4cf6-4b2f-bb0a-b7aafae64478"),
    "Throne of Empires",
    crate::card::CardArt::new("87352716-4cf6-4b2f-bb0a-b7aafae64478", "John Avon"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 222 — Worldslayer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDSLAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cb1b869-3e2d-4447-a12d-e790883feeee"),
    "Worldslayer",
    crate::card::CardArt::new("db6c6b15-40f3-4556-978f-878bedb13762", "Greg Staples"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 223 — Wurm's Tooth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WURM_S_TOOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("482cdbe0-b865-4e09-bd30-61ab93739b53"),
    "Wurm's Tooth",
    crate::card::CardArt::new("da965767-a8b1-4725-ae20-65c18e37ad27", "Alan Pollack"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 224 — Buried Ruin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BURIED_RUIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e910cf59-f7aa-44b1-bb8a-c2211179137c"),
    "Buried Ruin",
    crate::card::CardArt::new("e910cf59-f7aa-44b1-bb8a-c2211179137c", "Franz Vohwinkel"),
    crate::card::CardSet::Magic2012,
    crate::card::CardRules::unsupported(),
);

// M12 225 — Dragonskull Summit (reprint)

// M12 226 — Drowned Catacomb (reprint)

// M12 227 — Glacial Fortress (reprint)

// M12 228 — Rootbound Crag (reprint)

// M12 229 — Sunpetal Grove (reprint)

// M12 230 — Plains (reprint)

// M12 231 — Plains (alternate printing)

// M12 232 — Plains (alternate printing)

// M12 233 — Plains (alternate printing)

// M12 234 — Island (reprint)

// M12 235 — Island (alternate printing)

// M12 236 — Island (alternate printing)

// M12 237 — Island (alternate printing)

// M12 238 — Swamp (reprint)

// M12 239 — Swamp (alternate printing)

// M12 240 — Swamp (alternate printing)

// M12 241 — Swamp (alternate printing)

// M12 242 — Mountain (reprint)

// M12 243 — Mountain (alternate printing)

// M12 244 — Mountain (alternate printing)

// M12 245 — Mountain (alternate printing)

// M12 246 — Forest (reprint)

// M12 247 — Forest (alternate printing)

// M12 248 — Forest (alternate printing)

// M12 249 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AEGIS_ANGEL,
    &ALABASTER_MAGE,
    &ANGELIC_DESTINY,
    &ARBALEST_ELITE,
    &ARCHON_OF_JUSTICE,
    &ARMORED_WARHORSE,
    &BENALISH_VETERAN,
    &CELESTIAL_PURGE,
    &DAY_OF_JUDGMENT,
    &DEMYSTIFY,
    &ELITE_VANGUARD,
    &GIDEON_JURA,
    &GIDEON_S_AVENGER,
    &GIDEON_S_LAWKEEPER,
    &GRAND_ABOLISHER,
    &GRIFFIN_RIDER,
    &GUARDIANS_PLEDGE,
    &HONOR_OF_THE_PURE,
    &LIFELINK,
    &MESA_ENCHANTRESS,
    &MIGHTY_LEAP,
    &PEREGRINE_GRIFFIN,
    &PERSONAL_SANCTUARY,
    &PRIDE_GUARDIAN,
    &ROC_EGG,
    &SPIRIT_MANTLE,
    &STAVE_OFF,
    &STONEHORN_DIGNITARY,
    &STORMFRONT_PEGASUS,
    &SUN_TITAN,
    &TIMELY_REINFORCEMENTS,
    &AETHER_ADEPT,
    &ALLURING_SIREN,
    &AMPHIN_CUTTHROAT,
    &AVEN_FLEETWING,
    &AZURE_MAGE,
    &BELLTOWER_SPHINX,
    &CHASM_DRAKE,
    &DJINN_OF_WISHES,
    &FLASHFREEZE,
    &FROST_TITAN,
    &ICE_CAGE,
    &JACE_S_ARCHIVIST,
    &JACE_S_ERASURE,
    &LEVITATION,
    &LORD_OF_THE_UNREAL,
    &MASTER_THIEF,
    &MERFOLK_LOOTER,
    &MERFOLK_MESMERIST,
    &MIND_CONTROL,
    &MIND_UNBOUND,
    &PHANTASMAL_BEAR,
    &PHANTASMAL_DRAGON,
    &PHANTASMAL_IMAGE,
    &SKYWINDER_DRAKE,
    &TIME_REVERSAL,
    &TURN_TO_FROG,
    &VISIONS_OF_BEYOND,
    &BLOOD_SEEKER,
    &BLOODLORD_OF_VAASGOTH,
    &BLOODRAGE_VAMPIRE,
    &BRINK_OF_DISASTER,
    &CALL_TO_THE_GRAVE,
    &CEMETERY_REAPER,
    &CONSUME_SPIRIT,
    &DEATHMARK,
    &DEVOURING_SWARM,
    &DISTRESS,
    &DRIFTING_SHADE,
    &DUSKHUNTER_BAT,
    &GRAVE_TITAN,
    &GRAVEDIGGER,
    &HIDEOUS_VISAGE,
    &MONOMANIA,
    &ONYX_MAGE,
    &REASSEMBLING_SKELETON,
    &RUNE_SCARRED_DEMON,
    &SMALLPOX,
    &SORIN_MARKOV,
    &SORIN_S_THIRST,
    &SORIN_S_VENGEANCE,
    &TASTE_OF_BLOOD,
    &VAMPIRE_OUTCASTS,
    &VENGEFUL_PHARAOH,
    &WARPATH_GHOUL,
    &ZOMBIE_INFESTATION,
    &BLOOD_OGRE,
    &BONEBREAKER_GIANT,
    &CIRCLE_OF_FLAME,
    &COMBUST,
    &CRIMSON_MAGE,
    &FIERY_HELLHOUND,
    &FLAMEBLAST_DRAGON,
    &FURYBORN_HELLKITE,
    &GOBLIN_BANGCHUCKERS,
    &GOBLIN_CHIEFTAIN,
    &GOBLIN_FIRESLINGER,
    &GOBLIN_PIKER,
    &GOBLIN_TUNNELER,
    &GOBLIN_WAR_PAINT,
    &GOREHORN_MINOTAURS,
    &INFERNO_TITAN,
    &LIGHTNING_ELEMENTAL,
    &MANIC_VANDAL,
    &SCRAMBLEVERSE,
    &SLAUGHTER_CRY,
    &STORMBLOOD_BERSERKER,
    &TECTONIC_RIFT,
    &VOLCANIC_DRAGON,
    &WALL_OF_TORCHES,
    &WARSTORM_SURGE,
    &ARACHNUS_SPINNER,
    &ARACHNUS_WEB,
    &AUTUMN_S_VEIL,
    &CARNAGE_WURM,
    &CUDGEL_TROLL,
    &DOUBLING_CHANT,
    &DUNGROVE_ELDER,
    &GARRUK_S_COMPANION,
    &GREATER_BASILISK,
    &HUNTER_S_INSIGHT,
    &JADE_MAGE,
    &LURKING_CROCODILE,
    &OVERRUN,
    &RAMPANT_GROWTH,
    &RECLAIM,
    &RITES_OF_FLOURISHING,
    &RUNECLAW_BEAR,
    &SACRED_WOLF,
    &SKINSHIFTER,
    &STAMPEDING_RHINO,
    &STINGERFLING_SPIDER,
    &ADAPTIVE_AUTOMATON,
    &ANGEL_S_FEATHER,
    &CROWN_OF_EMPIRES,
    &CRUMBLING_COLOSSUS,
    &DEMON_S_HORN,
    &DRAGON_S_CLAW,
    &DRUIDIC_SATCHEL,
    &GREATSWORD,
    &KITE_SHIELD,
    &KRAKEN_S_EYE,
    &MANALITH,
    &PENTAVUS,
    &QUICKSILVER_AMULET,
    &RUSTED_SENTINEL,
    &SCEPTER_OF_EMPIRES,
    &SOLEMN_SIMULACRUM,
    &SUNDIAL_OF_THE_INFINITE,
    &SWIFTFOOT_BOOTS,
    &THRAN_GOLEM,
    &THRONE_OF_EMPIRES,
    &WORLDSLAYER,
    &WURM_S_TOOTH,
    &BURIED_RUIN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y2012::avacyn_restored::ANGELS_MERCY), // M12 4
    PrintingRecord::reprint(&crate::card::sets::y2013::gatecrash::ASSAULT_GRIFFIN),    // M12 8
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::AURAMANCER),        // M12 9
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::DIVINE_FAVOR),      // M12 14
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::GRIFFIN_SENTINEL),  // M12 21
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::OBLIVION_RING),     // M12 27
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::PACIFISM),          // M12 28
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SERRA_ANGEL),            // M12 33
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::SIEGE_MASTODON),    // M12 34
    PrintingRecord::reprint(&crate::card::sets::y2012::return_to_ravnica::CANCEL),     // M12 47
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::CORAL_MERFOLK),     // M12 49
    PrintingRecord::reprint(&crate::card::sets::y2012::dark_ascension::DIVINATION),    // M12 50
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FLIGHT),                 // M12 53
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::FROST_BREATH),      // M12 54
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::HARBOR_SERPENT),    // M12 56
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::JACE_MEMORY_ADEPT), // M12 58
    PrintingRecord::reprint(&crate::card::sets::y1998::stronghold::MANA_LEAK),         // M12 63
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::NEGATE),            // M12 69
    PrintingRecord::reprint(&crate::card::sets::y2007::lorwyn::PONDER),                // M12 73
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::REDIRECT),          // M12 74
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::SPHINX_OF_UTHUUN),  // M12 76
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::UNSUMMON),               // M12 79
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::CHILD_OF_NIGHT),    // M12 87
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::DARK_FAVOR),        // M12 89
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::DIABOLIC_TUTOR),    // M12 92
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::DISENTOMB),         // M12 93
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::DOOM_BLADE),        // M12 95
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::MIND_ROT),          // M12 101
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::ROYAL_ASSASSIN),         // M12 105
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SENGIR_VAMPIRE),         // M12 107
    PrintingRecord::reprint(&crate::card::sets::y2002::judgment::SUTURED_GHOUL),       // M12 112
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::TORMENTED_SOUL),    // M12 114
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::WRING_FLESH),       // M12 118
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::ZOMBIE_GOLIATH),    // M12 119
    PrintingRecord::reprint(&crate::card::sets::y2013::gatecrash::ACT_OF_TREASON),     // M12 121
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::CHANDRA_THE_FIREBRAND), // M12 124
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::CHANDRAS_OUTRAGE), // M12 125
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::CHANDRA_S_PHOENIX), // M12 126
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FIREBALL),              // M12 131
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FIREBREATHING),         // M12 132
    PrintingRecord::reprint(&crate::card::sets::y2012::dark_ascension::FLING),        // M12 134
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::GOBLIN_ARSONIST),  // M12 136
    PrintingRecord::reprint(&crate::card::sets::y1994::fallen_empires::GOBLIN_GRENADE), // M12 140
    PrintingRecord::reprint(&crate::card::sets::y2002::torment::GRIM_LAVAMANCER),     // M12 145
    PrintingRecord::reprint(&crate::card::sets::y1995::ice_age::INCINERATE),          // M12 146
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::LAVA_AXE),         // M12 148
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::MANABARBS),             // M12 150
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::REVERBERATE),      // M12 152
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::SHOCK),            // M12 154
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::ACIDIC_SLIME),     // M12 161
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::BIRDS_OF_PARADISE),     // M12 165
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::BOUNTIFUL_HARVEST), // M12 166
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::BRINDLE_BOAR),     // M12 167
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::ELVISH_ARCHDRUID), // M12 172
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FOG),                   // M12 173
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::GARRUK_PRIMAL_HUNTER), // M12 174
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::GARRUK_S_HORDE),   // M12 176
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::GIANT_SPIDER),          // M12 177
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::GLADECOVER_SCOUT), // M12 178
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::LLANOWAR_ELVES),        // M12 182
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::LURE),                  // M12 183
    PrintingRecord::reprint(&crate::card::sets::y2002::onslaught::NATURALIZE),        // M12 185
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::PLUMMET),          // M12 187
    PrintingRecord::reprint(&crate::card::sets::y2010::magic_2011::PRIMEVAL_TITAN),   // M12 188
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::PRIMORDIAL_HYDRA), // M12 189
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::TITANIC_GROWTH),   // M12 198
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::TROLLHIDE),        // M12 199
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::VASTWOOD_GORGER),  // M12 200
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::ELIXIR_OF_IMMORTALITY), // M12 208
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::DRAGONSKULL_SUMMIT), // M12 225
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::DROWNED_CATACOMB),   // M12 226
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::GLACIAL_FORTRESS),   // M12 227
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::ROOTBOUND_CRAG),     // M12 228
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::SUNPETAL_GROVE),     // M12 229
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::PLAINS),                  // M12 230
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),                                 // M12 231
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),                                 // M12 232
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3),                                 // M12 233
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::ISLAND),                  // M12 234
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),                                 // M12 235
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),                                 // M12 236
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3),                                 // M12 237
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SWAMP),                   // M12 238
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),                                  // M12 239
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),                                  // M12 240
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3),                                  // M12 241
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::MOUNTAIN),                // M12 242
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),                               // M12 243
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2),                               // M12 244
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3),                               // M12 245
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FOREST),                  // M12 246
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),                                 // M12 247
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),                                 // M12 248
    PrintingRecord::alternate(&catalog_lea::FOREST, 3),                                 // M12 249
];
