# Design doctrine

Penta is growing toward a full Magic engine incrementally: one general rule,
one supported format, and sometimes one card at a time. A change should be
correct for the behavior it claims without being required to solve every
neighboring rule first.

## Incremental development

Start by composing existing operations. Add a core primitive when a genuine
fundamental capability is missing, not merely because a card's composition is
awkward to write. A primitive may have only one or two consumers; frequency
alone neither qualifies nor disqualifies it.

Magic is a game of exceptions. A complete, contained card-local implementation
is a legitimate destination, even if it will never be generalized. Ordinary
Rust syntax in a set file is acceptable when it makes local behavior clearer.
Keep the card understandable in one place; this is not permission to fragment
each declaration into tiny helpers or scatter card-identity checks through the
engine. Local exceptions must participate in the engine's execution contracts.
If the available integration boundary cannot do that faithfully, keep the
whole card unsupported and identify the missing capability.

## Ownership of behavior

Place behavior at the narrowest useful ownership level:

1. **Core primitives and structures:** fundamental rules operations, data,
   combination, binding, and control flow.
2. **Broadly shared vocabulary:** recurring compositions bordering on
   primitives, such as enters/dies triggers and scry.
3. **Mechanics shared across many sets:** reusable mechanic programs with a
   clearly named common home.
4. **Mechanics shared across a few sets:** owned by their originating set and
   imported by later sets, as Metalcraft is.
5. **Several cards in one set:** a coherent helper in that set's preamble.
6. **One card:** its ordered clauses and, when justified, an adjacent local
   procedure.

These are ownership levels, not six mandatory module directories. Reuse can
justify moving a composition outward without turning it into a core primitive.
The [effect-program guide](effect-programs.md) records the execution contracts
and the incremental migration plan.

## Scope and correctness

These boundaries guide incremental work rather than imposing a purity gate.
Prefer an elegant shared abstraction to a hack, a contained hack to a diffuse
mess, and a working honest increment to waiting indefinitely for perfect
architecture. Every rung has the
same floor: accurate advertised behavior, honest implementation coverage, a
bounded blast radius, and tests.

Repetition is evidence for sharing a composition; one difficult card
is not automatically evidence for a framework. A working exception does not
need immediate migration merely for architectural purity. Refactor when a
stable semantic boundary emerges, or when the relevant code is already being
changed and the cleanup is reasonably in scope.

Card implementations should remain independently mergeable unless
they genuinely share a rules primitive or compatibility boundary. Do not make
each card change update checked-in global coverage counts, generated lists, or
tests that merely restate those derived artifacts. Keep declarations and inline
`// Audit:` entries authoritative, and generate
aggregate or card-by-card coverage reports from them on demand.

## Engine principles

- Identifiers are scoped implementation details, derived during compilation or
  runtime construction. Do not hand-maintain identifier or binding-number
  registries. Generated symbol tables and process-local interning are derived
  implementation details. Persist natural keys and resolve local IDs on load.
  References to game-created objects are scoped to their owning game/checkpoint;
  they are not durable identities across games.
- Game state changes only through explicit actions.
- All randomness comes from a recorded seed and a versioned PRNG.
- Runtime rules objects use zone-scoped identities while private physical-card
  lineage follows the underlying cards.
- Player observations do not expose an opponent's hidden information.
- Legal actions are enumerated and checked by the engine.
- The core engine has no UI, network, async-runtime, or training dependencies.
- Unsupported cards remain visible as whole-card `Unsupported` coverage and
  expose no executable subset or silent no-op.
- Optimization is derived from the semantic model rather than requested by
  card declarations; the reference implementation remains available wherever
  an optimized lowering cannot prove support.

These are durable constraints, not a demand that every implementation be
maximally general. The [card implementation guide](implementing-cards.md)
applies this doctrine to concrete card work. The [engine architecture](engine.md)
describes the abstractions that exist today, and the
[prepared-engine guide](prepared-engine.md) defines the optimization boundary.
