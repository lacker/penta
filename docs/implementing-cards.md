# Implementing cards

This guide describes where card behavior belongs in the current engine. The
[design doctrine](design-doctrine.md) explains why implementation boundaries
are preferences rather than purity requirements. The source-layout rules in
this guide are invariants.

## Definition boundary

Each built-in canonical card is declared once in the `CARDS` registry of its
representative set module. Prefer the card's first paper printing in an
English-language set when one exists; use its first paper printing in any
language only when no English printing exists. In the local Scryfall index,
this means selecting the earliest nondigital `printings` row with `lang = en`
when possible rather than letting an earlier non-English-only product own the
definition. Its `CardRecord` keeps its identity and primary `CardRules`
together: name, cost, types, creature stats, and ordered ability clauses can
all be understood at the declaration.

Within a printed set module, keep declarations and the `CARDS` registry in
natural collector-number order, with `CARDS` exactly mirroring declaration
order. Compare numeric portions numerically (`8`, `8a`, `8b`, `16`), not
lexicographically. Introduce each declaration with an identifying comment in
the form `// LEA 230 — Ankh of Mishra`, using the canonical printing's uppercase
set code, collector number, and card name. For a double-faced card, list both
faces in front-to-back order, such as `// ISD 51 — Delver of Secrets //
Insectile Aberration`. A modeled double-faced `CardRecord` uses the same
combined `front // back` name while the catalog retains the front name as a
deck-list lookup alias. Define both modeled faces together with
`CardRecord::new_dfc` for a transforming card or `CardRecord::new_mdfc` for a
modal double-faced card; pass their named face rules directly to those
constructors so they can derive the parts, topology, and play options.
The header immediately starts the declaration block. Inline every card-local
cost, target, effect, ability, predicate, query, value, and collection directly
in the `CardRecord` and its ordered `CardRules` clauses. A named card-local
component is allowed only when the definition references it more than once or
when it is genuinely recursive or self-referential. Shortening or visually
decomposing a declaration is not a reason to extract a component. A shared
power/toughness value qualifies because both characteristics reference it;
power/toughness values are not otherwise a special exception. Keep every
allowed extracted component after the header and before the `CardRecord`,
adjacent to the clause it supports and in printed-clause order.

An incomplete identity uses `blocked`, `partial`, or `metadata-only` as
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
exact preferred-printing Scryfall UUID through `PrintingAnchor::scryfall`: the
first English-language paper printing when possible, otherwise the first paper
printing in any language. The build derives a stable 52-bit ID and rejects
collisions across the whole corpus. Do not recompute the preferred printing
later: the committed anchor is the identity.
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
its costs, targets, effect, and coverage. Displayed rules text and
aggregate Complete, Partial, or MetadataOnly status derive from those clauses
rather than from parallel card-level assertions.

Reuse constructors from `card::abilities` and declarative rules primitives
where they fit. Keep rules text, implementation coverage, and execution tied to
the same clause. Card-specific execution is not an extension boundary.

Card declarations are oblivious to the
[prepared engine](prepared-engine.md). Do not add preparation flags, prepared
executors, optimization hints, or special constructors to a declaration or the
card schema. Declare the semantic operation in the ordinary model. The
prepared compiler may recognize that structure and lower it independently;
unsupported structures continue through the reference implementation.

## Extension boundaries

Use the smallest boundary that truthfully implements the behavior:

- A recurring mechanic or general Magic rules concept belongs in a reusable,
  card-agnostic primitive that ability definitions can invoke.
- Genuinely card-specific composition belongs directly in the relevant card's
  declarative ability clause. When the required semantic shape is reusable,
  add a shared primitive rather than a card-scoped resolver.
- If neither the definition nor a reasonably scoped shared primitive can
  express the complete card, leave the unsupported portion honestly partial or
  metadata-only. Do not add a direct card-identity branch in generic `Game` or
  state-machine flow.

Resolution must not silently change an explicit ability category or let a
supported activated or triggered non-mana ability bypass the shared stack.

## Coverage and partial support

Executable clauses use declarative effects and carry no separate behavior
identity. Unsupported cards may exist in catalogs and hidden zones, but the engine does not offer play options that
would resolve as silent no-ops.

When complete fidelity is too large for the current increment, implement the
working portion, mark the remainder accurately, and state the follow-up. A
contained special case with focused tests is preferable to scattering partial
logic through unrelated engine paths.

## Implementation workflow

1. Confirm the printed clauses and the format or card interaction being added.
2. Represent the clauses, categories, costs, targets, effects, and coverage in
   the card definition.
3. Reuse an existing primitive or add a shared primitive according to the
   preference ladder above; otherwise retain honest incomplete coverage.
4. Test new shared rule behavior once at the narrowest useful boundary. Add a
   card-level test only for text-sensitive composition, a legality boundary,
   or an interaction that could fail while the shared primitive still passes.
5. Run final focused native checks for the changed behavior before handoff.
   Add browser validation only when the card work also changes a browser-facing
   contract or web code; leave aggregate integration coverage to PR CI.

Before adding a card test, find the closest existing mechanic test. A second
card using the same primitive does not need another happy-path dispatch test;
extend the mechanic test only when the new card contributes a distinct case.
Do not test a named card merely to assert derived implementation status or audit
classification. Source audits and generated catalog reports own coverage facts.

## Adding a set or fixed card pool

Treat registration and catalog inventory as one change, even when card rules
will be implemented later:

1. Establish the complete legal set list from the format's authority and put
   that list in its `SetFormatDefinition`. For a cube, preserve the exact fixed
   list in its `CubeFormatDefinition` instead of inferring legality from sets.
2. Query the local Scryfall index for every paper printing in each newly legal
   set. Work in natural collector-number order and keep alternate-art collector
   numbers distinct. For a cube card, resolve its earliest English-language
   paper printing when possible, otherwise its earliest paper printing, and
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
5. Put a new identity in the module for its first English-language paper set
   when possible, otherwise its earliest paper set, and anchor it to that exact
   preferred printing. Add a `CardSet`, set module, registry entry, source-code
   mapping, and catalog JSON code when no modeled set can truthfully own the
   declaration. Append-only catalog growth does not require a protocol-version
   bump.
6. Make `CARDS` exactly mirror declaration order and
   `ADDITIONAL_PRINTINGS` mirror the ordered reprint comments. Run the focused
   source-organization and format-coverage tests, then `make catalog-report`.
   Every set format should account for its whole legal identity corpus, and a
   fixed cube should have no uncataloged names.

The [development guide](development.md) maps repository paths to validation
workflows. Current format and card coverage is described in
[formats and scope](formats.md).
