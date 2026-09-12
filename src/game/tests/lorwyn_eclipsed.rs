//! Regression coverage for Lorwyn Eclipsed combinations of existing game operations.

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
fn vivid_counts_each_represented_color_once() {
    let mut game = board(&[cards::FOREST; 8]);
    for _ in 0..2 {
        game.put_onto_battlefield(PlayerId::One, cards::BOGGART_CURSECRAFTER)
            .unwrap();
    }
    game.put_onto_battlefield(PlayerId::One, cards::SHINESTRIKER)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 3);
    game.put_onto_battlefield(PlayerId::One, cards::KITHKEEPER)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        4
    );
}

#[test]
fn optional_blight_can_use_another_creature_and_counts_before_state_based_death() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::SOURBREAD_AUNTIE)
        .unwrap();
    settle(&mut game);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        2
    );
    assert_eq!(
        permanent(&game, cards::SOURBREAD_AUNTIE).counters(CounterKind::MinusOneMinusOne),
        0
    );
}

#[test]
fn blight_follow_up_survives_its_original_source() {
    let mut game = board(&[cards::FOREST]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::BLIGHTED_BLACKTHORN)
        .unwrap();
    game.destroy_permanents(&[source], false);
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].life, 19);
}

#[test]
fn tapping_reejerey_removes_one_counter_and_untapping_does_not() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::ENCUMBERED_REEJEREY)
        .unwrap();
    assert_eq!(
        permanent(&game, cards::ENCUMBERED_REEJEREY).counters(CounterKind::MinusOneMinusOne),
        3
    );
    let spell = held(&mut game, cards::RIME_CHILL);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==spell&&choices.iter_targets().any(|t|*t==Target::Permanent(source)))).unwrap();
    // Supply a draw so the cantrip does not end the game.
    game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST]).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::ENCUMBERED_REEJEREY).counters(CounterKind::MinusOneMinusOne),
        2
    );
}

#[test]
fn thirst_for_identity_discards_one_creature_or_two_other_cards() {
    for (library, expected) in [
        ([cards::GRIZZLY_BEARS, cards::FOREST, cards::FOREST], 2),
        ([cards::FOREST; 3], 1),
    ] {
        let mut game = board(&library);
        cast(&mut game, cards::THIRST_FOR_IDENTITY);
        assert_eq!(game.players[0].hand.len(), expected);
    }
}

#[test]
fn brigid_transforms_at_first_main_and_uses_other_creature_count_for_mana() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::BRIGID_CLACHAN_S_HEART)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 2);
    game.step = Step::Draw;
    game.advance_step();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    settle(&mut game);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == source)
        .unwrap()
        .entered_controller_turn = 0;
    activate(&mut game, source);
    assert_eq!(game.players[0].mana.len(), 1);
}

#[test]
fn ashling_back_face_mana_keeps_its_large_spell_restriction() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::ASHLING_REKINDLED)
        .unwrap();
    settle(&mut game);
    game.step = Step::Draw;
    game.advance_step();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    settle(&mut game);
    assert_eq!(game.players[0].mana.len(), 2);
    let cheap = game
        .build_zone(PlayerId::One, &[cards::MIND_STONE])
        .unwrap();
    let id = cheap[0].id;
    game.players[0].hand.extend(cheap);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::CastSpell{card,..}if *card==id))
    );
    assert!(game.battlefield.iter().any(|p| p.card.id == source));
}

#[test]
fn lluwen_only_accepts_a_land_discard_and_counts_it_for_worms() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::LLUWEN_IMPERFECT_NATURALIST)
        .unwrap();
    settle(&mut game);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == source)
        .unwrap()
        .entered_controller_turn = 0;
    let land = held(&mut game, cards::FOREST);
    held(&mut game, cards::GRIZZLY_BEARS);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::ActivateAbility{source:s,cost_objects,..}if *s==source&&cost_objects.contains(&land))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.step, Step::PrecombatMain);
    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[0].mana.len(), 95);
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn ashling_command_draws_for_the_target_player() {
    let mut game = board(&[]);
    game.players[1].library = game.build_zone(PlayerId::Two, &[cards::FOREST; 3]).unwrap();
    let source = held(&mut game, cards::ASHLING_S_COMMAND);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==source && choices.modes().iter().map(|m|m.index()).collect::<Vec<_>>()==vec![1,3] && choices.iter_targets().all(|t|*t==Target::Player(PlayerId::Two)))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[1].hand.len(), 2);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token() && p.controller == PlayerId::Two)
            .count(),
        2
    );
}

#[test]
fn morningtides_light_returns_both_players_creatures_tapped() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let source = held(&mut game, cards::MORNINGTIDE_S_LIGHT);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==source&&choices.iter_targets().count()==2)).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.battlefield.is_empty());
    game.players[0].mana_pool = ManaPool::default();
    for _ in 0..12 {
        if game.step == Step::End {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }
    assert_eq!(game.battlefield.len(), 2);
    assert!(game.battlefield.iter().all(|p| p.tapped));
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.controller == PlayerId::Two)
    );
}

#[test]
fn figure_upgrades_and_its_protection_follows_current_control() {
    let mut game = board(&[]);
    let figure = game
        .put_onto_battlefield(PlayerId::One, cards::FIGURE_OF_FABLE)
        .unwrap();
    let opponent = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    for index in 0..3 {
        let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::ActivateAbility{source,ability:AbilityOrigin::Printed {ability,..},..}if *source==figure&&*ability==AbilityId(index))).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
    }
    assert_eq!(
        game.power(permanent(&game, cards::FIGURE_OF_FABLE)),
        Some(7)
    );
    game.damage_target_from(Some(opponent), Some(Target::Permanent(figure)), 1);
    assert_eq!(permanent(&game, cards::FIGURE_OF_FABLE).damage, 0);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == figure)
        .unwrap()
        .controller = PlayerId::Two;
    game.damage_target_from(Some(opponent), Some(Target::Permanent(figure)), 1);
    assert_eq!(permanent(&game, cards::FIGURE_OF_FABLE).damage, 1);
}

#[test]
fn meek_attack_sacrifices_the_returned_creature_after_the_enchantment_leaves() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::MEEK_ATTACK)
        .unwrap();
    held(&mut game, cards::GRIZZLY_BEARS);
    activate(&mut game, source);
    let bear = permanent(&game, cards::GRIZZLY_BEARS).card.id;
    game.destroy_permanents(&[source], false);
    game.players[0].mana_pool = ManaPool::default();
    for _ in 0..12 {
        if game.step == Step::End {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }
    assert!(!game.battlefield.iter().any(|p| p.card.id == bear));
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn mornsong_aria_replaces_the_opponents_draw_with_life_loss_and_a_search() {
    let mut game = board(&[]);
    game.players[1].library = game
        .build_zone(PlayerId::Two, &[cards::FOREST, cards::GRIZZLY_BEARS])
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::MORNSONG_ARIA)
        .unwrap();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::Upkeep;
    game.advance_step();
    settle(&mut game);
    assert_eq!(game.players[1].life, 17);
    assert_eq!(game.players[1].hand.len(), 1);
    assert_eq!(game.players[1].library.len(), 1);
    assert_eq!(game.cards_drawn_this_turn[1], 0);
}

#[test]
fn declining_blight_does_not_grant_the_reward() {
    let mut game = board(&[cards::FOREST; 4]);
    game.put_onto_battlefield(PlayerId::One, cards::BLIGHTED_BLACKTHORN)
        .unwrap();
    while game.pending_decisions.is_empty() {
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    let decision = game.pending_decisions[0].observation.clone();
    assert_eq!(decision.minimum, 0);
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![],
        },
    )
    .unwrap();
    settle(&mut game);
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].life, 20);
    assert_eq!(
        permanent(&game, cards::BLIGHTED_BLACKTHORN).counters(CounterKind::MinusOneMinusOne),
        0
    );

    let mut empty = board(&[]);
    empty
        .put_onto_battlefield(PlayerId::One, cards::BOGGART_MISCHIEF)
        .unwrap();
    settle(&mut empty);
    assert_eq!(
        empty.battlefield.len(),
        1,
        "no creature can be blighted for tokens"
    );
}
