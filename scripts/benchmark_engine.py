#!/usr/bin/env python3
"""Maintain and compare a local main-branch engine benchmark baseline."""

from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import hashlib
import json
import os
import platform
import shlex
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterator, List, Mapping, Optional, Sequence, Tuple

if os.name == "nt":
    import msvcrt
else:
    import fcntl


CACHE_LAYOUT_VERSION = 1
WORKLOAD_SCHEMA_VERSION = 1
CACHE_DIR_NAME = "penta-performance-cache"
MAX_U64 = (1 << 64) - 1
BINARY_FILE_NAME = "penta-match.exe" if os.name == "nt" else "penta-match"


class BenchmarkError(RuntimeError):
    """A user-facing benchmark setup or execution error."""


@dataclass(frozen=True)
class Settings:
    repo_root: Path
    baseline_ref: str
    games: int
    seed: int
    warmup: int
    runs: int
    comparison_output: Path

    @classmethod
    def from_environment(cls, repo_root: Path) -> "Settings":
        output_value = os.environ.get("BENCHMARK_OUTPUT", "")
        output = (
            resolve_path(output_value, base=repo_root)
            if output_value
            else repo_root / "target/profiles/engine-main-compare.json"
        )
        return cls(
            repo_root=repo_root,
            baseline_ref=os.environ.get(
                "PERFORMANCE_BASELINE_REF", "refs/heads/main"
            ),
            games=parse_u64("PROFILE_GAMES", default=4000, positive=True),
            seed=parse_u64("PROFILE_SEED", default=1, positive=False),
            warmup=parse_u64("BENCHMARK_WARMUP", default=1, positive=False),
            runs=parse_u64("BENCHMARK_RUNS", default=10, positive=True),
            comparison_output=output,
        )

    @property
    def workload_args(self) -> List[str]:
        return [
            "--p1",
            "random",
            "--p2",
            "random",
            "--deck1",
            "Random",
            "--deck2",
            "Random",
            "--games",
            str(self.games),
            "--seed",
            str(self.seed),
        ]

    @property
    def workload_identity(self) -> Dict[str, Any]:
        return {
            "schema_version": WORKLOAD_SCHEMA_VERSION,
            "binary": "penta-match",
            "build_profile": "release",
            "arguments": self.workload_args,
        }


@dataclass(frozen=True)
class BaselineArtifacts:
    revision: str
    binary: Path
    cached_binary: Path
    binary_manifest: Mapping[str, Any]
    benchmark: Path
    benchmark_manifest: Mapping[str, Any]
    outcome: str


def parse_u64(name: str, *, default: int, positive: bool) -> int:
    raw = os.environ.get(name, str(default))
    if not raw.isascii() or not raw.isdigit():
        requirement = "positive" if positive else "non-negative"
        raise BenchmarkError(f"{name} must be a {requirement} integer, got: {raw}")
    value = int(raw)
    if value > MAX_U64 or (positive and value == 0):
        requirement = "positive" if positive else "non-negative"
        raise BenchmarkError(
            f"{name} must be a {requirement} 64-bit integer, got: {raw}"
        )
    return value


def resolve_path(value: str, *, base: Path) -> Path:
    path = Path(value).expanduser()
    if not path.is_absolute():
        path = base / path
    return path.resolve()


def run_text(
    command: Sequence[str],
    *,
    cwd: Path,
    error_context: str,
    allow_failure: bool = False,
) -> Optional[str]:
    try:
        result = subprocess.run(
            command,
            cwd=cwd,
            check=False,
            capture_output=True,
            text=True,
        )
    except OSError as error:
        if allow_failure:
            return None
        raise BenchmarkError(f"{error_context}: {error}") from error
    if result.returncode != 0:
        if allow_failure:
            return None
        detail = result.stderr.strip() or result.stdout.strip()
        if detail:
            raise BenchmarkError(f"{error_context}: {detail}")
        raise BenchmarkError(f"{error_context}: command exited {result.returncode}")
    return result.stdout.strip()


def git_output(repo_root: Path, *arguments: str) -> str:
    output = run_text(
        ("git", "-C", str(repo_root), *arguments),
        cwd=repo_root,
        error_context="Git command failed",
    )
    assert output is not None
    if not output:
        raise BenchmarkError("Git returned an empty result")
    return output


def resolve_baseline_revision(repo_root: Path, baseline_ref: str) -> str:
    try:
        return git_output(repo_root, "rev-parse", "--verify", f"{baseline_ref}^{{commit}}")
    except BenchmarkError as error:
        raise BenchmarkError(
            f"cannot resolve the local baseline ref {baseline_ref!r}; update that ref "
            "or set PERFORMANCE_BASELINE_REF (for example, origin/main)"
        ) from error


def git_common_dir(repo_root: Path) -> Path:
    return Path(
        git_output(
            repo_root,
            "rev-parse",
            "--path-format=absolute",
            "--git-common-dir",
        )
    ).resolve()


def cache_root(repo_root: Path) -> Path:
    common_dir = git_common_dir(repo_root)
    configured = os.environ.get("PENTA_PERFORMANCE_CACHE_DIR", "")
    if configured:
        return resolve_path(configured, base=common_dir)
    return common_dir / CACHE_DIR_NAME / f"layout-v{CACHE_LAYOUT_VERSION}"


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat().replace("+00:00", "Z")


def canonical_hash(value: Any) -> str:
    payload = json.dumps(
        value, sort_keys=True, separators=(",", ":"), ensure_ascii=True
    ).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def optional_command_output(command: Sequence[str]) -> Optional[str]:
    try:
        result = subprocess.run(
            command,
            check=False,
            capture_output=True,
            text=True,
            timeout=5,
        )
    except (OSError, subprocess.TimeoutExpired):
        return None
    if result.returncode != 0:
        return None
    value = result.stdout.strip()
    return value or None


def machine_facts() -> Dict[str, str]:
    facts = {
        "system": platform.system(),
        "release": platform.release(),
        "machine": platform.machine(),
    }
    processor = platform.processor()
    if processor:
        facts["processor"] = processor

    if facts["system"] == "Darwin":
        for label, key in [
            ("model", "hw.model"),
            ("cpu", "machdep.cpu.brand_string"),
        ]:
            value = optional_command_output(("sysctl", "-n", key))
            if value:
                facts[label] = value
    elif facts["system"] == "Linux":
        try:
            for line in Path("/proc/cpuinfo").read_text(encoding="utf-8").splitlines():
                key, separator, value = line.partition(":")
                if separator and key.strip() in {"model name", "Hardware"}:
                    facts["cpu"] = value.strip()
                    break
        except (OSError, UnicodeError):
            pass
    return facts


def relevant_build_environment() -> Dict[str, str]:
    exact_names = {
        "AR",
        "CC",
        "CFLAGS",
        "CARGO_BUILD_TARGET",
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_INCREMENTAL",
        "MACOSX_DEPLOYMENT_TARGET",
        "RUSTC",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
        "RUSTFLAGS",
        "RUSTUP_TOOLCHAIN",
        "SOURCE_DATE_EPOCH",
    }
    safe_prefixes = (
        "CARGO_BUILD_",
        "CARGO_PROFILE_RELEASE_",
        "CARGO_TARGET_",
    )
    return {
        name: value
        for name, value in sorted(os.environ.items())
        if name in exact_names
        or (name != "CARGO_TARGET_DIR" and name.startswith(safe_prefixes))
    }


def read_json(path: Path) -> Optional[Mapping[str, Any]]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        return None
    return value if isinstance(value, dict) else None


def write_json(path: Path, value: Mapping[str, Any]) -> None:
    path.write_text(
        json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def atomic_write_json(path: Path, value: Mapping[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        prefix=f".{path.name}.", dir=path.parent
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as output:
            json.dump(value, output, indent=2, sort_keys=True, ensure_ascii=False)
            output.write("\n")
            output.flush()
            os.fsync(output.fileno())
        os.replace(temporary, path)
    finally:
        with contextlib.suppress(FileNotFoundError):
            temporary.unlink()


@contextlib.contextmanager
def cache_lock(root: Path) -> Iterator[None]:
    root.mkdir(parents=True, exist_ok=True)
    lock_path = root / "refresh.lock"
    with lock_path.open("a+b") as lock:
        if os.name == "nt":
            lock.seek(0, os.SEEK_END)
            if lock.tell() == 0:
                lock.write(b"\0")
                lock.flush()
            while True:
                lock.seek(0)
                try:
                    msvcrt.locking(lock.fileno(), msvcrt.LK_NBLCK, 1)
                    break
                except OSError:
                    time.sleep(0.1)
        else:
            fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
        try:
            yield
        finally:
            if os.name == "nt":
                lock.seek(0)
                msvcrt.locking(lock.fileno(), msvcrt.LK_UNLCK, 1)
            else:
                fcntl.flock(lock.fileno(), fcntl.LOCK_UN)


def path_is_within(path: Path, directory: Path) -> bool:
    try:
        path.relative_to(directory)
    except ValueError:
        return False
    return True


def extract_git_archive(archive_path: Path, destination: Path) -> None:
    destination = destination.resolve()
    with tarfile.open(archive_path, mode="r:") as archive:
        members = archive.getmembers()
        for member in members:
            extracted = (destination / member.name).resolve()
            if not path_is_within(extracted, destination):
                raise BenchmarkError(
                    f"main snapshot contains an unsafe archive path: {member.name}"
                )
            if member.issym():
                linked = (extracted.parent / member.linkname).resolve()
                if not path_is_within(linked, destination):
                    raise BenchmarkError(
                        f"main snapshot contains an unsafe symlink: {member.name}"
                    )
            elif member.islnk():
                linked = (destination / member.linkname).resolve()
                if not path_is_within(linked, destination):
                    raise BenchmarkError(
                        f"main snapshot contains an unsafe hard link: {member.name}"
                    )
            elif member.isdev() or member.isfifo():
                raise BenchmarkError(
                    f"main snapshot contains an unsupported special file: {member.name}"
                )
        archive.extractall(destination, members=members, filter="fully_trusted")


@contextlib.contextmanager
def main_source_snapshot(
    settings: Settings, revision: str
) -> Iterator[Tuple[Path, Path]]:
    with tempfile.TemporaryDirectory(prefix="penta-main-build-") as temporary_name:
        temporary = Path(temporary_name)
        source = temporary / "source"
        target = temporary / "target"
        archive = temporary / "source.tar"
        source.mkdir()
        try:
            subprocess.run(
                (
                    "git",
                    "-C",
                    str(settings.repo_root),
                    "archive",
                    "--format=tar",
                    f"--output={archive}",
                    revision,
                ),
                check=True,
            )
            extract_git_archive(archive, source)
        except (OSError, subprocess.CalledProcessError, tarfile.TarError) as error:
            raise BenchmarkError(f"could not extract the main snapshot: {error}") from error
        yield source, target


def cargo_configuration_fingerprints(source_root: Path) -> List[Mapping[str, str]]:
    locations: List[Tuple[str, Path]] = [("source", source_root / ".cargo")]
    cargo_home = Path(
        os.environ.get("CARGO_HOME", str(Path.home() / ".cargo"))
    ).expanduser()
    locations.append(("cargo-home", cargo_home))
    locations.extend(
        (f"ancestor-{index}", ancestor / ".cargo")
        for index, ancestor in enumerate(source_root.parents)
    )

    fingerprints: List[Mapping[str, str]] = []
    seen: set[Path] = set()
    for scope, directory in locations:
        for filename in ("config.toml", "config"):
            candidate = (directory / filename).resolve()
            if candidate in seen or not candidate.is_file():
                continue
            seen.add(candidate)
            fingerprints.append(
                {
                    "scope": scope,
                    "filename": filename,
                    "sha256": file_sha256(candidate),
                }
            )
    return fingerprints


def build_tool_versions(source_root: Path) -> Dict[str, str]:
    rustc = os.environ.get("RUSTC", "rustc")
    cargo = run_text(
        ("cargo", "--version", "--verbose"),
        cwd=source_root,
        error_context="could not read Cargo version",
    )
    rustc_version = run_text(
        (rustc, "--version", "--verbose"),
        cwd=source_root,
        error_context="could not read Rust compiler version",
    )
    assert cargo is not None and rustc_version is not None
    return {"cargo": cargo, "rustc": rustc_version}


def cargo_build(source_root: Path, *, target_dir: Optional[Path]) -> Path:
    environment = os.environ.copy()
    # Developer profiles are incremental, but benchmark binaries must be
    # reproducible release builds rather than products of an edit history.
    environment["CARGO_INCREMENTAL"] = "0"
    if target_dir is not None:
        environment["CARGO_TARGET_DIR"] = str(target_dir)
    command = [
        "cargo",
        "build",
        "--locked",
        "--release",
        "--bin",
        "penta-match",
        "--message-format=json-render-diagnostics",
    ]
    executable: Optional[Path] = None
    try:
        process = subprocess.Popen(
            command,
            cwd=source_root,
            env=environment,
            stdout=subprocess.PIPE,
            text=True,
        )
    except OSError as error:
        raise BenchmarkError(f"could not build the release workload: {error}") from error
    assert process.stdout is not None
    for line in process.stdout:
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            sys.stderr.write(line)
            continue
        if message.get("reason") == "compiler-message":
            rendered = message.get("message", {}).get("rendered")
            if rendered:
                sys.stderr.write(rendered)
        if message.get("reason") == "compiler-artifact":
            target = message.get("target", {})
            candidate = message.get("executable")
            if target.get("name") == "penta-match" and candidate:
                executable = Path(candidate)
    return_code = process.wait()
    if return_code != 0:
        raise BenchmarkError(f"release workload build exited {return_code}")
    if executable is None or not executable.is_file():
        raise BenchmarkError("Cargo did not report the penta-match release binary")
    return executable.resolve()


def replace_directory(staging: Path, destination: Path) -> None:
    if destination.exists():
        shutil.rmtree(destination)
    os.replace(staging, destination)


def binary_cache_valid(
    directory: Path,
    expected_identity: Mapping[str, Any],
    *,
    expected_tools: Optional[Mapping[str, str]] = None,
    expected_cargo_configs: Optional[Sequence[Mapping[str, str]]] = None,
) -> Optional[Tuple[Path, Mapping[str, Any]]]:
    manifest = read_json(directory / "manifest.json")
    binary = directory / BINARY_FILE_NAME
    if (
        manifest is None
        or manifest.get("identity") != expected_identity
        or (
            expected_tools is not None
            and manifest.get("tools") != expected_tools
        )
        or (
            expected_cargo_configs is not None
            and manifest.get("cargo_configurations") != expected_cargo_configs
        )
        or not (directory / "complete").is_file()
        or not binary.is_file()
    ):
        return None
    if manifest.get("binary_sha256") != file_sha256(binary):
        return None
    return binary, manifest


def ensure_baseline_binary(
    settings: Settings,
    *,
    root: Path,
    revision: str,
    facts: Mapping[str, str],
    build_environment: Mapping[str, str],
) -> Tuple[Path, Mapping[str, Any]]:
    identity = {
        "schema_version": CACHE_LAYOUT_VERSION,
        "revision": revision,
        "build_profile": "release",
        "machine": facts,
        "build_environment": build_environment,
    }
    machine_key = canonical_hash(facts)[:16]
    build_key = canonical_hash(identity)[:16]
    directory = root / "engine" / machine_key / revision / build_key
    with main_source_snapshot(settings, revision) as (source, target):
        tools = build_tool_versions(source)
        cargo_configs = cargo_configuration_fingerprints(source)
        cached = binary_cache_valid(
            directory,
            identity,
            expected_tools=tools,
            expected_cargo_configs=cargo_configs,
        )
        if cached is not None:
            return cached

        print(
            f"Building baseline binary from {settings.baseline_ref} "
            f"at {revision[:12]}"
        )
        directory.parent.mkdir(parents=True, exist_ok=True)
        staging = Path(tempfile.mkdtemp(prefix=".binary-", dir=directory.parent))
        try:
            built_binary = cargo_build(source, target_dir=target)
            published_binary = staging / BINARY_FILE_NAME
            shutil.copy2(built_binary, published_binary)
            manifest = {
                "schema_version": CACHE_LAYOUT_VERSION,
                "created_at": utc_now(),
                "baseline_ref": settings.baseline_ref,
                "identity": identity,
                "binary_sha256": file_sha256(published_binary),
                "tools": tools,
                "cargo_configurations": cargo_configs,
            }
            previous_binary = directory / BINARY_FILE_NAME
            previous_workloads = directory / "workloads"
            if (
                previous_binary.is_file()
                and previous_workloads.is_dir()
                and file_sha256(previous_binary) == manifest["binary_sha256"]
            ):
                shutil.copytree(previous_workloads, staging / "workloads")
            write_json(staging / "manifest.json", manifest)
            (staging / "complete").touch()
            replace_directory(staging, directory)
        except OSError as error:
            raise BenchmarkError(f"could not build the main baseline: {error}") from error
        finally:
            if staging.exists():
                shutil.rmtree(staging)

    cached = binary_cache_valid(
        directory,
        identity,
        expected_tools=tools,
        expected_cargo_configs=cargo_configs,
    )
    if cached is None:
        raise BenchmarkError(f"published baseline binary is incomplete: {directory}")
    return cached


def checked_tool_version(command: str, *, repo_root: Path) -> str:
    if shutil.which(command) is None:
        if command == "hyperfine":
            raise BenchmarkError(
                "hyperfine is required; install it with: cargo install --locked hyperfine"
            )
        raise BenchmarkError(f"required command not found: {command}")
    output = run_text(
        (command, "--version"),
        cwd=repo_root,
        error_context=f"could not read {command} version",
    )
    assert output is not None
    return output


def run_outcome(binary: Path, settings: Settings) -> str:
    try:
        result = subprocess.run(
            (str(binary), *settings.workload_args),
            cwd=settings.repo_root,
            check=False,
            capture_output=True,
            text=True,
        )
    except OSError as error:
        raise BenchmarkError(f"could not run {binary}: {error}") from error
    if result.stderr:
        sys.stderr.write(result.stderr)
    if result.returncode != 0:
        raise BenchmarkError(f"deterministic workload exited {result.returncode}")
    return result.stdout


def run_hyperfine(
    commands: Sequence[Tuple[str, Sequence[str]]],
    *,
    settings: Settings,
    output: Path,
) -> Mapping[str, Any]:
    try:
        output.parent.mkdir(parents=True, exist_ok=True)
        descriptor, temporary_name = tempfile.mkstemp(
            prefix=f".{output.name}.", dir=output.parent
        )
        os.close(descriptor)
        temporary = Path(temporary_name)
        temporary.unlink()
        arguments = [
            "hyperfine",
            "--style",
            "basic",
            "--shell=none",
            "--warmup",
            str(settings.warmup),
            "--runs",
            str(settings.runs),
            "--sort",
            "command",
        ]
        for name, _ in commands:
            arguments.extend(("--command-name", name))
        arguments.extend(("--export-json", str(temporary)))
        arguments.extend(
            (
                subprocess.list2cmdline(command)
                if os.name == "nt"
                else shlex.join(command)
            )
            for _, command in commands
        )
        subprocess.run(arguments, cwd=settings.repo_root, check=True)
        result = read_json(temporary)
        if result is None or not isinstance(result.get("results"), list):
            raise BenchmarkError(
                f"Hyperfine wrote an invalid JSON result: {temporary}"
            )
        os.replace(temporary, output)
        return result
    except (OSError, subprocess.CalledProcessError) as error:
        raise BenchmarkError(f"Hyperfine benchmark failed: {error}") from error
    finally:
        if "temporary" in locals():
            with contextlib.suppress(FileNotFoundError):
                temporary.unlink()


def benchmark_cache_valid(
    directory: Path,
    *,
    expected_identity: Mapping[str, Any],
    binary_sha256: str,
) -> Optional[Tuple[Path, Mapping[str, Any], str]]:
    manifest = read_json(directory / "manifest.json")
    benchmark = directory / "benchmark.json"
    outcome_path = directory / "outcome.txt"
    if (
        manifest is None
        or manifest.get("identity") != expected_identity
        or manifest.get("binary_sha256") != binary_sha256
        or not (directory / "complete").is_file()
        or read_json(benchmark) is None
        or not outcome_path.is_file()
    ):
        return None
    try:
        outcome = outcome_path.read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None
    return benchmark, manifest, outcome


def ensure_baseline_benchmark(
    settings: Settings,
    *,
    root: Path,
    revision: str,
    binary: Path,
    binary_manifest: Mapping[str, Any],
    hyperfine_version: str,
) -> Tuple[Path, Mapping[str, Any], str]:
    binary_sha256 = str(binary_manifest["binary_sha256"])
    identity = {
        "schema_version": CACHE_LAYOUT_VERSION,
        "revision": revision,
        "workload": settings.workload_identity,
        "measurement": {
            "warmup_runs": settings.warmup,
            "measured_runs": settings.runs,
            "hyperfine": hyperfine_version,
        },
    }
    directory = binary.parent / "workloads" / canonical_hash(identity)[:20]
    cached = benchmark_cache_valid(
        directory,
        expected_identity=identity,
        binary_sha256=binary_sha256,
    )
    if cached is not None:
        return cached

    print(
        f"Verifying deterministic output for {settings.games} main games "
        f"(seed {settings.seed})"
    )
    outcome = run_outcome(binary, settings)
    print(outcome, end="" if outcome.endswith("\n") else "\n")
    print(
        f"Benchmarking main with {settings.runs} measured run(s); "
        "this is advisory, not a threshold"
    )

    directory.parent.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix=".workload-", dir=directory.parent))
    try:
        benchmark = staging / "benchmark.json"
        exact_command = [str(binary), *settings.workload_args]
        run_hyperfine(
            [(f"main@{revision[:12]}", exact_command)],
            settings=settings,
            output=benchmark,
        )
        (staging / "outcome.txt").write_text(outcome, encoding="utf-8")
        manifest = {
            "schema_version": CACHE_LAYOUT_VERSION,
            "created_at": utc_now(),
            "baseline_ref": settings.baseline_ref,
            "binary_sha256": binary_sha256,
            "identity": identity,
            "command": exact_command,
        }
        write_json(staging / "manifest.json", manifest)
        (staging / "complete").touch()
        replace_directory(staging, directory)
    finally:
        if staging.exists():
            shutil.rmtree(staging)

    cached = benchmark_cache_valid(
        directory,
        expected_identity=identity,
        binary_sha256=binary_sha256,
    )
    if cached is None:
        raise BenchmarkError(f"published baseline result is incomplete: {directory}")
    return cached


def prepare_baseline(
    settings: Settings,
    *,
    binary_snapshot: Optional[Path] = None,
) -> BaselineArtifacts:
    revision = resolve_baseline_revision(settings.repo_root, settings.baseline_ref)
    root = cache_root(settings.repo_root)
    hyperfine_version = checked_tool_version("hyperfine", repo_root=settings.repo_root)
    facts = machine_facts()
    build_environment = relevant_build_environment()
    build_environment["CARGO_INCREMENTAL"] = "0"
    with cache_lock(root):
        binary, binary_manifest = ensure_baseline_binary(
            settings,
            root=root,
            revision=revision,
            facts=facts,
            build_environment=build_environment,
        )
        benchmark, benchmark_manifest, outcome = ensure_baseline_benchmark(
            settings,
            root=root,
            revision=revision,
            binary=binary,
            binary_manifest=binary_manifest,
            hyperfine_version=hyperfine_version,
        )
        cached_binary = binary
        if binary_snapshot is not None:
            binary_snapshot.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(cached_binary, binary_snapshot)
            if file_sha256(binary_snapshot) != binary_manifest["binary_sha256"]:
                raise BenchmarkError("the baseline binary changed while being copied")
            binary = binary_snapshot
    return BaselineArtifacts(
        revision=revision,
        binary=binary,
        cached_binary=cached_binary,
        binary_manifest=binary_manifest,
        benchmark=benchmark,
        benchmark_manifest=benchmark_manifest,
        outcome=outcome,
    )


def current_revision(repo_root: Path) -> str:
    return git_output(repo_root, "rev-parse", "HEAD^{commit}")


def current_label(repo_root: Path, revision: str) -> str:
    branch = run_text(
        ("git", "-C", str(repo_root), "branch", "--show-current"),
        cwd=repo_root,
        error_context="could not read current branch",
    )
    return f"{branch or 'HEAD'}@{revision[:12]}"


def current_status(repo_root: Path) -> List[str]:
    output = run_text(
        (
            "git",
            "-C",
            str(repo_root),
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
        ),
        cwd=repo_root,
        error_context="could not read worktree status",
    )
    return output.splitlines() if output else []


def comparison_metadata_path(output: Path) -> Path:
    if output.suffix:
        return output.with_name(f"{output.stem}.metadata{output.suffix}")
    return output.with_name(f"{output.name}.metadata.json")


def result_mean(result: Mapping[str, Any], index: int) -> float:
    results = result.get("results")
    if not isinstance(results, list) or len(results) <= index:
        raise BenchmarkError("Hyperfine result does not contain both comparison commands")
    mean = results[index].get("mean")
    if not isinstance(mean, (int, float)) or mean <= 0:
        raise BenchmarkError("Hyperfine result contains an invalid mean time")
    return float(mean)


def compare(settings: Settings) -> None:
    with tempfile.TemporaryDirectory(prefix="penta-engine-compare-") as temporary_name:
        temporary = Path(temporary_name)
        executable_suffix = ".exe" if os.name == "nt" else ""
        main_binary = temporary / f"penta-match-main{executable_suffix}"
        current_binary = temporary / f"penta-match-current{executable_suffix}"
        baseline = prepare_baseline(settings, binary_snapshot=main_binary)
        revision = current_revision(settings.repo_root)
        status = current_status(settings.repo_root)
        label = current_label(settings.repo_root, revision)

        print(
            f"Building current worktree at {revision[:12]}"
            f"{' (dirty)' if status else ''}"
        )
        built_current = cargo_build(settings.repo_root, target_dir=None)
        recorded_main_command = [
            str(baseline.cached_binary),
            *settings.workload_args,
        ]
        recorded_branch_command = [str(built_current), *settings.workload_args]
        shutil.copy2(built_current, current_binary)
        current_binary_sha256 = file_sha256(current_binary)

        print(
            f"Verifying deterministic output for main@{baseline.revision[:12]} "
            f"and {label}"
        )
        main_outcome = run_outcome(main_binary, settings)
        branch_outcome = run_outcome(current_binary, settings)
        print(f"main:\n{main_outcome}", end="" if main_outcome.endswith("\n") else "\n")
        print(
            f"current:\n{branch_outcome}",
            end="" if branch_outcome.endswith("\n") else "\n",
        )
        outcomes_match = main_outcome == branch_outcome
        if not outcomes_match:
            print(
                "WARNING: deterministic outcomes differ; the timing includes changed "
                "game paths as well as implementation cost",
                file=sys.stderr,
            )

        main_command = [str(main_binary), *settings.workload_args]
        branch_command = [str(current_binary), *settings.workload_args]
        print(
            f"Comparing main and current with {settings.runs} measured run(s); "
            "this is advisory, not a threshold"
        )
        result = run_hyperfine(
            [
                (f"main@{baseline.revision[:12]}", main_command),
                (label, branch_command),
            ],
            settings=settings,
            output=settings.comparison_output,
        )

    main_mean = result_mean(result, 0)
    branch_mean = result_mean(result, 1)
    delta_percent = (branch_mean / main_mean - 1.0) * 100.0
    current_tools = build_tool_versions(settings.repo_root)
    baseline_tools = baseline.binary_manifest.get("tools", {})
    toolchains_match = current_tools == baseline_tools
    current_cargo_configs = cargo_configuration_fingerprints(settings.repo_root)
    baseline_cargo_configs = baseline.binary_manifest.get(
        "cargo_configurations", []
    )
    cargo_configs_match = current_cargo_configs == baseline_cargo_configs
    if not toolchains_match:
        print(
            "WARNING: main and current used different Rust build tools; record that "
            "when interpreting the delta",
            file=sys.stderr,
        )
    if not cargo_configs_match:
        print(
            "WARNING: main and current used different effective Cargo "
            "configurations; record that when interpreting the delta",
            file=sys.stderr,
        )

    metadata = {
        "schema_version": CACHE_LAYOUT_VERSION,
        "created_at": utc_now(),
        "advisory_only": True,
        "baseline": {
            "ref": settings.baseline_ref,
            "revision": baseline.revision,
            "saved_benchmark": str(baseline.benchmark),
            "binary_sha256": baseline.binary_manifest["binary_sha256"],
            "cargo_configurations": baseline_cargo_configs,
            "tools": baseline_tools,
            "outcome": main_outcome,
        },
        "current": {
            "revision": revision,
            "dirty": bool(status),
            "status": status,
            "binary_sha256": current_binary_sha256,
            "cargo_configurations": current_cargo_configs,
            "tools": current_tools,
            "outcome": branch_outcome,
        },
        "workload": settings.workload_identity,
        "measurement": {
            "warmup_runs": settings.warmup,
            "measured_runs": settings.runs,
            "hyperfine": baseline.benchmark_manifest["identity"]["measurement"][
                "hyperfine"
            ],
            "main_mean_seconds": main_mean,
            "current_mean_seconds": branch_mean,
            "current_delta_percent": delta_percent,
        },
        "comparability": {
            "cargo_configurations_match": cargo_configs_match,
            "deterministic_outcomes_match": outcomes_match,
            "rust_toolchains_match": toolchains_match,
        },
        "commands": {
            "main": recorded_main_command,
            "current": recorded_branch_command,
        },
        "hyperfine_json_sha256": file_sha256(settings.comparison_output),
    }
    metadata_path = comparison_metadata_path(settings.comparison_output)
    atomic_write_json(metadata_path, metadata)

    direction = "slower" if delta_percent >= 0 else "faster"
    print(
        f"Current mean is {abs(delta_percent):.2f}% {direction} than main "
        f"({branch_mean:.6f}s vs {main_mean:.6f}s)."
    )
    print(f"Comparison JSON: {settings.comparison_output}")
    print(f"Comparison metadata: {metadata_path}")

    latest_revision = resolve_baseline_revision(
        settings.repo_root, settings.baseline_ref
    )
    if latest_revision != baseline.revision:
        print(
            f"WARNING: {settings.baseline_ref} advanced to {latest_revision[:12]} "
            "during measurement; rerun to compare with the new revision",
            file=sys.stderr,
        )


def baseline(settings: Settings) -> None:
    artifacts = prepare_baseline(settings)
    print(f"Baseline revision: {artifacts.revision}")
    print(f"Baseline benchmark: {artifacts.benchmark}")
    print(f"Baseline metadata: {artifacts.benchmark.parent / 'manifest.json'}")


def parse_args(arguments: Optional[Sequence[str]] = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Maintain an advisory engine benchmark baseline in Git's common "
            "directory and compare the current worktree with it."
        )
    )
    parser.add_argument("mode", choices=("baseline", "compare"))
    return parser.parse_args(arguments)


def main(arguments: Optional[Sequence[str]] = None) -> int:
    with contextlib.suppress(AttributeError, OSError, ValueError):
        sys.stdout.reconfigure(line_buffering=True)
    namespace = parse_args(arguments)
    repo_root = Path(__file__).resolve().parent.parent
    try:
        settings = Settings.from_environment(repo_root)
        if namespace.mode == "baseline":
            baseline(settings)
        else:
            compare(settings)
    except BenchmarkError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    except OSError as error:
        print(f"error: filesystem operation failed: {error}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        print("benchmark interrupted", file=sys.stderr)
        return 130
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
