# Repository instructions

This file is canonical for every agent. `CLAUDE.md` imports it rather than
restating it, so put repository guidance here and leave that file alone.

Skills live in `.agents/skills/<name>/`, and `.claude/skills/<name>/SKILL.md`
symlinks to the same file so Claude Code discovers them too. A skill body is
therefore read through two directories: write paths inside one from the
repository root, never relative to the file. `tests/agent_skills.rs` enforces
all of this.

## Optional reference material

Magic rules and card data may be available in an optional development cache
under Git's common directory. One cache is shared by every linked worktree in
the clone. Use `$query-magic-references` for efficient read-only access to its
generated Scryfall SQLite index and `$refresh-magic-references` to locate,
inspect, migrate, populate, or rebuild the cache.

Locating the cache, querying it, checking `status scryfall-index`, and viewing
`lock-status` are low-friction read operations; they do not justify a refresh.
Fetch, index, and migration commands mutate shared clone state and require
explicit human approval. Run them rarely: only when material is missing or
corrupt, the required database schema is unavailable, or the current task
genuinely requires fresher source data. Do not refresh merely because a new
worktree was created. If the cache is absent, stale for an irrelevant purpose,
or unavailable, continue with appropriate authoritative online sources.

Treat `refresh.lock` metadata as diagnostic information. The kernel lock is
authoritative; never delete or bypass the lock merely because its recorded
owner appears stale. Do not commit or ship downloaded reference payloads.

Treat both reference skills as maintained development tooling. When repeated
work exposes a missing field, relationship, index, or query pattern, update the
refresh builder, both skills, and the documented schema together, then rebuild
and validate the cache. Avoid expanding them for isolated one-off questions.

## Deployment

The web client is deployed to <https://penta.lacker.workers.dev>. Hosted
games and the bot registry are gated behind `HOSTED_GAMES`, and the engine
self-check behind `ENGINE_SELF_CHECK`. Do not claim from memory whether a deployment sets it -- one
request settles it:

```bash
curl -s -o /dev/null -w '%{http_code}\n' https://penta.lacker.workers.dev/_bots
```

## Design doctrine

Prefer the [design doctrine](docs/design-doctrine.md) and
[card extension boundaries](docs/implementing-cards.md#extension-boundaries)
in the project documentation. Treat them as the project's direction of travel
and the default starting point for implementation decisions, not as purity
gates. When a task benefits from a different boundary, keep the supported
behavior correct, the coverage honest, the exception contained, and the
tradeoff explicit.

## Card-set source organization

In each printed set module under `src/card/sets/y<year>/`, keep canonical
`CardRecord` declarations in natural collector-number order and keep `CARDS`
in exactly the same order. Keep `ADDITIONAL_PRINTINGS` in natural order by the
collector number of each printing in that module's set, including in
reprint-only modules whose `CARDS` registry is empty. Compare numeric portions
numerically (`8`, `8a`, `8b`, `16`), not lexicographically.

Introduce every declaration with an ordinary one-line comment in the exact
form `// LEA 230 — Ankh of Mishra`: uppercase set code, collector number
verbatim, an em dash, and the canonical `CardRecord` name. Ordinarily the
header immediately precedes the declaration. For an identity in the inline Old
School audit, put its explanation on the next line in the exact form
`// Audit: blocked — Needs ...`, using `partial` or `metadata-only` instead when
applicable. A partial or metadata-only audit line immediately precedes its
declaration; a blocked header and audit line stand alone at that identity's
collector position. Keep all identity headers in natural collector order. Put
every nonempty `ADDITIONAL_PRINTINGS` entry on its own line and end it with a
comment in the exact form `// LEB 233`: the target printing's uppercase set
code and collector number, with no card name. Empty additional-printing
registries need no comments.

The comment identifies the canonical printing in that module's set, even when
the chosen presentation art intentionally comes from another printing. Move
card-local helpers with the definition they support. This convention does not
apply to `src/card/sets/tokens.rs`, whose synthetic objects have no single
printed set or collector number.

## Performance awareness

Follow the [performance guide](docs/performance.md). Treat performance as
review context, not a merge gate. Most changes need only a qualitative impact
note; measure when evidence could change a design or review decision, using
`$profile-engine-performance` for the repository's reproducible comparison
workflow. Stop once the relevant question is answered rather than turning a
routine check into an open-ended optimization task.

## Protocol versioning

A branch or pull request containing one or more incompatible protocol changes
must set the protocol version to exactly one greater than the target branch's
version. Do not bump it again for additional incompatible changes or
intermediate commits in the same branch or pull request. After rebasing,
re-check the target branch's protocol version and adjust if it changed.

Treat the [bot guide](docs/bots.md) and [changelog](CHANGELOG.md) as part of
the protocol contract. A change to observation or catalog JSON, legal-action
contents or meaning, decision shapes, a bot binding, or version semantics must
update both documents in the same branch or pull request. Check the guide's
examples against every affected binding rather than deferring documentation to
a follow-up. Keep the root `BOTS.md` compatibility symlink pointed at
`docs/bots.md`.

## Bug reports

Players file bugs from the game menu with the game's replay attached. To
work through them, follow [the bug-report workflow](docs/bug-reports.md):
list the open reports from the dev server's `/_bugs/list`, reproduce each
natively with the `replay_bug` example, fix, and resolve with a note naming
the commit.

## Validation

Use the root `Makefile` as the canonical entry point; `make help` lists the
available suites.

- During implementation, run the narrowest target or filtered test that
  exercises the changed behavior. Do not run the full gate after every edit.
- Run `make check-fast` at coherent checkpoints. It covers formatting, lints,
  normal Rust tests, and the fast browser-facing WASM suite without a
  production web build or the simulation-heavy tests.
- Run the relevant slow target when a change affects simulation, policy,
  auto-pass, combat progression, or another behavior covered by that suite.
- Run `make check` once the change is stable and ready to push or open as a PR.
  If bindings changed, also run `make check-bindings`; `make ci` runs every
  repository gate.
- Do not rerun an unchanged passing suite unless later edits could affect it.
  In the handoff, list the exact targets run and call out any deferred gate.
- For UI changes, command-line checks do not replace the visual verification
  below.

Map changed paths to the narrowest useful target before broadening:

- `src/game/**`, `src/card/**`, and core rules: `make test-engine-unit
  FILTER=<name>` or `make test-engine-integration FILTER=<name>`.
- `src/policy.rs` and policy behavior: `make test-policy FILTER=<name>`; add
  `make test-rust-slow` when the simulation sweeps are relevant.
- `src/protocol.rs`: `make test-engine-unit FILTER=protocol`, then
  `make test-wasm-rust` plus the matching browser contract suite when the
  exposed bridge can change. For `wasm/**`, start with the latter two targets.
- `web/app/**`: `make lint-web`, `make typecheck-web`, and the matching WASM or
  render target, followed by the required browser verification for UI changes.
- `bindings/penta-ffi/**` or `bindings/penta-py/**`: use the corresponding
  `make check-bindings-*` target; use strict `make check-bindings` before handoff.
- `Makefile`, `scripts/**`, or `.github/workflows/**`: `make lint-infra` and
  exercise the changed orchestration target. For `.github/dependabot.yml`,
  validate against GitHub's Dependabot 2.0 schema. Run `make doctor` when
  prerequisites are in question.

## UI changes

For every change that can affect the web interface:

1. Start or restart the local server from the current working tree. Confirm that
   the worktree-specific URL from `cd web && pnpm run dev:url` is served by that
   process; do not accept a fallback port or assume an older server picked up
   the change.
2. Open the rendered application in a browser and inspect it visually. A
   successful build, DOM snapshot, or HTTP response is not sufficient.
3. Check at least a 1280×720 laptop viewport. Verify that important content is
   visible and readable, with no unintended clipping, overlap, off-screen
   controls, or inaccessible horizontal overflow.
4. Exercise enough UI state to display the changed component. For game-table
   changes, check cards in hand and cards on the battlefield when applicable.
5. Take a fresh screenshot after the final code change and inspect it before
   reporting completion.

Keep the verified local server running for the user unless they ask otherwise.
