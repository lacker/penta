# Implementing cards

This guide describes where card behavior belongs in the current engine. The
[design doctrine](design-doctrine.md) explains why implementation boundaries
are preferences rather than purity requirements. The source-layout rules in
this guide are invariants.

The [shared game action guide](game-actions.md) describes cost and effect wrappers
for discard, sacrifice, and gain-control programs.
The [named mechanic programs draft](composed-mechanics.md) describes the current
cycling and cumulative-upkeep composition slice and its payment boundaries.

## Definition boundary

Each built-in canonical card is declared once in the `CARDS` registry of its
representative set module. Prefer the card's first paper printing in an
English-language set when one exists; use its first paper printing in any
language only when no English printing exists. In the local Scryfall index,
this means selecting a nondigital, non-reprint `printings` row with `lang = en`
when possible rather than letting a non-English printing or a preview reprint
own the definition. Prefer the ordinary set row over a same-day promo treatment
when both introduce the identity. Its `CardRecord` keeps its identity and primary `CardRules`
together: name, cost, types, creature stats, and ordered ability clauses can
all be understood at the declaration.

Within a printed set module, keep declarations and the `CARDS` registry in
natural collector-number order, with `CARDS` exactly mirroring declaration
order. Compare numeric portions numerically (`8`, `8a`, `8b`, `16`), not
lexicographically. Introduce each declaration with an identifying comment in
the form `// LEA 230 — Ankh of Mishra`, using the canonical printing's uppercase
set code, collector number, and card name. For a double-faced card, list both
faces in front-to-back order, such as `// ISD 51 — Delver of Secrets //
Insectile Aberration`. A modeled double-faced `CardRecord` uses the same
combined `front // back` name while the catalog retains the front name as a
deck-list lookup alias. Define both modeled faces together with
`CardRecord::new_dfc` for a transforming card or `CardRecord::new_mdfc` for a
modal double-faced card; pass their named face rules directly to those
constructors so they can derive the parts, topology, and play options.
The header immediately starts the declaration block. Inline every card-local
cost, target, effect, ability, predicate, query, value, and collection directly
in the `CardRecord` and its ordered `CardRules` clauses. A named card-local
component is allowed only when the definition references it more than once or
when it is genuinely recursive or self-referential. Shortening or visually
decomposing a declaration is not a reason to extract a component. A shared
power/toughness value qualifies because both characteristics reference it;
power/toughness values are not otherwise a special exception. Keep every
allowed extracted component after the header and before the `CardRecord`,
adjacent to the clause it supports and in printed-clause order.

An incomplete identity uses `blocked` when it has no declaration or
`unsupported` when it has a whole-card `CardRules::unsupported()` declaration.
Blocked header-and-audit pairs stand alone at the identity's collector
position. Reprints do not repeat the audit. Keep every identity
header in natural collector order. The header identifies the canonical
printing and its debut artwork; later or alternate artwork belongs to an
additional printing record.

Helpers used by more than one card are set-level vocabulary rather than part of
one declaration block. Keep them at the top of the set module before the first
identity header, or promote generally reusable behavior to `card::abilities` as
appropriate. Do not leave card-local helpers in the shared preamble or between
other cards' blocks.

Every `CardRecord` constructor takes the card name, exact debut printing's
Scryfall UUID, artist, and rules in that order. The surrounding set module
supplies the debut `CardSet` when the record becomes a catalog definition, so a
declaration cannot disagree with its registry. The build normally derives a
stable JavaScript-safe 52-bit definition ID from the debut UUID and rejects
collisions across the whole corpus. Historical ID assignments that cannot be
derived from that UUID live exclusively in
`src/card/compatibility/definition_ids.txt`; never copy them back into card
declarations or allocate another sequential ID. That compatibility table is
fingerprinted so existing assignments cannot move. The ordinary
`card::cards::*` constants remain generated compatibility output, not an
independently authored ID registry.

Keep `ADDITIONAL_PRINTINGS` in natural order by the collector number in that
module's set, including for reprint-only modules with an empty `CARDS`
registry. Declare each printing as a card-derived constant immediately below
its collector-ordered header, such as `SAVANNAH_LIONS_REPRINT` or
`URZA_S_MINE_ALTERNATE_2`, and put only those constants in the bottom registry.
Pass the exact printing's Scryfall UUID and artist directly to the adjacent
`PrintingRecord::reprint(...)` or `PrintingRecord::alternate(...)` constructor.
Empty registries need no comments. Creator-owned token and emblem
characteristics and rules-owned face-down characteristics are built by the
effect or mechanism that creates them, are not card definitions, and remain
outside these conventions.

Use `// M14 1 — Ajani, Caller of the Pride (reprint)` for the default printing
and `// SET NUMBER — Name (alternate printing)` for another art or variant.
The constant immediately following each comment keeps the identity, kind, and
artwork readable in one place. The source-organization test checks that these
constants exactly mirror the additional-printing registry.

Start new and migrated work with the card's ordered `AbilityDef` clauses. Each
printed clause should carry its explicit timing category and, where applicable,
its costs, targets, and effect. Displayed rules text derives from those clauses;
implementation status is the whole-card choice `Complete` or `Unsupported`.

Reuse constructors from `card::abilities` and declarative rules primitives
where they fit. Keep rules text and execution tied to the same clause.
Card-specific execution is not an extension boundary.

Card declarations are oblivious to the
[prepared engine](prepared-engine.md). Do not add preparation flags, prepared
executors, optimization hints, or special constructors to a declaration or the
card schema. Declare the semantic operation in the ordinary model. The
prepared compiler may recognize that structure and lower it independently;
unsupported structures continue through the reference implementation.

## Extension boundaries

Use the smallest boundary that truthfully implements the behavior:

- A recurring mechanic or general Magic rules concept belongs in a reusable,
  card-agnostic primitive that ability definitions can invoke.
- Genuinely card-specific composition belongs directly in the relevant card's
  declarative ability clause. When the required semantic shape is reusable,
  add a shared primitive rather than a card-scoped resolver.
- If neither the definition nor a reasonably scoped shared primitive can
  express the complete card, make the whole card `CardRules::unsupported()`.
  Do not add a direct card-identity branch in generic `Game` or state-machine
  flow, and do not expose a working subset of the card.

Resolution must not silently change an explicit ability category or let a
supported activated or triggered non-mana ability bypass the shared stack.

Declare Evoke once with `abilities::evoke(costs)`. It expands into the alternative
cost and its sacrifice trigger, with their shared binding handled internally.
Use `ability_list!` to flatten complete ability groups in source order:

```rust
.with_abilities(&crate::ability_list![
    [abilities::flying()],
    abilities::evoke(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
])
```

The constructor also accepts the colored-card exile costs. Use
`abilities::evoke_with_text(costs, text)` for other nonmana wording or a printed
text override. Both constructors always return the complete mechanic.

Other clauses that refer to a particular alternative cost can use
`TriggerConditionDef::SourcePaidAlternativeCost(binding)`, paired with
`.with_alternative_cost_binding(binding)` on that cost. Ability order does
not affect the link.
Catalog validation rejects duplicate cost names within a card part and
references to undeclared cost names. These names occupy a separate namespace
from effect-output bindings; `ParentBinding` cannot name a cost.
`SourceCastWith` instead asks about a cost family such as escape. External
alternatives such as Omniscience do not acquire the card's cost bindings.

### Damage instructions and follow-ups

Use `EffectDef::DealDamage(DamageDef)` for ordinary damage instructions. Use
`EffectDef::damage(recipient, amount)` for the resolving spell or ability's
source, `EffectDef::damage_from(source, recipient, amount)` for an explicit
source, or `EffectDef::damage_simultaneously(assignments)` for several
assignments in one event. These constructors share one evaluator. An explicit
source retains its identity and last-known information even after sacrifice.
Every assignment is evaluated before any damage in that event is committed;
separate damage effects in a `Sequence` still represent separate events.

Attach an outcome-dependent rider with `DamageDef::with_follow_up`:

- `DamageFollowUpDef::IfDealtToIntended(then)` runs once if any intended
  recipient actually took damage. Use it for wording such as Mishra's War
  Machine's "If it deals damage to you this way". Full prevention or redirection
  entirely elsewhere skips the continuation; life-total changes are not a
  substitute for damage dealt.
- `DamageFollowUpDef::ApplyToDamaged { effect, duration }` applies to the actual
  damage recipients, including recipients reached by redirection. Use it for
  wording such as "a creature dealt damage this way".

Both riders work with ordinary sources, explicit sources, and simultaneous
batches. Use an ordinary `Sequence` for an unconditional instruction following
damage. Keep `EffectDef::Fight` for fighting: it requires both participants to
be creatures and snapshots their powers before either deals damage.

### Temporary self effects

Use `abilities::apply_to_self_until_end_of_turn` for activated stat changes,
ability grants, or combinations that last until end of turn. It accepts the
complete `&[CostDef]` alongside the effect:

```rust
abilities::apply_to_self_until_end_of_turn(
    "{B}: This creature gets +1/+1 until end of turn.",
    &[CostDef::Mana(mana_cost!("{B}"))],
    AppliedEffectDef::modify_power_toughness(
        ValueDef::Constant(1),
        ValueDef::Constant(1),
    ),
)
```

`AppliedEffectDef::modify_power_toughness` accepts signed or dynamic `ValueDef`
deltas. Pass `AppliedEffectDef::add_ability` for an ability grant, or an
`AppliedEffectDef::Composite` when a clause changes stats and grants an ability
together. The returned `AbilityDef` supports ordinary modifiers such as
`.once_each_turn()`. The helper constructs ordinary activated clauses using
`EffectDef::Apply`; the shared engine still owns cost payment, stack resolution,
and cleanup expiration.

## Coverage

Executable clauses use declarative effects and carry no separate behavior
identity. Unsupported cards may exist in catalogs and hidden zones, but the engine does not offer play options that
would resolve as silent no-ops.

When complete fidelity is too large for the current increment, leave the card
unsupported and state the missing shared capability in its audit comment. A
reusable primitive may land independently, but the card becomes executable
only when its complete printed behavior is declarative.

## Implementation workflow

1. Confirm the printed clauses and the format or card interaction being added.
2. Represent the clauses, categories, costs, targets, and effects in
   the card definition.
3. Reuse an existing primitive or add a shared primitive according to the
   preference ladder above; otherwise retain a whole-card unsupported declaration.
4. Test new shared rule behavior once at the narrowest useful boundary. Add a
   card-level test only for text-sensitive composition, a legality boundary,
   or an interaction that could fail while the shared primitive still passes.
5. Run final focused native checks for the changed behavior before handoff.
   Add browser validation only when the card work also changes a browser-facing
   contract or web code; leave aggregate integration coverage to PR CI.

Before adding a card test, find the closest existing mechanic test. A second
card using the same primitive does not need another happy-path dispatch test;
extend the mechanic test only when the new card contributes a distinct case.
Do not test a named card merely to assert derived implementation status or audit
classification. Source audits and generated catalog reports own coverage facts.

## Adding a set or fixed card pool

Treat registration and catalog inventory as one change, even when card rules
will be implemented later:

1. Establish the complete legal set list from the format's authority and put
   that list in its `SetFormatDefinition`. For a cube, preserve the exact fixed
   list in its `CubeFormatDefinition` instead of inferring legality from sets.
2. Query the local Scryfall index for every paper printing in each newly legal
   set. Work in natural collector-number order and keep alternate-art collector
   numbers distinct. For a cube card, resolve its earliest English-language
   paper printing when possible, otherwise its earliest paper printing, and
   use that exact UUID and artist in its `CardRecord` constructor.
3. Leave an existing `CardRecord` unchanged. If the identity is already
   declared elsewhere, add a `PrintingRecord::reprint` (and `alternate` records
   for further variants) plus the corresponding ordered upper comments.
4. If an identity has only a standalone blocked audit row, either replace that
   row with the reprint comment and printing record, or turn it into a
   `unsupported` audit followed by a `CardRecord::new` whose rules are
   `CardRules::unsupported()`. Preserve a useful existing capability-gap
   explanation. When there was no row, add the same unsupported declaration
   with `Card rules have not been implemented.` as its honest initial audit.
5. Put a new identity in the module for its first English-language paper set
   when possible, otherwise its earliest paper set, using that exact debut
   printing's UUID and artist. When no modeled set can truthfully own the
   declaration, add a set module with a `SET` constant referencing static
   `CardSetMetadata` with its uppercase official code and stable catalog slug,
   plus a `DEFINITION` joining that set to `CARDS`, `ADDITIONAL_PRINTINGS`, and
   `file!()`. Declare the module in its
   release year's `mod.rs` and add its `DEFINITION` to `SET_MODULES` in
   `src/card/sets/mod.rs`. Set metadata belongs only in the set module;
   protocol serialization and source audits consume that declaration. Native
   callers use `card::sets::<module>::SET`. Preserve existing wire slugs when
   reorganizing sources. Append-only catalog growth does not require a
   protocol-version bump.
6. Make `CARDS` exactly mirror declaration order and
   `ADDITIONAL_PRINTINGS` mirror the ordered reprint comments. Run the focused
   source-organization and format-coverage tests, then `make catalog-report`.
   Every set format should account for its whole legal identity corpus, and a
   fixed cube should have no uncataloged names.

The [development guide](development.md) maps repository paths to validation
workflows. Current format and card coverage is described in
[formats and scope](formats.md).

## Declaring costs

Use a plain `&'static [CostDef]` for an alternative cast, optional additional
cost, resolving payment, or parameterized mechanic. Supply only the mechanic's
variable cost: `cycling!`, `typecycling!`, and `bloodrush!` append `DiscardSource`;
`scavenge!` and `eternalize!` append `ExileSource`; `ninjutsu!` appends
`ReturnUnblockedAttackerToHand`. These constructor macros concatenate costs at
compile time into an ordinary static slice. For example,
`abilities::eternalize!("Eternalize {2}{G}{G}", &[CostDef::Mana(mana_cost!("{2}{G}{G}"))])`
already includes exiling the source. Runtime procedures can build a `Vec<CostDef>`
for computed costs; shared payment helpers accept slices.

For example, `&[CostDef::Mana(mana_cost!("{2}{U}"))]` is a mana-only alternative,
while `&[CostDef::Mana(mana_cost!("{1}")), CostDef::DiscardCards(1)]` asks for both
payments. Use authored text when a complex predicate needs wording the shared
cost renderer cannot derive.

`CostDef::discard(predicate)` discards one matching card by default. Use
`.with_quantity(CostQuantityDef::Fixed(2))` for two cards, or a computed
quantity such as `.with_quantity(CostQuantityDef::ChosenX)` where the payment
procedure supports it.

`PayOrDef::optional(costs, if_paid)`, `optional_or(costs, if_paid, otherwise)`,
and `unless(costs, otherwise)` ask the effect controller to pay by default.
Use `.with_payer(...)` when the text names another player, such as the controller
of a targeted spell. Each constructor takes the complete cost slice directly.

The surrounding procedure determines which expressions it can execute;
keep its runtime-support validation honest when adding a new expression there.

`NO_COSTS` (`&[]`) expresses an alternative paid without paying a mana cost,
such as Omniscience. It differs from `&[CostDef::Mana(mana_cost!("{0}"))]`:
only the latter includes a mana payment. CR 601.2g checks the complete selected
payment, including additional costs and increases; reducing a mana component
to zero preserves its presence. Neither form changes the card's printed mana
cost. Keep that printed characteristic separate from mandatory additional
costs in the text box, such as Bone Shards' sacrifice-or-discard cost.

Resolving payments can contain `CostDef::repeated(costs, count)`. The count is a
`ValueDef` evaluated when offered; only that sub-list repeats, and the complete
payment remains all-or-nothing. Use `PayOrDef::optional`, `optional_or`, or
`unless` to choose the continuation, with `.labeled(...)` when observers or
mana restrictions care about the payment's purpose.
