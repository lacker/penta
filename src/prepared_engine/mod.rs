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
    abilities: Box<[PreparedStaticAbility]>,
}

impl PreparedStaticProgram {
    pub(crate) const fn supplies_land_type_effect(&self) -> bool {
        self.supplies_land_type_effect
    }

    pub(crate) fn abilities(&self) -> &[PreparedStaticAbility] {
        &self.abilities
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
    dense_static_programs: Vec<Option<Vec<(CardPartId, PreparedStaticProgram)>>>,
    sparse_static_programs: HashMap<CardDefinitionId, Vec<(CardPartId, PreparedStaticProgram)>>,
}

impl PreparedCatalog {
    fn insert_static_program(
        &mut self,
        definition: CardDefinitionId,
        part: CardPartId,
        program: PreparedStaticProgram,
    ) {
        let parts = if let Ok(index) = u16::try_from(definition.get()) {
            let index = usize::from(index);
            if self.dense_static_programs.len() <= index {
                self.dense_static_programs.resize_with(index + 1, || None);
            }
            self.dense_static_programs[index].get_or_insert_with(Vec::new)
        } else {
            self.sparse_static_programs.entry(definition).or_default()
        };
        parts.push((part, program));
    }

    fn static_program(
        &self,
        definition: CardDefinitionId,
        part: CardPartId,
    ) -> Option<&PreparedStaticProgram> {
        let parts = if let Ok(index) = u16::try_from(definition.get()) {
            self.dense_static_programs
                .get(usize::from(index))?
                .as_ref()?
        } else {
            self.sparse_static_programs.get(&definition)?
        };
        parts
            .iter()
            .find_map(|(candidate, program)| (*candidate == part).then_some(program))
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

    pub(crate) fn static_program(
        &self,
        definition: CardDefinitionId,
        part: CardPartId,
    ) -> Option<&PreparedStaticProgram> {
        self.enabled
            .then(|| self.catalog.static_program(definition, part))
            .flatten()
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
