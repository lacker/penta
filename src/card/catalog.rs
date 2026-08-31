mod error;
mod name;
mod validation;

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use self::name::normalize_name;
use self::validation::validate_composition;
use super::{CardDefinition, CardPrinting, CardPrintingId, CardSet, CardStructure};
use crate::{CardDefinitionId, Format};

pub use self::error::{
    CatalogError, EffectSubjectKind, GrantedAbilityValidationError, MismatchedAdditionalCost,
    MismatchedAlternativeCost,
};

#[cfg(test)]
use self::validation::{
    validate_ability_targets, validate_replacement_ability_targets,
    validate_semantic_spell_presentation,
};

/// A catalog is immutable once built, and callers pass it around by value —
/// a game, a policy, and the protocol facade each hold one. Sharing the maps
/// behind an `Arc` makes those clones a refcount bump instead of a deep copy
/// of every definition.
#[derive(Clone, Debug, Default)]
pub struct CardCatalog {
    entries: Arc<CatalogEntries>,
    prepared: Arc<OnceLock<Arc<crate::prepared_engine::PreparedCatalog>>>,
}

#[derive(Debug, Default)]
struct CatalogEntries {
    definitions: Vec<CardDefinition>,
    dense_definition_indices: Vec<Option<usize>>,
    sparse_definition_indices: HashMap<CardDefinitionId, usize>,
    ids_by_name: HashMap<String, CardDefinitionId>,
    definition_by_printing: HashMap<CardPrintingId, CardDefinitionId>,
}

impl CatalogEntries {
    fn definition_index(&self, definition: CardDefinitionId) -> Option<usize> {
        let raw = definition.get();
        if let Ok(dense) = u16::try_from(raw) {
            self.dense_definition_indices
                .get(usize::from(dense))
                .copied()
                .flatten()
        } else {
            self.sparse_definition_indices.get(&definition).copied()
        }
    }

    fn definition(&self, definition: CardDefinitionId) -> Option<&CardDefinition> {
        self.definitions.get(self.definition_index(definition)?)
    }

    fn insert_definition(&mut self, definition: CardDefinition) {
        let raw = definition.id.get();
        let index = self.definitions.len();
        if let Ok(dense) = u16::try_from(raw) {
            let slot = usize::from(dense);
            if self.dense_definition_indices.len() <= slot {
                self.dense_definition_indices.resize(slot + 1, None);
            }
            self.dense_definition_indices[slot] = Some(index);
        } else {
            self.sparse_definition_indices.insert(definition.id, index);
        }
        self.definitions.push(definition);
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

    /// Freezes the public definition order once construction and printing
    /// attachment are complete. Runtime consumers ask for this order often;
    /// sorting a fresh vector on every observation made that read needlessly
    /// scale with the complete catalog.
    fn sort_definitions_by_id(&mut self) {
        self.definitions
            .sort_unstable_by_key(|definition| definition.id);
        self.dense_definition_indices.clear();
        self.sparse_definition_indices.clear();
        for (index, definition) in self.definitions.iter().enumerate() {
            let raw = definition.id.get();
            if let Ok(dense) = u16::try_from(raw) {
                let slot = usize::from(dense);
                if self.dense_definition_indices.len() <= slot {
                    self.dense_definition_indices.resize(slot + 1, None);
                }
                self.dense_definition_indices[slot] = Some(index);
            } else {
                self.sparse_definition_indices.insert(definition.id, index);
            }
        }
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
            let normalized_name = normalize_name(&definition.name);
            if entries.ids_by_name.contains_key(&normalized_name) {
                return Err(CatalogError::DuplicateName(definition.name));
            }
            validate_composition(&definition)?;
            let front_alias = if matches!(definition.structure, CardStructure::DoubleFaced { .. }) {
                definition.primary_part().map(|part| part.name.clone())
            } else {
                None
            };
            if let Some(front_alias) = &front_alias {
                let normalized_alias = normalize_name(front_alias);
                if normalized_alias != normalized_name
                    && entries.ids_by_name.contains_key(&normalized_alias)
                {
                    return Err(CatalogError::DuplicateName(front_alias.clone()));
                }
            }
            let supplied_printings = std::mem::take(&mut definition.printings);
            definition_printings.extend(
                supplied_printings
                    .into_iter()
                    .map(|printing| (definition.id, printing)),
            );
            entries.ids_by_name.insert(normalized_name, definition.id);
            if let Some(front_alias) = front_alias {
                entries
                    .ids_by_name
                    .insert(normalize_name(&front_alias), definition.id);
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
        entries.sort_definitions_by_id();
        Ok(Self {
            entries: Arc::new(entries),
            prepared: Arc::new(OnceLock::new()),
        })
    }

    pub(crate) fn prepared_catalog(&self) -> Arc<crate::prepared_engine::PreparedCatalog> {
        self.prepared
            .get_or_init(|| Arc::new(crate::prepared_engine::compile_catalog(self)))
            .clone()
    }

    #[must_use]
    pub fn get(&self, id: CardDefinitionId) -> Option<&CardDefinition> {
        self.entries.definition(id)
    }

    /// Every definition in the catalog, ordered by id so consumers see a
    /// stable listing.
    #[must_use]
    pub fn definitions(&self) -> Vec<&CardDefinition> {
        self.entries.definitions.iter().collect()
    }

    /// Looks up a card definition ID by its case-insensitive canonical name.
    /// Double-faced cards also accept their front-face name for deck-list
    /// compatibility.
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

#[cfg(test)]
mod tests;
