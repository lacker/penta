use super::{EffectDef, EffectResolutionContext, Game, ScopedEffect, StackObject, Target};

impl Game {
    pub(super) fn resolve_repeated_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) {
        let EffectDef::Repeat {
            mandatory_first,
            player,
            while_condition,
            effect,
        } = scoped.effect
        else {
            unreachable!("repeated effect resolver requires a repeat");
        };
        if mandatory_first {
            self.resolve_effects_in_order(
                vec![
                    scoped.with_effect(*effect),
                    scoped.with_effect(EffectDef::Repeat {
                        mandatory_first: false,
                        player,
                        while_condition,
                        effect,
                    }),
                ],
                object,
                context,
            );
            return;
        }
        if while_condition.is_some_and(|condition| {
            !self.trigger_condition_holds(
                condition,
                object.source.unwrap_or(object.id),
                object.controller,
                context.trigger,
                object.ability.as_ref().map(|ability| ability.origin),
                Some((object, &scoped, &context)),
            )
        }) {
            return;
        }
        for target in self.effect_recipients(player, object, &context, scoped) {
            if let Target::Player(player) = target {
                self.queue_optional_effect(player, object, context.fork_resolution(), scoped);
            }
        }
    }
}
