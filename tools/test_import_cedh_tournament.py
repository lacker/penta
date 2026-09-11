"""Offline behavior tests for the bounded cEDH tournament importer."""

import json
import sys
import tempfile
import unittest
from pathlib import Path


sys.path.insert(0, str(Path(__file__).parent))
import import_cedh_tournament as importer  # noqa: E402


def deck_page(decklist: str) -> str:
    return f'<script>window.__data = {{"decklistText":{json.dumps(decklist)}}};</script>'


class CedhImporterTests(unittest.TestCase):
    def test_sections_keep_commanders_main_and_auxiliary_cards_separate(self) -> None:
        sections = importer.parse_sections(
            "~~Commanders~~\n1 Tymna the Weaver\n"
            "~~Mainboard~~\n1 Plains\n2 Island\n"
            "~~Stickers~~\n1 Name Sticker Goblin\n"
        )

        self.assertEqual(sections["Commanders"], {"Tymna the Weaver": 1})
        self.assertEqual(sections["Mainboard"], {"Plains": 1, "Island": 2})
        self.assertEqual(sections["Sideboard"], {})
        self.assertEqual(sections["Stickers"], {"Name Sticker Goblin": 1})

    def test_duplicate_cards_and_sections_are_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "duplicate card"):
            importer.parse_sections("~~Commanders~~\n1 Kinnan\n~~Mainboard~~\n1 Island\n1 Island\n")
        with self.assertRaisesRegex(ValueError, "duplicate source section"):
            importer.parse_sections("~~Commanders~~\n1 Kinnan\n~~Mainboard~~\n1 Island\n~~Commanders~~\n1 Tymna\n")

    def test_unreadable_lines_and_required_sections_are_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "unreadable decklist line"):
            importer.parse_sections("~~Commanders~~\nKinnan\n~~Mainboard~~\n1 Island\n")
        with self.assertRaisesRegex(ValueError, "missing source sections"):
            importer.parse_sections("~~Commanders~~\n1 Kinnan\n")

    def test_deck_payload_and_unavailable_provenance(self) -> None:
        entry = {
            "id": "RW50cnk6MQ==",
            "standing": 1,
            "player": {"name": "A Player"},
            "decklist": "https://example.test/deck",
            "commander": {"name": "Kinnan"},
        }
        imported = importer.provenance_entry(
            entry,
            deck_page("~~Commanders~~\n1 Kinnan\n~~Mainboard~~\n1 Island\n"),
            None,
        )
        self.assertEqual(imported["status"], "imported")
        self.assertEqual(imported["sections"]["Commanders"], {"Kinnan": 1})
        unavailable = importer.provenance_entry(entry, "<html>unavailable</html>", None)
        self.assertEqual(unavailable["status"], "unavailable")
        self.assertIn("decklistText", unavailable["reason"])

    def test_event_parser_enforces_the_bounded_entry_count(self) -> None:
        entries = [{"standing": index} for index in range(123)]
        document = f'<script>{{"tournament":{{"entries":{json.dumps(entries)}}}}}</script>'
        self.assertEqual(len(importer.event_entries(document)), 123)
        with self.assertRaisesRegex(ValueError, "expected exactly 123"):
            importer.event_entries('{"tournament":{"entries":[]}}')

    def test_web_registry_is_stable_and_excludes_unavailable_records(self) -> None:
        rendered = [
            ("nacional_002", "yaml", "Zulu", "Second imported deck"),
            ("nacional_001", "yaml", "Alpha", "First imported deck"),
        ]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "cedh-decks.json"
            importer.write_web_registry(rendered, path)
            self.assertEqual(
                path.read_text(),
                '{\n  "Alpha": "First imported deck",\n  "Zulu": "Second imported deck"\n}\n',
            )


if __name__ == "__main__":
    unittest.main()
