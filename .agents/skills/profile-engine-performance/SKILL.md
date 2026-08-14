---
name: profile-engine-performance
description: "Benchmark and profile Penta's native engine with reproducible deterministic workloads, Hyperfine wall-clock measurements, saved Samply CPU captures, and allocation-profiler guidance. Use when investigating engine throughput, CPU hotspots, CPU spent in allocator routines, real allocation churn, before-and-after performance, or the next optimization target. Do not compare different workloads, seeds, build profiles, or machines, and do not treat sample percentages as benchmark speedups."
---

# Profile Engine Performance

Measure the native Rust engine first. Use the web/WASM path only when evidence
points to consumer-specific overhead. Keep branch captures, symbol sidecars,
ad hoc benchmark exports, and allocation traces under ignored
`target/profiles/`; never commit them. The maintained `main` benchmark is the
exception: manage it only through the baseline targets, which store it under
Git's common directory for reuse by every linked worktree.

## Decide whether to measure

Performance awareness does not mean benchmarking every change. Keep these
commands out of the normal edit-test loop and do not run them after every
iteration. Most work needs at most a qualitative expected-impact assessment.
Measure when the evidence is likely to change a design or review decision,
such as work on a known hot path, a suspected regression, an explicit
optimization, or a meaningful correctness-versus-throughput tradeoff. Use
agent judgment, and prefer one comparison at a coherent checkpoint over
repeated measurements while the implementation is still moving.

For routine work, stop after that comparison rules out a scale-changing
regression. Do not rerun benchmarks, capture profiles, or alter an otherwise
sound design merely to recover another roughly 10% or other modest movement.
Iterate only when performance is explicitly the task, a concrete user-visible
requirement exists, or the first result reveals a multiplier-sized regression
likely to change the decision. A benchmark result does not by itself expand the
task into optimization work.

Do not benchmark solely to populate a pull-request section or handoff. "No
expected impact" and "Not measured" are valid conclusions. Baseline and
comparison commands are opt-in development tools, not validation targets.

## Fix the workload

Record the command, game count, seed, revision, build profile, and machine. Use
the deterministic Random/Random deck rotation for normal engine work. Its cheap
policies keep the workload focused on `Game::observe`, legal-action generation,
and `Game::apply`:

```sh
target/release/penta-match --p1 random --p2 random \
  --deck1 Random --deck2 Random --games 2000 --seed 1
```

Preserve the outcome counts as a determinism check. Use the broader `make
profile-engine-all` policy gauntlet only when deck or format breadth matters
more than focused engine attribution.

Install missing performance tools only with the user's approval. Recording may
require host process-inspection or performance-counter permission; request that
permission when the sandbox or operating system denies it.

## Benchmark wall time with Hyperfine

Use the normal release binary, outside the profiler, for speedup evidence:

```sh
make benchmark-engine PROFILE_GAMES=2000 PROFILE_SEED=1 \
  BENCHMARK_WARMUP=1 BENCHMARK_RUNS=10 \
  BENCHMARK_OUTPUT=target/profiles/engine-before-benchmark.json
```

The target prints one untimed run for the outcome check, then lets Hyperfine
perform warmups, repeated measurements, outlier reporting, and optional JSON
export. Repeat the exact command after the change with a distinct output path.
Prefer a direct same-session comparison when two saved binaries are available.
Use Criterion only for a hot path that can be isolated as a stable in-process
benchmark without distorting its inputs; keep whole-game throughput as the
primary regression measure.

## Use the shared main baseline

Use the advisory baseline workflow for selected branch comparisons:

```sh
make benchmark-engine-baseline
make benchmark-engine-compare
```

The baseline target ensures there is a matching result for the exact commit
named by the local `refs/heads/main`, reusing it when one already exists. The
comparison target performs the same lazy check, so it selects a new cache entry
when local `main` advances without repeatedly measuring an unchanged baseline.
Neither target fetches or switches the current worktree. Set
`PERFORMANCE_BASELINE_REF=origin/main` only when that is deliberately the local
tracking ref to measure.

The helper stores the main release binary, deterministic outcome, manifest,
and Hyperfine JSON under
`$(git rev-parse --path-format=absolute --git-common-dir)/penta-performance-cache/layout-v1/`.
Entries are separated by revision, machine, build environment, effective Cargo
configuration, compiler/tool versions, workload, seed, and measurement
settings. Branch comparison exports stay under ignored `target/profiles/` by
default.

For comparable evidence, let the comparison target run the saved main binary
and the freshly built branch binary in the same Hyperfine session. Do not
calculate a speedup from a new branch mean and an old saved main mean. Check the
reported game outcomes and toolchains; if either differs, describe that limit
because the delta may include different game paths or compiler behavior. Small
noisy movements may be inconclusive. Focus attention on multiplier-sized
runtime regressions, such as work becoming 2× or 4× slower; a roughly 20%
slowdown is ordinarily context rather than a reason to optimize. Those figures
are calibration examples, not pass/fail cutoffs, and workload frequency and
user-visible impact still matter. Baseline tracking is review context, not a CI
or validation gate, and it does not justify recording a routine Samply CPU
capture.

## Capture and inspect CPU samples

Use the optimized symbol-rich build for CPU attribution. Do not overwrite the
baseline. Do not capture samples merely because a benchmark was run or to
explain a modest delta that does not warrant action; report that context and
stop. Capture only when further investigation is in scope and attribution is
likely to guide it:

```sh
make profile-engine PROFILE_GAMES=20000 PROFILE_SEED=1 \
  PROFILE_OUTPUT=target/profiles/engine-20000-seed1-before.json.gz
make profile-engine-open \
  PROFILE_OUTPUT=target/profiles/engine-20000-seed1-before.json.gz
```

Inspect the standard Firefox Profiler call tree, flame graph, timeline, and
source views first. Samply 0.13 writes symbols to an adjacent
`.json.syms.json` file; keep it beside the `.json.gz` capture.

Use absolute sample-weight changes first and shares as context. Profiler span
includes sampling overhead and is not elapsed-time evidence.

### Optional headless attribution

Use the repository analyzer only when terminal tables, caller attribution, or
machine-readable JSON materially help an agent workflow:

```sh
PROFILE_ANALYZER=.agents/skills/profile-engine-performance/scripts/profile_attribution.py
python3 "$PROFILE_ANALYZER" summary \
  target/profiles/engine-20000-seed1-before.json.gz --top 15
```

Repeat `--caller-of SUBSTRING` for immediate engine callers. Add `--json` for
stable output. The analyzer is a schema-guarded convenience layer over
Samply's Firefox processed-profile format, not a replacement for the standard
viewer. If it rejects a newer schema, open the capture normally and update the
analyzer with a representative fixture before extending support.

Interpret its sections distinctly:

- Raw leaf samples show what executed at the sample instant.
- Attributed self assigns native leaves to the nearest Penta frame on the
  sampled stack.
- Inclusive weight counts a function at most once per sample and shows stack
  presence, not exclusive time.
- Native-leaf attribution exposes which engine callers led to allocator,
  platform-memory, or kernel CPU work.

## Measure allocations with an allocation profiler

Allocator leaf samples mean CPU time observed in allocator routines. They do
not measure allocation counts, bytes, lifetimes, or peak heap use. When those
are the question, build the symbol-rich workload and run the exact same command
under Instruments Allocations on macOS or Valgrind DHAT on Linux:

```sh
make build-profile-engine

# macOS; requires full Xcode
xcrun xctrace record --template 'Allocations' \
  --output target/profiles/engine-allocations.trace --launch -- \
  target/profiling/penta-match --p1 random --p2 random \
  --deck1 Random --deck2 Random --games 2000 --seed 1

# Linux
valgrind --tool=dhat \
  --dhat-out-file=target/profiles/engine-dhat.out \
  target/profiling/penta-match --p1 random --p2 random \
  --deck1 Random --deck2 Random --games 2000 --seed 1
```

## Optimize and compare

Once measurement has identified a worthwhile optimization and the
implementation is stable enough to compare, change one well-supported hotspot
at a time. Preserve observable ordering, identity, deterministic outcomes, and
other semantics covered by the path. Run the narrowest relevant tests during
implementation, then the focused validation required by `AGENTS.md` before
handoff.

Capture the same CPU workload after the change and optionally compare it
headlessly:

```sh
make profile-engine PROFILE_GAMES=20000 PROFILE_SEED=1 \
  PROFILE_OUTPUT=target/profiles/engine-20000-seed1-after.json.gz
python3 "$PROFILE_ANALYZER" compare \
  target/profiles/engine-20000-seed1-before.json.gz \
  target/profiles/engine-20000-seed1-after.json.gz --top 15
```

The capture does not record `PROFILE_GAMES` or `PROFILE_SEED`; verify them from
the commands and filenames. Attribute speedup only from comparable Hyperfine
measurements. Report deterministic outcomes, benchmark distributions and
delta, supporting profile changes, allocation metrics when measured, current
hotspots, exact tests, and ignored artifact paths.
