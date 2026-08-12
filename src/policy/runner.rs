use std::error::Error;
use std::fmt;

use super::Policy;
use crate::{ActionError, Game, GameResult, PlayerId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlayError {
    PolicyReturnedNoAction(PlayerId),
    IllegalAction(Box<ActionError>),
    ActionLimitExceeded(usize),
}

impl fmt::Display for PlayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PolicyReturnedNoAction(player) => {
                write!(formatter, "policy for {player} returned no action")
            }
            Self::IllegalAction(error) => {
                write!(formatter, "policy returned an illegal action: {error}")
            }
            Self::ActionLimitExceeded(limit) => {
                write!(formatter, "game exceeded its action limit of {limit}")
            }
        }
    }
}

impl Error for PlayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IllegalAction(error) => Some(error.as_ref()),
            Self::PolicyReturnedNoAction(_) | Self::ActionLimitExceeded(_) => None,
        }
    }
}

/// Plays a game to completion using one policy for each player.
///
/// # Errors
///
/// Returns [`PlayError`] if a policy fails to choose an action, chooses an
/// illegal action, or the game exceeds `action_limit`.
pub fn play_game(
    game: &mut Game,
    player_one: &mut dyn Policy,
    player_two: &mut dyn Policy,
    action_limit: usize,
) -> Result<GameResult, PlayError> {
    for _ in 0..action_limit {
        if let Some(result) = game.result() {
            return Ok(result);
        }
        let Some(player) = game.decision_player() else {
            return game
                .result()
                .ok_or(PlayError::ActionLimitExceeded(action_limit));
        };
        let observation = game.observe(player);
        let action = match player {
            PlayerId::One => player_one.choose_action(&observation),
            PlayerId::Two => player_two.choose_action(&observation),
        }
        .ok_or(PlayError::PolicyReturnedNoAction(player))?;
        game.apply_observed_action(&observation, action)
            .map_err(|error| PlayError::IllegalAction(Box::new(error)))?;
    }
    game.result()
        .ok_or(PlayError::ActionLimitExceeded(action_limit))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Action, PlayerObservation, poc};

    struct IllegalPolicy;

    impl Policy for IllegalPolicy {
        fn choose_action(&mut self, _observation: &PlayerObservation) -> Option<Action> {
            Some(Action::PassPriority)
        }
    }

    #[test]
    fn rejects_an_action_absent_from_the_fresh_observation() {
        let catalog = poc::catalog().expect("catalog builds");
        let deck = poc::goblins();
        let mut game = Game::new(catalog, [deck.clone(), deck], 1).expect("game starts");
        let mut first = IllegalPolicy;
        let mut second = IllegalPolicy;

        let error = play_game(&mut game, &mut first, &mut second, 1)
            .expect_err("passing priority during the opening mulligan is illegal");

        assert!(matches!(
            error,
            PlayError::IllegalAction(error)
                if matches!(
                    error.as_ref(),
                    ActionError::NotLegal {
                        player: PlayerId::One,
                        action: Action::PassPriority,
                    }
                )
        ));
    }
}
