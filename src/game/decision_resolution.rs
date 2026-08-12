use super::{
    BalanceAction, BasicLandType, BasicLandTypeChange, CardRuntime, CounterKind,
    DecisionContinuation, DecisionOption, FORK_COPY_COLOR, Game, ManaCost,
    PendingReplacementEffect, PileChoice, PileSplit, PlayerId, ReplaceableEvent, Target,
    TargetSelection, TargetSlotId, ZoneKind, ZoneMoveCause, remove_card,
};

impl Game {
    #[allow(clippy::too_many_lines)]
    pub(super) fn choose_decision(&mut self, player: PlayerId, decision: u32, options: &[u32]) {
        let pending = self.pending_decisions.remove(0);
        debug_assert_eq!(pending.observation.id, decision);
        match pending.continuation {
            DecisionContinuation::DiscardForEffect {
                player,
                amount,
                mut remaining,
                mut chosen,
                cause,
            } => {
                let selected = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                chosen.push((player, selected));
                if remaining.is_empty() {
                    self.complete_effect_discards(chosen, cause);
                } else {
                    let next = remaining.remove(0);
                    self.queue_next_effect_discard(next, amount, remaining, chosen, cause);
                }
            }
            DecisionContinuation::BasicLandTypeTextChange { target } => {
                let Some(option) = options.first().copied() else {
                    return;
                };
                let width = u32::try_from(BasicLandType::ALL.len())
                    .expect("the basic-land-type count fits u32");
                let Some(from) = usize::try_from(option / width)
                    .ok()
                    .and_then(BasicLandType::from_index)
                else {
                    return;
                };
                let Some(to) = usize::try_from(option % width)
                    .ok()
                    .and_then(BasicLandType::from_index)
                else {
                    return;
                };
                if from == to {
                    return;
                }
                let change = BasicLandTypeChange { from, to };
                match target {
                    Target::Permanent(id) => {
                        if let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                        {
                            permanent.text_changes.push(change);
                        }
                    }
                    Target::Spell(id) => {
                        if let Some(index) = self.stack.iter().position(|spell| spell.id == id) {
                            self.stack[index].text_changes.push(change);
                        }
                    }
                    Target::Player(_) | Target::Card(_) => {}
                }
            }
            DecisionContinuation::ExileFromHand { victim } => {
                let Some((card, _)) = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                else {
                    return;
                };
                if let Some(card) = remove_card(&mut self.players[victim.index()].hand, card) {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[victim.index()].exile.push(card);
                }
            }
            DecisionContinuation::AugurOfBolas { player, revealed } => {
                let kept = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                    .map(|(card, _)| card);
                let (to_hand, to_bottom): (Vec<_>, Vec<_>) =
                    revealed.into_iter().partition(|card| Some(card.id) == kept);
                for card in to_hand {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                // "In any order" -- printed order is as good as any, and the
                // rest of the library is already unknown to everyone.
                for card in to_bottom {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].library.push(card);
                }
            }
            DecisionContinuation::BattlefieldEntryReplacement { candidates } => {
                let selected = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| candidates.get(index))
                    .copied();
                if let (Some(mut pending), Some(selected)) =
                    (self.pending_events.pop_front(), selected)
                {
                    pending.applied.push(selected.context.source);
                    pending.effects.push(PendingReplacementEffect {
                        context: selected.context,
                        effect: selected.effect,
                    });
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::BattlefieldEntryPayment {
                context,
                payment,
                if_paid,
                if_declined,
            } => {
                if let Some(mut pending) = self.pending_events.pop_front() {
                    let payment_player = self.pending_payment_player(&pending, context, payment);
                    let paid = options.contains(&1)
                        && payment_player.is_some_and(|player| self.pay_payment(player, payment));
                    Self::push_replacement_effects(
                        &mut pending,
                        context,
                        if paid { if_paid } else { if_declined },
                    );
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::BattlefieldEntryCopy {
                choices,
                added_types,
            } => {
                let copied = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .filter(|option| *option > 0)
                    .and_then(|option| choices.get(option - 1).copied())
                    .and_then(|id| {
                        self.battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == id)
                    })
                    .map(|permanent| {
                        let mut copy = Self::copiable_characteristics(permanent);
                        copy.added_types = copy.added_types.union(added_types);
                        copy
                    });
                if let Some(mut pending) = self.pending_events.pop_front() {
                    if let Some(copy) = copied {
                        let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
                        entry.permanent.copied_from = Some(copy.base);
                        entry.permanent.copy_effect = Some(copy);
                    }
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::BattlefieldEntryCardName { choices } => {
                let Some(selected) = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| choices.get(index))
                    .cloned()
                else {
                    return;
                };
                if let Some(mut pending) = self.pending_events.pop_front() {
                    let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
                    entry.permanent.chosen_card_name = Some(selected);
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::BattlefieldEntryCreatureType { choices } => {
                let Some(selected) = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| choices.get(index))
                    .cloned()
                else {
                    return;
                };
                if let Some(mut pending) = self.pending_events.pop_front() {
                    let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
                    entry.permanent.chosen_creature_type = Some(selected);
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::OptionalManaPayment {
                player,
                cost,
                object,
                context,
                effect,
            } => {
                if options.contains(&1) {
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                    self.resolve_effect_def(effect, &object, context);
                }
            }
            DecisionContinuation::ManaPaymentOrElse {
                player,
                cost,
                object,
                context,
                effect,
            } => {
                if options.contains(&1) {
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                } else {
                    self.resolve_effect_def(effect, &object, context);
                }
            }
            DecisionContinuation::ChainLightning {
                player,
                spell,
                targets,
            } => {
                if let Some(option) = options.first().copied()
                    && option > 0
                    && let Some(target) = targets.get(usize::try_from(option - 1).unwrap_or(0))
                {
                    let cost = ManaCost::new(0, 2);
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                    let replacements = spell
                        .signature
                        .as_ref()
                        .and_then(|signature| signature.targets().first())
                        .map(|selection| vec![TargetSelection::single(selection.slot(), *target)])
                        .unwrap_or_default();
                    self.push_copy(spell, player, replacements);
                }
            }
            DecisionContinuation::Fork {
                player,
                spell,
                target_lists,
            } => {
                if let Some(option) = options.first().copied()
                    && let Some(targets) = target_lists.get(usize::try_from(option).unwrap_or(0))
                {
                    self.push_copy_with_colors(
                        spell,
                        player,
                        targets.clone(),
                        Some(FORK_COPY_COLOR),
                    );
                }
            }
            DecisionContinuation::GrislySalvage { player, revealed } => {
                let kept = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                    .map(|(card, _)| card);
                let (to_hand, to_graveyard): (Vec<_>, Vec<_>) =
                    revealed.into_iter().partition(|card| Some(card.id) == kept);
                for card in to_hand {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                self.bury_cards(player, to_graveyard);
            }
            DecisionContinuation::CounterUnlessPaid {
                spell,
                player,
                cost,
                zone,
            } => {
                if options.contains(&1) {
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                } else {
                    self.counter_spell_into(spell, zone);
                }
            }
            DecisionContinuation::OptionalEffect {
                object,
                context,
                effect,
            } => {
                if options.contains(&1) {
                    self.resolve_effect_def(effect, &object, context);
                }
            }
            DecisionContinuation::MiracleReveal { card } => {
                if options.contains(&1) {
                    self.miracle_window = Some(card);
                }
            }
            DecisionContinuation::PileSplit { owner } => {
                let first = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                let second = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| !options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                self.queue_pile_choice(owner, first, second);
            }
            DecisionContinuation::RevealedPileSplit {
                player,
                revealed,
                rest,
                placement,
            } => {
                let chosen = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                let (first, second): (Vec<_>, Vec<_>) = revealed
                    .into_iter()
                    .partition(|card| chosen.contains(&card.id));
                self.queue_revealed_pile_choice(player, first, second, rest, placement);
            }
            DecisionContinuation::RevealedPileChoice {
                player,
                first,
                second,
                rest,
                placement,
            } => {
                let (to_hand, to_rest) = if options.contains(&0) {
                    (first, second)
                } else {
                    (second, first)
                };
                for card in to_hand {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                self.place_revealed_remainder(player, to_rest, rest, placement);
            }
            DecisionContinuation::PileChoice { first, second } => {
                let chosen = if options.contains(&0) { first } else { second };
                self.move_permanents_to_graveyard(&chosen);
            }
            DecisionContinuation::DestroyOfChoice { can_regenerate } => {
                let doomed = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                self.destroy_permanents(&doomed, can_regenerate);
            }
            DecisionContinuation::SeparateIntoPiles {
                resolving_controller,
                subject,
                items,
                on_complete,
            } => {
                let first = items
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .cloned()
                    .collect();
                let second = items
                    .iter()
                    .filter(|option| !options.contains(&option.id))
                    .cloned()
                    .collect();
                let mut runtime = CardRuntime { game: self };
                on_complete(
                    &mut runtime,
                    PileSplit {
                        resolving_controller,
                        subject,
                        first,
                        second,
                    },
                );
            }
            DecisionContinuation::ChoosePile { piles, on_complete } => {
                let chosen = if options.contains(&0) {
                    &piles.first
                } else {
                    &piles.second
                };
                let unchosen = if options.contains(&0) {
                    &piles.second
                } else {
                    &piles.first
                };
                let object_ids = |pile: &[DecisionOption]| {
                    pile.iter()
                        .flat_map(|option| {
                            if option.members.is_empty() {
                                option.card.into_iter().collect::<Vec<_>>()
                            } else {
                                option.members.clone()
                            }
                        })
                        .map(|(id, _)| id)
                        .collect::<Vec<_>>()
                };
                let mut runtime = CardRuntime { game: self };
                on_complete(
                    &mut runtime,
                    PileChoice {
                        resolving_controller: piles.resolving_controller,
                        subject: piles.subject,
                        chosen: object_ids(chosen),
                        unchosen: object_ids(unchosen),
                    },
                );
            }
            DecisionContinuation::SacrificeOfChoice { followup, optional } => {
                let sacrificed = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                let chosen = sacrificed.first().copied();
                self.move_permanents_to_graveyard(&sacrificed);
                // "If a player does" -- declining an optional sacrifice earns
                // nothing, while a compulsory one pays out even for nothing.
                if let Some(followup) = followup
                    && (chosen.is_some() || !optional)
                {
                    self.resolve_sacrifice_followup(&followup, chosen);
                }
            }
            DecisionContinuation::Duress { victim, cause } => {
                let Some(option) = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                else {
                    return;
                };
                let Some((card, _)) = option.card else {
                    return;
                };
                self.discard_cards_with_cause(victim, &[card], cause);
            }
            DecisionContinuation::LibrarySearch {
                destination,
                shuffle,
            } => {
                let found = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card);
                if let Some((card, _)) = found
                    && let Some(card) = remove_card(&mut self.players[player.index()].library, card)
                {
                    if destination == ZoneKind::Battlefield {
                        self.put_card_onto_battlefield_from(card, ZoneKind::Library, player, None);
                    } else {
                        let (card, _zone_change) = self.zone_change_card(card);
                        self.players[player.index()].hand.push(card);
                    }
                }
                if shuffle {
                    self.rng.shuffle(&mut self.players[player.index()].library);
                }
            }
            DecisionContinuation::Tutor => {
                let found = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card);
                if let Some((card, _)) = found
                    && let Some(card) = remove_card(&mut self.players[player.index()].library, card)
                {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                // The card says "then shuffle", and a search that finds
                // nothing still searched. Skipping this would hand a player
                // their library order for free: tutor, fail to find, and the
                // top of the deck is whatever it already was.
                self.rng.shuffle(&mut self.players[player.index()].library);
            }
            DecisionContinuation::RecallDiscard { player } => {
                let discarded = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                let count = discarded.len();
                self.discard_cards_with_cause(
                    player,
                    &discarded,
                    ZoneMoveCause::Effect { controller: player },
                );
                // "for each card discarded this way" -- and those cards are in
                // the graveyard now, so Recall can hand any of them straight
                // back.
                self.queue_recall_return(player, count);
            }
            DecisionContinuation::RecallReturn { player } => {
                for option in &pending.observation.options {
                    if options.contains(&option.id)
                        && let Some((card, _)) = option.card
                        && let Some(card) =
                            remove_card(&mut self.players[player.index()].graveyard, card)
                    {
                        let (card, _zone_change) = self.zone_change_card(card);
                        self.players[player.index()].hand.push(card);
                    }
                }
            }
            DecisionContinuation::Balance {
                controller,
                phase,
                task,
                mut remaining,
            } => {
                let mut discards = Vec::new();
                let mut sacrifices = Vec::new();
                for option in &pending.observation.options {
                    if !options.contains(&option.id) {
                        continue;
                    }
                    let Some((card, _)) = option.card else {
                        continue;
                    };
                    match task.action {
                        BalanceAction::Sacrifice => sacrifices.push(card),
                        BalanceAction::Discard => discards.push(card),
                    }
                }
                self.move_permanents_to_graveyard(&sacrifices);
                self.discard_cards_with_cause(task.player, &discards, task.cause);
                if !remaining.is_empty() {
                    let next = remaining.remove(0);
                    self.queue_balance_task(controller, phase, next, remaining);
                } else if let Some(next) = phase.next() {
                    self.queue_balance_phase(controller, next);
                }
            }
            DecisionContinuation::TimeVault {
                permanent,
                mut remaining,
            } => {
                if options.contains(&1) {
                    if let Some(vault) = self
                        .battlefield
                        .iter_mut()
                        .find(|candidate| candidate.card.id == permanent)
                    {
                        vault.tapped = false;
                    }
                    self.skipped_turns[player.index()] =
                        self.skipped_turns[player.index()].saturating_add(1);
                }
                if remaining.is_empty() {
                    self.handle_upkeep_triggers();
                } else {
                    let next = remaining.remove(0);
                    self.queue_time_vault_decision(next, remaining);
                }
            }
            DecisionContinuation::SylvanOffer { player } => {
                if !options.contains(&1) {
                    return;
                }
                self.draw_cards(player, 2);
                let candidates = self.sylvan_candidates(player);
                // Two cards, or every card drawn this turn if fewer remain.
                let choices = candidates.len().min(2);
                if choices > 0 {
                    self.queue_sylvan_select(player, candidates, choices);
                }
            }
            DecisionContinuation::SylvanSelect {
                player,
                mut candidates,
                choices_left,
            } => {
                let selected = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                    .map(|(card, _)| card);
                if let Some(card) = selected {
                    candidates.retain(|candidate| *candidate != card);
                    self.queue_sylvan_mode(player, card, candidates, choices_left);
                }
            }
            DecisionContinuation::SylvanMode {
                player,
                card,
                candidates,
                choices_left,
            } => {
                if options.contains(&1) {
                    self.players[player.index()].life -= 4;
                } else if let Some(card) = remove_card(&mut self.players[player.index()].hand, card)
                {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].library.push(card);
                }
                if choices_left > 1 && self.result.is_none() {
                    self.queue_sylvan_select(player, candidates, choices_left - 1);
                }
            }
            DecisionContinuation::TetravusDetach { source } => {
                // The counters have to still be there: two upkeep triggers
                // resolve one after the other, and the assemble trigger can
                // run first.
                let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == source)
                else {
                    return;
                };
                let removed = options
                    .len()
                    .min(usize::from(permanent.counters(CounterKind::PlusOnePlusOne)));
                let Ok(removed) = u16::try_from(removed) else {
                    return;
                };
                if removed == 0 {
                    return;
                }
                permanent.remove_counters(CounterKind::PlusOnePlusOne, removed);
                let controller = permanent.controller;
                for _ in 0..removed {
                    self.create_token_from(
                        controller,
                        crate::card::cards::TETRAVITE_TOKEN,
                        Some(source),
                    );
                }
            }
            DecisionContinuation::TetravusAssemble { source } => {
                let exiled = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card)
                    .map(|(card, _)| card)
                    .collect::<Vec<_>>();
                let mut returned: u16 = 0;
                for token in exiled {
                    // Only tokens this Tetravus still owns count, in case one
                    // changed hands or left between the offer and the answer.
                    if self.battlefield.iter().any(|permanent| {
                        permanent.card.id == token && permanent.created_by == Some(source)
                    }) {
                        self.exile_permanent(token);
                        returned = returned.saturating_add(1);
                    }
                }
                if returned == 0 {
                    return;
                }
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == source)
                {
                    permanent.add_counters(CounterKind::PlusOnePlusOne, returned);
                }
            }
            DecisionContinuation::TriggerOrder { batch, remaining } => {
                self.complete_trigger_order(&batch, remaining, options);
            }
            DecisionContinuation::TriggerPlacement {
                mut trigger,
                pending,
                remaining,
                candidates,
            } => {
                let target_index = trigger.targets.len();
                let selected = options
                    .iter()
                    .filter_map(|option| {
                        usize::try_from(*option)
                            .ok()
                            .and_then(|index| candidates.get(index))
                            .copied()
                    })
                    .collect();
                let slot = TargetSlotId::from_index(target_index)
                    .expect("validated trigger targets fit the runtime slot space");
                trigger.targets.push(TargetSelection::new(slot, selected));
                let mut continued = vec![trigger];
                continued.extend(pending);
                self.place_trigger_sequence(continued, remaining);
            }
        }
    }

    pub(super) fn cancel_decision(&mut self, decision: u32) {
        debug_assert_eq!(self.pending_decisions[0].observation.id, decision);
        self.pending_decisions.remove(0);
    }
}
