use std::borrow::Cow;
use std::hash::{Hash, Hasher};

use crate::AbilityId;

use super::{AbilityDef, CardRules, ImplementationStatus};

/// The only characteristics an emblem has: a display name and abilities.
///
/// Emblems are not cards, have no type line or card part, and never enter the
/// card catalog. The creating effect owns this compact value directly.
#[derive(Clone, Copy, Debug)]
pub struct EmblemCharacteristics {
    name: &'static str,
    abilities: &'static [AbilityDef],
}

impl EmblemCharacteristics {
    #[must_use]
    pub const fn new(name: &'static str, abilities: &'static [AbilityDef]) -> Self {
        Self { name, abilities }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn abilities(self) -> &'static [AbilityDef] {
        self.abilities
    }

    #[must_use]
    pub fn ability(self, id: AbilityId) -> Option<AbilityDef> {
        self.abilities.get(usize::from(id.0)).copied()
    }

    /// The creator-owned abilities formatted with the same presentation
    /// semantics as card rules.
    #[must_use]
    pub fn rules_text(self) -> Cow<'static, str> {
        self.rules_view().rules_text()
    }

    /// The aggregate implementation coverage of the emblem's abilities.
    #[must_use]
    pub fn implementation_status(self) -> ImplementationStatus {
        self.rules_view().implementation_status()
    }

    /// Internal adapter for shared ability/layer machinery. The resulting
    /// rules have no card types, mana cost, or other card characteristics.
    pub(crate) const fn rules_view(self) -> CardRules {
        CardRules::from_emblem_abilities(self.abilities)
    }
}

impl PartialEq for EmblemCharacteristics {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.abilities.len() == other.abilities.len()
            && std::ptr::eq(self.abilities.as_ptr(), other.abilities.as_ptr())
    }
}

impl Eq for EmblemCharacteristics {}

impl Hash for EmblemCharacteristics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.abilities.len().hash(state);
        self.abilities.as_ptr().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_copy_hash<T: Copy + Hash>() {}

    #[test]
    fn emblem_characteristics_are_compact_copyable_values() {
        assert_copy_hash::<EmblemCharacteristics>();
        assert_eq!(
            std::mem::size_of::<EmblemCharacteristics>(),
            4 * std::mem::size_of::<usize>(),
        );
    }

    #[test]
    fn emblem_presentation_uses_shared_declarative_rules() {
        static ABILITIES: [AbilityDef; 2] = [
            AbilityDef::activated(
                "Complete emblem ability.",
                &[],
                crate::card::EffectDef::None,
            ),
            AbilityDef::activated("Second emblem ability.", &[], crate::card::EffectDef::None),
        ];
        let emblem = EmblemCharacteristics::new("Test emblem", &ABILITIES);

        assert_eq!(
            emblem.rules_text(),
            "Complete emblem ability.\nSecond emblem ability."
        );
        assert_eq!(
            emblem.implementation_status(),
            ImplementationStatus::Complete
        );
    }
}
