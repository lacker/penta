use super::{
    BTreeMap, CardCatalog, CardDefinitionId, CardInstance, CharacteristicSource, CombatDamageStage,
    ContinuousEffectTimestamp, CounterKind, DamageSourceGroupDef, Deck, EnumeratedActions, Format,
    Game, GameError, GameEvent, GameObjectId, GameStack, ManaPool, ObjectBacking, ObjectInstance,
    ObjectKind, Permanent, PermanentLastKnownInformation, PhysicalCard, PhysicalCardId, PlayerId,
    PlayerState, Pregame, ReplayRng, RetiredObject, StackObject, Step, Target, TriggerContext,
    ValueDef, VecDeque, ZoneChangeOutcome, remove_card,
};
use crate::card::ZoneKind;

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
        Self::new_with_starting_player(format, catalog, decks, seed, PlayerId::One)
    }

    /// Creates a game with stable player identities and an explicit starting player.
    /// # Errors
    /// Returns an error for invalid decks or failed game construction.
    #[allow(clippy::too_many_lines)]
    pub fn new_with_starting_player(
        format: Format,
        catalog: CardCatalog,
        decks: [Deck; 2],
        seed: u64,
        starting_player: PlayerId,
    ) -> Result<Self, GameError> {
        Self::new_from_decks(format, catalog, decks, seed, starting_player, true, 0)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new_from_decks(
        format: Format,
        catalog: CardCatalog,
        decks: [Deck; 2],
        seed: u64,
        starting_player: PlayerId,
        validate: bool,
        first_object_id: u32,
    ) -> Result<Self, GameError> {
        let mut rng = ReplayRng::new(seed);
        let mut next_physical_id = 0_u32;
        let mut next_object_id = first_object_id;
        let mut physical_cards = Vec::new();
        let unpack = |deck: Deck, player| {
            if validate {
                deck.validate_for_format(&catalog, format)
                    .map(crate::deck::ValidatedDeck::into_parts)
                    .map_err(|error| GameError::InvalidDeck { player, error })
            } else {
                Ok((deck.main, deck.sideboard, deck.commanders))
            }
        };
        let [deck_one, deck_two] = decks;
        let (deck_one_main, deck_one_sideboard, commander_one) = unpack(deck_one, PlayerId::One)?;
        let (deck_two_main, deck_two_sideboard, commander_two) = unpack(deck_two, PlayerId::Two)?;
        let designated = [commander_one, commander_two];
        // Defer opening hands whenever a sideboard contains a Companion
        // clause. Eligibility is checked against the untouched libraries and
        // command zones when the pregame decision is built below.
        let choose_companions = deck_one_sideboard
            .iter()
            .chain(&deck_two_sideboard)
            .any(|id| {
                catalog
                    .get(*id)
                    .and_then(crate::card::CardDefinition::companion)
                    .is_some()
            });

        let format_rules = format.rules();
        let prepared_engine = crate::prepared_engine::PreparedEngine::compile(&catalog);

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
                        counters: crate::game::counters::Counters::new(),
                    });
                }
                rng.shuffle(&mut library);
                let short_hand = library.len() < format_rules.opening_hand_size;
                let count = if validate {
                    format_rules.opening_hand_size
                } else {
                    library.len().min(format_rules.opening_hand_size)
                };
                if validate && short_hand {
                    return Err(GameError::NotEnoughCardsForOpeningHand);
                }
                let initial_hand =
                    draw_opening_hand(&mut library, if choose_companions { 0 } else { count })?;
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
                    tried_to_draw_from_empty_library: short_hand,
                    hand,
                    graveyard: Vec::new(),
                    exile: Vec::new(),
                    command: Vec::new(),
                    sideboard: Vec::new(),
                    companion: None,
                    mana_pool: ManaPool::default(),
                    mana: Vec::new(),
                    lands_played_this_turn: 0,
                    counters: crate::game::counters::Counters::new(),
                })
            };

            [
                build_player(PlayerId::One, deck_one_main)?,
                build_player(PlayerId::Two, deck_two_main)?,
            ]
        };

        let commanders = super::commander::build_command_zones(
            &designated,
            &mut players,
            &mut physical_cards,
            &mut next_physical_id,
            &mut next_object_id,
        )?;

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
                players[player.index()].sideboard.push(CardInstance {
                    id: object_id,
                    definition,
                    owner: player,
                    backing: ObjectBacking::Cards(vec![physical_id]),
                    characteristics: CharacteristicSource::Card(definition),
                    counters: crate::game::counters::Counters::new(),
                });
            }
        }

        let mut game = Self {
            match_context: None,
            pending_restart: None,
            restart_arrivals: None,
            restart_count: 0,
            starting_player,
            format,
            arrived: None,
            enumerated: EnumeratedActions::default(),
            prospective_x: super::prospective_x::ProspectiveX::default(),
            payment_query: super::payment::query::PaymentQuery::default(),
            explicit_mana_payment: None,
            explicit_mana_payment_tail: std::collections::VecDeque::new(),
            payment_probe: None,
            explicit_funding: None,
            explicit_cast_contributions: None,
            successors: std::collections::HashMap::new(),
            seed,
            rng,
            catalog,
            prepared_engine,
            inline_rules: super::rules_cache::InlineRulesCache::default(),
            physical_cards,
            commanders,
            commander_move_answer: None,
            players,
            battlefield: Vec::new(),
            phased_out: Vec::new(),
            stack: GameStack::default(),
            retired_objects: BTreeMap::new(),
            nonbattlefield_ability_grants: Vec::new(),
            ongoing_effects: Vec::new(),
            next_object_id,
            next_continuous_effect_timestamp: u64::from(next_object_id),
            turn: 1,
            turns_started: if starting_player == PlayerId::One {
                [1, 0]
            } else {
                [0, 1]
            },
            damage_taken_this_turn: [0; 2],
            damage_taken_by_group_this_turn: [[0; DamageSourceGroupDef::COUNT]; 2],
            attacked_subtypes_this_turn: [Vec::new(), Vec::new()],
            active_player: starting_player,
            priority: starting_player,
            consecutive_passes: 0,
            step: Step::Upkeep,
            attackers_declared: false,
            combat_had_attackers: false,
            creature_died_this_turn: false,
            creatures_died_this_turn: 0,
            lost_life_this_turn: [false; 2],
            duration_exiles: Vec::new(),
            linked_exiles: Vec::new(),
            play_permission_uses: Vec::new(),
            monarch: None,
            ninjutsu_returned_defender: None,
            exile_play_permissions: Vec::new(),
            plotted_cards: BTreeMap::new(),
            damage_cannot_be_prevented_this_turn: false,
            cannot_gain_life: [false; 2],
            combat_damage_to_players: Vec::new(),
            turn_phase_queue: VecDeque::new(),
            turn_phase_resume: None,
            resolved_play_restrictions: Vec::new(),
            resolved_player_protections: Vec::new(),
            resolved_player_rules: Vec::new(),
            resolved_attack_restrictions: Vec::new(),
            resolved_play_permissions: Vec::new(),
            emblems: Vec::new(),
            abilities_used_this_turn: Vec::new(),
            spells_cast_this_turn: [0; 2],
            spells_cast_last_turn: [0; 2],
            spell_cast_history_this_turn: Vec::new(),
            total_spells_cast: [0; 2],
            cards_drawn_this_turn: [0; 2],
            cards_discarded_this_turn: [0; 2],
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
            pregame: Some(Pregame::Mulligan(starting_player)),
            mulligans: [0, 0],
            cleanup_pending: false,
            pending_decisions: Vec::new(),
            next_decision_id: 0,
            pending_events: VecDeque::new(),
            entry_event_batch: None,
            ready_entry_batch: None,
            deferred_token_creations: Vec::new(),
            building_entry_batch: false,
            pending_procedures: VecDeque::new(),
            pending_triggers: Vec::new(),
            next_trigger_id: 0,
            last_seen_hands: [None, None],
            pending_combat_assignments: Vec::new(),
            combat_damage_stage: CombatDamageStage::NotStarted,
            combat_blocked_attackers: Vec::new(),
            next_regular_player: starting_player.opponent(),
            extra_turns: Vec::new(),
            result: None,
            events: vec![GameEvent::GameStarted { seed }],
        };
        if choose_companions {
            game.begin_companion_selection(starting_player);
        }
        Ok(game)
    }

    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
    }

    /// The match-wide starting life total supplied by this game's format.
    /// Keeping the format as the authority avoids storing a second value that
    /// could disagree with it while still making starting life readable from
    /// every rules context that has the game state.
    #[must_use]
    pub const fn starting_life_total(&self) -> u8 {
        self.format.rules().starting_life
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
            .flat_map(|permanent| {
                std::iter::once(permanent.timestamp.0).chain(
                    permanent
                        .resolving_control_timestamp
                        .map(|timestamp| timestamp.0),
                )
            })
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

    /// Whether `target` names a card object that has ceased to exist.
    ///
    /// A binding outliving the object it named is ordinary. A group chosen at
    /// the start of a resolution keeps its ids while the effect works through
    /// them, and a card in that group may change zone on the way: Sylvan
    /// Library puts a chosen card on top of its owner's library, and
    /// `zone_change_card` below retires the object the group still lists.
    pub(super) fn card_object_retired(&self, target: Target) -> bool {
        match target {
            Target::Player(_) => false,
            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => {
                matches!(self.retired_objects.get(&id), Some(RetiredObject::Card(_)))
            }
        }
    }

    /// The group as the member at `index` should see it: the same members,
    /// less the ones the loop has already run past that no longer exist.
    /// `None` when every earlier member is still there, so the ordinary
    /// iteration keeps the group it was given.
    ///
    /// A dead id is not information the group still carries -- nothing on any
    /// board matches it, so no correct read can depend on it -- but it is
    /// information a checkpoint has to write down, and it cannot. Naming a
    /// retired card publishes what it was and, through `successors`, where it
    /// went; for a card put back on top of its owner's library that is a
    /// disclosure the observation never made. Dropping it here keeps the
    /// checkpoint honest without the wire having to describe a thing that no
    /// longer exists. Members from `index` on are left alone -- the loop still
    /// has to visit them.
    pub(super) fn live_group_before(
        &self,
        members: &[Target],
        index: usize,
    ) -> Option<Vec<Target>> {
        members[..index]
            .iter()
            .any(|target| self.card_object_retired(*target))
            .then(|| {
                members
                    .iter()
                    .enumerate()
                    .filter(|(position, target)| {
                        *position >= index || !self.card_object_retired(**target)
                    })
                    .map(|(_, target)| *target)
                    .collect()
            })
    }

    pub(super) fn remember_card_characteristics(
        &mut self,
        card: &CardInstance,
        zone: Option<ZoneKind>,
    ) {
        let stats = self.printed_card_stats(card, zone);
        let colors = if self.card_in_nonbattlefield_zone(card.id).is_some() {
            self.object_colors(card.id)
        } else {
            self.catalog
                .get(card.definition)
                .map_or([false; 5], |definition| definition.rules.colors())
        };
        self.retired_objects.entry(card.id).or_insert_with(|| {
            RetiredObject::Card(super::RetiredCard {
                card: card.clone(),
                stats,
                colors,
            })
        });
    }

    pub(super) fn zone_change_card(
        &mut self,
        mut card: CardInstance,
    ) -> (CardInstance, ZoneChangeOutcome) {
        let previous = card.id;
        self.plotted_cards.remove(&previous);
        self.nonbattlefield_ability_grants
            .retain(|grant| grant.object != previous);
        if !self.retired_objects.contains_key(&previous) {
            self.remember_card_characteristics(&card, None);
        }
        card.id = self.allocate_object_id();
        card.counters = crate::game::counters::Counters::new();
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
                colors: last_known.colors,
                power: last_known.power,
                toughness: last_known.toughness,
                mana_value: last_known.mana_value,
                keywords: last_known.keywords.clone(),
            },
        );
        permanent
    }

    pub(super) fn retire_stack_object(&mut self, object: &mut StackObject) {
        self.retired_objects
            .insert(object.id, RetiredObject::Stack(Box::new(object.clone())));
        let colors = self
            .stack_trigger_event_object(object)
            .map(|view| crate::card::ColorSet::from_flags(view.colors));
        object.last_known_colors = colors;
        if let Some(RetiredObject::Stack(retired)) = self.retired_objects.get_mut(&object.id) {
            retired.last_known_colors = colors;
        }
    }

    /// The stack object itself, as it last existed, once it has left the
    /// stack. A copy effect reads it there: the storm trigger copies the
    /// spell that raised it whether or not that spell is still waiting
    /// underneath (CR 707.10).
    pub(super) fn retired_stack_object(&self, object: GameObjectId) -> Option<StackObject> {
        match self.retired_objects.get(&object) {
            Some(RetiredObject::Stack(stack)) => Some((**stack).clone()),
            _ => None,
        }
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
            counters: crate::game::counters::Counters::new(),
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
            counters: crate::game::counters::Counters::new(),
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
        let count = self
            .format
            .rules()
            .opening_hand_size
            .min(self.players[player.index()].library.len());
        let initial_hand = draw_opening_hand(&mut self.players[player.index()].library, count)
            .expect("the requested hand fits the library");
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
        if player == self.starting_player {
            self.pregame = Some(Pregame::Mulligan(player.opponent()));
            self.priority = player.opponent();
        } else {
            self.begin_opening_hand_actions(self.starting_player);
        }
    }
}

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

mod source_values;

include!("lifecycle/last_known.rs");
