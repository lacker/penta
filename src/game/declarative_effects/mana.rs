//! Mana added by a resolving spell or ability.
//!
//! Distinct from the mana runtime, which offers and pays for activations: a
//! resolving object has no enumeration to lean on, so a clause that leaves
//! its colour open asks one question per mana instead.

use super::super::{
    AddManaEffectDef, ColorSet, EffectDef, EffectResolutionContext, Game, Mana, ManaColor,
    ManaSelectionDef, ManaSource, ManaTypeSourceDef, ScopedEffect, StackObject, Target,
};

impl Game {
    pub(super) fn resolve_mana_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::One(kind),
                // A second colour is offered by the mana runtime, which is
                // where an ability making two unlike mana is enumerated.
                also: _,
                amount,
                restrictions,
                spend_effects,
                damage_to_controller,
                amount_override,
                // Read only by the mana runtime, which offers the ability;
                // a triggered mana effect resolving here has a plain amount.
                variable_amount: _,
                // Resolving from the stack, the ability's own controller is
                // the only recipient any current card names.
                recipient: _,
                // A counter-spending land is offered by the mana runtime,
                // which is also where its rider is checked.
                sacrifice_source_when_out_of: _,
            }) => {
                let Some(color) =
                    self.mana_type_for_source(kind, object.source.unwrap_or(object.id))
                else {
                    return;
                };
                let source = object
                    .source
                    .zip(object.ability_origin())
                    .map(|(object, ability)| ManaSource { object, ability });
                let mana = Mana {
                    color,
                    source,
                    restrictions,
                    spend_effects,
                };
                let amount = amount_override
                    .filter(|override_| {
                        self.static_condition_holds(
                            override_.condition,
                            object.controller,
                            object.source.unwrap_or(object.id),
                        )
                    })
                    .map_or(amount, |override_| override_.amount);
                self.add_mana(
                    object.controller,
                    std::iter::repeat_n(mana, usize::from(amount)),
                );
                if damage_to_controller > 0 {
                    self.damage_target_from(
                        object.source.or(Some(object.id)),
                        Some(Target::Player(object.controller)),
                        damage_to_controller,
                    );
                }
            }
            // Both reduce to the same question here. A mana ability
            // enumerates its colours before it is offered, and there a
            // choice names one colour for the whole amount while a
            // combination splits it; a resolution has nowhere to enumerate,
            // so each mana is named as it is added either way.
            EffectDef::AddMana(
                effect @ AddManaEffectDef {
                    mana: ManaSelectionDef::Choice(colors) | ManaSelectionDef::Combination(colors),
                    ..
                },
            ) => {
                // A resolving ability, unlike a mana ability, has nowhere to
                // enumerate its colours in advance: each mana is named as it
                // is added, one question per mana.
                let amount = effect.variable_amount.map_or(effect.amount, |value| {
                    self.effect_value(value, object, context, scoped)
                        .max(0)
                        .try_into()
                        .unwrap_or(u16::MAX)
                });
                let prototype = Mana {
                    color: ManaColor::Colorless,
                    source: object
                        .source
                        .zip(object.ability_origin())
                        .map(|(object, ability)| ManaSource { object, ability }),
                    restrictions: effect.restrictions,
                    spend_effects: effect.spend_effects,
                };
                let mut choosable = ColorSet::empty();
                let ManaTypeSourceDef::Fixed(colors) = colors.source else {
                    return;
                };
                for color in colors {
                    if *color != ManaColor::Colorless {
                        choosable = choosable.with(*color);
                    }
                }
                self.queue_chosen_color_mana(object.controller, prototype, amount, choosable);
            }
            EffectDef::AddManaEqualTo { color, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                self.add_unrestricted_mana(object.controller, color, amount);
            }
            _ => unreachable!("resolve_mana_effect called for another effect"),
        }
    }
}
