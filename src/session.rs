//! Transport-independent, externally controlled matches with exact choices.
//!
//! Hosts own authentication, revisions, durable journals, and retry receipts.
//! This layer owns only rules execution and information visible to each seat.

mod updates;

pub use updates::SessionUpdates;

use crate::match_play::MatchMode;
use crate::{Action, CardCatalog, Format, Game, PlayerId, poc, protocol};
use serde_json::Value;

const FORCED_ACTION_LIMIT: usize = 50_000;

/// An authoritative match controlled by two external seats.
///
/// Cloning supports a host's rollback before committing a durable command.
/// It is not a serialized checkpoint or an observation reconstruction.
#[derive(Clone)]
pub struct ExactSession {
    game: Game,
    catalog: CardCatalog,
    updates: SessionUpdates,
    action_proposals: bool,
}

impl ExactSession {
    /// Starts a match using registered decks. Call `advance_forced` before
    /// offering the first observation and after each accepted command.
    ///
    /// # Errors
    /// Rejects unknown decks or invalid match setup.
    pub fn new(
        format: Format,
        p1_deck: &str,
        p2_deck: &str,
        seed: u64,
        mode: MatchMode,
    ) -> Result<Self, String> {
        let catalog = poc::catalog().map_err(|error| error.to_string())?;
        let deck = |name| {
            protocol::deck_by_name_for_format(format, name)
                .ok_or_else(|| format!("unknown deck for {}: {name}", format.slug()))
        };
        let mut game = Game::new_with_format(
            format,
            catalog.clone(),
            [deck(p1_deck)?, deck(p2_deck)?],
            seed,
        )
        .map_err(|error| error.to_string())?;
        game.set_match_mode(mode)?;
        Ok(Self {
            game,
            catalog,
            updates: SessionUpdates::default(),
            action_proposals: false,
        })
    }

    /// Returns canonical seat-safe protocol data and retained session updates.
    #[must_use]
    pub fn observe(&self, seat: PlayerId) -> Value {
        self.observation(seat, self.action_proposals)
    }

    /// Uses state observations and conservative forced continuation queries.
    #[must_use]
    pub fn with_action_proposals(mut self) -> Self {
        self.action_proposals = true;
        self
    }

    /// Explicit, potentially expensive legacy legal-action discovery.
    #[must_use]
    pub fn discover(&self, seat: PlayerId) -> Value {
        self.observation(seat, false)
    }

    fn observation(&self, seat: PlayerId, direct: bool) -> Value {
        let observation = if direct {
            self.game.observe_state(seat)
        } else {
            self.game.observe(seat)
        };
        let actions = protocol::protocol_actions(&observation);
        let mut value = protocol::observation_json_for_format(
            &self.catalog,
            self.game.format(),
            &observation,
            self.game.in_pregame(),
            &actions,
        );
        value["match"] = self.game.match_json(seat);
        value["updates"] = self.updates.json(&self.catalog, seat);
        if direct {
            let object = value.as_object_mut().expect("observation object");
            object.remove("legalActions");
            object.remove("checkpoint");
            value["actionInterface"] = Value::from("proposal-v1");
            value["actionContext"] = self.game.action_context(seat);
            if let Some(offers) = value["actionContext"]
                .get_mut("castOffers")
                .and_then(Value::as_array_mut)
            {
                for offer in offers {
                    if let Some(card) = offer["card"].as_u64().and_then(|id| u32::try_from(id).ok())
                        && let Some(origin) = self
                            .game
                            .cast_offer_ability_origin(seat, crate::GameObjectId(card))
                    {
                        offer["cost"]["ability"] = protocol::ability_origin_json(origin);
                    }
                }
            }
            value["forcedAction"] = self
                .game
                .forced_action_without_search()
                .filter(|(player, _)| *player == seat)
                .map_or(Value::Null, |(_, action)| protocol::action_json(&action));
            value["actionAbilities"] = observation.battlefield.iter().map(|p|p.id)
                .chain(observation.emblems.iter().map(|p|p.id))
                .chain(observation.hand.iter().map(|(id,_)|*id))
                .chain(observation.graveyards.iter().flatten().map(|(id,_)|*id))
                .chain(observation.exiles.iter().flatten().map(|(id,_)|*id))
                .chain(observation.command_zones.iter().flatten().map(|(id,_)|*id))
                .map(|source| serde_json::json!({"source":source.0,"playPermission":self.game.action_play_permission(seat,source),"grantedCastCosts":self.game.action_cast_costs(seat,source),"abilities":self.game.action_abilities(source).into_iter().map(|(origin,ability)|protocol::proposal_ability_json(origin,ability)).collect::<Vec<_>>()}))
                .collect();
        }
        value
    }

    /// Returns the seat holding the next decision.
    #[must_use]
    pub fn decision_seat(&self) -> Option<PlayerId> {
        self.game.decision_player()
    }

    /// Applies exactly one indexed action, exact action value, or explicit
    /// decision selection. Action values omit the observation's `index` field.
    ///
    /// # Errors
    /// Rejects malformed choices, stale decisions, out-of-turn submissions,
    /// and choices not legal in the current observation.
    pub fn apply_choice(&mut self, seat: PlayerId, choice: &Value) -> Result<(), String> {
        if let Some(proposal) = choice.get("proposal") {
            if choice.as_object().is_none_or(|object| object.len() != 1) {
                return Err("proposal choices contain only proposal".into());
            }
            return self
                .apply_proposal(seat, proposal)
                .map_err(|error| error.to_string());
        }
        if self.decision_seat() != Some(seat) {
            return Err("the seat does not hold a decision".into());
        }
        let choice = choice.as_object().ok_or("invalid choice")?;
        if choice.len() == 2 && choice.contains_key("decision") && choice.contains_key("options") {
            let decision = uint(&choice["decision"])?;
            if self
                .game
                .observe_state(seat)
                .decision
                .as_ref()
                .map(|value| value.id)
                != Some(decision)
            {
                return Err("stale decision".into());
            }
            let options = choice["options"].as_array().ok_or("invalid options")?;
            if options.len() > 4096 {
                return Err("too many decision options".into());
            }
            return self.apply_manual(
                seat,
                Action::ChooseDecision {
                    decision,
                    options: options.iter().map(uint).collect::<Result<_, _>>()?,
                },
            );
        }
        let observation = self.game.observe(seat);
        let actions = protocol::protocol_actions(&observation);
        let action = if choice.len() == 1 && choice.contains_key("index") {
            let index = uint(&choice["index"])?;
            actions
                .get(usize::try_from(index).map_err(|_| "illegal action index")?)
                .cloned()
                .ok_or("illegal action index")?
        } else if choice.len() == 1 && choice.contains_key("action") {
            let wanted = &choice["action"];
            let object = wanted.as_object().ok_or("invalid exact action")?;
            if object.contains_key("index") || !object.get("type").is_some_and(Value::is_string) {
                return Err("exact actions require a type and omit index".into());
            }
            let mut matches = actions
                .into_iter()
                .filter(|action| protocol::action_json(action) == *wanted);
            let action = matches
                .next()
                .ok_or("the exact action is not uniquely legal")?;
            if matches.next().is_some() {
                return Err("the exact action is not uniquely legal".into());
            }
            action
        } else {
            return Err("choose an index, an exact action, or decision and option IDs".into());
        };
        self.apply_manual(seat, action)
    }

    /// Concedes this seat's current game, regardless of priority.
    ///
    /// # Errors
    /// Rejects a concession the engine cannot accept.
    pub fn concede(&mut self, seat: PlayerId) -> Result<(), String> {
        self.apply_manual(seat, Action::Concede)
    }

    fn apply_manual(&mut self, seat: PlayerId, action: Action) -> Result<(), String> {
        let cursor = self.game.event_cursor();
        self.game
            .apply(seat, action)
            .map_err(|error| error.to_string())?;
        self.updates.clear(seat);
        self.updates.record(&self.game, cursor, None);
        Ok(())
    }

    /// Validates a complete typed proposal without discovering other plays.
    ///
    /// # Errors
    /// Returns a seat-safe typed refusal; the game remains unchanged.
    pub fn apply_proposal(
        &mut self,
        seat: PlayerId,
        proposal: &Value,
    ) -> Result<(), protocol::ProposalError> {
        let proposal = protocol::ActionProposal::parse(proposal)?;
        let cursor = self.game.event_cursor();
        proposal.apply(&mut self.game, seat)?;
        self.updates.clear(seat);
        self.updates.record(&self.game, cursor, None);
        Ok(())
    }

    /// Applies a legal ordinary priority pass without acknowledging retained updates.
    /// Hosts use this only for an explicitly authorized, bounded yield policy.
    ///
    /// # Errors
    /// Rejects a pass outside this seat's ordinary priority window.
    pub fn pass_priority_retaining_updates(&mut self, seat: PlayerId) -> Result<(), String> {
        if !self.game.has_action_priority(seat) {
            return Err("the seat does not hold ordinary priority".into());
        }
        let cursor = self.game.event_cursor();
        self.game
            .apply(seat, Action::PassPriority)
            .map_err(|error| error.to_string())?;
        self.updates.record(&self.game, cursor, None);
        Ok(())
    }

    /// Advances only unique continuations, preserving seat-visible events and
    /// automatic inspection decisions until that seat's next accepted command.
    ///
    /// # Errors
    /// Returns an engine error or a bounded-continuation limit failure. Hosts
    /// should restore their pre-command clone before retrying a failed commit.
    pub fn advance_forced(&mut self) -> Result<(), String> {
        self.advance_forced_until(|_| false).map(|_| ())
    }

    /// Advances unique continuations until a host's seat-visible stop guard fires.
    /// Returns the exact applied prefix length for deterministic host replay.
    ///
    /// # Errors
    /// Returns an engine error or the bounded-continuation limit failure.
    pub fn advance_forced_until(
        &mut self,
        mut stop: impl FnMut(&Self) -> bool,
    ) -> Result<usize, String> {
        for count in 0..FORCED_ACTION_LIMIT {
            if stop(self) || !self.advance_one_forced()? {
                return Ok(count);
            }
        }
        Err("game exceeded its forced action limit".into())
    }

    /// Replays a recorded forced prefix without recomputing a host policy.
    ///
    /// # Errors
    /// Rejects excessive counts, a prefix beyond available forced actions, or engine errors.
    pub fn advance_forced_exact(&mut self, count: usize) -> Result<(), String> {
        if count > FORCED_ACTION_LIMIT {
            return Err("invalid forced prefix length".into());
        }
        for _ in 0..count {
            if !self.advance_one_forced()? {
                return Err("forced prefix exceeds available continuations".into());
            }
        }
        Ok(())
    }

    fn advance_one_forced(&mut self) -> Result<bool, String> {
        let forced = if self.action_proposals {
            self.game.forced_action_without_search()
        } else {
            self.game.forced_action()
        };
        let Some((seat, action)) = forced else {
            return Ok(false);
        };
        let observation = self.game.observe_state(seat);
        let cursor = self.game.event_cursor();
        self.game
            .apply(seat, action.clone())
            .map_err(|error| error.to_string())?;
        self.updates.record(
            &self.game,
            cursor,
            observation
                .decision
                .as_ref()
                .map(|decision| (decision, &action)),
        );
        Ok(true)
    }

    /// Whether the complete match has concluded.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.game.result().is_some()
    }

    /// Public catalog data for this match's format.
    #[must_use]
    pub fn catalog_json(&self) -> Value {
        protocol::catalog_json_for_format(&self.catalog, self.game.format())
    }
}

fn uint(value: &Value) -> Result<u32, String> {
    value
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| "choice identifiers must be unsigned 32-bit integers".into())
}

#[cfg(test)]
mod tests;
