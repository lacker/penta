use super::*;

mod actions;
mod bot_game;
mod catalog;
mod compatibility;
mod observations;

fn structured_choices() -> CastChoices {
    CastChoices::new(crate::PlayOptionId(2))
        .with_modes(vec![crate::ModeId(3), crate::ModeId(1)])
        .with_costs(crate::CostConfiguration::new(
            Some(crate::AlternativeCostId(4)),
            vec![crate::AdditionalCostId(5)],
        ))
        .with_x(6)
        .with_targets(vec![
            crate::TargetSelection::single(
                crate::TargetSlotId(7),
                Target::Permanent(GameObjectId(20)),
            ),
            crate::TargetSelection::single(crate::TargetSlotId(8), Target::Spell(GameObjectId(21))),
        ])
}

fn finish(mut game: BotGame, mut pick: impl FnMut(usize, &Value) -> usize) -> GameResult {
    for turn in 0..ACTION_LIMIT {
        if let Some(result) = game.result() {
            return result;
        }
        let seat = game.decision_seat().expect("no result means a decision");
        let observation: Value =
            serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
        let count = observation["legalActions"]
            .as_array()
            .expect("legalActions is an array")
            .len();
        assert!(count > 0, "a decision always has options");
        game.act(pick(turn, &observation))
            .expect("chosen index is legal");
    }
    panic!("game did not finish");
}

/// A do-nothing bot written the way the docs tell people to write one:
/// read `legalActions`, prefer the quiet options by their `type` tags.
/// Note it never has to avoid anything: nothing in the list loses on
/// the spot.
fn pass_bot(observation: &Value) -> usize {
    let actions = observation["legalActions"].as_array().expect("array");
    for preferred in [
        "KeepHand",
        "ChooseDecision",
        "PassPriority",
        "FinishDeclaringAttackers",
        "FinishDeclaringBlockers",
        "AssignCombatDamage",
        "DiscardCards",
        "BottomCards",
        "ChooseUntap",
    ] {
        if let Some(action) = actions.iter().find(|action| action["type"] == preferred) {
            return usize::try_from(action["index"].as_u64().expect("index")).expect("index fits");
        }
    }
    0
}

fn assert_no_physical_lineage_keys(value: &Value) {
    fn visit(value: &Value, path: &str) {
        match value {
            Value::Object(fields) => {
                for (key, child) in fields {
                    let normalized = key.to_ascii_lowercase();
                    assert!(
                        !normalized.contains("physical") && !normalized.contains("backing"),
                        "protocol exposed physical-card lineage at {path}.{key}"
                    );
                    visit(child, &format!("{path}.{key}"));
                }
            }
            Value::Array(items) => {
                for (index, child) in items.iter().enumerate() {
                    visit(child, &format!("{path}[{index}]"));
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }

    visit(value, "$observed");
}
