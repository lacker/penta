use serde_json::Value;

use crate::{CardCatalog, CardPartId, GameObjectId};

use super::super::{
    CharacteristicSource, ContinuousEffectTimestamp, CounterKind, Game, ObjectBacking,
    ObjectInstance, ObjectKind, Permanent,
};
use super::model::EmblemSnapshot;
use super::semantics::{
    ability_locator_matches_origin, catalog_emblem_characteristics, emblem_characteristics_locator,
};
use super::stack::parse_ability_origin;
use super::{array, field, seat_value, str_field, u32_field};

pub(super) fn emblem_snapshot(catalog: &CardCatalog, emblem: &Permanent) -> Option<EmblemSnapshot> {
    let CharacteristicSource::Emblem(characteristics) = emblem.card.characteristics else {
        return None;
    };
    Some(EmblemSnapshot {
        object_id: emblem.card.id.0,
        characteristics: emblem_characteristics_locator(catalog, characteristics)?,
        owner: emblem.card.owner.index(),
        timestamp: emblem.timestamp.0,
        entered_controller_turn: emblem.entered_controller_turn,
    })
}

pub(super) fn parse_emblems(
    observation: &Value,
    snapshots: &[EmblemSnapshot],
    game: &Game,
) -> Result<Vec<Permanent>, String> {
    let visible = array(field(observation, "emblems")?)?;
    if visible.len() != snapshots.len() {
        return Err("checkpoint emblems do not match observation".into());
    }
    visible
        .iter()
        .zip(snapshots)
        .map(|(shown, state)| {
            let id = GameObjectId(u32_field(shown, "objectId")?);
            if id.0 != state.object_id {
                return Err("checkpoint emblem id does not match observation".into());
            }
            let characteristics =
                catalog_emblem_characteristics(&game.catalog, &state.characteristics)
                    .ok_or("checkpoint emblem characteristics cannot be reconstructed")?;
            if str_field(shown, "name")? != characteristics.name() {
                return Err("checkpoint emblem name does not match its characteristics".into());
            }
            let expected_texts = characteristics
                .abilities()
                .iter()
                .map(|ability| ability.text)
                .collect::<Vec<_>>();
            let shown_texts = array(field(shown, "abilityTexts")?)?
                .iter()
                .map(|value| value.as_str().ok_or("emblem ability text must be a string"))
                .collect::<Result<Vec<_>, _>>()?;
            if shown_texts != expected_texts {
                return Err(
                    "checkpoint emblem ability texts do not match its characteristics".into(),
                );
            }
            let owner = player(state.owner)?;
            let controller = seat_value(field(shown, "controller")?)?;
            let source = parse_ability_origin(field(shown, "sourceAbility")?)?;
            if !ability_locator_matches_origin(state.characteristics.creator(), source) {
                return Err("checkpoint emblem source does not match its creator".into());
            }
            let card = ObjectInstance {
                id,
                definition: ObjectKind::Emblem,
                owner,
                backing: ObjectBacking::None,
                characteristics: CharacteristicSource::Emblem(characteristics),
                counters: [0; CounterKind::COUNT],
            };
            let mut emblem = Permanent::entering(
                card,
                CardPartId::PRIMARY,
                controller,
                state.entered_controller_turn,
            );
            emblem.timestamp = ContinuousEffectTimestamp(state.timestamp);
            emblem.emblem_source = Some(source);
            Ok(emblem)
        })
        .collect()
}

fn player(index: usize) -> Result<crate::PlayerId, String> {
    match index {
        0 => Ok(crate::PlayerId::One),
        1 => Ok(crate::PlayerId::Two),
        _ => Err("seat index must be 0 or 1".into()),
    }
}
