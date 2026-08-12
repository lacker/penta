use super::{
    AbilityCostDef, AbilityOrigin, AbilityProcedureDef, BattlefieldExitCompletion, CardBehavior,
    CardInstance, CharacteristicContext, CounterKind, DeclarativeAbilityDef,
    FrozenActivatedAbility, Game, GameEvent, GameObjectId, ManaCost, ManaPaymentPurpose, PlayerId,
    Target, TargetSelection, ZoneKind, remove_card,
};

impl Game {
    #[allow(clippy::too_many_lines)]
    pub(super) fn activate_ability(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        targets: Vec<TargetSelection>,
        cost_object: Option<GameObjectId>,
        x: u16,
    ) {
        if let Some(source_card) = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == source)
            .cloned()
        {
            let Some(effective) = self.find_printed_card_ability(
                &source_card,
                &CharacteristicContext::Hand,
                |effective| effective.origin == ability,
            ) else {
                return;
            };
            let DeclarativeAbilityDef::Activated(definition) = effective.ability.definition else {
                return;
            };
            if !effective.ability.is_executable()
                || definition.procedure != AbilityProcedureDef::Shared
                || !definition.source_zones.contains(&ZoneKind::Hand)
            {
                return;
            }
            let frozen = FrozenActivatedAbility {
                origin: effective.origin,
                definition: Some(Box::new(effective.ability)),
                presentation_definition: Self::ability_presentation_definition(
                    effective.origin,
                    source_card.definition,
                ),
                text: Some(effective.ability.text),
                target_defs: definition.targets,
                resolver: Self::ability_resolver(effective.origin, &effective.ability),
                x,
            };
            let payment_purpose = ManaPaymentPurpose::Ability {
                source,
                taps_source: false,
                leaves_source: false,
            };
            for cost in definition.costs.as_slice() {
                match cost {
                    AbilityCostDef::Mana(cost) => {
                        self.activate_mana_for_cost_avoiding_for(
                            player,
                            *cost,
                            x,
                            None,
                            &payment_purpose,
                        );
                        let _ = self.pay_player_cost_for(player, *cost, x, &payment_purpose);
                    }
                    AbilityCostDef::DiscardSource => {
                        let discarded = remove_card(&mut self.players[player.index()].hand, source)
                            .expect("a legal hand activation still has its source");
                        let definition = discarded.definition;
                        let (discarded, _zone_change) = self.zone_change_card(discarded);
                        let discarded_id = discarded.id;
                        self.put_card_into_graveyard(player, discarded);
                        self.events.push(GameEvent::CardsDiscarded {
                            player,
                            cards: vec![(discarded_id, definition)],
                        });
                    }
                    AbilityCostDef::TapSource
                    | AbilityCostDef::UntapSource
                    | AbilityCostDef::SacrificeSource
                    | AbilityCostDef::RemoveCountersFromSource { .. }
                    | AbilityCostDef::PayLife(_)
                    | AbilityCostDef::DiscardCards(_)
                    | AbilityCostDef::SacrificePermanent { .. }
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::Loyalty(_)
                    | AbilityCostDef::ExileCardFromGraveyard(_)
                    | AbilityCostDef::Special(_) => {
                        unreachable!("unsupported hand-zone costs are not offered")
                    }
                }
            }
            let chosen_permanents = targets
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Permanent(permanent) => Some(*permanent),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
                .collect();
            self.push_activated_ability(
                source,
                &source_card,
                player,
                frozen,
                targets,
                chosen_permanents,
            );
            self.consecutive_passes = 0;
            self.check_state_based_actions();
            return;
        }
        let Some(source_permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        let source_card = source_permanent.card.clone();
        let mut frozen_ability = self.freeze_activated_ability(source_permanent, ability);
        frozen_ability.x = x;
        // `apply` validated these exact ordered slot selections against a
        // generated legal action. Freeze both their slot identity and values
        // before any activation cost can move or change the source.
        let frozen_targets = targets;
        let selected_ability = self
            .find_effective_ability(source_permanent, |effective| effective.origin == ability)
            .map(|effective| effective.ability);
        let declarative = selected_ability.filter(|ability| {
            ability.is_executable()
                && matches!(
                    ability.definition,
                    DeclarativeAbilityDef::Activated(definition)
                        if definition.procedure == AbilityProcedureDef::Shared
                )
        });
        let behavior = selected_ability.and_then(|ability| match ability.definition {
            DeclarativeAbilityDef::Activated(definition)
                if definition.procedure == AbilityProcedureDef::Legacy =>
            {
                ability.custom_behavior()
            }
            DeclarativeAbilityDef::Legacy => ability.custom_behavior(),
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::ActivatedMana(_)
            | DeclarativeAbilityDef::TriggeredMana(_)
            | DeclarativeAbilityDef::Activated(_)
            | DeclarativeAbilityDef::Triggered(_)
            | DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::Replacement(_)
            | DeclarativeAbilityDef::AlternativeCast(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Keyword(_) => None,
        });
        if let Some(ability_def) = declarative {
            let DeclarativeAbilityDef::Activated(definition) = ability_def.definition else {
                unreachable!("the declarative activation filter checked its category")
            };
            let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
            let leaves_source = definition.costs.iter().any(|cost| {
                matches!(
                    cost,
                    AbilityCostDef::SacrificeSource | AbilityCostDef::ExileSource
                )
            });
            let animates_source = Self::effect_animates_source(ability_def.declarative_effect());
            let has_generic_sacrifice = definition
                .costs
                .iter()
                .any(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }));
            let sacrifice_choice_is_source = has_generic_sacrifice && cost_object == Some(source);
            for cost in definition.costs.as_slice() {
                match cost {
                    AbilityCostDef::Mana(cost) => {
                        self.activate_mana_for_cost_avoiding_for(
                            player,
                            *cost,
                            x,
                            // Tapping the source to pay would hand back a
                            // tapped creature, so auto-payment leaves it
                            // alone even though the tap itself is legal.
                            (taps_source || animates_source).then_some(source),
                            &ManaPaymentPurpose::Ability {
                                source,
                                taps_source,
                                leaves_source,
                            },
                        );
                        let _ = self.pay_player_cost(player, *cost, x);
                    }
                    AbilityCostDef::TapSource => {
                        let _ = self.tap_permanent(source);
                    }
                    AbilityCostDef::SacrificeSource
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::SacrificePermanent { .. } => {
                        // Deferred until mana and source-dependent costs have
                        // been paid. A chosen permanent may itself produce
                        // mana first, and the source may still owe a tap or
                        // counter-removal cost before it leaves.
                    }
                    AbilityCostDef::RemoveCountersFromSource { kind, amount } => {
                        self.battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == source)
                            .expect("a legal activation has its source")
                            .remove_counters(*kind, *amount);
                    }
                    AbilityCostDef::DiscardSource => {
                        unreachable!("a battlefield source cannot discard itself")
                    }
                    AbilityCostDef::PayLife(amount) => {
                        self.lose_life(player, *amount);
                    }
                    AbilityCostDef::ExileCardFromGraveyard(_) => {
                        let chosen = cost_object.expect("a legal activation chose the exiled card");
                        if let Some(card) =
                            remove_card(&mut self.players[player.index()].graveyard, chosen)
                        {
                            let (card, _zone_change) = self.zone_change_card(card);
                            self.players[player.index()].exile.push(card);
                        }
                    }
                    AbilityCostDef::Loyalty(change) => {
                        if let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == source)
                        {
                            if *change >= 0 {
                                permanent.add_counters(
                                    CounterKind::Loyalty,
                                    u16::from(change.unsigned_abs()),
                                );
                            } else {
                                permanent.remove_counters(
                                    CounterKind::Loyalty,
                                    u16::from(change.unsigned_abs()),
                                );
                            }
                            permanent.activated_loyalty_this_turn = true;
                        }
                    }
                    AbilityCostDef::UntapSource
                    | AbilityCostDef::DiscardCards(_)
                    | AbilityCostDef::Special(_) => {
                        unreachable!("unsupported costs are not offered as legal actions")
                    }
                }
            }
            let mut remaining_sacrifices = Vec::new();
            if has_generic_sacrifice
                && let Some(sacrificed) = cost_object
                && sacrificed != source
            {
                remaining_sacrifices.push(sacrificed);
            }
            if definition.costs.contains(&AbilityCostDef::ExileSource) {
                self.exile_permanent(source);
            } else if definition.costs.contains(&AbilityCostDef::SacrificeSource)
                || sacrifice_choice_is_source
            {
                remaining_sacrifices.push(source);
            }
            let mut chosen_permanents = frozen_targets
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Permanent(permanent) => Some(*permanent),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
                .collect::<Vec<_>>();
            if let Some(sacrificed) = cost_object
                && !chosen_permanents.contains(&sacrificed)
            {
                chosen_permanents.push(sacrificed);
            }
            self.continue_activated_ability_costs(
                source,
                source_card,
                player,
                frozen_ability,
                frozen_targets,
                chosen_permanents,
                remaining_sacrifices,
            );
            return;
        }
        match behavior {
            Some(CardBehavior::SedgeTroll) => {
                let cost = ManaCost::colored(0, 0, 0, 1, 0, 0);
                self.activate_mana_for_cost(player, cost, 0);
                let _ = self.pay_player_cost(player, cost, 0);
                let card = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .map(|permanent| permanent.card.clone())
                    .expect("legal Sedge Troll activation has a source");
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    Vec::new(),
                    Vec::new(),
                );
            }
            Some(CardBehavior::LibraryOfAlexandria) => {
                let card = self
                    .tap_permanent(source)
                    .expect("legal activation has a source");
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    frozen_targets,
                    Vec::new(),
                );
            }
            _ => {}
        }
        self.consecutive_passes = 0;
        self.check_state_based_actions();
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn continue_activated_ability_costs(
        &mut self,
        source: GameObjectId,
        source_card: CardInstance,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
        mut remaining_sacrifices: Vec<GameObjectId>,
    ) {
        if !remaining_sacrifices.is_empty() {
            let sacrificed = remaining_sacrifices.remove(0);
            self.move_permanents_to_graveyard_then(
                &[sacrificed],
                Some(BattlefieldExitCompletion::CompleteActivatedAbility {
                    source,
                    source_card,
                    controller,
                    frozen,
                    targets,
                    chosen_permanents,
                    remaining_sacrifices,
                }),
            );
            return;
        }

        self.push_activated_ability(
            source,
            &source_card,
            controller,
            frozen,
            targets,
            chosen_permanents,
        );
        self.consecutive_passes = 0;
        self.check_state_based_actions();
    }
}
