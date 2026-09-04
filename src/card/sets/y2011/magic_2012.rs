//! Magic 2012 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::Binding;
use crate::ComparisonDef;
use crate::CounterKind;
use crate::ObjectQueryDef;
use crate::ObjectSetDef;
use crate::ParentBinding;
use crate::PlayerRefDef;
use crate::TriggerConditionDef;
use crate::TurnStepDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseGroupDef;
use crate::card::MoveObjectsDef;
use crate::card::PartitionGroupDef;
use crate::card::RevealObjectsDef;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CopyAbilityDef, CopyExceptionsDef, EffectDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, PlayerRelation, ReplacementEffectDef, ResolvedEffectDurationDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// M12 1 — Aegis Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AEGIS_ANGEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Aegis Angel",
    "4cdc19da-21af-45e7-ad1f-fcacd84a8d89",
    "Aleksi Briclot",
    crate::card::CardRules::unsupported(),
);

// M12 2 — Alabaster Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Alabaster Mage",
    "f82e6a81-6a45-45f9-829d-332859a32257",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// M12 3 — Angelic Destiny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_DESTINY: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Angelic Destiny",
    "a0cd7438-fde2-4e26-9c34-52c476a971e9",
    "Jana Schirmer & Johannes Voss",
    crate::card::CardRules::unsupported(),
);

// M12 4 — Angel's Mercy (reprint)
const ANGELS_MERCY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ANGELS_MERCY,
    "4e2b0942-423a-4378-b8cd-022a6b608a2e",
    "Andrew Robinson",
);

// M12 5 — Arbalest Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARBALEST_ELITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Arbalest Elite",
    "094c839e-0aea-4754-af37-edf6292623e1",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// M12 6 — Archon of Justice (reprint)
const ARCHON_OF_JUSTICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::eventide::ARCHON_OF_JUSTICE,
    "dcaee06f-edc1-4c3a-9ecc-97882c1b911e",
    "Jason Chan",
);

// M12 7 — Armored Warhorse
pub(in crate::card::sets) static ARMORED_WARHORSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Armored Warhorse",
    "52daf505-d436-4ea6-a157-4268af2ff7a8",
    "rk post",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Horse"], 2, 3),
);

// M12 8 — Assault Griffin (reprint)
const ASSAULT_GRIFFIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::ASSAULT_GRIFFIN,
    "1a791d03-bcbd-437a-8222-3de97d26f0d0",
    "Jesper Ejsing",
);

// M12 9 — Auramancer (reprint)
const AURAMANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::AURAMANCER,
    "b702dc6d-4c05-4313-b303-c321847ad6a9",
    "Rebecca Guay",
);

// M12 10 — Benalish Veteran
pub(in crate::card::sets) static BENALISH_VETERAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Benalish Veteran",
    "09a5603a-88c8-4b0c-b091-6d97e873859a",
    "Steven Belledin",
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

// M12 11 — Celestial Purge (reprint)
const CELESTIAL_PURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::conflux::CELESTIAL_PURGE,
    "75f75e85-9454-4008-aa51-a1d5965752d6",
    "David Palumbo",
);

// M12 12 — Day of Judgment (reprint)
const DAY_OF_JUDGMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::DAY_OF_JUDGMENT,
    "1ed43ed8-9490-4433-843f-9020cd3470a1",
    "Vincent Proce",
);

// M12 13 — Demystify (reprint)
const DEMYSTIFY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::onslaught::DEMYSTIFY,
    "8f1b042f-f059-4e9f-a459-8682688f45cf",
    "Véronique Meignaud",
);

// M12 14 — Divine Favor
pub(in crate::card::sets) static DIVINE_FAVOR: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Divine Favor",
    "1f44e053-95c2-410f-b35d-8ea3e3607e82",
    "Allen Williams",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, you gain 3 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(3),
                    ),
                },
            ),
        ]),
);

// M12 15 — Elite Vanguard (reprint)
const ELITE_VANGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ELITE_VANGUARD,
    "f03487e9-f584-4bbd-8335-4dd001a88b52",
    "Mark Tedin",
);

// M12 16 — Gideon Jura (reprint)
const GIDEON_JURA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::GIDEON_JURA,
    "1c58b63c-e3e5-4575-849c-9a6a00821286",
    "Aleksi Briclot",
);

// M12 17 — Gideon's Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIDEON_S_AVENGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Gideon's Avenger",
    "bb0a0d33-8862-433b-a078-82472e5f9af0",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// M12 18 — Gideon's Lawkeeper
pub(in crate::card::sets) static GIDEON_S_LAWKEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Gideon's Lawkeeper",
    "1c71eb81-a077-4c85-a4ce-4ad664486bee",
    "Steve Prescott",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAND_ABOLISHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Grand Abolisher",
    "67e35a40-37dd-436c-b4ac-b17b04508c1f",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// M12 20 — Griffin Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIFFIN_RIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Griffin Rider",
    "3f1a5517-e442-4fbc-b8c3-fea28e5e44d2",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// M12 21 — Griffin Sentinel (reprint)
const GRIFFIN_SENTINEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::GRIFFIN_SENTINEL,
    "3c8f2fea-2bc1-49fc-91c9-83698f43262f",
    "Warren Mahy",
);

// M12 22 — Guardians' Pledge
pub(in crate::card::sets) static GUARDIANS_PLEDGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Guardians' Pledge",
    "e7e6105c-8633-46f7-a7ca-2a5c36c6d548",
    "Christopher Moeller",
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

// M12 23 — Honor of the Pure (reprint)
const HONOR_OF_THE_PURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::HONOR_OF_THE_PURE,
    "650a6831-c352-4ca7-9f8f-43ea99a1cf33",
    "Greg Staples",
);

// M12 24 — Lifelink (reprint)
const LIFELINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::LIFELINK,
    "a8e207d4-9930-4aff-a7c8-b53bd1b5d566",
    "Terese Nielsen",
);

// M12 25 — Mesa Enchantress (reprint)
const MESA_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::planar_chaos::MESA_ENCHANTRESS,
    "691dcce5-ac3d-4970-b3ff-3db485f9f5c3",
    "Randy Gallegos",
);

// M12 26 — Mighty Leap (reprint)
const MIGHTY_LEAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::MIGHTY_LEAP,
    "446e1676-ae7d-46ee-af91-bb54e4d18a78",
    "rk post",
);

// M12 27 — Oblivion Ring (reprint)
const OBLIVION_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::lorwyn::OBLIVION_RING,
    "9efc5a2a-eb76-410d-98e6-1455108faa52",
    "Franz Vohwinkel",
);

// M12 28 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "783ccfbb-4063-4614-8135-11787227ce97",
    "Robert Bliss",
);

// M12 29 — Peregrine Griffin
pub(in crate::card::sets) static PEREGRINE_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Peregrine Griffin",
    "0296eaa6-f9fe-4fb8-af9c-04928d99e2e2",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Griffin"], 2, 4)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// M12 30 — Personal Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERSONAL_SANCTUARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Personal Sanctuary",
    "56f10d57-687d-4ee3-8226-bae525d56e9e",
    "Howard Lyon",
    crate::card::CardRules::unsupported(),
);

// M12 31 — Pride Guardian
pub(in crate::card::sets) static PRIDE_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Pride Guardian",
    "c8d8d723-743c-45d6-b11b-7213f4872cf1",
    "Chris Rahn",
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

// M12 32 — Roc Egg (reprint)
const ROC_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::ROC_EGG,
    "92ae6206-ff0d-4248-b9cb-4ffbf20504fa",
    "Paul Bonner",
);

// M12 33 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::SERRA_ANGEL,
    "3c31fb9d-ec0d-4555-814d-62642d52c710",
    "Greg Staples",
);

// M12 34 — Siege Mastodon (reprint)
const SIEGE_MASTODON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SIEGE_MASTODON,
    "39c340a3-0118-48d3-99ab-f4a0e7099325",
    "Matt Cavotta",
);

// M12 35 — Spirit Mantle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_MANTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Spirit Mantle",
    "930c8444-ccce-411e-bc4f-e5abca749608",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// M12 36 — Stave Off
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAVE_OFF: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Stave Off",
    "3bb09157-5d7a-4da2-92b6-9354489e607f",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// M12 37 — Stonehorn Dignitary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONEHORN_DIGNITARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Stonehorn Dignitary",
    "c3797f7f-489d-4735-af56-6359e0fa0a6b",
    "Dave Kendall",
    crate::card::CardRules::unsupported(),
);

// M12 38 — Stormfront Pegasus (reprint)
const STORMFRONT_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::STORMFRONT_PEGASUS,
    "bf0ba2d2-09d5-4755-a18f-40cf19d88f25",
    "rk post",
);

// M12 39 — Sun Titan (reprint)
const SUN_TITAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::SUN_TITAN,
    "ea3e77ed-9015-4407-b78c-494e46b67b07",
    "Todd Lockwood",
);

// M12 40 — Timely Reinforcements
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMELY_REINFORCEMENTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Timely Reinforcements",
    "6ae4669c-e526-4c24-9c25-38cb5c5ef59b",
    "Tomasz Jedruszek",
    crate::card::CardRules::unsupported(),
);

// M12 41 — Aether Adept (reprint)
const AETHER_ADEPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::AETHER_ADEPT,
    "fa6f04ca-cab7-4c86-a56c-79d6ae3b73e6",
    "Eric Deschamps",
);

// M12 42 — Alluring Siren (reprint)
const ALLURING_SIREN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ALLURING_SIREN,
    "a6434841-6cca-4397-b1fa-5ce34dc0b7f3",
    "Chippy",
);

// M12 43 — Amphin Cutthroat
pub(in crate::card::sets) static AMPHIN_CUTTHROAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Amphin Cutthroat",
    "fd169064-9c7b-40bd-8be0-a89fcb28ae2f",
    "Howard Lyon",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Salamander", "Rogue"], 2, 4),
);

// M12 44 — Aven Fleetwing
pub(in crate::card::sets) static AVEN_FLEETWING: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Aven Fleetwing",
    "57626fd2-d101-4e23-946f-8309c9676fe5",
    "Wayne Reynolds",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird", "Soldier"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::hexproof()]),
);

// M12 45 — Azure Mage
pub(in crate::card::sets) static AZURE_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Azure Mage",
    "a473897f-49eb-4e0f-a5b6-ea75e10be91a",
    "Izzy",
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

// M12 46 — Belltower Sphinx (reprint)
const BELLTOWER_SPHINX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::BELLTOWER_SPHINX,
    "d6829959-dae1-4ddf-8a75-33a77e6b4612",
    "Jim Nelson",
);

// M12 47 — Cancel (reprint)
const CANCEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::time_spiral::CANCEL,
    "c464b856-e3c0-4b06-a2b0-6663a9aafd26",
    "David Palumbo",
);

// M12 48 — Chasm Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHASM_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Chasm Drake",
    "5e7e246d-92f8-4e6e-89fc-991b888fc1e8",
    "Anthony Francisco",
    crate::card::CardRules::unsupported(),
);

// M12 49 — Coral Merfolk (reprint)
const CORAL_MERFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::CORAL_MERFOLK,
    "c9e8f212-d1c6-4140-a392-73d0e141e708",
    "rk post",
);

// M12 50 — Divination (reprint)
const DIVINATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DIVINATION,
    "1252243c-34e3-447b-b323-fffcbe128278",
    "Howard Lyon",
);

// M12 51 — Djinn of Wishes (reprint)
const DJINN_OF_WISHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DJINN_OF_WISHES,
    "74c621dd-9c60-4951-beaf-eb6b597c2f0f",
    "Kev Walker",
);

// M12 52 — Flashfreeze (reprint)
const FLASHFREEZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::coldsnap::FLASHFREEZE,
    "c425a629-371f-4624-b7a1-b34818ecccad",
    "Brian Despain",
);

// M12 53 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::FLIGHT,
    "15316953-dcb2-4428-b90a-c90d3d4c45f3",
    "Mark Zug",
);

// M12 54 — Frost Breath
pub(in crate::card::sets) static FROST_BREATH: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Frost Breath",
    "1724ec5b-5437-4688-aa10-b327a0ae2654",
    "Mike Bierek",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap up to two target creatures. Those creatures don't untap during their controller's \
         next untap step.",
        // "Up to two", so nothing at all is a legal declaration, and the skip is
        // counted on each creature separately -- the two may belong to different
        // players, who do not reach their untap steps together.
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            2,
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                count: 1,
            },
        ]),
    )),
);

// M12 55 — Frost Titan (reprint)
const FROST_TITAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::FROST_TITAN,
    "358baa9f-390f-4b99-a274-d28f3bd56824",
    "Mike Bierek",
);

// M12 56 — Harbor Serpent (reprint)
const HARBOR_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::HARBOR_SERPENT,
    "453d4f1d-b378-49c3-b697-6566b3c455cd",
    "Daarken",
);

// M12 57 — Ice Cage (reprint)
const ICE_CAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ICE_CAGE,
    "a5e14b62-c050-4d43-aeee-873f46d1e295",
    "Mike Bierek",
);

// M12 58 — Jace, Memory Adept
pub(in crate::card::sets) static JACE_MEMORY_ADEPT: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Jace, Memory Adept",
    "f3f2a5b6-c26c-4355-b760-d87f074a4921",
    "D. Alexander Gregory",
    CardRules::new_planeswalker(mana_cost!("{3}{U}{U}"), &["Jace"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Draw a card. Target player mills a card.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Mill {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "0: Target player mills ten cards.",
                &[AbilityCostDef::Loyalty(0)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(10),
                },
            ),
            AbilityDef::activated_with_targets(
                "−7: Any number of target players each draw twenty cards.",
                &[AbilityCostDef::Loyalty(-7)],
                // Two players means "any number" is up to two.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                    2,
                )],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(20),
                },
            ),
        ]),
);

// M12 59 — Jace's Archivist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JACE_S_ARCHIVIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Jace's Archivist",
    "47c6b294-3840-4007-a4e3-67309f6581dd",
    "James Ryman",
    crate::card::CardRules::unsupported(),
);

// M12 60 — Jace's Erasure (reprint)
const JACE_S_ERASURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::JACE_S_ERASURE,
    "970f4f34-f834-41a7-aff1-7cef82cefc74",
    "Jason Chan",
);

// M12 61 — Levitation (reprint)
const LEVITATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_legacy::LEVITATION,
    "63e5124a-67c0-44ed-8085-28bf37816423",
    "Jim Murray",
);

// M12 62 — Lord of the Unreal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_OF_THE_UNREAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Lord of the Unreal",
    "b09140f6-fa75-4bee-9ca0-3a71cd2b5a7b",
    "Jason Chan",
    crate::card::CardRules::unsupported(),
);

// M12 63 — Mana Leak (reprint)
const MANA_LEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::stronghold::MANA_LEAK,
    "6b123efa-8631-4a07-970d-ff4f980a0522",
    "Howard Lyon",
);

// M12 64 — Master Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_THIEF: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Master Thief",
    "77c273d3-ef0f-40c6-baf5-e39279d10509",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// M12 65 — Merfolk Looter (reprint)
const MERFOLK_LOOTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::exodus::MERFOLK_LOOTER,
    "aad3aaec-7c88-4925-8023-0cf61bf906c2",
    "Austin Hsu",
);

// M12 66 — Merfolk Mesmerist
pub(in crate::card::sets) static MERFOLK_MESMERIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Merfolk Mesmerist",
    "220dede5-472c-4a09-bdf0-73e722d9d4d2",
    "Jana Schirmer & Johannes Voss",
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
            },
        ),
    ),
);

// M12 67 — Mind Control (reprint)
const MIND_CONTROL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::MIND_CONTROL,
    "ec7f77af-17d7-4746-bc83-f455b9b6f9ea",
    "Ryan Pancoast",
);

// M12 68 — Mind Unbound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_UNBOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Mind Unbound",
    "fd90cf36-9841-4adf-b5cb-0a7bf103eb93",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

// M12 69 — Negate (reprint)
const NEGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::morningtide::NEGATE,
    "7f21cd05-2fdb-4c37-90a4-220a3eda23ef",
    "Jeremy Jarvis",
);

// M12 70 — Phantasmal Bear
pub(in crate::card::sets) static PHANTASMAL_BEAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Phantasmal Bear",
    "06cc574a-f687-4e41-b0a0-62a0eedea7c2",
    "Ryan Yee",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Phantasmal Dragon",
    "a2cd015c-0569-4e7f-9daf-b39e67fc7096",
    "Wayne Reynolds",
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

pub(in crate::card::sets) static PHANTASMAL_IMAGE: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Phantasmal Image",
    "98e7bf8f-dba7-4005-8cee-634c9153931d",
    "Nils Hamm",
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
const PONDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::lorwyn::PONDER,
    "81c908ee-e70a-4406-a32d-ab5ab17e67b1",
    "Dan Murayama Scott",
);

// M12 74 — Redirect (reprint)
const REDIRECT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::REDIRECT,
    "efdb01f8-209d-4f8b-a280-b45e1ab0b880",
    "Izzy",
);

// M12 75 — Skywinder Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYWINDER_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Skywinder Drake",
    "628213e9-bde9-43fd-a0d9-8c7fb17be879",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// M12 76 — Sphinx of Uthuun
const SPHINX_FIRST: Binding = Binding!("sphinx_first");
const SPHINX_SECOND: Binding = Binding!("sphinx_second");
const SPHINX_CHOSEN: Binding = Binding!("sphinx_chosen");
const SPHINX_UNCHOSEN: Binding = Binding!("sphinx_unchosen");

pub(in crate::card::sets) static SPHINX_OF_UTHUUN: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Sphinx of Uthuun",
    "a290648a-63c3-400b-98d3-5a5aa5505027",
    "Kekai Kotaki",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Sphinx"], 5, 6).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard.",
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::Constant(5),
                &const { EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::PartitionGroup(PartitionGroupDef {
                        actor: PlayerRefDef::Opponent,
                        input: ObjectSetDef::Binding(ParentBinding),
                        first: SPHINX_FIRST,
                        second: SPHINX_SECOND,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &const { EffectDef::ChooseGroup(ChooseGroupDef {
                            actor: PlayerRefDef::EffectController,
                            first: ObjectSetDef::Binding(SPHINX_FIRST),
                            second: ObjectSetDef::Binding(SPHINX_SECOND),
                            chosen: SPHINX_CHOSEN,
                            unchosen: SPHINX_UNCHOSEN,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &const { EffectDef::Sequence(&[
                                EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(SPHINX_CHOSEN),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Hand,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }),
                                EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(SPHINX_UNCHOSEN),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Graveyard,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }),
                            ]) },
                        }) },
                    }),
                ]) },
            ),
        ),
    ]),
);

// M12 77 — Time Reversal (reprint)
const TIME_REVERSAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::TIME_REVERSAL,
    "2d6500a1-5aea-4b83-b4dc-560fe547590d",
    "Howard Lyon",
);

// M12 78 — Turn to Frog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURN_TO_FROG: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Turn to Frog",
    "b43d9a1e-0767-4a9b-81b4-4ff2f3dde1d5",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// M12 79 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::UNSUMMON,
    "88439b79-f6f4-4d01-8404-a1e02f2aeb55",
    "Izzy",
);

// M12 80 — Visions of Beyond
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISIONS_OF_BEYOND: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Visions of Beyond",
    "75657d26-b0f8-4892-8684-533c103c921d",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// M12 81 — Blood Seeker (reprint)
const BLOOD_SEEKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::BLOOD_SEEKER,
    "8033de8d-a396-4097-aedd-f9facb800b33",
    "Greg Staples",
);

// M12 82 — Bloodlord of Vaasgoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODLORD_OF_VAASGOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Bloodlord of Vaasgoth",
    "125c5cff-d4e9-4655-9cc5-3ce21e577569",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// M12 83 — Bloodrage Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODRAGE_VAMPIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Bloodrage Vampire",
    "a078e438-fcf9-4648-95dc-3d4037f9b561",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// M12 84 — Brink of Disaster (reprint)
const BRINK_OF_DISASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::worldwake::BRINK_OF_DISASTER,
    "dbab78cd-a899-4c5d-86b3-0666adadba87",
    "Alex Horley-Orlandelli",
);

// M12 85 — Call to the Grave (reprint)
const CALL_TO_THE_GRAVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::scourge::CALL_TO_THE_GRAVE,
    "5e1324b6-dba0-4aff-a403-a45d2b405f5b",
    "Daarken",
);

// M12 86 — Cemetery Reaper (reprint)
const CEMETERY_REAPER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::CEMETERY_REAPER,
    "56494d1e-0d7e-4c29-942c-b376ff07cdf8",
    "Dave Allsop",
);

// M12 87 — Child of Night (reprint)
const CHILD_OF_NIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::CHILD_OF_NIGHT,
    "c4780079-7f4c-4a43-883f-4722423c4fec",
    "Ash Wood",
);

// M12 88 — Consume Spirit (reprint)
const CONSUME_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::CONSUME_SPIRIT,
    "ef144439-fc8e-4844-8ebb-3e36e05ac9a0",
    "Justin Sweet",
);

// M12 89 — Dark Favor
pub(in crate::card::sets) static DARK_FAVOR: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Dark Favor",
    "a258a235-086e-429b-9ac1-3178f902658b",
    "Allen Williams",
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, you lose 1 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// M12 90 — Deathmark (reprint)
const DEATHMARK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::coldsnap::DEATHMARK,
    "b101ff4a-8617-4c0a-8503-ed8c857ad000",
    "Steven Belledin",
);

// M12 91 — Devouring Swarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOURING_SWARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Devouring Swarm",
    "735c2c79-9b4f-4f86-9dec-0749237fe9ce",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// M12 92 — Diabolic Tutor (reprint)
const DIABOLIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::DIABOLIC_TUTOR,
    "2db75949-f8cd-4461-83c0-7eaee2196132",
    "Greg Staples",
);

// M12 93 — Disentomb (reprint)
const DISENTOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DISENTOMB,
    "ca10a691-a209-44a3-9925-8638a3c4e1d1",
    "Alex Horley-Orlandelli",
);

// M12 94 — Distress (reprint)
const DISTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::champions_of_kamigawa::DISTRESS,
    "630d4080-8183-41fb-8091-740719083765",
    "Michael C. Hayes",
);

// M12 95 — Doom Blade (reprint)
const DOOM_BLADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DOOM_BLADE,
    "077d5ca8-2a94-4d79-9314-c5ca2aa4d14b",
    "Chippy",
);

// M12 96 — Drifting Shade
pub(in crate::card::sets) static DRIFTING_SHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Drifting Shade",
    "00dcb25e-764b-47d6-bec4-225aaace77b0",
    "Tomasz Jedruszek",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKHUNTER_BAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Duskhunter Bat",
    "4560ee1a-1076-4ec5-a177-55ffe12e2165",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// M12 98 — Grave Titan (reprint)
const GRAVE_TITAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::GRAVE_TITAN,
    "5c70da33-ce5d-4b8b-9c1d-9a356a7e196f",
    "Nils Hamm",
);

// M12 99 — Gravedigger (reprint)
const GRAVEDIGGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::GRAVEDIGGER,
    "11055d4e-3efe-493c-8c18-9e2642267511",
    "Dermot Power",
);

// M12 100 — Hideous Visage
pub(in crate::card::sets) static HIDEOUS_VISAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Hideous Visage",
    "25925751-b6cb-45a3-915f-d5ec3edcda78",
    "Nils Hamm",
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
const MIND_ROT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::MIND_ROT,
    "e61ee72e-61ae-4558-8abe-f5eaf5b9fb8a",
    "Steve Luke",
);

// M12 102 — Monomania
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONOMANIA: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Monomania",
    "6af53d7f-7f02-4c35-b6f4-7365d121ba54",
    "James Ryman",
    crate::card::CardRules::unsupported(),
);

// M12 103 — Onyx Mage
pub(in crate::card::sets) static ONYX_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Onyx Mage",
    "eabd38e6-1e59-42d2-bd1a-555c77cf6747",
    "Izzy",
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

// M12 104 — Reassembling Skeleton (reprint)
const REASSEMBLING_SKELETON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::archenemy::REASSEMBLING_SKELETON,
    "75c219bc-a140-4ecd-953a-eef2cc552d58",
    "Austin Hsu",
);

// M12 105 — Royal Assassin (reprint)
const ROYAL_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::ROYAL_ASSASSIN,
    "d12e8109-8215-46b5-a0af-fe7e4b6b10b0",
    "Mark Zug",
);

// M12 106 — Rune-Scarred Demon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_SCARRED_DEMON: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Rune-Scarred Demon",
    "509e0f81-0591-4b28-978e-a2f1c46b7427",
    "Michael Komarck",
    crate::card::CardRules::unsupported(),
);

// M12 107 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::SENGIR_VAMPIRE,
    "e0da3971-0145-4975-8bb1-8c2898d10ae7",
    "Kev Walker",
);

// M12 108 — Smallpox (reprint)
const SMALLPOX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::time_spiral::SMALLPOX,
    "93c8159b-8c1d-480a-b517-dbd67bba1838",
    "Ryan Pancoast",
);

// M12 109 — Sorin Markov (reprint)
const SORIN_MARKOV_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::SORIN_MARKOV,
    "e25b3a89-3a99-4e02-bf0c-a3cf450da1a1",
    "Michael Komarck",
);

// M12 110 — Sorin's Thirst
pub(in crate::card::sets) static SORIN_S_THIRST: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Sorin's Thirst",
    "1f14a435-811d-4057-93a9-ce74aa852a09",
    "Karl Kopinski",
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
    crate::card::CardSet::Magic2012,
    "Sorin's Vengeance",
    "2cb62846-c5da-4c7c-b0d7-9b677dce68d1",
    "Jana Schirmer & Johannes Voss",
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
const SUTURED_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::judgment::SUTURED_GHOUL,
    "8390d9b7-5adf-4039-8682-02bfba421ff9",
    "Carl Critchlow",
);

// M12 113 — Taste of Blood
pub(in crate::card::sets) static TASTE_OF_BLOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Taste of Blood",
    "29268cef-da18-4c1d-9066-e0d513a61bf9",
    "Howard Lyon",
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

// M12 114 — Tormented Soul
pub(in crate::card::sets) static TORMENTED_SOUL: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Tormented Soul",
    "d2699c42-99bb-4b5a-82ec-9c6424c14ec1",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{B}"), &["Spirit"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't block and can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // The two halves point opposite ways: one keeps it out of blocks it would
                // join, the other out of blocks it would be caught by.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                ]),
            },
        ),
    ),
);

// M12 115 — Vampire Outcasts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_OUTCASTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Vampire Outcasts",
    "1286132d-1697-44da-ab97-387735265c01",
    "Clint Cearley",
    crate::card::CardRules::unsupported(),
);

// M12 116 — Vengeful Pharaoh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEFUL_PHARAOH: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Vengeful Pharaoh",
    "12e0ca97-bc57-4084-86b4-e2e06152cb1c",
    "Igor Kieryluk",
    crate::card::CardRules::unsupported(),
);

// M12 117 — Warpath Ghoul (reprint)
const WARPATH_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::WARPATH_GHOUL,
    "94785274-fa79-47cc-9896-0f5f695abb21",
    "rk post",
);

// M12 118 — Wring Flesh
pub(in crate::card::sets) static WRING_FLESH: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Wring Flesh",
    "663df3e8-12e5-46cf-9da7-39961feaa7f9",
    "Izzy",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -3/-1 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-3),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M12 119 — Zombie Goliath (reprint)
const ZOMBIE_GOLIATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ZOMBIE_GOLIATH,
    "1985e0bd-05b9-4eaf-9333-6262cf677acd",
    "E. M. Gist",
);

// M12 120 — Zombie Infestation (reprint)
const ZOMBIE_INFESTATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::ZOMBIE_INFESTATION,
    "c84a3e27-841a-4eb5-afcd-ddb87d4280f7",
    "Thomas M. Baxa",
);

// M12 121 — Act of Treason (reprint)
const ACT_OF_TREASON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ACT_OF_TREASON,
    "0ea8984c-7176-43e5-931b-c3b6f4747a8b",
    "Eric Deschamps",
);

// M12 122 — Blood Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_OGRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Blood Ogre",
    "b85ecba6-fc22-48c7-9f00-066cc1fce6b5",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// M12 123 — Bonebreaker Giant
pub(in crate::card::sets) static BONEBREAKER_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Bonebreaker Giant",
    "cc17e5c1-a6b4-401b-95eb-1c01cd1da570",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Giant"], 4, 4),
);

// M12 124 — Chandra, the Firebrand
// Audit: unsupported — InstalledTriggerDef cannot combine “once” with expiration at end of turn for the next-spell copy permission.
pub(in crate::card::sets) static CHANDRA_THE_FIREBRAND: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Chandra, the Firebrand",
    "efb37556-186f-4660-8b75-c52ef16a6d8f",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// M12 125 — Chandra's Outrage (reprint)
const CHANDRAS_OUTRAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::archenemy::CHANDRAS_OUTRAGE,
    "1b3850f0-13ed-40c5-8423-8b196132a97a",
    "Christopher Moeller",
);

// M12 126 — Chandra's Phoenix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHANDRA_S_PHOENIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Chandra's Phoenix",
    "e8371c83-d7c5-4432-8511-cb3c1dc7d59f",
    "Aleksi Briclot",
    crate::card::CardRules::unsupported(),
);

// M12 127 — Circle of Flame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_FLAME: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Circle of Flame",
    "419b1813-9760-47b9-b6f3-e501586cfe4d",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// M12 128 — Combust (reprint)
const COMBUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::COMBUST,
    "f10346e2-46bd-4257-b191-c36c2577c534",
    "Jaime Jones",
);

// M12 129 — Crimson Mage
pub(in crate::card::sets) static CRIMSON_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Crimson Mage",
    "0f69ccfc-e2a9-40af-b8ab-85bffe62c0f4",
    "Izzy",
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

// M12 130 — Fiery Hellhound (reprint)
const FIERY_HELLHOUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::FIERY_HELLHOUND,
    "7c96f7a0-99a3-4ba4-b0f0-9ea36c45d5d5",
    "Ted Galaday",
);

// M12 131 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::FIREBALL,
    "f6a86f5d-cfbf-4d8e-9f8e-0f8288907396",
    "Dave Dorman",
);

// M12 132 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::FIREBREATHING,
    "6fbcc269-aaa3-42aa-9898-3f908aaae272",
    "Aleksi Briclot",
);

// M12 133 — Flameblast Dragon (reprint)
const FLAMEBLAST_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shards_of_alara::FLAMEBLAST_DRAGON,
    "c01ab5c8-f9b7-482c-a900-1388b727b89f",
    "Jaime Jones",
);

// M12 134 — Fling (reprint)
const FLING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::stronghold::FLING,
    "3a439015-0c1c-4322-a6f1-a34040162ac4",
    "Paolo Parente",
);

// M12 135 — Furyborn Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FURYBORN_HELLKITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Furyborn Hellkite",
    "b5b735e5-da9d-4740-acff-aac9dd24334c",
    "Brad Rigney",
    crate::card::CardRules::unsupported(),
);

// M12 136 — Goblin Arsonist (reprint)
const GOBLIN_ARSONIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::GOBLIN_ARSONIST,
    "c24751fd-5e9b-4d7d-83ba-e306b439bbe1",
    "Wayne Reynolds",
);

// M12 137 — Goblin Bangchuckers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BANGCHUCKERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Goblin Bangchuckers",
    "b56ddad0-23ea-4139-a200-c76c9c46e8c5",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// M12 138 — Goblin Chieftain (reprint)
const GOBLIN_CHIEFTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::GOBLIN_CHIEFTAIN,
    "2540ec6b-9ffa-4ab0-bbd3-ddf1efd2db60",
    "Sam Wood",
);

// M12 139 — Goblin Fireslinger
pub(in crate::card::sets) static GOBLIN_FIRESLINGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Goblin Fireslinger",
    "3c11db78-f506-4af2-a7be-c7ac2c0ffcf3",
    "Pete Venters",
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
const GOBLIN_GRENADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1994::fallen_empires::GOBLIN_GRENADE,
    "394cc2aa-0318-4ccd-a550-99a7eac933c3",
    "Kev Walker",
);

// M12 141 — Goblin Piker (reprint)
const GOBLIN_PIKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::portal_second_age::GOBLIN_PIKER,
    "083ec3e7-950c-4e9d-aba5-02ed13d723f0",
    "DiTerlizzi",
);

// M12 142 — Goblin Tunneler (reprint)
const GOBLIN_TUNNELER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::GOBLIN_TUNNELER,
    "c466bbb3-9758-47e6-8996-3615f4c31924",
    "Jesper Ejsing",
);

// M12 143 — Goblin War Paint (reprint)
const GOBLIN_WAR_PAINT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::GOBLIN_WAR_PAINT,
    "fde711c9-fdef-4024-8269-a59ee0748f95",
    "Austin Hsu",
);

// M12 144 — Gorehorn Minotaurs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOREHORN_MINOTAURS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Gorehorn Minotaurs",
    "1087e015-a3c4-4207-8285-5bda6bb50e52",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// M12 145 — Grim Lavamancer (reprint)
const GRIM_LAVAMANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::torment::GRIM_LAVAMANCER,
    "d9fbe2c9-31d5-4e54-922d-1bc6a865b251",
    "Michael Sutfin",
);

// M12 146 — Incinerate (reprint)
const INCINERATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1995::ice_age::INCINERATE,
    "1af92296-455a-425e-9397-a96d09937767",
    "Zoltan Boros & Gabor Szikszai",
);

// M12 147 — Inferno Titan (reprint)
const INFERNO_TITAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::INFERNO_TITAN,
    "e04c24cb-3c3b-4a35-9694-db512bf394fa",
    "Kev Walker",
);

// M12 148 — Lava Axe (reprint)
const LAVA_AXE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::LAVA_AXE,
    "2cf9c3aa-9434-4a62-9bcb-0699a85de9cb",
    "Brian Snõddy",
);

// M12 149 — Lightning Elemental (reprint)
const LIGHTNING_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::tempest::LIGHTNING_ELEMENTAL,
    "e106b6af-a13c-42be-9368-9109795de517",
    "Kev Walker",
);

// M12 150 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::MANABARBS,
    "adf081d5-e644-4f46-8bc8-a754b089acb4",
    "Jeff Miracola",
);

// M12 151 — Manic Vandal (reprint)
const MANIC_VANDAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::MANIC_VANDAL,
    "985a5866-8c62-46af-a0c0-e69d01d87f4f",
    "Christopher Moeller",
);

// M12 152 — Reverberate (reprint)
const REVERBERATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::REVERBERATE,
    "7062e2f3-bf4a-4a9a-962c-b45718a464bc",
    "jD",
);

// M12 153 — Scrambleverse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAMBLEVERSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Scrambleverse",
    "2b61fa9d-3f69-4632-be0e-09924ca88501",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// M12 154 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::stronghold::SHOCK,
    "b8fed52c-e84e-41c9-b683-d8b26fa03c5f",
    "Jon Foster",
);

// M12 155 — Slaughter Cry (reprint)
const SLAUGHTER_CRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::SLAUGHTER_CRY,
    "65ec8b61-e602-41f2-ac1a-64e150b2ce18",
    "Matt Cavotta",
);

// M12 156 — Stormblood Berserker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMBLOOD_BERSERKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Stormblood Berserker",
    "fc9a50af-ca3e-461a-9dcb-444f56284165",
    "Min Yum",
    crate::card::CardRules::unsupported(),
);

// M12 157 — Tectonic Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TECTONIC_RIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Tectonic Rift",
    "e9838784-8c6d-4e64-bc34-e21efde99093",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// M12 158 — Volcanic Dragon (reprint)
const VOLCANIC_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::VOLCANIC_DRAGON,
    "56134669-9575-44bc-9203-edbd75acecbd",
    "Chris Rahn",
);

// M12 159 — Wall of Torches
pub(in crate::card::sets) static WALL_OF_TORCHES: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Wall of Torches",
    "76f69b92-7435-4aa8-9d90-89ea078befb1",
    "Mike Bierek",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Wall"], 4, 1)
        .with_abilities(&[abilities::defender()]),
);

// M12 160 — Warstorm Surge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARSTORM_SURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Warstorm Surge",
    "b16443df-52c6-4c9d-a7ff-89a37e593a0a",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// M12 161 — Acidic Slime (reprint)
const ACIDIC_SLIME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ACIDIC_SLIME,
    "ae8e5876-3eff-4075-9fa2-3ab6030848e9",
    "Karl Kopinski",
);

// M12 162 — Arachnus Spinner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARACHNUS_SPINNER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Arachnus Spinner",
    "23321b80-d7e7-48fd-985d-1e9dc3adcd35",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// M12 163 — Arachnus Web
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARACHNUS_WEB: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Arachnus Web",
    "9e4900b0-f934-42d9-92fb-0bb16d2e8bb1",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// M12 164 — Autumn's Veil (reprint)
const AUTUMN_S_VEIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::AUTUMN_S_VEIL,
    "b911fee0-c30b-4d68-a9e2-61c40ece68b0",
    "Kekai Kotaki",
);

// M12 165 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::BIRDS_OF_PARADISE,
    "307d4236-1e54-43e3-83f1-063d49d16dda",
    "Marcelo Vignali",
);

// M12 166 — Bountiful Harvest (reprint)
const BOUNTIFUL_HARVEST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::BOUNTIFUL_HARVEST,
    "8191e3cb-ef28-40ea-9eab-23455435d49e",
    "Jason Chan",
);

// M12 167 — Brindle Boar (reprint)
const BRINDLE_BOAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::BRINDLE_BOAR,
    "fc780658-fc10-481e-8319-d44a644e8fe8",
    "Dave Allsop",
);

// M12 168 — Carnage Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARNAGE_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Carnage Wurm",
    "c086eb41-3524-4815-97c9-761ba86a30b2",
    "Dave Kendall",
    crate::card::CardRules::unsupported(),
);

// M12 169 — Cudgel Troll (reprint)
const CUDGEL_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::CUDGEL_TROLL,
    "e156b8d8-5309-494e-9709-44f98826a69f",
    "Jesper Ejsing",
);

// M12 170 — Doubling Chant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOUBLING_CHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Doubling Chant",
    "f71dc232-9b7e-4c0e-ac05-8e48b4936aa9",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// M12 171 — Dungrove Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUNGROVE_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Dungrove Elder",
    "b8b4ebbf-1613-42a0-97ff-2f36dc8d984a",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// M12 172 — Elvish Archdruid (reprint)
const ELVISH_ARCHDRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ELVISH_ARCHDRUID,
    "99f37891-fac7-4868-a20b-0e879f7e0859",
    "Karl Kopinski",
);

// M12 173 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::FOG,
    "84387fb5-7929-4d08-9d94-ba2d94460ef3",
    "Jaime Jones",
);

// M12 174 — Garruk, Primal Hunter
pub(in crate::card::sets) static GARRUK_PRIMAL_HUNTER: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Garruk, Primal Hunter",
    "82d586bf-bed3-4390-b81d-d101c2ae524c",
    "D. Alexander Gregory",
    CardRules::new_planeswalker(mana_cost!("{2}{G}{G}{G}"), &["Garruk"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Create a 3/3 green Beast creature token.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::create_creature_token(&["Beast"], &[ManaColor::Green], 3, 3).with_art(
                    CardArt::new("c94010f1-cd4b-4f65-8a0e-2df6eec058ec", "John Donahue"),
                ),
            ),
            AbilityDef::activated(
                "−3: Draw cards equal to the greatest power among creatures you control.",
                &[AbilityCostDef::Loyalty(-3)],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: abilities::greatest_power_you_control(),
                },
            ),
            AbilityDef::activated(
                "−6: Create a 6/6 green Wurm creature token for each land you control.",
                &[AbilityCostDef::Loyalty(-6)],
                EffectDef::create_creature_token(&["Wurm"], &[ManaColor::Green], 6, 6)
                    .with_art(CardArt::new(
                        "a4d87f38-c342-4186-8768-c3f1aceb680a",
                        "Anthony Francisco",
                    ))
                    .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ))),
            ),
        ]),
);

// M12 175 — Garruk's Companion (reprint)
const GARRUK_S_COMPANION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::GARRUK_S_COMPANION,
    "b8d8806c-43c5-4c6c-9420-6210a17ec2b0",
    "Efrem Palacios",
);

// M12 176 — Garruk's Horde
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GARRUK_S_HORDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Garruk's Horde",
    "563c6959-9131-40a6-97ec-12baf6fb7ca0",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// M12 177 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::GIANT_SPIDER,
    "460133e5-00f1-47e2-91fe-c36802ef16a8",
    "Randy Gallegos",
);

// M12 178 — Gladecover Scout
pub(in crate::card::sets) static GLADECOVER_SCOUT: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Gladecover Scout",
    "26710d5c-01d1-498b-9f54-521dfd195843",
    "Allen Williams",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1)
        .with_abilities(&[abilities::hexproof()]),
);

// M12 179 — Greater Basilisk (reprint)
const GREATER_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::GREATER_BASILISK,
    "994711cb-e85b-4acb-9460-17231e1d66ad",
    "James Ryman",
);

// M12 180 — Hunter's Insight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTER_S_INSIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Hunter's Insight",
    "e4044a9f-43bd-4c32-9d53-29a27ad9be80",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// M12 181 — Jade Mage
pub(in crate::card::sets) static JADE_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Jade Mage",
    "32d6c8d3-04a1-4b35-b7d1-18bed82beaf4",
    "Izzy",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Shaman"], 2, 1).with_ability(
        AbilityDef::activated(
            "{2}{G}: Create a 1/1 green Saproling creature token.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}"))],
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
    ),
);

// M12 182 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::LLANOWAR_ELVES,
    "01c6f877-6b00-4d57-8a88-36cd3b16edbc",
    "Kev Walker",
);

// M12 183 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::LURE,
    "c9704ea0-4dad-4b37-a316-d00766e2a723",
    "D. Alexander Gregory",
);

// M12 184 — Lurking Crocodile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LURKING_CROCODILE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Lurking Crocodile",
    "4cd7d075-031e-4766-89e9-03a8a7197019",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// M12 185 — Naturalize (reprint)
const NATURALIZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::onslaught::NATURALIZE,
    "4bf9c75f-0319-416c-904a-49358f2f943c",
    "Tim Hildebrandt",
);

// M12 186 — Overrun (reprint)
const OVERRUN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::tempest::OVERRUN,
    "ae0559d4-0015-44e4-8ec4-08bb1c54eec5",
    "Carl Critchlow",
);

// M12 187 — Plummet (reprint)
const PLUMMET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::archenemy::PLUMMET,
    "54a1a949-c874-4739-9c7d-ad6fcd2aad44",
    "Pete Venters",
);

// M12 188 — Primeval Titan (reprint)
const PRIMEVAL_TITAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::PRIMEVAL_TITAN,
    "fd6ddbca-b943-49d6-b341-509bb72dd5a6",
    "Aleksi Briclot",
);

// M12 189 — Primordial Hydra
pub(in crate::card::sets) static PRIMORDIAL_HYDRA: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Primordial Hydra",
    "3dcc5521-df8f-4992-b93e-e430d8cc7715",
    "Aleksi Briclot",
    CardRules::new_creature(mana_cost!("{X}{G}{G}"), &["Hydra"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, double the number of +1/+1 counters on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::DoubleCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
            },
        ),
        AbilityDef::static_ability(
            "This creature has trample as long as it has ten or more +1/+1 counters on it.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 10,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            },
        ),
    ]),
);

// M12 190 — Rampant Growth (reprint)
const RAMPANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::RAMPANT_GROWTH,
    "fe45a787-6d8a-48d7-ad6c-fb20a9b468a4",
    "Steven Belledin",
);

// M12 191 — Reclaim (reprint)
const RECLAIM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::exodus::RECLAIM,
    "78f67503-2f0f-43bf-9c4f-a254cc6c501a",
    "Andrew Robinson",
);

// M12 192 — Rites of Flourishing (reprint)
const RITES_OF_FLOURISHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::future_sight::RITES_OF_FLOURISHING,
    "0e3d43ce-8297-47f6-a877-d723b9b43fdb",
    "Brandon Kitkouski",
);

// M12 193 — Runeclaw Bear (reprint)
const RUNECLAW_BEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::RUNECLAW_BEAR,
    "6caf2b93-1971-4702-9aa5-bd223eb37a39",
    "Jesper Ejsing",
);

// M12 194 — Sacred Wolf (reprint)
const SACRED_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::SACRED_WOLF,
    "ff4661dd-2075-48c3-b19b-fc7f8aaba1b8",
    "Matt Stewart",
);

// M12 195 — Skinshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKINSHIFTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Skinshifter",
    "d56c82ad-5eb1-4653-8f02-e9bb1f6f3154",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// M12 196 — Stampeding Rhino (reprint)
const STAMPEDING_RHINO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::STAMPEDING_RHINO,
    "09d34690-f7cc-4161-9a6f-bfc5393e40b2",
    "Steven Belledin",
);

// M12 197 — Stingerfling Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STINGERFLING_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Stingerfling Spider",
    "3b781626-f4ce-4d00-aa7c-0e07f58f688f",
    "Dave Allsop",
    crate::card::CardRules::unsupported(),
);

// M12 198 — Titanic Growth
pub(in crate::card::sets) static TITANIC_GROWTH: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Titanic Growth",
    "db3c8982-e1c2-48be-8094-683d00c2e52b",
    "Ryan Pancoast",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +4/+4 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(4),
                ValueDef::Constant(4),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M12 199 — Trollhide
pub(in crate::card::sets) static TROLLHIDE: CardRecord = CardRecord::new(
    CardSet::Magic2012,
    "Trollhide",
    "32c8d6ed-4764-433b-9617-363e46e5b250",
    "Steven Belledin",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has \"{1}{G}: Regenerate this creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::regenerate_self(
                            "{1}{G}: Regenerate this creature.",
                            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
                        )),
                    ]),
                },
            ),
        ]),
);

// M12 200 — Vastwood Gorger (reprint)
const VASTWOOD_GORGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::VASTWOOD_GORGER,
    "cdd9d448-ebd5-4e01-af88-e755833c2451",
    "Kieran Yanner",
);

// M12 201 — Adaptive Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADAPTIVE_AUTOMATON: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Adaptive Automaton",
    "79e42ead-df6e-4181-ae2b-a2abfc3f1d7c",
    "Igor Kieryluk",
    crate::card::CardRules::unsupported(),
);

// M12 202 — Angel's Feather (reprint)
const ANGEL_S_FEATHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::darksteel::ANGEL_S_FEATHER,
    "3992dc7c-61c0-4d5f-9c32-8febfad4ef6d",
    "Alan Pollack",
);

// M12 203 — Crown of Empires
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_EMPIRES: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Crown of Empires",
    "d4e36991-7b9f-4cc7-8da2-55b8baf19d70",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// M12 204 — Crumbling Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUMBLING_COLOSSUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Crumbling Colossus",
    "b09afa3b-c172-4cd7-b605-bacbfbd07c24",
    "Michael C. Hayes",
    crate::card::CardRules::unsupported(),
);

// M12 205 — Demon's Horn (reprint)
const DEMON_S_HORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::darksteel::DEMON_S_HORN,
    "3f56b129-fe2d-4061-b1c9-f1f5a4db564a",
    "Alan Pollack",
);

// M12 206 — Dragon's Claw (reprint)
const DRAGON_S_CLAW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::darksteel::DRAGON_S_CLAW,
    "0d732b87-08e5-41b6-8448-62dd6bf20d9c",
    "Alan Pollack",
);

// M12 207 — Druidic Satchel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRUIDIC_SATCHEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Druidic Satchel",
    "fddb054f-0617-4afb-8ed1-a067f234f8e7",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// M12 208 — Elixir of Immortality (reprint)
const ELIXIR_OF_IMMORTALITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::ELIXIR_OF_IMMORTALITY,
    "c64e25a3-fcde-4d8f-a376-0c83470ba84f",
    "Zoltan Boros & Gabor Szikszai",
);

// M12 209 — Greatsword
pub(in crate::card::sets) static GREATSWORD: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Greatsword",
    "63b4041d-7c95-4cb9-a18b-6568db05942b",
    "Nic Klein",
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
    crate::card::CardSet::Magic2012,
    "Kite Shield",
    "1a00d1e1-aaa4-4f4d-a887-1e477820d2c6",
    "Jim Pavelec",
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

// M12 211 — Kraken's Eye (reprint)
const KRAKEN_S_EYE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::darksteel::KRAKEN_S_EYE,
    "48052433-c4d3-434e-a609-e8400150a0f6",
    "Alan Pollack",
);

// M12 212 — Manalith
pub(in crate::card::sets) static MANALITH: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Manalith",
    "17bf5f25-82b4-460c-94da-b84daa8a53d9",
    "Charles Urbach",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any color.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// M12 213 — Pentavus (reprint)
const PENTAVUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::PENTAVUS,
    "eb10af81-8ff3-4063-a67a-b760fdba95f8",
    "Greg Staples",
);

// M12 214 — Quicksilver Amulet (reprint)
const QUICKSILVER_AMULET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_legacy::QUICKSILVER_AMULET,
    "04c0357a-e98d-4c49-83ad-d7a8ebe7e2d1",
    "Brad Rigney",
);

// M12 215 — Rusted Sentinel
pub(in crate::card::sets) static RUSTED_SENTINEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Rusted Sentinel",
    "cba5fc44-4b9a-418b-a4e0-26d2c3a1eca4",
    "Jason Felix",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 3, 4)
        .with_ability(abilities::enters_tapped("This creature enters tapped.")),
);

// M12 216 — Scepter of Empires
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCEPTER_OF_EMPIRES: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Scepter of Empires",
    "54f1aaef-94cc-45ab-99c9-8ffdcf331a7b",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// M12 217 — Solemn Simulacrum (reprint)
const SOLEMN_SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::SOLEMN_SIMULACRUM,
    "246d2ce1-6926-4acc-810a-4894dc346b8b",
    "Dan Murayama Scott",
);

// M12 218 — Sundial of the Infinite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNDIAL_OF_THE_INFINITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Sundial of the Infinite",
    "36d3da9c-cb7a-4cea-b6e6-6722bd16c73c",
    "Vincent Proce",
    crate::card::CardRules::unsupported(),
);

// M12 219 — Swiftfoot Boots
pub(in crate::card::sets) static SWIFTFOOT_BOOTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Swiftfoot Boots",
    "8b82753b-284c-44ba-9d48-d28913f02a5f",
    "Svetlin Velinov",
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

// M12 220 — Thran Golem (reprint)
const THRAN_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_destiny::THRAN_GOLEM,
    "f01b98a6-5683-4b1b-a14c-d0b50fc26beb",
    "Ron Spears",
);

// M12 221 — Throne of Empires
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRONE_OF_EMPIRES: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Throne of Empires",
    "87352716-4cf6-4b2f-bb0a-b7aafae64478",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// M12 222 — Worldslayer (reprint)
const WORLDSLAYER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::WORLDSLAYER,
    "db6c6b15-40f3-4556-978f-878bedb13762",
    "Greg Staples",
);

// M12 223 — Wurm's Tooth (reprint)
const WURM_S_TOOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::darksteel::WURM_S_TOOTH,
    "da965767-a8b1-4725-ae20-65c18e37ad27",
    "Alan Pollack",
);

// M12 224 — Buried Ruin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURIED_RUIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2012,
    "Buried Ruin",
    "e910cf59-f7aa-44b1-bb8a-c2211179137c",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// M12 225 — Dragonskull Summit (reprint)
const DRAGONSKULL_SUMMIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DRAGONSKULL_SUMMIT,
    "f99375bc-7465-4f20-897c-1bc61f65de61",
    "Jon Foster",
);

// M12 226 — Drowned Catacomb (reprint)
const DROWNED_CATACOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DROWNED_CATACOMB,
    "39ad4371-6c81-4b4c-98eb-d5c289c8c0e2",
    "Dave Kendall",
);

// M12 227 — Glacial Fortress (reprint)
const GLACIAL_FORTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::GLACIAL_FORTRESS,
    "8b3601d4-4091-465e-8c18-0cd717258211",
    "Franz Vohwinkel",
);

// M12 228 — Rootbound Crag (reprint)
const ROOTBOUND_CRAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ROOTBOUND_CRAG,
    "27c2dd5e-465f-47fd-8f5d-fd65ba133164",
    "Matt Stewart",
);

// M12 229 — Sunpetal Grove (reprint)
const SUNPETAL_GROVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SUNPETAL_GROVE,
    "0c0e02be-e41f-49b4-8393-c4cd2992e380",
    "Jason Chan",
);

// M12 230 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::PLAINS,
    "f768de9b-3d31-468d-876d-2cb7c5c601a3",
    "Rob Alexander",
);

// M12 231 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "1c1cad1f-c81e-4b65-905f-350b02047ea9",
    "D. J. Cleland-Hura",
);

// M12 232 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "d08e9aad-3b06-4cef-8b91-0087bc5881f8",
    "Howard Lyon",
);

// M12 233 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "f59dcb79-1ab1-4204-b50b-14bc3f709c46",
    "Charles Urbach",
);

// M12 234 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::ISLAND,
    "b29cf026-521f-4345-a885-da27f7981759",
    "Rob Alexander",
);

// M12 235 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "81a585d9-abf8-41ff-b6f6-c9396d36906b",
    "Cliff Childs",
);

// M12 236 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "c0e4f4d9-6168-4f83-b632-c369d2d6c256",
    "Michael Komarck",
);

// M12 237 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "589524db-68b9-46eb-a8ab-55986f37fbed",
    "Peter Mohrbacher",
);

// M12 238 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::SWAMP,
    "c29134a2-548b-4c80-8017-7e50a44c3585",
    "Cliff Childs",
);

// M12 239 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "bbd9e399-74fb-4650-ad43-439d79dc31ed",
    "Chippy",
);

// M12 240 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "b63ffa5f-9bab-4c49-bd8f-aa7d970c25c4",
    "Jung Park",
);

// M12 241 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "b930f28b-0415-42db-9591-e688a2d6bcd5",
    "Alan Pollack",
);

// M12 242 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::MOUNTAIN,
    "65d51676-2399-4db9-8c31-c8063be7b94a",
    "Cliff Childs",
);

// M12 243 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "9c81a162-9735-4c3f-8eee-43b4f3b5a59c",
    "Karl Kopinski",
);

// M12 244 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "1a8d9be0-255a-482a-b055-f483859266c5",
    "Robh Ruppel",
);

// M12 245 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "4ce797d6-6773-4f34-be40-2e3443af6466",
    "Sam Wood",
);

// M12 246 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1993::alpha::FOREST,
    "f6302ae7-0947-4a8d-ace4-5e6741aea9ba",
    "Glen Angus",
);

// M12 247 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "5ac2dbbc-8bf4-45fd-a5c1-7597c885fc6b",
    "Volkan Baǵa",
);

// M12 248 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "e84c1369-68a2-4e67-979f-64decb6133a1",
    "Jim Nelson",
);

// M12 249 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "a793d3b8-aa7a-4679-ae28-e0ca2f265b1f",
    "Ryan Pancoast",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AEGIS_ANGEL,
    &ALABASTER_MAGE,
    &ANGELIC_DESTINY,
    &ARBALEST_ELITE,
    &ARMORED_WARHORSE,
    &BENALISH_VETERAN,
    &DIVINE_FAVOR,
    &GIDEON_S_AVENGER,
    &GIDEON_S_LAWKEEPER,
    &GRAND_ABOLISHER,
    &GRIFFIN_RIDER,
    &GUARDIANS_PLEDGE,
    &PEREGRINE_GRIFFIN,
    &PERSONAL_SANCTUARY,
    &PRIDE_GUARDIAN,
    &SPIRIT_MANTLE,
    &STAVE_OFF,
    &STONEHORN_DIGNITARY,
    &TIMELY_REINFORCEMENTS,
    &AMPHIN_CUTTHROAT,
    &AVEN_FLEETWING,
    &AZURE_MAGE,
    &CHASM_DRAKE,
    &FROST_BREATH,
    &JACE_MEMORY_ADEPT,
    &JACE_S_ARCHIVIST,
    &LORD_OF_THE_UNREAL,
    &MASTER_THIEF,
    &MERFOLK_MESMERIST,
    &MIND_UNBOUND,
    &PHANTASMAL_BEAR,
    &PHANTASMAL_DRAGON,
    &PHANTASMAL_IMAGE,
    &SKYWINDER_DRAKE,
    &SPHINX_OF_UTHUUN,
    &TURN_TO_FROG,
    &VISIONS_OF_BEYOND,
    &BLOODLORD_OF_VAASGOTH,
    &BLOODRAGE_VAMPIRE,
    &DARK_FAVOR,
    &DEVOURING_SWARM,
    &DRIFTING_SHADE,
    &DUSKHUNTER_BAT,
    &HIDEOUS_VISAGE,
    &MONOMANIA,
    &ONYX_MAGE,
    &RUNE_SCARRED_DEMON,
    &SORIN_S_THIRST,
    &SORIN_S_VENGEANCE,
    &TASTE_OF_BLOOD,
    &TORMENTED_SOUL,
    &VAMPIRE_OUTCASTS,
    &VENGEFUL_PHARAOH,
    &WRING_FLESH,
    &BLOOD_OGRE,
    &BONEBREAKER_GIANT,
    &CHANDRA_THE_FIREBRAND,
    &CHANDRA_S_PHOENIX,
    &CIRCLE_OF_FLAME,
    &CRIMSON_MAGE,
    &FURYBORN_HELLKITE,
    &GOBLIN_BANGCHUCKERS,
    &GOBLIN_FIRESLINGER,
    &GOREHORN_MINOTAURS,
    &SCRAMBLEVERSE,
    &STORMBLOOD_BERSERKER,
    &TECTONIC_RIFT,
    &WALL_OF_TORCHES,
    &WARSTORM_SURGE,
    &ARACHNUS_SPINNER,
    &ARACHNUS_WEB,
    &CARNAGE_WURM,
    &DOUBLING_CHANT,
    &DUNGROVE_ELDER,
    &GARRUK_PRIMAL_HUNTER,
    &GARRUK_S_HORDE,
    &GLADECOVER_SCOUT,
    &HUNTER_S_INSIGHT,
    &JADE_MAGE,
    &LURKING_CROCODILE,
    &PRIMORDIAL_HYDRA,
    &SKINSHIFTER,
    &STINGERFLING_SPIDER,
    &TITANIC_GROWTH,
    &TROLLHIDE,
    &ADAPTIVE_AUTOMATON,
    &CROWN_OF_EMPIRES,
    &CRUMBLING_COLOSSUS,
    &DRUIDIC_SATCHEL,
    &GREATSWORD,
    &KITE_SHIELD,
    &MANALITH,
    &RUSTED_SENTINEL,
    &SCEPTER_OF_EMPIRES,
    &SUNDIAL_OF_THE_INFINITE,
    &SWIFTFOOT_BOOTS,
    &THRONE_OF_EMPIRES,
    &BURIED_RUIN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANGELS_MERCY_REPRINT,
    ARCHON_OF_JUSTICE_REPRINT,
    ASSAULT_GRIFFIN_REPRINT,
    AURAMANCER_REPRINT,
    CELESTIAL_PURGE_REPRINT,
    DAY_OF_JUDGMENT_REPRINT,
    DEMYSTIFY_REPRINT,
    ELITE_VANGUARD_REPRINT,
    GIDEON_JURA_REPRINT,
    GRIFFIN_SENTINEL_REPRINT,
    HONOR_OF_THE_PURE_REPRINT,
    LIFELINK_REPRINT,
    MESA_ENCHANTRESS_REPRINT,
    MIGHTY_LEAP_REPRINT,
    OBLIVION_RING_REPRINT,
    PACIFISM_REPRINT,
    ROC_EGG_REPRINT,
    SERRA_ANGEL_REPRINT,
    SIEGE_MASTODON_REPRINT,
    STORMFRONT_PEGASUS_REPRINT,
    SUN_TITAN_REPRINT,
    AETHER_ADEPT_REPRINT,
    ALLURING_SIREN_REPRINT,
    BELLTOWER_SPHINX_REPRINT,
    CANCEL_REPRINT,
    CORAL_MERFOLK_REPRINT,
    DIVINATION_REPRINT,
    DJINN_OF_WISHES_REPRINT,
    FLASHFREEZE_REPRINT,
    FLIGHT_REPRINT,
    FROST_TITAN_REPRINT,
    HARBOR_SERPENT_REPRINT,
    ICE_CAGE_REPRINT,
    JACE_S_ERASURE_REPRINT,
    LEVITATION_REPRINT,
    MANA_LEAK_REPRINT,
    MERFOLK_LOOTER_REPRINT,
    MIND_CONTROL_REPRINT,
    NEGATE_REPRINT,
    PONDER_REPRINT,
    REDIRECT_REPRINT,
    TIME_REVERSAL_REPRINT,
    UNSUMMON_REPRINT,
    BLOOD_SEEKER_REPRINT,
    BRINK_OF_DISASTER_REPRINT,
    CALL_TO_THE_GRAVE_REPRINT,
    CEMETERY_REAPER_REPRINT,
    CHILD_OF_NIGHT_REPRINT,
    CONSUME_SPIRIT_REPRINT,
    DEATHMARK_REPRINT,
    DIABOLIC_TUTOR_REPRINT,
    DISENTOMB_REPRINT,
    DISTRESS_REPRINT,
    DOOM_BLADE_REPRINT,
    GRAVE_TITAN_REPRINT,
    GRAVEDIGGER_REPRINT,
    MIND_ROT_REPRINT,
    REASSEMBLING_SKELETON_REPRINT,
    ROYAL_ASSASSIN_REPRINT,
    SENGIR_VAMPIRE_REPRINT,
    SMALLPOX_REPRINT,
    SORIN_MARKOV_REPRINT,
    SUTURED_GHOUL_REPRINT,
    WARPATH_GHOUL_REPRINT,
    ZOMBIE_GOLIATH_REPRINT,
    ZOMBIE_INFESTATION_REPRINT,
    ACT_OF_TREASON_REPRINT,
    CHANDRAS_OUTRAGE_REPRINT,
    COMBUST_REPRINT,
    FIERY_HELLHOUND_REPRINT,
    FIREBALL_REPRINT,
    FIREBREATHING_REPRINT,
    FLAMEBLAST_DRAGON_REPRINT,
    FLING_REPRINT,
    GOBLIN_ARSONIST_REPRINT,
    GOBLIN_CHIEFTAIN_REPRINT,
    GOBLIN_GRENADE_REPRINT,
    GOBLIN_PIKER_REPRINT,
    GOBLIN_TUNNELER_REPRINT,
    GOBLIN_WAR_PAINT_REPRINT,
    GRIM_LAVAMANCER_REPRINT,
    INCINERATE_REPRINT,
    INFERNO_TITAN_REPRINT,
    LAVA_AXE_REPRINT,
    LIGHTNING_ELEMENTAL_REPRINT,
    MANABARBS_REPRINT,
    MANIC_VANDAL_REPRINT,
    REVERBERATE_REPRINT,
    SHOCK_REPRINT,
    SLAUGHTER_CRY_REPRINT,
    VOLCANIC_DRAGON_REPRINT,
    ACIDIC_SLIME_REPRINT,
    AUTUMN_S_VEIL_REPRINT,
    BIRDS_OF_PARADISE_REPRINT,
    BOUNTIFUL_HARVEST_REPRINT,
    BRINDLE_BOAR_REPRINT,
    CUDGEL_TROLL_REPRINT,
    ELVISH_ARCHDRUID_REPRINT,
    FOG_REPRINT,
    GARRUK_S_COMPANION_REPRINT,
    GIANT_SPIDER_REPRINT,
    GREATER_BASILISK_REPRINT,
    LLANOWAR_ELVES_REPRINT,
    LURE_REPRINT,
    NATURALIZE_REPRINT,
    OVERRUN_REPRINT,
    PLUMMET_REPRINT,
    PRIMEVAL_TITAN_REPRINT,
    RAMPANT_GROWTH_REPRINT,
    RECLAIM_REPRINT,
    RITES_OF_FLOURISHING_REPRINT,
    RUNECLAW_BEAR_REPRINT,
    SACRED_WOLF_REPRINT,
    STAMPEDING_RHINO_REPRINT,
    VASTWOOD_GORGER_REPRINT,
    ANGEL_S_FEATHER_REPRINT,
    DEMON_S_HORN_REPRINT,
    DRAGON_S_CLAW_REPRINT,
    ELIXIR_OF_IMMORTALITY_REPRINT,
    KRAKEN_S_EYE_REPRINT,
    PENTAVUS_REPRINT,
    QUICKSILVER_AMULET_REPRINT,
    SOLEMN_SIMULACRUM_REPRINT,
    THRAN_GOLEM_REPRINT,
    WORLDSLAYER_REPRINT,
    WURM_S_TOOTH_REPRINT,
    DRAGONSKULL_SUMMIT_REPRINT,
    DROWNED_CATACOMB_REPRINT,
    GLACIAL_FORTRESS_REPRINT,
    ROOTBOUND_CRAG_REPRINT,
    SUNPETAL_GROVE_REPRINT,
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
