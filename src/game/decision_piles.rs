use std::ops::ControlFlow;

use super::{
    AppliedRuleDef, BalancePhase, BalanceTask, BattlefieldExitCompletion, CardInstance, CardPartId,
    CardRuntime, CommittedTriggerEvent, CounterKind, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, DeclarativeAbilityDef, DiscardFollowUp,
    EffectDef, EffectResolutionContext, Game, GameEvent, GameObjectId, ObjectCharacteristics,
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
            self.take_all_matching_top_cards(
                player,
                (chosen, rest),
                selection,
                object,
                context,
                scoped,
            );
            return;
        }
        let (options, preference) =
            self.top_card_selection_options(&revealed, &eligible, selection);
        let no_selection = options.len() == 1 && options[0].card.is_none();
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

    /// The cards the look offers and how a bot should lean. A selection that
    /// may take nothing is a look and nothing more, so it is presented the
    /// same way as one with no eligible card: the cards ride along as
    /// members and the only option acknowledges them.
    fn top_card_selection_options(
        &self,
        revealed: &[CardInstance],
        eligible: &[CardInstance],
        selection: &'static TopCardSelectionDef,
    ) -> (Vec<DecisionOption>, DecisionPreference) {
        let inspected = revealed
            .iter()
            .map(|card| {
                (
                    card.id,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                )
            })
            .collect::<Vec<_>>();
        let mut options = self.card_decision_options(eligible, DecisionZone::Library);
        for option in &mut options {
            option.members.clone_from(&inspected);
        }
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
        (options, preference)
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
    /// "Put all the ones that match into your hand", which asks nothing:
    /// the predicate has already partitioned the cards, and what the
    /// follow-up reads is what they were.
    fn take_all_matching_top_cards(
        &mut self,
        player: PlayerId,
        piles: (Vec<CardInstance>, Vec<CardInstance>),
        selection: &'static TopCardSelectionDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let (chosen, rest) = piles;
        let hider = object.source.unwrap_or(object.id);
        let (count, mana_value) = self.selected_card_totals(&chosen, selection.counted, hider);
        self.finish_top_card_selection_from(player, chosen, rest, selection, Some(hider));
        if let Some(then) = selection.then {
            let mut context = context;
            context.matched_count = Some(count);
            context.matched_mana_value = Some(mana_value);
            self.resolve_effect_def(scoped.with_effect(*then), object, context);
        }
    }

    /// How many cards a selection took and what they add up to in mana
    /// value. Read before they move: a card in a hand is no longer something
    /// the resolution can find.
    pub(super) fn selected_card_totals(
        &self,
        chosen: &[CardInstance],
        counted: Option<ObjectPredicateDef>,
        source: GameObjectId,
    ) -> (u16, u16) {
        let counted = chosen
            .iter()
            .filter(|card| {
                counted.is_none_or(|predicate| {
                    self.card_object_matches(predicate, card, ZoneKind::Library, source)
                })
            })
            .collect::<Vec<_>>();
        let count = u16::try_from(counted.len()).unwrap_or(u16::MAX);
        let mana_value = counted
            .into_iter()
            .filter_map(|card| self.catalog.get(card.definition))
            .map(|definition| definition.rules.printed_mana_cost().mana_value())
            .fold(0_u16, u16::saturating_add);
        (count, mana_value)
    }

    /// How many cards a resolving step handled and what they add up to in
    /// mana value. Read before they move: the follow-up may need the number
    /// after the cards have left the zone where the step found them.
    pub(super) fn card_totals(&self, cards: &[CardInstance]) -> (u16, u16) {
        let count = u16::try_from(cards.len()).unwrap_or(u16::MAX);
        let mana_value = cards
            .iter()
            .filter_map(|card| self.catalog.get(card.definition))
            .map(|definition| definition.rules.printed_mana_cost().mana_value())
            .fold(0_u16, u16::saturating_add);
        (count, mana_value)
    }

    /// Places what a look selected and what it passed over, told which
    /// object did the looking. Hideaway is why the source travels: the land
    /// that took the card is the only thing that can name it again, and
    /// exile says nothing about where a card came from.
    pub(super) fn finish_top_card_selection_from(
        &mut self,
        player: PlayerId,
        chosen: Vec<CardInstance>,
        rest: Vec<CardInstance>,
        selection: &'static TopCardSelectionDef,
        source: Option<GameObjectId>,
    ) {
        let records_selection =
            (selection.selected_linked_to_source && source.is_some()) || selection.selected_hidden;
        let hidden = if records_selection {
            chosen.iter().map(|card| card.id).collect::<Vec<_>>()
        } else {
            Vec::new()
        };
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
        // "The rest go back in a random order": the looker has seen them, so
        // the order they return in is decided by the game rather than by
        // what order they happened to come out in.
        let mut rest = rest;
        if selection.rest_random_order {
            self.rng.shuffle(&mut rest);
        }
        let unselected: Vec<_> = rest.iter().map(|card| card.id).collect();
        self.place_revealed_remainder(player, rest, selection.rest_zone, selection.rest_placement);
        // "Exile the other with a silver counter on it": settled after the
        // move, because what landed is a new object and the counter belongs
        // to that one.
        if let Some((kind, amount)) = selection.rest_counters {
            for card in unselected {
                let Some(moved) = self.successors.get(&card).copied() else {
                    continue;
                };
                if let Some(instance) = self.card_in_nonbattlefield_zone_mut(moved) {
                    instance.add_counters(kind, amount);
                }
            }
        }
        // Both are settled after the move: a card in exile is a new object,
        // and what names it has to name the one that is there now.
        for card in hidden {
            let Some(exiled) = self.successors.get(&card).copied() else {
                continue;
            };
            if let Some(source) = source
                && selection.selected_linked_to_source
            {
                self.linked_exiles.push((source, exiled));
            }
            if selection.selected_hidden {
                self.permit_look_while_exiled(exiled, player);
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
