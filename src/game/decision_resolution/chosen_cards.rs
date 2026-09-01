//! Moving the cards a "choose cards you own" decision named.
//!
//! Split out of the parent module for the source-size budget. What belongs
//! here is one procedure: the chosen cards leave the zones they were offered
//! from, in the order they were named, and land wherever the clause said.

#![allow(clippy::wildcard_imports)]

use super::*;
use crate::game::PendingProcedure;
use crate::game::decision_state::SearchFollowUp;

impl Game {
    fn finish_chosen_card_move_result(
        &mut self,
        follow_up: &SearchFollowUp,
        binding: crate::ObjectSetBindingIndex,
        then: &'static crate::card::EffectDef,
        moved: Vec<Target>,
    ) {
        let mut context = follow_up.context.clone();
        context.bind_object_group(binding, moved);
        let effect = follow_up.effect.with_effect(*then);
        if !self.pending_decisions.is_empty()
            || !self.pending_events.is_empty()
            || !self.pending_procedures.is_empty()
        {
            self.pending_procedures
                .push_back(PendingProcedure::ResolveEffects {
                    effects: vec![effect],
                    object: Box::new(follow_up.object.clone()),
                    context,
                });
        } else {
            self.resolve_effect_def(effect, &follow_up.object, context);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn move_chosen_cards(
        &mut self,
        player: PlayerId,
        controller: PlayerId,
        destination: ZoneKind,
        placement: ZonePlacement,
        reveal: bool,
        arrival: Option<&SearchFollowUp>,
        offered: &[DecisionOption],
        options: &[u32],
    ) {
        let move_result = arrival.and_then(|follow_up| match follow_up.effect.effect {
            crate::card::EffectDef::WithZoneMoveResult { binding, then, .. } => {
                Some((follow_up, binding, then))
            }
            _ => None,
        });
        let mut later_procedures = move_result
            .is_some()
            .then(|| std::mem::take(&mut self.pending_procedures));
        let selected = options
            .iter()
            .filter_map(|selected| offered.iter().find(|option| option.id == *selected))
            .cloned()
            .collect::<Vec<_>>();
        if reveal {
            self.events.extend(selected.iter().filter_map(|option| {
                option.card.and_then(|(card, characteristics)| {
                    characteristics
                        .card_definition()
                        .map(|definition| GameEvent::CardRevealed {
                            player,
                            card,
                            definition,
                        })
                })
            }));
        }
        let mut moved = Vec::new();
        for option in selected {
            let Some((id, _)) = option.card else {
                continue;
            };
            if option.zone == crate::game::DecisionZone::OutsideGame {
                // Outside-game imports currently have one shared
                // supported destination: the hand. Reject any broader
                // authored shape without consuming the physical card.
                if destination != ZoneKind::Hand {
                    continue;
                }
                let Some(card) = remove_card(&mut self.players[player.index()].outside_game, id)
                else {
                    continue;
                };
                let owner = card.owner;
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[owner.index()].hand.push(card);
                moved.push(Target::Card(id));
                continue;
            }
            let source = match option.zone {
                crate::game::DecisionZone::Library => ZoneKind::Library,
                crate::game::DecisionZone::Hand => ZoneKind::Hand,
                crate::game::DecisionZone::Graveyard => ZoneKind::Graveyard,
                crate::game::DecisionZone::Exile => ZoneKind::Exile,
                crate::game::DecisionZone::Battlefield
                | crate::game::DecisionZone::Stack
                | crate::game::DecisionZone::Command
                | crate::game::DecisionZone::OutsideGame
                | crate::game::DecisionZone::DrawnThisStep
                | crate::game::DecisionZone::None => continue,
            };
            let Some((moved_card, actual_destination)) = self.move_card_from_nonbattlefield_zone(
                id,
                source,
                destination,
                ZoneMoveCause::Effect { controller },
                (destination == ZoneKind::Battlefield).then(|| BattlefieldArrival::under(player)),
            ) else {
                continue;
            };
            moved.push(Target::Card(id));
            if actual_destination == ZoneKind::Library
                && placement == ZonePlacement::Bottom
                && let Some(card) = remove_card(
                    &mut self.players[moved_card.owner.index()].library,
                    moved_card.id,
                )
            {
                self.players[moved_card.owner.index()]
                    .library
                    .insert(0, card);
            }
        }
        if let Some((follow_up, binding, then)) = move_result {
            self.finish_chosen_card_move_result(follow_up, binding, then, moved);
        }
        if let Some(later) = later_procedures.as_mut() {
            self.pending_procedures.append(later);
        }
    }
}
