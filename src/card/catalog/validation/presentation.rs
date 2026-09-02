use crate::card::catalog::CatalogError;
use crate::card::{
    AbilityTargetDef, CardDefinition, CardEffectStatus, DeclarativeAbilityDef, PlayActionKind,
    PlayOptionDef, SpellForm,
};
use crate::{ModeId, TargetSlotId};

#[allow(clippy::too_many_lines)]
pub(in crate::card::catalog) fn validate_semantic_spell_presentation(
    definition: &CardDefinition,
    option: &PlayOptionDef,
) -> Result<(), CatalogError> {
    if option.action != PlayActionKind::CastSpell {
        return Ok(());
    }
    if let SpellForm::Combined(parts) = &option.form {
        let mut semantic_targets = Vec::new();
        let mut any_executable = false;
        for part_id in parts {
            let Some(part) = definition.part(*part_id) else {
                return Ok(());
            };
            let Some(ability) = part
                .rules
                .ability_clauses()
                .iter()
                .find(|ability| matches!(ability.definition, DeclarativeAbilityDef::Spell(_)))
            else {
                // A legacy combined form can continue to own its presentation
                // targets until every constituent part has a semantic spell
                // clause to derive them from.
                return Ok(());
            };
            let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                unreachable!("the selected ability was checked as a spell")
            };
            if spell.modal().is_some() {
                return Err(CatalogError::CombinedModalSpellUnsupported {
                    definition: definition.id,
                    option: option.id,
                    part: *part_id,
                });
            }
            any_executable |= true;
            semantic_targets.extend_from_slice(spell.targets());
        }
        if any_executable && option.modes.is_some() {
            return Err(CatalogError::UnexpectedPresentationSpellModes {
                definition: definition.id,
                option: option.id,
            });
        }
        return validate_nonmodal_spell_targets(definition, option, &semantic_targets);
    }
    let SpellForm::Part(part_id) = &option.form else {
        unreachable!("combined spell forms returned above")
    };
    let Some(part) = definition.part(*part_id) else {
        return Ok(());
    };
    let Some(ability) = part
        .rules
        .ability_clauses()
        .iter()
        .find(|ability| matches!(ability.definition, DeclarativeAbilityDef::Spell(_)))
    else {
        return Ok(());
    };
    let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
        unreachable!("the selected ability was checked as a spell")
    };
    let Some(modal) = spell.modal() else {
        if option.modes.is_some() {
            return Err(CatalogError::UnexpectedPresentationSpellModes {
                definition: definition.id,
                option: option.id,
            });
        }
        return validate_nonmodal_spell_targets(definition, option, spell.targets());
    };
    if !option.targets.is_empty() {
        return Err(CatalogError::UnexpectedModalSpellTargets {
            definition: definition.id,
            option: option.id,
            count: option.targets.len(),
        });
    }
    let semantic_modes = modal.modes;

    let presentation_modes = option.modes.as_ref();
    for (index, _semantic) in semantic_modes.iter().enumerate() {
        let mode = ModeId::from_index(index)
            .expect("attached ability validation limits positional spell mode IDs");
        if presentation_modes
            .is_none_or(|modes| !modes.modes.iter().any(|candidate| candidate.id == mode))
        {
            return Err(CatalogError::MissingPresentationSpellMode {
                definition: definition.id,
                option: option.id,
                mode,
            });
        }
    }
    let Some(presentation_modes) = presentation_modes else {
        unreachable!("every semantic mode found a presentation mode")
    };
    if (
        presentation_modes.minimum,
        presentation_modes.maximum,
        presentation_modes.may_repeat,
    ) != (modal.minimum, modal.maximum, modal.may_repeat)
    {
        return Err(CatalogError::MismatchedSpellModeSelection {
            definition: definition.id,
            option: option.id,
            presentation_minimum: presentation_modes.minimum,
            presentation_maximum: presentation_modes.maximum,
            presentation_may_repeat: presentation_modes.may_repeat,
            semantic_minimum: modal.minimum,
            semantic_maximum: modal.maximum,
            semantic_may_repeat: modal.may_repeat,
        });
    }
    for presentation in &presentation_modes.modes {
        let Some(semantic) = semantic_modes.get(presentation.id.index()) else {
            return Err(CatalogError::MissingSemanticSpellMode {
                definition: definition.id,
                option: option.id,
                mode: presentation.id,
            });
        };
        let DeclarativeAbilityDef::Spell(semantic_spell) = semantic.definition else {
            unreachable!("attached ability validation requires plain spell modes")
        };
        if presentation.label != semantic.text {
            return Err(CatalogError::MismatchedSpellModeLabel {
                definition: definition.id,
                option: option.id,
                mode: presentation.id,
                presentation: presentation.label.clone(),
                semantic: semantic.text,
            });
        }
        let semantic_additional_mana_cost = modal.mode_additional_mana_cost(presentation.id);
        if presentation.additional_mana_cost != semantic_additional_mana_cost {
            return Err(CatalogError::MismatchedSpellModeAdditionalManaCost {
                definition: definition.id,
                option: option.id,
                mode: presentation.id,
                presentation: Box::new(presentation.additional_mana_cost),
                semantic: Box::new(semantic_additional_mana_cost),
            });
        }
        let expected_status = CardEffectStatus::Implemented;
        if presentation.effect_status != expected_status {
            return Err(CatalogError::MismatchedSpellModeImplementation {
                definition: definition.id,
                option: option.id,
                mode: presentation.id,
                presentation: presentation.effect_status,
                semantic: expected_status,
            });
        }
        let mut projected = Vec::with_capacity(semantic_spell.targets().len());
        let mut unpresentable = None;
        for (position, semantic_target) in semantic_spell.targets().iter().enumerate() {
            let target = TargetSlotId::from_index(position)
                .expect("ability target validation limits positional target IDs");
            let Some(expected) = semantic_target.presentation(target) else {
                unpresentable = Some(target);
                break;
            };
            projected.push(expected);
        }
        if let Some(target) = unpresentable {
            if presentation.targets.is_empty() {
                // Runtime mode targeting uses the richer semantic predicate.
                // Keep the legacy projection empty instead of accepting an
                // incomplete approximation.
                continue;
            }
            return Err(CatalogError::UnpresentableSpellModeTarget {
                definition: definition.id,
                option: option.id,
                mode: presentation.id,
                target,
            });
        }
        for (position, (semantic_target, presentation_target)) in semantic_spell
            .targets()
            .iter()
            .zip(&presentation.targets)
            .enumerate()
        {
            let target = TargetSlotId::from_index(position)
                .expect("ability target validation limits positional target IDs");
            let expected = &projected[position];
            if presentation_target.id != expected.id {
                return Err(CatalogError::MismatchedSpellModeTargetPresentation {
                    definition: definition.id,
                    option: option.id,
                    mode: presentation.id,
                    position,
                    presentation: presentation_target.clone(),
                    semantic: expected.clone(),
                });
            }
            if (presentation_target.minimum, presentation_target.maximum)
                != (semantic_target.minimum, semantic_target.maximum)
            {
                return Err(CatalogError::MismatchedSpellModeTargetCardinality {
                    definition: definition.id,
                    option: option.id,
                    mode: presentation.id,
                    target,
                    presentation_minimum: presentation_target.minimum,
                    presentation_maximum: presentation_target.maximum,
                    semantic_minimum: semantic_target.minimum,
                    semantic_maximum: semantic_target.maximum,
                });
            }
            if (
                presentation_target.label.as_str(),
                presentation_target.predicate,
            ) != (expected.label.as_str(), expected.predicate)
            {
                return Err(CatalogError::MismatchedSpellModeTargetPresentation {
                    definition: definition.id,
                    option: option.id,
                    mode: presentation.id,
                    position,
                    presentation: presentation_target.clone(),
                    semantic: expected.clone(),
                });
            }
        }
        if semantic_spell
            .targets()
            .get(presentation.targets.len())
            .is_some()
        {
            let target = TargetSlotId::from_index(presentation.targets.len())
                .expect("ability target validation limits positional target IDs");
            return Err(CatalogError::MissingPresentationSpellModeTarget {
                definition: definition.id,
                option: option.id,
                mode: presentation.id,
                target,
            });
        }
        if let Some(presentation_target) = presentation.targets.get(semantic_spell.targets().len())
        {
            return Err(CatalogError::MissingSemanticSpellModeTarget {
                definition: definition.id,
                option: option.id,
                mode: presentation.id,
                target: presentation_target.id,
            });
        }
    }
    Ok(())
}

fn validate_nonmodal_spell_targets(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    semantic_targets: &[AbilityTargetDef],
) -> Result<(), CatalogError> {
    if semantic_targets.len() > usize::from(u8::MAX) + 1 {
        return Err(CatalogError::TooManyInstantiatedTargets {
            definition: definition.id,
            option: option.id,
            count: semantic_targets.len(),
        });
    }
    let mut projected = Vec::with_capacity(semantic_targets.len());
    for (position, semantic_target) in semantic_targets.iter().enumerate() {
        let target = TargetSlotId::from_index(position)
            .expect("semantic target validation limits positional target IDs");
        let Some(expected) = semantic_target.presentation(target) else {
            return if option.targets.is_empty() {
                // Runtime target generation uses the richer semantic
                // predicate directly. Keep the legacy presentation empty
                // rather than accepting an approximation that can drift.
                Ok(())
            } else {
                Err(CatalogError::UnpresentableSpellTarget {
                    definition: definition.id,
                    option: option.id,
                    target,
                })
            };
        };
        projected.push(expected);
    }
    for (position, (semantic_target, presentation_target)) in
        semantic_targets.iter().zip(&option.targets).enumerate()
    {
        let target = TargetSlotId::from_index(position)
            .expect("semantic target validation limits positional target IDs");
        let expected = &projected[position];
        if (presentation_target.minimum, presentation_target.maximum)
            != (semantic_target.minimum, semantic_target.maximum)
        {
            return Err(CatalogError::MismatchedSpellTargetCardinality {
                definition: definition.id,
                option: option.id,
                target,
                presentation_minimum: presentation_target.minimum,
                presentation_maximum: presentation_target.maximum,
                semantic_minimum: semantic_target.minimum,
                semantic_maximum: semantic_target.maximum,
            });
        }
        if presentation_target != expected {
            return Err(CatalogError::MismatchedSpellTargetPresentation {
                definition: definition.id,
                option: option.id,
                position,
                presentation: presentation_target.clone(),
                semantic: expected.clone(),
            });
        }
    }
    if semantic_targets.get(option.targets.len()).is_some() {
        let target = TargetSlotId::from_index(option.targets.len())
            .expect("semantic target validation limits positional target IDs");
        return Err(CatalogError::MissingPresentationSpellTarget {
            definition: definition.id,
            option: option.id,
            target,
        });
    }
    if let Some(presentation_target) = option.targets.get(semantic_targets.len()) {
        return Err(CatalogError::MissingSemanticSpellTarget {
            definition: definition.id,
            option: option.id,
            target: presentation_target.id,
        });
    }
    Ok(())
}
