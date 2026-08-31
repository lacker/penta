# Development guide

This guide covers repository layout and the common contributor workflow. Use
the root `Makefile` as the canonical task catalog; `make help` lists every
target and `make doctor` reports missing prerequisites.

## Repository layout

- `src/card/` owns the card model, catalog, stable IDs, and corpus. Set modules
  under `src/card/sets/y<year>/<set>.rs` declare canonical cards and additional
  printings.
- `src/game/` owns the rules state machine, including decision, event, mana,
  observation, and test vocabulary.
- `decks/<format>/` contains built-in decklists as YAML. `src/decks.rs` embeds
  them so engine and browser builds need no runtime filesystem access.
- `wasm/` is the small Rust adapter used by the browser.
- `web/` contains the local browser client and its browser-facing tests.
- `bindings/penta-ffi/` and `bindings/penta-py/` expose the bot protocol to C,
  C++, Python, and other FFI consumers.
- `.agents/skills/` contains maintained repository tooling shared by supported
  coding-agent harnesses.

The original `poc` module remains as a compatibility facade. New code should
prefer `card::catalog()` and the functions in `decks`. The generated
`card::cards::*` constants remain available for native compatibility, but
authored rules should refer to Magic card names rather than numeric identities.

For architecture and implementation boundaries, read the
[design doctrine](design-doctrine.md), [engine architecture](engine.md), and
[card implementation guide](implementing-cards.md).

## Local setup

The repository pins its Rust version, components, and WASM target in
`rust-toolchain.toml`. Rustup installs that toolchain automatically. Web work
also requires the Node and pnpm versions declared by the repository, plus the
matching `wasm-bindgen` CLI.

For a new clone, or when prerequisites are in doubt, check the environment:

```sh
make doctor
```

`make doctor` reports setup problems without installing tools implicitly. It
checks Rust, Node, pnpm, the WASM target and generator, binding prerequisites,
ShellCheck, and Actionlint. It is a diagnostic, not a routine preflight.

Cargo uses incremental compilation for local development, test, release, and
profiling builds. The repository's Rust compiler wrapper also uses `sccache`
when it is installed and falls back transparently to `rustc` otherwise. The
wrapper sends incremental calls directly to rustc because sccache cannot cache
them; it caches non-incremental builds such as CI and explicit
`CARGO_INCREMENTAL=0 cargo ...` clean builds. Install the optional cache with
`brew install sccache`; `make doctor` reports whether it is available.

CI deliberately disables incremental compilation: its clean jobs use sccache,
and rustc outputs compiled with `-C incremental` are not cacheable by sccache.
Normal tests use the lighter `quick-test` profile. Ignored whole-game sweeps use
the release-optimized `simulation-test` profile through `make test-rust-slow`.
The normal Rust lane builds library and integration-test targets; zero-test
binary and example harnesses stay covered by the separate all-targets Clippy
job without adding link work to the test job.

## Validation workflow

Read-only investigation requires no validation. For a change, inspect
`git status --short` and the complete branch diff, then choose checks from the
behavior and executable inputs that changed. Use `make help` to discover the
available targets.

### Ownership and timing

In a shared worktree, designate one validation owner. Parallel agents may
inspect or edit independent areas, but they must not start competing Cargo,
Clippy, WASM, formatting, or preflight commands. Unless the owner explicitly
delegates one focused test, a child reports its changed paths and suggested
filters; returning work to the parent is internal integration, not a final
handoff.

Validation has four phases:

1. **Iteration:** run exactly one narrow filtered owning-lane target that gives
   useful feedback while implementing. Do not run a full package, workspace,
   `check-*`, or `preflight` target after each edit when a named test or filter
   covers the change. Broaden only when the change crosses a contract boundary
   or the focused target cannot exercise it. Do not add a compilation-only
   warmup; the test compiles what it needs.
2. **Integration:** after parallel edits land, the validation owner runs the
   focused owning-lane checks whose covered behavior changed.
3. **Content freeze:** once code and any rebase conflict resolutions are final,
   run each remaining relevant check once, followed by `make preflight` once
   before an external handoff or push.
4. **Remote:** let PR CI run the complete Rust, web, tooling, and binding gates.
   Rust lint and Rust tests are separate parallel jobs so Clippy's build is not
   on the test job's critical path. Assign one watcher rather than starting
   parallel polling loops.

Passing validation attaches to covered contents, not a commit hash or PR
metadata. Do not repeat a passing command after a metadata-only rewrite,
content-equivalent rebase, PR-body edit, or unrelated file change. Rerun it only
when its executable inputs or covered behavior changed.

Let a running build or test finish before starting another compiler command in
the same worktree. For long commands, prefer an initial yield around 30 seconds
and subsequent polls 30--60 seconds apart. If output tracking is lost, inspect
the existing process or terminal state before starting a replacement.

### Choose the owning lane

Start with the smallest target that exercises the behavior. Common examples:

```sh
make test-engine-unit FILTER=mana_burn
make test-engine-integration FILTER=mountain_casts
make test-web-wasm-pacing PATTERN='auto-pass'
make typecheck-web
make lint-web
```

- `src/game/**`, `src/card/**`, decks, and core rules use
  `make test-engine-unit FILTER=<name>` or
  `make test-engine-integration FILTER=<name>`.
- Policy behavior uses `make test-policy FILTER=<name>`. Add the native slow
  sweep only when the changed behavior is specifically exercised there.
- Protocol work starts with `make test-engine-unit FILTER=protocol`. Add a
  binding target only when its exported interface changes. Protocol JSON,
  versions, capabilities, or compatibility values exported through WASM also
  require `make test-wasm-rust` and the closest browser contract test.
- `wasm/**` uses `make test-wasm-rust`, then the closest browser contract suite
  when browser-visible behavior changes. Add `make typecheck-web` when generated
  TypeScript types or their consumers can change.
- `web/**` uses the applicable static check and closest unit, WASM, or render
  target. A change to rendered output or interaction also follows the
  [visual-verification checklist](../web/README.md#visual-verification).
- `bindings/penta-ffi/**` and `bindings/penta-py/**` use their corresponding
  `make check-bindings-*` target. Run both only for shared binding behavior. An
  engine field that makes `Game` lose `Send + Sync` is already guarded by the
  native `src/game/tests/thread_safety.rs` test.
- Agent guidance and skill entrypoints use `make test-agent-guidance`.
  Performance-tool implementation changes add `make test-profile-attribution`;
  Magic-reference implementation changes add `make test-magic-references`.
- `Makefile`, `scripts/**`, and workflows use `make lint-infra` plus the smallest
  changed orchestration target or a dry run of its dependency graph. Use
  `make doctor` only when prerequisites are in question. Validate
  `.github/dependabot.yml` against GitHub's Dependabot 2.0 schema.
- Documentation-only changes use `git diff --check` and verification of changed
  links and commands; do not run an unrelated code suite.

Validate each part of a mixed change in its own lane. Native card definitions,
game rules, decks, and policies stay in native validation unless they also
change the WASM adapter, a browser-consumed contract, or web code. Ordinary
card/catalog/fingerprint changes carried by existing shapes do not cross that
boundary. Browser-visible registries and replay/protocol/capability values use
the one closest contract or replay test, not every browser suite.

### Final and broad checks

Run this once on final contents before an external handoff or push, whatever
paths changed:

```sh
make preflight
```

It sequentially runs `make fmt` and `make test-source-file-sizes`. The latter is
a dependency-free crate that enforces the 1,000-line Rust source-file limit
without building the engine. Direct card-set files are the semantic exception;
the checker discovers all Cargo roots and applies the repository rule.

Slow simulation sweeps are deferred to nightly CI. Run `make test-rust-slow` or
`make test-web-wasm-slow` locally only when changed behavior is specifically
covered there or the user asks. New Rust tests marked `#[ignore]` and suites in
`WEB_WASM_SLOW_SUITES` are picked up by the nightly lane.
When nightly fails, start with the narrow rerun command printed for each failing
sweep rather than rerunning the whole lane.

Aggregate targets such as `make check-fast`, `make check`, `make check-rust`,
`make check-web`, `make check-tooling`, `make check-bindings`, and `make ci` are
not routine PR prerequisites. Use one when explicitly requested, when most of
its lane changed, or when its orchestration is under test. Reproduce an
individual CI failure with the focused child target, not its parent aggregate.

### Rebases

For a rebase, fetch the target branch once, record that commit, and treat it as
the snapshot for the request. After resolving conflicts, rerun only checks
affected by the resolutions or overlapping upstream changes, then push without
refetching the target branch merely to chase a newer snapshot. Continue to use
remote feature-branch inspection and `--force-with-lease` safety after a
rewrite. Do not chase target-branch movement during local validation or CI. A
newer base SHA, `BEHIND` label, or pending CI is not by itself a reason to
rebase again; do so only when requested or when an actual conflict or merge
requirement blocks the PR. Passing checks attach to tested contents, so
metadata-only rewrites, content-equivalent rebases, and PR metadata edits do
not invalidate them.

## Web development

The browser runs the Rust engine through the WASM adapter. Generated bindings
live under the ignored `web/app/wasm` directory and are kept current by the
development, build, and test workflows.

The full setup, worktree-specific port behavior, test targets, and deployment
workflow are documented in the [web client guide](../web/README.md).

## Policy and rules diagnostics

The built-in `RandomPolicy` samples seeded non-concession actions.
`HandcraftedPolicy` is a deterministic, inspectable baseline with simple
mulligan, casting, targeting, combat, mana, and card-specific heuristics.

Run the reproducible seat-swapped policy gauntlet with:

```sh
cargo run --release --bin policy_sanity
```

Run the broader rules audit with:

```sh
cargo run --release --bin rules_audit
```

The audit plays Random/Random and both seatings of Handcrafted/Random across
built-in matchups and seeds. After every action it checks public-state
agreement, hidden hand sizes, unique legal actions, decision ownership, and
completed-game behavior. Pass a different seed count as the final argument for
a longer soak run.

## Catalog coverage

Print the current implementation and inline capability-gap totals with:

```sh
make catalog-report
```

The report derives repository-wide declarative, custom, partial, and
metadata-only definitions from the built-in catalog. Declarative and custom
partition the complete definitions: custom means at least one clause uses a
keyed custom implementation or card-owned resolver, while declarative means
every clause uses shared declarative execution. The report also includes
blocked cards from inline `// Audit:` rows for each set-based format and
uncataloged cards for every cube. Pass `CATALOG_REPORT_ARGS=--verbose` to list
the individual fixed-pool cards in each status. The report output is
intentionally not checked in: the source declarations remain authoritative, so
adding a card does not require updating a second copy of the totals or names.

Each complete definition on the frozen legacy custom allowlist also has a canonical
`// Audit: custom — Needs ...` annotation so declarative migration work remains
visible while browsing or searching collector-ordered set sources. Execution
metadata remains authoritative for classification; source-organization tests
keep the annotations in exact agreement with the derived custom set and reject
any addition outside the shrinking allowlist. Remove the annotation and its
allowlist entry when every part and modal clause has moved to declarative
execution.

## Performance work

Performance investigation has its own reproducibility and evidence rules. Use
the [performance guide](performance.md) rather than treating suite duration or
profiler sample percentages as benchmark results.
