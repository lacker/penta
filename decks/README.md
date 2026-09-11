# Built-in decks

Every `.yaml` or `.yml` file in `decks/<format>/` is compiled into the engine.
Adding, editing, or removing a file updates the registry on the next build;
no Rust registration or runtime filesystem access is needed. Native callers,
bindings, and the browser use the same registry.

Directories matching registered format slugs, such as `old_school_93_94`,
`premodern`, and `isd_m14_standard`, supply that format's deck collection.
Other directories, such as `woe_hob_standard`, retain inventories before their
format profile is registered; `make deck-report` reports that separately.

```yaml
name: Example Deck
id: example_deck
aliases: [Example]
commanders: {}
main:
  Mountain: 60
sideboard: {}
```

- `name` is the required display name and case-insensitive lookup name.
- `main` and `sideboard` are required card-name mappings with positive integer
  counts. Use `{}` for an empty section. Quote names containing a colon,
  such as `"Circle of Protection: Red"`.
- `commanders` optionally names the physical card or cards designated for the
  command zone. Like the other sections, it is a card-name mapping with
  positive counts; use it only for Commander-family formats. A commander is
  not also listed in `main` or `sideboard`.
- `id` defaults to the filename stem. If supplied, it must match that stem
  exactly (for example, `example_deck.yaml` has ID `example_deck`). IDs are
  accepted as lookup names and name the generated Rust constructor,
  `decks::<format>::<id>()`. Use lowercase letters, digits, and underscores,
  with no leading digit.
- `aliases` optionally adds lookup names that differ from the ID and display
  name. Lookup ignores case. Names, IDs, and aliases must not collide with
  another deck in the same format.
- `rust_aliases` optionally preserves additional Rust constructor names.
- `description` describes the deck in ordinary prose. It also appears in the
  generated constructor's documentation.

Menus sort alphanumerically by display name, ignoring case and comparing digit
runs numerically (`Deck 2` before `Deck 10`). Equivalent names sort by file path
for deterministic ordering. There is no authored `order` field.

Old School constructors also remain available directly under `decks::` for
compatibility. Preserve source citations and transcription notes as YAML
comments. Card entries expand in authored order, so avoid reordering them
unless you intend to change the deck order used by seeded games.

Run `make test-engine-unit FILTER=decks` to check YAML validation, registry
lookup, card resolution, and deck legality.

Implementation coverage is derived from the catalog. Run `make deck-report` to
list every unsupported card in each deck, including its sideboard. There is no
manual readiness flag: implementing a card changes the next validation result.
Unknown cards are errors; cataloged unsupported cards are named diagnostics.

Native callers can use `Deck::validate_supported_cards(&catalog)`. It returns
`DeckError::UnsupportedCards` with sorted, distinct names, or `UnknownCard` for
a missing definition. This check is independent of `validate_for_format`, which
checks construction and legality. A legal deck can still contain unsupported
cards, and an inventory can be checked before its format profile exists.

`cedh` is a two-player Commander gameplay baseline, not a claim of complete
Commander deck validation. Its list files preserve designated commanders and
the engine checks that every listed identity exists in the catalog; colour
identity, singleton/count rules, and the Commander ban policy remain deferred
until the shared Commander validator covers them. See Wizards of the Coast's
[Commander format page](https://magic.wizards.com/en/formats/commander) for the
current format description. The ban-list metadata follows
Wizards of the Coast's [Banned and Restricted List](https://magic.wizards.com/en/banned-restricted-list), retrieved 2026-09-11. In that source, Lutri, the Spellchaser is banned only as a companion, not as a card in a Commander deck.
