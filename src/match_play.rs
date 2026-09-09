//! Match bookkeeping uses stable seats, independent of who starts each game.
use std::collections::BTreeMap;

use crate::{CardCatalog, CardDefinitionId, Deck, Format, PlayerId};

/// Fixed registrations and completed game results for a first-to-two match.
#[derive(Clone, Debug)]
pub struct BestOfThree {
    registered: [Deck; 2],
    wins: [u8; 2],
    games: u32,
    chooser: PlayerId,
}

impl BestOfThree {
    #[must_use]
    pub const fn new(registered: [Deck; 2], chooser: PlayerId) -> Self {
        Self {
            registered,
            wins: [0, 0],
            games: 0,
            chooser,
        }
    }

    #[must_use]
    pub const fn wins(&self) -> [u8; 2] {
        self.wins
    }

    #[must_use]
    pub const fn games(&self) -> u32 {
        self.games
    }

    #[must_use]
    pub const fn chooser(&self) -> PlayerId {
        self.chooser
    }

    #[must_use]
    pub fn winner(&self) -> Option<PlayerId> {
        [PlayerId::One, PlayerId::Two]
            .into_iter()
            .find(|seat| self.wins[seat.index()] == 2)
    }

    /// Record one completed game. Draws preserve the previous play/draw chooser.
    /// The caller owns game identity and must record each result once.
    ///
    /// # Errors
    /// Returns an error if the match is already finished.
    pub fn record(&mut self, winner: Option<PlayerId>) -> Result<(), String> {
        if self.winner().is_some() {
            return Err("the match is already finished".into());
        }
        self.games += 1;
        if let Some(seat) = winner {
            self.wins[seat.index()] += 1;
            self.chooser = seat.opponent();
        }
        Ok(())
    }

    /// Checks construction rules and conserves every registered physical copy.
    /// Sideboard cards used during a game do not change the registration.
    ///
    /// # Errors
    /// Rejects additions, removals, or an illegal main deck / sideboard split.
    pub fn validate_sideboard(
        &self,
        seat: PlayerId,
        deck: &Deck,
        catalog: &CardCatalog,
        format: Format,
    ) -> Result<(), String> {
        if pool(deck) != pool(&self.registered[seat.index()]) {
            return Err("use exactly the cards registered for this match".into());
        }
        deck.clone()
            .validate_for_format(catalog, format)
            .map_err(|error| error.to_string())?;
        Ok(())
    }
}

fn pool(deck: &Deck) -> BTreeMap<CardDefinitionId, usize> {
    let mut counts = BTreeMap::new();
    for card in deck.main.iter().chain(&deck.sideboard) {
        *counts.entry(*card).or_default() += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{self, cards};

    fn deck() -> Deck {
        Deck {
            main: vec![cards::MOUNTAIN; 60],
            sideboard: vec![cards::FOREST; 15],
        }
    }

    #[test]
    fn best_of_three_scores_draws_and_stops_at_two_wins() {
        let mut series = BestOfThree::new([deck(), deck()], PlayerId::One);
        series.record(Some(PlayerId::One)).unwrap();
        series.record(None).unwrap();
        assert_eq!(series.chooser(), PlayerId::Two);
        series.record(Some(PlayerId::Two)).unwrap();
        assert_eq!(series.winner(), None);
        series.record(Some(PlayerId::One)).unwrap();
        assert_eq!(series.wins(), [2, 1]);
        assert_eq!(series.games(), 4);
        assert_eq!(series.winner(), Some(PlayerId::One));
        assert!(series.record(None).is_err());
    }

    #[test]
    fn best_of_three_sideboarding_conserves_copies_and_validates_sizes() {
        let catalog = card::catalog().unwrap();
        let series = BestOfThree::new([deck(), deck()], PlayerId::One);
        let mut swapped = deck();
        std::mem::swap(&mut swapped.main[0], &mut swapped.sideboard[0]);
        assert!(
            series
                .validate_sideboard(PlayerId::One, &swapped, &catalog, Format::OldSchool9394)
                .is_ok()
        );
        swapped.main[1] = cards::FOREST;
        assert!(
            series
                .validate_sideboard(PlayerId::One, &swapped, &catalog, Format::OldSchool9394)
                .is_err()
        );
        let mut short = deck();
        short.sideboard.push(short.main.pop().unwrap());
        assert!(
            series
                .validate_sideboard(PlayerId::One, &short, &catalog, Format::OldSchool9394)
                .is_err()
        );
        let mut larger = deck();
        larger.main.push(larger.sideboard.pop().unwrap());
        assert!(
            series
                .validate_sideboard(PlayerId::One, &larger, &catalog, Format::OldSchool9394)
                .is_ok()
        );
    }
}
