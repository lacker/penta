# Premodern implementation roadmap

Penta's first Premodern tranche is the Top 8 of the 109-player [Sacred Torch
Showdown 2026][tournament], played on July 25, 2026. The submitted lists are
staged under `decks/premodern/`, and all eight are now exposed as built-in
playable decks. Each was promoted only once every card in it resolved:
publishing one earlier would have created legal actions the engine could not
carry out.

## Snapshot

- 8 complete submitted main decks and 15-card sideboards captured (seven have
  60 cards; Drew Glauberg's Stasis list has 61)
- 145 distinct cards across the tranche, all of them cataloged
- All 8 lists registered and playable: Neal Sacks's Sligh, Daniel Sondike's
  GAT, Bryan Gulotta's Replenish, Drew Glauberg's Stasis, Chris Danis's BW
  Control, TentacleFan's Landstill, Andy Dominguez's RG Goblins, and Ryan
  Marvin's Angry Hermit. Nothing in any of them is metadata-only or partial
- per-card catalog and interaction-audit status tracked in the inventory below

Nothing is outstanding: every card in the tranche is cataloged, and every
list is registered and playable.

The eight decks, in finish order, are Neal Sacks's Sligh, Daniel
Sondike's GAT, Bryan Gulotta's Replenish, Drew Glauberg's Stasis, Chris Danis's
BW Control, TentacleFan's Landstill, Andy Dominguez's RG Goblins, and Ryan
Marvin's Angry Hermit.

## Format profile

`Format::Premodern` is in place: the twenty-nine-set window from Fourth
Edition through Scourge, the format's own thirty-three-card ban list, no
restricted list, and contemporary mana rules. All three are taken from the
[Premodern rules page][rules]. Every identity in those twenty-nine sets now has
an ordered source entry and catalog representation; unimplemented cards are
explicit metadata-only stubs.

The format is offered in the web client, and its picker lists exactly the
decks the engine has registered. Whole-game coverage matches the other two
formats: the deferred sweeps play every registered Premodern matchup to a
result and rebuild sampled Premodern positions from their observations.

## Remaining format work

- Replace metadata-only inventory stubs with accurate characteristics and
  Oracle clauses. Unsupported clauses must remain explicit rather than become
  executable no-ops.
- Implement reusable mechanics before card-local behavior. Cycling and
  typecycling, flashback, split cards, tutors, alternative costs that exile a
  card from hand, and single-card reanimation are all in place. One named
  mechanic is left, and it is the last card too. Phasing landed: a
  phased-out permanent is held apart from the battlefield rather than left
  on it behind a flag, so all hundred-odd walks over the battlefield are
  right without knowing phasing exists, and it phases in before its
  controller untaps. A run of sacrifices survives a checkpoint too: it is one
  resolution answered a creature at a time, so what it carries is that
  resolution beside how much is still owed. A phased-out permanent now survives a
  checkpoint: it is public information -- both players see it, and only the
  rules treat it as absent -- so it is shown behind a `phasedOut` flag and
  reconstruction routes it back to the phased-out list. Engine legality
  reads the battlefield, which excludes it, so nothing illegal follows from
  showing it. Vision Charm's other mode still
  needs work, and the shape is known: Magical Hack already offers a choice
  of an ordered pair of basic land types as one decision, so that option
  encoding is reusable. What is missing is a continuation that applies the
  answer -- setting the basic land type of every land of the first type to
  the second until end of turn, with CR 305.7's consequences, which
  `SetOperationDef::Set` already carries. The applied effect cannot go
  through the ordinary `Apply` path because its recipient and operation are
  chosen rather than authored, so it needs one static slice per basic land
  type to name the result. Restricting the first choice to basic land types
  narrows the printed "a land type", which nothing in this card pool
  exercises: no land in the tranche carries a nonbasic land subtype. Fading landed with
  Parallax Wave and needed only a counter kind: entering with counters, an
  upkeep trigger reading its own counters, and exiles linked to a source
  were all already there, so fading is the shape those pieces make rather
  than a mechanic of its own. Replenish itself needed no new mechanic
  either: returning every enchantment card at once is the ordinary zone
  move over a query. Opalescence needed no rewrite either, only a
  static power-and-toughness amount read from the affected object rather
  than from the effect's source, and a slightly wider stratified vocabulary
  for an animation's own query -- still excluding the basic land subtypes,
  which a static effect can itself supply. Morph landed with Exalted Angel: a permanent can be
  face down, presenting a shared 2/2 body with no name, and the special
  action that turns it up reads the morph cost off the physical card. Storm landed with Brain Freeze, along with a
  spell's own cast trigger and a copy chain; countering an ability landed
  with Teferi's Response and Stifle followed it; and the
  Stasis tranche added payments that return or sacrifice a named permanent
  and an additional cost counted in X. Naming a card and reading the name back,
  arranging the top of a library, spending a land's counters, and a payment
  that discards a card matching a predicate all landed with the GAT tranche.
- Audit the existing definitions against their Premodern Oracle text and
  interactions.
- Promote each staged deck into the runtime registry only when every main-deck
  card is playable and the sideboard has honest catalog coverage. GAT, RG
  Goblins, and Sligh are registered; the other five lists remain staged.
  Registration is also what first checks a list against the format's set
  window: promoting GAT found seven of its cards cataloged only from
  printings outside it, Landstill four more, and Stasis three.
- Keep the web deck notes in step with the registry as lists are promoted;
  `web/app/game-config.ts` and the deck names in `src/protocol/decks.rs` are
  checked against each other by the browser contract suite.

## Card inventory

Already cataloged (status annotations record the completed interaction audits;
older unannotated definitions still require one):

- `Abeyance` — complete; the lock spares mana abilities and nothing else
- `Adarkar Wastes` — complete
- `Akroma's Vengeance` — complete
- `Ancient Tomb` — complete
- `Annul` — complete
- `Arcane Denial` — complete; both draws wait a turn
- `Armageddon`
- `Attunement` — complete; the enchantment is the cost and comes back to be it again
- `Aura of Silence` — complete
- `Barbarian Ring` — complete
- `Black Vise`
- `Blue Elemental Blast`
- `Brain Freeze` — complete; storm copies what came before it
- `Cabal Therapy` — complete; the guess takes every copy of the name
- `Caves of Koilos` — complete
- `Cephalid Coliseum` — complete
- `Chain of Vapor` — complete; the chain is the opponent's to continue
- `Chill` — complete
- `Circle of Protection: Red` — complete; Fourth Edition brings it inside the window
- `City of Brass`
- `Claws of Gix` — complete
- `Coastal Tower` — complete
- `Counterspell`
- `Cursed Scroll` — complete; naming a card is modelled as naming one held
- `Cursed Totem` — complete
- `Daze` — complete
- `Decree of Justice` — complete; cycling buys Soldiers by the mana
- `Decree of Silence` — complete; three answered spells is as many as it gets
- `Defense Grid` — complete; the tax falls on the seat holding up an answer
- `Disenchant`
- `Dragon Breath` — complete; it listens from the graveyard and returns attached
- `Duress`
- `Dust Bowl` — complete
- `Earthquake`
- `Engineered Plague` — complete
- `Enlightened Tutor` — complete
- `Eternal Dragon` — complete
- `Exalted Angel` — complete; face down for three, face up for its morph cost
- `Fact or Fiction` — complete
- `Fire // Ice` — complete
- `Fireblast` — complete
- `Flash of Insight` — complete; the flashback exiles X blue cards
- `Flooded Strand` — complete
- `Forest`
- `Forsaken City` — complete
- `Frantic Search` — complete; the lands untap after the discard
- `Gempalm Incinerator` — complete
- `Gemstone Mine` — complete
- `Gerrard's Verdict` — complete; the life is counted after the discard
- `Gilded Drake` — complete; the exchange reads both seats before either moves
- `Goblin Lackey` — complete
- `Goblin Matron` — complete
- `Goblin Patrol` — complete
- `Goblin Piledriver` — complete
- `Goblin Pyromancer` — complete
- `Goblin Ringleader` — complete
- `Goblin Sharpshooter` — complete
- `Goblin Tinkerer` — complete
- `Goblin Vandal` — complete
- `Goblin Warchief` — complete
- `Grim Lavamancer` — complete
- `Gush` — complete
- `Haunting Echoes` — complete; the library copies follow what the graveyard lost
- `Hermit Druid` — complete; a library with no basic land empties
- `Humility` — complete
- `Hydroblast` — complete
- `Impulse` — complete
- `Incinerate` — complete; the rider follows the damage, not the target
- `Intuition` — complete; the opponent picks out of the three that were found
- `Island`
- `Jackal Pup` — complete
- `Karplusan Forest` — complete
- `Kor Haven` — complete
- `Krosan Reclamation` — complete; the cards are picked out of the targeted graveyard on resolution
- `Lightning Bolt`
- `Llanowar Wastes` — complete
- `Lotus Petal` — complete
- `Mana Leak` — complete
- `Mana Short` — complete
- `Meddling Mage` — complete; the lock is symmetric and leaves with it
- `Mishra's Factory`
- `Mogg Fanatic` — complete
- `Mogg Salvage` — complete
- `Monk Realist` — complete
- `Mountain`
- `Mox Diamond` — complete; an unpaid entry is replaced, not undone
- `Naturalize` — complete
- `Opalescence` — complete; each other enchantment is the size of its own cost
- `Opt` — complete
- `Overload` — complete
- `Parallax Wave` — complete; fading spent on creatures, and all of them return
- `Phyrexian Arena` — complete
- `Phyrexian Dreadnought` — complete; twelve power fed one creature at a time
- `Phyrexian Furnace` — complete; the tap mode eats the oldest card
- `Plains`
- `Portent` — complete; the arrangement is the order the cards are named
- `Powder Keg` — complete
- `Presence of the Master` — complete
- `Prohibit` — complete
- `Psychatog` — complete
- `Pyroblast` — complete
- `Pyrokinesis` — complete
- `Quirion Dryad` — complete
- `Ray of Revelation`
- `Reanimate` — complete
- `Red Elemental Blast`
- `Reflecting Pool` — complete; a type rather than a colour, from your own lands
- `Replenish` — complete; every enchantment card you own, all at once
- `Rishadan Port` — complete
- `Root Maze` — complete
- `Seal of Cleansing` — complete
- `Seal of Fire` — complete
- `Secluded Steppe` — complete
- `Shallow Grave` — complete; the creature carries its own end-step exile
- `Siege-Gang Commander` — complete
- `Skeletal Scrying` — complete; the graveyard pays for the cards
- `Skirk Prospector` — complete
- `Skycloud Expanse` — complete; two unlike mana from one activation
- `Sleight of Hand` — complete
- `Standstill` — complete
- `Stasis` — complete
- `Stifle` — complete; an ability only, never a spell
- `Sutured Ghoul` — complete; its body is the pile it exiled on the way in
- `Swamp`
- `Swords to Plowshares`
- `Sylvan Safekeeper` — complete
- `Syncopate`
- `Teferi's Response` — complete; the countered ability's source dies with it
- `Thawing Glaciers` — complete; the return is a cleanup-step trigger
- `Thwart` — complete
- `Tormod's Crypt` — complete
- `Tranquil Domain` — complete
- `Treva's Ruins` — complete
- `Tsabo's Web` — complete
- `Underground River` — complete
- `Upheaval` — complete
- `Vindicate` — complete
- `Vision Charm` — complete; all three modes, including phasing
- `Volcanic Hammer` — complete
- `Warmth` — complete
- `Wasteland` — complete
- `Wooded Foothills` — complete
- `Worldly Tutor` — complete
- `Wrath of God`
- `Yavimaya Coast` — complete

Not yet cataloged: none.

[tournament]: https://melee.gg/Tournament/View/441083
[rules]: https://premodernmagic.com/
