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
set code, collector number, and card name. For a double-faced card, list both
faces in front-to-back order, such as `// ISD 51 — Delver of Secrets //
Insectile Aberration`; the `CardRecord` itself remains named for the front
face. Ordinarily the header immediately starts the declaration block. Keep
every helper constant, static, or function
used by only that card inside the block, after the header and before the
`CardRecord`, so the declaration and the vocabulary it composes remain readable
together. A complete definition that still uses custom execution puts
`// Audit: custom — Needs ...` immediately below the header, naming the work
required to migrate it to declarative execution; card-local helpers follow the
audit. An incomplete identity uses `blocked`, `partial`, or `metadata-only` as
applicable. Blocked header-and-audit pairs stand alone at the identity's
collector position. Reprints do not repeat the audit. Keep every identity
header in natural collector order. The header identifies the canonical
printing in that module's set even when presentation art intentionally comes
from another printing.

Helpers used by more than one card are set-level vocabulary rather than part of
one declaration block. Keep them at the top of the set module before the first
identity header, or promote generally reusable behavior to `card::abilities` as
appropriate. Do not leave card-local helpers in the shared preamble or between
other cards' blocks.

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
outside these conventions.

Mirror every nonempty `ADDITIONAL_PRINTINGS` entry in the collector-ordered
upper portion of the set file. Use `// M14 1 — Ajani, Caller of the Pride
(reprint)` for the default printing and `// SET NUMBER — Name (alternate
printing)` for another art or variant. These standalone comments make the set
inventory readable from top to bottom; the source-organization test checks
that a file which uses them mirrors its additional-printing registry exactly.

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

The custom audit is a discoverable migration plan, not a second execution
switch. Catalog tests derive custom status from every part and modal clause,
require a matching canonical audit, and reject stale custom audits after the
last custom execution path is removed. The audit explanation should identify
the declarative primitive or concrete migration still needed rather than merely
repeat that the card has custom code.

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

## Adding a set or fixed card pool

Treat registration and catalog inventory as one change, even when card rules
will be implemented later:

1. Establish the complete legal set list from the format's authority and put
   that list in its `SetFormatDefinition`. For a cube, preserve the exact fixed
   list in its `CubeFormatDefinition` instead of inferring legality from sets.
2. Query the local Scryfall index for every paper printing in each newly legal
   set. Work in natural collector-number order and keep alternate-art collector
   numbers distinct. For a cube card, resolve its earliest paper printing and
   freeze that exact UUID as its identity anchor.
3. Leave an existing `CardRecord` unchanged. If the identity is already
   declared elsewhere, add a `PrintingRecord::reprint` (and `alternate` records
   for further variants) plus the corresponding ordered upper comments.
4. If an identity has only a standalone blocked audit row, either replace that
   row with the reprint comment and printing record, or turn it into a
   `metadata-only` audit followed by a `CardRecord::new` whose rules are
   `CardRules::unsupported()`. Preserve a useful existing capability-gap
   explanation. When there was no row, add the same metadata-only declaration
   with `Card rules have not been implemented.` as its honest initial audit.
5. Put a new identity in its debut or otherwise representative printed-set
   module and anchor it to the exact earliest paper printing. Add a `CardSet`,
   set module, registry entry, source-code mapping, and catalog JSON code when
   no modeled set can truthfully own the declaration. Append-only catalog
   growth does not require a protocol-version bump.
6. Make `CARDS` exactly mirror declaration order and
   `ADDITIONAL_PRINTINGS` mirror the ordered reprint comments. Run the focused
   source-organization and format-coverage tests, then `make catalog-report`.
   Every set format should account for its whole legal identity corpus, and a
   fixed cube should have no uncataloged names.

The [development guide](development.md) maps repository paths to validation
workflows. Current format and card coverage is described in
[formats and scope](formats.md).
