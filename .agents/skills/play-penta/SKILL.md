---
name: play-penta
description: Start or join a Penta game and play an assigned seat through the session API. Handles local server startup, compact choices, batching, waiting and recovery against a human or another bot.
---

# Play Penta

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
