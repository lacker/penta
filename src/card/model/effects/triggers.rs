//! The committed events a triggered ability can watch.
//!
//! Split out of the parent module for the source-size budget.

use super::{
    AttackDeclarationRangeDef, AttackEventMatcherDef, ColorSet, DamageEventMatcherDef,
    DamageKindDef, DamageRecipientMatcherDef, DamageSourceMatcherDef, DrawEventMatcherDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectRefDef, PlayerRelation, PlayerSetDef,
    TapEventMatcherDef, TriggerConditionDef, TurnStepDef, ZoneChangeEventMatcherDef, ZoneKind,
};

/// What a stack object must have newly targeted for the event to match.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StackTargetFilterDef {
    Player(PlayerRelation),
    Permanent(ObjectPredicateDef),
    Card(ObjectPredicateDef),
    Spell(ObjectPredicateDef),
    /// Any one of several recipient shapes in a single printed clause.
    AnyOf(&'static [StackTargetFilterDef]),
}

/// How matches inside one atomic target-selection event become triggers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StackTargetAggregationDef {
    /// Trigger for each distinct matching recipient.
    EachMatchingTarget,
    /// Trigger once if one or more recipients match.
    OneOrMoreMatchingTargets,
}

/// The occurrence involving a matching object on the stack.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StackObjectEventDef {
    /// A spell was cast, optionally from a particular zone.
    Cast { from: Option<ZoneKind> },
    /// A copy of a spell was created on the stack. A copy is not cast.
    Copied,
    /// Targets were locked in while a spell or ability was put on the stack,
    /// or changed later by an effect. Retargeting is deliberately the same
    /// occurrence even though it is not another cast or activation.
    TargetSelection {
        target: StackTargetFilterDef,
        aggregation: StackTargetAggregationDef,
    },
}

/// A declarative match over an object on the stack and what just happened to
/// it. The object predicate always reads the spell or ability; target clauses
/// additionally constrain the recipients newly selected by that occurrence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StackObjectEventMatcherDef {
    pub object: ObjectPredicateDef,
    pub event: StackObjectEventDef,
}

/// The committed event observed by a triggered ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TriggerEventDef {
    /// This source's cumulative-upkeep cost was paid. The trigger amount is
    /// how many mana spent on that payment had one of the named colors.
    CumulativeUpkeepPaid {
        mana_colors: ColorSet,
    },
    /// The controller declined or could not make this source's cumulative
    /// upkeep payment. The captured amount is its age-counter count.
    CumulativeUpkeepNotPaid,
    CoinFlipWon(PlayerRelation),
    CoinFlipLost(PlayerRelation),
    /// Any one of several events, for a printed ability that names more than
    /// one -- "whenever this creature enters or attacks". Splitting such a
    /// card into two abilities would misreport what it prints and would count
    /// as two triggered abilities where the card has one.
    AnyOf(&'static [TriggerEventDef]),
    /// A printed "while ...": a condition that belongs to the event rather
    /// than being an intervening if. Checked once, where the event is
    /// matched, and never rechecked as the ability resolves -- which is the
    /// whole difference between "attacks while you have the most life" and
    /// "attacks. If you have the most life, ..." (CR 603.2 against 603.4).
    While {
        event: &'static TriggerEventDef,
        condition: &'static TriggerConditionDef,
    },
    ZoneChanged(ZoneChangeEventMatcherDef),
    /// A permanent changed from untapped to tapped. The matcher can narrow
    /// this to a tap that paid for that permanent's mana ability.
    Tapped(TapEventMatcherDef),
    /// A creature was declared as an attacker. The matcher can constrain the
    /// declaration size and this creature's attack number for the turn.
    Attacks(AttackEventMatcherDef),
    /// A matching creature was declared as an attacker and no creature
    /// blocked it. This fires once blockers are declared, which is the only
    /// moment "isn't blocked" is knowable.
    AttacksAndIsNotBlocked {
        attacker: ObjectPredicateDef,
    },
    /// "Whenever one or more creatures die." One trigger for the whole
    /// simultaneous move rather than one apiece, which is what "one or more"
    /// means: a board wipe gives one. Dying is the battlefield-to-graveyard
    /// move (CR 700.4), so a permanent exiled instead is not in it, and the
    /// predicate says which of the batch count toward the clause.
    ObjectsDied {
        object: ObjectPredicateDef,
    },
    /// "Whenever one or more creatures an opponent controls attack you and
    /// aren't blocked." One trigger for the whole declaration, however many
    /// of them went unblocked, which is what "one or more" means -- distinct
    /// from [`Self::AttacksAndIsNotBlocked`] for the same reason
    /// [`Self::AttackDeclared`] is distinct from [`Self::Attacks`]. Only
    /// attackers pointed at the player count: a creature attacking a
    /// planeswalker is not attacking them.
    UnblockedAttackersDeclared {
        attacker: ObjectPredicateDef,
        defender: PlayerRelation,
    },
    /// "Whenever one or more creatures you control deal combat damage to one
    /// or more players." One trigger for the whole combat damage step,
    /// however many creatures dealt it and however many players took it,
    /// which is what "one or more" means. First strike gives a step of its
    /// own, and so a batch of its own.
    CombatDamageDealtToPlayers {
        sources: ObjectPredicateDef,
        players: PlayerRelation,
    },
    /// CR 509.1h: the attacker became blocked. The event carries how many
    /// creatures are blocking it beyond the first, which is the quantity
    /// every rampage-style clause is written against.
    BecomesBlocked(ObjectPredicateDef),
    /// A matching creature blocked a matching creature, or was blocked by
    /// one. The two directions are one printed clause; `creature` is the
    /// subject named before "blocks" and `other` is the triggering object.
    /// Keeping both predicates explicit lets an Equipment listen for its
    /// attached creature without pretending the Equipment itself blocked.
    BlocksOrBecomesBlockedBy {
        creature: ObjectPredicateDef,
        other: ObjectPredicateDef,
    },
    /// This creature blocked a matching creature. The blocking half of
    /// [`Self::BlocksOrBecomesBlockedBy`], for the cards that print only one
    /// direction; the creature it blocked is the triggering object.
    Blocks {
        blocked: ObjectPredicateDef,
    },
    /// A matching creature blocked this one. The attacking half, and the
    /// mirror of [`Self::Blocks`]; the blocker is the triggering object.
    ///
    /// Distinct from [`Self::BecomesBlocked`], which fires once however many
    /// creatures blocked and names no blocker at all.
    BecomesBlockedBy {
        blocker: ObjectPredicateDef,
    },
    /// Something happened to a matching object on the stack: it was cast,
    /// copied, or selected new targets. One representation keeps kind,
    /// recipient, and aggregation filters orthogonal while retaining the
    /// rules distinction between casting, copying, and retargeting.
    StackObject(StackObjectEventMatcherDef),
    StepBegins {
        step: TurnStepDef,
        player: PlayerRelation,
    },
    /// "When you play another land." Playing a land is a special action
    /// rather than a spell or an ability (CR 305.1), and it is not the same
    /// event as the land entering: one an effect puts onto the battlefield
    /// enters without ever having been played, and this does not fire for
    /// it.
    LandPlayed {
        land: ObjectPredicateDef,
        player: PlayerRelation,
    },
    /// "Whenever one or more cards are put into exile from your library
    /// and/or your graveyard." One trigger for the whole move, however many
    /// cards it took, and the pair of zones is one clause rather than two.
    ///
    /// The event is the exiling rather than any card in it, which is what
    /// "one or more" means: a clause that fired once per card would give a
    /// three-card exile three counters.
    CardsExiled {
        zones: &'static [ZoneKind],
        owner: PlayerRelation,
    },
    /// "Whenever you create one or more creature tokens." One trigger for
    /// the whole instruction however many tokens it made, which is what
    /// "one or more" means: a clause that fired once per token would give a
    /// three-token instruction three counters. The predicate says which of
    /// them count toward the batch rather than which one the trigger is
    /// about.
    TokensCreated {
        player: PlayerRelation,
        token: ObjectPredicateDef,
    },
    /// "Whenever you attack." One declaration, one trigger, however many
    /// creatures were declared (CR 508.1). Distinct from [`Self::Attacks`]
    /// because the event is the declaration rather than any creature in it:
    /// the predicate says which attackers count toward the size, not which
    /// of them the trigger is about.
    AttackDeclared {
        attacker: ObjectPredicateDef,
        declaration: AttackDeclarationRangeDef,
    },
    /// One actual, unprevented damage event. The source, recipient, combat
    /// status, damaged player, and amount all come from the same committed
    /// event rather than a family of overlapping publications.
    DamageDealt(DamageEventMatcherDef),
    /// A state trigger (CR 603.8). It has no event at all: it triggers
    /// whenever its ability's condition is true, and does not trigger again
    /// while it is already waiting or on the stack.
    StateCondition,
    /// This permanent turned over to the face carrying this ability, which is
    /// what "whenever this transforms into ..." names.
    Transforms(ObjectPredicateDef),
    /// A player gained life. The amount is available as
    /// `ValueDef::TriggerEventAmount`.
    LifeGained(PlayerRelation),
    /// A matching player drew a card. One trigger per card, so a spell that
    /// draws three fires this three times.
    DrewCard(DrawEventMatcherDef),
    /// A matching player became the monarch (CR 720). The crown passing
    /// from one player to another raises this once, for whoever received it.
    BecomesMonarch(PlayerRelation),
    /// A card was put into a graveyard from a matching player's hand. One
    /// trigger per card, so "whenever you discard a card" fires twice for a
    /// discard of two -- and a discard paid as a cost is still a discard.
    Discarded(PlayerRelation),
    /// "Whenever you discard one or more cards." One trigger for the whole
    /// discard however many cards it took, which is what separates it from
    /// [`Self::Discarded`]: a discard of two fires that one twice and this
    /// one once.
    DiscardedCards(PlayerRelation),
    /// "When you do", for the reflexive half of exert (CR 701.38a).
    ///
    /// Exerting is a choice made as the creature is declared as an attacker,
    /// not an event that happens to it afterwards, so this is what the
    /// second sentence of "You may exert this creature as it attacks. When
    /// you do, ..." watches. An ability that watches it is also what makes
    /// the creature exertable at all: nothing else on the card says so.
    Exerted(ObjectPredicateDef),
    /// "When you do", for the reflexive half of a "you may ..." clause
    /// (CR 603.1c).
    ///
    /// The optional clause is part of another ability's resolution rather
    /// than an event on the board, so what this watches is the moment its
    /// controller accepts the offer. The reflexive ability then goes on the
    /// stack on its own, chooses its own targets, and can be responded to --
    /// which is why "at the beginning of your upkeep, you may create a
    /// Treasure token" names nobody until the Treasure actually exists.
    OptionalEffectTaken(ObjectPredicateDef),
    /// "When you do", for the reflexive half of a clause that sacrifices a
    /// permanent during its own resolution (CR 603.1c).
    ///
    /// The sibling of [`Self::OptionalEffectTaken`], for the compulsory
    /// spelling: "Sacrifice a creature. When you do, ..." gives its
    /// controller no choice about the sacrifice, so nothing accepts an
    /// offer and that event never fires. What this watches instead is the
    /// sacrifice itself having happened, which is also why it says nothing
    /// when there was no creature to give up.
    ///
    /// The predicate matches the source of the clause rather than what it
    /// took, the same way a damage trigger matches the dealer. What was
    /// sacrificed is carried beside it: the reflexive ability reads its
    /// power as [`ValueDef::TriggerEventAmount`] and asks about the rest of
    /// it with [`TriggerConditionDef::SacrificedObjectMatches`], both from
    /// last-known information, because the permanent is already gone by the
    /// time the ability goes on the stack.
    SacrificePerformed(ObjectPredicateDef),
    /// "Whenever you sacrifice a Clue." A sacrifice is a way of putting a
    /// permanent into a graveyard rather than a thing that happens to it
    /// there, so it is its own event: a Clue somebody destroyed went to the
    /// same place and is not what this asks about. The relation is to the
    /// player who sacrificed it, which is who "you sacrifice" names.
    Sacrificed {
        object: ObjectPredicateDef,
        player: PlayerRelation,
    },
    /// "When you unlock this door" (CR 714.4c). A door becomes unlocked
    /// either on the battlefield, for the unlock special action, or as the
    /// Room enters because you cast that half.
    ///
    /// Raised at the door rather than at the permanent: a Room with both
    /// doors open has both doors' abilities, and only the one that just
    /// opened is the one this is about. Nothing publishes it as an ordinary
    /// committed event for that reason -- the unlock hands it to the door it
    /// belongs to, which is also why Panharmonicon does not double it.
    DoorUnlocked,
    /// "When this Class becomes level N." Only the Class carrying the clause
    /// can raise it, so the event names the level and nothing else.
    BecomesLevel(u8),
    /// One or more counters of this kind were put on a matching object.
    /// One event per placement rather than one per counter: "whenever one
    /// or more +1/+1 counters are put on this creature" fires once for a
    /// pair of them, which is what the wording is for.
    CountersPlaced {
        object: ObjectPredicateDef,
        kind: crate::card::CounterKind,
    },
    /// One or more counters of this kind were removed from a matching object.
    CountersRemoved {
        object: ObjectPredicateDef,
        kind: crate::card::CounterKind,
    },
    /// The removal left the matching object with none of that counter kind.
    LastCounterRemoved {
        object: ObjectPredicateDef,
        kind: crate::card::CounterKind,
    },
    /// "When you cycle this card" (CR 702.29b). Cycling is an activation, so
    /// this fires when the ability is activated rather than when it resolves,
    /// and the card is already in the graveyard by then. Only the cycled card
    /// carries the clause, so the event names nothing else.
    Cycled,
    /// "Whenever you commit a crime" (CR 701.51a). A player commits a crime
    /// as they cast a spell, activate an ability, or put a triggered ability
    /// onto the stack that targets an opponent, anything an opponent
    /// controls, or a card in an opponent's graveyard. It names only the
    /// player who did it -- what was targeted is not part of the event.
    CommittedCrime(PlayerRelation),
}

impl TriggerEventDef {
    const fn damage_source(source: ObjectPredicateDef) -> DamageSourceMatcherDef {
        match source {
            ObjectPredicateDef::Source => DamageSourceMatcherDef::Object(ObjectRefDef::Source),
            ObjectPredicateDef::AttachedToSource => {
                DamageSourceMatcherDef::Object(ObjectRefDef::AttachedToSource)
            }
            predicate => DamageSourceMatcherDef::Matching(predicate),
        }
    }

    #[must_use]
    pub const fn zone_changed(
        object: ObjectPredicateDef,
        from: Option<ZoneKind>,
        to: Option<ZoneKind>,
    ) -> Self {
        Self::ZoneChanged(ZoneChangeEventMatcherDef::new(object, from, to))
    }

    #[must_use]
    pub const fn spell_cast(object: ObjectPredicateDef) -> Self {
        Self::StackObject(StackObjectEventMatcherDef {
            object,
            event: StackObjectEventDef::Cast { from: None },
        })
    }

    #[must_use]
    pub const fn spell_cast_from(object: ObjectPredicateDef, from: ZoneKind) -> Self {
        Self::StackObject(StackObjectEventMatcherDef {
            object,
            event: StackObjectEventDef::Cast { from: Some(from) },
        })
    }

    #[must_use]
    pub const fn spell_copied(object: ObjectPredicateDef) -> Self {
        Self::StackObject(StackObjectEventMatcherDef {
            object,
            event: StackObjectEventDef::Copied,
        })
    }

    #[must_use]
    pub const fn targets_selected(
        stack_object: ObjectPredicateDef,
        target: StackTargetFilterDef,
        aggregation: StackTargetAggregationDef,
    ) -> Self {
        Self::StackObject(StackObjectEventMatcherDef {
            object: stack_object,
            event: StackObjectEventDef::TargetSelection {
                target,
                aggregation,
            },
        })
    }

    /// This permanent became the target of a matching stack object.
    #[must_use]
    pub const fn becomes_targeted(by: ObjectPredicateDef) -> Self {
        Self::targets_selected(
            by,
            StackTargetFilterDef::Permanent(ObjectPredicateDef::Source),
            StackTargetAggregationDef::EachMatchingTarget,
        )
    }

    #[must_use]
    pub const fn tapped(object: ObjectPredicateDef) -> Self {
        Self::Tapped(TapEventMatcherDef::any(object))
    }

    #[must_use]
    pub const fn tapped_for_mana(object: ObjectPredicateDef) -> Self {
        Self::Tapped(TapEventMatcherDef::mana(object))
    }

    #[must_use]
    pub const fn attacks(attacker: ObjectPredicateDef) -> Self {
        Self::Attacks(AttackEventMatcherDef::any(attacker))
    }

    /// "Whenever this creature attacks the player ...", which a planeswalker
    /// standing in front of that player does not satisfy.
    #[must_use]
    pub const fn attacks_a_player(attacker: ObjectPredicateDef) -> Self {
        Self::Attacks(AttackEventMatcherDef::any(attacker).against_a_player())
    }

    #[must_use]
    pub const fn attacks_first_time_this_turn(attacker: ObjectPredicateDef) -> Self {
        Self::Attacks(AttackEventMatcherDef::first(attacker))
    }

    /// "Whenever you attack", counted once for the whole declaration.
    #[must_use]
    pub const fn attack_declared(
        attacker: ObjectPredicateDef,
        minimum: u8,
        maximum: Option<u8>,
    ) -> Self {
        Self::AttackDeclared {
            attacker,
            declaration: AttackDeclarationRangeDef::between(minimum, maximum),
        }
    }

    #[must_use]
    pub const fn attacks_in_declaration(
        attacker: ObjectPredicateDef,
        minimum: u8,
        maximum: Option<u8>,
    ) -> Self {
        Self::Attacks(AttackEventMatcherDef::in_declaration(
            attacker, minimum, maximum,
        ))
    }

    #[must_use]
    pub const fn damage_to_source() -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Source),
            ..DamageEventMatcherDef::ANY
        })
    }

    #[must_use]
    pub const fn damage_dealt_by(source: ObjectPredicateDef) -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            source: Self::damage_source(source),
            ..DamageEventMatcherDef::ANY
        })
    }

    /// "Whenever <object> deals combat damage" -- to anything. A creature
    /// that was blocked deals its damage to the blockers, and the clause
    /// counts that no differently from a hit to the player.
    #[must_use]
    pub const fn combat_damage_dealt_by(source: ObjectPredicateDef) -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Combat,
            source: Self::damage_source(source),
            recipient: DamageRecipientMatcherDef::Any,
        })
    }

    #[must_use]
    pub const fn combat_damage_to_player(source: ObjectPredicateDef) -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Combat,
            source: Self::damage_source(source),
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::players(
                PlayerSetDef::All,
            )),
        })
    }

    /// Combat damage from a matching source to one player relation.
    #[must_use]
    pub const fn combat_damage_to_related_player(
        source: ObjectPredicateDef,
        player: PlayerRelation,
    ) -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Combat,
            source: Self::damage_source(source),
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::players(
                PlayerSetDef::Related(player),
            )),
        })
    }

    /// "Whenever this creature deals combat damage to a player or
    /// planeswalker." One event either way, so one trigger rather than two.
    #[must_use]
    pub const fn combat_damage_to_player_or_planeswalker(source: ObjectPredicateDef) -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Combat,
            source: Self::damage_source(source),
            recipient: DamageRecipientMatcherDef::PlayerOrPlaneswalker,
        })
    }

    #[must_use]
    pub const fn combat_damage_to_source(source: ObjectPredicateDef) -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Combat,
            source: Self::damage_source(source),
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Source),
        })
    }

    #[must_use]
    pub const fn damage_to_player(source: ObjectPredicateDef, player: PlayerRelation) -> Self {
        Self::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Any,
            source: Self::damage_source(source),
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::players(
                PlayerSetDef::Related(player),
            )),
        })
    }

    #[must_use]
    pub const fn transforms(object: ObjectPredicateDef) -> Self {
        Self::Transforms(object)
    }
}
