//! Regression coverage for Edge of Eternities' combinations of existing game operations.

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
    game.cards_drawn_this_turn = [0, 0];
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

fn cast_at(game: &mut Game, definition: CardDefinitionId, target: Target) {
    let id = held(game, definition);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,
        Action::CastSpell {card, choices,..} if *card == id && choices.iter_targets().any(|t| *t == target))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let id = held(game, definition);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,..} if *card==id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

#[test]
fn station_taps_another_summoning_sick_creature_and_unlocks_thresholds() {
    let mut game = board(&[]);
    let ship = game
        .put_onto_battlefield(PlayerId::One, cards::LUMEN_CLASS_FRIGATE)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    assert!(!is_creature(
        &game,
        permanent(&game, cards::LUMEN_CLASS_FRIGATE)
    ));
    activate(&mut game, ship);
    assert!(permanent(&game, cards::GRIZZLY_BEARS).tapped);
    assert_eq!(
        permanent(&game, cards::LUMEN_CLASS_FRIGATE).counters(CounterKind::named("charge")),
        2
    );
    assert_eq!(
        game.power(permanent(&game, cards::GRIZZLY_BEARS)).unwrap(),
        3
    );
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == ship)
        .unwrap()
        .add_counters(CounterKind::named("charge"), 10);
    assert!(is_creature(
        &game,
        permanent(&game, cards::LUMEN_CLASS_FRIGATE)
    ));
    assert_eq!(
        game.power(permanent(&game, cards::LUMEN_CLASS_FRIGATE))
            .unwrap(),
        3
    );
    assert!(game.permanent_has_executable_keyword(
        permanent(&game, cards::LUMEN_CLASS_FRIGATE),
        KeywordAbility::Flying
    ));
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == ship)
        .unwrap()
        .remove_counters(CounterKind::named("charge"), 11);
    assert!(!is_creature(
        &game,
        permanent(&game, cards::LUMEN_CLASS_FRIGATE)
    ));
    assert_eq!(
        game.power(permanent(&game, cards::GRIZZLY_BEARS)).unwrap(),
        2
    );
    game.battlefield.retain(|p| p.card.id != bear);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::ActivateAbility{source,..}if *source==ship))
    );
}
#[test]
fn station_reads_the_tapped_creatures_last_known_power_at_resolution() {
    let mut game = board(&[]);
    let ship = game
        .put_onto_battlefield(PlayerId::One, cards::GALVANIZING_SAWSHIP)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateAbility{source,..}if *source==ship))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    game.destroy_permanents(&[bear], false);
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::GALVANIZING_SAWSHIP).counters(CounterKind::named("charge")),
        2
    );
    assert!(!is_creature(
        &game,
        permanent(&game, cards::GALVANIZING_SAWSHIP)
    ));
}
#[test]
fn station_is_sorcery_speed_and_cannot_tap_the_animated_spacecraft_itself() {
    let mut game = board(&[]);
    let ship = game
        .put_onto_battlefield(PlayerId::One, cards::GALVANIZING_SAWSHIP)
        .unwrap();
    game.battlefield[0].add_counters(CounterKind::named("charge"), 3);
    assert!(is_creature(&game, &game.battlefield[0]));
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::ActivateAbility{source,..}if *source==ship))
    );
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.step = Step::DeclareAttackers;
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::ActivateAbility{source,..}if *source==ship))
    );
}
#[test]
fn planet_unlocks_a_mana_ability_without_becoming_a_creature() {
    let mut game = board(&[]);
    let planet = game
        .put_onto_battlefield(PlayerId::One, cards::EVENDO_WAKING_HAVEN)
        .unwrap();
    for _ in 0..3 {
        game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
    }
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == planet)
        .unwrap()
        .tapped = false;
    let before = game
        .legal_actions(PlayerId::One)
        .iter()
        .filter(|a| matches!(a,Action::ActivateManaAbility{source,..}if *source==planet))
        .count();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == planet)
        .unwrap()
        .add_counters(CounterKind::named("charge"), 12);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let choices = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|a| matches!(a,Action::ActivateManaAbility{source,..}if *source==planet))
        .collect::<Vec<_>>();
    assert!(choices.len() > before);
    game.apply(PlayerId::One, choices.last().unwrap().clone())
        .unwrap();
    assert!(!is_creature(
        &game,
        permanent(&game, cards::EVENDO_WAKING_HAVEN)
    ));
    assert_eq!(game.players[0].mana_pool.total(), 3);
}
#[test]
fn auxiliary_boosters_attaches_to_its_robot_and_grants_the_full_bonus() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::AUXILIARY_BOOSTERS)
        .unwrap();
    settle(&mut game);
    let robot = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(game.power(robot).unwrap(), 3);
    assert_eq!(game.toughness(robot).unwrap(), 4);
    assert!(game.permanent_has_executable_keyword(robot, KeywordAbility::Flying));
}
#[test]
fn pulsar_ace_gets_a_counter_when_no_spacecraft_is_found() {
    let mut game = board(&[cards::PLAINS; 5]);
    game.put_onto_battlefield(PlayerId::One, cards::PULSAR_SQUADRON_ACE)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::PULSAR_SQUADRON_ACE).counters(CounterKind::PlusOnePlusOne),
        1
    );
    assert_eq!(game.players[0].library.len(), 5);
}
#[test]
fn pulsar_ace_takes_a_spacecraft_without_getting_a_counter() {
    let mut game = board(&[cards::RESCUE_SKIFF, cards::PLAINS]);
    game.put_onto_battlefield(PlayerId::One, cards::PULSAR_SQUADRON_ACE)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::PULSAR_SQUADRON_ACE).counters(CounterKind::PlusOnePlusOne),
        0
    );
    assert_eq!(game.players[0].hand[0].definition, cards::RESCUE_SKIFF);
}
#[test]
fn emergency_eject_gives_the_destroyed_permanents_controller_a_working_lander() {
    let mut game = board(&[]);
    game.players[1].library = game.build_zone(PlayerId::Two, &[cards::ISLAND]).unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    cast_at(&mut game, cards::EMERGENCY_EJECT, Target::Permanent(bear));
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(token.controller, PlayerId::Two);
    let id = token.card.id;
    game.priority = PlayerId::Two;
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateAbility{source,..}if *source==id))
        .unwrap();
    game.apply(PlayerId::Two, action).unwrap();
    settle(&mut game);
    assert!(permanent(&game, cards::ISLAND).tapped);
    assert_eq!(permanent(&game, cards::ISLAND).controller, PlayerId::Two);
}
#[test]
fn sothera_uses_its_linked_exile_after_sacrificing_itself() {
    let mut game = board(&[]);
    let sothera = game
        .put_onto_battlefield(PlayerId::One, cards::SOTHERA_THE_SUPERVOID)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .unwrap();
    game.destroy_permanents(&[bear], false);
    settle(&mut game);
    assert_eq!(game.players[1].exile.len(), 1);
    for p in &mut game.players {
        p.mana_pool = ManaPool::default();
    }
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == sothera));
    let angel = permanent(&game, cards::SERRA_ANGEL);
    assert_eq!(angel.controller, PlayerId::One);
    assert_eq!(angel.counters(CounterKind::PlusOnePlusOne), 2);
}
#[test]
fn territorial_bruntar_exiles_through_nonland_and_permits_only_that_spell() {
    let mut game = board(&[cards::GRIZZLY_BEARS, cards::PLAINS, cards::ISLAND]);
    game.put_onto_battlefield(PlayerId::One, cards::TERRITORIAL_BRUNTAR)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .unwrap();
    settle(&mut game);
    assert!(game.players[0].library.is_empty());
    assert_eq!(game.players[0].exile.len(), 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    let actions = game.legal_actions(PlayerId::One);
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, Action::CastSpell { .. }))
    );
    assert!(!actions.iter().any(|a| matches!(a, Action::PlayLand { .. })));
}
#[test]
fn command_bridge_sacrifices_itself_without_an_untapped_payment() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::COMMAND_BRIDGE)
        .unwrap();
    settle(&mut game);
    assert!(game.battlefield.is_empty());
}
#[test]
fn vaultguard_can_discard_an_empty_hand_to_draw_two() {
    let mut game = board(&[cards::PLAINS; 5]);
    game.put_onto_battlefield(PlayerId::One, cards::VAULTGUARD_TROOPER)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    for p in &mut game.battlefield {
        p.tapped = true;
    }
    for p in &mut game.players {
        p.mana_pool = ManaPool::default();
    }
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
}
#[test]
fn alpharael_discards_one_artifact_or_two_other_cards() {
    let mut game = board(&[cards::PLAINS; 5]);
    held(&mut game, cards::BONESPLITTER);
    game.put_onto_battlefield(PlayerId::One, cards::ALPHARAEL_DREAMING_ACOLYTE)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].graveyard.len(), 2);
}

fn is_creature(game: &Game, permanent: &Permanent) -> bool {
    game.permanent_types(permanent)
        .is_some_and(|types| types.contains(CardType::Creature))
}

#[test]
fn frenzied_baloth_prohibits_prevention_for_both_players_combat_damage() {
    let mut game = board(&[]);
    cast(&mut game, cards::FRENZIED_BALOTH);
    let baloth = permanent(&game, cards::FRENZIED_BALOTH).card.id;
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    cast(&mut game, cards::FOG);
    assert_eq!(
        game.damage_target_from_kind(Some(baloth), Some(Target::Player(PlayerId::Two)), 1, true),
        1
    );
    assert_eq!(
        game.damage_target_from_kind(Some(bear), Some(Target::Player(PlayerId::One)), 1, true),
        1
    );
    game.destroy_permanents(&[baloth], false);
    assert_eq!(
        game.damage_target_from_kind(Some(bear), Some(Target::Player(PlayerId::One)), 1, true),
        0
    );
}

#[test]
fn larval_scoutlander_can_find_both_basic_lands_after_its_sacrifice() {
    let mut game = board(&[cards::ISLAND, cards::FOREST]);
    game.put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::LARVAL_SCOUTLANDER)
        .unwrap();
    for _ in 0..32 {
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
                    .take(decision.maximum)
                    .map(|o| o.id)
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
            break;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    assert!(game.players[0].library.is_empty());
    assert!(permanent(&game, cards::ISLAND).tapped);
    assert!(permanent(&game, cards::FOREST).tapped);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::PLAINS)
    );
}
