/// An object reference evaluated in the resolving effect's context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectRefDef {
    /// The game object from which the resolving spell or ability originated.
    Source,
    /// The object created by the named reference's next zone change,
    /// provided that exact successor still exists.
    ///
    /// Ordinary references never follow a zone change: the successor is a
    /// new game object. This explicit reference is for printed instructions
    /// such as a dies trigger's "return it", which are allowed to find the
    /// card created by that particular move. It follows one move only, so a
    /// card that moves again before resolution is no longer found.
    ZoneChangeSuccessor(ZoneChangeReferenceDef),
    /// The destination object produced by the triggering zone change,
    /// provided that exact object still exists.
    ///
    /// Zone-change events preserve the characteristics needed to match their
    /// printed trigger and expose the newly created object here. This is the
    /// event-relative counterpart of [`Self::ZoneChangeSuccessor`],
    /// for instructions that act on a different object whose move caused the
    /// trigger. A later move is still a new object and is not followed.
    ZoneChangeResultOfTriggeringObject,
    /// The object whose continuous effect granted the resolving ability.
    ///
    /// This deliberately names that exact battlefield incarnation rather than
    /// following a later zone change. A granted ability can sacrifice its
    /// granter as a cost and still attribute its effect to that granter using
    /// last-known information after the granter has left the battlefield.
    AbilityGrantSource,
    /// The spell or ability object currently resolving. This is distinct from
    /// [`Self::Source`], which names its originating game object.
    ResolvingObject,
    /// One object saved by an earlier choice in this resolution.
    Binding(ObjectBindingIndex),
    /// One object paid for the resolving spell's additional costs, by payment
    /// order. This names that exact object incarnation so characteristic
    /// reads can use last-known information after the payment moves it.
    AdditionalCostObject(AdditionalCostObjectIndex),
    AttachedToSource,
    Target(TargetIndex),
    TriggeringObject,
    /// What a damage event's damage was dealt to. Distinct from
    /// [`Self::TriggeringObject`], which for a damage trigger is the source
    /// that dealt it: "whenever this creature deals combat damage to a
    /// creature, exile that creature" names both, and never the same one.
    DamagedObject,
    /// The permanent a targeted stack ability came from. Read after that
    /// ability has left the stack, which is what "if a permanent's ability is
    /// countered this way, destroy that permanent" asks for; a targeted spell
    /// has no such source and resolves to nothing.
    SourceOfTargetedStackObject(TargetIndex),
}

/// A player reference evaluated in the resolving effect's context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerRefDef {
    /// The controller captured by the resolving spell or ability.
    EffectController,
    /// The player the resolving ability's source Aura enchants.
    EnchantedPlayer,
    EventPlayer,
    /// A target slot that directly names a player.
    Target(TargetIndex),
    /// The current controller of an object, falling back to last-known
    /// information. A target that directly names a player resolves to that
    /// player, preserving the ordinary meaning of "that player or its
    /// controller" selectors.
    ControllerOf(ObjectRefDef),
    /// The owner of an object, using last-known information when necessary.
    OwnerOf(ObjectRefDef),
    /// The one player the effect's controller is not. A printed "an
    /// opponent" is a choice in a game with several; in this two-player
    /// engine it names exactly one player, and there is nothing to ask.
    Opponent,
    /// "Each player other than its controller." The mirror of
    /// [`Self::ControllerOf`], read the same way and with the same fallback
    /// to last-known information: with two players, everybody who is not
    /// the named object's controller is exactly one player.
    OpponentOf(ObjectRefDef),
}

/// A set of players. Relations are measured from the resolving effect's
/// controller unless the relation itself names an event or chosen player.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerSetDef {
    All,
    One(PlayerRefDef),
    Related(PlayerRelation),
    /// The player members of a target slot. Mixed any-target slots are
    /// intentionally filtered rather than relying on each effect to ignore
    /// object members implicitly.
    LegalTargets(TargetIndex),
}

/// A set of objects selected without targeting.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectSetDef {
    One(ObjectRefDef),
    /// The nonplayer members of a target slot. This is the explicit object
    /// projection for a mixed any-target declaration.
    LegalTargets(TargetIndex),
    /// A set of objects saved by an earlier choice or partition in this
    /// resolution.
    Binding(ObjectSetBindingIndex),
    /// Cards the named player actually drew this turn that remain in that
    /// player's hand. The identities are rules history rather than a zone
    /// characteristic: a card that began the turn in hand is not included,
    /// and a drawn card that left and returned is a new object and no longer
    /// belongs to this set.
    CardsDrawnThisTurnInHand(PlayerRefDef),
    /// The members of a binding that match a predicate. "Put a creature card
    /// from among them into your hand" names a subset of what a mill just
    /// bound, which neither a plain binding nor a zone query can say: the
    /// query would reach cards that were already there.
    MatchingBinding {
        binding: ObjectSetBindingIndex,
        object: ObjectPredicateDef,
    },
    /// The permanents a stack object is targeting. "Gain control of those
    /// permanents" names what the spell that triggered this picked, which
    /// neither a query nor a binding can say: nothing chose them here, and
    /// the spell is still on the stack holding them.
    PermanentsTargetedBy(ObjectRefDef),
    Query(ObjectQueryDef),
    /// Every battlefield permanent sharing the referenced object's effective
    /// name, including the referenced object itself.
    SharingNameWith(ObjectRefDef),
    /// The newest matching card in one player's graveyard. A graveyard is a
    /// pile, so "the top creature card" is the last creature card put there
    /// rather than a choice among them.
    TopOfGraveyardMatching {
        player: PlayerRefDef,
        object: ObjectPredicateDef,
    },
    /// Every card in one player's zone whose name matches something in a
    /// bound set. "Search that player's library for all cards with the same
    /// name" reads the set the graveyard gave up.
    SharingNameWithBinding {
        binding: ObjectSetBindingIndex,
        player: PlayerRefDef,
        zone: ZoneKind,
    },
    /// The oldest card in a player's graveyard, which is what "the bottom
    /// card of target player's graveyard" names. Nothing is chosen: a
    /// graveyard has one bottom card, and an empty one has none.
    BottomOfGraveyard(PlayerRefDef),
    /// The cards exiled with the ability's own source that match this
    /// predicate. "A creature card exiled with this creature" names a pile
    /// no query can find: what puts a card in it is which permanent exiled
    /// it, not where it is or what it looks like.
    LinkedExiles(ObjectPredicateDef),
}

/// The typed subject of an effect. A target slot remains its own category
/// because one slot can legally contain players and objects, and because its
/// contents must be legality-checked again on resolution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectRecipientSetDef {
    LegalTargets(TargetIndex),
    Objects(ObjectSetDef),
    Players(PlayerSetDef),
    /// "Each opponent and each creature they control." One clause naming
    /// both kinds at once, which neither of the two above can say: an object
    /// set names objects, and a player set names players. The damage matcher
    /// beside it already draws the same pair for the other direction.
    PlayersAndCreaturesTheyControl(PlayerSetDef),
    /// "The player or planeswalker it's attacking." Which of the two it is
    /// is settled by the declaration rather than by the clause, so neither
    /// an object set nor a player set can say it alone. Nothing at all when
    /// the named object is not attacking.
    DefenderOf(ObjectRefDef),
}

/// An object or player set affected by an effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EffectRecipientDef(pub EffectRecipientSetDef);

// These const-friendly spellings keep card declarations compact while the
// runtime receives the compositional reference/query model above.
#[allow(non_snake_case, non_upper_case_globals)]
impl EffectRecipientDef {
    pub const Source: Self = Self::object(ObjectRefDef::Source);
    pub const SourceZoneChangeSuccessor: Self =
        Self::zone_change_successor(ZoneChangeReferenceDef::Source);
    pub const AttachedPermanent: Self = Self::object(ObjectRefDef::AttachedToSource);
    pub const Controller: Self = Self::player(PlayerRefDef::EffectController);
    pub const EnchantedPlayer: Self = Self::player(PlayerRefDef::EnchantedPlayer);
    pub const Opponent: Self = Self::players(PlayerSetDef::Related(PlayerRelation::Opponent));
    pub const EachPlayer: Self = Self::players(PlayerSetDef::All);
    pub const EachOpponentAndTheirCreatures: Self = Self(
        EffectRecipientSetDef::PlayersAndCreaturesTheyControl(PlayerSetDef::Related(
            PlayerRelation::Opponent,
        )),
    );
    pub const TriggeringObject: Self = Self::object(ObjectRefDef::TriggeringObject);
    pub const TriggeringZoneChangeResult: Self =
        Self::object(ObjectRefDef::ZoneChangeResultOfTriggeringObject);
    pub const DamagedObjectZoneChangeSuccessor: Self =
        Self::zone_change_successor(ZoneChangeReferenceDef::DamagedObject);
    pub const ControllerOfTriggeringObject: Self =
        Self::player(PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject));
    pub const EventPlayer: Self = Self::player(PlayerRefDef::EventPlayer);
    /// What the source is attacking, which is a player or a planeswalker.
    pub const DefenderOfSource: Self =
        Self(EffectRecipientSetDef::DefenderOf(ObjectRefDef::Source));

    #[must_use]
    pub const fn object(object: ObjectRefDef) -> Self {
        Self(EffectRecipientSetDef::Objects(ObjectSetDef::One(object)))
    }

    #[must_use]
    pub const fn zone_change_successor(reference: ZoneChangeReferenceDef) -> Self {
        Self::object(ObjectRefDef::ZoneChangeSuccessor(reference))
    }

    #[must_use]
    pub const fn binding_zone_change_successor(binding: ObjectBindingIndex) -> Self {
        Self::zone_change_successor(ZoneChangeReferenceDef::Binding(binding))
    }

    #[must_use]
    pub const fn objects(objects: ObjectSetDef) -> Self {
        Self(EffectRecipientSetDef::Objects(objects))
    }

    #[must_use]
    pub const fn target_objects(target: TargetIndex) -> Self {
        Self::objects(ObjectSetDef::LegalTargets(target))
    }

    #[must_use]
    pub const fn player(player: PlayerRefDef) -> Self {
        Self::players(PlayerSetDef::One(player))
    }

    #[must_use]
    pub const fn players(players: PlayerSetDef) -> Self {
        Self(EffectRecipientSetDef::Players(players))
    }

    #[must_use]
    pub const fn target_players(target: TargetIndex) -> Self {
        Self::players(PlayerSetDef::LegalTargets(target))
    }

    #[must_use]
    pub const fn legal_target(self) -> Option<TargetIndex> {
        match self.0 {
            EffectRecipientSetDef::LegalTargets(target) => Some(target),
            EffectRecipientSetDef::Objects(_)
            | EffectRecipientSetDef::Players(_)
            | EffectRecipientSetDef::DefenderOf(_)
            | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_) => None,
        }
    }

    #[must_use]
    pub const fn object_reference(self) -> Option<ObjectRefDef> {
        match self.0 {
            EffectRecipientSetDef::Objects(ObjectSetDef::One(reference)) => Some(reference),
            EffectRecipientSetDef::LegalTargets(_)
            | EffectRecipientSetDef::DefenderOf(_)
            | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
            | EffectRecipientSetDef::Objects(
                ObjectSetDef::Binding(_)
                | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                | ObjectSetDef::MatchingBinding { .. }
                | ObjectSetDef::PermanentsTargetedBy(_)
                | ObjectSetDef::LinkedExiles(_)
                | ObjectSetDef::BottomOfGraveyard(_)
                | ObjectSetDef::LegalTargets(_)
                | ObjectSetDef::Query(_)
                | ObjectSetDef::SharingNameWith(_)
                | ObjectSetDef::SharingNameWithBinding { .. }
                | ObjectSetDef::TopOfGraveyardMatching { .. },
            )
            | EffectRecipientSetDef::Players(_) => None,
        }
    }

    #[must_use]
    pub const fn object_query(self) -> Option<ObjectQueryDef> {
        match self.0 {
            EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => Some(query),
            EffectRecipientSetDef::LegalTargets(_)
            | EffectRecipientSetDef::DefenderOf(_)
            | EffectRecipientSetDef::Objects(
                ObjectSetDef::One(_)
                | ObjectSetDef::Binding(_)
                | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                | ObjectSetDef::MatchingBinding { .. }
                | ObjectSetDef::PermanentsTargetedBy(_)
                | ObjectSetDef::LinkedExiles(_)
                | ObjectSetDef::BottomOfGraveyard(_)
                | ObjectSetDef::LegalTargets(_)
                | ObjectSetDef::SharingNameWith(_)
                | ObjectSetDef::SharingNameWithBinding { .. }
                | ObjectSetDef::TopOfGraveyardMatching { .. },
            )
            | EffectRecipientSetDef::Players(_)
            | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_) => None,
        }
    }

    #[must_use]
    pub const fn object_binding(self) -> Option<ObjectBindingIndex> {
        match self.object_reference() {
            Some(ObjectRefDef::Binding(binding)) => Some(binding),
            Some(
                ObjectRefDef::Source
                | ObjectRefDef::ZoneChangeSuccessor(_)
                | ObjectRefDef::ZoneChangeResultOfTriggeringObject
                | ObjectRefDef::AbilityGrantSource
                | ObjectRefDef::ResolvingObject
                | ObjectRefDef::AdditionalCostObject(_)
                | ObjectRefDef::AttachedToSource
                | ObjectRefDef::Target(_)
                | ObjectRefDef::SourceOfTargetedStackObject(_)
                | ObjectRefDef::TriggeringObject
                | ObjectRefDef::DamagedObject,
            )
            | None => None,
        }
    }

    #[must_use]
    pub const fn Target(target: TargetIndex) -> Self {
        Self(EffectRecipientSetDef::LegalTargets(target))
    }

    #[must_use]
    pub const fn ControllerOfTarget(target: TargetIndex) -> Self {
        Self::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(target)))
    }

    #[must_use]
    pub const fn ObjectsSharingNameWithTarget(target: TargetIndex) -> Self {
        Self::objects(ObjectSetDef::SharingNameWith(ObjectRefDef::Target(target)))
    }

    #[must_use]
    pub const fn matching_objects(
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        controller_or_owner: PlayerRelation,
    ) -> Self {
        Self::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
            object,
            zones,
            controller_or_owner,
        )))
    }

    #[must_use]
    pub const fn objects_controlled_by_target(
        object: ObjectPredicateDef,
        slot: TargetIndex,
    ) -> Self {
        Self::objects(ObjectSetDef::Query(ObjectQueryDef::controlled_by(
            object,
            &[ZoneKind::Battlefield],
            PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(slot))),
        )))
    }

    #[must_use]
    pub const fn objects_owned_by_target(object: ObjectPredicateDef, slot: TargetIndex) -> Self {
        Self::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
            object,
            &[ZoneKind::Battlefield],
            PlayerSetDef::One(PlayerRefDef::Target(slot)),
        )))
    }

    #[must_use]
    pub const fn cards_owned_by_target(
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        slot: TargetIndex,
    ) -> Self {
        Self::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
            object,
            zones,
            PlayerSetDef::One(PlayerRefDef::Target(slot)),
        )))
    }
}

/// The lifetime of a continuous effect created by a resolving spell or
/// ability. Static effects use [`EffectDef::StaticApply`] instead: they are
/// derived live from the ability that creates them and have no stored
/// expiration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolvedEffectDurationDef {
    Permanent,
    UntilEndOfTurn,
    /// Until the beginning of the resolving ability's controller's next
    /// upkeep, which outlives the cleanup that ends an until-end-of-turn
    /// effect.
    UntilYourNextUpkeep,
    /// Until the next turn of the effect's controller begins. The affected
    /// turn is captured when the resolving effect is created.
    UntilYourNextTurn,
    /// Until the current combat phase ends. Shorter than
    /// [`Self::UntilEndOfTurn`]: it expires as the end-of-combat step
    /// finishes rather than waiting for cleanup, so a creature pumped for one
    /// combat is back to its printed size in the postcombat main phase.
    UntilEndOfCombat,
    /// For as long as the effect's own source stays tapped. Unlike every
    /// other resolving duration this one has no deadline: the artifact that
    /// tapped to make it decides when it ends by untapping.
    WhileSourceTapped,
    /// "For as long as this creature remains on the battlefield." The same
    /// open-ended shape as [`Self::WhileSourceTapped`] with a weaker
    /// condition: the source has only to still be there. A source that
    /// leaves and returns is a new object, so what it left behind stays
    /// ended.
    WhileSourceRemains,
}

/// How long a resolved control-changing effect lasts.
///
/// Source-dependent control changes are deliberately separate from ordinary
/// continuous-effect durations: they end when the source leaves its
/// controller (and sometimes when it untaps), so their runtime dependency is
/// not expressible as a turn-relative expiration alone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ControlDurationDef {
    UntilEndOfTurn,
    /// For as long as nothing changes it back. A control-change effect with
    /// no stated duration lasts indefinitely (CR 611.2b), which is what an
    /// exchange of control means: nothing is holding it and no cleanup ends
    /// it.
    Indefinitely,
    WhileSourceRemains {
        /// Whether the source also has to remain tapped.
        while_tapped: bool,
    },
}

/// Whether a damage-prevention rule matches combat damage, or damage of any
/// kind.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageKindDef {
    Any,
    Combat,
}

/// A named group of damage sources a turn-long prevention can answer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageSourceGroupDef {
    CreaturesWithFlying,
    AttackingCreaturesWithoutFlying,
    Artifacts,
    /// Attacking creatures nothing is blocking. The question is asked as the
    /// damage arrives, so a blocker removed mid-combat changes the answer.
    UnblockedCreatures,
}

impl DamageSourceGroupDef {
    /// Every group, in the order their per-player damage accumulators are
    /// stored. Appending is safe; reordering would misread a checkpoint.
    pub const ALL: [Self; 4] = [
        Self::CreaturesWithFlying,
        Self::AttackingCreaturesWithoutFlying,
        Self::Artifacts,
        Self::UnblockedCreatures,
    ];

    pub const COUNT: usize = Self::ALL.len();

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::CreaturesWithFlying => 0,
            Self::AttackingCreaturesWithoutFlying => 1,
            Self::Artifacts => 2,
            Self::UnblockedCreatures => 3,
        }
    }
}

/// The source side of a prospective damage event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageSourceMatcherDef {
    Any,
    /// A checkpoint-stable relational source group named by the card text.
    Group(DamageSourceGroupDef),
    /// The object receiving a static applied effect.
    AffectedObject,
    Object(ObjectRefDef),
    Except(ObjectRefDef),
    Matching(ObjectPredicateDef),
}

/// The recipient side of a prospective damage event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageRecipientMatcherDef {
    Any,
    /// The object receiving a static applied effect.
    AffectedObject,
    Recipients(EffectRecipientDef),
    /// An object matching a predicate. "Deals combat damage to a creature"
    /// is about what was hit rather than about which one in particular, so
    /// it is a predicate over the damaged object -- the mirror of
    /// [`DamageSourceMatcherDef::Matching`] on the other side of the event.
    MatchingObject(ObjectPredicateDef),
    /// A player resolved when the prevention is created, plus creatures that
    /// player controls when damage would be dealt.
    PlayerAndCreaturesControlledBy(PlayerRefDef),
    /// Any player, or any planeswalker. "Deals combat damage to a player or
    /// planeswalker" is one clause rather than two, and the two halves are
    /// not the same kind of thing, so neither a player set nor an object
    /// query can say it on its own.
    PlayerOrPlaneswalker,
}

/// How much damage a limiting rule lets through.
///
/// A limit is not a prevention: it has no capacity to spend and no follow-up,
/// it simply caps every matching event for as long as its source applies.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageLimitDef {
    /// "It deals N damage instead." Events already at or under the cap are
    /// untouched, which is why a printed "N or more" threshold needs no
    /// separate condition.
    CapAt(u16),
    /// "Damage that would reduce your life total to less than N reduces it to
    /// N instead." The cap depends on the recipient's life when the damage
    /// would be dealt, so it cannot be folded into [`Self::CapAt`].
    LeaveAtLeastLife(i16),
}

/// A conjunctive matcher over a prospective damage event.
///
/// Preventing damage both to and by one object is represented by two rules in
/// an [`EffectDef::Sequence`] or [`AppliedEffectDef::Composite`]. Keeping each
/// leaf conjunctive makes resolution and spending order explicit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DamageEventMatcherDef {
    pub kind: DamageKindDef,
    pub source: DamageSourceMatcherDef,
    pub recipient: DamageRecipientMatcherDef,
}

/// The number of creatures in the declaration containing one attack event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AttackDeclarationRangeDef {
    pub minimum: u8,
    /// An inclusive upper bound. `None` means no maximum.
    pub maximum: Option<u8>,
}

impl AttackDeclarationRangeDef {
    pub const ANY: Self = Self {
        minimum: 1,
        maximum: None,
    };

    #[must_use]
    pub const fn between(minimum: u8, maximum: Option<u8>) -> Self {
        Self { minimum, maximum }
    }
}

/// A conjunctive matcher over one creature being declared as an attacker.
///
/// `attack_number` is how many times that creature has attacked this turn,
/// including this declaration. It is frozen when attackers are declared, so
/// an extra combat phase can produce number two without re-reading mutable
/// battlefield state when the trigger is placed on the stack.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AttackEventMatcherDef {
    pub attacker: ObjectPredicateDef,
    pub declaration: AttackDeclarationRangeDef,
    pub attack_number: Option<u8>,
    /// Who the attack has to be aimed at. "Whenever a creature attacks you
    /// or a planeswalker you control" is one clause about one player: a
    /// creature attacking a planeswalker is attacking the player who
    /// controls it, so both halves are the same relation.
    pub defender: Option<PlayerRelation>,
    /// Which kind of defender the attack has to have been declared against.
    pub defender_kind: AttackDefenderKindDef,
}

/// Whether an attack clause counts a planeswalker as its player.
///
/// Attacking a planeswalker is attacking the player who controls it (CR
/// 506.3b), which is what "attacks you" means; a clause that says "attacks
/// the player" means the player and not their planeswalker, so the two are
/// different questions rather than one relation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AttackDefenderKindDef {
    /// "Attacks you", "attacks a player": either the player or a
    /// planeswalker they control.
    #[default]
    PlayerOrTheirPlaneswalker,
    /// "Attacks the player with the most life": that player themselves.
    PlayerOnly,
}

impl AttackEventMatcherDef {
    #[must_use]
    pub const fn any(attacker: ObjectPredicateDef) -> Self {
        Self {
            attacker,
            declaration: AttackDeclarationRangeDef::ANY,
            attack_number: None,
            defender: None,
            defender_kind: AttackDefenderKindDef::PlayerOrTheirPlaneswalker,
        }
    }

    /// "Attacks the player ...": a planeswalker is not the player, however
    /// much attacking it is attacking them.
    #[must_use]
    pub const fn against_a_player(mut self) -> Self {
        self.defender_kind = AttackDefenderKindDef::PlayerOnly;
        self
    }

    /// "Attacks <player>", for the clauses that care who is being attacked.
    #[must_use]
    pub const fn attacking(attacker: ObjectPredicateDef, defender: PlayerRelation) -> Self {
        let mut matcher = Self::any(attacker);
        matcher.defender = Some(defender);
        matcher
    }

    #[must_use]
    pub const fn first(attacker: ObjectPredicateDef) -> Self {
        Self {
            attacker,
            declaration: AttackDeclarationRangeDef::ANY,
            attack_number: Some(1),
            defender: None,
            defender_kind: AttackDefenderKindDef::PlayerOrTheirPlaneswalker,
        }
    }

    #[must_use]
    pub const fn in_declaration(
        attacker: ObjectPredicateDef,
        minimum: u8,
        maximum: Option<u8>,
    ) -> Self {
        Self {
            attacker,
            declaration: AttackDeclarationRangeDef::between(minimum, maximum),
            attack_number: None,
            defender: None,
            defender_kind: AttackDefenderKindDef::PlayerOrTheirPlaneswalker,
        }
    }
}

/// Why a permanent became tapped.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TapPurposeDef {
    Any,
    Mana,
}

/// Which way an attachment goes as a permanent enters the battlefield.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArrivalAttachmentDef {
    /// Attach the resolving source to what arrived. "Put target creature card
    /// onto the battlefield ... and attach this enchantment to it."
    SourceToArrival,
    /// Attach what arrived to a permanent already there. "Return this card
    /// from your graveyard to the battlefield, then attach it to that
    /// creature" moves the Equipment, so the host is what it is named
    /// against.
    ArrivalToHost(ObjectRefDef),
    /// Attach what arrived to a player. An Aura put directly onto the
    /// battlefield has to choose a player it could legally enchant, and the
    /// attachment must be established as part of that arrival.
    ArrivalToPlayer(PlayerRefDef),
}

/// A matcher over one card reaching a player's hand as a draw.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DrawEventMatcherDef {
    pub player: PlayerRelation,
    /// The card that was drawn, captured with its hand characteristics before
    /// anything can move it. Most draw triggers accept any card; Booby Trap
    /// is the recurring rules shape that names one.
    pub card: ObjectPredicateDef,
    /// Whether the card a player is handed at the start of their own draw
    /// step is spared. Orcish Bowmasters prints "except the first one they
    /// draw in each of their draw steps", which exempts the turn-based draw
    /// and nothing else: a second card drawn in that same step is caught,
    /// and so is every draw taken outside it.
    pub except_first_in_draw_step: bool,
    /// Which card of that player's turn this has to be. "Whenever an
    /// opponent draws their second card each turn" is about the ordinal
    /// rather than the draw: the first one and the third one are not it.
    pub nth_this_turn: Option<u16>,
}

impl DrawEventMatcherDef {
    #[must_use]
    pub const fn any(player: PlayerRelation) -> Self {
        Self {
            player,
            card: ObjectPredicateDef::Any,
            except_first_in_draw_step: false,
            nth_this_turn: None,
        }
    }

    /// "Their `nth` card each turn", counted over the whole turn rather than
    /// over any one step.
    #[must_use]
    pub const fn nth_each_turn(player: PlayerRelation, nth: u16) -> Self {
        Self {
            player,
            card: ObjectPredicateDef::Any,
            except_first_in_draw_step: false,
            nth_this_turn: Some(nth),
        }
    }

    #[must_use]
    pub const fn except_first_in_draw_step(player: PlayerRelation) -> Self {
        Self {
            player,
            card: ObjectPredicateDef::Any,
            except_first_in_draw_step: true,
            nth_this_turn: None,
        }
    }

    #[must_use]
    pub const fn matching(player: PlayerRelation, card: ObjectPredicateDef) -> Self {
        Self {
            player,
            card,
            except_first_in_draw_step: false,
            nth_this_turn: None,
        }
    }
}

/// A matcher over one untapped-to-tapped transition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TapEventMatcherDef {
    pub object: ObjectPredicateDef,
    pub purpose: TapPurposeDef,
}

impl TapEventMatcherDef {
    #[must_use]
    pub const fn any(object: ObjectPredicateDef) -> Self {
        Self {
            object,
            purpose: TapPurposeDef::Any,
        }
    }

    #[must_use]
    pub const fn mana(object: ObjectPredicateDef) -> Self {
        Self {
            object,
            purpose: TapPurposeDef::Mana,
        }
    }
}

/// A matcher over one committed zone transition.
///
/// `previously_damaged_by` consults the damage-source history frozen as the
/// object leaves the battlefield. It therefore remains valid for simultaneous
/// deaths and never re-reads a fresh object in the destination zone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZoneChangeEventMatcherDef {
    pub object: ObjectPredicateDef,
    pub from: Option<ZoneKind>,
    pub to: Option<ZoneKind>,
    pub previously_damaged_by: Option<ObjectRefDef>,
}

impl ZoneChangeEventMatcherDef {
    #[must_use]
    pub const fn new(
        object: ObjectPredicateDef,
        from: Option<ZoneKind>,
        to: Option<ZoneKind>,
    ) -> Self {
        Self {
            object,
            from,
            to,
            previously_damaged_by: None,
        }
    }

    #[must_use]
    pub const fn previously_damaged_by(mut self, source: ObjectRefDef) -> Self {
        self.previously_damaged_by = Some(source);
        self
    }
}

impl DamageEventMatcherDef {
    pub const ANY: Self = Self {
        kind: DamageKindDef::Any,
        source: DamageSourceMatcherDef::Any,
        recipient: DamageRecipientMatcherDef::Any,
    };

    pub const COMBAT: Self = Self {
        kind: DamageKindDef::Combat,
        source: DamageSourceMatcherDef::Any,
        recipient: DamageRecipientMatcherDef::Any,
    };

    #[must_use]
    pub const fn to(recipients: EffectRecipientDef) -> Self {
        Self {
            recipient: DamageRecipientMatcherDef::Recipients(recipients),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn from(source: ObjectRefDef) -> Self {
        Self {
            source: DamageSourceMatcherDef::Object(source),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn from_group_to(
        source: DamageSourceGroupDef,
        recipients: EffectRecipientDef,
    ) -> Self {
        Self {
            source: DamageSourceMatcherDef::Group(source),
            recipient: DamageRecipientMatcherDef::Recipients(recipients),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn combat_to(recipients: EffectRecipientDef) -> Self {
        Self {
            recipient: DamageRecipientMatcherDef::Recipients(recipients),
            ..Self::COMBAT
        }
    }

    #[must_use]
    pub const fn combat_from(source: ObjectRefDef) -> Self {
        Self {
            source: DamageSourceMatcherDef::Object(source),
            ..Self::COMBAT
        }
    }

    #[must_use]
    pub const fn combat_except(source: ObjectRefDef) -> Self {
        Self {
            source: DamageSourceMatcherDef::Except(source),
            ..Self::COMBAT
        }
    }

    #[must_use]
    pub const fn to_player_and_creatures_controlled_by(player: PlayerRefDef) -> Self {
        Self {
            recipient: DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn from_matching_to_affected(source: ObjectPredicateDef) -> Self {
        Self {
            kind: DamageKindDef::Any,
            source: DamageSourceMatcherDef::Matching(source),
            recipient: DamageRecipientMatcherDef::AffectedObject,
        }
    }

    pub const COMBAT_FROM_AFFECTED: Self = Self {
        kind: DamageKindDef::Combat,
        source: DamageSourceMatcherDef::AffectedObject,
        recipient: DamageRecipientMatcherDef::Any,
    };

    pub const COMBAT_TO_AFFECTED: Self = Self {
        kind: DamageKindDef::Combat,
        source: DamageSourceMatcherDef::Any,
        recipient: DamageRecipientMatcherDef::AffectedObject,
    };
}

/// How long or how often a resolving prevention rule can be spent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamagePreventionCapacityDef {
    Amount(ValueDef),
    Events(u8),
    Unlimited,
}

/// How much of each matched damage event is prevented.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageCoverageDef {
    All,
    HalfRoundedDown,
}

/// A synchronous consequence of damage prevented by one rule.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamagePreventionFollowUpDef {
    GainLife(PlayerRefDef),
}

/// One damage-prevention rule installed by a resolving effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DamagePreventionDef {
    pub matcher: DamageEventMatcherDef,
    pub capacity: DamagePreventionCapacityDef,
    pub coverage: DamageCoverageDef,
    pub follow_up: Option<DamagePreventionFollowUpDef>,
}

impl DamagePreventionDef {
    #[must_use]
    pub const fn amount(matcher: DamageEventMatcherDef, amount: ValueDef) -> Self {
        Self::new(matcher, DamagePreventionCapacityDef::Amount(amount))
    }

    #[must_use]
    pub const fn events(matcher: DamageEventMatcherDef, events: u8) -> Self {
        Self::new(matcher, DamagePreventionCapacityDef::Events(events))
    }

    #[must_use]
    pub const fn unlimited(matcher: DamageEventMatcherDef) -> Self {
        Self::new(matcher, DamagePreventionCapacityDef::Unlimited)
    }

    #[must_use]
    pub const fn new(
        matcher: DamageEventMatcherDef,
        capacity: DamagePreventionCapacityDef,
    ) -> Self {
        Self {
            matcher,
            capacity,
            coverage: DamageCoverageDef::All,
            follow_up: None,
        }
    }

    #[must_use]
    pub const fn with_coverage(mut self, coverage: DamageCoverageDef) -> Self {
        self.coverage = coverage;
        self
    }

    #[must_use]
    pub const fn with_follow_up(mut self, follow_up: DamagePreventionFollowUpDef) -> Self {
        self.follow_up = Some(follow_up);
        self
    }
}
