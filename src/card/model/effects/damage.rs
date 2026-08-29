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
