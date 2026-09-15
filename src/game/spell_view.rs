//! Read-only characteristics of a proposed or committed spell. Constructing a
//! view neither moves the card nor consumes casting permissions. Consumers
//! evaluate it against the current game at their own casting stage.

use super::{
    AlternativeCastKindDef, CardDefinitionId, CharacteristicContext, Game, GameObjectId,
    ObjectCharacteristics, PlayerId, StackObject, TriggerEventObject, ZoneKind,
};
use crate::card::{FaceDownCharacteristics, SpellForm};

#[derive(Clone, Copy)]
pub(super) struct SpellView<'a> {
    pub(super) object: GameObjectId,
    pub(super) definition: CardDefinitionId,
    pub(super) controller: PlayerId,
    pub(super) owner: PlayerId,
    pub(super) form: &'a SpellForm,
    pub(super) source_zone: Option<ZoneKind>,
    pub(super) x: u16,
    pub(super) face_down: Option<FaceDownCharacteristics>,
    pub(super) bestow: bool,
}

impl Game {
    /// The spell's abilities after face-down characteristics and static
    /// layer-6 grants and removals, including abilities supplied by emblems.
    pub(super) fn for_each_stack_spell_ability(
        &self,
        object: &super::StackObject,
        mut visitor: impl FnMut(super::EffectiveAbility),
    ) {
        if let Some(spell) = self.stack_spell_view(object) {
            self.for_each_spell_view_ability(spell, |ability| {
                if object.colors.is_none() || ability.ability.color_definition().is_none() {
                    visitor(ability);
                }
            });
        }
    }

    pub(super) fn for_each_spell_view_ability(
        &self,
        spell: super::SpellView<'_>,
        mut visitor: impl FnMut(super::EffectiveAbility),
    ) {
        let mut abilities = Vec::new();
        if let Some(face_down) = spell.face_down {
            for attached in face_down.rules().indexed_abilities() {
                abilities.push(super::EffectiveAbility {
                    origin: crate::AbilityOrigin::FaceDown {
                        ability: attached.id,
                    },
                    ability: attached.definition,
                });
            }
        } else {
            let _ = self.visit_printed_definition_abilities(
                spell.definition,
                &super::CharacteristicContext::Stack {
                    form: spell.form.clone(),
                },
                |ability| {
                    abilities.push(ability);
                    std::ops::ControlFlow::Continue(())
                },
            );
        }
        self.apply_static_spell_ability_operations(spell, &mut abilities);
        for ability in abilities {
            visitor(ability);
        }
    }

    pub(super) fn proposed_spell_view<'a>(
        &self,
        controller: PlayerId,
        object: GameObjectId,
        form: &'a SpellForm,
        alternative: Option<AlternativeCastKindDef>,
        x: u16,
    ) -> Option<SpellView<'a>> {
        let (zone, card) = self.card_in_nonbattlefield_zone(object)?;
        Some(SpellView {
            object,
            definition: card.definition,
            controller,
            owner: card.owner,
            form,
            source_zone: Some(zone),
            x,
            face_down: alternative.and_then(AlternativeCastKindDef::face_down),
            bestow: alternative == Some(AlternativeCastKindDef::Bestow),
        })
    }

    pub(super) fn stack_spell_view<'a>(&self, object: &'a StackObject) -> Option<SpellView<'a>> {
        let signature = object.signature.as_ref()?;
        Some(SpellView {
            object: object.id,
            definition: object.card.definition.card_definition()?,
            controller: object.controller,
            owner: object.card.owner,
            form: signature.form(),
            source_zone: object
                .cast
                .as_ref()
                .and_then(|cast| cast.source_zone)
                .map(super::CastSourceZone::zone),
            x: signature.x(),
            face_down: object.face_down,
            bestow: self.was_cast_for_bestow(object),
        })
    }

    pub(super) fn spell_view_characteristics(
        &self,
        spell: SpellView<'_>,
    ) -> Option<TriggerEventObject> {
        if let Some(face_down) = spell.face_down {
            let mut view = self.presentation_trigger_event_object(
                spell.object,
                ObjectCharacteristics::FaceDown { face_down },
                spell.controller,
                false,
            )?;
            self.apply_color_effects_to_object(&mut view);
            return Some(view);
        }
        let mut view = self.printed_trigger_event_object(
            spell.object,
            spell.definition,
            spell.controller,
            &CharacteristicContext::Stack {
                form: spell.form.clone(),
            },
        )?;
        view.mana_value = self.spell_view_mana_value(spell);
        if spell.bestow {
            view.types = Self::without_creature(view.types);
            view.power = None;
            view.toughness = None;
            view.subtypes.insert(crate::card::Subtype::Aura);
            view.subtypes.retain_for_card_types(view.types);
        }
        Some(view)
    }

    pub(super) fn spell_view_mana_value(&self, spell: SpellView<'_>) -> u16 {
        if let Some(face_down) = spell.face_down {
            return face_down.rules().printed_mana_cost().mana_value();
        }
        let Some(definition) = self.catalog.get(spell.definition) else {
            return 0;
        };
        let context = CharacteristicContext::Stack {
            form: spell.form.clone(),
        };
        let Ok(parts) = crate::card::applicable_part_ids_ref(definition, &context) else {
            return 0;
        };
        parts
            .iter()
            .filter_map(|part| definition.part(*part)?.mana_cost())
            .map(|cost| {
                super::mana_cost_value(cost)
                    .saturating_add(spell.x.saturating_mul(cost.x_multiplier))
            })
            .fold(0, u16::saturating_add)
    }
}
