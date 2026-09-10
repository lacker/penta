// The alternative-casting half of `AbilityDef`: how a card prints a second
// way to be cast, and the costs that way may carry. Split out of the
// constructor list next door for the source-size budget; included textually,
// so the imports here are that module's.

impl AbilityDef {
    /// Names this alternative cost for `SourcePaidAlternativeCost` conditions.
    /// The catalog requires names to be unique within their card part.
    ///
    /// # Panics
    ///
    /// Panics if this is not an alternative-casting clause.
    #[must_use]
    pub const fn with_alternative_cost_binding(mut self, binding: crate::Binding) -> Self {
        let DeclarativeAbilityDef::AlternativeCast(mut alternative) = self.definition else {
            panic!("only an alternative cast can declare a cost binding");
        };
        alternative.binding = Some(binding);
        self.definition = DeclarativeAbilityDef::AlternativeCast(alternative);
        self
    }

    #[must_use]
    pub const fn alternative_cast(
        costs: &'static [CostDef],
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        effect: EffectDef,
    ) -> Self {
        Self::alternative_cast_with_targets(costs, kind, stack_text, &[], effect)
    }

    #[must_use]
    pub const fn alternative_cast_with_targets(
        costs: &'static [CostDef],
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                costs,
                kind,
                binding: None,
                cost_header: None,
                stack_text,
                targets,
                condition: None,
                minimum_x: 0,
                from_graveyard: false,
            }),
            effect,
        )
    }

    /// "X can't be 0": the smallest X this alternative may be cast for.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not an alternative cast.
    #[must_use]
    pub const fn with_alternative_minimum_x(mut self, minimum: u16) -> Self {
        let DeclarativeAbilityDef::AlternativeCast(mut definition) = self.definition else {
            panic!("only an alternative cast bounds an X of its own");
        };
        definition.minimum_x = minimum;
        self.definition = DeclarativeAbilityDef::AlternativeCast(definition);
        self
    }

    /// The card's own permission to use this alternative from its owner's
    /// graveyard, which only Detective's Phoenix prints so far.
    ///
    /// # Panics
    ///
    /// Panics if the clause is not an alternative cast.
    #[must_use]
    pub const fn with_alternative_from_graveyard(mut self) -> Self {
        let DeclarativeAbilityDef::AlternativeCast(mut alternative) = self.definition else {
            panic!("only an alternative cast can be permitted from a graveyard");
        };
        alternative.from_graveyard = true;
        self.definition = DeclarativeAbilityDef::AlternativeCast(alternative);
        self
    }

    /// A board condition an alternative cast requires, for a free cast that
    /// is only available while something is true.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not an alternative cast.
    #[must_use]
    pub const fn with_alternative_condition(
        mut self,
        condition: &'static TriggerConditionDef,
    ) -> Self {
        let DeclarativeAbilityDef::AlternativeCast(mut definition) = self.definition else {
            panic!("only an alternative cast has a casting condition");
        };
        definition.condition = Some(condition);
        self.definition = DeclarativeAbilityDef::AlternativeCast(definition);
        self
    }
}
