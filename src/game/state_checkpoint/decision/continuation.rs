#[allow(clippy::too_many_lines)]
fn parse_continuation(
    value: &DecisionContinuationSnapshot,
    observation: &DecisionObservation,
    hidden: &Value,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    Ok(match value {
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
            replacements,
        } => DecisionContinuation::DrawReplacement {
            player: player(*owner)?,
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
        },
        DecisionContinuationSnapshot::BasicLandTypeTextChange { target } => {
            DecisionContinuation::BasicLandTypeTextChange {
                target: parse_target(*target),
            }
        }
        DecisionContinuationSnapshot::GrislySalvage {
            player: owner,
            revealed,
        } => DecisionContinuation::GrislySalvage {
            player: player(*owner)?,
            revealed: parse_detached_cards(revealed, game)?,
        },
        DecisionContinuationSnapshot::AugurOfBolas {
            player: owner,
            revealed,
        } => DecisionContinuation::AugurOfBolas {
            player: player(*owner)?,
            revealed: parse_detached_cards(revealed, game)?,
        },
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
            // The located effect is the follow-up the chosen name feeds,
            // not the choice itself: the choice is the pending question, and
            // what it continues into is what has to be found again.
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
        DecisionContinuationSnapshot::TopCardSelection {
            player: owner,
            revealed,
            continuation,
        } => {
            let owner = player(*owner)?;
            let continuation = parse_effect_continuation(continuation, game)?;
            let EffectDef::LookAtTopAndSelect {
                player: recipient,
                looker,
                selection,
            } = continuation.effect.effect
            else {
                return Err("top-card selection locator is not a top-card selection".into());
            };
            let resolve = |recipient| {
                game.effect_recipients(
                    recipient,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
            };
            // The library belongs to one player and the decision is shown to
            // another whenever a spy is looking, so the two are checked
            // against the two authored recipients rather than each other.
            if resolve(recipient).as_slice() != [Target::Player(owner)] {
                return Err("top-card selection player disagrees with its authored effect".into());
            }
            if resolve(looker).as_slice() != [Target::Player(observation.player)] {
                return Err("top-card selection looker disagrees with the visible decision".into());
            }
            let revealed = parse_detached_cards(revealed, game)?;
            validate_top_card_selection_observation(
                game,
                observation,
                owner,
                &revealed,
                selection,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            )?;
            DecisionContinuation::TopCardSelection {
                player: owner,
                revealed,
                selection,
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
        } => DecisionContinuation::Fork {
            // The only repaint in the corpus is Fork's own.
            colors: repainted.then_some(super::super::FORK_COPY_COLOR),
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
            if !matches!(definition.effect, EffectDef::ExileTopAndMayCast { .. }) {
                return Err("cast-offer locator does not identify an offered cast".into());
            }
            DecisionContinuation::MayCastExiled {
                player: caster,
                card: GameObjectId(*card),
                object,
                context: parse_effect_resolution_context(context.clone())?,
                definition,
            }
        }
        DecisionContinuationSnapshot::ChooseForEffect {
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("object-choice locator disagrees with its resolving ability".into());
            }
            let EffectDef::Choose(definition) = continuation.effect.effect else {
                return Err("object-choice locator does not identify an authored choice".into());
            };
            let state = game
                .effect_choice_decision_state(
                    definition,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("object-choice authored chooser is not singular")?;
            if definition.minimum > 0 && state.candidates.len() <= definition.minimum {
                return Err(
                    "object-choice checkpoint encodes a choice that would resolve automatically"
                        .into(),
                );
            }
            validate_authored_decision(
                observation,
                state.chooser,
                "Choose objects",
                effect_choice_visibility(definition.visibility),
                state.preference,
                state.minimum,
                state.maximum,
                &state.options,
                "object choice",
            )?;
            DecisionContinuation::ChooseForEffect {
                definition: continuation.effect,
                binding: definition.binding,
                object: continuation.object,
                context: continuation.context,
                candidates: state.candidates,
                effect: continuation.effect.with_effect(*definition.then),
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
        DecisionContinuationSnapshot::SplitForEffect {
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("pile-split locator disagrees with its resolving ability".into());
            }
            let EffectDef::SplitIntoPiles(definition) = continuation.effect.effect else {
                return Err("pile-split locator does not identify an authored partition".into());
            };
            let state = game
                .effect_pile_split_state(
                    definition,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("pile-split authored divider or chooser is not singular")?;
            validate_authored_decision(
                observation,
                state.divider,
                "Separate the objects into two piles",
                DecisionVisibility::Public,
                DecisionPreference::BalancedPartition,
                0,
                state.items.len(),
                &state.options,
                "pile split",
            )?;
            DecisionContinuation::SplitForEffect {
                definition: continuation.effect,
                chooser: state.chooser,
                items: state.items,
                object: continuation.object,
                context: continuation.context,
            }
        }
        DecisionContinuationSnapshot::ChoosePileForEffect {
            first,
            second,
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("pile-choice locator disagrees with its resolving ability".into());
            }
            let EffectDef::SplitIntoPiles(definition) = continuation.effect.effect else {
                return Err("pile-choice locator does not identify an authored partition".into());
            };
            let authored = game
                .effect_pile_split_state(
                    definition,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("pile-choice authored divider or chooser is not singular")?;
            let first = first.iter().copied().map(parse_target).collect::<Vec<_>>();
            let second = second.iter().copied().map(parse_target).collect::<Vec<_>>();
            validate_exact_partition(&authored.items, &first, &second)?;
            let state =
                game.effect_pile_choice_state(&first, &second, definition, continuation.effect);
            validate_authored_decision(
                observation,
                authored.chooser,
                "Choose a pile",
                DecisionVisibility::Public,
                state.preference,
                1,
                1,
                &state.options,
                "pile choice",
            )?;
            DecisionContinuation::ChoosePileForEffect {
                definition: continuation.effect,
                first,
                second,
                chosen: definition.chosen,
                unchosen: definition.unchosen,
                object: continuation.object,
                context: continuation.context,
                effect: continuation.effect.with_effect(*definition.then),
            }
        }
        DecisionContinuationSnapshot::TriggerOrder { batch, remaining } => {
            DecisionContinuation::TriggerOrder {
                batch: parse_trigger_batch(batch, game)?,
                remaining: remaining
                    .iter()
                    .map(|batch| parse_trigger_batch(batch, game))
                    .collect::<Result<Vec<_>, _>>()?,
            }
        }
        DecisionContinuationSnapshot::TriggerPlacement {
            trigger,
            pending,
            remaining,
            candidates,
        } => DecisionContinuation::TriggerPlacement {
            trigger: parse_pending_trigger(trigger, game)?,
            pending: pending
                .iter()
                .map(|trigger| parse_pending_trigger(trigger, game))
                .collect::<Result<Vec<_>, _>>()?,
            remaining: remaining
                .iter()
                .map(|batch| parse_trigger_batch(batch, game))
                .collect::<Result<Vec<_>, _>>()?,
            candidates: candidates.iter().copied().map(parse_target).collect(),
        },
        DecisionContinuationSnapshot::TriggerMode {
            trigger,
            pending,
            remaining,
        } => {
            let trigger = parse_pending_trigger(trigger, game)?;
            let modes = trigger
                .modes
                .ok_or("trigger-mode decision names a trigger that prints no modes")?;
            DecisionContinuation::TriggerMode {
                trigger,
                pending: pending
                    .iter()
                    .map(|trigger| parse_pending_trigger(trigger, game))
                    .collect::<Result<Vec<_>, _>>()?,
                remaining: remaining
                    .iter()
                    .map(|batch| parse_trigger_batch(batch, game))
                    .collect::<Result<Vec<_>, _>>()?,
                modes,
            }
        }
        DecisionContinuationSnapshot::TriggerDivision {
            trigger,
            pending,
            remaining,
            targets,
            divisions,
        } => DecisionContinuation::TriggerDivision {
            trigger: parse_pending_trigger(trigger, game)?,
            pending: pending
                .iter()
                .map(|trigger| parse_pending_trigger(trigger, game))
                .collect::<Result<Vec<_>, _>>()?,
            remaining: remaining
                .iter()
                .map(|batch| parse_trigger_batch(batch, game))
                .collect::<Result<Vec<_>, _>>()?,
            targets: targets.iter().copied().map(parse_target).collect(),
            divisions: divisions.clone(),
        },
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
        DecisionContinuationSnapshot::SeparateIntoPiles {
            resolving_controller,
            subject,
            items,
            on_complete,
        } => DecisionContinuation::SeparateIntoPiles {
            resolving_controller: player(*resolving_controller)?,
            subject: player(*subject)?,
            items: items
                .iter()
                .map(|option| parse_decision_option_snapshot(&game.catalog, option))
                .collect::<Result<Vec<_>, String>>()?,
            on_complete: crate::card::sets::piles_separated_resolver(on_complete)
                .ok_or("unknown piles-separated resolver")?,
        },
        DecisionContinuationSnapshot::ChoosePile { piles, on_complete } => {
            DecisionContinuation::ChoosePile {
                piles: parse_pile_split_snapshot(piles, &game.catalog)?,
                on_complete: crate::card::sets::pile_chosen_resolver(on_complete)
                    .ok_or("unknown pile-chosen resolver")?,
            }
        }
        DecisionContinuationSnapshot::ChooseColor {
            continuation,
            targets,
        } => {
            let followup = parse_effect_continuation(continuation, game)?;
            // The operation and the duration live on the effect itself,
            // which the locator already found; storing them again would be
            // two places for one fact to disagree.
            let EffectDef::ChooseColor {
                operation,
                duration,
                ..
            } = followup.effect.effect
            else {
                return Err("a color choice located a different effect".to_owned());
            };
            DecisionContinuation::ChooseColor {
                object: followup.object,
                context: followup.context,
                scoped: followup.effect,
                targets: targets.iter().copied().map(parse_target).collect(),
                operation,
                duration,
            }
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
        DecisionContinuationSnapshot::RecallDiscard { player: owner } => {
            DecisionContinuation::RecallDiscard {
                player: player(*owner)?,
            }
        }
        DecisionContinuationSnapshot::RecallReturn { player: owner } => {
            DecisionContinuation::RecallReturn {
                player: player(*owner)?,
            }
        }
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
        DecisionContinuationSnapshot::TetravusDetach { source } => {
            DecisionContinuation::TetravusDetach { source: GameObjectId(*source) }
        }
        DecisionContinuationSnapshot::TetravusAssemble { source } => {
            DecisionContinuation::TetravusAssemble {
                source: GameObjectId(*source),
            }
        }
        // What a prospective battlefield entry can suspend on is a family of
        // its own, and reads next door.
        entry => parse_battlefield_entry_continuation(entry, observation, hidden, game)?,
    })
}
