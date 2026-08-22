use super::{
    BalanceAction, BalancePhase, BalanceTask, CardBehavior, CardInstance, CardPartId, CardType,
    DecisionContinuation, DecisionPreference, DecisionVisibility, DecisionZone, Game, GameEvent,
    GameObjectId, ObjectCharacteristics, ObjectPredicateDef, PlayerId, StackObject, Target,
    ZoneKind, ZoneMoveCause, ZonePlacement,
};

impl Game {
    pub(super) fn resolve_custom_activated_ability(
        &mut self,
        object: &StackObject,
        behavior: CardBehavior,
    ) {
        match behavior {
            CardBehavior::SedgeTroll => {
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| Some(permanent.card.id) == object.source)
                {
                    permanent.regeneration_shields =
                        permanent.regeneration_shields.saturating_add(1);
                }
            }
            CardBehavior::LibraryOfAlexandria => {
                self.draw_cards(object.controller, 1);
            }
            _ => {}
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_spell_effect(&mut self, object: &StackObject, behavior: CardBehavior) {
        match behavior {
            CardBehavior::SphinxsRevelation => {
                let player = object.controller;
                self.gain_life(player, object.x());
                self.draw_cards(player, object.x());
            }
            CardBehavior::PillarOfFlame => {
                self.damage_target(object.first_target(), 2);
                if let Some(Target::Permanent(target)) = object.first_target()
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    permanent.exile_instead_of_dying = true;
                }
            }
            CardBehavior::GoblinGrenade => {
                self.damage_target(object.first_target(), 5);
            }
            CardBehavior::ChainLightning => {
                let deciding = match object.first_target() {
                    Some(Target::Player(player)) => Some(player),
                    Some(Target::Permanent(id)) => self.permanent_controller(id),
                    Some(Target::Card(_) | Target::Spell(_)) | None => None,
                };
                self.damage_target(object.first_target(), 3);
                if let Some(player) = deciding {
                    self.queue_chain_lightning_decision(player, object.clone());
                }
            }
            CardBehavior::Fireball => {
                let divisor = u16::try_from(object.target_count()).unwrap_or(u16::MAX);
                let amount = object.x().checked_div(divisor).unwrap_or(0);
                for target in object.targets() {
                    self.damage_target(Some(target), amount);
                }
            }
            CardBehavior::DustToDust => {
                for target in object.iter_targets().filter_map(|target| match target {
                    Target::Permanent(id) => Some(*id),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                }) {
                    self.exile_permanent(target);
                }
            }
            CardBehavior::Negate | CardBehavior::EssenceScatter => {
                if let Some(Target::Spell(target)) = object.first_target() {
                    self.counter_spell(target);
                }
            }
            CardBehavior::Fork => {
                if let Some(Target::Spell(target)) = object.first_target()
                    && let Some(original) =
                        self.stack.iter().find(|item| item.id == target).cloned()
                {
                    self.queue_fork_decision(object.controller, original);
                }
            }
            CardBehavior::Mulch => {
                let player = object.controller;
                let revealed = self.take_top_of_library(player, 4);
                let (lands, rest): (Vec<_>, Vec<_>) = revealed.into_iter().partition(|card| {
                    self.catalog
                        .get(card.definition)
                        .is_some_and(|definition| definition.rules.has_type(CardType::Land))
                });
                for card in lands {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                self.bury_cards(player, rest);
            }
            CardBehavior::GrislySalvage => {
                let player = object.controller;
                let revealed = self.take_top_of_library(player, 5);
                let eligible = revealed
                    .iter()
                    .filter(|card| {
                        self.catalog.get(card.definition).is_some_and(|definition| {
                            definition.rules.has_type(CardType::Creature)
                                || definition.rules.has_type(CardType::Land)
                        })
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let options = self.card_decision_options(&eligible, DecisionZone::Library);
                // "You may put ... into your hand": taking nothing is a real
                // choice, so the minimum is zero even when something qualifies.
                self.queue_decision(
                    player,
                    "Put a creature or land card into your hand",
                    DecisionVisibility::Public,
                    DecisionPreference::HigherCardValue,
                    0..=1,
                    false,
                    options,
                    DecisionContinuation::GrislySalvage { player, revealed },
                );
            }
            CardBehavior::Balance => self.resolve_balance(object.controller),
            CardBehavior::Recall => {
                let player = object.controller;
                // Discarding is part of the resolution, not a cost, so a
                // countered Recall costs nothing and the opponent never sees
                // the discard before deciding whether to counter.
                let count = usize::from(object.x()).min(self.players[player.index()].hand.len());
                if count == 0 {
                    return;
                }
                let options = self.card_decision_options(
                    &self.players[player.index()].hand.clone(),
                    DecisionZone::Hand,
                );
                self.queue_decision(
                    player,
                    format!("Discard {count} card(s)"),
                    DecisionVisibility::Private,
                    DecisionPreference::LowerCardValue,
                    count..=count,
                    false,
                    options,
                    DecisionContinuation::RecallDiscard { player },
                );
            }
            _ => {}
        }
    }

    pub(super) fn queue_recall_return(&mut self, player: PlayerId, count: usize) {
        let options = self.card_decision_options(
            &self.players[player.index()].graveyard,
            DecisionZone::Graveyard,
        );
        let count = count.min(options.len());
        if count == 0 {
            return;
        }
        self.queue_decision(
            player,
            format!("Return {count} card(s) from your graveyard"),
            DecisionVisibility::Private,
            DecisionPreference::HigherCardValue,
            count..=count,
            false,
            options,
            DecisionContinuation::RecallReturn { player },
        );
    }

    /// Lifts the top `count` cards off a library, fewer if it is short, in
    /// top-first order. A library keeps its top at the end, which is the end
    /// a draw takes from, so taking from the front would have handed back the
    /// bottom of the deck instead.
    ///
    /// Revealing them is informational only; nothing yet keys off having seen
    /// a card, so the mechanical effect is where they end up.
    pub(super) fn take_top_of_library(
        &mut self,
        player: PlayerId,
        count: usize,
    ) -> Vec<CardInstance> {
        let library = &mut self.players[player.index()].library;
        let taken = count.min(library.len());
        let remaining = library.len() - taken;
        let mut cards = library.split_off(remaining);
        cards.reverse();
        cards
    }

    /// Where the pile the controller did not take ends up. The library end
    /// that counts as the top is the back of the vector, so bottoming inserts
    /// at the front.
    pub(super) fn place_revealed_remainder(
        &mut self,
        player: PlayerId,
        cards: Vec<CardInstance>,
        zone: ZoneKind,
        placement: ZonePlacement,
    ) {
        match zone {
            ZoneKind::Hand => {
                for card in cards {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
            }
            ZoneKind::Library => {
                match placement {
                    // These arrive top-first, and the back of the vector is
                    // the top, so they go back on in reverse -- otherwise a
                    // group put back on top comes back inverted, which is
                    // wrong for anything that says "in the same order" and
                    // for a look that moved nothing at all.
                    ZonePlacement::Top => {
                        for card in cards.into_iter().rev() {
                            self.players[player.index()].library.push(card);
                        }
                    }
                    ZonePlacement::Bottom => {
                        for card in cards {
                            self.players[player.index()].library.insert(0, card);
                        }
                    }
                }
            }
            ZoneKind::Exile => {
                for card in cards {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].exile.push(card);
                }
            }
            ZoneKind::Graveyard => self.bury_cards(player, cards),
            // Under the player who looked, which is what "you may put it
            // onto the battlefield" means -- the card is still theirs and
            // was never cast.
            ZoneKind::Battlefield => {
                for card in cards {
                    let _ = self.put_card_onto_battlefield_from(
                        card,
                        ZoneKind::Library,
                        super::BattlefieldArrival::under(player),
                        None,
                    );
                }
            }
            ZoneKind::Stack | ZoneKind::Command => {
                debug_assert!(false, "unsupported revealed-card destination: {zone:?}");
                self.bury_cards(player, cards);
            }
        }
    }

    /// Take from the top until one matches, publicly reveal every card, and
    /// move the passed cards plus the match to their printed destinations.
    ///
    /// The returned targets are the new identities of cards put into a
    /// graveyard, in reveal order. The count includes the matching card even
    /// when it goes somewhere else, and includes every card in a library that
    /// contains no match.
    pub(super) fn mill_until_matching(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        matched_zone: ZoneKind,
        source: GameObjectId,
    ) -> (Vec<Target>, u16) {
        let mut revealed = Vec::new();
        let mut matched_card = None;
        while let Some(card) = self.players[player.index()].library.pop() {
            if self.card_object_matches(predicate, &card, ZoneKind::Library, source) {
                matched_card = Some(card);
                break;
            }
            revealed.push(card);
        }
        let revealed_count = revealed
            .len()
            .saturating_add(usize::from(matched_card.is_some()));
        self.events
            .extend(revealed.iter().chain(matched_card.iter()).map(|card| {
                GameEvent::CardRevealed {
                    player,
                    card: card.id,
                    definition: card.definition,
                }
            }));
        // The match keeps its own destination; a library with nothing
        // matching found no match and buries everything it passed.
        match matched_card {
            Some(card) if matched_zone == ZoneKind::Graveyard => revealed.push(card),
            Some(card) => {
                self.place_revealed_remainder(player, vec![card], matched_zone, ZonePlacement::Top);
            }
            None => {}
        }
        let buried = self.bury_cards_with_ids(player, revealed);
        (buried, u16::try_from(revealed_count).unwrap_or(u16::MAX))
    }

    pub(super) fn bury_cards(&mut self, player: PlayerId, cards: Vec<CardInstance>) {
        let _ = self.bury_cards_with_ids(player, cards);
    }

    fn bury_cards_with_ids(&mut self, player: PlayerId, cards: Vec<CardInstance>) -> Vec<Target> {
        let mut buried = Vec::with_capacity(cards.len());
        for card in cards {
            let (card, _zone_change) = self.zone_change_card(card);
            buried.push(Target::Card(card.id));
            self.put_card_into_graveyard(player, card);
        }
        buried
    }

    /// Discards at random from the cards in hand that match, leaving the
    /// rest alone. A hand with nothing matching discards nothing, which is
    /// what "discards a creature card at random" does to a hand of lands.
    pub(super) fn discard_random_matching(
        &mut self,
        player: PlayerId,
        count: u16,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        cause: ZoneMoveCause,
    ) {
        let mut matching: Vec<_> = self.players[player.index()]
            .hand
            .iter()
            .filter(|card| self.card_object_matches(predicate, card, ZoneKind::Hand, source))
            .map(|card| card.id)
            .collect();
        let mut discarded = Vec::new();
        for _ in 0..usize::from(count).min(matching.len()) {
            let index = self.rng.index_below(matching.len());
            discarded.push(matching.swap_remove(index));
        }
        self.discard_cards_with_cause(player, &discarded, cause);
    }

    pub(super) fn discard_random(&mut self, player: PlayerId, count: u16, cause: ZoneMoveCause) {
        self.rng.shuffle(&mut self.players[player.index()].hand);
        let hand_count = u16::try_from(self.players[player.index()].hand.len()).unwrap_or(u16::MAX);
        let discard_count = count.min(hand_count);
        let discarded = self.players[player.index()]
            .hand
            .iter()
            .rev()
            .take(usize::from(discard_count))
            .map(|card| card.id)
            .collect::<Vec<_>>();
        self.discard_cards_with_cause(player, &discarded, cause);
    }

    pub(super) fn resolve_balance(&mut self, controller: PlayerId) {
        self.queue_balance_phase(controller, BalancePhase::Lands);
    }

    pub(super) fn queue_balance_phase(&mut self, controller: PlayerId, phase: BalancePhase) {
        let mut tasks = self.balance_tasks(controller, phase);
        if tasks.is_empty() {
            if let Some(next) = phase.next() {
                self.queue_balance_phase(controller, next);
            }
            return;
        }
        let first = tasks.remove(0);
        self.queue_balance_task(controller, phase, first, tasks);
    }

    pub(super) fn balance_tasks(
        &self,
        controller: PlayerId,
        phase: BalancePhase,
    ) -> Vec<BalanceTask> {
        let mut tasks = Vec::new();
        if phase == BalancePhase::Hands {
            let keep = self.players[0].hand.len().min(self.players[1].hand.len());
            for player in [self.active_player, self.active_player.opponent()] {
                let count = self.players[player.index()].hand.len().saturating_sub(keep);
                if count > 0 {
                    tasks.push(BalanceTask {
                        player,
                        prompt: format!("Choose {count} card(s) to discard to Balance"),
                        zone: DecisionZone::Hand,
                        cards: self.players[player.index()]
                            .hand
                            .iter()
                            .map(|card| {
                                (
                                    card.id,
                                    ObjectCharacteristics::card(
                                        card.definition,
                                        CardPartId::PRIMARY,
                                    ),
                                )
                            })
                            .collect(),
                        count,
                        action: BalanceAction::Discard,
                        cause: ZoneMoveCause::Effect { controller },
                    });
                }
            }
            return tasks;
        }

        let card_type = match phase {
            BalancePhase::Lands => CardType::Land,
            BalancePhase::Creatures => CardType::Creature,
            BalancePhase::Hands => unreachable!("the hand phase returned above"),
        };
        let counts = [self.active_player, self.active_player.opponent()].map(|player| {
            self.battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && if card_type == CardType::Creature {
                            self.power(permanent).is_some()
                        } else {
                            self.permanent_types(permanent)
                                .is_some_and(|types| types.contains(CardType::Land))
                        }
                })
                .count()
        });
        let keep = counts[0].min(counts[1]);
        for player in [self.active_player, self.active_player.opponent()] {
            let cards = self
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && if card_type == CardType::Creature {
                            self.power(permanent).is_some()
                        } else {
                            self.permanent_types(permanent)
                                .is_some_and(|types| types.contains(CardType::Land))
                        }
                })
                .map(|permanent| (permanent.card.id, Self::effective_rules_source(permanent)))
                .collect::<Vec<_>>();
            let count = cards.len().saturating_sub(keep);
            if count > 0 {
                tasks.push(BalanceTask {
                    player,
                    prompt: format!(
                        "Choose {count} {} to sacrifice to Balance",
                        if card_type == CardType::Land {
                            "land(s)"
                        } else {
                            "creature(s)"
                        }
                    ),
                    zone: DecisionZone::Battlefield,
                    cards,
                    count,
                    action: BalanceAction::Sacrifice,
                    cause: ZoneMoveCause::Effect { controller },
                });
            }
        }
        tasks
    }
}
