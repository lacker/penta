impl Game {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_pay_or(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        visibility: ChoiceVisibilityDef,
        definition: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
        if_paid: Option<ScopedEffect>,
        otherwise: Option<ScopedEffect>,
    ) {
        if if_paid.is_none() && otherwise.is_none() {
            return;
        }
        let can_pay = self.can_pay_effect_payment(player, payment);
        if !can_pay && let Some(effect) = otherwise {
            self.resolve_effect_def(effect, object, context);
            return;
        }
        let options = self.payment_options(player, payment, can_pay, "Decline");
        self.queue_decision(
            player,
            object.ability_text().unwrap_or("Pay the cost?"),
            effect_choice_visibility(visibility),
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::PayOr {
                player,
                payment,
                definition,
                object: Box::new(object.clone()),
                context,
                if_paid,
                otherwise,
            },
        );
        self.associate_latest_decision_with(object);
    }
}
