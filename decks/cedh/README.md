# Nacional de cEDH 100K corpus

This bounded corpus records all 123 EDHTop16 standings entries for Nacional de cEDH 100K @ WolfCon 2026, held on 2026-08-29. The reproducible importer is `tools/import_cedh_tournament.py`; it reads the event payload and each linked TopDeck `decklistText` payload.

There are 119 imported YAML decklists. The remaining 4 entries have no readable TopDeck `decklistText`, so they deliberately have no invented list; their event IDs, source URLs, and precise import reason are in `provenance.json`.

`provenance.json` preserves every source decklist verbatim and its parsed source sections. Auxiliary non-gameplay sections contain 20 cards and remain there rather than being moved into a playable Commander zone.

To refresh the same bounded event and browser selector:

```sh
python3 tools/import_cedh_tournament.py --output decks/cedh --web-registry web/app/cedh-decks.json
```

Review changed source lists and card support after importing. The importer does
not fetch or modify the shared Magic reference cache, implement cards, or
validate format legality. See [Commander foundation](../../docs/commander.md).
