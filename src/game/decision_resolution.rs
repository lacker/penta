mod battlefield_entry;
mod chosen_cards;
mod leaving;
mod triggers;

use super::decision_search_resolution::SearchResolution;
use super::{
    BalanceAction, BasicLandType, BasicLandTypeChange, BattlefieldArrival,
    BattlefieldExitCompletion, CardRuntime, CounterKind, DecisionContinuation, DecisionOption,
    Game, GameEvent, ManaCost, PileChoice, PileSplit, PlayerId, ReplaceableEvent, Target,
    TargetSelection, ZoneKind, ZoneMoveCause, ZonePlacement, remove_card,
};
use crate::card::{BattlefieldEntryChoiceDestinationDef, ReplacementEffectDef};

impl Game {
    #[allow(clippy::too_many_lines)]
    pub(super) fn choose_decision(&mut self, player: PlayerId, decision: u32, options: &[u32]) {
        let pending = self.pending_decisions.remove(0);
        debug_assert_eq!(pending.observation.id, decision);
        // Kept beside the answer because a payment decision that offered one
        // option per candidate has to look up which card the chosen option
        // named.
        let pending_options = pending.observation.options.clone();
        match pending.continuation {
            DecisionContinuation::BeginTurn {
                player,
                kind,
                applied,
                replacements,
                deferred,
            } => {
                if let Some(option) = options.first().copied() {
                    self.choose_begin_turn(player, kind, applied, &replacements, deferred, option);
                }
            }
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
            DecisionContinuation::CardNameChoice {
                choices,
                searched,
                zone,
                binding,
                object,
                context,
                effect,
            } => self.resolve_card_name_choice(
                &choices, searched, zone, binding, &object, context, effect, options,
            ),
            DecisionContinuation::SimultaneousChoose {
                definition,
                task,
                players,
                mut chosen,
                object,
                context,
                candidates,
            } => {
                chosen.extend(
                    pending
                        .observation
                        .options
                        .iter()
                        .filter(|option| options.contains(&option.id))
                        .filter_map(|option| option.card.map(|(card, _)| card))
                        .filter(|card| candidates.contains(card)),
                );
                self.queue_next_simultaneous_choice(
                    definition,
                    task + 1,
                    players,
                    chosen,
                    &object,
                    context,
                    true,
                );
            }
            DecisionContinuation::ChosenColorMana {
                controller,
                mut prototype,
                remaining,
                choosable,
            } => {
                let colors = Self::chosen_mana_colors(choosable);
                let Some(color) = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| colors.get(index).copied())
                else {
                    return;
                };
                prototype.color = color;
                self.add_mana(controller, std::iter::once(prototype));
                self.queue_chosen_color_mana(
                    controller,
                    prototype,
                    remaining.saturating_sub(1),
                    choosable,
                );
            }
            DecisionContinuation::ChooseColor {
                object,
                context,
                scoped,
                targets,
                operation,
                duration,
            } => {
                let Some(index) = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .filter(|index| *index < Self::choosable_qualities(operation).len())
                else {
                    return;
                };
                self.apply_chosen_color(
                    &object, &context, scoped, &targets, operation, duration, index,
                );
            }
            DecisionContinuation::BasicLandTypeSubstitution {
                object,
                context,
                effect,
            } => {
                if let Some(option) = options.first().copied() {
                    self.resolve_basic_land_type_substitution(&object, &context, effect, option);
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
            DecisionContinuation::TopCardSelection {
                player,
                revealed,
                selection,
                object,
                context,
                effect,
            } => {
                // Walked in the order the cards were named rather than the
                // order they were offered, because an arrangement reads that
                // sequence back below. Membership is all the ordinary path
                // asks of it, so nothing else notices.
                let selected = options
                    .iter()
                    .filter_map(|chosen| {
                        pending
                            .observation
                            .options
                            .iter()
                            .find(|option| option.id == *chosen)
                    })
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                let (mut chosen, rest): (Vec<_>, Vec<_>) = revealed
                    .into_iter()
                    .partition(|card| selected.contains(&card.id));
                // "Put them back in any order": the arrangement is the order
                // the cards were named, so it has to survive the partition,
                // which otherwise reports them in library order.
                if selection.selected_order_follows_choice {
                    chosen.sort_by_key(|card| {
                        selected
                            .iter()
                            .position(|id| *id == card.id)
                            .unwrap_or(usize::MAX)
                    });
                }
                let hider = object.source.unwrap_or(object.id);
                let matched = self.selected_card_totals(&chosen, selection.counted, hider);
                self.finish_top_card_selection_from(player, chosen, rest, selection, Some(hider));
                if let Some(then) = selection.then {
                    let mut context = context;
                    context.matched_count = Some(matched.0);
                    context.matched_mana_value = Some(matched.1);
                    self.resolve_nested_effect_before_later(
                        effect.with_effect(*then),
                        &object,
                        context,
                    );
                }
            }
            continuation @ (DecisionContinuation::BattlefieldEntryReplacement { .. }
            | DecisionContinuation::BattlefieldEntryExile { .. }
            | DecisionContinuation::BattlefieldEntryOptional { .. }
            | DecisionContinuation::BattlefieldExitReplacement { .. }
            | DecisionContinuation::BattlefieldEntryPayment { .. }
            | DecisionContinuation::BattlefieldEntryCopy { .. }
            | DecisionContinuation::BattlefieldEntryScalarChoice { .. }) => {
                self.resolve_battlefield_entry_decision(continuation, &pending_options, options);
            }
            DecisionContinuation::PayOr {
                player,
                payment,
                definition: _,
                object,
                context,
                if_paid,
                otherwise,
            } => {
                // Creatures are named one at a time, so choosing to pay opens
                // its own run of decisions rather than settling here.
                if let super::ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total) =
                    payment
                {
                    if options.iter().copied().any(|option| option != 0) {
                        self.queue_total_power_sacrifice(
                            player,
                            i32::from(total),
                            &object,
                            context,
                            if_paid,
                        );
                    } else if let Some(effect) = otherwise {
                        self.resolve_nested_effect_before_later(effect, &object, context);
                    }
                    return;
                }
                let paid = self.settle_payment_decision(player, payment, options, &pending_options);
                let branch = if paid.is_some() { if_paid } else { otherwise };
                if let Some(effect) = branch {
                    // "If you do, create X ...": the branch reads back what
                    // the payment actually cost.
                    let mut context = context;
                    context.paid_amount = paid;
                    self.resolve_nested_effect_before_later(effect, &object, context);
                }
            }
            DecisionContinuation::ActivationCostSacrifice {
                player,
                quota,
                pending,
                chosen,
            } => {
                // The option ids are positions in the candidate list the
                // offer was built from, which is rebuilt the same way.
                let answer = options
                    .first()
                    .copied()
                    .and_then(|option| usize::try_from(option).ok());
                self.continue_activation_sacrifice(player, quota, *pending, chosen, answer);
            }
            DecisionContinuation::ActivationCostTap {
                player,
                remaining,
                pending,
                chosen,
            } => {
                let answer = options.first().copied();
                self.continue_activation_saddle(player, remaining, *pending, chosen, answer);
            }
            DecisionContinuation::SacrificeToTotalPower {
                player,
                remaining,
                object,
                context,
                if_paid,
            } => {
                let chosen = options
                    .iter()
                    .copied()
                    .find(|option| *option != 0)
                    .and_then(|chosen| {
                        pending_options
                            .iter()
                            .find(|option| option.id == chosen)
                            .and_then(|option| option.card)
                    })
                    .map(|(permanent, _)| permanent);
                self.continue_total_power_sacrifice(
                    player, remaining, chosen, &object, context, if_paid,
                );
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
                colors,
                remaining,
                player,
                spell,
                target_lists,
            } => {
                if let Some(option) = options.first().copied()
                    && let Some(targets) = target_lists.get(usize::try_from(option).unwrap_or(0))
                {
                    self.push_copy_with_colors(spell.clone(), player, targets.clone(), colors);
                }
                if remaining > 0 {
                    self.queue_copy_decision_chain(player, spell, colors, "the copy", remaining);
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
            DecisionContinuation::ExploredCardPlacement { player, revealed } => {
                self.place_explored_card(player, revealed, options.contains(&1));
            }
            DecisionContinuation::Proliferate { candidates } => {
                let chosen = pending
                    .observation
                    .options
                    .iter()
                    .enumerate()
                    .filter(|(_, option)| options.contains(&option.id))
                    .filter_map(|(index, _)| candidates.get(index).copied())
                    .collect::<Vec<_>>();
                self.proliferate(&chosen);
            }
            DecisionContinuation::MayCastGranted {
                card,
                ability,
                grant,
                ..
            } => {
                // Answering this decision is the decline: a cast would have
                // taken the decision away instead of resolving it. Either
                // way the lent ability goes back.
                self.revoke_temporary_grant(grant, card, &ability);
            }
            DecisionContinuation::MayCastExiled {
                card,
                object,
                context,
                definition,
                ..
            } => {
                // Answering this decision is the decline: a cast would have
                // taken the decision away instead of resolving it.
                self.consume_exile_play_permission(card);
                self.resolve_declined_cast(&object, context, definition);
            }
            DecisionContinuation::MayCastAlternative { .. } => {
                // Answering the standing decision is the decline. Its
                // permission is the decision itself, so removing it is all
                // that declining has to do.
            }
            DecisionContinuation::CascadeCast {
                player,
                card,
                exiled,
            } => {
                // Answering this decision is the decline: a cast would have
                // taken the decision away instead of resolving it. Either
                // way the pile goes home.
                self.consume_exile_play_permission(card);
                self.bury_cascade_exiles(player, &exiled);
            }
            DecisionContinuation::SpellLibraryEnd { spell, .. } => {
                let placement = if options.contains(&1) {
                    ZonePlacement::Bottom
                } else {
                    ZonePlacement::Top
                };
                self.put_spell_into_library(spell, placement);
            }
            DecisionContinuation::Endure {
                player,
                permanent,
                amount,
            } => self.finish_endure(player, permanent, amount, options),
            DecisionContinuation::OptionalEffect {
                object,
                context,
                effect,
            } => {
                if options.contains(&1) {
                    self.resolve_effect_def(effect, &object, context);
                }
            }
            DecisionContinuation::ChooseForEffect {
                definition,
                binding,
                object,
                mut context,
                candidates,
                effect,
            } => {
                let ordered = matches!(
                    binding,
                    crate::card::ObjectChoiceBindingDef::OrderedObjects(_)
                );
                let selected_ids = if ordered {
                    options.to_vec()
                } else {
                    pending
                        .observation
                        .options
                        .iter()
                        .filter(|option| options.contains(&option.id))
                        .map(|option| option.id)
                        .collect()
                };
                let selected = selected_ids
                    .iter()
                    .filter_map(|option| usize::try_from(*option).ok())
                    .filter_map(|index| candidates.get(index))
                    .copied()
                    .collect::<Vec<_>>();
                // "The rest" is whatever was offered and not taken, bound
                // before the chosen half so the two are one partition of the
                // same candidates.
                if let crate::card::EffectDef::Choose(choice) = definition.effect
                    && let Some(unchosen) = choice.unchosen
                {
                    let rest = candidates
                        .iter()
                        .filter(|candidate| !selected.contains(candidate))
                        .copied()
                        .collect();
                    context.bind_object_group(unchosen, rest);
                }
                Self::bind_effect_choice(&mut context, binding, selected);
                self.resolve_nested_effect_before_later(effect, &object, context);
            }
            DecisionContinuation::SplitForEffect {
                definition,
                chooser,
                items,
                object,
                context,
            } => {
                let (first, second) = items.into_iter().enumerate().fold(
                    (Vec::new(), Vec::new()),
                    |(mut first, mut second), (index, item)| {
                        if u32::try_from(index).is_ok_and(|id| options.contains(&id)) {
                            first.push(item);
                        } else {
                            second.push(item);
                        }
                        (first, second)
                    },
                );
                self.queue_effect_pile_choice(chooser, first, second, object, context, definition);
            }
            DecisionContinuation::ChoosePileForEffect {
                definition: _,
                first,
                second,
                chosen,
                unchosen,
                object,
                mut context,
                effect,
            } => {
                let choose_first = options.first().copied() == Some(0);
                let (chosen_objects, unchosen_objects) = if choose_first {
                    (first, second)
                } else {
                    (second, first)
                };
                context.bind_object_group(chosen, chosen_objects);
                context.bind_object_group(unchosen, unchosen_objects);
                self.resolve_nested_effect_before_later(effect, &object, context);
            }
            DecisionContinuation::DrawActionWindow { card } => {
                if options.contains(&1) {
                    self.reveal_miracle(player, card);
                }
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
                on_complete.run(
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
                on_complete.run(
                    &mut runtime,
                    PileChoice {
                        resolving_controller: piles.resolving_controller,
                        subject: piles.subject,
                        chosen: object_ids(chosen),
                        unchosen: object_ids(unchosen),
                    },
                );
            }
            DecisionContinuation::SearchZonesAndExileRest {
                player,
                zones,
                searched,
            } => {
                // The submitted order is the arrangement, so the ids are read
                // from the submission rather than from the offer.
                let ordered = options
                    .iter()
                    .filter_map(|selected| {
                        pending
                            .observation
                            .options
                            .iter()
                            .find(|option| option.id == *selected)
                            .and_then(|option| option.card.map(|(card, _)| card))
                    })
                    .collect::<Vec<_>>();
                self.finish_search_zones_and_exile_rest(player, &zones, &searched, &ordered);
            }
            DecisionContinuation::Vote {
                candidates,
                remaining,
                mut votes,
            } => {
                votes.extend(
                    pending
                        .observation
                        .options
                        .iter()
                        .filter(|option| options.contains(&option.id))
                        .filter_map(|option| option.card.map(|(card, _)| card)),
                );
                self.queue_next_vote(candidates, remaining, votes);
            }
            DecisionContinuation::SacrificeOfChoice {
                followup,
                declined,
                optional,
            } => {
                let sacrificed = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                let chosen = sacrificed.first().copied();
                self.capture_sacrifices(&sacrificed);
                // "If a player does" -- declining an optional sacrifice earns
                // nothing, while a compulsory one pays out even for nothing.
                if let Some(followup) = followup
                    && (chosen.is_some() || !optional)
                {
                    self.move_permanents_to_graveyard_then(
                        &sacrificed,
                        Some(BattlefieldExitCompletion::SacrificeFollowup {
                            followup,
                            sacrificed: chosen,
                        }),
                    );
                } else {
                    self.move_permanents_to_graveyard(&sacrificed);
                }
                // The declined branch is the other half of one printed
                // clause, so it runs only when nothing was actually given up.
                if chosen.is_none() {
                    self.resolve_sacrifice_declined(declined);
                }
            }
            DecisionContinuation::SearchZone {
                controller,
                source,
                destination,
                placement,
                reveal,
                shuffle,
                enters_tapped,
                attached_player,
                binding,
                follow_up,
            } => {
                let selected = selected_cards(&pending.observation.options, options);
                self.resolve_completed_search(
                    player,
                    &selected,
                    SearchResolution {
                        controller,
                        source,
                        destination,
                        placement,
                        reveal,
                        shuffle,
                        enters_tapped,
                        attached_player,
                        binding,
                        follow_up,
                    },
                );
            }
            DecisionContinuation::ChooseCards {
                controller,
                destination,
                placement,
                reveal,
                arrival,
            } => {
                self.move_chosen_cards(
                    player,
                    controller,
                    destination,
                    placement,
                    reveal,
                    arrival.as_deref(),
                    &pending.observation.options,
                    options,
                );
            }
            DecisionContinuation::DrawReplacement {
                player,
                mut replacements,
            } => {
                let selected = options
                    .first()
                    .and_then(|option| option.checked_sub(1))
                    .and_then(|option| usize::try_from(option).ok())
                    .filter(|index| *index < replacements.len());
                let Some(selected) = selected else {
                    if replacements.iter().all(|replacement| replacement.optional) {
                        self.draw_replacements[player.index()].extend(
                            replacements
                                .into_iter()
                                .filter(|replacement| replacement.installed),
                        );
                        self.commit_draw_card(player);
                    } else {
                        self.draw_replacements[player.index()].extend(
                            replacements
                                .into_iter()
                                .filter(|replacement| replacement.installed),
                        );
                    }
                    return;
                };
                let replacement = replacements.remove(selected);
                self.draw_replacements[player.index()].extend(
                    replacements
                        .into_iter()
                        .filter(|replacement| replacement.installed),
                );
                // The interrupted draw instruction and any enclosing effect
                // tail are already queued behind this choice. A chosen
                // replacement is part of the current draw, so let every
                // procedure it starts finish before restoring that later
                // work. In particular, a replacement that draws must not be
                // deferred until after the original instruction resumes.
                let mut later_procedures = std::mem::take(&mut self.pending_procedures);
                self.resolve_effect_def(
                    replacement.effect,
                    &replacement.object,
                    replacement.context,
                );
                self.pending_procedures.append(&mut later_procedures);
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
                if sacrifices.is_empty() {
                    self.discard_cards_with_cause(task.player, &discards, task.cause);
                    if !remaining.is_empty() {
                        let next = remaining.remove(0);
                        self.queue_balance_task(controller, phase, next, remaining);
                    } else if let Some(next) = phase.next() {
                        self.queue_balance_phase(controller, next);
                    }
                } else {
                    debug_assert!(discards.is_empty(), "a Balance task uses one zone action");
                    self.move_permanents_to_graveyard_then(
                        &sacrifices,
                        Some(BattlefieldExitCompletion::Balance {
                            controller,
                            phase,
                            remaining,
                        }),
                    );
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
                        crate::card::tokens::tetravite(),
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
            trigger @ (DecisionContinuation::TriggerOrder { .. }
            | DecisionContinuation::TriggerPlacement { .. }
            | DecisionContinuation::TriggerMode { .. }
            | DecisionContinuation::TriggerDivision { .. }) => {
                self.complete_trigger_continuation(trigger, options);
            }
        }
    }
}

/// The cards an answered search selected, in the order the options offered
/// them.
fn selected_cards(
    offered: &[super::DecisionOption],
    options: &[u32],
) -> Vec<(super::GameObjectId, super::CardDefinitionId)> {
    options
        .iter()
        .filter_map(|selected| {
            offered
                .iter()
                .find(|option| option.id == *selected)
                .and_then(|option| option.card)
                .and_then(|(object, characteristics)| {
                    characteristics
                        .card_definition()
                        .map(|definition| (object, definition))
                })
        })
        .collect()
}
