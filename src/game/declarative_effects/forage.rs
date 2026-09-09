//! The forage action shared by resolving effects and spell costs.

use crate::card::ObjectPredicateDef;
use crate::{GameObjectId, PlayerId, Target};

use super::super::{
    BattlefieldExitCompletion, CommittedTriggerEvent, DecisionContinuation, DecisionObservation,
    DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone, Game, ZoneKind,
};

impl Game {
    pub(in crate::game) fn capture_forage(&mut self, player: PlayerId) {
        self.capture_battlefield_triggers(&CommittedTriggerEvent::Foraged { player });
    }

    pub(in crate::game) fn exile_to_forage(
        &mut self,
        player: PlayerId,
        cards: &[GameObjectId],
    ) -> Option<Vec<GameObjectId>> {
        if cards.len() != 3
            || cards.iter().enumerate().any(|(index, id)| {
                cards[..index].contains(id)
                    || !self.players[player.index()]
                        .graveyard
                        .iter()
                        .any(|card| card.id == *id)
            })
        {
            return None;
        }
        let exiled = self.exile_graveyard_cards(player, cards);
        self.capture_forage(player);
        Some(exiled)
    }

    pub(in crate::game) fn sacrifice_food_to_forage(
        &mut self,
        player: PlayerId,
        food: GameObjectId,
        then: Option<BattlefieldExitCompletion>,
    ) -> bool {
        if !self
            .matching_permanents_controlled(player, ObjectPredicateDef::Subtype("Food"))
            .contains(&food)
        {
            return false;
        }
        let mut completions = vec![BattlefieldExitCompletion::Foraged { player }];
        completions.extend(then);
        self.capture_sacrifices(&[food]);
        self.move_permanents_to_graveyard_then(
            &[food],
            Some(BattlefieldExitCompletion::Completions(completions)),
        );
        true
    }

    /// One option per candidate, never one per three-card combination.
    pub(in crate::game) fn forage_options(
        &self,
        player: PlayerId,
        optional: bool,
        from: Option<ZoneKind>,
    ) -> Option<(&'static str, usize, Vec<DecisionOption>)> {
        let mut options = Vec::new();
        let (prompt, count) = match from {
            None => {
                for (id, label, available) in [
                    (0, "Decline to forage", optional),
                    (
                        1,
                        "Exile three cards from your graveyard",
                        self.players[player.index()].graveyard.len() >= 3,
                    ),
                    (
                        2,
                        "Sacrifice a Food",
                        !self
                            .matching_permanents_controlled(
                                player,
                                ObjectPredicateDef::Subtype("Food"),
                            )
                            .is_empty(),
                    ),
                ] {
                    if available {
                        options.push(DecisionOption {
                            id,
                            label: label.into(),
                            card: None,
                            members: Vec::new(),
                            ability_text: None,
                            zone: DecisionZone::None,
                        });
                    }
                }
                ("Forage", 1)
            }
            Some(ZoneKind::Graveyard) => {
                for card in &self.players[player.index()].graveyard {
                    let mut option =
                        self.effect_target_option(options.len(), Target::Card(card.id));
                    option.label = format!("Exile {}", option.label);
                    options.push(option);
                }
                ("Exile three cards to forage", 3)
            }
            Some(ZoneKind::Battlefield) => {
                for food in
                    self.matching_permanents_controlled(player, ObjectPredicateDef::Subtype("Food"))
                {
                    let mut option =
                        self.effect_target_option(options.len(), Target::Permanent(food));
                    option.label = format!("Sacrifice {}", option.label);
                    options.push(option);
                }
                ("Sacrifice a Food to forage", 1)
            }
            _ => return None,
        };
        (options.len() >= count).then_some((prompt, count, options))
    }

    pub(in crate::game) fn queue_forage(
        &mut self,
        player: PlayerId,
        optional: bool,
        from: Option<ZoneKind>,
        source: Option<GameObjectId>,
    ) {
        let Some((prompt, count, options)) = self.forage_options(player, optional, from) else {
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
            DecisionContinuation::Forage {
                player,
                optional,
                from,
            },
        );
        if let Some(pending) = self.pending_decisions.last_mut() {
            pending.observation.source = source;
        }
    }

    pub(in crate::game) fn resolve_forage_choice(
        &mut self,
        player: PlayerId,
        from: Option<ZoneKind>,
        observation: &DecisionObservation,
        choices: &[u32],
    ) {
        if from.is_none() {
            let from = match choices {
                [1] => ZoneKind::Graveyard,
                [2] => ZoneKind::Battlefield,
                _ => return,
            };
            self.queue_forage(player, false, Some(from), observation.source);
            return;
        }
        let cards = observation
            .options
            .iter()
            .filter(|option| choices.contains(&option.id))
            .filter_map(|option| option.card.map(|(id, _)| id))
            .collect::<Vec<_>>();
        match from {
            Some(ZoneKind::Graveyard) => {
                let _ = self.exile_to_forage(player, &cards);
            }
            Some(ZoneKind::Battlefield) => {
                if let [food] = cards.as_slice() {
                    self.sacrifice_food_to_forage(player, *food, None);
                }
            }
            _ => {}
        }
    }
}
