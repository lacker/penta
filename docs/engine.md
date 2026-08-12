# Engine architecture

This document describes the engine's current runtime abstractions and
invariants. See the [design doctrine](design-doctrine.md) for project
philosophy, [implementing cards](implementing-cards.md) for extension guidance,
[engine interfaces](interfaces.md) for consumer APIs, and
[formats and scope](formats.md) for current coverage.

## Identities and zones

A `CardDefinitionId` identifies one canonical card name and rules identity in
the catalog. Copy limits, banned and restricted lists, and executable behavior
all use that canonical identity. A `CardPrintingId` identifies one exact
set-and-variant printing of the definition. Multiple printing variants may
therefore share a set and canonical definition, which can represent different
basic-land art without duplicating gameplay rules.

The runtime model deliberately separates physical-card lineage from rules
object identity:

- A physical-card ID follows one piece of cardboard and its owner through the
  game. A printing can eventually be attached here without changing the card's
  canonical definition.
- A `GameObjectId` identifies one rules object in its current zone. Moving a
  card from hand to the stack, from the stack to the battlefield, or from the
  battlefield to exile creates a new game object. Effects and targets refer to
  this current incarnation, so an object that leaves and returns is not the
  object that was previously targeted.

Transforming a permanent and phasing it out do not change zones, so both retain
the same `GameObjectId` and permanent state. Shuffling or changing position
within a library likewise does not create a new object. These rules let the
engine distinguish, for example, a Goblin Balloon Brigade card in hand, its
creature spell on the stack, and the creature permanent it becomes without
pretending that they are the same rules object.

An object's characteristics are independent of its physical backing. The
backing is represented conceptually as zero, one, or several physical-card
IDs: a spell copy or token has no physical card, an ordinary card object has
one, and a future melded permanent can have two. A physical card may back at
most one live game object at a time. Physical lineage stays out of player
observations because exposing it would allow a client to track a known card
through a shuffle.

The core runtime uses this separation. Physical cards live in a private
game registry, live objects carry zero-or-more physical backing IDs separately
from their characteristic source, and actions, targets, observations, and
events use `GameObjectId`. The former `CardInstanceId` and `StackObjectId`
names remain deprecated source-compatibility aliases; they no longer identify
physical lineage or a separate stack-ID namespace. Printing IDs are catalog
metadata and are not an art-selection feature in the UI.

Historical events that outlive a zone object carry its immutable card
definition as well as the former object ID. Activated-ability events carry the
ability object's stack ID separately from the source permanent's ID, so a
sacrificed source does not make the resolution record ambiguous.

## Card parts and contextual characteristics

A `CardPart` is an independently addressable bundle of printed
characteristics. A part is intentionally broader than a physical face: Turn
and Burn are two parts printed on one face, while Huntmaster of the Fells and
Ravager of the Fells occupy opposite faces. `CardStructure` records how parts
are related for single, split, flip, double-faced, alternate-spell, and future
meld-part cards.

Which characteristics apply is a question about an object and its zone, not
about the physical card alone. The contextual resolver owns rules such as
these:

- a transforming double-faced card normally uses its front part outside the
  stack and battlefield;
- a spell on the stack uses the part or ordered combination selected by its
  play option;
- a permanent uses the part currently presented on the battlefield; and
- structure-specific rules, such as the combined characteristics of a split
  card outside the stack, are applied centrally rather than repeated by card
  effects and clients.

Visibility is separate from applicability. A player may be allowed to inspect
another face without that face contributing characteristics in the current
zone.

The catalog stores parts, structures, and play options. Ordinary cards
receive a synthesized single primary part, while Garruk Relentless, Huntmaster
of the Fells, Izzet Charm, and Turn // Burn exercise the structured metadata.
The contextual part resolver implements the zone/form/presentation selection
above. A permanent observation exposes its presented part, and baseline type,
power/toughness, flying, trample, mana production, and land-type queries read
that part. `CardDefinition.rules` remains a primary/front compatibility view
for older behavior code. Cataloging both faces does not itself define the
actions or triggers that transform an object; those remain separate runtime
behavior.

Spell resolution determines whether the result is a permanent from the locked
spell form rather than from the canonical front face. Structured target-slot
predicates are checked again at resolution; an object that remains on the
battlefield but ceases to satisfy “target creature,” for example, is no longer
a legal target.

## Priority and stack actions

Exactly one player has priority while a game is running. Concession is always
legal; other actions are generated only for the priority player.

- A non-pass action resets the consecutive-pass count.
- The first priority pass gives priority to the opponent.
- Two passes with a nonempty stack resolve its top object.
- Two passes with an empty stack advance the turn step.
- After a resolution or step change, the active player receives priority.

Activated and triggered mana abilities resolve immediately and do not use the
stack. This is an explicit ability category, not something inferred from its
effect: an ability that produces mana can still be an ordinary activated
ability and use the stack. Other supported activated abilities create stack
objects with their source, clause origin, text, targets, and resolver frozen at
activation. Removing or changing the source does not erase that independent
ability object. Definition-driven declarative and custom resolvers share this
lifecycle for supported non-mana activated and triggered abilities; custom
execution does not make one of those abilities atomic or bypass the stack.

Committed events capture matching triggered abilities from the objects that
declare them. The active player's simultaneous triggers are handled before the
nonactive player's; each player explicitly chooses the first-resolving-first
order of their own group and, when needed, places targeted triggers one at a
time with targets selected. After every pending trigger is on the stack,
priority returns to the player who was about to receive it. A trigger stack
object freezes its source object ID and event context; resolution consults the
live incarnation when it remains available or the engine's retained
last-known-information snapshot after it leaves. The source may therefore
disappear before resolution without losing required information.

Spell actions consider both floating mana and usable untapped mana sources.
Applying a spell action deterministically activates only the additional
sources needed to pay its cost, preferring colorless sources for generic costs
and avoiding excess production where possible. The read-only
`mana_sources_for_action` helper exposes that payment preview to UI clients
without cloning a complete game state. Explicit mana actions remain legal for
callers that intentionally want to float mana. Chaos Orb's non-mana activated
ability uses the stack and is identified separately from spells in
`StackObservation`; its permanent is chosen through the separate non-targeting
resolution path described below.

Attacker and blocker declaration are staged to keep legal-action generation
linear rather than enumerating exponential subsets. No player receives
priority until the declaring player submits the corresponding finish action.
When an attacker is blocked by multiple creatures, its controller explicitly
divides its damage among them. A trampling attacker can also assign damage to
the defending player once lethal damage has been assigned to every blocker.
This follows the current rules, which removed combat damage assignment order
in the [Foundations rules update][foundations-update].

Targets and choices are separate rules constructs. Targets are bound to stable
slots when a spell or ability is put on the stack, are constrained by targeting
restrictions such as hexproof, shroud, and protection, and are rechecked as the
object resolves. A declarative `ChoosePermanent` effect instead asks its named
player during resolution and makes the selected object available to its
continuation as `ChosenPermanent`; it does not create a target slot, trigger
target-fizzle rules, or re-run target legality. Chaos Orb uses that
non-targeting path.

Spell choices bind targets to stable target slots. Fireball's legacy behavior
uses one variable-cardinality slot: it enumerates affordable, distinct target
combinations, charges one additional generic mana for each target beyond the
first, and divides X evenly on resolution. Different slots are independent,
so two instructions may choose the same object. After Fork resolves, its
controller chooses legal replacement values for the existing slots or keeps
the original targets. Spell actions also carry explicit payment objects for
costs such as Goblin Grenade's sacrifice.

## Play options, modes, and cast signatures

The casting model keeps several choices separate because they obey different
rules:

- A play option selects what is being played: a split-card half, both halves
  with fuse, an Adventure spell, or one side of a modal double-faced card. It
  also says whether the action casts a spell or plays a land.
- A rules-text mode selects an effect branch, such as one of Izzet Charm's
  three instructions. A card can have one ordinary play option and several
  modes.
- Alternative and additional cost choices describe how the selected form is
  paid for. They do not become extra faces or modes.
- X, targets, and any required divisions are further choices made for the
  particular spell.

After validation, those choices form an immutable cast signature on the stack:
the selected play option and spell form, chosen modes, cost choices, X, and
target-slot assignments. Authored effects refer to clause-local target
positions; casting assigns runtime slots by flattening the selected parts and
mode occurrences in order. The resolver carries each effect's offset into that
flat list, so a modal spell retains the exact target schema that was chosen
rather than regenerating one from the canonical card definition.

Copying a spell creates a new game object with no physical backing and copies
the cast signature. It therefore retains the selected split/MDFC/alternate
form, rules modes, X, cost decisions, and targets. The copy is not cast and
does not pay those costs. A copy effect such as Fork may explicitly replace
legal target assignments, but it cannot choose different modes or a different
spell form.

The catalog types and runtime now share this model. `Action::CastSpell` carries
one authoritative `CastChoices` value rather than parallel mode/form/target/X
fields, and a validated cast stores a `CastSignature` on its stack object.
Existing single-faced cards receive a default play option and positional
target slots, so their behavior is unchanged while structured cards use their
declared options, modes, and target slots. Fork copies that signature and can
replace only the target values in its existing slots. Sacrificed, discarded,
or tapped objects remain payment records outside the signature because a copy
does not pay those costs again.

Izzet Charm and Turn // Burn exercise this structured catalog and validation
path, including ordered modes, fused forms, and independent target slots.
Ability implementation coverage remains separate from casting structure, so a
catalog can represent a form without offering an action that would resolve as
a silent no-op.

Catalog construction rejects ambiguous structured metadata: duplicate local
IDs, missing or out-of-structure parts, invalid mode and target bounds, and a
fused option that does not name every split part in printed order.

The identity model also leaves room for objects backed by multiple physical
cards. The future design is recorded separately in
[composite objects and meld](design-notes/composite-objects.md).

## Determinism and replay

All random choices use the engine-owned, versioned PRNG. A dependency upgrade
therefore cannot change the meaning of an existing seed. A replay can be
reconstructed from the engine version, format, decks, seed, and submitted
action sequence. Events provide a convenient derived trace for debugging and
UI use.

## Card model and behavior

Each built-in canonical card is declared once in the `CARDS` registry of its
representative or debut set module, under the set's release-year module. Its
`CardRecord` keeps identity and its primary `CardRules` together: name, cost,
types, creature stats, and ordered ability clauses can all be understood at
the card's declaration. Structured cards attach a `CardComposition` containing
their parts, topology, and play options; an ordinary record receives an
equivalent one-part composition automatically.

An `AbilityDef` owns one rules-text clause together with its explicit timing
category, costs, targets, structured effect, effect execution, and coverage.
The displayed card text is the clauses' text joined in printed order with
newlines, so presentation and execution do not duplicate Oracle text. Clause
IDs are assigned from that order when definitions are attached to a card part.
Effect execution is either declarative or a closed custom selector. Coverage
is independent: a declarative or custom clause can be Complete, Partial, or
MetadataOnly, with an explanation for custom complete clauses and every gap.
Card coverage is derived from all clauses and the executable land/creature
baseline rather than stored as a second card-level assertion.

A set module's `ADDITIONAL_PRINTINGS` registry points back to those canonical
records for reprints or additional variants in that set. The resulting
`CardPrintingId` combines the canonical definition, set, and variant, so
alternate art can be distinguished while sharing one runtime `CardDefinition`
and its rules. Format legality considers all known printings: a nonbasic card
is legal when at least one printing belongs to the format's allowed sets,
regardless of which printing might eventually be selected for presentation.

Many executable effects use reusable declarative primitives or constructors in
`card::abilities`. A `CardBehavior` value supplies a closed,
serialization-safe selector for custom effect execution, while declarative
effects need none. A clause keeps its selector, coverage, and explanation
together even though custom handlers remain centralized. Unsupported cards can
exist in other catalogs and hidden zones but do not generate play options that
would resolve as silent no-ops. This makes partial coverage explicit and keeps
arbitrary card code out of serialized game state.

Implementation choices and extension boundaries are documented in
[implementing cards](implementing-cards.md). Current format-specific rules and
support limitations belong in [formats and scope](formats.md).

[foundations-update]: https://magic.wizards.com/en/news/announcements/foundations-update-bulletin
