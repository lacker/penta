//! Seventh Edition has no unique card definitions.
//!
//! It is the last core set inside the Premodern window, so a card printed
//! only in a Portal set before it becomes legal here.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::alpha;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1993::arabian_nights;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1994::fallen_empires as catalog_fem;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1994::the_dark as catalog_drk;
use crate::card::sets::y1995::homelands as catalog_hml;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1996::alliances as catalog_all;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::tempest as catalog_tmp;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::sets::y1998::exodus as catalog_exo;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y1998::portal_second_age;
use crate::card::sets::y1998::stronghold as catalog_sth;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y1999::mercadian_masques as catalog_mmq;
use crate::card::sets::y1999::urzas_destiny as catalog_uds;
use crate::card::sets::y1999::urzas_legacy as catalog_ulg;
use crate::card::sets::y2000::invasion as catalog_inv;
use crate::card::sets::y2000::nemesis as catalog_nem;

// 7ED 1 — Angelic Page (reprint)
const ANGELIC_PAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::ANGELIC_PAGE,
    "b3aebe60-bbd4-4591-bf3c-6cec67e41cf6",
    "Marc Fishman",
);

// 7ED 1★ — Angelic Page (alternate printing)
const ANGELIC_PAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::ANGELIC_PAGE,
    1,
    "a610860b-cc2f-4f7e-99f4-481d15c7cd90",
    "Marc Fishman",
);

// 7ED 2 — Ardent Militia (reprint)
const ARDENT_MILITIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ARDENT_MILITIA,
    "c67252f6-0265-4f4e-8db8-2d1853262f91",
    "Paolo Parente",
);

// 7ED 2★ — Ardent Militia (alternate printing)
const ARDENT_MILITIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::ARDENT_MILITIA,
    1,
    "34b57b5e-134e-45a7-92d6-670d762d68b4",
    "Paolo Parente",
);

// 7ED 3 — Blessed Reversal (reprint)
const BLESSED_REVERSAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BLESSED_REVERSAL,
    "06a1d373-f619-4855-9154-aee6deeacb59",
    "Christopher Moeller",
);

// 7ED 3★ — Blessed Reversal (alternate printing)
const BLESSED_REVERSAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::BLESSED_REVERSAL,
    1,
    "d1aaa618-ebe9-43ff-90dd-7ab1429f2c58",
    "Christopher Moeller",
);

// 7ED 4 — Breath of Life (reprint)
const BREATH_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BREATH_OF_LIFE,
    "fee1bad3-85e0-4c65-916c-d744e6e6ec61",
    "Roger Raupp",
);

// 7ED 4★ — Breath of Life (alternate printing)
const BREATH_OF_LIFE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::BREATH_OF_LIFE,
    1,
    "9d7a985b-d57a-4409-9ec4-fabd31afc461",
    "Roger Raupp",
);

// 7ED 5 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "ffe82e29-e781-48bf-8673-7954553f7cf0",
    "Edward P. Beard, Jr.",
);

// 7ED 5★ — Castle (alternate printing)
const CASTLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::CASTLE,
    1,
    "5c73efe3-e3ab-4161-88cc-82d3ac7d6a4a",
    "Edward P. Beard, Jr.",
);

// 7ED 6 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "b7967ce4-6688-40d0-bedb-4e0f38502f09",
    "Mark Romanoski",
);

// 7ED 6★ — Circle of Protection: Black (alternate printing)
const CIRCLE_OF_PROTECTION_BLACK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    1,
    "f40c8da0-0201-41ab-be98-8ed3a899cd3d",
    "Mark Romanoski",
);

// 7ED 7 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "babb5c99-6cb1-4b13-8aac-e495d508a61c",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 7★ — Circle of Protection: Blue (alternate printing)
const CIRCLE_OF_PROTECTION_BLUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    1,
    "68cb254e-137f-4920-9e2b-d4fbe7861101",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 8 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "f7e0b146-4241-436b-b8b0-45a9be21ead7",
    "Alan Pollack",
);

// 7ED 8★ — Circle of Protection: Green (alternate printing)
const CIRCLE_OF_PROTECTION_GREEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    1,
    "5bf8b0e9-a420-4066-b423-90eec1f27eb0",
    "Alan Pollack",
);

// 7ED 9 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "9b4833ce-5e7b-402e-97a9-4411aa62a46c",
    "Gary Ruddell",
);

// 7ED 9★ — Circle of Protection: Red (alternate printing)
const CIRCLE_OF_PROTECTION_RED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    1,
    "6a65a2a3-ee40-4a00-aa66-f64b04ccdaa8",
    "Gary Ruddell",
);

// 7ED 10 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "b5cab871-5783-40e1-a065-0607a842998b",
    "Darrell Riche",
);

// 7ED 10★ — Circle of Protection: White (alternate printing)
const CIRCLE_OF_PROTECTION_WHITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    1,
    "d5d55297-9049-4627-8154-bafdef63c66d",
    "Darrell Riche",
);

// 7ED 11 — Cloudchaser Eagle (reprint)
const CLOUDCHASER_EAGLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::CLOUDCHASER_EAGLE,
    "81cd5854-56ef-48ec-ad12-1690fa45b4a5",
    "Aaron Boyd",
);

// 7ED 11★ — Cloudchaser Eagle (alternate printing)
const CLOUDCHASER_EAGLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::CLOUDCHASER_EAGLE,
    1,
    "8c95d068-e153-4ac9-bce6-c79ef4fe101d",
    "Aaron Boyd",
);

// 7ED 12 — Crossbow Infantry (reprint)
const CROSSBOW_INFANTRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mmq::CROSSBOW_INFANTRY,
    "d61cc896-1da5-419c-87db-01321522c40b",
    "James Bernardin",
);

// 7ED 12★ — Crossbow Infantry (alternate printing)
const CROSSBOW_INFANTRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mmq::CROSSBOW_INFANTRY,
    1,
    "bedaa217-8dbd-4d2c-961a-a7cff2398832",
    "James Bernardin",
);

// 7ED 13 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DISENCHANT,
    "402f71fc-807c-4718-956b-7ffe66c646d4",
    "Andrew Goldhawk",
);

// 7ED 13★ — Disenchant (alternate printing)
const DISENCHANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DISENCHANT,
    1,
    "4715aacd-ca2f-4dbf-b1a1-c171dea7305f",
    "Andrew Goldhawk",
);

// 7ED 14 — Eager Cadet (reprint)
const EAGER_CADET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::EAGER_CADET,
    "46b89ce6-8a73-4e27-8696-e65ea0c16925",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 14★ — Eager Cadet (alternate printing)
const EAGER_CADET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::starter_1999::EAGER_CADET,
    1,
    "d8b718ad-1ca3-4587-994d-6a9df47db1e0",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 15 — Elite Archers (reprint)
const ELITE_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::ELITE_ARCHERS,
    "03dc7a7b-ec69-406f-82c5-9af54ac1e9a3",
    "Dan Frazier",
);

// 7ED 15★ — Elite Archers (alternate printing)
const ELITE_ARCHERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::ELITE_ARCHERS,
    1,
    "269193a9-2647-4b7a-a51b-6e2eae160020",
    "Dan Frazier",
);

// 7ED 16 — Gerrard's Wisdom (reprint)
const GERRARD_S_WISDOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::GERRARD_S_WISDOM,
    "e43378f0-b518-499e-9950-c583bc2f11f8",
    "Donato Giancola",
);

// 7ED 16★ — Gerrard's Wisdom (alternate printing)
const GERRARD_S_WISDOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::GERRARD_S_WISDOM,
    1,
    "71618fb4-22ab-4ef5-932d-69bf596a69d0",
    "Donato Giancola",
);

// 7ED 17 — Glorious Anthem (reprint)
const GLORIOUS_ANTHEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::GLORIOUS_ANTHEM,
    "e6f18de6-a99b-49ce-812b-bca8b0aaec38",
    "Terese Nielsen",
);

// 7ED 17★ — Glorious Anthem (alternate printing)
const GLORIOUS_ANTHEM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::GLORIOUS_ANTHEM,
    1,
    "607ddd6d-db70-4db0-a3ac-caaa911c8fe4",
    "Terese Nielsen",
);

// 7ED 18 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "9670a69a-bd68-4de9-bf83-c912564012f6",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 18★ — Healing Salve (alternate printing)
const HEALING_SALVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::HEALING_SALVE,
    1,
    "9eff9691-f3e0-4622-ae8f-b884763bbcd6",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 19 — Heavy Ballista (reprint)
const HEAVY_BALLISTA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::HEAVY_BALLISTA,
    "8b665186-7ee5-47dd-b849-cb9c318f31e6",
    "Brian Snõddy",
);

// 7ED 19★ — Heavy Ballista (alternate printing)
const HEAVY_BALLISTA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::HEAVY_BALLISTA,
    1,
    "5b6e66f3-8a2e-4b68-b483-7cb6923bd9f9",
    "Brian Snõddy",
);

// 7ED 20 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "cdabca37-9099-41ca-a169-33332c82d76f",
    "Scott M. Fischer",
);

// 7ED 20★ — Holy Strength (alternate printing)
const HOLY_STRENGTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::HOLY_STRENGTH,
    1,
    "ce20ca0f-8747-43a6-9723-bca7669badb7",
    "Scott M. Fischer",
);

// 7ED 21 — Honor Guard (reprint)
const HONOR_GUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::HONOR_GUARD,
    "aff788a3-3182-4bf5-897e-521d977e6aaf",
    "Mark Zug",
);

// 7ED 21★ — Honor Guard (alternate printing)
const HONOR_GUARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::HONOR_GUARD,
    1,
    "ef3725d9-b58a-4632-b542-26fa94b3ba6b",
    "Mark Zug",
);

// 7ED 22 — Intrepid Hero (reprint)
const INTREPID_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::INTREPID_HERO,
    "49430d37-03af-4088-a92b-9e0ff3defe29",
    "Mike Ploog",
);

// 7ED 22★ — Intrepid Hero (alternate printing)
const INTREPID_HERO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::INTREPID_HERO,
    1,
    "f544e171-c26b-4c14-96ff-3d3ddce7e785",
    "Mike Ploog",
);

// 7ED 23 — Kjeldoran Royal Guard (reprint)
const KJELDORAN_ROYAL_GUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KJELDORAN_ROYAL_GUARD,
    "2da50eb5-e559-4d67-8568-d5be80ff62de",
    "Carl Critchlow",
);

// 7ED 23★ — Kjeldoran Royal Guard (alternate printing)
const KJELDORAN_ROYAL_GUARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::KJELDORAN_ROYAL_GUARD,
    1,
    "fb58da0d-4f11-4973-ac9c-d541031ff728",
    "Carl Critchlow",
);

// 7ED 24 — Knight Errant (alternate printing)
const KNIGHT_ERRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::KNIGHT_ERRANT,
    1,
    "d8d1d55b-6be0-4bb5-b452-ba4994b21774",
    "Matthew D. Wilson",
);

// 7ED 24★ — Knight Errant (reprint)
const KNIGHT_ERRANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::KNIGHT_ERRANT,
    "413f10fe-0e53-46ca-bd64-0d66dee8882d",
    "Matthew D. Wilson",
);

// 7ED 25 — Knighthood (reprint)
const KNIGHTHOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::KNIGHTHOOD,
    "1da88c0a-255d-4484-b464-bd742d66e3fe",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 25★ — Knighthood (alternate printing)
const KNIGHTHOOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::KNIGHTHOOD,
    1,
    "2e8e367b-329f-4ee6-be1e-6263ccc0f06d",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 26 — Longbow Archer (reprint)
const LONGBOW_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::LONGBOW_ARCHER,
    "8ac3672e-ebab-4bb1-bf4d-5020047296d8",
    "Ray Lago",
);

// 7ED 26★ — Longbow Archer (alternate printing)
const LONGBOW_ARCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::LONGBOW_ARCHER,
    1,
    "7a0437fd-070a-4679-b1c9-6cb22820d181",
    "Ray Lago",
);

// 7ED 27 — Master Healer (reprint)
const MASTER_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::MASTER_HEALER,
    "6cfdc12c-fd1f-4053-8a1a-63f440ef0102",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 27★ — Master Healer (alternate printing)
const MASTER_HEALER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::MASTER_HEALER,
    1,
    "5e5be6df-f35e-4696-a08e-fd725ee9b13c",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 28 — Northern Paladin (reprint)
const NORTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NORTHERN_PALADIN,
    "5ed1fb50-b7c3-43e9-a0ef-a33135a12300",
    "Carl Critchlow",
);

// 7ED 28★ — Northern Paladin (alternate printing)
const NORTHERN_PALADIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::NORTHERN_PALADIN,
    1,
    "5a5cce5e-3af9-4f51-8d65-35be576df236",
    "Carl Critchlow",
);

// 7ED 29 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "ee6758f0-86da-4812-bbe0-ebbb8c67937a",
    "Eric Peterson",
);

// 7ED 29★ — Pacifism (alternate printing)
const PACIFISM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1996::mirage::PACIFISM,
    1,
    "f3524e49-93df-4a3d-808e-7a2d31c3a12d",
    "Eric Peterson",
);

// 7ED 30 — Pariah (reprint)
const PARIAH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::PARIAH,
    "079b8e9b-74ac-48c0-8c65-82d70911dd9e",
    "Scott M. Fischer",
);

// 7ED 30★ — Pariah (alternate printing)
const PARIAH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::PARIAH,
    1,
    "1a94805d-9f71-4968-a297-7d93859fe79f",
    "Scott M. Fischer",
);

// 7ED 31 — Purify (reprint)
const PURIFY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::PURIFY,
    "0cfb77ea-6776-4bb4-886d-9a60f56e1fa7",
    "Doug Chaffee",
);

// 7ED 31★ — Purify (alternate printing)
const PURIFY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::PURIFY,
    1,
    "b8f84bff-a9e7-4bc4-9dc9-b5eb34166d40",
    "Doug Chaffee",
);

// 7ED 32 — Razorfoot Griffin (reprint)
const RAZORFOOT_GRIFFIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_inv::RAZORFOOT_GRIFFIN,
    "cc73761c-6d1a-4466-9be1-b350a1dcc48a",
    "Pete Venters",
);

// 7ED 32★ — Razorfoot Griffin (alternate printing)
const RAZORFOOT_GRIFFIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_inv::RAZORFOOT_GRIFFIN,
    1,
    "d3131a2d-05b3-494c-9373-e1a7913615f9",
    "Pete Venters",
);

// 7ED 33 — Reprisal (reprint)
const REPRISAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::REPRISAL,
    "3868f7ff-8a84-4153-bf5a-ff001d34e0f0",
    "Ciruelo",
);

// 7ED 33★ — Reprisal (alternate printing)
const REPRISAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_all::REPRISAL,
    1,
    "3f73b0f4-cf49-4ae3-8117-0e2e37c7ab31",
    "Ciruelo",
);

// 7ED 34 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "af458eaa-ee16-4b2b-9a55-259458357224",
    "Eric Peterson",
);

// 7ED 34★ — Reverse Damage (alternate printing)
const REVERSE_DAMAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::REVERSE_DAMAGE,
    1,
    "1867777d-000d-48b9-99da-1c2194722dde",
    "Eric Peterson",
);

// 7ED 35 — Rolling Stones (reprint)
const ROLLING_STONES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::ROLLING_STONES,
    "9993e9c6-2e30-4bf4-838d-751cbd153390",
    "Don Hazeltine",
);

// 7ED 35★ — Rolling Stones (alternate printing)
const ROLLING_STONES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::ROLLING_STONES,
    1,
    "c66582c6-2eaa-43a8-a0cd-43dab0abc656",
    "Don Hazeltine",
);

// 7ED 36 — Sacred Ground (reprint)
const SACRED_GROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::SACRED_GROUND,
    "d43ccab9-c059-4f25-b27c-3f2d506251bd",
    "Gary Ruddell",
);

// 7ED 36★ — Sacred Ground (alternate printing)
const SACRED_GROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::SACRED_GROUND,
    1,
    "d83d48a7-c6d3-4aa9-8416-88eece34a158",
    "Gary Ruddell",
);

// 7ED 37 — Sacred Nectar (alternate printing)
const SACRED_NECTAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::SACRED_NECTAR,
    1,
    "f298a56d-8a33-46b1-aca0-acb68cdb3e29",
    "Dana Knutson",
);

// 7ED 37★ — Sacred Nectar (reprint)
const SACRED_NECTAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::SACRED_NECTAR,
    "8d4b8de0-0bb5-40fb-8b73-d00d38a582d5",
    "Dana Knutson",
);

// 7ED 38 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "19d997ce-6b08-4058-a7f8-82cc74b9974d",
    "Anson Maddocks",
);

// 7ED 38★ — Samite Healer (alternate printing)
const SAMITE_HEALER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SAMITE_HEALER,
    1,
    "a23ff360-3f4c-40e7-acb0-273cb35a903d",
    "Anson Maddocks",
);

// 7ED 39 — Sanctimony (reprint)
const SANCTIMONY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::SANCTIMONY,
    "a51f15d4-e0e9-416b-9fe8-66c86160ff1a",
    "Patrick Faricy",
);

// 7ED 39★ — Sanctimony (alternate printing)
const SANCTIMONY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::SANCTIMONY,
    1,
    "273a8178-bae8-4668-9827-abcf9485554e",
    "Patrick Faricy",
);

// 7ED 40 — Seasoned Marshal (reprint)
const SEASONED_MARSHAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::SEASONED_MARSHAL,
    "c40221db-f36f-40e4-9029-5c18422cc172",
    "Edward P. Beard, Jr.",
);

// 7ED 40★ — Seasoned Marshal (alternate printing)
const SEASONED_MARSHAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::SEASONED_MARSHAL,
    1,
    "6a96600d-9592-4bf7-aea2-5ebf0660b915",
    "Edward P. Beard, Jr.",
);

// 7ED 41 — Serra Advocate (reprint)
const SERRA_ADVOCATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::SERRA_ADVOCATE,
    "a411716d-aff1-4b8f-961d-a873707b9f2a",
    "Matthew D. Wilson",
);

// 7ED 41★ — Serra Advocate (alternate printing)
const SERRA_ADVOCATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::SERRA_ADVOCATE,
    1,
    "1f05d654-f3d1-4965-9d89-9848d6fc2123",
    "Matthew D. Wilson",
);

// 7ED 42 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SERRA_ANGEL,
    "b7b4e357-de48-4461-8109-bbb07fff5171",
    "Mark Zug",
);

// 7ED 42★ — Serra Angel (alternate printing)
const SERRA_ANGEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SERRA_ANGEL,
    1,
    "3a398d30-2db9-4530-9f68-fb4dd6ebcbd2",
    "Mark Zug",
);

// 7ED 43 — Serra's Embrace (reprint)
const SERRA_S_EMBRACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::SERRA_S_EMBRACE,
    "ff41d966-5c11-4732-99e1-ada41a6020a7",
    "Ciruelo",
);

// 7ED 43★ — Serra's Embrace (alternate printing)
const SERRA_S_EMBRACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::SERRA_S_EMBRACE,
    1,
    "06573466-dd60-4cab-acdc-9287daf232cf",
    "Ciruelo",
);

// 7ED 44 — Shield Wall (reprint)
const SHIELD_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SHIELD_WALL,
    "d4b70c30-dbc9-4d30-81d8-b0bde9b626df",
    "Adam Rex",
);

// 7ED 44★ — Shield Wall (alternate printing)
const SHIELD_WALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::SHIELD_WALL,
    1,
    "5eb49655-ba5c-4a62-881d-669593a398c8",
    "Adam Rex",
);

// 7ED 45 — Skyshroud Falcon (reprint)
const SKYSHROUD_FALCON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::SKYSHROUD_FALCON,
    "a41aec1d-d86f-4a52-a446-5cef71d1ebd4",
    "Anson Maddocks",
);

// 7ED 45★ — Skyshroud Falcon (alternate printing)
const SKYSHROUD_FALCON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::SKYSHROUD_FALCON,
    1,
    "4f46ee43-9706-41cf-b413-3b5fec762194",
    "Anson Maddocks",
);

// 7ED 46 — Southern Paladin (reprint)
const SOUTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::SOUTHERN_PALADIN,
    "b2fa1570-3103-494c-8316-8f0bf484f22d",
    "Greg Staples",
);

// 7ED 46★ — Southern Paladin (alternate printing)
const SOUTHERN_PALADIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::SOUTHERN_PALADIN,
    1,
    "17a98f74-1b59-47ce-b5ab-dda5aefd9f78",
    "Greg Staples",
);

// 7ED 47 — Spirit Link (reprint)
const SPIRIT_LINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SPIRIT_LINK,
    "b7f92671-5023-4cf1-bc06-58e8c094c6a2",
    "Daren Bader",
);

// 7ED 47★ — Spirit Link (alternate printing)
const SPIRIT_LINK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::SPIRIT_LINK,
    1,
    "d3a6a4db-045d-48f7-8f82-4486231d755a",
    "Daren Bader",
);

// 7ED 48 — Standing Troops (reprint)
const STANDING_TROOPS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::STANDING_TROOPS,
    "8ccb299f-5601-4852-9fea-3c375b4851e8",
    "Gary Ruddell",
);

// 7ED 48★ — Standing Troops (alternate printing)
const STANDING_TROOPS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::STANDING_TROOPS,
    1,
    "71123df7-1a4d-47cd-a72e-44c431860e88",
    "Gary Ruddell",
);

// 7ED 49 — Starlight (alternate printing)
const STARLIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::STARLIGHT,
    1,
    "9f9ef338-1a74-433c-a7d9-f667246e6622",
    "Brian Despain",
);

// 7ED 49★ — Starlight (reprint)
const STARLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::STARLIGHT,
    "413c5a7e-e19d-4cbd-9279-88391b75c6c5",
    "Brian Despain",
);

// 7ED 50 — Staunch Defenders (reprint)
const STAUNCH_DEFENDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::STAUNCH_DEFENDERS,
    "2e5c5acf-efcb-4df9-b956-b7bce336a5cb",
    "Tristan Elwell",
);

// 7ED 50★ — Staunch Defenders (alternate printing)
const STAUNCH_DEFENDERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::STAUNCH_DEFENDERS,
    1,
    "fc642f16-049f-499b-a300-5f5a7372b98f",
    "Tristan Elwell",
);

// 7ED 51 — Sunweb (reprint)
const SUNWEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::SUNWEB,
    "8466271c-0d9b-4680-a856-efabc7dbc1ef",
    "Greg Staples",
);

// 7ED 51★ — Sunweb (alternate printing)
const SUNWEB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::SUNWEB,
    1,
    "e6fdb8e2-76d6-4471-8fe3-c83872f0c198",
    "Greg Staples",
);

// 7ED 52 — Sustainer of the Realm (reprint)
const SUSTAINER_OF_THE_REALM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::SUSTAINER_OF_THE_REALM,
    "ddb7af98-1fba-488b-9e92-2f6a35eb4866",
    "Mark Zug",
);

// 7ED 52★ — Sustainer of the Realm (alternate printing)
const SUSTAINER_OF_THE_REALM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::SUSTAINER_OF_THE_REALM,
    1,
    "286955b5-c866-44f4-a76b-54632192918d",
    "Mark Zug",
);

// 7ED 53 — Venerable Monk (reprint)
const VENERABLE_MONK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::VENERABLE_MONK,
    "3b59eeaf-5629-4216-8d97-1dbd1927b8fe",
    "Mark Brill",
);

// 7ED 53★ — Venerable Monk (alternate printing)
const VENERABLE_MONK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::VENERABLE_MONK,
    1,
    "4ac3fb9a-f4c8-421b-994f-418bb340e84b",
    "Mark Brill",
);

// 7ED 54 — Vengeance (reprint)
const VENGEANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::VENGEANCE,
    "011b9836-fee4-4e83-add7-5e13cb1275d6",
    "Paolo Parente",
);

// 7ED 54★ — Vengeance (alternate printing)
const VENGEANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::VENGEANCE,
    1,
    "ab0e4d5a-079c-429f-b4f7-aa12fded1dce",
    "Paolo Parente",
);

// 7ED 55 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "7ade80fd-813e-4823-a7ac-2989c440a4d8",
    "Hannibal King",
);

// 7ED 55★ — Wall of Swords (alternate printing)
const WALL_OF_SWORDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::WALL_OF_SWORDS,
    1,
    "3df722ab-3302-4158-a9fb-f0f9608f0cec",
    "Hannibal King",
);

// 7ED 56 — Worship (reprint)
const WORSHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::WORSHIP,
    "5a7cacaf-6dd9-4ed3-b589-30854a501021",
    "rk post",
);

// 7ED 56★ — Worship (alternate printing)
const WORSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::WORSHIP,
    1,
    "2c3283f4-1228-4fb4-9b97-8a9a90929cb2",
    "rk post",
);

// 7ED 57 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WRATH_OF_GOD,
    "0d223e83-0d3c-459e-96f5-ba9227fe49dd",
    "Kev Walker",
);

// 7ED 57★ — Wrath of God (alternate printing)
const WRATH_OF_GOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::WRATH_OF_GOD,
    1,
    "5bcc5ed6-41ff-4f8a-b47a-fc9390c2aa81",
    "Kev Walker",
);

// 7ED 58 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "e0331818-208c-460a-b15d-81c8fd54669d",
    "Wayne England",
);

// 7ED 58★ — Air Elemental (alternate printing)
const AIR_ELEMENTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::AIR_ELEMENTAL,
    1,
    "9004140e-0369-44ae-84eb-5208a7ef4ced",
    "Wayne England",
);

// 7ED 59 — Ancestral Memories (reprint)
const ANCESTRAL_MEMORIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ANCESTRAL_MEMORIES,
    "100f0ca8-a66a-452f-be5f-17f631ba0ee0",
    "Rebecca Guay",
);

// 7ED 59★ — Ancestral Memories (alternate printing)
const ANCESTRAL_MEMORIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::ANCESTRAL_MEMORIES,
    1,
    "edfe6027-1e97-45b1-949b-73e6cf25bc82",
    "Rebecca Guay",
);

// 7ED 60 — Arcane Laboratory (reprint)
const ARCANE_LABORATORY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::ARCANE_LABORATORY,
    "f9e3bcd7-60f8-472a-ada0-c3147cf06588",
    "Brian Snõddy",
);

// 7ED 60★ — Arcane Laboratory (alternate printing)
const ARCANE_LABORATORY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::ARCANE_LABORATORY,
    1,
    "783435f7-5fb1-49bb-98a3-b8da8f76ce25",
    "Brian Snõddy",
);

// 7ED 61 — Archivist (reprint)
const ARCHIVIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::ARCHIVIST,
    "8e44374d-b327-4193-ad3b-628191461d05",
    "Donato Giancola",
);

// 7ED 61★ — Archivist (alternate printing)
const ARCHIVIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::ARCHIVIST,
    1,
    "f8067421-8481-449e-9c97-6aa1c4610c71",
    "Donato Giancola",
);

// 7ED 62 — Baleful Stare (reprint)
const BALEFUL_STARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BALEFUL_STARE,
    "7c53b808-c2c5-4941-bead-1cb94adc5a2f",
    "Eric Peterson",
);

// 7ED 62★ — Baleful Stare (alternate printing)
const BALEFUL_STARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::BALEFUL_STARE,
    1,
    "90f3896d-f54f-44fc-a619-ea3a71a513c0",
    "Eric Peterson",
);

// 7ED 63 — Benthic Behemoth (reprint)
const BENTHIC_BEHEMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::BENTHIC_BEHEMOTH,
    "19577bda-2728-40c8-a262-26051e6c226b",
    "Heather Hudson",
);

// 7ED 63★ — Benthic Behemoth (alternate printing)
const BENTHIC_BEHEMOTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::BENTHIC_BEHEMOTH,
    1,
    "acb24340-0c10-485c-a9a8-3ac65d494a57",
    "Heather Hudson",
);

// 7ED 64 — Boomerang (reprint)
const BOOMERANG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BOOMERANG,
    "ee7e31b5-fe27-44ad-a1d8-7263c3edbc7d",
    "Rebecca Guay",
);

// 7ED 64★ — Boomerang (alternate printing)
const BOOMERANG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::BOOMERANG,
    1,
    "724b7896-74d0-491a-8aa6-183324660634",
    "Rebecca Guay",
);

// 7ED 65 — Confiscate (reprint)
const CONFISCATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::CONFISCATE,
    "d8440565-8359-4283-aad1-6b594a5a96eb",
    "Christopher Moeller",
);

// 7ED 65★ — Confiscate (alternate printing)
const CONFISCATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::CONFISCATE,
    1,
    "892f0980-eb6b-4454-88b8-e2e79983d065",
    "Christopher Moeller",
);

// 7ED 66 — Coral Merfolk (reprint)
const CORAL_MERFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::CORAL_MERFOLK,
    "80ea1af2-f944-4ee1-88bd-2f3ea09bf9e6",
    "Rebecca Guay",
);

// 7ED 66★ — Coral Merfolk (alternate printing)
const CORAL_MERFOLK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1998::urzas_saga::CORAL_MERFOLK,
    1,
    "98cb3579-1a75-4ff4-a007-5d65969b887c",
    "Rebecca Guay",
);

// 7ED 67 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "29bb1b85-9444-4bfa-b622-092a6873631c",
    "Mark Romanoski",
);

// 7ED 67★ — Counterspell (alternate printing)
const COUNTERSPELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::COUNTERSPELL,
    1,
    "8bed211e-f3ec-4e9e-b9a7-0989930dd049",
    "Mark Romanoski",
);

// 7ED 68 — Daring Apprentice (reprint)
const DARING_APPRENTICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::DARING_APPRENTICE,
    "7c5f0cbe-aba9-4f20-909e-c4692a5ad899",
    "Dany Orizio",
);

// 7ED 68★ — Daring Apprentice (alternate printing)
const DARING_APPRENTICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::DARING_APPRENTICE,
    1,
    "4a1b9eb1-fd88-48f9-bd06-d0b5578d7f82",
    "Dany Orizio",
);

// 7ED 69 — Deflection (reprint)
const DEFLECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::DEFLECTION,
    "5b18ce34-2ff4-4662-aab3-24c10e9657cc",
    "Jeff Easley",
);

// 7ED 69★ — Deflection (alternate printing)
const DEFLECTION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::DEFLECTION,
    1,
    "9ee1aa45-1cee-4710-8b61-a80b393d597d",
    "Jeff Easley",
);

// 7ED 70 — Delusions of Mediocrity (reprint)
const DELUSIONS_OF_MEDIOCRITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::DELUSIONS_OF_MEDIOCRITY,
    "ab43fba7-699e-4bb5-ab3e-b5b1c290340c",
    "Terese Nielsen",
);

// 7ED 70★ — Delusions of Mediocrity (alternate printing)
const DELUSIONS_OF_MEDIOCRITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::DELUSIONS_OF_MEDIOCRITY,
    1,
    "8239e492-0946-4278-add2-9bc6a8b3661d",
    "Terese Nielsen",
);

// 7ED 71 — Equilibrium (reprint)
const EQUILIBRIUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::EQUILIBRIUM,
    "dfbe8836-88c3-4f5d-88a0-73e54673960e",
    "Don Hazeltine",
);

// 7ED 71★ — Equilibrium (alternate printing)
const EQUILIBRIUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::EQUILIBRIUM,
    1,
    "bef600ab-dca3-4853-88b8-5f3bd7ec0a68",
    "Don Hazeltine",
);

// 7ED 72 — Evacuation (reprint)
const EVACUATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::EVACUATION,
    "1e1144eb-701d-4716-9051-e8b77480e72d",
    "Darrell Riche",
);

// 7ED 72★ — Evacuation (alternate printing)
const EVACUATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::EVACUATION,
    1,
    "89dd4a6d-271d-4997-8c61-bc6129bf26da",
    "Darrell Riche",
);

// 7ED 73 — Fighting Drake (reprint)
const FIGHTING_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::FIGHTING_DRAKE,
    "76952add-6ca8-4db0-97bd-4a85432be997",
    "Matt Cavotta",
);

// 7ED 73★ — Fighting Drake (alternate printing)
const FIGHTING_DRAKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::FIGHTING_DRAKE,
    1,
    "6a2b2206-1c9a-4a08-ae13-61e2b5a10727",
    "Matt Cavotta",
);

// 7ED 74 — Fleeting Image (reprint)
const FLEETING_IMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::FLEETING_IMAGE,
    "35971a15-7d8f-4b05-918e-605a26a11f4c",
    "Dave Dorman",
);

// 7ED 74★ — Fleeting Image (alternate printing)
const FLEETING_IMAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::FLEETING_IMAGE,
    1,
    "c86a6f29-6fbf-4fd9-a923-1fdf33abbe16",
    "Dave Dorman",
);

// 7ED 75 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "3abcd7f6-ce34-4c1f-8250-3e262ef0bc05",
    "Bradley Williams",
);

// 7ED 75★ — Flight (alternate printing)
const FLIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FLIGHT,
    1,
    "6d8b8d33-4b80-4ed6-95ce-b672d495be4e",
    "Bradley Williams",
);

// 7ED 76 — Force Spike (reprint)
const FORCE_SPIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FORCE_SPIKE,
    "1d03d73f-0530-4125-8689-1c43e502e331",
    "Nelson DeCastro",
);

// 7ED 76★ — Force Spike (alternate printing)
const FORCE_SPIKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::FORCE_SPIKE,
    1,
    "eb7d6594-a57d-4557-880a-d65f46bb6033",
    "Nelson DeCastro",
);

// 7ED 77 — Giant Octopus (reprint)
const GIANT_OCTOPUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::GIANT_OCTOPUS,
    "5b707b2d-63e1-4c2c-ba42-9e027f02b1ff",
    "Heather Hudson",
);

// 7ED 77★ — Giant Octopus (alternate printing)
const GIANT_OCTOPUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::GIANT_OCTOPUS,
    1,
    "d6343d49-1336-445d-ac7a-83e436953332",
    "Heather Hudson",
);

// 7ED 78 — Glacial Wall (reprint)
const GLACIAL_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::GLACIAL_WALL,
    "a3207ed8-a7b5-490e-bf7e-602937e4408d",
    "Heather Hudson",
);

// 7ED 78★ — Glacial Wall (alternate printing)
const GLACIAL_WALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::GLACIAL_WALL,
    1,
    "1c498cb2-02f7-487d-8561-974f3e516aac",
    "Heather Hudson",
);

// 7ED 79 — Hibernation (reprint)
const HIBERNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::HIBERNATION,
    "a8f7d826-ec78-48dd-bed1-e3768e8fa324",
    "Matt Cavotta",
);

// 7ED 79★ — Hibernation (alternate printing)
const HIBERNATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::HIBERNATION,
    1,
    "d5e87e73-b999-4d57-98ee-1c9f33baca74",
    "Matt Cavotta",
);

// 7ED 80 — Horned Turtle (reprint)
const HORNED_TURTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::HORNED_TURTLE,
    "15005ab0-da4e-415e-99fe-34c112819c45",
    "Edward P. Beard, Jr.",
);

// 7ED 80★ — Horned Turtle (alternate printing)
const HORNED_TURTLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::HORNED_TURTLE,
    1,
    "f64b45d0-4387-4187-8511-12c8a323a772",
    "Edward P. Beard, Jr.",
);

// 7ED 81 — Inspiration (reprint)
const INSPIRATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::visions::INSPIRATION,
    "8b38c335-5a85-4d39-8d58-2b284758de53",
    "Matt Cavotta",
);

// 7ED 81★ — Inspiration (alternate printing)
const INSPIRATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::visions::INSPIRATION,
    1,
    "494dd66f-44af-4a10-94bb-9447af645512",
    "Matt Cavotta",
);

// 7ED 82 — Levitation (reprint)
const LEVITATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_legacy::LEVITATION,
    "6f05d944-1800-4542-a496-62504efc5292",
    "John Matson",
);

// 7ED 82★ — Levitation (alternate printing)
const LEVITATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::urzas_legacy::LEVITATION,
    1,
    "ec5c04b2-ec1c-4954-8551-5421c0843e2b",
    "John Matson",
);

// 7ED 83 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "fd279366-8de2-47c5-9ac0-f41f8f81c643",
    "Greg Staples",
);

// 7ED 83★ — Lord of Atlantis (alternate printing)
const LORD_OF_ATLANTIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::LORD_OF_ATLANTIS,
    1,
    "5200f395-4bb9-4d70-9473-a131b46b60ce",
    "Greg Staples",
);

// 7ED 84 — Mahamoti Djinn (reprint)
const MAHAMOTI_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAHAMOTI_DJINN,
    "5d464226-5607-4db2-bd43-7855efb92628",
    "Eric Peterson",
);

// 7ED 84★ — Mahamoti Djinn (alternate printing)
const MAHAMOTI_DJINN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MAHAMOTI_DJINN,
    1,
    "4e138f24-73ca-4040-b9cc-7ebcf605372e",
    "Eric Peterson",
);

// 7ED 85 — Mana Breach (reprint)
const MANA_BREACH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::MANA_BREACH,
    "e36a7eff-89c8-4799-b264-38892912ba05",
    "Gary Ruddell",
);

// 7ED 85★ — Mana Breach (alternate printing)
const MANA_BREACH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::MANA_BREACH,
    1,
    "401f8b6a-415a-4b73-bfd9-5081c5fcefb3",
    "Gary Ruddell",
);

// 7ED 86 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_SHORT,
    "a0486784-de03-47a7-949d-550fd23492bc",
    "Greg Staples",
);

// 7ED 86★ — Mana Short (alternate printing)
const MANA_SHORT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MANA_SHORT,
    1,
    "5279acd2-7d73-4f42-8696-654326d64bba",
    "Greg Staples",
);

// 7ED 87 — Mawcor (reprint)
const MAWCOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::MAWCOR,
    "48494f33-34b5-4c76-bb24-23a78b856e3c",
    "Kev Walker",
);

// 7ED 87★ — Mawcor (alternate printing)
const MAWCOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::MAWCOR,
    1,
    "31abc4a5-25b5-4f05-9278-a557fb268114",
    "Kev Walker",
);

// 7ED 88 — Memory Lapse (reprint)
const MEMORY_LAPSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::MEMORY_LAPSE,
    "2d85cc30-ccae-4af8-834a-f7870dace679",
    "Tristan Elwell",
);

// 7ED 88★ — Memory Lapse (alternate printing)
const MEMORY_LAPSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_hml::MEMORY_LAPSE,
    1,
    "8c2437b9-22ed-4835-9ac4-e9625bf8464a",
    "Tristan Elwell",
);

// 7ED 89 — Merfolk Looter (reprint)
const MERFOLK_LOOTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::exodus::MERFOLK_LOOTER,
    "4ec07b20-9768-4c21-90d5-70d57959c698",
    "Tristan Elwell",
);

// 7ED 89★ — Merfolk Looter (alternate printing)
const MERFOLK_LOOTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1998::exodus::MERFOLK_LOOTER,
    1,
    "67ffaacf-bf53-49ca-8003-795773477ad3",
    "Tristan Elwell",
);

// 7ED 90 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "a2e7d1a5-b8ad-48e8-9b54-3663310eca33",
    "Ray Lago",
);

// 7ED 90★ — Merfolk of the Pearl Trident (alternate printing)
const MERFOLK_OF_THE_PEARL_TRIDENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    1,
    "fbd471f7-4d06-4a86-bb24-564b88344cdc",
    "Ray Lago",
);

// 7ED 91 — Opportunity (reprint)
const OPPORTUNITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_legacy::OPPORTUNITY,
    "cf01d4d9-c9e9-4826-a155-4527f9be758e",
    "Patrick Faricy",
);

// 7ED 91★ — Opportunity (alternate printing)
const OPPORTUNITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::urzas_legacy::OPPORTUNITY,
    1,
    "95f083a1-664f-46d7-a54d-95f5c10217f2",
    "Patrick Faricy",
);

// 7ED 92 — Opposition (reprint)
const OPPOSITION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::OPPOSITION,
    "8980e292-1384-4662-aa72-bc4f6ca30d51",
    "Mark Brill",
);

// 7ED 92★ — Opposition (alternate printing)
const OPPOSITION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::OPPOSITION,
    1,
    "d4f849df-8a20-4490-a573-1ad3af65e540",
    "Mark Brill",
);

// 7ED 93 — Phantom Warrior (reprint)
const PHANTOM_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::PHANTOM_WARRIOR,
    "594550a7-9d75-4bae-9295-14249f60cc7f",
    "Greg Staples",
);

// 7ED 93★ — Phantom Warrior (alternate printing)
const PHANTOM_WARRIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::PHANTOM_WARRIOR,
    1,
    "55672640-d58f-4e00-a0ac-00916c8375b1",
    "Greg Staples",
);

// 7ED 94 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "d30a846e-a5b1-4603-af4f-6494f3c4fbb3",
    "Tony Szczudlo",
);

// 7ED 94★ — Prodigal Sorcerer (alternate printing)
const PRODIGAL_SORCERER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PRODIGAL_SORCERER,
    1,
    "8da1bfc3-de07-4600-ae01-b22057ba59d1",
    "Tony Szczudlo",
);

// 7ED 95 — Remove Soul (reprint)
const REMOVE_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::REMOVE_SOUL,
    "f25f4f0e-bbf4-46b1-97fd-e796ff9e138f",
    "Adam Rex",
);

// 7ED 95★ — Remove Soul (alternate printing)
const REMOVE_SOUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::REMOVE_SOUL,
    1,
    "e1736d01-1bd0-444c-9d7a-3846441bd409",
    "Adam Rex",
);

// 7ED 96 — Sage Owl (reprint)
const SAGE_OWL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::SAGE_OWL,
    "ec9a3aba-25ae-4b5e-b356-9745e7236f35",
    "Mark Brill",
);

// 7ED 96★ — Sage Owl (alternate printing)
const SAGE_OWL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::SAGE_OWL,
    1,
    "877b79c1-71ee-483d-be6d-07a4a6320200",
    "Mark Brill",
);

// 7ED 97 — Sea Monster (reprint)
const SEA_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::SEA_MONSTER,
    "258c8829-80f5-49d3-9887-e6ed276440c9",
    "John Howe",
);

// 7ED 97★ — Sea Monster (alternate printing)
const SEA_MONSTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::SEA_MONSTER,
    1,
    "95dad56c-7f9a-4f11-920a-5a9d8eee45ce",
    "John Howe",
);

// 7ED 98 — Sleight of Hand (reprint)
const SLEIGHT_OF_HAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &portal_second_age::SLEIGHT_OF_HAND,
    "181fff80-4dea-47ee-a020-d8dc9ea7acdf",
    "Christopher Moeller",
);

// 7ED 98★ — Sleight of Hand (alternate printing)
const SLEIGHT_OF_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::SLEIGHT_OF_HAND,
    1,
    "14525e5c-3e50-4478-8149-7e9c012a85bf",
    "Christopher Moeller",
);

// 7ED 99 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "02f76c5c-f445-4c02-98d1-d7376a76d612",
    "Peter Bollinger",
);

// 7ED 99★ — Steal Artifact (alternate printing)
const STEAL_ARTIFACT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::STEAL_ARTIFACT,
    1,
    "dbad5469-0b78-476f-8573-22fb117392ab",
    "Peter Bollinger",
);

// 7ED 100 — Storm Crow (reprint)
const STORM_CROW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::STORM_CROW,
    "7e573308-40d0-43ce-be04-dbab6bc1ed35",
    "John Matson",
);

// 7ED 100★ — Storm Crow (alternate printing)
const STORM_CROW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_all::STORM_CROW,
    1,
    "782c0a35-09b5-449f-be30-d837e330cb6b",
    "John Matson",
);

// 7ED 101 — Telepathic Spies (reprint)
const TELEPATHIC_SPIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::TELEPATHIC_SPIES,
    "1aeb39c9-7853-4032-882a-83d4863dcbc5",
    "Jim Nelson",
);

// 7ED 101★ — Telepathic Spies (alternate printing)
const TELEPATHIC_SPIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::TELEPATHIC_SPIES,
    1,
    "ac8346c3-8e36-45de-97ec-dfad12ac980c",
    "Jim Nelson",
);

// 7ED 102 — Telepathy (reprint)
const TELEPATHY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::TELEPATHY,
    "00b65a4b-d0d9-4439-96f9-0e0dd532c824",
    "Mark Tedin",
);

// 7ED 102★ — Telepathy (alternate printing)
const TELEPATHY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::TELEPATHY,
    1,
    "42eeef0a-599d-455b-b482-83426f5fdc67",
    "Mark Tedin",
);

// 7ED 103 — Temporal Adept (reprint)
const TEMPORAL_ADEPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::TEMPORAL_ADEPT,
    "eef6d283-a966-4ee9-ab23-5485ccceaf5c",
    "Roger Raupp",
);

// 7ED 103★ — Temporal Adept (alternate printing)
const TEMPORAL_ADEPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::TEMPORAL_ADEPT,
    1,
    "40a9f738-3988-4c60-854d-fb6f9b82ee0b",
    "Roger Raupp",
);

// 7ED 104 — Thieving Magpie (reprint)
const THIEVING_MAGPIE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::THIEVING_MAGPIE,
    "7a487411-ef89-4500-be3c-8e191bd7ddc4",
    "Christopher Moeller",
);

// 7ED 104★ — Thieving Magpie (alternate printing)
const THIEVING_MAGPIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::THIEVING_MAGPIE,
    1,
    "8f5993ed-5ed6-4712-9f69-62d140a0bba5",
    "Christopher Moeller",
);

// 7ED 105 — Tolarian Winds (reprint)
const TOLARIAN_WINDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::TOLARIAN_WINDS,
    "3fdd9981-bb5e-450d-90c3-4405a7097939",
    "Bob Petillo",
);

// 7ED 105★ — Tolarian Winds (alternate printing)
const TOLARIAN_WINDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::TOLARIAN_WINDS,
    1,
    "9ad47859-0756-4aae-b21f-7689da1077bb",
    "Bob Petillo",
);

// 7ED 106 — Treasure Trove (reprint)
const TREASURE_TROVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::TREASURE_TROVE,
    "93f79962-ca0f-4df1-9dc8-d7ea1025cab1",
    "Brian Despain",
);

// 7ED 106★ — Treasure Trove (alternate printing)
const TREASURE_TROVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::TREASURE_TROVE,
    1,
    "c8646aef-f926-4e04-9a6d-c10bea1dd992",
    "Brian Despain",
);

// 7ED 107 — Twiddle (reprint)
const TWIDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWIDDLE,
    "d9483fed-0fc8-4aff-b1b4-8470166fdb9b",
    "Rebecca Guay",
);

// 7ED 107★ — Twiddle (alternate printing)
const TWIDDLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::TWIDDLE,
    1,
    "945956d4-ffd6-4fbc-ae2d-32f981337dd4",
    "Rebecca Guay",
);

// 7ED 108 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "922636e6-9e4e-4aa8-9030-6e1417d241a1",
    "Ron Spencer",
);

// 7ED 108★ — Unsummon (alternate printing)
const UNSUMMON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::UNSUMMON,
    1,
    "d5616986-970c-45d3-878e-403434b35988",
    "Ron Spencer",
);

// 7ED 109 — Vigilant Drake (reprint)
const VIGILANT_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::VIGILANT_DRAKE,
    "5bb4be1f-3e5b-4881-9fae-9ed022f8eeac",
    "Dave Dorman",
);

// 7ED 109★ — Vigilant Drake (alternate printing)
const VIGILANT_DRAKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::VIGILANT_DRAKE,
    1,
    "ae1e116a-370d-494f-8fbb-d964c34f49b4",
    "Dave Dorman",
);

// 7ED 110 — Vizzerdrix (alternate printing)
const VIZZERDRIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::starter_1999::VIZZERDRIX,
    1,
    "c2c681e3-fc54-4da1-80ff-13507688dbc3",
    "Dave Dorman",
);

// 7ED 110★ — Vizzerdrix (reprint)
const VIZZERDRIX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::VIZZERDRIX,
    "249ecab6-e145-4dfd-9e9e-56492db30b4c",
    "Dave Dorman",
);

// 7ED 111 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "ad4b43e8-23d7-4df6-822b-ae3b57c6f1dc",
    "John Avon",
);

// 7ED 111★ — Wall of Air (alternate printing)
const WALL_OF_AIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::WALL_OF_AIR,
    1,
    "a68367ab-cf67-4e0d-a446-676b2734141c",
    "John Avon",
);

// 7ED 112 — Wall of Wonder (reprint)
const WALL_OF_WONDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WALL_OF_WONDER,
    "e8c73e58-e906-4c67-9f84-b20456629cb0",
    "Carl Critchlow",
);

// 7ED 112★ — Wall of Wonder (alternate printing)
const WALL_OF_WONDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::WALL_OF_WONDER,
    1,
    "713698c2-fd5a-4763-b699-00fe73fa6641",
    "Carl Critchlow",
);

// 7ED 113 — Wind Dancer (reprint)
const WIND_DANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::WIND_DANCER,
    "314b5efa-c16f-4bcf-beed-bd0d77511a25",
    "Rob Alexander",
);

// 7ED 113★ — Wind Dancer (alternate printing)
const WIND_DANCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::WIND_DANCER,
    1,
    "f3c7c034-f70e-47c1-b4e9-ec34194bf0b6",
    "Rob Alexander",
);

// 7ED 114 — Wind Drake (reprint)
const WIND_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::WIND_DRAKE,
    "7ba63d1d-6170-4ccd-afd9-987e549fa58e",
    "Tom Wänerstrand",
);

// 7ED 114★ — Wind Drake (alternate printing)
const WIND_DRAKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::WIND_DRAKE,
    1,
    "45e2c8c5-b039-42ed-90f8-95c80d124564",
    "Tom Wänerstrand",
);

// 7ED 115 — Abyssal Horror (reprint)
const ABYSSAL_HORROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::ABYSSAL_HORROR,
    "ccb42943-f599-4068-a364-a023e70c4ed2",
    "Daren Bader",
);

// 7ED 115★ — Abyssal Horror (alternate printing)
const ABYSSAL_HORROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::ABYSSAL_HORROR,
    1,
    "37aa76c5-0391-496a-8f34-d966813fe4e5",
    "Daren Bader",
);

// 7ED 116 — Abyssal Specter (reprint)
const ABYSSAL_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ABYSSAL_SPECTER,
    "a7e6582d-e569-4131-b212-3ef1767be107",
    "Michael Sutfin",
);

// 7ED 116★ — Abyssal Specter (alternate printing)
const ABYSSAL_SPECTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::ABYSSAL_SPECTER,
    1,
    "8dcfb1fb-8e04-4312-9a06-ed3fc2e86c22",
    "Michael Sutfin",
);

// 7ED 117 — Agonizing Memories (reprint)
const AGONIZING_MEMORIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::AGONIZING_MEMORIES,
    "967e7ead-72a5-4f11-87a9-d9498e0d1a6c",
    "Adam Rex",
);

// 7ED 117★ — Agonizing Memories (alternate printing)
const AGONIZING_MEMORIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::AGONIZING_MEMORIES,
    1,
    "8c0c1ee9-5ebc-466d-a163-74d207ef8fd5",
    "Adam Rex",
);

// 7ED 118 — Befoul (reprint)
const BEFOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::BEFOUL,
    "7c5db137-33b9-4cea-9193-4e637d2966f1",
    "Ciruelo",
);

// 7ED 118★ — Befoul (alternate printing)
const BEFOUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::BEFOUL,
    1,
    "9c92a967-ed52-44a8-a07b-04e643fb2e78",
    "Ciruelo",
);

// 7ED 119 — Bellowing Fiend (reprint)
const BELLOWING_FIEND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::BELLOWING_FIEND,
    "2b0962d7-d797-4f07-bd73-9cd7a11ffad8",
    "Chippy",
);

// 7ED 119★ — Bellowing Fiend (alternate printing)
const BELLOWING_FIEND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::BELLOWING_FIEND,
    1,
    "d014dcd8-7132-48e2-b632-00fb0b5c372e",
    "Chippy",
);

// 7ED 120 — Bereavement (reprint)
const BEREAVEMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::BEREAVEMENT,
    "ce20daf7-9cdd-4694-8ed4-4c5dc9f9e7b3",
    "Marc Fishman",
);

// 7ED 120★ — Bereavement (alternate printing)
const BEREAVEMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::BEREAVEMENT,
    1,
    "21138eb5-4eaa-4143-91a6-989fb0c39e40",
    "Marc Fishman",
);

// 7ED 121 — Blood Pet (reprint)
const BLOOD_PET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::BLOOD_PET,
    "9ff08225-82a5-4636-be6a-d38d32f5663f",
    "Heather Hudson",
);

// 7ED 121★ — Blood Pet (alternate printing)
const BLOOD_PET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::BLOOD_PET,
    1,
    "14e67191-0195-4af7-9566-59df8a24cb82",
    "Heather Hudson",
);

// 7ED 122 — Bog Imp (reprint)
const BOG_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOG_IMP,
    "7f1e2c65-d2b0-4bf7-b302-d755e9259ce2",
    "Carl Critchlow",
);

// 7ED 122★ — Bog Imp (alternate printing)
const BOG_IMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::BOG_IMP,
    1,
    "d3f70e3a-b094-45f9-8e34-02db116292ce",
    "Carl Critchlow",
);

// 7ED 123 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "5644b8a9-8777-49da-813a-61ce75324d48",
    "Dave Dorman",
);

// 7ED 123★ — Bog Wraith (alternate printing)
const BOG_WRAITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::BOG_WRAITH,
    1,
    "cb6fd6e1-58b0-4c98-b0ca-58409d3b2383",
    "Dave Dorman",
);

// 7ED 124 — Corrupt (reprint)
const CORRUPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::CORRUPT,
    "57dc4337-86fd-49b4-8331-fcf44f9e7a74",
    "Scott M. Fischer",
);

// 7ED 124★ — Corrupt (alternate printing)
const CORRUPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::CORRUPT,
    1,
    "31fc008a-99cc-4f11-a47c-b7391249bdab",
    "Scott M. Fischer",
);

// 7ED 125 — Crypt Rats (reprint)
const CRYPT_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::CRYPT_RATS,
    "f9b2f697-01cc-4610-b4aa-cae83b38647a",
    "Matt Cavotta",
);

// 7ED 125★ — Crypt Rats (alternate printing)
const CRYPT_RATS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::CRYPT_RATS,
    1,
    "1ce46b67-e0a2-47d6-b3b8-6acedfd7b40f",
    "Matt Cavotta",
);

// 7ED 126 — Dakmor Lancer (reprint)
const DAKMOR_LANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::DAKMOR_LANCER,
    "660cc594-63f5-4819-a556-7a9484145f72",
    "Luca Zontini",
);

// 7ED 126★ — Dakmor Lancer (alternate printing)
const DAKMOR_LANCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::starter_1999::DAKMOR_LANCER,
    1,
    "faca0f0d-c47c-4c29-a47d-d44e2083c3d1",
    "Luca Zontini",
);

// 7ED 127 — Dark Banishing (reprint)
const DARK_BANISHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::DARK_BANISHING,
    "9d03720d-b0ca-4892-9ad1-52189f4a30a1",
    "Rebecca Guay",
);

// 7ED 127★ — Dark Banishing (alternate printing)
const DARK_BANISHING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::DARK_BANISHING,
    1,
    "2f1d76a2-4f74-4d96-841a-3246922df92e",
    "Rebecca Guay",
);

// 7ED 128 — Darkest Hour (reprint)
const DARKEST_HOUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::DARKEST_HOUR,
    "aeeb0f91-2084-448a-8226-95d7c87dd6bb",
    "Ciruelo",
);

// 7ED 128★ — Darkest Hour (alternate printing)
const DARKEST_HOUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::DARKEST_HOUR,
    1,
    "2ad13e04-f099-40c8-a25d-fe2329b47170",
    "Ciruelo",
);

// 7ED 129 — Dregs of Sorrow (reprint)
const DREGS_OF_SORROW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::DREGS_OF_SORROW,
    "c5c4223d-8846-489d-abfc-8330dc58d12b",
    "Massimiliano Frezzato",
);

// 7ED 129★ — Dregs of Sorrow (alternate printing)
const DREGS_OF_SORROW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::DREGS_OF_SORROW,
    1,
    "5b14c738-93cc-4c3a-8d59-6ef7fa22fb1c",
    "Massimiliano Frezzato",
);

// 7ED 130 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "be76e8d0-70d3-4fc7-9320-e78ad93bd362",
    "Jim Nelson",
);

// 7ED 130s — Drudge Skeletons (alternate printing)
const DRUDGE_SKELETONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DRUDGE_SKELETONS,
    1,
    "c55407e4-9d86-4ac9-9360-e44335852a29",
    "Arnie Swekel",
);

// 7ED 130★ — Drudge Skeletons (alternate printing)
const DRUDGE_SKELETONS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DRUDGE_SKELETONS,
    2,
    "de18b04b-16d2-4f92-8b06-35e195ec7b58",
    "Jim Nelson",
);

// 7ED 130★s — Drudge Skeletons (alternate printing)
const DRUDGE_SKELETONS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DRUDGE_SKELETONS,
    3,
    "fcd5d6e8-7e7e-4eb2-8994-10610ebcdf4c",
    "Arnie Swekel",
);

// 7ED 131 — Duress (reprint)
const DURESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::DURESS,
    "15c8d82e-6e65-4d36-bf09-b24dde016581",
    "Pete Venters",
);

// 7ED 131★ — Duress (alternate printing)
const DURESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1998::urzas_saga::DURESS,
    1,
    "08298bdd-38a4-43ef-a2b6-4d9b69b0d417",
    "Pete Venters",
);

// 7ED 132 — Eastern Paladin (reprint)
const EASTERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::EASTERN_PALADIN,
    "aa6dbd6e-c51f-427b-91ab-2d0988fb9966",
    "Kev Walker",
);

// 7ED 132★ — Eastern Paladin (alternate printing)
const EASTERN_PALADIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::EASTERN_PALADIN,
    1,
    "4283a695-57fd-442c-906b-f88b039de79d",
    "Kev Walker",
);

// 7ED 133 — Engineered Plague (reprint)
const ENGINEERED_PLAGUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::ENGINEERED_PLAGUE,
    "b669e43e-3b11-42c9-8f20-0acce129e63c",
    "Andrew Goldhawk",
);

// 7ED 133★ — Engineered Plague (alternate printing)
const ENGINEERED_PLAGUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::ENGINEERED_PLAGUE,
    1,
    "41ad057e-4314-47b3-ace8-e784635740db",
    "Andrew Goldhawk",
);

// 7ED 134 — Fallen Angel (reprint)
const FALLEN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FALLEN_ANGEL,
    "612ecb2c-e732-40cc-9e92-d18b80a26c4c",
    "Arnie Swekel",
);

// 7ED 134★ — Fallen Angel (alternate printing)
const FALLEN_ANGEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::FALLEN_ANGEL,
    1,
    "f84fea71-0018-43b2-ba74-1612cca4ae96",
    "Arnie Swekel",
);

// 7ED 135 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "9036d3b2-f13d-4cfd-aab3-2dd1f0dd3479",
    "Adam Rex",
);

// 7ED 135★ — Fear (alternate printing)
const FEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FEAR,
    1,
    "1d28b0f2-ea67-41ff-a187-356eef227bc0",
    "Adam Rex",
);

// 7ED 136 — Foul Imp (reprint)
const FOUL_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::FOUL_IMP,
    "d6fed5a2-807c-45c0-8692-c9781bee2da9",
    "Kev Walker",
);

// 7ED 136★ — Foul Imp (alternate printing)
const FOUL_IMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::FOUL_IMP,
    1,
    "4b3da66a-ac81-4300-80a1-f70cdb104a90",
    "Kev Walker",
);

// 7ED 137 — Fugue (reprint)
const FUGUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::FUGUE,
    "bee37df6-ca09-44f2-b4a4-ad21be4b2a4d",
    "Alan Pollack",
);

// 7ED 137★ — Fugue (alternate printing)
const FUGUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::FUGUE,
    1,
    "87e5c89d-5f26-420e-b490-3d94239a6280",
    "Alan Pollack",
);

// 7ED 138 — Giant Cockroach (reprint)
const GIANT_COCKROACH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::GIANT_COCKROACH,
    "90f6d42a-85fc-4a25-aedc-719b938661e5",
    "John Matson",
);

// 7ED 138★ — Giant Cockroach (alternate printing)
const GIANT_COCKROACH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::GIANT_COCKROACH,
    1,
    "ade86194-9b77-4175-8088-da784cbcfd49",
    "John Matson",
);

// 7ED 139 — Gravedigger (reprint)
const GRAVEDIGGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::GRAVEDIGGER,
    "961dec46-b96e-486c-939f-9c11aa75bd04",
    "James Bernardin",
);

// 7ED 139★ — Gravedigger (alternate printing)
const GRAVEDIGGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::GRAVEDIGGER,
    1,
    "590a3850-c59e-4357-9f34-f7a57c4486e0",
    "James Bernardin",
);

// 7ED 140 — Greed (reprint)
const GREED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GREED,
    "f06db06b-0780-41c7-9b6c-a688b0f5fa2c",
    "Peter Bollinger",
);

// 7ED 140★ — Greed (alternate printing)
const GREED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::GREED,
    1,
    "4b51b23a-d67d-4ede-a1b2-1ba6dad9883e",
    "Peter Bollinger",
);

// 7ED 141 — Hollow Dogs (reprint)
const HOLLOW_DOGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::HOLLOW_DOGS,
    "93e6ed54-a72c-413a-8038-a3ed571571bd",
    "Arnie Swekel",
);

// 7ED 141★ — Hollow Dogs (alternate printing)
const HOLLOW_DOGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::HOLLOW_DOGS,
    1,
    "c036abf0-9da3-4cc4-8aa1-92293fad77b2",
    "Arnie Swekel",
);

// 7ED 142 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "2fedf1b4-a3ee-410e-8b2b-ff848a8f60c4",
    "Dave Dorman",
);

// 7ED 142★ — Howl from Beyond (alternate printing)
const HOWL_FROM_BEYOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::HOWL_FROM_BEYOND,
    1,
    "15fd1202-ad8b-4526-ba6b-0460f56d91a6",
    "Dave Dorman",
);

// 7ED 143 — Infernal Contract (reprint)
const INFERNAL_CONTRACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::INFERNAL_CONTRACT,
    "f451a70f-1f1c-4fd6-ab0c-4b77a043c324",
    "Pete Venters",
);

// 7ED 143★ — Infernal Contract (alternate printing)
const INFERNAL_CONTRACT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::INFERNAL_CONTRACT,
    1,
    "86272ecc-33bd-4144-996a-9e0c9d0a9e20",
    "Pete Venters",
);

// 7ED 144 — Leshrac's Rite (reprint)
const LESHRAC_S_RITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::LESHRAC_S_RITE,
    "e4d37d29-b667-4124-907d-5636a6db044f",
    "rk post",
);

// 7ED 144★ — Leshrac's Rite (alternate printing)
const LESHRAC_S_RITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::LESHRAC_S_RITE,
    1,
    "2793f9ea-6a3d-4c02-92d2-0eb62a4dedb2",
    "rk post",
);

// 7ED 145 — Looming Shade (reprint)
const LOOMING_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::LOOMING_SHADE,
    "76c49d4b-cb89-4f63-ac94-a2075f79f628",
    "Kev Walker",
);

// 7ED 145★ — Looming Shade (alternate printing)
const LOOMING_SHADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::LOOMING_SHADE,
    1,
    "184befc7-be32-47b2-a634-a6d474daa553",
    "Kev Walker",
);

// 7ED 146 — Megrim (reprint)
const MEGRIM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::MEGRIM,
    "2c4f01f9-a673-43df-a263-7c2269c2235e",
    "Peter Bollinger",
);

// 7ED 146★ — Megrim (alternate printing)
const MEGRIM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::MEGRIM,
    1,
    "4d12b851-9b48-45ab-93fb-4236a63d59c9",
    "Peter Bollinger",
);

// 7ED 147 — Mind Rot (reprint)
const MIND_ROT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::MIND_ROT,
    "5681e85c-79d5-4300-bda6-4ae40bb7d5d4",
    "Adam Rex",
);

// 7ED 147★ — Mind Rot (alternate printing)
const MIND_ROT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::MIND_ROT,
    1,
    "d7628b7e-7da8-4ef8-8be2-1865605a7589",
    "Adam Rex",
);

// 7ED 148 — Nausea (reprint)
const NAUSEA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::NAUSEA,
    "b71315e3-14c1-433b-97be-2cdf99213bba",
    "James Bernardin",
);

// 7ED 148★ — Nausea (alternate printing)
const NAUSEA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::NAUSEA,
    1,
    "32037f20-7443-464c-917f-b062f05f58c3",
    "James Bernardin",
);

// 7ED 149 — Necrologia (reprint)
const NECROLOGIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::NECROLOGIA,
    "9da145ff-8989-4457-b408-792d7ad10df9",
    "Scott M. Fischer",
);

// 7ED 149★ — Necrologia (alternate printing)
const NECROLOGIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::NECROLOGIA,
    1,
    "fb901bc0-e544-4e3a-b447-0eabe8710d73",
    "Scott M. Fischer",
);

// 7ED 150 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "c3779fda-5de0-4d80-8af0-95956e87d9e1",
    "Carl Critchlow",
);

// 7ED 150★ — Nightmare (alternate printing)
const NIGHTMARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::NIGHTMARE,
    1,
    "8817ef5f-39f0-4358-bc98-f512f9cce8f0",
    "Carl Critchlow",
);

// 7ED 151 — Nocturnal Raid (reprint)
const NOCTURNAL_RAID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::NOCTURNAL_RAID,
    "77419949-7e62-48fe-b952-56d943ddd39f",
    "Pete Venters",
);

// 7ED 151★ — Nocturnal Raid (alternate printing)
const NOCTURNAL_RAID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::NOCTURNAL_RAID,
    1,
    "2b354cf4-ca4f-400c-ab74-ac80fe240b74",
    "Pete Venters",
);

// 7ED 152 — Oppression (reprint)
const OPPRESSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::OPPRESSION,
    "ac327f80-983e-4e28-96e8-91ff5377f5a3",
    "Alex Horley-Orlandelli",
);

// 7ED 152★ — Oppression (alternate printing)
const OPPRESSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::OPPRESSION,
    1,
    "9cc8791c-3d7a-4e61-ac5f-8aec5f44b12c",
    "Alex Horley-Orlandelli",
);

// 7ED 153 — Ostracize (reprint)
const OSTRACIZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::OSTRACIZE,
    "abdaffcc-59f6-4489-88bf-1061ad6b0512",
    "Hannibal King",
);

// 7ED 153★ — Ostracize (alternate printing)
const OSTRACIZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::OSTRACIZE,
    1,
    "e02ee53c-74dc-487d-88dc-e62a533c63b3",
    "Hannibal King",
);

// 7ED 154 — Persecute (reprint)
const PERSECUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::PERSECUTE,
    "de21334b-f06c-4525-89e1-dc3f148210ef",
    "Luca Zontini",
);

// 7ED 154★ — Persecute (alternate printing)
const PERSECUTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::PERSECUTE,
    1,
    "2385b6f1-e75c-41d3-9cab-39ae6c315b93",
    "Luca Zontini",
);

// 7ED 155 — Plague Beetle (reprint)
const PLAGUE_BEETLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::PLAGUE_BEETLE,
    "311dce7e-1de9-43c9-a29a-144c189873da",
    "Matt Cavotta",
);

// 7ED 155★ — Plague Beetle (alternate printing)
const PLAGUE_BEETLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::PLAGUE_BEETLE,
    1,
    "a91546e2-3384-40c0-a7b9-d83a7a642083",
    "Matt Cavotta",
);

// 7ED 156 — Rag Man (reprint)
const RAG_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::RAG_MAN,
    "8e3809e6-41ac-47e8-80dc-9e9c8be1ed7a",
    "Scott M. Fischer",
);

// 7ED 156★ — Rag Man (alternate printing)
const RAG_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::RAG_MAN,
    1,
    "94dd33f6-2b7b-483a-b38e-bcf8e69a935c",
    "Scott M. Fischer",
);

// 7ED 157 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "929cdbb4-d8b3-4e01-918c-2f46d74bb455",
    "Carl Critchlow",
);

// 7ED 157s — Raise Dead (alternate printing)
const RAISE_DEAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::RAISE_DEAD,
    1,
    "d936478a-c9a9-45b6-afd7-de30f1c3221f",
    "Douglas Shuler",
);

// 7ED 157★ — Raise Dead (alternate printing)
const RAISE_DEAD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::RAISE_DEAD,
    2,
    "062f3566-ff6a-4f2a-9896-9547e46e6bac",
    "Carl Critchlow",
);

// 7ED 157★s — Raise Dead (alternate printing)
const RAISE_DEAD_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::RAISE_DEAD,
    3,
    "3deb1c2e-49ae-4f26-9f7d-6643cb79eea5",
    "Douglas Shuler",
);

// 7ED 158 — Razortooth Rats (reprint)
const RAZORTOOTH_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::RAZORTOOTH_RATS,
    "68064995-c1e4-4ea7-b6b7-246ce49e2cf4",
    "Carl Critchlow",
);

// 7ED 158★ — Razortooth Rats (alternate printing)
const RAZORTOOTH_RATS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::RAZORTOOTH_RATS,
    1,
    "0cd2145f-298b-4c27-b8b2-18f6975087dd",
    "Carl Critchlow",
);

// 7ED 159 — Reprocess (reprint)
const REPROCESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::REPROCESS,
    "1dc8c769-f2d1-4ef3-bbcc-277f39a10dbd",
    "John Howe",
);

// 7ED 159★ — Reprocess (alternate printing)
const REPROCESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::REPROCESS,
    1,
    "3eaac30c-9ae6-4788-b065-ae4095ee3097",
    "John Howe",
);

// 7ED 160 — Revenant (reprint)
const REVENANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::REVENANT,
    "90c75244-68b3-463d-aad3-2b22f1eaf717",
    "Andrew Goldhawk",
);

// 7ED 160★ — Revenant (alternate printing)
const REVENANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::REVENANT,
    1,
    "c979ec16-65f4-4afa-8e03-c05ce46337b0",
    "Andrew Goldhawk",
);

// 7ED 161 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "ed515f6f-432e-4455-a871-5cefdd15a37c",
    "Kev Walker",
);

// 7ED 161s — Scathe Zombies (alternate printing)
const SCATHE_ZOMBIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SCATHE_ZOMBIES,
    1,
    "a7167902-939f-4aae-819a-8b941ce45cb1",
    "John Howe",
);

// 7ED 161★ — Scathe Zombies (alternate printing)
const SCATHE_ZOMBIES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SCATHE_ZOMBIES,
    2,
    "88bddc0b-f000-4679-bbfd-ab7674e30343",
    "Kev Walker",
);

// 7ED 161★s — Scathe Zombies (alternate printing)
const SCATHE_ZOMBIES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SCATHE_ZOMBIES,
    3,
    "6be514ca-dd8a-4b84-9125-03a0bf8a047d",
    "John Howe",
);

// 7ED 162 — Serpent Warrior (reprint)
const SERPENT_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::SERPENT_WARRIOR,
    "4470eb5d-a968-4e08-a96a-4e92ecc6d56c",
    "Eric Peterson",
);

// 7ED 162★ — Serpent Warrior (alternate printing)
const SERPENT_WARRIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::SERPENT_WARRIOR,
    1,
    "138658d9-368e-46c6-b957-8bdb0699457d",
    "Eric Peterson",
);

// 7ED 163 — Soul Feast (reprint)
const SOUL_FEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::SOUL_FEAST,
    "a273c4f4-b156-4bf5-a33d-656a4e49a0ff",
    "Adam Rex",
);

// 7ED 163★ — Soul Feast (alternate printing)
const SOUL_FEAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::SOUL_FEAST,
    1,
    "625e0586-dce6-47b5-adad-22d369d4c021",
    "Adam Rex",
);

// 7ED 164 — Spineless Thug (reprint)
const SPINELESS_THUG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_nem::SPINELESS_THUG,
    "0c091d07-5813-47b9-b1da-d749e3f4e5aa",
    "Alan Pollack",
);

// 7ED 164★ — Spineless Thug (alternate printing)
const SPINELESS_THUG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_nem::SPINELESS_THUG,
    1,
    "adc7ec32-96ca-472f-9a2c-4b4b30cdcc98",
    "Alan Pollack",
);

// 7ED 165 — Strands of Night (reprint)
const STRANDS_OF_NIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::STRANDS_OF_NIGHT,
    "d8f6685d-eb5d-403f-b89a-b8b67900b602",
    "Glen Angus",
);

// 7ED 165★ — Strands of Night (alternate printing)
const STRANDS_OF_NIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::STRANDS_OF_NIGHT,
    1,
    "fee79771-6bb8-4575-b9f4-bd1107a676c0",
    "Glen Angus",
);

// 7ED 166 — Stronghold Assassin (reprint)
const STRONGHOLD_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::STRONGHOLD_ASSASSIN,
    "f388ad92-fa79-4a1a-b75c-e340dba016f4",
    "Ron Walotsky",
);

// 7ED 166★ — Stronghold Assassin (alternate printing)
const STRONGHOLD_ASSASSIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::STRONGHOLD_ASSASSIN,
    1,
    "58fba666-9648-44d8-9cbd-f98bca90fccb",
    "Ron Walotsky",
);

// 7ED 167 — Tainted Aether (reprint)
const TAINTED_AETHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::TAINTED_AETHER,
    "838cffb6-8099-44d7-a127-b2ab3b53ea44",
    "Ciruelo",
);

// 7ED 167★ — Tainted Aether (alternate printing)
const TAINTED_AETHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::TAINTED_AETHER,
    1,
    "eca62508-4f7f-4dac-82ae-b978acb32c67",
    "Ciruelo",
);

// 7ED 168 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "0030407c-9aa0-49ad-b2d6-cde0adbd9d09",
    "Gary Ruddell",
);

// 7ED 168★ — Unholy Strength (alternate printing)
const UNHOLY_STRENGTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::UNHOLY_STRENGTH,
    1,
    "42101792-83a7-492d-a5b0-34aab7db56d1",
    "Gary Ruddell",
);

// 7ED 169 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "8fc5092f-a248-471e-87e0-8394d5e2d3fe",
    "Alan Pollack",
);

// 7ED 169★ — Wall of Bone (alternate printing)
const WALL_OF_BONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::WALL_OF_BONE,
    1,
    "eb18ebfa-9a70-48c6-a906-149f0e7cd474",
    "Alan Pollack",
);

// 7ED 170 — Western Paladin (reprint)
const WESTERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::WESTERN_PALADIN,
    "aa4d19d4-a29b-4b1f-b29b-adcad557c3c7",
    "Paolo Parente",
);

// 7ED 170★ — Western Paladin (alternate printing)
const WESTERN_PALADIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::WESTERN_PALADIN,
    1,
    "f0f9ad3e-f392-48ed-9f31-3a997a3b7358",
    "Paolo Parente",
);

// 7ED 171 — Yawgmoth's Edict (reprint)
const YAWGMOTH_S_EDICT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::YAWGMOTH_S_EDICT,
    "f72ed0e5-37a0-4909-a37e-ba4745bfae3b",
    "Donato Giancola",
);

// 7ED 171★ — Yawgmoth's Edict (alternate printing)
const YAWGMOTH_S_EDICT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::YAWGMOTH_S_EDICT,
    1,
    "0d2b1255-d917-4884-aa9e-4b1f92e88f79",
    "Donato Giancola",
);

// 7ED 172 — Aether Flash (reprint)
const AETHER_FLASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::AETHER_FLASH,
    "780f9197-e910-4c7a-bb4b-2c4a94903c39",
    "Wayne England",
);

// 7ED 172★ — Aether Flash (alternate printing)
const AETHER_FLASH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::AETHER_FLASH,
    1,
    "7ff6b8fa-3546-415f-89f1-1176d22ad15e",
    "Wayne England",
);

// 7ED 173 — Balduvian Barbarians (reprint)
const BALDUVIAN_BARBARIANS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BALDUVIAN_BARBARIANS,
    "e2507e14-96d1-40e6-9379-4d4749d74e1d",
    "Jim Nelson",
);

// 7ED 173★ — Balduvian Barbarians (alternate printing)
const BALDUVIAN_BARBARIANS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::BALDUVIAN_BARBARIANS,
    1,
    "c201ace3-4d99-467e-a558-7d22ad004cf2",
    "Jim Nelson",
);

// 7ED 174 — Bedlam (reprint)
const BEDLAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::BEDLAM,
    "2788ec1a-930e-4a19-ad69-ea456c1390fd",
    "Ron Spencer",
);

// 7ED 174★ — Bedlam (alternate printing)
const BEDLAM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::BEDLAM,
    1,
    "fdbcb398-85be-4923-9888-9dcca89a0b59",
    "Ron Spencer",
);

// 7ED 175 — Blaze (reprint)
const BLAZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BLAZE,
    "26f8c6ab-ae62-4e2e-a5ba-2ec5bbe22445",
    "Alex Horley-Orlandelli",
);

// 7ED 175★ — Blaze (alternate printing)
const BLAZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::BLAZE,
    1,
    "a4a9a4ba-98ef-4eb1-a650-a77b84226701",
    "Alex Horley-Orlandelli",
);

// 7ED 176 — Bloodshot Cyclops (reprint)
const BLOODSHOT_CYCLOPS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::BLOODSHOT_CYCLOPS,
    "387dd482-c67d-43da-a4d0-582241431ffd",
    "Daren Bader",
);

// 7ED 176★ — Bloodshot Cyclops (alternate printing)
const BLOODSHOT_CYCLOPS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::BLOODSHOT_CYCLOPS,
    1,
    "885c07aa-beae-4818-9424-6dbfb30a428d",
    "Daren Bader",
);

// 7ED 177 — Boil (reprint)
const BOIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::BOIL,
    "52969d36-f392-4920-b998-589c8356b898",
    "Christopher Moeller",
);

// 7ED 177★ — Boil (alternate printing)
const BOIL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::BOIL,
    1,
    "b09b9aae-0f87-4b0e-b4b2-7961f1e7f2e0",
    "Christopher Moeller",
);

// 7ED 178 — Crimson Hellkite (reprint)
const CRIMSON_HELLKITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::CRIMSON_HELLKITE,
    "5451e44f-8dd2-48c6-ba67-5c62a04819ef",
    "Carl Critchlow",
);

// 7ED 178★ — Crimson Hellkite (alternate printing)
const CRIMSON_HELLKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::CRIMSON_HELLKITE,
    1,
    "91ed1fde-817d-4e0a-8268-461a680aed96",
    "Carl Critchlow",
);

// 7ED 179 — Disorder (reprint)
const DISORDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::DISORDER,
    "b6d11422-60a9-4386-8e7f-dd7dcdac58d8",
    "Glen Angus",
);

// 7ED 179★ — Disorder (alternate printing)
const DISORDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::DISORDER,
    1,
    "4569ca10-6c5e-44ea-810e-62082e589d5a",
    "Glen Angus",
);

// 7ED 180 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHQUAKE,
    "8f04dc5c-2764-42d0-974e-6d902222c138",
    "Franz Vohwinkel",
);

// 7ED 180★ — Earthquake (alternate printing)
const EARTHQUAKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::EARTHQUAKE,
    1,
    "0c39fb5f-9eb7-4a4c-9382-80b5a9459afc",
    "Franz Vohwinkel",
);

// 7ED 181 — Fervor (reprint)
const FERVOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::weatherlight::FERVOR,
    "53834370-845c-4677-b665-e556eae8f9de",
    "Wayne England",
);

// 7ED 181★ — Fervor (alternate printing)
const FERVOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::weatherlight::FERVOR,
    1,
    "8c955ff9-0615-4872-8a05-770558ff81dd",
    "Wayne England",
);

// 7ED 182 — Final Fortune (reprint)
const FINAL_FORTUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FINAL_FORTUNE,
    "fb80afc3-4887-42a0-afb2-7fa997981fb2",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 182★ — Final Fortune (alternate printing)
const FINAL_FORTUNE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::FINAL_FORTUNE,
    1,
    "f60b7014-3e29-4308-976c-0d92df5cec3a",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 183 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "25b57e53-220c-4181-80d3-8063864aefc2",
    "Douglas Shuler",
);

// 7ED 183★ — Fire Elemental (alternate printing)
const FIRE_ELEMENTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FIRE_ELEMENTAL,
    1,
    "54b05f47-f86b-46e8-912f-b1273005d46d",
    "Douglas Shuler",
);

// 7ED 184 — Ghitu Fire-Eater (reprint)
const GHITU_FIRE_EATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::GHITU_FIRE_EATER,
    "0770cc34-0f38-4773-8633-6907f44436c4",
    "Eric Peterson",
);

// 7ED 184★ — Ghitu Fire-Eater (alternate printing)
const GHITU_FIRE_EATER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::GHITU_FIRE_EATER,
    1,
    "c6976444-a076-4e67-80f7-014a1c955eed",
    "Eric Peterson",
);

// 7ED 185 — Goblin Chariot (alternate printing)
const GOBLIN_CHARIOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::starter_1999::GOBLIN_CHARIOT,
    1,
    "f7571801-c1df-4387-ae61-1fefd449ebf9",
    "John Howe",
);

// 7ED 185★ — Goblin Chariot (reprint)
const GOBLIN_CHARIOT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::GOBLIN_CHARIOT,
    "1db520e2-9926-45d2-a140-37b119b88106",
    "John Howe",
);

// 7ED 186 — Goblin Digging Team (reprint)
const GOBLIN_DIGGING_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_DIGGING_TEAM,
    "8000425a-4761-4370-95eb-6fe3df628482",
    "Ben Thompson",
);

// 7ED 186★ — Goblin Digging Team (alternate printing)
const GOBLIN_DIGGING_TEAM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::GOBLIN_DIGGING_TEAM,
    1,
    "eb84e6b4-2f87-4662-843d-27d2eada54aa",
    "Ben Thompson",
);

// 7ED 187 — Goblin Elite Infantry (reprint)
const GOBLIN_ELITE_INFANTRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::GOBLIN_ELITE_INFANTRY,
    "3f5b7dfe-f24b-4d73-813f-f63c926f3672",
    "Daren Bader",
);

// 7ED 187★ — Goblin Elite Infantry (alternate printing)
const GOBLIN_ELITE_INFANTRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::GOBLIN_ELITE_INFANTRY,
    1,
    "9facc1bf-b088-4be0-acce-be95c0a3e9e6",
    "Daren Bader",
);

// 7ED 188 — Goblin Gardener (reprint)
const GOBLIN_GARDENER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::GOBLIN_GARDENER,
    "9f5b02af-140e-404d-bf8a-6a706b323a13",
    "Jerry Tiritilli",
);

// 7ED 188★ — Goblin Gardener (alternate printing)
const GOBLIN_GARDENER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::GOBLIN_GARDENER,
    1,
    "009c1788-a3ab-41cb-8f9a-d220c376953b",
    "Jerry Tiritilli",
);

// 7ED 189 — Goblin Glider (reprint)
const GOBLIN_GLIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::GOBLIN_GLIDER,
    "839a7e39-8d98-4e84-8a5c-2b067c8654d5",
    "Patrick Faricy",
);

// 7ED 189★ — Goblin Glider (alternate printing)
const GOBLIN_GLIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::GOBLIN_GLIDER,
    1,
    "5e8cf7c4-f217-42c8-a1ea-0866576b9524",
    "Patrick Faricy",
);

// 7ED 190 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GOBLIN_KING,
    "0c77029a-7f00-490e-bf8b-dce192d72e2f",
    "Ron Spears",
);

// 7ED 190★ — Goblin King (alternate printing)
const GOBLIN_KING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::GOBLIN_KING,
    1,
    "d32b3637-ffc8-4bda-bfc1-912f5789b5ed",
    "Ron Spears",
);

// 7ED 191 — Goblin Matron (reprint)
const GOBLIN_MATRON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::GOBLIN_MATRON,
    "862409e1-33e0-474c-8627-b03d25b654b9",
    "Dan Frazier",
);

// 7ED 191★ — Goblin Matron (alternate printing)
const GOBLIN_MATRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::GOBLIN_MATRON,
    1,
    "22a80d36-349d-4b49-9a0c-7e0a400abb67",
    "Dan Frazier",
);

// 7ED 192 — Goblin Raider (reprint)
const GOBLIN_RAIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::GOBLIN_RAIDER,
    "3315d75d-08dc-456c-a8e7-fe3136bf1a6b",
    "Arnie Swekel",
);

// 7ED 192★ — Goblin Raider (alternate printing)
const GOBLIN_RAIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::GOBLIN_RAIDER,
    1,
    "88af4784-e126-4628-b228-6c0a95f00a25",
    "Arnie Swekel",
);

// 7ED 193 — Goblin Spelunkers (reprint)
const GOBLIN_SPELUNKERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::GOBLIN_SPELUNKERS,
    "a3c4dfc6-8b3f-45fb-a1e2-b773f74cd9c2",
    "Matt Cavotta",
);

// 7ED 193★ — Goblin Spelunkers (alternate printing)
const GOBLIN_SPELUNKERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::GOBLIN_SPELUNKERS,
    1,
    "e27f5299-696a-4e00-8221-f349f9b1d461",
    "Matt Cavotta",
);

// 7ED 194 — Goblin War Drums (reprint)
const GOBLIN_WAR_DRUMS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::GOBLIN_WAR_DRUMS,
    "b226987d-e271-483c-9d18-09c461ebbf36",
    "Peter Bollinger",
);

// 7ED 194★ — Goblin War Drums (alternate printing)
const GOBLIN_WAR_DRUMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_fem::GOBLIN_WAR_DRUMS,
    1,
    "b2de28d4-ab1d-4e4d-a8b9-6faa19f9d283",
    "Peter Bollinger",
);

// 7ED 195 — Granite Grip (reprint)
const GRANITE_GRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::GRANITE_GRIP,
    "908c8414-5eff-4887-9007-43050058c2d0",
    "Ray Lago",
);

// 7ED 195★ — Granite Grip (alternate printing)
const GRANITE_GRIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::GRANITE_GRIP,
    1,
    "cfb23039-0f38-44f9-845d-b76ca85e4761",
    "Ray Lago",
);

// 7ED 196 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "e7ea1719-2bed-46f4-bb14-e3a4c87ce50a",
    "Dany Orizio",
);

// 7ED 196★ — Hill Giant (alternate printing)
const HILL_GIANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::HILL_GIANT,
    1,
    "07724b6b-73e6-43b9-980f-149047c8e786",
    "Dany Orizio",
);

// 7ED 197 — Impatience (reprint)
const IMPATIENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::IMPATIENCE,
    "11443908-358f-4886-a2da-9b98002a3a3a",
    "Kunio Hagio",
);

// 7ED 197★ — Impatience (alternate printing)
const IMPATIENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::IMPATIENCE,
    1,
    "11c59ed1-ecc2-4f0b-a49a-473e797059e7",
    "Kunio Hagio",
);

// 7ED 198 — Inferno (reprint)
const INFERNO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::INFERNO,
    "e411b7b5-ab91-410a-af6d-b3a21a8e3b70",
    "Don Hazeltine",
);

// 7ED 198★ — Inferno (alternate printing)
const INFERNO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::INFERNO,
    1,
    "4a0d589d-d539-453b-b401-14a74e3da4ea",
    "Don Hazeltine",
);

// 7ED 199 — Lava Axe (reprint)
const LAVA_AXE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::LAVA_AXE,
    "807e5102-1fab-4ff4-aad8-94defbbb8a6b",
    "Ray Lago",
);

// 7ED 199★ — Lava Axe (alternate printing)
const LAVA_AXE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::LAVA_AXE,
    1,
    "ec1292a1-3d0e-47a3-97ef-e4ffcdfb492e",
    "Ray Lago",
);

// 7ED 200 — Lightning Blast (reprint)
const LIGHTNING_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::LIGHTNING_BLAST,
    "83e3c502-9e3c-41db-806c-538243dc0453",
    "Ron Spencer",
);

// 7ED 200★ — Lightning Blast (alternate printing)
const LIGHTNING_BLAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::LIGHTNING_BLAST,
    1,
    "dd40ef12-d3fd-4bb9-8990-aa967d8df2dd",
    "Ron Spencer",
);

// 7ED 201 — Lightning Elemental (reprint)
const LIGHTNING_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::tempest::LIGHTNING_ELEMENTAL,
    "6df538e3-84c9-4580-85e9-8ac2f9a1294b",
    "Kev Walker",
);

// 7ED 201★ — Lightning Elemental (alternate printing)
const LIGHTNING_ELEMENTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::tempest::LIGHTNING_ELEMENTAL,
    1,
    "6f3d80a8-ac70-4847-b406-321af24de47c",
    "Kev Walker",
);

// 7ED 202 — Mana Clash (reprint)
const MANA_CLASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MANA_CLASH,
    "740b85ff-61a3-4de0-a055-60daad13ac2a",
    "Larry Elmore",
);

// 7ED 202★ — Mana Clash (alternate printing)
const MANA_CLASH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::MANA_CLASH,
    1,
    "74c092c3-1fbe-4319-aefe-d5b819ee953f",
    "Larry Elmore",
);

// 7ED 203 — Ogre Taskmaster (reprint)
const OGRE_TASKMASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::OGRE_TASKMASTER,
    "6b2229fe-5b8f-42c0-bfe6-2c0c3d84624a",
    "Jeff Easley",
);

// 7ED 203★ — Ogre Taskmaster (alternate printing)
const OGRE_TASKMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::OGRE_TASKMASTER,
    1,
    "aa53fb37-d746-491b-830f-46bd60bc2817",
    "Jeff Easley",
);

// 7ED 204 — Okk (reprint)
const OKK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::OKK,
    "342fd427-baec-435f-9dc7-339c93f43f89",
    "Peter Bollinger",
);

// 7ED 204★ — Okk (alternate printing)
const OKK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::OKK,
    1,
    "88c90704-063c-4774-b675-cb5727afa656",
    "Peter Bollinger",
);

// 7ED 205 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "e5b13fc4-e26a-4a7c-bde2-ea3626da6aa8",
    "Dan Frazier",
);

// 7ED 205★ — Orcish Artillery (alternate printing)
const ORCISH_ARTILLERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ORCISH_ARTILLERY,
    1,
    "e42afac9-b9a4-4d02-b026-3d03c41d15c6",
    "Dan Frazier",
);

// 7ED 206 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "45467389-980b-4eaf-9b4b-38fefb307a7c",
    "Ben Thompson",
);

// 7ED 206★ — Orcish Oriflamme (alternate printing)
const ORCISH_ORIFLAMME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ORCISH_ORIFLAMME,
    1,
    "b10c2e08-c18a-4fc6-9e8b-120fb87c4f26",
    "Ben Thompson",
);

// 7ED 207 — Pillage (reprint)
const PILLAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::PILLAGE,
    "9792efaa-1f73-48de-8c10-5d20c2856f3d",
    "Bradley Williams",
);

// 7ED 207★ — Pillage (alternate printing)
const PILLAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_all::PILLAGE,
    1,
    "0a4fc2a6-02ff-406c-8a8e-c97ea54b3f8b",
    "Bradley Williams",
);

// 7ED 208 — Pygmy Pyrosaur (reprint)
const PYGMY_PYROSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::PYGMY_PYROSAUR,
    "42e01129-254c-4a16-9f11-21a7a9c66f32",
    "Dan Frazier",
);

// 7ED 208★ — Pygmy Pyrosaur (alternate printing)
const PYGMY_PYROSAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::PYGMY_PYROSAUR,
    1,
    "57eab2ef-6c3c-4f0e-8566-113070604a1d",
    "Dan Frazier",
);

// 7ED 209 — Pyroclasm (reprint)
const PYROCLASM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PYROCLASM,
    "7afce33f-2ead-4943-9655-bff6eaa9fe6b",
    "Monte Michael Moore",
);

// 7ED 209★ — Pyroclasm (alternate printing)
const PYROCLASM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::PYROCLASM,
    1,
    "964b8482-9154-4f26-9ae7-641d0c00ca99",
    "Monte Michael Moore",
);

// 7ED 210 — Pyrotechnics (reprint)
const PYROTECHNICS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PYROTECHNICS,
    "4c5d0adf-9368-4d7f-8bd0-76d0db95ba16",
    "John Avon",
);

// 7ED 210★ — Pyrotechnics (alternate printing)
const PYROTECHNICS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::PYROTECHNICS,
    1,
    "8dfea5f9-500e-45e8-80b0-9ff9bd09e07a",
    "John Avon",
);

// 7ED 211 — Raging Goblin (reprint)
const RAGING_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::RAGING_GOBLIN,
    "657190fe-9c18-4134-a556-e081daff73cd",
    "Peter Bollinger",
);

// 7ED 211★ — Raging Goblin (alternate printing)
const RAGING_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::RAGING_GOBLIN,
    1,
    "bef1c9bb-fa05-4947-9e03-6fe397157265",
    "Peter Bollinger",
);

// 7ED 212 — Reckless Embermage (reprint)
const RECKLESS_EMBERMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::RECKLESS_EMBERMAGE,
    "b0f75ed4-b96c-444b-ac91-8aaa02f32f2d",
    "Bob Petillo",
);

// 7ED 212★ — Reckless Embermage (alternate printing)
const RECKLESS_EMBERMAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::RECKLESS_EMBERMAGE,
    1,
    "cb8a8bb2-e876-4e62-a437-96cc4dd6be89",
    "Bob Petillo",
);

// 7ED 213 — Reflexes (reprint)
const REFLEXES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::REFLEXES,
    "fdc8ce35-589b-4903-8d52-8ccbea7b767b",
    "Donato Giancola",
);

// 7ED 213★ — Reflexes (alternate printing)
const REFLEXES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::REFLEXES,
    1,
    "05f91d14-b409-44b6-bdfd-44427855373a",
    "Donato Giancola",
);

// 7ED 214 — Relentless Assault (reprint)
const RELENTLESS_ASSAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::RELENTLESS_ASSAULT,
    "a0c48308-12e8-4b85-a5ef-1e0643bd814c",
    "Greg Hildebrandt",
);

// 7ED 214★ — Relentless Assault (alternate printing)
const RELENTLESS_ASSAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::RELENTLESS_ASSAULT,
    1,
    "242d25fe-e03a-4fa9-8049-d1bb0aef3d07",
    "Greg Hildebrandt",
);

// 7ED 215 — Sabretooth Tiger (reprint)
const SABRETOOTH_TIGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SABRETOOTH_TIGER,
    "4aee4a17-49da-446f-a134-1261980a249f",
    "Monte Michael Moore",
);

// 7ED 215★ — Sabretooth Tiger (alternate printing)
const SABRETOOTH_TIGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::SABRETOOTH_TIGER,
    1,
    "9d923644-3137-4e72-93d3-231066314d9b",
    "Monte Michael Moore",
);

// 7ED 216 — Seismic Assault (reprint)
const SEISMIC_ASSAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::SEISMIC_ASSAULT,
    "d399a9ac-01da-44a4-8f81-084d41dfada8",
    "Greg Staples",
);

// 7ED 216★ — Seismic Assault (alternate printing)
const SEISMIC_ASSAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::SEISMIC_ASSAULT,
    1,
    "6a994998-11b1-46fd-a877-d0b67fa8f777",
    "Greg Staples",
);

// 7ED 217 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "a0017ebd-d672-4933-8136-d929737e5ecd",
    "Michael Koelsch",
);

// 7ED 217★ — Shatter (alternate printing)
const SHATTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SHATTER,
    1,
    "142a7f05-df30-4bac-801d-06f125e7365f",
    "Michael Koelsch",
);

// 7ED 218 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "7fec2b71-8fa9-4818-9c4f-5d2dcd2af495",
    "Donato Giancola",
);

// 7ED 218★ — Shivan Dragon (alternate printing)
const SHIVAN_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SHIVAN_DRAGON,
    1,
    "de98d9ad-d011-43de-92c8-f97037b42803",
    "Donato Giancola",
);

// 7ED 219 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::stronghold::SHOCK,
    "ea653772-a5fe-4416-bef3-fd41133371db",
    "Mike Sass",
);

// 7ED 219★ — Shock (alternate printing)
const SHOCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1998::stronghold::SHOCK,
    1,
    "297d1cef-c181-481b-912c-385b81efd972",
    "Mike Sass",
);

// 7ED 220 — Spitting Earth (reprint)
const SPITTING_EARTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::SPITTING_EARTH,
    "0db01746-2734-4686-b443-52de8e379bbe",
    "Michael Koelsch",
);

// 7ED 220★ — Spitting Earth (alternate printing)
const SPITTING_EARTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::SPITTING_EARTH,
    1,
    "724682ef-d591-4d99-8557-0c970a043bf7",
    "Michael Koelsch",
);

// 7ED 221 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "24b70f97-441c-41ae-ab10-22ddd7bff28d",
    "Tony Szczudlo",
);

// 7ED 221★ — Stone Rain (alternate printing)
const STONE_RAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::STONE_RAIN,
    1,
    "8bc5314f-bfc1-4960-8a1f-80291e81b7b9",
    "Tony Szczudlo",
);

// 7ED 222 — Storm Shaman (reprint)
const STORM_SHAMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::STORM_SHAMAN,
    "ae941462-9086-47e5-8c04-01e53195584f",
    "D. J. Cleland-Hura",
);

// 7ED 222★ — Storm Shaman (alternate printing)
const STORM_SHAMAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_all::STORM_SHAMAN,
    1,
    "54cee0af-29a3-48cc-8a81-c3dac65271d6",
    "D. J. Cleland-Hura",
);

// 7ED 223 — Sudden Impact (reprint)
const SUDDEN_IMPACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::SUDDEN_IMPACT,
    "ef178ad2-f0e1-4fbb-aada-dccc40463ece",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 223★ — Sudden Impact (alternate printing)
const SUDDEN_IMPACT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::SUDDEN_IMPACT,
    1,
    "c987fd13-c690-46f9-b3ed-d2fc052c45ff",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 224 — Trained Orgg (reprint)
const TRAINED_ORGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::TRAINED_ORGG,
    "14a83031-8b57-41d2-b586-bb4dcf16136a",
    "Alex Horley-Orlandelli",
);

// 7ED 224★ — Trained Orgg (alternate printing)
const TRAINED_ORGG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::starter_1999::TRAINED_ORGG,
    1,
    "dc81dc1a-6560-479b-8c6f-54d04b1853f7",
    "Alex Horley-Orlandelli",
);

// 7ED 225 — Tremor (reprint)
const TREMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::TREMOR,
    "b281c013-b35a-4c4a-aaee-b6f93968485c",
    "Michael Koelsch",
);

// 7ED 225★ — Tremor (alternate printing)
const TREMOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::TREMOR,
    1,
    "fd43f654-081f-4ebb-b82e-02a2a8d959fa",
    "Michael Koelsch",
);

// 7ED 226 — Volcanic Hammer (reprint)
const VOLCANIC_HAMMER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::VOLCANIC_HAMMER,
    "f8d93606-4864-4a5f-bcbf-8638211e979d",
    "Ben Thompson",
);

// 7ED 226★ — Volcanic Hammer (alternate printing)
const VOLCANIC_HAMMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::VOLCANIC_HAMMER,
    1,
    "4ecf796a-8d10-4c37-8fe3-789f99eb526b",
    "Ben Thompson",
);

// 7ED 227 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "35113d19-0d4a-4513-82f3-c8deaa1e4324",
    "Ron Spencer",
);

// 7ED 227★ — Wall of Fire (alternate printing)
const WALL_OF_FIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::WALL_OF_FIRE,
    1,
    "9ac8374f-abdf-495e-a11d-6dd7c30a21f7",
    "Ron Spencer",
);

// 7ED 228 — Wildfire (reprint)
const WILDFIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::WILDFIRE,
    "826fd527-9356-4eec-8542-781116f23eb7",
    "Ron Spencer",
);

// 7ED 228★ — Wildfire (alternate printing)
const WILDFIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::WILDFIRE,
    1,
    "997a6901-31f5-48d3-aa8d-44b34d5098ee",
    "Ron Spencer",
);

// 7ED 229 — Anaconda (reprint)
const ANACONDA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ANACONDA,
    "5eb01c40-8fd7-483f-a0da-e1a3db6c93ef",
    "John Gallagher",
);

// 7ED 229★ — Anaconda (alternate printing)
const ANACONDA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::ANACONDA,
    1,
    "2dccffce-5ebd-4aaa-be05-1c6537d211f4",
    "John Gallagher",
);

// 7ED 230 — Ancient Silverback (reprint)
const ANCIENT_SILVERBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::ANCIENT_SILVERBACK,
    "3f2f8a99-b01d-4d0a-bf1c-a3cf08fbc469",
    "Scott M. Fischer",
);

// 7ED 230★ — Ancient Silverback (alternate printing)
const ANCIENT_SILVERBACK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::ANCIENT_SILVERBACK,
    1,
    "70ebbf7d-30b6-4d57-a168-3b35c54fa8db",
    "Scott M. Fischer",
);

// 7ED 231 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BIRDS_OF_PARADISE,
    "a2985857-fee5-42a6-9b5d-e157ada52a03",
    "Edward P. Beard, Jr.",
);

// 7ED 231★ — Birds of Paradise (alternate printing)
const BIRDS_OF_PARADISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::BIRDS_OF_PARADISE,
    1,
    "ef7106d8-ec4f-4bc2-aa39-9a605b04cf88",
    "Edward P. Beard, Jr.",
);

// 7ED 232 — Blanchwood Armor (reprint)
const BLANCHWOOD_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::BLANCHWOOD_ARMOR,
    "97ee5083-078f-4e76-a172-8f2edf98aa80",
    "Paolo Parente",
);

// 7ED 232★ — Blanchwood Armor (alternate printing)
const BLANCHWOOD_ARMOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::BLANCHWOOD_ARMOR,
    1,
    "1205388c-dd0d-48c0-809f-420312e54cca",
    "Paolo Parente",
);

// 7ED 233 — Bull Hippo (reprint)
const BULL_HIPPO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BULL_HIPPO,
    "044785ad-0f05-4944-aab3-49236727078d",
    "Heather Hudson",
);

// 7ED 233★ — Bull Hippo (alternate printing)
const BULL_HIPPO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::BULL_HIPPO,
    1,
    "34f345c8-1c0f-4ad0-bb56-0caf2bbc158f",
    "Heather Hudson",
);

// 7ED 234 — Canopy Spider (reprint)
const CANOPY_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::CANOPY_SPIDER,
    "1e6a2b31-2601-4fdf-afc9-cceccf1b3379",
    "Mike Raabe",
);

// 7ED 234★ — Canopy Spider (alternate printing)
const CANOPY_SPIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::CANOPY_SPIDER,
    1,
    "c6ad0711-a36d-4a3f-bbc5-00c9d8f7c448",
    "Mike Raabe",
);

// 7ED 235 — Compost (reprint)
const COMPOST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::COMPOST,
    "91fc8eca-7549-45b5-b3db-89a58d7d2a4a",
    "Marc Fishman",
);

// 7ED 235★ — Compost (alternate printing)
const COMPOST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::COMPOST,
    1,
    "cedbed40-f29f-4d5a-adea-8e13680ae671",
    "Marc Fishman",
);

// 7ED 236 — Creeping Mold (reprint)
const CREEPING_MOLD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::CREEPING_MOLD,
    "e2b589a6-46d7-45a2-8352-dd04338192df",
    "Gary Ruddell",
);

// 7ED 236★ — Creeping Mold (alternate printing)
const CREEPING_MOLD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::CREEPING_MOLD,
    1,
    "a02cb174-60eb-47fa-8870-684629c084c4",
    "Gary Ruddell",
);

// 7ED 237 — Early Harvest (reprint)
const EARLY_HARVEST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::EARLY_HARVEST,
    "7dcbd047-7757-45ea-abe3-1064881ec90b",
    "Heather Hudson",
);

// 7ED 237★ — Early Harvest (alternate printing)
const EARLY_HARVEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::EARLY_HARVEST,
    1,
    "ffb10101-cbd5-4d79-9161-f518f34add11",
    "Heather Hudson",
);

// 7ED 238 — Elder Druid (reprint)
const ELDER_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ELDER_DRUID,
    "aa88ff5b-44df-40eb-b0bb-37936ae0d854",
    "Alan Pollack",
);

// 7ED 238★ — Elder Druid (alternate printing)
const ELDER_DRUID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::ELDER_DRUID,
    1,
    "19f1540e-58c4-4bec-abc8-e3e2760fcb1f",
    "Alan Pollack",
);

// 7ED 239 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "0e8411c9-4f6f-4301-ac36-386016a32852",
    "Doug Chaffee",
);

// 7ED 239★ — Elvish Archers (alternate printing)
const ELVISH_ARCHERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ELVISH_ARCHERS,
    1,
    "66166a6c-3127-4f17-99fb-f469c3da92fb",
    "Doug Chaffee",
);

// 7ED 240 — Elvish Champion (reprint)
const ELVISH_CHAMPION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_inv::ELVISH_CHAMPION,
    "61d00f31-d8fd-4272-87ba-6bcb65c609c6",
    "Paolo Parente",
);

// 7ED 240★ — Elvish Champion (alternate printing)
const ELVISH_CHAMPION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_inv::ELVISH_CHAMPION,
    1,
    "6812ca1e-10d0-43ae-a6fc-5ab801539ec9",
    "Paolo Parente",
);

// 7ED 241 — Elvish Lyrist (reprint)
const ELVISH_LYRIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::ELVISH_LYRIST,
    "40bab3e7-f2c8-4025-8463-9e4de10091e7",
    "Michael Koelsch",
);

// 7ED 241★ — Elvish Lyrist (alternate printing)
const ELVISH_LYRIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::ELVISH_LYRIST,
    1,
    "7364e2f8-df18-4710-a588-00f1f00f22da",
    "Michael Koelsch",
);

// 7ED 242 — Elvish Piper (reprint)
const ELVISH_PIPER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::ELVISH_PIPER,
    "89476260-19b1-495c-b23e-6f206483e84a",
    "Tristan Elwell",
);

// 7ED 242★ — Elvish Piper (alternate printing)
const ELVISH_PIPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::ELVISH_PIPER,
    1,
    "f7335452-36cc-4d76-bde5-4ca8761dc94d",
    "Tristan Elwell",
);

// 7ED 243 — Familiar Ground (reprint)
const FAMILIAR_GROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::FAMILIAR_GROUND,
    "a26e990d-726c-4b3b-84b4-9c15a5c4be8c",
    "Thomas Gianni",
);

// 7ED 243★ — Familiar Ground (alternate printing)
const FAMILIAR_GROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::FAMILIAR_GROUND,
    1,
    "e5be1074-fd40-46ba-b3bb-fb3207d2d41b",
    "Thomas Gianni",
);

// 7ED 244 — Femeref Archers (reprint)
const FEMEREF_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FEMEREF_ARCHERS,
    "5ed8a169-1f32-486c-9dfb-aa13fcf3c984",
    "Gary Ruddell",
);

// 7ED 244★ — Femeref Archers (alternate printing)
const FEMEREF_ARCHERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::FEMEREF_ARCHERS,
    1,
    "fde1096b-96b5-47c3-93ad-f36bdceb326e",
    "Gary Ruddell",
);

// 7ED 245 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "6213c038-d231-4e61-b0ec-d1e39637e5c3",
    "Arnie Swekel",
);

// 7ED 245★ — Fog (alternate printing)
const FOG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOG,
    1,
    "492699b8-0d03-474b-9b0f-569440387fb6",
    "Arnie Swekel",
);

// 7ED 246 — Fyndhorn Elder (reprint)
const FYNDHORN_ELDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FYNDHORN_ELDER,
    "7b1c3e8c-da95-4d27-9c60-5a2cdcff0b71",
    "Greg Staples",
);

// 7ED 246★ — Fyndhorn Elder (alternate printing)
const FYNDHORN_ELDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::FYNDHORN_ELDER,
    1,
    "90d03d69-2f6f-4c72-93b5-10bb8a278e24",
    "Greg Staples",
);

// 7ED 247 — Gang of Elk (reprint)
const GANG_OF_ELK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::GANG_OF_ELK,
    "cd0a61c9-8b14-4255-8453-4b74d90fe0a3",
    "Thomas Gianni",
);

// 7ED 247★ — Gang of Elk (alternate printing)
const GANG_OF_ELK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::GANG_OF_ELK,
    1,
    "91350320-6a51-42f0-b644-bc5ce2a80505",
    "Thomas Gianni",
);

// 7ED 248 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "9ade9b45-a1f5-4680-8d26-d2ae5879b1b6",
    "Terese Nielsen",
);

// 7ED 248★ — Giant Growth (alternate printing)
const GIANT_GROWTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::GIANT_GROWTH,
    1,
    "6376a82b-ca41-4a75-9d8c-396477b2f340",
    "Terese Nielsen",
);

// 7ED 249 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "c1bcde1e-d379-4b0c-8e59-01fbb3217f04",
    "Ray Lago",
);

// 7ED 249★ — Giant Spider (alternate printing)
const GIANT_SPIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::GIANT_SPIDER,
    1,
    "717ad85b-162e-4dfb-99e5-97ef038cc69d",
    "Ray Lago",
);

// 7ED 250 — Gorilla Chieftain (reprint)
const GORILLA_CHIEFTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::GORILLA_CHIEFTAIN,
    "9e76185a-519f-4bec-b399-989ebddbab71",
    "Carl Critchlow",
);

// 7ED 250★ — Gorilla Chieftain (alternate printing)
const GORILLA_CHIEFTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_all::GORILLA_CHIEFTAIN,
    1,
    "fb762ef0-59e9-4c64-b3cb-0f2596059307",
    "Carl Critchlow",
);

// 7ED 251 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "5b8dadf2-d31a-4b24-a9a7-f7f511ba2867",
    "D. J. Cleland-Hura",
);

// 7ED 251★ — Grizzly Bears (alternate printing)
const GRIZZLY_BEARS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::GRIZZLY_BEARS,
    1,
    "66e3de06-572e-48c5-888b-2dac7fa9d0d0",
    "D. J. Cleland-Hura",
);

// 7ED 252 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "f0526077-79b6-40ae-8178-8b97c33a53fb",
    "John Howe",
);

// 7ED 252★ — Hurricane (alternate printing)
const HURRICANE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::HURRICANE,
    1,
    "96005f68-1712-4818-ad31-a0e629c54d36",
    "John Howe",
);

// 7ED 253 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LLANOWAR_ELVES,
    "ed8a8cfd-baef-4198-b1f3-4926139588b2",
    "Jerry Tiritilli",
);

// 7ED 253★ — Llanowar Elves (alternate printing)
const LLANOWAR_ELVES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::LLANOWAR_ELVES,
    1,
    "6fca5b76-2e0b-4557-91c6-283000d17849",
    "Jerry Tiritilli",
);

// 7ED 254 — Lone Wolf (reprint)
const LONE_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::LONE_WOLF,
    "67f885c7-4947-4041-ab82-b5e8dc167f0d",
    "Douglas Shuler",
);

// 7ED 254★ — Lone Wolf (alternate printing)
const LONE_WOLF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::LONE_WOLF,
    1,
    "afae9668-4131-4cf7-acf0-6e5684165896",
    "Douglas Shuler",
);

// 7ED 255 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "e0ebfea3-e671-4c75-b0a2-c310d07351a6",
    "Larry Elmore",
);

// 7ED 255★ — Lure (alternate printing)
const LURE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::LURE,
    1,
    "9840d08e-cc99-4fe5-ad60-43833bf7d9eb",
    "Larry Elmore",
);

// 7ED 256 — Maro (reprint)
const MARO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MARO,
    "b4c3853c-8b41-4bce-b4e0-3824fc5888c4",
    "Pete Venters",
);

// 7ED 256★ — Maro (alternate printing)
const MARO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::MARO,
    1,
    "f6cc6b4e-a4a0-4658-ad96-1c70d0dd1297",
    "Pete Venters",
);

// 7ED 257 — Might of Oaks (reprint)
const MIGHT_OF_OAKS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::MIGHT_OF_OAKS,
    "b1bab68d-da36-43b9-9778-d385d81a0bc5",
    "Greg Staples",
);

// 7ED 257★ — Might of Oaks (alternate printing)
const MIGHT_OF_OAKS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::MIGHT_OF_OAKS,
    1,
    "cb5a9e47-57a1-45f8-978c-48ef90703832",
    "Greg Staples",
);

// 7ED 258 — Monstrous Growth (reprint)
const MONSTROUS_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::MONSTROUS_GROWTH,
    "eb56633e-692c-41bc-9253-ebd1528f4e99",
    "Ron Spencer",
);

// 7ED 258★ — Monstrous Growth (alternate printing)
const MONSTROUS_GROWTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::MONSTROUS_GROWTH,
    1,
    "0d911589-f2e8-4d9a-b339-5a5597a783fd",
    "Ron Spencer",
);

// 7ED 259 — Nature's Resurgence (reprint)
const NATURE_S_RESURGENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::NATURE_S_RESURGENCE,
    "287f9f55-829d-4b29-b1d2-34d20d23b3d5",
    "Gary Ruddell",
);

// 7ED 259★ — Nature's Resurgence (alternate printing)
const NATURE_S_RESURGENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wth::NATURE_S_RESURGENCE,
    1,
    "a8cf5760-5da1-4c17-859e-baddd0ec62ae",
    "Gary Ruddell",
);

// 7ED 260 — Nature's Revolt (reprint)
const NATURE_S_REVOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::NATURE_S_REVOLT,
    "e0c9b948-63f9-458d-82ae-69ebe2ac9fe0",
    "Dave Dorman",
);

// 7ED 260★ — Nature's Revolt (alternate printing)
const NATURE_S_REVOLT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::NATURE_S_REVOLT,
    1,
    "1e3f2a56-ec54-4983-9f09-aea67becec68",
    "Dave Dorman",
);

// 7ED 261 — Pride of Lions (alternate printing)
const PRIDE_OF_LIONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::starter_1999::PRIDE_OF_LIONS,
    1,
    "65c3274b-3eb0-450a-88bf-fb378e6cf94a",
    "Gary Ruddell",
);

// 7ED 261★ — Pride of Lions (reprint)
const PRIDE_OF_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::PRIDE_OF_LIONS,
    "1673b038-97b6-4139-8468-9cbbd01dd239",
    "Gary Ruddell",
);

// 7ED 262 — Rampant Growth (reprint)
const RAMPANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::RAMPANT_GROWTH,
    "c305376d-fcfb-48a1-947f-a9eec0dbc610",
    "Scott M. Fischer",
);

// 7ED 262★ — Rampant Growth (alternate printing)
const RAMPANT_GROWTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1996::mirage::RAMPANT_GROWTH,
    1,
    "afb685db-bb38-4b78-ae1e-80817c852096",
    "Scott M. Fischer",
);

// 7ED 263 — Reclaim (reprint)
const RECLAIM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::exodus::RECLAIM,
    "045e9ca4-ece8-49c2-b022-fb61f4b8b635",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 263★ — Reclaim (alternate printing)
const RECLAIM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1998::exodus::RECLAIM,
    1,
    "160e5d9e-b390-4a77-9435-f6cdc95ca728",
    "Greg Hildebrandt & Tim Hildebrandt",
);

// 7ED 264 — Redwood Treefolk (reprint)
const REDWOOD_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::REDWOOD_TREEFOLK,
    "9cc6d29d-2915-418d-856f-13b05430dfda",
    "D. J. Cleland-Hura",
);

// 7ED 264★ — Redwood Treefolk (alternate printing)
const REDWOOD_TREEFOLK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::REDWOOD_TREEFOLK,
    1,
    "fb172a6d-0f4f-42bf-8add-bec0e00e8a66",
    "D. J. Cleland-Hura",
);

// 7ED 265 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "6d9ea671-f3e9-4e17-b98a-51fac48f875e",
    "Adam Rex",
);

// 7ED 265★ — Regeneration (alternate printing)
const REGENERATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::REGENERATION,
    1,
    "20fe41ad-722f-4330-b1ee-c6ca3f81ce86",
    "Adam Rex",
);

// 7ED 266 — Rowen (reprint)
const ROWEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::ROWEN,
    "bbc536ff-3e12-4e1b-b96f-b0c32dcb6734",
    "Franz Vohwinkel",
);

// 7ED 266★ — Rowen (alternate printing)
const ROWEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::ROWEN,
    1,
    "0a7e6a58-eda0-4224-b517-ad6dfb721c88",
    "Franz Vohwinkel",
);

// 7ED 267 — Scavenger Folk (reprint)
const SCAVENGER_FOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::SCAVENGER_FOLK,
    "7ff2dd57-9d8c-44ec-9705-70ed02bf0799",
    "Matt Cavotta",
);

// 7ED 267★ — Scavenger Folk (alternate printing)
const SCAVENGER_FOLK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::SCAVENGER_FOLK,
    1,
    "d9637b1e-3a78-4946-afd9-29d0d90df282",
    "Matt Cavotta",
);

// 7ED 268 — Seeker of Skybreak (reprint)
const SEEKER_OF_SKYBREAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::SEEKER_OF_SKYBREAK,
    "d1f20175-8bfa-417a-912b-d6d472f091ab",
    "Alex Horley-Orlandelli",
);

// 7ED 268★ — Seeker of Skybreak (alternate printing)
const SEEKER_OF_SKYBREAK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::SEEKER_OF_SKYBREAK,
    1,
    "3c2c491a-1f86-4e44-8dc3-b8ba65f4017c",
    "Alex Horley-Orlandelli",
);

// 7ED 269 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "90e8ff87-22e8-4c04-84bc-f0ea2c12a86c",
    "Eric Peterson",
);

// 7ED 269★ — Shanodin Dryads (alternate printing)
const SHANODIN_DRYADS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SHANODIN_DRYADS,
    1,
    "f19f7690-87bf-4259-adc1-7e69cc3081a9",
    "Eric Peterson",
);

// 7ED 270 — Spined Wurm (reprint)
const SPINED_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::SPINED_WURM,
    "de6cb158-f66f-429b-ac73-bfea87f51def",
    "Carl Critchlow",
);

// 7ED 270★ — Spined Wurm (alternate printing)
const SPINED_WURM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::SPINED_WURM,
    1,
    "f54a9970-cffa-43a4-8eeb-4330a3ea2e82",
    "Carl Critchlow",
);

// 7ED 271 — Squall (reprint)
const SQUALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::starter_1999::SQUALL,
    "46460e5f-2756-486b-99a6-c3a9a209bfaa",
    "Greg Staples",
);

// 7ED 271★ — Squall (alternate printing)
const SQUALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1999::starter_1999::SQUALL,
    1,
    "b67e4e64-adf5-480c-b47b-c8eb03cb74c4",
    "Greg Staples",
);

// 7ED 272 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "4405ab25-c001-4da8-ac72-2afcb01db200",
    "Andrew Goldhawk",
);

// 7ED 272★ — Stream of Life (alternate printing)
const STREAM_OF_LIFE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::STREAM_OF_LIFE,
    1,
    "ce87747f-4565-43f2-b966-8bef41ebbe03",
    "Andrew Goldhawk",
);

// 7ED 273 — Thorn Elemental (reprint)
const THORN_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::THORN_ELEMENTAL,
    "e06bea52-3db1-4b61-8418-77ace9cd70b5",
    "rk post",
);

// 7ED 273★ — Thorn Elemental (alternate printing)
const THORN_ELEMENTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::THORN_ELEMENTAL,
    1,
    "9fd239e3-dbe5-4a30-973d-9707de425a33",
    "rk post",
);

// 7ED 274 — Thoughtleech (reprint)
const THOUGHTLEECH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::THOUGHTLEECH,
    "95e214df-93a2-4d21-8818-cc00bff1b318",
    "Rebecca Guay",
);

// 7ED 274★ — Thoughtleech (alternate printing)
const THOUGHTLEECH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::THOUGHTLEECH,
    1,
    "8f617305-8043-4576-bc79-67303b91bb69",
    "Rebecca Guay",
);

// 7ED 275 — Trained Armodon (reprint)
const TRAINED_ARMODON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::TRAINED_ARMODON,
    "bc7c1774-908f-43ab-b589-e46606267381",
    "John Matson",
);

// 7ED 275★ — Trained Armodon (alternate printing)
const TRAINED_ARMODON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::TRAINED_ARMODON,
    1,
    "05381dc1-50e1-4ac4-8f35-46d0b2d0b42d",
    "John Matson",
);

// 7ED 276 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "22b3f289-7999-4e47-82c6-a7c96293d4f8",
    "John Matson",
);

// 7ED 276★ — Tranquility (alternate printing)
const TRANQUILITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::TRANQUILITY,
    1,
    "1cf477ae-fc28-4c50-a41c-be6bfab489e4",
    "John Matson",
);

// 7ED 277 — Treefolk Seedlings (reprint)
const TREEFOLK_SEEDLINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::TREEFOLK_SEEDLINGS,
    "4c275bdc-e960-4b6f-a8d3-b662739113c1",
    "Don Hazeltine",
);

// 7ED 277★ — Treefolk Seedlings (alternate printing)
const TREEFOLK_SEEDLINGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::TREEFOLK_SEEDLINGS,
    1,
    "ecf9a759-13aa-4a8e-bb53-3706c8c69bd0",
    "Don Hazeltine",
);

// 7ED 278 — Uktabi Wildcats (reprint)
const UKTABI_WILDCATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::UKTABI_WILDCATS,
    "700bb560-5a4a-4422-9158-1c2edad1913f",
    "Thomas Gianni",
);

// 7ED 278★ — Uktabi Wildcats (alternate printing)
const UKTABI_WILDCATS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::UKTABI_WILDCATS,
    1,
    "b23d1a8c-01cb-4dd3-8562-c3ce7bc88d77",
    "Thomas Gianni",
);

// 7ED 279 — Untamed Wilds (reprint)
const UNTAMED_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::UNTAMED_WILDS,
    "ba496182-b249-40fa-8fdf-9823d521fcd9",
    "Thomas Gianni",
);

// 7ED 279★ — Untamed Wilds (alternate printing)
const UNTAMED_WILDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::UNTAMED_WILDS,
    1,
    "dbc908d9-2a08-4465-81cd-11199685c660",
    "Thomas Gianni",
);

// 7ED 280 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "da3f99fe-8d5f-4efe-af86-72031dfe562a",
    "Rob Alexander",
);

// 7ED 280★ — Verduran Enchantress (alternate printing)
const VERDURAN_ENCHANTRESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    1,
    "78b87a7f-41d0-4dd7-b479-f6bcccda54be",
    "Rob Alexander",
);

// 7ED 281 — Vernal Bloom (reprint)
const VERNAL_BLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::VERNAL_BLOOM,
    "7f0368c8-8021-40c3-b42d-7320d956a84f",
    "Scott Bailey",
);

// 7ED 281★ — Vernal Bloom (alternate printing)
const VERNAL_BLOOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::VERNAL_BLOOM,
    1,
    "c79ed17b-2a71-4146-8dd7-75faefdfa4fa",
    "Scott Bailey",
);

// 7ED 282 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "515daaba-c063-41d5-9539-25b8c8cb639c",
    "Tony Szczudlo",
);

// 7ED 282★ — Wild Growth (alternate printing)
const WILD_GROWTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::WILD_GROWTH,
    1,
    "c9a02887-beb9-454b-8592-f551f48cd93c",
    "Tony Szczudlo",
);

// 7ED 283 — Wing Snare (reprint)
const WING_SNARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::WING_SNARE,
    "d37ba325-5a14-473b-9def-6a4660a50d7a",
    "Daren Bader",
);

// 7ED 283★ — Wing Snare (alternate printing)
const WING_SNARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::WING_SNARE,
    1,
    "ad6a0916-c077-427f-983d-c595e8256507",
    "Daren Bader",
);

// 7ED 284 — Wood Elves (reprint)
const WOOD_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::WOOD_ELVES,
    "263feecf-a657-4892-a2bb-cd7080d283c2",
    "Christopher Moeller",
);

// 7ED 284★ — Wood Elves (alternate printing)
const WOOD_ELVES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::WOOD_ELVES,
    1,
    "38082344-f739-495d-947a-74e247b17433",
    "Christopher Moeller",
);

// 7ED 285 — Yavimaya Enchantress (reprint)
const YAVIMAYA_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::YAVIMAYA_ENCHANTRESS,
    "e41f45f1-9ed1-4ef2-8c2b-bab513d6a721",
    "Terese Nielsen",
);

// 7ED 285★ — Yavimaya Enchantress (alternate printing)
const YAVIMAYA_ENCHANTRESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::YAVIMAYA_ENCHANTRESS,
    1,
    "94ec6cdd-11ba-4acd-9bbc-87f01f703642",
    "Terese Nielsen",
);

// 7ED 286 — Aladdin's Ring (reprint)
const ALADDINS_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDINS_RING,
    "ed511375-e445-4d81-818d-92a793d7deee",
    "Dave Dorman",
);

// 7ED 286★ — Aladdin's Ring (alternate printing)
const ALADDINS_RING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_arn::ALADDINS_RING,
    1,
    "75da20cc-2737-4a03-a75c-ed4f7b63cf95",
    "Dave Dorman",
);

// 7ED 287 — Beast of Burden (reprint)
const BEAST_OF_BURDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::BEAST_OF_BURDEN,
    "677c3b59-bd04-4376-aec1-a77404c6072d",
    "Chippy",
);

// 7ED 287★ — Beast of Burden (alternate printing)
const BEAST_OF_BURDEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ulg::BEAST_OF_BURDEN,
    1,
    "53f92d6f-1d3a-495f-841d-75c6a046e27c",
    "Chippy",
);

// 7ED 288 — Caltrops (reprint)
const CALTROPS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::CALTROPS,
    "8769b55a-a0a1-4b6f-8c80-669385a34425",
    "Eric Peterson",
);

// 7ED 288★ — Caltrops (alternate printing)
const CALTROPS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_uds::CALTROPS,
    1,
    "fdf0d1da-7c16-4a9a-a71e-4805bd0c0995",
    "Eric Peterson",
);

// 7ED 289 — Charcoal Diamond (reprint)
const CHARCOAL_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::CHARCOAL_DIAMOND,
    "705c1d1d-e6e0-47fe-b642-b1ff0201bbf9",
    "David Martin",
);

// 7ED 289s — Charcoal Diamond (alternate printing)
const CHARCOAL_DIAMOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::CHARCOAL_DIAMOND,
    1,
    "eaab01ef-f14b-4f42-ab51-b1537fa642d1",
    "David Martin",
);

// 7ED 289★ — Charcoal Diamond (alternate printing)
const CHARCOAL_DIAMOND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::CHARCOAL_DIAMOND,
    2,
    "34a0ed0c-7092-4c5e-9b06-66ab4de157c6",
    "David Martin",
);

// 7ED 289★s — Charcoal Diamond (alternate printing)
const CHARCOAL_DIAMOND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::CHARCOAL_DIAMOND,
    3,
    "a52ecee5-ad2d-4f52-9845-f682119ee120",
    "David Martin",
);

// 7ED 290 — Coat of Arms (reprint)
const COAT_OF_ARMS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::COAT_OF_ARMS,
    "9642852b-8736-4fce-9f91-37594cbc3f71",
    "D. Alexander Gregory",
);

// 7ED 290★ — Coat of Arms (alternate printing)
const COAT_OF_ARMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::COAT_OF_ARMS,
    1,
    "b38ecdea-c269-4f0a-b297-e16a22410802",
    "D. Alexander Gregory",
);

// 7ED 291 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "2017cded-4157-4ccb-80f0-e298aa89d17d",
    "Ciruelo",
);

// 7ED 291★ — Crystal Rod (alternate printing)
const CRYSTAL_ROD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::CRYSTAL_ROD,
    1,
    "2fab1f2d-445e-4feb-93de-c59e2655fa1d",
    "Ciruelo",
);

// 7ED 292 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "8534792b-96a7-45a6-b854-81066a2e5d90",
    "Tony Szczudlo",
);

// 7ED 292★ — Dingus Egg (alternate printing)
const DINGUS_EGG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DINGUS_EGG,
    1,
    "812171e0-bc15-4eb3-924a-1c779414469b",
    "Tony Szczudlo",
);

// 7ED 293 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "f94d6842-0b35-4493-8800-cf2b2138d656",
    "Darrell Riche",
);

// 7ED 293★ — Disrupting Scepter (alternate printing)
const DISRUPTING_SCEPTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DISRUPTING_SCEPTER,
    1,
    "e3d47309-9595-4b19-bd9c-4973b87651eb",
    "Darrell Riche",
);

// 7ED 294 — Ensnaring Bridge (reprint)
const ENSNARING_BRIDGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::ENSNARING_BRIDGE,
    "97d4fb56-7f63-4087-9ab3-a0df66655886",
    "Ron Spencer",
);

// 7ED 294★ — Ensnaring Bridge (alternate printing)
const ENSNARING_BRIDGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_sth::ENSNARING_BRIDGE,
    1,
    "0752f333-002c-4c77-bb96-a7bd2dd4ff5e",
    "Ron Spencer",
);

// 7ED 295 — Feroz's Ban (reprint)
const FEROZ_S_BAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::FEROZ_S_BAN,
    "188b3863-fa7d-4fbb-b061-97580d394017",
    "Donato Giancola",
);

// 7ED 295★ — Feroz's Ban (alternate printing)
const FEROZ_S_BAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_hml::FEROZ_S_BAN,
    1,
    "0f1f6dea-3554-4649-966c-5f33229132d4",
    "Donato Giancola",
);

// 7ED 296 — Fire Diamond (reprint)
const FIRE_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FIRE_DIAMOND,
    "1a89ca2b-0cc9-421e-8dc0-1105879380a0",
    "David Martin",
);

// 7ED 296★ — Fire Diamond (alternate printing)
const FIRE_DIAMOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::FIRE_DIAMOND,
    1,
    "054ae635-1cc5-4367-9dd9-95acf8bfd937",
    "David Martin",
);

// 7ED 297 — Flying Carpet (reprint)
const FLYING_CARPET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::FLYING_CARPET,
    "7edcc350-ab84-4aff-a9f0-2cc05c7c43bb",
    "Scott M. Fischer",
);

// 7ED 297★ — Flying Carpet (alternate printing)
const FLYING_CARPET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_arn::FLYING_CARPET,
    1,
    "014b7a72-135e-4ed0-9b1c-0b836aaa260f",
    "Scott M. Fischer",
);

// 7ED 298 — Grafted Skullcap (reprint)
const GRAFTED_SKULLCAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::GRAFTED_SKULLCAP,
    "3ff78a37-c321-4f02-bd10-e73823b954cf",
    "Bradley Williams",
);

// 7ED 298★ — Grafted Skullcap (alternate printing)
const GRAFTED_SKULLCAP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::GRAFTED_SKULLCAP,
    1,
    "42ac035e-06dc-4b37-9b84-26ff0d671564",
    "Bradley Williams",
);

// 7ED 299 — Grapeshot Catapult (reprint)
const GRAPESHOT_CATAPULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::GRAPESHOT_CATAPULT,
    "07d63d66-5e59-4302-8b74-db1aa67f50c5",
    "Dave Dorman",
);

// 7ED 299★ — Grapeshot Catapult (alternate printing)
const GRAPESHOT_CATAPULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::GRAPESHOT_CATAPULT,
    1,
    "e0067413-338a-4a24-afe2-010a9915e73e",
    "Dave Dorman",
);

// 7ED 300 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "4dd270d3-da58-4195-80f4-ade6ae32d092",
    "Dana Knutson",
);

// 7ED 300★ — Howling Mine (alternate printing)
const HOWLING_MINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::HOWLING_MINE,
    1,
    "ec9d720e-68f9-464a-8b11-0f243f184ccf",
    "Dana Knutson",
);

// 7ED 301 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRON_STAR,
    "6da72ad2-1cdd-4505-b0f8-f036ac684776",
    "Pete Venters",
);

// 7ED 301★ — Iron Star (alternate printing)
const IRON_STAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::IRON_STAR,
    1,
    "09c1995f-a22e-4b3d-9d3e-f6ed6e75a3ea",
    "Pete Venters",
);

// 7ED 302 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "e27a54e2-ac65-479a-adfc-8a2edad61e81",
    "Alan Pollack",
);

// 7ED 302★ — Ivory Cup (alternate printing)
const IVORY_CUP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::IVORY_CUP,
    1,
    "3b89b134-f3b7-490a-ba79-d4cd5d82aadc",
    "Alan Pollack",
);

// 7ED 303 — Jalum Tome (reprint)
const JALUM_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::JALUM_TOME,
    "a0c76784-c4b2-48c8-9513-dd7196d27360",
    "Jerry Tiritilli",
);

// 7ED 303★ — Jalum Tome (alternate printing)
const JALUM_TOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::JALUM_TOME,
    1,
    "a38c5b83-449e-4503-a34e-94a302370606",
    "Jerry Tiritilli",
);

// 7ED 304 — Jandor's Saddlebags (reprint)
const JANDORS_SADDLEBAGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::JANDORS_SADDLEBAGS,
    "d4f8d1ec-a55c-409f-be42-ae1c0f8c66e1",
    "Brian Despain",
);

// 7ED 304★ — Jandor's Saddlebags (alternate printing)
const JANDORS_SADDLEBAGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_arn::JANDORS_SADDLEBAGS,
    1,
    "afb83a18-4efd-4e4e-99ad-bc6f09ddce6e",
    "Brian Despain",
);

// 7ED 305 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JAYEMDAE_TOME,
    "3bed9644-5dc1-4d1c-b60d-4bcf305f2d0b",
    "Donato Giancola",
);

// 7ED 305★ — Jayemdae Tome (alternate printing)
const JAYEMDAE_TOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::JAYEMDAE_TOME,
    1,
    "77d7ca08-92c3-42bb-98e3-d6d959d865d0",
    "Donato Giancola",
);

// 7ED 306 — Marble Diamond (reprint)
const MARBLE_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MARBLE_DIAMOND,
    "a90b2450-c814-47ee-8e85-d16e64d08af8",
    "David Martin",
);

// 7ED 306★ — Marble Diamond (alternate printing)
const MARBLE_DIAMOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::MARBLE_DIAMOND,
    1,
    "0e8dbea9-25ef-4048-999f-d62ec7e1a13f",
    "David Martin",
);

// 7ED 307 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "0a691664-9275-4b89-a8e3-b99c88717ffb",
    "David Martin",
);

// 7ED 307★ — Meekstone (alternate printing)
const MEEKSTONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MEEKSTONE,
    1,
    "d35fd53a-08ed-4f7c-937d-3dc6e834a481",
    "David Martin",
);

// 7ED 308 — Millstone (reprint)
const MILLSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::MILLSTONE,
    "1da21381-38bf-49d0-98d3-81eb9c99ce82",
    "John Avon",
);

// 7ED 308★ — Millstone (alternate printing)
const MILLSTONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::MILLSTONE,
    1,
    "d2d361c3-8aa7-4b3c-ae68-15910cc341ca",
    "John Avon",
);

// 7ED 309 — Moss Diamond (reprint)
const MOSS_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MOSS_DIAMOND,
    "9b4de9d3-56b2-40d9-adca-4fd2dadaaad9",
    "David Martin",
);

// 7ED 309★ — Moss Diamond (alternate printing)
const MOSS_DIAMOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::MOSS_DIAMOND,
    1,
    "ae76fd66-e91d-4f31-b393-465e9a7b795b",
    "David Martin",
);

// 7ED 310 — Patagia Golem (reprint)
const PATAGIA_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::PATAGIA_GOLEM,
    "90583dcc-6e0e-45e6-9209-8ad3a6c94555",
    "Kev Walker",
);

// 7ED 310★ — Patagia Golem (alternate printing)
const PATAGIA_GOLEM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::PATAGIA_GOLEM,
    1,
    "eb501ad8-d2b2-454b-bda3-c28c618f2d4f",
    "Kev Walker",
);

// 7ED 311 — Phyrexian Colossus (reprint)
const PHYREXIAN_COLOSSUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::PHYREXIAN_COLOSSUS,
    "887b377f-4080-4cfa-a5a7-bab32b6891f9",
    "Mark Tedin",
);

// 7ED 311★ — Phyrexian Colossus (alternate printing)
const PHYREXIAN_COLOSSUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::PHYREXIAN_COLOSSUS,
    1,
    "4c279c5b-70c3-426a-86ad-2de1a8771ae4",
    "Mark Tedin",
);

// 7ED 312 — Phyrexian Hulk (reprint)
const PHYREXIAN_HULK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::tempest::PHYREXIAN_HULK,
    "22e59c8b-735d-4796-b37c-619ee1782f65",
    "Brian Snõddy",
);

// 7ED 312★ — Phyrexian Hulk (alternate printing)
const PHYREXIAN_HULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::tempest::PHYREXIAN_HULK,
    1,
    "ba2b3869-7305-49f9-9d21-795c94cc83f5",
    "Brian Snõddy",
);

// 7ED 313 — Pit Trap (reprint)
const PIT_TRAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PIT_TRAP,
    "b6fc051a-df87-46d9-bc45-8b7b2721b374",
    "Nelson DeCastro",
);

// 7ED 313★ — Pit Trap (alternate printing)
const PIT_TRAP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::PIT_TRAP,
    1,
    "b4e1d888-43e5-4476-98be-e251090cf662",
    "Nelson DeCastro",
);

// 7ED 314 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "68498f5c-4924-4a56-b6a7-40c5fb2d31c1",
    "Ciruelo",
);

// 7ED 314★ — Rod of Ruin (alternate printing)
const ROD_OF_RUIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ROD_OF_RUIN,
    1,
    "754d90bd-d08d-4510-ad3c-f6ed52a92d6c",
    "Ciruelo",
);

// 7ED 315 — Sisay's Ring (reprint)
const SISAY_S_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::SISAY_S_RING,
    "1ae27220-2b62-4214-81b5-612dd770612b",
    "David Martin",
);

// 7ED 315★ — Sisay's Ring (alternate printing)
const SISAY_S_RING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::SISAY_S_RING,
    1,
    "ba8383ff-76e5-43b3-99ac-8bdae9830aca",
    "David Martin",
);

// 7ED 316 — Sky Diamond (reprint)
const SKY_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::SKY_DIAMOND,
    "b426b86d-b1d9-4aee-ac17-4eae7a3b69a3",
    "Tony Szczudlo",
);

// 7ED 316★ — Sky Diamond (alternate printing)
const SKY_DIAMOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mir::SKY_DIAMOND,
    1,
    "f8e50b64-3527-4d28-92a0-a832436b6c81",
    "Tony Szczudlo",
);

// 7ED 317 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "62e267df-22e1-422e-973b-d192b40d5f19",
    "Ron Spencer",
);

// 7ED 317★ — Soul Net (alternate printing)
const SOUL_NET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SOUL_NET,
    1,
    "c9454461-c2b4-45e0-8c99-ac8ea506e267",
    "Ron Spencer",
);

// 7ED 318 — Spellbook (reprint)
const SPELLBOOK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::SPELLBOOK,
    "fb1b6da4-89f8-442e-9c7a-2f60c180ef87",
    "Andrew Goldhawk",
);

// 7ED 318★ — Spellbook (alternate printing)
const SPELLBOOK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_exo::SPELLBOOK,
    1,
    "a55b4d06-43de-438a-8e75-736243b188e4",
    "Andrew Goldhawk",
);

// 7ED 319 — Static Orb (reprint)
const STATIC_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::STATIC_ORB,
    "86bf43b1-8d4e-4759-bb2d-0b2e03ba7012",
    "Terese Nielsen",
);

// 7ED 319★ — Static Orb (alternate printing)
const STATIC_ORB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tmp::STATIC_ORB,
    1,
    "b69e0877-c792-45f2-9cc3-d86eaa90d85c",
    "Terese Nielsen",
);

// 7ED 320 — Storm Cauldron (reprint)
const STORM_CAULDRON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::STORM_CAULDRON,
    "0bb5bdd3-6ecd-49cd-bfa2-e7da1ee85d88",
    "Doug Chaffee",
);

// 7ED 320★ — Storm Cauldron (alternate printing)
const STORM_CAULDRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_all::STORM_CAULDRON,
    1,
    "5a458f9a-bde3-40ee-847d-aca216d215d4",
    "Doug Chaffee",
);

// 7ED 321 — Teferi's Puzzle Box (reprint)
const TEFERI_S_PUZZLE_BOX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::TEFERI_S_PUZZLE_BOX,
    "7e5463b9-2088-4ecb-acc3-c00ef54648af",
    "Donato Giancola",
);

// 7ED 321★ — Teferi's Puzzle Box (alternate printing)
const TEFERI_S_PUZZLE_BOX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vis::TEFERI_S_PUZZLE_BOX,
    1,
    "5a784cae-4c1e-4813-8d13-3c435698f446",
    "Donato Giancola",
);

// 7ED 322 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "3e90f7cf-efcb-48cc-b725-2f5fb6e37da9",
    "Ron Spears",
);

// 7ED 322★ — Throne of Bone (alternate printing)
const THRONE_OF_BONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::THRONE_OF_BONE,
    1,
    "4337eed4-c219-4844-9620-980d2430a9bf",
    "Ron Spears",
);

// 7ED 323 — Wall of Spears (reprint)
const WALL_OF_SPEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::WALL_OF_SPEARS,
    "5c94f13f-6e7a-4bb1-a9a0-5c885e231d5c",
    "Christopher Moeller",
);

// 7ED 323★ — Wall of Spears (alternate printing)
const WALL_OF_SPEARS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::WALL_OF_SPEARS,
    1,
    "809e9c79-4acc-4933-8da5-993fd71164c4",
    "Christopher Moeller",
);

// 7ED 324 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "8e04657c-9a09-4e69-a884-39e083ad22b8",
    "Terese Nielsen",
);

// 7ED 324★ — Wooden Sphere (alternate printing)
const WOODEN_SPHERE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::WOODEN_SPHERE,
    1,
    "d452f2c4-1500-4301-8278-19922026fcaa",
    "Terese Nielsen",
);

// 7ED 325 — Adarkar Wastes (reprint)
const ADARKAR_WASTES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ADARKAR_WASTES,
    "9bd991b2-8c18-4c5f-9b70-461012fee61e",
    "John Avon",
);

// 7ED 325★ — Adarkar Wastes (alternate printing)
const ADARKAR_WASTES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::ADARKAR_WASTES,
    1,
    "ae0e5ea0-46f5-4a6c-8fa7-f851c97cb075",
    "John Avon",
);

// 7ED 326 — Brushland (reprint)
const BRUSHLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BRUSHLAND,
    "8b983b45-e8be-49e1-84c1-cec204395264",
    "Scott Bailey",
);

// 7ED 326★ — Brushland (alternate printing)
const BRUSHLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::BRUSHLAND,
    1,
    "3417c159-f524-42fe-93c4-f729cea41341",
    "Scott Bailey",
);

// 7ED 327 — City of Brass (reprint)
const CITY_OF_BRASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &arabian_nights::CITY_OF_BRASS,
    "2ac29c5c-3c55-4778-9bcd-642d38a0d3f9",
    "Ron Walotsky",
);

// 7ED 327★ — City of Brass (alternate printing)
const CITY_OF_BRASS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_arn::CITY_OF_BRASS,
    1,
    "537af4fa-001c-4943-be6f-780d15b0584f",
    "Ron Walotsky",
);

// 7ED 328 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "a8ba6d68-19da-4b85-9852-681c0fc1e400",
    "D. J. Cleland-Hura",
);

// 7ED 328★ — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "9730c936-a1fc-484d-a9ce-a91804a84609",
    "D. J. Cleland-Hura",
);

// 7ED 329 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "1def1ac1-652c-4160-9092-46549e577570",
    "Rob Alexander",
);

// 7ED 329★ — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "ddfd1de7-60d3-4bf3-8909-8f4fa5045479",
    "Rob Alexander",
);

// 7ED 330 — Forest (alternate printing)
const FOREST_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    4,
    "172e55c6-b797-4b49-94f1-cc93e4f5b939",
    "John Avon",
);

// 7ED 330★ — Forest (alternate printing)
const FOREST_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    5,
    "c75b7404-ae5a-4921-b70b-775deeb2c984",
    "John Avon",
);

// 7ED 331 — Forest (alternate printing)
const FOREST_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    6,
    "a69545e9-7219-43f1-8a31-5035d87dca68",
    "John Avon",
);

// 7ED 331★ — Forest (alternate printing)
const FOREST_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    7,
    "b4321609-4f11-4066-9820-cbb483e9214a",
    "John Avon",
);

// 7ED 332 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "a45610b3-9e9b-4ef9-aab2-426f85a6cce3",
    "Scott Bailey",
);

// 7ED 332★ — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "51487402-2a52-4117-ba88-327e863e5f56",
    "Scott Bailey",
);

// 7ED 333 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "28a0a9ae-a1ae-41da-8308-d49e4afca3c1",
    "Rob Alexander",
);

// 7ED 333★ — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "26b02969-1959-4422-8e10-63ffb23192a2",
    "Rob Alexander",
);

// 7ED 334 — Island (alternate printing)
const ISLAND_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    4,
    "cfb83039-9a06-4d79-abcc-6f36eec6f29e",
    "John Avon",
);

// 7ED 334★ — Island (alternate printing)
const ISLAND_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    5,
    "fea43ef7-99c5-40b6-9a59-9661b3b710cf",
    "John Avon",
);

// 7ED 335 — Island (alternate printing)
const ISLAND_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    6,
    "ff68b640-e194-4894-b9c0-76ed619c764d",
    "Tony Szczudlo",
);

// 7ED 335★ — Island (alternate printing)
const ISLAND_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    7,
    "f23b716f-51b2-402a-9a69-de190d3f6411",
    "Tony Szczudlo",
);

// 7ED 336 — Karplusan Forest (reprint)
const KARPLUSAN_FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KARPLUSAN_FOREST,
    "4c393b6f-2421-4d77-9025-dac9dfaaae36",
    "Scott Bailey",
);

// 7ED 336★ — Karplusan Forest (alternate printing)
const KARPLUSAN_FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::KARPLUSAN_FOREST,
    1,
    "c388f490-ee8d-495b-b400-9f83916f0fed",
    "Scott Bailey",
);

// 7ED 337 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "fe14fc96-f33a-4bd2-8b04-c339936d3c24",
    "D. J. Cleland-Hura",
);

// 7ED 337★ — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "16328444-5657-4f35-99c5-340d61472770",
    "D. J. Cleland-Hura",
);

// 7ED 338 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "01534212-5a65-4088-b45b-0823736a8fd5",
    "Rob Alexander",
);

// 7ED 338★ — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "5af90ad7-25e9-4a5c-9169-38f33d7640a6",
    "Rob Alexander",
);

// 7ED 339 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    4,
    "ae41792d-4aa0-42ae-a9e1-9261cf9181a8",
    "Rob Alexander",
);

// 7ED 339★ — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    5,
    "27ade9a7-2a2e-4ec9-aa9c-20bc8caaf8a3",
    "Rob Alexander",
);

// 7ED 340 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    6,
    "ff951c38-6e02-40f0-9f54-13ac0915332b",
    "John Avon",
);

// 7ED 340★ — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    7,
    "76f30404-4d66-4aa0-81d3-351820854eda",
    "John Avon",
);

// 7ED 341 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "4edb482d-6279-48d6-baa3-047b48c3e5df",
    "Scott Bailey",
);

// 7ED 341★ — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "301a6a9b-c91d-4337-bc8c-411a50793385",
    "Scott Bailey",
);

// 7ED 342 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "1d13dbd6-6c6e-4120-a05a-e77f1e863b35",
    "Rob Alexander",
);

// 7ED 342★ — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "3ddcb7af-1f99-427f-971d-dd16b42d1914",
    "Rob Alexander",
);

// 7ED 343 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "1abb3ffc-9471-42c7-a1f4-66b6a8f9672c",
    "Rob Alexander",
);

// 7ED 343★ — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    5,
    "0a8bfc4f-ba82-462a-a5e6-5717de69414b",
    "Rob Alexander",
);

// 7ED 344 — Plains (alternate printing)
const PLAINS_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    6,
    "43559560-354d-4147-977f-766b58bca9fa",
    "John Avon",
);

// 7ED 344★ — Plains (alternate printing)
const PLAINS_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    7,
    "4b7604bd-b3b0-4e55-91c4-3717bae7e457",
    "John Avon",
);

// 7ED 345 — Sulfurous Springs (reprint)
const SULFUROUS_SPRINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SULFUROUS_SPRINGS,
    "d31327f6-f076-4366-9c7a-b084516ba215",
    "Rob Alexander",
);

// 7ED 345★ — Sulfurous Springs (alternate printing)
const SULFUROUS_SPRINGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::SULFUROUS_SPRINGS,
    1,
    "3531a023-b37c-472c-9697-8b2c018139c5",
    "Rob Alexander",
);

// 7ED 346 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "de7b6238-752d-49b7-8cdc-56587676e52d",
    "D. J. Cleland-Hura",
);

// 7ED 346★ — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "40705a25-2de4-4a54-9fc4-02899085da27",
    "D. J. Cleland-Hura",
);

// 7ED 347 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "6cfae795-d3d2-4758-9717-ed33cd0f3bfb",
    "Rob Alexander",
);

// 7ED 347★ — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "870b65a1-314b-4c22-b942-38f60ba2c392",
    "Rob Alexander",
);

// 7ED 348 — Swamp (alternate printing)
const SWAMP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    4,
    "c956611e-5748-42ff-b8ea-bca0c3d94de4",
    "Larry Elmore",
);

// 7ED 348★ — Swamp (alternate printing)
const SWAMP_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    5,
    "94cf9d05-153f-42c5-b48a-ad06da0ef4d5",
    "Larry Elmore",
);

// 7ED 349 — Swamp (alternate printing)
const SWAMP_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    6,
    "cebdd6db-9434-4e60-bdf2-234e21ccbb10",
    "Tony Szczudlo",
);

// 7ED 349★ — Swamp (alternate printing)
const SWAMP_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    7,
    "8838fb77-a484-4323-aa62-c944f3fefa95",
    "Tony Szczudlo",
);

// 7ED 350 — Underground River (reprint)
const UNDERGROUND_RIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::UNDERGROUND_RIVER,
    "1b496660-5a5d-4dec-93ca-60e6b1286221",
    "Andrew Goldhawk",
);

// 7ED 350★ — Underground River (alternate printing)
const UNDERGROUND_RIVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::UNDERGROUND_RIVER,
    1,
    "a8f22de9-7cdb-464f-9354-61c5aa858c7a",
    "Andrew Goldhawk",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANGELIC_PAGE_REPRINT,
    ANGELIC_PAGE_ALTERNATE_1,
    ARDENT_MILITIA_REPRINT,
    ARDENT_MILITIA_ALTERNATE_1,
    BLESSED_REVERSAL_REPRINT,
    BLESSED_REVERSAL_ALTERNATE_1,
    BREATH_OF_LIFE_REPRINT,
    BREATH_OF_LIFE_ALTERNATE_1,
    CASTLE_REPRINT,
    CASTLE_ALTERNATE_1,
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLACK_ALTERNATE_1,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_ALTERNATE_1,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_ALTERNATE_1,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_RED_ALTERNATE_1,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_ALTERNATE_1,
    CLOUDCHASER_EAGLE_REPRINT,
    CLOUDCHASER_EAGLE_ALTERNATE_1,
    CROSSBOW_INFANTRY_REPRINT,
    CROSSBOW_INFANTRY_ALTERNATE_1,
    DISENCHANT_REPRINT,
    DISENCHANT_ALTERNATE_1,
    EAGER_CADET_REPRINT,
    EAGER_CADET_ALTERNATE_1,
    ELITE_ARCHERS_REPRINT,
    ELITE_ARCHERS_ALTERNATE_1,
    GERRARD_S_WISDOM_REPRINT,
    GERRARD_S_WISDOM_ALTERNATE_1,
    GLORIOUS_ANTHEM_REPRINT,
    GLORIOUS_ANTHEM_ALTERNATE_1,
    HEALING_SALVE_REPRINT,
    HEALING_SALVE_ALTERNATE_1,
    HEAVY_BALLISTA_REPRINT,
    HEAVY_BALLISTA_ALTERNATE_1,
    HOLY_STRENGTH_REPRINT,
    HOLY_STRENGTH_ALTERNATE_1,
    HONOR_GUARD_REPRINT,
    HONOR_GUARD_ALTERNATE_1,
    INTREPID_HERO_REPRINT,
    INTREPID_HERO_ALTERNATE_1,
    KJELDORAN_ROYAL_GUARD_REPRINT,
    KJELDORAN_ROYAL_GUARD_ALTERNATE_1,
    KNIGHT_ERRANT_ALTERNATE_1,
    KNIGHT_ERRANT_REPRINT,
    KNIGHTHOOD_REPRINT,
    KNIGHTHOOD_ALTERNATE_1,
    LONGBOW_ARCHER_REPRINT,
    LONGBOW_ARCHER_ALTERNATE_1,
    MASTER_HEALER_REPRINT,
    MASTER_HEALER_ALTERNATE_1,
    NORTHERN_PALADIN_REPRINT,
    NORTHERN_PALADIN_ALTERNATE_1,
    PACIFISM_REPRINT,
    PACIFISM_ALTERNATE_1,
    PARIAH_REPRINT,
    PARIAH_ALTERNATE_1,
    PURIFY_REPRINT,
    PURIFY_ALTERNATE_1,
    RAZORFOOT_GRIFFIN_REPRINT,
    RAZORFOOT_GRIFFIN_ALTERNATE_1,
    REPRISAL_REPRINT,
    REPRISAL_ALTERNATE_1,
    REVERSE_DAMAGE_REPRINT,
    REVERSE_DAMAGE_ALTERNATE_1,
    ROLLING_STONES_REPRINT,
    ROLLING_STONES_ALTERNATE_1,
    SACRED_GROUND_REPRINT,
    SACRED_GROUND_ALTERNATE_1,
    SACRED_NECTAR_ALTERNATE_1,
    SACRED_NECTAR_REPRINT,
    SAMITE_HEALER_REPRINT,
    SAMITE_HEALER_ALTERNATE_1,
    SANCTIMONY_REPRINT,
    SANCTIMONY_ALTERNATE_1,
    SEASONED_MARSHAL_REPRINT,
    SEASONED_MARSHAL_ALTERNATE_1,
    SERRA_ADVOCATE_REPRINT,
    SERRA_ADVOCATE_ALTERNATE_1,
    SERRA_ANGEL_REPRINT,
    SERRA_ANGEL_ALTERNATE_1,
    SERRA_S_EMBRACE_REPRINT,
    SERRA_S_EMBRACE_ALTERNATE_1,
    SHIELD_WALL_REPRINT,
    SHIELD_WALL_ALTERNATE_1,
    SKYSHROUD_FALCON_REPRINT,
    SKYSHROUD_FALCON_ALTERNATE_1,
    SOUTHERN_PALADIN_REPRINT,
    SOUTHERN_PALADIN_ALTERNATE_1,
    SPIRIT_LINK_REPRINT,
    SPIRIT_LINK_ALTERNATE_1,
    STANDING_TROOPS_REPRINT,
    STANDING_TROOPS_ALTERNATE_1,
    STARLIGHT_ALTERNATE_1,
    STARLIGHT_REPRINT,
    STAUNCH_DEFENDERS_REPRINT,
    STAUNCH_DEFENDERS_ALTERNATE_1,
    SUNWEB_REPRINT,
    SUNWEB_ALTERNATE_1,
    SUSTAINER_OF_THE_REALM_REPRINT,
    SUSTAINER_OF_THE_REALM_ALTERNATE_1,
    VENERABLE_MONK_REPRINT,
    VENERABLE_MONK_ALTERNATE_1,
    VENGEANCE_REPRINT,
    VENGEANCE_ALTERNATE_1,
    WALL_OF_SWORDS_REPRINT,
    WALL_OF_SWORDS_ALTERNATE_1,
    WORSHIP_REPRINT,
    WORSHIP_ALTERNATE_1,
    WRATH_OF_GOD_REPRINT,
    WRATH_OF_GOD_ALTERNATE_1,
    AIR_ELEMENTAL_REPRINT,
    AIR_ELEMENTAL_ALTERNATE_1,
    ANCESTRAL_MEMORIES_REPRINT,
    ANCESTRAL_MEMORIES_ALTERNATE_1,
    ARCANE_LABORATORY_REPRINT,
    ARCANE_LABORATORY_ALTERNATE_1,
    ARCHIVIST_REPRINT,
    ARCHIVIST_ALTERNATE_1,
    BALEFUL_STARE_REPRINT,
    BALEFUL_STARE_ALTERNATE_1,
    BENTHIC_BEHEMOTH_REPRINT,
    BENTHIC_BEHEMOTH_ALTERNATE_1,
    BOOMERANG_REPRINT,
    BOOMERANG_ALTERNATE_1,
    CONFISCATE_REPRINT,
    CONFISCATE_ALTERNATE_1,
    CORAL_MERFOLK_REPRINT,
    CORAL_MERFOLK_ALTERNATE_1,
    COUNTERSPELL_REPRINT,
    COUNTERSPELL_ALTERNATE_1,
    DARING_APPRENTICE_REPRINT,
    DARING_APPRENTICE_ALTERNATE_1,
    DEFLECTION_REPRINT,
    DEFLECTION_ALTERNATE_1,
    DELUSIONS_OF_MEDIOCRITY_REPRINT,
    DELUSIONS_OF_MEDIOCRITY_ALTERNATE_1,
    EQUILIBRIUM_REPRINT,
    EQUILIBRIUM_ALTERNATE_1,
    EVACUATION_REPRINT,
    EVACUATION_ALTERNATE_1,
    FIGHTING_DRAKE_REPRINT,
    FIGHTING_DRAKE_ALTERNATE_1,
    FLEETING_IMAGE_REPRINT,
    FLEETING_IMAGE_ALTERNATE_1,
    FLIGHT_REPRINT,
    FLIGHT_ALTERNATE_1,
    FORCE_SPIKE_REPRINT,
    FORCE_SPIKE_ALTERNATE_1,
    GIANT_OCTOPUS_REPRINT,
    GIANT_OCTOPUS_ALTERNATE_1,
    GLACIAL_WALL_REPRINT,
    GLACIAL_WALL_ALTERNATE_1,
    HIBERNATION_REPRINT,
    HIBERNATION_ALTERNATE_1,
    HORNED_TURTLE_REPRINT,
    HORNED_TURTLE_ALTERNATE_1,
    INSPIRATION_REPRINT,
    INSPIRATION_ALTERNATE_1,
    LEVITATION_REPRINT,
    LEVITATION_ALTERNATE_1,
    LORD_OF_ATLANTIS_REPRINT,
    LORD_OF_ATLANTIS_ALTERNATE_1,
    MAHAMOTI_DJINN_REPRINT,
    MAHAMOTI_DJINN_ALTERNATE_1,
    MANA_BREACH_REPRINT,
    MANA_BREACH_ALTERNATE_1,
    MANA_SHORT_REPRINT,
    MANA_SHORT_ALTERNATE_1,
    MAWCOR_REPRINT,
    MAWCOR_ALTERNATE_1,
    MEMORY_LAPSE_REPRINT,
    MEMORY_LAPSE_ALTERNATE_1,
    MERFOLK_LOOTER_REPRINT,
    MERFOLK_LOOTER_ALTERNATE_1,
    MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT,
    MERFOLK_OF_THE_PEARL_TRIDENT_ALTERNATE_1,
    OPPORTUNITY_REPRINT,
    OPPORTUNITY_ALTERNATE_1,
    OPPOSITION_REPRINT,
    OPPOSITION_ALTERNATE_1,
    PHANTOM_WARRIOR_REPRINT,
    PHANTOM_WARRIOR_ALTERNATE_1,
    PRODIGAL_SORCERER_REPRINT,
    PRODIGAL_SORCERER_ALTERNATE_1,
    REMOVE_SOUL_REPRINT,
    REMOVE_SOUL_ALTERNATE_1,
    SAGE_OWL_REPRINT,
    SAGE_OWL_ALTERNATE_1,
    SEA_MONSTER_REPRINT,
    SEA_MONSTER_ALTERNATE_1,
    SLEIGHT_OF_HAND_REPRINT,
    SLEIGHT_OF_HAND_ALTERNATE_1,
    STEAL_ARTIFACT_REPRINT,
    STEAL_ARTIFACT_ALTERNATE_1,
    STORM_CROW_REPRINT,
    STORM_CROW_ALTERNATE_1,
    TELEPATHIC_SPIES_REPRINT,
    TELEPATHIC_SPIES_ALTERNATE_1,
    TELEPATHY_REPRINT,
    TELEPATHY_ALTERNATE_1,
    TEMPORAL_ADEPT_REPRINT,
    TEMPORAL_ADEPT_ALTERNATE_1,
    THIEVING_MAGPIE_REPRINT,
    THIEVING_MAGPIE_ALTERNATE_1,
    TOLARIAN_WINDS_REPRINT,
    TOLARIAN_WINDS_ALTERNATE_1,
    TREASURE_TROVE_REPRINT,
    TREASURE_TROVE_ALTERNATE_1,
    TWIDDLE_REPRINT,
    TWIDDLE_ALTERNATE_1,
    UNSUMMON_REPRINT,
    UNSUMMON_ALTERNATE_1,
    VIGILANT_DRAKE_REPRINT,
    VIGILANT_DRAKE_ALTERNATE_1,
    VIZZERDRIX_ALTERNATE_1,
    VIZZERDRIX_REPRINT,
    WALL_OF_AIR_REPRINT,
    WALL_OF_AIR_ALTERNATE_1,
    WALL_OF_WONDER_REPRINT,
    WALL_OF_WONDER_ALTERNATE_1,
    WIND_DANCER_REPRINT,
    WIND_DANCER_ALTERNATE_1,
    WIND_DRAKE_REPRINT,
    WIND_DRAKE_ALTERNATE_1,
    ABYSSAL_HORROR_REPRINT,
    ABYSSAL_HORROR_ALTERNATE_1,
    ABYSSAL_SPECTER_REPRINT,
    ABYSSAL_SPECTER_ALTERNATE_1,
    AGONIZING_MEMORIES_REPRINT,
    AGONIZING_MEMORIES_ALTERNATE_1,
    BEFOUL_REPRINT,
    BEFOUL_ALTERNATE_1,
    BELLOWING_FIEND_REPRINT,
    BELLOWING_FIEND_ALTERNATE_1,
    BEREAVEMENT_REPRINT,
    BEREAVEMENT_ALTERNATE_1,
    BLOOD_PET_REPRINT,
    BLOOD_PET_ALTERNATE_1,
    BOG_IMP_REPRINT,
    BOG_IMP_ALTERNATE_1,
    BOG_WRAITH_REPRINT,
    BOG_WRAITH_ALTERNATE_1,
    CORRUPT_REPRINT,
    CORRUPT_ALTERNATE_1,
    CRYPT_RATS_REPRINT,
    CRYPT_RATS_ALTERNATE_1,
    DAKMOR_LANCER_REPRINT,
    DAKMOR_LANCER_ALTERNATE_1,
    DARK_BANISHING_REPRINT,
    DARK_BANISHING_ALTERNATE_1,
    DARKEST_HOUR_REPRINT,
    DARKEST_HOUR_ALTERNATE_1,
    DREGS_OF_SORROW_REPRINT,
    DREGS_OF_SORROW_ALTERNATE_1,
    DRUDGE_SKELETONS_REPRINT,
    DRUDGE_SKELETONS_ALTERNATE_1,
    DRUDGE_SKELETONS_ALTERNATE_2,
    DRUDGE_SKELETONS_ALTERNATE_3,
    DURESS_REPRINT,
    DURESS_ALTERNATE_1,
    EASTERN_PALADIN_REPRINT,
    EASTERN_PALADIN_ALTERNATE_1,
    ENGINEERED_PLAGUE_REPRINT,
    ENGINEERED_PLAGUE_ALTERNATE_1,
    FALLEN_ANGEL_REPRINT,
    FALLEN_ANGEL_ALTERNATE_1,
    FEAR_REPRINT,
    FEAR_ALTERNATE_1,
    FOUL_IMP_REPRINT,
    FOUL_IMP_ALTERNATE_1,
    FUGUE_REPRINT,
    FUGUE_ALTERNATE_1,
    GIANT_COCKROACH_REPRINT,
    GIANT_COCKROACH_ALTERNATE_1,
    GRAVEDIGGER_REPRINT,
    GRAVEDIGGER_ALTERNATE_1,
    GREED_REPRINT,
    GREED_ALTERNATE_1,
    HOLLOW_DOGS_REPRINT,
    HOLLOW_DOGS_ALTERNATE_1,
    HOWL_FROM_BEYOND_REPRINT,
    HOWL_FROM_BEYOND_ALTERNATE_1,
    INFERNAL_CONTRACT_REPRINT,
    INFERNAL_CONTRACT_ALTERNATE_1,
    LESHRAC_S_RITE_REPRINT,
    LESHRAC_S_RITE_ALTERNATE_1,
    LOOMING_SHADE_REPRINT,
    LOOMING_SHADE_ALTERNATE_1,
    MEGRIM_REPRINT,
    MEGRIM_ALTERNATE_1,
    MIND_ROT_REPRINT,
    MIND_ROT_ALTERNATE_1,
    NAUSEA_REPRINT,
    NAUSEA_ALTERNATE_1,
    NECROLOGIA_REPRINT,
    NECROLOGIA_ALTERNATE_1,
    NIGHTMARE_REPRINT,
    NIGHTMARE_ALTERNATE_1,
    NOCTURNAL_RAID_REPRINT,
    NOCTURNAL_RAID_ALTERNATE_1,
    OPPRESSION_REPRINT,
    OPPRESSION_ALTERNATE_1,
    OSTRACIZE_REPRINT,
    OSTRACIZE_ALTERNATE_1,
    PERSECUTE_REPRINT,
    PERSECUTE_ALTERNATE_1,
    PLAGUE_BEETLE_REPRINT,
    PLAGUE_BEETLE_ALTERNATE_1,
    RAG_MAN_REPRINT,
    RAG_MAN_ALTERNATE_1,
    RAISE_DEAD_REPRINT,
    RAISE_DEAD_ALTERNATE_1,
    RAISE_DEAD_ALTERNATE_2,
    RAISE_DEAD_ALTERNATE_3,
    RAZORTOOTH_RATS_REPRINT,
    RAZORTOOTH_RATS_ALTERNATE_1,
    REPROCESS_REPRINT,
    REPROCESS_ALTERNATE_1,
    REVENANT_REPRINT,
    REVENANT_ALTERNATE_1,
    SCATHE_ZOMBIES_REPRINT,
    SCATHE_ZOMBIES_ALTERNATE_1,
    SCATHE_ZOMBIES_ALTERNATE_2,
    SCATHE_ZOMBIES_ALTERNATE_3,
    SERPENT_WARRIOR_REPRINT,
    SERPENT_WARRIOR_ALTERNATE_1,
    SOUL_FEAST_REPRINT,
    SOUL_FEAST_ALTERNATE_1,
    SPINELESS_THUG_REPRINT,
    SPINELESS_THUG_ALTERNATE_1,
    STRANDS_OF_NIGHT_REPRINT,
    STRANDS_OF_NIGHT_ALTERNATE_1,
    STRONGHOLD_ASSASSIN_REPRINT,
    STRONGHOLD_ASSASSIN_ALTERNATE_1,
    TAINTED_AETHER_REPRINT,
    TAINTED_AETHER_ALTERNATE_1,
    UNHOLY_STRENGTH_REPRINT,
    UNHOLY_STRENGTH_ALTERNATE_1,
    WALL_OF_BONE_REPRINT,
    WALL_OF_BONE_ALTERNATE_1,
    WESTERN_PALADIN_REPRINT,
    WESTERN_PALADIN_ALTERNATE_1,
    YAWGMOTH_S_EDICT_REPRINT,
    YAWGMOTH_S_EDICT_ALTERNATE_1,
    AETHER_FLASH_REPRINT,
    AETHER_FLASH_ALTERNATE_1,
    BALDUVIAN_BARBARIANS_REPRINT,
    BALDUVIAN_BARBARIANS_ALTERNATE_1,
    BEDLAM_REPRINT,
    BEDLAM_ALTERNATE_1,
    BLAZE_REPRINT,
    BLAZE_ALTERNATE_1,
    BLOODSHOT_CYCLOPS_REPRINT,
    BLOODSHOT_CYCLOPS_ALTERNATE_1,
    BOIL_REPRINT,
    BOIL_ALTERNATE_1,
    CRIMSON_HELLKITE_REPRINT,
    CRIMSON_HELLKITE_ALTERNATE_1,
    DISORDER_REPRINT,
    DISORDER_ALTERNATE_1,
    EARTHQUAKE_REPRINT,
    EARTHQUAKE_ALTERNATE_1,
    FERVOR_REPRINT,
    FERVOR_ALTERNATE_1,
    FINAL_FORTUNE_REPRINT,
    FINAL_FORTUNE_ALTERNATE_1,
    FIRE_ELEMENTAL_REPRINT,
    FIRE_ELEMENTAL_ALTERNATE_1,
    GHITU_FIRE_EATER_REPRINT,
    GHITU_FIRE_EATER_ALTERNATE_1,
    GOBLIN_CHARIOT_ALTERNATE_1,
    GOBLIN_CHARIOT_REPRINT,
    GOBLIN_DIGGING_TEAM_REPRINT,
    GOBLIN_DIGGING_TEAM_ALTERNATE_1,
    GOBLIN_ELITE_INFANTRY_REPRINT,
    GOBLIN_ELITE_INFANTRY_ALTERNATE_1,
    GOBLIN_GARDENER_REPRINT,
    GOBLIN_GARDENER_ALTERNATE_1,
    GOBLIN_GLIDER_REPRINT,
    GOBLIN_GLIDER_ALTERNATE_1,
    GOBLIN_KING_REPRINT,
    GOBLIN_KING_ALTERNATE_1,
    GOBLIN_MATRON_REPRINT,
    GOBLIN_MATRON_ALTERNATE_1,
    GOBLIN_RAIDER_REPRINT,
    GOBLIN_RAIDER_ALTERNATE_1,
    GOBLIN_SPELUNKERS_REPRINT,
    GOBLIN_SPELUNKERS_ALTERNATE_1,
    GOBLIN_WAR_DRUMS_REPRINT,
    GOBLIN_WAR_DRUMS_ALTERNATE_1,
    GRANITE_GRIP_REPRINT,
    GRANITE_GRIP_ALTERNATE_1,
    HILL_GIANT_REPRINT,
    HILL_GIANT_ALTERNATE_1,
    IMPATIENCE_REPRINT,
    IMPATIENCE_ALTERNATE_1,
    INFERNO_REPRINT,
    INFERNO_ALTERNATE_1,
    LAVA_AXE_REPRINT,
    LAVA_AXE_ALTERNATE_1,
    LIGHTNING_BLAST_REPRINT,
    LIGHTNING_BLAST_ALTERNATE_1,
    LIGHTNING_ELEMENTAL_REPRINT,
    LIGHTNING_ELEMENTAL_ALTERNATE_1,
    MANA_CLASH_REPRINT,
    MANA_CLASH_ALTERNATE_1,
    OGRE_TASKMASTER_REPRINT,
    OGRE_TASKMASTER_ALTERNATE_1,
    OKK_REPRINT,
    OKK_ALTERNATE_1,
    ORCISH_ARTILLERY_REPRINT,
    ORCISH_ARTILLERY_ALTERNATE_1,
    ORCISH_ORIFLAMME_REPRINT,
    ORCISH_ORIFLAMME_ALTERNATE_1,
    PILLAGE_REPRINT,
    PILLAGE_ALTERNATE_1,
    PYGMY_PYROSAUR_REPRINT,
    PYGMY_PYROSAUR_ALTERNATE_1,
    PYROCLASM_REPRINT,
    PYROCLASM_ALTERNATE_1,
    PYROTECHNICS_REPRINT,
    PYROTECHNICS_ALTERNATE_1,
    RAGING_GOBLIN_REPRINT,
    RAGING_GOBLIN_ALTERNATE_1,
    RECKLESS_EMBERMAGE_REPRINT,
    RECKLESS_EMBERMAGE_ALTERNATE_1,
    REFLEXES_REPRINT,
    REFLEXES_ALTERNATE_1,
    RELENTLESS_ASSAULT_REPRINT,
    RELENTLESS_ASSAULT_ALTERNATE_1,
    SABRETOOTH_TIGER_REPRINT,
    SABRETOOTH_TIGER_ALTERNATE_1,
    SEISMIC_ASSAULT_REPRINT,
    SEISMIC_ASSAULT_ALTERNATE_1,
    SHATTER_REPRINT,
    SHATTER_ALTERNATE_1,
    SHIVAN_DRAGON_REPRINT,
    SHIVAN_DRAGON_ALTERNATE_1,
    SHOCK_REPRINT,
    SHOCK_ALTERNATE_1,
    SPITTING_EARTH_REPRINT,
    SPITTING_EARTH_ALTERNATE_1,
    STONE_RAIN_REPRINT,
    STONE_RAIN_ALTERNATE_1,
    STORM_SHAMAN_REPRINT,
    STORM_SHAMAN_ALTERNATE_1,
    SUDDEN_IMPACT_REPRINT,
    SUDDEN_IMPACT_ALTERNATE_1,
    TRAINED_ORGG_REPRINT,
    TRAINED_ORGG_ALTERNATE_1,
    TREMOR_REPRINT,
    TREMOR_ALTERNATE_1,
    VOLCANIC_HAMMER_REPRINT,
    VOLCANIC_HAMMER_ALTERNATE_1,
    WALL_OF_FIRE_REPRINT,
    WALL_OF_FIRE_ALTERNATE_1,
    WILDFIRE_REPRINT,
    WILDFIRE_ALTERNATE_1,
    ANACONDA_REPRINT,
    ANACONDA_ALTERNATE_1,
    ANCIENT_SILVERBACK_REPRINT,
    ANCIENT_SILVERBACK_ALTERNATE_1,
    BIRDS_OF_PARADISE_REPRINT,
    BIRDS_OF_PARADISE_ALTERNATE_1,
    BLANCHWOOD_ARMOR_REPRINT,
    BLANCHWOOD_ARMOR_ALTERNATE_1,
    BULL_HIPPO_REPRINT,
    BULL_HIPPO_ALTERNATE_1,
    CANOPY_SPIDER_REPRINT,
    CANOPY_SPIDER_ALTERNATE_1,
    COMPOST_REPRINT,
    COMPOST_ALTERNATE_1,
    CREEPING_MOLD_REPRINT,
    CREEPING_MOLD_ALTERNATE_1,
    EARLY_HARVEST_REPRINT,
    EARLY_HARVEST_ALTERNATE_1,
    ELDER_DRUID_REPRINT,
    ELDER_DRUID_ALTERNATE_1,
    ELVISH_ARCHERS_REPRINT,
    ELVISH_ARCHERS_ALTERNATE_1,
    ELVISH_CHAMPION_REPRINT,
    ELVISH_CHAMPION_ALTERNATE_1,
    ELVISH_LYRIST_REPRINT,
    ELVISH_LYRIST_ALTERNATE_1,
    ELVISH_PIPER_REPRINT,
    ELVISH_PIPER_ALTERNATE_1,
    FAMILIAR_GROUND_REPRINT,
    FAMILIAR_GROUND_ALTERNATE_1,
    FEMEREF_ARCHERS_REPRINT,
    FEMEREF_ARCHERS_ALTERNATE_1,
    FOG_REPRINT,
    FOG_ALTERNATE_1,
    FYNDHORN_ELDER_REPRINT,
    FYNDHORN_ELDER_ALTERNATE_1,
    GANG_OF_ELK_REPRINT,
    GANG_OF_ELK_ALTERNATE_1,
    GIANT_GROWTH_REPRINT,
    GIANT_GROWTH_ALTERNATE_1,
    GIANT_SPIDER_REPRINT,
    GIANT_SPIDER_ALTERNATE_1,
    GORILLA_CHIEFTAIN_REPRINT,
    GORILLA_CHIEFTAIN_ALTERNATE_1,
    GRIZZLY_BEARS_REPRINT,
    GRIZZLY_BEARS_ALTERNATE_1,
    HURRICANE_REPRINT,
    HURRICANE_ALTERNATE_1,
    LLANOWAR_ELVES_REPRINT,
    LLANOWAR_ELVES_ALTERNATE_1,
    LONE_WOLF_REPRINT,
    LONE_WOLF_ALTERNATE_1,
    LURE_REPRINT,
    LURE_ALTERNATE_1,
    MARO_REPRINT,
    MARO_ALTERNATE_1,
    MIGHT_OF_OAKS_REPRINT,
    MIGHT_OF_OAKS_ALTERNATE_1,
    MONSTROUS_GROWTH_REPRINT,
    MONSTROUS_GROWTH_ALTERNATE_1,
    NATURE_S_RESURGENCE_REPRINT,
    NATURE_S_RESURGENCE_ALTERNATE_1,
    NATURE_S_REVOLT_REPRINT,
    NATURE_S_REVOLT_ALTERNATE_1,
    PRIDE_OF_LIONS_ALTERNATE_1,
    PRIDE_OF_LIONS_REPRINT,
    RAMPANT_GROWTH_REPRINT,
    RAMPANT_GROWTH_ALTERNATE_1,
    RECLAIM_REPRINT,
    RECLAIM_ALTERNATE_1,
    REDWOOD_TREEFOLK_REPRINT,
    REDWOOD_TREEFOLK_ALTERNATE_1,
    REGENERATION_REPRINT,
    REGENERATION_ALTERNATE_1,
    ROWEN_REPRINT,
    ROWEN_ALTERNATE_1,
    SCAVENGER_FOLK_REPRINT,
    SCAVENGER_FOLK_ALTERNATE_1,
    SEEKER_OF_SKYBREAK_REPRINT,
    SEEKER_OF_SKYBREAK_ALTERNATE_1,
    SHANODIN_DRYADS_REPRINT,
    SHANODIN_DRYADS_ALTERNATE_1,
    SPINED_WURM_REPRINT,
    SPINED_WURM_ALTERNATE_1,
    SQUALL_REPRINT,
    SQUALL_ALTERNATE_1,
    STREAM_OF_LIFE_REPRINT,
    STREAM_OF_LIFE_ALTERNATE_1,
    THORN_ELEMENTAL_REPRINT,
    THORN_ELEMENTAL_ALTERNATE_1,
    THOUGHTLEECH_REPRINT,
    THOUGHTLEECH_ALTERNATE_1,
    TRAINED_ARMODON_REPRINT,
    TRAINED_ARMODON_ALTERNATE_1,
    TRANQUILITY_REPRINT,
    TRANQUILITY_ALTERNATE_1,
    TREEFOLK_SEEDLINGS_REPRINT,
    TREEFOLK_SEEDLINGS_ALTERNATE_1,
    UKTABI_WILDCATS_REPRINT,
    UKTABI_WILDCATS_ALTERNATE_1,
    UNTAMED_WILDS_REPRINT,
    UNTAMED_WILDS_ALTERNATE_1,
    VERDURAN_ENCHANTRESS_REPRINT,
    VERDURAN_ENCHANTRESS_ALTERNATE_1,
    VERNAL_BLOOM_REPRINT,
    VERNAL_BLOOM_ALTERNATE_1,
    WILD_GROWTH_REPRINT,
    WILD_GROWTH_ALTERNATE_1,
    WING_SNARE_REPRINT,
    WING_SNARE_ALTERNATE_1,
    WOOD_ELVES_REPRINT,
    WOOD_ELVES_ALTERNATE_1,
    YAVIMAYA_ENCHANTRESS_REPRINT,
    YAVIMAYA_ENCHANTRESS_ALTERNATE_1,
    ALADDINS_RING_REPRINT,
    ALADDINS_RING_ALTERNATE_1,
    BEAST_OF_BURDEN_REPRINT,
    BEAST_OF_BURDEN_ALTERNATE_1,
    CALTROPS_REPRINT,
    CALTROPS_ALTERNATE_1,
    CHARCOAL_DIAMOND_REPRINT,
    CHARCOAL_DIAMOND_ALTERNATE_1,
    CHARCOAL_DIAMOND_ALTERNATE_2,
    CHARCOAL_DIAMOND_ALTERNATE_3,
    COAT_OF_ARMS_REPRINT,
    COAT_OF_ARMS_ALTERNATE_1,
    CRYSTAL_ROD_REPRINT,
    CRYSTAL_ROD_ALTERNATE_1,
    DINGUS_EGG_REPRINT,
    DINGUS_EGG_ALTERNATE_1,
    DISRUPTING_SCEPTER_REPRINT,
    DISRUPTING_SCEPTER_ALTERNATE_1,
    ENSNARING_BRIDGE_REPRINT,
    ENSNARING_BRIDGE_ALTERNATE_1,
    FEROZ_S_BAN_REPRINT,
    FEROZ_S_BAN_ALTERNATE_1,
    FIRE_DIAMOND_REPRINT,
    FIRE_DIAMOND_ALTERNATE_1,
    FLYING_CARPET_REPRINT,
    FLYING_CARPET_ALTERNATE_1,
    GRAFTED_SKULLCAP_REPRINT,
    GRAFTED_SKULLCAP_ALTERNATE_1,
    GRAPESHOT_CATAPULT_REPRINT,
    GRAPESHOT_CATAPULT_ALTERNATE_1,
    HOWLING_MINE_REPRINT,
    HOWLING_MINE_ALTERNATE_1,
    IRON_STAR_REPRINT,
    IRON_STAR_ALTERNATE_1,
    IVORY_CUP_REPRINT,
    IVORY_CUP_ALTERNATE_1,
    JALUM_TOME_REPRINT,
    JALUM_TOME_ALTERNATE_1,
    JANDORS_SADDLEBAGS_REPRINT,
    JANDORS_SADDLEBAGS_ALTERNATE_1,
    JAYEMDAE_TOME_REPRINT,
    JAYEMDAE_TOME_ALTERNATE_1,
    MARBLE_DIAMOND_REPRINT,
    MARBLE_DIAMOND_ALTERNATE_1,
    MEEKSTONE_REPRINT,
    MEEKSTONE_ALTERNATE_1,
    MILLSTONE_REPRINT,
    MILLSTONE_ALTERNATE_1,
    MOSS_DIAMOND_REPRINT,
    MOSS_DIAMOND_ALTERNATE_1,
    PATAGIA_GOLEM_REPRINT,
    PATAGIA_GOLEM_ALTERNATE_1,
    PHYREXIAN_COLOSSUS_REPRINT,
    PHYREXIAN_COLOSSUS_ALTERNATE_1,
    PHYREXIAN_HULK_REPRINT,
    PHYREXIAN_HULK_ALTERNATE_1,
    PIT_TRAP_REPRINT,
    PIT_TRAP_ALTERNATE_1,
    ROD_OF_RUIN_REPRINT,
    ROD_OF_RUIN_ALTERNATE_1,
    SISAY_S_RING_REPRINT,
    SISAY_S_RING_ALTERNATE_1,
    SKY_DIAMOND_REPRINT,
    SKY_DIAMOND_ALTERNATE_1,
    SOUL_NET_REPRINT,
    SOUL_NET_ALTERNATE_1,
    SPELLBOOK_REPRINT,
    SPELLBOOK_ALTERNATE_1,
    STATIC_ORB_REPRINT,
    STATIC_ORB_ALTERNATE_1,
    STORM_CAULDRON_REPRINT,
    STORM_CAULDRON_ALTERNATE_1,
    TEFERI_S_PUZZLE_BOX_REPRINT,
    TEFERI_S_PUZZLE_BOX_ALTERNATE_1,
    THRONE_OF_BONE_REPRINT,
    THRONE_OF_BONE_ALTERNATE_1,
    WALL_OF_SPEARS_REPRINT,
    WALL_OF_SPEARS_ALTERNATE_1,
    WOODEN_SPHERE_REPRINT,
    WOODEN_SPHERE_ALTERNATE_1,
    ADARKAR_WASTES_REPRINT,
    ADARKAR_WASTES_ALTERNATE_1,
    BRUSHLAND_REPRINT,
    BRUSHLAND_ALTERNATE_1,
    CITY_OF_BRASS_REPRINT,
    CITY_OF_BRASS_ALTERNATE_1,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
    FOREST_ALTERNATE_4,
    FOREST_ALTERNATE_5,
    FOREST_ALTERNATE_6,
    FOREST_ALTERNATE_7,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    ISLAND_ALTERNATE_4,
    ISLAND_ALTERNATE_5,
    ISLAND_ALTERNATE_6,
    ISLAND_ALTERNATE_7,
    KARPLUSAN_FOREST_REPRINT,
    KARPLUSAN_FOREST_ALTERNATE_1,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_4,
    MOUNTAIN_ALTERNATE_5,
    MOUNTAIN_ALTERNATE_6,
    MOUNTAIN_ALTERNATE_7,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    PLAINS_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    PLAINS_ALTERNATE_6,
    PLAINS_ALTERNATE_7,
    SULFUROUS_SPRINGS_REPRINT,
    SULFUROUS_SPRINGS_ALTERNATE_1,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    SWAMP_ALTERNATE_4,
    SWAMP_ALTERNATE_5,
    SWAMP_ALTERNATE_6,
    SWAMP_ALTERNATE_7,
    UNDERGROUND_RIVER_REPRINT,
    UNDERGROUND_RIVER_ALTERNATE_1,
];
