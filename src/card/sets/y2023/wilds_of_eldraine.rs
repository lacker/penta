//! Wilds of Eldraine card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::CardPartId;
use crate::PlayOptionId;
use crate::card::AbilityDef;
use crate::card::AbilityOperationDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AdditionalCostValueDef;
use crate::card::AlternateSpellKind;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardComposition;
use crate::card::CardEffectStatus;
use crate::card::CardPart;
use crate::card::CardRules;
use crate::card::CardStructure;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::OptionalAdditionalCostAbilityDef;
use crate::card::OptionalAdditionalCostKindDef;
use crate::card::PlayOptionDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellForm;
use crate::card::SpellResolutionDestinationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y2004::champions_of_kamigawa as catalog_chk;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2014::khans_of_tarkir as catalog_ktk;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;
use crate::card::sets::y2020::theros_beyond_death as catalog_thb;
use crate::card::sets::y2022::dominaria_united as catalog_dmu;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WOE",
    slug: "wilds-of-eldraine",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const DEFENSELESS_RAT_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Rat"], &[ManaColor::Black], 1, 1)
        .with_abilities(&[AbilityDef::static_ability(
            "This token can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::CANNOT_BLOCK,
                )),
            },
        )])
        .with_art(CardArt::new(
            "1e0205f2-25c1-403b-b408-56e3f2d63b4d",
            "Kim Sokol",
        ));

// WOE 1 — Archon of the Wild Rose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHON_OF_THE_WILD_ROSE: CardRecord = CardRecord::new(
    "Archon of the Wild Rose",
    "00174be7-0dc8-43b9-81b6-f25a8c3fb4eb",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// WOE 2 — Archon's Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHON_S_GLORY: CardRecord = CardRecord::new(
    "Archon's Glory",
    "e71768e7-4ef7-4fb2-838b-eb3a7f662d38",
    "Anastasia Ovchinnikova",
    crate::card::CardRules::unsupported(),
);

// WOE 3 — Armory Mice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMORY_MICE: CardRecord = CardRecord::new(
    "Armory Mice",
    "4b041949-6fb6-40a6-9329-f209be537219",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// WOE 4 — Besotted Knight // Betroth the Beast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BESOTTED_KNIGHT: CardRecord = CardRecord::new(
    "Besotted Knight // Betroth the Beast",
    "5980a930-c7f8-45e1-a18a-87734d9ed09e",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// WOE 5 — Break the Spell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAK_THE_SPELL: CardRecord = CardRecord::new(
    "Break the Spell",
    "f7094fc0-1d26-429c-9b49-37718c4a5c80",
    "Miranda Meeks",
    crate::card::CardRules::unsupported(),
);

// WOE 6 — Charmed Clothier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARMED_CLOTHIER: CardRecord = CardRecord::new(
    "Charmed Clothier",
    "994f4473-dfc9-45cd-8528-945db3aa6a9a",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// WOE 7 — Cheeky House-Mouse // Squeak By
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHEEKY_HOUSE_MOUSE: CardRecord = CardRecord::new(
    "Cheeky House-Mouse // Squeak By",
    "1f3013bf-9647-4bdb-a638-d299ae00f88e",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// WOE 8 — Cooped Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COOPED_UP: CardRecord = CardRecord::new(
    "Cooped Up",
    "8acb8758-09c5-4e19-ada1-904e36ece1fc",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// WOE 9 — Cursed Courtier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURSED_COURTIER: CardRecord = CardRecord::new(
    "Cursed Courtier",
    "3d2d5a71-d6e1-4c96-9a53-0e370047a56e",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// WOE 10 — Discerning Financier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCERNING_FINANCIER: CardRecord = CardRecord::new(
    "Discerning Financier",
    "584774b5-640f-45e0-810b-f5faf119645b",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// WOE 11 — Dutiful Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUTIFUL_GRIFFIN: CardRecord = CardRecord::new(
    "Dutiful Griffin",
    "b8e40377-990f-41ea-8dd2-62a998dcd128",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// WOE 12 — Eerie Interference
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EERIE_INTERFERENCE: CardRecord = CardRecord::new(
    "Eerie Interference",
    "42a74545-75c8-4a6b-bee1-ac5665d9bcf0",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// WOE 13 — Expel the Interlopers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPEL_THE_INTERLOPERS: CardRecord = CardRecord::new(
    "Expel the Interlopers",
    "1094eef0-6c57-4bfa-a584-f708b87354fb",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// WOE 14 — Frostbridge Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROSTBRIDGE_GUARD: CardRecord = CardRecord::new(
    "Frostbridge Guard",
    "b9dec431-a20e-4c1c-9855-904779756509",
    "Paul Scott Canavan",
    crate::card::CardRules::unsupported(),
);

// WOE 15 — Gallant Pie-Wielder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALLANT_PIE_WIELDER: CardRecord = CardRecord::new(
    "Gallant Pie-Wielder",
    "e053d330-d0a2-4468-afba-42bf165b8fbf",
    "Matt Forsyth",
    crate::card::CardRules::unsupported(),
);

// WOE 16 — Glass Casket (reprint)
const GLASS_CASKET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::GLASS_CASKET,
    "02c5c395-ee8b-47fd-ac52-256354c19cdf",
    "Raoul Vitale",
);

// WOE 17 — Hopeful Vigil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOPEFUL_VIGIL: CardRecord = CardRecord::new(
    "Hopeful Vigil",
    "d382fd32-b1f5-4ec7-9d42-fc2915ed3bc9",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// WOE 18 — Kellan's Lightblades
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELLAN_S_LIGHTBLADES: CardRecord = CardRecord::new(
    "Kellan's Lightblades",
    "0cc727a6-f875-49b1-b7a4-67f22fbc3d50",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// WOE 19 — Knight of Doves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_DOVES: CardRecord = CardRecord::new(
    "Knight of Doves",
    "b4f33617-ad19-4d42-ab94-6f21a7fb3dd4",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// WOE 20 — Moment of Valor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENT_OF_VALOR: CardRecord = CardRecord::new(
    "Moment of Valor",
    "6f257fd6-24a4-4cc1-89e0-99cf5d821e3a",
    "Joshua Cairos",
    crate::card::CardRules::unsupported(),
);

// WOE 21 — Moonshaker Cavalry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONSHAKER_CAVALRY: CardRecord = CardRecord::new(
    "Moonshaker Cavalry",
    "092c48bd-b648-4c9e-aa99-cac3c407911d",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// WOE 22 — Plunge into Winter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLUNGE_INTO_WINTER: CardRecord = CardRecord::new(
    "Plunge into Winter",
    "fe4aceba-f386-457c-bc68-6382eda46754",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// WOE 23 — The Princess Takes Flight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_PRINCESS_TAKES_FLIGHT: CardRecord = CardRecord::new(
    "The Princess Takes Flight",
    "dad7bd06-22e4-40f8-bda9-bcdbb2d8f632",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// WOE 24 — Protective Parents
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROTECTIVE_PARENTS: CardRecord = CardRecord::new(
    "Protective Parents",
    "68cc9653-80ef-4606-a0ec-6d4228fdb118",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// WOE 25 — Regal Bunnicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REGAL_BUNNICORN: CardRecord = CardRecord::new(
    "Regal Bunnicorn",
    "03c7d409-90e7-44d7-a8c6-4eda35fbcc83",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// WOE 26 — Return Triumphant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETURN_TRIUMPHANT: CardRecord = CardRecord::new(
    "Return Triumphant",
    "f5209c13-9591-48eb-8d6c-112b3bdd429a",
    "Will Gist",
    crate::card::CardRules::unsupported(),
);

// WOE 27 — Rimefur Reindeer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIMEFUR_REINDEER: CardRecord = CardRecord::new(
    "Rimefur Reindeer",
    "60acc0b7-6842-44aa-a7cc-c5d315d90287",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// WOE 28 — Savior of the Sleeping
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVIOR_OF_THE_SLEEPING: CardRecord = CardRecord::new(
    "Savior of the Sleeping",
    "7b4979d9-fafb-4e0e-868f-f4772109d7a7",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// WOE 29 — Slumbering Keepguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLUMBERING_KEEPGUARD: CardRecord = CardRecord::new(
    "Slumbering Keepguard",
    "d7f41ae2-ebc6-439f-95af-c34818a3f4e6",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// WOE 30 — Solitary Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLITARY_SANCTUARY: CardRecord = CardRecord::new(
    "Solitary Sanctuary",
    "d155693c-def0-4290-b662-ab9932e07fe5",
    "Kasia 'Kafis' Zielińska",
    crate::card::CardRules::unsupported(),
);

// WOE 31 — Spellbook Vendor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLBOOK_VENDOR: CardRecord = CardRecord::new(
    "Spellbook Vendor",
    "4ceac5b5-05eb-4f00-9477-3db490be24dd",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// WOE 32 — Stockpiling Celebrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOCKPILING_CELEBRANT: CardRecord = CardRecord::new(
    "Stockpiling Celebrant",
    "0214ebc6-c59f-4170-8dd2-ce07caa6e6ad",
    "Raluca Marinescu",
    crate::card::CardRules::unsupported(),
);

// WOE 33 — Stroke of Midnight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STROKE_OF_MIDNIGHT: CardRecord = CardRecord::new(
    "Stroke of Midnight",
    "289ba7ec-e30e-436f-b8d4-c88b65ecd137",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// WOE 34 — A Tale for the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static A_TALE_FOR_THE_AGES: CardRecord = CardRecord::new(
    "A Tale for the Ages",
    "ca0c8d3b-ce30-4da5-a6a8-9bdcb3c757f9",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// WOE 35 — Three Blind Mice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_BLIND_MICE: CardRecord = CardRecord::new(
    "Three Blind Mice",
    "0d2ba371-854c-4529-b72b-e4a1887e33ab",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// WOE 36 — Tuinvale Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUINVALE_GUIDE: CardRecord = CardRecord::new(
    "Tuinvale Guide",
    "01334fed-781e-4864-83fd-d37f787a778b",
    "Anastasia Ovchinnikova",
    crate::card::CardRules::unsupported(),
);

// WOE 37 — Unassuming Sage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNASSUMING_SAGE: CardRecord = CardRecord::new(
    "Unassuming Sage",
    "a66fbaeb-1624-43b3-83e2-a37ce7588a5a",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// WOE 38 — Virtue of Loyalty // Ardenvale Fealty (alternate printing)
const VIRTUE_OF_LOYALTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_LOYALTY,
    1,
    "ea7e7daf-7c06-4c74-8bcf-e42c1f611861",
    "Piotr Dura",
);

// WOE 39 — Werefox Bodyguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEREFOX_BODYGUARD: CardRecord = CardRecord::new(
    "Werefox Bodyguard",
    "4494dfa1-1343-417e-b0c5-2b096442dd0e",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// WOE 40 — Aquatic Alchemist // Bubble Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AQUATIC_ALCHEMIST: CardRecord = CardRecord::new(
    "Aquatic Alchemist // Bubble Up",
    "e6f03f21-aeb9-428b-9167-b2604919bdd8",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// WOE 41 — Archive Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHIVE_DRAGON: CardRecord = CardRecord::new(
    "Archive Dragon",
    "2979104f-7570-487c-8024-131d7ee3ab91",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// WOE 42 — Asinine Antics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASININE_ANTICS: CardRecord = CardRecord::new(
    "Asinine Antics",
    "50b96a97-0d7d-4e05-9e2f-0b99a039b655",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// WOE 43 — Beluna's Gatekeeper // Entry Denied
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELUNA_S_GATEKEEPER: CardRecord = CardRecord::new(
    "Beluna's Gatekeeper // Entry Denied",
    "5c1d410e-4237-4963-b015-54d26730e63d",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// WOE 44 — Bitter Chill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BITTER_CHILL: CardRecord = CardRecord::new(
    "Bitter Chill",
    "888e3c71-e21d-4e77-b1b6-09769f9cd3d6",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// WOE 45 — Chancellor of Tales
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHANCELLOR_OF_TALES: CardRecord = CardRecord::new(
    "Chancellor of Tales",
    "f67bd5ef-305b-4bf7-990b-3014778b14a0",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// WOE 46 — Diminisher Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMINISHER_WITCH: CardRecord = CardRecord::new(
    "Diminisher Witch",
    "646d604f-b187-4122-bd4b-67634654b6f1",
    "Fariba Khamseh",
    crate::card::CardRules::unsupported(),
);

// WOE 47 — Disdainful Stroke (reprint)
const DISDAINFUL_STROKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::DISDAINFUL_STROKE,
    "588c6217-c460-417e-98bf-de5475780baf",
    "Eelis Kyttanen",
);

// WOE 48 — Extraordinary Journey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTRAORDINARY_JOURNEY: CardRecord = CardRecord::new(
    "Extraordinary Journey",
    "a69fb480-a9fc-4f09-ac4e-3ce52c485ea9",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// WOE 49 — Farsight Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FARSIGHT_RITUAL: CardRecord = CardRecord::new(
    "Farsight Ritual",
    "c958257e-fa70-4fa3-90a1-0497967abef3",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// WOE 50 — Freeze in Place
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREEZE_IN_PLACE: CardRecord = CardRecord::new(
    "Freeze in Place",
    "1a8bb9c7-2c4b-48a1-806e-742addb72b4b",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// WOE 51 — Gadwick's First Duel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GADWICK_S_FIRST_DUEL: CardRecord = CardRecord::new(
    "Gadwick's First Duel",
    "af07c47f-8b4e-43cb-b469-2efb82aa5590",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// WOE 52 — Galvanic Giant // Storm Reading
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALVANIC_GIANT: CardRecord = CardRecord::new(
    "Galvanic Giant // Storm Reading",
    "60976109-30ad-4f12-99eb-c5ef560fcf1b",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// WOE 53 — Horned Loch-Whale // Lagoon Breach
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORNED_LOCH_WHALE: CardRecord = CardRecord::new(
    "Horned Loch-Whale // Lagoon Breach",
    "96a05063-0556-42e4-8d4c-8e92be160ef5",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// WOE 54 — Ice Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_OUT: CardRecord = CardRecord::new(
    "Ice Out",
    "88ffafda-c852-497a-8156-4f759cdf3693",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// WOE 55 — Icewrought Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICEWROUGHT_SENTRY: CardRecord = CardRecord::new(
    "Icewrought Sentry",
    "859419d3-cd15-4362-98b3-a7ff98e29692",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// WOE 56 — Ingenious Prodigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INGENIOUS_PRODIGY: CardRecord = CardRecord::new(
    "Ingenious Prodigy",
    "cf224968-b676-40dd-83c1-a9ee2ceba574",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// WOE 57 — Into the Fae Court
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTO_THE_FAE_COURT: CardRecord = CardRecord::new(
    "Into the Fae Court",
    "969b13bb-6411-41f9-b6b4-af4ffca62e17",
    "Anna Steinbauer",
    crate::card::CardRules::unsupported(),
);

// WOE 58 — Johann's Stopgap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOHANN_S_STOPGAP: CardRecord = CardRecord::new(
    "Johann's Stopgap",
    "31408397-36f5-479f-b822-fa97411b7872",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// WOE 59 — Living Lectern
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_LECTERN: CardRecord = CardRecord::new(
    "Living Lectern",
    "169afcaf-7ebf-4590-9e51-2a1a5eb3ac76",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// WOE 60 — Merfolk Coralsmith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERFOLK_CORALSMITH: CardRecord = CardRecord::new(
    "Merfolk Coralsmith",
    "a7f6a79c-aa2f-47d2-a929-1606a61f9341",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// WOE 61 — Misleading Motes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISLEADING_MOTES: CardRecord = CardRecord::new(
    "Misleading Motes",
    "4c6c7a43-c3d1-450e-834a-54e1b9def1cd",
    "Raoul Vitale",
    crate::card::CardRules::unsupported(),
);

// WOE 62 — Mocking Sprite
pub(in crate::card::sets) static MOCKING_SPRITE: CardRecord = CardRecord::new(
    "Mocking Sprite",
    "e595014d-4ff4-4561-b7f2-a9bd56300b01",
    "Ben Hill",
    // The discount is read off the battlefield, so an evasive body that
    // survives is what makes it pay -- and flying is why it does.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Instant and sorcery spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
    ]),
);

// WOE 63 — Obyra's Attendants // Desperate Parry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBYRA_S_ATTENDANTS: CardRecord = CardRecord::new(
    "Obyra's Attendants // Desperate Parry",
    "0001e77a-7fff-49d2-a55c-42f6fdf6db08",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// WOE 64 — Picklock Prankster // Free the Fae
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PICKLOCK_PRANKSTER: CardRecord = CardRecord::new(
    "Picklock Prankster // Free the Fae",
    "5ebac73a-1ecf-4e6d-87b1-ea560bfeb064",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// WOE 65 — Quick Study
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICK_STUDY: CardRecord = CardRecord::new(
    "Quick Study",
    "b78e2bca-bc93-464a-8911-8361abff2ac6",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// WOE 66 — Sleep-Cursed Faerie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLEEP_CURSED_FAERIE: CardRecord = CardRecord::new(
    "Sleep-Cursed Faerie",
    "31051436-68f2-457e-8293-2b10ccf7684e",
    "Heonhwa",
    crate::card::CardRules::unsupported(),
);

// WOE 67 — Sleight of Hand (reprint)
const SLEIGHT_OF_HAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::SLEIGHT_OF_HAND,
    "80dea5c0-ada3-488a-9f2b-f895b92c762f",
    "Scott Murphy",
);

// WOE 68 — Snaremaster Sprite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAREMASTER_SPRITE: CardRecord = CardRecord::new(
    "Snaremaster Sprite",
    "eaa37390-5c32-46ae-89d1-c094c9aa01e5",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// WOE 69 — Spell Stutter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELL_STUTTER: CardRecord = CardRecord::new(
    "Spell Stutter",
    "24447e36-a42f-40a9-ad44-e904b6f9b276",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// WOE 70 — Splashy Spellcaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLASHY_SPELLCASTER: CardRecord = CardRecord::new(
    "Splashy Spellcaster",
    "73ebd7f0-a54d-43a8-a5ee-9d6835308794",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// WOE 71 — Stormkeld Prowler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMKELD_PROWLER: CardRecord = CardRecord::new(
    "Stormkeld Prowler",
    "ff065dbf-77e3-45a8-bcca-aff9eaeb151f",
    "Zara Alfonso",
    crate::card::CardRules::unsupported(),
);

// WOE 72 — Succumb to the Cold
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUCCUMB_TO_THE_COLD: CardRecord = CardRecord::new(
    "Succumb to the Cold",
    "c14d9fc0-bfbf-4359-93bf-5e53466965d6",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// WOE 73 — Talion's Messenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TALION_S_MESSENGER: CardRecord = CardRecord::new(
    "Talion's Messenger",
    "35fb0640-5b04-4687-b863-46a8b8d36809",
    "Marta Nael",
    crate::card::CardRules::unsupported(),
);

// WOE 74 — Tenacious Tomeseeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TENACIOUS_TOMESEEKER: CardRecord = CardRecord::new(
    "Tenacious Tomeseeker",
    "c40be735-0780-459b-8dd2-a298575beaab",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// WOE 75 — Vantress Transmuter // Croaking Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VANTRESS_TRANSMUTER: CardRecord = CardRecord::new(
    "Vantress Transmuter // Croaking Curse",
    "11507fa1-ef9e-41c9-b987-be57a03bd0df",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// WOE 76 — Virtue of Knowledge // Vantress Visions
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIRTUE_OF_KNOWLEDGE: CardRecord = CardRecord::new(
    "Virtue of Knowledge // Vantress Visions",
    "df606cf5-67dc-46f4-8c79-1d2f1d054391",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 77 — Water Wings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATER_WINGS: CardRecord = CardRecord::new(
    "Water Wings",
    "4ea4993c-d1ba-4b33-955b-e0874fd2132f",
    "Arash Radkia",
    crate::card::CardRules::unsupported(),
);

// WOE 78 — Ashiok, Wicked Manipulator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHIOK_WICKED_MANIPULATOR: CardRecord = CardRecord::new(
    "Ashiok, Wicked Manipulator",
    "6c4d0db1-74a5-42c0-ac95-e696585d8022",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// WOE 79 — Ashiok's Reaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHIOK_S_REAPER: CardRecord = CardRecord::new(
    "Ashiok's Reaper",
    "83957fe0-6500-420c-9b7a-2448a1c1d3b3",
    "Denis Zhbankov",
    crate::card::CardRules::unsupported(),
);

// WOE 80 — Back for Seconds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BACK_FOR_SECONDS: CardRecord = CardRecord::new(
    "Back for Seconds",
    "660845b5-96fa-4484-822b-aa0508801306",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// WOE 81 — Barrow Naughty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARROW_NAUGHTY: CardRecord = CardRecord::new(
    "Barrow Naughty",
    "67da5ad8-4de2-4bd4-8b95-f2657e1fdee5",
    "Matt Forsyth",
    crate::card::CardRules::unsupported(),
);

// WOE 82 — Beseech the Mirror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BESEECH_THE_MIRROR: CardRecord = CardRecord::new(
    "Beseech the Mirror",
    "18c59776-e1f1-4197-a128-db1d603f56b7",
    "Cynthia Sheppard",
    crate::card::CardRules::unsupported(),
);

// WOE 83 — Candy Grapple
pub(in crate::card::sets) static CANDY_GRAPPLE: CardRecord = CardRecord::new(
    "Candy Grapple",
    "190d97bc-dbef-496d-9bd1-b785bdf8a964",
    "Konstantin Porubov",
    // Two mana kills most of what a limited deck plays, and the Food this
    // set hands out is what turns the rest into targets too.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::optional_additional_cost(
            "Bargain (You may sacrifice an artifact, enchantment, or token as you cast this \
             spell.)",
            OptionalAdditionalCostAbilityDef {
                kind: OptionalAdditionalCostKindDef::Bargain,
                label: OptionalAdditionalCostKindDef::Bargain.label(),
                resolution_destination: SpellResolutionDestinationDef::Graveyard,
                costs: &[CostDef::Sacrifice {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::Token,
                    ]),
                    quantity: CostQuantityDef::Fixed(1),
                }],
            },
        ),
        AbilityDef::spell_with_targets(
            "Target creature gets -3/-3 until end of turn. If this spell was bargained, that \
             creature gets -5/-5 until end of turn instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                // One effect reading the payment back, not two: "instead"
                // means the bargained spell never applies the smaller number.
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(-5),
                        ValueDef::Constant(-3),
                    )),
                    ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(-5),
                        ValueDef::Constant(-3),
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);


/// The Rat this set's Rat deck keeps printing: a 1/1 body that attacks and
/// never blocks. A static rather than a const fn, because the ability slice
/// only gets a `'static` lifetime in a static initializer.
static DEFENSELESS_RAT_TOKEN: EffectDef =
    EffectDef::create_creature_token(&["Rat"], &[ManaColor::Black], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This token can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::CANNOT_BLOCK,
                )),
            },
        ),
    ]);

// WOE 84 — Conceited Witch // Price of Beauty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONCEITED_WITCH: CardRecord = CardRecord::new(
    "Conceited Witch // Price of Beauty",
    "f8a0c0f6-fef9-42c5-934d-a2855c11b440",
    "Anna Pavleeva",
    crate::card::CardRules::unsupported(),
);

// WOE 85 — Dream Spoilers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_SPOILERS: CardRecord = CardRecord::new(
    "Dream Spoilers",
    "4efd1963-fe71-42c1-8ad7-53fd80145ca6",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// WOE 86 — Ego Drain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EGO_DRAIN: CardRecord = CardRecord::new(
    "Ego Drain",
    "8faf36da-bbad-4d6e-a530-502d47a2dd23",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// WOE 87 — The End
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_END: CardRecord = CardRecord::new(
    "The End",
    "b18402dc-c4ab-417c-92d1-5e4d9cfb840d",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// WOE 88 — Eriette's Whisper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERIETTE_S_WHISPER: CardRecord = CardRecord::new(
    "Eriette's Whisper",
    "dfed2acf-9ac8-447b-94f5-7db3a713c991",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// WOE 89 — Faerie Dreamthief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_DREAMTHIEF: CardRecord = CardRecord::new(
    "Faerie Dreamthief",
    "57ca2ec5-442d-4909-be28-93c50fbc5f7a",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// WOE 90 — Faerie Fencing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_FENCING: CardRecord = CardRecord::new(
    "Faerie Fencing",
    "6bdcaf24-4352-47cf-a043-899be47ab1bb",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// WOE 91 — Feed the Cauldron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEED_THE_CAULDRON: CardRecord = CardRecord::new(
    "Feed the Cauldron",
    "0cfd18a5-e06a-4cb5-b78e-de18ec641321",
    "Marta Nael",
    crate::card::CardRules::unsupported(),
);

// WOE 92 — Fell Horseman // Deathly Ride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FELL_HORSEMAN: CardRecord = CardRecord::new(
    "Fell Horseman // Deathly Ride",
    "43bb3890-4013-48be-8cb5-54fd8fd8ec52",
    "Igor Krstic",
    crate::card::CardRules::unsupported(),
);

// WOE 93 — Gumdrop Poisoner // Tempt with Treats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUMDROP_POISONER: CardRecord = CardRecord::new(
    "Gumdrop Poisoner // Tempt with Treats",
    "5cb01d4d-91c2-41c6-981e-b4135a1e1e36",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// WOE 94 — High Fae Negotiator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_FAE_NEGOTIATOR: CardRecord = CardRecord::new(
    "High Fae Negotiator",
    "f0fc77e7-154d-4433-93b3-1a1dee34791b",
    "Anna Christenson",
    crate::card::CardRules::unsupported(),
);

// WOE 95 — Hopeless Nightmare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOPELESS_NIGHTMARE: CardRecord = CardRecord::new(
    "Hopeless Nightmare",
    "2c2ee817-9ca9-4f09-bc71-7994c19a9470",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// WOE 96 — Lich-Knights' Conquest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LICH_KNIGHTS_CONQUEST: CardRecord = CardRecord::new(
    "Lich-Knights' Conquest",
    "59cd67d3-3327-42ad-9db1-50e2f591818c",
    "Denis Zhbankov",
    crate::card::CardRules::unsupported(),
);

// WOE 97 — Lord Skitter, Sewer King
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_SKITTER_SEWER_KING: CardRecord = CardRecord::new(
    "Lord Skitter, Sewer King",
    "729877be-4894-4ef5-9e60-de8a8fb2bdc0",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// WOE 98 — Lord Skitter's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_SKITTER_S_BLESSING: CardRecord = CardRecord::new(
    "Lord Skitter's Blessing",
    "84f343a9-883f-4532-ae66-be6470d67d38",
    "Joseph Weston",
    crate::card::CardRules::unsupported(),
);

// WOE 99 — Lord Skitter's Butcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_SKITTER_S_BUTCHER: CardRecord = CardRecord::new(
    "Lord Skitter's Butcher",
    "21b31d2b-ef66-4e16-a75e-4e27eb5ebfe9",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// WOE 100 — Mintstrosity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINTSTROSITY: CardRecord = CardRecord::new(
    "Mintstrosity",
    "d902f154-6fe8-4b97-aa67-4d4696abf887",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// WOE 101 — Not Dead After All
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOT_DEAD_AFTER_ALL: CardRecord = CardRecord::new(
    "Not Dead After All",
    "d01a2b68-efe6-4027-846d-db7b19d9eef6",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// WOE 102 — Rankle's Prank
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANKLE_S_PRANK: CardRecord = CardRecord::new(
    "Rankle's Prank",
    "e8a9bdcf-160a-48c9-9750-778c910b805d",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// WOE 103 — Rat Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAT_OUT: CardRecord = CardRecord::new(
    "Rat Out",
    "f2c42755-bf91-4c75-95c6-d2a60ba3492a",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// WOE 104 — Rowan's Grim Search
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROWAN_S_GRIM_SEARCH: CardRecord = CardRecord::new(
    "Rowan's Grim Search",
    "1be6786e-0569-42dd-b03c-82da7b32a14f",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// WOE 105 — Scream Puff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCREAM_PUFF: CardRecord = CardRecord::new(
    "Scream Puff",
    "c62d0ae9-5a82-40bf-b8bb-9c2e2d55d458",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// WOE 106 — Shatter the Oath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTER_THE_OATH: CardRecord = CardRecord::new(
    "Shatter the Oath",
    "cc79f0f7-0a09-4a74-b2b9-cc1ce608d89f",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// WOE 107 — Specter of Mortality
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTER_OF_MORTALITY: CardRecord = CardRecord::new(
    "Specter of Mortality",
    "5e4c00b5-f6d6-4fbd-828f-ad30321f2cd9",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// WOE 108 — Spiteful Hexmage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITEFUL_HEXMAGE: CardRecord = CardRecord::new(
    "Spiteful Hexmage",
    "40c797b2-db51-4a39-b80e-44d58cd7a07c",
    "Anna Steinbauer",
    crate::card::CardRules::unsupported(),
);

// WOE 109 — Stingblade Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STINGBLADE_ASSASSIN: CardRecord = CardRecord::new(
    "Stingblade Assassin",
    "b71b7dd9-6a1d-4c71-873b-782a0a2e7d1d",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// WOE 110 — Sugar Rush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUGAR_RUSH: CardRecord = CardRecord::new(
    "Sugar Rush",
    "a638c9ba-45d6-4b50-a8ab-ef580a0a5d8e",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// WOE 111 — Sweettooth Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWEETTOOTH_WITCH: CardRecord = CardRecord::new(
    "Sweettooth Witch",
    "a6bdb984-06f8-4bca-a943-17fe5db97682",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// WOE 112 — Taken by Nightmares
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAKEN_BY_NIGHTMARES: CardRecord = CardRecord::new(
    "Taken by Nightmares",
    "f858d83d-a13e-4ffa-a91d-d695e5e5d71a",
    "Artur Treffner",
    crate::card::CardRules::unsupported(),
);

// WOE 113 — Tangled Colony
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLED_COLONY: CardRecord = CardRecord::new(
    "Tangled Colony",
    "77111ce8-6469-4bf7-882a-4ded1e5d7cad",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// WOE 114 — Twisted Sewer-Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWISTED_SEWER_WITCH: CardRecord = CardRecord::new(
    "Twisted Sewer-Witch",
    "d6e3ddf7-582d-4923-be30-8428e52237e4",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// WOE 115 — Virtue of Persistence // Locthwain Scorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIRTUE_OF_PERSISTENCE: CardRecord = CardRecord::new(
    "Virtue of Persistence // Locthwain Scorn",
    "f1e5cafb-b0e6-4ee5-8c58-6f8e5ef2b9da",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 116 — Voracious Vermin
pub(in crate::card::sets) static VORACIOUS_VERMIN: CardRecord = CardRecord::new(
    "Voracious Vermin",
    "8059be65-3c73-49bb-a3b6-c346ce2f9fa4",
    "Milivoj Ćeran",
// The Rat it brings is also the first thing to feed it: a sacrifice
    // outlet turns the token into a counter.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 black Rat creature token with \"This token can't block.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DEFENSELESS_RAT_TOKEN))),
        ),
        AbilityDef::triggered(
            "Whenever another creature you control dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WOE 117 — Warehouse Tabby
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAREHOUSE_TABBY: CardRecord = CardRecord::new(
    "Warehouse Tabby",
    "d500ad81-9659-4a00-8f99-8be7c23587e8",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// WOE 118 — Wicked Visitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WICKED_VISITOR: CardRecord = CardRecord::new(
    "Wicked Visitor",
    "e26ec4b8-0012-48c4-9ccb-0f062df0c250",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// WOE 119 — The Witch's Vanity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_WITCH_S_VANITY: CardRecord = CardRecord::new(
    "The Witch's Vanity",
    "47ca4926-b5ac-405a-8b58-f8db6df400ff",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// WOE 120 — Belligerent of the Ball
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELLIGERENT_OF_THE_BALL: CardRecord = CardRecord::new(
    "Belligerent of the Ball",
    "6658398a-46a5-4f41-9b1b-4a47f2822cf8",
    "Pascal Quidault",
    crate::card::CardRules::unsupported(),
);

// WOE 121 — Bellowing Bruiser // Beat a Path
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELLOWING_BRUISER: CardRecord = CardRecord::new(
    "Bellowing Bruiser // Beat a Path",
    "26ece013-f3ef-4c12-9dea-b2789f61f8a0",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// WOE 122 — Bespoke Battlegarb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BESPOKE_BATTLEGARB: CardRecord = CardRecord::new(
    "Bespoke Battlegarb",
    "a28ecd59-9166-473c-bafc-cb3c54c21388",
    "Nino Vecia",
    crate::card::CardRules::unsupported(),
);

// WOE 123 — Boundary Lands Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNDARY_LANDS_RANGER: CardRecord = CardRecord::new(
    "Boundary Lands Ranger",
    "ccdcbb1d-702e-4832-8eed-7ed3deffefe3",
    "Pascal Quidault",
    crate::card::CardRules::unsupported(),
);

// WOE 124 — Charming Scoundrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARMING_SCOUNDREL: CardRecord = CardRecord::new(
    "Charming Scoundrel",
    "c8090bcf-e17a-4110-a518-77ccd045b18f",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

// WOE 125 — Cut In
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUT_IN: CardRecord = CardRecord::new(
    "Cut In",
    "8ea4d40a-7657-4ff8-9fc2-915b99432275",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// WOE 126 — Edgewall Pack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EDGEWALL_PACK: CardRecord = CardRecord::new(
    "Edgewall Pack",
    "acda9d02-00fc-49e2-a9f3-176e9c0a8c5f",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// WOE 127 — Embereth Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERETH_VETERAN: CardRecord = CardRecord::new(
    "Embereth Veteran",
    "bc7130b8-3168-421f-912a-46ed5b769807",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// WOE 128 — Flick a Coin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLICK_A_COIN: CardRecord = CardRecord::new(
    "Flick a Coin",
    "673a67b2-fbb0-4be4-9edd-93946a583f23",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// WOE 129 — Food Fight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOD_FIGHT: CardRecord = CardRecord::new(
    "Food Fight",
    "1a7cc43c-6e8c-41d2-a885-24604dfc7e7f",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// WOE 130 — Frantic Firebolt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRANTIC_FIREBOLT: CardRecord = CardRecord::new(
    "Frantic Firebolt",
    "efd85f5a-258b-4ced-bf9e-3abe7fe72395",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// WOE 131 — Gnawing Crescendo
pub(in crate::card::sets) static GNAWING_CRESCENDO: CardRecord = CardRecord::new(
    "Gnawing Crescendo",
    "254fc64a-9734-44a6-8869-ab03512f1a99",
    "Alexey Kruglov",
    // The pump is what wins the combat; the watcher is what stops the
    // opponent from blocking profitably to answer it.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Creatures you control get +2/+0 until end of turn. Whenever a nontoken creature you \
         control dies this turn, create a 1/1 black Rat creature token with \"This token can't \
         block.\"",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            // A watcher installed for the rest of the turn rather than a
            // one-shot: every nontoken creature that dies makes its own Rat,
            // and the Rats it makes are excluded from feeding it.
            EffectDef::InstallTrigger(InstalledTriggerDef::this_turn(&AbilityDef::triggered(
                "Whenever a nontoken creature you control dies this turn, create a 1/1 black \
                 Rat creature token with \"This token can't block.\"",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    DEFENSELESS_RAT_TOKEN,
                ))),
            ))),
        ]),
    )),
);

// WOE 132 — Goddric, Cloaked Reveler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GODDRIC_CLOAKED_REVELER: CardRecord = CardRecord::new(
    "Goddric, Cloaked Reveler",
    "fe93ef82-51de-40ad-9b52-8f3fd11c144f",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// WOE 133 — Grabby Giant // That's Mine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRABBY_GIANT: CardRecord = CardRecord::new(
    "Grabby Giant // That's Mine",
    "fab7646a-61e8-446b-9dba-ac6e0db82f10",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// WOE 134 — Grand Ball Guest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAND_BALL_GUEST: CardRecord = CardRecord::new(
    "Grand Ball Guest",
    "d6e75228-16af-42b0-8441-ed253a660cc9",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// WOE 135 — Harried Spearguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARRIED_SPEARGUARD: CardRecord = CardRecord::new(
    "Harried Spearguard",
    "1db79785-4f55-445f-93f2-14c6e4606fc5",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// WOE 136 — Hearth Elemental // Stoke Genius
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTH_ELEMENTAL: CardRecord = CardRecord::new(
    "Hearth Elemental // Stoke Genius",
    "a8f5f102-cc75-4cee-a117-4bdaaf86c2e9",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// WOE 137 — Imodane, the Pyrohammer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMODANE_THE_PYROHAMMER: CardRecord = CardRecord::new(
    "Imodane, the Pyrohammer",
    "14b44833-0482-4b47-a594-4050bb87f1a5",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// WOE 138 — Kindled Heroism
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINDLED_HEROISM: CardRecord = CardRecord::new(
    "Kindled Heroism",
    "22a6d05e-e566-4a85-bcf8-9d0fbea3dd14",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// WOE 139 — Korvold and the Noble Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KORVOLD_AND_THE_NOBLE_THIEF: CardRecord = CardRecord::new(
    "Korvold and the Noble Thief",
    "a811b2cb-ffc7-4100-ac3e-bc4125842bb2",
    "Ben Hill",
    crate::card::CardRules::unsupported(),
);

// WOE 140 — Merry Bards
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERRY_BARDS: CardRecord = CardRecord::new(
    "Merry Bards",
    "b0058b9b-e919-45eb-9da0-690f62aa252e",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// WOE 141 — Minecart Daredevil // Ride the Rails
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINECART_DAREDEVIL: CardRecord = CardRecord::new(
    "Minecart Daredevil // Ride the Rails",
    "5b2a02f3-3921-4f40-9ffa-70bc08b052e1",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// WOE 142 — Monstrous Rage
pub(in crate::card::sets) static MONSTROUS_RAGE: CardRecord = CardRecord::new(
    "Monstrous Rage",
    "eef5a0ae-5907-42c9-a097-3f973737e392",
    "Borja Pindado",
    // One mana for three power and trample this turn, two of which stay
    // afterwards on the back of the Role.
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 until end of turn. Create a Monster Role token attached to \
         it. (If you control another Role on it, put that one into the graveyard. Enchanted \
         creature gets +1/+1 and has trample.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The pump is until end of turn and the Role is not: the +2/+0 lapses with
        // the turn and the +1/+1 stays for as long as the token does.
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateAttachedToken {
                // The Monster Role: an Aura token that is never cast, so it carries no
                // enchant clause of its own -- what it attaches to is decided by the effect
                // that creates it. Two Roles from one player on one creature is the older
                // one's problem, which the Role rule settles.
                token: TokenCharacteristics::enchantment(&["Aura", "Role"], &[])
                    // What a Role may be attached to. Held as a static because the token
                    // carries it by reference.
                    .enchanting(&ObjectPredicateDef::HasType(CardType::Creature))
                    .with_abilities(&[AbilityDef::static_ability(
                        "Enchanted creature gets +1/+1 and has trample.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::AttachedPermanent,
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(1),
                                ),
                                AppliedEffectDef::add_ability(&abilities::trample()),
                            ]),
                        },
                    )]),
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ]),
    )),
);

// WOE 143 — Raging Battle Mouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_BATTLE_MOUSE: CardRecord = CardRecord::new(
    "Raging Battle Mouse",
    "4d0ab162-540e-4999-902c-9dacd6687aca",
    "Rudy Siswanto",
    crate::card::CardRules::unsupported(),
);

// WOE 144 — Ratcatcher Trainee // Pest Problem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATCATCHER_TRAINEE: CardRecord = CardRecord::new(
    "Ratcatcher Trainee // Pest Problem",
    "7f4c0959-a107-4d61-9e51-256b2955f6ba",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// WOE 145 — Realm-Scorcher Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REALM_SCORCHER_HELLKITE: CardRecord = CardRecord::new(
    "Realm-Scorcher Hellkite",
    "845b3c26-05da-4a09-a6c9-4ea4166104a7",
    "Billy Christian",
    crate::card::CardRules::unsupported(),
);

// WOE 146 — Redcap Gutter-Dweller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDCAP_GUTTER_DWELLER: CardRecord = CardRecord::new(
    "Redcap Gutter-Dweller",
    "96bcd5f0-da79-47ab-83cf-976198b458d1",
    "Alexey Kruglov",
    crate::card::CardRules::unsupported(),
);

// WOE 147 — Redcap Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDCAP_THIEF: CardRecord = CardRecord::new(
    "Redcap Thief",
    "cda6bdeb-a0d6-46ca-ba8c-317ee0096416",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// WOE 148 — Rotisserie Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROTISSERIE_ELEMENTAL: CardRecord = CardRecord::new(
    "Rotisserie Elemental",
    "8d787045-3918-4ed6-85ea-843d1f2356f2",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// WOE 149 — Skewer Slinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKEWER_SLINGER: CardRecord = CardRecord::new(
    "Skewer Slinger",
    "e78e50ca-d27b-45db-91fa-7fef3cad16d0",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// WOE 150 — Song of Totentanz
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONG_OF_TOTENTANZ: CardRecord = CardRecord::new(
    "Song of Totentanz",
    "940e8bb7-0251-4fac-945c-d83618c10447",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// WOE 151 — Stonesplitter Bolt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONESPLITTER_BOLT: CardRecord = CardRecord::new(
    "Stonesplitter Bolt",
    "fb22f79c-3075-439d-a072-ceaabe35d76f",
    "Alexandr Leskinen",
    crate::card::CardRules::unsupported(),
);

// WOE 152 — Tattered Ratter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TATTERED_RATTER: CardRecord = CardRecord::new(
    "Tattered Ratter",
    "30f505b4-d61c-4da8-ab45-37125260d556",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// WOE 153 — Torch the Tower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORCH_THE_TOWER: CardRecord = CardRecord::new(
    "Torch the Tower",
    "b3d6027c-813f-46df-95b4-e2e305a67620",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// WOE 154 — Twisted Fealty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWISTED_FEALTY: CardRecord = CardRecord::new(
    "Twisted Fealty",
    "382d6085-79b9-48f7-8949-9f44dde2c753",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// WOE 155 — Two-Headed Hunter // Twice the Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWO_HEADED_HUNTER: CardRecord = CardRecord::new(
    "Two-Headed Hunter // Twice the Rage",
    "70c12e75-7e65-4706-b976-e47835910928",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// WOE 156 — Unruly Catapult
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNRULY_CATAPULT: CardRecord = CardRecord::new(
    "Unruly Catapult",
    "3dc275f8-f060-4c15-9b8f-63cf3857eaa7",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// WOE 157 — Virtue of Courage // Embereth Blaze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIRTUE_OF_COURAGE: CardRecord = CardRecord::new(
    "Virtue of Courage // Embereth Blaze",
    "8b0e6daf-0dec-4718-af79-b7ce137c3135",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 158 — Witch's Mark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITCH_S_MARK: CardRecord = CardRecord::new(
    "Witch's Mark",
    "0685afcb-06f6-4d18-b8c2-510764558dc1",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 159 — Witchstalker Frenzy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITCHSTALKER_FRENZY: CardRecord = CardRecord::new(
    "Witchstalker Frenzy",
    "649025a7-79d1-4d7c-b1db-d46bcf5a1ae2",
    "Pascal Quidault",
    crate::card::CardRules::unsupported(),
);

// WOE 160 — Agatha's Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGATHA_S_CHAMPION: CardRecord = CardRecord::new(
    "Agatha's Champion",
    "92652c41-1239-4299-a486-11fe1a96e912",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// WOE 161 — Beanstalk Wurm // Plant Beans
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEANSTALK_WURM: CardRecord = CardRecord::new(
    "Beanstalk Wurm // Plant Beans",
    "19f20c0a-22be-4a9c-96ce-4047f7a2d424",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// WOE 162 — Bestial Bloodline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BESTIAL_BLOODLINE: CardRecord = CardRecord::new(
    "Bestial Bloodline",
    "55b9b8e5-1ed8-4be0-aad5-041a599c6841",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// WOE 163 — Blossoming Tortoise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOSSOMING_TORTOISE: CardRecord = CardRecord::new(
    "Blossoming Tortoise",
    "7811a45d-6bfb-4c2a-b5a2-cccbd8cff186",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// WOE 164 — Bramble Familiar // Fetch Quest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAMBLE_FAMILIAR: CardRecord = CardRecord::new(
    "Bramble Familiar // Fetch Quest",
    "475d7e9a-759d-4523-a5cd-2a6e0d1b14ea",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// WOE 165 — Brave the Wilds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAVE_THE_WILDS: CardRecord = CardRecord::new(
    "Brave the Wilds",
    "821b9e86-d108-42e5-b642-c5e07ab16c37",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// WOE 166 — Commune with Nature (reprint)
const COMMUNE_WITH_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_chk::COMMUNE_WITH_NATURE,
    "5224eec3-2941-4d16-a713-099e34e93eee",
    "Jodie Muir",
);

// WOE 167 — Curse of the Werefox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURSE_OF_THE_WEREFOX: CardRecord = CardRecord::new(
    "Curse of the Werefox",
    "89148458-1fd6-48ef-a2d9-7b434c9723ec",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// WOE 168 — Elvish Archivist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_ARCHIVIST: CardRecord = CardRecord::new(
    "Elvish Archivist",
    "0670dbf0-b150-4e8d-bb40-f768b2f06fe5",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// WOE 169 — Feral Encounter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FERAL_ENCOUNTER: CardRecord = CardRecord::new(
    "Feral Encounter",
    "de21251f-40bf-4f7e-9e85-9033207a788f",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// WOE 170 — Ferocious Werefox // Guard Change
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEROCIOUS_WEREFOX: CardRecord = CardRecord::new(
    "Ferocious Werefox // Guard Change",
    "ac1907e8-0713-47dd-ac42-bf1323c5bec0",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

// WOE 171 — Graceful Takedown
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRACEFUL_TAKEDOWN: CardRecord = CardRecord::new(
    "Graceful Takedown",
    "83edf626-ed34-417f-818d-597ecf439167",
    "Sidharth Chaturvedi",
    crate::card::CardRules::unsupported(),
);

// WOE 172 — Gruff Triplets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRUFF_TRIPLETS: CardRecord = CardRecord::new(
    "Gruff Triplets",
    "f8760ab9-ac76-4e2e-b82f-0ee2a6dc5634",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// WOE 173 — Hamlet Glutton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAMLET_GLUTTON: CardRecord = CardRecord::new(
    "Hamlet Glutton",
    "a4ec5544-c138-44bb-a807-5798313c9a50",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// WOE 174 — Hollow Scavenger // Bakery Raid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLLOW_SCAVENGER: CardRecord = CardRecord::new(
    "Hollow Scavenger // Bakery Raid",
    "0ad345b6-7077-4dd2-b515-c774a3185fe4",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// WOE 175 — Howling Galefang
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOWLING_GALEFANG: CardRecord = CardRecord::new(
    "Howling Galefang",
    "86311523-d0eb-4db3-b586-8349de9c2d37",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// WOE 176 — The Huntsman's Redemption
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_HUNTSMAN_S_REDEMPTION: CardRecord = CardRecord::new(
    "The Huntsman's Redemption",
    "27003577-e276-4ad5-b3e9-8523b166ad49",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// WOE 177 — Leaping Ambush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEAPING_AMBUSH: CardRecord = CardRecord::new(
    "Leaping Ambush",
    "2785f716-274c-495c-b4e3-71a72e22f856",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// WOE 178 — Night of the Sweets' Revenge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHT_OF_THE_SWEETS_REVENGE: CardRecord = CardRecord::new(
    "Night of the Sweets' Revenge",
    "27f53bed-7075-4303-aa9e-fcca0a266e19",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// WOE 179 — Redtooth Genealogist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDTOOTH_GENEALOGIST: CardRecord = CardRecord::new(
    "Redtooth Genealogist",
    "99c81440-66eb-4443-a83f-e2f15cb68a3e",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// WOE 180 — Redtooth Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDTOOTH_VANGUARD: CardRecord = CardRecord::new(
    "Redtooth Vanguard",
    "55271960-b9bd-4bea-93ca-3321bf30be78",
    "Joshua Cairos",
    crate::card::CardRules::unsupported(),
);

// WOE 181 — Return from the Wilds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETURN_FROM_THE_WILDS: CardRecord = CardRecord::new(
    "Return from the Wilds",
    "9597d9ea-d9b2-4009-8e7c-02caa3585bc5",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// WOE 182 — Rootrider Faun
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTRIDER_FAUN: CardRecord = CardRecord::new(
    "Rootrider Faun",
    "4e87db5a-1a70-42df-83a2-c0f71fe8533a",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// WOE 183 — Royal Treatment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROYAL_TREATMENT: CardRecord = CardRecord::new(
    "Royal Treatment",
    "b6516b8f-ecfb-401e-ba8e-bf561aa2be64",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// WOE 184 — Sentinel of Lost Lore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SENTINEL_OF_LOST_LORE: CardRecord = CardRecord::new(
    "Sentinel of Lost Lore",
    "f109a5bf-1472-4b87-b3d3-70db0e123693",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// WOE 185 — Skybeast Tracker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYBEAST_TRACKER: CardRecord = CardRecord::new(
    "Skybeast Tracker",
    "a08da5c6-ebe7-4166-99d5-2aca5b0b529f",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// WOE 186 — Spider Food
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_FOOD: CardRecord = CardRecord::new(
    "Spider Food",
    "b9fd720b-e9c2-4e82-917e-bab6c544afb0",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// WOE 187 — Stormkeld Vanguard // Bear Down
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMKELD_VANGUARD: CardRecord = CardRecord::new(
    "Stormkeld Vanguard // Bear Down",
    "bacb1fe5-0adf-461f-b698-9d09a8728c63",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// WOE 188 — Tanglespan Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLESPAN_LOOKOUT: CardRecord = CardRecord::new(
    "Tanglespan Lookout",
    "3bc5c32d-be0a-4a5f-a8c7-9767a895bc76",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// WOE 189 — Territorial Witchstalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRITORIAL_WITCHSTALKER: CardRecord = CardRecord::new(
    "Territorial Witchstalker",
    "2515d53d-7a50-4da3-980d-91d91fea2020",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// WOE 190 — Thunderous Debut
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDEROUS_DEBUT: CardRecord = CardRecord::new(
    "Thunderous Debut",
    "d98f51ec-8eae-434a-ab62-85bdb6586fa2",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// WOE 191 — Titanic Growth (reprint)
const TITANIC_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m12::TITANIC_GROWTH,
    "46917de3-5e98-4dd6-8950-fc10338515df",
    "Iris Compiet",
);

// WOE 192 — Toadstool Admirer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOADSTOOL_ADMIRER: CardRecord = CardRecord::new(
    "Toadstool Admirer",
    "f0a6349a-beff-4abd-b005-86d27c1e2957",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// WOE 193 — Tough Cookie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOUGH_COOKIE: CardRecord = CardRecord::new(
    "Tough Cookie",
    "7aa37f85-5336-4372-97a2-9c8b00798c7a",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// WOE 194 — Troublemaker Ouphe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROUBLEMAKER_OUPHE: CardRecord = CardRecord::new(
    "Troublemaker Ouphe",
    "7f7b2fc0-d3f6-4c4d-a163-986a372e5b12",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// WOE 195 — Up the Beanstalk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UP_THE_BEANSTALK: CardRecord = CardRecord::new(
    "Up the Beanstalk",
    "2d5e991f-23b2-4db0-a452-7755125b1fd2",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// WOE 196 — Verdant Outrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDANT_OUTRIDER: CardRecord = CardRecord::new(
    "Verdant Outrider",
    "c34830e4-823a-40fa-ba41-bb2afbf1e499",
    "Taras Susak",
    crate::card::CardRules::unsupported(),
);

// WOE 197 — Virtue of Strength // Garenbrig Growth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIRTUE_OF_STRENGTH: CardRecord = CardRecord::new(
    "Virtue of Strength // Garenbrig Growth",
    "ff857d41-767d-4e99-83cc-444738341b92",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 198 — Welcome to Sweettooth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELCOME_TO_SWEETTOOTH: CardRecord = CardRecord::new(
    "Welcome to Sweettooth",
    "8e629a31-2e06-4f95-9628-34670dcf68b9",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// WOE 199 — Agatha of the Vile Cauldron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGATHA_OF_THE_VILE_CAULDRON: CardRecord = CardRecord::new(
    "Agatha of the Vile Cauldron",
    "d6c48f07-63b7-4a60-8da6-ce77405abf1e",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// WOE 200 — The Apprentice's Folly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_APPRENTICE_S_FOLLY: CardRecord = CardRecord::new(
    "The Apprentice's Folly",
    "0edb58bb-8ff3-4e34-b3d1-d83b5bd8c178",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// WOE 201 — Ash, Party Crasher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASH_PARTY_CRASHER: CardRecord = CardRecord::new(
    "Ash, Party Crasher",
    "d51e6610-25b6-4d8e-92d7-c50a2ff844ff",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// WOE 202 — Eriette of the Charmed Apple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERIETTE_OF_THE_CHARMED_APPLE: CardRecord = CardRecord::new(
    "Eriette of the Charmed Apple",
    "ecead4cd-47ae-4c42-b15c-1b29b5caba18",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// WOE 203 — Faunsbane Troll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAUNSBANE_TROLL: CardRecord = CardRecord::new(
    "Faunsbane Troll",
    "2d8bd585-c5ea-46f8-8e11-f33c067f2f8e",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// WOE 204 — The Goose Mother
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_GOOSE_MOTHER: CardRecord = CardRecord::new(
    "The Goose Mother",
    "1a55c370-d396-4c73-8ee2-83dc4c124005",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// WOE 205 — Greta, Sweettooth Scourge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRETA_SWEETTOOTH_SCOURGE: CardRecord = CardRecord::new(
    "Greta, Sweettooth Scourge",
    "2cfd365e-34d1-4224-b925-119000311934",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// WOE 206 — Hylda of the Icy Crown
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYLDA_OF_THE_ICY_CROWN: CardRecord = CardRecord::new(
    "Hylda of the Icy Crown",
    "ae9231fd-053d-4b84-a7a8-86063465bc49",
    "Ekaterina Burmak",
    crate::card::CardRules::unsupported(),
);

// WOE 207 — Johann, Apprentice Sorcerer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOHANN_APPRENTICE_SORCERER: CardRecord = CardRecord::new(
    "Johann, Apprentice Sorcerer",
    "b88a762d-19ed-451d-a3a9-b3e7eea40f67",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// WOE 208 — Likeness Looter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIKENESS_LOOTER: CardRecord = CardRecord::new(
    "Likeness Looter",
    "2957472a-825e-4904-b7e8-62bef1cb432d",
    "Ben Hill",
    crate::card::CardRules::unsupported(),
);

// WOE 209 — Neva, Stalked by Nightmares
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEVA_STALKED_BY_NIGHTMARES: CardRecord = CardRecord::new(
    "Neva, Stalked by Nightmares",
    "e6f925ab-ade7-4da8-a791-185e098d18f2",
    "Tyler Jacobson",
    crate::card::CardRules::unsupported(),
);

// WOE 210 — Obyra, Dreaming Duelist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBYRA_DREAMING_DUELIST: CardRecord = CardRecord::new(
    "Obyra, Dreaming Duelist",
    "63ce03ee-279b-4955-98e3-9ce1990f8b7b",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// WOE 211 — Rowan, Scion of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROWAN_SCION_OF_WAR: CardRecord = CardRecord::new(
    "Rowan, Scion of War",
    "4ee179ab-a15b-4bd6-b7f8-1e1abeeb31b7",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// WOE 212 — Ruby, Daring Tracker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUBY_DARING_TRACKER: CardRecord = CardRecord::new(
    "Ruby, Daring Tracker",
    "ffb5786b-6825-4ebf-a1e1-80011340adbb",
    "Ekaterina Burmak",
    crate::card::CardRules::unsupported(),
);

// WOE 213 — Sharae of Numbing Depths
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHARAE_OF_NUMBING_DEPTHS: CardRecord = CardRecord::new(
    "Sharae of Numbing Depths",
    "600bc36a-3ef0-459c-9a93-94ec45b8c3d9",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// WOE 214 — Syr Armont, the Redeemer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYR_ARMONT_THE_REDEEMER: CardRecord = CardRecord::new(
    "Syr Armont, the Redeemer",
    "85050609-baf0-430a-ab33-83a6ea6d4741",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// WOE 215 — Talion, the Kindly Lord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TALION_THE_KINDLY_LORD: CardRecord = CardRecord::new(
    "Talion, the Kindly Lord",
    "62a6b452-c796-45c6-b4d1-0ae3d675e38e",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 216 — Totentanz, Swarm Piper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOTENTANZ_SWARM_PIPER: CardRecord = CardRecord::new(
    "Totentanz, Swarm Piper",
    "1422d6db-fe5b-4a89-951a-fbd7985a29fc",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// WOE 217 — Troyan, Gutsy Explorer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROYAN_GUTSY_EXPLORER: CardRecord = CardRecord::new(
    "Troyan, Gutsy Explorer",
    "eadd2a1a-93a0-4257-897d-1aaee279449f",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// WOE 218 — Will, Scion of Peace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILL_SCION_OF_PEACE: CardRecord = CardRecord::new(
    "Will, Scion of Peace",
    "162088ea-5f99-4244-9427-2fdfb2168fc3",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// WOE 219 — Yenna, Redtooth Regent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YENNA_REDTOOTH_REGENT: CardRecord = CardRecord::new(
    "Yenna, Redtooth Regent",
    "e635d461-254a-434e-8e5d-dea61dd8ca4f",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 220 — Beluna Grandsquall // Seek Thrills
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELUNA_GRANDSQUALL: CardRecord = CardRecord::new(
    "Beluna Grandsquall // Seek Thrills",
    "3f5acc0d-33a6-476f-95ca-a1ad788334dd",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// WOE 221 — Callous Sell-Sword // Burn Together
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLOUS_SELL_SWORD: CardRecord = CardRecord::new(
    "Callous Sell-Sword // Burn Together",
    "770ee3da-d33e-466f-9a2e-ad2d08ef5012",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// WOE 222 — Cruel Somnophage // Can't Wake Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUEL_SOMNOPHAGE: CardRecord = CardRecord::new(
    "Cruel Somnophage // Can't Wake Up",
    "39b11ff0-9946-4337-86fb-42e967f3d2e4",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// WOE 223 — Decadent Dragon // Expensive Taste
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECADENT_DRAGON: CardRecord = CardRecord::new(
    "Decadent Dragon // Expensive Taste",
    "315cbbf7-a2ad-4565-9877-1e903d7fd797",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// WOE 224 — Devouring Sugarmaw // Have for Dinner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOURING_SUGARMAW: CardRecord = CardRecord::new(
    "Devouring Sugarmaw // Have for Dinner",
    "58c7f52e-a97d-4475-ae00-3149991e723e",
    "Nino Vecia",
    crate::card::CardRules::unsupported(),
);

// WOE 225 — Elusive Otter // Grove's Bounty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELUSIVE_OTTER: CardRecord = CardRecord::new(
    "Elusive Otter // Grove's Bounty",
    "bc9bdf96-3e3b-4dca-aae2-e81d4cbeafe8",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// WOE 226 — Frolicking Familiar // Blow Off Steam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROLICKING_FAMILIAR: CardRecord = CardRecord::new(
    "Frolicking Familiar // Blow Off Steam",
    "64c432d5-4f5b-44ac-9d61-891e78460d58",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// WOE 227 — Gingerbread Hunter // Puny Snack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GINGERBREAD_HUNTER: CardRecord = CardRecord::new(
    "Gingerbread Hunter // Puny Snack",
    "e77a8fd4-af5f-42b3-a87e-788baf2562dd",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// WOE 228 — Heartflame Duelist // Heartflame Slash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTFLAME_DUELIST: CardRecord = CardRecord::new(
    "Heartflame Duelist // Heartflame Slash",
    "811b283f-22f3-47b1-a802-11dc8c25d0ee",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 229 — Imodane's Recruiter // Train Troops
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMODANE_S_RECRUITER: CardRecord = CardRecord::new(
    "Imodane's Recruiter // Train Troops",
    "4dbaa855-3f8e-42e6-8ec8-5ffbc5c8acf0",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// WOE 230 — Kellan, the Fae-Blooded // Birthright Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELLAN_THE_FAE_BLOODED: CardRecord = CardRecord::new(
    "Kellan, the Fae-Blooded // Birthright Boon",
    "ec5e2680-8b42-4571-ab45-4936aec51901",
    "Anna Steinbauer",
    crate::card::CardRules::unsupported(),
);

// WOE 231 — Mosswood Dreadknight // Dread Whispers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOSSWOOD_DREADKNIGHT: CardRecord = CardRecord::new(
    "Mosswood Dreadknight // Dread Whispers",
    "9869ac70-5907-45fa-952c-31aef70c5066",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// WOE 232 — Picnic Ruiner // Stolen Goodies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PICNIC_RUINER: CardRecord = CardRecord::new(
    "Picnic Ruiner // Stolen Goodies",
    "66485c3e-3b21-4db4-ac12-af04e35b49b1",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// WOE 233 — Pollen-Shield Hare // Hare Raising
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLLEN_SHIELD_HARE: CardRecord = CardRecord::new(
    "Pollen-Shield Hare // Hare Raising",
    "139f31f5-28f0-4e90-abdf-3f6ed0992dea",
    "Olena Richards",
    crate::card::CardRules::unsupported(),
);

// WOE 234 — Questing Druid // Seek the Beast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUESTING_DRUID: CardRecord = CardRecord::new(
    "Questing Druid // Seek the Beast",
    "72c130e2-1e17-4996-a5ae-231155d68261",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// WOE 235 — Scalding Viper // Steam Clean
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALDING_VIPER: CardRecord = CardRecord::new(
    "Scalding Viper // Steam Clean",
    "58e72bfb-6f64-4647-afb6-b5ad4737121c",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// WOE 236 — Shrouded Shepherd // Cleave Shadows
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHROUDED_SHEPHERD: CardRecord = CardRecord::new(
    "Shrouded Shepherd // Cleave Shadows",
    "ab03c342-2bf4-41bf-8bb8-472d978d238a",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// WOE 237 — Spellscorn Coven // Take It Back
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLSCORN_COVEN: CardRecord = CardRecord::new(
    "Spellscorn Coven // Take It Back",
    "8c112f62-6034-4636-a75b-4a45bc916a91",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// WOE 238 — Tempest Hart // Scan the Clouds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPEST_HART: CardRecord = CardRecord::new(
    "Tempest Hart // Scan the Clouds",
    "559bacc8-facc-4d93-90b5-8ac21d3246f5",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// WOE 239 — Threadbind Clique // Rip the Seams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREADBIND_CLIQUE: CardRecord = CardRecord::new(
    "Threadbind Clique // Rip the Seams",
    "dd6ed252-c262-4062-97ba-75c50d6b5579",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// WOE 240 — Twining Twins // Swift Spiral
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWINING_TWINS: CardRecord = CardRecord::new(
    "Twining Twins // Swift Spiral",
    "043718ea-59f6-4d1a-94c5-271704c1a38a",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// WOE 241 — Woodland Acolyte // Mend the Wilds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOODLAND_ACOLYTE: CardRecord = CardRecord::new(
    "Woodland Acolyte // Mend the Wilds",
    "b9f10623-4783-4773-b9c8-a5a2bcfdb5d9",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// WOE 242 — Agatha's Soul Cauldron
pub(in crate::card::sets) static AGATHAS_SOUL_CAULDRON: CardRecord = CardRecord::new(
    "Agatha's Soul Cauldron",
    "019b51b0-e5c6-4208-922b-7736686dddcd",
    "Jason A. Engle",
CardRules::new_artifact(mana_cost!("{2}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may spend mana as though it were mana of any color to activate abilities of \
                 creatures you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities,
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Creatures you control with +1/+1 counters on them have all activated abilities of all \
                 creature cards exiled with Agatha's Soul Cauldron.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        // The Cauldron hands its abilities to creatures that are carrying a counter,
                        // whoever put it there. Read every time the layer is walked, so a creature
                        // that loses its last counter loses the abilities with it.
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                        AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ),
                    )),
                },
            ),
            AbilityDef::activated_with_targets(
                "{T}: Exile target card from a graveyard. When a creature card is exiled this way, put a \
                 +1/+1 counter on target creature you control.",
                &[CostDef::TapSource],
                // "Target card from a graveyard" reaches every graveyard, not only its
                // controller's.
                &[
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    }),
                    // The counter's target belongs to a reflexive trigger, which this engine
                    // declares up front alongside the activation's own target. "Up to one"
                    // rather than "one" is what keeps the activation legal for a player who
                    // controls no creature, which the printed card allows: the reflexive
                    // trigger simply never gets a target.
                    AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                        1,
                    ),
                ],
                EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        face_down: false,
                        then: None,
                    },
                    EffectDef::IfCondition {
                        // "When a creature card is exiled this way": asked of the card the
                        // activation named, which by then has already moved to exile.
                        condition: &TriggerConditionDef::TargetMatches {
                            slot: TargetIndex::PRIMARY,
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                        },
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex(1)),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    },
                ]),
            ),
        ]),
);

// WOE 243 — Candy Trail
pub(in crate::card::sets) static CANDY_TRAIL: CardRecord = CardRecord::new(
    "Candy Trail",
    "1a860925-d912-49e5-9ddc-41ab26916bb3",
    "Alix Branwyn",
    // A one-mana artifact that smooths the draw now and replaces itself
    // later, which is what makes it a fine card in a deck that just wants
    // its land drops.
    CardRules::new_artifact(mana_cost!("{1}"))
        // Food and Clue are printed types here rather than granted rules:
        // the sacrifice ability this card wants is its own, not either
        // token's.
        .with_subtypes(&["Food", "Clue"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this artifact enters, scry 2.",
                abilities::scry(ValueDef::Constant(2)),
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this artifact: You gain 3 life and draw a card.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// WOE 244 — Collector's Vault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLECTOR_S_VAULT: CardRecord = CardRecord::new(
    "Collector's Vault",
    "967a4f27-8cd6-437d-8c05-8aedbc7ddc4b",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// WOE 245 — Eriette's Tempting Apple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERIETTE_S_TEMPTING_APPLE: CardRecord = CardRecord::new(
    "Eriette's Tempting Apple",
    "5b61711f-8982-4ff0-86ba-01e0125cd705",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// WOE 246 — Gingerbrute (reprint)
const GINGERBRUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::GINGERBRUTE,
    "09a4578a-7dc6-4da3-93ee-913b10be5740",
    "Carlos Palma Cruchaga",
);

// WOE 247 — Hylda's Crown of Winter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYLDA_S_CROWN_OF_WINTER: CardRecord = CardRecord::new(
    "Hylda's Crown of Winter",
    "b0d4a6c0-f00e-45a7-9c88-899460007020",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// WOE 248 — The Irencrag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_IRENCRAG: CardRecord = CardRecord::new(
    "The Irencrag",
    "8051c5ec-54a6-45a8-8945-fb93c5feaa39",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// WOE 249 — Prophetic Prism (reprint)
const PROPHETIC_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::PROPHETIC_PRISM,
    "1fae351c-b918-4648-a361-d5239ae63156",
    "Quintin Gleim",
);

// WOE 250 — Scarecrow Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARECROW_GUIDE: CardRecord = CardRecord::new(
    "Scarecrow Guide",
    "3dc08461-bec6-4a18-b782-da4c92abb789",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// WOE 251 — Soul-Guide Lantern (reprint)
const SOUL_GUIDE_LANTERN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_thb::SOUL_GUIDE_LANTERN,
    "13571173-29e7-4915-af5f-05f13b463061",
    "Iris Compiet",
);

// WOE 252 — Syr Ginger, the Meal Ender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYR_GINGER_THE_MEAL_ENDER: CardRecord = CardRecord::new(
    "Syr Ginger, the Meal Ender",
    "7fbdba12-1369-41ae-b0e9-c405c0f0a2e5",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// WOE 253 — Three Bowls of Porridge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_BOWLS_OF_PORRIDGE: CardRecord = CardRecord::new(
    "Three Bowls of Porridge",
    "a508e040-d1e5-46aa-8404-1adc18f0f8bd",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// WOE 254 — Crystal Grotto (reprint)
const CRYSTAL_GROTTO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::CRYSTAL_GROTTO,
    "6f6f9d3d-600d-43f3-a915-612e5d53aaa1",
    "Andreas Zafiratos",
);

// WOE 255 — Edgewall Inn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EDGEWALL_INN: CardRecord = CardRecord::new(
    "Edgewall Inn",
    "ec435e54-628a-43bd-8804-cbc37e375bce",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// WOE 256 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::EVOLVING_WILDS,
    "74f9c819-719a-461b-8e7e-a26c88e8099b",
    "Alayna Danner",
);

// WOE 257 — Restless Bivouac
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTLESS_BIVOUAC: CardRecord = CardRecord::new(
    "Restless Bivouac",
    "b85e0aed-bfb2-4aa8-a754-849c4d9a6a58",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// WOE 258 — Restless Cottage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTLESS_COTTAGE: CardRecord = CardRecord::new(
    "Restless Cottage",
    "787eadf3-5005-4ae5-820f-4012a4d4e1a5",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// WOE 259 — Restless Fortress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTLESS_FORTRESS: CardRecord = CardRecord::new(
    "Restless Fortress",
    "675213bb-28d7-460c-a4f3-950f5b9090af",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// WOE 260 — Restless Spire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTLESS_SPIRE: CardRecord = CardRecord::new(
    "Restless Spire",
    "66386fe8-9d3c-47f7-9cd3-4cd30051535f",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// WOE 261 — Restless Vinestalk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTLESS_VINESTALK: CardRecord = CardRecord::new(
    "Restless Vinestalk",
    "e5f3161d-3f69-4b06-ab73-c31fc0c1520c",
    "Sam Burley",
    crate::card::CardRules::unsupported(),
);

// WOE 262 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "c9cd4d57-8c51-4fcf-8a9f-5d6a61c33e3d",
    "Hari & Deepti",
);

// WOE 263 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "bd4b4da4-83f6-4280-880b-b6033308f2a2",
    "Hari & Deepti",
);

// WOE 264 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "ee68f2cb-851b-4196-ac58-844d72628e6a",
    "Hari & Deepti",
);

// WOE 265 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "8822db23-34dc-452a-92bc-a3ceee4db375",
    "Hari & Deepti",
);

// WOE 266 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "ecd6d8fb-780c-446c-a8bf-93386b22fe95",
    "Hari & Deepti",
);

// WOE 267 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "eed1af19-e075-4e6f-9394-d258143e15a4",
    "Carlos Palma Cruchaga",
);

// WOE 268 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "486fbcf9-3a04-47f6-8927-886c2a454499",
    "Jonas De Ro",
);

// WOE 269 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "6245d2f8-8ef0-4d2a-9abb-99839dc3abf0",
    "Leanna Crossan",
);

// WOE 270 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "1a9798d6-34b3-4438-992d-d3616a7c8536",
    "Sarah Finnigan",
);

// WOE 271 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "d0a801ba-ebf7-4b9e-98c2-db50448845b7",
    "Jonas De Ro",
);

// WOE 272 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "f0de4ea5-77e6-474e-99f9-36192bbd37d5",
    "Julian Kok Joon Wen",
);

// WOE 273 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "8996cd43-5ecd-4a05-a5a9-e49326befaa1",
    "Sarah Finnigan",
);

// WOE 274 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "8e747bba-b521-4f81-9d9a-3e85747cefe9",
    "Julian Kok Joon Wen",
);

// WOE 275 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "a4c99f1b-f304-42d5-bbea-32283b01d43b",
    "Jonas De Ro",
);

// WOE 276 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "1bd51a22-3e0d-4826-aab7-0adbfce4478a",
    "Adam Paquette",
);

// WOE 277 — Virtue of Loyalty
/// "Those creatures" is the same set the clause just counted: nothing joins
/// or leaves the battlefield while one effect resolves, so asking twice and
/// binding the first answer come to the same thing.
static YOUR_CREATURES: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

const fn virtue_of_loyalty_rules() -> CardRules {
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::triggered(
        "At the beginning of your end step, put a +1/+1 counter on each creature you control. \
         Untap those creatures.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::AddCounters {
                        object: YOUR_CREATURES,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Untap {
                        object: YOUR_CREATURES,
                    },
                ]
            },
        ),
    ))
}

fn virtue_of_loyalty_composition() -> CardComposition {
    let virtue = virtue_of_loyalty_rules();
    let fealty = const {
        CardRules::new_instant(mana_cost!("{1}{W}"))
            .with_subtypes(&["Adventure"])
            .with_ability(
                AbilityDef::spell(
                    "Create a 2/2 white Knight creature token with vigilance.",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Knight"], &[ManaColor::White], 2, 2)
                            .with_abilities(&const { [abilities::vigilance()] }),
                    ))),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Virtue of Loyalty", virtue),
            CardPart::new(CardPartId(1), "Ardenvale Fealty", fealty),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Virtue of Loyalty",
                SpellForm::Part(CardPartId::PRIMARY),
                virtue
                    .mana_cost()
                    .expect("the enchantment has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Ardenvale Fealty",
                SpellForm::Part(CardPartId(1)),
                fealty
                    .mana_cost()
                    .expect("the Adventure has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static VIRTUE_OF_LOYALTY: CardRecord = CardRecord::new(
    "Virtue of Loyalty",
    "9622e597-dc7c-4198-9ce5-4df53bb0c96c",
    "Keith Garletts",
    virtue_of_loyalty_rules(),
)
.with_composition(virtue_of_loyalty_composition);

// WOE 278 — Horned Loch-Whale // Lagoon Breach (alternate printing)
const HORNED_LOCH_WHALE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HORNED_LOCH_WHALE,
    1,
    "af969428-8dd2-47d6-bdbf-65eca632c3d4",
    "Nils Hamm",
);

// WOE 279 — Virtue of Knowledge // Vantress Visions (alternate printing)
const VIRTUE_OF_KNOWLEDGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_KNOWLEDGE,
    1,
    "7cc2173a-b7fb-4bd6-9c8e-73de91c6903a",
    "Josu Hernaiz",
);

// WOE 280 — Gumdrop Poisoner // Tempt with Treats (alternate printing)
const GUMDROP_POISONER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUMDROP_POISONER,
    1,
    "ec5d0453-ef17-47f0-81d4-13941f1380d5",
    "Allen Williams",
);

// WOE 281 — Virtue of Persistence // Locthwain Scorn (alternate printing)
const VIRTUE_OF_PERSISTENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_PERSISTENCE,
    1,
    "337393e3-9081-4251-b4ca-afd7ce10dc99",
    "Allen Williams",
);

// WOE 282 — Virtue of Courage // Embereth Blaze (alternate printing)
const VIRTUE_OF_COURAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_COURAGE,
    1,
    "ed585964-68ed-4889-8d20-678b43c46018",
    "Nils Hamm",
);

// WOE 283 — Bramble Familiar // Fetch Quest (alternate printing)
const BRAMBLE_FAMILIAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRAMBLE_FAMILIAR,
    1,
    "1ee74605-63be-41cd-a6ba-1b33f8094ec9",
    "Alayna Danner",
);

// WOE 284 — Virtue of Strength // Garenbrig Growth (alternate printing)
const VIRTUE_OF_STRENGTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_STRENGTH,
    1,
    "684f8568-390f-426f-ba71-e4be5fdaceee",
    "Danny Schwartz",
);

// WOE 285 — Beluna Grandsquall // Seek Thrills (alternate printing)
const BELUNA_GRANDSQUALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BELUNA_GRANDSQUALL,
    1,
    "5ed9d78f-a556-435f-b95f-7317ae66e5e3",
    "Kev Walker",
);

// WOE 286 — Cruel Somnophage // Can't Wake Up (alternate printing)
const CRUEL_SOMNOPHAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRUEL_SOMNOPHAGE,
    1,
    "852d840c-c9b5-4486-b1f5-95ea88577fa0",
    "Kev Walker",
);

// WOE 287 — Decadent Dragon // Expensive Taste (alternate printing)
const DECADENT_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DECADENT_DRAGON,
    1,
    "8a717d27-596d-4341-b592-4f9777f778e5",
    "Aaron Miller",
);

// WOE 288 — Devouring Sugarmaw // Have for Dinner (alternate printing)
const DEVOURING_SUGARMAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEVOURING_SUGARMAW,
    1,
    "f2848594-1718-43f2-8ccf-34a18a55e4b2",
    "Kev Walker",
);

// WOE 289 — Elusive Otter // Grove's Bounty (alternate printing)
const ELUSIVE_OTTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELUSIVE_OTTER,
    1,
    "c5a61619-f951-42ff-8246-c51ee5dc18c8",
    "Andrea Sipl",
);

// WOE 290 — Heartflame Duelist // Heartflame Slash (alternate printing)
const HEARTFLAME_DUELIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEARTFLAME_DUELIST,
    1,
    "f54b25e1-193a-41fb-9d24-3147cb381dbe",
    "Allen Williams",
);

// WOE 291 — Kellan, the Fae-Blooded // Birthright Boon (alternate printing)
const KELLAN_THE_FAE_BLOODED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_THE_FAE_BLOODED,
    1,
    "e662dfa5-edf8-4e7b-8a29-86935e95ac35",
    "Omar Rayyan",
);

// WOE 292 — Mosswood Dreadknight // Dread Whispers (alternate printing)
const MOSSWOOD_DREADKNIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOSSWOOD_DREADKNIGHT,
    1,
    "2ab293c3-64cf-4d8a-909d-4cf73ffd828a",
    "Keith Garletts",
);

// WOE 293 — Pollen-Shield Hare // Hare Raising (alternate printing)
const POLLEN_SHIELD_HARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POLLEN_SHIELD_HARE,
    1,
    "f2d517ad-6df7-4cf9-982f-763379724d24",
    "Greg Hildebrandt",
);

// WOE 294 — Questing Druid // Seek the Beast (alternate printing)
const QUESTING_DRUID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUESTING_DRUID,
    1,
    "c6406eba-da58-4264-a213-20e22c1c3bec",
    "Greg Hildebrandt",
);

// WOE 295 — Scalding Viper // Steam Clean (alternate printing)
const SCALDING_VIPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCALDING_VIPER,
    1,
    "3014de04-66ce-4641-b36f-6ff1cc1c116a",
    "Omar Rayyan",
);

// WOE 296 — Twining Twins // Swift Spiral (alternate printing)
const TWINING_TWINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWINING_TWINS,
    1,
    "38708b07-df7d-4ea4-b10e-ead7cd03d849",
    "Alayna Danner",
);

// WOE 297 — Ashiok, Wicked Manipulator (alternate printing)
const ASHIOK_WICKED_MANIPULATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHIOK_WICKED_MANIPULATOR,
    1,
    "dd6f57de-21af-4c25-8a21-df3a75157939",
    "Serena Malyon",
);

// WOE 298 — Kellan, the Fae-Blooded // Birthright Boon (alternate printing)
const KELLAN_THE_FAE_BLOODED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_THE_FAE_BLOODED,
    2,
    "aa5b622a-66a6-4097-8478-e22854b9984d",
    "Andreas Zafiratos",
);

// WOE 299 — Eriette of the Charmed Apple (alternate printing)
const ERIETTE_OF_THE_CHARMED_APPLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ERIETTE_OF_THE_CHARMED_APPLE,
    1,
    "487a2cc0-ddbf-46b9-90ed-9455347724d5",
    "Zach Alexander",
);

// WOE 300 — Rowan, Scion of War (alternate printing)
const ROWAN_SCION_OF_WAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROWAN_SCION_OF_WAR,
    1,
    "844e29e9-e870-4433-ad63-0ff918a3e41a",
    "Abigail Larson",
);

// WOE 301 — Talion, the Kindly Lord (alternate printing)
const TALION_THE_KINDLY_LORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TALION_THE_KINDLY_LORD,
    1,
    "bb1f05ef-7607-4a63-a710-18d0061ec982",
    "Olena Richards",
);

// WOE 302 — Will, Scion of Peace (alternate printing)
const WILL_SCION_OF_PEACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILL_SCION_OF_PEACE,
    1,
    "0f25e3db-19ae-4f89-80ec-bf0ad561be39",
    "Matteo Marjoram",
);

// WOE 303 — Restless Bivouac (alternate printing)
const RESTLESS_BIVOUAC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_BIVOUAC,
    1,
    "901796b2-c567-41e2-a87b-6147a966c9a8",
    "Lucas Graciano",
);

// WOE 304 — Restless Cottage (alternate printing)
const RESTLESS_COTTAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_COTTAGE,
    1,
    "0dcb97ff-9356-4be7-8991-091f950c0b63",
    "Sergey Glushakov",
);

// WOE 305 — Restless Fortress (alternate printing)
const RESTLESS_FORTRESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_FORTRESS,
    1,
    "8034589b-3b57-49be-b0fd-a306f915d864",
    "Alayna Danner",
);

// WOE 306 — Restless Spire (alternate printing)
const RESTLESS_SPIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_SPIRE,
    1,
    "dc42bb4f-475e-4c76-bcbd-a9d8cf03117c",
    "Sergey Glushakov",
);

// WOE 307 — Restless Vinestalk (alternate printing)
const RESTLESS_VINESTALK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_VINESTALK,
    1,
    "a710643a-b9dd-49bf-a375-d9986b05ed7a",
    "Sergey Glushakov",
);

// WOE 308 — Food Coma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOD_COMA: CardRecord = CardRecord::new(
    "Food Coma",
    "61ba2aed-3514-4db9-8da0-329620b12f63",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// WOE 309 — Lady of Laughter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LADY_OF_LAUGHTER: CardRecord = CardRecord::new(
    "Lady of Laughter",
    "c26714f9-d42f-4bac-b185-8943d1621444",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// WOE 310 — Pests of Honor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PESTS_OF_HONOR: CardRecord = CardRecord::new(
    "Pests of Honor",
    "5671c2a0-51e8-4d5e-8409-87abd6c0a8ab",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// WOE 311 — Faerie Slumber Party
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_SLUMBER_PARTY: CardRecord = CardRecord::new(
    "Faerie Slumber Party",
    "f8e5de58-cc4c-40b2-94c8-4e4d7cebfaf8",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// WOE 312 — Rowdy Research
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROWDY_RESEARCH: CardRecord = CardRecord::new(
    "Rowdy Research",
    "89b7e901-5ba4-4374-9eeb-96354279a123",
    "Bram Sels",
    crate::card::CardRules::unsupported(),
);

// WOE 313 — Storyteller Pixie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORYTELLER_PIXIE: CardRecord = CardRecord::new(
    "Storyteller Pixie",
    "b9f4cf85-2120-4897-aecd-4ee4b02d6f9b",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// WOE 314 — Experimental Confectioner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPERIMENTAL_CONFECTIONER: CardRecord = CardRecord::new(
    "Experimental Confectioner",
    "49aaf5db-9418-4b97-97da-736c674905d4",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// WOE 315 — Malevolent Witchkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALEVOLENT_WITCHKITE: CardRecord = CardRecord::new(
    "Malevolent Witchkite",
    "d6cb3c6d-560d-40ca-b5c0-27322863cead",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// WOE 316 — Old Flitterfang
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OLD_FLITTERFANG: CardRecord = CardRecord::new(
    "Old Flitterfang",
    "67c77d6f-de14-423c-bf55-0fb289171004",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// WOE 317 — Become Brutes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BECOME_BRUTES: CardRecord = CardRecord::new(
    "Become Brutes",
    "a154dadc-be5b-4ad6-9946-bdc54c251bff",
    "Julia Griffin",
    crate::card::CardRules::unsupported(),
);

// WOE 318 — Charging Hooligan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_HOOLIGAN: CardRecord = CardRecord::new(
    "Charging Hooligan",
    "fddc6f47-202d-4764-abc1-ee453a8917c2",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// WOE 319 — Ogre Chitterlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_CHITTERLORD: CardRecord = CardRecord::new(
    "Ogre Chitterlord",
    "a70b2033-eec5-4c86-9ebe-a68960a86763",
    "Piotr Foksowicz",
    crate::card::CardRules::unsupported(),
);

// WOE 320 — Intrepid Trufflesnout // Go Hog Wild
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTREPID_TRUFFLESNOUT: CardRecord = CardRecord::new(
    "Intrepid Trufflesnout // Go Hog Wild",
    "4224747e-1dbc-4a29-b5da-5916d8ca2768",
    "Kisung Koh",
    crate::card::CardRules::unsupported(),
);

// WOE 321 — Provisions Merchant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROVISIONS_MERCHANT: CardRecord = CardRecord::new(
    "Provisions Merchant",
    "a015282d-bcd4-44ab-ae26-41e4e3b23fc0",
    "Raluca Marinescu",
    crate::card::CardRules::unsupported(),
);

// WOE 322 — Wildwood Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILDWOOD_MENTOR: CardRecord = CardRecord::new(
    "Wildwood Mentor",
    "96c247f4-06cf-4c41-8285-d44d40f4130c",
    "Piotr Foksowicz",
    crate::card::CardRules::unsupported(),
);

// WOE 323 — Archon of the Wild Rose (alternate printing)
const ARCHON_OF_THE_WILD_ROSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHON_OF_THE_WILD_ROSE,
    1,
    "cd2b19f1-2f92-458d-a658-25481c57dea0",
    "Chris Rahn",
);

// WOE 324 — Expel the Interlopers (alternate printing)
const EXPEL_THE_INTERLOPERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXPEL_THE_INTERLOPERS,
    1,
    "92f5907a-2d70-46a9-b9f9-0ae7bcf35d48",
    "Andreas Zafiratos",
);

// WOE 325 — Moonshaker Cavalry (alternate printing)
const MOONSHAKER_CAVALRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOONSHAKER_CAVALRY,
    1,
    "08e06198-b96a-4b52-ba86-3df8278c0785",
    "Aldo Domínguez",
);

// WOE 326 — Regal Bunnicorn (alternate printing)
const REGAL_BUNNICORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REGAL_BUNNICORN,
    1,
    "6e650b3f-30a7-4943-8829-055e8aa7b9cd",
    "Ilse Gort",
);

// WOE 327 — Spellbook Vendor (alternate printing)
const SPELLBOOK_VENDOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPELLBOOK_VENDOR,
    1,
    "6973853c-2e95-4b5b-9008-4b06ffc8a078",
    "Scott Murphy",
);

// WOE 328 — A Tale for the Ages (alternate printing)
const A_TALE_FOR_THE_AGES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &A_TALE_FOR_THE_AGES,
    1,
    "4fc7d7d8-0965-4097-98ee-e89cfa05fd05",
    "Julie Dillon",
);

// WOE 329 — Werefox Bodyguard (alternate printing)
const WEREFOX_BODYGUARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEREFOX_BODYGUARD,
    1,
    "572eff52-c141-4587-857b-e71050899bdb",
    "Néstor Ossandón Leal",
);

// WOE 330 — Asinine Antics (alternate printing)
const ASININE_ANTICS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASININE_ANTICS,
    1,
    "dba9c281-a6e0-4af9-a217-e6aee7a38ff0",
    "Brent Hollowell",
);

// WOE 331 — Extraordinary Journey (alternate printing)
const EXTRAORDINARY_JOURNEY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXTRAORDINARY_JOURNEY,
    1,
    "abd423cc-7a86-4b15-b591-85940faa323e",
    "Volkan Baǵa",
);

// WOE 332 — Farsight Ritual (alternate printing)
const FARSIGHT_RITUAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FARSIGHT_RITUAL,
    1,
    "a3f484b9-486f-4aff-8b23-d8e555b476d3",
    "Randy Gallegos",
);

// WOE 333 — Ingenious Prodigy (alternate printing)
const INGENIOUS_PRODIGY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INGENIOUS_PRODIGY,
    1,
    "49bf6c34-429b-4213-856b-e8189276f2db",
    "Brian Valeza",
);

// WOE 334 — Sleep-Cursed Faerie (alternate printing)
const SLEEP_CURSED_FAERIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLEEP_CURSED_FAERIE,
    1,
    "5cb94d9f-d679-4eb2-97d4-b519d8a166b0",
    "Heonhwa",
);

// WOE 335 — Talion's Messenger (alternate printing)
const TALION_S_MESSENGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TALION_S_MESSENGER,
    1,
    "b54e69e4-c6c9-4735-ab86-c437eba8d4e2",
    "Marta Nael",
);

// WOE 336 — Beseech the Mirror (alternate printing)
const BESEECH_THE_MIRROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BESEECH_THE_MIRROR,
    1,
    "5df3d7fa-2b43-4b9e-85eb-ef96537942fb",
    "Cynthia Sheppard",
);

// WOE 337 — The End (alternate printing)
const THE_END_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_END,
    1,
    "79220c46-01df-41c1-877e-577f9ce78593",
    "Donato Giancola",
);

// WOE 338 — Lich-Knights' Conquest (alternate printing)
const LICH_KNIGHTS_CONQUEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LICH_KNIGHTS_CONQUEST,
    1,
    "edee2cc6-955c-43fb-b359-9d0e261c6925",
    "Denis Zhbankov",
);

// WOE 339 — Lord Skitter, Sewer King (alternate printing)
const LORD_SKITTER_SEWER_KING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LORD_SKITTER_SEWER_KING,
    1,
    "728520b3-fd90-46e4-ad51-3121a2557e86",
    "Jesper Ejsing",
);

// WOE 340 — Lord Skitter's Blessing (alternate printing)
const LORD_SKITTER_S_BLESSING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LORD_SKITTER_S_BLESSING,
    1,
    "77c78dde-4059-4e8d-9feb-f03653773787",
    "Joseph Weston",
);

// WOE 341 — Rankle's Prank (alternate printing)
const RANKLE_S_PRANK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RANKLE_S_PRANK,
    1,
    "c342061c-6992-47d2-b862-db310d389b3d",
    "Tyler Walpole",
);

// WOE 342 — Specter of Mortality (alternate printing)
const SPECTER_OF_MORTALITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPECTER_OF_MORTALITY,
    1,
    "3026523f-65a0-49a4-a46e-bb91a8988ea4",
    "Daarken",
);

// WOE 343 — Spiteful Hexmage (alternate printing)
const SPITEFUL_HEXMAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPITEFUL_HEXMAGE,
    1,
    "d527d48d-d6f7-4587-955e-de23b80ce0d6",
    "Anna Steinbauer",
);

// WOE 344 — Tangled Colony (alternate printing)
const TANGLED_COLONY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TANGLED_COLONY,
    1,
    "45ba866b-eda5-45c0-8878-235abadc0a3c",
    "Filip Burburan",
);

// WOE 345 — Charming Scoundrel (alternate printing)
const CHARMING_SCOUNDREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHARMING_SCOUNDREL,
    1,
    "51f94c92-cf88-4770-8ab8-874cd6634510",
    "Caroline Gariba",
);

// WOE 346 — Food Fight (alternate printing)
const FOOD_FIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOOD_FIGHT,
    1,
    "fef34fc2-8d37-484d-9adf-e3bb70283f33",
    "Filipe Pagliuso",
);

// WOE 347 — Goddric, Cloaked Reveler (alternate printing)
const GODDRIC_CLOAKED_REVELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GODDRIC_CLOAKED_REVELER,
    1,
    "bb884762-5468-43be-870a-96328f8172c2",
    "Jason A. Engle",
);

// WOE 348 — Imodane, the Pyrohammer (alternate printing)
const IMODANE_THE_PYROHAMMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IMODANE_THE_PYROHAMMER,
    1,
    "77bf3c05-4cbb-4525-9724-2d04cb2084f3",
    "Chris Rahn",
);

// WOE 349 — Raging Battle Mouse (alternate printing)
const RAGING_BATTLE_MOUSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAGING_BATTLE_MOUSE,
    1,
    "640cabc6-f3a6-4992-acfc-1bed6bc5f05d",
    "Rudy Siswanto",
);

// WOE 350 — Realm-Scorcher Hellkite (alternate printing)
const REALM_SCORCHER_HELLKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REALM_SCORCHER_HELLKITE,
    1,
    "8d2d38fe-7fa5-4b29-b3e2-dd20d5afd127",
    "Billy Christian",
);

// WOE 351 — Redcap Gutter-Dweller (alternate printing)
const REDCAP_GUTTER_DWELLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REDCAP_GUTTER_DWELLER,
    1,
    "59cb9275-07f2-46b1-a64b-2971390a35de",
    "Alexey Kruglov",
);

// WOE 352 — Rotisserie Elemental (alternate printing)
const ROTISSERIE_ELEMENTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROTISSERIE_ELEMENTAL,
    1,
    "f4a55780-cb3b-429a-b0a8-ac616f85582f",
    "Leonardo Santanna",
);

// WOE 353 — Song of Totentanz (alternate printing)
const SONG_OF_TOTENTANZ_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SONG_OF_TOTENTANZ,
    1,
    "30c2f106-0df8-4d4f-a6ef-7946fe8e47c4",
    "Randy Gallegos",
);

// WOE 354 — Blossoming Tortoise (alternate printing)
const BLOSSOMING_TORTOISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOSSOMING_TORTOISE,
    1,
    "79593c3a-00d9-4d24-a49c-753520a1ce30",
    "Simon Dominic",
);

// WOE 355 — Elvish Archivist (alternate printing)
const ELVISH_ARCHIVIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVISH_ARCHIVIST,
    1,
    "e7678157-e18e-45bd-a28e-7594a49bf2d7",
    "Mila Pesic",
);

// WOE 356 — Feral Encounter (alternate printing)
const FERAL_ENCOUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FERAL_ENCOUNTER,
    1,
    "794c5066-724b-4f94-aa6e-7b0690dd706e",
    "Fajareka Setiawan",
);

// WOE 357 — Gruff Triplets (alternate printing)
const GRUFF_TRIPLETS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRUFF_TRIPLETS,
    1,
    "f48f7bd4-c12d-48a7-b39f-36033636e5c7",
    "Fajareka Setiawan",
);

// WOE 358 — Sentinel of Lost Lore (alternate printing)
const SENTINEL_OF_LOST_LORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SENTINEL_OF_LOST_LORE,
    1,
    "a8556d3c-9290-4600-afa5-5928d74e3c96",
    "Cristi Balanescu",
);

// WOE 359 — Thunderous Debut (alternate printing)
const THUNDEROUS_DEBUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_DEBUT,
    1,
    "b742aa20-4b18-4296-9693-c2c5a8ba6c04",
    "Aldo Domínguez",
);

// WOE 360 — Agatha of the Vile Cauldron (alternate printing)
const AGATHA_OF_THE_VILE_CAULDRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGATHA_OF_THE_VILE_CAULDRON,
    1,
    "cd5f6c96-698d-4f5c-81ec-91176bd2e6e8",
    "Jason A. Engle",
);

// WOE 361 — Faunsbane Troll (alternate printing)
const FAUNSBANE_TROLL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAUNSBANE_TROLL,
    1,
    "3bf2ce6a-f2ee-42a4-a026-b1d0c18a3ed4",
    "Artur Nakhodkin",
);

// WOE 362 — The Goose Mother (alternate printing)
const THE_GOOSE_MOTHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_GOOSE_MOTHER,
    1,
    "1024d5f1-69f4-4a09-ba83-6fcc249217fa",
    "Jesper Ejsing",
);

// WOE 363 — Hylda of the Icy Crown (alternate printing)
const HYLDA_OF_THE_ICY_CROWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HYLDA_OF_THE_ICY_CROWN,
    1,
    "18f5f043-0b59-40ba-bdab-1706adf44075",
    "Ekaterina Burmak",
);

// WOE 364 — Likeness Looter (alternate printing)
const LIKENESS_LOOTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIKENESS_LOOTER,
    1,
    "5d51232f-ee63-4929-adab-042101c97b23",
    "Ben Hill",
);

// WOE 365 — Yenna, Redtooth Regent (alternate printing)
const YENNA_REDTOOTH_REGENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YENNA_REDTOOTH_REGENT,
    1,
    "a02f04f7-a345-4aa9-933c-1969f0cdd5c6",
    "Justyna Dura",
);

// WOE 366 — Agatha's Soul Cauldron (alternate printing)
const AGATHAS_SOUL_CAULDRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGATHAS_SOUL_CAULDRON,
    1,
    "dec5fc59-73d8-4735-88f1-3dbd1f15a546",
    "Jason A. Engle",
);

// WOE 367 — Hylda's Crown of Winter (alternate printing)
const HYLDA_S_CROWN_OF_WINTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HYLDA_S_CROWN_OF_WINTER,
    1,
    "8141f900-a87b-40ce-8439-02c000aafa30",
    "Volkan Baǵa",
);

// WOE 368 — The Irencrag (alternate printing)
const THE_IRENCRAG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_IRENCRAG,
    1,
    "93c50432-28af-40e8-ae98-d4f33a5a936e",
    "Adam Paquette",
);

// WOE 369 — Syr Ginger, the Meal Ender (alternate printing)
const SYR_GINGER_THE_MEAL_ENDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYR_GINGER_THE_MEAL_ENDER,
    1,
    "522e43f3-4f9a-4f40-a5f5-d84277172040",
    "Michal Ivan",
);

// WOE 370 — Lady of Laughter (alternate printing)
const LADY_OF_LAUGHTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LADY_OF_LAUGHTER,
    1,
    "10ba4b10-5667-414c-a692-166a19b7d748",
    "Kai Carpenter",
);

// WOE 371 — Faerie Slumber Party (alternate printing)
const FAERIE_SLUMBER_PARTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAERIE_SLUMBER_PARTY,
    1,
    "f4d1945f-6462-4ec3-a966-3a8dafc0f5d8",
    "Lie Setiawan",
);

// WOE 372 — Malevolent Witchkite (alternate printing)
const MALEVOLENT_WITCHKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MALEVOLENT_WITCHKITE,
    1,
    "58ee6723-9b90-4d39-9cbb-9d3765a4fcb0",
    "Donato Giancola",
);

// WOE 373 — Ogre Chitterlord (alternate printing)
const OGRE_CHITTERLORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OGRE_CHITTERLORD,
    1,
    "cd8d687a-8935-4907-909f-4cadac6cdf67",
    "Piotr Foksowicz",
);

// WOE 374 — Wildwood Mentor (alternate printing)
const WILDWOOD_MENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILDWOOD_MENTOR,
    1,
    "fb7b456e-e56d-45dd-88b3-1ac58a9bfb5f",
    "Piotr Foksowicz",
);

// WOE 375 — Stroke of Midnight (alternate printing)
const STROKE_OF_MIDNIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STROKE_OF_MIDNIGHT,
    1,
    "73014a95-5251-4404-b3a3-037fb5ca3327",
    "Julia Metzger",
);

// WOE 376 — Sleight of Hand (alternate printing)
const SLEIGHT_OF_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::SLEIGHT_OF_HAND,
    1,
    "87c81647-6586-4c9c-93fc-9d5ee40b377b",
    "Scott Murphy",
);

// WOE 377 — Faerie Dreamthief (alternate printing)
const FAERIE_DREAMTHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAERIE_DREAMTHIEF,
    1,
    "373f235b-ce93-48b0-8e36-71f5bc5efcc2",
    "Randy Vargas",
);

// WOE 378 — Torch the Tower (alternate printing)
const TORCH_THE_TOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TORCH_THE_TOWER,
    1,
    "f603675e-84df-4dd2-b45b-47349341f64c",
    "Uriah Voth",
);

// WOE 379 — Tanglespan Lookout (alternate printing)
const TANGLESPAN_LOOKOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TANGLESPAN_LOOKOUT,
    1,
    "dca603dc-8a2d-484d-8b95-25dfa39369fd",
    "Dmitry Burmak",
);

// WOE 380 — Lich-Knights' Conquest (alternate printing)
const LICH_KNIGHTS_CONQUEST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LICH_KNIGHTS_CONQUEST,
    2,
    "89ffe314-b66b-48d8-b94d-36c8bb5861e4",
    "Kristina Carroll",
);

// WOE 381 — Expel the Interlopers (alternate printing)
const EXPEL_THE_INTERLOPERS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXPEL_THE_INTERLOPERS,
    2,
    "bdaac375-57ce-432b-b718-b547367faec7",
    "Awanqi (Angela Wang)",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHON_OF_THE_WILD_ROSE,
    &ARCHON_S_GLORY,
    &ARMORY_MICE,
    &BESOTTED_KNIGHT,
    &BREAK_THE_SPELL,
    &CHARMED_CLOTHIER,
    &CHEEKY_HOUSE_MOUSE,
    &COOPED_UP,
    &CURSED_COURTIER,
    &DISCERNING_FINANCIER,
    &DUTIFUL_GRIFFIN,
    &EERIE_INTERFERENCE,
    &EXPEL_THE_INTERLOPERS,
    &FROSTBRIDGE_GUARD,
    &GALLANT_PIE_WIELDER,
    &HOPEFUL_VIGIL,
    &KELLAN_S_LIGHTBLADES,
    &KNIGHT_OF_DOVES,
    &MOMENT_OF_VALOR,
    &MOONSHAKER_CAVALRY,
    &PLUNGE_INTO_WINTER,
    &THE_PRINCESS_TAKES_FLIGHT,
    &PROTECTIVE_PARENTS,
    &REGAL_BUNNICORN,
    &RETURN_TRIUMPHANT,
    &RIMEFUR_REINDEER,
    &SAVIOR_OF_THE_SLEEPING,
    &SLUMBERING_KEEPGUARD,
    &SOLITARY_SANCTUARY,
    &SPELLBOOK_VENDOR,
    &STOCKPILING_CELEBRANT,
    &STROKE_OF_MIDNIGHT,
    &A_TALE_FOR_THE_AGES,
    &THREE_BLIND_MICE,
    &TUINVALE_GUIDE,
    &UNASSUMING_SAGE,
    &WEREFOX_BODYGUARD,
    &AQUATIC_ALCHEMIST,
    &ARCHIVE_DRAGON,
    &ASININE_ANTICS,
    &BELUNA_S_GATEKEEPER,
    &BITTER_CHILL,
    &CHANCELLOR_OF_TALES,
    &DIMINISHER_WITCH,
    &EXTRAORDINARY_JOURNEY,
    &FARSIGHT_RITUAL,
    &FREEZE_IN_PLACE,
    &GADWICK_S_FIRST_DUEL,
    &GALVANIC_GIANT,
    &HORNED_LOCH_WHALE,
    &ICE_OUT,
    &ICEWROUGHT_SENTRY,
    &INGENIOUS_PRODIGY,
    &INTO_THE_FAE_COURT,
    &JOHANN_S_STOPGAP,
    &LIVING_LECTERN,
    &MERFOLK_CORALSMITH,
    &MISLEADING_MOTES,
    &MOCKING_SPRITE,
    &OBYRA_S_ATTENDANTS,
    &PICKLOCK_PRANKSTER,
    &QUICK_STUDY,
    &SLEEP_CURSED_FAERIE,
    &SNAREMASTER_SPRITE,
    &SPELL_STUTTER,
    &SPLASHY_SPELLCASTER,
    &STORMKELD_PROWLER,
    &SUCCUMB_TO_THE_COLD,
    &TALION_S_MESSENGER,
    &TENACIOUS_TOMESEEKER,
    &VANTRESS_TRANSMUTER,
    &VIRTUE_OF_KNOWLEDGE,
    &WATER_WINGS,
    &ASHIOK_WICKED_MANIPULATOR,
    &ASHIOK_S_REAPER,
    &BACK_FOR_SECONDS,
    &BARROW_NAUGHTY,
    &BESEECH_THE_MIRROR,
    &CANDY_GRAPPLE,
    &CONCEITED_WITCH,
    &DREAM_SPOILERS,
    &EGO_DRAIN,
    &THE_END,
    &ERIETTE_S_WHISPER,
    &FAERIE_DREAMTHIEF,
    &FAERIE_FENCING,
    &FEED_THE_CAULDRON,
    &FELL_HORSEMAN,
    &GUMDROP_POISONER,
    &HIGH_FAE_NEGOTIATOR,
    &HOPELESS_NIGHTMARE,
    &LICH_KNIGHTS_CONQUEST,
    &LORD_SKITTER_SEWER_KING,
    &LORD_SKITTER_S_BLESSING,
    &LORD_SKITTER_S_BUTCHER,
    &MINTSTROSITY,
    &NOT_DEAD_AFTER_ALL,
    &RANKLE_S_PRANK,
    &RAT_OUT,
    &ROWAN_S_GRIM_SEARCH,
    &SCREAM_PUFF,
    &SHATTER_THE_OATH,
    &SPECTER_OF_MORTALITY,
    &SPITEFUL_HEXMAGE,
    &STINGBLADE_ASSASSIN,
    &SUGAR_RUSH,
    &SWEETTOOTH_WITCH,
    &TAKEN_BY_NIGHTMARES,
    &TANGLED_COLONY,
    &TWISTED_SEWER_WITCH,
    &VIRTUE_OF_PERSISTENCE,
    &VORACIOUS_VERMIN,
    &WAREHOUSE_TABBY,
    &WICKED_VISITOR,
    &THE_WITCH_S_VANITY,
    &BELLIGERENT_OF_THE_BALL,
    &BELLOWING_BRUISER,
    &BESPOKE_BATTLEGARB,
    &BOUNDARY_LANDS_RANGER,
    &CHARMING_SCOUNDREL,
    &CUT_IN,
    &EDGEWALL_PACK,
    &EMBERETH_VETERAN,
    &FLICK_A_COIN,
    &FOOD_FIGHT,
    &FRANTIC_FIREBOLT,
    &GNAWING_CRESCENDO,
    &GODDRIC_CLOAKED_REVELER,
    &GRABBY_GIANT,
    &GRAND_BALL_GUEST,
    &HARRIED_SPEARGUARD,
    &HEARTH_ELEMENTAL,
    &IMODANE_THE_PYROHAMMER,
    &KINDLED_HEROISM,
    &KORVOLD_AND_THE_NOBLE_THIEF,
    &MERRY_BARDS,
    &MINECART_DAREDEVIL,
    &MONSTROUS_RAGE,
    &RAGING_BATTLE_MOUSE,
    &RATCATCHER_TRAINEE,
    &REALM_SCORCHER_HELLKITE,
    &REDCAP_GUTTER_DWELLER,
    &REDCAP_THIEF,
    &ROTISSERIE_ELEMENTAL,
    &SKEWER_SLINGER,
    &SONG_OF_TOTENTANZ,
    &STONESPLITTER_BOLT,
    &TATTERED_RATTER,
    &TORCH_THE_TOWER,
    &TWISTED_FEALTY,
    &TWO_HEADED_HUNTER,
    &UNRULY_CATAPULT,
    &VIRTUE_OF_COURAGE,
    &WITCH_S_MARK,
    &WITCHSTALKER_FRENZY,
    &AGATHA_S_CHAMPION,
    &BEANSTALK_WURM,
    &BESTIAL_BLOODLINE,
    &BLOSSOMING_TORTOISE,
    &BRAMBLE_FAMILIAR,
    &BRAVE_THE_WILDS,
    &CURSE_OF_THE_WEREFOX,
    &ELVISH_ARCHIVIST,
    &FERAL_ENCOUNTER,
    &FEROCIOUS_WEREFOX,
    &GRACEFUL_TAKEDOWN,
    &GRUFF_TRIPLETS,
    &HAMLET_GLUTTON,
    &HOLLOW_SCAVENGER,
    &HOWLING_GALEFANG,
    &THE_HUNTSMAN_S_REDEMPTION,
    &LEAPING_AMBUSH,
    &NIGHT_OF_THE_SWEETS_REVENGE,
    &REDTOOTH_GENEALOGIST,
    &REDTOOTH_VANGUARD,
    &RETURN_FROM_THE_WILDS,
    &ROOTRIDER_FAUN,
    &ROYAL_TREATMENT,
    &SENTINEL_OF_LOST_LORE,
    &SKYBEAST_TRACKER,
    &SPIDER_FOOD,
    &STORMKELD_VANGUARD,
    &TANGLESPAN_LOOKOUT,
    &TERRITORIAL_WITCHSTALKER,
    &THUNDEROUS_DEBUT,
    &TOADSTOOL_ADMIRER,
    &TOUGH_COOKIE,
    &TROUBLEMAKER_OUPHE,
    &UP_THE_BEANSTALK,
    &VERDANT_OUTRIDER,
    &VIRTUE_OF_STRENGTH,
    &WELCOME_TO_SWEETTOOTH,
    &AGATHA_OF_THE_VILE_CAULDRON,
    &THE_APPRENTICE_S_FOLLY,
    &ASH_PARTY_CRASHER,
    &ERIETTE_OF_THE_CHARMED_APPLE,
    &FAUNSBANE_TROLL,
    &THE_GOOSE_MOTHER,
    &GRETA_SWEETTOOTH_SCOURGE,
    &HYLDA_OF_THE_ICY_CROWN,
    &JOHANN_APPRENTICE_SORCERER,
    &LIKENESS_LOOTER,
    &NEVA_STALKED_BY_NIGHTMARES,
    &OBYRA_DREAMING_DUELIST,
    &ROWAN_SCION_OF_WAR,
    &RUBY_DARING_TRACKER,
    &SHARAE_OF_NUMBING_DEPTHS,
    &SYR_ARMONT_THE_REDEEMER,
    &TALION_THE_KINDLY_LORD,
    &TOTENTANZ_SWARM_PIPER,
    &TROYAN_GUTSY_EXPLORER,
    &WILL_SCION_OF_PEACE,
    &YENNA_REDTOOTH_REGENT,
    &BELUNA_GRANDSQUALL,
    &CALLOUS_SELL_SWORD,
    &CRUEL_SOMNOPHAGE,
    &DECADENT_DRAGON,
    &DEVOURING_SUGARMAW,
    &ELUSIVE_OTTER,
    &FROLICKING_FAMILIAR,
    &GINGERBREAD_HUNTER,
    &HEARTFLAME_DUELIST,
    &IMODANE_S_RECRUITER,
    &KELLAN_THE_FAE_BLOODED,
    &MOSSWOOD_DREADKNIGHT,
    &PICNIC_RUINER,
    &POLLEN_SHIELD_HARE,
    &QUESTING_DRUID,
    &SCALDING_VIPER,
    &SHROUDED_SHEPHERD,
    &SPELLSCORN_COVEN,
    &TEMPEST_HART,
    &THREADBIND_CLIQUE,
    &TWINING_TWINS,
    &WOODLAND_ACOLYTE,
    &AGATHAS_SOUL_CAULDRON,
    &CANDY_TRAIL,
    &COLLECTOR_S_VAULT,
    &ERIETTE_S_TEMPTING_APPLE,
    &HYLDA_S_CROWN_OF_WINTER,
    &THE_IRENCRAG,
    &SCARECROW_GUIDE,
    &SYR_GINGER_THE_MEAL_ENDER,
    &THREE_BOWLS_OF_PORRIDGE,
    &EDGEWALL_INN,
    &RESTLESS_BIVOUAC,
    &RESTLESS_COTTAGE,
    &RESTLESS_FORTRESS,
    &RESTLESS_SPIRE,
    &RESTLESS_VINESTALK,
    &VIRTUE_OF_LOYALTY,
    &FOOD_COMA,
    &LADY_OF_LAUGHTER,
    &PESTS_OF_HONOR,
    &FAERIE_SLUMBER_PARTY,
    &ROWDY_RESEARCH,
    &STORYTELLER_PIXIE,
    &EXPERIMENTAL_CONFECTIONER,
    &MALEVOLENT_WITCHKITE,
    &OLD_FLITTERFANG,
    &BECOME_BRUTES,
    &CHARGING_HOOLIGAN,
    &OGRE_CHITTERLORD,
    &INTREPID_TRUFFLESNOUT,
    &PROVISIONS_MERCHANT,
    &WILDWOOD_MENTOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    GLASS_CASKET_REPRINT,
    VIRTUE_OF_LOYALTY_ALTERNATE_1,
    DISDAINFUL_STROKE_REPRINT,
    SLEIGHT_OF_HAND_REPRINT,
    COMMUNE_WITH_NATURE_REPRINT,
    TITANIC_GROWTH_REPRINT,
    GINGERBRUTE_REPRINT,
    PROPHETIC_PRISM_REPRINT,
    SOUL_GUIDE_LANTERN_REPRINT,
    CRYSTAL_GROTTO_REPRINT,
    EVOLVING_WILDS_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    HORNED_LOCH_WHALE_ALTERNATE_1,
    VIRTUE_OF_KNOWLEDGE_ALTERNATE_1,
    GUMDROP_POISONER_ALTERNATE_1,
    VIRTUE_OF_PERSISTENCE_ALTERNATE_1,
    VIRTUE_OF_COURAGE_ALTERNATE_1,
    BRAMBLE_FAMILIAR_ALTERNATE_1,
    VIRTUE_OF_STRENGTH_ALTERNATE_1,
    BELUNA_GRANDSQUALL_ALTERNATE_1,
    CRUEL_SOMNOPHAGE_ALTERNATE_1,
    DECADENT_DRAGON_ALTERNATE_1,
    DEVOURING_SUGARMAW_ALTERNATE_1,
    ELUSIVE_OTTER_ALTERNATE_1,
    HEARTFLAME_DUELIST_ALTERNATE_1,
    KELLAN_THE_FAE_BLOODED_ALTERNATE_1,
    MOSSWOOD_DREADKNIGHT_ALTERNATE_1,
    POLLEN_SHIELD_HARE_ALTERNATE_1,
    QUESTING_DRUID_ALTERNATE_1,
    SCALDING_VIPER_ALTERNATE_1,
    TWINING_TWINS_ALTERNATE_1,
    ASHIOK_WICKED_MANIPULATOR_ALTERNATE_1,
    KELLAN_THE_FAE_BLOODED_ALTERNATE_2,
    ERIETTE_OF_THE_CHARMED_APPLE_ALTERNATE_1,
    ROWAN_SCION_OF_WAR_ALTERNATE_1,
    TALION_THE_KINDLY_LORD_ALTERNATE_1,
    WILL_SCION_OF_PEACE_ALTERNATE_1,
    RESTLESS_BIVOUAC_ALTERNATE_1,
    RESTLESS_COTTAGE_ALTERNATE_1,
    RESTLESS_FORTRESS_ALTERNATE_1,
    RESTLESS_SPIRE_ALTERNATE_1,
    RESTLESS_VINESTALK_ALTERNATE_1,
    ARCHON_OF_THE_WILD_ROSE_ALTERNATE_1,
    EXPEL_THE_INTERLOPERS_ALTERNATE_1,
    MOONSHAKER_CAVALRY_ALTERNATE_1,
    REGAL_BUNNICORN_ALTERNATE_1,
    SPELLBOOK_VENDOR_ALTERNATE_1,
    A_TALE_FOR_THE_AGES_ALTERNATE_1,
    WEREFOX_BODYGUARD_ALTERNATE_1,
    ASININE_ANTICS_ALTERNATE_1,
    EXTRAORDINARY_JOURNEY_ALTERNATE_1,
    FARSIGHT_RITUAL_ALTERNATE_1,
    INGENIOUS_PRODIGY_ALTERNATE_1,
    SLEEP_CURSED_FAERIE_ALTERNATE_1,
    TALION_S_MESSENGER_ALTERNATE_1,
    BESEECH_THE_MIRROR_ALTERNATE_1,
    THE_END_ALTERNATE_1,
    LICH_KNIGHTS_CONQUEST_ALTERNATE_1,
    LORD_SKITTER_SEWER_KING_ALTERNATE_1,
    LORD_SKITTER_S_BLESSING_ALTERNATE_1,
    RANKLE_S_PRANK_ALTERNATE_1,
    SPECTER_OF_MORTALITY_ALTERNATE_1,
    SPITEFUL_HEXMAGE_ALTERNATE_1,
    TANGLED_COLONY_ALTERNATE_1,
    CHARMING_SCOUNDREL_ALTERNATE_1,
    FOOD_FIGHT_ALTERNATE_1,
    GODDRIC_CLOAKED_REVELER_ALTERNATE_1,
    IMODANE_THE_PYROHAMMER_ALTERNATE_1,
    RAGING_BATTLE_MOUSE_ALTERNATE_1,
    REALM_SCORCHER_HELLKITE_ALTERNATE_1,
    REDCAP_GUTTER_DWELLER_ALTERNATE_1,
    ROTISSERIE_ELEMENTAL_ALTERNATE_1,
    SONG_OF_TOTENTANZ_ALTERNATE_1,
    BLOSSOMING_TORTOISE_ALTERNATE_1,
    ELVISH_ARCHIVIST_ALTERNATE_1,
    FERAL_ENCOUNTER_ALTERNATE_1,
    GRUFF_TRIPLETS_ALTERNATE_1,
    SENTINEL_OF_LOST_LORE_ALTERNATE_1,
    THUNDEROUS_DEBUT_ALTERNATE_1,
    AGATHA_OF_THE_VILE_CAULDRON_ALTERNATE_1,
    FAUNSBANE_TROLL_ALTERNATE_1,
    THE_GOOSE_MOTHER_ALTERNATE_1,
    HYLDA_OF_THE_ICY_CROWN_ALTERNATE_1,
    LIKENESS_LOOTER_ALTERNATE_1,
    YENNA_REDTOOTH_REGENT_ALTERNATE_1,
    AGATHAS_SOUL_CAULDRON_ALTERNATE_1,
    HYLDA_S_CROWN_OF_WINTER_ALTERNATE_1,
    THE_IRENCRAG_ALTERNATE_1,
    SYR_GINGER_THE_MEAL_ENDER_ALTERNATE_1,
    LADY_OF_LAUGHTER_ALTERNATE_1,
    FAERIE_SLUMBER_PARTY_ALTERNATE_1,
    MALEVOLENT_WITCHKITE_ALTERNATE_1,
    OGRE_CHITTERLORD_ALTERNATE_1,
    WILDWOOD_MENTOR_ALTERNATE_1,
    STROKE_OF_MIDNIGHT_ALTERNATE_1,
    SLEIGHT_OF_HAND_ALTERNATE_1,
    FAERIE_DREAMTHIEF_ALTERNATE_1,
    TORCH_THE_TOWER_ALTERNATE_1,
    TANGLESPAN_LOOKOUT_ALTERNATE_1,
    LICH_KNIGHTS_CONQUEST_ALTERNATE_2,
    EXPEL_THE_INTERLOPERS_ALTERNATE_2,
];
