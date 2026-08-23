//! Limited Edition Beta card definitions and printings.

use super::{CardRecord, PrintingRecord, alpha};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::{
    AbilityCostDef, CardArt, CardRules, CardSet, ManaColor, ObjectPredicateDef, abilities,
};
use crate::mana_cost;

// LEB 1 — Animate Wall (reprint)

// LEB 2 — Armageddon (reprint)

// LEB 3 — Balance (reprint)

// LEB 4 — Benalish Hero (reprint)

// LEB 5 — Black Ward (reprint)

// LEB 6 — Blaze of Glory (reprint)

// LEB 7 — Blessing (reprint)

// LEB 8 — Blue Ward (reprint)

// LEB 9 — Castle (reprint)

// LEB 10 — Circle of Protection: Black
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_BLACK: CardRecord = CardRecord::new_with_legacy_id(
    1450,
    "Circle of Protection: Black",
    CardArt::new("fa47b4cd-8da4-4544-b011-ba92b7009203", "Jesper Myrfors"),
    CardSet::Beta,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}: The next time a black source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Color(ManaColor::Black),
        ),
    ),
);

// LEB 11 — Circle of Protection: Blue (reprint)

// LEB 12 — Circle of Protection: Green (reprint)

// LEB 13 — Circle of Protection: Red (reprint)

// LEB 14 — Circle of Protection: White (reprint)

// LEB 15 — Consecrate Land (reprint)

// LEB 16 — Conversion (reprint)

// LEB 17 — Crusade (reprint)

// LEB 18 — Death Ward (reprint)

// LEB 19 — Disenchant (reprint)

// LEB 20 — Farmstead (reprint)

// LEB 21 — Green Ward (reprint)

// LEB 22 — Guardian Angel (reprint)

// LEB 23 — Healing Salve (reprint)

// LEB 24 — Holy Armor (reprint)

// LEB 25 — Holy Strength (reprint)

// LEB 26 — Island Sanctuary (reprint)

// LEB 27 — Karma (reprint)

// LEB 28 — Lance (reprint)

// LEB 29 — Mesa Pegasus (reprint)

// LEB 30 — Northern Paladin (reprint)

// LEB 31 — Pearled Unicorn (reprint)

// LEB 32 — Personal Incarnation (reprint)

// LEB 33 — Purelace (reprint)

// LEB 34 — Red Ward (reprint)

// LEB 35 — Resurrection (reprint)

// LEB 36 — Reverse Damage (reprint)

// LEB 37 — Righteousness (reprint)

// LEB 38 — Samite Healer (reprint)

// LEB 39 — Savannah Lions (reprint)

// LEB 40 — Serra Angel (reprint)

// LEB 41 — Swords to Plowshares (reprint)

// LEB 42 — Veteran Bodyguard (reprint)

// LEB 43 — Wall of Swords (reprint)

// LEB 44 — White Knight (reprint)

// LEB 45 — White Ward (reprint)

// LEB 46 — Wrath of God (reprint)

// LEB 47 — Air Elemental (reprint)

// LEB 48 — Ancestral Recall (reprint)

// LEB 49 — Animate Artifact (reprint)

// LEB 50 — Blue Elemental Blast (reprint)

// LEB 51 — Braingeyser (reprint)

// LEB 52 — Clone (reprint)

// LEB 53 — Control Magic (reprint)

// LEB 54 — Copy Artifact (reprint)

// LEB 55 — Counterspell (reprint)

// LEB 56 — Creature Bond (reprint)

// LEB 57 — Drain Power (reprint)

// LEB 58 — Feedback (reprint)

// LEB 59 — Flight (reprint)

// LEB 60 — Invisibility (reprint)

// LEB 61 — Jump (reprint)

// LEB 62 — Lifetap (reprint)

// LEB 63 — Lord of Atlantis (reprint)

// LEB 64 — Magical Hack (reprint)

// LEB 65 — Mahamoti Djinn (reprint)

// LEB 66 — Mana Short (reprint)

// LEB 67 — Merfolk of the Pearl Trident (reprint)

// LEB 68 — Phantasmal Forces (reprint)

// LEB 69 — Phantasmal Terrain (reprint)

// LEB 70 — Phantom Monster (reprint)

// LEB 71 — Pirate Ship (reprint)

// LEB 72 — Power Leak (reprint)

// LEB 73 — Power Sink (reprint)

// LEB 74 — Prodigal Sorcerer (reprint)

// LEB 75 — Psionic Blast (reprint)

// LEB 76 — Psychic Venom (reprint)

// LEB 77 — Sea Serpent (reprint)

// LEB 78 — Siren's Call (reprint)

// LEB 79 — Sleight of Mind (reprint)

// LEB 80 — Spell Blast (reprint)

// LEB 81 — Stasis (reprint)

// LEB 82 — Steal Artifact (reprint)

// LEB 83 — Thoughtlace (reprint)

// LEB 84 — Time Walk (reprint)

// LEB 85 — Timetwister (reprint)

// LEB 86 — Twiddle (reprint)

// LEB 87 — Unsummon (reprint)

// LEB 88 — Vesuvan Doppelganger (reprint)

// LEB 89 — Volcanic Eruption (reprint)

// LEB 90 — Wall of Air (reprint)

// LEB 91 — Wall of Water (reprint)

// LEB 92 — Water Elemental (reprint)

// LEB 93 — Animate Dead (reprint)

// LEB 94 — Bad Moon (reprint)

// LEB 95 — Black Knight (reprint)

// LEB 96 — Bog Wraith (reprint)

// LEB 97 — Contract from Below (reprint)

// LEB 98 — Cursed Land (reprint)

// LEB 99 — Dark Ritual (reprint)

// LEB 100 — Darkpact (reprint)

// LEB 101 — Deathgrip (reprint)

// LEB 102 — Deathlace (reprint)

// LEB 103 — Demonic Attorney (reprint)

// LEB 104 — Demonic Hordes (reprint)

// LEB 105 — Demonic Tutor (reprint)

// LEB 106 — Drain Life (reprint)

// LEB 107 — Drudge Skeletons (reprint)

// LEB 108 — Evil Presence (reprint)

// LEB 109 — Fear (reprint)

// LEB 110 — Frozen Shade (reprint)

// LEB 111 — Gloom (reprint)

// LEB 112 — Howl from Beyond (reprint)

// LEB 113 — Hypnotic Specter (reprint)

// LEB 114 — Lich (reprint)

// LEB 115 — Lord of the Pit (reprint)

// LEB 116 — Mind Twist (reprint)

// LEB 117 — Nether Shadow (reprint)

// LEB 118 — Nettling Imp (reprint)

// LEB 119 — Nightmare (reprint)

// LEB 120 — Paralyze (reprint)

// LEB 121 — Pestilence (reprint)

// LEB 122 — Plague Rats (reprint)

// LEB 123 — Raise Dead (reprint)

// LEB 124 — Royal Assassin (reprint)

// LEB 125 — Sacrifice (reprint)

// LEB 126 — Scathe Zombies (reprint)

// LEB 127 — Scavenging Ghoul (reprint)

// LEB 128 — Sengir Vampire (reprint)

// LEB 129 — Simulacrum (reprint)

// LEB 130 — Sinkhole (reprint)

// LEB 131 — Terror (reprint)

// LEB 132 — Unholy Strength (reprint)

// LEB 133 — Wall of Bone (reprint)

// LEB 134 — Warp Artifact (reprint)

// LEB 135 — Weakness (reprint)

// LEB 136 — Will-o'-the-Wisp (reprint)

// LEB 137 — Word of Command (reprint)

// LEB 138 — Zombie Master (reprint)

// LEB 139 — Burrowing (reprint)

// LEB 140 — Chaoslace (reprint)

// LEB 141 — Disintegrate (reprint)

// LEB 142 — Dragon Whelp (reprint)

// LEB 143 — Dwarven Demolition Team (reprint)

// LEB 144 — Dwarven Warriors (reprint)

// LEB 145 — Earth Elemental (reprint)

// LEB 146 — Earthbind (reprint)

// LEB 147 — Earthquake (reprint)

// LEB 148 — False Orders (reprint)

// LEB 149 — Fire Elemental (reprint)

// LEB 150 — Fireball (reprint)

// LEB 151 — Firebreathing (reprint)

// LEB 152 — Flashfires (reprint)

// LEB 153 — Fork (reprint)

// LEB 154 — Goblin Balloon Brigade (reprint)

// LEB 155 — Goblin King (reprint)

// LEB 156 — Granite Gargoyle (reprint)

// LEB 157 — Gray Ogre (reprint)

// LEB 158 — Hill Giant (reprint)

// LEB 159 — Hurloon Minotaur (reprint)

// LEB 160 — Ironclaw Orcs (reprint)

// LEB 161 — Keldon Warlord (reprint)

// LEB 162 — Lightning Bolt (reprint)

// LEB 163 — Mana Flare (reprint)

// LEB 164 — Manabarbs (reprint)

// LEB 165 — Mons's Goblin Raiders (reprint)

// LEB 166 — Orcish Artillery (reprint)

// LEB 167 — Orcish Oriflamme (reprint)

// LEB 168 — Power Surge (reprint)

// LEB 169 — Raging River (reprint)

// LEB 170 — Red Elemental Blast (reprint)

// LEB 171 — Roc of Kher Ridges (reprint)

// LEB 172 — Rock Hydra (reprint)

// LEB 173 — Sedge Troll (reprint)

// LEB 174 — Shatter (reprint)

// LEB 175 — Shivan Dragon (reprint)

// LEB 176 — Smoke (reprint)

// LEB 177 — Stone Giant (reprint)

// LEB 178 — Stone Rain (reprint)

// LEB 179 — Tunnel (reprint)

// LEB 180 — Two-Headed Giant of Foriys (reprint)

// LEB 181 — Uthden Troll (reprint)

// LEB 182 — Wall of Fire (reprint)

// LEB 183 — Wall of Stone (reprint)

// LEB 184 — Wheel of Fortune (reprint)

// LEB 185 — Aspect of Wolf (reprint)

// LEB 186 — Berserk (reprint)

// LEB 187 — Birds of Paradise (reprint)

// LEB 188 — Camouflage (reprint)

// LEB 189 — Channel (reprint)

// LEB 190 — Cockatrice (reprint)

// LEB 191 — Craw Wurm (reprint)

// LEB 192 — Elvish Archers (reprint)

// LEB 193 — Fastbond (reprint)

// LEB 194 — Fog (reprint)

// LEB 195 — Force of Nature (reprint)

// LEB 196 — Fungusaur (reprint)

// LEB 197 — Gaea's Liege (reprint)

// LEB 198 — Giant Growth (reprint)

// LEB 199 — Giant Spider (reprint)

// LEB 200 — Grizzly Bears (reprint)

// LEB 201 — Hurricane (reprint)

// LEB 202 — Ice Storm (reprint)

// LEB 203 — Instill Energy (reprint)

// LEB 204 — Ironroot Treefolk (reprint)

// LEB 205 — Kudzu (reprint)

// LEB 206 — Ley Druid (reprint)

// LEB 207 — Lifeforce (reprint)

// LEB 208 — Lifelace (reprint)

// LEB 209 — Living Artifact (reprint)

// LEB 210 — Living Lands (reprint)

// LEB 211 — Llanowar Elves (reprint)

// LEB 212 — Lure (reprint)

// LEB 213 — Natural Selection (reprint)

// LEB 214 — Regeneration (reprint)

// LEB 215 — Regrowth (reprint)

// LEB 216 — Scryb Sprites (reprint)

// LEB 217 — Shanodin Dryads (reprint)

// LEB 218 — Stream of Life (reprint)

// LEB 219 — Thicket Basilisk (reprint)

// LEB 220 — Timber Wolves (reprint)

// LEB 221 — Tranquility (reprint)

// LEB 222 — Tsunami (reprint)

// LEB 223 — Verduran Enchantress (reprint)

// LEB 224 — Wall of Brambles (reprint)

// LEB 225 — Wall of Ice (reprint)

// LEB 226 — Wall of Wood (reprint)

// LEB 227 — Wanderlust (reprint)

// LEB 228 — War Mammoth (reprint)

// LEB 229 — Web (reprint)

// LEB 230 — Wild Growth (reprint)

// LEB 231 — Ankh of Mishra (reprint)

// LEB 232 — Basalt Monolith (reprint)

// LEB 233 — Black Lotus (reprint)

// LEB 234 — Black Vise (reprint)

// LEB 235 — Celestial Prism (reprint)

// LEB 236 — Chaos Orb (reprint)

// LEB 237 — Clockwork Beast (reprint)

// LEB 238 — Conservator (reprint)

// LEB 239 — Copper Tablet (reprint)

// LEB 240 — Crystal Rod (reprint)

// LEB 241 — Cyclopean Tomb (reprint)

// LEB 242 — Dingus Egg (reprint)

// LEB 243 — Disrupting Scepter (reprint)

// LEB 244 — Forcefield (reprint)

// LEB 245 — Gauntlet of Might (reprint)

// LEB 246 — Glasses of Urza (reprint)

// LEB 247 — Helm of Chatzuk (reprint)

// LEB 248 — Howling Mine (reprint)

// LEB 249 — Icy Manipulator (reprint)

// LEB 250 — Illusionary Mask (reprint)

// LEB 251 — Iron Star (reprint)

// LEB 252 — Ivory Cup (reprint)

// LEB 253 — Jade Monolith (reprint)

// LEB 254 — Jade Statue (reprint)

// LEB 255 — Jayemdae Tome (reprint)

// LEB 256 — Juggernaut (reprint)

// LEB 257 — Kormus Bell (reprint)

// LEB 258 — Library of Leng (reprint)

// LEB 259 — Living Wall (reprint)

// LEB 260 — Mana Vault (reprint)

// LEB 261 — Meekstone (reprint)

// LEB 262 — Mox Emerald (reprint)

// LEB 263 — Mox Jet (reprint)

// LEB 264 — Mox Pearl (reprint)

// LEB 265 — Mox Ruby (reprint)

// LEB 266 — Mox Sapphire (reprint)

// LEB 267 — Nevinyrral's Disk (reprint)

// LEB 268 — Obsianus Golem (reprint)

// LEB 269 — Rod of Ruin (reprint)

// LEB 270 — Sol Ring (reprint)

// LEB 271 — Soul Net (reprint)

// LEB 272 — Sunglasses of Urza (reprint)

// LEB 273 — The Hive (reprint)

// LEB 274 — Throne of Bone (reprint)

// LEB 275 — Time Vault (reprint)

// LEB 276 — Winter Orb (reprint)

// LEB 277 — Wooden Sphere (reprint)

// LEB 278 — Badlands (reprint)

// LEB 279 — Bayou (reprint)

// LEB 280 — Plateau (reprint)

// LEB 281 — Savannah (reprint)

// LEB 282 — Scrubland (reprint)

// LEB 283 — Taiga (reprint)

// LEB 284 — Tropical Island (reprint)

// LEB 285 — Tundra (reprint)

// LEB 286 — Underground Sea (reprint)

// LEB 287 — Volcanic Island
pub(in crate::card::sets) static VOLCANIC_ISLAND: CardRecord = CardRecord::new_with_legacy_id(
    57,
    "Volcanic Island",
    CardArt::new("0324641d-af55-4c53-b4dc-c8262e967da5", "Brian Snõddy"),
    CardSet::Beta,
    CardRules::new_land(&["Island", "Mountain"]),
);

// LEB 288 — Plains (reprint)

// LEB 289 — Plains (alternate printing)

// LEB 290 — Plains (alternate printing)

// LEB 291 — Island (reprint)

// LEB 292 — Island (alternate printing)

// LEB 293 — Island (alternate printing)

// LEB 294 — Swamp (reprint)

// LEB 295 — Swamp (alternate printing)

// LEB 296 — Swamp (alternate printing)

// LEB 297 — Mountain (reprint)

// LEB 298 — Mountain (alternate printing)

// LEB 299 — Mountain (alternate printing)

// LEB 300 — Forest (reprint)

// LEB 301 — Forest (alternate printing)

// LEB 302 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&CIRCLE_OF_PROTECTION_BLACK, &VOLCANIC_ISLAND];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_lea::ANIMATE_WALL), // LEB 1
    PrintingRecord::reprint(&alpha::ARMAGEDDON),         // LEB 2
    PrintingRecord::reprint(&alpha::BALANCE),            // LEB 3
    PrintingRecord::reprint(&catalog_lea::BENALISH_HERO), // LEB 4
    PrintingRecord::reprint(&catalog_lea::BLACK_WARD),   // LEB 5
    PrintingRecord::reprint(&catalog_lea::BLAZE_OF_GLORY), // LEB 6
    PrintingRecord::reprint(&catalog_lea::BLESSING),     // LEB 7
    PrintingRecord::reprint(&catalog_lea::BLUE_WARD),    // LEB 8
    PrintingRecord::reprint(&catalog_lea::CASTLE),       // LEB 9
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE), // LEB 11
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // LEB 12
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED), // LEB 13
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // LEB 14
    PrintingRecord::reprint(&catalog_lea::CONSECRATE_LAND), // LEB 15
    PrintingRecord::reprint(&catalog_lea::CONVERSION),   // LEB 16
    PrintingRecord::reprint(&alpha::CRUSADE),            // LEB 17
    PrintingRecord::reprint(&catalog_lea::DEATH_WARD),   // LEB 18
    PrintingRecord::reprint(&alpha::DISENCHANT),         // LEB 19
    PrintingRecord::reprint(&catalog_lea::FARMSTEAD),    // LEB 20
    PrintingRecord::reprint(&catalog_lea::GREEN_WARD),   // LEB 21
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL),     // LEB 22
    PrintingRecord::reprint(&catalog_lea::HEALING_SALVE), // LEB 23
    PrintingRecord::reprint(&catalog_lea::HOLY_ARMOR),   // LEB 24
    PrintingRecord::reprint(&catalog_lea::HOLY_STRENGTH), // LEB 25
    PrintingRecord::reprint(&catalog_lea::ISLAND_SANCTUARY), // LEB 26
    PrintingRecord::reprint(&catalog_lea::KARMA),        // LEB 27
    PrintingRecord::reprint(&catalog_lea::LANCE),        // LEB 28
    PrintingRecord::reprint(&catalog_lea::MESA_PEGASUS), // LEB 29
    PrintingRecord::reprint(&catalog_lea::NORTHERN_PALADIN), // LEB 30
    PrintingRecord::reprint(&catalog_lea::PEARLED_UNICORN), // LEB 31
    PrintingRecord::reprint(&catalog_lea::PERSONAL_INCARNATION), // LEB 32
    PrintingRecord::reprint(&catalog_lea::PURELACE),     // LEB 33
    PrintingRecord::reprint(&catalog_lea::RED_WARD),     // LEB 34
    PrintingRecord::reprint(&catalog_lea::RESURRECTION), // LEB 35
    PrintingRecord::reprint(&catalog_lea::REVERSE_DAMAGE), // LEB 36
    PrintingRecord::reprint(&catalog_lea::RIGHTEOUSNESS), // LEB 37
    PrintingRecord::reprint(&catalog_lea::SAMITE_HEALER), // LEB 38
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS),     // LEB 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),        // LEB 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // LEB 41
    PrintingRecord::reprint(&catalog_lea::VETERAN_BODYGUARD), // LEB 42
    PrintingRecord::reprint(&catalog_lea::WALL_OF_SWORDS), // LEB 43
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),       // LEB 44
    PrintingRecord::reprint(&catalog_lea::WHITE_WARD),   // LEB 45
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),       // LEB 46
    PrintingRecord::reprint(&catalog_lea::AIR_ELEMENTAL), // LEB 47
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL),   // LEB 48
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT),   // LEB 49
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // LEB 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),        // LEB 51
    PrintingRecord::reprint(&catalog_lea::CLONE),        // LEB 52
    PrintingRecord::reprint(&catalog_lea::CONTROL_MAGIC), // LEB 53
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),      // LEB 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),       // LEB 55
    PrintingRecord::reprint(&catalog_lea::CREATURE_BOND), // LEB 56
    PrintingRecord::reprint(&catalog_lea::DRAIN_POWER),  // LEB 57
    PrintingRecord::reprint(&catalog_lea::FEEDBACK),     // LEB 58
    PrintingRecord::reprint(&catalog_lea::FLIGHT),       // LEB 59
    PrintingRecord::reprint(&catalog_lea::INVISIBILITY), // LEB 60
    PrintingRecord::reprint(&catalog_lea::JUMP),         // LEB 61
    PrintingRecord::reprint(&catalog_lea::LIFETAP),      // LEB 62
    PrintingRecord::reprint(&catalog_lea::LORD_OF_ATLANTIS), // LEB 63
    PrintingRecord::reprint(&catalog_lea::MAGICAL_HACK), // LEB 64
    PrintingRecord::reprint(&catalog_lea::MAHAMOTI_DJINN), // LEB 65
    PrintingRecord::reprint(&alpha::MANA_SHORT),         // LEB 66
    PrintingRecord::reprint(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT), // LEB 67
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_FORCES), // LEB 68
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_TERRAIN), // LEB 69
    PrintingRecord::reprint(&catalog_lea::PHANTOM_MONSTER), // LEB 70
    PrintingRecord::reprint(&catalog_lea::PIRATE_SHIP),  // LEB 71
    PrintingRecord::reprint(&catalog_lea::POWER_LEAK),   // LEB 72
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),   // LEB 73
    PrintingRecord::reprint(&catalog_lea::PRODIGAL_SORCERER), // LEB 74
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),      // LEB 75
    PrintingRecord::reprint(&catalog_lea::PSYCHIC_VENOM), // LEB 76
    PrintingRecord::reprint(&catalog_lea::SEA_SERPENT),  // LEB 77
    PrintingRecord::reprint(&catalog_lea::SIREN_S_CALL), // LEB 78
    PrintingRecord::reprint(&catalog_lea::SLEIGHT_OF_MIND), // LEB 79
    PrintingRecord::reprint(&catalog_lea::SPELL_BLAST),  // LEB 80
    PrintingRecord::reprint(&alpha::STASIS),             // LEB 81
    PrintingRecord::reprint(&catalog_lea::STEAL_ARTIFACT), // LEB 82
    PrintingRecord::reprint(&catalog_lea::THOUGHTLACE),  // LEB 83
    PrintingRecord::reprint(&alpha::TIME_WALK),          // LEB 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),        // LEB 85
    PrintingRecord::reprint(&catalog_lea::TWIDDLE),      // LEB 86
    PrintingRecord::reprint(&catalog_lea::UNSUMMON),     // LEB 87
    PrintingRecord::reprint(&catalog_lea::VESUVAN_DOPPELGANGER), // LEB 88
    PrintingRecord::reprint(&catalog_lea::VOLCANIC_ERUPTION), // LEB 89
    PrintingRecord::reprint(&catalog_lea::WALL_OF_AIR),  // LEB 90
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WATER), // LEB 91
    PrintingRecord::reprint(&catalog_lea::WATER_ELEMENTAL), // LEB 92
    PrintingRecord::reprint(&catalog_lea::ANIMATE_DEAD), // LEB 93
    PrintingRecord::reprint(&catalog_lea::BAD_MOON),     // LEB 94
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),       // LEB 95
    PrintingRecord::reprint(&catalog_lea::BOG_WRAITH),   // LEB 96
    PrintingRecord::reprint(&catalog_lea::CONTRACT_FROM_BELOW), // LEB 97
    PrintingRecord::reprint(&catalog_lea::CURSED_LAND),  // LEB 98
    PrintingRecord::reprint(&alpha::DARK_RITUAL),        // LEB 99
    PrintingRecord::reprint(&catalog_lea::DARKPACT),     // LEB 100
    PrintingRecord::reprint(&catalog_lea::DEATHGRIP),    // LEB 101
    PrintingRecord::reprint(&catalog_lea::DEATHLACE),    // LEB 102
    PrintingRecord::reprint(&catalog_lea::DEMONIC_ATTORNEY), // LEB 103
    PrintingRecord::reprint(&catalog_lea::DEMONIC_HORDES), // LEB 104
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),      // LEB 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),         // LEB 106
    PrintingRecord::reprint(&catalog_lea::DRUDGE_SKELETONS), // LEB 107
    PrintingRecord::reprint(&catalog_lea::EVIL_PRESENCE), // LEB 108
    PrintingRecord::reprint(&catalog_lea::FEAR),         // LEB 109
    PrintingRecord::reprint(&catalog_lea::FROZEN_SHADE), // LEB 110
    PrintingRecord::reprint(&catalog_lea::GLOOM),        // LEB 111
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND), // LEB 112
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER),   // LEB 113
    PrintingRecord::reprint(&catalog_lea::LICH),         // LEB 114
    PrintingRecord::reprint(&catalog_lea::LORD_OF_THE_PIT), // LEB 115
    PrintingRecord::reprint(&alpha::MIND_TWIST),         // LEB 116
    PrintingRecord::reprint(&catalog_lea::NETHER_SHADOW), // LEB 117
    PrintingRecord::reprint(&catalog_lea::NETTLING_IMP), // LEB 118
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),    // LEB 119
    PrintingRecord::reprint(&catalog_lea::PARALYZE),     // LEB 120
    PrintingRecord::reprint(&catalog_lea::PESTILENCE),   // LEB 121
    PrintingRecord::reprint(&catalog_lea::PLAGUE_RATS),  // LEB 122
    PrintingRecord::reprint(&catalog_lea::RAISE_DEAD),   // LEB 123
    PrintingRecord::reprint(&catalog_lea::ROYAL_ASSASSIN), // LEB 124
    PrintingRecord::reprint(&catalog_lea::SACRIFICE),    // LEB 125
    PrintingRecord::reprint(&catalog_lea::SCATHE_ZOMBIES), // LEB 126
    PrintingRecord::reprint(&catalog_lea::SCAVENGING_GHOUL), // LEB 127
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE),     // LEB 128
    PrintingRecord::reprint(&catalog_lea::SIMULACRUM),   // LEB 129
    PrintingRecord::reprint(&alpha::SINKHOLE),           // LEB 130
    PrintingRecord::reprint(&alpha::TERROR),             // LEB 131
    PrintingRecord::reprint(&catalog_lea::UNHOLY_STRENGTH), // LEB 132
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BONE), // LEB 133
    PrintingRecord::reprint(&catalog_lea::WARP_ARTIFACT), // LEB 134
    PrintingRecord::reprint(&catalog_lea::WEAKNESS),     // LEB 135
    PrintingRecord::reprint(&catalog_lea::WILL_O_THE_WISP), // LEB 136
    PrintingRecord::reprint(&catalog_lea::WORD_OF_COMMAND), // LEB 137
    PrintingRecord::reprint(&catalog_lea::ZOMBIE_MASTER), // LEB 138
    PrintingRecord::reprint(&catalog_lea::BURROWING),    // LEB 139
    PrintingRecord::reprint(&catalog_lea::CHAOSLACE),    // LEB 140
    PrintingRecord::reprint(&catalog_lea::DISINTEGRATE), // LEB 141
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),       // LEB 142
    PrintingRecord::reprint(&catalog_lea::DWARVEN_DEMOLITION_TEAM), // LEB 143
    PrintingRecord::reprint(&catalog_lea::DWARVEN_WARRIORS), // LEB 144
    PrintingRecord::reprint(&catalog_lea::EARTH_ELEMENTAL), // LEB 145
    PrintingRecord::reprint(&catalog_lea::EARTHBIND),    // LEB 146
    PrintingRecord::reprint(&alpha::EARTHQUAKE),         // LEB 147
    PrintingRecord::reprint(&catalog_lea::FALSE_ORDERS), // LEB 148
    PrintingRecord::reprint(&catalog_lea::FIRE_ELEMENTAL), // LEB 149
    PrintingRecord::reprint(&alpha::FIREBALL),           // LEB 150
    PrintingRecord::reprint(&catalog_lea::FIREBREATHING), // LEB 151
    PrintingRecord::reprint(&catalog_lea::FLASHFIRES),   // LEB 152
    PrintingRecord::reprint(&alpha::FORK),               // LEB 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // LEB 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),        // LEB 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE),   // LEB 156
    PrintingRecord::reprint(&catalog_lea::GRAY_OGRE),    // LEB 157
    PrintingRecord::reprint(&catalog_lea::HILL_GIANT),   // LEB 158
    PrintingRecord::reprint(&catalog_lea::HURLOON_MINOTAUR), // LEB 159
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),      // LEB 160
    PrintingRecord::reprint(&catalog_lea::KELDON_WARLORD), // LEB 161
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT),     // LEB 162
    PrintingRecord::reprint(&catalog_lea::MANA_FLARE),   // LEB 163
    PrintingRecord::reprint(&catalog_lea::MANABARBS),    // LEB 164
    PrintingRecord::reprint(&catalog_lea::MONSS_GOBLIN_RAIDERS), // LEB 165
    PrintingRecord::reprint(&catalog_lea::ORCISH_ARTILLERY), // LEB 166
    PrintingRecord::reprint(&catalog_lea::ORCISH_ORIFLAMME), // LEB 167
    PrintingRecord::reprint(&catalog_lea::POWER_SURGE),  // LEB 168
    PrintingRecord::reprint(&catalog_lea::RAGING_RIVER), // LEB 169
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // LEB 170
    PrintingRecord::reprint(&catalog_lea::ROC_OF_KHER_RIDGES), // LEB 171
    PrintingRecord::reprint(&catalog_lea::ROCK_HYDRA),   // LEB 172
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),        // LEB 173
    PrintingRecord::reprint(&alpha::SHATTER),            // LEB 174
    PrintingRecord::reprint(&catalog_lea::SHIVAN_DRAGON), // LEB 175
    PrintingRecord::reprint(&alpha::SMOKE),              // LEB 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),        // LEB 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),         // LEB 178
    PrintingRecord::reprint(&catalog_lea::TUNNEL),       // LEB 179
    PrintingRecord::reprint(&catalog_lea::TWO_HEADED_GIANT_OF_FORIYS), // LEB 180
    PrintingRecord::reprint(&catalog_lea::UTHDEN_TROLL), // LEB 181
    PrintingRecord::reprint(&catalog_lea::WALL_OF_FIRE), // LEB 182
    PrintingRecord::reprint(&catalog_lea::WALL_OF_STONE), // LEB 183
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE),   // LEB 184
    PrintingRecord::reprint(&catalog_lea::ASPECT_OF_WOLF), // LEB 185
    PrintingRecord::reprint(&alpha::BERSERK),            // LEB 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE),  // LEB 187
    PrintingRecord::reprint(&catalog_lea::CAMOUFLAGE),   // LEB 188
    PrintingRecord::reprint(&alpha::CHANNEL),            // LEB 189
    PrintingRecord::reprint(&catalog_lea::COCKATRICE),   // LEB 190
    PrintingRecord::reprint(&catalog_lea::CRAW_WURM),    // LEB 191
    PrintingRecord::reprint(&catalog_lea::ELVISH_ARCHERS), // LEB 192
    PrintingRecord::reprint(&catalog_lea::FASTBOND),     // LEB 193
    PrintingRecord::reprint(&catalog_lea::FOG),          // LEB 194
    PrintingRecord::reprint(&catalog_lea::FORCE_OF_NATURE), // LEB 195
    PrintingRecord::reprint(&catalog_lea::FUNGUSAUR),    // LEB 196
    PrintingRecord::reprint(&catalog_lea::GAEA_S_LIEGE), // LEB 197
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),       // LEB 198
    PrintingRecord::reprint(&catalog_lea::GIANT_SPIDER), // LEB 199
    PrintingRecord::reprint(&catalog_lea::GRIZZLY_BEARS), // LEB 200
    PrintingRecord::reprint(&catalog_lea::HURRICANE),    // LEB 201
    PrintingRecord::reprint(&catalog_lea::ICE_STORM),    // LEB 202
    PrintingRecord::reprint(&catalog_lea::INSTILL_ENERGY), // LEB 203
    PrintingRecord::reprint(&catalog_lea::IRONROOT_TREEFOLK), // LEB 204
    PrintingRecord::reprint(&catalog_lea::KUDZU),        // LEB 205
    PrintingRecord::reprint(&catalog_lea::LEY_DRUID),    // LEB 206
    PrintingRecord::reprint(&catalog_lea::LIFEFORCE),    // LEB 207
    PrintingRecord::reprint(&catalog_lea::LIFELACE),     // LEB 208
    PrintingRecord::reprint(&catalog_lea::LIVING_ARTIFACT), // LEB 209
    PrintingRecord::reprint(&catalog_lea::LIVING_LANDS), // LEB 210
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES),     // LEB 211
    PrintingRecord::reprint(&catalog_lea::LURE),         // LEB 212
    PrintingRecord::reprint(&catalog_lea::NATURAL_SELECTION), // LEB 213
    PrintingRecord::reprint(&catalog_lea::REGENERATION), // LEB 214
    PrintingRecord::reprint(&alpha::REGROWTH),           // LEB 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),      // LEB 216
    PrintingRecord::reprint(&catalog_lea::SHANODIN_DRYADS), // LEB 217
    PrintingRecord::reprint(&catalog_lea::STREAM_OF_LIFE), // LEB 218
    PrintingRecord::reprint(&catalog_lea::THICKET_BASILISK), // LEB 219
    PrintingRecord::reprint(&catalog_lea::TIMBER_WOLVES), // LEB 220
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),  // LEB 221
    PrintingRecord::reprint(&catalog_lea::TSUNAMI),      // LEB 222
    PrintingRecord::reprint(&catalog_lea::VERDURAN_ENCHANTRESS), // LEB 223
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BRAMBLES), // LEB 224
    PrintingRecord::reprint(&catalog_lea::WALL_OF_ICE),  // LEB 225
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WOOD), // LEB 226
    PrintingRecord::reprint(&catalog_lea::WANDERLUST),   // LEB 227
    PrintingRecord::reprint(&catalog_lea::WAR_MAMMOTH),  // LEB 228
    PrintingRecord::reprint(&catalog_lea::WEB),          // LEB 229
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),  // LEB 230
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA),     // LEB 231
    PrintingRecord::reprint(&catalog_lea::BASALT_MONOLITH), // LEB 232
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),        // LEB 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),         // LEB 234
    PrintingRecord::reprint(&catalog_lea::CELESTIAL_PRISM), // LEB 235
    PrintingRecord::reprint(&alpha::CHAOS_ORB),          // LEB 236
    PrintingRecord::reprint(&catalog_lea::CLOCKWORK_BEAST), // LEB 237
    PrintingRecord::reprint(&catalog_lea::CONSERVATOR),  // LEB 238
    PrintingRecord::reprint(&alpha::COPPER_TABLET),      // LEB 239
    PrintingRecord::reprint(&catalog_lea::CRYSTAL_ROD),  // LEB 240
    PrintingRecord::reprint(&catalog_lea::CYCLOPEAN_TOMB), // LEB 241
    PrintingRecord::reprint(&catalog_lea::DINGUS_EGG),   // LEB 242
    PrintingRecord::reprint(&catalog_lea::DISRUPTING_SCEPTER), // LEB 243
    PrintingRecord::reprint(&catalog_lea::FORCEFIELD),   // LEB 244
    PrintingRecord::reprint(&catalog_lea::GAUNTLET_OF_MIGHT), // LEB 245
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA),    // LEB 246
    PrintingRecord::reprint(&catalog_lea::HELM_OF_CHATZUK), // LEB 247
    PrintingRecord::reprint(&catalog_lea::HOWLING_MINE), // LEB 248
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR),    // LEB 249
    PrintingRecord::reprint(&catalog_lea::ILLUSIONARY_MASK), // LEB 250
    PrintingRecord::reprint(&alpha::IRON_STAR),          // LEB 251
    PrintingRecord::reprint(&catalog_lea::IVORY_CUP),    // LEB 252
    PrintingRecord::reprint(&catalog_lea::JADE_MONOLITH), // LEB 253
    PrintingRecord::reprint(&catalog_lea::JADE_STATUE),  // LEB 254
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),      // LEB 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),         // LEB 256
    PrintingRecord::reprint(&catalog_lea::KORMUS_BELL),  // LEB 257
    PrintingRecord::reprint(&catalog_lea::LIBRARY_OF_LENG), // LEB 258
    PrintingRecord::reprint(&catalog_lea::LIVING_WALL),  // LEB 259
    PrintingRecord::reprint(&alpha::MANA_VAULT),         // LEB 260
    PrintingRecord::reprint(&catalog_lea::MEEKSTONE),    // LEB 261
    PrintingRecord::reprint(&alpha::MOX_EMERALD),        // LEB 262
    PrintingRecord::reprint(&alpha::MOX_JET),            // LEB 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),          // LEB 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),           // LEB 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),       // LEB 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK),   // LEB 267
    PrintingRecord::reprint(&catalog_lea::OBSIANUS_GOLEM), // LEB 268
    PrintingRecord::reprint(&catalog_lea::ROD_OF_RUIN),  // LEB 269
    PrintingRecord::reprint(&alpha::SOL_RING),           // LEB 270
    PrintingRecord::reprint(&catalog_lea::SOUL_NET),     // LEB 271
    PrintingRecord::reprint(&catalog_lea::SUNGLASSES_OF_URZA), // LEB 272
    PrintingRecord::reprint(&catalog_lea::THE_HIVE),     // LEB 273
    PrintingRecord::reprint(&catalog_lea::THRONE_OF_BONE), // LEB 274
    PrintingRecord::reprint(&alpha::TIME_VAULT),         // LEB 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),         // LEB 276
    PrintingRecord::reprint(&catalog_lea::WOODEN_SPHERE), // LEB 277
    PrintingRecord::reprint(&alpha::BADLANDS),           // LEB 278
    PrintingRecord::reprint(&alpha::BAYOU),              // LEB 279
    PrintingRecord::reprint(&alpha::PLATEAU),            // LEB 280
    PrintingRecord::reprint(&alpha::SAVANNAH),           // LEB 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),          // LEB 282
    PrintingRecord::reprint(&alpha::TAIGA),              // LEB 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND),    // LEB 284
    PrintingRecord::reprint(&alpha::TUNDRA),             // LEB 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA),    // LEB 286
    PrintingRecord::reprint(&alpha::PLAINS),             // LEB 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),        // LEB 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),        // LEB 290
    PrintingRecord::reprint(&alpha::ISLAND),             // LEB 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),        // LEB 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),        // LEB 293
    PrintingRecord::reprint(&alpha::SWAMP),              // LEB 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),         // LEB 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),         // LEB 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),           // LEB 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),      // LEB 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),      // LEB 299
    PrintingRecord::reprint(&alpha::FOREST),             // LEB 300
    PrintingRecord::alternate(&alpha::FOREST, 1),        // LEB 301
    PrintingRecord::alternate(&alpha::FOREST, 2),        // LEB 302
];
