//! Python bindings over [`penta::protocol`].
//!
//! Observations and the catalog cross into Python as canonical protocol JSON
//! strings — the same bytes every other consumer of the protocol sees — and
//! bots answer with an index into `legalActions`. Parse with `json.loads`;
//! see BOTS.md at the repository root for the schema.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use engine::protocol::{
    BotGame, Opponent, catalog_json_for_format, deck_names_for_format, parse_format_slug,
};
use engine::{Format, PlayerId};

fn seat_from_name(name: &str) -> PyResult<PlayerId> {
    match name {
        "p1" => Ok(PlayerId::One),
        "p2" => Ok(PlayerId::Two),
        other => Err(PyValueError::new_err(format!(
            "seat must be \"p1\" or \"p2\", got {other:?}"
        ))),
    }
}

fn format_from_slug(slug: &str) -> PyResult<Format> {
    parse_format_slug(slug).map_err(PyValueError::new_err)
}

/// One game of a supported Magic format, driven from Python.
///
/// With a built-in opponent, `act` plays your action and then lets the
/// opponent play until you have a real choice again. With
/// `opponent="external"` the game stops at every decision for either seat,
/// so one loop can drive both sides for self-play.
#[pyclass]
struct Game {
    inner: BotGame,
}

#[pymethods]
impl Game {
    /// Builds a local rollout world from one hosted observation. `hidden`
    /// supplies card-definition arrays for `hands.p1`/`hands.p2` where hidden
    /// and both `libraries` and `outsideGame` lists; `rollout_seed` controls
    /// only future local random choices and is unrelated to the host's private seed.
    #[staticmethod]
    #[pyo3(signature = (observation, hidden, rollout_seed=0))]
    fn from_observation(observation: &str, hidden: &str, rollout_seed: u64) -> PyResult<Self> {
        BotGame::from_observation_json(observation, hidden, rollout_seed)
            .map(|inner| Self { inner })
            .map_err(PyValueError::new_err)
    }

    /// Starts a game. `opponent` is `"handcrafted"`, `"random"`, or
    /// `"external"`; `opponent_seat` is `"p1"` or `"p2"`. `format`
    /// defaults to Old School for compatibility.
    #[new]
    #[pyo3(signature = (
        p1_deck,
        p2_deck,
        opponent="handcrafted",
        opponent_seat="p2",
        seed=0,
        format="old-school-93-94"
    ))]
    fn new(
        p1_deck: &str,
        p2_deck: &str,
        opponent: &str,
        opponent_seat: &str,
        seed: u64,
        format: &str,
    ) -> PyResult<Self> {
        let opponent = match opponent {
            "external" => Opponent::External,
            "random" => Opponent::Random,
            "handcrafted" => Opponent::Handcrafted,
            other => {
                return Err(PyValueError::new_err(format!(
                    "opponent must be \"handcrafted\", \"random\", or \"external\", got {other:?}"
                )));
            }
        };
        let seat = seat_from_name(opponent_seat)?;
        let format = format_from_slug(format)?;
        BotGame::new_with_format(format, p1_deck, p2_deck, opponent, seat, seed)
            .map(|inner| Self { inner })
            .map_err(PyValueError::new_err)
    }

    /// The seat that must act next (`"p1"`/`"p2"`), or `None` when the game
    /// is over.
    fn decision_seat(&self) -> Option<&'static str> {
        self.inner.decision_seat_name()
    }

    /// An independent copy of the game: same state, same future for the
    /// same actions — the built-in opponent's state included. Fork a game
    /// to roll out candidate lines without disturbing the original.
    #[pyo3(name = "clone")]
    fn clone_game(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    /// One seat's observation as protocol JSON. Defaults to the seat that
    /// must act.
    #[pyo3(signature = (seat=None))]
    fn observe(&self, seat: Option<&str>) -> PyResult<String> {
        let seat = match seat {
            Some(name) => seat_from_name(name)?,
            None => self
                .inner
                .decision_seat()
                .ok_or_else(|| PyValueError::new_err("the game is over; pass a seat explicitly"))?,
        };
        Ok(self.inner.observe_json(seat))
    }

    /// A seat's hand as JSON `[{objectId, definition}]`, unredacted.
    ///
    /// This is the simulation surface. `observe` stays redacted and is what a
    /// client should be shown; this reports what is really there, so a search
    /// bot can rearrange hidden state before a rollout. Nothing is hidden from
    /// you here because the game runs in your own process.
    fn hand(&self, seat: &str) -> PyResult<String> {
        Ok(self.inner.hand_json(seat_from_name(seat)?))
    }

    /// A seat's library, top card first. See `hand`.
    fn library(&self, seat: &str) -> PyResult<String> {
        Ok(self.inner.library_json(seat_from_name(seat)?))
    }

    /// Replaces a seat's hand with exactly these card definitions.
    ///
    /// The cards are built fresh, so this says what a hand *is* rather than
    /// moving cards around: to explore "their last card is either Lightning
    /// Bolt or Counterspell", set the same hand twice with a different last
    /// entry and roll both out. Nothing is conserved -- a hypothetical world
    /// has no reason to be.
    ///
    /// Definitions are the `definition` ids from `catalog()`.
    #[allow(clippy::needless_pass_by_value)]
    fn set_hand(&mut self, seat: &str, definitions: Vec<u64>) -> PyResult<()> {
        let seat = seat_from_name(seat)?;
        let cards: Vec<_> = definitions
            .into_iter()
            .map(|definition| {
                engine::CardDefinitionId::try_new(definition).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "card definition IDs must be between 1 and {}",
                        engine::CardDefinitionId::MAX
                    ))
                })
            })
            .collect::<PyResult<_>>()?;
        self.inner
            .set_hand(seat, &cards)
            .map_err(PyValueError::new_err)
    }

    /// Replaces a seat's library, top card first. See `set_hand`.
    #[allow(clippy::needless_pass_by_value)]
    fn set_library(&mut self, seat: &str, definitions: Vec<u64>) -> PyResult<()> {
        let seat = seat_from_name(seat)?;
        let cards: Vec<_> = definitions
            .into_iter()
            .map(|definition| {
                engine::CardDefinitionId::try_new(definition).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "card definition IDs must be between 1 and {}",
                        engine::CardDefinitionId::MAX
                    ))
                })
            })
            .collect::<PyResult<_>>()?;
        self.inner
            .set_library(seat, &cards)
            .map_err(PyValueError::new_err)
    }

    /// How many legal actions the acting seat has; 0 when the game is over.
    fn legal_action_count(&self) -> usize {
        self.inner.legal_action_count()
    }

    /// Plays one index from the acting seat's `legalActions`.
    fn act(&mut self, action_index: usize) -> PyResult<()> {
        self.inner.act(action_index).map_err(PyValueError::new_err)
    }

    /// Answers a pending decision with explicit option ids, for multi-pick
    /// decisions where the default expansion in `legalActions` is not what
    /// you want. The observation's `decision` object lists the options.
    // PyO3 materializes a Python sequence as an owned Vec at the language
    // boundary, even though the engine only needs a borrowed slice.
    #[allow(clippy::needless_pass_by_value)]
    fn choose_decision(&mut self, option_ids: Vec<u32>) -> PyResult<()> {
        self.inner
            .choose_decision(&option_ids)
            .map_err(PyValueError::new_err)
    }

    /// `None` while the game runs; `"p1"`/`"p2"` for the winner, `"draw"`
    /// otherwise.
    fn result(&self) -> Option<&'static str> {
        use engine::GameResult;
        match self.inner.result()? {
            GameResult::Draw => Some("draw"),
            GameResult::Winner {
                winner: PlayerId::One,
                ..
            } => Some("p1"),
            GameResult::Winner {
                winner: PlayerId::Two,
                ..
            } => Some("p2"),
        }
    }
}

/// Every card definition with legality for `format`, as protocol JSON.
#[pyfunction]
#[pyo3(signature = (format = "old-school-93-94"))]
fn catalog(format: &str) -> PyResult<String> {
    let format = format_from_slug(format)?;
    let catalog =
        engine::card::catalog().map_err(|error| PyValueError::new_err(error.to_string()))?;
    Ok(catalog_json_for_format(&catalog, format).to_string())
}

/// The built-in deck names for `format`.
#[pyfunction]
#[pyo3(signature = (format = "old-school-93-94"))]
fn deck_names(format: &str) -> PyResult<Vec<&'static str>> {
    Ok(deck_names_for_format(format_from_slug(format)?))
}

/// The engine package version. Use `simulation_fingerprint` for source identity.
#[pyfunction]
fn engine_version() -> &'static str {
    engine::protocol::ENGINE_VERSION
}

/// The breaking bot-wire epoch the JSON shapes follow.
#[pyfunction]
fn protocol_version() -> u32 {
    engine::protocol::PROTOCOL_VERSION
}

/// The conservative simulation-source identity for replay and model provenance.
#[pyfunction]
fn simulation_fingerprint() -> &'static str {
    engine::protocol::SIMULATION_FINGERPRINT
}

#[pymodule]
fn penta(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Game>()?;
    module.add_function(wrap_pyfunction!(catalog, module)?)?;
    module.add_function(wrap_pyfunction!(deck_names, module)?)?;
    module.add_function(wrap_pyfunction!(engine_version, module)?)?;
    module.add_function(wrap_pyfunction!(protocol_version, module)?)?;
    module.add_function(wrap_pyfunction!(simulation_fingerprint, module)?)?;
    Ok(())
}
