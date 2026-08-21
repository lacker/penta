//! Reading one option of a pending decision back from a checkpoint.

use serde_json::Value;

use super::super::model::{DecisionCardSnapshot, DecisionOptionSnapshot};
use super::super::semantics::object_characteristics_from_snapshot;
use super::super::wire::{array, field, str_field, u32_field};
use super::parse_decision_zone;
use crate::CardCatalog;
use crate::game::DecisionOption;
use crate::ids::GameObjectId;

pub(super) fn parse_option(
    value: &Value,
    snapshot: &DecisionOptionSnapshot,
    catalog: &CardCatalog,
) -> Result<DecisionOption, String> {
    if u32_field(value, "id")? != snapshot.id {
        return Err("checkpoint decision option id does not match observation".into());
    }
    let parse_card = |value: &Value, snapshot: &DecisionCardSnapshot| {
        let object = GameObjectId(u32_field(value, "objectId")?);
        if object.0 != snapshot.object_id {
            return Err("checkpoint decision card id does not match observation".to_owned());
        }
        let characteristics =
            object_characteristics_from_snapshot(catalog, &snapshot.characteristics).ok_or_else(
                || "decision card characteristics are absent from this catalog".to_owned(),
            )?;
        Ok((object, characteristics))
    };
    let shown_card = value.get("card").filter(|value| !value.is_null());
    let card = match (shown_card, snapshot.card.as_ref()) {
        (Some(value), Some(snapshot)) => Some(parse_card(value, snapshot)?),
        (None, None) => None,
        _ => return Err("checkpoint decision card presence does not match observation".into()),
    };
    let shown_members = array(field(value, "members")?)?;
    if shown_members.len() != snapshot.members.len() {
        return Err("checkpoint decision members do not match observation".into());
    }
    Ok(DecisionOption {
        id: snapshot.id,
        label: str_field(value, "label")?.to_owned(),
        card,
        members: shown_members
            .iter()
            .zip(&snapshot.members)
            .map(|(value, snapshot)| parse_card(value, snapshot))
            .collect::<Result<Vec<_>, String>>()?,
        ability_text: value
            .get("abilityText")
            .and_then(Value::as_str)
            .map(str::to_owned),
        zone: parse_decision_zone(str_field(value, "zone")?)?,
    })
}
