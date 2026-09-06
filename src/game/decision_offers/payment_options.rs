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
        if self.append_exact_group_payment_options(&mut options, player, payment) {
            return options;
        }
        match payment {
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
            ResolvedEffectPayment::SacrificePermanentMatching(predicate) => {
                options.extend(self.permanent_payment_options(player, predicate, "Sacrifice"));
            }
            ResolvedEffectPayment::DiscardMatching(predicate) => {
                for (index, card) in self
                    .matching_cards_in_hand(player, predicate)
                    .into_iter()
                    .enumerate()
                {
                    let name = self
                        .catalog
                        .get(card.definition)
                        .map_or_else(|| "a card".to_string(), |card| card.name.clone());
                    options.push(DecisionOption {
                        id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                        label: format!("Discard {name}"),
                        card: Some((
                            card.id,
                            ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                        )),
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::Hand,
                    });
                }
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

    fn append_exact_group_payment_options(
        &self,
        options: &mut Vec<DecisionOption>,
        player: PlayerId,
        payment: ResolvedEffectPayment,
    ) -> bool {
        let (candidates, count, verb, zone) = match payment {
            ResolvedEffectPayment::DiscardCards(amount) => (
                self.players[player.index()]
                    .hand
                    .iter()
                    .map(|card| {
                        (
                            card.id,
                            ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                        )
                    })
                    .collect::<Vec<_>>(),
                amount,
                "Discard",
                DecisionZone::Hand,
            ),
            ResolvedEffectPayment::SacrificePermanents {
                object: predicate,
                amount,
            } => (
                self.group_payment_permanents(
                    self.matching_permanents_controlled(player, predicate),
                ),
                amount,
                "Sacrifice",
                DecisionZone::Battlefield,
            ),
            ResolvedEffectPayment::GainControlPermanents {
                object: predicate,
                amount,
                ..
            } => (
                self.group_payment_permanents(
                    self.matching_permanents_not_controlled(player, predicate),
                ),
                amount,
                "Gain control of",
                DecisionZone::Battlefield,
            ),
            _ => return false,
        };
        self.append_group_payment_options(
            options,
            &candidates,
            usize::from(count),
            verb,
            zone,
        );
        true
    }

    fn group_payment_permanents(
        &self,
        ids: Vec<GameObjectId>,
    ) -> Vec<(GameObjectId, ObjectCharacteristics)> {
        ids.into_iter()
            .filter_map(|id| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .map(|permanent| (id, Self::effective_rules_source(permanent)))
            })
            .collect()
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

    fn append_group_payment_options(
        &self,
        options: &mut Vec<DecisionOption>,
        candidates: &[(GameObjectId, ObjectCharacteristics)],
        count: usize,
        verb: &str,
        zone: DecisionZone,
    ) {
        fn combinations(
            candidates: &[(GameObjectId, ObjectCharacteristics)],
            count: usize,
            start: usize,
            chosen: &mut Vec<(GameObjectId, ObjectCharacteristics)>,
            result: &mut Vec<Vec<(GameObjectId, ObjectCharacteristics)>>,
        ) {
            if chosen.len() == count {
                result.push(chosen.clone());
                return;
            }
            for index in start..candidates.len() {
                chosen.push(candidates[index]);
                combinations(candidates, count, index + 1, chosen, result);
                chosen.pop();
            }
        }

        let mut groups = Vec::new();
        combinations(candidates, count, 0, &mut Vec::new(), &mut groups);
        for members in groups {
            let names = members
                .iter()
                .map(|(_, characteristics)| {
                    self.characteristics_name(*characteristics)
                        .map_or_else(|| "a permanent".to_owned(), Cow::into_owned)
                })
                .collect::<Vec<_>>()
                .join(", ");
            options.push(DecisionOption {
                id: u32::try_from(options.len()).unwrap_or(u32::MAX),
                label: format!("{verb} {names}"),
                card: None,
                members,
                ability_text: None,
                zone,
            });
        }
    }
}
