include!("pregame_continuation.rs");
include!("counter_choice_continuation.rs");
include!("trigger_continuation.rs");
include!("object_collection_continuation.rs");

#[allow(clippy::too_many_lines)]
fn parse_continuation(
    value: &DecisionContinuationSnapshot,
    observation: &DecisionObservation,
    hidden: &Value,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    Ok(match value {
        pregame @ (DecisionContinuationSnapshot::PregameActions { .. }
        | DecisionContinuationSnapshot::ScryBottom { .. }
        | DecisionContinuationSnapshot::ScryTop { .. }) => {
            parse_pregame_continuation(pregame, game)?
        }
        DecisionContinuationSnapshot::ArrivingAttackerDefender {
            player: chooser,
            defending,
            attackers,
        } => DecisionContinuation::ArrivingAttackerDefender {
            player: player(*chooser)?,
            defending: player(*defending)?,
            attackers: attackers.iter().copied().map(GameObjectId).collect(),
        },
        DecisionContinuationSnapshot::BeginTurn {
            player: prospective_player,
            turn_kind,
            applied,
            replacements,
            deferred,
        } => DecisionContinuation::BeginTurn {
            player: player(*prospective_player)?,
            kind: parse_turn_kind(*turn_kind),
            applied: applied.iter().copied().map(parse_ability_source).collect(),
            replacements: replacements
                .iter()
                .map(|replacement| parse_begin_turn_replacement(replacement, game))
                .collect::<Result<Vec<_>, _>>()?,
            deferred: deferred
                .iter()
                .map(|effect| parse_deferred_begin_turn_effect(effect, game))
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::SearchZone {
            controller,
            source,
            destination,
            placement,
            reveal,
            shuffle,
            enters_tapped,
            attached_player,
            binding,
            follow_up,
        } => DecisionContinuation::SearchZone {
            controller: player(*controller)?,
            source: parse_zone_kind(*source),
            destination: parse_zone_kind(*destination),
            placement: parse_zone_placement(*placement),
            reveal: *reveal,
            shuffle: *shuffle,
            enters_tapped: *enters_tapped,
            attached_player: attached_player.map(player).transpose()?,
            binding: binding
                .map(|index| {
                    u8::try_from(index)
                        .ok()
                        .filter(|index| {
                            usize::from(*index) < crate::ids::ObjectSetBindingIndex::COUNT
                        })
                        .map(crate::ids::ObjectSetBindingIndex::new)
                        .ok_or("search binding is out of range")
                })
                .transpose()?,
            follow_up: follow_up
                .as_ref()
                .map(|snapshot| {
                    let continuation = parse_effect_continuation(snapshot, game)?;
                    Ok::<_, String>(Box::new(super::super::SearchFollowUp {
                        object: *continuation.object,
                        context: continuation.context,
                        effect: continuation.effect,
                    }))
                })
                .transpose()?,
        },
        DecisionContinuationSnapshot::ChosenColorMana {
            controller,
            prototype,
            remaining,
            choosable,
        } => DecisionContinuation::ChosenColorMana {
            controller: player(*controller)?,
            prototype: crate::game::state_checkpoint::wire::parse_mana(
                std::slice::from_ref(prototype),
                &game.catalog,
            )?
            .into_iter()
            .next()
            .ok_or("a chosen-colour mana prototype is required")?,
            remaining: *remaining,
            choosable: crate::game::state_checkpoint::stack::color_set_from_flags(*choosable),
        },
        DecisionContinuationSnapshot::ChooseCards {
            controller,
            destination,
            placement,
            reveal,
            arrival,
        } => DecisionContinuation::ChooseCards {
            controller: player(*controller)?,
            destination: parse_zone_kind(*destination),
            placement: parse_zone_placement(*placement),
            reveal: *reveal,
            arrival: arrival
                .as_ref()
                .map(|snapshot| {
                    let continuation = parse_effect_continuation(snapshot, game)?;
                    Ok::<_, String>(Box::new(super::super::SearchFollowUp {
                        object: *continuation.object,
                        context: continuation.context,
                        effect: continuation.effect,
                    }))
                })
                .transpose()?,
        },
        DecisionContinuationSnapshot::DrawReplacement {
            player: owner,
            applied,
            replacements,
        } => DecisionContinuation::DrawReplacement {
            player: player(*owner)?,
            applied: applied.iter().copied().map(parse_ability_source).collect(),
            replacements: replacements
                .iter()
                .map(|replacement| parse_draw_replacement(replacement, game))
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::DiscardForEffect {
            player: current,
            amount,
            remaining,
            chosen,
            cause,
            follow_up,
        } => DecisionContinuation::DiscardForEffect {
            player: player(*current)?,
            amount: *amount,
            remaining: remaining
                .iter()
                .copied()
                .map(player)
                .collect::<Result<Vec<_>, _>>()?,
            chosen: chosen
                .iter()
                .map(|choice| {
                    let owner = player(choice.player)?;
                    let cards = match &choice.cards {
                        Some(cards) => game_ids(cards),
                        None => hidden_discard_choices(hidden, owner, choice.count, game)?,
                    };
                    Ok((owner, cards))
                })
                .collect::<Result<Vec<_>, String>>()?,
            cause: parse_cause(*cause)?,
            follow_up: follow_up
                .as_ref()
                .map(|snapshot| {
                    let continuation = parse_effect_continuation(snapshot, game)?;
                    let crate::card::EffectDef::Discard {
                        then: Some(definition),
                        ..
                    } = continuation.effect.effect
                    else {
                        return Err("discard follow-up locator does not identify a discard".into());
                    };
                    Ok::<_, String>(Box::new(crate::game::decision_state::DiscardFollowUp {
                        counted: definition.counted,
                        bound: definition.bound,
                        definition: continuation.effect,
                        effect: continuation.effect.with_effect(*definition.effect),
                        object: continuation.object,
                        context: continuation.context,
                    }))
                })
                .transpose()?,
        },
        DecisionContinuationSnapshot::BasicLandTypeTextChange { target } => {
            DecisionContinuation::BasicLandTypeTextChange {
                target: parse_target(*target),
            }
        }
        DecisionContinuationSnapshot::SacrificeToTotalPower {
            player: payer,
            remaining,
            object,
            context,
            if_paid,
        } => DecisionContinuation::SacrificeToTotalPower {
            player: player(*payer)?,
            remaining: *remaining,
            object: Box::new(parse_detached_stack(object, game)?),
            context: parse_effect_resolution_context(context.clone())?,
            if_paid: match if_paid {
                Some(snapshot) => Some(parse_effect_continuation(snapshot, game)?.effect),
                None => None,
            },
        },
        DecisionContinuationSnapshot::CardNameChoice {
            choices,
            searched,
            zone,
            binding,
            continuation,
        } => {
            // Locate the follow-up the chosen name feeds, not the choice itself.
            let continuation = parse_effect_continuation(continuation, game)?;
            DecisionContinuation::CardNameChoice {
                choices: choices.clone(),
                searched: player(*searched)?,
                zone: parse_zone_kind(*zone),
                binding: u8::try_from(*binding)
                    .ok()
                    .filter(|index| usize::from(*index) < crate::ids::ObjectSetBindingIndex::COUNT)
                    .map(crate::ids::ObjectSetBindingIndex::new)
                    .ok_or("card-name choice binding is out of range")?,
                object: continuation.object,
                context: continuation.context,
                effect: continuation.effect,
            }
        }
        DecisionContinuationSnapshot::ChainLightning {
            player: owner,
            spell,
            targets,
        } => DecisionContinuation::ChainLightning {
            player: player(*owner)?,
            spell: parse_detached_stack(spell, game)?,
            targets: targets.iter().copied().map(parse_target).collect(),
        },
        DecisionContinuationSnapshot::Fork {
            player: owner,
            spell,
            target_lists,
            repainted,
            remaining,
        } => DecisionContinuation::CopyStackObject {
            // The v1 checkpoint recorded only whether the copy was repainted;
            // its sole representable override was red.
            colors: repainted.then_some(crate::card::ColorSet::from_colors(&[
                crate::card::ManaColor::Red,
            ])),
            remaining: *remaining,
            player: player(*owner)?,
            spell: parse_detached_stack(spell, game)?,
            target_lists: target_lists
                .iter()
                .map(|targets| {
                    targets
                        .iter()
                        .map(parse_target_selection)
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::ChangeStackTargets {
            object,
            target_lists,
        } => DecisionContinuation::ChangeStackTargets {
            object: GameObjectId(*object),
            target_lists: target_lists
                .iter()
                .map(|targets| {
                    targets
                        .iter()
                        .map(parse_target_selection)
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::Endure {
            player: enduring_player,
            permanent,
            amount,
        } => DecisionContinuation::Endure {
            player: player(*enduring_player)?,
            permanent: GameObjectId(*permanent),
            amount: *amount,
        },
        DecisionContinuationSnapshot::OptionalEffect {
            object,
            ability,
            context,
            effect,
        } => DecisionContinuation::OptionalEffect {
            object: Box::new(parse_detached_stack(object, game)?),
            context: parse_effect_resolution_context(context.clone())?,
            effect: catalog_scoped_effect(&game.catalog, ability, effect)
                .ok_or("optional effect locator is absent from this catalog")?,
        },
        DecisionContinuationSnapshot::MayCastExiled {
            player: caster,
            card,
            object,
            ability,
            context,
            definition,
        } => {
            let caster = player(*caster)?;
            if caster != observation.player {
                return Err("cast offer names a player other than the deciding one".into());
            }
            let object = Box::new(parse_detached_stack(object, game)?);
            if !ability_locator_matches_origin(ability, &object) {
                return Err("cast-offer locator disagrees with its resolving ability".into());
            }
            let definition = catalog_scoped_effect(&game.catalog, ability, definition)
                .ok_or("cast-offer locator is absent from this catalog")?;
            match definition.effect {
                EffectDef::ExileTopAndMayCast { .. }
                | EffectDef::MayPlayWithoutPaying(crate::card::FreePlayDef {
                    mandatory: false,
                    ..
                }) => {}
                _ => return Err("cast-offer locator does not identify an offered cast".into()),
            }
            DecisionContinuation::MayCastExiled {
                player: caster,
                card: GameObjectId(*card),
                object,
                context: parse_effect_resolution_context(context.clone())?,
                definition,
            }
        }
        DecisionContinuationSnapshot::CastSuspended {
            player: caster,
            card,
        } => {
            let caster = player(*caster)?;
            if caster != observation.player {
                return Err("Suspend cast names a player other than the deciding one".into());
            }
            DecisionContinuation::CastSuspended {
                player: caster,
                card: GameObjectId(*card),
            }
        }
        DecisionContinuationSnapshot::ChooseForEffect {
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("object-choice locator disagrees with its resolving ability".into());
            }
            let (state, binding, then, prompt, visibility) = match continuation.effect.effect {
                EffectDef::Choose(definition) => {
                    let state = game
                        .effect_choice_decision_state(
                            definition,
                            &continuation.object,
                            &continuation.context,
                            continuation.effect,
                        )
                        .ok_or("object-choice authored chooser is not singular")?;
                    if super::super::decision_permanent_choice::effect_choice_resolves_automatically(
                        definition,
                        state.candidates.len(),
                    ) {
                        return Err(
                            "object-choice checkpoint encodes a choice that would resolve automatically"
                                .into(),
                        );
                    }
                    (
                        state,
                        definition.binding,
                        definition.then,
                        super::super::decision_permanent_choice::effect_choice_prompt(
                            *definition.then,
                            definition.binding,
                        ),
                        effect_choice_visibility(definition.visibility),
                    )
                }
                EffectDef::ChooseExact(definition) => {
                    let (fixed, state) = game
                        .exact_effect_choice_decision_state(
                            definition,
                            &continuation.object,
                            &continuation.context,
                            continuation.effect,
                        )
                        .ok_or("object-choice authored chooser is not singular")?;
                    if super::super::decision_permanent_choice::effect_choice_resolves_automatically(
                        fixed,
                        state.candidates.len(),
                    ) {
                        return Err(
                            "object-choice checkpoint encodes a choice that would resolve automatically"
                                .into(),
                        );
                    }
                    let binding = crate::card::ObjectChoiceBindingDef::Objects(definition.binding);
                    (
                        state,
                        binding,
                        definition.then,
                        super::super::decision_permanent_choice::effect_choice_prompt(
                            *definition.then,
                            binding,
                        ),
                        effect_choice_visibility(definition.visibility),
                    )
                }
                EffectDef::ChooseCardsFromCollection(definition) => {
                    let state = game
                        .collection_card_choice_decision_state(
                            definition,
                            &continuation.object,
                            &continuation.context,
                            continuation.effect,
                        )
                        .ok_or("collection choice authored actor is not singular")?;
                    let binding = crate::card::ObjectChoiceBindingDef::Objects(definition.chosen);
                    let prompt = if state.candidates.is_empty() {
                        "Continue"
                    } else {
                        super::super::decision_permanent_choice::effect_choice_prompt(
                            *definition.then,
                            binding,
                        )
                    };
                    let visibility = match definition.inspection {
                        crate::card::CollectionInspectionDef::Look => DecisionVisibility::Private,
                        crate::card::CollectionInspectionDef::Reveal => DecisionVisibility::Public,
                    };
                    (state, binding, definition.then, prompt, visibility)
                }
                _ => {
                    return Err(
                        "object-choice locator does not identify an authored choice".into(),
                    );
                }
            };
            validate_authored_decision(
                observation,
                state.chooser,
                prompt,
                visibility,
                state.preference,
                state.minimum,
                state.maximum,
                &state.options,
                "object choice",
            )?;
            DecisionContinuation::ChooseForEffect {
                definition: continuation.effect,
                binding,
                object: continuation.object,
                context: continuation.context,
                candidates: state.candidates,
                effect: continuation.effect.with_effect(*then),
            }
        }
        group @ (DecisionContinuationSnapshot::ChooseObjectOrderForEffect { .. }
        | DecisionContinuationSnapshot::LookAtObjectsForEffect { .. }
        | DecisionContinuationSnapshot::PartitionGroupForEffect { .. }) => {
            parse_basic_object_collection_continuation(group, observation, game)?
        }
        DecisionContinuationSnapshot::ChooseGroupForEffect {
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("group-choice locator disagrees with its resolving ability".into());
            }
            let EffectDef::ChooseGroup(definition) = continuation.effect.effect else {
                return Err("group-choice locator does not identify a group choice".into());
            };
            let actor = game
                .effect_player_reference(
                    definition.actor,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("group-choice actor is not singular")?;
            let first = game.effect_objects(
                definition.first,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            );
            let second = game.effect_objects(
                definition.second,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            );
            if first.is_empty() && second.is_empty() {
                return Err("group-choice checkpoint encodes two empty groups".into());
            }
            let options = [first.as_slice(), second.as_slice()]
                .into_iter()
                .enumerate()
                .map(|(index, group)| {
                    let names = group
                        .iter()
                        .copied()
                        .map(|target| game.effect_target_option(0, target).label)
                        .collect::<Vec<_>>();
                    DecisionOption {
                        id: u32::try_from(index).unwrap_or(u32::MAX),
                        label: format!(
                            "Choose pile {} ({})",
                            index + 1,
                            if names.is_empty() {
                                "empty".into()
                            } else {
                                names.join(", ")
                            }
                        ),
                        card: None,
                        members: group
                            .iter()
                            .copied()
                            .filter_map(|target| game.effect_target_card(target))
                            .collect(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    }
                })
                .collect::<Vec<_>>();
            let effect = continuation.effect.with_effect(*definition.then);
            let preference = if crate::game::decision_permanent_choice::effect_moves_group_to_hand(
                effect.effect,
                definition.chosen,
            ) {
                DecisionPreference::HigherCardValue
            } else if crate::game::decision_permanent_choice::effect_sacrifices_group(
                effect.effect,
                definition.chosen,
            ) {
                DecisionPreference::LowerCardValue
            } else {
                DecisionPreference::Neutral
            };
            validate_authored_decision(
                observation,
                actor,
                "Choose a pile",
                effect_choice_visibility(definition.visibility),
                preference,
                1,
                1,
                &options,
                "group choice",
            )?;
            DecisionContinuation::ChooseGroupForEffect {
                definition: continuation.effect,
                first,
                second,
                object: continuation.object,
                context: continuation.context,
                effect: continuation.effect.with_effect(*definition.then),
            }
        }
        DecisionContinuationSnapshot::ChooseOneOfEachForEffect {
            continuation: snapshot,
            next,
            remaining,
            chosen,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("one-of-each locator disagrees with its resolving ability".into());
            }
            let EffectDef::ChooseOneOfEach(definition) = continuation.effect.effect else {
                return Err("one-of-each locator does not identify an authored choice".into());
            };
            let remaining = remaining.iter().copied().map(parse_target).collect::<Vec<_>>();
            let chosen = chosen.iter().copied().map(parse_target).collect::<Vec<_>>();
            let authored = game.effect_objects(
                definition.input,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            );
            let combined = remaining
                .iter()
                .chain(&chosen)
                .copied()
                .collect::<Vec<_>>();
            if combined.len() != authored.len()
                || combined
                    .iter()
                    .enumerate()
                    .any(|(index, item)| combined[..index].contains(item))
                || authored.iter().any(|item| !combined.contains(item))
                || remaining
                    != authored
                        .iter()
                        .filter(|item| remaining.contains(item))
                        .copied()
                        .collect::<Vec<_>>()
            {
                return Err("one-of-each progress is not a partition of its authored group".into());
            }
            let predicate = definition
                .predicates
                .get(*next)
                .copied()
                .ok_or("one-of-each progress is past its last predicate")?;
            let source = continuation.object.source.unwrap_or(continuation.object.id);
            let candidates = remaining
                .iter()
                .copied()
                .filter(|target| game.bound_object_matches(*target, predicate, source))
                .collect::<Vec<_>>();
            if candidates.is_empty() {
                return Err("one-of-each progress stopped at a predicate with no candidates".into());
            }
            let actor = game
                .effect_player_reference(
                    definition.actor,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("one-of-each actor is not singular")?;
            let options = candidates
                .iter()
                .copied()
                .enumerate()
                .map(|(index, target)| game.effect_target_option(index, target))
                .collect::<Vec<_>>();
            validate_authored_decision(
                observation,
                actor,
                &Game::one_of_each_prompt(predicate),
                effect_choice_visibility(definition.visibility),
                DecisionPreference::HigherCardValue,
                0,
                1,
                &options,
                "one-of-each choice",
            )?;
            DecisionContinuation::ChooseOneOfEachForEffect {
                definition: continuation.effect,
                next: *next,
                candidates,
                remaining,
                chosen,
                object: continuation.object,
                context: continuation.context,
            }
        }
        DecisionContinuationSnapshot::SimultaneousChoose {
            continuation: snapshot,
            task,
            players,
            chosen,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err(
                    "simultaneous-choice locator disagrees with its resolving ability".into(),
                );
            }
            let EffectDef::SimultaneousChoose(definition) = continuation.effect.effect else {
                return Err(
                    "simultaneous-choice locator does not identify an authored choice".into(),
                );
            };
            let players = players
                .iter()
                .copied()
                .map(player)
                .collect::<Result<Vec<_>, _>>()?;
            let expected = game.simultaneous_choice_players(
                definition,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            );
            if players != expected {
                return Err("simultaneous-choice players disagree with the authored choice".into());
            }
            let chosen = chosen.iter().copied().map(GameObjectId).collect::<Vec<_>>();
            let state = game
                .simultaneous_choice_decision_state(
                    definition,
                    *task,
                    &players,
                    &chosen,
                    &continuation.object,
                )
                .ok_or("simultaneous-choice task is out of range")?;
            if state.candidates.len() <= 1 {
                return Err(
                    "simultaneous-choice checkpoint encodes an automatic choice".into(),
                );
            }
            validate_authored_decision(
                observation,
                state.chooser,
                "Choose a permanent",
                DecisionVisibility::Public,
                state.preference,
                1,
                1,
                &state.options,
                "simultaneous choice",
            )?;
            DecisionContinuation::SimultaneousChoose {
                definition: continuation.effect,
                task: *task,
                players,
                chosen,
                object: continuation.object,
                context: continuation.context,
                candidates: state.candidates,
            }
        }
        DecisionContinuationSnapshot::PayOr {
            player: payer,
            payment: payment_snapshot,
            object,
            ability,
            context,
            definition,
        } => {
            let payer = player(*payer)?;
            if payer != observation.player {
                return Err("pay-or payer disagrees with the visible decision".into());
            }
            let object = Box::new(parse_detached_stack(object, game)?);
            let context = parse_effect_resolution_context(context.clone())?;
            if !ability_locator_matches_origin(ability, &object) {
                return Err("pay-or locator disagrees with its resolving ability".into());
            }
            let scoped = catalog_scoped_effect(&game.catalog, ability, definition)
                .ok_or("pay-or locator is absent from this catalog")?;
            let EffectDef::PayOr(authored) = scoped.effect else {
                return Err("pay-or locator does not identify an optional payment".into());
            };
            // The payer was settled when the decision was queued, and the
            // state it was read from can have moved since: Chain of Vapor
            // asks the controller of a permanent it has already returned to
            // hand. So a payer that can no longer be derived is the recorded
            // one, while a payer that derives to somebody else is a
            // disagreement worth refusing.
            let authored_payment =
                resolved_effect_payment(game, authored.payment, &object, &context, scoped);
            let payment = match authored_payment {
                Some((expected_payer, payment)) if expected_payer == payer => payment,
                Some(_) => {
                    return Err("pay-or payer or payment disagrees with its authored effect".into());
                }
                None => resolved_effect_payment_for_payer(
                    game,
                    authored.payment,
                    &object,
                    &context,
                    scoped,
                )
                .ok_or("pay-or authored payment cannot be rebuilt")?,
            };
            // Compared as snapshots, and kept as the authored value: a
            // payment that names a predicate cannot be rebuilt from the
            // checkpoint alone, and the authored effect is what defines it.
            if resolved_effect_payment_snapshot(payment) != *payment_snapshot {
                return Err("pay-or payer or payment disagrees with its authored effect".into());
            }
            let can_pay = game.can_pay_effect_payment(payer, payment);
            if authored.if_paid.is_none() && authored.otherwise.is_none()
                || (!can_pay && authored.otherwise.is_some())
            {
                return Err(
                    "pay-or checkpoint encodes a choice that would resolve automatically".into(),
                );
            }
            let options = payment_decision_options(game, payer, payment, can_pay, "Decline");
            validate_authored_decision(
                observation,
                payer,
                object.ability_text().unwrap_or("Pay the cost?"),
                effect_choice_visibility(authored.visibility),
                DecisionPreference::Neutral,
                1,
                1,
                &options,
                "pay-or",
            )?;
            DecisionContinuation::PayOr {
                player: payer,
                payment,
                definition: scoped,
                object,
                context,
                if_paid: authored.if_paid.map(|effect| scoped.with_effect(*effect)),
                otherwise: authored.otherwise.map(|effect| scoped.with_effect(*effect)),
            }
        }
        triggers @ (DecisionContinuationSnapshot::TriggerOrder { .. }
        | DecisionContinuationSnapshot::TriggerPlacement { .. }
        | DecisionContinuationSnapshot::TriggerMode { .. }
        | DecisionContinuationSnapshot::TriggerDivision { .. }) => {
            parse_trigger_continuation(triggers, game)?
        }
        DecisionContinuationSnapshot::DrawActionWindow { card } => {
            parse_draw_action_window_continuation(game, observation, GameObjectId(*card))?
        }
        DecisionContinuationSnapshot::ExploredCardPlacement {
            player: seat,
            revealed,
        } => parse_explored_card_placement(
            observation,
            *seat,
            GameObjectId(*revealed),
        )?,
        DecisionContinuationSnapshot::Proliferate { candidates } => {
            // Rebuilt rather than trusted: what a proliferate could add to
            // is a fact of the board, so a checkpoint naming anything else
            // is not this game.
            let restored = candidates
                .iter()
                .copied()
                .map(parse_target)
                .collect::<Vec<_>>();
            if restored != game.proliferate_candidates() {
                return Err("proliferate candidates disagree with the board".into());
            }
            DecisionContinuation::Proliferate {
                candidates: restored,
            }
        }
        DecisionContinuationSnapshot::MayCastGranted {
            player: seat,
            card,
            ability,
            grant,
        } => parse_may_cast_granted_continuation(
            game,
            observation,
            *seat,
            GameObjectId(*card),
            ability,
            *grant,
        )?,
        DecisionContinuationSnapshot::MayCastAlternative {
            player: seat,
            card,
            ability,
        } => parse_may_cast_alternative_continuation(
            game,
            observation,
            *seat,
            GameObjectId(*card),
            ability_origin_from_snapshot(*ability),
        )?,
        DecisionContinuationSnapshot::CascadeCast {
            player: seat,
            card,
            exiled,
        } => {
            let seat = player(*seat)?;
            if seat != observation.player {
                return Err("a cascade offer names a player other than the deciding one".into());
            }
            DecisionContinuation::CascadeCast {
                player: seat,
                card: GameObjectId(*card),
                exiled: exiled.iter().copied().map(GameObjectId).collect(),
            }
        }
        DecisionContinuationSnapshot::SpellLibraryEnd { owner, spell } => {
            let owner = player(*owner)?;
            if owner != observation.player {
                return Err(
                    "a library-end choice names a player other than the deciding one".into(),
                );
            }
            DecisionContinuation::SpellLibraryEnd {
                owner,
                spell: GameObjectId(*spell),
            }
        }
        DecisionContinuationSnapshot::ChooseColor {
            continuation,
            targets,
        } => parse_choose_color_continuation(continuation, targets, game)?,
        DecisionContinuationSnapshot::ChooseCounter {
            continuation,
            target,
            kinds,
        } => parse_choose_counter_continuation(continuation, *target, kinds, game)?,
        DecisionContinuationSnapshot::ChooseEffect { continuation } => {
            parse_choose_effect_continuation(continuation, game)?
        }
        DecisionContinuationSnapshot::SacrificeOfChoice {
            followup,
            declined,
            optional,
        } => DecisionContinuation::SacrificeOfChoice {
            followup: followup
                .as_ref()
                .map(|followup| parse_effect_continuation(followup, game))
                .transpose()?,
            declined: declined
                .as_ref()
                .map(|declined| parse_sacrifice_declined(declined, game))
                .transpose()?,
            optional: *optional,
        },
        DecisionContinuationSnapshot::Balance {
            controller,
            phase,
            task,
            remaining,
        } => DecisionContinuation::Balance {
            controller: player(*controller)?,
            phase: parse_balance_phase(*phase),
            task: parse_balance_task(task, game)?,
            remaining: remaining
                .iter()
                .map(|task| parse_balance_task(task, game))
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::SearchZonesAndExileRest {
            player: owner,
            zones,
            searched,
        } => DecisionContinuation::SearchZonesAndExileRest {
            player: player(*owner)?,
            zones: zones.iter().copied().map(parse_zone_kind).collect(),
            searched: game_ids(searched),
        },
        DecisionContinuationSnapshot::Vote {
            candidates,
            remaining,
            votes,
        } => DecisionContinuation::Vote {
            candidates: game_ids(candidates),
            remaining: remaining
                .iter()
                .map(|owner| player(*owner))
                .collect::<Result<Vec<_>, _>>()?,
            votes: game_ids(votes),
        },
        // Prospective battlefield-entry continuations read next door.
        entry => parse_battlefield_entry_continuation(entry, observation, hidden, game)?,
    })
}
