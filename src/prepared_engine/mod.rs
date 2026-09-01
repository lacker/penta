//! Optional lowerings of the declarative card model.
//!
//! The card model remains the reference representation. Preparation is a
//! derived, process-local cache: unsupported roots are absent and callers run
//! the reference engine before making any mutation. Nothing in this module is
//! serialized into checkpoints.

mod compiler;
mod executor;

use std::collections::HashMap;

use crate::{
    AbilityId, AppliedEffectDef, CardCatalog, CardDefinitionId, CardPartId, EffectDef,
    EffectRecipientDef, GrantId, PlayerId, TriggerConditionDef, ZoneKind,
};

pub(crate) use compiler::{compile_catalog, compile_effect};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PreparedEffect {
    DrawCards { count: u16 },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PreparedStaticProgram {
    supplies_land_type_effect: bool,
    has_static_effects: bool,
    lanes: u8,
    abilities: Box<[PreparedStaticAbility]>,
}

impl PreparedStaticProgram {
    pub(crate) fn abilities(&self) -> &[PreparedStaticAbility] {
        &self.abilities
    }

    pub(crate) const fn supplies_land_type_effect(&self) -> bool {
        self.supplies_land_type_effect
    }

    pub(crate) const fn supplies(&self, lane: PreparedStaticLane) -> bool {
        if matches!(lane, PreparedStaticLane::Any) {
            self.has_static_effects
        } else {
            self.lanes & lane.mask() != 0
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedStaticAbility {
    pub(crate) id: AbilityId,
    pub(crate) source_zones: &'static [ZoneKind],
    pub(crate) reference_effect: EffectDef,
    pub(crate) applications: Option<Box<[PreparedStaticApplication]>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedStaticApplication {
    pub(crate) recipient: EffectRecipientDef,
    pub(crate) starts_in_type_layer: bool,
    pub(crate) trigger_conditions: Box<[(TriggerConditionDef, bool)]>,
    pub(crate) components: Box<[PreparedStaticComponent]>,
    lanes: u8,
}

impl PreparedStaticApplication {
    pub(crate) const fn supplies(&self, lane: PreparedStaticLane) -> bool {
        matches!(lane, PreparedStaticLane::Any) || self.lanes & lane.mask() != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PreparedStaticComponent {
    pub(crate) effect: AppliedEffectDef,
    pub(crate) grant: Option<GrantId>,
    pub(crate) component_order: u16,
    lane: PreparedStaticLane,
}

impl PreparedStaticComponent {
    pub(crate) fn supplies(self, lane: PreparedStaticLane) -> bool {
        lane == PreparedStaticLane::Any || self.lane == lane
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PreparedStaticLane {
    Any,
    Other,
    Rules,
    CardTypes,
    Colors,
    Abilities,
    Subtypes,
    PowerToughness,
}

impl PreparedStaticLane {
    const fn mask(self) -> u8 {
        match self {
            Self::Any => u8::MAX,
            Self::Other => 0,
            Self::Rules => 1 << 0,
            Self::CardTypes => 1 << 1,
            Self::Colors => 1 << 2,
            Self::Abilities => 1 << 3,
            Self::Subtypes => 1 << 4,
            Self::PowerToughness => 1 << 5,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PreparedCatalog {
    dense_primary_static_programs: Vec<Option<PreparedStaticProgram>>,
    sparse_primary_static_programs: HashMap<CardDefinitionId, PreparedStaticProgram>,
    other_static_programs: HashMap<(CardDefinitionId, CardPartId), PreparedStaticProgram>,
    dense_graveyard_static_sources: Vec<Option<bool>>,
    sparse_graveyard_static_sources: HashMap<CardDefinitionId, bool>,
}

impl PreparedCatalog {
    fn insert_graveyard_static_source(
        &mut self,
        definition: CardDefinitionId,
        supplies_graveyard_static: bool,
    ) {
        if let Ok(index) = u16::try_from(definition.get()) {
            let index = usize::from(index);
            if self.dense_graveyard_static_sources.len() <= index {
                self.dense_graveyard_static_sources.resize(index + 1, None);
            }
            self.dense_graveyard_static_sources[index] = Some(supplies_graveyard_static);
        } else {
            self.sparse_graveyard_static_sources
                .insert(definition, supplies_graveyard_static);
        }
    }

    fn insert_static_program(
        &mut self,
        definition: CardDefinitionId,
        part: CardPartId,
        program: PreparedStaticProgram,
    ) {
        if part != CardPartId::PRIMARY {
            self.other_static_programs
                .insert((definition, part), program);
            return;
        }
        if let Ok(index) = u16::try_from(definition.get()) {
            let index = usize::from(index);
            if self.dense_primary_static_programs.len() <= index {
                self.dense_primary_static_programs
                    .resize_with(index + 1, || None);
            }
            self.dense_primary_static_programs[index] = Some(program);
        } else {
            self.sparse_primary_static_programs
                .insert(definition, program);
        }
    }

    #[inline]
    fn static_program(
        &self,
        definition: CardDefinitionId,
        part: CardPartId,
    ) -> Option<&PreparedStaticProgram> {
        if part != CardPartId::PRIMARY {
            return self.other_static_programs.get(&(definition, part));
        }
        let program = if let Ok(index) = u16::try_from(definition.get()) {
            self.dense_primary_static_programs
                .get(usize::from(index))?
                .as_ref()?
        } else {
            self.sparse_primary_static_programs.get(&definition)?
        };
        Some(program)
    }

    #[inline]
    fn supplies_graveyard_static(&self, definition: CardDefinitionId) -> Option<bool> {
        if let Ok(index) = u16::try_from(definition.get()) {
            self.dense_graveyard_static_sources
                .get(usize::from(index))
                .copied()
                .flatten()
        } else {
            self.sparse_graveyard_static_sources
                .get(&definition)
                .copied()
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PreparedEngine {
    enabled: bool,
    catalog: std::sync::Arc<PreparedCatalog>,
}

impl PreparedEngine {
    pub(crate) fn compile(catalog: &CardCatalog) -> Self {
        Self {
            enabled: true,
            catalog: catalog.prepared_catalog(),
        }
    }

    pub(crate) const fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    #[inline]
    pub(crate) fn static_program(
        &self,
        definition: CardDefinitionId,
        part: CardPartId,
    ) -> Option<&PreparedStaticProgram> {
        if self.enabled {
            self.catalog.static_program(definition, part)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn supplies_graveyard_static(&self, definition: CardDefinitionId) -> Option<bool> {
        if self.enabled {
            self.catalog.supplies_graveyard_static(definition)
        } else {
            None
        }
    }
}

pub(crate) trait PreparedHost {
    fn draw_cards(&mut self, player: PlayerId, count: u16);
}

pub(crate) fn execute_effect(
    effect: PreparedEffect,
    host: &mut impl PreparedHost,
    controller: PlayerId,
) {
    executor::execute(effect, host, controller);
}
