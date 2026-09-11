# Game action consolidation audit

The September 2026 audit compared the stored `EffectDef`, `CostDef`, and
`GameActionDef` vocabulary with selection, execution, and payment consumers.
The immediate duplicate was the plain zone move. Other candidates require
expanding the action or payment contract before removing their current forms.

## Completed: plain zone moves

`EffectDef::move_to_zone(object, zone, placement)` expands directly to
`actions::move_to_zone(object, zone, placement).as_effect()`, just as the sacrifice
shorthand does. `EffectDef` no longer stores a separate `MoveToZone` variant.
Card declarations, validators, traversal, choice prompts, and policy inspection
use the shared action. Execution enters the existing replacement-aware zone
resolver without translating back to a second effect opcode.

The shorthand is justified by the repeated ordinary instruction in card clauses.
It adds no selection, defaults, or runtime behavior. Custom programs can still
compose `actions::move_to_zone` before calling `as_effect()`.
`WithBattlefieldArrival` and `WithZoneMoveResult` remain effect wrappers: they
supply prospective entry state and a continuation following exact successors.
The prepared compiler retains its reference fallback. No observation or
checkpoint encoding changes; the generated simulation fingerprint covers the
changed declarations and engine source.

## Ranked remaining candidates

| Priority | Family | Recommendation and required boundary |
| --- | --- | --- |
| 1 | Discard and sacrifice costs: source/object forms, `Discard`, `DiscardCards`, `DiscardHand`, `Sacrifice`, `SacrificePermanent(s)` | Lower their committed operations to the existing discard/sacrifice actions, then retire redundant stored variants once every owning planner supports the normalized shape. Preserve actor/control checks, source references, computed quantities, reservations, large bounded selections, and activation ordering. Random discard and total-power sacrifice additionally need their selection contracts. |
| 2 | `CostDef::MoveToZone`, `Exile`, `ReturnToHand`, `MovePermanentMatching`, source and top-card exile/return forms | Normalize explicit movement after adding payable move plans, source-zone constraints, and output bindings. The current move cost can save successor identities for another cost or resolution. Its hand-to-graveyard path currently means **discard**, so it must lower to `DiscardCards`, not a generic move. Ninjutsu also retains its unblocked-attacker selection constraint. |
| 3 | `EffectDef::Discard` and `SacrificeOfChoice` | Keep selection and outcome handling as front ends; route the selected operation through the action boundary when extending those procedures. A simple choice can use an existing action program, but wholesale replacement needs random/APNAP selection, prohibitions, optionality, matched counts/types, last-known values, and follow-up bindings. Sequential `Choose` actions alone do not establish simultaneous selection. |
| 4 | `DrawCards`, `MillCards`, `PutCountersOnSource`, `AddMana`, `GainLife`, `CreateTokens`, `FlipCoins` costs and corresponding effects | Strong candidates for new semantic action leaves shared by both wrappers. Start with one complete resolving-payment slice, including replacement suspension and completion. These costs currently lower to specialized `ResolvedEffectPayment` variants; merely wrapping ordinary effects would bypass payment planning and reservation. Repetition may preserve event boundaries rather than multiply a scalar. |
| 5 | Tap/untap and counter operations in effects and costs | Share mutation through semantic actions while retaining cost eligibility. Tap-symbol source costs, chosen taps, crew/saddle power thresholds, exertion, and loyalty activation restrictions are distinct obligations. Counter removal also requires exact availability for payment while an effect can remove what is available. |
| 6 | `MoveObjects` and higher-level zone-moving effects | Share more of the movement executor only after giving the action the missing ordering/result contract. `MoveObjects` filters the source zone, preserves library group order, consumes bindings, records matched count/mana value, and binds actual successors. Linked exile, play permissions, face-down entry, and search procedures carry further state that a plain move does not express. |

### Evidence and migration seams

- [Action definitions](../src/card/model/game_actions.rs) accept payment programs
  consisting of independent exact `Choose` operations for discard, sacrifice,
  and gain control, plus sequences of those operations. A plain move is currently
  rejected as a payment. `as_cost()` does not confer new planner support.
- [Action payment execution](../src/game/game_actions/payments.rs) freezes the
  program/context and commits its selected body. The
  [complete-list planner](../src/game/decision_offers/cost_lists.rs) reserves
  selected objects and life across the full obligation. Migration must retain
  these pre-commit guarantees and the existing completion barrier.
- [Activation](../src/game/activation.rs) still recognizes specialized cost
  shapes and orders source-dependent costs around mana and departing objects.
  [Nonbattlefield movement payment](../src/game/activation/nonbattlefield.rs)
  explicitly maps a graveyard destination to discard. The
  [move-cost model](../src/card/model/costs/quantities.rs) also carries quantity
  and an optional successor binding; neither belongs to the plain move leaf.
- [Discard completion](../src/game/decision_piles.rs) captures matching cards and
  card types before movement, then supplies successors to follow-ups.
  [Effect resolution](../src/game/declarative_effects.rs) gives sacrifice offers
  their prohibition and outcome handling. Both already use semantic discard or
  sacrifice machinery; their additional procedure state is not redundant.
- [Repeated payment lowering](../src/game/effect_support/repeated_payment.rs)
  and [payment settlement](../src/game/decision_offers/effect_payment_resolution.rs)
  show the remaining resource-action duplication. Add the corresponding action
  definitions and completion support before replacing these payment variants.
- [Collection movement](../src/game/declarative_effects/object_collections.rs)
  and its [model](../src/card/model/effects/object_collections.rs) own ordered
  movement and result accounting. The plain move resolver does not express
  this entire contract.

## Boundaries to retain

Cost composition (`All`, `Choice`, `Repeated`, `Parameter`) and mana pricing
remain payment vocabulary. Paying life must preserve its affordability rules;
it cannot become an unchecked lose-life effect. Mana payment retains its
presence, restrictions, provenance, and atomic commitment.

Effect composition, choices, conditions, bindings, continuations, installed
rules/triggers, and continuous effects remain effect-program vocabulary.
`PutSpellIntoOwnersLibrary` retains the spell owner's top/bottom decision;
`PutIntoLibraryBeneathTop` retains a computed depth absent from `ZonePlacement`.
Destroy, counter, fight, draw, mill, discard, sacrifice, and exchange control
retain distinct semantic identities even if their implementations eventually
move into action leaves. Generic movement cannot replace those identities or
simultaneous/replacement boundaries.

The next bounded implementation should normalize one discard/sacrifice payment
family across resolving, casting, and activation planners, with reservation,
replacement, decision, and reconstruction tests. Removing all similarly named
cost variants before those consumers support the action shape would shrink the
schema by shrinking supported behavior.
