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
        EffectPaymentCostDef::SacrificePermanentMatching(predicate) => {
            super::super::ResolvedEffectPayment::SacrificePermanentMatching(predicate)
        }
        EffectPaymentCostDef::SacrificeCreaturesWithTotalPower(total) => {
            super::super::ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total)
        }
        EffectPaymentCostDef::ReturnPermanentMatching(predicate) => {
            super::super::ResolvedEffectPayment::ReturnPermanentMatching(predicate)
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
fn validate_top_card_selection_observation(
    game: &Game,
    observation: &DecisionObservation,
    player: PlayerId,
    revealed: &[super::super::CardInstance],
    selection: &'static crate::card::TopCardSelectionDef,
    object: &super::super::StackObject,
    context: &super::super::EffectResolutionContext,
    scoped: ScopedEffect,
) -> Result<(), String> {
    let requested = game
        .effect_value(selection.count, object, context, scoped)
        .max(0);
    let requested = usize::try_from(requested).unwrap_or(usize::MAX);
    let available_before_inspection = game.players[player.index()]
        .library
        .len()
        .saturating_add(revealed.len());
    if revealed.len() != requested.min(available_before_inspection) {
        return Err("top-card selection inspected count disagrees with its authored effect".into());
    }
    let source = object.source.unwrap_or(object.id);
    let eligible = revealed
        .iter()
        .filter(|card| {
            selection.object.is_none_or(|predicate| {
                game.card_object_matches(predicate, card, crate::card::ZoneKind::Library, source)
            })
        })
        .cloned()
        .collect::<Vec<_>>();
    let inspected = revealed
        .iter()
        .map(|card| {
            (
                card.id,
                ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
            )
        })
        .collect::<Vec<_>>();
    let mut expected = game.card_decision_options(&eligible, DecisionZone::Library);
    for option in &mut expected {
        option.members.clone_from(&inspected);
    }
    let no_selection = expected.is_empty();
    if no_selection {
        expected.push(DecisionOption {
            id: 0,
            label: "No inspected card is eligible".into(),
            card: None,
            members: inspected,
            ability_text: None,
            zone: DecisionZone::Library,
        });
    }
    let (minimum, maximum, preference) = if no_selection {
        (0, 0, DecisionPreference::Neutral)
    } else {
        (
            usize::from(selection.minimum).min(expected.len()),
            usize::from(selection.maximum),
            if selection.selected_zone == crate::card::ZoneKind::Hand {
                DecisionPreference::HigherCardValue
            } else {
                DecisionPreference::LowerCardValue
            },
        )
    };
    validate_authored_decision(
        observation,
        player,
        Game::top_card_selection_prompt(selection),
        DecisionVisibility::Private,
        preference,
        minimum,
        maximum,
        &expected,
        "top-card selection",
    )
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
            card: Some((card, definition)),
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
    if !ability_definition.is_executable() || alternative.kind != AlternativeCastKindDef::Miracle {
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
        card: Some((card, definition)),
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
        .temporary_ability_grants
        .get(grant)
        .is_some_and(|candidate| candidate.object == card && candidate.ability == ability)
    {
        return Err("checkpoint granted-cast offer names the wrong temporary grant".into());
    }
    let Some((crate::card::ZoneKind::Graveyard, instance)) =
        game.card_in_nonbattlefield_zone(card)
    else {
        return Err("checkpoint granted-cast offer card is not in a graveyard".into());
    };
    let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition else {
        return Err("checkpoint granted-cast offer ability is not an alternative-cast clause".into());
    };
    if !ability.is_executable()
        || alternative.kind != AlternativeCastKindDef::WithoutPayingManaCost
    {
        return Err(
            "checkpoint granted-cast offers currently support only executable free casts".into(),
        );
    }
    let offer = CastOffer {
        player,
        card,
        source_zone: CastSourceZone::Graveyard,
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
        card: Some((card, instance.definition)),
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

fn validate_exact_partition(
    authored: &[Target],
    first: &[Target],
    second: &[Target],
) -> Result<(), String> {
    let combined = first.iter().chain(second).copied().collect::<Vec<_>>();
    if combined.len() != authored.len()
        || combined
            .iter()
            .enumerate()
            .any(|(index, item)| combined[..index].contains(item))
        || combined.iter().any(|item| !authored.contains(item))
        || authored.iter().any(|item| !combined.contains(item))
    {
        return Err(
            "pile-choice checkpoint is not an exact disjoint partition of authored items".into(),
        );
    }
    let canonical_first = authored
        .iter()
        .filter(|item| first.contains(item))
        .copied()
        .collect::<Vec<_>>();
    let canonical_second = authored
        .iter()
        .filter(|item| second.contains(item))
        .copied()
        .collect::<Vec<_>>();
    if canonical_first != first || canonical_second != second {
        return Err("pile-choice checkpoint changed the authored item order".into());
    }
    Ok(())
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
