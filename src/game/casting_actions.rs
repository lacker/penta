use crate::card::CostQuantityDef;

use super::{
    AbilityDef, AbilityId, AbilityOrigin, AbilityTargetDef, Action, AdditionalCostId,
    AlternativeCastAbilityDef, AlternativeCastKindDef, AlternativeCostId, CardBehavior,
    CardDefinition, CardDefinitionId, CardEffectStatus, CardPartId, CardType, CardTypeSet,
    CastChoices, CastCostContext, CastOffer, CastOfferCost, CastSignature, CastSourceZone,
    ControlFlow, CostConfiguration, CounterKind, DeclarativeAbilityDef, DividedTotal, Game,
    GameObjectId, KeywordAbility, ManaCost, ManaPaymentPurpose, ModeId, NonbattlefieldAbilityGrant,
    ObjectCharacteristics, OptionalAdditionalCostKindDef, PlayActionKind, PlayOptionDef,
    PlayOptionId, PlayRestriction, PlayerId, ScopedEffect, SelectedSpellPlan, StackAbilityPayload,
    StackAbilityResolver, TargetSelection, TargetSlotDef, TargetSlotId, TriggerContext,
    add_generic, add_mana_cost, extra_target_cost, mode_id_selections, positive_compositions,
    target_combinations,
};

use crate::card::{
    AlternateSpellKind, CardStructure, ModeSetDef, ObjectPredicateDef, SpellAdditionalCostDef,
    SpellForm, ZoneKind,
};
use crate::game::mana_planning::reduce_generic;

mod cost_configurations;
mod mana_payments;
pub(in crate::game) use cost_configurations::{
    CastScale, SpellAdditionalCostPayment, SpellAdditionalCostRequest,
};

impl Game {
    /// Whether this player could cast a sorcery right now: their own main
    /// phase, with nothing waiting on the stack (CR 307.1).
    pub(super) fn sorcery_speed_window(&self, player: PlayerId) -> bool {
        player == self.active_player && self.step.is_main() && self.stack.is_empty()
    }

    pub(super) fn add_land_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        let state = &self.players[player.index()];
        if player != self.active_player
            || !self.step.is_main()
            || !self.stack.is_empty()
            || (state.lands_played_this_turn > self.additional_land_plays(player)
                && !self.player_rule_applies(
                    player,
                    crate::card::AppliedRuleDef::MayPlayAnyNumberOfLands,
                ))
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
            // Exile is walked for both players, the way the cast offers walk
            // it: a permission to *play* a card reaches a land, and a land
            // somebody else exiled is still played from where it lies. A
            // permission to *cast* one does not -- playing a land is not
            // casting a spell (CR 305.1).
            .chain(
                self.players
                    .iter()
                    .flat_map(|state| state.exile.iter())
                    .filter(|card| {
                        self.exile_play_permission(card.id, player)
                            .is_some_and(|permission| permission.lands_may_be_played)
                    })
                    .map(|card| (card, ZoneKind::Exile)),
            )
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
                        // The permission was already checked to get here.
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
        // Split second stops the casting a player does with priority in hand
        // and nothing else: an offer made inside a resolution is answered
        // then or not at all, and cascade off a split-second spell still
        // casts what it found.
        if self.split_second_is_active() {
            return;
        }
        self.add_castable_spell_actions(player, None, actions);
    }

    /// Whether the selected spell form may begin to be cast in the current
    /// priority window. Suspend asks this exact question without casting the
    /// card, so the timing rule lives beside ordinary cast enumeration rather
    /// than inside the suspend procedure.
    pub(super) fn spell_form_timing_allows(
        &self,
        definition: &CardDefinition,
        card: &super::CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
        types: CardTypeSet,
    ) -> bool {
        let part_has_flash = match &option.form {
            crate::card::SpellForm::Part(part) => definition
                .part(*part)
                .is_some_and(|part| part.rules.has_executable_keyword(KeywordAbility::Flash)),
            crate::card::SpellForm::Combined(parts) => parts.iter().any(|part| {
                definition
                    .part(*part)
                    .is_some_and(|part| part.rules.has_executable_keyword(KeywordAbility::Flash))
            }),
        };
        types.contains(CardType::Instant)
            || part_has_flash
            || self.cast_as_though_it_had_flash(card, player, option)
            || self.sorcery_speed_window(player)
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
                    // A standing offer is itself the permission: rebound
                    // lends its own card back without granting exile the
                    // general permission an impulse draw does.
                    .filter(|card| {
                        self.exile_play_permission(card.id, player).is_some()
                            || offer.is_some_and(|offer| offer.card == card.id)
                    })
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
                .is_some_and(|energy| {
                    energy
                        > self.players[player.index()]
                            .counters
                            .count(CounterKind::named("energy"))
                })
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
                    && offer.is_none()
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
                if option.effect_status == CardEffectStatus::MetadataOnly
                    && (!types.is_creature()
                        || !definition.play_option_has_executable_creature_body(option))
                {
                    continue;
                }
                // An offer made during a resolution is answered then or not
                // at all (CR 608.2f), so it ignores the timing the card's
                // type would otherwise impose -- which is the only way a
                // cascaded sorcery, or one an Arcanist points at mid-combat,
                // is ever cast at all.
                if offer.is_none()
                    && !self.spell_form_timing_allows(definition, card, player, option, types)
                {
                    continue;
                }
                for spliced in self.splice_selections(definition, player, card.id) {
                    let Some(splice_clauses) = self.spliced_clauses_of(&spliced) else {
                        continue;
                    };
                    let splice_cost = self.total_splice_cost(&spliced);
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
                                    player,
                                    card.id,
                                    option,
                                    &costs,
                                    offer.map(|offer| offer.cost),
                                ) else {
                                    return ControlFlow::Continue(());
                                };
                                // A splice cost is paid as part of casting the
                                // spell it is spliced onto, so it joins the cost
                                // before anything else is worked out from it.
                                let cost = add_mana_cost(cost, splice_cost);
                                // X comes from the mana cost or any semantic
                                // additional-cost branch that names it. A spell
                                // naming more than one is bounded by whichever
                                // required resource runs out first.
                                let mana_x = if cost.variable_x {
                                    let increased = add_mana_cost(
                                        cost,
                                        self.spell_cost_increase(player, card.id, &[]),
                                    );
                                    Some(
                                        Self::mana_payment_choices(increased)
                                            .into_iter()
                                            .filter_map(|choice| {
                                                let (locked, phyrexian_life) =
                                                    Self::locked_mana_payment(
                                                        increased,
                                                        &choice,
                                                        self.card_mana_is_any_color(card.id),
                                                    )?;
                                                let total_life = self
                                                    .configured_cast_life_payment(
                                                        definition,
                                                        option,
                                                        card.id,
                                                        &costs,
                                                        0,
                                                        offer.map(|offer| offer.cost),
                                                    )
                                                    .saturating_add(phyrexian_life);
                                                self.life_available_after_payment(
                                                    player, total_life,
                                                )?;
                                                let exact_purpose = ManaPaymentPurpose::Spell {
                                                    object: card.id,
                                                    definition: card.definition,
                                                    controller: player,
                                                    form: option.form.clone(),
                                                    reserved_life_payment: total_life,
                                                };
                                                let maximum = self.maximum_x_for(
                                                    player,
                                                    Self::apply_spell_cost_reduction(
                                                        locked,
                                                        self.spell_cost_reduction(
                                                            definition.id,
                                                            player,
                                                            card.id,
                                                            &[],
                                                        ),
                                                    ),
                                                    &exact_purpose,
                                                );
                                                Some(maximum)
                                            })
                                            .max()
                                            .unwrap_or(0),
                                    )
                                } else {
                                    None
                                };
                                let additional_x = self.maximum_x_for_spell_additional_costs(
                                    SpellAdditionalCostRequest {
                                        definition,
                                        option,
                                        costs: &costs,
                                        card,
                                        player,
                                        modes: &modes,
                                        scale: CastScale {
                                            x: 0,
                                            modes: modes.len(),
                                            offer: offer.map(|offer| offer.cost),
                                        },
                                    },
                                );
                                let max_x = [mana_x, additional_x]
                                    .into_iter()
                                    .flatten()
                                    .min()
                                    .unwrap_or(0);
                                // "X can't be 0" starts the enumeration higher:
                                // the kicked cast is only ever offered for an X
                                // its own clause allows.
                                let min_x = Self::configured_alternative_minimum_x(
                                    definition, option, &costs,
                                );
                                for x in min_x..=max_x {
                                    let additional_cost_payments =
                                        Self::additional_cost_payment_counts_for(option, &costs);
                                    let base_cast_life = self.configured_cast_life_payment(
                                        definition,
                                        option,
                                        card.id,
                                        &costs,
                                        x,
                                        offer.map(|offer| offer.cost),
                                    );
                                    let additional_payments = if behavior
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
                                            .map(|permanent| SpellAdditionalCostPayment {
                                                objects: vec![(
                                                    permanent.card.id,
                                                    SpellAdditionalCostDef::sacrifice(
                                                        ObjectPredicateDef::Any,
                                                        CostQuantityDef::Fixed(1),
                                                    ),
                                                )],
                                                mana: ManaCost::default(),
                                                life: 0,
                                            })
                                            .collect()
                                    } else {
                                        self.spell_additional_cost_payments(
                                            SpellAdditionalCostRequest {
                                                definition,
                                                option,
                                                costs: &costs,
                                                card,
                                                player,
                                                modes: &modes,
                                                scale: CastScale {
                                                    x,
                                                    modes: modes.len(),
                                                    offer: offer.map(|offer| offer.cost),
                                                },
                                            },
                                        )
                                    };
                                    let library_life = if source_zone == CastSourceZone::LibraryTop
                                    {
                                        self.library_top_life_cost(card, player, option)
                                            .unwrap_or(0)
                                    } else {
                                        0
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
                                            &additional_cost_payments,
                                        )
                                    } else if let Some((_, ability)) =
                                        Self::spell_ability(definition, option)
                                    {
                                        let DeclarativeAbilityDef::Spell(spell) =
                                            ability.definition
                                        else {
                                            unreachable!("spell_ability returns a spell clause")
                                        };
                                        let Some(plan) = Self::selected_spell_plan(
                                            spell,
                                            &modes,
                                            &splice_clauses,
                                        ) else {
                                            continue;
                                        };
                                        self.legal_ability_target_selections(
                                            &plan.target_defs,
                                            player,
                                            card.id,
                                            TriggerContext::empty(),
                                            x,
                                            &additional_cost_payments,
                                        )
                                    } else if Self::uses_legacy_behavior_targets(definition, option)
                                    {
                                        self.legacy_target_selections(behavior, player, card.id)
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
                                        for additional_payment in &additional_payments {
                                            let cast_life = base_cast_life
                                                .saturating_add(additional_payment.life);
                                            let increased_cost = add_mana_cost(
                                                add_mana_cost(
                                                    add_generic(
                                                        cost,
                                                        extra_target_cost(definition, target_count),
                                                    ),
                                                    additional_payment.mana,
                                                ),
                                                self.spell_cost_increase(player, card.id, targets),
                                            );
                                            for mana_payment in
                                                Self::mana_payment_choices(increased_cost)
                                            {
                                                let Some((locked_cost, phyrexian_life)) =
                                                    Self::locked_mana_payment(
                                                        increased_cost,
                                                        &mana_payment,
                                                        self.card_mana_is_any_color(card.id),
                                                    )
                                                else {
                                                    continue;
                                                };
                                                let payable_cost = Self::apply_spell_cost_reduction(
                                                    locked_cost,
                                                    self.spell_cost_reduction(
                                                        definition.id,
                                                        player,
                                                        card.id,
                                                        targets,
                                                    ),
                                                );
                                                let Some(life_available) = self
                                                    .life_available_after_payment(
                                                        player,
                                                        cast_life
                                                            .saturating_add(library_life)
                                                            .saturating_add(phyrexian_life),
                                                    )
                                                else {
                                                    continue;
                                                };
                                                let exact_purpose = ManaPaymentPurpose::Spell {
                                                    object: card.id,
                                                    definition: card.definition,
                                                    controller: player,
                                                    form: option.form.clone(),
                                                    reserved_life_payment: cast_life
                                                        .saturating_add(library_life)
                                                        .saturating_add(phyrexian_life),
                                                };
                                                let sacrifices = additional_payment.object_ids();
                                                // Emerge is the one alternative
                                                // whose cost the sacrifice
                                                // settles, so the reduction is
                                                // read per way of paying it.
                                                let payable_cost = reduce_generic(
                                                    payable_cost,
                                                    self.emerge_generic_reduction(
                                                        alternative_kind,
                                                        &sacrifices,
                                                    ),
                                                );
                                                if !self.can_pay_cost_for_reserving_with_life(
                                                    player,
                                                    payable_cost,
                                                    x,
                                                    &exact_purpose,
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
                                                        .with_mana_payment(mana_payment.clone())
                                                        .with_x(x)
                                                        .with_targets(targets.clone())
                                                        .with_spliced(spliced.clone()),
                                                    sacrifices,
                                                });
                                            }
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
        let (_, ability, AlternativeCastKindDef::Kicked | AlternativeCastKindDef::Bestow) =
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
        source: GameObjectId,
    ) -> Vec<Vec<TargetSelection>> {
        self.legal_target_lists(behavior, player, None, source)
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
        additional_cost_payments: &[u16],
    ) -> Vec<Vec<TargetSelection>> {
        let mut selections = vec![Vec::new()];
        for (index, slot) in slots.iter().enumerate() {
            let id = TargetSlotId::from_index(index)
                .expect("validated ability targets fit the runtime slot space");
            let mut combined = Vec::new();
            for prefix in &selections {
                // A complete target alternative may depend on an earlier
                // slot, so candidates are always evaluated per prefix.
                let candidates = Self::without_excluded_source(
                    slot,
                    source,
                    self.ability_targets_matching_with_selections_at(
                        slot.predicate,
                        prefix,
                        controller,
                        source,
                        context,
                        x,
                        additional_cost_payments,
                    ),
                );
                let mut choices = Vec::new();
                if let Some(total) = slot.divided_total {
                    let total = match total {
                        DividedTotal::Fixed(total) => total,
                        DividedTotal::ChosenX => u8::try_from(x).unwrap_or(u8::MAX),
                    };
                    // Every chosen target takes at least one, so the number
                    // of targets follows from how the total is split.
                    for count in 1..=usize::from(total).min(candidates.len()) {
                        for targets in target_combinations(&candidates, count) {
                            for amounts in positive_compositions(total, count) {
                                choices.push(TargetSelection::divided(
                                    id,
                                    targets.clone(),
                                    amounts,
                                ));
                            }
                        }
                    }
                } else {
                    let (minimum, maximum) = slot.count_bounds(x, additional_cost_payments);
                    for count in minimum..=maximum {
                        choices.extend(
                            target_combinations(&candidates, usize::from(count))
                                .into_iter()
                                .map(|targets| TargetSelection::new(id, targets)),
                        );
                    }
                }
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

include!("casting_actions/spliced_plan.rs");
