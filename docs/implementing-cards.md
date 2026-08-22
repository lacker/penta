# Implementing cards

This guide describes where card behavior belongs in the current engine. The
[design doctrine](design-doctrine.md) explains why these boundaries are
preferences rather than purity requirements.

## Definition boundary

Each built-in canonical card is declared once in the `CARDS` registry of its
representative or debut set module. Its `CardRecord` keeps its identity and
primary `CardRules` together: name, cost, types, creature stats, and ordered
ability clauses can all be understood at the declaration.

Within a printed set module, keep declarations and the `CARDS` registry in
natural collector-number order, with `CARDS` exactly mirroring declaration
order. Compare numeric portions numerically (`8`, `8a`, `8b`, `16`), not
lexicographically. Introduce each declaration with an identifying comment in
the form `// LEA 230 — Ankh of Mishra`, using the canonical printing's uppercase
set code, collector number, and card name. Ordinarily the header immediately
precedes the declaration. In the inline Old School audit, an incomplete
identity puts `// Audit: blocked — Needs ...` on the next line, using `partial`
or `metadata-only` when applicable. Partial and metadata-only audit lines
immediately precede their declarations; blocked header-and-audit pairs stand
alone at the identity's collector position. Keep every identity header in
natural collector order. The header identifies the canonical printing in that
module's set even when presentation art intentionally comes from another
printing.

Existing definitions use `CardRecord::new_with_legacy_id`, with their historic
numeric value written beside the record. Never allocate another sequential
legacy value. A new definition uses `CardRecord::new` with an explicitly frozen
exact first-printing Scryfall UUID through `PrintingAnchor::scryfall`; the build
derives a stable 52-bit ID and rejects collisions across the whole corpus. Do
not recompute the earliest printing later: the committed anchor is the identity.
If the
vanishingly unlikely collision occurs, commit
`PrintingAnchor::scryfall_with_nonce` for the newcomer rather than changing any
existing record. The ordinary `card::cards::*` constants are compatibility
output generated from these declarations, not an independently authored ID
registry. A compact build fingerprint prevents any migrated legacy assignment
from moving. Presentation art may change independently; use
`with_identity_anchor` when the anchor printing differs from the selected art.

Keep `ADDITIONAL_PRINTINGS` in natural order by the collector number in that
module's set, including for reprint-only modules with an empty `CARDS`
registry. Put each nonempty entry on its own line with a trailing identity such
as `// LEB 233`: uppercase target set code and collector number, without the
card name. Empty registries need no comments. Creator-owned token and emblem
characteristics and rules-owned face-down characteristics are built by the
effect or mechanism that creates them, are not card definitions, and remain
outside these conventions. Move card-local helpers with the definition they
support.

Start new and migrated work with the card's ordered `AbilityDef` clauses. Each
printed clause should carry its explicit timing category and, where applicable,
its costs, targets, effect, execution, and coverage. Displayed rules text and
aggregate Complete, Partial, or MetadataOnly status derive from those clauses
rather than from parallel card-level assertions.

Reuse constructors from `card::abilities` and declarative rules primitives
where they fit. Keep rules text, implementation coverage, and execution tied to
the same clause even when the final effect requires custom code.

## Extension boundaries

Use the smallest boundary that truthfully implements the behavior:

- A recurring mechanic or general Magic rules concept belongs in a reusable,
  card-agnostic primitive that ability definitions can invoke.
- Genuinely card-specific behavior, or behavior whose reusable shape is not yet
  clear, belongs in a card-scoped implementation reached from the relevant
  ability clause. Keep timing, costs, targets, and stack behavior declarative
  around the custom portion whenever possible.
- A direct card-identity branch in generic `Game` or state-machine flow is an
  escape valve when the definition and card-scoped paths cannot reasonably
  express the card. Keep it narrow, searchable, documented, and tested.

Custom resolution must not silently change an explicit ability category or let
a supported activated or triggered non-mana ability bypass the shared stack.
Existing engine-level special cases are migration inventory rather than
templates, but they need not be migrated until repetition or nearby work makes
the better boundary clear and reasonably scoped.

## Coverage and partial support

Declarative effects need no custom behavior identity. A custom clause keeps its
closed effect selector, independent coverage, and explanation beside the clause
even when its handler remains centralized. Unsupported cards may exist in
catalogs and hidden zones, but the engine does not offer play options that
would resolve as silent no-ops.

When complete fidelity is too large for the current increment, implement the
working portion, mark the remainder accurately, and state the follow-up. A
contained special case with focused tests is preferable to scattering partial
logic through unrelated engine paths.

## Implementation workflow

1. Confirm the printed clauses and the format or card interaction being added.
2. Represent the clauses, categories, costs, targets, effects, and coverage in
   the card definition.
3. Reuse an existing primitive, add a shared primitive, attach card-local
   execution, or introduce a narrow special case according to the preference
   ladder above.
4. Test the shared rule behavior and the card's exceptional result at the
   narrowest useful level.
5. Run final focused native checks for the changed behavior before handoff.
   Add browser validation only when the card work also changes a browser-facing
   contract or web code; leave aggregate integration coverage to PR CI.

The [development guide](development.md) maps repository paths to validation
workflows. Current format and card coverage is described in
[formats and scope](formats.md).
