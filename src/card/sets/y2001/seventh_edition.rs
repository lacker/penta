//! Seventh Edition has no unique card definitions.
//!
//! It is the last core set inside the Premodern window, so a card printed
//! only in a Portal set before it becomes legal here.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
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
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2012::magic_2013;
use crate::card::sets::y2012::return_to_ravnica as catalog_rtr;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::{CardArt, CardRules, CardSet};
use crate::mana_cost;

// 7ED 1 — Angelic Page (reprint)

// 7ED 1★ — Angelic Page (alternate printing)

// 7ED 2 — Ardent Militia (reprint)

// 7ED 2★ — Ardent Militia (alternate printing)

// 7ED 3 — Blessed Reversal (reprint)

// 7ED 3★ — Blessed Reversal (alternate printing)

// 7ED 4 — Breath of Life (reprint)

// 7ED 4★ — Breath of Life (alternate printing)

// 7ED 5 — Castle (reprint)

// 7ED 5★ — Castle (alternate printing)

// 7ED 6 — Circle of Protection: Black (reprint)

// 7ED 6★ — Circle of Protection: Black (alternate printing)

// 7ED 7 — Circle of Protection: Blue (reprint)

// 7ED 7★ — Circle of Protection: Blue (alternate printing)

// 7ED 8 — Circle of Protection: Green (reprint)

// 7ED 8★ — Circle of Protection: Green (alternate printing)

// 7ED 9 — Circle of Protection: Red (reprint)

// 7ED 9★ — Circle of Protection: Red (alternate printing)

// 7ED 10 — Circle of Protection: White (reprint)

// 7ED 10★ — Circle of Protection: White (alternate printing)

// 7ED 11 — Cloudchaser Eagle (reprint)

// 7ED 11★ — Cloudchaser Eagle (alternate printing)

// 7ED 12 — Crossbow Infantry (reprint)

// 7ED 12★ — Crossbow Infantry (alternate printing)

// 7ED 13 — Disenchant (reprint)

// 7ED 13★ — Disenchant (alternate printing)

// 7ED 14 — Eager Cadet
pub(in crate::card::sets) static EAGER_CADET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1e1ce2f-d8af-4fd0-975e-9d910d12b883"),
    "Eager Cadet",
    CardArt::new(
        "46b89ce6-8a73-4e27-8696-e65ea0c16925",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::SeventhEdition,
    // A vanilla 1/1 for one, printed for the starter decks that needed a
    // creature everybody could read.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1),
);

// 7ED 14★ — Eager Cadet (alternate printing)

// 7ED 15 — Elite Archers (reprint)

// 7ED 15★ — Elite Archers (alternate printing)

// 7ED 16 — Gerrard's Wisdom (reprint)

// 7ED 16★ — Gerrard's Wisdom (alternate printing)

// 7ED 17 — Glorious Anthem (reprint)

// 7ED 17★ — Glorious Anthem (alternate printing)

// 7ED 18 — Healing Salve (reprint)

// 7ED 18★ — Healing Salve (alternate printing)

// 7ED 19 — Heavy Ballista (reprint)

// 7ED 19★ — Heavy Ballista (alternate printing)

// 7ED 20 — Holy Strength (reprint)

// 7ED 20★ — Holy Strength (alternate printing)

// 7ED 21 — Honor Guard (reprint)

// 7ED 21★ — Honor Guard (alternate printing)

// 7ED 22 — Intrepid Hero (reprint)

// 7ED 22★ — Intrepid Hero (alternate printing)

// 7ED 23 — Kjeldoran Royal Guard (reprint)

// 7ED 23★ — Kjeldoran Royal Guard (alternate printing)

// 7ED 24 — Knight Errant (alternate printing)

// 7ED 24★ — Knight Errant
pub(in crate::card::sets) static KNIGHT_ERRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c31b4b4-18fc-4a6e-8d74-fd5340964320"),
    "Knight Errant",
    CardArt::new("413f10fe-0e53-46ca-bd64-0d66dee8882d", "Matthew D. Wilson"),
    CardSet::SeventhEdition,
    // A vanilla 2/2 for two, and a Knight for the decks that count them.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 2),
);

// 7ED 25 — Knighthood (reprint)

// 7ED 25★ — Knighthood (alternate printing)

// 7ED 26 — Longbow Archer (reprint)

// 7ED 26★ — Longbow Archer (alternate printing)

// 7ED 27 — Master Healer (reprint)

// 7ED 27★ — Master Healer (alternate printing)

// 7ED 28 — Northern Paladin (reprint)

// 7ED 28★ — Northern Paladin (alternate printing)

// 7ED 29 — Pacifism (reprint)

// 7ED 29★ — Pacifism (alternate printing)

// 7ED 30 — Pariah (reprint)

// 7ED 30★ — Pariah (alternate printing)

// 7ED 31 — Purify (reprint)

// 7ED 31★ — Purify (alternate printing)

// 7ED 32 — Razorfoot Griffin (reprint)

// 7ED 32★ — Razorfoot Griffin (alternate printing)

// 7ED 33 — Reprisal (reprint)

// 7ED 33★ — Reprisal (alternate printing)

// 7ED 34 — Reverse Damage (reprint)

// 7ED 34★ — Reverse Damage (alternate printing)

// 7ED 35 — Rolling Stones (reprint)

// 7ED 35★ — Rolling Stones (alternate printing)

// 7ED 36 — Sacred Ground (reprint)

// 7ED 36★ — Sacred Ground (alternate printing)

// 7ED 37 — Sacred Nectar (alternate printing)

// 7ED 37★ — Sacred Nectar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_NECTAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("484d1b31-5363-49ef-9b13-2005568636c1"),
    "Sacred Nectar",
    crate::card::CardArt::new("8d4b8de0-0bb5-40fb-8b73-d00d38a582d5", "Dana Knutson"),
    crate::card::CardSet::SeventhEdition,
    crate::card::CardRules::unsupported(),
);

// 7ED 38 — Samite Healer (reprint)

// 7ED 38★ — Samite Healer (alternate printing)

// 7ED 39 — Sanctimony (reprint)

// 7ED 39★ — Sanctimony (alternate printing)

// 7ED 40 — Seasoned Marshal (reprint)

// 7ED 40★ — Seasoned Marshal (alternate printing)

// 7ED 41 — Serra Advocate (reprint)

// 7ED 41★ — Serra Advocate (alternate printing)

// 7ED 42 — Serra Angel (reprint)

// 7ED 42★ — Serra Angel (alternate printing)

// 7ED 43 — Serra's Embrace (reprint)

// 7ED 43★ — Serra's Embrace (alternate printing)

// 7ED 44 — Shield Wall (reprint)

// 7ED 44★ — Shield Wall (alternate printing)

// 7ED 45 — Skyshroud Falcon (reprint)

// 7ED 45★ — Skyshroud Falcon (alternate printing)

// 7ED 46 — Southern Paladin (reprint)

// 7ED 46★ — Southern Paladin (alternate printing)

// 7ED 47 — Spirit Link (reprint)

// 7ED 47★ — Spirit Link (alternate printing)

// 7ED 48 — Standing Troops (reprint)

// 7ED 48★ — Standing Troops (alternate printing)

// 7ED 49 — Starlight (alternate printing)

// 7ED 49★ — Starlight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARLIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6992524-6921-473b-8301-cb63fe502600"),
    "Starlight",
    crate::card::CardArt::new("413c5a7e-e19d-4cbd-9279-88391b75c6c5", "Brian Despain"),
    crate::card::CardSet::SeventhEdition,
    crate::card::CardRules::unsupported(),
);

// 7ED 50 — Staunch Defenders (reprint)

// 7ED 50★ — Staunch Defenders (alternate printing)

// 7ED 51 — Sunweb (reprint)

// 7ED 51★ — Sunweb (alternate printing)

// 7ED 52 — Sustainer of the Realm (reprint)

// 7ED 52★ — Sustainer of the Realm (alternate printing)

// 7ED 53 — Venerable Monk (reprint)

// 7ED 53★ — Venerable Monk (alternate printing)

// 7ED 54 — Vengeance (reprint)

// 7ED 54★ — Vengeance (alternate printing)

// 7ED 55 — Wall of Swords (reprint)

// 7ED 55★ — Wall of Swords (alternate printing)

// 7ED 56 — Worship (reprint)

// 7ED 56★ — Worship (alternate printing)

// 7ED 57 — Wrath of God (reprint)

// 7ED 57★ — Wrath of God (alternate printing)

// 7ED 58 — Air Elemental (reprint)

// 7ED 58★ — Air Elemental (alternate printing)

// 7ED 59 — Ancestral Memories (reprint)

// 7ED 59★ — Ancestral Memories (alternate printing)

// 7ED 60 — Arcane Laboratory (reprint)

// 7ED 60★ — Arcane Laboratory (alternate printing)

// 7ED 61 — Archivist (reprint)

// 7ED 61★ — Archivist (alternate printing)

// 7ED 62 — Baleful Stare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALEFUL_STARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49fb46c8-30ae-4457-a726-6fe1ddd183d5"),
    "Baleful Stare",
    crate::card::CardArt::new("7c53b808-c2c5-4941-bead-1cb94adc5a2f", "Eric Peterson"),
    crate::card::CardSet::SeventhEdition,
    crate::card::CardRules::unsupported(),
);

// 7ED 62★ — Baleful Stare (alternate printing)

// 7ED 63 — Benthic Behemoth (reprint)

// 7ED 63★ — Benthic Behemoth (alternate printing)

// 7ED 64 — Boomerang (reprint)

// 7ED 64★ — Boomerang (alternate printing)

// 7ED 65 — Confiscate (reprint)

// 7ED 65★ — Confiscate (alternate printing)

// 7ED 66 — Coral Merfolk (reprint)

// 7ED 66★ — Coral Merfolk (alternate printing)

// 7ED 67 — Counterspell (reprint)

// 7ED 67★ — Counterspell (alternate printing)

// 7ED 68 — Daring Apprentice (reprint)

// 7ED 68★ — Daring Apprentice (alternate printing)

// 7ED 69 — Deflection (reprint)

// 7ED 69★ — Deflection (alternate printing)

// 7ED 70 — Delusions of Mediocrity (reprint)

// 7ED 70★ — Delusions of Mediocrity (alternate printing)

// 7ED 71 — Equilibrium (reprint)

// 7ED 71★ — Equilibrium (alternate printing)

// 7ED 72 — Evacuation (reprint)

// 7ED 72★ — Evacuation (alternate printing)

// 7ED 73 — Fighting Drake (reprint)

// 7ED 73★ — Fighting Drake (alternate printing)

// 7ED 74 — Fleeting Image (reprint)

// 7ED 74★ — Fleeting Image (alternate printing)

// 7ED 75 — Flight (reprint)

// 7ED 75★ — Flight (alternate printing)

// 7ED 76 — Force Spike (reprint)

// 7ED 76★ — Force Spike (alternate printing)

// 7ED 77 — Giant Octopus
pub(in crate::card::sets) static GIANT_OCTOPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4528edca-cc36-4f63-9615-24ca315d672c"),
    "Giant Octopus",
    CardArt::new("5b707b2d-63e1-4c2c-ba42-9e027f02b1ff", "Heather Hudson"),
    CardSet::SeventhEdition,
    // A vanilla 3/3 for four in the colour that was not supposed to get
    // one, which is why it costs a mana more than it looks like it should.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Octopus"], 3, 3),
);

// 7ED 77★ — Giant Octopus (alternate printing)

// 7ED 78 — Glacial Wall (reprint)

// 7ED 78★ — Glacial Wall (alternate printing)

// 7ED 79 — Hibernation (reprint)

// 7ED 79★ — Hibernation (alternate printing)

// 7ED 80 — Horned Turtle (reprint)

// 7ED 80★ — Horned Turtle (alternate printing)

// 7ED 81 — Inspiration (reprint)

// 7ED 81★ — Inspiration (alternate printing)

// 7ED 82 — Levitation (reprint)

// 7ED 82★ — Levitation (alternate printing)

// 7ED 83 — Lord of Atlantis (reprint)

// 7ED 83★ — Lord of Atlantis (alternate printing)

// 7ED 84 — Mahamoti Djinn (reprint)

// 7ED 84★ — Mahamoti Djinn (alternate printing)

// 7ED 85 — Mana Breach (reprint)

// 7ED 85★ — Mana Breach (alternate printing)

// 7ED 86 — Mana Short (reprint)

// 7ED 86★ — Mana Short (alternate printing)

// 7ED 87 — Mawcor (reprint)

// 7ED 87★ — Mawcor (alternate printing)

// 7ED 88 — Memory Lapse (reprint)

// 7ED 88★ — Memory Lapse (alternate printing)

// 7ED 89 — Merfolk Looter (reprint)

// 7ED 89★ — Merfolk Looter (alternate printing)

// 7ED 90 — Merfolk of the Pearl Trident (reprint)

// 7ED 90★ — Merfolk of the Pearl Trident (alternate printing)

// 7ED 91 — Opportunity (reprint)

// 7ED 91★ — Opportunity (alternate printing)

// 7ED 92 — Opposition (reprint)

// 7ED 92★ — Opposition (alternate printing)

// 7ED 93 — Phantom Warrior (reprint)

// 7ED 93★ — Phantom Warrior (alternate printing)

// 7ED 94 — Prodigal Sorcerer (reprint)

// 7ED 94★ — Prodigal Sorcerer (alternate printing)

// 7ED 95 — Remove Soul (reprint)

// 7ED 95★ — Remove Soul (alternate printing)

// 7ED 96 — Sage Owl (reprint)

// 7ED 96★ — Sage Owl (alternate printing)

// 7ED 97 — Sea Monster (reprint)

// 7ED 97★ — Sea Monster (alternate printing)

// 7ED 98 — Sleight of Hand (reprint)

// 7ED 98★ — Sleight of Hand (alternate printing)

// 7ED 99 — Steal Artifact (reprint)

// 7ED 99★ — Steal Artifact (alternate printing)

// 7ED 100 — Storm Crow (reprint)

// 7ED 100★ — Storm Crow (alternate printing)

// 7ED 101 — Telepathic Spies (reprint)

// 7ED 101★ — Telepathic Spies (alternate printing)

// 7ED 102 — Telepathy (reprint)

// 7ED 102★ — Telepathy (alternate printing)

// 7ED 103 — Temporal Adept (reprint)

// 7ED 103★ — Temporal Adept (alternate printing)

// 7ED 104 — Thieving Magpie (reprint)

// 7ED 104★ — Thieving Magpie (alternate printing)

// 7ED 105 — Tolarian Winds (reprint)

// 7ED 105★ — Tolarian Winds (alternate printing)

// 7ED 106 — Treasure Trove (reprint)

// 7ED 106★ — Treasure Trove (alternate printing)

// 7ED 107 — Twiddle (reprint)

// 7ED 107★ — Twiddle (alternate printing)

// 7ED 108 — Unsummon (reprint)

// 7ED 108★ — Unsummon (alternate printing)

// 7ED 109 — Vigilant Drake (reprint)

// 7ED 109★ — Vigilant Drake (alternate printing)

// 7ED 110 — Vizzerdrix (alternate printing)

// 7ED 110★ — Vizzerdrix
pub(in crate::card::sets) static VIZZERDRIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25711022-7270-4335-a48b-9f2b8275ceeb"),
    "Vizzerdrix",
    CardArt::new("249ecab6-e145-4dfd-9e9e-56492db30b4c", "Dave Dorman"),
    CardSet::SeventhEdition,
    // Seven mana for a 6/6 with nothing on it. A starter-deck rare, and a
    // reminder of what blue paid for raw size before it stopped paying.
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Rabbit", "Beast"], 6, 6),
);

// 7ED 111 — Wall of Air (reprint)

// 7ED 111★ — Wall of Air (alternate printing)

// 7ED 112 — Wall of Wonder (reprint)

// 7ED 112★ — Wall of Wonder (alternate printing)

// 7ED 113 — Wind Dancer (reprint)

// 7ED 113★ — Wind Dancer (alternate printing)

// 7ED 114 — Wind Drake (reprint)

// 7ED 114★ — Wind Drake (alternate printing)

// 7ED 115 — Abyssal Horror (reprint)

// 7ED 115★ — Abyssal Horror (alternate printing)

// 7ED 116 — Abyssal Specter (reprint)

// 7ED 116★ — Abyssal Specter (alternate printing)

// 7ED 117 — Agonizing Memories (reprint)

// 7ED 117★ — Agonizing Memories (alternate printing)

// 7ED 118 — Befoul (reprint)

// 7ED 118★ — Befoul (alternate printing)

// 7ED 119 — Bellowing Fiend (reprint)

// 7ED 119★ — Bellowing Fiend (alternate printing)

// 7ED 120 — Bereavement (reprint)

// 7ED 120★ — Bereavement (alternate printing)

// 7ED 121 — Blood Pet (reprint)

// 7ED 121★ — Blood Pet (alternate printing)

// 7ED 122 — Bog Imp (reprint)

// 7ED 122★ — Bog Imp (alternate printing)

// 7ED 123 — Bog Wraith (reprint)

// 7ED 123★ — Bog Wraith (alternate printing)

// 7ED 124 — Corrupt (reprint)

// 7ED 124★ — Corrupt (alternate printing)

// 7ED 125 — Crypt Rats (reprint)

// 7ED 125★ — Crypt Rats (alternate printing)

// 7ED 126 — Dakmor Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAKMOR_LANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d012ddf-abe1-4de9-89cb-78d82afb9e7b"),
    "Dakmor Lancer",
    crate::card::CardArt::new("660cc594-63f5-4819-a556-7a9484145f72", "Luca Zontini"),
    crate::card::CardSet::SeventhEdition,
    crate::card::CardRules::unsupported(),
);

// 7ED 126★ — Dakmor Lancer (alternate printing)

// 7ED 127 — Dark Banishing (reprint)

// 7ED 127★ — Dark Banishing (alternate printing)

// 7ED 128 — Darkest Hour (reprint)

// 7ED 128★ — Darkest Hour (alternate printing)

// 7ED 129 — Dregs of Sorrow (reprint)

// 7ED 129★ — Dregs of Sorrow (alternate printing)

// 7ED 130 — Drudge Skeletons (reprint)

// 7ED 130s — Drudge Skeletons (alternate printing)

// 7ED 130★ — Drudge Skeletons (alternate printing)

// 7ED 130★s — Drudge Skeletons (alternate printing)

// 7ED 131 — Duress (reprint)

// 7ED 131★ — Duress (alternate printing)

// 7ED 132 — Eastern Paladin (reprint)

// 7ED 132★ — Eastern Paladin (alternate printing)

// 7ED 133 — Engineered Plague (reprint)

// 7ED 133★ — Engineered Plague (alternate printing)

// 7ED 134 — Fallen Angel (reprint)

// 7ED 134★ — Fallen Angel (alternate printing)

// 7ED 135 — Fear (reprint)

// 7ED 135★ — Fear (alternate printing)

// 7ED 136 — Foul Imp (reprint)

// 7ED 136★ — Foul Imp (alternate printing)

// 7ED 137 — Fugue (reprint)

// 7ED 137★ — Fugue (alternate printing)

// 7ED 138 — Giant Cockroach (reprint)

// 7ED 138★ — Giant Cockroach (alternate printing)

// 7ED 139 — Gravedigger (reprint)

// 7ED 139★ — Gravedigger (alternate printing)

// 7ED 140 — Greed (reprint)

// 7ED 140★ — Greed (alternate printing)

// 7ED 141 — Hollow Dogs (reprint)

// 7ED 141★ — Hollow Dogs (alternate printing)

// 7ED 142 — Howl from Beyond (reprint)

// 7ED 142★ — Howl from Beyond (alternate printing)

// 7ED 143 — Infernal Contract (reprint)

// 7ED 143★ — Infernal Contract (alternate printing)

// 7ED 144 — Leshrac's Rite (reprint)

// 7ED 144★ — Leshrac's Rite (alternate printing)

// 7ED 145 — Looming Shade (reprint)

// 7ED 145★ — Looming Shade (alternate printing)

// 7ED 146 — Megrim (reprint)

// 7ED 146★ — Megrim (alternate printing)

// 7ED 147 — Mind Rot (reprint)

// 7ED 147★ — Mind Rot (alternate printing)

// 7ED 148 — Nausea (reprint)

// 7ED 148★ — Nausea (alternate printing)

// 7ED 149 — Necrologia (reprint)

// 7ED 149★ — Necrologia (alternate printing)

// 7ED 150 — Nightmare (reprint)

// 7ED 150★ — Nightmare (alternate printing)

// 7ED 151 — Nocturnal Raid (reprint)

// 7ED 151★ — Nocturnal Raid (alternate printing)

// 7ED 152 — Oppression (reprint)

// 7ED 152★ — Oppression (alternate printing)

// 7ED 153 — Ostracize (reprint)

// 7ED 153★ — Ostracize (alternate printing)

// 7ED 154 — Persecute (reprint)

// 7ED 154★ — Persecute (alternate printing)

// 7ED 155 — Plague Beetle (reprint)

// 7ED 155★ — Plague Beetle (alternate printing)

// 7ED 156 — Rag Man (reprint)

// 7ED 156★ — Rag Man (alternate printing)

// 7ED 157 — Raise Dead (reprint)

// 7ED 157s — Raise Dead (alternate printing)

// 7ED 157★ — Raise Dead (alternate printing)

// 7ED 157★s — Raise Dead (alternate printing)

// 7ED 158 — Razortooth Rats (reprint)

// 7ED 158★ — Razortooth Rats (alternate printing)

// 7ED 159 — Reprocess (reprint)

// 7ED 159★ — Reprocess (alternate printing)

// 7ED 160 — Revenant (reprint)

// 7ED 160★ — Revenant (alternate printing)

// 7ED 161 — Scathe Zombies (reprint)

// 7ED 161s — Scathe Zombies (alternate printing)

// 7ED 161★ — Scathe Zombies (alternate printing)

// 7ED 161★s — Scathe Zombies (alternate printing)

// 7ED 162 — Serpent Warrior (reprint)

// 7ED 162★ — Serpent Warrior (alternate printing)

// 7ED 163 — Soul Feast (reprint)

// 7ED 163★ — Soul Feast (alternate printing)

// 7ED 164 — Spineless Thug (reprint)

// 7ED 164★ — Spineless Thug (alternate printing)

// 7ED 165 — Strands of Night (reprint)

// 7ED 165★ — Strands of Night (alternate printing)

// 7ED 166 — Stronghold Assassin (reprint)

// 7ED 166★ — Stronghold Assassin (alternate printing)

// 7ED 167 — Tainted Aether (reprint)

// 7ED 167★ — Tainted Aether (alternate printing)

// 7ED 168 — Unholy Strength (reprint)

// 7ED 168★ — Unholy Strength (alternate printing)

// 7ED 169 — Wall of Bone (reprint)

// 7ED 169★ — Wall of Bone (alternate printing)

// 7ED 170 — Western Paladin (reprint)

// 7ED 170★ — Western Paladin (alternate printing)

// 7ED 171 — Yawgmoth's Edict (reprint)

// 7ED 171★ — Yawgmoth's Edict (alternate printing)

// 7ED 172 — Aether Flash (reprint)

// 7ED 172★ — Aether Flash (alternate printing)

// 7ED 173 — Balduvian Barbarians (reprint)

// 7ED 173★ — Balduvian Barbarians (alternate printing)

// 7ED 174 — Bedlam (reprint)

// 7ED 174★ — Bedlam (alternate printing)

// 7ED 175 — Blaze (reprint)

// 7ED 175★ — Blaze (alternate printing)

// 7ED 176 — Bloodshot Cyclops (reprint)

// 7ED 176★ — Bloodshot Cyclops (alternate printing)

// 7ED 177 — Boil (reprint)

// 7ED 177★ — Boil (alternate printing)

// 7ED 178 — Crimson Hellkite (reprint)

// 7ED 178★ — Crimson Hellkite (alternate printing)

// 7ED 179 — Disorder (reprint)

// 7ED 179★ — Disorder (alternate printing)

// 7ED 180 — Earthquake (reprint)

// 7ED 180★ — Earthquake (alternate printing)

// 7ED 181 — Fervor (reprint)

// 7ED 181★ — Fervor (alternate printing)

// 7ED 182 — Final Fortune (reprint)

// 7ED 182★ — Final Fortune (alternate printing)

// 7ED 183 — Fire Elemental (reprint)

// 7ED 183★ — Fire Elemental (alternate printing)

// 7ED 184 — Ghitu Fire-Eater (reprint)

// 7ED 184★ — Ghitu Fire-Eater (alternate printing)

// 7ED 185 — Goblin Chariot (alternate printing)

// 7ED 185★ — Goblin Chariot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_CHARIOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ca11a7e-17f8-419f-9ba8-1bcaa3860f8b"),
    "Goblin Chariot",
    crate::card::CardArt::new("1db520e2-9926-45d2-a140-37b119b88106", "John Howe"),
    crate::card::CardSet::SeventhEdition,
    crate::card::CardRules::unsupported(),
);

// 7ED 186 — Goblin Digging Team (reprint)

// 7ED 186★ — Goblin Digging Team (alternate printing)

// 7ED 187 — Goblin Elite Infantry (reprint)

// 7ED 187★ — Goblin Elite Infantry (alternate printing)

// 7ED 188 — Goblin Gardener (reprint)

// 7ED 188★ — Goblin Gardener (alternate printing)

// 7ED 189 — Goblin Glider (reprint)

// 7ED 189★ — Goblin Glider (alternate printing)

// 7ED 190 — Goblin King (reprint)

// 7ED 190★ — Goblin King (alternate printing)

// 7ED 191 — Goblin Matron (reprint)

// 7ED 191★ — Goblin Matron (alternate printing)

// 7ED 192 — Goblin Raider (reprint)

// 7ED 192★ — Goblin Raider (alternate printing)

// 7ED 193 — Goblin Spelunkers (reprint)

// 7ED 193★ — Goblin Spelunkers (alternate printing)

// 7ED 194 — Goblin War Drums (reprint)

// 7ED 194★ — Goblin War Drums (alternate printing)

// 7ED 195 — Granite Grip (reprint)

// 7ED 195★ — Granite Grip (alternate printing)

// 7ED 196 — Hill Giant (reprint)

// 7ED 196★ — Hill Giant (alternate printing)

// 7ED 197 — Impatience (reprint)

// 7ED 197★ — Impatience (alternate printing)

// 7ED 198 — Inferno (reprint)

// 7ED 198★ — Inferno (alternate printing)

// 7ED 199 — Lava Axe (reprint)

// 7ED 199★ — Lava Axe (alternate printing)

// 7ED 200 — Lightning Blast (reprint)

// 7ED 200★ — Lightning Blast (alternate printing)

// 7ED 201 — Lightning Elemental (reprint)

// 7ED 201★ — Lightning Elemental (alternate printing)

// 7ED 202 — Mana Clash (reprint)

// 7ED 202★ — Mana Clash (alternate printing)

// 7ED 203 — Ogre Taskmaster (reprint)

// 7ED 203★ — Ogre Taskmaster (alternate printing)

// 7ED 204 — Okk (reprint)

// 7ED 204★ — Okk (alternate printing)

// 7ED 205 — Orcish Artillery (reprint)

// 7ED 205★ — Orcish Artillery (alternate printing)

// 7ED 206 — Orcish Oriflamme (reprint)

// 7ED 206★ — Orcish Oriflamme (alternate printing)

// 7ED 207 — Pillage (reprint)

// 7ED 207★ — Pillage (alternate printing)

// 7ED 208 — Pygmy Pyrosaur (reprint)

// 7ED 208★ — Pygmy Pyrosaur (alternate printing)

// 7ED 209 — Pyroclasm (reprint)

// 7ED 209★ — Pyroclasm (alternate printing)

// 7ED 210 — Pyrotechnics (reprint)

// 7ED 210★ — Pyrotechnics (alternate printing)

// 7ED 211 — Raging Goblin (reprint)

// 7ED 211★ — Raging Goblin (alternate printing)

// 7ED 212 — Reckless Embermage (reprint)

// 7ED 212★ — Reckless Embermage (alternate printing)

// 7ED 213 — Reflexes (reprint)

// 7ED 213★ — Reflexes (alternate printing)

// 7ED 214 — Relentless Assault (reprint)

// 7ED 214★ — Relentless Assault (alternate printing)

// 7ED 215 — Sabretooth Tiger (reprint)

// 7ED 215★ — Sabretooth Tiger (alternate printing)

// 7ED 216 — Seismic Assault (reprint)

// 7ED 216★ — Seismic Assault (alternate printing)

// 7ED 217 — Shatter (reprint)

// 7ED 217★ — Shatter (alternate printing)

// 7ED 218 — Shivan Dragon (reprint)

// 7ED 218★ — Shivan Dragon (alternate printing)

// 7ED 219 — Shock (reprint)

// 7ED 219★ — Shock (alternate printing)

// 7ED 220 — Spitting Earth (reprint)

// 7ED 220★ — Spitting Earth (alternate printing)

// 7ED 221 — Stone Rain (reprint)

// 7ED 221★ — Stone Rain (alternate printing)

// 7ED 222 — Storm Shaman (reprint)

// 7ED 222★ — Storm Shaman (alternate printing)

// 7ED 223 — Sudden Impact (reprint)

// 7ED 223★ — Sudden Impact (alternate printing)

// 7ED 224 — Trained Orgg
pub(in crate::card::sets) static TRAINED_ORGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("425540b0-c826-4814-b0df-032264b1c237"),
    "Trained Orgg",
    CardArt::new(
        "14a83031-8b57-41d2-b586-bb4dcf16136a",
        "Alex Horley-Orlandelli",
    ),
    CardSet::SeventhEdition,
    // Seven mana for a 6/6, red's half of the same starter-deck bargain.
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Orgg"], 6, 6),
);

// 7ED 224★ — Trained Orgg (alternate printing)

// 7ED 225 — Tremor (reprint)

// 7ED 225★ — Tremor (alternate printing)

// 7ED 226 — Volcanic Hammer (reprint)

// 7ED 226★ — Volcanic Hammer (alternate printing)

// 7ED 227 — Wall of Fire (reprint)

// 7ED 227★ — Wall of Fire (alternate printing)

// 7ED 228 — Wildfire (reprint)

// 7ED 228★ — Wildfire (alternate printing)

// 7ED 229 — Anaconda (reprint)

// 7ED 229★ — Anaconda (alternate printing)

// 7ED 230 — Ancient Silverback (reprint)

// 7ED 230★ — Ancient Silverback (alternate printing)

// 7ED 231 — Birds of Paradise (reprint)

// 7ED 231★ — Birds of Paradise (alternate printing)

// 7ED 232 — Blanchwood Armor (reprint)

// 7ED 232★ — Blanchwood Armor (alternate printing)

// 7ED 233 — Bull Hippo (reprint)

// 7ED 233★ — Bull Hippo (alternate printing)

// 7ED 234 — Canopy Spider (reprint)

// 7ED 234★ — Canopy Spider (alternate printing)

// 7ED 235 — Compost (reprint)

// 7ED 235★ — Compost (alternate printing)

// 7ED 236 — Creeping Mold (reprint)

// 7ED 236★ — Creeping Mold (alternate printing)

// 7ED 237 — Early Harvest (reprint)

// 7ED 237★ — Early Harvest (alternate printing)

// 7ED 238 — Elder Druid (reprint)

// 7ED 238★ — Elder Druid (alternate printing)

// 7ED 239 — Elvish Archers (reprint)

// 7ED 239★ — Elvish Archers (alternate printing)

// 7ED 240 — Elvish Champion (reprint)

// 7ED 240★ — Elvish Champion (alternate printing)

// 7ED 241 — Elvish Lyrist (reprint)

// 7ED 241★ — Elvish Lyrist (alternate printing)

// 7ED 242 — Elvish Piper (reprint)

// 7ED 242★ — Elvish Piper (alternate printing)

// 7ED 243 — Familiar Ground (reprint)

// 7ED 243★ — Familiar Ground (alternate printing)

// 7ED 244 — Femeref Archers (reprint)

// 7ED 244★ — Femeref Archers (alternate printing)

// 7ED 245 — Fog (reprint)

// 7ED 245★ — Fog (alternate printing)

// 7ED 246 — Fyndhorn Elder (reprint)

// 7ED 246★ — Fyndhorn Elder (alternate printing)

// 7ED 247 — Gang of Elk (reprint)

// 7ED 247★ — Gang of Elk (alternate printing)

// 7ED 248 — Giant Growth (reprint)

// 7ED 248★ — Giant Growth (alternate printing)

// 7ED 249 — Giant Spider (reprint)

// 7ED 249★ — Giant Spider (alternate printing)

// 7ED 250 — Gorilla Chieftain (reprint)

// 7ED 250★ — Gorilla Chieftain (alternate printing)

// 7ED 251 — Grizzly Bears (reprint)

// 7ED 251★ — Grizzly Bears (alternate printing)

// 7ED 252 — Hurricane (reprint)

// 7ED 252★ — Hurricane (alternate printing)

// 7ED 253 — Llanowar Elves (reprint)

// 7ED 253★ — Llanowar Elves (alternate printing)

// 7ED 254 — Lone Wolf (reprint)

// 7ED 254★ — Lone Wolf (alternate printing)

// 7ED 255 — Lure (reprint)

// 7ED 255★ — Lure (alternate printing)

// 7ED 256 — Maro (reprint)

// 7ED 256★ — Maro (alternate printing)

// 7ED 257 — Might of Oaks (reprint)

// 7ED 257★ — Might of Oaks (alternate printing)

// 7ED 258 — Monstrous Growth (reprint)

// 7ED 258★ — Monstrous Growth (alternate printing)

// 7ED 259 — Nature's Resurgence (reprint)

// 7ED 259★ — Nature's Resurgence (alternate printing)

// 7ED 260 — Nature's Revolt (reprint)

// 7ED 260★ — Nature's Revolt (alternate printing)

// 7ED 261 — Pride of Lions (alternate printing)

// 7ED 261★ — Pride of Lions
// Audit: unsupported — Needs a combat-damage assignment option that lets the attacker assign damage as though it were unblocked without actually becoming unblocked; see Lone Wolf.
pub(in crate::card::sets) static PRIDE_OF_LIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5006984-8e3d-4f13-b12e-1fbecd134bb3"),
    "Pride of Lions",
    crate::card::CardArt::new("1673b038-97b6-4139-8468-9cbbd01dd239", "Gary Ruddell"),
    crate::card::CardSet::SeventhEdition,
    crate::card::CardRules::unsupported(),
);

// 7ED 262 — Rampant Growth (reprint)

// 7ED 262★ — Rampant Growth (alternate printing)

// 7ED 263 — Reclaim (reprint)

// 7ED 263★ — Reclaim (alternate printing)

// 7ED 264 — Redwood Treefolk (reprint)

// 7ED 264★ — Redwood Treefolk (alternate printing)

// 7ED 265 — Regeneration (reprint)

// 7ED 265★ — Regeneration (alternate printing)

// 7ED 266 — Rowen (reprint)

// 7ED 266★ — Rowen (alternate printing)

// 7ED 267 — Scavenger Folk (reprint)

// 7ED 267★ — Scavenger Folk (alternate printing)

// 7ED 268 — Seeker of Skybreak (reprint)

// 7ED 268★ — Seeker of Skybreak (alternate printing)

// 7ED 269 — Shanodin Dryads (reprint)

// 7ED 269★ — Shanodin Dryads (alternate printing)

// 7ED 270 — Spined Wurm (reprint)

// 7ED 270★ — Spined Wurm (alternate printing)

// 7ED 271 — Squall (reprint)

// 7ED 271★ — Squall (alternate printing)

// 7ED 272 — Stream of Life (reprint)

// 7ED 272★ — Stream of Life (alternate printing)

// 7ED 273 — Thorn Elemental (reprint)

// 7ED 273★ — Thorn Elemental (alternate printing)

// 7ED 274 — Thoughtleech (reprint)

// 7ED 274★ — Thoughtleech (alternate printing)

// 7ED 275 — Trained Armodon (reprint)

// 7ED 275★ — Trained Armodon (alternate printing)

// 7ED 276 — Tranquility (reprint)

// 7ED 276★ — Tranquility (alternate printing)

// 7ED 277 — Treefolk Seedlings (reprint)

// 7ED 277★ — Treefolk Seedlings (alternate printing)

// 7ED 278 — Uktabi Wildcats (reprint)

// 7ED 278★ — Uktabi Wildcats (alternate printing)

// 7ED 279 — Untamed Wilds (reprint)

// 7ED 279★ — Untamed Wilds (alternate printing)

// 7ED 280 — Verduran Enchantress (reprint)

// 7ED 280★ — Verduran Enchantress (alternate printing)

// 7ED 281 — Vernal Bloom (reprint)

// 7ED 281★ — Vernal Bloom (alternate printing)

// 7ED 282 — Wild Growth (reprint)

// 7ED 282★ — Wild Growth (alternate printing)

// 7ED 283 — Wing Snare (reprint)

// 7ED 283★ — Wing Snare (alternate printing)

// 7ED 284 — Wood Elves (reprint)

// 7ED 284★ — Wood Elves (alternate printing)

// 7ED 285 — Yavimaya Enchantress (reprint)

// 7ED 285★ — Yavimaya Enchantress (alternate printing)

// 7ED 286 — Aladdin's Ring (reprint)

// 7ED 286★ — Aladdin's Ring (alternate printing)

// 7ED 287 — Beast of Burden (reprint)

// 7ED 287★ — Beast of Burden (alternate printing)

// 7ED 288 — Caltrops (reprint)

// 7ED 288★ — Caltrops (alternate printing)

// 7ED 289 — Charcoal Diamond (reprint)

// 7ED 289s — Charcoal Diamond (alternate printing)

// 7ED 289★ — Charcoal Diamond (alternate printing)

// 7ED 289★s — Charcoal Diamond (alternate printing)

// 7ED 290 — Coat of Arms (reprint)

// 7ED 290★ — Coat of Arms (alternate printing)

// 7ED 291 — Crystal Rod (reprint)

// 7ED 291★ — Crystal Rod (alternate printing)

// 7ED 292 — Dingus Egg (reprint)

// 7ED 292★ — Dingus Egg (alternate printing)

// 7ED 293 — Disrupting Scepter (reprint)

// 7ED 293★ — Disrupting Scepter (alternate printing)

// 7ED 294 — Ensnaring Bridge (reprint)

// 7ED 294★ — Ensnaring Bridge (alternate printing)

// 7ED 295 — Feroz's Ban (reprint)

// 7ED 295★ — Feroz's Ban (alternate printing)

// 7ED 296 — Fire Diamond (reprint)

// 7ED 296★ — Fire Diamond (alternate printing)

// 7ED 297 — Flying Carpet (reprint)

// 7ED 297★ — Flying Carpet (alternate printing)

// 7ED 298 — Grafted Skullcap (reprint)

// 7ED 298★ — Grafted Skullcap (alternate printing)

// 7ED 299 — Grapeshot Catapult (reprint)

// 7ED 299★ — Grapeshot Catapult (alternate printing)

// 7ED 300 — Howling Mine (reprint)

// 7ED 300★ — Howling Mine (alternate printing)

// 7ED 301 — Iron Star (reprint)

// 7ED 301★ — Iron Star (alternate printing)

// 7ED 302 — Ivory Cup (reprint)

// 7ED 302★ — Ivory Cup (alternate printing)

// 7ED 303 — Jalum Tome (reprint)

// 7ED 303★ — Jalum Tome (alternate printing)

// 7ED 304 — Jandor's Saddlebags (reprint)

// 7ED 304★ — Jandor's Saddlebags (alternate printing)

// 7ED 305 — Jayemdae Tome (reprint)

// 7ED 305★ — Jayemdae Tome (alternate printing)

// 7ED 306 — Marble Diamond (reprint)

// 7ED 306★ — Marble Diamond (alternate printing)

// 7ED 307 — Meekstone (reprint)

// 7ED 307★ — Meekstone (alternate printing)

// 7ED 308 — Millstone (reprint)

// 7ED 308★ — Millstone (alternate printing)

// 7ED 309 — Moss Diamond (reprint)

// 7ED 309★ — Moss Diamond (alternate printing)

// 7ED 310 — Patagia Golem (reprint)

// 7ED 310★ — Patagia Golem (alternate printing)

// 7ED 311 — Phyrexian Colossus (reprint)

// 7ED 311★ — Phyrexian Colossus (alternate printing)

// 7ED 312 — Phyrexian Hulk (reprint)

// 7ED 312★ — Phyrexian Hulk (alternate printing)

// 7ED 313 — Pit Trap (reprint)

// 7ED 313★ — Pit Trap (alternate printing)

// 7ED 314 — Rod of Ruin (reprint)

// 7ED 314★ — Rod of Ruin (alternate printing)

// 7ED 315 — Sisay's Ring (reprint)

// 7ED 315★ — Sisay's Ring (alternate printing)

// 7ED 316 — Sky Diamond (reprint)

// 7ED 316★ — Sky Diamond (alternate printing)

// 7ED 317 — Soul Net (reprint)

// 7ED 317★ — Soul Net (alternate printing)

// 7ED 318 — Spellbook (reprint)

// 7ED 318★ — Spellbook (alternate printing)

// 7ED 319 — Static Orb (reprint)

// 7ED 319★ — Static Orb (alternate printing)

// 7ED 320 — Storm Cauldron (reprint)

// 7ED 320★ — Storm Cauldron (alternate printing)

// 7ED 321 — Teferi's Puzzle Box (reprint)

// 7ED 321★ — Teferi's Puzzle Box (alternate printing)

// 7ED 322 — Throne of Bone (reprint)

// 7ED 322★ — Throne of Bone (alternate printing)

// 7ED 323 — Wall of Spears (reprint)

// 7ED 323★ — Wall of Spears (alternate printing)

// 7ED 324 — Wooden Sphere (reprint)

// 7ED 324★ — Wooden Sphere (alternate printing)

// 7ED 325 — Adarkar Wastes (reprint)

// 7ED 325★ — Adarkar Wastes (alternate printing)

// 7ED 326 — Brushland (reprint)

// 7ED 326★ — Brushland (alternate printing)

// 7ED 327 — City of Brass (reprint)

// 7ED 327★ — City of Brass (alternate printing)

// 7ED 328 — Forest (reprint)

// 7ED 328★ — Forest (alternate printing)

// 7ED 329 — Forest (alternate printing)

// 7ED 329★ — Forest (alternate printing)

// 7ED 330 — Forest (alternate printing)

// 7ED 330★ — Forest (alternate printing)

// 7ED 331 — Forest (alternate printing)

// 7ED 331★ — Forest (alternate printing)

// 7ED 332 — Island (reprint)

// 7ED 332★ — Island (alternate printing)

// 7ED 333 — Island (alternate printing)

// 7ED 333★ — Island (alternate printing)

// 7ED 334 — Island (alternate printing)

// 7ED 334★ — Island (alternate printing)

// 7ED 335 — Island (alternate printing)

// 7ED 335★ — Island (alternate printing)

// 7ED 336 — Karplusan Forest (reprint)

// 7ED 336★ — Karplusan Forest (alternate printing)

// 7ED 337 — Mountain (reprint)

// 7ED 337★ — Mountain (alternate printing)

// 7ED 338 — Mountain (alternate printing)

// 7ED 338★ — Mountain (alternate printing)

// 7ED 339 — Mountain (alternate printing)

// 7ED 339★ — Mountain (alternate printing)

// 7ED 340 — Mountain (alternate printing)

// 7ED 340★ — Mountain (alternate printing)

// 7ED 341 — Plains (reprint)

// 7ED 341★ — Plains (alternate printing)

// 7ED 342 — Plains (alternate printing)

// 7ED 342★ — Plains (alternate printing)

// 7ED 343 — Plains (alternate printing)

// 7ED 343★ — Plains (alternate printing)

// 7ED 344 — Plains (alternate printing)

// 7ED 344★ — Plains (alternate printing)

// 7ED 345 — Sulfurous Springs (reprint)

// 7ED 345★ — Sulfurous Springs (alternate printing)

// 7ED 346 — Swamp (reprint)

// 7ED 346★ — Swamp (alternate printing)

// 7ED 347 — Swamp (alternate printing)

// 7ED 347★ — Swamp (alternate printing)

// 7ED 348 — Swamp (alternate printing)

// 7ED 348★ — Swamp (alternate printing)

// 7ED 349 — Swamp (alternate printing)

// 7ED 349★ — Swamp (alternate printing)

// 7ED 350 — Underground River (reprint)

// 7ED 350★ — Underground River (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EAGER_CADET,
    &KNIGHT_ERRANT,
    &SACRED_NECTAR,
    &STARLIGHT,
    &BALEFUL_STARE,
    &GIANT_OCTOPUS,
    &VIZZERDRIX,
    &DAKMOR_LANCER,
    &GOBLIN_CHARIOT,
    &TRAINED_ORGG,
    &PRIDE_OF_LIONS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_usg::ANGELIC_PAGE), // 7ED 1
    PrintingRecord::alternate(&catalog_usg::ANGELIC_PAGE, 1), // 7ED 1★
    PrintingRecord::reprint(&catalog_wth::ARDENT_MILITIA), // 7ED 2
    PrintingRecord::alternate(&catalog_wth::ARDENT_MILITIA, 1), // 7ED 2★
    PrintingRecord::reprint(&catalog_ulg::BLESSED_REVERSAL), // 7ED 3
    PrintingRecord::alternate(&catalog_ulg::BLESSED_REVERSAL, 1), // 7ED 3★
    PrintingRecord::reprint(&catalog_p02::BREATH_OF_LIFE), // 7ED 4
    PrintingRecord::alternate(&catalog_p02::BREATH_OF_LIFE, 1), // 7ED 4★
    PrintingRecord::reprint(&catalog_lea::CASTLE),       // 7ED 5
    PrintingRecord::alternate(&catalog_lea::CASTLE, 1),  // 7ED 5★
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // 7ED 6
    PrintingRecord::alternate(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK, 1), // 7ED 6★
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE), // 7ED 7
    PrintingRecord::alternate(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE, 1), // 7ED 7★
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // 7ED 8
    PrintingRecord::alternate(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN, 1), // 7ED 8★
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED), // 7ED 9
    PrintingRecord::alternate(&catalog_lea::CIRCLE_OF_PROTECTION_RED, 1), // 7ED 9★
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // 7ED 10
    PrintingRecord::alternate(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE, 1), // 7ED 10★
    PrintingRecord::reprint(&catalog_tmp::CLOUDCHASER_EAGLE), // 7ED 11
    PrintingRecord::alternate(&catalog_tmp::CLOUDCHASER_EAGLE, 1), // 7ED 11★
    PrintingRecord::reprint(&catalog_mmq::CROSSBOW_INFANTRY), // 7ED 12
    PrintingRecord::alternate(&catalog_mmq::CROSSBOW_INFANTRY, 1), // 7ED 12★
    PrintingRecord::reprint(&alpha::DISENCHANT),         // 7ED 13
    PrintingRecord::alternate(&catalog_lea::DISENCHANT, 1), // 7ED 13★
    PrintingRecord::alternate(&EAGER_CADET, 1),          // 7ED 14★
    PrintingRecord::reprint(&catalog_usg::ELITE_ARCHERS), // 7ED 15
    PrintingRecord::alternate(&catalog_usg::ELITE_ARCHERS, 1), // 7ED 15★
    PrintingRecord::reprint(&catalog_wth::GERRARD_S_WISDOM), // 7ED 16
    PrintingRecord::alternate(&catalog_wth::GERRARD_S_WISDOM, 1), // 7ED 16★
    PrintingRecord::reprint(&catalog_usg::GLORIOUS_ANTHEM), // 7ED 17
    PrintingRecord::alternate(&catalog_usg::GLORIOUS_ANTHEM, 1), // 7ED 17★
    PrintingRecord::reprint(&catalog_lea::HEALING_SALVE), // 7ED 18
    PrintingRecord::alternate(&catalog_lea::HEALING_SALVE, 1), // 7ED 18★
    PrintingRecord::reprint(&catalog_wth::HEAVY_BALLISTA), // 7ED 19
    PrintingRecord::alternate(&catalog_wth::HEAVY_BALLISTA, 1), // 7ED 19★
    PrintingRecord::reprint(&catalog_lea::HOLY_STRENGTH), // 7ED 20
    PrintingRecord::alternate(&catalog_lea::HOLY_STRENGTH, 1), // 7ED 20★
    PrintingRecord::reprint(&catalog_sth::HONOR_GUARD),  // 7ED 21
    PrintingRecord::alternate(&catalog_sth::HONOR_GUARD, 1), // 7ED 21★
    PrintingRecord::reprint(&catalog_usg::INTREPID_HERO), // 7ED 22
    PrintingRecord::alternate(&catalog_usg::INTREPID_HERO, 1), // 7ED 22★
    PrintingRecord::reprint(&catalog_ice::KJELDORAN_ROYAL_GUARD), // 7ED 23
    PrintingRecord::alternate(&catalog_ice::KJELDORAN_ROYAL_GUARD, 1), // 7ED 23★
    PrintingRecord::alternate(&KNIGHT_ERRANT, 1),        // 7ED 24
    PrintingRecord::reprint(&catalog_ulg::KNIGHTHOOD),   // 7ED 25
    PrintingRecord::alternate(&catalog_ulg::KNIGHTHOOD, 1), // 7ED 25★
    PrintingRecord::reprint(&catalog_vis::LONGBOW_ARCHER), // 7ED 26
    PrintingRecord::alternate(&catalog_vis::LONGBOW_ARCHER, 1), // 7ED 26★
    PrintingRecord::reprint(&catalog_uds::MASTER_HEALER), // 7ED 27
    PrintingRecord::alternate(&catalog_uds::MASTER_HEALER, 1), // 7ED 27★
    PrintingRecord::reprint(&catalog_lea::NORTHERN_PALADIN), // 7ED 28
    PrintingRecord::alternate(&catalog_lea::NORTHERN_PALADIN, 1), // 7ED 28★
    PrintingRecord::reprint(&catalog_m13::PACIFISM),     // 7ED 29
    PrintingRecord::alternate(&catalog_m13::PACIFISM, 1), // 7ED 29★
    PrintingRecord::reprint(&catalog_usg::PARIAH),       // 7ED 30
    PrintingRecord::alternate(&catalog_usg::PARIAH, 1),  // 7ED 30★
    PrintingRecord::reprint(&catalog_ulg::PURIFY),       // 7ED 31
    PrintingRecord::alternate(&catalog_ulg::PURIFY, 1),  // 7ED 31★
    PrintingRecord::reprint(&catalog_inv::RAZORFOOT_GRIFFIN), // 7ED 32
    PrintingRecord::alternate(&catalog_inv::RAZORFOOT_GRIFFIN, 1), // 7ED 32★
    PrintingRecord::reprint(&catalog_all::REPRISAL),     // 7ED 33
    PrintingRecord::alternate(&catalog_all::REPRISAL, 1), // 7ED 33★
    PrintingRecord::reprint(&catalog_lea::REVERSE_DAMAGE), // 7ED 34
    PrintingRecord::alternate(&catalog_lea::REVERSE_DAMAGE, 1), // 7ED 34★
    PrintingRecord::reprint(&catalog_sth::ROLLING_STONES), // 7ED 35
    PrintingRecord::alternate(&catalog_sth::ROLLING_STONES, 1), // 7ED 35★
    PrintingRecord::reprint(&catalog_sth::SACRED_GROUND), // 7ED 36
    PrintingRecord::alternate(&catalog_sth::SACRED_GROUND, 1), // 7ED 36★
    PrintingRecord::alternate(&SACRED_NECTAR, 1),        // 7ED 37
    PrintingRecord::reprint(&catalog_lea::SAMITE_HEALER), // 7ED 38
    PrintingRecord::alternate(&catalog_lea::SAMITE_HEALER, 1), // 7ED 38★
    PrintingRecord::reprint(&catalog_uds::SANCTIMONY),   // 7ED 39
    PrintingRecord::alternate(&catalog_uds::SANCTIMONY, 1), // 7ED 39★
    PrintingRecord::reprint(&catalog_usg::SEASONED_MARSHAL), // 7ED 40
    PrintingRecord::alternate(&catalog_usg::SEASONED_MARSHAL, 1), // 7ED 40★
    PrintingRecord::reprint(&catalog_uds::SERRA_ADVOCATE), // 7ED 41
    PrintingRecord::alternate(&catalog_uds::SERRA_ADVOCATE, 1), // 7ED 41★
    PrintingRecord::reprint(&catalog_lea::SERRA_ANGEL),  // 7ED 42
    PrintingRecord::alternate(&catalog_lea::SERRA_ANGEL, 1), // 7ED 42★
    PrintingRecord::reprint(&catalog_usg::SERRA_S_EMBRACE), // 7ED 43
    PrintingRecord::alternate(&catalog_usg::SERRA_S_EMBRACE, 1), // 7ED 43★
    PrintingRecord::reprint(&catalog_leg::SHIELD_WALL),  // 7ED 44
    PrintingRecord::alternate(&catalog_leg::SHIELD_WALL, 1), // 7ED 44★
    PrintingRecord::reprint(&catalog_sth::SKYSHROUD_FALCON), // 7ED 45
    PrintingRecord::alternate(&catalog_sth::SKYSHROUD_FALCON, 1), // 7ED 45★
    PrintingRecord::reprint(&catalog_wth::SOUTHERN_PALADIN), // 7ED 46
    PrintingRecord::alternate(&catalog_wth::SOUTHERN_PALADIN, 1), // 7ED 46★
    PrintingRecord::reprint(&catalog_leg::SPIRIT_LINK),  // 7ED 47
    PrintingRecord::alternate(&catalog_leg::SPIRIT_LINK, 1), // 7ED 47★
    PrintingRecord::reprint(&catalog_exo::STANDING_TROOPS), // 7ED 48
    PrintingRecord::alternate(&catalog_exo::STANDING_TROOPS, 1), // 7ED 48★
    PrintingRecord::alternate(&STARLIGHT, 1),            // 7ED 49
    PrintingRecord::reprint(&catalog_tmp::STAUNCH_DEFENDERS), // 7ED 50
    PrintingRecord::alternate(&catalog_tmp::STAUNCH_DEFENDERS, 1), // 7ED 50★
    PrintingRecord::reprint(&catalog_mir::SUNWEB),       // 7ED 51
    PrintingRecord::alternate(&catalog_mir::SUNWEB, 1),  // 7ED 51★
    PrintingRecord::reprint(&catalog_ulg::SUSTAINER_OF_THE_REALM), // 7ED 52
    PrintingRecord::alternate(&catalog_ulg::SUSTAINER_OF_THE_REALM, 1), // 7ED 52★
    PrintingRecord::reprint(&catalog_sth::VENERABLE_MONK), // 7ED 53
    PrintingRecord::alternate(&catalog_sth::VENERABLE_MONK, 1), // 7ED 53★
    PrintingRecord::reprint(&catalog_p02::VENGEANCE),    // 7ED 54
    PrintingRecord::alternate(&catalog_p02::VENGEANCE, 1), // 7ED 54★
    PrintingRecord::reprint(&catalog_lea::WALL_OF_SWORDS), // 7ED 55
    PrintingRecord::alternate(&catalog_lea::WALL_OF_SWORDS, 1), // 7ED 55★
    PrintingRecord::reprint(&catalog_usg::WORSHIP),      // 7ED 56
    PrintingRecord::alternate(&catalog_usg::WORSHIP, 1), // 7ED 56★
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),       // 7ED 57
    PrintingRecord::alternate(&catalog_lea::WRATH_OF_GOD, 1), // 7ED 57★
    PrintingRecord::reprint(&catalog_lea::AIR_ELEMENTAL), // 7ED 58
    PrintingRecord::alternate(&catalog_lea::AIR_ELEMENTAL, 1), // 7ED 58★
    PrintingRecord::reprint(&catalog_mir::ANCESTRAL_MEMORIES), // 7ED 59
    PrintingRecord::alternate(&catalog_mir::ANCESTRAL_MEMORIES, 1), // 7ED 59★
    PrintingRecord::reprint(&catalog_usg::ARCANE_LABORATORY), // 7ED 60
    PrintingRecord::alternate(&catalog_usg::ARCANE_LABORATORY, 1), // 7ED 60★
    PrintingRecord::reprint(&catalog_ulg::ARCHIVIST),    // 7ED 61
    PrintingRecord::alternate(&catalog_ulg::ARCHIVIST, 1), // 7ED 61★
    PrintingRecord::alternate(&BALEFUL_STARE, 1),        // 7ED 62★
    PrintingRecord::reprint(&catalog_tmp::BENTHIC_BEHEMOTH), // 7ED 63
    PrintingRecord::alternate(&catalog_tmp::BENTHIC_BEHEMOTH, 1), // 7ED 63★
    PrintingRecord::reprint(&catalog_leg::BOOMERANG),    // 7ED 64
    PrintingRecord::alternate(&catalog_leg::BOOMERANG, 1), // 7ED 64★
    PrintingRecord::reprint(&catalog_usg::CONFISCATE),   // 7ED 65
    PrintingRecord::alternate(&catalog_usg::CONFISCATE, 1), // 7ED 65★
    PrintingRecord::reprint(&catalog_m14::CORAL_MERFOLK), // 7ED 66
    PrintingRecord::alternate(&catalog_m14::CORAL_MERFOLK, 1), // 7ED 66★
    PrintingRecord::reprint(&catalog_lea::COUNTERSPELL), // 7ED 67
    PrintingRecord::alternate(&catalog_lea::COUNTERSPELL, 1), // 7ED 67★
    PrintingRecord::reprint(&catalog_mir::DARING_APPRENTICE), // 7ED 68
    PrintingRecord::alternate(&catalog_mir::DARING_APPRENTICE, 1), // 7ED 68★
    PrintingRecord::reprint(&catalog_ice::DEFLECTION),   // 7ED 69
    PrintingRecord::alternate(&catalog_ice::DEFLECTION, 1), // 7ED 69★
    PrintingRecord::reprint(&catalog_ulg::DELUSIONS_OF_MEDIOCRITY), // 7ED 70
    PrintingRecord::alternate(&catalog_ulg::DELUSIONS_OF_MEDIOCRITY, 1), // 7ED 70★
    PrintingRecord::reprint(&catalog_exo::EQUILIBRIUM),  // 7ED 71
    PrintingRecord::alternate(&catalog_exo::EQUILIBRIUM, 1), // 7ED 71★
    PrintingRecord::reprint(&catalog_sth::EVACUATION),   // 7ED 72
    PrintingRecord::alternate(&catalog_sth::EVACUATION, 1), // 7ED 72★
    PrintingRecord::reprint(&catalog_tmp::FIGHTING_DRAKE), // 7ED 73
    PrintingRecord::alternate(&catalog_tmp::FIGHTING_DRAKE, 1), // 7ED 73★
    PrintingRecord::reprint(&catalog_ulg::FLEETING_IMAGE), // 7ED 74
    PrintingRecord::alternate(&catalog_ulg::FLEETING_IMAGE, 1), // 7ED 74★
    PrintingRecord::reprint(&catalog_lea::FLIGHT),       // 7ED 75
    PrintingRecord::alternate(&catalog_lea::FLIGHT, 1),  // 7ED 75★
    PrintingRecord::reprint(&catalog_leg::FORCE_SPIKE),  // 7ED 76
    PrintingRecord::alternate(&catalog_leg::FORCE_SPIKE, 1), // 7ED 76★
    PrintingRecord::alternate(&GIANT_OCTOPUS, 1),        // 7ED 77★
    PrintingRecord::reprint(&catalog_ice::GLACIAL_WALL), // 7ED 78
    PrintingRecord::alternate(&catalog_ice::GLACIAL_WALL, 1), // 7ED 78★
    PrintingRecord::reprint(&catalog_usg::HIBERNATION),  // 7ED 79
    PrintingRecord::alternate(&catalog_usg::HIBERNATION, 1), // 7ED 79★
    PrintingRecord::reprint(&catalog_tmp::HORNED_TURTLE), // 7ED 80
    PrintingRecord::alternate(&catalog_tmp::HORNED_TURTLE, 1), // 7ED 80★
    PrintingRecord::reprint(&catalog_rtr::INSPIRATION),  // 7ED 81
    PrintingRecord::alternate(&catalog_rtr::INSPIRATION, 1), // 7ED 81★
    PrintingRecord::reprint(&catalog_m12::LEVITATION),   // 7ED 82
    PrintingRecord::alternate(&catalog_m12::LEVITATION, 1), // 7ED 82★
    PrintingRecord::reprint(&catalog_lea::LORD_OF_ATLANTIS), // 7ED 83
    PrintingRecord::alternate(&catalog_lea::LORD_OF_ATLANTIS, 1), // 7ED 83★
    PrintingRecord::reprint(&catalog_lea::MAHAMOTI_DJINN), // 7ED 84
    PrintingRecord::alternate(&catalog_lea::MAHAMOTI_DJINN, 1), // 7ED 84★
    PrintingRecord::reprint(&catalog_exo::MANA_BREACH),  // 7ED 85
    PrintingRecord::alternate(&catalog_exo::MANA_BREACH, 1), // 7ED 85★
    PrintingRecord::reprint(&alpha::MANA_SHORT),         // 7ED 86
    PrintingRecord::alternate(&catalog_lea::MANA_SHORT, 1), // 7ED 86★
    PrintingRecord::reprint(&catalog_tmp::MAWCOR),       // 7ED 87
    PrintingRecord::alternate(&catalog_tmp::MAWCOR, 1),  // 7ED 87★
    PrintingRecord::reprint(&catalog_hml::MEMORY_LAPSE), // 7ED 88
    PrintingRecord::alternate(&catalog_hml::MEMORY_LAPSE, 1), // 7ED 88★
    PrintingRecord::reprint(&catalog_m12::MERFOLK_LOOTER), // 7ED 89
    PrintingRecord::alternate(&catalog_m12::MERFOLK_LOOTER, 1), // 7ED 89★
    PrintingRecord::reprint(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT), // 7ED 90
    PrintingRecord::alternate(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT, 1), // 7ED 90★
    PrintingRecord::reprint(&catalog_m14::OPPORTUNITY),  // 7ED 91
    PrintingRecord::alternate(&catalog_m14::OPPORTUNITY, 1), // 7ED 91★
    PrintingRecord::reprint(&catalog_uds::OPPOSITION),   // 7ED 92
    PrintingRecord::alternate(&catalog_uds::OPPOSITION, 1), // 7ED 92★
    PrintingRecord::reprint(&catalog_m14::PHANTOM_WARRIOR), // 7ED 93
    PrintingRecord::alternate(&catalog_m14::PHANTOM_WARRIOR, 1), // 7ED 93★
    PrintingRecord::reprint(&catalog_lea::PRODIGAL_SORCERER), // 7ED 94
    PrintingRecord::alternate(&catalog_lea::PRODIGAL_SORCERER, 1), // 7ED 94★
    PrintingRecord::reprint(&catalog_leg::REMOVE_SOUL),  // 7ED 95
    PrintingRecord::alternate(&catalog_leg::REMOVE_SOUL, 1), // 7ED 95★
    PrintingRecord::reprint(&catalog_wth::SAGE_OWL),     // 7ED 96
    PrintingRecord::alternate(&catalog_wth::SAGE_OWL, 1), // 7ED 96★
    PrintingRecord::reprint(&catalog_tmp::SEA_MONSTER),  // 7ED 97
    PrintingRecord::alternate(&catalog_tmp::SEA_MONSTER, 1), // 7ED 97★
    PrintingRecord::reprint(&portal_second_age::SLEIGHT_OF_HAND), // 7ED 98
    PrintingRecord::alternate(&catalog_p02::SLEIGHT_OF_HAND, 1), // 7ED 98★
    PrintingRecord::reprint(&catalog_lea::STEAL_ARTIFACT), // 7ED 99
    PrintingRecord::alternate(&catalog_lea::STEAL_ARTIFACT, 1), // 7ED 99★
    PrintingRecord::reprint(&catalog_all::STORM_CROW),   // 7ED 100
    PrintingRecord::alternate(&catalog_all::STORM_CROW, 1), // 7ED 100★
    PrintingRecord::reprint(&catalog_uds::TELEPATHIC_SPIES), // 7ED 101
    PrintingRecord::alternate(&catalog_uds::TELEPATHIC_SPIES, 1), // 7ED 101★
    PrintingRecord::reprint(&catalog_usg::TELEPATHY),    // 7ED 102
    PrintingRecord::alternate(&catalog_usg::TELEPATHY, 1), // 7ED 102★
    PrintingRecord::reprint(&catalog_uds::TEMPORAL_ADEPT), // 7ED 103
    PrintingRecord::alternate(&catalog_uds::TEMPORAL_ADEPT, 1), // 7ED 103★
    PrintingRecord::reprint(&catalog_uds::THIEVING_MAGPIE), // 7ED 104
    PrintingRecord::alternate(&catalog_uds::THIEVING_MAGPIE, 1), // 7ED 104★
    PrintingRecord::reprint(&catalog_usg::TOLARIAN_WINDS), // 7ED 105
    PrintingRecord::alternate(&catalog_usg::TOLARIAN_WINDS, 1), // 7ED 105★
    PrintingRecord::reprint(&catalog_exo::TREASURE_TROVE), // 7ED 106
    PrintingRecord::alternate(&catalog_exo::TREASURE_TROVE, 1), // 7ED 106★
    PrintingRecord::reprint(&catalog_lea::TWIDDLE),      // 7ED 107
    PrintingRecord::alternate(&catalog_lea::TWIDDLE, 1), // 7ED 107★
    PrintingRecord::reprint(&catalog_lea::UNSUMMON),     // 7ED 108
    PrintingRecord::alternate(&catalog_lea::UNSUMMON, 1), // 7ED 108★
    PrintingRecord::reprint(&catalog_ulg::VIGILANT_DRAKE), // 7ED 109
    PrintingRecord::alternate(&catalog_ulg::VIGILANT_DRAKE, 1), // 7ED 109★
    PrintingRecord::alternate(&VIZZERDRIX, 1),           // 7ED 110
    PrintingRecord::reprint(&catalog_lea::WALL_OF_AIR),  // 7ED 111
    PrintingRecord::alternate(&catalog_lea::WALL_OF_AIR, 1), // 7ED 111★
    PrintingRecord::reprint(&catalog_leg::WALL_OF_WONDER), // 7ED 112
    PrintingRecord::alternate(&catalog_leg::WALL_OF_WONDER, 1), // 7ED 112★
    PrintingRecord::reprint(&catalog_tmp::WIND_DANCER),  // 7ED 113
    PrintingRecord::alternate(&catalog_tmp::WIND_DANCER, 1), // 7ED 113★
    PrintingRecord::reprint(&catalog_m13::WIND_DRAKE),   // 7ED 114
    PrintingRecord::alternate(&catalog_m13::WIND_DRAKE, 1), // 7ED 114★
    PrintingRecord::reprint(&catalog_usg::ABYSSAL_HORROR), // 7ED 115
    PrintingRecord::alternate(&catalog_usg::ABYSSAL_HORROR, 1), // 7ED 115★
    PrintingRecord::reprint(&catalog_ice::ABYSSAL_SPECTER), // 7ED 116
    PrintingRecord::alternate(&catalog_ice::ABYSSAL_SPECTER, 1), // 7ED 116★
    PrintingRecord::reprint(&catalog_wth::AGONIZING_MEMORIES), // 7ED 117
    PrintingRecord::alternate(&catalog_wth::AGONIZING_MEMORIES, 1), // 7ED 117★
    PrintingRecord::reprint(&catalog_usg::BEFOUL),       // 7ED 118
    PrintingRecord::alternate(&catalog_usg::BEFOUL, 1),  // 7ED 118★
    PrintingRecord::reprint(&catalog_tmp::BELLOWING_FIEND), // 7ED 119
    PrintingRecord::alternate(&catalog_tmp::BELLOWING_FIEND, 1), // 7ED 119★
    PrintingRecord::reprint(&catalog_usg::BEREAVEMENT),  // 7ED 120
    PrintingRecord::alternate(&catalog_usg::BEREAVEMENT, 1), // 7ED 120★
    PrintingRecord::reprint(&catalog_tmp::BLOOD_PET),    // 7ED 121
    PrintingRecord::alternate(&catalog_tmp::BLOOD_PET, 1), // 7ED 121★
    PrintingRecord::reprint(&catalog_drk::BOG_IMP),      // 7ED 122
    PrintingRecord::alternate(&catalog_drk::BOG_IMP, 1), // 7ED 122★
    PrintingRecord::reprint(&catalog_lea::BOG_WRAITH),   // 7ED 123
    PrintingRecord::alternate(&catalog_lea::BOG_WRAITH, 1), // 7ED 123★
    PrintingRecord::reprint(&catalog_usg::CORRUPT),      // 7ED 124
    PrintingRecord::alternate(&catalog_usg::CORRUPT, 1), // 7ED 124★
    PrintingRecord::reprint(&catalog_vis::CRYPT_RATS),   // 7ED 125
    PrintingRecord::alternate(&catalog_vis::CRYPT_RATS, 1), // 7ED 125★
    PrintingRecord::alternate(&DAKMOR_LANCER, 1),        // 7ED 126★
    PrintingRecord::reprint(&catalog_ice::DARK_BANISHING), // 7ED 127
    PrintingRecord::alternate(&catalog_ice::DARK_BANISHING, 1), // 7ED 127★
    PrintingRecord::reprint(&catalog_usg::DARKEST_HOUR), // 7ED 128
    PrintingRecord::alternate(&catalog_usg::DARKEST_HOUR, 1), // 7ED 128★
    PrintingRecord::reprint(&catalog_tmp::DREGS_OF_SORROW), // 7ED 129
    PrintingRecord::alternate(&catalog_tmp::DREGS_OF_SORROW, 1), // 7ED 129★
    PrintingRecord::reprint(&catalog_lea::DRUDGE_SKELETONS), // 7ED 130
    PrintingRecord::alternate(&catalog_lea::DRUDGE_SKELETONS, 1), // 7ED 130s
    PrintingRecord::alternate(&catalog_lea::DRUDGE_SKELETONS, 2), // 7ED 130★
    PrintingRecord::alternate(&catalog_lea::DRUDGE_SKELETONS, 3), // 7ED 130★s
    PrintingRecord::reprint(&magic_2013::DURESS),        // 7ED 131
    PrintingRecord::alternate(&catalog_m13::DURESS, 1),  // 7ED 131★
    PrintingRecord::reprint(&catalog_usg::EASTERN_PALADIN), // 7ED 132
    PrintingRecord::alternate(&catalog_usg::EASTERN_PALADIN, 1), // 7ED 132★
    PrintingRecord::reprint(&catalog_ulg::ENGINEERED_PLAGUE), // 7ED 133
    PrintingRecord::alternate(&catalog_ulg::ENGINEERED_PLAGUE, 1), // 7ED 133★
    PrintingRecord::reprint(&catalog_leg::FALLEN_ANGEL), // 7ED 134
    PrintingRecord::alternate(&catalog_leg::FALLEN_ANGEL, 1), // 7ED 134★
    PrintingRecord::reprint(&catalog_lea::FEAR),         // 7ED 135
    PrintingRecord::alternate(&catalog_lea::FEAR, 1),    // 7ED 135★
    PrintingRecord::reprint(&catalog_sth::FOUL_IMP),     // 7ED 136
    PrintingRecord::alternate(&catalog_sth::FOUL_IMP, 1), // 7ED 136★
    PrintingRecord::reprint(&catalog_exo::FUGUE),        // 7ED 137
    PrintingRecord::alternate(&catalog_exo::FUGUE, 1),   // 7ED 137★
    PrintingRecord::reprint(&catalog_ulg::GIANT_COCKROACH), // 7ED 138
    PrintingRecord::alternate(&catalog_ulg::GIANT_COCKROACH, 1), // 7ED 138★
    PrintingRecord::reprint(&catalog_m12::GRAVEDIGGER),  // 7ED 139
    PrintingRecord::alternate(&catalog_m12::GRAVEDIGGER, 1), // 7ED 139★
    PrintingRecord::reprint(&catalog_leg::GREED),        // 7ED 140
    PrintingRecord::alternate(&catalog_leg::GREED, 1),   // 7ED 140★
    PrintingRecord::reprint(&catalog_usg::HOLLOW_DOGS),  // 7ED 141
    PrintingRecord::alternate(&catalog_usg::HOLLOW_DOGS, 1), // 7ED 141★
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND), // 7ED 142
    PrintingRecord::alternate(&catalog_lea::HOWL_FROM_BEYOND, 1), // 7ED 142★
    PrintingRecord::reprint(&catalog_mir::INFERNAL_CONTRACT), // 7ED 143
    PrintingRecord::alternate(&catalog_mir::INFERNAL_CONTRACT, 1), // 7ED 143★
    PrintingRecord::reprint(&catalog_ice::LESHRAC_S_RITE), // 7ED 144
    PrintingRecord::alternate(&catalog_ice::LESHRAC_S_RITE, 1), // 7ED 144★
    PrintingRecord::reprint(&catalog_usg::LOOMING_SHADE), // 7ED 145
    PrintingRecord::alternate(&catalog_usg::LOOMING_SHADE, 1), // 7ED 145★
    PrintingRecord::reprint(&catalog_sth::MEGRIM),       // 7ED 146
    PrintingRecord::alternate(&catalog_sth::MEGRIM, 1),  // 7ED 146★
    PrintingRecord::reprint(&catalog_m13::MIND_ROT),     // 7ED 147
    PrintingRecord::alternate(&catalog_m13::MIND_ROT, 1), // 7ED 147★
    PrintingRecord::reprint(&catalog_exo::NAUSEA),       // 7ED 148
    PrintingRecord::alternate(&catalog_exo::NAUSEA, 1),  // 7ED 148★
    PrintingRecord::reprint(&catalog_exo::NECROLOGIA),   // 7ED 149
    PrintingRecord::alternate(&catalog_exo::NECROLOGIA, 1), // 7ED 149★
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),    // 7ED 150
    PrintingRecord::alternate(&catalog_lea::NIGHTMARE, 1), // 7ED 150★
    PrintingRecord::reprint(&catalog_mir::NOCTURNAL_RAID), // 7ED 151
    PrintingRecord::alternate(&catalog_mir::NOCTURNAL_RAID, 1), // 7ED 151★
    PrintingRecord::reprint(&catalog_usg::OPPRESSION),   // 7ED 152
    PrintingRecord::alternate(&catalog_usg::OPPRESSION, 1), // 7ED 152★
    PrintingRecord::reprint(&catalog_ulg::OSTRACIZE),    // 7ED 153
    PrintingRecord::alternate(&catalog_ulg::OSTRACIZE, 1), // 7ED 153★
    PrintingRecord::reprint(&catalog_usg::PERSECUTE),    // 7ED 154
    PrintingRecord::alternate(&catalog_usg::PERSECUTE, 1), // 7ED 154★
    PrintingRecord::reprint(&catalog_ulg::PLAGUE_BEETLE), // 7ED 155
    PrintingRecord::alternate(&catalog_ulg::PLAGUE_BEETLE, 1), // 7ED 155★
    PrintingRecord::reprint(&catalog_drk::RAG_MAN),      // 7ED 156
    PrintingRecord::alternate(&catalog_drk::RAG_MAN, 1), // 7ED 156★
    PrintingRecord::reprint(&catalog_lea::RAISE_DEAD),   // 7ED 157
    PrintingRecord::alternate(&catalog_lea::RAISE_DEAD, 1), // 7ED 157s
    PrintingRecord::alternate(&catalog_lea::RAISE_DEAD, 2), // 7ED 157★
    PrintingRecord::alternate(&catalog_lea::RAISE_DEAD, 3), // 7ED 157★s
    PrintingRecord::reprint(&catalog_wth::RAZORTOOTH_RATS), // 7ED 158
    PrintingRecord::alternate(&catalog_wth::RAZORTOOTH_RATS, 1), // 7ED 158★
    PrintingRecord::reprint(&catalog_usg::REPROCESS),    // 7ED 159
    PrintingRecord::alternate(&catalog_usg::REPROCESS, 1), // 7ED 159★
    PrintingRecord::reprint(&catalog_sth::REVENANT),     // 7ED 160
    PrintingRecord::alternate(&catalog_sth::REVENANT, 1), // 7ED 160★
    PrintingRecord::reprint(&catalog_lea::SCATHE_ZOMBIES), // 7ED 161
    PrintingRecord::alternate(&catalog_lea::SCATHE_ZOMBIES, 1), // 7ED 161s
    PrintingRecord::alternate(&catalog_lea::SCATHE_ZOMBIES, 2), // 7ED 161★
    PrintingRecord::alternate(&catalog_lea::SCATHE_ZOMBIES, 3), // 7ED 161★s
    PrintingRecord::reprint(&catalog_sth::SERPENT_WARRIOR), // 7ED 162
    PrintingRecord::alternate(&catalog_sth::SERPENT_WARRIOR, 1), // 7ED 162★
    PrintingRecord::reprint(&catalog_uds::SOUL_FEAST),   // 7ED 163
    PrintingRecord::alternate(&catalog_uds::SOUL_FEAST, 1), // 7ED 163★
    PrintingRecord::reprint(&catalog_nem::SPINELESS_THUG), // 7ED 164
    PrintingRecord::alternate(&catalog_nem::SPINELESS_THUG, 1), // 7ED 164★
    PrintingRecord::reprint(&catalog_wth::STRANDS_OF_NIGHT), // 7ED 165
    PrintingRecord::alternate(&catalog_wth::STRANDS_OF_NIGHT, 1), // 7ED 165★
    PrintingRecord::reprint(&catalog_sth::STRONGHOLD_ASSASSIN), // 7ED 166
    PrintingRecord::alternate(&catalog_sth::STRONGHOLD_ASSASSIN, 1), // 7ED 166★
    PrintingRecord::reprint(&catalog_usg::TAINTED_AETHER), // 7ED 167
    PrintingRecord::alternate(&catalog_usg::TAINTED_AETHER, 1), // 7ED 167★
    PrintingRecord::reprint(&catalog_lea::UNHOLY_STRENGTH), // 7ED 168
    PrintingRecord::alternate(&catalog_lea::UNHOLY_STRENGTH, 1), // 7ED 168★
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BONE), // 7ED 169
    PrintingRecord::alternate(&catalog_lea::WALL_OF_BONE, 1), // 7ED 169★
    PrintingRecord::reprint(&catalog_usg::WESTERN_PALADIN), // 7ED 170
    PrintingRecord::alternate(&catalog_usg::WESTERN_PALADIN, 1), // 7ED 170★
    PrintingRecord::reprint(&catalog_usg::YAWGMOTH_S_EDICT), // 7ED 171
    PrintingRecord::alternate(&catalog_usg::YAWGMOTH_S_EDICT, 1), // 7ED 171★
    PrintingRecord::reprint(&catalog_wth::AETHER_FLASH), // 7ED 172
    PrintingRecord::alternate(&catalog_wth::AETHER_FLASH, 1), // 7ED 172★
    PrintingRecord::reprint(&catalog_ice::BALDUVIAN_BARBARIANS), // 7ED 173
    PrintingRecord::alternate(&catalog_ice::BALDUVIAN_BARBARIANS, 1), // 7ED 173★
    PrintingRecord::reprint(&catalog_usg::BEDLAM),       // 7ED 174
    PrintingRecord::alternate(&catalog_usg::BEDLAM, 1),  // 7ED 174★
    PrintingRecord::reprint(&catalog_p02::BLAZE),        // 7ED 175
    PrintingRecord::alternate(&catalog_p02::BLAZE, 1),   // 7ED 175★
    PrintingRecord::reprint(&catalog_uds::BLOODSHOT_CYCLOPS), // 7ED 176
    PrintingRecord::alternate(&catalog_uds::BLOODSHOT_CYCLOPS, 1), // 7ED 176★
    PrintingRecord::reprint(&catalog_tmp::BOIL),         // 7ED 177
    PrintingRecord::alternate(&catalog_tmp::BOIL, 1),    // 7ED 177★
    PrintingRecord::reprint(&catalog_mir::CRIMSON_HELLKITE), // 7ED 178
    PrintingRecord::alternate(&catalog_mir::CRIMSON_HELLKITE, 1), // 7ED 178★
    PrintingRecord::reprint(&catalog_usg::DISORDER),     // 7ED 179
    PrintingRecord::alternate(&catalog_usg::DISORDER, 1), // 7ED 179★
    PrintingRecord::reprint(&catalog_lea::EARTHQUAKE),   // 7ED 180
    PrintingRecord::alternate(&catalog_lea::EARTHQUAKE, 1), // 7ED 180★
    PrintingRecord::reprint(&catalog_m13::FERVOR),       // 7ED 181
    PrintingRecord::alternate(&catalog_m13::FERVOR, 1),  // 7ED 181★
    PrintingRecord::reprint(&catalog_mir::FINAL_FORTUNE), // 7ED 182
    PrintingRecord::alternate(&catalog_mir::FINAL_FORTUNE, 1), // 7ED 182★
    PrintingRecord::reprint(&catalog_lea::FIRE_ELEMENTAL), // 7ED 183
    PrintingRecord::alternate(&catalog_lea::FIRE_ELEMENTAL, 1), // 7ED 183★
    PrintingRecord::reprint(&catalog_ulg::GHITU_FIRE_EATER), // 7ED 184
    PrintingRecord::alternate(&catalog_ulg::GHITU_FIRE_EATER, 1), // 7ED 184★
    PrintingRecord::alternate(&GOBLIN_CHARIOT, 1),       // 7ED 185
    PrintingRecord::reprint(&catalog_drk::GOBLIN_DIGGING_TEAM), // 7ED 186
    PrintingRecord::alternate(&catalog_drk::GOBLIN_DIGGING_TEAM, 1), // 7ED 186★
    PrintingRecord::reprint(&catalog_mir::GOBLIN_ELITE_INFANTRY), // 7ED 187
    PrintingRecord::alternate(&catalog_mir::GOBLIN_ELITE_INFANTRY, 1), // 7ED 187★
    PrintingRecord::reprint(&catalog_uds::GOBLIN_GARDENER), // 7ED 188
    PrintingRecord::alternate(&catalog_uds::GOBLIN_GARDENER, 1), // 7ED 188★
    PrintingRecord::reprint(&catalog_p02::GOBLIN_GLIDER), // 7ED 189
    PrintingRecord::alternate(&catalog_p02::GOBLIN_GLIDER, 1), // 7ED 189★
    PrintingRecord::reprint(&catalog_lea::GOBLIN_KING),  // 7ED 190
    PrintingRecord::alternate(&catalog_lea::GOBLIN_KING, 1), // 7ED 190★
    PrintingRecord::reprint(&catalog_usg::GOBLIN_MATRON), // 7ED 191
    PrintingRecord::alternate(&catalog_usg::GOBLIN_MATRON, 1), // 7ED 191★
    PrintingRecord::reprint(&catalog_p02::GOBLIN_RAIDER), // 7ED 192
    PrintingRecord::alternate(&catalog_p02::GOBLIN_RAIDER, 1), // 7ED 192★
    PrintingRecord::reprint(&catalog_usg::GOBLIN_SPELUNKERS), // 7ED 193
    PrintingRecord::alternate(&catalog_usg::GOBLIN_SPELUNKERS, 1), // 7ED 193★
    PrintingRecord::reprint(&catalog_fem::GOBLIN_WAR_DRUMS), // 7ED 194
    PrintingRecord::alternate(&catalog_fem::GOBLIN_WAR_DRUMS, 1), // 7ED 194★
    PrintingRecord::reprint(&catalog_ulg::GRANITE_GRIP), // 7ED 195
    PrintingRecord::alternate(&catalog_ulg::GRANITE_GRIP, 1), // 7ED 195★
    PrintingRecord::reprint(&catalog_lea::HILL_GIANT),   // 7ED 196
    PrintingRecord::alternate(&catalog_lea::HILL_GIANT, 1), // 7ED 196★
    PrintingRecord::reprint(&catalog_uds::IMPATIENCE),   // 7ED 197
    PrintingRecord::alternate(&catalog_uds::IMPATIENCE, 1), // 7ED 197★
    PrintingRecord::reprint(&catalog_drk::INFERNO),      // 7ED 198
    PrintingRecord::alternate(&catalog_drk::INFERNO, 1), // 7ED 198★
    PrintingRecord::reprint(&catalog_m14::LAVA_AXE),     // 7ED 199
    PrintingRecord::alternate(&catalog_m14::LAVA_AXE, 1), // 7ED 199★
    PrintingRecord::reprint(&catalog_tmp::LIGHTNING_BLAST), // 7ED 200
    PrintingRecord::alternate(&catalog_tmp::LIGHTNING_BLAST, 1), // 7ED 200★
    PrintingRecord::reprint(&catalog_m12::LIGHTNING_ELEMENTAL), // 7ED 201
    PrintingRecord::alternate(&catalog_m12::LIGHTNING_ELEMENTAL, 1), // 7ED 201★
    PrintingRecord::reprint(&catalog_drk::MANA_CLASH),   // 7ED 202
    PrintingRecord::alternate(&catalog_drk::MANA_CLASH, 1), // 7ED 202★
    PrintingRecord::reprint(&catalog_p02::OGRE_TASKMASTER), // 7ED 203
    PrintingRecord::alternate(&catalog_p02::OGRE_TASKMASTER, 1), // 7ED 203★
    PrintingRecord::reprint(&catalog_usg::OKK),          // 7ED 204
    PrintingRecord::alternate(&catalog_usg::OKK, 1),     // 7ED 204★
    PrintingRecord::reprint(&catalog_lea::ORCISH_ARTILLERY), // 7ED 205
    PrintingRecord::alternate(&catalog_lea::ORCISH_ARTILLERY, 1), // 7ED 205★
    PrintingRecord::reprint(&catalog_lea::ORCISH_ORIFLAMME), // 7ED 206
    PrintingRecord::alternate(&catalog_lea::ORCISH_ORIFLAMME, 1), // 7ED 206★
    PrintingRecord::reprint(&catalog_all::PILLAGE),      // 7ED 207
    PrintingRecord::alternate(&catalog_all::PILLAGE, 1), // 7ED 207★
    PrintingRecord::reprint(&catalog_ulg::PYGMY_PYROSAUR), // 7ED 208
    PrintingRecord::alternate(&catalog_ulg::PYGMY_PYROSAUR, 1), // 7ED 208★
    PrintingRecord::reprint(&catalog_ice::PYROCLASM),    // 7ED 209
    PrintingRecord::alternate(&catalog_ice::PYROCLASM, 1), // 7ED 209★
    PrintingRecord::reprint(&catalog_leg::PYROTECHNICS), // 7ED 210
    PrintingRecord::alternate(&catalog_leg::PYROTECHNICS, 1), // 7ED 210★
    PrintingRecord::reprint(&catalog_exo::RAGING_GOBLIN), // 7ED 211
    PrintingRecord::alternate(&catalog_exo::RAGING_GOBLIN, 1), // 7ED 211★
    PrintingRecord::reprint(&catalog_mir::RECKLESS_EMBERMAGE), // 7ED 212
    PrintingRecord::alternate(&catalog_mir::RECKLESS_EMBERMAGE, 1), // 7ED 212★
    PrintingRecord::reprint(&catalog_usg::REFLEXES),     // 7ED 213
    PrintingRecord::alternate(&catalog_usg::REFLEXES, 1), // 7ED 213★
    PrintingRecord::reprint(&catalog_vis::RELENTLESS_ASSAULT), // 7ED 214
    PrintingRecord::alternate(&catalog_vis::RELENTLESS_ASSAULT, 1), // 7ED 214★
    PrintingRecord::reprint(&catalog_ice::SABRETOOTH_TIGER), // 7ED 215
    PrintingRecord::alternate(&catalog_ice::SABRETOOTH_TIGER, 1), // 7ED 215★
    PrintingRecord::reprint(&catalog_exo::SEISMIC_ASSAULT), // 7ED 216
    PrintingRecord::alternate(&catalog_exo::SEISMIC_ASSAULT, 1), // 7ED 216★
    PrintingRecord::reprint(&catalog_lea::SHATTER),      // 7ED 217
    PrintingRecord::alternate(&catalog_lea::SHATTER, 1), // 7ED 217★
    PrintingRecord::reprint(&catalog_lea::SHIVAN_DRAGON), // 7ED 218
    PrintingRecord::alternate(&catalog_lea::SHIVAN_DRAGON, 1), // 7ED 218★
    PrintingRecord::reprint(&catalog_m14::SHOCK),        // 7ED 219
    PrintingRecord::alternate(&catalog_m14::SHOCK, 1),   // 7ED 219★
    PrintingRecord::reprint(&catalog_mir::SPITTING_EARTH), // 7ED 220
    PrintingRecord::alternate(&catalog_mir::SPITTING_EARTH, 1), // 7ED 220★
    PrintingRecord::reprint(&catalog_lea::STONE_RAIN),   // 7ED 221
    PrintingRecord::alternate(&catalog_lea::STONE_RAIN, 1), // 7ED 221★
    PrintingRecord::reprint(&catalog_all::STORM_SHAMAN), // 7ED 222
    PrintingRecord::alternate(&catalog_all::STORM_SHAMAN, 1), // 7ED 222★
    PrintingRecord::reprint(&catalog_tmp::SUDDEN_IMPACT), // 7ED 223
    PrintingRecord::alternate(&catalog_tmp::SUDDEN_IMPACT, 1), // 7ED 223★
    PrintingRecord::alternate(&TRAINED_ORGG, 1),         // 7ED 224★
    PrintingRecord::reprint(&catalog_vis::TREMOR),       // 7ED 225
    PrintingRecord::alternate(&catalog_vis::TREMOR, 1),  // 7ED 225★
    PrintingRecord::reprint(&portal_second_age::VOLCANIC_HAMMER), // 7ED 226
    PrintingRecord::alternate(&catalog_p02::VOLCANIC_HAMMER, 1), // 7ED 226★
    PrintingRecord::reprint(&catalog_lea::WALL_OF_FIRE), // 7ED 227
    PrintingRecord::alternate(&catalog_lea::WALL_OF_FIRE, 1), // 7ED 227★
    PrintingRecord::reprint(&catalog_p02::WILDFIRE),     // 7ED 228
    PrintingRecord::alternate(&catalog_p02::WILDFIRE, 1), // 7ED 228★
    PrintingRecord::reprint(&catalog_usg::ANACONDA),     // 7ED 229
    PrintingRecord::alternate(&catalog_usg::ANACONDA, 1), // 7ED 229★
    PrintingRecord::reprint(&catalog_uds::ANCIENT_SILVERBACK), // 7ED 230
    PrintingRecord::alternate(&catalog_uds::ANCIENT_SILVERBACK, 1), // 7ED 230★
    PrintingRecord::reprint(&catalog_lea::BIRDS_OF_PARADISE), // 7ED 231
    PrintingRecord::alternate(&catalog_lea::BIRDS_OF_PARADISE, 1), // 7ED 231★
    PrintingRecord::reprint(&catalog_usg::BLANCHWOOD_ARMOR), // 7ED 232
    PrintingRecord::alternate(&catalog_usg::BLANCHWOOD_ARMOR, 1), // 7ED 232★
    PrintingRecord::reprint(&catalog_usg::BULL_HIPPO),   // 7ED 233
    PrintingRecord::alternate(&catalog_usg::BULL_HIPPO, 1), // 7ED 233★
    PrintingRecord::reprint(&catalog_tmp::CANOPY_SPIDER), // 7ED 234
    PrintingRecord::alternate(&catalog_tmp::CANOPY_SPIDER, 1), // 7ED 234★
    PrintingRecord::reprint(&catalog_uds::COMPOST),      // 7ED 235
    PrintingRecord::alternate(&catalog_uds::COMPOST, 1), // 7ED 235★
    PrintingRecord::reprint(&catalog_vis::CREEPING_MOLD), // 7ED 236
    PrintingRecord::alternate(&catalog_vis::CREEPING_MOLD, 1), // 7ED 236★
    PrintingRecord::reprint(&catalog_mir::EARLY_HARVEST), // 7ED 237
    PrintingRecord::alternate(&catalog_mir::EARLY_HARVEST, 1), // 7ED 237★
    PrintingRecord::reprint(&catalog_ice::ELDER_DRUID),  // 7ED 238
    PrintingRecord::alternate(&catalog_ice::ELDER_DRUID, 1), // 7ED 238★
    PrintingRecord::reprint(&catalog_lea::ELVISH_ARCHERS), // 7ED 239
    PrintingRecord::alternate(&catalog_lea::ELVISH_ARCHERS, 1), // 7ED 239★
    PrintingRecord::reprint(&catalog_inv::ELVISH_CHAMPION), // 7ED 240
    PrintingRecord::alternate(&catalog_inv::ELVISH_CHAMPION, 1), // 7ED 240★
    PrintingRecord::reprint(&catalog_usg::ELVISH_LYRIST), // 7ED 241
    PrintingRecord::alternate(&catalog_usg::ELVISH_LYRIST, 1), // 7ED 241★
    PrintingRecord::reprint(&catalog_uds::ELVISH_PIPER), // 7ED 242
    PrintingRecord::alternate(&catalog_uds::ELVISH_PIPER, 1), // 7ED 242★
    PrintingRecord::reprint(&catalog_wth::FAMILIAR_GROUND), // 7ED 243
    PrintingRecord::alternate(&catalog_wth::FAMILIAR_GROUND, 1), // 7ED 243★
    PrintingRecord::reprint(&catalog_mir::FEMEREF_ARCHERS), // 7ED 244
    PrintingRecord::alternate(&catalog_mir::FEMEREF_ARCHERS, 1), // 7ED 244★
    PrintingRecord::reprint(&catalog_lea::FOG),          // 7ED 245
    PrintingRecord::alternate(&catalog_lea::FOG, 1),     // 7ED 245★
    PrintingRecord::reprint(&catalog_ice::FYNDHORN_ELDER), // 7ED 246
    PrintingRecord::alternate(&catalog_ice::FYNDHORN_ELDER, 1), // 7ED 246★
    PrintingRecord::reprint(&catalog_ulg::GANG_OF_ELK),  // 7ED 247
    PrintingRecord::alternate(&catalog_ulg::GANG_OF_ELK, 1), // 7ED 247★
    PrintingRecord::reprint(&catalog_lea::GIANT_GROWTH), // 7ED 248
    PrintingRecord::alternate(&catalog_lea::GIANT_GROWTH, 1), // 7ED 248★
    PrintingRecord::reprint(&catalog_lea::GIANT_SPIDER), // 7ED 249
    PrintingRecord::alternate(&catalog_lea::GIANT_SPIDER, 1), // 7ED 249★
    PrintingRecord::reprint(&catalog_all::GORILLA_CHIEFTAIN), // 7ED 250
    PrintingRecord::alternate(&catalog_all::GORILLA_CHIEFTAIN, 1), // 7ED 250★
    PrintingRecord::reprint(&catalog_lea::GRIZZLY_BEARS), // 7ED 251
    PrintingRecord::alternate(&catalog_lea::GRIZZLY_BEARS, 1), // 7ED 251★
    PrintingRecord::reprint(&catalog_lea::HURRICANE),    // 7ED 252
    PrintingRecord::alternate(&catalog_lea::HURRICANE, 1), // 7ED 252★
    PrintingRecord::reprint(&catalog_lea::LLANOWAR_ELVES), // 7ED 253
    PrintingRecord::alternate(&catalog_lea::LLANOWAR_ELVES, 1), // 7ED 253★
    PrintingRecord::reprint(&catalog_p02::LONE_WOLF),    // 7ED 254
    PrintingRecord::alternate(&catalog_p02::LONE_WOLF, 1), // 7ED 254★
    PrintingRecord::reprint(&catalog_lea::LURE),         // 7ED 255
    PrintingRecord::alternate(&catalog_lea::LURE, 1),    // 7ED 255★
    PrintingRecord::reprint(&catalog_mir::MARO),         // 7ED 256
    PrintingRecord::alternate(&catalog_mir::MARO, 1),    // 7ED 256★
    PrintingRecord::reprint(&catalog_ulg::MIGHT_OF_OAKS), // 7ED 257
    PrintingRecord::alternate(&catalog_ulg::MIGHT_OF_OAKS, 1), // 7ED 257★
    PrintingRecord::reprint(&catalog_p02::MONSTROUS_GROWTH), // 7ED 258
    PrintingRecord::alternate(&catalog_p02::MONSTROUS_GROWTH, 1), // 7ED 258★
    PrintingRecord::reprint(&catalog_wth::NATURE_S_RESURGENCE), // 7ED 259
    PrintingRecord::alternate(&catalog_wth::NATURE_S_RESURGENCE, 1), // 7ED 259★
    PrintingRecord::reprint(&catalog_tmp::NATURE_S_REVOLT), // 7ED 260
    PrintingRecord::alternate(&catalog_tmp::NATURE_S_REVOLT, 1), // 7ED 260★
    PrintingRecord::alternate(&PRIDE_OF_LIONS, 1),       // 7ED 261
    PrintingRecord::reprint(&catalog_m12::RAMPANT_GROWTH), // 7ED 262
    PrintingRecord::alternate(&catalog_m12::RAMPANT_GROWTH, 1), // 7ED 262★
    PrintingRecord::reprint(&catalog_m12::RECLAIM),      // 7ED 263
    PrintingRecord::alternate(&catalog_m12::RECLAIM, 1), // 7ED 263★
    PrintingRecord::reprint(&catalog_wth::REDWOOD_TREEFOLK), // 7ED 264
    PrintingRecord::alternate(&catalog_wth::REDWOOD_TREEFOLK, 1), // 7ED 264★
    PrintingRecord::reprint(&catalog_lea::REGENERATION), // 7ED 265
    PrintingRecord::alternate(&catalog_lea::REGENERATION, 1), // 7ED 265★
    PrintingRecord::reprint(&catalog_vis::ROWEN),        // 7ED 266
    PrintingRecord::alternate(&catalog_vis::ROWEN, 1),   // 7ED 266★
    PrintingRecord::reprint(&catalog_drk::SCAVENGER_FOLK), // 7ED 267
    PrintingRecord::alternate(&catalog_drk::SCAVENGER_FOLK, 1), // 7ED 267★
    PrintingRecord::reprint(&catalog_tmp::SEEKER_OF_SKYBREAK), // 7ED 268
    PrintingRecord::alternate(&catalog_tmp::SEEKER_OF_SKYBREAK, 1), // 7ED 268★
    PrintingRecord::reprint(&catalog_lea::SHANODIN_DRYADS), // 7ED 269
    PrintingRecord::alternate(&catalog_lea::SHANODIN_DRYADS, 1), // 7ED 269★
    PrintingRecord::reprint(&catalog_sth::SPINED_WURM),  // 7ED 270
    PrintingRecord::alternate(&catalog_sth::SPINED_WURM, 1), // 7ED 270★
    PrintingRecord::reprint(&catalog_mmq::SQUALL),       // 7ED 271
    PrintingRecord::alternate(&catalog_mmq::SQUALL, 1),  // 7ED 271★
    PrintingRecord::reprint(&catalog_lea::STREAM_OF_LIFE), // 7ED 272
    PrintingRecord::alternate(&catalog_lea::STREAM_OF_LIFE, 1), // 7ED 272★
    PrintingRecord::reprint(&catalog_uds::THORN_ELEMENTAL), // 7ED 273
    PrintingRecord::alternate(&catalog_uds::THORN_ELEMENTAL, 1), // 7ED 273★
    PrintingRecord::reprint(&catalog_ice::THOUGHTLEECH), // 7ED 274
    PrintingRecord::alternate(&catalog_ice::THOUGHTLEECH, 1), // 7ED 274★
    PrintingRecord::reprint(&catalog_tmp::TRAINED_ARMODON), // 7ED 275
    PrintingRecord::alternate(&catalog_tmp::TRAINED_ARMODON, 1), // 7ED 275★
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),  // 7ED 276
    PrintingRecord::alternate(&catalog_lea::TRANQUILITY, 1), // 7ED 276★
    PrintingRecord::reprint(&catalog_usg::TREEFOLK_SEEDLINGS), // 7ED 277
    PrintingRecord::alternate(&catalog_usg::TREEFOLK_SEEDLINGS, 1), // 7ED 277★
    PrintingRecord::reprint(&catalog_mir::UKTABI_WILDCATS), // 7ED 278
    PrintingRecord::alternate(&catalog_mir::UKTABI_WILDCATS, 1), // 7ED 278★
    PrintingRecord::reprint(&catalog_leg::UNTAMED_WILDS), // 7ED 279
    PrintingRecord::alternate(&catalog_leg::UNTAMED_WILDS, 1), // 7ED 279★
    PrintingRecord::reprint(&catalog_lea::VERDURAN_ENCHANTRESS), // 7ED 280
    PrintingRecord::alternate(&catalog_lea::VERDURAN_ENCHANTRESS, 1), // 7ED 280★
    PrintingRecord::reprint(&catalog_usg::VERNAL_BLOOM), // 7ED 281
    PrintingRecord::alternate(&catalog_usg::VERNAL_BLOOM, 1), // 7ED 281★
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),  // 7ED 282
    PrintingRecord::alternate(&catalog_lea::WILD_GROWTH, 1), // 7ED 282★
    PrintingRecord::reprint(&catalog_ulg::WING_SNARE),   // 7ED 283
    PrintingRecord::alternate(&catalog_ulg::WING_SNARE, 1), // 7ED 283★
    PrintingRecord::reprint(&catalog_exo::WOOD_ELVES),   // 7ED 284
    PrintingRecord::alternate(&catalog_exo::WOOD_ELVES, 1), // 7ED 284★
    PrintingRecord::reprint(&catalog_uds::YAVIMAYA_ENCHANTRESS), // 7ED 285
    PrintingRecord::alternate(&catalog_uds::YAVIMAYA_ENCHANTRESS, 1), // 7ED 285★
    PrintingRecord::reprint(&catalog_arn::ALADDINS_RING), // 7ED 286
    PrintingRecord::alternate(&catalog_arn::ALADDINS_RING, 1), // 7ED 286★
    PrintingRecord::reprint(&catalog_ulg::BEAST_OF_BURDEN), // 7ED 287
    PrintingRecord::alternate(&catalog_ulg::BEAST_OF_BURDEN, 1), // 7ED 287★
    PrintingRecord::reprint(&catalog_uds::CALTROPS),     // 7ED 288
    PrintingRecord::alternate(&catalog_uds::CALTROPS, 1), // 7ED 288★
    PrintingRecord::reprint(&catalog_mir::CHARCOAL_DIAMOND), // 7ED 289
    PrintingRecord::alternate(&catalog_mir::CHARCOAL_DIAMOND, 1), // 7ED 289s
    PrintingRecord::alternate(&catalog_mir::CHARCOAL_DIAMOND, 2), // 7ED 289★
    PrintingRecord::alternate(&catalog_mir::CHARCOAL_DIAMOND, 3), // 7ED 289★s
    PrintingRecord::reprint(&catalog_exo::COAT_OF_ARMS), // 7ED 290
    PrintingRecord::alternate(&catalog_exo::COAT_OF_ARMS, 1), // 7ED 290★
    PrintingRecord::reprint(&catalog_lea::CRYSTAL_ROD),  // 7ED 291
    PrintingRecord::alternate(&catalog_lea::CRYSTAL_ROD, 1), // 7ED 291★
    PrintingRecord::reprint(&catalog_lea::DINGUS_EGG),   // 7ED 292
    PrintingRecord::alternate(&catalog_lea::DINGUS_EGG, 1), // 7ED 292★
    PrintingRecord::reprint(&catalog_lea::DISRUPTING_SCEPTER), // 7ED 293
    PrintingRecord::alternate(&catalog_lea::DISRUPTING_SCEPTER, 1), // 7ED 293★
    PrintingRecord::reprint(&catalog_sth::ENSNARING_BRIDGE), // 7ED 294
    PrintingRecord::alternate(&catalog_sth::ENSNARING_BRIDGE, 1), // 7ED 294★
    PrintingRecord::reprint(&catalog_hml::FEROZ_S_BAN),  // 7ED 295
    PrintingRecord::alternate(&catalog_hml::FEROZ_S_BAN, 1), // 7ED 295★
    PrintingRecord::reprint(&catalog_mir::FIRE_DIAMOND), // 7ED 296
    PrintingRecord::alternate(&catalog_mir::FIRE_DIAMOND, 1), // 7ED 296★
    PrintingRecord::reprint(&catalog_arn::FLYING_CARPET), // 7ED 297
    PrintingRecord::alternate(&catalog_arn::FLYING_CARPET, 1), // 7ED 297★
    PrintingRecord::reprint(&catalog_usg::GRAFTED_SKULLCAP), // 7ED 298
    PrintingRecord::alternate(&catalog_usg::GRAFTED_SKULLCAP, 1), // 7ED 298★
    PrintingRecord::reprint(&catalog_atq::GRAPESHOT_CATAPULT), // 7ED 299
    PrintingRecord::alternate(&catalog_atq::GRAPESHOT_CATAPULT, 1), // 7ED 299★
    PrintingRecord::reprint(&catalog_lea::HOWLING_MINE), // 7ED 300
    PrintingRecord::alternate(&catalog_lea::HOWLING_MINE, 1), // 7ED 300★
    PrintingRecord::reprint(&catalog_lea::IRON_STAR),    // 7ED 301
    PrintingRecord::alternate(&catalog_lea::IRON_STAR, 1), // 7ED 301★
    PrintingRecord::reprint(&catalog_lea::IVORY_CUP),    // 7ED 302
    PrintingRecord::alternate(&catalog_lea::IVORY_CUP, 1), // 7ED 302★
    PrintingRecord::reprint(&catalog_atq::JALUM_TOME),   // 7ED 303
    PrintingRecord::alternate(&catalog_atq::JALUM_TOME, 1), // 7ED 303★
    PrintingRecord::reprint(&catalog_arn::JANDORS_SADDLEBAGS), // 7ED 304
    PrintingRecord::alternate(&catalog_arn::JANDORS_SADDLEBAGS, 1), // 7ED 304★
    PrintingRecord::reprint(&catalog_lea::JAYEMDAE_TOME), // 7ED 305
    PrintingRecord::alternate(&catalog_lea::JAYEMDAE_TOME, 1), // 7ED 305★
    PrintingRecord::reprint(&catalog_mir::MARBLE_DIAMOND), // 7ED 306
    PrintingRecord::alternate(&catalog_mir::MARBLE_DIAMOND, 1), // 7ED 306★
    PrintingRecord::reprint(&catalog_lea::MEEKSTONE),    // 7ED 307
    PrintingRecord::alternate(&catalog_lea::MEEKSTONE, 1), // 7ED 307★
    PrintingRecord::reprint(&catalog_atq::MILLSTONE),    // 7ED 308
    PrintingRecord::alternate(&catalog_atq::MILLSTONE, 1), // 7ED 308★
    PrintingRecord::reprint(&catalog_mir::MOSS_DIAMOND), // 7ED 309
    PrintingRecord::alternate(&catalog_mir::MOSS_DIAMOND, 1), // 7ED 309★
    PrintingRecord::reprint(&catalog_mir::PATAGIA_GOLEM), // 7ED 310
    PrintingRecord::alternate(&catalog_mir::PATAGIA_GOLEM, 1), // 7ED 310★
    PrintingRecord::reprint(&catalog_usg::PHYREXIAN_COLOSSUS), // 7ED 311
    PrintingRecord::alternate(&catalog_usg::PHYREXIAN_COLOSSUS, 1), // 7ED 311★
    PrintingRecord::reprint(&catalog_m13::PHYREXIAN_HULK), // 7ED 312
    PrintingRecord::alternate(&catalog_m13::PHYREXIAN_HULK, 1), // 7ED 312★
    PrintingRecord::reprint(&catalog_ice::PIT_TRAP),     // 7ED 313
    PrintingRecord::alternate(&catalog_ice::PIT_TRAP, 1), // 7ED 313★
    PrintingRecord::reprint(&catalog_lea::ROD_OF_RUIN),  // 7ED 314
    PrintingRecord::alternate(&catalog_lea::ROD_OF_RUIN, 1), // 7ED 314★
    PrintingRecord::reprint(&catalog_vis::SISAY_S_RING), // 7ED 315
    PrintingRecord::alternate(&catalog_vis::SISAY_S_RING, 1), // 7ED 315★
    PrintingRecord::reprint(&catalog_mir::SKY_DIAMOND),  // 7ED 316
    PrintingRecord::alternate(&catalog_mir::SKY_DIAMOND, 1), // 7ED 316★
    PrintingRecord::reprint(&catalog_lea::SOUL_NET),     // 7ED 317
    PrintingRecord::alternate(&catalog_lea::SOUL_NET, 1), // 7ED 317★
    PrintingRecord::reprint(&catalog_exo::SPELLBOOK),    // 7ED 318
    PrintingRecord::alternate(&catalog_exo::SPELLBOOK, 1), // 7ED 318★
    PrintingRecord::reprint(&catalog_tmp::STATIC_ORB),   // 7ED 319
    PrintingRecord::alternate(&catalog_tmp::STATIC_ORB, 1), // 7ED 319★
    PrintingRecord::reprint(&catalog_all::STORM_CAULDRON), // 7ED 320
    PrintingRecord::alternate(&catalog_all::STORM_CAULDRON, 1), // 7ED 320★
    PrintingRecord::reprint(&catalog_vis::TEFERI_S_PUZZLE_BOX), // 7ED 321
    PrintingRecord::alternate(&catalog_vis::TEFERI_S_PUZZLE_BOX, 1), // 7ED 321★
    PrintingRecord::reprint(&catalog_lea::THRONE_OF_BONE), // 7ED 322
    PrintingRecord::alternate(&catalog_lea::THRONE_OF_BONE, 1), // 7ED 322★
    PrintingRecord::reprint(&catalog_atq::WALL_OF_SPEARS), // 7ED 323
    PrintingRecord::alternate(&catalog_atq::WALL_OF_SPEARS, 1), // 7ED 323★
    PrintingRecord::reprint(&catalog_lea::WOODEN_SPHERE), // 7ED 324
    PrintingRecord::alternate(&catalog_lea::WOODEN_SPHERE, 1), // 7ED 324★
    PrintingRecord::reprint(&catalog_ice::ADARKAR_WASTES), // 7ED 325
    PrintingRecord::alternate(&catalog_ice::ADARKAR_WASTES, 1), // 7ED 325★
    PrintingRecord::reprint(&catalog_ice::BRUSHLAND),    // 7ED 326
    PrintingRecord::alternate(&catalog_ice::BRUSHLAND, 1), // 7ED 326★
    PrintingRecord::reprint(&arabian_nights::CITY_OF_BRASS), // 7ED 327
    PrintingRecord::alternate(&catalog_arn::CITY_OF_BRASS, 1), // 7ED 327★
    PrintingRecord::reprint(&catalog_lea::FOREST),       // 7ED 328
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),  // 7ED 328★
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),  // 7ED 329
    PrintingRecord::alternate(&catalog_lea::FOREST, 3),  // 7ED 329★
    PrintingRecord::alternate(&catalog_lea::FOREST, 4),  // 7ED 330
    PrintingRecord::alternate(&catalog_lea::FOREST, 5),  // 7ED 330★
    PrintingRecord::alternate(&catalog_lea::FOREST, 6),  // 7ED 331
    PrintingRecord::alternate(&catalog_lea::FOREST, 7),  // 7ED 331★
    PrintingRecord::reprint(&catalog_lea::ISLAND),       // 7ED 332
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),  // 7ED 332★
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),  // 7ED 333
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3),  // 7ED 333★
    PrintingRecord::alternate(&catalog_lea::ISLAND, 4),  // 7ED 334
    PrintingRecord::alternate(&catalog_lea::ISLAND, 5),  // 7ED 334★
    PrintingRecord::alternate(&catalog_lea::ISLAND, 6),  // 7ED 335
    PrintingRecord::alternate(&catalog_lea::ISLAND, 7),  // 7ED 335★
    PrintingRecord::reprint(&catalog_ice::KARPLUSAN_FOREST), // 7ED 336
    PrintingRecord::alternate(&catalog_ice::KARPLUSAN_FOREST, 1), // 7ED 336★
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),     // 7ED 337
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1), // 7ED 337★
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2), // 7ED 338
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3), // 7ED 338★
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 4), // 7ED 339
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 5), // 7ED 339★
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 6), // 7ED 340
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 7), // 7ED 340★
    PrintingRecord::reprint(&catalog_lea::PLAINS),       // 7ED 341
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),  // 7ED 341★
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),  // 7ED 342
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3),  // 7ED 342★
    PrintingRecord::alternate(&catalog_lea::PLAINS, 4),  // 7ED 343
    PrintingRecord::alternate(&catalog_lea::PLAINS, 5),  // 7ED 343★
    PrintingRecord::alternate(&catalog_lea::PLAINS, 6),  // 7ED 344
    PrintingRecord::alternate(&catalog_lea::PLAINS, 7),  // 7ED 344★
    PrintingRecord::reprint(&catalog_ice::SULFUROUS_SPRINGS), // 7ED 345
    PrintingRecord::alternate(&catalog_ice::SULFUROUS_SPRINGS, 1), // 7ED 345★
    PrintingRecord::reprint(&catalog_lea::SWAMP),        // 7ED 346
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),   // 7ED 346★
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),   // 7ED 347
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3),   // 7ED 347★
    PrintingRecord::alternate(&catalog_lea::SWAMP, 4),   // 7ED 348
    PrintingRecord::alternate(&catalog_lea::SWAMP, 5),   // 7ED 348★
    PrintingRecord::alternate(&catalog_lea::SWAMP, 6),   // 7ED 349
    PrintingRecord::alternate(&catalog_lea::SWAMP, 7),   // 7ED 349★
    PrintingRecord::reprint(&catalog_ice::UNDERGROUND_RIVER), // 7ED 350
    PrintingRecord::alternate(&catalog_ice::UNDERGROUND_RIVER, 1), // 7ED 350★
];
