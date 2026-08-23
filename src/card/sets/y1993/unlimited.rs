//! Unlimited Edition has no unique card definitions.
//!
//! Every card in the built-in Unlimited catalog points to its first printing.

use super::{CardRecord, PrintingRecord, alpha, beta};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;

// 2ED 1 — Animate Wall (reprint)

// 2ED 2 — Armageddon (reprint)

// 2ED 3 — Balance (reprint)

// 2ED 4 — Benalish Hero (reprint)

// 2ED 5 — Black Ward (reprint)

// 2ED 6 — Blaze of Glory (reprint)

// 2ED 7 — Blessing (reprint)

// 2ED 8 — Blue Ward (reprint)

// 2ED 9 — Castle (reprint)

// 2ED 10 — Circle of Protection: Black (reprint)

// 2ED 11 — Circle of Protection: Blue (reprint)

// 2ED 12 — Circle of Protection: Green (reprint)

// 2ED 13 — Circle of Protection: Red (reprint)

// 2ED 14 — Circle of Protection: White (reprint)

// 2ED 15 — Consecrate Land (reprint)

// 2ED 16 — Conversion (reprint)

// 2ED 17 — Crusade (reprint)

// 2ED 18 — Death Ward (reprint)

// 2ED 19 — Disenchant (reprint)

// 2ED 20 — Farmstead (reprint)

// 2ED 21 — Green Ward (reprint)

// 2ED 22 — Guardian Angel (reprint)

// 2ED 23 — Healing Salve (reprint)

// 2ED 24 — Holy Armor (reprint)

// 2ED 25 — Holy Strength (reprint)

// 2ED 26 — Island Sanctuary (reprint)

// 2ED 27 — Karma (reprint)

// 2ED 28 — Lance (reprint)

// 2ED 29 — Mesa Pegasus (reprint)

// 2ED 30 — Northern Paladin (reprint)

// 2ED 31 — Pearled Unicorn (reprint)

// 2ED 32 — Personal Incarnation (reprint)

// 2ED 33 — Purelace (reprint)

// 2ED 34 — Red Ward (reprint)

// 2ED 35 — Resurrection (reprint)

// 2ED 36 — Reverse Damage (reprint)

// 2ED 37 — Righteousness (reprint)

// 2ED 38 — Samite Healer (reprint)

// 2ED 39 — Savannah Lions (reprint)

// 2ED 40 — Serra Angel (reprint)

// 2ED 41 — Swords to Plowshares (reprint)

// 2ED 42 — Veteran Bodyguard (reprint)

// 2ED 43 — Wall of Swords (reprint)

// 2ED 44 — White Knight (reprint)

// 2ED 45 — White Ward (reprint)

// 2ED 46 — Wrath of God (reprint)

// 2ED 47 — Air Elemental (reprint)

// 2ED 48 — Ancestral Recall (reprint)

// 2ED 49 — Animate Artifact (reprint)

// 2ED 50 — Blue Elemental Blast (reprint)

// 2ED 51 — Braingeyser (reprint)

// 2ED 52 — Clone (reprint)

// 2ED 53 — Control Magic (reprint)

// 2ED 54 — Copy Artifact (reprint)

// 2ED 55 — Counterspell (reprint)

// 2ED 56 — Creature Bond (reprint)

// 2ED 57 — Drain Power (reprint)

// 2ED 58 — Feedback (reprint)

// 2ED 59 — Flight (reprint)

// 2ED 60 — Invisibility (reprint)

// 2ED 61 — Jump (reprint)

// 2ED 62 — Lifetap (reprint)

// 2ED 63 — Lord of Atlantis (reprint)

// 2ED 64 — Magical Hack (reprint)

// 2ED 65 — Mahamoti Djinn (reprint)

// 2ED 66 — Mana Short (reprint)

// 2ED 67 — Merfolk of the Pearl Trident (reprint)

// 2ED 68 — Phantasmal Forces (reprint)

// 2ED 69 — Phantasmal Terrain (reprint)

// 2ED 70 — Phantom Monster (reprint)

// 2ED 71 — Pirate Ship (reprint)

// 2ED 72 — Power Leak (reprint)

// 2ED 73 — Power Sink (reprint)

// 2ED 74 — Prodigal Sorcerer (reprint)

// 2ED 75 — Psionic Blast (reprint)

// 2ED 76 — Psychic Venom (reprint)

// 2ED 77 — Sea Serpent (reprint)

// 2ED 78 — Siren's Call (reprint)

// 2ED 79 — Sleight of Mind (reprint)

// 2ED 80 — Spell Blast (reprint)

// 2ED 81 — Stasis (reprint)

// 2ED 82 — Steal Artifact (reprint)

// 2ED 83 — Thoughtlace (reprint)

// 2ED 84 — Time Walk (reprint)

// 2ED 85 — Timetwister (reprint)

// 2ED 86 — Twiddle (reprint)

// 2ED 87 — Unsummon (reprint)

// 2ED 88 — Vesuvan Doppelganger (reprint)

// 2ED 89 — Volcanic Eruption (reprint)

// 2ED 90 — Wall of Air (reprint)

// 2ED 91 — Wall of Water (reprint)

// 2ED 92 — Water Elemental (reprint)

// 2ED 93 — Animate Dead (reprint)

// 2ED 94 — Bad Moon (reprint)

// 2ED 95 — Black Knight (reprint)

// 2ED 96 — Bog Wraith (reprint)

// 2ED 97 — Contract from Below (reprint)

// 2ED 98 — Cursed Land (reprint)

// 2ED 99 — Dark Ritual (reprint)

// 2ED 100 — Darkpact (reprint)

// 2ED 101 — Deathgrip (reprint)

// 2ED 102 — Deathlace (reprint)

// 2ED 103 — Demonic Attorney (reprint)

// 2ED 104 — Demonic Hordes (reprint)

// 2ED 105 — Demonic Tutor (reprint)

// 2ED 106 — Drain Life (reprint)

// 2ED 107 — Drudge Skeletons (reprint)

// 2ED 108 — Evil Presence (reprint)

// 2ED 109 — Fear (reprint)

// 2ED 110 — Frozen Shade (reprint)

// 2ED 111 — Gloom (reprint)

// 2ED 112 — Howl from Beyond (reprint)

// 2ED 113 — Hypnotic Specter (reprint)

// 2ED 114 — Lich (reprint)

// 2ED 115 — Lord of the Pit (reprint)

// 2ED 116 — Mind Twist (reprint)

// 2ED 117 — Nether Shadow (reprint)

// 2ED 118 — Nettling Imp (reprint)

// 2ED 119 — Nightmare (reprint)

// 2ED 120 — Paralyze (reprint)

// 2ED 121 — Pestilence (reprint)

// 2ED 122 — Plague Rats (reprint)

// 2ED 123 — Raise Dead (reprint)

// 2ED 124 — Royal Assassin (reprint)

// 2ED 125 — Sacrifice (reprint)

// 2ED 126 — Scathe Zombies (reprint)

// 2ED 127 — Scavenging Ghoul (reprint)

// 2ED 128 — Sengir Vampire (reprint)

// 2ED 129 — Simulacrum (reprint)

// 2ED 130 — Sinkhole (reprint)

// 2ED 131 — Terror (reprint)

// 2ED 132 — Unholy Strength (reprint)

// 2ED 133 — Wall of Bone (reprint)

// 2ED 134 — Warp Artifact (reprint)

// 2ED 135 — Weakness (reprint)

// 2ED 136 — Will-o'-the-Wisp (reprint)

// 2ED 137 — Word of Command (reprint)

// 2ED 138 — Zombie Master (reprint)

// 2ED 139 — Burrowing (reprint)

// 2ED 140 — Chaoslace (reprint)

// 2ED 141 — Disintegrate (reprint)

// 2ED 142 — Dragon Whelp (reprint)

// 2ED 143 — Dwarven Demolition Team (reprint)

// 2ED 144 — Dwarven Warriors (reprint)

// 2ED 145 — Earth Elemental (reprint)

// 2ED 146 — Earthbind (reprint)

// 2ED 147 — Earthquake (reprint)

// 2ED 148 — False Orders (reprint)

// 2ED 149 — Fire Elemental (reprint)

// 2ED 150 — Fireball (reprint)

// 2ED 151 — Firebreathing (reprint)

// 2ED 152 — Flashfires (reprint)

// 2ED 153 — Fork (reprint)

// 2ED 154 — Goblin Balloon Brigade (reprint)

// 2ED 155 — Goblin King (reprint)

// 2ED 156 — Granite Gargoyle (reprint)

// 2ED 157 — Gray Ogre (reprint)

// 2ED 158 — Hill Giant (reprint)

// 2ED 159 — Hurloon Minotaur (reprint)

// 2ED 160 — Ironclaw Orcs (reprint)

// 2ED 161 — Keldon Warlord (reprint)

// 2ED 162 — Lightning Bolt (reprint)

// 2ED 163 — Mana Flare (reprint)

// 2ED 164 — Manabarbs (reprint)

// 2ED 165 — Mons's Goblin Raiders (reprint)

// 2ED 166 — Orcish Artillery (reprint)

// 2ED 167 — Orcish Oriflamme (reprint)

// 2ED 168 — Power Surge (reprint)

// 2ED 169 — Raging River (reprint)

// 2ED 170 — Red Elemental Blast (reprint)

// 2ED 171 — Roc of Kher Ridges (reprint)

// 2ED 172 — Rock Hydra (reprint)

// 2ED 173 — Sedge Troll (reprint)

// 2ED 174 — Shatter (reprint)

// 2ED 175 — Shivan Dragon (reprint)

// 2ED 176 — Smoke (reprint)

// 2ED 177 — Stone Giant (reprint)

// 2ED 178 — Stone Rain (reprint)

// 2ED 179 — Tunnel (reprint)

// 2ED 180 — Two-Headed Giant of Foriys (reprint)

// 2ED 181 — Uthden Troll (reprint)

// 2ED 182 — Wall of Fire (reprint)

// 2ED 183 — Wall of Stone (reprint)

// 2ED 184 — Wheel of Fortune (reprint)

// 2ED 185 — Aspect of Wolf (reprint)

// 2ED 186 — Berserk (reprint)

// 2ED 187 — Birds of Paradise (reprint)

// 2ED 188 — Camouflage (reprint)

// 2ED 189 — Channel (reprint)

// 2ED 190 — Cockatrice (reprint)

// 2ED 191 — Craw Wurm (reprint)

// 2ED 192 — Elvish Archers (reprint)

// 2ED 193 — Fastbond (reprint)

// 2ED 194 — Fog (reprint)

// 2ED 195 — Force of Nature (reprint)

// 2ED 196 — Fungusaur (reprint)

// 2ED 197 — Gaea's Liege (reprint)

// 2ED 198 — Giant Growth (reprint)

// 2ED 199 — Giant Spider (reprint)

// 2ED 200 — Grizzly Bears (reprint)

// 2ED 201 — Hurricane (reprint)

// 2ED 202 — Ice Storm (reprint)

// 2ED 203 — Instill Energy (reprint)

// 2ED 204 — Ironroot Treefolk (reprint)

// 2ED 205 — Kudzu (reprint)

// 2ED 206 — Ley Druid (reprint)

// 2ED 207 — Lifeforce (reprint)

// 2ED 208 — Lifelace (reprint)

// 2ED 209 — Living Artifact (reprint)

// 2ED 210 — Living Lands (reprint)

// 2ED 211 — Llanowar Elves (reprint)

// 2ED 212 — Lure (reprint)

// 2ED 213 — Natural Selection (reprint)

// 2ED 214 — Regeneration (reprint)

// 2ED 215 — Regrowth (reprint)

// 2ED 216 — Scryb Sprites (reprint)

// 2ED 217 — Shanodin Dryads (reprint)

// 2ED 218 — Stream of Life (reprint)

// 2ED 219 — Thicket Basilisk (reprint)

// 2ED 220 — Timber Wolves (reprint)

// 2ED 221 — Tranquility (reprint)

// 2ED 222 — Tsunami (reprint)

// 2ED 223 — Verduran Enchantress (reprint)

// 2ED 224 — Wall of Brambles (reprint)

// 2ED 225 — Wall of Ice (reprint)

// 2ED 226 — Wall of Wood (reprint)

// 2ED 227 — Wanderlust (reprint)

// 2ED 228 — War Mammoth (reprint)

// 2ED 229 — Web (reprint)

// 2ED 230 — Wild Growth (reprint)

// 2ED 231 — Ankh of Mishra (reprint)

// 2ED 232 — Basalt Monolith (reprint)

// 2ED 233 — Black Lotus (reprint)

// 2ED 234 — Black Vise (reprint)

// 2ED 235 — Celestial Prism (reprint)

// 2ED 236 — Chaos Orb (reprint)

// 2ED 237 — Clockwork Beast (reprint)

// 2ED 238 — Conservator (reprint)

// 2ED 239 — Copper Tablet (reprint)

// 2ED 240 — Crystal Rod (reprint)

// 2ED 241 — Cyclopean Tomb (reprint)

// 2ED 242 — Dingus Egg (reprint)

// 2ED 243 — Disrupting Scepter (reprint)

// 2ED 244 — Forcefield (reprint)

// 2ED 245 — Gauntlet of Might (reprint)

// 2ED 246 — Glasses of Urza (reprint)

// 2ED 247 — Helm of Chatzuk (reprint)

// 2ED 248 — Howling Mine (reprint)

// 2ED 249 — Icy Manipulator (reprint)

// 2ED 250 — Illusionary Mask (reprint)

// 2ED 251 — Iron Star (reprint)

// 2ED 252 — Ivory Cup (reprint)

// 2ED 253 — Jade Monolith (reprint)

// 2ED 254 — Jade Statue (reprint)

// 2ED 255 — Jayemdae Tome (reprint)

// 2ED 256 — Juggernaut (reprint)

// 2ED 257 — Kormus Bell (reprint)

// 2ED 258 — Library of Leng (reprint)

// 2ED 259 — Living Wall (reprint)

// 2ED 260 — Mana Vault (reprint)

// 2ED 261 — Meekstone (reprint)

// 2ED 262 — Mox Emerald (reprint)

// 2ED 263 — Mox Jet (reprint)

// 2ED 264 — Mox Pearl (reprint)

// 2ED 265 — Mox Ruby (reprint)

// 2ED 266 — Mox Sapphire (reprint)

// 2ED 267 — Nevinyrral's Disk (reprint)

// 2ED 268 — Obsianus Golem (reprint)

// 2ED 269 — Rod of Ruin (reprint)

// 2ED 270 — Sol Ring (reprint)

// 2ED 271 — Soul Net (reprint)

// 2ED 272 — Sunglasses of Urza (reprint)

// 2ED 273 — The Hive (reprint)

// 2ED 274 — Throne of Bone (reprint)

// 2ED 275 — Time Vault (reprint)

// 2ED 276 — Winter Orb (reprint)

// 2ED 277 — Wooden Sphere (reprint)

// 2ED 278 — Badlands (reprint)

// 2ED 279 — Bayou (reprint)

// 2ED 280 — Plateau (reprint)

// 2ED 281 — Savannah (reprint)

// 2ED 282 — Scrubland (reprint)

// 2ED 283 — Taiga (reprint)

// 2ED 284 — Tropical Island (reprint)

// 2ED 285 — Tundra (reprint)

// 2ED 286 — Underground Sea (reprint)

// 2ED 287 — Volcanic Island (reprint)

// 2ED 288 — Plains (reprint)

// 2ED 289 — Plains (alternate printing)

// 2ED 290 — Plains (alternate printing)

// 2ED 291 — Island (reprint)

// 2ED 292 — Island (alternate printing)

// 2ED 293 — Island (alternate printing)

// 2ED 294 — Swamp (reprint)

// 2ED 295 — Swamp (alternate printing)

// 2ED 296 — Swamp (alternate printing)

// 2ED 297 — Mountain (reprint)

// 2ED 298 — Mountain (alternate printing)

// 2ED 299 — Mountain (alternate printing)

// 2ED 300 — Forest (reprint)

// 2ED 301 — Forest (alternate printing)

// 2ED 302 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_lea::ANIMATE_WALL), // 2ED 1
    PrintingRecord::reprint(&alpha::ARMAGEDDON),         // 2ED 2
    PrintingRecord::reprint(&alpha::BALANCE),            // 2ED 3
    PrintingRecord::reprint(&catalog_lea::BENALISH_HERO), // 2ED 4
    PrintingRecord::reprint(&catalog_lea::BLACK_WARD),   // 2ED 5
    PrintingRecord::reprint(&catalog_lea::BLAZE_OF_GLORY), // 2ED 6
    PrintingRecord::reprint(&catalog_lea::BLESSING),     // 2ED 7
    PrintingRecord::reprint(&catalog_lea::BLUE_WARD),    // 2ED 8
    PrintingRecord::reprint(&catalog_lea::CASTLE),       // 2ED 9
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // 2ED 10
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE), // 2ED 11
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // 2ED 12
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED), // 2ED 13
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // 2ED 14
    PrintingRecord::reprint(&catalog_lea::CONSECRATE_LAND), // 2ED 15
    PrintingRecord::reprint(&catalog_lea::CONVERSION),   // 2ED 16
    PrintingRecord::reprint(&alpha::CRUSADE),            // 2ED 17
    PrintingRecord::reprint(&catalog_lea::DEATH_WARD),   // 2ED 18
    PrintingRecord::reprint(&alpha::DISENCHANT),         // 2ED 19
    PrintingRecord::reprint(&catalog_lea::FARMSTEAD),    // 2ED 20
    PrintingRecord::reprint(&catalog_lea::GREEN_WARD),   // 2ED 21
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL),     // 2ED 22
    PrintingRecord::reprint(&catalog_lea::HEALING_SALVE), // 2ED 23
    PrintingRecord::reprint(&catalog_lea::HOLY_ARMOR),   // 2ED 24
    PrintingRecord::reprint(&catalog_lea::HOLY_STRENGTH), // 2ED 25
    PrintingRecord::reprint(&catalog_lea::ISLAND_SANCTUARY), // 2ED 26
    PrintingRecord::reprint(&catalog_lea::KARMA),        // 2ED 27
    PrintingRecord::reprint(&catalog_lea::LANCE),        // 2ED 28
    PrintingRecord::reprint(&catalog_lea::MESA_PEGASUS), // 2ED 29
    PrintingRecord::reprint(&catalog_lea::NORTHERN_PALADIN), // 2ED 30
    PrintingRecord::reprint(&catalog_lea::PEARLED_UNICORN), // 2ED 31
    PrintingRecord::reprint(&catalog_lea::PERSONAL_INCARNATION), // 2ED 32
    PrintingRecord::reprint(&catalog_lea::PURELACE),     // 2ED 33
    PrintingRecord::reprint(&catalog_lea::RED_WARD),     // 2ED 34
    PrintingRecord::reprint(&catalog_lea::RESURRECTION), // 2ED 35
    PrintingRecord::reprint(&catalog_lea::REVERSE_DAMAGE), // 2ED 36
    PrintingRecord::reprint(&catalog_lea::RIGHTEOUSNESS), // 2ED 37
    PrintingRecord::reprint(&catalog_lea::SAMITE_HEALER), // 2ED 38
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS),     // 2ED 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),        // 2ED 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // 2ED 41
    PrintingRecord::reprint(&catalog_lea::VETERAN_BODYGUARD), // 2ED 42
    PrintingRecord::reprint(&catalog_lea::WALL_OF_SWORDS), // 2ED 43
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),       // 2ED 44
    PrintingRecord::reprint(&catalog_lea::WHITE_WARD),   // 2ED 45
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),       // 2ED 46
    PrintingRecord::reprint(&catalog_lea::AIR_ELEMENTAL), // 2ED 47
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL),   // 2ED 48
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT),   // 2ED 49
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // 2ED 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),        // 2ED 51
    PrintingRecord::reprint(&catalog_lea::CLONE),        // 2ED 52
    PrintingRecord::reprint(&catalog_lea::CONTROL_MAGIC), // 2ED 53
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),      // 2ED 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),       // 2ED 55
    PrintingRecord::reprint(&catalog_lea::CREATURE_BOND), // 2ED 56
    PrintingRecord::reprint(&catalog_lea::DRAIN_POWER),  // 2ED 57
    PrintingRecord::reprint(&catalog_lea::FEEDBACK),     // 2ED 58
    PrintingRecord::reprint(&catalog_lea::FLIGHT),       // 2ED 59
    PrintingRecord::reprint(&catalog_lea::INVISIBILITY), // 2ED 60
    PrintingRecord::reprint(&catalog_lea::JUMP),         // 2ED 61
    PrintingRecord::reprint(&catalog_lea::LIFETAP),      // 2ED 62
    PrintingRecord::reprint(&catalog_lea::LORD_OF_ATLANTIS), // 2ED 63
    PrintingRecord::reprint(&catalog_lea::MAGICAL_HACK), // 2ED 64
    PrintingRecord::reprint(&catalog_lea::MAHAMOTI_DJINN), // 2ED 65
    PrintingRecord::reprint(&alpha::MANA_SHORT),         // 2ED 66
    PrintingRecord::reprint(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT), // 2ED 67
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_FORCES), // 2ED 68
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_TERRAIN), // 2ED 69
    PrintingRecord::reprint(&catalog_lea::PHANTOM_MONSTER), // 2ED 70
    PrintingRecord::reprint(&catalog_lea::PIRATE_SHIP),  // 2ED 71
    PrintingRecord::reprint(&catalog_lea::POWER_LEAK),   // 2ED 72
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),   // 2ED 73
    PrintingRecord::reprint(&catalog_lea::PRODIGAL_SORCERER), // 2ED 74
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),      // 2ED 75
    PrintingRecord::reprint(&catalog_lea::PSYCHIC_VENOM), // 2ED 76
    PrintingRecord::reprint(&catalog_lea::SEA_SERPENT),  // 2ED 77
    PrintingRecord::reprint(&catalog_lea::SIREN_S_CALL), // 2ED 78
    PrintingRecord::reprint(&catalog_lea::SLEIGHT_OF_MIND), // 2ED 79
    PrintingRecord::reprint(&catalog_lea::SPELL_BLAST),  // 2ED 80
    PrintingRecord::reprint(&alpha::STASIS),             // 2ED 81
    PrintingRecord::reprint(&catalog_lea::STEAL_ARTIFACT), // 2ED 82
    PrintingRecord::reprint(&catalog_lea::THOUGHTLACE),  // 2ED 83
    PrintingRecord::reprint(&alpha::TIME_WALK),          // 2ED 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),        // 2ED 85
    PrintingRecord::reprint(&catalog_lea::TWIDDLE),      // 2ED 86
    PrintingRecord::reprint(&catalog_lea::UNSUMMON),     // 2ED 87
    PrintingRecord::reprint(&catalog_lea::VESUVAN_DOPPELGANGER), // 2ED 88
    PrintingRecord::reprint(&catalog_lea::VOLCANIC_ERUPTION), // 2ED 89
    PrintingRecord::reprint(&catalog_lea::WALL_OF_AIR),  // 2ED 90
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WATER), // 2ED 91
    PrintingRecord::reprint(&catalog_lea::WATER_ELEMENTAL), // 2ED 92
    PrintingRecord::reprint(&catalog_lea::ANIMATE_DEAD), // 2ED 93
    PrintingRecord::reprint(&catalog_lea::BAD_MOON),     // 2ED 94
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),       // 2ED 95
    PrintingRecord::reprint(&catalog_lea::BOG_WRAITH),   // 2ED 96
    PrintingRecord::reprint(&catalog_lea::CONTRACT_FROM_BELOW), // 2ED 97
    PrintingRecord::reprint(&catalog_lea::CURSED_LAND),  // 2ED 98
    PrintingRecord::reprint(&alpha::DARK_RITUAL),        // 2ED 99
    PrintingRecord::reprint(&catalog_lea::DARKPACT),     // 2ED 100
    PrintingRecord::reprint(&catalog_lea::DEATHGRIP),    // 2ED 101
    PrintingRecord::reprint(&catalog_lea::DEATHLACE),    // 2ED 102
    PrintingRecord::reprint(&catalog_lea::DEMONIC_ATTORNEY), // 2ED 103
    PrintingRecord::reprint(&catalog_lea::DEMONIC_HORDES), // 2ED 104
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),      // 2ED 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),         // 2ED 106
    PrintingRecord::reprint(&catalog_lea::DRUDGE_SKELETONS), // 2ED 107
    PrintingRecord::reprint(&catalog_lea::EVIL_PRESENCE), // 2ED 108
    PrintingRecord::reprint(&catalog_lea::FEAR),         // 2ED 109
    PrintingRecord::reprint(&catalog_lea::FROZEN_SHADE), // 2ED 110
    PrintingRecord::reprint(&catalog_lea::GLOOM),        // 2ED 111
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND), // 2ED 112
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER),   // 2ED 113
    PrintingRecord::reprint(&catalog_lea::LICH),         // 2ED 114
    PrintingRecord::reprint(&catalog_lea::LORD_OF_THE_PIT), // 2ED 115
    PrintingRecord::reprint(&alpha::MIND_TWIST),         // 2ED 116
    PrintingRecord::reprint(&catalog_lea::NETHER_SHADOW), // 2ED 117
    PrintingRecord::reprint(&catalog_lea::NETTLING_IMP), // 2ED 118
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),    // 2ED 119
    PrintingRecord::reprint(&catalog_lea::PARALYZE),     // 2ED 120
    PrintingRecord::reprint(&catalog_lea::PESTILENCE),   // 2ED 121
    PrintingRecord::reprint(&catalog_lea::PLAGUE_RATS),  // 2ED 122
    PrintingRecord::reprint(&catalog_lea::RAISE_DEAD),   // 2ED 123
    PrintingRecord::reprint(&catalog_lea::ROYAL_ASSASSIN), // 2ED 124
    PrintingRecord::reprint(&catalog_lea::SACRIFICE),    // 2ED 125
    PrintingRecord::reprint(&catalog_lea::SCATHE_ZOMBIES), // 2ED 126
    PrintingRecord::reprint(&catalog_lea::SCAVENGING_GHOUL), // 2ED 127
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE),     // 2ED 128
    PrintingRecord::reprint(&catalog_lea::SIMULACRUM),   // 2ED 129
    PrintingRecord::reprint(&alpha::SINKHOLE),           // 2ED 130
    PrintingRecord::reprint(&alpha::TERROR),             // 2ED 131
    PrintingRecord::reprint(&catalog_lea::UNHOLY_STRENGTH), // 2ED 132
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BONE), // 2ED 133
    PrintingRecord::reprint(&catalog_lea::WARP_ARTIFACT), // 2ED 134
    PrintingRecord::reprint(&catalog_lea::WEAKNESS),     // 2ED 135
    PrintingRecord::reprint(&catalog_lea::WILL_O_THE_WISP), // 2ED 136
    PrintingRecord::reprint(&catalog_lea::WORD_OF_COMMAND), // 2ED 137
    PrintingRecord::reprint(&catalog_lea::ZOMBIE_MASTER), // 2ED 138
    PrintingRecord::reprint(&catalog_lea::BURROWING),    // 2ED 139
    PrintingRecord::reprint(&catalog_lea::CHAOSLACE),    // 2ED 140
    PrintingRecord::reprint(&catalog_lea::DISINTEGRATE), // 2ED 141
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),       // 2ED 142
    PrintingRecord::reprint(&catalog_lea::DWARVEN_DEMOLITION_TEAM), // 2ED 143
    PrintingRecord::reprint(&catalog_lea::DWARVEN_WARRIORS), // 2ED 144
    PrintingRecord::reprint(&catalog_lea::EARTH_ELEMENTAL), // 2ED 145
    PrintingRecord::reprint(&catalog_lea::EARTHBIND),    // 2ED 146
    PrintingRecord::reprint(&alpha::EARTHQUAKE),         // 2ED 147
    PrintingRecord::reprint(&catalog_lea::FALSE_ORDERS), // 2ED 148
    PrintingRecord::reprint(&catalog_lea::FIRE_ELEMENTAL), // 2ED 149
    PrintingRecord::reprint(&alpha::FIREBALL),           // 2ED 150
    PrintingRecord::reprint(&catalog_lea::FIREBREATHING), // 2ED 151
    PrintingRecord::reprint(&catalog_lea::FLASHFIRES),   // 2ED 152
    PrintingRecord::reprint(&alpha::FORK),               // 2ED 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // 2ED 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),        // 2ED 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE),   // 2ED 156
    PrintingRecord::reprint(&catalog_lea::GRAY_OGRE),    // 2ED 157
    PrintingRecord::reprint(&catalog_lea::HILL_GIANT),   // 2ED 158
    PrintingRecord::reprint(&catalog_lea::HURLOON_MINOTAUR), // 2ED 159
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),      // 2ED 160
    PrintingRecord::reprint(&catalog_lea::KELDON_WARLORD), // 2ED 161
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT),     // 2ED 162
    PrintingRecord::reprint(&catalog_lea::MANA_FLARE),   // 2ED 163
    PrintingRecord::reprint(&catalog_lea::MANABARBS),    // 2ED 164
    PrintingRecord::reprint(&catalog_lea::MONSS_GOBLIN_RAIDERS), // 2ED 165
    PrintingRecord::reprint(&catalog_lea::ORCISH_ARTILLERY), // 2ED 166
    PrintingRecord::reprint(&catalog_lea::ORCISH_ORIFLAMME), // 2ED 167
    PrintingRecord::reprint(&catalog_lea::POWER_SURGE),  // 2ED 168
    PrintingRecord::reprint(&catalog_lea::RAGING_RIVER), // 2ED 169
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // 2ED 170
    PrintingRecord::reprint(&catalog_lea::ROC_OF_KHER_RIDGES), // 2ED 171
    PrintingRecord::reprint(&catalog_lea::ROCK_HYDRA),   // 2ED 172
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),        // 2ED 173
    PrintingRecord::reprint(&alpha::SHATTER),            // 2ED 174
    PrintingRecord::reprint(&catalog_lea::SHIVAN_DRAGON), // 2ED 175
    PrintingRecord::reprint(&alpha::SMOKE),              // 2ED 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),        // 2ED 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),         // 2ED 178
    PrintingRecord::reprint(&catalog_lea::TUNNEL),       // 2ED 179
    PrintingRecord::reprint(&catalog_lea::TWO_HEADED_GIANT_OF_FORIYS), // 2ED 180
    PrintingRecord::reprint(&catalog_lea::UTHDEN_TROLL), // 2ED 181
    PrintingRecord::reprint(&catalog_lea::WALL_OF_FIRE), // 2ED 182
    PrintingRecord::reprint(&catalog_lea::WALL_OF_STONE), // 2ED 183
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE),   // 2ED 184
    PrintingRecord::reprint(&catalog_lea::ASPECT_OF_WOLF), // 2ED 185
    PrintingRecord::reprint(&alpha::BERSERK),            // 2ED 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE),  // 2ED 187
    PrintingRecord::reprint(&catalog_lea::CAMOUFLAGE),   // 2ED 188
    PrintingRecord::reprint(&alpha::CHANNEL),            // 2ED 189
    PrintingRecord::reprint(&catalog_lea::COCKATRICE),   // 2ED 190
    PrintingRecord::reprint(&catalog_lea::CRAW_WURM),    // 2ED 191
    PrintingRecord::reprint(&catalog_lea::ELVISH_ARCHERS), // 2ED 192
    PrintingRecord::reprint(&catalog_lea::FASTBOND),     // 2ED 193
    PrintingRecord::reprint(&catalog_lea::FOG),          // 2ED 194
    PrintingRecord::reprint(&catalog_lea::FORCE_OF_NATURE), // 2ED 195
    PrintingRecord::reprint(&catalog_lea::FUNGUSAUR),    // 2ED 196
    PrintingRecord::reprint(&catalog_lea::GAEA_S_LIEGE), // 2ED 197
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),       // 2ED 198
    PrintingRecord::reprint(&catalog_lea::GIANT_SPIDER), // 2ED 199
    PrintingRecord::reprint(&catalog_lea::GRIZZLY_BEARS), // 2ED 200
    PrintingRecord::reprint(&catalog_lea::HURRICANE),    // 2ED 201
    PrintingRecord::reprint(&catalog_lea::ICE_STORM),    // 2ED 202
    PrintingRecord::reprint(&catalog_lea::INSTILL_ENERGY), // 2ED 203
    PrintingRecord::reprint(&catalog_lea::IRONROOT_TREEFOLK), // 2ED 204
    PrintingRecord::reprint(&catalog_lea::KUDZU),        // 2ED 205
    PrintingRecord::reprint(&catalog_lea::LEY_DRUID),    // 2ED 206
    PrintingRecord::reprint(&catalog_lea::LIFEFORCE),    // 2ED 207
    PrintingRecord::reprint(&catalog_lea::LIFELACE),     // 2ED 208
    PrintingRecord::reprint(&catalog_lea::LIVING_ARTIFACT), // 2ED 209
    PrintingRecord::reprint(&catalog_lea::LIVING_LANDS), // 2ED 210
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES),     // 2ED 211
    PrintingRecord::reprint(&catalog_lea::LURE),         // 2ED 212
    PrintingRecord::reprint(&catalog_lea::NATURAL_SELECTION), // 2ED 213
    PrintingRecord::reprint(&catalog_lea::REGENERATION), // 2ED 214
    PrintingRecord::reprint(&alpha::REGROWTH),           // 2ED 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),      // 2ED 216
    PrintingRecord::reprint(&catalog_lea::SHANODIN_DRYADS), // 2ED 217
    PrintingRecord::reprint(&catalog_lea::STREAM_OF_LIFE), // 2ED 218
    PrintingRecord::reprint(&catalog_lea::THICKET_BASILISK), // 2ED 219
    PrintingRecord::reprint(&catalog_lea::TIMBER_WOLVES), // 2ED 220
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),  // 2ED 221
    PrintingRecord::reprint(&catalog_lea::TSUNAMI),      // 2ED 222
    PrintingRecord::reprint(&catalog_lea::VERDURAN_ENCHANTRESS), // 2ED 223
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BRAMBLES), // 2ED 224
    PrintingRecord::reprint(&catalog_lea::WALL_OF_ICE),  // 2ED 225
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WOOD), // 2ED 226
    PrintingRecord::reprint(&catalog_lea::WANDERLUST),   // 2ED 227
    PrintingRecord::reprint(&catalog_lea::WAR_MAMMOTH),  // 2ED 228
    PrintingRecord::reprint(&catalog_lea::WEB),          // 2ED 229
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),  // 2ED 230
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA),     // 2ED 231
    PrintingRecord::reprint(&catalog_lea::BASALT_MONOLITH), // 2ED 232
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),        // 2ED 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),         // 2ED 234
    PrintingRecord::reprint(&catalog_lea::CELESTIAL_PRISM), // 2ED 235
    PrintingRecord::reprint(&alpha::CHAOS_ORB),          // 2ED 236
    PrintingRecord::reprint(&catalog_lea::CLOCKWORK_BEAST), // 2ED 237
    PrintingRecord::reprint(&catalog_lea::CONSERVATOR),  // 2ED 238
    PrintingRecord::reprint(&alpha::COPPER_TABLET),      // 2ED 239
    PrintingRecord::reprint(&catalog_lea::CRYSTAL_ROD),  // 2ED 240
    PrintingRecord::reprint(&catalog_lea::CYCLOPEAN_TOMB), // 2ED 241
    PrintingRecord::reprint(&catalog_lea::DINGUS_EGG),   // 2ED 242
    PrintingRecord::reprint(&catalog_lea::DISRUPTING_SCEPTER), // 2ED 243
    PrintingRecord::reprint(&catalog_lea::FORCEFIELD),   // 2ED 244
    PrintingRecord::reprint(&catalog_lea::GAUNTLET_OF_MIGHT), // 2ED 245
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA),    // 2ED 246
    PrintingRecord::reprint(&catalog_lea::HELM_OF_CHATZUK), // 2ED 247
    PrintingRecord::reprint(&catalog_lea::HOWLING_MINE), // 2ED 248
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR),    // 2ED 249
    PrintingRecord::reprint(&catalog_lea::ILLUSIONARY_MASK), // 2ED 250
    PrintingRecord::reprint(&alpha::IRON_STAR),          // 2ED 251
    PrintingRecord::reprint(&catalog_lea::IVORY_CUP),    // 2ED 252
    PrintingRecord::reprint(&catalog_lea::JADE_MONOLITH), // 2ED 253
    PrintingRecord::reprint(&catalog_lea::JADE_STATUE),  // 2ED 254
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),      // 2ED 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),         // 2ED 256
    PrintingRecord::reprint(&catalog_lea::KORMUS_BELL),  // 2ED 257
    PrintingRecord::reprint(&catalog_lea::LIBRARY_OF_LENG), // 2ED 258
    PrintingRecord::reprint(&catalog_lea::LIVING_WALL),  // 2ED 259
    PrintingRecord::reprint(&alpha::MANA_VAULT),         // 2ED 260
    PrintingRecord::reprint(&catalog_lea::MEEKSTONE),    // 2ED 261
    PrintingRecord::reprint(&alpha::MOX_EMERALD),        // 2ED 262
    PrintingRecord::reprint(&alpha::MOX_JET),            // 2ED 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),          // 2ED 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),           // 2ED 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),       // 2ED 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK),   // 2ED 267
    PrintingRecord::reprint(&catalog_lea::OBSIANUS_GOLEM), // 2ED 268
    PrintingRecord::reprint(&catalog_lea::ROD_OF_RUIN),  // 2ED 269
    PrintingRecord::reprint(&alpha::SOL_RING),           // 2ED 270
    PrintingRecord::reprint(&catalog_lea::SOUL_NET),     // 2ED 271
    PrintingRecord::reprint(&catalog_lea::SUNGLASSES_OF_URZA), // 2ED 272
    PrintingRecord::reprint(&catalog_lea::THE_HIVE),     // 2ED 273
    PrintingRecord::reprint(&catalog_lea::THRONE_OF_BONE), // 2ED 274
    PrintingRecord::reprint(&alpha::TIME_VAULT),         // 2ED 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),         // 2ED 276
    PrintingRecord::reprint(&catalog_lea::WOODEN_SPHERE), // 2ED 277
    PrintingRecord::reprint(&alpha::BADLANDS),           // 2ED 278
    PrintingRecord::reprint(&alpha::BAYOU),              // 2ED 279
    PrintingRecord::reprint(&alpha::PLATEAU),            // 2ED 280
    PrintingRecord::reprint(&alpha::SAVANNAH),           // 2ED 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),          // 2ED 282
    PrintingRecord::reprint(&alpha::TAIGA),              // 2ED 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND),    // 2ED 284
    PrintingRecord::reprint(&alpha::TUNDRA),             // 2ED 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA),    // 2ED 286
    PrintingRecord::reprint(&beta::VOLCANIC_ISLAND),     // 2ED 287
    PrintingRecord::reprint(&alpha::PLAINS),             // 2ED 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),        // 2ED 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),        // 2ED 290
    PrintingRecord::reprint(&alpha::ISLAND),             // 2ED 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),        // 2ED 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),        // 2ED 293
    PrintingRecord::reprint(&alpha::SWAMP),              // 2ED 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),         // 2ED 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),         // 2ED 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),           // 2ED 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),      // 2ED 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),      // 2ED 299
    PrintingRecord::reprint(&alpha::FOREST),             // 2ED 300
    PrintingRecord::alternate(&alpha::FOREST, 1),        // 2ED 301
    PrintingRecord::alternate(&alpha::FOREST, 2),        // 2ED 302
];
