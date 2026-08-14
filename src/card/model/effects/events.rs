use super::{EffectRecipientDef, ObjectPredicateDef, PlayerRelation, ZoneKind};

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
    /// CR 509.1h: the attacker became blocked. The event carries how many
    /// creatures are blocking it beyond the first, which is the quantity
    /// every rampage-style clause is written against.
    BecomesBlocked(ObjectPredicateDef),
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
    /// A matching object dealt damage to anything at all. The amount is
    /// available as [`super::ValueDef::TriggerEventAmount`]. This is the other
    /// direction from [`Self::DamageDealt`], which only watches damage
    /// arriving at the ability's own source.
    DamageDealtBy {
        source: ObjectPredicateDef,
    },
    /// A matching creature was declared as an attacker in a declaration of a
    /// given size. The attacker is the triggering object, so an ability
    /// watching this can reach it with
    /// [`EffectRecipientDef::TriggeringObject`], and the ability need not be
    /// on a creature.
    ///
    /// The size is known only at declaration, which is why this is its own
    /// event rather than a condition rechecked later. Exalted asks for
    /// exactly one; battalion asks for three or more.
    AttacksInGroup {
        attacker: ObjectPredicateDef,
        minimum_total: u8,
        /// An upper bound, for "attacks alone". `None` means no maximum.
        maximum_total: Option<u8>,
    },
    /// A creature matching `source` dealt combat damage to a player. The
    /// damaged player is the event player and the amount is available as
    /// [`super::ValueDef::TriggerEventAmount`]. Only damage dealt in a combat
    /// damage step counts, which is what separates this from
    /// [`Self::DamageDealt`].
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
    /// [`super::ValueDef::TriggerEventAmount`].
    LifeGained(PlayerRelation),
    /// A creature dealt damage by this ability's source this turn died.
    DamagedCreatureDied,
    Special(&'static str),
}
