use super::{GameObjectId, PlayerId};

/// A turn-long damage-prevention rule whose affected objects are evaluated
/// when damage would be dealt rather than frozen when the spell resolves.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RelationalDamagePrevention {
    ToPlayerAndControlledCreatures(PlayerId),
    FromAllExcept(GameObjectId),
}
