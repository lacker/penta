use super::{
    AbilityDef, AbilityId, AbilityOrigin, AbilityTargetDef, AbilityTargetPredicate, Action,
    AdditionalCostId, AlternativeCastAbilityDef, AlternativeCastKindDef, AlternativeCostId,
    CardBehavior, CardDefinition, CardDefinitionId, CardEffectStatus, CardPartId, CardType,
    CardTypeSet, CastChoices, CastCostContext, CastOffer, CastOfferCost, CastSignature,
    CastSourceZone, ControlFlow, CostConfiguration, DeclarativeAbilityDef, DividedTotal, Game,
    GameObjectId, KeywordAbility, ManaCost, ManaPaymentPurpose, ModeId, ObjectCharacteristics,
    OptionalAdditionalCostKindDef, PlayActionKind, PlayOptionDef, PlayOptionId, PlayRestriction,
    PlayerId, ScopedEffect, SelectedSpellPlan, StackAbilityPayload, StackAbilityResolver, Target,
    TargetSelection, TargetSlotDef, TargetSlotId, TemporaryAbilityGrant, TriggerContext,
    add_generic, add_mana_cost, extra_target_cost, mode_id_selections, positive_compositions,
    reduce_generic, target_combinations,
};

use crate::card::{AlternateSpellKind, CardStructure, ModeSetDef, SpellForm, ZoneKind};

mod cost_configurations;
pub(in crate::game) use cost_configurations::CastScale;

impl Game {
    pub(super) fn add_land_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        let state = &self.players[player.index()];
        if player != self.active_player
            || !self.step.is_main()
            || !self.stack.is_empty()
            || state.land_played_this_turn
        {
            return;
        }
        // A graveyard is walked too, for the permissions that reach into it.
        // Nothing there is playable without one, so the ordinary game pays
        // only the cost of the filter below.
        // The top of the library is walked for the same reason, and named
        // one card at a time.
        for (card, zone) in state
            .hand
            .iter()
            .map(|card| (card, ZoneKind::Hand))
            .chain(
                state
                    .graveyard
                    .iter()
                    .map(|card| (card, ZoneKind::Graveyard)),
            )
            .chain(state.library.last().map(|card| (card, ZoneKind::Library)))
        {
            let Some(definition) = self.catalog.get(card.definition) else {
                continue;
            };
            actions.extend(
                definition
                    .play_options
                    .iter()
                    .filter(|option| option.action == PlayActionKind::PlayLand)
                    .filter(|option| !self.play_is_prohibited(card, player, option))
                    .filter(|option| match zone {
                        ZoneKind::Graveyard => {
                            self.graveyard_play_is_permitted(card, player, option)
                        }
                        ZoneKind::Library => {
                            self.library_top_play_cost(card, player, option).is_some()
                        }
                        _ => true,
                    })
                    .filter(|option| match &option.form {
                        crate::card::SpellForm::Part(part) => definition
                            .part(*part)
                            .is_some_and(|part| part.rules.has_type(CardType::Land)),
                        crate::card::SpellForm::Combined(_) => false,
                    })
                    .map(|option| Action::PlayLand {
                        card: card.id,
                        option: option.id,
                    }),
            );
        }
    }

    pub(super) fn add_spell_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        self.add_castable_spell_actions(player, None, actions);
    }

    /// Every way `player` could cast `card` right now, ignoring the timing
    /// its type would normally impose. An offer made during a resolution is
    /// answered then or not at all (CR 608.2f), so a sorcery on the top of a
    /// library is castable in the middle of somebody else's turn.
    pub(super) fn add_offered_cast_actions(&self, offer: CastOffer, actions: &mut Vec<Action>) {
        self.add_castable_spell_actions(offer.player, Some(offer), actions);
    }

    pub(super) fn current_cast_offer(
        &self,
        player: PlayerId,
        card: GameObjectId,
        source_zone: CastSourceZone,
    ) -> Option<CastOffer> {
        self.pending_decisions
            .first()?
            .continuation
            .cast_offer()
            .filter(|offer| {
                offer.player == player && offer.card == card && offer.source_zone == source_zone
            })
    }

    #[allow(clippy::too_many_lines)]
    fn add_castable_spell_actions(
        &self,
        player: PlayerId,
        offer: Option<CastOffer>,
        actions: &mut Vec<Action>,
    ) {
        let state = &self.players[player.index()];
        for (card, source_zone) in state
            .hand
            .iter()
            .map(|card| (card, CastSourceZone::Hand))
            .chain(
                state
                    .graveyard
                    .iter()
                    .map(|card| (card, CastSourceZone::Graveyard)),
            )
            // Exile is walked for both players: a card on an adventure is
            // its owner's, but a card somebody took off the top of a library
            // is played from the exile of the player who owned it.
            .chain(
                self.players
                    .iter()
                    .flat_map(|state| state.exile.iter())
                    .filter(|card| self.exile_play_permission(card.id, player).is_some())
                    .map(|card| (card, CastSourceZone::Exile)),
            )
            // The top card of the caster's own library, which the
            // permissions that reach up there name one at a time.
            .chain(
                state
                    .library
                    .last()
                    .map(|card| (card, CastSourceZone::LibraryTop)),
            )
        {
            if offer.is_some_and(|offer| offer.card != card.id || offer.source_zone != source_zone)
            {
                continue;
            }
            // Energy replaces the mana cost rather than joining it, so a card
            // nobody has the energy for is not castable at all.
            if self
                .exile_energy_cost(card.id, player)
                .is_some_and(|energy| energy > self.players[player.index()].energy)
            {
                continue;
            }
            let Some(definition) = self.catalog.get(card.definition) else {
                continue;
            };
            for option in definition
                .play_options
                .iter()
                .filter(|option| option.action == PlayActionKind::CastSpell)
            {
                if self.play_is_prohibited(card, player, option) {
                    continue;
                }
                if source_zone != CastSourceZone::Hand
                    && option.restriction == PlayRestriction::FromHandOnly
                {
                    continue;
                }
                if source_zone == CastSourceZone::Exile
                    && !self.exile_play_is_permitted(definition, option, card.id, player)
                {
                    continue;
                }
                if source_zone == CastSourceZone::LibraryTop {
                    if self.library_top_play_cost(card, player, option).is_none() {
                        continue;
                    }
                    // Life replaces the mana cost rather than joining it, so
                    // a spell nobody has the life for is not castable this
                    // way at all. Paying down to exactly zero is allowed.
                    if self
                        .library_top_life_cost(card, player, option)
                        .is_some_and(|life| {
                            i64::from(life) > i64::from(self.players[player.index()].life)
                        })
                    {
                        continue;
                    }
                }
                if offer.is_none() && !self.play_timing_allows(player, option.restriction) {
                    continue;
                }
                // A declarative card intentionally has no custom behavior.
                // `Unsupported` is only a local neutral value for the legacy
                // helpers below; it is not stored as part of that card's rules.
                let behavior = Self::play_option_behavior(definition, option)
                    .unwrap_or(CardBehavior::Unsupported);
                let Some(types) = Self::play_option_types(definition, option) else {
                    continue;
                };
                // Metadata-only creatures retain baseline casting/combat. A
                // metadata-only noncreature spell or modal branch must not be
                // exposed as a legal action that would silently do nothing.
                if option.effect_status == CardEffectStatus::MetadataOnly && !types.is_creature() {
                    continue;
                }
                let part_has_flash = match &option.form {
                    crate::card::SpellForm::Part(part) => {
                        definition.part(*part).is_some_and(|part| {
                            part.rules.has_executable_keyword(KeywordAbility::Flash)
                        })
                    }
                    crate::card::SpellForm::Combined(parts) => parts.iter().any(|part| {
                        definition.part(*part).is_some_and(|part| {
                            part.rules.has_executable_keyword(KeywordAbility::Flash)
                        })
                    }),
                };
                // A granted flash covers the next sorcery whenever it is
                // cast, so it only matters when the timing would refuse.
                let granted_flash = types.contains(CardType::Sorcery)
                    && self.sorcery_flash_grants[player.index()] > 0;
                // An offer made during a resolution is answered then or not
                // at all (CR 608.2f), so it ignores the timing the card's
                // type would otherwise impose -- which is the only way a
                // cascaded sorcery, or one an Arcanist points at mid-combat,
                // is ever cast at all.
                if offer.is_none()
                    && !types.contains(CardType::Instant)
                    && !part_has_flash
                    && !granted_flash
                    && (player != self.active_player
                        || !self.step.is_main()
                        || !self.stack.is_empty())
                {
                    continue;
                }
                let payment_purpose = ManaPaymentPurpose::Spell {
                    object: card.id,
                    definition: card.definition,
                    controller: player,
                    form: option.form.clone(),
                };

                for modes in self.implemented_mode_selections(option, player, card.id) {
                    let declared_slots = Self::target_slots_for(option, &modes);
                    let _ = self.visit_cost_configurations(
                        definition,
                        card.id,
                        player,
                        option,
                        CastCostContext {
                            source_zone,
                            offer: offer.map(|offer| offer.cost),
                        },
                        |costs| {
                            let alternative_kind = self.selected_alternative_kind_for_offer(
                                definition,
                                option,
                                card.id,
                                &costs,
                                offer.map(|offer| offer.cost),
                            );
                            if alternative_kind == Some(AlternativeCastKindDef::Overload)
                                && !modes.is_empty()
                            {
                                return ControlFlow::Continue(());
                            }
                            let Some(cost) = self.configured_cast_mana_cost(
                                card.id,
                                option,
                                &costs,
                                offer.map(|offer| offer.cost),
                            ) else {
                                return ControlFlow::Continue(());
                            };
                            // X comes from the mana cost's {X}, from a
                            // printed "pay X life", or from both -- and a
                            // spell naming both is bounded by whichever runs
                            // out first.
                            let life_cost = Self::spell_life_cost(definition, option);
                            let mana_x = if cost.variable_x {
                                Some(self.maximum_x_for(player, cost, &payment_purpose))
                            } else {
                                None
                            };
                            let life_x = life_cost
                                .filter(|cost| cost.amount_is_x)
                                .map(|_| self.maximum_x_for_life(player));
                            let max_x = match (mana_x, life_x) {
                                (Some(mana), Some(life)) => mana.min(life),
                                (Some(bound), None) | (None, Some(bound)) => bound,
                                (None, None) => 0,
                            };
                            for x in 0..=max_x {
                                let cast_life = self.configured_cast_life_payment(
                                    definition,
                                    option,
                                    card.id,
                                    &costs,
                                    x,
                                    offer.map(|offer| offer.cost),
                                );
                                let library_life = if source_zone == CastSourceZone::LibraryTop {
                                    self.library_top_life_cost(card, player, option)
                                        .unwrap_or(0)
                                } else {
                                    0
                                };
                                let Some(life_available) = self.life_available_after_payment(
                                    player,
                                    cast_life.saturating_add(library_life),
                                ) else {
                                    continue;
                                };
                                let target_choices = if alternative_kind
                                    == Some(AlternativeCastKindDef::Overload)
                                {
                                    vec![Vec::new()]
                                } else if let Some(kicked) =
                                    Self::kicked_target_defs(definition, option, &costs)
                                {
                                    self.legal_ability_target_selections(
                                        kicked,
                                        player,
                                        card.id,
                                        TriggerContext::empty(),
                                        x,
                                    )
                                } else if let Some((_, ability)) =
                                    Self::spell_ability(definition, option)
                                {
                                    let DeclarativeAbilityDef::Spell(spell) = ability.definition
                                    else {
                                        unreachable!("spell_ability returns a spell clause")
                                    };
                                    let Some(plan) = Self::selected_spell_plan(spell, &modes)
                                    else {
                                        continue;
                                    };
                                    self.legal_ability_target_selections(
                                        &plan.target_defs,
                                        player,
                                        card.id,
                                        TriggerContext::empty(),
                                        x,
                                    )
                                } else if Self::uses_legacy_behavior_targets(definition, option) {
                                    self.legacy_target_selections(behavior, player)
                                } else {
                                    self.legal_target_selections(&declared_slots, x)
                                };
                                for targets in &target_choices {
                                    let target_count = targets
                                        .iter()
                                        .map(|selection| selection.targets().len())
                                        .sum();
                                    // Increases apply before discounts,
                                    // which is what keeps a discount from
                                    // eating generic mana an increase then
                                    // adds back (CR 601.2f).
                                    let payable_cost = reduce_generic(
                                        add_mana_cost(
                                            add_generic(
                                                cost,
                                                extra_target_cost(definition, target_count),
                                            ),
                                            self.spell_cost_increase(player, card.id),
                                        ),
                                        self.spell_cost_reduction(definition.id, player, card.id),
                                    );
                                    if !self.can_pay_cost_for_reserving_with_life(
                                        player,
                                        payable_cost,
                                        x,
                                        &payment_purpose,
                                        &[],
                                        life_available,
                                    ) {
                                        continue;
                                    }
                                    let sacrifice_choices = if behavior
                                        == CardBehavior::GoblinGrenade
                                    {
                                        self.battlefield
                                            .iter()
                                            .filter(|permanent| {
                                                permanent.controller == player
                                                    && self.effective_rules(permanent).is_some_and(
                                                        |rules| rules.has_subtype("Goblin"),
                                                    )
                                            })
                                            .map(|permanent| vec![permanent.card.id])
                                            .collect()
                                    } else {
                                        self.additional_cost_choices(
                                            definition,
                                            option,
                                            &costs,
                                            card,
                                            player,
                                            CastScale {
                                                x,
                                                modes: modes.len(),
                                                offer: offer.map(|offer| offer.cost),
                                            },
                                        )
                                    };
                                    for sacrifices in sacrifice_choices {
                                        if !self.can_pay_cost_for_reserving_with_life(
                                            player,
                                            payable_cost,
                                            x,
                                            &payment_purpose,
                                            &sacrifices,
                                            life_available,
                                        ) {
                                            continue;
                                        }
                                        actions.push(Action::CastSpell {
                                            card: card.id,
                                            choices: CastChoices::new(option.id)
                                                .with_modes(modes.clone())
                                                .with_costs(costs.clone())
                                                .with_x(x)
                                                .with_targets(targets.clone()),
                                            sacrifices,
                                        });
                                    }
                                }
                            }
                            ControlFlow::Continue(())
                        },
                    );
                }
            }
        }
    }

    pub(super) fn play_option_types(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<CardTypeSet> {
        match &option.form {
            crate::card::SpellForm::Part(part) => {
                definition.part(*part).map(|part| part.rules.types())
            }
            crate::card::SpellForm::Combined(parts) => {
                let mut combined = CardTypeSet::empty();
                let mut found = false;
                for part in parts {
                    combined = combined.union(definition.part(*part)?.rules.types());
                    found = true;
                }
                found.then_some(combined)
            }
        }
    }

    pub(super) fn play_option_behavior(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<CardBehavior> {
        let first = match &option.form {
            crate::card::SpellForm::Part(part) => *part,
            crate::card::SpellForm::Combined(parts) => *parts.first()?,
        };
        definition
            .part(first)
            .and_then(|part| part.rules.special_behavior())
    }

    /// The target slots a kicked cast declares. A kicked spell resolves its
    /// own clause, and that clause can name something the unkicked one
    /// cannot: Bloodchief's Thirst reaches past two mana only when kicked.
    /// So the enumeration and the validation both read the kicked slots
    /// rather than the base spell's, which is what the resolution already
    /// does. Overload is handled separately, having changed "target" to
    /// "each".
    pub(super) fn kicked_target_defs(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
    ) -> Option<&'static [AbilityTargetDef]> {
        let selected = costs.alternative()?;
        let (_, ability, AlternativeCastKindDef::Kicked) =
            Self::alternative_cast_ability(definition, option, selected)?
        else {
            return None;
        };
        let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition else {
            unreachable!("alternative_cast_ability returns an alternative-cast clause")
        };
        // A kicker that only costs more leaves the printed spell's targets
        // alone, so there is nothing here to read instead of them.
        if ability.declarative_effect() == Some(crate::card::EffectDef::None) {
            return None;
        }
        Some(alternative.targets)
    }

    /// Whether this play option is the one a card on an adventure may be
    /// cast with from exile: the main half of an Adventure card, never the
    /// adventure it just went on.
    pub(super) fn is_adventure_return_option(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> bool {
        let CardStructure::AlternateSpell {
            main,
            kind: AlternateSpellKind::Adventure,
            ..
        } = definition.structure
        else {
            return false;
        };
        option.form == SpellForm::Part(main)
    }

    pub(super) fn spell_ability(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<(AbilityOrigin, AbilityDef)> {
        let crate::card::SpellForm::Part(part_id) = &option.form else {
            return None;
        };
        let part_id = *part_id;
        let part = definition.part(part_id)?;
        part.rules
            .indexed_abilities()
            .find(|attached| {
                attached.definition.is_executable()
                    && matches!(
                        attached.definition.definition,
                        DeclarativeAbilityDef::Spell(_)
                    )
            })
            .map(|attached| {
                (
                    AbilityOrigin::Printed {
                        definition: definition.id,
                        part: part_id,
                        ability: attached.id,
                    },
                    attached.definition,
                )
            })
    }

    pub(super) fn selected_spell_plan(
        spell: crate::card::SpellAbilityDef,
        selected_modes: &[ModeId],
    ) -> Option<SelectedSpellPlan> {
        let mut target_defs = spell.targets().to_vec();
        if target_defs.len() > usize::from(u8::MAX) + 1 {
            return None;
        }
        if spell.modal().is_none() {
            return selected_modes.is_empty().then_some(SelectedSpellPlan {
                target_defs,
                mode_effects: Vec::new(),
            });
        }
        let mut selected = selected_modes.to_vec();
        selected.sort_by_key(|mode| mode.index());
        let mut mode_effects = Vec::with_capacity(selected.len());
        for selected in selected {
            let mode = spell.mode(selected)?;
            let effect = mode.declarative_effect()?;
            let DeclarativeAbilityDef::Spell(mode_spell) = mode.definition else {
                return None;
            };
            let target_base = target_defs.len();
            let target_count = mode_spell.targets().len();
            if target_base.checked_add(target_count)? > usize::from(u8::MAX) + 1 {
                return None;
            }
            target_defs.extend_from_slice(mode_spell.targets());
            mode_effects.push(ScopedEffect {
                effect,
                target_base,
            });
        }
        Some(SelectedSpellPlan {
            target_defs,
            mode_effects,
        })
    }

    pub(super) fn uses_legacy_behavior_targets(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> bool {
        matches!(
            (&definition.structure, &option.form),
            (
                crate::card::CardStructure::Single { main },
                crate::card::SpellForm::Part(part),
            ) if main == part
        ) && definition.play_options.len() == 1
            && option.id == PlayOptionId::DEFAULT
            && option.modes.is_none()
            && option.targets.is_empty()
            && Self::spell_ability(definition, option).is_none()
    }

    /// How many modes a spell may choose here. "If you control a Wizard as
    /// you cast this spell, you may choose two instead" is read where the
    /// spell is offered, which is what "as you cast" means, and it never
    /// lowers the printed maximum.
    pub(super) fn effective_mode_maximum(
        &self,
        mode_set: &ModeSetDef,
        controller: PlayerId,
        source: GameObjectId,
    ) -> u8 {
        let Some(conditional) = mode_set.conditional_maximum else {
            return mode_set.maximum;
        };
        if self.static_condition_holds(conditional.condition, controller, source) {
            mode_set.maximum.max(conditional.maximum)
        } else {
            mode_set.maximum
        }
    }

    pub(super) fn implemented_mode_selections(
        &self,
        option: &PlayOptionDef,
        controller: PlayerId,
        source: GameObjectId,
    ) -> Vec<Vec<ModeId>> {
        let Some(mode_set) = &option.modes else {
            return vec![Vec::new()];
        };
        let implemented = mode_set
            .modes
            .iter()
            .filter(|mode| mode.effect_status == CardEffectStatus::Implemented)
            .map(|mode| mode.id)
            .collect::<Vec<_>>();
        let mut implemented = implemented;
        implemented.sort_unstable();
        mode_id_selections(
            &implemented,
            usize::from(mode_set.minimum),
            usize::from(self.effective_mode_maximum(mode_set, controller, source)),
            mode_set.may_repeat,
        )
    }

    pub(super) fn target_slots_for(option: &PlayOptionDef, modes: &[ModeId]) -> Vec<TargetSlotDef> {
        let mut slots = option.targets.clone();
        if let Some(mode_set) = &option.modes {
            for mode in modes {
                if let Some(mode) = mode_set
                    .modes
                    .iter()
                    .find(|candidate| candidate.id == *mode)
                {
                    slots.extend(mode.targets.clone());
                }
            }
        }
        for (index, slot) in slots.iter_mut().enumerate() {
            slot.id = TargetSlotId::from_index(index)
                .expect("one play option presents at most 256 target slots");
        }
        slots
    }

    pub(super) fn legacy_target_selections(
        &self,
        behavior: CardBehavior,
        player: PlayerId,
    ) -> Vec<Vec<TargetSelection>> {
        self.legal_target_lists(behavior, player, None)
            .into_iter()
            .map(|targets| {
                if targets.is_empty() {
                    Vec::new()
                } else {
                    vec![TargetSelection::new(TargetSlotId(0), targets)]
                }
            })
            .collect()
    }

    pub(super) fn legal_target_selections(
        &self,
        slots: &[TargetSlotDef],
        x: u16,
    ) -> Vec<Vec<TargetSelection>> {
        let mut selections = vec![Vec::new()];
        for slot in slots {
            let candidates = self.targets_matching(slot.predicate);
            let mut choices = Vec::new();
            if let Some(total) = slot.divided_total {
                let total = match total {
                    DividedTotal::Fixed(total) => total,
                    DividedTotal::ChosenX => u8::try_from(x).unwrap_or(u8::MAX),
                };
                // Every chosen target takes at least one, so the number of
                // targets follows from how the total is split.
                for count in 1..=usize::from(total).min(candidates.len()) {
                    for targets in target_combinations(&candidates, count) {
                        for amounts in positive_compositions(total, count) {
                            choices.push(TargetSelection::divided(
                                slot.id,
                                targets.clone(),
                                amounts,
                            ));
                        }
                    }
                }
                let mut combined = Vec::new();
                for prefix in &selections {
                    for choice in &choices {
                        let mut selected = prefix.clone();
                        selected.push(choice.clone());
                        combined.push(selected);
                    }
                }
                selections = combined;
                continue;
            }
            // Clamped to what is actually on the board, the way the divided
            // branch above already does. A slot with no printed limit says so
            // with a sentinel, and every count past the candidate list would
            // enumerate nothing anyway.
            let maximum = usize::from(slot.maximum).min(candidates.len());
            for count in usize::from(slot.minimum)..=maximum {
                choices.extend(
                    target_combinations(&candidates, count)
                        .into_iter()
                        .map(|targets| TargetSelection::new(slot.id, targets)),
                );
            }
            let mut combined = Vec::new();
            for prefix in &selections {
                for choice in &choices {
                    let mut selected = prefix.clone();
                    selected.push(choice.clone());
                    combined.push(selected);
                }
            }
            selections = combined;
        }
        selections
    }

    pub(super) fn legal_ability_target_selections(
        &self,
        slots: &[AbilityTargetDef],
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        x: u16,
    ) -> Vec<Vec<TargetSelection>> {
        let mut selections = vec![Vec::new()];
        for (index, slot) in slots.iter().enumerate() {
            let id = TargetSlotId::from_index(index)
                .expect("validated ability targets fit the runtime slot space");
            // A slot that reads an earlier slot's choice has to be enumerated
            // once per prefix, because its candidates are different for each.
            if let AbilityTargetPredicate::ControlledByTargetOf {
                object,
                slot: other,
            } = slot.predicate
            {
                let other = TargetSlotId::from_index(other.index())
                    .expect("validated dependent target fits the runtime slot space");
                let mut combined = Vec::new();
                for prefix in &selections {
                    let candidates = prefix
                        .iter()
                        .find(|selection: &&TargetSelection| selection.slot() == other)
                        .and_then(|selection| selection.targets().first().copied())
                        .and_then(|target| match target {
                            Target::Player(player) => Some(player),
                            Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                                self.current_or_last_known_controller(id)
                            }
                        })
                        .map_or_else(Vec::new, |owner| {
                            self.battlefield
                                .iter()
                                .filter(|permanent| permanent.controller == owner)
                                .filter(|permanent| {
                                    self.trigger_object_matches(
                                        object,
                                        &self.trigger_event_object(permanent),
                                        source,
                                        false,
                                    ) && self
                                        .permanent_can_be_targeted_by(permanent, controller, source)
                                })
                                .map(|permanent| Target::Permanent(permanent.card.id))
                                .collect::<Vec<_>>()
                        });
                    let (minimum, maximum) = slot.count_bounds(x);
                    for count in minimum..=maximum {
                        for targets in target_combinations(&candidates, usize::from(count)) {
                            let mut selected = prefix.clone();
                            selected.push(TargetSelection::new(id, targets));
                            combined.push(selected);
                        }
                    }
                }
                selections = combined;
                continue;
            }
            let candidates =
                self.ability_targets_matching_at(slot.predicate, controller, source, context, x);
            let mut choices = Vec::new();
            if let Some(total) = slot.divided_total {
                let total = match total {
                    DividedTotal::Fixed(total) => total,
                    DividedTotal::ChosenX => u8::try_from(x).unwrap_or(u8::MAX),
                };
                // Every chosen target takes at least one, so the number of
                // targets follows from how the total is split.
                for count in 1..=usize::from(total).min(candidates.len()) {
                    for targets in target_combinations(&candidates, count) {
                        for amounts in positive_compositions(total, count) {
                            choices.push(TargetSelection::divided(id, targets.clone(), amounts));
                        }
                    }
                }
                let mut combined = Vec::new();
                for prefix in &selections {
                    for choice in &choices {
                        let mut selected = prefix.clone();
                        selected.push(choice.clone());
                        combined.push(selected);
                    }
                }
                selections = combined;
                continue;
            }
            let (minimum, maximum) = slot.count_bounds(x);
            for count in minimum..=maximum {
                choices.extend(
                    target_combinations(&candidates, usize::from(count))
                        .into_iter()
                        .map(|targets| TargetSelection::new(id, targets)),
                );
            }
            let mut combined = Vec::new();
            for prefix in &selections {
                for choice in &choices {
                    // "Another" is a restriction on the declaration, so a
                    // repeat is never offered rather than being caught later.
                    if slot.another && names_an_earlier_target(prefix, choice) {
                        continue;
                    }
                    let mut selected = prefix.clone();
                    selected.push(choice.clone());
                    combined.push(selected);
                }
            }
            selections = combined;
        }
        selections
    }
}

/// Whether a slot's choice names anything the slots before it already did.
fn names_an_earlier_target(prefix: &[TargetSelection], choice: &TargetSelection) -> bool {
    prefix.iter().any(|earlier| {
        earlier
            .targets()
            .iter()
            .any(|target| choice.targets().contains(target))
    })
}

include!("casting_actions/selected_clause.rs");
