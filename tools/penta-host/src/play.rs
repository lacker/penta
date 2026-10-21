use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Instant;

use penta::{PlayerId, session::ExactSession};
use serde_json::{Value, json};

use crate::{Host, HostError, Receipt, Result, keys, number, request_id, seat_name, secret};

pub(crate) enum Failure {
    Invalid(String),
    Proposal(penta::protocol::ProposalError),
    Engine(String),
}

impl Host {
    pub(crate) fn play(&mut self, seat: PlayerId, method: &str, args: &Value) -> Result<Value> {
        let allowed: &[&str] = match method {
            "choose" => &["requestId", "ticket", "options", "waitMs"],
            "play" => &["requestId", "revision", "choices", "waitMs"],
            "interact" => &["requestId", "revision", "choices", "yield", "waitMs"],
            "concede" => &["requestId", "waitMs"],
            _ => unreachable!(),
        };
        keys(args, allowed)?;
        number(args, "waitMs", 0, 25_000)?;
        let id = request_id(args)?.to_owned();
        if let Some(previous) = self.receipts[seat.index()].get(&id) {
            if previous.method != method || previous.args != *args {
                return Err(HostError::new(
                    409,
                    "requestId already used with different arguments",
                ));
            }
            let mut view = self.view(seat);
            view["receipt"] = previous.value.clone();
            return Ok(view);
        }
        let policy = if method == "interact" {
            crate::interaction::validate(args)?
        } else {
            None
        };
        if self.failure.is_some() {
            return Err(HostError::new(
                409,
                "match paused; ask the organizer to export diagnostics",
            ));
        }
        if self.session.is_finished() {
            return Err(HostError::new(409, "match is complete"));
        }
        let choices = self.choices(seat, method, args)?;
        if choices.is_empty()
            && !crate::yielding::ordinary_priority(&self.observations[seat.index()])
        {
            return Err(HostError::new(
                409,
                "yield requires this seat's ordinary priority",
            ));
        }
        let start = Instant::now();
        let (accepted, updates, stopped, rejection) = self.apply_choices(seat, method, &choices);
        if accepted == 0 && self.failure.is_none() && method != "interact" && rejection.is_none() {
            return Err(HostError::new(409, stopped.unwrap_or("no choice accepted")));
        }
        let mut receipt = json!({"requestId":id,"accepted":accepted,"updates":updates});
        if let Some(rejection) = rejection {
            receipt["rejection"] = rejection;
        }
        if method == "interact" {
            receipt["yieldInstallation"] =
                self.interaction_yield(seat, &id, policy, accepted == choices.len());
            receipt["yield"] = self.yields[seat.index()]
                .as_ref()
                .map_or(Value::Null, crate::yielding::YieldPolicy::json);
        }
        if accepted > 0 || choices.is_empty() {
            self.drive_yields();
        }
        if let Some(reason) = stopped {
            receipt["stopped"] = json!(reason);
        }
        self.receipts[seat.index()].insert(
            id,
            Receipt {
                method: method.into(),
                args: args.clone(),
                value: receipt.clone(),
            },
        );
        self.remember(seat, "receipt", receipt.clone());
        self.diagnose(
            json!({"kind":"submission","seat":seat_name(seat),"accepted":accepted,
            "elapsedMicros":start.elapsed().as_micros()}),
        );
        let mut view = self.view(seat);
        view["receipt"] = receipt;
        Ok(view)
    }

    fn apply_choices(
        &mut self,
        seat: PlayerId,
        method: &str,
        choices: &[Value],
    ) -> (usize, Vec<Value>, Option<&'static str>, Option<Value>) {
        let mut accepted = 0;
        let mut updates = Vec::new();
        let mut stopped = None;
        let mut rejection = None;
        for choice in choices {
            if method != "concede" && self.session.decision_seat() != Some(seat) {
                stopped = Some("the seat no longer holds a decision");
                break;
            }
            let applied = self.commit(seat, choice, |session| {
                if choice.as_object().is_some_and(|object| object.len() == 1)
                    && let Some(proposal) = choice.get("proposal")
                {
                    return session
                        .apply_proposal(seat, proposal)
                        .map_err(Failure::Proposal);
                }
                if method == "concede" {
                    session.concede(seat)
                } else {
                    session.apply_choice(seat, choice)
                }
                .map_err(Failure::Invalid)
            });
            match applied {
                Ok(()) => {
                    accepted += 1;
                    updates.extend(
                        self.observations[seat.index()]["updates"]
                            .as_array()
                            .expect("canonical updates")
                            .iter()
                            .cloned(),
                    );
                }
                Err(Failure::Invalid(message)) => {
                    self.diagnose(json!({"kind":"refused","seat":seat_name(seat),
                        "command":choice,"message":message}));
                    stopped = Some("choice is not legal in the current position");
                    break;
                }
                Err(Failure::Proposal(error)) => {
                    rejection =
                        Some(serde_json::to_value(error).expect("proposal error serializes"));
                    stopped = Some("proposal rejected; see rejection for the field and reason");
                    break;
                }
                Err(Failure::Engine(_)) => {
                    stopped = Some("engine failure; match paused for organizer inspection");
                    break;
                }
            }
        }
        (accepted, updates, stopped, rejection)
    }

    // Rendering observations is part of the commit: a failure there also restores
    // the last good engine and its cached views, leaving export usable.
    pub(crate) fn commit(
        &mut self,
        seat: PlayerId,
        choice: &Value,
        operation: impl FnOnce(&mut ExactSession) -> std::result::Result<(), Failure>,
    ) -> std::result::Result<(), Failure> {
        let checkpoint = self.session.clone();
        let mut policies = self.yields.clone();
        let applied = catch_unwind(AssertUnwindSafe(|| {
            operation(&mut self.session)?;
            if choice == &json!({"autoPass":true}) {
                policies[seat.index()]
                    .as_mut()
                    .expect("active yield")
                    .passes += 1;
            }
            let forced = crate::yielding::advance(&mut self.session, &mut policies)
                .map_err(Failure::Engine)?;
            let observations =
                [PlayerId::One, PlayerId::Two].map(|seat| self.session.observe(seat));
            let manual_seat = (choice != &json!({"autoPass":true})).then_some(seat);
            let events = crate::events::appended(&self.observations, &observations, manual_seat)?;
            let revision = secret().map_err(Failure::Engine)?;
            Ok((observations, revision, forced, events))
        }))
        .unwrap_or_else(|_| {
            Err(Failure::Engine(
                "engine panicked while applying this command".into(),
            ))
        });
        match applied {
            Ok((observations, revision, forced, events)) => {
                for (log, appended) in self.event_logs.iter_mut().zip(events) {
                    log.extend(appended);
                }
                self.yields = policies;
                self.observations = observations;
                self.revision = revision;
                self.journal
                    .push(json!({"seat":seat_name(seat),"choice":choice,"forced":forced}));
                self.remember(seat, "choice", choice.clone());
                self.evaluate_yields();
                self.remember_views();
                Ok(())
            }
            Err(failure) => {
                self.session = checkpoint;
                if let Failure::Engine(message) = &failure {
                    let detail = json!({"seat":seat_name(seat),"command":choice,"message":message,
                        "afterCommands":self.journal.len()});
                    self.diagnose(json!({"kind":"engineFailure","failure":detail}));
                    self.failure = Some(detail);
                    self.stop_yields("paused");
                }
                Err(failure)
            }
        }
    }

    fn choices(&self, seat: PlayerId, method: &str, args: &Value) -> Result<Vec<Value>> {
        if method == "concede" {
            return Ok(vec![json!({"concede":true})]);
        }
        // Reject before looking at the deciding seat's observation.
        if self.session.decision_seat() != Some(seat) {
            return Err(HostError::new(409, "the seat does not hold a decision"));
        }
        if method == "play" || method == "interact" {
            if args["revision"].as_str() != Some(&self.revision) {
                return Err(HostError::new(409, "stale revision; observe again"));
            }
            let choices = args["choices"]
                .as_array()
                .filter(|choices| {
                    (method == "interact" || !choices.is_empty()) && choices.len() <= 64
                })
                .ok_or_else(|| HostError::new(400, "choices must contain 1 to 64 entries"))?;
            if choices.len() > 1 && choices.iter().any(|choice| choice.get("index").is_some()) {
                return Err(HostError::new(
                    400,
                    "batches require exact actions or explicit decisions",
                ));
            }
            return Ok(choices.clone());
        }
        let (revision, index) = args["ticket"]
            .as_str()
            .and_then(|ticket| ticket.rsplit_once(":a"))
            .ok_or_else(|| HostError::new(400, "invalid ticket"))?;
        if revision != self.revision {
            return Err(HostError::new(409, "stale ticket; observe again"));
        }
        let index = index
            .parse::<usize>()
            .map_err(|_| HostError::new(400, "invalid ticket index"))?;
        let observation = self.session.discover(seat);
        let action = observation["legalActions"]
            .as_array()
            .and_then(|actions| actions.get(index))
            .ok_or_else(|| HostError::new(409, "ticket is not legal"))?;
        if action["type"] == "ChooseDecision" {
            let options = args
                .get("options")
                .filter(|value| value.is_array())
                .ok_or_else(|| HostError::new(400, "decision tickets require explicit options"))?;
            Ok(vec![
                json!({"decision":action["decision"],"options":options}),
            ])
        } else {
            if args.get("options").is_some() {
                return Err(HostError::new(
                    400,
                    "concrete action tickets do not accept options",
                ));
            }
            Ok(vec![json!({"index":index})])
        }
    }
}

#[cfg(test)]
mod tests;
