use penta::{PlayerId, protocol};
use serde_json::{Value, json};

use crate::Config;

/// Reconstructs the committed prefix of an organizer export and checks its views.
/// The failing command, if any, is reported separately and never auto-retried.
///
/// # Errors
/// Rejects incompatible provenance, malformed commands, and observation divergence.
pub fn replay_bundle(bundle: &Value) -> Result<Value, String> {
    let version = bundle["nativeReplayVersion"].as_u64().unwrap_or(0);
    if !(1..=3).contains(&version)
        || bundle["simulationFingerprint"].as_str() != Some(protocol::SIMULATION_FINGERPRINT)
    {
        return Err("incompatible native replay version or simulation fingerprint".into());
    }
    let config: Config =
        serde_json::from_value(bundle["config"].clone()).map_err(|error| error.to_string())?;
    let seed = bundle["seed"]
        .as_str()
        .ok_or("missing private seed")?
        .parse()
        .map_err(|_| "invalid private seed")?;
    let mut session = config.session_version(seed, version >= 3)?;
    let commands = bundle["commands"].as_array().ok_or("missing commands")?;
    for (index, command) in commands.iter().enumerate() {
        let seat = match command["seat"].as_str() {
            Some("p1") => PlayerId::One,
            Some("p2") => PlayerId::Two,
            _ => return Err(format!("invalid seat at command {index}")),
        };
        if version >= 2 && command["choice"] == json!({"autoPass":true}) {
            session.pass_priority_retaining_updates(seat)?;
        } else if command["choice"] == json!({"concede":true}) {
            session.concede(seat)?;
        } else {
            session.apply_choice(seat, &command["choice"])?;
        }
        if version >= 2 {
            let count = command["forced"]
                .as_u64()
                .and_then(|count| usize::try_from(count).ok())
                .ok_or("missing forced prefix length")?;
            session.advance_forced_exact(count)?;
        } else {
            session.advance_forced()?;
        }
    }
    let observations =
        json!({"p1":session.observe(PlayerId::One),"p2":session.observe(PlayerId::Two)});
    if bundle["observations"] != observations {
        return Err("replayed observations diverge from the exported match".into());
    }
    Ok(
        json!({"status":if session.is_finished() {"complete"} else {"active"},
        "commands":commands.len(),"observations":observations,"failure":bundle["failure"]}),
    )
}
