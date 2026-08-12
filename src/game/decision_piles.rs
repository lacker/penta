use super::{
    BalancePhase, BalanceTask, CardBehavior, CardInstance, CardRuntime, CommittedTriggerEvent,
    CounterKind, DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, DeclarativeAbilityDef, EffectDef, Game, GameObjectId, ObjectPredicateDef,
    Permanent, PileChoice, PileChosen, PileSplit, PilesSeparated, PlayerId, SacrificeFollowup,
    Step, TriggerContext, ZoneKind, ZoneMoveCause, ZonePlacement,
};

impl Game {
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
                card: Some((card.id, card.definition)),
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
                    .map_or_else(|| "Unknown permanent".into(), str::to_owned);
                Some(DecisionOption {
                    id: u32::try_from(index).unwrap_or(u32::MAX),
                    label,
                    card: Some((permanent.card.id, permanent.card.definition)),
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
            on_complete(
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
            on_complete(
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
        let options = self.card_decision_options(&task.cards, task.zone);
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
    pub(super) fn queue_effect_discards(
        &mut self,
        mut players: Vec<PlayerId>,
        amount: i32,
        cause: ZoneMoveCause,
    ) {
        let amount = usize::try_from(amount).unwrap_or(0);
        if amount == 0 || players.is_empty() {
            return;
        }
        players.sort_by_key(|player| (*player != self.active_player, player.index()));
        players.dedup();
        let first = players.remove(0);
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
        for (player, cards) in chosen {
            self.discard_cards_with_cause(player, &cards, cause);
        }
    }

    /// Whether a spell or ability an opponent of `player` controls can make
    /// them sacrifice a permanent. Sigarda says it cannot.
    pub(super) fn can_be_forced_to_sacrifice(&self, player: PlayerId, caused_by: PlayerId) -> bool {
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
                            && effective.ability.declarative_effect()
                                == Some(EffectDef::CannotBeForcedToSacrifice)
                    })
                    .is_some()
        })
    }

    /// Asks `player` which matching permanent they control to sacrifice. With
    /// nothing matching there is no choice and no sacrifice; with exactly one
    /// there is nothing to ask.
    /// Offers the top card of a library to its owner when it matches. Only a
    /// matching card is ever shown, because the decision is the only thing
    /// the other player can see and a non-matching card must stay secret.
    pub(super) fn queue_top_card_offer(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) {
        let Some(top) = self.players[player.index()].library.last() else {
            return;
        };
        if !self.card_object_matches(predicate, top, ZoneKind::Library, source) {
            return;
        }
        let options = self.card_decision_options(std::slice::from_ref(top), DecisionZone::Library);
        self.queue_decision(
            player,
            "Reveal the top card and put it into your hand?",
            DecisionVisibility::Private,
            DecisionPreference::HigherCardValue,
            0..=1,
            false,
            options,
            DecisionContinuation::LibrarySearch {
                destination: ZoneKind::Hand,
                shuffle: false,
            },
        );
    }

    /// Offers a library search over the cards a predicate admits. The shuffle
    /// happens either way, because a search that finds nothing still searched
    /// and skipping it would hand the searcher their library order for free.
    pub(super) fn queue_library_search(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        destination: ZoneKind,
        source: GameObjectId,
    ) {
        let options = self.players[player.index()]
            .library
            .iter()
            .filter(|card| self.card_object_matches(predicate, card, ZoneKind::Library, source))
            .enumerate()
            .map(|(index, card)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self
                    .catalog
                    .get(card.definition)
                    .map_or_else(|| "Unknown card".into(), |card| card.name.clone()),
                card: Some((card.id, card.definition)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Library,
            })
            .collect();
        self.queue_decision(
            player,
            match destination {
                ZoneKind::Battlefield => {
                    "Choose a card to put onto the battlefield, or fail to find"
                }
                _ => "Choose a card to put into your hand, or fail to find",
            },
            DecisionVisibility::Private,
            DecisionPreference::HigherCardValue,
            // CR 701.19c: searching a hidden zone never obliges the searcher
            // to find, so the minimum is zero even with a full library.
            0..=1,
            false,
            options,
            DecisionContinuation::LibrarySearch {
                destination,
                shuffle: true,
            },
        );
    }

    /// Whether a loyalty ability can be activated right now. CR 606.3: only
    /// during your own main phase with an empty stack, and only one loyalty
    /// ability per planeswalker per turn. CR 606.5: the cost cannot remove
    /// more counters than the permanent has.
    /// CR 602.5c as Pithing Needle writes it: a non-mana activated ability
    /// cannot be activated while something has named its source's card. The
    /// name is matched against the printed card, so a copy answering to the
    /// same name is locked too.
    pub(super) fn activated_abilities_are_named(&self, permanent: &Permanent) -> bool {
        let (definition, _part) = Self::effective_rules_source(permanent);
        let Some(name) = self
            .catalog
            .get(definition)
            .map(|definition| definition.name.as_str())
        else {
            return false;
        };
        self.battlefield.iter().any(|candidate| {
            candidate
                .chosen_card_name
                .as_deref()
                .is_some_and(|chosen| chosen == name)
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

    /// The ability's controller separates everything the other player
    /// controls into two piles, then that player sacrifices one. The split is
    /// recorded as the chosen pile; whatever is left is the other.
    pub(super) fn queue_pile_split(&mut self, splitter: PlayerId, owner: PlayerId) {
        let permanents = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == owner)
            .map(|permanent| permanent.card.clone())
            .collect::<Vec<_>>();
        if permanents.is_empty() {
            return;
        }
        let options = self.card_decision_options(&permanents, DecisionZone::Battlefield);
        let maximum = options.len();
        self.queue_decision(
            splitter,
            "Separate these permanents into two piles",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            0..=maximum,
            false,
            options,
            DecisionContinuation::PileSplit { owner },
        );
    }

    /// Reveals the top cards and hands the split to an opponent. The reveal
    /// takes them out of the library up front, so every path from here has to
    /// put all of them somewhere.
    pub(super) fn queue_revealed_pile_split(
        &mut self,
        player: PlayerId,
        count: usize,
        rest: ZoneKind,
        placement: ZonePlacement,
    ) {
        let revealed = self.take_top_of_library(player, count);
        if revealed.is_empty() {
            return;
        }
        let options = self.card_decision_options(&revealed, DecisionZone::Library);
        let maximum = options.len();
        self.queue_decision(
            player.opponent(),
            "Separate the revealed cards into two piles",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            0..=maximum,
            false,
            options,
            DecisionContinuation::RevealedPileSplit {
                player,
                revealed,
                rest,
                placement,
            },
        );
    }

    /// Offers the two revealed piles to the player who gets to keep one.
    pub(super) fn queue_revealed_pile_choice(
        &mut self,
        player: PlayerId,
        first: Vec<CardInstance>,
        second: Vec<CardInstance>,
        rest: ZoneKind,
        placement: ZonePlacement,
    ) {
        let describe = |game: &Self, pile: &[CardInstance]| {
            if pile.is_empty() {
                return "the empty pile".to_string();
            }
            pile.iter()
                .filter_map(|card| game.catalog.get(card.definition))
                .map(|definition| definition.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        };
        let options = [&first, &second]
            .into_iter()
            .enumerate()
            .map(|(index, pile)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: format!("Take {}", describe(self, pile)),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Library,
            })
            .collect();
        self.queue_decision(
            player,
            "Choose the pile to put into your hand",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::RevealedPileChoice {
                player,
                first,
                second,
                rest,
                placement,
            },
        );
    }

    /// Offers the split piles to the player who must give one up.
    pub(super) fn queue_pile_choice(
        &mut self,
        owner: PlayerId,
        first: Vec<GameObjectId>,
        second: Vec<GameObjectId>,
    ) {
        let describe = |game: &Self, pile: &[GameObjectId]| {
            if pile.is_empty() {
                return "the empty pile".to_string();
            }
            let names = pile
                .iter()
                .filter_map(|id| {
                    game.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == *id)
                })
                .filter_map(|permanent| game.catalog.get(permanent.card.definition))
                .map(|definition| definition.name.clone())
                .collect::<Vec<_>>();
            names.join(", ")
        };
        let options = vec![
            DecisionOption {
                id: 0,
                label: format!("Sacrifice {}", describe(self, &first)),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            },
            DecisionOption {
                id: 1,
                label: format!("Sacrifice {}", describe(self, &second)),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            },
        ];
        self.queue_decision(
            owner,
            "Choose the pile to sacrifice",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::PileChoice { first, second },
        );
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
        let definition = self.battlefield[index].card.definition;
        let Some(other) = self
            .catalog
            .get(definition)
            .and_then(|definition| definition.other_face(self.battlefield[index].presented))
        else {
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

    /// The permanents a player controls that an effect could pick out.
    pub(super) fn chosen_removal_candidates(
        &self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) -> Vec<CardInstance> {
        self.battlefield
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
            .map(|permanent| permanent.card.clone())
            .collect()
    }

    /// "Destroy target creature that player controls of their choice": the
    /// choice belongs to whoever controls the candidates, so it is asked of
    /// them rather than of the ability's controller.
    pub(super) fn queue_chosen_destruction(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        can_regenerate: bool,
    ) {
        let candidates = self.chosen_removal_candidates(player, predicate, source);
        if candidates.len() <= 1 {
            let doomed = candidates.first().map(|only| only.id);
            if let Some(doomed) = doomed {
                self.destroy_permanents(&[doomed], can_regenerate);
            }
            return;
        }
        let options = self.card_decision_options(&candidates, DecisionZone::Battlefield);
        self.queue_decision(
            player,
            "Choose a permanent to destroy",
            DecisionVisibility::Public,
            DecisionPreference::LowerCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::DestroyOfChoice { can_regenerate },
        );
    }

    pub(super) fn queue_chosen_sacrifice(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        followup: Option<SacrificeFollowup>,
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
            .map(|permanent| permanent.card.clone())
            .collect::<Vec<_>>();
        // An optional sacrifice is always a question, even with one candidate
        // or none: declining is a real answer. A compulsory one with a single
        // candidate has only one answer, so it happens without asking.
        if !optional && candidates.len() <= 1 {
            let sacrificed = candidates.first().map(|only| only.id);
            if let Some(sacrificed) = sacrificed {
                self.move_permanents_to_graveyard(&[sacrificed]);
            }
            if let Some(followup) = followup {
                self.resolve_sacrifice_followup(&followup, sacrificed);
            }
            return;
        }
        if optional && candidates.is_empty() {
            return;
        }
        let options = self.card_decision_options(&candidates, DecisionZone::Battlefield);
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
            DecisionContinuation::SacrificeOfChoice { followup, optional },
        );
    }

    /// Runs what a sacrifice owes once the permanent is chosen. The power is
    /// read before the sacrifice, because by the time this runs the permanent
    /// is already gone.
    pub(super) fn resolve_sacrifice_followup(
        &mut self,
        followup: &SacrificeFollowup,
        sacrificed: Option<GameObjectId>,
    ) {
        // A negative power gives nothing rather than draining the controller.
        let amount = i32::from(
            sacrificed
                .and_then(|id| self.current_or_last_known_power(id))
                .unwrap_or(0),
        )
        .max(0);
        let context = TriggerContext {
            amount: Some(amount),
            ..followup.context
        };
        self.resolve_effect_def(followup.effect, &followup.object, context);
    }

    pub(super) fn queue_time_vault_decision(
        &mut self,
        permanent: GameObjectId,
        remaining: Vec<GameObjectId>,
    ) {
        let card = self
            .battlefield
            .iter()
            .find(|candidate| candidate.card.id == permanent)
            .map(|permanent| (permanent.card.id, permanent.card.definition));
        self.queue_decision(
            self.active_player,
            "Time Vault would remain tapped",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Leave Time Vault tapped".into(),
                    card,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                },
                DecisionOption {
                    id: 1,
                    label: "Untap Time Vault and skip your next turn".into(),
                    card,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                },
            ],
            DecisionContinuation::TimeVault {
                permanent,
                remaining,
            },
        );
    }

    pub(super) fn finish_untap_choices(&mut self) {
        let mut vaults = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == self.active_player
                    && permanent.tapped
                    && self.effective_behavior(permanent) == Some(CardBehavior::TimeVault)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        if vaults.is_empty() {
            self.handle_upkeep_triggers();
        } else {
            let first = vaults.remove(0);
            self.queue_time_vault_decision(first, vaults);
        }
    }

    pub(super) fn queue_sylvan_select(
        &mut self,
        player: PlayerId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    ) {
        let cards = self.players[player.index()]
            .hand
            .iter()
            .filter(|card| candidates.contains(&card.id))
            .cloned()
            .collect::<Vec<_>>();
        let options = self.card_decision_options(&cards, DecisionZone::DrawnThisStep);
        self.queue_decision(
            player,
            format!("Choose a card drawn this step ({choices_left} remaining)"),
            DecisionVisibility::Private,
            DecisionPreference::LowerCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::SylvanSelect {
                player,
                candidates,
                choices_left,
            },
        );
    }

    pub(super) fn queue_sylvan_mode(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    ) {
        let card_info = self.players[player.index()]
            .hand
            .iter()
            .find(|candidate| candidate.id == card)
            .map(|card| (card.id, card.definition));
        let card_name = card_info
            .and_then(|(_, definition)| self.catalog.get(definition))
            .map_or("this card", |card| card.name.as_str());
        let mut options = vec![DecisionOption {
            id: 0,
            label: format!("Put {card_name} back on top"),
            card: card_info,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::DrawnThisStep,
        }];
        if self.players[player.index()].life >= 4 {
            options.push(DecisionOption {
                id: 1,
                label: format!("Pay 4 life to keep {card_name}"),
                card: card_info,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::DrawnThisStep,
            });
        }
        self.queue_decision(
            player,
            format!("Keep {card_name}?"),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::SylvanMode {
                player,
                card,
                candidates,
                choices_left,
            },
        );
    }
}
