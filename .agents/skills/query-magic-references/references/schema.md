# Scryfall SQLite schema and query patterns

The generated database is a schema-versioned artifact in the Git-common cache
shared by every worktree in a clone. Resolve its absolute path rather than
hardcoding a worktree-relative location:

```sh
REFERENCE_SCRIPT=.agents/skills/refresh-magic-references/scripts/reference_material.py
SCRYFALL_DB="$(python3 "$REFERENCE_SCRIPT" path scryfall-index)"
sqlite3 -readonly -header -column "$SCRYFALL_DB"
```

Schema version 2 derives from the cached `default-cards` and `rulings` Scryfall
JSONL sources; those compressed files remain canonical. `default-cards`
contains one default-language object per printing: English when available,
otherwise the printing's only available language. Resolve either source with
the refresh script's `path` command when an unmodeled field is needed. Other
schema versions may coexist for worktrees whose skill implementations differ.

## Tables

`metadata(key, value)` records the schema and build time, SQLite and FTS5
versions, input paths at build time, input checksums/source timestamps, source
and table counts, printings without Oracle IDs, unlinked printings, removed
duplicate rulings, orphan rulings, and the stable representative-card policy
with its validated violation count.
A migrated database can retain its old build-time path strings; locate current
inputs through the refresh script rather than treating those strings as paths.

`cards` has one row per non-null Oracle ID. It keeps Oracle-level
characteristics compact while selecting one source printing as its
representative; use `printings`, not representative fields, for exact printing
identity or printing history.

- IDs: `card_id`, `oracle_id`, `representative_scryfall_id`,
  `representative_lang`
- Lookup: `name`, `normalized_name`, `layout`
- Rules: `mana_cost`, `mana_value`, `type_line`, `oracle_text`, `power`,
  `toughness`, `loyalty`, `defense`
- Compact JSON: `colors_json`, `color_identity_json`, `produced_mana_json`,
  `keywords_json`, `legalities_json`, `games_json`
- Flags: `reserved`, `representative_digital`, `game_changer`
- Representative printing: `representative_released_at`,
  `representative_set_code`,
  `representative_set_name`, `representative_collector_number`,
  `representative_rarity`
- Links: `representative_scryfall_uri`, `rulings_uri`,
  `representative_image_uri`

`normalized_name` is Unicode NFKC plus case folding. It is indexed but not
unique; Scryfall contains cards and tokens with colliding names.

The representative is selected independently of bulk-file order: prefer an
English printing, then the lexicographically smallest Scryfall UUID. This gives
Oracle-level rows and FTS a stable source object; it is still only a compact
representative, so use `printings` for exact printing work.

`printings` has one row per object in `default-cards`:

- IDs and relationship: `printing_id`, `scryfall_id`, nullable `oracle_id`, and
  nullable `card_id`
- Lookup: canonical Scryfall `name`, `normalized_name`, `lang`, `layout`
- Set identity: `released_at`, `set_code`, `set_name`, `collector_number`,
  `rarity`
- Printing flags: `digital`, `promo`, `reprint`, `variation`
- Compact JSON: `games_json`, `finishes_json`
- Credit and links: `artist`, `scryfall_uri`, `image_uri`

`scryfall_id` identifies an exact printing and is unique. `card_id` connects a
printing to its Oracle-level `cards` row, but remains null for Scryfall objects
without an Oracle ID. Preserve such rows with a `LEFT JOIN` when auditing the
printing corpus. `collector_number` is opaque text: compare it exactly and do
not coerce it to an integer or assume one global sortable grammar. Set and
collector lookups can return multiple rows, so retain `scryfall_id` and `lang`
until identity is resolved.

`name` is Scryfall's canonical card name; localized `printed_name` is not
indexed. `image_uri` is the parent printing object's normal image when present.
Multifaced printings commonly keep images on their faces instead, and the
Oracle-level `card_faces.image_uri` belongs only to the selected representative
printing. Resolve the `default-cards` payload or Scryfall online when an exact
printing's localized name or face image is required.

`card_faces` contains face-specific fields keyed by `(card_id, face_index)`.
Parent fields can be absent for multifaced cards, so inspect faces when needed.

`card_names` covers both primary and face names. `name_index = -1` and
`name_kind = 'card'` identify the parent name; nonnegative indexes are faces.

`rulings` contains unique `(oracle_id, published_at, source, comment)` content.
The builder removes exact duplicate source rows using the internal
`fingerprint` column. Join it to `cards` through `oracle_id`; a ruling whose
Oracle ID is absent from the card snapshot remains available as an orphan row.

`card_keywords(card_id, keyword)`, `card_colors(card_id, kind, color)`, and
`card_parts` provide indexed relationships. Color `kind` is `color`,
`identity`, or `produced`. `card_parts` models Scryfall `all_parts` links.

`card_search` and `ruling_search` are optional contentless FTS5 indexes. Their
rowids map to `cards.card_id` and `rulings.ruling_id`; select display text from
the base tables, not the FTS columns. Check the `fts5` key in `metadata` before
relying on them.

Important ordinary indexes include unique exact lookup on
`printings.scryfall_id`, set-and-collector lookup, normalized printing name plus
set, printing history by `card_id`, normalized Oracle and face names, keyword
and color relationships, related Scryfall IDs, and rulings by Oracle ID and
date. Use `EXPLAIN QUERY PLAN` when a new batch pattern should exercise one of
them.

## Useful queries

### Build provenance and counts

```sql
SELECT key, value
FROM metadata
ORDER BY key;
```

### Exact printing by Scryfall UUID

Use `printings.scryfall_id` when a card definition supplies a `CardArt` UUID or
another exact printing identifier:

```sql
SELECT
  p.scryfall_id,
  p.name,
  upper(p.set_code) AS set_code,
  p.collector_number,
  p.lang,
  p.artist,
  p.scryfall_uri
FROM printings AS p
WHERE p.scryfall_id = 'd573ef03-4730-45aa-93dd-e45ac1dbaf4a';
```

No Oracle-level join is needed to resolve exact printing metadata. Join
`cards` through `card_id` only when the same query also needs Oracle text or
characteristics.

### Exact printing name and set

Use the NFKC/case-folded canonical full card name and lowercase set code when a
UUID is missing or stale:

```sql
SELECT
  p.scryfall_id,
  p.name,
  p.set_code,
  p.collector_number,
  p.lang
FROM printings AS p
WHERE p.normalized_name = 'wall of stone'
  AND p.set_code = 'lea'
ORDER BY p.scryfall_id;
```

This uses `idx_printings_normalized_name_set`. A source lookup may supply only
one face name while `printings.name` contains a combined multiface name; for
that case, resolve the face through `card_names` and then join `printings` by
`card_id`, as in the printing-history query below. A localized `printed_name`
requires the raw `default-cards` payload or Scryfall online.

### Exact set and collector number

Set codes are stored in Scryfall's lowercase form. Collector numbers are
verbatim text, including suffixes, prefixes, punctuation, and symbols:

```sql
SELECT
  p.scryfall_id,
  p.name,
  p.set_code,
  p.collector_number,
  p.lang,
  p.released_at
FROM printings AS p
WHERE p.set_code = 'lea'
  AND p.collector_number = '161'
ORDER BY p.lang, p.scryfall_id;
```

Do not write `CAST(collector_number AS INTEGER)` as a general ordering or
identity rule. Retain the exact collector text and Scryfall UUID in results.

### Printing history for a card or face name

Resolve a primary or face name through `card_names`, then join every indexed
printing for the resulting Oracle-level card:

```sql
SELECT DISTINCT
  c.name,
  p.scryfall_id,
  p.released_at,
  p.set_code,
  p.collector_number,
  p.lang
FROM card_names AS n
JOIN cards AS c USING (card_id)
JOIN printings AS p USING (card_id)
WHERE n.normalized_name = 'lightning bolt'
ORDER BY p.released_at, p.set_code, p.scryfall_id;
```

`default-cards` represents each printing in its default language, not every
localized object.

### Printings without Oracle rows

Some Scryfall printing objects intentionally have no Oracle ID. Keep them in
corpus audits instead of losing them to an inner join:

```sql
SELECT
  p.scryfall_id,
  p.name,
  p.set_code,
  p.collector_number,
  p.lang
FROM printings AS p
LEFT JOIN cards AS c USING (card_id)
WHERE c.card_id IS NULL
ORDER BY p.set_code, p.scryfall_id
LIMIT 50;
```

### Exact card or face name with rulings

Use the NFKC/case-folded name literal. Most English card names simply become
lowercase.

```sql
SELECT
  c.name,
  c.mana_cost,
  c.type_line,
  c.oracle_text,
  r.published_at,
  r.source,
  r.comment
FROM card_names AS n
JOIN cards AS c USING (card_id)
LEFT JOIN rulings AS r USING (oracle_id)
WHERE n.normalized_name = 'chaos orb'
ORDER BY c.name, r.published_at, r.ruling_id;
```

Because a name can match more than one object, retain identifying fields such
as `oracle_id`, `layout`, or `type_line` until ambiguity is resolved.

### Multifaced card

```sql
SELECT
  c.name AS card_name,
  f.face_index,
  f.name AS face_name,
  f.mana_cost,
  f.type_line,
  f.oracle_text
FROM card_names AS n
JOIN cards AS c USING (card_id)
JOIN card_faces AS f USING (card_id)
WHERE n.normalized_name = 'fire // ice'
ORDER BY c.card_id, f.face_index;
```

### Full-text card search

```sql
SELECT
  c.name,
  c.type_line,
  c.oracle_text,
  bm25(card_search) AS rank
FROM card_search
JOIN cards AS c ON c.card_id = card_search.rowid
WHERE card_search MATCH 'draw AND discard'
ORDER BY rank
LIMIT 20;
```

FTS aggregates parent and face names, types, Oracle text, and keywords into one
search row per card. Use quoted FTS phrases for adjacent words. Use `LIKE` on a
base column for punctuation-heavy strings such as mana symbols.

### Full-text ruling search

```sql
SELECT c.name, r.published_at, r.comment, bm25(ruling_search) AS rank
FROM ruling_search
JOIN rulings AS r ON r.ruling_id = ruling_search.rowid
LEFT JOIN cards AS c USING (oracle_id)
WHERE ruling_search MATCH 'copy AND target'
ORDER BY rank
LIMIT 20;
```

If FTS5 is unavailable, use a bounded fallback:

```sql
WITH searchable AS (
  SELECT
    c.card_id,
    c.name,
    c.type_line,
    lower(
      coalesce(c.oracle_text, '') || char(10) ||
      coalesce(group_concat(f.oracle_text, char(10)), '')
    ) AS search_text
  FROM cards AS c
  LEFT JOIN card_faces AS f USING (card_id)
  GROUP BY c.card_id
)
SELECT name, type_line
FROM searchable
WHERE search_text LIKE '%draw%' AND search_text LIKE '%discard%'
ORDER BY name
LIMIT 20;
```

### Characteristics, legality, keyword, and color

Legalities stay as compact JSON because normalizing every format would add
hundreds of thousands of rows. JSON scans over the card table are inexpensive.

```sql
SELECT DISTINCT c.name, c.mana_value, c.type_line
FROM cards AS c
JOIN card_keywords AS k USING (card_id)
JOIN card_colors AS color USING (card_id)
WHERE json_extract(c.legalities_json, '$.vintage') = 'legal'
  AND k.keyword = 'Flying'
  AND color.kind = 'identity'
  AND color.color = 'U'
ORDER BY c.name
LIMIT 50;
```

Scryfall format legality is not automatically equivalent to Eternal Central
93/94 legality. Use the cached EC rules for that determination.

### Related card parts

```sql
SELECT c.name AS parent, p.component, p.name AS part, p.type_line
FROM cards AS c
JOIN card_parts AS p USING (card_id)
WHERE c.normalized_name = 'hanweir battlements';
```

### Batch exact-printing lookup

Use a requested CTE for a bounded audit of source UUIDs. The `LEFT JOIN`
retains unknown identifiers so missing metadata is visible:

```sql
WITH requested(scryfall_id) AS (
  VALUES
    ('d573ef03-4730-45aa-93dd-e45ac1dbaf4a'),
    ('f594b7aa-d44e-47c4-989b-565f881e25f1')
)
SELECT
  requested.scryfall_id AS requested_id,
  p.name,
  p.set_code,
  p.collector_number,
  p.lang
FROM requested
LEFT JOIN printings AS p USING (scryfall_id)
ORDER BY requested.scryfall_id;
```

### Batch exact-name lookup

```sql
WITH requested(normalized_name) AS (
  VALUES ('black lotus'), ('chaos orb'), ('time vault')
)
SELECT requested.normalized_name, c.oracle_id, c.name, c.type_line, c.oracle_text
FROM requested
LEFT JOIN card_names AS n USING (normalized_name)
LEFT JOIN cards AS c USING (card_id)
ORDER BY requested.normalized_name, c.card_id;
```

### Inspect query planning

```sql
EXPLAIN QUERY PLAN
SELECT p.name, p.set_code, p.collector_number
FROM printings AS p
WHERE p.scryfall_id = 'd573ef03-4730-45aa-93dd-e45ac1dbaf4a';
```

Use indexed equality on printing UUID, set plus collector number, normalized
name, Oracle ID, keyword, and color relationships. Use FTS for prose. Project
only needed columns and add `LIMIT` while exploring.

## Interpretation limits

- `default-cards` supplies one default-language object per printing, not every
  localized object. Use the optional `all-cards` payload or Scryfall online for
  every language.
- `printings.card_id` and `printings.oracle_id` can be null. A printing lookup
  does not require an Oracle-level row.
- Collector numbers are opaque text and can contain prefixes, suffixes,
  punctuation, or symbols.
- Localized `printed_name` and per-printing face images are not indexed;
  `card_faces` describes only the selected Oracle-level representative.
- The representative printing fields on `cards` are conveniences, not exact
  printing-history selectors, and do not prove format legality.
- Current Oracle text and rulings may intentionally differ from Penta's
  historical adaptations.
- Prices are intentionally not indexed as a freshness guarantee.
- For unmodeled fields, stream the retained JSONL or use Scryfall online.
