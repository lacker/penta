#[allow(clippy::too_many_lines)]
fn parse_basic_object_collection_continuation(
    value: &DecisionContinuationSnapshot,
    observation: &DecisionObservation,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    match value {
        DecisionContinuationSnapshot::ChooseObjectOrderForEffect {
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("object-order locator disagrees with its resolving ability".into());
            }
            let EffectDef::ChooseObjectOrder(definition) = continuation.effect.effect else {
                return Err("object-order locator does not identify an ordering choice".into());
            };
            let actor = game
                .effect_player_reference(
                    definition.actor,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("object-order actor is not singular")?;
            let candidates = game.effect_objects(
                definition.input,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            );
            if candidates.len() <= 1 {
                return Err("object-order checkpoint encodes an automatic ordering".into());
            }
            let options = candidates
                .iter()
                .copied()
                .enumerate()
                .map(|(index, target)| game.effect_target_option(index, target))
                .collect::<Vec<_>>();
            validate_ordered_authored_decision(
                observation,
                actor,
                Game::library_order_prompt(definition.placement),
                effect_choice_visibility(definition.visibility),
                &options,
                "object ordering",
            )?;
            Ok(DecisionContinuation::ChooseObjectOrderForEffect {
                definition: continuation.effect,
                candidates,
                object: continuation.object,
                context: continuation.context,
                effect: continuation.effect.with_effect(*definition.then),
            })
        }
        DecisionContinuationSnapshot::LookAtObjectsForEffect {
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("object-look locator disagrees with its resolving ability".into());
            }
            let EffectDef::LookAtObjects(definition) = continuation.effect.effect else {
                return Err("object-look locator does not identify a look".into());
            };
            let actor = game
                .effect_player_reference(
                    definition.actor,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("object-look actor is not singular")?;
            let members = game
                .effect_object_collection(
                    definition.source,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("object-look collection source is not singular")?;
            if members.is_empty() {
                return Err("object-look checkpoint encodes an empty collection".into());
            }
            let options = vec![DecisionOption {
                id: 0,
                label: "Continue".into(),
                card: None,
                members: members
                    .iter()
                    .copied()
                    .filter_map(|target| game.effect_target_card(target))
                    .collect(),
                ability_text: None,
                zone: DecisionZone::None,
            }];
            validate_authored_decision(
                observation,
                actor,
                "Continue",
                effect_choice_visibility(definition.visibility),
                DecisionPreference::Neutral,
                0,
                0,
                &options,
                "object look",
            )?;
            Ok(DecisionContinuation::LookAtObjectsForEffect {
                definition: continuation.effect,
                object: continuation.object,
                context: continuation.context,
                effect: continuation.effect.with_effect(*definition.then),
            })
        }
        DecisionContinuationSnapshot::PartitionGroupForEffect {
            continuation: snapshot,
        } => {
            let continuation = parse_effect_continuation(snapshot, game)?;
            if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
                return Err("group-partition locator disagrees with its resolving ability".into());
            }
            let EffectDef::PartitionGroup(definition) = continuation.effect.effect else {
                return Err("group-partition locator does not identify a partition".into());
            };
            let actor = game
                .effect_player_reference(
                    definition.actor,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("group-partition actor is not singular")?;
            let items = game.effect_objects(
                definition.input,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            );
            if items.is_empty() {
                return Err("group-partition checkpoint encodes an empty group".into());
            }
            let options = items
                .iter()
                .copied()
                .enumerate()
                .map(|(index, target)| game.effect_target_option(index, target))
                .collect::<Vec<_>>();
            validate_authored_decision(
                observation,
                actor,
                "Separate the objects into two piles",
                effect_choice_visibility(definition.visibility),
                DecisionPreference::BalancedPartition,
                0,
                items.len(),
                &options,
                "group partition",
            )?;
            Ok(DecisionContinuation::PartitionGroupForEffect {
                definition: continuation.effect,
                items,
                object: continuation.object,
                context: continuation.context,
                effect: continuation.effect.with_effect(*definition.then),
            })
        }
        _ => Err("checkpoint is not a basic object-collection continuation".into()),
    }
}
