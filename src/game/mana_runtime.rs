use super::{
    AbilityCostDef, AbilityDef, AbilityOrigin, AbilityProcedureDef, Action, ActivatedAbilityDef,
    AddManaEffectDef, AppliedStackEffect, CardBehavior, CardType, CharacteristicContext,
    CounterKind, DeclarativeAbilityDef, EffectDef, Game, GameObjectId, Mana, ManaAbilityActivation,
    ManaColor, ManaCost, ManaPaymentPurpose, ManaPool, ManaRestrictionDef, ManaSelectionDef,
    ManaSource, ManaSpendEffectDef, Permanent, PlayerId, RetiredObject, StackObject,
    TriggerEventObject, ZoneKind, fold_restricted_x, pay_cost_with_orders,
};

impl Game {
    pub(super) fn mana_ability_is_usable(
        &self,
        permanent: &Permanent,
        definition: ActivatedAbilityDef,
    ) -> bool {
        let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
        definition.source_zones.contains(&ZoneKind::Battlefield)
            && !definition.costs.as_slice().is_empty()
            && definition
                .costs
                .iter()
                .filter(|cost| matches!(cost, AbilityCostDef::SacrificeSource))
                .count()
                <= 1
            && !(taps_source && (permanent.tapped || !self.can_use_tap_ability(permanent)))
            && definition.costs.iter().all(|cost| {
                matches!(
                    cost,
                    AbilityCostDef::TapSource
                        | AbilityCostDef::SacrificeSource
                        | AbilityCostDef::RemoveCountersFromSource { .. }
                        | AbilityCostDef::PayLife(_)
                )
            })
            && definition.costs.iter().all(|cost| match cost {
                AbilityCostDef::PayLife(amount) => {
                    self.players[permanent.controller.index()].life
                        >= i16::try_from(*amount).unwrap_or(i16::MAX)
                }
                AbilityCostDef::RemoveCountersFromSource { .. }
                | AbilityCostDef::Mana(_)
                | AbilityCostDef::TapSource
                | AbilityCostDef::UntapSource
                | AbilityCostDef::SacrificeSource
                | AbilityCostDef::DiscardSource
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::ExileSource
                | AbilityCostDef::Loyalty(_)
                | AbilityCostDef::ExileCardFromGraveyard(_)
                | AbilityCostDef::Special(_) => true,
            })
            && Self::source_counter_costs_are_payable(permanent, definition.costs.as_slice())
    }

    pub(super) fn source_counter_costs_are_payable(
        permanent: &Permanent,
        costs: &[AbilityCostDef],
    ) -> bool {
        let mut required = [0_u32; CounterKind::COUNT];
        for cost in costs {
            if let AbilityCostDef::RemoveCountersFromSource { kind, amount } = cost {
                required[kind.index()] = required[kind.index()].saturating_add(u32::from(*amount));
            }
        }
        CounterKind::ALL
            .into_iter()
            .all(|kind| u32::from(permanent.counters(kind)) >= required[kind.index()])
    }

    pub(super) fn mana_ability_activations(
        &self,
        permanent: &Permanent,
    ) -> Vec<ManaAbilityActivation> {
        let mut activations = Vec::new();
        self.for_each_effective_ability(permanent, |effective| {
            let ability = effective.ability;
            if !ability.is_executable() {
                return;
            }
            let DeclarativeAbilityDef::ActivatedMana(definition) = ability.definition else {
                return;
            };
            if !self.mana_ability_is_usable(permanent, definition) {
                return;
            }
            activations.extend(self.mana_activations_for(
                permanent,
                effective.origin,
                definition,
                &ability,
            ));
        });
        activations
    }

    /// The concrete activations one mana ability offers, which is one per
    /// colour it can produce.
    pub(super) fn mana_activations_for(
        &self,
        permanent: &Permanent,
        origin: AbilityOrigin,
        definition: ActivatedAbilityDef,
        ability: &AbilityDef,
    ) -> Vec<ManaAbilityActivation> {
        let mut activations = Vec::new();
        match ability.effect.definition {
            EffectDef::AddMana(effect)
                if definition.procedure == AbilityProcedureDef::Shared
                    && ability.declarative_effect().is_some() =>
            {
                let mut add_activation = |color| {
                    activations.push(ManaAbilityActivation {
                        source: permanent.card.id,
                        ability: origin,
                        color,
                        costs: definition.costs,
                        effect,
                    });
                };
                match effect.mana {
                    ManaSelectionDef::One(color) => {
                        add_activation(color);
                    }
                    ManaSelectionDef::Choice(colors) => {
                        for color in colors {
                            add_activation(*color);
                        }
                    }
                }
            }
            EffectDef::Special(_)
                if definition.procedure == AbilityProcedureDef::Legacy
                    && ability.custom_behavior() == Some(CardBehavior::FellwarStone) =>
            {
                activations.extend(self.fellwar_stone_activations(
                    permanent,
                    origin,
                    definition.costs,
                ));
            }
            EffectDef::AddMana(_)
            | EffectDef::AddManaEqualTo { .. }
            | EffectDef::None
            | EffectDef::Sequence(_)
            | EffectDef::DealDamage { .. }
            | EffectDef::DrainLife { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::Discard { .. }
            | EffectDef::ShuffleLibrary { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::PreventCombatDamageThisTurn { .. }
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::DestroyOfChoice { .. }
            | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
            | EffectDef::RevealAndSplitIntoPiles { .. }
            | EffectDef::LoseTheGame { .. }
            | EffectDef::Mill { .. }
            | EffectDef::LookAtTopAndMayTake { .. }
            | EffectDef::LookAtHand { .. }
            | EffectDef::SearchLibrary { .. }
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalManaPayment { .. }
            | EffectDef::UnlessPaid { .. }
            | EffectDef::May(_)
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CreateEmblem { .. }
            | EffectDef::Transform { .. }
            | EffectDef::AdditionalCombatPhase
            | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::GainControlThisTurn { .. }
            | EffectDef::AtNextStep { .. }
            | EffectDef::IfCondition { .. }
            | EffectDef::TriggerUntilYourNextTurn { .. }
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::PlayersCantPlay(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::Replacement(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::ChooseCardName { .. }
            | EffectDef::ChoosePlayer { .. }
            | EffectDef::CopyPermanentAsItEnters { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => {}
        }

        activations
    }

    /// One activation per colour Fellwar Stone can currently produce. The
    /// set is read off the battlefield, so it has to be recomputed rather
    /// than frozen on the ability.
    pub(super) fn fellwar_stone_activations(
        &self,
        permanent: &Permanent,
        ability: AbilityOrigin,
        costs: crate::card::AbilityCostList,
    ) -> Vec<ManaAbilityActivation> {
        let mut visiting = Vec::new();
        self.fellwar_stone_colors(permanent, &mut visiting)
            .into_iter()
            .map(|color| ManaAbilityActivation {
                source: permanent.card.id,
                ability,
                color,
                costs,
                effect: AddManaEffectDef::one(color),
            })
            .collect()
    }

    pub(super) fn fellwar_stone_colors(
        &self,
        permanent: &Permanent,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        if visiting.contains(&permanent.card.id) {
            return Vec::new();
        }
        visiting.push(permanent.card.id);
        let mut colors = self
            .battlefield
            .iter()
            .filter(|candidate| {
                candidate.controller == permanent.controller.opponent()
                    && self
                        .permanent_types(candidate)
                        .is_some_and(|types| types.contains(CardType::Land))
            })
            .flat_map(|candidate| self.colors_permanent_could_produce(candidate, visiting))
            .filter(|color| *color != ManaColor::Colorless)
            .collect::<Vec<_>>();
        visiting.pop();
        colors.sort_unstable();
        colors.dedup();
        colors
    }

    pub(super) fn colors_permanent_could_produce(
        &self,
        permanent: &Permanent,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        if visiting.contains(&permanent.card.id) {
            return Vec::new();
        }
        let mut colors = Vec::new();
        self.for_each_effective_ability(permanent, |effective| {
            if !effective.ability.is_executable() {
                return;
            }
            let DeclarativeAbilityDef::ActivatedMana(definition) = effective.ability.definition
            else {
                return;
            };
            match effective.ability.effect.definition {
                EffectDef::AddMana(effect)
                    if definition.procedure == AbilityProcedureDef::Shared
                        && effective.ability.declarative_effect().is_some() =>
                {
                    match effect.mana {
                        ManaSelectionDef::One(kind) => colors.push(kind),
                        ManaSelectionDef::Choice(kinds) => {
                            colors.extend_from_slice(kinds);
                        }
                    }
                }
                EffectDef::Special(_)
                    if definition.procedure == AbilityProcedureDef::Legacy
                        && effective.ability.custom_behavior()
                            == Some(CardBehavior::FellwarStone) =>
                {
                    colors.extend(self.fellwar_stone_colors(permanent, visiting));
                }
                EffectDef::AddMana(_)
                | EffectDef::AddManaEqualTo { .. }
                | EffectDef::None
                | EffectDef::Sequence(_)
                | EffectDef::DealDamage { .. }
                | EffectDef::DrainLife { .. }
                | EffectDef::GainLife { .. }
                | EffectDef::DrawCards { .. }
                | EffectDef::Discard { .. }
                | EffectDef::ShuffleLibrary { .. }
                | EffectDef::LoseLife { .. }
                | EffectDef::Tap { .. }
                | EffectDef::Untap { .. }
                | EffectDef::PreventCombatDamageThisTurn { .. }
                | EffectDef::Attach { .. }
                | EffectDef::CreateToken { .. }
                | EffectDef::Destroy { .. }
                | EffectDef::Sacrifice { .. }
                | EffectDef::SacrificeOfChoice { .. }
                | EffectDef::DestroyOfChoice { .. }
                | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
                | EffectDef::RevealAndSplitIntoPiles { .. }
                | EffectDef::LoseTheGame { .. }
                | EffectDef::Mill { .. }
                | EffectDef::LookAtTopAndMayTake { .. }
                | EffectDef::LookAtHand { .. }
                | EffectDef::SearchLibrary { .. }
                | EffectDef::Counter { .. }
                | EffectDef::CounterUnlessPaid { .. }
                | EffectDef::AddCounters { .. }
                | EffectDef::ChangeTextBasicLandType { .. }
                | EffectDef::BecomeCopyOf { .. }
                | EffectDef::OptionalManaPayment { .. }
                | EffectDef::UnlessPaid { .. }
                | EffectDef::May(_)
                | EffectDef::CannotBeForcedToSacrifice
                | EffectDef::CreateEmblem { .. }
                | EffectDef::Transform { .. }
                | EffectDef::AdditionalCombatPhase
                | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
                | EffectDef::GrantFlashToNextSorcery
                | EffectDef::ExileLinkedToSource { .. }
                | EffectDef::ReturnLinkedExiles { .. }
                | EffectDef::MakeUnblockableThisTurn { .. }
                | EffectDef::GainControlThisTurn { .. }
                | EffectDef::AtNextStep { .. }
                | EffectDef::IfCondition { .. }
                | EffectDef::TriggerUntilYourNextTurn { .. }
                | EffectDef::ReduceGenericCostBy(_)
                | EffectDef::PlayersCantPlay(_)
                | EffectDef::MultiplyEventAmount(_)
                | EffectDef::Replacement(_)
                | EffectDef::MoveToZone { .. }
                | EffectDef::ChooseCardName { .. }
                | EffectDef::ChoosePlayer { .. }
                | EffectDef::CopyPermanentAsItEnters { .. }
                | EffectDef::ChooseCreatureType { .. }
                | EffectDef::Apply { .. }
                | EffectDef::Special(_) => {}
            }
        });
        colors.sort_unstable();
        colors.dedup();
        colors
    }

    pub(super) fn mana_ability_activation(
        &self,
        permanent: &Permanent,
        ability: AbilityOrigin,
        color: ManaColor,
    ) -> Option<ManaAbilityActivation> {
        self.mana_ability_activations(permanent)
            .into_iter()
            .find(|activation| activation.ability == ability && activation.color == color)
    }

    pub(super) fn mana_production(activation: ManaAbilityActivation) -> ManaPool {
        let mut pool = ManaPool::default();
        pool.add_color(activation.color, activation.effect.amount);
        pool
    }

    pub(super) fn mana_for_activation(activation: ManaAbilityActivation) -> Vec<Mana> {
        let mana = Mana::from_ability(
            activation.color,
            ManaSource {
                object: activation.source,
                ability: activation.ability,
            },
            activation.effect.restrictions,
            activation.effect.spend_effects,
        );
        vec![mana; usize::from(activation.effect.amount)]
    }

    pub(super) fn add_mana(&mut self, player: PlayerId, mana: impl IntoIterator<Item = Mana>) {
        for mana in mana {
            self.players[player.index()]
                .mana_pool
                .add_color(mana.color, 1);
            self.players[player.index()].mana.push(mana);
        }
    }

    pub(super) fn add_unrestricted_mana(
        &mut self,
        player: PlayerId,
        color: ManaColor,
        amount: u16,
    ) {
        self.add_mana(
            player,
            std::iter::repeat_n(Mana::unrestricted(color), usize::from(amount)),
        );
    }

    pub(super) fn payment_object(
        &self,
        purpose: &ManaPaymentPurpose,
    ) -> Option<(TriggerEventObject, bool)> {
        match purpose {
            ManaPaymentPurpose::Spell {
                object,
                definition,
                controller,
                form,
            } => self
                .printed_trigger_event_object(
                    *object,
                    *definition,
                    *controller,
                    &CharacteristicContext::Stack { form: form.clone() },
                )
                .map(|object| (object, true)),
            ManaPaymentPurpose::Ability { source, .. } => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *source)
                .map(|permanent| (self.trigger_event_object(permanent), false))
                .or_else(|| match self.retired_objects.get(source) {
                    Some(RetiredObject::Permanent { permanent, .. }) => {
                        Some((self.trigger_event_object(permanent), false))
                    }
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
                })
                .or_else(|| {
                    let (zone, card) = self.card_in_nonbattlefield_zone(*source)?;
                    let context = match zone {
                        ZoneKind::Library => CharacteristicContext::Library,
                        ZoneKind::Hand => CharacteristicContext::Hand,
                        ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                        ZoneKind::Exile => CharacteristicContext::Exile,
                        ZoneKind::Command => CharacteristicContext::Command,
                        ZoneKind::Battlefield | ZoneKind::Stack => return None,
                    };
                    self.printed_trigger_event_object(
                        card.id,
                        card.definition,
                        card.owner,
                        &context,
                    )
                    .map(|object| (object, false))
                }),
            ManaPaymentPurpose::Other => None,
        }
    }

    pub(super) fn chosen_creature_type_for_mana_source(
        &self,
        source: GameObjectId,
    ) -> Option<&str> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| permanent.chosen_creature_type.as_deref())
            .or_else(|| match self.retired_objects.get(&source) {
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    permanent.chosen_creature_type.as_deref()
                }
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// The colour a spell's printed cost demands for X, if it prints such a
    /// restriction. Read from the compatibility primary-part view: no
    /// multi-part card prints one today.
    pub(super) fn x_spend_restriction(&self, purpose: &ManaPaymentPurpose) -> Option<ManaColor> {
        let ManaPaymentPurpose::Spell { definition, .. } = purpose else {
            return None;
        };
        self.catalog.get(*definition)?.rules.x_spend_restriction()
    }

    pub(super) fn restrict_x(
        &self,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> (ManaCost, u16) {
        self.x_spend_restriction(purpose)
            .map_or((cost, x), |color| fold_restricted_x(cost, x, color))
    }

    pub(super) fn mana_can_pay_for(&self, mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.restrictions
            .iter()
            .all(|restriction| match restriction {
                ManaRestrictionDef::CastSpell(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, true)
                    }),
                ManaRestrictionDef::CastCreatureSpellOfChosenType => {
                    let Some(source) = mana.source else {
                        return false;
                    };
                    let Some(chosen) = self.chosen_creature_type_for_mana_source(source.object)
                    else {
                        return false;
                    };
                    self.payment_object(purpose)
                        .is_some_and(|(object, is_spell)| {
                            is_spell
                                && object.types.contains(CardType::Creature)
                                && object.subtypes.contains(&chosen)
                        })
                }
                ManaRestrictionDef::ActivateAbility(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        !is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, false)
                    }),
                ManaRestrictionDef::Special(_) => false,
            })
    }

    pub(super) fn mana_has_spend_effect_for(mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.spend_effects.iter().any(|effect| {
            matches!(
                (purpose, effect),
                (
                    ManaPaymentPurpose::Spell { .. },
                    ManaSpendEffectDef::ApplyToPaidSpell(_)
                ) | (
                    ManaPaymentPurpose::Ability { .. },
                    ManaSpendEffectDef::ApplyToPaidAbility(_)
                )
            )
        })
    }

    pub(super) fn eligible_mana_pool(
        &self,
        player: PlayerId,
        purpose: &ManaPaymentPurpose,
    ) -> ManaPool {
        let aggregate = self.players[player.index()].mana_pool;
        let mut eligible = ManaPool::default();
        let mut tracked = ManaPool::default();
        for mana in &self.players[player.index()].mana {
            if tracked.amount(mana.color) >= aggregate.amount(mana.color) {
                continue;
            }
            tracked.add_color(mana.color, 1);
            if self.mana_can_pay_for(*mana, purpose) {
                eligible.add_color(mana.color, 1);
            }
        }
        // Compatibility callers and tests may still write aggregate pools
        // directly. Any units without per-mana records are unrestricted.
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            eligible.add_color(
                color,
                aggregate
                    .amount(color)
                    .saturating_sub(tracked.amount(color)),
            );
        }
        eligible
    }

    /// Turns life into the {C} this payment is short, one point at a time,
    /// so the ordinary payment below finds a pool that can cover the cost.
    pub(super) fn channel_for_shortfall(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) {
        let available = self.channel_mana_available(player);
        if available == 0 {
            return;
        }
        let pool = self.eligible_mana_pool(player, purpose);
        let needed = Self::generic_shortfall(pool, cost, x).min(available);
        for _ in 0..needed {
            self.players[player.index()].life -= 1;
            self.add_unrestricted_mana(player, ManaColor::Colorless, 1);
        }
    }

    pub(super) fn pay_player_cost_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<Mana> {
        let (cost, x) = self.restrict_x(cost, x, purpose);
        self.reconcile_mana(player);
        self.channel_for_shortfall(player, cost, x, purpose);
        let before = self.eligible_mana_pool(player, purpose);
        let mut after = before;
        let has_eligible_spend_effect = |color| {
            self.players[player.index()].mana.iter().any(|mana| {
                mana.color == color
                    && self.mana_can_pay_for(*mana, purpose)
                    && Self::mana_has_spend_effect_for(*mana, purpose)
            })
        };
        // A hybrid symbol prefers whichever of its colours carries a rider
        // this payment can use.
        let hybrid_preference = |color: ManaColor| !has_eligible_spend_effect(color);
        let mut generic_order = [
            ManaColor::Colorless,
            ManaColor::Green,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::White,
            ManaColor::Blue,
        ];
        generic_order.sort_by_key(|color| !has_eligible_spend_effect(*color));
        pay_cost_with_orders(&mut after, cost, x, &hybrid_preference, &generic_order);
        let mut spent = Vec::new();
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            let count = before.amount(color).saturating_sub(after.amount(color));
            for _ in 0..count {
                let index = self.players[player.index()]
                    .mana
                    .iter()
                    .enumerate()
                    .filter(|(_, mana)| {
                        mana.color == color && self.mana_can_pay_for(**mana, purpose)
                    })
                    .max_by_key(|(_, mana)| {
                        (
                            Self::mana_has_spend_effect_for(**mana, purpose),
                            !mana.restrictions.is_empty(),
                        )
                    })
                    .map(|(index, _)| index);
                if let Some(index) = index {
                    spent.push(self.players[player.index()].mana.remove(index));
                }
                self.players[player.index()]
                    .mana_pool
                    .remove_color(color, 1);
            }
        }
        spent
    }

    pub(super) fn pay_player_cost(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
    ) -> Vec<Mana> {
        self.pay_player_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }

    pub(super) fn apply_spent_mana_to_spell(object: &mut StackObject, spent: &[Mana]) {
        for mana in spent {
            for spend_effect in mana.spend_effects {
                if let ManaSpendEffectDef::ApplyToPaidSpell(effect) = *spend_effect {
                    object.applied_effects.push(AppliedStackEffect {
                        source: mana.source,
                        effect,
                    });
                }
            }
        }
    }

    /// Tests and compatibility callers can still construct aggregate pools
    /// directly. Trim any now-impossible annotations before authoritative
    /// payment so that those writes cannot leave stale spend riders behind.
    pub(super) fn reconcile_mana(&mut self, player: PlayerId) {
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            let allowed = usize::from(self.players[player.index()].mana_pool.amount(color));
            let mut retained = 0;
            self.players[player.index()].mana.retain(|mana| {
                if mana.color != color {
                    true
                } else if retained < allowed {
                    retained += 1;
                    true
                } else {
                    false
                }
            });
        }
    }

    pub(super) fn can_pay_cost(&self, player: PlayerId, cost: ManaCost, x: u16) -> bool {
        self.can_pay_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }

    pub(super) fn can_pay_cost_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> bool {
        self.assigned_mana_activations_for(player, cost, x, purpose)
            .is_some()
    }

    pub(super) fn add_mana_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
        {
            actions.extend(self.mana_ability_activations(permanent).into_iter().map(
                |activation| Action::ActivateManaAbility {
                    source: activation.source,
                    ability: activation.ability,
                    color: activation.color,
                },
            ));
        }
    }
}
