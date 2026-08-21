use std::borrow::Cow;

use super::{
    CardInstance, CardPartId, CastOffer, CastOfferCost, CastSourceZone, CharacteristicContext,
    CharacteristicSource, ColorSet, DecisionContinuation, DecisionKind, DecisionObservation,
    DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone, DeclarativeAbilityDef,
    EffectResolutionContext, FORK_COPY_COLOR, Game, ManaCost, ObjectCharacteristics,
    PendingDecision, PlayerId, ResolvedEffectPayment, ScopedEffect, StackObject, Target,
    TargetSelection, TargetSlotId, TemporaryAbilityGrant, TriggerContext, ZoneKind, ZoneMoveCause,
    ZonePlacement, flatten_target_selections, target_combinations,
};
use crate::card::{
    AbilityDef, AlternativeCastKindDef, ChoiceVisibilityDef, EffectDef, ObjectPredicateDef,
};
use crate::ids::GameObjectId;

pub(super) const fn effect_choice_visibility(
    visibility: ChoiceVisibilityDef,
) -> DecisionVisibility {
    match visibility {
        ChoiceVisibilityDef::Public => DecisionVisibility::Public,
        ChoiceVisibilityDef::Private => DecisionVisibility::Private,
    }
}

impl Game {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_decision(
        &mut self,
        player: PlayerId,
        prompt: impl Into<String>,
        visibility: DecisionVisibility,
        preference: DecisionPreference,
        bounds: std::ops::RangeInclusive<usize>,
        cancellable: bool,
        options: Vec<DecisionOption>,
        continuation: DecisionContinuation,
    ) {
        // A player can only choose from what is there. Asking for a minimum
        // the options cannot supply leaves no legal `ChooseDecision`, because
        // `is_legal` requires at least `minimum` of them — and when the
        // decision is also not cancellable, the game has no legal action at
        // all and deadlocks. Demonic Tutor did exactly that on an empty
        // library. Magic resolves as much of an effect as it can, so lower the
        // requirement to what exists and let the continuation take it from
        // there; each one already handles being handed nothing.
        let minimum = (*bounds.start()).min(options.len());

        let id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        self.pending_decisions.push(PendingDecision {
            observation: DecisionObservation {
                id,
                player,
                kind: DecisionKind::Choice,
                order_semantics: None,
                prompt: prompt.into(),
                visibility,
                preference,
                minimum,
                maximum: (*bounds.end()).max(minimum),
                cancellable,
                options,
            },
            continuation,
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_pay_or(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        visibility: ChoiceVisibilityDef,
        definition: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
        if_paid: Option<ScopedEffect>,
        otherwise: Option<ScopedEffect>,
    ) {
        if if_paid.is_none() && otherwise.is_none() {
            return;
        }
        let can_pay = self.can_pay_effect_payment(player, payment);
        if !can_pay && let Some(effect) = otherwise {
            self.resolve_effect_def(effect, object, context);
            return;
        }
        let options = self.payment_options(player, payment, can_pay, "Decline");
        self.queue_decision(
            player,
            object.ability_text().unwrap_or("Pay the cost?"),
            effect_choice_visibility(visibility),
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::PayOr {
                player,
                payment,
                definition,
                object: Box::new(object.clone()),
                context,
                if_paid,
                otherwise,
            },
        );
    }

    /// Applies a payment decision's answer: option zero declines, and every
    /// other option is a way of paying. Only a matching discard has more than
    /// one, and its option carries the card that goes.
    pub(super) fn settle_payment_decision(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        answered: &[u32],
        options: &[DecisionOption],
    ) -> Option<u16> {
        let chosen = answered.iter().copied().find(|option| *option != 0)?;
        match payment {
            // The option id is the amount, so the answer carries how much was
            // paid without a second question.
            ResolvedEffectPayment::ChosenGenericMana => {
                let amount = u16::try_from(chosen).unwrap_or(u16::MAX);
                let cost = ManaCost::new(amount, 0);
                if !self.can_pay_cost(player, cost, 0) {
                    return None;
                }
                self.activate_mana_for_cost(player, cost, 0);
                let _spent = self.pay_player_cost(player, cost, 0);
                Some(amount)
            }
            ResolvedEffectPayment::ReturnPermanentMatching(predicate) => {
                let permanent = options
                    .iter()
                    .find(|option| option.id == chosen)
                    .and_then(|option| option.card)
                    .map(|(permanent, _)| permanent)?;
                if !self
                    .matching_permanents_controlled(player, predicate)
                    .contains(&permanent)
                {
                    return None;
                }
                self.move_target_to_zone(
                    Target::Permanent(permanent),
                    ZoneKind::Hand,
                    ZoneMoveCause::Effect { controller: player },
                    None,
                    ZonePlacement::Top,
                );
                Some(0)
            }
            ResolvedEffectPayment::SacrificePermanentMatching(predicate) => {
                let permanent = options
                    .iter()
                    .find(|option| option.id == chosen)
                    .and_then(|option| option.card)
                    .map(|(permanent, _)| permanent)?;
                if !self
                    .matching_permanents_controlled(player, predicate)
                    .contains(&permanent)
                {
                    return None;
                }
                self.move_permanents_to_graveyard(&[permanent]);
                Some(0)
            }
            ResolvedEffectPayment::DiscardMatching(predicate) => {
                let card = options
                    .iter()
                    .find(|option| option.id == chosen)
                    .and_then(|option| option.card)
                    .map(|(card, _)| card)?;
                self.pay_matching_discard(player, predicate, card)
                    .then_some(0)
            }
            payment => (chosen == 1 && self.pay_effect_payment(player, payment)).then_some(0),
        }
    }

    /// The largest generic payment the player could make right now, which is
    /// what a chosen-amount payment offers. Read through the ordinary cost
    /// check so an unspendable source cannot inflate the list.
    pub(super) fn maximum_generic_payment(&self, player: PlayerId) -> u16 {
        let mut amount = 0;
        while amount < u16::MAX && self.can_pay_cost(player, ManaCost::new(amount + 1, 0), 0) {
            amount += 1;
        }
        amount
    }

    /// Asks `player` to name a card while an effect resolves, then continues
    /// that effect with the answer. The catalog supplies the list, which is
    /// the same one an entering permanent's naming choice offers.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_card_name_choice(
        &mut self,
        player: PlayerId,
        nonland_only: bool,
        searched: PlayerId,
        zone: ZoneKind,
        binding: crate::ObjectSetBindingIndex,
        object: StackObject,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    ) {
        let choice = if nonland_only {
            crate::card::BattlefieldEntryScalarChoiceDef::NONLAND_CARD_NAME
        } else {
            crate::card::BattlefieldEntryScalarChoiceDef::CARD_NAME
        };
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
            DecisionContinuation::CardNameChoice {
                choices,
                searched,
                zone,
                binding,
                object: Box::new(object),
                context,
                effect,
            },
        );
    }

    pub(super) fn can_pay_effect_payment(
        &self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
    ) -> bool {
        match payment {
            ResolvedEffectPayment::Mana(cost) => self.can_pay_cost(player, cost, 0),
            ResolvedEffectPayment::Life(amount) => i16::try_from(amount)
                .is_ok_and(|amount| self.players[player.index()].life >= amount),
            // Unlike life, energy cannot be spent past nothing: a player
            // short of the amount cannot pay at all.
            ResolvedEffectPayment::Energy(amount) => self.players[player.index()].energy >= amount,
            // A short library is not a failure to pay, so this is always
            // affordable. Running out of cards is answered by the draw that
            // finds none, not by refusing the payment.
            ResolvedEffectPayment::Mill(_) => true,
            // A discard needs cards to choose from, so an empty hand cannot
            // pay at all. That is the difference from a mill, where a short
            // library still pays with what it has.
            ResolvedEffectPayment::Discard(amount) => {
                self.players[player.index()].hand.len() >= usize::from(amount)
            }
            // A hand full of spells cannot pay for a land, which is the whole
            // difference between this and the count above.
            ResolvedEffectPayment::DiscardMatching(predicate) => {
                !self.matching_cards_in_hand(player, predicate).is_empty()
            }
            // Paying nothing is not paying, so this needs one generic mana
            // before the choice is worth offering at all.
            ResolvedEffectPayment::ChosenGenericMana => {
                self.can_pay_cost(player, ManaCost::new(1, 0), 0)
            }
            // Payable only when the creatures on the board could add up to
            // it at all, so a player who cannot pay is never asked.
            ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total) => {
                self.total_creature_power_controlled(player) >= i32::from(total)
            }
            ResolvedEffectPayment::ReturnPermanentMatching(predicate)
            | ResolvedEffectPayment::SacrificePermanentMatching(predicate) => !self
                .matching_permanents_controlled(player, predicate)
                .is_empty(),
        }
    }

    /// The payer's own cards that a payment predicate matches, in hand order.
    /// This is the candidate list a payment decision offers and the one its
    /// checkpoint rebuilds, so both read it from the same place.
    pub(super) fn matching_cards_in_hand(
        &self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
    ) -> Vec<CardInstance> {
        self.players[player.index()]
            .hand
            .iter()
            .filter(|card| {
                self.printed_trigger_event_object(
                    card.id,
                    card.definition,
                    player,
                    &CharacteristicContext::Hand,
                )
                .is_some_and(|object| {
                    self.trigger_object_matches(predicate, &object, card.id, false)
                })
            })
            .cloned()
            .collect()
    }

    /// The payer's own permanents a payment predicate matches, in battlefield
    /// order. Read by the option list and by the payment that follows it, so
    /// a permanent that stopped matching in between cannot be spent.
    pub(super) fn matching_permanents_controlled(
        &self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
    ) -> Vec<GameObjectId> {
        self.battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| {
                self.trigger_object_matches_for_controller(
                    predicate,
                    &self.trigger_event_object(permanent),
                    permanent.card.id,
                    false,
                    Some(player),
                )
            })
            .map(|permanent| permanent.card.id)
            .collect()
    }

    /// The options a payment decision offers: declining, and then one entry
    /// per way of paying. Everything but a matching discard has exactly one
    /// way, so it stays the single option it has always been.
    pub(super) fn payment_options(
        &self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        can_pay: bool,
        decline: &str,
    ) -> Vec<DecisionOption> {
        let mut options = vec![DecisionOption {
            id: 0,
            label: decline.into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::None,
        }];
        if !can_pay {
            return options;
        }
        match payment {
            // One option per amount the payer can actually afford, with the
            // amount as the option id.
            ResolvedEffectPayment::ChosenGenericMana => {
                for amount in 1..=self.maximum_generic_payment(player) {
                    options.push(DecisionOption {
                        id: u32::from(amount),
                        label: format!("Pay {{{amount}}}"),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    });
                }
            }
            ResolvedEffectPayment::ReturnPermanentMatching(predicate)
            | ResolvedEffectPayment::SacrificePermanentMatching(predicate) => {
                let returning =
                    matches!(payment, ResolvedEffectPayment::ReturnPermanentMatching(_));
                for (index, permanent) in self
                    .matching_permanents_controlled(player, predicate)
                    .into_iter()
                    .enumerate()
                {
                    let name = self
                        .permanent_card_name(permanent)
                        .map_or_else(|| "a permanent".to_string(), std::borrow::Cow::into_owned);
                    options.push(DecisionOption {
                        id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                        label: if returning {
                            format!("Return {name}")
                        } else {
                            format!("Sacrifice {name}")
                        },
                        card: self
                            .battlefield
                            .iter()
                            .find(|candidate| candidate.card.id == permanent)
                            .map(|candidate| (permanent, Self::effective_rules_source(candidate))),
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::Battlefield,
                    });
                }
            }
            ResolvedEffectPayment::DiscardMatching(predicate) => {
                for (index, card) in self
                    .matching_cards_in_hand(player, predicate)
                    .into_iter()
                    .enumerate()
                {
                    let name = self
                        .catalog
                        .get(card.definition)
                        .map_or_else(|| "a card".to_string(), |card| card.name.clone());
                    options.push(DecisionOption {
                        id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                        label: format!("Discard {name}"),
                        card: Some((
                            card.id,
                            ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                        )),
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::Hand,
                    });
                }
            }
            payment => options.push(DecisionOption {
                id: 1,
                label: Self::effect_payment_label(payment),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            }),
        }
        options
    }

    pub(super) fn pay_effect_payment(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
    ) -> bool {
        if !self.can_pay_effect_payment(player, payment) {
            return false;
        }
        match payment {
            ResolvedEffectPayment::Mana(cost) => {
                self.activate_mana_for_cost(player, cost, 0);
                let _spent = self.pay_player_cost(player, cost, 0);
            }
            ResolvedEffectPayment::Life(amount) => self.lose_life(player, amount),
            ResolvedEffectPayment::Energy(amount) => {
                let _paid = self.spend_energy(player, amount);
            }
            ResolvedEffectPayment::Mill(amount) => {
                let milled = self.take_top_of_library(player, usize::from(amount));
                self.bury_cards(player, milled);
            }
            // Queued rather than resolved here: the payer has already chosen
            // to pay, and which cards go is a separate choice that the branch
            // taken above does not depend on.
            ResolvedEffectPayment::Discard(amount) => self.queue_effect_discards(
                vec![player],
                i32::from(amount),
                ZoneMoveCause::Effect { controller: player },
            ),
            // Both are paid by [`Self::settle_payment_decision`], which knows
            // which card was named or how much was chosen. Reaching here
            // means a caller lost that answer.
            ResolvedEffectPayment::DiscardMatching(_)
            | ResolvedEffectPayment::ChosenGenericMana
            | ResolvedEffectPayment::ReturnPermanentMatching(_)
            | ResolvedEffectPayment::SacrificePermanentMatching(_)
            // Named one creature at a time by its own decision, which is
            // queued once the payer has already chosen to pay.
            | ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(_) => return false,
        }
        true
    }

    /// Pays a matching discard with the card the payer named. The card is
    /// checked against the predicate again rather than trusted: the option
    /// list was built before the decision was answered.
    pub(super) fn pay_matching_discard(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        card: GameObjectId,
    ) -> bool {
        if !self
            .matching_cards_in_hand(player, predicate)
            .iter()
            .any(|candidate| candidate.id == card)
        {
            return false;
        }
        self.discard_cards_with_cause(
            player,
            &[card],
            ZoneMoveCause::Effect { controller: player },
        );
        true
    }

    pub(super) fn effect_payment_label(payment: ResolvedEffectPayment) -> String {
        match payment {
            ResolvedEffectPayment::Mana(_) => "Pay the cost".to_string(),
            ResolvedEffectPayment::Life(amount) => format!("Pay {amount} life"),
            ResolvedEffectPayment::Energy(amount) => format!("Pay {amount} energy"),
            ResolvedEffectPayment::Mill(amount) => format!("Mill {amount} cards"),
            ResolvedEffectPayment::Discard(amount) => format!("Discard {amount} cards"),
            // Every candidate carries its own label, so this one only names
            // the prompt the decision is introduced with.
            ResolvedEffectPayment::DiscardMatching(_) => "Discard a matching card".to_string(),
            ResolvedEffectPayment::ChosenGenericMana => "Pay {X}".to_string(),
            ResolvedEffectPayment::ReturnPermanentMatching(_) => {
                "Return a matching permanent".to_string()
            }
            ResolvedEffectPayment::SacrificePermanentMatching(_) => {
                "Sacrifice a matching permanent".to_string()
            }
            ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total) => {
                format!("Sacrifice creatures with total power {total} or greater")
            }
        }
    }

    /// "Exile the top card of your library. You may cast that card."
    ///
    /// The card is exiled whatever happens next. The offer to cast it is a
    /// standing decision rather than a yes-or-no: casting is a `CastSpell`
    /// action taken while the decision waits, and the decision itself is the
    /// decline. An offer nobody could take is not made at all -- an empty
    /// library, or a card there is no legal way to cast -- and `otherwise`
    /// runs straight away instead.
    pub(super) fn exile_top_and_offer_cast(
        &mut self,
        player: PlayerId,
        object: &StackObject,
        context: EffectResolutionContext,
        definition: ScopedEffect,
    ) {
        let Some(card) = self.take_top_of_library(player, 1).into_iter().next() else {
            self.resolve_declined_cast(object, context, definition);
            return;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let exiled = card.id;
        let printed = card.definition;
        let name = self
            .catalog
            .get(printed)
            .map_or_else(|| "that card".to_owned(), |card| card.name.clone());
        self.players[player.index()].exile.push(card.clone());
        self.capture_cards_exiled(std::slice::from_ref(&card), ZoneKind::Library);
        self.permit_cast_this_turn(exiled, player);
        let mut castable = Vec::new();
        self.add_offered_cast_actions(
            CastOffer {
                player,
                card: exiled,
                source_zone: CastSourceZone::Exile,
                cost: CastOfferCost::Any,
            },
            &mut castable,
        );
        if castable.is_empty() {
            self.consume_exile_play_permission(exiled);
            self.resolve_declined_cast(object, context, definition);
            return;
        }
        self.queue_decision(
            player,
            format!("Cast {name} from exile, or decline"),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            1..=1,
            false,
            vec![DecisionOption {
                id: 0,
                label: "Decline".into(),
                card: Some((
                    exiled,
                    ObjectCharacteristics::card(printed, CardPartId::PRIMARY),
                )),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Exile,
            }],
            DecisionContinuation::MayCastExiled {
                player,
                card: exiled,
                object: Box::new(object.clone()),
                context,
                definition,
            },
        );
    }

    /// "You may cast target instant or sorcery card from your graveyard
    /// without paying its mana cost."
    ///
    /// The card is lent `ability` for exactly as long as the offer stands.
    /// Like the exile offer above, casting takes the decision away and
    /// answering it is the decline; unlike that one there is nothing else to
    /// do afterwards, so a card left uncast simply keeps nothing.
    pub(super) fn offer_granted_cast(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
        ability: &'static AbilityDef,
    ) {
        let Some((ZoneKind::Graveyard, instance)) = self.card_in_nonbattlefield_zone(card) else {
            return;
        };
        let printed = instance.definition;
        let name = self
            .catalog
            .get(printed)
            .map_or_else(|| "that card".to_owned(), |card| card.name.clone());
        let grant = TemporaryAbilityGrant {
            object: card,
            ability: *ability,
        };
        let grant_index = self.temporary_ability_grants.len();
        self.temporary_ability_grants.push(grant);
        let DeclarativeAbilityDef::AlternativeCast(_) = ability.definition else {
            self.revoke_temporary_grant(grant_index, card, ability);
            return;
        };
        let mut castable = Vec::new();
        self.add_offered_cast_actions(
            CastOffer {
                player,
                card,
                source_zone: CastSourceZone::Graveyard,
                cost: CastOfferCost::GrantedAlternative(grant_index),
            },
            &mut castable,
        );
        if castable.is_empty() {
            self.revoke_temporary_grant(grant_index, card, ability);
            return;
        }
        self.queue_decision(
            player,
            format!("Cast {name} without paying its mana cost, or decline"),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            1..=1,
            false,
            vec![DecisionOption {
                id: 0,
                label: "Decline".into(),
                card: Some((
                    card,
                    ObjectCharacteristics::card(printed, CardPartId::PRIMARY),
                )),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Graveyard,
            }],
            DecisionContinuation::MayCastGranted {
                player,
                card,
                ability: *ability,
                grant: grant_index,
            },
        );
    }

    /// Offers the exact alternative cast named by a resolving linked
    /// ability. The card has not moved; the standing decision is the entire
    /// permission and answering its sole option declines it.
    pub(super) fn queue_alternative_cast_offer(
        &mut self,
        player: PlayerId,
        card: crate::GameObjectId,
        ability: crate::AbilityOrigin,
    ) {
        let Some(held) = self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)
        else {
            return;
        };
        let definition = held.definition;
        let Some(ability_definition) = self.ability_for_origin(card, ability) else {
            return;
        };
        let DeclarativeAbilityDef::AlternativeCast(alternative) = ability_definition.definition
        else {
            return;
        };
        if !ability_definition.is_executable()
            || alternative.kind != AlternativeCastKindDef::Miracle
        {
            return;
        }
        let name = self
            .catalog
            .get(definition)
            .map_or_else(|| "that card".to_owned(), |card| card.name.clone());
        let offer = CastOffer {
            player,
            card,
            source_zone: CastSourceZone::Hand,
            cost: CastOfferCost::PrintedAlternative(ability),
        };
        let mut castable = Vec::new();
        self.add_offered_cast_actions(offer, &mut castable);
        if castable.is_empty() {
            return;
        }
        self.queue_decision(
            player,
            format!(
                "Cast {name} for its {} cost, or decline",
                alternative.kind.label()
            ),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            1..=1,
            false,
            vec![DecisionOption {
                id: 0,
                label: "Decline".into(),
                card: Some((
                    card,
                    ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
                )),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Hand,
            }],
            DecisionContinuation::MayCastAlternative {
                player,
                card,
                ability,
            },
        );
    }

    /// Takes back the exact lent ability behind an offer. Equal grants remain
    /// distinct: a card may be carrying somebody else's identical grant too.
    pub(super) fn revoke_temporary_grant(
        &mut self,
        grant: usize,
        card: GameObjectId,
        ability: &AbilityDef,
    ) {
        if self
            .temporary_ability_grants
            .get(grant)
            .is_some_and(|candidate| candidate.object == card && candidate.ability == *ability)
        {
            self.temporary_ability_grants.remove(grant);
        }
    }

    /// What happens when the card is not cast, whether it was declined or
    /// never offered.
    pub(super) fn resolve_declined_cast(
        &mut self,
        object: &StackObject,
        context: EffectResolutionContext,
        definition: ScopedEffect,
    ) {
        let EffectDef::ExileTopAndMayCast {
            otherwise: Some(otherwise),
            ..
        } = definition.effect
        else {
            return;
        };
        self.resolve_effect_def(definition.with_effect(*otherwise), object, context);
    }

    /// Asks a spell's owner which end of their library it goes on. Nothing
    /// is countered here: the spell is taken off the stack once the answer
    /// arrives, and a spell that cannot be countered leaves all the same.
    pub(super) fn queue_spell_library_end_choice(&mut self, spell: GameObjectId) {
        let Some(owner) = self
            .stack
            .iter()
            .find(|object| object.id == spell)
            .map(|object| object.card.owner)
        else {
            return;
        };
        let name = self
            .stack
            .iter()
            .find(|object| object.id == spell)
            .and_then(|object| self.characteristics_name(object.presentation()))
            .map_or_else(|| "that spell".to_owned(), std::borrow::Cow::into_owned);
        self.queue_decision(
            owner,
            format!("Put {name} on the top or bottom of your library"),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Top".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: "Bottom".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
            ],
            DecisionContinuation::SpellLibraryEnd { owner, spell },
        );
    }

    /// Offers an effect its controller may decline, resolving it only on a
    /// yes. Declining is always available, which is what "may" means.
    pub(super) fn queue_optional_effect(
        &mut self,
        player: PlayerId,
        object: &StackObject,
        context: EffectResolutionContext,
        effect: ScopedEffect,
    ) {
        self.queue_decision(
            player,
            object.ability_text().unwrap_or("Use this optional effect?"),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(1),
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Decline".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: "Do it".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
            ],
            DecisionContinuation::OptionalEffect {
                object: Box::new(object.clone()),
                context,
                effect,
            },
        );
    }

    pub(super) fn target_label(&self, viewer: PlayerId, target: Target) -> String {
        match target {
            Target::Player(player) if player == viewer => "you".into(),
            Target::Player(_) => "your opponent".into(),
            Target::Card(id) => self
                .card_in_nonbattlefield_zone(id)
                .and_then(|(_, card)| self.catalog.get(card.definition))
                .map_or_else(|| "that card".into(), |card| card.name.clone()),
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .and_then(|permanent| self.effective_permanent_name(permanent))
                .map_or_else(|| "that permanent".into(), Cow::into_owned),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|object| object.id == id)
                .and_then(|object| self.characteristics_name(object.presentation()))
                .map_or_else(|| "that spell".into(), Cow::into_owned),
        }
    }

    fn characteristics_name(&self, characteristics: ObjectCharacteristics) -> Option<Cow<'_, str>> {
        match characteristics {
            ObjectCharacteristics::Card { definition, part } => self
                .catalog
                .get(definition)?
                .part(part)
                .map(|part| Cow::Borrowed(part.name.as_str())),
            ObjectCharacteristics::Token { token, part } => {
                token.part(part).map(crate::card::TokenPart::name)
            }
            ObjectCharacteristics::Emblem { emblem } => Some(Cow::Borrowed(emblem.name())),
            ObjectCharacteristics::FaceDown { face_down } => {
                Some(Cow::Borrowed(face_down.display_name()))
            }
        }
    }
}

include!("decision_offers/copies.rs");
