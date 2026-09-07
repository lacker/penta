//! An activation's object selections precede every cost mutation. Small
//! selections can ride the announced action; larger ones use a bounded offer.

use crate::card::{CostDef, DeclarativeAbilityDef, ZoneKind};
use crate::game::{
    Action, ActivationChoices, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, Game, GameObjectId, PlayerId,
};

impl Game {
    pub(in crate::game) fn activation_object_payment_is_valid(
        &self,
        player: PlayerId,
        action: &Action,
        cost: CostDef,
        selected: &[u32],
        options: &[DecisionOption],
    ) -> bool {
        let members = selected
            .iter()
            .filter_map(|id| {
                options
                    .iter()
                    .find(|option| option.id == *id)
                    .and_then(|option| option.card.map(|(id, _)| id))
            })
            .collect::<Vec<_>>();
        let Some((_, _, quantity)) = cost.object_selection() else {
            return false;
        };
        if members.len() != selected.len()
            || !self.object_selection_is_valid(
                &self.activation_payment_candidates(player, action, cost),
                &members,
                quantity,
            )
        {
            return false;
        }
        let mut announced = action.clone();
        let Action::ActivateAbility { cost_objects, .. } = &mut announced else {
            return false;
        };
        *cost_objects = members;
        self.activation_selected_mana_is_payable(player, &announced)
    }

    pub(in crate::game) fn activation_selected_mana_is_payable(
        &self,
        player: PlayerId,
        action: &Action,
    ) -> bool {
        let Action::ActivateAbility { cost_objects, .. } = action else {
            return false;
        };
        self.mana_requirement(player, action)
            .is_none_or(|(mana, x, options, purpose)| {
                self.plan_mana_activations_for_reserving(
                    player,
                    mana,
                    x,
                    options.avoid,
                    &purpose,
                    cost_objects,
                )
                .is_some()
            })
    }

    pub(in crate::game) fn activation_object_cost_choices(
        &self,
        player: PlayerId,
        source: GameObjectId,
        cost: CostDef,
        fixed: &[GameObjectId],
        leaves_source: bool,
    ) -> Vec<Vec<GameObjectId>> {
        let candidates = self
            .object_cost_candidates(player, source, cost)
            .into_iter()
            .filter(|id| !fixed.contains(id) && (!leaves_source || *id != source))
            .collect::<Vec<_>>();
        let Some((_, _, quantity)) = cost.object_selection() else {
            return Vec::new();
        };
        match quantity.fixed_value() {
            Some(1) => candidates.into_iter().map(|id| vec![id]).collect(),
            Some(count) if count > 1 && self.object_selection_is_payable(&candidates, quantity) => {
                vec![Vec::new()]
            }
            _ => Vec::new(),
        }
    }

    pub(in crate::game) fn activation_payment_candidates(
        &self,
        player: PlayerId,
        action: &Action,
        cost: CostDef,
    ) -> Vec<GameObjectId> {
        let Action::ActivateAbility {
            source, ability, ..
        } = action
        else {
            return Vec::new();
        };
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == *source)
        else {
            return Vec::new();
        };
        let Some(effective) =
            self.find_effective_ability(permanent, |effective| effective.origin == *ability)
        else {
            return Vec::new();
        };
        let DeclarativeAbilityDef::Activated(definition) = effective.ability.definition else {
            return Vec::new();
        };
        let fixed = definition
            .costs
            .iter()
            .filter_map(|cost| match cost {
                CostDef::SacrificeSource | CostDef::ExileSource | CostDef::ReturnSourceToHand => {
                    Some(*source)
                }
                CostDef::SacrificeObject(reference) => {
                    Self::activation_object_reference(*reference, *source, *ability)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        self.object_cost_candidates(player, *source, cost)
            .into_iter()
            .filter(|id| !fixed.contains(id))
            .collect()
    }

    /// Called only after targets and modes have been announced, and before
    /// mana activation, life payment, tapping, or any other cost action.
    pub(in crate::game) fn queue_activation_object_payment(
        &mut self,
        player: PlayerId,
        action: Action,
        cost: CostDef,
    ) {
        let Some((_, zone, quantity)) = cost.object_selection() else {
            return;
        };
        let Some(count) = quantity.fixed_value() else {
            return;
        };
        let candidates = self.activation_payment_candidates(player, &action, cost);
        if !self.object_selection_is_payable(&candidates, quantity) {
            return;
        }
        let Action::ActivateAbility { source, .. } = action else {
            return;
        };
        let options = self.object_cost_options(&candidates, zone);
        self.queue_decision(
            player,
            "Select objects to pay the activation cost",
            if zone == ZoneKind::Hand {
                DecisionVisibility::Private
            } else {
                DecisionVisibility::Public
            },
            DecisionPreference::Neutral,
            usize::from(count)..=usize::from(count),
            true,
            options,
            DecisionContinuation::ActivationObjectCost {
                player,
                cost,
                action: Box::new(action),
            },
        );
        if let Some(pending) = self.pending_decisions.last_mut() {
            pending.observation.source = Some(source);
        }
    }

    pub(in crate::game) fn resolve_activation_object_payment(
        &mut self,
        player: PlayerId,
        mut action: Action,
        cost: CostDef,
        selected: &[u32],
        options: &[DecisionOption],
    ) {
        let members = selected
            .iter()
            .filter_map(|id| {
                options
                    .iter()
                    .find(|option| option.id == *id)
                    .and_then(|option| option.card.map(|(id, _)| id))
            })
            .collect::<Vec<_>>();
        let Some((_, _, quantity)) = cost.object_selection() else {
            return;
        };
        if members.len() != selected.len()
            || !self.object_selection_is_valid(
                &self.activation_payment_candidates(player, &action, cost),
                &members,
                quantity,
            )
        {
            return;
        }
        // Recheck the announced action too: selecting valid objects cannot
        // make an unaffordable mana/life component legal.
        let mut actions = Vec::new();
        self.add_ability_actions(player, &mut actions);
        if !actions.contains(&action) {
            return;
        }
        let Action::ActivateAbility {
            source,
            ability,
            targets,
            cost_objects,
            x,
            modes,
            mana_payment,
        } = &mut action
        else {
            return;
        };
        *cost_objects = members;
        self.activate_ability(
            player,
            *source,
            *ability,
            ActivationChoices {
                targets: targets.clone(),
                cost_objects,
                x: *x,
                modes,
                mana_payment: mana_payment.as_deref(),
            },
        );
    }
}
