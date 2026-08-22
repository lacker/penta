use super::*;
use crate::card::{
    AbilityDef, AppliedRuleDef, CardComposition, CardDefinition, CardRules, CardSet,
    DamageEventMatcherDef, DamagePreventionDef, DamageSourceMatcherDef, EffectDef, KeywordAbility,
    ObjectPredicateDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRelation,
    ResolvedEffectDurationDef, ValueDef,
};
use crate::game::{DecisionContinuation, DecisionOrderSemantics};
use crate::{Action, ManaColor, ObjectBindingIndex};
use serde_json::json;

mod adversarial;
mod broad_audit;
mod effect_walkers;
mod face_down_characteristics;
mod rare_reconstructions;
mod rare_states;
mod semantics_coverage;
mod tokens;
mod trajectory;

#[test]
fn catalog_semantics_rehydrate_top_level_and_nested_abilities() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let top_level = catalog
        .definitions()
        .into_iter()
        .flat_map(|definition| &definition.parts)
        .flat_map(|part| part.rules.indexed_abilities())
        .next()
        .expect("catalog has an ability")
        .definition;
    let locator = ability_locator(&catalog, |candidate| *candidate == top_level)
        .expect("top-level ability has a locator");
    assert_eq!(catalog_ability(&catalog, &locator), Some(top_level));

    let granted_text =
        "At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.";
    let locator = ability_locator(&catalog, |candidate| candidate.text == granted_text)
        .expect("nested granted ability has a locator");
    let rebuilt = catalog_ability(&catalog, &locator).expect("nested locator resolves");
    assert_eq!(rebuilt.text, granted_text);
    assert!(
        !locator_nested(&locator).is_empty(),
        "the granted clause is addressed beneath its printed source"
    );
}

#[test]
fn resolved_effect_locators_prefer_the_effect_source_ability() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut example = None;
    'definitions: for definition in catalog.definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let source_root = model::AbilityLocator::Card {
                    definition: definition.id,
                    part_id: part.id.0,
                    ability_id: attached.id.0,
                    nested: Vec::new(),
                };
                for effect in semantics::applied_effects(&attached.definition) {
                    let Some(global) = semantics::applied_effect_locator(&catalog, effect) else {
                        continue;
                    };
                    if printed_locator_root(&global.ability) != printed_locator_root(&source_root) {
                        example = Some((source_root, effect));
                        break 'definitions;
                    }
                }
            }
        }
    }
    let (source_root, effect) = example.expect("the catalog contains a repeated applied effect");
    let source = source_for_locator(GameObjectId(89_999), &source_root);
    let anchored = resolved_applied_effect_locator(&catalog, source, effect)
        .expect("the repeated effect has a source-anchored locator");
    assert_eq!(
        printed_locator_root(&anchored.ability),
        printed_locator_root(&source_root),
        "resolved provenance must not collapse to the catalog's first equal effect"
    );
}

fn source_for_locator(object: GameObjectId, locator: &model::AbilityLocator) -> AbilitySourceRef {
    let (definition, part, ability) =
        printed_locator_root(locator).expect("the test source locator is printed");
    AbilitySourceRef {
        object,
        ability: AbilityOrigin::Printed {
            definition,
            part: CardPartId(part),
            ability: AbilityId(ability),
        },
    }
}

fn locator_nested(locator: &model::AbilityLocator) -> &[usize] {
    match locator {
        model::AbilityLocator::Card { nested, .. }
        | model::AbilityLocator::Token { nested, .. }
        | model::AbilityLocator::Emblem { nested, .. } => nested,
    }
}

fn printed_locator_root(locator: &model::AbilityLocator) -> Option<(CardDefinitionId, u8, u8)> {
    let model::AbilityLocator::Card {
        definition,
        part_id,
        ability_id,
        ..
    } = locator
    else {
        return None;
    };
    Some((*definition, *part_id, *ability_id))
}

fn source_without_applied_effect(
    catalog: &CardCatalog,
    object: GameObjectId,
    expected: AppliedEffectDef,
) -> AbilitySourceRef {
    for definition in catalog.definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let locator = model::AbilityLocator::Card {
                    definition: definition.id,
                    part_id: part.id.0,
                    ability_id: attached.id.0,
                    nested: Vec::new(),
                };
                let source = source_for_locator(object, &locator);
                if resolved_applied_effect_locator(catalog, source, expected).is_none() {
                    return source;
                }
            }
        }
    }
    panic!("the catalog has an unrelated source ability");
}

fn splice_printed_source_ability(value: &mut Value, source: AbilitySourceRef) {
    let AbilityOrigin::Printed {
        definition,
        part,
        ability,
    } = source.ability
    else {
        panic!("the test splice source is printed");
    };
    value["definition"] = json!(definition.get());
    value["partId"] = json!(part.0);
    value["abilityId"] = json!(ability.0);
}

fn composite_modify_and_grant(
    catalog: &CardCatalog,
    source_object: GameObjectId,
) -> (
    AbilitySourceRef,
    AppliedEffectDef,
    AbilityDef,
    AppliedEffectDef,
) {
    let locator = ability_locator(catalog, |ability| {
        let effects = semantics::applied_effects(ability);
        effects.iter().any(|effect| {
            matches!(
                effect,
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::Modify { .. }
                ))
            )
        }) && effects.iter().any(|effect| {
            matches!(
                effect,
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                    AbilityOperationDef::Add(_)
                ))
            )
        })
    })
    .expect("the catalog has a compound pump-and-grant ability");
    let ability = catalog_ability(catalog, &locator).expect("the located ability rebuilds");
    let effects = semantics::applied_effects(&ability);
    let modify = effects
        .iter()
        .copied()
        .find(|effect| {
            matches!(
                effect,
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::Modify { .. }
                ))
            )
        })
        .expect("the compound ability modifies power and toughness");
    let (granted, grant) = effects
        .iter()
        .copied()
        .find_map(|effect| {
            let AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(granted),
            )) = effect
            else {
                return None;
            };
            Some((*granted, effect))
        })
        .expect("the compound ability grants an ability");
    (
        source_for_locator(source_object, &locator),
        modify,
        granted,
        grant,
    )
}

fn dynamic_modify(
    catalog: &CardCatalog,
    source_object: GameObjectId,
) -> (AbilitySourceRef, AppliedEffectDef) {
    let locator = ability_locator(catalog, |ability| {
        semantics::applied_effects(ability).iter().any(|effect| {
            matches!(
                effect,
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::Modify { power, toughness }
                )) if !matches!(power, ValueDef::Constant(_))
                    || !matches!(toughness, ValueDef::Constant(_))
            )
        })
    })
    .expect("the catalog has a dynamic power/toughness modifier");
    let ability = catalog_ability(catalog, &locator).expect("the located ability rebuilds");
    let definition = semantics::applied_effects(&ability)
        .into_iter()
        .find(|effect| {
            matches!(
                effect,
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::Modify { power, toughness }
                )) if !matches!(power, ValueDef::Constant(_))
                    || !matches!(toughness, ValueDef::Constant(_))
            )
        })
        .expect("the located ability contains its dynamic modifier");
    (source_for_locator(source_object, &locator), definition)
}

fn subtype_change(
    catalog: &CardCatalog,
    source_object: GameObjectId,
) -> (
    AbilitySourceRef,
    AppliedEffectDef,
    SetOperationDef<&'static [&'static str]>,
) {
    let locator = ability_locator(catalog, |ability| {
        semantics::applied_effects(ability).iter().any(|effect| {
            matches!(
                effect,
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(_))
            )
        })
    })
    .expect("the catalog has a generic subtype-changing effect");
    let ability = catalog_ability(catalog, &locator).expect("the located ability rebuilds");
    let definition = semantics::applied_effects(&ability)
        .into_iter()
        .find(|effect| {
            matches!(
                effect,
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(_))
            )
        })
        .expect("the located ability contains its subtype change");
    let AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(operation)) =
        definition
    else {
        unreachable!("the located definition was a subtype change")
    };
    (
        source_for_locator(source_object, &locator),
        definition,
        operation,
    )
}

fn rebuild_current_checkpoint(game: &Game, viewer: PlayerId, seed: u64) -> (Value, Game) {
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(game, viewer),
        seed,
    )
    .expect("the current checkpoint reconstructs");
    (wire, rebuilt)
}

include!("tests/resolved_effects.rs");

include!("tests/prevention_and_replacements.rs");

include!("tests/stack_completion.rs");

fn checkpoint_wire(game: &Game) -> (PlayerId, Value) {
    let viewer = game.decision_player().expect("the game awaits an action");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    (viewer, wire)
}

include!("tests/decisions_and_triggers.rs");

/// A hypothesis the seat could have chosen instead of the truth: both hidden
/// libraries reversed and the opposing hand rotated. Card counts and every
/// card the seat can actually see survive, so this stays consistent with what
/// the observation says while naming different cards than the host holds.
/// Reconstruction that only works from [`true_hidden_hypothesis`] would be no
/// use to a search bot, which never knows the truth.
fn determinized(hidden: &Value, viewer: PlayerId) -> Value {
    let mut hidden = hidden.clone();
    for seat in ["p1", "p2"] {
        if let Some(library) = hidden
            .get_mut("libraries")
            .and_then(|libraries| libraries.get_mut(seat))
            .and_then(Value::as_array_mut)
        {
            library.reverse();
        }
    }
    let opponent = seat_label(viewer.opponent());
    if let Some(hand) = hidden
        .get_mut("hands")
        .and_then(|hands| hands.get_mut(opponent))
        .and_then(Value::as_array_mut)
        && !hand.is_empty()
    {
        hand.rotate_left(1);
    }
    hidden
}

fn true_hidden_hypothesis(game: &Game, viewer: PlayerId) -> Value {
    let definitions = |cards: &[CardInstance]| {
        cards
            .iter()
            .map(|card| card.definition.get())
            .collect::<Vec<_>>()
    };
    let opponent = viewer.opponent();
    let opponent_hand = &game.players[opponent.index()].hand;
    let drawn_indices = game.drawn_this_turn[opponent.index()]
        .iter()
        .filter_map(|id| opponent_hand.iter().position(|card| card.id == *id))
        .collect::<Vec<_>>();
    let mut discard_choices = serde_json::Map::new();
    if let Some(DecisionContinuation::DiscardForEffect { chosen, .. }) = game
        .pending_decisions
        .first()
        .map(|pending| &pending.continuation)
    {
        for (player, cards) in chosen {
            if *player == viewer {
                continue;
            }
            let hand = &game.players[player.index()].hand;
            let indices = cards
                .iter()
                .filter_map(|id| hand.iter().position(|card| card.id == *id))
                .collect::<Vec<_>>();
            discard_choices.insert(seat_label(*player).into(), json!(indices));
        }
    }
    json!({
        "hands": {
            (seat_label(opponent)): definitions(opponent_hand),
        },
        "libraries": {
            "p1": definitions(&game.players[PlayerId::One.index()].library),
            "p2": definitions(&game.players[PlayerId::Two.index()].library),
        },
        "outsideGame": {
            "p1": definitions(&game.players[PlayerId::One.index()].outside_game),
            "p2": definitions(&game.players[PlayerId::Two.index()].outside_game),
        },
        "drawnThisTurn": {
            (seat_label(opponent)): drawn_indices,
        },
        "decision": {
            "discardChoices": discard_choices,
        },
    })
}

#[test]
fn checkpoint_round_trips_extra_turns_and_the_regular_turn_anchor() {
    let mut game = crate::game::tests::ready_game();
    game.extra_turns = vec![PlayerId::One];
    game.next_regular_player = PlayerId::Two;
    let viewer = PlayerId::One;
    let observation = game.observe(viewer);
    let observation = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &game.legal_actions(viewer),
    );
    let hidden = json!({
        "hands": {"p2": []},
        "libraries": {
            "p1": game.players[0].library.iter().map(|card| card.definition.get()).collect::<Vec<_>>(),
            "p2": game.players[1].library.iter().map(|card| card.definition.get()).collect::<Vec<_>>(),
        },
        "outsideGame": {"p1": [], "p2": []},
    });

    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &observation,
        &hidden,
        91,
    )
    .expect("scheduler checkpoint reconstructs");

    assert_eq!(rebuilt.extra_turns, vec![PlayerId::One]);
    assert_eq!(rebuilt.next_regular_player, PlayerId::Two);
    rebuilt.start_next_turn();
    assert_eq!(rebuilt.active_player, PlayerId::One);
    assert_eq!(rebuilt.next_regular_player, PlayerId::Two);
    rebuilt.start_next_turn();
    assert_eq!(rebuilt.active_player, PlayerId::Two);
    assert_eq!(rebuilt.next_regular_player, PlayerId::One);
}

#[test]
fn checkpoint_round_trips_the_ordered_turn_phase_queue_and_resume_boundary() {
    let mut game = crate::game::tests::ready_game();
    game.step = Step::BeginningOfCombat;
    game.turn_phase_queue = VecDeque::from([
        TurnPhaseDef::PostcombatMain,
        TurnPhaseDef::Combat,
        TurnPhaseDef::PostcombatMain,
    ]);
    game.turn_phase_resume = Some(TurnPhaseResume::Step(Step::End));

    let (wire, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 92);
    assert_eq!(
        wire["checkpoint"]["turnPhaseQueue"],
        json!(["postcombatMain", "combat", "postcombatMain"])
    );
    assert_eq!(wire["checkpoint"]["turnPhaseResume"], json!("end"));
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    assert_eq!(rebuilt.turn_phase_queue, game.turn_phase_queue);
    assert_eq!(rebuilt.turn_phase_resume, game.turn_phase_resume);

    rebuilt.step = Step::EndOfCombat;
    rebuilt.advance_step();
    assert_eq!(rebuilt.step, Step::PostcombatMain);
    rebuilt.advance_step();
    assert_eq!(rebuilt.step, Step::BeginningOfCombat);
    rebuilt.step = Step::EndOfCombat;
    rebuilt.advance_step();
    assert_eq!(rebuilt.step, Step::PostcombatMain);
    assert!(rebuilt.turn_phase_queue.is_empty());
    assert_eq!(
        rebuilt.turn_phase_resume,
        Some(TurnPhaseResume::Step(Step::End))
    );

    // The final inserted phase has no queued successor but still needs the
    // frozen continuation. That active state is independently reconstructible.
    let (final_wire, mut final_rebuilt) = rebuild_current_checkpoint(&rebuilt, PlayerId::One, 93);
    assert_eq!(final_wire["checkpoint"]["turnPhaseQueue"], json!([]));
    assert_eq!(final_wire["checkpoint"]["turnPhaseResume"], json!("end"));
    final_rebuilt.advance_step();
    assert_eq!(final_rebuilt.step, Step::End);
    assert!(final_rebuilt.turn_phase_queue.is_empty());
    assert_eq!(final_rebuilt.turn_phase_resume, None);
}
