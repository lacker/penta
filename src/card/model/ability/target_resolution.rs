impl AbilityDef {
    /// Suppresses rule 608.2b's ordinary all-targets-illegal early exit for a
    /// triggered ability whose Oracle text explicitly says it still resolves.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not an executable declarative targeted
    /// trigger.
    #[must_use]
    pub const fn resolves_with_illegal_targets(self) -> Self {
        let DeclarativeAbilityDef::Triggered(definition) = self.definition else {
            panic!("only a triggered ability can carry this resolution exception");
        };
        assert!(
            !definition.targets.is_empty(),
            "a target-resolution exception requires a targeted ability",
        );
        Self {
            definition: DeclarativeAbilityDef::Triggered(
                definition.resolving_with_illegal_targets(),
            ),
            ..self
        }
    }
}
