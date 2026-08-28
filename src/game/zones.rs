use super::{
    ArrivalAttachment, BattlefieldArrival, CardDefinitionId, CardInstance, CardPartId,
    CardStructure, CharacteristicContext, CharacteristicSource, CommittedTriggerEvent, CounterKind,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, EntryCompletion, Game, GameEvent,
    GameObjectId, KeywordAbility, ObjectBacking, PendingBattlefieldEntry, Permanent, PlayerId,
    PublicCard, ReplacementEffectDef, ReplacementEventDef, Target, TriggerContext, ZoneCard,
    ZoneError, ZoneKind, ZoneMoveCause, ZoneMoveCauseDef, ZonePlacement, applicable_part_ids,
};

/// Where a card headed for one zone actually goes when something on the
/// battlefield replaces the move, and what that replacement puts on it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ExternalZoneMoveReplacement {
    pub(super) zone: ZoneKind,
    pub(super) counters: Option<(CounterKind, u16)>,
}

impl Game {
    /// Every card of this name in one player's zone, as targets. Cabal
    /// Therapy names one and takes them all.
    pub(super) fn cards_named_in_zone(
        &self,
        player: PlayerId,
        zone: ZoneKind,
        name: &str,
    ) -> Vec<Target> {
        let cards = match zone {
            ZoneKind::Hand => &self.players[player.index()].hand,
            ZoneKind::Graveyard => &self.players[player.index()].graveyard,
            ZoneKind::Exile => &self.players[player.index()].exile,
            ZoneKind::Library => &self.players[player.index()].library,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return Vec::new(),
        };
        cards
            .iter()
            .filter(|card| {
                self.catalog
                    .get(card.definition)
                    .is_some_and(|definition| definition.name == name)
            })
            .map(|card| Target::Card(card.id))
            .collect()
    }

    /// Moves one object to a zone. Only the moves a supported card actually
    /// makes are handled; the rest stay seams rather than guesses.
    /// Returns the battlefield object the card became, when it arrived on
    /// the battlefield. A permanent that enters is a new object with a new
    /// identity, so this is the only handle a later effect has on it.
    pub(super) fn move_target_to_zone(
        &mut self,
        target: Target,
        zone: ZoneKind,
        cause: ZoneMoveCause,
        arriving_controller: Option<BattlefieldArrival>,
        placement: ZonePlacement,
    ) -> Option<GameObjectId> {
        if let Target::Spell(id) = target {
            if zone == ZoneKind::Hand {
                self.return_spell_to_hand(id);
            }
            return None;
        }
        if let Target::Permanent(id) = target {
            // Leaving the battlefield has its own procedure: last-known
            // information, exit events, and the triggers watching for them.
            match zone {
                ZoneKind::Exile => self.exile_permanent(id),
                ZoneKind::Hand => self.return_permanent_to_hand(id),
                ZoneKind::Graveyard => self.move_permanents_to_graveyard(&[id]),
                ZoneKind::Library => self.return_permanent_to_library(id, placement),
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {}
            }
            return None;
        }
        let Target::Card(id) = target else {
            return None;
        };
        let (moved, actual_destination) =
            self.move_card_target_to_zone(id, zone, cause, arriving_controller, placement)?;
        (actual_destination == ZoneKind::Battlefield)
            .then_some(moved)
            .and(self.arrived.take())
    }

    /// Move a card outside the battlefield while retaining the identity it
    /// becomes. Group workflows need that successor to bind the result for a
    /// later stage; the older single-object entrypoint intentionally returns
    /// only battlefield arrivals.
    pub(super) fn move_card_target_to_zone(
        &mut self,
        id: GameObjectId,
        zone: ZoneKind,
        cause: ZoneMoveCause,
        arriving_controller: Option<BattlefieldArrival>,
        placement: ZonePlacement,
    ) -> Option<(GameObjectId, ZoneKind)> {
        let from = self
            .card_in_nonbattlefield_zone(id)
            .map(|(from, _card)| from)?;
        if from == ZoneKind::Library && zone == ZoneKind::Library {
            let owner = self
                .card_in_nonbattlefield_zone(id)
                .map(|(_, card)| card.owner)?;
            let card = remove_card(&mut self.players[owner.index()].library, id)?;
            let library = &mut self.players[owner.index()].library;
            let index = placement.library_index(library.len());
            library.insert(index, card);
            return Some((id, ZoneKind::Library));
        }
        let (moved, actual_destination) =
            self.move_card_from_nonbattlefield_zone(id, from, zone, cause, arriving_controller)?;
        // The move above put it on top, which is where a card goes when
        // nothing says otherwise. Anywhere else is a lift and a reinsert.
        if actual_destination == ZoneKind::Library
            && placement != ZonePlacement::Top
            && let Some(card) =
                remove_card(&mut self.players[moved.owner.index()].library, moved.id)
        {
            let library = &mut self.players[moved.owner.index()].library;
            let index = placement.library_index(library.len());
            library.insert(index, card);
        }
        Some((moved.id, actual_destination))
    }

    /// One card in a hand or library, as a simulation sees it.
    ///
    /// This is not redacted. [`Self::observe`] is the redacted view, and it is
    /// what anything talking to a client should use; a `Game` in your own
    /// process has no one to hide from.
    #[must_use]
    pub fn hand(&self, player: PlayerId) -> Vec<ZoneCard> {
        zone_cards(&self.players[player.index()].hand)
    }

    /// The player's library from the top down, so index zero is the next draw.
    #[must_use]
    pub fn library(&self, player: PlayerId) -> Vec<ZoneCard> {
        zone_cards(&self.players[player.index()].library)
    }

    /// Replaces a hand with exactly these cards, named by definition.
    ///
    /// The cards are built fresh, so this states what a hand *is* rather than
    /// moving objects around: to explore "their last card is either Lightning
    /// Bolt or Counterspell", set the same hand twice with a different last
    /// entry and play both out. Nothing is conserved, because a simulation
    /// exploring a world it cannot see has no reason to be.
    ///
    /// The new cards get new object IDs. Rewrite an opponent's zones rather
    /// than your own if you are holding IDs from an earlier observation.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] if a definition is not in the
    /// catalog this game was built with.
    pub fn set_hand(
        &mut self,
        player: PlayerId,
        cards: &[CardDefinitionId],
    ) -> Result<(), ZoneError> {
        self.forget_enumeration();
        let built = self.build_zone(player, cards)?;
        self.players[player.index()].hand = built;
        Ok(())
    }

    /// Puts a permanent onto the battlefield under `player`, named by
    /// definition, and returns its object ID.
    ///
    /// This completes the simulation surface that [`Self::set_hand`] and
    /// [`Self::set_library`] start: those state what a hidden zone holds, and
    /// this states what is in play. It is how a caller reaches a board state
    /// directly instead of playing toward one.
    ///
    /// The permanent enters as though it resolved, raising the same
    /// zone-change event and applying the same replacement effects, so anything
    /// that replaces or triggers on entering sees it. If a replacement needs a
    /// choice, the returned ID is reserved for the prospective permanent until
    /// that choice commits it. The setup does not pay a spell cost, take a
    /// turn, or respect timing: setting up a board is not the same as playing
    /// to one, and the difference is the point.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] when the definition is not in this
    /// game's catalog, and [`ZoneError::TooManyCards`] when the game has run
    /// out of object identifiers.
    ///
    /// # Panics
    ///
    /// Panics if the catalog yields no card for a definition it just
    /// validated, which would mean the catalog changed mid-call.
    pub fn put_onto_battlefield(
        &mut self,
        player: PlayerId,
        definition: CardDefinitionId,
    ) -> Result<GameObjectId, ZoneError> {
        self.forget_enumeration();
        let Some(card) = self.catalog.get(definition) else {
            return Err(ZoneError::UnknownCard(definition));
        };
        let presented = card.battlefield_entry_part();
        let built = self.build_zone(player, &[definition])?;
        let card = built
            .into_iter()
            .next()
            .expect("build_zone returns one card for one definition");
        let id = card.id;
        let mut permanent = Permanent::entering(
            card,
            presented,
            player,
            self.turns_started[player.index()],
            self.turn,
        );
        self.initialize_battlefield_entry(&mut permanent);
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::Setup,
            redirected_to: None,
        });
        Ok(id)
    }

    /// Moves a card already on top of its owner's library down to sit just
    /// beneath the top `depth` cards. A library is stored bottom-first, so
    /// the top is the end and the card lands `depth` places back from it; a
    /// depth past the library's length puts it on the bottom.
    pub(super) fn sink_library_card(&mut self, card: GameObjectId, depth: usize) {
        let Some(owner) = self
            .card_in_nonbattlefield_zone(card)
            .map(|(_, card)| card.owner)
        else {
            return;
        };
        let library = &mut self.players[owner.index()].library;
        let Some(position) = library.iter().position(|held| held.id == card) else {
            return;
        };
        let held = library.remove(position);
        let target = library.len().saturating_sub(depth);
        library.insert(target, held);
    }

    /// Puts a card into a player's graveyard directly, as a simulation and
    /// test entry point. Nothing died and nothing resolved, so no trigger sees
    /// this.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] when the definition is not cataloged.
    ///
    /// # Panics
    ///
    /// Panics if building one card from one definition yields no card.
    pub fn put_into_graveyard(
        &mut self,
        player: PlayerId,
        definition: CardDefinitionId,
    ) -> Result<GameObjectId, ZoneError> {
        self.forget_enumeration();
        let built = self.build_zone(player, &[definition])?;
        let card = built
            .into_iter()
            .next()
            .expect("build_zone returns one card for one definition");
        let id = card.id;
        self.players[player.index()].graveyard.push(card);
        Ok(id)
    }

    /// Replaces a library with exactly these cards, top card first. Behaves
    /// like [`Self::set_hand`] in every other respect.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] under the same conditions as
    /// [`Self::set_hand`].
    pub fn set_library(
        &mut self,
        player: PlayerId,
        cards: &[CardDefinitionId],
    ) -> Result<(), ZoneError> {
        self.forget_enumeration();
        let built = self.build_zone(player, cards)?;
        self.players[player.index()].library = built;
        Ok(())
    }

    /// Mints fresh instances owned by `player`, one per definition.
    pub(super) fn build_zone(
        &mut self,
        player: PlayerId,
        cards: &[CardDefinitionId],
    ) -> Result<Vec<CardInstance>, ZoneError> {
        if let Some(unknown) = cards
            .iter()
            .find(|definition| self.catalog.get(**definition).is_none())
        {
            return Err(ZoneError::UnknownCard(*unknown));
        }

        cards
            .iter()
            .map(|definition| {
                let id = GameObjectId(self.next_object_id);
                self.next_object_id = self
                    .next_object_id
                    .checked_add(1)
                    .ok_or(ZoneError::TooManyCards)?;
                Ok(CardInstance {
                    id,
                    definition: *definition,
                    owner: player,
                    // A card conjured for a hypothetical has no physical
                    // provenance, which only meld and copy effects consult.
                    backing: ObjectBacking::None,
                    characteristics: CharacteristicSource::Card(*definition),
                    counters: crate::game::counters::Counters::new(),
                })
            })
            .collect()
    }

    /// Discards that many cards picked at random from a player's hand.
    ///
    /// The picks come off the seeded generator one at a time, so a replay of
    /// the same seed discards the same cards. Fewer cards in hand than asked
    /// for discards the whole hand, which is what paying a cost you can only
    /// partly afford would mean -- but activation legality already refuses
    /// that case, so in practice the hand is always big enough.
    pub(super) fn discard_at_random(&mut self, player: PlayerId, amount: usize) {
        let mut chosen = Vec::with_capacity(amount);
        let mut remaining: Vec<_> = self.players[player.index()]
            .hand
            .iter()
            .map(|card| card.id)
            .collect();
        for _ in 0..amount.min(remaining.len()) {
            let index = self.rng.index_below(remaining.len());
            chosen.push(remaining.swap_remove(index));
        }
        self.discard_cards(player, &chosen);
    }

    pub(super) fn discard_cards(&mut self, player: PlayerId, cards: &[GameObjectId]) {
        self.discard_cards_with_cause(player, cards, ZoneMoveCause::Rules);
        self.cleanup_pending = false;
        self.complete_cleanup();
    }

    /// Where a card headed for one zone actually goes, when a permanent on the
    /// battlefield replaces every such move. Rest in Peace is the example:
    /// nothing reaches a graveyard while it is out, from any zone at all.
    pub(super) fn external_zone_move_replacement(
        &self,
        to: ZoneKind,
        owner: PlayerId,
        is_token: bool,
    ) -> Option<ExternalZoneMoveReplacement> {
        let mut replacement = None;
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                if !ability.is_executable() {
                    return;
                }
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return;
                };
                let ReplacementEventDef::AnyObjectWouldMove {
                    to: watched,
                    owner: watched_owner,
                    tokens,
                } = definition.event
                else {
                    return;
                };
                if watched != to
                    || (!tokens && is_token)
                    || !self.player_relation_matches(
                        owner,
                        watched_owner,
                        permanent.controller,
                        TriggerContext::empty(),
                    )
                {
                    return;
                }
                if let Some(program) = ability.declarative_replacement()
                    && let Some(zone) = Self::replacement_move_destination(program)
                {
                    replacement = Some(ExternalZoneMoveReplacement {
                        zone,
                        counters: Self::replacement_move_counters(program),
                    });
                }
            });
            if replacement.is_some() {
                break;
            }
        }
        // And the same replacement made by a resolving spell rather than
        // printed on a permanent: the effect object holds it for as long as
        // it lasts, which is where Yawgmoth's Will keeps its turn.
        if replacement.is_none() {
            for ongoing in &self.ongoing_effects {
                let ability = ongoing.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    continue;
                };
                let ReplacementEventDef::AnyObjectWouldMove {
                    to: watched,
                    owner: watched_owner,
                    tokens,
                } = definition.event
                else {
                    continue;
                };
                if watched != to
                    || (!tokens && is_token)
                    || !self.player_relation_matches(
                        owner,
                        watched_owner,
                        ongoing.controller,
                        TriggerContext::empty(),
                    )
                {
                    continue;
                }
                if let Some(program) = ability.declarative_replacement()
                    && let Some(zone) = Self::replacement_move_destination(program)
                {
                    replacement = Some(ExternalZoneMoveReplacement {
                        zone,
                        counters: Self::replacement_move_counters(program),
                    });
                    break;
                }
            }
        }
        replacement
    }

    /// The one way a card reaches a graveyard, so a replacement that sends it
    /// somewhere else has a single place to apply.
    pub(super) fn put_card_into_graveyard(&mut self, owner: PlayerId, mut card: CardInstance) {
        // Tokens cease to exist as they leave the battlefield and never become
        // `CardInstance`s. A physical card remains a nontoken here even when
        // its former permanent was copying token characteristics.
        match self.external_zone_move_replacement(ZoneKind::Graveyard, owner, false) {
            Some(ExternalZoneMoveReplacement {
                zone: ZoneKind::Exile,
                counters,
            }) => {
                if let Some((kind, amount)) = counters {
                    card.add_counters(kind, amount);
                }
                self.players[owner.index()].exile.push(card);
            }
            _ => self.players[owner.index()].graveyard.push(card),
        }
    }

    /// What a replacement program puts on the card it moves, if anything.
    /// Read the same way the destination beside it is: a sequence carries
    /// the counter alongside the move rather than as an effect of its own.
    fn replacement_move_counters(effect: ReplacementEffectDef) -> Option<(CounterKind, u16)> {
        match effect {
            ReplacementEffectDef::PlaceCountersOnMovedObject { kind, amount } => {
                Some((kind, amount))
            }
            ReplacementEffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .find_map(Self::replacement_move_counters),
            _ => None,
        }
    }

    /// Whether a replacement program shuffles the library it just put a card
    /// into. "Reveal it and shuffle it into its owner's library" would
    /// otherwise leave the card on top, which is a very different card.
    fn replacement_shuffles_library(effect: ReplacementEffectDef) -> bool {
        match effect {
            ReplacementEffectDef::Perform(effect) => matches!(
                *effect,
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                }
            ),
            ReplacementEffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(Self::replacement_shuffles_library),
            _ => false,
        }
    }

    /// Whether the replacement that redirects this move also shuffles.
    fn zone_move_replacement_shuffles(
        &self,
        card: &CardInstance,
        from: ZoneKind,
        to: ZoneKind,
        actual_cause: ZoneMoveCause,
    ) -> bool {
        self.zone_move_replacement_program(card, from, to, actual_cause)
            .is_some_and(Self::replacement_shuffles_library)
    }

    /// Where a replacement program sends the card, if it sends it anywhere.
    /// A sequence carries the move alongside whatever else it does, and the
    /// move is the part a zone change needs to know about.
    pub(super) fn replacement_move_destination(effect: ReplacementEffectDef) -> Option<ZoneKind> {
        match effect {
            ReplacementEffectDef::MoveToZone(zone) => Some(zone),
            ReplacementEffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .find_map(Self::replacement_move_destination),
            _ => None,
        }
    }

    pub(super) fn zone_move_replacement_destination(
        &self,
        card: &CardInstance,
        from: ZoneKind,
        to: ZoneKind,
        actual_cause: ZoneMoveCause,
    ) -> Option<ZoneKind> {
        self.zone_move_replacement_program(card, from, to, actual_cause)
            .and_then(Self::replacement_move_destination)
    }

    /// The replacement program that answers this move, if one does.
    fn zone_move_replacement_program(
        &self,
        card: &CardInstance,
        from: ZoneKind,
        to: ZoneKind,
        actual_cause: ZoneMoveCause,
    ) -> Option<ReplacementEffectDef> {
        let characteristic_context = match from {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Command => CharacteristicContext::Command,
            ZoneKind::Battlefield | ZoneKind::Stack => return None,
        };
        let replacement_controller = card.owner;
        let definition = self.catalog.get(card.definition)?;
        let parts = applicable_part_ids(definition, &characteristic_context).ok()?;
        for part in parts {
            let Some(part) = definition.part(part) else {
                continue;
            };
            for ability in part.rules.ability_clauses() {
                let DeclarativeAbilityDef::Replacement(replacement) = ability.definition else {
                    continue;
                };
                let ReplacementEventDef::WouldMove {
                    from: event_from,
                    to: event_to,
                    cause,
                } = replacement.event
                else {
                    continue;
                };
                let cause_matches = match cause {
                    ZoneMoveCauseDef::Any => true,
                    ZoneMoveCauseDef::EffectControlledBy(relation) => {
                        let ZoneMoveCause::Effect { controller } = actual_cause else {
                            continue;
                        };
                        self.player_relation_matches(
                            controller,
                            relation,
                            replacement_controller,
                            TriggerContext::empty(),
                        )
                    }
                };
                if event_from.is_none_or(|expected| expected == from)
                    && event_to == to
                    && cause_matches
                    && ability.is_executable()
                    && replacement.source_zones.contains(&from)
                    && let Some(effect) = ability.declarative_replacement()
                    && Self::replacement_move_destination(effect).is_some()
                {
                    return Some(effect);
                }
            }
        }
        None
    }

    pub(super) fn put_card_onto_battlefield_from(
        &mut self,
        card: CardInstance,
        from: ZoneKind,
        arrival: BattlefieldArrival,
        grant: Option<KeywordAbility>,
    ) -> Option<CardInstance> {
        let controller = arrival.controller;
        let definition = self
            .catalog
            .get(card.definition)
            .expect("a card in hand remains cataloged");
        if definition.rules.has_metadata_only_creature_body() && arrival.face_down.is_none() {
            // Catalog-only bodies may still exist in hidden-zone fixtures and
            // may be manifested as the ordinary face-down 2/2, but no game
            // effect can turn their printed metadata into a face-up vanilla
            // permanent. Restore the card because the attempted move did not
            // happen.
            let owner = card.owner;
            match from {
                ZoneKind::Library => self.players[owner.index()].library.push(card),
                ZoneKind::Hand => self.players[owner.index()].hand.push(card),
                ZoneKind::Graveyard => self.players[owner.index()].graveyard.push(card),
                ZoneKind::Exile => self.players[owner.index()].exile.push(card),
                ZoneKind::Stack => self.put_card_into_graveyard(owner, card),
                ZoneKind::Battlefield | ZoneKind::Command => {
                    debug_assert!(false, "unsupported source for a battlefield arrival");
                }
            }
            return None;
        }
        let front = applicable_part_ids(definition, &CharacteristicContext::Hand)
            .ok()
            .and_then(|parts| parts.first().copied())
            .unwrap_or(CardPartId::PRIMARY);
        // A Room that arrives from anywhere but the stack has had no door
        // chosen for it, so it arrives with both of them locked.
        let front = if matches!(definition.structure, CardStructure::Room { .. }) {
            definition.battlefield_entry_part()
        } else {
            front
        };
        // A card told to arrive transformed enters showing its other face.
        // A single-faced card has no other face and simply ignores it.
        let presented = if arrival.transformed {
            definition.other_face(front).unwrap_or(front)
        } else {
            front
        };
        let entered_card = card.clone();
        let mut permanent = Permanent::entering(
            card,
            presented,
            controller,
            self.turns_started[controller.index()],
            self.turn,
        );
        if let Some(ArrivalAttachment::ArrivalToPlayer(player)) = arrival.attachment {
            permanent.attached_player = Some(player);
        }
        // Set before entry replacements run, the same way an as-enters clause
        // would, so nothing observes the permanent arriving untapped first.
        permanent.tapped = arrival.tapped;
        for modification in arrival.modifications {
            Self::modify_battlefield_entry_permanent(&mut permanent, *modification);
        }
        // A card put onto the battlefield face down was never face up there,
        // so these are part of the arrival rather than a later turn-over.
        permanent.face_down = arrival.face_down;
        permanent.turn_up_for_mana_cost = arrival.turn_up_for_mana_cost;
        // On it as it arrives, so an enters trigger reading its power sees
        // them and a keyword counter is granting its keyword already.
        if let Some((kind, amount)) = arrival.counters {
            permanent.add_counters(kind, amount);
        }
        self.initialize_battlefield_entry(&mut permanent);
        if let Some(keyword) = grant {
            permanent.temporary_keywords.push(keyword);
        }
        let expected = permanent.card.definition;
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from,
            completion: match arrival.attachment {
                Some(ArrivalAttachment::SourceToArrival(source)) => {
                    EntryCompletion::AttachSource { source }
                }
                Some(ArrivalAttachment::ArrivalToHost(host)) => {
                    EntryCompletion::AttachToHost { host }
                }
                Some(ArrivalAttachment::ArrivalToPlayer(_)) | None => EntryCompletion::None,
            },
            redirected_to: None,
        });
        // The entry is committed here unless a replacement needs an answer
        // first, and committing mints a fresh identity -- the card that left
        // the graveyard and the permanent now standing there are two objects.
        self.arrived = self
            .battlefield
            .last()
            .filter(|permanent| permanent.card.definition == expected)
            .map(|permanent| permanent.card.id);
        Some(entered_card)
    }

    /// Moves a card between non-stack zones after applying replacement
    /// abilities printed on that card. The replacement is selected before the
    /// old object leaves its source zone, so its source-zone characteristics
    /// remain available while matching the proposed move.
    pub(super) fn move_card_from_nonbattlefield_zone(
        &mut self,
        id: GameObjectId,
        expected_from: ZoneKind,
        requested_to: ZoneKind,
        cause: ZoneMoveCause,
        // How the permanent arrives, when the destination is the battlefield.
        // Reanimation that steals names a controller; a fetch land names
        // tapped. Everything else leaves this empty.
        arrival: Option<BattlefieldArrival>,
    ) -> Option<(CardInstance, ZoneKind)> {
        let (from, card) = self
            .card_in_nonbattlefield_zone(id)
            .map(|(zone, card)| (zone, card.clone()))?;
        if from != expected_from {
            return None;
        }
        let destination = self
            .zone_move_replacement_destination(&card, from, requested_to, cause)
            .unwrap_or(requested_to);
        let shuffles = self.zone_move_replacement_shuffles(&card, from, requested_to, cause);
        if matches!(destination, ZoneKind::Stack | ZoneKind::Command) {
            return None;
        }

        let owner = card.owner;
        let before_move = card.clone();
        let cards = match from {
            ZoneKind::Library => &mut self.players[owner.index()].library,
            ZoneKind::Hand => &mut self.players[owner.index()].hand,
            ZoneKind::Graveyard => &mut self.players[owner.index()].graveyard,
            ZoneKind::Exile => &mut self.players[owner.index()].exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return None,
        };
        let card = remove_card(cards, id)?;
        let card = if destination == ZoneKind::Battlefield {
            self.put_card_onto_battlefield_from(
                card,
                from,
                arrival.unwrap_or_else(|| BattlefieldArrival::under(owner)),
                None,
            )?
        } else {
            let (card, _zone_change) = self.zone_change_card(card);
            match destination {
                ZoneKind::Library => self.players[owner.index()].library.push(card.clone()),
                ZoneKind::Hand => self.players[owner.index()].hand.push(card.clone()),
                ZoneKind::Graveyard => self.put_card_into_graveyard(owner, card.clone()),
                ZoneKind::Exile => self.players[owner.index()].exile.push(card.clone()),
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported destinations returned before removing the card")
                }
            }
            card
        };
        if shuffles && destination == ZoneKind::Library {
            self.rng.shuffle(&mut self.players[owner.index()].library);
        }
        if destination == ZoneKind::Graveyard {
            self.capture_nonbattlefield_graveyard_arrival(&before_move, &card, from);
        }
        if destination == ZoneKind::Exile {
            self.capture_cards_exiled(std::slice::from_ref(&card), from);
        }
        if from == ZoneKind::Graveyard {
            self.note_card_left_graveyard(owner);
        }
        Some((card, destination))
    }

    /// "When this is put into a graveyard from anywhere" for the halves that
    /// are not a permanent dying: discarded from a hand, milled from a
    /// library, exiled and then returned.
    ///
    /// Raised after the card has landed, which is what lets the graveyard
    /// walk find the listener at all -- it reads the cards lying there. A
    /// battlefield departure uses the batched exit path instead, which keeps
    /// its pre-move LKI and installs the destination object before publishing.
    fn capture_nonbattlefield_graveyard_arrival(
        &mut self,
        before: &CardInstance,
        after: &CardInstance,
        from: ZoneKind,
    ) {
        let source_context = match from {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return,
        };
        let before = self.printed_trigger_event_object(
            before.id,
            before.definition,
            before.owner,
            &source_context,
        );
        let Some(after) = self.printed_trigger_event_object(
            after.id,
            after.definition,
            after.owner,
            &CharacteristicContext::Graveyard,
        ) else {
            return;
        };
        self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
            before,
            after: Some(after),
            from,
            to: ZoneKind::Graveyard,
            damage_sources: Vec::new(),
        });
    }

    /// "If a card left your graveyard this turn." Recorded rather than
    /// reconstructed: by the time an end step asks, the card it is about is
    /// somewhere else entirely and nothing left behind says where it came
    /// from.
    pub(super) fn note_card_left_graveyard(&mut self, owner: PlayerId) {
        self.card_left_graveyard_this_turn[owner.index()] = true;
    }

    /// "Whenever one or more cards are put into exile from ...": one event
    /// for the whole move, published once however many cards it took.
    ///
    /// Raised only where the cards came out of a hidden or public zone that
    /// a clause can name -- a permanent exiled from the battlefield is a
    /// zone change of its own and is published there.
    pub(super) fn capture_cards_exiled(&mut self, cards: &[CardInstance], from: ZoneKind) {
        let Some(owner) = cards.first().map(|card| card.owner) else {
            return;
        };
        let objects = cards
            .iter()
            .filter_map(|card| {
                self.printed_trigger_event_object(
                    card.id,
                    card.definition,
                    card.owner,
                    &CharacteristicContext::Exile,
                )
            })
            .collect::<Vec<_>>();
        if objects.is_empty() {
            return;
        }
        self.capture_battlefield_triggers(&CommittedTriggerEvent::CardsExiled {
            cards: objects,
            from,
            owner,
        });
    }

    /// Raises the exile event for cards already sitting in exile, for the
    /// tests that need one without an effect to make it.
    #[cfg(test)]
    pub(super) fn capture_exile_for_test(&mut self, cards: &[GameObjectId], from: ZoneKind) {
        let moved = cards
            .iter()
            .filter_map(|id| {
                self.card_in_nonbattlefield_zone(*id)
                    .map(|(_, card)| card.clone())
            })
            .collect::<Vec<_>>();
        self.capture_cards_exiled(&moved, from);
    }

    pub(super) fn discard_cards_with_cause(
        &mut self,
        player: PlayerId,
        cards: &[GameObjectId],
        cause: ZoneMoveCause,
    ) {
        let mut discarded = Vec::new();
        for id in cards {
            if !self.players[player.index()]
                .hand
                .iter()
                .any(|card| card.id == *id)
            {
                continue;
            }
            let Some((card, _destination)) = self.move_card_from_nonbattlefield_zone(
                *id,
                ZoneKind::Hand,
                ZoneKind::Graveyard,
                cause,
                None,
            ) else {
                continue;
            };
            let definition = card.definition;
            // Read where the card now lies: a trigger that exiles "that
            // card from your graveyard" needs the graveyard object, and the
            // one that was in hand no longer exists.
            let object = self.printed_trigger_event_object(
                card.id,
                definition,
                player,
                &CharacteristicContext::Graveyard,
            );
            discarded.push((card.id, definition, object));
        }
        if !discarded.is_empty() {
            self.events.push(GameEvent::CardsDiscarded {
                player,
                cards: discarded
                    .iter()
                    .map(|(id, definition, _)| (*id, *definition))
                    .collect(),
            });
            // One event per card: "whenever you discard a card" fires twice
            // for a discard of two. Raised after the cards have moved, so
            // anything the triggers read sees the finished hand.
            for (_, _, object) in discarded {
                self.capture_battlefield_triggers(&CommittedTriggerEvent::Discarded {
                    player,
                    card: object,
                });
            }
            // And once for the whole discard, which is what "one or more
            // cards" asks about.
            self.capture_battlefield_triggers(&CommittedTriggerEvent::CardsDiscarded { player });
        }
    }
}

/// Projects internal card instances into the public, unredacted zone view.
pub(super) fn zone_cards(cards: &[CardInstance]) -> Vec<ZoneCard> {
    cards
        .iter()
        .map(|card| ZoneCard {
            object: card.id,
            definition: card.definition,
        })
        .collect()
}

pub(super) fn remove_card(cards: &mut Vec<CardInstance>, id: GameObjectId) -> Option<CardInstance> {
    cards
        .iter()
        .position(|card| card.id == id)
        .map(|index| cards.remove(index))
}

pub(super) fn public_cards(cards: &[CardInstance]) -> Vec<PublicCard> {
    cards
        .iter()
        .map(|card| (card.id, card.definition))
        .collect()
}
