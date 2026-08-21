use serde_json::{Value, json};

use super::json_common::{
    ability_origin_json, cast_choices_json, defender_json, instances_json, target_json,
    target_selections_json,
};
use crate::{Action, PlayerObservation};

/// Serializes one legal action. The `type` tag names the engine's action
/// variant; the remaining fields identify what it operates on.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn action_json(action: &Action) -> Value {
    match action {
        Action::KeepHand => json!({ "type": "KeepHand" }),
        Action::TakeMulligan => json!({ "type": "TakeMulligan" }),
        Action::BottomCards { cards } => {
            json!({ "type": "BottomCards", "cards": instances_json(cards) })
        }
        Action::DiscardCards { cards } => {
            json!({ "type": "DiscardCards", "cards": instances_json(cards) })
        }
        Action::ChooseDecision { decision, options } => {
            json!({ "type": "ChooseDecision", "decision": decision, "options": options })
        }
        Action::CancelDecision { decision } => {
            json!({ "type": "CancelDecision", "decision": decision })
        }
        Action::Foretell { card } => {
            json!({ "type": "Foretell", "card": card.0 })
        }
        Action::UnlockDoor { room, door } => {
            json!({ "type": "UnlockDoor", "room": room.0, "door": door.0 })
        }
        Action::TurnFaceUp { permanent } => {
            json!({ "type": "TurnFaceUp", "permanent": permanent.0 })
        }
        Action::ChooseUntap { permanents } => {
            json!({ "type": "ChooseUntap", "permanents": instances_json(permanents) })
        }
        Action::PassPriority => json!({ "type": "PassPriority" }),
        Action::PlayLand { card, option } => json!({
            "type": "PlayLand",
            "card": card.0,
            "playOptionId": option.0,
        }),
        Action::ActivateManaAbility {
            source,
            ability,
            color,
            counters_removed,
            cost_object,
            combination,
        } => {
            let mut action = json!({
                "type": "ActivateManaAbility",
                "source": source.0,
                "ability": ability_origin_json(*ability),
                "color": super::json_common::mana_color_name(*color),
            });
            // Optional, and present only for the abilities that offer more
            // than one size: every other mana ability's wire shape is
            // unchanged.
            if let Some(removed) = counters_removed {
                action["countersRemoved"] = json!(removed);
            }
            // Likewise optional, and present only for a cost that sacrifices
            // some other permanent.
            if let Some(sacrificed) = cost_object {
                action["costObject"] = json!(sacrificed.0);
            }
            // Likewise optional, and present only for an ability that adds
            // mana "in any combination of" more than one type. Types the
            // division produces none of are left out, and `color` names its
            // first entry, so a bot that reads only `color` still sees a
            // colour it will receive.
            if let Some(division) = combination {
                action["combination"] = Value::Object(
                    division
                        .iter()
                        .map(|(color, amount)| {
                            (
                                super::json_common::mana_color_name(color).to_owned(),
                                json!(amount),
                            )
                        })
                        .collect(),
                );
            }
            action
        }
        Action::PayLifeForMana => json!({ "type": "PayLifeForMana" }),
        Action::CastSpell {
            card,
            choices,
            sacrifices,
        } => json!({
            "type": "CastSpell",
            "card": card.0,
            "choices": cast_choices_json(choices),
            "playOptionId": choices.play_option().0,
            "modeIds": choices.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
            "targets": choices.iter_targets().copied().map(target_json).collect::<Vec<_>>(),
            "sacrifices": instances_json(sacrifices),
            "x": choices.x(),
        }),
        Action::ActivateAbility {
            source,
            ability,
            targets,
            cost_objects,
            x,
            modes,
        } => json!({
            "type": "ActivateAbility",
            "x": x,
            // Present for every activation, empty for the abilities that
            // print no modes -- which is nearly all of them. A cast already
            // reports its modes the same way.
            "modeIds": modes.iter().map(|mode| mode.0).collect::<Vec<_>>(),
            "source": source.0,
            "ability": ability_origin_json(*ability),
            "target": targets
                .iter()
                .flat_map(crate::TargetSelection::targets)
                .next()
                .copied()
                .map(target_json),
            "targets": targets
                .iter()
                .flat_map(crate::TargetSelection::targets)
                .copied()
                .map(target_json)
                .collect::<Vec<_>>(),
            "targetSelections": target_selections_json(targets),
            "costObjects": cost_objects.iter().map(|card| card.0).collect::<Vec<_>>(),
        }),
        Action::DeclareAttacker { attacker, defender } => {
            json!({ "type": "DeclareAttacker", "attacker": attacker.0, "defender": defender_json(*defender) })
        }
        Action::BandAttackers { first, second } => {
            json!({ "type": "BandAttackers", "first": first.0, "second": second.0 })
        }
        Action::FinishDeclaringAttackers => json!({ "type": "FinishDeclaringAttackers" }),
        Action::DeclareBlocker { blocker, attacker } => {
            json!({ "type": "DeclareBlocker", "blocker": blocker.0, "attacker": attacker.0 })
        }
        Action::FinishDeclaringBlockers => json!({ "type": "FinishDeclaringBlockers" }),
        Action::AssignCombatDamage {
            attacker,
            assignments,
        } => json!({
            "type": "AssignCombatDamage",
            "attacker": attacker.0,
            "assignments": assignments.iter().map(|assignment| json!({
                "recipient": target_json(assignment.recipient),
                "amount": assignment.amount,
            })).collect::<Vec<_>>(),
        }),
        Action::Concede => json!({ "type": "Concede" }),
    }
}

/// Translates the engine's action list into the one bots see.
///
/// Two differences from [`PlayerObservation::legal_actions`]:
///
/// Conceding is dropped. It is legal in every state, and for a bot it is
/// strictly dominated — resigning can only lose a game that playing on might
/// win — so both built-in policies already refuse it and no rational bot
/// would pick it. Leaving it in made uniform-random exploration resign on
/// turn one, which is a poor action space for the audience this protocol is
/// for. Humans still concede through the browser, which reads the engine's
/// list directly.
///
/// Pending decisions are expanded. The engine lists one template action with
/// empty `options`, expecting the caller to fill in ids from the decision
/// schema. Bots act by index, so a pick-exactly-one decision becomes one
/// concrete action per option, and a multi-pick keeps a default choice of the
/// first `minimum` options so an index-only bot always has a legal move. Bots
/// that want a different multi-pick send it through
/// [`super::BotGame::choose_decision`].
#[must_use]
pub fn protocol_actions(observation: &PlayerObservation) -> Vec<Action> {
    let mut actions = Vec::with_capacity(observation.legal_actions.len());
    for action in &observation.legal_actions {
        if matches!(action, Action::Concede) {
            continue;
        }
        match (action, observation.decision.as_ref()) {
            (Action::ChooseDecision { decision, options }, Some(pending))
                if options.is_empty() && *decision == pending.id =>
            {
                if pending.minimum == 1 && pending.maximum == 1 {
                    for option in &pending.options {
                        actions.push(Action::ChooseDecision {
                            decision: *decision,
                            options: vec![option.id],
                        });
                    }
                } else if pending.minimum == 0 && pending.maximum == 1 {
                    // A draw-time opportunity is genuinely optional: the empty
                    // selection declines it, while each singleton accepts one
                    // available action. Keep every branch index-addressable so
                    // a bot need not use the separate decision endpoint merely
                    // to reveal a Miracle card.
                    actions.push(Action::ChooseDecision {
                        decision: *decision,
                        options: Vec::new(),
                    });
                    for option in &pending.options {
                        actions.push(Action::ChooseDecision {
                            decision: *decision,
                            options: vec![option.id],
                        });
                    }
                } else {
                    actions.push(Action::ChooseDecision {
                        decision: *decision,
                        // The neutral default: the first `minimum` options,
                        // which for a may-choose decision means declining.
                        options: pending
                            .options
                            .iter()
                            .take(pending.minimum)
                            .map(|option| option.id)
                            .collect(),
                    });
                }
            }
            _ => actions.push(action.clone()),
        }
    }
    actions
}
