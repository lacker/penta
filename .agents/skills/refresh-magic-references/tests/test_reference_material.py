"""Regression tests for the generated Scryfall reference database.

The fixtures are deliberately tiny and entirely offline.  They exercise the
same compressed JSONL and manifest paths used by a real cache rebuild while
keeping the suite suitable for ``make check-fast``.
"""

from __future__ import annotations

import gzip
import importlib.util
import io
import json
import shutil
import sqlite3
import sys
import tempfile
import unittest
from contextlib import closing
from contextlib import contextmanager
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock


SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts/reference_material.py"


def load_reference_module():
    spec = importlib.util.spec_from_file_location("penta_reference_material", SCRIPT_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {SCRIPT_PATH}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


reference = load_reference_module()

ANKH_ORACLE_ID = "63c1eda1-3e6f-4e9c-adf3-a43164df98bb"
ALPHA_ANKH_ID = "f594b7aa-d44e-47c4-989b-565f881e25f1"
VMA_ANKH_ID = "a6f86a32-8863-45da-99be-ef886c89f2c5"
JAPANESE_ANKH_ID = "00000000-0000-0000-0000-000000000001"
ORACLELESS_ID = "018830b2-dff9-45f3-9cc2-dc5b2eec0e54"


def ankh_printing(
    *,
    scryfall_id: str,
    set_code: str,
    set_name: str,
    number: str,
    lang: str = "en",
):
    representative_marker = {
        "4ed": "JapaneseRepresentative",
        "lea": "AlphaRepresentative",
        "vma": "VmaRepresentative",
    }[set_code]
    return {
        "object": "card",
        "id": scryfall_id,
        "oracle_id": ANKH_ORACLE_ID,
        "name": "Ankh of Mishra",
        "lang": lang,
        "released_at": "1993-08-05" if set_code == "lea" else "2014-06-16",
        "layout": "normal",
        "mana_cost": "{2}",
        "cmc": 2,
        "type_line": "Artifact",
        "oracle_text": (
            "Whenever a land enters, Ankh of Mishra deals 2 damage to that "
            "land's controller."
        ),
        "colors": [],
        "color_identity": [],
        "produced_mana": [],
        "keywords": [representative_marker],
        "legalities": {"vintage": "legal"},
        "games": ["paper"],
        "reserved": False,
        "digital": set_code == "vma",
        "game_changer": False,
        "set": set_code,
        "set_name": set_name,
        "collector_number": number,
        "rarity": "rare",
        "scryfall_uri": f"https://scryfall.com/card/{set_code}/{number}",
        "rulings_uri": f"https://api.scryfall.com/cards/{scryfall_id}/rulings",
        "image_uris": {"normal": f"https://cards.scryfall.io/{scryfall_id}.jpg"},
    }


def oracleless_printing():
    return {
        "object": "card",
        "id": ORACLELESS_ID,
        "oracle_id": None,
        "name": "Jinnie Fay, Jetmir's Second // Jinnie Fay, Jetmir's Second",
        "lang": "en",
        "released_at": "2023-12-01",
        "layout": "reversible_card",
        "cmc": 3,
        "type_line": "Legendary Creature — Elf Druid",
        "colors": ["G", "R", "W"],
        "color_identity": ["G", "R", "W"],
        "keywords": [],
        "legalities": {},
        "games": ["paper"],
        "reserved": False,
        "digital": False,
        "game_changer": False,
        "set": "sld",
        "set_name": "Secret Lair Drop",
        "collector_number": "1556",
        "rarity": "rare",
        "scryfall_uri": "https://scryfall.com/card/sld/1556",
        "rulings_uri": f"https://api.scryfall.com/cards/{ORACLELESS_ID}/rulings",
        "card_faces": [
            {
                "name": "Jinnie Fay, Jetmir's Second",
                "oracle_id": "61fbaaf2-4286-4e9a-b9cb-aa31262b596a",
                "mana_cost": "{R/G}{G}{G/W}",
                "type_line": "Legendary Creature — Elf Druid",
                "oracle_text": "If you would create one or more tokens, instead create that many 2/2 green Cat creature tokens with haste or that many 3/1 green Dog creature tokens with vigilance.",
                "colors": ["G", "R", "W"],
            },
            {
                "name": "Jinnie Fay, Jetmir's Second",
                "oracle_id": "61fbaaf2-4286-4e9a-b9cb-aa31262b596a",
                "mana_cost": "{R/G}{G}{G/W}",
                "type_line": "Legendary Creature — Elf Druid",
                "oracle_text": "If you would create one or more tokens, instead create that many 2/2 green Cat creature tokens with haste or that many 3/1 green Dog creature tokens with vigilance.",
                "colors": ["G", "R", "W"],
            },
        ],
    }


def ruling():
    return {
        "object": "ruling",
        "oracle_id": ANKH_ORACLE_ID,
        "published_at": "2004-10-04",
        "source": "scryfall",
        "comment": "This ability triggers whenever any land enters.",
    }


def default_card_rows():
    return [
        ankh_printing(
            scryfall_id=ALPHA_ANKH_ID,
            set_code="lea",
            set_name="Limited Edition Alpha",
            number="230",
        ),
        oracleless_printing(),
        ankh_printing(
            scryfall_id=VMA_ANKH_ID,
            set_code="vma",
            set_name="Vintage Masters",
            number="263",
        ),
        ankh_printing(
            scryfall_id=JAPANESE_ANKH_ID,
            set_code="4ed",
            set_name="Fourth Edition",
            number="294",
            lang="ja",
        ),
    ]


def write_resource(reference_dir: Path, name: str, rows: list[dict]):
    relative_path = reference.RESOURCE_FILES[name]
    path = reference_dir / relative_path
    path.parent.mkdir(parents=True, exist_ok=True)
    with gzip.open(path, "wt", encoding="utf-8") as output:
        for row in rows:
            output.write(json.dumps(row, ensure_ascii=False, separators=(",", ":")))
            output.write("\n")
    return {
        "path": relative_path,
        "source_url": f"https://data.scryfall.io/{name}.jsonl.gz",
        "source_updated_at": "2026-08-11T21:05:35Z",
        "retrieved_at": "2026-08-11T22:00:00Z",
        "size": path.stat().st_size,
        "sha256": reference.sha256_file(path),
        "record_count": len(rows),
        "format": "application/jsonl+gzip",
    }


class ReferenceConfigurationTests(unittest.TestCase):
    def test_default_resources_and_database_identity_use_default_cards(self):
        self.assertEqual(reference.SCHEMA_VERSION, 1)
        self.assertEqual(reference.DATABASE_SCHEMA_VERSION, 2)
        self.assertEqual(
            reference.DEFAULT_RESOURCES,
            (
                "comprehensive-rules",
                "eternal-central-rules",
                "default-cards",
                "rulings",
            ),
        )
        self.assertEqual(reference.DATABASE_INPUTS, ("default-cards", "rulings"))
        self.assertEqual(
            reference.DATABASE_REPRESENTATIVE_POLICY,
            "prefer-english-then-minimum-scryfall-id-binary",
        )
        self.assertEqual(
            reference.DATABASE_MANIFEST_KEY, "scryfall-index-schema-v2"
        )
        self.assertEqual(
            reference.DATABASE_RELATIVE_PATH, "indexes/schema-v2/scryfall.sqlite"
        )
        self.assertEqual(
            reference.normalize_targets([]), list(reference.DEFAULT_RESOURCES)
        )

    def test_implicit_status_checks_default_sources_and_the_index(self):
        args = reference.build_parser().parse_args(["status"])
        seen: list[str] = []

        def inspect_resource(name, **_kwargs):
            seen.append(name)
            return reference.ResourceStatus(name, "current", name, "fixture"), None

        index_status = reference.ResourceStatus(
            reference.DATABASE_RESOURCE_NAME,
            "current",
            reference.DATABASE_RELATIVE_PATH,
            "fixture",
        )
        with (
            tempfile.TemporaryDirectory() as temporary,
            mock.patch.object(reference, "find_repo_root", return_value=Path(temporary)),
            mock.patch.object(
                reference, "resolve_reference_dir", return_value=Path(temporary)
            ),
            mock.patch.object(reference, "cache_lock", return_value=_read_lock()),
            mock.patch.object(
                reference,
                "load_manifest",
                return_value={"resources": {}, "derived": {}},
            ),
            mock.patch.object(reference, "inspect_resource", side_effect=inspect_resource),
            mock.patch.object(
                reference, "inspect_scryfall_database", return_value=index_status
            ) as inspect_index,
            mock.patch.object(reference, "print_statuses"),
        ):
            self.assertEqual(reference.command_status(args), 0)

        self.assertEqual(seen, list(reference.DEFAULT_RESOURCES))
        inspect_index.assert_called_once()

    def test_implicit_fetch_maintains_the_default_cards_index(self):
        args = reference.build_parser().parse_args(
            ["fetch", "--approve-shared-write"]
        )
        seen: list[str] = []

        def inspect_resource(name, **_kwargs):
            seen.append(name)
            return reference.ResourceStatus(name, "current", name, "fixture"), None

        manifest = {"schema_version": 1, "resources": {}, "derived": {}}
        with (
            tempfile.TemporaryDirectory() as temporary,
            mock.patch.object(reference, "find_repo_root", return_value=Path(temporary)),
            mock.patch.object(
                reference, "resolve_reference_dir", return_value=Path(temporary)
            ),
            mock.patch.object(reference, "cache_lock", return_value=_write_lock()),
            mock.patch.object(reference, "load_manifest", return_value=manifest),
            mock.patch.object(reference, "inspect_resource", side_effect=inspect_resource),
            mock.patch.object(reference, "local_problem", return_value=None),
            mock.patch.object(reference, "ensure_scryfall_database") as ensure_index,
        ):
            with redirect_stdout(io.StringIO()):
                self.assertEqual(reference.command_fetch(args), 0)

        self.assertEqual(seen, list(reference.DEFAULT_RESOURCES))
        ensure_index.assert_called_once_with(Path(temporary), manifest, force=False)


class ReferenceDatabaseTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory()
        cls.reference_dir = Path(cls.temporary.name)
        cls.manifest = {
            "schema_version": reference.SCHEMA_VERSION,
            "resources": {
                "default-cards": write_resource(
                    cls.reference_dir, "default-cards", default_card_rows()
                ),
                "rulings": write_resource(cls.reference_dir, "rulings", [ruling()]),
            },
            "derived": {},
        }
        inputs = reference.database_input_records(cls.reference_dir, cls.manifest)
        cls.record = reference.atomic_build_scryfall_database(
            cls.reference_dir, inputs
        )
        cls.manifest["derived"][reference.DATABASE_MANIFEST_KEY] = cls.record
        cls.database_path = cls.reference_dir / reference.DATABASE_RELATIVE_PATH

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def connect(self):
        @contextmanager
        def connection():
            database = sqlite3.connect(
                f"{self.database_path.resolve().as_uri()}?mode=ro", uri=True
            )
            try:
                database.execute("PRAGMA query_only = ON")
                yield database
            finally:
                database.close()

        return connection()

    def test_build_indexes_printings_without_duplicating_oracle_cards(self):
        with self.connect() as database:
            self.assertEqual(database.execute("PRAGMA user_version").fetchone(), (2,))
            metadata = dict(database.execute("SELECT key, value FROM metadata"))
            counts = {
                table: database.execute(f"SELECT count(*) FROM {table}").fetchone()[0]
                for table in ("cards", "printings", "rulings")
            }
            orphan = database.execute(
                "SELECT card_id, oracle_id FROM printings WHERE scryfall_id = ?",
                (ORACLELESS_ID,),
            ).fetchone()
            policy_violations = reference.representative_policy_violations(database)

        self.assertEqual(counts, {"cards": 1, "printings": 4, "rulings": 1})
        self.assertEqual(orphan, (None, None))
        self.assertEqual(metadata["database_schema_version"], "2")
        self.assertEqual(metadata["source.default-cards.records"], "4")
        self.assertEqual(
            metadata["indexed.cards.representative_policy"],
            reference.DATABASE_REPRESENTATIVE_POLICY,
        )
        self.assertEqual(
            metadata["indexed.cards.representative_policy_violations"], "0"
        )
        self.assertEqual(policy_violations, 0)
        self.assertEqual(metadata["indexed.printings.without_oracle_id"], "1")
        self.assertEqual(metadata["indexed.printings.unlinked"], "0")
        self.assertIn("input.default-cards.sha256", metadata)
        self.assertNotIn("input.oracle-cards.sha256", metadata)
        self.assertEqual(self.record["table_counts"]["cards"], 1)
        self.assertEqual(self.record["table_counts"]["printings"], 4)
        self.assertEqual(self.record["representative_policy_violations"], 0)
        if self.record["fts5"]:
            self.assertEqual(self.record["table_counts"]["card_search"], 1)

        status = reference.inspect_scryfall_database(
            self.reference_dir, self.manifest
        )
        self.assertEqual(status.status, "current", status.detail)

    def test_lookup_exact_printing_by_set_and_collector_number(self):
        with self.connect() as database:
            row = database.execute(
                """
                SELECT
                    p.scryfall_id, p.oracle_id, p.name, p.set_code,
                    p.collector_number, c.oracle_id
                FROM printings AS p INDEXED BY idx_printings_set_collector
                LEFT JOIN cards AS c USING (card_id)
                WHERE p.set_code = ? AND p.collector_number = ? AND p.lang = ?
                """,
                ("lea", "230", "en"),
            ).fetchone()

        self.assertEqual(
            row,
            (
                ALPHA_ANKH_ID,
                ANKH_ORACLE_ID,
                "Ankh of Mishra",
                "lea",
                "230",
                ANKH_ORACLE_ID,
            ),
        )

    def test_lookup_exact_printing_by_scryfall_id_including_oracleless_rows(self):
        with self.connect() as database:
            alpha = database.execute(
                """
                SELECT name, set_code, collector_number, oracle_id, card_id
                FROM printings INDEXED BY idx_printings_scryfall_id
                WHERE scryfall_id = ?
                """,
                (ALPHA_ANKH_ID,),
            ).fetchone()
            oracleless = database.execute(
                """
                SELECT name, set_code, collector_number, oracle_id, card_id
                FROM printings INDEXED BY idx_printings_scryfall_id
                WHERE scryfall_id = ?
                """,
                (ORACLELESS_ID,),
            ).fetchone()

        self.assertEqual(
            alpha[:4], ("Ankh of Mishra", "lea", "230", ANKH_ORACLE_ID)
        )
        self.assertIsNotNone(alpha[4])
        self.assertEqual(
            oracleless,
            (
                "Jinnie Fay, Jetmir's Second // Jinnie Fay, Jetmir's Second",
                "sld",
                "1556",
                None,
                None,
            ),
        )

    def test_lookup_exact_printing_by_normalized_name_and_set(self):
        with self.connect() as database:
            rows = database.execute(
                """
                SELECT scryfall_id, name, set_code, collector_number, lang
                FROM printings INDEXED BY idx_printings_normalized_name_set
                WHERE normalized_name = ? AND set_code = ?
                ORDER BY printing_id
                """,
                ("ankh of mishra", "lea"),
            ).fetchall()

        self.assertEqual(
            rows,
            [(ALPHA_ANKH_ID, "Ankh of Mishra", "lea", "230", "en")],
        )

    def test_printing_lookup_indexes_have_queryable_prefixes(self):
        expected = {
            "idx_printings_scryfall_id": ["scryfall_id"],
            "idx_printings_set_collector": [
                "set_code",
                "collector_number",
                "printing_id",
            ],
            "idx_printings_normalized_name_set": [
                "normalized_name",
                "set_code",
                "printing_id",
            ],
            "idx_printings_card_set_collector": [
                "card_id",
                "set_code",
                "collector_number",
                "printing_id",
            ],
        }
        with self.connect() as database:
            index_rows = {
                row[1]: row for row in database.execute("PRAGMA index_list(printings)")
            }
            actual = {
                name: [row[2] for row in database.execute(f"PRAGMA index_info({name})")]
                for name in expected
            }

        self.assertEqual(actual, expected)
        self.assertEqual(index_rows["idx_printings_scryfall_id"][2], 1)

    def test_printing_card_link_uses_no_action_on_delete(self):
        with self.connect() as database:
            foreign_keys = database.execute(
                "PRAGMA foreign_key_list(printings)"
            ).fetchall()

        card_link = next(row for row in foreign_keys if row[3] == "card_id")
        self.assertEqual(card_link[2], "cards")
        self.assertEqual(card_link[4], "card_id")
        self.assertEqual(card_link[6], "NO ACTION")

    def test_representative_prefers_english_then_smallest_uuid_regardless_of_order(
        self,
    ):
        expected_card = (
            VMA_ANKH_ID,
            "en",
            "vma",
            "263",
            1,
            '["VmaRepresentative"]',
        )
        expected_keywords = ("VmaRepresentative",)

        forward = representative_snapshot(self.database_path, self.record["fts5"])
        with tempfile.TemporaryDirectory() as temporary:
            reversed_dir = Path(temporary)
            reversed_manifest = {
                "schema_version": reference.SCHEMA_VERSION,
                "resources": {
                    "default-cards": write_resource(
                        reversed_dir,
                        "default-cards",
                        list(reversed(default_card_rows())),
                    ),
                    "rulings": write_resource(reversed_dir, "rulings", [ruling()]),
                },
                "derived": {},
            }
            reversed_record = reference.atomic_build_scryfall_database(
                reversed_dir,
                reference.database_input_records(reversed_dir, reversed_manifest),
            )
            backward = representative_snapshot(
                reversed_dir / reference.DATABASE_RELATIVE_PATH,
                reversed_record["fts5"],
            )

        for snapshot in (forward, backward):
            card, keywords, fts_matches = snapshot
            self.assertEqual(card, expected_card)
            self.assertEqual(keywords, expected_keywords)
            if fts_matches is not None:
                self.assertEqual(fts_matches, (1, 0, 0))
        self.assertEqual(forward, backward)

    def test_policy_validator_rejects_tampered_representatives(self):
        cases = (
            ("unlinked UUID and language pair", VMA_ANKH_ID, "ja"),
            ("valid representative with a better candidate", ALPHA_ANKH_ID, "en"),
        )
        for label, representative_id, representative_lang in cases:
            with self.subTest(label), tempfile.TemporaryDirectory() as temporary:
                tampered_dir = Path(temporary)
                tampered_path = tampered_dir / reference.DATABASE_RELATIVE_PATH
                tampered_path.parent.mkdir(parents=True)
                shutil.copyfile(self.database_path, tampered_path)

                with closing(sqlite3.connect(tampered_path)) as database:
                    database.execute(
                        """
                        UPDATE cards
                        SET representative_scryfall_id = ?, representative_lang = ?
                        WHERE oracle_id = ?
                        """,
                        (representative_id, representative_lang, ANKH_ORACLE_ID),
                    )
                    violations = reference.representative_policy_violations(database)
                    database.execute(
                        """
                        UPDATE metadata SET value = ?
                        WHERE key = 'indexed.cards.representative_policy_violations'
                        """,
                        (str(violations),),
                    )
                    database.commit()

                self.assertEqual(violations, 1)
                tampered_manifest = json.loads(json.dumps(self.manifest))
                tampered_record = tampered_manifest["derived"][
                    reference.DATABASE_MANIFEST_KEY
                ]
                tampered_record["size"] = tampered_path.stat().st_size
                tampered_record["sha256"] = reference.sha256_file(tampered_path)

                status = reference.inspect_scryfall_database(
                    tampered_dir, tampered_manifest
                )
                self.assertEqual(status.status, "corrupt")
                self.assertIn("representative-policy violations", status.detail)

    def test_changed_default_cards_input_makes_the_index_stale(self):
        changed_manifest = json.loads(json.dumps(self.manifest))
        changed_manifest["resources"]["default-cards"]["sha256"] = "0" * 64

        status = reference.inspect_scryfall_database(
            self.reference_dir, changed_manifest
        )

        self.assertEqual(status.status, "stale")
        self.assertIn("default-cards", status.detail)


def representative_snapshot(database_path: Path, has_fts: bool):
    with closing(
        sqlite3.connect(f"{database_path.resolve().as_uri()}?mode=ro", uri=True)
    ) as database:
        database.execute("PRAGMA query_only = ON")
        card = database.execute(
            """
            SELECT
                representative_scryfall_id, representative_lang,
                representative_set_code,
                representative_collector_number, representative_digital,
                keywords_json
            FROM cards
            WHERE oracle_id = ?
            """,
            (ANKH_ORACLE_ID,),
        ).fetchone()
        keywords = tuple(
            row[0]
            for row in database.execute(
                """
                SELECT keyword
                FROM card_keywords
                JOIN cards USING (card_id)
                WHERE oracle_id = ?
                ORDER BY keyword
                """,
                (ANKH_ORACLE_ID,),
            )
        )
        fts_matches = None
        if has_fts:
            fts_matches = tuple(
                database.execute(
                    "SELECT count(*) FROM card_search WHERE card_search MATCH ?",
                    (marker,),
                ).fetchone()[0]
                for marker in (
                    "vmarepresentative",
                    "alpharepresentative",
                    "japaneserepresentative",
                )
            )
    return card, keywords, fts_matches


class LegacyIndexTests(unittest.TestCase):
    def test_schema_v1_index_is_planned_beside_schema_v2(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            legacy_dir = root / "legacy"
            shared_dir = root / "shared"
            legacy_dir.mkdir()
            database_path = legacy_dir / reference.DATABASE_NAME
            with closing(sqlite3.connect(database_path)) as database:
                database.execute("PRAGMA user_version = 1")
                database.execute(
                    "CREATE TABLE metadata(key TEXT PRIMARY KEY, value TEXT NOT NULL)"
                )
                database.execute("CREATE TABLE legacy_rows(value TEXT)")
                database.executemany(
                    "INSERT INTO metadata(key, value) VALUES (?, ?)",
                    [
                        ("database_schema_version", "1"),
                        ("fts5", "0"),
                        ("input.oracle-cards.sha256", "a" * 64),
                        ("input.oracle-cards.record_count", "1"),
                        ("input.rulings.sha256", "b" * 64),
                        ("input.rulings.record_count", "1"),
                    ],
                )
                database.commit()

            legacy_record = {
                "path": reference.DATABASE_NAME,
                "schema_version": 1,
                "size": database_path.stat().st_size,
                "sha256": reference.sha256_file(database_path),
                "fts5": False,
                "inputs": {
                    "oracle-cards": {
                        "path": "scryfall-oracle-cards.jsonl.gz",
                        "sha256": "a" * 64,
                        "record_count": 1,
                    },
                    "rulings": {
                        "path": "scryfall-rulings.jsonl.gz",
                        "sha256": "b" * 64,
                        "record_count": 1,
                    },
                },
                "table_counts": {"legacy_rows": 0},
            }
            existing_v2 = {"schema_version": 2, "path": reference.DATABASE_RELATIVE_PATH}
            target_derived = {reference.DATABASE_MANIFEST_KEY: existing_v2}

            plans = reference.legacy_database_plans(
                legacy_dir,
                {reference.DATABASE_RESOURCE_NAME: legacy_record},
                shared_dir,
                target_derived,
            )

        self.assertEqual(len(plans), 1)
        schema_version, key, _source, destination, translated = plans[0]
        self.assertEqual(schema_version, 1)
        self.assertEqual(key, "scryfall-index-schema-v1")
        self.assertEqual(
            destination, shared_dir / "indexes/schema-v1/scryfall.sqlite"
        )
        self.assertEqual(translated["path"], "indexes/schema-v1/scryfall.sqlite")
        self.assertIs(target_derived[reference.DATABASE_MANIFEST_KEY], existing_v2)


@contextmanager
def _read_lock():
    yield None


class _FixtureLock:
    def update(self, _phase, **_details):
        pass


@contextmanager
def _write_lock():
    yield _FixtureLock()


if __name__ == "__main__":
    unittest.main()
