# Changelog

Two numbers matter to a bot, and they move independently:

- **Protocol version** (`penta.protocol_version()`, `penta_protocol_version()`)
  covers consumer-facing shapes and the action space they describe. It bumps
  when a client written against the old number could misread the new output.
- **Engine version** (`penta.engine_version()`, the crate version) covers
  rules behavior. It bumps for anything that changes what a policy sees,
  including rules fixes that leave the shapes alone.

Pin both alongside trained weights. Until 1.0 the engine version bumps its
minor for breaking changes, per Cargo's 0.x convention.

## Unreleased — protocol 17

The current development checkout reports engine 0.6.0 and protocol 17. Pin
both; the engine version alone does not distinguish it from earlier 0.6.0
snapshots.

### Changed

- `ComparisonDef` now names the five ordering relations directly: `Less`,
  `LessOrEqual`, `Equal`, `GreaterOrEqual`, and `Greater`. Rust card definitions
  should migrate `AtMost` to `LessOrEqual`, `Exactly` to `Equal`, and `AtLeast`
  to `GreaterOrEqual`. This definition-only API change does not alter protocol
  JSON or rules behavior.
- The public Rust type `LibraryPlacement` is now `ZonePlacement`. Downstream
  Rust callers must update their imports and constructors; the wire protocol
  is unchanged.
- Mana Vault now uses shared declarative constructs for all four abilities:
  an effective static untap restriction, an optional upkeep mana payment, an
  intervening-if draw-step trigger, and its existing mana ability. This also
  corrects two rules edges: the upkeep payment is offered even while the Vault
  is untapped, and the draw trigger checks tapped status both when it triggers
  and on resolution, using last-known information if the Vault has left the
  battlefield. The upkeep choice now uses the shared optional-payment prompt
  and labels. It can now appear while the Vault is untapped, adding a
  supported-format decision state and introducing protocol 17; the strings
  remain presentation text rather than stable identifiers. The retired
  `CardBehavior::ManaVault`, `CardBehavior::ManaVaultUntap`, and
  `CardBehavior::ManaVaultDamage` Rust selectors have also been removed.
- Wheel of Fortune and Timetwister now resolve through shared declarative
  zone-move, shuffle, draw, and recipient-chosen discard effects rather than
  named card handlers. Their retired `CardBehavior::WheelOfFortune` and
  `CardBehavior::Timetwister` Rust selectors have been removed. Multi-player
  draws run active player first, and an
  attempted draw from an empty library remains pending until the next
  state-based action check, so resolution finishes and simultaneous
  empty-library or life-total losses settle together. The existing legal
  actions and protocol JSON shapes are unchanged, so this needs no further
  bump beyond protocol 17.
- Indestructible now stops destroy effects, including those that disallow
  regeneration, and destruction from lethal or deathtouch damage. Sacrifice,
  zero toughness, the legend rule, and other non-destroy graveyard moves remain
  unaffected. Boros Charm now executes all three printed modes: its untargeted
  mode grants Indestructible to each permanent its caster controls as it
  resolves, and its damage mode can target either a player or a planeswalker.
  Those newly offered supported-format actions introduce protocol 16. The
  unfiltered catalog also adds the off-format Darksteel Ingot test definition
  (`263`, debut set `darksteel`) as a compatible append-only entry.
- `DeclareAttacker` now carries a `defender`, naming the player or the
  planeswalker the creature is attacking. A bot that emitted the action
  without one must add it; every legal action the engine offers already
  does. Combat damage follows the defender, so an attacker can now reduce a
  planeswalker's loyalty. This change introduced protocol 15.
- Every battlefield permanent that is a planeswalker reports `loyalty` and
  `loyaltyAbilityUsedThisTurn`, and observations gained an `emblems` array
  for the command zone. Decision options gained a `members` array, which is
  empty except for the grouped piles a partition decision offers.
- An attacker with trample and exactly one blocker is now asked how to
  divide its damage, where before the engine assigned lethal to the blocker
  and spilled the rest automatically. Both remain legal (CR 510.1c); the
  choice is simply offered rather than made for the player.
- Continuous effects can now add or remove abilities with permanent or
  turn-bounded durations, and static ability changes are evaluated in
  timestamp order. Land-type setters separately implement the CR 305.7
  removal of rules-text and copiable abilities, so Blood Moon is declarative
  and its catalog coverage advances from `partial` to `complete` without
  suppressing the Mountain mana ability or independently granted abilities.
  This is a focused layer slice: static-source dependencies within the ability
  layer still await guarded fixed-point evaluation.
- Doom Blade, Swords to Plowshares, Divine Offering, Dispel, Dissipate,
  Putrefy, Ultimate Price, and Warleader's Helix now use shared declarative
  target and effect definitions instead of named custom spell dispatch. Their
  existing play options consequently expose one derived target slot in catalog
  JSON where the legacy definitions exposed none. The existing target-slot
  shape and cast-action encoding make this a compatible protocol-15 catalog
  enrichment. As a rules correction, casting now reads effective
  characteristics consistently and resolution rechecks the declared target
  predicate, including protection and hexproof, so an all-illegal spell
  correctly fizzles instead of applying a card-local partial effect.
- `CardBehavior` no longer exposes the 41 retired selectors whose built-in
  cards are declarative. The Rust enum now contains live custom-effect
  selectors plus the three documented `CardDefinition::new` compatibility
  keys; downstream Rust code naming a removed variant must use the card's
  declarative rules or catalog definition instead. This source-API cleanup
  does not change protocol JSON or legal actions.
- Nevinyrral's Disk now uses the shared activated-ability costs and a
  declarative `Destroy` effect over matching battlefield permanents instead of
  its card-specific activation and resolution paths. The retired
  `CardBehavior::NevinyrralsDisk` Rust selector has been removed, and the
  handcrafted policy scores the full sweep from the board swing. Protocol JSON
  and legal actions are unchanged.
- `EffectDef::DiscardCards` and `EffectDef::DiscardAtRandom` are now one
  `EffectDef::Discard` operation whose `DiscardSelectionDef` attribute is
  `RecipientChooses` or `Random`. Downstream Rust card definitions must migrate
  to the unified shape. The chosen and seeded-random resolution paths are
  unchanged, as are protocol JSON and legal actions.
- The unfiltered catalog appends `Urborg, Tomb of Yawgmoth` as definition 261
  with debut set `planar-chaos`, and `Yavimaya, Cradle of Growth` as definition
  262 with debut set `modern-horizons-2`. They are cross-format interaction
  fixtures and report `allowed: false` and `legal: false` in both supported
  formats. Existing definition IDs, JSON shapes, and supported-format legal
  actions are unchanged, so this is a compatible protocol-15 expansion rather
  than a protocol-version boundary.

- Every catalog `manaCost` object now reports its nonzero two-color hybrid
  symbols as `"hybrid": [{"symbol": "R/W", "count": 3}]`. This replaces the
  protocol-7 `whiteRedHybrid` integer and applies consistently to cards, card
  parts, play options, alternative costs, and additional costs. Clients should
  render each reported symbol `count` times and must not assume a fixed set of
  hybrid pairs. This change introduced protocol 8.
- Every serialized `targetSelections` entry now has an `amounts` array. It is
  empty for ordinary targets and parallel to `targets` for a divided effect,
  where each value is the share assigned to the target in the same position.
  This applies to cast choices, activated abilities, and stack signatures.
  Clients that compare or featurize actions must include the array because
  legal actions can otherwise differ only by their division. This change
  introduced protocol 9.
- `ActivateAbility.costObject` replaces the nullable `sacrifice` field. The
  value still identifies an object selected while paying a cost, but now also
  covers non-sacrifice costs such as exiling a graveyard card. Clients that
  compare actions must include it because otherwise identical activations can
  differ only by the payment object. This change introduced protocol 10.
- Instantiated spell and ability target slots now use consecutive zero-based
  positional IDs. A cast flattens base-option targets followed by each
  selected mode occurrence, giving repeated modes distinct target ranges.
  Clients must use the concrete action's `choices.targetSelections` or the
  stack signature rather than assuming a mode-local catalog slot ID remains
  its runtime ID.
  This change introduced protocol 11.
- A completed observation's `result.reason` can now be
  `OpponentLostToAnEffect` when an effect makes a player lose without changing
  their life total or making them draw from an empty library. Clients that
  treat result reasons as a closed enum must accept the new value. This change
  introduced protocol 12.
- `PermanentObservation` now carries a permanent's effective card types, and
  the browser derives its kind and type line from those current types rather
  than from printed rules. Animated lands therefore remain lands while also
  presenting as creatures. The canonical bot JSON did not add a `types` field.
  This change introduced protocol 13.
- `GameEvent::ErhnamForestwalkGranted` has been removed now that Erhnam Djinn's
  ability uses the ordinary stack and keyword machinery. Rust event-log
  consumers must stop matching that bespoke variant and use ordinary ability
  events or current state. Bot JSON shapes are otherwise unchanged from
  protocol 13. Catalog play options can also report the new
  `beforeCombatDamage` restriction used by Berserk. These changes belong to
  the protocol-14 development boundary.

A client migrating from the protocol-7 compatibility boundary should review
all nine changes above and apply those affecting the surfaces it consumes.

## 0.6.0 — protocol 7

### Changed

- Activated abilities can cost X. `ActivateAbility` carries the chosen value
  and `legal_actions` offers one activation per affordable X, so a bot that
  assumed a single activation per ability and target now sees several.
- Flashback and Overload are alternative-casting ability clauses. Their costs
  are exposed in a play option's `alternativeCosts`; selecting Flashback lets
  a card in its owner's graveyard produce a `CastSpell` action and exiles that
  spell when it leaves the stack. A bot that assumed every castable card was
  in hand, or that every spell used its ordinary cost and targets, needs
  updating.
- First strike and double strike deal combat damage in separate waves with a
  priority window between them. Observations expose that window as
  `regularCombatDamagePending`, and newly executable strike and Bloodrush
  abilities add legal actions that older bots did not see.
- Activated, mana, and triggered actions identify the exact printed,
  intrinsic, or granted ability that created them. Triggered abilities become
  independent stack objects with frozen source information and may be answered
  before they resolve; mana abilities remain immediate.
- Trigger placement now follows active-player/nonactive-player order, with
  each player explicitly ordering and targeting their own simultaneous
  triggers before priority returns. This intentionally changes replay and
  policy outcomes for lines such as answering Ankh of Mishra or City of Brass
  damage before it resolves.
- Card rules text and implementation coverage now belong to ordered ability
  clauses. Card-level `Complete`, `Partial`, and `MetadataOnly` status is
  derived from those clauses, exposed as `implementationStatus`, and used by
  the browser's coverage messaging instead of the internal execution gate.
- Common keyword and fixed-mana clauses come from the reusable
  `card::abilities` library. Printed lands with basic land types keep explicit,
  executable mana clauses but are marked partial until those abilities are
  derived intrinsically from the types; Blood Moon's synthesized Mountain
  ability remains intrinsic. Each produced mana value retains its restrictions
  and spell/ability riders.
- Bespoke engine dispatch is now an optional `CardRules` hook. Declarative and
  metadata-only cards no longer require a `CardBehavior` identity.
- Catalog and browser hand JSON now serialize cards with no mana cost as
  `"manaCost": null`; a printed `{0}` remains a mana-cost object whose
  `generic` value is zero.

All incompatible wire changes above ship together as protocol 7. A protocol
number identifies the compatibility boundary for a release, branch, or pull
request; it does not increment once per field or intermediate commit.

## 0.5.0 — protocol 2

### Added

- `Game` can be used as a simulation substrate, not only driven as a match.
  `hand` and `library` read a zone unredacted; `set_hand` and `set_library`
  say what a zone holds, by card definition. The Python module exposes the
  same surface. `observe` is unchanged and remains the redacted view anything
  client-facing should use — a game running in your own process has nobody to
  hide from.

  This is what determinized search needs. You do not know an opponent's last
  card, so you build the worlds you think are plausible and roll each out.
  Cards are built fresh rather than moved, and nothing is conserved: a
  hypothetical world has no reason to balance, and the engine ships no sampler
  because naming the cards is the whole API.

Protocol stays at 2. No JSON shape changed and no action was added or removed;
the new methods sit beside the protocol rather than in it.

## 0.4.0 — protocol 2

### Fixed

- A library search may now fail to find. Searching a hidden zone never obliges
  the searcher to find anything (CR 701.19c), but Demonic Tutor demanded
  exactly one card, so a player holding a full library was forced to take one.
  Failing to find is distinct from cancelling: the spell resolved and the
  search happened, so the library is still shuffled — otherwise a player could
  tutor, decline, and read their own deck order off the top.
- A decision never asks for more cards than it offers. An empty library made
  Demonic Tutor demand one of zero options with no way to cancel, which left
  no legal action at all and deadlocked the game for every policy.

### Changed

- The bundled handcrafted policy takes as many options as a beneficial
  decision allows rather than the bare minimum, so it still finds a card when
  a search permits declining.

Protocol stays at 2: no JSON field was added, removed, or renamed. A bot that
reads a decision's `minimum` needs no change, but one that assumed a search
always yields a card will now see games where it does not.

## 0.3.0 — protocol 2

### Changed

- Games now select an explicit format. Existing constructors and catalog/deck
  helpers still default to Eternal Central Old School 93/94, while new
  format-aware entry points also expose ISD–RTR Standard.
- Runtime IDs now identify one game object in its current zone rather than a
  physical card for the whole game. A card in hand, the spell it becomes on
  the stack, and the permanent it becomes on the battlefield therefore have
  different IDs.
- `PlayLand` actions carry a play-option ID, and `CastSpell` actions carry
  structured play-option, mode, cost, X, and target-slot choices. Stack
  observations retain the resulting cast signature for spell-copy effects.
- Catalog and observation JSON expose structured card parts and the currently
  presented permanent part. These wire-shape and legal-action changes require
  protocol 2; clients should continue selecting actions by their `type` and
  other semantic fields rather than hardcoded indices.

### Added

- The final pre-Theros ISD–RTR Standard format profile and the eight decks from
  the September 2013 SCG Open Atlanta Top 8.
- Optional `format` arguments in the Python binding and protocol config JSON,
  plus format-aware catalog and deck-list helpers in the Python and C APIs.

## 0.2.0 — protocol 1

### Changed

- **Conceding is no longer a bot action.** It appeared in `legalActions` in
  every state, always at index 0, and is strictly dominated for a bot —
  resigning only loses a game that playing on might win. A bot that picked
  blindly or explored uniformly resigned on turn one, which made the
  `random` baseline meaningless to measure against. It is gone from the
  bot's list entirely, so **every index in `legalActions` shifts down by
  one**; a bot that hardcoded indices needs revisiting, one that reads the
  `type` tags does not. Humans still concede in the browser client, which
  reads the engine's own action list.

### Added

- Local matches between the built-in policies via `penta-match`.
- CI on every push and pull request, running the same two scripts as local
  development.
- `rust-toolchain.toml` pins the Rust version, components, and wasm target,
  so contributors, maintainers, and CI share one compiler.

## 0.1.0 — protocol 0

First release of the bot-facing surfaces: the `penta::protocol` module and
its canonical JSON, the Python bindings, the C ABI, self-play through an
external opponent, and the [bot guide](docs/bots.md).
