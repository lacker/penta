#!/usr/bin/env python3
"""Validate repository agent guidance without compiling the engine."""

from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[1]
SKILLS = (
    "profile-engine-performance",
    "query-magic-references",
    "refresh-magic-references",
)
MAX_ROOT_INSTRUCTION_BYTES = 8 * 1024
REQUIRED_SCRIPTS = (
    ".agents/skills/profile-engine-performance/scripts/profile_attribution.py",
    ".agents/skills/refresh-magic-references/scripts/reference_material.py",
    "scripts/benchmark_engine.py",
)


def frontmatter_field(text: str, field: str) -> str | None:
    if not text.startswith("---\n"):
        return None
    end = text.find("\n---", 4)
    if end < 0:
        return None
    prefix = f"{field}:"
    for line in text[4:end].splitlines():
        if line.startswith(prefix):
            return line[len(prefix) :].strip().strip('"')
    return None


def markdown_link_targets(text: str) -> list[tuple[int, str]]:
    targets = []
    for number, line in enumerate(text.splitlines(), start=1):
        targets.extend((number, match) for match in re.findall(r"]\(([^)]+)\)", line))
    return targets


def check_repository_links(source: str, text: str, *, reject_relative: bool) -> None:
    for number, target in markdown_link_targets(text):
        if target.startswith(("http", "#")):
            continue
        path = target.split("#", 1)[0]
        if reject_relative:
            assert not path.startswith(("./", "../")), (
                f"{source}:{number}: link {target!r} is relative to the skill file; "
                "write it from the repository root"
            )
        assert (ROOT / path).exists(), (
            f"{source}:{number}: routed path {target!r} does not resolve"
        )


def main() -> None:
    agents_path = ROOT / "AGENTS.md"
    agents_bytes = agents_path.read_bytes()
    agents = agents_bytes.decode()
    assert len(agents_bytes) <= MAX_ROOT_INSTRUCTION_BYTES, (
        f"AGENTS.md is {len(agents_bytes)} bytes; keep the always-loaded router "
        f"at or below {MAX_ROOT_INSTRUCTION_BYTES} bytes and move task-specific "
        "detail into linked docs or skills"
    )
    check_repository_links("AGENTS.md", agents, reject_relative=False)

    claude = (ROOT / "CLAUDE.md").read_text()
    assert "@AGENTS.md" in claude, (
        "CLAUDE.md must import AGENTS.md so the two harnesses cannot drift"
    )

    for skill in SKILLS:
        canonical = ROOT / ".agents" / "skills" / skill / "SKILL.md"
        claude_copy = ROOT / ".claude" / "skills" / skill / "SKILL.md"
        assert canonical.is_file(), f"{skill}: missing canonical SKILL.md"
        assert claude_copy.is_file(), f"{skill}: missing Claude skill entrypoint"
        assert claude_copy.resolve() == canonical.resolve(), (
            f"{skill}: Claude and Codex entrypoints must resolve to the same file"
        )

        text = canonical.read_text()
        assert frontmatter_field(text, "name") == skill, (
            f"{skill}: frontmatter name must match its directory"
        )
        description = frontmatter_field(text, "description")
        assert description, f"{skill}: SKILL.md has no description"
        lowered = description.lower()
        for harness in ("codex", "claude", "copilot", "cursor"):
            assert harness not in lowered, (
                f"{skill}: description names {harness}; describe the work, not the agent"
            )
        check_repository_links(
            f".agents/skills/{skill}/SKILL.md", text, reject_relative=True
        )

    for script in REQUIRED_SCRIPTS:
        assert (ROOT / script).is_file(), f"skill invokes missing script: {script}"

    print("Agent guidance checks passed.")


if __name__ == "__main__":
    main()
