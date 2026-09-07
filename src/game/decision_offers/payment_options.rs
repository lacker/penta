// The options a payment decision offers, one per way of paying it.
//
// Split out of `decision_offers.rs` only to keep one file readable; these
// are ordinary members of the same `impl Game`, and the paths and imports
// are the parent module's.

impl Game {
    /// The options a payment decision offers: declining, and then one entry
    /// per way of paying. Exact-card and exact-group payments expose each
    /// legal choice; the rest keep their single option.
    pub(super) fn payment_options(
        &self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        can_pay: bool,
        decline: &str,
    ) -> Vec<DecisionOption> {
        let mut options = vec![DecisionOption {
            id: 0,
            label: decline.into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::None,
        }];
        if !can_pay {
            return options;
        }
        match payment {
            ResolvedEffectPayment::ObjectCost { source, cost } => {
                let (_, zone, _) = cost.object_selection().expect("an object cost");
                let verb = if matches!(cost, crate::card::CostDef::Discard { .. }) { "Discard" } else { "Exile" };
                options.extend(self.object_cost_options(&self.object_cost_candidates(player, source, cost), zone).into_iter().map(|mut option| {
                    option.label = format!("{verb} {}", option.label);
                    option
                }));
            }
            // One option per amount the payer can actually afford, with the
            // amount as the option id.
            ResolvedEffectPayment::ChosenGenericMana => {
                for amount in 1..=self.maximum_generic_payment(player) {
                    options.push(DecisionOption {
                        id: u32::from(amount),
                        label: format!("Pay {{{amount}}}"),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    });
                }
            }
            ResolvedEffectPayment::ChosenEnergy => {
                for amount in 1..=self.players[player.index()]
                    .counters
                    .count(CounterKind::named("energy"))
                {
                    options.push(DecisionOption {
                        id: u32::from(amount),
                        label: format!("Pay {amount} energy"),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    });
                }
            }
            ResolvedEffectPayment::RemoveAnyNumberOfCounters { object, kind } => {
                options.extend(self.counter_removal_payment_options(object, kind));
            }
            ResolvedEffectPayment::MovePermanentMatching {
                object: predicate,
                zone,
            } => {
                let verb = if zone == ZoneKind::Hand {
                    "Return"
                } else {
                    "Move"
                };
                options.extend(self.permanent_payment_options(player, predicate, verb));
            }
            payment => options.push(DecisionOption {
                id: 1,
                label: Self::effect_payment_label(payment),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            }),
        }
        options
    }

    fn counter_removal_payment_options(
        &self,
        object: GameObjectId,
        kind: CounterKind,
    ) -> Vec<DecisionOption> {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        else {
            return Vec::new();
        };
        let presentation = Self::effective_rules_source(permanent);
        (1..=permanent.counters(kind))
            .map(|amount| DecisionOption {
                id: u32::from(amount),
                label: format!("Remove {amount} counter(s)"),
                card: Some((object, presentation)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
            .collect()
    }

    fn settle_counter_removal_payment(
        &mut self,
        object: GameObjectId,
        kind: CounterKind,
        chosen: u32,
    ) -> Option<u16> {
        let amount = u16::try_from(chosen).unwrap_or(u16::MAX);
        let permanent = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == object)?;
        if amount == 0 || permanent.counters(kind) < amount {
            return None;
        }
        permanent.remove_counters(kind, amount);
        Some(amount)
    }

}
