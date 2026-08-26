// The continuations a prospective battlefield entry can suspend on.
//
// Split out of the parent match for the source-size budget, and included
// rather than declared so the imports and helpers around it stay in scope.

#[allow(clippy::too_many_lines)]
fn parse_battlefield_entry_continuation(
    value: &DecisionContinuationSnapshot,
    observation: &DecisionObservation,
    _hidden: &Value,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    Ok(match value {
        DecisionContinuationSnapshot::BattlefieldEntryPayment {
            context,
            player: payer,
            payment: payment_snapshot,
            effect,
        } => {
            let context = parse_replacement_context(*context)?;
            validate_entry_decision_context(game, context, effect)?;
            let definition = catalog_replacement_effect(&game.catalog, effect)
                .ok_or("battlefield entry payment locator is absent from this catalog")?;
            let ReplacementEffectDef::PayOr { .. } = definition else {
                return Err("battlefield entry payment locator is not an optional payment".into());
            };
            let payer = player(*payer)?;
            let pending = game
                .pending_events
                .front()
                .ok_or("battlefield entry payment lacks its pending event")?;
            let authored = game.pending_resolved_payment(
                pending,
                context,
                match definition {
                    ReplacementEffectDef::PayOr { payment, .. } => payment,
                    _ => unreachable!(),
                },
            );
            let Some((authored_payer, payment)) = authored else {
                return Err(
                    "battlefield entry payer or payment disagrees with its authored effect".into(),
                );
            };
            // As above: snapshots decide agreement, and the authored payment
            // is the one restored.
            if payer != observation.player
                || authored_payer != payer
                || resolved_effect_payment_snapshot(payment) != *payment_snapshot
            {
                return Err(
                    "battlefield entry payer or payment disagrees with its authored effect".into(),
                );
            }
            if !game.can_pay_effect_payment(payer, payment) {
                return Err("battlefield entry payment is no longer payable".into());
            }
            let name = game.pending_entry_name(pending);
            let payment_label = Game::effect_payment_label(payment);
            let options = payment_decision_options(game, payer, payment, true, "Do not pay");
            validate_authored_decision(
                observation,
                payer,
                &format!("{payment_label} as {name} enters the battlefield?"),
                DecisionVisibility::Public,
                DecisionPreference::Neutral,
                1,
                1,
                &options,
                "battlefield entry payment",
            )?;
            DecisionContinuation::BattlefieldEntryPayment {
                context,
                player: payer,
                payment,
                definition,
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryReplacement { candidates } => {
            DecisionContinuation::BattlefieldEntryReplacement {
                candidates: candidates
                    .iter()
                    .map(|candidate| parse_applicable_replacement(candidate, &game.catalog))
                    .collect::<Result<Vec<_>, _>>()?,
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryOptional { context, effect } => {
            let context = parse_replacement_context(*context)?;
            validate_entry_decision_context(game, context, effect)?;
            let definition = catalog_replacement_effect(&game.catalog, effect)
                .ok_or("optional entry replacement locator is absent from this catalog")?;
            let pending = game
                .pending_events
                .front()
                .ok_or("optional entry replacement lacks its pending event")?;
            let mut before_selection = pending.clone();
            before_selection
                .applied
                .retain(|source| *source != context.source);
            let candidate = game
                .applicable_replacements(&before_selection)
                .into_iter()
                .find(|candidate| candidate.context == context && candidate.effect == definition)
                .ok_or("optional entry replacement is not applicable to its pending event")?;
            if !candidate.optional {
                return Err("optional entry replacement locator names a mandatory ability".into());
            }
            let owner = Game::pending_event_controller(pending);
            let name = game.pending_entry_name(pending);
            validate_authored_decision(
                observation,
                owner,
                &format!("Apply the optional replacement for {name}?"),
                DecisionVisibility::Public,
                DecisionPreference::Neutral,
                1,
                1,
                &Game::optional_entry_replacement_options(),
                "optional entry replacement",
            )?;
            DecisionContinuation::BattlefieldEntryOptional {
                context,
                effect: definition,
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryScalarChoice {
            context,
            effect,
            choices,
        } => {
            let context = parse_replacement_context(*context)?;
            validate_entry_decision_context(game, context, effect)?;
            let ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)) =
                catalog_replacement_effect(&game.catalog, effect)
                    .ok_or("entry scalar choice locator is absent from this catalog")?
            else {
                return Err("entry scalar choice locator is not a scalar choice".into());
            };
            let pending = game
                .pending_events
                .front()
                .ok_or("entry scalar choice lacks its pending event")?;
            let owner = Game::pending_event_controller(pending);
            let (prompt, authored_choices) = game.entry_scalar_choices(owner, choice);
            if *choices != authored_choices {
                return Err(
                    "entry scalar choice vocabulary disagrees with its authored choice".into(),
                );
            }
            let options = authored_choices
                .iter()
                .enumerate()
                .map(|(index, label)| DecisionOption {
                    id: u32::try_from(index).unwrap_or(u32::MAX),
                    label: label.clone(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                })
                .collect::<Vec<_>>();
            validate_authored_decision(
                observation,
                owner,
                prompt,
                DecisionVisibility::Public,
                DecisionPreference::Neutral,
                1,
                1,
                &options,
                "entry scalar choice",
            )?;
            DecisionContinuation::BattlefieldEntryScalarChoice {
                context,
                choice,
                choices: choices.clone(),
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryCopy {
            choices,
            added_types,
            retain_printed_subtypes,
            base_power_toughness,
            colors,
            added_creature_types,
            no_mana_cost,
            added_abilities,
        } => DecisionContinuation::BattlefieldEntryCopy {
            choices: game_ids(choices),
            added_types: parse_card_type_set(*added_types),
            retain_printed_subtypes: *retain_printed_subtypes,
            base_power_toughness: base_power_toughness.map(|stats| (stats[0], stats[1])),
            colors: colors.map(|flags| {
                let mut colors = crate::card::ColorSet::empty();
                for (color, present) in crate::card::ManaColor::COLORS.into_iter().zip(flags) {
                    if present {
                        colors = colors.with(color);
                    }
                }
                colors
            }),
            added_creature_types: added_creature_types
                .iter()
                .map(|name| {
                    crate::card::creature_type_name(name).ok_or_else(|| {
                        "checkpoint entry copy names an unknown creature type".to_owned()
                    })
                })
                .collect::<Result<Vec<_>, String>>()?,
            no_mana_cost: *no_mana_cost,
            added_abilities: added_abilities
                .iter()
                .map(|ability| parse_copiable_ability(ability, &game.catalog))
                .collect::<Result<Vec<_>, String>>()?,
        },
        other => {
            return Err(format!(
                "checkpoint continuation {other:?} is not a battlefield entry"
            ));
        }
    })
}
