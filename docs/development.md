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
prefer `card::catalog()`, `card::cards::*`, and the functions in `decks`.

For architecture and implementation boundaries, read the
[design doctrine](design-doctrine.md), [engine architecture](engine.md), and
[card implementation guide](implementing-cards.md).

## Local setup

The repository pins its Rust version, components, and WASM target in
`rust-toolchain.toml`. Rustup installs that toolchain automatically. Web work
also requires the Node and pnpm versions declared by the repository, plus the
matching `wasm-bindgen` CLI.

Start by checking the environment:

```sh
make doctor
```

`make doctor` reports setup problems without installing tools implicitly. It
checks Rust, Node, pnpm, the WASM target and generator, binding prerequisites,
ShellCheck, and Actionlint.

## Validation workflow

During implementation, run the narrowest target that exercises the changed
behavior:

```sh
make test-engine-unit FILTER=mana_burn
make test-engine-integration FILTER=mountain_casts
make test-web-wasm-pacing PATTERN='auto-pass'
make typecheck-web
make lint-web
```

Use `make check-fast` for a broad checkpoint without the production web build
or simulation-heavy sweeps. Once a change is stable and ready for a push or PR,
`make check` runs the complete engine and web gate.

Binding changes can use `make check-bindings-c` or
`make check-bindings-python` while iterating, followed by strict
`make check-bindings`. `make check-bindings-available` is the explicit
best-effort local variant when Python is unavailable; `make ci` never skips a
repository gate.

All Cargo validation uses committed lockfiles. Clippy runs pedantic with
`-D warnings`, so the pinned toolchain makes lint changes deliberate rather
than dependent on a contributor's local compiler.

Rust source files are limited to 1,000 physical lines so modules retain clear
conceptual boundaries and remain practical to review and merge. The sole
semantic exception is a direct card-set file matching
`src/card/sets/y<four ASCII digits>/*.rs`; those files intentionally organize
the card corpus by set. There is no file-specific allowlist. The guard
discovers every Cargo root in the repository, including the standalone
`bindings/penta-py` crate, and scans its `src`, `tests`, `examples`, `benches`,
and `build.rs` sources. Run it directly with:

```sh
make test-source-file-sizes
```

Detailed path-specific validation and UI verification requirements live in
the canonical repository instructions in `AGENTS.md`.

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

The report derives repository-wide complete, partial, and metadata-only
definitions from the built-in catalog. It also gives a mutually exclusive
complete/partial/metadata-only/blocked partition for each supported set corpus,
using the inline `// Audit:` rows to identify cards that are not yet cataloged.
Its output is intentionally not checked in: the source declarations remain
authoritative, so adding a card does not require updating a second copy of the
totals.

## Performance work

Performance investigation has its own reproducibility and evidence rules. Use
the [performance guide](performance.md) rather than treating suite duration or
profiler sample percentages as benchmark results.
