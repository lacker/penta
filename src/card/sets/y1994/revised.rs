//! Revised Edition has no unique catalog records.
//!
//! Cards legal through this printing reuse their earliest built-in definition.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::alpha;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::antiquities as catalog_atq;

// 3ED 1 — Animate Wall (reprint)

// 3ED 2 — Armageddon (reprint)

// 3ED 3 — Balance (reprint)

// 3ED 4 — Benalish Hero (reprint)

// 3ED 5 — Black Ward (reprint)

// 3ED 6 — Blessing (reprint)

// 3ED 7 — Blue Ward (reprint)

// 3ED 8 — Castle (reprint)

// 3ED 9 — Circle of Protection: Black (reprint)

// 3ED 10 — Circle of Protection: Blue (reprint)

// 3ED 11 — Circle of Protection: Green (reprint)

// 3ED 12 — Circle of Protection: Red (reprint)

// 3ED 13 — Circle of Protection: White (reprint)

// 3ED 14 — Conversion (reprint)

// 3ED 15 — Crusade (reprint)

// 3ED 16 — Death Ward (reprint)

// 3ED 17 — Disenchant (reprint)

// 3ED 18 — Eye for an Eye (reprint)

// 3ED 19 — Farmstead (reprint)

// 3ED 20 — Green Ward (reprint)

// 3ED 21 — Guardian Angel (reprint)

// 3ED 22 — Healing Salve (reprint)

// 3ED 23 — Holy Armor (reprint)

// 3ED 24 — Holy Strength (reprint)

// 3ED 25 — Island Sanctuary (reprint)

// 3ED 26 — Karma (reprint)

// 3ED 27 — Lance (reprint)

// 3ED 28 — Mesa Pegasus (reprint)

// 3ED 29 — Northern Paladin (reprint)

// 3ED 30 — Pearled Unicorn (reprint)

// 3ED 31 — Personal Incarnation (reprint)

// 3ED 32 — Purelace (reprint)

// 3ED 33 — Red Ward (reprint)

// 3ED 34 — Resurrection (reprint)

// 3ED 35 — Reverse Damage (reprint)

// 3ED 36 — Reverse Polarity (reprint)

// 3ED 37 — Righteousness (reprint)

// 3ED 38 — Samite Healer (reprint)

// 3ED 39 — Savannah Lions (reprint)

// 3ED 40 — Serra Angel (reprint)

// 3ED 41 — Swords to Plowshares (reprint)

// 3ED 42 — Veteran Bodyguard (reprint)

// 3ED 43 — Wall of Swords (reprint)

// 3ED 44 — White Knight (reprint)

// 3ED 45 — White Ward (reprint)

// 3ED 46 — Wrath of God (reprint)

// 3ED 47 — Air Elemental (reprint)

// 3ED 48 — Animate Artifact (reprint)

// 3ED 49 — Blue Elemental Blast (reprint)

// 3ED 50 — Braingeyser (reprint)

// 3ED 51 — Clone (reprint)

// 3ED 52 — Control Magic (reprint)

// 3ED 53 — Copy Artifact (reprint)

// 3ED 54 — Counterspell (reprint)

// 3ED 55 — Creature Bond (reprint)

// 3ED 56 — Drain Power (reprint)

// 3ED 57 — Energy Flux (reprint)

// 3ED 58 — Feedback (reprint)

// 3ED 59 — Flight (reprint)

// 3ED 60 — Hurkyl's Recall (reprint)

// 3ED 61 — Island Fish Jasconius (reprint)

// 3ED 62 — Jump (reprint)

// 3ED 63 — Lifetap (reprint)

// 3ED 64 — Lord of Atlantis (reprint)

// 3ED 65 — Magical Hack (reprint)

// 3ED 66 — Mahamoti Djinn (reprint)

// 3ED 67 — Mana Short (reprint)

// 3ED 68 — Merfolk of the Pearl Trident (reprint)

// 3ED 69 — Phantasmal Forces (reprint)

// 3ED 70 — Phantasmal Terrain (reprint)

// 3ED 71 — Phantom Monster (reprint)

// 3ED 72 — Pirate Ship (reprint)

// 3ED 73 — Power Leak (reprint)

// 3ED 74 — Power Sink (reprint)

// 3ED 75 — Prodigal Sorcerer (reprint)

// 3ED 76 — Psychic Venom (reprint)

// 3ED 77 — Reconstruction (reprint)

// 3ED 78 — Sea Serpent (reprint)

// 3ED 79 — Serendib Efreet (reprint)

// 3ED 80 — Siren's Call (reprint)

// 3ED 81 — Sleight of Mind (reprint)

// 3ED 82 — Spell Blast (reprint)

// 3ED 83 — Stasis (reprint)

// 3ED 84 — Steal Artifact (reprint)

// 3ED 85 — Thoughtlace (reprint)

// 3ED 86 — Unstable Mutation (reprint)

// 3ED 87 — Unsummon (reprint)

// 3ED 88 — Vesuvan Doppelganger (reprint)

// 3ED 89 — Volcanic Eruption (reprint)

// 3ED 90 — Wall of Air (reprint)

// 3ED 91 — Wall of Water (reprint)

// 3ED 92 — Water Elemental (reprint)

// 3ED 93 — Animate Dead (reprint)

// 3ED 94 — Bad Moon (reprint)

// 3ED 95 — Black Knight (reprint)

// 3ED 96 — Bog Wraith (reprint)

// 3ED 97 — Contract from Below (reprint)

// 3ED 98 — Cursed Land (reprint)

// 3ED 99 — Dark Ritual (reprint)

// 3ED 100 — Darkpact (reprint)

// 3ED 101 — Deathgrip (reprint)

// 3ED 102 — Deathlace (reprint)

// 3ED 103 — Demonic Attorney (reprint)

// 3ED 104 — Demonic Hordes (reprint)

// 3ED 105 — Demonic Tutor (reprint)

// 3ED 106 — Drain Life (reprint)

// 3ED 107 — Drudge Skeletons (reprint)

// 3ED 108 — El-Hajjâj (reprint)

// 3ED 109 — Erg Raiders (reprint)

// 3ED 110 — Evil Presence (reprint)

// 3ED 111 — Fear (reprint)

// 3ED 112 — Frozen Shade (reprint)

// 3ED 113 — Gloom (reprint)

// 3ED 114 — Howl from Beyond (reprint)

// 3ED 115 — Hypnotic Specter (reprint)

// 3ED 116 — Lord of the Pit (reprint)

// 3ED 117 — Mind Twist (reprint)

// 3ED 118 — Nether Shadow (reprint)

// 3ED 119 — Nettling Imp (reprint)

// 3ED 120 — Nightmare (reprint)

// 3ED 121 — Paralyze (reprint)

// 3ED 122 — Pestilence (reprint)

// 3ED 123 — Plague Rats (reprint)

// 3ED 124 — Raise Dead (reprint)

// 3ED 125 — Royal Assassin (reprint)

// 3ED 126 — Sacrifice (reprint)

// 3ED 127 — Scathe Zombies (reprint)

// 3ED 128 — Scavenging Ghoul (reprint)

// 3ED 129 — Sengir Vampire (reprint)

// 3ED 130 — Simulacrum (reprint)

// 3ED 131 — Sorceress Queen (reprint)

// 3ED 132 — Terror (reprint)

// 3ED 133 — Unholy Strength (reprint)

// 3ED 134 — Wall of Bone (reprint)

// 3ED 135 — Warp Artifact (reprint)

// 3ED 136 — Weakness (reprint)

// 3ED 137 — Will-o'-the-Wisp (reprint)

// 3ED 138 — Zombie Master (reprint)

// 3ED 139 — Atog (reprint)

// 3ED 140 — Burrowing (reprint)

// 3ED 141 — Chaoslace (reprint)

// 3ED 142 — Disintegrate (reprint)

// 3ED 143 — Dragon Whelp (reprint)

// 3ED 144 — Dwarven Warriors (reprint)

// 3ED 145 — Dwarven Weaponsmith (reprint)

// 3ED 146 — Earth Elemental (reprint)

// 3ED 147 — Earthbind (reprint)

// 3ED 148 — Earthquake (reprint)

// 3ED 149 — Fire Elemental (reprint)

// 3ED 150 — Fireball (reprint)

// 3ED 151 — Firebreathing (reprint)

// 3ED 152 — Flashfires (reprint)

// 3ED 153 — Fork (reprint)

// 3ED 154 — Goblin Balloon Brigade (reprint)

// 3ED 155 — Goblin King (reprint)

// 3ED 156 — Granite Gargoyle (reprint)

// 3ED 157 — Gray Ogre (reprint)

// 3ED 158 — Hill Giant (reprint)

// 3ED 159 — Hurloon Minotaur (reprint)

// 3ED 160 — Keldon Warlord (reprint)

// 3ED 161 — Kird Ape (reprint)

// 3ED 162 — Lightning Bolt (reprint)

// 3ED 163 — Magnetic Mountain (reprint)

// 3ED 164 — Mana Flare (reprint)

// 3ED 165 — Manabarbs (reprint)

// 3ED 166 — Mijae Djinn (reprint)

// 3ED 167 — Mons's Goblin Raiders (reprint)

// 3ED 168 — Orcish Artillery (reprint)

// 3ED 169 — Orcish Oriflamme (reprint)

// 3ED 170 — Power Surge (reprint)

// 3ED 171 — Red Elemental Blast (reprint)

// 3ED 172 — Roc of Kher Ridges (reprint)

// 3ED 173 — Rock Hydra (reprint)

// 3ED 174 — Sedge Troll (reprint)

// 3ED 175 — Shatter (reprint)

// 3ED 176 — Shatterstorm (reprint)

// 3ED 177 — Shivan Dragon (reprint)

// 3ED 178 — Smoke (reprint)

// 3ED 179 — Stone Giant (reprint)

// 3ED 180 — Stone Rain (reprint)

// 3ED 181 — Tunnel (reprint)

// 3ED 182 — Uthden Troll (reprint)

// 3ED 183 — Wall of Fire (reprint)

// 3ED 184 — Wall of Stone (reprint)

// 3ED 185 — Wheel of Fortune (reprint)

// 3ED 186 — Aspect of Wolf (reprint)

// 3ED 187 — Birds of Paradise (reprint)

// 3ED 188 — Channel (reprint)

// 3ED 189 — Cockatrice (reprint)

// 3ED 190 — Craw Wurm (reprint)

// 3ED 191 — Crumble (reprint)

// 3ED 192 — Desert Twister (reprint)

// 3ED 193 — Elvish Archers (reprint)

// 3ED 194 — Fastbond (reprint)

// 3ED 195 — Fog (reprint)

// 3ED 196 — Force of Nature (reprint)

// 3ED 197 — Fungusaur (reprint)

// 3ED 198 — Gaea's Liege (reprint)

// 3ED 199 — Giant Growth (reprint)

// 3ED 200 — Giant Spider (reprint)

// 3ED 201 — Grizzly Bears (reprint)

// 3ED 202 — Hurricane (reprint)

// 3ED 203 — Instill Energy (reprint)

// 3ED 204 — Ironroot Treefolk (reprint)

// 3ED 205 — Kudzu (reprint)

// 3ED 206 — Ley Druid (reprint)

// 3ED 207 — Lifeforce (reprint)

// 3ED 208 — Lifelace (reprint)

// 3ED 209 — Living Artifact (reprint)

// 3ED 210 — Living Lands (reprint)

// 3ED 211 — Llanowar Elves (reprint)

// 3ED 212 — Lure (reprint)

// 3ED 213 — Regeneration (reprint)

// 3ED 214 — Regrowth (reprint)

// 3ED 215 — Scryb Sprites (reprint)

// 3ED 216 — Shanodin Dryads (reprint)

// 3ED 217 — Stream of Life (reprint)

// 3ED 218 — Thicket Basilisk (reprint)

// 3ED 219 — Timber Wolves (reprint)

// 3ED 220 — Titania's Song (reprint)

// 3ED 221 — Tranquility (reprint)

// 3ED 222 — Tsunami (reprint)

// 3ED 223 — Verduran Enchantress (reprint)

// 3ED 224 — Wall of Brambles (reprint)

// 3ED 225 — Wall of Ice (reprint)

// 3ED 226 — Wall of Wood (reprint)

// 3ED 227 — Wanderlust (reprint)

// 3ED 228 — War Mammoth (reprint)

// 3ED 229 — Web (reprint)

// 3ED 230 — Wild Growth (reprint)

// 3ED 231 — Aladdin's Lamp (reprint)

// 3ED 232 — Aladdin's Ring (reprint)

// 3ED 233 — Ankh of Mishra (reprint)

// 3ED 234 — Armageddon Clock (reprint)

// 3ED 235 — Basalt Monolith (reprint)

// 3ED 236 — Black Vise (reprint)

// 3ED 237 — Bottle of Suleiman (reprint)

// 3ED 238 — Brass Man (reprint)

// 3ED 239 — Celestial Prism (reprint)

// 3ED 240 — Clockwork Beast (reprint)

// 3ED 241 — Conservator (reprint)

// 3ED 242 — Crystal Rod (reprint)

// 3ED 243 — Dancing Scimitar (reprint)

// 3ED 244 — Dingus Egg (reprint)

// 3ED 245 — Disrupting Scepter (reprint)

// 3ED 246 — Dragon Engine (reprint)

// 3ED 247 — Ebony Horse (reprint)

// 3ED 248 — Flying Carpet (reprint)

// 3ED 249 — Glasses of Urza (reprint)

// 3ED 250 — Helm of Chatzuk (reprint)

// 3ED 251 — Howling Mine (reprint)

// 3ED 252 — Iron Star (reprint)

// 3ED 253 — Ivory Cup (reprint)

// 3ED 254 — Ivory Tower (reprint)

// 3ED 255 — Jade Monolith (reprint)

// 3ED 256 — Jandor's Ring (reprint)

// 3ED 257 — Jandor's Saddlebags (reprint)

// 3ED 258 — Jayemdae Tome (reprint)

// 3ED 259 — Juggernaut (reprint)

// 3ED 260 — Kormus Bell (reprint)

// 3ED 261 — Library of Leng (reprint)

// 3ED 262 — Living Wall (reprint)

// 3ED 263 — Mana Vault (reprint)

// 3ED 264 — Meekstone (reprint)

// 3ED 265 — Millstone (reprint)

// 3ED 266 — Mishra's War Machine (reprint)

// 3ED 267 — Nevinyrral's Disk (reprint)

// 3ED 268 — Obsianus Golem (reprint)

// 3ED 269 — Onulet (reprint)

// 3ED 270 — Ornithopter (reprint)

// 3ED 271 — Primal Clay (reprint)

// 3ED 272 — Rocket Launcher (reprint)

// 3ED 273 — Rod of Ruin (reprint)

// 3ED 274 — Sol Ring (reprint)

// 3ED 275 — Soul Net (reprint)

// 3ED 276 — Sunglasses of Urza (reprint)

// 3ED 277 — The Hive (reprint)

// 3ED 278 — The Rack (reprint)

// 3ED 279 — Throne of Bone (reprint)

// 3ED 280 — Winter Orb (reprint)

// 3ED 281 — Wooden Sphere (reprint)

// 3ED 282 — Badlands (reprint)

// 3ED 283 — Bayou (reprint)

// 3ED 284 — Plateau (reprint)

// 3ED 285 — Savannah (reprint)

// 3ED 286 — Scrubland (reprint)

// 3ED 287 — Taiga (reprint)

// 3ED 288 — Tropical Island (reprint)

// 3ED 289 — Tundra (reprint)

// 3ED 290 — Underground Sea (reprint)

// 3ED 291 — Volcanic Island (reprint)

// 3ED 292 — Plains (reprint)

// 3ED 293 — Plains (alternate printing)

// 3ED 294 — Plains (alternate printing)

// 3ED 295 — Island (reprint)

// 3ED 296 — Island (alternate printing)

// 3ED 297 — Island (alternate printing)

// 3ED 298 — Swamp (reprint)

// 3ED 299 — Swamp (alternate printing)

// 3ED 300 — Swamp (alternate printing)

// 3ED 301 — Mountain (reprint)

// 3ED 302 — Mountain (alternate printing)

// 3ED 303 — Mountain (alternate printing)

// 3ED 304 — Forest (reprint)

// 3ED 305 — Forest (alternate printing)

// 3ED 306 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_lea::ANIMATE_WALL), // 3ED 1
    PrintingRecord::reprint(&catalog_lea::ARMAGEDDON),   // 3ED 2
    PrintingRecord::reprint(&catalog_lea::BALANCE),      // 3ED 3
    PrintingRecord::reprint(&catalog_lea::BENALISH_HERO), // 3ED 4
    PrintingRecord::reprint(&catalog_lea::BLACK_WARD),   // 3ED 5
    PrintingRecord::reprint(&catalog_lea::BLESSING),     // 3ED 6
    PrintingRecord::reprint(&catalog_lea::BLUE_WARD),    // 3ED 7
    PrintingRecord::reprint(&catalog_lea::CASTLE),       // 3ED 8
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // 3ED 9
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE), // 3ED 10
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // 3ED 11
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED), // 3ED 12
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // 3ED 13
    PrintingRecord::reprint(&catalog_lea::CONVERSION),   // 3ED 14
    PrintingRecord::reprint(&catalog_lea::CRUSADE),      // 3ED 15
    PrintingRecord::reprint(&catalog_lea::DEATH_WARD),   // 3ED 16
    PrintingRecord::reprint(&catalog_lea::DISENCHANT),   // 3ED 17
    PrintingRecord::reprint(&catalog_arn::EYE_FOR_AN_EYE), // 3ED 18
    PrintingRecord::reprint(&catalog_lea::FARMSTEAD),    // 3ED 19
    PrintingRecord::reprint(&catalog_lea::GREEN_WARD),   // 3ED 20
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL),     // 3ED 21
    PrintingRecord::reprint(&catalog_lea::HEALING_SALVE), // 3ED 22
    PrintingRecord::reprint(&catalog_lea::HOLY_ARMOR),   // 3ED 23
    PrintingRecord::reprint(&catalog_lea::HOLY_STRENGTH), // 3ED 24
    PrintingRecord::reprint(&catalog_lea::ISLAND_SANCTUARY), // 3ED 25
    PrintingRecord::reprint(&catalog_lea::KARMA),        // 3ED 26
    PrintingRecord::reprint(&catalog_lea::LANCE),        // 3ED 27
    PrintingRecord::reprint(&catalog_lea::MESA_PEGASUS), // 3ED 28
    PrintingRecord::reprint(&catalog_lea::NORTHERN_PALADIN), // 3ED 29
    PrintingRecord::reprint(&catalog_lea::PEARLED_UNICORN), // 3ED 30
    PrintingRecord::reprint(&catalog_lea::PERSONAL_INCARNATION), // 3ED 31
    PrintingRecord::reprint(&catalog_lea::PURELACE),     // 3ED 32
    PrintingRecord::reprint(&catalog_lea::RED_WARD),     // 3ED 33
    PrintingRecord::reprint(&catalog_lea::RESURRECTION), // 3ED 34
    PrintingRecord::reprint(&catalog_lea::REVERSE_DAMAGE), // 3ED 35
    PrintingRecord::reprint(&catalog_atq::REVERSE_POLARITY), // 3ED 36
    PrintingRecord::reprint(&catalog_lea::RIGHTEOUSNESS), // 3ED 37
    PrintingRecord::reprint(&catalog_lea::SAMITE_HEALER), // 3ED 38
    PrintingRecord::reprint(&catalog_lea::SAVANNAH_LIONS), // 3ED 39
    PrintingRecord::reprint(&catalog_lea::SERRA_ANGEL),  // 3ED 40
    PrintingRecord::reprint(&catalog_lea::SWORDS_TO_PLOWSHARES), // 3ED 41
    PrintingRecord::reprint(&catalog_lea::VETERAN_BODYGUARD), // 3ED 42
    PrintingRecord::reprint(&catalog_lea::WALL_OF_SWORDS), // 3ED 43
    PrintingRecord::reprint(&catalog_lea::WHITE_KNIGHT), // 3ED 44
    PrintingRecord::reprint(&catalog_lea::WHITE_WARD),   // 3ED 45
    PrintingRecord::reprint(&catalog_lea::WRATH_OF_GOD), // 3ED 46
    PrintingRecord::reprint(&catalog_lea::AIR_ELEMENTAL), // 3ED 47
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT),   // 3ED 48
    PrintingRecord::reprint(&catalog_lea::BLUE_ELEMENTAL_BLAST), // 3ED 49
    PrintingRecord::reprint(&catalog_lea::BRAINGEYSER),  // 3ED 50
    PrintingRecord::reprint(&catalog_lea::CLONE),        // 3ED 51
    PrintingRecord::reprint(&catalog_lea::CONTROL_MAGIC), // 3ED 52
    PrintingRecord::reprint(&catalog_lea::COPY_ARTIFACT), // 3ED 53
    PrintingRecord::reprint(&catalog_lea::COUNTERSPELL), // 3ED 54
    PrintingRecord::reprint(&catalog_lea::CREATURE_BOND), // 3ED 55
    PrintingRecord::reprint(&catalog_lea::DRAIN_POWER),  // 3ED 56
    PrintingRecord::reprint(&catalog_atq::ENERGY_FLUX),  // 3ED 57
    PrintingRecord::reprint(&catalog_lea::FEEDBACK),     // 3ED 58
    PrintingRecord::reprint(&catalog_lea::FLIGHT),       // 3ED 59
    PrintingRecord::reprint(&catalog_atq::HURKYLS_RECALL), // 3ED 60
    PrintingRecord::reprint(&catalog_arn::ISLAND_FISH_JASCONIUS), // 3ED 61
    PrintingRecord::reprint(&catalog_lea::JUMP),         // 3ED 62
    PrintingRecord::reprint(&catalog_lea::LIFETAP),      // 3ED 63
    PrintingRecord::reprint(&catalog_lea::LORD_OF_ATLANTIS), // 3ED 64
    PrintingRecord::reprint(&catalog_lea::MAGICAL_HACK), // 3ED 65
    PrintingRecord::reprint(&catalog_lea::MAHAMOTI_DJINN), // 3ED 66
    PrintingRecord::reprint(&catalog_lea::MANA_SHORT),   // 3ED 67
    PrintingRecord::reprint(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT), // 3ED 68
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_FORCES), // 3ED 69
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_TERRAIN), // 3ED 70
    PrintingRecord::reprint(&catalog_lea::PHANTOM_MONSTER), // 3ED 71
    PrintingRecord::reprint(&catalog_lea::PIRATE_SHIP),  // 3ED 72
    PrintingRecord::reprint(&catalog_lea::POWER_LEAK),   // 3ED 73
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),   // 3ED 74
    PrintingRecord::reprint(&catalog_lea::PRODIGAL_SORCERER), // 3ED 75
    PrintingRecord::reprint(&catalog_lea::PSYCHIC_VENOM), // 3ED 76
    PrintingRecord::reprint(&catalog_atq::RECONSTRUCTION), // 3ED 77
    PrintingRecord::reprint(&catalog_lea::SEA_SERPENT),  // 3ED 78
    PrintingRecord::reprint(&catalog_arn::SERENDIB_EFREET), // 3ED 79
    PrintingRecord::reprint(&catalog_lea::SIREN_S_CALL), // 3ED 80
    PrintingRecord::reprint(&catalog_lea::SLEIGHT_OF_MIND), // 3ED 81
    PrintingRecord::reprint(&catalog_lea::SPELL_BLAST),  // 3ED 82
    PrintingRecord::reprint(&catalog_lea::STASIS),       // 3ED 83
    PrintingRecord::reprint(&catalog_lea::STEAL_ARTIFACT), // 3ED 84
    PrintingRecord::reprint(&catalog_lea::THOUGHTLACE),  // 3ED 85
    PrintingRecord::reprint(&catalog_arn::UNSTABLE_MUTATION), // 3ED 86
    PrintingRecord::reprint(&catalog_lea::UNSUMMON),     // 3ED 87
    PrintingRecord::reprint(&catalog_lea::VESUVAN_DOPPELGANGER), // 3ED 88
    PrintingRecord::reprint(&catalog_lea::VOLCANIC_ERUPTION), // 3ED 89
    PrintingRecord::reprint(&catalog_lea::WALL_OF_AIR),  // 3ED 90
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WATER), // 3ED 91
    PrintingRecord::reprint(&catalog_lea::WATER_ELEMENTAL), // 3ED 92
    PrintingRecord::reprint(&catalog_lea::ANIMATE_DEAD), // 3ED 93
    PrintingRecord::reprint(&catalog_lea::BAD_MOON),     // 3ED 94
    PrintingRecord::reprint(&catalog_lea::BLACK_KNIGHT), // 3ED 95
    PrintingRecord::reprint(&catalog_lea::BOG_WRAITH),   // 3ED 96
    PrintingRecord::reprint(&catalog_lea::CONTRACT_FROM_BELOW), // 3ED 97
    PrintingRecord::reprint(&catalog_lea::CURSED_LAND),  // 3ED 98
    PrintingRecord::reprint(&catalog_lea::DARK_RITUAL),  // 3ED 99
    PrintingRecord::reprint(&catalog_lea::DARKPACT),     // 3ED 100
    PrintingRecord::reprint(&catalog_lea::DEATHGRIP),    // 3ED 101
    PrintingRecord::reprint(&catalog_lea::DEATHLACE),    // 3ED 102
    PrintingRecord::reprint(&catalog_lea::DEMONIC_ATTORNEY), // 3ED 103
    PrintingRecord::reprint(&catalog_lea::DEMONIC_HORDES), // 3ED 104
    PrintingRecord::reprint(&catalog_lea::DEMONIC_TUTOR), // 3ED 105
    PrintingRecord::reprint(&catalog_lea::DRAIN_LIFE),   // 3ED 106
    PrintingRecord::reprint(&catalog_lea::DRUDGE_SKELETONS), // 3ED 107
    PrintingRecord::reprint(&catalog_arn::EL_HAJJAJ),    // 3ED 108
    PrintingRecord::reprint(&catalog_arn::ERG_RAIDERS),  // 3ED 109
    PrintingRecord::reprint(&catalog_lea::EVIL_PRESENCE), // 3ED 110
    PrintingRecord::reprint(&catalog_lea::FEAR),         // 3ED 111
    PrintingRecord::reprint(&catalog_lea::FROZEN_SHADE), // 3ED 112
    PrintingRecord::reprint(&catalog_lea::GLOOM),        // 3ED 113
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND), // 3ED 114
    PrintingRecord::reprint(&catalog_lea::HYPNOTIC_SPECTER), // 3ED 115
    PrintingRecord::reprint(&catalog_lea::LORD_OF_THE_PIT), // 3ED 116
    PrintingRecord::reprint(&catalog_lea::MIND_TWIST),   // 3ED 117
    PrintingRecord::reprint(&catalog_lea::NETHER_SHADOW), // 3ED 118
    PrintingRecord::reprint(&catalog_lea::NETTLING_IMP), // 3ED 119
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),    // 3ED 120
    PrintingRecord::reprint(&catalog_lea::PARALYZE),     // 3ED 121
    PrintingRecord::reprint(&catalog_lea::PESTILENCE),   // 3ED 122
    PrintingRecord::reprint(&catalog_lea::PLAGUE_RATS),  // 3ED 123
    PrintingRecord::reprint(&catalog_lea::RAISE_DEAD),   // 3ED 124
    PrintingRecord::reprint(&catalog_lea::ROYAL_ASSASSIN), // 3ED 125
    PrintingRecord::reprint(&catalog_lea::SACRIFICE),    // 3ED 126
    PrintingRecord::reprint(&catalog_lea::SCATHE_ZOMBIES), // 3ED 127
    PrintingRecord::reprint(&catalog_lea::SCAVENGING_GHOUL), // 3ED 128
    PrintingRecord::reprint(&catalog_lea::SENGIR_VAMPIRE), // 3ED 129
    PrintingRecord::reprint(&catalog_lea::SIMULACRUM),   // 3ED 130
    PrintingRecord::reprint(&catalog_arn::SORCERESS_QUEEN), // 3ED 131
    PrintingRecord::reprint(&catalog_lea::TERROR),       // 3ED 132
    PrintingRecord::reprint(&catalog_lea::UNHOLY_STRENGTH), // 3ED 133
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BONE), // 3ED 134
    PrintingRecord::reprint(&catalog_lea::WARP_ARTIFACT), // 3ED 135
    PrintingRecord::reprint(&catalog_lea::WEAKNESS),     // 3ED 136
    PrintingRecord::reprint(&catalog_lea::WILL_O_THE_WISP), // 3ED 137
    PrintingRecord::reprint(&catalog_lea::ZOMBIE_MASTER), // 3ED 138
    PrintingRecord::reprint(&catalog_atq::ATOG),         // 3ED 139
    PrintingRecord::reprint(&catalog_lea::BURROWING),    // 3ED 140
    PrintingRecord::reprint(&catalog_lea::CHAOSLACE),    // 3ED 141
    PrintingRecord::reprint(&catalog_lea::DISINTEGRATE), // 3ED 142
    PrintingRecord::reprint(&catalog_lea::DRAGON_WHELP), // 3ED 143
    PrintingRecord::reprint(&catalog_lea::DWARVEN_WARRIORS), // 3ED 144
    PrintingRecord::reprint(&catalog_atq::DWARVEN_WEAPONSMITH), // 3ED 145
    PrintingRecord::reprint(&catalog_lea::EARTH_ELEMENTAL), // 3ED 146
    PrintingRecord::reprint(&catalog_lea::EARTHBIND),    // 3ED 147
    PrintingRecord::reprint(&catalog_lea::EARTHQUAKE),   // 3ED 148
    PrintingRecord::reprint(&catalog_lea::FIRE_ELEMENTAL), // 3ED 149
    PrintingRecord::reprint(&catalog_lea::FIREBALL),     // 3ED 150
    PrintingRecord::reprint(&catalog_lea::FIREBREATHING), // 3ED 151
    PrintingRecord::reprint(&catalog_lea::FLASHFIRES),   // 3ED 152
    PrintingRecord::reprint(&catalog_lea::FORK),         // 3ED 153
    PrintingRecord::reprint(&catalog_lea::GOBLIN_BALLOON_BRIGADE), // 3ED 154
    PrintingRecord::reprint(&catalog_lea::GOBLIN_KING),  // 3ED 155
    PrintingRecord::reprint(&catalog_lea::GRANITE_GARGOYLE), // 3ED 156
    PrintingRecord::reprint(&catalog_lea::GRAY_OGRE),    // 3ED 157
    PrintingRecord::reprint(&catalog_lea::HILL_GIANT),   // 3ED 158
    PrintingRecord::reprint(&catalog_lea::HURLOON_MINOTAUR), // 3ED 159
    PrintingRecord::reprint(&catalog_lea::KELDON_WARLORD), // 3ED 160
    PrintingRecord::reprint(&catalog_arn::KIRD_APE),     // 3ED 161
    PrintingRecord::reprint(&catalog_lea::LIGHTNING_BOLT), // 3ED 162
    PrintingRecord::reprint(&catalog_arn::MAGNETIC_MOUNTAIN), // 3ED 163
    PrintingRecord::reprint(&catalog_lea::MANA_FLARE),   // 3ED 164
    PrintingRecord::reprint(&catalog_lea::MANABARBS),    // 3ED 165
    PrintingRecord::reprint(&catalog_arn::MIJAE_DJINN),  // 3ED 166
    PrintingRecord::reprint(&catalog_lea::MONSS_GOBLIN_RAIDERS), // 3ED 167
    PrintingRecord::reprint(&catalog_lea::ORCISH_ARTILLERY), // 3ED 168
    PrintingRecord::reprint(&catalog_lea::ORCISH_ORIFLAMME), // 3ED 169
    PrintingRecord::reprint(&catalog_lea::POWER_SURGE),  // 3ED 170
    PrintingRecord::reprint(&catalog_lea::RED_ELEMENTAL_BLAST), // 3ED 171
    PrintingRecord::reprint(&catalog_lea::ROC_OF_KHER_RIDGES), // 3ED 172
    PrintingRecord::reprint(&catalog_lea::ROCK_HYDRA),   // 3ED 173
    PrintingRecord::reprint(&catalog_lea::SEDGE_TROLL),  // 3ED 174
    PrintingRecord::reprint(&catalog_lea::SHATTER),      // 3ED 175
    PrintingRecord::reprint(&catalog_atq::SHATTERSTORM), // 3ED 176
    PrintingRecord::reprint(&catalog_lea::SHIVAN_DRAGON), // 3ED 177
    PrintingRecord::reprint(&catalog_lea::SMOKE),        // 3ED 178
    PrintingRecord::reprint(&catalog_lea::STONE_GIANT),  // 3ED 179
    PrintingRecord::reprint(&catalog_lea::STONE_RAIN),   // 3ED 180
    PrintingRecord::reprint(&catalog_lea::TUNNEL),       // 3ED 181
    PrintingRecord::reprint(&catalog_lea::UTHDEN_TROLL), // 3ED 182
    PrintingRecord::reprint(&catalog_lea::WALL_OF_FIRE), // 3ED 183
    PrintingRecord::reprint(&catalog_lea::WALL_OF_STONE), // 3ED 184
    PrintingRecord::reprint(&catalog_lea::WHEEL_OF_FORTUNE), // 3ED 185
    PrintingRecord::reprint(&catalog_lea::ASPECT_OF_WOLF), // 3ED 186
    PrintingRecord::reprint(&catalog_lea::BIRDS_OF_PARADISE), // 3ED 187
    PrintingRecord::reprint(&catalog_lea::CHANNEL),      // 3ED 188
    PrintingRecord::reprint(&catalog_lea::COCKATRICE),   // 3ED 189
    PrintingRecord::reprint(&catalog_lea::CRAW_WURM),    // 3ED 190
    PrintingRecord::reprint(&catalog_atq::CRUMBLE),      // 3ED 191
    PrintingRecord::reprint(&catalog_arn::DESERT_TWISTER), // 3ED 192
    PrintingRecord::reprint(&catalog_lea::ELVISH_ARCHERS), // 3ED 193
    PrintingRecord::reprint(&catalog_lea::FASTBOND),     // 3ED 194
    PrintingRecord::reprint(&catalog_lea::FOG),          // 3ED 195
    PrintingRecord::reprint(&catalog_lea::FORCE_OF_NATURE), // 3ED 196
    PrintingRecord::reprint(&catalog_lea::FUNGUSAUR),    // 3ED 197
    PrintingRecord::reprint(&catalog_lea::GAEA_S_LIEGE), // 3ED 198
    PrintingRecord::reprint(&catalog_lea::GIANT_GROWTH), // 3ED 199
    PrintingRecord::reprint(&catalog_lea::GIANT_SPIDER), // 3ED 200
    PrintingRecord::reprint(&catalog_lea::GRIZZLY_BEARS), // 3ED 201
    PrintingRecord::reprint(&catalog_lea::HURRICANE),    // 3ED 202
    PrintingRecord::reprint(&catalog_lea::INSTILL_ENERGY), // 3ED 203
    PrintingRecord::reprint(&catalog_lea::IRONROOT_TREEFOLK), // 3ED 204
    PrintingRecord::reprint(&catalog_lea::KUDZU),        // 3ED 205
    PrintingRecord::reprint(&catalog_lea::LEY_DRUID),    // 3ED 206
    PrintingRecord::reprint(&catalog_lea::LIFEFORCE),    // 3ED 207
    PrintingRecord::reprint(&catalog_lea::LIFELACE),     // 3ED 208
    PrintingRecord::reprint(&catalog_lea::LIVING_ARTIFACT), // 3ED 209
    PrintingRecord::reprint(&catalog_lea::LIVING_LANDS), // 3ED 210
    PrintingRecord::reprint(&catalog_lea::LLANOWAR_ELVES), // 3ED 211
    PrintingRecord::reprint(&catalog_lea::LURE),         // 3ED 212
    PrintingRecord::reprint(&catalog_lea::REGENERATION), // 3ED 213
    PrintingRecord::reprint(&catalog_lea::REGROWTH),     // 3ED 214
    PrintingRecord::reprint(&catalog_lea::SCRYB_SPRITES), // 3ED 215
    PrintingRecord::reprint(&catalog_lea::SHANODIN_DRYADS), // 3ED 216
    PrintingRecord::reprint(&catalog_lea::STREAM_OF_LIFE), // 3ED 217
    PrintingRecord::reprint(&catalog_lea::THICKET_BASILISK), // 3ED 218
    PrintingRecord::reprint(&catalog_lea::TIMBER_WOLVES), // 3ED 219
    PrintingRecord::reprint(&catalog_atq::TITANIA_S_SONG), // 3ED 220
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),  // 3ED 221
    PrintingRecord::reprint(&catalog_lea::TSUNAMI),      // 3ED 222
    PrintingRecord::reprint(&catalog_lea::VERDURAN_ENCHANTRESS), // 3ED 223
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BRAMBLES), // 3ED 224
    PrintingRecord::reprint(&catalog_lea::WALL_OF_ICE),  // 3ED 225
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WOOD), // 3ED 226
    PrintingRecord::reprint(&catalog_lea::WANDERLUST),   // 3ED 227
    PrintingRecord::reprint(&catalog_lea::WAR_MAMMOTH),  // 3ED 228
    PrintingRecord::reprint(&catalog_lea::WEB),          // 3ED 229
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),  // 3ED 230
    PrintingRecord::reprint(&catalog_arn::ALADDIN_S_LAMP), // 3ED 231
    PrintingRecord::reprint(&catalog_arn::ALADDINS_RING), // 3ED 232
    PrintingRecord::reprint(&catalog_lea::ANKH_OF_MISHRA), // 3ED 233
    PrintingRecord::reprint(&catalog_atq::ARMAGEDDON_CLOCK), // 3ED 234
    PrintingRecord::reprint(&catalog_lea::BASALT_MONOLITH), // 3ED 235
    PrintingRecord::reprint(&catalog_lea::BLACK_VISE),   // 3ED 236
    PrintingRecord::reprint(&catalog_arn::BOTTLE_OF_SULEIMAN), // 3ED 237
    PrintingRecord::reprint(&catalog_arn::BRASS_MAN),    // 3ED 238
    PrintingRecord::reprint(&catalog_lea::CELESTIAL_PRISM), // 3ED 239
    PrintingRecord::reprint(&catalog_lea::CLOCKWORK_BEAST), // 3ED 240
    PrintingRecord::reprint(&catalog_lea::CONSERVATOR),  // 3ED 241
    PrintingRecord::reprint(&catalog_lea::CRYSTAL_ROD),  // 3ED 242
    PrintingRecord::reprint(&catalog_arn::DANCING_SCIMITAR), // 3ED 243
    PrintingRecord::reprint(&catalog_lea::DINGUS_EGG),   // 3ED 244
    PrintingRecord::reprint(&catalog_lea::DISRUPTING_SCEPTER), // 3ED 245
    PrintingRecord::reprint(&catalog_atq::DRAGON_ENGINE), // 3ED 246
    PrintingRecord::reprint(&catalog_arn::EBONY_HORSE),  // 3ED 247
    PrintingRecord::reprint(&catalog_arn::FLYING_CARPET), // 3ED 248
    PrintingRecord::reprint(&catalog_lea::GLASSES_OF_URZA), // 3ED 249
    PrintingRecord::reprint(&catalog_lea::HELM_OF_CHATZUK), // 3ED 250
    PrintingRecord::reprint(&catalog_lea::HOWLING_MINE), // 3ED 251
    PrintingRecord::reprint(&catalog_lea::IRON_STAR),    // 3ED 252
    PrintingRecord::reprint(&catalog_lea::IVORY_CUP),    // 3ED 253
    PrintingRecord::reprint(&catalog_atq::IVORY_TOWER),  // 3ED 254
    PrintingRecord::reprint(&catalog_lea::JADE_MONOLITH), // 3ED 255
    PrintingRecord::reprint(&catalog_arn::JANDOR_S_RING), // 3ED 256
    PrintingRecord::reprint(&catalog_arn::JANDORS_SADDLEBAGS), // 3ED 257
    PrintingRecord::reprint(&catalog_lea::JAYEMDAE_TOME), // 3ED 258
    PrintingRecord::reprint(&catalog_lea::JUGGERNAUT),   // 3ED 259
    PrintingRecord::reprint(&catalog_lea::KORMUS_BELL),  // 3ED 260
    PrintingRecord::reprint(&catalog_lea::LIBRARY_OF_LENG), // 3ED 261
    PrintingRecord::reprint(&catalog_lea::LIVING_WALL),  // 3ED 262
    PrintingRecord::reprint(&catalog_lea::MANA_VAULT),   // 3ED 263
    PrintingRecord::reprint(&catalog_lea::MEEKSTONE),    // 3ED 264
    PrintingRecord::reprint(&catalog_atq::MILLSTONE),    // 3ED 265
    PrintingRecord::reprint(&catalog_atq::MISHRA_S_WAR_MACHINE), // 3ED 266
    PrintingRecord::reprint(&catalog_lea::NEVINYRRALS_DISK), // 3ED 267
    PrintingRecord::reprint(&catalog_lea::OBSIANUS_GOLEM), // 3ED 268
    PrintingRecord::reprint(&catalog_atq::ONULET),       // 3ED 269
    PrintingRecord::reprint(&catalog_atq::ORNITHOPTER),  // 3ED 270
    PrintingRecord::reprint(&catalog_atq::PRIMAL_CLAY),  // 3ED 271
    PrintingRecord::reprint(&catalog_atq::ROCKET_LAUNCHER), // 3ED 272
    PrintingRecord::reprint(&catalog_lea::ROD_OF_RUIN),  // 3ED 273
    PrintingRecord::reprint(&catalog_lea::SOL_RING),     // 3ED 274
    PrintingRecord::reprint(&catalog_lea::SOUL_NET),     // 3ED 275
    PrintingRecord::reprint(&catalog_lea::SUNGLASSES_OF_URZA), // 3ED 276
    PrintingRecord::reprint(&catalog_lea::THE_HIVE),     // 3ED 277
    PrintingRecord::reprint(&catalog_atq::THE_RACK),     // 3ED 278
    PrintingRecord::reprint(&catalog_lea::THRONE_OF_BONE), // 3ED 279
    PrintingRecord::reprint(&catalog_lea::WINTER_ORB),   // 3ED 280
    PrintingRecord::reprint(&catalog_lea::WOODEN_SPHERE), // 3ED 281
    PrintingRecord::reprint(&catalog_lea::BADLANDS),     // 3ED 282
    PrintingRecord::reprint(&catalog_lea::BAYOU),        // 3ED 283
    PrintingRecord::reprint(&catalog_lea::PLATEAU),      // 3ED 284
    PrintingRecord::reprint(&catalog_lea::SAVANNAH),     // 3ED 285
    PrintingRecord::reprint(&catalog_lea::SCRUBLAND),    // 3ED 286
    PrintingRecord::reprint(&catalog_lea::TAIGA),        // 3ED 287
    PrintingRecord::reprint(&catalog_lea::TROPICAL_ISLAND), // 3ED 288
    PrintingRecord::reprint(&catalog_lea::TUNDRA),       // 3ED 289
    PrintingRecord::reprint(&catalog_lea::UNDERGROUND_SEA), // 3ED 290
    PrintingRecord::reprint(&catalog_leb::VOLCANIC_ISLAND), // 3ED 291
    PrintingRecord::reprint(&catalog_lea::PLAINS),       // 3ED 292
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),  // 3ED 293
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),  // 3ED 294
    PrintingRecord::reprint(&catalog_lea::ISLAND),       // 3ED 295
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),  // 3ED 296
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),  // 3ED 297
    PrintingRecord::reprint(&catalog_lea::SWAMP),        // 3ED 298
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),   // 3ED 299
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),   // 3ED 300
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),     // 3ED 301
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1), // 3ED 302
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2), // 3ED 303
    PrintingRecord::reprint(&catalog_lea::FOREST),       // 3ED 304
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),  // 3ED 305
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),  // 3ED 306
];
