// Attacking and blocking restrictions used by continuously applied rules.
// Included textually into `effects.rs`, so imports come from the parent module.

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

/// One restriction or cost on declaring blockers.
///
/// Pair restrictions decide whether one prospective blocker-attacker pairing
/// is legal or adds a cost. Minimum-blocker restrictions instead constrain
/// the finished declaration: declining to block is legal, but assigning fewer
/// than the named number is not.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BlockRestrictionDef {
    /// `None` prohibits the matching block. `Some` is paid by the blocking
    /// creature's controller. Restrictions on a blocker are charged once for
    /// that blocker even if it can block several attackers; restrictions on
    /// an attacker are charged once for each matching blocker assigned to it.
    Pair {
        subject: BlockRestrictionSubjectDef,
        counterpart: BlockRestrictionMatchDef,
        cost: Option<ManaCost>,
    },
    /// "This creature can't be blocked except by N or more creatures."
    /// Menace is the same finished-declaration restriction with a two on it.
    MinimumBlockers(u8),
}

impl BlockRestrictionDef {
    #[must_use]
    pub const fn prohibit(
        subject: BlockRestrictionSubjectDef,
        counterpart: BlockRestrictionMatchDef,
    ) -> Self {
        Self::Pair {
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
        Self::Pair {
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
