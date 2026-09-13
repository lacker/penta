# Shared game action programs

`GameActionDef` describes selecting objects and changing game state.
`EffectDef::Perform` executes the program during resolution;
`CostDef::Perform` asks the payment planner to satisfy it as an
obligation. The wrapper determines the execution contract. The action retains
its rules identity, including discard and sacrifice events, last-known
information, and replacement handling.

For ordinary effects, prefer the thin constructors on `EffectDef`:

```rust
use penta::card::{EffectDef, EffectRecipientDef};

const SACRIFICE: EffectDef = EffectDef::sacrifice(EffectRecipientDef::Source);
```

`EffectDef::sacrifice(x)` is exactly `actions::sacrifice(x).as_effect()`.
`discard_cards`, `sacrifice_yours`, `gain_control`, and `move_to_zone` provide the same shortcuts
for the other shared primitives. These constructors operate on already
identified objects, preserving the distinct sacrifice actors and the explicit
controller and duration of control changes. They add no selection or separate
runtime implementation.

When composing a custom program or describing a cost, construct actions first,
then choose the execution contract with `as_cost()` or `as_effect()`. For example:

```rust
use penta::card::{
    actions, abilities, AbilityDef, CardType, CostDef, EffectDef, ObjectPredicateDef,
};

// Discard as many as possible during resolution:
const EFFECT: EffectDef = actions::choose_discard(3).as_effect();
// Require all three cards for payment:
const COST: CostDef = actions::choose_discard(3).as_cost();

// Herald of Leshrac's unit upkeep obligation:
const UPKEEP: AbilityDef = abilities::cumulative_upkeep(&[
    actions::choose_gain_control(1)
        .matching(ObjectPredicateDef::HasType(CardType::Land))
        .as_cost(),
])
.override_text("Cumulative upkeep—Gain control of a land you don't control.");
```

These `const` constructors return the ordinary `GameActionDef` directly;
there is no separate authoring representation or interpreter. The conversion
methods also work on explicit `GameActionDef::Choose` and `Sequence` programs.
`as_cost()` borrows a static program, including an inline expression promoted
within a static card declaration, while `as_effect()` takes the action by
value. Outside constant evaluation, `as_cost()` requires an existing static
program or an explicit `const { ... }` expression. The stored wrappers and
payment validation are unchanged.

`choose_discard(n)`, `choose_sacrifice(n)`, `choose_exile_from_graveyard(n)`, and
`choose_gain_control(n)` supply
the ordinary candidate zones, player relations, visibility, and binding.
`matching(predicate)` replaces the candidate query's predicate while retaining
its ownership and control constraints. `with_amount(value)` accepts computed
quantities; `with_chooser(player)` and `with_visibility(visibility)` override
the corresponding selection fields. Setting a chooser does not change which
player's objects the query describes. Selection builders require a `Choose`
action; `matching` additionally requires query candidates. Misuse fails during
constant evaluation in a static declaration.

For a one-off selection, `actions::choose(binding, candidates, then)` defaults
to one public choice by the executing player. The body explicitly names that
binding, and all ordinary selection builders remain available:

```rust
use penta::card::{
    actions, ChoiceVisibilityDef, CostDef, EffectRecipientDef, ObjectQueryDef,
    ObjectPredicateDef, ObjectSetDef, PlayerRelation, PlayerSetDef, ValueDef, ZoneKind,
};
use penta::Binding;

const CUSTOM_COST: CostDef = actions::choose(
    Binding!("cards"),
    ObjectSetDef::Query(ObjectQueryDef::owned_by(
        ObjectPredicateDef::HasType(penta::card::CardType::Creature),
        &[ZoneKind::Hand],
        PlayerSetDef::Related(PlayerRelation::You),
    )),
    &actions::discard_cards(EffectRecipientDef::objects(
        ObjectSetDef::Binding(Binding!("cards")),
    )),
)
.with_amount(ValueDef::SourcePower)
.with_visibility(ChoiceVisibilityDef::Private)
.as_cost();
```

`discard_cards`, `sacrifice`, `sacrifice_yours`, `gain_control`, and `move_to_zone` operate on
already identified objects. They introduce no selection. `gain_control`
keeps the recipient player and duration explicit. `actions::sequence(&[...])`
composes actions before either conversion; custom trees can always be written
inline using the full definitions. Cost conversion does not make an unsupported
composition payable; the validation boundary below still applies.

Mechanic text helpers retain their ordinary defaults. Use `override_text()`
on the resulting ability for uncommon wording; exact printed phrasing does
not require a new semantic action constructor.

`Choose(GameActionChoiceDef)` contains the chooser, candidate query, count,
visibility, binding, and action to perform on that binding. `Sequence` composes
actions in order. The initial semantic operations are `DiscardCards`,
`Sacrifice`, `SacrificeYours`, `Exile`, `GainControl`, and `MoveToZone`. The two
sacrifice forms state who must sacrifice: each object's controller, or the
executing player.
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

`EffectDef::May` checks a leading object choice's authored minimum before
offering acceptance. Unlike a mandatory instruction, an optional two-card
discard cannot be chosen with only one card (CR 608.2d). This applies to
`Choose`, computed `ChooseExact`, shared action selections and alternatives,
and ordinary discard instructions. Private selections keep their eligibility
offers private. An empty library still permits an optional draw, as specified
by the same rule. Later instructions execute in order rather than being tested
against the state before earlier instructions have resolved.

For "you may discard ... When you do," use `EffectDef::Discard` under `May`,
with `EffectDef::ReflexiveTrigger` in its `DiscardFollowUpDef`. The discard
effect owns the choice, and the follow-up can bind the discarded cards for the
separate trigger. See [reflexive triggers](implementing-cards.md#reflexive-triggers).

## Initial payment boundary

`GameActionDef::Named` attaches a numeric `MechanicId` to a program before
either wrapper is chosen. `actions::choice(&[...])` requires one fully executable
alternative; unlike an ordinary `Choose` effect, an alternative is not partially
performed. Forage is the [reference use case](effect-programs.md#identities-and-named-actions).

Public alternatives use branch selection followed by an exact object selection.
Mixed bundles and repeated action obligations use the existing complete-list
planner, with shared reservations and no object reuse. A payment-purpose label
can coexist with an action's mechanic identity.

The catalog rejects unsupported action payment shapes: dependent selections that
need earlier mutations to create candidates, private branch offers, and arbitrary
nested programs. Casting additionally accepts fixed public sacrifice and
graveyard-exile action selections, with zone-distinguishable alternatives.
Activation retains its existing supported cost forms. Higher-level effects
carrying random choices or outcome-dependent continuations also remain.

Herald of Leshrac supplies a gain-control action as its cumulative-upkeep cost.
Its separate leave-the-battlefield trigger returns every applicable land to
its owner, including lands acquired through other effects. The control action
has indefinite duration; the trigger later changes control. Vexing Sphinx,
Polar Kraken, and Phyrexian Soulgorger exercise discard and sacrifice payments.

## Explicit payment and automatic planning

Paying a cost and finding an automatic payment are separate responsibilities.
`BeginPayment` opens an engine decision for a spell, ordinary activated ability,
mana ability, or a resolving mana payment. It remains available when automatic
source search fails and when the player prefers another payment.

An operation binds its targets, cost objects, modes, alternative costs, and X
before funding. A prepared projection reaches the operation's real payment
boundary, freezing its cost and reserving resources still owed. The player then
selects mana abilities and their costs, followed by exact individual mana units.
For spells, convoke, delve, and improvise bind each chosen resource to a specific
cost symbol; those contributions never create floating mana. Mana abilities run
before direct contributions and the remaining costs, with no intervening
priority or state-based actions.

The automatic allocator proposes individual mana units to the same validator
and committer used by explicit payment. Validation checks exact quantities,
restrictions, resource conflicts, and source identity. Committing a complete
proposal replays its validated funding program through the existing semantic
owners and preserves mana provenance and spend effects. An invalid or cancelled
proposal spends nothing. Resolving payments retain their enclosing continuation;
opening a payment choice never grants priority.

Payment decisions collect choices incrementally, including replacement answers,
and use the ordinary decision protocol for browser users, agents, and bots.
Pending proposals reconstruct from checked continuation data. Checkpoint support
fails closed where a viewer's hidden information cannot reproduce a proposal.

The existing context-specific cost syntax boundaries still apply. Unsupported
cost semantics require implementation in their semantic owner; improving source
search cannot make them executable. Conversely, a bounded automatic search does
not define which supported, explicitly supplied payments the engine accepts.

## Traversal and reconstruction

`child_effects` exposes action sequences, alternatives, named selected-action bodies, and action
costs beneath `PayOr` and `WithCosts`. Validators and semantic checkpoint
locators can therefore reach the original authored program. Suspended
execution retains the lexical cost parameter and original ability. An action
payment's checkpoint records its frozen source and quantity; its program is
recovered from that ability rather than serialized as executable code.

Checkpoint format 17 replaces the former forage-specific continuation with
authored action-choice state and numeric mechanic identities. Reconstruction
requires the matching simulation fingerprint. Replacement continuations retain
the action completion independently of payment completion. Bot-wire and replay
versions are unchanged. The prepared engine uses its existing reference fallback
for these programs. Unsupported battlefield-exit completion graphs still mark
checkpoints as deferred instead of reconstructing an approximate game.

See [named mechanic programs](composed-mechanics.md) for payment provenance and
lexical cost parameters, and [implementing cards](implementing-cards.md) for
the extension and validation boundaries.

## Death-return keywords

Undying and persist expand each effective keyword instance into an ordinary
zone-change trigger. The intervening condition reads the source's last-known
counter state. Its action program uses `actions::move_to_zone` to return the
exact `ZoneChangeResultOfTriggeringObject`; it never follows a later move or
selects the top card of a graveyard. Both keywords can trigger on one death,
and their controller orders those triggers through the ordinary stack rules.

`EffectDef::WithBattlefieldArrival` wraps that action's `as_effect()` with the
entry counter and ordinary owner-control default. Counters are part of the
prospective entry, so replacements and entry triggers observe them correctly.
`EffectDef::move_to_zone(object, zone, placement)` is a thin constructor for
`actions::move_to_zone(object, zone, placement).as_effect()`. The action is the
only stored representation; arrival and result wrappers retain their ordinary
validation, traversal, and continuation behavior.
This adds effect execution, not a new payable zone-move cost; the existing
payment planner continues to reject unsupported action obligations.

See the [consolidation audit](game-action-consolidation.md) for remaining effect
and cost candidates and the payment boundaries their migration must preserve.
