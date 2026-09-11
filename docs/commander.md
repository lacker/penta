# Commander foundation

`cedh` uses Commander rules in the existing two-player engine: 40 starting life,
seven-card opening hands, the ordinary two-player first-draw and London mulligan
rules, and no multiplayer free mulligan. It is a development baseline for cEDH
card interactions, not a multiplayer or Duel Commander implementation.

## Decks and identity

`Deck.commanders` and a deck YAML's `commanders` section designate physical cards
separately from `main` and `sideboard`. Setup puts them face up in the command
zone before play, excluding them from opening hands and library shuffles. A
single list supports one commander, Partners, or a commander and a Background;
the shared construction vocabulary also represents Partner and Choose a
Background permissions. The designation is not a copied characteristic and
survives control and zone changes.

Full construction and format legality are deferred. cEDH setup checks catalog
identity but does not enforce commander eligibility, pairing permissions,
colour identity, singleton, deck size, sideboard rules, or bans. The WotC named
ban list is available as metadata, with companion-only restrictions separate.
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
tracks combat damage dealt to each player; 21 from one commander causes a loss,
regardless of its controller. Partners' damage is separate.

Canonical observations add `commandZones` and `commanders`; the latter reports
owner, definition, command-zone cast count and per-seat combat damage. `objectId`
is absent for a commander in an unobservable zone. Browser snapshots adapt the
same engine state into command-zone cards and history. Return choices use the
existing generic decision interface. `rules.commander.v1` advertises these
additions; clients selecting `cedh` must understand that capability and its
`OpponentCommanderDamage` result reason.

Observation checkpoints preserve visible commander identity and history, and
pending graveyard/exile return choices. Hidden-zone reconstruction binds a
commander only when the supplied hypothesis identifies exactly one matching
physical card; ambiguous hypotheses fail closed. As with the existing
battlefield exit-replacement pipeline, checkpoints paused inside a hand/library
replacement choice are not yet reconstructible. Command journals can replay
those choices through the authoritative action interface.

## Tournament corpus

[`decks/cedh`](../decks/cedh/README.md) contains all 119 downloadable lists from
the 123-entry Nacional de cEDH 100K @ WolfCon 2026 tournament. Its provenance
records all entries, the four unavailable lists, original deck text, links and
auxiliary sticker sections. Importing a list does not imply complete card
support or legality. Catalog coverage reports measure the union of the imported
cards, not the full Commander-legal card pool.

## Sources

Rules and policy metadata were consulted on 2026-09-11:

- [Wizards Commander format](https://magic.wizards.com/en/formats/commander).
- [Wizards banned and restricted cards](https://magic.wizards.com/en/banned-restricted-list).
- [Wizards Comprehensive Rules](https://magic.wizards.com/en/rules), especially
  903.3 (designation), 903.8 (casting), 903.9 (returns), 903.10a (damage), and
  702.124 (Partner and Background).
- [EDHTop16 tournament](https://edhtop16.com/tournament/nacional-de-cedh-100k-1).
