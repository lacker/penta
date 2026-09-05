//! The canonical wire format for bots, and a facade for driving games with it.
//!
//! Everything a bot ever sees crosses this boundary as JSON produced here:
//! the Python bindings, the C FFI, and any future tournament server all call
//! these functions, so a bot trained locally and a bot competing remotely
//! read byte-identical observations. The wire version changes only when an old
//! consumer could misinterpret an existing field or tag; additive fields and
//! ruleset changes are identified separately.
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
use observation_json::{decision_json, stack_object_json};

/// The breaking bot-wire epoch. Bumped when an old consumer could misread an
/// existing field or tag: removal, rename, type change, or changed meaning.
/// Adding an optional field or offering different actions through the existing
/// vocabulary does not move this number; use [`SIMULATION_FINGERPRINT`] to pin
/// a conservative simulation identity instead. Version 1 dropped conceding
/// from the bot's actions. Version 2 added formats, game-object identity, and
/// structured casting choices.
/// Version 3 identifies trigger procedures and triggered stack objects; names
/// the exact printed, intrinsic, or granted ability selected by an activation;
/// distinguishes no mana cost from a printed `{0}`; exposes clause-derived
/// implementation coverage; and preserves structural provenance for granted
/// abilities. These changes form one compatibility boundary even though they
/// were developed across several commits. Version 4 adds executable modal
/// spell choices, public counterability and permanent-choice state, and
/// enables previously unsupported cards whose actions now appear in
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
/// into resolution. Version 21 makes that checkpoint a complete typed
/// decision-boundary snapshot, including deferred execution, dynamic object
/// state, restricted mana, and retired-object last-known information. Version
/// 22 establishes open-world JSON objects, advertises
/// named capabilities and a simulation fingerprint, and separates
/// checkpoint/replay formats from the bot wire. Version 23 renames the
/// canonical Innistrad-through-Dragon's-Maze format slug from
/// `isd-rtr-standard` to `isd-dgm-standard`. Version 24 makes a permanent's
/// `blocking` an array of attacker ids rather than one id or null, because a
/// creature can block a band as a group and can be allowed more than one block.
/// Version 25 renames
/// an activated ability's singular `costObject` to the `costObjects` array.
/// Version 26 removes synthetic card-definition identities from tokens and
/// reports their creator-owned inline characteristics instead. Version 27
/// does the same for face-down spells and permanents: their rules-defined
/// characteristics travel inline while their physical card keeps its real
/// definition identity. Version 28 broadens catalog mana symbols beyond
/// ordinary two-color hybrid and records explicitly announced flexible-mana
/// alternatives on cast actions. Version 29 removes the `isd-dgm-standard`
/// format value; use the final pre-Theros `isd-m14-standard` profile. Version
/// 30 replaces the card implementation-status values `partial` and
/// `metadataOnly` with the single whole-card value `unsupported`.
pub const PROTOCOL_VERSION: u32 = 30;

/// The engine package release. This is ordinary Cargo `SemVer`, not an exact
/// ruleset identity; use [`SIMULATION_FINGERPRINT`] for replay and model
/// provenance.
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A deterministic identity for the production engine source, resolved core
/// dependency closure, repository deck data, and pinned toolchain that define
/// simulation behavior in this repository.
///
/// It is generated at build time, so parallel branches never edit a shared
/// successor value. Exact replays, reconstructed checkpoints, and trained
/// weights should pin this rather than the package release.
pub const SIMULATION_FINGERPRINT: &str = env!("PENTA_SIMULATION_FINGERPRINT");

/// The same fingerprint with a C terminator for borrowed FFI access.
pub const SIMULATION_FINGERPRINT_NUL: &str = concat!(env!("PENTA_SIMULATION_FINGERPRINT"), "\0");

/// Named additive facilities emitted by this wire epoch. Consumers may ignore
/// capabilities they do not use; hosted bots may require the subset they use.
pub const PROTOCOL_CAPABILITIES: &[&str] = &["reconstruction.checkpoint.v12"];

/// Capabilities every bot must understand before a host may assign it a game.
/// The base indexed-action contract currently needs no optional facility.
pub const REQUIRED_BOT_CAPABILITIES: &[&str] = &[];

/// Version of the hidden-safe reconstruction payload nested at `checkpoint`.
pub const CHECKPOINT_VERSION: u32 = 12;

/// Undeclared hosted bots predate negotiation and therefore belong to the last
/// wire epoch that could not make an explicit declaration. They do not
/// silently opt into protocol 22's open-world contract.
pub const LEGACY_UNDECLARED_PROTOCOL_VERSION: u32 = 21;

/// Checks whether a hosted bot and this engine can consume each other's
/// compatibility declarations. Unknown extra capabilities are harmless.
///
/// # Errors
///
/// Returns a stable explanatory message for an epoch mismatch, the first
/// missing capability on either side, or a required simulation mismatch.
pub fn check_bot_compatibility(
    protocol_version: u32,
    supported_capabilities: &[&str],
    required_capabilities: &[&str],
    required_simulation_fingerprint: Option<&str>,
) -> Result<(), String> {
    if protocol_version != PROTOCOL_VERSION {
        return Err(format!(
            "bot protocol {protocol_version} does not match {PROTOCOL_VERSION}"
        ));
    }
    if let Some(missing) = REQUIRED_BOT_CAPABILITIES
        .iter()
        .find(|required| !supported_capabilities.contains(required))
    {
        return Err(format!("bot is missing required capability {missing}"));
    }
    if let Some(missing) = required_capabilities
        .iter()
        .find(|required| !PROTOCOL_CAPABILITIES.contains(required))
    {
        return Err(format!("server is missing required capability {missing}"));
    }
    if required_simulation_fingerprint.is_some_and(|required| required != SIMULATION_FINGERPRINT) {
        return Err("server simulation fingerprint does not match bot requirement".into());
    }
    Ok(())
}

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
