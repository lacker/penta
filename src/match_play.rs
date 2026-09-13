//! Match bookkeeping uses stable seats, independent of who starts each game.
use std::collections::BTreeMap;

use crate::{CardCatalog, CardDefinitionId, Deck, Format, PlayerId};

fn pool(deck: &Deck) -> BTreeMap<CardDefinitionId, usize> {
    let mut counts = BTreeMap::new();
    for card in deck.main.iter().chain(&deck.sideboard) {
        *counts.entry(*card).or_default() += 1;
    }
    counts
}

/// A match ends after one conclusion, or when a player has won twice.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MatchMode {
    #[default]
    OneConclusion,
    FirstToTwoWins,
}

impl MatchMode {
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::OneConclusion => "one-conclusion",
            Self::FirstToTwoWins => "first-to-two-wins",
        }
    }

    /// # Errors
    /// Rejects unknown match modes.
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "one-conclusion" => Ok(Self::OneConclusion),
            "first-to-two-wins" => Ok(Self::FirstToTwoWins),
            _ => Err(format!("unknown match mode: {value}")),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct MatchContext {
    pub initial_choice: bool,
    pub registered: [Deck; 2],
    pub decks: [Deck; 2],
    pub wins: [u32; 2],
    pub draws: u32,
    pub chooser: PlayerId,
    pub submitted: [bool; 2],
    pub seed: u64,
}

impl MatchContext {
    pub fn score(&self, result: Option<crate::GameResult>) -> ([u32; 2], u32) {
        let mut wins = self.wins;
        let mut draws = self.draws;
        match result {
            Some(crate::GameResult::Winner { winner, .. }) => wins[winner.index()] += 1,
            Some(crate::GameResult::Draw) => draws += 1,
            None => (),
        }
        (wins, draws)
    }

    pub fn chooser_after(&self, result: Option<crate::GameResult>) -> PlayerId {
        match result {
            Some(crate::GameResult::Winner { winner, .. }) => winner.opponent(),
            _ => self.chooser,
        }
    }

    pub fn valid_deck(
        &self,
        player: PlayerId,
        deck: &Deck,
        catalog: &CardCatalog,
        format: Format,
    ) -> bool {
        let registered = &self.registered[player.index()];
        let same_pool = if format
            .commander_definition()
            .is_some_and(|rules| rules.commander_swapping)
        {
            let mut current_pool = pool(deck);
            for id in &deck.commanders {
                *current_pool.entry(*id).or_default() += 1;
            }
            let mut original_pool = pool(registered);
            for id in &registered.commanders {
                *original_pool.entry(*id).or_default() += 1;
            }
            (1..=2).contains(&deck.commanders.len())
                && deck.sideboard == registered.sideboard
                && current_pool == original_pool
        } else {
            deck.commanders == registered.commanders && pool(deck) == pool(registered)
        };
        same_pool && deck.clone().validate_for_format(catalog, format).is_ok()
    }
}
