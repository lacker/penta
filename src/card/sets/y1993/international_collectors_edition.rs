//! International Collector's Edition has no unique card definitions.
//!
//! Every card in the built-in International Collector's Edition catalog points to its first
//! printing.

use super::{CardRecord, PrintingRecord, alpha, beta};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;

// CEI 1 — Animate Wall (reprint)

// CEI 2 — Armageddon (reprint)

// CEI 3 — Balance (reprint)

// CEI 4 — Benalish Hero (reprint)

// CEI 5 — Black Ward (reprint)

// CEI 6 — Blaze of Glory (reprint)

// CEI 7 — Blessing (reprint)

// CEI 8 — Blue Ward (reprint)

// CEI 9 — Castle (reprint)

// CEI 10 — Circle of Protection: Black (reprint)

// CEI 11 — Circle of Protection: Blue (reprint)

// CEI 12 — Circle of Protection: Green (reprint)

// CEI 13 — Circle of Protection: Red (reprint)

// CEI 14 — Circle of Protection: White (reprint)

// CEI 15 — Consecrate Land (reprint)

// CEI 16 — Conversion (reprint)

// CEI 17 — Crusade (reprint)

// CEI 18 — Death Ward (reprint)

// CEI 19 — Disenchant (reprint)

// CEI 20 — Farmstead (reprint)

// CEI 21 — Green Ward (reprint)

// CEI 22 — Guardian Angel (reprint)

// CEI 23 — Healing Salve (reprint)

// CEI 24 — Holy Armor (reprint)

// CEI 25 — Holy Strength (reprint)

// CEI 26 — Island Sanctuary (reprint)

// CEI 27 — Karma (reprint)

// CEI 28 — Lance (reprint)

// CEI 29 — Mesa Pegasus (reprint)

// CEI 30 — Northern Paladin (reprint)

// CEI 31 — Pearled Unicorn (reprint)

// CEI 32 — Personal Incarnation (reprint)

// CEI 33 — Purelace (reprint)

// CEI 34 — Red Ward (reprint)

// CEI 35 — Resurrection (reprint)

// CEI 36 — Reverse Damage (reprint)

// CEI 37 — Righteousness (reprint)

// CEI 38 — Samite Healer (reprint)

// CEI 39 — Savannah Lions (reprint)

// CEI 40 — Serra Angel (reprint)

// CEI 41 — Swords to Plowshares (reprint)

// CEI 42 — Veteran Bodyguard (reprint)

// CEI 43 — Wall of Swords (reprint)

// CEI 44 — White Knight (reprint)

// CEI 45 — White Ward (reprint)

// CEI 46 — Wrath of God (reprint)

// CEI 47 — Air Elemental (reprint)

// CEI 48 — Ancestral Recall (reprint)

// CEI 49 — Animate Artifact (reprint)

// CEI 50 — Blue Elemental Blast (reprint)

// CEI 51 — Braingeyser (reprint)

// CEI 52 — Clone (reprint)

// CEI 53 — Control Magic (reprint)

// CEI 54 — Copy Artifact (reprint)

// CEI 55 — Counterspell (reprint)

// CEI 56 — Creature Bond (reprint)

// CEI 57 — Drain Power (reprint)

// CEI 58 — Feedback (reprint)

// CEI 59 — Flight (reprint)

// CEI 60 — Invisibility (reprint)

// CEI 61 — Jump (reprint)

// CEI 62 — Lifetap (reprint)

// CEI 63 — Lord of Atlantis (reprint)

// CEI 64 — Magical Hack (reprint)

// CEI 65 — Mahamoti Djinn (reprint)

// CEI 66 — Mana Short (reprint)

// CEI 67 — Merfolk of the Pearl Trident (reprint)

// CEI 68 — Phantasmal Forces (reprint)

// CEI 69 — Phantasmal Terrain (reprint)

// CEI 70 — Phantom Monster (reprint)

// CEI 71 — Pirate Ship (reprint)

// CEI 72 — Power Leak (reprint)

// CEI 73 — Power Sink (reprint)

// CEI 74 — Prodigal Sorcerer (reprint)

// CEI 75 — Psionic Blast (reprint)

// CEI 76 — Psychic Venom (reprint)

// CEI 77 — Sea Serpent (reprint)

// CEI 78 — Siren's Call (reprint)

// CEI 79 — Sleight of Mind (reprint)

// CEI 80 — Spell Blast (reprint)

// CEI 81 — Stasis (reprint)

// CEI 82 — Steal Artifact (reprint)

// CEI 83 — Thoughtlace (reprint)

// CEI 84 — Time Walk (reprint)

// CEI 85 — Timetwister (reprint)

// CEI 86 — Twiddle (reprint)

// CEI 87 — Unsummon (reprint)

// CEI 88 — Vesuvan Doppelganger (reprint)

// CEI 89 — Volcanic Eruption (reprint)

// CEI 90 — Wall of Air (reprint)

// CEI 91 — Wall of Water (reprint)

// CEI 92 — Water Elemental (reprint)

// CEI 93 — Animate Dead (reprint)

// CEI 94 — Bad Moon (reprint)

// CEI 95 — Black Knight (reprint)

// CEI 96 — Bog Wraith (reprint)

// CEI 97 — Contract from Below (reprint)

// CEI 98 — Cursed Land (reprint)

// CEI 99 — Dark Ritual (reprint)

// CEI 100 — Darkpact (reprint)

// CEI 101 — Deathgrip (reprint)

// CEI 102 — Deathlace (reprint)

// CEI 103 — Demonic Attorney (reprint)

// CEI 104 — Demonic Hordes (reprint)

// CEI 105 — Demonic Tutor (reprint)

// CEI 106 — Drain Life (reprint)

// CEI 107 — Drudge Skeletons (reprint)

// CEI 108 — Evil Presence (reprint)

// CEI 109 — Fear (reprint)

// CEI 110 — Frozen Shade (reprint)

// CEI 111 — Gloom (reprint)

// CEI 112 — Howl from Beyond (reprint)

// CEI 113 — Hypnotic Specter (reprint)

// CEI 114 — Lich (reprint)

// CEI 115 — Lord of the Pit (reprint)

// CEI 116 — Mind Twist (reprint)

// CEI 117 — Nether Shadow (reprint)

// CEI 118 — Nettling Imp (reprint)

// CEI 119 — Nightmare (reprint)

// CEI 120 — Paralyze (reprint)

// CEI 121 — Pestilence (reprint)

// CEI 122 — Plague Rats (reprint)

// CEI 123 — Raise Dead (reprint)

// CEI 124 — Royal Assassin (reprint)

// CEI 125 — Sacrifice (reprint)

// CEI 126 — Scathe Zombies (reprint)

// CEI 127 — Scavenging Ghoul (reprint)

// CEI 128 — Sengir Vampire (reprint)

// CEI 129 — Simulacrum (reprint)

// CEI 130 — Sinkhole (reprint)

// CEI 131 — Terror (reprint)

// CEI 132 — Unholy Strength (reprint)

// CEI 133 — Wall of Bone (reprint)

// CEI 134 — Warp Artifact (reprint)

// CEI 135 — Weakness (reprint)

// CEI 136 — Will-o'-the-Wisp (reprint)

// CEI 137 — Word of Command (reprint)

// CEI 138 — Zombie Master (reprint)

// CEI 139 — Burrowing (reprint)

// CEI 140 — Chaoslace (reprint)

// CEI 141 — Disintegrate (reprint)

// CEI 142 — Dragon Whelp (reprint)

// CEI 143 — Dwarven Demolition Team (reprint)

// CEI 144 — Dwarven Warriors (reprint)

// CEI 145 — Earth Elemental (reprint)

// CEI 146 — Earthbind (reprint)

// CEI 147 — Earthquake (reprint)

// CEI 148 — False Orders (reprint)

// CEI 149 — Fire Elemental (reprint)

// CEI 150 — Fireball (reprint)

// CEI 151 — Firebreathing (reprint)

// CEI 152 — Flashfires (reprint)

// CEI 153 — Fork (reprint)

// CEI 154 — Goblin Balloon Brigade (reprint)

// CEI 155 — Goblin King (reprint)

// CEI 156 — Granite Gargoyle (reprint)

// CEI 157 — Gray Ogre (reprint)

// CEI 158 — Hill Giant (reprint)

// CEI 159 — Hurloon Minotaur (reprint)

// CEI 160 — Ironclaw Orcs (reprint)

// CEI 161 — Keldon Warlord (reprint)

// CEI 162 — Lightning Bolt (reprint)

// CEI 163 — Mana Flare (reprint)

// CEI 164 — Manabarbs (reprint)

// CEI 165 — Mons's Goblin Raiders (reprint)

// CEI 166 — Orcish Artillery (reprint)

// CEI 167 — Orcish Oriflamme (reprint)

// CEI 168 — Power Surge (reprint)

// CEI 169 — Raging River (reprint)

// CEI 170 — Red Elemental Blast (reprint)

// CEI 171 — Roc of Kher Ridges (reprint)

// CEI 172 — Rock Hydra (reprint)

// CEI 173 — Sedge Troll (reprint)

// CEI 174 — Shatter (reprint)

// CEI 175 — Shivan Dragon (reprint)

// CEI 176 — Smoke (reprint)

// CEI 177 — Stone Giant (reprint)

// CEI 178 — Stone Rain (reprint)

// CEI 179 — Tunnel (reprint)

// CEI 180 — Two-Headed Giant of Foriys (reprint)

// CEI 181 — Uthden Troll (reprint)

// CEI 182 — Wall of Fire (reprint)

// CEI 183 — Wall of Stone (reprint)

// CEI 184 — Wheel of Fortune (reprint)

// CEI 185 — Aspect of Wolf (reprint)

// CEI 186 — Berserk (reprint)

// CEI 187 — Birds of Paradise (reprint)

// CEI 188 — Camouflage (reprint)

// CEI 189 — Channel (reprint)

// CEI 190 — Cockatrice (reprint)

// CEI 191 — Craw Wurm (reprint)

// CEI 192 — Elvish Archers (reprint)

// CEI 193 — Fastbond (reprint)

// CEI 194 — Fog (reprint)

// CEI 195 — Force of Nature (reprint)

// CEI 196 — Fungusaur (reprint)

// CEI 197 — Gaea's Liege (reprint)

// CEI 198 — Giant Growth (reprint)

// CEI 199 — Giant Spider (reprint)

// CEI 200 — Grizzly Bears (reprint)

// CEI 201 — Hurricane (reprint)

// CEI 202 — Ice Storm (reprint)

// CEI 203 — Instill Energy (reprint)

// CEI 204 — Ironroot Treefolk (reprint)

// CEI 205 — Kudzu (reprint)

// CEI 206 — Ley Druid (reprint)

// CEI 207 — Lifeforce (reprint)

// CEI 208 — Lifelace (reprint)

// CEI 209 — Living Artifact (reprint)

// CEI 210 — Living Lands (reprint)

// CEI 211 — Llanowar Elves (reprint)

// CEI 212 — Lure (reprint)

// CEI 213 — Natural Selection (reprint)

// CEI 214 — Regeneration (reprint)

// CEI 215 — Regrowth (reprint)

// CEI 216 — Scryb Sprites (reprint)

// CEI 217 — Shanodin Dryads (reprint)

// CEI 218 — Stream of Life (reprint)

// CEI 219 — Thicket Basilisk (reprint)

// CEI 220 — Timber Wolves (reprint)

// CEI 221 — Tranquility (reprint)

// CEI 222 — Tsunami (reprint)

// CEI 223 — Verduran Enchantress (reprint)

// CEI 224 — Wall of Brambles (reprint)

// CEI 225 — Wall of Ice (reprint)

// CEI 226 — Wall of Wood (reprint)

// CEI 227 — Wanderlust (reprint)

// CEI 228 — War Mammoth (reprint)

// CEI 229 — Web (reprint)

// CEI 230 — Wild Growth (reprint)

// CEI 231 — Ankh of Mishra (reprint)

// CEI 232 — Basalt Monolith (reprint)

// CEI 233 — Black Lotus (reprint)

// CEI 234 — Black Vise (reprint)

// CEI 235 — Celestial Prism (reprint)

// CEI 236 — Chaos Orb (reprint)

// CEI 237 — Clockwork Beast (reprint)

// CEI 238 — Conservator (reprint)

// CEI 239 — Copper Tablet (reprint)

// CEI 240 — Crystal Rod (reprint)

// CEI 241 — Cyclopean Tomb (reprint)

// CEI 242 — Dingus Egg (reprint)

// CEI 243 — Disrupting Scepter (reprint)

// CEI 244 — Forcefield (reprint)

// CEI 245 — Gauntlet of Might (reprint)

// CEI 246 — Glasses of Urza (reprint)

// CEI 247 — Helm of Chatzuk (reprint)

// CEI 248 — Howling Mine (reprint)

// CEI 249 — Icy Manipulator (reprint)

// CEI 250 — Illusionary Mask (reprint)

// CEI 251 — Iron Star (reprint)

// CEI 252 — Ivory Cup (reprint)

// CEI 253 — Jade Monolith (reprint)

// CEI 254 — Jade Statue (reprint)

// CEI 255 — Jayemdae Tome (reprint)

// CEI 256 — Juggernaut (reprint)

// CEI 257 — Kormus Bell (reprint)

// CEI 258 — Library of Leng (reprint)

// CEI 259 — Living Wall (reprint)

// CEI 260 — Mana Vault (reprint)

// CEI 261 — Meekstone (reprint)

// CEI 262 — Mox Emerald (reprint)

// CEI 263 — Mox Jet (reprint)

// CEI 264 — Mox Pearl (reprint)

// CEI 265 — Mox Ruby (reprint)

// CEI 266 — Mox Sapphire (reprint)

// CEI 267 — Nevinyrral's Disk (reprint)

// CEI 268 — Obsianus Golem (reprint)

// CEI 269 — Rod of Ruin (reprint)

// CEI 270 — Sol Ring (reprint)

// CEI 271 — Soul Net (reprint)

// CEI 272 — Sunglasses of Urza (reprint)

// CEI 273 — The Hive (reprint)

// CEI 274 — Throne of Bone (reprint)

// CEI 275 — Time Vault (reprint)

// CEI 276 — Winter Orb (reprint)

// CEI 277 — Wooden Sphere (reprint)

// CEI 278 — Badlands (reprint)

// CEI 279 — Bayou (reprint)

// CEI 280 — Plateau (reprint)

// CEI 281 — Savannah (reprint)

// CEI 282 — Scrubland (reprint)

// CEI 283 — Taiga (reprint)

// CEI 284 — Tropical Island (reprint)

// CEI 285 — Tundra (reprint)

// CEI 286 — Underground Sea (reprint)

// CEI 287 — Volcanic Island (reprint)

// CEI 288 — Plains (reprint)

// CEI 289 — Plains (alternate printing)

// CEI 290 — Plains (alternate printing)

// CEI 291 — Island (reprint)

// CEI 292 — Island (alternate printing)

// CEI 293 — Island (alternate printing)

// CEI 294 — Swamp (reprint)

// CEI 295 — Swamp (alternate printing)

// CEI 296 — Swamp (alternate printing)

// CEI 297 — Mountain (reprint)

// CEI 298 — Mountain (alternate printing)

// CEI 299 — Mountain (alternate printing)

// CEI 300 — Forest (reprint)

// CEI 301 — Forest (alternate printing)

// CEI 302 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_lea::ANIMATE_WALL), // CEI 1
    PrintingRecord::reprint(&alpha::ARMAGEDDON),         // CEI 2
    PrintingRecord::reprint(&alpha::BALANCE),            // CEI 3
    PrintingRecord::reprint(&catalog_lea::BENALISH_HERO), // CEI 4
    PrintingRecord::reprint(&catalog_lea::BLACK_WARD),   // CEI 5
    PrintingRecord::reprint(&catalog_lea::BLAZE_OF_GLORY), // CEI 6
    PrintingRecord::reprint(&catalog_lea::BLESSING),     // CEI 7
    PrintingRecord::reprint(&catalog_lea::BLUE_WARD),    // CEI 8
    PrintingRecord::reprint(&catalog_lea::CASTLE),       // CEI 9
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // CEI 10
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE), // CEI 11
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // CEI 12
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED), // CEI 13
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // CEI 14
    PrintingRecord::reprint(&catalog_lea::CONSECRATE_LAND), // CEI 15
    PrintingRecord::reprint(&catalog_lea::CONVERSION),   // CEI 16
    PrintingRecord::reprint(&alpha::CRUSADE),            // CEI 17
    PrintingRecord::reprint(&catalog_lea::DEATH_WARD),   // CEI 18
    PrintingRecord::reprint(&alpha::DISENCHANT),         // CEI 19
    PrintingRecord::reprint(&catalog_lea::FARMSTEAD),    // CEI 20
    PrintingRecord::reprint(&catalog_lea::GREEN_WARD),   // CEI 21
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL),     // CEI 22
    PrintingRecord::reprint(&catalog_lea::HEALING_SALVE), // CEI 23
    PrintingRecord::reprint(&catalog_lea::HOLY_ARMOR),   // CEI 24
    PrintingRecord::reprint(&catalog_lea::HOLY_STRENGTH), // CEI 25
    PrintingRecord::reprint(&catalog_lea::ISLAND_SANCTUARY), // CEI 26
    PrintingRecord::reprint(&catalog_lea::KARMA),        // CEI 27
    PrintingRecord::reprint(&catalog_lea::LANCE),        // CEI 28
    PrintingRecord::reprint(&catalog_lea::MESA_PEGASUS), // CEI 29
    PrintingRecord::reprint(&catalog_lea::NORTHERN_PALADIN), // CEI 30
    PrintingRecord::reprint(&catalog_lea::PEARLED_UNICORN), // CEI 31
    PrintingRecord::reprint(&catalog_lea::PERSONAL_INCARNATION), // CEI 32
    PrintingRecord::reprint(&catalog_lea::PURELACE),     // CEI 33
    PrintingRecord::reprint(&catalog_lea::RED_WARD),     // CEI 34
    PrintingRecord::reprint(&catalog_lea::RESURRECTION), // CEI 35
    PrintingRecord::reprint(&catalog_lea::REVERSE_DAMAGE), // CEI 36
    PrintingRecord::reprint(&catalog_lea::RIGHTEOUSNESS), // CEI 37
    PrintingRecord::reprint(&catalog_lea::SAMITE_HEALER), // CEI 38
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS),     // CEI 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),        // CEI 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // CEI 41
    PrintingRecord::reprint(&catalog_lea::VETERAN_BODYGUARD), // CEI 42
    PrintingRecord::reprint(&catalog_lea::WALL_OF_SWORDS), // CEI 43
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),       // CEI 44
    PrintingRecord::reprint(&catalog_lea::WHITE_WARD),   // CEI 45
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),       // CEI 46
    PrintingRecord::reprint(&catalog_lea::AIR_ELEMENTAL), // CEI 47
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL),   // CEI 48
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT),   // CEI 49
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // CEI 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),        // CEI 51
    PrintingRecord::reprint(&catalog_lea::CLONE),        // CEI 52
    PrintingRecord::reprint(&catalog_lea::CONTROL_MAGIC), // CEI 53
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),      // CEI 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),       // CEI 55
    PrintingRecord::reprint(&catalog_lea::CREATURE_BOND), // CEI 56
    PrintingRecord::reprint(&catalog_lea::DRAIN_POWER),  // CEI 57
    PrintingRecord::reprint(&catalog_lea::FEEDBACK),     // CEI 58
    PrintingRecord::reprint(&catalog_lea::FLIGHT),       // CEI 59
    PrintingRecord::reprint(&catalog_lea::INVISIBILITY), // CEI 60
    PrintingRecord::reprint(&catalog_lea::JUMP),         // CEI 61
    PrintingRecord::reprint(&catalog_lea::LIFETAP),      // CEI 62
    PrintingRecord::reprint(&catalog_lea::LORD_OF_ATLANTIS), // CEI 63
    PrintingRecord::reprint(&catalog_lea::MAGICAL_HACK), // CEI 64
    PrintingRecord::reprint(&catalog_lea::MAHAMOTI_DJINN), // CEI 65
    PrintingRecord::reprint(&alpha::MANA_SHORT),         // CEI 66
    PrintingRecord::reprint(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT), // CEI 67
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_FORCES), // CEI 68
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_TERRAIN), // CEI 69
    PrintingRecord::reprint(&catalog_lea::PHANTOM_MONSTER), // CEI 70
    PrintingRecord::reprint(&catalog_lea::PIRATE_SHIP),  // CEI 71
    PrintingRecord::reprint(&catalog_lea::POWER_LEAK),   // CEI 72
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),   // CEI 73
    PrintingRecord::reprint(&catalog_lea::PRODIGAL_SORCERER), // CEI 74
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),      // CEI 75
    PrintingRecord::reprint(&catalog_lea::PSYCHIC_VENOM), // CEI 76
    PrintingRecord::reprint(&catalog_lea::SEA_SERPENT),  // CEI 77
    PrintingRecord::reprint(&catalog_lea::SIREN_S_CALL), // CEI 78
    PrintingRecord::reprint(&catalog_lea::SLEIGHT_OF_MIND), // CEI 79
    PrintingRecord::reprint(&catalog_lea::SPELL_BLAST),  // CEI 80
    PrintingRecord::reprint(&alpha::STASIS),             // CEI 81
    PrintingRecord::reprint(&catalog_lea::STEAL_ARTIFACT), // CEI 82
    PrintingRecord::reprint(&catalog_lea::THOUGHTLACE),  // CEI 83
    PrintingRecord::reprint(&alpha::TIME_WALK),          // CEI 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),        // CEI 85
    PrintingRecord::reprint(&catalog_lea::TWIDDLE),      // CEI 86
    PrintingRecord::reprint(&catalog_lea::UNSUMMON),     // CEI 87
    PrintingRecord::reprint(&catalog_lea::VESUVAN_DOPPELGANGER), // CEI 88
    PrintingRecord::reprint(&catalog_lea::VOLCANIC_ERUPTION), // CEI 89
    PrintingRecord::reprint(&catalog_lea::WALL_OF_AIR),  // CEI 90
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WATER), // CEI 91
    PrintingRecord::reprint(&catalog_lea::WATER_ELEMENTAL), // CEI 92
    PrintingRecord::reprint(&catalog_lea::ANIMATE_DEAD), // CEI 93
    PrintingRecord::reprint(&catalog_lea::BAD_MOON),     // CEI 94
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),       // CEI 95
    PrintingRecord::reprint(&catalog_lea::BOG_WRAITH),   // CEI 96
    PrintingRecord::reprint(&catalog_lea::CONTRACT_FROM_BELOW), // CEI 97
    PrintingRecord::reprint(&catalog_lea::CURSED_LAND),  // CEI 98
    PrintingRecord::reprint(&alpha::DARK_RITUAL),        // CEI 99
    PrintingRecord::reprint(&catalog_lea::DARKPACT),     // CEI 100
    PrintingRecord::reprint(&catalog_lea::DEATHGRIP),    // CEI 101
    PrintingRecord::reprint(&catalog_lea::DEATHLACE),    // CEI 102
    PrintingRecord::reprint(&catalog_lea::DEMONIC_ATTORNEY), // CEI 103
    PrintingRecord::reprint(&catalog_lea::DEMONIC_HORDES), // CEI 104
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),      // CEI 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),         // CEI 106
    PrintingRecord::reprint(&catalog_lea::DRUDGE_SKELETONS), // CEI 107
    PrintingRecord::reprint(&catalog_lea::EVIL_PRESENCE), // CEI 108
    PrintingRecord::reprint(&catalog_lea::FEAR),         // CEI 109
    PrintingRecord::reprint(&catalog_lea::FROZEN_SHADE), // CEI 110
    PrintingRecord::reprint(&catalog_lea::GLOOM),        // CEI 111
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND), // CEI 112
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER),   // CEI 113
    PrintingRecord::reprint(&catalog_lea::LICH),         // CEI 114
    PrintingRecord::reprint(&catalog_lea::LORD_OF_THE_PIT), // CEI 115
    PrintingRecord::reprint(&alpha::MIND_TWIST),         // CEI 116
    PrintingRecord::reprint(&catalog_lea::NETHER_SHADOW), // CEI 117
    PrintingRecord::reprint(&catalog_lea::NETTLING_IMP), // CEI 118
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),    // CEI 119
    PrintingRecord::reprint(&catalog_lea::PARALYZE),     // CEI 120
    PrintingRecord::reprint(&catalog_lea::PESTILENCE),   // CEI 121
    PrintingRecord::reprint(&catalog_lea::PLAGUE_RATS),  // CEI 122
    PrintingRecord::reprint(&catalog_lea::RAISE_DEAD),   // CEI 123
    PrintingRecord::reprint(&catalog_lea::ROYAL_ASSASSIN), // CEI 124
    PrintingRecord::reprint(&catalog_lea::SACRIFICE),    // CEI 125
    PrintingRecord::reprint(&catalog_lea::SCATHE_ZOMBIES), // CEI 126
    PrintingRecord::reprint(&catalog_lea::SCAVENGING_GHOUL), // CEI 127
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE),     // CEI 128
    PrintingRecord::reprint(&catalog_lea::SIMULACRUM),   // CEI 129
    PrintingRecord::reprint(&alpha::SINKHOLE),           // CEI 130
    PrintingRecord::reprint(&alpha::TERROR),             // CEI 131
    PrintingRecord::reprint(&catalog_lea::UNHOLY_STRENGTH), // CEI 132
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BONE), // CEI 133
    PrintingRecord::reprint(&catalog_lea::WARP_ARTIFACT), // CEI 134
    PrintingRecord::reprint(&catalog_lea::WEAKNESS),     // CEI 135
    PrintingRecord::reprint(&catalog_lea::WILL_O_THE_WISP), // CEI 136
    PrintingRecord::reprint(&catalog_lea::WORD_OF_COMMAND), // CEI 137
    PrintingRecord::reprint(&catalog_lea::ZOMBIE_MASTER), // CEI 138
    PrintingRecord::reprint(&catalog_lea::BURROWING),    // CEI 139
    PrintingRecord::reprint(&catalog_lea::CHAOSLACE),    // CEI 140
    PrintingRecord::reprint(&catalog_lea::DISINTEGRATE), // CEI 141
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),       // CEI 142
    PrintingRecord::reprint(&catalog_lea::DWARVEN_DEMOLITION_TEAM), // CEI 143
    PrintingRecord::reprint(&catalog_lea::DWARVEN_WARRIORS), // CEI 144
    PrintingRecord::reprint(&catalog_lea::EARTH_ELEMENTAL), // CEI 145
    PrintingRecord::reprint(&catalog_lea::EARTHBIND),    // CEI 146
    PrintingRecord::reprint(&alpha::EARTHQUAKE),         // CEI 147
    PrintingRecord::reprint(&catalog_lea::FALSE_ORDERS), // CEI 148
    PrintingRecord::reprint(&catalog_lea::FIRE_ELEMENTAL), // CEI 149
    PrintingRecord::reprint(&alpha::FIREBALL),           // CEI 150
    PrintingRecord::reprint(&catalog_lea::FIREBREATHING), // CEI 151
    PrintingRecord::reprint(&catalog_lea::FLASHFIRES),   // CEI 152
    PrintingRecord::reprint(&alpha::FORK),               // CEI 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // CEI 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),        // CEI 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE),   // CEI 156
    PrintingRecord::reprint(&catalog_lea::GRAY_OGRE),    // CEI 157
    PrintingRecord::reprint(&catalog_lea::HILL_GIANT),   // CEI 158
    PrintingRecord::reprint(&catalog_lea::HURLOON_MINOTAUR), // CEI 159
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),      // CEI 160
    PrintingRecord::reprint(&catalog_lea::KELDON_WARLORD), // CEI 161
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT),     // CEI 162
    PrintingRecord::reprint(&catalog_lea::MANA_FLARE),   // CEI 163
    PrintingRecord::reprint(&catalog_lea::MANABARBS),    // CEI 164
    PrintingRecord::reprint(&catalog_lea::MONSS_GOBLIN_RAIDERS), // CEI 165
    PrintingRecord::reprint(&catalog_lea::ORCISH_ARTILLERY), // CEI 166
    PrintingRecord::reprint(&catalog_lea::ORCISH_ORIFLAMME), // CEI 167
    PrintingRecord::reprint(&catalog_lea::POWER_SURGE),  // CEI 168
    PrintingRecord::reprint(&catalog_lea::RAGING_RIVER), // CEI 169
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // CEI 170
    PrintingRecord::reprint(&catalog_lea::ROC_OF_KHER_RIDGES), // CEI 171
    PrintingRecord::reprint(&catalog_lea::ROCK_HYDRA),   // CEI 172
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),        // CEI 173
    PrintingRecord::reprint(&alpha::SHATTER),            // CEI 174
    PrintingRecord::reprint(&catalog_lea::SHIVAN_DRAGON), // CEI 175
    PrintingRecord::reprint(&alpha::SMOKE),              // CEI 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),        // CEI 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),         // CEI 178
    PrintingRecord::reprint(&catalog_lea::TUNNEL),       // CEI 179
    PrintingRecord::reprint(&catalog_lea::TWO_HEADED_GIANT_OF_FORIYS), // CEI 180
    PrintingRecord::reprint(&catalog_lea::UTHDEN_TROLL), // CEI 181
    PrintingRecord::reprint(&catalog_lea::WALL_OF_FIRE), // CEI 182
    PrintingRecord::reprint(&catalog_lea::WALL_OF_STONE), // CEI 183
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE),   // CEI 184
    PrintingRecord::reprint(&catalog_lea::ASPECT_OF_WOLF), // CEI 185
    PrintingRecord::reprint(&alpha::BERSERK),            // CEI 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE),  // CEI 187
    PrintingRecord::reprint(&catalog_lea::CAMOUFLAGE),   // CEI 188
    PrintingRecord::reprint(&alpha::CHANNEL),            // CEI 189
    PrintingRecord::reprint(&catalog_lea::COCKATRICE),   // CEI 190
    PrintingRecord::reprint(&catalog_lea::CRAW_WURM),    // CEI 191
    PrintingRecord::reprint(&catalog_lea::ELVISH_ARCHERS), // CEI 192
    PrintingRecord::reprint(&catalog_lea::FASTBOND),     // CEI 193
    PrintingRecord::reprint(&catalog_lea::FOG),          // CEI 194
    PrintingRecord::reprint(&catalog_lea::FORCE_OF_NATURE), // CEI 195
    PrintingRecord::reprint(&catalog_lea::FUNGUSAUR),    // CEI 196
    PrintingRecord::reprint(&catalog_lea::GAEA_S_LIEGE), // CEI 197
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),       // CEI 198
    PrintingRecord::reprint(&catalog_lea::GIANT_SPIDER), // CEI 199
    PrintingRecord::reprint(&catalog_lea::GRIZZLY_BEARS), // CEI 200
    PrintingRecord::reprint(&catalog_lea::HURRICANE),    // CEI 201
    PrintingRecord::reprint(&catalog_lea::ICE_STORM),    // CEI 202
    PrintingRecord::reprint(&catalog_lea::INSTILL_ENERGY), // CEI 203
    PrintingRecord::reprint(&catalog_lea::IRONROOT_TREEFOLK), // CEI 204
    PrintingRecord::reprint(&catalog_lea::KUDZU),        // CEI 205
    PrintingRecord::reprint(&catalog_lea::LEY_DRUID),    // CEI 206
    PrintingRecord::reprint(&catalog_lea::LIFEFORCE),    // CEI 207
    PrintingRecord::reprint(&catalog_lea::LIFELACE),     // CEI 208
    PrintingRecord::reprint(&catalog_lea::LIVING_ARTIFACT), // CEI 209
    PrintingRecord::reprint(&catalog_lea::LIVING_LANDS), // CEI 210
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES),     // CEI 211
    PrintingRecord::reprint(&catalog_lea::LURE),         // CEI 212
    PrintingRecord::reprint(&catalog_lea::NATURAL_SELECTION), // CEI 213
    PrintingRecord::reprint(&catalog_lea::REGENERATION), // CEI 214
    PrintingRecord::reprint(&alpha::REGROWTH),           // CEI 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),      // CEI 216
    PrintingRecord::reprint(&catalog_lea::SHANODIN_DRYADS), // CEI 217
    PrintingRecord::reprint(&catalog_lea::STREAM_OF_LIFE), // CEI 218
    PrintingRecord::reprint(&catalog_lea::THICKET_BASILISK), // CEI 219
    PrintingRecord::reprint(&catalog_lea::TIMBER_WOLVES), // CEI 220
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),  // CEI 221
    PrintingRecord::reprint(&catalog_lea::TSUNAMI),      // CEI 222
    PrintingRecord::reprint(&catalog_lea::VERDURAN_ENCHANTRESS), // CEI 223
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BRAMBLES), // CEI 224
    PrintingRecord::reprint(&catalog_lea::WALL_OF_ICE),  // CEI 225
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WOOD), // CEI 226
    PrintingRecord::reprint(&catalog_lea::WANDERLUST),   // CEI 227
    PrintingRecord::reprint(&catalog_lea::WAR_MAMMOTH),  // CEI 228
    PrintingRecord::reprint(&catalog_lea::WEB),          // CEI 229
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),  // CEI 230
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA),     // CEI 231
    PrintingRecord::reprint(&catalog_lea::BASALT_MONOLITH), // CEI 232
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),        // CEI 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),         // CEI 234
    PrintingRecord::reprint(&catalog_lea::CELESTIAL_PRISM), // CEI 235
    PrintingRecord::reprint(&alpha::CHAOS_ORB),          // CEI 236
    PrintingRecord::reprint(&catalog_lea::CLOCKWORK_BEAST), // CEI 237
    PrintingRecord::reprint(&catalog_lea::CONSERVATOR),  // CEI 238
    PrintingRecord::reprint(&alpha::COPPER_TABLET),      // CEI 239
    PrintingRecord::reprint(&catalog_lea::CRYSTAL_ROD),  // CEI 240
    PrintingRecord::reprint(&catalog_lea::CYCLOPEAN_TOMB), // CEI 241
    PrintingRecord::reprint(&catalog_lea::DINGUS_EGG),   // CEI 242
    PrintingRecord::reprint(&catalog_lea::DISRUPTING_SCEPTER), // CEI 243
    PrintingRecord::reprint(&catalog_lea::FORCEFIELD),   // CEI 244
    PrintingRecord::reprint(&catalog_lea::GAUNTLET_OF_MIGHT), // CEI 245
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA),    // CEI 246
    PrintingRecord::reprint(&catalog_lea::HELM_OF_CHATZUK), // CEI 247
    PrintingRecord::reprint(&catalog_lea::HOWLING_MINE), // CEI 248
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR),    // CEI 249
    PrintingRecord::reprint(&catalog_lea::ILLUSIONARY_MASK), // CEI 250
    PrintingRecord::reprint(&alpha::IRON_STAR),          // CEI 251
    PrintingRecord::reprint(&catalog_lea::IVORY_CUP),    // CEI 252
    PrintingRecord::reprint(&catalog_lea::JADE_MONOLITH), // CEI 253
    PrintingRecord::reprint(&catalog_lea::JADE_STATUE),  // CEI 254
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),      // CEI 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),         // CEI 256
    PrintingRecord::reprint(&catalog_lea::KORMUS_BELL),  // CEI 257
    PrintingRecord::reprint(&catalog_lea::LIBRARY_OF_LENG), // CEI 258
    PrintingRecord::reprint(&catalog_lea::LIVING_WALL),  // CEI 259
    PrintingRecord::reprint(&alpha::MANA_VAULT),         // CEI 260
    PrintingRecord::reprint(&catalog_lea::MEEKSTONE),    // CEI 261
    PrintingRecord::reprint(&alpha::MOX_EMERALD),        // CEI 262
    PrintingRecord::reprint(&alpha::MOX_JET),            // CEI 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),          // CEI 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),           // CEI 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),       // CEI 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK),   // CEI 267
    PrintingRecord::reprint(&catalog_lea::OBSIANUS_GOLEM), // CEI 268
    PrintingRecord::reprint(&catalog_lea::ROD_OF_RUIN),  // CEI 269
    PrintingRecord::reprint(&alpha::SOL_RING),           // CEI 270
    PrintingRecord::reprint(&catalog_lea::SOUL_NET),     // CEI 271
    PrintingRecord::reprint(&catalog_lea::SUNGLASSES_OF_URZA), // CEI 272
    PrintingRecord::reprint(&catalog_lea::THE_HIVE),     // CEI 273
    PrintingRecord::reprint(&catalog_lea::THRONE_OF_BONE), // CEI 274
    PrintingRecord::reprint(&alpha::TIME_VAULT),         // CEI 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),         // CEI 276
    PrintingRecord::reprint(&catalog_lea::WOODEN_SPHERE), // CEI 277
    PrintingRecord::reprint(&alpha::BADLANDS),           // CEI 278
    PrintingRecord::reprint(&alpha::BAYOU),              // CEI 279
    PrintingRecord::reprint(&alpha::PLATEAU),            // CEI 280
    PrintingRecord::reprint(&alpha::SAVANNAH),           // CEI 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),          // CEI 282
    PrintingRecord::reprint(&alpha::TAIGA),              // CEI 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND),    // CEI 284
    PrintingRecord::reprint(&alpha::TUNDRA),             // CEI 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA),    // CEI 286
    PrintingRecord::reprint(&beta::VOLCANIC_ISLAND),     // CEI 287
    PrintingRecord::reprint(&alpha::PLAINS),             // CEI 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),        // CEI 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),        // CEI 290
    PrintingRecord::reprint(&alpha::ISLAND),             // CEI 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),        // CEI 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),        // CEI 293
    PrintingRecord::reprint(&alpha::SWAMP),              // CEI 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),         // CEI 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),         // CEI 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),           // CEI 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),      // CEI 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),      // CEI 299
    PrintingRecord::reprint(&alpha::FOREST),             // CEI 300
    PrintingRecord::alternate(&alpha::FOREST, 1),        // CEI 301
    PrintingRecord::alternate(&alpha::FOREST, 2),        // CEI 302
];
