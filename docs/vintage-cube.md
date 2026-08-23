# Cube: Vintage implementation roadmap

The MTGO Vintage Cube is a 534-card singleton list, recorded verbatim in
[`src/formats/cubes/vintage.rs`](../src/formats/cubes/vintage.rs) as it stood on
2026-08-19. A cube is re-tuned between runs, so the pool is a dated snapshot
rather than a claim about what is current.

`Format::VintageCube` takes its legality from that list. This is the first
format here that is not a set window: a card is legal because the cube names
it, not because of where it was printed. Six names in the retrieved list match
no card Scryfall knows and were left out; the module records which.

## Current stage

- No decks are registered yet. `deck_names_for_format` returns nothing for the
  cube, so it is not offered in the web client.
- Drafting is deferred. The engine has no draft, and the plan is to reach a
  playable pool first and play fixed lists from it.
- Every pool identity is cataloged. Cards from previously unmodeled debut sets
  have printed-set modules, and unsupported rules remain metadata-only.

## Format profile

Forty-card minimum, one copy of each card, twenty life, seven-card opening
hand, contemporary mana rules, and no ban or restricted list -- a card is
either in the pool or it is not. `CubeFormatDefinition::cards` carries the
list, while set windows and banned/restricted policy live only on
`SetFormatDefinition`.

## Coverage

The fixed pool, card declarations, derived implementation status, and inline
`// Audit:` entries are the authoritative coverage inputs. This roadmap does
not repeat their derived totals or maintain another card-by-card partition:
doing so would make otherwise independent card implementations edit the same
bookkeeping and would turn parallel work into merge conflicts.

Run `make catalog-report` for current aggregate coverage, including the
declarative, custom, partial, metadata-only, and uncataloged Vintage Cube
totals. Run `make catalog-report CATALOG_REPORT_ARGS=--verbose` to include the
individual card names in each status.

Being cataloged is not the same as being audited against the rest of the cube.
A card authored for another format may meet cards here it has never been played
beside, so interaction coverage still belongs in focused gameplay tests rather
than in this inventory.
