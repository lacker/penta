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

#[test]
fn player_aura_attachment_round_trips() {
    let mut game = crate::game::tests::ready_game();
    let mut curse = crate::game::tests::creature(
        90_100,
        crate::card::cards::CURSE_OF_THE_BLOODY_TOME,
        PlayerId::One,
    );
    let curse_id = curse.card.id;
    curse.attached_player = Some(PlayerId::Two);
    game.battlefield.push(curse);

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 90_101);
    let checkpoint_curse = wire["checkpoint"]["battlefield"]
        .as_array()
        .expect("the checkpoint has a battlefield")
        .iter()
        .find(|permanent| permanent["objectId"] == curse_id.0)
        .expect("the Curse is checkpointed");
    assert_eq!(checkpoint_curse["attachedPlayer"], PlayerId::Two.index());
    assert_eq!(
        rebuilt
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == curse_id)
            .expect("the Curse reconstructs")
            .attached_player,
        Some(PlayerId::Two),
    );
}

#[test]
fn public_chosen_color_round_trips() {
    let mut game = crate::game::tests::ready_game();
    let mut aura = crate::game::tests::creature(
        90_110,
        crate::card::cards::SHIMMERWILDS_GROWTH,
        PlayerId::One,
    );
    let aura_id = aura.card.id;
    aura.chosen_color = Some(ManaColor::Blue);
    game.battlefield.push(aura);

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 90_111);
    let shown = wire["battlefield"]
        .as_array()
        .expect("the observation has a battlefield")
        .iter()
        .find(|permanent| permanent["objectId"] == aura_id.0)
        .expect("the Aura is observed");
    assert_eq!(shown["chosenColor"], "blue");
    assert_eq!(
        rebuilt
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == aura_id)
            .expect("the Aura reconstructs")
            .chosen_color,
        Some(ManaColor::Blue),
    );
}

#[test]
fn checkpoint_preserves_predicate_filterable_cast_history() {
    let mut game = crate::game::tests::ready_game();
    let spell = crate::game::tests::card(94_000, crate::card::cards::THINK_TWICE, PlayerId::One);
    let card_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == card_id))
        .expect("Think Twice can be cast");
    game.apply(PlayerId::One, cast).expect("the cast succeeds");
    let spell_id = game.stack.last().expect("the spell is on the stack").id;
    for _ in 0..4 {
        if game.stack.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority can be passed");
    }
    assert!(game.stack.is_empty(), "Think Twice resolved");
    assert!(
        matches!(
            game.retired_objects.get(&spell_id),
            Some(RetiredObject::Stack(_))
        ),
        "the history points at the locked retired spell"
    );

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 94);
    assert_eq!(
        wire["checkpoint"]["spellCastHistoryThisTurn"],
        json!([spell_id.0]),
    );
    assert_eq!(rebuilt.spell_cast_history_this_turn, vec![spell_id]);

    let mut legacy = wire;
    legacy["checkpoint"]
        .as_object_mut()
        .expect("the checkpoint is an object")
        .remove("spellCastHistoryThisTurn");
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &legacy,
        &true_hidden_hypothesis(&game, PlayerId::One),
        95,
    )
    .expect("an older checkpoint defaults the additive history");
    assert!(rebuilt.spell_cast_history_this_turn.is_empty());
}

/// A face-down exile that hides from its owner is on the wire, and an older
/// checkpoint that never carried the flag restores as the look-permitting
/// kind every face-down exile used to be.
#[test]
fn a_face_down_exile_keeps_who_may_look_at_it() {
    let mut game = crate::game::tests::ready_game();
    let hidden =
        crate::game::tests::card(90_400, crate::card::cards::LIGHTNING_BOLT, PlayerId::One);
    let hidden_id = hidden.id;
    game.players[PlayerId::One.index()].exile.push(hidden);
    game.hide_from_everyone_while_exiled(hidden_id, PlayerId::One);

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 90_401);
    assert!(
        rebuilt.exiled_card_is_hidden_from_owner(hidden_id),
        "the flag survives the round trip",
    );
    assert!(
        rebuilt.observe(PlayerId::One).exiles[PlayerId::One.index()].is_empty(),
        "so the rebuilt game hides it from its owner too",
    );

    let mut legacy = wire;
    for permission in legacy["checkpoint"]["exilePlayPermissions"]
        .as_array_mut()
        .expect("the checkpoint carries the permissions")
    {
        permission
            .as_object_mut()
            .expect("a permission is an object")
            .remove("hiddenFromOwner");
    }
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &legacy,
        &true_hidden_hypothesis(&game, PlayerId::One),
        90_402,
    )
    .expect("an older checkpoint defaults the additive flag");
    assert!(
        !rebuilt.exiled_card_is_hidden_from_owner(hidden_id),
        "and an older one restores as the kind its owner may look at",
    );
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
include!("tests/searches_and_installed_triggers.rs");

include!("tests/attached_search.rs");

include!("tests/gilded_drake.rs");

include!("tests/modal_triggers.rs");

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
