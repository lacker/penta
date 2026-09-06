use std::borrow::Cow;

use super::{
    CardInstance, CardPartId, CastContext, CastOffer, CastOfferCost, CastSourceZone,
    CharacteristicContext, CharacteristicSource, ColorSet, ContinuousEffectExpiration, CounterKind,
    DecisionContinuation, DecisionKind, DecisionObservation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, DeclarativeAbilityDef, EffectResolutionContext, Game,
    ManaCost, NonbattlefieldAbilityGrant, ObjectCharacteristics, PendingDecision, PlayerId,
    ResolvedEffectPayment, ScopedEffect, SettledEffectPayment, StackObject, Target,
    TargetSelection, TriggerContext, ZoneKind, ZoneMoveCause, ZonePlacement,
    flatten_target_selections,
};
use crate::card::{
    AbilityDef, AlternativeCastKindDef, ChoiceVisibilityDef, EffectDef, ObjectPredicateDef,
};
use crate::ids::GameObjectId;

fn selected_payment_members(chosen: u32, options: &[DecisionOption]) -> Vec<GameObjectId> {
    options
        .iter()
        .find(|option| option.id == chosen)
        .map(|option| option.members.iter().map(|(id, _)| *id).collect())
        .unwrap_or_default()
}

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
                source: None,
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

    /// A resolving ability has already left the stack by the time its choice
    /// is observed. Preserve the battlefield source explicitly so clients do
    /// not have to guess which same-name permanent the choice belongs to.
    fn associate_latest_decision_with(&mut self, object: &StackObject) {
        if let Some(source) = object.source
            && let Some(decision) = self.pending_decisions.last_mut()
        {
            decision.observation.source = Some(source);
        }
    }

    /// Applies a payment decision's answer: option zero declines, and every
    /// other option is a way of paying. Payments with multiple legal objects
    /// carry the chosen card or group in their option.
    pub(super) fn settle_payment_decision(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        answered: &[u32],
        options: &[DecisionOption],
    ) -> Option<SettledEffectPayment> {
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
                Some(SettledEffectPayment::without_mana(amount))
            }
            // The same shape in energy: the option id is how much is spent,
            // and energy is spent from the counters rather than raised.
            ResolvedEffectPayment::ChosenEnergy => {
                let amount = u16::try_from(chosen).unwrap_or(u16::MAX);
                self.spend_energy(player, amount)
                    .then_some(SettledEffectPayment::without_mana(amount))
            }
            ResolvedEffectPayment::RemoveAnyNumberOfCounters { object, kind } => self
                .settle_counter_removal_payment(object, kind, chosen)
                .map(SettledEffectPayment::without_mana),
            ResolvedEffectPayment::MovePermanentMatching {
                object: predicate,
                zone,
            } => {
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
                    zone,
                    ZoneMoveCause::Effect { controller: player },
                    None,
                    ZonePlacement::Top,
                );
                Some(SettledEffectPayment::without_mana(0))
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
                self.sacrifice_permanents(&[permanent]);
                Some(SettledEffectPayment::without_mana(0))
            }
            ResolvedEffectPayment::DiscardMatching(predicate) => {
                let card = options
                    .iter()
                    .find(|option| option.id == chosen)
                    .and_then(|option| option.card)
                    .map(|(card, _)| card)?;
                self.pay_matching_discard(player, predicate, card)
                    .then_some(SettledEffectPayment::without_mana(0))
            }
            payment @ (ResolvedEffectPayment::DiscardCards(_)
            | ResolvedEffectPayment::SacrificePermanents { .. }
            | ResolvedEffectPayment::GainControlPermanents { .. }) => {
                self.settle_group_payment_decision(player, payment, chosen, options)
            }
            payment => (chosen == 1)
                .then(|| self.pay_effect_payment_with_mana(player, payment))
                .flatten(),
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

    /// Asks `player` to name a card while an effect resolves. The surrounding
    /// sequence attaches its remaining steps before yielding the decision.
    pub(super) fn queue_card_name_choice(
        &mut self,
        player: PlayerId,
        names: crate::card::CardNameSetDef,
        binding: crate::Binding,
        object: StackObject,
        context: EffectResolutionContext,
    ) {
        let choice = crate::card::BattlefieldEntryScalarChoiceDef::card_name(names);
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
                binding: binding.into(),
                resume: Box::new(super::PendingProcedure::ResolveEffects {
                    effects: Vec::new(),
                    object: Box::new(object),
                    context,
                }),
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
            ResolvedEffectPayment::CumulativeMana { source, cost } => self.can_pay_cost_for(
                player,
                cost,
                0,
                &super::ManaPaymentPurpose::CumulativeUpkeep {
                    source,
                    snow: false,
                },
            ),
            ResolvedEffectPayment::SnowMana { source, amount } => self.can_pay_cost_for(
                player,
                ManaCost::new(amount, 0),
                0,
                &super::ManaPaymentPurpose::CumulativeUpkeep { source, snow: true },
            ),
            ResolvedEffectPayment::Life(amount) => self.can_pay_life(player, amount),
            // A short library does not make either action unpayable: draws
            // are attempted normally, while mill moves as many as remain.
            ResolvedEffectPayment::DrawCards(_)
            | ResolvedEffectPayment::Mill(_)
            | ResolvedEffectPayment::AddMana { .. }
            | ResolvedEffectPayment::OpponentCreatesTokens { .. }
            | ResolvedEffectPayment::FlipCoins(_) => true,
            ResolvedEffectPayment::OpponentGainsLife(amount) => {
                amount == 0
                    || (!self.cannot_gain_life(player.opponent())
                        && !self.life_total_cannot_change(player.opponent()))
            }
            ResolvedEffectPayment::ExileTopCards(amount) => {
                self.players[player.index()].library.len() >= usize::from(amount)
            }
            ResolvedEffectPayment::PutCounters { object, .. } => self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == object),
            // Unlike life, energy cannot be spent past nothing: a player
            // short of the amount cannot pay at all.
            ResolvedEffectPayment::Energy(amount) => {
                self.players[player.index()]
                    .counters
                    .count(CounterKind::named("energy"))
                    >= amount
            }
            // A discard needs cards to choose from, so an empty hand cannot
            // pay at all. That is the difference from a mill, where a short
            // library still pays with what it has.
            ResolvedEffectPayment::Discard(amount)
            | ResolvedEffectPayment::DiscardCards(amount) => {
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
            // Paying nothing is not paying, so one energy counter is what
            // makes the choice worth offering at all.
            ResolvedEffectPayment::ChosenEnergy => {
                self.players[player.index()]
                    .counters
                    .count(CounterKind::named("energy"))
                    >= 1
            }
            ResolvedEffectPayment::RemoveAnyNumberOfCounters { object, kind } => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == object)
                .is_some_and(|permanent| permanent.counters(kind) > 0),
            // Payable only when the creatures on the board could add up to
            // it at all, so a player who cannot pay is never asked.
            ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total) => {
                self.total_creature_power_controlled(player) >= i32::from(total)
            }
            ResolvedEffectPayment::MovePermanentMatching {
                object: predicate, ..
            }
            | ResolvedEffectPayment::SacrificePermanentMatching(predicate) => !self
                .matching_permanents_controlled(player, predicate)
                .is_empty(),
            ResolvedEffectPayment::SacrificePermanents { object, amount } => {
                self.matching_permanents_controlled(player, object).len() >= usize::from(amount)
            }
            ResolvedEffectPayment::GainControlPermanents { object, amount, .. } => {
                self.matching_permanents_not_controlled(player, object)
                    .len()
                    >= usize::from(amount)
            }
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

    pub(super) fn matching_permanents_not_controlled(
        &self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
    ) -> Vec<GameObjectId> {
        self.battlefield
            .iter()
            .filter(|permanent| permanent.controller != player)
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

    fn permanent_payment_options(
        &self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        verb: &str,
    ) -> Vec<DecisionOption> {
        self.matching_permanents_controlled(player, predicate)
            .into_iter()
            .enumerate()
            .map(|(index, permanent)| {
                let name = self
                    .permanent_card_name(permanent)
                    .map_or_else(|| "a permanent".to_string(), std::borrow::Cow::into_owned);
                DecisionOption {
                    id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                    label: format!("{verb} {name}"),
                    card: self
                        .battlefield
                        .iter()
                        .find(|candidate| candidate.card.id == permanent)
                        .map(|candidate| (permanent, Self::effective_rules_source(candidate))),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                }
            })
            .collect()
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

    /// Offers a card the resolving effect has just made playable, and takes
    /// the permission back if it is declined.
    ///
    /// The same standing-decision shape the exile-and-cast offer above uses:
    /// playing the card takes the decision away, and answering it is the
    /// decline. What differs is only where the card came from -- it was
    /// already sitting in exile with the clause's permission on it, rather
    /// than being put there by this effect.
    pub(super) fn offer_permitted_play(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
        mandatory: bool,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some((_, instance)) = self.card_in_nonbattlefield_zone(card) else {
            self.drop_exile_play_permission(card);
            return;
        };
        let printed = instance.definition;
        let name = self
            .catalog
            .get(printed)
            .map_or_else(|| "that card".to_owned(), |card| card.name.clone());
        let mut playable = Vec::new();
        self.add_offered_cast_actions(
            CastOffer {
                player,
                card,
                source_zone: CastSourceZone::Exile,
                cost: CastOfferCost::Any,
            },
            &mut playable,
        );
        // A land is played rather than cast, and the permission is what
        // makes that legal from exile. It is asked for without the ordinary
        // timing gate: this offer is made while an ability resolves, when
        // the stack is not empty and no land could otherwise be played.
        let lands = self.offered_land_actions(player, card);
        if playable.is_empty() && lands.is_empty() {
            self.drop_exile_play_permission(card);
            return;
        }
        self.queue_decision(
            player,
            if mandatory {
                format!("Play {name} from exile")
            } else {
                format!("Play {name} from exile, or decline")
            },
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            if mandatory { 0..=0 } else { 1..=1 },
            false,
            if mandatory {
                Vec::new()
            } else {
                vec![DecisionOption {
                    id: 0,
                    label: "Decline".into(),
                    card: Some((
                        card,
                        ObjectCharacteristics::card(printed, CardPartId::PRIMARY),
                    )),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Exile,
                }]
            },
            if mandatory {
                DecisionContinuation::CastSuspended { player, card }
            } else {
                DecisionContinuation::MayCastExiled {
                    player,
                    card,
                    object: Box::new(object.clone()),
                    context: context.clone(),
                    definition: scoped,
                }
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
        // The zone is the offering clause's business rather than this
        // machinery's: a graveyard for the clauses that buy a spell back,
        // exile for rebound's own card waiting there. What both have in
        // common is that the card is somewhere a cast can reach it.
        let Some((zone @ (ZoneKind::Graveyard | ZoneKind::Exile), instance)) =
            self.card_in_nonbattlefield_zone(card)
        else {
            return;
        };
        let source_zone = match zone {
            ZoneKind::Exile => CastSourceZone::Exile,
            _ => CastSourceZone::Graveyard,
        };
        let decision_zone = match zone {
            ZoneKind::Exile => DecisionZone::Exile,
            _ => DecisionZone::Graveyard,
        };
        let printed = instance.definition;
        let name = self
            .catalog
            .get(printed)
            .map_or_else(|| "that card".to_owned(), |card| card.name.clone());
        let grant = NonbattlefieldAbilityGrant {
            object: card,
            ability: *ability,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            source: None,
        };
        let grant_index = self.nonbattlefield_ability_grants.len();
        self.nonbattlefield_ability_grants.push(grant);
        let DeclarativeAbilityDef::AlternativeCast(_) = ability.definition else {
            self.revoke_temporary_grant(grant_index, card, ability);
            return;
        };
        let mut castable = Vec::new();
        self.add_offered_cast_actions(
            CastOffer {
                player,
                card,
                source_zone,
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
                zone: decision_zone,
            }],
            DecisionContinuation::MayCastGranted {
                player,
                card,
                ability: *ability,
                grant: grant_index,
                source_zone,
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
        if alternative.kind != AlternativeCastKindDef::Miracle {
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
            .nonbattlefield_ability_grants
            .get(grant)
            .is_some_and(|candidate| candidate.object == card && candidate.ability == *ability)
        {
            self.nonbattlefield_ability_grants.remove(grant);
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
        self.associate_latest_decision_with(object);
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
include!("decision_offers/effect_payment_resolution.rs");
include!("decision_offers/payment_options.rs");
include!("decision_offers/pay_or.rs");
