//! Alternative activation costs replace the printed payment before taxes.
use super::super::{Game, Permanent, PlayerId, TriggerContext};
use crate::card::{
    AbilityDef, ActivatedAbilityDef, CostDef, CostModificationDef, DeclarativeAbilityDef, EffectDef,
};
use crate::{AlternativeAbilityCost, GameObjectId};

impl Game {
    pub(in crate::game) fn alternative_activation_costs(
        &self,
        player: PlayerId,
        permanent: &Permanent,
        activated: &ActivatedAbilityDef,
    ) -> Vec<(AlternativeAbilityCost, &'static [CostDef])> {
        let mut offered = Vec::new();
        for provider in &self.battlefield {
            self.for_each_effective_ability(provider, |effective| {
                let DeclarativeAbilityDef::Static(definition) = effective.ability.definition else {
                    return;
                };
                if !definition
                    .source_zones
                    .contains(&crate::card::ZoneKind::Battlefield)
                {
                    return;
                }
                let Some(EffectDef::ModifyCost(CostModificationDef::AbilityAlternative {
                    abilities,
                    permanent: predicate,
                    condition,
                    first_each_turn,
                    costs,
                })) = effective.ability.declarative_effect()
                else {
                    return;
                };
                if !Self::activation_kind_matches(abilities, false, Some(activated))
                    || !self.trigger_object_matches_for_controller(
                        predicate,
                        &self.trigger_event_object(permanent),
                        provider.card.id,
                        false,
                        Some(provider.controller),
                    )
                    || (first_each_turn
                        && self
                            .activated_ability_kinds_this_turn
                            .contains(&(player, abilities)))
                    || condition.is_some_and(|condition| {
                        !self.trigger_condition_holds(
                            condition,
                            provider.card.id,
                            provider.controller,
                            TriggerContext::empty(),
                            Some(effective.origin),
                            None,
                        )
                    })
                {
                    return;
                }
                offered.push((
                    AlternativeAbilityCost {
                        source: provider.card.id,
                        ability: effective.origin,
                    },
                    costs,
                ));
            });
        }
        offered
    }

    pub(in crate::game) fn activation_with_alternative_cost(
        &self,
        player: PlayerId,
        source: GameObjectId,
        mut ability: AbilityDef,
        alternative: Option<AlternativeAbilityCost>,
    ) -> Option<AbilityDef> {
        let Some(alternative) = alternative else {
            return Some(ability);
        };
        let DeclarativeAbilityDef::Activated(mut activated) = ability.definition else {
            return None;
        };
        let permanent = self.battlefield.iter().find(|p| p.card.id == source)?;
        let (_, costs) = self
            .alternative_activation_costs(player, permanent, &activated)
            .into_iter()
            .find(|(choice, _)| *choice == alternative)?;
        activated.costs = costs;
        ability.definition = DeclarativeAbilityDef::Activated(activated);
        Some(ability)
    }
}
