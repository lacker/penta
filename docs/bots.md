# Writing an AI bot for Penta

penta is a deterministic engine for two-player Magic across set-based formats
and fixed-list cubes. Its profiles include Old School 93/94, Premodern, two
historical Standard windows, and two cubes. This guide is for writing a program
that plays it: from Python, C, C++, or Rust, against the included bots or
against itself.

This guide describes the current development wire contract, **protocol 29**,
which retains protocol 22's open-world model. Ignore JSON object members your bot does not use;
the epoch changes only when an existing field or tag is removed, renamed,
retyped, or reinterpreted. Additive fields and different legal actions expressed
through the existing indexed-action vocabulary do not move it.

`protocolCapabilities` advertises optional, named facilities. Ignore unknown
capabilities, and require one only when your implementation consumes it. Query
`protocol_version()`, `simulation_fingerprint()`, and `engine_version()` through
the selected binding. Pin the simulation fingerprint, rather than the package
version or wire epoch, alongside trained weights. Old School remains the default
for compatibility; new integrations should record and pass an explicit format
slug with each game.

A bot is a function from an **observation** (your seat's view of the game,
as JSON) to an **action index** (a position in that observation's
`legalActions` array). The engine validates every index against the legal
list, so an illegal move cannot even be expressed. Everything else —
mulligans, mana payment, combat — arrives as entries in that same list.

The included opponents:

- `random` — picks uniformly among legal actions. The sanity check: if your
  bot cannot beat noise, something is wrong. It plays a real, witless game
  rather than resigning, because nothing a bot can choose ends the game on
  the spot.
- `handcrafted` — a rules-based policy that plays lands on curve, attacks,
  blocks, and answers threats. The first real milestone.

Training-scale rollouts are intended to be practical on a laptop. Measure the
current checkout and your actual workload rather than relying on a historical
throughput number; the [performance guide](performance.md) has the reproducible
native-engine workflow.

## Quick start: Python

Requires Python 3.13+ and [rustup](https://rustup.rs), which installs the
repository's pinned Rust version automatically. From the repository root:

```bash
cd bindings/penta-py
cargo build --release
cp target/release/libpenta.dylib penta.so   # Linux: cp target/release/libpenta.so penta.so
python3 -c "import penta; print(penta.engine_version(), penta.protocol_version(), penta.simulation_fingerprint())"
```

(With [maturin](https://maturin.rs) installed, `maturin develop --release`
does the copy for you and installs into your virtualenv.)

Then, in a file next to `penta.so`:

```python
import json
import penta

game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=42)
while game.result() is None:
    observation = json.loads(game.observe())
    actions = observation["legalActions"]
    choice = actions[0]["index"]          # your bot's decision goes here
                                          # (nothing in the list resigns)
    game.act(choice)
print(game.result())                       # "p1", "p2", or "draw"
```

Old School remains the default for compatibility. Select Standard explicitly:

```python
game = penta.Game(
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    opponent="external",
    format="isd-m14-standard",
    seed=42,
)
```

A complete bot that plays lands, casts its biggest spells, attacks, and runs
100-game matches against both included opponents is in
[`examples/python/first_bot.py`](../examples/python/first_bot.py). Copy it next
to your built `penta.so` and run it.

The module surface:

| call | meaning |
| --- | --- |
| `penta.Game(p1_deck, p2_deck, opponent=, opponent_seat=, seed=, format=)` | start a game; `format` defaults to `"old-school-93-94"` and `opponent` is `"handcrafted"`, `"random"`, or `"external"` |
| `game.observe(seat=None)` | one seat's observation as JSON (default: the seat that must act) |
| `game.act(index)` | play one entry from `legalActions` |
| `game.choose_decision([ids])` | answer a multi-pick decision explicitly (see below) |
| `game.decision_seat()` | `"p1"` / `"p2"` / `None` when the game is over |
| `game.clone()` | an independent copy of the game — fork it, try a line, discard it |
| `penta.Game.from_observation(observation, hidden, rollout_seed=0)` | build a local rollout world from a hosted observation and hidden-zone hypothesis — how a hosted search bot gets something to clone |
| `game.hand(seat)`, `game.library(seat)` | a zone's real contents, unredacted — for simulating, not for playing |
| `game.set_hand(seat, defs)`, `game.set_library(seat, defs)` | say what a zone holds, in a fork |
| `game.result()` | `None`, `"p1"`, `"p2"`, or `"draw"` |
| `penta.catalog(format=)` | every canonical definition annotated with legality for the selected format, as JSON |
| `penta.deck_names(format=)` | the selected format's built-in decks |
| `penta.protocol_version()` | the breaking bot-wire epoch |
| `penta.simulation_fingerprint()` | the conservative simulation-source identity; pin this with trained weights |
| `penta.engine_version()` | the package release version, for provenance |

## Quick start: C and C++

```bash
cargo build --release -p penta-ffi
```

produces `target/release/libpenta_ffi.a` (and a shared library). Include
[`bindings/penta-ffi/include/penta.h`](https://github.com/lacker/penta/blob/main/bindings/penta-ffi/include/penta.h)
and link the library; the header documents every call and the ownership
rules. A complete program that plays full games through this interface is
[`bindings/penta-ffi/smoke.c`](https://github.com/lacker/penta/blob/main/bindings/penta-ffi/smoke.c):

```bash
cc mybot.c target/release/libpenta_ffi.a -I bindings/penta-ffi -o mybot
```

The C ABI is the same protocol with the same JSON: `penta_new` takes a
config, including an optional `"format"` slug; `penta_observe_json` returns an
observation; and `penta_act` takes an index. The original catalog and deck-name
functions remain Old School-compatible. New callers can use
`penta_catalog_json_for_format` and `penta_deck_names_for_format_json`.
`penta_legal_action_count` lets a minimal client act without parsing JSON at
all. `penta_protocol_version`, `penta_simulation_fingerprint`, and
`penta_engine_version` expose the same three identifiers as Python. From C++,
wrap the header and parse observations with any JSON library (e.g.
nlohmann/json). Anything else with a C FFI — Julia, Go, C# — can consume the
same library.

## Quick start: Rust

The engine is an ordinary crate. Depend on it by path (or git) and use the
same facade the bindings use:

```rust
use penta::protocol::{BotGame, Opponent};
use penta::{Format, PlayerId};

fn main() -> Result<(), String> {
    let mut game = BotGame::new_with_format(
        Format::IsdM14Standard,
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        Opponent::Handcrafted,
        PlayerId::Two,
        42,
    )?;
    while game.result().is_none() {
        let seat = game.decision_seat().ok_or("game ended without a result")?;
        let _observation = game.observe_json(seat);
        game.act(0)?; // your bot's index here
    }
    Ok(())
}
```

Rust bots can also implement the `penta::Policy` trait directly and skip
JSON entirely; that is how the built-in bots are written.

## Running matches

`penta-match` pits the built-in policies against each other, alternating
seats, with deterministic seeds:

```bash
cargo run --release --bin penta-match -- \
    --p1 random --p2 handcrafted --deck1 Sligh --deck2 "The Deck" \
    --games 100 --seed 1
```

A deck of `Random` (the default) rotates through the built-in list. For
your own bot, the harness in `examples/python/first_bot.py` shows the
pattern: a seed loop, one `penta.Game` per seed, win counting.

## Self-play

`opponent="external"` disables the built-in opponent entirely: the game
stops at **every** decision, and `decision_seat()` tells you whose it is.
One loop drives both sides — your current model against a frozen
checkpoint, or against another author's bot:

```python
game = penta.Game("Goblins", "White Weenie", opponent="external", seed=7)
while game.result() is None:
    seat = game.decision_seat()
    observation = json.loads(game.observe(seat))
    bot = my_model if seat == "p1" else frozen_model
    game.act(bot(observation))
```

Observations are per-seat and redacted: they never expose cards that seat has
not learned. A reveal effect can intentionally populate `lastSeenHand` with
the disclosed snapshot, but the opponent's live hidden hand remains a count.
Neither side of a self-play loop can accidentally inspect the true hidden
state through `observe()`.

Search bots build on the same surface plus one call: `game.clone()`
(`penta_clone` in C, plain `.clone()` on a Rust `BotGame`) forks a game
mid-state into an independent copy, the built-in opponent's state included.
Fork at a decision, roll each candidate action out to the end, play the
winner in the real game — the clone and the original never disturb each
other, and a clone fed the same indices replays byte-identically.


### Rolling out against worlds you cannot see

A clone forks the *true* state, hidden zones included. That is right for
self-play but wrong for a search bot in a hosted match: its rollouts must use
worlds consistent with its observation, not cards only the host knows.

The optional `reconstruction.checkpoint.v10` capability advertises a hidden-safe
current-state checkpoint in each observation. The checkpoint was introduced in
protocol 19, expanded in protocol 21 into the complete typed snapshot described
below, and given its own nested format version in protocol 22. Protocol 26's
format 5 restores creator-owned token and emblem characteristics through
semantic paths to their printed creating abilities, without synthetic card
definition IDs. Protocol 27's format 7 also preserves the standardized
rules-owned characteristics of face-down spells and permanents. Format 8 adds
semantic draw-replacement continuations, resumable object-set iteration, and
resolved player attack restrictions. Format 9 names sparse counter entries
rather than assigning counter names positions in a catalog-wide array, and
reconstructs the open named counter collections carried by players.
Format 10 represents resumable card handling as named object-collection
operations, so a checkpoint can preserve each collection, its ordering, the
player responsible for the next stage, and the ordinary effect that follows
it. Partition and group-choice stages retain their pile semantics. The format
also stores Quicken-style cast-timing permissions in the same resolved
permission collection as every other duration-bound timing grant, including
the composed end-of-turn-or-next-matching-cast expiration.
Stack objects and permanents preserve additive cast-context fields: the source
zone and selected alternative-cost kind, announced X and additional-cost
counts, colors and Phyrexian symbols actually paid, and the exile-zone object
IDs used to pay costs such as delve. A spell copy keeps copied casting choices
and references to payment objects, but clears its source zone and facts about
mana or life actually spent because the copy was not cast. The retired
`castTags` field remains readable for format-10 compatibility; new checkpoints
leave it empty and represent Escape as the `escape` alternative cast kind.
Supply a hypothesis for the zones the observation intentionally redacts, then
construct a live local game:

```python
view_json = hosted_observation
view = json.loads(view_json)
hidden = {
    "hands": {
        # Only the opposing hand is supplied. The observing seat's hand and
        # its public object IDs come from the observation.
        "p2": [mountain, lightning_bolt],
    },
    "libraries": {
        # Top card first; each length must match view["librarySizes"].
        "p1": my_library_hypothesis,
        "p2": their_library_hypothesis,
    },
    "outsideGame": {
        # Both lists are required, even when empty. They are cards currently
        # owned outside the game, such as each player's sideboard.
        "p1": my_outside_game_hypothesis,
        "p2": their_outside_game_hypothesis,
    },
}
world = penta.Game.from_observation(
    view_json,
    json.dumps(hidden),
    rollout_seed=1234,
)
```

The observation preserves every public object ID. Hypothesized hidden cards,
including outside-game cards, receive fresh IDs, so private identities cannot
collide with or disclose the host's objects. Outside-game contents are never
silently assumed empty: both `outsideGame` arrays must be present even when the
hypothesized world has none. `rollout_seed` controls random choices made
*after* local construction; it is not the host seed, and neither host seed nor
RNG state is ever present in `checkpoint`.

When opposing hand identity matters to a rule, the hypothesis can additionally
carry `drawnThisTurn: {"p2": [0, 2]}` using indexes into that hypothesized hand.
This belongs in the hidden hypothesis rather than the observation: even an
object ID with no card name can reveal which opposing card was drawn or
retained. Omit it when the hypothesized world has no such state.

If a suspended multi-player discard has already recorded opposing choices,
provide those as hand indexes too:

```python
hidden["decision"] = {
    "discardChoices": {
        "p2": [1, 3],
    },
}
```

The constructor asks for this only when the current continuation actually
contains such a hidden choice, and validates the number and range of indexes.

Construction fails closed when the bot-wire epoch, conservative simulation
fingerprint, or checkpoint `version` differs; when a hypothesized zone has the
wrong size; when a card definition is unknown; when executable state lacks a
stable catalog locator; or when the rebuilt legal actions or public observation
differ. Unknown additive object members are ignored. It never quietly creates
an approximate game. Internally, the engine creates one typed `GameSnapshot`
and serializes it at the protocol boundary; reconstruction deserializes that
same open-world schema before building a `Game`. Catalog-owned executable data
is represented by semantic locators, while hidden-zone identities are supplied
only by the separate hypothesis above.

A private pending decision is reconstructible only from its choosing seat's
observation. Other seats receive neither the decision nor its continuation in
their checkpoint; `hasDeferredState` is true, so importing that checkpoint
fails closed instead of exposing private candidates or effect-local bindings.
The first successful draw of each turn always takes the same private
draw-action path. Its empty selection means "take no draw action"; only the
drawing seat receives the candidate payload that says whether Reveal is also
available. The engine allocates and resolves an actionless ordinary window
inside the atomic draw, so it adds no UI or network pause. A declined Miracle
settles to the same decision counter, projected events, and opponent
checkpoint. Direct callers that inspect `Game::observe` during an unresolved
private choice can still distinguish that an action is pending; the raw engine
API does not promise arbitrary-intermediate-state indistinguishability. Hosted
rooms instead serve the opponent their last complete safe state until the
choice settles. They do not add a cover delay, however: an interactive Miracle
choice can take longer than an ordinary draw, so elapsed response time is not
a privacy claim.
An installed, pending, or stacked trigger likewise fails closed when its source,
retained lexical targets, or bindings name a card in a hidden zone that has no
stable public object ID; the checkpoint omits that executable state rather than
serializing a host-only identity. In particular, the non-owner's checkpoint is
deferred while a revealed Miracle card in hand is the source of its *pending*
linked trigger.

Once that trigger is on the stack the observation names the object itself, in
`stack[].sourceObjectId`, so the checkpoint says where the card sits rather
than dropping the payload: `stack[].abilityPayload.sourceOrigin` carries the
seat, hidden zone, and index, and the importer binds its minted card at that
position to the published id. It is the same disclosure the public
cast-or-decline decision one step later already makes through its
`decisionState.cardOrigins`, and it reconstructs for either seat. The member
is optional and absent whenever a source is already public, so a consumer that
does not read it is unaffected.

Checkpoint format 7 covers every ordinary action boundary emitted by the
hosted formats, subject to the explicit fail-closed cases above: pregame and
turn/combat progression; complete permanent,
emblem, stack, and combat state; restricted/source-specific mana; copied and
temporarily modified characteristics; retired-object last-known information;
pending battlefield-entry replacement programs; installed and pending
triggers; ordered resolved damage-prevention, characteristic, object-rule, and
player play-restriction effects; ordered inserted-turn-phase queues and their
frozen ordinary continuation; and every pending decision
continuation emitted by the hosted formats, including prospective begin-turn
replacement choices. Stack payloads retain target-slot groupings, divided
amounts, modes, X, complete lexical resolution context, flashback/copy state,
text and color changes, and mana-carried effects.
Card-owned pile callbacks use stable registry keys rather than serialized
function pointers. Public object IDs remain unchanged, including those needed
by suspended continuations, while hypothesized private cards are rebound to
fresh local IDs.

The off-format Ugin's Nexus interaction can suspend a battlefield-exit
replacement-order choice. That continuation currently reports
`hasDeferredState: true`, and reconstruction fails closed until the prospective
exit batch and the work that follows it have a stable typed encoding.

A rebuilt world is not just correct at the instant you build it; it stays in
step as you play it. Feed the reconstruction and the host game the same
`legalActions` indices and they remain the same game — same public state, same
legal actions — which is what makes a rollout worth anything. Two things
legitimately end that correspondence, and neither is a bug:

- **Local randomness.** `rollout_seed` is deliberately not the host's seed, so
  the first shuffle, or the first effect that discards at random, moves your
  world's hidden zones somewhere the host's did not go.
- **A wrong guess.** Everything after a hypothesized card is revealed is a
  sample from the world you guessed, not a prediction of the real one.

Both are the honest limits of determinized search rather than defects: run N
worlds and act on the consensus. Up to that point the reconstruction is exact,
and the engine's test suite holds it to that standard — it walks reconstructed
and host games forward side by side and requires them to agree action for
action until local randomness parts them.

`set_hand` and `set_library` remain useful when exploring alternate hidden
zones in a game already running locally. `hand(seat)` and `library(seat)` read
those local zones back unredacted. They are simulation helpers, not a hosted
match API; a remote bot receives only redacted observations, and
`from_observation` — not a sequence of setters — is how it turns one into a
world it can search.

## The observation

`observe()` returns one JSON object. The essential fields:

| field | meaning |
| --- | --- |
| `protocolVersion` | the breaking bot-wire epoch; protocol 29 objects are open-world, but an epoch mismatch requires migration |
| `protocolCapabilities` | optional named facilities emitted by this engine; currently includes `reconstruction.checkpoint.v10`; ignore unknown entries |
| `simulationFingerprint` | a conservative identity of simulation source and build requirements; pin it for training and require it for reconstruction |
| `engineVersion` | package-release provenance; it is not an exact simulation identity |
| `format` | the rules/deck profile slug: `"old-school-93-94"`, `"premodern"`, `"isd-m14-standard"`, `"som-m13-standard"`, `"vintage-cube"`, or `"pauper-cube"` |
| `seat` | whose view this is: `"p1"` or `"p2"` |
| `pregame` | true while mulligans are being settled |
| `turn`, `activeTurn`, `activeSeat`, `prioritySeat`, `step` | where the game is; `activeTurn` counts turns started by the active player, including extra turns, and `step` is one of `Upkeep`, `Draw`, `PrecombatMain`, `BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, `EndOfCombat`, `PostcombatMain`, `End`, `Cleanup` |
| `regularCombatDamagePending` | true during the priority window after first-strike damage and before regular combat damage; both damage waves otherwise use `step: "CombatDamage"` |
| `life`, `poison`, `energy`, `manaPools`, `librarySizes` | two-element arrays, indexed p1 then p2. Ten or more `poison` is a loss; `energy` is a resource and no amount of it wins or loses anything |
| `playerCounters` | two sparse arrays, indexed p1 then p2, of `{name, count}` entries for every counter each player carries. `poison` and `energy` remain compatibility projections |
| `monarch` | who holds the crown (CR 720) as `"p1"` or `"p2"`, or null while nobody does. The monarch draws a card at the beginning of their end step, and a creature that deals combat damage to them hands the crown to its controller |
| `hand` | your cards: `{objectId, instance, definition, name}`; `instance` is a compatibility alias for `objectId` |
| `opponentHandSize` | their current hidden hand as a count; learned snapshots are reported separately in `lastSeenHand` |
| `revealedLibraryTop` | null unless something lets you look at the top card of your own library, such as Bolas's Citadel; a one-card list in the same shape as `hand` when it does |
| `opponentRevealedLibraryTop` | null unless your opponent is playing with the top card of their library revealed, such as under Courser of Kruphix; a one-card list in the same shape as `hand` when they are. Their own view of that card arrives in `revealedLibraryTop`, which reports whichever library belongs to the viewer |
| `lastSeenHand` | null or the most recently revealed hand snapshot as `{seat, cards}`; it records known information and can outlive later hand changes |
| `battlefield` | every permanent, including its current-zone object ID, authoritative tagged `characteristics`, and sparse `{name, count}` `counters`; catalog-backed cards retain definition/part IDs, while tokens and face-down objects carry their display characteristics inline. `token` records token status independently of copied values, and `hasIndividualState` tells compact presentation clients not to collapse an attachment or otherwise object-specific affected permanent with a lookalike. A physical double-faced permanent also reports `physicalFace`; a planeswalker reports `loyalty` and `loyaltyAbilityUsedThisTurn` |
| `checkpoint` | the hidden-safe typed rules snapshot used by `Game.from_observation`, including its independent `version` and `simulationFingerprint`, deferred execution, dynamic objects, exact mana units, and reachable LKI; it never contains host RNG state or hidden-zone card identities |
| `emblems` | command-zone emblems, each with its controller, creator-owned name and clause texts, and the granting ability |
| `stack` | pending spells, activated abilities, and triggered abilities, bottom to top; entries expose the source object ID, tagged characteristics and ability origin/text, controller, counterability, targets, chosen permanents, X, and a locked cast signature when applicable |
| `graveyards`, `exiles` | public zones, both players; a card lying face down in exile is absent from the nonowner's view rather than shown, and counted by `faceDownExileSizes` instead |
| `faceDownExileSizes` | how many cards lie face down in each player's exile, the way `opponentHandSize` counts a hand; only their owner knows what they are |
| `cardCounters` | sparse counter state for visible cards outside the battlefield, as `{objectId, counters: [{name, count}]}` entries; suspend time counters are the common case |
| `decision` | a pending choice (see below), or null |
| `result` | null while running, else `{winner, reason}`; `reason` is `OpponentConceded`, `OpponentLostAllLife`, `OpponentTriedToDrawFromEmptyLibrary`, `OpponentLostToAnEffect`, `OpponentRanOutOfTime`, or `OpponentPoisoned` |
| `legalActions` | what you can do, each with an `index` |

Every protocol-27 JSON object is open-world: ignore members you do not use
rather than rejecting the whole observation or catalog. Treat documented
presentation strings as opaque. Where a string vocabulary has a safe fallback,
use it: for example, an unknown non-null `result.reason` still means the game
ended even if the client cannot give that ending a more specific label. An
unknown capability is harmless unless the host explicitly lists it as required.

An attempted draw from an empty library does not end resolution immediately.
The engine records it until the next state-based action check, when it settles
that condition together with both players' life totals; if both remaining
players lose in that check, `result` is a draw. When one effect instructs both
players to draw, the active player completes all of their individual draws
before the nonactive player begins theirs. This is visible in resulting hand
sizes and game events but adds no observation field or legal-action shape.

Objects are referenced two ways. The object ID identifies one rules object in
its current zone. Its `characteristics` object says how to present that object:

- `{kind: "printed", definition, partId}` joins a printed card and current
  part through `penta.catalog(format)`. The legacy top-level `definition` and
  `presentedPartId` projections remain on printed battlefield, stack, and
  decision objects.
- `{kind: "token", partId, name, structure, presentation, art?}` is complete
  inline presentation data. `structure` is tagged `single` or
  `transformingDoubleFaced`; `presentation` carries the current part's kind,
  type line, mana cost, power/toughness, rules text, colors, land status, and
  implementation status. `art` is present only when the creating card selected
  a token printing. A token object never carries a fake `definition` or
  `presentedPartId`.
- `{kind: "emblem", name, presentation}` is the complete inline identity of
  an emblem ability's source. Its `presentation` carries the emblem's rules
  text and uses the literal `kind` and `typeLine` value `Emblem` so existing
  card-shaped renderers can label it. That label is display data, not a card
  type or `CardRules` identity: emblems have no card definition, card part, or
  art.
- `{kind: "faceDown", name, presentation}` is the complete copiable value an
  opponent sees when a rule or effect turned the physical card face down. It
  has no `definition` or `partId`; the underlying physical card retains its
  real catalog identity, which its entitled controller can still inspect.
  Morph, Manifest, and
  Illusionary Mask use an ordinary nameless 2/2 creature presentation, while
  Disguise and Cloak add ward {2}. Card-specific mechanisms may supply another
  inline presentation. As with token characteristics, this tag names the
  source of copiable values rather than physical status: a face-up copy can
  retain `kind: "faceDown"`; read the permanent's separate `faceDown` boolean
  to know whether it is actually face down.

Native declarative definitions mirror those wire identities. Common token
effects use compact by-value builders: `EffectDef::create_creature_token`,
`create_artifact_creature_token`, and `create_artifact_token`, followed as
needed by `with_amount` or `with_count`, `with_name`, `with_abilities`, and
`with_art`. The default name joins the supplied subtypes in order. Standardized
artifact rules are functions such as `tokens::treasure`, `food`, `clue`,
`blood`, `map`, and `incubator`, rather than globally named token constants.
`EffectDef::create_emblem` similarly embeds a compact name-and-ability value in
the creating card's effect. Face-down rules use compact
`FaceDownCharacteristics` values from `face_down::morph`, `manifest`,
`illusionary_mask`, `disguise`, or `cloak`, or construct a card-specific value.
None of these APIs allocates a card definition.

The permanent's separate `token` boolean records whether the rules object is a
token. Do not infer that status from `characteristics.kind`: a token can copy a
printed card, and a nontoken permanent can copy a token. A face-up physical
double-faced permanent also carries
`physicalFace: {kind: "transforming"|"modal", side: "front"|"back"}`. This is
physical topology and orientation, not copied characteristics: it is absent
when a single-faced permanent copies a double-faced object and remains present
when a double-faced permanent copies a single-faced object. It is omitted for
physical single-faced and face-down permanents. Never infer transformability or
physical orientation from `characteristics.structure`. Treat an unknown
`physicalFace.kind` or `.side` as unavailable topology and fall back to the
authoritative legal actions rather than guessing that the permanent can
transform.

Public choices remembered by a permanent use optional fields such as
`chosenCardName`, `chosenCreatureType`, `chosenBasicLandType`, and
`chosenColor`. `chosenColor` is a lower-case mana-color name and is absent or
null when that permanent made no color choice. These fields belong to the
permanent incarnation and therefore disappear when it changes zones.

A true zone change
creates a new object ID, so a Goblin Balloon Brigade card in hand, its spell on
the stack, and its permanent on the battlefield are distinct. Transforming,
flipping, and phasing do not create a new object. Exact physical-card backing
identity remains private engine state; only the public double-faced topology
above appears in a player's observation. Fetch the
format's catalog once at startup for printed characteristics; token and emblem
characteristics need no catalog lookup.

### Actions

Every entry in `legalActions` has an `index` (what you pass to `act`) and a
`type` naming the engine action, plus fields saying what it operates on:

`KeepHand`, `TakeMulligan`, `BottomCards`, `PlayLand` (with `card` and
`playOptionId`), `CastSpell` (with the play option, ordered modes, cost
configuration, target slots, sacrifices, and X already filled in — one entry
per legal casting choice), `ActivateAbility`, `ActivateManaAbility`, `PayLifeForMana`,
`DeclareAttacker`, `BandAttackers` (with `first` and `second`),
`FinishDeclaringAttackers`, `DeclareBlocker`,
`FinishDeclaringBlockers`, `AssignCombatDamage`, `DiscardCards`,
`ChooseUntap`, `ChooseDecision`, `CancelDecision`, `PassPriority`,
`TurnFaceUp` (with `permanent`), `Foretell` (with `card`), `UnlockDoor` (with
`room` and the `door` part id), `ExertAttacker` (with `attacker`).

The `type` vocabulary is **open**, and the safe fallback is the one every bot
already uses: choose by `index`. A bot that does not recognize a `type` may
simply not choose that entry, so a new action kind is additive and does not
move the epoch. Never key required behaviour on the absence of a type you have
not seen before.

An index belongs only to the observation that contains it. Acting changes the
state and rebuilds `legalActions`, so re-observe before choosing again; do not
cache an index across actions even when the visible `type` is the same.

Wherever an action names a card — `card`, `sacrifices`, `attacker`, `blocker`,
`costObjects` — it does so by **object ID**, never by `definition`. To ask what
a cast is actually casting, go through the observation's own zone listing:

```python
hand = {card["objectId"]: card["definition"] for card in observation["hand"]}
card = CATALOG[hand[action["card"]]]
```

Reading the catalog with an object ID directly appears to work, because both
are small integers and the lookup usually finds *something*. It is simply a
different card, with no error to notice.

For `CastSpell`, `card` and `sacrifices` are top-level. The canonical nested
`choices` object contains `playOptionId`, ordered `modeIds`, nullable
`alternativeCostId`, `additionalCostIds`, `x`, and `targetSelections`. It also
contains `manaPayment` when a flexible symbol uses an explicitly announced
alternative. Each entry names the printed `symbol`, selected `count`, and
`payWith` (`life` for Phyrexian mana or `generic` for two-brid). Mana-paid
copies are omitted, and this payment choice is not copied into the spell's
stack signature.
Top-level `playOptionId`, `modeIds`, flattened `targets`, and `x` remain
compatibility projections; new clients should read `choices`. Each target
selection has `slotId`, ordered `targets`, and an `amounts` array. `amounts` is
empty for an ordinary slot; for a divided slot it is parallel to `targets` and
records the share assigned to each target. A spell stack entry preserves the
same data in `signature.targetSelections`.

Ability actions identify the exact clause being used in an `ability` object.
Its `kind` determines the rest of its provenance:

- `printed` carries the canonical `definition`, positional `partId`, and
  positional `abilityId`.
- `token` carries positional `partId` and `abilityId`; join it to the action's
  source object for the inline characteristics rather than looking up a
  definition.
- `emblem` carries positional `abilityId`; join it to the action or stack
  object's source for its inline emblem characteristics.
- `faceDown` carries positional `abilityId`; join it to the action or stack
  object's source for its inline face-down characteristics.
- `intrinsicBasicLand` carries the lowercase `landType` whose rules supplied
  the ability.
- `intrinsicCounter` carries the `counter` name whose keyword-counter rules
  supplied the ability.
- `granted` carries the granting `source` object together with
  `sourceDefinition`, `sourcePartId`, `sourceAbilityId`, and `grantId`.
- `tokenGranted` carries the granting `source` object together with
  `sourcePartId`, `sourceAbilityId`, and `grantId`; it deliberately has no
  `sourceDefinition`.
- `emblemGranted` carries the granting `source` object together with
  `sourceAbilityId` and `grantId`; it deliberately has neither a
  `sourceDefinition` nor `sourcePartId`.
- `faceDownGranted` carries the granting `source` object together with
  `sourceAbilityId` and `grantId`; it deliberately has neither a
  `sourceDefinition` nor `sourcePartId`.

`ActivateAbility` carries `source`, `ability`, `x`, an array `costObjects`,
canonical `targetSelections`, flattened `targets`, and a compatibility
`target` containing the first selected target. `costObjects` lists the objects
chosen to pay an object-naming cost, such as the permanent a sacrifice takes
or the cards an exile cost lifts from a graveyard. Most costs name one object
or none; a cost that spends several, as Grim Lavamancer's two graveyard cards
do, lists them all, and two activations of one ability can differ only in
which objects they name. Targets and that object are chosen before the
activated ability becomes an independent stack object. `ActivateManaAbility`
uses the same source and origin vocabulary, adds the selected `color`, and
resolves immediately because mana abilities never use the stack. It also
carries an optional `countersRemoved` when, and only when, the ability's cost
removes an open-ended number of counters: source, origin, and colour name the
storage lands' ability once per size it could be paid at, so that number is
what tells the offers apart. Every other mana ability omits the key. It
likewise carries an optional `combination` when, and only when, the ability
adds mana "in any combination of" more than one type: an object mapping colour
name to count, listing only the types that division actually produces. Source,
origin, and colour name such an ability once per division, and `color` is the
first type the division produces, so a bot that reads only `color` still sees
a colour it will receive. Every other mana ability omits the key. The engine
does not infer either classification merely because an effect happens to
produce mana. Finally, `triggeredMana` is present when immediate mana triggers
such as Mana Flare or Dawn's Reflection make output choices. It is the
aggregate colour-to-count compatibility projection. When more than one effect
chooses independently, `triggeredManaChoices` is also present as an ordered
array of those objects, preserving each effect's domain and source. Two Mana
Flares watching an activation that produces white and blue therefore expose
four actions: white/white, white/blue, blue/white, and blue/blue; the two mixed
actions have the same aggregate `triggeredMana` but different
`triggeredManaChoices`. An activation that causes no such choice omits both
keys. These optional open-world members do not move protocol 29.

Targets are tagged objects: a player is `{type: "player", seat}`, a card or
permanent has an `objectId`, and a spell has its stack `objectId`. Legacy
`instance` and `stackId` aliases remain where the current serializer emits
them. Runtime `slotId` values are positional within the instantiated spell or
ability, starting at zero. A cast flattens the base option's targets and each
selected mode occurrence into consecutive slots, so repeated modes receive
distinct ranges. Follow the action's canonical `targetSelections`; do not
assume a mode-local catalog slot keeps the same runtime ID. Flattened
compatibility `targets` discard slot identity and divided amounts.

Three things worth knowing:

- **Nothing in the list loses on the spot.** Conceding is legal in every
  state of Magic, but it is strictly dominated for a bot — resigning can
  only lose a game that playing on might win — so it is not offered here at
  all. Picking blindly, by index or at random, makes a weak bot rather than
  an instant loss. (Humans concede through the browser client, which reads
  the engine's own action list.)

- **Mana is handled for you.** If a `CastSpell` appears in `legalActions`,
  you can afford it; playing it taps lands automatically. Tapping lands by
  hand (`ActivateManaAbility`) exists but is never required.
- **Costs and targets are enumerated.** A Lightning Bolt with three legal
  targets appears as three `CastSpell` entries. Your bot chooses among
  ready-made legal plays; it never constructs one.

Every `DeclareAttacker` names a `defender`: `{"type": "player", "seat": "p2"}`
or `{"type": "planeswalker", "objectId": 42}`. The engine offers one action per
legal defender, so a creature that could attack either a player or a
planeswalker appears more than once. Combat damage follows the defender, and a
planeswalker takes it as loyalty counters.

Attacker and blocker declaration is incremental: choose one
`DeclareAttacker` or `DeclareBlocker`, re-observe, and eventually choose the
matching `FinishDeclaring...` action. Banding is declared the same way, a pair
at a time: `BandAttackers` puts `first`, `second`, and everything already
banded with either of them into one band, and the engine offers only the pairs
the printed limit allows. A permanent's `attackingBand` is the index every
member of its band shares, absent for an attacker in no band. `AssignCombatDamage` names one `attacker`
and an ordered `assignments` list of `{recipient, amount}`.

A permanent's `blocking` lists the attackers it is blocking right now, and
combat damage follows that list. It empties as those attackers leave combat,
which does not stop the creature being a blocking creature — read
`blockingThisCombat` for the status, as you read `blockedThisCombat` rather
than counting an attacker's surviving blockers.

### Decisions

Every decision has a `kind`:

- `Choice` asks an ordinary question during costs or resolution — "copy Chain
  Lightning?", "choose a card to return", and so on.
- `TriggerOrder` asks a player to order simultaneous triggers. Each option has
  a `triggerId` and frozen `abilityText`; `orderSemantics: "resolution"` means
  the submitted list is first-resolving-first, even though the stack itself is
  displayed bottom-to-top.
- `TriggerPlacement` asks for targets while one triggered ability is being put
  on the stack. Every player orders and targets their own triggers, in
  active-player/nonactive-player placement order, before priority returns.

These arrive as a `decision` object with `id`, chooser `seat`, `prompt`,
`visibility` (`Public` or `Private`), `minimum`/`maximum` counts,
`cancellable`, and `options`. When a resolving permanent ability created the
choice, additive `sourceObjectId` names that battlefield object even though
the ability has already left the stack. Each option has its own `id`, `label`, nullable
card and `abilityText`, and a `zone`. A card-backed option uses the same tagged
`characteristics` shape as battlefield and stack objects, so an offered token
needs no catalog definition. They also arrive as `ChooseDecision`
entries in `legalActions`: a pick-exactly-one decision becomes one indexed
action per option, so an index-only bot handles it like anything else. For a
pick-several decision, `legalActions` carries one default selection (the first
`minimum` options) and `choose_decision([option_ids])` submits any other
selection you'd prefer. Submit option IDs, not option-array offsets, within
the reported bounds. When `cancellable` is true, `CancelDecision` is a
distinct legal action; cancelling is not the same as choosing zero options.
`OutsideGame` is a provenance value used for a privately offered sideboard
card; it is not a Magic zone and sideboards are not added to observations.

For most decisions the submitted list is a set: which options you picked
matters and the order you list them does not. One shape reads the order too.
An arrangement -- "look at the top three cards, then put them back in any
order" -- offers every inspected card, requires all of them, and places them
in the sequence you submit, first ID on top. The decision is otherwise an
ordinary pick-several: a bot that ignores the ordering still submits a legal
answer, and gets an arrangement it did not choose rather than an error. The
prompt says so; nothing else in the decision distinguishes it, so a bot that
wants to arrange deliberately should look for a decision whose `minimum` and
`maximum` both equal the number of options.

Decision prompts and option labels are presentation text, not stable protocol
identifiers. They can become more precise when a rules procedure moves to a
shared implementation without changing the decision shape or option IDs. Read
them for display, but submit the IDs from the current observation and use the
structured decision fields and legal actions for control flow.

Begin-turn replacements are offered immediately before a proposed turn begins.
While that decision is pending, the observation still reports the previous
turn's active player and `Cleanup` step; the decision's `seat` is the player
whose turn would begin, so it need not match the active seat. Option ID `0`
appears only when every applicable replacement is optional; choosing it
declines them and begins the proposed turn. Each card-backed option applies
that source's own replacement. Time Vault skips the turn; its untap is deferred
until it is the first thing that happens in the next turn that actually begins.
Ugin's Nexus skips only an extra turn. Multiple applicable sources
appear together because only one can replace a given prospective turn. Read
the current options and card identities rather than assuming a fixed option
count or effect.

### Catalog and mana costs

`penta.catalog(format)` carries the same `protocolVersion`,
`protocolCapabilities`, `simulationFingerprint`, `engineVersion`, and `format`
as observations, plus `formatName` and the canonical `cards` array. The array is
ordered by `definition` and is not filtered: it contains printed cards outside
the selected format. Created-token, emblem, and face-down characteristics are
not card definitions and do not appear here; visible virtual-object
characteristics travel inline.
Their former synthetic definition IDs remain retired and are never reused, so
the ordered `cards` array may contain gaps. Join a card through its explicit
`definition`, never by treating that ID as an array index.
That includes off-format rules test cases such as Darksteel Ingot (definition
`263`, debut set `darksteel`), Enlightened Tutor (`313`, `mirage`), and the five
Onslaught fetch lands (definitions `283`, `284`, and `1363` through `1365`,
`onslaught`). Their abilities are
executable even though the cards are not legal in either shipped format.
`allowed` means the definition belongs to the format's card pool; `legal` is
`allowed && !banned`, so a restricted card is still legal. Definitions include
their structure, parts, play options, legality, printings, and clause-derived
implementation status.

Catalog contents may grow compatibly within one protocol version because new
opaque definition IDs can be added while existing card identities never move.
Retired IDs stay empty rather than being reassigned. Definition IDs are
positive integers no greater than `2^52 - 1`, so JSON and JavaScript represent
them exactly; the values are sparse and must never be treated as array indexes.
Zero is invalid, and absence or null—not any numeric value—means hidden,
missing, or redacted. The
out-of-format interaction fixtures
`Urborg, Tomb of Yawgmoth` (definition 261, debut set `planar-chaos`) and
`Yavimaya, Cradle of Growth` (definition 262, debut set
`modern-horizons-2`) appear in every unfiltered catalog but have `allowed` and
`legal` set to `false` for both supported formats. They therefore add no legal
actions to an ordinary supported-format game.

The protocol-18 catalog also appends the Premodern library-selection cards
`Impulse` (definition 310), `Sleight of Hand` (definition 311), and `Opt`
(definition 312). `Impulse` introduces the `visions` debut-set slug. These
definitions remain off-format in the currently shipped format profiles, so
the additive catalog entries do not add legal actions to those games.
`Enlightened Tutor` (definition 313) and `Worldly Tutor` (definition 314)
follow as off-format catalog additions; a successful search publicly reveals
the selected card, shuffles the remaining library, and puts that card on top.

The protocol-19 catalog appended definitions 315 through 605 for the expanded
Old School implementation: 286 legal card identities and five supporting
tokens. At that transition, the format had 421 cataloged legal identities, of
which 389 were `complete`, 30 were `partial`, and two were `metadataOnly`. The
identity-complete audit was kept inline at each identity's collector position in
the printed set modules. It recorded the concrete engine limitation for those
32 incomplete definitions and all 560 other legal identities that were
blocked, and also covered all seven banned identities in the same sets. This is
an additive catalog-content change, not a JSON-shape change; consumers must not
assume the older catalog length or that definition 314 is the maximum ID.

Definitions 607 through 1361 extended the compatible protocol-20 catalog growth
with 736 card identities used by the historical Standard tranche and nineteen
supporting tokens. Together with in-format printings of existing definitions,
Standard then exposed 878 legal identities: 839 `complete` and 39 `partial`.
The inline audit partitioned all 1,686 identities in the eight-set ISD–M14
window and recorded a concrete engine-capability gap for every one of the 847
incomplete identities, including 808 blocked cards that had no catalog
definition. The definition IDs are
append-only and the catalog,
observation, action, and decision JSON shapes are unchanged, so this does not
bump protocol 20. Consumers must not assume definition 606 remains the maximum
ID.

Cards and parts expose `implementationStatus` as `complete`, `partial`, or
`metadataOnly`; the old execution gate is not public coverage metadata.
Definition, part, play-option, mode, and cost IDs join directly. Target-slot
IDs are positional within the list that declares them; concrete casts flatten
base targets and each selected mode occurrence into new consecutive runtime
slot IDs. A card with no printed mana cost has `"manaCost": null`; a printed
`{0}` has a mana-cost object whose `generic` field is zero.

Protocol 16 makes every Boros Charm mode executable. Mode 0 legal actions
target a player or planeswalker, mode 1 has no targets and grants the caster's
current permanents Indestructible until cleanup, and mode 2 targets a creature.
The catalog's simplified projection presents the first mode as `AnyTarget`, but
its concrete legal actions never offer creatures; those actions are the
authority for the semantic restriction.

Protocol 20 changes Chaos Orb from a targeted activation-time approximation to
Eternal Central's non-targeting resolution-time choice. Its Old School action
space now offers one `ActivateAbility` with an empty `targets` list instead of
one action for each permanent. Once that ability resolves with more than one
eligible permanent, its controller sees a public `Choice` decision containing
the nontoken permanents then on the battlefield. Zero- and one-candidate
choices complete automatically. The choice ignores hexproof, shroud, and
protection and is not recorded as a stack target or `chosenPermanents` entry.
Answer an offered choice with the existing `ChooseDecision` action. A seeded
`0.9` random trial decides whether the chosen permanent is destroyed. Guardian
Beast (definition `606`) is also added compatibly to the unfiltered catalog and
is legal only where Arabian Nights is allowed.

As compatible protocol-22 simulation growth, definitions `1362` through `1367`
append Ring of Ma'rûf, the three remaining Onslaught fetch lands, Liliana's
Shade, and Seek the Horizon, moving the latter two from Standard's inline audit
into the executable catalog. Ring is executable in Old School. Its activation
can create a private choice whose option `zone` is `OutsideGame`; under the
Eternal Central profile the same choice can also contain cards from `Exile`.
Clients must treat the option list as authoritative and must not infer that
every card-backed decision refers to an observed zone. If more than one next-draw
replacement is applicable, the affected player gets a public one-option
`Choice`; the unchosen effects remain available for later draws that turn.

The same compatible simulation update corrects Demonic Tutor's unrestricted
library search: with a nonempty library its `Choice` has `minimum: 1` and
`maximum: 1`, so an empty `ChooseDecision` selection is no longer legal.
Qualified hidden-zone searches can still expose `minimum: 0` and be resolved
without selecting a card. These cards and choices use the existing catalog,
`Choice`, and `ChooseDecision` shapes, so they change the simulation fingerprint
rather than the bot-wire epoch.

`Ugin's Nexus` (definition `1368`, debut set `khans-of-tarkir`) follows as an
off-format interaction fixture and remains illegal in both shipped formats.

A play option's `restriction` is `normal`, `fromHandOnly`,
`beforeCombatDamage`, `beforeBlockersDeclared`, `opponentsUpkeep`,
`declareAttackersStep`, or `opponentsTurnAfterUpkeep`. Read the tag rather than
assuming every otherwise valid option is available from any zone or at every
casting window. The set is open: treat a tag you do not recognize as a window
the engine will enforce for you, and keep choosing from the legal actions it
offers rather than predicting them.

A play option's `modes` object carries `minimum`, `maximum`, `mayRepeat`, and
`choices`. It also carries an optional `conditionalMaximum` when, and only
when, the spell prints a clause that raises its maximum under a condition
read as it is cast -- "if you control a Wizard as you cast this spell, you may
choose two instead". The condition itself is not on the wire: the legal
actions already enumerate only the mode selections available right now, so
that larger maximum tells a bot what the card can do rather than what it may
do this turn. Every other modal spell omits the key.

Each entry in `modes.choices` may carry an optional `additionalManaCost` with
the ordinary mana-cost object shape. It is present when choosing that mode
itself requires the listed additional mana, as with Spree. Add the costs of
all selected modes to the spell's base or alternative cost; concrete legal
actions and the engine's payment validation remain authoritative after cost
increases and reductions.

Catalog target labels and simplified target projections are presentation data,
not identity or a complete rules predicate. A richer semantic target can lack
that legacy projection while still producing targeted legal actions; use the
concrete action as the authority.

As a compatible protocol-15 catalog enrichment, Doom Blade, Swords to
Plowshares, Divine Offering, Dispel, Dissipate, Putrefy, Ultimate Price, and
Warleader's Helix now expose the one target slot derived from their declarative
spell definitions. Their legacy definitions left `playOptions[].targets`
empty even though concrete cast actions already carried the target. The legal
action and target-slot encodings are unchanged, and clients should continue to
use each supplied concrete action as the authority. The shared target
machinery now evaluates effective characteristics consistently and rechecks
legality on resolution; continuous effects can therefore change which casts
are offered, and a spell whose targets are all illegal correctly fizzles.

Since protocol 8, every non-null mana-cost object has this shape:

```json
{
  "generic": 0,
  "white": 0,
  "blue": 0,
  "black": 0,
  "red": 0,
  "green": 0,
  "colorless": 0,
  "hybrid": [{"symbol": "R/W", "count": 3}],
  "variableX": false,
  "xMultiplier": 1
}
```

`hybrid` is sparse: it contains one entry for each nonzero flexible symbol,
and `count` says how many copies occur. Besides ordinary two-color hybrid such
as `R/W`, protocol 28 permits two-brid (`2/B`), Phyrexian (`R/P`), Phyrexian
hybrid (`G/U/P`), and colorless hybrid (`C/W`). Treat this symbol vocabulary
as open and display an unknown string as supplied. Protocol 8 replaced
protocol 7's one-off numeric `whiteRedHybrid` field with this general array.
The shape is used everywhere the catalog reports a cost, including parts,
play options, alternative costs, and additional costs.

### Migrating from protocol 28

Protocol 29 removes the closed `isd-dgm-standard` format value and its
`isd-rtr-standard` compatibility spellings. Use `isd-m14-standard` for the
final pre-Theros profile and rewrite stored game configurations before starting
them. Catalog and observation JSON shapes are unchanged. Browser replay version
2 is unchanged because replay acceptance also requires the exact simulation
fingerprint, which changes with the supported format registry.

### Migrating from protocol 22

Protocol 23 renames the Innistrad-through-Dragon's-Maze format's canonical wire
identifier from `isd-rtr-standard` to `isd-dgm-standard`. Protocol 29 removes
that historical profile and its compatibility spellings; migrate directly to
`isd-m14-standard`. Browser replay journals carrying the protocol-23 rename use
replay version 2. Checkpoint format 3 and `reconstruction.checkpoint.v3` are
unchanged.

The accompanying cards and built-in decks are append-only catalog and
simulation changes. They do not move the wire epoch independently and are
identified by the generated `simulationFingerprint`.

### Migrating checkpoint format 1 to 2

This rules change does not move protocol 22. Checkpoint format 2 replaces the
per-player `skippedTurns` debt with the zero-based `nextRegularPlayer` seat,
which preserves ordinary-turn order around the newest-first extra-turn queue.
It also gives pending prospective begin-turn replacements a typed continuation,
so a Time Vault choice can reconstruct before the proposed turn commits.
Consumers of reconstruction should require
`reconstruction.checkpoint.v2` for that historical format and continue checking
both the nested checkpoint version and exact simulation fingerprint. A pending
battlefield-exit replacement-order choice remains explicitly deferred and fails
reconstruction closed as described above.

### Migrating checkpoint format 2 to 3

Protocol 22 remains in place. Checkpoint format 3 separates event-only trigger
context from the typed single-object and object-set bindings accumulated while
a declarative effect resolves. Suspended choices, mana-or-life payments, and
pile procedures retain those bindings through shared typed continuations.
Battlefield-entry scalar choices use the same destination-tagged continuation,
and private top-card selections retain whether the chosen card must be
revealed. Each disclosed hidden-zone card records its exact seat, zone, and
index, preserving public object identity, duplicate definitions, and visible
option order under a hidden-zone hypothesis.

Resolved characteristic and object-rule modifications are one ordered,
expiration-aware continuous-effect collection. Every entry retains its exact
authored catalog location, source-ability provenance, timestamp and component
order, and frozen resolution-time values; resolved player play restrictions
use a parallel ordered collection. These replace the separate animation,
aggregate power/toughness, granted-ability, removed-ability, unblockable,
can't-block, and can't-regenerate fields.

Resolved damage prevention is likewise one ordered typed collection, replacing
the separate Fog flag, prevention shields, relational rules, and per-permanent
combat-prevention flags while retaining concrete matchers, remaining point or
event capacity, coverage, life-gain recipient, provenance, timestamp, and
expiration. Effect-scheduled instructions and floating listeners are unified
as installed triggers with their lexical targets and bindings; pending and
stacked triggers carry the same complete resolution context. Entry-replacement
continuations use typed replacement-program locators, including nested
branches.

The scalar additional-combat counter is now an ordered queue of inserted major
phases with a frozen ordinary continuation. Authored `combat, postcombat main`
sequences stay intact, later and nested schedules retain their ordering, and
the continuation distinguishes resuming ordinary combat from proceeding to the
end step. Legacy card-, payment-, destruction-, and pile-specific continuation
tags are replaced by the shared choice, PayOr, and partition procedures.

Format-2 checkpoints do not contain the individual ordering, provenance,
duration, trigger identity, lexical context, phase sequence, displaced
continuation, payment kind, reveal instruction, or exact hidden-zone positions
needed to recover this state. They cannot be upgraded by guessing. Consumers
targeting this historical format had to require `reconstruction.checkpoint.v3`;
current consumers should follow the format-4 and format-5 migrations below.

### Migrating checkpoint format 3 to 4

Protocol 25 remains in place. Checkpoint format 4 adds the closed `subtypes`
operation to resolved continuous-effect snapshots, so an effect that adds,
removes, or sets a named subtype can be reconstructed without flattening the
permanent's characteristics. Format-3 importers cannot interpret that operation
and must regenerate the checkpoint with the current engine. Reconstruction
consumers of format 4 had to require `reconstruction.checkpoint.v4`; current
consumers should follow the format-5 and format-6 migrations below. Continue
checking the exact simulation fingerprint in every case.

### Migrating checkpoint format 4 to 5

Protocol 26 removes synthetic token and emblem card definitions. Checkpoint format 5
therefore tags every frozen object presentation as either a catalog-backed card
or a semantic locator for creator-owned token or emblem characteristics. The
locator identifies a card-, token-, or emblem-owned creating ability and its
nested effect path or indexed custom-created virtual object. A virtual-object
chain is recursively rooted in a printed/custom card creator; restore rebinds
that source through the exact catalog and simulation fingerprint to recover
the token's selected art, complete rules, face structure, and current part, or
the emblem's name and complete rules. Ability origins likewise distinguish
printed, token, emblem, printed-granted, token-granted, and emblem-granted
provenance. This is executable state rather than optional presentation data: a
format-4 checkpoint's synthetic definition cannot recover creator-owned
characteristics after those global definitions disappear. Current
reconstruction consumers should require `reconstruction.checkpoint.v5` and
regenerate checkpoints with the current engine.

### Migrating checkpoint format 5 to 6

Protocol 26 and replay format 2 remain in place. Checkpoint format 6 removes
Miracle's ambient `miracleWindow` bookkeeping and represents the private
draw-action continuation directly. Its optional Reveal carries the card's real
characteristics rather than an undocumented placeholder; the actionless
ordinary path resolves atomically and leaves no pending checkpoint state.
Revealing creates the linked triggered ability; when that ability resolves, a
public standing decision offers exactly that card's Miracle cost or lets its
controller decline.

An already-open format-5 Miracle window does not record the linked trigger's
placement, priority history, or one-shot decision state, so it cannot be
upgraded without guessing. Consumers should require
`reconstruction.checkpoint.v6`, continue checking the exact simulation
fingerprint, and regenerate format-5 checkpoints with the current engine.

### Migrating from protocol 26 and checkpoint format 6

Protocol 27 removes the synthetic card definition formerly used to present
face-down spells and permanents to an opponent. That observer's
`characteristics` now has
`kind: "faceDown"`, an inline `name`, and an inline `presentation`; it omits
`definition` and `partId`. The physical card still has its real catalog
definition inside the authoritative game state, and an entitled controller's
observation may show it, but an opponent does not learn that identity merely
by observing the face-down object. Ability origins use
the matching `faceDown` and `faceDownGranted` tags rather than attributing
those abilities to a fake printed card.

Checkpoint format 7 stores the standardized ordinary-2/2 and ward-{2}-2/2
face-down presets on spells, permanents, and copied characteristics. A
card-specific value without a stable semantic locator fails closed as deferred
state, like another unlocatable runtime-authored characteristic value, rather
than serializing function pointers. Reconstruction consumers should require
`reconstruction.checkpoint.v7`, keep checking the exact simulation
fingerprint, and regenerate format-6 checkpoints. Replay version 2 is
unchanged.

### Migrating from protocol 27

Protocol 28 broadens the catalog's sparse `manaCost.hybrid` array beyond the
ordinary two-color pairs documented through protocol 27. Its `symbol` member
can now be two-brid (`2/B`), Phyrexian (`R/P`), Phyrexian hybrid (`G/U/P`), or
colorless hybrid (`C/W`). Treat the string as an open display value. Cast
actions can also include the optional `choices.manaPayment` array described
above. Replay version 2 is unchanged.

### Migrating checkpoint format 9 to 10

Protocol 29 and replay format 2 remain in place. Checkpoint format 10 replaces
the dedicated top-card-selection, distributed-selection, typed-selection, and
two-pile continuation tags with the same named object-collection continuations
used by live resolution. Those continuations preserve ordered collections and
can hand successive choices to different players before an ordinary nested
effect acts on chosen and unchosen groups. It also authenticates the ordered
battlefield-exit continuation used when several cards enter one library at the
same position. Format 10 also removes the standalone `sorceryFlashGrants`
counters. Quicken-style permissions are now ordinary resolved play
permissions, including their authored predicate, composed expiration
conditions, and next-matching-cast behavior.

Format 10 effect-resolution contexts may also contain an additive sparse
`namedObjectGroups` member. It preserves labeled outputs across a deferred
sequence without changing the surrounding continuation tags. A missing member
means that no labeled outputs have been declared; a present label with an
empty array is a declared output that produced nothing. Even effects that can
produce at most one object use a zero-or-one object group. Protocol 29 and
replay format 2 are unchanged.

A format-9 continuation in the middle of one of those workflows does not carry
the named bindings or remaining operation chain needed by the declarative
model, and the old grant count cannot reconstruct the richer timing rule.
Reconstruction consumers should require `reconstruction.checkpoint.v10`, keep
checking the exact simulation fingerprint, and regenerate format-9
checkpoints with the current engine.

### Migrating checkpoint format 8 to 9

Protocol 29 and replay format 2 remain in place. Checkpoint format 9 replaces
the positional `counters` array on each permanent with sparse
`{name, count}` entries and serializes intrinsic keyword-counter ability
origins by counter name. It also reads the observation's open
`playerCounters` collection while retaining the `poison` and `energy` arrays
as compatibility projections for ordinary protocol consumers.

A format-8 checkpoint depends on the old catalog-wide counter order and cannot
represent an ordinary counter name without first extending that layout.
Reconstruction consumers should require `reconstruction.checkpoint.v9`, keep
checking the exact simulation fingerprint, and regenerate format-8 checkpoints
with the current engine.

### Migrating checkpoint format 7 to 8

Protocol 28 and replay format 2 remain in place. Checkpoint format 8 records
whether a pending draw replacement is optional and whether it is an installed
one-shot effect, gives battlefield replacement abilities a distinct semantic
continuation kind, and preserves resumable `ForEachInBinding` procedures and
resolved player attack restrictions. It also repeats each live stack object's
kind inside the checkpoint so reconstruction can verify it against the public
observation, and counts each player's land plays in `landsPlayedThisTurn`
rather than flagging the first in `landPlayedThisTurn`.

A format-7 checkpoint cannot distinguish declining an optional static draw
replacement from consuming an installed replacement, and has no representation
for the new iterator or attack-restriction state. Consumers should require
`reconstruction.checkpoint.v8`, continue checking the exact simulation
fingerprint, and regenerate format-7 checkpoints with the current engine.

### Migrating from protocol 24

Protocol 25 makes `ActivateAbility.costObject` an array named `costObjects`:

- It was a nullable single object ID. A cost can name more than one object --
  "exile two cards from your graveyard" -- and the activation has no window in
  which to ask afterwards, so every chosen object travels with the action.
- Read `costObjects` as a list. An ability that spends nothing chosen has an
  empty array rather than a null. Source, ability, and targets no longer
  distinguish two activations that differ only in what they spend.
- `ActivateManaAbility.costObject` is unchanged and remains a single nullable
  object ID.

### Migrating from protocol 21

Protocol 22 splits wire compatibility from conservative source identity:

- `protocolVersion` is now the breaking bot-wire epoch. Ignore unknown members
  in every JSON object. Additional cards, fields, result reasons with a safe
  fallback, and legal actions using existing vocabulary do not by themselves
  move the epoch.
- Observations and catalogs add `protocolCapabilities` and
  `simulationFingerprint`. Capabilities are optional facilities, not another
  exact version vector. The fingerprint is a conservative identity to pin with
  trained weights: equal values identify the same covered inputs, while unequal
  values can also result from a non-behavioral source or build-requirement edit.
- The typed reconstruction `checkpoint` carries its own `version` and repeats
  the simulation fingerprint. The browser command journal similarly carries an
  independent `replayVersion`. Importers require the relevant format version
  and exact fingerprint rather than treating `engineVersion` as a rules hash.
- `engineVersion` now means package-release provenance. It can still be useful
  in diagnostics, but it does not distinguish every development simulation.
- Hosted bots declare `{protocolVersion, capabilities, requiredCapabilities}`
  inside `compatibility` at registration and heartbeat. The server accepts the
  same epoch when each side satisfies the other's required capability subset
  and returns its own compatibility manifest, including its simulation
  fingerprint, in each response. A trained bot may additionally send
  `requiredSimulationFingerprint` to refuse a different simulation before it
  is listed or assigned.

The current optional capability is `reconstruction.checkpoint.v10`. An ordinary
hosted bot that only reads `legalActions` should declare an empty capability
list; do not copy the server's advertised capabilities without implementing
them.

### Migrating from protocol 20

Protocol 21 expands the hidden-safe
`checkpoint` into the complete typed decision-boundary snapshot described
above. Closed protocol-20 decoders must accept its new nested fields. Code that
only reads documented observation fields and selects an index from
`legalActions` needs no action-space change, but should still reject protocol
versions it has not explicitly accepted. Under protocol 22's compatibility
model, future incompatible checkpoint encodings move `checkpoint.version`, not
the bot-wire epoch.

`Game.from_observation` can now resume ordinary hosted observations that carry
deferred triggers, pending replacement events or decisions, restricted mana,
dynamic object state, or retired-object LKI. It also validates the complete
engine-owned public observation after reconstruction, not only legal actions.

### Migrating from protocol 19

Chaos Orb's Old School activation no longer selects a permanent in its
`targets` list. Activate it with the empty list the engine supplies, then, when
more than one eligible permanent exists, answer the public resolution-time
`Choice` with `ChooseDecision`. Zero- and one-candidate choices complete
automatically. The choice is not a target, so hexproof, shroud, and protection
do not constrain it.

### Migrating from protocol 18

Protocol 19 adds the hidden-safe `checkpoint` object to every observation and
the observation-reconstruction entry points described above. Clients with
closed observation decoders must accept the checkpoint and the semantic state
now carried by reconstructible stack objects and pending decisions. Clients
that ignore unknown observation fields can otherwise keep selecting from the
indexed `legalActions` list, but should still reject protocol versions they
have not explicitly accepted.

### Migrating from protocol 17

`result.reason` gained `OpponentRanOutOfTime`, for a seat that lost to a
host's clock rather than to a concession anyone chose. A client that switches
exhaustively on the reason must handle it; one that treats an unrecognised
reason as "the game ended" already works.

Nothing else moved. Only hosted rooms impose a clock, so a bot playing
in-process through the bindings will never see this reason.

### Migrating from protocol 7

Protocols 8 through 17 introduced ten compatibility changes. Then apply the
protocol 18, 19, 20, 21, and 22 migration sections above after these:

- Protocol 8 replaced `manaCost.whiteRedHybrid` with the sparse `hybrid`
  array described above.
- Protocol 9 added `amounts` to every `targetSelections` entry. It is always
  present; include it when comparing or featurizing actions because otherwise
  identical target sets can represent different damage divisions.
- Protocol 10 renamed `ActivateAbility.sacrifice` to `costObject`. Include it
  when comparing actions because distinct payment objects can be the only
  difference between legal activations.
- Protocol 11 made instantiated target slots positional and flattened selected
  modal target ranges. Join targets through the concrete action or stack
  signature rather than assuming a mode-local catalog ID remains unchanged.
- Protocol 12 added the `OpponentLostToAnEffect` game-result reason for an
  effect that makes a player lose without changing their life total or drawing
  from an empty library. Clients that treat `result.reason` as a closed enum
  must accept it.
- Protocol 13 added effective card types to the shared `PermanentObservation`
  and browser presentation, so an animated land reports what it currently is.
  The canonical bot JSON did not gain a `types` field.
- Protocol 14 removed the bespoke `ErhnamForestwalkGranted` engine event after
  Erhnam Djinn moved to the ordinary ability stack and keyword machinery. Rust
  event-log consumers must stop matching that variant; bot JSON shapes are
  otherwise unchanged from protocol 13. Protocol 14 also added
  `beforeCombatDamage` to the catalog play-option `restriction` vocabulary.
- Protocol 15 added a `defender` to every `DeclareAttacker` action, loyalty
  state to planeswalker permanents, command-zone emblems to observations, and
  grouped `members` to decision options.
- Protocol 16 completes Boros Charm. Supported-format legal actions can now
  contain its target-free Mode 1 and planeswalker-targeted Mode 0 casts, and
  its catalog coverage advances from `partial` to `complete`. No action JSON
  field was added; clients must still consume the complete indexed action list
  rather than assuming which modal casts exist.
- Protocol 17 makes Mana Vault's optional upkeep payment decision available
  even when the artifact is untapped. The decision uses the shared
  optional-payment prompt and option labels; consume the indexed actions and
  option IDs rather than matching those presentation strings.
- Protocol 18 adds `OpponentRanOutOfTime` to `result.reason` for a seat that
  loses to a host-enforced clock rather than by concession.
- Protocol 19 adds the hidden-safe `checkpoint` object and observation
  reconstruction entry points used for local determinizations.
- Protocol 20 makes Chaos Orb's Old School activation untargeted and moves its
  nontoken-permanent choice into resolution.

## Putting your bot online

Everything above runs a bot in your own process. To let other people play it,
put it in the registry: register once, then heartbeat. Heartbeating is what
"online" means, and the heartbeat's reply is where games arrive. Both requests
declare the bot's wire epoch and the optional capabilities it actually
supports, so an incompatible bot is refused before a game is assigned.

**If your bot searches, read [Rolling out against worlds you cannot
see](#rolling-out-against-worlds-you-cannot-see) before you port it.** A local
bot searches by cloning the true game; a hosted one is handed a redacted
observation and cannot, which will quietly cost a search bot most of its
strength if it discovers this by measuring its own win rate. The replacement is
`Game.from_observation`: it turns the observation you were given, plus your own
hypothesis for the hidden zones, into a live game you can clone and roll out
like any other. Redaction should cost a bot the knowledge it is not entitled
to, not the ability to think ahead.

### Which server

Two, and the only difference to your code is one string.

| | |
| --- | --- |
| `http://localhost:3000` | **While you are building it.** Your own server, no limits, nothing you do is visible to anyone. Start it with `cd web && pnpm install && pnpm run dev`; a linked worktree gets its own port, which `pnpm run dev:url` prints. |
| `https://penta.lacker.workers.dev` | **When it is ready to meet people.** The public deployment. Your bot appears in the opponent picker and anyone can play it. |

Build against the local one. An iterating bot registers repeatedly, plays
badly on purpose, and restarts a lot -- all of which is fine on your own
machine and none of which needs an audience. The public server also holds
creations to ten a minute per address, which is generous for playing and
tight for a debug loop.

When it works, point it at the public one. Nothing else changes.

```python
import time, requests

# Local while building; the public deployment when you are ready.
SERVER = "http://localhost:3000"
# This bot consumes the protocol-29 indexed-action vocabulary and no optional
# facilities. Do not echo capabilities from the server unless you implement them.
COMPATIBILITY = {
    "protocolVersion": 29,
    "capabilities": [],
    "requiredCapabilities": [],
    # Trained bots may require the exact server artifact they target:
    # "requiredSimulationFingerprint": "sha256-...",
}

registration = requests.post(
    f"{SERVER}/_bots/register",
    json={
        "name": "Fizzbot",
        "deck": "Sligh",
        "compatibility": COMPATIBILITY,
    },
)
registration.raise_for_status()
me = registration.json()

due = 0.0

def beat(done=()):
    """Renew presence if the lease is due, and collect any invitations."""
    global due
    if time.monotonic() < due and not done:
        return None
    due = time.monotonic() + 10
    response = requests.post(
        f"{SERVER}/_bots/{me['id']}/heartbeat",
        json={
            "token": me["token"],
            "done": list(done),
            "compatibility": COMPATIBILITY,
        },
    )
    if response.status_code == 409:
        raise RuntimeError(f"bot is incompatible with this server: {response.text}")
    response.raise_for_status()
    return response.json()

finished = []
while True:
    # Passing `done` makes this beat due regardless; only a beat with nothing
    # to report can come back empty, having decided the lease was still good.
    reply = beat(finished) or {"invites": []}
    finished = []
    for invite in reply["invites"]:
        play(invite["room"], invite["token"], beat)   # see below
        finished.append(invite["room"])
    if not finished:
        time.sleep(10)
```

Calling `beat` on a due check rather than unconditionally is what lets the
play loop below call it on every pass without thinking about it.
`examples/python/hosted_bot.py` is this with the retries a long-running bot
needs.

Registration and heartbeat responses include the server's `compatibility`
manifest: `{protocolVersion, capabilities, requiredCapabilities,
simulationFingerprint}`. Compatibility
means the epochs are equal, every vocabulary capability the server requires is
in the bot's `capabilities`, and every facility the bot lists in its own
`requiredCapabilities` is in the server's advertised `capabilities`. Unknown
extras are harmless. If a bot sends `requiredSimulationFingerprint`, it must
also equal the server fingerprint. A well-formed mismatch returns HTTP 409 with code
`incompatible_bot`; malformed declarations return 400. The registry also hides
registrations that no longer match and rechecks immediately before issuing an
invitation.

A game is two ordinary requests in a loop. `opponent` says whether your seat
holds the decision and hands back the observation this guide describes;
`command` submits an index into its `legalActions`. Keep heartbeating while
you play: a game is the longest stretch a bot spends busy, and presence is a
lease on wall-clock time, so this loop is where most of a bot's heartbeats
belong.

**Wait rather than poll.** `opponent` takes an optional `wait`, in
milliseconds, and holds the request open until your seat holds the decision,
the game ends, or that long passes -- then answers exactly as it would have
anyway. This matters more than it looks: a game asks the opponent seat for a
decision at every priority pass, which is many times per turn, and a poller
pays a full poll interval for each one. Measured on a development server,
handing a decision to a bot polling at 250ms took **221ms on average and
496ms at the 90th percentile**; the same handoff to a waiting bot took
**9ms**. It also costs the server less than being asked repeatedly, so there
is no politeness argument for polling either.

```python
def play(room, token, beat):
    headers = {"x-penta-token": token}          # from the invitation
    while True:
        beat()                                  # heartbeat if the lease is due
        view = requests.get(
            f"{SERVER}/_game/{room}/opponent",
            params={"wait": 8000},              # park until it is our turn
            headers=headers, timeout=40,
        ).json()
        if view["result"]:
            return
        if not view["deciding"]:
            continue                            # the wait elapsed; ask again
        index = choose(view["observation"])
        requests.post(
            f"{SERVER}/_game/{room}/command",
            json={"t": "botAct", "index": index},
            headers=headers,
        )
```

A few rules the parameter follows. The server caps a wait at **30 seconds**
and silently uses the smaller of that and what you asked for; anything absent,
unparseable, or not positive means you did not ask to wait, and you get the
immediate answer bots have always got. Set your HTTP client's timeout above
your wait, or your own library will hang up on the answer. Waiting counts as
being present, so a parked bot is not one that has gone away -- which is why
the cap sits below the 45-second presence window rather than at some rounder
number.

Every invitation carries a `token`, and it is what lets you play that seat.
A room id is a name, not a permission: it travels in URLs and gets shared, so
the room asks for the token on every request and answers 403 without it. The
same token is how the room knows you and not some passer-by is its opponent.

That is the whole integration -- no WebSocket, no engine build, no penta
module. `examples/python/hosted_bot.py` is this with argument parsing and a
`choose` you can replace.

### What a public server limits

A deployment that serves these routes is open to anyone who can reach it, so
it holds the creating routes -- starting a room, registering, challenging --
to **ten a minute per address**, and answers 429 with a `retry-after` when
you pass that. Reading and moving are not counted: a game in progress is
chatty by nature, and the move clock below already bounds it.

Two housekeeping rules a long-running bot will eventually meet. A
registration nobody has used for a day is deleted, so a bot that stops for
good stops being listed; heartbeating at all keeps yours. And a finished
game's room is released an hour after it ends, so fetch a replay you care
about rather than assuming the room will be there tomorrow.

### Losing on time

A room runs a move clock. Whoever must act has **60 seconds** if they are a
bot and five minutes if they are a person; every applied command starts the
budget again, so it is a clock per move, not per game. Run out and that seat
loses, with `result.reason` reporting `OpponentRanOutOfTime`. It is
deliberately not a concession: nobody chose it, and a bot learning from its
own results should be able to tell "my opponent gave up" from "I was too
slow".

Go silent with a game in progress and you lose that game too, without waiting
for the clock. The registry notices within about two presence windows and
tells the room, and the result says so: `result.reason` is still
`OpponentRanOutOfTime`, but the message names the bot that stopped answering
rather than the clock, so the human can tell an absent opponent from a slow
one. This is the faster answer for a bot that is gone; the move clock is the
backstop for one that is still running but stuck.

Silent means silent everywhere. The registry counts a bot as present if it is
still talking to the rooms it owes games to, so a bot that polls `opponent`
and posts `botAct` will not be dropped mid-game merely because its heartbeat
lapsed. Do not lean on that: it is a safety net for the one mistake that is
easiest to make, not a substitute for heartbeating. Only the heartbeat keeps a
bot in the listing where people can find it, and only the heartbeat covers the
stretches between games.

Three consequences worth designing for:

- Answer promptly even when the answer is `PassPriority`. The clock does not
  care that a decision was uninteresting.
- Keep heartbeating from inside your play loop, not only between games. A
  loop that plays a whole game before its next heartbeat is a bot that
  disappears from the listing for the length of every game.
- Report finished rooms in `done`. A game that ended while you were not
  looking -- because you lost on time -- still counts against your one game at
  a time until you say so or the invitation expires.

### What being online means

| Call | Meaning |
| --- | --- |
| `POST /_bots/register {name, deck, discloseDeck?, compatibility: {protocolVersion, capabilities, requiredCapabilities, requiredSimulationFingerprint?}}` | Once per registration. Returns `{id, token, deck, discloseDeck, compatibility}`; keep the token. An incompatible declaration returns 409. |
| `POST /_bots/<id>/heartbeat {token, done, discloseDeck?, compatibility}` | Renews presence and compatibility, returning `{invites, deck, discloseDeck, compatibility}`. An incompatible declaration returns 409 before presence is renewed. |
| `GET /_bots` | Returns `{compatibility, bots}` for compatible bots that are online now, with `busy`. |
| `POST /_bots/<id>/challenge {room, token}` | Asks an idle bot to play a started room. `token` is that room's bot-seat token, which only whoever started the room has, so nobody can park your bot in a room of theirs. The web client does this when someone picks you. |

Presence is a lease, not a connection. Heartbeat at least every 15 seconds;
miss 45 and you drop off the list. Registering is not being online -- the
first heartbeat is -- and stopping is just not heartbeating, which is also
how you deploy a new version: stop, and your games in flight still finish,
because they run on their own requests.

A bot plays one game at a time. Report a finished room in `done` to free
yourself for the next challenger; an invitation you never pick up expires
after ten minutes, so a bot that dies mid-game unsticks itself.

The deck you register is what you play when a scheduler pairs you, and what
the web client offers as your side of the matchup. An omitted compatibility
declaration identifies a pre-negotiation protocol-21 bot. Protocol 23 therefore
requires an explicit declaration: this is how a bot opts into open-world
objects rather than being assumed to tolerate them.

### Open decklists (optional)

By default a hosted match discloses nothing about either deck beyond what
play itself reveals -- the situation this guide describes everywhere else, and
still the only option unless both sides ask for something different. A bot
that would rather know its opponent's archetype up front, the way an
archetype is known at a competitive paper table, can opt in.

Add `"discloseDeck": true` to your `/_bots/register` or heartbeat body. It is
a consent signal more than a disclosure of its own: the deck you registered is
already named in the public `/_bots` listing, so what the flag actually unlocks
is the *human* seat's deck being named to you, and then only when that seat has
opted in as well. The registration and heartbeat responses echo it back as
`discloseDeck`, and the listing shows it for every bot, so a human or scheduler
can tell which bots play open-decklist games. A heartbeat that omits the field
leaves your prior declaration as it was; send it as `false` to withdraw it.

Whoever starts a room declares the human seat's own willingness the same
way, with `"humanDiscloseDeck": true` in the `POST /_game/<room-id>/start`
body.

Disclosure only takes effect when *both* seats have opted in for that
specific room -- one side opting in, alone, changes nothing. When they have,
the bot seat's observation, from both `/opponent` and the `observe` message
on the bot socket, carries one extra field:

| field | meaning |
| --- | --- |
| `opponentDeck` | present only when both seats opted in to open decklists; the human seat's registered deck name |

Absent that mutual opt-in, nothing changes: no `opponentDeck` field appears,
and a bot that never asks for this sees exactly the redacted observation it
always has. This is additive, protocol-27-compatible JSON -- ignore it if you
do not use it, same as any other field this guide describes. It is also a
hosted-room convenience layered on top of the wire protocol by the deployment
itself, not a change to the core engine: a local `penta.Game`, and the
`observe()` fields the engine emits directly, are unaffected either way, since
neither the registry nor a room's own deck configuration exists there.

## Hosted games over WebSocket

Polling as above is the simplest way in. A bot that wants to answer the
instant it is asked can hold a socket instead: same contract, pushed rather
than pulled.

A deployment that enables its server-side game routes hosts one game per
room at `/_game/<room-id>/…`. The human plays through the ordinary web UI
(`/?hosted=<room-id>`); a bot drives the opponent seat by connecting to
`/_game/<room-id>/ws?role=bot` after the room is started with
`botPolicy: "External"`.

The bot's contract is the one this guide already describes, moved onto a
socket. Whenever the opponent seat holds the decision the room sends

```json
{ "t": "observe", "observation": { …the observation described above… } }
```

and the bot answers with an index into that observation's `legalActions`:

```json
{ "t": "act", "index": 3 }
```

That observation carries `opponentDeck` when [open decklists](#open-decklists-optional)
are in effect for this room, and is otherwise identical to what `/opponent`
already returns while polling.

A hosted room's state payload carries `moveClock` -- `{seat, deadline}`, with
the deadline in epoch milliseconds -- whenever a game is live, so a client can
show what remains.

While an external opponent owns a private decision, human polling and
reconnects receive the last complete human-safe state rather than a mixture of
that board and the live private boundary. Its clock is frozen with that state
until the next safe update. The human-only room-record route likewise withholds
both the seed and command journal of an unfinished external game: a private
decline can otherwise be exposed by one extra recorded command. The complete
seed and journal become available after the game ends; seat credentials are
never part of a room record.

A new observation follows if the seat still holds the decision; `{"t":
"result", …}` arrives when the game ends, and `{"t": "error", …}` reports a
rejected action, after which the previous observation still stands. The
room never sends the seed of an external game, and it rolls that seed
itself, ignoring the starter's suggestion — whoever picks the seed can
precompute both hands.

These routes, and the registry above, are gated by `HOSTED_GAMES`. Each seat
of a room is held by a token minted when the room starts, so knowing a room
id lets you name a room and nothing else.

## Determinism and versioning

A `simulationFingerprint` is the engine's conservative guard for a format,
ordered decks, seed, opponent configuration, and submitted action/decision
sequence. Record it with training episodes and control the target/build inputs
documented below when byte-for-byte reproducibility matters. Portable browser
replays stamp the fingerprint together with the independent `replayVersion` of
their command-journal envelope.

The identifiers answer different questions:

- `protocol_version()` / `protocolVersion` is the breaking bot-wire epoch. It
  changes when an existing JSON field, tag, identifier, or meaning can no longer
  be consumed safely. Protocol-23 objects are open-world, so optional fields,
  catalog growth, and new action instances using existing vocabulary do not
  require a bump.
- `protocolCapabilities` advertises optional facilities within that epoch.
  Ordinary consumers ignore unknown entries. Hosted compatibility requires an
  equal epoch, the server-required subset of bot-supported vocabulary, and the
  bot-required subset of server-advertised facilities.
- `simulation_fingerprint()` / `simulationFingerprint` is a conservative
  identity over production engine source, resolved core dependency closure,
  repository deck data, and the pinned toolchain. Equal values identify the
  same covered inputs; unequal values can also result from a source or
  package-metadata edit that does not change play. Pin it alongside trained
  weights and exact artifacts.
- `engine_version()` / `engineVersion` is ordinary package-release provenance.
  It does not uniquely identify a development simulation and is not an exact
  replay guard.
- `checkpoint.version` and replay `replayVersion` version those independently
  consumed encodings. Reconstruction and replay require the appropriate format
  version plus the same conservative simulation fingerprint.

[CHANGELOG.md](../CHANGELOG.md) records what moved between versions and what a
bot has to do about it. Before 1.0, expect the action space to keep
settling — reading the `type` tags rather than hardcoding indices costs
nothing now and survives those changes.

## Engine coverage

See [formats and current scope](formats.md) for supported formats, built-in
decks, rules deviations, and known limitations. `penta.catalog(format)` is the
authoritative machine-readable description of the selected format's card
legality and implementation coverage. Run `make catalog-report` for current
aggregate catalog and coverage counts.

## Where this is going

The local protocol is the intended basis for a future tournament service: the
authoritative engine can stay on the server while a bot receives redacted
observations and returns action indices. Search bots are meant to be
first-class there rather than structurally disadvantaged, which is what
[`Game.from_observation`](#rolling-out-against-worlds-you-cannot-see) is for:
a seat that can only see its own observation can still build worlds consistent
with it and search them. The wire contract is still evolving before 1.0, so
check the breaking epoch, negotiate required capabilities, and use the
changelog to migrate.
