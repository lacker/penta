fn parse_pregame_continuation(
    value: &DecisionContinuationSnapshot,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    Ok(match value {
        DecisionContinuationSnapshot::PregameActions { player: seat, actions } => {
            let player = player(*seat)?;
            let parsed = actions
                .iter()
                .map(|action| {
                    Ok(super::super::PregameAbilityAction {
                        source: GameObjectId(action.source),
                        ability: ability_origin_from_snapshot(action.ability),
                        cost_objects: action
                            .cost_objects
                            .iter()
                            .copied()
                            .map(GameObjectId)
                            .collect(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let available = game.pregame_ability_actions(
                player,
                crate::card::PregameTimingDef::OpeningHand,
            );
            if parsed.iter().any(|action| !available.contains(action)) {
                return Err("checkpoint pregame action is not available from the hand".into());
            }
            if parsed.iter().enumerate().any(|(index, action)| {
                parsed[index.saturating_add(1)..].contains(action)
            }) {
                return Err("checkpoint repeats a pregame action".into());
            }
            DecisionContinuation::PregameActions {
                player,
                actions: parsed,
            }
        }
        DecisionContinuationSnapshot::ScryBottom { player: seat, revealed } => {
            DecisionContinuation::ScryBottom {
                player: player(*seat)?,
                revealed: parse_detached_cards(revealed, game)?,
            }
        }
        DecisionContinuationSnapshot::ScryTop {
            player: seat,
            top,
            bottom,
        } => {
            let top = parse_detached_cards(top, game)?;
            let bottom = parse_detached_cards(bottom, game)?;
            if top
                .iter()
                .any(|card| bottom.iter().any(|other| other.id == card.id))
            {
                return Err("checkpoint repeats a card across the scry piles".into());
            }
            DecisionContinuation::ScryTop {
                player: player(*seat)?,
                top,
                bottom,
            }
        }
        _ => unreachable!("only pregame continuations reach this parser"),
    })
}
