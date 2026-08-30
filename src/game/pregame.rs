//! Card actions that happen before the first turn begins.

use std::collections::BTreeSet;

use super::{
    AbilityCostDef, AbilityOrigin, CharacteristicContext, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DeclarativeAbilityDef, EffectDef,
    EffectResolutionContext, Game, GameEvent, GameObjectId, ObjectCharacteristics, PlayerId,
    Pregame, PregameAbilityAction, PregameConditionDef, PregameTimingDef, ScopedEffect,
    StackAbilityPayload, StackAbilityResolver, StackObject, StackObjectKind, TargetSelection,
    TriggerContext, ZoneKind, remove_card,
};

impl Game {
    pub(super) fn mulligan_ability_actions(&self, player: PlayerId) -> Vec<crate::Action> {
        self.pregame_ability_actions(player, PregameTimingDef::Mulligan)
            .into_iter()
            .map(|action| action.action())
            .collect()
    }

    pub(super) fn pregame_ability_actions(
        &self,
        player: PlayerId,
        timing: PregameTimingDef,
    ) -> Vec<PregameAbilityAction> {
        let mut actions = Vec::new();
        for card in &self.players[player.index()].hand {
            self.for_each_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Pregame(definition) = ability.definition else {
                    return;
                };
                if !ability.is_executable()
                    || ability.declarative_effect().is_none()
                    || definition.timing != timing
                    || (definition.condition == PregameConditionDef::NotStartingPlayer
                        && player == PlayerId::One)
                {
                    return;
                }
                let mut object_cost = None;
                for cost in definition.costs {
                    match cost {
                        AbilityCostDef::ExileCardFromHand(predicate) if object_cost.is_none() => {
                            object_cost = Some(*predicate);
                        }
                        _ => return,
                    }
                }
                match object_cost {
                    None => actions.push(PregameAbilityAction {
                        source: card.id,
                        ability: effective.origin,
                        cost_objects: Vec::new(),
                    }),
                    Some(predicate) => {
                        actions.extend(
                            self.players[player.index()]
                                .hand
                                .iter()
                                .filter(|candidate| candidate.id != card.id)
                                .filter(|candidate| {
                                    self.card_object_matches(
                                        predicate,
                                        candidate,
                                        ZoneKind::Hand,
                                        card.id,
                                    )
                                })
                                .map(|candidate| PregameAbilityAction {
                                    source: card.id,
                                    ability: effective.origin,
                                    cost_objects: vec![candidate.id],
                                }),
                        );
                    }
                }
            });
        }
        actions
    }

    pub(super) fn begin_opening_hand_actions(&mut self, player: PlayerId) {
        let actions = self.pregame_ability_actions(player, PregameTimingDef::OpeningHand);
        if actions.is_empty() {
            if player == PlayerId::One {
                self.begin_opening_hand_actions(PlayerId::Two);
            } else {
                self.pregame = None;
                self.priority = PlayerId::One;
            }
            return;
        }
        self.pregame = Some(Pregame::OpeningHand(player));
        self.priority = player;
        self.queue_opening_hand_finish(player, actions);
    }

    fn queue_opening_hand_finish(&mut self, player: PlayerId, actions: Vec<PregameAbilityAction>) {
        self.queue_decision(
            player,
            "Take opening-hand actions, or continue",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            0..=0,
            false,
            Vec::<DecisionOption>::new(),
            DecisionContinuation::PregameActions { player, actions },
        );
    }

    pub(super) fn is_pregame_ability_action(
        &self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        cost_objects: &[GameObjectId],
    ) -> bool {
        let wanted = PregameAbilityAction {
            source,
            ability,
            cost_objects: cost_objects.to_vec(),
        };
        if matches!(self.pregame, Some(Pregame::Mulligan(chooser)) if chooser == player) {
            return self
                .pregame_ability_actions(player, PregameTimingDef::Mulligan)
                .contains(&wanted);
        }
        self.pending_decisions
            .first()
            .and_then(|decision| decision.continuation.pregame_actions(player))
            .is_some_and(|actions| actions.contains(&wanted))
    }

    pub(super) fn activate_pregame_ability(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        origin: AbilityOrigin,
        cost_objects: &[GameObjectId],
    ) {
        let remaining_opening_sources = self.take_remaining_opening_sources(source);

        let source_card = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == source)
            .cloned()
            .expect("a legal pregame ability has its source in hand");
        let effective = self
            .find_printed_card_ability(&source_card, &CharacteristicContext::Hand, |effective| {
                effective.origin == origin
            })
            .expect("a legal pregame action names its printed ability");
        let DeclarativeAbilityDef::Pregame(definition) = effective.ability.definition else {
            unreachable!("a legal pregame action names a pregame ability")
        };

        for chosen in cost_objects {
            if let Some(card) = remove_card(&mut self.players[player.index()].hand, *chosen) {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[player.index()].exile.push(card.clone());
                self.capture_cards_exiled(std::slice::from_ref(&card), ZoneKind::Hand);
            }
        }

        if definition.reveals_source {
            self.events.push(GameEvent::CardRevealed {
                player,
                card: source_card.id,
                definition: source_card.definition,
            });
        }

        let presentation = Self::ability_presentation(
            origin,
            ObjectCharacteristics::card(source_card.definition, crate::CardPartId::PRIMARY),
        );
        let card = self.unbacked_ability_object(presentation, player);
        let object = StackObject {
            id: card.id,
            kind: StackObjectKind::TriggeredAbility,
            card,
            source: Some(source),
            ability: Some(StackAbilityPayload {
                origin,
                definition: Some(Box::new(effective.ability)),
                presentation,
                text: Some(effective.ability.text),
                target_defs: Vec::new(),
                targets: Vec::<TargetSelection>::new(),
                context: EffectResolutionContext::new(TriggerContext::empty()),
                resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(
                    effective
                        .ability
                        .declarative_effect()
                        .unwrap_or(EffectDef::None),
                )),
                condition: None,
                mode_effects: Vec::new(),
                resolution_destination: None,
                x: 0,
                sacrificed_mana_value: 0,
            }),
            controller: player,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast: None,
            face_down: None,
            is_copy: false,
        };
        self.resolve_effect_def(
            ScopedEffect::primary(
                effective
                    .ability
                    .declarative_effect()
                    .unwrap_or(EffectDef::None),
            ),
            &object,
            EffectResolutionContext::new(TriggerContext::empty()),
        );

        if let Some(remaining_sources) = remaining_opening_sources {
            // An earlier action can consume another action's source or alter
            // the available cost cards. Re-enumerate from the actual hand,
            // then retain only sources that have not already acted.
            let actions = self
                .pregame_ability_actions(player, PregameTimingDef::OpeningHand)
                .into_iter()
                .filter(|action| remaining_sources.contains(&action.source))
                .collect();
            self.queue_opening_hand_finish(player, actions);
        }
    }

    fn take_remaining_opening_sources(
        &mut self,
        source: GameObjectId,
    ) -> Option<BTreeSet<GameObjectId>> {
        if !matches!(self.pregame, Some(Pregame::OpeningHand(_))) {
            return None;
        }
        let pending = self.pending_decisions.remove(0);
        let DecisionContinuation::PregameActions { actions, .. } = pending.continuation else {
            unreachable!("a legal opening-hand action has its finish decision")
        };
        Some(
            actions
                .into_iter()
                .filter(|action| action.source != source)
                .map(|action| action.source)
                .collect(),
        )
    }

    pub(super) fn finish_opening_hand_actions(&mut self, player: PlayerId) {
        if player == PlayerId::One {
            self.begin_opening_hand_actions(PlayerId::Two);
        } else {
            self.pregame = None;
            self.priority = PlayerId::One;
        }
    }
}
