//! Collector's Edition has no unique card definitions.
//!
//! Every card in the built-in Collector's Edition catalog points to its first printing.

use super::{CardRecord, PrintingRecord, alpha, beta};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;

// CED 1 — Animate Wall (reprint)

// CED 2 — Armageddon (reprint)

// CED 3 — Balance (reprint)

// CED 4 — Benalish Hero (reprint)

// CED 5 — Black Ward (reprint)

// CED 6 — Blaze of Glory (reprint)

// CED 7 — Blessing (reprint)

// CED 8 — Blue Ward (reprint)

// CED 9 — Castle (reprint)

// CED 10 — Circle of Protection: Black (reprint)

// CED 11 — Circle of Protection: Blue (reprint)

// CED 12 — Circle of Protection: Green (reprint)

// CED 13 — Circle of Protection: Red (reprint)

// CED 14 — Circle of Protection: White (reprint)

// CED 15 — Consecrate Land (reprint)

// CED 16 — Conversion (reprint)

// CED 17 — Crusade (reprint)

// CED 18 — Death Ward (reprint)

// CED 19 — Disenchant (reprint)

// CED 20 — Farmstead (reprint)

// CED 21 — Green Ward (reprint)

// CED 22 — Guardian Angel (reprint)

// CED 23 — Healing Salve (reprint)

// CED 24 — Holy Armor (reprint)

// CED 25 — Holy Strength (reprint)

// CED 26 — Island Sanctuary (reprint)

// CED 27 — Karma (reprint)

// CED 28 — Lance (reprint)

// CED 29 — Mesa Pegasus (reprint)

// CED 30 — Northern Paladin (reprint)

// CED 31 — Pearled Unicorn (reprint)

// CED 32 — Personal Incarnation (reprint)

// CED 33 — Purelace (reprint)

// CED 34 — Red Ward (reprint)

// CED 35 — Resurrection (reprint)

// CED 36 — Reverse Damage (reprint)

// CED 37 — Righteousness (reprint)

// CED 38 — Samite Healer (reprint)

// CED 39 — Savannah Lions (reprint)

// CED 40 — Serra Angel (reprint)

// CED 41 — Swords to Plowshares (reprint)

// CED 42 — Veteran Bodyguard (reprint)

// CED 43 — Wall of Swords (reprint)

// CED 44 — White Knight (reprint)

// CED 45 — White Ward (reprint)

// CED 46 — Wrath of God (reprint)

// CED 47 — Air Elemental (reprint)

// CED 48 — Ancestral Recall (reprint)

// CED 49 — Animate Artifact (reprint)

// CED 50 — Blue Elemental Blast (reprint)

// CED 51 — Braingeyser (reprint)

// CED 52 — Clone (reprint)

// CED 53 — Control Magic (reprint)

// CED 54 — Copy Artifact (reprint)

// CED 55 — Counterspell (reprint)

// CED 56 — Creature Bond (reprint)

// CED 57 — Drain Power (reprint)

// CED 58 — Feedback (reprint)

// CED 59 — Flight (reprint)

// CED 60 — Invisibility (reprint)

// CED 61 — Jump (reprint)

// CED 62 — Lifetap (reprint)

// CED 63 — Lord of Atlantis (reprint)

// CED 64 — Magical Hack (reprint)

// CED 65 — Mahamoti Djinn (reprint)

// CED 66 — Mana Short (reprint)

// CED 67 — Merfolk of the Pearl Trident (reprint)

// CED 68 — Phantasmal Forces (reprint)

// CED 69 — Phantasmal Terrain (reprint)

// CED 70 — Phantom Monster (reprint)

// CED 71 — Pirate Ship (reprint)

// CED 72 — Power Leak (reprint)

// CED 73 — Power Sink (reprint)

// CED 74 — Prodigal Sorcerer (reprint)

// CED 75 — Psionic Blast (reprint)

// CED 76 — Psychic Venom (reprint)

// CED 77 — Sea Serpent (reprint)

// CED 78 — Siren's Call (reprint)

// CED 79 — Sleight of Mind (reprint)

// CED 80 — Spell Blast (reprint)

// CED 81 — Stasis (reprint)

// CED 82 — Steal Artifact (reprint)

// CED 83 — Thoughtlace (reprint)

// CED 84 — Time Walk (reprint)

// CED 85 — Timetwister (reprint)

// CED 86 — Twiddle (reprint)

// CED 87 — Unsummon (reprint)

// CED 88 — Vesuvan Doppelganger (reprint)

// CED 89 — Volcanic Eruption (reprint)

// CED 90 — Wall of Air (reprint)

// CED 91 — Wall of Water (reprint)

// CED 92 — Water Elemental (reprint)

// CED 93 — Animate Dead (reprint)

// CED 94 — Bad Moon (reprint)

// CED 95 — Black Knight (reprint)

// CED 96 — Bog Wraith (reprint)

// CED 97 — Contract from Below (reprint)

// CED 98 — Cursed Land (reprint)

// CED 99 — Dark Ritual (reprint)

// CED 100 — Darkpact (reprint)

// CED 101 — Deathgrip (reprint)

// CED 102 — Deathlace (reprint)

// CED 103 — Demonic Attorney (reprint)

// CED 104 — Demonic Hordes (reprint)

// CED 105 — Demonic Tutor (reprint)

// CED 106 — Drain Life (reprint)

// CED 107 — Drudge Skeletons (reprint)

// CED 108 — Evil Presence (reprint)

// CED 109 — Fear (reprint)

// CED 110 — Frozen Shade (reprint)

// CED 111 — Gloom (reprint)

// CED 112 — Howl from Beyond (reprint)

// CED 113 — Hypnotic Specter (reprint)

// CED 114 — Lich (reprint)

// CED 115 — Lord of the Pit (reprint)

// CED 116 — Mind Twist (reprint)

// CED 117 — Nether Shadow (reprint)

// CED 118 — Nettling Imp (reprint)

// CED 119 — Nightmare (reprint)

// CED 120 — Paralyze (reprint)

// CED 121 — Pestilence (reprint)

// CED 122 — Plague Rats (reprint)

// CED 123 — Raise Dead (reprint)

// CED 124 — Royal Assassin (reprint)

// CED 125 — Sacrifice (reprint)

// CED 126 — Scathe Zombies (reprint)

// CED 127 — Scavenging Ghoul (reprint)

// CED 128 — Sengir Vampire (reprint)

// CED 129 — Simulacrum (reprint)

// CED 130 — Sinkhole (reprint)

// CED 131 — Terror (reprint)

// CED 132 — Unholy Strength (reprint)

// CED 133 — Wall of Bone (reprint)

// CED 134 — Warp Artifact (reprint)

// CED 135 — Weakness (reprint)

// CED 136 — Will-o'-the-Wisp (reprint)

// CED 137 — Word of Command (reprint)

// CED 138 — Zombie Master (reprint)

// CED 139 — Burrowing (reprint)

// CED 140 — Chaoslace (reprint)

// CED 141 — Disintegrate (reprint)

// CED 142 — Dragon Whelp (reprint)

// CED 143 — Dwarven Demolition Team (reprint)

// CED 144 — Dwarven Warriors (reprint)

// CED 145 — Earth Elemental (reprint)

// CED 146 — Earthbind (reprint)

// CED 147 — Earthquake (reprint)

// CED 148 — False Orders (reprint)

// CED 149 — Fire Elemental (reprint)

// CED 150 — Fireball (reprint)

// CED 151 — Firebreathing (reprint)

// CED 152 — Flashfires (reprint)

// CED 153 — Fork (reprint)

// CED 154 — Goblin Balloon Brigade (reprint)

// CED 155 — Goblin King (reprint)

// CED 156 — Granite Gargoyle (reprint)

// CED 157 — Gray Ogre (reprint)

// CED 158 — Hill Giant (reprint)

// CED 159 — Hurloon Minotaur (reprint)

// CED 160 — Ironclaw Orcs (reprint)

// CED 161 — Keldon Warlord (reprint)

// CED 162 — Lightning Bolt (reprint)

// CED 163 — Mana Flare (reprint)

// CED 164 — Manabarbs (reprint)

// CED 165 — Mons's Goblin Raiders (reprint)

// CED 166 — Orcish Artillery (reprint)

// CED 167 — Orcish Oriflamme (reprint)

// CED 168 — Power Surge (reprint)

// CED 169 — Raging River (reprint)

// CED 170 — Red Elemental Blast (reprint)

// CED 171 — Roc of Kher Ridges (reprint)

// CED 172 — Rock Hydra (reprint)

// CED 173 — Sedge Troll (reprint)

// CED 174 — Shatter (reprint)

// CED 175 — Shivan Dragon (reprint)

// CED 176 — Smoke (reprint)

// CED 177 — Stone Giant (reprint)

// CED 178 — Stone Rain (reprint)

// CED 179 — Tunnel (reprint)

// CED 180 — Two-Headed Giant of Foriys (reprint)

// CED 181 — Uthden Troll (reprint)

// CED 182 — Wall of Fire (reprint)

// CED 183 — Wall of Stone (reprint)

// CED 184 — Wheel of Fortune (reprint)

// CED 185 — Aspect of Wolf (reprint)

// CED 186 — Berserk (reprint)

// CED 187 — Birds of Paradise (reprint)

// CED 188 — Camouflage (reprint)

// CED 189 — Channel (reprint)

// CED 190 — Cockatrice (reprint)

// CED 191 — Craw Wurm (reprint)

// CED 192 — Elvish Archers (reprint)

// CED 193 — Fastbond (reprint)

// CED 194 — Fog (reprint)

// CED 195 — Force of Nature (reprint)

// CED 196 — Fungusaur (reprint)

// CED 197 — Gaea's Liege (reprint)

// CED 198 — Giant Growth (reprint)

// CED 199 — Giant Spider (reprint)

// CED 200 — Grizzly Bears (reprint)

// CED 201 — Hurricane (reprint)

// CED 202 — Ice Storm (reprint)

// CED 203 — Instill Energy (reprint)

// CED 204 — Ironroot Treefolk (reprint)

// CED 205 — Kudzu (reprint)

// CED 206 — Ley Druid (reprint)

// CED 207 — Lifeforce (reprint)

// CED 208 — Lifelace (reprint)

// CED 209 — Living Artifact (reprint)

// CED 210 — Living Lands (reprint)

// CED 211 — Llanowar Elves (reprint)

// CED 212 — Lure (reprint)

// CED 213 — Natural Selection (reprint)

// CED 214 — Regeneration (reprint)

// CED 215 — Regrowth (reprint)

// CED 216 — Scryb Sprites (reprint)

// CED 217 — Shanodin Dryads (reprint)

// CED 218 — Stream of Life (reprint)

// CED 219 — Thicket Basilisk (reprint)

// CED 220 — Timber Wolves (reprint)

// CED 221 — Tranquility (reprint)

// CED 222 — Tsunami (reprint)

// CED 223 — Verduran Enchantress (reprint)

// CED 224 — Wall of Brambles (reprint)

// CED 225 — Wall of Ice (reprint)

// CED 226 — Wall of Wood (reprint)

// CED 227 — Wanderlust (reprint)

// CED 228 — War Mammoth (reprint)

// CED 229 — Web (reprint)

// CED 230 — Wild Growth (reprint)

// CED 231 — Ankh of Mishra (reprint)

// CED 232 — Basalt Monolith (reprint)

// CED 233 — Black Lotus (reprint)

// CED 234 — Black Vise (reprint)

// CED 235 — Celestial Prism (reprint)

// CED 236 — Chaos Orb (reprint)

// CED 237 — Clockwork Beast (reprint)

// CED 238 — Conservator (reprint)

// CED 239 — Copper Tablet (reprint)

// CED 240 — Crystal Rod (reprint)

// CED 241 — Cyclopean Tomb (reprint)

// CED 242 — Dingus Egg (reprint)

// CED 243 — Disrupting Scepter (reprint)

// CED 244 — Forcefield (reprint)

// CED 245 — Gauntlet of Might (reprint)

// CED 246 — Glasses of Urza (reprint)

// CED 247 — Helm of Chatzuk (reprint)

// CED 248 — Howling Mine (reprint)

// CED 249 — Icy Manipulator (reprint)

// CED 250 — Illusionary Mask (reprint)

// CED 251 — Iron Star (reprint)

// CED 252 — Ivory Cup (reprint)

// CED 253 — Jade Monolith (reprint)

// CED 254 — Jade Statue (reprint)

// CED 255 — Jayemdae Tome (reprint)

// CED 256 — Juggernaut (reprint)

// CED 257 — Kormus Bell (reprint)

// CED 258 — Library of Leng (reprint)

// CED 259 — Living Wall (reprint)

// CED 260 — Mana Vault (reprint)

// CED 261 — Meekstone (reprint)

// CED 262 — Mox Emerald (reprint)

// CED 263 — Mox Jet (reprint)

// CED 264 — Mox Pearl (reprint)

// CED 265 — Mox Ruby (reprint)

// CED 266 — Mox Sapphire (reprint)

// CED 267 — Nevinyrral's Disk (reprint)

// CED 268 — Obsianus Golem (reprint)

// CED 269 — Rod of Ruin (reprint)

// CED 270 — Sol Ring (reprint)

// CED 271 — Soul Net (reprint)

// CED 272 — Sunglasses of Urza (reprint)

// CED 273 — The Hive (reprint)

// CED 274 — Throne of Bone (reprint)

// CED 275 — Time Vault (reprint)

// CED 276 — Winter Orb (reprint)

// CED 277 — Wooden Sphere (reprint)

// CED 278 — Badlands (reprint)

// CED 279 — Bayou (reprint)

// CED 280 — Plateau (reprint)

// CED 281 — Savannah (reprint)

// CED 282 — Scrubland (reprint)

// CED 283 — Taiga (reprint)

// CED 284 — Tropical Island (reprint)

// CED 285 — Tundra (reprint)

// CED 286 — Underground Sea (reprint)

// CED 287 — Volcanic Island (reprint)

// CED 288 — Plains (reprint)

// CED 289 — Plains (alternate printing)

// CED 290 — Plains (alternate printing)

// CED 291 — Island (reprint)

// CED 292 — Island (alternate printing)

// CED 293 — Island (alternate printing)

// CED 294 — Swamp (reprint)

// CED 295 — Swamp (alternate printing)

// CED 296 — Swamp (alternate printing)

// CED 297 — Mountain (reprint)

// CED 298 — Mountain (alternate printing)

// CED 299 — Mountain (alternate printing)

// CED 300 — Forest (reprint)

// CED 301 — Forest (alternate printing)

// CED 302 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_lea::ANIMATE_WALL), // CED 1
    PrintingRecord::reprint(&alpha::ARMAGEDDON),         // CED 2
    PrintingRecord::reprint(&alpha::BALANCE),            // CED 3
    PrintingRecord::reprint(&catalog_lea::BENALISH_HERO), // CED 4
    PrintingRecord::reprint(&catalog_lea::BLACK_WARD),   // CED 5
    PrintingRecord::reprint(&catalog_lea::BLAZE_OF_GLORY), // CED 6
    PrintingRecord::reprint(&catalog_lea::BLESSING),     // CED 7
    PrintingRecord::reprint(&catalog_lea::BLUE_WARD),    // CED 8
    PrintingRecord::reprint(&catalog_lea::CASTLE),       // CED 9
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // CED 10
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE), // CED 11
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // CED 12
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED), // CED 13
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // CED 14
    PrintingRecord::reprint(&catalog_lea::CONSECRATE_LAND), // CED 15
    PrintingRecord::reprint(&catalog_lea::CONVERSION),   // CED 16
    PrintingRecord::reprint(&alpha::CRUSADE),            // CED 17
    PrintingRecord::reprint(&catalog_lea::DEATH_WARD),   // CED 18
    PrintingRecord::reprint(&alpha::DISENCHANT),         // CED 19
    PrintingRecord::reprint(&catalog_lea::FARMSTEAD),    // CED 20
    PrintingRecord::reprint(&catalog_lea::GREEN_WARD),   // CED 21
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL),     // CED 22
    PrintingRecord::reprint(&catalog_lea::HEALING_SALVE), // CED 23
    PrintingRecord::reprint(&catalog_lea::HOLY_ARMOR),   // CED 24
    PrintingRecord::reprint(&catalog_lea::HOLY_STRENGTH), // CED 25
    PrintingRecord::reprint(&catalog_lea::ISLAND_SANCTUARY), // CED 26
    PrintingRecord::reprint(&catalog_lea::KARMA),        // CED 27
    PrintingRecord::reprint(&catalog_lea::LANCE),        // CED 28
    PrintingRecord::reprint(&catalog_lea::MESA_PEGASUS), // CED 29
    PrintingRecord::reprint(&catalog_lea::NORTHERN_PALADIN), // CED 30
    PrintingRecord::reprint(&catalog_lea::PEARLED_UNICORN), // CED 31
    PrintingRecord::reprint(&catalog_lea::PERSONAL_INCARNATION), // CED 32
    PrintingRecord::reprint(&catalog_lea::PURELACE),     // CED 33
    PrintingRecord::reprint(&catalog_lea::RED_WARD),     // CED 34
    PrintingRecord::reprint(&catalog_lea::RESURRECTION), // CED 35
    PrintingRecord::reprint(&catalog_lea::REVERSE_DAMAGE), // CED 36
    PrintingRecord::reprint(&catalog_lea::RIGHTEOUSNESS), // CED 37
    PrintingRecord::reprint(&catalog_lea::SAMITE_HEALER), // CED 38
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS),     // CED 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),        // CED 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // CED 41
    PrintingRecord::reprint(&catalog_lea::VETERAN_BODYGUARD), // CED 42
    PrintingRecord::reprint(&catalog_lea::WALL_OF_SWORDS), // CED 43
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),       // CED 44
    PrintingRecord::reprint(&catalog_lea::WHITE_WARD),   // CED 45
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),       // CED 46
    PrintingRecord::reprint(&catalog_lea::AIR_ELEMENTAL), // CED 47
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL),   // CED 48
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT),   // CED 49
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // CED 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),        // CED 51
    PrintingRecord::reprint(&catalog_lea::CLONE),        // CED 52
    PrintingRecord::reprint(&catalog_lea::CONTROL_MAGIC), // CED 53
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),      // CED 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),       // CED 55
    PrintingRecord::reprint(&catalog_lea::CREATURE_BOND), // CED 56
    PrintingRecord::reprint(&catalog_lea::DRAIN_POWER),  // CED 57
    PrintingRecord::reprint(&catalog_lea::FEEDBACK),     // CED 58
    PrintingRecord::reprint(&catalog_lea::FLIGHT),       // CED 59
    PrintingRecord::reprint(&catalog_lea::INVISIBILITY), // CED 60
    PrintingRecord::reprint(&catalog_lea::JUMP),         // CED 61
    PrintingRecord::reprint(&catalog_lea::LIFETAP),      // CED 62
    PrintingRecord::reprint(&catalog_lea::LORD_OF_ATLANTIS), // CED 63
    PrintingRecord::reprint(&catalog_lea::MAGICAL_HACK), // CED 64
    PrintingRecord::reprint(&catalog_lea::MAHAMOTI_DJINN), // CED 65
    PrintingRecord::reprint(&alpha::MANA_SHORT),         // CED 66
    PrintingRecord::reprint(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT), // CED 67
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_FORCES), // CED 68
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_TERRAIN), // CED 69
    PrintingRecord::reprint(&catalog_lea::PHANTOM_MONSTER), // CED 70
    PrintingRecord::reprint(&catalog_lea::PIRATE_SHIP),  // CED 71
    PrintingRecord::reprint(&catalog_lea::POWER_LEAK),   // CED 72
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),   // CED 73
    PrintingRecord::reprint(&catalog_lea::PRODIGAL_SORCERER), // CED 74
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),      // CED 75
    PrintingRecord::reprint(&catalog_lea::PSYCHIC_VENOM), // CED 76
    PrintingRecord::reprint(&catalog_lea::SEA_SERPENT),  // CED 77
    PrintingRecord::reprint(&catalog_lea::SIREN_S_CALL), // CED 78
    PrintingRecord::reprint(&catalog_lea::SLEIGHT_OF_MIND), // CED 79
    PrintingRecord::reprint(&catalog_lea::SPELL_BLAST),  // CED 80
    PrintingRecord::reprint(&alpha::STASIS),             // CED 81
    PrintingRecord::reprint(&catalog_lea::STEAL_ARTIFACT), // CED 82
    PrintingRecord::reprint(&catalog_lea::THOUGHTLACE),  // CED 83
    PrintingRecord::reprint(&alpha::TIME_WALK),          // CED 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),        // CED 85
    PrintingRecord::reprint(&catalog_lea::TWIDDLE),      // CED 86
    PrintingRecord::reprint(&catalog_lea::UNSUMMON),     // CED 87
    PrintingRecord::reprint(&catalog_lea::VESUVAN_DOPPELGANGER), // CED 88
    PrintingRecord::reprint(&catalog_lea::VOLCANIC_ERUPTION), // CED 89
    PrintingRecord::reprint(&catalog_lea::WALL_OF_AIR),  // CED 90
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WATER), // CED 91
    PrintingRecord::reprint(&catalog_lea::WATER_ELEMENTAL), // CED 92
    PrintingRecord::reprint(&catalog_lea::ANIMATE_DEAD), // CED 93
    PrintingRecord::reprint(&catalog_lea::BAD_MOON),     // CED 94
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),       // CED 95
    PrintingRecord::reprint(&catalog_lea::BOG_WRAITH),   // CED 96
    PrintingRecord::reprint(&catalog_lea::CONTRACT_FROM_BELOW), // CED 97
    PrintingRecord::reprint(&catalog_lea::CURSED_LAND),  // CED 98
    PrintingRecord::reprint(&alpha::DARK_RITUAL),        // CED 99
    PrintingRecord::reprint(&catalog_lea::DARKPACT),     // CED 100
    PrintingRecord::reprint(&catalog_lea::DEATHGRIP),    // CED 101
    PrintingRecord::reprint(&catalog_lea::DEATHLACE),    // CED 102
    PrintingRecord::reprint(&catalog_lea::DEMONIC_ATTORNEY), // CED 103
    PrintingRecord::reprint(&catalog_lea::DEMONIC_HORDES), // CED 104
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),      // CED 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),         // CED 106
    PrintingRecord::reprint(&catalog_lea::DRUDGE_SKELETONS), // CED 107
    PrintingRecord::reprint(&catalog_lea::EVIL_PRESENCE), // CED 108
    PrintingRecord::reprint(&catalog_lea::FEAR),         // CED 109
    PrintingRecord::reprint(&catalog_lea::FROZEN_SHADE), // CED 110
    PrintingRecord::reprint(&catalog_lea::GLOOM),        // CED 111
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND), // CED 112
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER),   // CED 113
    PrintingRecord::reprint(&catalog_lea::LICH),         // CED 114
    PrintingRecord::reprint(&catalog_lea::LORD_OF_THE_PIT), // CED 115
    PrintingRecord::reprint(&alpha::MIND_TWIST),         // CED 116
    PrintingRecord::reprint(&catalog_lea::NETHER_SHADOW), // CED 117
    PrintingRecord::reprint(&catalog_lea::NETTLING_IMP), // CED 118
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),    // CED 119
    PrintingRecord::reprint(&catalog_lea::PARALYZE),     // CED 120
    PrintingRecord::reprint(&catalog_lea::PESTILENCE),   // CED 121
    PrintingRecord::reprint(&catalog_lea::PLAGUE_RATS),  // CED 122
    PrintingRecord::reprint(&catalog_lea::RAISE_DEAD),   // CED 123
    PrintingRecord::reprint(&catalog_lea::ROYAL_ASSASSIN), // CED 124
    PrintingRecord::reprint(&catalog_lea::SACRIFICE),    // CED 125
    PrintingRecord::reprint(&catalog_lea::SCATHE_ZOMBIES), // CED 126
    PrintingRecord::reprint(&catalog_lea::SCAVENGING_GHOUL), // CED 127
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE),     // CED 128
    PrintingRecord::reprint(&catalog_lea::SIMULACRUM),   // CED 129
    PrintingRecord::reprint(&alpha::SINKHOLE),           // CED 130
    PrintingRecord::reprint(&alpha::TERROR),             // CED 131
    PrintingRecord::reprint(&catalog_lea::UNHOLY_STRENGTH), // CED 132
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BONE), // CED 133
    PrintingRecord::reprint(&catalog_lea::WARP_ARTIFACT), // CED 134
    PrintingRecord::reprint(&catalog_lea::WEAKNESS),     // CED 135
    PrintingRecord::reprint(&catalog_lea::WILL_O_THE_WISP), // CED 136
    PrintingRecord::reprint(&catalog_lea::WORD_OF_COMMAND), // CED 137
    PrintingRecord::reprint(&catalog_lea::ZOMBIE_MASTER), // CED 138
    PrintingRecord::reprint(&catalog_lea::BURROWING),    // CED 139
    PrintingRecord::reprint(&catalog_lea::CHAOSLACE),    // CED 140
    PrintingRecord::reprint(&catalog_lea::DISINTEGRATE), // CED 141
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),       // CED 142
    PrintingRecord::reprint(&catalog_lea::DWARVEN_DEMOLITION_TEAM), // CED 143
    PrintingRecord::reprint(&catalog_lea::DWARVEN_WARRIORS), // CED 144
    PrintingRecord::reprint(&catalog_lea::EARTH_ELEMENTAL), // CED 145
    PrintingRecord::reprint(&catalog_lea::EARTHBIND),    // CED 146
    PrintingRecord::reprint(&alpha::EARTHQUAKE),         // CED 147
    PrintingRecord::reprint(&catalog_lea::FALSE_ORDERS), // CED 148
    PrintingRecord::reprint(&catalog_lea::FIRE_ELEMENTAL), // CED 149
    PrintingRecord::reprint(&alpha::FIREBALL),           // CED 150
    PrintingRecord::reprint(&catalog_lea::FIREBREATHING), // CED 151
    PrintingRecord::reprint(&catalog_lea::FLASHFIRES),   // CED 152
    PrintingRecord::reprint(&alpha::FORK),               // CED 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // CED 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),        // CED 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE),   // CED 156
    PrintingRecord::reprint(&catalog_lea::GRAY_OGRE),    // CED 157
    PrintingRecord::reprint(&catalog_lea::HILL_GIANT),   // CED 158
    PrintingRecord::reprint(&catalog_lea::HURLOON_MINOTAUR), // CED 159
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),      // CED 160
    PrintingRecord::reprint(&catalog_lea::KELDON_WARLORD), // CED 161
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT),     // CED 162
    PrintingRecord::reprint(&catalog_lea::MANA_FLARE),   // CED 163
    PrintingRecord::reprint(&catalog_lea::MANABARBS),    // CED 164
    PrintingRecord::reprint(&catalog_lea::MONSS_GOBLIN_RAIDERS), // CED 165
    PrintingRecord::reprint(&catalog_lea::ORCISH_ARTILLERY), // CED 166
    PrintingRecord::reprint(&catalog_lea::ORCISH_ORIFLAMME), // CED 167
    PrintingRecord::reprint(&catalog_lea::POWER_SURGE),  // CED 168
    PrintingRecord::reprint(&catalog_lea::RAGING_RIVER), // CED 169
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // CED 170
    PrintingRecord::reprint(&catalog_lea::ROC_OF_KHER_RIDGES), // CED 171
    PrintingRecord::reprint(&catalog_lea::ROCK_HYDRA),   // CED 172
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),        // CED 173
    PrintingRecord::reprint(&alpha::SHATTER),            // CED 174
    PrintingRecord::reprint(&catalog_lea::SHIVAN_DRAGON), // CED 175
    PrintingRecord::reprint(&alpha::SMOKE),              // CED 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),        // CED 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),         // CED 178
    PrintingRecord::reprint(&catalog_lea::TUNNEL),       // CED 179
    PrintingRecord::reprint(&catalog_lea::TWO_HEADED_GIANT_OF_FORIYS), // CED 180
    PrintingRecord::reprint(&catalog_lea::UTHDEN_TROLL), // CED 181
    PrintingRecord::reprint(&catalog_lea::WALL_OF_FIRE), // CED 182
    PrintingRecord::reprint(&catalog_lea::WALL_OF_STONE), // CED 183
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE),   // CED 184
    PrintingRecord::reprint(&catalog_lea::ASPECT_OF_WOLF), // CED 185
    PrintingRecord::reprint(&alpha::BERSERK),            // CED 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE),  // CED 187
    PrintingRecord::reprint(&catalog_lea::CAMOUFLAGE),   // CED 188
    PrintingRecord::reprint(&alpha::CHANNEL),            // CED 189
    PrintingRecord::reprint(&catalog_lea::COCKATRICE),   // CED 190
    PrintingRecord::reprint(&catalog_lea::CRAW_WURM),    // CED 191
    PrintingRecord::reprint(&catalog_lea::ELVISH_ARCHERS), // CED 192
    PrintingRecord::reprint(&catalog_lea::FASTBOND),     // CED 193
    PrintingRecord::reprint(&catalog_lea::FOG),          // CED 194
    PrintingRecord::reprint(&catalog_lea::FORCE_OF_NATURE), // CED 195
    PrintingRecord::reprint(&catalog_lea::FUNGUSAUR),    // CED 196
    PrintingRecord::reprint(&catalog_lea::GAEA_S_LIEGE), // CED 197
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),       // CED 198
    PrintingRecord::reprint(&catalog_lea::GIANT_SPIDER), // CED 199
    PrintingRecord::reprint(&catalog_lea::GRIZZLY_BEARS), // CED 200
    PrintingRecord::reprint(&catalog_lea::HURRICANE),    // CED 201
    PrintingRecord::reprint(&catalog_lea::ICE_STORM),    // CED 202
    PrintingRecord::reprint(&catalog_lea::INSTILL_ENERGY), // CED 203
    PrintingRecord::reprint(&catalog_lea::IRONROOT_TREEFOLK), // CED 204
    PrintingRecord::reprint(&catalog_lea::KUDZU),        // CED 205
    PrintingRecord::reprint(&catalog_lea::LEY_DRUID),    // CED 206
    PrintingRecord::reprint(&catalog_lea::LIFEFORCE),    // CED 207
    PrintingRecord::reprint(&catalog_lea::LIFELACE),     // CED 208
    PrintingRecord::reprint(&catalog_lea::LIVING_ARTIFACT), // CED 209
    PrintingRecord::reprint(&catalog_lea::LIVING_LANDS), // CED 210
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES),     // CED 211
    PrintingRecord::reprint(&catalog_lea::LURE),         // CED 212
    PrintingRecord::reprint(&catalog_lea::NATURAL_SELECTION), // CED 213
    PrintingRecord::reprint(&catalog_lea::REGENERATION), // CED 214
    PrintingRecord::reprint(&alpha::REGROWTH),           // CED 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),      // CED 216
    PrintingRecord::reprint(&catalog_lea::SHANODIN_DRYADS), // CED 217
    PrintingRecord::reprint(&catalog_lea::STREAM_OF_LIFE), // CED 218
    PrintingRecord::reprint(&catalog_lea::THICKET_BASILISK), // CED 219
    PrintingRecord::reprint(&catalog_lea::TIMBER_WOLVES), // CED 220
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),  // CED 221
    PrintingRecord::reprint(&catalog_lea::TSUNAMI),      // CED 222
    PrintingRecord::reprint(&catalog_lea::VERDURAN_ENCHANTRESS), // CED 223
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BRAMBLES), // CED 224
    PrintingRecord::reprint(&catalog_lea::WALL_OF_ICE),  // CED 225
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WOOD), // CED 226
    PrintingRecord::reprint(&catalog_lea::WANDERLUST),   // CED 227
    PrintingRecord::reprint(&catalog_lea::WAR_MAMMOTH),  // CED 228
    PrintingRecord::reprint(&catalog_lea::WEB),          // CED 229
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),  // CED 230
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA),     // CED 231
    PrintingRecord::reprint(&catalog_lea::BASALT_MONOLITH), // CED 232
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),        // CED 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),         // CED 234
    PrintingRecord::reprint(&catalog_lea::CELESTIAL_PRISM), // CED 235
    PrintingRecord::reprint(&alpha::CHAOS_ORB),          // CED 236
    PrintingRecord::reprint(&catalog_lea::CLOCKWORK_BEAST), // CED 237
    PrintingRecord::reprint(&catalog_lea::CONSERVATOR),  // CED 238
    PrintingRecord::reprint(&alpha::COPPER_TABLET),      // CED 239
    PrintingRecord::reprint(&catalog_lea::CRYSTAL_ROD),  // CED 240
    PrintingRecord::reprint(&catalog_lea::CYCLOPEAN_TOMB), // CED 241
    PrintingRecord::reprint(&catalog_lea::DINGUS_EGG),   // CED 242
    PrintingRecord::reprint(&catalog_lea::DISRUPTING_SCEPTER), // CED 243
    PrintingRecord::reprint(&catalog_lea::FORCEFIELD),   // CED 244
    PrintingRecord::reprint(&catalog_lea::GAUNTLET_OF_MIGHT), // CED 245
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA),    // CED 246
    PrintingRecord::reprint(&catalog_lea::HELM_OF_CHATZUK), // CED 247
    PrintingRecord::reprint(&catalog_lea::HOWLING_MINE), // CED 248
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR),    // CED 249
    PrintingRecord::reprint(&catalog_lea::ILLUSIONARY_MASK), // CED 250
    PrintingRecord::reprint(&alpha::IRON_STAR),          // CED 251
    PrintingRecord::reprint(&catalog_lea::IVORY_CUP),    // CED 252
    PrintingRecord::reprint(&catalog_lea::JADE_MONOLITH), // CED 253
    PrintingRecord::reprint(&catalog_lea::JADE_STATUE),  // CED 254
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),      // CED 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),         // CED 256
    PrintingRecord::reprint(&catalog_lea::KORMUS_BELL),  // CED 257
    PrintingRecord::reprint(&catalog_lea::LIBRARY_OF_LENG), // CED 258
    PrintingRecord::reprint(&catalog_lea::LIVING_WALL),  // CED 259
    PrintingRecord::reprint(&alpha::MANA_VAULT),         // CED 260
    PrintingRecord::reprint(&catalog_lea::MEEKSTONE),    // CED 261
    PrintingRecord::reprint(&alpha::MOX_EMERALD),        // CED 262
    PrintingRecord::reprint(&alpha::MOX_JET),            // CED 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),          // CED 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),           // CED 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),       // CED 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK),   // CED 267
    PrintingRecord::reprint(&catalog_lea::OBSIANUS_GOLEM), // CED 268
    PrintingRecord::reprint(&catalog_lea::ROD_OF_RUIN),  // CED 269
    PrintingRecord::reprint(&alpha::SOL_RING),           // CED 270
    PrintingRecord::reprint(&catalog_lea::SOUL_NET),     // CED 271
    PrintingRecord::reprint(&catalog_lea::SUNGLASSES_OF_URZA), // CED 272
    PrintingRecord::reprint(&catalog_lea::THE_HIVE),     // CED 273
    PrintingRecord::reprint(&catalog_lea::THRONE_OF_BONE), // CED 274
    PrintingRecord::reprint(&alpha::TIME_VAULT),         // CED 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),         // CED 276
    PrintingRecord::reprint(&catalog_lea::WOODEN_SPHERE), // CED 277
    PrintingRecord::reprint(&alpha::BADLANDS),           // CED 278
    PrintingRecord::reprint(&alpha::BAYOU),              // CED 279
    PrintingRecord::reprint(&alpha::PLATEAU),            // CED 280
    PrintingRecord::reprint(&alpha::SAVANNAH),           // CED 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),          // CED 282
    PrintingRecord::reprint(&alpha::TAIGA),              // CED 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND),    // CED 284
    PrintingRecord::reprint(&alpha::TUNDRA),             // CED 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA),    // CED 286
    PrintingRecord::reprint(&beta::VOLCANIC_ISLAND),     // CED 287
    PrintingRecord::reprint(&alpha::PLAINS),             // CED 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),        // CED 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),        // CED 290
    PrintingRecord::reprint(&alpha::ISLAND),             // CED 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),        // CED 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),        // CED 293
    PrintingRecord::reprint(&alpha::SWAMP),              // CED 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),         // CED 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),         // CED 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),           // CED 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),      // CED 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),      // CED 299
    PrintingRecord::reprint(&alpha::FOREST),             // CED 300
    PrintingRecord::alternate(&alpha::FOREST, 1),        // CED 301
    PrintingRecord::alternate(&alpha::FOREST, 2),        // CED 302
];
