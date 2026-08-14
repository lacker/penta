use super::{
    AlternativeCastKindDef, AttachmentForm, BattlefieldExitCompletion, CardBehavior, CardPartId,
    CardRuntime, CardType, CounterKind, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, EntryCompletion, Game, GameEvent, GameObjectId,
    PendingBattlefieldEntry, Permanent, PlayerId, ResolvedAbility, StackAbilityResolver,
    StackObject, StackObjectKind, Target, ZoneKind, public_cards,
};

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
                let pending_before = self.pending_decisions.len();
                let resolved = self.resolve_stack_ability(&object);
                if self.defer_stack_resolution(pending_before, &object, resolved) {
                    return;
                }
                self.finish_stack_resolution(object, resolved);
                return;
            }
            StackObjectKind::Spell => {}
        }
        let definition = object.card.definition;
        let behavior = self
            .behavior(definition)
            .unwrap_or(CardBehavior::Unsupported);
        let spell_types = self
            .stack_spell_types(&object)
            .unwrap_or_else(|| behavior.types());
        let bestowed = object.signature.as_ref().and_then(|signature| {
            let definition = self.catalog.get(definition)?;
            let option = definition.play_option(signature.play_option())?;
            self.selected_alternative_kind(definition, option, object.id, signature.costs())
        }) == Some(AlternativeCastKindDef::Bestow);
        let targets_illegal = self.spell_fizzles(&object);
        let aura_host = (!bestowed || !targets_illegal)
            .then(|| Self::aura_host_for(&object))
            .flatten();
        let aura_fizzles =
            spell_types.is_permanent() && aura_host.is_some() && targets_illegal && !bestowed;
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
            );
            self.initialize_battlefield_entry(&mut permanent);
            permanent.chosen_player = chosen_player;
            permanent.text_changes = object.text_changes;
            permanent.attached_to = aura_host;
            if bestowed && aura_host.is_some() {
                permanent.attachment_form = Some(AttachmentForm::Bestowed {
                    timestamp: permanent.timestamp,
                });
            }
            self.enqueue_battlefield_entry(PendingBattlefieldEntry {
                permanent,
                from: ZoneKind::Stack,
                completion: EntryCompletion::SpellResolved {
                    card: object.id,
                    definition,
                },
            });
            return;
        } else if aura_fizzles || self.spell_fizzles(&object) {
            // 608.2b: a spell whose targets are all illegal on resolution does
            // nothing at all — a second Counterspell aimed at the same target
            // arrives to find it gone and goes to the graveyard spent.
            self.events.push(GameEvent::SpellFizzled {
                card: object.id,
                definition,
            });
        } else if object.ability.is_some() {
            let pending_before = self.pending_decisions.len();
            let _ = self.resolve_stack_ability(&object);
            if self.defer_stack_resolution(pending_before, &object, true) {
                return;
            }
        } else {
            let pending_before = self.pending_decisions.len();
            self.resolve_spell_effect(&object, behavior);
            if self.defer_stack_resolution(pending_before, &object, true) {
                return;
            }
        }
        self.finish_stack_resolution(object, true);
    }

    pub(super) fn finish_stack_resolution(&mut self, object: StackObject, resolved: bool) {
        let definition = object.card.definition;
        match object.kind {
            StackObjectKind::ActivatedAbility => {
                let source = object
                    .source
                    .expect("activated abilities remember their source");
                let event = if resolved {
                    GameEvent::AbilityResolved {
                        object: object.id,
                        source,
                        definition,
                    }
                } else {
                    GameEvent::AbilityFizzled {
                        object: object.id,
                        source,
                        definition,
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
                        definition,
                    }
                } else {
                    GameEvent::TriggeredAbilityFizzled {
                        object: object.id,
                        source,
                        definition,
                    }
                };
                self.events.push(event);
                return;
            }
            StackObjectKind::Spell => {}
        }

        let behavior = self
            .behavior(definition)
            .unwrap_or(CardBehavior::Unsupported);
        let spell_types = self
            .stack_spell_types(&object)
            .unwrap_or_else(|| behavior.types());
        let aura_fizzles = spell_types.is_permanent()
            && Self::aura_host_for(&object).is_some()
            && self.spell_fizzles(&object);
        let card_id = object.id;
        if (!spell_types.is_permanent() || aura_fizzles) && !object.is_copy {
            let owner = object.card.owner;
            // A flashback spell exiles itself instead of returning to the
            // graveyard it was cast from, which is what keeps it from being
            // flashed back again.
            let (card, _zone_change) = self.zone_change_card(object.card);
            if object.cast_via_flashback || behavior == CardBehavior::Recall {
                self.players[owner.index()].exile.push(card);
            } else {
                self.put_card_into_graveyard(owner, card);
            }
        }
        self.events.push(GameEvent::SpellResolved {
            card: card_id,
            definition,
        });
    }

    fn defer_stack_resolution(
        &mut self,
        pending_before: usize,
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
        false
    }

    /// Sin Collector and Lifebane Zombie reveal the targeted player's hand,
    /// then choose and exile one card matching the source's printed filter.
    pub(super) fn queue_reveal_and_exile(
        &mut self,
        controller: PlayerId,
        victim: PlayerId,
        behavior: CardBehavior,
    ) {
        self.last_seen_hands[controller.index()] =
            Some((victim, public_cards(&self.players[victim.index()].hand)));
        let eligible = self.players[victim.index()]
            .hand
            .iter()
            .filter(|card| {
                self.catalog
                    .get(card.definition)
                    .is_some_and(|definition| match behavior {
                        CardBehavior::LifebaneZombie => {
                            let colors = definition.rules.colors();
                            definition.rules.has_type(CardType::Creature)
                                && (colors[0] || colors[4])
                        }
                        CardBehavior::SinCollector => {
                            definition.rules.has_type(CardType::Instant)
                                || definition.rules.has_type(CardType::Sorcery)
                        }
                        _ => false,
                    })
            })
            .cloned()
            .collect::<Vec<_>>();
        if eligible.is_empty() {
            // The hand is revealed and holds nothing this card can take. A
            // prompt with no options is answerable, but there is nothing to
            // ask.
            return;
        }
        let options = self.card_decision_options(&eligible, DecisionZone::Hand);
        let prompt = if behavior == CardBehavior::LifebaneZombie {
            "Exile a green or white creature card from their hand"
        } else {
            "Exile an instant or sorcery card from their hand"
        };
        // The hand is revealed, so the choice is public rather than hidden.
        self.queue_decision(
            controller,
            prompt,
            DecisionVisibility::Public,
            DecisionPreference::HigherCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::ExileFromHand { victim },
        );
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
                ability.context,
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
                    ability.context,
                    ability.mode_effects.as_slice(),
                )
            })
            .expect("ability stack objects freeze their complete payload");
        match resolver {
            StackAbilityResolver::Declarative(effect) => {
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
            .map(|permanent| (permanent.card.id, permanent.card.definition))
            .collect::<Vec<_>>();
        if tokens.is_empty() {
            return;
        }
        let options = tokens
            .iter()
            .enumerate()
            .map(|(index, (id, definition))| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self.catalog.get(*definition).map_or_else(
                    || "Unknown token".into(),
                    |definition| definition.name.clone(),
                ),
                card: Some((*id, *definition)),
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

    /// "You may draw two additional cards. If you do, choose two cards in
    /// your hand drawn this turn..." The offer comes first because declining
    /// it skips the rest of the ability entirely.
    pub(super) fn queue_sylvan_offer(&mut self, player: PlayerId) {
        self.queue_decision(
            player,
            "Draw two additional cards?",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Do not draw".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: "Draw two additional cards".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
            ],
            DecisionContinuation::SylvanOffer { player },
        );
    }

    /// The cards this player drew this turn that are still in hand, which is
    /// the pool Sylvan Library chooses from.
    pub(super) fn sylvan_candidates(&self, player: PlayerId) -> Vec<GameObjectId> {
        self.drawn_this_turn[player.index()]
            .iter()
            .copied()
            .filter(|drawn| {
                self.players[player.index()]
                    .hand
                    .iter()
                    .any(|card| card.id == *drawn)
            })
            .collect()
    }

    pub(super) fn resolve_custom_triggered_ability(
        &mut self,
        object: &StackObject,
        behavior: CardBehavior,
    ) {
        if behavior == CardBehavior::SylvanLibrary {
            self.queue_sylvan_offer(object.controller);
            return;
        }
        if matches!(
            behavior,
            CardBehavior::SinCollector | CardBehavior::LifebaneZombie
        ) {
            if let Some(Target::Player(victim)) = self.first_legal_ability_target(object) {
                self.queue_reveal_and_exile(object.controller, victim, behavior);
            }
            return;
        }
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
