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

`protocolVersion` is the breaking epoch for canonical bot observation, action,
and catalog JSON. Bump it once relative to the target branch only when an old
consumer could misinterpret existing wire data: removing, renaming, or changing
the type or meaning of a key, tag, identifier, or index; or adding mandatory
vocabulary without a negotiated fallback. After rebasing such a branch,
re-check the target version and adjust it if necessary.

JSON objects are open-world. Do not bump the epoch for an optional member, a
documented open-enum value with a safe fallback, append-only catalog growth,
different legal-action membership expressed through existing shapes, a rules
fix, presentation text, browser state, a Rust API or event, or replay/checkpoint
encoding. Use named capabilities for additive facilities, the automatically
generated simulation fingerprint as a conservative rules/artifact guard,
Cargo SemVer for native APIs, and the dedicated replay or checkpoint format
version for those artifacts. Bump the replay format when the command journal's
envelope, commands, configuration, or interpretation changes; bump the
checkpoint format when its imported bookkeeping changes incompatibly. Classify
new string-enum values as open with a safe fallback or closed/capability-gated;
an unclassified mandatory value is a breaking change. Never derive stable wire
tags with Rust `Debug` formatting.

Treat the [bot guide](docs/bots.md) and [changelog](CHANGELOG.md) as part of the
contract. Update them when a bot wire contract, capability, exact-artifact
format, or versioning rule changes; ordinary card/rules changes need only the
appropriate behavioral documentation. Check examples against every affected
binding. Never add a test that hard-codes the current epoch or names a branch as
its owner. Keep the root `BOTS.md` compatibility symlink pointed at
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

- Choose local validation from the complete branch diff and the behavior it
  changes, not from the fact that work is ready for a push or PR. During
  implementation and before handoff, run the narrowest target or filtered test
  that exercises that behavior.
- Native card definitions, game rules, decks, and policies do not require web
  or WASM tests merely because the engine is compiled for the browser. Run
  browser-facing tests locally only when the change affects the WASM adapter,
  the shape or meaning of a browser-consumed contract, web code, or a bug that
  reproduces only there. Ordinary card additions, catalog IDs or counts, and
  generated fingerprint changes in existing shapes are not by themselves
  reasons to run web tests locally. Browser-visible format or deck registries,
  replay/protocol values, and capabilities do warrant the one closest contract
  or replay test when their consumed value changes, never the full web gate by
  default.
- Validate each part of a mixed change in its owning lane. Touching one web file
  does not promote otherwise native card or rules behavior into every browser
  suite; run the one web target that covers the browser-facing part.
- Run a slow test only when the changed behavior is covered by it, and use
  `FILTER` or `PATTERN` when available. Native simulation, policy, auto-pass,
  or combat work may justify native slow tests; it does not by itself justify
  slow browser pacing or combat suites.
- Aggregate targets are not routine PR prerequisites. This includes
  `make check-fast`, `make check`, `make check-rust`, `make check-web`,
  `make check-tooling`, `make check-bindings`, and `make ci`. Use one when the
  user explicitly requests that aggregate or a complete local CI mirror, when
  the whole covered lane is affected, or when its aggregate orchestration is
  itself failing. Reproduce an individual CI failure with its focused child
  target, not the containing aggregate. For validation-graph changes, exercise
  the smallest changed orchestration target or dry-run its dependency graph;
  do not widen to a parent aggregate merely because it contains that target.
  PR CI owns the complete Rust, web, tooling, and binding gates.
- Treat a rebase request as one bounded operation: fetch the target branch once
  at the start, record the fetched commit, and rebase onto that snapshot. After
  resolving conflicts, rerun only checks invalidated by the resolutions or by
  overlapping upstream changes, then push without refetching the target branch
  merely to chase a newer snapshot. Continue to inspect the remote feature
  branch and use `--force-with-lease` after a rewrite. Upstream movement during
  local validation or CI is expected; do not enter a fetch/rebase/validate
  loop. Rebase again only on a new user request or when an actual merge conflict
  or branch-protection/merge-queue refusal requires it. A newer base SHA, a
  `BEHIND` label, or pending CI alone is not such a requirement.
- Validation attaches to tested contents, not commit identity or PR metadata.
  A metadata-only commit rewrite, content-equivalent rebase, PR-body edit, or
  unrelated file change does not invalidate a passing check; rerun only checks
  whose executable inputs or covered behavior changed.
- If output tracking for a running validation command is lost, inspect the
  process or terminal state before starting the same target again. Do not run
  duplicate suites merely to recover missing tool output.
- In the handoff, list the exact local checks run and identify the full gates
  left to CI.
- For UI changes, command-line checks do not replace the visual verification
  below.

Map changed paths to the narrowest useful target before broadening:

- `src/game/**`, `src/card/**`, `src/deck*`, `decks/**`, and core rules: run
  `make test-engine-unit FILTER=<name>` or `make test-engine-integration
  FILTER=<name>`. Keep ordinary card-behavior validation in this native lane.
- `src/policy.rs`, `src/policy/**`, and policy behavior: run `make test-policy
  FILTER=<name>`; add `make test-rust-slow` when the simulation sweeps are
  relevant.
- `src/protocol.rs` and `src/protocol/**`: start with `make test-engine-unit
  FILTER=protocol`. Add a binding target only when its exported bot interface
  changes. Bot-only internals stay native; protocol JSON, versions,
  capabilities, or compatibility values exported through WASM also require
  `make test-wasm-rust` and the one closest browser contract test.
- `wasm/**`: run `make test-wasm-rust`, then the one matching browser contract
  suite when browser-visible behavior changes. Add `make typecheck-web` when
  generated TypeScript types or their use can change.
- `web/**`: run the static checks applicable to the changed files and the one
  nearest unit, WASM, or render target, followed by the required browser
  verification for UI changes. Do not broaden from one changed browser domain
  to every WASM suite.
- `bindings/penta-ffi/**` or `bindings/penta-py/**`: use the corresponding
  `make check-bindings-*` target. Run both only when shared binding behavior
  changes.
- `.agents/skills/profile-engine-performance/**`: run
  `make test-profile-attribution`.
- `.agents/skills/query-magic-references/**` or
  `.agents/skills/refresh-magic-references/**`: run
  `make test-magic-references` for implementation changes.
- `Makefile`, `scripts/**`, or `.github/workflows/**`: `make lint-infra` and
  exercise the changed orchestration target. For `.github/dependabot.yml`,
  validate against GitHub's Dependabot 2.0 schema. Run `make doctor` when
  prerequisites are in question.
- Documentation-only changes: run `git diff --check` and verify changed links
  and commands; do not run code suites without a specific dependency.

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
