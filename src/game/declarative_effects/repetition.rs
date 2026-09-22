use super::{EffectDef, EffectResolutionContext, Game, ScopedEffect, StackObject, Target};

impl Game {
    pub(super) fn resolve_repeated_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::Repeat {
                mandatory_first: true,
                player,
                effect,
            } => {
                self.resolve_effects_in_order(
                    vec![
                        scoped.with_effect(*effect),
                        scoped.with_effect(EffectDef::Repeat {
                            mandatory_first: false,
                            player,
                            effect,
                        }),
                    ],
                    object,
                    context,
                );
            }
            EffectDef::Repeat { player, .. } => {
                for target in self.effect_recipients(player, object, &context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_optional_effect(
                            player,
                            object,
                            context.fork_resolution(),
                            scoped,
                        );
                    }
                }
            }
            _ => unreachable!("repeated effect resolver requires a repeat"),
        }
    }
}
