use super::{
    CardDefinitionId, CardInstance, CardPartId, CharacteristicContext, CharacteristicSource,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, EntryCompletion, Game, GameEvent,
    GameObjectId, KeywordAbility, ObjectBacking, PendingBattlefieldEntry, Permanent, PlayerId,
    PublicCard, ReplacementEventDef, Target, TriggerContext, ZoneCard, ZoneError, ZoneKind,
    ZoneMoveCause, ZoneMoveCauseDef, ZonePlacement, applicable_part_ids,
};

impl Game {
    /// Moves one object to a zone. Only the moves a supported card actually
    /// makes are handled; the rest stay seams rather than guesses.
    pub(super) fn move_target_to_zone(
        &mut self,
        target: Target,
        zone: ZoneKind,
        cause: ZoneMoveCause,
        arriving_controller: Option<PlayerId>,
        placement: ZonePlacement,
    ) {
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
            return;
        }
        let Target::Card(id) = target else {
            return;
        };
        let Some(from) = self
            .card_in_nonbattlefield_zone(id)
            .map(|(from, _card)| from)
        else {
            return;
        };
        let _ = self.move_card_from_nonbattlefield_zone(id, from, zone, cause, arriving_controller);
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
        let Some(card) = self.catalog.get(definition) else {
            return Err(ZoneError::UnknownCard(definition));
        };
        let presented = card.primary_part_id();
        let built = self.build_zone(player, &[definition])?;
        let card = built
            .into_iter()
            .next()
            .expect("build_zone returns one card for one definition");
        let id = card.id;
        let mut permanent =
            Permanent::entering(card, presented, player, self.turns_started[player.index()]);
        self.initialize_battlefield_entry(&mut permanent);
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::Setup,
        });
        Ok(id)
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
                })
            })
            .collect()
    }

    pub(super) fn discard_cards(&mut self, player: PlayerId, cards: &[GameObjectId]) {
        self.discard_cards_with_cause(player, cards, ZoneMoveCause::Rules);
        self.cleanup_pending = false;
        self.complete_cleanup();
    }

    /// Where a card headed for one zone actually goes, when a permanent on the
    /// battlefield replaces every such move. Rest in Peace is the example:
    /// nothing reaches a graveyard while it is out, from any zone at all.
    pub(super) fn external_zone_move_replacement(&self, to: ZoneKind) -> Option<ZoneKind> {
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
                if definition.event != (ReplacementEventDef::AnyObjectWouldMove { to }) {
                    return;
                }
                if let Some(EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone,
                    ..
                }) = ability.declarative_effect()
                {
                    replacement = Some(zone);
                }
            });
            if replacement.is_some() {
                break;
            }
        }
        replacement
    }

    /// The one way a card reaches a graveyard, so a replacement that sends it
    /// somewhere else has a single place to apply.
    pub(super) fn put_card_into_graveyard(&mut self, owner: PlayerId, card: CardInstance) {
        match self.external_zone_move_replacement(ZoneKind::Graveyard) {
            Some(ZoneKind::Exile) => self.players[owner.index()].exile.push(card),
            _ => self.players[owner.index()].graveyard.push(card),
        }
    }

    pub(super) fn zone_move_replacement_destination(
        &self,
        card: &CardInstance,
        from: ZoneKind,
        to: ZoneKind,
        actual_cause: ZoneMoveCause,
    ) -> Option<ZoneKind> {
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
                if event_from == from
                    && event_to == to
                    && cause_matches
                    && ability.is_executable()
                    && replacement.source_zones.contains(&from)
                    && let Some(EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone,
                        ..
                    }) = ability.declarative_effect()
                {
                    return Some(zone);
                }
            }
        }
        None
    }

    pub(super) fn put_card_onto_battlefield_from(
        &mut self,
        card: CardInstance,
        from: ZoneKind,
        controller: PlayerId,
        grant: Option<KeywordAbility>,
    ) -> CardInstance {
        self.put_card_onto_battlefield_from_with_completion(
            card,
            from,
            controller,
            grant,
            EntryCompletion::None,
        )
    }

    pub(super) fn put_card_onto_battlefield_from_with_completion(
        &mut self,
        card: CardInstance,
        from: ZoneKind,
        controller: PlayerId,
        grant: Option<KeywordAbility>,
        completion: EntryCompletion,
    ) -> CardInstance {
        let definition = self
            .catalog
            .get(card.definition)
            .expect("a card in hand remains cataloged");
        let presented = applicable_part_ids(definition, &CharacteristicContext::Hand)
            .ok()
            .and_then(|parts| parts.first().copied())
            .unwrap_or(CardPartId::PRIMARY);
        let entered_card = card.clone();
        let mut permanent = Permanent::entering(
            card,
            presented,
            controller,
            self.turns_started[controller.index()],
        );
        self.initialize_battlefield_entry(&mut permanent);
        if let Some(keyword) = grant {
            permanent.temporary_keywords.push(keyword);
        }
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from,
            completion,
        });
        entered_card
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
        // Who controls the permanent when it arrives on the battlefield.
        // Reanimation that steals names a player; everything else leaves this
        // empty and the card arrives under its owner's control.
        arriving_controller: Option<PlayerId>,
    ) -> Option<(CardInstance, ZoneKind)> {
        self.move_card_from_nonbattlefield_zone_with_completion(
            id,
            expected_from,
            requested_to,
            cause,
            arriving_controller,
            EntryCompletion::None,
        )
    }

    pub(super) fn move_card_from_nonbattlefield_zone_with_completion(
        &mut self,
        id: GameObjectId,
        expected_from: ZoneKind,
        requested_to: ZoneKind,
        cause: ZoneMoveCause,
        arriving_controller: Option<PlayerId>,
        completion: EntryCompletion,
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
        if matches!(destination, ZoneKind::Stack | ZoneKind::Command) {
            return None;
        }

        let owner = card.owner;
        let cards = match from {
            ZoneKind::Library => &mut self.players[owner.index()].library,
            ZoneKind::Hand => &mut self.players[owner.index()].hand,
            ZoneKind::Graveyard => &mut self.players[owner.index()].graveyard,
            ZoneKind::Exile => &mut self.players[owner.index()].exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return None,
        };
        let card = remove_card(cards, id)?;
        let card = if destination == ZoneKind::Battlefield {
            self.put_card_onto_battlefield_from_with_completion(
                card,
                from,
                arriving_controller.unwrap_or(owner),
                None,
                completion,
            )
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
        Some((card, destination))
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
            discarded.push((card.id, definition));
        }
        if !discarded.is_empty() {
            self.events.push(GameEvent::CardsDiscarded {
                player,
                cards: discarded,
            });
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
