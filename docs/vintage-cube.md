# Vintage Cube implementation roadmap

The MTGO Vintage Cube is a 534-card singleton list, recorded verbatim in
[`src/format/vintage_cube.rs`](../src/format/vintage_cube.rs) as it stood on
2026-08-19. A cube is re-tuned between runs, so the pool is a dated snapshot
rather than a claim about what is current.

`Format::VintageCube` takes its legality from that list. This is the first
format here that is not a set window: a card is legal because the cube names
it, not because of where it was printed. Six names in the retrieved list match
no card Scryfall knows and were left out; the module records which.

## Snapshot

- 534 cards in the pool, of which 288 are cataloged and 246 are not
- The pool spans sets Penta has never touched, so most of the backlog needs a
  printed-set module before the card itself
- No decks are registered yet. `deck_names_for_format` returns nothing for the
  cube, so it is not offered in the web client
- Drafting is deferred. The engine has no draft, and the plan is to reach a
  playable pool first and play fixed lists from it

## Format profile

Forty-card minimum, one copy of each card, twenty life, seven-card opening
hand, contemporary mana rules, and no ban or restricted list -- a card is
either in the pool or it is not. `FormatRules::card_pool` carries the list, and
`Format::allows_card` consults it instead of `allowed_sets`, which the cube
leaves empty so nothing reads it as a set window by accident.

## Not yet cataloged

Grouped by color so a tranche can be scoped to one part of the pool. Basic
lands are legal in every format and are not listed.

### White (24)

- `Cosmogrand Zenith`
- `Eagles of the North`
- `Elspeth, Knight-Errant`
- `Elspeth, Storm Slayer`
- `Ephemerate`
- `Flickerwisp`
- `Giver of Runes`
- `Glimmer Lens`
- `Guide of Souls`
- `Leyline Binding`
- `Loran of the Third Path`
- `Luminarch Aspirant`
- `Oust`
- `Overlord of the Mistmoors`
- `Phelia, Exuberant Shepherd`
- `Serra Paragon`
- `Skyclave Apparition`
- `Solitude`
- `Staff of the Storyteller`
- `The Wandering Emperor`
- `Thraben Inspector`
- `Touch the Spirit Realm`
- `Virtue of Loyalty`
- `Witch Enchanter`

### Blue (32)

- `Astrologian's Planisphere`
- `Brainsurge`
- `Brazen Borrower`
- `Consider`
- `Consult the Star Charts`
- `Displacer Kitten`
- `Emry, Lurker of the Loch`
- `Faerie Mastermind`
- `Flash`
- `Gitaxian Probe`
- `Hullbreacher`
- `Jace, Vryn's Prodigy`
- `Kappa Cannoneer`
- `Kitsa, Otterball Elite`
- `Lose Focus`
- `Malcolm, Alluring Scoundrel`
- `Memory Lapse`
- `Narset, Parter of Veils`
- `Phyrexian Metamorph`
- `Plagon, Lord of the Beach`
- `Proft's Eidetic Memory`
- `Quantum Riddler`
- `Remand`
- `Show and Tell`
- `Sink into Stupor`
- `Thieving Skydiver`
- `Thundertrap Trainer`
- `Tinker`
- `Tishana's Tidebinder`
- `Trinket Mage`
- `Urza, Lord High Artificer`

### Black (25)

- `Animate Dead`
- `Archon of Cruelty`
- `Bitter Triumph`
- `Cabal Ritual`
- `Caustic Bronco`
- `Concealing Curtains`
- `Crabomination`
- `Dark Confidant`
- `Dauthi Voidwalker`
- `Dismember`
- `Grave Titan`
- `Grief`
- `Harvester of Misery`
- `Imperial Seal`
- `Infernal Grasp`
- `Inquisition of Kozilek`
- `Nethergoyf`
- `Preacher of the Schism`
- `Recurring Nightmare`
- `Sedgemoor Witch`
- `Sheoldred's Edict`
- `Troll of Khazad-dûm`
- `Unearth`
- `Vampire Hexmage`
- `Yawgmoth's Will`

### Red (32)

- `Abrade`
- `Broadside Bombardiers`
- `Burst Lightning`
- `Chainsaw`
- `Cori-Steel Cutter`
- `Death-Greeter's Champion`
- `Detective's Phoenix`
- `Dragon's Rage Channeler`
- `Fable of the Mirror-Breaker`
- `Fear of Missing Out`
- `Fiery Confluence`
- `Galvanic Blast`
- `Galvanic Discharge`
- `Generous Plunderer`
- `Goldspan Dragon`
- `Headliner Scarlett`
- `Inti, Seneschal of the Sun`
- `Kari Zev, Skyship Raider`
- `Kellan, Planar Trailblazer`
- `Monstrous Rage`
- `Oliphaunt`
- `Orcish Lumberjack`
- `Ragavan, Nimble Pilferer`
- `Robber of the Rich`
- `Screaming Nemesis`
- `Seasoned Pyromancer`
- `Slickshot Show-Off`
- `Sneak Attack`
- `Suplex`
- `Tarfire`
- `Tersa Lightshatter`
- `Voldaren Epicure`

### Green (35)

- `Baloth Prime`
- `Courser of Kruphix`
- `Elvish Reclaimer`
- `Endurance`
- `Esika's Chariot`
- `Exploration`
- `Fanatic of Rhonas`
- `Fastbond`
- `Green Sun's Zenith`
- `Hexdrinker`
- `Icetill Explorer`
- `Ignoble Hierarch`
- `Invigorate`
- `Keen-Eyed Curator`
- `Legolas's Quick Reflexes`
- `Lotus Cobra`
- `Malevolent Rumble`
- `Mightform Harmonizer`
- `Mutagenic Growth`
- `Noble Hierarch`
- `Oath of Druids`
- `Once Upon a Time`
- `Ouroboroid`
- `Pest Infestation`
- `Questing Beast`
- `Scythecat Cub`
- `Sentinel of the Nameless City`
- `Six`
- `Springheart Nantuko`
- `Tear Asunder`
- `Tireless Tracker`
- `Traveling Chocobo`
- `Ulvenwald Oddity`
- `Vaultborn Tyrant`
- `Woodfall Primus`

### Multicolor (39)

- `Arwen, Mortal Queen`
- `Atraxa, Grand Unifier`
- `Baleful Strix`
- `Bloodtithe Harvester`
- `Carnage Interpreter`
- `Ertai Resurrected`
- `Etali, Primal Conqueror`
- `Expressive Iteration`
- `Fire Covenant`
- `Forth Eorlingas!`
- `Fractured Identity`
- `Grist, the Hunger Tide`
- `Kaito, Bane of Nightmares`
- `Knight of the Reliquary`
- `Kolaghan's Command`
- `Leovold, Emissary of Trest`
- `Loot, the Pathfinder`
- `Lurrus of the Dream-Den`
- `Lutri, the Spellchaser`
- `Minsc & Boo, Timeless Heroes`
- `Nadu, Winged Wisdom`
- `No More Lies`
- `Oko, Thief of Crowns`
- `Otharri, Suns' Glory`
- `Pillage the Bog`
- `Shorikai, Genesis Engine`
- `Sorin of House Markov`
- `Tamiyo, Inquisitive Student`
- `Teferi, Hero of Dominaria`
- `Teferi, Time Raveler`
- `Territorial Kavu`
- `Third Path Iconoclast`
- `Thopter Foundry`
- `Torsten, Founder of Benalia`
- `Uro, Titan of Nature's Wrath`
- `Wight of the Reliquary`
- `Witherbloom Apprentice`
- `Wrenn and Six`
- `Zirda, the Dawnwaker`

### Colorless (32)

- `Aether Spellbomb`
- `Chromatic Star`
- `Chrome Mox`
- `Coveted Jewel`
- `Currency Converter`
- `Emrakul, the Aeons Torn`
- `Everflowing Chalice`
- `Kaldra Compleat`
- `Karn, Scion of Urza`
- `Lion's Eye Diamond`
- `Memory Jar`
- `Mishra's Bauble`
- `Mox Opal`
- `Myr Battlesphere`
- `Pentad Prism`
- `Portal to Phyrexia`
- `Relic of Sauron`
- `Retrofitter Foundry`
- `Sensei's Divining Top`
- `Smuggler's Copter`
- `Soul-Guide Lantern`
- `Talisman of Conviction`
- `Talisman of Creativity`
- `Talisman of Curiosity`
- `Talisman of Dominance`
- `Talisman of Progress`
- `Tezzeret, Cruel Captain`
- `The Endstone`
- `The Mightstone and Weakstone`
- `The One Ring`
- `Ugin, Eye of the Storms`
- `Urza's Bauble`

### Lands (27)

- `Arena of Glory`
- `Blazemire Verge`
- `Bleachbone Verge`
- `Boseiju, Who Endures`
- `City of Traitors`
- `Creeping Tar Pit`
- `Dark Depths`
- `Fabled Passage`
- `Field of the Dead`
- `Hedge Maze`
- `Multiversal Passage`
- `Otawara, Soaring City`
- `Prismatic Vista`
- `Raucous Theater`
- `Shadowy Backstreet`
- `Shelldock Isle`
- `Sheltering Landscape`
- `Shifting Woodland`
- `Starting Town`
- `Sunbillow Verge`
- `Talon Gates of Madara`
- `Thornspire Verge`
- `Twisted Landscape`
- `Undercity Sewers`
- `Underground Mortuary`
- `Urza's Saga`
- `Waterlogged Grove`

## Already cataloged

These 288 pool cards are in the catalog because an earlier format needed them.
Being cataloged is not the same as being audited against the rest of the cube:
a card authored for Old School or Premodern may meet cards here it has never
been played beside.

- `Abhorrent Oculus`
- `Adeline, Resplendent Cathar`
- `Agatha's Soul Cauldron`
- `Ajani, Nacatl Pariah`
- `Amped Raptor`
- `Ancestral Recall`
- `Ancient Tomb`
- `Arid Mesa`
- `Avacyn's Pilgrim`
- `Badlands`
- `Balance`
- `Baleful Mastery`
- `Barrowgoyf`
- `Basalt Monolith`
- `Bayou`
- `Berserk`
- `Birds of Paradise`
- `Black Lotus`
- `Blackcleave Cliffs`
- `Blightsteel Colossus`
- `Blood Crypt`
- `Bloodbraid Challenger`
- `Bloodchief's Thirst`
- `Bloodstained Mire`
- `Blooming Marsh`
- `Bolas's Citadel`
- `Bone Shards`
- `Bonecrusher Giant`
- `Botanical Sanctum`
- `Bountiful Landscape`
- `Brain Freeze`
- `Brainstorm`
- `Breeding Pool`
- `Brightglass Gearhulk`
- `Bristly Bill, Spine Sower`
- `Candelabra of Tawnos`
- `Cankerbloom`
- `Cathar Commando`
- `Cecil, Dark Knight`
- `Celestial Colonnade`
- `Chain Lightning`
- `Chain of Smog`
- `Chandra, Torch of Defiance`
- `Channel`
- `Coalition Relic`
- `Collective Brutality`
- `Commercial District`
- `Concealed Courtyard`
- `Containment Priest`
- `Copperline Gorge`
- `Corpse Dance`
- `Council's Judgment`
- `Counterspell`
- `Crop Rotation`
- `Crucible of Worlds`
- `Cryptic Command`
- `Cut Down`
- `Dack Fayden`
- `Damn`
- `Dark Ritual`
- `Darkslick Shores`
- `Daze`
- `Deathrite Shaman`
- `Deep-Cavern Bat`
- `Delayed Blast Fireball`
- `Delighted Halfling`
- `Demonic Tutor`
- `Descendant of Storms`
- `Doomsday`
- `Dreadhorde Arcanist`
- `Duelist of the Mind`
- `Duress`
- `Echo of Eons`
- `Elite Spellbinder`
- `Elvish Mystic`
- `Embereth Shieldbreaker`
- `Emperor of Bones`
- `Enduring Innocence`
- `Entomb`
- `Eternal Witness`
- `Exhume`
- `Expedition Map`
- `Faithless Looting`
- `Fallen Shinobi`
- `Fatal Push`
- `Figure of Destiny`
- `Fireblast`
- `Firebolt`
- `Flame Slash`
- `Flame of Anor`
- `Flooded Strand`
- `Force of Negation`
- `Force of Vigor`
- `Force of Will`
- `Forensic Gadgeteer`
- `Frantic Search`
- `Fury`
- `Gaea's Cradle`
- `Gau, Feral Youth`
- `Generous Ent`
- `Get Lost`
- `Ghost Vacuum`
- `Glorybringer`
- `Goblin Bombardment`
- `Goblin Rabblemaster`
- `Godless Shrine`
- `Grim Monolith`
- `Griselbrand`
- `Gush`
- `Gut, True Soul Zealot`
- `Hallowed Fountain`
- `Haywire Mite`
- `Horizon Canopy`
- `Hymn to Tourach`
- `Indatha Triome`
- `Inspiring Vantage`
- `Ivora, Insatiable Heir`
- `Jace, Wielder of Mysteries`
- `Jace, the Mind Sculptor`
- `Jacked Rabbit`
- `Jetmir's Garden`
- `Karakas`
- `Ketria Triome`
- `Kitesail Freebooter`
- `Laelia, the Blade Reforged`
- `Lavaspur Boots`
- `Ledger Shredder`
- `Legion Extruder`
- `Library of Alexandria`
- `Life // Death`
- `Lightning Bolt`
- `Lightning Greaves`
- `Liliana of the Veil`
- `Lingering Souls`
- `Lion Sash`
- `Llanowar Elves`
- `Lotus Petal`
- `Lush Portico`
- `Lórien Revealed`
- `Magda, Brazen Outlaw`
- `Mana Confluence`
- `Mana Crypt`
- `Mana Drain`
- `Mana Leak`
- `Mana Tithe`
- `Mana Vault`
- `Manamorphose`
- `Manifold Key`
- `Marsh Flats`
- `Metamorphosis Fanatic`
- `Meticulous Archive`
- `Mind Stone`
- `Mind Twist`
- `Mine Collapse`
- `Miscalculation`
- `Mishra's Workshop`
- `Misty Rainforest`
- `Mother of Runes`
- `Mox Diamond`
- `Mox Emerald`
- `Mox Jet`
- `Mox Pearl`
- `Mox Ruby`
- `Mox Sapphire`
- `Mystic Confluence`
- `Mystical Tutor`
- `Natural Order`
- `Necromancy`
- `Nettlecyst`
- `Night's Whisper`
- `Nissa, Who Shakes the World`
- `Occult Epiphany`
- `Ocelot Pride`
- `Omnath, Locus of Creation`
- `Orcish Bowmasters`
- `Overgrown Tomb`
- `Overlord of the Balemurk`
- `Palace Jailer`
- `Paradoxical Outcome`
- `Parallax Wave`
- `Path to Exile`
- `Phantasmal Image`
- `Phlage, Titan of Fire's Fury`
- `Plateau`
- `Polluted Delta`
- `Ponder`
- `Portable Hole`
- `Preordain`
- `Primeval Titan`
- `Prismatic Ending`
- `Psychic Frog`
- `Pyrogoyf`
- `Pyrokinesis`
- `Raffine's Tower`
- `Rancor`
- `Raugrin Triome`
- `Razorverge Thicket`
- `Reanimate`
- `Reprieve`
- `Riverpyre Verge`
- `Sacred Foundry`
- `Saheeli, Sublime Artificer`
- `Savai Triome`
- `Savannah`
- `Scalding Tarn`
- `Scrubland`
- `Seachrome Coast`
- `Securitron Squadron`
- `Shallow Grave`
- `Sheoldred, the Apocalypse`
- `Skullclamp`
- `Snapcaster Mage`
- `Snuff Out`
- `Sol Ring`
- `Sowing Mycospawn`
- `Spara's Headquarters`
- `Spell Pierce`
- `Spellseeker`
- `Spirebluff Canal`
- `Static Prison`
- `Steam Vents`
- `Stern Scolding`
- `Stock Up`
- `Stomping Ground`
- `Stoneforge Mystic`
- `Stormchaser's Talent`
- `Strip Mine`
- `Subtlety`
- `Sunbaked Canyon`
- `Sunfall`
- `Sword of the Meek`
- `Swords to Plowshares`
- `Sylvan Caryatid`
- `Sylvan Safekeeper`
- `Taiga`
- `Tamiyo, Collector of Tales`
- `Temple Garden`
- `Tendrils of Agony`
- `Thalia, Guardian of Thraben`
- `Thassa's Oracle`
- `Thespian's Stage`
- `Thought Scour`
- `Thoughtseize`
- `Through the Breach`
- `Thundering Falls`
- `Tidehollow Sculler`
- `Tifa Lockhart`
- `Time Spiral`
- `Time Walk`
- `Time Warp`
- `Timetwister`
- `Titania, Protector of Argoth`
- `Tolarian Academy`
- `Toxic Deluge`
- `Treasure Cruise`
- `Tropical Island`
- `Tundra`
- `Umezawa's Jitte`
- `Underground Sea`
- `Underworld Breach`
- `Unexpectedly Absent`
- `Unholy Heat`
- `Unruly Krasis`
- `Upheaval`
- `Urborg, Tomb of Yawgmoth`
- `Ursine Monstrosity`
- `Vampiric Tutor`
- `Verdant Catacombs`
- `Vindicate`
- `Vivi Ornitier`
- `Voice of Victory`
- `Volcanic Island`
- `Walk-In Closet // Forgotten Cellar`
- `Walking Ballista`
- `Wasteland`
- `Wastewood Verge`
- `Watery Grave`
- `Wheel of Fortune`
- `Winds of Abandon`
- `Windswept Heath`
- `Wishclaw Talisman`
- `Wooded Foothills`
- `Worldspine Wurm`
- `Wrath of God`
- `Xander's Lounge`
- `Yavimaya, Cradle of Growth`
- `Zagoth Triome`
- `Ziatora's Proving Ground`
- `Zuran Orb`
