//! Regression coverage for Karlov Manor's combinations of existing game operations.

use super::*;

fn board(library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
    }
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.turns_started = [3, 3];
    game
}

fn settle(game: &mut Game) {
    for _ in 0..64 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            let options = if matches!(
                pending.continuation,
                DecisionContinuation::OptionalEffect { .. } | DecisionContinuation::PayOr { .. }
            ) {
                vec![decision.options.last().unwrap().id]
            } else {
                decision
                    .options
                    .iter()
                    .map(|o| o.id)
                    .take(decision.minimum.max(1).min(decision.maximum))
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("resolution did not settle");
}

fn held(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let cards = game.build_zone(PlayerId::One, &[definition]).unwrap();
    let id = cards[0].id;
    game.players[0].hand.extend(cards);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    id
}

fn cast_action(game: &Game, id: GameObjectId, option: PlayOptionId) -> Action {
    game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::CastSpell {card, choices, ..} if *card == id && choices.play_option() == option && choices.costs().alternative().is_none())
    }).expect("the requested form is castable")
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let id = held(game, definition);
    let action = cast_action(game, id, PlayOptionId::DEFAULT);
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

fn permanent(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|p| p.card.definition == definition)
        .unwrap()
}

fn activate(game: &mut Game, id: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility {source,..} | Action::ActivateManaAbility {source,..} if *source == id))
        .expect("ability is available");
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

#[test]
fn disguise_casts_with_ward_and_turns_up_using_the_printed_cost() {
    let mut game = board(&[]);
    let id = held(&mut game, cards::DEFENESTRATED_PHANTOM);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| {
        matches!(a, Action::CastSpell {card, choices, ..} if *card == id && choices.costs().alternative().is_some())
    }).expect("disguise cast available");
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    let creature = permanent(&game, cards::DEFENESTRATED_PHANTOM);
    let id = creature.card.id;
    assert_eq!(creature.face_down, Some(crate::card::face_down::disguise()));
    assert_eq!(game.creature_stats(creature).unwrap().power, 2);
    assert!(!game.has_flying(creature));
    game.apply(PlayerId::One, Action::TurnFaceUp { permanent: id })
        .unwrap();
    let creature = permanent(&game, cards::DEFENESTRATED_PHANTOM);
    assert!(creature.face_down.is_none());
    assert!(game.has_flying(creature));
    assert_eq!(game.creature_stats(creature).unwrap().power, 4);
}

#[test]
fn cryptic_coat_cloaks_and_attaches_to_the_new_incarnation() {
    let mut game = board(&[cards::SAVANNAH_LIONS]);
    cast(&mut game, cards::CRYPTIC_COAT);
    let creature = permanent(&game, cards::SAVANNAH_LIONS);
    assert_eq!(creature.face_down, Some(crate::card::face_down::cloak()));
    assert_eq!(game.creature_stats(creature).unwrap().power, 3);
    assert_eq!(
        permanent(&game, cards::CRYPTIC_COAT).attached_to,
        Some(creature.card.id)
    );
    let coat = permanent(&game, cards::CRYPTIC_COAT).card.id;
    activate(&mut game, coat);
    assert_eq!(
        game.creature_stats(permanent(&game, cards::SAVANNAH_LIONS))
            .unwrap()
            .power,
        2
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::CRYPTIC_COAT)
    );
}

#[test]
fn hide_in_plain_sight_cloaks_two_and_keeps_the_rest_in_the_library() {
    let mut game = board(&[
        cards::PLAINS,
        cards::ISLAND,
        cards::MOUNTAIN,
        cards::SWAMP,
        cards::FOREST,
    ]);
    cast(&mut game, cards::HIDE_IN_PLAIN_SIGHT);
    assert_eq!(game.players[0].library.len(), 3);
    assert_eq!(game.battlefield.len(), 2);
    assert!(
        game.battlefield
            .iter()
            .all(|p| p.face_down == Some(crate::card::face_down::cloak()))
    );
}

#[test]
fn living_conundrum_skips_an_empty_library_draw_and_changes_characteristics() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::LIVING_CONUNDRUM)
        .unwrap();
    let creature = permanent(&game, cards::LIVING_CONUNDRUM);
    assert_eq!(game.creature_stats(creature).unwrap().power, 10);
    assert!(game.has_flying(creature));
    game.draw_card(PlayerId::One);
    settle(&mut game);
    assert!(!game.players[0].tried_to_draw_from_empty_library);
    game.players[0].library = game.build_zone(PlayerId::One, &[cards::PLAINS]).unwrap();
    assert!(!game.has_flying(permanent(&game, cards::LIVING_CONUNDRUM)));
    game.draw_card(PlayerId::One);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn no_witnesses_investigates_for_both_players_when_the_creature_count_is_tied() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .unwrap();
    cast(&mut game, cards::NO_WITNESSES);
    assert_eq!(game.battlefield.len(), 2);
    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| p.controller == player)
                .count(),
            1
        );
        assert_eq!(
            game.players[player.index()]
                .graveyard
                .iter()
                .filter(|c| c.definition == cards::SAVANNAH_LIONS)
                .count(),
            1
        );
    }
}

#[test]
fn homicide_investigator_fires_once_for_a_batch_and_only_once_per_turn() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::HOMICIDE_INVESTIGATOR)
        .unwrap();
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    game.destroy_permanents(&[first, second], false);
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    let third = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    game.destroy_permanent(third);
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
}

#[test]
fn crime_novelist_counts_an_artifact_sacrifice_paid_as_a_cost() {
    let mut game = board(&[cards::PLAINS]);
    game.put_onto_battlefield(PlayerId::One, cards::CRIME_NOVELIST)
        .unwrap();
    let glass = game
        .put_onto_battlefield(PlayerId::One, cards::MAGNIFYING_GLASS)
        .unwrap();
    held(&mut game, cards::PLAINS);
    // Investigate, then sacrifice the resulting Clue for its draw ability.
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility {source,..} if *source == glass))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    let clue = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    activate(&mut game, clue);
    let novelist = permanent(&game, cards::CRIME_NOVELIST);
    assert_eq!(game.creature_stats(novelist).unwrap().power, 2);
    assert_eq!(game.players[0].hand.len(), 2);
}

#[test]
fn soul_search_exiles_before_creating_the_small_card_reward() {
    let mut game = board(&[]);
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::SAVANNAH_LIONS])
        .unwrap();
    cast(&mut game, cards::SOUL_SEARCH);
    assert!(game.players[1].hand.is_empty());
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(game.has_flying(token));
    assert_eq!(game.creature_stats(token).unwrap().power, 1);
}

#[test]
fn fuss_and_bother_have_independent_castable_halves() {
    let mut game = board(&[]);
    let id = held(&mut game, cards::FUSS);
    game.step = Step::Upkeep;
    assert!(game.legal_actions(PlayerId::One).iter().any(|a| matches!(a, Action::CastSpell {card, choices, ..} if *card == id && choices.play_option() == PlayOptionId::DEFAULT)));
    assert!(!game.legal_actions(PlayerId::One).iter().any(|a| matches!(a, Action::CastSpell {card, choices, ..} if *card == id && choices.play_option() == PlayOptionId(1))));
    game.step = Step::PrecombatMain;
    let action = cast_action(&game, id, PlayOptionId(1));
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 3);
    assert!(game.battlefield.iter().all(|p| game.has_flying(p)));
}

#[test]
fn rakdos_offers_the_opponent_an_exact_two_permanent_sacrifice() {
    let mut game = board(&[cards::PLAINS, cards::ISLAND]);
    game.put_onto_battlefield(PlayerId::One, cards::RAKDOS_PATRON_OF_CHAOS)
        .unwrap();
    let first = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .unwrap();
    let second = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .unwrap();
    game.step = Step::PostcombatMain;
    game.advance_step();
    for _ in 0..32 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            let options = decision
                .options
                .iter()
                .filter(|o| o.label != "Decline")
                .take(decision.minimum.max(1).min(decision.maximum))
                .map(|o| o.id)
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    assert!(
        game.battlefield
            .iter()
            .all(|p| p.card.id != first && p.card.id != second)
    );
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[1].graveyard.len(), 2);
}

#[test]
fn public_thoroughfare_taps_an_existing_land_to_remain_on_the_battlefield() {
    let mut game = board(&[]);
    let plains = game
        .put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::PUBLIC_THOROUGHFARE)
        .unwrap();
    settle(&mut game);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == plains)
            .unwrap()
            .tapped
    );
    assert!(permanent(&game, cards::PUBLIC_THOROUGHFARE).tapped);
}

#[test]
fn unscrupulous_agent_does_not_expose_the_opponents_other_hand_cards() {
    let mut game = board(&[]);
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::SAVANNAH_LIONS, cards::PLAINS])
        .unwrap();
    let id = held(&mut game, cards::UNSCRUPULOUS_AGENT);
    let action = cast_action(&game, id, PlayOptionId::DEFAULT);
    game.apply(PlayerId::One, action).unwrap();
    for _ in 0..20 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            if decision.player == PlayerId::Two {
                assert_eq!(decision.visibility, DecisionVisibility::Private);
                assert_eq!(decision.options.len(), 2);
                return;
            }
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[0].id],
                },
            )
            .unwrap();
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("the opponent should choose a hand card privately");
}
