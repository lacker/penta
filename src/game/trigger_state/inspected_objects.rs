// Visibility already granted while resolving a collection instruction. It
// stays with the resolution so a later choice can still display and rebind
// a previously selected card, without revealing it to another player.
impl EffectResolutionContext {
    pub(super) fn remember_inspected_objects(
        &mut self,
        player: PlayerId,
        objects: &[GameObjectId],
    ) {
        self.inspected.with_mut(|known| {
            for object in objects {
                if !known.contains(&(player, *object)) {
                    known.push((player, *object));
                }
            }
        });
    }

    pub(super) fn inspected_objects(&self) -> Vec<(PlayerId, GameObjectId)> {
        self.inspected.snapshot()
    }
}
