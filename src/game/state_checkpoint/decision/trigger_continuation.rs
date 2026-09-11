/// The four trigger-ordering continuations, which share the same handful of
/// trigger parsers and are grouped here to keep the main continuation match
/// readable.
fn parse_trigger_continuation(
    value: &DecisionContinuationSnapshot,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    Ok(match value {
        DecisionContinuationSnapshot::TriggerOrder { batch, remaining } => {
            DecisionContinuation::TriggerOrder {
                batch: parse_trigger_batch(batch, game)?,
                remaining: remaining
                    .iter()
                    .map(|batch| parse_trigger_batch(batch, game))
                    .collect::<Result<Vec<_>, _>>()?,
            }
        }
        DecisionContinuationSnapshot::TriggerPlacement {
            trigger,
            pending,
            remaining,
            candidates,
        } => DecisionContinuation::TriggerPlacement {
            trigger: parse_pending_trigger(trigger, game)?,
            pending: pending
                .iter()
                .map(|trigger| parse_pending_trigger(trigger, game))
                .collect::<Result<Vec<_>, _>>()?,
            remaining: remaining
                .iter()
                .map(|batch| parse_trigger_batch(batch, game))
                .collect::<Result<Vec<_>, _>>()?,
            candidates: candidates.iter().copied().map(parse_target).collect(),
        },
        DecisionContinuationSnapshot::TriggerMode {
            trigger,
            pending,
            remaining,
        } => {
            let trigger = parse_pending_trigger(trigger, game)?;
            let modes = trigger
                .modes
                .ok_or("trigger-mode decision names a trigger that prints no modes")?;
            DecisionContinuation::TriggerMode {
                trigger,
                pending: pending
                    .iter()
                    .map(|trigger| parse_pending_trigger(trigger, game))
                    .collect::<Result<Vec<_>, _>>()?,
                remaining: remaining
                    .iter()
                    .map(|batch| parse_trigger_batch(batch, game))
                    .collect::<Result<Vec<_>, _>>()?,
                modes: Box::new(modes),
            }
        }
        DecisionContinuationSnapshot::TriggerDivision {
            trigger,
            pending,
            remaining,
            targets,
            divisions,
        } => DecisionContinuation::TriggerDivision {
            trigger: parse_pending_trigger(trigger, game)?,
            pending: pending
                .iter()
                .map(|trigger| parse_pending_trigger(trigger, game))
                .collect::<Result<Vec<_>, _>>()?,
            remaining: remaining
                .iter()
                .map(|batch| parse_trigger_batch(batch, game))
                .collect::<Result<Vec<_>, _>>()?,
            targets: targets.iter().copied().map(parse_target).collect(),
            divisions: divisions.clone(),
        },
        _ => return Err("not a trigger continuation".into()),
    })
}
