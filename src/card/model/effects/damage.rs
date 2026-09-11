/// One part of a declarative simultaneous-damage event. `None` uses the
/// resolving spell or ability's ordinary source; an explicit object reference
/// is for instructions such as fight where another object deals the damage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DamageAssignmentDef {
    pub source: Option<ObjectRefDef>,
    pub recipient: EffectRecipientDef,
    pub amount: ValueDef,
}

impl DamageAssignmentDef {
    #[must_use]
    pub const fn from(
        source: ObjectRefDef,
        recipient: EffectRecipientDef,
        amount: ValueDef,
    ) -> Self {
        Self {
            source: Some(source),
            recipient,
            amount,
        }
    }

    #[must_use]
    pub const fn from_effect(recipient: EffectRecipientDef, amount: ValueDef) -> Self {
        Self {
            source: None,
            recipient,
            amount,
        }
    }
}

/// A continuation that runs only when a fight assignment dealt excess damage
/// to the named participant. Fight itself is all-or-nothing: if either named
/// object is not a creature, neither deals damage. This continuation also
/// stays dormant when the fight does not happen, all damage is prevented, or
/// the named participant took no excess. The excess amount is exposed to
/// `then` through [`ValueDef::MatchedCount`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FightExcessDef {
    pub recipient: ObjectRefDef,
    pub then: &'static EffectDef,
}

/// Storage for one assignment or an authored list. Both forms resolve as one
/// simultaneous event through the same pipeline; the inline form lets const
/// constructors build ordinary damage without allocating a static slice.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageAssignmentsDef {
    One(DamageAssignmentDef),
    Many(&'static [DamageAssignmentDef]),
}

/// One damage instruction. Source, assignment count, and outcome handling are
/// independent: an explicit source or simultaneous batch can use either rider.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DamageDef {
    pub assignments: DamageAssignmentsDef,
    pub follow_up: Option<DamageFollowUpDef>,
}

impl DamageDef {
    #[must_use]
    pub const fn new(recipient: EffectRecipientDef, amount: ValueDef) -> Self {
        Self {
            assignments: DamageAssignmentsDef::One(DamageAssignmentDef::from_effect(
                recipient, amount,
            )),
            follow_up: None,
        }
    }

    #[must_use]
    pub const fn from_source(
        source: ObjectRefDef,
        recipient: EffectRecipientDef,
        amount: ValueDef,
    ) -> Self {
        Self {
            assignments: DamageAssignmentsDef::One(DamageAssignmentDef::from(
                source, recipient, amount,
            )),
            follow_up: None,
        }
    }

    #[must_use]
    pub const fn simultaneous(assignments: &'static [DamageAssignmentDef]) -> Self {
        Self {
            assignments: DamageAssignmentsDef::Many(assignments),
            follow_up: None,
        }
    }

    #[must_use]
    pub const fn with_follow_up(mut self, follow_up: DamageFollowUpDef) -> Self {
        self.follow_up = Some(follow_up);
        self
    }

    #[must_use]
    pub fn assignments(&self) -> &[DamageAssignmentDef] {
        match &self.assignments {
            DamageAssignmentsDef::One(assignment) => std::slice::from_ref(assignment),
            DamageAssignmentsDef::Many(assignments) => assignments,
        }
    }

    pub(crate) const fn continuation(self) -> Option<&'static EffectDef> {
        match self.follow_up {
            Some(DamageFollowUpDef::IfDealtToIntended(then)) => Some(then),
            Some(DamageFollowUpDef::ApplyToDamaged { .. }) | None => None,
        }
    }

    pub(crate) const fn applied_effect(self) -> Option<AppliedEffectDef> {
        match self.follow_up {
            Some(DamageFollowUpDef::ApplyToDamaged { effect, .. }) => Some(*effect),
            Some(DamageFollowUpDef::IfDealtToIntended(_)) | None => None,
        }
    }
}

/// An outcome-dependent rider. The original recipients and the actual damage
/// recipients can differ after prevention or redirection, so these consumers
/// deliberately ask different questions of the completed damage event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DamageFollowUpDef {
    /// Run once if any intended recipient actually took damage. Full
    /// prevention or redirection entirely elsewhere skips this continuation.
    /// Life lost is not a substitute for damage dealt (for example, lifelink).
    IfDealtToIntended(&'static EffectDef),
    /// Apply to every actual damage recipient, including redirected damage.
    /// The applied effect determines which kinds of recipients it affects.
    ApplyToDamaged {
        effect: &'static AppliedEffectDef,
        duration: ResolvedEffectDurationDef,
    },
}

impl EffectDef {
    #[must_use]
    pub const fn damage(recipient: EffectRecipientDef, amount: ValueDef) -> Self {
        Self::DealDamage(DamageDef::new(recipient, amount))
    }

    #[must_use]
    pub const fn damage_from(
        source: ObjectRefDef,
        recipient: EffectRecipientDef,
        amount: ValueDef,
    ) -> Self {
        Self::DealDamage(DamageDef::from_source(source, recipient, amount))
    }

    #[must_use]
    pub const fn damage_simultaneously(assignments: &'static [DamageAssignmentDef]) -> Self {
        Self::DealDamage(DamageDef::simultaneous(assignments))
    }
}
