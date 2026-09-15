use super::{
    AppliedEffectDef, CharacteristicOperationDef, ColorSet, ControlFlow, Game, ManaColor,
    ResolvedContinuousEffectKind, RetiredObject, SetOperationDef, StaticAffectedObject,
    StaticAppliedEffect, StaticEffectKind, StaticSetCharacteristicLayerGuard, TriggerEventObject,
    ZoneKind,
};

impl Game {
    pub(super) fn static_color_operation(
        &self,
        applied: &StaticAppliedEffect,
    ) -> Option<SetOperationDef<ColorSet>> {
        match applied.effect {
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(operation)) => {
                Some(applied.text_words.color_operation(operation))
            }
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Color(operation)) => {
                let resolve = |kind| {
                    self.mana_type_for_source(kind, applied.source)
                        .map(|color| {
                            if color == ManaColor::Colorless {
                                ColorSet::empty()
                            } else {
                                ColorSet::from_colors(&[color])
                            }
                        })
                };
                Some(match operation {
                    SetOperationDef::Set(kind) => SetOperationDef::Set(resolve(kind)?),
                    SetOperationDef::Add(kind) => SetOperationDef::Add(resolve(kind)?),
                    SetOperationDef::Remove(kind) => SetOperationDef::Remove(resolve(kind)?),
                })
            }
            _ => None,
        }
    }

    /// All nonbattlefield color readers share this layer-5 walk. Resolved
    /// spell effects keep their order relative to static color changes;
    /// copying a spell copies only its intrinsic rules and copy exceptions.
    pub(in crate::game) fn apply_color_effects_to_object(&self, object: &mut TriggerEventObject) {
        if object.zone == ZoneKind::Battlefield {
            return;
        }
        let spell = self
            .stack
            .iter()
            .find(|spell| spell.id == object.id)
            .or_else(|| match self.retired_objects.get(&object.id) {
                Some(RetiredObject::Stack(spell)) => Some(spell.as_ref()),
                _ => None,
            });
        if let Some(colors) = spell.and_then(|spell| spell.last_known_colors) {
            object.colors = colors.to_flags();
            return;
        }
        let mut colors = spell.and_then(|spell| spell.colors).unwrap_or_else(|| {
            ManaColor::COLORS
                .into_iter()
                .zip(object.colors)
                .filter_map(|(color, present)| present.then_some(color))
                .fold(ColorSet::empty(), ColorSet::with)
        });
        let mut operations = spell
            .into_iter()
            .flat_map(|spell| &spell.resolved_continuous_effects)
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .filter_map(|effect| match effect.kind {
                ResolvedContinuousEffectKind::Colors(operation) => {
                    Some((effect.timestamp, effect.component_order, operation))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if let Some(_pass) = StaticSetCharacteristicLayerGuard::enter() {
            let owner = spell
                .map(|spell| spell.card.owner)
                .or_else(|| {
                    self.card_in_nonbattlefield_zone(object.id)
                        .map(|(_, card)| card.owner)
                })
                .unwrap_or(object.controller);
            let mut baseline = object.clone();
            baseline.colors = colors.to_flags();
            let result = self.visit_battlefield_static_applied_effects_for_object(
                StaticAffectedObject::Object {
                    characteristics: &baseline,
                    controller: (object.zone == ZoneKind::Stack).then_some(object.controller),
                    owner,
                    zone: object.zone,
                    is_spell: object.zone == ZoneKind::Stack,
                },
                StaticEffectKind::Colors,
                |applied| {
                    if let Some(operation) = self.static_color_operation(&applied) {
                        operations.push((applied.timestamp, applied.component_order, operation));
                    }
                    ControlFlow::Continue(())
                },
            );
            debug_assert!(result.is_continue());
        }
        operations.sort_by_key(|(timestamp, order, _)| (*timestamp, *order));
        for (_, _, operation) in operations {
            colors = Self::apply_color_operation(colors, operation);
        }
        object.colors = colors.to_flags();
    }
}
