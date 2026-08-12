use serde_json::{Value, json};

use crate::{CardDefinitionId, GameObjectId, PlayerId, Target};

use super::super::{
    DecisionContinuation, DecisionKind, DecisionObservation, DecisionOption,
    DecisionOrderSemantics, DecisionPreference, DecisionVisibility, DecisionZone, PendingDecision,
};
use super::{array, bool_field, field, parse_ids, seat_value, str_field, u32_field, usize_field};

pub(super) fn decision_checkpoint_json(pending: &PendingDecision) -> Option<Value> {
    let continuation = continuation_json(&pending.continuation)?;
    Some(json!({
        "preference": preference_json(pending.observation.preference),
        "continuation": continuation,
    }))
}

#[allow(clippy::too_many_lines)]
fn continuation_json(continuation: &DecisionContinuation) -> Option<Value> {
    let value = match continuation {
        DecisionContinuation::BasicLandTypeTextChange { target } => json!({
            "kind": "basicLandTypeTextChange",
            "target": target_json(*target),
        }),
        DecisionContinuation::MiracleReveal { card } => {
            json!({"kind": "miracleReveal", "card": card.0})
        }
        DecisionContinuation::PileSplit { owner } => {
            json!({"kind": "pileSplit", "owner": owner.index()})
        }
        DecisionContinuation::PileChoice { first, second } => json!({
            "kind": "pileChoice",
            "first": ids_json(first),
            "second": ids_json(second),
        }),
        DecisionContinuation::SacrificeOfChoice {
            followup: None,
            optional,
        } => json!({"kind": "sacrificeOfChoice", "optional": optional}),
        DecisionContinuation::DestroyOfChoice { can_regenerate } => json!({
            "kind": "destroyOfChoice",
            "canRegenerate": can_regenerate,
        }),
        DecisionContinuation::TimeVault {
            permanent,
            remaining,
        } => json!({
            "kind": "timeVault",
            "permanent": permanent.0,
            "remaining": ids_json(remaining),
        }),
        DecisionContinuation::SylvanOffer { player } => {
            json!({"kind": "sylvanOffer", "player": player.index()})
        }
        DecisionContinuation::SylvanSelect {
            player,
            candidates,
            choices_left,
        } => json!({
            "kind": "sylvanSelect",
            "player": player.index(),
            "candidates": ids_json(candidates),
            "choicesLeft": choices_left,
        }),
        DecisionContinuation::SylvanMode {
            player,
            card,
            candidates,
            choices_left,
        } => json!({
            "kind": "sylvanMode",
            "player": player.index(),
            "card": card.0,
            "candidates": ids_json(candidates),
            "choicesLeft": choices_left,
        }),
        DecisionContinuation::TetravusDetach { source } => {
            json!({"kind": "tetravusDetach", "source": source.0})
        }
        DecisionContinuation::TetravusAssemble { source } => {
            json!({"kind": "tetravusAssemble", "source": source.0})
        }
        DecisionContinuation::DiscardForEffect { .. }
        | DecisionContinuation::Tutor
        | DecisionContinuation::LibrarySearch { .. }
        | DecisionContinuation::OptionalManaPayment { .. }
        | DecisionContinuation::ManaPaymentOrElse { .. }
        | DecisionContinuation::ChainLightning { .. }
        | DecisionContinuation::Fork { .. }
        | DecisionContinuation::OptionalEffect { .. }
        | DecisionContinuation::ChoosePermanentForEffect { .. }
        | DecisionContinuation::RevealedPileSplit { .. }
        | DecisionContinuation::RevealedPileChoice { .. }
        | DecisionContinuation::SeparateIntoPiles { .. }
        | DecisionContinuation::ChoosePile { .. }
        | DecisionContinuation::SacrificeOfChoice {
            followup: Some(_), ..
        }
        | DecisionContinuation::CounterUnlessPaid { .. }
        | DecisionContinuation::GrislySalvage { .. }
        | DecisionContinuation::RecallDiscard { .. }
        | DecisionContinuation::RecallReturn { .. }
        | DecisionContinuation::Duress { .. }
        | DecisionContinuation::Balance { .. }
        | DecisionContinuation::ExileFromHand { .. }
        | DecisionContinuation::AugurOfBolas { .. }
        | DecisionContinuation::TopCardSelection { .. }
        | DecisionContinuation::BattlefieldEntryReplacement { .. }
        | DecisionContinuation::BattlefieldEntryPayment { .. }
        | DecisionContinuation::BattlefieldEntryCardName { .. }
        | DecisionContinuation::BattlefieldEntryCopy { .. }
        | DecisionContinuation::BattlefieldEntryCreatureType { .. }
        | DecisionContinuation::TriggerOrder { .. }
        | DecisionContinuation::TriggerPlacement { .. } => return None,
    };
    Some(value)
}

pub(super) fn parse_pending_decision(
    observation: &Value,
    checkpoint: &Value,
) -> Result<Option<PendingDecision>, String> {
    let Some(visible) = observation.get("decision").filter(|value| !value.is_null()) else {
        if checkpoint
            .get("decisionState")
            .is_some_and(|value| !value.is_null())
        {
            return Err("checkpoint decision is not visible to its viewer".into());
        }
        return Ok(None);
    };
    let state = field(checkpoint, "decisionState")?;
    if state.is_null() {
        return Err("decision continuation lacks a semantic checkpoint encoding".into());
    }
    Ok(Some(PendingDecision {
        observation: parse_decision_observation(visible, field(state, "preference")?)?,
        continuation: parse_continuation(field(state, "continuation")?)?,
    }))
}

fn parse_decision_observation(
    value: &Value,
    preference: &Value,
) -> Result<DecisionObservation, String> {
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
        prompt: str_field(value, "prompt")?.to_owned(),
        visibility: match str_field(value, "visibility")? {
            "Public" => DecisionVisibility::Public,
            "Private" => DecisionVisibility::Private,
            other => return Err(format!("unknown decision visibility {other}")),
        },
        preference: parse_preference(preference)?,
        minimum: usize_field(value, "minimum")?,
        maximum: usize_field(value, "maximum")?,
        cancellable: bool_field(value, "cancellable")?,
        options: array(field(value, "options")?)?
            .iter()
            .map(parse_option)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn parse_option(value: &Value) -> Result<DecisionOption, String> {
    let parse_card = |value: &Value| {
        Ok((
            GameObjectId(u32_field(value, "objectId")?),
            CardDefinitionId(
                u16::try_from(usize_field(value, "definition")?)
                    .map_err(|_| "decision card definition is too large")?,
            ),
        ))
    };
    Ok(DecisionOption {
        id: u32_field(value, "id")?,
        label: str_field(value, "label")?.to_owned(),
        card: value
            .get("card")
            .filter(|value| !value.is_null())
            .map(parse_card)
            .transpose()?,
        members: array(field(value, "members")?)?
            .iter()
            .map(parse_card)
            .collect::<Result<Vec<_>, String>>()?,
        ability_text: value
            .get("abilityText")
            .and_then(Value::as_str)
            .map(str::to_owned),
        zone: parse_decision_zone(str_field(value, "zone")?)?,
    })
}

#[allow(clippy::too_many_lines)]
fn parse_continuation(value: &Value) -> Result<DecisionContinuation, String> {
    let player = |name| field(value, name).and_then(seat_index);
    let id = |name| u32_field(value, name).map(GameObjectId);
    match str_field(value, "kind")? {
        "basicLandTypeTextChange" => Ok(DecisionContinuation::BasicLandTypeTextChange {
            target: parse_target(field(value, "target")?)?,
        }),
        "miracleReveal" => Ok(DecisionContinuation::MiracleReveal { card: id("card")? }),
        "pileSplit" => Ok(DecisionContinuation::PileSplit {
            owner: player("owner")?,
        }),
        "pileChoice" => Ok(DecisionContinuation::PileChoice {
            first: parse_ids(field(value, "first")?)?,
            second: parse_ids(field(value, "second")?)?,
        }),
        "sacrificeOfChoice" => Ok(DecisionContinuation::SacrificeOfChoice {
            followup: None,
            optional: bool_field(value, "optional")?,
        }),
        "destroyOfChoice" => Ok(DecisionContinuation::DestroyOfChoice {
            can_regenerate: bool_field(value, "canRegenerate")?,
        }),
        "timeVault" => Ok(DecisionContinuation::TimeVault {
            permanent: id("permanent")?,
            remaining: parse_ids(field(value, "remaining")?)?,
        }),
        "sylvanOffer" => Ok(DecisionContinuation::SylvanOffer {
            player: player("player")?,
        }),
        "sylvanSelect" => Ok(DecisionContinuation::SylvanSelect {
            player: player("player")?,
            candidates: parse_ids(field(value, "candidates")?)?,
            choices_left: usize_field(value, "choicesLeft")?,
        }),
        "sylvanMode" => Ok(DecisionContinuation::SylvanMode {
            player: player("player")?,
            card: id("card")?,
            candidates: parse_ids(field(value, "candidates")?)?,
            choices_left: usize_field(value, "choicesLeft")?,
        }),
        "tetravusDetach" => Ok(DecisionContinuation::TetravusDetach {
            source: id("source")?,
        }),
        "tetravusAssemble" => Ok(DecisionContinuation::TetravusAssemble {
            source: id("source")?,
        }),
        other => Err(format!("unknown decision continuation {other}")),
    }
}

fn preference_json(preference: DecisionPreference) -> Value {
    match preference {
        DecisionPreference::HigherCardValue => Value::from("higherCardValue"),
        DecisionPreference::LowerCardValue => Value::from("lowerCardValue"),
        DecisionPreference::BalancedPartition => Value::from("balancedPartition"),
        DecisionPreference::LinkedExileTargets => Value::from("linkedExileTargets"),
        DecisionPreference::RemovalChoice => Value::from("removalChoice"),
        DecisionPreference::PreferOption(option) => json!({"preferOption": option}),
        DecisionPreference::Neutral => Value::from("neutral"),
    }
}

fn parse_preference(value: &Value) -> Result<DecisionPreference, String> {
    match value.as_str() {
        Some("higherCardValue") => Ok(DecisionPreference::HigherCardValue),
        Some("lowerCardValue") => Ok(DecisionPreference::LowerCardValue),
        Some("balancedPartition") => Ok(DecisionPreference::BalancedPartition),
        Some("linkedExileTargets") => Ok(DecisionPreference::LinkedExileTargets),
        Some("removalChoice") => Ok(DecisionPreference::RemovalChoice),
        Some("neutral") => Ok(DecisionPreference::Neutral),
        Some(other) => Err(format!("unknown decision preference {other}")),
        None => Ok(DecisionPreference::PreferOption(u32_field(
            value,
            "preferOption",
        )?)),
    }
}

fn ids_json(ids: &[GameObjectId]) -> Vec<u32> {
    ids.iter().map(|id| id.0).collect()
}

fn target_json(target: Target) -> Value {
    match target {
        Target::Player(player) => json!({"type": "player", "seat": seat_name(player)}),
        Target::Card(id) => json!({"type": "card", "objectId": id.0}),
        Target::Permanent(id) => json!({"type": "permanent", "objectId": id.0}),
        Target::Spell(id) => json!({"type": "spell", "objectId": id.0}),
    }
}

fn parse_target(value: &Value) -> Result<Target, String> {
    match str_field(value, "type")? {
        "player" => Ok(Target::Player(seat_value(field(value, "seat")?)?)),
        "card" => Ok(Target::Card(GameObjectId(u32_field(value, "objectId")?))),
        "permanent" => Ok(Target::Permanent(GameObjectId(u32_field(
            value, "objectId",
        )?))),
        "spell" => Ok(Target::Spell(GameObjectId(u32_field(value, "objectId")?))),
        other => Err(format!("unknown target kind {other}")),
    }
}

fn parse_decision_zone(value: &str) -> Result<DecisionZone, String> {
    match value {
        "Hand" => Ok(DecisionZone::Hand),
        "Graveyard" => Ok(DecisionZone::Graveyard),
        "Battlefield" => Ok(DecisionZone::Battlefield),
        "Stack" => Ok(DecisionZone::Stack),
        "Library" => Ok(DecisionZone::Library),
        "Exile" => Ok(DecisionZone::Exile),
        "Command" => Ok(DecisionZone::Command),
        "DrawnThisStep" => Ok(DecisionZone::DrawnThisStep),
        "None" => Ok(DecisionZone::None),
        other => Err(format!("unknown decision zone {other}")),
    }
}

fn seat_index(value: &Value) -> Result<PlayerId, String> {
    match value.as_u64() {
        Some(0) => Ok(PlayerId::One),
        Some(1) => Ok(PlayerId::Two),
        _ => Err("seat index must be 0 or 1".into()),
    }
}

fn seat_name(player: PlayerId) -> &'static str {
    if player == PlayerId::One { "p1" } else { "p2" }
}
