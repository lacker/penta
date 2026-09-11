# Named mechanic programs: first slice

Cycling and cumulative upkeep exercise a composition boundary for the reference
engine. Card authors keep reusable mechanic constructors, while those
constructors supply inspectable ordinary ability definitions and effect
programs. The prepared engine may derive specialized executors from those bodies.

## Design commitments

Shared discard, sacrifice, and control operations now use
[game action programs](game-actions.md), with separate cost and effect wrappers.

1. A semantic label identifies an ability or payment purpose. It never selects a
   hidden reference implementation.
2. A payment retains its purpose and repetition count. Its result records actual
   mana expenditure, and ordinary trigger matching can inspect that result.
3. A suspended program retains its lexical cost parameter, source, ability
   origin, and remaining instructions. Payment completion waits for cost actions
   and their replacement choices before publishing the result or running a branch.

## Ability identity

`AbilityDef::labeled(mechanic_id)` attaches identity to the entire ordinary
clause. Its activation/trigger category, costs, targets, and executable body stay
in the usual fields. `AbilityPredicateDef::Label` uses that identity when
querying or removing abilities. Granting and copying a complete `AbilityDef`
carry the label together with its behavior. Existing `AbilityOrigin` and stack
object identities distinguish separate instances and invocations.

The shared label constants are `abilities::CYCLING` and `abilities::CUMULATIVE_UPKEEP`. Other labels
use the same representation; the interpreter does not match those constants.
Label names are authored semantic vocabulary, independent of displayed text
and local object bindings.

Alternative-cost bindings, as used by Evoke, link a declared alternative to
its companion trigger on the same source. Ability and payment labels instead
classify behavior across sources; they do not select or replace that binding.
Evoke keeps its `SourcePaidAlternativeCost` link and its complete constructor.

Cycling is an ordinary activated ability from hand with mana and discard costs
and a draw effect. Typecycling uses the same identity with its ordinary search
body. A labeled activation's source discard supplies
`TriggerEventDef::DiscardedToActivate(label)`. Identical costs on an unlabeled
channel-shaped activation do not establish cycling identity.

## Parameterized composition

The model currently borrows composite bodies as static data. A reusable `const`
constructor cannot allocate a new recursive sequence around an arbitrary cost list.
`EffectDef::WithCosts { costs, effect }` supplies one immutable lexical cost-list
parameter to a reusable body. `CostDef::Parameter` expands that parameter into the surrounding list at a
payment instruction. Nested suppliers shadow it; a later sibling retains its
own scope. Supplied costs are borrowed static slices, just like composite
bodies, so a suspended scope retains a reference rather than another copy of
the cost list. Parameters can appear within repeated lists and scalar choices.
Catalog validation rejects unbound parameters and recursive parameter suppliers.

The cumulative-upkeep constructor supplies its cost to this ordinary program:

```text
if source is on the battlefield:
    add one age counter
    offer payment:
        purpose: cumulative upkeep
        costs: repeat(supplied parameter, current age-counter count)
        otherwise: sacrifice source
```

`CostDef::Repeated { costs, count }` evaluates its count when the payment is
offered. `CostDef::repeated(costs, count)` constructs that node. Repetition
belongs to the cost tree: a list may combine costs paid once with repeated
sub-lists, and repeated nodes may nest or occur inside scalar choices. Each
repetition chooses its alternatives independently. Zero repetitions add no
cost, including no mana-payment component.

`PayOrDef` offers the resulting complete obligation atomically and chooses its
continuation. For cumulative upkeep, adding the age counter runs before the
cost expression is evaluated. The frozen obligation remains attached to the
payment decision.

Authors normally use `PayOrDef::optional`, `optional_or`, or `unless`; their
shared `new(costs)` constructor defaults to an unlabeled, single payment
with private visibility and the effect controller as payer. `.with_payer(...)`
selects a different payer, `.labeled(purpose)` supplies the payment's semantic
purpose, and `.with_visibility(visibility)` overrides the default. Repetition
is expressed in the supplied costs; it is not metadata on the payment offer.
Ordinary payment clauses do not need to specify unused fields.

`ManaRestrictionDef::Payment(label)` restricts mana by the purpose of that
obligation. Adarkar Unicorn therefore participates in payment planning before
anything is spent. `PaymentPaid` and `PaymentNotPaid` trigger matchers inspect
that purpose; Balduvian Fallen reads the colors actually spent on its payment.
Payment events do not infer a repetition count from a potentially mixed cost
tree. Heart of Bogardan reads the source's current or last-known age counters
through the ordinary value expression.

## Suspension and reconstruction

`ScopedEffect` carries the lexical cost parameter. Checkpoints recover it by
walking the retained ability's effect path through enclosing `WithCosts` nodes.
Equal leaves beneath different cost suppliers remain distinguishable.

A `PayOr` decision retains generic payment provenance. Once the choice is
settled, `PendingProcedure::CompletePayment` follows any suspended cost work
and precedes the program's remaining instructions. It publishes the outcome
and selects the ordinary authored branch. It has no upkeep-specific logic.

Checkpoint format 15 records generic payment state and completion results.
The bot wire epoch and replay version do not change; reconstruction requires
the matching checkpoint capability and simulation fingerprint. See the
[checkpoint migration](bots.md#migrating-checkpoint-format-14-to-15).

## Scope and open design questions

This slice supports the repeated payment shapes used by implemented
cumulative-upkeep cards, plus life, energy, and mill payments. Every payment
uses the shared cost-slice interface. A lexical parameter supplies a complete
list, and repetition scales that complete obligation.

Nested mana/life `CostDef::Choice` expressions use the scalar combination
planner shared with spell additional costs. Each repetition may choose a
different alternative; the resulting complete alternatives enter the same
resolving cost-list planner as ordinary `PayOr` and special-action payments.
There is no separate scalar-batch settlement path. The list planner combines
mana with the same purpose, reserves life and selected objects before mana
abilities, and checks the whole plan before committing it. Explicit `{0}`
retains mana-payment identity; an empty list does not acquire it.

Mixed lists of supported scalar and object costs use that same boundary.
Repeated discard and sacrifice costs select their complete required group;
different components cannot spend the same selected object. The shared
completion procedure waits for suspended cost actions and replacement choices
before publishing the payment result. Independent exact selections in shared
action programs use this planner too. Arbitrary action programs and choices
between non-scalar costs still need further ordering and replacement support,
and remain rejected for repeated/labeled payment programs. The new dynamic
repetition node is supported by resolving payment procedures; casting and
activation retain their existing supported quantity expressions.

The initial payment trigger matchers observe the source's own named payment.
A broader event query for other objects' payments, linked cost-component
references, and bundles containing several named clauses need further design.
Forage now uses a [named game-action program](effect-programs.md#identities-and-named-actions)
through the ordinary cost and effect wrappers.
Persist and undying now compose ordinary death triggers and shared zone-move
actions. Class levels and other candidate mechanics remain follow-ups.

Review should focus on whether:

- the lexical cost supplier is a useful parameterization boundary;
- ability identity and payment purpose should remain separate fields sharing
  the same label vocabulary;
- the shared planners preserve all resource reservations across casting and resolving costs;
- payment completion has the right result and suspension contract;
- a second migration needs several named parameters or a richer cost expression.

No prepared optimization is added. A future lowering must recognize the whole
body and relevant arguments and preserve reference events, decisions, and
identities. A label alone is insufficient evidence for a lowering. See the
[prepared-engine guide](prepared-engine.md).
