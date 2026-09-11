#!/usr/bin/env python3
"""Import a bounded TopDeck tournament corpus into ``decks/cedh``.

The event page exposes standings and deck URLs through ``window.__router_ops``;
each deck page exposes a canonical, sectioned ``decklistText`` string.  This
tool preserves every source section in provenance.  Commander, mainboard, and
sideboard sections become imported YAML; auxiliary sections remain explicit
provenance because they have no supported gameplay representation.

Example (using an archived event page):
    python3 tools/import_cedh_tournament.py \
      --event-html /tmp/penta-cedh-event.html \
      --output decks/cedh \
      --web-registry web/app/cedh-decks.json
"""

from __future__ import annotations

import argparse
import base64
import concurrent.futures
import hashlib
import json
import re
import shutil
import sys
import tempfile
import unicodedata
import urllib.request
from collections import OrderedDict
from pathlib import Path


EVENT_URL = "https://edhtop16.com/tournament/nacional-de-cedh-100k-1"
EVENT_NAME = "Nacional de cEDH 100K @ WolfCon 2026"
EVENT_DATE = "2026-08-29"
PLAYABLE_SECTION_KEYS = {
    "Commanders": "commanders",
    "Mainboard": "main",
    "Sideboard": "sideboard",
}
DECKLIST_KEY = '"decklistText":'
ENTRY_PREFIX = '"tournament":{"entries":'
CARD_LINE = re.compile(r"(?P<count>[1-9][0-9]*) (?P<name>.+)")


def fetch(url: str) -> str:
    request = urllib.request.Request(url, headers={"User-Agent": "penta-corpus-importer/1"})
    with urllib.request.urlopen(request, timeout=30) as response:
        return response.read().decode("utf-8")


def decode_at(document: str, start: int) -> object:
    return json.JSONDecoder().raw_decode(document[start:])[0]


def event_entries(document: str) -> list[dict[str, object]]:
    try:
        tournament = decode_at(document, document.index(ENTRY_PREFIX) + len('"tournament":'))
    except (ValueError, json.JSONDecodeError) as error:
        raise ValueError("event page does not contain a readable tournament entry payload") from error
    entries = tournament.get("entries") if isinstance(tournament, dict) else None
    if not isinstance(entries, list) or len(entries) != 123:
        raise ValueError(f"expected exactly 123 tournament entries, got {len(entries) if isinstance(entries, list) else 'none'}")
    return entries


def decklist_text(document: str) -> str:
    try:
        value = decode_at(document, document.index(DECKLIST_KEY) + len(DECKLIST_KEY))
    except (ValueError, json.JSONDecodeError) as error:
        raise ValueError("deck page does not contain a readable decklistText payload") from error
    if not isinstance(value, str):
        raise ValueError("decklistText payload is not a string")
    return value


def parse_sections(text: str) -> OrderedDict[str, OrderedDict[str, int]]:
    sections: OrderedDict[str, OrderedDict[str, int]] = OrderedDict()
    current: OrderedDict[str, int] | None = None
    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        if line.startswith("~~") and line.endswith("~~"):
            title = line.removeprefix("~~").removesuffix("~~")
            if title in sections:
                raise ValueError(f"duplicate source section {title!r}")
            current = OrderedDict()
            sections[title] = current
            continue
        match = CARD_LINE.fullmatch(line)
        if current is None or match is None:
            raise ValueError(f"unreadable decklist line {raw_line!r}")
        name = match.group("name")
        if name in current:
            raise ValueError(f"duplicate card {name!r} in {next(reversed(sections))}")
        current[name] = int(match.group("count"))
    missing = {"Commanders", "Mainboard"} - set(sections)
    if missing:
        raise ValueError(f"missing source sections: {', '.join(sorted(missing))}")
    sections.setdefault("Sideboard", OrderedDict())
    return sections


def entry_number(entry_id: str) -> str:
    """Return TopDeck's stable numeric entry component for a readable file ID."""
    try:
        decoded = base64.b64decode(entry_id).decode("ascii")
    except (UnicodeDecodeError, ValueError) as error:
        raise ValueError(f"unreadable TopDeck entry ID {entry_id!r}") from error
    prefix, separator, number = decoded.partition(":")
    if prefix != "Entry" or not separator or not number.isdecimal():
        raise ValueError(f"unexpected TopDeck entry ID {entry_id!r}")
    return number


def slug(text: str) -> str:
    normalized = unicodedata.normalize("NFKD", text)
    ascii_text = normalized.encode("ascii", "ignore").decode("ascii").lower()
    stem = re.sub(r"[^a-z0-9]+", "_", ascii_text).strip("_")
    return stem or "player"


def yaml_scalar(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def ordinal(number: int) -> str:
    suffix = "th" if 10 <= number % 100 <= 20 else {1: "st", 2: "nd", 3: "rd"}.get(number % 10, "th")
    return f"{number}{suffix}"


def render_mapping(key: str, cards: OrderedDict[str, int]) -> list[str]:
    if not cards:
        return [f"{key}: {{}}"]
    lines = [f"{key}:"]
    lines.extend(f"  {yaml_scalar(name)}: {count}" for name, count in cards.items())
    return lines


def render_deck(
    entry: dict[str, object], sections: OrderedDict[str, OrderedDict[str, int]]
) -> tuple[str, str, str, str]:
    standing = entry.get("standing")
    player = entry.get("player")
    entry_id = entry.get("id")
    deck_url = entry.get("decklist")
    if not isinstance(standing, int) or not isinstance(player, dict) or not isinstance(entry_id, str) or not isinstance(deck_url, str):
        raise ValueError("event entry is missing standing, player, id, or decklist URL")
    player_name = player.get("name")
    if not isinstance(player_name, str) or not player_name.strip():
        raise ValueError("event entry has no player name")
    file_id = f"nacional_{standing:03d}_{entry_number(entry_id)}_{slug(player_name)}"
    display_name = f"{EVENT_NAME} — {ordinal(standing)} place, {player_name.strip()}"
    description = f"Exact tournament decklist for {player_name.strip()}, {ordinal(standing)} place at {EVENT_NAME}."
    lines = [
        f"# {EVENT_NAME}, {EVENT_DATE}; {ordinal(standing)} place.",
        f"# Source event: {EVENT_URL}",
        f"# Source decklist: {deck_url}",
        f"# TopDeck entry ID: {entry_id}",
        f"name: {yaml_scalar(display_name)}",
        f"id: {file_id}",
        f"description: {yaml_scalar(description)}",
    ]
    auxiliary = [section for section in sections if section not in PLAYABLE_SECTION_KEYS]
    if auxiliary:
        lines.append(f"# Auxiliary source sections retained in provenance.json and unsupported by gameplay: {', '.join(auxiliary)}.")
    for section, key in PLAYABLE_SECTION_KEYS.items():
        lines.extend(render_mapping(key, sections[section]))
    return file_id, "\n".join(lines) + "\n", display_name, description


def write_web_registry(rendered: list[tuple[str, str, str, str]], path: Path) -> None:
    """Write the browser's imported-deck display registry deterministically."""
    registry = {name: description for _, _, name, description in rendered}
    path.write_text(json.dumps(registry, ensure_ascii=False, indent=2, sort_keys=True) + "\n")


def provenance_entry(entry: dict[str, object], page: str | None, error: str | None) -> dict[str, object]:
    record: dict[str, object] = {
        "entry_id": entry["id"],
        "standing": entry["standing"],
        "player": entry["player"]["name"],
        "decklist_url": entry["decklist"],
        "event_commander": entry["commander"]["name"],
    }
    if page is None:
        record["status"] = "unavailable"
        record["reason"] = error
        return record
    try:
        raw = decklist_text(page)
        sections = parse_sections(raw)
    except ValueError as parse_error:
        record["status"] = "unavailable"
        record["reason"] = str(parse_error)
        record["page_sha256"] = hashlib.sha256(page.encode()).hexdigest()
        return record
    record["status"] = "imported"
    record["decklist_text"] = raw
    record["sections"] = sections
    return record


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--event-html", type=Path, help="saved EDHTop16 event HTML; omit to fetch the source event")
    parser.add_argument("--event-url", default=EVENT_URL)
    parser.add_argument("--output", type=Path, default=Path("decks/cedh"))
    parser.add_argument("--web-registry", type=Path, help="optional generated browser display registry path")
    parser.add_argument("--workers", type=int, default=8)
    args = parser.parse_args()

    event_html = args.event_html.read_text() if args.event_html else fetch(args.event_url)
    entries = event_entries(event_html)
    urls = [entry["decklist"] for entry in entries]
    if not all(isinstance(url, str) for url in urls):
        raise ValueError("event contains an entry without a decklist URL")
    def fetch_or_error(url: str) -> tuple[str | None, str | None]:
        try:
            return fetch(url), None
        except Exception as error:  # Preserve a bounded-source failure in the manifest.
            return None, f"fetch failed: {error}"

    with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        pages = list(pool.map(fetch_or_error, urls))
    failures = [f"{url}: {error}" for url, (_, error) in zip(urls, pages, strict=True) if error]
    if failures:
        raise ValueError("deck fetch failed; existing corpus was not replaced:\n" + "\n".join(failures))

    provenance = [provenance_entry(entry, page, error) for entry, (page, error) in zip(entries, pages, strict=True)]
    rendered = []
    for entry, record in zip(entries, provenance, strict=True):
        if record["status"] != "imported":
            continue
        sections = OrderedDict(
            (section, OrderedDict(cards)) for section, cards in record["sections"].items()
        )
        rendered.append(render_deck(entry, sections))
    file_ids = [file_id for file_id, _, _, _ in rendered]
    if len(set(file_ids)) != len(file_ids):
        raise ValueError("generated file IDs are not unique")

    output = args.output
    temporary = Path(tempfile.mkdtemp(prefix="penta-cedh-", dir=output.parent))
    try:
        for file_id, contents, _, _ in rendered:
            (temporary / f"{file_id}.yaml").write_text(contents)
        manifest = {
            "event": {
                "name": EVENT_NAME,
                "date": EVENT_DATE,
                "url": args.event_url,
                "entry_count": len(entries),
            },
            "entries": provenance,
        }
        (temporary / "provenance.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
        unavailable = [entry for entry in provenance if entry["status"] == "unavailable"]
        auxiliary_cards = sum(
            sum(cards.values())
            for entry in provenance
            for section, cards in entry.get("sections", {}).items()
            if section not in PLAYABLE_SECTION_KEYS
        )
        (temporary / "README.md").write_text(
            "# Nacional de cEDH 100K corpus\n\n"
            f"This bounded corpus records all {len(entries)} EDHTop16 standings entries for {EVENT_NAME}, held on {EVENT_DATE}. "
            "The reproducible importer is `tools/import_cedh_tournament.py`; it reads the event payload and each linked TopDeck `decklistText` payload.\n\n"
            f"There are {len(rendered)} imported YAML decklists. The remaining {len(unavailable)} entries have no readable TopDeck `decklistText`, "
            "so they deliberately have no invented list; their event IDs, source URLs, and precise import reason are in `provenance.json`.\n\n"
            "`provenance.json` preserves every source decklist verbatim and its parsed source sections. "
            f"Auxiliary non-gameplay sections contain {auxiliary_cards} cards and remain there rather than being moved into a playable Commander zone.\n"
        )
        if output.exists():
            shutil.rmtree(output)
        temporary.replace(output)
    except BaseException:
        shutil.rmtree(temporary, ignore_errors=True)
        raise
    if args.web_registry:
        write_web_registry(rendered, args.web_registry)
    print(f"wrote {len(rendered)} decklists to {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
