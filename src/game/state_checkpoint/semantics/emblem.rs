//! Durable semantic paths for creator-owned emblem characteristics.

use super::super::model::EmblemCharacteristicsLocator;
use super::catalog_ability;
use super::virtual_objects::{authored_virtual_objects, effect_at_path};
use crate::CardCatalog;
use crate::card::{EffectDef, EmblemCharacteristics};

pub(in crate::game::state_checkpoint) fn emblem_characteristics_locator(
    catalog: &CardCatalog,
    expected: EmblemCharacteristics,
) -> Option<EmblemCharacteristicsLocator> {
    authored_emblems(catalog).find_map(|(emblem, locator)| (emblem == expected).then_some(locator))
}

pub(in crate::game::state_checkpoint) fn catalog_emblem_characteristics(
    catalog: &CardCatalog,
    locator: &EmblemCharacteristicsLocator,
) -> Option<EmblemCharacteristics> {
    let creator = catalog_ability(catalog, locator.creator())?;
    match locator {
        EmblemCharacteristicsLocator::EffectPath { effect_path, .. } => {
            let EffectDef::CreateEmblem { emblem, .. } = effect_at_path(&creator, effect_path)?
            else {
                return None;
            };
            Some(emblem)
        }
    }
}

pub(super) fn authored_emblems(
    catalog: &CardCatalog,
) -> impl Iterator<Item = (EmblemCharacteristics, EmblemCharacteristicsLocator)> {
    let objects = authored_virtual_objects(catalog);
    (0..objects.emblems.len()).map(move |index| objects.emblems[index].clone())
}
