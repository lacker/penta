//! Drawing, discarding, milling, and searching: the effects that move cards
//! through a player's hand and library.

use super::super::{
    CardPartId, CastOffer, CastOfferCost, CastSourceZone, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, DiscardSelectionDef, DrawReplacement,
    EffectDef, EffectResolutionContext, Game, GameEvent, GameObjectId, ObjectCharacteristics,
    PlayerId, ScopedEffect, StackObject, Target, ZoneKind, ZoneMoveCause, public_cards,
    remove_card,
};
use crate::card::{ArrivalAttachmentDef, ObjectPredicateDef, ObjectRefDef};

impl Game {
    /// "Target opponent exiles the top card of their library, a card at
    /// random from their graveyard, and a card at random from their hand."
    ///
    /// One pile out of three zones. The random picks are drawn through the
    /// game's seeded RNG so a replay exiles the same cards, and each zone is
    /// read as it stands when its turn comes -- nothing here moves a card
    /// into a zone another pick could find.
    fn exile_one_from_each_zone(
        &mut self,
        player: PlayerId,
        zones: &'static [crate::card::ZonePickDef],
        permission: Option<crate::card::ExiledCastPermissionDef>,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let source = object.card.id;
        let caster = object.controller;
        let mut pile = Vec::new();
        for pick in zones {
            let size = match pick.zone {
                ZoneKind::Library => self.players[player.index()].library.len(),
                ZoneKind::Hand => self.players[player.index()].hand.len(),
                ZoneKind::Graveyard => self.players[player.index()].graveyard.len(),
                ZoneKind::Exile | ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => 0,
            };
            if size == 0 {
                continue;
            }
            let index = match pick.pick {
                // The top of a library is the end of the vector, which is
                // where a draw takes from.
                crate::card::ZonePickModeDef::Top => size.saturating_sub(1),
                crate::card::ZonePickModeDef::AtRandom => self.rng.index_below(size),
            };
            let cards = match pick.zone {
                ZoneKind::Library => &mut self.players[player.index()].library,
                ZoneKind::Hand => &mut self.players[player.index()].hand,
                ZoneKind::Graveyard => &mut self.players[player.index()].graveyard,
                ZoneKind::Exile | ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    continue;
                }
            };
            let card = cards.remove(index);
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].exile.push(card.clone());
            // One capture per zone: a clause watching for cards leaving a
            // hand should not be told a library card left it.
            self.capture_cards_exiled(core::slice::from_ref(&card), pick.zone);
            pile.push(card.id);
        }
        let Some(permission) = permission else {
            return;
        };
        // One permission over the pile, named by the object whose
        // resolution made it: casting any one of these spends it, which is
        // what "a spell from among cards exiled this way" allows.
        for card in &pile {
            match permission {
                crate::card::ExiledCastPermissionDef::EnergyEqualToManaValue => {
                    self.permit_energy_cast(*card, caster);
                }
                crate::card::ExiledCastPermissionDef::FreeWhileResolving => {
                    self.permit_free_play_this_turn(*card, caster);
                    // "You may cast", so a land among them stays in exile.
                    self.restrict_exile_permission_to_casting(*card);
                }
            }
            self.group_last_exile_permission(*card, source);
        }
        // "As this ability resolves": a free cast with no printed duration
        // happens here or not at all, so the permission is offered as a
        // standing decision and declining takes it straight back.
        if permission == crate::card::ExiledCastPermissionDef::FreeWhileResolving {
            for card in pile {
                self.offer_permitted_play(caster, card, false, object, context, scoped);
            }
        }
    }

    /// "Exile cards from the top of your library until you exile a nonland
    /// card." Everything walked past is exiled with it; only the card that
    /// matched gets the permission, and a library that ran out gives one to
    /// nobody.
    fn exile_from_top_until(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        permission: crate::card::ExiledCastPermissionDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let source = object.source.unwrap_or(object.id);
        let caster = object.controller;
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
        // The permission is the effect's controller's either way, which is
        // what lets one player cast what another player's library turned up.
        match permission {
            crate::card::ExiledCastPermissionDef::EnergyEqualToManaValue => {
                self.permit_energy_cast(exiled, caster);
                self.restrict_exile_permission_to_casting(exiled);
                self.offer_permitted_play(caster, exiled, false, object, context, scoped);
            }
            crate::card::ExiledCastPermissionDef::FreeWhileResolving => {
                self.permit_free_play_this_turn(exiled, caster);
                self.restrict_exile_permission_to_casting(exiled);
                self.offer_permitted_play(caster, exiled, false, object, context, scoped);
            }
        }
    }

    /// Cascade (CR 702.85), whole. The bound is the cascading spell's own
    /// mana value, which is a fact about the spell rather than about where it
    /// is: a countered spell still cascades, and still cascades into
    /// something cheaper than it was.
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
            // "If a spell with cascade is countered, the cascade ability will
            // still resolve normally": the spell it came from is gone by
            // then, and what it cost is last known information.
            .or_else(|| self.current_or_last_known_mana_value(source))
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
            EffectDef::ExileOneFromEachZone(pile) => {
                for target in self.effect_recipients(pile.player, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.exile_one_from_each_zone(
                            player,
                            pile.zones,
                            pile.permission,
                            object,
                            context,
                            scoped,
                        );
                    }
                }
            }
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
                    definition: scoped,
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
            EffectDef::Cascade => self.cascade(object),
            EffectDef::ExileFromTopUntil {
                player: recipient,
                object: predicate,
                permission,
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.exile_from_top_until(
                            player, predicate, permission, object, context, scoped,
                        );
                    }
                }
            }
            EffectDef::MillWhileMatching(mill) => {
                for target in self.effect_recipients(mill.player, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.mill_while_matching(player, mill, object, context, scoped);
                    }
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
                cast_only,
            } => {
                let count = self.effect_value(amount, object, context, scoped).max(0);
                let Ok(count) = usize::try_from(count) else {
                    return;
                };
                let permission = super::exile_to_play::ExilePlayGrant {
                    free,
                    face_down,
                    duration,
                    spend_any_color,
                    play_condition,
                    cast_only,
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.exile_top_of_library_to_play(
                            player,
                            count,
                            object.controller,
                            permission,
                        );
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
                attachment,
                binding,
                then,
            } => {
                let mut source = object.source.unwrap_or(object.id);
                let attached_player = match attachment {
                    None => None,
                    Some(ArrivalAttachmentDef::ArrivalToPlayer(reference)) => {
                        let Some(player) =
                            self.player_reference(reference, object, context, scoped)
                        else {
                            return;
                        };
                        Some(player)
                    }
                    // Search arrivals currently support the player-attached
                    // direction. The catalog boundary rejects the other
                    // attachment directions for this effect shape.
                    Some(
                        ArrivalAttachmentDef::SourceToArrival
                        | ArrivalAttachmentDef::ArrivalToHost(_),
                    ) => return,
                };
                // A computed mana-value bound belongs to this resolution, not
                // merely to the source object. Freeze it while the search is
                // created so values such as "the number of lands you
                // control" can use the full effect context before the hidden
                // zone choices are filtered.
                let predicate = match predicate {
                    ObjectPredicateDef::NameEquals(crate::card::CardNameDef::NameOf(reference)) => {
                        let Some(referenced) =
                            self.effect_object_reference_id(reference, object, context, scoped)
                        else {
                            return;
                        };
                        source = referenced;
                        ObjectPredicateDef::NameEquals(crate::card::CardNameDef::NameOf(
                            ObjectRefDef::Source,
                        ))
                    }
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
                            attached_player,
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
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        let _ = self.queue_owned_card_choice(
                            player,
                            sources,
                            predicate,
                            minimum,
                            maximum,
                            reveal,
                            destination,
                            placement,
                            None,
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

    pub(in crate::game) fn resolve_mill_until_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) -> (EffectResolutionContext, Vec<Target>) {
        let EffectDef::MillUntil(mill) = scoped.effect else {
            unreachable!("resolve_mill_until_effect called for another effect")
        };
        let source = object.source.unwrap_or(object.id);
        let mut revealed = Vec::new();
        for target in self.effect_recipients(mill.player, object, &context, scoped) {
            if let Target::Player(player) = target {
                let (mut moved, _) =
                    self.mill_until_matching(player, mill.until, mill.matched_zone, source);
                revealed.append(&mut moved);
            }
        }
        (context, revealed)
    }

    pub(in crate::game) fn resolve_random_zone_selection_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) -> (EffectResolutionContext, Vec<Target>) {
        let EffectDef::SelectAtRandomFromZone {
            player: recipient,
            source,
            object: predicate,
            amount,
        } = scoped.effect
        else {
            unreachable!("resolve_random_zone_selection_effect called for another effect")
        };
        let effect_source = object.source.unwrap_or(object.id);
        let count = self.effect_value(amount, object, &context, scoped).max(0);
        let Ok(count) = usize::try_from(count) else {
            return (context, Vec::new());
        };
        let mut selected = Vec::new();
        for target in self.effect_recipients(recipient, object, &context, scoped) {
            let Target::Player(player) = target else {
                continue;
            };
            let cards = match source {
                ZoneKind::Hand => &self.players[player.index()].hand,
                ZoneKind::Library => &self.players[player.index()].library,
                ZoneKind::Graveyard => &self.players[player.index()].graveyard,
                ZoneKind::Exile => &self.players[player.index()].exile,
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => continue,
            };
            let mut matching = cards
                .iter()
                .filter(|card| self.card_object_matches(predicate, card, source, effect_source))
                .map(|card| card.id)
                .collect::<Vec<_>>();
            for _ in 0..count.min(matching.len()) {
                let index = self.rng.index_below(matching.len());
                selected.push(Target::Card(matching.swap_remove(index)));
            }
        }
        (context, selected)
    }

    pub(in crate::game) fn resolve_random_hand_reveal_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) -> (EffectResolutionContext, Option<Target>) {
        let EffectDef::RevealAtRandomFromHand { player: recipient } = scoped.effect else {
            unreachable!("resolve_random_hand_reveal_effect called for another effect")
        };
        let mut revealed_object = None;
        for target in self.effect_recipients(recipient, object, &context, scoped) {
            if let Target::Player(revealer) = target {
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
                    revealed_object = Some(Target::Card(card));
                }
            }
        }
        (context, revealed_object)
    }

    /// Resolves one synchronous mill and returns the graveyard identities it
    /// produced. A `BindOutput` wrapper decides whether to retain that result.
    pub(in crate::game) fn resolve_mill_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) -> (EffectResolutionContext, Vec<Target>) {
        let EffectDef::Mill {
            player: recipient,
            amount,
        } = scoped.effect
        else {
            unreachable!("resolve_mill_effect called for another effect")
        };
        let count = self.effect_value(amount, object, &context, scoped).max(0);
        let Ok(count) = usize::try_from(count) else {
            return (context, Vec::new());
        };
        let mut buried = Vec::new();
        for target in self.effect_recipients(recipient, object, &context, scoped) {
            if let Target::Player(player) = target {
                let milled = self.take_top_of_library(player, count);
                // Bound by the identity the cards have in the graveyard:
                // burying them mints new objects, and "from among them"
                // means the ones lying there now.
                for card in milled {
                    let (card, _zone_change) = self.zone_change_card(card);
                    buried.push(Target::Card(card.id));
                    self.put_card_into_graveyard_replacing(player, card, ZoneKind::Library);
                }
            }
        }
        (context, buried)
    }
}

impl Game {
    /// "…and repeat this process." The mill belongs to the loop rather than
    /// to the body: what was milled decides whether there is another pass,
    /// so the output controls the loop itself rather than a later sibling.
    fn mill_while_matching(
        &mut self,
        player: PlayerId,
        loop_def: &'static crate::card::MillLoopDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        for _ in 0..loop_def.limit {
            self.resolve_effect_def(scoped.with_effect(*loop_def.body), object, context.clone());
            let mut milled = self.take_top_of_library(player, 1);
            let Some(card) = milled.pop() else {
                // An empty library mills nothing, and nothing is not a
                // match, so this pass is the last.
                return;
            };
            let matches = self.card_object_matches(
                loop_def.object,
                &card,
                ZoneKind::Library,
                object.source.unwrap_or(object.id),
            );
            let (card, _zone_change) = self.zone_change_card(card);
            self.put_card_into_graveyard_replacing(player, card, ZoneKind::Library);
            if !matches {
                return;
            }
            self.resolve_effect_def(
                scoped.with_effect(*loop_def.on_match),
                object,
                context.clone(),
            );
        }
    }
}
