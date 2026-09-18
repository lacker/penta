//! Immutable baseline rules, shared across board reads and game clones.
//!
//! Keys describe complete copiable characteristics, never an object ID or an
//! effective layered result. Changing an object's face, copy, words, colors, or
//! creation stats selects a different key; board mutation needs no invalidation.

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use super::{CardRules, ObjectCharacteristics};

/// Printed rules borrow the catalog; inline rules retain a shared materialization.
pub(super) enum RulesView<'a> {
    Printed(&'a CardRules),
    Inline(Arc<CardRules>),
}

impl Deref for RulesView<'_> {
    type Target = CardRules;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Printed(rules) => rules,
            Self::Inline(rules) => rules,
        }
    }
}

/// Runtime-created characteristics can have arbitrarily many distinct values.
/// Bound retention, not legal characteristics: eviction only causes a rebuild.
const MAX_RETAINED_RULES: usize = 128;

#[derive(Clone, Debug, Default)]
pub(super) struct InlineRulesCache {
    rules: Arc<Mutex<HashMap<ObjectCharacteristics, Arc<CardRules>>>>,
}

impl InlineRulesCache {
    pub(super) fn get(&self, source: ObjectCharacteristics) -> Option<Arc<CardRules>> {
        {
            let rules = self
                .rules
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(found) = rules.get(&source) {
                return Some(Arc::clone(found));
            }
        }
        let materialized = Arc::new(match source {
            ObjectCharacteristics::Card { .. } => return None,
            ObjectCharacteristics::Token { token, part } => token.part(part)?.rules,
            ObjectCharacteristics::Emblem { emblem } => emblem.rules_view(),
            ObjectCharacteristics::FaceDown { face_down } => face_down.rules(),
        });
        let mut rules = self
            .rules
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(found) = rules.get(&source) {
            return Some(Arc::clone(found));
        }
        if rules.len() >= MAX_RETAINED_RULES {
            rules.clear();
        }
        rules.insert(source, Arc::clone(&materialized));
        Some(materialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CardPartId, CreatureStats, TokenCharacteristics};

    #[test]
    fn rules_cache_shares_exact_characteristics_and_separates_runtime_values() {
        let cache = InlineRulesCache::default();
        let token = TokenCharacteristics::creature(&["Zombie"], &[], 2, 2);
        let key = ObjectCharacteristics::token(token, CardPartId::PRIMARY);
        let original = cache.get(key).unwrap();
        assert!(Arc::ptr_eq(&original, &cache.clone().get(key).unwrap()));

        let changed = token.with_resolved_stats(7, 4);
        let changed_rules = cache
            .get(ObjectCharacteristics::token(changed, CardPartId::PRIMARY))
            .unwrap();
        assert_eq!(
            changed_rules.creature_stats(),
            Some(CreatureStats {
                power: 7,
                toughness: 4
            })
        );
        assert_eq!(
            original.creature_stats(),
            Some(CreatureStats {
                power: 2,
                toughness: 2
            })
        );
        assert!(!Arc::ptr_eq(&original, &changed_rules));
        assert!(
            cache
                .get(ObjectCharacteristics::token(token, CardPartId(99)))
                .is_none()
        );
    }

    #[test]
    fn rules_cache_eviction_preserves_outstanding_views_and_allows_reconstruction() {
        let cache = InlineRulesCache::default();
        let token = TokenCharacteristics::creature(&["Zombie"], &[], 2, 2);
        let key = ObjectCharacteristics::token(token, CardPartId::PRIMARY);
        let original = cache.get(key).unwrap();
        for power in 0..=MAX_RETAINED_RULES {
            let token = token.with_resolved_stats(i16::try_from(power).unwrap(), 3);
            let source = ObjectCharacteristics::token(token, CardPartId::PRIMARY);
            assert_eq!(
                cache.get(source).unwrap().creature_stats().unwrap().power,
                i16::try_from(power).unwrap()
            );
        }
        assert_eq!(*original, *cache.get(key).unwrap());
        assert!(cache.rules.lock().unwrap().len() <= MAX_RETAINED_RULES);
    }

    #[test]
    fn rules_cache_keeps_faces_colors_and_word_maps_distinct() {
        use crate::card::{BasicLandType, ColorSet, ManaColor, TokenPart};

        static BACK: TokenPart = TokenPart::new(
            CardPartId(1),
            "Back",
            TokenCharacteristics::creature(&["Zombie"], &[], 5, 5).rules(),
        );
        let cache = InlineRulesCache::default();
        let front = TokenCharacteristics::creature(&["Zombie"], &[], 2, 2).transforming_into(&BACK);
        let blue = front.with_color_set(ColorSet::from_colors(&[ManaColor::Blue]));
        let words = blue.with_word_maps([BasicLandType::Island; 5], [ManaColor::Red; 5]);
        let cases = [
            (front, CardPartId::PRIMARY),
            (front, CardPartId(1)),
            (blue, CardPartId::PRIMARY),
            (words, CardPartId::PRIMARY),
        ];
        for (token, part) in cases.into_iter().chain(cases.into_iter().rev()) {
            assert_eq!(
                *cache
                    .get(ObjectCharacteristics::token(token, part))
                    .unwrap(),
                token.part(part).unwrap().rules
            );
        }
    }
}
