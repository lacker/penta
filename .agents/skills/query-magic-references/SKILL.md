---
name: query-magic-references
description: "Query this repository's optional generated Scryfall SQLite index for exact printings, set and collector metadata, card characteristics, faces, Oracle text, legalities, keywords, colors, related parts, and rulings. Use for fast UUID or name lookups, full-text searches, joins, aggregate analysis, batch audits, or comparisons while developing Penta. Do not use it as the authority for Comprehensive Rules, live prices, or printing languages outside the indexed sources; refresh the cache or use an authoritative online source when necessary."
---

# Query Magic References

Use the clone-wide, schema-versioned Scryfall index as a read-only development
reference. It is shared by all linked worktrees. Keep results narrow enough to
inspect; never dump the database into model context.

## Start safely

Read [the schema reference](.agents/skills/query-magic-references/references/schema.md)
before composing a nontrivial query. It documents the schema, indexes, FTS
behavior, examples, and interpretation limits. Like the script paths below,
this path is written from the repository root, so it resolves the same way
however the skill was loaded — including through `.claude/skills`.

Resolve the database path instead of constructing or hardcoding it:

```sh
REFERENCE_SCRIPT=.agents/skills/refresh-magic-references/scripts/reference_material.py
SCRYFALL_DB="$(python3 "$REFERENCE_SCRIPT" path scryfall-index)"
```

Check the database against its cached inputs without networking or refreshing:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py status scryfall-index
```

Path resolution and this local status check are low-friction reads. Use the
database if it is current relative to its inputs; do not refresh merely because
the worktree is new. For a genuinely freshness-sensitive task, separately
compare the inputs with Scryfall:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py status default-cards rulings
```

If the database is missing, corrupt, or unavailable for the required schema,
use `$refresh-magic-references` to repair it with explicit human approval. Only
refresh intact source data when the current task truly needs newer material.
If shared mutation is unavailable or unwarranted, use retained source data or
an authoritative online source instead.

## Choose the narrowest query

- Use `printings.scryfall_id` for an exact printing UUID.
- Use `(printings.normalized_name, printings.set_code)` when an exact UUID is
  absent or stale, retaining collector number and UUID in the result.
- Use `printings` for set, collector-number, language, rarity, artist, and
  parent-image metadata. Keep `scryfall_id` in projected results when a lookup
  can match more than one printing.
- Use `card_names.normalized_name` for an exact primary or face name.
- Use `cards` for Oracle-level characteristics and compact JSON fields.
- Use `card_faces` for face-specific cost, type, text, or stats.
- Join `printings` to `cards` through nullable `card_id`, and join `cards` to
  `rulings` through `oracle_id`.
- Use `card_keywords`, `card_colors`, and `card_parts` for relationships.
- Use `card_search` or `ruling_search` with `MATCH` for words and concepts.
- Use bounded `LIKE` queries when FTS5 is unavailable or punctuation matters.

Open the database read-only:

```sh
sqlite3 -readonly -header -column "$SCRYFALL_DB"
```

Use `-json` for structured consumption. Project only needed columns, run
`COUNT(*)` before broad extraction, add `LIMIT` while exploring, and use
`EXPLAIN QUERY PLAN` for slow or repeated batch queries. Never modify the
generated database directly.

## Interpret results carefully

`default-cards` supplies one default-language Scryfall object per printing:
English when available, otherwise the printing's only available language. It
does not contain every localized object; use `all-cards` or Scryfall online when
that distinction matters. Some Scryfall objects have no Oracle ID and therefore
have a `printings` row with no related `cards` row; preserve them with a
`LEFT JOIN` instead of silently dropping them.

Collector numbers are opaque text. Compare them exactly for identity, retain
suffixes and symbols, and do not assume the entire Scryfall corpus can be
sorted by parsing them as integers. Printing metadata and Scryfall legalities
do not establish Old School legality, and Scryfall format legality is not a
substitute for Eternal Central's rules. Oracle text may intentionally differ
from Penta's adapted historical behavior. Resolve compressed bulk-file paths
through the refresh script for unmodeled fields such as localized
`printed_name` values and exact-printing face images. Do not substitute
representative `card_faces` images for a different printing. Use an
appropriately live source for prices.
