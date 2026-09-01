#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mode="${1:-record}"

if [[ $# -gt 1 ]]; then
    echo "usage: $0 [benchmark|record|record-all|open]" >&2
    exit 2
fi

case "$mode" in
    benchmark | record | record-all | open) ;;
    *)
        echo "usage: $0 [benchmark|record|record-all|open]" >&2
        exit 2
        ;;
esac

if [[ "$mode" == "record-all" ]]; then
    default_profile_output="target/profiles/engine-all.json.gz"
else
    default_profile_output="target/profiles/engine.json.gz"
fi
profile_output="${PROFILE_OUTPUT:-$default_profile_output}"
if [[ "$profile_output" != /* ]]; then
    profile_output="$repo_root/$profile_output"
fi

if [[ "$mode" == open ]]; then
    if [[ ! -f "$profile_output" ]]; then
        echo "profile not found: $profile_output" >&2
        echo "record it first with: make profile-engine" >&2
        exit 1
    fi
    if ! command -v samply >/dev/null 2>&1; then
        echo "samply is required to open an engine profile" >&2
        echo "install it with: cargo install --locked samply" >&2
        exit 1
    fi
    exec samply load "$profile_output"
fi

is_u64() {
    local value="$1"
    local normalized
    local suffix

    [[ "$value" =~ ^[0-9]+$ ]] || return 1
    normalized="${value#"${value%%[!0]*}"}"
    [[ -n "$normalized" ]] || normalized="0"
    if (( ${#normalized} < 20 )); then
        return 0
    fi
    if (( ${#normalized} > 20 )); then
        return 1
    fi
    # Bash arithmetic is signed, so compare the leading digit first and only
    # parse a suffix that is known to fit in signed 64-bit arithmetic.
    [[ "${normalized:0:1}" == "1" ]] || return 1
    suffix="${normalized:1}"
    [[ "${suffix:0:1}" != "9" ]] || return 1
    (( 10#$suffix <= 8446744073709551615 ))
}

resolve_binary() {
    local binary_name="$1"
    local cargo_profile="$2"
    local profile_args=()
    local build_messages
    local executable

    if [[ "$cargo_profile" == "release" ]]; then
        profile_args=(--release)
    else
        profile_args=(--profile "$cargo_profile")
    fi
    if ! build_messages="$(
        cd "$repo_root"
        CARGO_INCREMENTAL=0 cargo build --locked "${profile_args[@]}" --bin "$binary_name" \
            --message-format=json-render-diagnostics
    )"; then
        echo "could not build $cargo_profile workload: $binary_name" >&2
        return 1
    fi

    executable="$(
        printf '%s\n' "$build_messages" \
            | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' \
            | tail -n 1
    )"
    if [[ -z "$executable" ]]; then
        echo "Cargo did not report the $cargo_profile workload path: $binary_name" >&2
        return 1
    fi
    printf '%s\n' "$executable"
}

workload_args=()
if [[ "$mode" == "benchmark" || "$mode" == "record" ]]; then
    profile_games="${PROFILE_GAMES-4000}"
    profile_seed="${PROFILE_SEED-1}"
    if ! is_u64 "$profile_games" || [[ ! "$profile_games" =~ [1-9] ]]; then
        echo "PROFILE_GAMES must be a positive 64-bit integer, got: $profile_games" >&2
        exit 2
    fi
    if ! is_u64 "$profile_seed"; then
        echo "PROFILE_SEED must be an unsigned 64-bit integer, got: $profile_seed" >&2
        exit 2
    fi

    binary_name="penta-match"
    workload_description="$profile_games deterministic games"
    workload_args=(
        --p1 random
        --p2 random
        --deck1 Random
        --deck2 Random
        --games "$profile_games"
        --seed "$profile_seed"
    )
else
    binary_name="policy_sanity"
    workload_description="the both-format policy gauntlet"
fi

if [[ "$mode" == "benchmark" ]]; then
    benchmark_runs="${BENCHMARK_RUNS-10}"
    benchmark_warmup="${BENCHMARK_WARMUP-1}"
    benchmark_output="${BENCHMARK_OUTPUT-}"
    if ! is_u64 "$benchmark_runs" || [[ ! "$benchmark_runs" =~ [1-9] ]]; then
        echo "BENCHMARK_RUNS must be a positive integer, got: $benchmark_runs" >&2
        exit 2
    fi
    if ! is_u64 "$benchmark_warmup"; then
        echo "BENCHMARK_WARMUP must be a non-negative integer, got: $benchmark_warmup" >&2
        exit 2
    fi
    if ! command -v hyperfine >/dev/null 2>&1; then
        echo "hyperfine is required to benchmark engine throughput" >&2
        echo "install it with: cargo install --locked hyperfine" >&2
        exit 1
    fi

    benchmark_binary="$(resolve_binary "$binary_name" release)"
    if [[ ! -x "$benchmark_binary" ]]; then
        echo "release workload not found: $benchmark_binary" >&2
        exit 1
    fi

    hyperfine_args=(--warmup "$benchmark_warmup" --runs "$benchmark_runs")
    if [[ -n "$benchmark_output" ]]; then
        if [[ "$benchmark_output" != /* ]]; then
            benchmark_output="$repo_root/$benchmark_output"
        fi
        mkdir -p "$(dirname "$benchmark_output")"
        hyperfine_args+=(--export-json "$benchmark_output")
    fi

    echo "Verifying deterministic output for $workload_description"
    "$benchmark_binary" "${workload_args[@]}"
    printf -v benchmark_command '%q ' "$benchmark_binary" "${workload_args[@]}"
    echo "Benchmarking $workload_description with $benchmark_runs measured run(s)"
    hyperfine --shell=bash "${hyperfine_args[@]}" "$benchmark_command"
    exit
fi

if ! command -v samply >/dev/null 2>&1; then
    echo "samply is required to record an engine profile" >&2
    echo "install it with: cargo install --locked samply" >&2
    exit 1
fi

samply_record_help="$(samply record --help)"
if [[ "$samply_record_help" == *"--presymbolicate"* ]]; then
    symbolication_flag="--presymbolicate"
elif [[ "$samply_record_help" == *"--unstable-presymbolicate"* ]]; then
    # Samply 0.13 writes a symbol sidecar; newer releases fold the same data
    # into the saved profile behind the stable spelling above.
    symbolication_flag="--unstable-presymbolicate"
else
    echo "this Samply version cannot save symbol information with a profile" >&2
    exit 1
fi

profile_binary="$(resolve_binary "$binary_name" profiling)"
if [[ ! -x "$profile_binary" ]]; then
    echo "profiling workload not found: $profile_binary" >&2
    exit 1
fi

mkdir -p "$(dirname "$profile_output")"

echo "Recording $workload_description to $profile_output"
if [[ "$mode" == "record" ]]; then
    exec samply record --save-only "$symbolication_flag" -o "$profile_output" -- \
        "$profile_binary" "${workload_args[@]}"
else
    exec samply record --save-only "$symbolication_flag" -o "$profile_output" -- \
        "$profile_binary"
fi
