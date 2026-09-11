//! The Hobbit card inventory.

use super::{CardRecord, PrintingRecord};

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1997::portal as catalog_por;
use crate::card::sets::y2017::ixalan as catalog_xln;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "HOB",
    slug: "the-hobbit",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// HOB 1 — Long-Bodied Grey Dog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONG_BODIED_GREY_DOG: CardRecord = CardRecord::new(
    "Long-Bodied Grey Dog",
    "d1a1e520-1fe2-4529-8afb-c187bb80da3c",
    "Anna Podedworna",
    crate::card::CardRules::unsupported(),
);

// HOB 2 — Old Thrush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OLD_THRUSH: CardRecord = CardRecord::new(
    "Old Thrush",
    "3ad02b56-13ec-46ef-92bd-ae078b8bb517",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// HOB 3 — Troop of Ponies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROOP_OF_PONIES: CardRecord = CardRecord::new(
    "Troop of Ponies",
    "0b4b1c59-bcec-4779-9e27-0e6f9feb4e11",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// HOB 4 — Belladonna Took
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELLADONNA_TOOK: CardRecord = CardRecord::new(
    "Belladonna Took",
    "88f0c189-c9ed-4ea3-ae62-3d8ac6c7fecf",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// HOB 5 — Bilbo's Gambit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BILBO_S_GAMBIT: CardRecord = CardRecord::new(
    "Bilbo's Gambit",
    "45ad01f0-cda8-4931-82bb-cb4949e56ae9",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// HOB 6 — Bofur, Reliable Guardian // Concerted Care
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOFUR_RELIABLE_GUARDIAN: CardRecord = CardRecord::new(
    "Bofur, Reliable Guardian // Concerted Care",
    "6b8e6435-7de4-41d5-bc7d-8e24c11897d0",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// HOB 7 — Celebrate the Mountain-king
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELEBRATE_THE_MOUNTAIN_KING: CardRecord = CardRecord::new(
    "Celebrate the Mountain-king",
    "42fbd61d-e1a6-465d-b1a3-f5ee0869d3af",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 8 — Dáin, Lord of the Iron Hills
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAIN_LORD_OF_THE_IRON_HILLS: CardRecord = CardRecord::new(
    "Dáin, Lord of the Iron Hills",
    "99d27749-d16c-45e9-accc-6a01351c17f9",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 9 — Dwarven Provisioner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_PROVISIONER: CardRecord = CardRecord::new(
    "Dwarven Provisioner",
    "1f9a61a1-454e-4d5b-a6dd-1a79fe9dedf3",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// HOB 10 — Dwarven Shortsword
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_SHORTSWORD: CardRecord = CardRecord::new(
    "Dwarven Shortsword",
    "f2341cf3-4d2c-4a4f-9aea-8834104a8910",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// HOB 11 — Eagle of the Great Shelf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EAGLE_OF_THE_GREAT_SHELF: CardRecord = CardRecord::new(
    "Eagle of the Great Shelf",
    "3feca644-5f65-4477-bbc8-d505cec6f3a5",
    "Yuhong Ding",
    crate::card::CardRules::unsupported(),
);

// HOB 12 — The Eagles Are Coming!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_EAGLES_ARE_COMING: CardRecord = CardRecord::new(
    "The Eagles Are Coming!",
    "0bda1b62-47fc-42c2-a841-ccad8ea0db48",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// HOB 13 — Esgaroth Garrison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESGAROTH_GARRISON: CardRecord = CardRecord::new(
    "Esgaroth Garrison",
    "573f67b0-6ce8-4857-a703-4a5728640736",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// HOB 14 — Fíli the Pathfinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FILI_THE_PATHFINDER: CardRecord = CardRecord::new(
    "Fíli the Pathfinder",
    "b02142f3-5e55-40dc-a02c-9113fb7d763c",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// HOB 15 — Gleaming Splendor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLEAMING_SPLENDOR: CardRecord = CardRecord::new(
    "Gleaming Splendor",
    "3b087bd4-bbb7-4963-bdb6-0a700ff19a04",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// HOB 16 — Iron Hills Blacksmith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_HILLS_BLACKSMITH: CardRecord = CardRecord::new(
    "Iron Hills Blacksmith",
    "370e09c2-36c5-4662-8350-1db798afad3e",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// HOB 17 — Kíli the Resourceful
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KILI_THE_RESOURCEFUL: CardRecord = CardRecord::new(
    "Kíli the Resourceful",
    "1805532f-6d99-47d0-9529-5f5831a7fdc8",
    "Yuhong Ding",
    crate::card::CardRules::unsupported(),
);

// HOB 18 — Lake-town Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAKE_TOWN_LOOKOUT: CardRecord = CardRecord::new(
    "Lake-town Lookout",
    "178c4cf6-6b11-40e4-9673-c560d6818a6b",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// HOB 19 — Lake-town Toymaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAKE_TOWN_TOYMAKER: CardRecord = CardRecord::new(
    "Lake-town Toymaker",
    "67304269-c595-4cf0-8dbf-fcb2e9e01fe2",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// HOB 20 — Magnificent End
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNIFICENT_END: CardRecord = CardRecord::new(
    "Magnificent End",
    "430c8916-1167-400b-9cad-d301f59d5e5d",
    "Yuhong Ding",
    crate::card::CardRules::unsupported(),
);

// HOB 21 — Moment of Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENT_OF_GLORY: CardRecord = CardRecord::new(
    "Moment of Glory",
    "0a6a6ff0-b1cd-4b06-bd31-612690094e0e",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// HOB 22 — The Mountain-king's Return
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_MOUNTAIN_KING_S_RETURN: CardRecord = CardRecord::new(
    "The Mountain-king's Return",
    "68f4893d-e9a5-4f89-ade3-9ab78a834ad5",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 23 — Ori, Keeper of Songs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORI_KEEPER_OF_SONGS: CardRecord = CardRecord::new(
    "Ori, Keeper of Songs",
    "c5727af5-a487-4b16-8278-81c3c928c417",
    "Yigit Koroglu",
    crate::card::CardRules::unsupported(),
);

// HOB 24 — The Queen of Dale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_QUEEN_OF_DALE: CardRecord = CardRecord::new(
    "The Queen of Dale",
    "c977fb5f-4436-41d0-af68-93b6d05897e5",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// HOB 25 — Roads Go Ever, Ever On
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROADS_GO_EVER_EVER_ON: CardRecord = CardRecord::new(
    "Roads Go Ever, Ever On",
    "b3c1ebd6-967f-4b8c-8f1f-442ce8c1da24",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 26 — Settle the Wreckage (reprint)
const SETTLE_THE_WRECKAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::SETTLE_THE_WRECKAGE,
    "31a7a5e2-4cb8-48fc-8351-18344b4a7560",
    "Chris Cold",
);

// HOB 27 — Stone by Sunlight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_BY_SUNLIGHT: CardRecord = CardRecord::new(
    "Stone by Sunlight",
    "c5752731-253c-4b41-bdd8-94c26d715206",
    "Kamila Szutenberg",
    crate::card::CardRules::unsupported(),
);

// HOB 28 — Thorin's Last Stand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORIN_S_LAST_STAND: CardRecord = CardRecord::new(
    "Thorin's Last Stand",
    "127367b6-9cfe-4516-9bfd-5b951468a25c",
    "John Di Giovanni",
    crate::card::CardRules::unsupported(),
);

// HOB 29 — An Unexpected Party // At the Door
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AN_UNEXPECTED_PARTY: CardRecord = CardRecord::new(
    "An Unexpected Party // At the Door",
    "3aa29fe8-1687-486f-b4df-c04977869ab1",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// HOB 30 — Velvetwing Butterflies // Gaze in Wonder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VELVETWING_BUTTERFLIES: CardRecord = CardRecord::new(
    "Velvetwing Butterflies // Gaze in Wonder",
    "5cc0f994-5048-4898-926e-b56cbc97e0ca",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// HOB 31 — Vow to Erebor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOW_TO_EREBOR: CardRecord = CardRecord::new(
    "Vow to Erebor",
    "8d4f3eb5-fedf-45d6-8bd8-aacbe0ce33b2",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// HOB 32 — Bilbo, Luckwearer // Burglar's Plot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BILBO_LUCKWEARER: CardRecord = CardRecord::new(
    "Bilbo, Luckwearer // Burglar's Plot",
    "8bff0aa6-16d9-4c83-b598-ef00a3b33d2c",
    "Anna Steinbauer",
    crate::card::CardRules::unsupported(),
);

// HOB 33 — Bilbo, Thief in the Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BILBO_THIEF_IN_THE_NIGHT: CardRecord = CardRecord::new(
    "Bilbo, Thief in the Night",
    "484c7f83-8339-4ae1-8350-68ce1f7d05a3",
    "Nia Kovalevski",
    crate::card::CardRules::unsupported(),
);

// HOB 34 — Bilbo Baggins, Burglar // Take a Glance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BILBO_BAGGINS_BURGLAR: CardRecord = CardRecord::new(
    "Bilbo Baggins, Burglar // Take a Glance",
    "6a109b3e-9f5b-4625-abb7-6b992c10530b",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// HOB 35 — Confusticate and Bebother
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONFUSTICATE_AND_BEBOTHER: CardRecord = CardRecord::new(
    "Confusticate and Bebother",
    "9de48690-e5ae-495a-addf-305f1db7ec21",
    "Colin Boyer",
    crate::card::CardRules::unsupported(),
);

// HOB 36 — Elrond, Moon-Reader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELROND_MOON_READER: CardRecord = CardRecord::new(
    "Elrond, Moon-Reader",
    "fbcb310c-be73-46f8-8e65-8632454ccc6e",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// HOB 37 — Elven Raft-Steerer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVEN_RAFT_STEERER: CardRecord = CardRecord::new(
    "Elven Raft-Steerer",
    "c141695c-c108-41d5-85cb-1f7485d9d533",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// HOB 38 — Elvenking's Harper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVENKING_S_HARPER: CardRecord = CardRecord::new(
    "Elvenking's Harper",
    "9c50656d-c74a-4e90-9ef7-afa237682516",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// HOB 39 — Enchanted River's Grasp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENCHANTED_RIVER_S_GRASP: CardRecord = CardRecord::new(
    "Enchanted River's Grasp",
    "ad40a4b9-9fab-49c1-8e9f-6e0776966833",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// HOB 40 — Fateful Discovery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FATEFUL_DISCOVERY: CardRecord = CardRecord::new(
    "Fateful Discovery",
    "a1142fa1-b861-4876-aa48-402af35aaa63",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// HOB 41 — Gandalf, Wandering Wizard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANDALF_WANDERING_WIZARD: CardRecord = CardRecord::new(
    "Gandalf, Wandering Wizard",
    "1f8403a2-849c-4a59-b0ed-c8803995028d",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// HOB 42 — Great Gilded Boat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREAT_GILDED_BOAT: CardRecord = CardRecord::new(
    "Great Gilded Boat",
    "b2fb3995-5b43-4776-88b2-346d353edee0",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// HOB 43 — Lakeshore Apothecary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAKESHORE_APOTHECARY: CardRecord = CardRecord::new(
    "Lakeshore Apothecary",
    "abfbb255-a39b-4df5-bfb6-5298584e89f0",
    "Wei Guan",
    crate::card::CardRules::unsupported(),
);

// HOB 44 — Lake-town Mariners // Gone Fishing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAKE_TOWN_MARINERS: CardRecord = CardRecord::new(
    "Lake-town Mariners // Gone Fishing",
    "4202a678-a5f4-47f9-9c18-e88ab9ad20a4",
    "Wei Guan",
    crate::card::CardRules::unsupported(),
);

// HOB 45 — Long Lake Nuisance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONG_LAKE_NUISANCE: CardRecord = CardRecord::new(
    "Long Lake Nuisance",
    "cd5af94d-6321-4834-8e5f-e5d0261b3ef3",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// HOB 46 — The Lord of the Eagles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_LORD_OF_THE_EAGLES: CardRecord = CardRecord::new(
    "The Lord of the Eagles",
    "fa0554fc-9448-4ae2-8712-4f4f7af3c7b4",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// HOB 47 — Master's Councillors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_S_COUNCILLORS: CardRecord = CardRecord::new(
    "Master's Councillors",
    "addcefdd-e012-4adf-9052-e60376a8d2d3",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// HOB 48 — Mirkwood Meditator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRKWOOD_MEDITATOR: CardRecord = CardRecord::new(
    "Mirkwood Meditator",
    "ad7ed4e6-3fe2-40f1-909b-a03b2a3c941a",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// HOB 49 — Most Decrepit Old Bird // Speak Secrets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOST_DECREPIT_OLD_BIRD: CardRecord = CardRecord::new(
    "Most Decrepit Old Bird // Speak Secrets",
    "2d838feb-89f2-4cdb-a5ab-ec880f28d873",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// HOB 50 — Old Fat Spider Can't See Me
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OLD_FAT_SPIDER_CAN_T_SEE_ME: CardRecord = CardRecord::new(
    "Old Fat Spider Can't See Me",
    "4a865cea-f947-4736-8ace-ba478fceeb22",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 51 — Plunder the Trollshaws
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLUNDER_THE_TROLLSHAWS: CardRecord = CardRecord::new(
    "Plunder the Trollshaws",
    "afb73190-b9bd-4744-a011-a37cd9c0148d",
    "Alexander Mokhov",
    crate::card::CardRules::unsupported(),
);

// HOB 52 — Ravenhill Flock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVENHILL_FLOCK: CardRecord = CardRecord::new(
    "Ravenhill Flock",
    "acbb4d32-2771-469e-a6de-0df15155cc62",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// HOB 53 — Riddles in the Dark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIDDLES_IN_THE_DARK: CardRecord = CardRecord::new(
    "Riddles in the Dark",
    "a6129286-7437-4ba4-be55-586a22cd67ca",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// HOB 54 — Roll-Roll-Roll-Roll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROLL_ROLL_ROLL_ROLL: CardRecord = CardRecord::new(
    "Roll-Roll-Roll-Roll",
    "a2e4099e-86bd-461f-87fa-7f7850ae7eec",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 55 — Sound the Trumpets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUND_THE_TRUMPETS: CardRecord = CardRecord::new(
    "Sound the Trumpets",
    "dd32a1dd-3541-4572-a717-1deabc14b827",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// HOB 56 — Thranduil's Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRANDUIL_S_DECREE: CardRecord = CardRecord::new(
    "Thranduil's Decree",
    "e4ded4c1-0e3e-47c5-8fdc-e7c187f68b12",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// HOB 57 — Uncover the Moon-Letters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNCOVER_THE_MOON_LETTERS: CardRecord = CardRecord::new(
    "Uncover the Moon-Letters",
    "79edf5f6-f6b6-4271-bd2a-14a980f30616",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// HOB 58 — Uneasy Partings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNEASY_PARTINGS: CardRecord = CardRecord::new(
    "Uneasy Partings",
    "e49866d4-966a-40f9-b08d-18e5af6d726b",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// HOB 59 — Wizard's Staff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIZARD_S_STAFF: CardRecord = CardRecord::new(
    "Wizard's Staff",
    "0de529a7-bdc5-4581-a169-1ad123bc099a",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// HOB 60 — Along the Crooked Way
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALONG_THE_CROOKED_WAY: CardRecord = CardRecord::new(
    "Along the Crooked Way",
    "3696d65c-fffd-4685-bb2d-e8769bf476e3",
    "Bruce Brenneise",
    crate::card::CardRules::unsupported(),
);

// HOB 61 — Azog, Moria's Ruin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AZOG_MORIA_S_RUIN: CardRecord = CardRecord::new(
    "Azog, Moria's Ruin",
    "135da718-affc-46ba-be57-c12c23b54dad",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 62 — Bilbo's Deadly Slice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BILBO_S_DEADLY_SLICE: CardRecord = CardRecord::new(
    "Bilbo's Deadly Slice",
    "17892c93-b9b2-4720-933b-998ed0200492",
    "Henry Peters",
    crate::card::CardRules::unsupported(),
);

// HOB 63 — Crude Bent Blade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUDE_BENT_BLADE: CardRecord = CardRecord::new(
    "Crude Bent Blade",
    "fa8fd3c4-bd00-485d-80b1-2b67f5786fce",
    "Russell Dongjun Lu",
    crate::card::CardRules::unsupported(),
);

// HOB 64 — Desolation Prowler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESOLATION_PROWLER: CardRecord = CardRecord::new(
    "Desolation Prowler",
    "63c87009-ff1b-44b9-88b1-e26219094c67",
    "Harkalé Linaï",
    crate::card::CardRules::unsupported(),
);

// HOB 65 — Down, Down to Goblin-town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOWN_DOWN_TO_GOBLIN_TOWN: CardRecord = CardRecord::new(
    "Down, Down to Goblin-town",
    "b72e193c-e030-4936-9b79-c636eff750e1",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 66 — Dreaded Bat-Cloud
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREADED_BAT_CLOUD: CardRecord = CardRecord::new(
    "Dreaded Bat-Cloud",
    "67d52db5-597e-46d5-af39-c3a2de107d30",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// HOB 67 — Front Porch Sentries
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRONT_PORCH_SENTRIES: CardRecord = CardRecord::new(
    "Front Porch Sentries",
    "07bfc803-e11b-47ab-9f25-0ace7e174200",
    "Stanislav Sherbakov",
    crate::card::CardRules::unsupported(),
);

// HOB 68 — Gathering of Darkness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GATHERING_OF_DARKNESS: CardRecord = CardRecord::new(
    "Gathering of Darkness",
    "2ce066be-e5ad-4b93-8245-1b5018990d03",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// HOB 69 — Gnashing of Teeth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GNASHING_OF_TEETH: CardRecord = CardRecord::new(
    "Gnashing of Teeth",
    "5d485d70-c7b9-40a4-9089-5e7f1c2b9213",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// HOB 70 — Gollum, Riddle Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLLUM_RIDDLE_MASTER: CardRecord = CardRecord::new(
    "Gollum, Riddle Master",
    "bbdc7e37-c65a-497a-92b7-a30a6e369c71",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// HOB 71 — Gollum, Silent Slinker // Meager Meal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLLUM_SILENT_SLINKER: CardRecord = CardRecord::new(
    "Gollum, Silent Slinker // Meager Meal",
    "6cfaa182-3fec-4907-8814-b4d29c33cec3",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 72 — Gollum the Abandoned
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLLUM_THE_ABANDONED: CardRecord = CardRecord::new(
    "Gollum the Abandoned",
    "50d91ef3-6f5d-4255-8d47-be731b5dad30",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// HOB 73 — Great Fierce Bee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREAT_FIERCE_BEE: CardRecord = CardRecord::new(
    "Great Fierce Bee",
    "9d9ef88f-d208-4788-9553-cd672b3be1fe",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// HOB 74 — Great Ugly-Looking Goblin // Clap! Snap!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREAT_UGLY_LOOKING_GOBLIN: CardRecord = CardRecord::new(
    "Great Ugly-Looking Goblin // Clap! Snap!",
    "c87f6004-e1cf-42b2-9647-322bc4939339",
    "Jason Kang",
    crate::card::CardRules::unsupported(),
);

// HOB 75 — Head of the Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAD_OF_THE_HUNT: CardRecord = CardRecord::new(
    "Head of the Hunt",
    "3ffe34d4-72f4-4562-a948-8909b9321e59",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// HOB 76 — Inside Information
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIDE_INFORMATION: CardRecord = CardRecord::new(
    "Inside Information",
    "9763bd56-fa4b-4907-ad15-c3f040c5fc0a",
    "Sean Vo",
    crate::card::CardRules::unsupported(),
);

// HOB 77 — The Master of Lake-town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_MASTER_OF_LAKE_TOWN: CardRecord = CardRecord::new(
    "The Master of Lake-town",
    "3788ada6-34a9-41af-a31c-2d090550e503",
    "Marius Bota",
    crate::card::CardRules::unsupported(),
);

// HOB 78 — Nighthowl Pursuer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTHOWL_PURSUER: CardRecord = CardRecord::new(
    "Nighthowl Pursuer",
    "d3cbe830-7e95-4019-89c4-cfb36bcf00f8",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 79 — Rage into the Valley
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGE_INTO_THE_VALLEY: CardRecord = CardRecord::new(
    "Rage into the Valley",
    "8651958c-3b94-47a9-a751-faf8f6236a42",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// HOB 80 — Ravening Warg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVENING_WARG: CardRecord = CardRecord::new(
    "Ravening Warg",
    "ea7b5052-b343-466d-879e-2a211657ef0a",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// HOB 81 — Reverent Howl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERENT_HOWL: CardRecord = CardRecord::new(
    "Reverent Howl",
    "16765eb2-d497-4cd6-b683-20eac2f10bbf",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// HOB 82 — Rhovanion Rampager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHOVANION_RAMPAGER: CardRecord = CardRecord::new(
    "Rhovanion Rampager",
    "5ee45a5e-3650-47b3-8d31-6b1de9e27a14",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// HOB 83 — The Sackville-Bagginses
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SACKVILLE_BAGGINSES: CardRecord = CardRecord::new(
    "The Sackville-Bagginses",
    "ed87b471-79f9-45ec-9188-69e970f6121e",
    "Denman Rooke",
    crate::card::CardRules::unsupported(),
);

// HOB 84 — Stir Up Trouble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STIR_UP_TROUBLE: CardRecord = CardRecord::new(
    "Stir Up Trouble",
    "fd145e3a-c889-4390-accb-863dbcc845ce",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// HOB 85 — Stony-Voiced Goblins
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONY_VOICED_GOBLINS: CardRecord = CardRecord::new(
    "Stony-Voiced Goblins",
    "6fcc3699-b475-4612-884d-81bd4f21e9c1",
    "Marius Bota",
    crate::card::CardRules::unsupported(),
);

// HOB 86 — Supper for Spiders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPPER_FOR_SPIDERS: CardRecord = CardRecord::new(
    "Supper for Spiders",
    "5b25e454-06bb-43ca-9a9f-57164f7a70c4",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// HOB 87 — Balin, Loremaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALIN_LOREMASTER: CardRecord = CardRecord::new(
    "Balin, Loremaster",
    "42d7ca7b-c983-40fd-ad57-59f6972bb375",
    "Colin Boyer",
    crate::card::CardRules::unsupported(),
);

// HOB 88 — Bombur, Gentle Dreamer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOMBUR_GENTLE_DREAMER: CardRecord = CardRecord::new(
    "Bombur, Gentle Dreamer",
    "63c317e7-432c-4817-8db4-3670a1d84be3",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// HOB 89 — Bothersome Noisemaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOTHERSOME_NOISEMAKER: CardRecord = CardRecord::new(
    "Bothersome Noisemaker",
    "cb25b11a-6bf5-4a9a-b60f-d4dcac3816d6",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// HOB 90 — Burn, Burn, Tree and Fern
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURN_BURN_TREE_AND_FERN: CardRecord = CardRecord::new(
    "Burn, Burn, Tree and Fern",
    "fceb1a2d-121e-49ad-acf2-1bb5aebec116",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 91 — Dáin Ironfoot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAIN_IRONFOOT: CardRecord = CardRecord::new(
    "Dáin Ironfoot",
    "7112e460-9160-4535-ad94-93f1f4ac04cf",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 92 — Desert Were-Worm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESERT_WERE_WORM: CardRecord = CardRecord::new(
    "Desert Were-Worm",
    "fc12c22a-11ff-4fb0-bc42-dd8490b8efb7",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// HOB 93 — Desolation of Smaug
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESOLATION_OF_SMAUG: CardRecord = CardRecord::new(
    "Desolation of Smaug",
    "2462358b-52c8-49b2-8d97-d65a9188f8f7",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// HOB 94 — Dori, Bearer of Friends
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DORI_BEARER_OF_FRIENDS: CardRecord = CardRecord::new(
    "Dori, Bearer of Friends",
    "d2f60ad0-c887-4585-85f8-afcf72fb80d0",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// HOB 95 — Dwarven Mauler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_MAULER: CardRecord = CardRecord::new(
    "Dwarven Mauler",
    "bd0f0415-43af-4f5d-8999-853c5d42780d",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// HOB 96 — Gandalf, Goblins' Bane // Flameshape
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANDALF_GOBLINS_BANE: CardRecord = CardRecord::new(
    "Gandalf, Goblins' Bane // Flameshape",
    "9b0d29a1-7da9-4fb3-8536-8ff8d8acae0b",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// HOB 97 — Gandalf, Spark Starter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANDALF_SPARK_STARTER: CardRecord = CardRecord::new(
    "Gandalf, Spark Starter",
    "7c5c6f1c-35cf-4172-b5a1-b73222b0723b",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// HOB 98 — Getaway Barrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GETAWAY_BARREL: CardRecord = CardRecord::new(
    "Getaway Barrel",
    "e4819aa6-5d28-4a37-942d-89523e30c4e1",
    "Pablo Mendoza",
    crate::card::CardRules::unsupported(),
);

// HOB 99 — Glóin the Mighty // Easy Pickings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOIN_THE_MIGHTY: CardRecord = CardRecord::new(
    "Glóin the Mighty // Easy Pickings",
    "5793b8eb-2fc5-454d-8fa2-20346fef167a",
    "Colin Boyer",
    crate::card::CardRules::unsupported(),
);

// HOB 100 — Goblin-town Flunkies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_TOWN_FLUNKIES: CardRecord = CardRecord::new(
    "Goblin-town Flunkies",
    "ccff7382-8609-494c-aeee-cd1436456dd0",
    "Jason Kang",
    crate::card::CardRules::unsupported(),
);

// HOB 101 — Gundabad Opportunist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUNDABAD_OPPORTUNIST: CardRecord = CardRecord::new(
    "Gundabad Opportunist",
    "bc4a60b8-a5bb-4dbf-8d48-95caf757eac3",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// HOB 102 — Iron Hills Stalwart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_HILLS_STALWART: CardRecord = CardRecord::new(
    "Iron Hills Stalwart",
    "46daa9ac-0ac7-4df9-b9d2-e03ab5b56c72",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// HOB 103 — Last Light of Durin's Day
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_LIGHT_OF_DURIN_S_DAY: CardRecord = CardRecord::new(
    "Last Light of Durin's Day",
    "df29484b-de4b-4bab-995a-7605745780d9",
    "Harkalé Linaï",
    crate::card::CardRules::unsupported(),
);

// HOB 104 — The Misty Mountains Cold
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_MISTY_MOUNTAINS_COLD: CardRecord = CardRecord::new(
    "The Misty Mountains Cold",
    "3d5f35ff-4146-4844-9da5-031461cc8c05",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 105 — Misty Mountains Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTY_MOUNTAINS_RAIDER: CardRecord = CardRecord::new(
    "Misty Mountains Raider",
    "6dff14cd-b60b-48f4-9d9f-c9019b55df4c",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 106 — Óin the Brave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OIN_THE_BRAVE: CardRecord = CardRecord::new(
    "Óin the Brave",
    "9984b9ef-e81c-48f4-aa33-0504171a2d3c",
    "Colin Boyer",
    crate::card::CardRules::unsupported(),
);

// HOB 107 — Pinecone Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINECONE_STRIKE: CardRecord = CardRecord::new(
    "Pinecone Strike",
    "ea174cea-40e5-424e-9734-e39aae6c6b17",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// HOB 108 — Ragged Short Spear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGGED_SHORT_SPEAR: CardRecord = CardRecord::new(
    "Ragged Short Spear",
    "7bf81a8b-52ad-49f5-a3d4-22613cad3a3d",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 109 — Smaug, the Great Calamity // Spew Flame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMAUG_THE_GREAT_CALAMITY: CardRecord = CardRecord::new(
    "Smaug, the Great Calamity // Spew Flame",
    "419ca9e5-8413-4378-a4ef-eda5a1024218",
    "Chris Cold",
    crate::card::CardRules::unsupported(),
);

// HOB 110 — Smaug the Magnificent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMAUG_THE_MAGNIFICENT: CardRecord = CardRecord::new(
    "Smaug the Magnificent",
    "6a5d8fad-2ffd-4645-8c49-907999b6cecf",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// HOB 111 — Smaug's Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMAUG_S_FURY: CardRecord = CardRecord::new(
    "Smaug's Fury",
    "a16f203a-785e-4c78-9410-fb9f8a0ffa01",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// HOB 112 — Snowslope Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWSLOPE_HUNTER: CardRecord = CardRecord::new(
    "Snowslope Hunter",
    "47666099-ffb2-4d07-a801-70524dba0837",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// HOB 113 — Stone-Giant of High Pass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_GIANT_OF_HIGH_PASS: CardRecord = CardRecord::new(
    "Stone-Giant of High Pass",
    "5f4f4683-ffd2-447a-932b-276f7fa17cca",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 114 — Thorin, Mountain-king
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORIN_MOUNTAIN_KING: CardRecord = CardRecord::new(
    "Thorin, Mountain-king",
    "117347af-0dd7-4350-901d-8c8a81387e22",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// HOB 115 — Tidings of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDINGS_OF_WAR: CardRecord = CardRecord::new(
    "Tidings of War",
    "38c16a0a-375e-48cb-9720-dbbc08c603ae",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// HOB 116 — Attercop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATTERCOP: CardRecord = CardRecord::new(
    "Attercop",
    "81263d5d-e402-4813-9458-161112da27ab",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// HOB 117 — Bejeweled Warg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEJEWELED_WARG: CardRecord = CardRecord::new(
    "Bejeweled Warg",
    "e95eba5c-e0d6-46b4-a0be-8e373b2185ea",
    "John Di Giovanni",
    crate::card::CardRules::unsupported(),
);

// HOB 118 — Beorn, Reluctant Host // Till and Tend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEORN_RELUCTANT_HOST: CardRecord = CardRecord::new(
    "Beorn, Reluctant Host // Till and Tend",
    "804589b7-3ef9-473d-97cc-c61a2d41f70d",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// HOB 119 — Beorn the Fierce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEORN_THE_FIERCE: CardRecord = CardRecord::new(
    "Beorn the Fierce",
    "367d5f8b-77ee-47f7-bc71-972d62c280a9",
    "Nia Kovalevski",
    crate::card::CardRules::unsupported(),
);

// HOB 120 — Beorn's Hospitality
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEORN_S_HOSPITALITY: CardRecord = CardRecord::new(
    "Beorn's Hospitality",
    "153ca57e-30f0-4ad7-ae9d-c55cbf0fd4c9",
    "Harkalé Linaï",
    crate::card::CardRules::unsupported(),
);

// HOB 121 — Boughside Wanderers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUGHSIDE_WANDERERS: CardRecord = CardRecord::new(
    "Boughside Wanderers",
    "71bec005-2925-4944-be16-2cc5eb30f5d6",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// HOB 122 — Cantankerous Keepers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANTANKEROUS_KEEPERS: CardRecord = CardRecord::new(
    "Cantankerous Keepers",
    "fae46a70-a6d3-4584-859d-6c7425fb1508",
    "Ramza Psyru",
    crate::card::CardRules::unsupported(),
);

// HOB 123 — Dancing from Dark to Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DANCING_FROM_DARK_TO_DAWN: CardRecord = CardRecord::new(
    "Dancing from Dark to Dawn",
    "550cd0b6-ca61-4db7-9d20-0b68c48066f9",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// HOB 124 — Down in the Valley
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOWN_IN_THE_VALLEY: CardRecord = CardRecord::new(
    "Down in the Valley",
    "c8aa5179-475b-4cc8-b21e-205b475eb4cf",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// HOB 125 — Galion, Elvenking's Butler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALION_ELVENKING_S_BUTLER: CardRecord = CardRecord::new(
    "Galion, Elvenking's Butler",
    "985bd676-58c4-42c7-a570-1b413e9aa94c",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// HOB 126 — Gigantic Big Bear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIGANTIC_BIG_BEAR: CardRecord = CardRecord::new(
    "Gigantic Big Bear",
    "7d6ece3d-8e7a-41ad-974f-3c9748de4825",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// HOB 127 — Guardian of the Halls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUARDIAN_OF_THE_HALLS: CardRecord = CardRecord::new(
    "Guardian of the Halls",
    "4265caec-8c28-44cd-8e6b-90b5af926d3c",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// HOB 128 — Little Bear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LITTLE_BEAR: CardRecord = CardRecord::new(
    "Little Bear",
    "8a50858a-33b5-4c45-9c31-5956ae5a33a6",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 129 — Mirkwood Pathmaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRKWOOD_PATHMAKER: CardRecord = CardRecord::new(
    "Mirkwood Pathmaker",
    "50fbedc0-bc66-4ffb-87f6-a2df69995091",
    "Sean Vo",
    crate::card::CardRules::unsupported(),
);

// HOB 130 — Nasty Little Rabbit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NASTY_LITTLE_RABBIT: CardRecord = CardRecord::new(
    "Nasty Little Rabbit",
    "96bc7d25-2828-478a-8fe5-a1f4ede8c9c0",
    "Harkalé Linaï",
    crate::card::CardRules::unsupported(),
);

// HOB 131 — The Notary Hobbits
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_NOTARY_HOBBITS: CardRecord = CardRecord::new(
    "The Notary Hobbits",
    "d876315f-b269-4254-a517-905c6e927462",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// HOB 132 — Old Fat Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OLD_FAT_SPIDER: CardRecord = CardRecord::new(
    "Old Fat Spider",
    "e0c0f842-40fe-4776-a988-a35216bcfd47",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// HOB 133 — Ordinary Bear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDINARY_BEAR: CardRecord = CardRecord::new(
    "Ordinary Bear",
    "0feb9817-56e1-465a-851c-b2fe202aa8ae",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// HOB 134 — Part in Friendship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PART_IN_FRIENDSHIP: CardRecord = CardRecord::new(
    "Part in Friendship",
    "b4ff1eac-6d97-40ab-9b7c-c2fdca0917d9",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// HOB 135 — Quarrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUARREL: CardRecord = CardRecord::new(
    "Quarrel",
    "5900a0b4-aa89-4019-94c9-7e9ea3b4792e",
    "Denman Rooke",
    crate::card::CardRules::unsupported(),
);

// HOB 136 — Radagast of Rhosgobel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADAGAST_OF_RHOSGOBEL: CardRecord = CardRecord::new(
    "Radagast of Rhosgobel",
    "5741bbad-a6e4-45e0-b827-73f48c9975bf",
    "Anna Podedworna",
    crate::card::CardRules::unsupported(),
);

// HOB 137 — Through the Forest Gate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THROUGH_THE_FOREST_GATE: CardRecord = CardRecord::new(
    "Through the Forest Gate",
    "880adfc8-69cf-4062-a804-e65b6cb6056d",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// HOB 138 — Troll Negotiations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROLL_NEGOTIATIONS: CardRecord = CardRecord::new(
    "Troll Negotiations",
    "ca0f7bf4-b8a2-4ec4-ad7e-b639de9fa76a",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// HOB 139 — Warg Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARG_TACTICS: CardRecord = CardRecord::new(
    "Warg Tactics",
    "b06d9cee-bb0f-4fe7-ab2a-b55d36461aec",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// HOB 140 — Wargling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARGLING: CardRecord = CardRecord::new(
    "Wargling",
    "1ccbf823-846f-4f09-9c67-1deebb5d1d92",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// HOB 141 — Wilderland Scrounger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILDERLAND_SCROUNGER: CardRecord = CardRecord::new(
    "Wilderland Scrounger",
    "63078f42-f404-4c61-86be-45d934393b0a",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 142 — Wood Elves (reprint)
const WOOD_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_por::WOOD_ELVES,
    "2476e42b-b209-4207-8ed9-cb668f89b218",
    "Andreia Ugrai",
);

// HOB 143 — Woodland Weavemaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOODLAND_WEAVEMASTER: CardRecord = CardRecord::new(
    "Woodland Weavemaster",
    "fe2b4bcf-56de-44d3-83af-aeb27f82c25e",
    "Nia Kovalevski",
    crate::card::CardRules::unsupported(),
);

// HOB 144 — Bard, King of Dale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARD_KING_OF_DALE: CardRecord = CardRecord::new(
    "Bard, King of Dale",
    "c05c2aa6-29c7-40f8-872e-91099b9225c4",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// HOB 145 — Bard the Bowman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARD_THE_BOWMAN: CardRecord = CardRecord::new(
    "Bard the Bowman",
    "0b84e232-428c-424a-848c-ef95debc6e50",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 146 — Bard's Company
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARD_S_COMPANY: CardRecord = CardRecord::new(
    "Bard's Company",
    "d14aa2ff-7bbd-47a6-8e36-481e56302a62",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// HOB 147 — Bifur, Melodic Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIFUR_MELODIC_RIDER: CardRecord = CardRecord::new(
    "Bifur, Melodic Rider",
    "ee88cc80-8fbf-451c-b2b8-09158426c26a",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// HOB 148 — Bolg of the North
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOLG_OF_THE_NORTH: CardRecord = CardRecord::new(
    "Bolg of the North",
    "7b2d2a7f-88e0-45a9-8579-a6736bcd66eb",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 149 — Bolg's Company
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOLG_S_COMPANY: CardRecord = CardRecord::new(
    "Bolg's Company",
    "ea3f5644-f7e3-40de-ada5-cea2e9113cfb",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// HOB 150 — The Chief Warg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_CHIEF_WARG: CardRecord = CardRecord::new(
    "The Chief Warg",
    "c397a298-bf7f-49d7-a26a-206ccf9e8120",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 151 — Chief Warg's Company
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIEF_WARG_S_COMPANY: CardRecord = CardRecord::new(
    "Chief Warg's Company",
    "bbc634af-63d2-444a-8123-85f16fe3e364",
    "Jason Kang",
    crate::card::CardRules::unsupported(),
);

// HOB 152 — Dáin's Company
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAIN_S_COMPANY: CardRecord = CardRecord::new(
    "Dáin's Company",
    "36db4405-8589-481f-b627-f26087488337",
    "Erikas Perl",
    crate::card::CardRules::unsupported(),
);

// HOB 153 — Duskwatch Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKWATCH_HUNTER: CardRecord = CardRecord::new(
    "Duskwatch Hunter",
    "3685c783-d837-4466-a960-ab3098db64c3",
    "Samuele Bandini",
    crate::card::CardRules::unsupported(),
);

// HOB 154 — Dwalin, Weaponmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWALIN_WEAPONMASTER: CardRecord = CardRecord::new(
    "Dwalin, Weaponmaster",
    "196d9287-a37d-4b27-a83b-a5489a54f081",
    "Marco Teixeira",
    crate::card::CardRules::unsupported(),
);

// HOB 155 — Eagle's Rescue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EAGLE_S_RESCUE: CardRecord = CardRecord::new(
    "Eagle's Rescue",
    "12c8f2cc-ac9d-4cf6-9025-efe366b4e07f",
    "Ramza Psyru",
    crate::card::CardRules::unsupported(),
);

// HOB 156 — Fearsome Goblin Pair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEARSOME_GOBLIN_PAIR: CardRecord = CardRecord::new(
    "Fearsome Goblin Pair",
    "2efe2dc7-3eaa-47f6-b1ae-f974c4a8ae79",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// HOB 157 — Goblin Plate Mail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_PLATE_MAIL: CardRecord = CardRecord::new(
    "Goblin Plate Mail",
    "cb982607-da37-4894-91a5-cf6307d4d703",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 158 — The Great Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_GREAT_GOBLIN: CardRecord = CardRecord::new(
    "The Great Goblin",
    "78d8f53e-537d-4eaa-99e3-cac57fa53d22",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 159 — Large Bear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LARGE_BEAR: CardRecord = CardRecord::new(
    "Large Bear",
    "50202288-f433-4b56-8f60-349bda7b4f6b",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// HOB 160 — Mirkwood Nurturer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRKWOOD_NURTURER: CardRecord = CardRecord::new(
    "Mirkwood Nurturer",
    "704b45e4-566e-40f6-a33a-9151018b44e5",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// HOB 161 — Nori, Teller of Tales
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NORI_TELLER_OF_TALES: CardRecord = CardRecord::new(
    "Nori, Teller of Tales",
    "b05adb48-980c-49a0-9ce6-7c7f3f20715d",
    "Marco Teixeira",
    crate::card::CardRules::unsupported(),
);

// HOB 162 — Patient Instructor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATIENT_INSTRUCTOR: CardRecord = CardRecord::new(
    "Patient Instructor",
    "e4800508-8bb9-41bb-8712-b55fba7a80a5",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// HOB 163 — Silvan Reveler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVAN_REVELER: CardRecord = CardRecord::new(
    "Silvan Reveler",
    "c71b74fb-fb0c-4953-b536-7a3f283c6918",
    "Bokun An",
    crate::card::CardRules::unsupported(),
);

// HOB 164 — Smaug, Wicked Worm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMAUG_WICKED_WORM: CardRecord = CardRecord::new(
    "Smaug, Wicked Worm",
    "19cc91f0-e724-41ac-b6d8-9a293bd63b42",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// HOB 165 — Thorin Oakenshield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORIN_OAKENSHIELD: CardRecord = CardRecord::new(
    "Thorin Oakenshield",
    "c7e18609-d1ed-4829-be11-f2ce2cfcbc49",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// HOB 166 — Thranduil, Sindarin Liege // Silvan Rally
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRANDUIL_SINDARIN_LIEGE: CardRecord = CardRecord::new(
    "Thranduil, Sindarin Liege // Silvan Rally",
    "481870ee-d1f7-421b-86e1-570ea933bbbc",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// HOB 167 — Thranduil, the Elvenking
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRANDUIL_THE_ELVENKING: CardRecord = CardRecord::new(
    "Thranduil, the Elvenking",
    "fe2fe8fa-3b99-44c1-bab9-922e5c864952",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// HOB 168 — Thranduil's Company
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRANDUIL_S_COMPANY: CardRecord = CardRecord::new(
    "Thranduil's Company",
    "abdb9d4e-e6ca-409b-b589-0cf71724340b",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// HOB 169 — Tom, Bert, and William
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOM_BERT_AND_WILLIAM: CardRecord = CardRecord::new(
    "Tom, Bert, and William",
    "211a9764-3c60-46ba-bb53-e6692640ec8f",
    "Leonardo Borazio",
    crate::card::CardRules::unsupported(),
);

// HOB 170 — The Arkenstone // Seek the Heart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_ARKENSTONE: CardRecord = CardRecord::new(
    "The Arkenstone // Seek the Heart",
    "a56a88ba-fcfa-4b56-bdae-a080b297b871",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// HOB 171 — The Black Arrow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_BLACK_ARROW: CardRecord = CardRecord::new(
    "The Black Arrow",
    "ab181190-d53d-4972-8cd5-8e54b45f2276",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// HOB 172 — Dwarven Mattock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_MATTOCK: CardRecord = CardRecord::new(
    "Dwarven Mattock",
    "92c6f09d-b525-4e8c-a87c-a74df9dc3b1e",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// HOB 173 — Giant's Boulder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_S_BOULDER: CardRecord = CardRecord::new(
    "Giant's Boulder",
    "ce254758-c928-4b43-a952-13fac1845668",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// HOB 174 — Glamdring, Foe-hammer // Gleam of Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLAMDRING_FOE_HAMMER: CardRecord = CardRecord::new(
    "Glamdring, Foe-hammer // Gleam of Death",
    "a5cfbfde-783e-46ca-b3cf-11f16209d6cb",
    "Chris Cold",
    crate::card::CardRules::unsupported(),
);

// HOB 175 — Key to the Side-Door
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEY_TO_THE_SIDE_DOOR: CardRecord = CardRecord::new(
    "Key to the Side-Door",
    "898c14a2-d897-4341-83ed-eee666df9648",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// HOB 176 — My Precious // Allure of Power
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MY_PRECIOUS: CardRecord = CardRecord::new(
    "My Precious // Allure of Power",
    "15ae4d50-be2f-412c-bb6b-b0a06b60474a",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// HOB 177 — Orcrist, Goblin-cleaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCRIST_GOBLIN_CLEAVER: CardRecord = CardRecord::new(
    "Orcrist, Goblin-cleaver",
    "f54f1c1d-6a22-43e9-a842-0a1ae25b323c",
    "Erikas Perl",
    crate::card::CardRules::unsupported(),
);

// HOB 178 — Sting, Bilbo's Sword
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STING_BILBO_S_SWORD: CardRecord = CardRecord::new(
    "Sting, Bilbo's Sword",
    "d6a8d698-c454-42c4-ad4e-9a7625d5569f",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// HOB 179 — Thrór's Map
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THROR_S_MAP: CardRecord = CardRecord::new(
    "Thrór's Map",
    "ad0dba36-d056-4bc1-987a-391da26ad267",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// HOB 180 — Well-Worn Spatula
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_WORN_SPATULA: CardRecord = CardRecord::new(
    "Well-Worn Spatula",
    "659b687f-4068-496f-81b2-7b606bf07ec1",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// HOB 181 — Elven Passage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVEN_PASSAGE: CardRecord = CardRecord::new(
    "Elven Passage",
    "dd1fd2ab-2565-4798-a832-fc849df82f74",
    "Shahab Alizadeh",
    crate::card::CardRules::unsupported(),
);

// HOB 182 — Elvenking's Halls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVENKING_S_HALLS: CardRecord = CardRecord::new(
    "Elvenking's Halls",
    "cd477096-41b1-4907-9cb3-852cb22c9ba2",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// HOB 183 — Goblin-town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_TOWN: CardRecord = CardRecord::new(
    "Goblin-town",
    "d76df9d0-56cf-4351-a5e8-e6ae6fc791d1",
    "Sean Vo",
    crate::card::CardRules::unsupported(),
);

// HOB 184 — Hobbit Hole
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOBBIT_HOLE: CardRecord = CardRecord::new(
    "Hobbit Hole",
    "0365c439-30bf-4d32-a791-166751bdb996",
    "Shahab Alizadeh",
    crate::card::CardRules::unsupported(),
);

// HOB 185 — Iron Hills
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_HILLS: CardRecord = CardRecord::new(
    "Iron Hills",
    "78045c43-5cbe-48ff-837d-e7c6baac2937",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// HOB 186 — Lake-town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAKE_TOWN: CardRecord = CardRecord::new(
    "Lake-town",
    "2fbd0584-81a7-4c47-8af1-1c8635899a97",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// HOB 187 — The Lonely Mountain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_LONELY_MOUNTAIN: CardRecord = CardRecord::new(
    "The Lonely Mountain",
    "b39ebc4d-a01a-4401-ab3a-bf6142c93b47",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// HOB 188 — Mirkwood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRKWOOD: CardRecord = CardRecord::new(
    "Mirkwood",
    "612cf954-f86c-4629-99df-4874d56fded3",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// HOB 189 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "7b7c408b-8660-4db5-9a16-5003c11b4ac1",
    "Chris Cold",
);

// HOB 190 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "c6aa89a8-3584-4906-b9a9-41ef2f021f8e",
    "Kamila Szutenberg",
);

// HOB 191 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "4031e5e4-e573-4130-8d20-4a606edef0a0",
    "Erikas Perl",
);

// HOB 192 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "c49d378e-9549-4320-b3c6-1aeb216d1e98",
    "Shahab Alizadeh",
);

// HOB 193 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "c3e84b42-5423-4d4d-b8fc-cfbb2c53a4ca",
    "Kamila Szutenberg",
);

// HOB 194 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "24dc369c-020a-4115-a4bb-d60a44de64e3",
    "WFlemming Illustration",
);

// HOB 195 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "739aaaac-c424-4ea7-a084-62a6fc0438b0",
    "WFlemming Illustration",
);

// HOB 196 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "c5f590a3-9993-4ac4-a93c-1beb44eda17b",
    "WFlemming Illustration",
);

// HOB 197 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "51acfb01-4b0b-48fc-9704-a9b4a1e43a23",
    "WFlemming Illustration",
);

// HOB 198 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "5f533364-0f91-4e49-aaeb-83c4c1f6d316",
    "WFlemming Illustration",
);

// HOB 199 — Troop of Ponies (alternate printing)
const TROOP_OF_PONIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TROOP_OF_PONIES,
    1,
    "6cbcd606-b864-4b81-9596-e7788befdd1f",
    "Ted Nasmith",
);

// HOB 200 — Rage into the Valley (alternate printing)
const RAGE_INTO_THE_VALLEY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAGE_INTO_THE_VALLEY,
    1,
    "349d7464-e778-4a24-a528-d31d36ff4799",
    "Ted Nasmith",
);

// HOB 201 — The Great Goblin (alternate printing)
const THE_GREAT_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_GREAT_GOBLIN,
    1,
    "720ae752-c3a7-4bc0-bc0f-a9772bea3960",
    "Ted Nasmith",
);

// HOB 202 — Thorin Oakenshield (alternate printing)
const THORIN_OAKENSHIELD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORIN_OAKENSHIELD,
    1,
    "b6d8d14d-4713-40ea-9509-81ce68b2184d",
    "Ted Nasmith",
);

// HOB 203 — Gandalf, Spark Starter (alternate printing)
const GANDALF_SPARK_STARTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GANDALF_SPARK_STARTER,
    1,
    "ccf2923e-5358-4e40-bc64-2953ac2d1692",
    "Ted Nasmith",
);

// HOB 204 — Glamdring, Foe-hammer // Gleam of Death (alternate printing)
const GLAMDRING_FOE_HAMMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLAMDRING_FOE_HAMMER,
    1,
    "53cd94e9-6006-4d43-8032-7920103740fa",
    "Ted Nasmith",
);

// HOB 205 — The Eagles Are Coming! (alternate printing)
const THE_EAGLES_ARE_COMING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_EAGLES_ARE_COMING,
    1,
    "62a7ff97-b940-4745-9e2e-3258fb51b846",
    "Denman Rooke",
);

// HOB 206 — Dreaded Bat-Cloud (alternate printing)
const DREADED_BAT_CLOUD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DREADED_BAT_CLOUD,
    1,
    "1d84b0f8-aa9d-46b5-b46d-cb291aa6af9a",
    "Denman Rooke",
);

// HOB 207 — The Lonely Mountain (alternate printing)
const THE_LONELY_MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LONELY_MOUNTAIN,
    1,
    "0015ae49-bc21-4cf3-bb46-3df52760e183",
    "Denman Rooke",
);

// HOB 208 — Chief Warg's Company (alternate printing)
const CHIEF_WARG_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHIEF_WARG_S_COMPANY,
    1,
    "0dd7b2d8-ca9d-4125-b57e-9b14deae8d9b",
    "Denman Rooke",
);

// HOB 209 — Thorin's Last Stand (alternate printing)
const THORIN_S_LAST_STAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORIN_S_LAST_STAND,
    1,
    "089e8e4b-c367-4d86-a69a-951f7d779aaf",
    "Denman Rooke",
);

// HOB 210 — Bard's Company (alternate printing)
const BARD_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARD_S_COMPANY,
    1,
    "7ca1300a-0ac3-413b-9617-5e8d1b5f1ca2",
    "Denman Rooke",
);

// HOB 211 — Bolg's Company (alternate printing)
const BOLG_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOLG_S_COMPANY,
    1,
    "a2ba9d82-83b6-43dd-8eb8-2401be0df754",
    "Denman Rooke",
);

// HOB 212 — Dáin's Company (alternate printing)
const DAIN_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAIN_S_COMPANY,
    1,
    "323f7ae8-9247-479e-aead-f46018a3d81d",
    "Denman Rooke",
);

// HOB 213 — Thranduil's Company (alternate printing)
const THRANDUIL_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_S_COMPANY,
    1,
    "fcdfce79-c8da-4662-93c2-054e5328e18c",
    "Denman Rooke",
);

// HOB 214 — Belladonna Took (alternate printing)
const BELLADONNA_TOOK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BELLADONNA_TOOK,
    1,
    "0ff767b1-b04d-4d1d-bc91-3bd1361c5ee8",
    "Kristina Carroll",
);

// HOB 215 — Bofur, Reliable Guardian // Concerted Care (alternate printing)
const BOFUR_RELIABLE_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOFUR_RELIABLE_GUARDIAN,
    1,
    "a221baee-6ed1-4f11-b38c-e0be8531e170",
    "Cory Godbey",
);

// HOB 216 — Iron Hills Blacksmith (alternate printing)
const IRON_HILLS_BLACKSMITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRON_HILLS_BLACKSMITH,
    1,
    "6b428a9a-3a59-4416-b00b-78e2cb73e2d6",
    "Francisco Badilla",
);

// HOB 217 — The Queen of Dale (alternate printing)
const THE_QUEEN_OF_DALE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_QUEEN_OF_DALE,
    1,
    "7d635dfb-0a48-4965-82d1-d98f943bb28d",
    "Ashley Mackenzie",
);

// HOB 218 — Bilbo, Luckwearer // Burglar's Plot (alternate printing)
const BILBO_LUCKWEARER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BILBO_LUCKWEARER,
    1,
    "b5fe776d-00c2-4d5d-9493-77dd754aa728",
    "Cory Godbey",
);

// HOB 219 — Bilbo, Thief in the Night (alternate printing)
const BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BILBO_THIEF_IN_THE_NIGHT,
    1,
    "dda07973-ed78-4f50-8794-f5cbfc6e8975",
    "Ashley Mackenzie",
);

// HOB 220 — Fateful Discovery (alternate printing)
const FATEFUL_DISCOVERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FATEFUL_DISCOVERY,
    1,
    "958f0de2-d209-4e97-9ad4-dedf5da304b8",
    "Ashley Mackenzie",
);

// HOB 221 — Most Decrepit Old Bird // Speak Secrets (alternate printing)
const MOST_DECREPIT_OLD_BIRD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOST_DECREPIT_OLD_BIRD,
    1,
    "9093b067-e18e-4a6a-b4a6-cd67bcd4d5a7",
    "Andrea Radeck",
);

// HOB 222 — Azog, Moria's Ruin (alternate printing)
const AZOG_MORIA_S_RUIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AZOG_MORIA_S_RUIN,
    1,
    "0b9ef17a-02a4-44c6-b0b3-9eccf7f324fd",
    "Dibujante Nocturno",
);

// HOB 223 — Great Ugly-Looking Goblin // Clap! Snap! (alternate printing)
const GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_UGLY_LOOKING_GOBLIN,
    1,
    "7f63f810-15a8-4bec-bd39-dbca453bcc1e",
    "Andrea Radeck",
);

// HOB 224 — Head of the Hunt (alternate printing)
const HEAD_OF_THE_HUNT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEAD_OF_THE_HUNT,
    1,
    "d670ed30-e7e0-4908-ac9c-bdcab61092ef",
    "Benjamin Ee",
);

// HOB 225 — Desert Were-Worm (alternate printing)
const DESERT_WERE_WORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DESERT_WERE_WORM,
    1,
    "3f4794de-701d-4619-8b8b-cbfdd860c567",
    "Dibujante Nocturno",
);

// HOB 226 — Desolation of Smaug (alternate printing)
const DESOLATION_OF_SMAUG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DESOLATION_OF_SMAUG,
    1,
    "738d50ff-1bff-4841-93de-ccf521b2542f",
    "Dibujante Nocturno",
);

// HOB 227 — Glóin the Mighty // Easy Pickings (alternate printing)
const GLOIN_THE_MIGHTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLOIN_THE_MIGHTY,
    1,
    "7c9bfc6e-648a-482b-9483-9e8538726f53",
    "Francisco Badilla",
);

// HOB 228 — Last Light of Durin's Day (alternate printing)
const LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAST_LIGHT_OF_DURIN_S_DAY,
    1,
    "ba42b0f3-3f2f-4fbe-b56d-c2cb477f1182",
    "Francisco Badilla",
);

// HOB 229 — Smaug the Magnificent (alternate printing)
const SMAUG_THE_MAGNIFICENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_THE_MAGNIFICENT,
    1,
    "4edcf8eb-d0b0-4aec-bec0-64c56982cdce",
    "John Tedrick",
);

// HOB 230 — Beorn the Fierce (alternate printing)
const BEORN_THE_FIERCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEORN_THE_FIERCE,
    1,
    "949e4c23-d0e7-4e11-abd4-33ff78b1c963",
    "Dibujante Nocturno",
);

// HOB 231 — Dancing from Dark to Dawn (alternate printing)
const DANCING_FROM_DARK_TO_DAWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DANCING_FROM_DARK_TO_DAWN,
    1,
    "85f488cf-eb8a-43f6-9ed8-523daba284fa",
    "Francisco Badilla",
);

// HOB 232 — The Notary Hobbits (alternate printing)
const THE_NOTARY_HOBBITS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_NOTARY_HOBBITS,
    1,
    "b2b6ac15-2266-442a-99e5-96db904e1fd7",
    "Ashley Mackenzie",
);

// HOB 233 — Thranduil, Sindarin Liege // Silvan Rally (alternate printing)
const THRANDUIL_SINDARIN_LIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_SINDARIN_LIEGE,
    1,
    "0992ce8b-7bef-4799-a52d-7a46021a8672",
    "Cory Godbey",
);

// HOB 234 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    1,
    "ba3ff39b-b591-4434-8d41-4826d0809ca7",
    "Francisco Badilla",
);

// HOB 235 — My Precious // Allure of Power (alternate printing)
const MY_PRECIOUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MY_PRECIOUS,
    1,
    "ceb193ed-b05f-4820-a17f-e2982bf547ae",
    "Barbara Rosiak",
);

// HOB 236 — Orcrist, Goblin-cleaver (alternate printing)
const ORCRIST_GOBLIN_CLEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORCRIST_GOBLIN_CLEAVER,
    1,
    "758b4398-4522-49c8-94de-b0d647532544",
    "Francisco Badilla",
);

// HOB 237 — Sting, Bilbo's Sword (alternate printing)
const STING_BILBO_S_SWORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STING_BILBO_S_SWORD,
    1,
    "30420a65-65f2-4331-9a5d-b1771da936b0",
    "Barbara Rosiak",
);

// HOB 238 — Elven Passage (alternate printing)
const ELVEN_PASSAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVEN_PASSAGE,
    1,
    "c9789696-a702-4294-afd7-c48e5fc84398",
    "Kristina Carroll",
);

// HOB 239 — Gleaming Splendor (alternate printing)
const GLEAMING_SPLENDOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEAMING_SPLENDOR,
    1,
    "4a4ca473-3000-4e98-9881-03372c4f79a5",
    "James Bousema",
);

// HOB 240 — The Lord of the Eagles (alternate printing)
const THE_LORD_OF_THE_EAGLES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LORD_OF_THE_EAGLES,
    1,
    "8acfaac6-5a63-4fd6-999b-562e989e2ce1",
    "Tyler Jacobson",
);

// HOB 241 — Gollum, Riddle Master (alternate printing)
const GOLLUM_RIDDLE_MASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOLLUM_RIDDLE_MASTER,
    1,
    "0c13e282-bbfb-4601-94ff-803874634371",
    "Tyler Jacobson",
);

// HOB 242 — Gandalf, Goblins' Bane // Flameshape (alternate printing)
const GANDALF_GOBLINS_BANE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GANDALF_GOBLINS_BANE,
    1,
    "efc66a6b-8ac7-4cb7-82f1-710fb0e86f82",
    "James Bousema",
);

// HOB 243 — Thorin, Mountain-king (alternate printing)
const THORIN_MOUNTAIN_KING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORIN_MOUNTAIN_KING,
    1,
    "2401d2a5-f65b-4a13-906c-999990589a1e",
    "James Bousema",
);

// HOB 244 — Bard, King of Dale (alternate printing)
const BARD_KING_OF_DALE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARD_KING_OF_DALE,
    1,
    "a4adb258-381e-4e64-9f93-9fcf943be360",
    "James Bousema",
);

// HOB 245 — Smaug, Wicked Worm (alternate printing)
const SMAUG_WICKED_WORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_WICKED_WORM,
    1,
    "189a7d55-c3ec-4402-a3b3-72817f533e87",
    "Tyler Jacobson",
);

// HOB 246 — Thranduil, the Elvenking (alternate printing)
const THRANDUIL_THE_ELVENKING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_THE_ELVENKING,
    1,
    "a416ca91-e646-40b4-824c-24aed6b9c683",
    "James Bousema",
);

// HOB 247 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    2,
    "6c6193e8-453b-49dd-ad65-ee6e363bff93",
    "Tyler Jacobson",
);

// HOB 248 — The Lonely Mountain (alternate printing)
const THE_LONELY_MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_LONELY_MOUNTAIN,
    2,
    "d9919a30-d60e-438e-b132-22e56b3cdf68",
    "Tyler Jacobson",
);

// HOB 249 — Smaug the Magnificent (alternate printing)
const SMAUG_THE_MAGNIFICENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_THE_MAGNIFICENT,
    2,
    "43c56e15-ba46-4a27-b9b0-55cde9f8c933",
    "Ted Nasmith",
);

// HOB 250 — Belladonna Took (alternate printing)
const BELLADONNA_TOOK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BELLADONNA_TOOK,
    2,
    "f2522398-39d2-43d3-9019-349e22438f65",
    "Kristina Carroll",
);

// HOB 251 — Bofur, Reliable Guardian // Concerted Care (alternate printing)
const BOFUR_RELIABLE_GUARDIAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BOFUR_RELIABLE_GUARDIAN,
    2,
    "7d9a89d1-3122-4fd8-b77f-5163c01b560c",
    "Cory Godbey",
);

// HOB 252 — Iron Hills Blacksmith (alternate printing)
const IRON_HILLS_BLACKSMITH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &IRON_HILLS_BLACKSMITH,
    2,
    "8b592de0-0bf9-4972-9f43-c8ac1c3a1299",
    "Francisco Badilla",
);

// HOB 253 — The Queen of Dale (alternate printing)
const THE_QUEEN_OF_DALE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_QUEEN_OF_DALE,
    2,
    "4cff1878-3f23-4a0a-ad79-28a44921cca8",
    "Ashley Mackenzie",
);

// HOB 254 — Bilbo, Luckwearer // Burglar's Plot (alternate printing)
const BILBO_LUCKWEARER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BILBO_LUCKWEARER,
    2,
    "79892a5d-80df-4ee1-b482-30e57aaabf21",
    "Cory Godbey",
);

// HOB 255 — Bilbo, Thief in the Night (alternate printing)
const BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BILBO_THIEF_IN_THE_NIGHT,
    2,
    "c0c313d6-5a76-459d-9db1-9491c57861fe",
    "Ashley Mackenzie",
);

// HOB 256 — Fateful Discovery (alternate printing)
const FATEFUL_DISCOVERY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FATEFUL_DISCOVERY,
    2,
    "6a928a14-b906-402f-96e4-8b9391d6536c",
    "Ashley Mackenzie",
);

// HOB 257 — Most Decrepit Old Bird // Speak Secrets (alternate printing)
const MOST_DECREPIT_OLD_BIRD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOST_DECREPIT_OLD_BIRD,
    2,
    "289a6c72-3d0d-402d-8e27-9ad0e254d5f6",
    "Andrea Radeck",
);

// HOB 258 — Azog, Moria's Ruin (alternate printing)
const AZOG_MORIA_S_RUIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AZOG_MORIA_S_RUIN,
    2,
    "2e359014-f003-4dd6-bc97-ef2a5c515a23",
    "Dibujante Nocturno",
);

// HOB 259 — Great Ugly-Looking Goblin // Clap! Snap! (alternate printing)
const GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GREAT_UGLY_LOOKING_GOBLIN,
    2,
    "68fe7914-21b4-44eb-9432-3b1637864bc1",
    "Andrea Radeck",
);

// HOB 260 — Head of the Hunt (alternate printing)
const HEAD_OF_THE_HUNT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HEAD_OF_THE_HUNT,
    2,
    "77d2a543-d94f-4814-a60f-72c224e5768a",
    "Benjamin Ee",
);

// HOB 261 — Desert Were-Worm (alternate printing)
const DESERT_WERE_WORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DESERT_WERE_WORM,
    2,
    "cfe91642-7651-4902-8386-b18cfb506925",
    "Dibujante Nocturno",
);

// HOB 262 — Desolation of Smaug (alternate printing)
const DESOLATION_OF_SMAUG_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DESOLATION_OF_SMAUG,
    2,
    "4bb4033e-8b96-4c35-ab03-ff188002ae73",
    "Dibujante Nocturno",
);

// HOB 263 — Glóin the Mighty // Easy Pickings (alternate printing)
const GLOIN_THE_MIGHTY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GLOIN_THE_MIGHTY,
    2,
    "5546390b-3957-4e3b-aa64-85692381f53f",
    "Francisco Badilla",
);

// HOB 264 — Last Light of Durin's Day (alternate printing)
const LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LAST_LIGHT_OF_DURIN_S_DAY,
    2,
    "7cb22e85-8d54-48a9-a006-d6e2f3ff00e1",
    "Francisco Badilla",
);

// HOB 265 — Smaug the Magnificent (alternate printing)
const SMAUG_THE_MAGNIFICENT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_THE_MAGNIFICENT,
    3,
    "b6529666-0ec8-45a8-8740-3bffc19d2265",
    "John Tedrick",
);

// HOB 266 — Beorn the Fierce (alternate printing)
const BEORN_THE_FIERCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BEORN_THE_FIERCE,
    2,
    "9f921e6b-12f0-4497-86ee-f2b4f4fa319a",
    "Dibujante Nocturno",
);

// HOB 267 — Dancing from Dark to Dawn (alternate printing)
const DANCING_FROM_DARK_TO_DAWN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DANCING_FROM_DARK_TO_DAWN,
    2,
    "9fa9a9de-ca46-4ea1-8fdb-4ccd3ef86805",
    "Francisco Badilla",
);

// HOB 268 — The Notary Hobbits (alternate printing)
const THE_NOTARY_HOBBITS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_NOTARY_HOBBITS,
    2,
    "0e88b614-f5b3-4e1e-beed-3c49e366be11",
    "Ashley Mackenzie",
);

// HOB 269 — Thranduil, Sindarin Liege // Silvan Rally (alternate printing)
const THRANDUIL_SINDARIN_LIEGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_SINDARIN_LIEGE,
    2,
    "a84cd965-fe5f-42e5-9775-d0285fe84308",
    "Cory Godbey",
);

// HOB 270 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    3,
    "2d08a806-c70d-4421-a9eb-e29e54e4268f",
    "Francisco Badilla",
);

// HOB 271 — My Precious // Allure of Power (alternate printing)
const MY_PRECIOUS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MY_PRECIOUS,
    2,
    "ce239994-1bc6-4083-b9cf-776e862c5479",
    "Barbara Rosiak",
);

// HOB 272 — Orcrist, Goblin-cleaver (alternate printing)
const ORCRIST_GOBLIN_CLEAVER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ORCRIST_GOBLIN_CLEAVER,
    2,
    "a496a9e4-8dd7-44a2-a4b2-d4ecfc8ee7c8",
    "Francisco Badilla",
);

// HOB 273 — Sting, Bilbo's Sword (alternate printing)
const STING_BILBO_S_SWORD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STING_BILBO_S_SWORD,
    2,
    "7d57f11f-1698-4cf3-bb0e-bd9b0ea8442d",
    "Barbara Rosiak",
);

// HOB 274 — Elven Passage (alternate printing)
const ELVEN_PASSAGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELVEN_PASSAGE,
    2,
    "75cb4095-b252-49b0-ad2e-0ea52ae4709a",
    "Kristina Carroll",
);

// HOB 275 — Gleaming Splendor (alternate printing)
const GLEAMING_SPLENDOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GLEAMING_SPLENDOR,
    2,
    "42a1986c-9585-4544-b5a7-bee4be5c4506",
    "James Bousema",
);

// HOB 276 — The Lord of the Eagles (alternate printing)
const THE_LORD_OF_THE_EAGLES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_LORD_OF_THE_EAGLES,
    2,
    "b0f29cce-324d-4229-8e29-692594b2e3c7",
    "Tyler Jacobson",
);

// HOB 277 — Gollum, Riddle Master (alternate printing)
const GOLLUM_RIDDLE_MASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOLLUM_RIDDLE_MASTER,
    2,
    "12d441b7-9349-47a1-b61e-9d563bc085b5",
    "Tyler Jacobson",
);

// HOB 278 — Gandalf, Goblins' Bane // Flameshape (alternate printing)
const GANDALF_GOBLINS_BANE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GANDALF_GOBLINS_BANE,
    2,
    "0db45ef8-5bc5-4914-9918-dae4ebf48153",
    "James Bousema",
);

// HOB 279 — Thorin, Mountain-king (alternate printing)
const THORIN_MOUNTAIN_KING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THORIN_MOUNTAIN_KING,
    2,
    "0e29ba51-763c-48ec-95ec-9d6916f4db44",
    "James Bousema",
);

// HOB 280 — Bard, King of Dale (alternate printing)
const BARD_KING_OF_DALE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BARD_KING_OF_DALE,
    2,
    "f7c75c32-a24f-46e5-9128-2df9a2ec11f9",
    "James Bousema",
);

// HOB 281 — Smaug, Wicked Worm (alternate printing)
const SMAUG_WICKED_WORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_WICKED_WORM,
    2,
    "5091e2bd-247d-4a1d-adf4-6e66324ee20b",
    "Tyler Jacobson",
);

// HOB 282 — Thranduil, the Elvenking (alternate printing)
const THRANDUIL_THE_ELVENKING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_THE_ELVENKING,
    2,
    "49539856-989e-4f95-8e67-9cff4408cd87",
    "James Bousema",
);

// HOB 283 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    4,
    "9a0d62ce-8fd3-4465-b3aa-5d5052df30dc",
    "Tyler Jacobson",
);

// HOB 284 — The Lonely Mountain (alternate printing)
const THE_LONELY_MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_LONELY_MOUNTAIN,
    3,
    "edff761b-2c2a-414f-b0a3-25c3fdbcb0bc",
    "Tyler Jacobson",
);

// HOB 285 — Bilbo's Gambit (alternate printing)
const BILBO_S_GAMBIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BILBO_S_GAMBIT,
    1,
    "fa74ca8a-8bcd-4dc5-ab2b-a2e18a70978e",
    "Randy Gallegos",
);

// HOB 286 — Fíli the Pathfinder (alternate printing)
const FILI_THE_PATHFINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FILI_THE_PATHFINDER,
    1,
    "0e4a1096-c40e-4471-8a44-9c2acad85769",
    "Valera Lutfullina",
);

// HOB 287 — Kíli the Resourceful (alternate printing)
const KILI_THE_RESOURCEFUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KILI_THE_RESOURCEFUL,
    1,
    "c241a594-24bc-4fd9-ac6d-501d11dddad1",
    "Yuhong Ding",
);

// HOB 288 — Settle the Wreckage (alternate printing)
const SETTLE_THE_WRECKAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_xln::SETTLE_THE_WRECKAGE,
    1,
    "a132a86d-1df2-42d1-a410-2a8479c84a55",
    "Chris Cold",
);

// HOB 289 — An Unexpected Party // At the Door (alternate printing)
const AN_UNEXPECTED_PARTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AN_UNEXPECTED_PARTY,
    1,
    "f75207fd-89d2-417b-8f36-4ea96b4e3794",
    "Matt Stewart",
);

// HOB 290 — Elrond, Moon-Reader (alternate printing)
const ELROND_MOON_READER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELROND_MOON_READER,
    1,
    "3e95e9c2-cdbf-48b8-a7d0-87267f58e5ac",
    "Christina Kraus",
);

// HOB 291 — Great Gilded Boat (alternate printing)
const GREAT_GILDED_BOAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_GILDED_BOAT,
    1,
    "d2b59872-0e11-41c3-9858-3e2dd5a1c3c3",
    "Josu Solano",
);

// HOB 292 — Riddles in the Dark (alternate printing)
const RIDDLES_IN_THE_DARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIDDLES_IN_THE_DARK,
    1,
    "234e941d-ad86-4a3e-bd8c-0e680944919a",
    "Lorenzo Mastroianni",
);

// HOB 293 — Uncover the Moon-Letters (alternate printing)
const UNCOVER_THE_MOON_LETTERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNCOVER_THE_MOON_LETTERS,
    1,
    "1849eb57-7560-4ffc-9400-02387b1a71e6",
    "Leon Tukker",
);

// HOB 294 — Wizard's Staff (alternate printing)
const WIZARD_S_STAFF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WIZARD_S_STAFF,
    1,
    "14e69ca4-5148-44ec-bb21-330e656a6833",
    "Gaboleps",
);

// HOB 295 — Along the Crooked Way (alternate printing)
const ALONG_THE_CROOKED_WAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALONG_THE_CROOKED_WAY,
    1,
    "e340aa1c-a497-4fc4-9be9-26e7c982f893",
    "Bruce Brenneise",
);

// HOB 296 — Inside Information (alternate printing)
const INSIDE_INFORMATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSIDE_INFORMATION,
    1,
    "5913d004-0ef6-4dce-8f74-a8fbfc794d43",
    "Sean Vo",
);

// HOB 297 — The Master of Lake-town (alternate printing)
const THE_MASTER_OF_LAKE_TOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MASTER_OF_LAKE_TOWN,
    1,
    "d36d9c58-2bfe-418c-8512-fc4a3f229535",
    "Marius Bota",
);

// HOB 298 — Rhovanion Rampager (alternate printing)
const RHOVANION_RAMPAGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RHOVANION_RAMPAGER,
    1,
    "df7bdf15-9448-4cb0-a809-c5ac128e6b02",
    "Kevin Sidharta",
);

// HOB 299 — The Sackville-Bagginses (alternate printing)
const THE_SACKVILLE_BAGGINSES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SACKVILLE_BAGGINSES,
    1,
    "11144267-15d1-47b4-8c26-d57328e11422",
    "Denman Rooke",
);

// HOB 300 — Supper for Spiders (alternate printing)
const SUPPER_FOR_SPIDERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPPER_FOR_SPIDERS,
    1,
    "de7a0139-ce52-4212-9514-b2daba282288",
    "Michele Giorgi",
);

// HOB 301 — Balin, Loremaster (alternate printing)
const BALIN_LOREMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BALIN_LOREMASTER,
    1,
    "f3a8e4e3-58c5-4380-94b6-6252f8ccb285",
    "Colin Boyer",
);

// HOB 302 — Dáin Ironfoot (alternate printing)
const DAIN_IRONFOOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAIN_IRONFOOT,
    1,
    "ff35d11c-6429-4aab-b9c8-0c0306f13db9",
    "Tomas Duchek",
);

// HOB 303 — Getaway Barrel (alternate printing)
const GETAWAY_BARREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GETAWAY_BARREL,
    1,
    "8db47144-e048-4996-9f9a-1184dd86151f",
    "Pablo Mendoza",
);

// HOB 304 — Stone-Giant of High Pass (alternate printing)
const STONE_GIANT_OF_HIGH_PASS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STONE_GIANT_OF_HIGH_PASS,
    1,
    "1adfc347-5c69-45e1-829b-aaae19e04b96",
    "Miklós Ligeti",
);

// HOB 305 — Bejeweled Warg (alternate printing)
const BEJEWELED_WARG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEJEWELED_WARG,
    1,
    "2b689d91-1ac0-474b-ae58-2c320e1af5e4",
    "John Di Giovanni",
);

// HOB 306 — Cantankerous Keepers (alternate printing)
const CANTANKEROUS_KEEPERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CANTANKEROUS_KEEPERS,
    1,
    "ace0642f-fa57-4645-9891-43331b4963ed",
    "Ramza Psyru",
);

// HOB 307 — Gigantic Big Bear (alternate printing)
const GIGANTIC_BIG_BEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GIGANTIC_BIG_BEAR,
    1,
    "0b6ed366-24a3-4697-8a87-12c54da7c238",
    "Xabi Gaztelua",
);

// HOB 308 — Part in Friendship (alternate printing)
const PART_IN_FRIENDSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PART_IN_FRIENDSHIP,
    1,
    "46b3b094-0d1b-412e-8655-353ecbf983bb",
    "Jarel Threat",
);

// HOB 309 — Radagast of Rhosgobel (alternate printing)
const RADAGAST_OF_RHOSGOBEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RADAGAST_OF_RHOSGOBEL,
    1,
    "3ea01785-add0-487d-b708-5bbf71033899",
    "Anna Podedworna",
);

// HOB 310 — Through the Forest Gate (alternate printing)
const THROUGH_THE_FOREST_GATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THROUGH_THE_FOREST_GATE,
    1,
    "f4408b22-c2b0-46ff-8d10-81d150c992ca",
    "Leon Tukker",
);

// HOB 311 — Dwalin, Weaponmaster (alternate printing)
const DWALIN_WEAPONMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DWALIN_WEAPONMASTER,
    1,
    "6833cb81-a373-4b83-93b4-b1ed5f82ce54",
    "Marco Teixeira",
);

// HOB 312 — Tom, Bert, and William (alternate printing)
const TOM_BERT_AND_WILLIAM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOM_BERT_AND_WILLIAM,
    1,
    "3909bdc4-e690-47fa-8cbc-4b437f7ab10c",
    "Leonardo Borazio",
);

// HOB 313 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "c5b472d5-da70-4fd9-a68c-10e17797ad0d",
    "David Petersen",
);

// HOB 314 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "a3ae6956-7f2d-4569-a9f8-3972b5ec5e56",
    "David Petersen",
);

// HOB 315 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "8e07d3e6-c602-446f-a453-f54f13bfc55a",
    "David Petersen",
);

// HOB 316 — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    5,
    "15dd9a36-55df-4942-a7eb-1c49a7999e68",
    "David Petersen",
);

// HOB 317 — Plains (alternate printing)
const PLAINS_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    6,
    "3550f914-cacb-4849-af06-1a7b25c2c95f",
    "David Petersen",
);

// HOB 318 — Plains (alternate printing)
const PLAINS_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    7,
    "f82548c7-24ed-4529-8b57-7407dc0b8c8b",
    "David Petersen",
);

// HOB 319 — Plains (alternate printing)
const PLAINS_ALTERNATE_8: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    8,
    "46c75f8e-6590-4338-abbd-a37e08dbd58c",
    "David Petersen",
);

// HOB 320 — Plains (alternate printing)
const PLAINS_ALTERNATE_9: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    9,
    "128226f4-a733-4f0a-ab2b-7789cfa8b443",
    "David Petersen",
);

// HOB 321 — The Misty Mountains Cold (alternate printing)
const THE_MISTY_MOUNTAINS_COLD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MISTY_MOUNTAINS_COLD,
    1,
    "1aaf8a8a-1032-4ee5-9c18-2debb40ec561",
    "Rovina Cai",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LONG_BODIED_GREY_DOG,
    &OLD_THRUSH,
    &TROOP_OF_PONIES,
    &BELLADONNA_TOOK,
    &BILBO_S_GAMBIT,
    &BOFUR_RELIABLE_GUARDIAN,
    &CELEBRATE_THE_MOUNTAIN_KING,
    &DAIN_LORD_OF_THE_IRON_HILLS,
    &DWARVEN_PROVISIONER,
    &DWARVEN_SHORTSWORD,
    &EAGLE_OF_THE_GREAT_SHELF,
    &THE_EAGLES_ARE_COMING,
    &ESGAROTH_GARRISON,
    &FILI_THE_PATHFINDER,
    &GLEAMING_SPLENDOR,
    &IRON_HILLS_BLACKSMITH,
    &KILI_THE_RESOURCEFUL,
    &LAKE_TOWN_LOOKOUT,
    &LAKE_TOWN_TOYMAKER,
    &MAGNIFICENT_END,
    &MOMENT_OF_GLORY,
    &THE_MOUNTAIN_KING_S_RETURN,
    &ORI_KEEPER_OF_SONGS,
    &THE_QUEEN_OF_DALE,
    &ROADS_GO_EVER_EVER_ON,
    &STONE_BY_SUNLIGHT,
    &THORIN_S_LAST_STAND,
    &AN_UNEXPECTED_PARTY,
    &VELVETWING_BUTTERFLIES,
    &VOW_TO_EREBOR,
    &BILBO_LUCKWEARER,
    &BILBO_THIEF_IN_THE_NIGHT,
    &BILBO_BAGGINS_BURGLAR,
    &CONFUSTICATE_AND_BEBOTHER,
    &ELROND_MOON_READER,
    &ELVEN_RAFT_STEERER,
    &ELVENKING_S_HARPER,
    &ENCHANTED_RIVER_S_GRASP,
    &FATEFUL_DISCOVERY,
    &GANDALF_WANDERING_WIZARD,
    &GREAT_GILDED_BOAT,
    &LAKESHORE_APOTHECARY,
    &LAKE_TOWN_MARINERS,
    &LONG_LAKE_NUISANCE,
    &THE_LORD_OF_THE_EAGLES,
    &MASTER_S_COUNCILLORS,
    &MIRKWOOD_MEDITATOR,
    &MOST_DECREPIT_OLD_BIRD,
    &OLD_FAT_SPIDER_CAN_T_SEE_ME,
    &PLUNDER_THE_TROLLSHAWS,
    &RAVENHILL_FLOCK,
    &RIDDLES_IN_THE_DARK,
    &ROLL_ROLL_ROLL_ROLL,
    &SOUND_THE_TRUMPETS,
    &THRANDUIL_S_DECREE,
    &UNCOVER_THE_MOON_LETTERS,
    &UNEASY_PARTINGS,
    &WIZARD_S_STAFF,
    &ALONG_THE_CROOKED_WAY,
    &AZOG_MORIA_S_RUIN,
    &BILBO_S_DEADLY_SLICE,
    &CRUDE_BENT_BLADE,
    &DESOLATION_PROWLER,
    &DOWN_DOWN_TO_GOBLIN_TOWN,
    &DREADED_BAT_CLOUD,
    &FRONT_PORCH_SENTRIES,
    &GATHERING_OF_DARKNESS,
    &GNASHING_OF_TEETH,
    &GOLLUM_RIDDLE_MASTER,
    &GOLLUM_SILENT_SLINKER,
    &GOLLUM_THE_ABANDONED,
    &GREAT_FIERCE_BEE,
    &GREAT_UGLY_LOOKING_GOBLIN,
    &HEAD_OF_THE_HUNT,
    &INSIDE_INFORMATION,
    &THE_MASTER_OF_LAKE_TOWN,
    &NIGHTHOWL_PURSUER,
    &RAGE_INTO_THE_VALLEY,
    &RAVENING_WARG,
    &REVERENT_HOWL,
    &RHOVANION_RAMPAGER,
    &THE_SACKVILLE_BAGGINSES,
    &STIR_UP_TROUBLE,
    &STONY_VOICED_GOBLINS,
    &SUPPER_FOR_SPIDERS,
    &BALIN_LOREMASTER,
    &BOMBUR_GENTLE_DREAMER,
    &BOTHERSOME_NOISEMAKER,
    &BURN_BURN_TREE_AND_FERN,
    &DAIN_IRONFOOT,
    &DESERT_WERE_WORM,
    &DESOLATION_OF_SMAUG,
    &DORI_BEARER_OF_FRIENDS,
    &DWARVEN_MAULER,
    &GANDALF_GOBLINS_BANE,
    &GANDALF_SPARK_STARTER,
    &GETAWAY_BARREL,
    &GLOIN_THE_MIGHTY,
    &GOBLIN_TOWN_FLUNKIES,
    &GUNDABAD_OPPORTUNIST,
    &IRON_HILLS_STALWART,
    &LAST_LIGHT_OF_DURIN_S_DAY,
    &THE_MISTY_MOUNTAINS_COLD,
    &MISTY_MOUNTAINS_RAIDER,
    &OIN_THE_BRAVE,
    &PINECONE_STRIKE,
    &RAGGED_SHORT_SPEAR,
    &SMAUG_THE_GREAT_CALAMITY,
    &SMAUG_THE_MAGNIFICENT,
    &SMAUG_S_FURY,
    &SNOWSLOPE_HUNTER,
    &STONE_GIANT_OF_HIGH_PASS,
    &THORIN_MOUNTAIN_KING,
    &TIDINGS_OF_WAR,
    &ATTERCOP,
    &BEJEWELED_WARG,
    &BEORN_RELUCTANT_HOST,
    &BEORN_THE_FIERCE,
    &BEORN_S_HOSPITALITY,
    &BOUGHSIDE_WANDERERS,
    &CANTANKEROUS_KEEPERS,
    &DANCING_FROM_DARK_TO_DAWN,
    &DOWN_IN_THE_VALLEY,
    &GALION_ELVENKING_S_BUTLER,
    &GIGANTIC_BIG_BEAR,
    &GUARDIAN_OF_THE_HALLS,
    &LITTLE_BEAR,
    &MIRKWOOD_PATHMAKER,
    &NASTY_LITTLE_RABBIT,
    &THE_NOTARY_HOBBITS,
    &OLD_FAT_SPIDER,
    &ORDINARY_BEAR,
    &PART_IN_FRIENDSHIP,
    &QUARREL,
    &RADAGAST_OF_RHOSGOBEL,
    &THROUGH_THE_FOREST_GATE,
    &TROLL_NEGOTIATIONS,
    &WARG_TACTICS,
    &WARGLING,
    &WILDERLAND_SCROUNGER,
    &WOODLAND_WEAVEMASTER,
    &BARD_KING_OF_DALE,
    &BARD_THE_BOWMAN,
    &BARD_S_COMPANY,
    &BIFUR_MELODIC_RIDER,
    &BOLG_OF_THE_NORTH,
    &BOLG_S_COMPANY,
    &THE_CHIEF_WARG,
    &CHIEF_WARG_S_COMPANY,
    &DAIN_S_COMPANY,
    &DUSKWATCH_HUNTER,
    &DWALIN_WEAPONMASTER,
    &EAGLE_S_RESCUE,
    &FEARSOME_GOBLIN_PAIR,
    &GOBLIN_PLATE_MAIL,
    &THE_GREAT_GOBLIN,
    &LARGE_BEAR,
    &MIRKWOOD_NURTURER,
    &NORI_TELLER_OF_TALES,
    &PATIENT_INSTRUCTOR,
    &SILVAN_REVELER,
    &SMAUG_WICKED_WORM,
    &THORIN_OAKENSHIELD,
    &THRANDUIL_SINDARIN_LIEGE,
    &THRANDUIL_THE_ELVENKING,
    &THRANDUIL_S_COMPANY,
    &TOM_BERT_AND_WILLIAM,
    &THE_ARKENSTONE,
    &THE_BLACK_ARROW,
    &DWARVEN_MATTOCK,
    &GIANT_S_BOULDER,
    &GLAMDRING_FOE_HAMMER,
    &KEY_TO_THE_SIDE_DOOR,
    &MY_PRECIOUS,
    &ORCRIST_GOBLIN_CLEAVER,
    &STING_BILBO_S_SWORD,
    &THROR_S_MAP,
    &WELL_WORN_SPATULA,
    &ELVEN_PASSAGE,
    &ELVENKING_S_HALLS,
    &GOBLIN_TOWN,
    &HOBBIT_HOLE,
    &IRON_HILLS,
    &LAKE_TOWN,
    &THE_LONELY_MOUNTAIN,
    &MIRKWOOD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    SETTLE_THE_WRECKAGE_REPRINT,
    WOOD_ELVES_REPRINT,
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
    TROOP_OF_PONIES_ALTERNATE_1,
    RAGE_INTO_THE_VALLEY_ALTERNATE_1,
    THE_GREAT_GOBLIN_ALTERNATE_1,
    THORIN_OAKENSHIELD_ALTERNATE_1,
    GANDALF_SPARK_STARTER_ALTERNATE_1,
    GLAMDRING_FOE_HAMMER_ALTERNATE_1,
    THE_EAGLES_ARE_COMING_ALTERNATE_1,
    DREADED_BAT_CLOUD_ALTERNATE_1,
    THE_LONELY_MOUNTAIN_ALTERNATE_1,
    CHIEF_WARG_S_COMPANY_ALTERNATE_1,
    THORIN_S_LAST_STAND_ALTERNATE_1,
    BARD_S_COMPANY_ALTERNATE_1,
    BOLG_S_COMPANY_ALTERNATE_1,
    DAIN_S_COMPANY_ALTERNATE_1,
    THRANDUIL_S_COMPANY_ALTERNATE_1,
    BELLADONNA_TOOK_ALTERNATE_1,
    BOFUR_RELIABLE_GUARDIAN_ALTERNATE_1,
    IRON_HILLS_BLACKSMITH_ALTERNATE_1,
    THE_QUEEN_OF_DALE_ALTERNATE_1,
    BILBO_LUCKWEARER_ALTERNATE_1,
    BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_1,
    FATEFUL_DISCOVERY_ALTERNATE_1,
    MOST_DECREPIT_OLD_BIRD_ALTERNATE_1,
    AZOG_MORIA_S_RUIN_ALTERNATE_1,
    GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_1,
    HEAD_OF_THE_HUNT_ALTERNATE_1,
    DESERT_WERE_WORM_ALTERNATE_1,
    DESOLATION_OF_SMAUG_ALTERNATE_1,
    GLOIN_THE_MIGHTY_ALTERNATE_1,
    LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_1,
    SMAUG_THE_MAGNIFICENT_ALTERNATE_1,
    BEORN_THE_FIERCE_ALTERNATE_1,
    DANCING_FROM_DARK_TO_DAWN_ALTERNATE_1,
    THE_NOTARY_HOBBITS_ALTERNATE_1,
    THRANDUIL_SINDARIN_LIEGE_ALTERNATE_1,
    THE_ARKENSTONE_ALTERNATE_1,
    MY_PRECIOUS_ALTERNATE_1,
    ORCRIST_GOBLIN_CLEAVER_ALTERNATE_1,
    STING_BILBO_S_SWORD_ALTERNATE_1,
    ELVEN_PASSAGE_ALTERNATE_1,
    GLEAMING_SPLENDOR_ALTERNATE_1,
    THE_LORD_OF_THE_EAGLES_ALTERNATE_1,
    GOLLUM_RIDDLE_MASTER_ALTERNATE_1,
    GANDALF_GOBLINS_BANE_ALTERNATE_1,
    THORIN_MOUNTAIN_KING_ALTERNATE_1,
    BARD_KING_OF_DALE_ALTERNATE_1,
    SMAUG_WICKED_WORM_ALTERNATE_1,
    THRANDUIL_THE_ELVENKING_ALTERNATE_1,
    THE_ARKENSTONE_ALTERNATE_2,
    THE_LONELY_MOUNTAIN_ALTERNATE_2,
    SMAUG_THE_MAGNIFICENT_ALTERNATE_2,
    BELLADONNA_TOOK_ALTERNATE_2,
    BOFUR_RELIABLE_GUARDIAN_ALTERNATE_2,
    IRON_HILLS_BLACKSMITH_ALTERNATE_2,
    THE_QUEEN_OF_DALE_ALTERNATE_2,
    BILBO_LUCKWEARER_ALTERNATE_2,
    BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_2,
    FATEFUL_DISCOVERY_ALTERNATE_2,
    MOST_DECREPIT_OLD_BIRD_ALTERNATE_2,
    AZOG_MORIA_S_RUIN_ALTERNATE_2,
    GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_2,
    HEAD_OF_THE_HUNT_ALTERNATE_2,
    DESERT_WERE_WORM_ALTERNATE_2,
    DESOLATION_OF_SMAUG_ALTERNATE_2,
    GLOIN_THE_MIGHTY_ALTERNATE_2,
    LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_2,
    SMAUG_THE_MAGNIFICENT_ALTERNATE_3,
    BEORN_THE_FIERCE_ALTERNATE_2,
    DANCING_FROM_DARK_TO_DAWN_ALTERNATE_2,
    THE_NOTARY_HOBBITS_ALTERNATE_2,
    THRANDUIL_SINDARIN_LIEGE_ALTERNATE_2,
    THE_ARKENSTONE_ALTERNATE_3,
    MY_PRECIOUS_ALTERNATE_2,
    ORCRIST_GOBLIN_CLEAVER_ALTERNATE_2,
    STING_BILBO_S_SWORD_ALTERNATE_2,
    ELVEN_PASSAGE_ALTERNATE_2,
    GLEAMING_SPLENDOR_ALTERNATE_2,
    THE_LORD_OF_THE_EAGLES_ALTERNATE_2,
    GOLLUM_RIDDLE_MASTER_ALTERNATE_2,
    GANDALF_GOBLINS_BANE_ALTERNATE_2,
    THORIN_MOUNTAIN_KING_ALTERNATE_2,
    BARD_KING_OF_DALE_ALTERNATE_2,
    SMAUG_WICKED_WORM_ALTERNATE_2,
    THRANDUIL_THE_ELVENKING_ALTERNATE_2,
    THE_ARKENSTONE_ALTERNATE_4,
    THE_LONELY_MOUNTAIN_ALTERNATE_3,
    BILBO_S_GAMBIT_ALTERNATE_1,
    FILI_THE_PATHFINDER_ALTERNATE_1,
    KILI_THE_RESOURCEFUL_ALTERNATE_1,
    SETTLE_THE_WRECKAGE_ALTERNATE_1,
    AN_UNEXPECTED_PARTY_ALTERNATE_1,
    ELROND_MOON_READER_ALTERNATE_1,
    GREAT_GILDED_BOAT_ALTERNATE_1,
    RIDDLES_IN_THE_DARK_ALTERNATE_1,
    UNCOVER_THE_MOON_LETTERS_ALTERNATE_1,
    WIZARD_S_STAFF_ALTERNATE_1,
    ALONG_THE_CROOKED_WAY_ALTERNATE_1,
    INSIDE_INFORMATION_ALTERNATE_1,
    THE_MASTER_OF_LAKE_TOWN_ALTERNATE_1,
    RHOVANION_RAMPAGER_ALTERNATE_1,
    THE_SACKVILLE_BAGGINSES_ALTERNATE_1,
    SUPPER_FOR_SPIDERS_ALTERNATE_1,
    BALIN_LOREMASTER_ALTERNATE_1,
    DAIN_IRONFOOT_ALTERNATE_1,
    GETAWAY_BARREL_ALTERNATE_1,
    STONE_GIANT_OF_HIGH_PASS_ALTERNATE_1,
    BEJEWELED_WARG_ALTERNATE_1,
    CANTANKEROUS_KEEPERS_ALTERNATE_1,
    GIGANTIC_BIG_BEAR_ALTERNATE_1,
    PART_IN_FRIENDSHIP_ALTERNATE_1,
    RADAGAST_OF_RHOSGOBEL_ALTERNATE_1,
    THROUGH_THE_FOREST_GATE_ALTERNATE_1,
    DWALIN_WEAPONMASTER_ALTERNATE_1,
    TOM_BERT_AND_WILLIAM_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    PLAINS_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    PLAINS_ALTERNATE_6,
    PLAINS_ALTERNATE_7,
    PLAINS_ALTERNATE_8,
    PLAINS_ALTERNATE_9,
    THE_MISTY_MOUNTAINS_COLD_ALTERNATE_1,
];
