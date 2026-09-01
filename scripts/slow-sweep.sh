#!/usr/bin/env bash
# Runs the deferred sweeps -- the tier that plays whole games looking for
# regressions -- and, when one fails, prints the narrowest command that
# reproduces it.
#
# usage: slow-sweep.sh rust
#        slow-sweep.sh web <suite.mjs>...
#
# These take minutes, which is why they are not in the per-push gate. That
# makes the failure report the important part: whoever reads it is looking at
# a nightly log, disconnected from the change that broke it, and their first
# question is how to get the failure back locally without waiting out the
# whole sweep. So each target runs on its own, and a failure names the one
# command that reruns just it.
#
# The web suite list is passed in from the Makefile rather than repeated here,
# so `WEB_WASM_SLOW_SUITES` stays the single source of truth.
set -uo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root" || exit 1

mode="${1:-}"
shift || true

repro_commands=()
failed_any=0

# Collects the failing test names cargo listed under "failures:".
collect_cargo_failures() {
  local log="$1"
  local test_name
  while IFS= read -r test_name; do
    [ -n "$test_name" ] && repro_commands+=("make test-rust-slow FILTER=$test_name")
  done < <(
    awk '/^failures:$/ { collecting = 1; next }
         /^test result:/ { collecting = 0 }
         collecting && /^    [^ ]/ { print $1 }' "$log" | sort -u
  )
}

sweep_rust() {
  # label|cargo target selector
  local targets=(
    "engine library|-p penta --lib"
    "engine integration|-p penta --test engine"
    "policy|-p penta --test policy"
  )

  local entry label selector output_file
  for entry in "${targets[@]}"; do
    label="${entry%%|*}"
    selector="${entry#*|}"

    printf '\n== %s ==\n' "$label"
    output_file="$(mktemp)"

    # shellcheck disable=SC2086 # selector is a deliberate multi-word argument list.
    if cargo test --locked --profile simulation-test $selector -- --ignored 2>&1 |
      tee "$output_file"; then
      rm -f "$output_file"
      continue
    fi

    failed_any=1
    collect_cargo_failures "$output_file"
    rm -f "$output_file"
  done
}

sweep_web() {
  if [ "$#" -eq 0 ]; then
    printf 'slow-sweep.sh web: no suites given\n' >&2
    exit 2
  fi

  # One file at a time. Node would happily run them in parallel, but nothing
  # is waiting on this and serial output makes a failure attributable to a
  # suite without reading interleaved logs.
  local suite
  for suite in "$@"; do
    printf '\n== %s ==\n' "$suite"
    if (cd web && CI=true node --test "$suite"); then
      continue
    fi
    failed_any=1
    repro_commands+=("cd web && CI=true node --test $suite")
  done
}

case "$mode" in
  rust) sweep_rust ;;
  web) sweep_web "$@" ;;
  *)
    printf 'usage: slow-sweep.sh rust | slow-sweep.sh web <suite.mjs>...\n' >&2
    exit 2
    ;;
esac

if [ "$failed_any" -eq 0 ]; then
  printf '\nAll deferred sweeps passed.\n'
  exit 0
fi

printf '\n%s\n' "----------------------------------------------------------------"
printf 'The nightly sweep failed. To reproduce locally, run just the\n'
printf 'failing test rather than the whole sweep:\n\n'

if [ "${#repro_commands[@]}" -eq 0 ]; then
  # A target can fail without naming a test: a panic outside a test, or a
  # build error. Say so instead of printing an empty list.
  printf '  make nightly-sweep\n\n'
  printf 'No individual test was named above, so the failure is likely a\n'
  printf 'build error or a crash outside a test body. Read the target log.\n'
else
  for command in "${repro_commands[@]}"; do
    printf '  %s\n' "$command"
  done
  printf '\nEach command runs only the failing sweep.\n'
fi

printf '%s\n' "----------------------------------------------------------------"
exit 1
