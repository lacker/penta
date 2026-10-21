//! Explicit seat-owned pass policies. Conditions inspect only that seat's view.

mod conditions;

use penta::PlayerId;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::play::Failure;
use crate::{Host, HostError, Receipt, Result, keys, number, request_id};
use conditions::Condition;

const MAX_PASSES: usize = 256;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct YieldArgs {
    request_id: String,
    revision: String,
    until: Vec<Condition>,
    #[serde(default = "default_passes")]
    max_passes: usize,
    #[serde(default)]
    wait_ms: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct YieldOptions {
    until: Vec<Condition>,
    #[serde(default = "default_passes")]
    max_passes: usize,
}

impl YieldOptions {
    pub(crate) fn parse(value: &Value) -> Result<Self> {
        let options: Self = serde_json::from_value(value.clone())
            .map_err(|error| HostError::new(400, format!("invalid yield: {error}")))?;
        if options.until.is_empty()
            || options.until.len() > 16
            || options.max_passes == 0
            || options.max_passes > MAX_PASSES
        {
            return Err(HostError::new(
                400,
                "yield requires 1..16 conditions and 1..256 maxPasses",
            ));
        }
        Ok(options)
    }
}

const fn default_passes() -> usize {
    64
}

#[derive(Clone)]
pub(crate) struct YieldPolicy {
    request_id: String,
    until: Vec<Condition>,
    max_passes: usize,
    pub(crate) passes: usize,
    cursor: usize,
    game: Value,
    restarts: Value,
    reason: Option<&'static str>,
    condition: Option<usize>,
    awaiting_stop: bool,
}

impl YieldPolicy {
    pub(crate) fn json(&self) -> Value {
        json!({"requestId":self.request_id, "status":if self.reason.is_some() {"stopped"} else {"active"},
            "until":self.until, "maxPasses":self.max_passes, "passes":self.passes,
            "stopReason":self.reason, "conditionIndex":self.condition})
    }

    fn stop(&mut self, reason: &'static str) {
        if self.reason.is_none() {
            self.reason = Some(reason);
        }
    }
}

pub(crate) fn ordinary_priority(observation: &Value) -> bool {
    if observation["actionInterface"] == "proposal-v1" {
        return observation["actionContext"]["ordinaryPriority"] == true;
    }
    observation["decision"].is_null()
        && observation["pregame"] == false
        && observation["legalActions"]
            .as_array()
            .is_some_and(|actions| {
                actions
                    .iter()
                    .any(|action| action["type"] == "PassPriority")
            })
}

fn needs_input(observation: &Value) -> bool {
    ordinary_priority(observation) || observation["forcedAction"].is_null()
}

impl Host {
    pub(crate) fn yield_request(
        &mut self,
        seat: PlayerId,
        method: &str,
        args: &Value,
    ) -> Result<Value> {
        let id = request_id(args)?.to_owned();
        if let Some(receipt) = self.receipts[seat.index()].get(&id) {
            if receipt.method != method || receipt.args != *args {
                return Err(HostError::new(
                    409,
                    "requestId already used with different arguments",
                ));
            }
            let mut view = self.view(seat);
            view["receipt"] = receipt.value.clone();
            return Ok(view);
        }
        if method == "cancelYield" {
            keys(args, &["requestId"])?;
            if let Some(policy) = &mut self.yields[seat.index()] {
                policy.stop("cancelled");
            }
        } else {
            let options: YieldArgs = serde_json::from_value(args.clone())
                .map_err(|error| HostError::new(400, format!("invalid yield: {error}")))?;
            number(args, "waitMs", 0, 25_000)?;
            if options.until.is_empty()
                || options.until.len() > 16
                || options.max_passes == 0
                || options.max_passes > MAX_PASSES
            {
                return Err(HostError::new(
                    400,
                    "yield requires 1..16 conditions and 1..256 maxPasses",
                ));
            }
            if self.failure.is_some() || self.session.is_finished() {
                return Err(HostError::new(409, "match is paused or complete"));
            }
            if self.session.decision_seat() != Some(seat)
                || !ordinary_priority(&self.observations[seat.index()])
            {
                return Err(HostError::new(
                    409,
                    "yield requires this seat's ordinary priority",
                ));
            }
            if options.revision != self.revision {
                return Err(HostError::new(409, "stale revision; observe again"));
            }
            self.install_yield(
                seat,
                options.request_id,
                YieldOptions {
                    until: options.until,
                    max_passes: options.max_passes,
                },
            );
            // Validation above bounds waiting; the transport owns the actual wait.
            let _ = options.wait_ms;
            self.drive_yields();
        }
        let receipt = json!({"requestId":id,"accepted":true,"yield":self.yields[seat.index()].as_ref().map(YieldPolicy::json)});
        self.receipts[seat.index()].insert(
            id,
            Receipt {
                method: method.into(),
                args: args.clone(),
                value: receipt.clone(),
            },
        );
        self.remember(seat, "receipt", receipt.clone());
        let mut view = self.view(seat);
        view["receipt"] = receipt;
        Ok(view)
    }

    pub(crate) fn install_yield(&mut self, seat: PlayerId, id: String, options: YieldOptions) {
        let observation = &self.observations[seat.index()];
        self.yields[seat.index()] = Some(YieldPolicy {
            request_id: id,
            until: options.until,
            max_passes: options.max_passes,
            passes: 0,
            cursor: observation["updates"]
                .as_array()
                .expect("canonical updates")
                .len(),
            game: observation["match"]["game"].clone(),
            restarts: observation["match"]["restarts"].clone(),
            reason: None,
            condition: None,
            awaiting_stop: true,
        });
    }

    pub(crate) fn stop_yields(&mut self, reason: &'static str) {
        for policy in self.yields.iter_mut().flatten() {
            policy.stop(reason);
        }
    }

    // Called after every successful commit, before another seat can auto-pass.
    // Retained updates include forced StepChanged events even when the final
    // observation is beyond the requested step. Stops latch across handoffs.
    pub(crate) fn evaluate_yields(&mut self) {
        let mut observations = self.observations.clone();
        discover_requested(&self.session, &self.yields, &mut observations);
        evaluate(
            &mut self.yields,
            &observations,
            self.session.decision_seat(),
            self.session.is_finished(),
        );
    }

    pub(crate) fn drive_yields(&mut self) {
        // Each iteration commits a pass and consumes one of at most 512 permits.
        for _ in 0..MAX_PASSES * 2 {
            self.evaluate_yields();
            if self.failure.is_some() || self.session.is_finished() {
                return;
            }
            let Some(seat) = self.session.decision_seat() else {
                return;
            };
            if self.yields[seat.index()]
                .as_ref()
                .is_none_or(|policy| policy.reason.is_some())
            {
                return;
            }
            let choice = json!({"autoPass":true});
            let result = self.commit(seat, &choice, |session| {
                session
                    .pass_priority_retaining_updates(seat)
                    .map_err(Failure::Invalid)
            });
            if result.is_err() {
                if let Some(policy) = &mut self.yields[seat.index()] {
                    policy.stop("paused");
                }
                return;
            }
        }
        self.evaluate_yields();
    }
}

/// A command's cloned policies are committed only after all engine/render work succeeds.
pub(crate) fn advance(
    session: &mut penta::session::ExactSession,
    policies: &mut [Option<YieldPolicy>; 2],
) -> std::result::Result<usize, String> {
    let guarded = policies.each_ref().map(|policy| {
        policy
            .as_ref()
            .is_some_and(|policy| policy.reason.is_none() || policy.awaiting_stop)
    });
    if !guarded.into_iter().any(|guard| guard) {
        return session.advance_forced_until(|_| false);
    }
    session.advance_forced_until(|session| {
        let mut observations = [PlayerId::One, PlayerId::Two].map(|seat| session.observe(seat));
        discover_requested(session, policies, &mut observations);
        evaluate(
            policies,
            &observations,
            session.decision_seat(),
            session.is_finished(),
        );
        session
            .decision_seat()
            .is_some_and(|seat| guarded[seat.index()] && needs_input(&observations[seat.index()]))
    })
}

// This condition explicitly requests an existence search. Ordinary phase/event
// yields do not discover actions or plan payments.
fn discover_requested(
    session: &penta::session::ExactSession,
    policies: &[Option<YieldPolicy>; 2],
    observations: &mut [Value; 2],
) {
    for seat in [PlayerId::One, PlayerId::Two] {
        if ordinary_priority(&observations[seat.index()])
            && policies[seat.index()].as_ref().is_some_and(|policy| {
                policy.reason.is_none() && policy.until.iter().any(Condition::needs_discovery)
            })
        {
            let discovered = session.discover(seat);
            observations[seat.index()]["nonManaActionAvailable"] = json!(
                policies[seat.index()]
                    .as_ref()
                    .expect("active policy")
                    .until
                    .iter()
                    .any(|condition| condition.matches_menu(&discovered))
            );
        }
    }
}

fn evaluate(
    policies: &mut [Option<YieldPolicy>; 2],
    observations: &[Value; 2],
    decision_seat: Option<PlayerId>,
    finished: bool,
) {
    for seat in [PlayerId::One, PlayerId::Two] {
        let Some(policy) = &mut policies[seat.index()] else {
            continue;
        };
        if policy.reason.is_some() {
            continue;
        }
        let observation = &observations[seat.index()];
        let updates = observation["updates"]
            .as_array()
            .expect("canonical updates");
        if finished
            || observation["match"]["stage"] != "playing"
            || observation["match"]["game"] != policy.game
            || observation["match"]["restarts"] != policy.restarts
        {
            policy.stop("gameEnded");
            continue;
        }
        for event in updates.iter().skip(policy.cursor) {
            if event["type"] == "GameEnded" {
                policy.stop("gameEnded");
                break;
            }
            if let Some(index) = policy
                .until
                .iter()
                .position(|condition| condition.matches_event(seat, event))
            {
                policy.condition = Some(index);
                policy.stop("condition");
                break;
            }
        }
        policy.cursor = updates.len();
        if policy.reason.is_some() {
            continue;
        }
        if decision_seat == Some(seat) && needs_input(observation) {
            if !ordinary_priority(observation) {
                policy.stop("decision");
            } else if let Some(index) = policy
                .until
                .iter()
                .position(|condition| condition.matches_menu(observation))
            {
                policy.condition = Some(index);
                policy.stop("condition");
            }
        }
        if policy.passes >= policy.max_passes {
            policy.stop("passLimit");
        }
    }
    if let Some(seat) = decision_seat
        && let Some(policy) = &mut policies[seat.index()]
        && policy.reason.is_some()
        && needs_input(&observations[seat.index()])
    {
        policy.awaiting_stop = false;
    }
}

#[cfg(test)]
mod tests;
