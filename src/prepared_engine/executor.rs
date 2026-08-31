use super::{PreparedEffect, PreparedHost};
use crate::PlayerId;

pub(super) fn execute(effect: PreparedEffect, host: &mut impl PreparedHost, controller: PlayerId) {
    match effect {
        PreparedEffect::DrawCards { count } => host.draw_cards(controller, count),
    }
}
