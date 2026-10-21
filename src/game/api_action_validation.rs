// Known-action validation shares api.rs imports.
impl Game {
    /// Validates an action against the current state without mutating the game.
    ///
    /// Unlike [`legal_actions`], this also validates the option IDs supplied to
    /// a bounded [`Action::ChooseDecision`] selection without expanding every
    /// possible combination into a vector.
    #[must_use]
    #[allow(clippy::too_many_lines)] // Keep the action authority dispatch together.
    pub fn is_legal_action(&self, player: PlayerId, action: &Action) -> bool {
        if self.between_games() {
            return self.match_action_is_legal(player, action);
        }
        if self.result.is_some() {
            return false;
        }
        match action {
            Action::Concede => return true,
            Action::PassPriority => return self.has_action_priority(player),
            Action::CastSpell { card, choices, sacrifices } => {
                return self.cast_action_window(player, *card)
                    && self.validated_cast_signature(player, *card, choices, sacrifices).is_some();
            }
            Action::PlayLand { card, .. } if !self.pending_decisions.is_empty() => {
                return self.pending_decisions.first().is_some_and(|pending| pending.observation.player == player)
                    && self.pending_decisions.iter().filter_map(|pending| pending.continuation.cast_offer())
                        .any(|offer| offer.player == player && offer.card == *card && self.offered_land_actions(player, *card).contains(action));
            }
            Action::ActivateManaAbility { .. } => {
                return (self.has_action_priority(player)
                    || (self.pending_decisions.first().is_some_and(|pending| pending.observation.player == player) && self.payment_mana_window()))
                    && self.mana_activation_for_action(player, action).is_some_and(|activation| self.has_action_priority(player) || !activation.only_as_instant);
            }
            _ => {}
        }
        if self.has_action_priority(player) {
            let mut actions = Vec::new();
            match action {
                Action::PlayLand { .. } => self.add_land_actions(player, &mut actions),
                Action::ActivateAbility { .. } | Action::ActivateAbilityWithAlternativeCost { .. } => self.add_ability_actions(player, &mut actions),
                Action::TurnFaceUp { .. } => self.add_face_up_actions(player, &mut actions),
                Action::Foretell { .. } => self.add_foretell_actions(player, &mut actions),
                Action::TakeCompanion { .. } => self.add_companion_actions(player, &mut actions),
                Action::Plot { .. } => self.add_plot_actions(player, &mut actions),
                Action::Suspend { .. } => self.add_suspend_actions(player, &mut actions),
                Action::UnlockDoor { .. } => self.add_unlock_door_actions(player, &mut actions),
                Action::BeginPayment => return self.legal_actions(player).contains(action),
                _ => return false,
            }
            return actions.contains(action);
        }
        if let Action::ChooseDecision { decision, options } = action {
            let Some(pending) = self.pending_decisions.first() else {
                return false;
            };
            let observation = &pending.observation;
            if observation.player != player || observation.id != *decision {
                return false;
            }
            if pending.continuation.cast_offer_is_mandatory()
                && pending.continuation.cast_offer().is_some_and(|offer| {
                    let mut castable = Vec::new();
                    self.add_offered_cast_actions(offer, &mut castable);
                    !castable.is_empty()
                })
            {
                return false;
            }
            if let super::DecisionContinuation::Payment(
                super::payment::state::PaymentDecision::Mana {
                    target,
                    obligation,
                    selected,
                },
            ) = &pending.continuation
            {
                if options.len() != 1 {
                    return false;
                }
                let Some(view) = self.exact_payment_view(target) else {
                    return false;
                };
                let legal = match options[0] {
                    0 => view.validate_mana_payment(
                        obligation,
                        &super::payment::BoundManaPayment {
                            units: selected.clone(),
                        },
                    ),
                    u32::MAX => !selected.is_empty(),
                    option => {
                        let mut next = selected.clone();
                        next.push(option as usize - 1);
                        view.mana_selection_can_complete(obligation, &next)
                    }
                };
                if !legal {
                    return false;
                }
            }
            let available = observation
                .options
                .iter()
                .map(|option| option.id)
                .collect::<std::collections::HashSet<_>>();
            let unique = options
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>();
            options.len() == unique.len()
                && options.len() >= observation.minimum
                && options.len() <= observation.maximum
                && options.iter().all(|option| available.contains(option))
        } else {
            self.legal_actions(player).contains(action)
        }
    }

}
