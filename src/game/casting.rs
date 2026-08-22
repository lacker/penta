use super::{
    AbilityCostDef, AbilityOrigin, AlternativeCastKindDef, BTreeMap, BattlefieldExitCompletion,
    CREATURE_TYPES, CardBehavior, CardDefinition, CardInstance, CardType, CardTypeSet, CastChoices,
    CastOfferCost, CastSignature, CastSourceZone, CommittedTriggerEvent, CostConfiguration,
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    DeclarativeAbilityDef, EntryCompletion, Game, GameEvent, GameObjectId, Mana,
    ManaAbilityActivation, ManaActivationChoices, ManaColor, ManaCost, ManaPaymentPurpose,
    PendingBattlefieldEntry, Permanent, PlayActionKind, PlayOptionDef, PlayOptionId, PlayerId,
    StackObject, StackObjectKind, Target, ZoneKind, ZoneMoveCause, ZonePlacement, remove_card,
};
mod signature_validation;
include!("casting/life_costs.rs");
include!("casting/mana_activation.rs");

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
        let permanent =
            Permanent::entering(card, presented, player, self.turns_started[player.index()]);
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

    /// "You may have this enter as a copy of ...": the copy is picked as the
    /// permanent enters, and entering as itself is always an option.
    pub(super) fn queue_entry_copy_choice(
        &mut self,
        player: PlayerId,
        choices: Vec<GameObjectId>,
        added_types: CardTypeSet,
        retain_printed_subtypes: bool,
        added_abilities: Vec<super::CopiableAbility>,
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
            let presentation = Self::effective_rules_source(permanent);
            Some(DecisionOption {
                id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                label: self.presentation_name(presentation).map_or_else(
                    || "Copy an unknown permanent".into(),
                    |name| format!("Enter as a copy of {name}"),
                ),
                card: Some((*id, presentation)),
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
                retain_printed_subtypes,
                added_abilities,
            },
        );
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

    pub(super) fn entry_scalar_choices(
        &self,
        player: PlayerId,
        choice: BattlefieldEntryScalarChoiceDef,
    ) -> (&'static str, Vec<String>) {
        let (prompt, mut choices, fallback) = match choice.list {
            ScalarChoiceListDef::BasicLandTypes => (
                "Choose a basic land type",
                crate::card::BasicLandType::ALL
                    .into_iter()
                    .map(|land_type| land_type.subtype().to_owned())
                    .collect::<Vec<_>>(),
                crate::card::BasicLandType::Plains.subtype(),
            ),
            ScalarChoiceListDef::CardNames | ScalarChoiceListDef::NonlandCardNames => {
                let nonland_only = choice.list == ScalarChoiceListDef::NonlandCardNames;
                let mut names = self
                    .catalog
                    .definitions()
                    .into_iter()
                    .filter(|definition| definition.debut_set != CardSet::Token)
                    .flat_map(|definition| definition.parts.iter())
                    // A split card is nameable half by half, so the land test
                    // belongs to the part rather than to the whole card.
                    .filter(|part| !nonland_only || !part.rules.types().contains(CardType::Land))
                    .map(|part| part.name.clone())
                    .collect::<Vec<_>>();
                names.sort();
                names.dedup();
                (
                    if nonland_only {
                        "Choose a nonland card name"
                    } else {
                        "Choose a card name"
                    },
                    names,
                    "Black Lotus",
                )
            }
            ScalarChoiceListDef::CreatureTypes => (
                "Choose a creature type",
                self.creature_type_choices(player),
                "Human",
            ),
        };
        // A deliberately tiny catalog must not strand an entry procedure.
        if choices.is_empty() {
            choices.push(fallback.into());
        }
        (prompt, choices)
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

    pub(super) fn cast_spell(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
        sacrifices: &[GameObjectId],
    ) {
        let (signature, cost, behavior, source_zone) = self
            .validated_cast_signature(player, card_id, choices)
            .expect("validated casting choices remain valid while paying costs");
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
        // Life named by the chosen alternative, by the spell's own additional
        // cost, or by the permission that let it be cast off a library, is
        // paid alongside its mana, before the spell is finished on the stack.
        if life > 0 {
            self.lose_life(player, life);
        }
        // Read before the permission is consumed above; spent here, where
        // every other cost for this cast is paid.
        if energy > 0 {
            self.spend_energy(player, energy);
        }
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
        Self::apply_spent_mana_to_spell(&mut stack_object, &spent_mana);
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

    pub(super) fn continue_spell_cast(
        &mut self,
        stack_object: StackObject,
        targets: Vec<Target>,
        remaining_sacrifices: Vec<(GameObjectId, SpendModeDef)>,
    ) {
        let Some((stack_object, targets)) =
            self.pay_spell_object_costs(stack_object, targets, remaining_sacrifices)
        else {
            return;
        };
        self.complete_spell_cast(stack_object, targets);
    }

    fn pay_spell_object_costs(
        &mut self,
        stack_object: StackObject,
        targets: Vec<Target>,
        mut remaining_sacrifices: Vec<(GameObjectId, SpendModeDef)>,
    ) -> Option<(StackObject, Vec<Target>)> {
        // The action carries object choices in the same order as their
        // additional-cost clauses. Process one at a time so a mandatory
        // return/exile cost and an optional sacrifice cost retain distinct
        // spend operations even when both were selected for the same cast.
        while let Some((spent, spend)) = remaining_sacrifices.first().copied() {
            remaining_sacrifices.remove(0);
            if self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == spent)
            {
                match spend {
                    SpendModeDef::ByZone => {
                        self.move_permanents_to_graveyard_then(
                            &[spent],
                            Some(BattlefieldExitCompletion::CompleteSpellCast {
                                object: Box::new(stack_object),
                                targets,
                                remaining_sacrifices,
                            }),
                        );
                        return None;
                    }
                    SpendModeDef::Exile | SpendModeDef::ReturnToHand => {
                        let destination =
                            Self::additional_cost_destination(spend, ZoneKind::Battlefield);
                        self.move_target_to_zone(
                            Target::Permanent(spent),
                            destination,
                            ZoneMoveCause::Effect {
                                controller: stack_object.controller,
                            },
                            None,
                            ZonePlacement::Top,
                        );
                    }
                }
                continue;
            }

            let Some((from, card)) = self
                .card_in_nonbattlefield_zone(spent)
                .map(|(zone, card)| (zone, card.clone()))
            else {
                continue;
            };
            let destination = Self::additional_cost_destination(spend, from);
            let owner = card.owner;
            // "One or more cards" exiled from a graveyard by one payment is
            // one move and therefore one trigger event. Keep that upstream
            // batching while retaining each object's own spend provenance:
            // a following return-to-hand object is not part of this batch.
            if from == ZoneKind::Graveyard && destination == ZoneKind::Exile {
                self.exile_graveyard_payment_batch(owner, spent, &mut remaining_sacrifices);
                continue;
            }
            let moved = self.move_card_from_nonbattlefield_zone(
                spent,
                from,
                destination,
                ZoneMoveCause::Effect {
                    controller: stack_object.controller,
                },
                None,
            );
            if from == ZoneKind::Hand
                && destination == ZoneKind::Graveyard
                && let Some((card, actual_destination)) = moved
                && actual_destination == ZoneKind::Graveyard
            {
                self.events.push(GameEvent::CardsDiscarded {
                    player: owner,
                    cards: vec![(card.id, card.definition)],
                });
                self.capture_battlefield_triggers(&CommittedTriggerEvent::Discarded {
                    player: owner,
                });
                self.capture_battlefield_triggers(&CommittedTriggerEvent::CardsDiscarded {
                    player: owner,
                });
            }
        }

        Some((stack_object, targets))
    }

    const fn additional_cost_destination(spend: SpendModeDef, from: ZoneKind) -> ZoneKind {
        match spend {
            SpendModeDef::ReturnToHand => ZoneKind::Hand,
            SpendModeDef::ByZone if matches!(from, ZoneKind::Hand | ZoneKind::Battlefield) => {
                ZoneKind::Graveyard
            }
            SpendModeDef::Exile | SpendModeDef::ByZone => ZoneKind::Exile,
        }
    }

    fn exile_graveyard_payment_batch(
        &mut self,
        owner: PlayerId,
        spent: GameObjectId,
        remaining_sacrifices: &mut Vec<(GameObjectId, SpendModeDef)>,
    ) {
        let mut exiled = Vec::new();
        let mut next = Some(spent);
        while let Some(id) = next.take() {
            if let Some(card) = remove_card(&mut self.players[owner.index()].graveyard, id) {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[owner.index()].exile.push(card.clone());
                exiled.push(card);
            }
            next =
                remaining_sacrifices
                    .first()
                    .copied()
                    .and_then(|(candidate, candidate_spend)| {
                        let (candidate_zone, candidate_card) =
                            self.card_in_nonbattlefield_zone(candidate)?;
                        let candidate_destination =
                            Self::additional_cost_destination(candidate_spend, candidate_zone);
                        (candidate_zone == ZoneKind::Graveyard
                            && candidate_destination == ZoneKind::Exile
                            && candidate_card.owner == owner)
                            .then_some(candidate)
                    });
            if next.is_some() {
                remaining_sacrifices.remove(0);
            }
        }
        if !exiled.is_empty() {
            self.capture_cards_exiled(&exiled, ZoneKind::Graveyard);
            self.note_card_left_graveyard(owner);
        }
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
        self.spells_cast_this_turn[player.index()] =
            self.spells_cast_this_turn[player.index()].saturating_add(1);
        // Kept for the targeting triggers below, which run after the cast
        // event has taken the list.
        let crime_targets = targets.clone();
        let mut targeted = Vec::new();
        for target in &targets {
            if let Target::Permanent(id) | Target::Card(id) = target
                && !targeted.contains(id)
            {
                targeted.push(*id);
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
        // The spell's own cast clause, which no battlefield listener carries.
        self.capture_own_cast_triggers(stack_id);
    }
}
