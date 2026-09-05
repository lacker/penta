use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use crate::{Binding, CardRules, CardSet, CardSupertype, CardType};

use super::{
    CardNameDef, CardNameSetDef, EffectResolutionContext, Game, GameObjectId, ObjectRefDef,
    ScopedEffect, StackObject, Target,
};

impl Game {
    fn names_of_targets(&self, targets: Vec<Target>) -> BTreeSet<String> {
        targets
            .into_iter()
            .filter_map(Self::target_object_id)
            .filter_map(|object| self.object_card_name(object).map(Cow::into_owned))
            .collect()
    }

    fn names_appearing_at_least(&self, targets: Vec<Target>, count: u8) -> BTreeSet<String> {
        let mut counts = BTreeMap::<String, usize>::new();
        for name in targets
            .into_iter()
            .filter_map(Self::target_object_id)
            .filter_map(|object| self.object_card_name(object).map(Cow::into_owned))
        {
            *counts.entry(name).or_default() += 1;
        }
        counts
            .into_iter()
            .filter_map(|(name, occurrences)| (occurrences >= usize::from(count)).then_some(name))
            .collect()
    }

    pub(in crate::game) fn catalog_card_names(
        &self,
        names: CardNameSetDef,
    ) -> Option<BTreeSet<String>> {
        if let CardNameSetDef::Union(sets) = names {
            let mut union = BTreeSet::new();
            for names in sets {
                union.extend(self.catalog_card_names(*names)?);
            }
            return Some(union);
        }
        if matches!(
            names,
            CardNameSetDef::NamesOf(_) | CardNameSetDef::NamesAppearingAtLeast { .. }
        ) {
            return None;
        }
        let matches = |rules: &CardRules| match names {
            CardNameSetDef::AllCardNames => true,
            CardNameSetDef::NonlandCardNames => !rules.has_type(CardType::Land),
            CardNameSetDef::LandCardNames => rules.has_type(CardType::Land),
            CardNameSetDef::NonbasicLandCardNames => {
                rules.has_type(CardType::Land) && !rules.has_supertype(CardSupertype::Basic)
            }
            CardNameSetDef::CardNamesOtherThanBasicLands => {
                !rules.has_type(CardType::Land) || !rules.has_supertype(CardSupertype::Basic)
            }
            CardNameSetDef::BasicLandNames => {
                rules.has_type(CardType::Land) && rules.has_supertype(CardSupertype::Basic)
            }
            CardNameSetDef::NamesOf(_)
            | CardNameSetDef::Union(_)
            | CardNameSetDef::NamesAppearingAtLeast { .. } => false,
        };
        Some(
            self.catalog
                .definitions()
                .into_iter()
                .filter(|definition| definition.debut_set != CardSet::Token)
                .flat_map(|definition| definition.parts.iter())
                .filter(|part| matches(&part.rules))
                .map(|part| part.name.clone())
                .collect(),
        )
    }

    fn permanent_bound_card_name(&self, source: GameObjectId, binding: Binding) -> Option<String> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| {
                (permanent.chosen_card_name_binding.is_none()
                    || permanent.chosen_card_name_binding == Some(binding))
                .then(|| permanent.chosen_card_name.clone())
                .flatten()
            })
    }

    pub(in crate::game) fn source_card_name(
        &self,
        name: CardNameDef,
        source: GameObjectId,
    ) -> Option<String> {
        match name {
            CardNameDef::Literal(name) => Some(name.to_owned()),
            CardNameDef::NameOf(ObjectRefDef::Source) => {
                self.object_card_name(source).map(Cow::into_owned)
            }
            CardNameDef::NameOf(ObjectRefDef::AttachedToSource) => self
                .current_or_last_known_attached_host(source)
                .and_then(|host| self.object_card_name(host))
                .map(Cow::into_owned),
            CardNameDef::Binding(binding) => self.permanent_bound_card_name(source, binding),
            CardNameDef::NameOf(_) => None,
        }
    }

    pub(in crate::game) fn source_card_name_set(
        &self,
        names: CardNameSetDef,
        source: GameObjectId,
    ) -> BTreeSet<String> {
        match names {
            CardNameSetDef::AllCardNames
            | CardNameSetDef::NonlandCardNames
            | CardNameSetDef::LandCardNames
            | CardNameSetDef::NonbasicLandCardNames
            | CardNameSetDef::CardNamesOtherThanBasicLands
            | CardNameSetDef::BasicLandNames => self.catalog_card_names(names).unwrap_or_default(),
            CardNameSetDef::NamesOf(objects) => {
                self.names_of_targets(self.source_object_set_targets(*objects, source))
            }
            CardNameSetDef::Union(sets) => {
                let mut union = BTreeSet::new();
                for names in sets {
                    union.extend(self.source_card_name_set(*names, source));
                }
                union
            }
            CardNameSetDef::NamesAppearingAtLeast { objects, count } => self
                .names_appearing_at_least(self.source_object_set_targets(*objects, source), count),
        }
    }

    pub(in crate::game) fn effect_card_name(
        &self,
        name: CardNameDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<String> {
        match name {
            CardNameDef::Literal(name) => Some(name.to_owned()),
            CardNameDef::NameOf(reference) => self
                .object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.object_card_name(referenced))
                .map(Cow::into_owned),
            CardNameDef::Binding(binding) => context.card_name(binding).or_else(|| {
                object
                    .source
                    .and_then(|source| self.permanent_bound_card_name(source, binding))
            }),
        }
    }

    pub(in crate::game) fn effect_card_name_set(
        &self,
        names: CardNameSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> BTreeSet<String> {
        match names {
            CardNameSetDef::AllCardNames
            | CardNameSetDef::NonlandCardNames
            | CardNameSetDef::LandCardNames
            | CardNameSetDef::NonbasicLandCardNames
            | CardNameSetDef::CardNamesOtherThanBasicLands
            | CardNameSetDef::BasicLandNames => self.catalog_card_names(names).unwrap_or_default(),
            CardNameSetDef::NamesOf(objects) => {
                self.names_of_targets(self.effect_objects(*objects, object, context, scoped))
            }
            CardNameSetDef::Union(sets) => {
                let mut union = BTreeSet::new();
                for names in sets {
                    union.extend(self.effect_card_name_set(*names, object, context, scoped));
                }
                union
            }
            CardNameSetDef::NamesAppearingAtLeast { objects, count } => self
                .names_appearing_at_least(
                    self.effect_objects(*objects, object, context, scoped),
                    count,
                ),
        }
    }
}
