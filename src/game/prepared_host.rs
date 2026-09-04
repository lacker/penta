use super::{Game, ObjectCharacteristics};
use crate::prepared_engine::{PreparedHost, PreparedStaticProgram};
use crate::{AbilityDef, AbilityOrigin, CardDefinitionId, GameObjectId, PlayerId};

impl Game {
    /// Enables or disables every optional prepared path for this game.
    ///
    /// Disabling is intended for differential tests, diagnostics, and
    /// benchmarks. It does not alter checkpoint or replay semantics.
    pub fn set_prepared_engine_enabled(&mut self, enabled: bool) {
        self.prepared_engine.set_enabled(enabled);
    }

    #[must_use]
    pub const fn prepared_engine_enabled(&self) -> bool {
        self.prepared_engine.enabled()
    }

    #[inline]
    pub(super) fn prepared_static_program(
        &self,
        source: ObjectCharacteristics,
    ) -> Option<&PreparedStaticProgram> {
        let ObjectCharacteristics::Card { definition, part } = source else {
            return None;
        };
        self.prepared_engine.static_program(definition, part)
    }

    #[inline]
    pub(super) fn prepared_supplies_graveyard_static(
        &self,
        definition: CardDefinitionId,
    ) -> Option<bool> {
        self.prepared_engine.supplies_graveyard_static(definition)
    }
}

impl PreparedHost for Game {
    fn draw_cards(&mut self, player: PlayerId, count: u16) {
        self.draw_instruction(player, count);
    }

    fn grant_source_ability_until_end_of_turn(
        &mut self,
        source: Option<GameObjectId>,
        origin: AbilityOrigin,
        ability: &'static AbilityDef,
    ) {
        Game::grant_source_ability_until_end_of_turn(self, source, origin, ability);
    }
}
