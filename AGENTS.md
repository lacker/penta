# Repository instructions

This file is canonical for every agent. `CLAUDE.md` imports it rather than
restating it. Skills live in `.agents/skills/<name>/`; matching
`.claude/skills/<name>/SKILL.md` files point to the same entrypoints. Write
repository paths in skill bodies from the repository root so both harnesses
resolve them identically.

Keep this file limited to rules that apply broadly. Put task-specific detail in
the linked guide or skill so agents load it only when the task requires it.

## Working in this repository

- Preserve unrelated user changes and inspect the relevant existing code before
  editing. Treat a dirty worktree as user-owned unless proven otherwise.
- Use the [design doctrine](docs/design-doctrine.md) as the direction of travel,
  not a purity gate. Keep supported behavior correct, coverage honest,
  exceptions contained, and tradeoffs explicit.
- For an architectural or card-extension-boundary decision, also read the
  relevant focused guide before choosing an implementation.

## Task router

- **Cards, sets, decks, and mechanics:** read
  [implementing cards](docs/implementing-cards.md), including its mandatory
  inline-definition rule, source-order, coverage, and extension-boundary
  rules. Use `$query-magic-references` when exact printing or card data
  matters.
- **Protocol, WASM contracts, bindings, replay, or checkpoints:** read
  [compatibility boundaries](docs/interfaces.md#compatibility-boundaries), the
  relevant [bot versioning section](docs/bots.md#determinism-and-versioning),
  and only the affected migration entry in [the changelog](CHANGELOG.md).
- **Web, rendered UI, or deployment:** read [the web guide](web/README.md).
- **Performance:** treat it as review context, not a merge gate. Read
  [the performance guide](docs/performance.md) and use
  `$profile-engine-performance` when measurement could affect a decision.
- **Player bug reports:** when explicitly asked to process the queue, follow
  [the bug-report workflow](docs/bug-reports.md).
- **Validation tooling, uncertain lane selection, or contributor workflow:**
  consult only the relevant section of
  [the development guide](docs/development.md#validation-workflow).

The optional Magic reference cache is shared by every linked worktree. Querying
it, locating it, checking status, and inspecting lock metadata are read-only.
Fetching, indexing, or migrating it mutates shared clone state and requires
explicit human approval. The kernel lock is authoritative: never delete or
bypass it because recorded metadata looks stale, and never commit downloaded
reference payloads. Use `$refresh-magic-references` only for approved cache
maintenance or repair.

## Agent coordination

- Delegate only concrete, bounded work whose parallelism outweighs the added
  context and integration cost. Prefer a small number of independent agents,
  no nested delegation by default, minimal self-contained briefs, and one owner
  per file. When the harness supports history selection, pass no parent history
  or only the few relevant turns; do not forward full history by default.
- Designate one validation owner for a shared worktree. A child returning work
  to its parent is internal integration, not the external handoff that triggers
  final repository checks. Unless explicitly assigned a focused test, children
  should report changed paths and suggested filters instead of running Cargo,
  WASM builds, Clippy, formatting, source-size checks, or final preflight.
- Never run compile-heavy Cargo or WASM commands concurrently in one worktree.
  Let a running command finish, use an initial yield around 30 seconds, and poll
  at 30--60 second intervals rather than creating one-second model/tool loops.
  If output tracking is lost, inspect the existing process or terminal before
  starting a replacement.
- Use one long-lived remote CI watcher, reuse its session, and poll it at the
  same bounded cadence instead of launching repeated status queries. Report
  only state changes or the final result.

## Validation

Use the root `Makefile` as the canonical command catalog; `make help` lists its
targets. Read-only investigation requires no validation. For changes, inspect
the complete diff and run the narrowest owning-lane check. Consult the
[path-to-target map](docs/development.md#choose-the-owning-lane) only when the
right target is not evident. Run focused checks while they are useful during
iteration, then validate the final integrated contents once.

Passing checks attach to covered contents, not commit identity or PR metadata.
Do not repeat a passing command unless its executable inputs or covered behavior
changed. Validate each part of a mixed change in its own lane. Native card,
rules, deck, or policy work does not require browser/WASM validation unless it
also changes a browser-consumed contract, adapter, or web code.

After content freeze, the validation owner runs `make preflight` once before an
external handoff or push, regardless of changed paths. It sequentially checks
formatting and the repository-wide source-file size invariant. Documentation-
only changes add `git diff --check` and verification of changed links/commands,
but no unrelated code suite.

Slow sweeps and aggregate targets such as `make check-fast`, `make check`, and
`make ci` are not routine PR prerequisites. Run them only when the user requests
one, the changed behavior is specifically covered by it, most of its lane is
affected, or its orchestration is itself under test. PR CI owns the complete
Rust, web, tooling, and binding gates; nightly CI owns deferred sweeps.

For a requested rebase, fetch the target once, record that snapshot, and rebase
onto it. Rerun only checks invalidated by conflicts or overlapping upstream
changes, then push with lease safety without chasing later target movement.
Rebase again only on a new request or an actual merge requirement. Report the
exact local checks run and which full gates remain to CI.

## Rendered UI changes

When rendered output or interaction can change, follow the visual-verification
checklist in [the web guide](web/README.md): serve the current worktree at its
assigned URL, inspect the actual UI in a browser at the required viewport and
state, and review a fresh final screenshot. Keep the verified server running
unless the user asks otherwise.
