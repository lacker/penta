---
name: play-penta
description: Play an assigned Penta seat through the hosted MCP interface, against a human or another bot. Covers local startup, connection, exact choices, references and recovery without gameplay advice.
---

# Play Penta

## Connect and recover

Use the Penta MCP tools directly. This repository's `.codex/config.toml`
configures the local adapter for a trusted worktree; no plugin or per-worktree
global registration is needed. The adapter and the game server are separate
processes. Before play, verify that this task has callable `attach`, `play`,
`choose`, `next`, `inspect_ref`, and `retry` tools.

If setup is incomplete, follow `docs/bot-sessions.md#run-locally` and
`docs/bot-sessions.md#startup-and-recovery`. Check the worktree and effective
configuration (`git rev-parse --show-toplevel`, `codex mcp get penta --json`),
Node/Codex versions, and pinned adapter dependencies. Start the assigned local
game server using `web/README.md` when needed; preserve an explicitly assigned
remote server. Routine dependency installation, local startup, diagnostics,
and adapter reconnection are part of getting ready to play.

Distinguish missing configuration, adapter startup failure, stale tool discovery,
and an unreachable game server before choosing a repair. A missing tool does
not establish that a plugin is missing. After updating adapter code or its
dependencies, restart the MCP connection through the host's supported control
and verify discovery again. Shell-launching `server.mjs` or passing the probe
does not add tools to this task. If the host exposes no reconnect control to
the agent, report the exact remaining host restart step after completing the
available checks; do not invent a restart tool or build a per-move relay.

Keep the assigned room and token across recovery. An adapter restart loses
connections, tickets, references, and retained retry requests; reattach to the
same seat and inspect the fresh state. Do not resubmit a pre-restart uncertain
move as a new command or create a replacement match. When the adapter is still
alive, resolve uncertain submissions with `retry` before considering restart.

## Play

Attach with only your assigned `{room, token}` and `presentation: "decision-v1"`.
Save the returned connection for waiting and recovery. The organizer creates
the match; the player does not need the other seat's credential.

At `ready`, read the current position, ordered updates, and available choices.
Each `{shared, rows}` table is self-contained: shared fields apply to every row.
IDs distinguish objects and option IDs are not array positions. Printed card
references can differ from current observed characteristics after effects.

Call `choose({ticket})` for a concrete action. Decision tickets require an
explicit `options` array, including `[]` for an empty selection. Preserve option
IDs and their order. The engine already advances truly forced continuations;
all remaining gameplay choices are yours. No per-move explanation is required.

Submit an already-decided group in one `play` call, using its connection and
revision plus `choices: [{ticket: "..."}, {ticket: "..."}]`. Tickets must belong
to the current view. This is the ordinary route for multiple attackers or block
assignments; do not require a model round trip per creature. Exact actions may
be mixed in, including `{action: {type: "FinishDeclaringAttackers"}}` or
`FinishDeclaringBlockers` when you want to commit the declaration. The engine
may already have advanced a forced finish: inspect `receipt.accepted` and the
returned state before trying anything else. Accepted prefixes stay committed.

A `BottomCards` ticket already includes the whole mulligan-bottom group.
Likewise, submit all selected option IDs for one decision together. Split a
batch when new information or an unresolved choice is needed; batching never
licenses filling that choice in automatically. `play` and `choose` return the
next view, so use it directly when ready instead of calling `next` again.
After an uncertain ticket batch, `retry` retains its exact resolved commands.

Use `inspect_ref({reference, offset})` for omitted details; follow `nextOffset`
until the needed entries are available. References are frozen to their view.
Use `next({connection})` when waiting, and `next({connection, full: true})` for
context refresh or expired references. A full refresh resends card text within
the inline budget; remaining text is still inspectable. Stop at `complete`.

After an uncertain submission, use `retry({connection})`. For `choose`, repeating
the identical retained ticket and options also recovers the receipt. Resolve it
before another move. Do not change a submitted ticket's options. After a definite stale error, refresh and choose
again from the new view. For an adapter restart, follow recovery above.

Exact inspection and action forms are described in `docs/bot-sessions.md`.
Report engine or information discrepancies with visible position, revision,
ticket and receipt context; omit seat credentials.
