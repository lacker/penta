//! Checking that a submitted cast's choices are ones the card offers.
//!
//! Separate from performing the cast because it answers a different
//! question: an action arrives from a client or a bot claiming a play
//! option, a set of modes, a cost configuration, and a list of targets, and
//! none of it can be trusted until it has been matched against what the
//! card actually prints.

use super::super::ManaPaymentPurpose;
use super::super::{
    AbilityTargetDef, AlternativeCastKindDef, CardBehavior, CardEffectStatus, CastChoices,
    CastCostContext, CastSignature, CastSourceZone, ControlFlow, DeclarativeAbilityDef, Game,
    GameObjectId, ManaCost, PlayActionKind, PlayOptionDef, PlayRestriction, PlayerId, Target,
    TargetPredicate, TargetSlotDef, TargetSlotId, TriggerContext, add_generic, add_mana_cost,
    extra_target_cost, reduce_generic,
};

impl Game {
    pub(in crate::game) fn mode_selection_is_valid(
        &self,
        option: &PlayOptionDef,
        choices: &CastChoices,
        controller: PlayerId,
        source: GameObjectId,
    ) -> bool {
        match &option.modes {
            None => choices.modes().is_empty(),
            Some(mode_set) => {
                let count = choices.modes().len();
                let maximum = self.effective_mode_maximum(mode_set, controller, source);
                if count < usize::from(mode_set.minimum) || count > usize::from(maximum) {
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
    pub(in crate::game) fn spell_target_selection_is_valid(
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
                    let legal = self.ability_targets_matching_at(
                        slot.predicate,
                        player,
                        card_id,
                        TriggerContext::empty(),
                        choices.x(),
                    );
                    // Read through the same sentinel the enumerator used, so
                    // a slot counted by X is checked against the X this cast
                    // actually chose rather than against the sentinel.
                    let (minimum, maximum) = slot.count_bounds(choices.x());
                    TargetSlotId::from_index(index) == Some(selection.slot())
                        && count >= usize::from(minimum)
                        && count <= usize::from(maximum)
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
    pub(in crate::game) fn declared_slot_selection_is_valid(
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
    pub(in crate::game) fn validated_cast_signature(
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
            })
            .or_else(|| {
                self.players
                    .iter()
                    .flat_map(|state| state.exile.iter())
                    .find(|card| {
                        card.id == card_id && self.exile_play_permission(card_id, player).is_some()
                    })
                    .map(|card| (card, CastSourceZone::Exile))
            })
            .or_else(|| {
                state
                    .library
                    .last()
                    .filter(|card| card.id == card_id)
                    .map(|card| (card, CastSourceZone::LibraryTop))
            })?;
        let definition = self.catalog.get(card.definition)?;
        let offer = self.current_cast_offer(player, card_id, source_zone);
        let option = definition
            .play_option(choices.play_option())
            .filter(|option| option.action == PlayActionKind::CastSpell)?;
        if self.play_is_prohibited(card, player, option) {
            return None;
        }
        if source_zone != CastSourceZone::Hand
            && option.restriction == PlayRestriction::FromHandOnly
        {
            return None;
        }
        if source_zone == CastSourceZone::Exile
            && !self.exile_play_is_permitted(definition, option, card_id, player)
        {
            return None;
        }
        if source_zone == CastSourceZone::LibraryTop {
            self.library_top_play_cost(card, player, option)?;
            if self
                .library_top_life_cost(card, player, option)
                .is_some_and(|life| i64::from(life) > i64::from(self.players[player.index()].life))
            {
                return None;
            }
        }
        if offer.is_none() && !self.play_timing_allows(player, option.restriction) {
            return None;
        }
        let behavior =
            Self::play_option_behavior(definition, option).unwrap_or(CardBehavior::Unsupported);
        let types = Self::play_option_types(definition, option)?;
        if option.effect_status == CardEffectStatus::MetadataOnly
            && (!types.is_creature()
                || !definition.play_option_has_executable_creature_body(option))
        {
            return None;
        }

        if !self.mode_selection_is_valid(option, choices, player, card_id) {
            return None;
        }

        if !self
            .visit_cost_configurations(
                definition,
                card_id,
                player,
                option,
                CastCostContext {
                    source_zone,
                    offer: offer.map(|offer| offer.cost),
                },
                |costs| {
                    if &costs == choices.costs() {
                        ControlFlow::Break(())
                    } else {
                        ControlFlow::Continue(())
                    }
                },
            )
            .is_break()
        {
            return None;
        }
        let alternative_kind = self.selected_alternative_kind_for_offer(
            definition,
            option,
            card_id,
            choices.costs(),
            offer.map(|offer| offer.cost),
        );
        if alternative_kind == Some(AlternativeCastKindDef::Overload) && !choices.modes().is_empty()
        {
            return None;
        }
        let mut cost = self.configured_cast_mana_cost(
            card_id,
            option,
            choices.costs(),
            offer.map(|offer| offer.cost),
        )?;
        // X comes from the mana cost's {X} or from a printed "pay X life",
        // and a spell with neither is cast for nothing at all.
        let life_cost = Self::spell_life_cost(definition, option);
        let x_is_chosen = cost.variable_x || life_cost.is_some_and(|cost| cost.amount_is_x);
        if !x_is_chosen && choices.x() != 0 {
            return None;
        }
        let cast_life = self.configured_cast_life_payment(
            definition,
            option,
            card_id,
            choices.costs(),
            choices.x(),
            offer.map(|offer| offer.cost),
        );
        let library_life = if source_zone == CastSourceZone::LibraryTop {
            self.library_top_life_cost(card, player, option)
                .unwrap_or(0)
        } else {
            0
        };
        let declared_slots = Self::target_slots_for(option, choices.modes());
        if alternative_kind == Some(AlternativeCastKindDef::Overload) {
            if !choices.targets().is_empty() {
                return None;
            }
        } else if let Some(kicked) = Self::kicked_target_defs(definition, option, choices.costs()) {
            if kicked.len() != choices.targets().len() {
                return None;
            }
            if !self.spell_target_selection_is_valid(kicked, choices, player, card_id) {
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
        cost = add_mana_cost(cost, self.spell_cost_increase(player, card_id));
        let (cost, phyrexian_life) = Self::locked_mana_payment(cost, choices.mana_payment())?;
        let cost = reduce_generic(
            cost,
            self.spell_cost_reduction(definition.id, player, card_id),
        );
        let total_life = cast_life
            .saturating_add(library_life)
            .saturating_add(phyrexian_life);
        let life_available = self.life_available_after_payment(player, total_life)?;
        let payment_purpose = ManaPaymentPurpose::Spell {
            object: card_id,
            definition: definition.id,
            controller: player,
            form: option.form.clone(),
            reserved_life_payment: total_life,
        };
        if cost.variable_x && choices.x() > self.maximum_x_for(player, cost, &payment_purpose) {
            return None;
        }
        if !self.can_pay_cost_for_reserving_with_life(
            player,
            cost,
            choices.x(),
            &payment_purpose,
            &[],
            life_available,
        ) {
            return None;
        }

        Some((
            CastSignature::from_validated_choices(option.form.clone(), choices.clone()),
            cost,
            behavior,
            source_zone,
        ))
    }

    pub(in crate::game) fn target_matches(
        &self,
        predicate: TargetPredicate,
        target: Target,
    ) -> bool {
        self.targets_matching(predicate).contains(&target)
    }
}
