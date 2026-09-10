//! A linear selection window for named, fixed object-cost alternatives.
//! All grammar and mechanic identity come from the declaration, not a card name.
use super::super::{
    BattlefieldExitCompletion, CommittedTriggerEvent, DecisionContinuation, DecisionObservation,
    DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone, EffectResolutionContext,
    Game, PendingProcedure, ScopedEffect, SettledEffectPayment, StackObject, ZoneKind,
};
use crate::card::{CostDef, MechanicId};
use crate::{GameObjectId, PlayerId, Target};

impl Game {
    pub(in crate::game) fn capture_mechanic(&mut self, mechanic: MechanicId, player: PlayerId) {
        self.capture_battlefield_triggers(&CommittedTriggerEvent::MechanicPerformed {
            mechanic,
            player,
        });
    }

    pub(in crate::game) fn named_cost_candidates(
        &self,
        player: PlayerId,
        cost: CostDef,
        source: GameObjectId,
    ) -> Vec<GameObjectId> {
        let Some((predicate, zone, _)) = cost.named_object_selection() else {
            return Vec::new();
        };
        match zone {
            ZoneKind::Graveyard => self
                .matching_graveyard_cards(player, predicate, source)
                .into_iter()
                .map(|(id, _)| id)
                .collect(),
            ZoneKind::Battlefield => self
                .battlefield
                .iter()
                .filter(|permanent| permanent.controller == player)
                .filter(|permanent| {
                    self.trigger_object_matches_for_controller(
                        predicate,
                        &self.trigger_event_object(permanent),
                        source,
                        false,
                        Some(player),
                    )
                })
                .map(|permanent| permanent.card.id)
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Commit one already selected action and report its named completion only
    /// after its replacement work finishes. Return the actual exiled identities.
    pub(in crate::game) fn commit_named_object_cost(
        &mut self,
        player: PlayerId,
        named: CostDef,
        cards: &[GameObjectId],
        source: GameObjectId,
        then: Option<BattlefieldExitCompletion>,
    ) -> Option<Vec<GameObjectId>> {
        let CostDef::Named { mechanic, cost } = named else {
            return None;
        };
        let (_, zone, count) = cost.named_object_selection()?;
        let candidates = self.named_cost_candidates(player, *cost, source);
        if cards.len() != usize::from(count)
            || cards
                .iter()
                .enumerate()
                .any(|(index, id)| cards[..index].contains(id) || !candidates.contains(id))
        {
            return None;
        }
        match zone {
            ZoneKind::Graveyard => {
                let exiled = self.exile_graveyard_cards(player, cards);
                self.capture_mechanic(mechanic, player);
                Some(exiled)
            }
            ZoneKind::Battlefield => {
                let mut completions =
                    vec![BattlefieldExitCompletion::MechanicPerformed { mechanic, player }];
                completions.extend(then);
                self.capture_sacrifices(cards);
                self.move_permanents_to_graveyard_then(
                    cards,
                    Some(BattlefieldExitCompletion::Completions(completions)),
                );
                Some(Vec::new())
            }
            _ => None,
        }
    }

    pub(in crate::game) fn named_cost_options(
        &self,
        player: PlayerId,
        named: CostDef,
        branch: Option<usize>,
        source: GameObjectId,
    ) -> Option<(&'static str, usize, Vec<DecisionOption>)> {
        let branches = named.named_choices()?;
        let mut options = Vec::new();
        let Some(branch) = branch else {
            options.push(DecisionOption {
                id: 0,
                label: "Decline".into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            });
            for (index, cost) in branches.iter().enumerate() {
                let (_, zone, count) = cost.named_object_selection()?;
                if self.named_cost_candidates(player, *cost, source).len() >= usize::from(count) {
                    let verb = if zone == ZoneKind::Graveyard {
                        "Exile"
                    } else {
                        "Sacrifice"
                    };
                    options.push(DecisionOption {
                        id: u32::try_from(index + 1).ok()?,
                        label: format!("{verb} {count} matching object(s)"),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    });
                }
            }
            return Some(("Choose how to pay", 1, options));
        };
        let cost = *branches.get(branch)?;
        let (_, zone, count) = cost.named_object_selection()?;
        for id in self.named_cost_candidates(player, cost, source) {
            let target = if zone == ZoneKind::Battlefield {
                Target::Permanent(id)
            } else {
                Target::Card(id)
            };
            options.push(self.effect_target_option(options.len(), target));
        }
        (options.len() >= usize::from(count)).then_some((
            "Select payment objects",
            usize::from(count),
            options,
        ))
    }

    pub(in crate::game) fn queue_named_cost(
        &mut self,
        player: PlayerId,
        cost: CostDef,
        branch: Option<usize>,
        definition: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) {
        let source = object.source.unwrap_or(object.id);
        let Some((prompt, count, options)) = self.named_cost_options(player, cost, branch, source)
        else {
            self.complete_effect_payment(player, None, None, definition, object, context);
            return;
        };
        self.queue_decision(
            player,
            prompt,
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            count..=count,
            false,
            options,
            DecisionContinuation::NamedCost {
                player,
                cost,
                branch,
                definition,
                object: Box::new(object.clone()),
                context,
            },
        );
        if let Some(pending) = self.pending_decisions.last_mut() {
            pending.observation.source = object.source;
        }
    }

    pub(in crate::game) fn resolve_named_cost_choice(
        &mut self,
        continuation: DecisionContinuation,
        observation: &DecisionObservation,
        choices: &[u32],
    ) {
        let DecisionContinuation::NamedCost {
            player,
            cost,
            branch,
            definition,
            object,
            context,
        } = continuation
        else {
            unreachable!("named object cost decision")
        };
        if branch.is_none() {
            if let [choice] = choices
                && *choice > 0
            {
                self.queue_named_cost(
                    player,
                    cost,
                    Some((*choice - 1) as usize),
                    definition,
                    &object,
                    context,
                );
            } else {
                self.complete_effect_payment(player, None, None, definition, &object, context);
            }
            return;
        }
        let CostDef::Named { mechanic, .. } = cost else {
            unreachable!()
        };
        let selected = &cost.named_choices().expect("validated named cost")[branch.unwrap()];
        let cards = observation
            .options
            .iter()
            .filter(|option| choices.contains(&option.id))
            .filter_map(|option| option.card.map(|(id, _)| id))
            .collect::<Vec<_>>();
        let mut later = std::mem::take(&mut self.pending_procedures);
        let paid = self
            .commit_named_object_cost(
                player,
                CostDef::Named {
                    mechanic,
                    cost: selected,
                },
                &cards,
                object.source.unwrap_or(object.id),
                None,
            )
            .map(|_| SettledEffectPayment::without_mana(0));
        self.pending_procedures
            .push_back(PendingProcedure::CompletePayment {
                player,
                provenance: None,
                paid,
                definition,
                object,
                context,
            });
        self.pending_procedures.append(&mut later);
    }
}
