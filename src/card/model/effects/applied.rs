// What a continuous effect applies to the object or player it names.
//
// Separated from the resolving vocabulary next door because the two answer
// different questions: an `EffectDef` says what happens once, while these
// leaves say what stays true. Included textually into `effects.rs`, so the
// paths and imports here are the parent module's.

/// An add, remove, or set operation over one set-valued characteristic.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SetOperationDef<T> {
    Add(T),
    Remove(T),
    Set(T),
}

/// Creature subtypes named by one layer-4 operation.
///
/// `all` remains semantic rather than expanding to the engine's current list,
/// so a permanent with all creature types also matches types added later.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreatureTypeSetDef {
    pub named: &'static [&'static str],
    pub all: bool,
}

impl CreatureTypeSetDef {
    #[must_use]
    pub const fn named(named: &'static [&'static str]) -> Self {
        Self { named, all: false }
    }

    pub const ALL: Self = Self {
        named: &[],
        all: true,
    };
}

/// One layer-6 operation over the affected object's abilities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityOperationDef {
    Add(&'static AbilityDef),
    Remove(AbilityPredicateDef),
    /// Every activated ability of each matching card exiled with the granting
    /// object. Agatha's Soul Cauldron names creature cards, while Myr Welder
    /// names every card in its linked pile. Unlike [`Self::Add`], the abilities
    /// are read at the moment the layer is walked and each one keeps its own
    /// grant identity.
    AddActivatedAbilitiesOfLinkedExiles(ObjectPredicateDef),
}

/// One layer-7 operation over power and toughness.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PowerToughnessOperationDef {
    /// Set base power and toughness in layer 7b.
    SetBase {
        power: ValueDef,
        toughness: ValueDef,
    },
    /// Set base power alone in layer 7b, leaving base toughness whatever it
    /// already was. "Has base power 0" says only half of what [`Self::SetBase`]
    /// says, and setting the other half to anything would be inventing it.
    SetBasePower(ValueDef),
    /// Set base toughness alone in layer 7b, the mirror of
    /// [`Self::SetBasePower`] and for the same reason: a card that changes
    /// only one half says only that half.
    SetBaseToughness(ValueDef),
    /// Define power, toughness, or both in layer 7a, which is what a
    /// characteristic-defining ability does (CR 604.3). Unlike every setter
    /// above it, this applies in every zone rather than only on the
    /// battlefield: a Lhurgoyf in a graveyard has the power its own text
    /// gives it, not the zero printed in its corner.
    ///
    /// One variant with two options rather than three variants, because a
    /// characteristic-defining ability names whichever halves it defines and
    /// leaves the other printed: `None` is "the card's own number stands".
    Define {
        power: Option<ValueDef>,
        toughness: Option<ValueDef>,
    },
    /// Modify power and toughness in layer 7c.
    Modify {
        power: ValueDef,
        toughness: ValueDef,
    },
    /// Exchange power and toughness in layer 7e, which CR 613.4e applies
    /// after every other power-and-toughness layer. It carries no values
    /// because it names none: two switches in effect at once cancel, so what
    /// matters is how many are applied rather than what each one says.
    Switch,
}

/// A typed continuous-effect leaf applied in its characteristic's rules
/// layer. Compound transformations use [`AppliedEffectDef::Composite`] so
/// each leaf keeps its own Add, Remove, or Set semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CharacteristicOperationDef {
    Abilities(AbilityOperationDef),
    /// Basic land-subtype operations in layer 4. `Set` additionally has the
    /// rules consequences of CR 305.7; `Add` and `Remove` do not.
    BasicLandTypes(SetOperationDef<&'static [BasicLandType]>),
    /// "This land is the chosen type." The basic land types are set to the
    /// one this permanent was told to be as it entered, so what it says is a
    /// layer-4 set with the same rules consequences and a subject nothing
    /// could have written down.
    ChosenBasicLandType,
    CardTypes(SetOperationDef<CardTypeSet>),
    Colors(SetOperationDef<ColorSet>),
    /// A color operation whose value is read from the source permanent.
    Color(SetOperationDef<ManaTypeDef>),
    CreatureTypes(SetOperationDef<CreatureTypeSetDef>),
    /// Named subtype operations across every subtype family. Unlike
    /// `CreatureTypes`, this can remove a noncreature subtype such as
    /// Equipment without disturbing the permanent's other subtypes.
    Subtypes(SetOperationDef<&'static [&'static str]>),
    PowerToughness(PowerToughnessOperationDef),
}

/// A continuous or rules-modifying effect applied to an object or player.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppliedEffectDef {
    /// Components applied to the same recipient for the same duration as one
    /// continuous effect.
    Composite(&'static [AppliedEffectDef]),
    /// One typed operation in the characteristic layer named by the leaf.
    Characteristic(CharacteristicOperationDef),
    /// One prohibition, permission, or prevention rule. Static rules are
    /// derived live from their source; resolving rules are stored with the
    /// authored duration alongside resolved characteristic changes.
    Rule(AppliedRuleDef),
}

/// Which defenders a rule applied to one player protects.
///
/// An unrestricted attacker-facing rule is instead applied to the creature
/// itself. Keeping the protected player's planeswalkers explicit preserves
/// the difference between "can't attack you" and "can't attack."
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AttackDefenderScopeDef {
    /// The rule is attached to an attacker and applies whichever defender it
    /// attacks. Player recipients cannot use this scope.
    Any,
    /// Only the affected player, not planeswalkers they control.
    AffectedPlayer,
    /// The affected player and planeswalkers they control.
    AffectedPlayerOrPlaneswalker,
}

/// One predicate-driven restriction on declaring an attacker.
///
/// `cost` is paid once for each matching attacker. `None` is a prohibition;
/// `Some` is the declaration cost that makes the attack legal. Several
/// restrictions compose by prohibiting if any one prohibits and otherwise
/// adding all of their costs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AttackRestrictionDef {
    pub attacker: ObjectPredicateDef,
    pub defender: AttackDefenderScopeDef,
    pub cost: Option<ManaCost>,
}

impl AttackRestrictionDef {
    #[must_use]
    pub const fn prohibit(
        attacker: ObjectPredicateDef,
        defender: AttackDefenderScopeDef,
    ) -> Self {
        Self {
            attacker,
            defender,
            cost: None,
        }
    }

    #[must_use]
    pub const fn unless_paid(
        attacker: ObjectPredicateDef,
        defender: AttackDefenderScopeDef,
        cost: ManaCost,
    ) -> Self {
        Self {
            attacker,
            defender,
            cost: Some(cost),
        }
    }

    /// The ordinary creature-facing "can't attack" prohibition.
    pub const CANNOT_ATTACK: Self = Self::prohibit(
        ObjectPredicateDef::Any,
        AttackDefenderScopeDef::Any,
    );
}

/// Which participant carries a restriction on one prospective block.
///
/// The rule is found on that participant through the ordinary applied-effect
/// walk. `counterpart` then describes the creature on the other side of the
/// block, which keeps blocker-facing and attacker-facing wording in one
/// declaration model without losing which object the effect affected.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BlockRestrictionSubjectDef {
    Blocker,
    Attacker,
}

/// Which counterpart makes a blocking restriction apply.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BlockRestrictionMatchDef {
    /// Every prospective counterpart, for an unqualified prohibition or cost.
    Any,
    /// Counterparts matching the predicate, as in "can't be blocked by Walls."
    Matching(ObjectPredicateDef),
    /// Counterparts outside the predicate, as in "can block only creatures
    /// with flying."
    Except(ObjectPredicateDef),
}

/// One predicate-driven restriction or cost on declaring a blocker.
///
/// `None` prohibits the matching block. `Some` is paid by the blocking
/// creature's controller. Restrictions on a blocker are charged once for
/// that blocker even if it can block several attackers; restrictions on an
/// attacker are charged once for each matching blocker assigned to it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockRestrictionDef {
    pub subject: BlockRestrictionSubjectDef,
    pub counterpart: BlockRestrictionMatchDef,
    pub cost: Option<ManaCost>,
}

impl BlockRestrictionDef {
    #[must_use]
    pub const fn prohibit(
        subject: BlockRestrictionSubjectDef,
        counterpart: BlockRestrictionMatchDef,
    ) -> Self {
        Self {
            subject,
            counterpart,
            cost: None,
        }
    }

    #[must_use]
    pub const fn unless_paid(
        subject: BlockRestrictionSubjectDef,
        counterpart: BlockRestrictionMatchDef,
        cost: ManaCost,
    ) -> Self {
        Self {
            subject,
            counterpart,
            cost: Some(cost),
        }
    }

    /// The ordinary creature-facing "can't block" prohibition.
    pub const CANNOT_BLOCK: Self = Self::prohibit(
        BlockRestrictionSubjectDef::Blocker,
        BlockRestrictionMatchDef::Any,
    );

    /// The ordinary attacker-facing "can't be blocked" prohibition.
    pub const CANNOT_BE_BLOCKED: Self = Self::prohibit(
        BlockRestrictionSubjectDef::Attacker,
        BlockRestrictionMatchDef::Any,
    );
}

/// A continuous rule modification applied to one object or player.
///
/// Keeping these leaves separate from characteristic operations makes their
/// layer-independent nature explicit without giving every printed wording a
/// top-level effect variant.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppliedRuleDef {
    /// The affected creature assigns no combat damage. This is a constraint
    /// on the assignment rather than a shield over the result: an attacker
    /// under it is not asked how to divide its damage at all, so trample has
    /// nothing to spill and no blocker is dealt a lethal share.
    AssignsNoCombatDamage,
    /// "Combat damage that would be dealt by creatures you control can't be
    /// prevented." A rule about every prevention rather than one of them,
    /// like the turn-wide version, but carried by a permanent and scoped to
    /// the combat damage the affected creatures deal. Protection prevents
    /// the damage it stops (CR 702.16e), so this switches that off too, for
    /// this damage only.
    CombatDamageCannotBePrevented,
    /// The affected creature assigns combat damage equal to its toughness
    /// rather than its power. Not a change to power: what it is remains what
    /// it is, and everything reading power sees the same number -- only the
    /// combat assignment reads the other one.
    AssignsCombatDamageEqualToToughness,
    CannotBeCountered,
    /// "If one or more tokens would be created under your control, twice
    /// that many of those tokens are created instead." A replacement on the
    /// creation rather than an effect of its own, so it applies to every
    /// clause that makes a token and to copies as much as to fresh ones.
    /// Several at once multiply, which is what each of them says on its own
    /// terms (CR 616.1).
    DoublesTokensCreated,
    /// "This creature can't be blocked except by N or more creatures."
    /// Menace is this rule with a two on it, printed as a keyword; the
    /// creatures that name a larger number write the clause out. It is a
    /// constraint on the finished declaration rather than on any one block,
    /// so a partial block is legal right up until it is the last one.
    CannotBeBlockedExceptByAtLeast(u8),
    /// "You may play lands from your graveyard." The mirror of
    /// [`Self::CannotPlay`]: a permission rather than a prohibition, matched
    /// against the same action and object the prohibition names, plus
    /// whatever bounds how often it may be used.
    MayPlayFromGraveyard(GraveyardPlayPermissionDef),
    /// "Each nonland card in your graveyard has escape. The escape cost is
    /// equal to the card's mana cost plus exile three other cards from your
    /// graveyard."
    ///
    /// A grant rather than a permission: what the affected cards gain is an
    /// alternative way to be cast, which the ability names in full because
    /// the cost is the card's own mana cost plus something the grant adds.
    /// It reaches cards in a graveyard, where nothing has a layer walk, so
    /// it is found by asking the battlefield rather than by asking the card.
    GrantsAlternativeCastFromGraveyard {
        object: ObjectPredicateDef,
        ability: &'static AbilityDef,
    },
    /// "You may play lands and cast spells from the top of your library."
    /// The same permission as [`Self::MayPlayFromGraveyard`] pointed at a
    /// different zone, plus what casting that way costs: Bolas's Citadel
    /// prints the replacement in the same sentence as the permission, and a
    /// permission without it would be a different card.
    MayPlayFromTopOfLibrary {
        restriction: PlayRestrictionDef,
        cost: TopOfLibraryCostDef,
    },
    /// "You may spend mana as though it were mana of any color to activate
    /// abilities of creatures you control." A player rule, found the same way
    /// [`Self::NoMaximumHandSize`] is found, whose scope is part of what it
    /// says: it reaches the activation costs of creatures that player
    /// controls and nothing else. The two other printed permissions this
    /// engine knows of -- North Star's and Grumgully's -- speak about spells
    /// and about one turn, so they will name their own scopes rather than
    /// widening this one.
    MaySpendManaAsAnyColorForCreatureAbilities,
    /// "You may look at the top card of your library any time." A player
    /// rule found the same way [`Self::NoMaximumHandSize`] is found, and
    /// separate from [`Self::MayPlayFromTopOfLibrary`] because the printed
    /// cards keep them separate: Oracle of Mul Daya lets you look without
    /// letting you cast, and a permission to play is not by itself a
    /// permission to look at what you are not playing.
    MayLookAtTopOfLibrary,
    /// "Play with the top card of your library revealed." The strictly
    /// louder half of [`Self::MayLookAtTopOfLibrary`]: the affected player's
    /// top card is public, so their opponent sees it as well. Both are
    /// needed because the printed cards differ -- Bolas's Citadel shows the
    /// card to nobody but its controller, and Courser of Kruphix shows it to
    /// the table.
    PlaysWithTopOfLibraryRevealed,
    /// "If a land or Bird you control entering the battlefield causes a
    /// triggered ability of a permanent you control to trigger, that ability
    /// triggers an additional time." A player rule: nothing about the
    /// doubled permanent is changed, and what decides the doubling is who
    /// controls both it and the arriving object.
    ///
    /// Only an entry to the battlefield is watched. Every printed card of
    /// this shape says "entering the battlefield", and reading a wider set
    /// of events would double abilities their text does not reach.
    TriggersAnAdditionalTime(&'static AdditionalTriggerDef),
    /// "You may play an additional land on each of your turns." A player
    /// rule found the way the hand-size one is found, and counted rather
    /// than merely present: two of them are two extra lands, which is what
    /// makes it a number instead of a flag.
    MayPlayAdditionalLands(u8),
    /// "You may play any number of lands on each of your turns." Not a
    /// number: Fastbond removes the bound rather than raising it, and a
    /// count large enough to look unbounded would still be one.
    MayPlayAnyNumberOfLands,
    /// "Each opponent can't draw more than one card each turn." A player
    /// rule found the same way, and a bound rather than a flag: a draw past
    /// the number simply does not happen, so nothing watching for a draw
    /// fires and no draw replacement is spent on it. Two such rules leave
    /// the smaller bound standing.
    CannotDrawMoreThanEachTurn(u8),
    /// The affected player has no maximum hand size, so the cleanup step
    /// never asks them to discard. A player rule rather than an object one:
    /// it is found by walking the battlefield for statics naming that player.
    NoMaximumHandSize,
    /// The affected player reveals each card they draw. This is a continuous
    /// rule rather than a trigger: the reveal happens as the draw completes,
    /// before either player receives priority.
    RevealsDrawnCards,
    /// Ascend (CR 702.131b). A permanent with it gives its controller the
    /// city's blessing while they control ten or more permanents. Written as
    /// a rule applied to the player rather than a keyword: nothing about
    /// combat or characteristics reads it, and what it produces is a state
    /// change rather than a continuous effect.
    Ascend,
    /// "You may activate her loyalty abilities any time you could cast an
    /// instant." A permission on the affected planeswalker rather than a
    /// timing printed on each ability: what it lifts is the sorcery-speed
    /// window CR 606.3 imposes on every loyalty ability, and the one-per-turn
    /// limit beside it is untouched.
    MayActivateLoyaltyAnyTime,
    /// The affected permanent's activated abilities can't be activated. Only
    /// the activations: its triggered and static clauses, and any mana it
    /// makes as a cost of something else, are untouched.
    CannotActivateAbilities,
    /// No Aura may attach to the affected permanent. This restricts both the
    /// Aura spell's targeting and whether an existing attachment stays legal,
    /// so an Aura already on the permanent falls off.
    CannotBeEnchanted,
    /// No new Aura may attach to the affected permanent, but an Aura already
    /// attached remains legal. Guardian Beast needs this narrower prohibition.
    CannotBecomeEnchanted,
    /// How many creatures beyond the first the affected creature may block.
    ///
    /// [`u8::MAX`] means any number. Blocking one attacker is the default
    /// allowance every creature has, so this counts the extra ones -- a card
    /// reading "an additional creature" says one.
    MayBlockAdditionalCreatures(u8),
    /// "This creature crews Vehicles as though its power were N greater."
    /// The bonus is only for the crewing: what the creature is worth in
    /// combat, or to anything else that reads its power, is untouched.
    CrewsAsThoughPowerGreater(u8),
    /// Another player cannot gain control of the affected permanent.
    CannotChangeController,
    /// The affected Aura stays attached even when protection would otherwise
    /// make its host an illegal one. This is the printed exception that lets
    /// an Aura grant protection from its own color without falling off.
    RemainsAttachedThroughProtection,
    /// A predicate-driven blocker prohibition or declaration cost. The rule's
    /// subject records whether the affected object is the prospective blocker
    /// or attacker; its matcher describes the creature on the other side.
    BlockRestriction(BlockRestrictionDef),
    /// A predicate-driven attacker prohibition or declaration cost. The
    /// recipient determines whether the rule is attached to one attacker or
    /// to a protected player; the defender scope makes that distinction
    /// explicit and keeps planeswalker attacks honest.
    AttackRestriction(AttackRestrictionDef),
    /// Defender does not stop the affected creature from attacking.
    ///
    /// A permission rather than an ability removal: the creature keeps the
    /// keyword, so anything reading "a creature with defender" still finds
    /// one. Every other reason it cannot attack still applies.
    MayAttackDespiteDefender,
    /// Summoning sickness does not stop the affected creature from attacking.
    ///
    /// "As though it had haste" and haste itself are not the same thing: this
    /// buys the attack only. The creature still cannot use an ability with
    /// {T} or {Q} in its cost, and anything reading "a creature with haste"
    /// still does not find one.
    MayAttackAsThoughHasty,
    /// Every creature matching this predicate that is able to block the
    /// affected creature must do so.
    ///
    /// A requirement never beats a restriction (CR 509.1c): "able" is read
    /// from the same legality that offers a block in the first place, so a
    /// tapped creature, one that cannot block at all, or one that cannot
    /// block *this* attacker is simply not required. What the requirement
    /// does is take away the alternatives -- a creature that could block the
    /// affected one may not be declared against anything else.
    MustBeBlockedBy(ObjectPredicateDef),
    /// The mirror of [`Self::MustBeBlockedBy`], read from the blocker: this
    /// creature blocks every attacker it legally can. "Able" is read the same
    /// way -- from the blocks it is actually offered -- so an attacker it
    /// cannot block does not hold the declaration open.
    MustBlockEachAttackerIfAble,
    /// Damage a matching source would deal to the affected permanent's
    /// controller is dealt to that permanent instead. The redirection is read
    /// live, so a condition on the recipient -- "as long as this creature is
    /// untapped" -- turns it off without the permanent being touched.
    RedirectPlayerDamageToThis(DamageSourceGroupDef),
    /// Damage the named source would deal to the affected player is dealt to
    /// the named destination instead. Resolving this rule freezes both object
    /// references for the authored duration.
    RedirectDamageFromTo {
        source: ObjectRefDef,
        destination: ObjectRefDef,
    },
    /// The affected player may untap at most one matching permanent during
    /// their untap step.
    ///
    /// A cap on the turn-based action, not a prohibition on untapping: the
    /// player still chooses which one, and anything that untaps a permanent
    /// outside the untap step is untouched. Several of these compose, each
    /// capping its own group.
    UntapAtMostOne(ObjectPredicateDef),
    /// The affected player cannot take matching cast or land-play actions.
    /// The recipient and lifetime live on `StaticApply` or `Apply`, just as
    /// they do for object-facing applied rules.
    CannotPlay(PlayRestrictionDef),
    /// Regeneration shields can still be created, but cannot replace a
    /// destruction while this rule applies. CR 701.19c.
    CannotRegenerate,
    /// The affected permanent is skipped by its controller's ordinary
    /// turn-based untap procedure. Other spells and abilities can still
    /// untap it.
    DoesNotUntapDuringUntapStep,
    /// The affected permanent's controller may choose to leave it tapped
    /// during their untap step. Unlike
    /// [`Self::DoesNotUntapDuringUntapStep`] this is a choice rather than a
    /// prohibition, so declining is what the printed cards are paying for.
    MayChooseNotToUntap,
    /// Caps matching damage while this rule applies. Unlike
    /// [`Self::PreventDamage`] nothing is spent: every matching event is
    /// limited for as long as the rule is there.
    LimitDamage {
        matcher: DamageEventMatcherDef,
        limit: DamageLimitDef,
    },
    /// "You gain protection from everything until your next turn." A
    /// player's protection, which is the object keyword's shorter list: no
    /// damage from a matching source reaches them, no matching spell or
    /// ability may target them, and no matching Aura may enchant them.
    /// Nothing about blocking or destruction applies to a player.
    ///
    /// Unlike hexproof this stops the protected player's own spells too:
    /// protection asks about the source's qualities rather than about who
    /// controls it, which is what makes "from everything" a real cost.
    PlayerProtectionFrom(ObjectPredicateDef),
    /// "That player can't gain life for the rest of the game." A prohibition
    /// on the player rather than a replacement of the gain: nothing is
    /// multiplied down to zero, the life simply never arrives, and nothing
    /// watching for a life gain sees one.
    CannotGainLife,
    /// "You may cast <these> spells as though they had flash." A timing
    /// permission rather than a granted keyword: nothing about the card
    /// changes, and the permission belongs to the player it was given to for
    /// as long as its own duration lasts.
    MayCastAsThoughItHadFlash(ObjectPredicateDef),
    /// "If that creature would die this turn, exile it instead." A
    /// replacement over the death itself rather than over the damage that
    /// caused it: it applies however the creature would die, and it outlives
    /// the damage, which is why it is a rule with a duration instead of a
    /// property of the damage event. A finality counter says the same thing
    /// permanently (CR 122.1h).
    ExileInsteadOfDying,
    /// An unlimited prevention rule derived live while this static applied
    /// effect exists. Two-sided prevention is an
    /// [`AppliedEffectDef::Composite`] of source and recipient matchers.
    PreventDamage(DamageEventMatcherDef),
}

impl AppliedRuleDef {
    /// The common object-facing rule used by Pacifism and similar effects.
    pub const CANNOT_ATTACK: Self = Self::AttackRestriction(AttackRestrictionDef::CANNOT_ATTACK);

    /// The common object-facing rule used by Pacifism and similar effects.
    pub const CANNOT_BLOCK: Self =
        Self::BlockRestriction(BlockRestrictionDef::CANNOT_BLOCK);

    /// The common attacker-facing rule for complete unblockability.
    pub const CANNOT_BE_BLOCKED: Self =
        Self::BlockRestriction(BlockRestrictionDef::CANNOT_BE_BLOCKED);

    /// The affected attacker cannot be blocked by matching creatures.
    #[must_use]
    pub const fn cannot_be_blocked_by(blocker: ObjectPredicateDef) -> Self {
        Self::BlockRestriction(BlockRestrictionDef::prohibit(
            BlockRestrictionSubjectDef::Attacker,
            BlockRestrictionMatchDef::Matching(blocker),
        ))
    }

    /// The affected blocker may block only matching attackers.
    #[must_use]
    pub const fn can_block_only(attacker: ObjectPredicateDef) -> Self {
        Self::BlockRestriction(BlockRestrictionDef::prohibit(
            BlockRestrictionSubjectDef::Blocker,
            BlockRestrictionMatchDef::Except(attacker),
        ))
    }
}

/// Which kind of play action a restriction matches.
///
/// Keeping this axis separate from the object predicate lets one rule cover
/// both halves of text such as City in a Bottle while a cast-only rule such as
/// Aurelia's Fury leaves land plays untouched.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayActionMatcherDef {
    Any,
    CastSpell,
    PlayLand,
    /// Activating an ability that is not a mana ability. Not a play action
    /// at all -- which is why [`Self::matches`] never admits it -- but the
    /// same prohibition machinery answers it, because a rule barring one is
    /// aimed at a player exactly the way a rule barring a cast is.
    ActivateNonManaAbility,
}

impl PlayActionMatcherDef {
    #[must_use]
    pub const fn matches(self, action: PlayActionKind) -> bool {
        // `Any` is any *play* action; an activation is asked about
        // separately because it is not one.
        if matches!(self, Self::ActivateNonManaAbility) {
            return false;
        }
        matches!(self, Self::Any)
            || matches!(
                (self, action),
                (Self::CastSpell, PlayActionKind::CastSpell)
                    | (Self::PlayLand, PlayActionKind::PlayLand)
            )
    }
}

/// A prohibition over one play-action family and one object predicate.
///
/// This deliberately models prohibition rather than a per-turn quota. A
/// future Deafening Silence-style limit can share these two match axes, but
/// also needs matching cast history rather than being approximated as a
/// boolean prohibition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlayRestrictionDef {
    pub action: PlayActionMatcherDef,
    pub object: ObjectPredicateDef,
    /// "... only any time they could cast a sorcery." A timing restriction
    /// rather than a flat prohibition: matching plays are barred only at the
    /// moments a sorcery could not be cast, which is every moment except the
    /// player's own main phase with an empty stack.
    pub only_at_sorcery_speed: bool,
    /// The restriction starts after this many spells have already been cast
    /// by the affected player this turn. Zero is an unconditional ban.
    pub minimum_spells_cast_this_turn: u16,
}

/// What a spell cast off the top of a library costs its caster.
///
/// A land played from up there costs nothing either way: only spells have a
/// mana cost for this to replace.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TopOfLibraryCostDef {
    /// Its own cost, as printed. Future Sight's permission says no more than
    /// that you may play what is up there.
    Printed,
    /// "Pay life equal to its mana value rather than pay its mana cost."
    /// The mana cost goes away and the life takes its place, so a spell
    /// nobody has the life for is not castable this way at all.
    LifeEqualToManaValue,
}

/// One "that ability triggers an additional time" clause: which arrival
/// does the causing, and whose triggered ability is doubled.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AdditionalTriggerDef {
    /// What has to be entering the battlefield. Read from the affected
    /// player's perspective, so "you control" means they do.
    pub entering: ObjectPredicateDef,
    /// The permanent whose triggered ability is doubled, read from the same
    /// perspective. A trigger whose source is not a permanent on the
    /// battlefield matches nothing here.
    pub permanent: ObjectPredicateDef,
}

/// A permission to play cards out of a graveyard, and what bounds it.
///
/// Crucible's line is unbounded: as many lands as your land drops allow, on
/// anybody's turn. Lurrus prints the other shape -- one such spell, and only
/// during your own turns -- and the difference belongs to the permission
/// rather than to what it names.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GraveyardPlayPermissionDef {
    pub restriction: PlayRestrictionDef,
    /// How many plays each qualifying turn allows. `None` is as many as the
    /// rest of the rules permit.
    pub per_turn: Option<u8>,
    /// Whether it opens only on its controller's own turns.
    pub your_turns_only: bool,
    /// "If you do, it gains ...": what the permanent played this way carries
    /// afterwards. It belongs to the permission because the permission is
    /// the only thing that knows a play was made under it -- and it outlives
    /// the permission's own source, which is why it rides on the permanent
    /// rather than being read back off the card that allowed it.
    pub grants: Option<&'static AppliedEffectDef>,
}

impl GraveyardPlayPermissionDef {
    #[must_use]
    pub const fn unlimited(restriction: PlayRestrictionDef) -> Self {
        Self {
            restriction,
            per_turn: None,
            your_turns_only: false,
            grants: None,
        }
    }

    /// "Once during each of your turns, you may cast ..."
    #[must_use]
    pub const fn once_each_of_your_turns(restriction: PlayRestrictionDef) -> Self {
        Self {
            restriction,
            per_turn: Some(1),
            your_turns_only: true,
            grants: None,
        }
    }

    /// The same permission, with what it played gaining `effect`.
    #[must_use]
    pub const fn granting(mut self, effect: &'static AppliedEffectDef) -> Self {
        self.grants = Some(effect);
        self
    }
}

impl PlayRestrictionDef {
    #[must_use]
    pub const fn new(action: PlayActionMatcherDef, object: ObjectPredicateDef) -> Self {
        Self {
            action,
            object,
            only_at_sorcery_speed: false,
            minimum_spells_cast_this_turn: 0,
        }
    }

    /// The same restriction, narrowed to the moments a sorcery could not be
    /// cast. This is what "can cast spells only any time they could cast a
    /// sorcery" prints.
    #[must_use]
    pub const fn only_at_sorcery_speed(mut self) -> Self {
        self.only_at_sorcery_speed = true;
        self
    }

    /// Begin prohibiting matching plays after `amount` spells this turn.
    #[must_use]
    pub const fn after_spells_cast(mut self, amount: u16) -> Self {
        self.minimum_spells_cast_this_turn = amount;
        self
    }
}

impl AppliedEffectDef {
    #[must_use]
    pub const fn add_ability(ability: &'static AbilityDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        ))
    }

    #[must_use]
    pub const fn remove_abilities(predicate: AbilityPredicateDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Remove(predicate),
        ))
    }

    #[must_use]
    pub const fn add_basic_land_types(types: &'static [BasicLandType]) -> Self {
        Self::Characteristic(CharacteristicOperationDef::BasicLandTypes(
            SetOperationDef::Add(types),
        ))
    }

    #[must_use]
    pub const fn set_chosen_basic_land_type() -> Self {
        Self::Characteristic(CharacteristicOperationDef::ChosenBasicLandType)
    }

    #[must_use]
    pub const fn set_basic_land_types(types: &'static [BasicLandType]) -> Self {
        Self::Characteristic(CharacteristicOperationDef::BasicLandTypes(
            SetOperationDef::Set(types),
        ))
    }

    #[must_use]
    pub const fn add_card_types(types: CardTypeSet) -> Self {
        Self::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
            types,
        )))
    }

    /// "It's an enchantment. (It's not a creature.)" Replaces the type line
    /// rather than adding to it, which is what takes the creature away.
    #[must_use]
    pub const fn set_card_types(types: CardTypeSet) -> Self {
        Self::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Set(
            types,
        )))
    }

    #[must_use]
    pub const fn add_creature_types(types: CreatureTypeSetDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::CreatureTypes(
            SetOperationDef::Add(types),
        ))
    }

    #[must_use]
    pub const fn set_creature_types(types: CreatureTypeSetDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::CreatureTypes(
            SetOperationDef::Set(types),
        ))
    }

    /// Remove the named subtypes, regardless of which card-type family they
    /// belong to. This is the layer-4 operation used by "no longer an
    /// Equipment" animations.
    #[must_use]
    pub const fn remove_subtypes(types: &'static [&'static str]) -> Self {
        Self::Characteristic(CharacteristicOperationDef::Subtypes(
            SetOperationDef::Remove(types),
        ))
    }

    /// "In addition to its other colors", which adds rather than replaces.
    #[must_use]
    pub const fn add_colors(colors: ColorSet) -> Self {
        Self::Characteristic(CharacteristicOperationDef::Colors(SetOperationDef::Add(
            colors,
        )))
    }

    #[must_use]
    pub const fn set_colors(colors: ColorSet) -> Self {
        Self::Characteristic(CharacteristicOperationDef::Colors(SetOperationDef::Set(
            colors,
        )))
    }

    #[must_use]
    pub const fn set_color(color: ManaTypeDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::Color(SetOperationDef::Set(color)))
    }

    #[must_use]
    pub const fn set_base_power_toughness(power: ValueDef, toughness: ValueDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase { power, toughness },
        ))
    }

    #[must_use]
    pub const fn set_base_power(power: ValueDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBasePower(power),
        ))
    }

    #[must_use]
    pub const fn set_base_toughness(toughness: ValueDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBaseToughness(toughness),
        ))
    }

    /// CR 613.4e: exchange power and toughness after every other layer.
    #[must_use]
    pub const fn switch_power_toughness() -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::Switch,
        ))
    }

    /// A characteristic-defining power, with the printed toughness left
    /// standing.
    #[must_use]
    pub const fn define_power(power: ValueDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::Define {
                power: Some(power),
                toughness: None,
            },
        ))
    }

    /// A characteristic-defining toughness, with the printed power left
    /// standing.
    #[must_use]
    pub const fn define_toughness(toughness: ValueDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::Define {
                power: None,
                toughness: Some(toughness),
            },
        ))
    }

    /// A characteristic-defining power and toughness, each given by its own
    /// amount: "its toughness is equal to that number plus 1" is a different
    /// amount from the power beside it, not an offset applied to it.
    #[must_use]
    pub const fn define_power_toughness(power: ValueDef, toughness: ValueDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::Define {
                power: Some(power),
                toughness: Some(toughness),
            },
        ))
    }

    #[must_use]
    pub const fn modify_power_toughness(power: ValueDef, toughness: ValueDef) -> Self {
        Self::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::Modify { power, toughness },
        ))
    }

    #[must_use]
    pub const fn prevent_damage_from(source: ObjectPredicateDef) -> Self {
        Self::Rule(AppliedRuleDef::PreventDamage(
            DamageEventMatcherDef::from_matching_to_affected(source),
        ))
    }

    #[must_use]
    pub const fn prevent_combat_damage_from(source: ObjectPredicateDef) -> Self {
        Self::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef {
            kind: DamageKindDef::Combat,
            source: DamageSourceMatcherDef::Matching(source),
            recipient: DamageRecipientMatcherDef::AffectedObject,
        }))
    }
}
