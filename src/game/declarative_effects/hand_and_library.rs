//! Drawing, discarding, milling, and searching: the effects that move cards
//! through a player's hand and library.

use super::super::{
    CardPartId, CastOffer, CastOfferCost, CastSourceZone, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, DiscardSelectionDef, DrawReplacement,
    EffectDef, EffectResolutionContext, Game, GameEvent, GameObjectId, ObjectCharacteristics,
    PlayerId, ScopedEffect, StackObject, Target, ZoneKind, ZoneMoveCause, public_cards,
    remove_card,
};
use crate::card::{ExilePlayDurationDef, ObjectPredicateDef};

/// Manifest dread (CR 701.34, 702.169): look at the top two, one goes down
/// face down as a 2/2 and the other goes to the graveyard. The procedure is
/// fixed, so the card names the keyword and this states it once.
static MANIFEST_DREAD: crate::card::TopCardSelectionDef = crate::card::TopCardSelectionDef {
    count: crate::card::ValueDef::Constant(2),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    counted: None,
    selected_zone: ZoneKind::Battlefield,
    selected_placement: crate::card::ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: Some(crate::card::face_down::manifest()),
    rest_zone: ZoneKind::Graveyard,
    rest_placement: crate::card::ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
};

impl Game {
    /// "Exile cards from the top of your library until you exile a nonland
    /// card." Everything walked past is exiled with it; only the card that
    /// matched gets the permission, and a library that ran out gives one to
    /// nobody.
    fn exile_from_top_until(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        caster: PlayerId,
    ) {
        let mut passed = Vec::new();
        let mut matched = None;
        while let Some(card) = self.players[player.index()].library.pop() {
            if self.card_object_matches(predicate, &card, ZoneKind::Library, source) {
                matched = Some(card);
                break;
            }
            passed.push(card);
        }
        // One move however many cards it walked past, so a clause reading
        // "one or more cards" sees the whole of it at once.
        let mut moved = Vec::new();
        for card in passed {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].exile.push(card.clone());
            moved.push(card);
        }
        let Some(card) = matched else {
            self.capture_cards_exiled(&moved, ZoneKind::Library);
            return;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let exiled = card.id;
        self.players[player.index()].exile.push(card.clone());
        moved.push(card);
        self.capture_cards_exiled(&moved, ZoneKind::Library);
        self.permit_energy_cast(exiled, caster);
    }

    /// Cascade (CR 702.85), whole. The bound is the cascading spell's own
    /// mana value, read off the stack object the trigger came from, so a
    /// spell that has left the stack cascades into nothing rather than into
    /// everything.
    fn cascade(&mut self, object: &StackObject) {
        let player = object.controller;
        let Some(limit) = self.cascading_spell_mana_value(object) else {
            return;
        };
        let mut exiled = Vec::new();
        let mut moved = Vec::new();
        let mut matched = None;
        while let Some(card) = self.players[player.index()].library.pop() {
            let qualifies = self.catalog.get(card.definition).is_some_and(|definition| {
                !definition.rules.has_type(crate::card::CardType::Land)
                    && definition.rules.printed_mana_cost().mana_value() < limit
            });
            let (card, _zone_change) = self.zone_change_card(card);
            let id = card.id;
            self.players[player.index()].exile.push(card.clone());
            moved.push(card);
            exiled.push(id);
            if qualifies {
                matched = Some(id);
                break;
            }
        }
        self.capture_cards_exiled(&moved, ZoneKind::Library);
        let Some(matched) = matched else {
            self.bury_cascade_exiles(player, &exiled);
            return;
        };
        self.permit_free_play_this_turn(matched, player);
        let mut castable = Vec::new();
        self.add_offered_cast_actions(
            CastOffer {
                player,
                card: matched,
                source_zone: CastSourceZone::Exile,
                cost: CastOfferCost::Any,
            },
            &mut castable,
        );
        if castable.is_empty() {
            self.consume_exile_play_permission(matched);
            self.bury_cascade_exiles(player, &exiled);
            return;
        }
        let name = self
            .card_in_nonbattlefield_zone(matched)
            .and_then(|(_, card)| self.catalog.get(card.definition))
            .map_or_else(|| "that card".to_owned(), |card| card.name.clone());
        self.queue_decision(
            player,
            format!("Cast {name} without paying its mana cost, or decline"),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            1..=1,
            false,
            vec![DecisionOption {
                id: 0,
                label: "Decline".into(),
                card: self.card_in_nonbattlefield_zone(matched).map(|(_, card)| {
                    (
                        matched,
                        ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                    )
                }),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Exile,
            }],
            DecisionContinuation::CascadeCast {
                player,
                card: matched,
                exiled,
            },
        );
    }

    /// The mana value of the spell a cascade trigger came from. The trigger
    /// is a separate object, so this reads the spell it names.
    fn cascading_spell_mana_value(&self, object: &StackObject) -> Option<u16> {
        let source = object.source.unwrap_or(object.id);
        self.stack
            .iter()
            .find(|candidate| candidate.id == source)
            .and_then(|candidate| self.stack_object_event_object(candidate))
            .map(|event| event.mana_value)
    }

    /// "Then put all cards exiled this way on the bottom of your library in
    /// a random order." A card that is no longer in exile -- the one that
    /// was cast -- simply is not among them.
    pub(in crate::game) fn bury_cascade_exiles(
        &mut self,
        player: PlayerId,
        exiled: &[GameObjectId],
    ) {
        let mut returning = Vec::new();
        for id in exiled {
            if let Some(card) = remove_card(&mut self.players[player.index()].exile, *id) {
                let (card, _zone_change) = self.zone_change_card(card);
                returning.push(card);
            }
        }
        self.rng.shuffle(&mut returning);
        for card in returning.into_iter().rev() {
            self.players[player.index()].library.insert(0, card);
        }
    }

    /// "Puts all the cards from their graveyard on the bottom of their
    /// library in a random order." The order is randomized before anything
    /// moves, so what lands where is decided once for the whole pile rather
    /// than card by card.
    fn bury_graveyard(&mut self, player: PlayerId) {
        let graveyard = std::mem::take(&mut self.players[player.index()].graveyard);
        if graveyard.is_empty() {
            return;
        }
        self.note_card_left_graveyard(player);
        let mut returning = graveyard
            .into_iter()
            .map(|card| self.zone_change_card(card).0)
            .collect::<Vec<_>>();
        self.rng.shuffle(&mut returning);
        for card in returning.into_iter().rev() {
            self.players[player.index()].library.insert(0, card);
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_hand_and_library_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::DrawCards { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let mut players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                // CR 121.2c: when multiple players draw, the active player
                // performs every individual draw first, followed by the
                // nonactive player. This order belongs to drawing rather than
                // to the general `EachPlayer` recipient.
                players.sort_by_key(|player| (*player != self.active_player, player.index()));
                for player in players {
                    self.draw_instruction(player, amount);
                }
            }
            EffectDef::ShuffleLibrary { player: recipient } => {
                let mut players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                players.sort_by_key(|player| (*player != self.active_player, player.index()));
                for player in players {
                    self.rng.shuffle(&mut self.players[player.index()].library);
                }
            }
            EffectDef::BuryGraveyard { player: recipient } => {
                let players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                for player in players {
                    self.bury_graveyard(player);
                }
            }
            EffectDef::Discard {
                recipient,
                amount,
                selection: DiscardSelectionDef::RecipientChooses,
                then,
            } => {
                let amount = self.effect_value(amount, object, context, scoped).max(0);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                let players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    // A player nobody can force to discard is simply not
                    // among the ones asked to.
                    .filter(|player| self.can_be_forced_to_discard(*player, object.controller))
                    .collect();
                let follow_up = then.map(|follow_up| crate::game::DiscardFollowUp {
                    counted: follow_up.counted,
                    bound: follow_up.bound,
                    effect: scoped.with_effect(*follow_up.effect),
                    object: Box::new(object.clone()),
                    context: context.clone(),
                });
                self.queue_effect_discards_then(players, amount, cause, follow_up);
            }
            EffectDef::Discard {
                recipient,
                amount,
                selection: DiscardSelectionDef::Random,
                then: None,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target
                        && self.can_be_forced_to_discard(player, object.controller)
                    {
                        self.discard_random(player, amount, cause);
                    }
                }
            }
            EffectDef::Discard {
                recipient,
                amount,
                selection: DiscardSelectionDef::RandomMatching(predicate),
                then: None,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.discard_random_matching(
                            player,
                            amount,
                            *predicate,
                            object.source.unwrap_or(object.id),
                            cause,
                        );
                    }
                }
            }
            EffectDef::DiscardCards { object: recipient } => {
                let recipients = self.effect_recipients(recipient, object, context, scoped);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                for player in [self.active_player, self.active_player.opponent()] {
                    let cards = recipients
                        .iter()
                        .filter_map(|target| match target {
                            Target::Card(card) => Some(*card),
                            Target::Player(_) | Target::Permanent(_) | Target::Spell(_) => None,
                        })
                        .filter(|card| {
                            self.players[player.index()]
                                .hand
                                .iter()
                                .any(|candidate| candidate.id == *card)
                        })
                        .collect::<Vec<_>>();
                    self.discard_cards_with_cause(player, &cards, cause);
                }
            }
            EffectDef::MillUntil(mill) => {
                let (recipient, predicate, matched_zone, binding, then) = (
                    mill.player,
                    mill.object,
                    mill.matched_zone,
                    mill.binding,
                    mill.then,
                );
                let source = object.source.unwrap_or(object.id);
                let mut revealed = Vec::new();
                let mut revealed_count = 0_u16;
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        let (mut moved, count) =
                            self.mill_until_matching(player, predicate, matched_zone, source);
                        revealed.append(&mut moved);
                        revealed_count = revealed_count.saturating_add(count);
                    }
                }
                let Some(then) = then else {
                    return;
                };
                // Revealing and moving are synchronous, so the follow-up runs
                // inline with both the exact moved identities and the frozen
                // number revealed available to it.
                let mut context = context.clone();
                context.matched_count = Some(revealed_count);
                if let Some(binding) = binding {
                    context.bind_object_group(binding, revealed);
                }
                self.resolve_effect_def(scoped.with_effect(*then), object, context);
            }
            EffectDef::Cascade => self.cascade(object),
            EffectDef::ManifestDread { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_top_card_selection(
                            player,
                            player,
                            &MANIFEST_DREAD,
                            object,
                            context.clone(),
                            scoped,
                        );
                    }
                }
            }
            EffectDef::ExileFromTopUntil {
                player: recipient,
                object: predicate,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.exile_from_top_until(player, predicate, source, object.controller);
                    }
                }
            }
            EffectDef::Mill {
                player: recipient,
                amount,
                binding,
                then,
            } => {
                let count = self.effect_value(amount, object, context, scoped).max(0);
                let Ok(count) = usize::try_from(count) else {
                    return;
                };
                let mut buried = Vec::new();
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        let milled = self.take_top_of_library(player, count);
                        // Bound by the identity the cards have in the
                        // graveyard: burying them mints new objects, and
                        // "from among them" means the ones lying there now.
                        for card in milled {
                            let (card, _zone_change) = self.zone_change_card(card);
                            buried.push(Target::Card(card.id));
                            self.put_card_into_graveyard(player, card);
                        }
                    }
                }
                let Some(then) = then else {
                    return;
                };
                // A mill never stops to ask, so the follow-up runs here
                // rather than out of a continuation.
                let mut context = context.clone();
                if let Some(binding) = binding {
                    context.bind_object_group(binding, buried);
                }
                self.resolve_effect_def(scoped.with_effect(*then), object, context);
            }
            EffectDef::ExileAtRandomFromGraveyardToPlay { player: recipient } => {
                let controller = object.controller;
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let Target::Player(player) = target else {
                        continue;
                    };
                    let graveyard = &self.players[player.index()].graveyard;
                    if graveyard.is_empty() {
                        continue;
                    }
                    let index = self
                        .rng
                        .index_below(self.players[player.index()].graveyard.len());
                    let card = self.players[player.index()].graveyard.remove(index);
                    // A zone change mints a new object, and the permission
                    // has to name the card that ended up in exile.
                    let (card, _zone_change) = self.zone_change_card(card);
                    let exiled = card.id;
                    self.players[player.index()].exile.push(card.clone());
                    self.note_card_left_graveyard(player);
                    self.capture_cards_exiled(&[card], ZoneKind::Graveyard);
                    self.permit_cast_this_turn(exiled, controller);
                }
            }
            EffectDef::ExileTopOfLibraryToPlay {
                player: recipient,
                amount,
                free,
                face_down,
                duration,
                spend_any_color,
                play_condition,
            } => {
                let count = self.effect_value(amount, object, context, scoped).max(0);
                let Ok(count) = usize::try_from(count) else {
                    return;
                };
                let controller = object.controller;
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        let mut moved = Vec::new();
                        for card in self.take_top_of_library(player, count) {
                            let (card, _zone_change) = self.zone_change_card(card);
                            let exiled = card.id;
                            self.players[player.index()].exile.push(card.clone());
                            moved.push(card);
                            match (free, face_down, duration) {
                                (true, _, _) => self.permit_free_play_this_turn(exiled, controller),
                                (false, true, _) => {
                                    self.permit_face_down_play_this_turn(exiled, controller);
                                }
                                (false, false, ExilePlayDurationDef::ThisTurn) => {
                                    self.permit_cast_this_turn(exiled, controller);
                                }
                                (false, false, ExilePlayDurationDef::UntilYourNextEndStep) => {
                                    self.permit_play_until_your_next_end_step(exiled, controller);
                                }
                                // Bounded by the exile rather than by a turn:
                                // what limits it is whatever the clause asks
                                // for each time it is played.
                                (false, false, ExilePlayDurationDef::WhileExiled) => {
                                    self.permit_conditional_cast_while_exiled(exiled, controller);
                                }
                            }
                            if spend_any_color || play_condition.is_some() {
                                self.qualify_exile_permission(
                                    exiled,
                                    spend_any_color,
                                    play_condition,
                                );
                            }
                        }
                        self.capture_cards_exiled(&moved, ZoneKind::Library);
                    }
                }
            }
            EffectDef::ExileTopAndMayCast {
                player: recipient, ..
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.exile_top_and_offer_cast(player, object, context.clone(), scoped);
                    }
                }
            }
            // One card, seen by the looker alone: what they now know about
            // that hand is that card and nothing else, which is what the
            // last-seen record holds.
            EffectDef::LookAtRandomCardInHand { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(seen) = target {
                        let hand = &self.players[seen.index()].hand;
                        if hand.is_empty() {
                            continue;
                        }
                        // Drawn through the game's seeded RNG, so a replay
                        // looks at the same card.
                        let index = self.rng.index_below(hand.len());
                        let card = &self.players[seen.index()].hand[index];
                        let looked = vec![(card.id, card.definition)];
                        self.last_seen_hands[object.controller.index()] = Some((seen, looked));
                    }
                }
            }
            EffectDef::LookAtHand { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(seen) = target {
                        self.last_seen_hands[object.controller.index()] =
                            Some((seen, public_cards(&self.players[seen.index()].hand)));
                    }
                }
            }
            EffectDef::RevealHand { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(revealer) = target {
                        let hand = &self.players[revealer.index()].hand;
                        let events = hand
                            .iter()
                            .map(|card| GameEvent::CardRevealed {
                                player: revealer,
                                card: card.id,
                                definition: card.definition,
                            })
                            .collect::<Vec<_>>();
                        let seen = public_cards(hand);
                        self.events.extend(events);
                        // Everyone saw it, so everyone remembers it.
                        for viewer in &mut self.last_seen_hands {
                            *viewer = Some((revealer, seen.clone()));
                        }
                    }
                }
            }
            EffectDef::RevealAtRandomFromHand {
                player: recipient,
                binding,
                then,
            } => {
                let mut context = context.clone();
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Player(revealer) = target {
                        // Drawn through the game's seeded RNG so a replay
                        // reveals the same card, and read before anything
                        // moves so the reveal is of the hand as it stands.
                        let hand = &self.players[revealer.index()].hand;
                        let revealed = (!hand.is_empty()).then(|| {
                            let index = self.rng.index_below(hand.len());
                            let card = &self.players[revealer.index()].hand[index];
                            (card.id, card.definition)
                        });
                        if let Some((card, definition)) = revealed {
                            self.events.push(GameEvent::CardRevealed {
                                player: revealer,
                                card,
                                definition,
                            });
                            context.bind_single_object(binding, Some(Target::Card(card)));
                        }
                    }
                }
                self.resolve_effect_def(scoped.with_effect(*then), object, context);
            }
            EffectDef::LookAtTopAndSelect {
                player: recipient,
                looker,
                selection,
            } => {
                // The looker is resolved first and once: a spy that has left
                // the table still finishes looking, but nobody else does it
                // for them.
                let Some(Target::Player(looker)) = self
                    .effect_recipients(looker, object, context, scoped)
                    .into_iter()
                    .next()
                else {
                    return;
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_top_card_selection(
                            player,
                            looker,
                            selection,
                            object,
                            context.clone(),
                            scoped,
                        );
                    }
                }
            }
            EffectDef::SearchZone {
                player: recipient,
                source: source_zone,
                object: predicate,
                minimum,
                maximum,
                reveal,
                destination,
                placement,
                shuffle,
                enters_tapped,
                binding,
                then,
            } => {
                let source = object.source.unwrap_or(object.id);
                // A computed mana-value bound belongs to this resolution, not
                // merely to the source object. Freeze it while the search is
                // created so values such as "the number of lands you
                // control" can use the full effect context before the hidden
                // zone choices are filtered.
                let predicate = match predicate {
                    ObjectPredicateDef::ManaValueAtMostValue(value) => {
                        ObjectPredicateDef::ManaValueAtMostValue(crate::card::ValueDef::Constant(
                            self.effect_value(value, object, context, scoped),
                        ))
                    }
                    _ => predicate,
                };
                // Sized once, before the search is offered: "up to X, where X
                // is the number of lands you control" is answered by the
                // board as the spell resolves.
                let maximum =
                    usize::try_from(self.effect_value(maximum, object, context, scoped).max(0))
                        .unwrap_or(usize::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_zone_search(
                            player,
                            source_zone,
                            predicate,
                            minimum,
                            maximum,
                            reveal,
                            destination,
                            placement,
                            shuffle,
                            binding,
                            then.map(|effect| {
                                (object.clone(), context.clone(), scoped.with_effect(*effect))
                            }),
                            enters_tapped,
                            source,
                            object.controller,
                        );
                    }
                }
            }
            EffectDef::ChooseCards {
                player: recipient,
                sources,
                object: predicate,
                minimum,
                maximum,
                reveal,
                destination,
                placement,
                arrival_effect,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_owned_card_choice(
                            player,
                            sources,
                            predicate,
                            minimum,
                            maximum,
                            reveal,
                            destination,
                            placement,
                            // Only a battlefield arrival can carry anything,
                            // and only the clauses that print one do.
                            arrival_effect.map(|_| (object.clone(), context.clone(), scoped)),
                            source,
                            object.controller,
                        );
                    }
                }
            }
            EffectDef::ReplaceNextDrawThisTurn {
                player: recipient,
                effect,
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.draw_replacements[player.index()].push_back(DrawReplacement {
                            object: Box::new(object.clone()),
                            context: context.clone(),
                            effect: scoped.with_effect(*effect),
                            optional: false,
                            installed: true,
                        });
                    }
                }
            }
            _ => unreachable!("resolve_hand_and_library_effect called for another effect"),
        }
    }
}
