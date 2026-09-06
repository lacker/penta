/// Declarative effect primitives interpreted by the rules engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDef {
    AddCounters {
        object: EffectRecipientDef,
        kind: CounterKind,
        amount: ValueDef,
    },
    /// Choose a counter kind on the recipient and bind it for the nested effect.
    ChooseCounterKind {
        object: EffectRecipientDef,
        then: &'static EffectDef,
    },
    /// Add or remove a resolved amount of one fixed or previously chosen counter kind.
    ModifyCounters {
        object: EffectRecipientDef,
        kind: CounterKindDef,
        operation: CounterOperationDef,
        amount: ValueDef,
    },
    /// Let the named player choose one labelled effect branch on resolution.
    ChooseEffect {
        player: EffectRecipientDef,
        choices: &'static [EffectChoiceDef],
    },
    AddMana(AddManaEffectDef),
    /// Adds a value's amount of one color outside a mana ability.
    AddManaEqualTo {
        color: ManaColor,
        amount: ValueDef,
    },
    /// "{cost}: Level N." Adds counters until the Class reaches that level;
    /// reaching it raises the matching event exactly once (CR 717.3).
    GainClassLevel {
        level: u8,
    },
    /// Adds player counters; their kind supplies poison or energy semantics.
    AddPlayerCounters {
        recipient: EffectRecipientDef,
        kind: CounterKind,
        amount: ValueDef,
    },
    Apply {
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: ResolvedEffectDurationDef,
    },
    /// Scopes a rules modifier to the nested effect (CR 608.2c).
    WithRule {
        rule: AppliedRuleDef,
        effect: &'static EffectDef,
    },
    /// An Aura spell attaching itself to what it enchants. The permanent the
    /// spell becomes is what attaches, so this is only meaningful on the spell
    /// clause of an Aura.
    Attach { object: EffectRecipientDef },
    /// The mirror of [`Self::Attach`]: the named permanent moves onto this
    /// ability's own source, which is what "attach it to this creature" says.
    AttachToSource { object: EffectRecipientDef },
    /// Soulbond's pairing. The chosen creature and the ability's source
    /// record each other; the pair is symmetric and survives until one of
    /// them stops being a creature its controller controls.
    PairWithSource { object: EffectRecipientDef },
    /// Reconfigure's paired attach/unattach procedure. A selected creature
    /// becomes the new host; selecting none ends this attachment incarnation.
    Reconfigure { object: EffectRecipientDef },
    /// Detach the named Equipment or Fortification without moving it. This is
    /// a rules action rather than a zone change: Elbrus does it immediately
    /// before transforming, while the host and both objects remain otherwise
    /// unchanged.
    Unattach { object: EffectRecipientDef },
    /// Phase the recipient out. It is treated as though it does not exist
    /// until it phases in, which happens before its controller untaps during
    /// their next untap step (CR 702.25). Phasing is not a zone change:
    /// nothing enters or leaves, and the permanent keeps everything it had.
    PhaseOut {
        object: EffectRecipientDef,
    },
    /// Replaces one permanent's copiable values with another object's.
    BecomeCopyOf {
        /// What is copied.
        object: EffectRecipientDef,
        /// What becomes the copy. `None` is the source itself, which is what
        /// "this land becomes a copy of target land" means; a clause that
        /// points at something else names it here.
        copier: Option<EffectRecipientDef>,
        /// Everything the copy has or replaces as part of the copy process.
        /// An empty definition is a plain copy; additions may include a
        /// reference to this very ability.
        exceptions: CopyExceptionsDef,
        /// How long the copy lasts. A copy with no stated duration is
        /// indefinite, which is what almost every printed one is.
        duration: Option<ResolvedEffectDurationDef>,
    },
    /// A static attack restriction: this creature cannot be declared as an
    /// attacker unless the query matches. The query carries its own controller
    /// relation, so "unless defending player controls an Island" is an
    /// opponent-relative battlefield query rather than a special case.
    CannotAttackUnless(&'static ObjectQueryDef),
    /// The mirror of [`Self::CannotAttackUnless`]: the source cannot attack
    /// while anything matches. The negation is over the existential rather
    /// than the object, which is why it is its own clause rather than a
    /// negated query.
    CannotAttackIf(&'static ObjectQueryDef),
    /// A static prohibition: no spell or ability an opponent controls can
    /// make this ability's controller sacrifice a permanent.
    CannotBeForcedToSacrifice,
    /// The same prohibition over discarding: no spell or ability an opponent
    /// controls can make this ability's controller discard a card. A discard
    /// they choose to make themselves, as a cost of their own spell, is not
    /// something anyone caused them to do.
    CannotBeForcedToDiscard,
    /// On resolution, choose two different basic land-type words and apply
    /// the resulting indefinite, noncopiable text change to the object.
    /// Ask a player to name a colour, then apply the named operation to the
    /// recipients in that colour. The choice is made as the effect resolves,
    /// so it cannot be a fixed colour in the declaration.
    ///
    /// The recipients are resolved before the question is asked -- targets
    /// are already chosen by then, and a group is whatever it is at that
    /// moment -- so the decision only has to carry the answer.
    ChooseColor {
        object: EffectRecipientDef,
        operation: ColorChoiceOperationDef,
        duration: ResolvedEffectDurationDef,
    },
    ChangeTextBasicLandType {
        object: EffectRecipientDef,
    },
    /// Choose two basic land types, and make every land of the first the
    /// second until end of turn. Both types are chosen as this resolves, so
    /// neither the lands it affects nor the type it gives them can be
    /// written down in advance.
    SubstituteBasicLandTypeUntilEndOfTurn {
        chooser: PlayerRefDef,
    },
    Choose(ChooseDef),
    /// The computed-cardinality counterpart of [`Self::Choose`].
    ChooseExact(super::ChooseExactDef),
    /// Inspect one collection, make one bounded card choice from it, and
    /// continue with bindings for the chosen cards and everything else.
    ChooseCardsFromCollection(super::ChooseCardsFromCollectionDef),
    /// Show a collection to one player, then continue after that player
    /// acknowledges it.
    LookAtObjects(super::LookAtObjectsDef),
    /// Arrange every member of a frozen collection, preserving the submitted
    /// order in a new binding for a later effect.
    ChooseObjectOrder(super::ChooseObjectOrderDef),
    /// Choose owned cards from one or more places without performing the
    /// keyword action "search." Ring of Ma'rûf uses this for outside-game
    /// cards, and Old School expands the same choice to exile.
    ChooseCards {
        player: EffectRecipientDef,
        sources: &'static [CardChoiceSourceDef],
        object: ObjectPredicateDef,
        minimum: usize,
        maximum: usize,
        reveal: bool,
        destination: ZoneKind,
        placement: ZonePlacement,
    },
    /// Return a spell from the stack to its owner's hand. Not a counter: the
    /// spell is never countered, so "can't be countered" does not stop this
    /// and nothing watching for a countered spell sees one.
    /// "Its owner puts it on their choice of the top or bottom of their
    /// library." The end is chosen by the spell's owner rather than by
    /// whoever is resolving, and nothing is countered -- a spell that cannot
    /// be countered goes there all the same.
    PutSpellIntoOwnersLibrary {
        object: EffectRecipientDef,
    },
    /// Counter a spell and put its card into `zone`: a graveyard ordinarily,
    /// exile for Dissipate, a library's top for Memory Lapse.
    Counter {
        object: EffectRecipientDef,
        zone: ZoneKind,
        /// Which end of a library the card lands on. Meaningless for every
        /// other destination, the same way it is for a plain move.
        placement: ZonePlacement,
    },
    /// Classify a frozen collection by a predicate without asking a player.
    ClassifyObjects(super::ClassifyObjectsDef),
    /// Reveal a materialized collection and classify all of it in one
    /// mandatory instruction.
    RevealAndClassifyCards(super::RevealAndClassifyCardsDef),
    /// Concatenate previously bound collections in authored order.
    CombineObjects(super::CombineObjectsDef),
    /// Declare one string-labeled object-set binding and resolve an effect
    /// whose output may populate it. The declaration itself creates the empty
    /// set, so a conditional effect that produces nothing is distinct from a
    /// reference to a label that was never declared. Keep `binding` first at
    /// construction sites so its declaration lexically precedes every use in
    /// the producer's continuation.
    BindOutput {
        binding: Binding,
        effect: &'static EffectDef,
    },
    /// Choose one distinct member for each of several predicates.
    ChooseOneOfEach(super::ChooseOneOfEachDef),
    /// Choose between two previously bound groups.
    ChooseGroup(super::ChooseGroupDef),
    /// Picks matching cards from a player's zone with the recorded RNG.
    /// Nothing moves. Wrap this in [`Self::BindOutput`] when later steps need
    /// the selected objects.
    SelectAtRandomFromZone {
        player: EffectRecipientDef,
        source: ZoneKind,
        object: ObjectPredicateDef,
        amount: ValueDef,
    },
    /// Names a card while this effect resolves and continues. Wrap it in
    /// `BindOutput` so the follow-up can read the chosen name from an explicit
    /// binding.
    ChooseCardName {
        chooser: PlayerRefDef,
        names: CardNameSetDef,
    },
    /// Copies one or more named spells or abilities on the stack.
    CopyStackObject(&'static CopyStackObjectDef),
    /// Changes the locked targets of one spell or ability on the stack.
    ChangeStackTargets(&'static ChangeStackTargetsDef),
    /// Gives its controller an emblem, an object that sits outside every
    /// zone and does nothing but carry its abilities.
    CreateEmblem {
        emblem: super::EmblemCharacteristics,
    },
    /// Creates a duration-scoped rules object outside every zone. It is not a
    /// permanent, but its activated ability uses the ordinary action, cost,
    /// stack, response, and resolution machinery.
    CreateOngoingEffect(OngoingEffectDef),
    /// Creates tokens with `token`'s complete authored characteristics under
    /// the resolving object's controller.
    CreateToken {
        token: TokenCharacteristics,
        /// When present, the authored token shell is replaced by the named
        /// object's copiable values before it enters the battlefield.
        copy: Option<&'static TokenCopyDef>,
        /// Who the tokens arrive under. `None` is the resolving object's own
        /// controller, which is what "create a token" means; a clause that
        /// hands them to somebody else -- "its controller creates two Map
        /// tokens" -- names that player instead.
        controller: Option<PlayerRefDef>,
        count: ValueDef,
        /// Whether the created token arrives tapped.
        tapped: bool,
        /// Whether it also arrives attacking. It was never declared as an
        /// attacker, so nothing that watches a declaration sees it, but it
        /// is an attacking creature in every other respect.
        attacking: bool,
        /// Counters the token arrives carrying. Incubate makes a token with
        /// X +1/+1 counters on it, where X is what the effect worked out
        /// rather than anything the token prints: the same Incubator token
        /// comes out of every Incubate. `None` for the ordinary token, which
        /// arrives with nothing on it.
        counters: Option<TokenCountersDef>,
        /// What to do with the tokens this made, when a following clause has
        /// to name exactly them. Mobilize sacrifices the tokens it created
        /// and no others, and by the time the delayed clause fires nothing
        /// about the board could tell them apart.
        created: Option<CreatedTokensDef>,
    },
    /// Creates one token and attaches it in whichever direction the clause
    /// says: `host` is what the token goes onto, which is how a Role is made,
    /// and `None` is living weapon, where the permanent goes onto the token.
    CreateAttachedToken {
        token: TokenCharacteristics,
        host: Option<EffectRecipientDef>,
    },
    CreateMyriadTokens, // Exact no-op in two-player games: there is no other opponent.
    /// Endure N (CR 702.183a): put N +1/+1 counters on the object, or create
    /// an N/N white Spirit creature token. Its controller chooses, as the
    /// ability resolves.
    ///
    /// A procedure of its own rather than a composition, for the same reason
    /// explore is one: the branch is a choice between two whole effects, and
    /// nothing else in the vocabulary offers one. The token is not authored
    /// beside the number because the keyword fixes it -- an N/N white
    /// Spirit, whatever N turns out to be.
    Endure {
        object: EffectRecipientDef,
        amount: ValueDef,
    },
    DealDamage {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// One simultaneous damage event evaluated from a shared pre-damage state.
    DealDamageSimultaneously(&'static [DamageAssignmentDef]),
    /// Deals damage using an explicitly named source rather than the
    /// resolving spell or ability's ordinary source.
    ///
    /// The reference is resolved as an object identity and may name a
    /// permanent that paid a sacrifice cost. Damage attribution then reads
    /// that identity through last-known information.
    DealDamageFrom {
        source: ObjectRefDef,
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Deals damage exactly as [`Self::DealDamage`] does, then applies
    /// `applied` for `duration` to each object that actually took damage.
    ///
    /// The two sets are not the same. Prevention, redirection, and a
    /// recipient that has already left the battlefield all mean a chosen
    /// recipient can end up dealt nothing, and a rider that reads "a creature
    /// dealt damage this way" must not touch it when that happens. That
    /// linkage is why this is one effect rather than a sequence of two: an
    /// ordinary sequence would have to name the recipients again, and naming
    /// them is precisely what loses the distinction.
    DealDamageAndApply {
        recipient: EffectRecipientDef,
        amount: ValueDef,
        applied: AppliedEffectDef,
        duration: ResolvedEffectDurationDef,
    },
    /// Two creatures simultaneously deal noncombat damage equal to their powers.
    Fight {
        first: ObjectRefDef,
        second: ObjectRefDef,
        excess: Option<FightExcessDef>,
    },
    Destroy {
        object: EffectRecipientDef,
        /// Binds only recipients actually put into a graveyard, then continues.
        /// Indestructible, regenerated, and zone-change-replaced permanents are absent.
        then: Option<DestroyFollowUpDef>,
    },
    /// Until the resolving controller's next turn, the permanent cannot
    /// attack, block, or activate its activated abilities.
    Detain {
        object: EffectRecipientDef,
    },
    /// Each recipient discards that many cards selected in the specified way.
    /// A player holding fewer cards discards their whole hand.
    Discard {
        recipient: EffectRecipientDef,
        amount: ValueDef,
        selection: DiscardSelectionDef,
        /// What to do once the cards are gone, and what to count among them.
        /// "You gain 3 life for each land card discarded this way" reads the
        /// discard's own result, which is not knowable before the player has
        /// chosen.
        then: Option<DiscardFollowUpDef>,
    },
    /// Discard the named card objects from their owners' hands. Selection is
    /// expressed separately (usually with [`Self::Choose`]); this leaf is the
    /// rules action that moves the chosen cards and emits discard events.
    DiscardCards {
        object: EffectRecipientDef,
    },
    /// Deals damage and gains its controller that much life, but no more
    /// than the recipient had to give: a player's life total, a
    /// planeswalker's loyalty, or a creature's toughness, each read before
    /// the damage. Draining an almost-dead target gains only what was there.
    DrainLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    DrawCards {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Resume the prospective draw whose replacement program is currently
    /// resolving. This is not a fresh draw: the replacement effects already
    /// applied to that event remain applied under CR 614.5.
    ContinueReplacedDraw,
    /// The affected player loses all unspent mana without invoking the
    /// turn-based mana-pool emptying procedure (and therefore without mana
    /// burn in formats that use it).
    EmptyManaPool {
        player: EffectRecipientDef,
    },
    /// Exiles, remembering which object sent it there so a later clause can
    /// bring it back. This is the Oblivion Ring shape. A continuation can
    /// immediately consume that link for blink effects while still naming
    /// the new exiled object rather than the one that left its prior zone.
    ExileLinkedToSource {
        object: EffectRecipientDef,
        /// Whether the cards lie face down there. Everyone can count a
        /// face-down exile; only its owner knows what is in it, which is
        /// what keeps Memory Jar from showing both hands to the table.
        face_down: bool,
        /// Whether the card prints "until this permanent leaves the
        /// battlefield". Such an exile does nothing at all when the source
        /// has already gone by the time the ability resolves (CR 610.3b) --
        /// unlike an ordinary linked exile with a separate leave trigger,
        /// where what it takes on the way past stays taken.
        until_source_leaves: bool,
        then: Option<&'static EffectDef>,
    },
    /// Let one player look at cards already in exile. This is information,
    /// not a play permission; it normally follows an effect that put a linked
    /// card there face down.
    PermitLookAtExiled {
        object: EffectRecipientDef,
        player: PlayerRefDef,
        then: &'static EffectDef,
    },
    /// Exiles, and leaves the card's own owner able to play it from there
    /// for as long as it stays exiled -- for a surcharge, which is what
    /// distinguishes the clause from plain theft.
    ///
    /// Unlike [`Self::ExileLinkedToSource`] nothing remembers who did it: the
    /// card does not come back when the exiling permanent dies, and the
    /// permission outlives whatever granted it.
    ExileGrantingOwnerPlay {
        object: EffectRecipientDef,
        /// What a spell cast from that exile costs on top of its own mana
        /// cost. Empty is a legal value and means the card is simply
        /// castable from where it now sits.
        surcharge: ManaCost,
    },
    /// Exiles the recipient and lets this effect's controller play the new
    /// exiled object this turn. Moving and granting permission are one zone
    /// operation because the card in exile has a new identity.
    ExileGrantingControllerPlayThisTurn {
        object: EffectRecipientDef,
    },
    /// Move control of the recipient for the stated duration. Source-bound
    /// durations also remember whether the source must remain tapped.
    ///
    /// Almost every card that says this gives control to the effect's own
    /// controller, which is what "gain control" means; Wishclaw Talisman
    /// hands the permanent to an opponent instead, so who receives it is
    /// part of the effect.
    GainControl {
        object: EffectRecipientDef,
        controller: PlayerRefDef,
        duration: ControlDurationDef,
    },
    /// Swap who controls two permanents, reading both controllers before
    /// either moves. Two ordinary control changes cannot say this: whichever
    /// ran first would change the answer the second one needs, so an
    /// exchange is one effect rather than a pair. If the entire exchange is
    /// impossible, `otherwise` runs without applying either control change.
    ExchangeControl {
        first: EffectRecipientDef,
        second: EffectRecipientDef,
        otherwise: Option<&'static EffectDef>,
    },
    GainLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Sets life through one ordinary gain or loss event (CR 119.5).
    SetLifeTotal {
        recipient: EffectRecipientDef,
        total: ValueDef,
    },
    /// Runs `then` only if the condition holds where this effect resolves.
    /// A condition on a triggered ability is an intervening-if and is checked
    /// twice; this one is part of the effect and is checked once.
    IfCondition {
        condition: &'static TriggerConditionDef,
        then: &'static EffectDef,
    },
    /// Chooses exactly one branch from a condition evaluated where this
    /// effect resolves. Unlike two sequential [`Self::IfCondition`] effects,
    /// the chosen branch cannot change the condition before the alternative
    /// is selected.
    IfElseCondition {
        condition: &'static TriggerConditionDef,
        then: &'static EffectDef,
        otherwise: &'static EffectDef,
    },
    /// Resolve one branch under a particular per-game format profile. Card
    /// definitions remain format-neutral; only the rules procedure varies.
    IfFormat {
        format: Format,
        then: &'static EffectDef,
        otherwise: &'static EffectDef,
    },
    /// "Search your library and graveyard for N cards and exile the rest.
    /// Put the chosen cards on top of your library in any order."
    ///
    /// Three things at once that an ordinary search is not: it looks in more
    /// than one zone as a single search, the cards it does not take are
    /// exiled rather than left where they were, and the order they are
    /// chosen in is the order they end up in.
    SearchZonesAndExileRest {
        player: EffectRecipientDef,
        zones: &'static [ZoneKind],
        count: u8,
    },
    /// "Put target nonland permanent into its owner's library just beneath
    /// the top N cards of that library." Neither top nor bottom: the depth
    /// is the whole point, and it is chosen as the spell is cast. A depth
    /// past the end of the library puts the card on the bottom.
    PutIntoLibraryBeneathTop {
        object: EffectRecipientDef,
        depth: ValueDef,
    },
    /// Ninjutsu's payoff (CR 702.49b): put the card this ability came from
    /// onto the battlefield from its owner's hand, tapped and attacking the
    /// same defender the returned creature was attacking.
    PutSourceOntoBattlefieldAttacking,
    /// "Will of the council" (CR 701.34): starting with the resolving
    /// controller, each player in turn votes for one of the permanents the
    /// predicate names, and then every permanent with the most votes --
    /// including every one tied for most -- is exiled.
    ///
    /// The predicate is read against the resolving controller, so "a
    /// nonland permanent you don't control" means the same set for every
    /// voter: "you" is the spell's controller, not whoever is voting.
    VoteForPermanentToExile {
        object: ObjectPredicateDef,
    },
    /// The named player becomes the monarch (CR 720.2), taking the crown from
    /// whoever held it; a player who has it keeps it.
    BecomeMonarch {
        player: PlayerRefDef,
    },
    /// Installs a triggered ability that listens from outside every zone.
    InstallTrigger(InstalledTriggerDef),
    /// Freeze the top cards of a library as a named collection before subsequent
    /// selection, partition, arrangement, or ordinary effects.
    BindObjects(super::BindObjectsDef),
    /// Branch according to whether a previously frozen collection is empty.
    IfNoObjects(super::IfNoObjectsDef),
    /// A static effect that turns off one landwalk for blocking purposes:
    /// creatures with it can be blocked as though they did not have it. The
    /// keyword is untouched -- anything else reading it still sees it -- so
    /// this is a blocking rule rather than an ability-removing one.
    LandwalkCanBeBlocked(BasicLandType),
    /// One player looks at another's hand. Nothing changes zones and no
    /// decision follows; the looking player simply knows.
    LookAtHand {
        player: EffectRecipientDef,
    },
    LoseLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// A state-based loss with no life total involved (CR 104.3b). Nothing
    /// can be done about it once the effect resolves.
    LoseTheGame {
        player: EffectRecipientDef,
    },
    /// "You win the game." The mirror of [`Self::LoseTheGame`], and rarer:
    /// most cards that end a game do it to the other player.
    WinTheGame {
        player: EffectRecipientDef,
    },
    /// An effect the named player may decline. Held by reference so that
    /// `EffectDef` does not grow a recursive inline copy of itself.
    May {
        player: EffectRecipientDef,
        effect: &'static EffectDef,
    },
    /// Reveal from the top of a library until a matching card turns up.
    /// Held behind a reference the way a top-card selection is: what it
    /// takes, where the match goes, and where its output is bound are several
    /// knobs rather than one.
    ///
    /// Distinct from [`Self::Mill`], whose count is known before it starts:
    /// how deep this goes is whatever the library says.
    MillUntil(&'static MillUntilDef),
    /// "Exile cards from the top of your library until you exile a nonland
    /// card. You may cast that card by paying an amount of {E} equal to its
    /// mana value rather than paying its mana cost."
    ///
    /// Everything passed on the way is exiled too, so this is not a mill with
    /// a different destination: what distinguishes the matched card is the
    /// permission it carries, not where it lands. Nothing states a duration,
    /// so the permission lasts as long as the card sits there.
    ExileFromTopUntil {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
        /// What the card that matched may be cast for afterwards, and by
        /// whom: the permission always belongs to the effect's controller,
        /// which is what lets Etali cast out of somebody else's library.
        permission: ExiledCastPermissionDef,
    },
    /// Cascade (CR 702.85). Exile cards from the top of the controller's
    /// library until a nonland card with mana value less than this spell's
    /// is exiled, offer to cast that card without paying its mana cost, then
    /// put every card exiled this way on the bottom of the library in a
    /// random order.
    ///
    /// One effect rather than three composed ones: the bound is read off the
    /// spell the keyword is printed on, the offer belongs to this resolution
    /// rather than to a window afterwards, and the pile goes home whether or
    /// not anything was cast. It carries nothing because everything it asks
    /// about is the spell it is printed on.
    Cascade,
    /// Put that many cards from the top of a library into its owner's
    /// graveyard.
    Mill {
        player: EffectRecipientDef,
        amount: ValueDef,
    },
    /// "That player exiles the top N cards of their library. Until end of
    /// turn, you may play those cards without paying their mana costs."
    ///
    /// The exile and the permission are one clause: the cards are gone
    /// whether or not anything is played, and the permission belongs to the
    /// resolving controller rather than to the player who lost them.
    ExileTopOfLibraryToPlay {
        player: EffectRecipientDef,
        amount: ValueDef,
        /// Whether the permission also waives the cards' mana costs: "you
        /// may play those cards" and "without paying their mana costs" are
        /// different clauses, and only the second one is free.
        free: bool,
        /// Whether the cards are exiled face down. Only their owner sees
        /// what they are; everybody can count them.
        face_down: bool,
        /// How long the permission lasts.
        duration: ExilePlayDurationDef,
        /// Whether mana spent on the card may be of any colour, which is a
        /// property of the permission rather than of the card.
        spend_any_color: bool,
        /// What has to be true where the card is played, asked there rather
        /// than where it was granted.
        play_condition: Option<ExilePlayConditionDef>,
        /// Whether the clause says "cast" rather than "play". A cast
        /// permission does not reach a land, which is played rather than
        /// cast (CR 305.1); a play permission reaches both.
        cast_only: bool,
    },
    /// "Exile the top card of your library. You may cast that card. If you
    /// don't, ..." The card is cast during this resolution rather than at
    /// some later window, so the offer is the resolution: the player either
    /// answers it by casting, which ignores the timing its type would
    /// normally impose, or declines and `otherwise` runs. A card left
    /// unplayed stays in exile with no permission attached to it.
    ExileTopAndMayCast {
        player: EffectRecipientDef,
        otherwise: Option<&'static EffectDef>,
    },
    /// "You may cast target instant or sorcery card from your graveyard
    /// without paying its mana cost."
    ///
    /// Unlike a granted flashback the permission is this resolution's alone:
    /// the offer is the resolution, and a card left uncast keeps nothing. The
    /// ability named here is what the card is lent while the offer stands,
    /// which is also what says the cast is free and what exiles the card
    /// afterwards.
    MayCastTargetWithoutPaying {
        object: EffectRecipientDef,
        ability: &'static AbilityDef,
    },
    MoveToZone {
        object: EffectRecipientDef,
        zone: ZoneKind,
        /// Which end of a library the card lands on. Meaningless for every
        /// other destination.
        placement: ZonePlacement,
    },
    /// Composes a prospective permanent's entry state around a zone-moving
    /// effect. Nothing here is a later effect such as [`Self::Tap`].
    WithBattlefieldArrival {
        effect: &'static EffectDef,
        arrival: BattlefieldArrivalDef,
    },
    /// Runs `then` after a zone-moving effect, with the moved objects saved
    /// in `binding`. The continuation explicitly follows each object's next
    /// zone-change successor, so it remains correct across a delayed entry.
    WithZoneMoveResult {
        effect: &'static EffectDef,
        binding: Binding,
        then: &'static EffectDef,
    },
    /// "…then mill a card. If an Insect card was milled this way, … and
    /// repeat this process." The mill belongs to the loop because its output
    /// controls whether that same body runs again, not a later sequence step.
    MillWhileMatching(&'static MillLoopDef),
    /// "Target opponent exiles the top card of their library, a card at
    /// random from their graveyard, and a card at random from their hand."
    /// One effect because what it grants is one permission over the pile.
    ExileOneFromEachZone(&'static PileExileDef),
    /// "You may cast that card this turn." The cost is still owed and the
    /// timing rules still apply: the graveyard is merely a legal place to
    /// cast the named card from, until the turn ends.
    PermitCastFromGraveyardThisTurn { object: EffectRecipientDef },
    /// "Look at a card at random in target player's hand." Private to the
    /// looker rather than published, and one card rather than the hand.
    LookAtRandomCardInHand {
        player: EffectRecipientDef,
    },
    /// Every card in the named player's hand is revealed to everyone.
    ///
    /// Reveal one card chosen at random from a player's hand. Wrap this in
    /// [`Self::BindOutput`] when a later step needs the revealed object.
    ///
    /// The card does not move; what changes is that everyone has seen it and
    /// the following clause can read it. An empty hand reveals nothing and
    /// produces no object output.
    RevealAtRandomFromHand {
        player: EffectRecipientDef,
    },
    /// Nothing moves; what changes is what the table knows. It is a separate
    /// step from whatever reads the hand afterwards, because the reveal
    /// happens even when the clause that follows finds nothing to do -- and
    /// unlike [`Self::LookAtHand`], one player learning is not enough.
    RevealHand {
        player: EffectRecipientDef,
    },
    /// CR 506.4: the permanent stops attacking or blocking, and anything
    /// blocking it stops. An attacker removed this way was still blocked, so
    /// it deals no damage rather than getting through.
    RemoveFromCombat {
        object: EffectRecipientDef,
    },
    None,
    PayOr(PayOrDef),
    /// Resolve the nested effect once for each member of an object-set
    /// binding, in binding order. Each iteration places its current member in
    /// the single-object binding. The set was already frozen by the effect
    /// that introduced it, so cards moving during earlier iterations do not
    /// change the remaining work.
    ForEachInBinding {
        objects: Binding,
        binding: Binding,
        effect: &'static EffectDef,
    },
    /// "Damage can't be prevented this turn." A rule about every prevention
    /// rather than one of them, protection included (CR 702.16e).
    DamageCannotBePreventedThisTurn,
    /// Install a resolved damage-prevention rule for the named duration.
    PreventDamage {
        prevention: DamagePreventionDef,
        duration: ResolvedEffectDurationDef,
    },
    /// Select one branch using the game's replay-stable seeded RNG.
    Randomized {
        likelihood: LikelihoodDef,
        on_success: &'static EffectDef,
        on_failure: &'static EffectDef,
    },
    /// This card costs that much less generic mana to cast. A static ability
    /// that works from the hand, where casting reads it.
    ReduceGenericCostBy(ValueDef),
    /// A permanent on the battlefield changing what a spell or an activated
    /// ability costs, which is the difference from
    /// [`Self::ReduceGenericCostBy`]: that one is a card in hand cutting its
    /// own cost and names nothing, while this has to say which spells and
    /// cast by whom. The related spellings live together in
    /// [`CostModificationDef`] because every consumer takes them together:
    /// the mana planner prices a spell against all of them at once, and
    /// every clause that is not about cost passes over the whole family.
    ModifyCost(CostModificationDef),
    /// Creates a regeneration shield (CR 701.15). The shield is not the
    /// regeneration: it waits, and the next destruction this turn is replaced
    /// by tapping the permanent, removing it from combat, and removing all
    /// damage from it. Shields that go unused are cleared in cleanup, so a
    /// creature that was never destroyed keeps nothing.
    Regenerate {
        object: EffectRecipientDef,
    },
    /// Removes every counter of one kind from the recipient.
    /// "Double the number of <kind> counters on each ..." Every matching
    /// permanent gains as many counters as it already has, read one
    /// permanent at a time. A shared amount cannot say this: the whole point
    /// is that each object doubles its own, and a permanent carrying none
    /// gains none.
    DoubleCounters {
        object: EffectRecipientDef,
        kind: CounterKind,
    },
    /// Removes every counter of one kind, or -- when no kind is named --
    /// every counter of every kind. "Remove all counters from target
    /// permanent" is the second: what it takes off a planeswalker is its
    /// loyalty, which is why the thing then dies.
    RemoveAllCounters {
        object: EffectRecipientDef,
        kind: Option<CounterKind>,
    },
    /// Removes that many counters of one kind, or as many as are there. The
    /// mirror of [`Self::AddCounters`], for the clauses that take some back
    /// rather than all of them.
    RemoveCounters {
        object: EffectRecipientDef,
        kind: CounterKind,
        amount: ValueDef,
    },
    /// Replace the named player's next draw this turn with another effect.
    /// The replacement is frozen with the resolving object and consumed even
    /// when its instructions cannot move a card.
    ReplaceNextDrawThisTurn {
        player: EffectRecipientDef,
        effect: &'static EffectDef,
    },
    /// Explore (CR 701.40a). Reveal the top card of that creature's
    /// controller's library. Put that card into their hand if it is a land
    /// card. Otherwise put a +1/+1 counter on the creature, then put the
    /// card back on top of the library or into the graveyard, at that
    /// player's choice.
    ///
    /// A procedure of its own rather than a composition: what happens to the
    /// revealed card and whether the creature grows both turn on a card type
    /// nobody knows until the card is revealed, and the branch that does not
    /// take it ends in a choice.
    Explore {
        object: EffectRecipientDef,
    },
    /// Proliferate (CR 701.28a). Choose any number of permanents and/or
    /// players, then give each another counter of each kind already there.
    ///
    /// A procedure of its own rather than a composition. The choice runs
    /// over permanents and players at once, which no object set can say, and
    /// what each chosen thing gets is read off what is already on it rather
    /// than named by the card.
    Proliferate,
    /// Freely divide a frozen collection into two groups. A later stage decides
    /// whether anyone chooses between them.
    PartitionGroup(super::PartitionGroupDef),
    /// Shuffle the order of a frozen collection without moving its members.
    RandomizeObjectOrder(super::RandomizeObjectOrderDef),
    /// Reveal every card in a frozen collection before continuing.
    RevealObjects(super::RevealObjectsDef),
    /// Move a frozen collection in order and bind the objects it becomes.
    MoveObjects(super::MoveObjectsDef),
    /// Put a frozen collection onto the battlefield with face-down copiable
    /// values, binding the permanents it becomes.
    PutObjectsOntoBattlefieldFaceDown(super::PutObjectsOntoBattlefieldFaceDownDef),
    /// Returns everything this ability's source exiled, to the named zone.
    /// A returned permanent keeps `grant` until end of turn, which is how
    /// Obzedat comes back ready to attack.
    ReturnLinkedExiles {
        /// Which of them come back. "Each creature card exiled with this
        /// artifact" leaves the rest of the pile where it is, so the
        /// predicate is part of the clause rather than a filter on the exile
        /// that put them there.
        object: ObjectPredicateDef,
        zone: ZoneKind,
        grant: Option<KeywordAbility>,
        /// Counters each returning permanent arrives carrying. They belong to
        /// the arrival for the same reason the controller does: what enters
        /// is a new object, and a later clause would have nothing to name.
        counters: Option<TokenCountersDef>,
        /// "Return him to the battlefield transformed." The returning card
        /// is a new object, so which face it shows is settled as it arrives
        /// rather than by a transform afterwards.
        transformed: bool,
        /// Who the returning permanent arrives under. `None` is its owner,
        /// which is what an ordinary "return it to the battlefield" means.
        /// A card that says "under your control" needs the ability's own
        /// controller instead, and the two differ exactly when the creature
        /// was stolen -- which is the reason a blink is worth playing.
        controller: Option<PlayerRelation>,
    },
    Sacrifice {
        object: EffectRecipientDef,
    },
    /// "You sacrifice it": CR 701.17a lets a player sacrifice only what they
    /// control, so a permanent the ability's controller has lost is left
    /// alone, where [`Self::Sacrifice`] is "sacrificed by its controller".
    SacrificeYours {
        object: EffectRecipientDef,
    },
    /// Several players make non-targeting permanent choices before the
    /// resulting partition is exposed to an ordinary nested effect.
    ChooseForEachPlayer(super::ChooseForEachPlayerDef),
    /// Each recipient player chooses one permanent they control that matches,
    /// and sacrifices it. This remains a dedicated multi-player procedure:
    /// every affected player's APNAP-ordered choice is frozen before any
    /// permanent moves, forced-sacrifice prohibitions are applied, and an
    /// optional follow-up can read the sacrificed permanent's last-known
    /// power. A generic [`Self::Choose`] followed by [`Self::Sacrifice`] cannot
    /// preserve those multiplayer and LKI semantics.
    SacrificeOfChoice {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
        /// How many the player gives up. A player with fewer than this many
        /// gives up every one they have, which is what "sacrifices three
        /// creatures" means to somebody holding two. The follow-up below
        /// reads one sacrificed permanent, so a clause that counts on it
        /// takes exactly one.
        count: ValueDef,
        /// Run after the sacrifice, with the characteristic named by
        /// `amount` as [`ValueDef::TriggerEventAmount`]. A sacrifice of
        /// choice waits on a decision, so anything reading what was
        /// sacrificed has to be part of the same continuation rather than the
        /// next effect in sequence.
        then: Option<&'static EffectDef>,
        /// Which of the sacrificed permanent's characteristics the follow-up
        /// reads. Both are last-known by the time it runs, so neither is
        /// harder to reach than the other -- the card simply has to say.
        amount: SacrificedAmountDef,
        /// Run instead when an optional sacrifice was declined or had nothing
        /// to take. This is the "unless" half: a card saying "unless you
        /// sacrifice an Island, ..." is one offer with two branches rather
        /// than a payment and a separate check.
        otherwise: Option<&'static EffectDef>,
        /// Whether the player may decline. An optional sacrifice runs `then`
        /// only when something was actually sacrificed, which is what "if a
        /// player does" means; a compulsory one runs it either way, so an
        /// amount read off nothing is zero rather than skipped.
        optional: bool,
    },
    /// Schedules these additional phases after the current phase. Later
    /// schedules at the same boundary happen before earlier ones, while the
    /// order inside one schedule is preserved.
    ScheduleTurnPhases(&'static [TurnPhaseDef]),
    /// Search one player's card zone for matching cards and move the chosen
    /// cards. `minimum` and `maximum` model the stated quantity independently
    /// from whether the predicate describes a quality: a search for simply
    /// "a card" is compulsory when one exists, while a qualified hidden-zone
    /// search may legally fail to find and therefore uses a minimum of zero.
    SearchZone {
        player: EffectRecipientDef,
        source: ZoneKind,
        object: ObjectPredicateDef,
        minimum: usize,
        /// How many cards may be taken. A value rather than a constant
        /// because "up to X basic land cards, where X is the number of lands
        /// you control" sizes the search from the board it is cast into.
        maximum: ValueDef,
        reveal: bool,
        destination: ZoneKind,
        placement: ZonePlacement,
        shuffle: bool,
        /// Whether a permanent this search puts onto the battlefield arrives
        /// tapped.
        enters_tapped: bool,
        /// Which way an attachment participates in the battlefield arrival.
        attachment: Option<ArrivalAttachmentDef>,
        /// Where the cards this search found are saved, for the follow-up
        /// below to speak about. Scoped to `then` exactly the way every
        /// other binding is scoped to the effect it introduces.
        binding: Option<Binding>,
        /// What happens once the search is answered. Intuition's opponent
        /// chooses among the three that were found, so the choice has to be
        /// inside the search rather than after it: a sequence step following
        /// a search runs from a context captured before the search was
        /// answered, and would see nothing.
        then: Option<&'static EffectDef>,
    },
    /// Resolve authored sibling instructions in order. A [`Self::BindOutput`]
    /// step explicitly publishes its labeled result to later siblings.
    Sequence(&'static [EffectDef]),
    /// Randomizes each recipient player's library. Effects that shuffle
    /// cards from other zones into a library first express those zone moves
    /// with [`Self::MoveToZone`], then use this shared operation.
    ShuffleLibrary {
        player: EffectRecipientDef,
    },
    /// "Puts all the cards from their graveyard on the bottom of their
    /// library in a random order." One effect rather than a queried move
    /// plus a shuffle: the randomization is what the clause is for.
    BuryGraveyard {
        player: EffectRecipientDef,
    },
    /// "This Mount becomes saddled until end of turn" (CR 702.166a). A fact
    /// about the permanent rather than a counter, and it ends with the turn.
    Saddle {
        object: EffectRecipientDef,
    },
    /// "You may play those cards without paying their mana costs."
    MayPlayWithoutPaying(FreePlayDef),
    /// The object sits out this many of its controller's untap steps.
    SkipNextUntapSteps {
        object: EffectRecipientDef,
        count: u8,
    },
    /// A descriptive marker for an effect shape the shared vocabulary does not
    /// yet represent. Catalog runtime validation rejects this marker from fully
    /// declarative cards.
    Special(&'static str),
    /// A static application gated by a separately composed condition over an
    /// object set. This owns its nonrecursive parts so mechanic helpers can
    /// assemble them without card-local staging tables.
    ConditionalStatic(ConditionalStaticEffectDef),
    /// A continuous or rules-modifying effect derived live from a static
    /// ability. Its lifetime is the ability's own applicability rather than a
    /// stored duration.
    StaticApply {
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
    },
    /// Gives each affected player an extra turn after the current one. Extra
    /// turns are queued by the turn engine, so a later-created turn happens
    /// before an earlier-created one.
    TakeExtraTurn {
        player: EffectRecipientDef,
    },
    Tap { object: EffectRecipientDef },
    /// "Put it onto the battlefield, then <clause about it>." What enters is
    /// a new object, so the arrival is saved in `binding` for the clause that
    /// names it.
    PutOntoBattlefieldThen {
        object: EffectRecipientDef,
        binding: Binding,
        counters: Option<TokenCountersDef>,
        then: &'static EffectDef,
    },
    /// Turns a double-faced permanent over to its other face.
    Transform { object: EffectRecipientDef },
    Untap { object: EffectRecipientDef },
}
