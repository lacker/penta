//! Reuse ability-layer inputs within one immutable board read.
//!
//! Recursive layer walks deliberately see a lower-layer answer. Each of the
//! three pass flags and the prospective X therefore belongs in the key, even
//! though the board itself is borrowed immutably. Prospective and last-known
//! objects take the uncached path, as do recursive object-count conditions
//! whose private memo can influence the answers observed during that walk.

use std::collections::HashMap;

use crate::game::{AbilityLayerOperation, EffectiveAbility, Game, Permanent};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(in crate::game) struct AbilityReadKey {
    permanent: usize,
    ability_pass: bool,
    characteristic_pass: bool,
    stat_pass: bool,
    x: Option<u16>,
}

#[derive(Default)]
pub(super) struct AbilityReadMemo {
    base: HashMap<AbilityReadKey, Vec<EffectiveAbility>>,
    operations: HashMap<AbilityReadKey, Vec<AbilityLayerOperation>>,
}

impl Game {
    pub(in crate::game) fn ability_read_key(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Option<AbilityReadKey> {
        if prospective.is_some() || Self::object_count_condition_read_active() {
            return None;
        }
        Some(AbilityReadKey {
            permanent: self.live_read_key(permanent)?,
            ability_pass: crate::game::ability_layers::STATIC_ABILITY_LAYER_PASS
                .with(std::cell::Cell::get),
            characteristic_pass:
                crate::game::continuous_effects::STATIC_SET_CHARACTERISTIC_LAYER_PASS
                    .with(std::cell::Cell::get),
            stat_pass: crate::game::creature_characteristics::STATIC_POWER_TOUGHNESS_LAYER_PASS
                .with(std::cell::Cell::get),
            x: self.prospective_x.get(),
        })
    }

    pub(in crate::game) fn remembered_base_abilities(
        &self,
        key: Option<AbilityReadKey>,
    ) -> Option<Vec<EffectiveAbility>> {
        self.board_memo(|memo| memo.abilities.base.get(&key?).cloned())
    }

    pub(in crate::game) fn remember_base_abilities(
        &self,
        key: Option<AbilityReadKey>,
        abilities: &[EffectiveAbility],
    ) {
        if let Some(key) = key {
            self.remember_board(|memo| {
                memo.abilities.base.insert(key, abilities.to_vec());
            });
        }
    }

    pub(in crate::game) fn remembered_ability_operations(
        &self,
        key: Option<AbilityReadKey>,
    ) -> Option<Vec<AbilityLayerOperation>> {
        self.board_memo(|memo| memo.abilities.operations.get(&key?).cloned())
    }

    pub(in crate::game) fn remember_ability_operations(
        &self,
        key: Option<AbilityReadKey>,
        operations: &[AbilityLayerOperation],
    ) {
        if let Some(key) = key {
            self.remember_board(|memo| {
                memo.abilities.operations.insert(key, operations.to_vec());
            });
        }
    }
}
