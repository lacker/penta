# Shared game action programs

`GameActionDef` describes selecting objects and changing game state.
`EffectDef::Perform` executes the program during resolution;
`CostDef::Perform` asks the resolving payment planner to satisfy it as an
obligation. The wrapper determines the execution contract. The action retains
its rules identity, including discard and sacrifice events, last-known
information, and replacement handling.

For example, the same program can appear in either position:

```rust
const DISCARD_THREE: GameActionDef = GameActionDef::choose_discard(
    ObjectPredicateDef::Any,
    ValueDef::Constant(3),
);
// Discard as many as possible during resolution:
EffectDef::Perform(DISCARD_THREE)
// Require all three cards for payment:
CostDef::Perform(&DISCARD_THREE)
```

`Choose(GameActionChoiceDef)` contains the chooser, candidate query, count,
visibility, binding, and action to perform on that binding. `Sequence` composes
actions in order. The initial semantic operations are `DiscardCards`,
`Sacrifice`, `SacrificeYours`, and `GainControl`. The two sacrifice forms state
who must sacrifice: each object's controller, or the executing player.
Neither lowers to a generic zone move.

## Selection, commitment, and completion

During effect resolution, a fixed-count choice is clamped to the available
objects. During payment, the planner requires the exact count and a complete
legal plan for the entire cost list. Selecting an offered complete group
confirms that plan; no separate UI confirmation action is introduced. An
insufficient hand cannot pay a three-card discard cost, and the engine does
not discard the available two.

The payment path distinguishes three stages:

1. **Plan:** freeze quantities, enumerate complete choices, and reserve objects
   across components. No cost action has happened yet.
2. **Commit:** validate the chosen complete plan and enqueue its action bodies.
   The player has accepted payment. Replacements may change what happens to
   the objects without turning payment into a refusal or failure.
3. **Complete:** finish action execution and replacement choices, then publish
   payment events and execute `if_paid`. A pending replacement decision keeps
   `CompletePayment` behind the unfinished work.

This follows CR 118.11 and 118.12: payment is not a postcondition such as
"three cards are now in the graveyard." The action must be legally payable
when the player commits; a replacement can send those cards elsewhere. A
sequence finishes each action's replacement work before its next action.

## Initial payment boundary

The resolving planner accepts independent exact selections from the payer's
hand for discard, their battlefield permanents for sacrifice, and permanents
they do not control for gaining control. A selection must feed its own bound
group directly into the corresponding action. Counts use existing value
expressions, evaluated when the offer is created; `CostDef::Repeated` scales
the obligation. Sequences reserve distinct objects across their components
using the existing complete-list planner.

The catalog rejects unsupported action payment shapes. This does not add
arbitrary branching, dependent selections that require earlier actions to
create candidates, or choices between non-scalar cost programs. Casting and
activation retain their existing supported cost shapes and specialized
variants. Higher-level discard and sacrifice effects carrying random choices
or outcome-dependent continuations also remain. Further migrations can lower
those front ends to the same semantic operations.

Herald of Leshrac supplies a gain-control action as its cumulative-upkeep cost.
Its separate leave-the-battlefield trigger returns every applicable land to
its owner, including lands acquired through other effects. The control action
has indefinite duration; the trigger later changes control. Vexing Sphinx,
Polar Kraken, and Phyrexian Soulgorger exercise discard and sacrifice payments.

## Traversal and reconstruction

`child_effects` exposes action sequences, selected-action bodies, and action
costs beneath `PayOr` and `WithCosts`. Validators and semantic checkpoint
locators can therefore reach the original authored program. Suspended
execution retains the lexical cost parameter and original ability. An action
payment's checkpoint records its frozen source and quantity; its program is
recovered from that ability rather than serialized as executable code.

The additive action-payment tag and resolving-control timestamp remain within
checkpoint format 15. Reconstruction still requires the simulation fingerprint.
Existing unavailable states, including battlefield-exit replacement decisions,
remain explicitly deferred. Bot-wire and replay versions are unchanged. The
prepared engine uses its existing reference fallback for these programs.

See [named mechanic programs](composed-mechanics.md) for payment provenance and
lexical cost parameters, and [implementing cards](implementing-cards.md) for
the extension and validation boundaries.
