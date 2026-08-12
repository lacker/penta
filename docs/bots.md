# Writing an AI bot for Penta

penta is a deterministic engine for two-player constructed Magic. It currently
ships Eternal Central Old School 93/94 and the final pre-Theros ISD–RTR
Standard format. This guide is for writing a program that plays it: from
Python, C, C++, or Rust, against the included bots or against itself.

This guide describes the current development wire contract, **protocol 17**.
Query `protocol_version()` and `engine_version()` through the selected binding
and reject or migrate versions your client does not understand; pin both
alongside trained weights. Old School remains the default for compatibility;
new integrations should record and pass an explicit format slug with each
game.

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

Requires Python 3.9+ and [rustup](https://rustup.rs), which installs the
repository's pinned Rust version automatically. From the repository root:

```bash
cd bindings/penta-py
cargo build --release
cp target/release/libpenta.dylib penta.so   # Linux: cp target/release/libpenta.so penta.so
python3 -c "import penta; print(penta.engine_version())"
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
    format="isd-rtr-standard",
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
| `game.hand(seat)`, `game.library(seat)` | a zone's real contents, unredacted — for simulating, not for playing |
| `game.set_hand(seat, defs)`, `game.set_library(seat, defs)` | say what a zone holds, in a fork |
| `game.result()` | `None`, `"p1"`, `"p2"`, or `"draw"` |
| `penta.catalog(format=)` | every canonical definition annotated with legality for the selected format, as JSON |
| `penta.deck_names(format=)` | the selected format's built-in decks |
| `penta.engine_version()`, `penta.protocol_version()` | pin these with your trained weights |

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
all. From C++, wrap the header and parse observations with any JSON library
(e.g. nlohmann/json). Anything else with a C FFI — Julia, Go, C# — can consume
the same library.

## Quick start: Rust

The engine is an ordinary crate. Depend on it by path (or git) and use the
same facade the bindings use:

```rust
use penta::protocol::{BotGame, Opponent};
use penta::{Format, PlayerId};

fn main() -> Result<(), String> {
    let mut game = BotGame::new_with_format(
        Format::IsdRtrStandard,
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

A clone forks the *true* state, hidden zones included. For self-play training
that is exactly right. For a search bot choosing a move in a real match it is
not: rollouts on the true world are influenced by cards the searcher has not
seen, so the outcomes it measures encode information it does not have.

The fix is to search over worlds consistent with what your seat actually
knows. You do not know their last card — it could be a Lightning Bolt, it
could be a Counterspell — so build both worlds and roll each one out:

```python
catalog = {c["name"]: c["definition"] for c in json.loads(penta.catalog())["cards"]}

for guess in ("Lightning Bolt", "Counterspell"):
    world = game.clone()
    world.set_hand("p2", [catalog["Mountain"], catalog[guess]])
    # ... roll this world out and score it
```

`set_hand` and `set_library` say what a zone *holds*, by card definition. The
cards are built fresh, so you are stating a hypothesis rather than shuffling
the real one, and nothing is conserved: stack a library, empty it, or hand
someone a card that was never in their deck. A world you invented has no
reason to balance.

`hand(seat)` and `library(seat)` read the zones back as
`[{objectId, definition}]`, unredacted, so you can see the true state you are
replacing and weight your guesses however you like. The engine ships no
sampler — a uniform re-deal, a weighting by what the opponent has cast, and a
belief filter maintained across turns are all just different lists of
definitions.

Two things to know. Rewritten cards get new object IDs, so rewrite an
opponent's zones rather than your own if you are holding IDs from an earlier
observation. And these accessors are not redacted, which is not a hole in
match secrecy: a tournament server hands a bot redacted observations over a
wire and never a game object, so transparency here only reaches someone
simulating in their own process, where there is nobody to hide from.

## The observation

`observe()` returns one JSON object. The essential fields:

| field | meaning |
| --- | --- |
| `protocolVersion`, `engineVersion` | the wire and rules versions that produced this observation; verify both before acting |
| `format` | the rules/deck profile slug, such as `"old-school-93-94"` or `"isd-rtr-standard"` |
| `seat` | whose view this is: `"p1"` or `"p2"` |
| `pregame` | true while mulligans are being settled |
| `turn`, `activeTurn`, `activeSeat`, `prioritySeat`, `step` | where the game is; `activeTurn` counts turns started by the active player, including extra turns, and `step` is one of `Upkeep`, `Draw`, `PrecombatMain`, `BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, `EndOfCombat`, `PostcombatMain`, `End`, `Cleanup` |
| `regularCombatDamagePending` | true during the priority window after first-strike damage and before regular combat damage; both damage waves otherwise use `step: "CombatDamage"` |
| `life`, `manaPools`, `librarySizes` | two-element arrays, indexed p1 then p2 |
| `hand` | your cards: `{objectId, instance, definition, name}`; `instance` is a compatibility alias for `objectId` |
| `opponentHandSize` | their current hidden hand as a count; learned snapshots are reported separately in `lastSeenHand` |
| `lastSeenHand` | null or the most recently revealed hand snapshot as `{seat, cards}`; it records known information and can outlive later hand changes |
| `battlefield` | every permanent, including its current-zone object ID, canonical definition, and presented card-part ID; a planeswalker also reports `loyalty` and `loyaltyAbilityUsedThisTurn` |
| `emblems` | command-zone emblems, each with its controller, name, granting ability, and clause texts |
| `stack` | pending spells, activated abilities, and triggered abilities, bottom to top; entries expose the source object ID, creating definition and ability origin/text, controller, counterability, targets, chosen permanents, X, and a locked cast signature when applicable |
| `graveyards`, `exiles` | public zones, both players |
| `decision` | a pending choice (see below), or null |
| `result` | null while running, else `{winner, reason}`; `reason` is `OpponentConceded`, `OpponentLostAllLife`, `OpponentTriedToDrawFromEmptyLibrary`, or `OpponentLostToAnEffect` |
| `legalActions` | what you can do, each with an `index` |

An attempted draw from an empty library does not end resolution immediately.
The engine records it until the next state-based action check, when it settles
that condition together with both players' life totals; if both remaining
players lose in that check, `result` is a draw. When one effect instructs both
players to draw, the active player completes all of their individual draws
before the nonactive player begins theirs. This is visible in resulting hand
sizes and game events but adds no observation field or legal-action shape.

Cards are referenced two ways: the object ID identifies one rules object in
its current zone, while `definition` identifies the canonical card kind and is
the key into `penta.catalog(format)`. A true zone change creates a new object
ID, so a Goblin Balloon Brigade card in hand, its spell on the stack, and its
permanent on the battlefield are distinct. Transforming, flipping, and phasing
do not create a new object. Physical-card lineage is private engine state and
never appears in a player's observation. Fetch the format's catalog once at
startup.

### Actions

Every entry in `legalActions` has an `index` (what you pass to `act`) and a
`type` naming the engine action, plus fields saying what it operates on:

`KeepHand`, `TakeMulligan`, `BottomCards`, `PlayLand` (with `card` and
`playOptionId`), `CastSpell` (with the play option, ordered modes, cost
configuration, target slots, sacrifices, and X already filled in — one entry
per legal casting choice), `ActivateAbility`, `ActivateManaAbility`, `PayLifeForMana`,
`DeclareAttacker`, `FinishDeclaringAttackers`, `DeclareBlocker`,
`FinishDeclaringBlockers`, `AssignCombatDamage`, `DiscardCards`,
`ChooseUntap`, `ChooseDecision`, `CancelDecision`, `PassPriority`.

An index belongs only to the observation that contains it. Acting changes the
state and rebuilds `legalActions`, so re-observe before choosing again; do not
cache an index across actions even when the visible `type` is the same.

For `CastSpell`, `card` and `sacrifices` are top-level. The canonical nested
`choices` object contains `playOptionId`, ordered `modeIds`, nullable
`alternativeCostId`, `additionalCostIds`, `x`, and `targetSelections`.
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
- `intrinsicBasicLand` carries the lowercase `landType` whose rules supplied
  the ability.
- `granted` carries the granting `source` object together with
  `sourceDefinition`, `sourcePartId`, `sourceAbilityId`, and `grantId`.

`ActivateAbility` carries `source`, `ability`, `x`, a nullable `costObject`,
canonical `targetSelections`, flattened `targets`, and a compatibility
`target` containing the first selected target. `costObject` identifies an
object chosen to pay an object-naming cost, such as sacrificing a permanent or
exiling a card from a graveyard. Targets and that object are chosen before the
activated ability becomes an independent stack object. `ActivateManaAbility`
uses the same source and origin vocabulary, adds the selected `color`, and
resolves immediately because mana abilities never use the stack. The engine
does not infer that classification merely because an effect happens to produce
mana.

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
matching `FinishDeclaring...` action. `AssignCombatDamage` names one `attacker`
and an ordered `assignments` list of `{recipient, amount}`.

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
`cancellable`, and `options`. Each option has its own `id`, `label`, nullable
card and `abilityText`, and a `zone`. They also arrive as `ChooseDecision`
entries in `legalActions`: a pick-exactly-one decision becomes one indexed
action per option, so an index-only bot handles it like anything else. For a
pick-several decision, `legalActions` carries one default selection (the first
`minimum` options) and `choose_decision([option_ids])` submits any other
selection you'd prefer. Submit option IDs, not option-array offsets, within
the reported bounds. When `cancellable` is true, `CancelDecision` is a
distinct legal action; cancelling is not the same as choosing zero options.

Decision prompts and option labels are presentation text, not stable protocol
identifiers. They can become more precise when a rules procedure moves to a
shared implementation without changing the decision shape or option IDs. Read
them for display, but submit the IDs from the current observation and use the
structured decision fields and legal actions for control flow.

### Catalog and mana costs

`penta.catalog(format)` carries the same `protocolVersion`, `engineVersion`,
and `format` as observations, plus `formatName` and the canonical `cards`
array. The array is ordered by `definition` and is not filtered: it contains
tokens and definitions outside the selected format as well as playable cards.
That includes off-format rules test cases such as Darksteel Ingot (definition
`263`, debut set `darksteel`), whose indestructible ability is executable even
though the card is not legal in either shipped format.
`allowed` means the definition belongs to the format's card pool; `legal` is
`allowed && !banned`, so a restricted card is still legal. Definitions include
their structure, parts, play options, legality, printings, and clause-derived
implementation status.

Catalog contents may grow compatibly within one protocol version because
definition IDs are append-only. The out-of-format interaction fixtures
`Urborg, Tomb of Yawgmoth` (definition 261, debut set `planar-chaos`) and
`Yavimaya, Cradle of Growth` (definition 262, debut set
`modern-horizons-2`) appear in every unfiltered catalog but have `allowed` and
`legal` set to `false` for both supported formats. They therefore add no legal
actions to an ordinary supported-format game.

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

A play option's `restriction` is `normal`, `fromHandOnly`, or
`beforeCombatDamage`. Read the tag rather than assuming every otherwise valid
option is available from any zone or at every casting window.

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
  "hybrid": [{"symbol": "R/W", "count": 3}],
  "variableX": false,
  "xMultiplier": 1
}
```

`hybrid` is sparse: it contains one entry for each nonzero two-color pair, and
`count` says how many copies of that printed symbol occur. Protocol 8 replaced
protocol 7's one-off numeric `whiteRedHybrid` field with this general array.
The shape is used everywhere the catalog reports a cost, including parts,
play options, alternative costs, and additional costs.

### Migrating from protocol 7

Protocols 8 through 17 introduced ten compatibility changes:

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

## Hosted games over WebSocket

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

A new observation follows if the seat still holds the decision; `{"t":
"result", …}` arrives when the game ends, and `{"t": "error", …}` reports a
rejected action, after which the previous observation still stands. The
room never sends the seed of an external game, and it rolls that seed
itself, ignoring the starter's suggestion — whoever picks the seed can
precompute both hands.

These routes are development-flagged (`HOSTED_GAMES`), unauthenticated, and
carry no move clock yet; treat them as a local or trusted-network surface.

## Determinism and versioning

The same engine and protocol versions, format, ordered decks, seed, opponent
configuration, and submitted action/decision sequence produce the identical
game, byte for byte. Record all of them for reproducible replays and training
episodes.

Two numbers describe what you trained against, and both are worth pinning
alongside your weights:

- `protocol_version()` covers versioned consumer-facing shapes and the action
  space they describe. It bumps when an older integration could misread
  current output — including changes to `legalActions`, shared observations,
  events, or browser state — even when canonical bot JSON otherwise holds.
- `engine_version()` covers rules behavior, which is part of the contract
  too: a rules fix can change what a trained policy sees even when the
  shapes hold still.

[CHANGELOG.md](../CHANGELOG.md) records what moved between versions and what a
bot has to do about it. Before 1.0, expect the action space to keep
settling — reading the `type` tags rather than hardcoding indices costs
nothing now and survives those changes.

## Engine coverage

See [formats and current scope](formats.md) for supported formats, built-in
decks, rules deviations, and known limitations. `penta.catalog(format)` is the
authoritative machine-readable description of the selected format's card
legality and implementation coverage.

## Where this is going

The local protocol is the intended basis for a future tournament service: the
authoritative engine can stay on the server while a bot receives redacted
observations and returns action indices. The wire contract is still evolving
before 1.0, so version-check at startup and use the changelog to migrate.
