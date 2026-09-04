use super::{PreparedEffect, PreparedHost};
use crate::{AbilityOrigin, GameObjectId, PlayerId};

pub(super) fn execute(
    effect: PreparedEffect,
    host: &mut impl PreparedHost,
    controller: PlayerId,
    source: Option<GameObjectId>,
    origin: AbilityOrigin,
) {
    match effect {
        PreparedEffect::DrawCards { count } => host.draw_cards(controller, count),
        PreparedEffect::GrantSourceAbilityUntilEndOfTurn { ability } => {
            host.grant_source_ability_until_end_of_turn(source, origin, ability);
        }
    }
}
