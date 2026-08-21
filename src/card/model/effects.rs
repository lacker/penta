mod children;
mod composites;
mod conditions;
mod emblem_creation;
mod likelihood;
mod replacements;
mod selection;
mod token_creation;
mod triggers;
mod turn_structure;
mod values;

pub(crate) use children::child_effects;
pub use composites::*;
pub use conditions::*;
pub use likelihood::*;
pub use replacements::*;
pub use selection::*;
pub use triggers::*;
pub use turn_structure::*;
pub use values::*;

use super::payments::{EffectPaymentDef, PayOrDef};
use crate::Format;
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};

use super::{
    AbilityDef, AddManaEffectDef, BasicLandType, CardTypeSet, ColorSet, ComparisonDef, CounterKind,
    KeywordAbility, ManaColor, ManaCost, ObjectPredicateDef, PlayActionKind, PlayerRelation,
    TokenCharacteristics, TriggerConditionDef, ZoneKind, ZonePlacement,
};

// Effect subjects, lifetimes, and event matchers form the shared vocabulary
// consumed by both resolving and continuously applied effects below.
include!("effects/recipients_and_matchers.rs");
include!("effects/applied.rs");
include!("effects/vocabulary.rs");
/// Declarative effect primitives interpreted by the rules engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDef {
    AddCounters {
        object: EffectRecipientDef,
        kind: CounterKind,
        amount: ValueDef,
    },
    AddMana(AddManaEffectDef),
    /// Adds mana of one colour, however much a value says. Mana abilities use
    /// [`Self::AddMana`] with a fixed amount so the mana planner can read
    /// them without resolving anything; this is for the effects that cannot
    /// know their amount until they resolve.
    AddManaEqualTo {
        color: ManaColor,
        amount: ValueDef,
    },
    /// Poison counters given to a player. Ten of them is a state-based loss,
    /// which is why this is not expressible as life loss.
    /// "{cost}: Level N." Puts level counters on the source until its level
    /// is `level`, which is one fewer counter than the printed number: a
    /// Class is level 1 with none on it (CR 717.3). Reaching a level raises
    /// an event, so a "when this Class becomes level N" clause fires exactly
    /// once however many levels were climbed at a time.
    GainClassLevel {
        level: u8,
    },
    AddPoisonCounters {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// "You get {E}{E}." Energy is a resource rather than a clock: nothing
    /// checks how much of it a player has, it persists between turns, and it
    /// leaves only by being spent.
    AddEnergyCounters {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    Apply {
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: ResolvedEffectDurationDef,
    },
    /// An Aura spell attaching itself to what it enchants. The permanent the
    /// spell becomes is what attaches, so this is only meaningful on the spell
    /// clause of an Aura.
    Attach {
        object: EffectRecipientDef,
    },
    /// Phase the recipient out. It is treated as though it does not exist
    /// until it phases in, which happens before its controller untaps during
    /// their next untap step (CR 702.25). Phasing is not a zone change:
    /// nothing enters or leaves, and the permanent keeps everything it had.
    PhaseOut {
        object: EffectRecipientDef,
    },
    /// Put an Aura onto the battlefield from another zone, already attached
    /// to a host. One effect rather than a move followed by an attach: the
    /// card that arrives is a new object, so nothing an ordinary attach
    /// could name still points at it.
    ReturnAttached {
        object: EffectRecipientDef,
        attach_to: EffectRecipientDef,
    },
    /// Soulbond's pairing. The chosen creature and the ability's source
    /// record each other; the pair is symmetric and survives until one of
    /// them stops being a creature its controller controls.
    PairWithSource {
        object: EffectRecipientDef,
    },
    /// Reconfigure's paired attach/unattach procedure. A selected creature
    /// becomes the new host; selecting none ends this attachment incarnation.
    Reconfigure {
        object: EffectRecipientDef,
    },
    /// Detach the named Equipment or Fortification without moving it. This is
    /// a rules action rather than a zone change: Elbrus does it immediately
    /// before transforming, while the host and both objects remain otherwise
    /// unchanged.
    Unattach {
        object: EffectRecipientDef,
    },
    /// Replaces the source permanent's copiable values with the target's.
    /// Some copy effects, such as Thespian's Stage, retain the resolving
    /// ability as an exception to the copied values.
    BecomeCopyOf {
        /// What is copied.
        object: EffectRecipientDef,
        /// What becomes the copy. `None` is the source itself, which is what
        /// "this land becomes a copy of target land" means; a clause that
        /// points at something else names it here.
        copier: Option<EffectRecipientDef>,
        retain_source_ability: bool,
        /// Types the copy has on top of what it copied, for "except it's an
        /// artifact in addition to its other types".
        added_types: CardTypeSet,
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
        /// A continuous effect a permanent this choice puts onto the
        /// battlefield arrives carrying, for the same reason
        /// [`Self::MoveToZone`] has one: what enters is a new object, and by
        /// the time a following effect ran there would be nothing to name.
        /// Ignored for every other destination.
        arrival_effect: Option<&'static AppliedEffectDef>,
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
    ReturnSpellToHand {
        object: EffectRecipientDef,
    },
    /// Counter a spell and put its card into `zone`. Ordinary counters use
    /// the graveyard; replacement-style counters such as Dissipate use exile.
    Counter {
        object: EffectRecipientDef,
        zone: ZoneKind,
    },
    /// Saves a set of objects for the rest of this resolution and continues.
    /// Nothing is asked and nothing moves: Haunting Echoes needs to know
    /// which cards it exiled from a graveyard *before* it empties one, so
    /// that what it then hunts through the library is the set it took.
    BindMatching {
        objects: ObjectSetDef,
        binding: ObjectSetBindingIndex,
        then: &'static EffectDef,
    },
    /// Names a card while this effect resolves, binds every card of that name
    /// where it looks, and continues. "Discards all cards with that name" is
    /// the follow-up naming that binding. Distinct from the entry choice a
    /// permanent records, which outlives its resolution.
    ChooseCardName {
        chooser: PlayerRefDef,
        nonland_only: bool,
        /// Whose cards the name is matched against, and where.
        matched_in: PlayerRefDef,
        zone: ZoneKind,
        /// Where the matching cards are saved for the rest of the effect.
        /// Binding them as the name is chosen means the follow-up names a set
        /// rather than re-deriving it from a name it cannot see.
        binding: ObjectSetBindingIndex,
        then: &'static EffectDef,
    },
    /// Copies the spell this effect belongs to, letting `chooser` retarget the
    /// copy. Fork copies something else and repaints it; this is the shape a
    /// card uses to copy itself, so the copy keeps its own colours.
    CopyResolvingSpell {
        chooser: PlayerRefDef,
        /// How many copies. Each is offered its own retarget choice, which is
        /// what "you may choose new targets for the copies" asks for.
        count: ValueDef,
    },
    /// Gives its controller an emblem, an object that sits outside every
    /// zone and does nothing but carry its abilities.
    CreateEmblem {
        emblem: super::EmblemCharacteristics,
    },
    /// Creates tokens with `token`'s complete authored characteristics under
    /// the resolving object's controller.
    CreateToken {
        token: TokenCharacteristics,
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
    /// Creates one token, then attaches the resolving permanent to that
    /// exact battlefield incarnation before state-based actions run. This is
    /// the common living-weapon shape; the delayed entry completion keeps it
    /// correct through replacement effects and zone-change identity.
    CreateAttachedToken {
        token: TokenCharacteristics,
    },
    /// Creates a token copying the recipient's copiable values. Populate uses
    /// this after its generic choice has selected a creature token.
    CreateTokenCopyOf {
        object: EffectRecipientDef,
    },
    DealDamage {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
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
    Destroy {
        object: EffectRecipientDef,
        can_regenerate: bool,
    },
    /// The object is destroyed when this combat phase ends.
    DestroyAtEndOfCombat {
        object: EffectRecipientDef,
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
    /// The affected player loses all unspent mana without invoking the
    /// turn-based mana-pool emptying procedure (and therefore without mana
    /// burn in formats that use it).
    EmptyManaPool {
        player: EffectRecipientDef,
    },
    /// Exiles, remembering which object sent it there so a later clause can
    /// bring it back. This is the Oblivion Ring shape.
    ExileLinkedToSource {
        object: EffectRecipientDef,
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
    /// exchange is one effect rather than a pair.
    ExchangeControl {
        first: EffectRecipientDef,
        second: EffectRecipientDef,
    },
    GainLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Lets the next sorcery its controller casts this turn be cast as
    /// though it had flash.
    GrantFlashToNextSorcery,
    /// Runs `then` only if the condition holds where this effect resolves.
    /// A condition on a triggered ability is an intervening-if and is checked
    /// twice; this one is part of the effect and is checked once.
    IfCondition {
        condition: &'static TriggerConditionDef,
        then: &'static EffectDef,
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
    /// The named player becomes the monarch (CR 720.2). There is only ever
    /// one, so this takes the crown from whoever held it; a player who
    /// already has it keeps it and nothing happens.
    BecomeMonarch {
        player: PlayerRefDef,
    },
    /// Installs a triggered ability that listens from outside every zone.
    InstallTrigger(InstalledTriggerDef),
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
    /// Look privately at the top cards of a library, choose a bounded subset,
    /// place both groups, optionally reveal the selected cards, then continue
    /// resolving. A predicate restricts what may be selected without hiding
    /// the rest of the inspected group.
    LookAtTopAndSelect {
        /// Whose library is inspected.
        player: EffectRecipientDef,
        /// Who looks at the cards and makes the choice. Digging through your
        /// own library names the same player twice, which is the ordinary
        /// case; a spy names someone else's library and keeps the looking.
        looker: EffectRecipientDef,
        selection: &'static TopCardSelectionDef,
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
    /// Reveal from the top of a library until a matching card turns up. What
    /// was passed over goes to the graveyard; the match itself goes where
    /// `matched_zone` says, which is the graveyard for a plain dig and the
    /// hand for Hermit Druid.
    ///
    /// Distinct from [`Self::Mill`], whose count is known before it starts:
    /// how deep this goes is whatever the library says, and a library with
    /// nothing matching empties.
    MillUntil {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
        matched_zone: ZoneKind,
        /// Saves the identities of cards this effect put into a graveyard for
        /// a same-resolution follow-up. They are bound under their new zone
        /// identities rather than reconstructed from the graveyard. When the
        /// matching card has another destination, use [`ValueDef::MatchedCount`]
        /// to count every revealed card; the binding contains only the cards
        /// that were milled.
        binding: Option<ObjectSetBindingIndex>,
        /// Runs immediately after the named reveal-and-move procedures.
        /// [`ValueDef::MatchedCount`] describes every card
        /// revealed, including a match sent somewhere other than a graveyard.
        then: Option<&'static EffectDef>,
    },
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
    },
    /// Manifest dread (CR 701.34, 702.169). Look at the top two cards of
    /// your library, put one onto the battlefield face down as a 2/2
    /// creature, and put the other into your graveyard.
    ///
    /// One effect rather than a top-card selection with a flag, because
    /// what it puts down is not a card arriving in a zone but a body: the
    /// permanent has no name, no types beyond creature, and no abilities
    /// while it is face down, and what turns it up is its own mana cost
    /// rather than anything the selection could say.
    ManifestDread {
        player: EffectRecipientDef,
    },
    /// Cascade (CR 702.85). Exile cards from the top of the controller's
    /// library until a nonland card with mana value less than this spell's
    /// is exiled, offer to cast that card without paying its mana cost, then
    /// put every card exiled this way on the bottom of the library in a
    /// random order.
    ///
    /// One effect rather than three composed ones, because the three cannot
    /// be written apart: the bound is read off the spell the keyword is
    /// printed on, the offer belongs to this resolution rather than to a
    /// window afterwards, and the pile goes home whether or not anything was
    /// cast. It carries nothing because the card names nothing -- everything
    /// it asks about is the spell it is printed on.
    Cascade,
    /// Put that many cards from the top of a library into its owner's
    /// graveyard.
    Mill {
        player: EffectRecipientDef,
        amount: ValueDef,
        /// Where the milled cards are saved for `then` to speak about, by
        /// the identity they have in the graveyard. "Put a creature card
        /// from among them into your hand" names what this mill put there
        /// rather than what the graveyard already held.
        binding: Option<ObjectSetBindingIndex>,
        /// What happens once the cards are in the graveyard. A mill resolves
        /// without stopping to ask, so unlike a search this runs inline.
        then: Option<&'static EffectDef>,
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
        /// Who controls the permanent when the destination is the
        /// battlefield. `None` is the ordinary case, where a card arrives
        /// under its owner's control; reanimation that steals names a
        /// relation instead.
        controller: Option<PlayerRelation>,
        /// A continuous effect the permanent arrives carrying, for the
        /// clauses that say what the thing they just reanimated now is. It
        /// belongs here rather than in a following effect because a
        /// permanent that enters is a new object with a new identity: by the
        /// time the next effect ran, nothing would name it. Lasts as long as
        /// the permanent does.
        arrival_effect: Option<&'static AppliedEffectDef>,
        /// Which way an attachment goes as the permanent arrives, if it is
        /// attached at all. The attachment rides the arrival for the same
        /// reason the effect above does: what enters is a new object, and a
        /// following effect would have nothing left to name.
        attachment: Option<ArrivalAttachmentDef>,
    },
    /// Every card in the named player's hand is revealed to everyone.
    ///
    /// Reveal one card chosen at random from a player's hand, bind it, and
    /// continue.
    ///
    /// The card does not move; what changes is that everyone has seen it and
    /// the following clause can read it. An empty hand reveals nothing and
    /// binds nothing, so the continuation still runs and simply finds no
    /// bound object.
    RevealAtRandomFromHand {
        player: EffectRecipientDef,
        binding: ObjectBindingIndex,
        then: &'static EffectDef,
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
    /// "Damage can't be prevented this turn." Not a prevention of its own
    /// but a rule about every other one, including what protection prevents
    /// (CR 702.16e). Damage limits are not prevention and still apply.
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
    /// Matching spells cost that much less generic mana to cast.
    ///
    /// The difference from [`Self::ReduceGenericCostBy`] is who is
    /// discounted: that one is a card in hand cutting its own cost and needs
    /// to name nothing, while this is read off a permanent and so has to say
    /// which spells, and cast by whom.
    /// A permanent making the activated abilities of matching permanents
    /// cost more. Like the spell increase beside it the amount is a whole
    /// mana cost, and like it the effect is read off the battlefield rather
    /// than baked into the affected permanent.
    IncreaseMatchingAbilityCostBy {
        permanent: ObjectPredicateDef,
        amount: ManaCost,
    },
    /// A permanent making matching permanents' activated abilities cost less
    /// to activate. The mirror of [`Self::IncreaseMatchingAbilityCostBy`],
    /// and like a spell discount it touches generic mana only.
    ///
    /// The printed floor is part of the effect rather than a rule of its
    /// own: "this effect can't reduce the mana in that cost to less than one
    /// mana" is what keeps a free ability from becoming free twice over.
    ReduceMatchingAbilityCostBy {
        permanent: ObjectPredicateDef,
        amount: ValueDef,
        /// The least mana the cost may be left with. An ability whose cost
        /// already holds no more than this is untouched.
        minimum: u16,
    },
    /// A permanent making matching spells cost more. The mirror of
    /// [`Self::ReduceMatchingSpellCostBy`], but the amount is a whole mana
    /// cost rather than a number: an increase can name a colour, which a
    /// discount never does (CR 601.2f lets a reduction touch generic mana
    /// only).
    IncreaseMatchingSpellCostBy {
        spell: ObjectPredicateDef,
        caster: PlayerRelation,
        amount: ManaCost,
    },
    ReduceMatchingSpellCostBy {
        spell: ObjectPredicateDef,
        caster: PlayerRelation,
        amount: ValueDef,
    },
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
    RemoveAllCounters {
        object: EffectRecipientDef,
        kind: CounterKind,
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
        /// A continuous effect each arriving permanent carries, for the
        /// clauses that say what the thing they just returned now is. Lasts
        /// as long as the permanent does.
        arrival_effect: Option<&'static AppliedEffectDef>,
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
    /// "Each opponent chooses an artifact, a creature, an enchantment, and a
    /// planeswalker from among the nonland permanents they control, then
    /// sacrifices the rest." The keeping is a run of choices rather than one
    /// selection: a player may keep at most one of each named type, and a
    /// single multi-select could not say that.
    SacrificeKeepingOnePerType {
        player: EffectRecipientDef,
        types: &'static [crate::card::CardType],
    },
    /// Each recipient player chooses one permanent they control that matches,
    /// and sacrifices it. This remains a dedicated simultaneous procedure:
    /// every affected player's APNAP-ordered choice is frozen before any
    /// permanent moves, forced-sacrifice prohibitions are applied, and an
    /// optional follow-up can read the sacrificed permanent's last-known
    /// power. A generic [`Self::Choose`] followed by [`Self::Sacrifice`] cannot
    /// preserve those multiplayer and LKI semantics.
    SacrificeOfChoice {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
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
        /// Where the cards this search found are saved, for the follow-up
        /// below to speak about. Scoped to `then` exactly the way every
        /// other binding is scoped to the effect it introduces.
        binding: Option<ObjectSetBindingIndex>,
        /// What happens once the search is answered. Intuition's opponent
        /// chooses among the three that were found, so the choice has to be
        /// inside the search rather than after it: a sequence step following
        /// a search runs from a context captured before the search was
        /// answered, and would see nothing.
        then: Option<&'static EffectDef>,
    },
    Sequence(&'static [EffectDef]),
    /// Randomizes each recipient player's library. Effects that shuffle
    /// cards from other zones into a library first express those zone moves
    /// with [`Self::MoveToZone`], then use this shared operation.
    ShuffleLibrary {
        player: EffectRecipientDef,
    },
    /// The object sits out this many of its controller's untap steps.
    SkipNextUntapSteps {
        object: EffectRecipientDef,
        count: u8,
    },
    /// A descriptive marker for an effect portion the shared vocabulary does
    /// not yet represent. The surrounding costs, targets, and timing can still
    /// remain declarative; clause coverage records whether and how it executes.
    Special(&'static str),
    SplitIntoPiles(SplitIntoPilesDef),
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
    Tap {
        object: EffectRecipientDef,
    },
    /// "Put that card onto the battlefield under your control with a
    /// finality counter on it. It gains haste. Sacrifice it at the beginning
    /// of the next end step."
    ///
    /// One effect rather than four, because three of the four name the
    /// permanent that just entered: what enters is a new object, and a
    /// following clause in a sequence would have nothing left to point at.
    /// The card it takes is named by an earlier choice in the same
    /// resolution.
    ReturnWithHasteAndFinality {
        object: EffectRecipientDef,
        /// Where the permanent that arrived is saved. What enters is a new
        /// object, so the clause after it -- "sacrifice it at the beginning
        /// of the next end step" -- cannot name the card that was in exile.
        binding: ObjectSetBindingIndex,
        /// What that clause is. Scoped to the binding the same way every
        /// other effect that introduces one scopes its follow-up.
        then: &'static EffectDef,
    },
    /// Turns a double-faced permanent over to its other face.
    Transform {
        object: EffectRecipientDef,
    },
    Untap {
        object: EffectRecipientDef,
    },
}

impl EffectDef {
    #[must_use]
    pub const fn counter_target(target: TargetIndex) -> Self {
        Self::Counter {
            object: EffectRecipientDef::Target(target),
            zone: ZoneKind::Graveyard,
        }
    }

    #[must_use]
    pub const fn destroy_target(target: TargetIndex, can_regenerate: bool) -> Self {
        Self::Destroy {
            object: EffectRecipientDef::Target(target),
            can_regenerate,
        }
    }
}
