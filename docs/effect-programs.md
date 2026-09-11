# Effect programs and ownership

Keep behavior at the narrowest useful ownership level: genuine core operations;
broadly shared compositions; mechanics shared across many sets; mechanics owned
by an originating set and imported elsewhere; set-local helpers; and card-local
programs. These are ownership decisions, not six mandatory directories.

The core must supply real rules operations, selections, bindings, outcomes, and
resumable control flow. A primitive can have one consumer. Conversely, recurring
syntax alone does not make a whole mechanic a primitive. Magic's exceptions
need contained implementation boundaries, not card-identity dispatch in Game.

## Current examples

- Upstream's [game action programs](game-actions.md) share discard, sacrifice,
  and control changes between ordinary effects and resolving costs.
- Cumulative upkeep remains its ordinary counter/payment/consequence program;
  Herald of Leshrac supplies its local gain-control cost.
- Battle cry is owned by Mirrodin Besieged and imported by Modern Horizons.
  Battalion belongs to Gatecrash; mobilize belongs to Tarkir: Dragonstorm.
- Endurance constructs a queried collection, random ordering, and movement in
  its own clause. There is no Endurance-shaped graveyard operation in the core.
- Bloomburrow owns forage's identity and choice of ordinary exile/sacrifice
  game actions. Corpseberry Cultivator and Feed the Cycle share that composition.

## Identities and named actions

`MechanicId::from_name("mtg:forage")` computes a stable u64 during constant
evaluation. Define the constant beside its owner and import it; source tests
reject duplicate or colliding declarations. Moving a mechanic must not rename
its identity. Runtime abilities, payments, and events contain the number, not
the spelling. Checkpoint boundaries encode it as fixed-width hexadecimal text
to avoid JSON numeric precision loss.

`AbilityLabel` is a role-naming alias for `MechanicId`. Ability classification,
payment purpose, and a completed action remain separate uses of that vocabulary.
A label does not dispatch a hidden implementation or promise a prepared lowering.

`GameActionDef::Named { mechanic, action }` wraps the action program, not its
cost or effect wrapper. Bloomburrow labels its whole exile-or-sacrifice program
with `ACTION.named(FORAGE)`; Corpseberry uses `as_effect()` beneath `May`, and
Feed the Cycle uses `as_cost()`. Both enter the same selection planner and
semantic executor. No named-cost grammar or separate commit implementation exists.

The action publishes `MechanicPerformed` once its selected branch and replacement
work finish. Declining or merely selecting objects does not publish an occurrence.
Sacrificing a Food still emits ordinary sacrifice events; that same sacrifice for
another purpose is not forage. Completion is not inferred from the final zone.

Completion-aware naming currently supports sacrifice and graveyard-exile actions,
their object selection, and a choice of those selections. A name on an arbitrary
sequence, nested names, or a discard replacement is not yet supported. Ordinary
unnamed discard and control programs retain upstream's support. Named selections
can participate in resolving cost bundles, repetitions, and labeled payments;
payment-purpose labels are independent of action identity.

Public action alternatives use a linear window: choose a fully executable branch,
then select its exact object group with one option per candidate. A resolving
payment may decline before commitment; an ordinary instruction uses an explicit
`May` when optional. Both decision stages reconstruct from the authored action
or PayOr and revalidate their offers. Private alternatives are not advertised
through this public window. No information-exposing rewind is introduced.

Casting reuses action candidate evaluation and semantic execution, while retaining
complete-plan enumeration. Its supported action slice is fixed positive sacrifice
or graveyard-exile selections. Alternatives must be distinguishable by zone,
because the cast wire records objects rather than action-branch IDs. An outer cost
choice involving an action program may have only one object-bearing alternative.
Activation programs and a universal editable payment UI remain follow-ups.

## Readability and local exceptions

Inline declarations by default. Extract a coherent procedure when it genuinely
makes complicated behavior easier to understand, not to name every subexpression.
Keep it adjacent to the card; set-shared helpers belong in the set preamble.
No special comment marker grants an exemption, and a comment cannot justify a
chain of one-use data helpers. Ordinary Rust may build the inspectable program.

There is not yet an unrestricted card-local runtime callback. A future bounded
interface must integrate engine-mediated actions, replacements, hidden
information, seeded randomness, typed results, resumable state, validation,
and checkpoint reconstruction. Separate hand-maintained affordability and
execution callbacks would recreate the divergence we are removing.

## Remaining work

Extend shared action/cost coverage using concrete consumers. Candidates include
joint casting/activation selection windows, compatible mana/convoke reservations,
and richer named bundles; general typed results and resumable local loops;
endure, detain, and unleash compositions; voting separate from its consequence;
and exile selection separate from casting permission. Preserve atomic operations
and simultaneous reads where the rules require them. Enum size is not a goal.

The [prepared engine](prepared-engine.md) may recognize and fuse any supported
composition independently of its owner. Reference semantics must work without
preparation. Choose a complete lowering or reference fallback before mutation;
preserve choices, events, identities, and continuation boundaries.
