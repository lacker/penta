//! Regression coverage for Aetherdrift's combinations of existing game operations.

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
fn exhaust_is_consumed_per_object_and_grows_the_creature() {
    let mut game = board(&[cards::PLAINS]);
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::KEEN_BUCCANEER)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 4);
    activate(&mut game, id);
    assert_eq!(
        permanent(&game, cards::KEEN_BUCCANEER)
            .counters
            .count(CounterKind::PlusOnePlusOne),
        1
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::ActivateAbility{source,..} if *source==id))
    );
    game.commit_next_turn(PlayerId::One, Vec::new());
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::ActivateAbility{source,..} if *source==id))
    );
}
#[test]
fn kolodin_animates_a_vehicle_after_it_enters() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::KOLODIN_TRIUMPH_CASTER)
        .unwrap();
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::AIR_RESPONSE_UNIT)
        .unwrap();
    settle(&mut game);
    let vehicle = game.battlefield.iter().find(|p| p.card.id == id).unwrap();
    assert_eq!(game.creature_stats(vehicle).unwrap().power, 3);
    assert!(game.permanent_has_executable_keyword(vehicle, KeywordAbility::Haste));
}
#[test]
fn basri_cycling_protects_cats_before_drawing() {
    let mut game = board(&[cards::PLAINS]);
    let cat = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    let id = held(&mut game, cards::BASRI_TOMORROW_S_CHAMPION);
    activate(&mut game, id);
    let cat = game.battlefield.iter().find(|p| p.card.id == cat).unwrap();
    assert!(game.permanent_has_executable_keyword(cat, KeywordAbility::Hexproof));
    assert!(game.permanent_has_executable_keyword(cat, KeywordAbility::Indestructible));
    assert_eq!(game.players[0].hand.len(), 1);
}
#[test]
fn tune_up_returns_and_permanently_animates_a_vehicle() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::AIR_RESPONSE_UNIT])
        .unwrap();
    cast(&mut game, cards::TUNE_UP);
    let vehicle = permanent(&game, cards::AIR_RESPONSE_UNIT);
    assert_eq!(game.creature_stats(vehicle).unwrap().power, 3);
    game.players[0].mana_pool = ManaPool::default();
    game.commit_next_turn(PlayerId::Two, Vec::new());
    assert!(
        game.creature_stats(permanent(&game, cards::AIR_RESPONSE_UNIT))
            .is_some()
    );
}
#[test]
fn marketback_draws_its_last_known_counter_count_after_dying() {
    let mut game = board(&[cards::PLAINS, cards::ISLAND, cards::SWAMP]);
    let id = held(&mut game, cards::MARKETBACK_WALKER);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,choices,..} if *card==id && choices.x()==3))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    let id = permanent(&game, cards::MARKETBACK_WALKER).card.id;
    game.destroy_permanents(&[id], false);
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 3);
}
#[test]
fn thunderous_velocipede_changes_entry_counters_by_mana_value() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::THUNDEROUS_VELOCIPEDE)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::SHIVAN_DRAGON)
        .unwrap();
    assert_eq!(
        permanent(&game, cards::GRIZZLY_BEARS)
            .counters
            .count(CounterKind::PlusOnePlusOne),
        1
    );
    assert_eq!(
        permanent(&game, cards::SHIVAN_DRAGON)
            .counters
            .count(CounterKind::PlusOnePlusOne),
        3
    );
}
#[test]
fn coalstoke_returns_an_opponents_card_with_finality_and_exiles_it_next_own_end_step() {
    let mut game = board(&[]);
    game.players[1].graveyard = game
        .build_zone(PlayerId::Two, &[cards::GRIZZLY_BEARS])
        .unwrap();
    cast(&mut game, cards::COALSTOKE_GEARHULK);
    let bear = permanent(&game, cards::GRIZZLY_BEARS);
    assert_eq!(bear.controller, PlayerId::One);
    assert_eq!(bear.counters.count(CounterKind::Finality), 1);
    assert!(game.permanent_has_executable_keyword(bear, KeywordAbility::Haste));
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
}
#[test]
fn guidelight_pathmaker_puts_a_small_artifact_directly_onto_the_battlefield() {
    let mut game = board(&[cards::SOL_RING]);
    cast(&mut game, cards::GUIDELIGHT_PATHMAKER);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::SOL_RING)
    );
    assert!(game.players[0].hand.is_empty());
}
#[test]
fn intimidation_tactics_exiles_the_chosen_artifact() {
    let mut game = board(&[]);
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::SOL_RING, cards::LIGHTNING_BOLT])
        .unwrap();
    cast_at(
        &mut game,
        cards::INTIMIDATION_TACTICS,
        Target::Player(PlayerId::Two),
    );
    assert_eq!(game.players[1].hand.len(), 1);
    assert_eq!(game.players[1].exile[0].definition, cards::SOL_RING);
}

#[test]
fn redshift_mana_pays_for_activations_but_cannot_cast_spells() {
    let mut game = board(&[]);
    let redshift = game
        .put_onto_battlefield(PlayerId::One, cards::REDSHIFT_ROCKETEER_CHIEF)
        .unwrap();
    let doll = game
        .put_onto_battlefield(PlayerId::One, cards::WRETCHED_DOLL)
        .unwrap();
    for p in &mut game.battlefield {
        p.entered_controller_turn = 0;
    }
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::DOOM_BLADE])
        .unwrap();
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::ActivateManaAbility{source,color:ManaColor::Black,..} if *source==redshift)).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 2);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell { .. }))
    );
    activate(&mut game, doll);
    assert_eq!(game.players[0].mana_pool.total(), 1);
}

#[test]
fn boommobile_entry_mana_retains_its_activation_restriction() {
    let mut game = board(&[]);
    cast(&mut game, cards::BOOMMOBILE);
    assert_eq!(
        game.players[0]
            .mana
            .iter()
            .filter(|mana| mana.restrictions
                == [ManaRestrictionDef::ActivateAbility(ObjectPredicateDef::Any)])
            .count(),
        4
    );
}

#[test]
fn greasewrench_draws_for_cards_discarded_and_keeps_its_counter() {
    let mut game = board(&[cards::PLAINS, cards::ISLAND]);
    let goblin = game
        .put_onto_battlefield(PlayerId::One, cards::GREASEWRENCH_GOBLIN)
        .unwrap();
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::MOUNTAIN, cards::SWAMP])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);
    activate(&mut game, goblin);
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert_eq!(game.players[0].library.len(), 1);
    assert_eq!(
        permanent(&game, cards::GREASEWRENCH_GOBLIN)
            .counters
            .count(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn dune_drifter_entry_target_keeps_cast_x_after_the_vehicle_leaves() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    let id = held(&mut game, cards::DUNE_DRIFTER);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,choices,..} if *card==id && choices.x()==2))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    for _ in 0..16 {
        if game
            .battlefield
            .iter()
            .any(|p| p.card.definition == cards::DUNE_DRIFTER)
        {
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    let drifter = permanent(&game, cards::DUNE_DRIFTER).card.id;
    game.destroy_permanents(&[drifter], false);
    settle(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn rover_blades_detaches_when_crewed() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let blades = game
        .put_onto_battlefield(PlayerId::One, cards::ROVER_BLADES)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == blades)
        .unwrap()
        .attached_to = Some(bear);
    activate(&mut game, blades);
    let blades = permanent(&game, cards::ROVER_BLADES);
    assert!(game.creature_stats(blades).is_some());
    assert_eq!(blades.attached_to, None);
}
