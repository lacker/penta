impl Game {
    fn triggered_mana_choice_options(
        &self,
        effect: EffectDef,
        source: GameObjectId,
        controller: PlayerId,
        produced: &[ManaColor],
        choices: &mut Vec<Vec<ManaSplit>>,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.triggered_mana_choice_options(
                        *effect,
                        source,
                        controller,
                        produced,
                        choices,
                    );
                }
            }
            EffectDef::AddMana(effect) => {
                let types = match effect.mana {
                    ManaSelectionDef::Choice(types) | ManaSelectionDef::Combination(types) => {
                        types
                    }
                    ManaSelectionDef::One(_)
                    | ManaSelectionDef::ColorsOfLinkedExiles
                    | ManaSelectionDef::ChoiceOfBundles(_) => return,
                };
                let domain = match types.source {
                    ManaTypeSourceDef::Fixed(colors) => colors,
                    ManaTypeSourceDef::ProducedBy(ObjectRefDef::TriggeringObject) => produced,
                    ManaTypeSourceDef::ProducedBy(_) | ManaTypeSourceDef::CouldBeProducedBy(_) => {
                        return;
                    }
                };
                let domain = domain
                    .iter()
                    .copied()
                    .filter(|color| {
                        types.filter == ManaTypeFilterDef::AnyType
                            || *color != ManaColor::Colorless
                    })
                    .collect::<Vec<_>>();
                let amount = self.mana_amount_for(effect, controller, source);
                let options = match effect.mana {
                    ManaSelectionDef::Choice(_) => domain
                        .iter()
                        .copied()
                        .map(|color| {
                            let mut split = ManaSplit::empty();
                            split.add(color, amount);
                            split
                        })
                        .collect(),
                    ManaSelectionDef::Combination(_) => {
                        Self::mana_combinations(&domain, amount)
                    }
                    _ => unreachable!("only choice-bearing mana effects reach this branch"),
                };
                choices.push(options);
            }
            _ => {}
        }
    }

    /// Enumerate the choices made by immediate mana triggers caused by one
    /// activation. A trigger has no later decision window, so each independent
    /// selected output belongs to the same action as the source ability's own
    /// counter, sacrifice, and combination choices.
    fn with_triggered_mana_choices(
        &self,
        permanent: &Permanent,
        activations: Vec<ManaAbilityActivation>,
    ) -> Vec<ManaAbilityActivation> {
        if !activations
            .iter()
            .any(|activation| activation.costs.contains(&CostDef::TapSource))
        {
            return activations;
        }

        let mut event_object = self.trigger_event_object(permanent);
        event_object.tapped = true;
        let event = CommittedTriggerEvent::Tapped {
            object: event_object,
            for_mana: true,
        };
        let captures = self
            .battlefield_trigger_listeners()
            .into_iter()
            .filter_map(|listener| {
                if listener.uses_stack
                    || !self.trigger_event_matches_for_controller(
                        listener.event,
                        &event,
                        listener.capture.source.object,
                        Some(listener.capture.controller),
                    )
                    || listener.trigger_limit.is_some_and(|limit| {
                        self.triggers_this_turn(listener.capture.source) >= limit
                    })
                {
                    return None;
                }
                let mut capture = listener.capture;
                capture.context.trigger = event.context();
                self.trigger_capture_condition_holds(&capture)
                    .then_some(capture)
            })
            .collect::<Vec<_>>();

        activations
            .into_iter()
            .flat_map(|activation| {
                if !activation.costs.contains(&CostDef::TapSource) {
                    return vec![activation];
                }
                let mut produced = Self::mana_for_activation(&activation)
                    .into_iter()
                    .map(|mana| mana.color)
                    .collect::<Vec<_>>();
                produced.sort_unstable();
                produced.dedup();
                let mut slots = Vec::new();
                for capture in &captures {
                    self.triggered_mana_choice_options(
                        capture.effect,
                        capture.source.object,
                        capture.controller,
                        &produced,
                        &mut slots,
                    );
                }
                if slots.is_empty() {
                    return vec![activation];
                }
                let mut selections = vec![Vec::new()];
                for options in slots {
                    selections = selections
                        .into_iter()
                        .flat_map(|selected| {
                            options.iter().copied().map(move |option| {
                                let mut selected = selected.clone();
                                selected.push(option);
                                selected
                            })
                        })
                        .collect();
                }
                selections
                    .into_iter()
                    .map(|triggered_mana| ManaAbilityActivation {
                        triggered_mana: Some(triggered_mana),
                        ..activation.clone()
                    })
                    .collect()
            })
            .collect()
    }
}
