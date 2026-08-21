//! Moving the cards a "choose cards you own" decision named.
//!
//! Split out of the parent module for the source-size budget. What belongs
//! here is one procedure: the chosen cards leave the zones they were offered
//! from, in the order they were named, and land wherever the clause said.

#![allow(clippy::wildcard_imports)]

use super::*;
use crate::game::decision_state::SearchFollowUp;

impl Game {
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
        // Read back off the effect the choice came from: what a
        // permanent arrives carrying is part of that printed clause,
        // not a second thing the continuation has to remember.
        let arrival_effect = arrival
            .as_ref()
            .and_then(|arrival| match arrival.effect.effect {
                crate::card::EffectDef::ChooseCards { arrival_effect, .. } => arrival_effect,
                _ => None,
            });
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
            let Some((moved, actual_destination)) = self.move_card_from_nonbattlefield_zone(
                id,
                source,
                destination,
                ZoneMoveCause::Effect { controller },
                (destination == ZoneKind::Battlefield).then(|| BattlefieldArrival::under(player)),
            ) else {
                continue;
            };
            if actual_destination == ZoneKind::Library
                && placement == ZonePlacement::Bottom
                && let Some(card) =
                    remove_card(&mut self.players[moved.owner.index()].library, moved.id)
            {
                self.players[moved.owner.index()].library.insert(0, card);
            }
            // What arrived is a new object: the card that left the
            // hand and the permanent now standing there are two
            // identities, and only the second one can carry
            // anything.
            if actual_destination == ZoneKind::Battlefield
                && let (Some(effect), Some(arrival), Some(arrived)) =
                    (arrival_effect, arrival.as_ref(), self.arrived)
            {
                self.apply_arrival_effect(
                    arrived,
                    *effect,
                    &arrival.object,
                    &arrival.context,
                    arrival.effect,
                );
            }
        }
    }
}
