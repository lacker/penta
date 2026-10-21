# Native single-match host

`penta-host` owns one match in one native process. It calls the shared Rust
engine directly; it does not start Vite, a browser, WASM, or the hosted-room
daemon. The existing [hosted session workflow](bot-sessions.md) remains available.
Native API 2 changes the default observation to `proposal-v1` state and the
default client presentation to `pilot-v3`. Native API 1 consumers must migrate
to proposals or explicitly call `discover`, which returns the API 1 menu envelope.
The canonical bot wire and hosted web sessions keep their existing contracts.
The native host, one-shot client, and optional persistent CLI session make no model calls. Pilots choose
exact actions or explicitly authorize conditional priority passes.

## Start and distribute seats

Build the host with `make build-native-host`. Start the resulting binary
with registered format and deck names:

```sh
target/release/penta-host --format FORMAT_SLUG --p1-deck P1_DECK --p2-deck P2_DECK --match-mode first-to-two-wins --port 0
```

Replace the uppercase placeholders with actual registry values, quoting names
containing spaces. `one-conclusion` is also supported. Port zero asks the OS for
an available port. Keep the process running; its startup stdout is JSON:

```json
{"apiVersion":2,"server":"http://127.0.0.1:PORT","organizerSecret":"SECRET","status":"ready"}
```

The organizer calls `seats` with its secret. Every native client invocation
accepts a single JSON object, inline, through `@FILE`, or through stdin (`-`):

```sh
node tools/penta-agent/native.mjs seats -
```

Supply `{"server":"http://127.0.0.1:PORT","secret":"ORGANIZER_SECRET"}` on stdin.
The result contains `seats.p1.secret` and `seats.p2.secret`. Give each pilot only
the server URL and its own seat secret. Retain the organizer secret separately.
Start pilots with fresh model context so they do not inherit the organizer's
launch output or the other seat's secret. For Codex subagents, use
`fork_turns: "none"` and pass only the seat assignment, skill, URL, and own secret.
The client never writes credentials, connection handles, retry bodies or
observations to disk. Stdin avoids putting credentials in command arguments;
tool transcripts may still contain submitted inputs.

## First-to-two matches and sideboarding

The native host defaults to `first-to-two-wins`. Keep the host and seat secrets
for the entire match. `position.situation.match` reports `wins`, `draws`, `game`,
and `stage`; an individual game ending does not make the packet `complete`.

At `sideboarding`, follow the seat's private decision prompt. When asked for the
next main deck, expand option `defaults`, then select **all its option IDs**, including
retained cards. `Library` marks the current main deck and `OutsideGame` the
current sideboard. Each copy has its own option ID. Submit the selection with
the normal `{decision: ID, options: [IDs]}` choice; the engine validates deck
size and the registered card pool. The pilot packet includes printed rules for
the seat's entire main deck and sideboard, including cards not yet drawn.

After both submissions, `play-draw` asks the previous game's loser to choose
“Play first” or “Draw first.” After a draw, the prior chooser keeps that choice.
The host starts the next game with the submitted
decks and the same seat identities. Yields stop at game boundaries and never
answer sideboarding or play/draw choices. Continue until packet `status` and
match `stage` are `complete`, when a seat has two wins. `one-conclusion` skips
this match lifecycle.

## Persistent local client

For agent tools, use a memory-only local session with finite request commands.
Start the session once, supplying the engine URL and seat secret on stdin:

```sh
node tools/penta-agent/session-rpc.mjs serve -
```

The initialization object is `{"server":"http://127.0.0.1:PORT","secret":"SEAT_SECRET"}`.
Close stdin after this object. Keep the process running and retain its returned
`clientServer` URL. For every request, start a finite command:

```sh
node tools/penta-agent/session-rpc.mjs call -
```

Supply `{"clientServer":"RETURNED_URL","secret":"SEAT_SECRET","request":{"method":"next"}}`
on stdin. The command exits when the reply arrives, while the local session
retains catalog, rule, and event delivery context. This avoids the fixed output
collection delay that some terminal tools impose on persistent PTYs. In Codex,
use a 30000-ms initial `exec_command` yield for the finite call; it returns early
when the process exits. Poll only if the command is still running.

The session binds to loopback, authenticates every request with the same seat
secret, and rejects concurrent requests. It writes no files. Send
`{"method":"shutdown"}` as the request to close this local session; the match
host remains running. After a disconnected caller, the next valid request
refreshes delivered context. Resolve an uncertain mutation before playing again.
This transport acknowledges HTTP delivery, not whether a model read the reply;
use `full:true` after any lost tool output.

The JSON-lines client remains available for callers with responsive streaming I/O:

```sh
node tools/penta-agent/native.mjs session
```

Send one initialization line containing `{"server":"http://127.0.0.1:PORT","secret":"SEAT_SECRET"}`.
After `{"status":"initialized"}`, send one explicit request per line:

```json
{"method":"observe"}
```

Later requests omit credentials and return the ordinary response directly.
For example, `{"method":"choose","ticket":"REVISION:a7","requestId":"move-1","waitMs":25000}`.
Requests execute serially. An individual request error returns `{"error":"..."}`
and leaves the client running; invalid initialization ends it. Each input line
is limited to 1 MiB. EOF exits without closing the match host.

For a local agent terminal, start `stty -echo && node tools/penta-agent/native.mjs session`
in a persistent PTY, then write JSON lines to that terminal session. Use the local
shell runtime that can reach the loopback host. `native.mjs` is a CLI entrypoint;
programmatic callers import `nativeCommand` from `native-client.mjs`.

### One pilot turn per interaction

Prefer `{"method":"next"}` for the initial decision and `interact` thereafter.
Each interaction contains typed proposals decided by the pilot, optionally
followed by an explicit yield policy:

```json
{"method":"interact","requestId":"move-1","revision":"CURRENT_REVISION","choices":[{"proposal":{"action":{"type":"PlayLand","card":12,"playOptionId":0}}}],"yield":{"until":[{"type":"nextStep","step":"BeginningOfCombat"},{"type":"opponentActs"}]}}
```

Construct the proposal from the visible state, printed rules, and action context.
Answer a pending decision with `{decision: ID, options: [IDs]}`; preserve option
order. `choices: []` with a yield installs only that pass instruction. New
information or unresolved decisions require another pilot choice.

A proposal contains `action` and optional `payment`. Omit payment to allow the
engine's planner to pay for that chosen action. To select mana sources yourself,
provide `payment: {funding: [...]}`. Each ordered funding entry contains an
`ActivateManaAbility` action, optional exact `manaUnits`, and optional `answers`
(arrays of option IDs for funding decisions). `payment.announcements` similarly
answers payment announcement decisions. `payment.manaUnits` selects the exact
final pool units. `payment.contributions` identifies non-mana contributions by
`source`, `kind` (`convoke`, `delve`, `improvise`, or `waterbend`), and `symbol`.
An explicit payment uses only the funded pool and selected contributions; an
empty funding list spends the current pool without choosing more sources.
For mana in any combination, supply `combination: {blue: 1, green: 1}`; it
determines the internal representative color, so `color` can be omitted.

The host validates the whole request shape and yield before mutation, executes
the accepted action prefix, then installs the yield only if the entire intended
batch succeeded. A new own non-priority decision, completion, pause, or partial
batch leaves the yield unapplied. `host.receipt.yieldInstallation` reports the
result and reason; `yieldAtAcceptance` records the policy before automatic passes.
The policy can remain armed while the opponent acts. Identical retries return
the same receipt without repeating choices or installing another policy.

The session submits once with a 25-second wait, then renews read-only waits
internally until the seat is ready, complete, or paused. Local `maxWaitMs`
defaults to and is capped at 60000; zero disables waiting. A bounded timeout or
read failure returns `sessionWait` and preserves the committed receipt. Use
`next` to continue waiting. A failed initial mutation may be uncertain: recover
its receipt or retry identical arguments before choosing again.
These are session operations; one-shot `interact` accepts only host `waitMs`.
The initial read/submission uses the ordinary transport timeout rather than the
local wait deadline, so a long-poll response at the budget boundary can still
deliver its committed receipt. The local deadline bounds subsequent read-only
waits; it can therefore be exceeded by the initial request's transport overhead.
The session caches the public catalog after its first successful retrieval.

`{"method":"sessionStats"}` returns local `callsByMethod`, `errors`, `outputBytes`,
and `elapsedMs` without contacting the host. Counts include dispatched failures
but exclude `sessionStats`; bytes count serialized stdout before that stats
response, including initialization and errors. Credentials remain in memory.
The client generates no request IDs, chooses no actions, and performs no retries.
It additionally reports command durations, underlying HTTP counts and durations,
request bytes, reserialized response JSON bytes, and internal wait renewals.
`inputGapMs` measures time from writing a ready response to receiving the next
request; it includes model, tool, and scheduling delays and is not model-only
reasoning time. Organizer status/export contain lifetime host `commandMetrics`,
excluding authority lock waiting and HTTP long waits. Byte counts are token-cost
proxies, not tokenizer or billing measurements.

The session prints each new or changed card's printed summary, then marks
identical repeats using only `definition`, `reference`, `printed: null`, and
`printedPreviouslySent: true`. Unchanged action ability metadata uses
`{source, abilitiesPreviouslySent: true}` after successful delivery. Every
visible source remains listed; changed, new, or reappearing sources include
full records. Current zones and pending decision options remain complete inline.
Decision option collections include typed `defaults` for repeated empty metadata.
Interpret each option as `{...defaults, ...row}`; IDs, labels, order, nonempty
values, and unknown fields are preserved. The position reference retains the
exact canonical observation. This avoids repeating null card/trigger metadata
hundreds of times for choices such as creature types.
After context loss, send
`{"method":"observe","full":true}` to reprint rules and event history. `full` is a local boolean
option, stripped before the host request; it also works on `wait` and mutations.
A fresh client session starts with no remembered rules. One-shot commands always
remain self-contained. Only successfully written inline rules count as delivered;
paged or unavailable rules do not.

Native v2 and v3 sessions also retain an explicit event delivery cursor. The host's
optional `afterEvent` argument returns the seat's ordered stream as
`events:{after,next,items}`. It is independent of manual action resets and request
idempotency; identical events remain distinct. Invalid cursors fail before a
mutation. Each seat can retrieve only its own stream.

The session sends its last delivered cursor through all internal waits and
advances it after writing the resulting packet. `position.updates` contains
every newly delivered event inline; `host.eventDelivery` reports the boundaries.
While waiting without a position, `host.eventDelivery.items` carries the events.
An explicit `full:true` resets the cursor to zero. The raw `receipt` operation
returns its receipt and optional `events` envelope without a v2 position.
Receipt updates remain unchanged, so receipt events may also appear in the
delivery stream. One-shot and legacy clients retain snapshot updates unless
they explicitly request a cursor.

## Play and reconnect

Use the same invocation pattern for each method. All inputs include `server`
and `secret`; the examples below show the additional fields.

| Method | Additional fields | Purpose |
| --- | --- | --- |
| `observe` | none | Current seat-visible proposal state |
| `discover` | none; use `presentation: "pilot-v2"` for tickets | Explicit optional legal-action menu |
| `wait` | `waitMs` up to 25000 | Bounded wait for a seat view |
| `choose` | `ticket`, `requestId`, optional `options` | Submit one displayed choice |
| `play` | `revision`, `choices`, `requestId` | Submit an explicitly decided batch |
| `interact` | `revision`, `choices`, `requestId`, optional `yield`, `waitMs` | Submit choices and an explicit continuation together |
| `yield` | `revision`, `until`, `requestId`, optional `maxPasses`, `waitMs` | Pass priority until any requested condition matches |
| `cancelYield` | `requestId` | Cancel this seat's retained yield instruction |
| `concede` | `requestId` | Concede |
| `receipt` | `requestId` | Recover a submitted request's outcome |
| `transcript` | optional `offset`, `limit` up to 100 | Recover seat-visible history |
| `note` | `text` | Retain a seat note in host memory |
| `catalog` | none | Public canonical card catalog |
| `inspect_ref` | `reference`, optional `offset`, `limit` up to 100 | Inspect a current compact-view reference |

Responses distinguish `ready`, `waiting`, `complete`, and `paused`. At `ready`,
the default `pilot-v3` supplies complete seat-visible state without exhaustive
legal-action menus. Submit `{proposal: {action: ...}}` through `interact` or
`play`. For example, after deciding both attacks:

```json
{
  "revision": "CURRENT_REVISION",
  "requestId": "attack-1",
  "choices": [
    {"proposal": {"action": {"type": "DeclareAttacker", "attacker": 12, "defender": {"type": "player", "seat": "p2"}}}},
    {"proposal": {"action": {"type": "DeclareAttacker", "attacker": 18, "defender": {"type": "player", "seat": "p2"}}}}
  ]
}
```

Cast proposals use `{type: "CastSpell", card, choices: {playOptionId, ...}}`.
Omitted or null `choices` supplies the same empty defaults; it never selects
required targets or modes on the pilot's behalf.
Choose modes with `choices.modeIds`, X with `choices.x`, and targets with
`choices.targetSelections: [{slotId, targets, amounts?}]`. Targets are
`{type: "player", seat}` or `{type: "permanent" | "card" | "spell", objectId}`.
Do not duplicate these selections as flattened `targets`, `modeIds`, or `x`
fields on the cast. Omitted modes, targets, cost objects, and additional costs
are empty; omitted X and play-option ID are zero and omitted alternative cost
is none. Optional fields also accept null with these defaults. Ability
proposals use `source` and the supplied `ability` identity, with optional
`targetSelections`, `costObjects`, `modeIds`, and `x`.
Cast `costObjects` also names objects spent on costs: sacrifices, discards,
exiled evidence, and similar payments. It is separate from spell targets.
Each visible object's `actionAbilities` entry includes `grantedCastCosts` with
the current `playOptionId`, `alternativeCostId`, symbolic mana cost, and grant
description. These identities include temporary flashback and battlefield
alternatives that do not appear in the printed catalog. They are metadata;
the engine still validates timing, permission, targets, and payment.
The entry's `playPermission` separately describes an applicable exile/graveyard
permission, its base cost, surcharges, and restrictions. For example, airbend
reports `cost: {kind: "alternativeMana", manaCost: "{2}"}` and uses the default
null `alternativeCostId`, rather than a granted-cost ID. These are permission
terms, not a final cost quote or proof of legal timing; cost modifiers still apply.

Pending decisions retain their explicit IDs, option IDs, ordering and selection
bounds. Submit `{decision: ID, options: [IDs]}` including `[]` for an empty
selection. The exported `NativePlayChoice` schema describes the wrappers.
Malformed proposals fail locally or return a structured
`host.receipt.rejection: {code, path, message}` from the host. An accepted batch
prefix remains committed; a rejected proposal does not partially fund or cast.
Local schema failures retain the offending field path and allowed action tags.
Combat declarations can automatically finish once no further declaration choice
remains. A redundant finish in a batch is rejected after the accepted attacks or
blocks, and its yield is not installed; consult `actionContext` before continuing.

Optional discovery is explicit: call `discover` with
`presentation: "pilot-v2"`, then `choose` with the returned ticket and a new
`requestId`. Legacy `{action}`, single `{index}`, and decision wrappers remain
available. Copy discovered exact actions unchanged; the client removes only
observation index metadata. Native batches do not accept tickets or bare actions.
`host.receipt.updates` in the typed view preserves this seat's new events and automatic inspections
across the entire accepted prefix, including when the next view is `waiting`.
Read these alongside the current observation; retries return the same receipt.
Supply `waitMs: 25000` with a submission to wait for the next own decision in
the same request. Waiting never chooses another action.

Choose a new request ID for each logical mutation and retain its exact method
and arguments until the outcome is known. After a timeout or lost output,
resend the identical request or query `receipt`; never invent a replacement ID.
The host retains receipts in memory across client restarts. The client does
not generate IDs, retry automatically, or remember the last submission.

To reconnect a pilot, keep the host running and reuse that seat's server URL
and secret. Resolve an uncertain submission first, then call `observe` and
page through `transcript` as needed. A new model process does not require a new
match or a new CLI daemon. `note` can retain a concise strategic reminder for
that seat, but model conversation context is not automatically transferred.
Stop the old pilot before handing control to its replacement; reusing a seat
secret does not revoke the old pilot's access.

Compact references are rebuilt from the current same-revision view. They expire
when the revision changes; call `observe` again after a stale-reference error.
Pass `presentation: "exact"` to receive the host's raw response, including the
canonical observation. A catalog failure leaves the typed position available
with unavailable printed references and a warning. If rendering itself fails
after a committed move, the adapter returns the exact response and a
presentation warning, preserving the receipt.

## Typed pilot presentation

The native client defaults to `pilot-v3`. Read its JSON Schema without credentials:

```sh
node tools/penta-agent/native.mjs schema
```

The [composed schema](../tools/penta-agent/pilot-v3-contract.mjs) includes the
[typed proposal union](../tools/penta-agent/proposal-schema.mjs).
[Generated TypeScript](../tools/penta-agent/pilot-v3.d.ts) is refreshed with
`node tools/penta-agent/generate-pilot-v3-types.mjs`.

`position.actionInterface` is `proposal-v1`. Collections use
`{items,total,reference}`; visible zones, updates, rules, and pending decision
options are complete inline. Zone objects retain canonical IDs, definitions,
characteristics, costs, stack details, and all other seat-visible fields.
`position.actionContext` identifies the current decision and casting offers.
`position.actionAbilities` maps visible object IDs to effective ability origins,
rule clauses, target slots, and modes. For selected modes, concatenate their
target slots in selection order after the base ability slots, assigning sequential
slot IDs; a mode-local slot ID is not the final combined slot ID.
`position.facts` retains other observed state. Printed rules retain catalog
rules fields, including card parts and play options; artwork and printing lists
remain available through the rule reference. No `choices` collection is produced. Hidden information stays
redacted by the engine. `position.reference` retrieves this seat's proposal
observation; provenance references retain protocol and fingerprint information.

`host.currentYield` is the current policy; `host.receipt.yieldAtAcceptance` is
its immutable acceptance snapshot. Read receipt updates as well as current
position updates, including while waiting. Sessions can omit already delivered
printed rules and per-source ability metadata with explicit cached markers;
`full:true` restores them. One-shot views always include full metadata.

Explicit `pilot-v2`, `pilot-v1`, and `decision-v1` presentations still render
menu observations returned by `discover`. Ordinary state-only responses fall
back to exact output for those presentations; they never trigger implicit
menu discovery. `presentation: "exact"` returns the host envelope unchanged.
The browser and hosted adapters retain their existing contracts.

## Conditional priority yields

The usual instruction is "pass until a particular phase or step, unless the
opponent does something." Conditions are a typed OR list, not a natural-language
program. For example, call `yield` with the current revision:

```json
{
  "server": "http://127.0.0.1:PORT",
  "secret": "SEAT_SECRET",
  "requestId": "p1-yield-17",
  "revision": "CURRENT_REVISION",
  "until": [
    {"type": "nextStep", "step": "End", "activeSeat": "either"},
    {"type": "opponentActs"}
  ],
  "waitMs": 25000
}
```

This passes ordinary priority until the next end step begins or an opponent
action interrupts it, whichever happens first. It stays active while the other
pilot plays and across client reconnects. It returns control at the next legal
decision for this seat; it cannot interrupt resolution or grant priority.

| Condition | Meaning |
| --- | --- |
| `nextStep` | Stop on a future entry into `step`, optionally on `self`, `opponent`, or `either` player's turn |
| `opponentActs` | Stop for an opponent's visible spell, ability, land, mana activation, or combat declaration |
| `opponentCastsSpell` | Stop specifically when the opponent casts a spell |
| `nonManaAction` | Stop when this seat has a legal alternative other than passing or activating a mana ability |

`nextStep` uses the canonical names `Upkeep`, `Draw`, `PrecombatMain`,
`BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`,
`EndOfCombat`, `PostcombatMain`, `End`, and `Cleanup`. Use `PrecombatMain`,
`BeginningOfCombat`, or `PostcombatMain` for the corresponding phase boundary.
The current occurrence does not count: requesting `End` during an end step
targets a later end step. A skipped step does not count as an entry.

Add `{"type":"nonManaAction"}` to the example when the pilot also wants to
reconsider as soon as another legal action is available. This condition requests
legal-action discovery and can be expensive; phase/event conditions avoid it. Explicit payment
entry counts as a non-mana action; the host does not guess whether a payment
will enable a useful play. If this condition already matches, yielding stops
without passing. `opponentActs` includes mana, so pilots wanting a narrower
interruption should select `opponentCastsSpell` explicitly.

Every non-priority decision requiring a pilot stops the yield; attackers,
blockers, targets, payments, and other selections still belong to the pilot.
The engine continues to advance uniquely determined continuations, such as
finishing an empty attack declaration. The instruction expires at a game
boundary and after `maxPasses` automatic passes (default 64,
maximum 256), or can be stopped with `cancelYield`. Conditions are evaluated
using only the yielding seat's visible information. The host retains the
stop reason and the seat's updates, and records each actual pass for replay.
Yielding does not change the engine's classification of forced actions.

## Lifetime, isolation and debug export

All match state, receipts, transcripts and notes are volatile host memory.
Closing the client does not close the host. A hard host crash, process exit,
or machine restart loses the live match; this is not a durable restart service.
Keep the original host alive for reconnects.

The organizer can call `status`, `export`, and `close`. Export before closing
when a debug record is wanted:

```sh
node tools/penta-agent/native.mjs export - > native-match-export.json
```

Supply the organizer's server and secret through stdin. This explicitly saved
debug bundle can contain information hidden from players; do not give it to
pilots during play or treat it as a sanitized public transcript. Export is not
a promise of an import/resume facility. Call `close` with the organizer secret
when the match is no longer needed.

An unexpected engine failure restores the last committed position and pauses
play. The organizer can still export the accepted command prefix, last good
observations, failing command, and bounded diagnostics. Completed matches also
stay available until explicit close. Transcripts retain seat-visible positions,
choices, receipts and notes. Proposal observations omit the large derived
reconstruction checkpoint; explicit legacy discovery still provides it.

Check that a saved bundle reproduces its committed position with:

```sh
target/release/penta-host replay native-match-export.json
```

Replay requires the same simulation fingerprint and native replay version. It
compares both final seat observations and reports the failed command separately;
it does not automatically retry that command. The private seed is encoded as a
decimal string so JSON tools preserve its full precision. Exports omit credentials.

Seat credentials select the host's seat-specific observations and operations;
the organizer is separate from the pilots. This is not a hardened sandbox:
same-user agents with unrestricted shell, process inspection or access to each
other's tool transcripts may bypass intended secrecy. For enforced isolation,
restrict pilots to seat-bound tools and prevent access to organizer credentials,
other pilots' inputs, and host memory. Separate model prompts alone provide an
honor-system boundary.

See the [manual proposal evaluation](native-proposal-evaluation.md) for measured
engine latency, model-game results, and remaining discovery paths.
