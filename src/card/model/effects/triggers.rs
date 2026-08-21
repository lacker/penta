//! The committed events a triggered ability can watch.
//!
//! Split out of the parent module for the source-size budget.

use super::{
    AttackDeclarationRangeDef, AttackEventMatcherDef, DamageEventMatcherDef, DamageKindDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, DrawEventMatcherDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectRefDef, PlayerRelation, PlayerSetDef, TapEventMatcherDef,
    TurnStepDef, ZoneChangeEventMatcherDef, ZoneKind,
};

/// The committed event observed by a triggered ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TriggerEventDef {
    /// Any one of several events, for a printed ability that names more than
    /// one -- "whenever this creature enters or attacks". Splitting such a
    /// card into two abilities would misreport what it prints and would count
    /// as two triggered abilities where the card has one.
    AnyOf(&'static [TriggerEventDef]),
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
    SpellCast(ObjectPredicateDef),
    /// This object became the target of a spell the predicate matches.
    /// Raised where the targets are locked in -- as the spell is cast, which
    /// is what "becomes the target" means -- and once per targeting spell
    /// however many of its slots name the same object (CR 115.7c). Activated
    /// abilities target too, and this is not about them.
    BecomesTargetOfSpell(ObjectPredicateDef),
    /// This object became the target of a spell or of an ability, which is
    /// the pair ward asks about. Raised where each locks its targets in --
    /// as the spell is cast, or as the ability goes onto the stack -- and
    /// once per targeting object however many of its slots name this one.
    BecomesTargetOfSpellOrAbility(ObjectPredicateDef),
    StepBegins {
        step: TurnStepDef,
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
