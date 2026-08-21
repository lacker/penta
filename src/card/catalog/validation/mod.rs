mod abilities;
mod composition;
mod presentation;
mod program_context;
mod targeting;

use std::collections::HashSet;

use self::abilities::{
    validate_abilities, validate_alternative_cast_abilities,
    validate_optional_additional_cost_abilities,
};
use self::composition::{
    structure_parts, validate_cost_ids, validate_fused_option, validate_modes_and_targets,
    validate_spell_form,
};
pub(super) use self::presentation::validate_semantic_spell_presentation;
#[cfg(test)]
pub(super) use self::targeting::{validate_ability_targets, validate_replacement_ability_targets};
use crate::card::CardDefinition;
use crate::card::catalog::CatalogError;

pub(super) fn validate_composition(definition: &CardDefinition) -> Result<(), CatalogError> {
    if let Some(explanation) = definition.rules.coherence_error() {
        return Err(CatalogError::IncoherentCardRules {
            definition: definition.id,
            part: definition.primary_part_id(),
            explanation,
        });
    }

    let mut defined_parts = HashSet::new();
    for part in &definition.parts {
        if !defined_parts.insert(part.id) {
            return Err(CatalogError::DuplicatePartId {
                definition: definition.id,
                part: part.id,
            });
        }
        if let Some(explanation) = part.rules.coherence_error() {
            return Err(CatalogError::IncoherentCardRules {
                definition: definition.id,
                part: part.id,
                explanation,
            });
        }
        validate_abilities(definition, part.id, part.rules.ability_clauses())?;
    }

    let structure_parts = structure_parts(definition)?;
    for part in &structure_parts {
        if !defined_parts.contains(part) {
            return Err(CatalogError::UndefinedStructurePart {
                definition: definition.id,
                part: *part,
            });
        }
    }
    for part in &definition.parts {
        if !structure_parts.contains(&part.id) {
            return Err(CatalogError::PartOutsideStructure {
                definition: definition.id,
                part: part.id,
            });
        }
    }

    let primary_part = definition.primary_part_id();
    if definition
        .part(primary_part)
        .is_some_and(|part| part.rules != definition.rules)
    {
        return Err(CatalogError::MismatchedPrimaryRules {
            definition: definition.id,
            part: primary_part,
        });
    }

    let mut play_options = HashSet::new();
    let mut additional_costs = HashSet::new();
    for option in &definition.play_options {
        if !play_options.insert(option.id) {
            return Err(CatalogError::DuplicatePlayOptionId {
                definition: definition.id,
                option: option.id,
            });
        }
        validate_spell_form(definition, option, &defined_parts, &structure_parts)?;
        validate_cost_ids(definition, option, &mut additional_costs)?;
        validate_modes_and_targets(definition, option)?;
        validate_semantic_spell_presentation(definition, option)?;
    }

    validate_alternative_cast_abilities(definition)?;
    validate_optional_additional_cost_abilities(definition)?;

    validate_fused_option(definition)
}
