use std::borrow::Cow;
use std::hash::{Hash, Hasher};

use crate::ids::CardPartId;

use super::{
    AbilityDef, CardArt, CardRules, CardSupertype, CardType, CardTypeSet, ColorSet, CreatureStats,
    ManaColor,
};

fn derived_token_name(
    explicit: Option<&'static str>,
    subtypes: &'static [&'static str],
) -> Cow<'static, str> {
    if let Some(name) = explicit {
        return Cow::Borrowed(name);
    }
    match subtypes {
        [] => Cow::Borrowed("Token"),
        [subtype] => Cow::Borrowed(subtype),
        _ => Cow::Owned(subtypes.join(" ")),
    }
}

/// Compact rules for one ordinary token face.
///
/// Abilities stay behind a slice because an ability can itself create a
/// token. Storing an [`AbilityDef`] inline here would make the declarative
/// effect schema recursively sized.
#[derive(Clone, Copy, Debug)]
struct TokenRules {
    card_types: CardTypeSet,
    supertypes: [bool; CardSupertype::COUNT],
    subtypes: &'static [&'static str],
    colors: ColorSet,
    creature_stats: Option<CreatureStats>,
    abilities: &'static [AbilityDef],
}

impl PartialEq for TokenRules {
    fn eq(&self, other: &Self) -> bool {
        self.card_types == other.card_types
            && self.supertypes == other.supertypes
            && self.subtypes == other.subtypes
            && self.colors == other.colors
            && self.creature_stats == other.creature_stats
            && std::ptr::eq(self.abilities, other.abilities)
    }
}

impl Eq for TokenRules {}

impl Hash for TokenRules {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.card_types.hash(state);
        self.supertypes.hash(state);
        self.subtypes.hash(state);
        self.colors.hash(state);
        self.creature_stats.hash(state);
        self.abilities.as_ptr().hash(state);
        self.abilities.len().hash(state);
    }
}

impl TokenRules {
    const fn new(
        card_types: CardTypeSet,
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        creature_stats: Option<CreatureStats>,
    ) -> Self {
        Self {
            card_types,
            supertypes: [false; CardSupertype::COUNT],
            subtypes,
            colors: ColorSet::from_colors(colors),
            creature_stats,
            abilities: &[],
        }
    }

    const fn with_type(mut self, card_type: CardType) -> Self {
        self.card_types = self.card_types.with(card_type);
        self
    }

    const fn with_supertype(mut self, supertype: CardSupertype) -> Self {
        self.supertypes[supertype.index()] = true;
        self
    }

    const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        self.abilities = abilities;
        self
    }

    const fn materialize(self) -> CardRules {
        CardRules::from_token_characteristics(
            self.card_types,
            self.supertypes,
            self.subtypes,
            self.colors,
            self.creature_stats,
            self.abilities,
        )
    }
}

/// The characteristics of one exceptional secondary token face.
///
/// The rare transforming token holds this full value behind a static
/// reference so ordinary tokens do not pay for a second face.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TokenPart {
    pub id: CardPartId,
    explicit_name: Option<&'static str>,
    pub rules: CardRules,
}

impl TokenPart {
    #[must_use]
    pub const fn new(id: CardPartId, name: &'static str, rules: CardRules) -> Self {
        Self {
            id,
            explicit_name: Some(name),
            rules,
        }
    }

    #[must_use]
    pub const fn derived(id: CardPartId, rules: CardRules) -> Self {
        Self {
            id,
            explicit_name: None,
            rules,
        }
    }

    #[must_use]
    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.explicit_name = Some(name);
        self
    }

    #[must_use]
    pub fn name(self) -> Cow<'static, str> {
        derived_token_name(self.explicit_name, self.rules.subtypes())
    }

    #[must_use]
    pub const fn rules(self) -> CardRules {
        self.rules
    }
}

/// The physical face arrangement authored by a token-creating effect.
#[derive(Clone, Copy, Debug)]
pub enum TokenStructure {
    Single,
    TransformingDoubleFaced { back: &'static TokenPart },
}

impl PartialEq for TokenStructure {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single, Self::Single) => true,
            (
                Self::TransformingDoubleFaced { back: left },
                Self::TransformingDoubleFaced { back: right },
            ) => std::ptr::eq(*left, *right),
            (Self::Single, Self::TransformingDoubleFaced { .. })
            | (Self::TransformingDoubleFaced { .. }, Self::Single) => false,
        }
    }
}

impl Eq for TokenStructure {}

impl Hash for TokenStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Single => 0_u8.hash(state),
            Self::TransformingDoubleFaced { back } => {
                1_u8.hash(state);
                std::ptr::from_ref(*back).hash(state);
            }
        }
    }
}

/// Complete characteristics needed to create a token without a card catalog
/// definition or separately named global constant.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TokenCharacteristics {
    explicit_name: Option<&'static str>,
    pub art: Option<CardArt>,
    rules: TokenRules,
    pub structure: TokenStructure,
}

impl TokenCharacteristics {
    /// Builds an arbitrary described token face. Prefer the creature and
    /// artifact helpers for the common Oracle-text shapes.
    #[must_use]
    pub const fn new(
        card_types: CardTypeSet,
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        creature_stats: Option<CreatureStats>,
    ) -> Self {
        Self {
            explicit_name: None,
            art: None,
            rules: TokenRules::new(card_types, subtypes, colors, creature_stats),
            structure: TokenStructure::Single,
        }
    }

    #[must_use]
    pub const fn creature(
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        power: i16,
        toughness: i16,
    ) -> Self {
        Self::new(
            CardTypeSet::single(CardType::Creature),
            subtypes,
            colors,
            Some(CreatureStats { power, toughness }),
        )
    }

    #[must_use]
    pub const fn artifact_creature(
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        power: i16,
        toughness: i16,
    ) -> Self {
        Self::new(
            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
            subtypes,
            colors,
            Some(CreatureStats { power, toughness }),
        )
    }

    #[must_use]
    pub const fn artifact(subtypes: &'static [&'static str], colors: &'static [ManaColor]) -> Self {
        Self::new(
            CardTypeSet::single(CardType::Artifact),
            subtypes,
            colors,
            None,
        )
    }

    #[must_use]
    pub const fn with_art(mut self, art: CardArt) -> Self {
        self.art = Some(art);
        self
    }

    #[must_use]
    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.explicit_name = Some(name);
        self
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
    pub const fn transforming_into(mut self, back: &'static TokenPart) -> Self {
        self.structure = TokenStructure::TransformingDoubleFaced { back };
        self
    }

    #[must_use]
    pub fn name(self) -> Cow<'static, str> {
        derived_token_name(self.explicit_name, self.rules.subtypes)
    }

    #[must_use]
    pub const fn rules(self) -> CardRules {
        self.rules.materialize()
    }

    /// A cycle-safe key for semantic discovery and validation. Equality and
    /// hashing compare ordinary scalar characteristics and subtype contents,
    /// but deliberately treat the ability slice and rare back face by their
    /// creator-owned static addresses rather than recursively walking an
    /// ability that may create this token again.
    #[must_use]
    pub(crate) const fn semantic_identity(self) -> Self {
        self
    }

    #[must_use]
    pub const fn primary_part_id(self) -> CardPartId {
        CardPartId::PRIMARY
    }

    #[must_use]
    pub const fn primary_part(self) -> TokenPart {
        TokenPart {
            id: CardPartId::PRIMARY,
            explicit_name: self.explicit_name,
            rules: self.rules.materialize(),
        }
    }

    #[must_use]
    pub const fn part(self, id: CardPartId) -> Option<TokenPart> {
        if id.0 == CardPartId::PRIMARY.0 {
            return Some(self.primary_part());
        }
        match self.structure {
            TokenStructure::TransformingDoubleFaced { back } if back.id.0 == id.0 => Some(*back),
            TokenStructure::Single | TokenStructure::TransformingDoubleFaced { .. } => None,
        }
    }

    #[must_use]
    pub const fn other_face(self, presented: CardPartId) -> Option<CardPartId> {
        let TokenStructure::TransformingDoubleFaced { back } = self.structure else {
            return None;
        };
        if presented.0 == CardPartId::PRIMARY.0 {
            Some(back.id)
        } else if presented.0 == back.id.0 {
            Some(CardPartId::PRIMARY)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{hash::Hash, mem::size_of};

    use super::*;

    fn assert_copy_hash<T: Copy + Hash>() {}

    #[test]
    fn token_schema_is_copy_and_hashable() {
        assert_copy_hash::<TokenPart>();
        assert_copy_hash::<TokenStructure>();
        assert_copy_hash::<TokenCharacteristics>();
    }

    #[test]
    fn compact_token_characteristics_stay_smaller_than_an_effect() {
        assert!(
            size_of::<TokenCharacteristics>() <= 128,
            "token characteristics exceeded their 128-byte inline budget",
        );
        assert!(
            size_of::<TokenCharacteristics>() < size_of::<super::super::EffectDef>(),
            "token characteristics should remain a compact inline effect payload",
        );
    }

    #[test]
    fn ordinary_names_are_derived_from_all_subtypes() {
        let token = TokenCharacteristics::artifact_creature(&["Phyrexian", "Wurm"], &[], 3, 3);
        assert_eq!(token.name(), "Phyrexian Wurm");
        assert_eq!(
            TokenCharacteristics::artifact_creature(&["Insect"], &[], 1, 1)
                .with_name("Wasp")
                .name(),
            "Wasp",
        );
    }

    #[test]
    fn single_and_transforming_parts_are_addressable() {
        static PHYREXIAN: TokenPart = TokenPart::derived(
            CardPartId(1),
            CardRules::new_artifact_creature_without_mana_cost(&["Phyrexian"], 0, 0),
        );
        let single = TokenCharacteristics::creature(&["Goblin"], &[ManaColor::Red], 1, 1);
        assert_eq!(single.primary_part_id(), CardPartId::PRIMARY);
        assert_eq!(
            single
                .part(CardPartId::PRIMARY)
                .map(TokenPart::name)
                .as_deref(),
            Some("Goblin"),
        );
        assert_eq!(single.part(CardPartId(1)), None);
        assert_eq!(single.other_face(CardPartId::PRIMARY), None);

        let transforming =
            TokenCharacteristics::artifact(&["Incubator"], &[]).transforming_into(&PHYREXIAN);
        assert_eq!(
            transforming
                .part(CardPartId(1))
                .map(TokenPart::name)
                .as_deref(),
            Some("Phyrexian"),
        );
        assert_eq!(
            transforming.other_face(CardPartId::PRIMARY),
            Some(CardPartId(1)),
        );
        assert_eq!(
            transforming.other_face(CardPartId(1)),
            Some(CardPartId::PRIMARY),
        );
    }
}
