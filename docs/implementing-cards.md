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
The header immediately starts the declaration block. Inline card-local
cost, target, effect, ability, predicate, query, value, and collection directly
in the `CardRecord` and its ordered `CardRules` clauses by default. A named
card-local component is appropriate when reused, recursive, or when a coherent
local procedure materially improves understanding of genuinely complicated
behavior. Line count, indentation, and naming every predicate are not reasons
to extract components. Do not make a reader chase a chain of small helpers to
understand one ability. A shared power/toughness value qualifies because both
characteristics reference it; power/toughness values are not otherwise a special
exception. Keep every
allowed extracted component after the header and before the `CardRecord`,
adjacent to the clause it supports and in printed-clause order. Judge a one-use
local procedure by whether it keeps a coherent operation understandable, not
by a mandatory comment marker. Explain non-obvious constraints with ordinary
comments; do not repeat what the surrounding declaration already says. This
is not a blanket exemption for one-use constants or trivial wrappers.
Ordinary Rust functions, local variables, loops, and branches
are acceptable authoring syntax; keep timing, costs, targets, and the ordered
ability clauses apparent at the card.

Prefer ordinary borrows such as `&[abilities::flying()]` and
`&EffectDef::None` when they compile with the required lifetime. Retain
`&const { ... }` where it is needed to back a static reference, such as
constructor results inside helper functions or inline const blocks, and blocks
that compute a borrowed value using local bindings. A `const fn`
does not make its temporaries static; top-level static initializers and inline
const blocks do not have identical temporary lifetime behavior.

An incomplete identity uses `blocked` when it has no declaration or
`unsupported` when it has a whole-card `CardRules::unsupported()` declaration.
Blocked header-and-audit pairs stand alone at the identity's collector
position. Reprints do not repeat the audit. Keep every identity
header in natural collector order. The header identifies the canonical
printing and its debut artwork; later or alternate artwork belongs to an
additional printing record.

Helpers used by more than one card are set-level vocabulary rather than part of
one declaration block. Keep them at the top of the set module before the first
identity header. Limited-scope named set mechanics can also live there before
a second card is implemented; their rules-defined identity is the boundary.
Import a mechanic from its originating set when a few later sets reuse it.
Use `card::abilities` for broadly shared vocabulary and widely reused mechanic
programs. See the [ownership hierarchy](design-doctrine.md#ownership-of-behavior).
Do not leave card-local helpers in the shared preamble or between other cards'
blocks.

For example, use `sets::outlaws_of_thunder_junction::spree(modes)` and
`sets::eldritch_moon::escalate(text, cost, modes)` for those complete modal
mechanics. Use `sets::aetherdrift::exhaust(ability)` to label and restrict an
ordinary activation. Generic `ModalSpellDef` cost/selection data and
`AbilityDef::once_per_object()` remain available for unrelated compositions.

Every `CardRecord` constructor takes the card name, exact debut printing's
Scryfall UUID, artist, and rules in that order. The surrounding set module
supplies the debut `CardSet` when the record becomes a catalog definition, so a
declaration cannot disagree with its registry. The canonical printing UUID is
the definition's natural key. The build generates
compact `card::cards::*` IDs and their natural-key table from those declarations;
catalogs use the generated indices internally. Custom definitions resolve a
`CardDefinitionKey` to a process-local `CardDefinitionId` during construction.
There is no numeric ID allocation or compatibility table
for card authors to maintain. Persist the UUID, never a catalog index.

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
Card-specific composition and contained exceptions belong at the narrowest useful
ownership level. See the [effect-program guide](effect-programs.md).

Card declarations are oblivious to the
[prepared engine](prepared-engine.md). Do not add preparation flags, prepared
executors, optimization hints, or special constructors to a declaration or the
card schema. Declare the semantic operation in the ordinary model. The
prepared compiler may recognize that structure and lower it independently;
unsupported structures continue through the reference implementation.

## Extension boundaries

Use the smallest boundary that truthfully implements the behavior:

- A genuine primitive belongs in the core, even with one consumer. A recurring
  composition belongs in shared vocabulary at the appropriate ownership level.
- Card-specific composition belongs in the card's ordered clauses or a
  justified adjacent procedure. Do not invent a general-looking engine variant
  whose fields merely describe one exceptional card's entire program.
- A bounded local exception is legitimate when the available execution
  interface can integrate it correctly. Do not introduce scattered
  card-identity branches in generic `Game` or state-machine flow.
- If no available integration boundary can implement the complete behavior,
  use `CardRules::unsupported()` and document the exact gap. Never expose a
  working subset or bypass an execution guarantee to claim support.

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

`Binding!("name")` accepts any nonempty local name without global registration.
Effect bindings belong to one resolution and its continuations; ordinary clones
have independent binding state. Runtime slots are allocated within that scope
and checkpoints store names, allowing restoration to allocate fresh slots.

Catalog validation rejects duplicate cost names within a card part and
references to undeclared cost names. These names occupy a separate namespace
from effect-output bindings; `ParentBinding` cannot name a cost.
`SourceCastWith` instead asks about a cost family such as escape. External
alternatives such as Omniscience do not acquire the card's cost bindings.

### Token declarations and creation

Use `TokenCharacteristics` to declare what a token is, `TokenDef` to select its
source, and `CreateTokenDef` to declare the creation operation:

```rust
EffectDef::CreateToken(
    CreateTokenDef::new(TokenDef::Literal(
        TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1)
            .with_abilities(&[abilities::vigilance()]),
    ))
    .with_amount(2)
    .entering_tapped(),
)
```

Characteristics builders such as `with_name`, `with_art`, and `with_abilities`
belong on the declaration. Quantity, controller, entry counters, tapped or
attacking status, and created-object continuations belong on `CreateTokenDef`.
Its constructor creates one token with ordinary entry conditions; add builders
only for the differences the clause specifies.

Use `TokenDef::Copy(&TokenCopyDef { object, exceptions })` for copiable values
read at resolution. Copy exceptions belong to that source. Copy creation
currently supports ordinary entry; the catalog rejects tapped, attacking, or
entry-counter modifiers on a copy source. Ordinary token
constructors live on `TokenCharacteristics`. Extract repeated token declarations
into constants in the set file that uses them: put a token shared across cards in
the set preamble, and one reused by a single card beside that card. Share concrete
characteristics, leaving quantity, controller, and entry modifiers at each creation
site. Keep unique ordinary token rules inline with their creating card.

Prefer matching token art from that set. If it has no matching token printing,
use its companion product or block, then an appropriate existing printing; do not
invent art metadata. Each local constant owns its art. Keep global declarations in
`card::tokens` limited to rules-defined tokens such as Food, Clue, and Treasure;
even these get set-local constants with art, including when used by only one card.
Nonstandard tokens such as Pests belong to their set files.

Token creation as a payment remains `CostDef::create_tokens`, because payment
owns its timing and atomicity. `CreateAttachedToken` retains the attachment
operation's entry sequencing. Neither needs a dummy copy source or a second
representation of token characteristics.

### Reflexive triggers

Use `EffectDef::ReflexiveTrigger` with an ordinary triggered `AbilityDef` whose
event is `TriggerEventDef::Reflexive`. Place it in the completed action's outcome
branch, such as the nonempty branch of a created-token binding or the paid branch
of a resolving payment. Reaching the operation queues one trigger; it does not
listen for future events or infer whether the preceding action succeeded.

The nested ability declares its own targets, chosen during ordinary trigger
placement after the enclosing resolution finishes. It retains the creator's
source, controller, X, and effect bindings, and can resolve after that source
leaves. Its target slots start at zero and do not inherit the creator's targets.
Express conditions about the completed action in the enclosing program; the
initial reflexive boundary accepts nonmodal abilities without listener limits
or intervening-if conditions. `InstallTrigger` remains the operation for a
listener awaiting a future event.

### Bound entry choices in mana restrictions

Wrap an entry-time creature-type choice in `ReplacementEffectDef::BindOutput`
to attach its authored label to the entering permanent. Read that value through
`ObjectPredicateDef::Subtype(SubtypeDef::Binding(binding))`. Compose it with
`HasType(CardType::Creature)` inside `ManaRestrictionDef::CastSpell` when the
mana may pay only for creature spells of that type. The subtype predicate alone
does not require the object to be a creature. Fixed subtypes use the same
predicate with `SubtypeDef::Literal("Soldier")`.

Mana restrictions evaluate the paid object in the producing ability source's
scope. Its label must match exactly; an absent or differently labeled choice
matches nothing. Floating mana retains its original object incarnation's choice
when that source leaves. Keep riders on the paid spell in `with_spend_effects`.

The restriction list is a conjunction. Use `ManaRestrictionDef::AnyOf` for
alternative permitted uses, such as casting a matching creature spell or
activating a matching creature source's ability. `ActivateAbility` accepts
activation costs, including abilities from other zones, but excludes payments
made while resolving a spell or ability. `Payment(label)` remains the separate
restriction for those named payments.

For a bounded choice between complete outputs, use
`AddManaEffectDef::choice_of_bundles`. For "two mana of different colors," use
`AddManaEffectDef::two_different_colors()`, which constructs the ten distinct
color pairs as ordinary bundle choices. Interplanar Beacon adds its spell
restriction to that shared constructor; each produced unit retains the same
restriction and can be spent separately.

### Changeling

Use `abilities::changeling()` on the card or token's ordinary ability list.
The shared subtype evaluator applies this characteristic-defining ability in
all zones before other layer-4 effects. It is part of copiable abilities, not a
printed expansion of the creature-type list. Losing abilities in layer 6 does
not undo those types; a later subtype-setting effect can replace them.

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

### Bound token declarations

For a token declaration selected on entry, author `TokenChoiceDef { label, token }`
options. Each label is opaque display text; `token` supplies the complete
`TokenCharacteristics`. Wrap `BattlefieldEntryScalarChoiceDef::tokens(options)`
in `ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(...))` and
`ReplacementEffectDef::BindOutput { binding, effect }`. The entry decision binds
the selected declaration without creating a token.

Later clauses use the bound source in the ordinary creation operation:

```rust
EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Binding(binding)))
```

Token counts, controllers,
entry modifiers, and created-token continuations compose normally. The consumer
needs no labels or option table. Ordinary inline token creation uses
`TokenDef::Literal(...)`.

Labels must be nonempty and unique. A missing binding creates no token, as with
an acquired linked ability. Bound declarations are not copiable values; a new
object chooses anew. `ChooseEffect { player, choices }` remains the separate
instruction for an immediate player choice between effects.

### Modifying trigger occurrences

Use `AppliedRuleDef::ModifyTriggers(&TriggerModificationDef { cause, permanent,
kind })` in a player-facing static ability. The recipient selects whose abilities
are affected. `cause` matches a zone-change event; `permanent: Some(predicate)`
requires an ability of a matching permanent, while `None` also reaches delayed
abilities and abilities in other zones. `kind` either suppresses the occurrence
or adds one occurrence. Suppression wins, additional occurrences add together,
and per-turn trigger limits still apply.

The event matcher chooses the entering object's characteristics after entry or
the departing object's last-known characteristics. Modifier sources are frozen
with look-back listeners, so simultaneous deaths retain the rules that applied
before the move. From-anywhere graveyard triggers instead use the post-move
battlefield. Entry replacements remain `ReplacementEffectDef` programs: counters,
tapped status, copy choices, and as-enters choices are neither suppressed nor
multiplied by trigger modifiers.

`TriggerEventDef::AbilityTriggeredBy(&cause)` observes another ability triggering.
Its `TriggeringObject` identifies that specific ability and `EventPlayer` identifies
its controller. Compose an ordinary `PayOrDef::unless` with `Counter` for a trigger
tax. The engine freezes these observers at the event and places them in the second
APNAP pass required by CR 603.3b, after the abilities that caused them to trigger.

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

Executable clauses currently use inspectable effect and action programs. Ordinary
Rust may construct those programs locally; there is no unrestricted local runtime
callback. Mechanic identities describe semantics, never hidden execution handlers.
Unsupported cards expose no executable subset.

When complete fidelity is too large for the current increment, leave the card
unsupported and state the missing shared capability in its audit comment. A
reusable primitive may land independently, but the card becomes executable
only when its complete printed behavior is declarative.

## Implementation workflow

1. Confirm the printed clauses and the format or card interaction being added.
2. Represent the clauses, categories, costs, targets, and effects in
   the card definition.
3. Compose existing actions, add genuine primitives, or use a contained local
   procedure through a supported execution boundary. Otherwise retain a
   whole-card unsupported declaration.
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

## Pool-dependent mana amounts

Use `AddManaEffectDef::one(color).with_variable_amount(value)` to add a computed
quantity of one mana type. `ValueDef::ManaInPool { player, color }` counts unspent
mana, including restricted mana; `color: None` counts all types. Constants,
sums, and scaling compose with this query in activated mana abilities.

Use `AddManaEffectDef::amounts(&[(color, value), ...])` when one activation
produces several types with independent quantities. Every entry reads the same
pool after activation costs, before any entry produces mana. Doubling Cube
composes six entries, each counting and adding one matching type. Production
creates new mana carrying the producing effect's source and payload; the count
query does not copy restrictions, spend effects, or source properties.
