//! The boundary between a seat and the engine.
//!
//! The browser owns a [`Game`] today, which is why it can clone one to make a
//! checkpoint and read the raw event log. Neither is available to a client
//! whose engine is on a server, so both go behind this type: a seat asks for
//! a view, submits an action, and takes a checkpoint token it cannot look
//! inside.
//!
//! [`LocalSession`] is the in-process implementation, and its checkpoints
//! really are cloned games. A remote one would hold an identifier and let the
//! server keep the state. Nothing above this line can tell the difference,
//! which is the point.

use penta::{Action, Game, GameEvent, GameResult, PlayerId, PlayerObservation};

/// A point a session can be returned to. Nearly opaque: a seat may hold one,
/// hand it back, and ask how the board looked from its own seat. It cannot
/// read the engine out of it.
pub struct Checkpoint(Game);

impl Checkpoint {
    /// The saved position as this seat saw it, for deciding whether the
    /// checkpoint is still worth offering.
    pub fn observed_by(&self, seat: PlayerId) -> PlayerObservation {
        self.0.observe(seat)
    }
}

/// One seat's connection to a game.
pub struct LocalSession {
    game: Game,
}

impl LocalSession {
    pub const fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn decision_seat(&self) -> Option<PlayerId> {
        self.game.decision_player()
    }

    pub fn observe(&self, seat: PlayerId) -> PlayerObservation {
        self.game.observe(seat)
    }

    pub fn apply(&mut self, seat: PlayerId, action: Action) -> Result<(), Box<penta::ActionError>> {
        self.game.apply(seat, action).map_err(Box::new)
    }

    /// Ends the game because a seat ran out of time. Imposed by the host's
    /// clock rather than played, so it takes no action and needs no priority.
    pub fn lose_on_time(&mut self, seat: PlayerId) {
        self.game.lose_on_time(seat);
    }

    pub fn result(&self) -> Option<GameResult> {
        self.game.result()
    }

    pub fn in_pregame(&self) -> bool {
        self.game.in_pregame()
    }

    pub fn format(&self) -> penta::Format {
        self.game.format()
    }

    /// The seed. A local seat may show it because it owns the engine; a remote
    /// one would not be told.
    pub const fn seed(&self) -> u64 {
        self.game.seed()
    }

    pub fn event_cursor(&self) -> usize {
        self.game.event_cursor()
    }

    pub fn events_for(&self, seat: PlayerId) -> Vec<GameEvent> {
        self.game.events_for(seat)
    }

    pub fn events_for_since(&self, seat: PlayerId, cursor: usize) -> Vec<GameEvent> {
        self.game.events_for_since(seat, cursor)
    }

    /// Marks a point to come back to. Only ever taken before a choice that
    /// reveals nothing -- tapping for mana, declaring an attacker -- so
    /// returning to it cannot unsee anything.
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.game.clone())
    }

    pub fn restore(&mut self, checkpoint: Checkpoint) {
        self.game = checkpoint.0;
    }

    /// Which sources the engine's own payment policy would tap for an action,
    /// so the browser can show them before the click. A read of the shared
    /// rules, not of hidden state.
    pub fn mana_sources_for_action(
        &self,
        seat: PlayerId,
        action: &Action,
    ) -> Vec<penta::GameObjectId> {
        self.game.mana_sources_for_action(seat, action)
    }

    /// The printed ability behind an origin, for labelling it.
    pub fn ability_for_origin(
        &self,
        source: penta::GameObjectId,
        origin: penta::AbilityOrigin,
    ) -> Option<penta::card::AbilityDef> {
        self.game.ability_for_origin(source, origin)
    }

    pub fn special_action_for_effect(
        &self,
        source: penta::GameObjectId,
        effect_id: u64,
    ) -> Option<penta::card::AbilityDef> {
        self.game.special_action_for_effect(source, effect_id)
    }

    /// A throwaway copy for the pass preview to walk forward. Only a session
    /// that holds the engine can offer this; a remote one would ask its
    /// server for the destination instead.
    pub fn fork_for_preview(&self) -> Game {
        self.game.clone()
    }

    /// Dev cheats. Only a session that holds the engine can conjure a card,
    /// which is the honest shape: a server would refuse, and the feature that
    /// gates these is off in anything shipped.
    #[cfg(feature = "dev-cheats")]
    pub fn put_onto_battlefield(
        &mut self,
        seat: PlayerId,
        definition: penta::CardDefinitionId,
    ) -> Result<penta::GameObjectId, penta::ZoneError> {
        self.game.put_onto_battlefield(seat, definition)
    }

    #[cfg(feature = "dev-cheats")]
    pub fn put_into_graveyard(
        &mut self,
        seat: PlayerId,
        definition: penta::CardDefinitionId,
    ) -> Result<penta::GameObjectId, penta::ZoneError> {
        self.game.put_into_graveyard(seat, definition)
    }

    /// Engine access for a caller that genuinely owns the engine. Tests reach
    /// through here to build positions; nothing in the seat-facing client
    /// should need it.
    #[cfg(test)]
    pub const fn engine_mut(&mut self) -> &mut Game {
        &mut self.game
    }

    #[cfg(test)]
    pub const fn engine(&self) -> &Game {
        &self.game
    }
}
