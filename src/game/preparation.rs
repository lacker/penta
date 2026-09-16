//! CR 722: preparation designations, distinct exiled copies, and their live
//! controller-relative cast permission. Physical cards never use this frame.
use super::{
    CardInstance, CharacteristicSource, ExilePlayCost, ExilePlayPermission, Game, GameObjectId,
    ObjectBacking, Permanent, PlayerId,
};
use crate::card::{
    AlternateSpellKind, CardStructure, PermanentDesignationDef, SpellForm, ZoneKind,
};
use crate::{CardDefinitionId, CardPartId, ObjectCharacteristics, PlayOptionDef};

impl Game {
    pub(super) fn prepare_spell_characteristics(
        &self,
        permanent: &Permanent,
    ) -> Option<(CardDefinitionId, CardPartId)> {
        let ObjectCharacteristics::Card { definition, .. } =
            Self::effective_rules_source(permanent)
        else {
            return None;
        };
        let CardStructure::AlternateSpell {
            alternate,
            kind: AlternateSpellKind::Prepare,
            ..
        } = self.catalog.get(definition)?.structure
        else {
            return None;
        };
        Some((definition, alternate))
    }

    pub(super) fn part_copy(&self, id: GameObjectId) -> Option<CardPartId> {
        let card = self
            .card_in_nonbattlefield_zone(id)
            .map(|(_, card)| card)
            .or_else(|| match self.retired_objects.get(&id) {
                Some(super::RetiredObject::Card(card)) => Some(&card.card),
                _ => None,
            })?;
        match card.characteristics {
            CharacteristicSource::PartCopy { part, .. } => Some(part),
            _ => None,
        }
    }

    pub(super) fn card_can_use_spell_form(
        &self,
        card: &CardInstance,
        option: &PlayOptionDef,
    ) -> bool {
        if let CharacteristicSource::PartCopy { part, .. } = card.characteristics {
            return option.form == SpellForm::Part(part);
        }
        self.catalog
            .get(card.definition)
            .is_some_and(|definition| match definition.structure {
                CardStructure::AlternateSpell {
                    alternate,
                    kind: AlternateSpellKind::Prepare,
                    ..
                } => option.form != SpellForm::Part(alternate),
                _ => true,
            })
    }

    pub(super) fn set_permanent_designation(
        &mut self,
        source: GameObjectId,
        designation: PermanentDesignationDef,
        present: bool,
    ) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        if present
            && (permanent.designations.contains(&designation)
                || (designation == PermanentDesignationDef::Prepared
                    && self.prepare_spell_characteristics(permanent).is_none()))
        {
            return;
        }
        let permanent = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
            .unwrap();
        permanent.designations.retain(|held| *held != designation);
        if present {
            permanent.designations.push(designation);
        }
        if designation == PermanentDesignationDef::Prepared {
            if present {
                self.create_prepared_spell(source);
            } else {
                self.prune_prepared_spell_copies();
            }
        }
    }

    pub(super) fn create_prepared_spell(&mut self, source: GameObjectId) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        if !permanent
            .designations
            .contains(&PermanentDesignationDef::Prepared)
        {
            return;
        }
        let Some((definition, part)) = self.prepare_spell_characteristics(permanent) else {
            return;
        };
        let owner = permanent.controller;
        let id = self.allocate_object_id();
        self.players[owner.index()].exile.push(CardInstance {
            id,
            definition,
            owner,
            backing: ObjectBacking::None,
            characteristics: CharacteristicSource::PartCopy { definition, part },
            counters: super::counters::Counters::new(),
        });
        self.prepared_spell_copies.push((id, source));
    }

    pub(super) fn prune_prepared_spell_copies(&mut self) {
        let expired = self
            .prepared_spell_copies
            .iter()
            .filter(|(_, source)| {
                !self.battlefield.iter().any(|permanent| {
                    permanent.card.id == *source
                        && permanent
                            .designations
                            .contains(&PermanentDesignationDef::Prepared)
                })
            })
            .map(|(copy, _)| *copy)
            .collect::<Vec<_>>();
        for player in &mut self.players {
            player.exile.retain(|card| !expired.contains(&card.id));
            // A copy moved elsewhere ceases to exist the next time SBAs are
            // checked. None of those moves turns it into a physical card.
            player.hand.retain(|card| {
                !matches!(card.characteristics, CharacteristicSource::PartCopy { .. })
            });
            player.library.retain(|card| {
                !matches!(card.characteristics, CharacteristicSource::PartCopy { .. })
            });
            player.graveyard.retain(|card| {
                !matches!(card.characteristics, CharacteristicSource::PartCopy { .. })
            });
        }
        self.prepared_spell_copies.retain(|(copy, _)| {
            self.players
                .iter()
                .any(|player| player.exile.iter().any(|card| card.id == *copy))
        });
    }

    pub(super) fn prepared_cast_permission(
        &self,
        card: GameObjectId,
        player: PlayerId,
    ) -> Option<ExilePlayPermission> {
        let (_, source) = self
            .prepared_spell_copies
            .iter()
            .find(|(copy, _)| *copy == card)?;
        self.battlefield.iter().find(|permanent| {
            permanent.card.id == *source
                && permanent.controller == player
                && permanent
                    .designations
                    .contains(&PermanentDesignationDef::Prepared)
        })?;
        Some(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: crate::ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            spend_any_type: false,
            maximum_spell_mana_value: None,
            condition: None,
            until_holder_cleanup: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: false,
            grants_haste: false,
        })
    }
}
