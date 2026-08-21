use super::{
    AbilityOrigin, Action, CardInstanceId, PlayerId, PlayerObservation, Target, Value, json,
};

/// The per-slot target choices of a cast action, hoisted beside the flattened
/// target metadata so the browser can tell which slot each target belongs to
/// without unpacking the whole signature.
pub(super) fn action_target_selections(action: &Action, human: PlayerId) -> Value {
    match action {
        Action::CastSpell { choices, .. } => target_selections_value(choices.targets(), human),
        _ => Value::Array(Vec::new()),
    }
}

fn target_selections_value(selections: &[penta::TargetSelection], human: PlayerId) -> Value {
    Value::Array(
        selections
            .iter()
            .map(|selection| {
                json!({
                    "slotId": selection.slot().0,
                    "targetCardIds": selection.targets().iter().filter_map(|target| match target {
                        Target::Card(id) | Target::Permanent(id) => Some(id.0),
                        Target::Player(_) | Target::Spell(_) => None,
                    }).collect::<Vec<_>>(),
                    "targetPlayers": selection.targets().iter().filter_map(|target| match target {
                        Target::Player(player) => Some(if *player == human {
                            "human"
                        } else {
                            "opponent"
                        }),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    }).collect::<Vec<_>>(),
                })
            })
            .collect(),
    )
}

pub(super) fn cast_signature_value(signature: &penta::CastSignature, human: PlayerId) -> Value {
    let form = match signature.form() {
        penta::SpellForm::Part(part) => json!({
            "kind": "part",
            "partId": part.0,
        }),
        penta::SpellForm::Combined(parts) => json!({
            "kind": "combined",
            "partIds": parts.iter().map(|part| part.0).collect::<Vec<_>>(),
        }),
    };
    let target_selections = signature
        .targets()
        .iter()
        .map(|selection| {
            json!({
                "slotId": selection.slot().0,
                "targetCardIds": selection.targets().iter().filter_map(|target| match target {
                    Target::Card(id) | Target::Permanent(id) => Some(id.0),
                    Target::Player(_) | Target::Spell(_) => None,
                }).collect::<Vec<_>>(),
                "targetPlayers": selection.targets().iter().filter_map(|target| match target {
                    Target::Player(player) => Some(if *player == human {
                        "human"
                    } else {
                        "opponent"
                    }),
                    Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                }).collect::<Vec<_>>(),
                "targetStackIds": selection.targets().iter().filter_map(|target| match target {
                    Target::Spell(id) => Some(id.0),
                    Target::Player(_) | Target::Card(_) | Target::Permanent(_) => None,
                }).collect::<Vec<_>>(),
                // Present only for a slot the card divides; each entry is the
                // share of the target at the same position.
                "amounts": selection.amounts(),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "playOptionId": signature.play_option().0,
        "form": form,
        "modeIds": signature.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
        "alternativeCostId": signature.costs().alternative().map(|cost| cost.0),
        "additionalCostIds": signature
            .costs()
            .additional()
            .iter()
            .map(|cost| cost.0)
            .collect::<Vec<_>>(),
        "x": signature.x(),
        "targetSelections": target_selections,
    })
}

pub(super) fn action_kind(action: &Action) -> &'static str {
    match action {
        Action::Concede => "danger",
        Action::PassPriority
        | Action::FinishDeclaringAttackers
        | Action::FinishDeclaringBlockers => "pass",
        Action::DeclareAttacker { .. }
        | Action::BandAttackers { .. }
        | Action::DeclareBlocker { .. }
        | Action::AssignCombatDamage { .. } => "combat",
        _ => "primary",
    }
}

pub(super) fn action_card(action: &Action) -> Option<CardInstanceId> {
    match action {
        Action::PlayLand { card, .. } | Action::CastSpell { card, .. } => Some(*card),
        Action::ActivateManaAbility { source, .. } | Action::ActivateAbility { source, .. } => {
            Some(*source)
        }
        Action::DeclareAttacker { attacker, .. } | Action::AssignCombatDamage { attacker, .. } => {
            Some(*attacker)
        }
        Action::DeclareBlocker { blocker, .. } => Some(*blocker),
        // The band is a pair; anchoring on the first member is what the rest
        // of combat does with an action that names more than one object.
        Action::BandAttackers { first, .. } => Some(*first),
        _ => None,
    }
}

pub(super) fn action_ability_origin(action: &Action) -> Option<Value> {
    let origin = match action {
        Action::ActivateManaAbility { ability, .. } | Action::ActivateAbility { ability, .. } => {
            *ability
        }
        _ => return None,
    };
    Some(ability_origin_value(origin))
}

pub(super) fn source_has_multiple_activated_abilities(
    observation: &PlayerObservation,
    source: CardInstanceId,
) -> bool {
    let mut first = None;
    for action in &observation.legal_actions {
        let Action::ActivateAbility {
            source: candidate,
            ability,
            ..
        } = action
        else {
            continue;
        };
        if *candidate != source {
            continue;
        }
        match first {
            None => first = Some(*ability),
            Some(first) if first != *ability => return true,
            Some(_) => {}
        }
    }
    false
}

pub(super) fn source_ability_has_multiple_x_values(
    observation: &PlayerObservation,
    source: CardInstanceId,
    ability: AbilityOrigin,
) -> bool {
    let mut first = None;
    for action in &observation.legal_actions {
        let Action::ActivateAbility {
            source: candidate,
            ability: candidate_ability,
            x,
            ..
        } = action
        else {
            continue;
        };
        if *candidate != source || *candidate_ability != ability {
            continue;
        }
        match first {
            None => first = Some(*x),
            Some(first) if first != *x => return true,
            Some(_) => {}
        }
    }
    false
}

pub(super) fn ability_origin_value(origin: AbilityOrigin) -> Value {
    match origin {
        AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } => json!({
            "kind": "printed",
            "definition": definition.0,
            "partId": part.0,
            "abilityId": ability.0,
        }),
        AbilityOrigin::Token { part, ability } => json!({
            "kind": "token",
            "partId": part.0,
            "abilityId": ability.0,
        }),
        AbilityOrigin::Emblem { ability } => json!({
            "kind": "emblem",
            "abilityId": ability.0,
        }),
        AbilityOrigin::IntrinsicBasicLand(land_type) => json!({
            "kind": "intrinsicBasicLand",
            "landType": match land_type {
                penta::BasicLandType::Plains => "plains",
                penta::BasicLandType::Island => "island",
                penta::BasicLandType::Swamp => "swamp",
                penta::BasicLandType::Mountain => "mountain",
                penta::BasicLandType::Forest => "forest",
            },
        }),
        AbilityOrigin::IntrinsicCounter(kind) => json!({
            "kind": "intrinsicCounter",
            "counter": kind.name(),
        }),
        AbilityOrigin::Granted {
            source,
            source_definition,
            source_part,
            source_ability,
            grant,
        } => json!({
            "kind": "granted",
            "source": source.0,
            "sourceDefinition": source_definition.0,
            "sourcePartId": source_part.0,
            "sourceAbilityId": source_ability.0,
            "grantId": grant.0,
        }),
        AbilityOrigin::TokenGranted {
            source,
            source_part,
            source_ability,
            grant,
        } => json!({
            "kind": "tokenGranted",
            "source": source.0,
            "sourcePartId": source_part.0,
            "sourceAbilityId": source_ability.0,
            "grantId": grant.0,
        }),
        AbilityOrigin::EmblemGranted {
            source,
            source_ability,
            grant,
        } => json!({
            "kind": "emblemGranted",
            "source": source.0,
            "sourceAbilityId": source_ability.0,
            "grantId": grant.0,
        }),
    }
}

/// Who a declaration is attacking. The browser draws the arrow from this
/// rather than from target metadata, because a defender is not a target.
pub(super) fn action_attack_defender(action: &Action) -> Option<penta::AttackDefender> {
    match action {
        Action::DeclareAttacker { defender, .. } => Some(*defender),
        _ => None,
    }
}

pub(super) fn attack_defender_value(defender: penta::AttackDefender, human: PlayerId) -> Value {
    match defender {
        penta::AttackDefender::Player(player) => json!({
            "kind": "player",
            "player": if player == human { "human" } else { "opponent" },
        }),
        penta::AttackDefender::Planeswalker(card) => {
            json!({ "kind": "planeswalker", "cardId": card.0 })
        }
    }
}

/// Permanents this action would destroy as part of its cost. The browser makes
/// the player pick these explicitly rather than spending whatever is to hand.
pub(super) fn action_sacrifices(action: &Action) -> Vec<u32> {
    match action {
        Action::CastSpell { sacrifices, .. } => sacrifices.iter().map(|id| id.0).collect(),
        Action::ActivateAbility {
            source,
            cost_objects,
            ..
        } => cost_objects
            .iter()
            .filter(|spent| *spent != source)
            .map(|spent| spent.0)
            .collect(),
        _ => Vec::new(),
    }
}

pub(super) fn action_target_card(action: &Action) -> Option<CardInstanceId> {
    if let Action::DeclareBlocker { attacker, .. } = action {
        return Some(*attacker);
    }
    action_targets(action)
        .iter()
        .find_map(|target| match target {
            Target::Card(id) | Target::Permanent(id) => Some(*id),
            Target::Player(_) | Target::Spell(_) => None,
        })
}

pub(super) fn action_target_player(action: &Action, human: PlayerId) -> Option<&'static str> {
    action_targets(action)
        .iter()
        .find_map(|target| match target {
            Target::Player(player) => Some(if *player == human {
                "human"
            } else {
                "opponent"
            }),
            Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
        })
}

pub(super) fn action_target_stack(action: &Action) -> Option<u32> {
    action_targets(action)
        .iter()
        .find_map(|target| match target {
            Target::Spell(id) => Some(id.0),
            Target::Player(_) | Target::Card(_) | Target::Permanent(_) => None,
        })
}

pub(super) fn action_targets(action: &Action) -> Vec<Target> {
    match action {
        Action::CastSpell { choices, .. } => choices.iter_targets().copied().collect(),
        Action::ActivateAbility { targets, .. } => targets
            .iter()
            .flat_map(penta::TargetSelection::targets)
            .copied()
            .collect(),
        _ => Vec::new(),
    }
}

pub(super) fn action_target_cards(action: &Action) -> Vec<u32> {
    action_targets(action)
        .iter()
        .filter_map(|target| match target {
            Target::Card(id) | Target::Permanent(id) => Some(id.0),
            Target::Player(_) | Target::Spell(_) => None,
        })
        .collect()
}

pub(super) fn action_target_players(action: &Action, human: PlayerId) -> Vec<&'static str> {
    action_targets(action)
        .iter()
        .filter_map(|target| match target {
            Target::Player(player) => Some(if *player == human {
                "human"
            } else {
                "opponent"
            }),
            Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
        })
        .collect()
}

pub(super) fn action_target_stacks(action: &Action) -> Vec<u32> {
    action_targets(action)
        .iter()
        .filter_map(|target| match target {
            Target::Spell(id) => Some(id.0),
            Target::Player(_) | Target::Card(_) | Target::Permanent(_) => None,
        })
        .collect()
}

pub(super) fn should_animate_action(action: &Action) -> bool {
    !matches!(
        action,
        Action::KeepHand
            | Action::TakeMulligan
            | Action::BottomCards { .. }
            | Action::Concede
            | Action::PassPriority
            | Action::ActivateManaAbility { .. }
            | Action::FinishDeclaringAttackers
            | Action::FinishDeclaringBlockers
    )
}

pub(super) fn animated_action_kind(action: &Action) -> &'static str {
    match action {
        Action::PlayLand { .. } => "land",
        Action::CastSpell { .. } => "spell",
        // Turning a permanent face up changes what a permanent is, which
        // reads as an ability being used rather than a bare choice.
        Action::ActivateAbility { .. }
        | Action::TurnFaceUp { .. }
        | Action::Foretell { .. }
        | Action::UnlockDoor { .. } => "ability",
        Action::DeclareAttacker { .. }
        | Action::BandAttackers { .. }
        | Action::DeclareBlocker { .. }
        | Action::AssignCombatDamage { .. } => "combat",
        Action::KeepHand
        | Action::TakeMulligan
        | Action::BottomCards { .. }
        | Action::DiscardCards { .. }
        | Action::ChooseDecision { .. }
        | Action::CancelDecision { .. }
        | Action::ChooseUntap { .. } => "choice",
        Action::Concede
        | Action::PassPriority
        | Action::ActivateManaAbility { .. }
        | Action::PayLifeForMana
        | Action::FinishDeclaringAttackers
        | Action::FinishDeclaringBlockers => "quiet",
    }
}
