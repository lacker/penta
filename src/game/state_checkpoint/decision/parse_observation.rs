// Reading a pending decision back off the wire.
//
// Split out of `decision.rs` for the source-size budget: the file next door
// writes a decision and its continuation down, and this reads the visible
// half back. Included textually, so the imports here are that module's.

pub(super) fn parse_pending_decision(
    observation: &Value,
    state: Option<&DecisionStateSnapshot>,
    hidden: &Value,
    game: &Game,
) -> Result<Option<PendingDecision>, String> {
    if game.between_games() { return Ok(None); }
    let Some(visible) = observation.get("decision").filter(|value| !value.is_null()) else {
        if state.is_some() {
            return Err("checkpoint decision is not visible to its viewer".into());
        }
        return Ok(None);
    };
    let state = state
        .ok_or("invalid game snapshot: decisionState is absent for the visible pending decision")?;
    let observation =
        parse_decision_observation(visible, &state.preference, &state.options, &game.catalog)?;
    let continuation = parse_continuation(&state.continuation, &observation, hidden, game)?;
    Ok(Some(PendingDecision {
        observation,
        continuation,
    }))
}

fn parse_decision_observation(
    value: &Value,
    preference: &DecisionPreferenceSnapshot,
    option_snapshots: &[DecisionOptionSnapshot],
    catalog: &CardCatalog,
) -> Result<DecisionObservation, String> {
    let options = array(field(value, "options")?)?;
    if options.len() != option_snapshots.len() {
        return Err("checkpoint decision options do not match observation".into());
    }
    Ok(DecisionObservation {
        id: u32_field(value, "id")?,
        player: seat_value(field(value, "seat")?)?,
        kind: match str_field(value, "kind")? {
            "Choice" => DecisionKind::Choice,
            "TriggerOrder" => DecisionKind::TriggerOrder,
            "TriggerPlacement" => DecisionKind::TriggerPlacement,
            other => return Err(format!("unknown decision kind {other}")),
        },
        order_semantics: value
            .get("orderSemantics")
            .filter(|value| !value.is_null())
            .map(|value| match value.as_str() {
                Some("resolution") => Ok(DecisionOrderSemantics::Resolution),
                _ => Err("unknown decision order semantics".to_owned()),
            })
            .transpose()?,
        source: value
            .get("sourceObjectId")
            .filter(|source| !source.is_null())
            .map(|_| u32_field(value, "sourceObjectId").map(GameObjectId))
            .transpose()?,
        prompt: str_field(value, "prompt")?.to_owned(),
        visibility: match str_field(value, "visibility")? {
            "Public" => DecisionVisibility::Public,
            "PublicNotice" => DecisionVisibility::PublicNotice,
            "Private" => DecisionVisibility::Private,
            other => return Err(format!("unknown decision visibility {other}")),
        },
        preference: parse_preference(preference)?,
        minimum: usize_field(value, "minimum")?,
        maximum: usize_field(value, "maximum")?,
        cancellable: bool_field(value, "cancellable")?,
        options: options
            .iter()
            .zip(option_snapshots)
            .map(|(value, snapshot)| parse_option(value, snapshot, catalog))
            .collect::<Result<Vec<_>, _>>()?,
    })
}
