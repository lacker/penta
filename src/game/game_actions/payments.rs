use super::super::{
    EffectResolutionContext, Game, GameObjectId, PlayerId, ResolvedEffectPayment, ScopedEffect,
    StackObject, Target,
};
use crate::card::{EffectRecipientDef, GameActionDef};

/// A frozen obligation. Its program supplies semantic selection and execution;
/// quantity and source are captured before the payment decision is offered.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::game) struct ActionPayment {
    pub(in crate::game) program: &'static GameActionDef,
    pub(in crate::game) source: GameObjectId,
    pub(in crate::game) amount: u16,
    object: Box<StackObject>,
    context: EffectResolutionContext,
    scoped: ScopedEffect,
}

impl Game {
    pub(in crate::game) fn resolve_action_payment(
        &self,
        program: &'static GameActionDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
        times: u16,
    ) -> ResolvedEffectPayment {
        if let GameActionDef::Sequence(actions) = program {
            return ResolvedEffectPayment::all(
                actions
                    .iter()
                    .map(|action| {
                        self.resolve_action_payment(action, object, context, scoped, times)
                    })
                    .collect(),
            );
        }
        let choice = program
            .payment_choice()
            .expect("validated action payment program");
        let amount = u16::try_from(
            self.effect_value(choice.amount, object, context, scoped)
                .max(0),
        )
        .unwrap_or(u16::MAX)
        .saturating_mul(times);
        ResolvedEffectPayment::Action(Box::new(ActionPayment {
            program,
            source: object.source.unwrap_or(object.id),
            amount,
            object: Box::new(object.clone()),
            context: context.clone(),
            scoped,
        }))
    }

    pub(in crate::game) fn action_payment_candidates(
        &self,
        player: PlayerId,
        payment: &ActionPayment,
    ) -> Vec<Target> {
        let choice = payment
            .program
            .payment_choice()
            .expect("validated action payment");
        let mut object = payment.object.clone();
        object.controller = player;
        self.effect_recipients(
            EffectRecipientDef::objects(choice.candidates),
            &object,
            &payment.context,
            payment.scoped,
        )
        .into_iter()
        .filter(|target| {
            if !matches!(*choice.then, GameActionDef::GainControl { .. }) {
                return true;
            }
            let Target::Permanent(id) = target else {
                return false;
            };
            self.battlefield
                .iter()
                .find(|permanent| permanent.card.id == *id)
                .is_some_and(|permanent| !self.cannot_change_controller(permanent))
        })
        .collect()
    }

    pub(in crate::game) fn settle_action_payment(
        &mut self,
        player: PlayerId,
        payment: &ActionPayment,
        selected: &[GameObjectId],
    ) -> bool {
        let candidates = self.action_payment_candidates(player, payment);
        let targets = selected.iter().filter_map(|id| candidates.iter().copied().find(|target| {
            matches!(target, Target::Card(candidate) | Target::Permanent(candidate) if candidate == id)
        })).collect::<Vec<_>>();
        if selected.len() != usize::from(payment.amount)
            || targets.len() != selected.len()
            || selected
                .iter()
                .enumerate()
                .any(|(index, id)| selected[..index].contains(id))
        {
            return false;
        }
        let choice = payment
            .program
            .payment_choice()
            .expect("validated action payment");
        // Selection is now committed. Replacement outcomes do not decide
        // whether this was a payment; CompletePayment waits for their work.
        let mut object = payment.object.clone();
        object.controller = player;
        let mut context = payment.context.clone();
        context.bind_object_group(choice.binding, targets);
        self.pending_procedures
            .push_back(super::super::PendingProcedure::ResolveEffects {
                effects: vec![
                    payment
                        .scoped
                        .with_effect(crate::card::EffectDef::Perform(*choice.then)),
                ],
                object,
                context,
            });
        true
    }
}

impl ActionPayment {
    pub(in crate::game) fn verb(&self) -> &'static str {
        match *self
            .program
            .payment_choice()
            .expect("validated action payment")
            .then
        {
            GameActionDef::DiscardCards { .. } => "Discard",
            GameActionDef::Sacrifice { .. } | GameActionDef::SacrificeYours { .. } => "Sacrifice",
            GameActionDef::GainControl { .. } => "Gain control of",
            _ => unreachable!("validated action payment leaf"),
        }
    }
}
