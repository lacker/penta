# Changelog

Three identifiers now have deliberately different jobs:

- **Protocol version** (`penta.protocol_version()`, `penta_protocol_version()`)
  is the breaking bot-wire epoch. It moves only when an old consumer could
  misinterpret an existing field, tag, identifier, or index.
- **Simulation fingerprint** (`penta.simulation_fingerprint()`,
  `penta_simulation_fingerprint()`) is generated from the production engine
  source, resolved core dependency closure, repository deck data, and pinned
  toolchain. It is a conservative identity: equal values identify the same
  covered inputs, but non-behavioral source edits can also change it. Pin it
  with trained weights and replays.
- **Engine version** (`penta.engine_version()`, the crate version) is ordinary
  package SemVer for releases and native APIs, not an exact ruleset identity.

Observations and catalogs also advertise named additive capabilities. Replay
and reconstruction payloads carry their own format versions instead of moving
the bot-wire epoch.

## 0.7.0 — protocol 22

This release reports engine 0.7.0 and protocol 22. The simulation fingerprint
distinguishes snapshots of the covered source and build inputs.

### Added

- **Protocol 22 establishes the durable compatibility model.** JSON objects are
  open-world, so consumers ignore members they do not use. `protocolVersion`
  now moves only for incompatible interpretation changes; new cards, rules
  fixes, and different legal-action membership through existing action shapes
  change the automatic `simulationFingerprint` instead. Observations and
  catalogs advertise `protocolCapabilities`; the first optional facility is
  now `reconstruction.checkpoint.v2`. Stable wire tags are explicit mappings
  rather than Rust `Debug` output. Protocol 22 is the one-time transition from
  the former all-purpose counter to this breaking-only epoch.
- Reconstruction checkpoints now carry their own version and simulation
  fingerprint, independent of the bot-wire epoch. Format 2 replaces the old
  skipped-turn debt with an explicit ordinary-turn anchor and can reconstruct
  a prospective begin-turn replacement choice.
- Replay journals carry `replayVersion: 1` and the simulation fingerprint. Web
  replays, durable rooms, and observation reconstruction reject the exact
  artifact boundary they consume while treating `engineVersion` as package
  provenance. Existing engine/package and protocol metadata remain present for
  diagnostics and compatibility.
- Python adds `penta.simulation_fingerprint()` and C adds
  `penta_simulation_fingerprint()`. Both return the same SHA-256 identity
  advertised in protocol JSON and exported to the WASM host.
- Hosted bots declare `{protocolVersion, capabilities, requiredCapabilities}`
  at registration and heartbeat. The registry compares both required subsets
  before listing or assigning a bot, advertises the server fingerprint in its
  manifest, honors an optional bot `requiredSimulationFingerprint`, and returns
  `409 incompatible_bot` for a mismatch. Registrations without a declaration
  remain protocol-21 clients and are refused by protocol 22 until they opt into
  the open-world contract explicitly.

- **Protocol 21.** Game reconstruction now has one typed `GameSnapshot` serde
  schema behind `checkpoint`. Encoding and decoding share that schema, replacing
  the parallel hand-written JSON constructors and field parsers. The snapshot
  carries every ordinary hosted action-boundary continuation: pending decisions
  and entry events, delayed/floating/pending triggers, restricted and
  source-specific mana, retired-object last-known information, combat
  assignments, dynamic/copy characteristics, temporary abilities, and stack
  copies or runtime modifications. Catalog executable data is addressed by
  semantic locators rather than serialized code or mutating `set_*` calls.
  Construction verifies both the legal-action list and every engine-owned
  public observation field; malformed, inconsistent, or unlocatable state
  continues to fail explicitly.

- **Protocol 19.** Every observation now includes a hidden-safe `checkpoint`
  object with turn counters, combat progression, once-per-turn flags, delayed
  turn changes, per-permanent raw counters, and the other rules bookkeeping
  that cannot be recovered from display labels. It contains neither the host
  seed nor RNG state and does not reveal either library or the opposing hand.
  `Game.from_observation(observation, hidden, rollout_seed)` in Python,
  `BotGame::from_observation_json` in Rust, and `penta_from_observation` in C
  build a live local determinization while preserving public object IDs and
  minting fresh IDs for hypothesized hidden cards. The constructor validates
  protocol, checkpoint, and simulation versions, hidden-zone sizes, and the
  rebuilt legal-action list instead of accepting an approximate world.
  Activated and triggered
  stack objects now carry catalog-relative semantic ability locators, complete
  target selections, and captured trigger context, so their response windows
  reconstruct too when resolution does not require retired-object
  last-known information. Data-only pending decisions over preserved public
  objects or the viewer's own hand also reconstruct with their exact options,
  bounds, visibility, and policy preference; unsupported continuations still
  fail closed. Command-zone emblems preserve their public object ID,
  controller, catalog definition, timestamp, and creating ability provenance.

- The catalog appends definitions 315 through 605: 286 Eternal Central Old
  School 93/94 card identities and five supporting tokens. With Guardian Beast
  below, the Old School pool now exposes 421 legal identities: 389 complete,
  30 partial, and two
  metadata-only. An identity-complete audit, kept inline at each identity's
  collector position in the printed set modules, names the concrete engine gap
  for those 32 cataloged incomplete cards and the other 560 legal identities
  that remain blocked, as well as all seven banned identities in those sets.
  Definition IDs remain append-only and the catalog JSON shape is unchanged,
  so this is compatible protocol-19 catalog growth.

- The catalog appends definitions 607 through 1361: 736 card identities used
  by ISD–RTR Standard and nineteen supporting tokens. Together with in-format
  printings of existing definitions, Standard now exposes 878 legal identities:
  839 complete and 39 partial. Its identity-complete audit
  covers all 1,686 identities from Innistrad through Magic 2014 and keeps a
  concrete capability gap inline at the collector position of every one of the
  847 incomplete identities, including the 808 that remain blocked. Definition
  IDs remain append-only and no catalog, observation, action, or decision JSON
  shape changes, so this is compatible protocol-20 catalog growth and does not
  bump the protocol.

- The unfiltered catalog appends the off-format Premodern cards `Impulse`,
  `Sleight of Hand`, and `Opt` as definitions 310 through 312, and recognizes
  the `visions` debut-set slug. Their shared private top-of-library selection
  procedure supports moving the chosen and unchosen groups independently and
  resumes follow-up effects only after the choice, which makes Opt draw after
  its scry decision. This is a compatible protocol-18 catalog growth: the
  definitions are not legal in either currently shipped format.
- `Enlightened Tutor` and `Worldly Tutor` join the unfiltered catalog as
  append-only definitions 313 and 314. The shared library search can now
  reveal a selected card, shuffle the rest, and put the selection on top;
  both tutors remain off-format in the currently shipped profiles.
- `Ugin's Nexus` joins the unfiltered catalog as append-only definition 1368,
  with the `khans-of-tarkir` debut-set slug. It remains off-format in the
  currently shipped profiles while exercising shared extra-turn and zone-move
  replacement effects.
- Hosted rooms are no longer open to whoever knows their id. Starting a room
  mints a token per seat and returns both; every route then requires the
  token for the seat it speaks for, so a room id names a room without
  authorising anything. `POST /_bots/<id>/challenge` must present the room's
  bot-seat token, which the registry verifies with the room itself -- without
  that, anyone could park every listed bot in rooms of their own. Invitations
  carry the token on to the bot. `lose-on-time` is no longer routable from
  outside at all: only a room's own alarm and the registry reach it.
- Limits for a public deployment: ten creations a minute per address across
  starting a room, registering, and challenging; registrations deleted after
  a day unused; finished rooms released an hour after they end; at most 200
  registered bots; bot names cut to 40 characters.
- A bot registry, so a bot can be online and other people can play it. `POST
  /_bots/register` returns an id and token; `POST /_bots/<id>/heartbeat`
  renews presence and returns the games the bot has been invited to; `GET
  /_bots` lists who is online. Presence is a lease -- heartbeat at least every
  15 seconds, miss 45 and you drop off -- so a crashed bot leaves the list on
  its own. A bot plays one game at a time and frees itself by reporting a
  finished room in `done`.
- `GET /_game/<room>/opponent` reports whether the external seat holds the
  decision and hands back its observation, so a remote bot can play a hosted
  game with two ordinary HTTP requests instead of a WebSocket. The socket path
  is unchanged and remains the low-latency option.
- **Protocol 18.** `result.reason` gained `OpponentRanOutOfTime`, reported
  when a seat lost to a host's clock instead of conceding. A client that
  switches exhaustively on the reason must handle it. `Game::lose_on_time` is
  the engine entry point -- deliberately not an `Action`, because a clock is
  imposed rather than played, and it does not require the losing seat to hold
  priority.
- A move clock in every hosted room, enforced by a Durable Object alarm so a
  timeout lands whether or not anyone is connected. The seat to act gets 60
  seconds if it is a bot and five minutes if it is a person, restarted by each
  applied command. Running out ends the game through
  `WebGame.loseOnTime(seat)`. A live room's state payload carries `moveClock`
  with the deadline, and the web client counts down the last minute of your
  own.
- A bot that stops heartbeating loses any game it is in, without waiting for
  the clock: the registry notices its lease has lapsed and tells the room.
  `POST /_game/<room>/lose-on-time {seat, reason}` is that instruction.
- The web client's opponent picker lists bots that are online now, and
  challenging one deals a hosted game against it. `examples/python/hosted_bot.py`
  is a complete bot on this surface: register, heartbeat, play.

These are additive routes on the development-flagged (`HOSTED_GAMES`) server
surface; no observation, action, or decision shape changed, so the protocol
version is unmoved.

### Changed

- Chaos Orb now uses shared declarative effects and Eternal Central's 93/94
  non-targeting timing. Its controller activates it without a target, chooses a
  nontoken permanent during resolution, and then gets one seeded trial with a
  `0.9` likelihood to destroy that permanent before the Orb attempts to destroy
  itself.
  Hexproof, shroud, protection, and target-fizzle rules do not constrain this
  choice.
  The change from per-permanent activation actions to a resolution-time
  decision introduces protocol 20.
- Added Guardian Beast (definition `606`) to the Old School pool. While
  untapped, it declaratively prevents new Auras on its controller's noncreature
  artifacts, grants them indestructible, and prevents opponents gaining control
  of them; already-attached Auras remain. The card exposes the intended Chaos
  Orb interaction without a card-specific resolver.
- `EffectDef` now supports floating-point `Randomized` branches driven by the
  replay-stable seeded RNG and a reusable resolution-time `ChoosePermanent`
  continuation. `ChosenPermanent` is deliberately distinct from a target and
  never passes through target legality or fizzle machinery.

- Extra turns are now a shared declarative effect used by Time Walk, Time
  Vault, and Ugin's Nexus. The scheduler keeps ordinary turns anchored
  separately from its newest-first extra-turn queue, including across
  checkpoint reconstruction. Time Vault's four clauses are declarative: its
  optional replacement is offered before the prospective turn begins and is
  composed from the generic operations to replace an event with nothing and
  perform an ordinary untap effect. Under CR 614.10b that untap is deferred
  until it is the first action of the next turn that actually begins. Ugin's
  Nexus uses the same vocabulary to skip extra turns, and its
  battlefield-to-graveyard replacement competes correctly with Rest in
  Peace before exiling the Nexus and scheduling its controller's extra turn.
  These rules and append-only catalog changes use existing bot-wire vocabulary,
  so protocol remains 22 and the automatic simulation fingerprint identifies
  the new behavior. Checkpoint format 2 replaces `skippedTurns` with
  `nextRegularPlayer` and reconstructs pending begin-turn choices. A checkpoint
  taken during a battlefield-exit replacement-order choice still reports
  deferred state and reconstruction fails closed until that suspended batch
  and its completion have a stable typed encoding.
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
- Library and other card-zone searches now use one declarative procedure with
  explicit selection bounds, reveal behavior, destination placement, and
  shuffle semantics. Demonic Tutor is no longer custom and correctly requires
  a card when a nonempty library can supply one; qualified searches may still
  fail to find. The catalog adds Ring of Ma'rûf (`1362`, Arabian Nights) and
  the remaining Onslaught fetch lands: Bloodstained Mire (`1363`), Polluted
  Delta (`1364`), and Windswept Heath (`1365`). Alongside the already-cataloged
  Enlightened Tutor (`313`), Flooded Strand (`283`), and Wooded Foothills
  (`284`), all five fetch lands now have complete shared abilities. Liliana's
  Shade (`1366`) and Seek the Horizon (`1367`) also move from the ISD–RTR audit
  into the executable catalog using the same search procedure. Standard now
  exposes 880 legal identities: 841 complete, 39 partial, and 806 blocked; the
  inline audit covers the remaining 845 incomplete identities. Ring retains
  sideboards as private outside-game cards, replaces the next draw, and follows
  Eternal Central's exile-or-sideboard wording in Old School while using its
  Oracle outside-game-only wording elsewhere. Its supported-format activation,
  the new `OutsideGame` decision-option provenance value, and Demonic Tutor's
  mandatory choice bounds are compatible protocol-22 simulation growth. They
  change the generated simulation fingerprint rather than the bot-wire epoch.
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
- `CardBehavior` no longer exposes the 43 retired selectors whose built-in
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
- `EffectDef::OptionalManaPayment` is now `EffectDef::OptionalPayment`, using
  the same `PaymentDef` vocabulary as replacement effects. Rust card definitions
  should express a mana payment as one `CostDef::Mana` atom and name its payer;
  protocol JSON and rules behavior are unchanged.
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
