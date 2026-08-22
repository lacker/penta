/// Parses the current snapshot while retaining protocol-28 fields whose
/// engine representation has moved elsewhere.
fn parse_compatible_game_snapshot(checkpoint_value: &Value) -> Result<GameSnapshot, String> {
    let version = u32_field(checkpoint_value, "version")
        .map_err(|error| format!("invalid game snapshot: {error}"))?;
    if version != crate::protocol::CHECKPOINT_VERSION {
        return Err(format!(
            "checkpoint version {version} does not match {}",
            crate::protocol::CHECKPOINT_VERSION
        ));
    }
    let fingerprint = str_field(checkpoint_value, "simulationFingerprint")
        .map_err(|error| format!("invalid game snapshot: {error}"))?;
    if fingerprint != crate::protocol::SIMULATION_FINGERPRINT {
        return Err(format!(
            "checkpoint simulation fingerprint {fingerprint:?} does not match {}",
            crate::protocol::SIMULATION_FINGERPRINT
        ));
    }
    let checkpoint: GameSnapshot = serde_json::from_value(checkpoint_value.clone())
        .map_err(|error| format!("invalid game snapshot: {error}"))?;
    if checkpoint.channel_active != [false; 2] {
        return Err(
            "checkpoint legacy channelActive state must be represented by ongoingEffects".into(),
        );
    }
    if checkpoint.has_deferred_state {
        return Err(
            "checkpoint contains executable rules state without stable catalog semantics".into(),
        );
    }
    Ok(checkpoint)
}
