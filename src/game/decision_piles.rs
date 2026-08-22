use super::{
    BalancePhase, BalanceTask, BattlefieldExitCompletion, CardInstance, CardPartId, CardRuntime,
    CommittedTriggerEvent, CounterKind, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, DeclarativeAbilityDef, DiscardFollowUp, EffectDef,
    EffectResolutionContext, Game, GameEvent, GameObjectId, ObjectCharacteristics,
    ObjectPredicateDef, Permanent, PileChoice, PileChosen, PileSplit, PilesSeparated, PlayerId,
    SacrificeDeclined, SacrificeFollowup, SacrificedAmountDef, ScopedEffect, StackObject, Step,
    TopCardSelectionDef, ZoneKind, ZoneMoveCause,
};

impl Game {
    pub(super) fn queue_top_card_selection(
        &mut self,
        player: PlayerId,
        looker: PlayerId,
        selection: &'static TopCardSelectionDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let count = self
            .effect_value(selection.count, object, &context, scoped)
            .max(0);
        let Ok(count) = usize::try_from(count) else {
            return;
        };
        let revealed = self.take_top_of_library(player, count);
        if revealed.is_empty() {
            if let Some(then) = selection.then {
                self.resolve_effect_def(scoped.with_effect(*then), object, context);
            }
            return;
        }
        let source = object.source.unwrap_or(object.id);
        let eligible = revealed
            .iter()
            .filter(|card| {
                selection.object.is_none_or(|predicate| {
                    // The name a step earlier in this resolution chose is
                    // read here, where the resolution is still in hand;
                    // the shared matcher has no context to read it from.
                    if predicate == ObjectPredicateDef::HasChosenName {
                        return context.chosen_name.as_ref().is_some_and(|name| {
                            self.catalog
                                .get(card.definition)
                                .is_some_and(|definition| definition.name == *name)
                        });
                    }
                    self.card_object_matches(predicate, card, ZoneKind::Library, source)
                })
            })
            .cloned()
            .collect::<Vec<_>>();
        // "Put all Goblin cards revealed this way into your hand" asks
        // nothing, so there is no decision to queue: the predicate has
        // already partitioned the cards and both halves go where they go.
        if selection.select_all_matching {
            let selected = eligible.iter().map(|card| card.id).collect::<Vec<_>>();
            let (chosen, rest): (Vec<_>, Vec<_>) = revealed
                .into_iter()
                .partition(|card| selected.contains(&card.id));
            self.finish_top_card_selection(player, chosen, rest, selection);
            if let Some(then) = selection.then {
                self.resolve_effect_def(scoped.with_effect(*then), object, context);
            }
            return;
        }
        let inspected = revealed
            .iter()
            .map(|card| {
                (
                    card.id,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                )
            })
            .collect::<Vec<_>>();
        let mut options = self.card_decision_options(&eligible, DecisionZone::Library);
        for option in &mut options {
            option.members.clone_from(&inspected);
        }
        // A selection that may take nothing is a look and nothing more, so it
        // is presented the same way as one with no eligible card: the cards
        // ride along as members and the only option acknowledges them.
        let looking_only = selection.maximum == 0;
        let no_selection = options.is_empty() || looking_only;
        if no_selection {
            options.clear();
            options.push(DecisionOption {
                id: 0,
                label: if looking_only {
                    "Put them back".into()
                } else {
                    "No inspected card is eligible".into()
                },
                card: None,
                members: inspected,
                ability_text: None,
                zone: DecisionZone::Library,
            });
        }
        let preference = if no_selection {
            DecisionPreference::Neutral
        } else if selection.selected_zone == ZoneKind::Hand {
            DecisionPreference::HigherCardValue
        } else {
            DecisionPreference::LowerCardValue
        };
        self.queue_decision(
            looker,
            Self::top_card_selection_prompt(selection),
            DecisionVisibility::Private,
            preference,
            if no_selection {
                0..=0
            } else {
                usize::from(selection.minimum)..=usize::from(selection.maximum)
            },
            false,
            options,
            DecisionContinuation::TopCardSelection {
                player,
                revealed,
                selection,
                object: Box::new(object.clone()),
                context,
                effect: scoped,
            },
        );
    }

    /// What the looker is asked. An arrangement says so, because the order is
    /// the whole answer and nothing else in the decision distinguishes one
    /// from an ordinary dig. Shared with the checkpoint, which compares the
    /// prompt it rebuilds against the one the observation carries.
    pub(super) const fn top_card_selection_prompt(
        selection: &'static crate::card::TopCardSelectionDef,
    ) -> &'static str {
        if selection.selected_order_follows_choice {
            "Put them back in any order, naming the top card first"
        } else {
            "Choose cards from the top of the library"
        }
    }

    /// Where the two halves of an inspected group go once they are settled.
    /// Shared by the decision's continuation and by the selection that asks
    /// nothing, so both place cards and reveal them the same way.
    pub(super) fn finish_top_card_selection(
        &mut self,
        player: PlayerId,
        chosen: Vec<CardInstance>,
        rest: Vec<CardInstance>,
        selection: &'static TopCardSelectionDef,
    ) {
        if selection.reveal_selected {
            self.events
                .extend(chosen.iter().map(|card| GameEvent::CardRevealed {
                    player,
                    card: card.id,
                    definition: card.definition,
                }));
        }
        if let Some(face_down) = selection.selected_face_down {
            // Manifested rather than placed: what goes down is a body, and
            // the card under it is what the mana cost to turn it up reads.
            for card in chosen {
                let _ = self.put_card_onto_battlefield_from(
                    card,
                    ZoneKind::Library,
                    super::BattlefieldArrival::face_down_under(player, face_down, true),
                    None,
                );
            }
        } else {
            self.place_revealed_remainder(
                player,
                chosen,
                selection.selected_zone,
                selection.selected_placement,
            );
        }
        self.place_revealed_remainder(player, rest, selection.rest_zone, selection.rest_placement);
    }

    pub(super) fn card_decision_options(
        &self,
        cards: &[CardInstance],
        zone: DecisionZone,
    ) -> Vec<DecisionOption> {
        cards
            .iter()
            .enumerate()
            .map(|(index, card)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self.catalog.get(card.definition).map_or_else(
                    || "Unknown card".into(),
                    |definition| definition.name.clone(),
                ),
                card: Some((
                    card.id,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                )),
                members: Vec::new(),
                ability_text: None,
                zone,
            })
            .collect()
    }

    pub(super) fn permanent_decision_options(
        &self,
        permanents: &[GameObjectId],
    ) -> Vec<DecisionOption> {
        permanents
            .iter()
            .enumerate()
            .filter_map(|(index, id)| {
                let permanent = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)?;
                let label = self
                    .effective_permanent_name(permanent)
                    .map_or_else(|| "Unknown permanent".into(), std::borrow::Cow::into_owned);
                Some(DecisionOption {
                    id: u32::try_from(index).unwrap_or(u32::MAX),
                    label,
                    card: Some((permanent.card.id, Self::effective_rules_source(permanent))),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                })
            })
            .collect()
    }

    pub(super) fn queue_two_pile_partition(
        &mut self,
        resolving_controller: PlayerId,
        divider: PlayerId,
        subject: PlayerId,
        prompt: impl Into<String>,
        items: Vec<DecisionOption>,
        on_complete: PilesSeparated,
    ) {
        if items.is_empty() {
            let mut runtime = CardRuntime { game: self };
            on_complete.run(
                &mut runtime,
                PileSplit {
                    resolving_controller,
                    subject,
                    first: Vec::new(),
                    second: Vec::new(),
                },
            );
            return;
        }
        self.queue_decision(
            divider,
            prompt,
            DecisionVisibility::Public,
            DecisionPreference::BalancedPartition,
            0..=items.len(),
            false,
            items.clone(),
            DecisionContinuation::SeparateIntoPiles {
                resolving_controller,
                subject,
                items,
                on_complete,
            },
        );
    }

    pub(super) fn queue_card_owned_pile_choice(
        &mut self,
        chooser: PlayerId,
        piles: PileSplit,
        prompt: impl Into<String>,
        option_prefix: &str,
        on_complete: PileChosen,
    ) {
        if piles.first.is_empty() && piles.second.is_empty() {
            let mut runtime = CardRuntime { game: self };
            on_complete.run(
                &mut runtime,
                PileChoice {
                    resolving_controller: piles.resolving_controller,
                    subject: piles.subject,
                    chosen: Vec::new(),
                    unchosen: Vec::new(),
                },
            );
            return;
        }
        let pile_label = |pile: &[DecisionOption]| {
            if pile.is_empty() {
                "Empty pile".to_owned()
            } else {
                pile.iter()
                    .map(|option| option.label.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        };
        let pile_members = |pile: &[DecisionOption]| {
            let mut members = Vec::new();
            for option in pile {
                if option.members.is_empty() {
                    members.extend(option.card);
                } else {
                    members.extend(option.members.iter().copied());
                }
            }
            members
        };
        let options = vec![
            DecisionOption {
                id: 0,
                label: format!("{option_prefix} 1: {}", pile_label(&piles.first)),
                card: None,
                members: pile_members(&piles.first),
                ability_text: None,
                zone: DecisionZone::None,
            },
            DecisionOption {
                id: 1,
                label: format!("{option_prefix} 2: {}", pile_label(&piles.second)),
                card: None,
                members: pile_members(&piles.second),
                ability_text: None,
                zone: DecisionZone::None,
            },
        ];
        self.queue_decision(
            chooser,
            prompt,
            DecisionVisibility::Public,
            DecisionPreference::LowerCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::ChoosePile { piles, on_complete },
        );
    }

    pub(super) fn queue_balance_task(
        &mut self,
        controller: PlayerId,
        phase: BalancePhase,
        task: BalanceTask,
        remaining: Vec<BalanceTask>,
    ) {
        let options = task
            .cards
            .iter()
            .enumerate()
            .map(|(index, (id, presentation))| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self
                    .presentation_name(*presentation)
                    .map_or_else(|| "Unknown object".to_owned(), std::borrow::Cow::into_owned),
                card: Some((*id, *presentation)),
                members: Vec::new(),
                ability_text: None,
                zone: task.zone,
            })
            .collect();
        self.queue_decision(
            task.player,
            task.prompt.clone(),
            if task.zone == DecisionZone::Hand {
                DecisionVisibility::Private
            } else {
                DecisionVisibility::Public
            },
            DecisionPreference::LowerCardValue,
            task.count..=task.count,
            false,
            options,
            DecisionContinuation::Balance {
                controller,
                phase,
                task,
                remaining,
            },
        );
    }

    /// Freezes every affected player's choice before any selected cards move.
    /// Applies a name chosen mid-resolution: bind every card of that name
    /// where the effect looks, then continue with the rest of it.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn resolve_card_name_choice(
        &mut self,
        choices: &[String],
        searched: PlayerId,
        zone: ZoneKind,
        binding: crate::ObjectSetBindingIndex,
        object: &StackObject,
        mut context: EffectResolutionContext,
        effect: ScopedEffect,
        options: &[u32],
    ) {
        let Some(name) = options
            .first()
            .and_then(|option| usize::try_from(*option).ok())
            .and_then(|index| choices.get(index))
            .cloned()
        else {
            return;
        };
        // Bound as the name is chosen: the rest of the effect names a set of
        // cards rather than a name it would have to match again.
        let matched = self.cards_named_in_zone(searched, zone, &name);
        context.bind_object_group(binding, matched);
        context.chosen_name = Some(name);
        self.resolve_nested_effect_before_later(effect, object, context);
    }

    pub(super) fn queue_effect_discards(
        &mut self,
        players: Vec<PlayerId>,
        amount: i32,
        cause: ZoneMoveCause,
    ) {
        self.queue_effect_discards_then(players, amount, cause, None);
    }

    /// The same, with something to do once the cards are gone. The follow-up
    /// reads how many of them matched, which is not knowable until the player
    /// has chosen.
    pub(super) fn queue_effect_discards_then(
        &mut self,
        mut players: Vec<PlayerId>,
        amount: i32,
        cause: ZoneMoveCause,
        follow_up: Option<DiscardFollowUp>,
    ) {
        let amount = usize::try_from(amount).unwrap_or(0);
        if amount == 0 || players.is_empty() {
            return;
        }
        players.sort_by_key(|player| (*player != self.active_player, player.index()));
        players.dedup();
        let first = players.remove(0);
        self.pending_discard_follow_up = follow_up;
        self.queue_next_effect_discard(first, amount, players, Vec::new(), cause);
    }

    pub(super) fn queue_next_effect_discard(
        &mut self,
        player: PlayerId,
        amount: usize,
        mut remaining: Vec<PlayerId>,
        mut chosen: Vec<(PlayerId, Vec<GameObjectId>)>,
        cause: ZoneMoveCause,
    ) {
        let hand = &self.players[player.index()].hand;
        let count = amount.min(hand.len());
        if count == 0 || count == hand.len() {
            chosen.push((player, hand.iter().map(|card| card.id).collect()));
            if remaining.is_empty() {
                self.complete_effect_discards(chosen, cause);
            } else {
                let next = remaining.remove(0);
                self.queue_next_effect_discard(next, amount, remaining, chosen, cause);
            }
            return;
        }
        let options = self.card_decision_options(hand, DecisionZone::Hand);
        self.queue_decision(
            player,
            format!("Choose {count} card(s) to discard"),
            DecisionVisibility::Private,
            DecisionPreference::LowerCardValue,
            count..=count,
            false,
            options,
            DecisionContinuation::DiscardForEffect {
                player,
                amount,
                remaining,
                chosen,
                cause,
            },
        );
    }

    pub(super) fn complete_effect_discards(
        &mut self,
        chosen: Vec<(PlayerId, Vec<GameObjectId>)>,
        cause: ZoneMoveCause,
    ) {
        // Counted before the cards move: "each land card discarded this way"
        // asks what went, and by the time they are in a graveyard they are
        // indistinguishable from what was already there.
        let follow_up = self.pending_discard_follow_up.take();
        let counted = follow_up.as_ref().map_or_else(Vec::new, |follow_up| {
            chosen
                .iter()
                .flat_map(|(player, cards)| cards.iter().map(move |card| (*player, *card)))
                .filter(|(player, card)| {
                    self.discarded_card_matches(follow_up.counted, *player, *card)
                })
                .collect::<Vec<_>>()
        });
        let matched = counted.len();
        // "For each card type among cards discarded this way" counts the
        // types, not the cards: one artifact creature is two and two
        // creatures are one.
        let matched_card_types = counted
            .iter()
            .filter_map(|(player, card)| self.discarded_card_types(*player, *card))
            .fold(
                crate::card::CardTypeSet::EMPTY,
                crate::card::CardTypeSet::union,
            )
            .count();
        for (player, cards) in chosen {
            self.discard_cards_with_cause(player, &cards, cause);
        }
        if let Some(follow_up) = follow_up {
            let mut context = follow_up.context;
            context.matched_count = u16::try_from(matched).ok();
            context.matched_card_types = Some(matched_card_types);
            let object = *follow_up.object;
            self.resolve_effect_def(follow_up.effect, &object, context);
        }
    }

    /// The card types a card about to be discarded has, read while it is
    /// still in hand for the same reason its match is.
    fn discarded_card_types(
        &self,
        player: PlayerId,
        card: GameObjectId,
    ) -> Option<crate::card::CardTypeSet> {
        let held = self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)?;
        Some(self.catalog.get(held.definition)?.rules.types())
    }

    /// Whether a card about to be discarded matches what a follow-up counts.
    fn discarded_card_matches(
        &self,
        predicate: ObjectPredicateDef,
        player: PlayerId,
        card: GameObjectId,
    ) -> bool {
        self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)
            .is_some_and(|held| self.card_object_matches(predicate, held, ZoneKind::Hand, held.id))
    }

    /// Whether a spell or ability an opponent of `player` controls can make
    /// them sacrifice a permanent. Sigarda says it cannot.
    pub(super) fn can_be_forced_to_sacrifice(&self, player: PlayerId, caused_by: PlayerId) -> bool {
        self.can_be_forced_to(player, caused_by, EffectDef::CannotBeForcedToSacrifice)
    }

    /// The same question about discarding. A player who discards as a cost of
    /// their own spell was not caused to by anybody, which is why the check
    /// is on who is causing it rather than on who is discarding.
    pub(super) fn can_be_forced_to_discard(&self, player: PlayerId, caused_by: PlayerId) -> bool {
        self.can_be_forced_to(player, caused_by, EffectDef::CannotBeForcedToDiscard)
    }

    fn can_be_forced_to(
        &self,
        player: PlayerId,
        caused_by: PlayerId,
        prohibition: EffectDef,
    ) -> bool {
        if caused_by == player {
            return true;
        }
        !self.battlefield.iter().any(|permanent| {
            permanent.controller == player
                && self
                    .find_effective_ability(permanent, |effective| {
                        effective.ability.is_executable()
                            && matches!(
                                effective.ability.definition,
                                DeclarativeAbilityDef::Static(_)
                            )
                            && effective
                                .ability
                                .declarative_effect()
                                .is_some_and(|effect| Self::names_prohibition(effect, prohibition))
                    })
                    .is_some()
        })
    }

    /// Whether a static clause states this prohibition, on its own or as one
    /// of several the same sentence states.
    fn names_prohibition(effect: EffectDef, prohibition: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(|effect| Self::names_prohibition(effect, prohibition)),
            other => other == prohibition,
        }
    }

    /// Whether a loyalty ability can be activated right now. CR 606.3: only
    /// during your own main phase with an empty stack, and only one loyalty
    /// ability per planeswalker per turn. CR 606.5: the cost cannot remove
    /// more counters than the permanent has.
    /// CR 602.5c as Pithing Needle writes it: a non-mana activated ability
    /// cannot be activated while something has named its source's card. The
    /// name is matched against the object's effective card part, so transformed
    /// faces and copies answer to the name whose abilities they currently
    /// present.
    pub(super) fn activated_abilities_are_named(&self, permanent: &Permanent) -> bool {
        let Some(name) = self.effective_permanent_name(permanent) else {
            return false;
        };
        self.battlefield.iter().any(|candidate| {
            candidate
                .chosen_card_name
                .as_deref()
                .is_some_and(|chosen| chosen == name.as_ref())
        })
    }

    pub(super) fn can_activate_loyalty(
        &self,
        permanent: &Permanent,
        player: PlayerId,
        change: i8,
    ) -> bool {
        if permanent.controller != player
            || permanent.activated_loyalty_this_turn
            || self.active_player != player
            || !matches!(self.step, Step::PrecombatMain | Step::PostcombatMain)
            || !self.stack.is_empty()
        {
            return false;
        }
        i32::from(permanent.counters(CounterKind::Loyalty)) + i32::from(change) >= 0
    }

    /// Turns a double-faced permanent over. The face is which part the
    /// permanent presents, so transforming is choosing the other one; the
    /// object itself does not change, which is why counters and damage stay.
    pub(super) fn transform_permanent(&mut self, id: GameObjectId) {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        let Some(other) = self.physical_other_face(&self.battlefield[index]) else {
            return;
        };
        self.battlefield[index].presented = other;
        let listeners = self.battlefield_trigger_listeners();
        let object = self.trigger_event_object(&self.battlefield[index]);
        self.capture_battlefield_triggers_from_snapshot(
            &listeners,
            &CommittedTriggerEvent::Transformed { object },
        );
    }

    pub(super) fn queue_chosen_sacrifice(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        followup: Option<SacrificeFollowup>,
        declined: Option<SacrificeDeclined>,
        optional: bool,
    ) {
        let candidates = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| {
                self.trigger_object_matches(
                    predicate,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                )
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        // An optional sacrifice is always a question, even with one candidate
        // or none: declining is a real answer. A compulsory one with a single
        // candidate has only one answer, so it happens without asking.
        if !optional && candidates.len() <= 1 {
            let sacrificed = candidates.first().copied();
            if let Some(followup) = followup {
                self.move_permanents_to_graveyard_then(
                    sacrificed.as_slice(),
                    Some(BattlefieldExitCompletion::SacrificeFollowup {
                        followup,
                        sacrificed,
                    }),
                );
            } else if let Some(sacrificed) = sacrificed {
                self.move_permanents_to_graveyard(&[sacrificed]);
            }
            return;
        }
        // Nothing to take is a declined offer, not a skipped one: "unless you
        // sacrifice an Island" bites hardest when there is no Island.
        if optional && candidates.is_empty() {
            self.resolve_sacrifice_declined(declined);
            return;
        }
        let options = self.permanent_decision_options(&candidates);
        self.queue_decision(
            player,
            if optional {
                "You may sacrifice a permanent"
            } else {
                "Choose a permanent to sacrifice"
            },
            DecisionVisibility::Public,
            DecisionPreference::LowerCardValue,
            usize::from(!optional)..=1,
            false,
            options,
            DecisionContinuation::SacrificeOfChoice {
                followup,
                declined,
                optional,
            },
        );
    }

    /// Runs the branch an optional sacrifice owes when it was declined.
    pub(super) fn resolve_sacrifice_declined(&mut self, declined: Option<SacrificeDeclined>) {
        let Some(declined) = declined else {
            return;
        };
        self.resolve_effect_def(declined.effect, &declined.object, declined.context);
    }

    /// Runs what a sacrifice owes once the permanent is chosen. The
    /// characteristic is last-known, because by the time this runs the
    /// permanent is already gone.
    pub(super) fn resolve_sacrifice_followup(
        &mut self,
        followup: &SacrificeFollowup,
        sacrificed: Option<GameObjectId>,
    ) {
        // A negative value gives nothing rather than draining the
        // controller.
        let amount = i32::from(
            sacrificed
                .and_then(|id| match followup.amount {
                    SacrificedAmountDef::Power => self.current_or_last_known_power(id),
                    SacrificedAmountDef::Toughness => self.current_or_last_known_toughness(id),
                })
                .unwrap_or(0),
        )
        .max(0);
        let mut context = followup.context.clone();
        context.trigger.amount = Some(amount);
        self.resolve_effect_def(followup.effect, &followup.object, context);
    }
}
