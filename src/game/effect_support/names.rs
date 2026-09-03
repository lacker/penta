use std::collections::{BTreeMap, BTreeSet};

use super::*;

impl Game {
    fn names_of_targets(&self, targets: Vec<Target>) -> BTreeSet<String> {
        targets
            .into_iter()
            .filter_map(Self::target_object_id)
            .filter_map(|object| self.object_card_name(object).map(|name| name.into_owned()))
            .collect()
    }

    fn names_appearing_at_least(&self, targets: Vec<Target>, count: u8) -> BTreeSet<String> {
        let mut counts = BTreeMap::<String, usize>::new();
        for name in targets
            .into_iter()
            .filter_map(Self::target_object_id)
            .filter_map(|object| self.object_card_name(object).map(|name| name.into_owned()))
        {
            *counts.entry(name).or_default() += 1;
        }
        counts
            .into_iter()
            .filter_map(|(name, occurrences)| (occurrences >= usize::from(count)).then_some(name))
            .collect()
    }

    fn basic_land_names(&self) -> BTreeSet<String> {
        self.catalog
            .definitions()
            .into_iter()
            .filter(|definition| definition.is_basic_land())
            .flat_map(|definition| definition.parts.iter().map(|part| part.name.clone()))
            .collect()
    }

    pub(in crate::game) fn source_card_name(
        &self,
        name: CardNameDef,
        source: GameObjectId,
    ) -> Option<String> {
        match name {
            CardNameDef::Literal(name) => Some(name.to_owned()),
            CardNameDef::Object(ObjectRefDef::Source) => {
                self.object_card_name(source).map(|name| name.into_owned())
            }
            CardNameDef::Object(ObjectRefDef::AttachedToSource) => self
                .current_or_last_known_attached_host(source)
                .and_then(|host| self.object_card_name(host))
                .map(|name| name.into_owned()),
            CardNameDef::SourceChoice => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .and_then(|permanent| permanent.chosen_card_name.clone()),
            CardNameDef::EffectChoice | CardNameDef::Object(_) => None,
        }
    }

    pub(in crate::game) fn source_card_name_set(
        &self,
        names: CardNameSetDef,
        source: GameObjectId,
    ) -> BTreeSet<String> {
        match names {
            CardNameSetDef::NamesOf(objects) => {
                self.names_of_targets(self.source_object_set_targets(*objects, source))
            }
            CardNameSetDef::NamesAppearingAtLeast { objects, count } => self
                .names_appearing_at_least(self.source_object_set_targets(*objects, source), count),
            CardNameSetDef::BasicLandNames => self.basic_land_names(),
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
            CardNameDef::Object(reference) => self
                .object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.object_card_name(referenced))
                .map(|name| name.into_owned()),
            CardNameDef::EffectChoice => context.chosen_name.clone(),
            CardNameDef::SourceChoice => object
                .source
                .and_then(|source| {
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == source)
                })
                .and_then(|permanent| permanent.chosen_card_name.clone()),
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
            CardNameSetDef::NamesOf(objects) => {
                self.names_of_targets(self.effect_objects(*objects, object, context, scoped))
            }
            CardNameSetDef::NamesAppearingAtLeast { objects, count } => self
                .names_appearing_at_least(
                    self.effect_objects(*objects, object, context, scoped),
                    count,
                ),
            CardNameSetDef::BasicLandNames => self.basic_land_names(),
        }
    }
}
