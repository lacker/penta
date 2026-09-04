// The alternative-casting half of `AbilityDef`: how a card prints a second
// way to be cast, and the costs that way may carry. Split out of the
// constructor list next door for the source-size budget; included textually,
// so the imports here are that module's.

impl AbilityDef {
    #[must_use]
    pub const fn alternative_cast(
        mana_cost: ManaCost,
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        effect: EffectDef,
    ) -> Self {
        Self::alternative_cast_with_targets(mana_cost, kind, stack_text, &[], effect)
    }

    #[must_use]
    pub const fn alternative_cast_with_targets(
        mana_cost: ManaCost,
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost: AlternativeCastManaCostDef::Fixed(mana_cost),
                kind,
                stack_text,
                targets,
                additional_cost: None,
                condition: None,
                life: 0,
                opponent_life_gain: 0,
                minimum_x: 0,
                from_graveyard: false,
            }),
            effect,
        )
    }

    /// Builds an alternative cast whose nonmana payment is part of that same
    /// casting procedure. Taking the value directly lets set-level mechanic
    /// helpers compose costs from their parameters in a `const fn`.
    #[must_use]
    pub const fn alternative_cast_with_additional_cost(
        mana_cost: AlternativeCastManaCostDef,
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        additional_cost: CostDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost,
                kind,
                stack_text,
                targets: &[],
                additional_cost: Some(additional_cost),
                condition: None,
                life: 0,
                opponent_life_gain: 0,
                minimum_x: 0,
                from_graveyard: false,
            }),
            effect,
        )
    }

    /// Builds an alternative-casting ability whose cost is the mana cost of
    /// the card carrying the ability. This is resolved only after a concrete
    /// spell form has been selected.
    #[must_use]
    pub const fn alternative_cast_for_card_mana_cost(
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost: AlternativeCastManaCostDef::ThisCardManaCost,
                kind,
                stack_text,
                targets: &[],
                additional_cost: None,
                condition: None,
                life: 0,
                opponent_life_gain: 0,
                minimum_x: 0,
                from_graveyard: false,
            }),
            effect,
        )
    }

    /// "Have an opponent gain N life": what this alternative costs, which is
    /// paid where every other cast cost is.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not an alternative cast.
    #[must_use]
    pub const fn with_alternative_opponent_life_gain(mut self, amount: u16) -> Self {
        let DeclarativeAbilityDef::AlternativeCast(mut definition) = self.definition else {
            panic!("only an alternative cast costs an opponent's life gain");
        };
        definition.opponent_life_gain = amount;
        self.definition = DeclarativeAbilityDef::AlternativeCast(definition);
        self
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

    /// A nonmana cost paid in place of a spell's mana cost.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not an alternative cast.
    #[must_use]
    pub const fn with_alternative_additional_cost(
        mut self,
        cost: &'static CostDef,
    ) -> Self {
        let DeclarativeAbilityDef::AlternativeCast(mut definition) = self.definition else {
            panic!("only an alternative cast pays instead of a mana cost");
        };
        definition.additional_cost = Some(*cost);
        self.definition = DeclarativeAbilityDef::AlternativeCast(definition);
        self
    }

    /// Life this alternative pays alongside its mana. An alternative that
    /// names no mana and four life is Snuff Out's whole cost.
    #[must_use]
    pub const fn with_alternative_life(mut self, life: u16) -> Self {
        let DeclarativeAbilityDef::AlternativeCast(mut definition) = self.definition else {
            return self;
        };
        definition.life = life;
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
