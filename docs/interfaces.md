# Engine interfaces

Penta exposes one authoritative state machine through several adapters. This
document describes the boundaries between them. Bot authors should use the
task-oriented [bot guide](bots.md), which also documents the JSON observation
and action vocabulary.

## Core `Game` API

`Game` owns authoritative state. Consumers do not mutate zones, life, mana,
priority, or the stack directly. They ask for `legal_actions(player)` and
submit one of those values to `apply(player, action)`. `apply` checks legality
again so a stale decision fails without changing state.

For a generic `DecisionObservation`, `legal_actions` returns a compact
`ChooseDecision` marker. Callers select option IDs from the observation and use
`is_legal_action` and `apply` for validation without expanding every possible
combination.

A runner asks `decision_player()` who must act, observes that player, and
submits one of the observation's legal actions:

```rust
while let Some(player) = game.decision_player() {
    let observation = game.observe(player);
    let action = bots[player.index()].choose_action(&observation);
    game.apply(player, action)?;
}
```

The decision player is normally the player with priority, but differs during
mulligans, blocker declaration, restricted untaps, cleanup discards, and
triggered or combat-damage choices.

## Observations and events

`PlayerObservation` is the hidden-information-safe input for a player or bot.
It contains that player's hand and only counts for an opponent's unrevealed
hidden zones. Rules-driven disclosures can add the known, possibly stale
`lastSeenHand` snapshot documented in the [bot guide](bots.md); they never
expose cards the player has not learned. `GameEvent` is an omniscient debugging
and replay stream; it must not be used as a player observation.

`PermanentObservation.characteristics` reports effective copiable values.
`physical_face` separately reports the public topology and side of a face-up
physical double-faced permanent, so consumers must not infer transformability
from copied characteristics.

Protocol 27 represents created tokens, emblems, and face-down objects without
inventing catalog cards. Printed characteristics retain catalog definition and
part IDs; token, emblem, and rules-defined face-down characteristics travel
inline and never invent a card-definition identity. A token's local part index
describes only its own face structure, while a permanent's separate `token`
flag remains the rules object's token status even after copying. A face-down
object's physical card keeps its real definition. An opponent sees only the
characteristics assigned by its face-down mechanism, while a controller who
may inspect the card retains its real catalog presentation.

Like the token characteristics tag, the face-down characteristics tag records
where copiable values came from, not the current physical status. A face-up
copy can retain those values; consumers use the permanent's separate
`faceDown` boolean to decide whether the physical object is face down.

The engine enumerates legal actions rather than asking consumers to construct
partially legal commands. Complex multi-selection decisions expose bounded
options and are submitted through the same checked state-machine boundary.

## Consumer layers

- `Game` is the native Rust rules state machine.
- `protocol::BotGame` presents stable JSON observations and indexed actions for
  bot and binding consumers.
- `bindings/penta-py` and `bindings/penta-ffi` expose that same protocol to
  Python, C, C++, and other FFI-capable languages.
- `wasm/` exposes the engine to the browser. The web client selects from engine
  actions and decisions rather than reconstructing rules in TypeScript.

## Compatibility boundaries

The adapters expose several identifiers because compatibility is directional,
not one exact-version comparison:

- `protocolVersion` is the breaking epoch for canonical bot observation,
  action, and catalog JSON. Protocol JSON objects are open-world: consumers
  ignore members they do not use. The epoch changes when an existing field or tag is
  removed, renamed, retyped, or reinterpreted, not when an optional field or a
  legal action expressed through existing vocabulary is added.
- `protocolCapabilities` advertises named, additive facilities such as
  `reconstruction.checkpoint.v11`. A consumer may ignore capabilities it does
  not use. Hosted bots declare both supported vocabulary and facilities they
  require; compatibility needs an equal `protocolVersion` and each side's
  required subset to be supplied by the other.
- `simulationFingerprint` conservatively identifies the production engine
  source, resolved core dependency closure, repository deck data, and pinned
  toolchain. Equal values identify the same covered inputs; an unequal value can
  also come from a source or package-metadata edit that does not change play.
  Pin it with trained policies and require it for deterministic reconstruction
  and replay.
- `engineVersion` is the package release version. It is useful provenance and
  follows Cargo SemVer, but it is not an exact simulation identity.
- The nested checkpoint payload and browser command journal have independent
  `version` and `replayVersion` fields. Their encodings can therefore move
  without changing the ordinary bot wire epoch.

Query `protocol_version()`, `simulation_fingerprint()`, and `engine_version()`
through the relevant binding. Release history and migration notes live in the
[changelog](../CHANGELOG.md); the precise JSON and hosted negotiation contract
lives in the [bot guide](bots.md).

When authoring a compatibility change, bump `protocolVersion` once relative to
the target branch only when an old consumer could misinterpret existing wire
data: removing, renaming, retyping, or changing the meaning of a key, tag,
identifier, or index, or adding mandatory vocabulary with no negotiated
fallback. Do not bump it for an optional open-world member, append-only catalog
growth, legal-action membership expressed through existing shapes, a rules fix,
presentation text, browser state, native Rust APIs, or replay/checkpoint
encoding. Use a named capability for an additive facility and the appropriate
replay or checkpoint version for incompatible changes to those artifacts.

Classify new string-enum values as open with a safe fallback or as closed and
capability-gated. Never derive a stable wire tag from Rust `Debug` formatting.
When a public contract or exact-artifact format changes, update this guide's
authoritative consumer documentation, the changelog, and affected binding
examples. Tests should exercise behavior rather than hard-code the current
epoch or name a branch as its owner. Keep the root `BOTS.md` compatibility
symlink pointed at `docs/bots.md`.
