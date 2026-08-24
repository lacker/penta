use super::{
    BattlefieldExitCompletion, CardBehavior, CardPartId, CardRuntime, CardType, CounterKind,
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    EntryCompletion, Game, GameEvent, GameObjectId, PendingBattlefieldEntry, PendingProcedure,
    Permanent, PlayerId, ResolvedAbility, StackAbilityResolver, StackObject, StackObjectKind,
    Target, ZoneKind,
};
use crate::SpellResolutionDestinationDef;

impl Game {
    pub(super) fn pass_priority(&mut self, _player: PlayerId) {
        self.consecutive_passes += 1;
        if self.consecutive_passes == 1 {
            self.priority = self.priority.opponent();
            return;
        }

        self.consecutive_passes = 0;
        if self.stack.is_empty() {
            self.advance_step();
        } else {
            self.resolve_stack_top();
            if self.result.is_none() {
                self.priority = self.active_player;
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_stack_top(&mut self) {
        let object = self
            .stack
            .pop()
            .expect("resolution is requested only for a nonempty stack");
        self.retire_stack_object(&object);
        match object.kind {
            StackObjectKind::ActivatedAbility | StackObjectKind::TriggeredAbility => {
                // Counted as the resolution begins rather than after it, so
                // an ability asking whether this is the first time it has
                // resolved this turn counts the one asking. A resolution
                // that suspends on a decision resumes at the finish, never
                // here, so nothing is counted twice.
                self.record_ability_resolution(&object);
                let pending_before = self.pending_decisions.len();
                let procedures_before = self.pending_procedures.len();
                let events_before = self.pending_events.len();
                let resolved = self.resolve_stack_ability(&object);
                if self.defer_stack_resolution(
                    pending_before,
                    procedures_before,
                    events_before,
                    &object,
                    resolved,
                ) {
                    return;
                }
                self.finish_stack_resolution(&object, resolved);
                return;
            }
            StackObjectKind::Spell => {}
        }
        let definition = object
            .card
            .definition
            .card_definition()
            .expect("a spell object is backed by a card definition");
        let behavior = self
            .behavior(definition)
            .unwrap_or(CardBehavior::Unsupported);
        let spell_types = self
            .stack_spell_types(&object)
            .unwrap_or_else(|| behavior.types());
        let aura_host = Self::aura_host_for(&object);
        let aura_fizzles =
            spell_types.is_permanent() && aura_host.is_some() && self.spell_fizzles(&object);
        if spell_types.is_permanent() && !aura_fizzles {
            let chosen_player = match object.first_target() {
                Some(Target::Player(player)) => Some(player),
                _ => None,
            };
            let presented = object
                .signature
                .as_ref()
                .and_then(|signature| match signature.form() {
                    crate::card::SpellForm::Part(part) => Some(*part),
                    crate::card::SpellForm::Combined(parts) => parts.first().copied(),
                })
                .unwrap_or(CardPartId::PRIMARY);
            let mut permanent = Permanent::entering(
                object.card,
                presented,
                object.controller,
                self.turns_started[object.controller.index()],
                self.turn,
            );
            permanent.face_down = object.face_down;
            self.initialize_battlefield_entry(&mut permanent);
            if object.phyrexian_symbols_paid_with_life > 0
                && self.effective_rules(&permanent).is_some_and(|rules| {
                    rules.has_executable_keyword(crate::card::KeywordAbility::Compleated)
                })
            {
                let loyalty = permanent
                    .counters(CounterKind::Loyalty)
                    .saturating_sub(object.phyrexian_symbols_paid_with_life.saturating_mul(2));
                permanent.set_counters(CounterKind::Loyalty, loyalty);
            }
            permanent.chosen_player = chosen_player;
            permanent.cast_x = object
                .signature
                .as_ref()
                .map_or(0, crate::casting::CastSignature::x);
            permanent.cast_alternative = object.signature.as_ref().and_then(|signature| {
                let card = self.catalog.get(definition)?;
                let option = card.play_option(signature.play_option())?;
                self.selected_alternative_kind(card, option, object.id, signature.costs())
            });
            permanent.cast_at_instant_speed = object.cast_at_instant_speed;
            permanent.cast_from_zone = object.cast_from_zone;
            permanent.text_changes = object.text_changes;
            permanent.attached_to = aura_host;
            self.enqueue_battlefield_entry(PendingBattlefieldEntry {
                permanent,
                from: ZoneKind::Stack,
                completion: EntryCompletion::SpellResolved {
                    card: object.id,
                    definition,
                },
                redirected_to: None,
            });
            return;
        }
        let spell_fizzled = aura_fizzles || self.spell_fizzles(&object);
        if spell_fizzled {
            // 608.2b: a spell whose targets are all illegal on resolution does
            // nothing at all — a second Counterspell aimed at the same target
            // arrives to find it gone and goes to the graveyard spent.
            self.events.push(GameEvent::SpellFizzled {
                card: object.id,
                definition,
            });
        } else if object.ability.is_some() {
            let pending_before = self.pending_decisions.len();
            let procedures_before = self.pending_procedures.len();
            let events_before = self.pending_events.len();
            let _ = self.resolve_stack_ability(&object);
            if self.defer_stack_resolution(
                pending_before,
                procedures_before,
                events_before,
                &object,
                true,
            ) {
                return;
            }
        } else {
            let pending_before = self.pending_decisions.len();
            let procedures_before = self.pending_procedures.len();
            let events_before = self.pending_events.len();
            self.resolve_spell_effect(&object, behavior);
            if self.defer_stack_resolution(
                pending_before,
                procedures_before,
                events_before,
                &object,
                true,
            ) {
                return;
            }
        }
        self.finish_stack_resolution(&object, !spell_fizzled);
    }

    /// Records that one of a permanent's abilities is resolving, for the
    /// cards that count their own resolutions.
    fn record_ability_resolution(&mut self, object: &StackObject) {
        let Some(payload) = object.ability.as_ref() else {
            return;
        };
        let Some(source) = object.source else {
            return;
        };
        let origin = payload.origin;
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        match permanent
            .resolutions_this_turn
            .iter_mut()
            .find(|(recorded, _)| *recorded == origin)
        {
            Some((_, count)) => *count = count.saturating_add(1),
            None => permanent.resolutions_this_turn.push((origin, 1)),
        }
    }

    pub(super) fn finish_stack_resolution(&mut self, object: &StackObject, resolved: bool) {
        let presentation = object.presentation();
        match object.kind {
            StackObjectKind::ActivatedAbility => {
                let source = object
                    .source
                    .expect("activated abilities remember their source");
                let event = if resolved {
                    GameEvent::AbilityResolved {
                        object: object.id,
                        source,
                        presentation,
                    }
                } else {
                    GameEvent::AbilityFizzled {
                        object: object.id,
                        source,
                        presentation,
                    }
                };
                self.events.push(event);
                return;
            }
            StackObjectKind::TriggeredAbility => {
                let source = object
                    .source
                    .expect("triggered abilities remember their source");
                let event = if resolved {
                    GameEvent::TriggeredAbilityResolved {
                        object: object.id,
                        source,
                        presentation,
                    }
                } else {
                    GameEvent::TriggeredAbilityFizzled {
                        object: object.id,
                        source,
                        presentation,
                    }
                };
                self.events.push(event);
                return;
            }
            StackObjectKind::Spell => {}
        }

        let definition = object
            .card
            .definition
            .card_definition()
            .expect("a spell object is backed by a card definition");

        let behavior = self
            .behavior(definition)
            .unwrap_or(CardBehavior::Unsupported);
        let spell_types = self
            .stack_spell_types(object)
            .unwrap_or_else(|| behavior.types());
        let aura_fizzles = spell_types.is_permanent()
            && Self::aura_host_for(object).is_some()
            && self.spell_fizzles(object);
        let card_id = object.id;
        if !spell_types.is_permanent() || aura_fizzles {
            self.finish_spell_destination(object, behavior, resolved);
        }
        self.events.push(GameEvent::SpellResolved {
            card: card_id,
            definition,
        });
    }

    fn finish_spell_destination(
        &mut self,
        object: &StackObject,
        behavior: CardBehavior,
        resolved: bool,
    ) {
        let owner = object.card.owner;
        let destination = if resolved {
            object
                .ability
                .as_ref()
                .and_then(|ability| ability.resolution_destination)
                .unwrap_or(SpellResolutionDestinationDef::Graveyard)
        } else {
            SpellResolutionDestinationDef::Graveyard
        };
        // Rebound only exiles a spell its caster cast from hand. Settled here
        // rather than in the walk below because it is a question about the
        // cast rather than about the move: from anywhere else this is an
        // ordinary spell going to an ordinary graveyard.
        let destination = match destination {
            SpellResolutionDestinationDef::ExileIfCastFromHand => {
                if object
                    .cast_from_zone
                    .is_some_and(|from| from.zone() == ZoneKind::Hand)
                {
                    SpellResolutionDestinationDef::Exile
                } else {
                    SpellResolutionDestinationDef::Graveyard
                }
            }
            other => other,
        };
        if object.is_copy {
            // A copy has no card to move, but "shuffle it into its owner's
            // library" still instructs its controller to shuffle.
            if destination == SpellResolutionDestinationDef::LibraryShuffled {
                self.rng.shuffle(&mut self.players[owner.index()].library);
            }
            return;
        }

        // Flashback replaces the move from the stack with exile, not the rest
        // of the resolution. In particular, White Sun's Zenith still shuffles
        // its owner's library, and a spell that already exiles itself still
        // gets its destination counters.
        let flashback_replaces_move = object.cast_via_flashback || behavior == CardBehavior::Recall;
        let (mut card, _zone_change) = self.zone_change_card(
            object
                .card
                .clone()
                .into_card()
                .expect("a spell object is backed by a card"),
        );
        match destination {
            SpellResolutionDestinationDef::Graveyard if !flashback_replaces_move => {
                self.put_card_into_graveyard(owner, card);
            }
            SpellResolutionDestinationDef::Hand if !flashback_replaces_move => {
                self.players[owner.index()].hand.push(card);
            }
            // Flashback exiles the card wherever else it would have gone, so
            // a bought-back flashback spell is still exiled rather than
            // returned (CR 702.34a).
            SpellResolutionDestinationDef::Graveyard
            | SpellResolutionDestinationDef::Hand
            | SpellResolutionDestinationDef::Exile => {
                self.players[owner.index()].exile.push(card);
            }
            SpellResolutionDestinationDef::ExileOnAdventure => {
                // The exiled card is a new object, and it is that object its
                // owner may cast the creature half of later.
                self.permit_adventure_return(card.id, owner);
                self.players[owner.index()].exile.push(card);
            }
            SpellResolutionDestinationDef::ExileWithCounters(counters) => {
                for &(kind, amount) in counters {
                    card.add_counters(kind, amount);
                }
                self.players[owner.index()].exile.push(card);
            }
            // Resolved into one of the two above before the walk began.
            SpellResolutionDestinationDef::ExileIfCastFromHand => {}
            SpellResolutionDestinationDef::LibraryShuffled => {
                if flashback_replaces_move {
                    self.players[owner.index()].exile.push(card);
                } else {
                    self.players[owner.index()].library.push(card);
                }
                self.rng.shuffle(&mut self.players[owner.index()].library);
            }
        }
    }

    fn defer_stack_resolution(
        &mut self,
        pending_before: usize,
        procedures_before: usize,
        events_before: usize,
        object: &StackObject,
        resolved: bool,
    ) -> bool {
        if self.defer_after_battlefield_exit(
            pending_before,
            BattlefieldExitCompletion::FinishStackResolution {
                object: Box::new(object.clone()),
                resolved,
            },
        ) {
            return true;
        }
        if self.pending_decisions.len() > pending_before
            || self.pending_procedures.len() > procedures_before
            || self.pending_events.len() > events_before
        {
            self.pending_procedures
                .push_back(PendingProcedure::FinishStackResolution {
                    object: Box::new(object.clone()),
                    resolved,
                });
            return true;
        }
        false
    }

    pub(super) fn resolve_stack_ability(&mut self, object: &StackObject) -> bool {
        if self.stack_ability_fizzles(object) {
            return false;
        }
        // Rule 603.4's second look. A condition that has stopped holding
        // since the ability triggered makes it do nothing at all, which is
        // reported the same way an ability with no legal target is.
        if let Some(ability) = object.ability.as_ref()
            && let Some(condition) = ability.condition
            && !self.trigger_condition_holds(
                condition,
                object.source.unwrap_or(object.id),
                object.controller,
                ability.context.trigger,
                Some(ability.origin),
                None,
            )
        {
            return false;
        }
        let (resolver, context, mode_effects) = object
            .ability
            .as_ref()
            .map(|ability| {
                (
                    ability.resolver,
                    ability.context.clone(),
                    ability.mode_effects.as_slice(),
                )
            })
            .expect("ability stack objects freeze their complete payload");
        match resolver {
            StackAbilityResolver::Declarative(effect)
            | StackAbilityResolver::DeclarativeIgnoringTargetFizzle(effect) => {
                let mut effects = Vec::with_capacity(mode_effects.len() + 1);
                effects.push(effect);
                effects.extend_from_slice(mode_effects);
                self.resolve_effects_in_order(effects, object, context, None);
            }
            StackAbilityResolver::DeclarativeWithCustomFollowup { effect, behavior } => {
                let mut effects = Vec::with_capacity(mode_effects.len() + 1);
                effects.push(effect);
                effects.extend_from_slice(mode_effects);
                self.resolve_effects_in_order(effects, object, context, Some(behavior));
            }
            StackAbilityResolver::Custom(behavior) => match object.kind {
                StackObjectKind::Spell => self.resolve_spell_effect(object, behavior),
                StackObjectKind::ActivatedAbility => {
                    self.resolve_custom_activated_ability(object, behavior);
                }
                StackObjectKind::TriggeredAbility => {
                    self.resolve_custom_triggered_ability(object, behavior);
                }
            },
            StackAbilityResolver::CardOwned(resolver) => {
                let targets = object
                    .ability
                    .as_ref()
                    .map_or_else(Vec::new, |ability| ability.targets.clone());
                let mut runtime = CardRuntime { game: self };
                resolver.resolve(
                    &mut runtime,
                    &ResolvedAbility {
                        controller: object.controller,
                        targets,
                    },
                );
            }
            StackAbilityResolver::CastOffer(alternative) => {
                if let (Some(card), Some(payload)) = (object.source, object.ability.as_ref()) {
                    debug_assert_eq!(
                        self.ability_for_origin(card, payload.origin)
                            .and_then(|ability| match ability.definition {
                                super::DeclarativeAbilityDef::AlternativeCast(definition) => {
                                    Some(definition.kind)
                                }
                                _ => None,
                            }),
                        Some(alternative),
                    );
                    self.queue_alternative_cast_offer(object.controller, card, payload.origin);
                }
            }
        }
        true
    }

    /// Tetravus offers one option per +1/+1 counter it carries, so the number
    /// of options taken is the number of counters traded away.
    pub(super) fn queue_tetravus_detach(&mut self, controller: PlayerId, source: GameObjectId) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        let counters = usize::from(permanent.counters(CounterKind::PlusOnePlusOne));
        if counters == 0 {
            return;
        }
        let options = (0..counters)
            .map(|index| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: "Trade a +1/+1 counter for a Tetravite".into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
            .collect();
        self.queue_decision(
            controller,
            "Remove any number of +1/+1 counters from Tetravus",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            0..=counters,
            false,
            options,
            DecisionContinuation::TetravusDetach { source },
        );
    }

    /// Only the Tetravites this Tetravus made are eligible; a second Tetravus
    /// keeps its own, and a token that outlived its creator can never come
    /// back.
    pub(super) fn queue_tetravus_assemble(&mut self, controller: PlayerId, source: GameObjectId) {
        let tokens = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.created_by == Some(source))
            .map(|permanent| (permanent.card.id, Self::effective_rules_source(permanent)))
            .collect::<Vec<_>>();
        if tokens.is_empty() {
            return;
        }
        let options = tokens
            .iter()
            .enumerate()
            .map(|(index, (id, presentation))| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self
                    .presentation_name(*presentation)
                    .map_or_else(|| "Unknown token".to_owned(), std::borrow::Cow::into_owned),
                card: Some((*id, *presentation)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
            .collect();
        let total = tokens.len();
        self.queue_decision(
            controller,
            "Exile any number of Tetravites created with Tetravus",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            0..=total,
            false,
            options,
            DecisionContinuation::TetravusAssemble { source },
        );
    }

    pub(super) fn resolve_custom_triggered_ability(
        &mut self,
        object: &StackObject,
        behavior: CardBehavior,
    ) {
        if matches!(
            behavior,
            CardBehavior::TetravusDetach | CardBehavior::TetravusAssemble
        ) {
            let source = object.source.unwrap_or(object.id);
            if behavior == CardBehavior::TetravusDetach {
                self.queue_tetravus_detach(object.controller, source);
            } else {
                self.queue_tetravus_assemble(object.controller, source);
            }
            return;
        }
        if behavior == CardBehavior::AugurOfBolas {
            let controller = object.controller;
            let revealed = self.take_top_of_library(controller, 3);
            let eligible = revealed
                .iter()
                .filter(|card| {
                    self.catalog.get(card.definition).is_some_and(|definition| {
                        definition.rules.has_type(CardType::Instant)
                            || definition.rules.has_type(CardType::Sorcery)
                    })
                })
                .cloned()
                .collect::<Vec<_>>();
            let options = self.card_decision_options(&eligible, DecisionZone::Library);
            // "You may reveal": taking nothing is a real choice, so the minimum
            // is zero even when something qualifies.
            self.queue_decision(
                controller,
                "Put an instant or sorcery card into your hand",
                DecisionVisibility::Public,
                DecisionPreference::HigherCardValue,
                0..=1,
                false,
                options,
                DecisionContinuation::AugurOfBolas {
                    player: controller,
                    revealed,
                },
            );
        }
    }

    pub(super) fn resolve_custom_spell_followup(
        &mut self,
        object: &StackObject,
        behavior: CardBehavior,
    ) {
        if behavior == CardBehavior::ChainLightning {
            let deciding = match object.first_target() {
                Some(Target::Player(player)) => Some(player),
                Some(Target::Permanent(id)) => self.permanent_controller(id),
                Some(Target::Card(_) | Target::Spell(_)) | None => None,
            };
            if let Some(player) = deciding {
                self.queue_chain_lightning_decision(player, object.clone());
            }
        }
    }

    pub(super) fn stack_ability_fizzles(&self, object: &StackObject) -> bool {
        let Some(ability) = &object.ability else {
            return false;
        };
        if matches!(
            ability.resolver,
            StackAbilityResolver::DeclarativeIgnoringTargetFizzle(_)
        ) {
            return false;
        }
        let mut had_target = false;
        let mut has_legal_target = false;
        for selection in &ability.targets {
            if ability.target_defs.get(selection.slot().index()).is_none() {
                continue;
            }
            for target in selection.targets() {
                had_target = true;
                has_legal_target |=
                    self.stack_ability_target_is_legal(object, selection.slot(), *target);
            }
        }
        had_target && !has_legal_target
    }
}
