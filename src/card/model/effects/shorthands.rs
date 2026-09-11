// Common effect constructors and a normalized conditional shape.
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
    /// Move already identified objects using the shared game action.
    /// Placement selects the end of a library and is ignored for other zones.
    #[must_use]
    pub const fn move_to_zone(
        object: EffectRecipientDef,
        zone: ZoneKind,
        placement: ZonePlacement,
    ) -> Self {
        super::actions::move_to_zone(object, zone, placement).as_effect()
    }

    /// Discard already identified cards using the shared game action.
    /// Introduces no selection; compose `actions::choose_discard()` when needed.
    #[must_use]
    pub const fn discard_cards(object: EffectRecipientDef) -> Self {
        super::actions::discard_cards(object).as_effect()
    }

    /// Have each named permanent's controller sacrifice it.
    #[must_use]
    pub const fn sacrifice(object: EffectRecipientDef) -> Self {
        super::actions::sacrifice(object).as_effect()
    }

    /// Have the executing player sacrifice the named permanents they control.
    #[must_use]
    pub const fn sacrifice_yours(object: EffectRecipientDef) -> Self {
        super::actions::sacrifice_yours(object).as_effect()
    }

    /// Give the named player control for the specified duration.
    #[must_use]
    pub const fn gain_control(
        object: EffectRecipientDef,
        controller: PlayerRefDef,
        duration: ControlDurationDef,
    ) -> Self {
        super::actions::gain_control(object, controller, duration).as_effect()
    }

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
