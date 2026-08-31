use std::ops::ControlFlow;

use super::{
    AppliedRuleDef, BalancePhase, BalanceTask, BattlefieldExitCompletion, CardInstance, CardPartId,
    CommittedTriggerEvent, CounterKind, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, DeclarativeAbilityDef, DiscardFollowUp, EffectDef,
    EffectResolutionContext, Game, GameObjectId, ObjectCharacteristics, ObjectPredicateDef,
    Permanent, PlayerId, SacrificeDeclined, SacrificeFollowup, SacrificedAmountDef, ScopedEffect,
    StackObject, Step, ZoneKind, ZoneMoveCause, ZonePlacement,
};

impl Game {
    /// The player-facing direction for one library arrangement.
    pub(super) const fn library_order_prompt(placement: ZonePlacement) -> &'static str {
        match placement {
            ZonePlacement::Top => "Order cards for your library, naming the top card first",
            ZonePlacement::Bottom => "Order cards for your library, naming the bottom card first",
            ZonePlacement::FromTop(_) => {
                "Order cards for the same library position, naming the upper card first"
            }
        }
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
            self.complete_effect_discards(Vec::new(), cause, follow_up);
            return;
        }
        players.sort_by_key(|player| (*player != self.active_player, player.index()));
        players.dedup();
        let first = players.remove(0);
        self.queue_next_effect_discard(first, amount, players, Vec::new(), cause, follow_up);
    }

    pub(super) fn queue_next_effect_discard(
        &mut self,
        player: PlayerId,
        amount: usize,
        mut remaining: Vec<PlayerId>,
        mut chosen: Vec<(PlayerId, Vec<GameObjectId>)>,
        cause: ZoneMoveCause,
        follow_up: Option<DiscardFollowUp>,
    ) {
        let hand = &self.players[player.index()].hand;
        let count = amount.min(hand.len());
        if count == 0 || count == hand.len() {
            chosen.push((player, hand.iter().map(|card| card.id).collect()));
            if remaining.is_empty() {
                self.complete_effect_discards(chosen, cause, follow_up);
            } else {
                let next = remaining.remove(0);
                self.queue_next_effect_discard(next, amount, remaining, chosen, cause, follow_up);
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
                follow_up: follow_up.map(Box::new),
            },
        );
    }

    pub(super) fn complete_effect_discards(
        &mut self,
        chosen: Vec<(PlayerId, Vec<GameObjectId>)>,
        cause: ZoneMoveCause,
        follow_up: Option<DiscardFollowUp>,
    ) {
        // Counted before the cards move: "each land card discarded this way"
        // asks what went, and by the time they are in a graveyard they are
        // indistinguishable from what was already there.
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
            // A discarded card is a new object in its graveyard, so what the
            // follow-up is handed is the successor rather than the identity
            // the card had in hand.
            if let Some(binding) = follow_up.bound {
                let discarded = counted
                    .iter()
                    .filter_map(|(_, card)| self.successors.get(card).copied())
                    .map(crate::Target::Card)
                    .collect();
                context.bind_object_group(binding, discarded);
            }
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
    /// Whether a permission on this planeswalker opens its loyalty abilities
    /// to instant speed.
    fn loyalty_may_be_activated_any_time(&self, permanent: &Permanent) -> bool {
        self.visit_applied_rules(permanent, |applied| {
            if applied.rule == AppliedRuleDef::MayActivateLoyaltyAnyTime {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    pub(super) fn can_activate_loyalty(
        &self,
        permanent: &Permanent,
        player: PlayerId,
        change: i8,
    ) -> bool {
        if permanent.controller != player || permanent.activated_loyalty_this_turn {
            return false;
        }
        // CR 606.3: sorcery speed, unless the planeswalker itself says
        // otherwise. The one-per-turn limit above is not part of the window
        // and stands either way.
        if !self.loyalty_may_be_activated_any_time(permanent)
            && (self.active_player != player
                || !matches!(self.step, Step::PrecombatMain | Step::PostcombatMain)
                || !self.stack.is_empty())
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

    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_chosen_sacrifice(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        // How many are given up. A player with fewer than this gives up
        // everything they have.
        count: usize,
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
        // or none: declining is a real answer. A compulsory one with no more
        // candidates than it asks for has only one answer, so it happens
        // without asking -- which is also how a player holding two creatures
        // "sacrifices three".
        if !optional && candidates.len() <= count {
            let sacrificed = candidates.first().copied();
            self.capture_sacrifices(&candidates);
            if let Some(followup) = followup {
                self.move_permanents_to_graveyard_then(
                    &candidates,
                    Some(BattlefieldExitCompletion::SacrificeFollowup {
                        followup,
                        sacrificed,
                    }),
                );
            } else if !candidates.is_empty() {
                self.move_permanents_to_graveyard(&candidates);
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
            } else if count == 1 {
                "Choose a permanent to sacrifice"
            } else {
                "Choose permanents to sacrifice"
            },
            DecisionVisibility::Public,
            DecisionPreference::LowerCardValue,
            if optional { 0 } else { count }..=count,
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
