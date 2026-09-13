# Commander formats

`cedh` uses Commander rules in the existing two-player engine: 40 starting life,
seven-card opening hands, the ordinary two-player first-draw and London mulligan
rules, and no multiplayer free mulligan. It is a development baseline for cEDH
card interactions; multiplayer remains deferred.

`duel-commander` implements the Duel Commander committee’s one-on-one rules:
20 starting life, the London mulligan and ordinary first-draw rule, and no
commander-damage loss condition. It has its own ban metadata, distinct from
Wizards Commander policy.

## Decks and identity

`Deck.commanders` and a deck YAML's `commanders` section designate physical cards
separately from `main` and `sideboard`. Setup puts them face up in the command
zone before play, excluding them from opening hands and library shuffles. A
single list supports one commander, Partners, or a commander and a Background;
the shared construction vocabulary also represents Partner and Choose a
Background permissions. The designation is not a copied characteristic and
survives control and zone changes.

Full construction and format legality are deferred. Both formats check catalog
identity but do not enforce commander eligibility, pairing permissions,
colour identity, singleton, deck size, sideboard rules, or bans. Named ban lists are metadata: Wizards for cEDH and the Duel
Commander committee for Duel Commander. Commander-only and companion-only restrictions
are separate from general bans.
An unsupported card has its identity and source printing in the catalog, but
cannot be cast or executed as a partially implemented card.

## Gameplay and consumers

Designated commanders can be cast from their owner's command zone through the
ordinary legal-action interface, with normal timing, targeting and payment.
Each physical card tracks its command-zone casts separately. Its additional
`{2}` per previous command-zone cast is applied with cost increases before
reductions, including alternative costs. Casts from other zones do not add tax.

An owner may replace a commander's move to hand or library with a move to the
command zone. Graveyard and exile returns are optional state-based actions,
offered once for each new incarnation. A blink that returns during the same
resolution therefore offers no intermediate return. Each designated card also
tracks combat damage dealt to each player; in cEDH, 21 from one commander causes
a loss, regardless of its controller. Partners' damage is separate. Duel
Commander records the same history without using it as a loss condition.

In Duel Commander, the first commander cast from the command zone locks the
other designated commander out of command-zone casting for the rest of that
game, even if the spell is countered. Casting from another zone does not choose
or change this restriction. The existing per-card cast counts preserve it
through checkpoints; a new game or restart resets the counts.

In a best-of-three Duel Commander match, each player may select one or two
commanders from their original deck pool between games. Choices remain private
until both players submit and the next game begins. This reuses the generic
between-games deck-selection stage. As with initial construction, commander
eligibility, permitted pairings and unchanged color identity are not verified
yet. Restarts keep the current game’s commanders. Effects cannot retrieve cards
from outside the game in Duel Commander; the companion special action remains
available. Duel Commander has no sideboard under its rules, although setup does
not yet enforce deck construction constraints.

Canonical observations add `commandZones` and `commanders`; the latter reports
owner, definition, command-zone cast count and per-seat combat damage. `objectId`
is absent for a commander in an unobservable zone. Browser snapshots adapt the
same engine state into command-zone cards and history. Return choices use the
existing generic decision interface. `rules.commander.v1` advertises these
additions; clients selecting `cedh` must understand that capability and its
`OpponentCommanderDamage` result reason. `duel-commander` additionally requires
`rules.duel-commander.v1`, covering its cast restriction, between-game commander
selection, and outside-game effect policy.

Observation checkpoints preserve visible commander identity and history, and
pending graveyard/exile return choices. Hidden-zone reconstruction binds a
commander only when the supplied hypothesis identifies exactly one matching
physical card; ambiguous hypotheses fail closed. As with the existing
battlefield exit-replacement pipeline, checkpoints paused inside a hand/library
replacement choice are not yet reconstructible. Command journals can replay
those choices through the authoritative action interface.

## Seed decklists

[`decks/cedh`](../decks/cedh/README.md) seeds cEDH with the top 16 competitive
lists from Nacional de cEDH 100K @ WolfCon 2026. These are starting points for
the deck catalog and can evolve independently of the event. Including a list
does not imply complete card support or legality. Catalog coverage reports
measure the cards used by these decks.

[`decks/duel_commander`](../decks/duel_commander/README.md) contains the eight
published top-eight lists from CommandFest Italy 2026 in Bologna.

## Sources

Wizards sources were consulted on 2026-09-11; Duel Commander committee sources
on 2026-09-12 (ban policy dated 2026-07-27):

- [Wizards Commander format](https://magic.wizards.com/en/formats/commander).
- [Wizards banned and restricted cards](https://magic.wizards.com/en/banned-restricted-list).
- [Wizards Comprehensive Rules](https://magic.wizards.com/en/rules), especially
  903.3 (designation), 903.8 (casting), 903.9 (returns), 903.10a (damage), and
  702.124 (Partner and Background).
- [EDHTop16 tournament](https://edhtop16.com/tournament/nacional-de-cedh-100k-1).

- [Duel Commander comprehensive rules](https://www.duelcommander.org/rules/duelcommander_comprehensiverules/).
- [Duel Commander banlist](https://www.duelcommander.org/banlist/).
- [CommandFest Italy event](https://mtgtop8.com/event?e=90544&f=EDH).
