# Effect programs and local behavior

The [design doctrine](design-doctrine.md#ownership-of-behavior) separates core
semantics from ownership of compositions. The [card guide](implementing-cards.md)
keeps each card readable at its declaration. Declarative syntax is useful, but
is not a requirement to force every exceptional procedure into a universal
grammar or a card-shaped engine variant.

## Current authoring and execution

Rust functions, local values, branches, and loops may construct ordinary
effect, cost, and ability structures in their owning card or set module.
Runtime code currently interprets those structures; construction-time Rust is
not a new callback or serialization protocol. Keep one-use components inline
unless a coherent local procedure materially improves readability. Comments
explain non-obvious constraints; no annotation is required to permit a procedure.

The first migration demonstrates several ownership levels:

- Common enters/dies triggers and scry remain in `card::abilities`.
- Battle cry belongs in Mirrodin Besieged and is imported by Modern Horizons.
- Battalion belongs in Gatecrash; mobilize belongs in Tarkir: Dragonstorm.
- Bloomburrow's forage helper composes a choice of ordinary exile and sacrifice
  costs. The core need not recognize a `Forage` cost variant.
- Endurance's graveyard instruction composes a target-relative collection,
  random ordering, and a move in its own declaration. The core need not know
  an Endurance-shaped `BuryGraveyard` operation.

Helper names and source locations are not runtime dispatch keys. Moving a
helper between ownership levels does not require adding a core operation.

## Named mechanics and observable actions

`MechanicId` is an opaque `u64`. Define a constant beside the mechanic with
`MechanicId::from_name("mtg:forage")`, then import that constant. The fixed
FNV-1a computation runs at compile time for constants; live costs, events, and
trigger predicates contain only the number. Catalog source validation rejects
duplicate or colliding declarations. The spelling is an identity contract:
moving a definition does not rename it. It is distinct from the provenance of
one particular ability instance.

Forage's ID and cost composition belong to Bloomburrow. Sacrifice's ID belongs
to shared vocabulary (`abilities::SACRIFICE`). Both publish the same committed
`MechanicPerformed` event and use the ordinary trigger-capture, APNAP placement,
and resolution paths. An event can retain an affected object's characteristics;
sacrifice retains that snapshot before the permanent leaves. Per-object and
"one or more" listeners retain distinct aggregation semantics. A sacrifice
is still a genuine primitive, not an ordinary move-to-graveyard effect.

`CostDef::Named` wraps a cost expression and publishes one occurrence when
its selected payment completes. It does not infer identity from the inner
actions: sacrificing a Food while foraging produces both sacrifice and forage,
whereas sacrificing that Food for another purpose produces only sacrifice.
Casting preserves named completion boundaries and individual action batches
through cost expansion, including suspended battlefield-exit replacements.
`AbilityDef::mechanics` separately labels the ability itself. Cumulative upkeep
uses both: its triggered clause carries `CUMULATIVE_UPKEEP`, while its payment
publishes a typed `MechanicPayment` result (paid/unpaid, repetition count, and
actual mana colors spent). Ability identity is not inferred from rules text.

Cycling and typecycling carry the shared `CYCLING` ability identity and compose
mana plus `Named(CYCLING, DiscardSource)` costs. The discard publishes one
occurrence labeled with both `DISCARD` and `CYCLING`, so a "cycle or discard"
clause triggers once. Ordinary discards use the same semantic action without
the cycling label. Self-listeners follow the discarded card into its actual
destination; other battlefield listeners see that same event. Copying or
countering the draw/search ability does not repeat or undo its payment.
The hand activation adapter currently admits named source-discard actions,
not arbitrary named activation programs.

`AbilityPredicateDef::Mechanic` queries an individual ability's metadata.
Fluctuator uses it in an ordinary ability-cost reduction; channel on the same
card remains undiscounted. Lightning Rift observes the shared cycling event
and composes ordinary target selection with a resolving optional mana payment.
Both reference cards remain local declarations. Event identities are not new
checkpoint fields: captured triggers and frozen abilities retain authored
locators, including across a discarded source's zone change.

Corpseberry Cultivator is the reference composition. Its optional combat
forage and Feed the Cycle's additional cost use the same set-owned helper;
its second clause observes forage independently of where it was performed.
Object costs share one eligibility and selection-validation layer across
casting, activation, and resolving payments. Sacrifice, discard, and exile
remain distinct actions; quantity and aggregate constraints describe their
selections, not separate execution-specific cost variants. A resolving window
collects exact-count or aggregate-constrained choices before commitment.
Aggregate choices can exceed the threshold and use signed characteristic values.
Hand choices stay private. Large activation selections precede mana and other
cost payments; small selections may remain in the announced action. Mana
planning reserves selected objects against incompatible consumption.
Entry replacements retain their single-card payment adapter, backed by the
same eligibility, validation, and commit routines; hand candidates are private.

Cancelling a resolving payment selects its unpaid continuation without rewind.
Cumulative upkeep uses this same window while keeping the added age counter.
Checkpoints retain the authored locator, ordered selection answers, and
tentative aggregate selections; reconstruction validates the offer. A committed
payment suspended by an ordinary draw replacement retains its remaining action
suffix, frozen repetition counts, and actual mana spent. It resumes without
re-paying earlier actions. Unsupported or unlocatable suspended state fails
closed. Checkpoint v14 removes superseded payment continuations.
Paused activation declarations remain outside checkpoint coverage, as before.
The prepared engine can fall back to the semantic lane before mutation.

This is not yet a general joint casting/activation planner. Casting still
enumerates combinations and limits aggregate-cost offers to minimal selections.
Arbitrary action programs and remaining activation-cost families remain
follow-ups. Cost modifiers' remaining activation/source-specific shapes can
be consolidated around ability, source, and activating-player selectors.

## Contract for local runtime exceptions

A bounded runtime exception is a legitimate future extension. It need not be
generalized merely to claim declarative coverage. This migration does not add
that interface; introducing one requires an explicit design for:

- ordinary timing, targets, source identity, stack use, and ability granting;
- engine-mediated actions, replacement processing, and event ordering;
- allowed state queries, hidden information, and engine-owned seeded randomness;
- typed inputs, outputs, and explicit resumable continuation state;
- catalog validation, complete coverage, checkpoint/replay reconstruction, and
  any affected compatibility migration;
- a correct reference path independent of optional prepared compilation.

A local callback taking unrestricted mutable `Game` is not this contract.
Neither are independently maintained affordability and execution callbacks.
When a card overrides a shared rule, expose the override and its lifetime
explicitly rather than teaching unrelated engine paths the card's identity.

## Resolving payment programs

Keep one general cost grammar. Common payments and locally composed action
costs should use the same payment window, selections, and execution validator.
Alternatives, bundles, and repetitions are composition, not new card-specific
cost variants. Card-local behavior remains local, including Herald of
Leshrac's choice of land and control-changing action.

The lifecycle is selection, executable-plan validation, and commitment:

- Collect all resource selections for one payment, allowing edits or decline
  where the rules permit it. Do not enumerate every complete combination as a
  separate player-facing choice.
- Validate the combined projected state and legal action order. An object may
  participate in several compatible actions; a tap-for-mana and a convoke tap
  cannot both consume its one untapped state. Mana contributions remain
  distinct from actual mana production and spending.
- A ready plan fixes resource assignments and accounts for required follow-up
  decisions. Revalidate relevant state before commitment. A plan promises
  legal execution, not that replacements leave every intended outcome intact.
- Do not use visible mutation followed by general rollback as cancellation.
  Protect both players' information and the recorded random stream. Costs
  involving random or information-producing actions need explicit treatment;
  they are not permission to simulate hidden outcomes for the planner.
- Publish a typed payment result and continue the caller's procedure. Scope
  decline to the payment: declining cumulative upkeep does not undo its age
  counter. `abilities::cumulative_upkeep!(cost)` constructs an ordinary tagged
  upkeep trigger with a source-on-battlefield guard, counter addition,
  `PayOr` of a named `Repeat`, and an unpaid sacrifice consequence.

`Repeat` freezes the number of unit payments in one window. Choices are made
independently per repetition; the planner checks the entire quantity before
commitment, while execution retains individual action/event boundaries.
`All` collects a constrained bundle of mana, life, and fixed object actions.
Selected objects are reserved against incompatible mana-source consumption,
and mana production shares the remaining life budget. A creature may tap for
mana and then be sacrificed, but cannot be sacrificed twice. The existing
convoke lane still owns its distinct tap-contribution reservations.

Herald of Leshrac authors `CostDef::Action` around ordinary `ChooseExact` and
`GainControl` in Coldsnap. The payment interpreter currently admits that bounded
action shape and rejects unsupported programs; it does not identify Herald or
carry a gain-control cost variant. Herald's independent leaves-the-battlefield
trigger is also authored locally, including lands acquired by other means.

The supported subset is deliberate: mixed snow/ordinary allocations, differently
named mana subcosts inside bundles, repeated aggregate-threshold selections,
and arbitrary information-producing bundles need additional planning support.
Existing repeated draw/coin-flip costs do not preview hidden cards or randomness:
they begin only after commitment and retain ordinary replacement processing.
These limits are coverage boundaries, not silent approximations.

The runtime payment plan is semantic state. It is distinct from the optional,
catalog-derived programs in `src/prepared_engine` and must work with prepared
execution disabled. No fused cumulative-upkeep instruction is required in the
core engine. Prepared execution may recognize its composition later without
changing the authoring grammar or observable boundaries.

## Subsequent migrations

Audit engine procedures as three different categories: genuine primitives,
misplaced compositions, and legitimate local exceptions. Start from concrete
consumers and preserve their complete behavior; do not shrink enums as an end
in itself.

- Endure can compose an effect choice, counters, and token creation. Detain
  and unleash need ordinary duration/conditional rule compositions while
  preserving mechanic identity and ability-grant/removal behavior.
- Typed effect outcomes should let later steps read actual damage, discarded
  or destroyed objects, and zone successors without dedicated follow-up fields.
- Separate cast permissions from exile selection/movement for Crabomination
  and related cards; preserve group-wide cast limits and resolution timing.
- Give Grist a resumable local repetition program; give Doomsday a genuine
  multi-zone search and independently authored remainder handling.
- Separate voting from Council's Judgment's exile consequence. Conspiracy
  should own the voting composition, with card-specific outcomes.

Preserve observable mechanic identity and event boundaries when expanding a
keyword. Keep genuinely atomic operations, such as exchange and simultaneous
damage, atomic. Ordinary sequential effects do not acquire the payment
window's all-or-nothing guarantee.

## Prepared execution

The [prepared engine](prepared-engine.md) may recognize and fuse compositions
regardless of whether a common helper, a set helper, or a card authored them.
Preserve choices, events, identity, and continuation boundaries. Select a full
supported lowering or the reference implementation before mutation; local
programs must remain correct when no lowering exists. Do not put optimization
flags or prepared payloads in card declarations.
