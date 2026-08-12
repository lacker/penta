//! The canonical wire format for bots, and a facade for driving games with it.
//!
//! Everything a bot ever sees crosses this boundary as JSON produced here:
//! the Python bindings, the C FFI, and any future tournament server all call
//! these functions, so a bot trained locally and a bot competing remotely
//! read byte-identical observations. Treat any change to the shapes below as
//! a protocol version bump.
//!
//! Seats are named `"p1"` ([`PlayerId::One`]) and `"p2"` ([`PlayerId::Two`]).
//! A bot acts by sending back the `index` of one of its observation's
//! `legalActions`; the engine validates every index against the legal list,
//! so no illegal move can be expressed at all.

use crate::{CardCatalog, Format, Game, HandcraftedPolicy, PlayerId, RandomPolicy};

#[cfg(test)]
use crate::card::{BasicLandType, SpellForm};
#[cfg(test)]
use crate::casting::{CastChoices, CastSignature};
#[cfg(test)]
use crate::game::{DecisionKind, DecisionObservation, DecisionOrderSemantics, StackObservation};
#[cfg(test)]
use crate::{
    AbilityOrigin, Action, GameObjectId, GameResult, ManaColor, PlayerObservation, StackObjectKind,
    Target, poc,
};
#[cfg(test)]
use serde_json::{Value, json};

mod action_json;
mod bot_game;
mod catalog_json;
mod decks;
mod json_common;
mod observation_json;

pub use action_json::{action_json, protocol_actions};
pub use catalog_json::{catalog_json, catalog_json_for_format};
pub use decks::{
    deck_by_name, deck_by_name_for_format, deck_names, deck_names_for_format, parse_format_slug,
};
pub use observation_json::{observation_json, observation_json_for_format};

#[cfg(test)]
use observation_json::{card_part_name, decision_json, stack_object_json};

/// The wire contract: the JSON shapes here and the action space they
/// describe. Bumped whenever a bot written against the old number could
/// misread the new output — a renamed field, or a change to what appears in
/// `legalActions`. Version 1 dropped conceding from the bot's actions. Version
/// 2 added formats, game-object identity, and structured casting choices.
/// Version 3 identifies trigger procedures and triggered stack objects; names
/// the exact printed, intrinsic, or granted ability selected by an activation;
/// distinguishes no mana cost from a printed `{0}`; exposes clause-derived
/// implementation coverage; and preserves structural provenance for granted
/// abilities. These changes form one compatibility boundary even though they
/// were developed across several commits. Version 4 adds executable modal
/// spell choices, public counterability and permanent-choice state, and
/// enables previously metadata-only cards whose actions now appear in
/// legal-action lists. Version 5 is upstream's post-Innistrad action contract.
/// Version 6 adds one activation action per affordable value of X. Version 7
/// exposes the priority window between first-strike and regular combat damage
/// and adds newly executable keyword and alternative-casting actions to
/// legal-action lists. Version 11 assigns instantiated spell and ability target
/// slots positionally, including flattened target ranges for selected modes.
/// Version 15 adds planeswalker combat: attack defenders, damage to
/// planeswalkers, and the loyalty state a client needs to render them.
/// Version 16 completes Boros Charm, adding its target-free Indestructible mode
/// and planeswalker targets for its damage mode to supported-format actions.
/// Version 17 makes Mana Vault's optional upkeep payment available even while
/// it is untapped, adding that decision to supported-format legal actions.
/// Version 18 adds host-enforced timeout as a game-result reason. Version 19
/// adds the hidden-safe `checkpoint` object and observation reconstruction
/// entry points used for local determinizations. Version 20 makes Chaos Orb's
/// Old School activation untargeted and moves its nontoken-permanent choice
/// into resolution.
pub const PROTOCOL_VERSION: u32 = 20;

/// The engine crate version. Rules behavior is part of the contract too: a
/// fix can change what a trained policy sees even when the shapes hold
/// still, so pin this alongside any trained weights.
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// One shy of the engine's own replay guard, so a runaway game fails here
/// with a protocol error instead of an engine panic.
const ACTION_LIMIT: usize = 50_000;

/// Which policy, if any, plays the seat a bot is not driving.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opponent {
    /// No built-in opponent: the caller drives both seats (self-play).
    External,
    /// The seeded uniform-random baseline.
    Random,
    /// The strongest built-in policy.
    Handcrafted,
}

// Concrete policy types rather than a boxed trait object, so the enum stays
// Clone — forking a game for search clones the opponent's state with it —
// and Send + Sync, which the Python bindings need to move games across
// threads and parallel rollout collection wants anyway.
#[derive(Clone)]
enum OpponentPolicy {
    External,
    Random(RandomPolicy),
    Handcrafted(HandcraftedPolicy),
}

/// A game driven through the bot protocol.
///
/// With a scripted opponent, [`BotGame::act`] plays your action and then lets
/// the opponent play until you have a real choice again, exactly like a
/// hotseat against the built-in bot. With [`Opponent::External`] it stops at
/// every decision, whichever seat owns it, so one loop can drive both sides
/// for self-play.
///
/// Cloning a `BotGame` snapshots everything — the game and any scripted
/// opponent's state — so a clone fed the same indices replays the identical
/// game, and a clone fed different ones never disturbs the original. That
/// fork-and-try is the primitive rollout- and tree-search bots are built on.
#[derive(Clone)]
pub struct BotGame {
    game: Game,
    catalog: CardCatalog,
    format: Format,
    opponent_seat: PlayerId,
    opponent: OpponentPolicy,
}

#[cfg(test)]
mod tests;
