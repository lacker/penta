//! Carrying out a search once its selection has been answered.
//!
//! Long enough to sit apart from the other continuations: a search can put
//! its results into any zone, and a library destination has to hold them
//! aside across the shuffle so that an explicit placement still means
//! something.

use super::{
    BattlefieldArrival, CardDefinitionId, Game, GameEvent, GameObjectId, PendingProcedure,
    PlayerId, SearchFollowUp, Target, ZoneKind, ZoneMoveCause, ZonePlacement, remove_card,
};

/// Everything the answered search still needs, gathered from its
/// continuation so the resolver takes one argument rather than ten.
pub(super) struct SearchResolution {
    pub(super) controller: PlayerId,
    pub(super) source: ZoneKind,
    pub(super) destination: ZoneKind,
    pub(super) placement: ZonePlacement,
    pub(super) reveal: bool,
    pub(super) shuffle: bool,
    pub(super) enters_tapped: bool,
    pub(super) attached_player: Option<PlayerId>,
    pub(super) binding: Option<crate::ids::ObjectSetBindingIndex>,
    pub(super) follow_up: Option<Box<SearchFollowUp>>,
}

impl Game {
    pub(super) fn resolve_completed_search(
        &mut self,
        player: PlayerId,
        selected: &[(GameObjectId, CardDefinitionId)],
        resolution: SearchResolution,
    ) {
        let SearchResolution {
            controller,
            source,
            destination,
            placement,
            reveal,
            shuffle,
            enters_tapped,
            attached_player,
            binding,
            follow_up,
        } = resolution;
        let selected = selected.to_vec();
        // Bound where the cards were found. A move out of a hidden zone
        // gives each card a new identity, so a destination that changes
        // them rebinds below; a library destination and a battlefield
        // arrival do not, the first because the card never leaves and the
        // second because what enters is settled after this.
        let mut follow_up = follow_up.map(|follow_up| {
            let mut follow_up = *follow_up;
            if let Some(binding) = binding {
                let found = selected
                    .iter()
                    .map(|(card, _)| Target::Card(*card))
                    .collect::<Vec<_>>();
                follow_up.context.bind_object_group(binding, found);
            }
            follow_up
        });
        if reveal {
            self.events.extend(
                selected
                    .iter()
                    .map(|(card, definition)| GameEvent::CardRevealed {
                        player,
                        card: *card,
                        definition: *definition,
                    }),
            );
        }

        if destination == ZoneKind::Library {
            self.return_search_results_to_library(player, &selected, source, placement, shuffle);
            if let Some(follow_up) = follow_up {
                self.resolve_nested_effect_before_later(
                    follow_up.effect,
                    &follow_up.object,
                    follow_up.context,
                );
            }
            return;
        }

        if source != destination {
            let mut moved = Vec::new();
            for (card, _) in selected {
                let landed = self.move_card_from_nonbattlefield_zone(
                    card,
                    source,
                    destination,
                    ZoneMoveCause::Effect { controller },
                    (destination == ZoneKind::Battlefield).then(|| {
                        let arrival = if enters_tapped {
                            BattlefieldArrival::tapped_under(player)
                        } else {
                            BattlefieldArrival::under(player)
                        };
                        attached_player
                            .map_or(arrival, |attached| arrival.attached_to_player(attached))
                    }),
                );
                if let Some((landed, _)) = landed {
                    moved.push(Target::Card(landed.id));
                }
            }
            // "Exile them, then ... you may cast those cards": the cards the
            // follow-up names are the ones now sitting in the destination,
            // which are new objects.
            if destination != ZoneKind::Battlefield
                && let Some(binding) = binding
                && let Some(follow_up) = follow_up.as_mut()
            {
                follow_up.context.bind_object_group(binding, moved);
            }
        }
        if shuffle {
            // Putting a searched-for permanent onto the battlefield can
            // suspend for an as-enters choice. Finish that prospective
            // entry before carrying out the search's subsequent
            // shuffle, but still precede any enclosing effect tail.
            self.pending_procedures
                .push_front(PendingProcedure::ShuffleLibrary { player });
        }
        if let Some(follow_up) = follow_up {
            self.resolve_nested_effect_before_later(
                follow_up.effect,
                &follow_up.object,
                follow_up.context,
            );
        }
    }
}

impl Game {
    /// Where a search that puts its results back into a library finishes.
    ///
    /// A card excluded from its own library's shuffle never changed zones,
    /// so it keeps its object identity; cards arriving from elsewhere make
    /// the ordinary zone change first and are then held aside, because an
    /// explicit placement only means something after the shuffle.
    fn return_search_results_to_library(
        &mut self,
        player: PlayerId,
        selected: &[(GameObjectId, CardDefinitionId)],
        source: ZoneKind,
        placement: ZonePlacement,
        shuffle: bool,
    ) {
        let mut held = Vec::new();
        if source == ZoneKind::Library {
            held.extend(selected.iter().filter_map(|(card, _)| {
                remove_card(&mut self.players[player.index()].library, *card)
                    .map(|card| (card.owner, card))
            }));
        } else {
            for (card, _) in selected {
                let Some((moved, actual_destination)) = self.move_card_from_nonbattlefield_zone(
                    *card,
                    source,
                    ZoneKind::Library,
                    ZoneMoveCause::Effect { controller: player },
                    None,
                ) else {
                    continue;
                };
                if actual_destination == ZoneKind::Library
                    && let Some(card) =
                        remove_card(&mut self.players[moved.owner.index()].library, moved.id)
                {
                    held.push((moved.owner, card));
                }
            }
        }
        if shuffle {
            self.rng.shuffle(&mut self.players[player.index()].library);
        }
        for (owner, card) in held {
            match placement {
                ZonePlacement::Top => {
                    self.players[owner.index()].library.push(card);
                }
                ZonePlacement::Bottom => {
                    self.players[owner.index()].library.insert(0, card);
                }
            }
        }
    }
}
