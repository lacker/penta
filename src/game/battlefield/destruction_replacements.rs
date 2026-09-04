// Destruction-specific pieces of the battlefield-exit replacement pipeline.
//
// Split from `exits.rs` for the source-size budget. These remain ordinary
// members of the same `Game` implementation and share the parent's imports.

impl Game {
    fn perform_battlefield_exit_replacement_effect(
        &mut self,
        context: ReplacementEffectContext,
        affected: GameObjectId,
        effect: EffectDef,
    ) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == context.source.object)
        else {
            return;
        };
        let object = StackObject {
            id: permanent.card.id,
            kind: StackObjectKind::TriggeredAbility,
            card: permanent.card.clone(),
            source: Some(permanent.card.id),
            ability: None,
            controller: context.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast: None,
            face_down: None,
            is_copy: false,
        };
        self.resolve_effect_def(
            ScopedEffect::primary(effect),
            &object,
            TriggerContext {
                object: Some(affected),
                object_controller: self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == affected)
                    .map(|permanent| permanent.controller),
                ..TriggerContext::empty()
            },
        );
    }

    pub(super) fn has_battlefield_exit_since(&self, pending_before: usize) -> bool {
        self.pending_decisions
            .get(pending_before..)
            .is_some_and(|pending| {
                pending.iter().any(|pending| {
                    matches!(
                        pending.continuation,
                        DecisionContinuation::BattlefieldExitReplacement { .. }
                    )
                })
            })
    }

    fn replacement_regenerates(effect: ReplacementEffectDef) -> bool {
        match effect {
            ReplacementEffectDef::RegenerateDestroyedObject => true,
            ReplacementEffectDef::Sequence(effects) => {
                effects.iter().copied().any(Self::replacement_regenerates)
            }
            ReplacementEffectDef::Conditional {
                if_true, if_false, ..
            } => if_true
                .iter()
                .chain(if_false.iter())
                .copied()
                .any(Self::replacement_regenerates),
            ReplacementEffectDef::PayOr {
                if_paid,
                if_declined,
                ..
            } => if_paid
                .iter()
                .chain(if_declined.iter())
                .copied()
                .any(Self::replacement_regenerates),
            _ => false,
        }
    }

    fn expire_once_replacement(&mut self, context: ReplacementEffectContext) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == context.source.object)
        else {
            return;
        };
        let fallback = Self::effective_rules_source(permanent);
        let remove = permanent
            .resolved_continuous_effects
            .iter()
            .position(|effect| {
                let ResolvedContinuousEffectKind::Abilities(
                    ResolvedAbilityOperation::Add { grant, .. },
                ) = effect.kind
                else {
                    return false;
                };
                Self::granted_ability_origin(
                    effect.source.object,
                    effect.source.ability,
                    fallback,
                    grant,
                ) == context.source.ability
            });
        if let Some(remove) = remove {
            self.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == context.source.object)
                .expect("the replacement carrier remains on the battlefield")
                .resolved_continuous_effects
                .remove(remove);
        }
    }
}
