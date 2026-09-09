// Validation of one ability clause, included into the parent validation module.

fn validate_ability_definition(
    ability: &AbilityDef,
    cost_bindings: &[crate::Binding],
) -> Result<(), GrantedAbilityValidationError> {
    let mut grant_sites = program_ability_grant_sites(ability.effect.definition);
    if let Some(modal) = ability.modal() {
        grant_sites = modal
            .modes
            .iter()
            .map(|mode| program_ability_grant_sites(mode.effect.definition))
            .fold(grant_sites, usize::saturating_add);
    }
    if grant_sites > usize::from(u8::MAX) + 1 {
        return Err(GrantedAbilityValidationError::TooManyGrantSites { count: grant_sites });
    }
    if ability.text.trim().is_empty() {
        return Err(GrantedAbilityValidationError::EmptyText);
    }
    validate_ability_coverage(ability)?;
    validate_ability_program(ability)?;
    let (source_zones, targets, is_mana_ability) = match &ability.definition {
        DeclarativeAbilityDef::Spell(spell) => (None, spell.targets(), false),
        DeclarativeAbilityDef::ActivatedMana(activated) => {
            (Some(activated.source_zones), activated.targets, true)
        }
        DeclarativeAbilityDef::TriggeredMana(triggered) => {
            (Some(triggered.source_zones), triggered.targets, true)
        }
        DeclarativeAbilityDef::Activated(activated) => {
            (Some(activated.source_zones), activated.targets, false)
        }
        DeclarativeAbilityDef::Triggered(triggered) => {
            (Some(triggered.source_zones), triggered.targets, false)
        }
        DeclarativeAbilityDef::Static(static_ability) => {
            (Some(static_ability.source_zones), &[][..], false)
        }
        DeclarativeAbilityDef::Replacement(replacement) => {
            (Some(replacement.source_zones), &[][..], false)
        }
        DeclarativeAbilityDef::SpecialAction(special_action) => {
            (Some(special_action.source_zones), &[][..], false)
        }
        DeclarativeAbilityDef::Pregame(_) => (Some(&[ZoneKind::Hand][..]), &[][..], false),
        DeclarativeAbilityDef::AlternativeCast(alternative) => (None, alternative.targets, false),
        DeclarativeAbilityDef::OptionalAdditionalCost(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::DeckConstruction(_) => (None, &[][..], false),
    };

    if source_zones.is_some_and(<[ZoneKind]>::is_empty) {
        return Err(GrantedAbilityValidationError::HasNoSourceZone);
    }
    if is_mana_ability && !targets.is_empty() {
        return Err(GrantedAbilityValidationError::ManaAbilityHasTargets);
    }
    if let DeclarativeAbilityDef::Activated(activated) = ability.definition {
        validate_activated_target_choosers(activated.targets)?;
        validate_ability_cost_target_references(activated.costs.as_slice(), targets)?;
    }
    validate_triggered_ability_shape(ability, targets.len())?;
    if let Err(problem) = validate_ability_effect_context(ability) {
        return Err(
            GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: problem.context,
                operation: problem.operation,
            },
        );
    }
    let trigger_event = match ability.definition {
        DeclarativeAbilityDef::TriggeredMana(definition)
        | DeclarativeAbilityDef::Triggered(definition) => Some(definition.event),
        DeclarativeAbilityDef::Spell(_)
        | DeclarativeAbilityDef::ActivatedMana(_)
        | DeclarativeAbilityDef::Activated(_)
        | DeclarativeAbilityDef::Static(_)
        | DeclarativeAbilityDef::Replacement(_)
        | DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::OptionalAdditionalCost(_)
        | DeclarativeAbilityDef::SpecialAction(_)
        | DeclarativeAbilityDef::Pregame(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::DeckConstruction(_) => None,
    };
    let chosen_cost_card_binding = match ability.definition {
        DeclarativeAbilityDef::Activated(definition) => {
            definition.costs.iter().find_map(|cost| match cost {
                CostDef::MoveToZone(movement) => movement.binding,
                _ => None,
            })
        }
        _ => None,
    };
    let replacement_event = match ability.definition {
        DeclarativeAbilityDef::Replacement(definition) => Some(definition.event),
        _ => None,
    };
    let condition = match ability.definition {
        DeclarativeAbilityDef::Triggered(definition)
        | DeclarativeAbilityDef::TriggeredMana(definition) => definition.condition,
        DeclarativeAbilityDef::Activated(definition)
        | DeclarativeAbilityDef::ActivatedMana(definition) => definition.condition,
        DeclarativeAbilityDef::AlternativeCast(definition) => definition.condition,
        _ => None,
    };
    validate_ability_program_targets(
        targets,
        ability.effect.definition,
        trigger_event,
        replacement_event,
        chosen_cost_card_binding,
        cost_bindings,
        condition,
    )?;
    Ok(())
}
