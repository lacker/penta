//! Additive action-then-yield orchestration over the ordinary commit path.

use penta::PlayerId;
use serde_json::{Value, json};

use crate::yielding::{YieldOptions, ordinary_priority};
use crate::{Host, HostError, Result};

pub(crate) fn validate(args: &Value) -> Result<Option<YieldOptions>> {
    let policy = args.get("yield").map(YieldOptions::parse).transpose()?;
    let choices = args["choices"]
        .as_array()
        .filter(|choices| choices.len() <= 64 && (!choices.is_empty() || policy.is_some()))
        .ok_or_else(|| {
            HostError::new(400, "choices requires 0..64 entries; empty requires yield")
        })?;
    for choice in choices {
        let valid = choice.as_object().is_some_and(|object| {
            (object.len() == 1 && object.contains_key("proposal"))
                || (object.len() == 1
                    && object.get("action").is_some_and(|action| {
                        action.as_object().is_some_and(|action| {
                            action.get("type").is_some_and(Value::is_string)
                                && !action.contains_key("index")
                        })
                    }))
                || (object.len() == 2
                    && object.get("decision").is_some_and(Value::is_u64)
                    && object
                        .get("options")
                        .and_then(Value::as_array)
                        .is_some_and(|options| {
                            options.len() <= 4096 && options.iter().all(Value::is_u64)
                        }))
        });
        if !valid {
            return Err(HostError::new(
                400,
                "interact requires exact actions or explicit decisions",
            ));
        }
    }
    Ok(policy)
}

impl Host {
    pub(crate) fn interaction_yield(
        &mut self,
        seat: PlayerId,
        id: &str,
        policy: Option<YieldOptions>,
        full_batch: bool,
    ) -> Value {
        let reason = if policy.is_none() {
            Some("notRequested")
        } else if self.failure.is_some() {
            Some("paused")
        } else if self.session.is_finished() {
            Some("complete")
        } else if !full_batch {
            Some("partialBatch")
        } else if self.session.decision_seat() == Some(seat)
            && !ordinary_priority(&self.observations[seat.index()])
        {
            Some("decision")
        } else {
            None
        };
        if let Some(reason) = reason {
            return json!({"status":"notInstalled","reason":reason});
        }
        self.install_yield(seat, id.to_owned(), policy.expect("validated policy"));
        json!({"status":"installed","reason":null})
    }
}

#[cfg(test)]
mod tests;
