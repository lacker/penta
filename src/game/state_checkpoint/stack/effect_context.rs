use super::super::model::{AbilitySourceSnapshot, ReplacedDrawSnapshot};
use super::super::{
    AbilitySourceRef, GameObjectId, PlayerId, ability_origin_from_snapshot, ability_origin_snapshot,
};
use crate::game::ReplacedDrawContinuation;

pub(super) fn replaced_draw_snapshot(draw: &ReplacedDrawContinuation) -> ReplacedDrawSnapshot {
    ReplacedDrawSnapshot {
        player: draw.player.index(),
        applied: draw
            .applied
            .iter()
            .map(|source| AbilitySourceSnapshot {
                object: source.object.0,
                ability: ability_origin_snapshot(source.ability),
            })
            .collect(),
    }
}

pub(super) fn parse_replaced_draw(
    draw: ReplacedDrawSnapshot,
) -> Result<ReplacedDrawContinuation, String> {
    Ok(ReplacedDrawContinuation {
        player: match draw.player {
            0 => PlayerId::One,
            1 => PlayerId::Two,
            _ => return Err("replaced draw player is out of range".into()),
        },
        applied: draw
            .applied
            .into_iter()
            .map(|source| AbilitySourceRef {
                object: GameObjectId(source.object),
                ability: ability_origin_from_snapshot(source.ability),
            })
            .collect(),
    })
}
