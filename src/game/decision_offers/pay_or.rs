impl Game {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_pay_or(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        payment_provenance: Option<super::PaymentProvenance>,
        visibility: ChoiceVisibilityDef,
        definition: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
        if_paid: Option<ScopedEffect>,
        otherwise: Option<ScopedEffect>,
    ) {
        if let ResolvedEffectPayment::Choice(choices) = &payment
            && choices.iter().all(|choice| matches!(choice, ResolvedEffectPayment::Action(payment) if payment.program.public_alternative_supported()))
        {
            self.queue_action_choice(player, choices.clone(), None, definition, object, context);
            return;
        }
        if if_paid.is_none() && otherwise.is_none() && payment_provenance.is_none() {
            return;
        }
        let can_pay = self.can_pay_effect_payment(player, payment.clone());
        if !can_pay && let Some(effect) = otherwise {
            if let Some(provenance) = payment_provenance {
                self.capture_payment_not_paid(object, player, provenance);
            }
            self.resolve_effect_def(effect, object, context);
            return;
        }
        let options = self.payment_options(player, payment.clone(), can_pay, "Decline");
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
                payment_provenance,
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

impl Game {
    pub(in crate::game) fn complete_effect_payment(
        &mut self,
        player: PlayerId,
        provenance: Option<super::PaymentProvenance>,
        paid: Option<&super::EffectPaymentReceipt>,
        scoped: ScopedEffect,
        object: &StackObject,
        mut context: EffectResolutionContext,
    ) {
        let EffectDef::PayOr(definition) = scoped.effect else {
            unreachable!("payment completion retains its authored offer")
        };
        if let Some(provenance) = provenance {
            match paid {
                Some(receipt) => {
                    self.capture_payment_paid(object, player, provenance, &receipt.mana_spent);
                }
                None => self.capture_payment_not_paid(object, player, provenance),
            }
        }
        context.paid_amount = paid.map(|receipt| receipt.paid_amount);
        let branch = if paid.is_some() {
            definition.if_paid
        } else {
            definition.otherwise
        };
        if let Some(effect) = branch {
            self.resolve_nested_effect_before_later(scoped.with_effect(*effect), object, context);
        }
        if paid.is_some() {
            self.capture_optional_effect_taken(object);
        }
    }
}
