//! Drawing cards: the ordinary one, the replacements that can interrupt it,
//! the miracle reveal that rides on the turn's first, and the simultaneous
//! opening draw. Split from the turn structure that schedules them because
//! a draw is one instruction whose interruptions are its own.

use super::super::{
    AbilityDef, AbilityOrigin, AbilitySourceRef, AlternativeCastKindDef, CardDefinitionId,
    CardPartId, CommittedTriggerEvent, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, DeclarativeAbilityDef, EffectDef, Game, GameEvent,
    GameObjectId, ObjectCharacteristics, PendingProcedure, Permanent, PlayerId,
    ReplacementConditionDef, ReplacementEffectDef, ReplacementEventDef, ScopedEffect,
    StackAbilityPayload, StackAbilityResolver, StackObject, StackObjectKind, Step, TriggerCapture,
    TriggerContext, ZoneKind,
};

impl Game {
    /// "Each opponent can't draw more than one card each turn": the bound
    /// the live static abilities put on this player, or none at all. Two
    /// such rules leave the smaller standing, which is what each of them
    /// says on its own terms.
    fn draw_bound_this_turn(&self, player: PlayerId) -> Option<u16> {
        let mut bound = None::<u16>;
        self.visit_player_static_rules(player, |rule| {
            if let crate::card::AppliedRuleDef::CannotDrawMoreThanEachTurn(amount) = rule {
                let amount = u16::from(amount);
                bound = Some(bound.map_or(amount, |current: u16| current.min(amount)));
            }
        });
        bound
    }

    pub(in crate::game) fn draw_card(&mut self, player: PlayerId) -> Option<GameObjectId> {
        self.continue_draw_card(player, Vec::new())
    }

    /// Re-evaluates one prospective draw after replacement effects have
    /// changed it. `applied` is event-local CR 614.5 state: another copy of
    /// an effect may still apply, but the same source ability may not.
    pub(in crate::game) fn continue_draw_card(
        &mut self,
        player: PlayerId,
        mut applied: Vec<AbilitySourceRef>,
    ) -> Option<GameObjectId> {
        // A draw past the bound simply does not happen (CR 121.3), so
        // nothing watching for a draw fires and no replacement is spent on
        // it -- the instruction is not carried out at all.
        if self
            .draw_bound_this_turn(player)
            .is_some_and(|bound| self.cards_drawn_this_turn[player.index()] >= bound)
        {
            return None;
        }
        let mut replacements = self.draw_replacements[player.index()]
            .drain(..)
            .collect::<Vec<_>>();
        replacements.extend(self.applicable_static_draw_replacements(player, &applied));
        match replacements.as_slice() {
            [] => self.commit_draw_card(player),
            [replacement] if !replacement.optional => {
                let replacement = replacement.clone();
                if let Some(source) = Self::draw_replacement_source(&replacement) {
                    applied.push(source);
                }
                self.apply_draw_replacement(player, replacement, applied);
                None
            }
            _ => {
                self.queue_draw_replacement_choice(player, applied, replacements);
                None
            }
        }
    }

    pub(in crate::game) fn commit_draw_card(&mut self, player: PlayerId) -> Option<GameObjectId> {
        let Some(card) = self.players[player.index()].library.pop() else {
            self.players[player.index()].tried_to_draw_from_empty_library = true;
            return None;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let card_id = card.id;
        let definition = card.definition;
        self.players[player.index()].hand.push(card);
        self.events.push(GameEvent::CardDrawn {
            player,
            card: card_id,
        });
        if self.player_rule_applies(player, crate::card::AppliedRuleDef::RevealsDrawnCards) {
            self.events.push(GameEvent::CardRevealed {
                player,
                card: card_id,
                definition,
            });
            for viewer in &mut self.last_seen_hands {
                *viewer = Some((player, vec![(card_id, definition)]));
            }
        }
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
        let card = self
            .printed_trigger_event_object(
                card_id,
                definition,
                player,
                &crate::CharacteristicContext::Hand,
            )
            .expect("a drawn catalog card has hand characteristics");
        self.capture_battlefield_triggers(&CommittedTriggerEvent::DrewCard {
            player,
            card,
            first_in_draw_step,
            nth_this_turn: self.cards_drawn_this_turn[player.index()],
        });
        Some(card_id)
    }

    fn queue_draw_replacement_choice(
        &mut self,
        player: PlayerId,
        applied: Vec<AbilitySourceRef>,
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
                applied,
                replacements,
            },
        );
    }

    fn applicable_static_draw_replacements(
        &self,
        player: PlayerId,
        applied: &[AbilitySourceRef],
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
                    except_first_in_draw_step,
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
                let source = AbilitySourceRef {
                    object: permanent.card.id,
                    ability: effective.origin,
                };
                let condition_matches = definition.condition.is_none_or(|condition| {
                    self.per_card_draw_replacement_condition_matches(permanent, condition)
                });
                let event_context = TriggerContext {
                    event_player: Some(player),
                    ..TriggerContext::empty()
                };
                if applied.contains(&source)
                    || !definition.source_zones.contains(&ZoneKind::Battlefield)
                    || (during_own_draw_step
                        && (self.step != Step::Draw || self.active_player != player))
                    // "Except the first one they draw in each of their draw
                    // steps": the exempt draw is the one that actually
                    // happens, which is what sets the flag this reads.
                    || (except_first_in_draw_step
                        && self.step == Step::Draw
                        && self.active_player == player
                        && !self.draw_step_draw_taken[player.index()])
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
                replacements.push(Self::static_draw_replacement(
                    permanent,
                    effective.origin,
                    presentation,
                    ability.text,
                    event_context,
                    effect,
                    definition.optional,
                ));
            });
        }
        replacements
    }

    #[allow(clippy::too_many_arguments)]
    fn static_draw_replacement(
        permanent: &Permanent,
        origin: AbilityOrigin,
        presentation: ObjectCharacteristics,
        text: &'static str,
        event_context: TriggerContext,
        effect: EffectDef,
        optional: bool,
    ) -> super::super::DrawReplacement {
        let scoped = ScopedEffect::primary(effect);
        let object = StackObject {
            id: permanent.card.id,
            kind: StackObjectKind::TriggeredAbility,
            card: permanent.card.clone(),
            source: Some(permanent.card.id),
            ability: Some(StackAbilityPayload {
                origin,
                definition: None,
                presentation,
                text: Some(text),
                target_defs: Vec::new(),
                targets: Vec::new(),
                context: event_context.into(),
                resolver: StackAbilityResolver::Declarative(scoped),
                condition: None,
                mode_effects: Vec::new(),
                resolution_destination: None,
                x: 0,
                sacrificed_mana_value: 0,
            }),
            controller: permanent.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast: None,
            face_down: None,
            is_copy: false,
        };
        super::super::DrawReplacement {
            object: Box::new(object),
            context: event_context.into(),
            effect: scoped,
            optional,
            installed: false,
        }
    }

    /// Conditions read for each prospective card draw. A hand-size condition
    /// belongs to the whole instruction instead, while cast information
    /// belongs to a battlefield entry.
    fn per_card_draw_replacement_condition_matches(
        &self,
        permanent: &Permanent,
        condition: ReplacementConditionDef,
    ) -> bool {
        match condition {
            ReplacementConditionDef::SourceTapped => permanent.tapped,
            ReplacementConditionDef::CreatureDiedThisTurn => self.creature_died_this_turn,
            ReplacementConditionDef::ControllerLibraryEmpty => self.players
                [permanent.controller.index()]
            .library
            .is_empty(),
            ReplacementConditionDef::SourceCastWith(_)
            | ReplacementConditionDef::SourcePaidAdditionalCost(_)
            | ReplacementConditionDef::SourceNotCastFrom(_)
            | ReplacementConditionDef::OpponentWasDealtDamageThisTurn
            | ReplacementConditionDef::ControllerHandAtMost(_) => false,
        }
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

    pub(in crate::game) fn draw_replacement_source(
        replacement: &super::super::DrawReplacement,
    ) -> Option<AbilitySourceRef> {
        if replacement.installed {
            return None;
        }
        let ability = replacement.object.ability.as_ref()?;
        Some(AbilitySourceRef {
            object: replacement.object.source.unwrap_or(replacement.object.id),
            ability: ability.origin,
        })
    }

    pub(in crate::game) fn apply_draw_replacement(
        &mut self,
        player: PlayerId,
        replacement: super::super::DrawReplacement,
        applied: Vec<AbilitySourceRef>,
    ) {
        let mut context = replacement.context;
        context.replaced_draw = Some(super::super::ReplacedDrawContinuation { player, applied });
        self.resolve_effect_def(replacement.effect, &replacement.object, context);
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
                (matches!(
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
            modes: None,
            x: 0,
        });
    }

    /// One printed draw instruction, which is where "you draw that many
    /// cards plus one instead" applies: the whole instruction is replaced
    /// once, not each of its cards. Resuming a deferred draw goes to
    /// [`Self::draw_cards`] instead, which is why the addition lives here.
    pub(in crate::game) fn draw_instruction(&mut self, player: PlayerId, count: u16) {
        if count == 0 {
            return;
        }
        let extra = self.additional_cards_drawn(player);
        self.draw_cards(player, count.saturating_add(extra));
    }

    /// How many cards a static replacement adds to this player's next draw
    /// instruction. Read as the instruction would happen, so a hand that is
    /// small now counts however large it becomes while the cards arrive.
    fn additional_cards_drawn(&self, player: PlayerId) -> u16 {
        let mut extra = 0_u16;
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return;
                };
                let ReplacementEventDef::WouldDraw {
                    player: relation,
                    during_own_draw_step,
                    except_first_in_draw_step,
                } = definition.event
                else {
                    return;
                };
                let Some(ReplacementEffectDef::AddToEventAmount(amount)) =
                    ability.declarative_replacement()
                else {
                    return;
                };
                let condition_matches = match definition.condition {
                    None => true,
                    Some(ReplacementConditionDef::ControllerHandAtMost(most)) => {
                        self.players[permanent.controller.index()].hand.len() <= usize::from(most)
                    }
                    Some(
                        ReplacementConditionDef::SourceTapped
                        | ReplacementConditionDef::CreatureDiedThisTurn
                        | ReplacementConditionDef::SourceCastWith(_)
                        | ReplacementConditionDef::SourcePaidAdditionalCost(_)
                        | ReplacementConditionDef::SourceNotCastFrom(_)
                        | ReplacementConditionDef::OpponentWasDealtDamageThisTurn
                        | ReplacementConditionDef::ControllerLibraryEmpty,
                    ) => false,
                };
                let event_context = TriggerContext {
                    event_player: Some(player),
                    ..TriggerContext::empty()
                };
                if !definition.source_zones.contains(&ZoneKind::Battlefield)
                    || (during_own_draw_step
                        && (self.step != Step::Draw || self.active_player != player))
                    // "Except the first one they draw in each of their draw
                    // steps": the exempt draw is the one that actually
                    // happens, which is what sets the flag this reads.
                    || (except_first_in_draw_step
                        && self.step == Step::Draw
                        && self.active_player == player
                        && !self.draw_step_draw_taken[player.index()])
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
                extra = extra.saturating_add(amount);
            });
        }
        extra
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
