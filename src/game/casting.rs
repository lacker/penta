use super::{
    AbilityCostDef, AbilityOrigin, AbilitySourceRef, AlternativeCastKindDef, AppliedEffectDef,
    AppliedStackEffect, BTreeMap, BattlefieldExitCompletion, CREATURE_TYPES, CardBehavior,
    CardDefinition, CardInstance, CardType, CastChoices, CastOfferCost, CastSignature,
    CastSourceZone, CharacteristicContext, CommittedTriggerEvent, CostConfiguration,
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
    BattlefieldEntryScalarChoiceDef, CardSet, ScalarChoiceListDef, SpellLifeCostDef, SpendModeDef,
};

impl Game {
    pub(super) fn play_land(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        option_id: PlayOptionId,
    ) {
        // A land is ordinarily played from hand; a permission can also offer
        // one out of a graveyard, out of exile, or off the top of a library.
        // The exile is looked for in both players' -- a card somebody else
        // exiled is still played from where it lies.
        let owner = if self.players[player.index()]
            .exile
            .iter()
            .any(|card| card.id == card_id)
        {
            player
        } else {
            player.opponent()
        };
        let state = &self.players[player.index()];
        let from = if state.hand.iter().any(|card| card.id == card_id) {
            ZoneKind::Hand
        } else if state.graveyard.iter().any(|card| card.id == card_id) {
            ZoneKind::Graveyard
        } else if self.players[owner.index()]
            .exile
            .iter()
            .any(|card| card.id == card_id)
        {
            ZoneKind::Exile
        } else {
            ZoneKind::Library
        };
        let definition_id = match from {
            ZoneKind::Graveyard => &state.graveyard,
            ZoneKind::Library => &state.library,
            ZoneKind::Exile => &self.players[owner.index()].exile,
            _ => &state.hand,
        }
        .iter()
        .find(|card| card.id == card_id)
        .map(|card| card.definition)
        .expect("legal land action references a card in a playable zone");
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
        if from == ZoneKind::Graveyard {
            let option = option.clone();
            self.spend_graveyard_land_permission(&mut permanent, player, &option);
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
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryScalarChoice {
                context,
                choice,
                choices,
            },
        );
    }

    pub(super) fn activate_mana_source(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        color: ManaColor,
        choices: ManaActivationChoices,
    ) {
        let activation = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| self.mana_ability_activation(permanent, ability, color, choices))
            .or_else(|| {
                self.hand_mana_ability_activations(player)
                    .into_iter()
                    .chain(self.ongoing_mana_ability_activations(player))
                    .find(|activation| {
                        activation.source == source
                            && activation.ability == ability
                            && activation.color == color
                            && activation.counters_removed == choices.counters_removed
                            && activation.cost_object == choices.cost_object
                            && activation.combination == choices.combination
                    })
            })
            .expect("legal mana action references a mana source");
        let produced_mana = Self::mana_for_activation(&activation);
        // Counted for the same reason an ordinary activation is: a printed
        // "only once each turn" is read off this tally when the ability is
        // next offered.
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        {
            match permanent
                .activations_this_turn
                .iter_mut()
                .find(|(origin, _)| *origin == ability)
            {
                Some((_, count)) => *count = count.saturating_add(1),
                None => permanent.activations_this_turn.push((ability, 1)),
            }
        }
        self.pay_immediate_mana_activation_costs(player, source, &activation);
        if self.pay_moving_mana_activation_costs(player, source, &activation, &produced_mana) {
            return;
        }
        self.complete_mana_ability(player, &activation, produced_mana);
    }

    /// Which alternative cast, if any, the chosen play option and paid costs
    /// amount to. Read while the card is still in the zone it is cast from,
    /// which is why it takes the player rather than the stack object.
    /// How a spell still on the stack was cast, if it was cast some
    /// alternative way. Read off the signature rather than from a permanent,
    /// because a spell that has not resolved has no permanent yet.
    /// Whether this player could cast a sorcery right now (CR 307.1): their
    /// own main phase, with an empty stack.
    pub(super) fn sorcery_speed_available(&self, player: PlayerId) -> bool {
        player == self.active_player && self.step.is_main() && self.stack.is_empty()
    }

    /// Which alternative a spell was cast with, read off the spell object.
    ///
    /// A "when you cast this spell, if it was kicked" trigger asks while the
    /// spell is still on the stack. The spell's own resolution asks after it
    /// has left, so the retired record answers there -- the signature is the
    /// same either way.
    pub(super) fn stack_object_cast_with(
        &self,
        object: GameObjectId,
    ) -> Option<AlternativeCastKindDef> {
        let stack_object = self
            .stack
            .iter()
            .find(|candidate| candidate.id == object)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(super::RetiredObject::Stack(retired)) => Some(retired.as_ref()),
                Some(super::RetiredObject::Card(_) | super::RetiredObject::Permanent { .. })
                | None => None,
            })?;
        let signature = stack_object.signature.as_ref()?;
        let definition = self
            .catalog
            .get(stack_object.card.definition.card_definition()?)?;
        let option = definition.play_option(signature.play_option())?;
        self.selected_alternative_kind(definition, option, object, signature.costs())
    }

    fn cast_alternative_kind(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
        offer: Option<CastOfferCost>,
    ) -> Option<AlternativeCastKindDef> {
        self.players[player.index()]
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
                self.selected_alternative_kind_for_offer(
                    definition,
                    option,
                    card_id,
                    signature.costs(),
                    offer,
                )
            })
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

    pub(super) fn cast_spell(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
        sacrifices: &[GameObjectId],
    ) {
        let (signature, cost, behavior, source_zone) = self
            .validated_cast_signature(player, card_id, choices, sacrifices)
            .expect("validated casting choices remain valid while paying costs");
        self.reveal_spliced_cards(player, choices.spliced());
        let phyrexian_life = Self::mana_payment_life(choices.mana_payment())
            .expect("validated casting choices carry a valid flexible-mana payment");
        let phyrexian_symbols_paid_with_life =
            Self::phyrexian_symbols_paid_with_life(choices.mana_payment())
                .expect("validated casting choices carry a valid flexible-mana payment");
        // A standing cast offer is the permission that made this signature
        // legal, so keep it through validation and consume it atomically
        // before paying costs or moving the card.
        let targets = signature.iter_targets().copied().collect::<Vec<_>>();
        let x = signature.x();
        let offer = self
            .current_cast_offer(player, card_id, source_zone)
            .map(|offer| offer.cost);
        let (object_payments, cast_life) = self.cast_object_payments_and_life(
            player,
            card_id,
            &signature,
            behavior,
            super::CastCostContext { source_zone, offer },
            sacrifices,
        );
        let alternative_kind = self.cast_alternative_kind(player, card_id, &signature, offer);
        let granted_by_permission = self.spend_graveyard_cast_permission(
            player,
            card_id,
            &signature,
            source_zone,
            alternative_kind,
        );
        self.take_answered_cast_offer(card_id);
        // Both exile the card rather than burying it wherever it would otherwise have gone.
        let cast_via_flashback = matches!(
            alternative_kind,
            Some(AlternativeCastKindDef::Flashback | AlternativeCastKindDef::WithoutPayingManaCost)
        );
        let face_down = alternative_kind.and_then(AlternativeCastKindDef::face_down);
        let energy = self.exile_energy_cost(card_id, player).unwrap_or(0);
        // Read while the card is still on the library, which is the only
        // place the permission reaching it can be found.
        let library_top_life = if source_zone == CastSourceZone::LibraryTop {
            self.library_top_life_for_cast(player, card_id, choices)
        } else {
            0
        };
        // Read before the signature is handed to the stack object, and paid
        // below beside every other cost this cast owes.
        let opponent_life_gain = self.cast_opponent_life_gain(card_id, &signature);
        let card = self.remove_card_for_cast(player, card_id, source_zone);
        let mut stack_object = self.propose_spell_on_stack(
            player,
            card,
            signature,
            source_zone,
            cast_via_flashback,
            face_down,
        );
        stack_object.phyrexian_symbols_paid_with_life = phyrexian_symbols_paid_with_life;
        // "If you do, it gains ...": the permission that allowed this cast
        // hands the spell what the permanent it becomes will carry, which
        // rides beside the keyword riders a mana payment can leave.
        Self::attach_permission_grant(&mut stack_object, granted_by_permission);
        let stack_id = stack_object.id;
        let definition = stack_object
            .card
            .definition
            .card_definition()
            .expect("a cast spell is backed by a card definition");
        let life = cast_life
            .saturating_add(library_top_life)
            .saturating_add(phyrexian_life);
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
            reserved_life_payment: life,
        };
        self.pay_cast_life_and_energy(player, life, opponent_life_gain, energy);
        let Some(plan) = self.plan_mana_activations_for_reserving(
            player,
            cost,
            x,
            None,
            &payment_purpose,
            sacrifices,
        ) else {
            panic!(
                "{}",
                self.unplannable_payment(player, cost, x, None, &payment_purpose)
            );
        };
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
        match source_zone {
            CastSourceZone::Hand => remove_card(&mut self.players[player.index()].hand, card_id),
            CastSourceZone::Graveyard => {
                // Cast out of a graveyard is a card leaving it, which the
                // clauses that ask about the turn have to see.
                self.note_card_left_graveyard(player);
                remove_card(&mut self.players[player.index()].graveyard, card_id)
            }
            CastSourceZone::Exile => {
                self.consume_exile_play_permission(card_id);
                // The card is in its owner's exile, which need not be the
                // exile of the player casting it.
                remove_card(&mut self.players[0].exile, card_id)
                    .or_else(|| remove_card(&mut self.players[1].exile, card_id))
            }
            CastSourceZone::LibraryTop => {
                remove_card(&mut self.players[player.index()].library, card_id)
            }
        }
        .expect("legal cast action references a card in its validated source zone")
    }

    fn propose_spell_on_stack(
        &mut self,
        player: PlayerId,
        card: CardInstance,
        signature: CastSignature,
        source_zone: CastSourceZone,
        cast_via_flashback: bool,
        face_down: Option<crate::card::FaceDownCharacteristics>,
    ) -> StackObject {
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
        let id = card.id;
        let frozen_spell_ability = self.frozen_spell_payload(card.definition, &signature);
        // Read now, because nothing afterwards can tell: by resolution the
        // step has usually moved on, and the stack is empty again.
        let cast_at_instant_speed = !self.sorcery_speed_available(player);
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
            colors: None,
            cast_via_flashback,
            cast_at_instant_speed,
            cast_from_zone: Some(source_zone),
            face_down,
            colors_of_mana_spent: crate::card::ColorSet::empty(),
            phyrexian_symbols_paid_with_life: 0,
            is_copy: false,
        }
    }

    /// Completes the mana-payment portion of a cast. A mana ability may pay
    /// by sacrificing a permanent whose exit needs a CR 616 replacement
    /// choice. In that case the spell and the rest of its frozen payment plan
    /// remain on the prospective exit batch until the ability has completed.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn continue_spell_mana_payment(
        &mut self,
        mut stack_object: StackObject,
        targets: Vec<Target>,
        object_payments: Vec<(GameObjectId, SpendModeDef)>,
        cost: ManaCost,
        x: u16,
        purpose: ManaPaymentPurpose,
        plan: Vec<super::PlannedManaActivation>,
        mut next_activation: usize,
    ) {
        while let Some(payment) = plan.get(next_activation).copied() {
            next_activation += 1;
            let super::PlannedPaymentKind::Mana {
                ability,
                color,
                counters_removed,
                cost_object,
                combination,
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
                ManaActivationChoices {
                    counters_removed,
                    cost_object,
                    combination,
                },
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
        self.exile_graveyard_cards(stack_object.controller, &exiled);
        let (mana_cost, mana_x) =
            self.residual_cost_after_contributions(cost, x, &purpose, &plan, true);
        // The spell's nonmana life bill was paid before this continuation
        // began. Do not reserve it a second time when repeatable life mana
        // supplies the final shortfall after the planned abilities resolve.
        let payment_purpose = match &purpose {
            ManaPaymentPurpose::Spell {
                object,
                definition,
                controller,
                form,
                ..
            } => ManaPaymentPurpose::Spell {
                object: *object,
                definition: *definition,
                controller: *controller,
                form: form.clone(),
                reserved_life_payment: 0,
            },
            ManaPaymentPurpose::Ability { .. } | ManaPaymentPurpose::Other => purpose.clone(),
        };
        let spent_mana =
            self.pay_player_cost_for(stack_object.controller, mana_cost, mana_x, &payment_purpose);
        self.apply_spent_mana_to_spell(&mut stack_object, &spent_mana);
        // Recorded whether or not this spell counts them: what paid for a
        // spell is a fact about the cast, and a clause that asks later has
        // nothing else to read it from.
        for mana in &spent_mana {
            if mana.color != ManaColor::Colorless {
                stack_object.colors_of_mana_spent =
                    stack_object.colors_of_mana_spent.with(mana.color);
            }
        }
        self.continue_spell_cast(stack_object, targets, object_payments);
    }

    /// The same bookkeeping for a land, which has no signature to resolve
    /// and whose permanent is still in hand rather than on the battlefield.
    fn spend_graveyard_land_permission(
        &mut self,
        permanent: &mut Permanent,
        player: PlayerId,
        option: &PlayOptionDef,
    ) {
        let Some(card) = permanent.card.clone().into_card() else {
            return;
        };
        if let Some((source, effect)) = self.graveyard_play_grant(&card, player, option) {
            self.grant_resolved_ability_to_entering_permanent(permanent, source, *effect);
        }
        self.record_graveyard_permission_use(&card, player, option);
    }

    /// The graveyard-permission bookkeeping for a cast, resolved from the
    /// card and the play option the signature names.
    /// Hands a spell what the permission that allowed it grants, for the
    /// permanent it will become to carry.
    fn attach_permission_grant(
        stack_object: &mut StackObject,
        granted: Option<(AbilitySourceRef, &'static AppliedEffectDef)>,
    ) {
        let Some((granting, effect)) = granted else {
            return;
        };
        stack_object.applied_effects.push(AppliedStackEffect {
            source: None,
            granting: Some(granting),
            effect: *effect,
        });
    }

    /// A cast from a graveyard that is not one of the card's own printed ways
    /// of being cast is happening under somebody's permission, and a
    /// permission that allows only so many spends one here.
    fn spend_graveyard_cast_permission(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
        source_zone: CastSourceZone,
        alternative_kind: Option<AlternativeCastKindDef>,
    ) -> Option<(AbilitySourceRef, &'static AppliedEffectDef)> {
        if source_zone != CastSourceZone::Graveyard || alternative_kind.is_some() {
            return None;
        }
        self.record_graveyard_permission_use_for_cast(player, card_id, signature)
    }

    fn record_graveyard_permission_use_for_cast(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
    ) -> Option<(AbilitySourceRef, &'static AppliedEffectDef)> {
        let card = self.players[player.index()]
            .graveyard
            .iter()
            .find(|candidate| candidate.id == card_id)
            .cloned()?;
        let option = self
            .catalog
            .get(card.definition)
            .and_then(|definition| definition.play_option(signature.play_option()))
            .cloned()?;
        // "If you do, it gains ...": read here, where the card is still in
        // the graveyard and the permission that names it can still be found.
        // The spell it becomes does not exist yet, so the caller carries it
        // the few lines to the stack object.
        let granted = self.graveyard_play_grant(&card, player, &option);
        self.record_graveyard_permission_use(&card, player, &option);
        granted
    }

    fn complete_spell_cast(&mut self, stack_object: StackObject, targets: Vec<Target>) {
        let player = stack_object.controller;
        let stack_id = stack_object.id;
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
        let mut targeted = Vec::new();
        let mut targeted_players = Vec::new();
        for target in &targets {
            match target {
                Target::Permanent(id) | Target::Card(id) if !targeted.contains(id) => {
                    targeted.push(*id);
                }
                // Once per targeting spell however many of its slots name
                // the same player, exactly as for an object (CR 115.7c).
                Target::Player(player) if !targeted_players.contains(player) => {
                    targeted_players.push(*player);
                }
                _ => {}
            }
        }
        self.events.push(GameEvent::SpellCast {
            player,
            card: stack_id,
            definition,
            targets,
        });
        self.capture_battlefield_triggers(&CommittedTriggerEvent::SpellCast {
            object: cast_event.clone(),
        });
        self.capture_crime_triggers(player, &crime_targets);
        // "Whenever this becomes the target of a spell" fires here, where the
        // targets are locked in -- once per targeting spell however many of
        // its slots name the same permanent (CR 115.7c).
        for target in targeted {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::BecameTargetOfSpell {
                target,
                object: cast_event.clone(),
            });
        }
        for player in targeted_players {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::PlayerBecameTarget {
                player,
                object: cast_event.clone(),
            });
        }
        // The spell's own cast clause, which no battlefield listener carries.
        self.capture_own_cast_triggers(stack_id);
    }
}
