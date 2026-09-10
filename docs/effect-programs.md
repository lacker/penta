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
  costs. Corpseberry Cultivator and Feed the Cycle share that composition.

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

`CostDef::Named` wraps ordinary cost grammar and publishes `MechanicPerformed`
once after the selected payment's actions and replacement work finish. Declining
or merely selecting objects does not publish an occurrence. Sacrificing a Food
while foraging still emits the ordinary sacrifice event; that same sacrifice
for another purpose is not forage. Each event uses shared trigger capture and
stack placement. Named completion is not inferred from the final zone.

The initial named-cost adapter admits fixed positive sacrifice and graveyard
exile costs, either singly or as alternatives. Resolving use requires a sole
named cost in an unlabeled PayOr; nested named costs, repetitions, activation
costs, and mixed resolving bundles are not admitted. Casting supports named
object costs inside its existing bundles/alternatives. Because cast actions
record objects rather than mechanic-branch IDs, an outer choice involving a
named cost may have only one object-bearing alternative. These are explicit
coverage boundaries, not a claim of an unrestricted program interpreter.

The resolving window first chooses a legal alternative or declines, then
selects its exact object group with one option per candidate. All selections
are validated before any action runs. Both stages reconstruct from the source's
authored PayOr and revalidate the offer. The public-zone adapter retains public
visibility. It does not introduce a universal editable payment UI or general
rewind; casting retains its existing complete-plan enumeration.

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
