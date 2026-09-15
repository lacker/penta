use super::{
    AbilityOrigin, AbilitySourceRef, AlternativeCastKindDef, AppliedEffectDef, AppliedStackEffect,
    BTreeMap, BattlefieldExitCompletion, CREATURE_TYPES, CardDefinition, CardInstance, CardType,
    CastChoices, CastContext, CastOfferCost, CastSignature, CastSourceZone, CharacteristicContext,
    CommittedStackObjectEvent, CommittedTriggerEvent, CostConfiguration, CostDef,
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    DeclarativeAbilityDef, EntryCompletion, Game, GameEvent, GameObjectId, Mana,
    ManaAbilityActivation, ManaActivationChoices, ManaColor, ManaCost, ManaPaymentPurpose,
    PendingBattlefieldEntry, Permanent, PlayActionKind, PlayOptionDef, PlayOptionId, PlayerId,
    StackObject, StackObjectKind, Target, ZoneKind, ZoneMoveCause, ZonePlacement, remove_card,
};
mod signature_validation;
include!("casting/life_costs.rs");
include!("casting/entry_copy_choice.rs");
include!("casting/mana_activation.rs");
include!("casting/object_costs.rs");
include!("casting/scalar_choices.rs");

use crate::card::{
    BattlefieldEntryScalarChoiceDef, FaceDownCharacteristics, ReplacementEffectDef,
    ScalarChoiceListDef,
};

struct SpellCastProposal {
    card: CardInstance,
    signature: CastSignature,
    source_zone: CastSourceZone,
    alternative: Option<AlternativeCastKindDef>,
    cast_via_flashback: bool,
    exile_if_put_into_graveyard: bool,
    face_down: Option<FaceDownCharacteristics>,
}

impl Game {
    pub(in crate::game) fn play_land_using_permission(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        option_id: PlayOptionId,
        selected_permission: Option<GameObjectId>,
    ) {
        let (from, card) = self
            .card_in_nonbattlefield_zone(card_id)
            .expect("legal land action references a card in a playable zone");
        let owner = card.owner;
        let definition_id = card.definition;
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
        let selected = if selected_permission.is_some() {
            self.selected_play_permission(
                card,
                player,
                option,
                0,
                &CostConfiguration::default().with_permission_source(selected_permission),
            )
        } else {
            None
        };
        let source_zone = match from {
            ZoneKind::Graveyard => &mut self.players[player.index()].graveyard,
            ZoneKind::Library => &mut self.players[player.index()].library,
            ZoneKind::Exile => &mut self.players[owner.index()].exile,
            _ => &mut self.players[player.index()].hand,
        };
        let card = remove_card(source_zone, card_id)
            .expect("legal land action references a card in a playable zone");
        self.players[player.index()].lands_played_this_turn = self.players[player.index()]
            .lands_played_this_turn
            .saturating_add(1);
        self.consecutive_passes = 0;
        let mut permanent = Permanent::entering(
            card,
            presented,
            player,
            self.turns_started[player.index()],
            self.turn,
        );
        // A land played out of a graveyard is played under somebody's
        // permission, which may allow only so many and may hand the land
        // something as it arrives. A land played from hand is nobody's
        // business but the land-drop count above.
        if from == ZoneKind::Graveyard && selected.is_none() {
            let option = option.clone();
            self.spend_graveyard_land_permission(&mut permanent, player, &option);
        }
        // A land played out of exile may be answering a standing "you may
        // play the exiled card" offer, which playing it takes rather than
        // declines -- the same way casting one does.
        if from == ZoneKind::Exile {
            self.take_answered_cast_offer(card_id);
            self.consume_exile_play_permission(card_id);
        }
        if let Some((source, permission)) = selected {
            if let Some(effect) = permission.grants {
                self.grant_resolved_ability_to_entering_permanent(&mut permanent, source, *effect);
            }
            self.spend_play_permission(source, permission);
        }
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from,
            completion: EntryCompletion::LandPlayed { player },
            redirected_to: None,
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

    pub(super) fn queue_entry_scalar_choice(
        &mut self,
        player: PlayerId,
        context: super::ReplacementEffectContext,
        authored_effect: ReplacementEffectDef,
        choice: BattlefieldEntryScalarChoiceDef,
    ) {
        let (prompt, choices) = self.entry_scalar_choices(player, choice);
        let options = choices
            .iter()
            .enumerate()
            .map(|(index, value)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: value.clone(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            prompt,
            DecisionVisibility::PublicNotice,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryScalarChoice {
                context,
                authored_effect,
                choice,
                choices,
            },
        );
    }

    pub(super) fn queue_entry_basic_land_type_pair_choice(
        &mut self,
        player: PlayerId,
        context: super::ReplacementEffectContext,
    ) {
        self.queue_decision(
            player,
            "Choose two different basic land types",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            Self::basic_land_type_pair_options(),
            DecisionContinuation::BattlefieldEntryBasicLandTypePairChoice { context },
        );
    }

    pub(super) fn activate_mana_source(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        color: ManaColor,
        choices: &ManaActivationChoices,
    ) {
        let mut activation = self
            .concrete_mana_activation(player, source, ability, color, choices)
            .expect("legal mana action references a mana source");
        if self.payment_probe.is_some() {
            let cost = activation
                .costs
                .iter()
                .filter_map(|cost| match cost {
                    CostDef::Mana(mana) => Some(*mana),
                    _ => None,
                })
                .reduce(super::add_mana_cost)
                .unwrap_or_default();
            let chosen = activation.cost_object.into_iter().collect::<Vec<_>>();
            let reserved = Self::activation_payment_reservations(
                source,
                ability,
                &activation.costs,
                &chosen,
                0,
            );
            if self.capture_payment_probe(
                player,
                cost,
                0,
                &super::payment::mana_ability_payment_purpose(source, &activation.costs),
                reserved,
                activation
                    .costs
                    .iter()
                    .any(|cost| matches!(cost, CostDef::Mana(_))),
            ) {
                return;
            }
        }
        self.run_explicit_funding(player);
        self.price_explicit_mana_production(player, &mut activation);
        let produced_mana = Self::mana_for_activation(&activation);
        // Mana and stack-using abilities share per-turn and per-object history.
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        {
            permanent.record_activation(ability);
        }
        self.pay_immediate_mana_activation_costs(player, source, &activation);
        if self.pay_moving_mana_activation_costs(player, source, &activation, &produced_mana) {
            return;
        }
        self.complete_mana_ability(player, &activation, produced_mana);
    }

    /// Whether this player could cast a sorcery right now (CR 307.1): their
    /// own main phase, with an empty stack.
    pub(super) fn sorcery_speed_available(&self, player: PlayerId) -> bool {
        player == self.active_player && self.step.is_main() && self.stack.is_empty()
    }

    fn cast_alternative_kind(
        &self,
        card_id: GameObjectId,
        signature: &CastSignature,
        offer: Option<CastOfferCost>,
    ) -> Option<AlternativeCastKindDef> {
        self.card_in_nonbattlefield_zone(card_id)
            .map(|(_, card)| card)
            .and_then(|card| self.catalog.get(card.definition))
            .and_then(|definition| {
                definition
                    .play_option(signature.play_option())
                    .map(|option| (definition, option))
            })
            .and_then(|(definition, option)| {
                self.selected_alternative_kind_for_offer(
                    definition,
                    option,
                    card_id,
                    signature.costs(),
                    offer,
                )
            })
    }

    fn cast_exiles_if_put_into_graveyard(
        &self,
        card_id: GameObjectId,
        signature: &CastSignature,
        offer: Option<CastOfferCost>,
    ) -> bool {
        self.card_in_nonbattlefield_zone(card_id)
            .map(|(_, card)| card)
            .and_then(|card| self.catalog.get(card.definition))
            .and_then(|definition| {
                definition
                    .play_option(signature.play_option())
                    .map(|option| (definition, option))
            })
            .and_then(|(definition, option)| {
                self.selected_alternative_ability_for_offer(
                    definition,
                    option,
                    card_id,
                    signature.costs(),
                    offer,
                )
            })
            .is_some_and(|alternative| alternative.exile_if_put_into_graveyard)
    }

    pub(super) fn complete_mana_ability(
        &mut self,
        player: PlayerId,
        activation: &super::ManaAbilityActivation,
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
        // "If there are no mining counters on this land, sacrifice it."
        // Checked here because a mana ability resolves without the stack:
        // the land is gone by the time anyone could respond, and a counter
        // removed by anything other than this ability leaves it alone.
        if let Some(kind) = activation.effect.sacrifice_source_when_out_of
            && self.battlefield.iter().any(|permanent| {
                permanent.card.id == activation.source && permanent.counters(kind) == 0
            })
        {
            self.move_permanents_to_graveyard(&[activation.source]);
        }
        self.consecutive_passes = 0;
        self.events.push(GameEvent::ManaAdded {
            player,
            source: activation.source,
        });
    }

    /// "You may reveal this card from your hand": the spliced cards stay
    /// where they are and everybody sees what was added to the spell.
    fn reveal_spliced_cards(&mut self, player: PlayerId, spliced: &[GameObjectId]) {
        let revealed: Vec<_> = spliced
            .iter()
            .filter_map(|spliced| {
                self.players[player.index()]
                    .hand
                    .iter()
                    .find(|candidate| candidate.id == *spliced)
                    .map(|card| (card.id, card.definition))
            })
            .collect();
        for (card, definition) in revealed {
            self.events.push(GameEvent::CardRevealed {
                player,
                card,
                definition,
            });
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn cast_spell(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
        sacrifices: &[GameObjectId],
    ) {
        let (signature, cost, source_zone) = self
            .validated_cast_signature(player, card_id, choices, sacrifices)
            .expect("validated casting choices remain valid while paying costs");
        self.reveal_spliced_cards(player, choices.spliced());
        let phyrexian_life = Self::mana_payment_life(choices.mana_payment())
            .expect("validated casting choices carry a valid flexible-mana payment");
        let phyrexian_life_symbols = Self::phyrexian_symbols_paid_with_life(choices.mana_payment())
            .expect("validated casting choices carry a valid flexible-mana payment");
        // Keep the standing offer through validation; consume it before costs or movement.
        let targets = signature.iter_targets().copied().collect::<Vec<_>>();
        let x = signature.x();
        let offer = self
            .current_cast_offer(player, card_id, source_zone)
            .map(|offer| offer.cost);
        let (object_payments, cast_life, includes_mana_payment) = self
            .cast_object_payments_and_life(
                player,
                card_id,
                &signature,
                super::CastCostContext { source_zone, offer },
                sacrifices,
            );
        let alternative_kind = self.cast_alternative_kind(card_id, &signature, offer);
        let selected = if signature.costs().permission_source().is_some() {
            let card = self
                .card_in_nonbattlefield_zone(card_id)
                .expect("cast card in a zone")
                .1;
            let option = self
                .catalog
                .get(card.definition)
                .and_then(|def| def.play_option(signature.play_option()))
                .expect("validated play option");
            self.selected_play_permission(card, player, option, signature.x(), signature.costs())
        } else {
            None
        };
        let exile_if_put_into_graveyard =
            self.cast_exiles_if_put_into_graveyard(card_id, &signature, offer);
        let (granted_by_permission, cast_via_suspend) =
            self.spend_cast_permissions(player, card_id, &signature, source_zone, alternative_kind);
        self.take_answered_cast_offer(card_id);
        let cast_via_flashback = alternative_kind == Some(AlternativeCastKindDef::Flashback);
        let face_down = alternative_kind.and_then(AlternativeCastKindDef::face_down);
        let energy = self.exile_energy_cost(card_id, player).unwrap_or(0);
        // Read before moving the card out of the zone its permission names.
        let permission_life = if signature.costs().permission_source().is_some() {
            self.permission_life_for_cast(player, card_id, choices)
        } else {
            0
        };
        // Read before the signature reaches the stack object, then pay below.
        let opponent_life_gain = self.cast_opponent_life_gain(player, card_id, &signature);
        let commander_owner = self.commander_owner(card_id);
        let card = self.remove_card_for_cast(player, card_id, source_zone);
        let mut stack_object = self.propose_spell_on_stack(
            player,
            SpellCastProposal {
                card,
                signature,
                source_zone,
                alternative: alternative_kind,
                cast_via_flashback,
                exile_if_put_into_graveyard,
                face_down,
            },
        );
        Self::record_cast_context(
            &mut stack_object,
            cast_via_suspend,
            phyrexian_life_symbols,
            granted_by_permission,
        );
        if let Some((source, benefit)) = selected {
            if let Some(benefit) = benefit.benefit
                && self
                    .stack_spell_types(&stack_object)
                    .is_some_and(|types| types.contains(CardType::Creature))
            {
                stack_object
                    .cast
                    .as_mut()
                    .expect("cast context")
                    .permission_entry_counters = benefit.creature_entry_counters.to_vec();
            }
            self.spend_play_permission(source, benefit);
        }
        let stack_id = stack_object.id;
        let definition = stack_object
            .card
            .definition
            .card_definition()
            .expect("a cast spell is backed by a card definition");
        let life = cast_life
            .saturating_add(permission_life)
            .saturating_add(phyrexian_life);
        let payment_purpose = ManaPaymentPurpose::Spell {
            object: stack_id,
            commander_owner,
            definition,
            controller: player,
            form: stack_object
                .signature
                .as_ref()
                .expect("a spell has a cast signature")
                .form()
                .clone(),
            alternative: alternative_kind,
            x,
            reserved_life_payment: life,
        };
        self.pay_cast_life_and_energy(player, life, opponent_life_gain, energy);
        if self.payment_probe.is_some() {
            let reserved = object_payments
                .iter()
                .map(|(object, cost)| match cost {
                    CostDef::Tap { .. } => {
                        super::payment::resources::PaymentReservation::Untapped(*object)
                    }
                    _ => super::payment::resources::PaymentReservation::Object(*object),
                })
                .collect();
            if self.capture_payment_probe(
                player,
                cost,
                x,
                &payment_purpose,
                reserved,
                includes_mana_payment,
            ) {
                return;
            }
        }
        let plan = self.cast_mana_plan(
            player,
            cost,
            x,
            &payment_purpose,
            super::mana_planning::ManaPaymentReservations::with_object_costs(
                sacrifices,
                &object_payments,
            ),
            includes_mana_payment,
        );
        self.continue_spell_mana_payment(
            stack_object,
            targets,
            object_payments,
            cost,
            x,
            payment_purpose,
            plan,
            0,
        );
    }

    fn remove_card_for_cast(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        source_zone: CastSourceZone,
    ) -> CardInstance {
        if let Some((zone, card)) = self
            .card_in_nonbattlefield_zone(card_id)
            .map(|(zone, card)| (zone, card.clone()))
        {
            self.remember_card_characteristics(&card, Some(zone));
        }
        match source_zone {
            CastSourceZone::Command => {
                remove_card(&mut self.players[player.index()].command, card_id)
            }
            CastSourceZone::Hand => remove_card(&mut self.players[player.index()].hand, card_id),
            CastSourceZone::Graveyard => {
                // Cast out of a graveyard is a card leaving it, which the
                // clauses that ask about the turn have to see.
                let owner = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .find(|owner| {
                        self.players[owner.index()]
                            .graveyard
                            .iter()
                            .any(|card| card.id == card_id)
                    })
                    .expect("a validated graveyard cast still has an owner");
                self.note_card_left_graveyard(owner);
                remove_card(&mut self.players[owner.index()].graveyard, card_id)
            }
            CastSourceZone::Exile => {
                self.consume_exile_play_permission(card_id);
                // The card is in its owner's exile, which need not be the
                // exile of the player casting it.
                remove_card(&mut self.players[0].exile, card_id)
                    .or_else(|| remove_card(&mut self.players[1].exile, card_id))
            }
            CastSourceZone::Library => {
                remove_card(&mut self.players[player.index()].library, card_id)
            }
        }
        .expect("legal cast action references a card in its validated source zone")
    }

    fn propose_spell_on_stack(
        &mut self,
        player: PlayerId,
        proposal: SpellCastProposal,
    ) -> StackObject {
        let SpellCastProposal {
            card,
            signature,
            source_zone,
            alternative,
            cast_via_flashback,
            exile_if_put_into_graveyard,
            face_down,
        } = proposal;
        let timing_option = self
            .catalog
            .get(card.definition)
            .and_then(|definition| definition.play_option(signature.play_option()))
            .cloned();
        // A spell is first proposed on the stack, then mana abilities may be
        // activated and costs are paid. The operation cannot fail after the
        // validated signature above, so keeping the provisional object local
        // gives mana spend riders a concrete destination without exposing a
        // half-paid spell to priority or trigger placement.
        let (card, _zone_change) = self.zone_change_card(card);
        let id = card.id;
        let frozen_spell_ability = self.frozen_spell_payload(card.definition, &signature);
        // Read now, because nothing afterwards can tell: by resolution the
        // step has usually moved on, and the stack is empty again.
        let cast_at_instant_speed = !self.sorcery_speed_available(player);
        let option = timing_option
            .as_ref()
            .expect("a validated cast signature names a play option");
        let mut cast = CastContext::for_cast(
            source_zone,
            alternative,
            cast_at_instant_speed,
            option,
            &signature,
            cast_via_flashback,
            exile_if_put_into_graveyard,
        );
        cast.caster = Some(player);
        cast.player_bindings =
            self.selected_cast_player_bindings(card.definition, &signature, player);
        StackObject {
            id,
            kind: StackObjectKind::Spell,
            card: card.into(),
            source: None,
            ability: frozen_spell_ability,
            controller: player,
            signature: Some(signature),
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            resolved_continuous_effects: Vec::new(),
            last_known_colors: None,
            colors: None,
            cast: Some(cast),
            face_down,
            is_copy: false,
        }
    }

    fn cast_mana_plan(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        payment_purpose: &ManaPaymentPurpose,
        sacrifices: super::mana_planning::ManaPaymentReservations<'_>,
        includes_mana_payment: bool,
    ) -> Vec<super::PlannedManaActivation> {
        // CR 601.2g: omitting a mana payment never opens a mana-ability
        // window. An explicit {0}, including a reduced mana cost, still does.
        self.run_explicit_funding(player);
        if let Some(bound) = &self.explicit_cast_contributions {
            return bound.plan.clone();
        }
        if includes_mana_payment && self.explicit_mana_payment.is_none() {
            let Some(plan) = self.plan_mana_activations_for_reserving(
                player,
                cost,
                x,
                None,
                payment_purpose,
                sacrifices,
            ) else {
                panic!(
                    "{}",
                    self.unplannable_payment(player, cost, x, None, payment_purpose)
                );
            };
            plan
        } else {
            Vec::new()
        }
    }

    /// Completes the mana-payment portion of a cast. A mana ability may pay
    /// by sacrificing a permanent whose exit needs a CR 616 replacement
    /// choice. In that case the spell and the rest of its frozen payment plan
    /// remain on the prospective exit batch until the ability has completed.
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_lines)]
    pub(super) fn continue_spell_mana_payment(
        &mut self,
        mut stack_object: StackObject,
        targets: Vec<Target>,
        object_payments: Vec<(GameObjectId, CostDef)>,
        cost: ManaCost,
        x: u16,
        purpose: ManaPaymentPurpose,
        plan: Vec<super::PlannedManaActivation>,
        mut next_activation: usize,
    ) {
        while let Some(payment) = plan.get(next_activation).cloned() {
            next_activation += 1;
            let super::PlannedPaymentKind::Mana {
                ability,
                color,
                counters_removed,
                cost_object,
                combination,
                triggered_mana,
                ..
            } = payment.kind
            else {
                continue;
            };
            let pending_before = self.pending_decisions.len();
            self.activate_mana_source(
                stack_object.controller,
                payment.source,
                ability,
                color,
                &ManaActivationChoices::new(
                    counters_removed,
                    cost_object,
                    combination,
                    triggered_mana,
                ),
            );
            let suspended = self.pending_decisions[pending_before..]
                .iter()
                .any(|pending| {
                    matches!(
                        &pending.continuation,
                        DecisionContinuation::BattlefieldExitReplacement { .. }
                    )
                });
            if suspended {
                let deferred = self.defer_after_battlefield_exit(
                    pending_before,
                    BattlefieldExitCompletion::ContinueSpellManaPayment {
                        object: Box::new(stack_object),
                        targets,
                        object_payments,
                        cost,
                        x,
                        purpose,
                        plan,
                        next_activation,
                    },
                );
                debug_assert!(deferred, "the observed exit choice retains the cast");
                return;
            }
        }

        // CR 601.2g completes every mana ability before 601.2h spends the
        // objects chosen for direct contributions.
        for payment in &plan {
            if payment
                .kind
                .contribution()
                .is_some_and(super::ManaContributionKind::taps_source)
            {
                self.tap_permanent(payment.source)
                    .expect("a planned contribution source remains on the battlefield");
            }
        }
        let exiled = plan
            .iter()
            .filter(|payment| {
                payment
                    .kind
                    .contribution()
                    .is_some_and(super::ManaContributionKind::exiles_source)
            })
            .map(|payment| payment.source)
            .collect::<Vec<_>>();
        let exiled = self.exile_graveyard_cards(stack_object.controller, &exiled);
        stack_object
            .cast
            .as_mut()
            .expect("a cast spell retains its context through payment")
            .exiled_payment_cards
            .extend(exiled);
        let (mana_cost, mana_x) = if let Some(bound) = self.explicit_cast_contributions.take() {
            (bound.remaining.cost, bound.remaining.x)
        } else {
            self.residual_cost_after_contributions(cost, x, &purpose, &plan, true)
        };
        // The spell's nonmana life bill was paid before this continuation
        // began. Do not reserve it a second time when repeatable life mana
        // supplies the final shortfall after the planned abilities resolve.
        let payment_purpose = match &purpose {
            ManaPaymentPurpose::Spell {
                object,
                definition,
                controller,
                commander_owner,
                form,
                alternative,
                x: chosen_x,
                ..
            } => ManaPaymentPurpose::Spell {
                object: *object,
                commander_owner: *commander_owner,
                definition: *definition,
                controller: *controller,
                form: form.clone(),
                alternative: *alternative,
                x: *chosen_x,
                reserved_life_payment: 0,
            },
            ManaPaymentPurpose::Ability { .. }
            | ManaPaymentPurpose::Payment { .. }
            | ManaPaymentPurpose::Other => purpose.clone(),
        };
        let spent_mana =
            self.pay_player_cost_for(stack_object.controller, mana_cost, mana_x, &payment_purpose);
        self.apply_spent_mana_to_spell(&mut stack_object, &spent_mana);
        // Recorded whether or not this spell counts them: what paid for a
        // spell is a fact about the cast, and a clause that asks later has
        // nothing else to read it from.
        let cast_context = stack_object
            .cast
            .as_mut()
            .expect("a cast spell retains its context through payment");
        for mana in &spent_mana {
            if mana.color != ManaColor::Colorless {
                cast_context.colors_of_mana_spent =
                    cast_context.colors_of_mana_spent.with(mana.color);
            }
        }
        self.continue_spell_cast(stack_object, targets, object_payments);
    }

    fn complete_spell_cast(&mut self, mut stack_object: StackObject, targets: Vec<Target>) {
        self.apply_matching_cast_rules(&mut stack_object);
        let face_down = stack_object.face_down.is_some();
        let player = stack_object.controller;
        let stack_id = stack_object.id;
        let cast_from = stack_object
            .cast
            .as_ref()
            .and_then(|cast| cast.source_zone)
            .expect("a cast spell remembers where it was cast from");
        let definition = stack_object
            .card
            .definition
            .card_definition()
            .expect("a cast spell is backed by a card definition");
        let cast_event = self
            .stack_trigger_event_object(&stack_object)
            .expect("a cast spell has locked characteristics");
        self.stack.push(stack_object);
        self.consecutive_passes = 0;
        self.record_spell_cast(player, stack_id);
        // Kept for the targeting triggers below, which run after the cast
        // event has taken the list.
        let crime_targets = targets.clone();
        self.events.push(if face_down {
            GameEvent::FaceDownSpellCast {
                player,
                card: stack_id,
                targets,
            }
        } else {
            GameEvent::SpellCast {
                player,
                card: stack_id,
                definition,
                targets,
            }
        });
        self.capture_battlefield_triggers(&CommittedTriggerEvent::StackObject {
            object: cast_event.clone(),
            kind: StackObjectKind::Spell,
            event: CommittedStackObjectEvent::Cast { from: cast_from },
        });
        self.capture_crime_triggers(player, &crime_targets);
        // Targets are locked in here. Publish one atomic batch so both
        // recipient-local and "one or more" targeting clauses see the right
        // number of events (CR 115.7c).
        self.capture_targeting_triggers(StackObjectKind::Spell, &cast_event, &crime_targets);
        // The spell's own cast clause, which no battlefield listener carries.
        self.capture_own_cast_triggers(stack_id);
    }
}

mod permissions;
