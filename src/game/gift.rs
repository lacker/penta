//! Gift's choice is an additional cost; giving it is a resolution event.

use super::{Game, ScopedEffect, StackObject, StackObjectKind};
use crate::card::{AbilityDef, OptionalAdditionalCostKindDef};
use crate::{CardDefinitionId, CastSignature};

impl Game {
    pub(super) fn selected_gift_ability(
        &self,
        definition: CardDefinitionId,
        signature: &CastSignature,
    ) -> Option<AbilityDef> {
        let definition = self.catalog.get(definition)?;
        let option = definition.play_option(signature.play_option())?;
        signature.costs().additional().iter().find_map(|cost| {
            let (_, ability, kind) =
                Self::optional_additional_cost_clause(definition, option, *cost)?;
            (kind == OptionalAdditionalCostKindDef::Gift).then_some(ability)
        })
    }

    pub(super) fn spell_gift_effect(&self, object: &StackObject) -> Option<ScopedEffect> {
        if object.kind != StackObjectKind::Spell
            || object.cast.as_ref()?.gift_recipient.is_none()
            || self.stack_spell_types(object)?.is_permanent()
        {
            return None;
        }
        self.selected_gift_ability(
            object.card.definition.card_definition()?,
            object.signature.as_ref()?,
        )?
        .declarative_effect()
        .map(ScopedEffect::primary)
    }

    pub(super) fn capture_resolved_gift(&mut self, object: &StackObject) {
        let gift_trigger = object.kind == StackObjectKind::TriggeredAbility
            && object
                .ability
                .as_ref()
                .and_then(|payload| payload.definition.as_ref())
                .is_some_and(|ability| ability.label == Some(crate::card::sets::bloomburrow::GIFT));
        if gift_trigger || self.spell_gift_effect(object).is_some() {
            self.capture_mechanic(crate::card::sets::bloomburrow::GIFT, object.controller);
        }
    }
}
