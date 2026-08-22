//! Drawing cards: the ordinary one, the replacements that can interrupt it,
//! the miracle reveal that rides on the turn's first, and the simultaneous
//! opening draw. Split from the turn structure that schedules them because
//! a draw is one instruction whose interruptions are its own.

use super::super::{
    AbilityDef, AbilityOrigin, AbilitySourceRef, AlternativeCastKindDef, CardDefinitionId,
    CardPartId, CommittedTriggerEvent, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, DeclarativeAbilityDef, EffectDef, Game, GameEvent,
    GameObjectId, GameResult, ObjectCharacteristics, PendingProcedure, Permanent, PlayerId,
    ReplacementConditionDef, ReplacementEffectDef, ReplacementEventDef, ScopedEffect,
    StackAbilityPayload, StackAbilityResolver, StackObject, StackObjectKind, Step, TriggerCapture,
    TriggerContext, WinReason, ZoneKind,
};

impl Game {
    pub(in crate::game) fn draw_card(&mut self, player: PlayerId) -> Option<GameObjectId> {
        let mut replacements = self.draw_replacements[player.index()]
            .drain(..)
            .collect::<Vec<_>>();
        replacements.extend(self.applicable_static_draw_replacements(player));
        match replacements.as_slice() {
            [] => self.commit_draw_card(player),
            [replacement] if !replacement.optional => {
                let replacement = replacement.clone();
                self.resolve_effect_def(
                    replacement.effect,
                    &replacement.object,
                    replacement.context,
                );
                None
            }
            _ => {
                self.queue_draw_replacement_choice(player, replacements);
                None
            }
        }
    }

    pub(in crate::game) fn commit_draw_card(&mut self, player: PlayerId) -> Option<GameObjectId> {
        let Some(card) = self.players[player.index()].library.pop() else {
            // Jace, Wielder of Mysteries turns the loss into a win. The draw
            // is replaced, so the flag that would end the game is never set.
            if self.player_wins_on_empty_library_draw(player) {
                self.finish(GameResult::Winner {
                    winner: player,
                    reason: WinReason::OpponentLostToAnEffect,
                });
                return None;
            }
            self.players[player.index()].tried_to_draw_from_empty_library = true;
            return None;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let card_id = card.id;
        self.players[player.index()].hand.push(card);
        self.events.push(GameEvent::CardDrawn {
            player,
            card: card_id,
        });
        let drawn = &mut self.cards_drawn_this_turn[player.index()];
        *drawn = drawn.saturating_add(1);
        self.drawn_this_turn[player.index()].push(card_id);
        if self.cards_drawn_this_turn[player.index()] == 1 {
            self.queue_draw_action_window(player, card_id);
        }
        // Raised where the card actually reaches the hand: a draw that was
        // replaced above never happened, so nothing watching for one fires.
        // Asked before the flag is set, so the draw that claims the
        // exemption is the one that reports having it.
        let first_in_draw_step = self.step == Step::Draw
            && self.active_player == player
            && !self.draw_step_draw_taken[player.index()];
        if first_in_draw_step {
            self.draw_step_draw_taken[player.index()] = true;
        }
        self.capture_battlefield_triggers(&CommittedTriggerEvent::DrewCard {
            player,
            first_in_draw_step,
        });
        Some(card_id)
    }

    fn queue_draw_replacement_choice(
        &mut self,
        player: PlayerId,
        replacements: Vec<super::super::DrawReplacement>,
    ) {
        let may_draw = replacements.iter().all(|replacement| replacement.optional);
        let mut options = may_draw
            .then(|| DecisionOption {
                id: 0,
                label: "Draw the card".into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .into_iter()
            .collect::<Vec<_>>();
        options.extend(replacements.iter().enumerate().map(|(index, replacement)| {
            let name = self
                .presentation_name(replacement.object.presentation())
                .unwrap_or_else(|| "Draw replacement".into());
            DecisionOption {
                id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                label: replacement
                    .object
                    .ability_text()
                    .map_or_else(|| name.to_string(), |text| format!("{name} — {text}")),
                card: None,
                members: Vec::new(),
                ability_text: replacement.object.ability_text().map(str::to_owned),
                zone: DecisionZone::None,
            }
        }));
        self.queue_decision(
            player,
            "Choose which effect replaces this draw",
            DecisionVisibility::Public,
            if may_draw {
                DecisionPreference::PreferOption(0)
            } else {
                DecisionPreference::Neutral
            },
            1..=1,
            false,
            options,
            DecisionContinuation::DrawReplacement {
                player,
                replacements,
            },
        );
    }

    fn applicable_static_draw_replacements(
        &self,
        player: PlayerId,
    ) -> Vec<super::super::DrawReplacement> {
        let mut replacements = Vec::new();
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return;
                };
                let ReplacementEventDef::WouldDraw {
                    player: relation,
                    during_own_draw_step,
                } = definition.event
                else {
                    return;
                };
                let Some(program) = ability.declarative_replacement() else {
                    return;
                };
                let Some(effect) = Self::draw_replacement_performed_effect(program) else {
                    return;
                };
                let condition_matches = match definition.condition {
                    None => true,
                    Some(ReplacementConditionDef::SourceTapped) => permanent.tapped,
                    Some(ReplacementConditionDef::CreatureDiedThisTurn) => {
                        self.creature_died_this_turn
                    }
                    Some(ReplacementConditionDef::SourceCastWith(_)) => false,
                };
                let event_context = TriggerContext {
                    event_player: Some(player),
                    ..TriggerContext::empty()
                };
                if !ability.is_executable()
                    || !definition.source_zones.contains(&ZoneKind::Battlefield)
                    || (during_own_draw_step
                        && (self.step != Step::Draw || self.active_player != player))
                    || !condition_matches
                    || !self.draw_replacement_relation_matches(
                        permanent,
                        player,
                        relation,
                        event_context,
                    )
                {
                    return;
                }
                let presentation = Self::ability_presentation(
                    effective.origin,
                    Self::effective_rules_source(permanent),
                );
                let scoped = ScopedEffect::primary(effect);
                let object = StackObject {
                    id: permanent.card.id,
                    kind: StackObjectKind::TriggeredAbility,
                    card: permanent.card.clone(),
                    source: Some(permanent.card.id),
                    ability: Some(StackAbilityPayload {
                        origin: effective.origin,
                        definition: None,
                        presentation,
                        text: Some(ability.text),
                        target_defs: Vec::new(),
                        targets: Vec::new(),
                        context: event_context.into(),
                        resolver: StackAbilityResolver::Declarative(scoped),
                        condition: None,
                        mode_effects: Vec::new(),
                        resolution_destination: None,
                        x: 0,
                    }),
                    controller: permanent.controller,
                    signature: None,
                    chosen_permanents: Vec::new(),
                    applied_effects: Vec::new(),
                    text_changes: Vec::new(),
                    colors: None,
                    cast_via_flashback: false,
                    cast_at_instant_speed: false,
                    cast_from_zone: None,
                    face_down: None,
                    colors_of_mana_spent: crate::card::ColorSet::empty(),
                    phyrexian_symbols_paid_with_life: 0,
                    is_copy: false,
                };
                replacements.push(super::super::DrawReplacement {
                    object: Box::new(object),
                    context: event_context.into(),
                    effect: scoped,
                    optional: definition.optional,
                    installed: false,
                });
            });
        }
        replacements
    }

    fn draw_replacement_relation_matches(
        &self,
        permanent: &Permanent,
        player: PlayerId,
        relation: crate::card::PlayerRelation,
        context: TriggerContext,
    ) -> bool {
        match relation {
            crate::card::PlayerRelation::ChosenPlayer => permanent.chosen_player == Some(player),
            crate::card::PlayerRelation::ControllerOfAttachedPermanent => self
                .attached_host_controller_of(permanent.card.id)
                .is_some_and(|controller| controller == player),
            _ => self.player_relation_matches(player, relation, permanent.controller, context),
        }
    }

    fn draw_replacement_performed_effect(program: ReplacementEffectDef) -> Option<EffectDef> {
        let ReplacementEffectDef::Sequence(effects) = program else {
            return None;
        };
        let replaces = effects
            .iter()
            .filter(|effect| matches!(effect, ReplacementEffectDef::ReplaceEventWithNothing))
            .count();
        let mut performed = effects.iter().filter_map(|effect| match effect {
            ReplacementEffectDef::Perform(effect) => Some(**effect),
            _ => None,
        });
        let effect = performed.next()?;
        (effects.len() == 2 && replaces == 1 && performed.next().is_none()).then_some(effect)
    }

    /// Whether a card offers a miracle cost at all.
    pub(in crate::game) fn has_miracle(&self, definition: CardDefinitionId) -> bool {
        self.miracle_ability(definition).is_some()
    }

    pub(in crate::game) fn miracle_ability(
        &self,
        definition: CardDefinitionId,
    ) -> Option<(AbilityOrigin, AbilityDef)> {
        let definition = self.catalog.get(definition)?;
        definition.parts.iter().find_map(|part| {
            part.rules.indexed_abilities().find_map(|attached| {
                (attached.definition.is_executable()
                    && matches!(
                        attached.definition.definition,
                        DeclarativeAbilityDef::AlternativeCast(alternative)
                            if alternative.kind == AlternativeCastKindDef::Miracle
                    ))
                .then_some((
                    AbilityOrigin::Printed {
                        definition: definition.id,
                        part: part.id,
                        ability: attached.id,
                    },
                    attached.definition,
                ))
            })
        })
    }

    /// Offers every private action available specifically because this was
    /// the player's first card drawn this turn. The window exists even when
    /// it has no actions, so declining Miracle and drawing an ordinary card
    /// follow the same hidden decision path.
    pub(in crate::game) fn queue_draw_action_window(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
    ) {
        let Some((definition, name)) = self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)
            .and_then(|held| {
                self.catalog
                    .get(held.definition)
                    .map(|definition| (held.definition, definition.name.clone()))
            })
        else {
            return;
        };
        let options = self
            .has_miracle(definition)
            .then(|| DecisionOption {
                id: 1,
                label: format!("Reveal {name}"),
                card: Some((
                    card,
                    ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
                )),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Hand,
            })
            .into_iter()
            .collect::<Vec<_>>();
        let has_action = !options.is_empty();
        let decision = self.next_decision_id;
        self.queue_decision(
            player,
            format!("Take an action with {name}?"),
            DecisionVisibility::Private,
            DecisionPreference::PreferOption(1),
            0..=1,
            false,
            options,
            DecisionContinuation::DrawActionWindow { card },
        );
        if !has_action {
            // Allocate and resolve the same decision path as a declined
            // Miracle, but do it inside the atomic draw: there is no player
            // choice to suspend a multi-card instruction or present to a
            // host. The shared allocation keeps later public decision IDs
            // independent of the hidden card's identity.
            self.choose_decision(player, decision, &[]);
        }
    }

    /// Reveals the drawn card and captures the triggered half of Miracle.
    /// Trigger placement waits until the interrupted draw/effect procedure is
    /// complete, so the cast offer cannot appear in the middle of a draw.
    pub(in crate::game) fn reveal_miracle(&mut self, player: PlayerId, card: GameObjectId) {
        let Some(held) = self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)
            .cloned()
        else {
            return;
        };
        let Some((origin, ability)) = self.miracle_ability(held.definition) else {
            return;
        };
        self.events.push(GameEvent::CardRevealed {
            player,
            card,
            definition: held.definition,
        });
        self.capture_trigger(&TriggerCapture {
            source: AbilitySourceRef {
                object: card,
                ability: origin,
            },
            presentation: Self::ability_presentation(
                origin,
                ObjectCharacteristics::card(held.definition, CardPartId::PRIMARY),
            ),
            owner: held.owner,
            controller: player,
            text: ability.text,
            target_defs: Vec::new(),
            targets: Vec::new(),
            effect: EffectDef::None,
            resolver: Self::ability_resolver(origin, &ability),
            context: TriggerContext::empty().into(),
            condition: None,
            x: 0,
        });
    }

    pub(in crate::game) fn draw_cards(&mut self, player: PlayerId, count: u16) {
        if count == 0 {
            return;
        }
        if !self.pending_decisions.is_empty()
            || !self.pending_events.is_empty()
            || !self.pending_procedures.is_empty()
        {
            self.pending_procedures
                .push_back(PendingProcedure::DrawCards {
                    player,
                    remaining: count,
                });
            return;
        }
        let mut remaining = count;
        while remaining > 0 {
            if self.result.is_some() {
                break;
            }
            remaining -= 1;
            let _ = self.draw_card(player);
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                if remaining > 0 {
                    self.pending_procedures
                        .push_back(PendingProcedure::DrawCards { player, remaining });
                }
                break;
            }
        }
    }

    /// Draws every card for the active player first, then every card for the
    /// other player. Each player's draws still happen one at a time so draw
    /// replacements can suspend the instruction. One spell can deck both
    /// players, so empty-library losses remain deferred until the complete
    /// simultaneous instruction finishes. Empty-library loss is recorded on
    /// each player and settled at the next state-based-action check.
    #[cfg(test)]
    pub(in crate::game) fn draw_cards_simultaneously(&mut self, counts: [u16; 2]) {
        let was_deferred = self.defer_empty_library_loss;
        self.defer_empty_library_loss = true;
        self.continue_simultaneous_draws(counts, self.active_player, was_deferred);
    }

    pub(in crate::game) fn continue_simultaneous_draws(
        &mut self,
        mut remaining: [u16; 2],
        mut next: PlayerId,
        was_deferred: bool,
    ) {
        while remaining.iter().any(|count| *count > 0) && self.result.is_none() {
            let player = next;
            if remaining[player.index()] == 0 {
                next = player.opponent();
                continue;
            }
            remaining[player.index()] -= 1;
            let _ = self.draw_card(player);
            if remaining[player.index()] == 0 {
                next = player.opponent();
            }
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                self.pending_procedures
                    .push_back(PendingProcedure::SimultaneousDraws {
                        remaining,
                        next,
                        was_deferred,
                    });
                return;
            }
        }
        self.defer_empty_library_loss = was_deferred;
    }
}
