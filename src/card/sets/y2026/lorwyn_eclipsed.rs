//! Lorwyn Eclipsed card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaTypeDef;
use crate::card::ObjectPredicateDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::sets::y2006::dissension as catalog_dis;
use crate::card::sets::y2006::guildpact as catalog_gpt;
use crate::card::sets::y2007::lorwyn as catalog_lrw;
use crate::card::sets::y2008::eventide as catalog_eve;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2019::modern_horizons as catalog_mh1;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;
use crate::card::sets::y2023::march_of_the_machine as catalog_mom;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ECL",
    slug: "lorwyn-eclipsed",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ECL 1 — Changeling Wayfinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHANGELING_WAYFINDER: CardRecord = CardRecord::new(
    "Changeling Wayfinder",
    "6c6061aa-a4da-4115-85b6-d0aa22f2386c",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// ECL 2 — Rooftop Percher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOFTOP_PERCHER: CardRecord = CardRecord::new(
    "Rooftop Percher",
    "2d89595c-1542-41fe-8d23-997522922698",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// ECL 3 — Adept Watershaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADEPT_WATERSHAPER: CardRecord = CardRecord::new(
    "Adept Watershaper",
    "6e53f246-8347-4632-9d5b-4aeb12f7b762",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// ECL 4 — Ajani, Outland Chaperone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AJANI_OUTLAND_CHAPERONE: CardRecord = CardRecord::new(
    "Ajani, Outland Chaperone",
    "6124a691-ae83-4d22-a177-0aee65b47064",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ECL 5 — Appeal to Eirdu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APPEAL_TO_EIRDU: CardRecord = CardRecord::new(
    "Appeal to Eirdu",
    "68ce9752-21f9-48e9-bf48-7f76f1cecbc5",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// ECL 6 — Bark of Doran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARK_OF_DORAN: CardRecord = CardRecord::new(
    "Bark of Doran",
    "98210276-1b85-4db5-8ab4-ecb08f5d2ee2",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// ECL 7 — Brigid, Clachan's Heart // Brigid, Doun's Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIGID_CLACHAN_S_HEART: CardRecord = CardRecord::new(
    "Brigid, Clachan's Heart // Brigid, Doun's Mind",
    "cb7d5bbb-4f68-4e38-8bb0-a95af21b24c8",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// ECL 8 — Burdened Stoneback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURDENED_STONEBACK: CardRecord = CardRecord::new(
    "Burdened Stoneback",
    "3278b8d0-3d2b-4d3d-bbf1-fd9b714b53ed",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ECL 9 — Champion of the Clachan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMPION_OF_THE_CLACHAN: CardRecord = CardRecord::new(
    "Champion of the Clachan",
    "46ce3474-381c-433b-acd8-4e628d0048d2",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// ECL 10 — Clachan Festival
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLACHAN_FESTIVAL: CardRecord = CardRecord::new(
    "Clachan Festival",
    "324b5234-ffbf-4801-a475-8f693679ae2f",
    "Kev Fang",
    crate::card::CardRules::unsupported(),
);

// ECL 11 — Crib Swap (reprint)
const CRIB_SWAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::CRIB_SWAP,
    "8f2fb3c6-af75-47a3-9f97-521872c32890",
    "Pete Venters",
);

// ECL 12 — Curious Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURIOUS_COLOSSUS: CardRecord = CardRecord::new(
    "Curious Colossus",
    "582b6e5d-0bab-471d-af4d-19438c5fd524",
    "Raoul Vitale",
    crate::card::CardRules::unsupported(),
);

// ECL 13 — Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EIRDU_CARRIER_OF_DAWN: CardRecord = CardRecord::new(
    "Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight",
    "b2d9d5ca-7e15-437a-bdfc-5972b42148fe",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// ECL 14 — Encumbered Reejerey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENCUMBERED_REEJEREY: CardRecord = CardRecord::new(
    "Encumbered Reejerey",
    "15ff6797-f59c-4333-98ea-5711150fd5b8",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// ECL 15 — Evershrike's Gift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVERSHRIKE_S_GIFT: CardRecord = CardRecord::new(
    "Evershrike's Gift",
    "231e46b6-b91a-4582-8894-e7de1c50213f",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ECL 16 — Flock Impostor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOCK_IMPOSTOR: CardRecord = CardRecord::new(
    "Flock Impostor",
    "d32d0336-5140-41f9-bc67-f3d743b9231d",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// ECL 17 — Gallant Fowlknight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALLANT_FOWLKNIGHT: CardRecord = CardRecord::new(
    "Gallant Fowlknight",
    "fb6096ba-8083-4207-9a3f-c1e4ff095204",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// ECL 18 — Goldmeadow Nomad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLDMEADOW_NOMAD: CardRecord = CardRecord::new(
    "Goldmeadow Nomad",
    "00ddbe6c-11de-4bc6-aabe-d6d8385a838a",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// ECL 19 — Keep Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEP_OUT: CardRecord = CardRecord::new(
    "Keep Out",
    "4ab1601c-634c-4f21-8926-ba3cb92008c1",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ECL 20 — Kinbinding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINBINDING: CardRecord = CardRecord::new(
    "Kinbinding",
    "b6e35fd5-de7e-40a8-a23c-00d07fd1ac56",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// ECL 21 — Kinsbaile Aspirant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINSBAILE_ASPIRANT: CardRecord = CardRecord::new(
    "Kinsbaile Aspirant",
    "56dfdab1-ea3f-4663-a855-a9e72505f85e",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ECL 22 — Kinscaer Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINSCAER_SENTRY: CardRecord = CardRecord::new(
    "Kinscaer Sentry",
    "333bf101-14e8-4753-99bc-9174f42c4122",
    "Kev Fang",
    crate::card::CardRules::unsupported(),
);

// ECL 23 — Kithkeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KITHKEEPER: CardRecord = CardRecord::new(
    "Kithkeeper",
    "ef29eff7-72be-46e6-9275-0f0a44d29233",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// ECL 24 — Liminal Hold
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIMINAL_HOLD: CardRecord = CardRecord::new(
    "Liminal Hold",
    "a5a40c16-7a5c-4ad1-be53-6b1b1be2affe",
    "Ovidio Cartagena",
    crate::card::CardRules::unsupported(),
);

// ECL 25 — Meanders Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEANDERS_GUIDE: CardRecord = CardRecord::new(
    "Meanders Guide",
    "8c41a0ad-138e-4eef-8f7f-35017e3b086f",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// ECL 26 — Moonlit Lamenter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONLIT_LAMENTER: CardRecord = CardRecord::new(
    "Moonlit Lamenter",
    "fb8fc509-cff6-470f-abf6-b07f6c3f94e1",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// ECL 27 — Morningtide's Light
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORNINGTIDE_S_LIGHT: CardRecord = CardRecord::new(
    "Morningtide's Light",
    "181ee045-5650-479a-8c03-015b38fdcd63",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ECL 28 — Personify
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERSONIFY: CardRecord = CardRecord::new(
    "Personify",
    "1172582d-fb2d-4022-95b1-e48b03df3a95",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// ECL 29 — Protective Response
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROTECTIVE_RESPONSE: CardRecord = CardRecord::new(
    "Protective Response",
    "113975e1-7712-4760-96ad-405f8b4e41e3",
    "Gustavo Pelissari",
    crate::card::CardRules::unsupported(),
);

// ECL 30 — Pyrrhic Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYRRHIC_STRIKE: CardRecord = CardRecord::new(
    "Pyrrhic Strike",
    "cce5b16d-07fb-4e64-8ec9-b8b29ba86cff",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// ECL 31 — Reluctant Dounguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELUCTANT_DOUNGUARD: CardRecord = CardRecord::new(
    "Reluctant Dounguard",
    "dbce93c0-5efc-4b60-9cef-9d9b374d397b",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// ECL 32 — Rhys, the Evermore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYS_THE_EVERMORE: CardRecord = CardRecord::new(
    "Rhys, the Evermore",
    "a7072412-4aa2-40ef-a267-bd717551a42b",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// ECL 33 — Riverguard's Reflexes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVERGUARD_S_REFLEXES: CardRecord = CardRecord::new(
    "Riverguard's Reflexes",
    "88a79f3e-ca74-467c-b0a8-22802ac5b465",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// ECL 34 — Shore Lurker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHORE_LURKER: CardRecord = CardRecord::new(
    "Shore Lurker",
    "bb353c27-e311-4677-9277-cab6820562ce",
    "Tiffany Turrill",
    crate::card::CardRules::unsupported(),
);

// ECL 35 — Slumbering Walker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLUMBERING_WALKER: CardRecord = CardRecord::new(
    "Slumbering Walker",
    "81e82915-e734-4754-829f-f5da6a7c550d",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// ECL 36 — Spiral into Solitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRAL_INTO_SOLITUDE: CardRecord = CardRecord::new(
    "Spiral into Solitude",
    "e7a12664-a930-4159-8311-19862488fb05",
    "Drew Baker",
    crate::card::CardRules::unsupported(),
);

// ECL 37 — Sun-Dappled Celebrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUN_DAPPLED_CELEBRANT: CardRecord = CardRecord::new(
    "Sun-Dappled Celebrant",
    "91c715a5-6643-4064-a8a8-3e05dce15979",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// ECL 38 — Thoughtweft Imbuer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTWEFT_IMBUER: CardRecord = CardRecord::new(
    "Thoughtweft Imbuer",
    "7e585a6c-2d95-450e-98cb-8794d7d8ecca",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// ECL 39 — Timid Shieldbearer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMID_SHIELDBEARER: CardRecord = CardRecord::new(
    "Timid Shieldbearer",
    "1c672d38-a1a6-4912-a9a6-b11e7bf0cc67",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// ECL 40 — Tributary Vaulter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBUTARY_VAULTER: CardRecord = CardRecord::new(
    "Tributary Vaulter",
    "a1db2c83-41d1-4cc6-b9a3-fed504b01127",
    "Tiffany Turrill",
    crate::card::CardRules::unsupported(),
);

// ECL 41 — Wanderbrine Preacher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERBRINE_PREACHER: CardRecord = CardRecord::new(
    "Wanderbrine Preacher",
    "3fc3f5f2-5a83-4358-8f23-42f26f345140",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// ECL 42 — Wanderbrine Trapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERBRINE_TRAPPER: CardRecord = CardRecord::new(
    "Wanderbrine Trapper",
    "f4c134ed-adbf-4e88-80e0-75c176ce94c3",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// ECL 43 — Winnowing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINNOWING: CardRecord = CardRecord::new(
    "Winnowing",
    "f943a7d8-9550-427e-8c45-ef834329d345",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// ECL 44 — Aquitect's Defenses
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AQUITECT_S_DEFENSES: CardRecord = CardRecord::new(
    "Aquitect's Defenses",
    "9af9a907-fd2e-4ae0-ac7b-529074b79a14",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// ECL 45 — Blossombind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOSSOMBIND: CardRecord = CardRecord::new(
    "Blossombind",
    "382b83c0-bbbc-4db8-bc04-cfea79aed1b3",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ECL 46 — Champions of the Shoal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMPIONS_OF_THE_SHOAL: CardRecord = CardRecord::new(
    "Champions of the Shoal",
    "e1acdd9c-4a6d-4373-950c-f5539b54679f",
    "Daniel Zrom",
    crate::card::CardRules::unsupported(),
);

// ECL 47 — Disruptor of Currents
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISRUPTOR_OF_CURRENTS: CardRecord = CardRecord::new(
    "Disruptor of Currents",
    "4c6dbbaa-6844-4d7c-abbb-472a83bb99ab",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// ECL 48 — Flitterwing Nuisance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLITTERWING_NUISANCE: CardRecord = CardRecord::new(
    "Flitterwing Nuisance",
    "ad0f6536-5295-4835-8883-35d711dfe6de",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// ECL 49 — Glamer Gifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLAMER_GIFTER: CardRecord = CardRecord::new(
    "Glamer Gifter",
    "bd764dd4-2395-4138-ac29-260eac1aeaae",
    "Ben Hill",
    crate::card::CardRules::unsupported(),
);

// ECL 50 — Glamermite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLAMERMITE: CardRecord = CardRecord::new(
    "Glamermite",
    "b8b7c23a-0034-453c-ab44-f6ec0f31d1eb",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// ECL 51 — Glen Elendra Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLEN_ELENDRA_GUARDIAN: CardRecord = CardRecord::new(
    "Glen Elendra Guardian",
    "388d2e4a-0aa5-4b82-a86c-4777ca60161c",
    "Yohann Schepacz",
    crate::card::CardRules::unsupported(),
);

// ECL 52 — Glen Elendra's Answer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLEN_ELENDRA_S_ANSWER: CardRecord = CardRecord::new(
    "Glen Elendra's Answer",
    "fa5bfbf9-dca2-42b7-a431-f9afedb54528",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// ECL 53 — Gravelgill Scoundrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVELGILL_SCOUNDREL: CardRecord = CardRecord::new(
    "Gravelgill Scoundrel",
    "320bc30c-7a8a-411a-9f36-9c69126e131b",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// ECL 54 — Harmonized Crescendo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARMONIZED_CRESCENDO: CardRecord = CardRecord::new(
    "Harmonized Crescendo",
    "2715e0c0-9913-4bea-9a42-ad1164f6130a",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// ECL 55 — Illusion Spinners
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSION_SPINNERS: CardRecord = CardRecord::new(
    "Illusion Spinners",
    "eb4229a9-8df4-4adc-9d3e-acd2221fa3e9",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// ECL 56 — Kulrath Mystic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KULRATH_MYSTIC: CardRecord = CardRecord::new(
    "Kulrath Mystic",
    "377d257c-920c-4dd4-a4b1-01cbc631ef8f",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// ECL 57 — Loch Mare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOCH_MARE: CardRecord = CardRecord::new(
    "Loch Mare",
    "ad6c4baf-a803-45e4-81ac-708a41631a28",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// ECL 58 — Lofty Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOFTY_DREAMS: CardRecord = CardRecord::new(
    "Lofty Dreams",
    "a2908ed1-517e-4f1a-94e5-b06fb033c1a6",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// ECL 59 — Mirrorform
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRORFORM: CardRecord = CardRecord::new(
    "Mirrorform",
    "55d30256-f5d8-4f61-a3f5-878970ced6d1",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// ECL 60 — Noggle the Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOGGLE_THE_MIND: CardRecord = CardRecord::new(
    "Noggle the Mind",
    "076c0ac1-722e-49ef-b815-ace210069972",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ECL 61 — Oko, Lorwyn Liege // Oko, Shadowmoor Scion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OKO_LORWYN_LIEGE: CardRecord = CardRecord::new(
    "Oko, Lorwyn Liege // Oko, Shadowmoor Scion",
    "1dab370a-1067-4d94-be1f-10362d4abf5a",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// ECL 62 — Omni-Changeling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMNI_CHANGELING: CardRecord = CardRecord::new(
    "Omni-Changeling",
    "f29ce8f9-42a3-43fa-8197-666cc26e2c76",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// ECL 63 — Pestered Wellguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PESTERED_WELLGUARD: CardRecord = CardRecord::new(
    "Pestered Wellguard",
    "3e06c99e-ecb2-42e9-ac58-2542de8d54a5",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// ECL 64 — Rime Chill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIME_CHILL: CardRecord = CardRecord::new(
    "Rime Chill",
    "a9a425f4-2103-4f96-88a0-91fe554037d7",
    "Igor Krstic",
    crate::card::CardRules::unsupported(),
);

// ECL 65 — Rimefire Torque
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIMEFIRE_TORQUE: CardRecord = CardRecord::new(
    "Rimefire Torque",
    "4f8931b2-ff3f-4167-8a40-f33460c2d27e",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// ECL 66 — Rimekin Recluse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIMEKIN_RECLUSE: CardRecord = CardRecord::new(
    "Rimekin Recluse",
    "ba6b5368-3262-4002-bf1e-fce62f7f7901",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// ECL 67 — Run Away Together (reprint)
const RUN_AWAY_TOGETHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::RUN_AWAY_TOGETHER,
    "35c56aff-1f0f-464a-b705-d67803e3d060",
    "Annie Stegg",
);

// ECL 68 — Shinestriker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHINESTRIKER: CardRecord = CardRecord::new(
    "Shinestriker",
    "214e78f9-5364-49ab-b17e-0c76549c583e",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ECL 69 — Silvergill Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVERGILL_MENTOR: CardRecord = CardRecord::new(
    "Silvergill Mentor",
    "e6e37fe8-459c-4992-8ae9-f782cddab2fe",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// ECL 70 — Silvergill Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVERGILL_PEDDLER: CardRecord = CardRecord::new(
    "Silvergill Peddler",
    "feba2bb6-6005-4ade-a4c5-97bbd54b43a3",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// ECL 71 — Spell Snare (reprint)
const SPELL_SNARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::SPELL_SNARE,
    "b7551b61-656e-4f37-b9da-73174db983b7",
    "Iris Compiet",
);

// ECL 72 — Stratosoarer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRATOSOARER: CardRecord = CardRecord::new(
    "Stratosoarer",
    "4f607889-6f3f-4511-8920-88a39f3b28ca",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// ECL 73 — Summit Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUMMIT_SENTINEL: CardRecord = CardRecord::new(
    "Summit Sentinel",
    "81251057-f270-4f05-9dc5-205c70e1f295",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// ECL 74 — Sunderflock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNDERFLOCK: CardRecord = CardRecord::new(
    "Sunderflock",
    "e5b6221e-cb22-45e4-bb98-2b960afc614c",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// ECL 75 — Swat Away
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWAT_AWAY: CardRecord = CardRecord::new(
    "Swat Away",
    "2fb0ea3f-2f6d-4b64-a9d7-e822c8854a03",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// ECL 76 — Sygg, Wanderwine Wisdom // Sygg, Wanderbrine Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYGG_WANDERWINE_WISDOM: CardRecord = CardRecord::new(
    "Sygg, Wanderwine Wisdom // Sygg, Wanderbrine Shield",
    "70adc870-f0db-4d4b-863b-673c2c258751",
    "Justin Gerard",
    crate::card::CardRules::unsupported(),
);

// ECL 77 — Tanufel Rimespeaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANUFEL_RIMESPEAKER: CardRecord = CardRecord::new(
    "Tanufel Rimespeaker",
    "b357022c-7cd5-4e82-a183-7144f5a84102",
    "Lauren K. Cannon",
    crate::card::CardRules::unsupported(),
);

// ECL 78 — Temporal Cleansing (reprint)
const TEMPORAL_CLEANSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mom::TEMPORAL_CLEANSING,
    "50ee7315-ec53-43d2-841e-8ec192b850f1",
    "Wylie Beckert",
);

// ECL 79 — Thirst for Identity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THIRST_FOR_IDENTITY: CardRecord = CardRecord::new(
    "Thirst for Identity",
    "c3949f8c-d1c5-45c2-80ed-a57f4f9af86e",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// ECL 80 — Unexpected Assistance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNEXPECTED_ASSISTANCE: CardRecord = CardRecord::new(
    "Unexpected Assistance",
    "7b540a5f-7f1b-421a-864f-5af469556fc6",
    "Gustavo Pelissari",
    crate::card::CardRules::unsupported(),
);

// ECL 81 — Unwelcome Sprite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNWELCOME_SPRITE: CardRecord = CardRecord::new(
    "Unwelcome Sprite",
    "902f8d86-fde2-4cdf-88f0-bf63d616f3af",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// ECL 82 — Wanderwine Distracter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERWINE_DISTRACTER: CardRecord = CardRecord::new(
    "Wanderwine Distracter",
    "cbf593a7-d4ae-4771-926a-3c1b2c8c901a",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// ECL 83 — Wanderwine Farewell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERWINE_FAREWELL: CardRecord = CardRecord::new(
    "Wanderwine Farewell",
    "473123a0-1366-406b-9b9e-154c0e9c224f",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// ECL 84 — Wild Unraveling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_UNRAVELING: CardRecord = CardRecord::new(
    "Wild Unraveling",
    "01522fec-9136-4fa6-91a3-370a8bb08b42",
    "Jabari Weathers",
    crate::card::CardRules::unsupported(),
);

// ECL 85 — Auntie's Sentence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUNTIE_S_SENTENCE: CardRecord = CardRecord::new(
    "Auntie's Sentence",
    "e64bfe16-7362-4982-9136-1f4e0d335441",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// ECL 86 — Barbed Bloodletter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_BLOODLETTER: CardRecord = CardRecord::new(
    "Barbed Bloodletter",
    "8ff1a13c-e338-42e9-8eb3-b303fabd67de",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// ECL 87 — Bile-Vial Boggart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BILE_VIAL_BOGGART: CardRecord = CardRecord::new(
    "Bile-Vial Boggart",
    "2c0fcf98-1f3c-4cff-9234-7f3d0c8b22e9",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// ECL 88 — Bitterbloom Bearer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BITTERBLOOM_BEARER: CardRecord = CardRecord::new(
    "Bitterbloom Bearer",
    "7127164d-f2a3-4d79-b6db-93507ff5ab47",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// ECL 89 — Blight Rot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIGHT_ROT: CardRecord = CardRecord::new(
    "Blight Rot",
    "5201bdeb-ba47-459b-ac0d-603367914578",
    "Forrest Schehl",
    crate::card::CardRules::unsupported(),
);

// ECL 90 — Blighted Blackthorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIGHTED_BLACKTHORN: CardRecord = CardRecord::new(
    "Blighted Blackthorn",
    "3515531a-45d6-4fe0-96a3-7ca1ce545068",
    "Omar Rayyan",
    crate::card::CardRules::unsupported(),
);

// ECL 91 — Bloodline Bidding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODLINE_BIDDING: CardRecord = CardRecord::new(
    "Bloodline Bidding",
    "877d9b75-ad2f-45a6-94d8-68e80d7db789",
    "Drew Baker",
    crate::card::CardRules::unsupported(),
);

// ECL 92 — Boggart Mischief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOGGART_MISCHIEF: CardRecord = CardRecord::new(
    "Boggart Mischief",
    "aaeb9c9c-0f15-49dc-ae6d-2a958680f327",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ECL 93 — Boggart Prankster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOGGART_PRANKSTER: CardRecord = CardRecord::new(
    "Boggart Prankster",
    "eb228ea6-9235-4093-aea7-708e743b1b44",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// ECL 94 — Bogslither's Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOGSLITHER_S_EMBRACE: CardRecord = CardRecord::new(
    "Bogslither's Embrace",
    "4beca2e7-9c6d-493b-b15f-69e483a8dfff",
    "Justin Gerard",
    crate::card::CardRules::unsupported(),
);

// ECL 95 — Champion of the Weird
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMPION_OF_THE_WEIRD: CardRecord = CardRecord::new(
    "Champion of the Weird",
    "e600dd01-65ac-489c-a52a-decbc3a9a4f3",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// ECL 96 — Creakwood Safewright
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CREAKWOOD_SAFEWRIGHT: CardRecord = CardRecord::new(
    "Creakwood Safewright",
    "3bcc24cf-776a-4182-bf77-a611ad90b28f",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ECL 97 — Darkness Descends
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARKNESS_DESCENDS: CardRecord = CardRecord::new(
    "Darkness Descends",
    "c8aa3895-df83-4f8e-9e23-1a665614b662",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

// ECL 98 — Dawnhand Dissident
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWNHAND_DISSIDENT: CardRecord = CardRecord::new(
    "Dawnhand Dissident",
    "6ac1f765-f348-4813-88dc-26376e0f3f33",
    "Jacob Walker",
    crate::card::CardRules::unsupported(),
);

// ECL 99 — Dawnhand Eulogist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWNHAND_EULOGIST: CardRecord = CardRecord::new(
    "Dawnhand Eulogist",
    "75a97f69-f3dc-4d33-8008-c1f1a7c15a2f",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// ECL 100 — Dose of Dawnglow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOSE_OF_DAWNGLOW: CardRecord = CardRecord::new(
    "Dose of Dawnglow",
    "47414323-ca30-45b7-a0b2-6668312bee04",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// ECL 101 — Dream Seizer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_SEIZER: CardRecord = CardRecord::new(
    "Dream Seizer",
    "3573f425-9251-4c17-9619-15278ce5d8fb",
    "Omar Rayyan",
    crate::card::CardRules::unsupported(),
);

// ECL 102 — Gloom Ripper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOOM_RIPPER: CardRecord = CardRecord::new(
    "Gloom Ripper",
    "e9ee5d4f-05d1-4e3a-b8d6-f22d0fddedf7",
    "Annie Stegg",
    crate::card::CardRules::unsupported(),
);

// ECL 103 — Gnarlbark Elm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GNARLBARK_ELM: CardRecord = CardRecord::new(
    "Gnarlbark Elm",
    "1e9d65b6-22ff-49f2-8b2a-aeaef91088d3",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// ECL 104 — Graveshifter (reprint)
const GRAVESHIFTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::GRAVESHIFTER,
    "dadb02b9-d3a0-4b51-bbc3-53b2316cd70d",
    "Deborah Garcia",
);

// ECL 105 — Grub, Storied Matriarch // Grub, Notorious Auntie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRUB_STORIED_MATRIARCH: CardRecord = CardRecord::new(
    "Grub, Storied Matriarch // Grub, Notorious Auntie",
    "1f51adf8-8234-4dae-aedf-7633310d5111",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// ECL 106 — Gutsplitter Gang
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUTSPLITTER_GANG: CardRecord = CardRecord::new(
    "Gutsplitter Gang",
    "9ff74349-693d-4373-a194-9796316dd1f1",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// ECL 107 — Heirloom Auntie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEIRLOOM_AUNTIE: CardRecord = CardRecord::new(
    "Heirloom Auntie",
    "cac251b2-d2cc-45b6-9ca8-678e0eab56ea",
    "Raph Lomotan",
    crate::card::CardRules::unsupported(),
);

// ECL 108 — Iron-Shield Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_SHIELD_ELF: CardRecord = CardRecord::new(
    "Iron-Shield Elf",
    "9e0140b2-0185-4adb-b365-2611ce89a0e2",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// ECL 109 — Moonglove Extractor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONGLOVE_EXTRACTOR: CardRecord = CardRecord::new(
    "Moonglove Extractor",
    "8383e0ab-81b4-4a6b-b87b-dd9180dca1c2",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// ECL 110 — Moonshadow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONSHADOW: CardRecord = CardRecord::new(
    "Moonshadow",
    "2573e694-eaa0-42ca-b470-2ab507cbcec1",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// ECL 111 — Mornsong Aria
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORNSONG_ARIA: CardRecord = CardRecord::new(
    "Mornsong Aria",
    "9985c554-8338-46b1-ac36-526d2eb61570",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// ECL 112 — Mudbutton Cursetosser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUDBUTTON_CURSETOSSER: CardRecord = CardRecord::new(
    "Mudbutton Cursetosser",
    "35bc841a-9a21-4c17-a60a-a3ee01472fcb",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// ECL 113 — Nameless Inversion (reprint)
const NAMELESS_INVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::NAMELESS_INVERSION,
    "7a4944ac-839e-4aa1-8200-78cff36bb2ed",
    "Dominik Mayer",
);

// ECL 114 — Nightmare Sower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTMARE_SOWER: CardRecord = CardRecord::new(
    "Nightmare Sower",
    "35dfa0f9-faf3-4a85-b02d-0c5830783511",
    "Tommy Arnold",
    crate::card::CardRules::unsupported(),
);

// ECL 115 — Perfect Intimidation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERFECT_INTIMIDATION: CardRecord = CardRecord::new(
    "Perfect Intimidation",
    "ff05b7f4-e0d0-4304-99b3-66ac804129fe",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ECL 116 — Requiting Hex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REQUITING_HEX: CardRecord = CardRecord::new(
    "Requiting Hex",
    "f21b0fb7-91b6-403f-a81a-562665961276",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ECL 117 — Retched Wretch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETCHED_WRETCH: CardRecord = CardRecord::new(
    "Retched Wretch",
    "c7a4d7f1-976a-4a28-97a9-ff089a241c9d",
    "Raph Lomotan",
    crate::card::CardRules::unsupported(),
);

// ECL 118 — Scarblade Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARBLADE_SCOUT: CardRecord = CardRecord::new(
    "Scarblade Scout",
    "c2412fea-4591-4345-913f-edc2da9ad975",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// ECL 119 — Scarblade's Malice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARBLADE_S_MALICE: CardRecord = CardRecord::new(
    "Scarblade's Malice",
    "aea9b5c0-3b32-44be-9773-566b9daafa6b",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// ECL 120 — Shimmercreep
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERCREEP: CardRecord = CardRecord::new(
    "Shimmercreep",
    "f7c02899-5f0f-4b38-bbbc-fbc8c46419a6",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// ECL 121 — Taster of Wares
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TASTER_OF_WARES: CardRecord = CardRecord::new(
    "Taster of Wares",
    "bc0b64b6-8984-431c-8a2f-84402b429e2b",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// ECL 122 — Twilight Diviner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWILIGHT_DIVINER: CardRecord = CardRecord::new(
    "Twilight Diviner",
    "443b6f30-1493-4d48-93d9-a91e22a7ebb3",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// ECL 123 — Unbury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNBURY: CardRecord = CardRecord::new(
    "Unbury",
    "b00766db-4109-4225-a62a-fa12fd526970",
    "Kev Fang",
    crate::card::CardRules::unsupported(),
);

// ECL 124 — Ashling, Rekindled // Ashling, Rimebound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHLING_REKINDLED: CardRecord = CardRecord::new(
    "Ashling, Rekindled // Ashling, Rimebound",
    "7d7faefe-9c0d-45b6-8ea4-5fa666762a2c",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// ECL 125 — Boldwyr Aggressor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOLDWYR_AGGRESSOR: CardRecord = CardRecord::new(
    "Boldwyr Aggressor",
    "76bbccd2-8a90-49a7-921a-6565b495efdf",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// ECL 126 — Boneclub Berserker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONECLUB_BERSERKER: CardRecord = CardRecord::new(
    "Boneclub Berserker",
    "b3dbbe30-3d6e-46f8-92c1-caee995cba1a",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// ECL 127 — Boulder Dash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOULDER_DASH: CardRecord = CardRecord::new(
    "Boulder Dash",
    "657a2a24-22d1-4bc2-9f22-ed361ae487e3",
    "Chuck Lukacs",
    crate::card::CardRules::unsupported(),
);

// ECL 128 — Brambleback Brute
// Audit: unsupported — Needs a cost that removes a counter of any kind. RemoveCountersFromSource names one kind, and naming -1/-1 would take away the choice the card gives once anything else has put a counter on it.
pub(in crate::card::sets) static BRAMBLEBACK_BRUTE: CardRecord = CardRecord::new(
    "Brambleback Brute",
    "5ebb8365-c6e1-46e8-a242-6aa27b21e68a",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// ECL 129 — Burning Curiosity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNING_CURIOSITY: CardRecord = CardRecord::new(
    "Burning Curiosity",
    "689ed288-6228-4d7e-b198-56a12b8be299",
    "Jim Pavelec",
    crate::card::CardRules::unsupported(),
);

// ECL 130 — Champion of the Path
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMPION_OF_THE_PATH: CardRecord = CardRecord::new(
    "Champion of the Path",
    "e369cd31-3e22-47eb-bf6a-00d823651710",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// ECL 131 — Cinder Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_STRIKE: CardRecord = CardRecord::new(
    "Cinder Strike",
    "6fb6faa4-236c-4cae-9140-0981c44d2392",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// ECL 132 — Collective Inferno
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLECTIVE_INFERNO: CardRecord = CardRecord::new(
    "Collective Inferno",
    "1ec084cc-997d-4079-b445-8f701ec3c277",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// ECL 133 — Elder Auntie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELDER_AUNTIE: CardRecord = CardRecord::new(
    "Elder Auntie",
    "84678e98-2258-4ea1-aaf0-8ac4cc2ecf8d",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// ECL 134 — End-Blaze Epiphany
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static END_BLAZE_EPIPHANY: CardRecord = CardRecord::new(
    "End-Blaze Epiphany",
    "0f0a90ae-b3b3-4f52-8997-eac514b29e57",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// ECL 135 — Enraged Flamecaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENRAGED_FLAMECASTER: CardRecord = CardRecord::new(
    "Enraged Flamecaster",
    "104baa2c-75c5-44fc-a3b9-b19efcf3d7c2",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// ECL 136 — Explosive Prodigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPLOSIVE_PRODIGY: CardRecord = CardRecord::new(
    "Explosive Prodigy",
    "5515ea5e-ce28-4938-a31e-5e48522a5f93",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// ECL 137 — Feed the Flames
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEED_THE_FLAMES: CardRecord = CardRecord::new(
    "Feed the Flames",
    "59740755-c353-4b6c-a84c-3b76133ce3ec",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// ECL 138 — Flame-Chain Mauler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAME_CHAIN_MAULER: CardRecord = CardRecord::new(
    "Flame-Chain Mauler",
    "752d7e8e-0dd0-4ace-9c89-a8f9ce73775e",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// ECL 139 — Flamebraider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMEBRAIDER: CardRecord = CardRecord::new(
    "Flamebraider",
    "b8aa428c-5a77-444f-b75e-a113e46fe4e0",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ECL 140 — Flamekin Gildweaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMEKIN_GILDWEAVER: CardRecord = CardRecord::new(
    "Flamekin Gildweaver",
    "b1628ece-a028-49d4-9065-ee997838c20a",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// ECL 141 — Giantfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANTFALL: CardRecord = CardRecord::new(
    "Giantfall",
    "1ac52728-adb3-4220-8392-73f7bd379ab4",
    "Drew Baker",
    crate::card::CardRules::unsupported(),
);

// ECL 142 — Goatnap (reprint)
const GOATNAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::GOATNAP,
    "7d8dbec5-ae71-4f82-898a-b930ec677403",
    "Vincent Christiaens",
);

// ECL 143 — Goliath Daydreamer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLIATH_DAYDREAMER: CardRecord = CardRecord::new(
    "Goliath Daydreamer",
    "88e8cd13-2a29-4df6-937c-1bed68fbeafa",
    "Omar Rayyan",
    crate::card::CardRules::unsupported(),
);

// ECL 144 — Gristle Glutton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRISTLE_GLUTTON: CardRecord = CardRecord::new(
    "Gristle Glutton",
    "a4164af6-356e-4de2-8377-dbe70434a996",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// ECL 145 — Hexing Squelcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEXING_SQUELCHER: CardRecord = CardRecord::new(
    "Hexing Squelcher",
    "674960ce-ff33-4d5e-a24a-a4582b2e9809",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// ECL 146 — Impolite Entrance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPOLITE_ENTRANCE: CardRecord = CardRecord::new(
    "Impolite Entrance",
    "45be88d8-0be1-47a2-a1c1-d6e693fc706f",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// ECL 147 — Kindle the Inner Flame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINDLE_THE_INNER_FLAME: CardRecord = CardRecord::new(
    "Kindle the Inner Flame",
    "9a2adcea-f6b1-4611-b8b8-f19fdee2c571",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// ECL 148 — Kulrath Zealot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KULRATH_ZEALOT: CardRecord = CardRecord::new(
    "Kulrath Zealot",
    "3502685d-4e57-4c5c-94c6-ae69048cdfbf",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// ECL 149 — Lasting Tarfire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LASTING_TARFIRE: CardRecord = CardRecord::new(
    "Lasting Tarfire",
    "9c4a95ac-072f-4219-80a0-1ce71f1b8411",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// ECL 150 — Lavaleaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVALEAPER: CardRecord = CardRecord::new(
    "Lavaleaper",
    "82902488-d178-4752-bcfb-dd3050654d23",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ECL 151 — Meek Attack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEEK_ATTACK: CardRecord = CardRecord::new(
    "Meek Attack",
    "21157461-5435-4879-80a9-100afc5bbf4c",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// ECL 152 — Reckless Ransacking
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_RANSACKING: CardRecord = CardRecord::new(
    "Reckless Ransacking",
    "24a5b025-4cdb-416d-aad6-0fc7e8da3df2",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ECL 153 — Scuzzback Scrounger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCUZZBACK_SCROUNGER: CardRecord = CardRecord::new(
    "Scuzzback Scrounger",
    "0ea4a895-19c0-47af-ad9c-5db88ea9ae05",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ECL 154 — Sear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEAR: CardRecord = CardRecord::new(
    "Sear",
    "aeb4612c-758b-4492-ba03-eb6741b4176e",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// ECL 155 — Sizzling Changeling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIZZLING_CHANGELING: CardRecord = CardRecord::new(
    "Sizzling Changeling",
    "e58f0722-9ad1-4952-9aee-ea8137c58911",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// ECL 156 — Soul Immolation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_IMMOLATION: CardRecord = CardRecord::new(
    "Soul Immolation",
    "234b70df-8c34-4da7-946e-b8b55a8df390",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ECL 157 — Soulbright Seeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULBRIGHT_SEEKER: CardRecord = CardRecord::new(
    "Soulbright Seeker",
    "895ac890-608a-47de-8bc8-9337fd2064e8",
    "Kev Fang",
    crate::card::CardRules::unsupported(),
);

// ECL 158 — Sourbread Auntie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOURBREAD_AUNTIE: CardRecord = CardRecord::new(
    "Sourbread Auntie",
    "32fd226f-d1c6-432c-b846-9482f1944363",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// ECL 159 — Spinerock Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINEROCK_TYRANT: CardRecord = CardRecord::new(
    "Spinerock Tyrant",
    "478bbb7a-4b96-4e04-921e-bdf23185de25",
    "Cory Godbey",
    crate::card::CardRules::unsupported(),
);

// ECL 160 — Squawkroaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUAWKROASTER: CardRecord = CardRecord::new(
    "Squawkroaster",
    "4112f960-70af-4e2d-bcd4-9e9cf7aac4fb",
    "Alessandra Pisano",
    crate::card::CardRules::unsupported(),
);

// ECL 161 — Sting-Slinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STING_SLINGER: CardRecord = CardRecord::new(
    "Sting-Slinger",
    "386c5f73-fb8f-46c8-ad45-56e2c19b7d1f",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

// ECL 162 — Tweeze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWEEZE: CardRecord = CardRecord::new(
    "Tweeze",
    "3ceab0e6-1bb8-487d-ab4b-2da8457b970a",
    "Scott Gustafson",
    crate::card::CardRules::unsupported(),
);

// ECL 163 — Warren Torchmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARREN_TORCHMASTER: CardRecord = CardRecord::new(
    "Warren Torchmaster",
    "8f067a14-6667-4acf-b33d-e1149188a84d",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// ECL 164 — Assert Perfection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASSERT_PERFECTION: CardRecord = CardRecord::new(
    "Assert Perfection",
    "6995b308-5582-4ca1-ab10-a536d5ca0a6d",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// ECL 165 — Aurora Awakener
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURORA_AWAKENER: CardRecord = CardRecord::new(
    "Aurora Awakener",
    "913977c2-73f9-466b-bd01-827c1736e070",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// ECL 166 — Bloom Tender (reprint)
const BLOOM_TENDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eve::BLOOM_TENDER,
    "ba86688d-18f0-4b5c-a797-42bf125a6c9f",
    "Nils Hamm",
);

// ECL 167 — Blossoming Defense (reprint)
const BLOSSOMING_DEFENSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::BLOSSOMING_DEFENSE,
    "3cce80d0-b937-4325-a603-1278c110f244",
    "Eelis Kyttanen",
);

// ECL 168 — Bristlebane Battler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRISTLEBANE_BATTLER: CardRecord = CardRecord::new(
    "Bristlebane Battler",
    "c857fa32-1b5d-4139-8809-b4d0df44b472",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// ECL 169 — Bristlebane Outrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRISTLEBANE_OUTRIDER: CardRecord = CardRecord::new(
    "Bristlebane Outrider",
    "38b17a3c-4457-47e3-986e-ff0b94c41b1a",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// ECL 170 — Celestial Reunion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_REUNION: CardRecord = CardRecord::new(
    "Celestial Reunion",
    "583b2863-aca1-4dab-9196-ea453b5d9454",
    "Justin Gerard",
    crate::card::CardRules::unsupported(),
);

// ECL 171 — Champions of the Perfect
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMPIONS_OF_THE_PERFECT: CardRecord = CardRecord::new(
    "Champions of the Perfect",
    "4f359211-8be5-4818-b73c-14f24b7ddb21",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// ECL 172 — Chomping Changeling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHOMPING_CHANGELING: CardRecord = CardRecord::new(
    "Chomping Changeling",
    "e187dcc6-19ad-4cf6-94b4-daf07f5144e5",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// ECL 173 — Crossroads Watcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSSROADS_WATCHER: CardRecord = CardRecord::new(
    "Crossroads Watcher",
    "6d62fcbb-f1a0-46ce-a4af-2a33bcc3ac8e",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// ECL 174 — Dawn's Light Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWN_S_LIGHT_ARCHER: CardRecord = CardRecord::new(
    "Dawn's Light Archer",
    "76e80656-6bcb-4d99-8bd2-ca5f75f40daf",
    "Scott Gustafson",
    crate::card::CardRules::unsupported(),
);

// ECL 175 — Dundoolin Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUNDOOLIN_WEAVER: CardRecord = CardRecord::new(
    "Dundoolin Weaver",
    "2260912b-4dfb-49dd-bf95-060d44333645",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// ECL 176 — Formidable Speaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORMIDABLE_SPEAKER: CardRecord = CardRecord::new(
    "Formidable Speaker",
    "265522eb-4f6a-40e7-b374-3833fa63c80b",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// ECL 177 — Gilt-Leaf's Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GILT_LEAF_S_EMBRACE: CardRecord = CardRecord::new(
    "Gilt-Leaf's Embrace",
    "739e5ab5-d562-407a-906e-c5c5173ad325",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// ECL 178 — Great Forest Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREAT_FOREST_DRUID: CardRecord = CardRecord::new(
    "Great Forest Druid",
    "8793a19e-6743-4031-86d9-2ff55f384549",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ECL 179 — Luminollusk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMINOLLUSK: CardRecord = CardRecord::new(
    "Luminollusk",
    "3d591e66-4bf9-47e7-bcef-57769ec3edc6",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

// ECL 180 — Lys Alana Dignitary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LYS_ALANA_DIGNITARY: CardRecord = CardRecord::new(
    "Lys Alana Dignitary",
    "94e8d6a9-7aa3-4e93-8e4d-e50da7ff09d2",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ECL 181 — Lys Alana Informant
pub(in crate::card::sets) static LYS_ALANA_INFORMANT: CardRecord = CardRecord::new(
    "Lys Alana Informant",
    "a79649c4-559e-4306-a102-5fd8750629c7",
    "Sidharth Chaturvedi",
    // A 3/1 that surveils coming and going, so trading it away is still a
    // profitable turn for a deck that wants its graveyard filled.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Scout"], 3, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters or dies, surveil 1. (Look at the top card of your \
             library. You may put it into your graveyard.)",
            // Entering and dying are two ways for one printed ability to
            // fire, so what it does is written once.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ),
);

// ECL 182 — Midnight Tilling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIDNIGHT_TILLING: CardRecord = CardRecord::new(
    "Midnight Tilling",
    "c5112bbf-752c-41e7-9c61-4c81e6a77463",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// ECL 183 — Mistmeadow Council
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTMEADOW_COUNCIL: CardRecord = CardRecord::new(
    "Mistmeadow Council",
    "d4a7c9bc-81c8-4c31-96a9-eb6ba7715e7f",
    "Jim Pavelec",
    crate::card::CardRules::unsupported(),
);

// ECL 184 — Moon-Vigil Adherents
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOON_VIGIL_ADHERENTS: CardRecord = CardRecord::new(
    "Moon-Vigil Adherents",
    "60621c37-62e1-4261-ae76-3946b4a0cfa3",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// ECL 185 — Morcant's Eyes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORCANT_S_EYES: CardRecord = CardRecord::new(
    "Morcant's Eyes",
    "a730b254-ff7c-4f89-a559-b44ad7fd6c6c",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// ECL 186 — Mutable Explorer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUTABLE_EXPLORER: CardRecord = CardRecord::new(
    "Mutable Explorer",
    "8f35d95a-caea-4d5e-b98e-55da1ba7c92d",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// ECL 187 — Pitiless Fists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PITILESS_FISTS: CardRecord = CardRecord::new(
    "Pitiless Fists",
    "295d828b-b2e7-41c7-afbc-5fb5f4eb242c",
    "A. M. Sartor",
    crate::card::CardRules::unsupported(),
);

// ECL 188 — Prismabasher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMABASHER: CardRecord = CardRecord::new(
    "Prismabasher",
    "65c057a7-70af-4464-bd8d-1e7e158d1ae7",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// ECL 189 — Prismatic Undercurrents
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_UNDERCURRENTS: CardRecord = CardRecord::new(
    "Prismatic Undercurrents",
    "bb7490f2-1425-495f-b7d4-2f1e0df7490e",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// ECL 190 — Pummeler for Hire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUMMELER_FOR_HIRE: CardRecord = CardRecord::new(
    "Pummeler for Hire",
    "42208996-7b99-474e-aba7-75190d7ee8e2",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// ECL 191 — Safewright Cavalry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAFEWRIGHT_CAVALRY: CardRecord = CardRecord::new(
    "Safewright Cavalry",
    "f7de412a-9731-4bfc-8fbc-c95988a3dd70",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// ECL 192 — Sapling Nursery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPLING_NURSERY: CardRecord = CardRecord::new(
    "Sapling Nursery",
    "3199bea9-fef7-45fe-8777-2103d84a9347",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// ECL 193 — Selfless Safewright
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SELFLESS_SAFEWRIGHT: CardRecord = CardRecord::new(
    "Selfless Safewright",
    "ac95b1c3-9eb2-4f80-bb32-72b36817d622",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// ECL 194 — Shimmerwilds Growth
pub(in crate::card::sets) static SHIMMERWILDS_GROWTH: CardRecord = CardRecord::new(
    "Shimmerwilds Growth",
    "c122719c-f0d1-4170-a0d1-d62172df1d21",
    "Jorge Jacinto",
CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::as_enters(
                "As this Aura enters, choose a color.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::COLOR,
                )),
            ),
            AbilityDef::static_ability(
                "Enchanted land is the chosen color.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::set_color(ManaTypeDef::ChosenColor),
                },
            ),
            AbilityDef::triggered_mana(
                "Whenever enchanted land is tapped for mana, its controller adds an additional one mana of the chosen color.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddMana(
                    AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)
                        .to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// ECL 195 — Spry and Mighty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPRY_AND_MIGHTY: CardRecord = CardRecord::new(
    "Spry and Mighty",
    "152b7374-e991-443f-b6ca-914415635c4a",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ECL 196 — Surly Farrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURLY_FARRIER: CardRecord = CardRecord::new(
    "Surly Farrier",
    "056f7f51-18a9-4d80-8928-decaf4d12c0d",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// ECL 197 — Tend the Sprigs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEND_THE_SPRIGS: CardRecord = CardRecord::new(
    "Tend the Sprigs",
    "388f6d9d-bb9a-4a3d-93c5-701db194863c",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// ECL 198 — Thoughtweft Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTWEFT_CHARGE: CardRecord = CardRecord::new(
    "Thoughtweft Charge",
    "19cfc015-b6d5-4918-99a7-990209e77441",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// ECL 199 — Trystan, Callous Cultivator // Trystan, Penitent Culler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRYSTAN_CALLOUS_CULTIVATOR: CardRecord = CardRecord::new(
    "Trystan, Callous Cultivator // Trystan, Penitent Culler",
    "2094f9b6-ce84-47d9-8819-81db14ba483f",
    "Annie Stegg",
    crate::card::CardRules::unsupported(),
);

// ECL 200 — Unforgiving Aim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNFORGIVING_AIM: CardRecord = CardRecord::new(
    "Unforgiving Aim",
    "0bef3905-24f8-419b-aeca-396adfc6d0dc",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// ECL 201 — Vinebred Brawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINEBRED_BRAWLER: CardRecord = CardRecord::new(
    "Vinebred Brawler",
    "63c573fe-e74c-48ca-ad05-92f37dc466f1",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// ECL 202 — Virulent Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIRULENT_EMISSARY: CardRecord = CardRecord::new(
    "Virulent Emissary",
    "0702efed-915e-466a-96bb-ac09af06b21e",
    "Tiffany Turrill",
    crate::card::CardRules::unsupported(),
);

// ECL 203 — Wildvine Pummeler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILDVINE_PUMMELER: CardRecord = CardRecord::new(
    "Wildvine Pummeler",
    "11bad5c7-fe9a-4d89-a531-8d4f03d5a0e4",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ECL 204 — Abigale, Eloquent First-Year
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABIGALE_ELOQUENT_FIRST_YEAR: CardRecord = CardRecord::new(
    "Abigale, Eloquent First-Year",
    "bf708169-a307-494b-b8d8-baae53b2e2f2",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ECL 205 — Ashling's Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHLING_S_COMMAND: CardRecord = CardRecord::new(
    "Ashling's Command",
    "fe3a421d-07b6-4b94-b177-aec44c7fe689",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// ECL 206 — Boggart Cursecrafter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOGGART_CURSECRAFTER: CardRecord = CardRecord::new(
    "Boggart Cursecrafter",
    "b0b67eb9-0d88-4f2c-8063-e8bedfa78556",
    "Alex Stone",
    crate::card::CardRules::unsupported(),
);

// ECL 207 — Bre of Clan Stoutarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRE_OF_CLAN_STOUTARM: CardRecord = CardRecord::new(
    "Bre of Clan Stoutarm",
    "013cefb7-a059-45c2-81b0-187f35aac4a2",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// ECL 208 — Brigid's Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIGID_S_COMMAND: CardRecord = CardRecord::new(
    "Brigid's Command",
    "cf034777-3da7-4d5c-9213-5e9d235c315a",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// ECL 209 — Catharsis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATHARSIS: CardRecord = CardRecord::new(
    "Catharsis",
    "affb4500-2704-49fc-bbb4-02ed4bfb3b76",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// ECL 210 — Chaos Spewer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_SPEWER: CardRecord = CardRecord::new(
    "Chaos Spewer",
    "b5918fa5-1d13-447b-8838-633b6b61e791",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// ECL 211 — Chitinous Graspling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHITINOUS_GRASPLING: CardRecord = CardRecord::new(
    "Chitinous Graspling",
    "a9767360-d536-4902-9d2d-1f3474ce89d6",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ECL 212 — Deceit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECEIT: CardRecord = CardRecord::new(
    "Deceit",
    "bd82c9e4-9871-4e6d-b691-ee00b4b9a3c6",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// ECL 213 — Deepchannel Duelist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPCHANNEL_DUELIST: CardRecord = CardRecord::new(
    "Deepchannel Duelist",
    "1b742172-7118-45e7-9945-62bd77d94e85",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ECL 214 — Deepway Navigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPWAY_NAVIGATOR: CardRecord = CardRecord::new(
    "Deepway Navigator",
    "d988e28b-fa60-4b60-8229-7a15932c784b",
    "Jacob Walker",
    crate::card::CardRules::unsupported(),
);

// ECL 215 — Doran, Besieged by Time
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DORAN_BESIEGED_BY_TIME: CardRecord = CardRecord::new(
    "Doran, Besieged by Time",
    "568aa70a-6765-486a-bd37-5d38b16c46de",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ECL 216 — Dream Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_HARVEST: CardRecord = CardRecord::new(
    "Dream Harvest",
    "a4ebc0e7-02d7-4d38-9376-a39963e6d3fa",
    "Ben Hill",
    crate::card::CardRules::unsupported(),
);

// ECL 217 — Eclipsed Boggart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECLIPSED_BOGGART: CardRecord = CardRecord::new(
    "Eclipsed Boggart",
    "0ac1d0b2-92a4-4a10-b2d1-e9bb90265cc3",
    "Tiffany Turrill",
    crate::card::CardRules::unsupported(),
);

// ECL 218 — Eclipsed Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECLIPSED_ELF: CardRecord = CardRecord::new(
    "Eclipsed Elf",
    "9c8579f3-6125-4f22-b1c2-b7a0cfc50eed",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// ECL 219 — Eclipsed Flamekin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECLIPSED_FLAMEKIN: CardRecord = CardRecord::new(
    "Eclipsed Flamekin",
    "d907ae44-cd07-4409-946c-e97f584d9a81",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// ECL 220 — Eclipsed Kithkin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECLIPSED_KITHKIN: CardRecord = CardRecord::new(
    "Eclipsed Kithkin",
    "29e1cfa4-0ad8-4228-9f7e-cbab114d1d5f",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// ECL 221 — Eclipsed Merrow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECLIPSED_MERROW: CardRecord = CardRecord::new(
    "Eclipsed Merrow",
    "2352750d-404d-4928-9bdb-1b0db599b70f",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// ECL 222 — Emptiness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMPTINESS: CardRecord = CardRecord::new(
    "Emptiness",
    "c6409eca-bef6-4f3a-8bbb-d69ec5dbfc13",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// ECL 223 — Feisty Spikeling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEISTY_SPIKELING: CardRecord = CardRecord::new(
    "Feisty Spikeling",
    "f69f3a27-ecda-4d27-82fe-612ed57dbb28",
    "Tiffany Turrill",
    crate::card::CardRules::unsupported(),
);

// ECL 224 — Figure of Fable
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIGURE_OF_FABLE: CardRecord = CardRecord::new(
    "Figure of Fable",
    "e0ef33dd-5f6d-48fa-8ef6-a8092868d50f",
    "Omar Rayyan",
    crate::card::CardRules::unsupported(),
);

// ECL 225 — Flaring Cinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLARING_CINDER: CardRecord = CardRecord::new(
    "Flaring Cinder",
    "0691a1a9-18e7-44b7-9a34-764b1ab45a76",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// ECL 226 — Gangly Stompling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANGLY_STOMPLING: CardRecord = CardRecord::new(
    "Gangly Stompling",
    "502000a7-a3c1-4259-aea5-ff01724396a1",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// ECL 227 — Glister Bairn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLISTER_BAIRN: CardRecord = CardRecord::new(
    "Glister Bairn",
    "dcccdd98-a1ba-41f4-a9ae-84d263da0af9",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// ECL 228 — Grub's Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRUB_S_COMMAND: CardRecord = CardRecord::new(
    "Grub's Command",
    "73013908-feff-495a-a62e-1e19548ffe6f",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// ECL 229 — High Perfect Morcant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_PERFECT_MORCANT: CardRecord = CardRecord::new(
    "High Perfect Morcant",
    "dfe7b8bf-c150-4be0-aef4-e8bb6f09787a",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// ECL 230 — Hovel Hurler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOVEL_HURLER: CardRecord = CardRecord::new(
    "Hovel Hurler",
    "adc6a4c5-4e92-43ee-8d0c-204042965eb7",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// ECL 231 — Kirol, Attentive First-Year
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KIROL_ATTENTIVE_FIRST_YEAR: CardRecord = CardRecord::new(
    "Kirol, Attentive First-Year",
    "c11cb0eb-819e-4905-ad95-e43618d3c81e",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// ECL 232 — Lluwen, Imperfect Naturalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLUWEN_IMPERFECT_NATURALIST: CardRecord = CardRecord::new(
    "Lluwen, Imperfect Naturalist",
    "127a30a6-c25a-448a-a242-dc04f273a854",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// ECL 233 — Maralen, Fae Ascendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARALEN_FAE_ASCENDANT: CardRecord = CardRecord::new(
    "Maralen, Fae Ascendant",
    "c50f5408-5b5c-41dc-807e-136233403a09",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// ECL 234 — Merrow Skyswimmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERROW_SKYSWIMMER: CardRecord = CardRecord::new(
    "Merrow Skyswimmer",
    "075b419a-fd44-4a9a-8c40-474562b7e11a",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ECL 235 — Mischievous Sneakling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISCHIEVOUS_SNEAKLING: CardRecord = CardRecord::new(
    "Mischievous Sneakling",
    "3b66fa00-2fa6-4060-9e7f-8e3fde6deb73",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ECL 236 — Morcant's Loyalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORCANT_S_LOYALIST: CardRecord = CardRecord::new(
    "Morcant's Loyalist",
    "175c6b1c-8790-41a2-ae15-5031206c410f",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// ECL 237 — Noggle Robber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOGGLE_ROBBER: CardRecord = CardRecord::new(
    "Noggle Robber",
    "0082ca96-10f3-4823-be16-117556b2afc3",
    "Steve Ellis",
    crate::card::CardRules::unsupported(),
);

// ECL 238 — Prideful Feastling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIDEFUL_FEASTLING: CardRecord = CardRecord::new(
    "Prideful Feastling",
    "7578cb61-f606-4559-a10a-343a583228ab",
    "Raph Lomotan",
    crate::card::CardRules::unsupported(),
);

// ECL 239 — Raiding Schemes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAIDING_SCHEMES: CardRecord = CardRecord::new(
    "Raiding Schemes",
    "0bab5da5-72a7-4340-9b53-492ab14a9f71",
    "Justin Gerard",
    crate::card::CardRules::unsupported(),
);

// ECL 240 — Reaping Willow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REAPING_WILLOW: CardRecord = CardRecord::new(
    "Reaping Willow",
    "97c33cc2-3573-410a-908b-c0392fff524b",
    "Igor Krstic",
    crate::card::CardRules::unsupported(),
);

// ECL 241 — Sanar, Innovative First-Year
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANAR_INNOVATIVE_FIRST_YEAR: CardRecord = CardRecord::new(
    "Sanar, Innovative First-Year",
    "11215561-bbcd-4564-a2e4-a1d77d177a1d",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// ECL 242 — Shadow Urchin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOW_URCHIN: CardRecord = CardRecord::new(
    "Shadow Urchin",
    "4e54c39b-6149-467b-a9a8-7ad09ca0cbd4",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ECL 243 — Stoic Grove-Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOIC_GROVE_GUIDE: CardRecord = CardRecord::new(
    "Stoic Grove-Guide",
    "3d5a3e25-c17a-47b1-a36d-d24d50e5bab3",
    "Tran Nguyen",
    crate::card::CardRules::unsupported(),
);

// ECL 244 — Sygg's Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYGG_S_COMMAND: CardRecord = CardRecord::new(
    "Sygg's Command",
    "8bc13fc2-e254-4db5-ad4d-f92711a1a6ca",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ECL 245 — Tam, Mindful First-Year
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAM_MINDFUL_FIRST_YEAR: CardRecord = CardRecord::new(
    "Tam, Mindful First-Year",
    "6cb0f825-b75b-4f2a-803c-08142ca07e76",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// ECL 246 — Thoughtweft Lieutenant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTWEFT_LIEUTENANT: CardRecord = CardRecord::new(
    "Thoughtweft Lieutenant",
    "2c54ec67-9317-455e-a045-fa4ed9cb676f",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// ECL 247 — Trystan's Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRYSTAN_S_COMMAND: CardRecord = CardRecord::new(
    "Trystan's Command",
    "ba1d99ca-740a-481a-be89-615e40d56d06",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// ECL 248 — Twinflame Travelers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWINFLAME_TRAVELERS: CardRecord = CardRecord::new(
    "Twinflame Travelers",
    "5fc9f409-3aef-4403-bf17-6e9a72ecfada",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// ECL 249 — Vibrance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIBRANCE: CardRecord = CardRecord::new(
    "Vibrance",
    "b9f71c3b-0840-475f-9c17-fdacbc7f3213",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// ECL 250 — Voracious Tome-Skimmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VORACIOUS_TOME_SKIMMER: CardRecord = CardRecord::new(
    "Voracious Tome-Skimmer",
    "1a696fe5-410c-4699-b1d1-1a9c771d664a",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// ECL 251 — Wary Farmer
// Audit: unsupported — Needs an event-tracked "a creature entered under your control this turn" condition. The only available reading is ObjectPredicateDef::EnteredThisTurn over the battlefield, which misses a creature that entered and left before the end step -- the printed intervening-if would still be satisfied. The condition vocabulary already tracks comparable events (CreatureDiedThisTurn, ControllerHadPermanentLeaveThisTurn) but not this one.
pub(in crate::card::sets) static WARY_FARMER: CardRecord = CardRecord::new(
    "Wary Farmer",
    "22d20c0d-176d-49c9-aa0b-2c5778548cc5",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ECL 252 — Wistfulness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WISTFULNESS: CardRecord = CardRecord::new(
    "Wistfulness",
    "db9aa986-ac2a-44bb-a88b-04c5d0d502b2",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// ECL 253 — Chronicle of Victory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHRONICLE_OF_VICTORY: CardRecord = CardRecord::new(
    "Chronicle of Victory",
    "b3c2d68d-690b-41e7-99ed-2d20c7e0a9b4",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// ECL 254 — Dawn-Blessed Pennant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWN_BLESSED_PENNANT: CardRecord = CardRecord::new(
    "Dawn-Blessed Pennant",
    "294266b6-0343-4fa5-90b8-0adf7df490e4",
    "Igor Krstic",
    crate::card::CardRules::unsupported(),
);

// ECL 255 — Firdoch Core
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRDOCH_CORE: CardRecord = CardRecord::new(
    "Firdoch Core",
    "8e45cd37-bf97-4742-978d-96f96ed653cd",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// ECL 256 — Foraging Wickermaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORAGING_WICKERMAW: CardRecord = CardRecord::new(
    "Foraging Wickermaw",
    "f524bc08-caeb-4362-b960-eb8e0e4159d0",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ECL 257 — Gathering Stone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GATHERING_STONE: CardRecord = CardRecord::new(
    "Gathering Stone",
    "81dfbfe1-143a-4637-b683-a34cfc51993d",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// ECL 258 — Mirrormind Crown
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRORMIND_CROWN: CardRecord = CardRecord::new(
    "Mirrormind Crown",
    "061d765e-df27-406a-9ba0-b51b0cbb65da",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ECL 259 — Puca's Eye
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUCA_S_EYE: CardRecord = CardRecord::new(
    "Puca's Eye",
    "c3a5ca02-3829-4c42-b0dc-98b660f8a8f0",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ECL 260 — Springleaf Drum (reprint)
const SPRINGLEAF_DRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::SPRINGLEAF_DRUM,
    "e15ab0aa-4059-4923-9816-6f7a9e5b5a18",
    "Cory Godbey",
);

// ECL 261 — Stalactite Dagger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALACTITE_DAGGER: CardRecord = CardRecord::new(
    "Stalactite Dagger",
    "6954df09-95f3-46cf-9ba8-2a1aea653d8f",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ECL 262 — Blood Crypt (reprint)
const BLOOD_CRYPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::BLOOD_CRYPT,
    "6da63cc5-4624-4491-abd9-9b600c3fefe2",
    "Adam Paquette",
);

// ECL 263 — Eclipsed Realms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECLIPSED_REALMS: CardRecord = CardRecord::new(
    "Eclipsed Realms",
    "a174f0db-8b4f-4c37-9583-44c92d37b9c0",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// ECL 264 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::EVOLVING_WILDS,
    "8c632984-5176-4c37-91df-6577cc294b85",
    "Alayna Danner",
);

// ECL 265 — Hallowed Fountain (reprint)
const HALLOWED_FOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::HALLOWED_FOUNTAIN,
    "e056b55f-82ed-4fe0-ab0c-bb20fa4a218a",
    "Adam Paquette",
);

// ECL 266 — Overgrown Tomb (reprint)
const OVERGROWN_TOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::OVERGROWN_TOMB,
    "45b92924-baa1-4c9b-9932-9a5eda8f3446",
    "Adam Paquette",
);

// ECL 267 — Steam Vents (reprint)
const STEAM_VENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::STEAM_VENTS,
    "b66daa94-d367-4812-9f18-f35378c1febb",
    "Adam Paquette",
);

// ECL 268 — Temple Garden (reprint)
const TEMPLE_GARDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::TEMPLE_GARDEN,
    "6cdd2a74-63b3-4ff2-9c5a-a85dee63c3c9",
    "Adam Paquette",
);

// ECL 269 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "3a438199-54f8-4702-81cf-a9d42e7cd9f1",
    "Zoltan Boros",
);

// ECL 270 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "b0da67fb-1cb2-4105-ab5c-b7c680b8116c",
    "Ron Spears",
);

// ECL 271 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "1dd4d605-02a2-4183-b191-0bca8dfbf962",
    "Jorge Jacinto",
);

// ECL 272 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "295b92bc-d66f-45d8-9bbe-5f5f13e39fd4",
    "Raymond Bonilla",
);

// ECL 273 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "b460f5f7-c7c9-400c-8419-23d614f45bf9",
    "Jorge Jacinto",
);

// ECL 274 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "6242dbef-8412-4ebd-9486-42cec1dc6794",
    "Justin Gerard",
);

// ECL 275 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "12ebd9c8-6587-456e-aba9-7aad1c2a09ea",
    "Annie Stegg",
);

// ECL 276 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "0cf904da-496c-4d88-a62b-c736ba895078",
    "Raoul Vitale",
);

// ECL 277 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "9b6fe3e2-a2cf-4209-ac9b-e04d02599360",
    "Ralph Horsley",
);

// ECL 278 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "b43297de-7378-474f-a85f-910fa7cbb4f4",
    "Jason Mowry",
);

// ECL 279 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "d8fdfa7d-fd11-4c11-8743-a21538474314",
    "Justin Gerard",
);

// ECL 280 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "d6a5ba11-3156-4a0c-958d-5756e18b767b",
    "Annie Stegg",
);

// ECL 281 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "35fe42f9-dd55-4ca4-af0a-59ecdff0dba8",
    "Raoul Vitale",
);

// ECL 282 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "96e14936-5614-46ba-874c-c1357243fe02",
    "Ralph Horsley",
);

// ECL 283 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "bdbce923-c05e-4554-8c4c-5c4e6d791856",
    "Jason Mowry",
);

// ECL 284 — Ajani, Outland Chaperone (alternate printing)
const AJANI_OUTLAND_CHAPERONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AJANI_OUTLAND_CHAPERONE,
    1,
    "b5734a2c-0e8f-4087-97ee-51e26cfbf62d",
    "Greg Staples",
);

// ECL 285 — Brigid, Clachan's Heart // Brigid, Doun's Mind (alternate printing)
const BRIGID_CLACHAN_S_HEART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRIGID_CLACHAN_S_HEART,
    1,
    "887002ae-794b-427c-9f98-e909b22313d9",
    "Jesper Ejsing",
);

// ECL 286 — Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight (alternate printing)
const EIRDU_CARRIER_OF_DAWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EIRDU_CARRIER_OF_DAWN,
    1,
    "ec3c80c4-4935-455c-a14f-a0532b6a41a8",
    "Omar Rayyan",
);

// ECL 287 — Oko, Lorwyn Liege // Oko, Shadowmoor Scion (alternate printing)
const OKO_LORWYN_LIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OKO_LORWYN_LIEGE,
    1,
    "48d43194-cbcb-472f-b1c8-b97e347b654c",
    "Steve Prescott",
);

// ECL 288 — Sygg, Wanderwine Wisdom // Sygg, Wanderbrine Shield (alternate printing)
const SYGG_WANDERWINE_WISDOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYGG_WANDERWINE_WISDOM,
    1,
    "62b5ab41-7a85-49aa-8669-8a09a09d02fa",
    "Warren Mahy",
);

// ECL 289 — Grub, Storied Matriarch // Grub, Notorious Auntie (alternate printing)
const GRUB_STORIED_MATRIARCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRUB_STORIED_MATRIARCH,
    1,
    "0f836c3b-d1fb-4b3b-9c14-cc9052be3a94",
    "Zoltan Boros",
);

// ECL 290 — Ashling, Rekindled // Ashling, Rimebound (alternate printing)
const ASHLING_REKINDLED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHLING_REKINDLED,
    1,
    "7110cae6-b0ed-4495-9f36-751ce25c9538",
    "Chuck Lukacs",
);

// ECL 291 — Trystan, Callous Cultivator // Trystan, Penitent Culler (alternate printing)
const TRYSTAN_CALLOUS_CULTIVATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRYSTAN_CALLOUS_CULTIVATOR,
    1,
    "4086c9a8-f9bf-4359-a67a-9f7b45528ccf",
    "Larry MacDougall",
);

// ECL 292 — Catharsis (alternate printing)
const CATHARSIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CATHARSIS,
    1,
    "f7604799-1d32-4c6c-92c6-0d1b274a1115",
    "Wayne Reynolds",
);

// ECL 293 — Deceit (alternate printing)
const DECEIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DECEIT,
    1,
    "0a1c101b-3fad-421c-a081-69e75db95c46",
    "Kev Walker",
);

// ECL 294 — Emptiness (alternate printing)
const EMPTINESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMPTINESS,
    1,
    "9f53e76d-3038-41f7-b050-bb9d7c7d2ea3",
    "Jeff Miracola",
);

// ECL 295 — Vibrance (alternate printing)
const VIBRANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIBRANCE,
    1,
    "ea49c01f-3f32-4c66-893c-56931e0e8981",
    "Mark Zug",
);

// ECL 296 — Wistfulness (alternate printing)
const WISTFULNESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WISTFULNESS,
    1,
    "280eb666-e918-4640-9c43-19bc71af06b6",
    "Paolo Parente",
);

// ECL 297 — Adept Watershaper (alternate printing)
const ADEPT_WATERSHAPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ADEPT_WATERSHAPER,
    1,
    "8700c0ce-2a71-4bc8-a14c-685b5d188b54",
    "Julie Benbassat",
);

// ECL 298 — Curious Colossus (alternate printing)
const CURIOUS_COLOSSUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CURIOUS_COLOSSUS,
    1,
    "aef1f79b-c536-4bf3-ab71-0a812c4c0728",
    "Hayden Goodman",
);

// ECL 299 — Kinbinding (alternate printing)
const KINBINDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KINBINDING,
    1,
    "c6ef5697-35ef-4603-92b9-ed1993f5d401",
    "Yas Imamura",
);

// ECL 300 — Kinscaer Sentry (alternate printing)
const KINSCAER_SENTRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KINSCAER_SENTRY,
    1,
    "bf7ab51b-301e-48fe-b1e8-0c33d057e7de",
    "Felicita Sala",
);

// ECL 301 — Morningtide's Light (alternate printing)
const MORNINGTIDE_S_LIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORNINGTIDE_S_LIGHT,
    1,
    "d0de86f5-4dd5-415f-ba9a-6321986fc9e2",
    "adelinaillustration",
);

// ECL 302 — Slumbering Walker (alternate printing)
const SLUMBERING_WALKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLUMBERING_WALKER,
    1,
    "69f6248e-d8d0-40eb-a89e-bc3017327711",
    "Rebecca Green",
);

// ECL 303 — Disruptor of Currents (alternate printing)
const DISRUPTOR_OF_CURRENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DISRUPTOR_OF_CURRENTS,
    1,
    "29c2c66e-a3d3-4993-82cc-adf8f6877204",
    "Julie Benbassat",
);

// ECL 304 — Flitterwing Nuisance (alternate printing)
const FLITTERWING_NUISANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FLITTERWING_NUISANCE,
    1,
    "7ecc95a7-51c8-45a1-bebf-9dfa053f7403",
    "Serena Malyon",
);

// ECL 305 — Glen Elendra Guardian (alternate printing)
const GLEN_ELENDRA_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_GUARDIAN,
    1,
    "8b8996a5-4d22-42d3-a706-44c258c938e9",
    "Danny Schwartz",
);

// ECL 306 — Glen Elendra's Answer (alternate printing)
const GLEN_ELENDRA_S_ANSWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_S_ANSWER,
    1,
    "8375ad0c-5d8d-47f6-bd04-ffd59fa9c0be",
    "Matthew Forsythe",
);

// ECL 307 — Loch Mare (alternate printing)
const LOCH_MARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOCH_MARE,
    1,
    "539a7e8f-97a9-4aa2-957b-0cc30de797bb",
    "Isabella Mazzanti",
);

// ECL 308 — Mirrorform (alternate printing)
const MIRRORFORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIRRORFORM,
    1,
    "02a9adde-1604-46c1-b481-fa0a73c72bd9",
    "Felicita Sala",
);

// ECL 309 — Sunderflock (alternate printing)
const SUNDERFLOCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNDERFLOCK,
    1,
    "ee7486fc-844d-4515-9028-8d53ba396a61",
    "Danny Schwartz",
);

// ECL 310 — Bitterbloom Bearer (alternate printing)
const BITTERBLOOM_BEARER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BITTERBLOOM_BEARER,
    1,
    "58aa273f-9ec7-4024-b30e-e75fa404de72",
    "Taryn Knight",
);

// ECL 311 — Dawnhand Dissident (alternate printing)
const DAWNHAND_DISSIDENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAWNHAND_DISSIDENT,
    1,
    "fea91416-2836-447f-ad07-8b96ba727cbd",
    "Lauren Degraaf",
);

// ECL 312 — Gloom Ripper (alternate printing)
const GLOOM_RIPPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLOOM_RIPPER,
    1,
    "d81ca098-2dca-48a2-876d-06e89c2b1d1c",
    "Isabella Mazzanti",
);

// ECL 313 — Moonshadow (alternate printing)
const MOONSHADOW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOONSHADOW,
    1,
    "4366c1d7-d5b7-4f63-94fa-9d0e5edd9a73",
    "Julie Benbassat",
);

// ECL 314 — Taster of Wares (alternate printing)
const TASTER_OF_WARES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TASTER_OF_WARES,
    1,
    "8a6c2e99-2d4c-455a-a394-55ff32a95142",
    "Felicita Sala",
);

// ECL 315 — Twilight Diviner (alternate printing)
const TWILIGHT_DIVINER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWILIGHT_DIVINER,
    1,
    "40e615a7-93a9-4753-a96c-a18237ab240b",
    "Isabella Mazzanti",
);

// ECL 316 — Goliath Daydreamer (alternate printing)
const GOLIATH_DAYDREAMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOLIATH_DAYDREAMER,
    1,
    "b5e1905d-4fd8-4923-8612-75678aa3f212",
    "Vanessa Gillings",
);

// ECL 317 — Hexing Squelcher (alternate printing)
const HEXING_SQUELCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEXING_SQUELCHER,
    1,
    "68618675-3e00-4b07-b1da-0e4be5700a1c",
    "Matthew Forsythe",
);

// ECL 318 — Lavaleaper (alternate printing)
const LAVALEAPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAVALEAPER,
    1,
    "0eee01c8-6588-407c-9e19-97699e43e713",
    "Matthew Forsythe",
);

// ECL 319 — Meek Attack (alternate printing)
const MEEK_ATTACK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEEK_ATTACK,
    1,
    "7d6a0d52-08af-45d1-9c4c-3d694c40b372",
    "Matt Rockefeller",
);

// ECL 320 — Scuzzback Scrounger (alternate printing)
const SCUZZBACK_SCROUNGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCUZZBACK_SCROUNGER,
    1,
    "9377b69c-9677-4865-a0c4-af0bfd7ebfd5",
    "Matthew Forsythe",
);

// ECL 321 — Soul Immolation (alternate printing)
const SOUL_IMMOLATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOUL_IMMOLATION,
    1,
    "c79d97c0-76aa-439c-b47d-dc4b00ffda96",
    "Serena Malyon",
);

// ECL 322 — Spinerock Tyrant (alternate printing)
const SPINEROCK_TYRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPINEROCK_TYRANT,
    1,
    "65825a3c-a442-476d-b594-e61d6104dffa",
    "Danny Schwartz",
);

// ECL 323 — Aurora Awakener (alternate printing)
const AURORA_AWAKENER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AURORA_AWAKENER,
    1,
    "9c87e3f0-4305-4b07-a92b-b61430e80e97",
    "Matthew Forsythe",
);

// ECL 324 — Bloom Tender (alternate printing)
const BLOOM_TENDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_eve::BLOOM_TENDER,
    1,
    "46aa1d12-ee23-4b0f-b574-7f1a625ca279",
    "Danny Schwartz",
);

// ECL 325 — Bristlebane Battler (alternate printing)
const BRISTLEBANE_BATTLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRISTLEBANE_BATTLER,
    1,
    "7028368a-5711-416c-b5d9-874ef373335d",
    "Matt Rockefeller",
);

// ECL 326 — Celestial Reunion (alternate printing)
const CELESTIAL_REUNION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CELESTIAL_REUNION,
    1,
    "e9777cbc-064c-4c11-9a43-8d44f7e8e403",
    "Serena Malyon",
);

// ECL 327 — Mutable Explorer (alternate printing)
const MUTABLE_EXPLORER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MUTABLE_EXPLORER,
    1,
    "8d10a060-5abd-4982-996c-a8fc420e44ba",
    "Felicita Sala",
);

// ECL 328 — Sapling Nursery (alternate printing)
const SAPLING_NURSERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAPLING_NURSERY,
    1,
    "ff984cb9-cd72-47a4-adce-2521c025679b",
    "Julie Benbassat",
);

// ECL 329 — Spry and Mighty (alternate printing)
const SPRY_AND_MIGHTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPRY_AND_MIGHTY,
    1,
    "ddc88ce9-716c-4328-95ec-f1c336d04c33",
    "Matthew Forsythe",
);

// ECL 330 — Ashling's Command (alternate printing)
const ASHLING_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHLING_S_COMMAND,
    1,
    "201c73b6-368f-4dac-b034-49c6aa6408ff",
    "adelinaillustration",
);

// ECL 331 — Boggart Cursecrafter (alternate printing)
const BOGGART_CURSECRAFTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOGGART_CURSECRAFTER,
    1,
    "a942f56d-69ee-4151-baae-9a718ff6e7b6",
    "Heikala",
);

// ECL 332 — Brigid's Command (alternate printing)
const BRIGID_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRIGID_S_COMMAND,
    1,
    "70b67bc1-0535-48bb-9f5c-4f43ddeb16cb",
    "Rebecca Green",
);

// ECL 333 — Deepchannel Duelist (alternate printing)
const DEEPCHANNEL_DUELIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEPCHANNEL_DUELIST,
    1,
    "0fe0730e-a549-427d-99be-68d6fded5ac5",
    "Lauren Degraaf",
);

// ECL 334 — Doran, Besieged by Time (alternate printing)
const DORAN_BESIEGED_BY_TIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DORAN_BESIEGED_BY_TIME,
    1,
    "9f0a1644-58bb-4826-9832-0ffc372fac1e",
    "Serena Malyon",
);

// ECL 335 — Eclipsed Boggart (alternate printing)
const ECLIPSED_BOGGART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_BOGGART,
    1,
    "343a04c9-ebf8-41d9-9ebf-9d17da5ad605",
    "Yas Imamura",
);

// ECL 336 — Eclipsed Elf (alternate printing)
const ECLIPSED_ELF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_ELF,
    1,
    "57708ef7-ef10-49c4-8cf7-7055cf7280a0",
    "Heikala",
);

// ECL 337 — Eclipsed Flamekin (alternate printing)
const ECLIPSED_FLAMEKIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_FLAMEKIN,
    1,
    "6271b82d-5922-4de0-b763-129908ece6ee",
    "Vanessa Gillings",
);

// ECL 338 — Eclipsed Kithkin (alternate printing)
const ECLIPSED_KITHKIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_KITHKIN,
    1,
    "e40d5d7c-4176-4faa-9701-933307eebb26",
    "Yas Imamura",
);

// ECL 339 — Eclipsed Merrow (alternate printing)
const ECLIPSED_MERROW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_MERROW,
    1,
    "45d96555-4976-49d1-b529-09cc7ca37cd3",
    "Felicita Sala",
);

// ECL 340 — Grub's Command (alternate printing)
const GRUB_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRUB_S_COMMAND,
    1,
    "54a38e72-b308-410f-b182-454c935e2242",
    "Phoebe Wahl",
);

// ECL 341 — Morcant's Loyalist (alternate printing)
const MORCANT_S_LOYALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORCANT_S_LOYALIST,
    1,
    "c157b97e-905e-4c68-b6ae-9c43caa7c6e7",
    "Yas Imamura",
);

// ECL 342 — Sygg's Command (alternate printing)
const SYGG_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYGG_S_COMMAND,
    1,
    "a2cb17e2-982e-4914-ab09-6edcf2546509",
    "Felicita Sala",
);

// ECL 343 — Thoughtweft Lieutenant (alternate printing)
const THOUGHTWEFT_LIEUTENANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THOUGHTWEFT_LIEUTENANT,
    1,
    "fea71783-1ac5-483b-8a91-1ba45af4ea3c",
    "Vanessa Gillings",
);

// ECL 344 — Trystan's Command (alternate printing)
const TRYSTAN_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRYSTAN_S_COMMAND,
    1,
    "aefed846-e367-4940-842f-b504c71d0a53",
    "Yas Imamura",
);

// ECL 345 — Twinflame Travelers (alternate printing)
const TWINFLAME_TRAVELERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWINFLAME_TRAVELERS,
    1,
    "a422ae59-abb6-423d-8400-670196560920",
    "adelinaillustration",
);

// ECL 346 — Chronicle of Victory (alternate printing)
const CHRONICLE_OF_VICTORY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHRONICLE_OF_VICTORY,
    1,
    "44407660-79ed-442d-a215-d388c27c5130",
    "adelinaillustration",
);

// ECL 347 — Hallowed Fountain // Hallowed Fountain (alternate printing)
const HALLOWED_FOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::HALLOWED_FOUNTAIN,
    1,
    "19cba6be-7291-4788-9241-87dad3b68363",
    "Justin Gerard",
);

// ECL 348 — Steam Vents // Steam Vents (alternate printing)
const STEAM_VENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::STEAM_VENTS,
    1,
    "eb96c335-9ed3-4f7d-b07a-185ff4044976",
    "Raoul Vitale",
);

// ECL 349 — Blood Crypt // Blood Crypt (alternate printing)
const BLOOD_CRYPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::BLOOD_CRYPT,
    1,
    "28d37803-d3f9-4806-9a0b-79362a517a7a",
    "Valera Lutfullina",
);

// ECL 350 — Overgrown Tomb // Overgrown Tomb (alternate printing)
const OVERGROWN_TOMB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::OVERGROWN_TOMB,
    1,
    "307fdde3-43f6-4d80-884a-4a85314ab8e4",
    "Matt Stewart",
);

// ECL 351 — Temple Garden // Temple Garden (alternate printing)
const TEMPLE_GARDEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::TEMPLE_GARDEN,
    1,
    "ec75b840-420d-4512-a74a-d70f19be0085",
    "Annie Stegg",
);

// ECL 352 — Bitterbloom Bearer (alternate printing)
const BITTERBLOOM_BEARER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BITTERBLOOM_BEARER,
    2,
    "895e7a6e-cd10-4a00-bfec-30e6330e193c",
    "Rebecca Guay",
);

// ECL 353 — Champion of the Clachan (alternate printing)
const CHAMPION_OF_THE_CLACHAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPION_OF_THE_CLACHAN,
    1,
    "8a91a96d-843c-4ec6-8078-89f87c5ce85a",
    "Edgar Sánchez Hidalgo",
);

// ECL 354 — Rhys, the Evermore (alternate printing)
const RHYS_THE_EVERMORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RHYS_THE_EVERMORE,
    1,
    "c3bef4e2-d5f4-4e95-a212-d8e02bef99f7",
    "Kai Carpenter",
);

// ECL 355 — Winnowing (alternate printing)
const WINNOWING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINNOWING,
    1,
    "6e434b0d-d438-432c-9684-5d3edf30790a",
    "David Palumbo",
);

// ECL 356 — Champions of the Shoal (alternate printing)
const CHAMPIONS_OF_THE_SHOAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPIONS_OF_THE_SHOAL,
    1,
    "cbb067b0-1bb6-467c-b361-2c9ed6bd3a22",
    "Daniel Zrom",
);

// ECL 357 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    1,
    "42465c20-16f8-46a2-96fe-b52a9eac3df9",
    "Tyler Walpole",
);

// ECL 358 — Rimefire Torque (alternate printing)
const RIMEFIRE_TORQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIMEFIRE_TORQUE,
    1,
    "23d41fc1-5b74-4ab5-9b14-43aaf3a7d125",
    "Jorge Jacinto",
);

// ECL 359 — Bloodline Bidding (alternate printing)
const BLOODLINE_BIDDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOODLINE_BIDDING,
    1,
    "f99ccd3b-07f7-47c2-8de9-6aa5251d7a76",
    "Drew Baker",
);

// ECL 360 — Champion of the Weird (alternate printing)
const CHAMPION_OF_THE_WEIRD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPION_OF_THE_WEIRD,
    1,
    "3aeaeb01-eab1-4bc6-bd58-c00a7e41a8ce",
    "Lucas Graciano",
);

// ECL 361 — Mornsong Aria (alternate printing)
const MORNSONG_ARIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORNSONG_ARIA,
    1,
    "05a553f3-3ff4-4601-bbf6-2f8b5ca6aaca",
    "Scott M. Fischer",
);

// ECL 362 — Champion of the Path (alternate printing)
const CHAMPION_OF_THE_PATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPION_OF_THE_PATH,
    1,
    "874a32a5-0a9b-4225-a877-bb6e74cd6c5f",
    "Tyler Walpole",
);

// ECL 363 — Collective Inferno (alternate printing)
const COLLECTIVE_INFERNO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLLECTIVE_INFERNO,
    1,
    "7250001d-a170-4b23-8290-6fdad48b4ac8",
    "Jason A. Engle",
);

// ECL 364 — End-Blaze Epiphany (alternate printing)
const END_BLAZE_EPIPHANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &END_BLAZE_EPIPHANY,
    1,
    "d8adeb65-f6f4-4d21-bca7-71fe8ef71f3b",
    "Tyler Walpole",
);

// ECL 365 — Champions of the Perfect (alternate printing)
const CHAMPIONS_OF_THE_PERFECT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPIONS_OF_THE_PERFECT,
    1,
    "2e35eb02-d541-45fe-9de7-a3c6599a9b0c",
    "Chris Rahn",
);

// ECL 366 — Formidable Speaker (alternate printing)
const FORMIDABLE_SPEAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FORMIDABLE_SPEAKER,
    1,
    "1444f39f-042b-4bb3-8307-6b8878198086",
    "Aurore Folny",
);

// ECL 367 — Selfless Safewright (alternate printing)
const SELFLESS_SAFEWRIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SELFLESS_SAFEWRIGHT,
    1,
    "ee08e321-6011-4545-920f-5b35180e896b",
    "Quintin Gleim",
);

// ECL 368 — Abigale, Eloquent First-Year (alternate printing)
const ABIGALE_ELOQUENT_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABIGALE_ELOQUENT_FIRST_YEAR,
    1,
    "c8d85a84-42a2-47d1-8fc2-1a081fd6bd2a",
    "Mark Zug",
);

// ECL 369 — Bre of Clan Stoutarm (alternate printing)
const BRE_OF_CLAN_STOUTARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRE_OF_CLAN_STOUTARM,
    1,
    "0f5f63df-f401-4c13-a105-e831fd841e80",
    "Jesper Ejsing",
);

// ECL 370 — Deepway Navigator (alternate printing)
const DEEPWAY_NAVIGATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEPWAY_NAVIGATOR,
    1,
    "c0c2b1a7-d03e-4266-9bae-f2deebd714af",
    "Jacob Walker",
);

// ECL 371 — Dream Harvest (alternate printing)
const DREAM_HARVEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DREAM_HARVEST,
    1,
    "be97b021-0844-4bac-8d17-c96b4920744e",
    "Ben Hill",
);

// ECL 372 — Figure of Fable (alternate printing)
const FIGURE_OF_FABLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIGURE_OF_FABLE,
    1,
    "eb1dc48d-81c4-4ecb-b5b3-d75b198f61db",
    "Omar Rayyan",
);

// ECL 373 — High Perfect Morcant (alternate printing)
const HIGH_PERFECT_MORCANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIGH_PERFECT_MORCANT,
    1,
    "fdb4192b-73ae-41e9-a3b7-a20c69350b79",
    "Victor Adame Minguez",
);

// ECL 374 — Kirol, Attentive First-Year (alternate printing)
const KIROL_ATTENTIVE_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KIROL_ATTENTIVE_FIRST_YEAR,
    1,
    "a77c3a75-4880-4b86-ab1e-92f045abc484",
    "Evyn Fong",
);

// ECL 375 — Lluwen, Imperfect Naturalist (alternate printing)
const LLUWEN_IMPERFECT_NATURALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LLUWEN_IMPERFECT_NATURALIST,
    1,
    "57289725-5ad2-46dd-a7f9-a78db67022e5",
    "Evyn Fong",
);

// ECL 376 — Maralen, Fae Ascendant (alternate printing)
const MARALEN_FAE_ASCENDANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARALEN_FAE_ASCENDANT,
    1,
    "4b1ace1f-84ec-499d-8a52-c0ad26a70d77",
    "Steve Prescott",
);

// ECL 377 — Raiding Schemes (alternate printing)
const RAIDING_SCHEMES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAIDING_SCHEMES,
    1,
    "0e04140a-11a1-4f4a-965a-5c3c6569595a",
    "Justin Gerard",
);

// ECL 378 — Sanar, Innovative First-Year (alternate printing)
const SANAR_INNOVATIVE_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SANAR_INNOVATIVE_FIRST_YEAR,
    1,
    "31075628-a238-4ed5-b126-16ef5c23d364",
    "Steven Belledin",
);

// ECL 379 — Shadow Urchin (alternate printing)
const SHADOW_URCHIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHADOW_URCHIN,
    1,
    "ec1b5c33-2025-4d01-9b51-433a40a105ed",
    "Ron Spencer",
);

// ECL 380 — Tam, Mindful First-Year (alternate printing)
const TAM_MINDFUL_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TAM_MINDFUL_FIRST_YEAR,
    1,
    "ca89fb7a-5575-4875-80d3-6a05f78ea65e",
    "Zoltan Boros",
);

// ECL 381 — Mirrormind Crown (alternate printing)
const MIRRORMIND_CROWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIRRORMIND_CROWN,
    1,
    "00b44e41-828b-4da4-b625-92943b8989ac",
    "Dan Frazier",
);

// ECL 382 — Winnowing (alternate printing)
const WINNOWING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WINNOWING,
    2,
    "9003e4f9-273f-4754-beaa-cd770c674a2b",
    "Yukoring",
);

// ECL 383 — Glen Elendra Guardian (alternate printing)
const GLEN_ELENDRA_GUARDIAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_GUARDIAN,
    2,
    "a7c5b57c-f6da-4de0-9645-42e30c25ca32",
    "NgN",
);

// ECL 384 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    2,
    "31764ac0-6acb-496f-bb44-d556235bb47e",
    "konomura",
);

// ECL 385 — Bloodline Bidding (alternate printing)
const BLOODLINE_BIDDING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BLOODLINE_BIDDING,
    2,
    "aca1b882-0fe9-4022-98e4-bae528b2a1e2",
    "morizo",
);

// ECL 386 — Moonshadow (alternate printing)
const MOONSHADOW_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOONSHADOW,
    2,
    "485a801c-0e10-4943-8456-852aacb24d9d",
    "Mizugoromaru",
);

// ECL 387 — Collective Inferno (alternate printing)
const COLLECTIVE_INFERNO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COLLECTIVE_INFERNO,
    2,
    "72bd8ce5-795a-4503-b92c-a4a39f0bd4db",
    "YUE",
);

// ECL 388 — Meek Attack (alternate printing)
const MEEK_ATTACK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MEEK_ATTACK,
    2,
    "530aa72f-0a77-443b-9ad9-cce8d91b8657",
    "Yoshioka",
);

// ECL 389 — Spinerock Tyrant (alternate printing)
const SPINEROCK_TYRANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPINEROCK_TYRANT,
    2,
    "933f0eb4-7c8a-44c9-b91b-f3ff35081d1d",
    "D-suzuki",
);

// ECL 390 — Bloom Tender (alternate printing)
const BLOOM_TENDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_eve::BLOOM_TENDER,
    2,
    "0b5f1455-8124-408f-9ca5-9b281ec1708f",
    "Yuyuharu",
);

// ECL 391 — Selfless Safewright (alternate printing)
const SELFLESS_SAFEWRIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SELFLESS_SAFEWRIGHT,
    2,
    "28ab33be-cbfc-46eb-a5fc-9c770e1051d6",
    "Karuta Shiki",
);

// ECL 392 — Winnowing (alternate printing)
const WINNOWING_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WINNOWING,
    3,
    "1384b7c7-81fe-4444-89a2-94c9de51df6a",
    "Yukoring",
);

// ECL 393 — Glen Elendra Guardian (alternate printing)
const GLEN_ELENDRA_GUARDIAN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_GUARDIAN,
    3,
    "af008f52-d83d-455a-b217-7e40a8849a99",
    "NgN",
);

// ECL 394 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    3,
    "7b200330-6463-4ce0-828e-5e1a4f074478",
    "konomura",
);

// ECL 395 — Bloodline Bidding (alternate printing)
const BLOODLINE_BIDDING_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BLOODLINE_BIDDING,
    3,
    "cc187974-f1e7-4fb4-9c81-51cd6b395974",
    "morizo",
);

// ECL 396 — Moonshadow (alternate printing)
const MOONSHADOW_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MOONSHADOW,
    3,
    "deb91ea3-f355-4aa2-a029-908cecc55fc2",
    "Mizugoromaru",
);

// ECL 397 — Collective Inferno (alternate printing)
const COLLECTIVE_INFERNO_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &COLLECTIVE_INFERNO,
    3,
    "2ada6e6f-77bd-49cb-8aa7-933acc02a0f2",
    "YUE",
);

// ECL 398 — Meek Attack (alternate printing)
const MEEK_ATTACK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MEEK_ATTACK,
    3,
    "ed87df86-3a71-444b-a7a0-8221b1627610",
    "Yoshioka",
);

// ECL 399 — Spinerock Tyrant (alternate printing)
const SPINEROCK_TYRANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPINEROCK_TYRANT,
    3,
    "10808193-6ddc-4d53-a3fa-32d7ba80a04f",
    "D-suzuki",
);

// ECL 400 — Bloom Tender (alternate printing)
const BLOOM_TENDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_eve::BLOOM_TENDER,
    3,
    "52c440f6-95f7-473a-8b81-0e423492f205",
    "Yuyuharu",
);

// ECL 401 — Selfless Safewright (alternate printing)
const SELFLESS_SAFEWRIGHT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SELFLESS_SAFEWRIGHT,
    3,
    "9a7a526e-b5cc-4a44-9a2c-722603343b21",
    "Karuta Shiki",
);

// ECL 402 — Personify (alternate printing)
const PERSONIFY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PERSONIFY,
    1,
    "08ed333b-358a-4e44-954e-562ecf5666b3",
    "Slawomir Maniak",
);

// ECL 403 — Silvergill Mentor (alternate printing)
const SILVERGILL_MENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SILVERGILL_MENTOR,
    1,
    "3841d32f-4e41-409a-b1ec-3db9390d61a1",
    "Iris Compiet",
);

// ECL 404 — Iron-Shield Elf (alternate printing)
const IRON_SHIELD_ELF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRON_SHIELD_ELF,
    1,
    "3d47b1f2-da4a-42c4-992e-e0bfdd1a3e2a",
    "Adrián Rodríguez Pérez",
);

// ECL 405 — Sear (alternate printing)
const SEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEAR,
    1,
    "0a13f8e0-8b23-4553-ad76-9e52f8d4c63c",
    "Lars Grant-West",
);

// ECL 406 — Virulent Emissary (alternate printing)
const VIRULENT_EMISSARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRULENT_EMISSARY,
    1,
    "6044d540-70ff-4ce3-9a26-4164e7543baa",
    "Tiffany Turrill",
);

// ECL 407 — Kinbinding (alternate printing)
const KINBINDING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KINBINDING,
    2,
    "df94f428-f097-42cb-be03-3046e6867344",
    "Heather Hudson",
);

// ECL 408 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    4,
    "2dbd1ec1-2b66-4fcc-84cb-9d35afcec921",
    "Jeff Miracola",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CHANGELING_WAYFINDER,
    &ROOFTOP_PERCHER,
    &ADEPT_WATERSHAPER,
    &AJANI_OUTLAND_CHAPERONE,
    &APPEAL_TO_EIRDU,
    &BARK_OF_DORAN,
    &BRIGID_CLACHAN_S_HEART,
    &BURDENED_STONEBACK,
    &CHAMPION_OF_THE_CLACHAN,
    &CLACHAN_FESTIVAL,
    &CURIOUS_COLOSSUS,
    &EIRDU_CARRIER_OF_DAWN,
    &ENCUMBERED_REEJEREY,
    &EVERSHRIKE_S_GIFT,
    &FLOCK_IMPOSTOR,
    &GALLANT_FOWLKNIGHT,
    &GOLDMEADOW_NOMAD,
    &KEEP_OUT,
    &KINBINDING,
    &KINSBAILE_ASPIRANT,
    &KINSCAER_SENTRY,
    &KITHKEEPER,
    &LIMINAL_HOLD,
    &MEANDERS_GUIDE,
    &MOONLIT_LAMENTER,
    &MORNINGTIDE_S_LIGHT,
    &PERSONIFY,
    &PROTECTIVE_RESPONSE,
    &PYRRHIC_STRIKE,
    &RELUCTANT_DOUNGUARD,
    &RHYS_THE_EVERMORE,
    &RIVERGUARD_S_REFLEXES,
    &SHORE_LURKER,
    &SLUMBERING_WALKER,
    &SPIRAL_INTO_SOLITUDE,
    &SUN_DAPPLED_CELEBRANT,
    &THOUGHTWEFT_IMBUER,
    &TIMID_SHIELDBEARER,
    &TRIBUTARY_VAULTER,
    &WANDERBRINE_PREACHER,
    &WANDERBRINE_TRAPPER,
    &WINNOWING,
    &AQUITECT_S_DEFENSES,
    &BLOSSOMBIND,
    &CHAMPIONS_OF_THE_SHOAL,
    &DISRUPTOR_OF_CURRENTS,
    &FLITTERWING_NUISANCE,
    &GLAMER_GIFTER,
    &GLAMERMITE,
    &GLEN_ELENDRA_GUARDIAN,
    &GLEN_ELENDRA_S_ANSWER,
    &GRAVELGILL_SCOUNDREL,
    &HARMONIZED_CRESCENDO,
    &ILLUSION_SPINNERS,
    &KULRATH_MYSTIC,
    &LOCH_MARE,
    &LOFTY_DREAMS,
    &MIRRORFORM,
    &NOGGLE_THE_MIND,
    &OKO_LORWYN_LIEGE,
    &OMNI_CHANGELING,
    &PESTERED_WELLGUARD,
    &RIME_CHILL,
    &RIMEFIRE_TORQUE,
    &RIMEKIN_RECLUSE,
    &SHINESTRIKER,
    &SILVERGILL_MENTOR,
    &SILVERGILL_PEDDLER,
    &STRATOSOARER,
    &SUMMIT_SENTINEL,
    &SUNDERFLOCK,
    &SWAT_AWAY,
    &SYGG_WANDERWINE_WISDOM,
    &TANUFEL_RIMESPEAKER,
    &THIRST_FOR_IDENTITY,
    &UNEXPECTED_ASSISTANCE,
    &UNWELCOME_SPRITE,
    &WANDERWINE_DISTRACTER,
    &WANDERWINE_FAREWELL,
    &WILD_UNRAVELING,
    &AUNTIE_S_SENTENCE,
    &BARBED_BLOODLETTER,
    &BILE_VIAL_BOGGART,
    &BITTERBLOOM_BEARER,
    &BLIGHT_ROT,
    &BLIGHTED_BLACKTHORN,
    &BLOODLINE_BIDDING,
    &BOGGART_MISCHIEF,
    &BOGGART_PRANKSTER,
    &BOGSLITHER_S_EMBRACE,
    &CHAMPION_OF_THE_WEIRD,
    &CREAKWOOD_SAFEWRIGHT,
    &DARKNESS_DESCENDS,
    &DAWNHAND_DISSIDENT,
    &DAWNHAND_EULOGIST,
    &DOSE_OF_DAWNGLOW,
    &DREAM_SEIZER,
    &GLOOM_RIPPER,
    &GNARLBARK_ELM,
    &GRUB_STORIED_MATRIARCH,
    &GUTSPLITTER_GANG,
    &HEIRLOOM_AUNTIE,
    &IRON_SHIELD_ELF,
    &MOONGLOVE_EXTRACTOR,
    &MOONSHADOW,
    &MORNSONG_ARIA,
    &MUDBUTTON_CURSETOSSER,
    &NIGHTMARE_SOWER,
    &PERFECT_INTIMIDATION,
    &REQUITING_HEX,
    &RETCHED_WRETCH,
    &SCARBLADE_SCOUT,
    &SCARBLADE_S_MALICE,
    &SHIMMERCREEP,
    &TASTER_OF_WARES,
    &TWILIGHT_DIVINER,
    &UNBURY,
    &ASHLING_REKINDLED,
    &BOLDWYR_AGGRESSOR,
    &BONECLUB_BERSERKER,
    &BOULDER_DASH,
    &BRAMBLEBACK_BRUTE,
    &BURNING_CURIOSITY,
    &CHAMPION_OF_THE_PATH,
    &CINDER_STRIKE,
    &COLLECTIVE_INFERNO,
    &ELDER_AUNTIE,
    &END_BLAZE_EPIPHANY,
    &ENRAGED_FLAMECASTER,
    &EXPLOSIVE_PRODIGY,
    &FEED_THE_FLAMES,
    &FLAME_CHAIN_MAULER,
    &FLAMEBRAIDER,
    &FLAMEKIN_GILDWEAVER,
    &GIANTFALL,
    &GOLIATH_DAYDREAMER,
    &GRISTLE_GLUTTON,
    &HEXING_SQUELCHER,
    &IMPOLITE_ENTRANCE,
    &KINDLE_THE_INNER_FLAME,
    &KULRATH_ZEALOT,
    &LASTING_TARFIRE,
    &LAVALEAPER,
    &MEEK_ATTACK,
    &RECKLESS_RANSACKING,
    &SCUZZBACK_SCROUNGER,
    &SEAR,
    &SIZZLING_CHANGELING,
    &SOUL_IMMOLATION,
    &SOULBRIGHT_SEEKER,
    &SOURBREAD_AUNTIE,
    &SPINEROCK_TYRANT,
    &SQUAWKROASTER,
    &STING_SLINGER,
    &TWEEZE,
    &WARREN_TORCHMASTER,
    &ASSERT_PERFECTION,
    &AURORA_AWAKENER,
    &BRISTLEBANE_BATTLER,
    &BRISTLEBANE_OUTRIDER,
    &CELESTIAL_REUNION,
    &CHAMPIONS_OF_THE_PERFECT,
    &CHOMPING_CHANGELING,
    &CROSSROADS_WATCHER,
    &DAWN_S_LIGHT_ARCHER,
    &DUNDOOLIN_WEAVER,
    &FORMIDABLE_SPEAKER,
    &GILT_LEAF_S_EMBRACE,
    &GREAT_FOREST_DRUID,
    &LUMINOLLUSK,
    &LYS_ALANA_DIGNITARY,
    &LYS_ALANA_INFORMANT,
    &MIDNIGHT_TILLING,
    &MISTMEADOW_COUNCIL,
    &MOON_VIGIL_ADHERENTS,
    &MORCANT_S_EYES,
    &MUTABLE_EXPLORER,
    &PITILESS_FISTS,
    &PRISMABASHER,
    &PRISMATIC_UNDERCURRENTS,
    &PUMMELER_FOR_HIRE,
    &SAFEWRIGHT_CAVALRY,
    &SAPLING_NURSERY,
    &SELFLESS_SAFEWRIGHT,
    &SHIMMERWILDS_GROWTH,
    &SPRY_AND_MIGHTY,
    &SURLY_FARRIER,
    &TEND_THE_SPRIGS,
    &THOUGHTWEFT_CHARGE,
    &TRYSTAN_CALLOUS_CULTIVATOR,
    &UNFORGIVING_AIM,
    &VINEBRED_BRAWLER,
    &VIRULENT_EMISSARY,
    &WILDVINE_PUMMELER,
    &ABIGALE_ELOQUENT_FIRST_YEAR,
    &ASHLING_S_COMMAND,
    &BOGGART_CURSECRAFTER,
    &BRE_OF_CLAN_STOUTARM,
    &BRIGID_S_COMMAND,
    &CATHARSIS,
    &CHAOS_SPEWER,
    &CHITINOUS_GRASPLING,
    &DECEIT,
    &DEEPCHANNEL_DUELIST,
    &DEEPWAY_NAVIGATOR,
    &DORAN_BESIEGED_BY_TIME,
    &DREAM_HARVEST,
    &ECLIPSED_BOGGART,
    &ECLIPSED_ELF,
    &ECLIPSED_FLAMEKIN,
    &ECLIPSED_KITHKIN,
    &ECLIPSED_MERROW,
    &EMPTINESS,
    &FEISTY_SPIKELING,
    &FIGURE_OF_FABLE,
    &FLARING_CINDER,
    &GANGLY_STOMPLING,
    &GLISTER_BAIRN,
    &GRUB_S_COMMAND,
    &HIGH_PERFECT_MORCANT,
    &HOVEL_HURLER,
    &KIROL_ATTENTIVE_FIRST_YEAR,
    &LLUWEN_IMPERFECT_NATURALIST,
    &MARALEN_FAE_ASCENDANT,
    &MERROW_SKYSWIMMER,
    &MISCHIEVOUS_SNEAKLING,
    &MORCANT_S_LOYALIST,
    &NOGGLE_ROBBER,
    &PRIDEFUL_FEASTLING,
    &RAIDING_SCHEMES,
    &REAPING_WILLOW,
    &SANAR_INNOVATIVE_FIRST_YEAR,
    &SHADOW_URCHIN,
    &STOIC_GROVE_GUIDE,
    &SYGG_S_COMMAND,
    &TAM_MINDFUL_FIRST_YEAR,
    &THOUGHTWEFT_LIEUTENANT,
    &TRYSTAN_S_COMMAND,
    &TWINFLAME_TRAVELERS,
    &VIBRANCE,
    &VORACIOUS_TOME_SKIMMER,
    &WARY_FARMER,
    &WISTFULNESS,
    &CHRONICLE_OF_VICTORY,
    &DAWN_BLESSED_PENNANT,
    &FIRDOCH_CORE,
    &FORAGING_WICKERMAW,
    &GATHERING_STONE,
    &MIRRORMIND_CROWN,
    &PUCA_S_EYE,
    &STALACTITE_DAGGER,
    &ECLIPSED_REALMS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    CRIB_SWAP_REPRINT,
    RUN_AWAY_TOGETHER_REPRINT,
    SPELL_SNARE_REPRINT,
    TEMPORAL_CLEANSING_REPRINT,
    GRAVESHIFTER_REPRINT,
    NAMELESS_INVERSION_REPRINT,
    GOATNAP_REPRINT,
    BLOOM_TENDER_REPRINT,
    BLOSSOMING_DEFENSE_REPRINT,
    SPRINGLEAF_DRUM_REPRINT,
    BLOOD_CRYPT_REPRINT,
    EVOLVING_WILDS_REPRINT,
    HALLOWED_FOUNTAIN_REPRINT,
    OVERGROWN_TOMB_REPRINT,
    STEAM_VENTS_REPRINT,
    TEMPLE_GARDEN_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    ISLAND_ALTERNATE_1,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_1,
    FOREST_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_2,
    AJANI_OUTLAND_CHAPERONE_ALTERNATE_1,
    BRIGID_CLACHAN_S_HEART_ALTERNATE_1,
    EIRDU_CARRIER_OF_DAWN_ALTERNATE_1,
    OKO_LORWYN_LIEGE_ALTERNATE_1,
    SYGG_WANDERWINE_WISDOM_ALTERNATE_1,
    GRUB_STORIED_MATRIARCH_ALTERNATE_1,
    ASHLING_REKINDLED_ALTERNATE_1,
    TRYSTAN_CALLOUS_CULTIVATOR_ALTERNATE_1,
    CATHARSIS_ALTERNATE_1,
    DECEIT_ALTERNATE_1,
    EMPTINESS_ALTERNATE_1,
    VIBRANCE_ALTERNATE_1,
    WISTFULNESS_ALTERNATE_1,
    ADEPT_WATERSHAPER_ALTERNATE_1,
    CURIOUS_COLOSSUS_ALTERNATE_1,
    KINBINDING_ALTERNATE_1,
    KINSCAER_SENTRY_ALTERNATE_1,
    MORNINGTIDE_S_LIGHT_ALTERNATE_1,
    SLUMBERING_WALKER_ALTERNATE_1,
    DISRUPTOR_OF_CURRENTS_ALTERNATE_1,
    FLITTERWING_NUISANCE_ALTERNATE_1,
    GLEN_ELENDRA_GUARDIAN_ALTERNATE_1,
    GLEN_ELENDRA_S_ANSWER_ALTERNATE_1,
    LOCH_MARE_ALTERNATE_1,
    MIRRORFORM_ALTERNATE_1,
    SUNDERFLOCK_ALTERNATE_1,
    BITTERBLOOM_BEARER_ALTERNATE_1,
    DAWNHAND_DISSIDENT_ALTERNATE_1,
    GLOOM_RIPPER_ALTERNATE_1,
    MOONSHADOW_ALTERNATE_1,
    TASTER_OF_WARES_ALTERNATE_1,
    TWILIGHT_DIVINER_ALTERNATE_1,
    GOLIATH_DAYDREAMER_ALTERNATE_1,
    HEXING_SQUELCHER_ALTERNATE_1,
    LAVALEAPER_ALTERNATE_1,
    MEEK_ATTACK_ALTERNATE_1,
    SCUZZBACK_SCROUNGER_ALTERNATE_1,
    SOUL_IMMOLATION_ALTERNATE_1,
    SPINEROCK_TYRANT_ALTERNATE_1,
    AURORA_AWAKENER_ALTERNATE_1,
    BLOOM_TENDER_ALTERNATE_1,
    BRISTLEBANE_BATTLER_ALTERNATE_1,
    CELESTIAL_REUNION_ALTERNATE_1,
    MUTABLE_EXPLORER_ALTERNATE_1,
    SAPLING_NURSERY_ALTERNATE_1,
    SPRY_AND_MIGHTY_ALTERNATE_1,
    ASHLING_S_COMMAND_ALTERNATE_1,
    BOGGART_CURSECRAFTER_ALTERNATE_1,
    BRIGID_S_COMMAND_ALTERNATE_1,
    DEEPCHANNEL_DUELIST_ALTERNATE_1,
    DORAN_BESIEGED_BY_TIME_ALTERNATE_1,
    ECLIPSED_BOGGART_ALTERNATE_1,
    ECLIPSED_ELF_ALTERNATE_1,
    ECLIPSED_FLAMEKIN_ALTERNATE_1,
    ECLIPSED_KITHKIN_ALTERNATE_1,
    ECLIPSED_MERROW_ALTERNATE_1,
    GRUB_S_COMMAND_ALTERNATE_1,
    MORCANT_S_LOYALIST_ALTERNATE_1,
    SYGG_S_COMMAND_ALTERNATE_1,
    THOUGHTWEFT_LIEUTENANT_ALTERNATE_1,
    TRYSTAN_S_COMMAND_ALTERNATE_1,
    TWINFLAME_TRAVELERS_ALTERNATE_1,
    CHRONICLE_OF_VICTORY_ALTERNATE_1,
    HALLOWED_FOUNTAIN_ALTERNATE_1,
    STEAM_VENTS_ALTERNATE_1,
    BLOOD_CRYPT_ALTERNATE_1,
    OVERGROWN_TOMB_ALTERNATE_1,
    TEMPLE_GARDEN_ALTERNATE_1,
    BITTERBLOOM_BEARER_ALTERNATE_2,
    CHAMPION_OF_THE_CLACHAN_ALTERNATE_1,
    RHYS_THE_EVERMORE_ALTERNATE_1,
    WINNOWING_ALTERNATE_1,
    CHAMPIONS_OF_THE_SHOAL_ALTERNATE_1,
    HARMONIZED_CRESCENDO_ALTERNATE_1,
    RIMEFIRE_TORQUE_ALTERNATE_1,
    BLOODLINE_BIDDING_ALTERNATE_1,
    CHAMPION_OF_THE_WEIRD_ALTERNATE_1,
    MORNSONG_ARIA_ALTERNATE_1,
    CHAMPION_OF_THE_PATH_ALTERNATE_1,
    COLLECTIVE_INFERNO_ALTERNATE_1,
    END_BLAZE_EPIPHANY_ALTERNATE_1,
    CHAMPIONS_OF_THE_PERFECT_ALTERNATE_1,
    FORMIDABLE_SPEAKER_ALTERNATE_1,
    SELFLESS_SAFEWRIGHT_ALTERNATE_1,
    ABIGALE_ELOQUENT_FIRST_YEAR_ALTERNATE_1,
    BRE_OF_CLAN_STOUTARM_ALTERNATE_1,
    DEEPWAY_NAVIGATOR_ALTERNATE_1,
    DREAM_HARVEST_ALTERNATE_1,
    FIGURE_OF_FABLE_ALTERNATE_1,
    HIGH_PERFECT_MORCANT_ALTERNATE_1,
    KIROL_ATTENTIVE_FIRST_YEAR_ALTERNATE_1,
    LLUWEN_IMPERFECT_NATURALIST_ALTERNATE_1,
    MARALEN_FAE_ASCENDANT_ALTERNATE_1,
    RAIDING_SCHEMES_ALTERNATE_1,
    SANAR_INNOVATIVE_FIRST_YEAR_ALTERNATE_1,
    SHADOW_URCHIN_ALTERNATE_1,
    TAM_MINDFUL_FIRST_YEAR_ALTERNATE_1,
    MIRRORMIND_CROWN_ALTERNATE_1,
    WINNOWING_ALTERNATE_2,
    GLEN_ELENDRA_GUARDIAN_ALTERNATE_2,
    HARMONIZED_CRESCENDO_ALTERNATE_2,
    BLOODLINE_BIDDING_ALTERNATE_2,
    MOONSHADOW_ALTERNATE_2,
    COLLECTIVE_INFERNO_ALTERNATE_2,
    MEEK_ATTACK_ALTERNATE_2,
    SPINEROCK_TYRANT_ALTERNATE_2,
    BLOOM_TENDER_ALTERNATE_2,
    SELFLESS_SAFEWRIGHT_ALTERNATE_2,
    WINNOWING_ALTERNATE_3,
    GLEN_ELENDRA_GUARDIAN_ALTERNATE_3,
    HARMONIZED_CRESCENDO_ALTERNATE_3,
    BLOODLINE_BIDDING_ALTERNATE_3,
    MOONSHADOW_ALTERNATE_3,
    COLLECTIVE_INFERNO_ALTERNATE_3,
    MEEK_ATTACK_ALTERNATE_3,
    SPINEROCK_TYRANT_ALTERNATE_3,
    BLOOM_TENDER_ALTERNATE_3,
    SELFLESS_SAFEWRIGHT_ALTERNATE_3,
    PERSONIFY_ALTERNATE_1,
    SILVERGILL_MENTOR_ALTERNATE_1,
    IRON_SHIELD_ELF_ALTERNATE_1,
    SEAR_ALTERNATE_1,
    VIRULENT_EMISSARY_ALTERNATE_1,
    KINBINDING_ALTERNATE_2,
    HARMONIZED_CRESCENDO_ALTERNATE_4,
];
