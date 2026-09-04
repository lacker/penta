// Constructors for clauses that act before the first turn.
//
// Split out of `ability.rs` for the source-size budget, and split here
// because these clauses share a timing nothing else has: they are taken
// while opening hands are being settled, before any player has priority.
// Included textually, so the imports here are that module's.

impl AbilityDef {
    /// A rules-defined action taken before the first turn, without using the
    /// stack. Its timing is structural rather than inferred from Oracle text.
    #[must_use]
    pub const fn pregame(
        text: &'static str,
        timing: PregameTimingDef,
        condition: PregameConditionDef,
        costs: &'static [CostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Pregame(
                PregameAbilityDef::new(timing)
                    .with_condition(condition)
                    .with_costs(costs),
            ),
            effect,
        )
    }

    #[must_use]
    pub const fn opening_hand(text: &'static str, effect: EffectDef) -> Self {
        Self::pregame(
            text,
            PregameTimingDef::OpeningHand,
            PregameConditionDef::Always,
            &[],
            effect,
        )
    }

    #[must_use]
    pub const fn opening_hand_reveal(text: &'static str, effect: EffectDef) -> Self {
        let mut ability = Self::opening_hand(text, effect);
        let DeclarativeAbilityDef::Pregame(definition) = ability.definition else {
            unreachable!()
        };
        ability.definition = DeclarativeAbilityDef::Pregame(definition.revealing_source());
        ability
    }

    #[must_use]
    pub const fn opening_hand_with(
        text: &'static str,
        condition: PregameConditionDef,
        costs: &'static [CostDef],
        effect: EffectDef,
    ) -> Self {
        Self::pregame(
            text,
            PregameTimingDef::OpeningHand,
            condition,
            costs,
            effect,
        )
    }

    #[must_use]
    pub const fn mulligan_action(text: &'static str, effect: EffectDef) -> Self {
        Self::pregame(
            text,
            PregameTimingDef::Mulligan,
            PregameConditionDef::Always,
            &[],
            effect,
        )
    }
}
