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

## 0.7.0 — protocol 24

This release reports engine 0.7.0 and protocol 24. The simulation fingerprint
distinguishes snapshots of the covered source and build inputs.

### Changed

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

### Added

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
