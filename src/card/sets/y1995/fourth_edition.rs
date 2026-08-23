//! Fourth Edition has no unique card definitions.
//!
//! It is the set the Premodern window opens on, so a card whose only earlier
//! printings predate that window becomes legal here.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::alpha;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1994::antiquities;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1994::the_dark as catalog_drk;

// 4ED 1 — Alabaster Potion (reprint)

// 4ED 2 — Amrou Kithkin (reprint)

// 4ED 3 — Angry Mob (reprint)

// 4ED 4 — Animate Wall (reprint)

// 4ED 5 — Armageddon (reprint)

// 4ED 6 — Balance (reprint)

// 4ED 7 — Benalish Hero (reprint)

// 4ED 8 — Black Ward (reprint)

// 4ED 9 — Blessing (reprint)

// 4ED 10 — Blue Ward (reprint)

// 4ED 11 — Brainwash (reprint)

// 4ED 12 — Castle (reprint)

// 4ED 13 — Circle of Protection: Artifacts (reprint)

// 4ED 14 — Circle of Protection: Black (reprint)

// 4ED 15 — Circle of Protection: Blue (reprint)

// 4ED 16 — Circle of Protection: Green (reprint)

// 4ED 17 — Circle of Protection: Red (reprint)

// 4ED 18 — Circle of Protection: White (reprint)

// 4ED 19 — Conversion (reprint)

// 4ED 20 — Crusade (reprint)

// 4ED 21 — Death Ward (reprint)

// 4ED 22 — Disenchant (reprint)

// 4ED 23 — Divine Transformation (reprint)

// 4ED 24 — Elder Land Wurm (reprint)

// 4ED 25 — Eye for an Eye (reprint)

// 4ED 26 — Fortified Area (reprint)

// 4ED 27 — Green Ward (reprint)

// 4ED 28 — Healing Salve (reprint)

// 4ED 29 — Holy Armor (reprint)

// 4ED 30 — Holy Strength (reprint)

// 4ED 31 — Island Sanctuary (reprint)

// 4ED 32 — Karma (reprint)

// 4ED 33 — Kismet (reprint)

// 4ED 34 — Land Tax (reprint)

// 4ED 35 — Mesa Pegasus (reprint)

// 4ED 36 — Morale (reprint)

// 4ED 37 — Northern Paladin (reprint)

// 4ED 38 — Osai Vultures (reprint)

// 4ED 39 — Pearled Unicorn (reprint)

// 4ED 40 — Personal Incarnation (reprint)

// 4ED 41 — Piety (reprint)

// 4ED 42 — Pikemen (reprint)

// 4ED 43 — Purelace (reprint)

// 4ED 44 — Red Ward (reprint)

// 4ED 45 — Reverse Damage (reprint)

// 4ED 46 — Righteousness (reprint)

// 4ED 47 — Samite Healer (reprint)

// 4ED 48 — Savannah Lions (reprint)

// 4ED 49 — Seeker (reprint)

// 4ED 50 — Serra Angel (reprint)

// 4ED 51 — Spirit Link (reprint)

// 4ED 52 — Swords to Plowshares (reprint)

// 4ED 53 — Tundra Wolves (reprint)

// 4ED 54 — Visions (reprint)

// 4ED 55 — Wall of Swords (reprint)

// 4ED 56 — White Knight (reprint)

// 4ED 57 — White Ward (reprint)

// 4ED 58 — Wrath of God (reprint)

// 4ED 59 — Air Elemental (reprint)

// 4ED 60 — Animate Artifact (reprint)

// 4ED 61 — Apprentice Wizard (reprint)

// 4ED 62 — Backfire (reprint)

// 4ED 63 — Blue Elemental Blast (reprint)

// 4ED 64 — Control Magic (reprint)

// 4ED 65 — Counterspell (reprint)

// 4ED 66 — Creature Bond (reprint)

// 4ED 67 — Drain Power (reprint)

// 4ED 68 — Energy Flux (reprint)

// 4ED 69 — Energy Tap (reprint)

// 4ED 70 — Erosion (reprint)

// 4ED 71 — Feedback (reprint)

// 4ED 72 — Flight (reprint)

// 4ED 73 — Flood (reprint)

// 4ED 74 — Gaseous Form (reprint)

// 4ED 75 — Ghost Ship (reprint)

// 4ED 76 — Giant Tortoise (reprint)

// 4ED 77 — Hurkyl's Recall (reprint)

// 4ED 78 — Island Fish Jasconius (reprint)

// 4ED 79 — Jump (reprint)

// 4ED 80 — Leviathan (reprint)

// 4ED 81 — Lifetap (reprint)

// 4ED 82 — Lord of Atlantis (reprint)

// 4ED 83 — Magical Hack (reprint)

// 4ED 84 — Mahamoti Djinn (reprint)

// 4ED 85 — Mana Short (reprint)

// 4ED 86 — Merfolk of the Pearl Trident (reprint)

// 4ED 87 — Mind Bomb (reprint)

// 4ED 88 — Phantasmal Forces (reprint)

// 4ED 89 — Phantasmal Terrain (reprint)

// 4ED 90 — Phantom Monster (reprint)

// 4ED 91 — Pirate Ship (reprint)

// 4ED 92 — Power Leak (reprint)

// 4ED 93 — Power Sink (reprint)

// 4ED 94 — Prodigal Sorcerer (reprint)

// 4ED 95 — Psionic Entity (reprint)

// 4ED 96 — Psychic Venom (reprint)

// 4ED 97 — Relic Bind (reprint)

// 4ED 98 — Sea Serpent (reprint)

// 4ED 99 — Segovian Leviathan (reprint)

// 4ED 100 — Sindbad (reprint)

// 4ED 101 — Siren's Call (reprint)

// 4ED 102 — Sleight of Mind (reprint)

// 4ED 103 — Spell Blast (reprint)

// 4ED 104 — Stasis (reprint)

// 4ED 105 — Steal Artifact (reprint)

// 4ED 106 — Sunken City (reprint)

// 4ED 107 — Thoughtlace (reprint)

// 4ED 107† — Drudge Skeletons (reprint)

// 4ED 108 — Time Elemental (reprint)

// 4ED 109 — Twiddle (reprint)

// 4ED 110 — Unstable Mutation (reprint)

// 4ED 111 — Unsummon (reprint)

// 4ED 112 — Volcanic Eruption (reprint)

// 4ED 113 — Wall of Air (reprint)

// 4ED 114 — Wall of Water (reprint)

// 4ED 115 — Water Elemental (reprint)

// 4ED 116 — Zephyr Falcon (reprint)

// 4ED 117 — Abomination (reprint)

// 4ED 118 — Animate Dead (reprint)

// 4ED 119 — Ashes to Ashes (reprint)

// 4ED 120 — Bad Moon (reprint)

// 4ED 121 — Black Knight (reprint)

// 4ED 122 — Blight (reprint)

// 4ED 123 — Bog Imp (reprint)

// 4ED 124 — Bog Wraith (reprint)

// 4ED 125 — Carrion Ants (reprint)

// 4ED 126 — Cosmic Horror (reprint)

// 4ED 127 — Cursed Land (reprint)

// 4ED 128 — Cyclopean Mummy (reprint)

// 4ED 129 — Dark Ritual (reprint)

// 4ED 130 — Deathgrip (reprint)

// 4ED 131 — Deathlace (reprint)

// 4ED 132 — Drain Life (reprint)

// 4ED 133 — Drudge Skeletons (alternate printing)

// 4ED 134 — El-Hajjâj (reprint)

// 4ED 134† — El-Hajjâj (alternate printing)

// 4ED 135 — Erg Raiders (reprint)

// 4ED 136 — Evil Presence (reprint)

// 4ED 137 — Fear (reprint)

// 4ED 138 — Frozen Shade (reprint)

// 4ED 139 — Gloom (reprint)

// 4ED 140 — Greed (reprint)

// 4ED 141 — Howl from Beyond (reprint)

// 4ED 142 — Hypnotic Specter (reprint)

// 4ED 143 — Junún Efreet (reprint)

// 4ED 144 — Lord of the Pit (reprint)

// 4ED 145 — Lost Soul (reprint)

// 4ED 146 — Marsh Gas (reprint)

// 4ED 147 — Mind Twist (reprint)

// 4ED 148 — Murk Dwellers (reprint)

// 4ED 149 — Nether Shadow (reprint)

// 4ED 150 — Nightmare (reprint)

// 4ED 151 — Paralyze (reprint)

// 4ED 152 — Pestilence (reprint)

// 4ED 153 — Pit Scorpion (reprint)

// 4ED 154 — Plague Rats (reprint)

// 4ED 155 — Rag Man (reprint)

// 4ED 156 — Raise Dead (reprint)

// 4ED 157 — Royal Assassin (reprint)

// 4ED 158 — Scathe Zombies (reprint)

// 4ED 159 — Scavenging Ghoul (reprint)

// 4ED 160 — Sengir Vampire (reprint)

// 4ED 161 — Simulacrum (reprint)

// 4ED 162 — Sorceress Queen (reprint)

// 4ED 163 — Spirit Shackle (reprint)

// 4ED 164 — Terror (reprint)

// 4ED 165 — Uncle Istvan (reprint)

// 4ED 166 — Unholy Strength (reprint)

// 4ED 167 — Vampire Bats (reprint)

// 4ED 168 — Wall of Bone (reprint)

// 4ED 169 — Warp Artifact (reprint)

// 4ED 170 — Weakness (reprint)

// 4ED 171 — Will-o'-the-Wisp (reprint)

// 4ED 172 — Word of Binding (reprint)

// 4ED 173 — Xenic Poltergeist (reprint)

// 4ED 174 — Zombie Master (reprint)

// 4ED 175 — Ali Baba (reprint)

// 4ED 176 — Ball Lightning (reprint)

// 4ED 177 — Bird Maiden (reprint)

// 4ED 178 — Blood Lust (reprint)

// 4ED 179 — Brothers of Fire (reprint)

// 4ED 180 — Burrowing (reprint)

// 4ED 181 — Cave People (reprint)

// 4ED 182 — Chaoslace (reprint)

// 4ED 183 — Crimson Manticore (reprint)

// 4ED 184 — Detonate (reprint)

// 4ED 185 — Disintegrate (reprint)

// 4ED 186 — Dragon Whelp (reprint)

// 4ED 187 — Dwarven Warriors (reprint)

// 4ED 188 — Earth Elemental (reprint)

// 4ED 189 — Earthquake (reprint)

// 4ED 190 — Eternal Warrior (reprint)

// 4ED 191 — Fire Elemental (reprint)

// 4ED 192 — Fireball (reprint)

// 4ED 193 — Firebreathing (reprint)

// 4ED 194 — Fissure (reprint)

// 4ED 195 — Flashfires (reprint)

// 4ED 196 — Giant Strength (reprint)

// 4ED 197 — Goblin Balloon Brigade (reprint)

// 4ED 198 — Goblin King (reprint)

// 4ED 199 — Goblin Rock Sled (reprint)

// 4ED 200 — Gray Ogre (reprint)

// 4ED 201 — Hill Giant (reprint)

// 4ED 202 — Hurloon Minotaur (reprint)

// 4ED 203 — Hurr Jackal (reprint)

// 4ED 204 — Immolation (reprint)

// 4ED 205 — Inferno (reprint)

// 4ED 206 — Ironclaw Orcs (reprint)

// 4ED 207 — Keldon Warlord (reprint)

// 4ED 208 — Lightning Bolt (reprint)

// 4ED 209 — Magnetic Mountain (reprint)

// 4ED 210 — Mana Clash (reprint)

// 4ED 211 — Mana Flare (reprint)

// 4ED 212 — Manabarbs (reprint)

// 4ED 213 — Mons's Goblin Raiders (reprint)

// 4ED 214 — Orcish Artillery (reprint)

// 4ED 215 — Orcish Oriflamme (reprint)

// 4ED 216 — Power Surge (reprint)

// 4ED 217 — Pyrotechnics (reprint)

// 4ED 218 — Red Elemental Blast (reprint)

// 4ED 219 — Shatter (reprint)

// 4ED 220 — Shivan Dragon (reprint)

// 4ED 221 — Sisters of the Flame (reprint)

// 4ED 222 — Smoke (reprint)

// 4ED 223 — Stone Giant (reprint)

// 4ED 224 — Stone Rain (reprint)

// 4ED 225 — Tempest Efreet (reprint)

// 4ED 226 — The Brute (reprint)

// 4ED 227 — Tunnel (reprint)

// 4ED 228 — Uthden Troll (reprint)

// 4ED 229 — Wall of Dust (reprint)

// 4ED 230 — Wall of Fire (reprint)

// 4ED 231 — Wall of Stone (reprint)

// 4ED 232 — Winds of Change (reprint)

// 4ED 233 — Aspect of Wolf (reprint)

// 4ED 234 — Birds of Paradise (reprint)

// 4ED 235 — Carnivorous Plant (reprint)

// 4ED 236 — Channel (reprint)

// 4ED 237 — Cockatrice (reprint)

// 4ED 238 — Craw Wurm (reprint)

// 4ED 239 — Crumble (reprint)

// 4ED 240 — Desert Twister (reprint)

// 4ED 241 — Durkwood Boars (reprint)

// 4ED 242 — Elven Riders (reprint)

// 4ED 243 — Elvish Archers (reprint)

// 4ED 244 — Fog (reprint)

// 4ED 245 — Force of Nature (reprint)

// 4ED 246 — Fungusaur (reprint)

// 4ED 247 — Gaea's Liege (reprint)

// 4ED 248 — Giant Growth (reprint)

// 4ED 249 — Giant Spider (reprint)

// 4ED 250 — Grizzly Bears (reprint)

// 4ED 251 — Hurricane (reprint)

// 4ED 252 — Instill Energy (reprint)

// 4ED 253 — Ironroot Treefolk (reprint)

// 4ED 254 — Killer Bees (reprint)

// 4ED 255 — Land Leeches (reprint)

// 4ED 256 — Ley Druid (reprint)

// 4ED 257 — Lifeforce (reprint)

// 4ED 258 — Lifelace (reprint)

// 4ED 259 — Living Artifact (reprint)

// 4ED 260 — Living Lands (reprint)

// 4ED 261 — Llanowar Elves (reprint)

// 4ED 262 — Lure (reprint)

// 4ED 263 — Marsh Viper (reprint)

// 4ED 264 — Nafs Asp (reprint)

// 4ED 265 — Pradesh Gypsies (reprint)

// 4ED 266 — Radjan Spirit (reprint)

// 4ED 267 — Rebirth (reprint)

// 4ED 268 — Regeneration (reprint)

// 4ED 269 — Sandstorm (reprint)

// 4ED 270 — Scryb Sprites (reprint)

// 4ED 271 — Shanodin Dryads (reprint)

// 4ED 272 — Stream of Life (reprint)

// 4ED 273 — Sylvan Library (reprint)

// 4ED 274 — Thicket Basilisk (reprint)

// 4ED 275 — Timber Wolves (reprint)

// 4ED 276 — Titania's Song (reprint)

// 4ED 277 — Tranquility (reprint)

// 4ED 278 — Tsunami (reprint)

// 4ED 279 — Untamed Wilds (reprint)

// 4ED 280 — Venom (reprint)

// 4ED 281 — Verduran Enchantress (reprint)

// 4ED 282 — Wall of Brambles (reprint)

// 4ED 283 — Wall of Ice (reprint)

// 4ED 284 — Wall of Wood (reprint)

// 4ED 285 — Wanderlust (reprint)

// 4ED 286 — War Mammoth (reprint)

// 4ED 287 — Web (reprint)

// 4ED 288 — Whirling Dervish (reprint)

// 4ED 289 — Wild Growth (reprint)

// 4ED 290 — Winter Blast (reprint)

// 4ED 291 — Aladdin's Lamp (reprint)

// 4ED 292 — Aladdin's Ring (reprint)

// 4ED 293 — Amulet of Kroog (reprint)

// 4ED 294 — Ankh of Mishra (reprint)

// 4ED 295 — Armageddon Clock (reprint)

// 4ED 296 — Ashnod's Battle Gear (reprint)

// 4ED 297 — Battering Ram (reprint)

// 4ED 298 — Black Mana Battery (reprint)

// 4ED 299 — Black Vise (reprint)

// 4ED 300 — Blue Mana Battery (reprint)

// 4ED 301 — Bottle of Suleiman (reprint)

// 4ED 302 — Brass Man (reprint)

// 4ED 303 — Bronze Tablet (reprint)

// 4ED 304 — Celestial Prism (reprint)

// 4ED 305 — Clay Statue (reprint)

// 4ED 306 — Clockwork Avian (reprint)

// 4ED 307 — Clockwork Beast (reprint)

// 4ED 308 — Colossus of Sardia (reprint)

// 4ED 309 — Conservator (reprint)

// 4ED 310 — Coral Helm (reprint)

// 4ED 311 — Crystal Rod (reprint)

// 4ED 312 — Cursed Rack (reprint)

// 4ED 313 — Dancing Scimitar (reprint)

// 4ED 314 — Diabolic Machine (reprint)

// 4ED 315 — Dingus Egg (reprint)

// 4ED 316 — Disrupting Scepter (reprint)

// 4ED 317 — Dragon Engine (reprint)

// 4ED 318 — Ebony Horse (reprint)

// 4ED 319 — Fellwar Stone (reprint)

// 4ED 320 — Flying Carpet (reprint)

// 4ED 321 — Glasses of Urza (reprint)

// 4ED 322 — Grapeshot Catapult (reprint)

// 4ED 323 — Green Mana Battery (reprint)

// 4ED 324 — Helm of Chatzuk (reprint)

// 4ED 325 — Howling Mine (reprint)

// 4ED 326 — Iron Star (reprint)

// 4ED 327 — Ivory Cup (reprint)

// 4ED 328 — Ivory Tower (reprint)

// 4ED 329 — Jade Monolith (reprint)

// 4ED 330 — Jandor's Saddlebags (reprint)

// 4ED 331 — Jayemdae Tome (reprint)

// 4ED 332 — Kormus Bell (reprint)

// 4ED 333 — Library of Leng (reprint)

// 4ED 334 — Mana Vault (reprint)

// 4ED 335 — Meekstone (reprint)

// 4ED 336 — Millstone (reprint)

// 4ED 337 — Mishra's War Machine (reprint)

// 4ED 338 — Nevinyrral's Disk (reprint)

// 4ED 339 — Obsianus Golem (reprint)

// 4ED 340 — Onulet (reprint)

// 4ED 341 — Ornithopter (reprint)

// 4ED 342 — Primal Clay (reprint)

// 4ED 343 — Red Mana Battery (reprint)

// 4ED 344 — Rod of Ruin (reprint)

// 4ED 345 — Shapeshifter (reprint)

// 4ED 346 — Soul Net (reprint)

// 4ED 347 — Sunglasses of Urza (reprint)

// 4ED 348 — Tawnos's Wand (reprint)

// 4ED 349 — Tawnos's Weaponry (reprint)

// 4ED 350 — Tetravus (reprint)

// 4ED 351 — The Hive (reprint)

// 4ED 352 — The Rack (reprint)

// 4ED 353 — Throne of Bone (reprint)

// 4ED 354 — Triskelion (reprint)

// 4ED 355 — Urza's Avenger (reprint)

// 4ED 356 — Wall of Spears (reprint)

// 4ED 357 — White Mana Battery (reprint)

// 4ED 358 — Winter Orb (reprint)

// 4ED 359 — Wooden Sphere (reprint)

// 4ED 360 — Yotian Soldier (reprint)

// 4ED 361 — Mishra's Factory (reprint)

// 4ED 362 — Oasis (reprint)

// 4ED 363 — Strip Mine (reprint)

// 4ED 364 — Plains (reprint)

// 4ED 365 — Plains (alternate printing)

// 4ED 366 — Plains (alternate printing)

// 4ED 367 — Island (reprint)

// 4ED 368 — Island (alternate printing)

// 4ED 369 — Island (alternate printing)

// 4ED 370 — Swamp (reprint)

// 4ED 371 — Swamp (alternate printing)

// 4ED 372 — Swamp (alternate printing)

// 4ED 373 — Mountain (reprint)

// 4ED 374 — Mountain (alternate printing)

// 4ED 375 — Mountain (alternate printing)

// 4ED 376 — Forest (reprint)

// 4ED 377 — Forest (alternate printing)

// 4ED 378 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_leg::ALABASTER_POTION), // 4ED 1
    PrintingRecord::reprint(&catalog_leg::AMROU_KITHKIN),    // 4ED 2
    PrintingRecord::reprint(&catalog_drk::ANGRY_MOB),        // 4ED 3
    PrintingRecord::reprint(&catalog_lea::ANIMATE_WALL),     // 4ED 4
    PrintingRecord::reprint(&alpha::ARMAGEDDON),             // 4ED 5
    PrintingRecord::reprint(&catalog_lea::BALANCE),          // 4ED 6
    PrintingRecord::reprint(&catalog_lea::BENALISH_HERO),    // 4ED 7
    PrintingRecord::reprint(&catalog_lea::BLACK_WARD),       // 4ED 8
    PrintingRecord::reprint(&catalog_lea::BLESSING),         // 4ED 9
    PrintingRecord::reprint(&catalog_lea::BLUE_WARD),        // 4ED 10
    PrintingRecord::reprint(&catalog_drk::BRAINWASH),        // 4ED 11
    PrintingRecord::reprint(&catalog_lea::CASTLE),           // 4ED 12
    PrintingRecord::reprint(&catalog_atq::CIRCLE_OF_PROTECTION_ARTIFACTS), // 4ED 13
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // 4ED 14
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE), // 4ED 15
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // 4ED 16
    PrintingRecord::reprint(&alpha::CIRCLE_OF_PROTECTION_RED), // 4ED 17
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // 4ED 18
    PrintingRecord::reprint(&catalog_lea::CONVERSION),       // 4ED 19
    PrintingRecord::reprint(&catalog_lea::CRUSADE),          // 4ED 20
    PrintingRecord::reprint(&catalog_lea::DEATH_WARD),       // 4ED 21
    PrintingRecord::reprint(&catalog_lea::DISENCHANT),       // 4ED 22
    PrintingRecord::reprint(&catalog_leg::DIVINE_TRANSFORMATION), // 4ED 23
    PrintingRecord::reprint(&catalog_leg::ELDER_LAND_WURM),  // 4ED 24
    PrintingRecord::reprint(&catalog_arn::EYE_FOR_AN_EYE),   // 4ED 25
    PrintingRecord::reprint(&catalog_leg::FORTIFIED_AREA),   // 4ED 26
    PrintingRecord::reprint(&catalog_lea::GREEN_WARD),       // 4ED 27
    PrintingRecord::reprint(&catalog_lea::HEALING_SALVE),    // 4ED 28
    PrintingRecord::reprint(&catalog_lea::HOLY_ARMOR),       // 4ED 29
    PrintingRecord::reprint(&catalog_lea::HOLY_STRENGTH),    // 4ED 30
    PrintingRecord::reprint(&catalog_lea::ISLAND_SANCTUARY), // 4ED 31
    PrintingRecord::reprint(&catalog_lea::KARMA),            // 4ED 32
    PrintingRecord::reprint(&catalog_leg::KISMET),           // 4ED 33
    PrintingRecord::reprint(&catalog_leg::LAND_TAX),         // 4ED 34
    PrintingRecord::reprint(&catalog_lea::MESA_PEGASUS),     // 4ED 35
    PrintingRecord::reprint(&catalog_drk::MORALE),           // 4ED 36
    PrintingRecord::reprint(&catalog_lea::NORTHERN_PALADIN), // 4ED 37
    PrintingRecord::reprint(&catalog_leg::OSAI_VULTURES),    // 4ED 38
    PrintingRecord::reprint(&catalog_lea::PEARLED_UNICORN),  // 4ED 39
    PrintingRecord::reprint(&catalog_lea::PERSONAL_INCARNATION), // 4ED 40
    PrintingRecord::reprint(&catalog_arn::PIETY),            // 4ED 41
    PrintingRecord::reprint(&catalog_drk::PIKEMEN),          // 4ED 42
    PrintingRecord::reprint(&catalog_lea::PURELACE),         // 4ED 43
    PrintingRecord::reprint(&catalog_lea::RED_WARD),         // 4ED 44
    PrintingRecord::reprint(&catalog_lea::REVERSE_DAMAGE),   // 4ED 45
    PrintingRecord::reprint(&catalog_lea::RIGHTEOUSNESS),    // 4ED 46
    PrintingRecord::reprint(&catalog_lea::SAMITE_HEALER),    // 4ED 47
    PrintingRecord::reprint(&catalog_lea::SAVANNAH_LIONS),   // 4ED 48
    PrintingRecord::reprint(&catalog_leg::SEEKER),           // 4ED 49
    PrintingRecord::reprint(&catalog_lea::SERRA_ANGEL),      // 4ED 50
    PrintingRecord::reprint(&catalog_leg::SPIRIT_LINK),      // 4ED 51
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES),   // 4ED 52
    PrintingRecord::reprint(&catalog_leg::TUNDRA_WOLVES),    // 4ED 53
    PrintingRecord::reprint(&catalog_leg::VISIONS),          // 4ED 54
    PrintingRecord::reprint(&catalog_lea::WALL_OF_SWORDS),   // 4ED 55
    PrintingRecord::reprint(&catalog_lea::WHITE_KNIGHT),     // 4ED 56
    PrintingRecord::reprint(&catalog_lea::WHITE_WARD),       // 4ED 57
    PrintingRecord::reprint(&catalog_lea::WRATH_OF_GOD),     // 4ED 58
    PrintingRecord::reprint(&catalog_lea::AIR_ELEMENTAL),    // 4ED 59
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT),       // 4ED 60
    PrintingRecord::reprint(&catalog_drk::APPRENTICE_WIZARD), // 4ED 61
    PrintingRecord::reprint(&catalog_leg::BACKFIRE),         // 4ED 62
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST),   // 4ED 63
    PrintingRecord::reprint(&catalog_lea::CONTROL_MAGIC),    // 4ED 64
    PrintingRecord::reprint(&alpha::COUNTERSPELL),           // 4ED 65
    PrintingRecord::reprint(&catalog_lea::CREATURE_BOND),    // 4ED 66
    PrintingRecord::reprint(&catalog_lea::DRAIN_POWER),      // 4ED 67
    PrintingRecord::reprint(&catalog_atq::ENERGY_FLUX),      // 4ED 68
    PrintingRecord::reprint(&catalog_leg::ENERGY_TAP),       // 4ED 69
    PrintingRecord::reprint(&catalog_drk::EROSION),          // 4ED 70
    PrintingRecord::reprint(&catalog_lea::FEEDBACK),         // 4ED 71
    PrintingRecord::reprint(&catalog_lea::FLIGHT),           // 4ED 72
    PrintingRecord::reprint(&catalog_drk::FLOOD),            // 4ED 73
    PrintingRecord::reprint(&catalog_leg::GASEOUS_FORM),     // 4ED 74
    PrintingRecord::reprint(&catalog_drk::GHOST_SHIP),       // 4ED 75
    PrintingRecord::reprint(&catalog_arn::GIANT_TORTOISE),   // 4ED 76
    PrintingRecord::reprint(&catalog_atq::HURKYLS_RECALL),   // 4ED 77
    PrintingRecord::reprint(&catalog_arn::ISLAND_FISH_JASCONIUS), // 4ED 78
    PrintingRecord::reprint(&catalog_lea::JUMP),             // 4ED 79
    PrintingRecord::reprint(&catalog_drk::LEVIATHAN),        // 4ED 80
    PrintingRecord::reprint(&catalog_lea::LIFETAP),          // 4ED 81
    PrintingRecord::reprint(&catalog_lea::LORD_OF_ATLANTIS), // 4ED 82
    PrintingRecord::reprint(&catalog_lea::MAGICAL_HACK),     // 4ED 83
    PrintingRecord::reprint(&catalog_lea::MAHAMOTI_DJINN),   // 4ED 84
    PrintingRecord::reprint(&catalog_lea::MANA_SHORT),       // 4ED 85
    PrintingRecord::reprint(&catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT), // 4ED 86
    PrintingRecord::reprint(&catalog_drk::MIND_BOMB),        // 4ED 87
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_FORCES), // 4ED 88
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_TERRAIN), // 4ED 89
    PrintingRecord::reprint(&catalog_lea::PHANTOM_MONSTER),  // 4ED 90
    PrintingRecord::reprint(&catalog_lea::PIRATE_SHIP),      // 4ED 91
    PrintingRecord::reprint(&catalog_lea::POWER_LEAK),       // 4ED 92
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),       // 4ED 93
    PrintingRecord::reprint(&catalog_lea::PRODIGAL_SORCERER), // 4ED 94
    PrintingRecord::reprint(&catalog_leg::PSIONIC_ENTITY),   // 4ED 95
    PrintingRecord::reprint(&catalog_lea::PSYCHIC_VENOM),    // 4ED 96
    PrintingRecord::reprint(&catalog_leg::RELIC_BIND),       // 4ED 97
    PrintingRecord::reprint(&catalog_lea::SEA_SERPENT),      // 4ED 98
    PrintingRecord::reprint(&catalog_leg::SEGOVIAN_LEVIATHAN), // 4ED 99
    PrintingRecord::reprint(&catalog_arn::SINDBAD),          // 4ED 100
    PrintingRecord::reprint(&catalog_lea::SIREN_S_CALL),     // 4ED 101
    PrintingRecord::reprint(&catalog_lea::SLEIGHT_OF_MIND),  // 4ED 102
    PrintingRecord::reprint(&catalog_lea::SPELL_BLAST),      // 4ED 103
    PrintingRecord::reprint(&alpha::STASIS),                 // 4ED 104
    PrintingRecord::reprint(&catalog_lea::STEAL_ARTIFACT),   // 4ED 105
    PrintingRecord::reprint(&catalog_drk::SUNKEN_CITY),      // 4ED 106
    PrintingRecord::reprint(&catalog_lea::THOUGHTLACE),      // 4ED 107
    PrintingRecord::reprint(&catalog_lea::DRUDGE_SKELETONS), // 4ED 107†
    PrintingRecord::reprint(&catalog_leg::TIME_ELEMENTAL),   // 4ED 108
    PrintingRecord::reprint(&catalog_lea::TWIDDLE),          // 4ED 109
    PrintingRecord::reprint(&catalog_arn::UNSTABLE_MUTATION), // 4ED 110
    PrintingRecord::reprint(&catalog_lea::UNSUMMON),         // 4ED 111
    PrintingRecord::reprint(&catalog_lea::VOLCANIC_ERUPTION), // 4ED 112
    PrintingRecord::reprint(&catalog_lea::WALL_OF_AIR),      // 4ED 113
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WATER),    // 4ED 114
    PrintingRecord::reprint(&catalog_lea::WATER_ELEMENTAL),  // 4ED 115
    PrintingRecord::reprint(&catalog_leg::ZEPHYR_FALCON),    // 4ED 116
    PrintingRecord::reprint(&catalog_leg::ABOMINATION),      // 4ED 117
    PrintingRecord::reprint(&catalog_lea::ANIMATE_DEAD),     // 4ED 118
    PrintingRecord::reprint(&catalog_drk::ASHES_TO_ASHES),   // 4ED 119
    PrintingRecord::reprint(&catalog_lea::BAD_MOON),         // 4ED 120
    PrintingRecord::reprint(&catalog_lea::BLACK_KNIGHT),     // 4ED 121
    PrintingRecord::reprint(&catalog_leg::BLIGHT),           // 4ED 122
    PrintingRecord::reprint(&catalog_drk::BOG_IMP),          // 4ED 123
    PrintingRecord::reprint(&catalog_lea::BOG_WRAITH),       // 4ED 124
    PrintingRecord::reprint(&catalog_leg::CARRION_ANTS),     // 4ED 125
    PrintingRecord::reprint(&catalog_leg::COSMIC_HORROR),    // 4ED 126
    PrintingRecord::reprint(&catalog_lea::CURSED_LAND),      // 4ED 127
    PrintingRecord::reprint(&catalog_leg::CYCLOPEAN_MUMMY),  // 4ED 128
    PrintingRecord::reprint(&catalog_lea::DARK_RITUAL),      // 4ED 129
    PrintingRecord::reprint(&catalog_lea::DEATHGRIP),        // 4ED 130
    PrintingRecord::reprint(&catalog_lea::DEATHLACE),        // 4ED 131
    PrintingRecord::reprint(&catalog_lea::DRAIN_LIFE),       // 4ED 132
    PrintingRecord::alternate(&catalog_lea::DRUDGE_SKELETONS, 1), // 4ED 133
    PrintingRecord::reprint(&catalog_arn::EL_HAJJAJ),        // 4ED 134
    PrintingRecord::alternate(&catalog_arn::EL_HAJJAJ, 1),   // 4ED 134†
    PrintingRecord::reprint(&catalog_arn::ERG_RAIDERS),      // 4ED 135
    PrintingRecord::reprint(&catalog_lea::EVIL_PRESENCE),    // 4ED 136
    PrintingRecord::reprint(&catalog_lea::FEAR),             // 4ED 137
    PrintingRecord::reprint(&catalog_lea::FROZEN_SHADE),     // 4ED 138
    PrintingRecord::reprint(&catalog_lea::GLOOM),            // 4ED 139
    PrintingRecord::reprint(&catalog_leg::GREED),            // 4ED 140
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND), // 4ED 141
    PrintingRecord::reprint(&catalog_lea::HYPNOTIC_SPECTER), // 4ED 142
    PrintingRecord::reprint(&catalog_arn::JUNUN_EFREET),     // 4ED 143
    PrintingRecord::reprint(&catalog_lea::LORD_OF_THE_PIT),  // 4ED 144
    PrintingRecord::reprint(&catalog_leg::LOST_SOUL),        // 4ED 145
    PrintingRecord::reprint(&catalog_drk::MARSH_GAS),        // 4ED 146
    PrintingRecord::reprint(&catalog_lea::MIND_TWIST),       // 4ED 147
    PrintingRecord::reprint(&catalog_drk::MURK_DWELLERS),    // 4ED 148
    PrintingRecord::reprint(&catalog_lea::NETHER_SHADOW),    // 4ED 149
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),        // 4ED 150
    PrintingRecord::reprint(&catalog_lea::PARALYZE),         // 4ED 151
    PrintingRecord::reprint(&catalog_lea::PESTILENCE),       // 4ED 152
    PrintingRecord::reprint(&catalog_leg::PIT_SCORPION),     // 4ED 153
    PrintingRecord::reprint(&catalog_lea::PLAGUE_RATS),      // 4ED 154
    PrintingRecord::reprint(&catalog_drk::RAG_MAN),          // 4ED 155
    PrintingRecord::reprint(&catalog_lea::RAISE_DEAD),       // 4ED 156
    PrintingRecord::reprint(&catalog_lea::ROYAL_ASSASSIN),   // 4ED 157
    PrintingRecord::reprint(&catalog_lea::SCATHE_ZOMBIES),   // 4ED 158
    PrintingRecord::reprint(&catalog_lea::SCAVENGING_GHOUL), // 4ED 159
    PrintingRecord::reprint(&catalog_lea::SENGIR_VAMPIRE),   // 4ED 160
    PrintingRecord::reprint(&catalog_lea::SIMULACRUM),       // 4ED 161
    PrintingRecord::reprint(&catalog_arn::SORCERESS_QUEEN),  // 4ED 162
    PrintingRecord::reprint(&catalog_leg::SPIRIT_SHACKLE),   // 4ED 163
    PrintingRecord::reprint(&catalog_lea::TERROR),           // 4ED 164
    PrintingRecord::reprint(&catalog_drk::UNCLE_ISTVAN),     // 4ED 165
    PrintingRecord::reprint(&catalog_lea::UNHOLY_STRENGTH),  // 4ED 166
    PrintingRecord::reprint(&catalog_leg::VAMPIRE_BATS),     // 4ED 167
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BONE),     // 4ED 168
    PrintingRecord::reprint(&catalog_lea::WARP_ARTIFACT),    // 4ED 169
    PrintingRecord::reprint(&catalog_lea::WEAKNESS),         // 4ED 170
    PrintingRecord::reprint(&catalog_lea::WILL_O_THE_WISP),  // 4ED 171
    PrintingRecord::reprint(&catalog_drk::WORD_OF_BINDING),  // 4ED 172
    PrintingRecord::reprint(&catalog_atq::XENIC_POLTERGEIST), // 4ED 173
    PrintingRecord::reprint(&catalog_lea::ZOMBIE_MASTER),    // 4ED 174
    PrintingRecord::reprint(&catalog_arn::ALI_BABA),         // 4ED 175
    PrintingRecord::reprint(&catalog_drk::BALL_LIGHTNING),   // 4ED 176
    PrintingRecord::reprint(&catalog_arn::BIRD_MAIDEN),      // 4ED 177
    PrintingRecord::reprint(&catalog_leg::BLOOD_LUST),       // 4ED 178
    PrintingRecord::reprint(&catalog_drk::BROTHERS_OF_FIRE), // 4ED 179
    PrintingRecord::reprint(&catalog_lea::BURROWING),        // 4ED 180
    PrintingRecord::reprint(&catalog_drk::CAVE_PEOPLE),      // 4ED 181
    PrintingRecord::reprint(&catalog_lea::CHAOSLACE),        // 4ED 182
    PrintingRecord::reprint(&catalog_leg::CRIMSON_MANTICORE), // 4ED 183
    PrintingRecord::reprint(&catalog_atq::DETONATE),         // 4ED 184
    PrintingRecord::reprint(&catalog_lea::DISINTEGRATE),     // 4ED 185
    PrintingRecord::reprint(&catalog_lea::DRAGON_WHELP),     // 4ED 186
    PrintingRecord::reprint(&catalog_lea::DWARVEN_WARRIORS), // 4ED 187
    PrintingRecord::reprint(&catalog_lea::EARTH_ELEMENTAL),  // 4ED 188
    PrintingRecord::reprint(&alpha::EARTHQUAKE),             // 4ED 189
    PrintingRecord::reprint(&catalog_leg::ETERNAL_WARRIOR),  // 4ED 190
    PrintingRecord::reprint(&catalog_lea::FIRE_ELEMENTAL),   // 4ED 191
    PrintingRecord::reprint(&catalog_lea::FIREBALL),         // 4ED 192
    PrintingRecord::reprint(&catalog_lea::FIREBREATHING),    // 4ED 193
    PrintingRecord::reprint(&catalog_drk::FISSURE),          // 4ED 194
    PrintingRecord::reprint(&catalog_lea::FLASHFIRES),       // 4ED 195
    PrintingRecord::reprint(&catalog_leg::GIANT_STRENGTH),   // 4ED 196
    PrintingRecord::reprint(&catalog_lea::GOBLIN_BALLOON_BRIGADE), // 4ED 197
    PrintingRecord::reprint(&catalog_lea::GOBLIN_KING),      // 4ED 198
    PrintingRecord::reprint(&catalog_drk::GOBLIN_ROCK_SLED), // 4ED 199
    PrintingRecord::reprint(&catalog_lea::GRAY_OGRE),        // 4ED 200
    PrintingRecord::reprint(&catalog_lea::HILL_GIANT),       // 4ED 201
    PrintingRecord::reprint(&catalog_lea::HURLOON_MINOTAUR), // 4ED 202
    PrintingRecord::reprint(&catalog_arn::HURR_JACKAL),      // 4ED 203
    PrintingRecord::reprint(&catalog_leg::IMMOLATION),       // 4ED 204
    PrintingRecord::reprint(&catalog_drk::INFERNO),          // 4ED 205
    PrintingRecord::reprint(&catalog_lea::IRONCLAW_ORCS),    // 4ED 206
    PrintingRecord::reprint(&catalog_lea::KELDON_WARLORD),   // 4ED 207
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT),         // 4ED 208
    PrintingRecord::reprint(&catalog_arn::MAGNETIC_MOUNTAIN), // 4ED 209
    PrintingRecord::reprint(&catalog_drk::MANA_CLASH),       // 4ED 210
    PrintingRecord::reprint(&catalog_lea::MANA_FLARE),       // 4ED 211
    PrintingRecord::reprint(&catalog_lea::MANABARBS),        // 4ED 212
    PrintingRecord::reprint(&catalog_lea::MONSS_GOBLIN_RAIDERS), // 4ED 213
    PrintingRecord::reprint(&catalog_lea::ORCISH_ARTILLERY), // 4ED 214
    PrintingRecord::reprint(&catalog_lea::ORCISH_ORIFLAMME), // 4ED 215
    PrintingRecord::reprint(&catalog_lea::POWER_SURGE),      // 4ED 216
    PrintingRecord::reprint(&catalog_leg::PYROTECHNICS),     // 4ED 217
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST),    // 4ED 218
    PrintingRecord::reprint(&catalog_lea::SHATTER),          // 4ED 219
    PrintingRecord::reprint(&catalog_lea::SHIVAN_DRAGON),    // 4ED 220
    PrintingRecord::reprint(&catalog_drk::SISTERS_OF_THE_FLAME), // 4ED 221
    PrintingRecord::reprint(&catalog_lea::SMOKE),            // 4ED 222
    PrintingRecord::reprint(&catalog_lea::STONE_GIANT),      // 4ED 223
    PrintingRecord::reprint(&catalog_lea::STONE_RAIN),       // 4ED 224
    PrintingRecord::reprint(&catalog_leg::TEMPEST_EFREET),   // 4ED 225
    PrintingRecord::reprint(&catalog_leg::THE_BRUTE),        // 4ED 226
    PrintingRecord::reprint(&catalog_lea::TUNNEL),           // 4ED 227
    PrintingRecord::reprint(&catalog_lea::UTHDEN_TROLL),     // 4ED 228
    PrintingRecord::reprint(&catalog_leg::WALL_OF_DUST),     // 4ED 229
    PrintingRecord::reprint(&catalog_lea::WALL_OF_FIRE),     // 4ED 230
    PrintingRecord::reprint(&catalog_lea::WALL_OF_STONE),    // 4ED 231
    PrintingRecord::reprint(&catalog_leg::WINDS_OF_CHANGE),  // 4ED 232
    PrintingRecord::reprint(&catalog_lea::ASPECT_OF_WOLF),   // 4ED 233
    PrintingRecord::reprint(&catalog_lea::BIRDS_OF_PARADISE), // 4ED 234
    PrintingRecord::reprint(&catalog_drk::CARNIVOROUS_PLANT), // 4ED 235
    PrintingRecord::reprint(&catalog_lea::CHANNEL),          // 4ED 236
    PrintingRecord::reprint(&catalog_lea::COCKATRICE),       // 4ED 237
    PrintingRecord::reprint(&catalog_lea::CRAW_WURM),        // 4ED 238
    PrintingRecord::reprint(&catalog_atq::CRUMBLE),          // 4ED 239
    PrintingRecord::reprint(&catalog_arn::DESERT_TWISTER),   // 4ED 240
    PrintingRecord::reprint(&catalog_leg::DURKWOOD_BOARS),   // 4ED 241
    PrintingRecord::reprint(&catalog_leg::ELVEN_RIDERS),     // 4ED 242
    PrintingRecord::reprint(&catalog_lea::ELVISH_ARCHERS),   // 4ED 243
    PrintingRecord::reprint(&catalog_lea::FOG),              // 4ED 244
    PrintingRecord::reprint(&catalog_lea::FORCE_OF_NATURE),  // 4ED 245
    PrintingRecord::reprint(&catalog_lea::FUNGUSAUR),        // 4ED 246
    PrintingRecord::reprint(&catalog_lea::GAEA_S_LIEGE),     // 4ED 247
    PrintingRecord::reprint(&catalog_lea::GIANT_GROWTH),     // 4ED 248
    PrintingRecord::reprint(&catalog_lea::GIANT_SPIDER),     // 4ED 249
    PrintingRecord::reprint(&catalog_lea::GRIZZLY_BEARS),    // 4ED 250
    PrintingRecord::reprint(&catalog_lea::HURRICANE),        // 4ED 251
    PrintingRecord::reprint(&catalog_lea::INSTILL_ENERGY),   // 4ED 252
    PrintingRecord::reprint(&catalog_lea::IRONROOT_TREEFOLK), // 4ED 253
    PrintingRecord::reprint(&catalog_leg::KILLER_BEES),      // 4ED 254
    PrintingRecord::reprint(&catalog_drk::LAND_LEECHES),     // 4ED 255
    PrintingRecord::reprint(&catalog_lea::LEY_DRUID),        // 4ED 256
    PrintingRecord::reprint(&catalog_lea::LIFEFORCE),        // 4ED 257
    PrintingRecord::reprint(&catalog_lea::LIFELACE),         // 4ED 258
    PrintingRecord::reprint(&catalog_lea::LIVING_ARTIFACT),  // 4ED 259
    PrintingRecord::reprint(&catalog_lea::LIVING_LANDS),     // 4ED 260
    PrintingRecord::reprint(&catalog_lea::LLANOWAR_ELVES),   // 4ED 261
    PrintingRecord::reprint(&catalog_lea::LURE),             // 4ED 262
    PrintingRecord::reprint(&catalog_drk::MARSH_VIPER),      // 4ED 263
    PrintingRecord::reprint(&catalog_arn::NAFS_ASP),         // 4ED 264
    PrintingRecord::reprint(&catalog_leg::PRADESH_GYPSIES),  // 4ED 265
    PrintingRecord::reprint(&catalog_leg::RADJAN_SPIRIT),    // 4ED 266
    PrintingRecord::reprint(&catalog_leg::REBIRTH),          // 4ED 267
    PrintingRecord::reprint(&catalog_lea::REGENERATION),     // 4ED 268
    PrintingRecord::reprint(&catalog_arn::SANDSTORM),        // 4ED 269
    PrintingRecord::reprint(&catalog_lea::SCRYB_SPRITES),    // 4ED 270
    PrintingRecord::reprint(&catalog_lea::SHANODIN_DRYADS),  // 4ED 271
    PrintingRecord::reprint(&catalog_lea::STREAM_OF_LIFE),   // 4ED 272
    PrintingRecord::reprint(&catalog_leg::SYLVAN_LIBRARY),   // 4ED 273
    PrintingRecord::reprint(&catalog_lea::THICKET_BASILISK), // 4ED 274
    PrintingRecord::reprint(&catalog_lea::TIMBER_WOLVES),    // 4ED 275
    PrintingRecord::reprint(&catalog_atq::TITANIA_S_SONG),   // 4ED 276
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),      // 4ED 277
    PrintingRecord::reprint(&catalog_lea::TSUNAMI),          // 4ED 278
    PrintingRecord::reprint(&catalog_leg::UNTAMED_WILDS),    // 4ED 279
    PrintingRecord::reprint(&catalog_drk::VENOM),            // 4ED 280
    PrintingRecord::reprint(&catalog_lea::VERDURAN_ENCHANTRESS), // 4ED 281
    PrintingRecord::reprint(&catalog_lea::WALL_OF_BRAMBLES), // 4ED 282
    PrintingRecord::reprint(&catalog_lea::WALL_OF_ICE),      // 4ED 283
    PrintingRecord::reprint(&catalog_lea::WALL_OF_WOOD),     // 4ED 284
    PrintingRecord::reprint(&catalog_lea::WANDERLUST),       // 4ED 285
    PrintingRecord::reprint(&catalog_lea::WAR_MAMMOTH),      // 4ED 286
    PrintingRecord::reprint(&catalog_lea::WEB),              // 4ED 287
    PrintingRecord::reprint(&catalog_leg::WHIRLING_DERVISH), // 4ED 288
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),      // 4ED 289
    PrintingRecord::reprint(&catalog_leg::WINTER_BLAST),     // 4ED 290
    PrintingRecord::reprint(&catalog_arn::ALADDIN_S_LAMP),   // 4ED 291
    PrintingRecord::reprint(&catalog_arn::ALADDINS_RING),    // 4ED 292
    PrintingRecord::reprint(&catalog_atq::AMULET_OF_KROOG),  // 4ED 293
    PrintingRecord::reprint(&catalog_lea::ANKH_OF_MISHRA),   // 4ED 294
    PrintingRecord::reprint(&catalog_atq::ARMAGEDDON_CLOCK), // 4ED 295
    PrintingRecord::reprint(&catalog_atq::ASHNODS_BATTLE_GEAR), // 4ED 296
    PrintingRecord::reprint(&catalog_atq::BATTERING_RAM),    // 4ED 297
    PrintingRecord::reprint(&catalog_leg::BLACK_MANA_BATTERY), // 4ED 298
    PrintingRecord::reprint(&alpha::BLACK_VISE),             // 4ED 299
    PrintingRecord::reprint(&catalog_leg::BLUE_MANA_BATTERY), // 4ED 300
    PrintingRecord::reprint(&catalog_arn::BOTTLE_OF_SULEIMAN), // 4ED 301
    PrintingRecord::reprint(&catalog_arn::BRASS_MAN),        // 4ED 302
    PrintingRecord::reprint(&catalog_atq::BRONZE_TABLET),    // 4ED 303
    PrintingRecord::reprint(&catalog_lea::CELESTIAL_PRISM),  // 4ED 304
    PrintingRecord::reprint(&catalog_atq::CLAY_STATUE),      // 4ED 305
    PrintingRecord::reprint(&catalog_atq::CLOCKWORK_AVIAN),  // 4ED 306
    PrintingRecord::reprint(&catalog_lea::CLOCKWORK_BEAST),  // 4ED 307
    PrintingRecord::reprint(&catalog_atq::COLOSSUS_OF_SARDIA), // 4ED 308
    PrintingRecord::reprint(&catalog_lea::CONSERVATOR),      // 4ED 309
    PrintingRecord::reprint(&catalog_atq::CORAL_HELM),       // 4ED 310
    PrintingRecord::reprint(&catalog_lea::CRYSTAL_ROD),      // 4ED 311
    PrintingRecord::reprint(&catalog_atq::CURSED_RACK),      // 4ED 312
    PrintingRecord::reprint(&catalog_arn::DANCING_SCIMITAR), // 4ED 313
    PrintingRecord::reprint(&catalog_drk::DIABOLIC_MACHINE), // 4ED 314
    PrintingRecord::reprint(&catalog_lea::DINGUS_EGG),       // 4ED 315
    PrintingRecord::reprint(&catalog_lea::DISRUPTING_SCEPTER), // 4ED 316
    PrintingRecord::reprint(&catalog_atq::DRAGON_ENGINE),    // 4ED 317
    PrintingRecord::reprint(&catalog_arn::EBONY_HORSE),      // 4ED 318
    PrintingRecord::reprint(&catalog_drk::FELLWAR_STONE),    // 4ED 319
    PrintingRecord::reprint(&catalog_arn::FLYING_CARPET),    // 4ED 320
    PrintingRecord::reprint(&catalog_lea::GLASSES_OF_URZA),  // 4ED 321
    PrintingRecord::reprint(&catalog_atq::GRAPESHOT_CATAPULT), // 4ED 322
    PrintingRecord::reprint(&catalog_leg::GREEN_MANA_BATTERY), // 4ED 323
    PrintingRecord::reprint(&catalog_lea::HELM_OF_CHATZUK),  // 4ED 324
    PrintingRecord::reprint(&catalog_lea::HOWLING_MINE),     // 4ED 325
    PrintingRecord::reprint(&catalog_lea::IRON_STAR),        // 4ED 326
    PrintingRecord::reprint(&catalog_lea::IVORY_CUP),        // 4ED 327
    PrintingRecord::reprint(&catalog_atq::IVORY_TOWER),      // 4ED 328
    PrintingRecord::reprint(&catalog_lea::JADE_MONOLITH),    // 4ED 329
    PrintingRecord::reprint(&catalog_arn::JANDORS_SADDLEBAGS), // 4ED 330
    PrintingRecord::reprint(&catalog_lea::JAYEMDAE_TOME),    // 4ED 331
    PrintingRecord::reprint(&catalog_lea::KORMUS_BELL),      // 4ED 332
    PrintingRecord::reprint(&catalog_lea::LIBRARY_OF_LENG),  // 4ED 333
    PrintingRecord::reprint(&catalog_lea::MANA_VAULT),       // 4ED 334
    PrintingRecord::reprint(&catalog_lea::MEEKSTONE),        // 4ED 335
    PrintingRecord::reprint(&catalog_atq::MILLSTONE),        // 4ED 336
    PrintingRecord::reprint(&catalog_atq::MISHRA_S_WAR_MACHINE), // 4ED 337
    PrintingRecord::reprint(&catalog_lea::NEVINYRRALS_DISK), // 4ED 338
    PrintingRecord::reprint(&catalog_lea::OBSIANUS_GOLEM),   // 4ED 339
    PrintingRecord::reprint(&catalog_atq::ONULET),           // 4ED 340
    PrintingRecord::reprint(&catalog_atq::ORNITHOPTER),      // 4ED 341
    PrintingRecord::reprint(&catalog_atq::PRIMAL_CLAY),      // 4ED 342
    PrintingRecord::reprint(&catalog_leg::RED_MANA_BATTERY), // 4ED 343
    PrintingRecord::reprint(&catalog_lea::ROD_OF_RUIN),      // 4ED 344
    PrintingRecord::reprint(&catalog_atq::SHAPESHIFTER),     // 4ED 345
    PrintingRecord::reprint(&catalog_lea::SOUL_NET),         // 4ED 346
    PrintingRecord::reprint(&catalog_lea::SUNGLASSES_OF_URZA), // 4ED 347
    PrintingRecord::reprint(&catalog_atq::TAWNOSS_WAND),     // 4ED 348
    PrintingRecord::reprint(&catalog_atq::TAWNOSS_WEAPONRY), // 4ED 349
    PrintingRecord::reprint(&catalog_atq::TETRAVUS),         // 4ED 350
    PrintingRecord::reprint(&catalog_lea::THE_HIVE),         // 4ED 351
    PrintingRecord::reprint(&catalog_atq::THE_RACK),         // 4ED 352
    PrintingRecord::reprint(&catalog_lea::THRONE_OF_BONE),   // 4ED 353
    PrintingRecord::reprint(&catalog_atq::TRISKELION),       // 4ED 354
    PrintingRecord::reprint(&catalog_atq::URZA_S_AVENGER),   // 4ED 355
    PrintingRecord::reprint(&catalog_atq::WALL_OF_SPEARS),   // 4ED 356
    PrintingRecord::reprint(&catalog_leg::WHITE_MANA_BATTERY), // 4ED 357
    PrintingRecord::reprint(&catalog_lea::WINTER_ORB),       // 4ED 358
    PrintingRecord::reprint(&catalog_lea::WOODEN_SPHERE),    // 4ED 359
    PrintingRecord::reprint(&catalog_atq::YOTIAN_SOLDIER),   // 4ED 360
    PrintingRecord::reprint(&antiquities::MISHRA_S_FACTORY), // 4ED 361
    PrintingRecord::reprint(&catalog_arn::OASIS),            // 4ED 362
    PrintingRecord::reprint(&catalog_atq::STRIP_MINE),       // 4ED 363
    PrintingRecord::reprint(&catalog_lea::PLAINS),           // 4ED 364
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),      // 4ED 365
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),      // 4ED 366
    PrintingRecord::reprint(&catalog_lea::ISLAND),           // 4ED 367
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),      // 4ED 368
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),      // 4ED 369
    PrintingRecord::reprint(&catalog_lea::SWAMP),            // 4ED 370
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),       // 4ED 371
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),       // 4ED 372
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),         // 4ED 373
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),    // 4ED 374
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2),    // 4ED 375
    PrintingRecord::reprint(&catalog_lea::FOREST),           // 4ED 376
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),      // 4ED 377
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),      // 4ED 378
];
