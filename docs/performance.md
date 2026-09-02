# Engine performance

Measure the native Rust engine first. The default workload drives deterministic
Random/Random games through the normal `observe` and `apply` loop; the cheap
policy keeps policy selection from obscuring engine costs. Profile the WASM
adapter or browser only when a consumer-specific problem remains after the
native engine is measured.

Performance work is advisory and evidence-driven, not a merge gate. Most
changes need only a qualitative expected-impact note. Measure when the result
could inform a design or review decision, preferably once at a coherent
checkpoint. The workflow is meant mainly to reveal accidental multiplier-sized
regressions such as 2× or 4×; roughly 20% slower is ordinarily review context,
not a reason to optimize or block. These examples calibrate judgment rather
than define thresholds.

The [prepared engine](prepared-engine.md) is the optional optimization layer
over declarative card definitions. Its guide describes compiler boundaries,
fallback requirements, differential testing, and how to add another lowering.
Use the workflow below to establish whether a lowering improves whole-game
throughput; preparation is not justified merely by moving work into a second
implementation.

## Benchmark elapsed time

Use [Hyperfine](https://github.com/sharkdp/hyperfine) on the normal release
binary:

```sh
cargo install --locked hyperfine
make benchmark-engine PROFILE_GAMES=2000 PROFILE_SEED=1 \
  BENCHMARK_OUTPUT=target/profiles/engine-before-benchmark.json
```

The target prints one untimed run so deterministic outcome counts can be
checked, then runs Hyperfine with warmups and repeated measurements. Override
`BENCHMARK_WARMUP` and `BENCHMARK_RUNS` when needed.

Compare the exact game count, seed, build profile, and machine before and after
a change. Prefer one same-session comparison when both binaries are available.
Use Criterion only after a hot path can be isolated as a stable in-process
benchmark; whole-game throughput remains the primary regression measure.

### Compare with main

For an advisory branch comparison, save a baseline for the exact commit named
by the selected local ref, then benchmark that binary and the current worktree
together in one Hyperfine session:

```sh
make benchmark-engine-baseline
make benchmark-engine-compare
```

The baseline target reuses a matching saved result and measures only when the
selected revision or relevant environment has changed. The comparison target
performs the same lazy check, so advancing local `main` selects its new commit
without switching the current worktree. Neither command fetches; set
`PERFORMANCE_BASELINE_REF=origin/main` when that is deliberately the local ref
to measure.

The shared release binary, deterministic outcome, metadata, and baseline
Hyperfine JSON live under
`$(git rev-parse --path-format=absolute --git-common-dir)/penta-performance-cache/layout-v1/`,
where linked worktrees in the clone can reuse them. Comparison JSON and its
metadata default to `target/profiles/engine-main-compare*.json`. The cache keys
the selected revision, machine, build environment, effective Cargo
configuration, compiler and tool versions, workload, seed, and measurement
settings. Main and the current worktree still run together rather than treating
an older saved mean as same-session evidence.

A deterministic outcome mismatch is reported because timing then includes
changed game paths as well as implementation cost. State that limitation when
it applies. Once a routine comparison rules out a scale-changing regression,
report the result and stop. Repeated tuning for another modest improvement
belongs in an explicit optimization task or a concrete user-visible need.

## Attribute CPU time

Use [Samply](https://github.com/mstange/samply) to locate CPU hotspots with the
Firefox Profiler call tree, flame graph, timeline, and source views. The
separate `profiling` build retains release optimization and adds symbols:

```sh
cargo install --locked samply
make profile-engine PROFILE_GAMES=20000 PROFILE_SEED=1 \
  PROFILE_OUTPUT=target/profiles/engine-before.json.gz
make profile-engine-open \
  PROFILE_OUTPUT=target/profiles/engine-before.json.gz
```

The profiler UI visualizes a native process capture; it does not profile
Penta's browser interface. For a broader sample covering every built-in deck
in both formats, use `make profile-engine-all`.

Branch captures, ad hoc Hyperfine exports, and allocation traces belong under
the ignored `target/profiles/` directory. The maintained main baseline is the
exception and is managed under Git's common directory. Samply releases that
store symbols in an adjacent `.syms.json` sidecar need that file kept beside
the capture.

Use sampled profiles to choose one optimization target. Profiler span and
sample weights support attribution; benchmark distributions establish speed.
Do not treat sample percentages as benchmark deltas.

## Measure allocations

Samples in allocator functions measure allocator CPU time, not allocation
counts, bytes, lifetimes, or peak heap use. When heap churn is the question,
run the same symbol-rich native workload under an allocation profiler:

```sh
make build-profile-engine

# macOS with full Xcode installed
xcrun xctrace record --template 'Allocations' \
  --output target/profiles/engine-allocations.trace --launch -- \
  target/profiling/penta-match --p1 random --p2 random \
  --deck1 Random --deck2 Random --games 2000 --seed 1

# Linux with Valgrind installed
valgrind --tool=dhat \
  --dhat-out-file=target/profiles/engine-dhat.out \
  target/profiling/penta-match --p1 random --p2 random \
  --deck1 Random --deck2 Random --games 2000 --seed 1
```

See Apple's [Instruments memory guidance](https://developer.apple.com/documentation/xcode/gathering-information-about-memory-use)
and the [Valgrind DHAT manual](https://valgrind.org/docs/manual/dh-manual.html)
for interpreting allocation results.

## Agent workflow

The shared `profile-engine-performance` skill provides the repository's full
agent workflow and optional schema-guarded attribution analyzer. Standard
profiler views remain the primary way to inspect captures. Profiling is opt-in
and is not part of CI; routine baseline comparisons do not record a Samply
capture.
