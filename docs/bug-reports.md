# Bug reports

A bug report is a description plus the game's replay: the deal configuration
and every command applied since, which rebuild the reported board exactly.
Players file them from the game menu ("Report a bug"); they land in the
deployment's bug ledger, a Durable Object whose state lives with the
checkout's `.wrangler` directory in development.

## Filing

The game menu's form attaches the replay automatically -- a local game's
journal via `WebGame::replayJson`, a hosted game's room record. The replay
stamps its independent `replayVersion` and `simulationFingerprint`, with
the engine package and bot-wire versions retained as provenance. Nothing else
is collected beyond the page URL, and hosted room records never include either
seat credential.

An unfinished hosted game against an external opponent deliberately returns a
redacted room record with neither its seed nor command journal, because either
can disclose private choices. File after that game ends when the report needs
an exact replay; local and scripted-opponent games remain replayable while
they are live.

## The agent workflow

"Check the outstanding bugs and deal with them" is mechanical:

1. Start the dev server (`pnpm run dev` in `web/`; the worktree port comes
   from `pnpm run dev:url`).
2. `curl -s http://localhost:<port>/_bugs/list` — open reports, oldest first.
3. For each: `curl -s http://localhost:<port>/_bugs/<id>` and pipe it to the
   native harness, which replays the exact game and prints the reported
   board's snapshot:

   ```sh
   curl -s http://localhost:<port>/_bugs/<id> \
     | cargo run -p penta-wasm --example replay_bug
   ```

   From there it is ordinary debugging: the game is native, deterministic,
   and at the moment the player saw the problem. A replay-format or simulation
   mismatch is refused before commands run. A replay that passes those guards
   but fails midway is diagnostic too -- the failing position names where the
   recorded command no longer applies.
4. Fix, test, and resolve with a note:

   ```sh
   curl -s -X POST http://localhost:<port>/_bugs/<id>/resolve \
     -H 'content-type: application/json' \
     -d '{"resolution": "fixed in <commit>: <what was wrong>"}'
   ```

A replay is only guaranteed when its `replayVersion` is understood and its
`simulationFingerprint` matches; `replay_bug` refuses either mismatch by name.
`engineVersion` and `protocolVersion` remain useful provenance, but neither is
the exact replay guard. When the simulation has moved on, use a checkout whose
computed fingerprint matches the report to reproduce, then verify the fix by
playing the same commands as far as they still apply.

## Routes

Behind the `HOSTED_GAMES` flag, unauthenticated, single ledger per
deployment:

| Route | Method | Body | Returns |
| --- | --- | --- | --- |
| `/_bugs/report` | POST | `{description, replay, context?}` | `{id}` |
| `/_bugs/list` | GET | — | `{bugs: [{id, reportedAt, status, description, commands, resolution?}]}` |
| `/_bugs/<id>` | GET | — | the full report |
| `/_bugs/<id>/resolve` | POST | `{resolution}` | `{id, status}` |
