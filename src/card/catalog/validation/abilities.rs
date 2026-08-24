use std::collections::HashSet;

use super::program_context::validate_ability_effect_context;
use super::targeting::{validate_ability_program_targets, validate_ability_trigger_event};
use crate::card::catalog::{
    CatalogError, GrantedAbilityValidationError, MismatchedAdditionalCost,
    MismatchedAlternativeCost,
};
use crate::card::{
    AbilityDef, AbilityOperationDef, AbilityProcedureDef, AbilityProgramDef, AppliedEffectDef,
    CardDefinition, CharacteristicOperationDef, DeclarativeAbilityDef, EffectDef,
    EffectExecutionDef, EffectRecipientDef, EmblemCharacteristics, ImplementationStatus,
    ReplacementEffectDef, ReplacementEventDef, SpellForm, TokenCharacteristics, ZoneKind,
    ZoneMoveCauseDef,
};
use crate::{AbilityId, AdditionalCostId, AlternativeCostId, CardPartId, GrantId, ModeId};

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
                    return Err(CatalogError::MismatchedAlternativeCostForAbility(Box::new(
                        MismatchedAlternativeCost {
                            definition: definition.id,
                            part: part.id,
                            ability: attached.id,
                            option: option.id,
                            cost: expected.id,
                            expected_label: expected.label,
                            actual_label: actual.label.clone(),
                            expected_mana_cost: expected.mana_cost,
                            actual_mana_cost: actual.mana_cost,
                        },
                    )));
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

pub(super) fn validate_optional_additional_cost_abilities(
    definition: &CardDefinition,
) -> Result<(), CatalogError> {
    for part in &definition.parts {
        for attached in part.rules.indexed_abilities() {
            let DeclarativeAbilityDef::OptionalAdditionalCost(optional) =
                attached.definition.definition
            else {
                continue;
            };
            let cost = AdditionalCostId(attached.id.0);
            let mut owning_option_found = false;
            for option in definition.play_options.iter().filter(
                |option| matches!(option.form, SpellForm::Part(candidate) if candidate == part.id),
            ) {
                owning_option_found = true;
                let expected = optional.additional_cost(attached.id);
                let Some(actual) = option
                    .additional_costs
                    .iter()
                    .find(|cost| cost.id == expected.id)
                else {
                    return Err(CatalogError::MissingAdditionalCostForAbility {
                        definition: definition.id,
                        part: part.id,
                        ability: attached.id,
                        cost: expected.id,
                    });
                };
                if actual != &expected {
                    return Err(CatalogError::MismatchedAdditionalCostForAbility(Box::new(
                        MismatchedAdditionalCost {
                            definition: definition.id,
                            part: part.id,
                            ability: attached.id,
                            option: option.id,
                            cost: expected.id,
                            expected_label: expected.label,
                            actual_label: actual.label.clone(),
                            expected_mana_cost: expected.mana_cost,
                            actual_mana_cost: actual.mana_cost,
                        },
                    )));
                }
            }
            if !owning_option_found {
                return Err(CatalogError::MissingAdditionalCostForAbility {
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
    validate_abilities_with_created_virtuals(
        definition,
        part,
        abilities,
        &mut CreatedVirtualObjects::default(),
    )
}

#[derive(Default)]
struct CreatedVirtualObjects {
    tokens: HashSet<TokenCharacteristics>,
    emblems: HashSet<EmblemCharacteristics>,
}

fn validate_abilities_with_created_virtuals(
    definition: &CardDefinition,
    part: CardPartId,
    abilities: &[AbilityDef],
    created: &mut CreatedVirtualObjects,
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
        validate_attached_ability(definition, part, ability_id, ability, created)?;
    }
    Ok(())
}

fn validate_attached_ability(
    definition: &CardDefinition,
    part: CardPartId,
    ability_id: AbilityId,
    ability: &AbilityDef,
    created: &mut CreatedVirtualObjects,
) -> Result<(), CatalogError> {
    if let Err(problem) = validate_ability_definition(ability) {
        return Err(top_level_ability_error(
            definition, part, ability_id, &problem,
        ));
    }
    if let Some(modal) = ability.modal() {
        // An activated ability's own effect is the thing it does before its
        // modes; a modal spell prints nothing but its modes. Only the spell
        // is required to be empty.
        if matches!(ability.definition, DeclarativeAbilityDef::Spell(_))
            && (ability.coverage.status != ImplementationStatus::Complete
                || ability.effect.execution != EffectExecutionDef::Declarative
                || ability.effect.definition != AbilityProgramDef::Effects(EffectDef::None))
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
        // A conditional "you may choose two instead" has to be a real
        // increase and has to stay within the modes the card prints, or the
        // condition would offer a selection that cannot be made.
        let conditional_maximum = modal
            .conditional_maximum
            .map_or(modal.maximum, |conditional| conditional.maximum);
        if modal.modes.is_empty()
            || modal.minimum > modal.maximum
            || modal.maximum == 0
            || conditional_maximum < modal.maximum
            || (!modal.may_repeat && usize::from(conditional_maximum) > modal.modes.len())
        {
            return Err(CatalogError::InvalidModalSpellSelection {
                definition: definition.id,
                part,
                ability: ability_id,
                minimum: modal.minimum,
                maximum: conditional_maximum,
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
    validate_granted_abilities(
        definition,
        part,
        ability_id,
        ability,
        &mut Vec::new(),
        created,
    )
}

fn validate_granted_abilities(
    definition: &CardDefinition,
    part: CardPartId,
    outer_ability: AbilityId,
    ability: &AbilityDef,
    path: &mut Vec<GrantId>,
    created: &mut CreatedVirtualObjects,
) -> Result<(), CatalogError> {
    let mut grants = Vec::new();
    let mut tokens = Vec::new();
    let mut emblems = Vec::new();
    collect_direct_ability_contents(ability, &mut grants, &mut tokens, &mut emblems);
    for (index, granted) in grants.into_iter().enumerate() {
        let grant = GrantId::from_index(index)
            .expect("the containing ability's grant-site capacity was validated");
        path.push(grant);
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
        if let Err(problem) = validate_ability_definition(granted) {
            return Err(CatalogError::InvalidGrantedAbility {
                definition: definition.id,
                part,
                ability: outer_ability,
                grant_path: path.clone(),
                problem,
            });
        }
        validate_granted_abilities(definition, part, outer_ability, granted, path, created)?;
        path.pop();
    }
    for token in tokens {
        validate_created_token(definition, token, created)?;
    }
    for emblem in emblems {
        validate_created_emblem(definition, emblem, created)?;
    }
    Ok(())
}

fn validate_created_token(
    definition: &CardDefinition,
    token: TokenCharacteristics,
    created: &mut CreatedVirtualObjects,
) -> Result<(), CatalogError> {
    if !created.tokens.insert(token.semantic_identity()) {
        return Ok(());
    }
    let primary = token.primary_part();
    validate_created_token_part(definition, &primary, created)?;
    if let Some(back) = token
        .other_face(primary.id)
        .and_then(|part| token.part(part))
    {
        validate_created_token_part(definition, &back, created)?;
    }
    Ok(())
}

fn validate_created_token_part(
    definition: &CardDefinition,
    part: &crate::card::TokenPart,
    created: &mut CreatedVirtualObjects,
) -> Result<(), CatalogError> {
    let rules = part.rules();
    if let Some(explanation) = rules.coherence_error() {
        return Err(CatalogError::IncoherentCardRules {
            definition: definition.id,
            part: part.id,
            explanation,
        });
    }
    validate_abilities_with_created_virtuals(definition, part.id, rules.ability_clauses(), created)
}

fn validate_created_emblem(
    definition: &CardDefinition,
    emblem: EmblemCharacteristics,
    created: &mut CreatedVirtualObjects,
) -> Result<(), CatalogError> {
    if !created.emblems.insert(emblem) {
        return Ok(());
    }
    validate_abilities_with_created_virtuals(
        definition,
        CardPartId::PRIMARY,
        emblem.abilities(),
        created,
    )
}

/// Collects the grant sites owned directly by one ability clause. Modal spell
/// branches are part of their parent clause's effect tree, so their sites
/// continue the same [`GrantId`] sequence in printed mode order.
fn collect_direct_ability_contents<'a>(
    ability: &'a AbilityDef,
    grants: &mut Vec<&'a AbilityDef>,
    tokens: &mut Vec<TokenCharacteristics>,
    emblems: &mut Vec<EmblemCharacteristics>,
) {
    collect_program_ability_grants(ability.effect.definition, grants, tokens, emblems);
    if let Some(behavior) = ability.effect.custom_behavior() {
        tokens.extend(crate::card::tokens::custom_created_tokens(behavior));
    }
    if let Some(modal) = ability.modal() {
        for mode in modal.modes {
            collect_program_ability_grants(mode.effect.definition, grants, tokens, emblems);
            if let Some(behavior) = mode.effect.custom_behavior() {
                tokens.extend(crate::card::tokens::custom_created_tokens(behavior));
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
/// What an ability's own coverage line has to say about it: a compatibility
/// procedure needs a card-local resolver, and anything less than complete
/// declarative execution needs an explanation of why.
fn validate_ability_coverage(ability: &AbilityDef) -> Result<(), GrantedAbilityValidationError> {
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
        | DeclarativeAbilityDef::OptionalAdditionalCost(_)
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
    Ok(())
}

/// The shape checks a triggered ability answers: a discoverable zone claim,
/// an event the shared capture pass raises, and -- for a triggered mana
/// ability -- a program that finishes without stopping to ask.
fn validate_triggered_ability_shape(
    ability: &AbilityDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    let (DeclarativeAbilityDef::TriggeredMana(triggered)
    | DeclarativeAbilityDef::Triggered(triggered)) = ability.definition
    else {
        return Ok(());
    };
    let is_mana = matches!(ability.definition, DeclarativeAbilityDef::TriggeredMana(_));
    if triggered.procedure == AbilityProcedureDef::Shared
        && (!trigger_source_zones_are_discoverable(triggered.source_zones, triggered.event)
            || (triggered.event == crate::card::TriggerEventDef::StateCondition
                && triggered.condition.is_none())
            || (is_mana
                && (triggered.condition.is_some()
                    || !matches!(
                        triggered.event,
                        crate::card::TriggerEventDef::Tapped(matcher)
                            if matcher.purpose == crate::card::TapPurposeDef::Mana
                    ))))
    {
        return Err(GrantedAbilityValidationError::UnsupportedTriggerEvent {
            event: triggered.event,
        });
    }
    validate_ability_trigger_event(triggered.event, target_count)?;
    if triggered.procedure == AbilityProcedureDef::Shared
        && is_mana
        && !matches!(
            ability.effect.definition,
            AbilityProgramDef::Effects(effect)
                if triggered_mana_program_is_immediate(effect)
        )
    {
        return Err(GrantedAbilityValidationError::UnsupportedTriggeredManaProgram);
    }
    Ok(())
}

/// Whether a shared trigger's zone claim is one some capture walk can
/// actually find it from.
///
/// One zone is the ordinary case: a card is in one place, and the walk over
/// that place finds it. Both zones is admitted for exactly one clause --
/// "when this is put into a graveyard from anywhere" -- because that event
/// is the one no single walk sees: a permanent dying is captured off a
/// snapshot taken before it left the battlefield, when the graveyard walk
/// cannot see it yet, and a card discarded or milled is captured after it
/// lands, when the battlefield walk never held it. Every other event would
/// simply be found from whichever zone the card happened to be in, which
/// makes claiming both an authoring mistake rather than a listener.
fn trigger_source_zones_are_discoverable(
    source_zones: &[ZoneKind],
    event: crate::card::TriggerEventDef,
) -> bool {
    match source_zones {
        [ZoneKind::Battlefield | ZoneKind::Graveyard] => true,
        [ZoneKind::Battlefield, ZoneKind::Graveyard] => matches!(
            event,
            crate::card::TriggerEventDef::ZoneChanged(matcher)
                if matcher.from.is_none() && matcher.to == Some(ZoneKind::Graveyard)
        ),
        _ => false,
    }
}

fn validate_ability_definition(ability: &AbilityDef) -> Result<(), GrantedAbilityValidationError> {
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
        DeclarativeAbilityDef::AlternativeCast(alternative) => (None, alternative.targets, false),
        DeclarativeAbilityDef::OptionalAdditionalCost(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::Legacy => (None, &[][..], false),
    };

    if source_zones.is_some_and(<[ZoneKind]>::is_empty) {
        return Err(GrantedAbilityValidationError::HasNoSourceZone);
    }
    if is_mana_ability && !targets.is_empty() {
        return Err(GrantedAbilityValidationError::ManaAbilityHasTargets);
    }
    if ability.is_executable() {
        validate_triggered_ability_shape(ability, targets.len())?;
    }
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
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::Legacy => None,
    };
    validate_ability_program_targets(targets, ability.effect.definition, trigger_event)?;
    Ok(())
}

fn triggered_mana_program_is_immediate(effect: EffectDef) -> bool {
    match effect {
        EffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(triggered_mana_program_is_immediate)
        }
        EffectDef::AddMana(mana) => {
            matches!(mana.mana, crate::card::ManaSelectionDef::One(_)) && mana.amount > 0
        }
        _ => false,
    }
}

fn validate_ability_program(ability: &AbilityDef) -> Result<(), GrantedAbilityValidationError> {
    match (ability.definition, ability.effect.definition) {
        (
            DeclarativeAbilityDef::Replacement(definition),
            AbilityProgramDef::Replacement(effect),
        ) => {
            if ability.is_executable()
                && ability.effect.execution == EffectExecutionDef::Declarative
                && let Err(operation) =
                    validate_replacement_program_for_event(definition.event, effect)
            {
                return Err(
                    GrantedAbilityValidationError::UnsupportedReplacementProgram {
                        event: definition.event,
                        operation,
                    },
                );
            }
        }
        (DeclarativeAbilityDef::Replacement(_), AbilityProgramDef::Effects(_)) => {
            return Err(
                GrantedAbilityValidationError::ReplacementAbilityRequiresReplacementProgram,
            );
        }
        (_, AbilityProgramDef::Replacement(_)) => {
            return Err(
                GrantedAbilityValidationError::ReplacementProgramRequiresReplacementAbility,
            );
        }
        (_, AbilityProgramDef::Effects(_)) => {}
    }
    Ok(())
}

fn validate_replacement_program_for_event(
    event: ReplacementEventDef,
    effect: ReplacementEffectDef,
) -> Result<(), &'static str> {
    match event {
        ReplacementEventDef::SourceEntersBattlefield
        | ReplacementEventDef::ObjectEntersBattlefield { .. } => {
            validate_entry_replacement_program(effect)
        }
        ReplacementEventDef::WouldMove {
            from: Some(ZoneKind::Hand),
            to: ZoneKind::Graveyard,
            ..
        } if effect == ReplacementEffectDef::MoveToZone(ZoneKind::Battlefield) => Ok(()),
        // "From anywhere" replaces the same move wherever it starts, so it
        // is held to the same program as the battlefield exit it includes.
        ReplacementEventDef::WouldMove {
            from: None | Some(ZoneKind::Battlefield),
            to: ZoneKind::Graveyard,
            cause: ZoneMoveCauseDef::Any,
        } => validate_battlefield_exit_replacement_program(effect),
        ReplacementEventDef::WouldGainLife(_)
            if matches!(effect, ReplacementEffectDef::MultiplyEventAmount(_)) =>
        {
            Ok(())
        }
        ReplacementEventDef::WouldBeginTurn { .. } => {
            validate_begin_turn_replacement_program(effect)
        }
        ReplacementEventDef::WouldDraw { .. } => validate_draw_replacement_program(effect),
        ReplacementEventDef::AnyObjectWouldMove {
            to: ZoneKind::Graveyard,
            ..
        } if effect == ReplacementEffectDef::MoveToZone(ZoneKind::Exile) => Ok(()),
        ReplacementEventDef::WouldMove { .. }
        | ReplacementEventDef::WouldGainLife(_)
        | ReplacementEventDef::AnyObjectWouldMove { .. }
        | ReplacementEventDef::Special(_) => Err(replacement_operation_name(effect)),
    }
}

fn validate_draw_replacement_program(effect: ReplacementEffectDef) -> Result<(), &'static str> {
    // "You draw that many cards plus one instead" replaces the instruction
    // with a larger one rather than with instructions of its own, so it is a
    // whole program by itself.
    if matches!(effect, ReplacementEffectDef::AddToEventAmount(_)) {
        return Ok(());
    }
    let ReplacementEffectDef::Sequence(effects) = effect else {
        return Err(replacement_operation_name(effect));
    };
    let replaces_draw = effects
        .iter()
        .filter(|effect| **effect == ReplacementEffectDef::ReplaceEventWithNothing)
        .count();
    let performs_effect = effects
        .iter()
        .filter(|effect| matches!(effect, ReplacementEffectDef::Perform(_)))
        .count();
    if effects.len() == 2 && replaces_draw == 1 && performs_effect == 1 {
        Ok(())
    } else {
        Err("unsupported draw replacement sequence")
    }
}

fn validate_entry_replacement_program(effect: ReplacementEffectDef) -> Result<(), &'static str> {
    match effect {
        ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        // Sending the entering card somewhere else instead, which is how an
        // unpaid Mox Diamond reaches its owner's graveyard.
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::CopyEntering { .. } => Ok(()),
        // Adding to an amount is a draw's clause, not an entry's.
        ReplacementEffectDef::AddToEventAmount(_) => Err("AddToEventAmount"),
        ReplacementEffectDef::Sequence(effects) => {
            if effects.is_empty() {
                return Err("empty Sequence");
            }
            for effect in effects {
                validate_entry_replacement_program(*effect)?;
            }
            Ok(())
        }
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                validate_entry_replacement_program(*effect)?;
            }
            Ok(())
        }
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                validate_entry_replacement_program(*effect)?;
            }
            Ok(())
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::Perform(_)
        | ReplacementEffectDef::MultiplyEventAmount(_) => Err(replacement_operation_name(effect)),
    }
}

fn validate_begin_turn_replacement_program(
    effect: ReplacementEffectDef,
) -> Result<(), &'static str> {
    match effect {
        ReplacementEffectDef::ReplaceEventWithNothing => Ok(()),
        ReplacementEffectDef::Perform(effect)
            if matches!(
                *effect,
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                }
            ) =>
        {
            Ok(())
        }
        ReplacementEffectDef::Sequence(effects) => {
            if effects.is_empty() {
                return Err("empty Sequence");
            }
            for effect in effects {
                validate_begin_turn_replacement_program(*effect)?;
            }
            if !effects
                .iter()
                .any(|effect| matches!(effect, ReplacementEffectDef::ReplaceEventWithNothing))
            {
                return Err("Sequence without ReplaceEventWithNothing");
            }
            Ok(())
        }
        ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::Perform(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. }
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::PayOr { .. } => Err(replacement_operation_name(effect)),
    }
}

fn validate_battlefield_exit_replacement_program(
    effect: ReplacementEffectDef,
) -> Result<(), &'static str> {
    match effect {
        // Exile and library are the two destinations that answer "instead":
        // one takes the card out of the game, the other puts it back where
        // it came from.
        ReplacementEffectDef::MoveToZone(ZoneKind::Exile | ZoneKind::Library) => Ok(()),
        ReplacementEffectDef::Perform(effect)
            if matches!(
                *effect,
                EffectDef::TakeExtraTurn {
                    player: EffectRecipientDef::Controller,
                } | EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                }
            ) =>
        {
            Ok(())
        }
        ReplacementEffectDef::Sequence(effects) => {
            if effects.is_empty() {
                return Err("empty Sequence");
            }
            for effect in effects {
                validate_battlefield_exit_replacement_program(*effect)?;
            }
            if !effects
                .iter()
                .any(|effect| matches!(effect, ReplacementEffectDef::MoveToZone(_)))
            {
                return Err("Sequence without MoveToZone");
            }
            Ok(())
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::Perform(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. }
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::PayOr { .. } => Err(replacement_operation_name(effect)),
    }
}

const fn replacement_operation_name(effect: ReplacementEffectDef) -> &'static str {
    match effect {
        ReplacementEffectDef::Sequence(_) => "Sequence",
        ReplacementEffectDef::ReplaceEventWithNothing => "ReplaceEventWithNothing",
        ReplacementEffectDef::MoveToZone(_) => "MoveToZone",
        ReplacementEffectDef::Perform(_) => "Perform",
        ReplacementEffectDef::ModifyBattlefieldEntry(_) => "ModifyBattlefieldEntry",
        ReplacementEffectDef::MultiplyEventAmount(_) => "MultiplyEventAmount",
        ReplacementEffectDef::AddToEventAmount(_) => "AddToEventAmount",
        ReplacementEffectDef::Choose(_) => "Choose",
        ReplacementEffectDef::LookAtHand(_) => "LookAtHand",
        ReplacementEffectDef::CopyEntering { .. } => "CopyEntering",
        ReplacementEffectDef::Conditional { .. } => "Conditional",
        ReplacementEffectDef::PayOr { .. } => "PayOr",
    }
}

// Mapping internal validation failures onto card/catalog identity is kept
// separate from the recursive ability-program walk below.
include!("abilities/top_level_errors.rs");

// Walking a definition for the abilities it grants, and counting where it
// grants them. Kept beside the validation above rather than in it: the
// walk is one arm per effect variant and says nothing about validity.
include!("abilities/ability_grants.rs");

#[cfg(test)]
mod custom_token_tests {
    use super::*;

    #[test]
    fn tetravite_enters_the_creator_owned_validation_walk() {
        let catalog = crate::poc::catalog().expect("the catalog builds");
        let definition = catalog
            .get(crate::card::cards::TETRAVUS)
            .expect("Tetravus is cataloged");
        let creator = definition
            .part(CardPartId::PRIMARY)
            .expect("Tetravus has its primary part")
            .rules
            .ability_clauses()
            .iter()
            .find(|ability| {
                ability.effect.custom_behavior() == Some(crate::card::CardBehavior::TetravusDetach)
            })
            .expect("Tetravus has its detach creator");
        let mut grants = Vec::new();
        let mut tokens = Vec::new();
        let mut emblems = Vec::new();
        collect_direct_ability_contents(creator, &mut grants, &mut tokens, &mut emblems);

        assert!(tokens.iter().any(|token| token.semantic_identity()
            == crate::card::tokens::tetravite().semantic_identity()));
        let mut validated = CreatedVirtualObjects::default();
        for token in tokens {
            validate_created_token(definition, token, &mut validated)
                .expect("the registered Tetravite receives catalog validation");
        }
    }

    #[test]
    fn creator_owned_emblems_enter_recursive_validation() {
        static INVALID_EMBLEM: EmblemCharacteristics = EmblemCharacteristics::new(
            "Invalid emblem",
            &[
                AbilityDef::spell("First spell.", EffectDef::None),
                AbilityDef::spell("Second spell.", EffectDef::None),
            ],
        );

        let catalog = crate::poc::catalog().expect("the catalog builds");
        let definition = catalog
            .get(crate::card::cards::DOMRI_RADE)
            .expect("Domri is cataloged");
        let creator = definition
            .part(CardPartId::PRIMARY)
            .and_then(|part| part.rules.ability(AbilityId(2)))
            .expect("Domri has an emblem-creating ultimate");
        let mut grants = Vec::new();
        let mut tokens = Vec::new();
        let mut emblems = Vec::new();
        collect_direct_ability_contents(creator, &mut grants, &mut tokens, &mut emblems);
        assert!(
            emblems
                .iter()
                .any(|emblem| emblem.name() == "Domri Rade emblem"),
            "the creator-owned emblem is part of the recursive validation walk",
        );
        let error = validate_created_emblem(
            definition,
            INVALID_EMBLEM,
            &mut CreatedVirtualObjects::default(),
        )
        .expect_err("invalid emblem-owned abilities are rejected");
        assert!(matches!(error, CatalogError::MultipleSpellAbilities { .. }));
    }
}
