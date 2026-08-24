impl TriggeredAbilityDef {
    #[must_use]
    pub const fn new(event: TriggerEventDef) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event,
            targets: &[],
            resolves_with_illegal_targets: false,
            procedure: AbilityProcedureDef::Shared,
            trigger_limit: None,
            condition: None,
            modes: None,
        }
    }

    /// "Choose one --", for a trigger that prints modes.
    #[must_use]
    pub const fn with_modes(mut self, modes: ModalSpellDef) -> Self {
        self.modes = Some(modes);
        self
    }

    /// "This ability triggers only once each turn."
    #[must_use]
    pub const fn triggering_at_most(mut self, times: u8) -> Self {
        self.trigger_limit = Some(times);
        self
    }

    #[must_use]
    pub const fn with_condition(mut self, condition: &'static TriggerConditionDef) -> Self {
        self.condition = Some(condition);
        self
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }

    /// "This ability still resolves if its target becomes illegal."
    #[must_use]
    pub const fn resolving_with_illegal_targets(mut self) -> Self {
        self.resolves_with_illegal_targets = true;
        self
    }

    #[must_use]
    pub const fn with_procedure(mut self, procedure: AbilityProcedureDef) -> Self {
        self.procedure = procedure;
        self
    }
}
