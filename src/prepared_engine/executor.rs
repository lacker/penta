use super::{PreparedEffect, PreparedHost};
use crate::{AbilityOrigin, GameObjectId, PlayerId};

pub(super) fn execute(
    effect: PreparedEffect,
    host: &mut impl PreparedHost,
    controller: PlayerId,
    source: Option<GameObjectId>,
    origin: AbilityOrigin,
) {
    #[cfg(feature = "engine-profiling")]
    crate::engine_profiling::record(
        "effect_dispatch",
        match effect {
            PreparedEffect::DrawCards { .. } => "DrawCards",
            PreparedEffect::GrantSourceAbilityUntilEndOfTurn { .. } => "Apply",
        },
        "prepared",
        "entered",
    );
    match effect {
        PreparedEffect::DrawCards { count } => host.draw_cards(controller, count),
        PreparedEffect::GrantSourceAbilityUntilEndOfTurn { ability } => {
            host.grant_source_ability_until_end_of_turn(source, origin, ability);
        }
    }
}
