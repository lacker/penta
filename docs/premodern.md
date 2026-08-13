# Premodern implementation roadmap

Penta's first Premodern tranche is the Top 8 of the 109-player [Sacred Torch
Showdown 2026][tournament], played on July 25, 2026. The submitted lists are
staged under `decks/premodern/` and covered by a repository test, but they are
not exposed as built-in playable decks yet. Publishing them before their cards
have honest execution coverage would create legal actions that cannot resolve
correctly.

## Snapshot

- 8 complete submitted main decks and 15-card sideboards captured (seven have
  60 cards; Drew Glauberg's Stasis list has 61)
- 145 distinct cards across the tranche
- per-card catalog and interaction-audit status tracked in the inventory below

The eight staged decks, in finish order, are Neal Sacks's Sligh, Daniel
Sondike's GAT, Bryan Gulotta's Replenish, Drew Glauberg's Stasis, Chris Danis's
BW Control, TentacleFan's Landstill, Andy Dominguez's RG Goblins, and Ryan
Marvin's Angry Hermit.

## Remaining format work

- Add the `Premodern` format profile and its legal set window, from Fourth
  Edition through Scourge. Premodern uses contemporary game rules and its own
  maintained banned list; take both from the [Premodern rules page][rules] at
  implementation time.
- Add the missing printed-set modules and canonical printings. Definition IDs
  remain append-only even when a card belongs to an older set.
- Add accurate characteristics and Oracle clauses for uncataloged cards.
  Unsupported clauses must be metadata-only rather than executable no-ops.
- Implement reusable mechanics before card-local behavior: cycling and
  landcycling, flashback, threshold, fading, echo, alternative costs, split
  cards, graveyard replacement/reanimation, named-card choices, tutors,
  continuous type/PT changes, and triggered mana/untap restrictions.
- Audit the existing definitions against their Premodern Oracle text and
  interactions.
- Promote each staged deck into the runtime registry only when every main-deck
  card is playable and the sideboard has honest catalog coverage.
- Add protocol/binding documentation and UI format/deck selection when the
  profile becomes public.

## Card inventory

Already cataloged (status annotations record the completed interaction audits;
older unannotated definitions still require one):

- `Adarkar Wastes` — complete
- `Ancient Tomb` — complete
- `Annul` — complete
- `Armageddon`
- `Black Vise`
- `Blue Elemental Blast`
- `Caves of Koilos` — complete
- `City of Brass`
- `Claws of Gix` — complete
- `Coastal Tower` — complete
- `Counterspell`
- `Disenchant`
- `Duress`
- `Dust Bowl` — complete
- `Earthquake`
- `Enlightened Tutor` — complete
- `Fact or Fiction` — complete
- `Fire // Ice` — complete
- `Flooded Strand` — complete
- `Forest`
- `Goblin Pyromancer` — complete
- `Goblin Sharpshooter` — complete
- `Hydroblast` — complete
- `Impulse` — complete
- `Incinerate` — damage complete; no-regeneration rider is partial
- `Island`
- `Jackal Pup` — complete
- `Karplusan Forest` — complete
- `Kor Haven` — complete
- `Lightning Bolt`
- `Llanowar Wastes` — complete
- `Lotus Petal` — complete
- `Mana Leak` — complete
- `Mana Short` — complete
- `Mishra's Factory`
- `Mogg Fanatic` — complete
- `Monk Realist` — complete
- `Mountain`
- `Naturalize` — complete
- `Opt` — complete
- `Phyrexian Arena` — complete
- `Plains`
- `Presence of the Master` — complete
- `Pyroblast` — complete
- `Quirion Dryad` — complete
- `Ray of Revelation`
- `Red Elemental Blast`
- `Reanimate` — complete
- `Rishadan Port` — complete
- `Root Maze` — complete
- `Seal of Cleansing` — complete
- `Seal of Fire` — complete
- `Sleight of Hand` — complete
- `Stasis` — complete
- `Swamp`
- `Swords to Plowshares`
- `Syncopate`
- `Sylvan Safekeeper` — complete
- `Tormod's Crypt` — complete
- `Tranquil Domain` — complete
- `Tsabo's Web` — complete
- `Underground River` — complete
- `Upheaval` — complete
- `Vindicate` — complete
- `Volcanic Hammer` — complete
- `Warmth` — complete
- `Wasteland` — complete
- `Wooded Foothills` — complete
- `Worldly Tutor` — complete
- `Wrath of God`
- `Yavimaya Coast` — complete

Not yet cataloged:

- [ ] `Abeyance`
- [ ] `Akroma's Vengeance`
- [ ] `Arcane Denial`
- [ ] `Attunement`
- [ ] `Aura of Silence`
- [ ] `Barbarian Ring`
- [ ] `Brain Freeze`
- [ ] `Cabal Therapy`
- [ ] `Cephalid Coliseum`
- [ ] `Chain of Vapor`
- [ ] `Chill`
- [ ] `Circle of Protection: Red`
- [ ] `Cursed Scroll`
- [ ] `Cursed Totem`
- [ ] `Daze`
- [ ] `Decree of Justice`
- [ ] `Decree of Silence`
- [ ] `Defense Grid`
- [ ] `Dragon Breath`
- [ ] `Engineered Plague`
- [ ] `Eternal Dragon`
- [ ] `Exalted Angel`
- [ ] `Fireblast`
- [ ] `Flash of Insight`
- [ ] `Forsaken City`
- [ ] `Frantic Search`
- [ ] `Gempalm Incinerator`
- [ ] `Gemstone Mine`
- [ ] `Gerrard's Verdict`
- [ ] `Gilded Drake`
- [ ] `Goblin Lackey`
- [ ] `Goblin Matron`
- [ ] `Goblin Patrol`
- [ ] `Goblin Piledriver`
- [ ] `Goblin Ringleader`
- [ ] `Goblin Tinkerer`
- [ ] `Goblin Vandal`
- [ ] `Goblin Warchief`
- [ ] `Grim Lavamancer`
- [ ] `Gush`
- [ ] `Haunting Echoes`
- [ ] `Hermit Druid`
- [ ] `Humility`
- [ ] `Intuition`
- [ ] `Krosan Reclamation`
- [ ] `Meddling Mage`
- [ ] `Mogg Salvage`
- [ ] `Mox Diamond`
- [ ] `Opalescence`
- [ ] `Overload`
- [ ] `Parallax Wave`
- [ ] `Phyrexian Dreadnought`
- [ ] `Phyrexian Furnace`
- [ ] `Portent`
- [ ] `Powder Keg`
- [ ] `Prohibit`
- [ ] `Psychatog`
- [ ] `Pyrokinesis`
- [ ] `Reflecting Pool`
- [ ] `Replenish`
- [ ] `Secluded Steppe`
- [ ] `Shallow Grave`
- [ ] `Siege-Gang Commander`
- [ ] `Skeletal Scrying`
- [ ] `Skirk Prospector`
- [ ] `Skycloud Expanse`
- [ ] `Standstill`
- [ ] `Stifle`
- [ ] `Sutured Ghoul`
- [ ] `Teferi's Response`
- [ ] `Thawing Glaciers`
- [ ] `Thwart`
- [ ] `Treva's Ruins`
- [ ] `Vision Charm`

[tournament]: https://melee.gg/Tournament/View/441083
[rules]: https://premodernmagic.com/
