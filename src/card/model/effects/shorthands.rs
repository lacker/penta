// The two effects written the same way everywhere they appear.
//
// Split out of the vocabulary next door for the source-size budget, and
// included rather than declared so the definitions above stay in scope.

/// The common semantic shape shared by one- and two-armed conditional effects.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ConditionalEffectDef {
    pub condition: &'static TriggerConditionDef,
    pub then: &'static EffectDef,
    pub otherwise: Option<&'static EffectDef>,
}

impl ConditionalEffectDef {
    /// Selects the one branch that should run for an already-evaluated condition.
    #[must_use]
    pub const fn branch(self, condition_holds: bool) -> Option<&'static EffectDef> {
        if condition_holds {
            Some(self.then)
        } else {
            self.otherwise
        }
    }
}

impl EffectDef {
    /// Exposes both conditional variants through one semantic shape so
    /// interpreters cannot give their shared fields different meanings.
    #[must_use]
    pub(crate) const fn conditional(self) -> Option<ConditionalEffectDef> {
        match self {
            Self::IfCondition { condition, then } => Some(ConditionalEffectDef {
                condition,
                then,
                otherwise: None,
            }),
            Self::IfElseCondition {
                condition,
                then,
                otherwise,
            } => Some(ConditionalEffectDef {
                condition,
                then,
                otherwise: Some(otherwise),
            }),
            _ => None,
        }
    }

    #[must_use]
    pub const fn counter_target(target: TargetIndex) -> Self {
        Self::Counter {
            object: EffectRecipientDef::Target(target),
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        }
    }

    #[must_use]
    pub const fn destroy_target(target: TargetIndex) -> Self {
        Self::Destroy {
            object: EffectRecipientDef::Target(target),
            then: None,
        }
    }
}
