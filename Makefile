FILTER ?=
PATTERN ?=
WEB_TEST_CONCURRENCY ?=
PROFILE_GAMES ?= 4000
PROFILE_SEED ?= 1
PROFILE_OUTPUT ?=
BENCHMARK_RUNS ?= 10
BENCHMARK_WARMUP ?= 1
BENCHMARK_OUTPUT ?=
PERFORMANCE_BASELINE_REF ?= refs/heads/main
PENTA_PERFORMANCE_CACHE_DIR ?=
override RUST_TEST_FILTER := $(value FILTER)
override TEST_PATTERN := $(value PATTERN)
export RUST_TEST_FILTER
export TEST_PATTERN
export PROFILE_GAMES
export PROFILE_SEED
export PROFILE_OUTPUT
export BENCHMARK_RUNS
export BENCHMARK_WARMUP
export BENCHMARK_OUTPUT
export PERFORMANCE_BASELINE_REF
export PENTA_PERFORMANCE_CACHE_DIR

WEB_WASM_CONTRACT_SUITE := tests/wasm-contract.suite.mjs
WEB_WASM_CASTING_SUITE := tests/wasm-casting.suite.mjs
WEB_WASM_COMBAT_SUITE := tests/wasm-combat.suite.mjs
WEB_WASM_PACING_SUITE := tests/wasm-pacing.suite.mjs
WEB_WASM_STATE_SUITE := tests/wasm-state.suite.mjs
WEB_WASM_FAST_SUITES := $(WEB_WASM_CONTRACT_SUITE) $(WEB_WASM_CASTING_SUITE) \
	$(WEB_WASM_COMBAT_SUITE) $(WEB_WASM_PACING_SUITE) $(WEB_WASM_STATE_SUITE)
# Node parallelizes test files, while the synchronous WASM sweeps within each
# file stay serial. Keep the longest pacing group first so it starts promptly.
WEB_WASM_SLOW_SUITES := tests/wasm-pacing-handover-slow.suite.mjs \
	tests/wasm-pacing-actions-slow.suite.mjs \
	tests/wasm-pacing-replay-slow.suite.mjs \
	tests/wasm-combat-slow.suite.mjs
WEB_ROOT_TESTS := $(patsubst web/%,%,$(filter-out web/tests/wasm-game.test.mjs,$(wildcard web/tests/*.test.mjs)))
WEB_FAST_ROOT_TESTS := $(filter-out tests/rendered-html.test.mjs,$(WEB_ROOT_TESTS))
WEB_TEST_CONCURRENCY_ARG = $(if $(strip $(WEB_TEST_CONCURRENCY)),--test-concurrency=$(WEB_TEST_CONCURRENCY))

define run_web_tests
	cd web && if [ -n "$$TEST_PATTERN" ]; then \
		CI=true node --test $(WEB_TEST_CONCURRENCY_ARG) --test-name-pattern="$$TEST_PATTERN" $(1); \
	else \
		CI=true node --test $(WEB_TEST_CONCURRENCY_ARG) $(1); \
	fi
endef

define run_rust_tests
	if [ -n "$$RUST_TEST_FILTER" ]; then \
		cargo test --locked --profile quick-test $(1) "$$RUST_TEST_FILTER" $(2); \
	else \
		cargo test --locked --profile quick-test $(1) $(2); \
	fi
endef

.PHONY: help doctor fmt fmt-rust fmt-python-binding \
	lint lint-rust lint-web lint-infra lint-infra-available lint-python-binding \
	test test-rust test-rust-full test-rust-slow \
	test-engine test-engine-unit test-engine-integration test-policy test-wasm-rust \
	test-profile-attribution test-magic-references test-rust-budget test-source-file-sizes \
	build-profile-engine benchmark-engine benchmark-engine-baseline benchmark-engine-compare \
	profile-engine profile-engine-all profile-engine-open \
	build-wasm build-web \
	test-web test-web-fast test-web-unit test-web-full \
	test-web-wasm test-web-wasm-full test-web-wasm-slow \
	test-web-wasm-contract test-web-wasm-casting test-web-wasm-combat \
	test-web-wasm-pacing test-web-wasm-state typecheck-web \
	test-web-render test-slow \
	check-fast check check-rust check-web \
	check-bindings check-bindings-available check-bindings-c check-bindings-python ci

help: ## List the available validation and build targets.
	@awk 'BEGIN { FS = ":.*## " } /^[a-zA-Z0-9_.-]+:.*## / { printf "  %-28s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@printf '\nOptional filters:\n'
	@printf '  FILTER=<substring>           Narrow a Rust test target.\n'
	@printf '  PATTERN=<regular-expression> Narrow a browser/WASM test target.\n'
	@printf '  WEB_TEST_CONCURRENCY=<count> Cap concurrent Node test-file processes.\n'
	@printf '\nEngine performance options:\n'
	@printf '  PROFILE_GAMES=<count>        Number of deterministic games to run.\n'
	@printf '  PROFILE_SEED=<number>        First deterministic game seed.\n'
	@printf '  PROFILE_OUTPUT=<path>        Saved Samply profile (defaults depend on workload).\n'
	@printf '  BENCHMARK_WARMUP=<count>     Hyperfine warmup runs (default: 1).\n'
	@printf '  BENCHMARK_RUNS=<count>       Hyperfine measured runs (default: 10).\n'
	@printf '  BENCHMARK_OUTPUT=<path>      Optional Hyperfine JSON export.\n'
	@printf '  PERFORMANCE_BASELINE_REF=<ref> Local main ref for shared baselines.\n'

doctor: ## Verify the local toolchain and exact generator versions.
	./scripts/doctor.sh

fmt-rust: ## Check formatting for the root Rust workspace.
	cargo fmt --all -- --check

fmt-python-binding: ## Check formatting for the standalone Python binding crate.
	cargo fmt --manifest-path bindings/penta-py/Cargo.toml -- --check

fmt: fmt-rust fmt-python-binding ## Check formatting for every Rust crate.

lint-rust: ## Lint every Rust workspace target and feature.
	cargo clippy --locked --workspace --all-targets --all-features -- -D warnings

lint-web: ## Lint the web client.
	cd web && CI=true pnpm lint

lint-infra: ## Statically check shell scripts and workflows; require both linters.
	./scripts/lint-infra.sh all

lint-infra-available: ## Same, but skip an infrastructure linter this machine lacks.
	./scripts/lint-infra.sh available

lint-python-binding: ## Lint the standalone Python binding crate.
	cargo clippy --manifest-path bindings/penta-py/Cargo.toml --locked --all-targets --all-features -- -D warnings

lint: lint-rust lint-web lint-infra-available ## Run engine, web, and infrastructure linters.

test-engine: ## Run the normal tests for the core engine package.
	$(call run_rust_tests,-p penta,)

test-engine-unit: ## Run core engine library tests, optionally filtered.
	$(call run_rust_tests,-p penta --lib,)

test-engine-integration: ## Run engine integration tests, optionally filtered.
	$(call run_rust_tests,-p penta --test engine,)

test-policy: ## Run policy integration tests, optionally filtered.
	$(call run_rust_tests,-p penta --test policy,)

test-wasm-rust: ## Run native unit tests for the Rust WASM adapter.
	$(call run_rust_tests,-p penta-wasm --lib,)

test-rust: ## Run normal Rust tests; simulation sweeps stay deferred.
	$(call run_rust_tests,--workspace --all-targets,)

test-rust-slow: ## Run only ignored Rust simulation sweeps.
	$(call run_rust_tests,--workspace --all-targets,-- --ignored)

test-rust-full: ## Run every normal and slow Rust test in one pass.
	cargo test --locked --profile quick-test --workspace --all-targets -- --include-ignored

test-source-file-sizes: ## Enforce the repository-wide Rust source-file size limit.
	cargo test --locked --profile quick-test --test source_file_sizes

# Seconds the Rust suite may spend *running*. Compilation is excluded: it is
# bounded by the job timeout and says nothing about whether a test got slow.
RUST_TEST_BUDGET_SECONDS ?= 120

test-rust-budget: ## Fail when the Rust suite runs longer than its time budget.
	cargo test --locked --profile quick-test --workspace --all-targets --no-run
	@start=$$(date +%s); \
	cargo test --locked --profile quick-test --workspace --all-targets -- --include-ignored; \
	status=$$?; \
	elapsed=$$(($$(date +%s) - start)); \
	echo "Rust tests ran in $${elapsed}s (budget $(RUST_TEST_BUDGET_SECONDS)s)"; \
	if [ $$status -ne 0 ]; then exit $$status; fi; \
	if [ $$elapsed -gt $(RUST_TEST_BUDGET_SECONDS) ]; then \
		echo "Rust tests exceeded their $(RUST_TEST_BUDGET_SECONDS)s budget." >&2; \
		echo "Profile the slow test rather than raising the budget by reflex." >&2; \
		exit 1; \
	fi

test-profile-attribution: ## Test the repository-local engine performance tooling.
	python3 -m unittest discover \
		-s .agents/skills/profile-engine-performance/tests -p 'test_*.py'

test-magic-references: ## Test the repository-local Magic reference tooling.
	python3 -m unittest discover \
		-s .agents/skills/refresh-magic-references/tests -p 'test_*.py'

build-profile-engine: ## Build the optimized engine workloads with profiling symbols.
	cargo build --locked --profile profiling --bin penta-match --bin policy_sanity

benchmark-engine: ## Benchmark deterministic native-engine throughput with Hyperfine.
	./scripts/profile-engine.sh benchmark

benchmark-engine-baseline: ## Ensure an advisory main baseline exists in Git's common directory.
	python3 ./scripts/benchmark_engine.py baseline

benchmark-engine-compare: ## Compare this worktree with the matching local-main baseline.
	python3 ./scripts/benchmark_engine.py compare

profile-engine: ## Record a deterministic engine CPU profile with Samply.
	./scripts/profile-engine.sh record

profile-engine-all: ## Profile the broader both-format policy gauntlet with Samply.
	./scripts/profile-engine.sh record-all

profile-engine-open: ## Open the saved engine CPU profile with Samply.
	./scripts/profile-engine.sh open

build-wasm: ## Build the release WASM module and generated bindings.
	./scripts/build-wasm.sh

test-web-wasm-contract: build-wasm ## Run browser contract and packaging tests.
	$(call run_web_tests,$(WEB_WASM_CONTRACT_SUITE))

test-web-wasm-casting: build-wasm ## Run browser casting and targeting tests.
	$(call run_web_tests,$(WEB_WASM_CASTING_SUITE))

test-web-wasm-combat: build-wasm ## Run fast browser combat tests.
	$(call run_web_tests,$(WEB_WASM_COMBAT_SUITE))

test-web-wasm-pacing: build-wasm ## Run fast browser priority and pacing tests.
	$(call run_web_tests,$(WEB_WASM_PACING_SUITE))

test-web-wasm-state: build-wasm ## Run browser state and event-log tests.
	$(call run_web_tests,$(WEB_WASM_STATE_SUITE))

test-web-wasm: build-wasm ## Run all fast browser-facing WASM suites.
	$(call run_web_tests,$(WEB_WASM_FAST_SUITES))

test-web-wasm-slow: build-wasm ## Run only slow browser-facing WASM sweeps.
	$(call run_web_tests,$(WEB_WASM_SLOW_SUITES))

test-web-wasm-full: build-wasm ## Run every browser-facing WASM test unfiltered.
	cd web && CI=true node --test $(WEB_TEST_CONCURRENCY_ARG) $(WEB_WASM_FAST_SUITES) $(WEB_WASM_SLOW_SUITES)

typecheck-web: build-wasm ## Type-check the web client without writing compiler state.
	cd web && CI=true pnpm exec tsc --noEmit --incremental false --pretty false

build-web: build-wasm ## Build the production web application.
	cd web && CI=true pnpm run build:app

test-web-render: build-web ## Test the built server-rendered application shell.
	cd web && CI=true node --test $(WEB_TEST_CONCURRENCY_ARG) tests/rendered-html.test.mjs

test-web-unit: ## Run fast standalone Node tests outside the WASM suites.
	@if [ -n "$(strip $(WEB_FAST_ROOT_TESTS))" ]; then \
		cd web && CI=true node --test $(WEB_TEST_CONCURRENCY_ARG) $(WEB_FAST_ROOT_TESTS); \
	else \
		echo "No standalone fast web tests discovered"; \
	fi

test-web-fast: test-web-unit test-web-wasm ## Run every fast web test without a production build.

test-web: test-web-fast test-web-render ## Run the normal web tests.

test-web-full: build-web ## Run every discovered web test unfiltered.
	cd web && CI=true node --test $(WEB_TEST_CONCURRENCY_ARG) $(WEB_ROOT_TESTS) $(WEB_WASM_FAST_SUITES) $(WEB_WASM_SLOW_SUITES)

test: test-rust test-profile-attribution test-magic-references test-web ## Run normal Rust, tooling, and web tests.

test-slow: test-rust-slow test-web-wasm-slow ## Run only simulation-heavy suites.

check-fast: fmt-rust lint test-rust test-profile-attribution test-magic-references typecheck-web test-web-fast ## Run the broad checkpoint without slow tests or a production web build.

check-rust: fmt-rust lint-rust test-rust-budget ## Run the complete root Rust workspace gate.

check-web: lint-web typecheck-web test-web-full ## Run the complete web gate.

check: check-rust check-web lint-infra-available test-profile-attribution test-magic-references ## Run the complete engine, web, and tooling gate.

check-bindings-c: test-source-file-sizes ## Build and smoke-test only the C ABI.
	./scripts/check-bindings.sh c

check-bindings-python: test-source-file-sizes ## Build and smoke-test only the Python module.
	./scripts/check-bindings.sh python

check-bindings: fmt-python-binding lint-python-binding test-source-file-sizes ## Strictly validate both bot bindings.
	./scripts/check-bindings.sh all

check-bindings-available: test-source-file-sizes ## Smoke-test bindings available on this machine.
	./scripts/check-bindings.sh available

ci: check check-bindings ## Run every repository gate.
