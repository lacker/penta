use serde_json::{Value, json};

use super::json_common::{
    ability_origin_json, cast_choices_json, defender_json, instances_json, target_json,
    target_selections_json,
};
use crate::{AbilityOrigin, Action, GameObjectId, PlayerObservation, TargetSelection};

fn activated_ability_json(
    source: GameObjectId,
    ability: AbilityOrigin,
    targets: &[TargetSelection],
    cost_object: Option<GameObjectId>,
    x: u16,
) -> Value {
    json!({
        "type": "ActivateAbility",
        "x": x,
        "source": source.0,
        "ability": ability_origin_json(ability),
        "target": targets
            .iter()
            .flat_map(TargetSelection::targets)
            .next()
            .copied()
            .map(target_json),
        "targets": targets
            .iter()
            .flat_map(TargetSelection::targets)
            .copied()
            .map(target_json)
            .collect::<Vec<_>>(),
        "targetSelections": target_selections_json(targets),
        "costObject": cost_object.map(|card| card.0),
    })
}

/// Serializes one legal action. The `type` tag names the engine's action
/// variant; the remaining fields identify what it operates on.
#[must_use]
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
        } => json!({
            "type": "ActivateManaAbility",
            "source": source.0,
            "ability": ability_origin_json(*ability),
            "color": super::json_common::mana_color_name(*color),
        }),
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
            cost_object,
            x,
        } => activated_ability_json(*source, *ability, targets, *cost_object, *x),
        Action::TakeSpecialAction {
            source,
            ability,
            effect_id,
        } => json!({
            "type": "TakeSpecialAction",
            "source": source.0,
            "ability": ability_origin_json(*ability),
            "effectId": effect_id,
        }),
        Action::DeclareAttacker { attacker, defender } => {
            json!({ "type": "DeclareAttacker", "attacker": attacker.0, "defender": defender_json(*defender) })
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
