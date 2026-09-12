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

impl Game {
    pub(super) fn prepared_battlefield_predicate_matches(
        &self,
        plan: &crate::prepared_engine::PreparedPredicate,
        permanent: &super::Permanent,
        source: GameObjectId,
    ) -> bool {
        use crate::prepared_engine::PreparedPredicateLeaf as Leaf;
        plan.matches(&mut |leaf| match leaf {
            Leaf::Constant(value) => value,
            Leaf::Source => permanent.card.id == source,
            Leaf::Token => permanent.card.definition.is_token(),
            Leaf::Tapped => permanent.tapped,
            Leaf::Type(kind) => self
                .permanent_types(permanent)
                .expect("battlefield types")
                .contains(kind),
            Leaf::BasicLandTypes(types) => {
                if !self
                    .permanent_types(permanent)
                    .expect("battlefield types")
                    .contains(crate::CardType::Land)
                {
                    return false;
                }
                let subtypes = self.effective_subtypes(permanent);
                types.iter().any(|kind| subtypes.contains(&kind.subtype()))
            }
            Leaf::Color(color) => color
                .color_index()
                .is_some_and(|index| self.permanent_colors(permanent)[index]),
            Leaf::ColorCount(count) => {
                self.permanent_colors(permanent)
                    .iter()
                    .filter(|present| **present)
                    .count()
                    == usize::from(count)
            }
            Leaf::Subtype(subtype) => self.effective_subtypes(permanent).contains(&subtype),
            Leaf::Supertype(kind) => self
                .permanent_supertypes(permanent)
                .unwrap_or_default()
                .contains(kind),
            Leaf::ManaValueAtMost(limit) => {
                self.permanent_mana_value(permanent) <= u16::from(limit)
            }
        })
    }

    /// Printed summaries answer only when no live ability overlay can change
    /// them. All other objects retain the complete ordered ability evaluator.
    pub(super) fn prepared_keyword_mask(&self, permanent: &super::Permanent) -> Option<u64> {
        use crate::prepared_engine::PreparedStaticLane;
        if !self.prepared_engine.enabled()
            || permanent.active_copy_values().is_some()
            || !permanent.resolved_continuous_effects.is_empty()
            || !permanent.counters.is_empty()
            || !permanent.temporary_keywords.is_empty()
            || !permanent.keywords_until_upkeep_of.is_empty()
            || !permanent.text_changes.is_empty()
            || permanent.suspend_haste
        {
            return None;
        }
        let program = self.prepared_static_program(Self::effective_rules_source(permanent))?;
        if self
            .battlefield
            .iter()
            .chain(self.emblems.iter())
            .any(|source| {
                source.active_copy_values().is_some()
                    || self
                        .prepared_static_program(Self::effective_rules_source(source))
                        .is_none_or(|program| program.supplies(PreparedStaticLane::Abilities))
            })
            || self
                .players
                .iter()
                .flat_map(|player| &player.graveyard)
                .any(|card| self.prepared_supplies_graveyard_static(card.definition) != Some(false))
        {
            return None;
        }
        Some(if self.rules_text_abilities_removed(permanent) {
            0
        } else {
            program.base_keywords()
        })
    }
}
