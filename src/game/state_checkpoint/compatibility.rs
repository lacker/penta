/// Parses the current snapshot while retaining protocol-28 fields whose
/// engine representation has moved elsewhere.
fn parse_compatible_game_snapshot(checkpoint_value: &Value) -> Result<GameSnapshot, String> {
    let checkpoint: GameSnapshot = serde_json::from_value(checkpoint_value.clone())
        .map_err(|error| format!("invalid game snapshot: {error}"))?;
    if checkpoint.channel_active != [false; 2] {
        return Err(
            "checkpoint legacy channelActive state must be represented by ongoingEffects".into(),
        );
    }
    Ok(checkpoint)
}
