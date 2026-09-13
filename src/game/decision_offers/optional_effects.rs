impl Game {
    /// A "may" cannot choose an impossible object selection (CR 608.2d).
    /// Read the authored minimum before ordinary effect execution clamps it.
    /// Continuations still run normally: later instructions may depend on
    /// objects produced by the first action or on its replacement outcome.
    fn optional_effect_availability(
        &self,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> (bool, DecisionVisibility) {
        match scoped.effect {
            EffectDef::Choose(choice) => {
                self.optional_object_choice_availability(choice, object, context, scoped)
            }
            EffectDef::ChooseExact(choice) => {
                let amount = usize::try_from(
                    self.effect_value(choice.amount, object, context, scoped)
                        .max(0),
                )
                .unwrap_or(usize::MAX);
                self.optional_object_choice_availability(
                    Self::fixed_effect_choice(choice, amount),
                    object,
                    context,
                    scoped,
                )
            }
            EffectDef::Perform(action) => {
                self.optional_action_availability(action, object, context, scoped)
            }
            EffectDef::Discard {
                recipient,
                amount,
                selection,
                ..
            } => {
                let amount =
                    usize::try_from(self.effect_value(amount, object, context, scoped).max(0))
                        .unwrap_or(usize::MAX);
                let available = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .all(|target| match target {
                        Target::Player(player) => {
                            let count = match selection {
                                crate::card::DiscardSelectionDef::RandomMatching(predicate) => self
                                    .players[player.index()]
                                .hand
                                .iter()
                                .filter(|card| {
                                    self.card_object_matches(
                                        *predicate,
                                        card,
                                        ZoneKind::Hand,
                                        object.source.unwrap_or(object.id),
                                    )
                                })
                                .count(),
                                _ => self.players[player.index()].hand.len(),
                            };
                            amount == 0
                                || (count >= amount
                                    && self.can_be_forced_to_discard(player, object.controller))
                        }
                        _ => false,
                    });
                (available, DecisionVisibility::Private)
            }
            EffectDef::Sequence(effects) => effects
                .iter()
                .find(|effect| !matches!(effect, EffectDef::None))
                .map_or((true, DecisionVisibility::Public), |effect| {
                    self.optional_effect_availability(object, context, scoped.with_effect(*effect))
                }),
            EffectDef::BindOutput { effect, .. } => {
                self.optional_effect_availability(object, context, scoped.with_effect(*effect))
            }
            EffectDef::WithCosts { costs, effect } => self.optional_effect_availability(
                object,
                context,
                scoped.with_costs(costs).with_effect(*effect),
            ),
            // This is an eligibility check, not a simulation of resolution.
            // In particular, an empty library does not make drawing illegal.
            _ => (true, DecisionVisibility::Public),
        }
    }

    fn optional_object_choice_availability(
        &self,
        choice: crate::card::ChooseDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> (bool, DecisionVisibility) {
        let available = self
            .effect_choice_decision_state(choice, object, context, scoped)
            .is_some_and(|state| state.candidates.len() >= choice.minimum);
        (available, effect_choice_visibility(choice.visibility))
    }

    fn optional_action_availability(
        &self,
        action: crate::card::GameActionDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> (bool, DecisionVisibility) {
        use crate::card::GameActionDef;
        match action.unnamed() {
            GameActionDef::Choose(choice) => self.optional_object_choice_availability(
                self.fixed_game_action_choice(choice, object, context, scoped),
                object,
                context,
                scoped,
            ),
            GameActionDef::Choice(actions) => {
                let offers = actions
                    .iter()
                    .map(|action| {
                        self.optional_action_availability(*action, object, context, scoped)
                    })
                    .collect::<Vec<_>>();
                let visibility = if offers
                    .iter()
                    .any(|(_, visibility)| *visibility == DecisionVisibility::Private)
                {
                    DecisionVisibility::Private
                } else {
                    DecisionVisibility::Public
                };
                (offers.iter().any(|(available, _)| *available), visibility)
            }
            GameActionDef::Sequence(actions) => actions
                .first()
                .map_or((true, DecisionVisibility::Public), |action| {
                    self.optional_action_availability(*action, object, context, scoped)
                }),
            _ => (true, DecisionVisibility::Public),
        }
    }
}
