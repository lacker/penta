/// The payment an authored cost describes, without asking who pays it. The
/// payer is settled when the decision is queued; this is what the payment
/// itself is, for a checkpoint restoring one whose payer has since moved.
pub(super) fn resolved_effect_payment_for_payer(
    game: &Game,
    payment: EffectPaymentDef,
    object: &super::super::StackObject,
    context: &super::super::EffectResolutionContext,
    scoped: ScopedEffect,
) -> Option<super::super::ResolvedEffectPayment> {
    let mut payment = payment;
    payment.payer = crate::card::PlayerSetDef::One(crate::card::PlayerRefDef::EffectController);
    resolved_effect_payment(game, payment, object, context, scoped).map(|(_, payment)| payment)
}

fn resolved_effect_payment(
    game: &Game,
    payment: EffectPaymentDef,
    object: &super::super::StackObject,
    context: &super::super::EffectResolutionContext,
    scoped: ScopedEffect,
) -> Option<(PlayerId, super::super::ResolvedEffectPayment)> {
    let payers = game.effect_players(payment.payer, object, context, scoped);
    let [player] = payers.as_slice() else {
        return None;
    };
    let payment = match payment.cost {
        EffectPaymentCostDef::Mana(cost) => super::super::ResolvedEffectPayment::Mana(cost),
        EffectPaymentCostDef::ObjectManaCostReducedBy {
            object: reference,
            generic,
        } => super::super::ResolvedEffectPayment::Mana(
            game.object_mana_cost_reduced_by(reference, generic, object, context, scoped),
        ),
        EffectPaymentCostDef::GenericMana(amount) => {
            let amount = game
                .effect_value(amount, object, context, scoped)
                .max(0)
                .try_into()
                .unwrap_or(u16::MAX);
            super::super::ResolvedEffectPayment::Mana(ManaCost::new(amount, 0))
        }
        EffectPaymentCostDef::ColoredMana { color, amount } => {
            let amount = game
                .effect_value(amount, object, context, scoped)
                .max(0)
                .try_into()
                .unwrap_or(u16::MAX);
            super::super::ResolvedEffectPayment::Mana(ManaCost::of_color(color, amount))
        }
        EffectPaymentCostDef::Life(amount) => super::super::ResolvedEffectPayment::Life(amount),
        EffectPaymentCostDef::Energy(amount) => super::super::ResolvedEffectPayment::Energy(amount),
        EffectPaymentCostDef::Mill(amount) => super::super::ResolvedEffectPayment::Mill(amount),
        EffectPaymentCostDef::Discard(amount) => {
            super::super::ResolvedEffectPayment::Discard(amount)
        }
        EffectPaymentCostDef::ChosenGenericMana => {
            super::super::ResolvedEffectPayment::ChosenGenericMana
        }
        EffectPaymentCostDef::ChosenEnergy => super::super::ResolvedEffectPayment::ChosenEnergy,
        EffectPaymentCostDef::RemoveAnyNumberOfCounters {
            object: recipient,
            kind,
        } => game
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .find_map(|target| match target {
                crate::Target::Permanent(object) => Some(
                    super::super::ResolvedEffectPayment::RemoveAnyNumberOfCounters {
                        object,
                        kind,
                    },
                ),
                crate::Target::Player(_)
                | crate::Target::Card(_)
                | crate::Target::Spell(_) => None,
            })
            .unwrap_or(super::super::ResolvedEffectPayment::RemoveAnyNumberOfCounters {
                object: crate::GameObjectId(0),
                kind,
            }),
        EffectPaymentCostDef::SacrificePermanentMatching(predicate) => {
            super::super::ResolvedEffectPayment::SacrificePermanentMatching(predicate)
        }
        EffectPaymentCostDef::SacrificeCreaturesWithTotalPower(total) => {
            super::super::ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total)
        }
        EffectPaymentCostDef::MovePermanentMatching { object, zone } => {
            super::super::ResolvedEffectPayment::MovePermanentMatching { object, zone }
        }
        EffectPaymentCostDef::DiscardMatching(predicate) => {
            super::super::ResolvedEffectPayment::DiscardMatching(predicate)
        }
    };
    Some((*player, payment))
}

/// The same options the live game offers, built by the same code: a payment
/// whose candidates come off the payer's hand cannot be checked against a
/// second implementation of the list.
fn payment_decision_options(
    game: &Game,
    player: PlayerId,
    payment: super::super::ResolvedEffectPayment,
    can_pay: bool,
    decline: &str,
) -> Vec<DecisionOption> {
    game.payment_options(player, payment, can_pay, decline)
}

#[allow(clippy::too_many_arguments)]
fn validate_authored_decision(
    observation: &DecisionObservation,
    player: PlayerId,
    prompt: &str,
    visibility: DecisionVisibility,
    preference: DecisionPreference,
    minimum: usize,
    maximum: usize,
    options: &[DecisionOption],
    description: &str,
) -> Result<(), String> {
    let expected_minimum = minimum.min(options.len());
    let expected_maximum = maximum.max(expected_minimum);
    let mismatch = if observation.player != player {
        "player"
    } else if observation.kind != DecisionKind::Choice || observation.order_semantics.is_some() {
        "kind"
    } else if observation.prompt != prompt {
        "prompt"
    } else if observation.visibility != visibility {
        "visibility"
    } else if observation.preference != preference {
        "preference"
    } else if observation.minimum != expected_minimum || observation.maximum != expected_maximum {
        "bounds"
    } else if observation.cancellable {
        "cancellability"
    } else if observation.options != options {
        return Err(format!(
            "{description} decision options disagree with its authored effect: observed {:?}, expected {options:?}",
            observation.options,
        ));
    } else {
        return Ok(());
    };
    Err(format!(
        "{description} decision {mismatch} disagrees with its authored effect"
    ))
}

#[allow(clippy::too_many_arguments)]
fn validate_ordered_authored_decision(
    observation: &DecisionObservation,
    player: PlayerId,
    prompt: &str,
    visibility: DecisionVisibility,
    options: &[DecisionOption],
    description: &str,
) -> Result<(), String> {
    let count = options.len();
    if observation.player != player
        || observation.kind != DecisionKind::Choice
        || observation.order_semantics != Some(DecisionOrderSemantics::Resolution)
        || observation.prompt != prompt
        || observation.visibility != visibility
        || observation.preference != DecisionPreference::Neutral
        || observation.minimum != count
        || observation.maximum != count
        || observation.cancellable
        || observation.options != options
    {
        return Err(format!(
            "{description} decision disagrees with its authored effect"
        ));
    }
    Ok(())
}

fn first_drawn_card(
    game: &Game,
    player: PlayerId,
    card: GameObjectId,
    description: &str,
) -> Result<CardDefinitionId, String> {
    let held = game.players[player.index()]
        .hand
        .iter()
        .find(|held| held.id == card)
        .ok_or_else(|| format!("{description} card is not in the deciding player's hand"))?;
    if game.drawn_this_turn[player.index()].first().copied() != Some(card)
        || game.cards_drawn_this_turn[player.index()] == 0
    {
        return Err(format!(
            "{description} card is not that player's first drawn card this turn"
        ));
    }
    Ok(held.definition)
}

fn miracle_drawn_card(
    game: &Game,
    player: PlayerId,
    card: GameObjectId,
    description: &str,
) -> Result<CardDefinitionId, String> {
    let definition = first_drawn_card(game, player, card, description)?;
    if !game.has_miracle(definition) {
        return Err(format!(
            "{description} card lacks an executable Miracle alternative"
        ));
    }
    Ok(definition)
}

fn parse_draw_action_window_continuation(
    game: &Game,
    observation: &DecisionObservation,
    card: GameObjectId,
) -> Result<DecisionContinuation, String> {
    let player = observation.player;
    let definition = first_drawn_card(game, player, card, "draw-action window")?;
    let name = game
        .catalog
        .get(definition)
        .ok_or("draw-action window definition is absent from this catalog")?
        .name
        .clone();
    let options = game
        .has_miracle(definition)
        .then(|| DecisionOption {
            id: 1,
            label: format!("Reveal {name}"),
            card: Some((
                card,
                ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
            )),
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::Hand,
        })
        .into_iter()
        .collect::<Vec<_>>();
    validate_authored_decision(
        observation,
        player,
        &format!("Take an action with {name}?"),
        DecisionVisibility::Private,
        DecisionPreference::PreferOption(1),
        0,
        1,
        &options,
        "draw-action window",
    )?;
    Ok(DecisionContinuation::DrawActionWindow { card })
}

fn parse_may_cast_alternative_continuation(
    game: &Game,
    observation: &DecisionObservation,
    seat: usize,
    card: GameObjectId,
    ability: super::super::AbilityOrigin,
) -> Result<DecisionContinuation, String> {
    let player = player(seat)?;
    if player != observation.player {
        return Err("an alternative-cast offer names a player other than the deciding one".into());
    }
    let definition = miracle_drawn_card(game, player, card, "alternative-cast offer")?;
    let linked_ability = game
        .miracle_ability(definition)
        .map(|(origin, _)| origin)
        .ok_or("alternative-cast offer card lacks its linked Miracle ability")?;
    if ability != linked_ability {
        return Err("alternative-cast offer ability is not the card's linked Miracle clause".into());
    }
    let ability_definition = game
        .ability_for_origin(card, ability)
        .ok_or("alternative-cast offer ability is absent from the named card")?;
    let DeclarativeAbilityDef::AlternativeCast(alternative) = ability_definition.definition else {
        return Err("alternative-cast offer ability is not an alternative-cast clause".into());
    };
    if alternative.kind != AlternativeCastKindDef::Miracle {
        return Err("checkpoint alternative-cast offers currently support only Miracle".into());
    }
    let name = game
        .catalog
        .get(definition)
        .ok_or("alternative-cast offer definition is absent from this catalog")?
        .name
        .clone();
    let offer = CastOffer {
        player,
        card,
        source_zone: CastSourceZone::Hand,
        cost: CastOfferCost::PrintedAlternative(ability),
    };
    let mut castable = Vec::new();
    game.add_offered_cast_actions(offer, &mut castable);
    if castable.is_empty() {
        return Err("alternative-cast offer has no legal offered cast".into());
    }
    let options = [DecisionOption {
        id: 0,
        label: "Decline".into(),
        card: Some((
            card,
            ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
        )),
        members: Vec::new(),
        ability_text: None,
        zone: DecisionZone::Hand,
    }];
    validate_authored_decision(
        observation,
        player,
        &format!(
            "Cast {name} for its {} cost, or decline",
            alternative.kind.label()
        ),
        DecisionVisibility::Public,
        DecisionPreference::PreferOption(0),
        1,
        1,
        &options,
        "alternative-cast offer",
    )?;
    Ok(DecisionContinuation::MayCastAlternative {
        player,
        card,
        ability,
    })
}

fn parse_may_cast_granted_continuation(
    game: &Game,
    observation: &DecisionObservation,
    seat: usize,
    card: GameObjectId,
    locator: &AbilityLocator,
    grant: usize,
) -> Result<DecisionContinuation, String> {
    let player = player(seat)?;
    if player != observation.player {
        return Err("a granted-cast offer names a player other than the deciding one".into());
    }
    let ability = catalog_ability(&game.catalog, locator)
        .ok_or("checkpoint granted-cast ability is absent from this catalog")?;
    if !game
        .nonbattlefield_ability_grants
        .get(grant)
        .is_some_and(|candidate| candidate.object == card && candidate.ability == ability)
    {
        return Err("checkpoint granted-cast offer names the wrong temporary grant".into());
    }
    let Some((zone @ (crate::card::ZoneKind::Graveyard | crate::card::ZoneKind::Exile), instance)) =
        game.card_in_nonbattlefield_zone(card)
    else {
        return Err("checkpoint granted-cast offer card is not in a graveyard or in exile".into());
    };
    let source_zone = match zone {
        crate::card::ZoneKind::Exile => CastSourceZone::Exile,
        _ => CastSourceZone::Graveyard,
    };
    let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition else {
        return Err("checkpoint granted-cast offer ability is not an alternative-cast clause".into());
    };
    if !matches!(
            alternative.kind,
            AlternativeCastKindDef::WithoutPayingManaCost | AlternativeCastKindDef::Rebound
        )
    {
        return Err(
            "checkpoint granted-cast offers currently support only executable free casts".into(),
        );
    }
    let offer = CastOffer {
        player,
        card,
        source_zone,
        cost: CastOfferCost::GrantedAlternative(grant),
    };
    let mut castable = Vec::new();
    game.add_offered_cast_actions(offer, &mut castable);
    if castable.is_empty() {
        return Err("granted-cast offer has no legal offered cast".into());
    }
    let name = game
        .catalog
        .get(instance.definition)
        .ok_or("granted-cast offer definition is absent from this catalog")?
        .name
        .clone();
    let options = [DecisionOption {
        id: 0,
        label: "Decline".into(),
        card: Some((
            card,
            ObjectCharacteristics::card(instance.definition, CardPartId::PRIMARY),
        )),
        members: Vec::new(),
        ability_text: None,
        zone: DecisionZone::Graveyard,
    }];
    validate_authored_decision(
        observation,
        player,
        &format!("Cast {name} without paying its mana cost, or decline"),
        DecisionVisibility::Public,
        DecisionPreference::PreferOption(0),
        1,
        1,
        &options,
        "granted-cast offer",
    )?;
    Ok(DecisionContinuation::MayCastGranted {
        player,
        card,
        ability,
        grant,
        source_zone,
    })
}

fn parse_explored_card_placement(
    observation: &DecisionObservation,
    seat: usize,
    revealed: GameObjectId,
) -> Result<DecisionContinuation, String> {
    let player = player(seat)?;
    if player != observation.player {
        return Err("an explore placement names a player other than the deciding one".into());
    }
    Ok(DecisionContinuation::ExploredCardPlacement { player, revealed })
}

fn ability_locator_matches_origin(
    locator: &AbilityLocator,
    object: &super::super::StackObject,
) -> bool {
    let Some(payload) = &object.ability else {
        return false;
    };
    super::semantics::ability_locator_matches_origin(locator, payload.origin)
}

fn validate_entry_decision_context(
    game: &Game,
    context: ReplacementEffectContext,
    locator: &ReplacementEffectLocator,
) -> Result<(), String> {
    if !replacement_effect_locator_matches_source(locator, context.source) {
        return Err("entry decision locator disagrees with its replacement source".into());
    }
    let pending = game
        .pending_events
        .front()
        .ok_or("entry decision lacks its pending event")?;
    if !pending.applied.contains(&context.source)
        || Game::pending_event_controller(pending) != context.controller
    {
        return Err("entry decision context disagrees with its pending event".into());
    }
    Ok(())
}
