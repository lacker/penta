use super::{
    PreparedCatalog, PreparedEffect, PreparedStaticAbility, PreparedStaticApplication,
    PreparedStaticComponent, PreparedStaticLane, PreparedStaticProgram,
};
use crate::card::EffectExecutorDef;
use crate::{
    AbilityDef, AbilityOperationDef, AppliedEffectDef, CardCatalog, CharacteristicOperationDef,
    DeclarativeAbilityDef, EffectDef, GrantId, TriggerConditionDef, ValueDef,
};

pub(crate) fn compile_catalog(catalog: &CardCatalog) -> PreparedCatalog {
    let mut prepared = PreparedCatalog::default();
    for definition in catalog.definitions() {
        for part in &definition.parts {
            prepared.insert_static_program(
                definition.id,
                part.id,
                compile_static_program(part.rules.ability_clauses()),
            );
        }
    }
    prepared
}

pub(crate) fn compile_effect(executor: EffectExecutorDef) -> Option<PreparedEffect> {
    match executor {
        EffectExecutorDef::DrawCards(ValueDef::Constant(count)) => u16::try_from(count)
            .ok()
            .map(|count| PreparedEffect::DrawCards { count }),
        EffectExecutorDef::DrawCards(_) => None,
    }
}

fn compile_static_program(abilities: &[AbilityDef]) -> PreparedStaticProgram {
    let mut prepared_abilities = Vec::new();
    for (index, ability) in abilities.iter().copied().enumerate() {
        let DeclarativeAbilityDef::Static(definition) = ability.definition else {
            continue;
        };
        if !ability.is_executable() {
            continue;
        }
        let Some(effect) = ability.declarative_effect() else {
            continue;
        };
        let mut compiler = StaticAbilityCompiler::default();
        let prepared = compiler.compile_effect(effect);
        prepared_abilities.push(PreparedStaticAbility {
            id: crate::AbilityId::from_index(index)
                .expect("validated card parts contain at most 256 abilities"),
            source_zones: definition.source_zones,
            reference_effect: effect,
            applications: prepared.then(|| compiler.applications.into_boxed_slice()),
        });
    }
    PreparedStaticProgram {
        supplies_land_type_effect: abilities.iter().copied().any(|ability| {
            ability.is_executable()
                && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                && ability
                    .declarative_effect()
                    .is_some_and(effect_contains_land_type_operation)
        }),
        abilities: prepared_abilities.into_boxed_slice(),
    }
}

#[derive(Default)]
struct StaticAbilityCompiler {
    applications: Vec<PreparedStaticApplication>,
    trigger_conditions: Vec<(TriggerConditionDef, bool)>,
    next_component_order: u16,
    next_grant: usize,
}

impl StaticAbilityCompiler {
    fn compile_effect(&mut self, effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => {
                let mut prepared = true;
                for effect in effects {
                    prepared &= self.compile_effect(*effect);
                }
                prepared
            }
            effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
                let conditional = effect
                    .conditional()
                    .expect("conditional variants expose their shared shape");
                self.trigger_conditions.push((*conditional.condition, true));
                let then_prepared = self.compile_effect(*conditional.then);
                self.trigger_conditions.pop();
                let otherwise_prepared = if let Some(otherwise) = conditional.otherwise {
                    self.trigger_conditions
                        .push((*conditional.condition, false));
                    let prepared = self.compile_effect(*otherwise);
                    self.trigger_conditions.pop();
                    prepared
                } else {
                    true
                };
                then_prepared && otherwise_prepared
            }
            // Whether this subtree exists structurally depends on live game
            // state. It can therefore shift every later component order and
            // grant ID in the same ability, so the whole ability stays on the
            // reference walker until preparation models that control flow.
            EffectDef::ConditionalStatic(_) => false,
            EffectDef::StaticApply { recipient, effect } => {
                self.compile_application(recipient, effect);
                true
            }
            _ => true,
        }
    }

    fn compile_application(
        &mut self,
        recipient: crate::EffectRecipientDef,
        effect: AppliedEffectDef,
    ) {
        let starts_in_type_layer = applied_effect_starts_in_type_layer(effect);
        let mut components = Vec::new();
        let mut lanes = 0;
        self.compile_components(effect, &mut components, &mut lanes);
        if components.is_empty() {
            return;
        }
        self.applications.push(PreparedStaticApplication {
            recipient,
            starts_in_type_layer,
            trigger_conditions: self.trigger_conditions.clone().into_boxed_slice(),
            components: components.into_boxed_slice(),
            lanes,
        });
    }

    fn compile_components(
        &mut self,
        effect: AppliedEffectDef,
        components: &mut Vec<PreparedStaticComponent>,
        lanes: &mut u8,
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    self.compile_components(*effect, components, lanes);
                }
            }
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => {
                let component_order = self.next_component_order;
                self.next_component_order = self
                    .next_component_order
                    .checked_add(1)
                    .expect("one static ability contains at most 65,536 applied components");
                let grant = if matches!(
                    effect,
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                        AbilityOperationDef::Add(_)
                    ))
                ) {
                    let grant = GrantId::from_index(self.next_grant)
                        .expect("one static ability contains at most 256 grant sites");
                    self.next_grant += 1;
                    Some(grant)
                } else {
                    None
                };
                let lane = static_lane(effect);
                *lanes |= lane.mask();
                components.push(PreparedStaticComponent {
                    effect,
                    grant,
                    component_order,
                    lane,
                });
            }
        }
    }
}

fn static_lane(effect: AppliedEffectDef) -> PreparedStaticLane {
    match effect {
        AppliedEffectDef::Composite(_) => {
            unreachable!("composites are flattened before assigning static lanes")
        }
        AppliedEffectDef::Rule(_) => PreparedStaticLane::Rules,
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(_)) => {
            PreparedStaticLane::CardTypes
        }
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::Colors(_) | CharacteristicOperationDef::Color(_),
        ) => PreparedStaticLane::Colors,
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(_)) => {
            PreparedStaticLane::Abilities
        }
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::CreatureTypes(_) | CharacteristicOperationDef::Subtypes(_),
        ) => PreparedStaticLane::Subtypes,
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(_)) => {
            PreparedStaticLane::PowerToughness
        }
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::BasicLandTypes(_)
            | CharacteristicOperationDef::ChosenBasicLandType,
        ) => PreparedStaticLane::Other,
    }
}

fn applied_effect_starts_in_type_layer(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .copied()
            .any(applied_effect_starts_in_type_layer),
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::ChosenBasicLandType
            | CharacteristicOperationDef::BasicLandTypes(_)
            | CharacteristicOperationDef::CardTypes(_)
            | CharacteristicOperationDef::CreatureTypes(_)
            | CharacteristicOperationDef::Subtypes(_),
        ) => true,
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::Abilities(_)
            | CharacteristicOperationDef::Color(_)
            | CharacteristicOperationDef::Colors(_)
            | CharacteristicOperationDef::PowerToughness(_),
        )
        | AppliedEffectDef::Rule(_) => false,
    }
}

fn effect_contains_land_type_operation(effect: EffectDef) -> bool {
    match effect {
        EffectDef::Sequence(effects) => effects
            .iter()
            .copied()
            .any(effect_contains_land_type_operation),
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            effect_contains_land_type_operation(*conditional.then)
                || conditional
                    .otherwise
                    .is_some_and(|otherwise| effect_contains_land_type_operation(*otherwise))
        }
        EffectDef::ConditionalStatic(conditional) => {
            applied_effect_contains_land_type_operation(conditional.then.effect)
        }
        EffectDef::StaticApply { effect, .. } => {
            applied_effect_contains_land_type_operation(effect)
        }
        _ => false,
    }
}

fn applied_effect_contains_land_type_operation(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .copied()
            .any(applied_effect_contains_land_type_operation),
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::BasicLandTypes(_)
            | CharacteristicOperationDef::ChosenBasicLandType,
        ) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::abilities;

    #[test]
    fn dynamic_draw_collapses_the_whole_prepared_root() {
        assert_eq!(
            compile_effect(EffectExecutorDef::DrawCards(ValueDef::ChosenX)),
            None
        );
    }

    #[test]
    fn constant_draw_prepares_to_an_intrinsic() {
        let prepared = abilities::prepared_draw_cards(ValueDef::Constant(3));
        assert_eq!(
            compile_effect(prepared.executor),
            Some(PreparedEffect::DrawCards { count: 3 })
        );
        assert_eq!(
            prepared.effect,
            EffectDef::DrawCards {
                recipient: crate::EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            }
        );
    }
}
