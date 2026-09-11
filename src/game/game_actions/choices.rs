//! Linear alternatives and object selections over the shared action planner.
use super::super::{
    DecisionContinuation, DecisionObservation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, EffectResolutionContext, Game, PendingProcedure, PlayerId,
    ResolvedEffectPayment, ScopedEffect, SettledEffectPayment, StackObject,
};
use crate::card::EffectDef;

impl Game {
    pub(in crate::game) fn action_choice_options(
        &self,
        player: PlayerId,
        choices: &[ResolvedEffectPayment],
        branch: Option<usize>,
        definition: ScopedEffect,
    ) -> Option<(&'static str, usize, DecisionVisibility, Vec<DecisionOption>)> {
        let (optional, visibility) = match definition.effect {
            EffectDef::PayOr(payment) => (
                true,
                super::super::decision_offers::effect_choice_visibility(payment.visibility),
            ),
            _ => (false, DecisionVisibility::Public),
        };
        let mut options = Vec::new();
        if let Some(branch) = branch {
            let ResolvedEffectPayment::Action(payment) = choices.get(branch)? else {
                return None;
            };
            for target in self.action_payment_candidates(player, payment) {
                options.push(self.effect_target_option(options.len(), target));
            }
            let count = usize::from(payment.amount);
            return (options.len() >= count).then_some((
                "Select action objects",
                count,
                visibility,
                options,
            ));
        }
        if optional {
            options.push(DecisionOption {
                id: 0,
                label: "Decline".into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            });
        }
        for (index, choice) in choices.iter().enumerate() {
            let ResolvedEffectPayment::Action(payment) = choice else {
                return None;
            };
            if self.action_payment_candidates(player, payment).len() >= usize::from(payment.amount)
            {
                options.push(DecisionOption {
                    id: u32::try_from(index + 1).ok()?,
                    label: format!("{} {} matching object(s)", payment.verb(), payment.amount),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                });
            }
        }
        Some(("Choose an action", 1, visibility, options))
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::game) fn queue_action_choice(
        &mut self,
        player: PlayerId,
        choices: Vec<ResolvedEffectPayment>,
        branch: Option<usize>,
        definition: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) {
        let Some((prompt, count, visibility, options)) =
            self.action_choice_options(player, &choices, branch, definition)
        else {
            return;
        };
        if options.is_empty() {
            return;
        }
        self.queue_decision(
            player,
            prompt,
            visibility,
            DecisionPreference::Neutral,
            count..=count,
            false,
            options,
            DecisionContinuation::ActionChoice {
                player,
                choices,
                branch,
                definition,
                object: Box::new(object.clone()),
                context,
            },
        );
        self.associate_latest_decision_with(object);
    }

    pub(in crate::game) fn resolve_action_choice(
        &mut self,
        continuation: DecisionContinuation,
        observation: &DecisionObservation,
        selected: &[u32],
    ) {
        let DecisionContinuation::ActionChoice {
            player,
            choices,
            branch,
            definition,
            object,
            context,
        } = continuation
        else {
            unreachable!("action choice continuation")
        };
        let provenance = match definition.effect {
            EffectDef::PayOr(payment) => payment
                .label
                .map(|label| super::super::PaymentProvenance { label }),
            _ => None,
        };
        let Some(branch) = branch else {
            if let [choice] = selected
                && *choice > 0
            {
                self.queue_action_choice(
                    player,
                    choices,
                    Some((*choice - 1) as usize),
                    definition,
                    &object,
                    context,
                );
            } else if matches!(definition.effect, EffectDef::PayOr(_)) {
                self.complete_effect_payment(
                    player, provenance, None, definition, &object, context,
                );
            }
            return;
        };
        let ResolvedEffectPayment::Action(payment) = &choices[branch] else {
            unreachable!("validated action alternative")
        };
        let cards = observation
            .options
            .iter()
            .filter(|option| selected.contains(&option.id))
            .filter_map(|option| option.card.map(|(id, _)| id))
            .collect::<Vec<_>>();
        let mut later = std::mem::take(&mut self.pending_procedures);
        let paid = self
            .settle_action_payment(player, payment, &cards)
            .then_some(SettledEffectPayment::without_mana(0));
        if matches!(definition.effect, EffectDef::PayOr(_)) {
            self.pending_procedures
                .push_back(PendingProcedure::CompletePayment {
                    player,
                    provenance,
                    paid,
                    definition,
                    object,
                    context,
                });
        }
        self.pending_procedures.append(&mut later);
    }
}
