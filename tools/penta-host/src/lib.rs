//! A single match's volatile authority. No files, model calls, or strategic choices.

mod config;
mod events;
mod interaction;
mod metrics;
mod play;
mod replay;
pub mod server;
mod yielding;

pub use config::Config;
pub use replay::replay_bundle;

use std::collections::{BTreeMap, VecDeque};
use std::fmt::Write;
use std::time::Instant;

use penta::{PlayerId, protocol, session::ExactSession};
use serde_json::{Value, json};

const DIAGNOSTIC_LIMIT: usize = 128;

/// An authenticated request refusal. Diagnostics never include bearer secrets.
#[derive(Debug)]
pub struct HostError {
    pub status: u16,
    pub message: String,
}

impl HostError {
    pub(crate) fn new(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

type Result<T> = std::result::Result<T, HostError>;

#[derive(Clone, Copy)]
enum Access {
    Organizer,
    Seat(PlayerId),
}

#[derive(Clone)]
struct Receipt {
    method: String,
    args: Value,
    value: Value,
}

/// Match state and all capabilities live in this object until explicit close.
pub struct Host {
    config: Config,
    seed: u64,
    session: ExactSession,
    observations: [Value; 2],
    event_logs: [Vec<Value>; 2],
    organizer: String,
    seats: [String; 2],
    revision: String,
    journal: Vec<Value>,
    transcripts: [Vec<Value>; 2],
    receipts: [BTreeMap<String, Receipt>; 2],
    diagnostics: VecDeque<Value>,
    command_metrics: BTreeMap<String, metrics::CommandMetric>,
    failure: Option<Value>,
    yields: [Option<yielding::YieldPolicy>; 2],
    catalog: Option<Value>,
    started: Instant,
    closed: bool,
}

impl Host {
    /// Creates one match and independent random organizer/seat credentials.
    ///
    /// # Errors
    /// Fails for invalid setup, unavailable OS randomness, or engine failure.
    pub fn new(config: Config) -> std::result::Result<Self, String> {
        let mut seed = [0_u8; 8];
        getrandom::fill(&mut seed).map_err(|error| error.to_string())?;
        Self::with_seed(config, u64::from_le_bytes(seed))
    }

    fn with_seed(config: Config, seed: u64) -> std::result::Result<Self, String> {
        let session = config.session(seed)?;
        let observations = [PlayerId::One, PlayerId::Two].map(|seat| session.observe(seat));
        let event_logs = observations.each_ref().map(|observation| {
            observation["updates"]
                .as_array()
                .expect("canonical updates")
                .clone()
        });
        let mut host = Self {
            config,
            seed,
            session,
            observations,
            event_logs,
            organizer: secret()?,
            seats: [secret()?, secret()?],
            revision: secret()?,
            journal: Vec::new(),
            transcripts: [Vec::new(), Vec::new()],
            receipts: [BTreeMap::new(), BTreeMap::new()],
            diagnostics: VecDeque::new(),
            command_metrics: BTreeMap::new(),
            failure: None,
            yields: [None, None],
            catalog: None,
            started: Instant::now(),
            closed: false,
        };
        host.remember_views();
        Ok(host)
    }

    /// Launch output belongs to the organizer; seat credentials are retrieved later.
    #[must_use]
    pub fn launch_info(&self, server: &str) -> Value {
        json!({"apiVersion":2,"server":server,"organizerSecret":self.organizer,"status":"ready"})
    }

    /// Whether the organizer has explicitly closed the host.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.closed
    }

    fn authorize(&self, token: &str) -> Result<Access> {
        if token_matches(&self.organizer, token) {
            return Ok(Access::Organizer);
        }
        for seat in [PlayerId::One, PlayerId::Two] {
            if token_matches(&self.seats[seat.index()], token) {
                return Ok(Access::Seat(seat));
            }
        }
        Err(HostError::new(403, "invalid capability"))
    }

    /// Dispatches a request using only the supplied capability's authority.
    ///
    /// # Errors
    /// Rejects unknown methods, invalid input, stale choices, or insufficient access.
    pub fn request(&mut self, token: &str, method: &str, args: &Value) -> Result<Value> {
        let access = self.authorize(token)?;
        if !args.is_object() {
            return Err(HostError::new(400, "args must be an object"));
        }
        if self.closed {
            return Err(HostError::new(410, "match host is closed"));
        }
        match access {
            Access::Organizer => self.organize(method, args),
            Access::Seat(seat) => self.deliver(seat, method, args),
        }
    }

    fn organize(&mut self, method: &str, args: &Value) -> Result<Value> {
        keys(args, &[])?;
        match method {
            "seats" => {
                Ok(json!({"seats":{"p1":{"secret":self.seats[0]},"p2":{"secret":self.seats[1]}}}))
            }
            "status" => Ok(json!({
                "status":self.status(), "config":self.config,
                "decisionSeat":self.session.decision_seat().map(seat_name),
                "commands":self.journal.len(), "elapsedMs":self.started.elapsed().as_millis(),
                "failure":self.failure, "simulationFingerprint":protocol::SIMULATION_FINGERPRINT,
                "commandMetrics":self.command_metrics,
            })),
            "export" => Ok(self.export()),
            "close" => {
                self.closed = true;
                Ok(json!({"status":"closed"}))
            }
            _ => Err(HostError::new(403, "method unavailable to this capability")),
        }
    }

    fn pilot(&mut self, seat: PlayerId, method: &str, args: &Value) -> Result<Value> {
        match method {
            "observe" | "wait" => {
                keys(args, &["waitMs"])?;
                number(args, "waitMs", 0, 25_000)?;
                Ok(self.view(seat))
            }
            "discover" => {
                keys(args, &[])?;
                let mut view = self.view(seat);
                view["apiVersion"] = json!(1); // Explicit legacy discovery retains its menu envelope.
                if view.get("observation").is_some() {
                    view["observation"] = self.session.discover(seat);
                }
                Ok(view)
            }
            "choose" | "play" | "concede" | "interact" => self.play(seat, method, args),
            "yield" | "cancelYield" => self.yield_request(seat, method, args),
            "receipt" => {
                keys(args, &["requestId"])?;
                let id = request_id(args)?;
                Ok(json!({"receipt":self.receipts[seat.index()].get(id).map(|r| &r.value)}))
            }
            "transcript" => {
                keys(args, &["offset", "limit"])?;
                let transcript = &self.transcripts[seat.index()];
                let offset = number(args, "offset", 0, usize::MAX)?;
                let limit = number(args, "limit", 25, 100)?;
                if limit == 0 {
                    return Err(HostError::new(400, "limit must be positive"));
                }
                let end = offset.saturating_add(limit).min(transcript.len());
                let entries = transcript.get(offset..end).unwrap_or_default();
                Ok(
                    json!({"seat":seat_name(seat),"offset":offset,"total":transcript.len(),
                    "entries":entries,"nextOffset":(end < transcript.len()).then_some(end)}),
                )
            }
            "note" => {
                keys(args, &["text"])?;
                let text = args["text"]
                    .as_str()
                    .filter(|text| text.len() <= 16_384)
                    .ok_or_else(|| {
                        HostError::new(400, "text must be a string of at most 16384 bytes")
                    })?;
                self.remember(seat, "note", json!(text));
                Ok(json!({"saved":true}))
            }
            "catalog" => {
                keys(args, &[])?;
                Ok(self
                    .catalog
                    .get_or_insert_with(|| self.session.catalog_json())
                    .clone())
            }
            _ => Err(HostError::new(403, "method unavailable to this capability")),
        }
    }

    fn status(&self) -> &'static str {
        if self.failure.is_some() {
            "paused"
        } else if self.session.is_finished() {
            "complete"
        } else {
            "active"
        }
    }

    fn view(&self, seat: PlayerId) -> Value {
        let mut view = self.base_view(seat);
        view["yield"] = self.yields[seat.index()]
            .as_ref()
            .map_or(Value::Null, yielding::YieldPolicy::json);
        view
    }

    fn base_view(&self, seat: PlayerId) -> Value {
        if self.failure.is_some() {
            return json!({"apiVersion":2,"status":"paused",
                "message":"Match paused after an engine failure; the organizer can export diagnostics."});
        }
        if !self.session.is_finished() && self.session.decision_seat() != Some(seat) {
            return json!({"apiVersion":2,"status":"waiting"});
        }
        json!({"apiVersion":2,
            "status":if self.session.is_finished() { "complete" } else { "ready" },
            "revision":self.revision,"observation":self.observations[seat.index()]})
    }

    fn remember(&mut self, seat: PlayerId, kind: &str, value: Value) {
        let transcript = &mut self.transcripts[seat.index()];
        let mut entry = json!({"sequence":transcript.len(),"kind":kind});
        entry["value"] = value;
        transcript.push(entry);
    }

    fn remember_views(&mut self) {
        for seat in [PlayerId::One, PlayerId::Two] {
            let mut view = self.view(seat);
            if let Some(observation) = view.get_mut("observation") {
                // Checkpoints are large derived search data, not remembered game facts.
                observation
                    .as_object_mut()
                    .expect("canonical observation")
                    .remove("checkpoint");
                self.remember(seat, "observation", view);
            }
        }
    }

    fn diagnose(&mut self, value: Value) {
        if self.diagnostics.len() == DIAGNOSTIC_LIMIT {
            self.diagnostics.pop_front();
        }
        self.diagnostics.push_back(value);
    }

    fn export(&self) -> Value {
        json!({
            "nativeReplayVersion":3,
            "simulationFingerprint":protocol::SIMULATION_FINGERPRINT,
            "protocolVersion":protocol::PROTOCOL_VERSION,
            "engineVersion":protocol::ENGINE_VERSION,
            "config":self.config, "seed":self.seed.to_string(), "commands":self.journal,
            "status":self.status(), "failure":self.failure,
            "observations":{"p1":self.observations[0],"p2":self.observations[1]},
            "events":{"p1":self.event_logs[0],"p2":self.event_logs[1]},
            "transcripts":{"p1":self.transcripts[0],"p2":self.transcripts[1]},
            "receipts":{"p1":self.receipts[0].values().map(|r| &r.value).collect::<Vec<_>>(),
                        "p2":self.receipts[1].values().map(|r| &r.value).collect::<Vec<_>>()},
            "diagnostics":self.diagnostics, "elapsedMs":self.started.elapsed().as_millis(),
            "commandMetrics":self.command_metrics,
        })
    }
}

fn secret() -> std::result::Result<String, String> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes).map_err(|error| error.to_string())?;
    let mut token = String::with_capacity(64);
    for byte in bytes {
        write!(token, "{byte:02x}").expect("writing into String");
    }
    Ok(token)
}

fn token_matches(expected: &str, presented: &str) -> bool {
    expected.len() == presented.len()
        && expected
            .bytes()
            .zip(presented.bytes())
            .fold(0_u8, |difference, (a, b)| difference | (a ^ b))
            == 0
}

fn seat_name(seat: PlayerId) -> &'static str {
    match seat {
        PlayerId::One => "p1",
        PlayerId::Two => "p2",
    }
}

fn keys(args: &Value, allowed: &[&str]) -> Result<()> {
    if args
        .as_object()
        .is_some_and(|object| object.keys().all(|key| allowed.contains(&key.as_str())))
    {
        Ok(())
    } else {
        Err(HostError::new(400, "unknown argument"))
    }
}

fn number(args: &Value, key: &str, default: usize, maximum: usize) -> Result<usize> {
    args.get(key).map_or(Ok(default), |value| {
        value
            .as_u64()
            .and_then(|n| usize::try_from(n).ok())
            .filter(|n| *n <= maximum)
            .ok_or_else(|| HostError::new(400, format!("invalid {key}")))
    })
}

fn request_id(args: &Value) -> Result<&str> {
    args["requestId"]
        .as_str()
        .filter(|id| !id.is_empty() && id.len() <= 128)
        .ok_or_else(|| HostError::new(400, "requestId must contain 1 to 128 bytes"))
}

#[cfg(test)]
mod proposal_tests;
#[cfg(test)]
mod tests;
