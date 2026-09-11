# Built-in decks

Every `.yaml` or `.yml` file in `decks/<format>/` is compiled into the engine.
Adding, editing, or removing a file updates the registry on the next build;
no Rust registration or runtime filesystem access is needed. Native callers,
bindings, and the browser use the same registry.

The directory names match the format slugs with underscores, for example
`old_school_93_94`, `premodern`, and `isd_m14_standard`.

```yaml
name: Example Deck
order: 10
aliases: [Example]
main:
  Mountain: 60
sideboard: {}
```

- `name` is the required display name and case-insensitive lookup name.
- `main` and `sideboard` are required card-name mappings with positive integer
  counts. Use `{}` for an empty section. Quote names containing a colon,
  such as `"Circle of Protection: Red"`.
- `aliases` optionally adds case-insensitive lookup names. Names and aliases
  must be unique within their format, including across decks.
- `order` optionally sets menu position, lowest first. Decks without it sort
  last; equal positions sort by file path for deterministic ordering.
- `id` optionally overrides the filename stem used for the generated Rust
  constructor, `decks::<format>::<id>()`. Use lowercase letters, digits, and
  underscores, with no leading digit.
- `rust_aliases` optionally preserves additional Rust constructor names.
- `description` optionally supplies the constructor's documentation.

Old School constructors also remain available directly under `decks::` for
compatibility. Preserve source citations and transcription notes as YAML
comments. Card entries expand in authored order, so avoid reordering them
unless you intend to change the deck order used by seeded games.

Run `make test-engine-unit FILTER=decks` to check YAML validation, registry
lookup, card resolution, and deck legality.
