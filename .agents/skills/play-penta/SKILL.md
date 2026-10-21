---
name: play-penta
description: Start or join a Penta game and play an assigned seat through the session API. Handles local server startup, compact choices, batching, waiting and recovery against a human or another bot.
---

# Play Penta

## Native host route

When given a native `penta-host` server URL and seat secret, use
`node tools/penta-agent/native.mjs METHOD -` with JSON on stdin containing
`server`, `secret`, and the method's arguments. Follow `docs/native-matches.md`.
This route uses no hosted connection handle, shared agent daemon, or credential
files. Do not run the hosted `up`/`attach` commands for a native assignment.

For repeated play, use the maintained memory-only local session. Start
`node tools/penta-agent/session-rpc.mjs serve -` with `{server, secret}` on
stdin, then close stdin. Keep this process running and retain `clientServer`
from its startup reply. For each request, run the finite command
`node tools/penta-agent/session-rpc.mjs call -` with
`{clientServer, secret, request: {method, ...args}}` on stdin. Use
`exec_command` with an initial yield of 30000 ms; it returns early when the
request command exits. Poll only while that finite process is still running.
This avoids persistent PTY collection delays while retaining session context.
Use the local shell runtime that can reach the loopback host.

Repeated rules use minimal rows containing `definition`, `reference`,
`printed: null`, and `printedPreviouslySent: true`. The session also delivers
new seat-visible events using an explicit cursor; identical events remain
distinct. Unchanged ability metadata uses `{source,abilitiesPreviouslySent:true}`;
new, changed, or reappearing sources include full records. Every visible source
remains listed. Current zones and pending decision options remain complete. After context compaction
or lost tool output, send `{"method":"observe","full":true}` to restore rules
and event history. `sessionStats` returns calls, bytes, timings, and errors.
Send `{"method":"shutdown"}` to close only your local session after play.
The JSON-lines `native.mjs session` route remains available for streaming
callers; avoid long fixed PTY output-collection intervals.

For normal native play, start with `{"method":"next"}` and then use one
`interact` per decision. Native output defaults to `pilot-v3` with
`position.actionInterface: "proposal-v1"`. Read complete seat-visible zones,
printed rules, `position.actionContext`, `position.actionAbilities`, and any
pending decision options. Construct your move; no exhaustive action menu is
needed. Supply `requestId`, current `revision`, and
`choices: [{proposal: {action: YOUR_TYPED_ACTION, payment?: YOUR_PAYMENT}}]`.
Answer pending decisions with `{decision: ID, options: [IDs]}`; retain order.
Decision option rows inherit `position.decision.options.defaults`; overlay each
row on those defaults. Every option ID and label remains inline. Mulligan actions
are `KeepHand` and `TakeMulligan`; `next` accepts `maxWaitMs`, not `waitMs`.

In `first-to-two-wins`, follow `position.situation.match.stage` between games.
Follow the private decision prompt. When asked for the next main deck, select
all its option IDs, including retained cards; `Library` marks current main-deck copies and `OutsideGame` sideboard
copies. Your full main-deck and sideboard rules are available in the packet.
After both submissions, answer the losing seat's play/draw decision. Keep the
same host and seat secret, and stop only when the packet is `complete`.

Cast actions use `{type:"CastSpell",card,choices:{playOptionId,modeIds?,x?,
targetSelections?,alternativeCostId?,additionalCostIds?},costObjects?}`. Target
selections use `{slotId,targets:[{type:"player",seat} or
{type:"permanent"|"card"|"spell",objectId}],amounts?}`. Omitted arrays are empty,
X is zero, and alternative cost is none. Do not add redundant flattened cast
fields. Omitted `choices` uses those empty defaults, without choosing required
targets or modes. Ability actions use `source`, the provided `ability` identity, and
optional `targetSelections`, `costObjects`, `modeIds`, and `x`. For selected
modes, concatenate their target slots after the base ability slots in selection
order and assign sequential slot IDs; mode-local IDs are not combined IDs.
Cast `costObjects` names resources sacrificed, discarded, or exiled for costs
(including collect evidence), separately from targets. Inspect an object's
`grantedCastCosts` for runtime alternative-cost IDs such as granted flashback.
Its `playPermission` describes an applicable exile/graveyard permission and cost.
Airbend's `alternativeMana` cost applies with the default/null alternative-cost
ID; it is not an entry in `grantedCastCosts`. Timing and final payment still apply.

Omitting payment delegates payment planning for your chosen action. When mana
sources matter, supply `payment:{funding:[{action:ACTIVATE_MANA_ABILITY,
manaUnits?:[IDs],answers?:[[OPTION_IDS]]}],announcements?:[[OPTION_IDS]],
manaUnits?:[IDs],contributions?:[{source,kind,symbol}]}`. Funding is ordered;
explicit payment does not silently select more sources. Use
`combination:{blue:1,green:1}` instead of a separate color when splitting mana
from one ability; otherwise supply `color`. Contributions use
`convoke`, `delve`, `improvise`, or `waterbend`. Read `docs/native-matches.md`
and `node tools/penta-agent/native.mjs schema` for the complete typed contract;
`tools/penta-agent/pilot-v3.d.ts` contains generated types.

An optional `yield:{until:[...],maxPasses:64}` records your explicit decision to
pass those windows. Empty choices with a yield are supported. The session
submits once and renews read-only waits up to 60 seconds (`maxWaitMs`, local
only), retaining the original receipt. Read `host.receipt.yieldInstallation`:
a partial batch or new own decision prevents installation. Use `next` after a
wait timeout; never resubmit a committed move merely to wait. A rejected
proposal reports `host.receipt.rejection` with code, path and message; a batch
can have a committed prefix. Retain the exact mutation until its outcome is
known and retry identical arguments after uncertain output, or query `receipt`.
Combat declarations can auto-finish after the last eligible attacker/blocker.
Avoid blindly appending a finish action; a redundant finish rejects only that
suffix and prevents installing its yield. Inspect the returned action context.

Use `host.currentYield` for current policy and `yieldAtAcceptance` for the
acceptance snapshot. Read `host.receipt.updates` alongside `position.updates`;
waiting events appear in `host.eventDelivery.items`. After context loss call
`observe` with `full:true` to restore printed rules and events. Use `inspect_ref`
for referenced details. No per-move explanation is needed.

If you need optional legal-action discovery, explicitly call `discover` with
`presentation:"pilot-v2"`, then use `choose` with the displayed ticket and a
new `requestId`, or copy the exact action into `{action:...}`. Legacy menu views
are opt-in; normal native play does not discover or paginate exhaustive menus.

For "pass until phase X unless the opponent acts", call `yield` with the current
`revision`, a new `requestId`, and an OR list such as
`until: [{type:"nextStep",step:"End"},{type:"opponentActs"}]`.
Use `PrecombatMain`, `BeginningOfCombat`, or `PostcombatMain` for those phase
boundaries. `nextStep` means a future entry, not the current occurrence;
`activeSeat` optionally selects `self`, `opponent`, or `either`.
`opponentActs` includes mana activations. Use `opponentCastsSpell` for a narrower
interrupt, or add `nonManaAction` to stop when another legal action is available.
This is an explicit decision to pass the covered priority windows, not a change
to which actions are forced. Non-forced decisions still need the pilot.
Read `host.currentYield` for the stop reason; `cancelYield` with a new `requestId` cancels
the instruction. Yields expire at game boundaries and the requested pass limit
(`maxPasses`, default 64, maximum 256). Conditions and updates survive reconnects
while the host remains alive.

After model context loss, reuse the same seat secret, resolve any uncertain
move, call `observe`, and recover seat-visible `transcript` pages and notes.
Keep the host alive: state is volatile and a host crash loses the live match.
Only the organizer obtains both seats, exports private debug data, or closes
the host. Never inspect the other pilot's credentials, transcript or process.
This workflow is not a hardened sandbox for agents sharing unrestricted shell
access. When asked to start a native match, follow the native guide's binary
startup and distribute only each pilot's own credential.
Start each pilot with fresh context, not inherited organizer history. With
Codex subagents, use `fork_turns: "none"` and a brief containing the skill,
server URL, assigned seat, and only that seat's secret.

## Hosted route

The instructions below apply only to hosted room/connection assignments. Native
server/seat-secret assignments use the route above, including its batch shapes.

Use `node tools/penta-agent/cli.mjs COMMAND 'JSON'` from the repository root.
Arguments can also come from `@FILE` or `-` for stdin; use those for credentials.
The maintained client handles transport and presentation. All gameplay choices
remain yours. No MCP configuration, client restart or model API key is needed.
Use `--help` for command arguments and `docs/bot-sessions.md` for exact forms.

## Start or join

For a new local game, run `up`, then `options` to discover formats and decks.
`up` installs missing web packages and starts the existing web/WASM development
server. If it returns `starting`, call it again after inspecting the returned
log as needed; do not start a second build. Diagnose missing prerequisites from
that log and `web/README.md`. `PENTA_SERVER_URL` selects an existing deployment
and disables local server startup. Keep the same environment for every command.

Create a game with `start_match` and a unique `requestId`, `format`, `p1Deck`,
and `p2Deck`. Set `humanSeat` to `p1` or `p2` for a human opponent and open the
returned `humanUrl`. `matchMode` defaults to `first-to-two-wins`; use
`one-conclusion` when requested. Repeating the identical creation request
retrieves its saved result; do not invent another ID to recover lost output.

The organizer receives both seat credentials. Give each player only its own
`{room, token}`. When assigned an existing seat, skip creation and attach only
to that seat. `attach` defaults to `presentation: "decision-v1"`. Retain the
returned `connection`. The local session process persists across CLI commands;
`status` lists connection handles after context loss. Do not read its private
recovery files to inspect another player's credentials or game information.

## Play

At `ready`, read the current position, ordered updates, and available choices.
Each `{shared, rows}` table is self-contained: shared fields apply to every row.
IDs distinguish objects and option IDs are not array positions. Printed card
references can differ from current observed characteristics after effects.

Use `choose '{"ticket":"..."}'` for a concrete action. Decision tickets require
an explicit `options` array, including `[]` for an empty selection. Preserve
option IDs and their order. The engine advances truly forced continuations;
all remaining gameplay choices are yours. No per-move explanation is required.

Submit an already-decided group with `play`, supplying `connection`, `revision`,
and `choices: [{ticket: "..."}, {ticket: "..."}]`. Tickets must belong to the
current view. Batch multiple attackers or block assignments together. Exact
actions may be mixed in, including `{action: {type: "FinishDeclaringAttackers"}}`
or `FinishDeclaringBlockers` when you choose to commit the declaration. The
engine may already have advanced a forced finish: inspect `receipt.accepted`
and the returned state. Accepted prefixes stay committed.

A `BottomCards` ticket includes the whole mulligan-bottom group. Submit all
selected option IDs for one decision together. Split a batch when new
information or an unresolved choice is needed; batching never licenses making
that choice automatically. `play` and `choose` return the next view: use it
directly when ready instead of calling `next` again.

Use `inspect_ref` with `{reference, offset}` for omitted details; follow
`nextOffset` until the needed entries are available. References are frozen to
their view. Use `next` with `{connection}` when waiting, and
`{connection, full: true}` for context refresh or expired references. Calls wait
up to 25 seconds. Full refreshes resend card text within the inline budget;
remaining text is inspectable. Stop at `complete`.

## Recover

After an uncertain submission or lost CLI result, use `retry` with
`{connection}`. It retains the exact resolved body and request ID, including
for a batch or a successful move whose output was lost. Repeating an identical
retained `choose` ticket and options also recovers its receipt. Resolve the
uncertainty before another move; do not change the submitted options.
After a definite stale refusal, refresh and choose again from the new view.

If the local session process restarts, saved connections and retry bodies
survive. Recover any outstanding move with `retry`, then refresh with
`next(full: true)` because ordinary tickets and references expire on restart.
An uncertain room creation whose credentials never reached the client is
reported explicitly; stop and explain it rather than silently dealing again.

`stop` ends the session process and leaves the game server running. Ordinary
play does not require it. Report engine or information discrepancies with
visible position, revision, ticket and receipt context; omit seat credentials.
