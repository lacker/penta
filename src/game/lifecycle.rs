use super::{
    BTreeMap, CardCatalog, CardDefinitionId, CardInstance, CharacteristicSource, CombatDamageStage,
    ContinuousEffectTimestamp, CounterKind, Deck, Format, Game, GameError, GameEvent, GameObjectId,
    GameStack, ManaPool, ObjectBacking, Permanent, PermanentLastKnownInformation, PhysicalCard,
    PhysicalCardId, PlayerId, PlayerState, Pregame, ReplayRng, RetiredObject, StackObject, Step,
    ValueDef, VecDeque, ZoneChangeOutcome, remove_card,
};

impl Game {
    /// Creates a game, shuffles both decks, and draws opening hands.
    ///
    /// Player one takes the first turn and skips that turn's draw. Mulligans
    /// are not yet part of this constructor.
    ///
    /// # Errors
    ///
    /// Returns [`GameError`] if a deck references a card absent from the
    /// supplied catalog, card instance IDs are exhausted, or a deck cannot
    /// supply an opening hand.
    pub fn new(catalog: CardCatalog, decks: [Deck; 2], seed: u64) -> Result<Self, GameError> {
        Self::new_with_format(Format::OldSchool9394, catalog, decks, seed)
    }

    /// Creates a game using the construction and gameplay rules of `format`.
    ///
    /// # Errors
    ///
    /// Returns [`GameError`] if a deck is illegal in the selected format,
    /// references a card absent from the supplied catalog, exhausts card
    /// instance IDs, or cannot supply an opening hand.
    #[allow(clippy::too_many_lines)]
    pub fn new_with_format(
        format: Format,
        catalog: CardCatalog,
        decks: [Deck; 2],
        seed: u64,
    ) -> Result<Self, GameError> {
        let mut rng = ReplayRng::new(seed);
        let mut next_physical_id = 0_u32;
        let mut next_object_id = 0_u32;
        let mut physical_cards = Vec::new();
        let [deck_one, deck_two] = decks;
        let deck_one = deck_one
            .validate_for_format(&catalog, format)
            .map_err(|error| GameError::InvalidDeck {
                player: PlayerId::One,
                error,
            })?;
        let deck_two = deck_two
            .validate_for_format(&catalog, format)
            .map_err(|error| GameError::InvalidDeck {
                player: PlayerId::Two,
                error,
            })?;
        let (deck_one_main, deck_one_sideboard) = deck_one.into_parts();
        let (deck_two_main, deck_two_sideboard) = deck_two.into_parts();

        let format_rules = format.rules();

        let mut players = {
            let mut build_player = |player: PlayerId,
                                    definitions: Vec<CardDefinitionId>|
             -> Result<PlayerState, GameError> {
                let mut library = Vec::with_capacity(definitions.len());
                for definition in definitions {
                    let physical_id = PhysicalCardId(next_physical_id);
                    next_physical_id = next_physical_id
                        .checked_add(1)
                        .ok_or(GameError::TooManyCards)?;
                    let object_id = GameObjectId(next_object_id);
                    next_object_id = next_object_id
                        .checked_add(1)
                        .ok_or(GameError::TooManyCards)?;
                    physical_cards.push(PhysicalCard {
                        id: physical_id,
                        definition,
                        owner: player,
                    });
                    library.push(CardInstance {
                        id: object_id,
                        definition,
                        owner: player,
                        backing: ObjectBacking::Cards(vec![physical_id]),
                        characteristics: CharacteristicSource::Card(definition),
                    });
                }
                rng.shuffle(&mut library);
                let initial_hand = draw_opening_hand(&mut library, format_rules.opening_hand_size)?;
                let mut hand = Vec::with_capacity(initial_hand.len());
                for mut card in initial_hand {
                    card.id = GameObjectId(next_object_id);
                    next_object_id = next_object_id
                        .checked_add(1)
                        .ok_or(GameError::TooManyCards)?;
                    hand.push(card);
                }
                Ok(PlayerState {
                    life: i16::from(format_rules.starting_life),
                    library,
                    tried_to_draw_from_empty_library: false,
                    hand,
                    graveyard: Vec::new(),
                    exile: Vec::new(),
                    outside_game: Vec::new(),
                    mana_pool: ManaPool::default(),
                    mana: Vec::new(),
                    land_played_this_turn: false,
                })
            };

            [
                build_player(PlayerId::One, deck_one_main)?,
                build_player(PlayerId::Two, deck_two_main)?,
            ]
        };

        // Sideboards are owned cards outside the game, rather than a game
        // zone. Allocate their backing and runtime identities only after both
        // main decks and opening hands so adding a sideboard cannot perturb
        // the identities of cards that began the game in either main deck.
        for (player, definitions) in [
            (PlayerId::One, deck_one_sideboard),
            (PlayerId::Two, deck_two_sideboard),
        ] {
            for definition in definitions {
                let physical_id = PhysicalCardId(next_physical_id);
                next_physical_id = next_physical_id
                    .checked_add(1)
                    .ok_or(GameError::TooManyCards)?;
                let object_id = GameObjectId(next_object_id);
                next_object_id = next_object_id
                    .checked_add(1)
                    .ok_or(GameError::TooManyCards)?;
                physical_cards.push(PhysicalCard {
                    id: physical_id,
                    definition,
                    owner: player,
                });
                players[player.index()].outside_game.push(CardInstance {
                    id: object_id,
                    definition,
                    owner: player,
                    backing: ObjectBacking::Cards(vec![physical_id]),
                    characteristics: CharacteristicSource::Card(definition),
                });
            }
        }

        Ok(Self {
            format,
            seed,
            rng,
            catalog,
            physical_cards,
            players,
            battlefield: Vec::new(),
            stack: GameStack::default(),
            retired_objects: BTreeMap::new(),
            temporary_ability_grants: Vec::new(),
            next_object_id,
            next_continuous_effect_timestamp: u64::from(next_object_id),
            turn: 1,
            turns_started: [1, 0],
            active_player: PlayerId::One,
            priority: PlayerId::One,
            consecutive_passes: 0,
            step: Step::Upkeep,
            attackers_declared: false,
            creature_died_this_turn: false,
            linked_exiles: Vec::new(),
            sorcery_flash_grants: [0; 2],
            additional_combat_phases: 0,
            noncreature_casts_locked: [false; 2],
            emblems: Vec::new(),
            spells_cast_this_turn: [0; 2],
            spells_cast_last_turn: [0; 2],
            cards_drawn_this_turn: [0; 2],
            drawn_this_turn: [Vec::new(), Vec::new()],
            defer_empty_library_loss: false,
            draw_replacements: std::array::from_fn(|_| VecDeque::new()),
            miracle_window: None,
            delayed_triggers: Vec::new(),
            floating_triggers: Vec::new(),
            blockers_declared: false,
            untap_pending: false,
            pregame: Some(Pregame::Mulligan(PlayerId::One)),
            mulligans: [0, 0],
            cleanup_pending: false,
            pending_decisions: Vec::new(),
            next_decision_id: 0,
            pending_events: VecDeque::new(),
            pending_procedures: VecDeque::new(),
            pending_triggers: Vec::new(),
            next_trigger_id: 0,
            last_seen_hands: [None, None],
            pending_combat_attackers: Vec::new(),
            combat_damage_stage: CombatDamageStage::NotStarted,
            combat_blocked_attackers: Vec::new(),
            extra_turns: Vec::new(),
            channel_active: [false, false],
            skipped_turns: [0, 0],
            result: None,
            events: vec![GameEvent::GameStarted { seed }],
        })
    }

    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
    }

    /// The seed the libraries were shuffled from. It reproduces the whole
    /// game, so it belongs to whoever owns the engine rather than to a seat.
    /// [`Self::events_for`] keeps it out of a seat's event stream.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns the printed definition associated with one physical card.
    #[must_use]
    #[cfg(test)]
    pub(super) fn physical_card_definition(&self, id: PhysicalCardId) -> Option<CardDefinitionId> {
        self.physical_cards
            .iter()
            .find(|card| card.id == id)
            .map(|card| card.definition)
    }

    /// Returns the owner of one physical card. Object control and copied
    /// characteristics are intentionally independent of this value.
    #[must_use]
    #[cfg(test)]
    pub(super) fn physical_card_owner(&self, id: PhysicalCardId) -> Option<PlayerId> {
        self.physical_cards
            .iter()
            .find(|card| card.id == id)
            .map(|card| card.owner)
    }

    pub(super) fn allocate_object_id(&mut self) -> GameObjectId {
        let id = GameObjectId(self.next_object_id);
        self.next_object_id = self
            .next_object_id
            .checked_add(1)
            .expect("game object IDs exhausted");
        id
    }

    pub(super) fn allocate_continuous_effect_timestamp(&mut self) -> ContinuousEffectTimestamp {
        let observed_next = self
            .battlefield
            .iter()
            .chain(self.emblems.iter())
            .map(|permanent| permanent.timestamp.0)
            .chain(self.battlefield.iter().flat_map(|permanent| {
                permanent
                    .temporary_granted_abilities
                    .iter()
                    .map(|effect| effect.timestamp.0)
                    .chain(
                        permanent
                            .temporary_removed_abilities
                            .iter()
                            .map(|effect| effect.timestamp.0),
                    )
            }))
            .max()
            .map_or(0, |timestamp| timestamp.saturating_add(1));
        self.next_continuous_effect_timestamp =
            self.next_continuous_effect_timestamp.max(observed_next);
        let timestamp = ContinuousEffectTimestamp(self.next_continuous_effect_timestamp);
        self.next_continuous_effect_timestamp = self
            .next_continuous_effect_timestamp
            .checked_add(1)
            .expect("continuous-effect timestamps exhausted");
        timestamp
    }

    pub(super) fn zone_change_card(
        &mut self,
        mut card: CardInstance,
    ) -> (CardInstance, ZoneChangeOutcome) {
        let previous = card.id;
        self.retired_objects
            .entry(previous)
            .or_insert_with(|| RetiredObject::Card(card.clone()));
        card.id = self.allocate_object_id();
        let created = vec![card.id];
        (card, ZoneChangeOutcome { previous, created })
    }

    pub(super) fn remove_battlefield_object(
        &mut self,
        index: usize,
        last_known: &PermanentLastKnownInformation,
    ) -> Permanent {
        let permanent = self.battlefield.remove(index);
        self.retired_objects.insert(
            permanent.card.id,
            RetiredObject::Permanent {
                permanent: Box::new(permanent.clone()),
                power: last_known.power,
                toughness: last_known.toughness,
                mana_value: last_known.mana_value,
                keywords: last_known.keywords.clone(),
            },
        );
        permanent
    }

    pub(super) fn retire_stack_object(&mut self, object: &StackObject) {
        self.retired_objects
            .insert(object.id, RetiredObject::Stack(Box::new(object.clone())));
    }

    pub(super) fn current_or_last_known_power(&self, object: GameObjectId) -> Option<i16> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| self.power(permanent))
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { power, .. }) => *power,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// How many counters of one kind an object has, using last-known
    /// information once it has left the battlefield. An ability whose cost
    /// sacrificed its own source still reads the counters it had.
    pub(super) fn current_or_last_known_counters(
        &self,
        object: GameObjectId,
        kind: CounterKind,
    ) -> u16 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map_or_else(
                || match self.retired_objects.get(&object) {
                    Some(RetiredObject::Permanent { permanent, .. }) => permanent.counters(kind),
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => 0,
                },
                |permanent| permanent.counters(kind),
            )
    }

    /// Whether an object is tapped, using its last existence on the
    /// battlefield after it has left. Intervening-if conditions re-read this
    /// information as their abilities resolve.
    pub(super) fn current_or_last_known_tapped(&self, object: GameObjectId) -> bool {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map_or_else(
                || match self.retired_objects.get(&object) {
                    Some(RetiredObject::Permanent { permanent, .. }) => permanent.tapped,
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => false,
                },
                |permanent| permanent.tapped,
            )
    }

    /// The values a predicate can read while matching, where the only context
    /// is the ability's source. Anything wider stays outside the boundary.
    pub(super) fn value_from_source(&self, value: ValueDef, source: GameObjectId) -> Option<i32> {
        match value {
            ValueDef::Constant(amount) => Some(amount),
            ValueDef::CountersOnSource(kind) => {
                Some(i32::from(self.current_or_last_known_counters(source, kind)))
            }
            // A spell is its own source, so its chosen X is right there --
            // by way of the retired record, because a spell leaves the stack
            // before its effect runs. An activated ability's source is the
            // permanent instead, and its X is not reachable from a predicate.
            ValueDef::ChosenX => self
                .stack
                .iter()
                .find(|object| object.id == source)
                .map(|object| i32::from(object.x()))
                .or_else(|| match self.retired_objects.get(&source) {
                    Some(RetiredObject::Stack(object)) => Some(i32::from(object.x())),
                    Some(RetiredObject::Card(_) | RetiredObject::Permanent { .. }) | None => None,
                }),
            // Read live, so pumping the source widens what its ability can
            // reach. This is safe here because a target predicate is not
            // consulted while static effects are being applied; a static
            // ability whose own recipient predicate read this would not
            // terminate.
            ValueDef::SourcePower => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .and_then(|permanent| self.power(permanent))
                .map(i32::from),
            _ => None,
        }
    }

    pub(super) fn current_or_last_known_toughness(&self, object: GameObjectId) -> Option<i16> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| self.toughness(permanent))
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { toughness, .. }) => *toughness,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    pub(super) fn current_or_last_known_controller(
        &self,
        object: GameObjectId,
    ) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.controller)
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|candidate| candidate.id == object)
                    .map(|candidate| candidate.controller)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.controller),
                Some(RetiredObject::Stack(stack)) => Some(stack.controller),
                Some(RetiredObject::Card(_)) | None => None,
            })
    }

    pub(super) fn unbacked_object(
        &mut self,
        definition: CardDefinitionId,
        owner: PlayerId,
        characteristics: CharacteristicSource,
    ) -> CardInstance {
        CardInstance {
            id: self.allocate_object_id(),
            definition,
            owner,
            backing: ObjectBacking::None,
            characteristics,
        }
    }

    pub(super) fn keep_hand(&mut self, player: PlayerId) {
        if self.mulligans[player.index()] > 0 {
            self.pregame = Some(Pregame::Bottom(player));
        } else {
            self.advance_pregame(player);
        }
    }

    pub(super) fn take_mulligan(&mut self, player: PlayerId) {
        let hand = std::mem::take(&mut self.players[player.index()].hand);
        for card in hand {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].library.push(card);
        }
        self.rng.shuffle(&mut self.players[player.index()].library);
        let initial_hand = draw_opening_hand(
            &mut self.players[player.index()].library,
            self.format.rules().opening_hand_size,
        )
        .expect("a validated deck always contains at least seven cards");
        for card in initial_hand {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].hand.push(card);
        }
        self.mulligans[player.index()] = self.mulligans[player.index()].saturating_add(1);
    }

    pub(super) fn bottom_cards(&mut self, player: PlayerId, cards: &[GameObjectId]) {
        for id in cards.iter().rev() {
            if let Some(card) = remove_card(&mut self.players[player.index()].hand, *id) {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[player.index()].library.insert(0, card);
            }
        }
        self.advance_pregame(player);
    }

    pub(super) fn advance_pregame(&mut self, player: PlayerId) {
        if player == PlayerId::One {
            self.pregame = Some(Pregame::Mulligan(PlayerId::Two));
            self.priority = PlayerId::Two;
        } else {
            self.pregame = None;
            self.priority = PlayerId::One;
        }
    }
}

#[cfg(test)]
pub(super) fn backing_cards(backing: &ObjectBacking) -> Vec<PhysicalCardId> {
    match backing {
        ObjectBacking::Cards(cards) => cards.clone(),
        ObjectBacking::None => Vec::new(),
    }
}

pub(super) fn draw_opening_hand(
    library: &mut Vec<CardInstance>,
    opening_hand_size: usize,
) -> Result<Vec<CardInstance>, GameError> {
    if library.len() < opening_hand_size {
        return Err(GameError::NotEnoughCardsForOpeningHand);
    }
    let split_at = library.len() - opening_hand_size;
    Ok(library.split_off(split_at))
}
