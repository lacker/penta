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
    /// Every activated ability of every creature card exiled with the
    /// granting object. Agatha's Soul Cauldron hands out a set the board
    /// decides rather than a clause anyone could write down, so unlike
    /// [`Self::Add`] the abilities are read at the moment the layer is walked
    /// and each one keeps its own grant identity.
    AddActivatedAbilitiesOfLinkedExiles,
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
    CardTypes(SetOperationDef<CardTypeSet>),
    Colors(SetOperationDef<ColorSet>),
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
    CannotBeCountered,
    /// "You may play lands from your graveyard." The mirror of
    /// [`Self::CannotPlay`]: a permission rather than a prohibition, matched
    /// against the same action and object the prohibition names, so a card
    /// that widens it to spells later says so in the same vocabulary.
    MayPlayFromGraveyard(PlayRestrictionDef),
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
    /// The affected player has no maximum hand size, so the cleanup step
    /// never asks them to discard. A player rule rather than an object one:
    /// it is found by walking the battlefield for statics naming that player.
    NoMaximumHandSize,
    /// Ascend (CR 702.131b). A permanent with it gives its controller the
    /// city's blessing while they control ten or more permanents. Written as
    /// a rule applied to the player rather than a keyword: nothing about
    /// combat or characteristics reads it, and what it produces is a state
    /// change rather than a continuous effect.
    Ascend,
    /// "If you would draw a card while your library has no cards in it, you
    /// win the game instead." A player rule rather than a permanent's,
    /// because the draw it replaces belongs to the player.
    WinsInsteadOfDrawingFromEmptyLibrary,
    /// The affected permanent's activated abilities can't be activated. Only
    /// the activations: its triggered and static clauses, and any mana it
    /// makes as a cost of something else, are untouched.
    CannotActivateAbilities,
    /// A creature matching this predicate cannot block the affected creature.
    CannotBeBlockedBy(ObjectPredicateDef),
    /// No Aura may attach to the affected permanent. This restricts both the
    /// Aura spell's targeting and whether an existing attachment stays legal,
    /// so an Aura already on the permanent falls off.
    CannotBeEnchanted,
    /// No new Aura may attach to the affected permanent, but an Aura already
    /// attached remains legal. Guardian Beast needs this narrower prohibition.
    CannotBecomeEnchanted,
    /// The affected creature cannot block at all.
    CannotBlock,
    /// How many creatures beyond the first the affected creature may block.
    ///
    /// [`u8::MAX`] means any number. Blocking one attacker is the default
    /// allowance every creature has, so this counts the extra ones -- a card
    /// reading "an additional creature" says one.
    MayBlockAdditionalCreatures(u8),
    /// Another player cannot gain control of the affected permanent.
    CannotChangeController,
    /// The affected Aura stays attached even when protection would otherwise
    /// make its host an illegal one. This is the printed exception that lets
    /// an Aura grant protection from its own color without falling off.
    RemainsAttachedThroughProtection,
    /// The affected creature may block only creatures matching this
    /// predicate.
    CanBlockOnly(ObjectPredicateDef),
    /// The affected creature cannot be declared as an attacker.
    CannotAttack,
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
    /// Nothing can block the affected creature.
    CannotBeBlocked,
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
    /// An unlimited prevention rule derived live while this static applied
    /// effect exists. Two-sided prevention is an
    /// [`AppliedEffectDef::Composite`] of source and recipient matchers.
    PreventDamage(DamageEventMatcherDef),
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

impl PlayRestrictionDef {
    #[must_use]
    pub const fn new(action: PlayActionMatcherDef, object: ObjectPredicateDef) -> Self {
        Self { action, object }
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
