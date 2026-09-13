//! Stable player names and indices for casting-time bindings.
use crate::game::{CastContext, PlayerId};
use std::collections::BTreeMap;

pub(super) fn snapshot_player_bindings(cast: &CastContext) -> BTreeMap<String, usize> {
    cast.player_bindings
        .iter()
        .map(|(name, player)| (name.clone(), player.index()))
        .collect()
}

pub(super) fn restore_player_bindings(
    bindings: &BTreeMap<String, usize>,
    definition: Option<crate::CardDefinitionId>,
    catalog: &crate::CardCatalog,
) -> Result<BTreeMap<String, PlayerId>, String> {
    bindings
        .iter()
        .map(|(name, player)| {
            let declared = definition
                .and_then(|id| catalog.get(id))
                .is_some_and(|definition| {
                    definition
                        .parts
                        .iter()
                        .flat_map(|part| part.rules.ability_clauses())
                        .any(|ability| {
                            let crate::card::DeclarativeAbilityDef::OptionalAdditionalCost(cost) =
                                ability.definition
                            else {
                                return false;
                            };
                            match cost.kind {
                                crate::card::OptionalAdditionalCostKindDef::ChooseOpponent(
                                    binding,
                                ) => binding.label() == Some(name.as_str()),
                                _ => false,
                            }
                        })
                });
            if !declared {
                return Err(format!("undeclared casting player binding {name}"));
            }
            Ok((name.clone(), super::wire::player_from_index(*player)?))
        })
        .collect()
}
