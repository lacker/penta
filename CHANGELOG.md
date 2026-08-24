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

## 0.7.0 — protocol 29

This release reports engine 0.7.0 and protocol 29. The simulation fingerprint
distinguishes snapshots of the covered source and build inputs.

### Added

- **"Choose up to one" on a triggered ability.** A modal trigger settles its
  mode as it is put onto the stack, and a minimum of zero now makes declining
  every mode an answer in its own right: the question is asked even when only
  one mode is executable, and a trigger placed with no mode goes onto the
  stack carrying nothing. Ertai Resurrected is the first card to print it.

- **Declarative simultaneous permanent choices.** A shared choice now asks
  affected players in APNAP order to choose matching permanents, binds the
  chosen and unchosen sets after every public choice is frozen, and continues
  into an ordinary nested effect. **Divine Reckoning** applies standard
  destruction to the unchosen creatures and uses the existing Flashback
  ability; **Ajani, Nacatl Avenger** now expresses its ultimate through the
  same choice followed by standard sacrifice. The new pending-decision
  checkpoint tag is additive; checkpoint format 8, replay version 2, and
  protocol 29 are unchanged.

- **Static attachment control.** `GainControl` can now execute as a live static
  ability over the attached permanent, using attachment timestamps rather than
  a fabricated enters-the-battlefield trigger. Control Magic and Steal Artifact
  therefore take their hosts without using the stack, follow a moved Aura, and
  stop applying when the Aura or its ability leaves. The source-attached marker
  is an additive checkpoint member; checkpoint format 8, replay version 2, and
  protocol 29 are unchanged.

- **Predicate-driven blocking restrictions and declaration costs.** Blocking
  now uses the same declarative shape as attacking: a rule records which side
  of the block carries it, predicates the creature on the other side, and can
  either prohibit the pairing or add a fixed mana cost paid as blockers are
  declared. Predicates read current characteristics, group effects can apply a
  restriction for a turn, and blocker-scoped costs are paid once even when one
  creature blocks several attackers. Tower of Coireall, Amrou Kithkin,
  Champion of Lambholt, Cyclops Tyrant, Seismic Stomp, Awe for the Guilds,
  Branded Brawlers, and Veteran Brawlers now use the shared runtime.

- **Protection qualities are composable object predicates.** Colors, card
  types, subtypes, spell status, color counts, controllers, and boolean
  combinations now pass through the same damage, enchanting, blocking, and
  targeting rules. Ordinary resolved ability grants can carry those qualities
  for a duration, which implements **Spare from Evil**, **Azorius First-Wing**,
  **Stonecoil Serpent**, **Beloved Chaplain**, and **Devoted Caretaker** without
  card-local behavior. **True-Name Nemesis** adds a scalar "as this enters"
  player choice and reads the chosen player through the same predicate path.
  **Emrakul, the Aeons Torn** adds shared annihilator, using the attack event's
  defending player, alongside its cast trigger, colored-spell protection, and
  graveyard shuffle. New closed keyword checkpoint tags are additive;
  checkpoint format 8, replay version 2, and protocol 29 are unchanged.

- **Land-type-conditioned static abilities.** Reusable query and condition
  constructors now ask whether a matching player controls a land with a named
  effective basic land type, so dual lands and type-changing effects count
  while extra matching lands do not multiply the result. Static traversal
  checks each condition only after its recipient and characteristic layer are
  relevant, preserving conditional control flow without unrelated layer work.
  Kird Ape and Sedge Troll have moved off their legacy characteristic and
  activation branches, and Dire Wolves, Mire Kavu, and Tek are now complete
  declarative cards using the same predicate.

- **Plot, and a `Plot` action.** The mirror of foretell's economics: a
  special action pays the plot cost to exile a card from your hand face up,
  and what it buys is a free cast on a later turn. Only the first half is new
  machinery; the second is the ordinary free permission to cast from exile,
  which is why nothing in the casting path knows the word. On the wire it is
  the additive action `Plot`, naming the `card`, in the same open vocabulary
  `Foretell` and `UnlockDoor` already use, so the epoch does not move. The
  permission carries no sorcery-speed restriction of its own: every card that
  prints the keyword so far is a sorcery, and its type already says so.

- **"Whenever you cast or copy a spell."** A copy put on the stack now
  publishes an event of its own. It is not a cast -- nothing was announced
  and nothing was paid (CR 707.12) -- so it is a separate event rather than a
  second way to raise the casting one: magecraft prints both halves in one
  clause, and every other clause that watches casting means casting only.

- **Vehicles and crew.** A Vehicle is the one noncreature card type that
  prints power and toughness, which the card-coherence check previously
  refused outright. Crewing is an activated ability whose cost is tapping
  creatures rather than paying mana -- that cost already existed with no
  card using it -- and what it buys is the creature half of "artifact
  creature", until end of turn.

- **Token doubling.** "If one or more tokens would be created under your
  control, twice that many of those tokens are created instead" is a rule a
  player carries rather than an effect of its own, so it applies wherever
  tokens are made -- a printed clause, a token copy, or the Germ a living
  weapon brings. Several doublers multiply, which is what each of them says
  on its own terms.

- **Eternalize, and copy exceptions generally.** A token copy could say one
  thing after "except": that it is a 1/1. Embalm and eternalize say four --
  a body, a colour, a creature type on top of the ones it had, and no mana
  cost -- so the exceptions are now a set the clause carries rather than a
  single field, and each of them is a copiable value in its own right
  (CR 707.9a), so a later copy of the token copies them too.

  A token copy can also be made of a card rather than a permanent now, which
  is what eternalize copies: the card it exiled to pay for itself.

- **Rebound.** Two halves that only make sense together: a spell cast from
  hand exiles itself as it resolves, and a delayed trigger offers it back for
  nothing at the caster's next upkeep. Both are gated on the same question --
  whether this cast came from hand -- so the rebounded cast, which comes from
  exile, goes to the graveyard like any other spell and does not rebound
  again.

  What it needed underneath was for a lent cast to work from exile at all.
  The machinery that offers a card back was written for the clauses that buy
  a spell out of a graveyard and hard-coded that zone in four places; the
  zone is the offering clause's business, and a standing offer is itself the
  permission that reaches the card.

- **Bestow, and collect evidence.** Bestow is the first alternative cast
  that changes what the spell *is* rather than only what it costs: the clause
  carries its own enchant-creature target and its own attaching effect, so a
  bestowed spell resolves as an Aura. While the permanent stays attached it
  is an Aura enchantment and not a creature -- structural, the same way
  impending is, because it follows from how the spell was paid for. When the
  enchanted creature leaves it comes unattached and becomes a creature rather
  than dying, which is checked ahead of the Aura state-based action so that
  rule never sees an Aura to bin.

  Collect evidence is an additional cost with no card count in it: what it
  names is a total mana value the exiled cards have to reach between them.
  Payments are enumerated minimally -- a set counts only if dropping any one
  card leaves it short -- because every larger set is a strictly worse
  payment of the same cost and enumerating them all would grow with the size
  of a graveyard.

- **"Whenever one or more creatures die."** One event for the whole
  simultaneous exit rather than one per creature, published beside the
  per-object zone changes that already describe the same move: a board wipe
  gives one trigger, not one apiece. It carries only the graveyard half of
  the batch, because a permanent exiled instead of dying did not die
  (CR 700.4).

- **"Whenever one or more creatures attack you and aren't blocked."** The
  batched counterpart of the per-attacker unblocked trigger, published once
  as blockers are declared for each player the unblocked attackers were
  aimed at: two creatures through gives one trigger, not two. Attackers
  pointed at a planeswalker are left out of the batch, because a creature
  attacking a planeswalker is not attacking the player who controls it.

### Changed

- **A bot can wait for its turn instead of asking for it.** `GET
  /_game/<room>/opponent` takes an optional `wait`, in milliseconds, and holds
  the request open until the bot seat holds the decision, the game ends, or
  that long elapses; the reply is then exactly what it always was. A game asks
  the opponent seat for a decision at every priority pass, so a polling bot
  paid a full poll interval many times per turn. Measured on a development
  server, handing a decision to a bot polling at 250ms took 221ms on average
  and 496ms at the 90th percentile, against 9ms for a waiting one. The server
  caps a wait at 30 seconds, deliberately under the 45-second presence window
  so that waiting cannot read as having gone away, and an absent or
  unparseable `wait` means the immediate answer every existing bot already
  gets. This is an optional query parameter with the old behavior as its
  fallback, so protocol 28 is unchanged.

- **The room stops building the whole board to read one field.** Asking
  whether a game had finished serialized the entire human-visible state and
  parsed back a single member -- on every bot poll and every applied command,
  against a payload that grew all game. `WebGame` gained `isFinished()` and
  `resultJson()`, and the room's hot paths use them. Measured per applied
  command in a late-game position, the room's own work outside the engine is
  now about a millisecond. The command journal, long suspected here, was never
  the cost: rewriting it measured at zero.

- **A bot that is playing its game counts as present.** The registry used to
  end a hosted game the moment a bot's heartbeat lease lapsed, which caught
  bots that were sitting right there polling `opponent` and posting `botAct`
  -- including any bot copied from this repository's own examples, whose play
  loop blocked the heartbeat for the length of every game. Before declaring a
  bot gone, the registry now asks the rooms it owes games to, via the
  object-to-object `bot-activity` route; heartbeating from inside the play
  loop is still what keeps a bot listed, and `examples/python/hosted_bot.py`
  and the bot guide's loop now do it. The move clock is unchanged and remains
  the answer for a bot that is running but wedged.

- **A timeout says what actually happened.** `WebGame.loseOnTime(seat)` takes
  an optional second argument, the host's own account of the ending, and a
  reason other than the default `"ran out of time"` is what the player is
  shown -- so a bot whose process vanished reads as "Fizzbot stopped
  answering" rather than as one that merely thought too long. The reason was
  already journaled and already replayed; only the live path dropped it.
  `result.reason` is still `OpponentRanOutOfTime`, so protocol 28 and replay
  version 2 are unchanged.

- **Formats are organized by legality model and category.** Set-based formats
  now carry set windows plus banned/restricted policy, while cubes carry fixed
  card lists. Historical Standard adds `isd-m14-standard` and
  `som-m13-standard`; cubes add `pauper-cube` from the dated 450-card Cube
  Cobra snapshot. The redundant `isd-dgm-standard` profile and its old
  `isd-rtr-standard` input aliases are removed; use `isd-m14-standard` for the
  final pre-Theros window and its ten built-in decks. Display labels use
  `Standard: ...` and `Cube: ...` prefixes.
  The catalog gains metadata-only audited stubs for every newly introduced
  identity, and `make catalog-report` derives every section from the category
  registry. Removing the closed format value advances protocol 28 to 29.
  Checkpoint format 8 and replay version 2 are unchanged; replay compatibility
  still requires the new simulation fingerprint.

- **Protocol 28 broadens flexible mana and adds announced payment choices.**
  Catalog `manaCost.hybrid[].symbol` values can now describe two-brid (`2/B`),
  Phyrexian (`R/P`), Phyrexian hybrid (`G/U/P`), and colorless hybrid (`C/W`)
  symbols as well as ordinary two-color hybrid. Cast actions add the optional
  `choices.manaPayment` array when one or more symbols use an explicitly
  announced life or generic alternative; mana-paid copies are omitted. Treat
  symbol strings as open display values. Checkpoint format 7 and replay
  version 2 are unchanged.

- **Island Sanctuary, Sylvan Library, Elephant Grass, Moat, Caller of the Pack,
  and Ensnaring Bridge now use shared declarative rules, and checkpoint
  reconstruction moves to format 8.** Optional static draw replacements can
  replace each applicable draw with an ordinary effect; attack restrictions
  carry an attacker predicate, an explicit defender scope, and an optional
  additive declaration cost; drawn-this-turn cards still
  in hand form a reusable object set; and a generic resumable
  `ForEachInBinding` effect applies a continuation to every chosen object, with
  ordered object-set bindings preserving a player's resolution order. The
  former Sylvan Library and Moat behavior keys and their card-specific paths are
  removed. Cumulative upkeep uses appended age counters, and Myriad is an
  ordinary attack trigger whose exact two-player result is no token. Format 8
  records optional versus installed draw replacements, replacement-effect
  continuations, object-set iteration, and resolved attack restrictions, and
  freezes the stack kind inside the checkpoint as well as the public
  observation. It also counts each player's land plays rather than flagging
  them, which is what makes a second land drop expressible at all.
  Reconstruction consumers must require `reconstruction.checkpoint.v8` and
  regenerate format-7 checkpoints. Protocol 28 and replay version 2 are
  unchanged.

- **Card definition IDs now use printing-anchored, JavaScript-safe 52-bit
  identities.** Every existing card keeps its historic numeric value, now
  authored beside its `CardRecord`; new cards deterministically hash an
  immutable exact-printing UUID with a committed collision nonce when needed.
  The former hand-maintained flat ID registry is generated as a native
  compatibility surface, a compact fingerprint freezes every migrated legacy
  assignment, and the catalog uses a dense legacy index plus a sparse map for
  derived values. `CardDefinitionId` is now a constrained,
  nonzero `u64` newtype with `new`, `try_new`, and `get`; Python setters accept
  the wider integer range. Protocol and checkpoint JSON remain exact JSON
  numbers with unchanged legacy meanings, so protocol 28, checkpoint format 7,
  and replay version 2 are unchanged.

- **Protocol 27 removes the synthetic face-down card definition.** Face-down
  spells and permanents now carry rules-owned inline characteristics while the
  physical card retains its real definition identity. Shared constructors
  describe Morph, Manifest, and Illusionary Mask as ordinary nameless 2/2
  creatures; Disguise and Cloak add ward {2}; other rules and effects can
  supply their own face-down characteristics without allocating a catalog ID.
  Observers not entitled to inspect the physical card receive the new
  `faceDown` characteristics and ability-origin tags without a definition or
  card part; an entitled controller still receives the real card identity. The
  retired definition ID remains a catalog tombstone. Checkpoint format 7 and
  `reconstruction.checkpoint.v7` preserve the standardized face-down presets;
  replay version 2 is unchanged. This infrastructure does not by itself
  implement every cast, turn-face-up, or card-specific face-down procedure.

- **Protocol 26 gives tokens and emblems inline characteristics instead of fake
  card definitions.** Battlefield, stack, and decision objects now carry an
  authoritative tagged `characteristics` object. Printed presentations retain
  their catalog `definition` and `partId`; token presentations carry their
  current name, optional selected art, structure, and rules-derived display
  fields inline, and omit the legacy top-level definition projections. The
  separate permanent `token` flag remains the rules-object status, because a
  token can copy a printed card and a nontoken object can copy a token. Ability
  origins likewise add explicit `token`, `tokenGranted`, `emblem`, and
  `emblemGranted` tags without inventing source definitions. Emblem-origin
  characteristics carry their name and rules-derived presentation inline,
  without a card part or art. Face-up physical double-faced permanents add
  `physicalFace` with their physical kind and side independently of copied
  characteristics, so clients never infer transformability from effective
  token/card structure. Created-token characteristics and emblem descriptions
  are no longer entries in the global card catalog. Their former definition
  IDs remain permanently retired; the catalog is ordered by definition but
  may contain gaps, so consumers must join by the `definition` field rather
  than array index.

- **The declarative effect API now authors virtual objects as compact values.**
  `EffectDef::create_creature_token`, `create_artifact_creature_token`, and
  `create_artifact_token` take the token's subtype, color, and applicable
  power/toughness characteristics directly. Token names are derived by joining
  their subtypes unless `with_name` overrides the name; `with_amount`,
  `with_count`, `with_abilities`, and `with_art` compose the common Oracle-text
  variations without global per-token constants. Rules-defined artifact
  characteristics are reusable functions such as `tokens::treasure`, `food`,
  `clue`, `blood`, `map`, and `incubator`. `EffectDef::create_emblem` likewise
  stores a compact name-and-ability value owned by its creating effect. Neither
  value is a `CardDefinition` or participates in globally unique card naming.

- **Resolved ongoing effects can now carry ordinary activated and mana
  abilities.** `EffectDef::CreateOngoingEffect` can freeze an affected
  recipient into the nested ability's resolution context or create an unbound,
  self-contained effect for a declared duration, while giving either shape its
  own nonpermanent game-object identity. Ordinary activations use the shared
  timing, mana-payment, stack, response, and resolution machinery; mana
  activations resolve immediately and participate in casting-time payment
  planning. Guardian Angel and Channel are the first declarative cards to use
  the two shapes. Penta classifies the source as command-zone-resident for
  activation checks, even though the rules effect technically has no zone; it
  remains distinct from both permanents and emblems. Checkpoint format 7 stores
  these effects in an additive
  `ongoingEffects` member. Channel now emits the existing
  `ActivateManaAbility` shape; the legacy `PayLifeForMana` action tag remains
  in the protocol vocabulary but is no longer offered. Protocol 28, checkpoint
  format 7, and replay version 2 are unchanged.

- **Mana planning now shares zone-aware sources and direct cost
  contributions.** Activated mana abilities may be supplied by battlefield
  permanents, cards in hand, or ongoing effects, with one resource ledger for
  life and consumed objects. Convoke, Delve, and Improvise are distinct direct
  contributions that never produce mana; Delve and Improvise pay only generic
  requirements, and a tapped permanent cannot also pay through a tap-based
  contribution. Cadaverous Bloom, Elvish Spirit Guide, Treasure Cruise, and
  Foundry Assembler use these declarations. This is compatible catalog and
  legal-action growth within protocol 28; checkpoint format 7 and replay
  version 2 are unchanged.

- **Checkpoint format 5 preserves creator-owned token and emblem
  characteristics.** The reconstruction capability is now
  `reconstruction.checkpoint.v5`; format 4
  depended on synthetic token and emblem definition IDs and cannot recover the
  complete creator-owned characteristics once those definitions are gone.
  Format 5 instead stores semantic paths to card-, token-, or emblem-owned
  creating abilities and their nested effects or indexed custom-created
  virtual objects. Recursive creators remain rooted in a printed/custom card
  creator, and restore rebinds that chain under the exact simulation
  fingerprint. It retains format 4's
  generic named-subtype operation for reconstructed continuous effects. Replay
  version 2 is unchanged.

- **Checkpoint reconstruction moves to format 6 for linked Miracle state.**
  Format 6 replaces Miracle's ambient window with its private draw action,
  linked trigger, and exact one-shot cast offer. Reconstruction consumers must
  require `reconstruction.checkpoint.v6` and regenerate older checkpoints. The
  bot protocol remains 26 and replay version 2 is unchanged.

- **The native declarative API exposes the new Equipment primitives in the
  current 0.7.0 release.** `BlocksOrBecomesBlockedBy` now names both the
  subject creature and the creature on the other side of combat, and
  `MillUntil` carries explicit binding and continuation members. New public
  cost, effect, object-reference, characteristic-operation, and value variants
  cover the remaining shared mechanics. The reusable `abilities::equip`
  constructor now takes one ordered `AbilityCostDef` list, with mana represented
  by `AbilityCostDef::Mana` alongside any nonmana costs, replacing the separate
  mana-only and `equip_with_costs` helpers. Native exhaustive matches and struct
  constructors must be updated for these additions; no bot action shape changes.

- **Protocol 25 makes `ActivateAbility.costObject` an array, `costObjects`.**
  A cost can name more than one object -- "exile two cards from your
  graveyard" -- and an activation never holds priority, so there is no window
  in which to ask afterwards: every chosen object travels with the action.
  Read it as a list; an ability that spends nothing chosen has an empty array
  rather than a null. `ActivateManaAbility.costObject` is unchanged.

- **Protocol 24 makes a permanent's `blocking` a list.** It was one attacker
  id or `null`. A creature can be blocking several attackers at once: a band is
  blocked as a group, so one declaration records every member, and a card can
  grant a creature an additional block outright. Read the array; a creature
  blocking nothing has an empty one.

- **Protocol 23 canonicalizes the final pre-Theros format as ISD–DGM.**
  Catalogs and observations now emit `isd-dgm-standard` rather than
  `isd-rtr-standard`; parsers retain both former spellings as input-only
  aliases. Replay journals move to version 2 because their configuration stores
  the slug. Checkpoint format 3 and `reconstruction.checkpoint.v3` are
  unchanged.

### Fixed

- **"You may play that card" is no longer free.** The clause that exiles the
  top of a library and lets you play what it finds granted the free-play
  permission whatever the card said, so Laelia's attack trigger was handing
  out its find for nothing. Only "without paying their mana costs" waives
  the cost now; the plain permission owes it. The same clause can also exile
  face down, which is a property of the permission rather than of what it
  costs.

- **A stack ability whose source card was discarded as a cost is
  checkpointable again.** The gate that keeps a hidden source out of a
  checkpoint was refusing every source that had retired as a card -- which is
  the ordinary shape of an ability activated from a hand for a cost that
  discards it, bloodrush and everything like it. Nothing was protected by
  that: the observation publishes `stack[].sourceObjectId` whatever became of
  the card, and the checkpoint already carries the card among its retired
  objects. Such a state now reconstructs instead of failing closed.

- **A decision that offers a token no longer breaks reconstruction.** The
  importer read a `definition` off every card an option named. A token, an
  emblem, and a face-down body have no catalog definition on the wire and
  cannot have come out of a hidden zone, so there is nothing there to rebind
  and they are now skipped. A decision card in a hidden zone with no
  definition is still an error.

- **A stacked Miracle trigger reconstructs for the opponent.** It used to
  fail closed, because the card it is about is still in a hand that seat
  cannot read. The observation names that object anyway, so the checkpoint
  now says where the card sits --
  `stack[].abilityPayload.sourceOrigin`, an optional seat/zone/index -- and
  the importer binds its minted card at that position to the published id.
  This is the disclosure the public cast-or-decline decision one step later
  already made through `decisionState.cardOrigins`; the trigger stage now
  matches it. A *pending* Miracle trigger still fails closed, and the
  checkpoint format is unchanged: the member is additive and absent whenever
  a source is already public.

- **A permanent chosen for a tap cost could also be planned as the mana
  source for that activation.** Mixed mana and `TapPermanent` costs could
  therefore offer an action that tried to tap the same permanent twice. Mana
  planning now carries the chosen tap payer through enumeration, previews, and
  payment, excluding only mana abilities that would tap, sacrifice, or move
  it; counter-based mana abilities that leave it available remain legal. This
  changes which activations are legal through the existing action shape, so
  the protocol epoch does not move.

- **Miracle now uses its linked trigger and a one-shot cast-or-decline offer.**
  Every first draw now crosses the same private draw-action window; an ordinary
  card has no action and is completed internally before a UI or bot host can
  render it, while a Miracle card privately adds its real Reveal option.
  Once settled, declining that option leaves the same opponent-visible state
  as the ordinary empty path; hosted rooms retain the last safe opponent state
  while the private choice is open. Revealing creates the linked trigger, both
  players receive priority, and resolving it offers only that card's Miracle
  cost through the shared one-shot cast machinery. Definition IDs remain
  opaque catalog values; zero is not reserved. Checkpoint format 6 and
  `reconstruction.checkpoint.v6` replace the old `miracleWindow` bookkeeping;
  the bot protocol and replay epochs are unchanged.

- **A kicked spell could not target anything the unkicked one could not.**
  A kicked cast resolves its own clause, and that clause has always been able
  to declare its own target slots -- but the enumeration of legal casts and
  the validation of a submitted one both read the base spell's slots instead,
  so a kicker that widens what may be targeted offered nothing new. Both now
  read the kicked clause's slots, matching what resolution already did.
  Bloodchief's Thirst is the first card that asks: unkicked it reaches a
  two-mana creature, and kicked it reaches any of them. This changes which
  casts are legal, expressed through the existing action shape, so the epoch
  does not move.

- **A condition reading a target's characteristics never saw a spell.** It
  matched permanents on the battlefield and nothing else, so "counter target
  spell if its mana value is 2 or less" would have been false for every spell
  ever targeted. Nothing shipped depended on it; Prohibit is the first card
  that asks.

- **An ability activated from the graveyard could not name its own card.** A
  source reference always resolved to a permanent, so a graveyard-source
  clause that said "return this card to your hand" found nothing on the
  battlefield and silently did nothing. A source now answers as whatever kind
  of object it actually is; one that has left every zone still answers as the
  permanent it was, which is the last-known information a "sacrifice this"
  clause reads after the fact.

- **Evolve never compared toughness.** A predicate reading the source's
  toughness resolved to nothing at all, so only the power half of "greater
  power or toughness" could fire. A 2/2 arriving beside a 2/1 evolve creature
  now grows it, as it always should have.

- **The bundled handcrafted policy cast X spells for X=0.** With exactly the
  base cost available and nothing to pay into X, the only enumerated cast is
  the X=0 one, and the policy scored it far above passing — so it spent
  Braingeyser to draw no cards. A cast whose every effect scales with the
  chosen X now scores below passing, and the bot holds the card until it can
  pay into X. Detonate is not such a spell: only its damage scales with X, so
  an X=0 cast still destroys a zero-cost artifact and is still made.

- **"Until your next upkeep" ended before the untap step rather than at the
  upkeep.** Continuous effects expired as the turn began, which is one step
  early: the untap step comes first, so an untap prohibition written this way
  never reached the untap it was meant to stop. These effects now expire once
  the untap is done. Effects that do not care about the untap step, such as a
  granted keyword, behave as before.

- **A creature blocking two attackers dealt its damage to each of them.**
  Combat damage ran attacker by attacker and each pass dealt every blocker its
  full power, so a creature holding off two attackers hit both for its whole
  power instead of dividing it once. Blockers now deal their damage in a pass
  of their own, dividing it among what they block, and a division with more
  than one legal split is offered as a choice like an attacker's.

- **A blocker stopped being a blocking creature when its attacker left
  combat.** Blocker status was read off the list of attackers being blocked,
  and removing an attacker from combat empties that list on everything that
  blocked it — so a Sedge Troll that regenerated took its blocker out of
  combat with it. CR 506.4 lists every way a permanent leaves combat and this
  is not one of them. Blocker status now outlives those departures the way an
  attacker's blocked status already outlived its blockers, so "target
  attacking or blocking creature" still reaches such a blocker, and
  Righteousness still finds one to pump. Observations gain
  `blockingThisCombat` beside `blockedThisCombat`; `blocking` continues to
  name only the attackers still being blocked, which is what combat damage
  follows. Protocol 24 is unchanged: the member is additive, and a consumer
  that ignores it reads exactly what it read before.

- **"Blocked by this creature" went blank once the creature left.** The
  relationship is recorded on the blocker, so a blocker that died in combat
  took the answer with it and its own death trigger found nothing. It is now
  read from last-known information, like the attachment relationship beside
  it. Abu Ja'far is the card that noticed.

- **Cards put back on top of a library kept their order.** The group arrives
  top-first and the top is the back of the vector, so putting them back one at
  a time inverted them. Nothing depended on it before, because every existing
  effect either took a single card or bottomed the rest in any order.

- **Combat damage could not be divided freely among three or more blockers.**
  The assignment enumerator still enforced the former ordered-blocker rule and
  rejected a legal split such as 1, 1, and 2 damage from a 4-power attacker.
  Current CR 510.1c now permits that division, while trample still requires
  lethal damage on every blocker before any damage can spill to the defender.
- **Damage prevention leaked the proposed amount into downstream effects.**
  Drain Life gained life from damage that had been prevented, and a combat
  damage trigger could report the assigned amount instead of the amount that
  reached the player. Damage application now returns the amount actually
  dealt, suppressing zero-damage triggers and carrying partial prevention into
  both life gain and trigger context.
- **Target legality read power and toughness without continuous static
  effects.** Trigger capture and static resolution share a characteristics view
  that deliberately leaves statics out, because it is used while those effects
  are being resolved. Target legality is asked from outside that resolution and
  was using the same view, so a creature a Crusade had made 2/2 was still a
  legal target for Pendelhaven's "target 1/1 creature", and one pushed past a
  "power 2 or less" ceiling still qualified. Target legality now reads the real
  values. The keyword mask had the same seam and is now closed too, by a
  different route -- see the keyword-predicate entry under Added.
- **Berserk never destroyed the creature it pumped.** Its delayed trigger asks
  whether the creature attacked this turn, and that predicate was reading
  whether the creature is *still* attacking. End of combat clears that flag
  before the end step arrives, so the check was always false in a real game.
  The existing coverage left the creature flagged as attacking into the end
  step, which no game does, and so passed either way.

- **"Whenever one or more cards are put into exile."** One event for the
  whole move, published once however many cards it took, so a clause reading
  it gives one counter for a three-card exile rather than three. The zones
  are a list on the matcher, because "from your library and/or your
  graveyard" is one clause rather than two.

  It is raised where cards leave a hidden or public zone for exile -- the
  generic nonbattlefield move, an impulse off the top, cascade, exile-until,
  a graveyard-exiling activation cost, and a spell's additional cost. A
  permanent exiled from the battlefield is a zone change of its own and is
  published there.

- **Foretell, and a `Foretell` action.** Two halves that meet in exile: a
  special action that pays {2} to exile a card from your hand face down, and
  the alternative cast the card prints, taken from exile on a later turn. A
  foretold card is absent from the opponent's `exiles` rather than shown
  there, and counted by the additive `faceDownExileSizes` -- the same way a
  hand is a size rather than a list. Both are additive, so the epoch does not
  move; a consumer that ignores the count sees one fewer exiled card, which
  is what its player sees too.

- **Damage can name a player and their creatures at once.** "Each opponent
  and each creature they control" is one clause and one damage event; neither
  an object set nor a player set can say it alone. The damage *matcher*
  beside it already drew the same pair for the other direction.

- **A spell records which zone it was cast from**, replacing the
  cast-from-hand flag. "If you cast it from your hand" and "if this spell was
  cast from exile" are the same question asked of different zones, and one
  field answers both. Checkpoints carry it as an additive optional label.

### Added

- **"Whenever you discard one or more cards", and a permission that outlasts
  the turn.** A discard now raises one batched event beside its per-card
  ones, so both printed wordings are answerable. And a permission to play an
  exiled card can run to the holder's next end step rather than to the end
  of whoever's turn it is -- a discard on somebody else's turn buys the whole
  of yours.

- **An additional cost can be paid with life instead.** "Discard a card or
  pay 3 life" is one cost with two ways to pay it, and only one of them
  names anything -- so paying the life spends no object, and the empty
  payment is how the two are told apart afterwards. Offered only when the
  life is there, down to exactly zero (CR 118.4).

- **A land can be played out of exile.** The land walk now looks there under
  the same permission the cast walk already honoured, in both players' exiles
  -- a card somebody else exiled is still played from where it lies. Nothing
  could play an exiled land before, which quietly made every "you may play
  that card" permission a cast permission.

- **A permanent can be told which basic land type to be.** "As this land
  enters, choose a basic land type" joins the card-name and creature-type
  entry choices, and "this land is the chosen type" is a layer-4 set whose
  subject nothing could have written down. Observations carry the choice in
  the additive `chosenBasicLandType`, beside `chosenCreatureType`.

  A land that sets its own type is not thereby silenced. What CR 305.7
  silences is a land whose types somebody else replaced; an ability cannot
  be the reason it is itself ignored, or such a land would never be anything
  at all.

- **A player may be allowed additional land plays.** The turn counts each
  player's land plays instead of flagging the first, and a static ability can
  say how many more than one they get. Counted rather than merely present:
  two Explorations are two extra lands.

- **Endure.** "Endure N" is a choice between two whole effects -- N +1/+1
  counters on the object, or an N/N white Spirit token -- made by its
  controller as the ability resolves. A procedure of its own for the same
  reason explore is one: nothing else in the effect vocabulary offers a
  branch between two effects at resolution time. The token is not authored
  beside the number because the keyword fixes it, an N/N white Spirit
  whatever N turns out to be.

- **Exert.** A choice made as a creature is declared as an attacker, and
  paid for afterwards: an exerted creature is skipped by its controller's
  next untap step. The second sentence every printed exert card carries --
  "when you do" -- is a reflexive trigger, and having one is also what makes
  a creature exertable at all, the way a cycling clause is what makes a card
  cyclable. On the wire it is the additive action `ExertAttacker`, naming the
  `attacker`; the open `type` vocabulary means it does not move the epoch.

  Offered as its own action rather than folded into the declaration. Nothing
  can observe the difference: no player receives priority between declaring
  an attacker and finishing the declaration, and the trigger it captures
  waits for the declaration to finish the way every other attack trigger
  does.

- **A permanent can be put onto the battlefield carrying counters.**
  `MoveToZone` gained the `counters` its exile-returning cousin already had.
  A counter is not a continuous effect: "with a lifelink counter on it" sits
  on the permanent and outlives every duration a spell could have named,
  which is the whole reason the clause is written that way. Lifelink joins
  flying as a keyword counter.

- **Open decklists, as a mutual opt-in on hosted bot games.** A bot adds
  `discloseDeck` to its `/_bots/register` or heartbeat body, whoever starts a
  room adds `humanDiscloseDeck` to `/_game/<id>/start`, and only when both
  seats have opted in for that room does the bot's observation -- from
  `/opponent` and from the `observe` socket push alike -- carry an additive
  `opponentDeck` naming the human seat's deck. One side opting in alone
  changes nothing, and a bot that never asks sees exactly the redacted
  observation it always has. The registry tells a room which bot claimed its
  seat over a new object-to-object `disclose-bot-deck` route, so the
  declaration is the bot's own rather than something a challenger could
  assert on its behalf. This is a hosted-room convenience the deployment
  layers on top of the wire protocol, not an engine change: protocol 27 and
  the `observe()` fields the engine emits are unchanged, and a local
  `penta.Game` has neither a registry nor a room's deck configuration.

- **Rooms, and the special action that opens one.** A Room is a split
  enchantment whose halves are doors: you cast one of them, that door is
  unlocked as the permanent enters, and the other stays shut until somebody
  pays its printed cost. What the permanent is at any moment is the
  combination of its unlocked doors, so a Room's parts are the two doors, the
  pair of them, and neither of them -- a Room that arrived from anywhere but
  the stack has no name and no abilities at all.

  Unlocking is a special action: it uses no stack, cannot be responded to, and
  is available only in your own main phase with the stack empty. On the wire
  it is the additive action `UnlockDoor`, naming the `room` and the `door`
  part id, and the catalog gains the additive structure kind `room`. Both are
  open vocabularies with the fallbacks bots already use, so neither moves the
  epoch. "When you unlock this door" is handed to the door that opened rather
  than published as an event, which is why an effect that doubles enter
  triggers leaves it alone.

- **A permission to play from a graveyard can be granted for a turn.**
  "You may cast spells from your graveyard this turn" is aimed at a player and
  outlives nothing but the turn, so it is stored beside the resolved play
  prohibitions rather than derived from a permanent's printed text. A card in
  a graveyard is now castable for what it prints while something says so;
  flashback and escape are unchanged, because those are permissions to cast it
  for *their* cost and leave the printed one where it was.

- **Explore, and tokens that arrive under somebody else's control.** Explore
  is a procedure of its own for the same reason proliferate is: what happens
  to the revealed card and whether the creature grows both turn on a card type
  nobody knows until the card is revealed, and the branch that does not take
  it ends in a choice. "Its controller creates two Map tokens" is the other
  half -- creating a token now names who gets them, defaulting to the
  resolving object's controller as every clause before it meant.

- **Proliferate.** A procedure of its own rather than a composition: the
  choice runs over permanents and players at once, which no object set can
  say, and what each chosen thing gets is read off what is already on it
  rather than named by the card. Only things carrying at least one counter
  are on the menu -- another counter of each kind already there is nothing at
  all when there is no kind already there.

- **Keyword counters.** A flying counter is not a marker the card putting it
  there gives meaning to: the permanent carrying it has flying for exactly as
  long as the counter is on it (CR 122.1e), which is why it survives
  everything a duration-scoped grant would not. It is read off the counters
  the way a basic land's mana ability is read off its subtypes, and gets the
  matching origin: `intrinsicCounter` joins `intrinsicBasicLand` on the wire
  as a third kind of ability nobody printed.

- **A clause can name the tokens it just made.** Mobilize sacrifices exactly
  the tokens that attack created and no others, and by the time the delayed
  clause fires nothing about the board can tell them apart from the pair the
  last attack made. `CreateToken` now takes an optional binding and a nested
  continuation, the way every other binding clause does -- a sequence hands
  each component its own copy of the resolution context, so a binding made in
  one component would be gone by the next. Creating a token also hands back
  the identity it actually arrived with rather than the prospective one, which
  a battlefield entry replaces as it commits.

- **"Whenever you attack" is one trigger for the whole declaration.** It was
  being read as the per-attacker `Attacks` event with a size condition on it,
  so Gut, True Soul Zealot offered its sacrifice once per attacking creature
  rather than once (CR 508.1). The declaration is now its own committed event,
  published once, and `AttackDeclared` counts the attackers matching its
  predicate rather than matching any one of them. "Whenever a creature you
  control attacks alone" is unchanged: that clause really is about a creature,
  and a declaration of one has only the one to be about.

- **A cast offered during a resolution ignores the timing its type would
  impose.** An offer made while something resolves is answered then or not at
  all (CR 608.2f), which is the only way a cascaded sorcery, or one Dreadhorde
  Arcanist points at mid-combat, is ever cast: the enumeration still refused
  every sorcery outside a main phase with an empty stack, which is exactly the
  moment such an offer is made. `WithoutPayingManaCost` joins the
  alternative-cast kinds on the wire -- an open enum with a documented
  fallback, so the epoch does not move -- and says both halves of that clause
  at once: the cast is free, and the card is exiled rather than buried.

- **A copy can keep something of its own.** "You may have this creature enter
  as a copy of any creature on the battlefield, except it's an Illusion in
  addition to its other types and it has ..." -- a copy took the other
  permanent's characteristics wholesale, so an "except it has" clause had
  nothing to hand back. Both exceptions read the copying card's own printed
  line, so `CopyEntering` names them by position rather than restating them:
  the subtype line it already prints, and the abilities it already carries.
  Checkpoints gain an optional `retainPrintedSubtypes` beside each copy
  effect -- additive, so one written before this restores a copy that keeps
  nothing, which is what every copy did then.

- **A permission to play a card from exile can carry its own tax.** Elite
  Spellbinder leaves the card with its owner and charges {2} for it, and the
  charge outlives the Spellbinder: it lives on the exile permission rather
  than on any permanent. Checkpoints gain an optional `surcharge` beside each
  exile-play permission -- additive, so a checkpoint written without one
  restores with nothing owed, which is what every other permission carries.

- **Ten more ISD–DGM Equipment identities.** Demonmail Hauberk, Avacyn's
  Collar, Angelic Armaments, Moonsilver Spear, Wooden Stake, Civic Saber,
  Haunted Plate Mail, Blazing Torch, and Trepanation Blade are complete
  declarative cards. Elbrus now equips, unattaches, transforms, and presents
  Withengar's complete combat body; it remains explicitly partial only for
  Withengar's post-player-loss trigger, because Penta's supported two-player
  game has already ended at that point. The shared machinery added along the
  way covers nonmana equip costs, an exact ability-grant source, explicit
  damage attribution, live color counts, subtype removal, unattachment,
  attached-creature block events, frozen defending players, and
  reveal-until continuations. None of the ten uses custom card behavior.

- **`revealedLibraryTop` on the observation.** Null unless something lets you
  look at the top card of your own library, and a one-card list in the same
  shape as `hand` when it does. An additive optional member, so the epoch does
  not move; an old consumer that ignores it sees exactly what it saw before.

- **Energy counters.** A player resource rather than a clock: nothing checks
  how much of it anyone has, it persists between turns, and it leaves only by
  being spent. `energy` joins `poison` on the observation as a two-element
  array -- an additive member, so the epoch does not move -- and "unless you
  pay {E}" is a payment that is made in full or not at all, because a player
  short of the amount cannot pay part of it.

- **Converge.** Which colours of mana actually paid for a spell are now
  recorded on the stack object, beside the effects mana riders leave rather
  than inside the cast signature: a copy is never cast, so nothing was spent
  on it and its count is zero however the original was paid for. A spell that
  prints converge says so on its rules, which makes the generic portion of
  its cost reach first for a colour its coloured symbols have not already
  spent, one mana at a time, and for colourless last of all.

- **Modal activated abilities.** "Choose one --" was a casting shape, so only
  a spell could print it. An activated ability chooses its modes as it is
  activated (CR 601.2b) rather than as it resolves, which puts the choice on
  the action beside its targets and its X: one printed ability is offered once
  per selection, each with the modes' own target slots appended to its own,
  and each named after the clause it picked. Umezawa's Jitte is the first
  card to print one.

- **Convoke.** A spell may tap the caster's untapped creatures while its final
  cost is paid, each covering one generic symbol or one symbol of that
  creature's color. These contributions are payment rather than mana: they do
  not fire mana events or count as colors of mana spent. A creature whose mana
  ability tapped it cannot also convoke the spell, while a non-tapping mana
  ability may legally be activated before its source convokes. Sprout Swarm is
  the first covered card.

- **Buyback as an optional additional cost.** It combines with an alternative
  way of casting a spell instead of replacing that way, and its mana or object
  surcharge is included in the spell's final cost. Paying it freezes the
  successful-resolution destination as the owner's hand without changing the
  spell's instructions. Flashback still wins: a bought-back flashback spell is
  exiled wherever else it would have gone. Corpse Dance is migrated to this
  shared model; Sprout Swarm and Constant Mists add mana and sacrifice forms.

- **"Can't cause you to discard cards."** The prohibition over being made to
  sacrifice existed; the same prohibition over discarding did not. Both are
  now one shape, and a static clause may state several of them in the one
  printed sentence Tamiyo prints them in. Neither stops a player discarding
  as a cost of their own spell: nobody caused that.

  Sorting cards by a name chosen earlier in the same resolution is also new.
  The name lives in the resolution rather than on the board, so the predicate
  matches nothing wherever the resolution cannot be seen, and the one effect
  that can see it reads it there.

- **A search that spans two zones and exiles what it leaves behind.**
  Doomsday looks through your library and graveyard as one search, keeps
  five, and exiles the rest; the order the five are chosen in is the order
  they are drawn. What "the rest" means is fixed before anybody answers,
  because the spell doing the searching reaches the graveyard while the
  decision is still open and was never part of the search -- so it goes to
  the graveyard afterward like any other sorcery rather than exiling itself.

- **Infect, and a graveyard replacement that watches from anywhere.** Infect
  (CR 702.90) changes what a source's damage does rather than how much of it
  there is, so it is read after every prevention and limit has settled the
  amount: to a player it is poison counters, and to a creature it is -1/-1
  counters, which are not damage and so survive cleanup. A planeswalker
  still takes ordinary loyalty loss.

  A move-replacement can now name no source zone at all, meaning "from
  anywhere". Blightsteel Colossus comes back whether it died, was
  discarded, or was milled, and the shuffle its clause performs now runs on
  the non-battlefield paths too -- without it the card would sit on top of
  the library and simply be redrawn.

- **Counting what an earlier step bound.** "For each creature exiled this
  way" has to count what the exile actually took, and by the time the
  follow-up runs those creatures are no longer anywhere to count. A value
  can now read the size of an object set an earlier step in the same
  resolution bound. Winds of Abandon is the first card to ask, and the first
  to use overload at all.

- **Ninjutsu, and playing somebody else's cards from exile.** Ninjutsu
  (CR 702.49) is an activated ability that works from its owner's hand in
  the priority round the attack declaration opens: it returns an unblocked
  attacker and puts the ninja onto the battlefield in its place, tapped and
  attacking the same defender. It was never declared as an attacker, so
  nothing watching for a declaration fires.

  The permission a card on an adventure carries became the general shape it
  always was: who may play the card, whether the mana cost is waived, and
  when the permission lapses. An adventure is its owner's, costs what it
  prints, and never lapses; "that player exiles the top two cards of their
  library, and until end of turn you may play those cards without paying
  their mana costs" is the other player's, free, and gone with the turn.
  Additional costs still apply to a free play (CR 601.2h); only the mana
  cost is waived.

- **`{C}`, devoid, and a kicker that only costs more.** Sowing Mycospawn
  needed all three. A mana cost can now carry `{C}` symbols, which unlike
  generic can only be paid with colorless mana. Devoid is a
  characteristic-defining ability rather than a behaviour, so it is the
  card's empty printed color set and the keyword names it. And a kicker no
  longer has to replace what the spell does: one that carries no
  instructions of its own leaves the printed spell to resolve, and being
  kicked becomes a fact its other clauses read -- which is what "when you
  cast this spell, if it was kicked" needs. A spell still on the stack now
  answers that question from its own cast signature, since the permanent it
  will become does not exist yet.

  A mana cost object on the wire gained a `colorless` member. It is
  additive, so the epoch does not move.

- **Voting (CR 701.34).** "Will of the council" asks each player in turn,
  starting with the resolving controller, and then acts on what got the most
  votes -- including everything tied for most. The ballot is read once,
  against that controller, before anyone votes: "a nonland permanent you
  don't control" means the same set for every voter, because "you" is the
  spell's controller rather than whoever is voting. Council's Judgment is
  the first card to use it, and exiling without targeting is what it is
  played for.

- **The monarch (CR 720).** At most one player holds the crown; they draw a
  card at the beginning of their own end step, and a creature that deals
  combat damage to them hands it to that creature's controller. Both are
  turn-based and rules-based actions rather than abilities anyone controls,
  so they are read where the step begins and where the damage lands. Cards
  can become the monarch and can trigger on someone becoming it. Palace
  Jailer is the first card to use it, and its "exile until an opponent
  becomes the monarch" is a linked exile released by a delayed trigger --
  which means a Jailer that has already died still gives the creature back,
  and a crown that never moves keeps it forever.

  Observations gained an optional `monarch`, present only while someone
  holds the crown. An optional member is additive, so the epoch does not
  move.

- **An additional cost with two ways to pay it.** "Sacrifice a creature or
  discard a card" is one printed cost, and a spell could only name one way of
  paying. A cost can now carry an alternative, and the ways of paying are the
  union of both halves -- each enumerated over its own zone, so a half nothing
  can pay simply contributes nothing. Which half is paid is settled as the
  spell is cast, like every other spent object, and the payment path already
  knew to sacrifice what it finds on the battlefield and discard what it finds
  in hand. Bone Shards is the first card to ask.

- **"Return target spell to its owner's hand."** Taking a spell off the stack
  and putting it somewhere is what a counter does, so the engine only had the
  countering version -- which checks "can't be countered" and is watched by
  everything that cares about a countered spell. Returning a spell is neither:
  Reprieve answers a spell that cannot be countered, and its controller keeps
  the card. A spell cast via flashback is still exiled instead (CR 702.34a),
  and a copy still ceases to exist.

- **Adventure.** An Adventure card is castable from hand either way; the
  adventure half exiles itself on resolution rather than going to the
  graveyard, and its owner may then cast the creature half from exile
  (CR 715.3d) -- as the creature, never as the adventure again. Casting from
  exile is a third cast source alongside hand and graveyard. Bonecrusher
  Giant is the first card to use it.

- **"Whenever this becomes the target of a spell."** Raised where a spell's
  targets are locked in, which is as it is cast, and once per targeting spell
  however many of its slots name the same permanent (CR 115.7c). The event
  carries the spell, so "that spell's controller" reads off it. Activated
  abilities target too, and this is deliberately not about them.

- **"Damage can't be prevented this turn."** A rule about every prevention
  rather than a prevention of its own, so it is read where damage is dealt,
  ahead of both the installed rules and the static ones. It also switches off
  what protection prevents (CR 702.16e) -- what protection does to targeting
  is untouched. Damage limits are not prevention (CR 615.1) and still apply.

- **A control change can hand a permanent to someone other than you.**
  "Gain control" was exactly that -- the effect's own controller received the
  permanent, with no way to say who else might. Wishclaw Talisman gives its
  artifact away as the price of the tutor, so a control change now names who
  receives it, and player references gained an `Opponent`. In this two-player
  engine "an opponent" names exactly one player and there is nothing to ask;
  a game with several would make it a choice. Nothing on the wire changes.

- **"As an additional cost to cast this spell, pay X life."** Toxic Deluge
  chooses X without printing `{X}` in its mana cost, and the engine derived a
  spell's X from the mana cost alone. A spell can now name a life cost, fixed
  or variable, and that cost bounds X the way an `{X}` bounds it: the casts on
  offer stop at the life its caster actually has, and every clause reading X
  reads the same choice. A spell naming both is bounded by whichever runs out
  first. The life is paid where the alternative-cost life already was, before
  the spell is finished on the stack.

  A bot reading `x` off a cast action should not assume a nonzero value
  implies `{X}` in the mana cost. Nothing else on the wire changes: a spell's
  own additional cost has never been catalog data, and the legal actions
  remain the authority.

- **A mode count that rises under a condition read as the spell is cast.**
  "Choose one. If you control a Wizard as you cast this spell, you may choose
  two instead" is Flame of Anor's, and a modal spell's maximum was previously
  fixed at what it printed. The larger maximum is now read where the spell is
  offered, which is what "as you cast" means, so the two-mode selections are
  legal actions only while the condition holds; the minimum is unchanged,
  because the extra mode is always optional. A play option's `modes` object
  gained an optional `conditionalMaximum`, present only for such a spell. An
  optional member is additive, so the epoch does not move.

- **Mana "in any combination of" more than one type.** A mana ability that
  divides one amount across several types is now offered once per division
  rather than once per type, so Vivi Ornitier at two power can make two blue,
  two red, or one of each. A mana ability resolves without ever holding
  priority, so the division travels with the action the way a counter size
  and a sacrificed permanent already do: `ActivateManaAbility` gained an
  optional `combination` mapping colour name to count, present only for such
  an ability, and `color` names the first type the division produces. An
  optional member is additive, so the epoch does not move.

  Two restrictions such an ability can print are now read on the mana path,
  which enumerates its offers separately from every other activated ability:
  "activate only during your turn" and "only once each turn". The second is
  also what lets a mana ability whose only cost is mana be offered at all --
  what an offered mana ability needs is a bound, and a printed per-turn cap
  is one.

- **Morph, and face-down permanents.** A card with morph may be cast face
  down for {3} as a 2/2 colourless creature with no name and no abilities
  (CR 708.2), and turned face up later by paying its morph cost. Turning it
  up is the new `TurnFaceUp` action: a special action, so it uses no stack
  and nothing responds to it. A permanent's own identity is unchanged
  underneath -- it is not a token, and it is the card that goes to the
  graveyard -- and only its controller's observation carries which card it
  is. A permanent gained an optional `faceDown` member; a spell cast face
  down reports the face-down body as its `definition` to everyone but its
  controller.

  The `legalActions` `type` vocabulary is **open** and its safe fallback is
  the one every bot already uses, choosing by `index`, so a new action kind
  is additive: the epoch does not move.

- **Cycling and typecycling.** Cycling is an activated ability that exists
  only while its card is in hand: the discard is a cost, so the card is
  already in the graveyard when the draw resolves. Typecycling buys a library
  search for the named type instead of a draw, and failing to find is allowed.
  "When you cycle this card" (CR 702.29b) fires on activation rather than on
  resolution, so it goes on the stack beside the discard rather than after
  the draw, and the cycled card is already in the graveyard when it does.
  Akroma's Vengeance, Secluded Steppe, Eternal Dragon, and Gempalm
  Incinerator.

- **Sligh is the second registered Premodern deck.** Registering it needed
  Mogg Salvage, whose free cast is conditional on the board -- an alternative
  cast can now carry a condition, and conditions can be conjoined, since the
  clause names two facts at once. Lightning Bolt, Earthquake, and Volcanic
  Hammer also gained the printings that put them inside the window.

  The registration test no longer demands that every main-deck card be
  complete: Incinerate's "can't be regenerated" rider needs damage-result
  linkage the engine lacks. It demands instead that nothing is metadata-only,
  and names the partial cards explicitly so the list cannot grow quietly.

- **Spending has a mode.** A nonmana cost used to spend whatever it named the
  way that object's zone implied -- a permanent sacrificed, a card in hand
  discarded. A cost can now say otherwise, which is what the free-spell cycle
  needs: Daze, Gush, and Thwart return their Islands to hand rather than
  losing them, and Pyrokinesis exiles the red card it spends rather than
  discarding it. That last one completes Pyrokinesis, which shipped partial.

- **Cursed Scroll**, and the two primitives it needed: revealing a card at
  random from a hand and binding it, and a condition comparing two bound
  objects by name. The reveal is drawn through the game's seeded RNG, so a
  replay reveals the same card. Naming a card is modelled as naming one of
  the cards in hand -- every name worth choosing is one of those, the choice
  is public either way, and nothing achievable is lost.

- **Grim Lavamancer**, the card that made the cost an array: two graveyard
  cards per shot, chosen by the player, so every pair is its own offered
  activation.

- **A spell can be paid for with something other than mana.** "You may
  sacrifice two Mountains rather than pay this spell's mana cost" is an
  alternative cast that changes only the cost, the way flashback does, and
  carries a nonmana cost instead of a mana one. The objects it names are
  spent as their zone dictates, which the cast path already did for a spell's
  own additional cost. Fireblast.

- **Echo.** "At the beginning of your upkeep, if this came under your control
  since the beginning of your last upkeep, sacrifice it unless you pay its
  echo cost." The intervening-if is what makes the cost come due exactly
  once, so the condition it reads is the new part; the trigger and the
  sacrifice-unless-paid were already there. Goblin Patrol.

- **"Activate only if ..." on an activated ability.** Threshold is a
  restriction on whether the activation is offered at all, not on what it
  does, so a false condition means there is no legal action rather than one
  that resolves and does nothing. Barbarian Ring.

- **Goblin Vandal**, which needed nothing new: an unblocked-attacker trigger,
  an optional payment, and a rule that stops the attacker assigning damage
  were all already there.

- **The first Premodern deck is registered.** `RG Goblins` is selectable in
  the Premodern format: every main-deck card resolves, and the list validates
  against the format's own set window and ban list. Registering it needed the
  Fourth Edition and Chronicles reprints of Red Elemental Blast and Tormod's
  Crypt, which are what put those two inside the window. The other seven
  staged lists stay in `decks/premodern/` until their cards resolve too.

- **Pyrokinesis**, with its damage complete and its alternative cost not.
  Every layer that carries an alternative cost is mana-only, so "exile a red
  card from your hand rather than pay this spell's mana cost" is recorded as
  metadata rather than executed. The card is castable for its printed cost.

- **Kicker.** A kicked spell is a spell cast for more mana with different
  instructions, which is what an alternative cast already models -- so kicker
  is one, carrying the whole kicked total rather than the surcharge. Overload
  and Prohibit, both of which target first and ask how big the thing was
  afterwards, so a too-large target is named legally and simply survives.

- **A mana ability can sacrifice another permanent.** "Sacrifice a Goblin:
  Add {R}" is a choice of which Goblin, and a mana ability never holds
  priority, so there is no window in which to ask: each candidate is
  enumerated as its own activation, the way an open-ended counter removal
  already becomes one activation per size. `ActivateManaAbility` carries the
  chosen permanent in an optional `costObject`, absent for every ability that
  sacrifices nothing but itself. Skirk Prospector.

- **Goblin Ringleader.** A look at the top of a library can now take every
  card matching its predicate without asking. "Put all Goblin cards revealed
  this way into your hand" is mandatory and has no printed bound, so a
  minimum and maximum could only approximate it -- any maximum small enough
  to be safe would let a player decline cards the card does not let them
  decline.

- **Banding.** `BandAttackers` names two declared attackers and puts them,
  with everything already banded with either, into one band; the engine offers
  only the pairs CR 702.21b allows, which is one or more creatures with banding
  plus at most one without. Each band member's `attackingBand` carries the
  index they share. A band is blocked as a group -- one declaration against any
  member puts the blocker in front of all of them, for one block rather than
  one per creature -- and a creature with banding on either side of a block
  hands its controller the other creature's damage division. Benalish Hero,
  Mesa Pegasus, Timber Wolves, Helm of Chatzuk, War Elephant, Icatian Infantry,
  Icatian Phalanx, Icatian Skirmishers, Knights of Thorn, Pikemen, Nalathni
  Dragon, and Fortified Area.

- Six Premodern **Goblins**: Goblin Lackey, Goblin Matron, Goblin Piledriver,
  Goblin Tinkerer, Goblin Warchief, and Siege-Gang Commander. None needed new
  engine work -- the put-a-card-from-hand-onto-the-battlefield choice, tribal
  counting, matching-spell cost reduction, and last-known mana value were all
  already there. Scourge is a new set module. RG Goblins is now the staged
  Premodern deck closest to playable, three main-deck cards short.

- **Premodern** is now a supported format: the twenty-nine-set window from
  Fourth Edition through Scourge, its own thirty-three-card ban list, nothing
  restricted, and contemporary mana rules. `CardSet` gained the fifteen sets
  in that window it was missing. The format is not offered in the web client
  yet -- it has no registered decks, and the staged tournament lists are
  promoted one at a time as their cards become playable.

- **Killing Glare**. A target predicate may now read the X a cast is being
  considered at. A spell has no stack object while its targets are being
  enumerated, so its chosen X was unreadable there; the enumerator already
  walks one X at a time, so it now says which one it means, and the same
  answer is used when a chosen cast is validated. "Power X or less" is the
  strict comparison against X plus one, so a sum became reachable from a
  predicate too.

- **Mentor of the Meek**. Nothing new: an arrival trigger, a power predicate,
  and an optional mana payment all existed, and "power 2 or less" is the
  strict comparison the predicates offer against three -- power is an integer,
  so at most two and below three are the same set.

- **Incursion Specialist**. A trigger condition may now compare the spells
  cast so far this turn, mirroring the one that reads last turn's. The count
  already includes the spell that caused the trigger, so "your second spell
  each turn" compares equal to two -- and equal rather than at least, so a
  third spell adds nothing.

- **Keymaster Rogue**. Its bounce is a non-targeted choice made as the
  trigger resolves, which the object-choice effect has always supported; a
  minimum of one is what makes it mandatory, so with nothing else out the
  Rogue returns itself.

- **Spell Rupture** and **Giant Adephage**, neither of which needed anything
  new. The greatest-power value landed a few commits ago and the
  counter-unless-paid helper predates it; a token that copies the permanent
  the trigger came from has existed as long as populate has.

- **Death's Presence** and **Ajani, Caller of the Pride**. The first needed
  nothing: the power of a creature that has already died has been readable
  since Sengir Vampire's own trigger. The second needed a life total as an
  amount, which is distinct from the fateful-hour conditions that compare one.

- **Traumatize**. A value may now read the size of the library belonging to
  the player a target slot points at, which composes with the halving that
  was already there.

- **Into the Wilds**. A top-card look may now put what it took onto the
  battlefield. Only what was taken: a card nobody chose has no reason to be
  put anywhere but back into a zone, so the leftover destination is unchanged.

- **Goblin Wizard** and **Gaea's Touch**. Putting a card from your hand onto
  the battlefield needed no new machinery: the card choice reads hidden zones
  and moves what it finds, and the destination the runtime has always handled
  was simply outside what the boundary check admitted. Widened to match --
  an outside-game import still has one destination, and a card already in a
  zone may also go to the battlefield.

- **Abattoir Ghoul** and **Dread Slaver**. Neither needed the trigger their
  audit lines named -- "whenever a creature dealt damage by this creature this
  turn dies" has shipped since Sengir Vampire -- and the toughness and
  reanimation halves landed in the two commits before this one.

  Authoring them did expose a real bug. A trigger captured on the battlefield
  names the object that was standing there, and the card it becomes on the way
  to a graveyard has a different identity, so "return it" resolved to nothing.
  Mortus Strider has shipped with that since it was written and quietly
  returned nothing at all; it now has a regression test. The move is followed
  through a successor map, which the checkpoint carries for the objects a
  pending trigger might still name.

- **Rise from the Grave**. A zone move may now carry a continuous effect the
  permanent arrives with, for the clauses that say what the thing they just
  reanimated now is. It belongs on the move rather than in a following effect
  because a permanent that enters is a new object with a new identity: by the
  time the next effect ran, nothing would name it. Dread Slaver stays blocked
  -- it also needs "dealt damage by this creature this turn", which is a
  separate gap.

- **Nephalia Smuggler** and **Conjurer's Closet**. A linked return may now
  name who the permanent arrives under rather than always handing it to its
  owner. "Under your control" and "under its owner's control" differ exactly
  when the creature was stolen, which is the reason a blink is worth playing.

- **Balustrade Spy** and **Undercity Informer**. A library may now be milled
  until a matching card turns up, with the match buried alongside everything
  above it. Distinct from an ordinary mill, whose count is known before it
  starts: how deep this goes is whatever the library says, and a library with
  nothing matching empties -- which is what makes these two a combo piece
  rather than a mill spell.

- **Cartel Aristocrat**, **Midvast Protector**, **Brave the Elements**, and
  **Alchor's Tomb**. An effect may now ask for a colour as it resolves and
  apply the answer, which no card could say before: the colour depends on
  what is on the stack, so it cannot be fixed in the declaration. Recipients
  are settled before the question is asked -- targets are already chosen, and
  a group is whatever it is at that moment -- so the decision carries only the
  answer. What the answer does is one of two operations: gain protection from
  the named colour, or become it. The checkpoint stores only the recipients;
  the operation and duration are read back off the effect the continuation
  already locates.

- **Transmutation** and **Fluxcharger**. Power and toughness may now be
  switched, in layer 7e and therefore after every other power-and-toughness
  layer. The effect carries no values because it names none: two switches in
  effect at once cancel, so only the parity of how many apply matters. Both
  cards are tested against a lopsided creature, since a switch is invisible on
  an evenly-statted one.

- The four Gatecrash **Keyrunes** -- Boros, Gruul, Orzhov, and Simic. No
  engine work: the Return to Ravnica half of the cycle has shipped with this
  exact animation all along, and these four audit lines had simply not caught
  up with it.

- The five **storage lands** -- Bottomless Vault, Dwarven Hold, Hollow Trees,
  Icatian Store, and Sand Silos. A cost may now remove an open-ended number of
  counters, with the size chosen as the ability is activated: the mana path
  turns it into one activation per removable count, so nothing downstream ever
  carries an unanswered question. `ActivateManaAbility` gained an optional
  `countersRemoved`, present only for such an ability, because source, origin,
  and colour name one storage land's ability once per size. Adding an optional
  member is not a breaking epoch change, and every other mana ability's wire
  shape is unchanged.

- **Boundless Realms**, **Diabolic Revelation**, and **Frenzied Tilling**. A
  library search's ceiling is now a value rather than a constant, so "up to X"
  can be sized by the chosen X or by the board the spell is cast into. The
  boundary check still holds a constant maximum to its minimum and to the
  one-card ceiling a library destination needs; a maximum sized at resolution
  answers neither question in the declaration, so it is supported everywhere
  except back into a library. The tapped arrival two of these three wanted had
  been available all along.

- **Essence Harvest** and **Fungal Sprouting**, and Garruk, Primal Hunter's
  draw ability, which had stood at metadata-only. A value now reads the
  greatest power among a set of objects -- one creature's size rather than a
  count of them or a total across them, and zero when nothing matches.

- **Death's Caress**, **Disciple of Griselbrand**, and **Korozda Guildmage**.
  A target condition now reads its object as it last existed rather than only
  where it currently is: "if that creature was a Human" is asked after the
  destruction that removed it, and a permanent that leaves the battlefield
  gets a fresh object identity in its new zone, so the corpse in the retired
  table is the only thing the old target still names. The other two take the
  sacrifice as the ability's own decision, the shape Diamond Valley
  established, because what was sacrificed has to be readable by what follows.

- **Predator's Rapport**, **Sheltering Word**, and **Tribute to Hunger**. A
  target slot's power was readable and its toughness was not, which is the
  only thing all three were waiting on; the sum of the two was already
  expressible. Tribute to Hunger is Devour Flesh with the life pointed the
  other way, and its audit line had gone stale on its own.

- **Devour Flesh** and **Feed the Pack**, both paid in the sacrificed
  creature's toughness. Neither needed engine work -- the follow-up learned to
  read toughness two commits ago, and their audit lines had simply not caught
  up. Devour Flesh pays the player who lost the creature rather than the
  caster; Feed the Pack names a nontoken creature, so the Wolves it makes can
  never feed it back.

- **Righteous Authority**. A hand-count power/toughness may now be read for the
  enchanted permanent's controller rather than the source's own, which the
  general player relation cannot answer because it has no source to follow.

- **Stony Silence** and **Sturmgeist**. The activation prohibition now reaches
  mana abilities, which are enumerated on their own path and so needed the
  rule read in a second place; and a static power/toughness may be defined by
  its controller's hand size, read live.

- **Curse Artifact**, the second card on the declined branch. It names exactly
  the permanent it is attached to rather than any the player controls, and
  asks that permanent's controller rather than the Aura's.

- **Elder Spawn**, and with it a declined branch on an optional sacrifice.
  "Unless you sacrifice an Island" is one offer with two branches rather than
  a payment and a separate check, so the toll falls both when the controller
  says no and when there is no Island to say yes with -- the second case
  without asking at all. Checkpoints carry the branch as an additive field.

- **Skaab Goliath**, and with it additional costs that name more than one
  object. The casting enumeration offers one action per way of paying, so a
  cost naming two cards enumerates every pair -- three creature cards in the
  graveyard is three ways to pay, not three cards to pick.

- **Bone Splinters** and **Infernal Plunge**, both of which eat a creature on
  the way to the stack. Neither needed engine work -- Altar's Reap in the same
  set already pays the identical cost.

- **Discarding a chosen card as an activation cost**, and with it **Mad
  Prophet** and **Tin Street Market**. The card travels with the activation
  the way a chosen sacrifice already does, so there is no mid-payment
  decision: the enumerator offers one activation per discardable card, and an
  empty hand offers none.

- **Way of the Thief** and **Dimir Keyrune**. The Thief's evasion is another
  Gate-conditional static, on the Aura's controller rather than the creature's;
  the Keyrune applies its animation and its evasion as one effect, so both
  lapse together at end of turn.

- **Ethereal Armor** and **Ogre Jailbreaker**, two statics whose answer is a
  battlefield count. The Jailbreaker's audit line named the exact reason it
  was blocked -- a static condition counting Gates -- and that reason stopped
  being true when static clauses were allowed to read the board.

- **One Thousand Lashes**, which is Arrest's three prohibitions plus a drain
  that follows the enchanted creature's controller. Nothing new was needed.

- **Encrust** and **Skygames**, two more Auras. Encrust reuses Arrest's new
  activation prohibition and pairs it with the untap one; Skygames grants an
  activated ability that keeps its own sorcery-speed restriction.

- **Arrest** and **Mugging**. Arrest brings a prohibition on activating a
  permanent's activated abilities -- only the activations, so its triggered
  and static clauses are untouched -- and applies it with the attack and block
  prohibitions as one effect, so the Aura leaving returns all three together.
  Mugging needed nothing new.

- **Reliquary Tower**, and with it a player rule removing the maximum hand
  size. Read at cleanup rather than captured, so losing the Tower puts the
  limit back for that very cleanup, and it says "you" -- an opposing Tower
  does nothing for your discard.

- **Flowering Lumberknot**, which can neither attack nor block unpaired. A
  static "as long as" clause may now read a predicate off its own source, the
  same way it already could off an attached permanent.

- **Six more soulbond cards**: Nearheath Pilgrim, Galvanic Alchemist, Stern
  Mentor, Tandem Lookout, Stonewright and Diregraf Escort. The four with a
  quoted ability grant it to each creature separately, so the pair holds two
  copies rather than sharing one.

- **Eleven more soulbond cards**: Silverblade Paladin, Spectral Gateguards,
  Elgaud Shieldmate, Wingcrafter, Hanweir Lancer, Lightning Mauler, Druid's
  Familiar, Geist Trappers, Nightshade Peddler, Pathbreaker Wurm and Wolfir
  Silverheart. Every one is the same clause with a different grant, and each
  grant reaches both halves of the pair and lapses when the pair breaks.

- **Soulbond**, and with it **Trusted Forcemage**. Pairing is state on the
  permanent rather than a one-shot effect: CR 702.94 is two triggered
  abilities, the relation is symmetric, and the pair breaks with the other
  state-based actions the moment one of the two stops being a creature its
  controller controls. Twenty more Avacyn Restored cards are blocked on
  nothing else.

- **Wolfhunter's Quiver**, whose two granted tap abilities share the equipped
  creature's one untap. Only the three-damage one names a Werewolf.

- **Runechanter's Pike**, **Tormentor's Trident** and **Vanguard's Shield**,
  three more Equipment. The Pike recounts your graveyard continuously, the
  Trident's attack requirement travels with the Equipment, and the Shield's
  extra block reuses the additional-block rule banding needed.

- **Riot Gear**, **Kitesail**, **Executioner's Hood**, **Heavy Mattock** and
  **Bladed Bracers**. Two dozen audit lines claim Equipment and the equip
  procedure are unavailable; they have been available for a while. The Mattock
  and the Bracers read the equipped creature's type live, so moving them
  changes what they give.

- **Builder's Blessing** and **Eternal Flame**, two more stale audits. The
  Blessing wanted a recipient narrowed to untapped creatures, which Arcades
  Sabboth already needed; the Flame wanted a halved count rounded up, which
  Aspect of Wolf already needed. Neither needed engine work.

- **Ivory Guardians** and **Woodborn Behemoth**, two more board-conditioned
  statics. The Guardians pump by name rather than by control, so a copy on
  either side of the table is covered -- but each copy's own "an opponent" is
  read from its own controller, so only one of two facing copies is usually
  live.

- **Angelic Voices**, **Beasts of Bogardan**, **Goblin Caves** and **Goblin
  Shrine**, four old anthems the board switches on and off. All four fell out
  of letting a static clause read the battlefield: Angelic Voices reads an
  absence, the Beasts read the other side of the table, and the two Auras read
  the land they are sitting on.

- **Guildscorn Ward**, **Fog Bank** and **Night Revelers**. Protection gains a
  fourth quality -- multicolored, which is a colour *count* rather than any one
  colour, so a mono-black source gets past a Ward that stopped a black-red one.
  A static "as long as" clause may now read a battlefield count, which is what
  the Revelers watch the opponent's Humans with.

- **Protection from creatures**, and with it **Holy Mantle**. The third
  protection quality, and the only one with no parameter at all: it reads the
  source's card type. The Aura granting it is an enchantment, so it does not
  throw itself off the creature it protects.

- **Protection from a creature type**, and with it **Elite Inquisitor**,
  **Grave Bramble** and **Midnight Duelist**. Protection is one keyword per
  quality and every quality used to be a color; a type is now equally a
  quality, read in the same four places -- targeting, damage, being blocked,
  and Aura legality. The types are a closed set for the same reason the basic
  land types are, so the checkpoint tag stays exhaustive.

- **Avenging Arrow** and **Executioner's Swing**, both of which can only aim
  at a creature that has already dealt damage this turn. That is the mirror of
  the record Giant Shark reads, and it counts damage to anything -- a creature
  that traded in combat is as legal a target as one that connected.

- **Giant Shark**, and with it a turn-long record of damage dealt to a
  permanent. "Has been dealt damage this turn" is not "has damage marked on
  it": regeneration and cleanup both wipe the marks, so the new
  `WasDealtDamageThisTurn` predicate reads a flag that only the turn boundary
  clears. Checkpoints carry it as an additive field.

- **Rootwalla** and **Stab Wound**, two more cards blocked on machinery that
  already existed. The Rootwalla's once-a-turn quota belongs to the permanent,
  so a second one still has its own, and Stab Wound's upkeep drain follows the
  enchanted creature's controller rather than the player who cast the Aura.

- **Spitting Slug** and **Arcades Sabboth**, two old cards whose audit lines
  blamed missing combat constraints. The Slug needed an optional payment with
  a branch either way -- declining hands the first strike to the other side of
  the block, which is not the same as nothing happening -- and Arcades needed
  only a recipient narrowed by what each creature is doing right now.

- **Pontiff of Blight**, **Battering Krasis**, **Emmara Tandris** and **Tithe
  Drinker**. Extort moved into the shared ability helpers so the Pontiff can
  grant it, each granted instance offering its own payment, and Emmara's
  prevention is the first shield installed on a whole group of permanents
  rather than on one.

- **Ogre Slumlord** and **Sublime Archangel**, two cards that hand an ability
  to a group. The Slumlord's is a keyword given to Rats, which brings a 1/1
  black Rat token with it; the Archangel's is exalted, and each granted copy
  is its own instance, so attacking alone scales with the board.

- **Illness in the Ranks**, **Phantom General**, **Harvester of Souls** and
  **Soul of the Harvest**, four cards that read a permanent's token status.
  Two want tokens and two want everything but tokens: the General's anthem
  skips its own creature cards, and neither draw trigger fires off a token.

- **Intangible Virtue**, **Army of the Damned** and **Endless Ranks of the
  Dead**, three Innistrad token cards resting on a token predicate, a tapped
  token creation and a halved count. Endless Ranks rounds down, so a single
  Zombie makes none and the engine only starts at two.

- **Clinging Mists** and **Village Survivors**, two more fateful-hour cards
  built from the threshold plus machinery that already existed. The Survivors
  has vigilance printed as well as granted, so losing the branch takes it from
  everything else and leaves its own alone.

- **Gather the Townsfolk** and **Thraben Doomsayer**, finishing the fateful-hour
  cards that need only the life threshold. The Townsfolk's "instead" is one
  token creation of a chosen size rather than two creations one of which is
  skipped, which needed a life-conditioned value alongside the condition.

- **Break of Day** and **Gavony Ironwright**, with a controller-life threshold
  condition for fateful hour. The two read it differently: Break of Day checks
  once as it resolves, while the Ironwright's "as long as" is continuous, so
  its anthem switches off again when life climbs back above five.

- **Thalia, Guardian of Thraben** and **Archangel's Light**. Thalia is the
  spell-cost increase Derelor introduced, with no "you cast" clause, so she
  taxes her own controller too. The Light gains one doubled amount rather than
  two separate gains, counted before the shuffle empties what it counted.

- **Farbog Explorer**, **Goldnight Redeemer** and **Fettergeist**, three more
  cards resting on swampwalk, a doubled count and a dynamic generic payment.
  Two of the three say "other creatures you control", and the Fettergeist's
  tax is zero when it stands alone -- still a choice, just a free one.

- **Goblin Battle Jester** and **Predatory Rampage**, two turn-long block
  clauses pointing opposite ways. Both audit lines said no turn-long effect
  could express them; both rules existed.

- **Duty-Bound Dead**, **Hamletback Goliath** and **Elvish Archdruid**. The
  Archdruid extends the mana-ability amount from a counter count to a
  battlefield count: either is knowable before the ability is activated, which
  is the property the boundary rule actually cares about.

- **Golgari Decoy**, **Experiment One** and **Thrashing Mossdog**, three more
  cards resting on evolve, scavenge and regeneration. The Decoy's block clause
  is a lure -- every able blocker must block *it* -- rather than a requirement
  on the blocker to block everything it can.

- **Gyre Sage** and **Sewer Shambler**, with a mana ability whose amount is
  read off the permanent offering it. A mana ability's amount has to be known
  before it is activated, which a counter count is; it is resolved as the
  activation is built, so the payment planner and the pool see one number.

- **Wake the Reflections** and **Druid's Deliverance**, closing the populate
  cluster. The Deliverance's shield is scoped to its controller rather than
  covering the whole combat the way a Fog does, so creatures on both sides
  still trade.

- **Congregate** and **Wall of Frost**, two more stale audit lines. A doubled
  object count is what `Scaled` has always done, and the identity of the
  creature a Wall blocked is the block trigger's own object.

- **Master of the Pearl Trident** and **Sleep**, whose audit lines named
  islandwalk and the tap-plus-untap-skip pair as unavailable. Both had been
  available for a while; the scoping is what needed care, since the Master
  reaches only your own other Merfolk and Sleep only the targeted player's
  creatures.

- **Trostani, Selesnya's Voice** and **Vitu-Ghazi Guildmage**, closing all but
  one of the populate cluster. Trostani reads the entering creature's
  toughness rather than her own, and does not feed on her own arrival.

- **Wayfaring Temple** and **Sundering Growth**, two of the five cards whose
  audit lines said populate was unavailable. It has its own procedure and its
  own tests; what was missing was the authoring.

- **Morkrut Banshee** and **Hollowhenge Scavenger**, the Innistrad half of the
  same morbid pattern. The Banshee is the sharpest case for suppressing the
  trigger rather than its effect: -4/-4 forced onto whatever it had to choose
  would kill it.

- **Wakedancer** and **Ulvenwald Bear**, two morbid entry triggers. Morbid is
  an intervening if, so with nothing dead the trigger is never created rather
  than created and doing nothing -- which on the Bear means no target is ever
  chosen.

- **Crippling Chill** and **Frost Breath**, whose audit lines both asked for a
  duration tied to the affected creature's controller's untap step. A duration
  is the wrong shape for a spell that can reach both sides at once; the
  per-permanent skip that already existed is what makes each creature miss its
  own controller's step.

- **Frilled Oculus** and **Gridlock**, two more cards whose audit lines had
  gone stale. The once-per-turn activation ration and the X-counted target
  slot they asked for were both built for other cards, so neither needed
  engine work.

- **Street Spasm**, whose audit line said the mana model could not represent
  its {X}{X}{R}{R} overload cost. It can: repeated symbols accumulate, so the
  doubled X was already handled and the card needed no engine work.

- **Gloom**, which taxes both casting and activating. The spell half reuses
  the increase Derelor introduced; the activation half is new, and applies at
  the offer, at the X ceiling, and at payment, so what is charged is what the
  ability was priced at. It names white *enchantments*, so a white creature's
  ability is untouched.

- **Derelor**, with a coloured spell-cost increase. An increase is not a
  discount with the sign flipped: a discount may only touch generic mana
  (CR 601.2f), while this adds a black pip that only black mana pays. It
  applies before any discount, so a discount cannot eat mana the increase then
  adds back.

- **Sentinel**, with a toughness-only base set and a summed value. Setting
  only toughness is the mirror of the existing power-only setter and for the
  same reason: a card that changes one half says only that half, so the
  Sentinel keeps its printed power of 1 however large a creature it faces.

- **Spore Cloud**, also needing no new engine work: per-permanent untap skips
  already existed, and its audit line asked for them as though they did not.
  Counting the skip per permanent rather than expressing it as a duration is
  what makes it right for a card reaching both sides of a combat, since the
  two players do not arrive at their untap steps together.

- **Paralyze**, which needed no new engine work: the upkeep trigger keyed to
  the enchanted permanent's controller already existed, and its audit line had
  gone stale. Everything about the card points at the host's controller rather
  than the Aura's -- their upkeep, their mana, their creature.

- **Cyclone**, with a single-colour mana payment whose size is counted at
  resolution. The counter goes on before the bill arrives, so the first upkeep
  already costs {G} rather than nothing, and what paying buys is symmetric:
  the damage reaches every creature and every player, its controller included.

- **Khabál Ghoul** and **Scavenging Ghoul**, with a running count of the
  turn's creature deaths and a corpse counter. The count is tallied as
  creatures die rather than read off a graveyard, because a graveyard is not a
  record of this turn: bodies already there when the turn began feed neither
  Ghoul.

- **Aspect of Wolf**, with halving as a value form. The rounding direction
  belongs to the division rather than sitting over it, which is what lets one
  count of Forests be read twice and land on two different numbers: five
  Forests is +2/+3.

- **Part Water**, whose X-counted target slot needed no new engine work: the
  count sentinel added for Candelabra of Tawnos already covers a spell. Its
  {X}{X}{U} cost doubles X, so seven mana reaches three creatures rather than
  six, and the targeting and the payment agree about that.

- **Mishra's War Machine**, with discarding as an unless-payment. Unlike a
  mill, an empty hand cannot pay at all, so the damage becomes the only
  branch; which card goes is settled after the branch is, because the branch
  does not depend on it.

- **Deep Spawn**, with milling as an unless-payment. A library shorter than
  the amount mills what it has rather than failing to pay, so the choice is a
  real one down to the last card.

- **Word of Binding**, the X-linked target count reached through the casting
  path rather than an activation. Cast-time revalidation read the count
  sentinel literally, so a spell that enumerated correctly was then rejected
  as it paid; it now resolves the sentinel against the chosen X the same way
  the enumerator does.

- **Urza's Mine**, **Urza's Power Plant**, and **Urza's Tower**, with a mana
  amount conditioned on other permanents you control and a conjunction of
  conditions to express it. The amount is resolved as the activation is
  offered, so payment planning and the mana pool agree about what a tap is
  worth, and losing a piece takes the bonus away again.

- **Candelabra of Tawnos**, with a target count taken from the X that was
  paid. "Untap X target lands" links the two numbers, so an X larger than the
  board offers no declaration rather than untapping fewer lands than paid for.

- **Jade Statue**, with a during-combat activation window. The window is the
  whole combat phase on either player's turn rather than one step, so the
  Statue can animate before attackers are chosen or after blockers are, and
  the animation expires with the combat rather than at cleanup.

- **Howling Mine**, with an untapped-source trigger condition. The "if
  untapped" is an intervening-if, read both when the draw step begins and
  again as the trigger resolves, so tapping the Mine in response still denies
  the extra card.

- **Instill Energy**, with attacking as though hasty. The permission is
  narrower than haste and stops where the printed text stops: the enchanted
  creature may attack the turn it arrives, but its own {T} ability is still
  summoning sick.

- **Living Artifact**, with a vitality counter. It banks by the amount of the
  damage rather than one per event, and its upkeep offer is gated on having
  something to spend: "you may remove a counter" with none banked is not a
  choice worth asking about.

- **Reset**, the third card in the casting-window cluster, and the last of
  those three windows: an opponent's turn past their upkeep. A play option's
  `restriction` may read `opponentsTurnAfterUpkeep`.

- **Festival and Teleport**, and the two casting windows they name. "Only
  during an opponent's upkeep" is the first window that depends on who is
  casting rather than only on the step, so the timing check now takes the
  caster. A play option's `restriction` may read `opponentsUpkeep` or
  `declareAttackersStep`.

- **Energy Tap.** Its audit line asked for mana provenance; the mana it makes
  is an ordinary amount that happens to be read off the creature it tapped
  rather than printed, which the effect model already supported.

- **A permanent remembers the X it was cast for.** An enters trigger is a new
  object, so it could not ask the spell that made the permanent what X was and
  read it as zero. Permanents now carry it, and a value reads it back. Venarian
  Gold, whose sleep counters are X of them.

- **Cocoon**, with a pupa counter. Everything else it wants was already there:
  a static gated on the source's own counters, an untap prohibition, and a
  permanent keyword grant. Its "if you can't" branch is two complementary
  conditions rather than a branch, so the upkeep that sheds the last counter
  is not also the one that opens it.

- **Osai Vultures**, with a carrion counter and morbid as an intervening-if.
  "A creature died this turn" existed as a value that picks between two
  amounts; as a condition it can now gate whether a trigger does anything at
  all.

- **Armageddon Clock**, with a doom counter and a way to take counters off.
  The counter vocabulary could add counters and clear a kind entirely, but not
  remove a few; removing some is the mirror of adding some.

- **Abilities any player may activate.** A printed "any player may activate
  this ability" now puts somebody else's permanent in your action list, for
  that ability alone. The permanent stays the source, so what it does is still
  its doing, and the player who activates is the one who pays. Ifh-Bíff Efreet.

- **Xenic Poltergeist.** Its audit line asked for temporary artifact animation
  with a dynamic mana-value size. Animation is a card type and a base size
  applied together, both of which existed, and the size is the mana value of
  what the ability pointed at -- a value the model already had.

- **Primordial Ooze.** Its audit line blamed a combat constraint; "attacks
  each combat if able" has been a keyword for a while, and the upkeep toll is
  an unless-payment whose amount is read from the counters on the source
  after the new one goes on.

- **Erg Raiders**, and two ways for an intervening-if to read its own source.
  A condition can now ask whether the ability's source matches a predicate,
  the way it could already ask about the permanent an Aura is attached to; and
  a permanent can be asked whether it came under its controller's control this
  turn, which is the fact summoning sickness already reads.

- **Blaze of Glory**, and the blocking requirement it needs. A creature can be
  made to block every attacker it legally can, which is the mirror of the
  requirement an attacker could already carry. The card is that plus the
  already-implemented permission to block any number: either half alone would
  be a different card.

- **Creature Bond.** A death trigger can read the dead creature's toughness
  now, the way it could already read its power. Both come from last-known
  information, because the creature is in the graveyard by the time the
  trigger resolves.

- **Ashnod's Transmogrant.** Its audit line asked for card-specific counter
  state; it puts one ordinary +1/+1 counter and adds a card type, both of which
  the vocabulary already had.

- **Discarding at random as an activation cost.** Unlike the discard its payer
  chooses, nobody decides which cards go, so paying it needs no decision at
  all: the cards leave as the cost is paid, picked off the seeded generator.
  Coral Helm and Draconian Cylix. A random discard can also be filtered now --
  "discards a creature card at random" reaches past everything else and takes
  nothing from a hand holding none. Rag Man.

- **Desert and Island of Wak-Wak.** Desert waits for the end-of-combat step,
  which is a new activation window and the whole point of the card: it finishes
  off something that survived rather than stopping it. Island of Wak-Wak is
  Singing Tree's base-power setter pointed at a flier.

- **Disharmony**, and the casting window it needs. "Only during combat before
  blockers are declared" is narrower than any window the engine had; it is the
  two steps before the declaration, because nobody holds priority inside the
  declaration itself. A play option's `restriction` may now read
  `beforeBlockersDeclared`; the set is open and unknown tags are safe to treat
  as a window the engine enforces.

- **Control Magic and Steal Artifact.** Both audit lines wanted an
  attachment-scoped control change. The control the engine already has is
  scoped to its source remaining on the battlefield, and for an Aura those are
  the same thing: an Aura with nothing under it goes to its owner's graveyard,
  so destroying it hands the permanent back.

- **Goblin War Drums.** Its audit line wanted menace as an executable
  constraint and a way to grant a keyword from outside. Both have existed since
  the menace pass; the card is a static grant over a whole side.

- **Goblin Kites.** Berserk's shape with a coin in it: a pump now, and a
  delayed trigger that remembers the same creature and may take it away. Both
  halves were already there.

- **Sol'kanar the Swamp King.** Its audit line said trigger capture could not
  see a spell's color. It has been able to since the cast event started
  carrying locked characteristics; nothing was missing but the card.

- **Setting base power alone.** "Has base power 0" says half of what a base
  power-and-toughness effect says, and the layer kept only its latest setter,
  so there was no way to name one half without inventing the other. Base
  setters now apply in timestamp order over the printed stats. Singing Tree.

- **Three combat triggers whose audit lines blamed a missing constraint.**
  None of them needed one. Elder Land Wurm drops defender the first time it
  blocks and keeps it dropped, Dwarven Soldier grows against Orcs on either
  side of the block, and Battering Ram bands into combat and marks the Wall
  that stopped it.

- **Mana added off another player's tap.** A mana trigger can now name the
  controller of whatever was tapped rather than its own controller, which is
  what "its controller adds an additional {G}" asks for when the watcher and
  the land belong to different players. Wild Growth and Gauntlet of Might.

- **"Bands with other."** CR 702.21j's banding variant, narrowed to a quality:
  every member of the band must have it and at least one must carry the
  ability, and the damage rule wants two qualifying creatures rather than one.
  The five Legends band lands grant it to legendary creatures of their own
  color, Master of the Hunt's Wolves print it against their own name, and
  Tolaria and Shelkin Brownie take it away. Adventurers' Guildhouse, Cathedral
  of Serra, Mountain Stronghold, Seafarer's Quay, Unholy Citadel, Tolaria,
  Master of the Hunt, and Shelkin Brownie.

- **Blocking an additional creature.** A creature blocks one attacker unless a
  card says otherwise. Two-Headed Giant of Foriys.

- **Targets with no printed limit.** "One or more target creatures" is
  bounded by the board rather than by a number, which the declaration model
  could not previously say. Heaven's Gate, Sea Kings' Blessing, Touch of
  Darkness, Dwarven Song, and Sylvan Paradise.

- **Reading the toughness of what was sacrificed.** A sacrifice follow-up
  could only ever read the sacrificed permanent's power. Both are last-known
  by the time it runs, so neither was harder to reach -- the card simply had
  no way to say which it meant. Diamond Valley and Life Chisel.

- **"Spells you cast cost less to cast."** Read off a permanent rather than
  the card in hand, so unlike a card discounting itself it has to name which
  spells and whose. Several stack, and none can reach a cost's coloured pips.
  Goblin Electromancer, Arcane Melee, Planar Gate, and Mana Matrix.

- **Menace.** "Can't be blocked except by two or more creatures" is a
  constraint on the finished declaration rather than on any one block: the
  first blocker is legal and only becomes illegal by being the last, so the
  declaration is what refuses to end. Ripscale Predator, Madcap Skills, and
  Gruul War Chant, the last two granting it rather than printing it.

- **Ghostly Possession**, which wears the same two-sided shield Gaseous Form
  has worn since Legends.

- **Fortress Cyclops, Somberwald Vigilante, and Hamlet Captain**, the first
  cards to use the one-directional blocking triggers added earlier in this
  release. Hamlet Captain's single printed clause becomes two triggers, since
  a creature cannot both attack and block and so exactly one of them fires.

- **Pacifism, Crippling Blight, and Tormented Soul.** "Can't attack or block"
  is two prohibitions rather than one combat ban, which is why nothing in the
  vocabulary bars combat wholesale and nothing needs to.

- **Goblin Shortcutter, Welkin Tern, and Defang.** Three more whose audit
  lines named capabilities the engine had: a turn-long blocking prohibition, a
  blocking restriction that reads a keyword rather than a type, and a static
  shield over every damage event its host is the source of.

- **Seraph of the Sword and Armored Transport**, two static combat shields.
  The Seraph's is blanket; the Transport's names only the creatures blocking
  it, which is a narrower thing than it looks -- a creature the Transport is
  blocking is not one blocking the Transport, and its damage lands.

- **Hunted Ghoul, Fervent Cathar, and Malicious Intent.** All three were
  marked as needing blocking restrictions the engine already had: one names a
  creature type, and two hand the prohibition out for a turn.

- **Nivix Cyclops**, whose trigger hands out the attack-despite-defender
  permission for a turn. Ogre Jailbreaker prints the same permission and stays
  blocked: its condition counts Gates across the battlefield, and the static
  walk is deliberately trusted only with conditions reachable from the source.
  Its audit line now says so.

- **Deadly Allure and Enlarge**, which hand out the must-be-blocked
  requirement for a turn rather than printing it as a static. The rule already
  reached both ways; only the two printed statics had been authored.

- **Elvish Scout and Glyph of Destruction**, both of which needed nothing new.
  Their audit lines had blamed a missing duration-scoped prevention effect,
  which has since arrived by other routes -- so these are the first two of the
  stale entries to be cleared rather than implemented.

- **"Players can't untap more than one ... during their untap steps."** Winter
  Orb and Smoke each carried their own version of this in the untap procedure,
  one for lands and one for creatures. It is now a player-facing static naming
  the group it caps, so several compose and each narrows only its own. Damping
  Field joins them, and the `WinterOrb` and `Smoke` behaviors are gone.

- **The halves of "blocks or becomes blocked by."** The union was already
  expressible; these are the two directions on their own, for the cards that
  print only one. Both read the same ordered pair of events and tell the sides
  apart by which creature was attacking. Infernal Medusa and Venom.

- **"Can attack as though it didn't have defender."** A permission rather than
  an ability removal, so the Wall keeps the keyword and anything reading "a
  creature with defender" still finds one. Every other reason it cannot attack
  is untouched. Animate Wall and Wall of Wonder.

- **"Can't block creatures with power 2 or greater."** Ironclaw Orcs carried
  this as a hardcoded behavior in the combat action generator; it is now an
  ordinary static ability, authored as the permission the restriction leaves
  behind. Blocking restrictions also read the attacker's real current power,
  so a creature a Crusade has pumped is one the Orcs will no longer block.
  Brassclaw Orcs and Orcish Veteran join it, and the `IronclawOrcs` behavior
  is gone.

- **"Target player reveals their hand."** A public reveal, where looking at a
  hand told only one player. It is its own step, so a hand with nothing to
  discard and nothing to count is still shown, and what follows reads the hand
  afresh rather than the reveal's result. Amnesia and Inquisition.

- **Looking at the top of a library that is not yours.** Digging through your
  own library names one player twice, so the library and the player being
  asked were the same thing; a spy separates them. A selection that may take
  nothing is now presented as what it is -- a look, with the cards shown and
  nothing to choose between. Orcish Spy and Visions.

- **"If it attacked during your last turn."** History rather than turn state:
  the answer has to outlive the cleanup that clears "attacked this turn", so
  the turn a creature last swung on is recorded alongside who controlled it
  then. The condition sits on each static's recipient, so the prohibition is
  read live -- nothing is installed when the creature attacks and nothing has
  to expire when it stops being true. Giant Turtle, Goblin Rock Sled, and
  Tangle Kelp.

- **"This creature assigns no combat damage this turn."** A constraint on the
  assignment rather than a shield over the result: a creature under it is not
  asked how to divide its damage at all, so trample has nothing to spill and
  no blocker is dealt a lethal share. Both printed carriers pay for their
  effect with the swing they were about to land. Farrel's Zealot and Floral
  Spuzzem.

- **Combat requirements: "all creatures able to block this do so."** The
  vocabulary had only blocking prohibitions, so a requirement had no shape at
  all. A requirement never beats a restriction, so "able" is read from the
  same legality that offers a block in the first place -- a tapped creature,
  or one that cannot block that particular attacker, is simply not required.
  What the requirement does is take the alternatives away: a creature that
  could block the lured attacker is offered no other seat, and the defending
  player cannot finish declaring blockers while one of them is still free.
  Lure and Marble Priest.

- **Damage that is capped rather than prevented.** A limit has no capacity to
  spend and no follow-up: it applies to every matching event for as long as
  its source is there, which is what separates it from a prevention shield.
  The two printed shapes differ in what the cap depends on -- a flat number,
  or the recipient's life when the damage would be dealt -- so the
  life-relative one cannot be folded into the flat one. A limit protecting a
  player is found by its own walk over the battlefield, since nothing about
  the damage event points back at the permanent carrying the rule. Ali from
  Cairo and Forethought Amulet.

- **"Damage dealt to you this turn."** A running total per player,
  accumulated as the damage lands rather than derived from life totals, so
  gaining life in between does not erase it. Damage is also recorded under
  each source group it belongs to, since a group such as "by artifacts" is
  only answerable while the artifact is still the source. Simulacrum and
  Reverse Polarity.

- **"Can't attack if ..." as the mirror of "can't attack unless ...".** The
  negation is over the existential rather than the object -- stopped when
  anything matches, rather than when nothing does -- which a negated object
  predicate cannot express. Read as attackers are declared, so tapping the
  deterrent frees the attacker. Orgg.

- **The other side of the blocking relationship.** A Wall printing "creatures
  it's blocking" reads the relationship outwards from itself; The Wretched
  reads it inwards, from the creatures that blocked it. Both are needed
  because only the blocker records what it blocked, and together in an
  `AnyOf` they are the printed "blocking or blocked by this creature".

- **An until-end-of-combat duration.** The shortest lifetime the engine has:
  it expires as the end-of-combat step finishes rather than waiting for
  cleanup, so a creature pumped for one combat is back to its printed size in
  the postcombat main phase, and a second combat starts it over. Murk
  Dwellers.

- **Two ISD–DGM Standard decks and their remaining catalog coverage.** Todd
  Anderson's Omnidoor Thragfire and Brian Braun-Duin's Naya Midrange January
  2013 Star City Games lists are built in at 60 cards plus 15-card sideboards.
  Their previously missing identities use current shared declarative effects
  where exact, including hand-only Increasing Ambition searches, Temporal
  Mastery's extra turn and miracle, and tapped battlefield searches for Farseek
  and Ranger's Path; unsupported clauses remain explicitly partial or
  metadata-only.

- **Protocol 22 establishes the durable compatibility model.** JSON objects are
  open-world, so consumers ignore members they do not use. `protocolVersion`
  now moves only for incompatible interpretation changes; new cards, rules
  fixes, and different legal-action membership through existing action shapes
  change the automatic `simulationFingerprint` instead. Observations and
  catalogs advertise `protocolCapabilities`; the current reconstruction
  facility is `reconstruction.checkpoint.v3`. Stable wire tags are explicit
  mappings rather than Rust `Debug` output. Protocol 22 is the one-time
  transition from the former all-purpose counter to this breaking-only epoch.
- **Banding, in part.** CR 702.22 gives banding two separate jobs, and the
  engine now does one: a creature with banding blocking an attacker moves the
  choice of how that attacker assigns its combat damage to the defending
  player. Attacking in a band is still absent -- bands are neither declared nor
  blocked as a group -- so the keyword reports itself `Partial` rather than
  complete, and the twenty-one identities that print it keep an audit line
  naming only what is left. No card is claimed as executable on the strength
  of half a keyword.
- **Five identities unblocked by earlier work in this release**, with no new
  machinery. Three pair unleash with something that already existed --
  regeneration, a counter-conditional trample grant, and a sacrifice ability
  reading its own last-known power. Two are Equipment whose lines still said
  equip was not declarative. Grim Roustabout, Chaos Imps, Hellhole Flailer,
  Accorder's Shield, and Fireshrieker; Mask of Avacyn and Rakdos Drake join
  them.
- **A turn-scoped redirection naming one source.** The static bodyguards
  redirect from a whole group; Shimian Night Stalker names a single attacker
  for the turn, so the rule carries both ends as object ids rather than a
  vocabulary word. A second attacker still gets through.
- **The M13 Rings.** All five were blocked on "Equipment attachment plus an
  upkeep bonus conditioned on the attached creature's color", and both halves
  had since been built -- equip, and the attached-permanent trigger condition.
  Ring of Evos Isle, Kalonia, Thune, Valkas, and Xathrid.
- **Bodyguards: damage aimed at a player that lands on a creature instead.**
  Redirection happens before anything else looks at the damage (CR 614.9), so
  the shields and preventions downstream all answer the creature that took it
  rather than the player it was aimed at. The source group is the same closed
  vocabulary the turn-long preventions use, now with artifacts and unblocked
  creatures, and the "as long as this creature is untapped" condition rides on
  the recipient. Veteran Bodyguard and Martyrs of Korlis.
- **Standing Stones.** Three cost kinds on one mana ability -- mana, a tap,
  and a life payment -- which the mana-cost work made expressible; its audit
  line had gone stale since.
- **Holding a permanent down while the source stays tapped.** The same
  deadline-free shape as the tapped-artifact stat bonus, applied to untapping:
  the source is recorded and the question asked afresh at each untap step, so
  the hold ends when the source untaps without anything being undone on the
  permanent it held. Phyrexian Gremlins.
- **A static bonus that counts, and then scales.** Static power/toughness
  could count matching objects but not multiply the count, so "+2/+2 for each
  Aura attached to it" had no form. The value walk now handles a scale over
  any value it already understood, and the runtime boundary allows exactly
  that. Rabid Wombat.
- **Targeting by what a permanent is attached to.** `Enchanted` asks the
  host's question -- is anything on it? -- and the new `AttachedTo` asks the
  Aura's: what is it on? Both are needed because an Aura and its host are
  both permanents, so a predicate that confused the two would still find
  something to destroy. Ramses Overdark, Miracle Worker, and Savaen Elves.
- **Meekstone.** Its prohibition is aimed by a live power reading rather than
  a list frozen when the artifact entered, so a creature pumped past two stays
  tapped and one shrunk below three untaps as usual. Every piece was already
  built; the audit line had gone stale.
- **Preventing every kind of damage one creature would deal.** The
  turn-scoped by-direction prevention covered combat damage only, so a card
  that stops a creature's damage outright had no form. Kry Shield silences an
  Orcish Artillery's ability as well as its attack; Subdue, whose audit line
  had gone stale, names combat and leaves the ability alone.
- **Turn-long prevention that names a group of sources, and a fog's window.**
  A relational prevention could cover a player and their creatures, or every
  source but one; it could not name a group. The group is a closed vocabulary
  rather than a predicate, because the rule outlives the resolution that made
  it and has to survive a checkpoint -- and it is re-read as each damage
  arrives, so an attacker that gains flying afterwards walks straight through
  Al-abara's Carpet. Also an activation window matching the one Berserk
  already used for casting. Al-abara's Carpet, Scarecrow, Angus Mackenzie.
- **Combat-damage prevention gains a direction and a source filter.** The
  static prevention covered damage to *and* by a permanent in one lump, and
  the source-filtered one covered all damage rather than combat damage. Both
  now have the narrower form the cards actually print: Demonic Torment stops
  only what its host deals, and its host still takes what its blocker deals
  back; Enchanted Being names combat, so a burn spell from the same enchanted
  creature still lands.
- **Two more identities from the tap/untap group.** Giant Tortoise reads its
  own tapped state through the recipient condition, and Elvish Hunter spends
  its tap to take one untap step away. Both audit lines had gone stale.
- **"For as long as this artifact remains tapped."** Every other resolving
  duration has a deadline to file the effect under; this one has none, since
  the artifact that tapped to make the bonus decides when it ends by
  untapping. So the source is recorded instead of a deadline and the bonus is
  read against it, which is what lets cleanup leave it alone while still
  dropping a bonus whose source has untapped. Ashnod's Battle Gear, Tawnos's
  Weaponry, Spirit Shield, and Zelyon Sword -- and Castle, which asks the same
  question from the other side, with the condition on the creature receiving
  the bonus.
- **Removing a creature from combat.** Regeneration already did exactly this
  as part of its shield, so the step is now shared rather than reimplemented:
  the permanent stops attacking or blocking and anything blocking it stops.
  An attacker removed this way was still blocked, so it deals no damage
  rather than getting through. Mijae Djinn.
- **Per-turn activation caps count rather than flag.** "Activate only once
  each turn" was a boolean, so "no more than twice each turn" had nowhere to
  go. The engine was already counting every activation per ability and
  clearing the counts each turn, so the cap is now a number read against those
  counts. Vampire Bats and Beetleform Mage.
- **Shroud granted for a while, and shroud granted on a condition.** Both
  shapes were already expressible and the audit lines saying otherwise had
  gone stale. Homarid Warrior buys its shroud with a tap and a skipped untap
  step; Spectral Cloak carries the condition on the recipient, so tapping the
  host takes the shroud away and untapping gives it back without the Aura
  being touched.
- **Lands animated by a static ability.** "All Forests are 1/1 creatures that
  are still lands" keeps applying as Forests come and go, so unlike a resolved
  animation it cannot be written onto the land; it is read live. That raises
  the usual live-read problem -- the effect changes characteristics, and it
  picks its permanents by characteristics -- so the stratification is narrow
  on purpose: a static animation may only add the creature type and stats, and
  may only be aimed by predicates that cannot read what it supplies. The
  runtime boundary holds cards to exactly those limits. Living Lands, Kormus
  Bell, and Living Plane.
- **Auras that watch their own host being tapped.** Every piece this needs --
  the becomes-tapped event, the attached-permanent relation, and the host
  controller as a recipient -- was already built, and the audit lines saying
  otherwise had gone stale. Psychic Venom, Blight, and Spirit Shackle, the
  last on a new -0/-2 counter that takes toughness without touching power.
  The three identities still blocked here now name what actually blocks them.
- **Prevention that names its source by relationship.** Damage prevention
  could name the source by its characteristics, but not by what it is to the
  permanent being protected. Two predicates fill that in: the attacker a
  permanent is blocking, read from the blocker because the attacker's own
  record does not name who stopped it, and a permanent with an Aura attached.
  Wall of Vapor, Wall of Shadows, and Wall of Putrid Flesh.
- **Mana abilities that cost mana.** A mana ability could tap, sacrifice, or
  exile its source, but not spend mana, so filters had nothing to filter with.
  The cost comes out of the pool and nowhere else: the ability is offered only
  while the pool already covers it, and the automatic payment planner leaves
  such an ability alone rather than counting production it has not paid for.
  Fire Sprites, Apprentice Wizard, Coal Golem, and Implements of Sacrifice.
- **Tapping a chosen permanent as a cost.** A cost could tap the ability's own
  source, or sacrifice a chosen permanent, but not tap one -- so "tap an
  untapped Gate you control" had no way to be paid. The candidate has to be
  untapped and cannot be the source, which is already tapping itself if asked.
  Gateway Shade and Crackling Perimeter.
- **Tokens that arrive tapped.** A search could put a permanent onto the
  battlefield tapped after the previous entry; token creation could not, so
  "create a tapped 2/2 black Zombie creature token" had nowhere to say so.
  Liliana's Reaver and Xathrid Necromancer.
- **"With a +1/+1 counter on it" as a predicate.** Counters could be counted
  and compared but not asked about while selecting which objects a continuous
  effect covers, so "each creature you control with a +1/+1 counter on it"
  could not be written. Read live, and safe to read live because a counter is
  permanent state rather than a characteristic that could feed back into the
  layer being computed. Sapphire Drake, Crowned Ceratok, and Exava.
- **Populate.** Choosing a permanent already existed; copying one did not, and
  populate only ever copies a token, whose copiable values are the definition
  it was made from -- so this is a narrow copy rather than the general one. A
  token that is itself a copy of something else copies what it became. Eyes in
  the Skies, Rootborn Defenses, and Growing Ranks.
- **"If you control the creature with the greatest power."** A tie counts, so
  the question is whether anything on the battlefield is strictly bigger
  rather than whether one creature stands alone, and an empty battlefield
  makes it false rather than vacuously true. Triumph of Cruelty and Triumph of
  Ferocity.
- **An Aura triggering on its host's death**, reading the dead creature's
  power. The trigger is the attached permanent's move from battlefield to
  graveyard, which the predicate vocabulary already described; the amount is
  new, and comes from last known information because the creature is gone by
  the time it resolves. Murder Investigation.
- **Morbid**, and with it entry replacements that read a condition. A
  replacement ability could carry one, but the entry path never looked at it,
  so "enters with two +1/+1 counters if a creature died this turn" had nowhere
  to put the "if". It is read as the permanent enters rather than when its
  spell was cast. Festerhide Boar and Somberwald Spider.
- **Additional casting costs that spend an object.** The only one in the
  corpus was Goblin Grenade's, hardcoded into the casting enumeration by card
  behavior. It is a declaration now: what may be spent, from which zone, and
  how many. A spell with nothing to spend is not offered at all, and each
  payable object is its own action rather than one blanket choice. Makeshift
  Mauler, Stitched Drake, Headless Skaab, and Relentless Skaabs; Altar's Reap
  sacrifices from the battlefield and Wild Guess discards from hand, which are
  the same declaration pointed at different zones.
- **Searching a permanent onto the battlefield tapped.** A search could put a
  permanent into play but always untapped, so a fetch land had no way to be
  written. The flag is set on the prospective permanent before entry
  replacements run, the same way an as-enters clause would. On the wire it is
  additive: a checkpoint written before it existed reconstructs as an untapped
  arrival. Dawntreader Elk and Evolving Wilds.
- **"Can block only ..."**, the blocker's own restriction on what it may
  block. The engine had the attacker's side of this and the blocker's flat
  prohibition, but nothing that narrowed a blocker to a subset. Stormbound
  Geist, Scrapskin Drake, and Gloomwidow.
- **"Power less than this creature's power"** as a blocking restriction. The
  mirror predicate for greater power already existed for evolve; this is the
  other direction, read live against the source so pumping the attacker widens
  the restriction mid-combat. Howlgeist and Wandering Wolf.
- **Clauses conditioned on what an Equipment is attached to**, which is what
  "as long as equipped creature is a Human" asks. It is read live rather than
  frozen when the Equipment moved, so the same Pitchfork gives its bonus on
  one creature and nothing on the next. Butcher's Cleaver, Sharpened
  Pitchfork, and Silver-Inlaid Dagger.
- **Equip**, and with it attachment as something an ability does rather than
  only something an Aura spell does. `EffectDef::Attach` was a no-op because
  an Aura attaches as it enters and nothing else needed it. Telling the two
  apart is the substance: an Aura is now recognised by attaching from a *spell*
  clause, and Equipment that loses its creature comes loose and stays on the
  battlefield instead of dying. Cobbled Wings and Skyblinder Staff, with ten
  more Equipment identities each blocked on one further thing.
- **Two exalted identities with no engine change**, Knight of Glory and Knight
  of Infamy. Their audit lines said exalted needed an attacks-alone event; it
  has had one since exalted was implemented.
- **Clauses that read a counter count.** Conditions could count objects, spells
  cast, loyalty, and activations, but not the counters on their own source, so
  "as long as there are exactly three tide counters on this creature" had no
  way to be written. Removing every counter of a kind at once came with it.
  Homarid, Icatian Moneychanger, and Merchant Ship.
- **Counters that change power and toughness**, rather than only +1/+1. The
  amounts now live on the counter kind, so a kind whose halves differ is
  ordinary rather than a special case, and CR 121.3 annihilation is a
  state-based action so a permanent never carries both +1/+1 and -1/-1. New
  kinds are appended, leaving the serialized counter positions of the existing
  ones alone, and a checkpoint written before a kind existed reconstructs with
  none of it. Unstable Mutation and Armor Thrull.
- **Three prevention identities with no engine change.** Healing Salve and
  Alabaster Potion offer prevention as one mode of a choose-one spell, and
  Rakalite pairs it with a delayed self-return; modes, "any target" shields,
  and delayed end-step effects all already worked. Their audit lines named the
  prevention effect, which five cards were already using.
- **"Whenever this creature attacks and isn't blocked."** Not answerable when
  attackers are declared -- only once blocking is done -- so it is captured
  where the blocking relationships are, alongside the pair events. Mindstab
  Thrull and Necrite, both of which offer to sacrifice themselves for the
  effect and so exercise the optional half in both directions.
- **A group-wide "can't attack".** The printed "can't attack unless ..." is
  something a creature says about itself and was read only from its own
  clauses; this one is applied from elsewhere, so one permanent can hold back
  whatever a query matches and stop doing so when it leaves. Akron Legionnaire
  and Evil Eye of Orms-by-Gore.
- **"Whenever this creature blocks or becomes blocked by ..."**, which is one
  printed clause covering both sides of a block rather than two. The event
  fires once per ordered pair, so a card reads the creature opposite it as the
  triggering object without knowing which of them attacked. "Destroy that
  creature at end of combat" came with it: end of combat is earlier than the
  end step an ordinary delayed destruction waits for. Cockatrice, Thicket
  Basilisk, Abomination, and Aisling Leprechaun -- who repaints instead of
  destroying, using the colour change added alongside the Laces.
- **Auras that trigger on their host's controller's upkeep.** "The upkeep of
  enchanted land's controller" is not the Aura's controller's upkeep, and the
  two come apart the moment the Aura sits on something an opponent controls --
  which is how every card in this family is played. Both the trigger's player
  relation and the effect's recipient now read the host, following the same
  resolve-at-the-source route the chosen-player relation already took.
  Feedback, Cursed Land, Warp Artifact, and Wanderlust.
- **Skipped untap steps**, the spent-as-they-arrive counterpart to the
  continuous "doesn't untap" prohibition. It is a count rather than a flag
  because Telekinesis names two, and the count is spent by its controller's
  own untap step, so an intervening turn by anyone else does not use it up.
  Telekinesis and Barl's Cage.
- **The Lace cycle**, and with it colour as something an effect can replace.
  "Becomes" is not "in addition to", and the Laces print no duration, so the
  new colour is permanent state on the permanent rather than an expiring
  continuous effect. Every colour question now goes through one funnel, so a
  repainted permanent answers the same way to protection, to Aura legality,
  and to blocking. A spell still on the stack can be repainted too. Purelace,
  Thoughtlace, Deathlace, Chaoslace, and Lifelace.
- **Poison counters**, a second way to lose that has nothing to do with life.
  Observations gain a `poison` pair alongside `life`, and `result.reason` gains
  `OpponentPoisoned`; both are additive, so a bot that ignores them or treats
  an unknown reason as "the game ended" is unaffected. Marsh Viper, Pit
  Scorpion, and Serpent Generator, whose Snake token carries the trigger the
  artifact only quotes.
- **The Ward cycle**, and the printed exception that lets an Aura survive its
  own effect. Protection already made an existing attachment illegal, so a
  white Aura granting protection from white fell off the moment it worked.
  "This effect doesn't remove this Aura" is now its own applied effect,
  exempting that one Aura rather than weakening the protection: another white
  Aura on the same creature still falls off. Black, Blue, Green, Red, and
  White Ward.
- **Scavenge**, and with it activated abilities that work from a graveyard.
  Nothing activated from that zone before, so this adds four things: the
  graveyard is enumerated for its owner's activations, one resolves through
  its own path, `ActivationTimingDef::SorcerySpeed` closes the window outside
  a controller's own main phase with an empty stack, and a card's power stays
  readable after it has left for exile -- which scavenge needs, because paying
  its cost exiles the very card whose power it counts. Eight Return to Ravnica
  identities. Sewer Shambler and Golgari Decoy stay blocked on swampwalk and
  on a forced-block requirement.
- **Two more detain identities**, with no engine change. Lyev Decree detains
  from a spell and New Prahv Guildmage from an activated ability, which are the
  two ways into the restriction the first pass did not use.
- **A printed static "can't be blocked"**, the attacker's side of the
  prohibition added with the blocker's. The turn-scoped form was a resolving
  rider; this one holds while its source does. Elusive Krasis and Soulsworn
  Spirit complete evolve and detain respectively.
- **Evolve**, and the two stat comparisons it is written against. A predicate
  could ask whether a toughness was below a computed value; the mirrors for
  greater power and greater toughness are what the keyword needs, read against
  the source's current stats so a creature stops evolving from arrivals it has
  outgrown. Five Gatecrash identities are now executable.
- **Detain**, the last of Return to Ravnica's four keywords. It is one effect
  rather than three: the restrictions always travel together and end at one
  moment, the detaining player's next turn, which is recorded the way floating
  triggers already record "until your next turn". Seven identities are now
  executable.
- **Unleash**, which needed an entry replacement its controller may decline.
  The entry path had no optional handling at all -- the only optional
  replacement anywhere was on beginning a turn. The suspended decision records
  the ability rather than the effect, so it stays checkpointable without
  teaching the snapshot about effect bodies, and the effect is re-read when the
  answer arrives. Eight Return to Ravnica identities are now executable, and
  the blocking half falls out of the prohibition added alongside.
- **A blocker-side "can't block" prohibition.** The vocabulary had only the
  attacker's side, so "this creature can't block" and "target creature can't
  block this turn" had no shape at all. It exists in both forms: a printed
  static read from the continuous layer, and a resolving rider recorded on the
  permanent until cleanup. Sightless Ghoul, Markov Warlord, Vampire Interloper,
  Crossway Vampire, Nightbird's Clutches, and Firefist Striker are now
  executable, the last completing battalion.
- **Checkpoint reconstruction v3 consolidates the complete migration from
  v2.** Suspended declarative effects separate event context from typed object
  bindings, and shared choice, mana-or-life payment, scalar, top-card, and pile
  continuations retain every value needed to resume. Disclosed hidden-zone
  cards carry exact seat, zone, and index provenance. Resolved characteristic
  and object rules are one ordered, expiration-aware continuous-effect
  collection with authored locators, source provenance, component order, and
  frozen values; player play restrictions use a parallel collection. Damage
  prevention is one ordered typed collection, installed triggers retain full
  lexical context, and entry replacements use typed replacement-program
  locators. Inserted combat and postcombat-main phases form an ordered queue
  with a frozen ordinary continuation. These structures replace the lossy
  aggregate permanent fields, fragmented prevention state, delayed/floating
  trigger split, scalar additional-combat counter, and procedure-specific
  continuation tags. Format-2 checkpoints must be regenerated; the dedicated
  capability is `reconstruction.checkpoint.v3`, while the bot protocol epoch
  remains 22.
- **Battalion**, sharing exalted's event. The attack-declaration event now
  carries how many creatures attacked, and each keyword states the range it
  wants: exactly one for exalted, three or more for battalion. Eight Gatecrash
  identities are now executable. Their audit line said declarative trigger
  conditions are rechecked on resolution as intervening-if conditions, which is
  true and is why the count belongs in the event rather than in a condition.
- **Exalted**, and the attacks-alone event it is written against. The event is
  decided by the attack declaration as a whole, so it is captured there and
  carries the lone attacker as its triggering object. Exalted is a keyword that
  is defined as a triggered ability, which matters twice: several instances
  each trigger, and the permanent carrying it need not be a creature -- one of
  the six is a land. Angelic Benediction, Aven Squire, Guardians of Akrasa,
  Duskmantle Prowler, Servant of Nefarox, and Cathedral of War are now
  executable.
- **Two Premodern regeneration cards**, with no engine change. Vampire Warlord
  sacrifices another creature -- a cost that has to refuse the source itself --
  and Trollhide grants its host the regeneration ability rather than carrying
  it. Both patterns were already in use in Old School.
- **Printed coin flips**, with no engine change. The seeded randomiser and its
  two branches have existed since Chaos Orb used them; a coin is that with an
  even chance. Orcish Captain and Bottle of Suleiman are now executable, along
  with the Djinn token the Bottle makes.
- **Feldon's Cane**, with no engine change. Its audit line asked for a
  zone-object query, and the query vocabulary has reached graveyards, hands,
  and exile for a while -- the effect is the composition the shuffle operation
  documents: move the cards, then shuffle the library they arrived in.
- **A "whenever this deals damage" trigger**, carrying the amount. The existing
  damage trigger only ever matched damage arriving at the ability's own source,
  whatever predicate it named, so the other direction had no event at all.
  `ObjectPredicateDef::AttachedToSource` lets an Aura watch its host rather
  than itself. El-Hajjâj and Spirit Link are now executable, and Spirit Link is
  not lifelink: the life goes to the Aura's controller, not the creature's.
- **"Can't be regenerated" as a standalone effect**, rather than only a
  property of a destroy. CR 701.19c draws a distinction the implementation
  keeps: a shield is not removed and regeneration effects can still create
  one, but the shield cannot apply while the prohibition holds. Hurr Jackal is
  now executable.
  Elves of Deep Shadow joins it with no engine change -- its audit line blamed
  the mana runtime for an ability whose cost has no mana in it, and Ancient
  Tomb has printed the same damage-to-controller mana clause for a while.
- **An optional untap, and a control change held by staying tapped.** Rubinia
  Soulsinger and Willow Satyr pair the two, and each half is useless alone:
  untapping would hand the creature straight back, and without the choice the
  untap step would do it every turn. Untapping stays mandatory for everything
  that does not print otherwise.
- **Control changes that outlive the turn.** The engine could only take control
  until cleanup. `EffectDef::GainControl` now carries a typed
  `ControlDurationDef`; its source-bound form lasts as long as the permanent
  holding it stays on the battlefield under the same controller, and ends the
  moment either stops being true. Aladdin and Thrull Champion are now
  executable. The holder is an additive checkpoint member.
- **A `Blocking` object predicate**, the other half of "attacking or
  blocking", which neither single-sided predicate could express. Tetsuo
  Umezawa needs it and is now executable; People of the Woods joins the
  counted bodies with its toughness alone.
- **Creatures whose printed body is a battlefield count**, declared the way the
  token vocabulary already declared them: a zero-or-one body plus a static
  counted bonus. Plague Rats, Keldon Warlord, Gaea's Avenger, and Dakkon
  Blackblade are executable and `partial` -- a real characteristic-defining
  ability sets power and toughness in every zone, and this is a
  battlefield-only continuous effect. Nightmare stays blocked: its M14 printing
  already carried an audit line saying exactly that, and overruling it for one
  more card would have been the wrong trade.
- **Two identities the activation window unblocked on its own.** Colossus of
  Sardia pairs a static untap restriction with an upkeep-only untap, and
  Hell's Caretaker trades a creature for one in its graveyard during its own
  upkeep. Neither needed anything beyond the window; both had audit lines
  naming other clusters.
- **Printed "only once each turn" caps.** An activated ability can carry a
  per-turn cap, and it needed no new state: the engine already counted every
  activation per ability and cleared the counts each turn, so the cap reads
  what was already there and already in the checkpoint. Gate to Phyrexia, Fire
  Drake, and Darkthicket Wolf are now executable, and Gate to Phyrexia carries
  both a window and a cap.
- **Printed "Activate only during ..." windows.** An activated ability can now
  carry an activation window, checked where activations are enumerated, so a
  restricted ability is simply not offered outside it. Twenty identities name
  such a clause; four needed nothing else and are now executable: Disrupting
  Scepter, Dwarven Weaponsmith, Svyelunite Priest, and Gwendlyn Di Corci.
  Three of the four had audit lines blaming capabilities that already existed
  -- a hidden-zone discard, seeded random selection, and executable shroud --
  with the window as the only real gap.
- **Two identities the spore pass left behind.** Elvish Farmer and Thallid
  Devourer print the same two spore clauses the Thallid cycle already has, plus
  a third that spends a Saproling as an activation cost -- a use for the token
  those clauses make that the original cycle never had. The predicate-matched
  sacrifice cost and both payoffs already existed, so neither needed engine
  work.
- **Shields that stop part of a hit, or pay a rider when they fire.** A shield
  now carries how much of a covered hit it stops and whether spending it gains
  its recipient that much life. Dark Sphere stops half, rounded down, so an odd
  point still lands and a single point is not reduced at all. Reverse Damage
  gains exactly what it prevented rather than what was aimed. Both are additive
  checkpoint members skipped when false.
- **A prevention shield keyed to a chosen source**, which is what a Circle of
  Protection is. The existing shields attach to a recipient and spend on the
  next damage from anything; a Circle names one source and answers that source
  alone, preventing all of the first damage it deals and then being gone.
  `EffectDef::ChooseDamageSource` makes the choice as the ability resolves and
  searches the stack as well as the battlefield, because a Circle that could
  not name a burn spell would be the wrong card. The checkpoint carries the
  named source as an additive member. Circle of Protection: Blue, Green, Red,
  White, Black, and Artifacts are now executable, as is Greater Realm of
  Preservation.
- **A continuous combat-damage prevention.** Static and resolving prevention
  use the same typed damage-event matchers while retaining their different
  lifetimes. `AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(...))` is
  asked afresh each time combat damage is dealt, so it holds while an Aura is
  attached and stops the moment it is not. Gaseous Form is now executable.
- **Seven identities behind a prevention line that had already been built.**
  Sixty-six identities cited "a duration-scoped replacement/prevention effect"
  after the prevention shields landed. Conservator, Oasis, Argivian Blacksmith,
  Kei Takahashi, Lady Evangela, Horn of Deafening, and Combat Medic needed no
  engine work: a shield aimed at a player and prevention of only the combat
  damage a creature deals were both already there. Twelve more -- the Circles
  of Protection and their relatives -- keep an audit line rewritten to name the
  gap that is real, a shield keyed to a source chosen as the ability resolves
  rather than to a recipient.
- **Two more identities behind a landwalk line that was already false**, and
  ten audit lines rewritten to name their real gap. Twelve identities cited
  "the printed landwalk variant and its defending-player land/blocking
  semantics" after landwalk landed; three of them -- Land's Edge, The Fallen,
  and Eternal Flame -- print no landwalk text at all. Wormwood Treefolk and
  Merfolk Assassin are both complete.
- **Seven identities whose audit lines outlived their gap.** Regeneration and
  rampage both landed as primitives, but eleven audit lines still named them,
  and seven of those identities turned out to need no engine work at all:
  Horror of Horrors, Chromium, Goblin Chirurgeon, Manor Skeleton, Marrow Bats,
  Necrobite, and Wolfir Avenger are now executable. The four that stay blocked
  have rewritten lines naming their real gap -- a random discard cost, a
  conditional grant, a four-way random ability choice, and a granted
  counter-consuming ability -- rather than a primitive that already exists.
- **Keyword predicates read the keywords a static effect grants or removes.**
  A creature wearing a Lord of Atlantis grant used to be unblockable across an
  Island and untargetable by "target creature with islandwalk", because the
  bitmask behind `HasKeyword` deliberately omitted live static effects while the
  blocking rules did not. It omitted them to terminate: a static ability is
  matched against the characteristics of the permanent it might apply to, so
  reading the ability set back inside that match calls itself. The two readers
  are now one. Gathering the layer-6 slice is stratified instead of truncated --
  the first caller owns the pass and any query raised underneath it sees the
  printed, copied, and already-resolved abilities alone -- so everything outside
  that walk gets the complete set: target legality, blocking, trigger matching,
  resolution-time recipients, and static power and toughness effects keyed on a
  keyword. Nineteen identities across ten sets drop the coverage line they
  carried for this, among them Earthquake, Hurricane, Elven Riders, Flood,
  Merfolk Assassin, Grapeshot Catapult, Doorkeeper, One-Eyed Scarecrow, Air
  Servant, and Windstorm. What is left is one level down and pinned by its own
  test: a static ability that grants or removes abilities still picks its
  recipients from the layer below itself, so it cannot see a keyword another
  static ability hands out. That is the CR 613.8 dependency case, and it needs
  the fixed-point evaluator rather than a deeper recursion.
- **The ten identities that stratification left implementable.** Every one of
  them asks "does this creature have flying?" from a place that used to answer
  from printed abilities alone, and none needed anything else. Favorable Winds
  and Smog Elemental are static power and toughness effects keyed on the
  keyword; Bower Passage and Spire Tracer are blocking restrictions; Plummet,
  Thunderbolt, and Clan Defiance are target predicates; Magmaquake and Silklash
  Spider are damage sweeps; Mwonvuli Beast Tracker searches a library by four
  keywords at once. Bower Passage is the one that widened anything: a blocking
  restriction is read off the ordinary static-effect walk over the attacker, so
  a group recipient always worked, and only the declaration boundary had to
  admit it. Clan Defiance is the second "choose one or more" card, and
  Thunderbolt the second modal spell at all. The audit lines all named this one
  gap and are gone.

- **A tapped object predicate**, which was the whole of Royal Assassin's gap.
  Island Fish Jasconius joins it from the same sweep with no engine change:
  the untap restriction it needed already existed, and its other three clauses
  are the attack restriction, the optional upkeep payment, and the
  state-condition sacrifice, all of which were already there.

- **A sweep for stale audit lines**, prompted by two turning up incidentally.
  Regeneration takes any recipient, so "regenerate target creature" and
  "regenerate enchanted creature" have been expressible since the effect
  existed; eight identities were blocked on their audit line rather than on a
  capability. Death Ward, Regeneration, The Brute, Elephant Graveyard, Niall
  Silvain, Ragnar, Thrull Retainer, and Zombie Master are now executable, with
  no engine change at all. Zombie Master grants both a landwalk and a
  regeneration ability to other Zombies, so it exercises two of this series'
  primitives through a third path -- a granted activated ability.

- **Legendary landwalk**, which reads a land supertype rather than a basic
  land type and so needs its own keyword rather than a new `BasicLandType`.
  Livonya Silone is now executable. Argothian Treefolk joins it: its clause
  was expressible all along with the existing static source-filtered
  prevention, and its audit line was simply stale.

- **Spore counters.** A `CounterKind::Spore` was the only thing the Fallen
  Empires Thallid cycle was missing; the upkeep trigger, the
  remove-three-counters cost, and the token creation all existed. Thallid,
  Thorn Thallid, Feral Thallid, Spore Flower, and Fungal Bloom are now
  executable, and two of them spend their counters on capabilities added
  earlier in this series -- regeneration and turn-wide combat prevention.

- **A Fog prevents all combat damage for the turn.** Combat-damage prevention
  existed as a property of a permanent, which is enough for a Maze of Ith and
  not for a Fog: the spell has no permanent to attach to, and it has to cover
  creatures that were not on the battlefield when it resolved.
  `EffectDef::PreventAllCombatDamageThisTurn` is game state that lives until
  cleanup, and the checkpoint carries it as an additive member so a
  reconstruction mid-Fog is faithful. Fog, Holy Day, and Darkness are now
  executable, and Fog's M13 printing joins the definition it was blocked
  against. `EffectDef::PreventNextDamage` and `EffectDef::PreventAllDamageThisTurn`
  add the general shield: a promise attached to a recipient that waits for
  damage, is spent as the damage it covers arrives, and is discarded at
  cleanup. Prevented damage is never dealt, so nothing watching for damage
  sees it. Samite Healer, Indestructible Aura, and Amulet of Kroog follow.

- **Attack restrictions.** `EffectDef::CannotAttackUnless` is a static clause
  read while attackers are declared, carrying an ordinary object query rather
  than a card-specific rule, so "unless defending player controls an Island" is
  an opponent-relative battlefield query and reads effective land types. Four
  Old School identities are now executable: Dandân, Vodalian Knights, Pirate
  Ship, and Sea Serpent. Their "when you control no Islands, sacrifice this
  creature" clause needed nothing new; the state-condition trigger and object
  count already expressed it.

- **Rampage, and the becomes-blocked event under it.** The engine had no
  trigger for a creature becoming blocked, so none of the nine printed rampage
  cards could be expressed. `TriggerEventDef::BecomesBlocked` fires once per
  attacker when blockers are finalized and carries the blocker count beyond the
  first, which is the quantity every rampage clause is written against.
  `ValueDef::Scaled` multiplies a value by a constant so the printed amounts 1,
  2, and 3 all read the same event. Six Legends identities are now executable:
  Aerathi Berserker, Frost Giant, Craw Giant, Wolverine Pack, Hunding
  Gjornersen, and Marhault Elsdragon. Chromium, Gabriel Angelfire, and Rapid
  Fire print rampage alongside capabilities this does not add and keep their
  audit lines.

- **Landwalk is one keyword parameterized by land type.** CR 702.14 is a single
  rule, but the engine carried Mountainwalk and Forestwalk as separate keywords
  with the blocking check spelled out once per variant, so the other three could
  not be printed at all. `KeywordAbility::Landwalk(BasicLandType)` replaces both,
  the blocking rule is stated once for all five types, and a creature may carry
  several. Six Old School identities blocked only on this are now executable:
  Bog Wraith, Righteous Avengers, Devouring Deep, Segovian Leviathan, Lost Soul,
  Marsh Goblins, Lord of Atlantis, and Fishliver Oil. The last two grant the
  walk rather than printing it, from a lord clause and from an Aura. The Aura
  spell clause and its enchant targets moved from the Alpha module into
  `card::abilities`, where every set can reach them.
  `EffectDef::LandwalkCanBeBlocked` turns one walk off for blocking without
  removing the keyword, which is the Legends answer cycle: Great Wall,
  Undertow, Quagmire, Crevasse, Deadfall, Gosta Dirk, Lord Magnus, and
  Ur-Drago. Checkpoint keyword tags keep their printed names and gain
  `plainswalk`, `islandwalk`, and `swampwalk` additively.

- **Regeneration is a declarative effect.** Arming a regeneration shield is now
  an ordinary `EffectDef`, so a printed "{cost}: Regenerate this creature" is an
  activated ability like any other rather than an engine-level card branch. The
  shield machinery and its destroy-event replacement already existed; what was
  missing was a way for a card to reach them. Sedge Troll's clause moved off its
  card-identity escape valve onto the shared path, and eleven Old School
  identities that were blocked only on this are now executable: Drudge
  Skeletons, Wall of Bone, Will-o'-the-Wisp, Uthden Troll, Wall of Brambles,
  Living Wall, Clay Statue, Drowned, Ghost Ship, Diabolic Machine, and Walking
  Dead. New cards move the simulation fingerprint, not the protocol epoch.
- **The remaining shared regeneration and turn-scoped prevention forms.** Aura
  activations now retain their formerly enchanted permanent through source
  last-known information, including when the Aura is sacrificed as the cost.
  Regeneration no longer erases damage-source history, and the shared
  turn-scoped prohibition now also stops a shield from applying to lethal
  damage without preventing its creation or consuming it. Dynamic prevention
  rules cover a player and creatures they control, including later entrants,
  or every combat damage source except one chosen creature. Fifteen audited
  ISD–RTR Standard identities become executable, while Blessing, Holy Armor,
  Firebreathing, and Axelrod Gunnarson lose their final partial gap. The new
  relational checkpoint-v2 member is additive and defaults empty, so neither
  the checkpoint format nor the protocol epoch moves.

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
  School 93/94 card identities and five supporting tokens. At that protocol
  transition, the Old School pool exposed 421 legal identities: 389 complete,
  30 partial, and two
  metadata-only. An identity-complete audit, kept inline at each identity's
  collector position in the printed set modules, named the concrete engine gap
  for those 32 cataloged incomplete cards and the other 560 legal identities
  that were blocked, as well as all seven banned identities in those sets.
  Definition IDs remain append-only and the catalog JSON shape is unchanged,
  so this is compatible protocol-19 catalog growth.

- The catalog appends definitions 607 through 1361: 736 card identities used
  by ISD–RTR Standard and nineteen supporting tokens. Together with in-format
  printings of existing definitions, Standard then exposed 878 legal
  identities: 839 complete and 39 partial. Its identity-complete audit
  covered all 1,686 identities from Innistrad through Magic 2014 and kept a
  concrete capability gap inline at the collector position of every one of the
  847 incomplete identities, including the 808 that were blocked. Definition
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

- The Python binding and repository tooling now require Python 3.13 or newer.
  Its stable-ABI extension targets `abi3-py313`, and CI exercises Python 3.13
  as the supported floor rather than retaining compatibility with EOL releases.

- Catalog coverage tests now enforce structural invariants without pinning
  mutable repository totals. `make catalog-report` derives current catalog and
  implementation-coverage counts when a snapshot is useful, so ordinary card
  additions no longer require synchronized count edits across tests and docs.

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
  replay-stable seeded RNG and a reusable resolution-time `Choose(ChooseDef)`
  operation. Its typed object or object-set binding is deliberately distinct
  from a target and never passes through target legality or fizzle machinery.

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
  into the executable catalog using the same search procedure. Ring retains
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
- Affected-player discard remains one `EffectDef::Discard` operation whose
  `DiscardSelectionDef` is `RecipientChooses` or `Random`. A separate
  `EffectDef::DiscardCards` rules action now moves specific card objects that a
  preceding generic `Choose` bound, without asking their owner to choose again;
  Duress uses that composition. Protocol JSON and legal actions are unchanged.
- Optional and unless-paid branches now share `EffectDef::PayOr` and the same
  `EffectPaymentDef` in ordinary and replacement programs. Its payer is a
  `PlayerSetDef` that must select at most one player, and its explicit cost is
  fixed mana, dynamically evaluated generic mana, or life. The ambiguous
  `PaymentDef` list of general cost atoms has been removed; protocol JSON and
  legal actions are unchanged.
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
