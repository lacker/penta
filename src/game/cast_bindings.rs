//! Named player choices paid while casting and declarative completion events.
use super::{Game, PlayerId, StackObject};
use crate::card::OptionalAdditionalCostKindDef;
use crate::{CardDefinitionId, CastSignature};
use std::collections::BTreeMap;

impl Game {
    pub(super) fn selected_cast_player_bindings(
        &self,
        definition: CardDefinitionId,
        signature: &CastSignature,
        player: PlayerId,
    ) -> BTreeMap<String, PlayerId> {
        let Some(definition) = self.catalog.get(definition) else {
            return BTreeMap::new();
        };
        let Some(option) = definition.play_option(signature.play_option()) else {
            return BTreeMap::new();
        };
        signature
            .costs()
            .additional()
            .iter()
            .filter_map(|cost| {
                let (_, _, kind) =
                    Self::optional_additional_cost_clause(definition, option, *cost)?;
                let OptionalAdditionalCostKindDef::ChooseOpponent(binding) = kind else {
                    return None;
                };
                // The two-player rules have exactly one eligible opponent. The
                // optional cost selection chooses that player without targeting.
                Some((binding.label()?.to_owned(), player.opponent()))
            })
            .collect()
    }

    pub(super) fn capture_resolution_completion(&mut self, object: &StackObject) {
        let Some(payload) = object.ability.as_ref() else {
            return;
        };
        let Some((mechanic, condition)) = payload
            .definition
            .as_ref()
            .and_then(|ability| ability.resolution_event)
        else {
            return;
        };
        if condition.is_none_or(|condition| {
            self.trigger_condition_holds(
                condition,
                object.source.unwrap_or(object.id),
                object.controller,
                payload.context.trigger,
                Some(payload.origin),
                Some((
                    object,
                    &super::ScopedEffect::primary(crate::card::EffectDef::None),
                    &payload.context,
                )),
            )
        }) {
            self.capture_mechanic(mechanic, object.controller);
        }
    }
}

impl Game {
    pub(super) fn object_for_effect_clause<'a>(
        &self,
        scoped: super::ScopedEffect,
        object: &'a StackObject,
    ) -> std::borrow::Cow<'a, StackObject> {
        let Some(origin) = scoped
            .clause_origin
            .filter(|origin| Some(*origin) != object.ability_origin())
        else {
            return std::borrow::Cow::Borrowed(object);
        };
        let definition = match origin {
            super::AbilityOrigin::Printed {
                definition,
                part,
                ability,
            } => self
                .catalog
                .get(definition)
                .and_then(|card| card.part(part))
                .and_then(|part| part.rules.ability(ability))
                .copied(),
            _ => self.ability_for_origin(object.source.unwrap_or(object.id), origin),
        }
        .expect("a frozen clause retains its authored definition");
        let mut local = object.clone();
        let payload = local
            .ability
            .as_mut()
            .expect("a composed spell has a payload");
        payload.origin = origin;
        payload.definition = Some(Box::new(definition));
        payload.resolver = self.cached_ability_resolver(origin, &definition);
        payload.text = Some(definition.text);
        std::borrow::Cow::Owned(local)
    }
}
