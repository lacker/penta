# Built-in decks

Every `.yaml` or `.yml` file in `decks/<format>/` is compiled into the engine.
Adding, editing, or removing a file updates the registry on the next build;
no Rust registration or runtime filesystem access is needed. Native callers,
bindings, and the browser use the same registry.

For playable decks, directory names match the format slugs with underscores, for example
`old_school_93_94`, `premodern`, and `isd_m14_standard`.

```yaml
name: Example Deck
id: example_deck
aliases: [Example]
main:
  Mountain: 60
sideboard: {}
```

- `name` is the required display name and case-insensitive lookup name.
- `main` and `sideboard` are required card-name mappings with positive integer
  counts. Use `{}` for an empty section. Quote names containing a colon,
  such as `"Circle of Protection: Red"`.
- `id` defaults to the filename stem. If supplied, it must match that stem
  exactly (for example, `example_deck.yaml` has ID `example_deck`). IDs are
  accepted as lookup names and name the generated Rust constructor,
  `decks::<format>::<id>()`. Use lowercase letters, digits, and underscores,
  with no leading digit.
- `aliases` optionally adds lookup names that differ from the ID and display
  name. Lookup ignores case. Names, IDs, and aliases must not collide with
  another deck in the same format.
- `rust_aliases` optionally preserves additional Rust constructor names.
- `staged: true` retains an implementation inventory without registering it for
  gameplay. Staged files use the same schema, name checks, and card resolution,
  and may belong to a pool directory without a supported format, such as
  `woe_hob_standard`. Remove this flag once the format and deck are ready.
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
