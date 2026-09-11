//! Teenage Mutant Ninja Turtles card inventory.

use super::{CardRecord, PrintingRecord};

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2008::morningtide as catalog_mor;
use crate::card::sets::y2024::murders_at_karlov_manor as catalog_mkm;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TMT",
    slug: "teenage-mutant-ninja-turtles",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// TMT 1 — Action News Crew
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACTION_NEWS_CREW: CardRecord = CardRecord::new(
    "Action News Crew",
    "bc0f5ca8-47bd-4451-8fd1-a312ff7d31ec",
    "Gabriel Tanko",
    crate::card::CardRules::unsupported(),
);

// TMT 2 — Agent Bishop, Man in Black
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_BISHOP_MAN_IN_BLACK: CardRecord = CardRecord::new(
    "Agent Bishop, Man in Black",
    "9c769202-178c-442a-a9e2-aa08b5ae5c9a",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// TMT 3 — April O'Neil, Kunoichi Trainee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APRIL_O_NEIL_KUNOICHI_TRAINEE: CardRecord = CardRecord::new(
    "April O'Neil, Kunoichi Trainee",
    "8b1982ea-686b-4acd-b677-47571430efb0",
    "Jo Cordisco",
    crate::card::CardRules::unsupported(),
);

// TMT 4 — Dimensional Exile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMENSIONAL_EXILE: CardRecord = CardRecord::new(
    "Dimensional Exile",
    "be8d96fb-a1be-4fff-b844-e38d185884e1",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// TMT 5 — East Wind Avatar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EAST_WIND_AVATAR: CardRecord = CardRecord::new(
    "East Wind Avatar",
    "c9d5b56c-ad2a-4958-8783-4eceb8733610",
    "Andrea Tentori Montalto",
    crate::card::CardRules::unsupported(),
);

// TMT 6 — Featherbrained Filcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEATHERBRAINED_FILCHER: CardRecord = CardRecord::new(
    "Featherbrained Filcher",
    "78c55706-4d00-4b97-965c-0b2d0963e59d",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// TMT 7 — Grounded for Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROUNDED_FOR_LIFE: CardRecord = CardRecord::new(
    "Grounded for Life",
    "72388199-85fa-4eba-9a9c-c2904e6da9ed",
    "Andrea Tentori Montalto",
    crate::card::CardRules::unsupported(),
);

// TMT 8 — Hamato Guardian Stance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAMATO_GUARDIAN_STANCE: CardRecord = CardRecord::new(
    "Hamato Guardian Stance",
    "735b540b-b472-46fa-a232-d444cabf6c4c",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// TMT 9 — High-Flying Ace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_FLYING_ACE: CardRecord = CardRecord::new(
    "High-Flying Ace",
    "e927eb0b-1c69-421a-8acd-01e92f729ffb",
    "Daniel Romanovsky",
    crate::card::CardRules::unsupported(),
);

// TMT 10 — Jennika, Bad Apple Big Sister
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JENNIKA_BAD_APPLE_BIG_SISTER: CardRecord = CardRecord::new(
    "Jennika, Bad Apple Big Sister",
    "a5b83101-e3ef-4ffe-a886-4fc2b57a0947",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// TMT 11 — Koya, Death from Above
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOYA_DEATH_FROM_ABOVE: CardRecord = CardRecord::new(
    "Koya, Death from Above",
    "7a4f1ccc-2225-4cd0-abef-0eb7a8eae9cf",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// TMT 12 — The Last Ronin's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_LAST_RONIN_S_TECHNIQUE: CardRecord = CardRecord::new(
    "The Last Ronin's Technique",
    "dfb18239-d373-4795-8598-c82abae2cb62",
    "Adam Volker",
    crate::card::CardRules::unsupported(),
);

// TMT 13 — Leader's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEADER_S_TALENT: CardRecord = CardRecord::new(
    "Leader's Talent",
    "4cbcb622-1aff-460f-b8fa-4502d991e0ad",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// TMT 14 — Leonardo, Big Brother
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONARDO_BIG_BROTHER: CardRecord = CardRecord::new(
    "Leonardo, Big Brother",
    "e802838f-cc8c-4313-8c3b-32a6a7248e64",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// TMT 15 — Leonardo, Cutting Edge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONARDO_CUTTING_EDGE: CardRecord = CardRecord::new(
    "Leonardo, Cutting Edge",
    "74c11ee3-19d1-4ad5-a727-1d74c565d6a5",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// TMT 16 — Leonardo, Leader in Blue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONARDO_LEADER_IN_BLUE: CardRecord = CardRecord::new(
    "Leonardo, Leader in Blue",
    "d6eaae35-d513-43d8-be2d-b97c15e25937",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// TMT 17 — Leonardo, Sewer Samurai
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONARDO_SEWER_SAMURAI: CardRecord = CardRecord::new(
    "Leonardo, Sewer Samurai",
    "0e3a0a26-c163-4f31-bed8-1be52a35feea",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// TMT 18 — Leonardo's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONARDO_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Leonardo's Technique",
    "405e4057-e26c-4882-89a9-706868548c37",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// TMT 19 — Lita, Little Orphan Amphibian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LITA_LITTLE_ORPHAN_AMPHIBIAN: CardRecord = CardRecord::new(
    "Lita, Little Orphan Amphibian",
    "9fbaabb5-e981-4cbf-888c-46449412711f",
    "Anna Pavleeva",
    crate::card::CardRules::unsupported(),
);

// TMT 20 — Make Your Move (reprint)
const MAKE_YOUR_MOVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mkm::MAKE_YOUR_MOVE,
    "ed8bdd98-6377-40cf-b381-cee38b1bda2a",
    "Nathaniel Himawan",
);

// TMT 21 — Mighty Mutanimals
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIGHTY_MUTANIMALS: CardRecord = CardRecord::new(
    "Mighty Mutanimals",
    "5dd5369c-174c-450b-b776-553866787f8f",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// TMT 22 — Prehistoric Pet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PREHISTORIC_PET: CardRecord = CardRecord::new(
    "Prehistoric Pet",
    "148e6acd-a96a-4bea-8cfe-a129cd8d1003",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// TMT 23 — Quintessential Katana
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUINTESSENTIAL_KATANA: CardRecord = CardRecord::new(
    "Quintessential Katana",
    "ef9e227e-c581-479c-a962-2f191352b07f",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// TMT 24 — Sally Pride, Lioness Leader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SALLY_PRIDE_LIONESS_LEADER: CardRecord = CardRecord::new(
    "Sally Pride, Lioness Leader",
    "bcd8ee6b-7142-4548-8b7a-691a36411851",
    "Andrea Tentori Montalto",
    crate::card::CardRules::unsupported(),
);

// TMT 25 — Triceraton Commander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRICERATON_COMMANDER: CardRecord = CardRecord::new(
    "Triceraton Commander",
    "6445b690-71ce-4371-ae9c-bac0e70dda81",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// TMT 26 — Turncoat Kunoichi
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURNCOAT_KUNOICHI: CardRecord = CardRecord::new(
    "Turncoat Kunoichi",
    "3ae61336-a4ee-40cd-9a18-392d96b873a4",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// TMT 27 — Turtles Forever
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURTLES_FOREVER: CardRecord = CardRecord::new(
    "Turtles Forever",
    "f0db974a-3289-4727-9aaf-e9cca9113c87",
    "Devin Elle Kurtz",
    crate::card::CardRules::unsupported(),
);

// TMT 28 — Uneasy Alliance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNEASY_ALLIANCE: CardRecord = CardRecord::new(
    "Uneasy Alliance",
    "5d9a4f9a-1e3a-4de8-a7c0-7158c6703b4e",
    "Rose Benjamin",
    crate::card::CardRules::unsupported(),
);

// TMT 29 — April O'Neil, Hacktivist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APRIL_O_NEIL_HACKTIVIST: CardRecord = CardRecord::new(
    "April O'Neil, Hacktivist",
    "f0147f6d-b797-4d5e-aca2-a9d309896eca",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// TMT 30 — April, Reporter of the Weird
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APRIL_REPORTER_OF_THE_WEIRD: CardRecord = CardRecord::new(
    "April, Reporter of the Weird",
    "31aa943f-c9db-43dc-8a72-7ef56f9f5c8b",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// TMT 31 — Bespoke Bō
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BESPOKE_BO: CardRecord = CardRecord::new(
    "Bespoke Bō",
    "6c517308-831b-4e41-a82c-02ddf6383a0c",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// TMT 32 — Buzz Bots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUZZ_BOTS: CardRecord = CardRecord::new(
    "Buzz Bots",
    "7c375190-f81b-4ab1-a1b6-fe432796821f",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// TMT 33 — Crustacean Commando
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUSTACEAN_COMMANDO: CardRecord = CardRecord::new(
    "Crustacean Commando",
    "9528cc07-df4b-417f-ad65-c2fae6fc2d49",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// TMT 34 — Does Machines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOES_MACHINES: CardRecord = CardRecord::new(
    "Does Machines",
    "989da63a-2cbd-41a9-9bbb-99f4ad1c6a25",
    "Aeron Ng",
    crate::card::CardRules::unsupported(),
);

// TMT 35 — Donatello, Gadget Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DONATELLO_GADGET_MASTER: CardRecord = CardRecord::new(
    "Donatello, Gadget Master",
    "8b5dd830-dab8-4628-845f-3d17973f9ffa",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// TMT 36 — Donatello, Mutant Mechanic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DONATELLO_MUTANT_MECHANIC: CardRecord = CardRecord::new(
    "Donatello, Mutant Mechanic",
    "3271b821-8efc-49e2-96fd-c48e2b2585c6",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// TMT 37 — Donatello, Turtle Techie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DONATELLO_TURTLE_TECHIE: CardRecord = CardRecord::new(
    "Donatello, Turtle Techie",
    "683cfef0-f164-4db8-b83f-79eb804e50ae",
    "Ryan Valle",
    crate::card::CardRules::unsupported(),
);

// TMT 38 — Donatello, Way with Machines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DONATELLO_WAY_WITH_MACHINES: CardRecord = CardRecord::new(
    "Donatello, Way with Machines",
    "a3a5b63a-3263-4f06-a643-808c38f64c77",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// TMT 39 — Donatello's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DONATELLO_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Donatello's Technique",
    "e570082f-8129-44d2-b471-ec7f46a98dbd",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// TMT 40 — Fugitive Droid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUGITIVE_DROID: CardRecord = CardRecord::new(
    "Fugitive Droid",
    "50c4e65f-00dc-4fc5-bd5c-8482c2848f4c",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// TMT 41 — Kitsune, Dragon's Daughter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KITSUNE_DRAGON_S_DAUGHTER: CardRecord = CardRecord::new(
    "Kitsune, Dragon's Daughter",
    "a87a9257-4535-4286-8b59-a842ac45d05e",
    "Robin Har",
    crate::card::CardRules::unsupported(),
);

// TMT 42 — Kitsune's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KITSUNE_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Kitsune's Technique",
    "9ca5327e-df90-483c-875f-73a23781f56d",
    "Rose Benjamin",
    crate::card::CardRules::unsupported(),
);

// TMT 43 — Krang, Master Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRANG_MASTER_MIND: CardRecord = CardRecord::new(
    "Krang, Master Mind",
    "d27fa497-e842-4812-80fe-28517544e1c5",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// TMT 44 — Metalhead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METALHEAD: CardRecord = CardRecord::new(
    "Metalhead",
    "3c2d8b09-8694-45ab-be01-f8dc17378cf0",
    "Daniel Romanovsky",
    crate::card::CardRules::unsupported(),
);

// TMT 45 — Mind Transfer Protocol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_TRANSFER_PROTOCOL: CardRecord = CardRecord::new(
    "Mind Transfer Protocol",
    "2ddfcc4d-dd5f-42f1-8f33-a8e8b5354534",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// TMT 46 — Mondo Gecko
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONDO_GECKO: CardRecord = CardRecord::new(
    "Mondo Gecko",
    "665e44f9-bc0d-40aa-83a4-f7fe64f2506d",
    "Maël Ollivier-Henry",
    crate::card::CardRules::unsupported(),
);

// TMT 47 — Negate (reprint)
const NEGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mor::NEGATE,
    "52d58fe4-6070-4022-9cd7-c35a11b44525",
    "Ryan Valle",
);

// TMT 48 — Ooze Spill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OOZE_SPILL: CardRecord = CardRecord::new(
    "Ooze Spill",
    "beef76b7-856e-48ab-bc73-e4f456c3a100",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// TMT 49 — Ray Fillet, Man Ray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAY_FILLET_MAN_RAY: CardRecord = CardRecord::new(
    "Ray Fillet, Man Ray",
    "14e0892c-a556-43c3-974f-6be44188da2e",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// TMT 50 — Renet, Temporal Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENET_TEMPORAL_APPRENTICE: CardRecord = CardRecord::new(
    "Renet, Temporal Apprentice",
    "2ae30766-c858-4f4e-a042-14af55698cb2",
    "Yuhong Ding",
    crate::card::CardRules::unsupported(),
);

// TMT 51 — Retro-Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETRO_MUTATION: CardRecord = CardRecord::new(
    "Retro-Mutation",
    "4af284ba-fa54-43c5-8c76-3e1128957452",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// TMT 52 — Return to the Sewers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETURN_TO_THE_SEWERS: CardRecord = CardRecord::new(
    "Return to the Sewers",
    "52e99505-17dc-4051-99f6-e23559fc6c95",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMT 53 — Sewer-veillance Cam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEWER_VEILLANCE_CAM: CardRecord = CardRecord::new(
    "Sewer-veillance Cam",
    "ab47a37b-b66d-4f70-9bf0-4d5ed6b518f3",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// TMT 54 — Stockman, Mad Fly-entist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOCKMAN_MAD_FLY_ENTIST: CardRecord = CardRecord::new(
    "Stockman, Mad Fly-entist",
    "01ca65e2-898e-4150-a6bf-e61b183fd98a",
    "Xavier Ribeiro",
    crate::card::CardRules::unsupported(),
);

// TMT 55 — Turtles in Time
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURTLES_IN_TIME: CardRecord = CardRecord::new(
    "Turtles in Time",
    "bdb3efe7-7b12-4503-83e9-7977eb099db5",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// TMT 56 — Utrom Scientists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UTROM_SCIENTISTS: CardRecord = CardRecord::new(
    "Utrom Scientists",
    "6da89625-5278-49d4-813b-a1a631f114f5",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMT 57 — Anchovy & Banana Pizza
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCHOVY_BANANA_PIZZA: CardRecord = CardRecord::new(
    "Anchovy & Banana Pizza",
    "44443ad2-ef8c-4106-ba1b-ee3fb6fd8b17",
    "Daniel Romanovsky",
    crate::card::CardRules::unsupported(),
);

// TMT 58 — Armaggon, Future Shark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMAGGON_FUTURE_SHARK: CardRecord = CardRecord::new(
    "Armaggon, Future Shark",
    "5989378b-0eac-43cf-bc83-8f7765536789",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// TMT 59 — Bebop, Warthog Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEBOP_WARTHOG_WARRIOR: CardRecord = CardRecord::new(
    "Bebop, Warthog Warrior",
    "371ba16d-73f7-450c-8b1f-c05012a4ca93",
    "April Prime",
    crate::card::CardRules::unsupported(),
);

// TMT 60 — The Cloning of Shredder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_CLONING_OF_SHREDDER: CardRecord = CardRecord::new(
    "The Cloning of Shredder",
    "94449d88-2df4-4850-8ffa-d6d193835dda",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// TMT 61 — Death in the Family
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_IN_THE_FAMILY: CardRecord = CardRecord::new(
    "Death in the Family",
    "2e4f2148-d398-4f3d-9c59-81c87b6a4588",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// TMT 62 — Dream Beavers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_BEAVERS: CardRecord = CardRecord::new(
    "Dream Beavers",
    "600e3bc1-9777-4057-a11e-4f61582636c6",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// TMT 63 — Foot Mystic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOT_MYSTIC: CardRecord = CardRecord::new(
    "Foot Mystic",
    "61e40a18-6cc8-436d-826b-1cf8cd037df3",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// TMT 64 — Insectoid Exterminator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSECTOID_EXTERMINATOR: CardRecord = CardRecord::new(
    "Insectoid Exterminator",
    "ff26f7ff-7f70-4204-9b48-de9e21dc89ec",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// TMT 65 — Lord Dregg, Insect Invader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_DREGG_INSECT_INVADER: CardRecord = CardRecord::new(
    "Lord Dregg, Insect Invader",
    "8d1a9c8a-d0b6-4d83-a168-8078de4b14c7",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// TMT 66 — Madame Null, Power Broker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MADAME_NULL_POWER_BROKER: CardRecord = CardRecord::new(
    "Madame Null, Power Broker",
    "4e712d5c-d6fe-40bb-b8e5-5f50c6ea8d4f",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// TMT 67 — Ninja Teen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NINJA_TEEN: CardRecord = CardRecord::new(
    "Ninja Teen",
    "0825a28f-f60b-4f80-83e3-cad6f9b266ce",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// TMT 68 — Oroku Saki, Shredder Rising
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OROKU_SAKI_SHREDDER_RISING: CardRecord = CardRecord::new(
    "Oroku Saki, Shredder Rising",
    "e65ff1f4-a061-4c31-a78e-92af6e7bc56f",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// TMT 69 — Pain 101
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAIN_101: CardRecord = CardRecord::new(
    "Pain 101",
    "a1c70bf2-b2bd-4585-ba50-304f4dad8e62",
    "Thomas Chamberlain-Keen",
    crate::card::CardRules::unsupported(),
);

// TMT 70 — Paramecia Coloniex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARAMECIA_COLONIEX: CardRecord = CardRecord::new(
    "Paramecia Coloniex",
    "654e2646-78ac-4b08-bed1-3c71355d55fc",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// TMT 71 — Rat King, Verminister
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAT_KING_VERMINISTER: CardRecord = CardRecord::new(
    "Rat King, Verminister",
    "be464d88-8933-46e8-97b0-3be05f1976a3",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMT 72 — Savanti Romero, Time's Exile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVANTI_ROMERO_TIME_S_EXILE: CardRecord = CardRecord::new(
    "Savanti Romero, Time's Exile",
    "01cb8ded-7f77-4b75-b799-23e9f5efb513",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// TMT 73 — Shark Shredder, Killer Clone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHARK_SHREDDER_KILLER_CLONE: CardRecord = CardRecord::new(
    "Shark Shredder, Killer Clone",
    "8f946b3d-7d2a-4210-a909-d16078757e3b",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// TMT 74 — Shredder, Unrelenting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHREDDER_UNRELENTING: CardRecord = CardRecord::new(
    "Shredder, Unrelenting",
    "88a48867-5c65-483c-92b0-f70c53ea2a9e",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// TMT 75 — Shredder's Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHREDDER_S_ARMOR: CardRecord = CardRecord::new(
    "Shredder's Armor",
    "d2cbe512-725f-4884-ac49-a98a78e7d14e",
    "Maël Ollivier-Henry",
    crate::card::CardRules::unsupported(),
);

// TMT 76 — Shredder's Revenge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHREDDER_S_REVENGE: CardRecord = CardRecord::new(
    "Shredder's Revenge",
    "72fadf47-2e0b-4b24-b1bc-7c86780716b6",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// TMT 77 — Shredder's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHREDDER_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Shredder's Technique",
    "99a24349-7d11-421b-a161-c1edbb8f53b1",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// TMT 78 — South Wind Avatar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUTH_WIND_AVATAR: CardRecord = CardRecord::new(
    "South Wind Avatar",
    "c17bc07c-7147-4c84-aa0b-8a0865fba94e",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// TMT 79 — Splinter, Hamato Yoshi
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLINTER_HAMATO_YOSHI: CardRecord = CardRecord::new(
    "Splinter, Hamato Yoshi",
    "9ea9d072-aa98-405e-a475-26f93cc37e53",
    "April Prime",
    crate::card::CardRules::unsupported(),
);

// TMT 80 — Splinter's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLINTER_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Splinter's Technique",
    "fd3a5465-074a-4688-b79b-68e232076581",
    "Jo Cordisco",
    crate::card::CardRules::unsupported(),
);

// TMT 81 — Squirrelanoids
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUIRRELANOIDS: CardRecord = CardRecord::new(
    "Squirrelanoids",
    "be08d2b0-375b-434f-9e6d-060809e0ed34",
    "Eilene Cherie",
    crate::card::CardRules::unsupported(),
);

// TMT 82 — Stomped by the Foot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOMPED_BY_THE_FOOT: CardRecord = CardRecord::new(
    "Stomped by the Foot",
    "3fdec16f-77f5-4792-9698-b9099a204028",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// TMT 83 — Super Shredder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_SHREDDER: CardRecord = CardRecord::new(
    "Super Shredder",
    "37a497b8-e908-4ddc-996e-a8470df72afb",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// TMT 84 — Tunnel Rats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUNNEL_RATS: CardRecord = CardRecord::new(
    "Tunnel Rats",
    "70faf7d8-008a-454a-a21b-702aa661b8f9",
    "Daniel Romanovsky",
    crate::card::CardRules::unsupported(),
);

// TMT 85 — Bot Bashing Time
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOT_BASHING_TIME: CardRecord = CardRecord::new(
    "Bot Bashing Time",
    "08b3af61-d516-45ca-aede-2f50ba23cb07",
    "Xavier Ribeiro",
    crate::card::CardRules::unsupported(),
);

// TMT 86 — Broadcast Takeover
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROADCAST_TAKEOVER: CardRecord = CardRecord::new(
    "Broadcast Takeover",
    "e640e930-0907-40f2-9015-88f8c1e8ee5c",
    "Hokyoung Kim",
    crate::card::CardRules::unsupported(),
);

// TMT 87 — Casey Jones, Jury-Rig Justiciar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASEY_JONES_JURY_RIG_JUSTICIAR: CardRecord = CardRecord::new(
    "Casey Jones, Jury-Rig Justiciar",
    "808a5bc0-0999-47cf-854c-30db6277efe5",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// TMT 88 — Casey Jones, Vigilante
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASEY_JONES_VIGILANTE: CardRecord = CardRecord::new(
    "Casey Jones, Vigilante",
    "a6a3258d-2e9b-4862-b2e3-bbfae9bd4d33",
    "Xavier Ribeiro",
    crate::card::CardRules::unsupported(),
);

// TMT 89 — Cool but Rude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COOL_BUT_RUDE: CardRecord = CardRecord::new(
    "Cool but Rude",
    "a566ab2d-6ec8-4833-8ad6-210378b1a20e",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// TMT 90 — General Traag, Heart of Stone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENERAL_TRAAG_HEART_OF_STONE: CardRecord = CardRecord::new(
    "General Traag, Heart of Stone",
    "e02e971f-6008-4c82-acd2-2aed13009ccb",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// TMT 91 — Hard-Won Jitte
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARD_WON_JITTE: CardRecord = CardRecord::new(
    "Hard-Won Jitte",
    "5d3dc219-f024-49de-b88d-dc1e9e84184c",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// TMT 92 — Improvised Arsenal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPROVISED_ARSENAL: CardRecord = CardRecord::new(
    "Improvised Arsenal",
    "003265d9-b2fc-4916-afc8-53ed2d6aa053",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// TMT 93 — Jennika's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JENNIKA_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Jennika's Technique",
    "df559199-c8b9-455b-aa07-0a042348de96",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// TMT 94 — Manhole Missile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANHOLE_MISSILE: CardRecord = CardRecord::new(
    "Manhole Missile",
    "243e392b-48b0-4e90-ae22-9a696f45e878",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// TMT 95 — Mouser Attack!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUSER_ATTACK: CardRecord = CardRecord::new(
    "Mouser Attack!",
    "058490f4-0ada-45e6-b4f0-e433537f52d6",
    "Erikas Perl",
    crate::card::CardRules::unsupported(),
);

// TMT 96 — Mouser Foundry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUSER_FOUNDRY: CardRecord = CardRecord::new(
    "Mouser Foundry",
    "8aae3bd6-a935-44a5-aa4d-a525b00cbf50",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// TMT 97 — Mutant Town Musicians
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUTANT_TOWN_MUSICIANS: CardRecord = CardRecord::new(
    "Mutant Town Musicians",
    "2dbe9207-06e8-4837-8227-0f960490771b",
    "Leonardo Vincent (Levinky)",
    crate::card::CardRules::unsupported(),
);

// TMT 98 — Null Group Biological Assets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NULL_GROUP_BIOLOGICAL_ASSETS: CardRecord = CardRecord::new(
    "Null Group Biological Assets",
    "eb44c134-fcd8-4ad8-841c-f1723ba93216",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMT 99 — Old Hob, Alleycat Blues
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OLD_HOB_ALLEYCAT_BLUES: CardRecord = CardRecord::new(
    "Old Hob, Alleycat Blues",
    "1515eed9-3b21-42f0-ab57-de5df19317af",
    "Rose Benjamin",
    crate::card::CardRules::unsupported(),
);

// TMT 100 — Purple Dragon Punks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURPLE_DRAGON_PUNKS: CardRecord = CardRecord::new(
    "Purple Dragon Punks",
    "bbe1d7e5-68e2-4458-aa3f-5e97fa8e7cae",
    "Rose Benjamin",
    crate::card::CardRules::unsupported(),
);

// TMT 101 — Raphael, Most Attitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPHAEL_MOST_ATTITUDE: CardRecord = CardRecord::new(
    "Raphael, Most Attitude",
    "88385a87-f931-409f-8a21-250f0866d63d",
    "Thomas Chamberlain-Keen",
    crate::card::CardRules::unsupported(),
);

// TMT 102 — Raphael, Ninja Destroyer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPHAEL_NINJA_DESTROYER: CardRecord = CardRecord::new(
    "Raphael, Ninja Destroyer",
    "eeffadda-cb71-4434-89bf-36db1a36da0b",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// TMT 103 — Raphael, the Nightwatcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPHAEL_THE_NIGHTWATCHER: CardRecord = CardRecord::new(
    "Raphael, the Nightwatcher",
    "6af50b50-3776-4383-b493-7c5dd732c965",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// TMT 104 — Raphael, Tough Turtle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPHAEL_TOUGH_TURTLE: CardRecord = CardRecord::new(
    "Raphael, Tough Turtle",
    "f24c90c8-ed04-4b4e-8b19-c7d07a90c6b8",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// TMT 105 — Raphael's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPHAEL_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Raphael's Technique",
    "7ce8b00f-5a4f-4206-8eb3-e79308e91f47",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// TMT 106 — Ravenous Robots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVENOUS_ROBOTS: CardRecord = CardRecord::new(
    "Ravenous Robots",
    "3b303ea3-9f4d-4c28-9446-285a23f841a0",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// TMT 107 — Rock Soldiers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCK_SOLDIERS: CardRecord = CardRecord::new(
    "Rock Soldiers",
    "0fada65d-fd8d-4be9-b2bb-ea5cac78fdd7",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMT 108 — Slash, Reptile Rampager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLASH_REPTILE_RAMPAGER: CardRecord = CardRecord::new(
    "Slash, Reptile Rampager",
    "025aafbc-5b24-4f5b-9b4a-81f6b9cb5ef3",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// TMT 109 — Spicy Oatmeal Pizza
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPICY_OATMEAL_PIZZA: CardRecord = CardRecord::new(
    "Spicy Oatmeal Pizza",
    "c4aeb45b-13ed-43ad-a366-7be10dec6222",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// TMT 110 — Wingnut, Bat on the Belfry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINGNUT_BAT_ON_THE_BELFRY: CardRecord = CardRecord::new(
    "Wingnut, Bat on the Belfry",
    "40b9b1d0-b7d1-473e-b6a9-a29d527c2f35",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// TMT 111 — Zog, Triceraton Castaway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOG_TRICERATON_CASTAWAY: CardRecord = CardRecord::new(
    "Zog, Triceraton Castaway",
    "a83a70ba-448e-45f3-b23b-6f38af54811f",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// TMT 112 — Courier of Comestibles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COURIER_OF_COMESTIBLES: CardRecord = CardRecord::new(
    "Courier of Comestibles",
    "53f5f704-2265-42bb-bff5-fd9d85bc2bfb",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// TMT 113 — Cowabunga!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COWABUNGA: CardRecord = CardRecord::new(
    "Cowabunga!",
    "0ffce972-bed9-445a-a9a0-4816f156d88d",
    "Thomas Chamberlain-Keen",
    crate::card::CardRules::unsupported(),
);

// TMT 114 — Frog Butler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROG_BUTLER: CardRecord = CardRecord::new(
    "Frog Butler",
    "d1a72d09-9cfc-463a-a9ec-3359003d54da",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// TMT 115 — Groundchuck & Dirtbag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROUNDCHUCK_DIRTBAG: CardRecord = CardRecord::new(
    "Groundchuck & Dirtbag",
    "1563592e-0f21-4488-a3ec-ca766386e423",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// TMT 116 — Guac & Marshmallow Pizza
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUAC_MARSHMALLOW_PIZZA: CardRecord = CardRecord::new(
    "Guac & Marshmallow Pizza",
    "a5405de0-16c9-4e5a-8ceb-00508e212f12",
    "Brandon L. Hunt",
    crate::card::CardRules::unsupported(),
);

// TMT 117 — Leatherhead, Swamp Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEATHERHEAD_SWAMP_STALKER: CardRecord = CardRecord::new(
    "Leatherhead, Swamp Stalker",
    "b1f6b5b5-12ca-468d-bc53-dd0cde60e7b6",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// TMT 118 — Michelangelo, Game Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MICHELANGELO_GAME_MASTER: CardRecord = CardRecord::new(
    "Michelangelo, Game Master",
    "2e914c3d-2eed-48bf-af9a-a8998fd5111d",
    "Eilene Cherie",
    crate::card::CardRules::unsupported(),
);

// TMT 119 — Michelangelo, Improviser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MICHELANGELO_IMPROVISER: CardRecord = CardRecord::new(
    "Michelangelo, Improviser",
    "955848c0-5092-4e13-97c9-5978d44d5586",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// TMT 120 — Michelangelo, Mutant BFF
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MICHELANGELO_MUTANT_BFF: CardRecord = CardRecord::new(
    "Michelangelo, Mutant BFF",
    "02f7281f-b50c-4a1c-a2fc-3caeb6ab7d41",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// TMT 121 — Michelangelo, Weirdness to 11
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MICHELANGELO_WEIRDNESS_TO_11: CardRecord = CardRecord::new(
    "Michelangelo, Weirdness to 11",
    "18477047-218d-4b2a-a086-37431b6a3025",
    "Jason Kiantoro",
    crate::card::CardRules::unsupported(),
);

// TMT 122 — Michelangelo's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MICHELANGELO_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Michelangelo's Technique",
    "3a63c06a-7c59-4b72-b916-e5b6ad78c684",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// TMT 123 — Mona Lisa, Science Geek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONA_LISA_SCIENCE_GEEK: CardRecord = CardRecord::new(
    "Mona Lisa, Science Geek",
    "c2164204-120c-4dff-86ac-46ca9012ccd9",
    "Oriana Menendez",
    crate::card::CardRules::unsupported(),
);

// TMT 124 — Mutagen Man, Living Ooze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUTAGEN_MAN_LIVING_OOZE: CardRecord = CardRecord::new(
    "Mutagen Man, Living Ooze",
    "9f9085f0-3702-462c-8c81-b4576236792d",
    "Ignatius Budi",
    crate::card::CardRules::unsupported(),
);

// TMT 125 — Mutant Chain Reaction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUTANT_CHAIN_REACTION: CardRecord = CardRecord::new(
    "Mutant Chain Reaction",
    "8763e0ee-a48e-424f-8bce-132582d5944b",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// TMT 126 — New Generation's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEW_GENERATION_S_TECHNIQUE: CardRecord = CardRecord::new(
    "New Generation's Technique",
    "f1616a07-e4a1-4574-b341-d6b31d76b3c5",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// TMT 127 — Novel Nunchaku
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOVEL_NUNCHAKU: CardRecord = CardRecord::new(
    "Novel Nunchaku",
    "f37ef012-c566-4d16-acf8-6079244907be",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// TMT 128 — Party Dude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARTY_DUDE: CardRecord = CardRecord::new(
    "Party Dude",
    "d27b6f2a-84df-4097-a66a-8e463db47f58",
    "Gabriel Rubio",
    crate::card::CardRules::unsupported(),
);

// TMT 129 — Primordial Pachyderm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMORDIAL_PACHYDERM: CardRecord = CardRecord::new(
    "Primordial Pachyderm",
    "e1a866e6-4108-4290-9680-8f1652fbcf77",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// TMT 130 — Ragamuffin Raptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGAMUFFIN_RAPTOR: CardRecord = CardRecord::new(
    "Ragamuffin Raptor",
    "2edc1d40-3154-4ec0-88c7-faf35fd5e560",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// TMT 131 — Rocksteady, Crash Courser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCKSTEADY_CRASH_COURSER: CardRecord = CardRecord::new(
    "Rocksteady, Crash Courser",
    "ebbb5756-80fa-406f-8daa-be1f5a6c3c80",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// TMT 132 — Saved by the Shell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVED_BY_THE_SHELL: CardRecord = CardRecord::new(
    "Saved by the Shell",
    "f6314c7f-41dc-4bbd-99db-3a8a1d2977b2",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// TMT 133 — Tenderize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TENDERIZE: CardRecord = CardRecord::new(
    "Tenderize",
    "bc19807b-09e8-4923-abc3-d4bbe8cde5fc",
    "Jo Cordisco",
    crate::card::CardRules::unsupported(),
);

// TMT 134 — Transdimensional Bovine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRANSDIMENSIONAL_BOVINE: CardRecord = CardRecord::new(
    "Transdimensional Bovine",
    "de124563-205c-4f40-8c13-4ae203599912",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// TMT 135 — Turtle Power!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURTLE_POWER: CardRecord = CardRecord::new(
    "Turtle Power!",
    "4079ca90-dcc3-4184-9dfc-a626e0776332",
    "Hokyoung Kim",
    crate::card::CardRules::unsupported(),
);

// TMT 136 — Venus, Torn Between Worlds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENUS_TORN_BETWEEN_WORLDS: CardRecord = CardRecord::new(
    "Venus, Torn Between Worlds",
    "fcb59b16-3d76-478e-9306-969dd2cb1b5a",
    "April Prime",
    crate::card::CardRules::unsupported(),
);

// TMT 137 — West Wind Avatar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEST_WIND_AVATAR: CardRecord = CardRecord::new(
    "West Wind Avatar",
    "f4b8f7e6-9bff-430b-b2f5-4308d57d1194",
    "Oriana Menendez",
    crate::card::CardRules::unsupported(),
);

// TMT 138 — Zoo Escapees
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOO_ESCAPEES: CardRecord = CardRecord::new(
    "Zoo Escapees",
    "f57ba8ff-2f8c-4ca1-9b13-42a7cc213e99",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// TMT 139 — Baxter Stockman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAXTER_STOCKMAN: CardRecord = CardRecord::new(
    "Baxter Stockman",
    "117b1341-2cf0-466e-b3a8-7e1afa42cd4c",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// TMT 140 — Bebop & Rocksteady
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEBOP_ROCKSTEADY: CardRecord = CardRecord::new(
    "Bebop & Rocksteady",
    "535461bd-f763-408b-816f-64b7bfb9210d",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// TMT 141 — Brilliance Unleashed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRILLIANCE_UNLEASHED: CardRecord = CardRecord::new(
    "Brilliance Unleashed",
    "b7ab2110-5aad-46c9-8dc4-1eac24b6f46b",
    "Hokyoung Kim",
    crate::card::CardRules::unsupported(),
);

// TMT 142 — Dark Leo & Shredder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_LEO_SHREDDER: CardRecord = CardRecord::new(
    "Dark Leo & Shredder",
    "bab93474-2f98-49a1-874f-b794bf81bd0c",
    "Thomas Chamberlain-Keen",
    crate::card::CardRules::unsupported(),
);

// TMT 143 — Don & Leo, Problem Solvers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DON_LEO_PROBLEM_SOLVERS: CardRecord = CardRecord::new(
    "Don & Leo, Problem Solvers",
    "d239beaf-4f5b-494e-be5c-ffa8b6e28bde",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// TMT 144 — Don & Raph, Hard Science
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DON_RAPH_HARD_SCIENCE: CardRecord = CardRecord::new(
    "Don & Raph, Hard Science",
    "4f38126c-26a9-4447-801f-4f19e84c4aa5",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// TMT 145 — EPF Point Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EPF_POINT_SQUAD: CardRecord = CardRecord::new(
    "EPF Point Squad",
    "faab52c0-ce79-40af-a156-b193a62d439e",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// TMT 146 — Foot Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOT_ELITE: CardRecord = CardRecord::new(
    "Foot Elite",
    "24bd571e-652a-4e7c-afc6-a45f0ccf62f6",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// TMT 147 — Foot Ninjas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOT_NINJAS: CardRecord = CardRecord::new(
    "Foot Ninjas",
    "abb1ab9c-b067-4b75-8e5b-a893b7948df1",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMT 148 — Genghis Frog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENGHIS_FROG: CardRecord = CardRecord::new(
    "Genghis Frog",
    "7df26085-eedb-4bdd-a60a-aabfbe9c3157",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// TMT 149 — Go Ninja Go
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GO_NINJA_GO: CardRecord = CardRecord::new(
    "Go Ninja Go",
    "90e68cd8-ea76-4bd1-9199-474da9c3e8bf",
    "Patrick Gañas",
    crate::card::CardRules::unsupported(),
);

// TMT 150 — Ice Cream Kitty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_CREAM_KITTY: CardRecord = CardRecord::new(
    "Ice Cream Kitty",
    "66081fd3-2457-4602-96c4-3075ddfec1c2",
    "Maël Ollivier-Henry",
    crate::card::CardRules::unsupported(),
);

// TMT 151 — Karai, Future of the Foot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARAI_FUTURE_OF_THE_FOOT: CardRecord = CardRecord::new(
    "Karai, Future of the Foot",
    "0dded2d4-1640-4431-809e-403b51b27db6",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// TMT 152 — Karai's Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARAI_S_TECHNIQUE: CardRecord = CardRecord::new(
    "Karai's Technique",
    "037fe188-3355-4724-b2eb-e2448fc55607",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// TMT 153 — Krang & Shredder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRANG_SHREDDER: CardRecord = CardRecord::new(
    "Krang & Shredder",
    "9b5437e2-e3f4-4c19-a2aa-a75f65a001bb",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// TMT 154 — The Last Ronin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_LAST_RONIN: CardRecord = CardRecord::new(
    "The Last Ronin",
    "72ab3ccf-3ddb-4dd1-9cfb-98802a18d954",
    "Hokyoung Kim",
    crate::card::CardRules::unsupported(),
);

// TMT 155 — Lessons from Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LESSONS_FROM_LIFE: CardRecord = CardRecord::new(
    "Lessons from Life",
    "0f886117-aff3-4db7-9cf5-7cbe94c8cd02",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// TMT 156 — Mechanized Ninja Cavalry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MECHANIZED_NINJA_CAVALRY: CardRecord = CardRecord::new(
    "Mechanized Ninja Cavalry",
    "2cb65388-dc6c-4e2a-93ac-49ea484849e9",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// TMT 157 — Mikey & Don, Party Planners
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIKEY_DON_PARTY_PLANNERS: CardRecord = CardRecord::new(
    "Mikey & Don, Party Planners",
    "6353df1a-9a1b-41fd-985b-8c8acba36c23",
    "Gabriel Rubio",
    crate::card::CardRules::unsupported(),
);

// TMT 158 — Mikey & Leo, Chaos & Order
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIKEY_LEO_CHAOS_ORDER: CardRecord = CardRecord::new(
    "Mikey & Leo, Chaos & Order",
    "9cfeb2b4-937c-4bcc-bb03-467cc0effba2",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// TMT 159 — Mouser Mark III
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUSER_MARK_III: CardRecord = CardRecord::new(
    "Mouser Mark III",
    "608a13b4-0b94-4be5-ac4b-e939901e8419",
    "Gabriel Tanko",
    crate::card::CardRules::unsupported(),
);

// TMT 160 — The Neutrinos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_NEUTRINOS: CardRecord = CardRecord::new(
    "The Neutrinos",
    "1308dadc-08a9-40bd-98a4-fb66d792e27d",
    "Brandon L. Hunt",
    crate::card::CardRules::unsupported(),
);

// TMT 161 — Nobody
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBODY: CardRecord = CardRecord::new(
    "Nobody",
    "0966b8ff-61d3-4394-a357-97c35f042a29",
    "Yuhong Ding",
    crate::card::CardRules::unsupported(),
);

// TMT 162 — North Wind Avatar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NORTH_WIND_AVATAR: CardRecord = CardRecord::new(
    "North Wind Avatar",
    "41439cfe-bb3e-42f3-9d81-de9db537ce68",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// TMT 163 — Pizza Face, Gastromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIZZA_FACE_GASTROMANCER: CardRecord = CardRecord::new(
    "Pizza Face, Gastromancer",
    "b03cf0bb-3207-4e8e-bb3f-e3e4367aa86e",
    "Villarrte",
    crate::card::CardRules::unsupported(),
);

// TMT 164 — Punk Frogs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUNK_FROGS: CardRecord = CardRecord::new(
    "Punk Frogs",
    "375c91a3-53c4-4f2d-bf7b-2c79a219d3ae",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// TMT 165 — Putrid Pals
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUTRID_PALS: CardRecord = CardRecord::new(
    "Putrid Pals",
    "42cdb88d-675a-43bb-b85c-51c4cc526315",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// TMT 166 — Raph & Leo, Sibling Rivals
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPH_LEO_SIBLING_RIVALS: CardRecord = CardRecord::new(
    "Raph & Leo, Sibling Rivals",
    "49293f77-5d7b-4106-b485-db6ce0ed37e6",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// TMT 167 — Raph & Mikey, Troublemakers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPH_MIKEY_TROUBLEMAKERS: CardRecord = CardRecord::new(
    "Raph & Mikey, Troublemakers",
    "8795fba4-0ff3-4c04-a81c-60408608a00c",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// TMT 168 — Slithering Cryptid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLITHERING_CRYPTID: CardRecord = CardRecord::new(
    "Slithering Cryptid",
    "6d35cb39-8832-4cf1-be73-8de49fbea529",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// TMT 169 — Splinter, Radical Rat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLINTER_RADICAL_RAT: CardRecord = CardRecord::new(
    "Splinter, Radical Rat",
    "f0797466-c527-4d35-86bc-e0e90fd04073",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// TMT 170 — Tainted Treats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAINTED_TREATS: CardRecord = CardRecord::new(
    "Tainted Treats",
    "fa684608-ecd3-43e2-99a1-71318f133e29",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// TMT 171 — Tokka & Rahzar, Terrible Twos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOKKA_RAHZAR_TERRIBLE_TWOS: CardRecord = CardRecord::new(
    "Tokka & Rahzar, Terrible Twos",
    "284f9012-a58d-41da-be7c-962dca052711",
    "Yuhong Ding",
    crate::card::CardRules::unsupported(),
);

// TMT 172 — Chrome Dome
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROME_DOME: CardRecord = CardRecord::new(
    "Chrome Dome",
    "994a01eb-2689-49e7-be23-0713da9da9a8",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// TMT 173 — Everything Pizza
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVERYTHING_PIZZA: CardRecord = CardRecord::new(
    "Everything Pizza",
    "df2cdaa5-9ea0-4aa5-89d3-9edf40fa2a39",
    "James Bousema",
    crate::card::CardRules::unsupported(),
);

// TMT 174 — Henchbots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HENCHBOTS: CardRecord = CardRecord::new(
    "Henchbots",
    "d77aa46e-fa57-4427-b0a3-0a957dc994dc",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// TMT 175 — Krang, Utrom Warlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRANG_UTROM_WARLORD: CardRecord = CardRecord::new(
    "Krang, Utrom Warlord",
    "88d36c32-d6f9-46e4-9cc8-08e6c0ff05d2",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// TMT 176 — Omni-Cheese Pizza
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMNI_CHEESE_PIZZA: CardRecord = CardRecord::new(
    "Omni-Cheese Pizza",
    "f2c8397c-2014-4a43-9ed7-41795880011f",
    "Gabriel Tanko",
    crate::card::CardRules::unsupported(),
);

// TMT 177 — The Ooze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_OOZE: CardRecord = CardRecord::new(
    "The Ooze",
    "1f9bd4da-4626-40ba-95f4-14e3de36f989",
    "Gabriel Tanko",
    crate::card::CardRules::unsupported(),
);

// TMT 178 — Skateboard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKATEBOARD: CardRecord = CardRecord::new(
    "Skateboard",
    "deadb6d8-3eea-4261-a07c-8536df89e85c",
    "Hokyoung Kim",
    crate::card::CardRules::unsupported(),
);

// TMT 179 — Technodrome
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TECHNODROME: CardRecord = CardRecord::new(
    "Technodrome",
    "287f5ab0-b15b-4507-8a24-585f9a9841ad",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// TMT 180 — Turtle Blimp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURTLE_BLIMP: CardRecord = CardRecord::new(
    "Turtle Blimp",
    "6e118fc5-b0ae-4592-8292-d611856ae203",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// TMT 181 — Turtle Van
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURTLE_VAN: CardRecord = CardRecord::new(
    "Turtle Van",
    "fce6a8b6-b43c-4045-9378-97b2463f9b4d",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// TMT 182 — Weather Maker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEATHER_MAKER: CardRecord = CardRecord::new(
    "Weather Maker",
    "aaba62c1-760b-491b-8047-869bb1d3f67e",
    "Florent Lebrun",
    crate::card::CardRules::unsupported(),
);

// TMT 183 — Dimension X
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMENSION_X: CardRecord = CardRecord::new(
    "Dimension X",
    "1c244fc2-70f0-4149-b0d2-d49fc6bac2b0",
    "Maël Ollivier-Henry",
    crate::card::CardRules::unsupported(),
);

// TMT 184 — Escape Tunnel (reprint)
const ESCAPE_TUNNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mkm::ESCAPE_TUNNEL,
    "5df90940-15ea-418c-8547-6c75d69ec6d3",
    "Aenami",
);

// TMT 185 — Foot Headquarters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOT_HEADQUARTERS: CardRecord = CardRecord::new(
    "Foot Headquarters",
    "45e68113-3f05-4547-947d-4cb9ebfa73c7",
    "Hokyoung Kim",
    crate::card::CardRules::unsupported(),
);

// TMT 186 — Illegitimate Business
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLEGITIMATE_BUSINESS: CardRecord = CardRecord::new(
    "Illegitimate Business",
    "71597acf-1ce6-46a8-b6c0-88755c8a377c",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// TMT 187 — Mutant Town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUTANT_TOWN: CardRecord = CardRecord::new(
    "Mutant Town",
    "c6eac43d-08b6-45a4-803b-10a321a241d7",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// TMT 188 — Northampton Farm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NORTHAMPTON_FARM: CardRecord = CardRecord::new(
    "Northampton Farm",
    "dbca168e-095f-4fbc-88f8-3048d83caf94",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// TMT 189 — TCRI Building
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TCRI_BUILDING: CardRecord = CardRecord::new(
    "TCRI Building",
    "8817a1d6-ef39-4e7c-8277-74aea012803b",
    "Aenami",
    crate::card::CardRules::unsupported(),
);

// TMT 190 — Turtle Lair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURTLE_LAIR: CardRecord = CardRecord::new(
    "Turtle Lair",
    "c50c4580-979b-4863-86b9-16e258198972",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// TMT 191 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "1b405611-27d4-435a-8e8e-6d528626fd27",
    "BEMOCS",
);

// TMT 192 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "be2319a6-d089-4d13-8a7b-3552e654cdc5",
    "BEMOCS",
);

// TMT 193 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "41c7688d-8155-45fd-83ba-ff9be4d414a3",
    "BEMOCS",
);

// TMT 194 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "6243f79f-b0e3-4fba-b899-7535e1a277e2",
    "BEMOCS",
);

// TMT 195 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "6194850b-d70a-4f3e-be3e-bfb23ce1170b",
    "BEMOCS",
);

// TMT 196 — Leonardo, Leader in Blue (alternate printing)
const LEONARDO_LEADER_IN_BLUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEONARDO_LEADER_IN_BLUE,
    1,
    "b7f4581f-e0c1-4cc3-b94c-dd104606c4af",
    "Daniel Elson",
);

// TMT 197 — Donatello, Way with Machines (alternate printing)
const DONATELLO_WAY_WITH_MACHINES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DONATELLO_WAY_WITH_MACHINES,
    1,
    "14ff6944-fc47-4c90-b577-cad3a1355165",
    "Daniel Elson",
);

// TMT 198 — Michelangelo, Mutant BFF (alternate printing)
const MICHELANGELO_MUTANT_BFF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MICHELANGELO_MUTANT_BFF,
    1,
    "ff378d21-eb8c-4e6d-9e63-5f1b6030a001",
    "Daniel Elson",
);

// TMT 199 — Raphael, Most Attitude (alternate printing)
const RAPHAEL_MOST_ATTITUDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAPHAEL_MOST_ATTITUDE,
    1,
    "65c50cfe-1452-407c-a651-6fc8bcc92a98",
    "Daniel Elson",
);

// TMT 200 — Mikey & Leo, Chaos & Order (alternate printing)
const MIKEY_LEO_CHAOS_ORDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIKEY_LEO_CHAOS_ORDER,
    1,
    "919d670a-7555-4981-afba-b7c0706458d8",
    "Marija Tiurina",
);

// TMT 201 — Krang & Shredder (alternate printing)
const KRANG_SHREDDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRANG_SHREDDER,
    1,
    "5fbf2786-c591-49f8-8cfe-7137f02d0bb9",
    "Marija Tiurina",
);

// TMT 202 — Renet, Temporal Apprentice (alternate printing)
const RENET_TEMPORAL_APPRENTICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RENET_TEMPORAL_APPRENTICE,
    1,
    "fc367cbb-1e39-475f-9611-56e9638877d3",
    "Marija Tiurina",
);

// TMT 203 — Jennika, Bad Apple Big Sister (alternate printing)
const JENNIKA_BAD_APPLE_BIG_SISTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JENNIKA_BAD_APPLE_BIG_SISTER,
    1,
    "64c6cf57-bc43-4b36-a01c-f4b2fadc8ce2",
    "Marija Tiurina",
);

// TMT 204 — Bebop & Rocksteady (alternate printing)
const BEBOP_ROCKSTEADY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEBOP_ROCKSTEADY,
    1,
    "da099bf3-786c-4c49-9a7f-3603df9c6cbc",
    "Marija Tiurina",
);

// TMT 205 — Don & Raph, Hard Science (alternate printing)
const DON_RAPH_HARD_SCIENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DON_RAPH_HARD_SCIENCE,
    1,
    "51d67bd5-b704-4e9d-9c39-c045f3c7a4e5",
    "Marija Tiurina",
);

// TMT 206 — April, Reporter of the Weird (alternate printing)
const APRIL_REPORTER_OF_THE_WEIRD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &APRIL_REPORTER_OF_THE_WEIRD,
    1,
    "cbce9297-f601-4478-8865-6848700c4a42",
    "Marija Tiurina",
);

// TMT 207 — Casey Jones, Jury-Rig Justiciar (alternate printing)
const CASEY_JONES_JURY_RIG_JUSTICIAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CASEY_JONES_JURY_RIG_JUSTICIAR,
    1,
    "f1210cc4-b968-4454-b314-4e9af9f56946",
    "Marija Tiurina",
);

// TMT 208 — Slash, Reptile Rampager (alternate printing)
const SLASH_REPTILE_RAMPAGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLASH_REPTILE_RAMPAGER,
    1,
    "c6dc0dbf-0d41-4677-a200-62a966876c1d",
    "Marija Tiurina",
);

// TMT 209 — Foot Ninjas (alternate printing)
const FOOT_NINJAS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOOT_NINJAS,
    1,
    "fa84045f-f14a-42fc-ba07-9674239b3587",
    "Jim Cheung & Jay David Ramos",
);

// TMT 210 — Shredder, Unrelenting (alternate printing)
const SHREDDER_UNRELENTING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHREDDER_UNRELENTING,
    1,
    "43c80d9e-0180-491a-ab08-2ba00f273604",
    "Jim Cheung & Jay David Ramos",
);

// TMT 211 — Leonardo, Cutting Edge (alternate printing)
const LEONARDO_CUTTING_EDGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEONARDO_CUTTING_EDGE,
    1,
    "a183e8f9-f2e3-4219-9efe-839e21aad4b7",
    "Jim Cheung & Jay David Ramos",
);

// TMT 212 — Donatello, Gadget Master (alternate printing)
const DONATELLO_GADGET_MASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DONATELLO_GADGET_MASTER,
    1,
    "22b2d126-ddb4-413f-94ca-1ff39291c7c1",
    "Jim Cheung & Jay David Ramos",
);

// TMT 213 — Raphael, the Nightwatcher (alternate printing)
const RAPHAEL_THE_NIGHTWATCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAPHAEL_THE_NIGHTWATCHER,
    1,
    "d2011f85-7ae2-423f-90b9-abbc69fd1056",
    "Jim Cheung & Jay David Ramos",
);

// TMT 214 — Michelangelo, Weirdness to 11 (alternate printing)
const MICHELANGELO_WEIRDNESS_TO_11_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MICHELANGELO_WEIRDNESS_TO_11,
    1,
    "4b1f1162-9470-4dd7-94ef-83c09b50d8ae",
    "Jim Cheung & Jay David Ramos",
);

// TMT 215 — Leonardo, Sewer Samurai (alternate printing)
const LEONARDO_SEWER_SAMURAI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEONARDO_SEWER_SAMURAI,
    1,
    "1c1b662b-3cf7-4dae-9657-927d7ea6cffc",
    "Andrew Griffith",
);

// TMT 216 — Donatello, Mutant Mechanic (alternate printing)
const DONATELLO_MUTANT_MECHANIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DONATELLO_MUTANT_MECHANIC,
    1,
    "b6ca4caa-0b4b-46ae-9b43-1f27f5f6a1e3",
    "Andrew Griffith",
);

// TMT 217 — Super Shredder (alternate printing)
const SUPER_SHREDDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPER_SHREDDER,
    1,
    "819798cf-f685-49b6-b1fd-6bb75fc1f050",
    "Andrew Griffith",
);

// TMT 218 — Raphael, Ninja Destroyer (alternate printing)
const RAPHAEL_NINJA_DESTROYER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAPHAEL_NINJA_DESTROYER,
    1,
    "183c6c8d-3e5c-4a7b-bd5a-cbaa0a781a58",
    "Joshua Alvarado",
);

// TMT 219 — Michelangelo, Improviser (alternate printing)
const MICHELANGELO_IMPROVISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MICHELANGELO_IMPROVISER,
    1,
    "e5737e15-60d1-4423-bd3c-8d21f274a548",
    "Joshua Alvarado",
);

// TMT 220 — Dark Leo & Shredder (alternate printing)
const DARK_LEO_SHREDDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DARK_LEO_SHREDDER,
    1,
    "b8988f95-1c0b-4886-ad3f-b9d38edb672d",
    "Andrew Griffith",
);

// TMT 221 — Krang, Utrom Warlord (alternate printing)
const KRANG_UTROM_WARLORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRANG_UTROM_WARLORD,
    1,
    "9b8a9315-893d-4eee-a0a8-44f2372202a8",
    "Joshua Alvarado",
);

// TMT 222 — Technodrome (alternate printing)
const TECHNODROME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TECHNODROME,
    1,
    "6e7b458c-c4be-44d6-905a-daf3916f0b27",
    "Terry Dodson",
);

// TMT 223 — The Last Ronin's Technique (alternate printing)
const THE_LAST_RONIN_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_RONIN_S_TECHNIQUE,
    1,
    "e77abb3a-8034-4aa9-baac-21a223cde74b",
    "Caleb Meurer",
);

// TMT 224 — Leonardo's Technique (alternate printing)
const LEONARDO_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEONARDO_S_TECHNIQUE,
    1,
    "2884abe5-80f4-4beb-850a-98f91ee929e3",
    "Gregg Schigiel",
);

// TMT 225 — Sally Pride, Lioness Leader (alternate printing)
const SALLY_PRIDE_LIONESS_LEADER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SALLY_PRIDE_LIONESS_LEADER,
    1,
    "e07e1c31-d6f0-491c-b231-e1df87c9ea9f",
    "Gregg Schigiel",
);

// TMT 226 — Triceraton Commander (alternate printing)
const TRICERATON_COMMANDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRICERATON_COMMANDER,
    1,
    "71ce86cc-6367-4166-82f5-7e794a9e4c5d",
    "Florey",
);

// TMT 227 — April O'Neil, Hacktivist (alternate printing)
const APRIL_O_NEIL_HACKTIVIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &APRIL_O_NEIL_HACKTIVIST,
    1,
    "6ccc8ba7-d5c1-4c52-9b57-21f847215f80",
    "Felipe Sobreiro",
);

// TMT 228 — Donatello's Technique (alternate printing)
const DONATELLO_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DONATELLO_S_TECHNIQUE,
    1,
    "3d3d0a9a-3bde-451f-9d5a-af6c2a393953",
    "Florey",
);

// TMT 229 — Kitsune's Technique (alternate printing)
const KITSUNE_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KITSUNE_S_TECHNIQUE,
    1,
    "4996c35b-892a-4e9e-8337-cc656d8cecf8",
    "William Tempest",
);

// TMT 230 — Krang, Master Mind (alternate printing)
const KRANG_MASTER_MIND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRANG_MASTER_MIND,
    1,
    "3942f813-6241-49f5-8df3-e60e2e332410",
    "Florey",
);

// TMT 231 — Mondo Gecko (alternate printing)
const MONDO_GECKO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MONDO_GECKO,
    1,
    "018e9fd1-3e7f-4909-bab1-3edb9c9e4326",
    "Florey",
);

// TMT 232 — Shredder's Technique (alternate printing)
const SHREDDER_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHREDDER_S_TECHNIQUE,
    1,
    "4208f96f-49ea-46d9-b6fc-2093ded1fadb",
    "Florey",
);

// TMT 233 — Splinter's Technique (alternate printing)
const SPLINTER_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPLINTER_S_TECHNIQUE,
    1,
    "7111e3e9-7175-4231-976c-c73e7d4cb40d",
    "Daniel Elson",
);

// TMT 234 — Broadcast Takeover (alternate printing)
const BROADCAST_TAKEOVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BROADCAST_TAKEOVER,
    1,
    "c4a798f3-9adf-4748-ae87-5f44762491ff",
    "Florey",
);

// TMT 235 — Casey Jones, Vigilante (alternate printing)
const CASEY_JONES_VIGILANTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CASEY_JONES_VIGILANTE,
    1,
    "4b9d7244-f3db-4ec1-95c8-64321d0343f4",
    "Florey",
);

// TMT 236 — Jennika's Technique (alternate printing)
const JENNIKA_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JENNIKA_S_TECHNIQUE,
    1,
    "562395ef-fb28-4b34-b03b-5dbf0507fdc3",
    "Felipe Sobreiro",
);

// TMT 237 — Raphael's Technique (alternate printing)
const RAPHAEL_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAPHAEL_S_TECHNIQUE,
    1,
    "27619fa6-f9bf-4e9c-9d5e-bc0d050aacd6",
    "Daniel Elson",
);

// TMT 238 — Groundchuck & Dirtbag (alternate printing)
const GROUNDCHUCK_DIRTBAG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GROUNDCHUCK_DIRTBAG,
    1,
    "53a7b366-f27e-4aad-a732-11b2f2c6e452",
    "Michael Walsh",
);

// TMT 239 — Michelangelo's Technique (alternate printing)
const MICHELANGELO_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MICHELANGELO_S_TECHNIQUE,
    1,
    "e5f41ef3-9f3a-43ae-bb17-626788f84c01",
    "Florey",
);

// TMT 240 — New Generation's Technique (alternate printing)
const NEW_GENERATION_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NEW_GENERATION_S_TECHNIQUE,
    1,
    "081944c3-f1c8-4906-911d-a9133835218c",
    "William Tempest",
);

// TMT 241 — Bebop & Rocksteady (alternate printing)
const BEBOP_ROCKSTEADY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BEBOP_ROCKSTEADY,
    2,
    "9c925f63-9559-4a57-b57b-56ba296042e6",
    "Michael Walsh",
);

// TMT 242 — Don & Leo, Problem Solvers (alternate printing)
const DON_LEO_PROBLEM_SOLVERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DON_LEO_PROBLEM_SOLVERS,
    1,
    "807ed47f-53a6-40e5-a1d2-82414299c5e1",
    "Florey",
);

// TMT 243 — Don & Raph, Hard Science (alternate printing)
const DON_RAPH_HARD_SCIENCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DON_RAPH_HARD_SCIENCE,
    2,
    "3a74a878-7fc4-44a6-ac4a-0eb964a56e77",
    "Caleb Meurer",
);

// TMT 244 — Karai's Technique (alternate printing)
const KARAI_S_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KARAI_S_TECHNIQUE,
    1,
    "e39a0633-3f65-4ae1-aef0-99018bbfa4f2",
    "Caleb Meurer",
);

// TMT 245 — Krang & Shredder (alternate printing)
const KRANG_SHREDDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KRANG_SHREDDER,
    2,
    "eaba0387-dbbf-4589-949c-2acbdcd6b5bd",
    "Daniel Elson",
);

// TMT 246 — Mikey & Don, Party Planners (alternate printing)
const MIKEY_DON_PARTY_PLANNERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIKEY_DON_PARTY_PLANNERS,
    1,
    "6edb8f34-f6d2-42d8-aa3f-dc8053dfb079",
    "Caleb Meurer",
);

// TMT 247 — Mikey & Leo, Chaos & Order (alternate printing)
const MIKEY_LEO_CHAOS_ORDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MIKEY_LEO_CHAOS_ORDER,
    2,
    "53f1ae0b-e3fd-467a-9fc2-a5d6cce47765",
    "Florey",
);

// TMT 248 — North Wind Avatar (alternate printing)
const NORTH_WIND_AVATAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NORTH_WIND_AVATAR,
    1,
    "d4316e3e-550f-459d-bbcb-1006d029246d",
    "Tyler Walpole",
);

// TMT 249 — Raph & Leo, Sibling Rivals (alternate printing)
const RAPH_LEO_SIBLING_RIVALS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAPH_LEO_SIBLING_RIVALS,
    1,
    "397036c0-57e7-40e2-a8cb-06ef54077bb4",
    "William Tempest",
);

// TMT 250 — Raph & Mikey, Troublemakers (alternate printing)
const RAPH_MIKEY_TROUBLEMAKERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAPH_MIKEY_TROUBLEMAKERS,
    1,
    "20882daf-a2ae-40ec-a363-8d564a2f3b2a",
    "Tyler Walpole",
);

// TMT 251 — Splinter, Radical Rat (alternate printing)
const SPLINTER_RADICAL_RAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPLINTER_RADICAL_RAT,
    1,
    "4673c261-8c4e-42c1-a4c9-d40bd7ce63b5",
    "Florey",
);

// TMT 252 — Tokka & Rahzar, Terrible Twos (alternate printing)
const TOKKA_RAHZAR_TERRIBLE_TWOS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOKKA_RAHZAR_TERRIBLE_TWOS,
    1,
    "e7900a00-39be-40f2-bb36-2badfb98af3a",
    "Caleb Meurer",
);

// TMT 253 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "ec9ecf19-9f9d-4642-ac70-ddc08fad7e17",
    "Gaboleps",
);

// TMT 254 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "57d9b053-ed45-41f3-a0ab-0a08c41f587a",
    "Gaboleps",
);

// TMT 255 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "20f26dde-d1a3-4d0f-9ed9-cfcd9e4ce01e",
    "Gaboleps",
);

// TMT 256 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "b7bf8555-019d-4a08-8004-b31fe7bf494d",
    "Gaboleps",
);

// TMT 257 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "2f60db2d-7594-43af-8554-ba54e9023306",
    "Gaboleps",
);

// TMT 258 — Agent Bishop, Man in Black (alternate printing)
const AGENT_BISHOP_MAN_IN_BLACK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGENT_BISHOP_MAN_IN_BLACK,
    1,
    "a9c717bf-0b1d-4234-8909-22488aed7379",
    "Adrián Rodríguez Pérez",
);

// TMT 259 — Prehistoric Pet (alternate printing)
const PREHISTORIC_PET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PREHISTORIC_PET,
    1,
    "b74bc1ad-72b0-4ea8-a43c-040a6fe0276b",
    "Jakob Eirich",
);

// TMT 260 — Turncoat Kunoichi (alternate printing)
const TURNCOAT_KUNOICHI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TURNCOAT_KUNOICHI,
    1,
    "415d29b5-778b-4a22-bc9b-d3491e60d99d",
    "Manuel Castañón",
);

// TMT 261 — Turtles Forever (alternate printing)
const TURTLES_FOREVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TURTLES_FOREVER,
    1,
    "53c9184f-fa56-447e-b58d-a17eb5485cd3",
    "Devin Elle Kurtz",
);

// TMT 262 — Kitsune, Dragon's Daughter (alternate printing)
const KITSUNE_DRAGON_S_DAUGHTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KITSUNE_DRAGON_S_DAUGHTER,
    1,
    "6634e5d1-b16c-4aba-aab1-e3991022e5c6",
    "Robin Har",
);

// TMT 263 — Turtles in Time (alternate printing)
const TURTLES_IN_TIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TURTLES_IN_TIME,
    1,
    "0b39c8fa-adbc-4ace-992d-45306e53bad4",
    "Inkognit",
);

// TMT 264 — Armaggon, Future Shark (alternate printing)
const ARMAGGON_FUTURE_SHARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARMAGGON_FUTURE_SHARK,
    1,
    "00436b30-d342-4c1d-81af-8f305bf53c86",
    "Mathias Kollros",
);

// TMT 265 — Madame Null, Power Broker (alternate printing)
const MADAME_NULL_POWER_BROKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MADAME_NULL_POWER_BROKER,
    1,
    "b4676f98-2b9d-4a1b-9847-2a7ff9dc2676",
    "Irina Nordsol",
);

// TMT 266 — Rat King, Verminister (alternate printing)
const RAT_KING_VERMINISTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAT_KING_VERMINISTER,
    1,
    "c8c0be82-6448-4ae9-aed6-147b8e2988f0",
    "Miklós Ligeti",
);

// TMT 267 — Savanti Romero, Time's Exile (alternate printing)
const SAVANTI_ROMERO_TIME_S_EXILE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAVANTI_ROMERO_TIME_S_EXILE,
    1,
    "96549dcb-86e4-4d88-9aea-6ceb51b109eb",
    "Michele Giorgi",
);

// TMT 268 — Shark Shredder, Killer Clone (alternate printing)
const SHARK_SHREDDER_KILLER_CLONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHARK_SHREDDER_KILLER_CLONE,
    1,
    "99841fff-706c-469a-8551-6d397b0d6f8a",
    "Nicholas Gregory",
);

// TMT 269 — South Wind Avatar (alternate printing)
const SOUTH_WIND_AVATAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOUTH_WIND_AVATAR,
    1,
    "ac182338-ab26-4f9e-87f9-be83c5cc7cd7",
    "InHyuk Lee",
);

// TMT 270 — Improvised Arsenal (alternate printing)
const IMPROVISED_ARSENAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IMPROVISED_ARSENAL,
    1,
    "02654f5c-177d-4088-8a4d-cafa03943337",
    "Leanna Crossan",
);

// TMT 271 — Ravenous Robots (alternate printing)
const RAVENOUS_ROBOTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAVENOUS_ROBOTS,
    1,
    "c3e68d0d-c259-423e-a319-8b2397c4892c",
    "Kevin Sidharta",
);

// TMT 272 — Leatherhead, Swamp Stalker (alternate printing)
const LEATHERHEAD_SWAMP_STALKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEATHERHEAD_SWAMP_STALKER,
    1,
    "745b5a3b-6fbb-467c-91a8-ebc2bfe144c4",
    "Lie Setiawan",
);

// TMT 273 — Mutagen Man, Living Ooze (alternate printing)
const MUTAGEN_MAN_LIVING_OOZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MUTAGEN_MAN_LIVING_OOZE,
    1,
    "3630d2ad-9bfe-4e86-addb-c4b620b52b14",
    "Ignatius Budi",
);

// TMT 274 — Transdimensional Bovine (alternate printing)
const TRANSDIMENSIONAL_BOVINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRANSDIMENSIONAL_BOVINE,
    1,
    "56b62da5-80eb-4058-bcad-b7e2b374a9ba",
    "Lius Lasahido",
);

// TMT 275 — Turtle Power! (alternate printing)
const TURTLE_POWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TURTLE_POWER,
    1,
    "f0b8dc67-40a6-4e54-bf4d-2afb59c58fef",
    "Hokyoung Kim",
);

// TMT 276 — Chrome Dome (alternate printing)
const CHROME_DOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHROME_DOME,
    1,
    "8bae4950-bf6d-4e96-91f9-26d27021b748",
    "Mathias Kollros",
);

// TMT 277 — The Ooze (alternate printing)
const THE_OOZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_OOZE,
    1,
    "511f6a96-520d-4e6d-aba8-c553c5c9c0a0",
    "Gabriel Tanko",
);

// TMT 278 — Turtle Van (alternate printing)
const TURTLE_VAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TURTLE_VAN,
    1,
    "16ceb9da-cf42-4202-a3ae-265f24532c04",
    "Jakob Eirich",
);

// TMT 279 — Weather Maker (alternate printing)
const WEATHER_MAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEATHER_MAKER,
    1,
    "73a5749e-eec1-4dfc-acbc-b6e4fcabe9a4",
    "Florent Lebrun",
);

// TMT 280 — Northampton Farm (alternate printing)
const NORTHAMPTON_FARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NORTHAMPTON_FARM,
    1,
    "e8770f21-c869-400c-9031-2eb7c685910d",
    "Marina Ortega Lorente",
);

// TMT 281 — Leonardo, Cutting Edge (alternate printing)
const LEONARDO_CUTTING_EDGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LEONARDO_CUTTING_EDGE,
    2,
    "5aaae49a-75bf-44fd-b819-415cbcfa0827",
    "A4Mitsuori",
);

// TMT 282 — April O'Neil, Hacktivist (alternate printing)
const APRIL_O_NEIL_HACKTIVIST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &APRIL_O_NEIL_HACKTIVIST,
    2,
    "83f216cf-4b48-464f-ac65-655c1941f15a",
    "Ryo Kamei",
);

// TMT 283 — Donatello, Gadget Master (alternate printing)
const DONATELLO_GADGET_MASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DONATELLO_GADGET_MASTER,
    2,
    "82bc7c05-d107-43db-89ba-5c5984925d72",
    "Kotakan",
);

// TMT 284 — Turtles in Time (alternate printing)
const TURTLES_IN_TIME_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TURTLES_IN_TIME,
    2,
    "36d5aba8-03e0-4ffb-9f0d-bc9d824fec95",
    "Sansyu",
);

// TMT 285 — Super Shredder (alternate printing)
const SUPER_SHREDDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SUPER_SHREDDER,
    2,
    "9b89c6c0-97c4-4cd4-9f1e-ab229f2a2af0",
    "Inuchiyo Meimaru",
);

// TMT 286 — Casey Jones, Vigilante (alternate printing)
const CASEY_JONES_VIGILANTE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CASEY_JONES_VIGILANTE,
    2,
    "07959845-63ea-4932-ae8a-743fe00b572a",
    "Koji Nishino",
);

// TMT 287 — Raphael, the Nightwatcher (alternate printing)
const RAPHAEL_THE_NIGHTWATCHER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RAPHAEL_THE_NIGHTWATCHER,
    2,
    "83b46e45-2919-49b2-bb58-9c3e55c03490",
    "Dai-XT",
);

// TMT 288 — Michelangelo, Weirdness to 11 (alternate printing)
const MICHELANGELO_WEIRDNESS_TO_11_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MICHELANGELO_WEIRDNESS_TO_11,
    2,
    "61696380-e152-43e4-9374-c643d8d8c5f6",
    "O-G Osahune",
);

// TMT 289 — Dark Leo & Shredder (alternate printing)
const DARK_LEO_SHREDDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DARK_LEO_SHREDDER,
    2,
    "453f1766-c369-423a-8ce6-f6c25753125a",
    "Yukke Morita",
);

// TMT 290 — Krang, Utrom Warlord (alternate printing)
const KRANG_UTROM_WARLORD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KRANG_UTROM_WARLORD,
    2,
    "51409265-aa99-423e-be34-f62e122dbac3",
    "Taka-F",
);

// TMT 291 — Leonardo, Cutting Edge (alternate printing)
const LEONARDO_CUTTING_EDGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LEONARDO_CUTTING_EDGE,
    3,
    "6c2e0f39-2941-4136-a673-6d5317643105",
    "A4Mitsuori",
);

// TMT 292 — April O'Neil, Hacktivist (alternate printing)
const APRIL_O_NEIL_HACKTIVIST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &APRIL_O_NEIL_HACKTIVIST,
    3,
    "519359e5-72d8-4d3d-887e-cd7208aa3f95",
    "Ryo Kamei",
);

// TMT 293 — Donatello, Gadget Master (alternate printing)
const DONATELLO_GADGET_MASTER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DONATELLO_GADGET_MASTER,
    3,
    "86dea688-a84b-4fe2-9c3a-165061d2bf63",
    "Kotakan",
);

// TMT 294 — Turtles in Time (alternate printing)
const TURTLES_IN_TIME_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TURTLES_IN_TIME,
    3,
    "239107f5-0912-4a74-bd6a-51f87a481425",
    "Sansyu",
);

// TMT 295 — Super Shredder (alternate printing)
const SUPER_SHREDDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SUPER_SHREDDER,
    3,
    "ba38abbd-97b2-47e0-8cb6-7f4a02843e42",
    "Inuchiyo Meimaru",
);

// TMT 296 — Casey Jones, Vigilante (alternate printing)
const CASEY_JONES_VIGILANTE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CASEY_JONES_VIGILANTE,
    3,
    "9982e108-69d7-4a80-950d-4e30af140fe0",
    "Koji Nishino",
);

// TMT 297 — Raphael, the Nightwatcher (alternate printing)
const RAPHAEL_THE_NIGHTWATCHER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RAPHAEL_THE_NIGHTWATCHER,
    3,
    "711b5c8e-4c9a-449f-a495-ba960fc9752f",
    "Dai-XT",
);

// TMT 298 — Michelangelo, Weirdness to 11 (alternate printing)
const MICHELANGELO_WEIRDNESS_TO_11_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MICHELANGELO_WEIRDNESS_TO_11,
    3,
    "ffbff776-40d2-4d9b-9a4b-82ba4a456c0c",
    "O-G Osahune",
);

// TMT 299 — Dark Leo & Shredder (alternate printing)
const DARK_LEO_SHREDDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DARK_LEO_SHREDDER,
    3,
    "8cca5d97-c754-4d8a-8f59-b59a712a8fd0",
    "Yukke Morita",
);

// TMT 300 — Krang, Utrom Warlord (alternate printing)
const KRANG_UTROM_WARLORD_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KRANG_UTROM_WARLORD,
    3,
    "a1eb7fd4-2aec-45dd-8d0a-925581131e70",
    "Taka-F",
);

// TMT 301 — Leonardo, Sewer Samurai (alternate printing)
const LEONARDO_SEWER_SAMURAI_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LEONARDO_SEWER_SAMURAI,
    2,
    "74628e5a-7500-45d3-8147-30d2ef06da50",
    "Kevin Eastman",
);

// TMT 302 — Donatello, Mutant Mechanic (alternate printing)
const DONATELLO_MUTANT_MECHANIC_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DONATELLO_MUTANT_MECHANIC,
    2,
    "b818aa60-b312-457a-b167-b66f7c0cc5f4",
    "Kevin Eastman",
);

// TMT 303 — Raphael, Ninja Destroyer (alternate printing)
const RAPHAEL_NINJA_DESTROYER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RAPHAEL_NINJA_DESTROYER,
    2,
    "55f67220-3b95-49b9-9712-fedc6a67ff4b",
    "Kevin Eastman",
);

// TMT 304 — Michelangelo, Improviser (alternate printing)
const MICHELANGELO_IMPROVISER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MICHELANGELO_IMPROVISER,
    2,
    "dc54cba7-732c-4e09-9413-ad36603a6517",
    "Kevin Eastman",
);

// TMT 305 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "5c2812cb-4fce-4ec0-a2d5-88ef4e34d5ef",
    "BEMOCS",
);

// TMT 306 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "aedefd13-4a00-4b8c-b955-ced6e73b46fe",
    "BEMOCS",
);

// TMT 307 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "5f1ecc8a-4f83-46ce-80cc-acdff31d9ece",
    "BEMOCS",
);

// TMT 308 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "eb359844-c9e7-4ab2-ac5f-d426c4fe6fc7",
    "BEMOCS",
);

// TMT 309 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "815261f8-daaa-4d76-86d9-d2801eb3f1f7",
    "BEMOCS",
);

// TMT 310 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "b0e22017-301e-4c02-b496-c648ef97d919",
    "Gaboleps",
);

// TMT 311 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "affd8e02-5bae-447a-8621-d8fcfeff7135",
    "Gaboleps",
);

// TMT 312 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "fcdfd30f-91f0-493c-92ac-b2e64eb08d89",
    "Gaboleps",
);

// TMT 313 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "50e349f8-9860-4197-a40f-610659057365",
    "Gaboleps",
);

// TMT 314 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "1d91255d-e3d1-412b-8031-994779372496",
    "Gaboleps",
);

// TMT 315 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "2dfe1926-c0d5-40a2-b1aa-988524aefc31",
    "Sylvain Sarrailh",
);

// TMT 316 — Island (alternate printing)
const ISLAND_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    4,
    "836f371b-34f5-40e8-a806-e457841e5bc7",
    "Sylvain Sarrailh",
);

// TMT 317 — Swamp (alternate printing)
const SWAMP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    4,
    "e43ac31a-942e-4871-be29-426e19e52701",
    "Sylvain Sarrailh",
);

// TMT 318 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    4,
    "021fa322-f38c-4d94-8122-5b13425106d9",
    "Sylvain Sarrailh",
);

// TMT 319 — Forest (alternate printing)
const FOREST_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    4,
    "3b84ec1e-ccce-4b8b-9302-b26f84cfa469",
    "Sylvain Sarrailh",
);

// TMT 320 — Shark Shredder, Killer Clone (alternate printing)
const SHARK_SHREDDER_KILLER_CLONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SHARK_SHREDDER_KILLER_CLONE,
    2,
    "f2bc8cc8-715b-42a3-83fe-25972b5cbfe1",
    "Adam Volker",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ACTION_NEWS_CREW,
    &AGENT_BISHOP_MAN_IN_BLACK,
    &APRIL_O_NEIL_KUNOICHI_TRAINEE,
    &DIMENSIONAL_EXILE,
    &EAST_WIND_AVATAR,
    &FEATHERBRAINED_FILCHER,
    &GROUNDED_FOR_LIFE,
    &HAMATO_GUARDIAN_STANCE,
    &HIGH_FLYING_ACE,
    &JENNIKA_BAD_APPLE_BIG_SISTER,
    &KOYA_DEATH_FROM_ABOVE,
    &THE_LAST_RONIN_S_TECHNIQUE,
    &LEADER_S_TALENT,
    &LEONARDO_BIG_BROTHER,
    &LEONARDO_CUTTING_EDGE,
    &LEONARDO_LEADER_IN_BLUE,
    &LEONARDO_SEWER_SAMURAI,
    &LEONARDO_S_TECHNIQUE,
    &LITA_LITTLE_ORPHAN_AMPHIBIAN,
    &MIGHTY_MUTANIMALS,
    &PREHISTORIC_PET,
    &QUINTESSENTIAL_KATANA,
    &SALLY_PRIDE_LIONESS_LEADER,
    &TRICERATON_COMMANDER,
    &TURNCOAT_KUNOICHI,
    &TURTLES_FOREVER,
    &UNEASY_ALLIANCE,
    &APRIL_O_NEIL_HACKTIVIST,
    &APRIL_REPORTER_OF_THE_WEIRD,
    &BESPOKE_BO,
    &BUZZ_BOTS,
    &CRUSTACEAN_COMMANDO,
    &DOES_MACHINES,
    &DONATELLO_GADGET_MASTER,
    &DONATELLO_MUTANT_MECHANIC,
    &DONATELLO_TURTLE_TECHIE,
    &DONATELLO_WAY_WITH_MACHINES,
    &DONATELLO_S_TECHNIQUE,
    &FUGITIVE_DROID,
    &KITSUNE_DRAGON_S_DAUGHTER,
    &KITSUNE_S_TECHNIQUE,
    &KRANG_MASTER_MIND,
    &METALHEAD,
    &MIND_TRANSFER_PROTOCOL,
    &MONDO_GECKO,
    &OOZE_SPILL,
    &RAY_FILLET_MAN_RAY,
    &RENET_TEMPORAL_APPRENTICE,
    &RETRO_MUTATION,
    &RETURN_TO_THE_SEWERS,
    &SEWER_VEILLANCE_CAM,
    &STOCKMAN_MAD_FLY_ENTIST,
    &TURTLES_IN_TIME,
    &UTROM_SCIENTISTS,
    &ANCHOVY_BANANA_PIZZA,
    &ARMAGGON_FUTURE_SHARK,
    &BEBOP_WARTHOG_WARRIOR,
    &THE_CLONING_OF_SHREDDER,
    &DEATH_IN_THE_FAMILY,
    &DREAM_BEAVERS,
    &FOOT_MYSTIC,
    &INSECTOID_EXTERMINATOR,
    &LORD_DREGG_INSECT_INVADER,
    &MADAME_NULL_POWER_BROKER,
    &NINJA_TEEN,
    &OROKU_SAKI_SHREDDER_RISING,
    &PAIN_101,
    &PARAMECIA_COLONIEX,
    &RAT_KING_VERMINISTER,
    &SAVANTI_ROMERO_TIME_S_EXILE,
    &SHARK_SHREDDER_KILLER_CLONE,
    &SHREDDER_UNRELENTING,
    &SHREDDER_S_ARMOR,
    &SHREDDER_S_REVENGE,
    &SHREDDER_S_TECHNIQUE,
    &SOUTH_WIND_AVATAR,
    &SPLINTER_HAMATO_YOSHI,
    &SPLINTER_S_TECHNIQUE,
    &SQUIRRELANOIDS,
    &STOMPED_BY_THE_FOOT,
    &SUPER_SHREDDER,
    &TUNNEL_RATS,
    &BOT_BASHING_TIME,
    &BROADCAST_TAKEOVER,
    &CASEY_JONES_JURY_RIG_JUSTICIAR,
    &CASEY_JONES_VIGILANTE,
    &COOL_BUT_RUDE,
    &GENERAL_TRAAG_HEART_OF_STONE,
    &HARD_WON_JITTE,
    &IMPROVISED_ARSENAL,
    &JENNIKA_S_TECHNIQUE,
    &MANHOLE_MISSILE,
    &MOUSER_ATTACK,
    &MOUSER_FOUNDRY,
    &MUTANT_TOWN_MUSICIANS,
    &NULL_GROUP_BIOLOGICAL_ASSETS,
    &OLD_HOB_ALLEYCAT_BLUES,
    &PURPLE_DRAGON_PUNKS,
    &RAPHAEL_MOST_ATTITUDE,
    &RAPHAEL_NINJA_DESTROYER,
    &RAPHAEL_THE_NIGHTWATCHER,
    &RAPHAEL_TOUGH_TURTLE,
    &RAPHAEL_S_TECHNIQUE,
    &RAVENOUS_ROBOTS,
    &ROCK_SOLDIERS,
    &SLASH_REPTILE_RAMPAGER,
    &SPICY_OATMEAL_PIZZA,
    &WINGNUT_BAT_ON_THE_BELFRY,
    &ZOG_TRICERATON_CASTAWAY,
    &COURIER_OF_COMESTIBLES,
    &COWABUNGA,
    &FROG_BUTLER,
    &GROUNDCHUCK_DIRTBAG,
    &GUAC_MARSHMALLOW_PIZZA,
    &LEATHERHEAD_SWAMP_STALKER,
    &MICHELANGELO_GAME_MASTER,
    &MICHELANGELO_IMPROVISER,
    &MICHELANGELO_MUTANT_BFF,
    &MICHELANGELO_WEIRDNESS_TO_11,
    &MICHELANGELO_S_TECHNIQUE,
    &MONA_LISA_SCIENCE_GEEK,
    &MUTAGEN_MAN_LIVING_OOZE,
    &MUTANT_CHAIN_REACTION,
    &NEW_GENERATION_S_TECHNIQUE,
    &NOVEL_NUNCHAKU,
    &PARTY_DUDE,
    &PRIMORDIAL_PACHYDERM,
    &RAGAMUFFIN_RAPTOR,
    &ROCKSTEADY_CRASH_COURSER,
    &SAVED_BY_THE_SHELL,
    &TENDERIZE,
    &TRANSDIMENSIONAL_BOVINE,
    &TURTLE_POWER,
    &VENUS_TORN_BETWEEN_WORLDS,
    &WEST_WIND_AVATAR,
    &ZOO_ESCAPEES,
    &BAXTER_STOCKMAN,
    &BEBOP_ROCKSTEADY,
    &BRILLIANCE_UNLEASHED,
    &DARK_LEO_SHREDDER,
    &DON_LEO_PROBLEM_SOLVERS,
    &DON_RAPH_HARD_SCIENCE,
    &EPF_POINT_SQUAD,
    &FOOT_ELITE,
    &FOOT_NINJAS,
    &GENGHIS_FROG,
    &GO_NINJA_GO,
    &ICE_CREAM_KITTY,
    &KARAI_FUTURE_OF_THE_FOOT,
    &KARAI_S_TECHNIQUE,
    &KRANG_SHREDDER,
    &THE_LAST_RONIN,
    &LESSONS_FROM_LIFE,
    &MECHANIZED_NINJA_CAVALRY,
    &MIKEY_DON_PARTY_PLANNERS,
    &MIKEY_LEO_CHAOS_ORDER,
    &MOUSER_MARK_III,
    &THE_NEUTRINOS,
    &NOBODY,
    &NORTH_WIND_AVATAR,
    &PIZZA_FACE_GASTROMANCER,
    &PUNK_FROGS,
    &PUTRID_PALS,
    &RAPH_LEO_SIBLING_RIVALS,
    &RAPH_MIKEY_TROUBLEMAKERS,
    &SLITHERING_CRYPTID,
    &SPLINTER_RADICAL_RAT,
    &TAINTED_TREATS,
    &TOKKA_RAHZAR_TERRIBLE_TWOS,
    &CHROME_DOME,
    &EVERYTHING_PIZZA,
    &HENCHBOTS,
    &KRANG_UTROM_WARLORD,
    &OMNI_CHEESE_PIZZA,
    &THE_OOZE,
    &SKATEBOARD,
    &TECHNODROME,
    &TURTLE_BLIMP,
    &TURTLE_VAN,
    &WEATHER_MAKER,
    &DIMENSION_X,
    &FOOT_HEADQUARTERS,
    &ILLEGITIMATE_BUSINESS,
    &MUTANT_TOWN,
    &NORTHAMPTON_FARM,
    &TCRI_BUILDING,
    &TURTLE_LAIR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    MAKE_YOUR_MOVE_REPRINT,
    NEGATE_REPRINT,
    ESCAPE_TUNNEL_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    LEONARDO_LEADER_IN_BLUE_ALTERNATE_1,
    DONATELLO_WAY_WITH_MACHINES_ALTERNATE_1,
    MICHELANGELO_MUTANT_BFF_ALTERNATE_1,
    RAPHAEL_MOST_ATTITUDE_ALTERNATE_1,
    MIKEY_LEO_CHAOS_ORDER_ALTERNATE_1,
    KRANG_SHREDDER_ALTERNATE_1,
    RENET_TEMPORAL_APPRENTICE_ALTERNATE_1,
    JENNIKA_BAD_APPLE_BIG_SISTER_ALTERNATE_1,
    BEBOP_ROCKSTEADY_ALTERNATE_1,
    DON_RAPH_HARD_SCIENCE_ALTERNATE_1,
    APRIL_REPORTER_OF_THE_WEIRD_ALTERNATE_1,
    CASEY_JONES_JURY_RIG_JUSTICIAR_ALTERNATE_1,
    SLASH_REPTILE_RAMPAGER_ALTERNATE_1,
    FOOT_NINJAS_ALTERNATE_1,
    SHREDDER_UNRELENTING_ALTERNATE_1,
    LEONARDO_CUTTING_EDGE_ALTERNATE_1,
    DONATELLO_GADGET_MASTER_ALTERNATE_1,
    RAPHAEL_THE_NIGHTWATCHER_ALTERNATE_1,
    MICHELANGELO_WEIRDNESS_TO_11_ALTERNATE_1,
    LEONARDO_SEWER_SAMURAI_ALTERNATE_1,
    DONATELLO_MUTANT_MECHANIC_ALTERNATE_1,
    SUPER_SHREDDER_ALTERNATE_1,
    RAPHAEL_NINJA_DESTROYER_ALTERNATE_1,
    MICHELANGELO_IMPROVISER_ALTERNATE_1,
    DARK_LEO_SHREDDER_ALTERNATE_1,
    KRANG_UTROM_WARLORD_ALTERNATE_1,
    TECHNODROME_ALTERNATE_1,
    THE_LAST_RONIN_S_TECHNIQUE_ALTERNATE_1,
    LEONARDO_S_TECHNIQUE_ALTERNATE_1,
    SALLY_PRIDE_LIONESS_LEADER_ALTERNATE_1,
    TRICERATON_COMMANDER_ALTERNATE_1,
    APRIL_O_NEIL_HACKTIVIST_ALTERNATE_1,
    DONATELLO_S_TECHNIQUE_ALTERNATE_1,
    KITSUNE_S_TECHNIQUE_ALTERNATE_1,
    KRANG_MASTER_MIND_ALTERNATE_1,
    MONDO_GECKO_ALTERNATE_1,
    SHREDDER_S_TECHNIQUE_ALTERNATE_1,
    SPLINTER_S_TECHNIQUE_ALTERNATE_1,
    BROADCAST_TAKEOVER_ALTERNATE_1,
    CASEY_JONES_VIGILANTE_ALTERNATE_1,
    JENNIKA_S_TECHNIQUE_ALTERNATE_1,
    RAPHAEL_S_TECHNIQUE_ALTERNATE_1,
    GROUNDCHUCK_DIRTBAG_ALTERNATE_1,
    MICHELANGELO_S_TECHNIQUE_ALTERNATE_1,
    NEW_GENERATION_S_TECHNIQUE_ALTERNATE_1,
    BEBOP_ROCKSTEADY_ALTERNATE_2,
    DON_LEO_PROBLEM_SOLVERS_ALTERNATE_1,
    DON_RAPH_HARD_SCIENCE_ALTERNATE_2,
    KARAI_S_TECHNIQUE_ALTERNATE_1,
    KRANG_SHREDDER_ALTERNATE_2,
    MIKEY_DON_PARTY_PLANNERS_ALTERNATE_1,
    MIKEY_LEO_CHAOS_ORDER_ALTERNATE_2,
    NORTH_WIND_AVATAR_ALTERNATE_1,
    RAPH_LEO_SIBLING_RIVALS_ALTERNATE_1,
    RAPH_MIKEY_TROUBLEMAKERS_ALTERNATE_1,
    SPLINTER_RADICAL_RAT_ALTERNATE_1,
    TOKKA_RAHZAR_TERRIBLE_TWOS_ALTERNATE_1,
    PLAINS_ALTERNATE_1,
    ISLAND_ALTERNATE_1,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_1,
    FOREST_ALTERNATE_1,
    AGENT_BISHOP_MAN_IN_BLACK_ALTERNATE_1,
    PREHISTORIC_PET_ALTERNATE_1,
    TURNCOAT_KUNOICHI_ALTERNATE_1,
    TURTLES_FOREVER_ALTERNATE_1,
    KITSUNE_DRAGON_S_DAUGHTER_ALTERNATE_1,
    TURTLES_IN_TIME_ALTERNATE_1,
    ARMAGGON_FUTURE_SHARK_ALTERNATE_1,
    MADAME_NULL_POWER_BROKER_ALTERNATE_1,
    RAT_KING_VERMINISTER_ALTERNATE_1,
    SAVANTI_ROMERO_TIME_S_EXILE_ALTERNATE_1,
    SHARK_SHREDDER_KILLER_CLONE_ALTERNATE_1,
    SOUTH_WIND_AVATAR_ALTERNATE_1,
    IMPROVISED_ARSENAL_ALTERNATE_1,
    RAVENOUS_ROBOTS_ALTERNATE_1,
    LEATHERHEAD_SWAMP_STALKER_ALTERNATE_1,
    MUTAGEN_MAN_LIVING_OOZE_ALTERNATE_1,
    TRANSDIMENSIONAL_BOVINE_ALTERNATE_1,
    TURTLE_POWER_ALTERNATE_1,
    CHROME_DOME_ALTERNATE_1,
    THE_OOZE_ALTERNATE_1,
    TURTLE_VAN_ALTERNATE_1,
    WEATHER_MAKER_ALTERNATE_1,
    NORTHAMPTON_FARM_ALTERNATE_1,
    LEONARDO_CUTTING_EDGE_ALTERNATE_2,
    APRIL_O_NEIL_HACKTIVIST_ALTERNATE_2,
    DONATELLO_GADGET_MASTER_ALTERNATE_2,
    TURTLES_IN_TIME_ALTERNATE_2,
    SUPER_SHREDDER_ALTERNATE_2,
    CASEY_JONES_VIGILANTE_ALTERNATE_2,
    RAPHAEL_THE_NIGHTWATCHER_ALTERNATE_2,
    MICHELANGELO_WEIRDNESS_TO_11_ALTERNATE_2,
    DARK_LEO_SHREDDER_ALTERNATE_2,
    KRANG_UTROM_WARLORD_ALTERNATE_2,
    LEONARDO_CUTTING_EDGE_ALTERNATE_3,
    APRIL_O_NEIL_HACKTIVIST_ALTERNATE_3,
    DONATELLO_GADGET_MASTER_ALTERNATE_3,
    TURTLES_IN_TIME_ALTERNATE_3,
    SUPER_SHREDDER_ALTERNATE_3,
    CASEY_JONES_VIGILANTE_ALTERNATE_3,
    RAPHAEL_THE_NIGHTWATCHER_ALTERNATE_3,
    MICHELANGELO_WEIRDNESS_TO_11_ALTERNATE_3,
    DARK_LEO_SHREDDER_ALTERNATE_3,
    KRANG_UTROM_WARLORD_ALTERNATE_3,
    LEONARDO_SEWER_SAMURAI_ALTERNATE_2,
    DONATELLO_MUTANT_MECHANIC_ALTERNATE_2,
    RAPHAEL_NINJA_DESTROYER_ALTERNATE_2,
    MICHELANGELO_IMPROVISER_ALTERNATE_2,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_3,
    PLAINS_ALTERNATE_4,
    ISLAND_ALTERNATE_4,
    SWAMP_ALTERNATE_4,
    MOUNTAIN_ALTERNATE_4,
    FOREST_ALTERNATE_4,
    SHARK_SHREDDER_KILLER_CLONE_ALTERNATE_2,
];
