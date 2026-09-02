# Design doctrine

Penta is growing toward a full Magic engine incrementally: one general rule,
one supported format, and sometimes one card at a time. A change should be
correct for the behavior it claims without being required to solve every
neighboring rule first.

## Incremental development

The preferred implementation order is:

1. Reuse or extend a shared, card-agnostic primitive when the behavior has a
   clear general shape.
2. Compose unusual behavior in the card's declarative ability clauses while
   shared timing, targeting, and stack rules remain in force.
3. If the shared model cannot yet express the complete behavior, keep the
   whole card unsupported until a reusable primitive exists.
4. Shared primitives may land in useful increments, but a card becomes
   executable only when its complete printed behavior is declarative.

Do not add a card-specific resolver or direct card-identity branch to make a
card executable.

This order is a preference ladder, not a purity gate. Prefer an elegant shared
abstraction to a hack, a contained hack to a diffuse mess, and a working honest
increment to waiting indefinitely for perfect architecture. Every rung has the
same floor: accurate advertised behavior, honest implementation coverage, a
bounded blast radius, and tests.

Repetition is evidence for extracting a shared primitive; one difficult card
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
