use super::targeting::validate_ability_targets;
use crate::card::catalog::{CatalogError, GrantedAbilityValidationError};
use crate::card::{
    AbilityDef, AbilityProcedureDef, AppliedEffectDef, CardDefinition, DeclarativeAbilityDef,
    EffectDef, EffectExecutionDef, ImplementationStatus, SpellForm, ZoneKind,
};
use crate::{AbilityId, AlternativeCostId, CardPartId, GrantId, ModeId};

pub(super) fn validate_alternative_cast_abilities(
    definition: &CardDefinition,
) -> Result<(), CatalogError> {
    for part in &definition.parts {
        for attached in part.rules.indexed_abilities() {
            let DeclarativeAbilityDef::AlternativeCast(alternative_cast) =
                attached.definition.definition
            else {
                continue;
            };
            let cost = AlternativeCostId(attached.id.0);
            let mut owning_option_found = false;
            for option in definition.play_options.iter().filter(
                |option| matches!(option.form, SpellForm::Part(candidate) if candidate == part.id),
            ) {
                owning_option_found = true;
                let Some(expected) =
                    alternative_cast.alternative_cost(attached.id, option.mana_cost)
                else {
                    return Err(CatalogError::MissingAlternativeCostForAbility {
                        definition: definition.id,
                        part: part.id,
                        ability: attached.id,
                        cost,
                    });
                };
                let Some(actual) = option
                    .alternative_costs
                    .iter()
                    .find(|cost| cost.id == expected.id)
                else {
                    return Err(CatalogError::MissingAlternativeCostForAbility {
                        definition: definition.id,
                        part: part.id,
                        ability: attached.id,
                        cost: expected.id,
                    });
                };
                if actual != &expected {
                    return Err(CatalogError::MismatchedAlternativeCostForAbility {
                        definition: definition.id,
                        part: part.id,
                        ability: attached.id,
                        option: option.id,
                        cost: expected.id,
                        expected_label: expected.label,
                        actual_label: actual.label.clone(),
                        expected_mana_cost: expected.mana_cost,
                        actual_mana_cost: actual.mana_cost,
                    });
                }
            }
            if !owning_option_found {
                return Err(CatalogError::MissingAlternativeCostForAbility {
                    definition: definition.id,
                    part: part.id,
                    ability: attached.id,
                    cost,
                });
            }
        }
    }
    Ok(())
}

pub(super) fn validate_abilities(
    definition: &CardDefinition,
    part: CardPartId,
    abilities: &[AbilityDef],
) -> Result<(), CatalogError> {
    if abilities.len() > usize::from(u8::MAX) + 1 {
        return Err(CatalogError::TooManyAbilities {
            definition: definition.id,
            part,
            count: abilities.len(),
        });
    }
    let spell_count = abilities
        .iter()
        .filter(|ability| matches!(ability.definition, DeclarativeAbilityDef::Spell(_)))
        .count();
    if spell_count > 1 {
        return Err(CatalogError::MultipleSpellAbilities {
            definition: definition.id,
            part,
            count: spell_count,
        });
    }

    for (index, ability) in abilities.iter().enumerate() {
        let ability_id = AbilityId::from_index(index)
            .expect("the ability count was validated before assigning positional IDs");
        validate_attached_ability(definition, part, ability_id, ability)?;
    }
    Ok(())
}

fn validate_attached_ability(
    definition: &CardDefinition,
    part: CardPartId,
    ability_id: AbilityId,
    ability: &AbilityDef,
) -> Result<(), CatalogError> {
    if let Err(problem) = validate_ability_definition(ability) {
        return Err(top_level_ability_error(
            definition, part, ability_id, &problem,
        ));
    }
    if let DeclarativeAbilityDef::Spell(spell) = ability.definition
        && let Some(modal) = spell.modal()
    {
        if ability.coverage.status != ImplementationStatus::Complete
            || ability.effect.execution != EffectExecutionDef::Declarative
            || ability.effect.definition != EffectDef::None
        {
            return Err(CatalogError::InvalidModalSpellParent {
                definition: definition.id,
                part,
                ability: ability_id,
            });
        }
        if modal.modes.len() > usize::from(u8::MAX) + 1 {
            return Err(CatalogError::TooManySpellModes {
                definition: definition.id,
                part,
                ability: ability_id,
                count: modal.modes.len(),
            });
        }
        if modal.modes.is_empty()
            || modal.minimum > modal.maximum
            || modal.maximum == 0
            || (!modal.may_repeat && usize::from(modal.maximum) > modal.modes.len())
        {
            return Err(CatalogError::InvalidModalSpellSelection {
                definition: definition.id,
                part,
                ability: ability_id,
                minimum: modal.minimum,
                maximum: modal.maximum,
                may_repeat: modal.may_repeat,
                available: modal.modes.len(),
            });
        }
        for (index, mode) in modal.modes.iter().enumerate() {
            let mode_id = ModeId::from_index(index)
                .expect("the spell mode count was validated before assigning positional IDs");
            let DeclarativeAbilityDef::Spell(mode_spell) = mode.definition else {
                return Err(CatalogError::NonSpellMode {
                    definition: definition.id,
                    part,
                    ability: ability_id,
                    mode: mode_id,
                });
            };
            if mode_spell.modal().is_some() {
                return Err(CatalogError::NestedModalSpellMode {
                    definition: definition.id,
                    part,
                    ability: ability_id,
                    mode: mode_id,
                });
            }
            if mode.is_executable() && mode.declarative_effect().is_none() {
                return Err(CatalogError::CustomSpellModeImplementation {
                    definition: definition.id,
                    part,
                    ability: ability_id,
                    mode: mode_id,
                });
            }
            if let Err(problem) = validate_ability_definition(mode) {
                return Err(CatalogError::InvalidSpellMode {
                    definition: definition.id,
                    part,
                    ability: ability_id,
                    mode: mode_id,
                    problem,
                });
            }
        }
    }
    validate_granted_abilities(definition, part, ability_id, ability, &mut Vec::new())
}

fn validate_granted_abilities(
    definition: &CardDefinition,
    part: CardPartId,
    outer_ability: AbilityId,
    ability: &AbilityDef,
    path: &mut Vec<GrantId>,
) -> Result<(), CatalogError> {
    let mut grants = Vec::new();
    collect_direct_ability_grants(ability, &mut grants);
    for (index, granted) in grants.into_iter().enumerate() {
        let grant = GrantId::from_index(index)
            .expect("the containing ability's grant-site capacity was validated");
        path.push(grant);
        if let Err(problem) = validate_ability_definition(granted) {
            return Err(CatalogError::InvalidGrantedAbility {
                definition: definition.id,
                part,
                ability: outer_ability,
                grant_path: path.clone(),
                problem,
            });
        }
        if granted.is_executable() && matches!(granted.definition, DeclarativeAbilityDef::Static(_))
        {
            return Err(CatalogError::InvalidGrantedAbility {
                definition: definition.id,
                part,
                ability: outer_ability,
                grant_path: path.clone(),
                problem: GrantedAbilityValidationError::ExecutableStaticAbility,
            });
        }
        validate_granted_abilities(definition, part, outer_ability, granted, path)?;
        path.pop();
    }
    Ok(())
}

/// Collects the grant sites owned directly by one ability clause. Modal spell
/// branches are part of their parent clause's effect tree, so their sites
/// continue the same [`GrantId`] sequence in printed mode order.
fn collect_direct_ability_grants<'a>(ability: &'a AbilityDef, grants: &mut Vec<&'a AbilityDef>) {
    collect_ability_grants(ability.effect.definition, grants);
    if let DeclarativeAbilityDef::Spell(spell) = ability.definition
        && let Some(modal) = spell.modal()
    {
        for mode in modal.modes {
            collect_ability_grants(mode.effect.definition, grants);
        }
    }
}

fn validate_ability_definition(ability: &AbilityDef) -> Result<(), GrantedAbilityValidationError> {
    let mut grant_sites = ability_grant_sites(ability.effect.definition);
    if let DeclarativeAbilityDef::Spell(spell) = ability.definition
        && let Some(modal) = spell.modal()
    {
        grant_sites = modal
            .modes
            .iter()
            .map(|mode| ability_grant_sites(mode.effect.definition))
            .fold(grant_sites, usize::saturating_add);
    }
    if grant_sites > usize::from(u8::MAX) + 1 {
        return Err(GrantedAbilityValidationError::TooManyGrantSites { count: grant_sites });
    }
    if ability.text.trim().is_empty() {
        return Err(GrantedAbilityValidationError::EmptyText);
    }
    let uses_legacy_procedure = match ability.definition {
        DeclarativeAbilityDef::ActivatedMana(definition)
        | DeclarativeAbilityDef::Activated(definition) => {
            definition.procedure == AbilityProcedureDef::Legacy
        }
        DeclarativeAbilityDef::TriggeredMana(definition)
        | DeclarativeAbilityDef::Triggered(definition) => {
            definition.procedure == AbilityProcedureDef::Legacy
        }
        DeclarativeAbilityDef::Spell(_)
        | DeclarativeAbilityDef::Static(_)
        | DeclarativeAbilityDef::Replacement(_)
        | DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::SpecialAction(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::Legacy => false,
    };
    if ability.is_executable()
        && uses_legacy_procedure
        && !matches!(ability.effect.execution, EffectExecutionDef::Custom(_))
    {
        return Err(GrantedAbilityValidationError::LegacyProcedureRequiresCustomExecution);
    }
    let explanation = ability.coverage.explanation;
    let explanation_required = ability.coverage.status != ImplementationStatus::Complete
        || ability.effect.execution != EffectExecutionDef::Declarative
        || uses_legacy_procedure;
    if explanation.is_some_and(|explanation| explanation.trim().is_empty())
        || (explanation_required && explanation.is_none())
    {
        return Err(GrantedAbilityValidationError::MissingImplementationExplanation);
    }
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
        DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::Legacy => (None, &[][..], false),
    };

    if source_zones.is_some_and(<[ZoneKind]>::is_empty) {
        return Err(GrantedAbilityValidationError::HasNoSourceZone);
    }
    if is_mana_ability && !targets.is_empty() {
        return Err(GrantedAbilityValidationError::ManaAbilityHasTargets);
    }
    validate_ability_targets(targets, ability.effect.definition)?;
    Ok(())
}

fn top_level_ability_error(
    definition: &CardDefinition,
    part: CardPartId,
    ability: AbilityId,
    problem: &GrantedAbilityValidationError,
) -> CatalogError {
    match problem {
        GrantedAbilityValidationError::TooManyGrantSites { count } => {
            CatalogError::TooManyAbilityGrantSites {
                definition: definition.id,
                part,
                ability,
                count: *count,
            }
        }
        GrantedAbilityValidationError::EmptyText => CatalogError::EmptyAbilityText {
            definition: definition.id,
            part,
            ability,
        },
        GrantedAbilityValidationError::MissingImplementationExplanation => {
            CatalogError::MissingImplementationExplanation {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::LegacyProcedureRequiresCustomExecution => {
            CatalogError::LegacyProcedureRequiresCustomExecution {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::HasNoSourceZone => CatalogError::AbilityHasNoSourceZone {
            definition: definition.id,
            part,
            ability,
        },
        GrantedAbilityValidationError::ManaAbilityHasTargets => {
            CatalogError::ManaAbilityHasTargets {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::TooManyTargets { count } => {
            CatalogError::TooManyAbilityTargets {
                definition: definition.id,
                part,
                ability,
                count: *count,
            }
        }
        GrantedAbilityValidationError::InvalidTargetBounds {
            target,
            minimum,
            maximum,
        } => CatalogError::InvalidAbilityTargetBounds {
            definition: definition.id,
            part,
            ability,
            target: *target,
            minimum: *minimum,
            maximum: *maximum,
        },
        GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target,
            target_count,
        } => CatalogError::AbilityTargetReferenceOutOfBounds {
            definition: definition.id,
            part,
            ability,
            target: *target,
            target_count: *target_count,
        },
        GrantedAbilityValidationError::ExecutableStaticAbility => {
            unreachable!("only granted static abilities are rejected")
        }
    }
}

fn collect_ability_grants(effect: EffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                collect_ability_grants(*effect, grants);
            }
        }
        EffectDef::OptionalManaPayment { effect, .. }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | EffectDef::May(effect)
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::AtNextStep { effect, .. } => {
            collect_ability_grants(*effect, grants);
        }
        EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => collect_ability_grants(*effect, grants),
        EffectDef::Apply { effect, .. } => collect_applied_ability_grants(effect, grants),
        EffectDef::TriggerUntilYourNextTurn { .. }
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Tap { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
        | EffectDef::Attach { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { then: None, .. }
        | EffectDef::DestroyOfChoice { .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchLibrary { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CounterUnlessPaid { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::AdditionalCombatPhase
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MakeUnblockableThisTurn { .. }
        | EffectDef::GainControlThisTurn { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::MoveToZone { .. }
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::Special(_) => {}
    }
}

fn collect_applied_ability_grants(effect: AppliedEffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                collect_applied_ability_grants(*effect, grants);
            }
        }
        AppliedEffectDef::GrantAbility(ability) => grants.push(ability),
        AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::Animate(_)
        | AppliedEffectDef::ModifyPowerToughness { .. }
        | AppliedEffectDef::Special(_) => {}
    }
}

fn ability_grant_sites(effect: EffectDef) -> usize {
    match effect {
        EffectDef::Sequence(effects) => effects
            .iter()
            .map(|effect| ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        EffectDef::OptionalManaPayment { effect, .. }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | EffectDef::May(effect)
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::AtNextStep { effect, .. }
        | EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => ability_grant_sites(*effect),
        EffectDef::Apply { effect, .. } => applied_ability_grant_sites(effect),
        EffectDef::TriggerUntilYourNextTurn { .. }
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Tap { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
        | EffectDef::Attach { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { then: None, .. }
        | EffectDef::DestroyOfChoice { .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchLibrary { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CounterUnlessPaid { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::AdditionalCombatPhase
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MakeUnblockableThisTurn { .. }
        | EffectDef::GainControlThisTurn { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::MoveToZone { .. }
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::Special(_) => 0,
    }
}

fn applied_ability_grant_sites(effect: AppliedEffectDef) -> usize {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .map(|effect| applied_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        AppliedEffectDef::GrantAbility(_) => 1,
        AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::Animate(_)
        | AppliedEffectDef::ModifyPowerToughness { .. }
        | AppliedEffectDef::Special(_) => 0,
    }
}
