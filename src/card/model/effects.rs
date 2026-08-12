use crate::ids::{CardDefinitionId, TargetIndex};

use super::{
    AbilityDef, AddManaEffectDef, BasicLandType, CardType, CardTypeSet, ColorSet, CostDef,
    CounterKind, KeywordAbility, ManaColor, ManaCost, ObjectPredicateDef, PlayerRelation,
    TriggerConditionDef, ZoneKind, ZonePlacement,
};

/// A value evaluated from the resolving spell or ability and its captured
/// event. `SourcePower` and `SourceToughness` deliberately leave current-versus
/// last-known-information selection to the runtime source reference.
/// A set of objects described the way [`EffectRecipientDef::MatchingObjects`]
/// describes one, so a count and a sweep name their subject identically.
/// The two branches of a conditional value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConditionalValueDef {
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

/// A conditional value that asks how many objects match.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CountConditionDef {
    pub query: ObjectQueryDef,
    pub equals: u8,
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

/// A conditional value that asks what the chosen target is.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TargetConditionDef {
    pub slot: TargetIndex,
    pub object: ObjectPredicateDef,
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectQueryDef {
    pub object: ObjectPredicateDef,
    pub zones: &'static [ZoneKind],
    pub controller: PlayerRelation,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ValueDef {
    Constant(i32),
    ChosenX,
    SourcePower,
    SourceToughness,
    TriggerEventAmount,
    CardsInHandAbove {
        player: PlayerRelation,
        threshold: u8,
    },
    /// How many objects match, for the "for each" clauses. Held by reference
    /// so that `ValueDef` stays small enough to embed freely.
    CountMatchingObjects(&'static ObjectQueryDef),
    /// One when at least one object matches, zero otherwise. "As long as you
    /// control a Mountain" is a condition rather than a count, so counting
    /// matches would pay a second Mountain twice.
    AnyMatchingObject(&'static ObjectQueryDef),
    /// The negation of another value, so a "for each" penalty can reuse the
    /// same count a bonus would.
    Negate(&'static ValueDef),
    /// How many counters of one kind sit on the ability's own source.
    CountersOnSource(CounterKind),
    /// The morbid condition. Held by reference so that `ValueDef` stays one
    /// word wide; a second inline value would grow everything embedding it.
    IfCreatureDiedThisTurn(&'static ConditionalValueDef),
    /// One value when the chosen target matches, another when it does not.
    /// Held by reference for the same reason.
    IfTargetMatches(&'static TargetConditionDef),
    /// One value when exactly that many objects match, another otherwise.
    /// This is how an intervening-if condition becomes an amount.
    IfMatchingObjectCount(&'static CountConditionDef),
    /// How much of a divided total the target being affected takes. Only
    /// meaningful for an effect aimed at a slot the card divides.
    DividedAmongTargets,
    /// The power of what a target slot points at, for "damage equal to its
    /// power".
    TargetPower(TargetIndex),
    /// The mana value of what a target slot points at, read from last-known
    /// information after a permanent or spell has left its zone.
    TargetManaValue(TargetIndex),
}

/// An object or player affected by an effect. Targets are chosen when a spell
/// or stack ability is formed; triggering subjects come from captured events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectRecipientDef {
    Source,
    /// What this permanent is attached to, for an Aura's own static clauses.
    AttachedPermanent,
    /// Every battlefield permanent sharing a name with the chosen target,
    /// including the target itself. "And each other one with the same name"
    /// names the same set.
    ObjectsSharingNameWithTarget(TargetIndex),
    Controller,
    Opponent,
    /// Every player in turn order, starting with the ability's controller.
    /// This keeps effects such as Liliana's +1 simultaneous rather than
    /// resolving one player's discard before the other chooses.
    EachPlayer,
    Target(TargetIndex),
    TriggeringObject,
    /// The triggering object's controller when this effect resolves, using
    /// last-known information if that object is no longer live.
    ControllerOfTriggeringObject,
    /// Everything a query matches among the permanents controlled by whoever
    /// controls a target slot, for "each creature that player controls".
    ObjectsControlledByTarget {
        object: ObjectPredicateDef,
        slot: TargetIndex,
    },
    /// Everything a query matches among the permanents *owned* by the player
    /// a target slot names. Ownership survives a control-changing effect, so
    /// this is a different set from [`Self::ObjectsControlledByTarget`]
    /// whenever anything has changed hands.
    ObjectsOwnedByTarget {
        object: ObjectPredicateDef,
        slot: TargetIndex,
    },
    /// The controller of what a target slot points at, for "its controller".
    /// Read when the effect resolves, using last-known information if that
    /// object has already left the battlefield.
    ControllerOfTarget(TargetIndex),
    /// The player named directly by the event, such as the player whose
    /// upkeep began or who cast the triggering spell.
    EventPlayer,
    MatchingObjects {
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        controller: PlayerRelation,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDurationDef {
    Permanent,
    UntilEndOfTurn,
    /// Until the beginning of the resolving ability's controller's next
    /// upkeep, which outlives the cleanup that ends an until-end-of-turn
    /// effect.
    UntilYourNextUpkeep,
    /// Until the next turn of the effect's controller begins. The affected
    /// turn is captured when the resolving effect is created.
    UntilYourNextTurn,
    WhileSourceRemainsInZone,
    UntilSourceLeavesZone,
}

/// A continuous or rules-modifying effect applied to a game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppliedEffectDef {
    /// Components applied to the same recipient for the same duration as one
    /// continuous effect.
    Composite(&'static [AppliedEffectDef]),
    CannotBeCountered,
    /// No Aura may attach to the affected permanent. This restricts both the
    /// Aura spell's targeting and whether an existing attachment stays legal,
    /// so an Aura already on the permanent falls off.
    CannotBeEnchanted,
    /// A creature matching this predicate cannot block the affected creature.
    CannotBeBlockedBy(ObjectPredicateDef),
    /// Damage from a source matching this predicate is prevented before it
    /// touches the affected permanent. Only a permanent can be the source
    /// today, which is all "damage from artifact creatures" needs.
    PreventDamageFrom(ObjectPredicateDef),
    /// Adds land subtypes without removing the object's existing subtypes.
    AddLandTypes(&'static [BasicLandType]),
    /// Sets the object's land subtypes, removing its existing land subtypes and
    /// abilities supplied by its rules text or copiable values under CR 305.7.
    /// Independently granted abilities are not part of that removal.
    SetLandTypes(&'static [BasicLandType]),
    ModifyPowerToughness {
        power: ValueDef,
        toughness: ValueDef,
    },
    /// Give the affected object an ordinary ability. The granted definition
    /// carries its own keyword, activation, or alternative-casting procedure.
    GrantAbility(&'static AbilityDef),
    /// Remove each ability matching the predicate. Unlike
    /// [`Self::SetLandTypes`], this is an ordinary ability-layer operation and
    /// can remove intrinsic or independently granted abilities.
    RemoveAbilities(AbilityPredicateDef),
    /// Turn the affected permanent into a creature. This is what a manland's
    /// activated ability does, and it keeps the permanent's other types.
    Animate(&'static AnimationDef),
    Special(&'static str),
}

/// A reusable selector for ability-removing continuous effects.
///
/// `Any` supports ordinary "loses all abilities" effects. The keyword form is
/// also the seam needed by text-changing cards that replace one landwalk
/// ability with another without treating the whole rules box as opaque text.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityPredicateDef {
    Any,
    Keyword(KeywordAbility),
}

/// The creature a permanent becomes while an animation effect is active. A
/// manland stays a land, so these types and subtypes are added rather than
/// replacing what is printed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AnimationDef {
    pub power: i16,
    pub toughness: i16,
    /// Added on top of the printed types. `Creature` belongs here; a card
    /// that becomes an artifact creature names both.
    pub types: CardTypeSet,
    pub subtypes: &'static [&'static str],
    /// "With all creature types", which no fixed subtype list can express
    /// because changelings must keep matching types printed later.
    pub all_creature_types: bool,
    /// Whether the printed subtypes are replaced rather than added to, for
    /// "becomes a Weird" as opposed to "becomes an Assembly-Worker as well".
    pub replaces_subtypes: bool,
    /// Whether the permanent loses its printed abilities.
    pub loses_abilities: bool,
    /// The colours the permanent becomes, when the animation repaints it.
    pub colors: Option<ColorSet>,
}

impl AnimationDef {
    #[must_use]
    pub const fn new(power: i16, toughness: i16) -> Self {
        Self {
            power,
            toughness,
            types: CardTypeSet::single(CardType::Creature),
            subtypes: &[],
            all_creature_types: false,
            replaces_subtypes: false,
            loses_abilities: false,
            colors: None,
        }
    }

    /// "Loses all abilities and becomes a ..." — the printed subtypes,
    /// abilities, and colours all give way to what the effect names.
    #[must_use]
    pub const fn becoming(mut self, subtypes: &'static [&'static str], colors: ColorSet) -> Self {
        self.subtypes = subtypes;
        self.replaces_subtypes = true;
        self.loses_abilities = true;
        self.colors = Some(colors);
        self
    }

    #[must_use]
    pub const fn with_types(mut self, types: CardTypeSet) -> Self {
        self.types = types;
        self
    }

    #[must_use]
    pub const fn with_subtypes(mut self, subtypes: &'static [&'static str]) -> Self {
        self.subtypes = subtypes;
        self
    }

    #[must_use]
    pub const fn with_all_creature_types(mut self) -> Self {
        self.all_creature_types = true;
        self
    }
}

/// An event that a replacement ability can modify before it is committed.
///
/// Replacement events deliberately have their own vocabulary rather than
/// reusing [`TriggerEventDef`]: triggers observe events that have already
/// happened, while replacement abilities inspect and modify prospective
/// events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementEventDef {
    /// The object carrying this ability would enter the battlefield.
    SourceEntersBattlefield,
    /// A matching object would enter the battlefield.
    ObjectEntersBattlefield {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
    },
    /// This ability's source would move between the named zones for the
    /// specified reason. Matching happens before the object leaves `from`.
    WouldMove {
        from: ZoneKind,
        to: ZoneKind,
        cause: ZoneMoveCauseDef,
    },
    /// A player would gain life, matched relative to the replacement
    /// ability's controller.
    WouldGainLife(PlayerRelation),
    /// Any object anywhere would be put into this zone. Unlike
    /// [`Self::WouldMove`] this does not describe the moving object's own
    /// ability: the replacement source watches from the battlefield.
    AnyObjectWouldMove { to: ZoneKind },
    /// Compatibility event for existing entry replacements whose exact
    /// subject is identified by their effect primitive.
    EntersBattlefield,
    /// A narrow, named event that is not yet part of the shared vocabulary.
    Special(&'static str),
}

/// What is causing a proposed zone move. A controlled effect is matched
/// relative to the replacement ability's controller; rules and costs do not
/// have an effect controller and therefore only match [`Self::Any`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZoneMoveCauseDef {
    Any,
    EffectControlledBy(PlayerRelation),
}

/// Costs a player may pay while a replacement effect is modifying an event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PaymentDef {
    pub payer: PlayerRelation,
    pub costs: &'static [CostDef],
}

impl PaymentDef {
    #[must_use]
    pub const fn new(payer: PlayerRelation, costs: &'static [CostDef]) -> Self {
        Self { payer, costs }
    }
}

/// A reusable condition evaluated in an effect's source and event context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConditionDef {
    /// At least one object matches this zone, controller, and object query.
    Exists(ObjectQueryDef),
}

/// A typed modification to the permanent an object would become as it enters
/// the battlefield.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BattlefieldEntryModificationDef {
    Tapped,
    AddCounters { kind: CounterKind, amount: u16 },
}

/// Declarative operations performed by a replacement ability.
///
/// Branches are slices so complex replacements remain const-friendly and can
/// be resumed around a player choice without baking card names into the game
/// engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementEffectDef {
    None,
    Sequence(&'static [ReplacementEffectDef]),
    ModifyBattlefieldEntry(BattlefieldEntryModificationDef),
    Conditional {
        condition: ConditionDef,
        if_true: &'static [ReplacementEffectDef],
        if_false: &'static [ReplacementEffectDef],
    },
    OptionalPayment {
        payment: PaymentDef,
        if_paid: &'static [ReplacementEffectDef],
        if_declined: &'static [ReplacementEffectDef],
    },
}

/// Declarative effect primitives interpreted by the rules engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDef {
    None,
    Sequence(&'static [EffectDef]),
    AddMana(AddManaEffectDef),
    DealDamage {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    GainLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    DrawCards {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Each recipient chooses that many cards from their own hand and
    /// discards them. A player holding fewer cards discards their whole hand.
    DiscardCards {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// The recipients discard at random rather than choosing. Nobody is asked
    /// anything, so unlike [`Self::DiscardCards`] this needs no decision.
    DiscardAtRandom {
        recipient: EffectRecipientDef,
        amount: ValueDef,
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
    Tap {
        object: EffectRecipientDef,
    },
    Untap {
        object: EffectRecipientDef,
    },
    /// No combat damage is dealt to or by the affected permanent for the rest
    /// of the turn. This is prevention rather than removal from combat: the
    /// creature is still attacking, and everything that reads that still
    /// sees it.
    PreventCombatDamageThisTurn {
        object: EffectRecipientDef,
    },
    /// Puts token copies of `token` onto the battlefield under the resolving
    /// object's controller.
    CreateToken {
        token: CardDefinitionId,
        count: ValueDef,
    },
    /// An Aura spell attaching itself to what it enchants. The permanent the
    /// spell becomes is what attaches, so this is only meaningful on the spell
    /// clause of an Aura.
    Attach {
        object: EffectRecipientDef,
    },
    Destroy {
        object: EffectRecipientDef,
        can_regenerate: bool,
    },
    Sacrifice {
        object: EffectRecipientDef,
    },
    /// Each recipient player chooses one permanent they control that matches,
    /// and it is destroyed. The choice belongs to the player who owns the
    /// permanents, not to the ability's controller, which is what "of their
    /// choice" means; unlike [`Self::SacrificeOfChoice`] nothing is
    /// sacrificed, so a prohibition on being forced to sacrifice does not
    /// apply.
    DestroyOfChoice {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
        can_regenerate: bool,
    },
    /// Each recipient player chooses one permanent they control that matches,
    /// and sacrifices it. Unlike [`Self::Sacrifice`] the choice is the
    /// player's, so nothing happens when they control nothing matching.
    SacrificeOfChoice {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
        /// Run after the sacrifice, with the sacrificed permanent's power as
        /// [`ValueDef::TriggerEventAmount`]. A sacrifice of choice waits on a
        /// decision, so anything reading what was sacrificed has to be part
        /// of the same continuation rather than the next effect in sequence.
        then: Option<&'static EffectDef>,
        /// Whether the player may decline. An optional sacrifice runs `then`
        /// only when something was actually sacrificed, which is what "if a
        /// player does" means; a compulsory one runs it either way, so an
        /// amount read off nothing is zero rather than skipped.
        optional: bool,
    },
    /// Separate everything a player controls into two piles, then let that
    /// player sacrifice the pile of their choice. The ability's controller
    /// makes the split, which is what makes the choice hard for both.
    SplitPermanentsAndSacrificeAPile {
        player: EffectRecipientDef,
    },
    /// Put that many cards from the top of a library into its owner's
    /// graveyard.
    Mill {
        player: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Reveal the top `count` cards of the controller's library, have an
    /// opponent separate them into two piles, and let the controller take one
    /// pile into hand. Whatever is left goes to `rest`, using `placement`
    /// when that is the library. Fact or Fiction and Jace's second ability
    /// are the same procedure with different losing zones.
    RevealAndSplitIntoPiles {
        count: ValueDef,
        rest: ZoneKind,
        placement: ZonePlacement,
    },
    /// One player looks at another's hand. Nothing changes zones and no
    /// decision follows; the looking player simply knows.
    LookAtHand {
        player: EffectRecipientDef,
    },
    /// Look at the top card of a library and, if it matches, offer to take
    /// it. Looking is private and changes nothing, so declining leaves the
    /// card exactly where it was.
    LookAtTopAndMayTake {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
    },
    /// Search a library for one matching card and put it somewhere, then
    /// shuffle. Searching a hidden zone never obliges the searcher to find,
    /// so a printed "may" adds nothing on top of this.
    SearchLibrary {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
        destination: ZoneKind,
    },
    /// Counter a spell and put its card into `zone`. Ordinary counters use
    /// the graveyard; replacement-style counters such as Dissipate use exile.
    Counter {
        object: EffectRecipientDef,
        zone: ZoneKind,
    },
    /// Deals damage and gains its controller that much life, but no more
    /// than the recipient had to give: a player's life total, a
    /// planeswalker's loyalty, or a creature's toughness, each read before
    /// the damage. Draining an almost-dead target gains only what was there.
    DrainLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Adds mana of one colour, however much a value says. Mana abilities use
    /// [`Self::AddMana`] with a fixed amount so the mana planner can read
    /// them without resolving anything; this is for the effects that cannot
    /// know their amount until they resolve.
    AddManaEqualTo {
        color: ManaColor,
        amount: ValueDef,
    },
    /// Counters unless the spell's own controller pays this much generic
    /// mana. `zone` is where a spell countered this way goes, which is the
    /// graveyard unless the card says otherwise.
    CounterUnlessPaid {
        object: EffectRecipientDef,
        amount: ValueDef,
        zone: ZoneKind,
    },
    AddCounters {
        object: EffectRecipientDef,
        kind: CounterKind,
        amount: ValueDef,
    },
    /// On resolution, choose two different basic land-type words and apply
    /// the resulting indefinite, noncopiable text change to the object.
    ChangeTextBasicLandType {
        object: EffectRecipientDef,
    },
    /// Replaces the source permanent's copiable values with the target's.
    /// Some copy effects, such as Thespian's Stage, retain the resolving
    /// ability as an exception to the copied values.
    BecomeCopyOf {
        object: EffectRecipientDef,
        retain_source_ability: bool,
    },
    OptionalManaPayment {
        cost: ManaCost,
        effect: &'static EffectDef,
    },
    /// The inverse of [`Self::OptionalManaPayment`]: `otherwise` happens
    /// unless the resolving object's controller pays. A controller who cannot
    /// pay is not asked, because there is nothing to decide.
    UnlessPaid {
        cost: ManaCost,
        otherwise: &'static EffectDef,
    },
    /// Stops the affected players casting noncreature spells for the rest of
    /// the turn.
    CannotCastNoncreatureSpellsThisTurn {
        player: EffectRecipientDef,
    },
    /// Lets the next sorcery its controller casts this turn be cast as
    /// though it had flash.
    GrantFlashToNextSorcery,
    /// An effect its controller may decline. Held by reference so that
    /// `EffectDef` does not grow a recursive inline copy of itself.
    May(&'static EffectDef),
    /// Exiles, remembering which object sent it there so a later clause can
    /// bring it back. This is the Oblivion Ring shape.
    ExileLinkedToSource {
        object: EffectRecipientDef,
    },
    /// Returns everything this ability's source exiled, to the named zone.
    /// A returned permanent keeps `grant` until end of turn, which is how
    /// Obzedat comes back ready to attack.
    ReturnLinkedExiles {
        zone: ZoneKind,
        grant: Option<KeywordAbility>,
    },
    /// Makes an object unblockable for the rest of the turn.
    MakeUnblockableThisTurn {
        object: EffectRecipientDef,
    },
    /// Gain control of a permanent for the rest of the turn. Control reverts
    /// in cleanup, so nothing needs to remember which effect took it.
    GainControlThisTurn {
        object: EffectRecipientDef,
    },
    /// Queues an effect for the next time that step begins.
    /// Runs `then` only if the condition holds where this effect resolves.
    /// A condition on a triggered ability is an intervening-if and is checked
    /// twice; this one is part of the effect and is checked once.
    IfCondition {
        condition: &'static TriggerConditionDef,
        then: &'static EffectDef,
    },
    AtNextStep {
        step: TurnStepDef,
        player: PlayerRelation,
        effect: &'static EffectDef,
    },
    /// Installs a triggered ability that listens from nowhere until its
    /// controller's next turn begins. The ability outlives the resolution
    /// that created it and does not belong to any permanent, which is what
    /// separates it from an ability a source grants.
    TriggerUntilYourNextTurn {
        ability: &'static AbilityDef,
    },
    /// A static prohibition: no spell or ability an opponent controls can
    /// make this ability's controller sacrifice a permanent.
    CannotBeForcedToSacrifice,
    /// This card costs that much less generic mana to cast. A static ability
    /// that works from the hand, where casting reads it.
    ReduceGenericCostBy(ValueDef),
    /// "Players can't cast spells or play lands with ..." A static
    /// prohibition read while play options are being offered, so a card it
    /// matches is never a legal action rather than a spell that fizzles.
    PlayersCantPlay(&'static ObjectPredicateDef),
    /// Adds a combat phase after the one now ending.
    AdditionalCombatPhase,
    /// Gives its controller an emblem, an object that sits outside every
    /// zone and does nothing but carry its abilities.
    CreateEmblem {
        emblem: CardDefinitionId,
    },
    /// Turns a double-faced permanent over to its other face.
    Transform {
        object: EffectRecipientDef,
    },
    /// Multiplies the amount of the event a replacement ability is replacing.
    /// This means nothing outside a replacement whose event carries an amount.
    MultiplyEventAmount(u8),
    /// An effect interpreted while replacing a prospective event, rather than
    /// when a spell or ability resolves from the stack.
    Replacement(ReplacementEffectDef),
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
    },
    /// Choose and store a card name for an object as it enters, the same
    /// replacement procedure as choosing a creature type.
    ChooseCardName {
        object: EffectRecipientDef,
    },
    /// "As this permanent enters, choose a player." The choice is recorded on
    /// the permanent, where [`PlayerRelation::ChosenPlayer`] reads it.
    ChoosePlayer {
        object: EffectRecipientDef,
        relation: PlayerRelation,
    },
    /// "You may have this permanent enter as a copy of ...". The copy is
    /// chosen as the permanent enters rather than targeted by the spell, so
    /// nothing about it can be responded to and declining is always allowed.
    /// `added_types` are kept on top of what is copied.
    CopyPermanentAsItEnters {
        object: ObjectPredicateDef,
        added_types: CardTypeSet,
    },
    /// Choose and store a creature type for an object as it enters. This is a
    /// replacement procedure rather than a resolving stack effect.
    ChooseCreatureType {
        object: EffectRecipientDef,
    },
    Apply {
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: EffectDurationDef,
    },
    /// A descriptive marker for an effect portion the shared vocabulary does
    /// not yet represent. The surrounding costs, targets, and timing can still
    /// remain declarative; clause coverage records whether and how it executes.
    Special(&'static str),
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

/// Turn structure used by beginning/end-of-step trigger declarations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TurnStepDef {
    Untap,
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

/// The committed event observed by a triggered ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TriggerEventDef {
    ZoneChanged {
        object: ObjectPredicateDef,
        from: Option<ZoneKind>,
        to: Option<ZoneKind>,
    },
    BecomesTapped(ObjectPredicateDef),
    /// A permanent was tapped to pay for one of its own mana abilities. This
    /// is narrower than [`Self::BecomesTapped`]: attacking or a tap effect
    /// does not produce mana and does not fire this.
    TappedForMana(ObjectPredicateDef),
    /// A creature was declared as an attacker. Every matching attacker in one
    /// declaration triggers separately, as CR 508.2 has them all attack at
    /// once rather than one at a time.
    Attacks(ObjectPredicateDef),
    /// The first time a matching creature attacks in a turn. An extra combat
    /// phase is the only way a creature attacks twice, which is exactly what
    /// the cards carrying this wording tend to grant.
    AttacksFirstTimeThisTurn(ObjectPredicateDef),
    SpellCast(ObjectPredicateDef),
    AbilityActivated(ObjectPredicateDef),
    StepBegins {
        step: TurnStepDef,
        player: PlayerRelation,
    },
    DamageDealt {
        source: ObjectPredicateDef,
        recipient: EffectRecipientDef,
    },
    /// A creature matching `source` dealt combat damage to a player. The
    /// damaged player is the event player and the amount is available as
    /// [`ValueDef::TriggerEventAmount`]. Only damage dealt in a combat damage
    /// step counts, which is what separates this from [`Self::DamageDealt`].
    CombatDamageDealtToPlayer {
        source: ObjectPredicateDef,
    },
    /// A permanent matching `source` dealt combat damage to this ability's own
    /// source. The player-facing variants cannot express this: a planeswalker
    /// is dealt combat damage as a permanent, and Vraska's retaliation is
    /// about damage arriving at her rather than at anyone's life total.
    CombatDamageDealtToSource {
        source: ObjectPredicateDef,
    },
    /// An object matching `source` dealt damage to a player by any means. The
    /// combat variant is the narrower case; this one also sees an ability's
    /// damage. `player` is read against the source's controller, so
    /// "an opponent" excludes damage the source deals to its own side.
    DamageDealtToPlayer {
        source: ObjectPredicateDef,
        player: PlayerRelation,
    },
    ManaAdded(PlayerRelation),
    /// A state trigger (CR 603.8). It has no event at all: it triggers
    /// whenever its ability's condition is true, and does not trigger again
    /// while it is already waiting or on the stack.
    StateCondition,
    /// This permanent turned over to the face carrying this ability, which is
    /// what "whenever this transforms into ..." names.
    TransformsIntoThisFace,
    /// A player gained life. The amount is available as
    /// [`ValueDef::TriggerEventAmount`].
    LifeGained(PlayerRelation),
    /// A creature dealt damage by this ability's source this turn died.
    DamagedCreatureDied,
    Special(&'static str),
}
