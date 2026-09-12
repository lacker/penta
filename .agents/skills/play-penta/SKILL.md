---
name: play-penta
description: Play an assigned Penta seat through the hosted MCP interface, against a human or another bot. Covers connection, exact choices, references and recovery without gameplay advice.
---

# Play Penta

Use the Penta MCP tools directly. Before starting play, verify that the pilot
has callable `attach`, `play`, `choose`, `next`, `inspect_ref`, and `retry` tools.
If tools are missing, consult `docs/bot-sessions.md#run-locally`; a successful
transport probe does not establish discovery in this task. Report the missing
integration instead of constructing a per-move shell or Python relay.

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
again from the new view. A process restart loses local tickets; reattach.

Exact inspection and action forms are described in `docs/bot-sessions.md`.
Report engine or information discrepancies with visible position, revision,
ticket and receipt context; omit seat credentials.
