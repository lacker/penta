// The two effects written the same way everywhere they appear.
//
// Split out of the vocabulary next door for the source-size budget, and
// included rather than declared so the definitions above stay in scope.

impl EffectDef {
    #[must_use]
    pub const fn counter_target(target: TargetIndex) -> Self {
        Self::Counter {
            object: EffectRecipientDef::Target(target),
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        }
    }

    #[must_use]
    pub const fn destroy_target(target: TargetIndex, can_regenerate: bool) -> Self {
        Self::Destroy {
            object: EffectRecipientDef::Target(target),
            can_regenerate,
            then: None,
        }
    }
}
