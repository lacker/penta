use super::{
    BTreeMap, CardCatalog, CardDefinitionId, CardInstance, CharacteristicSource, CombatDamageStage,
    ContinuousEffectTimestamp, CounterKind, DamageSourceGroupDef, Deck, Format, Game, GameError,
    GameEvent, GameObjectId, GameStack, ManaPool, ObjectBacking, ObjectCharacteristics,
    ObjectInstance, ObjectKind, Permanent, PermanentLastKnownInformation, PhysicalCard,
    PhysicalCardId, PlayerId, PlayerState, Pregame, ReplayRng, RetiredObject, StackObject, Step,
    TriggerContext, ValueDef, VecDeque, ZoneChangeOutcome, remove_card,
};
use crate::card::PlayerRelation;

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
                        counters: [0; CounterKind::COUNT],
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
                    lands_played_this_turn: 0,
                    poison: 0,
                    energy: 0,
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
                    counters: [0; CounterKind::COUNT],
                });
            }
        }

        Ok(Self {
            format,
            arrived: None,
            prospective_x: super::prospective_x::ProspectiveX::default(),
            successors: std::collections::HashMap::new(),
            seed,
            rng,
            catalog,
            physical_cards,
            players,
            battlefield: Vec::new(),
            phased_out: Vec::new(),
            stack: GameStack::default(),
            retired_objects: BTreeMap::new(),
            temporary_ability_grants: Vec::new(),
            ongoing_effects: Vec::new(),
            next_object_id,
            next_continuous_effect_timestamp: u64::from(next_object_id),
            turn: 1,
            turns_started: [1, 0],
            damage_taken_this_turn: [0; 2],
            damage_taken_by_group_this_turn: [[0; DamageSourceGroupDef::COUNT]; 2],
            active_player: PlayerId::One,
            priority: PlayerId::One,
            consecutive_passes: 0,
            step: Step::Upkeep,
            attackers_declared: false,
            creature_died_this_turn: false,
            creatures_died_this_turn: 0,
            linked_exiles: Vec::new(),
            monarch: None,
            ninjutsu_returned_defender: None,
            exile_play_permissions: Vec::new(),
            damage_cannot_be_prevented_this_turn: false,
            sorcery_flash_grants: [0; 2],
            turn_phase_queue: VecDeque::new(),
            turn_phase_resume: None,
            resolved_play_restrictions: Vec::new(),
            resolved_attack_restrictions: Vec::new(),
            resolved_play_permissions: Vec::new(),
            emblems: Vec::new(),
            spells_cast_this_turn: [0; 2],
            spells_cast_last_turn: [0; 2],
            cards_drawn_this_turn: [0; 2],
            citys_blessing: [false; 2],
            permanent_left_battlefield_this_turn: [false; 2],
            card_left_graveyard_this_turn: [false; 2],
            life_gained_this_turn: [0; 2],
            draw_step_draw_taken: [false; 2],
            drawn_this_turn: [Vec::new(), Vec::new()],
            defer_empty_library_loss: false,
            draw_replacements: std::array::from_fn(|_| VecDeque::new()),
            damage_preventions: Vec::new(),
            damage_redirects: Vec::new(),
            installed_triggers: Vec::new(),
            next_installed_trigger_id: 0,
            blockers_declared: false,
            untap_pending: false,
            pregame: Some(Pregame::Mulligan(PlayerId::One)),
            mulligans: [0, 0],
            cleanup_pending: false,
            pending_decisions: Vec::new(),
            pending_discard_follow_up: None,
            next_decision_id: 0,
            pending_events: VecDeque::new(),
            pending_procedures: VecDeque::new(),
            pending_triggers: Vec::new(),
            next_trigger_id: 0,
            last_seen_hands: [None, None],
            pending_combat_assignments: Vec::new(),
            combat_damage_stage: CombatDamageStage::NotStarted,
            combat_blocked_attackers: Vec::new(),
            next_regular_player: PlayerId::Two,
            extra_turns: Vec::new(),
            channel_active: [false, false],
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
            .chain(
                self.battlefield
                    .iter()
                    .chain(self.emblems.iter())
                    .flat_map(|permanent| {
                        permanent
                            .resolved_continuous_effects
                            .iter()
                            .map(|effect| effect.timestamp.0)
                    }),
            )
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
        card.counters = [0; CounterKind::COUNT];
        self.successors.insert(previous, card.id);
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

    /// The source recorded on a stack object that has already left the stack.
    /// A countered ability is retired with its source intact, which is how
    /// "destroy that permanent" finds the permanent afterwards.
    pub(super) fn retired_stack_object_source(&self, object: GameObjectId) -> Option<GameObjectId> {
        match self.retired_objects.get(&object) {
            Some(RetiredObject::Stack(stack)) => stack.source,
            _ => None,
        }
    }

    pub(super) fn current_or_last_known_power(&self, object: GameObjectId) -> Option<i16> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| self.power(permanent))
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { power, .. }) => *power,
                // A card that was never a permanent still has a power to
                // read, and nothing in a graveyard or exile modifies it, so
                // the printed value is the whole answer. Scavenge asks this
                // of a card it has already exiled to pay its own cost.
                Some(RetiredObject::Card(card)) => self.printed_card_power(card),
                Some(RetiredObject::Stack(_)) | None => self
                    .card_in_nonbattlefield_zone(object)
                    .and_then(|(_, card)| self.printed_card_power(card)),
            })
    }

    fn printed_card_power(&self, card: &CardInstance) -> Option<i16> {
        self.catalog
            .get(card.definition)
            .and_then(|definition| definition.rules.creature_stats())
            .map(|stats| stats.power)
    }

    /// The object an Aura was attached to immediately before it left the
    /// battlefield. Activated abilities are independent of their source once
    /// on the stack, so sacrificing the Aura as a cost or removing it in
    /// response must not erase what "enchanted permanent" means.
    pub(super) fn current_or_last_known_attached_host(
        &self,
        object: GameObjectId,
    ) -> Option<GameObjectId> {
        self.attached_host(object)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => permanent.attached_to,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// What a permanent was blocking, using last-known information once it
    /// has left the battlefield. A creature that died in combat still knows
    /// what it had blocked, which is what a death trigger reading "creatures
    /// blocked by it" has to see.
    pub(super) fn current_or_last_known_blocking(
        &self,
        object: GameObjectId,
    ) -> Option<GameObjectId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| permanent.blocking.first().copied())
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    permanent.blocking.first().copied()
                }
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
                })
                // A spell being cast has no stack object yet, and "target
                // creature with power X or less" has to be answered before
                // it gets one.
                .or_else(|| self.prospective_x.get().map(i32::from)),
            // Converge, reached the same way and for the same reason: the
            // spell is its own source, and it has left the stack by the time
            // its effect asks what paid for it.
            ValueDef::ColorsOfManaSpent => self
                .stack
                .iter()
                .find(|object| object.id == source)
                .map(|object| i32::from(object.colors_spent_count()))
                .or_else(|| match self.retired_objects.get(&source) {
                    Some(RetiredObject::Stack(object)) => {
                        Some(i32::from(object.colors_spent_count()))
                    }
                    Some(RetiredObject::Card(_) | RetiredObject::Permanent { .. }) | None => None,
                }),
            // Pumping the source widens the predicate while it is live. If
            // source and triggering object leave simultaneously, use the
            // same last-known power frozen for the rest of the trigger.
            ValueDef::SourcePower => self.current_or_last_known_power(source).map(i32::from),
            ValueDef::SourceToughness => {
                self.current_or_last_known_toughness(source).map(i32::from)
            }
            ValueDef::CardsInHandAbove { player, threshold } => {
                let controller = self.current_or_last_known_controller(source)?;
                let counted = if player == PlayerRelation::ControllerOfAttachedPermanent {
                    self.attached_host_controller_of(source)
                        .unwrap_or(controller)
                } else {
                    [PlayerId::One, PlayerId::Two]
                        .into_iter()
                        .find(|candidate| {
                            self.player_relation_matches(
                                *candidate,
                                player,
                                controller,
                                TriggerContext::empty(),
                            )
                        })
                        .unwrap_or(controller)
                };
                Some(
                    i32::try_from(
                        self.players[counted.index()]
                            .hand
                            .len()
                            .saturating_sub(usize::from(threshold)),
                    )
                    .unwrap_or(i32::MAX),
                )
            }
            // "Power X or less" is said as "below X plus one", so a sum has
            // to be reachable from here as well as its parts.
            ValueDef::Sum(sum) => self
                .value_from_source(sum.left, source)
                .zip(self.value_from_source(sum.right, source))
                .map(|(left, right)| left.saturating_add(right)),
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
            .chain(self.emblems.iter())
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

    pub(super) fn current_or_last_known_owner(&self, object: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.card.owner)
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|candidate| candidate.id == object)
                    .map(|candidate| candidate.card.owner)
            })
            .or_else(|| {
                self.card_in_nonbattlefield_zone(object)
                    .map(|(_, card)| card.owner)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Card(card)) => Some(card.owner),
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.card.owner),
                Some(RetiredObject::Stack(stack)) => Some(stack.card.owner),
                None => None,
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
            counters: [0; CounterKind::COUNT],
        }
    }

    pub(super) fn unbacked_ability_object(
        &mut self,
        presentation: ObjectCharacteristics,
        owner: PlayerId,
    ) -> ObjectInstance {
        let characteristics = match presentation {
            ObjectCharacteristics::Card { definition, .. } => {
                CharacteristicSource::Ability(definition)
            }
            ObjectCharacteristics::Token { token, .. } => CharacteristicSource::Token(token),
            ObjectCharacteristics::Emblem { emblem } => CharacteristicSource::Emblem(emblem),
            ObjectCharacteristics::FaceDown { face_down } => {
                CharacteristicSource::FaceDown(face_down)
            }
        };
        ObjectInstance {
            id: self.allocate_object_id(),
            definition: ObjectKind::Ability,
            owner,
            backing: ObjectBacking::None,
            characteristics,
            counters: [0; CounterKind::COUNT],
        }
    }

    /// Mints the object shell for a creator-owned command-zone emblem.
    pub(super) fn unbacked_emblem_object(
        &mut self,
        emblem: crate::EmblemCharacteristics,
        owner: PlayerId,
    ) -> ObjectInstance {
        ObjectInstance {
            id: self.allocate_object_id(),
            definition: ObjectKind::Emblem,
            owner,
            backing: ObjectBacking::None,
            characteristics: CharacteristicSource::Emblem(emblem),
            counters: [0; CounterKind::COUNT],
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
