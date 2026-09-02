# Prepared engine

The prepared engine is an optional optimization layer over Penta's declarative
card model. It recognizes supported semantic shapes, derives compact
process-local programs, and executes those programs through narrow runtime
interfaces. It is enabled by default, but the ordinary declarative engine
remains the reference implementation and the authority for game semantics.

Preparation is not a second card-definition language. A card author declares
only rules concepts such as `EffectDef`, `AppliedEffectDef`, predicates, and
values. The prepared compiler decides whether a complete declaration subtree
has a safe lowering. Unsupported shapes retain the reference path without
requiring a declaration change.

## Boundary and dependency direction

The dependency direction is deliberately one way:

```text
card declarations -> declarative card model -> prepared compiler -> prepared program
                                                |                    |
                                                +-> no lowering -----+-> reference path
```

The following are invariants:

- Card declarations do not mention preparation, intrinsics, executors, or
  prepared program types.
- The card schema does not carry optimization hints or prepared payloads.
- `src/card` does not depend on `src/prepared_engine`.
- The declarative model remains available for validation, traversal,
  presentation, checkpointing, and fallback.
- Prepared artifacts are derived process-local state and are never serialized
  into a checkpoint or exposed as part of a compatibility contract.
- A prepared path is selected only before that path mutates game state. If the
  compiler cannot prove that the supported root and its required inputs can be
  handled, the runtime uses the reference implementation for that root.

The immutable `CardCatalog` exposes an opaque process-local cache identity.
That is generic cache infrastructure rather than preparation metadata: the
prepared engine owns the cache, its keys, and every compiled value.

## Compilation and runtime flow

Catalog-derived preparation happens when a `Game` is constructed or restored
from a checkpoint. `PreparedEngine::compile` looks up a compiled artifact for
the immutable catalog identity and compiles it on a miss. Games backed by the
same catalog share the resulting `Arc<PreparedCatalog>`. Entries whose source
catalog has gone away are removed when another catalog is inserted.

There are currently two execution shapes.

### Resolving effects

When an ability resolver is frozen, `compile_effect` inspects its ordinary
`EffectDef`. A successful lowering is stored in the stack payload alongside
the authoritative `ScopedEffect`. Resolution uses the prepared effect only
when preparation is enabled and no modal or spliced effects have been added.
Otherwise it resolves the retained reference effect.

The initial intrinsic recognizes exactly this semantic shape:

```text
DrawCards(recipient = Controller, amount = Constant(n))
    -> PreparedEffect::DrawCards { count: n }
```

A dynamic amount, another recipient, an out-of-range count, or any other
effect returns no lowering. Card declarations use the ordinary `draw_cards`
constructor in both cases; they neither request nor observe the intrinsic.

### Static programs and summaries

Catalog compilation visits each card definition and part. It derives:

- a compact program for executable printed static abilities;
- lane summaries for rules, card types, colors, abilities, subtypes, and
  power/toughness;
- a summary of whether a definition can supply a land-type effect; and
- a summary of whether a definition supplies a static ability from the
  graveyard.

Static programs flatten composite applied effects, preserve source-zone and
conditional checks, and preassign component order and ability-grant identity.
Callers can reject definitions or applications that do not supply the lane
being evaluated before walking their components.

An `IfCondition` or `IfElseCondition` whose branches have a stable structure
can be represented as runtime trigger-condition checks. A live
`ConditionalStatic` can change whether a structural subtree exists, which can
shift all later component and grant identifiers. Its entire static ability
therefore retains the reference walker. The prepared catalog still retains
the reference effect for that fallback.

Catalog programs apply only when the runtime has the matching catalog-backed
card characteristics. Tokens, emblems, face-down characteristics, copy-added
abilities, and other runtime-derived structures continue through the general
path wherever a catalog program cannot describe them.

## Fallback rules

Fallback is part of the design, not an error case.

- A resolving effect is prepared only when the complete supported root and
  every input needed by its executor are representable.
- A modal or spliced stack payload resolves through the reference engine as a
  unit even if its primary effect has a prepared form.
- A static ability with structurally dynamic control flow falls back at the
  ability root so component ordering and grant identity cannot diverge.
- Runtime objects outside the proof represented by a catalog program use the
  existing general traversal.
- Disabling preparation causes retained reference effects and walkers to run;
  it does not change the card model or checkpoint format.

Do not partially execute a prepared root and then fall back. The selection
must happen before mutation unless an executor has an explicit transactional
design that preserves the same guarantee.

## Adding a prepared lowering

Start from a semantic operation that already belongs in the declarative model.
If a new rules concept is required, add it for semantic clarity and implement
the reference behavior first. Do not add a card constructor, field, variant,
or marker whose purpose is to opt into optimization.

Then:

1. Identify a measured repeated cost or a common semantic root whose lowering
   can remove meaningful interpretation overhead.
2. Add a private prepared representation under `src/prepared_engine`.
3. Extend the compiler to recognize the ordinary declarative shape. Reject the
   whole relevant root when an input or structural invariant is unsupported.
4. Execute through a narrow host operation that preserves the reference
   engine's events, decisions, ordering, identity, and state transitions.
5. Retain the reference representation at every runtime fallback boundary.
6. Add compiler tests for both the accepted shape and nearby rejected shapes.
7. Add differential tests that run prepared and reference execution from the
   same state and compare all affected state, events, pending work, and result.
8. Benchmark identical deterministic workloads with the same binary, seed,
   decks, game count, build profile, and machine.

Repeated declaration helpers such as `card::abilities` constructors are useful
semantic recognition targets, but they remain ordinary model constructors.
The compiler recognizes the structure they produce rather than the name of the
helper that produced it. This lets hand-authored equivalent declarations gain
the same optimization automatically.

For static programs, differential coverage must compare lane output as well as
the final visible characteristic. Component order, timestamps, ability origins,
and grant identifiers are observable inputs to later continuous-effect work
and must remain identical.

## Disabling and differential diagnosis

Preparation is enabled for new games by default. Native callers can toggle all
prepared paths for one game:

```rust
game.set_prepared_engine_enabled(false);
```

The `penta-match` diagnostic runner accepts the hidden
`--reference-engine` flag. It is intended for differential tests, profiling,
and benchmark comparisons, not as a separate supported rules mode.

A useful correctness check runs the same deterministic seed in both modes and
compares the resulting state or outcome. Matching outcomes alone are weaker
than focused differential tests: two executions can reach the same winner
while differing in intermediate events, choices, or identities.

## Checkpoints and compatibility

Checkpoints serialize the declarative game state and reference resolver data,
not `PreparedCatalog`, `PreparedStaticProgram`, or another prepared artifact.
Restore recompiles or reuses the process-local catalog cache. Consequently,
prepared layouts can change without a checkpoint or protocol migration as long
as reference semantics and serialized structures are unchanged.

If a proposed optimization would require prepared data on the wire, in a card
declaration, or in serialized state, it has crossed the intended boundary and
needs an explicit architecture and compatibility review.

## Source map

- [`src/prepared_engine/mod.rs`](../src/prepared_engine/mod.rs) owns private IR,
  catalog caching, enablement, and host boundaries.
- [`src/prepared_engine/compiler.rs`](../src/prepared_engine/compiler.rs)
  recognizes declarative shapes and builds prepared programs and summaries.
- [`src/prepared_engine/executor.rs`](../src/prepared_engine/executor.rs)
  dispatches prepared resolving effects through the host interface.
- [`src/game/prepared_host.rs`](../src/game/prepared_host.rs) integrates the
  prepared engine with `Game` and exposes the diagnostic toggle.
- [`src/game/tests/prepared_engine.rs`](../src/game/tests/prepared_engine.rs)
  contains the primary prepared/reference differential tests.

Use the [performance guide](performance.md) for reproducible timing and
profiling. Use the [card implementation guide](implementing-cards.md) for
declaration and extension-boundary rules.
