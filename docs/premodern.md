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
  Marvin's Angry Hermit. Every card in them is complete

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
explicit unsupported declarations.

The format is offered in the web client, and its picker lists exactly the
decks the engine has registered. Whole-game coverage matches the other two
formats: the deferred sweeps play every registered Premodern matchup to a
result and rebuild sampled Premodern positions from their observations.

## Remaining format work

- Replace unsupported inventory declarations with complete declarative cards and
  Oracle clauses. Unsupported cards must remain explicit rather than become
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

## Coverage

The card declarations and their inline `// Audit:` entries are the authoritative
implementation inventory. Run `make catalog-report` for current Premodern
coverage counts. The report partitions the format's full twenty-nine-set legal
window into declarative, unsupported, and blocked cards, so
it gives a rough picture of how much of Premodern is implemented rather than an
interaction audit limited to these eight decks.

[tournament]: https://melee.gg/Tournament/View/441083
[rules]: https://premodernmagic.com/
