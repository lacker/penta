//! The pile an entering permanent takes with it.
//!
//! "As this creature enters, exile any number of creature cards from your
//! graveyard" is a replacement rather than a trigger, because the creature
//! it makes is a 0/0 until the pile is chosen and a trigger would let
//! state-based actions kill it first. So the entry waits behind the choice
//! and resumes once the pile is exiled and linked to it.

use super::super::{
    CardDefinitionId, CardPartId, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, Game, GameObjectId, ObjectCharacteristics,
    ObjectPredicateDef, PlayerId, ZoneKind, remove_card,
};

impl Game {
    /// The cards such a clause may take.
    pub(in crate::game) fn matching_graveyard_cards(
        &self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) -> Vec<(GameObjectId, CardDefinitionId)> {
        self.players[player.index()]
            .graveyard
            .iter()
            .filter(|card| self.card_object_matches(predicate, card, ZoneKind::Graveyard, source))
            .map(|card| (card.id, card.definition))
            .collect()
    }

    pub(in crate::game) fn queue_entry_exile_choice(
        &mut self,
        player: PlayerId,
        name: &str,
        entering: GameObjectId,
        candidates: &[(GameObjectId, CardDefinitionId)],
    ) {
        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, (card, definition))| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self
                    .catalog
                    .get(*definition)
                    .map_or_else(|| "Unknown card".into(), |card| card.name.clone()),
                card: Some((
                    *card,
                    ObjectCharacteristics::card(*definition, CardPartId::PRIMARY),
                )),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Graveyard,
            })
            .collect::<Vec<_>>();
        let maximum = options.len();
        self.queue_decision(
            player,
            format!("Exile any number of cards as {name} enters the battlefield"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            0..=maximum,
            false,
            options,
            DecisionContinuation::BattlefieldEntryExile {
                player,
                entering,
                candidates: candidates.to_vec(),
            },
        );
    }

    /// Exiles the chosen cards, links them to the permanent that is
    /// arriving, and lets the entry go on.
    pub(in crate::game) fn resume_entry_exile_choice(
        &mut self,
        player: PlayerId,
        entering: GameObjectId,
        candidates: &[(GameObjectId, CardDefinitionId)],
        options: &[u32],
    ) {
        let chosen = options
            .iter()
            .filter_map(|option| usize::try_from(*option).ok())
            .filter_map(|index| candidates.get(index))
            .map(|(card, _)| *card)
            .collect::<Vec<_>>();
        for card in chosen {
            let Some(card) = remove_card(&mut self.players[player.index()].graveyard, card) else {
                continue;
            };
            let (card, _zone_change) = self.zone_change_card(card);
            let exiled = card.id;
            self.players[player.index()].exile.push(card);
            self.linked_exiles.push((entering, exiled));
        }
        self.continue_pending_events();
    }
}
