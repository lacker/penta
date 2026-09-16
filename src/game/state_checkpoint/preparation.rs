//! Validate the separately castable, unbacked copies made by preparation.
use super::{CardPartId, CharacteristicSource, Game, GameObjectId, Value};
use crate::card::{AlternateSpellKind, CardStructure, PermanentDesignationDef};

impl Game {
    pub(super) fn restore_prepared_spell_copies(
        &mut self,
        copies: &[(u32, u32, u8)],
        observation: &Value,
    ) -> Result<(), String> {
        for &(copy, source, part) in copies {
            let copy = GameObjectId(copy);
            let source = GameObjectId(source);
            if self
                .prepared_spell_copies
                .iter()
                .any(|(held, _)| *held == copy)
                || self
                    .prepared_spell_copies
                    .iter()
                    .any(|(_, held)| *held == source)
                || !self.battlefield.iter().any(|permanent| {
                    permanent.card.id == source
                        && permanent
                            .designations
                            .contains(&PermanentDesignationDef::Prepared)
                })
            {
                return Err("invalid prepared spell source or duplicate copy".into());
            }
            let card = self
                .players
                .iter_mut()
                .flat_map(|player| &mut player.exile)
                .find(|card| card.id == copy)
                .ok_or("prepared spell copy is not in exile")?;
            let definition = self
                .catalog
                .get(card.definition)
                .ok_or("unknown prepared copy definition")?;
            if !matches!(definition.structure, CardStructure::AlternateSpell {
                alternate, kind: AlternateSpellKind::Prepare, .. } if alternate.0 == part)
            {
                return Err("invalid prepare spell frame".into());
            }
            // The copy retains the frame it had when created even if its
            // prepared permanent subsequently changes its copiable values.
            card.characteristics = CharacteristicSource::PartCopy {
                definition: card.definition,
                part: CardPartId(part),
            };
            self.prepared_spell_copies.push((copy, source));
        }
        for shown in observation["exiles"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|zone| zone.as_array().into_iter().flatten())
        {
            let Some(id) = shown["objectId"].as_u64() else {
                continue;
            };
            let copy = copies.iter().find(|(copy, _, _)| u64::from(*copy) == id);
            if let Some(value) = shown.get("isCopy")
                && value.as_bool() != Some(copy.is_some())
            {
                return Err("exiled copy identity does not match checkpoint".into());
            }
            if let Some(part) = shown.get("partId")
                && part.as_u64() != copy.map(|(_, _, part)| u64::from(*part))
            {
                return Err("exiled copy frame does not match checkpoint".into());
            }
        }
        Ok(())
    }
}
