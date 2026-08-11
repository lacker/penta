use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use super::{
    AbilityDef, AbilityProcedureDef, AbilityTargetDef, AppliedEffectDef, CardDefinition,
    CardEffectStatus, CardPrinting, CardPrintingId, CardSet, CardStructure, DeclarativeAbilityDef,
    EffectDef, EffectExecutionDef, EffectRecipientDef, ImplementationStatus, ManaCost, ModeSetDef,
    PlayActionKind, PlayOptionDef, SpellForm, TargetSlotDef, ValueDef,
};
use crate::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, Format, GrantId,
    ModeId, PlayOptionId, TargetIndex, TargetSlotId,
};

/// A catalog is immutable once built, and callers pass it around by value —
/// a game, a policy, and the protocol facade each hold one. Sharing the maps
/// behind an `Arc` makes those clones a refcount bump instead of a deep copy
/// of every definition.
#[derive(Clone, Debug, Default)]
pub struct CardCatalog {
    entries: Arc<CatalogEntries>,
}

#[derive(Debug, Default)]
struct CatalogEntries {
    definitions: Vec<CardDefinition>,
    definition_indices: Vec<Option<usize>>,
    ids_by_name: HashMap<String, CardDefinitionId>,
    definition_by_printing: HashMap<CardPrintingId, CardDefinitionId>,
}

impl CatalogEntries {
    fn definition_index(&self, definition: CardDefinitionId) -> Option<usize> {
        self.definition_indices
            .get(usize::from(definition.0))
            .copied()
            .flatten()
    }

    fn definition(&self, definition: CardDefinitionId) -> Option<&CardDefinition> {
        self.definitions.get(self.definition_index(definition)?)
    }

    fn insert_definition(&mut self, definition: CardDefinition) {
        let slot = usize::from(definition.id.0);
        if self.definition_indices.len() <= slot {
            self.definition_indices.resize(slot + 1, None);
        }
        let index = self.definitions.len();
        self.definitions.push(definition);
        self.definition_indices[slot] = Some(index);
    }

    fn attach_printing(&mut self, printing: CardPrinting) -> Result<(), CatalogError> {
        let definition = printing.id.definition;
        let Some(index) = self.definition_index(definition) else {
            return Err(CatalogError::OrphanPrinting(printing.id));
        };
        if self.definition_by_printing.contains_key(&printing.id) {
            return Err(CatalogError::DuplicatePrintingId(printing.id));
        }

        self.definition_by_printing.insert(printing.id, definition);
        self.definitions[index].printings.push(printing);
        Ok(())
    }
}

impl CardCatalog {
    /// Builds a catalog whose definition, printing, and case-insensitive name
    /// identities are unique.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogError`] when a definition ID, printing ID, or
    /// normalized card name is repeated; a printing belongs to another
    /// definition; or structured parts, forms, modes, costs, and target slots
    /// are missing, invalid, or non-positional.
    pub fn new(
        definitions: impl IntoIterator<Item = CardDefinition>,
    ) -> Result<Self, CatalogError> {
        Self::with_additional_printings(definitions, std::iter::empty())
    }

    /// Builds a catalog and attaches reprints or alternate printings to their
    /// canonical card definitions.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogError`] under the same conditions as [`Self::new`], or
    /// when an additional printing references a definition outside the
    /// supplied catalog.
    pub fn with_additional_printings(
        definitions: impl IntoIterator<Item = CardDefinition>,
        printings: impl IntoIterator<Item = CardPrinting>,
    ) -> Result<Self, CatalogError> {
        let mut entries = CatalogEntries::default();
        let mut definition_printings = Vec::new();
        for mut definition in definitions {
            if entries.definition_index(definition.id).is_some() {
                return Err(CatalogError::DuplicateId(definition.id));
            }
            // Tokens are not deck-legal and are never looked up by name, and
            // Magic prints several that share one. Only the cards a decklist
            // can name have to be distinguishable by name.
            let is_token = definition.debut_set == CardSet::Token;
            let normalized_name = normalize_name(&definition.name);
            if !is_token && entries.ids_by_name.contains_key(&normalized_name) {
                return Err(CatalogError::DuplicateName(definition.name));
            }
            validate_composition(&definition)?;
            let supplied_printings = std::mem::take(&mut definition.printings);
            definition_printings.extend(
                supplied_printings
                    .into_iter()
                    .map(|printing| (definition.id, printing)),
            );
            if !is_token {
                entries.ids_by_name.insert(normalized_name, definition.id);
            }
            entries.insert_definition(definition);
        }

        for (definition, printing) in definition_printings {
            if printing.id.definition != definition {
                return Err(CatalogError::MismatchedPrintingDefinition {
                    definition,
                    printing: printing.id,
                });
            }
            entries.attach_printing(printing)?;
        }
        for printing in printings {
            entries.attach_printing(printing)?;
        }
        Ok(Self {
            entries: Arc::new(entries),
        })
    }

    #[must_use]
    pub fn get(&self, id: CardDefinitionId) -> Option<&CardDefinition> {
        self.entries.definition(id)
    }

    /// Every definition in the catalog, ordered by id so consumers see a
    /// stable listing.
    #[must_use]
    pub fn definitions(&self) -> Vec<&CardDefinition> {
        let mut definitions: Vec<_> = self.entries.definitions.iter().collect();
        definitions.sort_by_key(|definition| definition.id);
        definitions
    }

    /// Looks up a card definition ID by its case-insensitive printed name.
    #[must_use]
    pub fn find_by_name(&self, name: &str) -> Option<CardDefinitionId> {
        self.entries.ids_by_name.get(&normalize_name(name)).copied()
    }

    #[must_use]
    pub fn get_printing(&self, id: CardPrintingId) -> Option<&CardPrinting> {
        let definition = self.entries.definition_by_printing.get(&id)?;
        self.entries
            .definition(*definition)?
            .printings
            .iter()
            .find(|printing| printing.id == id)
    }

    /// Returns every known printing of `id`, or an empty slice for an unknown
    /// definition.
    #[must_use]
    pub fn printings_for(&self, id: CardDefinitionId) -> &[CardPrinting] {
        self.get(id).map_or(&[], |card| card.printings.as_slice())
    }

    #[must_use]
    pub fn has_printing_in(&self, id: CardDefinitionId, set: CardSet) -> bool {
        self.printings_for(id)
            .iter()
            .any(|printing| printing.id.set == set)
    }

    #[must_use]
    pub fn is_banned(&self, id: CardDefinitionId) -> bool {
        self.is_banned_in(id, Format::OldSchool9394)
    }

    #[must_use]
    pub fn is_allowed_in(&self, id: CardDefinitionId, format: Format) -> bool {
        self.get(id).is_some_and(|card| format.allows_card(card))
    }

    #[must_use]
    pub fn is_banned_in(&self, id: CardDefinitionId, format: Format) -> bool {
        self.get(id)
            .is_some_and(|card| format.is_banned(&card.name))
    }

    #[must_use]
    pub fn is_restricted(&self, id: CardDefinitionId) -> bool {
        self.is_restricted_in(id, Format::OldSchool9394)
    }

    #[must_use]
    pub fn is_restricted_in(&self, id: CardDefinitionId, format: Format) -> bool {
        self.get(id)
            .is_some_and(|card| format.is_restricted(&card.name))
    }
}

/// Folds a printed card name to the key both the catalog and a lookup use.
///
/// Magic prints accented names — Juzám Djinn, Márton Stromgald, Lim-Dûl —
/// and decklists, search boxes, and bot authors overwhelmingly type them
/// without the accents. Folding here means the catalog can store the name as
/// printed while every spelling still resolves to the same card.
fn normalize_name(name: &str) -> String {
    let mut normalized = String::with_capacity(name.len());

    for lowered in name.trim().chars().flat_map(char::to_lowercase) {
        match ascii_fold(lowered) {
            Some(replacement) => normalized.push_str(replacement),
            None => normalized.push(lowered),
        }
    }

    normalized
}

/// The ASCII spelling of one lowercase Latin-1 letter, or `None` when the
/// character is already the key form.
///
/// Ligatures and the letters that conventionally spell out are handled as
/// strings rather than single characters, so Æther folds to `aether` and not
/// to `ather` — otherwise typing the unaccented name would stop matching,
/// which is the whole point of folding.
fn ascii_fold(lowered: char) -> Option<&'static str> {
    Some(match lowered {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => "a",
        'æ' => "ae",
        'ç' => "c",
        'è' | 'é' | 'ê' | 'ë' => "e",
        'ì' | 'í' | 'î' | 'ï' => "i",
        'ð' => "d",
        'ñ' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => "o",
        'ù' | 'ú' | 'û' | 'ü' => "u",
        'ý' | 'ÿ' => "y",
        'þ' => "th",
        'ß' => "ss",
        _ => return None,
    })
}

fn validate_composition(definition: &CardDefinition) -> Result<(), CatalogError> {
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

    validate_fused_option(definition)
}

fn validate_alternative_cast_abilities(definition: &CardDefinition) -> Result<(), CatalogError> {
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

fn validate_abilities(
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

    if source_zones.is_some_and(<[super::ZoneKind]>::is_empty) {
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

fn collect_ability_grants(effect: super::EffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        super::EffectDef::Sequence(effects) => {
            for effect in effects {
                collect_ability_grants(*effect, grants);
            }
        }
        super::EffectDef::OptionalManaPayment { effect, .. }
        | super::EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | super::EffectDef::May(effect)
        | super::EffectDef::IfCondition { then: effect, .. }
        | super::EffectDef::AtNextStep { effect, .. } => {
            collect_ability_grants(*effect, grants);
        }
        super::EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => collect_ability_grants(*effect, grants),
        super::EffectDef::Apply { effect, .. } => collect_applied_ability_grants(effect, grants),
        super::EffectDef::TriggerUntilYourNextTurn { .. }
        | super::EffectDef::None
        | super::EffectDef::AddMana(_)
        | super::EffectDef::AddManaEqualTo { .. }
        | super::EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | super::EffectDef::GainLife { .. }
        | super::EffectDef::DrawCards { .. }
        | super::EffectDef::DiscardCards { .. }
        | EffectDef::DiscardAtRandom { .. }
        | super::EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | super::EffectDef::Tap { .. }
        | super::EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
        | super::EffectDef::Attach { .. }
        | super::EffectDef::CreateToken { .. }
        | super::EffectDef::Destroy { .. }
        | super::EffectDef::Sacrifice { .. }
        | super::EffectDef::SacrificeOfChoice { then: None, .. }
        | super::EffectDef::DestroyOfChoice { .. }
        | super::EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | super::EffectDef::Mill { .. }
        | super::EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtHand { .. }
        | super::EffectDef::SearchLibrary { .. }
        | super::EffectDef::Counter { .. }
        | super::EffectDef::CounterUnlessPaid { .. }
        | super::EffectDef::AddCounters { .. }
        | super::EffectDef::ChangeTextBasicLandType { .. }
        | super::EffectDef::BecomeCopyOf { .. }
        | super::EffectDef::CannotBeForcedToSacrifice
        | super::EffectDef::CreateEmblem { .. }
        | super::EffectDef::Transform { .. }
        | super::EffectDef::AdditionalCombatPhase
        | super::EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | super::EffectDef::GrantFlashToNextSorcery
        | super::EffectDef::ExileLinkedToSource { .. }
        | super::EffectDef::ReturnLinkedExiles { .. }
        | super::EffectDef::MakeUnblockableThisTurn { .. }
        | super::EffectDef::GainControlThisTurn { .. }
        | super::EffectDef::ReduceGenericCostBy(_)
        | super::EffectDef::PlayersCantPlay(_)
        | super::EffectDef::MultiplyEventAmount(_)
        | super::EffectDef::Replacement(_)
        | super::EffectDef::MoveToZone { .. }
        | super::EffectDef::ChooseCardName { .. }
        | super::EffectDef::ChoosePlayer { .. }
        | super::EffectDef::CopyPermanentAsItEnters { .. }
        | super::EffectDef::ChooseCreatureType { .. }
        | super::EffectDef::Special(_) => {}
    }
}

fn collect_applied_ability_grants(effect: super::AppliedEffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        super::AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                collect_applied_ability_grants(*effect, grants);
            }
        }
        super::AppliedEffectDef::GrantAbility(ability) => grants.push(ability),
        super::AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::CannotBeEnchanted
        | super::AppliedEffectDef::CannotBeBlockedBy(_)
        | super::AppliedEffectDef::PreventDamageFrom(_)
        | super::AppliedEffectDef::AddLandTypes(_)
        | super::AppliedEffectDef::SetLandTypes(_)
        | super::AppliedEffectDef::RemoveAbilities(_)
        | super::AppliedEffectDef::Animate(_)
        | super::AppliedEffectDef::ModifyPowerToughness { .. }
        | super::AppliedEffectDef::Special(_) => {}
    }
}

fn ability_grant_sites(effect: super::EffectDef) -> usize {
    match effect {
        super::EffectDef::Sequence(effects) => effects
            .iter()
            .map(|effect| ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        super::EffectDef::OptionalManaPayment { effect, .. }
        | super::EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | super::EffectDef::May(effect)
        | super::EffectDef::IfCondition { then: effect, .. }
        | super::EffectDef::AtNextStep { effect, .. }
        | super::EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => ability_grant_sites(*effect),
        super::EffectDef::Apply { effect, .. } => applied_ability_grant_sites(effect),
        super::EffectDef::TriggerUntilYourNextTurn { .. }
        | super::EffectDef::None
        | super::EffectDef::AddMana(_)
        | super::EffectDef::AddManaEqualTo { .. }
        | super::EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | super::EffectDef::GainLife { .. }
        | super::EffectDef::DrawCards { .. }
        | super::EffectDef::DiscardCards { .. }
        | EffectDef::DiscardAtRandom { .. }
        | super::EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | super::EffectDef::Tap { .. }
        | super::EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
        | super::EffectDef::Attach { .. }
        | super::EffectDef::CreateToken { .. }
        | super::EffectDef::Destroy { .. }
        | super::EffectDef::Sacrifice { .. }
        | super::EffectDef::SacrificeOfChoice { then: None, .. }
        | super::EffectDef::DestroyOfChoice { .. }
        | super::EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | super::EffectDef::Mill { .. }
        | super::EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtHand { .. }
        | super::EffectDef::SearchLibrary { .. }
        | super::EffectDef::Counter { .. }
        | super::EffectDef::CounterUnlessPaid { .. }
        | super::EffectDef::AddCounters { .. }
        | super::EffectDef::ChangeTextBasicLandType { .. }
        | super::EffectDef::BecomeCopyOf { .. }
        | super::EffectDef::CannotBeForcedToSacrifice
        | super::EffectDef::CreateEmblem { .. }
        | super::EffectDef::Transform { .. }
        | super::EffectDef::AdditionalCombatPhase
        | super::EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | super::EffectDef::GrantFlashToNextSorcery
        | super::EffectDef::ExileLinkedToSource { .. }
        | super::EffectDef::ReturnLinkedExiles { .. }
        | super::EffectDef::MakeUnblockableThisTurn { .. }
        | super::EffectDef::GainControlThisTurn { .. }
        | super::EffectDef::ReduceGenericCostBy(_)
        | super::EffectDef::PlayersCantPlay(_)
        | super::EffectDef::MultiplyEventAmount(_)
        | super::EffectDef::Replacement(_)
        | super::EffectDef::MoveToZone { .. }
        | super::EffectDef::ChooseCardName { .. }
        | super::EffectDef::ChoosePlayer { .. }
        | super::EffectDef::CopyPermanentAsItEnters { .. }
        | super::EffectDef::ChooseCreatureType { .. }
        | super::EffectDef::Special(_) => 0,
    }
}

fn applied_ability_grant_sites(effect: super::AppliedEffectDef) -> usize {
    match effect {
        super::AppliedEffectDef::Composite(effects) => effects
            .iter()
            .map(|effect| applied_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        super::AppliedEffectDef::GrantAbility(_) => 1,
        super::AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::CannotBeEnchanted
        | super::AppliedEffectDef::CannotBeBlockedBy(_)
        | super::AppliedEffectDef::PreventDamageFrom(_)
        | super::AppliedEffectDef::AddLandTypes(_)
        | super::AppliedEffectDef::SetLandTypes(_)
        | super::AppliedEffectDef::RemoveAbilities(_)
        | super::AppliedEffectDef::Animate(_)
        | super::AppliedEffectDef::ModifyPowerToughness { .. }
        | super::AppliedEffectDef::Special(_) => 0,
    }
}

fn validate_ability_targets(
    targets: &[AbilityTargetDef],
    effect: EffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    if targets.len() > usize::from(u8::MAX) + 1 {
        return Err(GrantedAbilityValidationError::TooManyTargets {
            count: targets.len(),
        });
    }
    for (position, definition) in targets.iter().enumerate() {
        let target = TargetIndex::from_index(position)
            .expect("the target count was validated before assigning positional indices");
        if definition.minimum > definition.maximum {
            return Err(GrantedAbilityValidationError::InvalidTargetBounds {
                target,
                minimum: definition.minimum,
                maximum: definition.maximum,
            });
        }
    }
    validate_effect_target_references(effect, targets.len())
}

fn validate_target_index(
    target: TargetIndex,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    if target.index() < target_count {
        Ok(())
    } else {
        Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target,
            target_count,
        })
    }
}

fn validate_recipient_target_references(
    recipient: EffectRecipientDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    match recipient {
        EffectRecipientDef::ObjectsSharingNameWithTarget(target)
        | EffectRecipientDef::Target(target)
        | EffectRecipientDef::ControllerOfTarget(target) => {
            validate_target_index(target, target_count)
        }
        EffectRecipientDef::ObjectsControlledByTarget { slot, .. }
        | EffectRecipientDef::ObjectsOwnedByTarget { slot, .. } => {
            validate_target_index(slot, target_count)
        }
        EffectRecipientDef::Source
        | EffectRecipientDef::AttachedPermanent
        | EffectRecipientDef::Controller
        | EffectRecipientDef::Opponent
        | EffectRecipientDef::TriggeringObject
        | EffectRecipientDef::ControllerOfTriggeringObject
        | EffectRecipientDef::EventPlayer
        | EffectRecipientDef::MatchingObjects { .. }
        | EffectRecipientDef::EachPlayer => Ok(()),
    }
}

fn validate_value_target_references(
    value: ValueDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    match value {
        ValueDef::Negate(value) => validate_value_target_references(*value, target_count),
        ValueDef::IfCreatureDiedThisTurn(condition) => {
            validate_value_target_references(condition.then, target_count)?;
            validate_value_target_references(condition.otherwise, target_count)
        }
        ValueDef::IfTargetMatches(condition) => {
            validate_target_index(condition.slot, target_count)?;
            validate_value_target_references(condition.then, target_count)?;
            validate_value_target_references(condition.otherwise, target_count)
        }
        ValueDef::IfMatchingObjectCount(condition) => {
            validate_value_target_references(condition.then, target_count)?;
            validate_value_target_references(condition.otherwise, target_count)
        }
        ValueDef::TargetPower(target)
        | ValueDef::TargetManaValue(target) => validate_target_index(target, target_count),
        ValueDef::Constant(_)
        | ValueDef::ChosenX
        | ValueDef::SourcePower
        | ValueDef::SourceToughness
        | ValueDef::TriggerEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::CountMatchingObjects(_)
        | ValueDef::AnyMatchingObject(_)
        | ValueDef::CountersOnSource(_)
        // This reads the share assigned to the target currently being
        // affected; the surrounding recipient carries the slot reference.
        | ValueDef::DividedAmongTargets => Ok(()),
    }
}

fn validate_applied_effect_target_references(
    effect: AppliedEffectDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                validate_applied_effect_target_references(*effect, target_count)?;
            }
            Ok(())
        }
        AppliedEffectDef::ModifyPowerToughness { power, toughness } => {
            validate_value_target_references(power, target_count)?;
            validate_value_target_references(toughness, target_count)
        }
        // A granted ability introduces its own target scope and is validated
        // separately when the grant tree is traversed.
        AppliedEffectDef::GrantAbility(_)
        | AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::Animate(_)
        | AppliedEffectDef::Special(_) => Ok(()),
    }
}

#[allow(clippy::too_many_lines)]
fn validate_effect_target_references(
    effect: EffectDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                validate_effect_target_references(*effect, target_count)?;
            }
            Ok(())
        }
        EffectDef::DealDamage { recipient, amount }
        | EffectDef::DrainLife { recipient, amount }
        | EffectDef::GainLife { recipient, amount }
        | EffectDef::DrawCards { recipient, amount }
        | EffectDef::DiscardCards { recipient, amount }
        | EffectDef::DiscardAtRandom { recipient, amount }
        | EffectDef::LoseLife { recipient, amount } => {
            validate_recipient_target_references(recipient, target_count)?;
            validate_value_target_references(amount, target_count)
        }
        EffectDef::LoseTheGame { player: object }
        | EffectDef::Tap { object }
        | EffectDef::Untap { object }
        | EffectDef::PreventCombatDamageThisTurn { object }
        | EffectDef::Attach { object }
        | EffectDef::Destroy { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::BecomeCopyOf { object, .. }
        | EffectDef::ExileLinkedToSource { object }
        | EffectDef::MakeUnblockableThisTurn { object }
        | EffectDef::GainControlThisTurn { object }
        | EffectDef::Transform { object }
        | EffectDef::MoveToZone { object, .. }
        | EffectDef::ChooseCardName { object }
        | EffectDef::ChooseCreatureType { object } => {
            validate_recipient_target_references(object, target_count)
        }
        // A reveal always comes off the resolving object's controller's own
        // library, so its count is the only part that could name a target.
        EffectDef::RevealAndSplitIntoPiles { count, .. }
        | EffectDef::CreateToken { count, .. }
        | EffectDef::ReduceGenericCostBy(count) => {
            validate_value_target_references(count, target_count)
        }
        EffectDef::SacrificeOfChoice { player, then, .. } => {
            validate_recipient_target_references(player, target_count)?;
            if let Some(effect) = then {
                validate_effect_target_references(*effect, target_count)?;
            }
            Ok(())
        }
        EffectDef::DestroyOfChoice { player, .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { player }
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { player }
        | EffectDef::SearchLibrary { player, .. }
        | EffectDef::LookAtHand { player }
        | EffectDef::LookAtTopAndMayTake { player, .. } => {
            validate_recipient_target_references(player, target_count)
        }
        EffectDef::Mill { player, amount } => {
            validate_recipient_target_references(player, target_count)?;
            validate_value_target_references(amount, target_count)
        }
        EffectDef::Counter { object } => validate_recipient_target_references(object, target_count),
        EffectDef::CounterUnlessPaid { object, amount, .. }
        | EffectDef::AddCounters { object, amount, .. } => {
            validate_recipient_target_references(object, target_count)?;
            validate_value_target_references(amount, target_count)
        }
        EffectDef::OptionalManaPayment { effect, .. }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | EffectDef::May(effect)
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::AtNextStep { effect, .. } => {
            validate_effect_target_references(*effect, target_count)
        }
        EffectDef::Apply {
            recipient, effect, ..
        } => {
            validate_recipient_target_references(recipient, target_count)?;
            validate_applied_effect_target_references(effect, target_count)
        }
        // An installed ability chooses its own targets when it triggers, so
        // nothing in it can refer to this ability's target slots.
        // The chosen player is recorded on the permanent, not read from a
        // target slot.
        // A prohibition names a card shape, never a target.
        EffectDef::PlayersCantPlay(_)
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::TriggerUntilYourNextTurn { .. }
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::AdditionalCombatPhase
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::Special(_) => Ok(()),
    }
}

fn structure_parts(definition: &CardDefinition) -> Result<Vec<CardPartId>, CatalogError> {
    let parts = match &definition.structure {
        CardStructure::Single { main } => vec![*main],
        CardStructure::Split { parts, .. } => {
            if parts.len() < 2 {
                return Err(CatalogError::InvalidSplitPartCount {
                    definition: definition.id,
                    actual: parts.len(),
                });
            }
            parts.clone()
        }
        CardStructure::Flip { normal, flipped } => vec![*normal, *flipped],
        CardStructure::DoubleFaced { front, back, .. } => vec![*front, *back],
        CardStructure::AlternateSpell {
            main, alternate, ..
        } => vec![*main, *alternate],
        CardStructure::MeldPart { front, .. } => vec![*front],
    };

    let mut seen = HashSet::new();
    for part in &parts {
        if !seen.insert(*part) {
            return Err(CatalogError::DuplicateStructurePart {
                definition: definition.id,
                part: *part,
            });
        }
    }
    Ok(parts)
}

fn validate_spell_form(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    defined_parts: &HashSet<CardPartId>,
    structure_parts: &[CardPartId],
) -> Result<(), CatalogError> {
    let form_parts = match &option.form {
        SpellForm::Part(part) => vec![*part],
        SpellForm::Combined(parts) => {
            if parts.is_empty() {
                return Err(CatalogError::EmptySpellForm {
                    definition: definition.id,
                    option: option.id,
                });
            }
            let mut seen = HashSet::new();
            for part in parts {
                if !seen.insert(*part) {
                    return Err(CatalogError::DuplicateSpellFormPart {
                        definition: definition.id,
                        option: option.id,
                        part: *part,
                    });
                }
            }
            parts.clone()
        }
    };

    for part in form_parts {
        if !defined_parts.contains(&part) {
            return Err(CatalogError::UndefinedSpellFormPart {
                definition: definition.id,
                option: option.id,
                part,
            });
        }
        if !structure_parts.contains(&part) {
            return Err(CatalogError::SpellFormPartOutsideStructure {
                definition: definition.id,
                option: option.id,
                part,
            });
        }
    }
    Ok(())
}

fn validate_cost_ids(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    additional_costs: &mut HashSet<AdditionalCostId>,
) -> Result<(), CatalogError> {
    // Alternative identities are interpreted together with their play option.
    // In particular, alternative-cast clauses on two split-card parts may
    // have the same positional AbilityId and therefore the same projected ID.
    let mut alternative_costs = HashSet::new();
    for cost in &option.alternative_costs {
        if !alternative_costs.insert(cost.id) {
            return Err(CatalogError::DuplicateAlternativeCostId {
                definition: definition.id,
                option: option.id,
                cost: cost.id,
            });
        }
    }
    for cost in &option.additional_costs {
        if !additional_costs.insert(cost.id) {
            return Err(CatalogError::DuplicateAdditionalCostId {
                definition: definition.id,
                cost: cost.id,
            });
        }
    }
    Ok(())
}

fn validate_modes_and_targets(
    definition: &CardDefinition,
    option: &PlayOptionDef,
) -> Result<(), CatalogError> {
    validate_target_slots(definition, option, None, &option.targets)?;

    let Some(mode_set) = &option.modes else {
        return Ok(());
    };
    validate_mode_selection_bounds(definition, option, mode_set)?;

    let mut option_modes = HashSet::new();
    for mode in &mode_set.modes {
        if !option_modes.insert(mode.id) {
            return Err(CatalogError::DuplicateModeId {
                definition: definition.id,
                option: option.id,
                mode: mode.id,
            });
        }
    }

    for (index, mode) in mode_set.modes.iter().enumerate() {
        let expected = ModeId::from_index(index)
            .expect("validated mode sets cannot exceed the positional ID range");
        if mode.id != expected {
            return Err(CatalogError::NonPositionalModeId {
                definition: definition.id,
                option: option.id,
                expected,
                actual: mode.id,
            });
        }
        validate_target_slots(definition, option, Some(mode.id), &mode.targets)?;
    }

    let mut mode_target_counts = mode_set
        .modes
        .iter()
        .map(|mode| mode.targets.len())
        .collect::<Vec<_>>();
    let selected_target_count = if mode_set.may_repeat {
        mode_target_counts
            .into_iter()
            .max()
            .unwrap_or(0)
            .saturating_mul(usize::from(mode_set.maximum))
    } else {
        mode_target_counts.sort_unstable_by(|left, right| right.cmp(left));
        mode_target_counts
            .into_iter()
            .take(usize::from(mode_set.maximum))
            .fold(0, usize::saturating_add)
    };
    let instantiated = option.targets.len().saturating_add(selected_target_count);
    if instantiated > usize::from(u8::MAX) + 1 {
        return Err(CatalogError::TooManyInstantiatedTargets {
            definition: definition.id,
            option: option.id,
            count: instantiated,
        });
    }
    Ok(())
}

fn validate_mode_selection_bounds(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    mode_set: &ModeSetDef,
) -> Result<(), CatalogError> {
    if mode_set.modes.is_empty() {
        return Err(CatalogError::EmptyModeSet {
            definition: definition.id,
            option: option.id,
        });
    }
    if mode_set.modes.len() > usize::from(u8::MAX) + 1 {
        return Err(CatalogError::TooManyModes {
            definition: definition.id,
            option: option.id,
            count: mode_set.modes.len(),
        });
    }
    if mode_set.minimum > mode_set.maximum {
        return Err(CatalogError::InvalidModeBounds {
            definition: definition.id,
            option: option.id,
            minimum: mode_set.minimum,
            maximum: mode_set.maximum,
        });
    }
    if mode_set.maximum == 0 {
        return Err(CatalogError::ZeroModeMaximum {
            definition: definition.id,
            option: option.id,
        });
    }
    if !mode_set.may_repeat && usize::from(mode_set.maximum) > mode_set.modes.len() {
        return Err(CatalogError::TooManyModesWithoutRepetition {
            definition: definition.id,
            option: option.id,
            maximum: mode_set.maximum,
            available: mode_set.modes.len(),
        });
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_semantic_spell_presentation(
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
            any_executable |= ability.is_executable();
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
        if ability.is_executable() && option.modes.is_some() {
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
        let expected_status = if ability.is_executable() && semantic.is_executable() {
            CardEffectStatus::Implemented
        } else {
            CardEffectStatus::MetadataOnly
        };
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

fn validate_target_slots(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    mode: Option<ModeId>,
    slots: &[TargetSlotDef],
) -> Result<(), CatalogError> {
    if slots.len() > usize::from(u8::MAX) + 1 {
        return Err(CatalogError::TooManyTargetSlots {
            definition: definition.id,
            option: option.id,
            mode,
            count: slots.len(),
        });
    }
    for (position, slot) in slots.iter().enumerate() {
        let expected = TargetSlotId::from_index(position)
            .expect("the target slot count was validated before assigning positional IDs");
        if slot.id != expected {
            return Err(CatalogError::NonPositionalTargetSlot {
                definition: definition.id,
                option: option.id,
                mode,
                expected,
                actual: slot.id,
            });
        }
        if slot.minimum > slot.maximum {
            return Err(CatalogError::InvalidTargetBounds {
                definition: definition.id,
                option: option.id,
                mode,
                slot: slot.id,
                minimum: slot.minimum,
                maximum: slot.maximum,
            });
        }
    }
    Ok(())
}

fn validate_fused_option(definition: &CardDefinition) -> Result<(), CatalogError> {
    let CardStructure::Split { parts, fused } = &definition.structure else {
        if let Some(option) = definition
            .play_options
            .iter()
            .find(|option| matches!(option.form, SpellForm::Combined(_)))
        {
            return Err(CatalogError::UnexpectedCombinedSpellForm {
                definition: definition.id,
                option: option.id,
            });
        }
        return Ok(());
    };

    for option in &definition.play_options {
        if matches!(option.form, SpellForm::Combined(_)) && Some(option.id) != *fused {
            return Err(CatalogError::UnexpectedCombinedSpellForm {
                definition: definition.id,
                option: option.id,
            });
        }
    }

    let Some(fused) = fused else {
        return Ok(());
    };
    let Some(option) = definition.play_option(*fused) else {
        return Err(CatalogError::MissingFusedPlayOption {
            definition: definition.id,
            option: *fused,
        });
    };
    if option.action != PlayActionKind::CastSpell
        || option.form != SpellForm::Combined(parts.clone())
    {
        return Err(CatalogError::InvalidFusedPlayOption {
            definition: definition.id,
            option: *fused,
            expected: parts.clone(),
            actual: option.form.clone(),
            actual_action: option.action,
        });
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GrantedAbilityValidationError {
    TooManyGrantSites {
        count: usize,
    },
    EmptyText,
    MissingImplementationExplanation,
    LegacyProcedureRequiresCustomExecution,
    HasNoSourceZone,
    ManaAbilityHasTargets,
    TooManyTargets {
        count: usize,
    },
    InvalidTargetBounds {
        target: TargetIndex,
        minimum: u8,
        maximum: u8,
    },
    TargetReferenceOutOfBounds {
        target: TargetIndex,
        target_count: usize,
    },
    /// Runtime static-effect discovery currently starts from attached printed
    /// or copied clauses. Reject an executable static ability granted by
    /// another ability until continuous effects have guarded fixed-point
    /// evaluation rather than silently claiming support.
    ExecutableStaticAbility,
}

impl fmt::Display for GrantedAbilityValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyGrantSites { count } => write!(
                formatter,
                "defines {count} grant sites, but grant IDs support at most 256"
            ),
            Self::EmptyText => formatter.write_str("has empty rules text"),
            Self::MissingImplementationExplanation => formatter.write_str(
                "has a non-declarative implementation without an explanation",
            ),
            Self::LegacyProcedureRequiresCustomExecution => formatter.write_str(
                "uses the legacy rules procedure without a custom effect executor",
            ),
            Self::HasNoSourceZone => formatter.write_str("has no source zone"),
            Self::ManaAbilityHasTargets => formatter.write_str("is a mana ability that declares targets"),
            Self::TooManyTargets { count } => write!(
                formatter,
                "defines {count} targets, but positional target indices support at most 256"
            ),
            Self::InvalidTargetBounds {
                target,
                minimum,
                maximum,
            } => write!(
                formatter,
                "defines target {target:?} requiring at least {minimum} targets but allowing at most {maximum}",
            ),
            Self::TargetReferenceOutOfBounds {
                target,
                target_count,
            } => write!(
                formatter,
                "references target {target:?}, but the clause defines only {target_count} target slots"
            ),
            Self::ExecutableStaticAbility => formatter.write_str(
                "is an executable static ability, but granted static abilities are not evaluated yet",
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogError {
    DuplicateId(CardDefinitionId),
    DuplicateName(String),
    DuplicatePrintingId(CardPrintingId),
    MismatchedPrintingDefinition {
        definition: CardDefinitionId,
        printing: CardPrintingId,
    },
    OrphanPrinting(CardPrintingId),
    EmptyAbilityText {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    MissingImplementationExplanation {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    LegacyProcedureRequiresCustomExecution {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    DuplicatePartId {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    IncoherentCardRules {
        definition: CardDefinitionId,
        part: CardPartId,
        explanation: &'static str,
    },
    MismatchedPrimaryRules {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    TooManyAbilities {
        definition: CardDefinitionId,
        part: CardPartId,
        count: usize,
    },
    MultipleSpellAbilities {
        definition: CardDefinitionId,
        part: CardPartId,
        count: usize,
    },
    InvalidModalSpellParent {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    TooManySpellModes {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        count: usize,
    },
    InvalidModalSpellSelection {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
        available: usize,
    },
    NonSpellMode {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
    },
    NestedModalSpellMode {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
    },
    CustomSpellModeImplementation {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
    },
    InvalidSpellMode {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        mode: ModeId,
        problem: GrantedAbilityValidationError,
    },
    TooManyAbilityGrantSites {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        count: usize,
    },
    InvalidGrantedAbility {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        grant_path: Vec<GrantId>,
        problem: GrantedAbilityValidationError,
    },
    AbilityHasNoSourceZone {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    ManaAbilityHasTargets {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    TooManyAbilityTargets {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        count: usize,
    },
    InvalidAbilityTargetBounds {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        target: TargetIndex,
        minimum: u8,
        maximum: u8,
    },
    AbilityTargetReferenceOutOfBounds {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        target: TargetIndex,
        target_count: usize,
    },
    DuplicateStructurePart {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    InvalidSplitPartCount {
        definition: CardDefinitionId,
        actual: usize,
    },
    UndefinedStructurePart {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    PartOutsideStructure {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    DuplicatePlayOptionId {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    EmptySpellForm {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    DuplicateSpellFormPart {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    UndefinedSpellFormPart {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    SpellFormPartOutsideStructure {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    MissingFusedPlayOption {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    InvalidFusedPlayOption {
        definition: CardDefinitionId,
        option: PlayOptionId,
        expected: Vec<CardPartId>,
        actual: SpellForm,
        actual_action: PlayActionKind,
    },
    UnexpectedCombinedSpellForm {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    CombinedModalSpellUnsupported {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    DuplicateModeId {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
    },
    NonPositionalModeId {
        definition: CardDefinitionId,
        option: PlayOptionId,
        expected: ModeId,
        actual: ModeId,
    },
    EmptyModeSet {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    TooManyModes {
        definition: CardDefinitionId,
        option: PlayOptionId,
        count: usize,
    },
    InvalidModeBounds {
        definition: CardDefinitionId,
        option: PlayOptionId,
        minimum: u8,
        maximum: u8,
    },
    ZeroModeMaximum {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    TooManyModesWithoutRepetition {
        definition: CardDefinitionId,
        option: PlayOptionId,
        maximum: u8,
        available: usize,
    },
    UnexpectedPresentationSpellModes {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    MissingPresentationSpellTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
    },
    MissingSemanticSpellTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
    },
    MismatchedSpellTargetCardinality {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
        presentation_minimum: u8,
        presentation_maximum: u8,
        semantic_minimum: u8,
        semantic_maximum: u8,
    },
    UnpresentableSpellTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        target: TargetSlotId,
    },
    MismatchedSpellTargetPresentation {
        definition: CardDefinitionId,
        option: PlayOptionId,
        position: usize,
        presentation: TargetSlotDef,
        semantic: TargetSlotDef,
    },
    UnexpectedModalSpellTargets {
        definition: CardDefinitionId,
        option: PlayOptionId,
        count: usize,
    },
    MissingPresentationSpellMode {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
    },
    MissingSemanticSpellMode {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
    },
    MissingPresentationSpellModeTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
    },
    MissingSemanticSpellModeTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
    },
    MismatchedSpellModeTargetCardinality {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
        presentation_minimum: u8,
        presentation_maximum: u8,
        semantic_minimum: u8,
        semantic_maximum: u8,
    },
    UnpresentableSpellModeTarget {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        target: TargetSlotId,
    },
    MismatchedSpellModeTargetPresentation {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        position: usize,
        presentation: TargetSlotDef,
        semantic: TargetSlotDef,
    },
    MismatchedSpellModeSelection {
        definition: CardDefinitionId,
        option: PlayOptionId,
        presentation_minimum: u8,
        presentation_maximum: u8,
        presentation_may_repeat: bool,
        semantic_minimum: u8,
        semantic_maximum: u8,
        semantic_may_repeat: bool,
    },
    MismatchedSpellModeImplementation {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        presentation: CardEffectStatus,
        semantic: CardEffectStatus,
    },
    MismatchedSpellModeLabel {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
        presentation: String,
        semantic: &'static str,
    },
    DuplicateAlternativeCostId {
        definition: CardDefinitionId,
        option: PlayOptionId,
        cost: AlternativeCostId,
    },
    MissingAlternativeCostForAbility {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        cost: AlternativeCostId,
    },
    MismatchedAlternativeCostForAbility {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        option: PlayOptionId,
        cost: AlternativeCostId,
        expected_label: String,
        actual_label: String,
        expected_mana_cost: ManaCost,
        actual_mana_cost: ManaCost,
    },
    DuplicateAdditionalCostId {
        definition: CardDefinitionId,
        cost: AdditionalCostId,
    },
    InvalidTargetBounds {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: Option<ModeId>,
        slot: TargetSlotId,
        minimum: u8,
        maximum: u8,
    },
    TooManyTargetSlots {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: Option<ModeId>,
        count: usize,
    },
    NonPositionalTargetSlot {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: Option<ModeId>,
        expected: TargetSlotId,
        actual: TargetSlotId,
    },
    TooManyInstantiatedTargets {
        definition: CardDefinitionId,
        option: PlayOptionId,
        count: usize,
    },
}

impl fmt::Display for CatalogError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate card definition ID {id:?}"),
            Self::DuplicateName(name) => write!(formatter, "duplicate card name {name:?}"),
            Self::DuplicatePrintingId(id) => write!(formatter, "duplicate card printing ID {id:?}"),
            Self::MismatchedPrintingDefinition {
                definition,
                printing,
            } => write!(
                formatter,
                "card printing {printing:?} was supplied by definition {definition:?}"
            ),
            Self::OrphanPrinting(id) => write!(
                formatter,
                "card printing {id:?} references an unknown definition"
            ),
            Self::EmptyAbilityText {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} has empty rules text"
            ),
            Self::MissingImplementationExplanation {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} has a non-declarative implementation without an explanation"
            ),
            Self::LegacyProcedureRequiresCustomExecution {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses the legacy rules procedure without a custom effect executor"
            ),
            Self::DuplicatePartId { definition, part } => write!(
                formatter,
                "card definition {definition:?} defines part {part:?} more than once"
            ),
            Self::IncoherentCardRules {
                definition,
                part,
                explanation,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} has incoherent rules: {explanation}"
            ),
            Self::MismatchedPrimaryRules { definition, part } => write!(
                formatter,
                "card definition {definition:?} has compatibility rules that differ from primary part {part:?}"
            ),
            Self::TooManyAbilities {
                definition,
                part,
                count,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} defines {count} abilities, but positional ability IDs support at most 256"
            ),
            Self::MultipleSpellAbilities {
                definition,
                part,
                count,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} defines {count} spell abilities, but one castable card part must have at most one"
            ),
            Self::InvalidModalSpellParent {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "modal spell ability {ability:?} on part {part:?} of card definition {definition:?} must be a targetless declarative wrapper with no effect of its own"
            ),
            Self::TooManySpellModes {
                definition,
                part,
                ability,
                count,
            } => write!(
                formatter,
                "spell ability {ability:?} on part {part:?} of card definition {definition:?} defines {count} modes, but positional mode IDs support at most 256"
            ),
            Self::InvalidModalSpellSelection {
                definition,
                part,
                ability,
                minimum,
                maximum,
                may_repeat,
                available,
            } => write!(
                formatter,
                "spell ability {ability:?} on part {part:?} of card definition {definition:?} declares {available} modes with selection bounds {minimum}..={maximum} (repeat={may_repeat}), which cannot produce a legal selection"
            ),
            Self::NonSpellMode {
                definition,
                part,
                ability,
                mode,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} is not an ordinary spell ability"
            ),
            Self::NestedModalSpellMode {
                definition,
                part,
                ability,
                mode,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} is itself modal"
            ),
            Self::CustomSpellModeImplementation {
                definition,
                part,
                ability,
                mode,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} uses a custom implementation, but modal branches currently require declarative effects"
            ),
            Self::InvalidSpellMode {
                definition,
                part,
                ability,
                mode,
                problem,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} {problem}"
            ),
            Self::TooManyAbilityGrantSites {
                definition,
                part,
                ability,
                count,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} defines {count} grant sites, but grant IDs support at most 256"
            ),
            Self::InvalidGrantedAbility {
                definition,
                part,
                ability,
                grant_path,
                problem,
            } => write!(
                formatter,
                "granted ability at path {grant_path:?} from ability {ability:?} on part {part:?} of card definition {definition:?} {problem}"
            ),
            Self::AbilityHasNoSourceZone {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} has no source zone"
            ),
            Self::ManaAbilityHasTargets {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "mana ability {ability:?} on part {part:?} of card definition {definition:?} declares targets"
            ),
            Self::TooManyAbilityTargets {
                definition,
                part,
                ability,
                count,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} defines {count} targets, but positional target indices support at most 256"
            ),
            Self::InvalidAbilityTargetBounds {
                definition,
                part,
                ability,
                target,
                minimum,
                maximum,
            } => write!(
                formatter,
                "target {target:?} of ability {ability:?} on part {part:?} of card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
            ),
            Self::AbilityTargetReferenceOutOfBounds {
                definition,
                part,
                ability,
                target,
                target_count,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} references target {target:?}, but defines only {target_count} target slots"
            ),
            Self::DuplicateStructurePart { definition, part } => write!(
                formatter,
                "card definition {definition:?}'s structure references part {part:?} more than once"
            ),
            Self::InvalidSplitPartCount { definition, actual } => write!(
                formatter,
                "split card definition {definition:?} must contain at least two ordered parts, but contains {actual}"
            ),
            Self::UndefinedStructurePart { definition, part } => write!(
                formatter,
                "card definition {definition:?}'s structure references undefined part {part:?}"
            ),
            Self::PartOutsideStructure { definition, part } => write!(
                formatter,
                "card definition {definition:?} defines part {part:?}, but its structure does not contain that part"
            ),
            Self::DuplicatePlayOptionId { definition, option } => write!(
                formatter,
                "card definition {definition:?} defines play option {option:?} more than once"
            ),
            Self::EmptySpellForm { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has an empty combined spell form"
            ),
            Self::DuplicateSpellFormPart {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} references part {part:?} more than once in its spell form"
            ),
            Self::UndefinedSpellFormPart {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} references undefined spell-form part {part:?}"
            ),
            Self::SpellFormPartOutsideStructure {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} uses part {part:?}, which is not in the card's structure"
            ),
            Self::MissingFusedPlayOption { definition, option } => write!(
                formatter,
                "split card definition {definition:?} names missing fused play option {option:?}"
            ),
            Self::InvalidFusedPlayOption {
                definition,
                option,
                expected,
                actual,
                actual_action,
            } => write!(
                formatter,
                "fused play option {option:?} of card definition {definition:?} must cast combined parts {expected:?} in printed order, but has action {actual_action:?} and form {actual:?}"
            ),
            Self::UnexpectedCombinedSpellForm { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has a combined spell form but is not its declared fused split option"
            ),
            Self::CombinedModalSpellUnsupported {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "combined play option {option:?} of card definition {definition:?} includes modal part {part:?}, but combined mode selections are not part-scoped"
            ),
            Self::DuplicateModeId {
                definition,
                option,
                mode,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} defines mode {mode:?} more than once"
            ),
            Self::NonPositionalModeId {
                definition,
                option,
                expected,
                actual,
            } => write!(
                formatter,
                "mode position {expected:?} in play option {option:?} of card definition {definition:?} uses ID {actual:?}; mode IDs must match printed position"
            ),
            Self::EmptyModeSet { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has a mode set with no modes"
            ),
            Self::TooManyModes {
                definition,
                option,
                count,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} defines {count} modes, but positional mode IDs support at most 256"
            ),
            Self::InvalidModeBounds {
                definition,
                option,
                minimum,
                maximum,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} requires at least {minimum} modes but allows at most {maximum}"
            ),
            Self::ZeroModeMaximum { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has modes but allows none to be selected"
            ),
            Self::TooManyModesWithoutRepetition {
                definition,
                option,
                maximum,
                available,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} allows {maximum} modes without repetition but defines only {available}"
            ),
            Self::UnexpectedPresentationSpellModes { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} presents mode choices for an executable nonmodal spell"
            ),
            Self::MissingPresentationSpellTarget {
                definition,
                option,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} has no presentation counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingSemanticSpellTarget {
                definition,
                option,
                target,
            } => write!(
                formatter,
                "presentation target {target:?} has no semantic counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MismatchedSpellTargetCardinality {
                definition,
                option,
                target,
                presentation_minimum,
                presentation_maximum,
                semantic_minimum,
                semantic_maximum,
            } => write!(
                formatter,
                "target {target:?} in play option {option:?} of card definition {definition:?} has presentation cardinality {presentation_minimum}..={presentation_maximum} but semantic cardinality {semantic_minimum}..={semantic_maximum}"
            ),
            Self::UnpresentableSpellTarget {
                definition,
                option,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} in play option {option:?} of card definition {definition:?} cannot be represented by the presentation target vocabulary"
            ),
            Self::MismatchedSpellTargetPresentation {
                definition,
                option,
                position,
                presentation,
                semantic,
            } => write!(
                formatter,
                "target at position {position} in play option {option:?} of card definition {definition:?} presents {presentation:?} but its semantic target projects to {semantic:?}"
            ),
            Self::UnexpectedModalSpellTargets {
                definition,
                option,
                count,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} presents {count} top-level targets for a semantic modal spell; targets must belong to its mode branches"
            ),
            Self::MissingPresentationSpellMode {
                definition,
                option,
                mode,
            } => write!(
                formatter,
                "semantic spell mode {mode:?} has no presentation counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingSemanticSpellMode {
                definition,
                option,
                mode,
            } => write!(
                formatter,
                "presentation mode {mode:?} has no semantic spell counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingPresentationSpellModeTarget {
                definition,
                option,
                mode,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} of spell mode {mode:?} has no presentation counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingSemanticSpellModeTarget {
                definition,
                option,
                mode,
                target,
            } => write!(
                formatter,
                "presentation target {target:?} of spell mode {mode:?} has no semantic counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MismatchedSpellModeTargetCardinality {
                definition,
                option,
                mode,
                target,
                presentation_minimum,
                presentation_maximum,
                semantic_minimum,
                semantic_maximum,
            } => write!(
                formatter,
                "target {target:?} of spell mode {mode:?} in play option {option:?} of card definition {definition:?} has presentation cardinality {presentation_minimum}..={presentation_maximum} but semantic cardinality {semantic_minimum}..={semantic_maximum}"
            ),
            Self::UnpresentableSpellModeTarget {
                definition,
                option,
                mode,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} of spell mode {mode:?} in play option {option:?} of card definition {definition:?} cannot be represented by the presentation target vocabulary"
            ),
            Self::MismatchedSpellModeTargetPresentation {
                definition,
                option,
                mode,
                position,
                presentation,
                semantic,
            } => write!(
                formatter,
                "target at position {position} of spell mode {mode:?} in play option {option:?} of card definition {definition:?} presents {presentation:?} but its semantic target projects to {semantic:?}"
            ),
            Self::MismatchedSpellModeSelection {
                definition,
                option,
                presentation_minimum,
                presentation_maximum,
                presentation_may_repeat,
                semantic_minimum,
                semantic_maximum,
                semantic_may_repeat,
            } => write!(
                formatter,
                "spell modes in play option {option:?} of card definition {definition:?} present {presentation_minimum}..={presentation_maximum} (repeat={presentation_may_repeat}) but declare {semantic_minimum}..={semantic_maximum} (repeat={semantic_may_repeat})"
            ),
            Self::MismatchedSpellModeImplementation {
                definition,
                option,
                mode,
                presentation,
                semantic,
            } => write!(
                formatter,
                "spell mode {mode:?} in play option {option:?} of card definition {definition:?} presents {presentation:?} but its semantic branch is {semantic:?}"
            ),
            Self::MismatchedSpellModeLabel {
                definition,
                option,
                mode,
                presentation,
                semantic,
            } => write!(
                formatter,
                "spell mode {mode:?} in play option {option:?} of card definition {definition:?} is labeled {presentation:?} but its semantic branch is labeled {semantic:?}"
            ),
            Self::DuplicateAlternativeCostId {
                definition,
                option,
                cost,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} defines alternative cost {cost:?} more than once"
            ),
            Self::MissingAlternativeCostForAbility {
                definition,
                part,
                ability,
                cost,
            } => write!(
                formatter,
                "alternative-cast ability {ability:?} on part {part:?} of card definition {definition:?} references missing cost {cost:?}"
            ),
            Self::MismatchedAlternativeCostForAbility {
                definition,
                part,
                ability,
                option,
                cost,
                expected_label,
                actual_label,
                expected_mana_cost,
                actual_mana_cost,
            } => write!(
                formatter,
                "alternative cost {cost:?} on play option {option:?}, projected from ability {ability:?} on part {part:?} of card definition {definition:?}, must be labeled {expected_label:?} with mana cost {expected_mana_cost}, but is labeled {actual_label:?} with mana cost {actual_mana_cost}"
            ),
            Self::DuplicateAdditionalCostId { definition, cost } => write!(
                formatter,
                "card definition {definition:?} defines additional cost {cost:?} more than once"
            ),
            Self::InvalidTargetBounds {
                definition,
                option,
                mode,
                slot,
                minimum,
                maximum,
            } => {
                if let Some(mode) = mode {
                    write!(
                        formatter,
                        "target slot {slot:?} in mode {mode:?} of play option {option:?} on card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
                    )
                } else {
                    write!(
                        formatter,
                        "target slot {slot:?} in play option {option:?} of card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
                    )
                }
            }
            Self::TooManyTargetSlots {
                definition,
                option,
                mode,
                count,
            } => write!(
                formatter,
                "{count} target slots are declared for mode {mode:?} of play option {option:?} on card definition {definition:?}, but positional target IDs support at most 256"
            ),
            Self::NonPositionalTargetSlot {
                definition,
                option,
                mode,
                expected,
                actual,
            } => write!(
                formatter,
                "target position {expected:?} in mode {mode:?} of play option {option:?} on card definition {definition:?} uses ID {actual:?}; target slot IDs must match instantiated order"
            ),
            Self::TooManyInstantiatedTargets {
                definition,
                option,
                count,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} can instantiate {count} targets, but runtime target slot IDs support at most 256"
            ),
        }
    }
}

impl Error for CatalogError {}

#[cfg(test)]
mod tests {
    use super::{
        CardCatalog, CatalogError, GrantedAbilityValidationError,
        validate_semantic_spell_presentation,
    };
    use crate::card::{
        AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
        ActivatedAbilityDef, AdditionalCostDef, AlternateSpellKind, AlternativeCastKindDef,
        AlternativeCostDef, AppliedEffectDef, CardBehavior, CardDefinition, CardEffectStatus,
        CardPart, CardPrinting, CardPrintingId, CardSet, CardStructure, DeclarativeAbilityDef,
        DoubleFacedKind, EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef,
        ManaCost, ModeDef, ModeSetDef, ObjectPredicateDef, PlayOptionDef, PlayerRelation,
        PrintedManaCost, SpellForm, TargetConditionDef, TargetPredicate, TargetSlotDef,
        TurnStepDef, ValueDef,
    };
    use crate::{
        AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, Format,
        GrantId, MeldRecipeId, ModeId, PlayOptionId, TargetIndex, TargetSlotId,
    };

    fn definition(id: u16, name: &str, set: CardSet) -> CardDefinition {
        CardDefinition::new(
            CardDefinitionId(id),
            name,
            set,
            false,
            CardBehavior::Unsupported,
        )
    }

    fn target(id: u8, minimum: u8, maximum: u8) -> TargetSlotDef {
        TargetSlotDef {
            id: TargetSlotId(id),
            label: "any target".into(),
            predicate: TargetPredicate::AnyTarget,
            minimum,
            maximum,
            divided_total: None,
        }
    }

    fn mode(id: u8, targets: Vec<TargetSlotDef>) -> ModeDef {
        ModeDef {
            id: ModeId(id),
            label: "test mode".into(),
            targets,
            effect_status: CardEffectStatus::MetadataOnly,
        }
    }

    fn semantic_target(minimum: u8, maximum: u8) -> AbilityTargetDef {
        AbilityTargetDef {
            predicate: AbilityTargetPredicate::AnyTarget,
            minimum,
            maximum,
            divided_total: None,
        }
    }

    fn semantic_mode(targets: Vec<AbilityTargetDef>) -> AbilityDef {
        AbilityDef::spell_with_targets(
            "test mode",
            Box::leak(targets.into_boxed_slice()),
            EffectDef::None,
        )
    }

    fn semantic_modal_definition(
        semantic_modes: Vec<AbilityDef>,
        presentation_modes: Option<ModeSetDef>,
    ) -> CardDefinition {
        let semantic_modes = Box::leak(semantic_modes.into_boxed_slice());
        semantic_spell_definition(
            &AbilityDef::choose_one_spell("Choose one.", semantic_modes),
            presentation_modes,
        )
    }

    fn semantic_spell_definition(
        ability: &AbilityDef,
        mut presentation_modes: Option<ModeSetDef>,
    ) -> CardDefinition {
        let abilities = Box::leak(vec![*ability].into_boxed_slice());
        let rules = crate::CardRules::new_instant(ManaCost::default()).with_abilities(abilities);
        let mut card = definition(1, "Test Modal Spell", CardSet::Alpha);
        set_primary_rules(&mut card, &rules);
        card.play_options = vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Test Modal Spell",
            SpellForm::Part(CardPartId::PRIMARY),
            ManaCost::default(),
            CardEffectStatus::Implemented,
        )];
        if let Some(modes) = &mut presentation_modes {
            for mode in &mut modes.modes {
                mode.effect_status = CardEffectStatus::Implemented;
            }
        }
        card.play_options[0].modes = presentation_modes;
        card
    }

    fn split_definition(fused: Option<PlayOptionId>) -> CardDefinition {
        let mut card = definition(1, "Left // Right", CardSet::Alpha);
        let spell_rules = crate::CardRules::new_instant(ManaCost::default());
        card.rules = spell_rules;
        card.parts[0].rules = spell_rules;
        card.parts
            .push(CardPart::new(CardPartId(1), "Right", spell_rules));
        card.structure = CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused,
        };
        card.play_options[0].label = "Left".into();
        card.play_options[0].mana_cost = Some(ManaCost::default());
        card.play_options.push(PlayOptionDef::cast(
            PlayOptionId(1),
            "Right",
            SpellForm::Part(CardPartId(1)),
            ManaCost::default(),
            CardEffectStatus::Implemented,
        ));
        card
    }

    fn error(card: CardDefinition) -> CatalogError {
        CardCatalog::new([card]).unwrap_err()
    }

    fn set_primary_rules(card: &mut CardDefinition, rules: &crate::CardRules) {
        card.rules = *rules;
        let primary = card.primary_part_id();
        card.parts
            .iter_mut()
            .find(|part| part.id == primary)
            .expect("the test definition has a primary part")
            .rules = *rules;
    }

    fn definition_granting(granted: &'static AbilityDef) -> CardDefinition {
        let abilities = Box::leak(
            vec![AbilityDef::static_ability(
                "This object grants an ability.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(granted),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            )]
            .into_boxed_slice(),
        );
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(abilities);
        set_primary_rules(&mut card, &rules);
        card
    }

    #[test]
    fn primary_and_additional_printings_are_indexed_by_canonical_definition() {
        let id = CardDefinitionId(1);
        let primary = CardPrintingId::new(id, CardSet::Alpha);
        let beta = CardPrinting::new(id, CardSet::Beta);
        let alternate_beta = CardPrinting::with_variant(id, CardSet::Beta, 1);
        let catalog = CardCatalog::with_additional_printings(
            [definition(1, "Test Card", CardSet::Alpha)],
            [beta, alternate_beta],
        )
        .unwrap();

        assert_eq!(catalog.find_by_name(" test card "), Some(id));
        assert_eq!(catalog.get_printing(primary).unwrap().id, primary);
        assert_eq!(catalog.get_printing(beta.id), Some(&beta));
        assert_eq!(
            catalog.get_printing(alternate_beta.id),
            Some(&alternate_beta)
        );
        assert_eq!(catalog.printings_for(id).len(), 3);
        assert!(catalog.has_printing_in(id, CardSet::Alpha));
        assert!(catalog.has_printing_in(id, CardSet::Beta));
        assert!(!catalog.has_printing_in(id, CardSet::Unlimited));
    }

    #[test]
    fn duplicate_printing_ids_are_rejected() {
        let id = CardDefinitionId(1);
        let duplicate = CardPrinting::new(id, CardSet::Alpha);
        assert_eq!(
            CardCatalog::with_additional_printings(
                [definition(1, "Test Card", CardSet::Alpha)],
                [duplicate],
            )
            .unwrap_err(),
            CatalogError::DuplicatePrintingId(duplicate.id)
        );
    }

    #[test]
    fn an_allowed_reprint_makes_the_canonical_identity_format_legal() {
        let id = CardDefinitionId(1);
        let catalog = CardCatalog::with_additional_printings(
            [definition(1, "Test Card", CardSet::Alpha)],
            [CardPrinting::new(id, CardSet::Magic2014)],
        )
        .unwrap();

        assert_eq!(catalog.get(id).unwrap().debut_set, CardSet::Alpha);
        assert!(catalog.is_allowed_in(id, Format::OldSchool9394));
        assert!(catalog.is_allowed_in(id, Format::IsdRtrStandard));
    }

    #[test]
    fn additional_printings_must_reference_a_cataloged_definition() {
        let orphan = CardPrinting::new(CardDefinitionId(2), CardSet::Beta);
        assert_eq!(
            CardCatalog::with_additional_printings(
                [definition(1, "Test Card", CardSet::Alpha)],
                [orphan],
            )
            .unwrap_err(),
            CatalogError::OrphanPrinting(orphan.id)
        );
    }

    #[test]
    fn definition_supplied_printings_must_belong_to_that_definition() {
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let mismatched = CardPrinting::new(CardDefinitionId(2), CardSet::Beta);
        card.printings.push(mismatched);

        assert_eq!(
            CardCatalog::new([card]).unwrap_err(),
            CatalogError::MismatchedPrintingDefinition {
                definition: CardDefinitionId(1),
                printing: mismatched.id,
            }
        );
    }

    #[test]
    fn unknown_definitions_have_no_printings() {
        let catalog = CardCatalog::new([definition(1, "Test Card", CardSet::Alpha)]).unwrap();
        assert!(catalog.printings_for(CardDefinitionId(2)).is_empty());
        assert!(
            catalog
                .get_printing(CardPrintingId::new(CardDefinitionId(2), CardSet::Alpha))
                .is_none()
        );
    }

    #[test]
    fn part_and_play_option_ids_are_unique_within_a_definition() {
        let mut duplicate_part = definition(1, "Test Card", CardSet::Alpha);
        duplicate_part.parts.push(duplicate_part.parts[0].clone());
        assert_eq!(
            error(duplicate_part),
            CatalogError::DuplicatePartId {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
            }
        );

        let mut duplicate_option = definition(1, "Test Card", CardSet::Alpha);
        duplicate_option
            .play_options
            .push(duplicate_option.play_options[0].clone());
        assert_eq!(
            error(duplicate_option),
            CatalogError::DuplicatePlayOptionId {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
            }
        );
    }

    #[test]
    fn incoherent_rules_cannot_enter_the_catalog() {
        let invalid_rules = crate::CardRules::new_land(&[])
            .with_printed_mana_cost_for_test(PrintedManaCost::Cost(ManaCost::default()));

        let mut invalid_compatibility_view = definition(1, "Test Card", CardSet::Alpha);
        invalid_compatibility_view.rules = invalid_rules;
        assert_eq!(
            error(invalid_compatibility_view),
            CatalogError::IncoherentCardRules {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                explanation: "a land cannot have a printed mana cost",
            }
        );

        let mut invalid_part = definition(1, "Test Card", CardSet::Alpha);
        invalid_part.parts[0].rules = invalid_rules;
        assert_eq!(
            error(invalid_part),
            CatalogError::IncoherentCardRules {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                explanation: "a land cannot have a printed mana cost",
            }
        );
    }

    #[test]
    fn compatibility_rules_must_match_the_primary_part() {
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        card.rules = crate::CardRules::new_artifact(ManaCost::default());

        assert_eq!(
            error(card),
            CatalogError::MismatchedPrimaryRules {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
            }
        );
    }

    #[test]
    fn ability_ids_follow_clause_order_within_each_card_part() {
        static ABILITIES: [AbilityDef; 2] = [
            AbilityDef::spell("first", EffectDef::None),
            AbilityDef::not_implemented("second", "Only positional identity matters here."),
        ];
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(&ABILITIES);
        set_primary_rules(&mut card, &rules);

        let attached = card.parts[0].rules.indexed_abilities().collect::<Vec<_>>();
        assert_eq!(attached[0].id, AbilityId(0));
        assert_eq!(attached[1].id, AbilityId(1));
        CardCatalog::new(vec![card]).expect("ordered clauses receive distinct positional IDs");
    }

    #[test]
    fn one_card_part_cannot_define_multiple_spell_abilities() {
        static ABILITIES: [AbilityDef; 2] = [
            AbilityDef::spell("first", EffectDef::None),
            AbilityDef::spell("second", EffectDef::None),
        ];
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(&ABILITIES);
        set_primary_rules(&mut card, &rules);

        assert_eq!(
            error(card),
            CatalogError::MultipleSpellAbilities {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                count: 2,
            }
        );
    }

    #[test]
    fn positional_ability_ids_reject_more_than_their_address_space() {
        let abilities = Box::leak(
            vec![AbilityDef::spell("A spell ability.", EffectDef::None); 257].into_boxed_slice(),
        );
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(abilities);
        set_primary_rules(&mut card, &rules);

        assert_eq!(
            error(card),
            CatalogError::TooManyAbilities {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                count: 257,
            }
        );
    }

    #[test]
    fn grant_ids_reject_more_than_their_structural_address_space() {
        static GRANTED: AbilityDef = AbilityDef::not_implemented(
            "A granted ability.",
            "The test only needs a reusable definition.",
        );
        let effects = Box::leak(
            vec![
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&GRANTED),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                };
                257
            ]
            .into_boxed_slice(),
        );
        let abilities = Box::leak(
            vec![AbilityDef::static_ability(
                "This object receives many abilities.",
                EffectDef::Sequence(effects),
            )]
            .into_boxed_slice(),
        );
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(abilities);
        set_primary_rules(&mut card, &rules);

        assert_eq!(
            error(card),
            CatalogError::TooManyAbilityGrantSites {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                count: 257,
            }
        );
    }

    #[test]
    fn delayed_grants_count_toward_the_structural_address_space() {
        static GRANTED: AbilityDef = AbilityDef::not_implemented(
            "A granted ability.",
            "The test only needs a reusable definition.",
        );
        static GRANT: EffectDef = EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&GRANTED),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        };
        static DELAYED_GRANT: EffectDef = EffectDef::AtNextStep {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
            effect: &GRANT,
        };
        let effects = Box::leak(vec![DELAYED_GRANT; 257].into_boxed_slice());
        let abilities = Box::leak(
            vec![AbilityDef::static_ability(
                "This object schedules many granted abilities.",
                EffectDef::Sequence(effects),
            )]
            .into_boxed_slice(),
        );
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(abilities);
        set_primary_rules(&mut card, &rules);

        assert_eq!(
            error(card),
            CatalogError::TooManyAbilityGrantSites {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                count: 257,
            }
        );
    }

    #[test]
    fn executable_granted_static_abilities_are_rejected_until_fixed_point_evaluation_exists() {
        static GRANTED: AbilityDef =
            AbilityDef::static_ability("This object gets +1/+1.", EffectDef::None);

        assert_eq!(
            error(definition_granting(&GRANTED)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::ExecutableStaticAbility,
            }
        );
    }

    #[test]
    fn granted_ability_validation_reports_nested_structural_paths() {
        static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
        static CHILD: AbilityDef = AbilityDef::activated(
            "This ability grants another ability.",
            &[],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&INVALID),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        );

        assert_eq!(
            error(definition_granting(&CHILD)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY, GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::EmptyText,
            }
        );
    }

    #[test]
    fn granted_ability_validation_follows_sacrifice_continuations() {
        static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
        static THEN: EffectDef = EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&INVALID),
            duration: EffectDurationDef::UntilEndOfTurn,
        };
        static CHILD: AbilityDef = AbilityDef::activated(
            "Sacrifice a permanent, then grant an ability.",
            &[],
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::Any,
                then: Some(&THEN),
                optional: false,
            },
        );

        assert_eq!(
            error(definition_granting(&CHILD)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY, GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::EmptyText,
            }
        );
    }

    #[test]
    fn granted_modal_branches_validate_nested_grants_in_printed_order() {
        static VALID: AbilityDef = AbilityDef::not_implemented(
            "A valid granted ability.",
            "Only nested validation matters in this fixture.",
        );
        static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
        static MODES: [AbilityDef; 2] = [
            AbilityDef::spell(
                "The first mode grants a valid ability.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&VALID),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "The second mode grants an invalid ability.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&INVALID),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ];
        static GRANTED_MODAL: AbilityDef = AbilityDef::choose_one_spell("Choose one.", &MODES);

        assert_eq!(
            error(definition_granting(&GRANTED_MODAL)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY, GrantId(1)],
                problem: GrantedAbilityValidationError::EmptyText,
            }
        );
    }

    #[test]
    fn granted_modal_capacity_counts_grants_across_all_modes() {
        static TERMINAL: AbilityDef = AbilityDef::not_implemented(
            "A terminal granted ability.",
            "The terminal ability is intentionally not executable.",
        );
        let grants = |count| {
            Box::leak(
                vec![
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::GrantAbility(&TERMINAL),
                        duration: EffectDurationDef::UntilEndOfTurn,
                    };
                    count
                ]
                .into_boxed_slice(),
            )
        };
        let modes = Box::leak(
            vec![
                AbilityDef::spell("First mode.", EffectDef::Sequence(grants(128))),
                AbilityDef::spell("Second mode.", EffectDef::Sequence(grants(129))),
            ]
            .into_boxed_slice(),
        );
        let granted_modal = Box::leak(Box::new(AbilityDef::choose_one_spell("Choose one.", modes)));

        assert_eq!(
            error(definition_granting(granted_modal)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::TooManyGrantSites { count: 257 },
            }
        );
    }

    #[test]
    fn granted_ability_validation_checks_zones_mana_targets_and_target_slots() {
        static MANA_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )];
        static NO_ZONES: AbilityDef =
            AbilityDef::activated("An activated ability.", &[], EffectDef::None)
                .with_source_zones(&[]);
        static TARGETED_MANA: AbilityDef = AbilityDef::defined(
            "A targeted mana ability.",
            DeclarativeAbilityDef::ActivatedMana(
                ActivatedAbilityDef::new(&[AbilityCostDef::TapSource]).with_targets(&MANA_TARGETS),
            ),
            EffectDef::None,
        );
        static OUT_OF_RANGE_TARGET: AbilityDef = AbilityDef::activated_with_targets(
            "An activated ability.",
            &[],
            &MANA_TARGETS,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: crate::ValueDef::Constant(1),
            },
        );

        let cases = [
            (&NO_ZONES, GrantedAbilityValidationError::HasNoSourceZone),
            (
                &TARGETED_MANA,
                GrantedAbilityValidationError::ManaAbilityHasTargets,
            ),
            (
                &OUT_OF_RANGE_TARGET,
                GrantedAbilityValidationError::TargetReferenceOutOfBounds {
                    target: TargetIndex(1),
                    target_count: 1,
                },
            ),
        ];
        for (granted, problem) in cases {
            assert_eq!(
                error(definition_granting(granted)),
                CatalogError::InvalidGrantedAbility {
                    definition: CardDefinitionId(1),
                    part: CardPartId::PRIMARY,
                    ability: AbilityId::PRIMARY,
                    grant_path: vec![GrantId::PRIMARY],
                    problem,
                }
            );
        }
    }

    #[test]
    fn target_references_are_validated_through_nested_values() {
        static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )];
        static CONDITION: TargetConditionDef = TargetConditionDef {
            slot: TargetIndex(1),
            object: crate::ObjectPredicateDef::Any,
            then: ValueDef::Constant(1),
            otherwise: ValueDef::Constant(0),
        };
        static ABILITIES: [AbilityDef; 1] = [AbilityDef::spell_with_targets(
            "Use a nested value from the chosen target.",
            &TARGETS,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::IfTargetMatches(&CONDITION),
            },
        )];
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(&ABILITIES);
        set_primary_rules(&mut card, &rules);

        assert_eq!(
            error(card),
            CatalogError::AbilityTargetReferenceOutOfBounds {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                target: TargetIndex(1),
                target_count: 1,
            }
        );
    }

    #[test]
    fn merged_effect_vocabulary_preserves_local_target_bounds() {
        static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )];
        let out_of_range = TargetIndex(1);
        let recipient = EffectRecipientDef::ControllerOfTarget(out_of_range);
        let effects = [
            EffectDef::Tap {
                object: EffectRecipientDef::ObjectsControlledByTarget {
                    object: ObjectPredicateDef::Any,
                    slot: out_of_range,
                },
            },
            EffectDef::SplitPermanentsAndSacrificeAPile { player: recipient },
            EffectDef::Mill {
                player: recipient,
                amount: ValueDef::DividedAmongTargets,
            },
            EffectDef::CannotCastNoncreatureSpellsThisTurn { player: recipient },
            EffectDef::ChooseCardName { object: recipient },
        ];

        for effect in effects {
            assert_eq!(
                super::validate_ability_targets(&TARGETS, effect),
                Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
                    target: out_of_range,
                    target_count: 1,
                })
            );
        }

        super::validate_ability_targets(
            &TARGETS,
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::DividedAmongTargets,
                },
                EffectDef::AdditionalCombatPhase,
            ]),
        )
        .expect("implicit divided values and target-free combat effects add no slot reference");
    }

    #[test]
    fn authored_target_count_fits_the_positional_index_space() {
        let targets = Box::leak(
            vec![
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any),);
                257
            ]
            .into_boxed_slice(),
        );
        let abilities = Box::leak(
            vec![AbilityDef::activated_with_targets(
                "An ability with too many targets.",
                &[],
                targets,
                EffectDef::None,
            )]
            .into_boxed_slice(),
        );
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(abilities);
        set_primary_rules(&mut card, &rules);

        assert_eq!(
            error(card),
            CatalogError::TooManyAbilityTargets {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                count: 257,
            }
        );
    }

    #[test]
    fn nested_grant_capacity_is_validated_per_granted_definition() {
        static TERMINAL: AbilityDef = AbilityDef::not_implemented(
            "A terminal granted ability.",
            "The terminal ability is intentionally not executable.",
        );
        let effects = Box::leak(
            vec![
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&TERMINAL),
                    duration: EffectDurationDef::UntilEndOfTurn,
                };
                257
            ]
            .into_boxed_slice(),
        );
        let child = Box::leak(Box::new(AbilityDef::activated(
            "This ability contains too many nested grant sites.",
            &[],
            EffectDef::Sequence(effects),
        )));

        assert_eq!(
            error(definition_granting(child)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::TooManyGrantSites { count: 257 },
            }
        );
    }

    #[test]
    fn granted_non_declarative_implementations_require_an_explanation() {
        static GRANTED: AbilityDef =
            AbilityDef::activated("An incompletely implemented ability.", &[], EffectDef::None)
                .with_coverage(AbilityCoverageDef::metadata_only(""));

        assert_eq!(
            error(definition_granting(&GRANTED)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::MissingImplementationExplanation,
            }
        );
    }

    #[test]
    fn executable_legacy_procedures_require_custom_effect_execution() {
        static LEGACY: AbilityDef = AbilityDef::activated(
            "An ability routed through the legacy procedure.",
            &[],
            EffectDef::None,
        )
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The test supplies the required legacy-procedure explanation.",
        ))
        .with_legacy_procedure();

        let mut top_level = definition(1, "Test Card", CardSet::Alpha);
        let rules = top_level.rules.with_ability(LEGACY);
        set_primary_rules(&mut top_level, &rules);
        assert_eq!(
            error(top_level),
            CatalogError::LegacyProcedureRequiresCustomExecution {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            }
        );

        assert_eq!(
            error(definition_granting(&LEGACY)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::LegacyProcedureRequiresCustomExecution,
            }
        );
    }

    #[test]
    fn explicitly_tagged_mana_abilities_cannot_declare_targets() {
        static COSTS: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];
        static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )];
        static ABILITIES: [AbilityDef; 1] = [AbilityDef::defined(
            "Target player adds mana.",
            DeclarativeAbilityDef::ActivatedMana(
                ActivatedAbilityDef::new(&COSTS).with_targets(&TARGETS),
            ),
            EffectDef::None,
        )];
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let rules = card.rules.with_abilities(&ABILITIES);
        set_primary_rules(&mut card, &rules);

        assert_eq!(
            error(card),
            CatalogError::ManaAbilityHasTargets {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            }
        );
    }

    #[test]
    fn every_structure_family_rejects_undefined_or_repeated_parts() {
        let invalid_structures = [
            CardStructure::Single {
                main: CardPartId(9),
            },
            CardStructure::Split {
                parts: vec![CardPartId::PRIMARY, CardPartId(9)],
                fused: None,
            },
            CardStructure::Flip {
                normal: CardPartId::PRIMARY,
                flipped: CardPartId(9),
            },
            CardStructure::DoubleFaced {
                front: CardPartId::PRIMARY,
                back: CardPartId(9),
                kind: DoubleFacedKind::Transforming,
            },
            CardStructure::AlternateSpell {
                main: CardPartId::PRIMARY,
                alternate: CardPartId(9),
                kind: AlternateSpellKind::Adventure,
            },
            CardStructure::MeldPart {
                front: CardPartId(9),
                recipe: MeldRecipeId(1),
            },
        ];
        for structure in invalid_structures {
            let mut card = definition(1, "Test Card", CardSet::Alpha);
            card.structure = structure;
            assert!(matches!(
                error(card),
                CatalogError::UndefinedStructurePart {
                    definition: CardDefinitionId(1),
                    part: CardPartId(9),
                }
            ));
        }

        let mut repeated = definition(1, "Test Card", CardSet::Alpha);
        repeated.structure = CardStructure::Flip {
            normal: CardPartId::PRIMARY,
            flipped: CardPartId::PRIMARY,
        };
        assert_eq!(
            error(repeated),
            CatalogError::DuplicateStructurePart {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
            }
        );
    }

    #[test]
    fn spell_forms_must_reference_defined_structural_parts() {
        let mut undefined = definition(1, "Test Card", CardSet::Alpha);
        undefined.play_options[0].form = SpellForm::Part(CardPartId(9));
        assert_eq!(
            error(undefined),
            CatalogError::UndefinedSpellFormPart {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                part: CardPartId(9),
            }
        );

        let mut empty = split_definition(Some(PlayOptionId(2)));
        empty.play_options.push(PlayOptionDef::cast(
            PlayOptionId(2),
            "Left // Right",
            SpellForm::Combined(Vec::new()),
            ManaCost::default(),
            CardEffectStatus::MetadataOnly,
        ));
        assert_eq!(
            error(empty),
            CatalogError::EmptySpellForm {
                definition: CardDefinitionId(1),
                option: PlayOptionId(2),
            }
        );
    }

    #[test]
    fn fused_option_must_exist_and_match_all_split_parts_in_printed_order() {
        assert_eq!(
            error(split_definition(Some(PlayOptionId(2)))),
            CatalogError::MissingFusedPlayOption {
                definition: CardDefinitionId(1),
                option: PlayOptionId(2),
            }
        );

        let mut reversed = split_definition(Some(PlayOptionId(2)));
        reversed.play_options.push(PlayOptionDef::cast(
            PlayOptionId(2),
            "Right // Left",
            SpellForm::Combined(vec![CardPartId(1), CardPartId::PRIMARY]),
            ManaCost::default(),
            CardEffectStatus::MetadataOnly,
        ));
        assert!(matches!(
            error(reversed),
            CatalogError::InvalidFusedPlayOption {
                expected,
                actual: SpellForm::Combined(actual),
                ..
            } if expected == vec![CardPartId::PRIMARY, CardPartId(1)]
                && actual == vec![CardPartId(1), CardPartId::PRIMARY]
        ));

        let mut undeclared = split_definition(None);
        undeclared.play_options.push(PlayOptionDef::cast(
            PlayOptionId(2),
            "Left // Right",
            SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
            ManaCost::default(),
            CardEffectStatus::MetadataOnly,
        ));
        assert_eq!(
            error(undeclared),
            CatalogError::UnexpectedCombinedSpellForm {
                definition: CardDefinitionId(1),
                option: PlayOptionId(2),
            }
        );
    }

    #[test]
    fn mode_and_alternative_cost_ids_are_local_to_options() {
        let modes = ModeSetDef::choose_one(vec![mode(3, Vec::new()), mode(3, Vec::new())]);
        let mut duplicate_mode = definition(1, "Test Card", CardSet::Alpha);
        duplicate_mode.play_options[0].modes = Some(modes);
        assert_eq!(
            error(duplicate_mode),
            CatalogError::DuplicateModeId {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(3),
            }
        );

        let mut nonpositional_mode = definition(1, "Test Card", CardSet::Alpha);
        nonpositional_mode.play_options[0].modes =
            Some(ModeSetDef::choose_one(vec![mode(3, Vec::new())]));
        assert_eq!(
            error(nonpositional_mode),
            CatalogError::NonPositionalModeId {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                expected: ModeId(0),
                actual: ModeId(3),
            }
        );

        let mut duplicate_alternative = definition(1, "Test Card", CardSet::Alpha);
        duplicate_alternative.play_options[0].alternative_costs = vec![
            AlternativeCostDef {
                id: AlternativeCostId(4),
                label: "first".into(),
                mana_cost: ManaCost::default(),
            },
            AlternativeCostDef {
                id: AlternativeCostId(4),
                label: "second".into(),
                mana_cost: ManaCost::default(),
            },
        ];
        assert_eq!(
            error(duplicate_alternative),
            CatalogError::DuplicateAlternativeCostId {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                cost: AlternativeCostId(4),
            }
        );

        let mut alternatives_on_distinct_options = split_definition(None);
        for option in &mut alternatives_on_distinct_options.play_options {
            option.alternative_costs.push(AlternativeCostDef {
                id: AlternativeCostId(4),
                label: "Generic alternative".into(),
                mana_cost: ManaCost::default(),
            });
        }
        CardCatalog::new([alternatives_on_distinct_options])
            .expect("alternative-cost identities are local to a play option");

        let mut duplicate_additional = definition(1, "Test Card", CardSet::Alpha);
        duplicate_additional.play_options[0].additional_costs = vec![
            AdditionalCostDef {
                id: AdditionalCostId(5),
                label: "first".into(),
                mana_cost: None,
            },
            AdditionalCostDef {
                id: AdditionalCostId(5),
                label: "second".into(),
                mana_cost: None,
            },
        ];
        assert_eq!(
            error(duplicate_additional),
            CatalogError::DuplicateAdditionalCostId {
                definition: CardDefinitionId(1),
                cost: AdditionalCostId(5),
            }
        );
    }

    #[test]
    fn alternative_cast_ability_requires_its_derived_cost_projection() {
        let flashback_cost = ManaCost {
            generic: 2,
            blue: 1,
            ..ManaCost::default()
        };
        let missing_abilities = Box::leak(
            vec![AbilityDef::alternative_cast(
                flashback_cost,
                AlternativeCastKindDef::Flashback,
                None,
                EffectDef::None,
            )]
            .into_boxed_slice(),
        );
        let mut missing = definition(1, "Test Card", CardSet::Alpha);
        let rules =
            crate::CardRules::new_instant(ManaCost::default()).with_abilities(missing_abilities);
        set_primary_rules(&mut missing, &rules);
        assert_eq!(
            error(missing),
            CatalogError::MissingAlternativeCostForAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                cost: AlternativeCostId(AbilityId::PRIMARY.0),
            }
        );

        let projected_abilities = Box::leak(
            vec![
                AbilityDef::spell("Draw a card.", EffectDef::None),
                AbilityDef::alternative_cast(
                    flashback_cost,
                    AlternativeCastKindDef::Flashback,
                    None,
                    EffectDef::None,
                ),
            ]
            .into_boxed_slice(),
        );
        let mut projected = definition(1, "Test Card", CardSet::Alpha);
        projected.play_options[0]
            .alternative_costs
            .push(AlternativeCostDef {
                id: AlternativeCostId(1),
                label: "Flashback".into(),
                mana_cost: flashback_cost,
            });
        let rules =
            crate::CardRules::new_instant(ManaCost::default()).with_abilities(projected_abilities);
        set_primary_rules(&mut projected, &rules);
        CardCatalog::new([projected.clone()])
            .expect("the ability's positional ID derives its matching cost projection");

        let mut mismatched_label = projected.clone();
        mismatched_label.play_options[0].alternative_costs[0].label = "Overload".into();
        assert_eq!(
            error(mismatched_label),
            CatalogError::MismatchedAlternativeCostForAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId(1),
                option: PlayOptionId::DEFAULT,
                cost: AlternativeCostId(1),
                expected_label: "Flashback".into(),
                actual_label: "Overload".into(),
                expected_mana_cost: flashback_cost,
                actual_mana_cost: flashback_cost,
            }
        );

        let mut mismatched_mana = projected;
        mismatched_mana.play_options[0].alternative_costs[0].mana_cost = ManaCost::default();
        assert_eq!(
            error(mismatched_mana),
            CatalogError::MismatchedAlternativeCostForAbility {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId(1),
                option: PlayOptionId::DEFAULT,
                cost: AlternativeCostId(1),
                expected_label: "Flashback".into(),
                actual_label: "Flashback".into(),
                expected_mana_cost: flashback_cost,
                actual_mana_cost: ManaCost::default(),
            }
        );
    }

    #[test]
    fn incomplete_alternative_cast_ability_remains_non_executable_catalog_metadata() {
        let alternative = AlternativeCostId(1);
        let abilities = Box::leak(
            vec![
                AbilityDef::spell("Draw a card.", EffectDef::None),
                AbilityDef::alternative_cast(
                    ManaCost::default(),
                    AlternativeCastKindDef::Overload,
                    Some("Draw a card for each opponent."),
                    EffectDef::None,
                )
                .with_coverage(AbilityCoverageDef::metadata_only(
                    "Test-only incomplete overload.",
                )),
            ]
            .into_boxed_slice(),
        );
        let mut definition = definition(1, "Test Card", CardSet::Alpha);
        definition.play_options[0]
            .alternative_costs
            .push(AlternativeCostDef {
                id: alternative,
                label: "Overload".into(),
                mana_cost: ManaCost::default(),
            });
        let rules = crate::CardRules::new_instant(ManaCost::default()).with_abilities(abilities);
        set_primary_rules(&mut definition, &rules);

        let catalog = CardCatalog::new([definition]).expect("incomplete clauses stay cataloged");
        let stored = catalog.get(CardDefinitionId(1)).unwrap();
        assert_eq!(
            stored.implementation_status(),
            crate::ImplementationStatus::Partial,
        );
        assert!(
            !stored.parts[0]
                .rules
                .ability(AbilityId(1))
                .unwrap()
                .is_executable(),
        );
    }

    #[test]
    fn mode_and_target_cardinality_bounds_are_sane() {
        let mut invalid_modes = definition(1, "Test Card", CardSet::Alpha);
        invalid_modes.play_options[0].modes = Some(ModeSetDef {
            minimum: 2,
            maximum: 1,
            may_repeat: false,
            modes: vec![mode(0, Vec::new()), mode(1, Vec::new())],
        });
        assert_eq!(
            error(invalid_modes),
            CatalogError::InvalidModeBounds {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                minimum: 2,
                maximum: 1,
            }
        );

        let mut too_many_modes = definition(1, "Test Card", CardSet::Alpha);
        too_many_modes.play_options[0].modes = Some(ModeSetDef {
            minimum: 1,
            maximum: 2,
            may_repeat: false,
            modes: vec![mode(0, Vec::new())],
        });
        assert_eq!(
            error(too_many_modes),
            CatalogError::TooManyModesWithoutRepetition {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                maximum: 2,
                available: 1,
            }
        );

        let mut invalid_targets = definition(1, "Test Card", CardSet::Alpha);
        invalid_targets.play_options[0].targets = vec![target(0, 2, 1)];
        assert_eq!(
            error(invalid_targets),
            CatalogError::InvalidTargetBounds {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: None,
                slot: TargetSlotId(0),
                minimum: 2,
                maximum: 1,
            }
        );
    }

    #[test]
    fn semantic_spell_modes_require_matching_presentation_mode_ids() {
        let valid = semantic_modal_definition(
            vec![semantic_mode(Vec::new())],
            Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
        );
        CardCatalog::new([valid]).unwrap();

        let missing_presentation = semantic_modal_definition(vec![semantic_mode(Vec::new())], None);
        assert_eq!(
            error(missing_presentation),
            CatalogError::MissingPresentationSpellMode {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(0),
            }
        );

        let missing_semantic = semantic_modal_definition(
            vec![semantic_mode(Vec::new())],
            Some(ModeSetDef::choose_one(vec![
                mode(0, Vec::new()),
                mode(1, Vec::new()),
            ])),
        );
        assert_eq!(
            error(missing_semantic),
            CatalogError::MissingSemanticSpellMode {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(1),
            }
        );
    }

    #[test]
    fn semantic_modal_spell_selection_must_be_possible() {
        let definition = semantic_spell_definition(
            &AbilityDef::modal_spell("Choose one.", &[], 1, 1, false),
            None,
        );

        assert_eq!(
            error(definition),
            CatalogError::InvalidModalSpellSelection {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId(0),
                minimum: 1,
                maximum: 1,
                may_repeat: false,
                available: 0,
            }
        );
    }

    #[test]
    fn executable_nonmodal_spells_reject_presentation_modes() {
        let definition = semantic_spell_definition(
            &AbilityDef::spell("Do the thing.", EffectDef::None),
            Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
        );

        assert_eq!(
            error(definition),
            CatalogError::UnexpectedPresentationSpellModes {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
            }
        );
    }

    #[test]
    fn nonmodal_spell_target_presentations_are_derived_positionally() {
        let targets = Box::leak(vec![semantic_target(1, 1)].into_boxed_slice());
        let ability = AbilityDef::spell_with_targets("Target something.", targets, EffectDef::None);
        let missing = semantic_spell_definition(&ability, None);

        assert_eq!(
            error(missing),
            CatalogError::MissingPresentationSpellTarget {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                target: TargetSlotId(0),
            }
        );

        let mut valid = semantic_spell_definition(&ability, None);
        valid.play_options[0].targets = vec![target(0, 1, 1)];
        CardCatalog::new([valid.clone()]).expect("the positional projection matches");

        let mut mismatched = valid;
        mismatched.play_options[0].targets[0].predicate = TargetPredicate::Player;
        assert!(matches!(
            error(mismatched),
            CatalogError::MismatchedSpellTargetPresentation {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                position: 0,
                ..
            }
        ));
    }

    #[test]
    fn unpresentable_nonmodal_targets_use_only_the_semantic_runtime_definition() {
        let targets = Box::leak(
            vec![AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[],
                    controller: None,
                    owner: None,
                },
            )]
            .into_boxed_slice(),
        );
        let ability = AbilityDef::spell_with_targets("Target a card.", targets, EffectDef::None);
        let semantic_only = semantic_spell_definition(&ability, None);
        CardCatalog::new([semantic_only.clone()])
            .expect("an empty presentation leaves semantic runtime targeting authoritative");

        let mut approximated = semantic_only;
        approximated.play_options[0].targets = vec![target(0, 1, 1)];
        assert_eq!(
            error(approximated),
            CatalogError::UnpresentableSpellTarget {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                target: TargetSlotId(0),
            }
        );
    }

    #[test]
    fn unpresentable_modal_targets_use_only_the_semantic_runtime_definition() {
        let semantic = semantic_mode(vec![AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(crate::CardType::Creature),
                zones: &[crate::ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )]);
        let presentation = ModeSetDef::choose_one(vec![mode(0, Vec::new())]);
        let semantic_only = semantic_modal_definition(vec![semantic], Some(presentation));
        CardCatalog::new([semantic_only.clone()])
            .expect("an empty modal projection leaves semantic runtime targeting authoritative");

        let mut approximated = semantic_only;
        approximated.play_options[0]
            .modes
            .as_mut()
            .expect("the test supplied modal presentation")
            .modes[0]
            .targets = vec![target(0, 1, 1)];
        assert_eq!(
            error(approximated),
            CatalogError::UnpresentableSpellModeTarget {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(0),
                target: TargetSlotId(0),
            }
        );
    }

    #[test]
    fn combined_play_options_reject_modal_constituent_parts() {
        static MODES: [AbilityDef; 1] = [AbilityDef::spell("Test mode.", EffectDef::None)];
        static ABILITIES: [AbilityDef; 1] = [AbilityDef::choose_one_spell("Choose one.", &MODES)];
        let modal_rules =
            crate::CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
        let mut definition = split_definition(Some(PlayOptionId(2)));
        definition.rules = modal_rules;
        definition.parts[0].rules = modal_rules;
        let option = PlayOptionDef::cast(
            PlayOptionId(2),
            "Left // Right",
            SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
            ManaCost::default(),
            CardEffectStatus::MetadataOnly,
        );

        assert_eq!(
            validate_semantic_spell_presentation(&definition, &option),
            Err(CatalogError::CombinedModalSpellUnsupported {
                definition: CardDefinitionId(1),
                option: PlayOptionId(2),
                part: CardPartId::PRIMARY,
            })
        );
    }

    #[test]
    fn semantic_modal_spells_keep_targets_on_their_branches() {
        let mut definition = semantic_modal_definition(
            vec![semantic_mode(Vec::new())],
            Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
        );
        definition.play_options[0].targets = vec![target(0, 1, 1)];

        assert_eq!(
            error(definition),
            CatalogError::UnexpectedModalSpellTargets {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                count: 1,
            }
        );
    }

    #[test]
    fn semantic_spell_mode_selection_rules_cannot_drift_from_presentation() {
        let mismatched = semantic_modal_definition(
            vec![semantic_mode(Vec::new()), semantic_mode(Vec::new())],
            Some(ModeSetDef {
                minimum: 1,
                maximum: 2,
                may_repeat: true,
                modes: vec![mode(0, Vec::new()), mode(1, Vec::new())],
            }),
        );

        assert_eq!(
            error(mismatched),
            CatalogError::MismatchedSpellModeSelection {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                presentation_minimum: 1,
                presentation_maximum: 2,
                presentation_may_repeat: true,
                semantic_minimum: 1,
                semantic_maximum: 1,
                semantic_may_repeat: false,
            }
        );
    }

    #[test]
    fn executable_spell_mode_branches_are_declarative() {
        let custom_mode = AbilityDef::spell("Custom mode", EffectDef::None)
            .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::LightningBolt))
            .with_coverage(AbilityCoverageDef::explained_complete("test custom branch"));
        let definition = semantic_modal_definition(
            vec![custom_mode],
            Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
        );

        assert_eq!(
            error(definition),
            CatalogError::CustomSpellModeImplementation {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId(0),
                mode: ModeId(0),
            }
        );
    }

    #[test]
    fn semantic_spell_mode_targets_require_matching_positions_and_cardinalities() {
        let valid = semantic_modal_definition(
            vec![semantic_mode(vec![semantic_target(1, 1)])],
            Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 1, 1)])])),
        );
        CardCatalog::new([valid]).unwrap();

        let missing_presentation = semantic_modal_definition(
            vec![semantic_mode(vec![semantic_target(1, 1)])],
            Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
        );
        assert_eq!(
            error(missing_presentation),
            CatalogError::MissingPresentationSpellModeTarget {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(0),
                target: TargetSlotId(0),
            }
        );

        let missing_semantic = semantic_modal_definition(
            vec![semantic_mode(Vec::new())],
            Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 1, 1)])])),
        );
        assert_eq!(
            error(missing_semantic),
            CatalogError::MissingSemanticSpellModeTarget {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(0),
                target: TargetSlotId(0),
            }
        );

        let mismatched_cardinality = semantic_modal_definition(
            vec![semantic_mode(vec![semantic_target(1, 1)])],
            Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 0, 1)])])),
        );
        assert_eq!(
            error(mismatched_cardinality),
            CatalogError::MismatchedSpellModeTargetCardinality {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(0),
                target: TargetSlotId(0),
                presentation_minimum: 0,
                presentation_maximum: 1,
                semantic_minimum: 1,
                semantic_maximum: 1,
            }
        );
    }

    #[test]
    fn semantic_spell_mode_presentation_matches_branch_order_and_predicates() {
        let reordered = semantic_modal_definition(
            vec![semantic_mode(vec![
                semantic_target(1, 1),
                semantic_target(1, 1),
            ])],
            Some(ModeSetDef::choose_one(vec![mode(
                0,
                vec![target(1, 1, 1), target(0, 1, 1)],
            )])),
        );
        assert!(matches!(
            error(reordered),
            CatalogError::NonPositionalTargetSlot {
                mode: Some(ModeId(0)),
                expected: TargetSlotId(0),
                actual: TargetSlotId(1),
                ..
            }
        ));

        let mut wrong_predicate = semantic_modal_definition(
            vec![semantic_mode(vec![semantic_target(1, 1)])],
            Some(ModeSetDef::choose_one(vec![mode(0, vec![target(0, 1, 1)])])),
        );
        wrong_predicate.play_options[0]
            .modes
            .as_mut()
            .unwrap()
            .modes[0]
            .targets[0]
            .predicate = TargetPredicate::Player;
        assert!(matches!(
            error(wrong_predicate),
            CatalogError::MismatchedSpellModeTargetPresentation {
                mode: ModeId(0),
                position: 0,
                ..
            }
        ));

        let mut wrong_label = semantic_modal_definition(
            vec![semantic_mode(Vec::new())],
            Some(ModeSetDef::choose_one(vec![mode(0, Vec::new())])),
        );
        wrong_label.play_options[0].modes.as_mut().unwrap().modes[0].label =
            "different mode".into();
        assert!(matches!(
            error(wrong_label),
            CatalogError::MismatchedSpellModeLabel {
                mode: ModeId(0),
                ..
            }
        ));
    }

    #[test]
    fn metadata_only_presentation_modes_do_not_require_semantic_modes() {
        let mut card = definition(1, "Metadata-Only Modal Spell", CardSet::Alpha);
        card.play_options[0].modes = Some(ModeSetDef::choose_one(vec![
            mode(0, vec![target(0, 1, 1)]),
            mode(1, Vec::new()),
        ]));

        CardCatalog::new([card]).unwrap();
    }

    #[test]
    fn composed_target_count_fits_the_runtime_slot_space() {
        let targets = || (0_u8..200).map(|id| target(id, 1, 1)).collect::<Vec<_>>();
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        card.play_options[0].modes = Some(ModeSetDef {
            minimum: 2,
            maximum: 2,
            may_repeat: false,
            modes: vec![mode(0, targets()), mode(1, targets())],
        });

        assert_eq!(
            error(card),
            CatalogError::TooManyInstantiatedTargets {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                count: 400,
            }
        );
    }

    #[test]
    fn modal_target_slots_are_local_to_each_selected_occurrence() {
        let mutually_exclusive = ModeSetDef::choose_one(vec![
            mode(0, vec![target(0, 1, 1)]),
            mode(1, vec![target(0, 1, 1)]),
        ]);
        let mut valid = definition(1, "Test Card", CardSet::Alpha);
        valid.play_options[0].modes = Some(mutually_exclusive);
        CardCatalog::new([valid]).unwrap();

        let mut coexisting = definition(1, "Test Card", CardSet::Alpha);
        coexisting.play_options[0].modes = Some(ModeSetDef {
            minimum: 2,
            maximum: 2,
            may_repeat: false,
            modes: vec![
                mode(0, vec![target(0, 1, 1)]),
                mode(1, vec![target(0, 1, 1)]),
            ],
        });
        CardCatalog::new([coexisting]).unwrap();

        let mut repeatable = definition(1, "Test Card", CardSet::Alpha);
        repeatable.play_options[0].modes = Some(ModeSetDef {
            minimum: 2,
            maximum: 2,
            may_repeat: true,
            modes: vec![mode(0, vec![target(0, 1, 1)])],
        });
        CardCatalog::new([repeatable]).unwrap();
    }
}

#[cfg(test)]
mod name_normalization_tests {
    use super::normalize_name;
    use crate::card;

    #[test]
    fn an_accented_name_is_found_by_either_spelling() {
        let catalog = card::catalog().expect("built-in catalog");
        let printed = catalog
            .find_by_name("Juzám Djinn")
            .expect("the name as printed on the card resolves");
        let typed = catalog
            .find_by_name("Juzam Djinn")
            .expect("the name as players type it resolves");

        assert_eq!(printed, typed);
        assert_eq!(
            catalog.get(printed).expect("definition").name,
            "Juzám Djinn",
            "the catalog stores the printed name; folding only affects lookup"
        );
    }

    #[test]
    fn folding_spells_out_ligatures_instead_of_dropping_them() {
        // Æ is the case that a single-character fold gets wrong: mapping it to
        // "a" would make the unaccented spelling stop matching, which is the
        // opposite of what folding is for.
        assert_eq!(normalize_name("Æther Vial"), "aether vial");
        assert_eq!(normalize_name("Aether Vial"), "aether vial");
    }

    #[test]
    fn folding_covers_the_accents_magic_actually_prints() {
        for (printed, plain) in [
            ("Juzám Djinn", "Juzam Djinn"),
            ("Márton Stromgald", "Marton Stromgald"),
            ("Lim-Dûl's Vault", "Lim-Dul's Vault"),
            ("Séance", "Seance"),
            ("Jötun Grunt", "Jotun Grunt"),
            ("Ærathi Berserker", "Aerathi Berserker"),
        ] {
            assert_eq!(
                normalize_name(printed),
                normalize_name(plain),
                "{printed} and {plain} must resolve to the same card"
            );
        }
    }

    #[test]
    fn normalization_still_trims_and_lowercases() {
        assert_eq!(normalize_name("  Black Lotus  "), "black lotus");
        assert_eq!(normalize_name("BLACK LOTUS"), "black lotus");
    }
}
