use super::{
    AbilityDef, CardRules, CardSupertype, CardType, CardTypeSet, CreatureStats, ManaColor,
    inline_rules::InlineRules,
};

/// The copiable characteristics supplied by the rule, ability, or effect that
/// allows a spell or permanent to be face down (CR 708.2).
///
/// This is presentation, not object identity. The physical card underneath
/// retains its own [`crate::CardDefinitionId`], while the face-down object
/// carries these complete rules inline. Different face-down mechanisms may
/// share the same value, and card-specific effects may construct another one
/// without inventing a catalog definition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FaceDownCharacteristics {
    display_name: &'static str,
    rules: InlineRules,
}

impl FaceDownCharacteristics {
    /// Builds arbitrary face-down characteristics listed by a rule, ability,
    /// or effect. `display_name` is only a client label; face-down objects have
    /// no rules-visible name unless the mechanism model is extended to say so.
    #[must_use]
    pub const fn new(
        display_name: &'static str,
        card_types: CardTypeSet,
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        creature_stats: Option<CreatureStats>,
    ) -> Self {
        Self {
            display_name,
            rules: InlineRules::new(card_types, subtypes, colors, creature_stats),
        }
    }

    #[must_use]
    pub const fn creature(
        display_name: &'static str,
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        power: i16,
        toughness: i16,
    ) -> Self {
        Self::new(
            display_name,
            CardTypeSet::single(CardType::Creature),
            subtypes,
            colors,
            Some(CreatureStats { power, toughness }),
        )
    }

    #[must_use]
    pub const fn artifact_creature(
        display_name: &'static str,
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        power: i16,
        toughness: i16,
    ) -> Self {
        Self::new(
            display_name,
            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
            subtypes,
            colors,
            Some(CreatureStats { power, toughness }),
        )
    }

    #[must_use]
    pub const fn land(display_name: &'static str, subtypes: &'static [&'static str]) -> Self {
        Self::new(
            display_name,
            CardTypeSet::single(CardType::Land),
            subtypes,
            &[],
            None,
        )
    }

    #[must_use]
    pub const fn with_type(mut self, card_type: CardType) -> Self {
        self.rules = self.rules.with_type(card_type);
        self
    }

    #[must_use]
    pub const fn with_supertype(mut self, supertype: CardSupertype) -> Self {
        self.rules = self.rules.with_supertype(supertype);
        self
    }

    #[must_use]
    pub const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        self.rules = self.rules.with_abilities(abilities);
        self
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        self.display_name
    }

    #[must_use]
    pub const fn rules(self) -> CardRules {
        self.rules.materialize()
    }
}
