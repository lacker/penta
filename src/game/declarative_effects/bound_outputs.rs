use super::{EffectDef, EffectResolutionContext, Game, ScopedEffect, StackObject, Target};

enum ResolvedEffectOutput {
    Objects(Vec<Target>),
}

impl Game {
    pub(in crate::game) fn resolve_bound_output_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        mut context: EffectResolutionContext,
    ) -> EffectResolutionContext {
        let EffectDef::BindOutput { effect, binding } = scoped.effect else {
            unreachable!("resolve_bound_output_effect called for another effect")
        };
        let Some(label) = binding.label() else {
            unreachable!("catalog validation rejected a parent binding on BindOutput")
        };
        context.declare_binding_group_label(label);
        let (mut context, output) =
            self.resolve_effect_output(scoped.with_effect(*effect), object, context);
        if let Some(ResolvedEffectOutput::Objects(value)) = output {
            context.bind_binding_group_label(label, value);
        }
        context
    }

    fn resolve_effect_output(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) -> (EffectResolutionContext, Option<ResolvedEffectOutput>) {
        match scoped.effect {
            EffectDef::Mill { .. } => {
                let (context, objects) = self.resolve_mill_effect(scoped, object, context);
                (context, Some(ResolvedEffectOutput::Objects(objects)))
            }
            EffectDef::MillUntil(_) => {
                let (context, objects) = self.resolve_mill_until_effect(scoped, object, context);
                (context, Some(ResolvedEffectOutput::Objects(objects)))
            }
            EffectDef::SelectAtRandomFromZone { .. } => {
                let (context, objects) =
                    self.resolve_random_zone_selection_effect(scoped, object, context);
                (context, Some(ResolvedEffectOutput::Objects(objects)))
            }
            EffectDef::RevealAtRandomFromHand { .. } => {
                let (context, value) =
                    self.resolve_random_hand_reveal_effect(scoped, object, context);
                (
                    context,
                    Some(ResolvedEffectOutput::Objects(value.into_iter().collect())),
                )
            }
            EffectDef::IfCondition { condition, then } => {
                if self.trigger_condition_holds(
                    condition,
                    object.source.unwrap_or(object.id),
                    object.controller,
                    context.trigger,
                    object.ability.as_ref().map(|ability| ability.origin),
                    Some((object, scoped, &context)),
                ) {
                    self.resolve_effect_output(scoped.with_effect(*then), object, context)
                } else {
                    (context, None)
                }
            }
            EffectDef::IfFormat {
                format,
                then,
                otherwise,
            } => {
                let effect = if self.format == format {
                    then
                } else {
                    otherwise
                };
                self.resolve_effect_output(scoped.with_effect(*effect), object, context)
            }
            EffectDef::Randomized {
                likelihood,
                on_success,
                on_failure,
            } => {
                let effect = if self.rng.sample_probability(likelihood.value()) {
                    on_success
                } else {
                    on_failure
                };
                self.resolve_effect_output(scoped.with_effect(*effect), object, context)
            }
            EffectDef::None => (context, None),
            _ => unreachable!("catalog validation accepted an effect without a bindable output"),
        }
    }
}
