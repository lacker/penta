# Formats and current scope

This document records the formats, decks, rules deviations, and implementation
coverage available in the current engine. It describes product scope rather
than engine architecture; the catalog exposed by each binding remains the
authoritative machine-readable account of card legality and coverage.

## Supported formats

Penta currently ships two explicit format profiles:

- **Eternal Central Old School 93/94**: the original card pool, EC banned and
  restricted lists, phase-boundary mana burn, and fifteen powered archetypes.
- **ISD–RTR Standard (final pre-Theros snapshot)**: Innistrad, Dark Ascension,
  Avacyn Restored, Magic 2013, Return to Ravnica, Gatecrash, Dragon's Maze,
  and Magic 2014; no banned or restricted cards; modern mana-pool emptying with
  no mana burn; and the eight decks from SCG Open Atlanta in September 2013.

Both use 20 starting life, 60-card minimum decks, sideboards of up to 15 cards,
and a four-copy limit except for basic lands. Both currently use London
mulligans.

The selected format is stored on each game. Format-specific construction and
mana rules live in one profile rather than as global switches, so adding a
format does not change existing games.

## Planned formats

Premodern is staged as the next format expansion. The exact Top 8 decklists
from the July 2026 Sacred Torch Showdown and the card-by-card implementation
backlog are tracked in the [Premodern roadmap](premodern.md). These lists are
not advertised as playable until their catalog and rules coverage are honest.

### Old School 93/94

The profile includes Alpha, Beta, Unlimited, Collector's Edition,
International Collector's Edition, Arabian Nights, Antiquities, Revised,
Legends, The Dark, Fallen Empires, and the three 1994 promotional cards. It
uses the Eternal Central banned and restricted list and current Magic rules
except where [Eternal Central's 93/94 rules][ec-rules] differ, notably mana
burn. Paper-only reprint policies have no meaning in the simulator.

### ISD–RTR Standard

The profile represents the final Standard environment before Theros. It uses
its eight-set legality snapshot, empties mana after each step and phase, and has
no mana burn.

## Engine coverage

The engine currently supports:

- per-game legality, construction, banned/restricted, and mana-pool profiles;
- seeded reproducible setup and hidden-information-safe observations;
- deterministic legal actions, priority-bearing turn steps, and the stack;
- spells and supported non-mana activated and triggered abilities as
  independently addressable stack objects;
- land plays, colored and colorless mana, generic and variable-X costs, and
  phase-boundary mana burn where the format requires it;
- player damage, concession, and empty-library loss conditions;
- public battlefield, graveyard, exile, and stack observations;
- an omniscient event log for replay and debugging consumers;
- London mulligans and player-selected cleanup discards;
- staged combat declaration, player-selected combat-damage assignment,
  first-strike and regular damage waves, and trample;
- summoning sickness, haste, temporary modifiers, marked damage, and death;
- multi-target spells, copy retargeting, activated and triggered choices,
  restricted untaps, and structured decisions;
- an identity-complete Old School implementation audit inline in the printed
  set modules, with executable declarative records for the supported tranche
  and a collector-ordered, named engine-capability gap for every incomplete
  legal card;
- declarative records for every additional card in the Standard Top 8 decks;
  and
- twenty-three fixed 60-card decks with 15-card sideboards across both formats.

The engine is playable end to end but is not a general implementation of the
Comprehensive Rules. Interactions are implemented to the depth required by the
supported tranche. Unsupported clauses remain visible through derived
Complete, Partial, or MetadataOnly coverage. Metadata-only noncreature spells
are withheld from legal actions rather than resolving as silent no-ops.

Representative deeper implementations include Fireball's multi-target
additional cost and damage division, Fork's copy retargeting, player-selected
combat damage, simultaneous trigger ordering, and frozen source, target, and
event information for stack abilities. Explicit mana abilities remain
immediate rather than using the stack.

## Built-in decks

### Old School 93/94

The fifteen built-in EC archetypes are:

- `decks::goblins()`: tribal aggro built around Goblin King, Goblin Grenade,
  Goblin Balloon Brigade, and Goblins of the Flarg.
- `decks::sligh()`: curve-based aggro and burn with Ironclaw Orcs, Ball
  Lightning, Granite Gargoyle, Dragon Whelp, and direct damage.
- `decks::artifacts()`: Atog Smash using Atog, Orcish Mechanics, Black Vise,
  Ankh of Mishra, Copper Tablet, and fast artifact mana.
- `decks::robots()`: Mana Vault into Juggernaut, Su-Chi, and Triskelion,
  backed by Atog and red removal.
- `decks::the_deck()`: the format's namesake control strategy with permission,
  Balance, Demonic Tutor, Jayemdae Tome, and restricted card draw.
- `decks::mono_black()`: Dark Ritual, Hypnotic Specter, Hymn to Tourach,
  Sinkhole, and Juzam Djinn.
- `decks::white_weenie()`: Savannah Lions and Icatian Javelineers into Crusade
  and Armageddon.
- `decks::erhnamgeddon()`: Birds of Paradise and Erhnam Djinn with white
  removal and Armageddon.
- `decks::counterburn()`: blue-red tempo with Serendib Efreet, permission,
  Psionic Blast, and burn.
- `decks::lions_dib()`: the blue-white Savannah Lions and Serendib Efreet
  tempo shell.
- `decks::bwr_aggro()`: a black-white-red knight and burn aggro shell.
- `decks::gr_aggro()`: Kird Ape, Argothian Pixies, mana Elves, and pump
  spells.
- `decks::troll_disk()`: black-red Sedge Troll control with Nevinyrral's Disk
  and land destruction.
- `decks::jeskai_aggro()`: blue-white-red tempo with burn and permission.
- `decks::lions_dib_bolt()`: the Lion/Dib shell with a dedicated Bolt package.

Their cores draw from the [TC Decks Goblins aggregate][goblins-data], the
[Wak-Wak Sligh guide][sligh-guide], a representative [EC Atog Smash
list][atog-list], the [TC Decks Artifact Aggro aggregate][robots-data], and the
[TC Decks The Deck aggregate][the-deck-data]. The corpus follows cards used by
real archetypes rather than every card technically legal in the format.

Chaos Orb normally requires physical dexterity. The headless engine instead
models a deterministic successful flip against the selected permanent. Its
activation uses the stack; removing the source before resolution leaves the
ability object in place but causes the card-specific source-presence check to
nullify the flip.

### ISD–RTR Standard

The profile contains the complete main deck and sideboard for each member of
the [SCG Open Atlanta Top 8][scg-atlanta]:

- Rudy Briksza — Naya Midrange
- Joseph Greer — G/R Aggro
- Mike Fyrberg — B/G Midrange
- Jimmie Smith — Naya Midrange
- Korey McDuffie — U/W/R Flash
- Phillip Lorren — U/W Flash
- Clayton Arch — U/W Flash
- Drew Kuenzinger — Junk Reanimator

The published Clayton Arch list contains three copies of Celestial Purge, which
was not legal in this Standard pool. The built-in list uses Celestial Flare as
the likely transcription correction and records that inference in the deck
source comments.

Ordinary mana sources and creatures are playable. Specialized effects are
being added incrementally through shared rules primitives or card-scoped
execution. Card previews expose staged implementation coverage so bot and UI
consumers can distinguish supported behavior from metadata.

[ec-rules]: https://www.eternalcentral.com/9394rules/
[goblins-data]: https://www.tcdecks.net/archetype.php?archetype=Goblins&format=Old+School&src=all
[sligh-guide]: https://www.wak-wak.se/9394decks/sligh
[atog-list]: https://tappedout.net/mtg-decks/atog-smash-9394-1/
[robots-data]: https://www.tcdecks.net/archetype.php?archetype=Artifact+Aggro&format=Old+School&src=all
[the-deck-data]: https://www.tcdecks.net/archetype.php?archetype=The+Deck&format=Old+School&src=all
[scg-atlanta]: https://www.mtgtop8.com/event?e=5640&f=ST
