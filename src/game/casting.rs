use super::{
    AbilityCostDef, AbilityOrigin, AbilitySourceRef, AbilityTargetDef, AlternativeCastKindDef,
    BTreeMap, BattlefieldExitCompletion, CREATURE_TYPES, CardBehavior, CardEffectStatus, CardType,
    CardTypeSet, CastChoices, CastSignature, CastSourceZone, CommittedTriggerEvent, ControlFlow,
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    DeclarativeAbilityDef, EntryCompletion, Game, GameEvent, GameObjectId, Mana, ManaColor,
    ManaCost, ManaPaymentPurpose, PendingBattlefieldEntry, Permanent, PlayActionKind,
    PlayOptionDef, PlayOptionId, PlayRestriction, PlayerId, StackObject, StackObjectKind, Target,
    TargetPredicate, TargetSlotDef, TargetSlotId, TriggerCapture, TriggerContext, ZoneKind,
    add_generic, extra_target_cost, reduce_generic, remove_card,
};

impl Game {
    pub(super) fn play_land(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        option_id: PlayOptionId,
    ) {
        let definition_id = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == card_id)
            .map(|card| card.definition)
            .expect("legal land action references a card in hand");
        let definition = self
            .catalog
            .get(definition_id)
            .expect("legal land action references a cataloged card");
        let option = definition
            .play_option(option_id)
            .filter(|option| option.action == PlayActionKind::PlayLand)
            .expect("legal land action references a land play option");
        let presented = match &option.form {
            crate::card::SpellForm::Part(part) => *part,
            crate::card::SpellForm::Combined(_) => {
                unreachable!("a land play option presents exactly one card part")
            }
        };
        definition
            .part(presented)
            .filter(|part| part.rules.has_type(CardType::Land))
            .expect("land play option references a land part");
        let card = remove_card(&mut self.players[player.index()].hand, card_id)
            .expect("legal land action references a card in hand");
        self.players[player.index()].land_played_this_turn = true;
        self.consecutive_passes = 0;
        let permanent =
            Permanent::entering(card, presented, player, self.turns_started[player.index()]);
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Hand,
            completion: EntryCompletion::LandPlayed { player },
        });
    }

    pub(super) fn creature_type_choices(&self, player: PlayerId) -> Vec<String> {
        let mut counts = CREATURE_TYPES
            .iter()
            .map(|creature_type| ((*creature_type).into(), 0))
            .collect::<BTreeMap<String, usize>>();
        for card in &self.players[player.index()].hand {
            let Some(definition) = self.catalog.get(card.definition) else {
                continue;
            };
            for part in &definition.parts {
                if part.rules.has_type(CardType::Creature) {
                    for subtype in part.rules.subtypes() {
                        if let Some(count) = counts.get_mut(*subtype) {
                            *count += 1;
                        }
                    }
                }
            }
        }
        let mut choices = counts.into_iter().collect::<Vec<_>>();
        choices.sort_by(|(left_name, left_count), (right_name, right_count)| {
            right_count
                .cmp(left_count)
                .then_with(|| left_name.cmp(right_name))
        });
        choices.into_iter().map(|(name, _)| name).collect()
    }

    /// Offers the card names worth naming. Naming a card with no activated
    /// ability does nothing at all, so leaving those out of the list changes
    /// no outcome and keeps the choice readable.
    /// "You may have this enter as a copy of ...": the copy is picked as the
    /// permanent enters, and entering as itself is always an option.
    pub(super) fn queue_entry_copy_choice(
        &mut self,
        player: PlayerId,
        choices: Vec<GameObjectId>,
        added_types: CardTypeSet,
    ) {
        let mut options = vec![DecisionOption {
            id: 0,
            label: "Enter as itself".into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::None,
        }];
        options.extend(choices.iter().enumerate().filter_map(|(index, id)| {
            let permanent = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *id)?;
            let definition = permanent.card.definition;
            Some(DecisionOption {
                id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                label: self.catalog.get(definition).map_or_else(
                    || "Copy an unknown permanent".into(),
                    |card| format!("Enter as a copy of {}", card.name),
                ),
                card: Some((*id, definition)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
        }));
        self.queue_decision(
            player,
            "Choose what this permanent enters as",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryCopy {
                choices,
                added_types,
            },
        );
    }

    pub(super) fn queue_card_name_choice(&mut self, player: PlayerId) {
        let mut names = self
            .catalog
            .definitions()
            .into_iter()
            .filter(|definition| {
                definition.parts.iter().any(|part| {
                    part.rules.ability_clauses().iter().any(|ability| {
                        matches!(
                            ability.definition,
                            DeclarativeAbilityDef::Activated(_)
                                | DeclarativeAbilityDef::ActivatedMana(_)
                        )
                    })
                })
            })
            .map(|definition| definition.name.clone())
            .collect::<Vec<_>>();
        names.sort();
        names.dedup();
        // A catalog with nothing to name would strand the entry procedure.
        if names.is_empty() {
            names.push("Black Lotus".into());
        }
        let options = names
            .iter()
            .enumerate()
            .map(|(index, name)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: name.clone(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            "Choose a card name",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryCardName { choices: names },
        );
    }

    pub(super) fn queue_creature_type_choice(&mut self, player: PlayerId) {
        let mut choices = self.creature_type_choices(player);
        // The complete game vocabulary always contains creature types. Keep a
        // legal fallback so a deliberately tiny test catalog cannot strand
        // the land in an unfinished entry procedure.
        if choices.is_empty() {
            choices.push("Human".into());
        }
        let options = choices
            .iter()
            .enumerate()
            .map(|(index, creature_type)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: creature_type.clone(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            "Choose a creature type",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryCreatureType { choices },
        );
    }

    pub(super) fn activate_mana_source(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        color: ManaColor,
    ) {
        let activation = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| self.mana_ability_activation(permanent, ability, color))
            .expect("legal mana action references a mana source");
        let produced_mana = Self::mana_for_activation(activation);
        for cost in activation.costs.as_slice() {
            match cost {
                AbilityCostDef::TapSource => {
                    // Captured before the tap so the land's own characteristics
                    // are the ones a watcher sees, and only here: a mana
                    // ability with no tap cost never taps anything for mana.
                    let tapped_for_mana = self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == source)
                        .map(|permanent| CommittedTriggerEvent::TappedForMana {
                            object: self.trigger_event_object(permanent),
                        });
                    let _ = self.tap_permanent(source);
                    if let Some(event) = tapped_for_mana {
                        self.capture_battlefield_triggers(&event);
                    }
                }
                AbilityCostDef::SacrificeSource | AbilityCostDef::ExileSource => {}
                AbilityCostDef::RemoveCountersFromSource { kind, amount } => {
                    self.battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                        .expect("a legal mana activation has its source")
                        .remove_counters(*kind, *amount);
                }
                AbilityCostDef::PayLife(amount) => {
                    self.lose_life(player, *amount);
                }
                AbilityCostDef::Mana(_)
                | AbilityCostDef::DiscardSource
                | AbilityCostDef::UntapSource
                | AbilityCostDef::Loyalty(_)
                | AbilityCostDef::ExileCardFromGraveyard(_)
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::Special(_) => {
                    unreachable!("unsupported mana-ability costs are not enumerated")
                }
            }
        }
        if activation.costs.contains(&AbilityCostDef::ExileSource) {
            self.exile_permanent(source);
        } else if activation.costs.contains(&AbilityCostDef::SacrificeSource) {
            self.move_permanents_to_graveyard_then(
                &[source],
                Some(BattlefieldExitCompletion::CompleteManaAbility {
                    player,
                    activation,
                    produced_mana,
                }),
            );
            return;
        }
        self.complete_mana_ability(player, activation, produced_mana);
    }

    pub(super) fn complete_mana_ability(
        &mut self,
        player: PlayerId,
        activation: super::ManaAbilityActivation,
        produced_mana: Vec<Mana>,
    ) {
        self.add_mana(player, produced_mana);
        if activation.effect.damage_to_controller > 0 {
            self.damage_target_from(
                Some(activation.source),
                Some(Target::Player(player)),
                activation.effect.damage_to_controller,
            );
        }
        self.consecutive_passes = 0;
        self.events.push(GameEvent::ManaAdded {
            player,
            source: activation.source,
        });
    }

    /// Whether the chosen modes suit the play option: the right number, in
    /// ascending order, without repeats unless the card allows them, and all
    /// of them actually executable.
    pub(super) fn mode_selection_is_valid(option: &PlayOptionDef, choices: &CastChoices) -> bool {
        match &option.modes {
            None => choices.modes().is_empty(),
            Some(mode_set) => {
                let count = choices.modes().len();
                if count < usize::from(mode_set.minimum) || count > usize::from(mode_set.maximum) {
                    return false;
                }
                if !mode_set.may_repeat {
                    let unique = choices
                        .modes()
                        .iter()
                        .copied()
                        .collect::<std::collections::HashSet<_>>();
                    if unique.len() != count {
                        return false;
                    }
                }
                if choices.modes().windows(2).any(|pair| pair[0] > pair[1]) {
                    return false;
                }
                choices.modes().iter().all(|selected| {
                    mode_set.modes.iter().any(|mode| {
                        mode.id == *selected && mode.effect_status == CardEffectStatus::Implemented
                    })
                })
            }
        }
    }

    /// Whether the chosen targets fill a declarative spell clause's slots and
    /// every one of them is legal right now.
    pub(super) fn spell_target_selection_is_valid(
        &self,
        target_defs: &[AbilityTargetDef],
        choices: &CastChoices,
        player: PlayerId,
        card_id: GameObjectId,
    ) -> bool {
        target_defs.len() == choices.targets().len()
            && target_defs.iter().enumerate().zip(choices.targets()).all(
                |((index, slot), selection)| {
                    let count = selection.targets().len();
                    let legal = self.ability_targets_matching(
                        slot.predicate,
                        player,
                        card_id,
                        TriggerContext::empty(),
                    );
                    TargetSlotId::from_index(index) == Some(selection.slot())
                        && count >= usize::from(slot.minimum)
                        && count <= usize::from(slot.maximum)
                        && selection
                            .targets()
                            .iter()
                            .all(|target| legal.contains(target))
                },
            )
    }

    /// Whether the chosen targets fill the play option's own declared slots,
    /// used by cards whose targeting comes from the option rather than from a
    /// declarative spell clause.
    pub(super) fn declared_slot_selection_is_valid(
        &self,
        declared_slots: &[TargetSlotDef],
        choices: &CastChoices,
    ) -> bool {
        if declared_slots.len() != choices.targets().len() {
            return false;
        }
        declared_slots
            .iter()
            .zip(choices.targets())
            .all(|(slot, selection)| {
                let count = selection.targets().len();
                slot.id == selection.slot()
                    && count >= usize::from(slot.minimum)
                    && count <= usize::from(slot.maximum)
                    && selection
                        .targets()
                        .iter()
                        .all(|target| self.target_matches(slot.predicate, *target))
            })
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn validated_cast_signature(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
    ) -> Option<(CastSignature, ManaCost, CardBehavior, CastSourceZone)> {
        let state = &self.players[player.index()];
        let (card, source_zone) = state
            .hand
            .iter()
            .find(|card| card.id == card_id)
            .map(|card| (card, CastSourceZone::Hand))
            .or_else(|| {
                state
                    .graveyard
                    .iter()
                    .find(|card| card.id == card_id)
                    .map(|card| (card, CastSourceZone::Graveyard))
            })?;
        let definition = self.catalog.get(card.definition)?;
        if self.play_is_prohibited(card, player) {
            return None;
        }
        let option = definition
            .play_option(choices.play_option())
            .filter(|option| option.action == PlayActionKind::CastSpell)?;
        if source_zone == CastSourceZone::Graveyard
            && option.restriction == PlayRestriction::FromHandOnly
        {
            return None;
        }
        if !self.play_timing_allows(option.restriction) {
            return None;
        }
        let behavior =
            Self::play_option_behavior(definition, option).unwrap_or(CardBehavior::Unsupported);
        let types = Self::play_option_types(definition, option)?;
        if option.effect_status == CardEffectStatus::MetadataOnly && !types.is_creature() {
            return None;
        }

        if !Self::mode_selection_is_valid(option, choices) {
            return None;
        }

        if !self
            .visit_cost_configurations(definition, card_id, option, source_zone, |costs| {
                if &costs == choices.costs() {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            })
            .is_break()
        {
            return None;
        }
        let alternative_kind =
            self.selected_alternative_kind(definition, option, card_id, choices.costs());
        if alternative_kind == Some(AlternativeCastKindDef::Overload) && !choices.modes().is_empty()
        {
            return None;
        }
        let mut cost = self.configured_cast_mana_cost(card_id, option, choices.costs())?;
        if !cost.variable_x && choices.x() != 0 {
            return None;
        }

        let declared_slots = Self::target_slots_for(option, choices.modes());
        if alternative_kind == Some(AlternativeCastKindDef::Overload) {
            if !choices.targets().is_empty() {
                return None;
            }
        } else if alternative_kind == Some(AlternativeCastKindDef::Bestow) {
            let selected = choices.costs().alternative()?;
            let (_, ability, _) = Self::alternative_cast_ability(definition, option, selected)?;
            let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition else {
                return None;
            };
            if alternative.targets.len() != choices.targets().len()
                || !self.spell_target_selection_is_valid(
                    alternative.targets,
                    choices,
                    player,
                    card_id,
                )
            {
                return None;
            }
        } else if let Some((_, ability)) = Self::spell_ability(definition, option) {
            let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                unreachable!("spell_ability returns a spell clause")
            };
            let plan = Self::selected_spell_plan(spell, choices.modes())?;
            if plan.target_defs.len() != choices.targets().len() {
                return None;
            }
            if !self.spell_target_selection_is_valid(&plan.target_defs, choices, player, card_id) {
                return None;
            }
        } else if Self::uses_legacy_behavior_targets(definition, option) {
            let flat_targets = choices.iter_targets().copied().collect::<Vec<_>>();
            let has_legacy_shape = if flat_targets.is_empty() {
                choices.targets().is_empty()
            } else {
                matches!(choices.targets(), [selection]
                    if selection.slot() == TargetSlotId(0)
                        && selection.targets() == flat_targets)
            };
            if !has_legacy_shape
                || !self
                    .legal_target_lists(behavior, player, None)
                    .contains(&flat_targets)
            {
                return None;
            }
            cost = add_generic(cost, extra_target_cost(definition, flat_targets.len()));
        } else if !self.declared_slot_selection_is_valid(&declared_slots, choices) {
            return None;
        }
        cost = reduce_generic(cost, self.spell_cost_reduction(definition.id, player));
        let payment_purpose = ManaPaymentPurpose::Spell {
            object: card_id,
            definition: definition.id,
            controller: player,
            form: option.form.clone(),
        };
        if cost.variable_x && choices.x() > self.maximum_x_for(player, cost, &payment_purpose) {
            return None;
        }
        if !self.can_pay_cost_for(player, cost, choices.x(), &payment_purpose) {
            return None;
        }

        Some((
            CastSignature::from_validated_choices(option.form.clone(), choices.clone()),
            cost,
            behavior,
            source_zone,
        ))
    }

    pub(super) fn target_matches(&self, predicate: TargetPredicate, target: Target) -> bool {
        self.targets_matching(predicate).contains(&target)
    }

    fn schedule_cleanup_trigger_for_cast(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
    ) -> Option<u32> {
        let cleanup_permission = self.players[player.index()]
            .hand
            .iter()
            .chain(&self.players[player.index()].graveyard)
            .find(|card| card.id == card_id)
            .and_then(|card| {
                let definition = self.catalog.get(card.definition)?;
                let option = definition.play_option(signature.play_option())?;
                let off_timing =
                    player != self.active_player || !self.step.is_main() || !self.stack.is_empty();
                if !off_timing || Self::play_option_has_keyword_flash(definition, option) {
                    return None;
                }
                let (origin, trigger) =
                    Self::play_option_cleanup_flash_trigger(definition, option)?;
                Some((card.owner, definition.id, origin, trigger))
            });

        cleanup_permission.and_then(|(owner, presentation_definition, origin, trigger)| {
            let DeclarativeAbilityDef::Triggered(definition) = trigger.definition else {
                return None;
            };
            let capture = TriggerCapture {
                source: AbilitySourceRef {
                    object: card_id,
                    ability: origin,
                },
                definition: presentation_definition,
                owner,
                controller: player,
                text: trigger.text,
                target_defs: definition.targets,
                effect: trigger.effect.definition,
                resolver: Self::ability_resolver(origin, &trigger),
                context: TriggerContext::empty(),
                condition: definition.condition,
            };
            Some(self.schedule_one_shot_event_trigger(definition.event, &capture))
        })
    }

    pub(super) fn cast_spell(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
        sacrifices: &[GameObjectId],
    ) {
        let (signature, cost, _behavior, source_zone) = self
            .validated_cast_signature(player, card_id, choices)
            .expect("validated casting choices remain valid while paying costs");
        let targets = signature.iter_targets().copied().collect::<Vec<_>>();
        let x = signature.x();
        let cast_via_flashback = self.players[player.index()]
            .hand
            .iter()
            .chain(&self.players[player.index()].graveyard)
            .find(|card| card.id == card_id)
            .and_then(|card| self.catalog.get(card.definition))
            .and_then(|definition| {
                definition
                    .play_option(signature.play_option())
                    .map(|option| (definition, option))
            })
            .and_then(|(definition, option)| {
                self.selected_alternative_kind(definition, option, card_id, signature.costs())
            })
            == Some(AlternativeCastKindDef::Flashback);
        // The delayed ability is created by casting with the permission, not
        // by the spell resolving. A countered spell therefore leaves an
        // unbound listener that will trigger and resolve harmlessly. Entry
        // completion fills in the exact resulting permanent only on success.
        let schedule_on_entry = self.schedule_cleanup_trigger_for_cast(player, card_id, &signature);
        let card = match source_zone {
            CastSourceZone::Hand => remove_card(&mut self.players[player.index()].hand, card_id),
            CastSourceZone::Graveyard => {
                remove_card(&mut self.players[player.index()].graveyard, card_id)
            }
        }
        .expect("legal cast action references a card in its validated source zone");
        // Every outstanding grant applies to the same next sorcery, whatever
        // its timing, so consume them together based on the form actually cast.
        let cast_is_sorcery = self
            .catalog
            .get(card.definition)
            .and_then(|definition| {
                let option = definition.play_option(signature.play_option())?;
                Self::play_option_types(definition, option)
            })
            .is_some_and(|types| types.contains(CardType::Sorcery));
        if cast_is_sorcery {
            self.sorcery_flash_grants[player.index()] = 0;
        }
        // A spell is first proposed on the stack, then mana abilities may be
        // activated and costs are paid. The operation cannot fail after the
        // validated signature above, so keeping the provisional object local
        // gives mana spend riders a concrete destination without exposing a
        // half-paid spell to priority or trigger placement.
        let (card, _zone_change) = self.zone_change_card(card);
        let stack_id = card.id;
        let definition = card.definition;
        let frozen_spell_ability = self.frozen_spell_payload(definition, &signature);
        let mut stack_object = StackObject {
            id: stack_id,
            kind: StackObjectKind::Spell,
            card,
            source: None,
            ability: frozen_spell_ability,
            controller: player,
            signature: Some(signature),
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback,
            schedule_on_entry,
            is_copy: false,
        };
        let payment_purpose = ManaPaymentPurpose::Spell {
            object: stack_id,
            definition,
            controller: player,
            form: stack_object
                .signature
                .as_ref()
                .expect("a spell has a cast signature")
                .form()
                .clone(),
        };
        self.activate_mana_for_cost_avoiding_for(player, cost, x, None, &payment_purpose);
        let spent_mana = self.pay_player_cost_for(player, cost, x, &payment_purpose);
        Self::apply_spent_mana_to_spell(&mut stack_object, &spent_mana);
        self.continue_spell_cast(stack_object, targets, sacrifices.to_vec());
    }

    pub(super) fn continue_spell_cast(
        &mut self,
        stack_object: StackObject,
        targets: Vec<Target>,
        mut remaining_sacrifices: Vec<GameObjectId>,
    ) {
        if !remaining_sacrifices.is_empty() {
            let sacrificed = remaining_sacrifices.remove(0);
            self.move_permanents_to_graveyard_then(
                &[sacrificed],
                Some(BattlefieldExitCompletion::CompleteSpellCast {
                    object: Box::new(stack_object),
                    targets,
                    remaining_sacrifices,
                }),
            );
            return;
        }

        let player = stack_object.controller;
        let stack_id = stack_object.id;
        let definition = stack_object.card.definition;
        let cast_event = self
            .stack_trigger_event_object(&stack_object)
            .expect("a cast spell has locked characteristics");
        self.stack.push(stack_object);
        self.consecutive_passes = 0;
        self.spells_cast_this_turn[player.index()] =
            self.spells_cast_this_turn[player.index()].saturating_add(1);
        self.events.push(GameEvent::SpellCast {
            player,
            card: stack_id,
            definition,
            targets,
        });
        self.capture_battlefield_triggers(&CommittedTriggerEvent::SpellCast { object: cast_event });
    }
}
