use super::{
    ChoiceIndex, DecisionContinuation, DecisionPreference, DecisionVisibility, EffectDef, Game,
    ObjectPredicateDef, PlayerId, PlayerRelation, ScopedEffect, StackObject, TriggerContext,
    ZoneKind,
};

impl Game {
    fn effect_removes_chosen_permanent(effect: EffectDef, choice: ChoiceIndex) -> bool {
        match effect {
            EffectDef::Destroy {
                object: super::EffectRecipientDef::ChosenPermanent(candidate),
                ..
            }
            | EffectDef::Sacrifice {
                object: super::EffectRecipientDef::ChosenPermanent(candidate),
            }
            | EffectDef::MoveToZone {
                object: super::EffectRecipientDef::ChosenPermanent(candidate),
                zone: ZoneKind::Graveyard | ZoneKind::Exile,
                ..
            } => candidate == choice,
            EffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(|effect| Self::effect_removes_chosen_permanent(effect, choice)),
            EffectDef::Randomized {
                on_success,
                on_failure,
                ..
            } => {
                Self::effect_removes_chosen_permanent(*on_success, choice)
                    || Self::effect_removes_chosen_permanent(*on_failure, choice)
            }
            EffectDef::May(effect)
            | EffectDef::OptionalPayment {
                if_paid: effect, ..
            }
            | EffectDef::UnlessPaid {
                otherwise: effect, ..
            }
            | EffectDef::IfCondition { then: effect, .. }
            | EffectDef::AtNextStep { effect, .. }
            | EffectDef::ChoosePermanent { then: effect, .. } => {
                Self::effect_removes_chosen_permanent(*effect, choice)
            }
            _ => false,
        }
    }

    /// Offers a required, non-targeting permanent choice and resumes the
    /// nested declarative effect with the chosen object in its context.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_permanent_effect_choice(
        &mut self,
        choice: ChoiceIndex,
        chooser: PlayerId,
        predicate: ObjectPredicateDef,
        controller: PlayerRelation,
        object: &StackObject,
        context: TriggerContext,
        effect: ScopedEffect,
    ) {
        let source = object.source.unwrap_or(object.id);
        let candidates = self
            .battlefield
            .iter()
            .filter(|permanent| {
                self.player_relation_matches(
                    permanent.controller,
                    controller,
                    object.controller,
                    context,
                )
            })
            .filter(|permanent| {
                self.trigger_object_matches(
                    predicate,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                )
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();

        if candidates.len() <= 1 {
            let mut context = context;
            context.bind_choice(choice, candidates.first().copied());
            self.resolve_effect_def(effect, object, context);
            return;
        }

        let preference = if Self::effect_removes_chosen_permanent(effect.effect, choice) {
            DecisionPreference::RemovalChoice
        } else {
            DecisionPreference::Neutral
        };
        let options = self.permanent_decision_options(&candidates);
        self.queue_decision(
            chooser,
            "Choose a permanent",
            DecisionVisibility::Public,
            preference,
            1..=1,
            false,
            options,
            DecisionContinuation::ChoosePermanentForEffect {
                choice,
                object: Box::new(object.clone()),
                context,
                effect,
            },
        );
    }
}
